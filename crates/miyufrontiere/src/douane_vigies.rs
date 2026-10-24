//! @id     miyufrontiere_douane_vigies
//! @do     adapter_evenements_douane_vers_vigies
//! @role   integration
//! @layer  operateur
//! @human  miyufrontiere — adaptateur Douane → Vigies : EvenementDouane (canal de l'IPS) en EvenementFlux.

//! # Douane → Vigies
//!
//! [`SourceDouane`] relève le `Receiver<EvenementDouane>` créé par
//! `miyudouane::canal_evenements` et rend des [`EvenementFlux`] :
//!
//! | `EvenementDouane` | `EvenementFlux` |
//! |---|---|
//! | `zones` (BorderGuard) | `zone_src`, `zone_dst` via [`zone_vigie`] ; Origin et International → `Exterieur` |
//! | `voie` | `voie` ; à défaut, voie de la **sandbox source** ([`AttributionVoies`]) — côté d'entrée seulement, voir ci-dessous |
//! | `nature` | `motif` (`NON-DECLARE`, `SORTIE-COG`, … : règles intégrées A-005) via [`motif_douane`] |
//! | `bloque` | `verdict` `Bloque` / `Autorise` |
//! | `flux.taille` | `octets` |
//! | `horodatage_secs` | `horodatage_ms` (× 1000, saturé) |
//! | — | `id` unique : `douane-<frontière>-a<arrêté>-t<secondes>-n<rang>` |
//!
//! # Attribution de voie (anti-usurpation, A-005)
//!
//! Un flux **non déclaré** n'a pas de voie dans l'événement de l'IPS (aucune
//! règle ne l'a admis). Une sandbox étant dédiée à un service (INV-Z8, EX-21)
//! et une voie à un service (EX-44), la sandbox **émettrice** désigne la voie
//! fautive : c'est elle que WorrySentinel coupe (EX-49). L'adresse source d'un
//! paquet peut être usurpée ; la voie n'est donc attribuée que si **toutes**
//! ces conditions tiennent :
//!
//! - tentative de **connexion nouvelle** (`MetaFlux::nouvelle_connexion`) et
//!   nature autre que `HorsEtat` (un paquet hors connexion ne prouve rien) ;
//! - interface d'entrée connue (`MetaFlux::interface_entree`) et rattachée,
//!   dans le plan réseau, à la frontière de l'événement (lien de zone ou
//!   interface extérieure) ;
//! - zone de l'adresse source = zone de cette interface d'entrée (une adresse
//!   de la zone COG arrivant par le lien Franche est usurpée).
//!
//! Sinon : **aucune voie**. Les règles critiques produisent alors un incident
//! **hors voie** (WorrySentinel `IncidentHorsVoie`) : la détection n'est pas
//! perdue, seule la coupure ciblée est refusée. La destination n'attribue
//! jamais de voie. À une frontière le recoupement est par zone ; l'usurpation
//! entre sandboxes d'une même zone est arrêtée au pont de zone (port `sv<n>`).
//!
//! Les événements **d'état** de la Douane (voie coupée / rétablie, gel,
//! bannissement prononcé…) ne décrivent pas un flux : ils ne sont pas transmis
//! aux règles (ils reflètent des décisions déjà prises) et sont comptés
//! ([`SourceDouane::evenements_etat`]).
//!
//! Fail-closed : canal fermé (Douane arrêtée) ⇒ `Err` (capteur en échec,
//! voie aveugle côté WorrySentinel) ; les événements reçus avant la fermeture
//! sont d'abord rendus.

use std::collections::BTreeMap;
use std::net::IpAddr;
use std::sync::mpsc::{Receiver, TryRecvError};

use borderguard::Protocole as ProtocoleDouane;
use miyudouane::{EvenementDouane, MetaFlux, NatureEvenement, PlanReseau, RegleCompilee};
use miyuvigie::{
    CapteurId, ErreurSource, EvenementFlux, EvenementId, MotifJournal, Protocole, SourceEvenements, Verdict, VoieId,
    Zone,
};

/// Nombre maximal d'événements rendus par relève (le reste attend la suivante).
pub const MAX_EVENEMENTS_PAR_RELEVE: usize = 4096;

/// Zone des Vigies correspondant à une zone BorderGuard.
pub fn zone_vigie(zone: borderguard::Zone) -> Zone {
    match zone {
        borderguard::Zone::Cog => Zone::Cog,
        borderguard::Zone::Franche => Zone::Franche,
        borderguard::Zone::FrancheLocale => Zone::FrancheLocale,
        borderguard::Zone::Federation => Zone::Federation,
        borderguard::Zone::Origin | borderguard::Zone::International => Zone::Exterieur,
    }
}

/// Motif de journal des Vigies correspondant à la nature d'un événement de
/// l'IPS (même vocabulaire que les journaux `MDOU|…|<MOTIF>|`, A-004 / A-005) ;
/// `None` pour une nature sans équivalent (bannissement, voie coupée…).
pub fn motif_douane(nature: &NatureEvenement) -> Option<MotifJournal> {
    match nature {
        NatureEvenement::FluxNonDeclare => Some(MotifJournal::NonDeclare),
        NatureEvenement::SortieZoneCog => Some(MotifJournal::SortieCog),
        NatureEvenement::EntreeZoneCog => Some(MotifJournal::EntreeCog),
        NatureEvenement::HorsEtat => Some(MotifJournal::Invalide),
        NatureEvenement::TailleDepassee => Some(MotifJournal::Taille),
        NatureEvenement::DebitDepasse => Some(MotifJournal::Debit),
        NatureEvenement::ArreteHorsVigueur | NatureEvenement::Gel => Some(MotifJournal::Ferme),
        _ => None,
    }
}

/// Table « adresse de sandbox → voie », dérivée du plan réseau et des règles
/// compilées de l'arrêté en vigueur (une voie par service), et plan réseau
/// servant au recoupement du côté d'entrée.
///
/// Table vide ([`AttributionVoies::default`]) : aucune voie n'est attribuée
/// (fail-closed, incidents hors voie).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AttributionVoies {
    adresses: BTreeMap<IpAddr, VoieId>,
    plan: Option<PlanReseau>,
}

impl AttributionVoies {
    /// Construit la table : pour chaque règle, les sandboxes du service dans
    /// la zone source et la zone destination. Une adresse revendiquée par deux
    /// voies différentes n'est attribuée à aucune (ambiguïté = pas de voie).
    pub fn nouvelle(plan: &PlanReseau, regles: &[RegleCompilee]) -> Self {
        let mut adresses: BTreeMap<IpAddr, Option<VoieId>> = BTreeMap::new();
        for r in regles {
            let Some(voie) = VoieId::new(r.voie.as_str()) else {
                continue;
            };
            for zone in [r.zone_source, r.zone_destination] {
                if let Some(s) = plan.sandbox(zone, &r.service) {
                    adresses
                        .entry(s.adresse)
                        .and_modify(|v| {
                            if v.as_ref() != Some(&voie) {
                                *v = None;
                            }
                        })
                        .or_insert_with(|| Some(voie.clone()));
                }
            }
        }
        Self {
            adresses: adresses.into_iter().filter_map(|(a, v)| v.map(|v| (a, v))).collect(),
            plan: Some(plan.clone()),
        }
    }

    /// Voie de la sandbox d'adresse `adresse` (sans recoupement : voir
    /// [`AttributionVoies::voie_emettrice`] pour l'attribution d'un événement).
    pub fn voie(&self, adresse: IpAddr) -> Option<&VoieId> {
        self.adresses.get(&adresse)
    }

    /// Vrai si l'adresse source de `flux` est cohérente avec son interface
    /// d'entrée sur la frontière `frontiere` (zone de l'interface dans le plan
    /// = zone de l'adresse). Interface absente ou inconnue : faux.
    pub fn entree_coherente(&self, frontiere: borderguard::Frontiere, flux: &MetaFlux) -> bool {
        let (Some(plan), Some(interface)) = (self.plan.as_ref(), flux.interface_entree.as_deref()) else {
            return false;
        };
        let Some(def) = plan.frontiere(frontiere) else {
            return false;
        };
        let zone_source = plan.zone_de(flux.source);
        def.liens.iter().any(|l| l.interface == interface && l.zone == zone_source)
            || def.exterieures.iter().any(|e| e.interface == interface && e.zones.contains(&zone_source))
    }

    /// Voie de la sandbox **émettrice** d'un événement de flux de l'IPS, ou
    /// `None` si l'attribution n'est pas sûre (connexion non nouvelle,
    /// `HorsEtat`, côté d'entrée absent ou incohérent avec la source).
    pub fn voie_emettrice(&self, ev: &EvenementDouane) -> Option<VoieId> {
        let flux = ev.flux.as_ref()?;
        if !flux.nouvelle_connexion || ev.nature == NatureEvenement::HorsEtat {
            return None;
        }
        if !self.entree_coherente(ev.frontiere, flux) {
            return None;
        }
        ev.voie.as_deref().and_then(VoieId::new).or_else(|| self.adresses.get(&flux.source).cloned())
    }
}

/// Source d'événements des Vigies alimentée par l'IPS d'une instance Douane.
#[derive(Debug)]
pub struct SourceDouane {
    capteur: CapteurId,
    recepteur: Receiver<EvenementDouane>,
    attribution: AttributionVoies,
    rang: u64,
    ferme: bool,
    evenements_etat: u64,
}

impl SourceDouane {
    /// Source rattachée au capteur `capteur`, sans attribution par sandbox.
    pub fn new(capteur: CapteurId, recepteur: Receiver<EvenementDouane>) -> Self {
        Self { capteur, recepteur, attribution: AttributionVoies::default(), rang: 0, ferme: false, evenements_etat: 0 }
    }

    /// Ajoute l'attribution de voie par adresse de sandbox source.
    #[must_use]
    pub fn avec_attribution(mut self, attribution: AttributionVoies) -> Self {
        self.attribution = attribution;
        self
    }

    /// Remplace l'attribution (nouvel arrêté chargé).
    pub fn remplacer_attribution(&mut self, attribution: AttributionVoies) {
        self.attribution = attribution;
    }

    /// Événements d'état reçus (non transmis aux règles).
    pub fn evenements_etat(&self) -> u64 {
        self.evenements_etat
    }

    /// Convertit un événement ; `None` pour un événement d'état (sans flux).
    pub fn convertir(&mut self, ev: &EvenementDouane) -> Option<EvenementFlux> {
        let (Some(flux), Some((zs, zd))) = (ev.flux.as_ref(), ev.zones) else {
            self.evenements_etat += 1;
            return None;
        };
        let rang = self.rang;
        self.rang += 1;
        let id = EvenementId::new(format!(
            "douane-{}-a{}-t{}-n{rang}",
            ev.frontiere.nom(),
            ev.numero_arrete,
            ev.horodatage_secs
        ))?;
        // Côté d'entrée seulement (A-005) : sinon aucune voie, incident hors voie.
        let voie = self.attribution.voie_emettrice(ev);
        Some(EvenementFlux {
            id,
            horodatage_ms: ev.horodatage_secs.saturating_mul(1000),
            zone_src: zone_vigie(zs),
            zone_dst: zone_vigie(zd),
            voie,
            verdict: if ev.bloque { Verdict::Bloque } else { Verdict::Autorise },
            octets: u64::from(flux.taille),
            ip_src: Some(flux.source),
            ip_dst: Some(flux.destination),
            port_dst: Some(flux.port_destination),
            protocole: Some(match flux.protocole {
                ProtocoleDouane::Tcp => Protocole::Tcp,
                ProtocoleDouane::Udp => Protocole::Udp,
            }),
            motif: motif_douane(&ev.nature),
        })
    }
}

impl SourceEvenements for SourceDouane {
    fn capteur(&self) -> &CapteurId {
        &self.capteur
    }

    fn relever(&mut self) -> Result<Vec<EvenementFlux>, ErreurSource> {
        if self.ferme {
            return Err(ErreurSource("canal de la Douane fermé".into()));
        }
        let mut sortie = Vec::new();
        while sortie.len() < MAX_EVENEMENTS_PAR_RELEVE {
            match self.recepteur.try_recv() {
                Ok(ev) => {
                    if let Some(e) = self.convertir(&ev) {
                        sortie.push(e);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.ferme = true;
                    if sortie.is_empty() {
                        return Err(ErreurSource("canal de la Douane fermé".into()));
                    }
                    break;
                }
            }
        }
        Ok(sortie)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use borderguard::Frontiere;
    use miyudouane::{canal_evenements, Gravite, MetaFlux, NatureEvenement};

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    fn evenement(t: u64, voie: Option<&str>, flux: bool) -> EvenementDouane {
        EvenementDouane {
            horodatage_secs: t,
            frontiere: Frontiere::PortailInterne,
            numero_arrete: 7,
            gravite: Gravite::Critique,
            nature: if flux { NatureEvenement::SortieZoneCog } else { NatureEvenement::VoieCoupee },
            flux: flux.then(|| MetaFlux {
                horodatage_secs: t,
                source: ip("10.77.0.10"),
                destination: ip("93.184.216.34"),
                protocole: ProtocoleDouane::Tcp,
                port_source: 40000,
                port_destination: 443,
                taille: 60,
                nouvelle_connexion: true,
                interface_entree: Some("pi-cog".into()),
            }),
            zones: flux.then_some((borderguard::Zone::Cog, borderguard::Zone::International)),
            voie: voie.map(str::to_owned),
            bloque: true,
        }
    }

    #[test]
    fn conversion_des_champs_et_identifiants_uniques() {
        let (tx, rx) = canal_evenements();
        let plan = PlanReseau::reference(&[(borderguard::Zone::Cog, "jaymail")]).unwrap();
        let mut s = SourceDouane::new(CapteurId::new("douane").unwrap(), rx)
            .avec_attribution(AttributionVoies::nouvelle(&plan, &[]));
        tx.send(evenement(1_800_000_000, Some("jaymail"), true)).unwrap();
        tx.send(evenement(1_800_000_000, Some("jaymail"), true)).unwrap();
        tx.send(evenement(1_800_000_000, Some("jaymail"), false)).unwrap();
        let evts = s.relever().unwrap();
        assert_eq!(evts.len(), 2, "l'événement d'état n'est pas un flux");
        assert_eq!(s.evenements_etat(), 1);
        let e = &evts[0];
        assert_eq!(e.horodatage_ms, 1_800_000_000_000);
        assert_eq!((e.zone_src, e.zone_dst), (Zone::Cog, Zone::Exterieur));
        assert_eq!(e.verdict, Verdict::Bloque);
        assert_eq!(e.voie.as_ref().map(VoieId::as_str), Some("jaymail"));
        assert_eq!(e.motif, Some(MotifJournal::SortieCog));
        assert_eq!(e.octets, 60);
        assert_eq!(e.port_dst, Some(443));
        assert_ne!(evts[0].id, evts[1].id, "identifiants uniques");
        assert!(s.relever().unwrap().is_empty(), "source vivante, rien de neuf");
    }

    #[test]
    fn canal_ferme_rend_d_abord_les_evenements_puis_echoue() {
        let (tx, rx) = canal_evenements();
        let mut s = SourceDouane::new(CapteurId::new("douane").unwrap(), rx);
        tx.send(evenement(1, None, true)).unwrap();
        drop(tx);
        assert_eq!(s.relever().unwrap().len(), 1);
        assert!(s.relever().is_err());
        assert!(s.relever().is_err());
    }

    fn attribution(plan: &PlanReseau) -> AttributionVoies {
        // Voie « jaymail » revendiquée par la sandbox COG du service.
        let mut a = AttributionVoies::nouvelle(plan, &[]);
        let cog = plan.sandbox(borderguard::Zone::Cog, "jaymail").unwrap().adresse;
        a.adresses.insert(cog, VoieId::new("jaymail").unwrap());
        a
    }

    /// Anti-usurpation (A-005) : la voie n'est attribuée que depuis le côté
    /// d'entrée cohérent, pour une connexion nouvelle.
    #[test]
    fn voie_attribuee_seulement_par_le_cote_d_entree() {
        let plan = PlanReseau::reference(&[(borderguard::Zone::Cog, "jaymail")]).unwrap();
        let a = attribution(&plan);
        let mut ev = evenement(1, None, true);
        ev.flux.as_mut().unwrap().source = plan.sandbox(borderguard::Zone::Cog, "jaymail").unwrap().adresse;
        assert_eq!(a.voie_emettrice(&ev), Some(VoieId::new("jaymail").unwrap()), "sortie légitime");

        // Adresse COG usurpée arrivant par le lien Franche : aucune voie.
        let mut usurpe = ev.clone();
        usurpe.flux.as_mut().unwrap().interface_entree = Some("pi-franche".into());
        assert_eq!(a.voie_emettrice(&usurpe), None);
        // Interface absente, inconnue ou d'une autre frontière : aucune voie.
        for i in [None, Some("eth9"), Some("pl-cog")] {
            let mut e = ev.clone();
            e.flux.as_mut().unwrap().interface_entree = i.map(str::to_owned);
            assert_eq!(a.voie_emettrice(&e), None, "{i:?}");
        }
        // Paquet d'une connexion existante, ou HorsEtat : aucune voie.
        let mut ancien = ev.clone();
        ancien.flux.as_mut().unwrap().nouvelle_connexion = false;
        assert_eq!(a.voie_emettrice(&ancien), None);
        let mut hors_etat = ev.clone();
        hors_etat.nature = NatureEvenement::HorsEtat;
        assert_eq!(a.voie_emettrice(&hors_etat), None);
        // La voie de l'événement suit la même règle.
        let mut avec_voie = usurpe.clone();
        avec_voie.voie = Some("jaymail".into());
        assert_eq!(a.voie_emettrice(&avec_voie), None);
        // Sans plan (table vide) : jamais de voie.
        assert_eq!(AttributionVoies::default().voie_emettrice(&ev), None);
    }

    #[test]
    fn natures_vers_motifs() {
        assert_eq!(motif_douane(&NatureEvenement::FluxNonDeclare), Some(MotifJournal::NonDeclare));
        assert_eq!(motif_douane(&NatureEvenement::HorsEtat), Some(MotifJournal::Invalide));
        assert_eq!(motif_douane(&NatureEvenement::VoieCoupee), None);
    }

    #[test]
    fn zones_exterieures() {
        assert_eq!(zone_vigie(borderguard::Zone::Origin), Zone::Exterieur);
        assert_eq!(zone_vigie(borderguard::Zone::International), Zone::Exterieur);
        assert_eq!(zone_vigie(borderguard::Zone::FrancheLocale), Zone::FrancheLocale);
    }
}
