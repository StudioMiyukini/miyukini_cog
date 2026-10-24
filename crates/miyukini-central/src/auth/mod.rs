//! @id     miyukini_central_auth_mod
//! @do     declarer_module_mother_client
//! @role   securite
//! @layer  core
//! @human  miyukini-central — module « auth mod ».

//! Authentification Miyukini Central (Miyukini COG) via KindMother Service.
//!
//! Connexion et création de compte dans une fenêtre frame ; mot de passe complexe.
//! Validation des ID de profils auprès de la DB mère à la première connexion internet puis à intervalle régulier.
//!
//! ## Profil central et services
//!
//! Le profil central est la base pour les autres services. Chaque service peut lier le profil
//! à sa table dédiée via [`CentralAuthDb::get_profile_service_ref`] et [`CentralAuthDb::set_profile_service_ref`].
//! La table `profile_service_refs(profile_id, service_key, ref_id)` stocke une référence par service :
//! `ref_id` pointe vers l'ID de la ligne dans la table du service (ex. sauvegarde de jeu).
//! Exemple : le service JayKoa utilise `service_key = "jaykoa"` et stocke
//! l'ID de la sauvegarde du joueur dans `ref_id`.
//!
//! ## Persistance (K-13 : KindMother seul écrivain)
//!
//! - **`kindmother-only`** (défaut) : `db_client`, toutes les écritures passent par des
//!   intentions typées KindMother (aucun texte SQL d'écriture, EX-07).
//! - **`legacy-sqlite`** (HÉRITÉ, transitoire) : `db`, accès SQLite direct.
//!
//! Si les deux features sont actives (cas d'`apps/central` et `crates/central-remote`,
//! qui activent `legacy-sqlite` sans désactiver les features par défaut), **le legacy
//! l'emporte** : activer explicitement `legacy-sqlite` est un choix délibéré de l'appelant.
//! Les deux implémentations exposent la même API (mêmes noms, signatures, sémantique).

mod mother_client;
mod password;

// Legacy prioritaire lorsqu'il est demandé explicitement (voir doc du module).
#[cfg(feature = "legacy-sqlite")]
mod db;

#[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
mod db_client;

#[cfg(feature = "legacy-sqlite")]
pub use db::{AuthDbError, CentralAuthDb};

#[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
pub use db_client::{AuthDbError, CentralAuthDb};

pub use modele::{CentralProfile, CentralProfileSave, CentralProfileSaveRow};
pub use mother_client::{validate_profiles_with_mother, MotherClientError};
pub use password::{password_rules_hint, validate_password, PasswordError, PASSWORD_MIN_LEN};

/// Modèle de données partagé par les deux implémentations (legacy et KindMother),
/// afin qu'elles ne puissent pas diverger.
mod modele {
    /// Ligne de sauvegarde liée au profil Central (données complètes).
    #[derive(Debug, Clone)]
    pub struct CentralProfileSave {
        /// Identifiant unique de la sauvegarde.
        pub id: String,
        /// Identifiant du profil Central.
        pub profile_id: String,
        /// Clé du service (ex. `jaykoa`).
        pub service_key: String,
        /// Numéro de slot (0, 1, 2…).
        pub slot: i64,
        /// Données sérialisées (BLOB).
        pub data: Vec<u8>,
        /// Date de création (ISO).
        pub created_at: String,
        /// Date de dernière mise à jour (ISO).
        pub updated_at: String,
    }

    /// Ligne allégée pour lister les sauvegardes (sans le BLOB).
    #[derive(Debug, Clone)]
    pub struct CentralProfileSaveRow {
        /// Identifiant unique de la sauvegarde.
        pub id: String,
        /// Numéro de slot.
        pub slot: i64,
        /// Date de création (ISO).
        pub created_at: String,
        /// Date de dernière mise à jour (ISO).
        pub updated_at: String,
    }

    /// Profil Miyukini COG (email = login, champs enrichis modifiables).
    #[derive(Debug, Clone)]
    #[allow(missing_docs)]
    pub struct CentralProfile {
        pub id: String,
        pub email: String,
        pub pseudonyme: Option<String>,
        pub nom: Option<String>,
        pub prenom: Option<String>,
        pub date_naissance: Option<String>,
        pub telephone: Option<String>,
        pub numero_voie: Option<String>,
        pub rue: Option<String>,
        pub code_postal: Option<String>,
        pub ville: Option<String>,
        /// Premier compte créé = admin de l'environnement (MiyukiniAdmin).
        pub is_admin: bool,
        /// Genre (Masculin, Féminin, Neutre) — Rite d'Entrée.
        pub genre: Option<String>,
        /// Statut relationnel (Célibataire, En couple, Marié) — Rite d'Entrée.
        pub statut_marital: Option<String>,
        /// Genre du/de la partenaire si non célibataire.
        pub partenaire_genre: Option<String>,
        /// Nom ou prénom du/de la partenaire si non célibataire.
        pub partenaire_nom: Option<String>,
        /// Nombre d'enfants (0 si aucun ou non renseigné).
        pub enfants_nombre: Option<i32>,
        /// Prénoms des enfants (JSON array ou séparés par virgule).
        pub enfants_noms: Option<String>,
        /// Profession ou activité — Rite d'Entrée.
        pub profession: Option<String>,
        /// Langue maternelle.
        pub langue_maternelle: Option<String>,
    }

    impl CentralProfile {
        /// Profil réduit à son identité (champs enrichis vides).
        pub(crate) fn minimal(id: String, email: String, is_admin: bool) -> Self {
            Self {
                id,
                email,
                pseudonyme: None,
                nom: None,
                prenom: None,
                date_naissance: None,
                telephone: None,
                numero_voie: None,
                rue: None,
                code_postal: None,
                ville: None,
                is_admin,
                genre: None,
                statut_marital: None,
                partenaire_genre: None,
                partenaire_nom: None,
                enfants_nombre: None,
                enfants_noms: None,
                profession: None,
                langue_maternelle: None,
            }
        }
    }
}

/// Hachage des mots de passe, partagé par les deux implémentations (mêmes règles).
mod hachage {
    use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
    use sha2::{Digest, Sha256};

    /// Hash SHA-256 legacy (pour rétrocompatibilité).
    pub(crate) fn hash_password_sha256(password: &str) -> String {
        let mut h = Sha256::new();
        h.update(password.as_bytes());
        format!("{:x}", h.finalize())
    }

    /// Hash Argon2id (recommandé pour la production).
    pub(crate) fn hash_password_argon2(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = argon2::Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| format!("argon2 hash error: {e}"))
    }

    /// Vérifie un mot de passe contre un hash stocké (Argon2id ou SHA-256 legacy).
    pub(crate) fn verify_password(password: &str, stored_hash: &str) -> bool {
        if stored_hash.starts_with("$argon2") {
            if let Ok(parsed) = PasswordHash::new(stored_hash) {
                return argon2::Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok();
            }
            false
        } else {
            hash_password_sha256(password) == stored_hash
        }
    }
}
