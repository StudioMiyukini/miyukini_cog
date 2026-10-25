//! @id     miyufederation_fraicheur
//! @do     garantir_fraicheur_etat_federal
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — compteur de fraîcheur local (génération et empreinte du dernier état écrit, tenus hors du support) : retour arrière d'un état authentique refusé au chargement.

//! Compteur de **fraîcheur** de l'état fédéral (A-019, constat de revue,
//! **à consigner par amendement**).
//!
//! Le HMAC `MFED` prouve qu'un état a été scellé par ce COG, pas qu'il est le
//! **dernier** : un acteur capable d'écrire dans le support (base KindMother
//! `miyufederation`, fichier) pourrait y restaurer un état authentique plus
//! ancien (pair revenu à un statut antérieur, note annulée). La vue
//! persistante ([`crate::FederationPersistante`]) lie donc chaque état à un
//! compteur monotone tenu **hors du support**, sous le contrôle du COG :
//!
//! - chaque écriture porte une **génération** `g` (numéro d'écriture, sous le
//!   HMAC `MFED` v4) ; après l'écriture de l'état, le compteur reçoit
//!   `(g, SHA-256 de l'état scellé)` ;
//! - au chargement, l'état lu (revérifié) doit avoir la génération du
//!   compteur **et** son empreinte ; génération inférieure ⇒
//!   [`ErreurEtat::EtatPerime`] ; même génération, empreinte différente ⇒
//!   [`ErreurEtat::EtatDivergent`] ; génération `compteur + 1` ⇒ acceptée
//!   (écriture de l'état faite, mise à jour du compteur interrompue) et le
//!   compteur est aussitôt avancé ; au-delà ⇒
//!   [`ErreurEtat::CompteurIncoherent`] ;
//! - **compteur perdu** (absent alors qu'un état existe) ⇒
//!   [`ErreurEtat::CompteurAbsent`] : **aucune reprise silencieuse**, le
//!   rechargement reste refusé jusqu'à l'intervention explicite de
//!   l'administrateur ([`crate::FederationPersistante::reamorcer_compteur`],
//!   qui accepte l'état présent après revérification complète — à ne faire
//!   qu'après avoir établi qu'il n'a pas été restauré) ;
//! - compteur présent mais état absent ⇒ [`ErreurEtat::EtatAbsent`], même si
//!   l'appelant ne l'attendait pas (suppression détectée).
//!
//! # Fichier `fraicheur.mffr` ([`CompteurFichier`])
//!
//! Dossier privé `0700` (mêmes contrôles que [`crate::StockageFederal`] :
//! lien symbolique refusé, propriétaire = uid effectif), verrou `flock`
//! exclusif `verrou-fraicheur`, fichier `0600` écrit atomiquement
//! (temporaire, `fsync`, renommage, `fsync` du dossier) :
//!
//! ```text
//! "MFFR" | version 0x01 | génération (8, big-endian) | empreinte (32) | mac (32)   = 77 octets
//! mac = HMAC-SHA256(K', "MFFR" | 0x01 | génération | empreinte)
//! K'  = HMAC-SHA256(secret local du COG, "miyukini/miyufederation/fraicheur/v1")
//! ```
//!
//! Le compteur refuse de reculer ([`ErreurEtat::CompteurIncoherent`]).
//! Avec le support KindMother, le dossier du compteur est local au COG (ex.
//! `<données>/federation/`), jamais dans KindMother ; avec le support
//! fichier, il peut partager le dossier de l'état (protection moindre :
//! un accès disque restaurant les deux ensemble reste hors de portée, comme
//! la paire état + registre local, voir [`crate::persistance`]).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use hmac::Mac;
use sha2::{Digest, Sha256};
use zeroize::Zeroize;

use crate::persistance::{ecrire_prive, hmac_cle32, lire_prive, preparer_dossier, ErreurEtat};

/// Nom du fichier du compteur de fraîcheur.
pub const FICHIER_COMPTEUR: &str = "fraicheur.mffr";
/// Nom du fichier de verrou du compteur.
pub const FICHIER_VERROU_COMPTEUR: &str = "verrou-fraicheur";
const MAGIC: &[u8; 4] = b"MFFR";
const VERSION: u8 = 0x01;
const ETIQUETTE: &[u8] = b"miyukini/miyufederation/fraicheur/v1";
const TAILLE_CORPS: usize = 4 + 1 + 8 + 32;
/// Taille exacte du fichier du compteur.
pub const TAILLE_COMPTEUR: usize = TAILLE_CORPS + 32;

/// Génération et empreinte du dernier état écrit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraicheur {
    /// Génération scellée dans l'état (`MFED` v4).
    pub generation: u64,
    /// SHA-256 de l'état scellé complet.
    pub empreinte: [u8; 32],
}

impl Fraicheur {
    /// Fraîcheur de l'état scellé `octets` de génération `generation`.
    pub fn de_etat(generation: u64, octets: &[u8]) -> Self {
        Self { generation, empreinte: Sha256::digest(octets).into() }
    }
}

/// Compteur monotone de fraîcheur, tenu hors du support de l'état.
pub trait CompteurFraicheur: Send + Sync + std::fmt::Debug {
    /// Dernière fraîcheur enregistrée ; `None` : aucun compteur.
    fn lire(&self) -> Result<Option<Fraicheur>, ErreurEtat>;
    /// Enregistre atomiquement `fraicheur` ; refuse un recul (génération
    /// inférieure, ou égale d'empreinte différente).
    fn ecrire(&self, fraicheur: &Fraicheur) -> Result<(), ErreurEtat>;
}

/// Refus d'un recul du compteur.
fn controler_avance(actuel: Option<Fraicheur>, suivant: &Fraicheur) -> Result<(), ErreurEtat> {
    match actuel {
        Some(a) if suivant.generation < a.generation => Err(ErreurEtat::CompteurIncoherent("recul du compteur")),
        Some(a) if suivant.generation == a.generation && suivant.empreinte != a.empreinte => {
            Err(ErreurEtat::CompteurIncoherent("même génération, autre empreinte"))
        }
        _ => Ok(()),
    }
}

/// Clé de scellement du compteur, effacée à la libération.
struct CleCompteur([u8; 32]);

impl Drop for CleCompteur {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl CleCompteur {
    fn deriver(secret_local: &[u8; 32]) -> Self {
        let mut m = hmac_cle32(secret_local);
        m.update(ETIQUETTE);
        Self(m.finalize().into_bytes().into())
    }

    fn mac(&self, corps: &[u8]) -> [u8; 32] {
        let mut m = hmac_cle32(&self.0);
        m.update(corps);
        m.finalize().into_bytes().into()
    }
}

/// Compteur de fraîcheur dans un fichier local privé (voir module).
pub struct CompteurFichier {
    dossier: PathBuf,
    cle: CleCompteur,
    /// Verrou exclusif tenu jusqu'à la libération.
    #[cfg(unix)]
    _verrou: std::fs::File,
}

impl std::fmt::Debug for CompteurFichier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompteurFichier").field("dossier", &self.dossier).finish_non_exhaustive()
    }
}

impl CompteurFichier {
    /// Ouvre (crée en `0700` au besoin) le dossier du compteur et prend son
    /// verrou exclusif ; mêmes refus que [`crate::StockageFederal::ouvrir`].
    pub fn ouvrir(dossier: &Path, secret_local: &[u8; 32]) -> Result<Self, ErreurEtat> {
        preparer_dossier(dossier)?;
        Ok(Self {
            dossier: dossier.to_path_buf(),
            cle: CleCompteur::deriver(secret_local),
            #[cfg(unix)]
            _verrou: crate::persistance::verrouiller(dossier, FICHIER_VERROU_COMPTEUR)?,
        })
    }

    /// Chemin du fichier du compteur.
    pub fn chemin(&self) -> PathBuf {
        self.dossier.join(FICHIER_COMPTEUR)
    }

    fn sceller(&self, f: &Fraicheur) -> Vec<u8> {
        let mut o = Vec::with_capacity(TAILLE_COMPTEUR);
        o.extend_from_slice(MAGIC);
        o.push(VERSION);
        o.extend_from_slice(&f.generation.to_be_bytes());
        o.extend_from_slice(&f.empreinte);
        let m = self.cle.mac(&o);
        o.extend_from_slice(&m);
        o
    }

    fn ouvrir_sceau(&self, o: &[u8]) -> Result<Fraicheur, ErreurEtat> {
        if o.len() != TAILLE_COMPTEUR || &o[..4] != MAGIC {
            return Err(ErreurEtat::CompteurIncoherent("illisible"));
        }
        if o[4] != VERSION {
            return Err(ErreurEtat::CompteurIncoherent("version inconnue"));
        }
        let (corps, m) = o.split_at(TAILLE_CORPS);
        let mut verif = hmac_cle32(&self.cle.0);
        verif.update(corps);
        verif.verify_slice(m).map_err(|_| ErreurEtat::CompteurIncoherent("altéré (HMAC)"))?;
        let mut g = [0u8; 8];
        g.copy_from_slice(&corps[5..13]);
        let mut empreinte = [0u8; 32];
        empreinte.copy_from_slice(&corps[13..45]);
        Ok(Fraicheur { generation: u64::from_be_bytes(g), empreinte })
    }
}

impl CompteurFraicheur for CompteurFichier {
    fn lire(&self) -> Result<Option<Fraicheur>, ErreurEtat> {
        lire_prive(&self.chemin(), TAILLE_COMPTEUR as u64)?.map(|o| self.ouvrir_sceau(&o)).transpose()
    }

    fn ecrire(&self, fraicheur: &Fraicheur) -> Result<(), ErreurEtat> {
        controler_avance(self.lire()?, fraicheur)?;
        ecrire_prive(&self.dossier, FICHIER_COMPTEUR, &self.sceller(fraicheur))
    }
}

/// Compteur en mémoire **pour les tests** : les clones partagent la valeur
/// (un clone survit au « redémarrage ») ; perte et rejeu injectables.
#[derive(Debug, Clone, Default)]
pub struct CompteurMemoire {
    interne: Arc<Mutex<Option<Fraicheur>>>,
}

impl CompteurMemoire {
    /// Compteur vide.
    pub fn new() -> Self {
        Self::default()
    }

    fn avec<T>(&self, f: impl FnOnce(&mut Option<Fraicheur>) -> T) -> T {
        let mut g = self.interne.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        f(&mut g)
    }

    /// Valeur courante.
    pub fn valeur(&self) -> Option<Fraicheur> {
        self.avec(|v| *v)
    }

    /// Remplace la valeur (perte du compteur, rejeu).
    pub fn remplacer(&self, valeur: Option<Fraicheur>) {
        self.avec(|v| *v = valeur);
    }
}

impl CompteurFraicheur for CompteurMemoire {
    fn lire(&self) -> Result<Option<Fraicheur>, ErreurEtat> {
        Ok(self.valeur())
    }

    fn ecrire(&self, fraicheur: &Fraicheur) -> Result<(), ErreurEtat> {
        self.avec(|v| {
            controler_avance(*v, fraicheur)?;
            *v = Some(*fraicheur);
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compteur_fichier_aller_retour_altere_et_recul() {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let d = std::env::temp_dir().join(format!("miyufederation-fraicheur-{}-{n}", std::process::id()));
        let c = CompteurFichier::ouvrir(&d, &[3; 32]).unwrap();
        assert_eq!(c.lire().unwrap(), None);
        let f1 = Fraicheur::de_etat(1, b"un");
        c.ecrire(&f1).unwrap();
        assert_eq!(c.lire().unwrap(), Some(f1));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(std::fs::metadata(c.chemin()).unwrap().permissions().mode() & 0o777, 0o600);
        }
        // Deuxième ouverture : verrouillé.
        assert!(matches!(CompteurFichier::ouvrir(&d, &[3; 32]), Err(ErreurEtat::Verrouille)));
        // Recul refusé.
        let f2 = Fraicheur::de_etat(2, b"deux");
        c.ecrire(&f2).unwrap();
        assert!(matches!(c.ecrire(&f1), Err(ErreurEtat::CompteurIncoherent(_))));
        assert!(matches!(c.ecrire(&Fraicheur::de_etat(2, b"autre")), Err(ErreurEtat::CompteurIncoherent(_))));
        c.ecrire(&f2).unwrap();
        // Un octet altéré : refus.
        let mut o = std::fs::read(c.chemin()).unwrap();
        o[7] ^= 1;
        std::fs::write(c.chemin(), &o).unwrap();
        assert!(matches!(c.lire(), Err(ErreurEtat::CompteurIncoherent(_))));
        // Autre secret local : refus.
        drop(c);
        let c = CompteurFichier::ouvrir(&d, &[4; 32]).unwrap();
        c.ecrire(&f2).unwrap_err();
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn compteur_memoire_partage_et_recul() {
        let c = CompteurMemoire::new();
        let d = c.clone();
        c.ecrire(&Fraicheur::de_etat(3, b"x")).unwrap();
        assert_eq!(d.lire().unwrap().map(|f| f.generation), Some(3));
        assert!(d.ecrire(&Fraicheur::de_etat(2, b"x")).is_err());
    }
}
