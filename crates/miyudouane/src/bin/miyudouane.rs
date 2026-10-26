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
//!            [--couper <voie>]…
//! miyudouane fermeture --frontiere <f> [--appliquer --fichier <chemin>]
//! miyudouane zone --zone <cog|franche|locale> --arrete <frontière>=<fichier>…
//!            --ancre <clé publique hex> [--maintenant <secs>] [--sandbox <zone>:<service>]…
//!            [--couper <voie>]… [--appliquer --fichier <chemin>]
//! miyudouane zones [--sandbox <zone>:<service>]…
//! miyudouane mise-en-place [--sandbox <zone>:<service>]…
//! miyudouane retrait [--sandbox <zone>:<service>]…
//! miyudouane hote --description <fichier> [--arrete <portail>=<fichier>… --ancre <hex>]
//!            [--maintenant <secs>] [--sandbox <zone>:<service>]… [--couper <voie>]…
//!            [--appliquer [--fichier <chemin>]]
//! miyudouane tc --description <fichier> --arrete <fichier> --ancre <hex>
//!            [--maintenant <secs>] [--sandbox <zone>:<service>]… [--couper <voie>]…
//!            [--appliquer [--fichier <chemin>]]
//! ```
//!
//! - `regles` : règles d'une frontière ; `--couper` omet une voie (EX-49).
//! - `zone` : règles du pont filtré d'une zone (segmentation par service),
//!   compilées depuis les arrêtés de **toutes** les frontières qui touchent la
//!   zone (chacun vérifié contre l'ancre) ; `--couper` omet une voie.
//! - `hote` : table `inet miyudouane_hote` de l'hôte (services COG lancés sur
//!   l'hôte) depuis une description validée (format : module
//!   `miyudouane::hote`) ; exceptions de sortie seulement par voie d'un arrêté
//!   signé de portail (`--arrete portail-interne=…`). Avec `--appliquer` :
//!   `nft -c -f` puis `nft -f` sur l'hôte, seule cette table est remplacée
//!   (défaut `--fichier` : `/etc/miyukini/douane/miyudouane-hote.nft`).
//! - `tc` : mise en forme du trafic de la frontière externe (A-022 §7,
//!   module `miyudouane::tc`) : débits et marques lus de l'arrêté **signé**
//!   de la frontière externe (en vigueur), capacités d'interfaces lues de la
//!   description (`miyudouane-tc 1`) ; `--couper` omet une voie comme
//!   `regles`. Avec `--appliquer` : audit, refus de toute racine `tc`
//!   étrangère, puis `tc -batch` dans l'espace de la frontière (défaut
//!   `--fichier` : `/etc/miyukini/douane/<netns>.tc`).
//! - `zones` : script de mise en place + unités systemd ; `mise-en-place` et
//!   `retrait` : le script seul (pour un banc d'essai).
//!
//! Sans `--appliquer`, rien n'est exécuté : le texte est imprimé (dry-run).
//! Avec `--appliquer` : audit, `nft -c -f`, puis `nft -f` dans l'espace de
//! noms de la frontière ou de la zone (droits root requis).
//!
//! L'ordre des `--sandbox` fixe les adresses et les ports de pont : il doit
//! être identique pour toutes les commandes d'un même déploiement.

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

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use borderguard::{verifier_arrete, ArreteVerifie, Constitution, Frontiere, Zone};
use miyudouane::hote::{self, DescriptionHote};
use miyudouane::tc::{self, DescriptionTc};
use miyudouane::{application, compiler, nft, zones, InstanceDouane, PlanReseau};
use miyusceau::{AncresDeConfiance, NiveauConfiance};

struct Options {
    commande: String,
    frontiere: Option<Frontiere>,
    arretes: Vec<String>,
    zone: Option<Zone>,
    coupees: BTreeSet<String>,
    ancre: Option<[u8; 32]>,
    maintenant: Option<u64>,
    dernier: u64,
    sandboxes: Vec<(Zone, String)>,
    appliquer: bool,
    fichier: Option<PathBuf>,
    description: Option<PathBuf>,
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
    let commande =
        args.next().ok_or("commande attendue : regles | fermeture | zone | zones | mise-en-place | retrait | hote | tc")?;
    let mut o = Options {
        commande,
        frontiere: None,
        arretes: Vec::new(),
        zone: None,
        coupees: BTreeSet::new(),
        ancre: None,
        maintenant: None,
        dernier: 0,
        sandboxes: Vec::new(),
        appliquer: false,
        fichier: None,
        description: None,
    };
    while let Some(a) = args.next() {
        let mut valeur = || args.next().ok_or_else(|| format!("{a} : valeur attendue"));
        match a.as_str() {
            "--frontiere" => o.frontiere = Some(valeur()?.parse().map_err(|()| "frontière inconnue".to_string())?),
            "--arrete" => o.arretes.push(valeur()?),
            "--zone" => o.zone = Some(zone(&valeur()?)?),
            "--couper" => {
                let v = valeur()?;
                if !borderguard::arrete::identifiant_valide(&v) {
                    return Err(format!("--couper : identifiant de voie invalide « {v} »"));
                }
                o.coupees.insert(v);
            }
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
            "--description" => o.description = Some(PathBuf::from(valeur()?)),
            autre => return Err(format!("option inconnue « {autre} »")),
        }
    }
    Ok(o)
}

/// Lit et vérifie un arrêté scellé contre l'ancre épinglée (fail-closed).
fn lire_arrete(chemin: &str, ancre: Option<&[u8; 32]>, maintenant: u64) -> Result<ArreteVerifie, String> {
    let octets = std::fs::read(chemin).map_err(|e| format!("{chemin}: {e}"))?;
    let mut ancres = AncresDeConfiance::new();
    ancres
        .ajouter(ancre.ok_or("--ancre attendu (clé d'usage « arrêtés », épinglée)")?, NiveauConfiance::Souverain)
        .map_err(|e| format!("ancre : {e:?}"))?;
    verifier_arrete(&octets, &ancres, &Constitution::reference(), maintenant).map_err(|e| format!("{chemin}: {e}"))
}

/// Lit une description d'hôte (taille bornée avant analyse).
fn lire_description(chemin: &std::path::Path) -> Result<DescriptionHote, String> {
    use std::io::Read as _;
    let f = std::fs::File::open(chemin).map_err(|e| format!("{}: {e}", chemin.display()))?;
    let mut texte = String::new();
    f.take(hote::TAILLE_MAX_DESCRIPTION as u64 + 1)
        .read_to_string(&mut texte)
        .map_err(|e| format!("{}: {e}", chemin.display()))?;
    DescriptionHote::analyser(&texte).map_err(|e| format!("{}: {e}", chemin.display()))
}

/// Lit un fichier texte borné (taille vérifiée avant analyse).
fn lire_borne(chemin: &std::path::Path, max: usize) -> Result<String, String> {
    use std::io::Read as _;
    let f = std::fs::File::open(chemin).map_err(|e| format!("{}: {e}", chemin.display()))?;
    let mut texte = String::new();
    f.take(max as u64 + 1).read_to_string(&mut texte).map_err(|e| format!("{}: {e}", chemin.display()))?;
    Ok(texte)
}

/// Commande `tc` : mise en forme du trafic de la frontière externe.
fn commande_tc(o: Options, plan: &PlanReseau, maintenant: u64) -> Result<(), String> {
    let chemin = o.description.as_deref().ok_or("--description <fichier> attendu")?;
    let d = DescriptionTc::analyser(&lire_borne(chemin, tc::TAILLE_MAX_DESCRIPTION)?)
        .map_err(|e| format!("{}: {e}", chemin.display()))?;
    let [arrete] = o.arretes.as_slice() else {
        return Err("--arrete <fichier> attendu (une seule fois, frontière externe)".into());
    };
    let verifie = lire_arrete(arrete, o.ancre.as_ref(), maintenant)?;
    if !verifie.en_vigueur(maintenant) {
        return Err(format!("{arrete} : arrêté hors vigueur"));
    }
    if let Some(v) = o.coupees.iter().find(|v| !verifie.arrete().voies.iter().any(|x| x.id == **v)) {
        return Err(format!("--couper : voie « {v} » absente de l'arrêté"));
    }
    let p = tc::planifier(&verifie, plan, &d, &o.coupees).map_err(|e| e.to_string())?;
    let texte = tc::generer_tc(&p);
    if o.appliquer {
        let fichier =
            o.fichier.unwrap_or_else(|| PathBuf::from(format!("{}/{}.tc", zones::REPERTOIRE_REGLES, p.netns)));
        application::appliquer_tc(&texte, &p, &fichier).map_err(|e| e.to_string())?;
        eprintln!("appliqué : mise en forme {} ({})", p.netns, fichier.display());
    } else {
        let ecarts = tc::auditer_tc(&texte, &p);
        if !ecarts.is_empty() {
            return Err(format!("audit : {}", ecarts.join(" ; ")));
        }
        print!("{texte}");
    }
    Ok(())
}

/// Commande `hote` : table de la zone COG sur l'hôte.
fn commande_hote(o: Options, plan: &PlanReseau, maintenant: u64) -> Result<(), String> {
    let chemin = o.description.as_deref().ok_or("--description <fichier> attendu")?;
    let d = lire_description(chemin)?;
    let mut arretes = Vec::new();
    let mut voies = BTreeSet::new();
    for spec in &o.arretes {
        let (f, chemin) = spec.split_once('=').ok_or("--arrete : <frontière>=<fichier> attendu pour « hote »")?;
        let f: Frontiere = f.parse().map_err(|()| format!("frontière inconnue « {f} »"))?;
        let verifie = lire_arrete(chemin, o.ancre.as_ref(), maintenant)?;
        if verifie.arrete().frontiere != f {
            return Err(format!("{chemin} : arrêté de la frontière {}, pas {f}", verifie.arrete().frontiere));
        }
        voies.extend(verifie.arrete().voies.iter().map(|v| v.id.clone()));
        arretes.push(verifie);
    }
    if let Some(v) = o.coupees.iter().find(|v| !voies.contains(*v)) {
        return Err(format!("--couper : voie « {v} » absente des arrêtés"));
    }
    let exceptions = hote::exceptions_autorisees(&d, &arretes, plan, &o.coupees, maintenant)?;
    let texte = hote::generer_hote(&d, &exceptions);
    if o.appliquer {
        let fichier = o
            .fichier
            .unwrap_or_else(|| PathBuf::from(format!("{}/miyudouane-hote.nft", zones::REPERTOIRE_REGLES)));
        application::appliquer_hote(&texte, &d.exceptions, &fichier).map_err(|e| e.to_string())?;
        eprintln!("appliqué : table inet {} ({})", hote::TABLE_HOTE, fichier.display());
    } else {
        let ecarts = hote::auditer_hote(&texte, &d.exceptions);
        if !ecarts.is_empty() {
            return Err(format!("audit : {}", ecarts.join(" ; ")));
        }
        print!("{texte}");
    }
    Ok(())
}

fn appliquer_ou_imprimer(
    texte: &str,
    voies: &BTreeSet<String>,
    appliquer: bool,
    netns: &str,
    fichier: Option<PathBuf>,
) -> Result<(), String> {
    if appliquer {
        let fichier = fichier.unwrap_or_else(|| PathBuf::from(zones::chemin_regles(netns)));
        application::appliquer(texte, voies, Some(netns), &fichier).map_err(|e| e.to_string())?;
        eprintln!("appliqué : {netns} ({})", fichier.display());
    } else {
        let ecarts = nft::auditer(texte, voies);
        if !ecarts.is_empty() {
            return Err(format!("audit : {}", ecarts.join(" ; ")));
        }
        print!("{texte}");
    }
    Ok(())
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
        "mise-en-place" => {
            print!("{}", zones::script_mise_en_place(&plan));
            Ok(())
        }
        "retrait" => {
            print!("{}", zones::script_retrait(&plan));
            Ok(())
        }
        "regles" | "fermeture" => {
            let frontiere = o.frontiere.ok_or("--frontiere attendu")?;
            let mut instance = InstanceDouane::nouvelle(frontiere, plan.clone(), o.dernier).map_err(|e| e.to_string())?;
            let mut voies = BTreeSet::new();
            if o.commande == "regles" {
                let [chemin] = o.arretes.as_slice() else {
                    return Err("--arrete attendu (une seule fois)".into());
                };
                let verifie = lire_arrete(chemin, o.ancre.as_ref(), maintenant)?;
                voies = verifie.arrete().voies.iter().map(|v| v.id.clone()).collect();
                if let Some(v) = o.coupees.iter().find(|v| !voies.contains(*v)) {
                    return Err(format!("--couper : voie « {v} » absente de l'arrêté"));
                }
                instance.charger(verifie, maintenant).map_err(|e| e.to_string())?;
            } else if !o.coupees.is_empty() {
                return Err("--couper sans objet pour une fermeture".into());
            }
            let texte = instance.ruleset(maintenant, &o.coupees);
            let def = plan.frontiere(frontiere).ok_or("frontière hors plan")?;
            appliquer_ou_imprimer(&texte, &voies, o.appliquer, &def.netns, o.fichier)
        }
        "zone" => {
            let z = o.zone.ok_or("--zone attendu")?;
            let defz = plan.zone(z).ok_or("zone hors plan")?;
            if o.arretes.is_empty() {
                return Err("--arrete <frontière>=<fichier> attendu (au moins un)".into());
            }
            let mut voies = BTreeSet::new();
            let mut par_frontiere = Vec::new();
            for spec in &o.arretes {
                let (f, chemin) = spec.split_once('=').ok_or("--arrete : <frontière>=<fichier> attendu pour « zone »")?;
                let f: Frontiere = f.parse().map_err(|()| format!("frontière inconnue « {f} »"))?;
                let verifie = lire_arrete(chemin, o.ancre.as_ref(), maintenant)?;
                let a = verifie.arrete();
                if a.frontiere != f {
                    return Err(format!("{chemin} : arrêté de la frontière {}, pas {f}", a.frontiere));
                }
                if !verifie.en_vigueur(maintenant) {
                    return Err(format!("{chemin} : arrêté hors vigueur"));
                }
                if par_frontiere.iter().any(|(g, _)| *g == f) {
                    return Err(format!("frontière {f} donnée deux fois"));
                }
                let def = plan.frontiere(f).ok_or("frontière hors plan")?;
                if def.lien(z).is_none() {
                    return Err(format!("la frontière {f} ne touche pas la zone {z}"));
                }
                voies.extend(a.voies.iter().map(|v| v.id.clone()));
                let regles: Vec<_> = compiler(&verifie, &plan)
                    .map_err(|e| e.to_string())?
                    .into_iter()
                    .filter(|r| !o.coupees.contains(&r.voie))
                    .collect();
                par_frontiere.push((f, regles));
            }
            if let Some(v) = o.coupees.iter().find(|v| !voies.contains(*v)) {
                return Err(format!("--couper : voie « {v} » absente des arrêtés"));
            }
            let texte = nft::generer_zone(z, &par_frontiere, &plan).ok_or("génération de zone impossible")?;
            appliquer_ou_imprimer(&texte, &voies, o.appliquer, &defz.netns, o.fichier)
        }
        "hote" => commande_hote(o, &plan, maintenant),
        "tc" => commande_tc(o, &plan, maintenant),
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
