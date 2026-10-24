//! @id     bondingbrother_lib
//! @do     declarer_module_connection
//! @role   orchestration
//! @layer  core
//! @human  bondingbrother — module « lib ».

//! # BondingBrother
//!
//! Stratégie de liaison gouvernée du Miyukini Core System.
//!
//! BondingBrother médiatise, traduit, et filtre les interactions entre les Opérateurs
//! et l'écosystème autoritaire sans jamais décider : la décision appartient à StrongFather.
//!
//! ## Porte obligatoire (K-11, C-31, C-32)
//!
//! - Toute action gouvernée (outil MAIA, action Alicia, installation Market, action Admin,
//!   écriture KindMother) passe par [`Gate::autoriser`] ou, mieux, [`executer_si_autorise`]
//! - [`PorteStrongFather`] : porte adossée à `strongfather::PolicyEngine::decide` ; synchrone,
//!   clonable à coût constant ; construite par `depuis_fichier(chemin)` (TOML / JSON) ou
//!   `depuis_politiques(set)`
//! - [`DecisionAutorisee`] : jeton infalsifiable (champs privés, ni `Clone` ni constructeur
//!   public) ; seule une porte de ce crate le délivre, après une décision ALLOW liée à
//!   l'empreinte de l'intention
//! - [`Refus`] : porte la décision de refus complète (règle décisive, raison, empreinte)
//!
//! ## Règles fail-closed
//!
//! - Refus par défaut : aucune politique Authorize applicable ⇒ refus « défaut: refus »
//! - Refus explicite > autorisation ; contrainte violée ⇒ refus
//! - Expression mal formée, attribut inconnu, politique incohérente ⇒ refus
//! - Fichier de politiques illisible, invalide ou vide ⇒ erreur de construction (pas de porte)
//! - Stubs non implémentés (`DefaultConnectionManager`, `DefaultSyncManager`) ⇒ erreur
//!   `NonImplemente`, jamais de succès factice (C-14)
//!
//! ## Joindre la décision à l'action
//!
//! - `executer_si_autorise(&porte, &intent, |jeton| …)` : la fermeture reçoit le jeton ;
//!   journaliser `jeton.regle()` et `jeton.empreinte()` avec l'action
//! - Un auditeur rejoue `decision.matches_intent(&intent)` pour vérifier que la décision
//!   porte bien sur l'action exécutée
//!
//! ## Exemple
//!
//! ```
//! use bondingbrother::{executer_si_autorise, PorteStrongFather};
//! use bondingbrother::strongfather::{
//!     ActionType, CallContext, Intent, IntentData, PolicySet,
//! };
//!
//! let politiques = PolicySet::from_toml_str(r#"
//! version = 1
//! [[policy]]
//! id = "maia-lit-agenda"
//! type = "permission"
//! effect = "authorize"
//! rule = "caller == 'maia'"
//! [policy.target]
//! actions = ["read"]
//! subjects = ["jaykoa.agenda"]
//! "#).unwrap();
//! let porte = PorteStrongFather::depuis_politiques(politiques).unwrap();
//!
//! let intent = Intent::new(
//!     "req-1".into(),
//!     ActionType::Read,
//!     "jaykoa.agenda".into(),
//!     CallContext { caller_identity: "maia".into(), origin: "maia".into(), instance: "cog".into() },
//!     IntentData { content: Default::default() },
//! );
//! let resultat = executer_si_autorise(&porte, &intent, |jeton| jeton.regle().to_string());
//! assert_eq!(resultat.unwrap(), "maia-lit-agenda");
//! ```

pub mod connection;
pub mod gate;
pub mod sync;
pub mod translation;

/// Ré-export de StrongFather : les services passent par BondingBrother (C-32) sans
/// dépendre directement du Core de décision.
pub use strongfather;

pub use connection::{Connection, ConnectionError, ConnectionManager, DefaultConnectionManager};
pub use gate::{executer_si_autorise, DecisionAutorisee, Gate, PorteStrongFather, Refus};
pub use strongfather::{Decision, Intent, Verdict};
pub use sync::{DefaultSyncManager, SyncError, SyncManager, SyncStrategy};
pub use translation::{DefaultTranslator, Translation, TranslationError, Translator};
