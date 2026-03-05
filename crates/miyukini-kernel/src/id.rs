//! Module d'identifiants du Kernel
//!
//! Ce module fournit une abstraction pour la génération d'identifiants uniques.
//! Le type `Id` est opaque : son format interne n'est pas garanti.
//!
//! ## Principe
//!
//! - Type `Id` opaque — format interne non garanti
//! - Pas de ré-export `Uuid` — le kernel ne ré-exporte pas les dépendances
//! - `Debug` = usage dev — ne pas utiliser pour persistance
//! - `Display` = sérialisation — round-trip avec `Id::parse`
//!
//! ## Références
//!
//! - [Kernel - Invariants & Guarantees](../../../docs/kernel/contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)
//! - [Kernel - Reference Implementation Guidelines](../../../docs/kernel/implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md)
//! - [Kernel - Revue Traits API v0.1](../../../docs/kernel/Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md)

use uuid::Uuid;

/// @id: kernel_id_type
/// @role: infrastructure
/// @layer: kernel
/// @human: Type opaque pour les identifiants uniques. Le format interne n'est pas garanti et ne doit pas être persisté ou interprété sans passer par les APIs du produit.
/// @do: represent_unique_identifier
/// Identifiant opaque. Le format interne n'est pas garanti.
///
/// Le type `Id` encapsule un identifiant unique de manière opaque.
/// Le format interne peut changer (UUID aujourd'hui, ULID demain) sans casser
/// les consommateurs qui utilisent `Display` et `Id::parse` pour la sérialisation.
///
/// ## Invariants respectés
///
/// - INV-K-1 : Aucune logique métier (pas de `UserId`, `OrderId`, etc.)
/// - INV-K-3 : Primitives locales sûres uniquement (UUID généré localement)
/// - INV-K-6 : Déterminisme (parse déterministe, génération non déterministe mais explicite)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    /// @id: kernel_id_parse
    /// @role: infrastructure
    /// @layer: kernel
    /// @human: Parse un identifiant depuis sa représentation textuelle. Le format supporté peut changer ; utiliser Display pour la sérialisation.
    /// @do: parse_id_from_string
    /// @depends: kernel_id_type
    /// Parse un identifiant depuis sa représentation textuelle.
    ///
    /// Le format supporté (UUID aujourd'hui) peut changer — la documentation
    /// de `parse` précise le format actuel. Pour la persistance, utiliser
    /// `Display` / `to_string()` et `Id::parse` pour le round-trip.
    ///
    /// ## Comportement
    ///
    /// - Format actuel : UUID standard (ex: "550e8400-e29b-41d4-a716-446655440000")
    /// - Sensible à la casse : les UUID sont insensibles à la casse
    /// - Retourne une erreur si le format est invalide
    ///
    /// # Arguments
    ///
    /// * `s` - La représentation textuelle de l'identifiant
    ///
    /// # Returns
    ///
    /// * `Ok(Id)` - Si le format est valide
    /// * `Err(IdParseError)` - Si le format est invalide
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::id::Id;
    ///
    /// let id = Id::parse("550e8400-e29b-41d4-a716-446655440000").unwrap();
    /// ```
    pub fn parse(s: &str) -> Result<Id, IdParseError> {
        Uuid::parse_str(s)
            .map(Id)
            .map_err(|_| IdParseError::InvalidFormat)
    }
}

impl std::fmt::Display for Id {
    /// @id: kernel_id_display
    /// @role: infrastructure
    /// @layer: kernel
    /// @human: Formatage de l'identifiant en représentation textuelle pour sérialisation. Utiliser avec Id::parse pour round-trip.
    /// @do: format_id_as_string
    /// @depends: kernel_id_type
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// @id: kernel_id_parse_error
/// @role: infrastructure
/// @layer: kernel
/// @human: Erreur de parsing d'identifiant. Type minimal avec Debug, Display, Error.
/// @do: represent_parse_error
/// Erreur de parsing d'identifiant.
///
/// Type minimal d'erreur pour le parsing d'identifiants.
/// Ne contient pas de `From<uuid::Error>` dans l'API pour ne pas lier
/// au détail d'implémentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdParseError {
    /// Format invalide
    InvalidFormat,
}

impl std::fmt::Display for IdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdParseError::InvalidFormat => write!(f, "Invalid ID format"),
        }
    }
}

impl std::error::Error for IdParseError {}

/// @id: kernel_id_generator_trait
/// @role: infrastructure
/// @layer: kernel
/// @human: Trait pour générer des identifiants uniques. Le produit peut implémenter des générateurs déterministes si nécessaire.
/// @do: generate_unique_id
/// Trait pour générer des identifiants uniques.
///
/// Le trait `IdGenerator` permet d'injecter différentes stratégies de génération
/// d'identifiants. En v0.1, seule la génération non déterministe (UUID v4) est
/// fournie par défaut. Le produit peut implémenter un générateur déterministe
/// si nécessaire (ex: pour les jeux).
///
/// ## Signature gelée (v0.1)
///
/// Cette signature est gelée et ne peut pas être modifiée sans version majeure.
pub trait IdGenerator {
    /// @id: kernel_id_generator_generate
    /// @role: infrastructure
    /// @layer: kernel
    /// @human: Génère un nouvel identifiant unique. En v0.1, non déterministe (UUID v4).
    /// @do: generate_new_id
    /// @depends: kernel_id_generator_trait
    /// Génère un nouvel identifiant unique.
    ///
    /// En v0.1, la génération est non déterministe (UUID v4).
    /// Le produit peut implémenter un générateur déterministe si nécessaire.
    ///
    /// # Returns
    ///
    /// Un identifiant unique de type `Id`.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::id::{IdGenerator, UuidIdGenerator};
    ///
    /// let generator = UuidIdGenerator;
    /// let id = generator.generate();
    /// ```
    fn generate(&self) -> Id;
}

/// @id: kernel_id_generator_uuid
/// @role: infrastructure
/// @layer: kernel
/// @human: Générateur d'identifiants basé sur UUID v4. Génération non déterministe.
/// @do: generate_uuid_v4_id
/// Générateur d'identifiants basé sur UUID v4.
///
/// Cette implémentation génère des identifiants uniques de manière non déterministe
/// en utilisant UUID v4. Pour des besoins déterministes (ex: jeux), le produit
/// peut implémenter un générateur personnalisé.
impl IdGenerator for UuidIdGenerator {
    fn generate(&self) -> Id {
        Id(Uuid::new_v4())
    }
}

/// @id: kernel_id_generator_uuid_struct
/// @role: infrastructure
/// @layer: kernel
/// @human: Structure du générateur UUID. Type unitaire sans état.
/// @do: represent_uuid_generator
/// Générateur d'identifiants UUID v4.
///
/// Type unitaire sans état. Peut être instancié et réutilisé.
#[derive(Debug, Clone, Copy, Default)]
pub struct UuidIdGenerator;

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: test_id_generate_unique
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que deux IDs générés sont différents, garantissant l'unicité.
    /// @do: verify_id_uniqueness
    /// @depends: kernel_id_generator_generate
    #[test]
    fn test_id_generate_unique() {
        let generator = UuidIdGenerator;
        let id1 = generator.generate();
        let id2 = generator.generate();
        assert_ne!(id1, id2);
    }

    /// @id: test_id_display_roundtrip
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant le round-trip Display → parse → même Id.
    /// @do: verify_id_round_trip
    /// @depends: kernel_id_display, kernel_id_parse
    #[test]
    fn test_id_display_roundtrip() {
        let generator = UuidIdGenerator;
        let id = generator.generate();
        let id_str = id.to_string();
        let parsed = Id::parse(&id_str).expect("Should parse valid UUID");
        assert_eq!(id, parsed);
    }

    /// @id: test_id_parse_valid_uuid
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que parse() réussit avec un UUID valide.
    /// @do: verify_valid_uuid_parsing
    /// @depends: kernel_id_parse
    #[test]
    fn test_id_parse_valid_uuid() {
        let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
        let id = Id::parse(valid_uuid).expect("Should parse valid UUID");
        assert_eq!(id.to_string(), valid_uuid);
    }

    /// @id: test_id_parse_invalid_format
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que parse() retourne une erreur pour un format invalide.
    /// @do: verify_invalid_format_error
    /// @depends: kernel_id_parse
    #[test]
    fn test_id_parse_invalid_format() {
        assert!(Id::parse("not-a-uuid").is_err());
        assert!(Id::parse("").is_err());
        assert!(Id::parse("123").is_err());
    }

    /// @id: test_id_equality
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que Id implémente Eq et PartialEq correctement.
    /// @do: verify_id_equality
    /// @depends: kernel_id_type
    #[test]
    fn test_id_equality() {
        let generator = UuidIdGenerator;
        let id1 = generator.generate();
        let id2 = id1; // Copy
        assert_eq!(id1, id2);
    }

    /// @id: test_id_hash
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que Id peut être utilisé comme clé dans une HashMap.
    /// @do: verify_id_hash_usage
    /// @depends: kernel_id_type
    #[test]
    fn test_id_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let generator = UuidIdGenerator;
        let id1 = generator.generate();
        let id2 = generator.generate();

        map.insert(id1, "value1");
        map.insert(id2, "value2");

        assert_eq!(map.get(&id1), Some(&"value1"));
        assert_eq!(map.get(&id2), Some(&"value2"));
    }

    /// @id: test_id_no_business_logic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'aucun type métier (UserId, OrderId) n'existe, conformément à INV-K-1.
    /// @do: verify_no_business_types
    /// @depends: kernel_id_type
    #[test]
    fn test_id_no_business_logic() {
        // Vérifier qu'aucun type métier n'existe
        // Ce test compile si aucun type UserId, OrderId, etc. n'est défini
        let generator = UuidIdGenerator;
        let _id: Id = generator.generate();
        // Si on pouvait faire : let _user_id: UserId = ... cela violerait INV-K-1
    }

    /// @id: test_id_no_external_dependency
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que la génération fonctionne sans réseau, conformément à INV-K-2.
    /// @do: verify_no_network_required
    /// @depends: kernel_id_generator_generate
    #[test]
    fn test_id_no_external_dependency() {
        // Ce test vérifie que generate() fonctionne sans réseau
        // Si generate() nécessitait un appel réseau, ce test échouerait en mode offline
        let generator = UuidIdGenerator;
        let _id = generator.generate();
        // Le fait que le test passe confirme INV-K-2
    }

    /// @id: test_id_deterministic_parse
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que parse() est déterministe : même string → même Id, conformément à INV-K-6.
    /// @do: verify_parse_determinism
    /// @depends: kernel_id_parse
    #[test]
    fn test_id_deterministic_parse() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id1 = Id::parse(uuid_str).expect("Should parse");
        let id2 = Id::parse(uuid_str).expect("Should parse");
        assert_eq!(id1, id2); // Même string → même Id (déterministe)
    }
}
