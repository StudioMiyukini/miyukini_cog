//! Garde SQL appliquée par SQLite lui-même (K-05).
//!
//! @id     kindmother_service_sql_guard
//! @do     appliquer_politique_sql_par_autorisateur
//! @role   securite
//! @layer  core
//! @human  kindmother-service — autorisateur SQLite (sqlite3_set_authorizer) et une instruction par requête.
//!
//! L'arbitrage ne lit plus le texte SQL : il délègue à SQLite, qui appelle
//! l'autorisateur pour **chaque action** découverte lors de la préparation
//! (y compris dans les CTE, sous-requêtes, vues et corps de déclencheurs).
//! Toute action non reconnue est refusée (fail-closed, C-12).
//!
//! Prochaine étape (lot C) : remplacer le SQL libre par des requêtes typées/nommées.

use rusqlite::hooks::{AuthAction, AuthContext, Authorization};
use rusqlite::{Batch, Connection, ErrorCode, Statement};

use crate::arbitration::{AccessMode, Permission};
use crate::errors::ServiceError;

/// Pragmas en lecture seule autorisés aux opérateurs (introspection du schéma).
const PRAGMA_ALLOW_LIST: &[&str] = &[
    "table_info",
    "table_xinfo",
    "index_list",
    "index_info",
    "index_xinfo",
    "foreign_key_list",
];

/// Fonctions SQL interdites (accès fichiers, chargement de code).
const FUNCTION_DENY_LIST: &[&str] = &[
    "load_extension",
    "fts3_tokenizer",
    "readfile",
    "writefile",
    "edit",
];

/// Préfixes de tables protégées (catalogue SQLite, tables système KindMother).
const PROTECTED_TABLE_PREFIXES: &[&str] = &["sqlite_", "_kindmother_"];

/// Politique appliquée à une instruction d'opérateur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqlPolicy {
    /// Écritures de données (INSERT, UPDATE, DELETE) permises
    pub can_write: bool,
    /// Définition de schéma (CREATE, DROP, ALTER, REINDEX, ANALYZE) permise
    pub can_ddl: bool,
}

impl SqlPolicy {
    /// Politique en lecture seule.
    pub const READ_ONLY: Self = Self {
        can_write: false,
        can_ddl: false,
    };

    /// Déduit la politique de la permission de l'opérateur et du mode d'accès demandé.
    pub fn for_permission(permission: &Permission, mode: AccessMode) -> Self {
        match (mode, permission) {
            (AccessMode::Read, _) | (AccessMode::Write, Permission::Read) => Self::READ_ONLY,
            (AccessMode::Write, Permission::ReadWrite) => Self {
                can_write: true,
                can_ddl: false,
            },
            (AccessMode::Write, Permission::Admin) => Self {
                can_write: true,
                can_ddl: true,
            },
        }
    }
}

fn is_protected_table(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    PROTECTED_TABLE_PREFIXES
        .iter()
        .any(|prefix| lower.starts_with(prefix))
}

fn is_schema_table(name: &str) -> bool {
    name.eq_ignore_ascii_case("sqlite_master") || name.eq_ignore_ascii_case("sqlite_schema")
}

fn is_main(database_name: Option<&str>) -> bool {
    database_name == Some("main")
}

fn allow_if(condition: bool) -> Authorization {
    if condition {
        Authorization::Allow
    } else {
        Authorization::Deny
    }
}

/// Décision de l'autorisateur pour une action SQLite.
pub fn decide(policy: SqlPolicy, ctx: &AuthContext<'_>) -> Authorization {
    let main = is_main(ctx.database_name);
    match ctx.action {
        AuthAction::Select | AuthAction::Recursive => Authorization::Allow,

        // `database_name` absent : lecture interne (ex. optimisation de `COUNT(*)`) ;
        // seules `main` et `temp` existent (ATTACH refusé, objets temporaires refusés).
        AuthAction::Read { table_name, .. } => allow_if(
            (main || ctx.database_name.is_none())
                && !table_name.to_ascii_lowercase().starts_with("_kindmother_"),
        ),

        // Le catalogue n'est modifié que par le DDL lui-même (CREATE/DROP/ALTER émettent
        // ces actions) ; une écriture directe reste refusée par SQLite sans
        // `writable_schema`, pragma lui-même refusé.
        AuthAction::Insert { table_name }
        | AuthAction::Delete { table_name }
        | AuthAction::Update { table_name, .. } => allow_if(
            main && if is_schema_table(table_name) {
                policy.can_ddl
            } else {
                policy.can_write && !is_protected_table(table_name)
            },
        ),

        AuthAction::CreateTable { table_name }
        | AuthAction::DropTable { table_name }
        | AuthAction::CreateIndex { table_name, .. }
        | AuthAction::DropIndex { table_name, .. }
        | AuthAction::CreateTrigger { table_name, .. }
        | AuthAction::DropTrigger { table_name, .. } => {
            allow_if(policy.can_ddl && main && !is_protected_table(table_name))
        }

        // Pour ALTER TABLE, SQLite passe la base dans l'action, pas dans le contexte.
        AuthAction::AlterTable {
            database_name,
            table_name,
        } => allow_if(
            policy.can_ddl && database_name == "main" && !is_protected_table(table_name),
        ),

        AuthAction::CreateView { view_name } | AuthAction::DropView { view_name } => {
            allow_if(policy.can_ddl && main && !is_protected_table(view_name))
        }

        AuthAction::Reindex { .. } | AuthAction::Analyze { .. } => {
            allow_if(policy.can_ddl && main)
        }

        AuthAction::Pragma { pragma_name, .. } => {
            let name = pragma_name.to_ascii_lowercase();
            allow_if(
                PRAGMA_ALLOW_LIST.contains(&name.as_str())
                    && (ctx.database_name.is_none() || main),
            )
        }

        AuthAction::Function { function_name } => {
            let name = function_name.to_ascii_lowercase();
            allow_if(!FUNCTION_DENY_LIST.contains(&name.as_str()))
        }

        // ATTACH/DETACH, objets temporaires (persistants sur la connexion partagée),
        // tables virtuelles, transactions et points de sauvegarde (gérés par le service),
        // et toute action inconnue : refus.
        _ => Authorization::Deny,
    }
}

/// Installe l'autorisateur sur une connexion et le retire à la destruction.
///
/// Doit vivre plus longtemps que toute instruction préparée sous sa politique
/// (SQLite rappelle l'autorisateur en cas de re-préparation pendant l'exécution).
pub struct AuthorizerGuard<'c> {
    conn: &'c Connection,
}

impl<'c> AuthorizerGuard<'c> {
    /// Installe la politique `policy` sur `conn`.
    pub fn install(conn: &'c Connection, policy: SqlPolicy) -> Self {
        conn.authorizer(Some(move |ctx: AuthContext<'_>| decide(policy, &ctx)));
        Self { conn }
    }
}

impl Drop for AuthorizerGuard<'_> {
    fn drop(&mut self) {
        self.conn
            .authorizer(None::<fn(AuthContext<'_>) -> Authorization>);
    }
}

/// Convertit une erreur de préparation en erreur de service (refus d'autorisation distingué).
pub fn map_prepare_error(err: &rusqlite::Error) -> ServiceError {
    if err.sqlite_error_code() == Some(ErrorCode::AuthorizationForStatementDenied) {
        ServiceError::PermissionDenied("SQL action not authorized".to_string())
    } else {
        ServiceError::Database(format!("Prepare failed: {err}"))
    }
}

/// Prépare exactement une instruction SQL ; refuse les textes vides ou multiples.
///
/// La suite éventuelle est préparée (jamais exécutée) pour détecter une seconde
/// instruction ; les commentaires et espaces finaux sont acceptés.
pub fn prepare_single<'c>(conn: &'c Connection, sql: &str) -> Result<Statement<'c>, ServiceError> {
    let mut batch = Batch::new(conn, sql);
    let first = batch
        .next()
        .map_err(|e| map_prepare_error(&e))?
        .ok_or_else(|| ServiceError::Validation("Empty SQL statement".to_string()))?;
    match batch.next() {
        Ok(None) => Ok(first),
        Ok(Some(_)) | Err(_) => Err(ServiceError::Validation(
            "Exactly one SQL statement per request".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RW: SqlPolicy = SqlPolicy {
        can_write: true,
        can_ddl: false,
    };
    const ADMIN: SqlPolicy = SqlPolicy {
        can_write: true,
        can_ddl: true,
    };

    fn base() -> Connection {
        let conn = Connection::open_in_memory().expect("mem");
        conn.execute_batch(
            "CREATE TABLE t (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT);
             INSERT INTO t (v) VALUES ('a');
             CREATE TABLE _kindmother_audit (id INTEGER PRIMARY KEY, x TEXT);",
        )
        .expect("schema");
        conn
    }

    /// Prépare et exécute `sql` sous `policy` ; `Ok` si l'instruction a pu s'exécuter.
    fn run(conn: &Connection, policy: SqlPolicy, sql: &str) -> Result<(), ServiceError> {
        let _guard = AuthorizerGuard::install(conn, policy);
        let mut stmt = prepare_single(conn, sql)?;
        let mut rows = stmt
            .raw_query();
        while rows
            .next()
            .map_err(|e| ServiceError::Database(e.to_string()))?
            .is_some()
        {}
        Ok(())
    }

    fn count(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM t", [], |r| r.get(0))
            .expect("count")
    }

    /// Corpus de contournements K-05 : tous refusés en lecture seule.
    const CORPUS: &[&str] = &[
        "ATTACH 'x.db' AS y",
        "attach database 'x.db' as y",
        "ATTACH  DATABASE  'x.db'  AS  y",
        "DETACH DATABASE main",
        "WITH x AS (SELECT 1) DELETE FROM t",
        "WITH x AS (SELECT 1) INSERT INTO t (v) SELECT 'b' FROM x",
        "WITH x AS (SELECT 1) UPDATE t SET v = 'z'",
        "PRAGMA writable_schema=1",
        "PRAGMA writable_schema = ON",
        "PRAGMA key = 'x'",
        "PRAGMA journal_mode = DELETE",
        "SELECT load_extension('x')",
        "SELECT LOAD_EXTENSION('x')",
        "SELECT 1; DROP TABLE t",
        "SELECT 1;DROP TABLE t",
        "SELECT 1 /* ; */ ; DELETE FROM t",
        "/* commentaire */ DELETE FROM t",
        "-- commentaire\nDELETE FROM t",
        "DELETE/**/FROM t",
        "REPLACE INTO t (id, v) VALUES (1, 'z')",
        "INSERT INTO t (v) VALUES ('b')",
        "DROP TABLE t",
        "CREATE TABLE u (x)",
        "ALTER TABLE t ADD COLUMN w",
        "CREATE TEMP TRIGGER tr AFTER INSERT ON t BEGIN DELETE FROM t; END",
        "CREATE TEMP TABLE tt (x)",
        "BEGIN",
        "COMMIT",
        "SAVEPOINT s",
        "VACUUM",
        "VACUUM INTO 'copie.db'",
        "SELECT * FROM temp.sqlite_master",
        "SELECT * FROM _kindmother_audit",
        "",
        "   -- vide",
    ];

    #[test]
    fn corpus_refuse_en_lecture_seule() {
        let conn = base();
        for sql in CORPUS {
            assert!(
                run(&conn, SqlPolicy::READ_ONLY, sql).is_err(),
                "doit être refusé en lecture seule : {sql:?}"
            );
        }
        assert_eq!(count(&conn), 1, "aucune écriture ne doit avoir eu lieu");
    }

    #[test]
    fn corpus_dangereux_refuse_meme_admin() {
        let conn = base();
        let dangereux = [
            "ATTACH 'x.db' AS y",
            "attach database 'x.db' as y",
            "PRAGMA writable_schema=1",
            "SELECT load_extension('x')",
            "SELECT 1; DROP TABLE t",
            "INSERT INTO t (v) VALUES ('b'); DROP TABLE t",
            "CREATE TEMP TRIGGER tr AFTER INSERT ON t BEGIN DELETE FROM t; END",
            "BEGIN",
            "VACUUM INTO 'copie.db'",
            "INSERT INTO _kindmother_audit (x) VALUES ('faux')",
            "DELETE FROM _kindmother_audit",
            "DROP TABLE _kindmother_audit",
            "UPDATE sqlite_master SET sql = ''",
        ];
        for sql in dangereux {
            assert!(
                run(&conn, ADMIN, sql).is_err(),
                "doit être refusé même pour Admin : {sql:?}"
            );
        }
        assert_eq!(count(&conn), 1);
    }

    #[test]
    fn lectures_legitimes_autorisees() {
        let conn = base();
        for sql in [
            "SELECT * FROM t",
            "SELECT v FROM t WHERE id = 1;",
            "SELECT v FROM t -- commentaire final",
            "WITH x AS (SELECT id FROM t) SELECT COUNT(*) FROM x",
            "WITH RECURSIVE c(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM c WHERE n < 3) SELECT n FROM c",
            "SELECT name FROM sqlite_master WHERE type = 'table'",
            "PRAGMA table_info(t)",
            "SELECT lower(v), length(v) FROM t",
        ] {
            assert!(
                run(&conn, SqlPolicy::READ_ONLY, sql).is_ok(),
                "lecture légitime refusée : {sql:?}"
            );
        }
    }

    #[test]
    fn ecriture_selon_permission() {
        let conn = base();
        // ReadWrite : données oui, schéma non
        assert!(run(&conn, RW, "INSERT INTO t (v) VALUES ('b')").is_ok());
        assert!(run(&conn, RW, "UPDATE t SET v = 'c' WHERE id = 2").is_ok());
        assert!(run(&conn, RW, "WITH x AS (SELECT 2 AS i) DELETE FROM t WHERE id IN (SELECT i FROM x)").is_ok());
        assert!(run(&conn, RW, "CREATE TABLE u (x)").is_err());
        assert!(run(&conn, RW, "DROP TABLE t").is_err());
        // Admin : schéma oui
        assert!(run(&conn, ADMIN, "CREATE TABLE IF NOT EXISTS u (x)").is_ok());
        assert!(run(&conn, ADMIN, "CREATE INDEX IF NOT EXISTS idx_u ON u (x)").is_ok());
        assert!(run(&conn, ADMIN, "ALTER TABLE u ADD COLUMN y").is_ok());
        assert!(run(&conn, ADMIN, "DROP TABLE u").is_ok());
        assert_eq!(count(&conn), 1);
    }

    #[test]
    fn autorisateur_retire_apres_garde() {
        let conn = base();
        {
            let _guard = AuthorizerGuard::install(&conn, SqlPolicy::READ_ONLY);
            assert!(conn.execute("INSERT INTO t (v) VALUES ('x')", []).is_err());
        }
        // Le service (sans autorisateur) peut écrire l'audit.
        assert!(conn
            .execute("INSERT INTO _kindmother_audit (x) VALUES ('ok')", [])
            .is_ok());
    }

    #[test]
    fn politique_selon_permission() {
        use crate::arbitration::{AccessMode, Permission};
        assert_eq!(
            SqlPolicy::for_permission(&Permission::Admin, AccessMode::Read),
            SqlPolicy::READ_ONLY
        );
        assert_eq!(
            SqlPolicy::for_permission(&Permission::Read, AccessMode::Write),
            SqlPolicy::READ_ONLY
        );
        assert_eq!(SqlPolicy::for_permission(&Permission::ReadWrite, AccessMode::Write), RW);
        assert_eq!(SqlPolicy::for_permission(&Permission::Admin, AccessMode::Write), ADMIN);
    }
}
