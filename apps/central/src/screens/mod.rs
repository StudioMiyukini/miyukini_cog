//! @id     central_screens_mod
//! @do     declarer_module_connexion
//! @role   ui
//! @layer  app
//! @human  central — module « screens mod ».

//! Écrans d'accueil : Rite d'Entrée (COG vierge), Connexion, fenêtre Profil.

mod connexion;
mod profile_window;
mod rite_entree;

pub use connexion::Connexion;
pub use profile_window::ProfileWindow;
pub use rite_entree::RiteEntree;
