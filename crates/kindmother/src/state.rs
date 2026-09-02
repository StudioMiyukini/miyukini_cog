//! Module de gestion d'état de KindMother
//!
//! Ce module gère l'identité et l'état des instances de base de données (DB Mère et DB Filles).
//! Il fournit les types et les opérations nécessaires pour identifier, créer, et gérer les instances.

use miyukini_kernel::{Id, IdGenerator, UuidIdGenerator};

/// @id kindmother_instance_type
/// @role data
/// @layer core
/// @do represent_instance_type
//@human / @human Type d'instance de base de données. Distingue une DB Mère d'une DB Fille.
/// Type d'instance de base de données.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstanceType {
    /// @id kindmother_instance_type_mother
    /// @role data
    /// @layer core
    /// @do represent_mother_instance
    //@human / @human DB Mère - source de vérité unique et autorité centrale.
    /// @depends kindmother_instance_type
    Mother,
    /// @id kindmother_instance_type_daughter
    /// @role data
    /// @layer core
    /// @do represent_daughter_instance
    //@human / @human DB Fille - instance locale dérivée fonctionnant en mode offline-first.
    /// @depends kindmother_instance_type
    Daughter,
}

/// @id kindmother_instance_identity
/// @role data
/// @layer core
/// @do represent_instance_identity
//@human / @human Identité unique d'une instance de base de données. Permet de distinguer les instances et de tracer l'origine des données.
/// Identité unique d'une instance de base de données.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceIdentity {
    /// @id kindmother_instance_identity_id
    /// @role data
    /// @layer core
    /// @do store_instance_id
    //@human / @human Identifiant unique de l'instance, généré par le kernel.
    /// @depends kindmother_instance_identity
    pub id: Id,
    /// @id kindmother_instance_identity_type
    /// @role data
    /// @layer core
    /// @do store_instance_type
    //@human / @human Type de l'instance (Mère ou Fille).
    /// @depends kindmother_instance_identity
    pub instance_type: InstanceType,
}

impl InstanceIdentity {
    /// @id kindmother_instance_identity_new
    /// @role infrastructure
    /// @layer core
    /// @do create_instance_identity
    //@human / @human Crée une nouvelle identité d'instance avec un identifiant unique généré.
    /// @depends kindmother_instance_identity, kernel_id_generator_generate
    /// Crée une nouvelle identité d'instance.
    ///
    /// # Arguments
    ///
    /// * `instance_type` - Type d'instance (Mère ou Fille)
    ///
    /// # Returns
    ///
    /// Une nouvelle identité avec un identifiant unique généré.
    #[must_use]
    pub fn new(instance_type: InstanceType) -> Self {
        let generator = UuidIdGenerator;
        Self {
            id: generator.generate(),
            instance_type,
        }
    }

    /// @id kindmother_instance_identity_is_mother
    /// @role accessor
    /// @layer core
    /// @do check_if_mother_instance
    //@human / @human Vérifie si l'instance est une DB Mère.
    /// @depends kindmother_instance_identity
    /// Vérifie si l'instance est une DB Mère.
    #[must_use]
    pub fn is_mother(&self) -> bool {
        matches!(self.instance_type, InstanceType::Mother)
    }

    /// @id kindmother_instance_identity_is_daughter
    /// @role accessor
    /// @layer core
    /// @do check_if_daughter_instance
    //@human / @human Vérifie si l'instance est une DB Fille.
    /// @depends kindmother_instance_identity
    /// Vérifie si l'instance est une DB Fille.
    #[must_use]
    pub fn is_daughter(&self) -> bool {
        matches!(self.instance_type, InstanceType::Daughter)
    }
}

/// @id kindmother_instance_state
/// @role data
/// @layer core
/// @do represent_instance_state
//@human / @human État d'une instance de base de données. Contient l'identité et les métadonnées de l'instance.
/// État d'une instance de base de données.
#[derive(Debug, Clone)]
pub struct InstanceState {
    /// @id kindmother_instance_state_identity
    /// @role data
    /// @layer core
    /// @do store_instance_identity
    //@human / @human Identité de l'instance.
    /// @depends kindmother_instance_state, kindmother_instance_identity
    pub identity: InstanceIdentity,
    /// @id kindmother_instance_state_created_at
    /// @role data
    /// @layer core
    /// @do store_creation_timestamp
    //@human / @human Moment de création de l'instance (horodatage).
    /// @depends kindmother_instance_state
    pub created_at: std::time::SystemTime,
}

impl InstanceState {
    /// @id kindmother_instance_state_new
    /// @role infrastructure
    /// @layer core
    /// @do create_instance_state
    //@human / @human Crée un nouvel état d'instance avec une identité générée.
    /// @depends kindmother_instance_state, kindmother_instance_identity_new, kernel_clock_now
    /// Crée un nouvel état d'instance.
    ///
    /// # Arguments
    ///
    /// * `instance_type` - Type d'instance (Mère ou Fille)
    ///
    /// # Returns
    ///
    /// Un nouvel état avec une identité générée et l'horodatage de création.
    #[must_use]
    pub fn new(instance_type: InstanceType) -> Self {
        Self {
            identity: InstanceIdentity::new(instance_type),
            created_at: std::time::SystemTime::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id kindmother_instance_state_test_mother_creation
    /// @role test
    /// @layer core
    /// @do verify_mother_instance_creation
    //@human / @human Test de création d'une instance Mère.
    /// @depends kindmother_instance_state_new
    #[test]
    fn test_mother_instance_creation() {
        let state = InstanceState::new(InstanceType::Mother);
        assert!(state.identity.is_mother());
        assert!(!state.identity.is_daughter());
    }

    /// @id kindmother_instance_state_test_daughter_creation
    /// @role test
    /// @layer core
    /// @do verify_daughter_instance_creation
    //@human / @human Test de création d'une instance Fille.
    /// @depends kindmother_instance_state_new
    #[test]
    fn test_daughter_instance_creation() {
        let state = InstanceState::new(InstanceType::Daughter);
        assert!(state.identity.is_daughter());
        assert!(!state.identity.is_mother());
    }

    /// @id kindmother_instance_state_test_unique_ids
    /// @role test
    /// @layer core
    /// @do verify_unique_instance_ids
    //@human / @human Test que chaque instance a un identifiant unique.
    /// @depends kindmother_instance_state_new
    #[test]
    fn test_unique_ids() {
        let state1 = InstanceState::new(InstanceType::Mother);
        let state2 = InstanceState::new(InstanceType::Mother);
        assert_ne!(state1.identity.id, state2.identity.id);
    }
}
