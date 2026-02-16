//! Module de gestion de versions d'EverBuddy

/// @id: everbuddy_version
/// @role: data
/// @layer: core
/// @human: Version d'un composant ou du système.
/// @do: represent_version
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    /// @id: everbuddy_version_major
    /// @role: data
    /// @layer: core
    /// @human: Numéro majeur de version.
    /// @do: store_major_version
    /// @depends: everbuddy_version
    pub major: u32,
    /// @id: everbuddy_version_minor
    /// @role: data
    /// @layer: core
    /// @human: Numéro mineur de version.
    /// @do: store_minor_version
    /// @depends: everbuddy_version
    pub minor: u32,
    /// @id: everbuddy_version_patch
    /// @role: data
    /// @layer: core
    /// @human: Numéro de patch de version.
    /// @do: store_patch_version
    /// @depends: everbuddy_version
    pub patch: u32,
}

impl Version {
    /// @id: everbuddy_version_new
    /// @role: infrastructure
    /// @layer: core
    /// @human: Crée une nouvelle version.
    /// @do: create_version
    /// @depends: everbuddy_version
    #[must_use] 
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// @id: everbuddy_version_manager_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de gestion des versions.
/// @do: define_version_manager_contract
pub trait VersionManager {
    /// @id: everbuddy_version_manager_get_current
    /// @role: accessor
    /// @layer: core
    /// @human: Récupère la version actuelle.
    /// @do: get_current_version
    /// @depends: everbuddy_version_manager_trait
    fn get_current(&self) -> Version;
}

/// Gestionnaire par défaut : version fixe en mémoire.
#[derive(Debug)]
pub struct DefaultVersionManager {
    current: Version,
}

impl DefaultVersionManager {
    /// Crée un gestionnaire avec la version donnée.
    #[must_use]
    pub fn new(version: Version) -> Self {
        Self { current: version }
    }

    /// Définit la version courante.
    pub fn set_current(&mut self, version: Version) {
        self.current = version;
    }
}

impl Default for DefaultVersionManager {
    fn default() -> Self {
        Self {
            current: Version::new(0, 1, 0),
        }
    }
}

impl VersionManager for DefaultVersionManager {
    fn get_current(&self) -> Version {
        self.current.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: everbuddy_version_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une version.
    /// @do: verify_version_creation
    /// @depends: everbuddy_version_new
    #[test]
    fn test_version_creation() {
        let version = Version::new(1, 0, 0);
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
    }

    #[test]
    fn test_default_version_manager() {
        let mgr = DefaultVersionManager::new(Version::new(1, 2, 3));
        let v = mgr.get_current();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }
}
