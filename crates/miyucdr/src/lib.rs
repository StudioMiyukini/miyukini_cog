//! @id     miyucdr_lib
//! @do     desarmer_reconstruire_contenu
//! @role   securite
//! @layer  toolkit
//! @human  miyucdr — désarmement et reconstruction (CDR) de toute entrée de fichier.

//! # MiyuCDR — désarmement et reconstruction de contenu
//!
//! Lot 3, exigence **EX-45** « CDR obligatoire sur toute entrée de fichier /
//! document » (`docs/implementation/MATRICE-EXIGENCES.md`), contrôle n° 4 de la
//! voie entrante du Portail (`docs/architecture/SERVICES-HYBRIDES-ET-PORTAIL.md`) :
//! « l'objet est décodé puis **réencodé** dans un format interne sûr ; aucun
//! octet brut ne traverse ».
//!
//! ## Principe : fermé par défaut
//!
//! [`desarmer`] reçoit le type **déclaré** et les octets. Il :
//! 1. refuse tout type inconnu ou non pris en charge (aucun passage tel quel) ;
//! 2. borne la taille d'entrée ;
//! 3. détecte le type **réel** par signature (octets magiques) et refuse toute
//!    discordance avec le type déclaré (polyglottes, renommages) ;
//! 4. décode entièrement l'objet sous limites (côtés, pixels, allocation, délai) ;
//! 5. **reconstruit** une sortie neuve à partir des seules données utiles :
//!    - images PNG, JPEG, GIF (première image), WebP, BMP → pixels décodés puis
//!      réencodés en PNG (ou JPEG pour une source JPEG), sans aucune métadonnée
//!      (EXIF, XMP, ICC, blocs texte, commentaires, données après la fin) ;
//!    - texte brut, CSV, Markdown → UTF-8 strict, caractères de contrôle et de
//!      contrôle bidirectionnel retirés, fins de ligne normalisées, NFC ;
//!      CSV analysé strictement (RFC 4180) puis réécrit, formules neutralisées ;
//!    - JSON → analysé, chaînes nettoyées, profondeur bornée, resérialisé.
//!
//! PDF, documents bureautiques et archives sont **refusés** dans cet incrément
//! ([`Refus::NonPrisEnCharge`] indique ce qu'il faudrait pour les accepter).
//!
//! ## Point d'intégration
//!
//! Le trait [`Desarmement`] est la forme consommée par le Portail ; [`Cdr`] en
//! est l'implémentation. Le décodage s'exécute dans un fil dédié borné par
//! [`Limites::delai_max`].
//!
//! ## Délai, saturation et isolement
//!
//! Un fil ne peut pas être tué : au-delà du délai, l'appelant reçoit
//! [`Refus::DelaiDepasse`] mais le fil abandonné continue jusqu'à la fin de
//! l'étape en cours. Deux protections bornent la consommation :
//!
//! - **plafond global** de [`MAX_DECODAGES_SIMULTANES`] désarmements en cours
//!   dans le processus, fils abandonnés **compris** (le jeton n'est rendu qu'à
//!   la fin réelle du fil) ; au-delà, l'appel attend une place au plus
//!   [`Limites::delai_max`] puis est refusé ([`Refus::Sature`]) — des entrées
//!   lentes répétées ne peuvent pas épuiser CPU et mémoire ;
//! - **annulation coopérative** : à l'expiration du délai, un drapeau est levé ;
//!   le fil le consulte entre les étapes (détection, décodage, reconstruction,
//!   contrôle final) et s'arrête sans produire de sortie.
//!
//! L'isolement **fort** (processus séparé, rlimits, seccomp) reste celui du
//! sas de décodage du Portail ; un appelant en processus (MiyuCloud) bénéficie
//! du plafond et de l'annulation, pas d'un confinement.

mod detection;
mod images;
mod json;
mod tableur;
mod texte;

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

pub use detection::{detecter, TypeDetecte};

/// Types de contenu acceptés par le CDR (entrée déclarée et sortie reconstruite).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeContenu {
    /// `image/png`
    Png,
    /// `image/jpeg`
    Jpeg,
    /// `image/gif` (seule la première image est conservée)
    Gif,
    /// `image/webp`
    Webp,
    /// `image/bmp`
    Bmp,
    /// `text/plain`
    TexteBrut,
    /// `text/csv`
    Csv,
    /// `text/markdown`
    Markdown,
    /// `application/json`
    Json,
}

impl TypeContenu {
    /// Type MIME canonique.
    pub fn mime(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::Webp => "image/webp",
            Self::Bmp => "image/bmp",
            Self::TexteBrut => "text/plain",
            Self::Csv => "text/csv",
            Self::Markdown => "text/markdown",
            Self::Json => "application/json",
        }
    }

    /// Vrai pour les types image.
    pub fn est_image(self) -> bool {
        matches!(
            self,
            Self::Png | Self::Jpeg | Self::Gif | Self::Webp | Self::Bmp
        )
    }

    /// Type image correspondant à une détection par signature, s'il est pris en
    /// charge. Sert aux appelants qui ne déclarent qu'une famille (« image »),
    /// comme `NatureDocument::Image` du Portail : la famille reste imposée,
    /// le format précis est celui du contenu.
    pub fn image_depuis_detecte(detecte: TypeDetecte) -> Option<Self> {
        match detecte {
            TypeDetecte::Png => Some(Self::Png),
            TypeDetecte::Jpeg => Some(Self::Jpeg),
            TypeDetecte::Gif => Some(Self::Gif),
            TypeDetecte::Webp => Some(Self::Webp),
            TypeDetecte::Bmp => Some(Self::Bmp),
            _ => None,
        }
    }

    /// Interprète un type MIME déclaré.
    ///
    /// Insensible à la casse ; les paramètres (`; charset=…`) sont analysés :
    /// un `charset` autre qu'UTF-8 (ou US-ASCII) est refusé.
    pub fn depuis_mime(declare: &str) -> Result<Self, Refus> {
        let normalise = declare.trim().to_ascii_lowercase();
        let mut parties = normalise.split(';');
        let essence = parties.next().unwrap_or("").trim();
        for parametre in parties {
            let (cle, valeur) = parametre.split_once('=').unwrap_or((parametre, ""));
            if cle.trim() == "charset" {
                let valeur = valeur.trim().trim_matches('"');
                if !matches!(valeur, "utf-8" | "utf8" | "us-ascii") {
                    return Err(Refus::NonPrisEnCharge {
                        type_declare: declare.to_string(),
                        besoin: "jeu de caractères autre qu'UTF-8 : transcodage non prévu",
                    });
                }
            }
        }
        let type_contenu = match essence {
            "image/png" => Self::Png,
            "image/jpeg" | "image/jpg" | "image/pjpeg" => Self::Jpeg,
            "image/gif" => Self::Gif,
            "image/webp" => Self::Webp,
            "image/bmp" | "image/x-bmp" | "image/x-ms-bmp" => Self::Bmp,
            "text/plain" => Self::TexteBrut,
            "text/csv" => Self::Csv,
            "text/markdown" | "text/x-markdown" => Self::Markdown,
            "application/json" | "text/json" => Self::Json,
            _ => {
                return Err(Refus::NonPrisEnCharge {
                    type_declare: declare.to_string(),
                    besoin: besoin_pour(essence),
                })
            }
        };
        Ok(type_contenu)
    }
}

impl fmt::Display for TypeContenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.mime())
    }
}

/// Ce qu'il faudrait pour accepter un type aujourd'hui refusé.
fn besoin_pour(essence: &str) -> &'static str {
    match essence {
        "application/pdf" => {
            "PDF : rastérisation page par page dans un sas isolé puis réencodage en images \
             (ou extraction texte seule) ; aucun moteur PDF pur Rust sûr retenu"
        }
        e if e.contains("officedocument")
            || e.contains("opendocument")
            || e.contains("msword")
            || e.contains("ms-excel")
            || e.contains("ms-powerpoint") =>
        {
            "bureautique : conversion vers un format interne sans macro ni objet embarqué \
             (texte structuré / CSV / images) par un convertisseur isolé"
        }
        "application/zip"
        | "application/x-zip-compressed"
        | "application/gzip"
        | "application/x-gzip"
        | "application/x-tar"
        | "application/x-7z-compressed"
        | "application/vnd.rar"
        | "application/x-rar-compressed" => {
            "archive : dépaquetage borné (taille, nombre, profondeur, ratio) puis CDR \
             de chaque membre un à un"
        }
        "image/svg+xml" => "SVG : rastérisation (pas de script, pas de référence externe)",
        "text/html" => "HTML : réécriture par liste blanche de balises, ou texte seul",
        _ => "type inconnu : aucun désarmement défini (fermé par défaut)",
    }
}

/// Limites de décodage. Tout dépassement est un refus.
#[derive(Debug, Clone)]
pub struct Limites {
    /// Taille maximale de l'entrée, tous types (octets).
    pub octets_max: usize,
    /// Taille maximale d'une entrée ou sortie texte / CSV / JSON (octets).
    pub texte_octets_max: usize,
    /// Côté maximal d'une image (pixels).
    pub cote_max: u32,
    /// Nombre maximal de pixels d'une image (largeur × hauteur).
    pub pixels_max: u64,
    /// Allocation maximale autorisée au décodeur d'image (octets).
    pub allocation_max: u64,
    /// Délai maximal d'un désarmement.
    pub delai_max: Duration,
    /// Profondeur maximale d'imbrication JSON.
    pub json_profondeur_max: usize,
    /// Nombre maximal d'enregistrements CSV.
    pub csv_enregistrements_max: usize,
    /// Nombre maximal de champs par enregistrement CSV.
    pub csv_champs_max: usize,
    /// Normalisation Unicode NFC des textes.
    pub normaliser_nfc: bool,
    /// Qualité du réencodage JPEG (1 à 100).
    pub qualite_jpeg: u8,
}

impl Default for Limites {
    fn default() -> Self {
        Self {
            octets_max: 32 * 1024 * 1024,
            texte_octets_max: 8 * 1024 * 1024,
            cote_max: 16_384,
            pixels_max: 40_000_000,
            allocation_max: 512 * 1024 * 1024,
            delai_max: Duration::from_secs(10),
            json_profondeur_max: 64,
            csv_enregistrements_max: 1_000_000,
            csv_champs_max: 4_096,
            normaliser_nfc: true,
            qualite_jpeg: 90,
        }
    }
}

/// Compte rendu d'un désarmement réussi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rapport {
    /// Type déclaré par l'émetteur.
    pub type_declare: TypeContenu,
    /// Type détecté par signature.
    pub type_detecte: TypeDetecte,
    /// Taille de l'entrée.
    pub octets_entree: usize,
    /// Taille de la sortie reconstruite.
    pub octets_sortie: usize,
    /// Dimensions (images).
    pub dimensions: Option<(u32, u32)>,
    /// Éléments retirés ou transformés lors de la reconstruction.
    pub retraits: Vec<String>,
    /// Signaux suspects relevés (sans effet sur la sortie, déjà reconstruite).
    pub anomalies: Vec<String>,
}

/// Objet reconstruit : seuls ces octets peuvent traverser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reconstruit {
    /// Octets reconstruits.
    pub octets: Vec<u8>,
    /// Type réel de la sortie (peut différer du type déclaré : GIF → PNG…).
    pub type_reel: TypeContenu,
    /// Compte rendu.
    pub rapport: Rapport,
}

/// Motif de refus. Un refus est définitif : rien n'est transmis.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Refus {
    /// Type déclaré inconnu ou non pris en charge dans cet incrément.
    #[error("type « {type_declare} » non pris en charge ({besoin})")]
    NonPrisEnCharge {
        /// Type déclaré.
        type_declare: String,
        /// Ce qu'il faudrait pour l'accepter.
        besoin: &'static str,
    },
    /// Le contenu réel ne correspond pas au type déclaré.
    #[error("type déclaré {declare} mais contenu détecté : {detecte}")]
    TypeDiscordant {
        /// Type déclaré.
        declare: TypeContenu,
        /// Type détecté.
        detecte: TypeDetecte,
    },
    /// Entrée ou sortie trop volumineuse.
    #[error("taille {octets} octets au-delà de la limite {max}")]
    TropVolumineux {
        /// Taille constatée.
        octets: usize,
        /// Limite.
        max: usize,
    },
    /// Image hors limites (côtés, pixels, allocation) : bombe de décompression probable.
    #[error("limite de décodage dépassée : {0}")]
    LimiteDepassee(String),
    /// Contenu malformé, tronqué ou indécodable.
    #[error("contenu malformé : {0}")]
    Malforme(String),
    /// Délai de désarmement dépassé.
    #[error("délai de désarmement dépassé")]
    DelaiDepasse,
    /// Plafond de désarmements simultanés atteint (fils abandonnés compris) :
    /// réessayer plus tard.
    #[error("CDR saturé : {MAX_DECODAGES_SIMULTANES} désarmements déjà en cours")]
    Sature,
    /// Défaillance interne (panique du décodeur…) : refus par prudence.
    #[error("défaillance interne du désarmement : {0}")]
    Interne(String),
}

/// Nombre maximal de désarmements simultanés dans le processus, fils
/// abandonnés après dépassement du délai **compris**.
pub const MAX_DECODAGES_SIMULTANES: usize = 4;

/// Compteur global des fils de décodage vivants et sa condition d'attente.
static DECODAGES: (Mutex<usize>, Condvar) = (Mutex::new(0), Condvar::new());

fn compteur() -> MutexGuard<'static, usize> {
    // Un empoisonnement ne peut venir que d'une panique pendant une simple
    // incrémentation : la valeur reste cohérente, on la reprend.
    DECODAGES.0.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Désarmements en cours dans le processus (fils abandonnés compris).
pub fn decodages_en_cours() -> usize {
    *compteur()
}

/// Jeton de décodage : rendu à la fin **réelle** du fil (destruction).
struct Jeton;

impl Jeton {
    /// Attend au plus `attente` qu'une place se libère.
    fn acquerir(attente: Duration) -> Option<Self> {
        let garde = compteur();
        let (mut garde, _) = DECODAGES
            .1
            .wait_timeout_while(garde, attente, |n| *n >= MAX_DECODAGES_SIMULTANES)
            .unwrap_or_else(PoisonError::into_inner);
        if *garde >= MAX_DECODAGES_SIMULTANES {
            return None;
        }
        *garde += 1;
        Some(Jeton)
    }
}

impl Drop for Jeton {
    fn drop(&mut self) {
        let mut n = compteur();
        *n = n.saturating_sub(1);
        drop(n);
        DECODAGES.1.notify_one();
    }
}

/// Drapeau d'annulation coopérative (levé à l'expiration du délai).
#[derive(Debug, Clone, Default)]
struct Annulation(Arc<AtomicBool>);

impl Annulation {
    fn lever(&self) {
        self.0.store(true, Ordering::Release);
    }

    fn verifier(&self) -> Result<(), Refus> {
        if self.0.load(Ordering::Acquire) {
            Err(Refus::DelaiDepasse)
        } else {
            Ok(())
        }
    }
}

/// Désarme et reconstruit `octets` déclarés de type `type_declare` (MIME).
///
/// Fermé par défaut : tout ce qui n'est pas explicitement reconstruit est refusé.
/// Si [`MAX_DECODAGES_SIMULTANES`] désarmements sont déjà en cours, attend une
/// place au plus [`Limites::delai_max`], sinon [`Refus::Sature`] (voir la
/// documentation du crate).
pub fn desarmer(
    type_declare: &str,
    octets: &[u8],
    limites: &Limites,
) -> Result<Reconstruit, Refus> {
    let type_contenu = TypeContenu::depuis_mime(type_declare)?;
    let max = if type_contenu.est_image() {
        limites.octets_max
    } else {
        limites.texte_octets_max.min(limites.octets_max)
    };
    if octets.len() > max {
        return Err(Refus::TropVolumineux {
            octets: octets.len(),
            max,
        });
    }

    // Plafond global : le jeton suit le fil, même abandonné. Attente bornée
    // par le délai (les appels légitimes concurrents patientent).
    let jeton = Jeton::acquerir(limites.delai_max).ok_or(Refus::Sature)?;

    // Décodage dans un fil dédié : délai borné, panique confinée.
    let entree = octets.to_vec();
    let limites_fil = limites.clone();
    let annulation = Annulation::default();
    let annulation_fil = annulation.clone();
    let (envoi, reception) = mpsc::sync_channel(1);
    std::thread::Builder::new()
        .name("miyucdr".into())
        .spawn(move || {
            let _jeton = jeton;
            let resultat = desarmer_type(type_contenu, &entree, &limites_fil, &annulation_fil);
            // Le destinataire a pu abandonner (délai) : rien à faire.
            let _ = envoi.send(resultat);
        })
        .map_err(|e| Refus::Interne(format!("fil de décodage : {e}")))?;

    match reception.recv_timeout(limites.delai_max) {
        Ok(resultat) => resultat,
        Err(mpsc::RecvTimeoutError::Timeout) => {
            annulation.lever();
            Err(Refus::DelaiDepasse)
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(Refus::Interne("le décodeur s'est interrompu".into()))
        }
    }
}

/// Aiguillage par type, après bornage de taille.
fn desarmer_type(
    declare: TypeContenu,
    octets: &[u8],
    limites: &Limites,
    annulation: &Annulation,
) -> Result<Reconstruit, Refus> {
    annulation.verifier()?;
    let detecte = detecter(octets);
    let attendu = match declare {
        TypeContenu::Png => TypeDetecte::Png,
        TypeContenu::Jpeg => TypeDetecte::Jpeg,
        TypeContenu::Gif => TypeDetecte::Gif,
        TypeContenu::Webp => TypeDetecte::Webp,
        TypeContenu::Bmp => TypeDetecte::Bmp,
        TypeContenu::TexteBrut | TypeContenu::Csv | TypeContenu::Markdown | TypeContenu::Json => {
            TypeDetecte::Texte
        }
    };
    if detecte != attendu {
        return Err(Refus::TypeDiscordant { declare, detecte });
    }

    let mut rapport = Rapport {
        type_declare: declare,
        type_detecte: detecte,
        octets_entree: octets.len(),
        octets_sortie: 0,
        dimensions: None,
        retraits: Vec::new(),
        anomalies: detection::anomalies(octets, detecte),
    };

    annulation.verifier()?;
    let (sortie, type_reel) = match declare {
        TypeContenu::Png
        | TypeContenu::Jpeg
        | TypeContenu::Gif
        | TypeContenu::Webp
        | TypeContenu::Bmp => images::reconstruire(declare, octets, limites, &mut rapport)?,
        TypeContenu::TexteBrut | TypeContenu::Markdown => (
            texte::reconstruire(declare, octets, limites, &mut rapport)?,
            declare,
        ),
        TypeContenu::Csv => (
            tableur::reconstruire(octets, limites, &mut rapport)?,
            TypeContenu::Csv,
        ),
        TypeContenu::Json => (
            json::reconstruire(octets, limites, &mut rapport)?,
            TypeContenu::Json,
        ),
    };

    annulation.verifier()?;
    if !type_reel.est_image() && sortie.len() > limites.texte_octets_max {
        return Err(Refus::TropVolumineux {
            octets: sortie.len(),
            max: limites.texte_octets_max,
        });
    }
    rapport.octets_sortie = sortie.len();
    Ok(Reconstruit {
        octets: sortie,
        type_reel,
        rapport,
    })
}

/// Désarmement tel que consommé par le Portail (voie entrante, contrôle 4).
///
/// Contrat : `Ok` ne contient **que** des octets reconstruits ; tout type
/// inconnu, non pris en charge, discordant, malformé ou hors limites est `Err`.
pub trait Desarmement: Send + Sync {
    /// Désarme `octets` déclarés de type MIME `type_declare`.
    fn desarmer(&self, type_declare: &str, octets: &[u8]) -> Result<Vec<u8>, String>;
}

/// Implémentation du CDR, paramétrée par ses [`Limites`].
#[derive(Debug, Clone, Default)]
pub struct Cdr {
    limites: Limites,
}

impl Cdr {
    /// CDR avec des limites explicites.
    pub fn new(limites: Limites) -> Self {
        Self { limites }
    }

    /// Limites en vigueur.
    pub fn limites(&self) -> &Limites {
        &self.limites
    }

    /// Variante détaillée : sortie, type réel et rapport, ou refus typé.
    pub fn desarmer_detaille(
        &self,
        type_declare: &str,
        octets: &[u8],
    ) -> Result<Reconstruit, Refus> {
        desarmer(type_declare, octets, &self.limites)
    }
}

impl Desarmement for Cdr {
    fn desarmer(&self, type_declare: &str, octets: &[u8]) -> Result<Vec<u8>, String> {
        desarmer(type_declare, octets, &self.limites)
            .map(|r| r.octets)
            .map_err(|refus| refus.to_string())
    }
}
