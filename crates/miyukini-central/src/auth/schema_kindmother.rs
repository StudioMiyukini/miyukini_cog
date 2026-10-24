//! @id     miyukini_central_schema_kindmother
//! @do     declarer_migrations_kindmother
//! @role   donnee
//! @layer  infra
//! @human  miyukini-central — schéma KindMother versionné, partagé par la persistance KindMother
//!         et par l'import de la base héritée (feature `import-heritage`).

//! Migrations versionnées de la base KindMother du service.
//!
//! Module séparé de la couche de persistance pour être compilé aussi avec la feature
//! `import-heritage`, y compris quand `legacy-sqlite` est activée par unification des
//! features d'un espace de travail (le chemin KindMother n'est alors pas compilé).

use kindmother::intent::SchemaMigration;

/// Schéma v1 : identique au schéma final du chemin legacy (`db.rs`, colonnes ajoutées
/// par `migrate_add_profile_columns` comprises).
///
/// Évolutions futures : **ajouter** une `SchemaMigration` de version suivante (v2, v3…),
/// ne jamais modifier une version publiée ni supprimer de table de données — le service
/// refuse toute migration destructive (aucun `DROP`, aucune disparition de table/colonne).
const SCHEMA_V1: &str = r"
CREATE TABLE IF NOT EXISTS central_profiles (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    pseudonyme TEXT,
    nom TEXT,
    prenom TEXT,
    date_naissance TEXT,
    telephone TEXT,
    numero_voie TEXT,
    rue TEXT,
    code_postal TEXT,
    ville TEXT,
    is_admin INTEGER NOT NULL DEFAULT 0,
    genre TEXT,
    statut_marital TEXT,
    partenaire_genre TEXT,
    partenaire_nom TEXT,
    enfants_nombre INTEGER,
    enfants_noms TEXT,
    profession TEXT,
    langue_maternelle TEXT
);
CREATE TABLE IF NOT EXISTS cog_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS profile_service_refs (
    profile_id TEXT NOT NULL,
    service_key TEXT NOT NULL,
    ref_id TEXT NOT NULL,
    PRIMARY KEY (profile_id, service_key),
    FOREIGN KEY (profile_id) REFERENCES central_profiles(id)
);
CREATE TABLE IF NOT EXISTS central_profile_saves (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    service_key TEXT NOT NULL,
    slot INTEGER NOT NULL DEFAULT 0,
    data BLOB NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(profile_id, service_key, slot),
    FOREIGN KEY (profile_id) REFERENCES central_profiles(id)
);
CREATE INDEX IF NOT EXISTS idx_central_profile_saves_profile_service
    ON central_profile_saves(profile_id, service_key);
CREATE TABLE IF NOT EXISTS connect_credentials (
    profile_id TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES central_profiles(id)
);
";

/// Migrations versionnées de la base `central` (contiguës depuis 1, ajout seulement).
pub(crate) fn migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "central auth : profils, cog_meta, références de service, sauvegardes, hash Connect",
        &[SCHEMA_V1],
    )]
}
