//! @id     miyudouane_tests_hote
//! @do     tester_table_hote_zone_cog
//! @role   test
//! @layer  operateur
//! @human  miyudouane — table hôte : rendu déterministe, validation de la description, absence d'injection, exceptions par arrêté signé, CLI.

mod commun;

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

use borderguard::{sceller_arrete, Constitution, Frontiere, Protocole, Zone};
use commun::*;
use miyudouane::hote::{
    auditer_hote, exceptions_autorisees, generer_hote, Compte, DescriptionHote, ExceptionSortie, TABLE_HOTE,
};

/// Zone COG de la VM d'essai (`deploy/cog-vm`) : comptes, ports d'écoute et
/// flux locaux réels des services.
const DESCRIPTION: &str = "\
# Zone COG de la VM d'essai (deploy/cog-vm)
miyudouane-hote 2
groupe 110
compte miyukini-kindmother
compte miyukini-maia
compte miyukini-miyucloud
compte miyukini-portail
port tcp 50051
port tcp 11435
port tcp 8086
port tcp 8090
port tcp 5432
# MAIA -> KindMother (gRPC), dont l'unite d'attente lancee sous le compte de MAIA.
boucle miyukini-maia tcp 127.0.0.1 50051
# MiyuCloud -> PostgreSQL (compte postgres, hors zone COG).
boucle miyukini-miyucloud tcp 127.0.0.1 5432
# KindMother, portail web : aucun flux local sortant.
exception site
";

/// Variante d'essai : compte par UID, port UDP, flux IPv6 (::1) déclaré.
const DESCRIPTION_ESSAI: &str = "\
miyudouane-hote 2
compte miyukini-maia
compte miyukini-kindmother
uid 998
port tcp 50051
port udp 5353
boucle miyukini-maia tcp 127.0.0.1 50051
boucle miyukini-maia tcp ::1 50051
boucle 998 udp ::1 5353
";

fn description() -> DescriptionHote {
    DescriptionHote::analyser(DESCRIPTION).unwrap()
}

fn description_essai() -> DescriptionHote {
    DescriptionHote::analyser(DESCRIPTION_ESSAI).unwrap()
}

fn voies(d: &DescriptionHote) -> BTreeSet<String> {
    d.exceptions.clone()
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

fn rendu_reference() -> String {
    let d = description();
    let arretes = [verifie(&arrete_portail(1)), verifie(&arrete_local(1))];
    let ex = exceptions_autorisees(&d, &arretes, &plan(), &BTreeSet::new(), T0 + 1).unwrap();
    generer_hote(&d, &ex)
}

#[test]
fn reference_hote_deterministe() {
    let t = rendu_reference();
    reference("hote.nft", &sans_empreinte(&t));
    assert_eq!(t, rendu_reference(), "deux rendus identiques");
    assert_eq!(auditer_hote(&t, &voies(&description())), Vec::<String>::new());

    // Ordre des directives indifférent : rendu canonique.
    let mut lignes: Vec<&str> = DESCRIPTION.lines().skip(2).collect();
    lignes.reverse();
    let permutee = format!("miyudouane-hote 2\n{}\n", lignes.join("\n"));
    assert_eq!(DescriptionHote::analyser(&permutee).unwrap(), description());
}

#[test]
fn regles_attendues() {
    let t = rendu_reference();
    // Remplacement de la seule table, jamais de flush.
    assert!(t.contains(&format!("add table inet {TABLE_HOTE}\ndelete table inet {TABLE_HOTE}\n")));
    assert!(!t.contains("flush"));
    assert_eq!(t.matches("table inet").count(), 3);
    // Comptes : noms triés et quotés, puis UID.
    assert!(t.contains(
        "elements = { \"miyukini-kindmother\", \"miyukini-maia\", \"miyukini-miyucloud\", \"miyukini-portail\" }"
    ));
    // Sortie : boucle locale des autres comptes, connexions établies et flux
    // de boucle par compte, exception signée, puis rejet journalisé.
    assert!(!t.contains("\t\toifname \"lo\" accept\n"), "aucune boucle locale non qualifiée");
    assert!(t.contains(
        "\t\tmeta skuid != @comptes_cog oifname \"lo\" accept\n\
         \t\tmeta skuid @comptes_cog oifname \"lo\" ct state established,related accept\n\
         \t\tmeta skuid \"miyukini-maia\" oifname \"lo\" ip daddr 127.0.0.1 tcp dport 50051 accept\n\
         \t\tmeta skuid \"miyukini-miyucloud\" oifname \"lo\" ip daddr 127.0.0.1 tcp dport 5432 accept\n\
         \t\tmeta skuid @comptes_cog ip daddr 10.77.0.253"
    ));
    assert_eq!(t.matches("oifname \"lo\"").count(), 4, "aucun autre flux de boucle que ceux déclarés");
    assert!(t.contains(
        "meta skuid @comptes_cog ip daddr 10.77.0.253 tcp dport 7080 counter accept comment \"voie=site service=site flux=0\""
    ));
    assert!(!t.contains("voie=jayoffice"), "voie non déclarée en exception");
    assert!(!t.contains("voie=alicia"));
    assert!(t.contains(
        "meta skuid @comptes_cog update @journal_v4 { ip saddr . ip daddr . meta l4proto limit rate 1/second } \
         limit name \"journal\" log prefix \"MDOU|zone-cog|sortie|SORTIE-COG|\" group 110\n"
    ));
    assert!(t.contains(
        "meta skuid @comptes_cog update @journal_v6 { ip6 saddr . ip6 daddr . meta l4proto limit rate 1/second } \
         limit name \"journal\" log prefix \"MDOU|zone-cog|sortie|SORTIE-COG|\" group 110\n"
    ));
    assert!(t.contains("meta skuid @comptes_cog counter name \"rejets\" drop\n\t}"));
    // Entrée : ports COG, IPv4 et IPv6 (th dport, famille inet).
    assert!(t.contains(
        "meta l4proto tcp th dport { 5432, 8086, 8090, 11435, 50051 } update @journal_v4 { ip saddr . ip daddr . \
         meta l4proto limit rate 1/second } limit name \"journal\" log prefix \"MDOU|zone-cog|entree|NON-DECLARE|\" group 110\n"
    ));
    assert!(t.contains("meta l4proto tcp th dport { 5432, 8086, 8090, 11435, 50051 } counter name \"rejets\" drop\n"));
    assert!(!t.contains("meta l4proto udp"), "aucun port UDP déclaré");
    // Variante : UID, UDP, IPv6.
    let e = generer_hote(&description_essai(), &[]);
    assert!(e.contains("elements = { \"miyukini-kindmother\", \"miyukini-maia\", 998 }"));
    assert!(e.contains(
        "\t\tmeta skuid \"miyukini-maia\" oifname \"lo\" ip daddr 127.0.0.1 tcp dport 50051 accept\n\
         \t\tmeta skuid \"miyukini-maia\" oifname \"lo\" ip6 daddr ::1 tcp dport 50051 accept\n\
         \t\tmeta skuid 998 oifname \"lo\" ip6 daddr ::1 udp dport 5353 accept\n"
    ));
    assert!(e.contains("meta l4proto udp th dport { 5353 } counter name \"rejets\" drop\n"));
    assert_eq!(auditer_hote(&e, &BTreeSet::new()), Vec::<String>::new());
    // Journalisation bornée : le drop ne dépend d'aucune limite.
    for l in t.lines() {
        if l.contains(" log ") {
            assert!(l.contains("update @journal_v") && l.contains("limit name \"journal\"") && !l.contains("drop"), "{l}");
        }
        if l.ends_with(" drop") {
            assert!(!l.contains("limit") && !l.contains(" log "), "{l}");
        }
    }
    assert!(t.contains("\tlimit journal {\n\t\trate 50/second burst 100 packets\n\t}\n\tcounter rejets {\n\t}\n"));
    assert!(!t.contains("policy drop"));
}

#[test]
fn sans_exception_par_defaut() {
    let d = DescriptionHote::analyser("miyudouane-hote 2\nuid 998\n").unwrap();
    assert_eq!(d.groupe, 110);
    assert!(d.exceptions.is_empty());
    // Arrêtés fournis mais aucune exception déclarée : rien n'est ouvert.
    let ex = exceptions_autorisees(&d, &[verifie(&arrete_portail(1))], &plan(), &BTreeSet::new(), T0 + 1).unwrap();
    assert_eq!(ex, vec![]);
    let t = generer_hote(&d, &ex);
    assert!(!t.contains("counter accept"));
    assert!(!t.contains("chain entree"), "aucun port : pas de chaîne d'entrée");
    assert_eq!(auditer_hote(&t, &BTreeSet::new()), Vec::<String>::new());
}

#[test]
fn groupe_configurable() {
    let d = DescriptionHote::analyser("miyudouane-hote 2\ngroupe 4242\nuid 998\nport tcp 1\n").unwrap();
    let t = generer_hote(&d, &[]);
    assert!(t.contains("\"MDOU|zone-cog|sortie|SORTIE-COG|\" group 4242\n"));
    assert!(t.contains("\"MDOU|zone-cog|entree|NON-DECLARE|\" group 4242\n"));
}

fn refus(texte: &str) -> (usize, String) {
    let e = DescriptionHote::analyser(texte).unwrap_err();
    (e.ligne, e.motif)
}

#[test]
fn validation_stricte() {
    let h = "miyudouane-hote 2\n";
    // En-tête.
    assert!(refus("uid 998\n").1.contains("en-tête"));
    assert!(refus("miyudouane-hote 3\nuid 998\n").1.contains("en-tête"));
    // Format 1 (boucle locale ouverte d'office) : refusé, jamais réinterprété.
    let (ligne, motif) = refus("miyudouane-hote 1\nuid 998\nport tcp 50051\n");
    assert_eq!(ligne, 1);
    assert!(motif.contains("format 1 périmé") && motif.contains("boucle"), "{motif}");
    assert!(refus("").1.contains("en-tête"));
    // Au moins un compte.
    assert!(refus(h).1.contains("au moins un compte"));
    assert!(refus(&format!("{h}port tcp 80\n")).1.contains("au moins un compte"));
    // Noms invalides.
    for nom in ["Root", "root", "1abc", "a.b", "-a", "a b", "é", "abcdefghijabcdefghijabcdefghijabc", "a\"b", "a}b", "a;b", "a$"] {
        assert!(DescriptionHote::analyser(&format!("{h}compte {nom}\n")).is_err(), "nom accepté : {nom:?}");
    }
    assert!(DescriptionHote::analyser(&format!("{h}compte _svc-cog_1\n")).is_ok());
    // UID hors bornes ou mal écrits.
    for u in ["0", "65534", "65535", "4294967295", "4294967296", "-1", "+5", "0998", "9a", "99999999999"] {
        let (ligne, motif) = refus(&format!("{h}uid {u}\n"));
        assert_eq!(ligne, 2);
        assert!(motif.contains("UID hors bornes"), "{u} : {motif}");
    }
    assert!(DescriptionHote::analyser(&format!("{h}uid 1\nuid 65533\n")).is_ok());
    // Ports.
    for p in ["0", "65536", "-1", "080", "x"] {
        assert!(refus(&format!("{h}uid 9\nport tcp {p}\n")).1.contains("port hors bornes"), "{p}");
    }
    assert!(refus(&format!("{h}uid 9\nport sctp 80\n")).1.contains("protocole inconnu"));
    // Groupe NFLOG.
    for g in ["0", "65536", "x"] {
        assert!(refus(&format!("{h}groupe {g}\nuid 9\n")).1.contains("groupe NFLOG"));
    }
    // Doublons.
    assert!(refus(&format!("{h}compte maia\ncompte maia\n")).1.contains("double"));
    assert!(refus(&format!("{h}uid 998\nuid 998\n")).1.contains("double"));
    assert!(refus(&format!("{h}uid 9\nport tcp 80\nport tcp 80\n")).1.contains("double"));
    assert!(DescriptionHote::analyser(&format!("{h}uid 9\nport tcp 80\nport udp 80\n")).is_ok());
    assert!(refus(&format!("{h}uid 9\nexception a\nexception a\n")).1.contains("double"));
    assert!(refus(&format!("{h}groupe 110\ngroupe 111\nuid 9\n")).1.contains("répété"));
    // Directives inconnues ou mal formées, voies invalides.
    assert!(refus(&format!("{h}uid 9\nautoriser tout\n")).1.contains("directive"));
    assert!(refus(&format!("{h}uid 9 998\n")).1.contains("directive"));
    assert!(refus(&format!("{h}uid 9\nexception Site\n")).1.contains("voie invalide"));
    // Bornes de taille et caractères.
    assert!(refus(&format!("{h}uid 9\n# {}\n", "x".repeat(300))).1.contains("trop longue"));
    assert!(refus(&format!("{h}uid 9\r\n")).1.contains("contrôle"));
    assert!(refus(&format!("{h}{}", "# x\n".repeat(5000))).1.contains("taille"));
    let trop = (1..=65).fold(String::new(), |mut s, u| {
        use std::fmt::Write as _;
        let _ = writeln!(s, "uid {u}");
        s
    });
    assert!(refus(&format!("{h}{trop}")).1.contains("nombre maximal"));
}

#[test]
fn aucune_injection_dans_le_texte() {
    let h = "miyudouane-hote 2\n";
    let tentatives = [
        "compte a\"; flush ruleset; #",
        "compte a } } table inet x { chain y { type filter hook output priority 0; policy drop; }",
        "compte a\nflush ruleset",
        "uid 1; flush ruleset",
        "port tcp 22 } accept",
        "exception x\" accept comment \"voie=y",
        "groupe 110 drop",
        "compte $(id)",
        "compte a\u{0}b",
    ];
    for t in tentatives {
        let r = DescriptionHote::analyser(&format!("{h}uid 9\n{t}\n"));
        assert!(r.is_err(), "tentative acceptée : {t:?}");
    }
    // Tout ce qui est accepté ne produit que des jetons sûrs : aucun guillemet
    // hors des noms quotés, aucune instruction hors de la table.
    let d = description();
    let t = generer_hote(&d, &[]);
    for c in &d.comptes {
        if let Compte::Nom(n) = c {
            assert!(n.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-'));
        }
    }
    assert_eq!(auditer_hote(&t, &BTreeSet::new()), Vec::<String>::new());
}

const BOUCLE_AUTRES: &str = "\t\tmeta skuid != @comptes_cog oifname \"lo\" accept\n";

#[test]
fn audit_detecte_les_ecarts() {
    let d = description();
    let base = rendu_reference();
    let v = voies(&d);
    assert_eq!(auditer_hote(&base, &v), Vec::<String>::new());
    let cas = [
        base.replace("add table inet miyudouane_hote\n", "flush ruleset\n"),
        base.replace("delete table inet miyudouane_hote\n", "delete table inet filter\n"),
        format!("{base}table inet autre {{\n}}\n"),
        base.replace("policy accept;", "policy drop;"),
        base.replace("voie=site", "voie=jayoffice"),
        base.replace(BOUCLE_AUTRES, &format!("{BOUCLE_AUTRES}\t\tip daddr 0.0.0.0/0 accept\n")),
        base.replace(BOUCLE_AUTRES, "\t\toifname \"lo\" accept\n"),
        base.replace(BOUCLE_AUTRES, &format!("{BOUCLE_AUTRES}\t\toifname \"lo\" accept\n")),
        base.replace(BOUCLE_AUTRES, &format!("{BOUCLE_AUTRES}\t\tmeta skuid @comptes_cog oifname \"lo\" accept\n")),
        base.replace(BOUCLE_AUTRES, &format!("{BOUCLE_AUTRES}\t\tmeta skuid != @comptes_cog accept\n")),
        base.replace("ip daddr 127.0.0.1 tcp dport 50051", "ip daddr 127.0.0.53 tcp dport 50051"),
        base.replace("ip daddr 127.0.0.1 tcp dport 50051", "ip daddr 127.0.0.1 udp dport 53"),
        base.replace("ip daddr 127.0.0.1 tcp dport 50051", "ip daddr 127.0.0.1 tcp dport { 1-65535 }"),
        base.replace("meta skuid \"miyukini-maia\" oifname", "meta skuid \"intrus\" oifname"),
        base.replace("meta skuid \"miyukini-miyucloud\" oifname", "meta skuid 0 oifname"),
        base.replace("meta skuid \"miyukini-maia\" oifname", "meta skuid @comptes_cog oifname"),
        base.replace("meta skuid \"miyukini-maia\" oifname \"lo\" ", "meta skuid \"miyukini-maia\" "),
        base.replace("tcp dport 5432 accept", "tcp dport 5432 accept\n\t\tmeta skuid \"miyukini-maia\" oifname \"lo\" accept"),
        base.replace("ct state established,related accept", "ct state new accept"),
        base.replace(
            "meta skuid @comptes_cog ip daddr 10.77.0.253",
            "meta skuid @comptes_cog oifname \"lo\" ip daddr 10.77.0.253",
        ),
        base.replace("\t\tiifname \"lo\" accept\n", "\t\toifname \"lo\" accept\n"),
        base.replace("\t\tmeta skuid @comptes_cog counter name \"rejets\" drop\n", ""),
        base.lines().filter(|l| !l.contains("sortie|SORTIE-COG")).flat_map(|l| [l, "\n"]).collect(),
        base.replace(BOUCLE_AUTRES, &format!("{BOUCLE_AUTRES}\t\tjump ailleurs\n")),
        base.trim_end().trim_end_matches('}').to_string(),
    ];
    for (i, t) in cas.iter().enumerate() {
        assert_ne!(t, &base, "cas {i} : altération sans effet (motif absent du rendu)");
        assert!(!auditer_hote(t, &v).is_empty(), "cas {i} non détecté :\n{t}");
    }
    // Exception présente dans le texte mais non déclarée à l'audit.
    assert_ne!(auditer_hote(&base, &BTreeSet::new()), Vec::<String>::new());
}

#[test]
fn exceptions_seulement_par_arrete_signe() {
    let p = plan();
    let rien = BTreeSet::new();
    let d = description();
    // Exception sans arrêté : refus (fail-closed).
    let e = exceptions_autorisees(&d, &[], &p, &rien, T0 + 1).unwrap_err();
    assert!(e.contains("aucun flux"), "{e}");
    // Arrêté de la frontière externe : la zone COG n'ouvre jamais vers l'extérieur.
    let e = exceptions_autorisees(&d, &[verifie(&arrete_externe(1))], &p, &rien, T0 + 1).unwrap_err();
    assert!(e.contains("INV-Z1"), "{e}");
    // Arrêté hors vigueur.
    let e = exceptions_autorisees(&d, &[verifie(&arrete_portail(1))], &p, &rien, T0 + 40 * 86_400).unwrap_err();
    assert!(e.contains("hors vigueur"), "{e}");
    // Même frontière deux fois.
    let deux = [verifie(&arrete_portail(1)), verifie(&arrete_portail(2))];
    assert!(exceptions_autorisees(&d, &deux, &p, &rien, T0 + 1).unwrap_err().contains("deux fois"));
    // Voie ambiguë : déclarée par les deux portails.
    let mut local = arrete_local(1);
    local.voies.push(voie("site"));
    local.flux.push(flux("site", Zone::Cog, Zone::FrancheLocale, Protocole::Tcp, 8883, &[]));
    let mut plan_site = p.clone();
    plan_site.ajouter_sandbox(Zone::FrancheLocale, "site", miyudouane::Ressources::reference()).unwrap();
    let e = exceptions_autorisees(&d, &[verifie(&arrete_portail(1)), verifie(&local)], &plan_site, &rien, T0 + 1)
        .unwrap_err();
    assert!(e.contains("ambiguë"), "{e}");
    // Coupure : la voie disparaît, sans erreur.
    let coupees: BTreeSet<String> = ["site".to_string()].into();
    let ex = exceptions_autorisees(&d, &[verifie(&arrete_portail(1))], &p, &coupees, T0 + 1).unwrap();
    assert_eq!(ex, vec![]);
    // Cas nominal : destination = adresse du portail côté zone COG.
    let ex = exceptions_autorisees(&d, &[verifie(&arrete_portail(1))], &p, &rien, T0 + 1).unwrap();
    let [ExceptionSortie { frontiere, voie, port, destinations, .. }] = ex.as_slice() else { panic!("{ex:?}") };
    assert_eq!((*frontiere, voie.as_str(), *port), (Frontiere::PortailInterne, "site", 7080));
    assert_eq!(destinations.len(), 1);
    assert_eq!(p.zone_de(destinations[0].adresse()), Zone::Cog);
}

// ------------------------------------------------- boucle locale (M3)

/// Paquet émis par un processus local (chaîne `sortie`).
struct Paquet<'a> {
    /// Élément du set tel que rendu (`"miyukini-maia"`, `998`).
    compte: &'a str,
    /// Le compte est-il dans `@comptes_cog` ?
    cog: bool,
    oif: &'a str,
    v6: bool,
    daddr: &'a str,
    proto: &'a str,
    dport: u16,
    /// Conntrack : connexion déjà établie (réponse à un client local…).
    etabli: bool,
}

const fn paquet<'a>(compte: &'a str, cog: bool, daddr: &'a str, proto: &'a str, dport: u16) -> Paquet<'a> {
    Paquet { compte, cog, oif: "lo", v6: false, daddr, proto, dport, etabli: false }
}

/// Évalue la chaîne `sortie` (formes de règles produites par le générateur) :
/// `Some(true)` accept, `Some(false)` drop, `None` = policy accept.
fn verdict_sortie(texte: &str, p: &Paquet<'_>) -> Option<bool> {
    let debut = texte.find("\tchain sortie {\n").expect("chaîne sortie");
    let corps = &texte[debut..];
    let corps = &corps[..corps.find("\n\t}").unwrap()];
    for regle in corps.lines().skip(2) {
        let regle = regle.trim();
        if regle.contains(" log ") {
            continue; // journal : non terminal
        }
        let mots: Vec<&str> = regle.split_whitespace().collect();
        let mut pos = 0;
        let mut ok = true;
        let terminal = loop {
            match &mots[pos..] {
                ["meta", "skuid", "!=", "@comptes_cog", ..] => {
                    ok &= !p.cog;
                    pos += 4;
                }
                ["meta", "skuid", "@comptes_cog", ..] => {
                    ok &= p.cog;
                    pos += 3;
                }
                ["meta", "skuid", val, ..] => {
                    ok &= p.compte == *val;
                    pos += 3;
                }
                ["oifname", val, ..] => {
                    ok &= val.trim_matches('"') == p.oif;
                    pos += 2;
                }
                ["ct", "state", "established,related", ..] => {
                    ok &= p.etabli;
                    pos += 3;
                }
                ["ip", "daddr", adr, ..] => {
                    ok &= !p.v6 && p.daddr == *adr;
                    pos += 3;
                }
                ["ip6", "daddr", adr, ..] => {
                    ok &= p.v6 && p.daddr == *adr;
                    pos += 3;
                }
                [pr @ ("tcp" | "udp"), "dport", num, ..] => {
                    ok &= p.proto == *pr && num.parse::<u16>() == Ok(p.dport);
                    pos += 3;
                }
                ["counter", "accept", ..] | ["accept"] => break true,
                ["counter", "name", "\"rejets\"", "drop"] => break false,
                reste => panic!("forme de règle inattendue : {reste:?} dans {regle}"),
            }
        };
        if ok {
            return Some(terminal);
        }
    }
    None
}

#[test]
fn boucle_locale_restreinte_par_compte() {
    // Description réelle de la VM : MAIA -> KindMother, MiyuCloud -> PostgreSQL.
    let t = rendu_reference();
    let v = |p: Paquet<'_>| verdict_sortie(&t, &p);
    let maia = "\"miyukini-maia\"";
    let km = "\"miyukini-kindmother\"";
    let cloud = "\"miyukini-miyucloud\"";
    let portail = "\"miyukini-portail\"";
    // Résolveur local (systemd-resolved 127.0.0.53:53) : rejet journalisé, pour
    // tous les comptes COG, en UDP comme en TCP ; 127.0.0.1:53 aussi.
    for c in [maia, km, cloud, portail] {
        assert_eq!(v(paquet(c, true, "127.0.0.53", "udp", 53)), Some(false), "{c}");
        assert_eq!(v(paquet(c, true, "127.0.0.53", "tcp", 53)), Some(false), "{c}");
        assert_eq!(v(paquet(c, true, "127.0.0.1", "udp", 53)), Some(false), "{c}");
        assert_eq!(v(Paquet { v6: true, ..paquet(c, true, "::1", "udp", 53) }), Some(false), "{c}");
    }
    // Flux déclarés : acceptés, pour CE compte seulement.
    assert_eq!(v(paquet(maia, true, "127.0.0.1", "tcp", 50051)), Some(true));
    assert_eq!(v(paquet(cloud, true, "127.0.0.1", "tcp", 5432)), Some(true));
    for c in [km, cloud, portail] {
        assert_eq!(v(paquet(c, true, "127.0.0.1", "tcp", 50051)), Some(false), "{c} -> KindMother");
    }
    for c in [maia, km, portail] {
        assert_eq!(v(paquet(c, true, "127.0.0.1", "tcp", 5432)), Some(false), "{c} -> PostgreSQL");
    }
    // Ports COG d'écoute (`port`) non déclarés en `boucle` : refusés, y compris
    // vers son propre service (MiyuCloud -> 8086) et la passerelle LLM de MAIA.
    assert_eq!(v(paquet(cloud, true, "127.0.0.1", "tcp", 8086)), Some(false));
    assert_eq!(v(paquet(maia, true, "127.0.0.1", "tcp", 11435)), Some(false));
    assert_eq!(v(paquet(portail, true, "127.0.0.1", "tcp", 8090)), Some(false));
    // Relais / tunnel en écoute locale sur un port quelconque : refusé.
    assert_eq!(v(paquet(maia, true, "127.0.0.1", "tcp", 3128)), Some(false));
    assert_eq!(v(paquet(maia, true, "127.0.0.1", "udp", 50051)), Some(false));
    // Autre adresse de boucle (127/8, adresse de l'hôte livrée par lo) : refusée.
    assert_eq!(v(paquet(maia, true, "127.0.0.2", "tcp", 50051)), Some(false));
    assert_eq!(v(paquet(maia, true, "192.168.1.10", "tcp", 50051)), Some(false));
    // IPv6 ::1 : seul l'IPv4 est déclaré pour la VM, l'IPv6 est donc refusé.
    assert_eq!(v(Paquet { v6: true, ..paquet(maia, true, "::1", "tcp", 50051) }), Some(false));
    assert_eq!(v(Paquet { v6: true, ..paquet(cloud, true, "::1", "tcp", 5432) }), Some(false));
    // Réponses d'un service COG à un client local (connexion établie) : admises
    // en boucle locale seulement.
    assert_eq!(v(Paquet { etabli: true, ..paquet(km, true, "127.0.0.1", "tcp", 41_000) }), Some(true));
    assert_eq!(v(Paquet { etabli: true, v6: true, ..paquet(portail, true, "::1", "tcp", 41_000) }), Some(true));
    assert_eq!(v(Paquet { etabli: true, oif: "eth0", ..paquet(km, true, "203.0.113.9", "tcp", 443) }), Some(false));
    // Sortie hors boucle : rejet, sauf exception signée.
    assert_eq!(v(Paquet { oif: "eth0", ..paquet(maia, true, "1.1.1.1", "udp", 53) }), Some(false));
    assert_eq!(v(Paquet { oif: "veth-cog", ..paquet(maia, true, "10.77.0.253", "tcp", 7080) }), Some(true));
    // Autres comptes de l'hôte (postgres, utilisateurs) : non concernés.
    let pg = "\"postgres\"";
    assert_eq!(v(paquet(pg, false, "127.0.0.53", "udp", 53)), Some(true));
    assert_eq!(v(paquet(pg, false, "127.0.0.1", "tcp", 50051)), Some(true));
    assert_eq!(v(Paquet { v6: true, ..paquet(pg, false, "::1", "tcp", 9999) }), Some(true));
    assert_eq!(v(Paquet { oif: "eth0", ..paquet(pg, false, "1.1.1.1", "udp", 53) }), None);
    // Le rejet des comptes COG est bien journalisé avant d'être compté.
    let sortie = &t[t.find("\tchain sortie").unwrap()..];
    let journal = sortie.find("sortie|SORTIE-COG|").unwrap();
    assert!(journal < sortie.find("meta skuid @comptes_cog counter name \"rejets\" drop").unwrap());
}

#[test]
fn boucle_locale_ipv6_et_uid() {
    let t = generer_hote(&description_essai(), &[]);
    let v = |p: Paquet<'_>| verdict_sortie(&t, &p);
    let maia = "\"miyukini-maia\"";
    let km = "\"miyukini-kindmother\"";
    // ::1 déclaré : accepté pour ce compte, refusé pour un autre compte COG.
    assert_eq!(v(Paquet { v6: true, ..paquet(maia, true, "::1", "tcp", 50051) }), Some(true));
    assert_eq!(v(Paquet { v6: true, ..paquet(km, true, "::1", "tcp", 50051) }), Some(false));
    // Compte par UID, UDP sur ::1 seulement.
    assert_eq!(v(Paquet { v6: true, ..paquet("998", true, "::1", "udp", 5353) }), Some(true));
    assert_eq!(v(paquet("998", true, "127.0.0.1", "udp", 5353)), Some(false));
    assert_eq!(v(Paquet { v6: true, ..paquet(maia, true, "::1", "udp", 5353) }), Some(false));
    assert_eq!(v(Paquet { v6: true, ..paquet("998", true, "::1", "tcp", 5353) }), Some(false));
    // DNS sur ::1 : refusé.
    assert_eq!(v(Paquet { v6: true, ..paquet("998", true, "::1", "udp", 53) }), Some(false));
}

#[test]
fn boucle_validation() {
    let h = "miyudouane-hote 2\ncompte maia\nuid 998\n";
    assert!(DescriptionHote::analyser(&format!("{h}boucle maia udp ::1 5353\n")).is_ok());
    // Ordre indifférent : compte déclaré après la boucle.
    assert!(DescriptionHote::analyser("miyudouane-hote 2\nboucle 42 tcp 127.0.0.1 80\nuid 42\n").is_ok());
    let cas = [
        ("boucle kindmother tcp 127.0.0.1 50051", "non déclaré"),
        ("boucle 999 tcp 127.0.0.1 50051", "non déclaré"),
        ("boucle maia tcp 127.0.0.53 53", "adresse de boucle"),
        ("boucle maia tcp 127.0.0.2 80", "adresse de boucle"),
        ("boucle maia tcp localhost 80", "adresse de boucle"),
        ("boucle maia tcp 0:0:0:0:0:0:0:1 80", "adresse de boucle"),
        ("boucle maia tcp 10.77.0.253 80", "adresse de boucle"),
        ("boucle maia udp 127.0.0.1 53", "DNS"),
        ("boucle maia tcp ::1 53", "DNS"),
        ("boucle maia tcp 127.0.0.1 0", "port hors bornes"),
        ("boucle maia tcp 127.0.0.1 65536", "port hors bornes"),
        ("boucle maia sctp 127.0.0.1 80", "protocole inconnu"),
        ("boucle 0998 tcp 127.0.0.1 80", "UID hors bornes"),
        ("boucle 0 tcp 127.0.0.1 80", "UID hors bornes"),
        ("boucle root tcp 127.0.0.1 80", "nom de compte invalide"),
        ("boucle @comptes_cog tcp 127.0.0.1 80", "nom de compte invalide"),
        ("boucle maia tcp 127.0.0.1", "directive"),
        ("boucle maia tcp 127.0.0.1 80 81", "directive"),
    ];
    for (ligne, motif) in cas {
        let (n, m) = refus(&format!("{h}{ligne}\n"));
        assert!(m.contains(motif), "{ligne} : {m}");
        assert_eq!(n, 4, "{ligne}");
    }
    assert!(refus(&format!("{h}boucle maia tcp 127.0.0.1 80\nboucle maia tcp 127.0.0.1 80\n")).1.contains("double"));
    // Sans `boucle` : aucun accès en boucle locale pour les comptes COG.
    let d = DescriptionHote::analyser(&format!("{h}port tcp 50051\n")).unwrap();
    let t = generer_hote(&d, &[]);
    assert_eq!(verdict_sortie(&t, &paquet("\"maia\"", true, "127.0.0.1", "tcp", 50051)), Some(false));
    assert_eq!(auditer_hote(&t, &BTreeSet::new()), Vec::<String>::new());
}

// ------------------------------------------------------------------ CLI

struct Banc {
    dossier: PathBuf,
    ancre: String,
}

impl Banc {
    fn nouveau(nom: &str) -> Self {
        let dossier = std::env::temp_dir().join(format!("miyudouane-hote-{nom}-{}", std::process::id()));
        std::fs::create_dir_all(&dossier).unwrap();
        let ancre = cle().cle_publique().iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write as _;
            let _ = write!(s, "{b:02x}");
            s
        });
        Self { dossier, ancre }
    }

    fn ecrire(&self, nom: &str, octets: &[u8]) -> String {
        let chemin = self.dossier.join(nom);
        std::fs::write(&chemin, octets).unwrap();
        chemin.display().to_string()
    }

    fn lancer(&self, args: &[&str]) -> (bool, String, String) {
        assert!(self.dossier.exists());
        let mut c = Command::new(env!("CARGO_BIN_EXE_miyudouane"));
        c.arg("hote").args(args).args(["--maintenant", &(T0 + 1).to_string()]);
        for s in ["cog:jayoffice", "franche:jayoffice", "franche:site", "cog:alicia", "locale:alicia", "cog:site"] {
            c.args(["--sandbox", s]);
        }
        let o = c.output().unwrap();
        (o.status.success(), String::from_utf8_lossy(&o.stdout).into_owned(), String::from_utf8_lossy(&o.stderr).into_owned())
    }
}

impl Drop for Banc {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dossier);
    }
}

#[test]
fn cli_hote() {
    let b = Banc::nouveau("cli");
    let desc = b.ecrire("cog.hote", DESCRIPTION.as_bytes());
    let pi = b.ecrire("pi.arrete", &sceller_arrete(&arrete_portail(1), &cle(), &Constitution::reference()).unwrap());
    let (ok, texte, err) =
        b.lancer(&["--description", &desc, "--arrete", &format!("portail-interne={pi}"), "--ancre", &b.ancre]);
    assert!(ok, "{err}");
    // Même texte que la bibliothèque (signature déterministe, même empreinte).
    assert_eq!(texte, rendu_reference());
    // Coupure de la voie : exception retirée.
    let (ok, coupe, err) = b.lancer(&[
        "--description",
        &desc,
        "--arrete",
        &format!("portail-interne={pi}"),
        "--ancre",
        &b.ancre,
        "--couper",
        "site",
    ]);
    assert!(ok, "{err}");
    assert!(!coupe.contains("voie=site"));
    // Exception déclarée sans arrêté : refus.
    let (ok, _, err) = b.lancer(&["--description", &desc]);
    assert!(!ok && err.contains("aucun flux"), "{err}");
    // Mauvaise ancre : refus.
    let (ok, _, _) =
        b.lancer(&["--description", &desc, "--arrete", &format!("portail-interne={pi}"), "--ancre", &"11".repeat(32)]);
    assert!(!ok);
    // Frontière annoncée ≠ frontière de l'arrêté.
    let (ok, _, err) =
        b.lancer(&["--description", &desc, "--arrete", &format!("portail-local={pi}"), "--ancre", &b.ancre]);
    assert!(!ok && err.contains("portail-interne"), "{err}");
    // Description invalide : refus avec la ligne.
    let mauvaise = b.ecrire("mauvaise.hote", b"miyudouane-hote 2\nuid 0\n");
    let (ok, _, err) = b.lancer(&["--description", &mauvaise]);
    assert!(!ok && err.contains("ligne 2"), "{err}");
    // Description trop grande : refus sans tout lire.
    let grande = b.ecrire("grande.hote", "# x\n".repeat(10_000).as_bytes());
    let (ok, _, err) = b.lancer(&["--description", &grande]);
    assert!(!ok && err.contains("taille"), "{err}");
    // Sans description.
    let (ok, _, err) = b.lancer(&[]);
    assert!(!ok && err.contains("--description"), "{err}");
}

/// `nft -c -f` (contrôle à blanc). Sans root (ni espace de noms utilisateur),
/// `nft` analyse la syntaxe puis échoue à l'initialisation du cache
/// (« Operation not permitted ») avant l'évaluation : seule la syntaxe est
/// alors vérifiée (les noms de comptes ne sont pas résolus). Avec root,
/// contrôle complet. Facultatif : `cargo test -p miyudouane -- --ignored`.
#[test]
#[ignore = "nécessite nft"]
fn nft_controle_a_blanc_hote() {
    use std::io::Write as _;
    // Comptes existants partout pour le contrôle complet en root.
    let d = DescriptionHote::analyser(
        "miyudouane-hote 2\ncompte nobody\nuid 998\nport tcp 5432\nport udp 5353\nexception site\n\
         boucle nobody tcp 127.0.0.1 5432\nboucle 998 udp ::1 5353\n",
    )
    .unwrap();
    let ex = exceptions_autorisees(&d, &[verifie(&arrete_portail(1))], &plan(), &BTreeSet::new(), T0 + 1).unwrap();
    let textes = [generer_hote(&d, &ex), generer_hote(&DescriptionHote::analyser("miyudouane-hote 2\nuid 9\n").unwrap(), &[])];
    for (n, t) in textes.iter().enumerate() {
        let chemin = std::env::temp_dir().join(format!("miyudouane-hote-essai-{}-{n}.nft", std::process::id()));
        std::fs::File::create(&chemin).unwrap().write_all(t.as_bytes()).unwrap();
        let sortie = Command::new("nft").args(["-c", "-f"]).arg(&chemin).output().expect("nft");
        let _ = std::fs::remove_file(&chemin);
        let erreurs = String::from_utf8_lossy(&sortie.stderr).into_owned();
        let seulement_privilege =
            erreurs.lines().filter(|l| l.contains("Error:")).all(|l| l.contains("Operation not permitted"));
        assert!(sortie.status.success() || seulement_privilege, "{erreurs}\n{t}");
    }
}
