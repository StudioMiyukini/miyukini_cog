//! RLS et permissions : type utilisateur et accès édition (aligné Reference Base de Donnees).
//!
//! @id: auth_user_type_from_profile
//! @do: dérive le type utilisateur (admin, manager, exhibitor, volunteer, visitor) depuis le profil.
//! @layer: domain
//!
//! @id: auth_can_access_edition
//! @do: indique si l'utilisateur peut accéder à une édition (admin, manager de l'édition, ou RLS exposant).
//! @layer: domain

use crate::data::{Profile, UserType};

/// Déduit le type utilisateur depuis le champ `user_type` du profil (table `profiles`).
///
/// @id: auth_user_type_from_profile
/// @do: dérive le type utilisateur (admin, manager, exhibitor, volunteer, visitor) depuis le profil.
/// @layer: domain
pub fn auth_user_type_from_profile(profile: &Profile) -> UserType {
    profile
        .user_type
        .as_deref()
        .map_or(UserType::Unknown, UserType::from_str)
}

/// Indique si l'utilisateur peut accéder à une édition donnée.
///
/// Règles alignées Reference Base de Donnees (RLS) :
/// - Admin : accès à toutes les éditions.
/// - Manager : accès aux éditions où il est dans `edition_team` (vérification côté appelant avec les données édition_team).
/// - Exposant : accès en lecture à ses propres lignes `editions_exposants` (exposant_id = auth.uid()).
///
/// Ce module ne fait pas d'appel REST : l'appelant fournit le profil et, si besoin, un indicateur
/// `is_manager_of_edition` (résultat d'une requête préalable sur `edition_team`).
///
/// @id: auth_can_access_edition
/// @do: indique si l'utilisateur peut accéder à une édition (admin, manager de l'édition, ou RLS exposant).
/// @layer: domain
#[must_use] 
pub fn auth_can_access_edition(
    profile: &Profile,
    _edition_id: &str,
    is_manager_of_edition: bool,
) -> bool {
    let ut = auth_user_type_from_profile(profile);
    match ut {
        UserType::Admin => true,
        UserType::Manager => is_manager_of_edition,
        UserType::Exhibitor | UserType::Volunteer | UserType::Visitor | UserType::Unknown => {
            is_manager_of_edition
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{auth_can_access_edition, auth_user_type_from_profile};
    use crate::data::Profile;
    use crate::data::UserType;

    fn profile_with_user_type(ut: &str) -> Profile {
        Profile {
            user_type: Some(ut.to_string()),
            ..Profile::default()
        }
    }

    #[test]
    fn auth_user_type_from_profile_known() {
        assert_eq!(
            auth_user_type_from_profile(&profile_with_user_type("admin")),
            UserType::Admin
        );
        assert_eq!(
            auth_user_type_from_profile(&profile_with_user_type("manager")),
            UserType::Manager
        );
        assert_eq!(
            auth_user_type_from_profile(&profile_with_user_type("exhibitor")),
            UserType::Exhibitor
        );
    }

    #[test]
    fn auth_user_type_from_profile_unknown() {
        assert_eq!(
            auth_user_type_from_profile(&profile_with_user_type("")),
            UserType::Unknown
        );
        assert_eq!(
            auth_user_type_from_profile(&Profile::default()),
            UserType::Unknown
        );
    }

    #[test]
    fn auth_can_access_edition_admin_always() {
        let p = profile_with_user_type("admin");
        assert!(auth_can_access_edition(&p, "ed1", false));
        assert!(auth_can_access_edition(&p, "ed1", true));
    }

    #[test]
    fn auth_can_access_edition_manager_only_when_is_manager() {
        let p = profile_with_user_type("manager");
        assert!(!auth_can_access_edition(&p, "ed1", false));
        assert!(auth_can_access_edition(&p, "ed1", true));
    }

    #[test]
    fn auth_can_access_edition_exhibitor_only_when_is_manager() {
        let p = profile_with_user_type("exhibitor");
        assert!(!auth_can_access_edition(&p, "ed1", false));
        assert!(auth_can_access_edition(&p, "ed1", true));
    }

    #[test]
    fn auth_can_access_edition_volunteer_only_when_is_manager() {
        let p = profile_with_user_type("volunteer");
        assert!(!auth_can_access_edition(&p, "ed1", false));
        assert!(auth_can_access_edition(&p, "ed1", true));
    }

    #[test]
    fn auth_can_access_edition_visitor_only_when_is_manager() {
        let p = profile_with_user_type("visitor");
        assert!(!auth_can_access_edition(&p, "ed1", false));
        assert!(auth_can_access_edition(&p, "ed1", true));
    }

    #[test]
    fn auth_can_access_edition_unknown_only_when_is_manager() {
        let p = Profile::default();
        assert!(!auth_can_access_edition(&p, "ed1", false));
        assert!(auth_can_access_edition(&p, "ed1", true));
    }

    #[test]
    fn auth_user_type_from_profile_volunteer_visitor() {
        assert_eq!(
            auth_user_type_from_profile(&profile_with_user_type("volunteer")),
            UserType::Volunteer
        );
        assert_eq!(
            auth_user_type_from_profile(&profile_with_user_type("visitor")),
            UserType::Visitor
        );
    }
}
