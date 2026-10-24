//! @id     miyuvigie_test_journaux_douane
//! @do     verifier_lecture_prefixes_douane
//! @role   test
//! @layer  toolkit
//! @human  Vigies : chaque préfixe `MDOU|…|` et groupe NFLOG réellement généré par la Douane (fichiers de référence) est lu et compris (A-004).

use std::collections::BTreeSet;

use miyuvigie::{
    CapteurId, Categorie, Chaine, ConfigVigie, ControleSortie, JeuRegles, LecteurNftables, Lieu,
    MotifJournal, PlanAdressage, ResultatLigne, Severite, Verdict, Vigie, VoieId, Zone,
};

/// Fichiers de référence de la Douane (`miyudouane/tests/golden`).
const GOLDEN: [&str; 3] = [
    include_str!("../../miyudouane/tests/golden/externe.nft"),
    include_str!("../../miyudouane/tests/golden/portail-interne.nft"),
    include_str!("../../miyudouane/tests/golden/zone-cog.nft"),
];

/// `(préfixe, groupe)` de chaque règle journalisée des fichiers de référence.
fn prefixes_reels() -> BTreeSet<(String, u16)> {
    let mut sortie = BTreeSet::new();
    for texte in GOLDEN {
        for ligne in texte.lines().filter(|l| !l.trim_start().starts_with('#')) {
            let Some(reste) = ligne.split("log prefix \"").nth(1) else {
                continue;
            };
            let (prefixe, apres) = reste.split_once('"').unwrap();
            let groupe: u16 = apres
                .trim()
                .strip_prefix("group ")
                .and_then(|g| g.split(' ').next())
                .unwrap()
                .parse()
                .unwrap();
            sortie.insert((prefixe.to_string(), groupe));
        }
    }
    sortie
}

/// Ligne NFLOG plausible pour un préfixe : interfaces selon la chaîne,
/// adresses selon le motif (comme le noyau les écrirait).
fn ligne_pour(prefixe: &str, groupe: u16) -> String {
    let parties: Vec<&str> = prefixe.split('|').collect();
    let (chaine, motif) = (parties[2], parties[3]);
    let interfaces = match (chaine, motif) {
        (_, "LATERAL") => "IN=sv0 OUT=sv3",
        ("entree", _) => "IN=eth0 OUT=",
        ("sortie", _) => "IN= OUT=eth0",
        _ => "IN=eth0 OUT=eth1",
    };
    let adresses = match motif {
        "SORTIE-COG" => "SRC=10.77.0.10 DST=93.184.216.34",
        "ENTREE-COG" => "SRC=10.77.1.10 DST=10.77.0.10",
        _ => "SRC=10.77.1.10 DST=203.0.113.9",
    };
    format!(
        "1800000000.250000 hote nflog-{groupe}: {prefixe} {interfaces} {adresses} LEN=60 TTL=64 PROTO=TCP SPT=40000 DPT=443 SYN"
    )
}

#[test]
fn tous_les_prefixes_generes_par_la_douane_sont_lus() {
    let lecteur = LecteurNftables::new(PlanAdressage::indicatif());
    let prefixes = prefixes_reels();
    assert!(prefixes.len() >= 20, "fichiers de référence lus : {prefixes:?}");
    let mut lieux = BTreeSet::new();
    for (p, g) in &prefixes {
        let parties: Vec<&str> = p.split('|').collect();
        let lieu = Lieu::depuis_nom(parties[1]).expect("lieu connu");
        assert_eq!(lieu.groupe_nflog(), *g, "groupe du lieu {lieu}");
        let chaine = Chaine::depuis_nom(parties[2]).expect("chaîne connue");
        let motif = MotifJournal::depuis_nom(parties[3]).expect("motif connu");
        assert!(motif.produit_par_la_douane(lieu, chaine), "{p}");
        lieux.insert(lieu);
        let l = ligne_pour(p, *g);
        let ResultatLigne::Evenement(e) = lecteur.analyser(&l) else {
            panic!("préfixe réel refusé : {l} → {:?}", lecteur.analyser(&l));
        };
        assert_eq!(e.verdict, Verdict::Bloque, "toute règle journalisée rejette");
        if motif == MotifJournal::SortieCog {
            assert_eq!(e.zone_src, Zone::Cog);
        }
    }
    assert_eq!(
        lieux,
        BTreeSet::from([Lieu::Externe, Lieu::PortailInterne, Lieu::ZoneCog])
    );
}

/// Sortie de zone COG journalisée par le pont de zone, attribuée à la voie de
/// la sandbox par son port : signal critique EX-12 sur cette voie, même bloquée.
#[test]
fn sortie_cog_du_pont_attribuee_a_la_voie_critique() {
    let mut v = Vigie::new(ConfigVigie {
        capteur: CapteurId::new("nflog-zone-cog").unwrap(),
        zone: Zone::Cog,
        voies_couvertes: vec![VoieId::new("site").unwrap()],
        plan: PlanAdressage::indicatif(),
        regles: JeuRegles::vide(),
        controle_sortie: ControleSortie::new(),
        capacite_dedup: 64,
        fenetre_rejeu_ms: 60_000,
    });
    // Port `sv5` = sandbox COG du service « site » (golden zone-cog.nft).
    v.declarer_interface_voie("sv5", VoieId::new("site").unwrap());
    let ligne = "1800000000.250000 hote nflog-110: MDOU|zone-cog|transit|SORTIE-COG| IN=sv5 OUT=z-pi SRC=10.77.0.12 DST=93.184.216.34 LEN=60 TTL=64 PROTO=TCP SPT=40000 DPT=443 SYN";
    let lot = v.analyser_journal([ligne], 1_800_000_001_000);
    assert!(lot.pouls.is_some());
    assert_eq!(lot.signaux.len(), 1);
    let s = &lot.signaux[0];
    assert_eq!(s.severite, Severite::Critique);
    assert_eq!(s.categorie, Categorie::SortieZoneCog);
    assert_eq!(s.voie.as_ref().map(VoieId::as_str), Some("site"));
}
