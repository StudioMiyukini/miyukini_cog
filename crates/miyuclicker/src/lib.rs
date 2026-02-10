//! MiyuClicker — Premier jeu officiel Miyukini.
//!
//! Idle/Clicker (gestion) + Carte stratégique (conquête).
//!
//! ## Architecture gameplay (déduite du schéma GUI)
//!
//! ### Boucles de gameplay
//! 1. **Production** : Ouvriers assignés aux 6 bâtiments (Ferme, Scierie, Carrière, Mine, Atelier, Forge)
//! 2. **Transformation** : Ouvrier → Bâtisseur (20 outils + 1/existant) | Ouvrier → Soldat (10 armes + 1/existant)
//! 3. **Construction** : Bâtisseurs assignés aux 3 bâtiments (Maisons, Casernes, Guilde des Maçons)
//! 4. **Croissance** : Maisons → +pop max → +ouvriers naturels → +production
//! 5. **Militaire** : Soldats → déploiement carte → conquête cités
//!
//! @id: miyuclicker_lib_module
//! @role: application
//! @layer: application
//! @do: aggregate_miyuclicker_operators_and_toolkits
//! @human: Service jeu : IdleSim, Save, Combat, Carte. UI migrée vers Tauri + React.

pub mod carte;
pub mod combat;
pub mod idlesim;
pub mod save;
pub mod state;

pub use state::{
    Allocation, AllocationBatisseurs, BuildingCost, BuildingType, Cite, ClickTarget, Deplacement,
    GameState, ProductionRates, Proprietaire, Route,
};
