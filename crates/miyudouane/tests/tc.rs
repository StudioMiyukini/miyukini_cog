//! @id     miyudouane_tests_tc
//! @do     tester_mise_en_forme_tc
//! @role   test
//! @layer  operateur
//! @human  miyudouane — mise en forme tc : rendu golden déterministe, valeurs limites, plusieurs voies, audit, racines étrangères, filet nftables, CLI, syntaxe tc.

#![allow(clippy::assert_is_empty, clippy::many_single_char_names, clippy::format_collect)]

mod commun;

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

use borderguard::{sceller_arrete, Arrete, Constitution, Flux, Frontiere, Protocole, Zone};
use commun::*;
use miyudouane::tc::{
    auditer_tc, classer_racine, codel_us, filet_kio_s, generer_tc, planifier, rafale_octets, DescriptionTc, PlanTc,
    RacineTc, Sens, CAPACITE_DEFAUT_KIO_S,
};
use miyudouane::nft::MiseEnForme;
use miyudouane::{nft, ErreurDouane, InstanceDouane};

const DESCRIPTION: &str = "\
# Frontiere externe de la VM d'essai
miyudouane-tc 1
capacite wg-fed 12500
capacite ext0 125000
";

fn flux_debit(voie: &str, source: Zone, proto: Protocole, port: u16, pairs: &[&str], debit: u32, taille: u16) -> Flux {
    let mut f = flux(voie, source, Zone::Franche, proto, port, pairs);
    f.debit_max_kio_s = debit;
    f.taille_max_paquet = taille;
    f
}

/// Deux voies, trois flux, débits distincts (dont une entrée externe du
/// manifeste : Fédération → Franche).
fn arrete_essai() -> Arrete {
    arrete(
        Frontiere::Externe,
        7,
        vec![voie("jayoffice"), voie("site")],
        vec![
            flux_debit("site", Zone::International, Protocole::Tcp, 443, &["203.0.113.0/24"], 512, 1400),
            flux_debit("jayoffice", Zone::Federation, Protocole::Udp, 51820, &[], 2048, 1420),
            flux_debit("site", Zone::International, Protocole::Tcp, 80, &["203.0.113.0/24"], 256, 1400),
        ],
    )
}

fn description() -> DescriptionTc {
    DescriptionTc::analyser(DESCRIPTION).unwrap()
}

fn plan_tc(a: &Arrete, coupees: &[&str]) -> Result<PlanTc, String> {
    let coupees: BTreeSet<String> = coupees.iter().map(|s| (*s).to_owned()).collect();
    planifier(&verifie(a), &plan(), &description(), &coupees).map_err(|e| e.to_string())
}

/// Compare au fichier de référence ; `MIYUDOUANE_BENIR=1` le réécrit.
fn reference(nom: &str, texte: &str) {
    let chemin = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden").join(nom);
    if std::env::var_os("MIYUDOUANE_BENIR").is_some() {
        std::fs::write(&chemin, texte).unwrap();
    }
    let attendu = std::fs::read_to_string(&chemin).unwrap_or_default();
    assert_eq!(texte, attendu, "écart avec {}", chemin.display());
}

fn sans_empreinte(t: &str) -> String {
    t.lines()
        .map(|l| match l.find("empreinte=") {
            Some(i) => format!("{}empreinte=<sha256>", &l[..i]),
            None => l.to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn commandes(t: &str) -> Vec<&str> {
    t.lines().filter(|l| !l.is_empty() && !l.starts_with('#')).collect()
}

#[test]
fn reference_externe_tc() {
    let p = plan_tc(&arrete_essai(), &[]).unwrap();
    let t = generer_tc(&p);
    assert!(auditer_tc(&t, &p).is_empty(), "{:?}", auditer_tc(&t, &p));
    reference("externe.tc", &sans_empreinte(&t));
}

#[test]
fn rendu_deterministe_et_canonique() {
    let a = generer_tc(&plan_tc(&arrete_essai(), &[]).unwrap());
    let b = generer_tc(&plan_tc(&arrete_essai(), &[]).unwrap());
    assert_eq!(a, b);
    // Ordre des directives indifférent.
    let d = DescriptionTc::analyser("miyudouane-tc 1\ncapacite ext0 125000\ncapacite wg-fed 12500\n").unwrap();
    let p = planifier(&verifie(&arrete_essai()), &plan(), &d, &BTreeSet::new()).unwrap();
    assert_eq!(generer_tc(&p), a);
}

#[test]
fn une_classe_par_voie_une_feuille_par_flux_et_par_sens() {
    let p = plan_tc(&arrete_essai(), &[]).unwrap();
    let noms: Vec<&str> = p.interfaces.iter().map(|i| i.interface.as_str()).collect();
    // Toutes les interfaces de la frontière, triées.
    assert_eq!(noms, ["ext-franche", "ext0", "wg-fed"]);
    assert_eq!(p.netns, "mcog-f-externe");
    let i = |n: &str| p.interfaces.iter().find(|i| i.interface == n).unwrap();
    // Vers la Franche : sens original des trois flux, deux voies.
    let franche = i("ext-franche");
    assert_eq!(franche.capacite_kio_s, CAPACITE_DEFAUT_KIO_S);
    assert_eq!(franche.voies.len(), 2);
    assert_eq!((franche.voies[0].voie.as_str(), franche.voies[0].classe), ("jayoffice", 0x1000));
    assert_eq!((franche.voies[1].voie.as_str(), franche.voies[1].classe), ("site", 0x1001));
    assert_eq!(franche.voies[1].debit_kio_s(), 768);
    assert!(franche.voies.iter().flat_map(|v| &v.flux).all(|f| f.sens == Sens::Original));
    // Réponses : par l'interface d'entrée du flux.
    let ext0 = i("ext0");
    assert_eq!(ext0.voies.len(), 1);
    // Marques = rang canonique du flux dans l'arrêté scellé (+1).
    assert_eq!(ext0.voies[0].flux.iter().map(|f| (f.marque, f.sens)).collect::<Vec<_>>(), [
        (1, Sens::Reponse),
        (2, Sens::Reponse)
    ]);
    let wg = i("wg-fed");
    assert_eq!(wg.capacite_kio_s, 12_500);
    assert_eq!(wg.voies[0].flux[0].marque, 3);
    assert_eq!(wg.voies[0].flux[0].debit_kio_s, 2048);

    let t = generer_tc(&p);
    let c = commandes(&t);
    // 3 interfaces × (racine, classe racine, défaut, file défaut) + voies + feuilles.
    assert_eq!(c.iter().filter(|l| l.contains(" root handle 4d44: htb default ffff")).count(), 3);
    assert!(t.contains("class add dev ext-franche parent 4d44:f000 classid 4d44:1001 htb rate 768KiBps ceil 768KiBps"));
    assert!(t.contains("class add dev ext-franche parent 4d44:1001 classid 4d44:1 htb rate 256KiBps ceil 256KiBps"));
    assert!(t.contains("filter add dev wg-fed parent 4d44: protocol all prio 10 handle 0x3 fw classid 4d44:3"));
    assert!(t.contains("qdisc add dev ext0 parent 4d44:1 handle 1: fq_codel"));
    // Classe par défaut : capacité − garanti.
    assert!(t.contains("class add dev wg-fed parent 4d44:f000 classid 4d44:ffff htb rate 10452KiBps ceil 12500KiBps"));
    // Classification par marque seulement : aucune adresse, aucun port.
    assert!(c.iter().all(|l| !l.contains("203.0.113") && !l.contains("u32") && !l.contains("match")));
    // Jamais de suppression, de remplacement ni d'action.
    assert!(c.iter().all(|l| l.starts_with("qdisc add dev ") || l.starts_with("class add dev ") || l.starts_with("filter add dev ")));
}

#[test]
fn valeurs_limites_de_debit() {
    // Débit minimal (1 Kio/s) : accepté, file CoDel adaptée au temps d'émission.
    let a = arrete(
        Frontiere::Externe,
        1,
        vec![voie("jayoffice")],
        vec![flux_debit("jayoffice", Zone::Federation, Protocole::Udp, 51820, &[], 1, 80)],
    );
    let t = generer_tc(&plan_tc(&a, &[]).unwrap());
    assert!(t.contains("classid 4d44:1 htb rate 1KiBps ceil 1KiBps burst 1611 cburst 1611 quantum 1600"));
    let (cible, intervalle) = codel_us(1, 80);
    assert_eq!((cible, intervalle), (117_188, 212_188));
    assert!(t.contains(&format!("target {cible}us interval {intervalle}us")));
    assert_eq!(codel_us(512, 1400), (5_000, 100_000));

    // Débit nul : refusé dès le scellement de l'arrêté (et par le manifeste).
    let mut nul = a.clone();
    nul.flux[0].debit_max_kio_s = 0;
    assert!(sceller_arrete(&nul, &cle(), &Constitution::reference()).is_err());

    // Maximum constitutionnel (100 Mio/s) : accepté ; au-delà : refusé.
    let mut max = a.clone();
    max.flux[0].debit_max_kio_s = Constitution::reference().plafond_debit_kio_s;
    let defaut = DescriptionTc::analyser("miyudouane-tc 1\n").unwrap();
    let t = generer_tc(&planifier(&verifie(&max), &plan(), &defaut, &BTreeSet::new()).unwrap());
    assert!(t.contains("classid 4d44:1 htb rate 102400KiBps ceil 102400KiBps burst 1050176 cburst 1050176"));
    let mut trop = max.clone();
    trop.flux[0].debit_max_kio_s += 1;
    assert!(sceller_arrete(&trop, &cle(), &Constitution::reference()).is_err());

    // Somme des débits ≥ capacité de l'interface : refus explicite.
    let d = DescriptionTc::analyser("miyudouane-tc 1\ncapacite wg-fed 102400\n").unwrap();
    let e = planifier(&verifie(&max), &plan(), &d, &BTreeSet::new()).unwrap_err();
    assert!(e.0.contains("wg-fed") && e.0.contains("capacité"), "{e}");
    let d = DescriptionTc::analyser("miyudouane-tc 1\ncapacite wg-fed 102401\n").unwrap();
    let p = planifier(&verifie(&max), &plan(), &d, &BTreeSet::new()).unwrap();
    assert!(generer_tc(&p).contains("class add dev wg-fed parent 4d44:f000 classid 4d44:ffff htb rate 1KiBps ceil 102401KiBps"));
}

#[test]
fn filet_nftables_coherent_avec_la_mise_en_forme() {
    assert_eq!(filet_kio_s(512), 640);
    assert_eq!(filet_kio_s(1), 2);
    assert_eq!(filet_kio_s(102_400), 128_000);
    assert_eq!(rafale_octets(512), 6843);
    let a = arrete_essai();
    let mut i = InstanceDouane::nouvelle(Frontiere::Externe, plan(), 0).unwrap();
    i.charger(verifie(&a), T0 + 1).unwrap();
    let r = i.ruleset_selon(T0 + 1, &BTreeSet::new(), MiseEnForme::Appliquee).unwrap();
    // Filet par sens, au-delà de 125 %, même rafale que la feuille HTB.
    for sens in ["original", "reply"] {
        assert!(r.contains(&format!(
            "ct mark 0x00000003 ct direction {sens} limit rate over 2560 kbytes/second burst {} bytes counter name \"rejets\" drop",
            rafale_octets(2048)
        )));
    }
    let t = generer_tc(&plan_tc(&a, &[]).unwrap());
    assert!(t.contains(&format!("rate 2048KiBps ceil 2048KiBps burst {b} cburst {b}", b = rafale_octets(2048))));
    // Marque du paquet posée pour le classifieur fw (nouvelles et établies).
    assert!(r.contains(nft::COPIE_MARQUE));
    assert!(r.contains("ct mark set 0x00000003 meta mark set 0x00000003 counter accept"));
    let voies: BTreeSet<String> = a.voies.iter().map(|v| v.id.clone()).collect();
    assert!(nft::auditer_selon(&r, &voies, MiseEnForme::Appliquee).is_empty());
    assert!(r.contains(&format!("# {}", nft::INFO_FILET)));
    // Le même texte, sans mise en forme déclarée : écart (filet non admis).
    assert!(!nft::auditer(&r, &voies).is_empty());
}

/// Sans mise en forme déclarée (défaut), la frontière externe garde la règle
/// stricte d'avant A-023 : plafond `D`, sans rafale, sans `ct direction`.
#[test]
fn externe_sans_mise_en_forme_regle_stricte() {
    let a = arrete_essai();
    let mut i = InstanceDouane::nouvelle(Frontiere::Externe, plan(), 0).unwrap();
    i.charger(verifie(&a), T0 + 1).unwrap();
    let r = i.ruleset(T0 + 1, &BTreeSet::new());
    assert_eq!(r, i.ruleset_selon(T0 + 1, &BTreeSet::new(), MiseEnForme::Absente).unwrap());
    for (m, d) in [(1, 256), (2, 512), (3, 2048)] {
        assert!(r.contains(&format!(
            "ct mark 0x{m:08x} limit rate over {d} kbytes/second counter name \"rejets\" drop"
        )));
    }
    assert!(!r.contains("ct direction"));
    assert!(!r.contains(" bytes "), "aucune rafale en octets");
    assert!(!r.contains(nft::INFO_FILET));
    assert!(!nft::porte_filet(&r));
    // Copie de marque conservée en externe (sans effet de sécurité) : `tc`
    // classe dès l'étape (b), sous la règle stricte.
    assert!(r.contains(nft::COPIE_MARQUE));
    let voies: BTreeSet<String> = a.voies.iter().map(|v| v.id.clone()).collect();
    assert!(nft::auditer(&r, &voies).is_empty());
    // Forme stricte présentée comme « mise en forme appliquée » : écart.
    let e = nft::auditer_selon(&r, &voies, MiseEnForme::Appliquee);
    assert!(e.iter().any(|e| e.contains("stricte")), "{e:?}");
}

/// Filet demandé hors frontière externe : refusé (aucun texte).
#[test]
fn filet_refuse_hors_externe() {
    for a in [arrete_portail(1), arrete_local(1)] {
        let mut i = InstanceDouane::nouvelle(a.frontiere, plan(), 0).unwrap();
        i.charger(verifie(&a), T0 + 1).unwrap();
        assert_eq!(
            i.ruleset_selon(T0 + 1, &BTreeSet::new(), MiseEnForme::Appliquee),
            Err(ErreurDouane::MiseEnFormeHorsExterne(a.frontiere))
        );
        let regles = miyudouane::compiler(&verifie(&a), &plan()).unwrap();
        assert!(nft::generer_frontiere(&verifie(&a), &regles, &plan(), &BTreeSet::new(), MiseEnForme::Appliquee).is_none());
        assert!(nft::generer_frontiere(&verifie(&a), &regles, &plan(), &BTreeSet::new(), MiseEnForme::Absente).is_some());
    }
    assert!(!MiseEnForme::Appliquee.admise(Frontiere::PortailInterne));
    assert!(MiseEnForme::Appliquee.admise(Frontiere::Externe));
    assert!(MiseEnForme::Absente.admise(Frontiere::PortailLocal));
    assert_eq!(MiseEnForme::default(), MiseEnForme::Absente);
}

/// Audit croisé : chaque forme n'est admise que sous l'état déclaré, et un
/// filet altéré est refusé même sous mise en forme déclarée.
#[test]
fn audit_croise_forme_et_etat_declare() {
    let a = arrete_essai();
    let voies: BTreeSet<String> = a.voies.iter().map(|v| v.id.clone()).collect();
    let mut i = InstanceDouane::nouvelle(Frontiere::Externe, plan(), 0).unwrap();
    i.charger(verifie(&a), T0 + 1).unwrap();
    let strict = i.ruleset_selon(T0 + 1, &BTreeSet::new(), MiseEnForme::Absente).unwrap();
    let filet = i.ruleset_selon(T0 + 1, &BTreeSet::new(), MiseEnForme::Appliquee).unwrap();
    assert!(nft::porte_filet(&filet));

    assert!(nft::auditer_selon(&strict, &voies, MiseEnForme::Absente).is_empty());
    assert!(nft::auditer_selon(&filet, &voies, MiseEnForme::Appliquee).is_empty());
    let e = nft::auditer_selon(&filet, &voies, MiseEnForme::Absente);
    assert!(e.iter().any(|e| e.contains("sans mise en forme tc déclarée")), "{e:?}");
    assert!(!nft::auditer_selon(&strict, &voies, MiseEnForme::Appliquee).is_empty());

    // Une seule ligne de filet glissée dans un texte strict : écart.
    let glisse = strict.replacen(
        "ct mark 0x00000003 limit rate over 2048 kbytes/second",
        "ct mark 0x00000003 ct direction original limit rate over 2560 kbytes/second burst 22572 bytes",
        1,
    );
    assert_ne!(glisse, strict);
    assert!(!nft::auditer(&glisse, &voies).is_empty());
    // Rafale ajoutée à la règle stricte : forme inconnue.
    let rafale = strict.replacen("2048 kbytes/second", "2048 kbytes/second burst 9999 bytes", 1);
    assert!(!nft::auditer(&rafale, &voies).is_empty());
    // Filet relevé (ne dérive plus du débit déclaré via la rafale) : écart.
    let releve = filet.replace("over 2560 kbytes/second", "over 4096 kbytes/second");
    assert!(nft::auditer_selon(&releve, &voies, MiseEnForme::Appliquee).iter().any(|e| e.contains("même débit")));
    // Un sens manquant : écart.
    let un_sens: String = filet.lines().filter(|l| !l.contains("ct direction reply")).map(|l| format!("{l}\n")).collect();
    assert!(nft::auditer_selon(&un_sens, &voies, MiseEnForme::Appliquee).iter().any(|e| e.contains("un sens")));
    // En-tête d'une autre frontière : écart.
    let ailleurs = filet.replace("# frontiere=externe ", "# frontiere=portail-interne ");
    assert!(!nft::auditer_selon(&ailleurs, &voies, MiseEnForme::Appliquee).is_empty());
    // Sans déclaration de mise en forme dans l'en-tête : écart.
    let sans_info = filet.replace(&format!("# {}\n", nft::INFO_FILET), "");
    assert!(!nft::auditer_selon(&sans_info, &voies, MiseEnForme::Appliquee).is_empty());
    // Fermeture : aucune règle de débit, admise sous les deux états.
    let ferme = InstanceDouane::nouvelle(Frontiere::Externe, plan(), 0).unwrap();
    let f = ferme.ruleset_selon(T0, &BTreeSet::new(), MiseEnForme::Appliquee).unwrap();
    assert!(f.contains("|FERME|"));
    assert!(nft::auditer_selon(&f, &BTreeSet::new(), MiseEnForme::Appliquee).is_empty());
    assert!(nft::auditer(&f, &BTreeSet::new()).is_empty());
}

#[test]
fn reference_externe_mise_en_forme() {
    let mut i = InstanceDouane::nouvelle(Frontiere::Externe, plan(), 0).unwrap();
    i.charger(verifie(&arrete_externe(1)), T0 + 1).unwrap();
    let r = i.ruleset_selon(T0 + 1, &BTreeSet::new(), MiseEnForme::Appliquee).unwrap();
    reference("externe-mise-en-forme.nft", &sans_empreinte(&r));
}

#[test]
fn coupure_de_voie() {
    let p = plan_tc(&arrete_essai(), &["site"]).unwrap();
    let t = generer_tc(&p);
    assert!(!t.contains("voie=site"));
    assert!(t.contains("voie=jayoffice"));
    assert!(t.contains("# voies coupees : site"));
    // Les trois interfaces restent sous la Douane (une voie coupée disparaît).
    assert_eq!(commandes(&t).iter().filter(|l| l.contains(" root handle 4d44:")).count(), 3);
    assert!(auditer_tc(&t, &p).is_empty());
}

#[test]
fn audit_refuse_tout_ecart() {
    let p = plan_tc(&arrete_essai(), &[]).unwrap();
    let t = generer_tc(&p);
    assert!(auditer_tc(&t, &p).is_empty());

    // Classe de flux modifiée (plafond ×10).
    let modifie =
        t.replacen("classid 4d44:1 htb rate 256KiBps ceil 256KiBps", "classid 4d44:1 htb rate 2560KiBps ceil 2560KiBps", 1);
    assert_ne!(modifie, t);
    let e = auditer_tc(&modifie, &p);
    assert!(e.iter().any(|x| x.contains("feuille 4d44:1") && x.contains("256KiBps")), "{e:?}");
    assert!(e.iter().any(|x| x.contains("écart au rendu attendu")), "{e:?}");

    // Plafond seul relevé (ceil > rate : emprunt possible).
    let e = auditer_tc(&t.replacen("rate 256KiBps ceil 256KiBps", "rate 256KiBps ceil 1024KiBps", 1), &p);
    assert!(e.iter().any(|x| x.contains("feuille 4d44:1")), "{e:?}");

    // Filtre détourné vers une autre feuille.
    let e = auditer_tc(&t.replacen("handle 0x3 fw classid 4d44:3", "handle 0x3 fw classid 4d44:1", 1), &p);
    assert!(e.iter().any(|x| x.contains("filtre")), "{e:?}");

    // Classe ajoutée (même conforme en apparence) : écart au rendu.
    let ajout = format!("{t}class add dev ext0 parent 4d44:f000 classid 4d44:1005 htb rate 1KiBps ceil 1KiBps\n");
    assert!(!auditer_tc(&ajout, &p).is_empty());

    // Suppression, remplacement, interface étrangère, racine étrangère, action.
    for ligne in [
        "qdisc del dev ext0 root",
        "qdisc replace dev ext0 root handle 4d44: htb default ffff",
        "qdisc add dev eth0 root handle 4d44: htb default ffff",
        "qdisc add dev ext0 root handle 1: htb default ffff",
        "filter add dev ext0 parent 4d44: protocol all prio 1 matchall action drop",
        "qdisc add dev ext0 ingress",
    ] {
        let e = auditer_tc(&format!("{t}{ligne}\n"), &p);
        assert!(e.len() >= 2, "{ligne} : {e:?}");
    }
    // Feuille retirée : absente.
    let sans: String = t.lines().filter(|l| !l.contains("handle 0x3 fw")).map(|l| format!("{l}\n")).collect();
    assert!(auditer_tc(&sans, &p).iter().any(|x| x.contains("absent")));
}

#[test]
fn racines_etrangeres_jamais_touchees() {
    // Racines du noyau (poignée 0:) : remplaçables.
    for s in [
        "qdisc noqueue 0: root refcnt 2\n",
        "qdisc fq_codel 0: root refcnt 2 limit 10240p flows 1024 quantum 1514 target 5ms interval 100ms\n",
        "qdisc mq 0: root\n",
        "",
    ] {
        assert_eq!(classer_racine(s), RacineTc::Defaut, "{s}");
    }
    assert_eq!(
        classer_racine("qdisc htb 4d44: root refcnt 2 r2q 10 default 0xffff direct_packets_stat 0 direct_qlen 1000\n"),
        RacineTc::Douane
    );
    // Toute racine explicite d'un autre : étrangère (l'application refuse
    // avant toute modification).
    for (s, attendu) in [
        ("qdisc htb 1: root refcnt 2 r2q 10 default 0x30\n", "htb 1:"),
        ("qdisc cake 8001: root refcnt 2 bandwidth 100Mbit\n", "cake 8001:"),
        ("qdisc tbf 4d44: root refcnt 2 rate 1Mbit\n", "tbf 4d44:"),
    ] {
        assert_eq!(classer_racine(s), RacineTc::Etrangere(attendu.into()), "{s}");
    }
    // Le texte ne touche que les interfaces de la frontière et sa propre racine.
    let p = plan_tc(&arrete_essai(), &[]).unwrap();
    let interfaces: BTreeSet<&str> = p.interfaces.iter().map(|i| i.interface.as_str()).collect();
    for l in commandes(&generer_tc(&p)) {
        let mots: Vec<&str> = l.split_whitespace().collect();
        assert!(interfaces.contains(mots[3]), "{l}");
        assert!(!l.contains(" del ") && !l.contains(" replace ") && !l.contains("ingress"), "{l}");
        assert!(l.contains("4d44:"), "{l}");
    }
}

#[test]
fn description_stricte() {
    let refus = |t: &str| DescriptionTc::analyser(t).unwrap_err().to_string();
    assert!(refus("").contains("en-tête absent"));
    assert!(refus("miyudouane-tc 2\n").contains("en-tête"));
    assert!(refus("miyudouane-tc 1\ncapacite wg-fed 0\n").contains("hors bornes"));
    assert!(refus("miyudouane-tc 1\ncapacite wg-fed 4000001\n").contains("hors bornes"));
    assert!(refus("miyudouane-tc 1\ncapacite wg-fed 012\n").contains("hors bornes"));
    assert!(refus("miyudouane-tc 1\ncapacite wg-fed 10\ncapacite wg-fed 20\n").contains("double"));
    assert!(refus("miyudouane-tc 1\ncapacite WG 10\n").contains("interface invalide"));
    assert!(refus("miyudouane-tc 1\ncapacite une-interface-trop-longue 10\n").contains("interface invalide"));
    assert!(refus("miyudouane-tc 1\ndebit site 10\n").contains("directive inconnue"));
    assert!(refus("miyudouane-tc 1\ncapacite wg-fed 10 # x\n").contains("mal formée"));
    assert!(refus(&"#".repeat(5000)).contains("taille"));
    let d = DescriptionTc::analyser("miyudouane-tc 1\n").unwrap();
    assert!(d.capacites.is_empty());
    // Interface inconnue de la frontière : refus à la planification.
    let d = DescriptionTc::analyser("miyudouane-tc 1\ncapacite eth0 1000\n").unwrap();
    let e = planifier(&verifie(&arrete_essai()), &plan(), &d, &BTreeSet::new()).unwrap_err();
    assert!(e.0.contains("eth0"), "{e}");
}

#[test]
fn frontiere_externe_seule() {
    let e = planifier(&verifie(&arrete_portail(1)), &plan(), &description(), &BTreeSet::new()).unwrap_err();
    assert!(e.0.contains("frontière externe"), "{e}");
}

struct Banc {
    dossier: PathBuf,
}

impl Banc {
    fn nouveau(nom: &str) -> Self {
        let dossier = std::env::temp_dir().join(format!("miyudouane-tc-{nom}-{}", std::process::id()));
        std::fs::create_dir_all(&dossier).unwrap();
        Self { dossier }
    }

    fn ecrire(&self, nom: &str, octets: &[u8]) -> String {
        let chemin = self.dossier.join(nom);
        std::fs::write(&chemin, octets).unwrap();
        chemin.display().to_string()
    }
}

impl Drop for Banc {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dossier);
    }
}

fn ancre_hex() -> String {
    cle().cle_publique().iter().map(|b| format!("{b:02x}")).collect()
}

fn lancer(args: &[&str]) -> (bool, String, String) {
    let mut c = Command::new(env!("CARGO_BIN_EXE_miyudouane"));
    c.args(args).args(["--maintenant", &(T0 + 1).to_string()]);
    for s in ["cog:jayoffice", "franche:jayoffice", "franche:site", "cog:alicia", "locale:alicia", "cog:site"] {
        c.args(["--sandbox", s]);
    }
    let o = c.output().unwrap();
    (o.status.success(), String::from_utf8_lossy(&o.stdout).into_owned(), String::from_utf8_lossy(&o.stderr).into_owned())
}

#[test]
fn cli_tc_a_blanc() {
    let b = Banc::nouveau("cli");
    let desc = b.ecrire("tc.txt", DESCRIPTION.as_bytes());
    let externe = b.ecrire("externe.arrete", &sceller_arrete(&arrete_essai(), &cle(), &Constitution::reference()).unwrap());
    let portail = b.ecrire("portail.arrete", &sceller_arrete(&arrete_portail(1), &cle(), &Constitution::reference()).unwrap());
    let ancre = ancre_hex();

    let (ok, sortie, err) = lancer(&["tc", "--description", &desc, "--arrete", &externe, "--ancre", &ancre]);
    assert!(ok, "{err}");
    assert_eq!(sortie, generer_tc(&plan_tc(&arrete_essai(), &[]).unwrap()));

    let (ok, sortie, err) =
        lancer(&["tc", "--description", &desc, "--arrete", &externe, "--ancre", &ancre, "--couper", "site"]);
    assert!(ok, "{err}");
    assert!(!sortie.contains("voie=site"));

    for (args, motif) in [
        (vec!["tc", "--arrete", externe.as_str(), "--ancre", ancre.as_str()], "--description"),
        (vec!["tc", "--description", desc.as_str(), "--ancre", ancre.as_str()], "--arrete"),
        (vec!["tc", "--description", desc.as_str(), "--arrete", externe.as_str()], "--ancre"),
        (vec!["tc", "--description", desc.as_str(), "--arrete", portail.as_str(), "--ancre", ancre.as_str()], "externe"),
        (
            vec!["tc", "--description", desc.as_str(), "--arrete", externe.as_str(), "--ancre", ancre.as_str(), "--couper", "x"],
            "--couper",
        ),
    ] {
        let (ok, _, err) = lancer(&args);
        assert!(!ok && err.contains(motif), "{args:?} : {err}");
    }
    // `regles` : stricte par défaut ; filet seulement sur déclaration, en externe.
    let base = ["regles", "--frontiere", "externe", "--arrete", externe.as_str(), "--ancre", ancre.as_str()];
    let (ok, strict, err) = lancer(&base);
    assert!(ok, "{err}");
    assert!(strict.contains("ct mark 0x00000003 limit rate over 2048 kbytes/second") && !nft::porte_filet(&strict));
    let (ok, filet, err) = lancer(&[&base[..], &["--mise-en-forme-appliquee"]].concat());
    assert!(ok, "{err}");
    assert!(nft::porte_filet(&filet) && !filet.contains("limit rate over 2048 kbytes/second"));
    let (ok, _, err) = lancer(&[
        "regles",
        "--frontiere",
        "portail-interne",
        "--arrete",
        &portail,
        "--ancre",
        &ancre,
        "--mise-en-forme-appliquee",
    ]);
    assert!(!ok && err.contains("externe seulement"), "{err}");
    let (ok, _, err) = lancer(&["fermeture", "--frontiere", "externe", "--mise-en-forme-appliquee"]);
    assert!(!ok && err.contains("sans objet"), "{err}");
    let (ok, _, err) =
        lancer(&["tc", "--description", &desc, "--arrete", &externe, "--ancre", &ancre, "--mise-en-forme-appliquee"]);
    assert!(!ok && err.contains("sans objet"), "{err}");

    // Ancre étrangère : arrêté refusé (fail-closed).
    let autre = "11".repeat(32);
    let (ok, _, _) = lancer(&["tc", "--description", &desc, "--arrete", &externe, "--ancre", &autre]);
    assert!(!ok);
}

/// Contrôle de syntaxe par `tc` lui-même, **sans rien appliquer** : chaque
/// interface est renommée en une interface inexistante, `tc -force -batch`
/// analyse chaque commande (options HTB / fq_codel comprises) puis échoue sur
/// « Cannot find device » avant tout envoi au noyau. Aucune règle n'est posée,
/// ni sur l'hôte ni ailleurs. Ignoré (avec raison) si `tc` est absent ou si le
/// nom de substitution existe.
#[test]
fn tc_batch_syntaxe_sans_application() {
    const ABSENTE: &str = "mdou-absent0";
    if Command::new("tc").arg("-V").output().is_err() {
        eprintln!("ignoré : tc absent");
        return;
    }
    if std::path::Path::new("/sys/class/net").join(ABSENTE).exists() {
        eprintln!("ignoré : l'interface {ABSENTE} existe (le contrôle enverrait des commandes au noyau)");
        return;
    }
    let p = plan_tc(&arrete_essai(), &[]).unwrap();
    let mut t = generer_tc(&p);
    for i in &p.interfaces {
        t = t.replace(&format!(" dev {} ", i.interface), &format!(" dev {ABSENTE} "));
    }
    let b = Banc::nouveau("syntaxe");
    let chemin = b.ecrire("essai.tc", t.as_bytes());
    let o = Command::new("tc").args(["-force", "-batch", &chemin]).output().unwrap();
    let erreurs = String::from_utf8_lossy(&o.stderr).into_owned();
    let n = commandes(&t).len();
    let introuvable = erreurs.lines().filter(|l| *l == format!("Cannot find device \"{ABSENTE}\"")).count();
    let autres: Vec<&str> = erreurs
        .lines()
        .filter(|l| !l.starts_with("Command failed ") && *l != format!("Cannot find device \"{ABSENTE}\""))
        .collect();
    assert!(autres.is_empty(), "erreurs de syntaxe tc :\n{erreurs}\n{t}");
    assert_eq!(introuvable, n, "{erreurs}");
}
