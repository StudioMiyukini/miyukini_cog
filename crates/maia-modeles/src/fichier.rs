//! @id     maia_modeles_fichier
//! @do     verifier_modele_sur_disque
//! @role   securite
//! @layer  toolkit
//! @human  maia-modeles — vérification d'un modèle sur disque : ouverture unique sans lien symbolique, droits, hachage du descripteur, détection de substitution (TOCTOU).

//! Vérification d'un modèle **sur disque**.
//!
//! - [`ModeleOuvert`] : fichier ouvert une seule fois, sans suivre de lien
//!   symbolique, droits contrôlés (Unix), identité relevée (TOCTOU) ;
//! - [`verifier_modele`] / [`verifier_modele_ouvert`] : manifeste voisin
//!   `X.gguf.sceau` puis empreinte des octets du descripteur ;
//! - [`verifier_modele_sous_nom`] : même contrôle pour un fichier dont le nom
//!   sur disque n'est pas (encore) le nom signé — fichier temporaire de
//!   l'installateur avant renommage.
//!
//! La signature est toujours vérifiée **avant** le hachage (C-23) : un
//! manifeste forgé ne coûte pas la lecture de plusieurs Go.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use miyusceau::AncresDeConfiance;
use sha2::{Digest, Sha256};

use crate::artefact::TAILLE_MAX_MANIFESTE;
use crate::manifeste::{
    controler_empreinte, controler_manifeste, ErreurProvenance, ManifesteModele,
    EXTENSION_MANIFESTE,
};

/// Chemin du manifeste associé à un modèle (`X.gguf` → `X.gguf.sceau`).
pub fn chemin_manifeste(modele: &Path) -> PathBuf {
    let mut nom = modele.as_os_str().to_os_string();
    nom.push(".");
    nom.push(EXTENSION_MANIFESTE);
    PathBuf::from(nom)
}

/// Lit un manifeste sur disque, borné à [`TAILLE_MAX_MANIFESTE`] octets.
pub fn lire_manifeste(chemin: &Path) -> Result<Vec<u8>, ErreurProvenance> {
    match std::fs::metadata(chemin) {
        Ok(m) if m.is_file() => {}
        _ => return Err(ErreurProvenance::ManifesteAbsent(chemin.to_path_buf())),
    }
    let illisible = |e: std::io::Error| ErreurProvenance::ManifesteIllisible(e.to_string());
    let f = File::open(chemin).map_err(illisible)?;
    let mut octets = Vec::new();
    // Lecture bornée même si le fichier grossit après le contrôle de taille.
    let borne = u64::try_from(TAILLE_MAX_MANIFESTE)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    f.take(borne).read_to_end(&mut octets).map_err(illisible)?;
    if octets.len() > TAILLE_MAX_MANIFESTE {
        return Err(ErreurProvenance::ManifesteIllisible(
            "manifeste trop gros".into(),
        ));
    }
    Ok(octets)
}

/// SHA-256 d'un fichier, lu par blocs de 1 Mio.
pub fn sha256_fichier(chemin: &Path) -> std::io::Result<[u8; 32]> {
    sha256_lecteur(&mut File::open(chemin)?)
}

fn sha256_lecteur(f: &mut impl Read) -> std::io::Result<[u8; 32]> {
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1024 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        let Some(bloc) = buf.get(..n).filter(|b| !b.is_empty()) else {
            break;
        };
        h.update(bloc);
    }
    Ok(h.finalize().into())
}

/// Identité d'un fichier ouvert : un changement révèle un remplacement ou une
/// réécriture entre le contrôle et le chargement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IdentiteFichier {
    dev: u64,
    ino: u64,
    taille: u64,
    mtime_ns: i128,
    /// Date de changement d'inode : non réglable par l'utilisateur (une
    /// réécriture qui restaure la mtime la modifie quand même).
    ctime_ns: i128,
}

impl IdentiteFichier {
    fn de(meta: &std::fs::Metadata) -> Self {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            Self {
                dev: meta.dev(),
                ino: meta.ino(),
                taille: meta.len(),
                mtime_ns: i128::from(meta.mtime()) * 1_000_000_000 + i128::from(meta.mtime_nsec()),
                ctime_ns: i128::from(meta.ctime()) * 1_000_000_000 + i128::from(meta.ctime_nsec()),
            }
        }
        #[cfg(not(unix))]
        {
            let mtime_ns = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map_or(0, |d| i128::try_from(d.as_nanos()).unwrap_or(i128::MAX));
            Self {
                dev: 0,
                ino: 0,
                taille: meta.len(),
                mtime_ns,
                ctime_ns: 0,
            }
        }
    }
}

/// Fichier modèle ouvert **une seule fois**, sans suivre de lien symbolique.
///
/// - Le hachage lit ce descripteur ; sous Linux le chargement passe par
///   `/proc/self/fd/N`, donc par le même inode, même si le chemin est remplacé.
/// - [`ModeleOuvert::inchange`] compare l'identité (périphérique, inode,
///   taille, dates) à celle relevée à l'ouverture : à appeler juste avant et
///   juste après le chargement.
#[derive(Debug)]
pub struct ModeleOuvert {
    fichier: File,
    chemin: PathBuf,
    identite: IdentiteFichier,
}

impl ModeleOuvert {
    /// Ouvre `chemin` en refusant les liens symboliques et les fichiers non réguliers.
    pub fn ouvrir(chemin: &Path) -> Result<Self, ErreurProvenance> {
        let lecture = |e: std::io::Error| ErreurProvenance::LectureModele(e.to_string());
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        #[cfg(not(unix))]
        if std::fs::symlink_metadata(chemin)
            .map_err(lecture)?
            .file_type()
            .is_symlink()
        {
            return Err(ErreurProvenance::LectureModele(
                "lien symbolique refusé".into(),
            ));
        }
        let fichier = options.open(chemin).map_err(lecture)?;
        let meta = fichier.metadata().map_err(lecture)?;
        if !meta.is_file() {
            return Err(ErreurProvenance::LectureModele(
                "pas un fichier régulier".into(),
            ));
        }
        #[cfg(unix)]
        {
            // Personne d'autre que le service (ou root) ne doit pouvoir réécrire
            // le modèle pendant son usage.
            use std::os::unix::fs::MetadataExt;
            // UID et GID effectifs du service (toutes cibles Unix).
            let euid = rustix::process::geteuid().as_raw();
            let egid = rustix::process::getegid().as_raw();
            if meta.mode() & 0o002 != 0 {
                return Err(ErreurProvenance::LectureModele(
                    "modèle modifiable par tous (chmod o-w requis)".into(),
                ));
            }
            // Écriture de groupe admise seulement pour le groupe du service
            // (groupe privé d'utilisateur, umask 002).
            if meta.mode() & 0o020 != 0 && meta.gid() != egid {
                return Err(ErreurProvenance::LectureModele(
                    "modèle modifiable par un autre groupe (chmod g-w requis)".into(),
                ));
            }
            if meta.uid() != euid && meta.uid() != 0 {
                return Err(ErreurProvenance::LectureModele(
                    "modèle appartenant à un autre utilisateur que le service ou root".into(),
                ));
            }
        }
        Ok(Self {
            fichier,
            chemin: chemin.to_path_buf(),
            identite: IdentiteFichier::de(&meta),
        })
    }

    /// Chemin d'ouverture.
    pub fn chemin(&self) -> &Path {
        &self.chemin
    }

    /// Taille relevée à l'ouverture.
    pub fn taille(&self) -> u64 {
        self.identite.taille
    }

    fn sha256(&mut self) -> Result<[u8; 32], ErreurProvenance> {
        let lecture = |e: std::io::Error| ErreurProvenance::LectureModele(e.to_string());
        self.fichier.seek(SeekFrom::Start(0)).map_err(lecture)?;
        let h = sha256_lecteur(&mut self.fichier).map_err(lecture)?;
        self.inchange()?;
        Ok(h)
    }

    /// Refuse si le fichier ouvert a changé depuis l'ouverture.
    pub fn inchange(&self) -> Result<(), ErreurProvenance> {
        let meta = self
            .fichier
            .metadata()
            .map_err(|e| ErreurProvenance::LectureModele(e.to_string()))?;
        if IdentiteFichier::de(&meta) != self.identite {
            return Err(ErreurProvenance::ModeleModifie);
        }
        // Le chemin doit toujours désigner le fichier vérifié (lien symbolique
        // ou remplacement ⇒ refus) : indispensable hors Linux, où le chargeur
        // reçoit le chemin et non le descripteur.
        match std::fs::symlink_metadata(&self.chemin) {
            Ok(p) if !p.file_type().is_symlink() && IdentiteFichier::de(&p) == self.identite => {
                Ok(())
            }
            _ => Err(ErreurProvenance::ModeleModifie),
        }
    }

    /// Chemin à donner au chargeur : le descripteur vérifié sous Linux, sinon
    /// le chemin d'origine (fenêtre résiduelle couverte par [`Self::inchange`]).
    pub fn chemin_de_chargement(&self) -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::io::AsRawFd;
            let p = PathBuf::from(format!("/proc/self/fd/{}", self.fichier.as_raw_fd()));
            if p.exists() {
                return p;
            }
        }
        self.chemin.clone()
    }
}

/// Vérification complète d'un modèle sur disque (manifeste voisin + empreinte).
///
/// Coûteux (hachage du fichier entier) : à appeler hors du runtime async.
pub fn verifier_modele(
    modele: &Path,
    ancres: &AncresDeConfiance,
) -> Result<ManifesteModele, ErreurProvenance> {
    let mut ouvert = ModeleOuvert::ouvrir(modele)?;
    verifier_modele_ouvert(&mut ouvert, ancres)
}

/// Vérifie le manifeste voisin (`X.gguf.sceau`) puis l'empreinte des octets du
/// descripteur ouvert. Le nom signé doit être le nom du fichier ouvert.
pub fn verifier_modele_ouvert(
    ouvert: &mut ModeleOuvert,
    ancres: &AncresDeConfiance,
) -> Result<ManifesteModele, ErreurProvenance> {
    let octets = lire_manifeste(&chemin_manifeste(&ouvert.chemin))?;
    let nom = ouvert
        .chemin
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .ok_or_else(|| ErreurProvenance::LectureModele("chemin sans nom de fichier".into()))?;
    verifier_modele_sous_nom(ouvert, &octets, &nom, ancres)
}

/// Vérifie `manifeste` (octets) contre le fichier ouvert présenté sous le nom
/// `nom_attendu` (nom final, signé), puis l'empreinte de ses octets.
///
/// Sert à contrôler un fichier **temporaire** avant son renommage : le nom
/// sur disque diffère, le nom signé doit être celui de la destination.
pub fn verifier_modele_sous_nom(
    ouvert: &mut ModeleOuvert,
    manifeste: &[u8],
    nom_attendu: &str,
    ancres: &AncresDeConfiance,
) -> Result<ManifesteModele, ErreurProvenance> {
    let m = controler_manifeste(manifeste, ancres, nom_attendu, ouvert.taille())?;
    let calculee = ouvert.sha256()?;
    controler_empreinte(&m, &calculee)?;
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::essai::{ancres, cle, dossier, modele};
    use crate::outillage::sceller_modele;
    use miyusceau::{CleSignature, ErreurSceau};

    #[test]
    fn manifeste_valide_accepte() {
        let d = dossier("fichier");
        let p = modele(&d);
        let c = cle();
        sceller_modele(&p, &c, "Apache-2.0", "2025-06").unwrap();
        let m = verifier_modele(&p, &ancres(&c)).unwrap();
        assert_eq!(m.nom_fichier, "qwen-mini.gguf");
        assert_eq!(m.licence, "Apache-2.0");
        assert_eq!(m.date_coupure, "2025-06");
        assert_eq!(m.sha256, sha256_fichier(&p).unwrap());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn verification_sous_le_nom_final_d_un_temporaire() {
        let d = dossier("fichier");
        let p = modele(&d);
        let c = cle();
        let sceau = sceller_modele(&p, &c, "MIT", "2025-06").unwrap();
        let octets = std::fs::read(&sceau).unwrap();
        let tmp = d.join(".qwen-mini.gguf.tmp");
        std::fs::rename(&p, &tmp).unwrap();
        let mut ouvert = ModeleOuvert::ouvrir(&tmp).unwrap();
        assert!(
            verifier_modele_sous_nom(&mut ouvert, &octets, "qwen-mini.gguf", &ancres(&c)).is_ok()
        );
        let mut ouvert = ModeleOuvert::ouvrir(&tmp).unwrap();
        assert!(matches!(
            verifier_modele_sous_nom(&mut ouvert, &octets, ".qwen-mini.gguf.tmp", &ancres(&c)),
            Err(ErreurProvenance::NomDifferent { .. })
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn manifeste_trop_gros_refuse() {
        let d = dossier("fichier");
        let p = modele(&d);
        std::fs::write(chemin_manifeste(&p), vec![0u8; TAILLE_MAX_MANIFESTE + 1]).unwrap();
        assert!(matches!(
            verifier_modele(&p, &ancres(&cle())),
            Err(ErreurProvenance::ManifesteIllisible(_))
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn modification_apres_controle_detectee() {
        let d = dossier("fichier");
        let p = modele(&d);
        let c = cle();
        sceller_modele(&p, &c, "Apache-2.0", "2025-06").unwrap();
        let mut ouvert = ModeleOuvert::ouvrir(&p).unwrap();
        verifier_modele_ouvert(&mut ouvert, &ancres(&c)).unwrap();
        assert!(ouvert.inchange().is_ok());
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(&p, b"GGUF\x03\x00\x00\x00 modele substitue, plus long").unwrap();
        assert!(matches!(
            ouvert.inchange(),
            Err(ErreurProvenance::ModeleModifie)
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn chargement_par_le_descripteur_verifie() {
        let d = dossier("fichier");
        let p = modele(&d);
        let c = cle();
        sceller_modele(&p, &c, "Apache-2.0", "2025-06").unwrap();
        let attendu = sha256_fichier(&p).unwrap();
        let mut ouvert = ModeleOuvert::ouvrir(&p).unwrap();
        verifier_modele_ouvert(&mut ouvert, &ancres(&c)).unwrap();
        let chemin = ouvert.chemin_de_chargement();
        assert!(chemin.starts_with("/proc/self/fd/"));
        // Le chemin d'origine est remplacé : le descripteur garde les octets vérifiés.
        std::fs::remove_file(&p).unwrap();
        std::fs::write(&p, b"autre contenu").unwrap();
        assert_eq!(sha256_fichier(&chemin).unwrap(), attendu);
        // Chemin remplacé : refus avant ou après chargement.
        assert!(matches!(
            ouvert.inchange(),
            Err(ErreurProvenance::ModeleModifie)
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[cfg(unix)]
    #[test]
    fn lien_symbolique_et_droits_larges_refuses() {
        use std::os::unix::fs::PermissionsExt;
        let d = dossier("fichier");
        let p = modele(&d);
        let lien = d.join("lien.gguf");
        std::os::unix::fs::symlink(&p, &lien).unwrap();
        assert!(matches!(
            ModeleOuvert::ouvrir(&lien),
            Err(ErreurProvenance::LectureModele(_))
        ));
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o646)).unwrap();
        assert!(matches!(
            ModeleOuvert::ouvrir(&p),
            Err(ErreurProvenance::LectureModele(_))
        ));
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(ModeleOuvert::ouvrir(&p).is_ok());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn fichier_modifie_refuse() {
        let d = dossier("fichier");
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
    fn signataire_inconnu_refuse() {
        let d = dossier("fichier");
        let p = modele(&d);
        sceller_modele(
            &p,
            &CleSignature::depuis_graine(&[3u8; 32]),
            "MIT",
            "2025-06",
        )
        .unwrap();
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
        let d = dossier("fichier");
        let p = modele(&d);
        // Un ancien sidecar SHA-256 non signé ne suffit pas.
        std::fs::write(d.join("qwen-mini.gguf.sha256"), "00".repeat(32)).unwrap();
        assert!(matches!(
            verifier_modele(&p, &ancres(&cle())),
            Err(ErreurProvenance::ManifesteAbsent(_))
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn manifeste_d_un_autre_fichier_refuse() {
        let d = dossier("fichier");
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
}
