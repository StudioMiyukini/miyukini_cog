//! @id     miyuportail_bin_sas
//! @do     lancer_sas_decodage
//! @role   securite
//! @layer  kernel
//! @human  portail-sas — processus de décodage confiné (seccomp + rlimits), lancé par lot par le Portail interne (EX-46).

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(miyuportail::sas::executer(&arguments));
}
