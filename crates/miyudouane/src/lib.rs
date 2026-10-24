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
//! - [`application`] : l'unique étape privilégiée (`nft -c -f` puis `nft -f`),
//!   appelée par la CLI `miyudouane … --appliquer`, jamais en test.
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

pub mod application;
pub mod compilation;
pub mod instance;
pub mod ips;
pub mod nft;
pub mod plan;
pub mod zones;

pub use compilation::{compiler, Crochet, ErreurCompilation, RegleCompilee};
pub use instance::{ErreurDouane, InstanceDouane};
pub use ips::{
    canal_evenements, DecisionSentinelle, EvenementDouane, Gravite, MetaFlux, MoteurIps, NatureEvenement, SourceFlux,
    Verdict,
};
pub use plan::{ErreurPlan, ModeFrontiere, PlanReseau, Ressources, Sandbox};
