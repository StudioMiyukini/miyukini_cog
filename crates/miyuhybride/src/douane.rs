//! @id     miyuhybride_douane
//! @do     deriver_entrees_douane_du_manifeste
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — dérivation (sans application) des entrées BorderGuard / Douane depuis le manifeste : voie et flux d'arrêté, sandbox Franche (MODELE-ZONES §10).

//! # Entrées Douane (feature `douane`)
//!
//! MODELE-ZONES §10 : « les Cores en dérivent la taille de la Franche, les
//! flux des portails et les règles de BorderGuard ». Ce module **génère** ces
//! entrées depuis un manifeste vérifié ; il n'émet aucun arrêté et n'applique
//! rien : BorderGuard scelle l'arrêté (clé d'usage « arrêtés »), la Douane
//! l'applique (`miyudouane … --appliquer`).
//!
//! - frontière **externe** : une voie au nom du service et un flux par
//!   entrée déclarée, `Fédération → Franche` (liste blanche vide admise pour
//!   la Fédération par la constitution) ;
//! - sandbox **Franche** : plafonds CPU / mémoire / tâches du contrat, DB
//!   secondaire au chemin réservé par `miyudouane` ;
//! - le Portail interne n'a pas de flux IP par service : sa voie est le
//!   [`miyuportail::ContratVoies`] du manifeste.

use borderguard::arrete::identifiant_valide;
use borderguard::{Flux, Protocole, Voie, Zone};
use miyudouane::{ErreurPlan, PlanReseau, Ressources};

use crate::erreurs::ErreurManifeste;
use crate::manifeste::{ManifesteVerifie, OrigineExterne, Transport};

/// Poids IO par défaut d'une sandbox (le contrat ne le déclare pas).
pub const POIDS_IO_DEFAUT: u32 = 100;

/// Entrées générées pour la Douane et BorderGuard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntreesDouane {
    /// Voie de la frontière externe.
    pub voie: Voie,
    /// Flux autorisés vers la façade (à inclure dans le prochain arrêté de la frontière externe).
    pub flux: Vec<Flux>,
    /// Plafonds de la sandbox Franche.
    pub ressources: Ressources,
    /// Taille maximale de la DB secondaire (Mio).
    pub db_secondaire_max_mio: u32,
}

/// Dérive les entrées Douane d'un manifeste H1 / H2 (H0 : aucune entrée, refus).
pub fn entrees_douane(m: &ManifesteVerifie) -> Result<EntreesDouane, ErreurManifeste> {
    let id = m.service().as_str();
    if !identifiant_valide(id) {
        return Err(ErreurManifeste::NonSpecifie("identifiant de service hors alphabet des arrêtés ([a-z0-9-], ≤ 32)"));
    }
    let c = m.contrat();
    let facade = c.facade.ok_or(ErreurManifeste::Classe("H0 : aucune façade, aucune entrée"))?;
    let voie = Voie { id: id.to_string(), service: id.to_string() };
    let flux = c
        .entrees
        .iter()
        .map(|e| Flux {
            voie: voie.id.clone(),
            source: match e.origine {
                OrigineExterne::Federation => Zone::Federation,
            },
            destination: Zone::Franche,
            protocole: match e.transport {
                Transport::Tcp => Protocole::Tcp,
                Transport::Udp => Protocole::Udp,
            },
            port: e.port,
            pairs: Vec::new(),
            debit_max_kio_s: e.debit_max_kio_s,
            taille_max_paquet: e.taille_max_paquet,
        })
        .collect();
    let r = c.ressources;
    Ok(EntreesDouane {
        voie,
        flux,
        ressources: Ressources {
            cpu_pourcent: r.cpu_pourcent,
            memoire_mio: r.memoire_mio,
            taches_max: r.taches_max,
            poids_io: POIDS_IO_DEFAUT,
        },
        db_secondaire_max_mio: facade.db_secondaire_max_mio,
    })
}

/// Ajoute la sandbox Franche de la façade au plan réseau (texte généré par
/// `miyudouane::zones`, jamais appliqué ici).
pub fn ajouter_sandbox_facade(plan: &mut PlanReseau, entrees: &EntreesDouane) -> Result<(), ErreurPlan> {
    plan.ajouter_sandbox(Zone::Franche, &entrees.voie.service, entrees.ressources)?;
    plan.valider()
}
