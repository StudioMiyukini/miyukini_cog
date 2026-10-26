//! @id     miyuportail_bin_entree
//! @do     lancer_entree_portail
//! @role   securite
//! @layer  kernel
//! @human  portail-entree — processus « entrée » du Portail scindé (Franche → zone COG) : relève les dépôts, sas, deuxième validation, remise au moteur ; aucune capacité de publier (A-022 §3).

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

#[cfg(target_os = "linux")]
fn main() {
    std::process::exit(miyuportail::demon::cli::executer_entree(&std::env::args().skip(1).collect::<Vec<_>>()));
}

#[cfg(not(target_os = "linux"))]
fn main() {
    std::process::exit(2);
}
