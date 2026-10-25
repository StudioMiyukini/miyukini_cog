//! Reprise de l'ancienne base SQLite de la mémoire d'Alicia (feature `import-heritage`).
//!
//! Moteur commun `kindmother-heritage` : lecture seule du fichier hérité
//! (empreinte SHA-256 contrôlée avant et après), upserts typés sur la clé
//! primaire (identifiants conservés), lots atomiques, vérification ligne à
//! ligne, puis marqueur `<base>.importe-kindmother`. Relancer après succès :
//! rien à faire ; après un échec : reprise sans doublon. Le moteur ne modifie
//! jamais le fichier hérité ; le store, une fois l'import vérifié, réapplique
//! les pierres tombales puis **efface** l'ancienne base de façon sûre
//! ([`crate::copie_heritee`] ; conservation seulement explicite).
//!
//! Les index FTS5 hérités (`facts_fts`, `episodes_fts` et leurs tables
//! internes) ne sont pas importés : ce sont des index dérivés du contenu de
//! `facts` et `episodes`, recalculés à la lecture par le store.

// @id service.alicia.memory.heritage
// @role memory_legacy_import
// @layer 7
// @do import_legacy_alicia_memory_once
// @human Plan d'import de l'ancienne base SQLite de la mémoire d'Alicia vers KindMother.

use kindmother_heritage::{PlanImport, PlanTable};

use crate::kindmother::{BASE_KINDMOTHER, OPERATEUR_KINDMOTHER};
use crate::schema::{
    migrations, EPISODES, FACTS, FACT_TOMBSTONES, FACT_VERSIONS, LEARNED_SKILLS, PENDING, PERSONAS,
    PREFERENCES,
};

/// Index plein texte hérités, volontairement non importés (dérivés du contenu).
const IGNOREES: &[(&str, &str)] = &[
    ("facts_fts", "index FTS5 dérivé de facts.content"),
    ("facts_fts_config", "table interne FTS5"),
    ("facts_fts_content", "table interne FTS5"),
    ("facts_fts_data", "table interne FTS5"),
    ("facts_fts_docsize", "table interne FTS5"),
    ("facts_fts_idx", "table interne FTS5"),
    ("episodes_fts", "index FTS5 dérivé de episodes.user_phrase"),
    ("episodes_fts_config", "table interne FTS5"),
    ("episodes_fts_content", "table interne FTS5"),
    ("episodes_fts_data", "table interne FTS5"),
    ("episodes_fts_docsize", "table interne FTS5"),
    ("episodes_fts_idx", "table interne FTS5"),
];

/// Plan d'import de la mémoire d'Alicia (tables et colonnes identiques à l'ancienne base).
pub fn plan_import() -> PlanImport {
    PlanImport {
        service: "miyualicia-memory",
        operateur: OPERATEUR_KINDMOTHER,
        base: BASE_KINDMOTHER,
        migrations: migrations(),
        tables: vec![
            PlanTable::simple(PERSONAS, &["id"]),
            PlanTable::simple(FACTS, &["id"]),
            PlanTable::simple(FACT_VERSIONS, &["fact_id", "version"]),
            PlanTable::simple(FACT_TOMBSTONES, &["fact_id"]),
            PlanTable::simple(PREFERENCES, &["scope", "key"]),
            PlanTable::simple(EPISODES, &["id"]),
            PlanTable::simple(LEARNED_SKILLS, &["id"]),
            PlanTable::simple(PENDING, &["id"]),
        ],
        tables_ignorees: IGNOREES,
    }
}
