//! @id     miyuportail_bac_a_sable
//! @do     confiner_sas_landlock_seccomp_rlimits
//! @role   securite
//! @layer  kernel
//! @human  miyuportail — confinement du sas : Landlock (aucun accès fichier ni réseau), rlimits, puis filtre seccomp en liste blanche, action « tuer le processus » (EX-46).

//! Ordre d'installation (avant toute lecture d'entrée non fiable) :
//!
//! 1. **Landlock** (crate `landlock`, API sûre) : domaine **sans aucune
//!    règle** qui gère tous les droits du système de fichiers (ABI v1 exigée,
//!    droits des ABI suivantes au mieux), les droits réseau TCP (ABI ≥ 4) et
//!    les portées (sockets abstraites, signaux ; ABI ≥ 6) : **aucun** chemin
//!    n'est accessible, ni en lecture, ni en écriture, ni en exécution. Les
//!    descripteurs 0, 1, 2 déjà ouverts (tubes du parent) ne sont pas touchés.
//!    Installé en premier : la création du jeu de règles consomme un
//!    descripteur, que la limite `RLIMIT_NOFILE = 3` interdirait ensuite.
//!    Pose aussi `no_new_privs`.
//! 2. **rlimits** : mémoire virtuelle 256 Mio, CPU 5 s, taille de fichier 0,
//!    descripteurs 3 (0, 1, 2 déjà ouverts), processus 0, cœur 0, mémoire
//!    verrouillée 0 ; processus non « dumpable » (aucun vidage mémoire).
//! 3. **Filtre seccomp-bpf** (crate `seccompiler`, API sûre) appliqué à tous
//!    les fils (`TSYNC`). Toute instruction hors liste blanche **tue le
//!    processus** (`SECCOMP_RET_KILL_PROCESS`, SIGSYS).
//!
//! Landlock double seccomp (défense en profondeur) : un contournement du
//! filtre (appel oublié, nouvel appel d'ouverture) bute encore sur un domaine
//! où aucun chemin n'existe.
//!
//! **Noyau sans Landlock** (absent, désactivé au démarrage, ABI < 1) : le sas
//! **refuse de démarrer** ([`ErreurSas::LandlockIndisponible`], code de
//! sortie [`crate::sas::SORTIE_LANDLOCK`]) et tout lot est refusé. Aucun
//! réglage ne lève ce refus (fail-closed, C-12) : il faut un noyau ≥ 5.13
//! avec `landlock` dans `lsm=`.
//!
//! Liste blanche seccomp (aucun `open`, `socket`, `connect`, `clone`, `execve`…) :
//!
//! | appel | condition |
//! |---|---|
//! | `read` | descripteur 0 uniquement |
//! | `write` | descripteurs 1 et 2 uniquement |
//! | `mmap`, `mprotect` | sans `PROT_EXEC` |
//! | `brk`, `munmap`, `mremap`, `madvise` | allocateur |
//! | `futex`, `sched_yield` | verrous de la bibliothèque standard |
//! | `rt_sigreturn`, `rt_sigprocmask`, `sigaltstack` | signaux du runtime |
//! | `clock_gettime`, `getrandom` | horloge, aléa |
//! | `exit`, `exit_group` | fin |
//!
//! Hors Linux : refus (fail-closed).

use crate::erreurs::ErreurSas;

/// Mémoire virtuelle maximale du sas.
pub const LIMITE_MEMOIRE: u64 = 256 * 1024 * 1024;
/// Temps CPU maximal du sas (secondes).
pub const LIMITE_CPU_SECS: u64 = 5;

/// Installe Landlock, les rlimits puis le filtre seccomp. Irréversible.
#[cfg(target_os = "linux")]
pub fn installer() -> Result<(), ErreurSas> {
    interdire_systeme_de_fichiers()?;
    limiter_ressources()?;
    appliquer_seccomp()
}

/// Domaine Landlock sans aucune règle : tout accès au système de fichiers
/// (et, selon le noyau, au réseau TCP, aux sockets abstraites et aux
/// signaux hors du domaine) est refusé. Irréversible.
///
/// Les droits de l'ABI v1 sont **exigés** (`HardRequirement`) : noyau sans
/// Landlock ⇒ [`ErreurSas::LandlockIndisponible`]. Les droits des ABI
/// suivantes sont ajoutés au mieux. Le statut renvoyé par le noyau est
/// contrôlé : domaine non appliqué ou `no_new_privs` absent ⇒ refus.
#[cfg(target_os = "linux")]
pub fn interdire_systeme_de_fichiers() -> Result<(), ErreurSas> {
    use landlock::{
        Access, AccessFs, AccessNet, CompatLevel, Compatible, Ruleset, RulesetAttr, RulesetStatus, Scope, ABI,
    };

    // ABI la plus récente connue de la crate : droits ajoutés au mieux.
    const ABI_RECENTE: ABI = ABI::V9;
    let indisponible = |_| ErreurSas::LandlockIndisponible;
    let statut = Ruleset::default()
        .set_compatibility(CompatLevel::HardRequirement)
        .handle_access(AccessFs::from_all(ABI::V1))
        .map_err(indisponible)?
        .set_compatibility(CompatLevel::BestEffort)
        .handle_access(AccessFs::from_all(ABI_RECENTE))
        .map_err(indisponible)?
        .handle_access(AccessNet::from_all(ABI_RECENTE))
        .map_err(indisponible)?
        .scope(Scope::from_all(ABI_RECENTE))
        .map_err(indisponible)?
        .create()
        .map_err(indisponible)?
        .restrict_self()
        .map_err(indisponible)?;
    if statut.ruleset == RulesetStatus::NotEnforced || !statut.no_new_privs {
        return Err(ErreurSas::LandlockIndisponible);
    }
    Ok(())
}

/// Hors Linux : aucun confinement possible, refus.
#[cfg(not(target_os = "linux"))]
pub fn installer() -> Result<(), ErreurSas> {
    Err(ErreurSas::PlateformeNonSupportee)
}

#[cfg(target_os = "linux")]
fn limiter_ressources() -> Result<(), ErreurSas> {
    use rustix::process::{setrlimit, Resource, Rlimit};
    let fixe = |v: u64| Rlimit { current: Some(v), maximum: Some(v) };
    let limites = [
        (Resource::As, LIMITE_MEMOIRE),
        (Resource::Cpu, LIMITE_CPU_SECS),
        (Resource::Fsize, 0),
        (Resource::Nofile, 3),
        (Resource::Nproc, 0),
        (Resource::Core, 0),
        (Resource::Memlock, 0),
    ];
    for (ressource, valeur) in limites {
        setrlimit(ressource, fixe(valeur)).map_err(|_| ErreurSas::BacASable)?;
    }
    // Processus non « dumpable » : aucun vidage mémoire (même via un
    // `core_pattern` en tube), pas de ptrace par un processus de même uid.
    rustix::process::set_dumpable_behavior(rustix::process::DumpableBehavior::NotDumpable)
        .map_err(|_| ErreurSas::BacASable)
}

#[cfg(target_os = "linux")]
fn appliquer_seccomp() -> Result<(), ErreurSas> {
    use std::collections::BTreeMap;

    use seccompiler::{
        apply_filter_all_threads, BpfProgram, SeccompAction, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition,
        SeccompFilter, SeccompRule, TargetArch,
    };

    fn err<E>(_: E) -> ErreurSas {
        ErreurSas::BacASable
    }
    let si_arg = |index: u8, op: SeccompCmpOp, valeur: u64| -> Result<SeccompRule, ErreurSas> {
        let c = SeccompCondition::new(index, SeccompCmpArgLen::Qword, op, valeur).map_err(err)?;
        SeccompRule::new(vec![c]).map_err(err)
    };
    let sans_exec = || si_arg(2, SeccompCmpOp::MaskedEq(libc::PROT_EXEC as u64), 0);

    let mut regles: BTreeMap<i64, Vec<SeccompRule>> = BTreeMap::new();
    regles.insert(libc::SYS_read, vec![si_arg(0, SeccompCmpOp::Eq, 0)?]);
    regles.insert(libc::SYS_write, vec![si_arg(0, SeccompCmpOp::Eq, 1)?, si_arg(0, SeccompCmpOp::Eq, 2)?]);
    regles.insert(libc::SYS_mmap, vec![sans_exec()?]);
    regles.insert(libc::SYS_mprotect, vec![sans_exec()?]);
    for libre in [
        libc::SYS_brk,
        libc::SYS_munmap,
        libc::SYS_mremap,
        libc::SYS_madvise,
        libc::SYS_futex,
        libc::SYS_sched_yield,
        libc::SYS_rt_sigreturn,
        libc::SYS_rt_sigprocmask,
        libc::SYS_sigaltstack,
        libc::SYS_clock_gettime,
        libc::SYS_getrandom,
        libc::SYS_exit,
        libc::SYS_exit_group,
    ] {
        regles.insert(libre, Vec::new());
    }

    let arch = TargetArch::try_from(std::env::consts::ARCH).map_err(err)?;
    let filtre = SeccompFilter::new(regles, SeccompAction::KillProcess, SeccompAction::Allow, arch).map_err(err)?;
    let programme: BpfProgram = filtre.try_into().map_err(err)?;
    apply_filter_all_threads(&programme).map_err(err)
}
