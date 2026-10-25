//! @id     miyudouane_exemple_arretes_essai
//! @do     generer_arretes_essai_intrusion
//! @role   test
//! @layer  operateur
//! @human  miyudouane — arrêtés d'essai signés par une clé jetable, pour le banc d'intrusion du lot 3 (tests/intrusion-lot3).

//! Génère les arrêtés du banc d'intrusion local (critère de sortie du lot 3).
//!
//! ```text
//! cargo run --release -p miyudouane --example arretes_essai -- --sortie <répertoire> [--maintenant <secs>]
//! ```
//!
//! Écrit dans `<répertoire>` :
//!
//! - `ancre.hex` : clé publique de la clé **jetable** (tirée de `/dev/urandom`,
//!   jamais écrite sur disque, oubliée à la fin du programme) ;
//! - `externe.arrete`, `portail-interne.arrete`, `portail-local.arrete` :
//!   arrêtés scellés (Sceau v1.1), valides de `maintenant − 300 s` à
//!   `maintenant + 1 jour`.
//!
//! Scénario (doit rester aligné sur `tests/intrusion-lot3/scenario.sh`) :
//!
//! | Frontière       | Voie      | Flux                                      |
//! |-----------------|-----------|-------------------------------------------|
//! | portail-interne | `voie-a`  | COG → Franche, tcp 7443                   |
//! | portail-interne | `voie-b`  | COG → Franche, tcp 7080                   |
//! | portail-local   | `capteur` | COG → Franche locale, tcp 1883            |
//! | externe         | `voie-a`  | International (203.0.113.0/24) → Franche, tcp 443 |
//!
//! **Jamais** pour un déploiement réel : la clé des arrêtés est une clé
//! d'usage dérivée de la racine COG (cérémonie), pas une clé jetable.

use std::io::Read as _;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use borderguard::{sceller_arrete, Arrete, Constitution, Flux, Frontiere, Protocole, Voie, Zone};
use miyusceau::CleSignature;

/// Ports du scénario.
const PORT_A: u16 = 7443;
const PORT_B: u16 = 7080;
const PORT_CAPTEUR: u16 = 1883;
const PORT_EXTERNE: u16 = 443;
/// Liste blanche International du scénario (TEST-NET-3, RFC 5737).
const PAIRS_INTERNATIONAL: &str = "203.0.113.0/24";

fn voie(id: &str) -> Voie {
    Voie { id: id.into(), service: id.into() }
}

fn flux(voie: &str, source: Zone, destination: Zone, port: u16, pairs: &[&str]) -> Result<Flux, String> {
    let pairs = pairs
        .iter()
        .map(|p| p.parse().map_err(|()| format!("réseau invalide « {p} »")))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Flux {
        voie: voie.into(),
        source,
        destination,
        protocole: Protocole::Tcp,
        port,
        pairs,
        debit_max_kio_s: 512,
        taille_max_paquet: 1400,
    })
}

fn arrete(frontiere: Frontiere, maintenant: u64, voies: Vec<Voie>, flux: Vec<Flux>) -> Arrete {
    Arrete {
        frontiere,
        numero: 1,
        emis_le: maintenant.saturating_sub(300),
        pas_avant: maintenant.saturating_sub(300),
        pas_apres: maintenant + 86_400,
        emetteur: "banc-intrusion-lot3".into(),
        motif: "essai d'intrusion local, cle jetable".into(),
        voies,
        flux,
    }
}

fn graine_jetable() -> Result<[u8; 32], String> {
    let mut g = [0u8; 32];
    std::fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut g))
        .map_err(|e| format!("/dev/urandom : {e}"))?;
    Ok(g)
}

fn hex(octets: &[u8]) -> String {
    use std::fmt::Write as _;
    octets.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

fn executer() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut sortie: Option<PathBuf> = None;
    let mut maintenant = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    while let Some(a) = args.next() {
        let mut valeur = || args.next().ok_or_else(|| format!("{a} : valeur attendue"));
        match a.as_str() {
            "--sortie" => sortie = Some(PathBuf::from(valeur()?)),
            "--maintenant" => maintenant = valeur()?.parse().map_err(|_| "--maintenant : entier attendu")?,
            autre => return Err(format!("option inconnue « {autre} »")),
        }
    }
    let sortie = sortie.ok_or("--sortie <répertoire> attendu")?;
    std::fs::create_dir_all(&sortie).map_err(|e| format!("{}: {e}", sortie.display()))?;

    let cle = CleSignature::depuis_graine(&graine_jetable()?);
    let constitution = Constitution::reference();
    let arretes = [
        arrete(
            Frontiere::PortailInterne,
            maintenant,
            vec![voie("voie-a"), voie("voie-b")],
            vec![
                flux("voie-a", Zone::Cog, Zone::Franche, PORT_A, &[])?,
                flux("voie-b", Zone::Cog, Zone::Franche, PORT_B, &[])?,
            ],
        ),
        arrete(
            Frontiere::PortailLocal,
            maintenant,
            vec![voie("capteur")],
            vec![flux("capteur", Zone::Cog, Zone::FrancheLocale, PORT_CAPTEUR, &[])?],
        ),
        arrete(
            Frontiere::Externe,
            maintenant,
            vec![voie("voie-a")],
            vec![flux("voie-a", Zone::International, Zone::Franche, PORT_EXTERNE, &[PAIRS_INTERNATIONAL])?],
        ),
    ];
    for a in &arretes {
        let octets = sceller_arrete(a, &cle, &constitution).map_err(|e| format!("{} : {e}", a.frontiere))?;
        let chemin = sortie.join(format!("{}.arrete", a.frontiere.nom()));
        std::fs::write(&chemin, octets).map_err(|e| format!("{}: {e}", chemin.display()))?;
    }
    let ancre = hex(&cle.cle_publique());
    std::fs::write(sortie.join("ancre.hex"), format!("{ancre}\n")).map_err(|e| format!("ancre.hex : {e}"))?;
    println!("{ancre}");
    Ok(())
}

fn main() -> ExitCode {
    match executer() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("arretes_essai : {e}");
            ExitCode::FAILURE
        }
    }
}
