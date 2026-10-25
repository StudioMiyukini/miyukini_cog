//! @id     miyuportail_bin_sas
//! @do     lancer_sas_decodage
//! @role   securite
//! @layer  kernel
//! @human  portail-sas — processus de décodage confiné (seccomp + rlimits), lancé par lot par le Portail interne (EX-46).

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

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(miyuportail::sas::executer(&arguments));
}
