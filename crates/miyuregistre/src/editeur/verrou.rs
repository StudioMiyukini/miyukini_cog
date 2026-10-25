//! @id     miyuregistre_editeur_verrou
//! @do     verrouiller_flock_avec_delai
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — verrou flock exclusif pris avec un délai borné (essais non bloquants), jamais d'attente infinie.

//! Verrou `flock` exclusif pris **avec délai** : essais non bloquants
//! (`LOCK_EX | LOCK_NB`) répétés toutes les [`PAS_ESSAI`] jusqu'au délai ;
//! au-delà ⇒ [`ErreurVerrou::Delai`]. Un écrivain bloqué (processus figé
//! tenant le verrou) ne fige donc jamais l'éditeur : le cycle échoue
//! explicitement et la péremption des têtes reste visible chez les
//! vérificateurs.

use std::os::fd::AsFd;
use std::time::{Duration, Instant};

use rustix::fs::FlockOperation;
use rustix::io::Errno;

/// Délai par défaut pour prendre le verrou du journal.
pub const DELAI_VERROU: Duration = Duration::from_secs(10);
/// Intervalle entre deux essais.
pub const PAS_ESSAI: Duration = Duration::from_millis(50);

/// Verrou non obtenu.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ErreurVerrou {
    /// Verrou tenu par un autre au-delà du délai.
    #[error("verrou tenu par un autre écrivain au-delà de {0:?}")]
    Delai(Duration),
    /// Erreur du système.
    #[error("verrou : {0}")]
    Systeme(Errno),
}

/// Prend le verrou exclusif de `fd` en au plus `delai` (voir le module).
pub fn verrouiller_exclusif(fd: impl AsFd, delai: Duration) -> Result<(), ErreurVerrou> {
    let limite = Instant::now() + delai;
    loop {
        match rustix::fs::flock(&fd, FlockOperation::NonBlockingLockExclusive) {
            Ok(()) => return Ok(()),
            Err(Errno::INTR) => {}
            Err(Errno::WOULDBLOCK) => {
                let reste = limite.saturating_duration_since(Instant::now());
                if reste.is_zero() {
                    return Err(ErreurVerrou::Delai(delai));
                }
                std::thread::sleep(reste.min(PAS_ESSAI));
            }
            Err(e) => return Err(ErreurVerrou::Systeme(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// B5 : verrou tenu ⇒ erreur au délai, jamais d'attente infinie ; libéré ⇒ pris.
    #[test]
    fn delai_borne() {
        let d = tempfile::tempdir().unwrap();
        let chemin = d.path().join("verrou");
        let a = std::fs::File::create(&chemin).unwrap();
        let b = std::fs::File::open(&chemin).unwrap();
        verrouiller_exclusif(&a, DELAI_VERROU).unwrap();
        let debut = Instant::now();
        assert_eq!(verrouiller_exclusif(&b, Duration::from_millis(200)), Err(ErreurVerrou::Delai(Duration::from_millis(200))));
        let ecoule = debut.elapsed();
        assert!(ecoule >= Duration::from_millis(200) && ecoule < Duration::from_secs(5), "{ecoule:?}");
        drop(a);
        verrouiller_exclusif(&b, Duration::from_millis(200)).unwrap();
    }
}
