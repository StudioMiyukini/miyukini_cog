//! Authentification Miyukini Central (Miyukini COG).
//!
//! Connexion et création de compte dans une fenêtre frame ; mot de passe complexe.
//! Validation des ID de profils auprès de la DB mère à la première connexion internet puis à intervalle régulier.
//!
//! ## Profil central et services
//!
//! Le profil central est la base pour les autres services. Chaque service peut lier le profil
//! à sa table dédiée via [`CentralAuthDb::get_profile_service_ref`] et [`CentralAuthDb::set_profile_service_ref`].
//! La table `profile_service_refs(profile_id, service_key, ref_id)` stocke une référence par service :
//! `ref_id` pointe vers l’ID de la ligne dans la table du service (ex. sauvegarde de jeu).
//! Exemple : le service Lord of the Castle utilise `service_key = "lord_of_the_castle"` et stocke
//! l’ID de la sauvegarde du joueur dans `ref_id`.

mod db;
mod mother_client;
mod password;

pub use db::{AuthDbError, CentralAuthDb, CentralProfile};
pub use mother_client::{validate_profiles_with_mother, MotherClientError};
pub use password::{validate_password, password_rules_hint, PasswordError, PASSWORD_MIN_LEN};
