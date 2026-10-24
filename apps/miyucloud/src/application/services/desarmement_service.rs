//! @id     miyucloud_application_services_desarmement_service
//! @do     desarmer_fichier_entrant
//! @role   securite
//! @layer  app
//! @human  miyucloud — entonnoir CDR : tout fichier entrant est désarmé, mis en quarantaine ou refusé.

//! Entonnoir de désarmement (CDR) de MiyuCloud — décision A-004, EX-45.
//!
//! Tout octet de fichier qui entre dans MiyuCloud passe par
//! [`DesarmementService::desarmer_fichier`] **avant** d'être haché et rangé
//! dans le magasin de blobs. Trois issues :
//!
//! 1. **désarmé** — le CDR (`miyucdr`) a reconstruit l'objet : le fichier
//!    temporaire est **réécrit** avec les octets reconstruits, la taille, le
//!    hachage (BLAKE3) et le type MIME stockés sont ceux de la reconstruction ;
//! 2. **quarantaine** — type non pris en charge par le CDR (PDF, vidéo,
//!    archive, bureautique…) ou taille au-delà de sa limite en mémoire : les
//!    octets d'origine sont stockés tels quels mais marqués « non désarmé »
//!    (jamais ouverts par COG, téléchargement sur accord explicite) ;
//! 3. **refus** — contenu discordant avec son type déclaré (polyglotte,
//!    renommage), malformé, hors limites de décodage, délai dépassé :
//!    l'écriture est refusée (`InvalidInput` → 400), rien n'est stocké.
//!
//! Le service est appelé par `FileUploadService` (tous les chemins d'écriture :
//! téléversement multipart, téléversement par morceaux, WebDAV, Nextcloud,
//! WOPI) et par les deux points qui reçoivent des octets hors de ce service
//! (vignettes fournies par le client, dépôt de blob administrateur).

use std::path::Path;
use std::sync::Arc;

use miyucdr::{Cdr, Limites, Reconstruit, Refus, TypeContenu, TypeDetecte, detecter};
use serde_json::json;
use tokio::io::AsyncReadExt;
use tokio::sync::Semaphore;

use crate::common::errors::DomainError;
use crate::domain::entities::etat_cdr::{EtatCdr, MarqueCdr};

/// Octets lus en tête de fichier pour vérifier la signature d'un fichier trop
/// volumineux pour le CDR (polyglottes et renommages restent refusés).
const TETE_SIGNATURE: usize = 8192;

/// Résultat d'un passage dans l'entonnoir (hors refus).
#[derive(Debug, Clone)]
pub struct IssueDesarmement {
    /// `Desarme` ou `Quarantaine` (jamais `Inconnu`).
    pub etat: EtatCdr,
    /// Taille des octets à stocker (reconstruits si désarmé).
    pub taille: u64,
    /// Hachage BLAKE3 des octets à stocker ; `None` = à calculer par le
    /// magasin de blobs (quarantaine sans hachage fourni).
    pub hachage: Option<String>,
    /// Type MIME à stocker (type réel de la reconstruction si désarmé).
    pub type_mime: String,
    /// Compte rendu résumé, stocké dans `storage.files.cdr_rapport`.
    pub rapport: serde_json::Value,
}

impl IssueDesarmement {
    /// Marque d'état à transmettre au dépôt.
    pub fn marque(&self) -> MarqueCdr {
        MarqueCdr {
            etat: self.etat,
            rapport: Some(self.rapport.clone()),
        }
    }
}

/// Entonnoir CDR unique de MiyuCloud.
pub struct DesarmementService {
    cdr: Cdr,
    /// Borne le nombre de désarmements simultanés (chacun tient l'objet en
    /// mémoire, jusqu'à `Limites::octets_max`).
    concurrence: Arc<Semaphore>,
}

impl Default for DesarmementService {
    fn default() -> Self {
        Self::new(Limites::default())
    }
}

impl DesarmementService {
    /// Entonnoir avec les limites CDR données.
    pub fn new(limites: Limites) -> Self {
        let paralleles = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(2)
            .clamp(2, 8);
        Self {
            cdr: Cdr::new(limites),
            concurrence: Arc::new(Semaphore::new(paralleles)),
        }
    }

    /// Limites CDR en vigueur.
    pub fn limites(&self) -> &Limites {
        self.cdr.limites()
    }

    /// Taille maximale traitée en mémoire par le CDR pour ce type.
    fn taille_max(&self, type_contenu: TypeContenu) -> usize {
        let limites = self.cdr.limites();
        if type_contenu.est_image() {
            limites.octets_max
        } else {
            limites.texte_octets_max.min(limites.octets_max)
        }
    }

    /// Passe le fichier temporaire `chemin` (déclaré `type_declare`) dans le CDR.
    ///
    /// En cas de désarmement, `chemin` est réécrit avec les octets reconstruits.
    /// `hachage_fourni` (BLAKE3 des octets d'origine, calculé pendant la
    /// réception) n'est conservé que pour la quarantaine, où les octets
    /// stockés sont ceux d'origine.
    pub async fn desarmer_fichier(
        &self,
        chemin: &Path,
        type_declare: &str,
        hachage_fourni: Option<String>,
    ) -> Result<IssueDesarmement, DomainError> {
        let taille = tokio::fs::metadata(chemin)
            .await
            .map_err(|e| {
                DomainError::internal_error("Desarmement", format!("métadonnées du fichier : {e}"))
            })?
            .len();

        // Fichier vide : aucun contenu à désarmer.
        if taille == 0 {
            return Ok(IssueDesarmement {
                etat: EtatCdr::Desarme,
                taille: 0,
                hachage: Some(blake3::hash(&[]).to_hex().to_string()),
                type_mime: type_declare.to_string(),
                rapport: json!({ "issue": "desarme", "motif": "fichier vide" }),
            });
        }

        // Type déclaré hors du CDR : quarantaine, sans lire le contenu.
        let type_contenu = match TypeContenu::depuis_mime(type_declare) {
            Ok(t) => t,
            Err(Refus::NonPrisEnCharge { besoin, .. }) => {
                return Ok(Self::quarantaine(
                    taille,
                    hachage_fourni,
                    type_declare,
                    format!("type non pris en charge par le CDR : {besoin}"),
                ));
            }
            Err(refus) => return Err(Self::refus(&refus)),
        };

        // Trop volumineux pour le CDR en mémoire : quarantaine, après
        // vérification de la signature (un polyglotte reste refusé).
        let max = self.taille_max(type_contenu);
        if taille > max as u64 {
            self.verifier_signature(chemin, type_contenu).await?;
            return Ok(Self::quarantaine(
                taille,
                hachage_fourni,
                type_declare,
                format!("taille {taille} octets au-delà de la limite CDR {max}"),
            ));
        }

        let octets = tokio::fs::read(chemin).await.map_err(|e| {
            DomainError::internal_error("Desarmement", format!("lecture du fichier : {e}"))
        })?;

        match self.desarmer_octets(type_declare, octets).await? {
            Ok(reconstruit) => {
                tokio::fs::write(chemin, &reconstruit.octets)
                    .await
                    .map_err(|e| {
                        DomainError::internal_error(
                            "Desarmement",
                            format!("écriture des octets reconstruits : {e}"),
                        )
                    })?;
                Ok(IssueDesarmement {
                    etat: EtatCdr::Desarme,
                    taille: reconstruit.octets.len() as u64,
                    hachage: Some(blake3::hash(&reconstruit.octets).to_hex().to_string()),
                    type_mime: reconstruit.type_reel.mime().to_string(),
                    rapport: Self::rapport_desarme(&reconstruit),
                })
            }
            // Jeu de caractères non UTF-8 déclaré, sortie texte trop grande :
            // le CDR ne sait pas le reconstruire, mais rien d'hostile n'est établi.
            Err(Refus::NonPrisEnCharge { besoin, .. }) => Ok(Self::quarantaine(
                taille,
                hachage_fourni,
                type_declare,
                format!("non pris en charge par le CDR : {besoin}"),
            )),
            Err(refus @ Refus::TropVolumineux { .. }) => Ok(Self::quarantaine(
                taille,
                hachage_fourni,
                type_declare,
                refus.to_string(),
            )),
            Err(refus) => Err(Self::refus(&refus)),
        }
    }

    /// Désarme une image reçue en mémoire (vignette fournie par le client).
    ///
    /// Aucune quarantaine possible ici : tout ce qui n'est pas une image
    /// reconstructible est refusé.
    pub async fn desarmer_image(&self, octets: Vec<u8>) -> Result<Reconstruit, DomainError> {
        let type_image = TypeContenu::image_depuis_detecte(detecter(&octets)).ok_or_else(|| {
            DomainError::validation_error(
                "Contenu refusé par le désarmement (CDR) : image PNG, JPEG, GIF, WebP ou BMP attendue",
            )
        })?;
        match self.desarmer_octets(type_image.mime(), octets).await? {
            Ok(reconstruit) => Ok(reconstruit),
            Err(refus) => Err(Self::refus(&refus)),
        }
    }

    /// Désarme des octets déposés hors fichier (dépôt de blob administrateur) :
    /// seuls les contenus reconstruits sont acceptés, la quarantaine n'y étant
    /// pas représentable (pas de ligne `storage.files`).
    pub async fn desarmer_sans_quarantaine(
        &self,
        chemin: &Path,
        type_declare: &str,
    ) -> Result<IssueDesarmement, DomainError> {
        let issue = self.desarmer_fichier(chemin, type_declare, None).await?;
        if issue.etat != EtatCdr::Desarme {
            return Err(DomainError::validation_error(
                "Contenu non désarmable (quarantaine impossible pour un blob sans fichier)",
            ));
        }
        Ok(issue)
    }

    /// Appel du CDR hors de la boucle asynchrone, sous borne de concurrence.
    async fn desarmer_octets(
        &self,
        type_declare: &str,
        octets: Vec<u8>,
    ) -> Result<Result<Reconstruit, Refus>, DomainError> {
        let _jeton = self
            .concurrence
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| DomainError::internal_error("Desarmement", format!("sémaphore : {e}")))?;
        let cdr = self.cdr.clone();
        let declare = type_declare.to_string();
        tokio::task::spawn_blocking(move || cdr.desarmer_detaille(&declare, &octets))
            .await
            .map_err(|e| DomainError::internal_error("Desarmement", format!("tâche CDR : {e}")))
    }

    /// Vérifie la signature d'un fichier trop volumineux pour le CDR : le
    /// contenu doit correspondre au type déclaré (sinon refus, comme le CDR).
    async fn verifier_signature(
        &self,
        chemin: &Path,
        type_contenu: TypeContenu,
    ) -> Result<(), DomainError> {
        let mut tete = vec![0u8; TETE_SIGNATURE];
        let lus = {
            let mut fichier = tokio::fs::File::open(chemin).await.map_err(|e| {
                DomainError::internal_error("Desarmement", format!("ouverture du fichier : {e}"))
            })?;
            let mut total = 0;
            while total < tete.len() {
                let n = fichier.read(&mut tete[total..]).await.map_err(|e| {
                    DomainError::internal_error("Desarmement", format!("lecture de tête : {e}"))
                })?;
                if n == 0 {
                    break;
                }
                total += n;
            }
            total
        };
        tete.truncate(lus);
        let detecte = if type_contenu.est_image() {
            detecter(&tete)
        } else {
            // Une tête de texte peut couper un caractère UTF-8 en fin de fenêtre.
            let utile = match std::str::from_utf8(&tete) {
                Err(e) if e.error_len().is_none() => e.valid_up_to(),
                _ => tete.len(),
            };
            detecter(&tete[..utile])
        };
        let attendu = match type_contenu {
            TypeContenu::Png => TypeDetecte::Png,
            TypeContenu::Jpeg => TypeDetecte::Jpeg,
            TypeContenu::Gif => TypeDetecte::Gif,
            TypeContenu::Webp => TypeDetecte::Webp,
            TypeContenu::Bmp => TypeDetecte::Bmp,
            TypeContenu::TexteBrut
            | TypeContenu::Csv
            | TypeContenu::Markdown
            | TypeContenu::Json => TypeDetecte::Texte,
        };
        if detecte != attendu {
            return Err(Self::refus(&Refus::TypeDiscordant {
                declare: type_contenu,
                detecte,
            }));
        }
        Ok(())
    }

    fn quarantaine(
        taille: u64,
        hachage: Option<String>,
        type_declare: &str,
        motif: String,
    ) -> IssueDesarmement {
        tracing::info!(
            "🔒 CDR : quarantaine ({} octets, type déclaré « {} ») — {}",
            taille,
            type_declare,
            motif
        );
        IssueDesarmement {
            etat: EtatCdr::Quarantaine,
            taille,
            hachage,
            type_mime: type_declare.to_string(),
            rapport: json!({
                "issue": "quarantaine",
                "type_declare": type_declare,
                "motif": motif,
            }),
        }
    }

    fn rapport_desarme(r: &Reconstruit) -> serde_json::Value {
        json!({
            "issue": "desarme",
            "type_declare": r.rapport.type_declare.mime(),
            "type_detecte": format!("{:?}", r.rapport.type_detecte),
            "type_reel": r.type_reel.mime(),
            "octets_entree": r.rapport.octets_entree,
            "octets_sortie": r.rapport.octets_sortie,
            "dimensions": r.rapport.dimensions.map(|(l, h)| json!([l, h])),
            "retraits": r.rapport.retraits,
            "anomalies": r.rapport.anomalies,
        })
    }

    fn refus(refus: &Refus) -> DomainError {
        tracing::warn!("⛔ CDR : contenu refusé — {}", refus);
        DomainError::validation_error(format!("Contenu refusé par le désarmement (CDR) : {refus}"))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::common::errors::ErrorKind;
    use image::codecs::png::PngEncoder;
    use image::{DynamicImage, Rgb, RgbImage};
    use std::time::Duration;

    fn crc32(octets: &[u8]) -> u32 {
        let mut crc = 0xFFFF_FFFFu32;
        for &o in octets {
            crc ^= u32::from(o);
            for _ in 0..8 {
                crc = if crc & 1 == 1 {
                    (crc >> 1) ^ 0xEDB8_8320
                } else {
                    crc >> 1
                };
            }
        }
        !crc
    }

    /// PNG 16×12 portant un bloc tEXt (métadonnée) inséré après IHDR.
    pub(crate) fn png_avec_metadonnees() -> Vec<u8> {
        let img = DynamicImage::ImageRgb8(RgbImage::from_fn(16, 12, |x, y| {
            Rgb([(x * 9) as u8, (y * 11) as u8, 77])
        }));
        let mut png = Vec::new();
        img.write_with_encoder(PngEncoder::new(&mut png)).unwrap();
        let donnees = b"Comment\0SECRET-METADONNEE";
        let mut bloc = (donnees.len() as u32).to_be_bytes().to_vec();
        bloc.extend_from_slice(b"tEXt");
        bloc.extend_from_slice(donnees);
        let mut c = b"tEXt".to_vec();
        c.extend_from_slice(donnees);
        bloc.extend_from_slice(&crc32(&c).to_be_bytes());
        let mut sortie = png[..33].to_vec();
        sortie.extend_from_slice(&bloc);
        sortie.extend_from_slice(&png[33..]);
        sortie
    }

    pub(crate) const PDF_MINIMAL: &[u8] =
        b"%PDF-1.4\n1 0 obj << /Type /Catalog >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF\n";

    fn contient(foin: &[u8], aiguille: &[u8]) -> bool {
        foin.windows(aiguille.len()).any(|w| w == aiguille)
    }

    async fn temporaire(octets: &[u8]) -> tempfile::NamedTempFile {
        let t = tempfile::NamedTempFile::new().unwrap();
        tokio::fs::write(t.path(), octets).await.unwrap();
        t
    }

    #[tokio::test]
    async fn image_desarmee_sans_metadonnees_et_hachage_des_octets_reconstruits() {
        let service = DesarmementService::default();
        let brut = png_avec_metadonnees();
        assert!(contient(&brut, b"SECRET-METADONNEE"));
        let t = temporaire(&brut).await;
        let hachage_brut = blake3::hash(&brut).to_hex().to_string();

        let issue = service
            .desarmer_fichier(t.path(), "image/png", Some(hachage_brut.clone()))
            .await
            .expect("PNG désarmé");

        assert_eq!(issue.etat, EtatCdr::Desarme);
        let stocke = tokio::fs::read(t.path()).await.unwrap();
        assert!(!contient(&stocke, b"tEXt"), "bloc tEXt retiré");
        assert!(!contient(&stocke, b"SECRET-METADONNEE"));
        assert_eq!(issue.taille, stocke.len() as u64);
        let hachage = issue.hachage.clone().unwrap();
        assert_eq!(hachage, blake3::hash(&stocke).to_hex().to_string());
        assert_ne!(hachage, hachage_brut, "le hachage brut n'est pas réutilisé");
        assert_eq!(issue.type_mime, "image/png");
        assert_eq!(issue.rapport["issue"], "desarme");
    }

    #[tokio::test]
    async fn texte_desarme() {
        let service = DesarmementService::default();
        let t = temporaire(b"ligne 1\r\nligne\x07 2\r\n\xE2\x80\xAEcache").await;
        let issue = service
            .desarmer_fichier(t.path(), "text/plain", None)
            .await
            .expect("texte désarmé");
        assert_eq!(issue.etat, EtatCdr::Desarme);
        let stocke = tokio::fs::read(t.path()).await.unwrap();
        assert!(!stocke.contains(&b'\r'), "fins de ligne normalisées");
        assert!(!stocke.contains(&0x07), "caractère de contrôle retiré");
        assert!(
            !contient(&stocke, "\u{202E}".as_bytes()),
            "contrôle bidirectionnel retiré"
        );
        assert_eq!(
            issue.hachage.unwrap(),
            blake3::hash(&stocke).to_hex().to_string()
        );
    }

    #[tokio::test]
    async fn pdf_en_quarantaine_octets_intacts() {
        let service = DesarmementService::default();
        let t = temporaire(PDF_MINIMAL).await;
        let issue = service
            .desarmer_fichier(t.path(), "application/pdf", Some("h-brut".into()))
            .await
            .expect("PDF en quarantaine");
        assert_eq!(issue.etat, EtatCdr::Quarantaine);
        assert_eq!(issue.hachage.as_deref(), Some("h-brut"));
        assert_eq!(issue.type_mime, "application/pdf");
        assert_eq!(tokio::fs::read(t.path()).await.unwrap(), PDF_MINIMAL);
        assert_eq!(issue.rapport["issue"], "quarantaine");
    }

    #[tokio::test]
    async fn polyglotte_refuse() {
        let service = DesarmementService::default();
        // Texte UTF-8 valide qui est aussi un PDF : discordant, refusé.
        let t = temporaire(PDF_MINIMAL).await;
        let err = service
            .desarmer_fichier(t.path(), "text/plain", None)
            .await
            .expect_err("polyglotte refusé");
        assert_eq!(err.kind, ErrorKind::InvalidInput);
        // PDF renommé en image : refusé.
        let t = temporaire(PDF_MINIMAL).await;
        let err = service
            .desarmer_fichier(t.path(), "image/png", None)
            .await
            .expect_err("PDF déclaré PNG refusé");
        assert_eq!(err.kind, ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn trop_volumineux_en_quarantaine_mais_signature_verifiee() {
        let limites = Limites {
            octets_max: 64,
            texte_octets_max: 64,
            delai_max: Duration::from_secs(10),
            ..Limites::default()
        };
        let service = DesarmementService::new(limites);
        let gros = png_avec_metadonnees();
        assert!(gros.len() > 64);
        let t = temporaire(&gros).await;
        let issue = service
            .desarmer_fichier(t.path(), "image/png", None)
            .await
            .unwrap();
        assert_eq!(issue.etat, EtatCdr::Quarantaine);
        assert_eq!(tokio::fs::read(t.path()).await.unwrap(), gros);

        let mut faux = PDF_MINIMAL.to_vec();
        faux.resize(200, b' ');
        let t = temporaire(&faux).await;
        let err = service
            .desarmer_fichier(t.path(), "image/png", None)
            .await
            .expect_err("signature discordante refusée même au-delà de la limite");
        assert_eq!(err.kind, ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn fichier_vide_desarme() {
        let service = DesarmementService::default();
        let t = temporaire(b"").await;
        let issue = service
            .desarmer_fichier(t.path(), "application/pdf", None)
            .await
            .unwrap();
        assert_eq!(issue.etat, EtatCdr::Desarme);
        assert_eq!(issue.taille, 0);
    }

    #[tokio::test]
    async fn vignette_client_desarmee_ou_refusee() {
        let service = DesarmementService::default();
        let r = service
            .desarmer_image(png_avec_metadonnees())
            .await
            .expect("vignette désarmée");
        assert!(!contient(&r.octets, b"SECRET-METADONNEE"));
        let err = service
            .desarmer_image(PDF_MINIMAL.to_vec())
            .await
            .expect_err("non-image refusée");
        assert_eq!(err.kind, ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn blob_sans_fichier_refuse_la_quarantaine() {
        let service = DesarmementService::default();
        let t = temporaire(PDF_MINIMAL).await;
        let err = service
            .desarmer_sans_quarantaine(t.path(), "application/pdf")
            .await
            .expect_err("quarantaine impossible");
        assert_eq!(err.kind, ErrorKind::InvalidInput);
    }
}
