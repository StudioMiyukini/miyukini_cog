//! @id     miyudouane_tests_regles
//! @do     tester_regles_nftables
//! @role   test
//! @layer  operateur
//! @human  miyudouane — règles nftables : fichiers de référence, refus par défaut, sortie de zone COG, audit, propriété des voies.

#![allow(clippy::assert_is_empty)]

mod commun;

use std::collections::BTreeSet;
use std::path::PathBuf;

use borderguard::{Frontiere, Protocole, Zone};
use commun::*;
use miyudouane::{compiler, nft, zones, InstanceDouane};

fn voies(a: &borderguard::Arrete) -> BTreeSet<String> {
    a.voies.iter().map(|v| v.id.clone()).collect()
}

fn ruleset(a: &borderguard::Arrete) -> String {
    let p = plan();
    let mut i = InstanceDouane::nouvelle(a.frontiere, p, 0).unwrap();
    i.charger(verifie(a), T0 + 1).unwrap();
    i.ruleset(T0 + 1, &BTreeSet::new())
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

/// Les empreintes changent avec la signature : on les masque pour la référence.
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

#[test]
fn reference_portail_interne() {
    reference("portail-interne.nft", &sans_empreinte(&ruleset(&arrete_portail(1))));
}

#[test]
fn reference_externe() {
    reference("externe.nft", &sans_empreinte(&ruleset(&arrete_externe(1))));
}

#[test]
fn reference_zone_cog() {
    let p = plan();
    let portail = compiler(&verifie(&arrete_portail(1)), &p).unwrap();
    let local = compiler(&verifie(&arrete_local(1)), &p).unwrap();
    let t = nft::generer_zone(Zone::Cog, &[(Frontiere::PortailInterne, portail), (Frontiere::PortailLocal, local)], &p)
        .unwrap();
    assert!(nft::auditer(&t, &["jayoffice", "site", "alicia"].map(String::from).into()).is_empty());
    reference("zone-cog.nft", &t);
}

#[test]
fn refus_par_defaut_et_sortie_cog() {
    for a in [arrete_portail(1), arrete_externe(1), arrete_local(1)] {
        let t = ruleset(&a);
        assert!(nft::auditer(&t, &voies(&a)).is_empty(), "{t}");
        assert!(!t.contains("policy accept"));
        assert_eq!(t.matches("policy drop;").count(), 3);
        // Chaque chaîne rejette explicitement toute sortie de la zone COG et toute entrée vers elle.
        assert_eq!(t.matches("ip saddr 10.77.0.0/24 counter log prefix").count(), 3);
        assert_eq!(t.matches("ip daddr 10.77.0.0/24 counter log prefix").count(), 3);
        assert_eq!(t.matches("|NON-DECLARE|").count(), 3);
        // Toute acceptation hors bouclage / établi est une connexion nouvelle d'une voie déclarée.
        for l in t.lines().filter(|l| l.contains(" accept")) {
            let l = l.trim();
            assert!(
                l == "iif \"lo\" accept"
                    || l == "oif \"lo\" accept"
                    || l == "ct state established,related counter accept"
                    || (l.contains("ct state new") && l.contains("comment \"voie=")),
                "acceptation inattendue : {l}"
            );
        }
    }
    // Portail : rupture protocolaire, rien en transit.
    let t = ruleset(&arrete_portail(1));
    let transit = t.split("chain transit").nth(1).unwrap().split("chain sortie").next().unwrap();
    assert!(!transit.contains("ct state new"));
}

#[test]
fn audit_detecte_les_ecarts() {
    let a = arrete_portail(1);
    let t = ruleset(&a);
    let v = voies(&a);
    assert!(!nft::auditer(&t.replacen("policy drop;", "policy accept;", 1), &v).is_empty());
    let intrus = t.replacen("ct state established,related counter accept", "ct state established,related counter accept\n\t\ttcp dport 22 accept", 1);
    assert!(!nft::auditer(&intrus, &v).is_empty());
    let mut reduit = v.clone();
    reduit.remove("site");
    assert!(!nft::auditer(&t, &reduit).is_empty());
    let sans_drop_final = t.replace("|NON-DECLARE|\" group 102 drop", "|NON-DECLARE|\" group 102 accept");
    assert!(!nft::auditer(&sans_drop_final, &v).is_empty());
}

#[test]
fn fermeture_et_coupure_de_voie() {
    let p = plan();
    let i = InstanceDouane::nouvelle(Frontiere::PortailInterne, p.clone(), 0).unwrap();
    let ferme = i.ruleset(T0, &BTreeSet::new());
    assert!(ferme.contains("|FERME|"));
    assert!(nft::auditer(&ferme, &BTreeSet::new()).is_empty());
    assert!(!ferme.contains("ct state new"));

    let a = arrete_portail(1);
    let mut i = InstanceDouane::nouvelle(Frontiere::PortailInterne, p, 0).unwrap();
    i.charger(verifie(&a), T0 + 1).unwrap();
    let coupe = i.ruleset(T0 + 1, &["site".to_string()].into());
    assert!(!coupe.contains("voie=site"));
    assert!(coupe.contains("voie=jayoffice"));
    // Expiré : fermeture.
    assert!(i.ruleset(T0 + 31 * 86_400, &BTreeSet::new()).contains("|FERME|"));
}

#[test]
fn instance_anti_rejeu_et_frontiere() {
    let p = plan();
    let mut i = InstanceDouane::nouvelle(Frontiere::PortailInterne, p, 4).unwrap();
    assert!(matches!(i.charger(verifie(&arrete_portail(4)), T0 + 1), Err(miyudouane::ErreurDouane::Rejeu { .. })));
    assert!(matches!(
        i.charger(verifie(&arrete_externe(9)), T0 + 1),
        Err(miyudouane::ErreurDouane::MauvaiseFrontiere { .. })
    ));
    i.charger(verifie(&arrete_portail(5)), T0 + 1).unwrap();
    assert_eq!(i.dernier_numero(), 5);
    assert!(i.charger(verifie(&arrete_portail(5)), T0 + 1).is_err());
}

#[test]
fn service_sans_sandbox_refuse() {
    let mut a = arrete_portail(1);
    a.voies.push(commun::voie("inconnu"));
    a.flux.push(flux("inconnu", Zone::Cog, Zone::Franche, Protocole::Tcp, 9000, &[]));
    assert!(compiler(&verifie(&a), &plan()).is_err());
}

/// Générateur pseudo-aléatoire déterministe (pas de dépendance de test).
struct Lcg(u64);
impl Lcg {
    fn suivant(&mut self, n: u64) -> u64 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) % n
    }
}

#[test]
fn propriete_aucune_regle_sur_voie_non_declaree() {
    let services = ["jayoffice", "site"];
    let mut g = Lcg(42);
    for essai in 0..300 {
        let frontiere = if g.suivant(2) == 0 { Frontiere::PortailInterne } else { Frontiere::Externe };
        let mut voies_a = Vec::new();
        let mut flux_a = Vec::new();
        for s in services {
            if g.suivant(3) == 0 {
                continue;
            }
            voies_a.push(commun::voie(s));
            for k in 0..=g.suivant(3) {
                let port = 1000 + g.suivant(500) as u16 + k as u16 * 1000;
                let proto = if g.suivant(2) == 0 { Protocole::Tcp } else { Protocole::Udp };
                let f = match frontiere {
                    Frontiere::PortailInterne => flux(s, Zone::Cog, Zone::Franche, proto, port, &[]),
                    _ => match g.suivant(3) {
                        0 => flux(s, Zone::International, Zone::Franche, proto, port, &["192.0.2.0/24"]),
                        1 => flux(s, Zone::Federation, Zone::Franche, proto, port, &[]),
                        _ => flux(s, Zone::Franche, Zone::Origin, proto, port, &["198.51.100.1/32"]),
                    },
                };
                if !flux_a.contains(&f) {
                    flux_a.push(f);
                }
            }
        }
        if flux_a.is_empty() {
            continue;
        }
        let a = arrete(frontiere, essai + 1, voies_a, flux_a);
        let t = ruleset(&a);
        let declarees = voies(&a);
        assert!(nft::auditer(&t, &declarees).is_empty(), "essai {essai}");
        for morceau in t.split("voie=").skip(1) {
            let id = morceau.split(' ').next().unwrap();
            assert!(declarees.contains(id), "essai {essai} : voie {id} non déclarée");
        }
        // Autant de règles « nouvelle connexion » que de flux × (1 en routage, 2 en rupture).
        let attendu = a.flux.len() * if frontiere == Frontiere::PortailInterne { 2 } else { 1 };
        assert_eq!(t.matches("ct state new").count(), attendu, "essai {essai}");
    }
}

#[test]
fn rendu_des_zones() {
    let p = plan();
    let s = zones::script_mise_en_place(&p);
    assert!(s.contains("ip netns add mcog-z-cog"));
    assert!(s.contains("# mcog-s-cog-jayoffice : aucune route par défaut (zone cog)"));
    assert!(!s.contains("ip -n mcog-s-cog-jayoffice route add"));
    assert!(s.contains("ip -n mcog-s-franche-jayoffice route add default via 10.77.1.254"));
    assert!(s.contains("ip netns exec mcog-f-externe sysctl -qw net.ipv4.ip_forward=1"));
    assert!(!s.contains("ip netns exec mcog-f-portail sysctl -qw net.ipv4.ip_forward=1"));
    assert!(s.contains("ip netns exec mcog-f-portail nft -f /etc/miyukini/douane/mcog-f-portail.nft"));
    let unites = zones::unites_systemd(&p);
    let franche = &unites.iter().find(|(c, _)| c == "jayoffice-franche.service.d/miyudouane.conf").unwrap().1;
    assert!(franche.contains("ReadWritePaths=/var/lib/miyukini/franche/jayoffice/db-secondaire"));
    assert!(franche.contains("InaccessiblePaths=/var/lib/miyukini/cog"));
    assert!(franche.contains("NetworkNamespacePath=/run/netns/mcog-s-franche-jayoffice"));
    let cog = &unites.iter().find(|(c, _)| c == "jayoffice-cog.service.d/miyudouane.conf").unwrap().1;
    assert!(!cog.contains("ReadWritePaths"));
    assert!(unites.iter().any(|(c, t)| c == "mcog-cog-jayoffice.slice" && t.contains("MemoryMax=512M")));
    assert!(zones::script_retrait(&p).contains("ip netns del mcog-z-cog"));
}

/// Contrôle à blanc par `nft -c -f`, dans un espace de noms jetable
/// (`unshare -rn`) si possible ; sinon `nft -c -f` sans privilège : l'analyse
/// et l'évaluation (syntaxe, types, unités) se font en espace utilisateur, seul
/// l'envoi au noyau échoue (« Operation not permitted »), toléré ici.
/// Facultatif : `cargo test -p miyudouane -- --ignored`.
#[test]
#[ignore = "nécessite nft"]
fn nft_controle_a_blanc() {
    use std::io::Write as _;
    let p = plan();
    let portail = compiler(&verifie(&arrete_portail(1)), &p).unwrap();
    let local = compiler(&verifie(&arrete_local(1)), &p).unwrap();
    let i = InstanceDouane::nouvelle(Frontiere::PortailLocal, p.clone(), 0).unwrap();
    let textes = [
        ruleset(&arrete_portail(1)),
        ruleset(&arrete_externe(1)),
        ruleset(&arrete_local(1)),
        i.ruleset(T0, &BTreeSet::new()),
        nft::generer_zone(Zone::Cog, &[(Frontiere::PortailInterne, portail), (Frontiere::PortailLocal, local)], &p)
            .unwrap(),
    ];
    for (n, t) in textes.iter().enumerate() {
        let chemin = std::env::temp_dir().join(format!("miyudouane-essai-{}-{n}.nft", std::process::id()));
        std::fs::File::create(&chemin).unwrap().write_all(t.as_bytes()).unwrap();
        let isole = std::process::Command::new("unshare").args(["-rn", "nft", "-c", "-f"]).arg(&chemin).output();
        let sortie = match isole {
            Ok(o) if o.status.success() => o,
            _ => std::process::Command::new("nft").args(["-c", "-f"]).arg(&chemin).output().expect("nft"),
        };
        let _ = std::fs::remove_file(&chemin);
        let erreurs = String::from_utf8_lossy(&sortie.stderr).into_owned();
        let seulement_privilege = erreurs
            .lines()
            .filter(|l| l.contains("Error:"))
            .all(|l| l.contains("Operation not permitted"));
        assert!(sortie.status.success() || seulement_privilege, "{erreurs}\n{t}");
    }
}
