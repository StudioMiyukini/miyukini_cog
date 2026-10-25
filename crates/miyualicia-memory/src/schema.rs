//! Schéma de la mémoire d'Alicia dans KindMother.
//!
//! Mêmes tables et colonnes que l'ancienne base SQLite (import identifiants
//! conservés, `kindmother-heritage`), sans les index FTS5 (la recherche plein
//! texte est faite côté store, voir `recherche`), plus une table `meta` qui
//! porte l'étiquette locale `nationale`.
//!
//! Évolutions : ajouter une version (`SchemaMigration::new(2, …)`) à la suite
//! de [`migrations`] ; ne jamais modifier une version publiée ni supprimer une
//! table (KindMother applique une politique non destructive).

// @id service.alicia.memory.schema
// @role memory_schema
// @layer 7
// @do declare_memory_schema
// @human Schéma versionné de la base KindMother de la mémoire d'Alicia.

use kindmother_client::{SchemaMigration, SqlValue};

/// Tables de la mémoire.
pub(crate) const FACTS: &str = "facts";
pub(crate) const FACT_VERSIONS: &str = "fact_versions";
pub(crate) const FACT_TOMBSTONES: &str = "fact_tombstones";
pub(crate) const PREFERENCES: &str = "preferences";
pub(crate) const EPISODES: &str = "episodes";
pub(crate) const PERSONAS: &str = "personas";
pub(crate) const LEARNED_SKILLS: &str = "learned_skills";
pub(crate) const PENDING: &str = "pending_confirmations";
pub(crate) const META: &str = "meta";

/// Étiquette locale de la base (même convention que `maia-memoire`).
pub(crate) const ETIQUETTE: &str = "nationale";
/// Clé de l'étiquette dans `meta`.
pub(crate) const CLE_ETIQUETTE: &str = "etiquette";

const SCHEMA_V1: &str = r"
CREATE TABLE IF NOT EXISTS facts (
    id          TEXT PRIMARY KEY,
    persona     TEXT NOT NULL,
    subject     TEXT NOT NULL,
    content     TEXT NOT NULL,
    source      TEXT NOT NULL,
    confidence  REAL NOT NULL,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_facts_persona ON facts(persona);
CREATE INDEX IF NOT EXISTS idx_facts_subject ON facts(subject);
CREATE TABLE IF NOT EXISTS preferences (
    scope       TEXT NOT NULL,
    key         TEXT NOT NULL,
    value_json  TEXT NOT NULL,
    updated_at  TEXT NOT NULL,
    PRIMARY KEY (scope, key)
);
CREATE TABLE IF NOT EXISTS episodes (
    id           TEXT PRIMARY KEY,
    persona      TEXT NOT NULL,
    service_id   TEXT NOT NULL,
    intent       TEXT NOT NULL,
    user_phrase  TEXT NOT NULL,
    params_json  TEXT NOT NULL,
    outcome      TEXT NOT NULL,
    error        TEXT,
    created_at   TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_episodes_persona ON episodes(persona);
CREATE INDEX IF NOT EXISTS idx_episodes_service ON episodes(service_id);
CREATE INDEX IF NOT EXISTS idx_episodes_created ON episodes(created_at);
CREATE TABLE IF NOT EXISTS personas (
    id            TEXT PRIMARY KEY,
    display_name  TEXT NOT NULL,
    description   TEXT NOT NULL,
    is_default    INTEGER NOT NULL,
    created_at    TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS learned_skills (
    id              TEXT PRIMARY KEY,
    persona         TEXT NOT NULL,
    trigger_phrase  TEXT NOT NULL,
    qualified_id    TEXT NOT NULL,
    params_template TEXT NOT NULL,
    status          TEXT NOT NULL,
    created_at      TEXT NOT NULL,
    accepted_at     TEXT
);
CREATE INDEX IF NOT EXISTS idx_learned_persona ON learned_skills(persona);
CREATE INDEX IF NOT EXISTS idx_learned_status  ON learned_skills(status);
CREATE INDEX IF NOT EXISTS idx_learned_qid     ON learned_skills(qualified_id);
CREATE TABLE IF NOT EXISTS pending_confirmations (
    id            TEXT PRIMARY KEY,
    qualified_id  TEXT NOT NULL,
    params_json   TEXT NOT NULL,
    persona       TEXT NOT NULL,
    user_phrase   TEXT NOT NULL,
    prompt        TEXT NOT NULL,
    status        TEXT NOT NULL,
    created_at    TEXT NOT NULL,
    resolved_at   TEXT,
    claimed_at    INTEGER NOT NULL DEFAULT 0,
    reason        TEXT
);
CREATE INDEX IF NOT EXISTS idx_pending_status   ON pending_confirmations(status);
CREATE INDEX IF NOT EXISTS idx_pending_persona  ON pending_confirmations(persona);
CREATE INDEX IF NOT EXISTS idx_pending_created  ON pending_confirmations(created_at);
CREATE TABLE IF NOT EXISTS fact_versions (
    fact_id     TEXT NOT NULL,
    version     INTEGER NOT NULL CHECK (version >= 1),
    persona     TEXT NOT NULL,
    subject     TEXT NOT NULL,
    content     TEXT NOT NULL,
    source      TEXT NOT NULL,
    confidence  REAL NOT NULL,
    recorded_at TEXT NOT NULL,
    PRIMARY KEY (fact_id, version)
);
CREATE TABLE IF NOT EXISTS fact_tombstones (
    fact_id          TEXT PRIMARY KEY,
    forgotten_at     TEXT NOT NULL,
    forgotten_by     TEXT NOT NULL,
    reason           TEXT NOT NULL,
    versions_erased  INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS meta (
    cle    TEXT PRIMARY KEY,
    valeur TEXT NOT NULL
);
";

/// Suite des migrations (versions contiguës à partir de 1, ajout en fin seulement).
pub fn migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "mémoire d'Alicia v1 : faits versionnés, pierres tombales, préférences, épisodes, \
         personas, skills apprises, confirmations, étiquette",
        &[SCHEMA_V1],
    )]
}

/// Définition d'une table pour le stockage en mémoire vive : clé primaire et
/// colonnes (avec leur valeur par défaut à l'insertion).
pub(crate) struct DefTable {
    pub nom: &'static str,
    pub cle: &'static [&'static str],
    pub colonnes: &'static [(&'static str, Defaut)],
}

/// Valeur par défaut d'une colonne absente d'une insertion.
#[derive(Clone, Copy)]
pub(crate) enum Defaut {
    /// `NOT NULL` sans défaut : l'insertion doit fournir la valeur.
    Requise,
    /// Colonne nullable : `NULL`.
    Nulle,
    /// `DEFAULT <entier>`.
    Entier(i64),
}

impl Defaut {
    pub(crate) fn valeur(self) -> Option<SqlValue> {
        match self {
            Defaut::Requise => None,
            Defaut::Nulle => Some(SqlValue::Null),
            Defaut::Entier(i) => Some(SqlValue::Integer(i)),
        }
    }
}

use Defaut::{Entier, Nulle, Requise};

/// Tables du schéma v1 (miroir de [`SCHEMA_V1`] pour le stockage en mémoire vive).
pub(crate) const TABLES: &[DefTable] = &[
    DefTable {
        nom: FACTS,
        cle: &["id"],
        colonnes: &[
            ("id", Requise),
            ("persona", Requise),
            ("subject", Requise),
            ("content", Requise),
            ("source", Requise),
            ("confidence", Requise),
            ("created_at", Requise),
            ("updated_at", Requise),
        ],
    },
    DefTable {
        nom: PREFERENCES,
        cle: &["scope", "key"],
        colonnes: &[
            ("scope", Requise),
            ("key", Requise),
            ("value_json", Requise),
            ("updated_at", Requise),
        ],
    },
    DefTable {
        nom: EPISODES,
        cle: &["id"],
        colonnes: &[
            ("id", Requise),
            ("persona", Requise),
            ("service_id", Requise),
            ("intent", Requise),
            ("user_phrase", Requise),
            ("params_json", Requise),
            ("outcome", Requise),
            ("error", Nulle),
            ("created_at", Requise),
        ],
    },
    DefTable {
        nom: PERSONAS,
        cle: &["id"],
        colonnes: &[
            ("id", Requise),
            ("display_name", Requise),
            ("description", Requise),
            ("is_default", Requise),
            ("created_at", Requise),
        ],
    },
    DefTable {
        nom: LEARNED_SKILLS,
        cle: &["id"],
        colonnes: &[
            ("id", Requise),
            ("persona", Requise),
            ("trigger_phrase", Requise),
            ("qualified_id", Requise),
            ("params_template", Requise),
            ("status", Requise),
            ("created_at", Requise),
            ("accepted_at", Nulle),
        ],
    },
    DefTable {
        nom: PENDING,
        cle: &["id"],
        colonnes: &[
            ("id", Requise),
            ("qualified_id", Requise),
            ("params_json", Requise),
            ("persona", Requise),
            ("user_phrase", Requise),
            ("prompt", Requise),
            ("status", Requise),
            ("created_at", Requise),
            ("resolved_at", Nulle),
            ("claimed_at", Entier(0)),
            ("reason", Nulle),
        ],
    },
    DefTable {
        nom: FACT_VERSIONS,
        cle: &["fact_id", "version"],
        colonnes: &[
            ("fact_id", Requise),
            ("version", Requise),
            ("persona", Requise),
            ("subject", Requise),
            ("content", Requise),
            ("source", Requise),
            ("confidence", Requise),
            ("recorded_at", Requise),
        ],
    },
    DefTable {
        nom: FACT_TOMBSTONES,
        cle: &["fact_id"],
        colonnes: &[
            ("fact_id", Requise),
            ("forgotten_at", Requise),
            ("forgotten_by", Requise),
            ("reason", Requise),
            ("versions_erased", Requise),
        ],
    },
    DefTable {
        nom: META,
        cle: &["cle"],
        colonnes: &[("cle", Requise), ("valeur", Requise)],
    },
];
