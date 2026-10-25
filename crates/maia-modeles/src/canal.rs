//! @id     maia_modeles_canal
//! @do     installer_modele_canal_officiel
//! @role   securite
//! @layer  toolkit
//! @human  maia-modeles — installateur des modèles IA reçus par le canal officiel (catégorie ModeleIa) : nom dérivé du composant, temporaire vérifié avant renommage, retour arrière.

//! Installateur du canal officiel pour la catégorie **ModeleIa** (EX-26,
//! amendement A-006 ; `crates/miyucanal`). Fonctionnalité `canal`.
//!
//! Le canal remet une [`MiseAJourVerifiee`] : annonce 2 sur 2 {Origin,
//! éditeur}, quarantaine écoulée, artefact dont taille et SHA-256 sont ceux de
//! l'annonce. L'installateur ajoute, **par-dessus** (défense en profondeur), la
//! provenance du modèle ([`crate::manifeste`]) : manifeste signé par une ancre
//! `model_trust_anchors`, nom, taille, licence et SHA-256 du fichier.
//!
//! # Règle de nommage (composant → fichier)
//!
//! Le fichier d'un composant n'est **pas** choisi par l'annonce : il est
//! **dérivé** du composant ([`nom_fichier_du_composant`]) :
//!
//! - composant : 1 à 64 octets, premier caractère `[a-z0-9]`, suivants
//!   `[a-z0-9._-]`, sans `..` (sous-ensemble des identifiants de
//!   `miyuregistre` : `:` et majuscules exclus, aucun nom caché) ;
//! - fichier = `<composant>.gguf` ; le manifeste signé doit nommer **exactement**
//!   ce fichier, sinon refus (une annonce pour `a` ne peut pas écrire `b.gguf`).
//!
//! # Index (`.canal-modeles.json`)
//!
//! `composant → { version, fichier }`. Contrôles avant toute écriture :
//! version strictement supérieure ; l'entrée du composant désigne bien son
//! fichier dérivé ; **aucun autre composant** ne tient ce fichier. Index
//! illisible ou incohérent ⇒ refus (fail-closed) ; `version_installee` rend
//! alors `u64::MAX` (aucune installation).
//!
//! # Écriture et retour arrière
//!
//! 1. tout est vérifié **en mémoire** (artefact, signature, nom dérivé,
//!    taille, licence, empreinte) ;
//! 2. modèle et manifeste écrits dans des **temporaires** du même dossier
//!    (création exclusive, `0644`, `fsync`) ;
//! 3. le **temporaire** est vérifié comme au chargement (ouverture sans lien
//!    symbolique, droits, manifeste relu sur disque, empreinte du descripteur)
//!    sous le nom final signé ;
//! 4. l'ancien couple modèle/manifeste, s'il existe, est **conservé** (lien
//!    physique, à défaut copie) ;
//! 5. renommages atomiques (modèle, puis manifeste), contrôle de chargement
//!    ([`verifier_modele`]) rejoué sur le fichier final, puis index ;
//! 6. tout échec aux étapes 2 à 5 ⇒ temporaires supprimés et ancien couple
//!    **restauré** (ou fichiers neufs retirés s'il n'y en avait pas) ; les
//!    sauvegardes ne sont supprimées qu'après succès complet.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use miyucanal::{Installateur, MiseAJourVerifiee};
use miyuregistre::Categorie;
use miyusceau::{verifier, AncresDeConfiance};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::artefact::decoder_artefact;
pub use crate::artefact::{encoder_artefact, MAGIC_ARTEFACT, VERSION_ARTEFACT};
use crate::fichier::{
    chemin_manifeste, lire_manifeste, verifier_modele, verifier_modele_sous_nom, ModeleOuvert,
};
use crate::manifeste::{controler_empreinte, controler_manifeste, ManifesteModele};

/// Index des versions installées par le canal (dans le dossier des modèles).
pub const FICHIER_INDEX: &str = ".canal-modeles.json";
/// Extension du fichier modèle dérivé du composant.
pub const EXTENSION_MODELE: &str = ".gguf";
/// Longueur maximale d'un composant de modèle (celle de `miyuregistre`).
const COMPOSANT_MAX: usize = 64;
/// Droits des fichiers modèles (lecture pour tous, écriture pour le service seul).
const MODE_MODELE: u32 = 0o644;

/// Nom de fichier imposé au modèle du composant `composant` (voir module).
pub fn nom_fichier_du_composant(composant: &str) -> Result<String, String> {
    let octets = composant.as_bytes();
    let premier_ok = octets
        .first()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    let valide = premier_ok
        && octets.len() <= COMPOSANT_MAX
        && octets.iter().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, b'.' | b'_' | b'-')
        })
        && !composant.contains("..");
    if valide {
        Ok(format!("{composant}{EXTENSION_MODELE}"))
    } else {
        Err(format!(
            "composant de modèle « {composant} » refusé (1 à {COMPOSANT_MAX} caractères \
             [a-z0-9._-], commençant par [a-z0-9], sans « .. »)"
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EntreeIndex {
    version: u64,
    fichier: String,
}

/// Étapes de l'installation (défaillances injectées par les tests).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Etape {
    VerificationTemporaire,
    EntreRenommages,
    VerificationFinale,
    Index,
}

/// Installateur des modèles IA du canal officiel.
#[derive(Debug, Clone)]
pub struct InstallateurModeles {
    dossier: PathBuf,
    ancres: AncresDeConfiance,
    #[cfg(test)]
    defaillance: Option<Etape>,
}

impl InstallateurModeles {
    /// Installe dans `dossier` (dossier des modèles), manifestes vérifiés contre `ancres`
    /// (`model_trust_anchors`). Aucune ancre ⇒ toute installation refusée.
    pub fn new(dossier: impl Into<PathBuf>, ancres: AncresDeConfiance) -> Self {
        Self {
            dossier: dossier.into(),
            ancres,
            #[cfg(test)]
            defaillance: None,
        }
    }

    #[cfg(test)]
    fn etape(&self, e: Etape) -> Result<(), String> {
        if self.defaillance == Some(e) {
            return Err(format!("défaillance injectée : {e:?}"));
        }
        Ok(())
    }

    #[cfg(not(test))]
    #[allow(clippy::unnecessary_wraps)]
    fn etape(&self, _: Etape) -> Result<(), String> {
        Ok(())
    }

    fn chemin_index(&self) -> PathBuf {
        self.dossier.join(FICHIER_INDEX)
    }

    /// Index des versions ; absent = vide ; illisible = erreur.
    fn lire_index(&self) -> Result<BTreeMap<String, EntreeIndex>, String> {
        match std::fs::read(self.chemin_index()) {
            Ok(o) => {
                serde_json::from_slice(&o).map_err(|e| format!("index des modèles illisible : {e}"))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(BTreeMap::new()),
            Err(e) => Err(format!("index des modèles illisible : {e}")),
        }
    }

    /// Vérifie puis écrit le modèle reçu du canal (voir module).
    pub fn installer_artefact(
        &self,
        composant: &str,
        version: u64,
        artefact: &[u8],
    ) -> Result<PathBuf, String> {
        let nom = nom_fichier_du_composant(composant)?;
        let mut index = self.lire_index()?;
        controler_index(&index, composant, &nom, version)?;

        let (manifeste, modele) = decoder_artefact(artefact)?;
        // Signature du manifeste d'abord (ancre épinglée), puis nom imposé,
        // taille, licence, empreinte.
        let signe: ManifesteModele =
            verifier::<ManifesteModele>(manifeste, &self.ancres, crate::maintenant_secs())
                .map_err(|e| format!("manifeste du modèle refusé : {e}"))?
                .into_titre();
        if signe.nom_fichier != nom {
            return Err(format!(
                "modèle « {composant} » : le manifeste nomme « {} », fichier imposé « {nom} »",
                signe.nom_fichier
            ));
        }
        let taille = u64::try_from(modele.len()).map_err(|e| e.to_string())?;
        let m = controler_manifeste(manifeste, &self.ancres, &nom, taille)
            .map_err(|e| e.to_string())?;
        let calculee: [u8; 32] = Sha256::digest(modele).into();
        controler_empreinte(&m, &calculee).map_err(|e| e.to_string())?;

        std::fs::create_dir_all(&self.dossier).map_err(|e| format!("dossier des modèles : {e}"))?;
        let final_modele = self.dossier.join(&nom);
        let final_manifeste = chemin_manifeste(&final_modele);

        let mut transaction = Transaction::default();
        let resultat = self.ecrire_puis_basculer(
            &mut transaction,
            (&final_modele, modele),
            (&final_manifeste, manifeste),
            &nom,
        );
        let resultat = resultat.and_then(|()| {
            index.insert(
                composant.to_string(),
                EntreeIndex {
                    version,
                    fichier: nom.clone(),
                },
            );
            self.etape(Etape::Index)?;
            let json = serde_json::to_vec_pretty(&index).map_err(|e| e.to_string())?;
            ecrire_atomique(&self.chemin_index(), &json, MODE_MODELE)
        });
        match resultat {
            Ok(()) => {
                transaction.valider();
                synchroniser_dossier(&self.dossier);
                tracing::info!(composant, version, fichier = %nom, "modèle installé par le canal officiel");
                Ok(final_modele)
            }
            Err(e) => {
                let restauration = transaction.annuler();
                synchroniser_dossier(&self.dossier);
                match restauration {
                    Ok(()) => Err(e),
                    Err(r) => Err(format!("{e} ; RESTAURATION INCOMPLÈTE : {r}")),
                }
            }
        }
    }

    /// Étapes 2 à 5 (voir module) ; l'état à défaire est noté dans `t`.
    fn ecrire_puis_basculer(
        &self,
        t: &mut Transaction,
        (final_modele, modele): (&Path, &[u8]),
        (final_manifeste, manifeste): (&Path, &[u8]),
        nom: &str,
    ) -> Result<(), String> {
        let tmp_modele = ecrire_temporaire(final_modele, modele, MODE_MODELE)?;
        t.temporaires.push(tmp_modele.clone());
        let tmp_manifeste = ecrire_temporaire(final_manifeste, manifeste, MODE_MODELE)?;
        t.temporaires.push(tmp_manifeste.clone());

        // Le temporaire est contrôlé comme au chargement, sous le nom final signé.
        let mut ouvert = ModeleOuvert::ouvrir(&tmp_modele).map_err(|e| e.to_string())?;
        let relu = lire_manifeste(&tmp_manifeste).map_err(|e| e.to_string())?;
        verifier_modele_sous_nom(&mut ouvert, &relu, nom, &self.ancres)
            .map_err(|e| format!("modèle écrit refusé au contrôle : {e}"))?;
        drop(ouvert);
        self.etape(Etape::VerificationTemporaire)?;

        // Ancien couple conservé jusqu'au succès.
        for cible in [final_modele, final_manifeste] {
            let sauvegarde = sauvegarder(cible)?;
            t.basculements.push((cible.to_path_buf(), sauvegarde));
        }
        renommer(&tmp_modele, final_modele)?;
        self.etape(Etape::EntreRenommages)?;
        renommer(&tmp_manifeste, final_manifeste)?;
        t.temporaires.clear();
        synchroniser_dossier(&self.dossier);

        // Contrôle de chargement rejoué sur le fichier final.
        verifier_modele(final_modele, &self.ancres)
            .map_err(|e| format!("modèle écrit refusé au contrôle : {e}"))?;
        self.etape(Etape::VerificationFinale)
    }
}

/// Refuse une installation incompatible avec l'index (voir module).
fn controler_index(
    index: &BTreeMap<String, EntreeIndex>,
    composant: &str,
    nom: &str,
    version: u64,
) -> Result<(), String> {
    if let Some(e) = index.get(composant) {
        if version <= e.version {
            return Err(format!(
                "modèle « {composant} » : version {version} non supérieure à {}",
                e.version
            ));
        }
        if e.fichier != nom {
            return Err(format!(
                "index des modèles incohérent : « {composant} » tient « {} » au lieu de « {nom} »",
                e.fichier
            ));
        }
    }
    if let Some((autre, _)) = index
        .iter()
        .find(|(c, e)| c.as_str() != composant && e.fichier == nom)
    {
        return Err(format!(
            "fichier « {nom} » déjà tenu par le composant « {autre} »"
        ));
    }
    Ok(())
}

/// État à défaire en cas d'échec : temporaires, puis cibles basculées avec
/// leur sauvegarde (`None` : la cible n'existait pas).
#[derive(Debug, Default)]
struct Transaction {
    temporaires: Vec<PathBuf>,
    basculements: Vec<(PathBuf, Option<PathBuf>)>,
}

impl Transaction {
    /// Succès : les sauvegardes de l'ancien couple sont supprimées.
    fn valider(self) {
        for (_, sauvegarde) in self.basculements {
            if let Some(s) = sauvegarde {
                if let Err(e) = std::fs::remove_file(&s) {
                    tracing::warn!("sauvegarde {} non supprimée : {e}", s.display());
                }
            }
        }
    }

    /// Échec : temporaires supprimés, ancien couple restauré (ou fichiers neufs retirés).
    fn annuler(self) -> Result<(), String> {
        for tmp in self.temporaires {
            let _ = std::fs::remove_file(tmp);
        }
        let mut erreurs = Vec::new();
        for (cible, sauvegarde) in self.basculements.into_iter().rev() {
            let r = match sauvegarde {
                // Si la cible n'a pas encore été remplacée, sauvegarde et cible
                // sont deux liens du même fichier : `rename` ne fait alors rien
                // (POSIX) et la sauvegarde, redondante, est retirée.
                Some(s) => {
                    std::fs::rename(&s, &cible).and_then(|()| match std::fs::remove_file(&s) {
                        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
                        _ => Ok(()),
                    })
                }
                None => match std::fs::remove_file(&cible) {
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                    autre => autre,
                },
            };
            if let Err(e) = r {
                erreurs.push(format!("{} : {e}", cible.display()));
            }
        }
        if erreurs.is_empty() {
            Ok(())
        } else {
            tracing::error!(
                "restauration du modèle incomplète : {}",
                erreurs.join(" ; ")
            );
            Err(erreurs.join(" ; "))
        }
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
            return Err(format!(
                "catégorie {:?} : pas un modèle IA",
                maj.categorie()
            ));
        }
        self.installer_artefact(maj.composant(), maj.version(), maj.artefact())
            .map(|_| ())
    }
}

/// Nom caché unique dans le dossier de `cible` (`.<nom>.canal-<rôle>-…`).
fn chemin_voisin(cible: &Path, role: &str) -> Result<PathBuf, String> {
    static COMPTEUR: AtomicU64 = AtomicU64::new(0);
    let parent = cible.parent().ok_or("chemin sans dossier parent")?;
    let nom = cible
        .file_name()
        .ok_or("chemin sans nom de fichier")?
        .to_string_lossy();
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let instant = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    Ok(parent.join(format!(
        ".{nom}.canal-{role}-{}-{instant:08x}-{n}",
        std::process::id()
    )))
}

/// Écrit `octets` dans un temporaire voisin de `cible` (création exclusive,
/// `mode`, `fsync`) et rend son chemin ; rien ne reste en cas d'échec.
fn ecrire_temporaire(cible: &Path, octets: &[u8], mode: u32) -> Result<PathBuf, String> {
    let tmp = chemin_voisin(cible, "tmp")?;
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
        f.sync_all()
    })();
    #[cfg(not(unix))]
    let _ = mode;
    match resultat {
        Ok(()) => Ok(tmp),
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            Err(format!("écriture de {} : {e}", cible.display()))
        }
    }
}

/// Temporaire puis renommage atomique (index).
fn ecrire_atomique(cible: &Path, octets: &[u8], mode: u32) -> Result<(), String> {
    let tmp = ecrire_temporaire(cible, octets, mode)?;
    renommer(&tmp, cible).inspect_err(|_| {
        let _ = std::fs::remove_file(&tmp);
    })
}

fn renommer(de: &Path, vers: &Path) -> Result<(), String> {
    std::fs::rename(de, vers).map_err(|e| format!("renommage vers {} : {e}", vers.display()))
}

/// Conserve `cible` (si elle existe) sous un nom voisin : lien physique, à
/// défaut copie. Rend le chemin de la sauvegarde.
fn sauvegarder(cible: &Path) -> Result<Option<PathBuf>, String> {
    let meta = match std::fs::symlink_metadata(cible) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("{} : {e}", cible.display())),
    };
    if !meta.is_file() {
        return Err(format!(
            "{} : pas un fichier régulier, installation refusée",
            cible.display()
        ));
    }
    let sauvegarde = chemin_voisin(cible, "ancien")?;
    if std::fs::hard_link(cible, &sauvegarde).is_err() {
        std::fs::copy(cible, &sauvegarde).map_err(|e| {
            let _ = std::fs::remove_file(&sauvegarde);
            format!("sauvegarde de {} : {e}", cible.display())
        })?;
    }
    Ok(Some(sauvegarde))
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
    use crate::essai::dossier;
    use miyucanal::banc::BancCanal;
    use miyucanal::{Evenement, Releveur, QUARANTAINE_SECS};
    use miyuregistre::{HorlogeConfiance, StockageMemoire};
    use miyusceau::{sceller, CleSignature, NiveauConfiance};

    const T0: u64 = 1_800_000_000;
    const H: u64 = 3_600;

    fn cle() -> CleSignature {
        CleSignature::depuis_graine(&[0x2a; 32])
    }

    fn ancres() -> AncresDeConfiance {
        let mut a = AncresDeConfiance::new();
        a.ajouter(&cle().cle_publique(), NiveauConfiance::Souverain)
            .unwrap();
        a
    }

    fn artefact(nom: &str, modele: &[u8]) -> Vec<u8> {
        let m = ManifesteModele {
            nom_fichier: nom.into(),
            sha256: Sha256::digest(modele).into(),
            taille: modele.len() as u64,
            licence: "Apache-2.0".into(),
            date_coupure: "2025-06".into(),
            emis_le: crate::maintenant_secs(),
        };
        encoder_artefact(&sceller(&m, &cle()).unwrap(), modele)
    }

    /// Fichiers temporaires ou sauvegardes laissés dans `d`.
    fn restes(d: &Path) -> Vec<String> {
        std::fs::read_dir(d)
            .unwrap()
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n != FICHIER_INDEX && n.contains(".canal-"))
            .collect()
    }

    #[test]
    fn modele_ecrit_atomiquement_avec_son_manifeste() {
        let d = dossier("canal");
        let inst = InstallateurModeles::new(&d, ancres());
        let modele = b"GGUF\x03 modele du canal v2".to_vec();
        let p = inst
            .installer_artefact("qwen-mini", 2, &artefact("qwen-mini.gguf", &modele))
            .unwrap();
        assert_eq!(p, d.join("qwen-mini.gguf"));
        assert_eq!(std::fs::read(&p).unwrap(), modele);
        assert!(chemin_manifeste(&p).is_file());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for f in [p.clone(), chemin_manifeste(&p)] {
                assert_eq!(
                    std::fs::metadata(&f).unwrap().permissions().mode() & 0o777,
                    0o644
                );
            }
        }
        // Le contrôle de chargement accepte le couple écrit.
        assert!(verifier_modele(&p, &ancres()).is_ok());
        assert_eq!(
            inst.version_installee(Categorie::ModeleIa, "qwen-mini"),
            Some(2)
        );
        assert_eq!(inst.version_installee(Categorie::Cores, "qwen-mini"), None);
        assert_eq!(
            restes(&d),
            Vec::<String>::new(),
            "aucun temporaire ni sauvegarde laissé"
        );
        // Rétrogradation refusée.
        assert!(inst
            .installer_artefact("qwen-mini", 2, &artefact("qwen-mini.gguf", &modele))
            .is_err());
        // Version supérieure : remplacement, sauvegarde supprimée.
        let v3 = b"GGUF\x03 modele du canal v3, plus long".to_vec();
        inst.installer_artefact("qwen-mini", 3, &artefact("qwen-mini.gguf", &v3))
            .unwrap();
        assert_eq!(std::fs::read(&p).unwrap(), v3);
        assert!(verifier_modele(&p, &ancres()).is_ok());
        assert_eq!(restes(&d), Vec::<String>::new());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn modele_altere_ou_manifeste_etranger_refuse_sans_ecriture() {
        let d = dossier("canal");
        let inst = InstallateurModeles::new(&d, ancres());
        let modele = b"GGUF\x03 modele authentique".to_vec();
        let bon = artefact("m.gguf", &modele);
        // Octets du modèle altérés (même taille) : empreinte différente.
        let mut altere = bon.clone();
        let n = altere.len();
        altere[n - 1] ^= 1;
        assert!(inst
            .installer_artefact("m", 1, &altere)
            .unwrap_err()
            .contains("empreinte"));
        // Taille différente.
        let mut long = bon.clone();
        long.push(0);
        assert!(inst
            .installer_artefact("m", 1, &long)
            .unwrap_err()
            .contains("taille"));
        // Manifeste signé par une clé non épinglée.
        let autre = InstallateurModeles::new(&d, {
            let mut a = AncresDeConfiance::new();
            a.ajouter(
                &CleSignature::depuis_graine(&[1; 32]).cle_publique(),
                NiveauConfiance::Souverain,
            )
            .unwrap();
            a
        });
        assert!(autre.installer_artefact("m", 1, &bon).is_err());
        // Artefact mal formé, nom réservé ou caché.
        assert!(inst.installer_artefact("m", 1, b"MMIA\x01").is_err());
        assert!(inst
            .installer_artefact("x", 1, &artefact("x.sceau", &modele))
            .is_err());
        assert!(inst
            .installer_artefact("x", 1, &artefact(".x.gguf", &modele))
            .is_err());
        assert!(
            std::fs::read_dir(&d).unwrap().next().is_none(),
            "rien ne doit être écrit"
        );
        // Index illisible : fail-closed.
        std::fs::write(d.join(FICHIER_INDEX), b"{").unwrap();
        assert_eq!(
            inst.version_installee(Categorie::ModeleIa, "m"),
            Some(u64::MAX)
        );
        assert!(inst.installer_artefact("m", 1, &bon).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn nom_de_fichier_derive_du_composant() {
        assert_eq!(
            nom_fichier_du_composant("qwen-mini").unwrap(),
            "qwen-mini.gguf"
        );
        assert_eq!(
            nom_fichier_du_composant("llama3.2_1b").unwrap(),
            "llama3.2_1b.gguf"
        );
        let trop_long = "a".repeat(COMPOSANT_MAX + 1);
        for c in [
            "",
            "Qwen",
            ".cache",
            "-x",
            "a/b",
            "a\\b",
            "a:b",
            "a..b",
            "a b",
            "é",
            trop_long.as_str(),
        ] {
            assert!(nom_fichier_du_composant(c).is_err(), "{c:?}");
        }
        assert!(nom_fichier_du_composant(&"a".repeat(COMPOSANT_MAX)).is_ok());
    }

    #[test]
    fn annonce_pour_a_nommant_le_fichier_de_b_refusee() {
        let d = dossier("canal");
        let inst = InstallateurModeles::new(&d, ancres());
        let b = b"GGUF\x03 modele b".to_vec();
        inst.installer_artefact("b", 1, &artefact("b.gguf", &b))
            .unwrap();
        let index_avant = std::fs::read(d.join(FICHIER_INDEX)).unwrap();
        // Annonce authentique pour « a » dont le manifeste (signé) nomme « b.gguf ».
        let intrus = b"GGUF\x03 modele substitue".to_vec();
        let e = inst
            .installer_artefact("a", 9, &artefact("b.gguf", &intrus))
            .unwrap_err();
        assert!(e.contains("imposé"), "{e}");
        // Ni contournement par un nom quelconque.
        assert!(inst
            .installer_artefact("a", 9, &artefact("autre.gguf", &intrus))
            .is_err());
        assert_eq!(
            std::fs::read(d.join("b.gguf")).unwrap(),
            b,
            "fichier de b intact"
        );
        assert!(verifier_modele(&d.join("b.gguf"), &ancres()).is_ok());
        assert_eq!(std::fs::read(d.join(FICHIER_INDEX)).unwrap(), index_avant);
        assert_eq!(inst.version_installee(Categorie::ModeleIa, "a"), None);
        assert!(!d.join("a.gguf").exists());
        // « a » nommant son propre fichier : admis.
        inst.installer_artefact("a", 1, &artefact("a.gguf", &intrus))
            .unwrap();
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn index_incoherent_refuse() {
        let d = dossier("canal");
        let inst = InstallateurModeles::new(&d, ancres());
        let m = b"GGUF\x03 modele".to_vec();
        // Un autre composant tient déjà « a.gguf ».
        std::fs::write(
            d.join(FICHIER_INDEX),
            br#"{"z":{"version":1,"fichier":"a.gguf"}}"#,
        )
        .unwrap();
        assert!(inst
            .installer_artefact("a", 1, &artefact("a.gguf", &m))
            .unwrap_err()
            .contains("déjà tenu"));
        // L'entrée de « a » désigne un autre fichier.
        std::fs::write(
            d.join(FICHIER_INDEX),
            br#"{"a":{"version":1,"fichier":"b.gguf"}}"#,
        )
        .unwrap();
        assert!(inst
            .installer_artefact("a", 2, &artefact("a.gguf", &m))
            .unwrap_err()
            .contains("incohérent"));
        assert!(!d.join("a.gguf").exists());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn echec_apres_ecriture_conserve_l_ancien_modele() {
        for etape in [
            Etape::VerificationTemporaire,
            Etape::EntreRenommages,
            Etape::VerificationFinale,
            Etape::Index,
        ] {
            let d = dossier("canal");
            let inst = InstallateurModeles::new(&d, ancres());
            let v1 = b"GGUF\x03 modele v1".to_vec();
            let p = inst
                .installer_artefact("m", 1, &artefact("m.gguf", &v1))
                .unwrap();
            let manifeste_v1 = std::fs::read(chemin_manifeste(&p)).unwrap();
            let index_v1 = std::fs::read(d.join(FICHIER_INDEX)).unwrap();

            let mut defaillant = inst.clone();
            defaillant.defaillance = Some(etape);
            let v2 = b"GGUF\x03 modele v2, autre taille".to_vec();
            let e = defaillant
                .installer_artefact("m", 2, &artefact("m.gguf", &v2))
                .unwrap_err();
            assert!(
                e.contains("injectée") && !e.contains("RESTAURATION"),
                "{etape:?} : {e}"
            );

            assert_eq!(
                std::fs::read(&p).unwrap(),
                v1,
                "{etape:?} : ancien modèle intact"
            );
            assert_eq!(
                std::fs::read(chemin_manifeste(&p)).unwrap(),
                manifeste_v1,
                "{etape:?}"
            );
            assert!(
                verifier_modele(&p, &ancres()).is_ok(),
                "{etape:?} : couple cohérent"
            );
            assert_eq!(
                std::fs::read(d.join(FICHIER_INDEX)).unwrap(),
                index_v1,
                "{etape:?}"
            );
            assert_eq!(inst.version_installee(Categorie::ModeleIa, "m"), Some(1));
            assert_eq!(restes(&d), Vec::<String>::new(), "{etape:?}");
            // La même version s'installe ensuite normalement.
            inst.installer_artefact("m", 2, &artefact("m.gguf", &v2))
                .unwrap();
            assert_eq!(std::fs::read(&p).unwrap(), v2);
            let _ = std::fs::remove_dir_all(&d);
        }
    }

    #[test]
    fn echec_d_une_premiere_installation_ne_laisse_rien() {
        for etape in [
            Etape::VerificationTemporaire,
            Etape::EntreRenommages,
            Etape::VerificationFinale,
            Etape::Index,
        ] {
            let d = dossier("canal");
            let mut inst = InstallateurModeles::new(&d, ancres());
            inst.defaillance = Some(etape);
            assert!(inst
                .installer_artefact("m", 1, &artefact("m.gguf", b"GGUF\x03 m"))
                .is_err());
            let fichiers: Vec<_> = std::fs::read_dir(&d)
                .unwrap()
                .filter_map(Result::ok)
                .map(|e| e.file_name())
                .collect();
            assert_eq!(fichiers, Vec::<std::ffi::OsString>::new(), "{etape:?}");
            let _ = std::fs::remove_dir_all(&d);
        }
    }

    #[test]
    fn canal_officiel_de_bout_en_bout() {
        let d = dossier("canal");
        let mut banc = BancCanal::new([0x4d; 32]);
        let mut releveur = Releveur::new(banc.config(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();
        let mut inst = InstallateurModeles::new(&d, ancres());
        let modele = b"GGUF\x03 modele officiel v1".to_vec();
        banc.publier(
            Categorie::ModeleIa,
            "qwen-mini",
            1,
            &artefact("qwen-mini.gguf", &modele),
            T0,
        );
        let mut installees = 0;
        for t in [
            T0,
            T0 + 24 * H,
            T0 + QUARANTAINE_SECS + 1,
            T0 + QUARANTAINE_SECS + 2,
        ] {
            banc.emettre(t);
            let ev = releveur
                .cycle(&mut banc.depot, &mut inst, &mut horloge, t)
                .unwrap();
            installees += ev
                .iter()
                .filter(|e| matches!(e, Evenement::Installee { .. }))
                .count();
        }
        assert_eq!(installees, 1);
        assert_eq!(std::fs::read(d.join("qwen-mini.gguf")).unwrap(), modele);
        assert_eq!(
            inst.version_installee(Categorie::ModeleIa, "qwen-mini"),
            Some(1)
        );
        let _ = std::fs::remove_dir_all(&d);
    }
}
