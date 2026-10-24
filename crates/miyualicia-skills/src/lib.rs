//! # miyualicia-skills
//!
//! Registre des capacités (skills) exposées par les services COG vers Alicia.
//!
//! ## Concept
//!
//! Chaque service Jay (JayKoa, JayKonta, JayShop, …) déclare ce qu'il sait faire
//! dans un fichier `service.manifest.json` côté package miyumarket, complété par
//! un bloc `alicia_capabilities`. Au démarrage, Alicia scanne ces manifestes,
//! charge les capacités dans un [`SkillRegistry`] in-memory et expose une API
//! de recherche par phrase / par service.
//!
//! Cet état est volontairement P1 : pas encore d'intégration NLU, pas encore
//! de dispatcher d'exécution. Ces deux briques arrivent en P2 / P3.
//!
//! ## Modules publics
//!
//! - [`capability`] — types décrivant une capacité (intent, params, side-effects)
//! - [`manifest`] — parsing du bloc `alicia_capabilities` d'un service
//! - [`registry`] — registre in-memory des skills disponibles
//! - [`discovery`] — chargement des manifestes depuis un répertoire
//! - [`policy`] — politique d'exécution (gates cores + confirmation)
//! - [`errors`] — types d'erreur
//!
//! ## Lois d'autonomie
//!
//! Le registre fonctionne en mode dégradé sans manifeste valide trouvé : il reste
//! vide. Aucune dépendance réseau, aucun appel KindMother, aucune persistance.

#![forbid(unsafe_code)]

// @id service.alicia.skills.lib
// @role skill_registry_root
// @layer 7
// @do expose_alicia_skill_modules
// @human Crate Alicia Skills — exports publics.

pub mod capability;
pub mod discovery;
pub mod errors;
pub mod manifest;
pub mod policy;
pub mod registry;
pub mod tooling;
pub mod watcher;

pub use capability::{ConfirmationPolicy, ParamSchema, ParamType, ServiceCapability};
pub use discovery::{discover_in_dir, reload_in_dir, ManifestSource, ReloadReport};
pub use errors::SkillsError;
pub use manifest::{AliciaCapabilityManifest, ManifestEntry};
pub use policy::AliciaPolicy;
pub use registry::{Skill, SkillMatch, SkillRegistry};
pub use watcher::{ManifestWatcher, TickOutcome};
pub use tooling::{
    build_tool_response, capabilities_to_tools, capability_to_tool, resolve_tool_call,
    split_qualified_id, FunctionCall, FunctionDefinition, ResolvedToolCall, ToolCall,
    ToolDefinition,
};
