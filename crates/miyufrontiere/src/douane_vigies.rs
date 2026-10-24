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
//! | `voie` | `voie` ; à défaut, voie de la **sandbox source** ([`AttributionVoies`]) |
//! | `bloque` | `verdict` `Bloque` / `Autorise` |
//! | `flux.taille` | `octets` |
//! | `horodatage_secs` | `horodatage_ms` (× 1000, saturé) |
//! | — | `id` unique : `douane-<frontière>-a<arrêté>-t<secondes>-n<rang>` |
//!
//! Attribution de voie : un flux **non déclaré** n'a pas de voie dans
//! l'événement de l'IPS (aucune règle ne l'a admis). Une sandbox étant dédiée à
//! un service (INV-Z8, EX-21) et une voie à un service (EX-44), l'adresse
//! **source** désigne la voie émettrice : c'est elle que WorrySentinel coupe
//! (EX-49). La destination n'attribue jamais de voie : une adresse source
//! usurpée vers une sandbox ne doit pas permettre de couper la voie d'autrui ;
//! un tel flux reste un incident hors voie (blocage à la frontière).
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
use miyudouane::{EvenementDouane, PlanReseau, RegleCompilee};
use miyuvigie::{
    CapteurId, ErreurSource, EvenementFlux, EvenementId, Protocole, SourceEvenements, Verdict, VoieId, Zone,
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

/// Table « adresse de sandbox → voie », dérivée du plan réseau et des règles
/// compilées de l'arrêté en vigueur (une voie par service).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AttributionVoies {
    adresses: BTreeMap<IpAddr, VoieId>,
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
        Self { adresses: adresses.into_iter().filter_map(|(a, v)| v.map(|v| (a, v))).collect() }
    }

    /// Voie de la sandbox d'adresse `adresse`.
    pub fn voie(&self, adresse: IpAddr) -> Option<&VoieId> {
        self.adresses.get(&adresse)
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
        let voie = ev
            .voie
            .as_deref()
            .and_then(VoieId::new)
            .or_else(|| self.attribution.voie(flux.source).cloned());
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
            }),
            zones: flux.then_some((borderguard::Zone::Cog, borderguard::Zone::International)),
            voie: voie.map(str::to_owned),
            bloque: true,
        }
    }

    #[test]
    fn conversion_des_champs_et_identifiants_uniques() {
        let (tx, rx) = canal_evenements();
        let mut s = SourceDouane::new(CapteurId::new("douane").unwrap(), rx);
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

    #[test]
    fn zones_exterieures() {
        assert_eq!(zone_vigie(borderguard::Zone::Origin), Zone::Exterieur);
        assert_eq!(zone_vigie(borderguard::Zone::International), Zone::Exterieur);
        assert_eq!(zone_vigie(borderguard::Zone::FrancheLocale), Zone::FrancheLocale);
    }
}
