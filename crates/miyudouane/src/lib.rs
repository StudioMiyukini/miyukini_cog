//! @id     miyudouane_lib
//! @do     declarer_modules_douane
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — Douane : pare-feu nftables + IPS maison appliquant les arrêtés de BorderGuard, une instance par frontière (EX-19, EX-20, EX-21, EX-29).

//! # MiyuDouane — la Douane du COG
//!
//! Opérateur d'exécution sous l'autorité de **BorderGuard** (décision D1 :
//! les Cores gouvernent, des opérateurs exécutent). BorderGuard **définit**
//! les arrêtés signés ([`borderguard::Arrete`]) ; la Douane les **applique** :
//!
//! - [`instance`] : une instance par frontière (externe, portail interne,
//!   portail local), arrêtés vérifiés seulement, anti-rejeu, fermeture sans arrêté ;
//! - [`compilation`] : arrêté + [`plan`] réseau → règles concrètes, source
//!   unique du pare-feu et de l'IPS ;
//! - [`nft`] : texte `nft -f` en refus par défaut, journaux NFLOG pour les
//!   Vigies, audit structurel ;
//! - [`ips`] : IPS en ligne, événements [`ips::EvenementDouane`] vers les Vigies ;
//! - [`zones`] : rendu Linux des zones (espaces de noms, veth, nftables,
//!   unités systemd des sandboxes), texte seulement ;
//! - [`hote`] : table hôte `inet miyudouane_hote` (zone COG sans sortie pour
//!   les services COG lancés sur l'hôte), depuis une description validée ;
//! - [`tc`] : mise en forme du trafic (`tc -batch`) de la frontière externe
//!   depuis les débits déclarés (A-022 §7) : une classe HTB par voie, une
//!   feuille `fq_codel` par flux, classification par marque, audit ;
//! - [`application`] : l'unique étape privilégiée (`nft -c -f` puis `nft -f` ;
//!   `tc -batch` pour la mise en forme), appelée par la CLI
//!   `miyudouane … --appliquer`, jamais en test.
//!
//! ## Convention de journalisation (Vigies)
//!
//! - Préfixe : `MDOU|<lieu>|<chaine>|<MOTIF>|` ; `<lieu>` = `externe`,
//!   `portail-interne`, `portail-local`, `zone-cog`, `zone-franche`,
//!   `zone-locale` ; `<chaine>` = `entree` / `transit` / `sortie` ;
//!   `<MOTIF>` ∈ [`nft::MOTIFS_JOURNAL`].
//! - Groupes NFLOG : 101 externe, 102 portail interne, 103 portail local,
//!   110 zone COG, 111 Franche, 112 Franche locale.
//! - Gravité recommandée : `SORTIE-COG`, `ENTREE-COG`, `NON-DECLARE`,
//!   `LATERAL`, `INVALIDE` = critique ; `DEBIT`, `TAILLE` = élevée ;
//!   `FERME` = information (frontière fermée volontairement).
//!
//! ## Contrat IPS → Vigies
//!
//! ```text
//! let (tx, rx): (Sender<EvenementDouane>, Receiver<EvenementDouane>) = miyudouane::ips::canal_evenements();
//! let mut ips = instance.moteur_ips(tx).expect("arrêté chargé");
//! // côté Vigies : while let Ok(ev) = rx.recv() { … ev.gravite, ev.nature, ev.flux … }
//! ```

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

pub mod application;
pub mod compilation;
pub mod hote;
pub mod instance;
pub mod ips;
pub mod nft;
pub mod plan;
pub mod tc;
pub mod zones;

pub use compilation::{compiler, Crochet, ErreurCompilation, RegleCompilee};
pub use instance::{ErreurDouane, InstanceDouane};
pub use ips::{
    canal_evenements, DecisionSentinelle, EvenementDouane, Gravite, MetaFlux, MoteurIps, NatureEvenement, SourceFlux,
    Verdict,
};
pub use plan::{ErreurPlan, ModeFrontiere, PlanReseau, Ressources, Sandbox};
