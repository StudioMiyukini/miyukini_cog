//! @id     miyuportail_bac_a_sable
//! @do     confiner_sas_seccomp_rlimits
//! @role   securite
//! @layer  kernel
//! @human  miyuportail — confinement du sas : rlimits puis filtre seccomp en liste blanche, action « tuer le processus » (EX-46).

//! Ordre d'installation (avant toute lecture d'entrée non fiable) :
//!
//! 1. **rlimits** : mémoire virtuelle 256 Mio, CPU 5 s, taille de fichier 0,
//!    descripteurs 3 (0, 1, 2 déjà ouverts), processus 0, cœur 0, mémoire
//!    verrouillée 0 ; processus non « dumpable » (aucun vidage mémoire).
//! 2. **`no_new_privs`** puis **filtre seccomp-bpf** (crate `seccompiler`,
//!    API sûre) appliqué à tous les fils (`TSYNC`). Toute instruction hors
//!    liste blanche **tue le processus** (`SECCOMP_RET_KILL_PROCESS`, SIGSYS).
//!
//! Liste blanche (aucun `open`, `socket`, `connect`, `clone`, `execve`…) :
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

/// Installe les rlimits puis le filtre seccomp. Irréversible.
#[cfg(target_os = "linux")]
pub fn installer() -> Result<(), ErreurSas> {
    limiter_ressources()?;
    appliquer_seccomp()
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
