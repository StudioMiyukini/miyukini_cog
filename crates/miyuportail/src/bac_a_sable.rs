//! @id     miyuportail_bac_a_sable
//! @do     confiner_sas_landlock_seccomp_rlimits
//! @role   securite
//! @layer  kernel
//! @human  miyuportail — confinement du sas : Landlock (aucun accès fichier ni réseau), rlimits, puis filtre seccomp en liste blanche, action « tuer le processus » (EX-46).

//! Ordre d'installation (avant toute lecture d'entrée non fiable) :
//!
//! 0. **Descripteurs** ([`refuser_descripteurs_herites`]) : `/proc/self/fd`
//!    est listé ; tout descripteur au-delà de 0, 1, 2 (hérité du parent ou
//!    d'un intermédiaire) ⇒ refus de démarrer, code de sortie
//!    [`crate::sas::SORTIE_DESCRIPTEURS`] ([`ErreurSas::DescripteurHerite`]).
//!    Un décodeur compromis ne peut donc lire, écrire ni projeter
//!    (`mmap`) aucun fichier hérité. Côté parent, le lancement est refusé
//!    de même si un descripteur > 2 du processus n'est pas `FD_CLOEXEC`
//!    ([`descripteurs_heritables`]) : la bibliothèque standard ouvre tout en
//!    `O_CLOEXEC`, un descripteur sans ce drapeau est une anomalie.
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
//! | `mmap` | anonyme uniquement : `MAP_ANONYMOUS` et descripteur `-1`, sans `PROT_EXEC` |
//! | `mprotect` | sans `PROT_EXEC` |
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
    refuser_descripteurs_herites()?;
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

/// Descripteurs ouverts au-delà de 0, 1, 2 (celui qui sert à lister
/// `/proc/self/fd` est exclu). Liste illisible ⇒ erreur (fail-closed).
#[cfg(target_os = "linux")]
fn descripteurs_au_dela_de_2() -> Result<Vec<i32>, ErreurSas> {
    use rustix::fd::AsRawFd;
    use rustix::fs::{Dir, Mode, OFlags};

    let erreur = |_| ErreurSas::DescripteurHerite;
    let fd = rustix::fs::open("/proc/self/fd", OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC, Mode::empty())
        .map_err(erreur)?;
    let liste = fd.as_raw_fd();
    let mut dir = Dir::new(fd).map_err(erreur)?;
    let mut au_dela = Vec::new();
    while let Some(entree) = dir.read() {
        let entree = entree.map_err(erreur)?;
        let nom = entree.file_name().to_bytes();
        if nom == b"." || nom == b".." {
            continue;
        }
        let n: i32 = std::str::from_utf8(nom).ok().and_then(|t| t.parse().ok()).ok_or(ErreurSas::DescripteurHerite)?;
        if n > 2 && n != liste {
            au_dela.push(n);
        }
    }
    Ok(au_dela)
}

/// **Dans le sas, avant Landlock** : refuse de démarrer si un descripteur
/// au-delà de 0, 1, 2 est ouvert (hérité). Irréversible côté appelant :
/// le sas sort avec [`crate::sas::SORTIE_DESCRIPTEURS`].
#[cfg(target_os = "linux")]
pub fn refuser_descripteurs_herites() -> Result<(), ErreurSas> {
    if descripteurs_au_dela_de_2()?.is_empty() {
        Ok(())
    } else {
        Err(ErreurSas::DescripteurHerite)
    }
}

/// `O_CLOEXEC` tel que l'affiche `/proc/<pid>/fdinfo/<fd>` (champ `flags`, octal).
#[cfg(target_os = "linux")]
const O_CLOEXEC_FDINFO: u32 = 0o2_000_000;

/// Drapeaux du champ `flags:` (octal) d'un `fdinfo`.
#[cfg(target_os = "linux")]
fn drapeaux_fdinfo(texte: &str) -> Option<u32> {
    texte
        .lines()
        .find_map(|l| l.strip_prefix("flags:"))
        .and_then(|v| u32::from_str_radix(v.trim(), 8).ok())
}

/// **Côté parent, avant le lancement** : descripteurs > 2 du processus
/// courant qui seraient transmis au sas (sans `FD_CLOEXEC`). Lecture de
/// `/proc/self/fdinfo/<n>` (aucun `unsafe`) ; un descripteur fermé entre la
/// liste et la lecture est ignoré ; tout autre échec ⇒ erreur.
///
/// Fenêtre résiduelle : un descripteur créé sans `O_CLOEXEC` par un autre
/// fil entre ce contrôle et l'`exec` serait transmis ; le sas le refuse
/// alors lui-même ([`refuser_descripteurs_herites`]).
#[cfg(target_os = "linux")]
pub fn descripteurs_heritables() -> Result<Vec<i32>, ErreurSas> {
    let mut heritables = Vec::new();
    for n in descripteurs_au_dela_de_2()? {
        let texte = match std::fs::read_to_string(format!("/proc/self/fdinfo/{n}")) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => return Err(ErreurSas::DescripteurHerite),
        };
        let drapeaux = drapeaux_fdinfo(&texte).ok_or(ErreurSas::DescripteurHerite)?;
        if drapeaux & O_CLOEXEC_FDINFO == 0 {
            heritables.push(n);
        }
    }
    Ok(heritables)
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

/// Programme seccomp-bpf de la liste blanche (voir le tableau du module).
#[cfg(target_os = "linux")]
pub(crate) fn construire_filtre() -> Result<seccompiler::BpfProgram, ErreurSas> {
    use std::collections::BTreeMap;

    use seccompiler::{
        SeccompAction, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition, SeccompFilter, SeccompRule, TargetArch,
    };

    fn err<E>(_: E) -> ErreurSas {
        ErreurSas::BacASable
    }
    let condition = |index: u8, longueur: SeccompCmpArgLen, op: SeccompCmpOp, valeur: u64| {
        SeccompCondition::new(index, longueur, op, valeur).map_err(err)
    };
    let si_arg = |index: u8, op: SeccompCmpOp, valeur: u64| -> Result<SeccompRule, ErreurSas> {
        SeccompRule::new(vec![condition(index, SeccompCmpArgLen::Qword, op, valeur)?]).map_err(err)
    };
    let sans_exec = || condition(2, SeccompCmpArgLen::Qword, SeccompCmpOp::MaskedEq(libc::PROT_EXEC as u64), 0);
    // `mmap` anonyme seulement : `MAP_ANONYMOUS` posé et descripteur `-1`
    // (comparé sur 32 bits : `fd` est un `int`), jamais de projection de fichier.
    let anonyme = SeccompRule::new(vec![
        sans_exec()?,
        condition(
            3,
            SeccompCmpArgLen::Qword,
            SeccompCmpOp::MaskedEq(libc::MAP_ANONYMOUS as u64),
            libc::MAP_ANONYMOUS as u64,
        )?,
        condition(4, SeccompCmpArgLen::Dword, SeccompCmpOp::Eq, u64::from(u32::MAX))?,
    ])
    .map_err(err)?;

    let mut regles: BTreeMap<i64, Vec<SeccompRule>> = BTreeMap::new();
    regles.insert(libc::SYS_read, vec![si_arg(0, SeccompCmpOp::Eq, 0)?]);
    regles.insert(libc::SYS_write, vec![si_arg(0, SeccompCmpOp::Eq, 1)?, si_arg(0, SeccompCmpOp::Eq, 2)?]);
    regles.insert(libc::SYS_mmap, vec![anonyme]);
    regles.insert(libc::SYS_mprotect, vec![SeccompRule::new(vec![sans_exec()?]).map_err(err)?]);
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
    filtre.try_into().map_err(err)
}

#[cfg(target_os = "linux")]
fn appliquer_seccomp() -> Result<(), ErreurSas> {
    let programme = construire_filtre()?;
    seccompiler::apply_filter_all_threads(&programme).map_err(|_| ErreurSas::BacASable)
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    const RET_ALLOW: u32 = 0x7fff_0000;

    /// Interprète BPF classique minimal (opcodes produits par `seccompiler`).
    fn evaluer(programme: &[seccompiler::sock_filter], nr: i64, args: [u64; 6]) -> u32 {
        let arch: u32 = match std::env::consts::ARCH {
            "x86_64" => 0x3e | 0x8000_0000 | 0x4000_0000,
            "aarch64" => 0xb7 | 0x8000_0000 | 0x4000_0000,
            autre => panic!("architecture non couverte par l'essai : {autre}"),
        };
        let mut donnees = [0u8; 64];
        donnees[0..4].copy_from_slice(&(nr as u32).to_le_bytes());
        donnees[4..8].copy_from_slice(&arch.to_le_bytes());
        for (i, a) in args.iter().enumerate() {
            donnees[16 + 8 * i..24 + 8 * i].copy_from_slice(&a.to_le_bytes());
        }
        let mut acc: u32 = 0;
        let mut pc = 0usize;
        loop {
            let i = &programme[pc];
            pc += 1;
            match i.code {
                0x20 => {
                    let k = i.k as usize;
                    acc = u32::from_le_bytes(donnees[k..k + 4].try_into().unwrap());
                }
                0x54 => acc &= i.k,
                0x05 => pc += i.k as usize,
                0x15 | 0x25 | 0x35 => {
                    let vrai = match i.code {
                        0x15 => acc == i.k,
                        0x25 => acc > i.k,
                        _ => acc >= i.k,
                    };
                    pc += usize::from(if vrai { i.jt } else { i.jf });
                }
                0x06 => return i.k,
                autre => panic!("opcode BPF inattendu : {autre:#x}"),
            }
        }
    }

    #[test]
    fn mmap_anonyme_seul_admis() {
        let p = construire_filtre().unwrap();
        let rw = (libc::PROT_READ | libc::PROT_WRITE) as u64;
        let anon = (libc::MAP_PRIVATE | libc::MAP_ANONYMOUS) as u64;
        let moins_un = u64::MAX;
        // Allocation anonyme (fd = -1, sur 64 ou 32 bits) : admise.
        assert_eq!(evaluer(&p, libc::SYS_mmap, [0, 4096, rw, anon, moins_un, 0]), RET_ALLOW);
        assert_eq!(evaluer(&p, libc::SYS_mmap, [0, 4096, rw, anon, u64::from(u32::MAX), 0]), RET_ALLOW);
        // Projection d'un descripteur (hérité ou entrée standard) : tuée.
        assert_ne!(evaluer(&p, libc::SYS_mmap, [0, 4096, rw, libc::MAP_PRIVATE as u64, 0, 0]), RET_ALLOW);
        assert_ne!(evaluer(&p, libc::SYS_mmap, [0, 4096, rw, libc::MAP_SHARED as u64, 3, 0]), RET_ALLOW);
        // MAP_ANONYMOUS avec un vrai descripteur, ou fd = -1 sans MAP_ANONYMOUS : tués.
        assert_ne!(evaluer(&p, libc::SYS_mmap, [0, 4096, rw, anon, 3, 0]), RET_ALLOW);
        assert_ne!(evaluer(&p, libc::SYS_mmap, [0, 4096, rw, libc::MAP_PRIVATE as u64, moins_un, 0]), RET_ALLOW);
        // Exécutable : tué même anonyme.
        let rwx = rw | libc::PROT_EXEC as u64;
        assert_ne!(evaluer(&p, libc::SYS_mmap, [0, 4096, rwx, anon, moins_un, 0]), RET_ALLOW);
        // Témoins : lecture de 0 admise, de 3 tuée ; openat tué.
        assert_eq!(evaluer(&p, libc::SYS_read, [0, 0, 1, 0, 0, 0]), RET_ALLOW);
        assert_ne!(evaluer(&p, libc::SYS_read, [3, 0, 1, 0, 0, 0]), RET_ALLOW);
        assert_ne!(evaluer(&p, libc::SYS_openat, [0; 6]), RET_ALLOW);
    }

    #[test]
    fn drapeaux_fdinfo_octaux() {
        assert_eq!(drapeaux_fdinfo("pos:\t0\nflags:\t02100002\nmnt_id:\t1\n"), Some(0o2_100_002));
        assert_eq!(drapeaux_fdinfo("pos:\t0\nflags:\t0100002\n").map(|f| f & O_CLOEXEC_FDINFO), Some(0));
        assert_eq!(drapeaux_fdinfo("pos:\t0\n"), None);
    }
}
