//! @id     maia_llm_provenance
//! @do     verifier_provenance_modele
//! @role   securite
//! @layer  app
//! @human  maia-llm — provenance signée des modèles GGUF (EX-26, C-53, C-72).

//! Provenance signée des modèles (EX-26 « modèles IA uniquement via le canal
//! officiel, signés »).
//!
//! # Règle
//!
//! Un fichier `X.gguf` n'est chargé que si :
//!
//! 1. un **manifeste signé** `X.gguf.sceau` existe à côté de lui ;
//! 2. ce manifeste est un Sceau v1 (`miyusceau`) de type `ManifestePaquet`
//!    (`0x03`), de genre [`GENRE_MODELE`], signé par une **ancre de confiance
//!    épinglée** dans la configuration (`model_trust_anchors`) ;
//! 3. le nom de fichier, la taille et le **SHA-256 recalculé** du fichier
//!    correspondent au manifeste ;
//! 4. la licence déclarée est autorisée (C-72 : `Apache-2.0` ou `MIT`).
//!
//! Sinon le chargement est refusé (fail-closed). Aucune ancre configurée =
//! aucun modèle chargeable.
//!
//! # Format du manifeste (charge TLV, tags croissants)
//!
//! | Tag | Champ            | Type            | Contrainte                         |
//! |-----|------------------|-----------------|------------------------------------|
//! | 1   | genre            | texte           | exactement `maia/modele-gguf/v1`   |
//! | 2   | nom de fichier   | texte           | 1..=255 octets, sans `/` ni `\`    |
//! | 3   | SHA-256          | 32 octets       | empreinte du fichier complet       |
//! | 4   | taille           | u64 (BE)        | taille exacte en octets            |
//! | 5   | licence          | texte (SPDX)    | `Apache-2.0` ou `MIT`              |
//! | 6   | date de coupure  | texte           | connaissances du modèle, ex. `2025-06` (1..=32) |
//! | 7   | émis le          | u64             | secondes Unix de la signature      |
//!
//! Enveloppe : `MSCE` · version `0x01` · type `0x03` · kid · longueur · charge ·
//! signature Ed25519 (voir `docs/tools/MiyuSceau/_index.md`).
//!
//! # Outillage
//!
//! - `maia-llm cle-dev --sortie cle.hex` : génère une graine de signature de
//!   développement (fichier `0600`) et affiche la clé publique à épingler ;
//! - `maia-llm sceller-modele --graine cle.hex --modele X.gguf --licence MIT
//!   --coupure 2025-06` : écrit `X.gguf.sceau`.
//!
//! # Dérogation développeur
//!
//! `--dev-allow-unsigned-models` ou `MAIA_LLM_DEV_ALLOW_UNSIGNED_MODELS=1`
//! autorise un modèle non signé **uniquement dans un binaire de debug**
//! (`debug_assertions`), avec un journal d'erreur à chaque chargement. Dans un
//! binaire de release, la demande est ignorée et journalisée.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use miyusceau::{
    sceller, verifier, AncresDeConfiance, CleSignature, EcrivainTlv, ErreurSceau, LecteurTlv,
    NiveauConfiance, Titre, TypeSceau,
};
use sha2::{Digest, Sha256};

/// Genre obligatoire du manifeste (séparation des manifestes de paquets).
pub const GENRE_MODELE: &str = "maia/modele-gguf/v1";
/// Extension du manifeste signé, ajoutée au nom du fichier modèle.
pub const EXTENSION_MANIFESTE: &str = "sceau";
/// Licences de modèles autorisées (C-72).
pub const LICENCES_AUTORISEES: &[&str] = &["Apache-2.0", "MIT"];
/// Variable d'environnement de dérogation développeur.
pub const ENV_DEROGATION: &str = "MAIA_LLM_DEV_ALLOW_UNSIGNED_MODELS";
/// Option de ligne de commande de dérogation développeur.
pub const OPTION_DEROGATION: &str = "--dev-allow-unsigned-models";

/// Taille maximale lue pour un manifeste (C-13).
const TAILLE_MAX_MANIFESTE: u64 = 128 * 1024;
const NOM_MAX: usize = 255;
const LICENCE_MAX: usize = 64;
const COUPURE_MAX: usize = 32;

/// Manifeste signé d'un modèle GGUF.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifesteModele {
    /// Nom du fichier modèle (sans chemin).
    pub nom_fichier: String,
    /// SHA-256 du fichier complet.
    pub sha256: [u8; 32],
    /// Taille exacte en octets.
    pub taille: u64,
    /// Licence SPDX.
    pub licence: String,
    /// Date de coupure des connaissances (ex. `2025-06`).
    pub date_coupure: String,
    /// Date d'émission (secondes Unix).
    pub emis_le: u64,
}

fn texte_borne(valeur: &str, tag: u8, max: usize) -> Result<(), ErreurSceau> {
    if valeur.is_empty() || valeur.len() > max {
        return Err(ErreurSceau::ValeurInvalide(tag));
    }
    Ok(())
}

fn nom_fichier_valide(nom: &str) -> bool {
    !nom.is_empty()
        && nom.len() <= NOM_MAX
        && !nom.contains(['/', '\\', '\0'])
        && nom != "."
        && nom != ".."
}

impl Titre for ManifesteModele {
    const TYPE: TypeSceau = TypeSceau::ManifestePaquet;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        if !nom_fichier_valide(&self.nom_fichier) {
            return Err(ErreurSceau::ValeurInvalide(2));
        }
        texte_borne(&self.licence, 5, LICENCE_MAX)?;
        texte_borne(&self.date_coupure, 6, COUPURE_MAX)?;
        e.texte(1, GENRE_MODELE)?
            .texte(2, &self.nom_fichier)?
            .b32(3, &self.sha256)?
            .u64(4, self.taille)?
            .texte(5, &self.licence)?
            .texte(6, &self.date_coupure)?
            .u64(7, self.emis_le)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        if l.exiger_texte(1)? != GENRE_MODELE {
            return Err(ErreurSceau::ValeurInvalide(1));
        }
        let nom_fichier = l.exiger_texte(2)?.to_string();
        if !nom_fichier_valide(&nom_fichier) {
            return Err(ErreurSceau::ValeurInvalide(2));
        }
        let sha256 = l.exiger_b32(3)?;
        let taille = l.exiger_u64(4)?;
        let licence = l.exiger_texte(5)?.to_string();
        texte_borne(&licence, 5, LICENCE_MAX)?;
        let date_coupure = l.exiger_texte(6)?.to_string();
        texte_borne(&date_coupure, 6, COUPURE_MAX)?;
        let emis_le = l.exiger_u64(7)?;
        Ok(Self { nom_fichier, sha256, taille, licence, date_coupure, emis_le })
    }
}

/// Motif de refus de chargement d'un modèle (une variante par cause, C-11).
#[derive(Debug)]
pub enum ErreurProvenance {
    /// Aucun manifeste signé à côté du modèle.
    ManifesteAbsent(PathBuf),
    /// Manifeste illisible ou trop gros.
    ManifesteIllisible(String),
    /// Sceau refusé (format, type, signataire inconnu, signature…).
    Sceau(ErreurSceau),
    /// Le manifeste décrit un autre fichier.
    NomDifferent {
        /// Nom attendu par le manifeste.
        attendu: String,
        /// Nom du fichier présenté.
        trouve: String,
    },
    /// Taille du fichier différente de celle signée.
    TailleDifferente {
        /// Taille signée.
        attendue: u64,
        /// Taille constatée.
        trouvee: u64,
    },
    /// Empreinte SHA-256 du fichier différente de celle signée.
    EmpreinteDifferente,
    /// Licence non autorisée (C-72).
    LicenceRefusee(String),
    /// Lecture du fichier modèle impossible.
    LectureModele(String),
    /// Ancre de confiance mal formée dans la configuration.
    AncreInvalide(String),
}

impl std::fmt::Display for ErreurProvenance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManifesteAbsent(p) => {
                write!(f, "manifeste signé absent ({}) : modèle refusé", p.display())
            }
            Self::ManifesteIllisible(e) => write!(f, "manifeste illisible : {e}"),
            Self::Sceau(e) => write!(f, "manifeste refusé : {e}"),
            Self::NomDifferent { attendu, trouve } => {
                write!(f, "manifeste pour « {attendu} », fichier « {trouve} »")
            }
            Self::TailleDifferente { attendue, trouvee } => {
                write!(f, "taille {trouvee} o, {attendue} o signés")
            }
            Self::EmpreinteDifferente => write!(f, "empreinte SHA-256 différente de la valeur signée"),
            Self::LicenceRefusee(l) => write!(f, "licence « {l} » non autorisée (Apache-2.0 ou MIT)"),
            Self::LectureModele(e) => write!(f, "lecture du modèle impossible : {e}"),
            Self::AncreInvalide(e) => write!(f, "ancre de confiance invalide : {e}"),
        }
    }
}

impl std::error::Error for ErreurProvenance {}

/// Chemin du manifeste associé à un modèle (`X.gguf` → `X.gguf.sceau`).
pub fn chemin_manifeste(modele: &Path) -> PathBuf {
    let mut nom = modele.as_os_str().to_os_string();
    nom.push(".");
    nom.push(EXTENSION_MANIFESTE);
    PathBuf::from(nom)
}

/// Décode une liste de clés publiques Ed25519 hexadécimales (64 caractères)
/// en ancres de niveau `Souverain`. Une seule entrée invalide = erreur.
pub fn ancres_depuis_hex(cles: &[String]) -> Result<AncresDeConfiance, ErreurProvenance> {
    let mut ancres = AncresDeConfiance::new();
    for c in cles {
        let octets = hex::decode(c.trim())
            .map_err(|e| ErreurProvenance::AncreInvalide(format!("{c} : {e}")))?;
        let cle: [u8; 32] = octets
            .try_into()
            .map_err(|_| ErreurProvenance::AncreInvalide(format!("{c} : 32 octets attendus")))?;
        ancres
            .ajouter(&cle, NiveauConfiance::Souverain)
            .map_err(|e| ErreurProvenance::AncreInvalide(format!("{c} : {e}")))?;
    }
    Ok(ancres)
}

fn maintenant_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// SHA-256 d'un fichier, lu par blocs de 1 Mio.
pub fn sha256_fichier(chemin: &Path) -> std::io::Result<[u8; 32]> {
    let mut f = std::fs::File::open(chemin)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1024 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(h.finalize().into())
}

/// Contrôle **pur** d'un manifeste face à un fichier présenté : signature et
/// signataire épinglé, genre, nom, taille, licence. L'empreinte est contrôlée
/// séparément ([`controler_empreinte`]) car elle exige de lire le fichier.
pub fn controler_manifeste(
    octets: &[u8],
    ancres: &AncresDeConfiance,
    nom_fichier: &str,
    taille: u64,
) -> Result<ManifesteModele, ErreurProvenance> {
    let verifie = verifier::<ManifesteModele>(octets, ancres, maintenant_secs())
        .map_err(ErreurProvenance::Sceau)?;
    let m = verifie.into_titre();
    if m.nom_fichier != nom_fichier {
        return Err(ErreurProvenance::NomDifferent {
            attendu: m.nom_fichier,
            trouve: nom_fichier.to_string(),
        });
    }
    if m.taille != taille {
        return Err(ErreurProvenance::TailleDifferente { attendue: m.taille, trouvee: taille });
    }
    if !LICENCES_AUTORISEES.contains(&m.licence.as_str()) {
        return Err(ErreurProvenance::LicenceRefusee(m.licence));
    }
    Ok(m)
}

/// Compare l'empreinte calculée à l'empreinte signée.
pub fn controler_empreinte(m: &ManifesteModele, calculee: &[u8; 32]) -> Result<(), ErreurProvenance> {
    if &m.sha256 == calculee {
        Ok(())
    } else {
        Err(ErreurProvenance::EmpreinteDifferente)
    }
}

/// Vérification complète d'un modèle sur disque (manifeste + empreinte).
///
/// Coûteux (hachage du fichier entier) : à appeler hors du runtime async.
pub fn verifier_modele(
    modele: &Path,
    ancres: &AncresDeConfiance,
) -> Result<ManifesteModele, ErreurProvenance> {
    let manifeste = chemin_manifeste(modele);
    let meta_manifeste = match std::fs::metadata(&manifeste) {
        Ok(m) if m.is_file() => m,
        _ => return Err(ErreurProvenance::ManifesteAbsent(manifeste)),
    };
    if meta_manifeste.len() > TAILLE_MAX_MANIFESTE {
        return Err(ErreurProvenance::ManifesteIllisible("manifeste trop gros".into()));
    }
    let octets = std::fs::read(&manifeste)
        .map_err(|e| ErreurProvenance::ManifesteIllisible(e.to_string()))?;

    let nom = modele
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .ok_or_else(|| ErreurProvenance::LectureModele("chemin sans nom de fichier".into()))?;
    let taille = std::fs::metadata(modele)
        .map_err(|e| ErreurProvenance::LectureModele(e.to_string()))?
        .len();

    // Signature vérifiée avant tout hachage (C-23) : un manifeste forgé ne
    // coûte pas la lecture de plusieurs Go.
    let m = controler_manifeste(&octets, ancres, &nom, taille)?;
    let calculee =
        sha256_fichier(modele).map_err(|e| ErreurProvenance::LectureModele(e.to_string()))?;
    controler_empreinte(&m, &calculee)?;
    Ok(m)
}

/// Politique de chargement des modèles, fixée au démarrage.
#[derive(Debug, Clone)]
pub struct PolitiqueModeles {
    ancres: AncresDeConfiance,
    derogation_dev: bool,
}

/// Issue d'un contrôle de chargement autorisé.
#[derive(Debug, Clone)]
pub enum Autorisation {
    /// Modèle vérifié (manifeste signé, empreinte conforme).
    Verifie(ManifesteModele),
    /// Modèle non vérifié, admis par la dérogation développeur.
    DerogationDev(String),
}

impl PolitiqueModeles {
    /// Construit la politique. `derogation_demandee` n'a d'effet que dans un
    /// binaire de debug ; elle est sinon ignorée (journal d'erreur).
    pub fn new(ancres: AncresDeConfiance, derogation_demandee: bool) -> Self {
        let derogation_dev = derogation_demandee && cfg!(debug_assertions);
        if derogation_demandee && !derogation_dev {
            tracing::error!(
                "Dérogation « modèles non signés » demandée mais IGNORÉE : binaire de release \
                 (EX-26). Seuls les modèles signés seront chargés."
            );
        }
        if derogation_dev {
            tracing::error!(
                "!!! DÉROGATION DÉVELOPPEUR ACTIVE : des modèles NON SIGNÉS peuvent être chargés \
                 (EX-26 non respectée). Ne jamais utiliser en production. !!!"
            );
        }
        if ancres.is_empty() {
            tracing::warn!(
                "Aucune ancre de confiance de modèles (model_trust_anchors) : \
                 aucun modèle signé ne peut être chargé."
            );
        }
        Self { ancres, derogation_dev }
    }

    /// Vrai si la dérogation développeur est effectivement active.
    pub fn derogation_active(&self) -> bool {
        self.derogation_dev
    }

    /// Autorise (ou refuse) le chargement de `modele`.
    pub fn autoriser(&self, modele: &Path) -> Result<Autorisation, ErreurProvenance> {
        match verifier_modele(modele, &self.ancres) {
            Ok(m) => {
                tracing::info!(
                    modele = %m.nom_fichier,
                    licence = %m.licence,
                    coupure = %m.date_coupure,
                    "provenance du modèle vérifiée"
                );
                Ok(Autorisation::Verifie(m))
            }
            Err(e) if self.derogation_dev => {
                tracing::error!(
                    "!!! MODÈLE NON VÉRIFIÉ CHARGÉ PAR DÉROGATION DÉVELOPPEUR : {} — {e} !!!",
                    modele.display()
                );
                Ok(Autorisation::DerogationDev(e.to_string()))
            }
            Err(e) => Err(e),
        }
    }
}

/// Vrai si la dérogation développeur est demandée (option ou variable).
pub fn derogation_demandee(args: &[String]) -> bool {
    args.iter().any(|a| a == OPTION_DEROGATION)
        || std::env::var(ENV_DEROGATION).is_ok_and(|v| v == "1")
}

// ── Outillage de signature (développement / canal officiel) ──────

/// Scelle le manifeste d'un modèle et l'écrit dans `X.gguf.sceau`.
pub fn sceller_modele(
    modele: &Path,
    cle: &CleSignature,
    licence: &str,
    date_coupure: &str,
) -> Result<PathBuf, String> {
    let nom = modele
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .ok_or("chemin sans nom de fichier")?;
    let taille = std::fs::metadata(modele).map_err(|e| e.to_string())?.len();
    let sha256 = sha256_fichier(modele).map_err(|e| e.to_string())?;
    let m = ManifesteModele {
        nom_fichier: nom,
        sha256,
        taille,
        licence: licence.to_string(),
        date_coupure: date_coupure.to_string(),
        emis_le: maintenant_secs(),
    };
    if !LICENCES_AUTORISEES.contains(&m.licence.as_str()) {
        return Err(format!("licence « {licence} » non autorisée (Apache-2.0 ou MIT)"));
    }
    let octets = sceller(&m, cle).map_err(|e| e.to_string())?;
    let sortie = chemin_manifeste(modele);
    std::fs::write(&sortie, octets).map_err(|e| e.to_string())?;
    Ok(sortie)
}

/// Lit une graine de signature (64 caractères hexadécimaux) depuis un fichier.
pub fn lire_graine(chemin: &Path) -> Result<CleSignature, String> {
    let texte = std::fs::read_to_string(chemin).map_err(|e| e.to_string())?;
    let octets = hex::decode(texte.trim()).map_err(|e| e.to_string())?;
    let graine: [u8; 32] = octets.try_into().map_err(|_| "graine : 32 octets attendus".to_string())?;
    Ok(CleSignature::depuis_graine(&graine))
}

/// Génère une graine de développement dans `sortie` (créée, jamais écrasée,
/// droits `0600` sous Unix). Renvoie la clé publique hexadécimale à épingler.
pub fn generer_graine_dev(sortie: &Path) -> Result<String, String> {
    use std::io::Write;
    let cle = CleSignature::generer();
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut f = opts.open(sortie).map_err(|e| e.to_string())?;
    f.write_all(hex::encode(cle.graine()).as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(hex::encode(cle.cle_publique()))
}

/// Sous-commandes d'outillage (`cle-dev`, `sceller-modele`). Renvoie `None`
/// si `args` ne désigne pas une sous-commande d'outillage.
pub fn executer_outil(args: &[String]) -> Option<Result<String, String>> {
    let valeur = |nom: &str| {
        args.iter()
            .position(|a| a == nom)
            .and_then(|i| args.get(i + 1))
            .map(PathBuf::from)
    };
    let texte = |nom: &str| {
        args.iter()
            .position(|a| a == nom)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    match args.get(1).map(String::as_str) {
        Some("cle-dev") => Some((|| -> Result<String, String> {
            let sortie = valeur("--sortie").ok_or("usage : maia-llm cle-dev --sortie <fichier>")?;
            let publique = generer_graine_dev(&sortie)?;
            Ok(format!(
                "Graine écrite dans {} (secrète, 0600).\nClé publique à épingler dans \
                 model_trust_anchors : {publique}",
                sortie.display()
            ))
        })()),
        Some("sceller-modele") => Some((|| -> Result<String, String> {
            const USAGE: &str = "usage : maia-llm sceller-modele --graine <fichier> --modele <X.gguf> \
                                 --licence <Apache-2.0|MIT> --coupure <AAAA-MM>";
            let graine = valeur("--graine").ok_or(USAGE)?;
            let modele = valeur("--modele").ok_or(USAGE)?;
            let licence = texte("--licence").ok_or(USAGE)?;
            let coupure = texte("--coupure").ok_or(USAGE)?;
            let cle = lire_graine(&graine)?;
            let sortie = sceller_modele(&modele, &cle, &licence, &coupure)?;
            Ok(format!(
                "Manifeste écrit : {} (signataire {})",
                sortie.display(),
                hex::encode(cle.cle_publique())
            ))
        })()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dossier() -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "maia-llm-prov-{}-{}",
            std::process::id(),
            rand::random::<u64>()
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn cle() -> CleSignature {
        CleSignature::depuis_graine(&[9u8; 32])
    }

    fn ancres(c: &CleSignature) -> AncresDeConfiance {
        ancres_depuis_hex(&[hex::encode(c.cle_publique())]).unwrap()
    }

    fn modele(d: &Path) -> PathBuf {
        let p = d.join("qwen-mini.gguf");
        std::fs::write(&p, b"GGUF\x03\x00\x00\x00 faux modele de test").unwrap();
        p
    }

    #[test]
    fn manifeste_valide_accepte() {
        let d = dossier();
        let p = modele(&d);
        let c = cle();
        sceller_modele(&p, &c, "Apache-2.0", "2025-06").unwrap();
        let m = verifier_modele(&p, &ancres(&c)).unwrap();
        assert_eq!(m.nom_fichier, "qwen-mini.gguf");
        assert_eq!(m.licence, "Apache-2.0");
        assert_eq!(m.date_coupure, "2025-06");
        assert_eq!(m.sha256, sha256_fichier(&p).unwrap());
        let pol = PolitiqueModeles::new(ancres(&c), false);
        assert!(matches!(pol.autoriser(&p), Ok(Autorisation::Verifie(_))));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn fichier_modifie_refuse() {
        let d = dossier();
        let p = modele(&d);
        let c = cle();
        sceller_modele(&p, &c, "MIT", "2025-06").unwrap();
        // Même taille, contenu différent : seule l'empreinte le détecte.
        let mut octets = std::fs::read(&p).unwrap();
        let dernier = octets.len() - 1;
        octets[dernier] ^= 0xFF;
        std::fs::write(&p, &octets).unwrap();
        assert!(matches!(
            verifier_modele(&p, &ancres(&c)),
            Err(ErreurProvenance::EmpreinteDifferente)
        ));
        // Taille différente.
        octets.push(0);
        std::fs::write(&p, &octets).unwrap();
        assert!(matches!(
            verifier_modele(&p, &ancres(&c)),
            Err(ErreurProvenance::TailleDifferente { .. })
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn empreinte_signee_falsifiee_refusee() {
        let c = cle();
        let m = ManifesteModele {
            nom_fichier: "a.gguf".into(),
            sha256: [1u8; 32],
            taille: 3,
            licence: "MIT".into(),
            date_coupure: "2025-06".into(),
            emis_le: 1,
        };
        let mut octets = sceller(&m, &c).unwrap();
        // Altération d'un octet de l'empreinte dans la charge signée.
        let pos = octets.windows(32).position(|w| w == [1u8; 32]).unwrap();
        octets[pos] ^= 1;
        assert!(matches!(
            controler_manifeste(&octets, &ancres(&c), "a.gguf", 3),
            Err(ErreurProvenance::Sceau(ErreurSceau::SignatureInvalide))
        ));
        // Empreinte calculée différente de l'empreinte signée.
        let ok = sceller(&m, &c).unwrap();
        let mv = controler_manifeste(&ok, &ancres(&c), "a.gguf", 3).unwrap();
        assert!(controler_empreinte(&mv, &[2u8; 32]).is_err());
        assert!(controler_empreinte(&mv, &[1u8; 32]).is_ok());
    }

    #[test]
    fn signataire_inconnu_refuse() {
        let d = dossier();
        let p = modele(&d);
        sceller_modele(&p, &CleSignature::depuis_graine(&[3u8; 32]), "MIT", "2025-06").unwrap();
        assert!(matches!(
            verifier_modele(&p, &ancres(&cle())),
            Err(ErreurProvenance::Sceau(ErreurSceau::SignataireInconnu))
        ));
        // Aucune ancre : tout est refusé.
        assert!(matches!(
            verifier_modele(&p, &AncresDeConfiance::new()),
            Err(ErreurProvenance::Sceau(ErreurSceau::SignataireInconnu))
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn manifeste_absent_refuse() {
        let d = dossier();
        let p = modele(&d);
        // Un ancien sidecar SHA-256 non signé ne suffit pas.
        std::fs::write(d.join("qwen-mini.gguf.sha256"), "00".repeat(32)).unwrap();
        assert!(matches!(
            verifier_modele(&p, &ancres(&cle())),
            Err(ErreurProvenance::ManifesteAbsent(_))
        ));
        let pol = PolitiqueModeles::new(ancres(&cle()), false);
        assert!(pol.autoriser(&p).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn manifeste_d_un_autre_fichier_refuse() {
        let d = dossier();
        let p = modele(&d);
        let c = cle();
        sceller_modele(&p, &c, "MIT", "2025-06").unwrap();
        let autre = d.join("autre.gguf");
        std::fs::copy(&p, &autre).unwrap();
        std::fs::copy(chemin_manifeste(&p), chemin_manifeste(&autre)).unwrap();
        assert!(matches!(
            verifier_modele(&autre, &ancres(&c)),
            Err(ErreurProvenance::NomDifferent { .. })
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn licence_non_autorisee_refusee() {
        let c = cle();
        let m = ManifesteModele {
            nom_fichier: "a.gguf".into(),
            sha256: [0u8; 32],
            taille: 1,
            licence: "Llama-3-Community".into(),
            date_coupure: "2024-12".into(),
            emis_le: 1,
        };
        let octets = sceller(&m, &c).unwrap();
        assert!(matches!(
            controler_manifeste(&octets, &ancres(&c), "a.gguf", 1),
            Err(ErreurProvenance::LicenceRefusee(_))
        ));
    }

    #[test]
    fn genre_et_type_imposes() {
        // Un sceau d'un autre type (ex. passeport) est refusé par le type.
        struct Autre;
        impl Titre for Autre {
            const TYPE: TypeSceau = TypeSceau::Passeport;
            fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
                e.texte(1, GENRE_MODELE)?;
                Ok(())
            }
            fn decoder(_: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
                Ok(Self)
            }
        }
        let c = cle();
        let octets = sceller(&Autre, &c).unwrap();
        assert!(matches!(
            controler_manifeste(&octets, &ancres(&c), "a.gguf", 1),
            Err(ErreurProvenance::Sceau(ErreurSceau::TypeInattendu { .. }))
        ));
        // Un manifeste de paquet d'un autre genre est refusé par le genre.
        struct Paquet;
        impl Titre for Paquet {
            const TYPE: TypeSceau = TypeSceau::ManifestePaquet;
            fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
                e.texte(1, "service/paquet")?;
                Ok(())
            }
            fn decoder(_: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
                Ok(Self)
            }
        }
        let octets = sceller(&Paquet, &c).unwrap();
        assert!(matches!(
            controler_manifeste(&octets, &ancres(&c), "a.gguf", 1),
            Err(ErreurProvenance::Sceau(ErreurSceau::ValeurInvalide(1)))
        ));
    }

    #[test]
    fn ancres_invalides_refusees() {
        assert!(ancres_depuis_hex(&["zz".into()]).is_err());
        assert!(ancres_depuis_hex(&["00".repeat(31)]).is_err());
        assert!(ancres_depuis_hex(&[]).unwrap().is_empty());
    }

    #[test]
    fn derogation_seulement_en_debug() {
        let d = dossier();
        let p = modele(&d);
        let pol = PolitiqueModeles::new(AncresDeConfiance::new(), true);
        assert_eq!(pol.derogation_active(), cfg!(debug_assertions));
        let r = pol.autoriser(&p);
        if cfg!(debug_assertions) {
            assert!(matches!(r, Ok(Autorisation::DerogationDev(_))));
        } else {
            assert!(r.is_err());
        }
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn outil_cle_dev_puis_scellement() {
        let d = dossier();
        let p = modele(&d);
        let graine = d.join("cle.hex");
        let publique = generer_graine_dev(&graine).unwrap();
        assert!(generer_graine_dev(&graine).is_err(), "jamais écrasée");
        let args: Vec<String> = [
            "maia-llm", "sceller-modele", "--graine", graine.to_str().unwrap(),
            "--modele", p.to_str().unwrap(), "--licence", "MIT", "--coupure", "2025-01",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        assert!(executer_outil(&args).unwrap().is_ok());
        let a = ancres_depuis_hex(&[publique]).unwrap();
        assert!(verifier_modele(&p, &a).is_ok());
        assert!(executer_outil(&["maia-llm".into(), "serve".into()]).is_none());
        let _ = std::fs::remove_dir_all(&d);
    }
}
