//! @id     miyudouane_cli
//! @do     piloter_douane_en_ligne_de_commande
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — CLI : génération à blanc (par défaut) ou application explicite (`--appliquer`).

//! CLI de la Douane.
//!
//! ```text
//! miyudouane regles --frontiere <externe|portail-interne|portail-local> --arrete <fichier>
//!            --ancre <clé publique hex> [--maintenant <secs>] [--dernier <n°>]
//!            [--sandbox <zone>:<service>]… [--appliquer --fichier <chemin>]
//! miyudouane fermeture --frontiere <f> [--appliquer --fichier <chemin>]
//! miyudouane zones [--sandbox <zone>:<service>]…
//! ```
//!
//! Sans `--appliquer`, rien n'est exécuté : le texte est imprimé (dry-run).
//! Avec `--appliquer` : audit, `nft -c -f`, puis `nft -f` dans l'espace de
//! noms de la frontière (droits root requis).

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use borderguard::{verifier_arrete, Constitution, Frontiere, Zone};
use miyudouane::{application, nft, zones, InstanceDouane, PlanReseau};
use miyusceau::{AncresDeConfiance, NiveauConfiance};

struct Options {
    commande: String,
    frontiere: Option<Frontiere>,
    arrete: Option<PathBuf>,
    ancre: Option<[u8; 32]>,
    maintenant: Option<u64>,
    dernier: u64,
    sandboxes: Vec<(Zone, String)>,
    appliquer: bool,
    fichier: Option<PathBuf>,
}

fn hex32(s: &str) -> Result<[u8; 32], String> {
    if s.len() != 64 || !s.is_ascii() {
        return Err("--ancre : 64 caractères hexadécimaux attendus".into());
    }
    let mut o = [0u8; 32];
    for (i, b) in o.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).map_err(|_| "--ancre : hexadécimal invalide".to_string())?;
    }
    Ok(o)
}

fn zone(s: &str) -> Result<Zone, String> {
    Zone::TOUTES.into_iter().find(|z| z.nom() == s).ok_or_else(|| format!("zone inconnue « {s} »"))
}

fn lire_options() -> Result<Options, String> {
    let mut args = std::env::args().skip(1);
    let commande = args.next().ok_or("commande attendue : regles | fermeture | zones")?;
    let mut o = Options {
        commande,
        frontiere: None,
        arrete: None,
        ancre: None,
        maintenant: None,
        dernier: 0,
        sandboxes: Vec::new(),
        appliquer: false,
        fichier: None,
    };
    while let Some(a) = args.next() {
        let mut valeur = || args.next().ok_or_else(|| format!("{a} : valeur attendue"));
        match a.as_str() {
            "--frontiere" => o.frontiere = Some(valeur()?.parse().map_err(|()| "frontière inconnue".to_string())?),
            "--arrete" => o.arrete = Some(PathBuf::from(valeur()?)),
            "--ancre" => o.ancre = Some(hex32(&valeur()?)?),
            "--maintenant" => o.maintenant = Some(valeur()?.parse().map_err(|_| "--maintenant : entier attendu")?),
            "--dernier" => o.dernier = valeur()?.parse().map_err(|_| "--dernier : entier attendu")?,
            "--sandbox" => {
                let v = valeur()?;
                let (z, s) = v.split_once(':').ok_or("--sandbox : <zone>:<service>")?;
                o.sandboxes.push((zone(z)?, s.to_owned()));
            }
            "--appliquer" => o.appliquer = true,
            "--fichier" => o.fichier = Some(PathBuf::from(valeur()?)),
            autre => return Err(format!("option inconnue « {autre} »")),
        }
    }
    Ok(o)
}

fn executer() -> Result<(), String> {
    let o = lire_options()?;
    let services: Vec<(Zone, &str)> = o.sandboxes.iter().map(|(z, s)| (*z, s.as_str())).collect();
    let plan = PlanReseau::reference(&services).map_err(|e| e.to_string())?;
    let maintenant = o.maintenant.unwrap_or_else(|| {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
    });
    match o.commande.as_str() {
        "zones" => {
            println!("{}", zones::script_mise_en_place(&plan));
            for (chemin, contenu) in zones::unites_systemd(&plan) {
                println!("# ---- {chemin}\n{contenu}");
            }
            Ok(())
        }
        "regles" | "fermeture" => {
            let frontiere = o.frontiere.ok_or("--frontiere attendu")?;
            let mut instance = InstanceDouane::nouvelle(frontiere, plan.clone(), o.dernier).map_err(|e| e.to_string())?;
            let mut voies = BTreeSet::new();
            if o.commande == "regles" {
                let chemin = o.arrete.ok_or("--arrete attendu")?;
                let octets = std::fs::read(&chemin).map_err(|e| format!("{}: {e}", chemin.display()))?;
                let mut ancres = AncresDeConfiance::new();
                ancres
                    .ajouter(&o.ancre.ok_or("--ancre attendu (clé d'usage « arrêtés », épinglée)")?, NiveauConfiance::Souverain)
                    .map_err(|e| format!("ancre : {e:?}"))?;
                let verifie = verifier_arrete(&octets, &ancres, &Constitution::reference(), maintenant)
                    .map_err(|e| e.to_string())?;
                voies = verifie.arrete().voies.iter().map(|v| v.id.clone()).collect();
                instance.charger(verifie, maintenant).map_err(|e| e.to_string())?;
            }
            let texte = instance.ruleset(maintenant, &BTreeSet::new());
            if o.appliquer {
                let def = plan.frontiere(frontiere).ok_or("frontière hors plan")?;
                let fichier = o.fichier.unwrap_or_else(|| PathBuf::from(zones::chemin_regles(&def.netns)));
                application::appliquer(&texte, &voies, Some(&def.netns), &fichier).map_err(|e| e.to_string())?;
                eprintln!("appliqué : {} ({})", def.netns, fichier.display());
            } else {
                let ecarts = nft::auditer(&texte, &voies);
                if !ecarts.is_empty() {
                    return Err(format!("audit : {}", ecarts.join(" ; ")));
                }
                print!("{texte}");
            }
            Ok(())
        }
        autre => Err(format!("commande inconnue « {autre} »")),
    }
}

fn main() -> ExitCode {
    match executer() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("miyudouane : {e}");
            ExitCode::FAILURE
        }
    }
}
