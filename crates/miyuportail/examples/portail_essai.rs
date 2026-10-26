//! @id     miyuportail_exemple_portail_essai
//! @do     jouer_cores_facade_moteur_portail_scinde
//! @role   test
//! @layer  toolkit
//! @human  portail_essai — outil d'essai (VM jetable) du Portail scindé : joue les Cores (descripteurs), la façade (dépôt, lecture vérifiée) et le moteur (relève, réponse) autour des processus portail-entree et portail-sortie.

//! Clés d'essai **jetables** dans un fichier d'état (jamais en production).
//!
//! ```text
//! portail_essai cores    --cles F --service S --voies-entree D --voies-sortie D
//!                        [--etat active|desactivee|coupee] [--rotation N] [--entree-uid U --entree-gid G]
//! portail_essai deposer  --cles F --service S --depot D --sequence N
//! portail_essai relever  --admises D --jeton F            (écrit le jeton de la 1re admise, affiche sa séquence)
//! portail_essai repondre --reponses D --jeton F --texte T
//! portail_essai lire     --publication D --service S      (vérifie chaque trame, affiche leur nombre)
//! ```

#![allow(clippy::print_stdout, clippy::print_stderr, clippy::many_single_char_names)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use ed25519_dalek::SigningKey;
use miyuportail::demon::nom_cle_publication;
use miyuportail::files::{ecrire_descripteur, Admise, DescripteurVoie, File, Reponse, Role, SessionDecrite, EXT_ADMISE, EXT_MSCE, EXT_REPONSE};
use miyuportail::msce::{decoder_trame, sceller, Contexte};
use miyuportail::*;
use rand_core::{OsRng, RngCore};

const ENTRANT: u64 = 1;
const SORTANT: u64 = 101;

fn traite() -> Traite {
    Traite {
        version: 1,
        types: vec![
            TypeDeclaration {
                code: ENTRANT,
                sens: Sens::Entrante,
                champs: vec![ChampTraite { tag: 1, nature: NatureChamp::Entier, obligatoire: true }],
            },
            TypeDeclaration {
                code: SORTANT,
                sens: Sens::Sortante,
                champs: vec![ChampTraite { tag: 1, nature: NatureChamp::Texte { max: 128 }, obligatoire: true }],
            },
        ],
    }
}

fn hex(o: &[u8]) -> String {
    use std::fmt::Write;
    o.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

fn unhex32(t: &str) -> Result<[u8; 32], String> {
    let t = t.trim();
    if t.len() != 64 {
        return Err("hex de 32 octets attendu".into());
    }
    let mut o = [0u8; 32];
    for (i, b) in o.iter_mut().enumerate() {
        *b = u8::from_str_radix(&t[2 * i..2 * i + 2], 16).map_err(|e| e.to_string())?;
    }
    Ok(o)
}

/// Clés d'essai : façade, auteur, session, graine des jetons.
struct Cles {
    facade: [u8; 32],
    auteur: [u8; 32],
    session: [u8; 32],
    jetons: [u8; 32],
}

fn alea() -> [u8; 32] {
    let mut o = [0u8; 32];
    OsRng.fill_bytes(&mut o);
    o
}

fn cles(chemin: &str, creer: bool) -> Result<Cles, String> {
    if let Ok(t) = std::fs::read_to_string(chemin) {
        let l: Vec<&str> = t.lines().collect();
        if l.len() == 4 {
            return Ok(Cles { facade: unhex32(l[0])?, auteur: unhex32(l[1])?, session: unhex32(l[2])?, jetons: unhex32(l[3])? });
        }
    }
    if !creer {
        return Err(format!("{chemin} : clés d'essai absentes"));
    }
    let c = Cles { facade: alea(), auteur: alea(), session: alea(), jetons: alea() };
    let texte = format!("{}\n{}\n{}\n{}\n", hex(&c.facade), hex(&c.auteur), hex(&c.session), hex(&c.jetons));
    std::fs::write(chemin, texte).map_err(|e| e.to_string())?;
    Ok(c)
}

fn args(v: &[String]) -> Result<BTreeMap<String, String>, String> {
    let mut m = BTreeMap::new();
    let mut it = v.iter();
    while let Some(k) = it.next() {
        let k = k.strip_prefix("--").ok_or_else(|| format!("option attendue : {k}"))?;
        m.insert(k.to_string(), it.next().ok_or_else(|| format!("valeur manquante : --{k}"))?.clone());
    }
    Ok(m)
}

fn opt<'a>(m: &'a BTreeMap<String, String>, k: &str) -> Result<&'a str, String> {
    m.get(k).map(String::as_str).ok_or_else(|| format!("--{k} requis"))
}

fn service(m: &BTreeMap<String, String>) -> Result<IdService, String> {
    IdService::new(opt(m, "service")?).map_err(|e| e.to_string())
}

fn cores(m: &BTreeMap<String, String>) -> Result<(), String> {
    let c = cles(opt(m, "cles")?, true)?;
    let s = service(m)?;
    let etat = match m.get("etat").map_or("active", String::as_str) {
        "active" => EtatVoie::Active,
        "desactivee" => EtatVoie::Desactivee,
        "coupee" => EtatVoie::Coupee(MotifCoupure::Decision),
        x => return Err(format!("état inconnu : {x}")),
    };
    let rotation: u64 = m.get("rotation").map_or(Ok(0), |r| r.parse()).map_err(|_| "rotation".to_string())?;
    let contrat = ContratVoies {
        service: s,
        classe: ClasseExposition::H2,
        traite: traite(),
        taille_max_trame: 8192,
        debit_max_par_minute: 600,
        quota_file: 64,
    };
    let jetons = CleJetons::depuis_graine(&c.jetons);
    let active = etat == EtatVoie::Active;
    let e = DescripteurVoie {
        role: Role::Entree,
        contrat: contrat.clone(),
        etat,
        rotation,
        session: active.then(|| SessionDecrite::Entree {
            id: c.session,
            cle_facade: SigningKey::from_bytes(&c.facade).verifying_key().to_bytes(),
            auteurs: vec![SigningKey::from_bytes(&c.auteur).verifying_key().to_bytes()],
            graine_jetons: c.jetons,
        }),
    };
    let so = DescripteurVoie {
        role: Role::Sortie,
        contrat,
        etat,
        rotation,
        session: active.then(|| SessionDecrite::Sortie { id: c.session, ancre_jetons: jetons.ancre() }),
    };
    let proprietaire = match (m.get("entree-uid"), m.get("entree-gid")) {
        (Some(u), Some(g)) => Some((u.parse().map_err(|_| "uid")?, g.parse().map_err(|_| "gid")?)),
        _ => None,
    };
    ecrire_descripteur(&PathBuf::from(opt(m, "voies-entree")?), &e, proprietaire).map_err(|e| e.to_string())?;
    ecrire_descripteur(&PathBuf::from(opt(m, "voies-sortie")?), &so, None).map_err(|e| e.to_string())?;
    println!("descripteurs écrits ({etat:?})");
    Ok(())
}

fn deposer(m: &BTreeMap<String, String>) -> Result<(), String> {
    let c = cles(opt(m, "cles")?, false)?;
    let s = service(m)?;
    let sequence: u64 = opt(m, "sequence")?.parse().map_err(|_| "séquence")?;
    let d = Declaration {
        service: s.clone(),
        sens: Sens::Entrante,
        version_traite: 1,
        type_code: ENTRANT,
        sequence,
        emise_le: 1_790_000_000,
        champs: vec![Champ { tag: 1, valeur: Valeur::Entier(sequence) }],
        signataires: Vec::new(),
    };
    let t = sceller(&d, &[&SigningKey::from_bytes(&c.facade), &SigningKey::from_bytes(&c.auteur)]).map_err(|e| e.to_string())?;
    let nom = File::new(opt(m, "depot")?).ecrire(&s, EXT_MSCE, &t).map_err(|e| e.to_string())?;
    println!("{nom}");
    Ok(())
}

fn relever(m: &BTreeMap<String, String>) -> Result<(), String> {
    let f = File::new(opt(m, "admises")?);
    let e = f.elements(1024).map_err(|e| e.to_string())?;
    let Some(e) = e.into_iter().find(|e| e.extension == EXT_ADMISE) else {
        return Err("aucune déclaration admise".into());
    };
    let a = Admise::decoder(&f.lire(&e.nom, 1 << 22).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    f.retirer(&e.nom).map_err(|e| e.to_string())?;
    std::fs::write(opt(m, "jeton")?, hex(a.jeton.octets())).map_err(|e| e.to_string())?;
    println!("{}", a.declaration.sequence);
    Ok(())
}

fn repondre(m: &BTreeMap<String, String>) -> Result<(), String> {
    let t = std::fs::read_to_string(opt(m, "jeton")?).map_err(|e| e.to_string())?;
    let t = t.trim();
    let octets: Vec<u8> =
        (0..t.len() / 2).map(|i| u8::from_str_radix(&t[2 * i..2 * i + 2], 16)).collect::<Result<_, _>>().map_err(|e| e.to_string())?;
    let jeton = JetonReponse::decoder(&octets).map_err(|e| e.to_string())?;
    let s = jeton.service().clone();
    let message = Declaration {
        service: s.clone(),
        sens: Sens::Sortante,
        version_traite: 1,
        type_code: SORTANT,
        sequence: 0,
        emise_le: 0,
        champs: vec![Champ { tag: 1, valeur: Valeur::Texte(opt(m, "texte")?.to_string()) }],
        signataires: Vec::new(),
    };
    let r = Reponse { jeton, messages: vec![message] }.encoder().map_err(|e| e.to_string())?;
    println!("{}", File::new(opt(m, "reponses")?).ecrire(&s, EXT_REPONSE, &r).map_err(|e| e.to_string())?);
    Ok(())
}

fn lire(m: &BTreeMap<String, String>) -> Result<(), String> {
    let s = service(m)?;
    let f = File::new(opt(m, "publication")?);
    let cle = unhex32(&String::from_utf8_lossy(&f.lire(&nom_cle_publication(&s), 200).map_err(|e| e.to_string())?))?;
    let ancres = Ancres { emetteur: cle, auteurs: Vec::new() };
    let t = traite();
    let ctx = Contexte { service: &s, sens: Sens::Sortante, traite: &t, ancres: &ancres };
    let mut n = 0;
    for e in f.elements(4096).map_err(|e| e.to_string())? {
        if e.extension != EXT_MSCE || e.service.as_ref() != Some(&s) {
            continue;
        }
        let d = decoder_trame(&f.lire(&e.nom, 1 << 20).map_err(|e| e.to_string())?, &ctx).map_err(|x| format!("{}: {x}", e.nom))?;
        f.retirer(&e.nom).map_err(|e| e.to_string())?;
        eprintln!("publication {} vérifiée : {:?}", d.sequence, d.champs);
        n += 1;
    }
    println!("{n}");
    Ok(())
}

fn main() {
    let v: Vec<String> = std::env::args().skip(1).collect();
    let r = match v.split_first() {
        Some((cmd, reste)) => args(reste).and_then(|m| match cmd.as_str() {
            "cores" => cores(&m),
            "deposer" => deposer(&m),
            "relever" => relever(&m),
            "repondre" => repondre(&m),
            "lire" => lire(&m),
            x => Err(format!("commande inconnue : {x}")),
        }),
        None => Err("commande attendue".into()),
    };
    if let Err(e) = r {
        eprintln!("portail_essai : {e}");
        std::process::exit(1);
    }
}
