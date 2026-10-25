//! @id     miyurelais_bin
//! @do     relayer_nflog_en_lignes
//! @role   operateur
//! @layer  operateur
//! @human  miyurelais — binaire : écoute des groupes NFLOG et écriture des lignes canoniques MDOU| sur la sortie standard.

//! `miyurelais --groupe <G> [--groupe <G2>…] --hote <nom> [--copie <octets>]
//! [--delai-muet-s <N> --voie-active] [--releve-ms <ms>] [--duree-s <s>]`
//!
//! Écrit une ligne canonique par paquet journalisé sur la sortie standard
//! (format : module `ligne`). Les échecs de relève (pertes, capteur muet,
//! socket) sont écrits sur l'erreur standard, préfixés `ECHEC ` ; les lignes
//! des relèves suivantes continuent. Code de sortie : 0 fin normale
//! (`--duree-s`), 2 arguments, 3 ouverture de la socket.

use std::io::Write as _;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use miyurelais::{ConfigRelais, ParametresInstance, Relais, SocketNflog};
use miyuvigie::CapteurId;

struct Options {
    groupes: Vec<u16>,
    hote: String,
    copie: u32,
    delai_muet_s: u64,
    voie_active: bool,
    releve_ms: u64,
    duree_s: Option<u64>,
}

fn analyser(args: &[String]) -> Result<Options, String> {
    let mut o = Options {
        groupes: Vec::new(),
        hote: String::new(),
        copie: ParametresInstance::default().copie,
        delai_muet_s: 10,
        voie_active: false,
        releve_ms: 200,
        duree_s: None,
    };
    let mut i = 0;
    while i < args.len() {
        let valeur = |i: usize| args.get(i + 1).cloned().ok_or_else(|| format!("valeur manquante après {}", args[i]));
        match args[i].as_str() {
            "--groupe" => o.groupes.push(valeur(i)?.parse().map_err(|_| "groupe invalide")?),
            "--hote" => o.hote = valeur(i)?,
            "--copie" => o.copie = valeur(i)?.parse().map_err(|_| "copie invalide")?,
            "--delai-muet-s" => o.delai_muet_s = valeur(i)?.parse().map_err(|_| "délai invalide")?,
            "--releve-ms" => o.releve_ms = valeur(i)?.parse().map_err(|_| "période invalide")?,
            "--duree-s" => o.duree_s = Some(valeur(i)?.parse().map_err(|_| "durée invalide")?),
            "--voie-active" => {
                o.voie_active = true;
                i += 1;
                continue;
            }
            a => return Err(format!("argument inconnu : {a}")),
        }
        i += 2;
    }
    if o.groupes.is_empty() || o.hote.is_empty() {
        return Err("--groupe et --hote sont obligatoires".into());
    }
    Ok(o)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let o = match analyser(&args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("miyurelais : {e}");
            return ExitCode::from(2);
        }
    };
    let parametres = ParametresInstance { copie: o.copie, ..ParametresInstance::default() };
    let socket = match SocketNflog::ouvrir(&o.groupes, parametres) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("miyurelais : {e}");
            return ExitCode::from(3);
        }
    };
    let Some(capteur) = CapteurId::new("relais-nflog") else {
        return ExitCode::from(2);
    };
    let config = ConfigRelais {
        capteur,
        hote: o.hote.clone(),
        groupes: o.groupes.clone(),
        delai_muet_ms: o.delai_muet_s.saturating_mul(1000),
    };
    let mut relais = match Relais::new(config, socket) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("miyurelais : {e}");
            return ExitCode::from(2);
        }
    };
    if o.voie_active {
        relais.activer_voie();
    }
    let debut = Instant::now();
    let mut sortie = std::io::stdout().lock();
    loop {
        match relais.relever() {
            Ok(r) => {
                for l in &r.lignes {
                    let _ = writeln!(sortie, "{l}");
                }
                if r.mal_formees > 0 {
                    eprintln!("DEGRADE {} paquet(s) non convertis", r.mal_formees);
                }
                let _ = sortie.flush();
            }
            Err(e) => eprintln!("ECHEC {}", e.0),
        }
        if o.duree_s.is_some_and(|d| debut.elapsed() >= Duration::from_secs(d)) {
            break;
        }
        std::thread::sleep(Duration::from_millis(o.releve_ms));
    }
    let s = relais.statistiques();
    eprintln!(
        "STATS datagrammes={} paquets={} lignes={} temoins={} refuses={} pertes={} debordements={}",
        s.datagrammes, s.paquets, s.lignes, s.temoins, s.paquets_refuses, s.pertes, s.debordements
    );
    ExitCode::SUCCESS
}
