//! @id     miyuracine_durcissement
//! @do     durcir_processus_racine
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « durcissement » (core dumps, mlock ; NC-ANSSI-25).

//! Durcissement du processus qui manipule la racine (NC-ANSSI-25).
//!
//! [`durcir_processus`] doit être appelée **avant** toute lecture de part, de
//! phrase, de mot de passe ou de fichier de clé (le binaire `miyuracine` le
//! fait en tête de `main`). Elle applique, dans cet ordre :
//!
//! 1. **Core dumps désactivés** — toujours fail-closed, non configurable :
//!    `setrlimit(RLIMIT_CORE, 0)` (limites courante et maximale) puis
//!    `prctl(PR_SET_DUMPABLE, 0)` (aucun vidage, même via un `core_pattern` en
//!    tube ; pas de `ptrace` par un processus de même uid). Échec ⇒ refus.
//! 2. **Mémoire verrouillée** — `mlockall(MCL_CURRENT | MCL_FUTURE)` : toutes
//!    les pages, présentes et futures (tampons de secrets, état Argon2id),
//!    sont exclues du swap. Le verrouillage global est préféré au `mlock` par
//!    tampon : il couvre aussi la mémoire de travail d'Argon2id et des copies
//!    intermédiaires, et ne demande aucun code `unsafe` (`rustix::mm::mlock`
//!    est `unsafe`, le crate est `forbid(unsafe_code)`).
//!
//!    Avant `mlockall`, si la limite `RLIMIT_MEMLOCK` courante est inférieure
//!    au **besoin** (budget demandé par l'appelant — mémoire Argon2id — plus
//!    [`MARGE_VERROUILLAGE`]), elle est **relevée** : d'abord au besoin
//!    (limite maximale relevée elle aussi si nécessaire, ce que root ou
//!    `CAP_SYS_RESOURCE` obtiennent), à défaut jusqu'à la limite maximale.
//!    Puis :
//!    - limite ≥ besoin : `mlockall` est tenté ; refus **seulement** s'il
//!      échoue réellement ;
//!    - limite < besoin mais `CAP_IPC_LOCK` effective (root, ou capacité
//!      accordée : le noyau ignore alors `RLIMIT_MEMLOCK`) : `mlockall` est
//!      tenté de même, budget illimité ;
//!    - limite < besoin sans `CAP_IPC_LOCK` : refus anticipé — `mlockall`
//!      réussirait sur les pages présentes mais, sous `MCL_FUTURE`, la
//!      première allocation au-delà de la limite échouerait de fait et
//!      interromprait brutalement le processus.
//!
//!    Le budget retenu est mémorisé ; [`verifier_budget`] refuse ensuite
//!    proprement (sans allouer) tout calcul Argon2id qui le dépasserait.
//!
//! **Politique de verrouillage** ([`PolitiqueVerrouillage`]) :
//! - [`PolitiqueVerrouillage::Exiger`] (**défaut**, fail-closed) : limite
//!   insuffisante ou `mlockall` refusé ⇒ [`ErreurRacine::VerrouillageMemoire`].
//!   Sur le poste de cérémonie : exécuter en root, ou accorder `CAP_IPC_LOCK`,
//!   ou relever la limite (`ulimit -l unlimited`, `LimitMEMLOCK=` systemd,
//!   `/etc/security/limits.conf`) ; root et `CAP_IPC_LOCK` suffisent même
//!   avec une limite basse (relèvement ou dérogation du noyau).
//! - [`PolitiqueVerrouillage::Avertir`] (dérogation explicite, option
//!   `--sans-mlock` du binaire) : aucun verrouillage n'est tenté et
//!   [`EtatDurcissement::memoire_verrouillee`] vaut `false` ; l'appelant
//!   **doit** afficher un avertissement (le binaire le fait sur la sortie
//!   d'erreur). Les core dumps restent désactivés dans tous les cas.
//!
//! Hors Linux : refus ([`ErreurRacine::Durcissement`]) — les garanties
//! `PR_SET_DUMPABLE` / `mlockall` ne sont pas établies ailleurs.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::erreurs::ErreurRacine;

/// Marge ajoutée au budget Argon2id pour le reste du processus (pile, tas,
/// binaire, bibliothèques) : 64 Mio.
pub const MARGE_VERROUILLAGE: u64 = 64 * 1024 * 1024;

/// Budget de mémoire verrouillée retenu par [`durcir_processus`] :
/// 0 = pas de verrouillage actif, `u64::MAX` = limite infinie.
static BUDGET_VERROUILLE: AtomicU64 = AtomicU64::new(0);

/// Conduite à tenir si la mémoire ne peut être verrouillée.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PolitiqueVerrouillage {
    /// Refus (fail-closed) : défaut.
    #[default]
    Exiger,
    /// Dérogation explicite : pas de verrouillage, l'appelant avertit.
    Avertir,
}

/// Résultat du durcissement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct EtatDurcissement {
    /// Core dumps désactivés (toujours `true` si la fonction réussit).
    pub core_dumps_desactives: bool,
    /// Mémoire verrouillée par `mlockall(MCL_CURRENT | MCL_FUTURE)`.
    pub memoire_verrouillee: bool,
}

/// Durcit le processus courant (voir la documentation du module).
///
/// `budget_octets` : mémoire verrouillée nécessaire, au moins la mémoire
/// Argon2id maximale qui sera demandée (la [`MARGE_VERROUILLAGE`] est ajoutée
/// ici). Ignoré avec [`PolitiqueVerrouillage::Avertir`].
#[cfg(target_os = "linux")]
pub fn durcir_processus(
    politique: PolitiqueVerrouillage,
    budget_octets: u64,
) -> Result<EtatDurcissement, ErreurRacine> {
    use rustix::process::{
        getrlimit, set_dumpable_behavior, setrlimit, DumpableBehavior, Resource, Rlimit,
    };

    // 1. Core dumps : toujours, fail-closed.
    setrlimit(
        Resource::Core,
        Rlimit {
            current: Some(0),
            maximum: Some(0),
        },
    )
    .map_err(|_| ErreurRacine::Durcissement("setrlimit(RLIMIT_CORE, 0) refusé"))?;
    set_dumpable_behavior(DumpableBehavior::NotDumpable)
        .map_err(|_| ErreurRacine::Durcissement("prctl(PR_SET_DUMPABLE, 0) refusé"))?;

    // 2. Verrouillage mémoire.
    if politique == PolitiqueVerrouillage::Avertir {
        BUDGET_VERROUILLE.store(0, Ordering::SeqCst);
        return Ok(EtatDurcissement {
            core_dumps_desactives: true,
            memoire_verrouillee: false,
        });
    }
    let besoin = budget_octets.saturating_add(MARGE_VERROUILLAGE);
    let mut limite = getrlimit(Resource::Memlock);
    if limite.current.is_some_and(|c| c < besoin) {
        // Relèvement au besoin (maximale comprise : root / CAP_SYS_RESOURCE),
        // sinon jusqu'à la limite maximale (aucun privilège requis).
        let au_besoin = Rlimit {
            current: Some(besoin),
            maximum: limite.maximum.map(|m| m.max(besoin)),
        };
        if setrlimit(Resource::Memlock, au_besoin).is_err() && limite.current != limite.maximum {
            let au_maximum = Rlimit {
                current: limite.maximum,
                maximum: limite.maximum,
            };
            let _ = setrlimit(Resource::Memlock, au_maximum);
        }
        limite = getrlimit(Resource::Memlock);
    }
    let disponible = limite.current.unwrap_or(u64::MAX);
    let Some(budget) = decider_verrouillage(disponible, besoin, capacite_ipc_lock()) else {
        return Err(ErreurRacine::VerrouillageMemoire);
    };
    rustix::mm::mlockall(rustix::mm::MlockAllFlags::CURRENT | rustix::mm::MlockAllFlags::FUTURE)
        .map_err(|_| ErreurRacine::VerrouillageMemoire)?;
    BUDGET_VERROUILLE.store(budget, Ordering::SeqCst);
    Ok(EtatDurcissement {
        core_dumps_desactives: true,
        memoire_verrouillee: true,
    })
}

/// Décision avant `mlockall` : budget Argon2id à retenir (`Some`, jamais 0)
/// ou refus anticipé (`None`). `disponible` : limite `RLIMIT_MEMLOCK`
/// courante après relèvement ; `ipc_lock` : `CAP_IPC_LOCK` effective (le
/// noyau ignore alors la limite).
fn decider_verrouillage(disponible: u64, besoin: u64, ipc_lock: bool) -> Option<u64> {
    if disponible >= besoin {
        Some(disponible.saturating_sub(MARGE_VERROUILLAGE).max(1))
    } else if ipc_lock {
        Some(u64::MAX)
    } else {
        None
    }
}

/// `CAP_IPC_LOCK` (bit 14) dans l'ensemble effectif, lu sur
/// `/proc/self/status` (ligne `CapEff:`). Illisible ⇒ `false` (fail-closed :
/// pas de dérogation supposée).
#[cfg(target_os = "linux")]
fn capacite_ipc_lock() -> bool {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|t| capacites_effectives(&t))
        .is_some_and(|c| c & (1 << CAP_IPC_LOCK) != 0)
}

/// Numéro de la capacité `CAP_IPC_LOCK`.
const CAP_IPC_LOCK: u32 = 14;

/// Masque `CapEff:` (hexadécimal) d'un texte `/proc/<pid>/status`.
fn capacites_effectives(status: &str) -> Option<u64> {
    status
        .lines()
        .find_map(|l| l.strip_prefix("CapEff:"))
        .and_then(|v| u64::from_str_radix(v.trim(), 16).ok())
}

/// Hors Linux : refus (garanties non établies).
#[cfg(not(target_os = "linux"))]
pub fn durcir_processus(
    _politique: PolitiqueVerrouillage,
    _budget_octets: u64,
) -> Result<EtatDurcissement, ErreurRacine> {
    Err(ErreurRacine::Durcissement(
        "plateforme non prise en charge (Linux requis)",
    ))
}

/// Refuse un calcul demandant `octets` de mémoire si la mémoire est
/// verrouillée et que ce besoin dépasse le budget retenu (évite l'échec
/// d'allocation, fatal sous `MCL_FUTURE`). Sans verrouillage actif : accepté.
pub(crate) fn verifier_budget(octets: u64) -> Result<(), ErreurRacine> {
    let budget = BUDGET_VERROUILLE.load(Ordering::SeqCst);
    if budget == 0 || octets <= budget {
        Ok(())
    } else {
        Err(ErreurRacine::VerrouillageMemoire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIO: u64 = 1024 * 1024;

    #[test]
    fn limite_suffisante_budget_borne() {
        let besoin = 256 * MIO + MARGE_VERROUILLAGE;
        assert_eq!(decider_verrouillage(besoin, besoin, false), Some(256 * MIO));
        assert_eq!(decider_verrouillage(u64::MAX, besoin, false), Some(u64::MAX - MARGE_VERROUILLAGE));
        // Jamais 0 (0 = « pas de verrouillage actif »).
        assert_eq!(decider_verrouillage(MARGE_VERROUILLAGE, MARGE_VERROUILLAGE, false), Some(1));
    }

    /// B1 : limite basse mais `CAP_IPC_LOCK` (root, capacité) ⇒ pas de refus
    /// anticipé, `mlockall` est tenté ; sans la capacité ⇒ refus.
    #[test]
    fn limite_basse_ipc_lock_tente_mlockall() {
        let besoin = 1024 * MIO + MARGE_VERROUILLAGE;
        assert_eq!(decider_verrouillage(8 * MIO, besoin, true), Some(u64::MAX));
        assert_eq!(decider_verrouillage(8 * MIO, besoin, false), None);
    }

    #[test]
    fn lecture_de_cap_eff() {
        let racine = "Name:\tx\nCapInh:\t0000000000000000\nCapEff:\t000001ffffffffff\n";
        assert_eq!(capacites_effectives(racine), Some(0x01ff_ffff_ffff));
        assert!(capacites_effectives(racine).is_some_and(|c| c & (1 << CAP_IPC_LOCK) != 0));
        let ipc = format!("CapEff:\t{:016x}\n", 1u64 << CAP_IPC_LOCK);
        assert_eq!(capacites_effectives(&ipc), Some(1 << CAP_IPC_LOCK));
        let rien = "CapEff:\t0000000000000000\n";
        assert!(capacites_effectives(rien).is_some_and(|c| c & (1 << CAP_IPC_LOCK) == 0));
        assert_eq!(capacites_effectives("Name:\tx\n"), None);
    }
}
