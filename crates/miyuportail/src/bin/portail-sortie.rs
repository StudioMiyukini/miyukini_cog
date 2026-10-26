//! @id     miyuportail_bin_sortie
//! @do     lancer_sortie_portail
//! @role   securite
//! @layer  kernel
//! @human  portail-sortie — processus « sortie » du Portail scindé (zone COG → Franche) : réponses à jeton à usage unique et publications H1, signées par la clé de la voie ; aucune capacité de relever (A-022 §3).

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
    std::process::exit(miyuportail::demon::cli::executer_sortie(&std::env::args().skip(1).collect::<Vec<_>>()));
}

#[cfg(not(target_os = "linux"))]
fn main() {
    std::process::exit(2);
}
