//! @id     miyuvigie_fuzz_commun
//! @do     outiller_cibles_fuzzing_vigies
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — outillage commun des cibles des Vigies : Vigie de la zone COG, exutoire, lignes types (EX-10).

#![allow(dead_code)]

use miyuvigie::{
    CapteurId, ConfigVigie, ControleSortie, ExutoirePortail, JeuRegles, PlanAdressage, Prefixe, Vigie, VoieId, Zone,
};

/// Ligne NFLOG type de la Douane (convention MDOU).
pub const LIGNE_TYPE: &str = "1729760000.123456 hote nflog-110: MDOU|zone-cog|transit|SORTIE-COG| IN=sv0 OUT=z-pi SRC=10.77.0.10 DST=93.184.216.34 LEN=60 TTL=64 DF PROTO=TCP SPT=40000 DPT=443 SYN";

/// Plan indicatif complété du Portail.
pub fn plan() -> PlanAdressage {
    let mut p = PlanAdressage::indicatif();
    if let Some(portail) = Prefixe::v4(10, 77, 0, 248, 29) {
        p.declarer(portail, Zone::Portail);
    }
    p
}

/// Vigie de la zone COG (voie « jayoffice » sur `sv0`, exutoire du Portail).
pub fn vigie(regles: JeuRegles) -> Vigie {
    let voie = VoieId::new("jayoffice").expect("voie");
    let mut controle = ControleSortie::new();
    controle.declarer_exutoire(ExutoirePortail {
        ip: "10.77.0.250".parse().expect("ip"),
        port: 7001,
        voie: voie.clone(),
    });
    let mut v = Vigie::new(ConfigVigie {
        capteur: CapteurId::new("fuzz-zone-cog").expect("capteur"),
        zone: Zone::Cog,
        voies_couvertes: vec![voie.clone()],
        plan: plan(),
        regles,
        controle_sortie: controle,
        capacite_dedup: 64,
        fenetre_rejeu_ms: 60_000,
    });
    v.declarer_interface_voie("sv0", voie);
    v
}

/// Texte de l'entrée (UTF-8 avec remplacement).
pub fn texte(octets: &[u8]) -> String {
    String::from_utf8_lossy(octets).into_owned()
}
