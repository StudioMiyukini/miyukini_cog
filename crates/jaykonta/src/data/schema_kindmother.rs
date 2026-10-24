//! @id     jaykonta_schema_kindmother
//! @do     declarer_migrations_kindmother
//! @role   donnee
//! @layer  infra
//! @human  jaykonta — schéma KindMother versionné, partagé par la persistance KindMother
//!         et par l'import de la base héritée (feature `import-heritage`).

//! Migrations versionnées de la base KindMother du service.
//!
//! Module séparé de la couche de persistance pour être compilé aussi avec la feature
//! `import-heritage`, y compris quand `legacy-sqlite` est activée par unification des
//! features d'un espace de travail (le chemin KindMother n'est alors pas compilé).

use kindmother::intent::SchemaMigration;

/// DDL de référence : le même fichier que le chemin hérité.
const SCHEMA_SQL: &str = include_str!("schema.sql");

/// Migrations versionnées de la base `jaykonta` côté KindMother.
///
/// v1 = `schema.sql` complet (tables, index, déclencheurs, vues), en un seul script
/// multi-instructions contrôlé par le service (politique non destructive).
///
/// **Évolutions** : ajouter `SchemaMigration::new(2, …)`, puis 3… en fin de liste.
/// Ne jamais modifier une version publiée, ne jamais supprimer de table de données
/// (le service refuse une suite non contiguë et toute disparition de table/colonne).
#[must_use]
pub fn schema_migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "JayKonta v1 : schéma complet Purse + Account (schema.sql)",
        &[SCHEMA_SQL],
    )]
}
