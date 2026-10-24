//! Store SQLite + FTS5 pour la mémoire Alicia.
//!
//! Schéma P1 :
//! - `facts(id, persona, subject, content, source, confidence, created_at, updated_at)`
//! - `facts_fts(content)` — index FTS5 sur `facts.content`, lié par `rowid`
//! - `preferences(scope, key, value_json, updated_at)` — `PRIMARY KEY(scope,key)`
//! - `episodes(id, persona, service_id, intent, user_phrase, params_json, outcome, error, created_at)`
//! - `episodes_fts(user_phrase)` — index FTS5
//! - `personas(id, display_name, description, is_default, created_at)`
//!
//! Le store est volontairement **synchrone** (rusqlite). En P2 on l'enveloppera
//! côté `miyualicia` dans une tâche dédiée si besoin.

// @id service.alicia.memory.store
// @role memory_store
// @layer 7
// @do persist_alicia_memory
// @human Store SQLite local pour les faits, préférences, épisodes et personas d'Alicia.

use std::path::Path;

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension, Row};
use serde_json::Value;
use uuid::Uuid;

use crate::errors::MemoryError;
use crate::types::{
    Episode, EpisodeOutcome, Fact, FactTombstone, FactVersion, LearnedSkill, LearnedSkillStatus, PendingConfirmation,
    PendingExpiryReport, PendingStatus, Persona, Preference, PENDING_REASON_EXPIRED,
};

/// Store mémoire — wrapper rusqlite + helpers typés.
///
/// La connexion SQLite est encapsulée dans un `Mutex` interne pour garantir
/// que `Arc<MemoryStore>` soit `Send + Sync` — indispensable pour partager le
/// store entre handlers axum, tâches tokio, et le runtime Alicia.
pub struct MemoryStore {
    conn: std::sync::Mutex<Connection>,
}

impl MemoryStore {
    fn lock_conn(&self) -> Result<std::sync::MutexGuard<'_, Connection>, MemoryError> {
        // Le lock ne peut être empoisonné que si une autre tâche a paniqué en
        // tenant le mutex. Fail-closed (C-12) : on remonte une erreur typée
        // plutôt que de continuer avec un état potentiellement incohérent.
        self.conn.lock().map_err(|_| MemoryError::LockPoisoned)
    }

    /// Ouvre (ou crée) un store dans le fichier `path`. Crée le dossier parent
    /// si nécessaire.
    pub fn open(path: &Path) -> Result<Self, MemoryError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| MemoryError::Io {
                    path: parent.to_path_buf(),
                    source: e,
                })?;
            }
        }
        let conn = Connection::open(path)?;
        let store = Self {
            conn: std::sync::Mutex::new(conn),
        };
        store.init_schema()?;
        Ok(store)
    }

    /// Ouvre un store en mémoire (tests).
    pub fn open_in_memory() -> Result<Self, MemoryError> {
        let conn = Connection::open_in_memory()?;
        let store = Self {
            conn: std::sync::Mutex::new(conn),
        };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<(), MemoryError> {
        self.lock_conn()?.execute_batch(
            r"
            -- EX-52 : un oubli demandé efface réellement (pages réécrites à zéro).
            PRAGMA secure_delete = ON;

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

            CREATE VIRTUAL TABLE IF NOT EXISTS facts_fts
                USING fts5(content, content='facts', content_rowid='rowid');

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

            CREATE VIRTUAL TABLE IF NOT EXISTS episodes_fts
                USING fts5(user_phrase, content='episodes', content_rowid='rowid');

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

            -- EX-52 : historique append-only des faits (jamais réécrit).
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

            -- EX-52 / EX-56 : pierres tombales des oublis demandés (sans contenu).
            CREATE TABLE IF NOT EXISTS fact_tombstones (
                fact_id          TEXT PRIMARY KEY,
                forgotten_at     TEXT NOT NULL,
                forgotten_by     TEXT NOT NULL,
                reason           TEXT NOT NULL,
                versions_erased  INTEGER NOT NULL
            );
            ",
        )?;
        self.migrate_schema()?;
        self.migrate_fact_versions()?;
        Ok(())
    }

    /// Migration additive EX-52 : chaque fait existant sans historique reçoit
    /// une version 1 (son contenu courant). Les contenus écrasés par l'ancien
    /// `remember` (avant versionnage) sont perdus et ne peuvent être
    /// reconstitués. L'index FTS est reconstruit à cette occasion (l'ancien
    /// `remember` y insérait des doublons à chaque mise à jour).
    ///
    /// Idempotente ; réversibilité (C-41) : tables additives, ignorées par
    /// l'ancien code.
    fn migrate_fact_versions(&self) -> Result<(), MemoryError> {
        let mut conn = self.lock_conn()?;
        let tx = conn.transaction()?;
        let sans_historique: i64 = tx.query_row(
            "SELECT COUNT(*) FROM facts
             WHERE id NOT IN (SELECT fact_id FROM fact_versions)",
            [],
            |r| r.get(0),
        )?;
        if sans_historique > 0 {
            tx.execute(
                "INSERT INTO fact_versions
                     (fact_id, version, persona, subject, content, source, confidence, recorded_at)
                 SELECT id, 1, persona, subject, content, source, confidence, updated_at
                 FROM facts WHERE id NOT IN (SELECT fact_id FROM fact_versions)",
                [],
            )?;
            tx.execute("INSERT INTO facts_fts(facts_fts) VALUES('rebuild')", [])?;
            tracing::info!(
                faits = sans_historique,
                "migration mémoire : version initiale créée pour les faits existants"
            );
        }
        tx.commit()?;
        Ok(())
    }

    /// Migrations additives des bases créées avec un schéma antérieur.
    ///
    /// `pending_confirmations` : ajout de `claimed_at` (horodatage de
    /// réclamation, epoch ms, défaut `0`) et `reason` (motif de résolution
    /// automatique, `NULL` par défaut). `ALTER TABLE ... ADD COLUMN` ne touche
    /// pas les lignes existantes ; une ligne `executing` héritée garde
    /// `claimed_at = 0` (date de réclamation inconnue) et sera donc expirée
    /// en `Failed` au premier balayage — jamais ré-exécutée.
    ///
    /// Réversibilité (C-41) : colonnes purement additives, ignorées par
    /// l'ancien code ; retour arrière = `ALTER TABLE ... DROP COLUMN`.
    fn migrate_schema(&self) -> Result<(), MemoryError> {
        let conn = self.lock_conn()?;
        let colonnes: Vec<String> = {
            let mut stmt = conn.prepare("PRAGMA table_info(pending_confirmations)")?;
            let rows = stmt.query_map([], |r| r.get::<_, String>(1))?;
            rows.collect::<Result<Vec<_>, _>>()?
        };
        let a = |nom: &str| colonnes.iter().any(|c| c == nom);
        if !a("claimed_at") {
            conn.execute_batch(
                "ALTER TABLE pending_confirmations
                 ADD COLUMN claimed_at INTEGER NOT NULL DEFAULT 0;",
            )?;
            tracing::info!("migration mémoire : colonne pending_confirmations.claimed_at ajoutée");
        }
        if !a("reason") {
            conn.execute_batch("ALTER TABLE pending_confirmations ADD COLUMN reason TEXT;")?;
            tracing::info!("migration mémoire : colonne pending_confirmations.reason ajoutée");
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    // FACTS
    // ------------------------------------------------------------------

    /// Enregistre un fait, ou en crée une **nouvelle version** s'il existe
    /// déjà (EX-52). Identifiant stable par `id`.
    ///
    /// - La version précédente reste dans l'historique (`fact_history`) ;
    ///   aucune écriture destructive.
    /// - Réécrire un fait identique (persona, sujet, contenu, source,
    ///   confiance) ne crée pas de version (idempotent).
    /// - Un fait oublié (pierre tombale) ne peut pas être réécrit sous le même
    ///   identifiant : [`MemoryError::FactForgotten`].
    ///
    /// L'ensemble (version, vue courante, index FTS) est écrit dans une seule
    /// transaction.
    pub fn remember(&self, fact: &Fact) -> Result<(), MemoryError> {
        let id = fact.id.to_string();
        let mut conn = self.lock_conn()?;
        let tx = conn.transaction()?;

        let oublie = tx
            .query_row(
                "SELECT 1 FROM fact_tombstones WHERE fact_id = ?1",
                params![id],
                |_| Ok(()),
            )
            .optional()?
            .is_some();
        if oublie {
            return Err(MemoryError::FactForgotten(fact.id));
        }

        let actuel = tx
            .query_row(
                "SELECT rowid, persona, subject, content, source, confidence
                 FROM facts WHERE id = ?1",
                params![id],
                |r| {
                    Ok(EtatFait {
                        rowid: r.get(0)?,
                        persona: r.get(1)?,
                        subject: r.get(2)?,
                        content: r.get(3)?,
                        source: r.get(4)?,
                        confidence: r.get(5)?,
                    })
                },
            )
            .optional()?;

        if actuel.as_ref().is_some_and(|a| a.identique(fact)) {
            // Rien de nouveau : pas de version supplémentaire.
            return Ok(());
        }

        let version: i64 = tx.query_row(
            "SELECT COALESCE(MAX(version), 0) + 1 FROM fact_versions WHERE fact_id = ?1",
            params![id],
            |r| r.get(0),
        )?;
        tx.execute(
            "INSERT INTO fact_versions
                 (fact_id, version, persona, subject, content, source, confidence, recorded_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                version,
                fact.persona,
                fact.subject,
                fact.content,
                fact.source,
                fact.confidence,
                fact.updated_at.to_rfc3339(),
            ],
        )?;

        match actuel {
            Some(a) => {
                // Index FTS à contenu externe : retirer l'ancien contenu avant
                // de mettre à jour la vue courante.
                tx.execute(
                    "INSERT INTO facts_fts(facts_fts, rowid, content) VALUES('delete', ?1, ?2)",
                    params![a.rowid, a.content],
                )?;
                tx.execute(
                    "UPDATE facts SET persona = ?2, subject = ?3, content = ?4, source = ?5,
                                      confidence = ?6, updated_at = ?7
                     WHERE id = ?1",
                    params![
                        id,
                        fact.persona,
                        fact.subject,
                        fact.content,
                        fact.source,
                        fact.confidence,
                        fact.updated_at.to_rfc3339(),
                    ],
                )?;
                tx.execute(
                    "INSERT INTO facts_fts(rowid, content) VALUES (?1, ?2)",
                    params![a.rowid, fact.content],
                )?;
            }
            None => {
                tx.execute(
                    "INSERT INTO facts(id, persona, subject, content, source, confidence,
                                       created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        id,
                        fact.persona,
                        fact.subject,
                        fact.content,
                        fact.source,
                        fact.confidence,
                        fact.created_at.to_rfc3339(),
                        fact.updated_at.to_rfc3339(),
                    ],
                )?;
                tx.execute(
                    "INSERT INTO facts_fts(rowid, content)
                     SELECT rowid, content FROM facts WHERE id = ?1",
                    params![id],
                )?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// **Oubli demandé** d'un fait (EX-52, EX-56) : seule écriture destructive
    /// autorisée sur la mémoire.
    ///
    /// Efface réellement la vue courante, l'index FTS et **toutes** les
    /// versions, puis enregistre une pierre tombale sans contenu (qui, quand,
    /// pourquoi, nombre de versions effacées). `forgotten_by` et `reason` sont
    /// obligatoires (non vides, bornés).
    ///
    /// Renvoie le nombre de faits retirés de la vue courante (0 ou 1). Un fait
    /// inconnu ou déjà oublié renvoie 0 sans nouvelle pierre tombale.
    pub fn forget(&self, id: Uuid, forgotten_by: &str, reason: &str) -> Result<usize, MemoryError> {
        valider_champ_oubli("auteur", forgotten_by, OUBLI_AUTEUR_MAX)?;
        valider_champ_oubli("motif", reason, OUBLI_MOTIF_MAX)?;
        let id_s = id.to_string();
        let mut conn = self.lock_conn()?;
        let tx = conn.transaction()?;

        let actuel: Option<(i64, String)> = tx
            .query_row(
                "SELECT rowid, content FROM facts WHERE id = ?1",
                params![id_s],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let versions: i64 = tx.query_row(
            "SELECT COUNT(*) FROM fact_versions WHERE fact_id = ?1",
            params![id_s],
            |r| r.get(0),
        )?;
        if actuel.is_none() && versions == 0 {
            return Ok(0);
        }

        let mut retires = 0usize;
        if let Some((rowid, content)) = actuel {
            tx.execute(
                "INSERT INTO facts_fts(facts_fts, rowid, content) VALUES('delete', ?1, ?2)",
                params![rowid, content],
            )?;
            retires = tx.execute("DELETE FROM facts WHERE id = ?1", params![id_s])?;
        }
        tx.execute("DELETE FROM fact_versions WHERE fact_id = ?1", params![id_s])?;
        tx.execute(
            "INSERT INTO fact_tombstones(fact_id, forgotten_at, forgotten_by, reason, versions_erased)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(fact_id) DO NOTHING",
            params![
                id_s,
                Utc::now().to_rfc3339(),
                forgotten_by.trim(),
                reason.trim(),
                versions
            ],
        )?;
        tx.commit()?;
        tracing::info!(fait = %id, versions, "mémoire : fait oublié à la demande (pierre tombale)");
        Ok(retires)
    }

    /// Version courante d'un fait (None si inconnu ou oublié).
    pub fn fact(&self, id: Uuid) -> Result<Option<Fact>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, persona, subject, content, source, confidence, created_at, updated_at
             FROM facts WHERE id = ?1",
        )?;
        stmt.query_row(params![id.to_string()], row_to_fact)
            .optional()
            .map_err(MemoryError::from)
    }

    /// Historique complet d'un fait, de la version 1 à la version courante
    /// (ordre croissant). Vide si inconnu ou oublié.
    pub fn fact_history(&self, id: Uuid) -> Result<Vec<FactVersion>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT fact_id, version, persona, subject, content, source, confidence, recorded_at
             FROM fact_versions WHERE fact_id = ?1 ORDER BY version ASC",
        )?;
        let rows = stmt.query_map(params![id.to_string()], row_to_fact_version)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(MemoryError::from)
    }

    /// Pierre tombale d'un fait oublié (None si jamais oublié).
    pub fn fact_tombstone(&self, id: Uuid) -> Result<Option<FactTombstone>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT fact_id, forgotten_at, forgotten_by, reason, versions_erased
             FROM fact_tombstones WHERE fact_id = ?1",
        )?;
        stmt.query_row(params![id.to_string()], row_to_tombstone)
            .optional()
            .map_err(MemoryError::from)
    }

    /// Liste les faits d'un persona (triés par `updated_at` décroissant).
    pub fn facts_for_persona(&self, persona: &str) -> Result<Vec<Fact>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, persona, subject, content, source, confidence, created_at, updated_at
             FROM facts WHERE persona = ?1 ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map(params![persona], row_to_fact)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Recherche plein texte sur les faits — retourne au plus `limit` résultats.
    pub fn recall(&self, query: &str, limit: usize) -> Result<Vec<Fact>, MemoryError> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT f.id, f.persona, f.subject, f.content, f.source, f.confidence,
                   f.created_at, f.updated_at
            FROM facts_fts ft
            JOIN facts f ON f.rowid = ft.rowid
            WHERE facts_fts MATCH ?1
            ORDER BY rank
            LIMIT ?2
            "#,
        )?;
        let rows = stmt.query_map(params![query, limit as i64], row_to_fact)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // ------------------------------------------------------------------
    // PREFERENCES
    // ------------------------------------------------------------------

    /// Définit (ou met à jour) une préférence.
    pub fn prefer(&self, pref: &Preference) -> Result<(), MemoryError> {
        let value_json = serde_json::to_string(&pref.value)?;
        self.lock_conn()?.execute(
            r#"
            INSERT INTO preferences(scope, key, value_json, updated_at)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(scope, key) DO UPDATE SET
                value_json = excluded.value_json,
                updated_at = excluded.updated_at
            "#,
            params![pref.scope, pref.key, value_json, pref.updated_at.to_rfc3339()],
        )?;
        Ok(())
    }

    /// Récupère une préférence (None si absente).
    pub fn preference(&self, scope: &str, key: &str) -> Result<Option<Preference>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT scope, key, value_json, updated_at FROM preferences
             WHERE scope = ?1 AND key = ?2",
        )?;
        let opt = stmt
            .query_row(params![scope, key], |row| {
                let scope: String = row.get(0)?;
                let key: String = row.get(1)?;
                let value_json: String = row.get(2)?;
                let updated_at: String = row.get(3)?;
                Ok((scope, key, value_json, updated_at))
            })
            .optional()?;
        match opt {
            None => Ok(None),
            Some((scope, key, value_json, updated_at)) => {
                let value: Value = serde_json::from_str(&value_json)?;
                let updated_at = parse_rfc3339(&updated_at)?;
                Ok(Some(Preference {
                    scope,
                    key,
                    value,
                    updated_at,
                }))
            }
        }
    }

    /// Liste toutes les préférences d'un scope (persona ou `"global"`).
    pub fn preferences_for_scope(&self, scope: &str) -> Result<Vec<Preference>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT scope, key, value_json, updated_at FROM preferences
             WHERE scope = ?1 ORDER BY key",
        )?;
        let rows = stmt.query_map(params![scope], |row| {
            let scope: String = row.get(0)?;
            let key: String = row.get(1)?;
            let value_json: String = row.get(2)?;
            let updated_at: String = row.get(3)?;
            Ok((scope, key, value_json, updated_at))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (scope, key, value_json, updated_at) = row?;
            let value: Value = serde_json::from_str(&value_json)?;
            let updated_at = parse_rfc3339(&updated_at)?;
            out.push(Preference {
                scope,
                key,
                value,
                updated_at,
            });
        }
        Ok(out)
    }

    /// Supprime une préférence. Renvoie le nombre de lignes supprimées.
    pub fn unprefer(&self, scope: &str, key: &str) -> Result<usize, MemoryError> {
        let n = self.lock_conn()?.execute(
            "DELETE FROM preferences WHERE scope = ?1 AND key = ?2",
            params![scope, key],
        )?;
        Ok(n)
    }

    // ------------------------------------------------------------------
    // EPISODES
    // ------------------------------------------------------------------

    /// Enregistre un épisode (insertion uniquement — pas d'upsert).
    pub fn record_episode(&self, ep: &Episode) -> Result<(), MemoryError> {
        let params_json = serde_json::to_string(&ep.params)?;
        let outcome = match ep.outcome {
            EpisodeOutcome::Ok => "ok",
            EpisodeOutcome::Cancelled => "cancelled",
            EpisodeOutcome::Failed => "failed",
        };
        self.lock_conn()?.execute(
            r#"
            INSERT INTO episodes(id, persona, service_id, intent, user_phrase, params_json,
                                 outcome, error, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                ep.id.to_string(),
                ep.persona,
                ep.service_id,
                ep.intent,
                ep.user_phrase,
                params_json,
                outcome,
                ep.error,
                ep.created_at.to_rfc3339(),
            ],
        )?;
        self.lock_conn()?.execute(
            "INSERT INTO episodes_fts(rowid, user_phrase) SELECT rowid, user_phrase FROM episodes WHERE id = ?1",
            params![ep.id.to_string()],
        )?;
        Ok(())
    }

    /// Récupère les `limit` derniers épisodes (tous personas confondus).
    pub fn recent_episodes(&self, limit: usize) -> Result<Vec<Episode>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, persona, service_id, intent, user_phrase, params_json,
                    outcome, error, created_at
             FROM episodes ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], row_to_episode)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    /// Épisodes récents restreints à une liste de personas (filtre en SQL,
    /// avant le `LIMIT`). Liste vide → aucun épisode. Au plus 64 personas.
    pub fn recent_episodes_for_personas(
        &self,
        personas: &[String],
        limit: usize,
    ) -> Result<Vec<Episode>, MemoryError> {
        if personas.is_empty() {
            return Ok(Vec::new());
        }
        let personas = &personas[..personas.len().min(64)];
        let marques = vec!["?"; personas.len()].join(", ");
        let sql = format!(
            "SELECT id, persona, service_id, intent, user_phrase, params_json,
                    outcome, error, created_at
             FROM episodes WHERE persona IN ({marques})
             ORDER BY created_at DESC LIMIT ?{}",
            personas.len() + 1
        );
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(&sql)?;
        let mut valeurs: Vec<&dyn rusqlite::ToSql> =
            personas.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
        let limite = i64::try_from(limit).unwrap_or(i64::MAX);
        valeurs.push(&limite);
        let rows = stmt.query_map(valeurs.as_slice(), row_to_episode)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    /// Liste les épisodes d'un service donné, plus récents d'abord.
    pub fn episodes_for_service(
        &self,
        service_id: &str,
        limit: usize,
    ) -> Result<Vec<Episode>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, persona, service_id, intent, user_phrase, params_json,
                    outcome, error, created_at
             FROM episodes WHERE service_id = ?1
             ORDER BY created_at DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![service_id, limit as i64], row_to_episode)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    /// Recherche plein texte sur les phrases utilisateur des épisodes.
    pub fn search_episodes(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Episode>, MemoryError> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT e.id, e.persona, e.service_id, e.intent, e.user_phrase, e.params_json,
                   e.outcome, e.error, e.created_at
            FROM episodes_fts ft
            JOIN episodes e ON e.rowid = ft.rowid
            WHERE episodes_fts MATCH ?1
            ORDER BY rank
            LIMIT ?2
            "#,
        )?;
        let rows = stmt.query_map(params![query, limit as i64], row_to_episode)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    // ------------------------------------------------------------------
    // PERSONAS
    // ------------------------------------------------------------------

    /// Crée ou met à jour un persona.
    pub fn upsert_persona(&self, persona: &Persona) -> Result<(), MemoryError> {
        if persona.id.trim().is_empty() {
            return Err(MemoryError::InvalidId(persona.id.clone()));
        }
        self.lock_conn()?.execute(
            r#"
            INSERT INTO personas(id, display_name, description, is_default, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                display_name = excluded.display_name,
                description  = excluded.description,
                is_default   = excluded.is_default
            "#,
            params![
                persona.id,
                persona.display_name,
                persona.description,
                i32::from(persona.is_default),
                persona.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Récupère un persona par son `id`.
    pub fn persona(&self, id: &str) -> Result<Option<Persona>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, display_name, description, is_default, created_at
             FROM personas WHERE id = ?1",
        )?;
        stmt.query_row(params![id], row_to_persona)
            .optional()
            .map_err(MemoryError::from)
    }

    /// Liste tous les personas (triés par `id`).
    pub fn personas(&self) -> Result<Vec<Persona>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, display_name, description, is_default, created_at
             FROM personas ORDER BY id",
        )?;
        let rows = stmt.query_map([], row_to_persona)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    /// Persona marqué `is_default = 1` (s'il y en a un).
    pub fn default_persona(&self) -> Result<Option<Persona>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, display_name, description, is_default, created_at
             FROM personas WHERE is_default = 1 LIMIT 1",
        )?;
        stmt.query_row([], row_to_persona)
            .optional()
            .map_err(MemoryError::from)
    }

    // ------------------------------------------------------------------
    // PERSONA ACTIVE (runtime)
    // ------------------------------------------------------------------

    /// Identifiant de la persona active.
    ///
    /// Résolution :
    /// 1. `Preference` scope=`global`, key=`active_persona` (string)
    /// 2. sinon le `default_persona()`
    /// 3. sinon `None`
    pub fn active_persona_id(&self) -> Result<Option<String>, MemoryError> {
        if let Some(pref) = self.preference("global", "active_persona")? {
            if let Value::String(s) = &pref.value {
                if !s.trim().is_empty() {
                    return Ok(Some(s.clone()));
                }
            }
        }
        Ok(self.default_persona()?.map(|p| p.id))
    }

    /// Pose la persona active. Vérifie qu'elle existe — sinon `UnknownPersona`.
    pub fn set_active_persona(&self, persona_id: &str) -> Result<(), MemoryError> {
        if persona_id.trim().is_empty() {
            return Err(MemoryError::InvalidId(persona_id.into()));
        }
        if self.persona(persona_id)?.is_none() {
            return Err(MemoryError::UnknownPersona(persona_id.into()));
        }
        self.prefer(&Preference::new(
            "global".into(),
            "active_persona".into(),
            Value::String(persona_id.into()),
        ))
    }

    // ------------------------------------------------------------------
    // PRÉFÉRENCE DE CONFIRMATION (par capability)
    // ------------------------------------------------------------------

    /// Lit la préférence de confirmation pour une capability.
    ///
    /// Convention de stockage :
    /// - clé : `confirm.<qualified_id>` (ex: `confirm.jaykoa.schedule_event`)
    /// - valeur JSON : `true` (toujours demander) / `false` (ne jamais demander)
    ///
    /// Résolution avec fallback :
    /// 1. scope = `persona`
    /// 2. scope = `global`
    /// 3. `None` (= aucune préférence posée → caller applique son défaut)
    pub fn confirmation_preference(
        &self,
        persona: &str,
        qualified_id: &str,
    ) -> Result<Option<bool>, MemoryError> {
        let key = format!("confirm.{qualified_id}");
        for scope in [persona, "global"] {
            if let Some(pref) = self.preference(scope, &key)? {
                if let Value::Bool(b) = pref.value {
                    return Ok(Some(b));
                }
                tracing::warn!(
                    persona = %persona,
                    qualified_id = %qualified_id,
                    scope = %scope,
                    "préférence confirmation non booléenne, ignorée"
                );
            }
        }
        Ok(None)
    }

    /// Pose la préférence de confirmation pour une capability sur un persona
    /// donné (ou `"global"`).
    pub fn set_confirmation_preference(
        &self,
        scope: &str,
        qualified_id: &str,
        ask: bool,
    ) -> Result<(), MemoryError> {
        if scope.trim().is_empty() || qualified_id.trim().is_empty() {
            return Err(MemoryError::InvalidId(format!(
                "scope='{scope}' qualified_id='{qualified_id}'"
            )));
        }
        self.prefer(&Preference::new(
            scope.into(),
            format!("confirm.{qualified_id}"),
            Value::Bool(ask),
        ))
    }

    // ------------------------------------------------------------------
    // SKILLS APPRISES (P5)
    // ------------------------------------------------------------------

    /// Insère ou met à jour une `LearnedSkill` (clé = `id`).
    pub fn upsert_learned_skill(&self, skill: &LearnedSkill) -> Result<(), MemoryError> {
        let params_json = serde_json::to_string(&skill.params_template)?;
        self.lock_conn()?.execute(
            r#"
            INSERT INTO learned_skills(id, persona, trigger_phrase, qualified_id,
                                       params_template, status, created_at, accepted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(id) DO UPDATE SET
                persona         = excluded.persona,
                trigger_phrase  = excluded.trigger_phrase,
                qualified_id    = excluded.qualified_id,
                params_template = excluded.params_template,
                status          = excluded.status,
                accepted_at     = excluded.accepted_at
            "#,
            params![
                skill.id.to_string(),
                skill.persona,
                skill.trigger_phrase,
                skill.qualified_id,
                params_json,
                learned_status_as_str(skill.status),
                skill.created_at.to_rfc3339(),
                skill.accepted_at.map(|d| d.to_rfc3339()),
            ],
        )?;
        Ok(())
    }

    /// Propose une nouvelle skill apprise (raccourci pour les détecteurs).
    pub fn propose_learned_skill(
        &self,
        persona: &str,
        trigger_phrase: &str,
        qualified_id: &str,
        params_template: Value,
    ) -> Result<LearnedSkill, MemoryError> {
        if persona.trim().is_empty()
            || trigger_phrase.trim().is_empty()
            || qualified_id.trim().is_empty()
        {
            return Err(MemoryError::InvalidId(format!(
                "persona='{persona}' qualified_id='{qualified_id}' trigger='{trigger_phrase}'"
            )));
        }
        let skill = LearnedSkill::proposed(
            persona.to_string(),
            trigger_phrase.to_string(),
            qualified_id.to_string(),
            params_template,
        );
        self.upsert_learned_skill(&skill)?;
        Ok(skill)
    }

    /// Marque une skill comme acceptée (statut `Accepted`, `accepted_at` posé).
    pub fn accept_learned_skill(&self, id: Uuid) -> Result<(), MemoryError> {
        let n = self.lock_conn()?.execute(
            "UPDATE learned_skills SET status = ?1, accepted_at = ?2 WHERE id = ?3",
            params![
                learned_status_as_str(LearnedSkillStatus::Accepted),
                Utc::now().to_rfc3339(),
                id.to_string(),
            ],
        )?;
        if n == 0 {
            return Err(MemoryError::LearnedSkillNotFound(id));
        }
        Ok(())
    }

    /// Marque une skill comme refusée (statut `Rejected`).
    pub fn reject_learned_skill(&self, id: Uuid) -> Result<(), MemoryError> {
        let n = self.lock_conn()?.execute(
            "UPDATE learned_skills SET status = ?1 WHERE id = ?2",
            params![
                learned_status_as_str(LearnedSkillStatus::Rejected),
                id.to_string(),
            ],
        )?;
        if n == 0 {
            return Err(MemoryError::LearnedSkillNotFound(id));
        }
        Ok(())
    }

    /// Récupère une skill apprise par son id.
    pub fn learned_skill(&self, id: Uuid) -> Result<Option<LearnedSkill>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, persona, trigger_phrase, qualified_id, params_template,
                    status, created_at, accepted_at
             FROM learned_skills WHERE id = ?1",
        )?;
        stmt.query_row(params![id.to_string()], row_to_learned_skill)
            .optional()
            .map_err(MemoryError::from)
    }

    /// Liste les skills apprises **acceptées** pour une persona donnée
    /// (utilisable au runtime pour matcher des phrases).
    pub fn active_learned_skills(&self, persona: &str) -> Result<Vec<LearnedSkill>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, persona, trigger_phrase, qualified_id, params_template,
                    status, created_at, accepted_at
             FROM learned_skills
             WHERE persona = ?1 AND status = 'accepted'
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![persona], row_to_learned_skill)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    /// Liste les skills apprises **proposées** pour une persona — c'est ce
    /// qu'on pousse dans le dialogue de promotion.
    pub fn pending_learned_skills(
        &self,
        persona: &str,
    ) -> Result<Vec<LearnedSkill>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, persona, trigger_phrase, qualified_id, params_template,
                    status, created_at, accepted_at
             FROM learned_skills
             WHERE persona = ?1 AND status = 'proposed'
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![persona], row_to_learned_skill)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    /// `true` si une skill apprise existe déjà (proposée OU rejetée OU
    /// acceptée) pour ce `(persona, qualified_id)`. Sert au détecteur de
    /// patterns pour ne pas re-proposer en boucle.
    pub fn learned_skill_exists(
        &self,
        persona: &str,
        qualified_id: &str,
    ) -> Result<bool, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT 1 FROM learned_skills
             WHERE persona = ?1 AND qualified_id = ?2 LIMIT 1",
        )?;
        let exists = stmt
            .query_row(params![persona, qualified_id], |_| Ok(()))
            .optional()?
            .is_some();
        Ok(exists)
    }

    /// Cherche, parmi les `LearnedSkill` **acceptées** pour la persona, une
    /// dont le `trigger_phrase` matche la phrase utilisateur.
    ///
    /// Matching P5 (volontairement simple) :
    /// 1. Normalisation : lowercase + suppression de la ponctuation +
    ///    compactage des whitespaces
    /// 2. Match si phrase normalisée == trigger normalisé **OU** phrase
    ///    contient trigger (utile si l'utilisateur ajoute des mots autour)
    /// 3. En cas d'ambiguïté (plusieurs matches) : la skill **la plus
    ///    récemment créée** gagne — `active_learned_skills` renvoie déjà
    ///    triées DESC par `created_at`.
    ///
    /// Renvoie `None` si la phrase est vide après normalisation, ou si rien
    /// ne matche.
    pub fn match_learned_skill(
        &self,
        persona: &str,
        phrase: &str,
    ) -> Result<Option<LearnedSkill>, MemoryError> {
        let phrase_norm = normalize_phrase(phrase);
        if phrase_norm.is_empty() {
            return Ok(None);
        }
        for skill in self.active_learned_skills(persona)? {
            let trigger_norm = normalize_phrase(&skill.trigger_phrase);
            if trigger_norm.is_empty() {
                continue;
            }
            if phrase_norm == trigger_norm || phrase_norm.contains(&trigger_norm) {
                return Ok(Some(skill));
            }
        }
        Ok(None)
    }
}

/// Normalise une phrase pour le matching : lowercase, supprime la
/// ponctuation (remplacée par espace), compacte les whitespaces.
fn normalize_phrase(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c.to_lowercase().next().unwrap_or(c)
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// ----------------------------------------------------------------------
// PENDING CONFIRMATIONS (queue async)
// ----------------------------------------------------------------------

impl MemoryStore {
    /// Pose une nouvelle confirmation en attente dans la queue. Retourne la
    /// `PendingConfirmation` créée (statut `Pending`).
    pub fn enqueue_pending_confirmation(
        &self,
        qualified_id: &str,
        params: Value,
        persona: &str,
        user_phrase: &str,
        prompt: &str,
    ) -> Result<PendingConfirmation, MemoryError> {
        if qualified_id.trim().is_empty() || persona.trim().is_empty() {
            return Err(MemoryError::InvalidId(format!(
                "qualified_id='{qualified_id}' persona='{persona}'"
            )));
        }
        let pc = PendingConfirmation::new(
            qualified_id.to_string(),
            params,
            persona.to_string(),
            user_phrase.to_string(),
            prompt.to_string(),
        );
        let params_json = serde_json::to_string(&pc.params)?;
        self.lock_conn()?.execute(
            r#"
            INSERT INTO pending_confirmations(id, qualified_id, params_json, persona,
                                              user_phrase, prompt, status, created_at, resolved_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                pc.id.to_string(),
                pc.qualified_id,
                params_json,
                pc.persona,
                pc.user_phrase,
                pc.prompt,
                pending_status_as_str(pc.status),
                pc.created_at.to_rfc3339(),
                pc.resolved_at.map(|d| d.to_rfc3339()),
            ],
        )?;
        Ok(pc)
    }

    /// Liste toutes les confirmations actuellement `Pending` (toutes personas
    /// confondues, plus récentes d'abord).
    pub fn list_pending_confirmations(&self) -> Result<Vec<PendingConfirmation>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, qualified_id, params_json, persona, user_phrase, prompt,
                    status, created_at, resolved_at, reason
             FROM pending_confirmations
             WHERE status = 'pending'
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], row_to_pending)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    /// Liste les confirmations `Pending` filtrées par persona.
    pub fn pending_confirmations_for_persona(
        &self,
        persona: &str,
    ) -> Result<Vec<PendingConfirmation>, MemoryError> {
        let conn = self.lock_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, qualified_id, params_json, persona, user_phrase, prompt,
                    status, created_at, resolved_at, reason
             FROM pending_confirmations
             WHERE persona = ?1 AND status = 'pending'
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![persona], row_to_pending)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(MemoryError::from)
    }

    /// Récupère une confirmation par id, quel que soit son statut.
    pub fn pending_confirmation(
        &self,
        id: Uuid,
    ) -> Result<Option<PendingConfirmation>, MemoryError> {
        let conn = self.lock_conn()?;
        pending_by_id(&conn, id)
    }

    /// Marque une confirmation `Confirmed` (statut + resolved_at) directement
    /// depuis `Pending`, **sans exécution**. Refuse si la confirmation
    /// n'existe pas ou n'est plus `Pending`.
    ///
    /// Le chemin d'exécution réel doit passer par
    /// [`MemoryStore::claim_pending_confirmation`] puis
    /// [`MemoryStore::finalize_pending_confirmation`] (R-017).
    pub fn mark_pending_confirmed(&self, id: Uuid) -> Result<(), MemoryError> {
        self.transition_pending(id, PendingStatus::Pending, PendingStatus::Confirmed)
    }

    /// Marque une confirmation `Cancelled`. Transition atomique
    /// `Pending → Cancelled` : refuse si la confirmation n'existe pas ou n'est
    /// plus `Pending` (y compris si une exécution l'a déjà réclamée).
    pub fn mark_pending_cancelled(&self, id: Uuid) -> Result<(), MemoryError> {
        self.transition_pending(id, PendingStatus::Pending, PendingStatus::Cancelled)
    }

    /// Réclame **atomiquement** une confirmation en vue de son exécution
    /// (R-017) : `UPDATE ... SET status = 'executing' WHERE id = ? AND
    /// status = 'pending'`. Seul l'appelant qui fait passer exactement une
    /// ligne obtient l'enregistrement ; tout appel concurrent reçoit
    /// `PendingConfirmationAlreadyResolved` (ou `NotFound`).
    ///
    /// Retourne l'enregistrement stocké (statut `Executing`) : c'est lui, et
    /// lui seul, qui doit être exécuté.
    pub fn claim_pending_confirmation(
        &self,
        id: Uuid,
    ) -> Result<PendingConfirmation, MemoryError> {
        let conn = self.lock_conn()?;
        let n = conn.execute(
            "UPDATE pending_confirmations
             SET status = ?1, claimed_at = ?4
             WHERE id = ?2 AND status = ?3",
            params![
                pending_status_as_str(PendingStatus::Executing),
                id.to_string(),
                pending_status_as_str(PendingStatus::Pending),
                Utc::now().timestamp_millis(),
            ],
        )?;
        let current = pending_by_id(&conn, id)?;
        match (n, current) {
            (1, Some(pc)) => Ok(pc),
            (_, None) => Err(MemoryError::PendingConfirmationNotFound(id)),
            (_, Some(pc)) => Err(MemoryError::PendingConfirmationAlreadyResolved {
                id,
                status: pending_status_as_str(pc.status).into(),
            }),
        }
    }

    /// Finalise une confirmation réclamée par
    /// [`MemoryStore::claim_pending_confirmation`] : `Executing → Confirmed`
    /// si `executed`, sinon `Executing → Failed`. Refuse toute autre
    /// transition.
    pub fn finalize_pending_confirmation(
        &self,
        id: Uuid,
        executed: bool,
    ) -> Result<(), MemoryError> {
        let target = if executed {
            PendingStatus::Confirmed
        } else {
            PendingStatus::Failed
        };
        self.transition_pending(id, PendingStatus::Executing, target)
    }

    /// Balaye les confirmations périmées (horloge courante). Voir
    /// [`MemoryStore::expire_stale_pending_confirmations_at`].
    pub fn expire_stale_pending_confirmations(
        &self,
        executing_ttl: std::time::Duration,
        pending_ttl: std::time::Duration,
    ) -> Result<PendingExpiryReport, MemoryError> {
        self.expire_stale_pending_confirmations_at(Utc::now(), executing_ttl, pending_ttl)
    }

    /// Balaye les confirmations périmées par rapport à l'instant `now` :
    ///
    /// - `Executing` réclamée depuis au moins `executing_ttl` (arrêt brutal
    ///   pendant l'exécution) → `Failed`, motif `"expired"`. Jamais
    ///   ré-exécutée : l'issue réelle de l'exécution interrompue est
    ///   inconnue, une reprise automatique risquerait un double effet ;
    /// - `Pending` créée depuis au moins `pending_ttl` → `Cancelled`, motif
    ///   `"expired"` : une confirmation ancienne ne peut plus être confirmée.
    ///
    /// Chaque transition est une requête conditionnelle unique
    /// (`UPDATE ... WHERE status = ... AND <date> <= ?`) exécutée sous le
    /// verrou de connexion dans une transaction : aucune course possible avec
    /// une réclamation ou une finalisation concurrente (la condition sur le
    /// statut est réévaluée par SQLite).
    pub fn expire_stale_pending_confirmations_at(
        &self,
        now: DateTime<Utc>,
        executing_ttl: std::time::Duration,
        pending_ttl: std::time::Duration,
    ) -> Result<PendingExpiryReport, MemoryError> {
        let now_ms = now.timestamp_millis();
        let ttl_ms = |d: std::time::Duration| i64::try_from(d.as_millis()).unwrap_or(i64::MAX);
        let seuil_exec = now_ms.saturating_sub(ttl_ms(executing_ttl));
        let seuil_pending = now_ms.saturating_sub(ttl_ms(pending_ttl));
        let resolu = now.to_rfc3339();

        let mut conn = self.lock_conn()?;
        let tx = conn.transaction()?;
        let failed = expire_where(
            &tx,
            PendingStatus::Executing,
            PendingStatus::Failed,
            "claimed_at <= ?1",
            seuil_exec,
            &resolu,
        )?;
        // `created_at` est en RFC 3339 : conversion en epoch ms côté SQLite
        // (julianday gère fuseau et fraction de seconde).
        let cancelled = expire_where(
            &tx,
            PendingStatus::Pending,
            PendingStatus::Cancelled,
            "CAST((julianday(created_at) - 2440587.5) * 86400000.0 AS INTEGER) <= ?1",
            seuil_pending,
            &resolu,
        )?;
        tx.commit()?;
        Ok(PendingExpiryReport { failed, cancelled })
    }

    /// Transition atomique `from → to` (une seule requête conditionnelle,
    /// sous le verrou de connexion). Erreur typée si aucune ligne ne change.
    fn transition_pending(
        &self,
        id: Uuid,
        from: PendingStatus,
        to: PendingStatus,
    ) -> Result<(), MemoryError> {
        let conn = self.lock_conn()?;
        let n = conn.execute(
            "UPDATE pending_confirmations
             SET status = ?1, resolved_at = ?2
             WHERE id = ?3 AND status = ?4",
            params![
                pending_status_as_str(to),
                Utc::now().to_rfc3339(),
                id.to_string(),
                pending_status_as_str(from),
            ],
        )?;
        if n == 1 {
            return Ok(());
        }
        match pending_by_id(&conn, id)? {
            None => Err(MemoryError::PendingConfirmationNotFound(id)),
            Some(pc) => Err(MemoryError::PendingConfirmationAlreadyResolved {
                id,
                status: pending_status_as_str(pc.status).into(),
            }),
        }
    }
}

/// Fait expirer (`from → to`, motif `"expired"`) les lignes vérifiant
/// `condition` (paramètre `?1` = seuil epoch ms). Retourne les ids touchés.
///
/// `UPDATE ... RETURNING` : sélection et transition dans la même requête
/// conditionnelle (atomique).
fn expire_where(
    conn: &Connection,
    from: PendingStatus,
    to: PendingStatus,
    condition: &str,
    seuil_ms: i64,
    resolu: &str,
) -> Result<Vec<Uuid>, MemoryError> {
    let sql = format!(
        "UPDATE pending_confirmations
         SET status = ?2, resolved_at = ?3, reason = ?4
         WHERE status = ?5 AND {condition}
         RETURNING id"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(
        params![
            seuil_ms,
            pending_status_as_str(to),
            resolu,
            PENDING_REASON_EXPIRED,
            pending_status_as_str(from),
        ],
        |r| r.get::<_, String>(0),
    )?;
    let mut ids = Vec::new();
    for r in rows {
        let brut = r?;
        let id = Uuid::parse_str(&brut)
            .map_err(|e| MemoryError::InvalidId(format!("uuid invalide '{brut}' : {e}")))?;
        ids.push(id);
    }
    Ok(ids)
}

/// Lecture d'une confirmation par id sur une connexion déjà verrouillée.
fn pending_by_id(
    conn: &Connection,
    id: Uuid,
) -> Result<Option<PendingConfirmation>, MemoryError> {
    let mut stmt = conn.prepare(
        "SELECT id, qualified_id, params_json, persona, user_phrase, prompt,
                status, created_at, resolved_at, reason
         FROM pending_confirmations
         WHERE id = ?1",
    )?;
    stmt.query_row(params![id.to_string()], row_to_pending)
        .optional()
        .map_err(MemoryError::from)
}

fn pending_status_as_str(s: PendingStatus) -> &'static str {
    match s {
        PendingStatus::Pending => "pending",
        PendingStatus::Executing => "executing",
        PendingStatus::Confirmed => "confirmed",
        PendingStatus::Failed => "failed",
        PendingStatus::Cancelled => "cancelled",
    }
}

fn row_to_pending(row: &Row<'_>) -> rusqlite::Result<PendingConfirmation> {
    let id_str: String = row.get(0)?;
    let id = Uuid::parse_str(&id_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let params_json: String = row.get(2)?;
    let params: Value = serde_json::from_str(&params_json).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let status_str: String = row.get(6)?;
    let status = match status_str.as_str() {
        "pending" => PendingStatus::Pending,
        "executing" => PendingStatus::Executing,
        "confirmed" => PendingStatus::Confirmed,
        "failed" => PendingStatus::Failed,
        "cancelled" => PendingStatus::Cancelled,
        other => {
            return Err(rusqlite::Error::FromSqlConversionFailure(
                6,
                rusqlite::types::Type::Text,
                format!("statut PendingConfirmation inconnu : {other}").into(),
            ));
        }
    };
    let created_at: String = row.get(7)?;
    let resolved_at: Option<String> = row.get(8)?;
    let resolved_at = resolved_at.map(|s| rfc3339_or_err(&s, 8)).transpose()?;
    let reason: Option<String> = row.get(9)?;
    Ok(PendingConfirmation {
        id,
        qualified_id: row.get(1)?,
        params,
        persona: row.get(3)?,
        user_phrase: row.get(4)?,
        prompt: row.get(5)?,
        status,
        created_at: rfc3339_or_err(&created_at, 7)?,
        resolved_at,
        reason,
    })
}

fn learned_status_as_str(s: LearnedSkillStatus) -> &'static str {
    match s {
        LearnedSkillStatus::Proposed => "proposed",
        LearnedSkillStatus::Accepted => "accepted",
        LearnedSkillStatus::Rejected => "rejected",
    }
}

fn row_to_learned_skill(row: &Row<'_>) -> rusqlite::Result<LearnedSkill> {
    let id_str: String = row.get(0)?;
    let id = Uuid::parse_str(&id_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let params_json: String = row.get(4)?;
    let params_template: Value = serde_json::from_str(&params_json).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let status_str: String = row.get(5)?;
    let status = match status_str.as_str() {
        "proposed" => LearnedSkillStatus::Proposed,
        "accepted" => LearnedSkillStatus::Accepted,
        "rejected" => LearnedSkillStatus::Rejected,
        other => {
            return Err(rusqlite::Error::FromSqlConversionFailure(
                5,
                rusqlite::types::Type::Text,
                format!("statut LearnedSkill inconnu : {other}").into(),
            ));
        }
    };
    let created_at: String = row.get(6)?;
    let accepted_at: Option<String> = row.get(7)?;
    let accepted_at = accepted_at.map(|s| rfc3339_or_err(&s, 7)).transpose()?;
    Ok(LearnedSkill {
        id,
        persona: row.get(1)?,
        trigger_phrase: row.get(2)?,
        qualified_id: row.get(3)?,
        params_template,
        status,
        created_at: rfc3339_or_err(&created_at, 6)?,
        accepted_at,
    })
}

// ----------------------------------------------------------------------
// Versionnage des faits (EX-52)
// ----------------------------------------------------------------------

/// Longueur maximale de l'auteur d'un oubli (C-13).
const OUBLI_AUTEUR_MAX: usize = 128;
/// Longueur maximale du motif d'un oubli (C-13).
const OUBLI_MOTIF_MAX: usize = 512;

/// État courant d'un fait, lu avant une nouvelle écriture.
struct EtatFait {
    rowid: i64,
    persona: String,
    subject: String,
    content: String,
    source: String,
    confidence: f64,
}

impl EtatFait {
    /// Vrai si `fait` n'apporte aucun changement à l'état courant.
    fn identique(&self, fait: &Fact) -> bool {
        self.persona == fait.persona
            && self.subject == fait.subject
            && self.content == fait.content
            && self.source == fait.source
            && (self.confidence - f64::from(fait.confidence)).abs() < f64::EPSILON
    }
}

/// Valide un champ obligatoire d'une demande d'oubli (non vide, borné).
fn valider_champ_oubli(nom: &str, valeur: &str, max: usize) -> Result<(), MemoryError> {
    let v = valeur.trim();
    if v.is_empty() {
        return Err(MemoryError::InvalidForgetRequest(format!("{nom} manquant")));
    }
    if v.chars().count() > max {
        return Err(MemoryError::InvalidForgetRequest(format!(
            "{nom} trop long (max {max} caractères)"
        )));
    }
    Ok(())
}

fn uuid_col(row: &Row<'_>, idx: usize) -> rusqlite::Result<Uuid> {
    let s: String = row.get(idx)?;
    Uuid::parse_str(&s).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(idx, rusqlite::types::Type::Text, Box::new(e))
    })
}

fn u32_col(row: &Row<'_>, idx: usize) -> rusqlite::Result<u32> {
    let v: i64 = row.get(idx)?;
    u32::try_from(v).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(idx, rusqlite::types::Type::Integer, Box::new(e))
    })
}

fn row_to_fact_version(row: &Row<'_>) -> rusqlite::Result<FactVersion> {
    let recorded_at: String = row.get(7)?;
    Ok(FactVersion {
        fact_id: uuid_col(row, 0)?,
        version: u32_col(row, 1)?,
        persona: row.get(2)?,
        subject: row.get(3)?,
        content: row.get(4)?,
        source: row.get(5)?,
        confidence: row.get(6)?,
        recorded_at: rfc3339_or_err(&recorded_at, 7)?,
    })
}

fn row_to_tombstone(row: &Row<'_>) -> rusqlite::Result<FactTombstone> {
    let forgotten_at: String = row.get(1)?;
    Ok(FactTombstone {
        fact_id: uuid_col(row, 0)?,
        forgotten_at: rfc3339_or_err(&forgotten_at, 1)?,
        forgotten_by: row.get(2)?,
        reason: row.get(3)?,
        versions_erased: u32_col(row, 4)?,
    })
}

// ----------------------------------------------------------------------
// Row → struct helpers
// ----------------------------------------------------------------------

fn row_to_fact(row: &Row<'_>) -> rusqlite::Result<Fact> {
    let id_str: String = row.get(0)?;
    let id = Uuid::parse_str(&id_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let created_at: String = row.get(6)?;
    let updated_at: String = row.get(7)?;
    Ok(Fact {
        id,
        persona: row.get(1)?,
        subject: row.get(2)?,
        content: row.get(3)?,
        source: row.get(4)?,
        confidence: row.get(5)?,
        created_at: rfc3339_or_err(&created_at, 6)?,
        updated_at: rfc3339_or_err(&updated_at, 7)?,
    })
}

fn row_to_episode(row: &Row<'_>) -> rusqlite::Result<Episode> {
    let id_str: String = row.get(0)?;
    let id = Uuid::parse_str(&id_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let params_json: String = row.get(5)?;
    let params: Value = serde_json::from_str(&params_json).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let outcome_str: String = row.get(6)?;
    let outcome = match outcome_str.as_str() {
        "ok" => EpisodeOutcome::Ok,
        "cancelled" => EpisodeOutcome::Cancelled,
        "failed" => EpisodeOutcome::Failed,
        other => {
            return Err(rusqlite::Error::FromSqlConversionFailure(
                6,
                rusqlite::types::Type::Text,
                format!("outcome inconnu : {other}").into(),
            ));
        }
    };
    let created_at: String = row.get(8)?;
    Ok(Episode {
        id,
        persona: row.get(1)?,
        service_id: row.get(2)?,
        intent: row.get(3)?,
        user_phrase: row.get(4)?,
        params,
        outcome,
        error: row.get(7)?,
        created_at: rfc3339_or_err(&created_at, 8)?,
    })
}

fn row_to_persona(row: &Row<'_>) -> rusqlite::Result<Persona> {
    let created_at: String = row.get(4)?;
    let is_default: i32 = row.get(3)?;
    Ok(Persona {
        id: row.get(0)?,
        display_name: row.get(1)?,
        description: row.get(2)?,
        is_default: is_default != 0,
        created_at: rfc3339_or_err(&created_at, 4)?,
    })
}

fn parse_rfc3339(s: &str) -> Result<DateTime<Utc>, MemoryError> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| MemoryError::InvalidId(format!("date RFC3339 invalide '{s}' : {e}")))
}

fn rfc3339_or_err(s: &str, idx: usize) -> rusqlite::Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(idx, rusqlite::types::Type::Text, Box::new(e))
        })
}
