#![allow(missing_docs)]
//! # MiyuSocialProfile — toolkit.social.profile
//!
//! Kit d'outils profil social (profile, follow). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

// @id: toolkit.social.miyusocialprofile
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuSocialProfile ; expose les modules tools.
/// @do: expose_miyusocialprofile_toolkit
pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod follow;
pub mod profile;
mod store;

pub use admin_cell::{
    miyusocialprofile_admin_cell, MiyusocialprofileAdminCell, MiyusocialprofileIdentification,
    MiyusocialprofileIntegrity, MiyusocialprofileTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyusocialprofileError;
pub use follow::{add as follow_add, followers_list, following_list, remove as follow_remove};
pub use profile::{get as profile_get, update as profile_update};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn ctx() -> GovernedContext {
        GovernedContext::new("u1".into(), 0)
    }

    #[test]
    fn profile_and_follow() {
        let c = ctx();
        let mut data = HashMap::new();
        data.insert("name".into(), "Alice".into());
        profile_update(&c, "u1", &data).unwrap();
        assert_eq!(profile_get(&c, "u1").unwrap().get("name").unwrap(), "Alice");
        follow_add(&c, "u1", "u2").unwrap();
        assert_eq!(following_list(&c, "u1").unwrap(), vec!["u2"]);
        assert_eq!(followers_list(&c, "u2").unwrap(), vec!["u1"]);
        follow_remove(&c, "u1", "u2").unwrap();
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(profile_get(&c, "x").is_err());
    }
}
