//! @id     miyuvigie_vigie
//! @do     produire_signaux_capteur
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « vigie » : un capteur IDS et ses analyses.

//! Une [`Vigie`] est un capteur implanté dans une zone (Franche, zone COG,
//! BorderGuard, Portail). Elle analyse ses entrées, applique les règles et rend
//! un [`LotVigie`] destiné à CaringNanny.
//!
//! Règles de preuve de vie ([`Pouls`]) :
//! - émis seulement si l'entrée a été obtenue **et** qu'aucune entrée n'a été
//!   rejetée (mal formée, incohérente, rejouée) ;
//! - sinon l'état est [`EtatCapteur::Degrade`] (ou [`EtatCapteur::EnEchec`] si la
//!   source n'a pas pu être relevée) et un signal `capteur_degrade` est émis.
//!
//! Attribution de voie pour le contrôle de sortie : une Vigie implantée dans la
//! sandbox d'**un seul** service (une seule voie couverte) attribue à cette voie
//! les connexions non attribuées (zone COG segmentée, INV-Z8).

use crate::dedup::{Admission, Dedoublonneur};
use crate::evenement::{EvenementFlux, SourceEvenements, SourceJournal};
use crate::nftables::{LecteurNftables, ResultatLigne};
use crate::regles::{JeuRegles, REGLE_SORTIE};
use crate::signal::{
    CapteurId, Categorie, Compteurs, EtatCapteur, EvenementId, LotVigie, Pouls, RegleId, Severite,
    Signal, VoieId,
};
use crate::sortie::{
    analyser_proc_net_tcp, ControleSortie, LigneProc, TentativeConnexion, VerdictSortie,
};
use crate::zone::{PlanAdressage, Zone};

/// Règle des signaux de capteur dégradé.
pub const REGLE_CAPTEUR_DEGRADE: &str = "R-CAPTEUR-DEGRADE";
/// Règle des mouvements latéraux entre voies.
pub const REGLE_LATERAL: &str = "R-LATERAL-VOIE";

/// Configuration d'une Vigie.
#[derive(Debug, Clone)]
pub struct ConfigVigie {
    /// Identifiant du capteur.
    pub capteur: CapteurId,
    /// Zone d'implantation.
    pub zone: Zone,
    /// Voies surveillées.
    pub voies_couvertes: Vec<VoieId>,
    /// Plan d'adressage.
    pub plan: PlanAdressage,
    /// Règles de détection (les règles intégrées s'ajoutent toujours).
    pub regles: JeuRegles,
    /// Exutoires autorisés pour le contrôle de sortie.
    pub controle_sortie: ControleSortie,
    /// Capacité de la mémoire de déduplication.
    pub capacite_dedup: usize,
    /// Fenêtre anti-rejeu (ms).
    pub fenetre_rejeu_ms: u64,
}

/// Capteur IDS.
#[derive(Debug, Clone)]
pub struct Vigie {
    config: ConfigVigie,
    lecteur: LecteurNftables,
    dedup: Dedoublonneur,
    cumul: Compteurs,
}

impl Vigie {
    /// Crée une Vigie.
    pub fn new(config: ConfigVigie) -> Self {
        let lecteur = LecteurNftables::new(config.plan.clone());
        let dedup = Dedoublonneur::new(config.capacite_dedup, config.fenetre_rejeu_ms);
        Self {
            config,
            lecteur,
            dedup,
            cumul: Compteurs::default(),
        }
    }

    /// Attribue à `voie` les journaux du pont de zone passant par le port
    /// `interface` (port de sandbox `sv<n>` du plan réseau de la Douane).
    pub fn declarer_interface_voie(&mut self, interface: impl Into<String>, voie: VoieId) {
        self.lecteur.declarer_interface_voie(interface, voie);
    }

    /// Identifiant du capteur.
    pub fn capteur(&self) -> &CapteurId {
        &self.config.capteur
    }

    /// Compteurs cumulés depuis la création.
    pub fn compteurs_cumules(&self) -> Compteurs {
        self.cumul
    }

    /// Relève une source d'événements (adaptateur Douane) et analyse.
    pub fn relever(&mut self, source: &mut dyn SourceEvenements, maintenant_ms: u64) -> LotVigie {
        if source.capteur() != &self.config.capteur {
            return self.lot_en_echec(maintenant_ms, "source rattachée à un autre capteur");
        }
        match source.relever() {
            Ok(evts) => self.analyser_flux(evts, maintenant_ms),
            Err(e) => self.lot_en_echec(maintenant_ms, &e.to_string()),
        }
    }

    /// Analyse des événements de flux déjà relevés.
    pub fn analyser_flux(&mut self, evts: Vec<EvenementFlux>, maintenant_ms: u64) -> LotVigie {
        let mut c = Compteurs::default();
        let signaux = self.evaluer_flux(evts, &mut c);
        self.clore(signaux, c, maintenant_ms)
    }

    /// Relève une source de lignes de journal (relais NFLOG) et analyse.
    ///
    /// Source en erreur (muette, débordée, liaison perdue) ⇒ lot en échec, sans
    /// pouls ; entrées non converties par la source ⇒ comptées mal formées,
    /// capteur dégradé.
    pub fn relever_journal(&mut self, source: &mut dyn SourceJournal, maintenant_ms: u64) -> LotVigie {
        if source.capteur() != &self.config.capteur {
            return self.lot_en_echec(maintenant_ms, "source rattachée à un autre capteur");
        }
        match source.relever_lignes() {
            Ok(releve) => {
                let mut lot = self.analyser_journal(releve.lignes.iter().map(String::as_str), maintenant_ms);
                if releve.mal_formees > 0 {
                    self.cumul.mal_formees += releve.mal_formees;
                    lot.compteurs.mal_formees += releve.mal_formees;
                    self.degrader(&mut lot, maintenant_ms);
                }
                lot
            }
            Err(e) => self.lot_en_echec(maintenant_ms, &e.to_string()),
        }
    }

    /// Analyse des lignes de journal NFLOG de la Douane (convention `MDOU|`).
    pub fn analyser_journal<'a>(
        &mut self,
        lignes: impl IntoIterator<Item = &'a str>,
        maintenant_ms: u64,
    ) -> LotVigie {
        let mut evts = Vec::new();
        let mut c = Compteurs::default();
        for l in lignes {
            match self.lecteur.analyser(l) {
                ResultatLigne::Evenement(e) => evts.push(e),
                ResultatLigne::HorsConvention => c.hors_convention += 1,
                ResultatLigne::MalFormee(_) => c.mal_formees += 1,
                ResultatLigne::Incoherente(_) => c.incoherentes += 1,
            }
        }
        // Les rejets de lecture s'ajoutent à ceux de l'analyse.
        let signaux = self.evaluer_flux(evts, &mut c);
        self.clore(signaux, c, maintenant_ms)
    }

    /// Contrôle de sortie de la zone COG sur des tentatives de connexion (EX-12).
    pub fn analyser_connexions(
        &mut self,
        tentatives: Vec<TentativeConnexion>,
        maintenant_ms: u64,
    ) -> LotVigie {
        let mut c = Compteurs::default();
        let mut signaux = Vec::new();
        for t in tentatives {
            if !self.admettre(&t.id, t.horodatage_ms, &mut c) {
                continue;
            }
            c.acceptees += 1;
            let voie = t.voie.clone().or_else(|| self.voie_unique());
            let verdict = self
                .config
                .controle_sortie
                .evaluer(voie.as_ref(), t.ip_dst, t.port_dst);
            let zone_dst = self.config.plan.zone_de(t.ip_dst);
            let proc = t.processus.as_deref().unwrap_or("?");
            match verdict {
                VerdictSortie::BoucleLocale | VerdictSortie::Portail(_) => {}
                VerdictSortie::SortieZoneCog => signaux.push(self.signal(
                    &t.id,
                    t.horodatage_ms,
                    voie,
                    (Zone::Cog, zone_dst),
                    Severite::Critique,
                    Categorie::SortieZoneCog,
                    regle(REGLE_SORTIE),
                    format!(
                        "connexion de « {proc} » vers {}:{} hors boucle locale et hors Portail — EX-12",
                        t.ip_dst, t.port_dst
                    ),
                )),
                VerdictSortie::MouvementLateral { origine, cible } => signaux.push(self.signal(
                    &t.id,
                    t.horodatage_ms,
                    Some(origine.clone()),
                    (Zone::Cog, Zone::Portail),
                    Severite::Critique,
                    Categorie::MouvementLateral,
                    regle(REGLE_LATERAL),
                    format!("connexion de « {proc} » (voie {origine}) vers l'extrémité de la voie {cible}"),
                )),
            }
        }
        self.clore(signaux, c, maintenant_ms)
    }

    /// Contrôle de sortie depuis un relevé de `/proc/net/tcp` de la zone COG.
    pub fn analyser_proc_net_tcp(&mut self, contenu: &str, maintenant_ms: u64) -> LotVigie {
        let mut tentatives = Vec::new();
        let mut mal_formees = 0;
        for l in analyser_proc_net_tcp(contenu, maintenant_ms) {
            match l {
                LigneProc::Connexion(t) => tentatives.push(t),
                LigneProc::SansPair => {}
                LigneProc::MalFormee(_) => mal_formees += 1,
            }
        }
        let mut lot = self.analyser_connexions(tentatives, maintenant_ms);
        if mal_formees > 0 {
            // Recalcule l'état : des lignes ont été rejetées.
            self.cumul.mal_formees += mal_formees;
            lot.compteurs.mal_formees += mal_formees;
            self.degrader(&mut lot, maintenant_ms);
        }
        lot
    }

    fn voie_unique(&self) -> Option<VoieId> {
        match self.config.voies_couvertes.as_slice() {
            [v] => Some(v.clone()),
            _ => None,
        }
    }

    fn admettre(&mut self, id: &EvenementId, horodatage_ms: u64, c: &mut Compteurs) -> bool {
        match self.dedup.admettre(id, horodatage_ms) {
            Admission::Nouveau => true,
            Admission::Doublon => {
                c.doublons += 1;
                false
            }
            Admission::Rejeu => {
                c.rejeus += 1;
                false
            }
        }
    }

    fn evaluer_flux(&mut self, evts: Vec<EvenementFlux>, c: &mut Compteurs) -> Vec<Signal> {
        let mut signaux = Vec::new();
        for e in evts {
            if !self.admettre(&e.id, e.horodatage_ms, c) {
                continue;
            }
            c.acceptees += 1;
            for d in self.config.regles.evaluer(&e) {
                signaux.push(self.signal(
                    &e.id,
                    e.horodatage_ms,
                    e.voie.clone(),
                    (e.zone_src, e.zone_dst),
                    d.severite,
                    d.categorie,
                    d.regle,
                    d.explication,
                ));
            }
        }
        signaux
    }

    #[allow(clippy::too_many_arguments)]
    fn signal(
        &self,
        evt: &EvenementId,
        horodatage_ms: u64,
        voie: Option<VoieId>,
        (zone_src, zone_dst): (Zone, Zone),
        severite: Severite,
        categorie: Categorie,
        regle: RegleId,
        detail: String,
    ) -> Signal {
        Signal {
            id: format!("{}:{}:{}", self.config.capteur, evt, regle),
            capteur: self.config.capteur.clone(),
            horodatage_ms,
            voie,
            zone_src,
            zone_dst,
            severite,
            categorie,
            regle,
            preuves: vec![evt.clone()],
            detail,
        }
    }

    fn clore(&mut self, signaux: Vec<Signal>, c: Compteurs, maintenant_ms: u64) -> LotVigie {
        self.cumul.cumuler(&c);
        let mut lot = LotVigie {
            capteur: self.config.capteur.clone(),
            voies_couvertes: self.config.voies_couvertes.clone(),
            signaux,
            pouls: Some(Pouls {
                capteur: self.config.capteur.clone(),
                horodatage_ms: maintenant_ms,
            }),
            etat: EtatCapteur::Nominal,
            compteurs: c,
        };
        if c.rejets() > 0 {
            self.degrader(&mut lot, maintenant_ms);
        }
        lot
    }

    /// Passe le lot en état dégradé : pas de pouls, signal `capteur_degrade`.
    fn degrader(&self, lot: &mut LotVigie, maintenant_ms: u64) {
        if lot.etat != EtatCapteur::Nominal {
            return;
        }
        lot.pouls = None;
        lot.etat = EtatCapteur::Degrade;
        let c = lot.compteurs;
        lot.signaux.push(Signal {
            id: format!(
                "{}:lot-{maintenant_ms}:{REGLE_CAPTEUR_DEGRADE}",
                self.config.capteur
            ),
            capteur: self.config.capteur.clone(),
            horodatage_ms: maintenant_ms,
            voie: None,
            zone_src: self.config.zone,
            zone_dst: self.config.zone,
            severite: Severite::Moyenne,
            categorie: Categorie::CapteurDegrade,
            regle: regle(REGLE_CAPTEUR_DEGRADE),
            preuves: Vec::new(),
            detail: format!(
                "entrées rejetées : {} mal formées, {} incohérentes, {} rejouées",
                c.mal_formees, c.incoherentes, c.rejeus
            ),
        });
    }

    fn lot_en_echec(&self, maintenant_ms: u64, motif: &str) -> LotVigie {
        LotVigie {
            capteur: self.config.capteur.clone(),
            voies_couvertes: self.config.voies_couvertes.clone(),
            signaux: vec![Signal {
                id: format!(
                    "{}:echec-{maintenant_ms}:{REGLE_CAPTEUR_DEGRADE}",
                    self.config.capteur
                ),
                capteur: self.config.capteur.clone(),
                horodatage_ms: maintenant_ms,
                voie: None,
                zone_src: self.config.zone,
                zone_dst: self.config.zone,
                severite: Severite::Moyenne,
                categorie: Categorie::CapteurDegrade,
                regle: regle(REGLE_CAPTEUR_DEGRADE),
                preuves: Vec::new(),
                detail: format!("source non relevée : {motif}"),
            }],
            pouls: None,
            etat: EtatCapteur::EnEchec,
            compteurs: Compteurs::default(),
        }
    }
}

fn regle(id: &'static str) -> RegleId {
    // Identifiants constants valides par construction (vérifiés par test).
    RegleId::constante(id)
}
