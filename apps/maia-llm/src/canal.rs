//! @id     maia_llm_canal
//! @do     installer_modele_canal_officiel
//! @role   securite
//! @layer  app
//! @human  maia-llm — installateur des modèles IA reçus par le canal officiel (EX-26, lot 4 : catégorie ModeleIa).

//! Installateur du canal officiel pour la catégorie **ModeleIa** (EX-26,
//! amendement A-006 ; `crates/miyucanal`).
//!
//! Le canal remet une [`MiseAJourVerifiee`] : annonce 2 sur 2 {Origin,
//! éditeur}, quarantaine écoulée, artefact dont taille et SHA-256 sont ceux de
//! l'annonce. L'installateur ajoute, **par-dessus** (défense en profondeur), la
//! provenance existante du modèle ([`crate::provenance`]) : manifeste signé par
//! une ancre `model_trust_anchors`, nom, taille, licence et SHA-256 du fichier.
//!
//! # Artefact « modèle » (non spécifié par la doc gelée, à amender)
//!
//! ```text
//! "MMIA" | version 0x01 | longueur du manifeste (4, BE, 1..=128 Kio)
//!        | manifeste signé (Sceau ManifesteModele) | octets du modèle
//! ```
//!
//! # Écriture
//!
//! Tout est vérifié **en mémoire** avant la moindre écriture ; puis fichier
//! temporaire (création exclusive, `0644`, `fsync`) et renommage atomique, le
//! modèle d'abord, son manifeste `X.gguf.sceau` ensuite (une coupure entre les
//! deux laisse un couple incohérent, refusé au chargement : fail-closed). Le
//! contrôle de chargement existant ([`verifier_modele`]) est rejoué sur le
//! fichier final avant d'inscrire la version.
//!
//! Versions installées : index `.canal-modeles.json` du dossier des modèles
//! (`composant → version, fichier`). Index illisible ⇒ toute version est
//! considérée installée au maximum (aucune installation, fail-closed).

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use miyucanal::{Installateur, MiseAJourVerifiee};
use miyuregistre::Categorie;
use miyusceau::{verifier, AncresDeConfiance};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::provenance::{chemin_manifeste, controler_empreinte, controler_manifeste, verifier_modele, ManifesteModele};

/// Magic de l'artefact « modèle » du canal officiel.
pub const MAGIC_ARTEFACT: &[u8; 4] = b"MMIA";
/// Version de l'artefact.
pub const VERSION_ARTEFACT: u8 = 0x01;
/// Index des versions installées par le canal (dans le dossier des modèles).
pub const FICHIER_INDEX: &str = ".canal-modeles.json";
/// Taille maximale du manifeste embarqué (même borne que la lecture sur disque).
const TAILLE_MAX_MANIFESTE: usize = 128 * 1024;
/// Droits des fichiers modèles (lecture pour tous, écriture pour le service seul).
const MODE_MODELE: u32 = 0o644;

/// Encode un artefact « modèle » (outillage, tests).
pub fn encoder_artefact(manifeste: &[u8], modele: &[u8]) -> Vec<u8> {
    let mut s = Vec::with_capacity(9 + manifeste.len() + modele.len());
    s.extend_from_slice(MAGIC_ARTEFACT);
    s.push(VERSION_ARTEFACT);
    s.extend_from_slice(&(manifeste.len() as u32).to_be_bytes());
    s.extend_from_slice(manifeste);
    s.extend_from_slice(modele);
    s
}

/// Décodage strict : (manifeste, modèle).
fn decoder_artefact(octets: &[u8]) -> Result<(&[u8], &[u8]), String> {
    let refus = || "artefact modèle mal formé".to_string();
    if octets.len() < 9 || &octets[0..4] != MAGIC_ARTEFACT || octets[4] != VERSION_ARTEFACT {
        return Err(refus());
    }
    let n = u32::from_be_bytes([octets[5], octets[6], octets[7], octets[8]]) as usize;
    if n == 0 || n > TAILLE_MAX_MANIFESTE {
        return Err(refus());
    }
    let manifeste = octets.get(9..9 + n).ok_or_else(refus)?;
    let modele = &octets[9 + n..];
    if modele.is_empty() {
        return Err(refus());
    }
    Ok((manifeste, modele))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EntreeIndex {
    version: u64,
    fichier: String,
}

/// Installateur des modèles IA du canal officiel.
#[derive(Debug, Clone)]
pub struct InstallateurModeles {
    dossier: PathBuf,
    ancres: AncresDeConfiance,
}

impl InstallateurModeles {
    /// Installe dans `dossier` (dossier des modèles), manifestes vérifiés contre `ancres`
    /// (`model_trust_anchors`). Aucune ancre ⇒ toute installation refusée.
    pub fn new(dossier: impl Into<PathBuf>, ancres: AncresDeConfiance) -> Self {
        Self { dossier: dossier.into(), ancres }
    }

    fn chemin_index(&self) -> PathBuf {
        self.dossier.join(FICHIER_INDEX)
    }

    /// Index des versions ; absent = vide ; illisible = erreur.
    fn lire_index(&self) -> Result<BTreeMap<String, EntreeIndex>, String> {
        match std::fs::read(self.chemin_index()) {
            Ok(o) => serde_json::from_slice(&o).map_err(|e| format!("index des modèles illisible : {e}")),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(BTreeMap::new()),
            Err(e) => Err(format!("index des modèles illisible : {e}")),
        }
    }

    /// Vérifie puis écrit le modèle reçu du canal (voir module).
    pub fn installer_artefact(&self, composant: &str, version: u64, artefact: &[u8]) -> Result<PathBuf, String> {
        let mut index = self.lire_index()?;
        if let Some(e) = index.get(composant) {
            if version <= e.version {
                return Err(format!("modèle « {composant} » : version {version} non supérieure à {}", e.version));
            }
        }
        let (manifeste, modele) = decoder_artefact(artefact)?;
        // Signature du manifeste d'abord (ancre épinglée), puis nom, taille, licence, empreinte.
        let m: ManifesteModele = verifier::<ManifesteModele>(manifeste, &self.ancres, maintenant_secs())
            .map_err(|e| format!("manifeste du modèle refusé : {e}"))?
            .into_titre();
        if m.nom_fichier.starts_with('.') || m.nom_fichier.to_ascii_lowercase().ends_with(".sceau") {
            return Err(format!("nom de fichier modèle refusé : « {} »", m.nom_fichier));
        }
        let m = controler_manifeste(manifeste, &self.ancres, &m.nom_fichier, modele.len() as u64)
            .map_err(|e| e.to_string())?;
        let calculee: [u8; 32] = Sha256::digest(modele).into();
        controler_empreinte(&m, &calculee).map_err(|e| e.to_string())?;

        std::fs::create_dir_all(&self.dossier).map_err(|e| format!("dossier des modèles : {e}"))?;
        let final_modele = self.dossier.join(&m.nom_fichier);
        ecrire_atomique(&final_modele, modele, MODE_MODELE)?;
        ecrire_atomique(&chemin_manifeste(&final_modele), manifeste, MODE_MODELE)?;
        synchroniser_dossier(&self.dossier);
        // Contrôle de chargement existant, rejoué sur le fichier final.
        verifier_modele(&final_modele, &self.ancres).map_err(|e| format!("modèle écrit refusé au contrôle : {e}"))?;

        index.insert(composant.to_string(), EntreeIndex { version, fichier: m.nom_fichier.clone() });
        let json = serde_json::to_vec_pretty(&index).map_err(|e| e.to_string())?;
        ecrire_atomique(&self.chemin_index(), &json, MODE_MODELE)?;
        tracing::info!(composant, version, fichier = %m.nom_fichier, "modèle installé par le canal officiel");
        Ok(final_modele)
    }
}

impl Installateur for InstallateurModeles {
    fn version_installee(&self, categorie: Categorie, composant: &str) -> Option<u64> {
        if categorie != Categorie::ModeleIa {
            return None;
        }
        match self.lire_index() {
            Ok(i) => i.get(composant).map(|e| e.version),
            Err(e) => {
                tracing::error!("{e} : aucune installation de modèle (fail-closed)");
                Some(u64::MAX)
            }
        }
    }

    fn installer(&mut self, maj: &MiseAJourVerifiee) -> Result<(), String> {
        if maj.categorie() != Categorie::ModeleIa {
            return Err(format!("catégorie {:?} : pas un modèle IA", maj.categorie()));
        }
        self.installer_artefact(maj.composant(), maj.version(), maj.artefact()).map(|_| ())
    }
}

fn maintenant_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Fichier temporaire du même dossier (création exclusive, `mode`), `fsync`, renommage.
fn ecrire_atomique(cible: &Path, octets: &[u8], mode: u32) -> Result<(), String> {
    let parent = cible.parent().ok_or("chemin sans dossier parent")?;
    let nom = cible.file_name().ok_or("chemin sans nom de fichier")?.to_string_lossy();
    let tmp = parent.join(format!(".{nom}.canal-{}-{:016x}", std::process::id(), rand::random::<u64>()));
    let resultat = (|| -> std::io::Result<()> {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(mode);
        }
        let mut f = options.open(&tmp)?;
        f.write_all(octets)?;
        #[cfg(unix)]
        {
            // Droits exacts, indépendants de l'umask (jamais d'écriture groupe / autres).
            use std::os::unix::fs::PermissionsExt;
            f.set_permissions(std::fs::Permissions::from_mode(mode))?;
        }
        f.sync_all()?;
        drop(f);
        std::fs::rename(&tmp, cible)
    })();
    if let Err(e) = resultat {
        let _ = std::fs::remove_file(&tmp);
        return Err(format!("écriture de {} : {e}", cible.display()));
    }
    #[cfg(not(unix))]
    let _ = mode;
    Ok(())
}

fn synchroniser_dossier(dossier: &Path) {
    #[cfg(unix)]
    if let Ok(d) = std::fs::File::open(dossier) {
        let _ = d.sync_all();
    }
    #[cfg(not(unix))]
    let _ = dossier;
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyucanal::banc::BancCanal;
    use miyucanal::{Evenement, Releveur, QUARANTAINE_SECS};
    use miyuregistre::{HorlogeConfiance, StockageMemoire};
    use miyusceau::{sceller, CleSignature, NiveauConfiance};

    const T0: u64 = 1_800_000_000;
    const H: u64 = 3_600;

    fn dossier() -> PathBuf {
        let d = std::env::temp_dir().join(format!("maia-llm-canal-{}-{}", std::process::id(), rand::random::<u64>()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn cle() -> CleSignature {
        CleSignature::depuis_graine(&[0x2a; 32])
    }

    fn ancres() -> AncresDeConfiance {
        let mut a = AncresDeConfiance::new();
        a.ajouter(&cle().cle_publique(), NiveauConfiance::Souverain).unwrap();
        a
    }

    fn artefact(nom: &str, modele: &[u8]) -> Vec<u8> {
        let m = ManifesteModele {
            nom_fichier: nom.into(),
            sha256: Sha256::digest(modele).into(),
            taille: modele.len() as u64,
            licence: "Apache-2.0".into(),
            date_coupure: "2025-06".into(),
            emis_le: maintenant_secs(),
        };
        encoder_artefact(&sceller(&m, &cle()).unwrap(), modele)
    }

    #[test]
    fn modele_ecrit_atomiquement_avec_son_manifeste() {
        let d = dossier();
        let inst = InstallateurModeles::new(&d, ancres());
        let modele = b"GGUF\x03 modele du canal v2".to_vec();
        let p = inst.installer_artefact("qwen-mini", 2, &artefact("qwen-mini.gguf", &modele)).unwrap();
        assert_eq!(std::fs::read(&p).unwrap(), modele);
        assert!(chemin_manifeste(&p).is_file());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for f in [p.clone(), chemin_manifeste(&p)] {
                assert_eq!(std::fs::metadata(&f).unwrap().permissions().mode() & 0o777, 0o644);
            }
        }
        // Le contrôle de chargement existant accepte le couple écrit.
        assert!(verifier_modele(&p, &ancres()).is_ok());
        assert_eq!(inst.version_installee(Categorie::ModeleIa, "qwen-mini"), Some(2));
        assert_eq!(inst.version_installee(Categorie::Cores, "qwen-mini"), None);
        // Aucun fichier temporaire laissé.
        let restes: Vec<_> = std::fs::read_dir(&d)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| e.file_name() != FICHIER_INDEX && e.file_name().to_string_lossy().contains(".canal-"))
            .collect();
        assert!(restes.is_empty());
        // Rétrogradation refusée.
        assert!(inst.installer_artefact("qwen-mini", 2, &artefact("qwen-mini.gguf", &modele)).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn modele_altere_ou_manifeste_etranger_refuse_sans_ecriture() {
        let d = dossier();
        let inst = InstallateurModeles::new(&d, ancres());
        let modele = b"GGUF\x03 modele authentique".to_vec();
        let bon = artefact("m.gguf", &modele);
        // Octets du modèle altérés (même taille) : empreinte différente.
        let mut altere = bon.clone();
        let n = altere.len();
        altere[n - 1] ^= 1;
        assert!(inst.installer_artefact("m", 1, &altere).unwrap_err().contains("empreinte"));
        // Taille différente.
        let mut long = bon.clone();
        long.push(0);
        assert!(inst.installer_artefact("m", 1, &long).unwrap_err().contains("taille"));
        // Manifeste signé par une clé non épinglée.
        let autre = InstallateurModeles::new(&d, {
            let mut a = AncresDeConfiance::new();
            a.ajouter(&CleSignature::depuis_graine(&[1; 32]).cle_publique(), NiveauConfiance::Souverain).unwrap();
            a
        });
        assert!(autre.installer_artefact("m", 1, &bon).is_err());
        // Artefact mal formé, nom réservé.
        assert!(inst.installer_artefact("m", 1, b"MMIA\x01").is_err());
        assert!(inst.installer_artefact("m", 1, &artefact("x.sceau", &modele)).is_err());
        assert!(std::fs::read_dir(&d).unwrap().next().is_none(), "rien ne doit être écrit");
        // Index illisible : fail-closed.
        std::fs::write(d.join(FICHIER_INDEX), b"{").unwrap();
        assert_eq!(inst.version_installee(Categorie::ModeleIa, "m"), Some(u64::MAX));
        assert!(inst.installer_artefact("m", 1, &bon).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn canal_officiel_de_bout_en_bout() {
        let d = dossier();
        let mut banc = BancCanal::new([0x4d; 32]);
        let mut releveur = Releveur::new(banc.config(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();
        let mut inst = InstallateurModeles::new(&d, ancres());
        let modele = b"GGUF\x03 modele officiel v1".to_vec();
        banc.publier(Categorie::ModeleIa, "qwen-mini", 1, &artefact("qwen-mini.gguf", &modele), T0);
        let mut installees = 0;
        for t in [T0, T0 + 24 * H, T0 + QUARANTAINE_SECS + 1, T0 + QUARANTAINE_SECS + 2] {
            banc.emettre(t);
            let ev = releveur.cycle(&mut banc.depot, &mut inst, &mut horloge, t).unwrap();
            installees += ev.iter().filter(|e| matches!(e, Evenement::Installee { .. })).count();
        }
        assert_eq!(installees, 1);
        assert_eq!(std::fs::read(d.join("qwen-mini.gguf")).unwrap(), modele);
        assert_eq!(inst.version_installee(Categorie::ModeleIa, "qwen-mini"), Some(1));
        let _ = std::fs::remove_dir_all(&d);
    }
}
