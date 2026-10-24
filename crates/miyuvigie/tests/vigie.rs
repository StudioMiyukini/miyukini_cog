//! @id     miyuvigie_test_vigie
//! @do     verifier_vigie_bout_en_bout
//! @role   test
//! @layer  toolkit
//! @human  Vigie : contrôle de sortie depuis /proc/net/tcp, source étrangère, règles chargées.

use miyuvigie::{
    CapteurId, Categorie, ConfigVigie, ControleSortie, EtatCapteur, EvenementFlux, EvenementId,
    JeuRegles, PlanAdressage, Severite, SourceEvenements, Verdict, Vigie, VoieId, Zone,
};

fn vigie(voies: &[&str], regles: JeuRegles) -> Vigie {
    Vigie::new(ConfigVigie {
        capteur: CapteurId::new("v1").unwrap(),
        zone: Zone::Cog,
        voies_couvertes: voies.iter().map(|v| VoieId::new(*v).unwrap()).collect(),
        plan: PlanAdressage::indicatif(),
        regles,
        controle_sortie: ControleSortie::new(),
        capacite_dedup: 64,
        fenetre_rejeu_ms: 10_000,
    })
}

#[test]
fn proc_net_tcp_sortie_critique_et_ligne_rejetee() {
    let ext = u32::from_ne_bytes([93, 184, 216, 34]);
    let loc = u32::from_ne_bytes([10, 77, 0, 5]);
    let contenu = format!(
        "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n\
         \x20  0: {loc:08X}:9C40 {ext:08X}:01BB 01 00000000:00000000 00:00000000 00000000  1000        0 2222 1\n\
         \x20  1: n'importe quoi\n"
    );
    let mut v = vigie(&["jaymail"], JeuRegles::vide());
    let lot = v.analyser_proc_net_tcp(&contenu, 5_000);
    assert_eq!(lot.etat, EtatCapteur::Degrade);
    assert!(lot.pouls.is_none());
    let sortie: Vec<_> = lot
        .signaux
        .iter()
        .filter(|s| s.categorie == Categorie::SortieZoneCog)
        .collect();
    assert_eq!(sortie.len(), 1);
    assert_eq!(sortie[0].severite, Severite::Critique);
    assert_eq!(sortie[0].voie.as_ref().map(VoieId::as_str), Some("jaymail"));
    // Relevé suivant identique : connexion déjà signalée (dédoublonnée), lot nominal.
    let propre = contenu.lines().take(2).collect::<Vec<_>>().join("\n");
    let lot = v.analyser_proc_net_tcp(&propre, 6_000);
    assert_eq!(lot.etat, EtatCapteur::Nominal);
    assert_eq!(lot.signaux.len(), 0);
    assert_eq!(lot.compteurs.doublons, 1);
    assert_eq!(v.compteurs_cumules().mal_formees, 1);
}

struct Etrangere(CapteurId);
impl SourceEvenements for Etrangere {
    fn capteur(&self) -> &CapteurId {
        &self.0
    }
    fn relever(&mut self) -> Result<Vec<EvenementFlux>, miyuvigie::ErreurSource> {
        Ok(vec![])
    }
}

#[test]
fn source_d_un_autre_capteur_refusee() {
    let mut v = vigie(&[], JeuRegles::vide());
    let lot = v.relever(&mut Etrangere(CapteurId::new("autre").unwrap()), 1);
    assert_eq!(lot.etat, EtatCapteur::EnEchec);
    assert!(lot.pouls.is_none());
}

#[test]
fn regle_chargee_produit_un_signal() {
    let regles = JeuRegles::charger(
        "vigies-regles 1\nregle R-FR-EXT si zone_src=franche et zone_dst=ext et octets>100 alors elevee flux_hors_contrat\n",
    )
    .unwrap();
    let mut v = vigie(&[], regles);
    let e = EvenementFlux {
        id: EvenementId::new("f1").unwrap(),
        horodatage_ms: 1,
        zone_src: Zone::Franche,
        zone_dst: Zone::Exterieur,
        voie: Some(VoieId::new("jaymail").unwrap()),
        verdict: Verdict::Autorise,
        octets: 5_000,
        ip_src: None,
        ip_dst: None,
        port_dst: None,
        protocole: None,
        motif: None,
    };
    let lot = v.analyser_flux(vec![e], 1);
    assert_eq!(lot.signaux.len(), 1);
    assert_eq!(lot.signaux[0].id, "v1:f1:R-FR-EXT");
    assert_eq!(lot.signaux[0].severite, Severite::Elevee);
}
