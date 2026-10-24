//! @id     jaykonta_data_migrations
//! @do     migrer_schema_jaykonta
//! @role   donnee
//! @layer  core
//! @human  jaykonta — migrations versionnées de la base locale (EX-08, C-41).

//! Migrations versionnées de la base locale JayKonta.
//!
//! Règles (EX-08 « DB principale versionnée, sauvegardes réversibles », C-41) :
//!
//! - **Jamais de suppression de table de données.** Les migrations sont des
//!   étapes `vN → vN+1` appliquées dans une transaction unique ; seules les vues
//!   et les déclencheurs (objets dérivés) peuvent être recréés.
//! - **Fail-closed** : une lecture de version impossible est une erreur, la base
//!   n'est pas touchée. Une base plus récente que le code est refusée.
//! - **Sauvegarde réversible** : avant toute migration d'une base existante, le
//!   fichier est copié à côté (`<nom>.v<N>-<horodatage>.bak`). Retour arrière =
//!   remettre cette copie en place avec l'ancien binaire.
//! - Après migration, le schéma est contrôlé contre `schema.sql` (tables et
//!   colonnes attendues) ; un écart annule la transaction.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, Transaction};

/// Version courante du schéma JayKonta (`PRAGMA user_version`).
pub const VERSION_SCHEMA: i32 = 2;

/// DDL de référence de la version courante (idempotent : `IF NOT EXISTS`).
const SCHEMA_SQL: &str = include_str!("schema.sql");

/// Vues et déclencheurs : objets dérivés, recréables sans perte de données.
const OBJETS_DERIVES: &str = "
    DROP VIEW    IF EXISTS v_movements_by_category;
    DROP VIEW    IF EXISTS v_invoice_summary;
    DROP VIEW    IF EXISTS v_quote_summary;
    DROP TRIGGER IF EXISTS trg_movements_updated;
    DROP TRIGGER IF EXISTS trg_budgets_updated;
    DROP TRIGGER IF EXISTS trg_goals_updated;
    DROP TRIGGER IF EXISTS trg_quotes_updated;
    DROP TRIGGER IF EXISTS trg_invoices_updated;
    DROP TRIGGER IF EXISTS trg_budget_spent_insert;
    DROP TRIGGER IF EXISTS trg_invoice_paid_insert;
    DROP TRIGGER IF EXISTS trg_recurring_updated;
";

/// Motif de refus ou d'échec d'une migration (une variante par cause, C-11).
#[derive(Debug, thiserror::Error)]
pub enum ErreurMigration {
    /// La version du schéma n'a pas pu être lue (base illisible, clé fausse…).
    #[error("lecture de la version du schéma impossible (base non modifiée) : {0}")]
    VersionIlisible(rusqlite::Error),
    /// Version négative : en-tête incohérent.
    #[error("version de schéma invalide : {0}")]
    VersionInvalide(i32),
    /// La base a été écrite par un code plus récent.
    #[error("base en version {trouvee}, plus récente que le code (version {supportee}) : refus")]
    BasePlusRecente {
        /// Version lue dans la base.
        trouvee: i32,
        /// Version la plus haute connue du code.
        supportee: i32,
    },
    /// Aucune étape de migration ne part de cette version.
    #[error("aucune migration depuis la version {0}")]
    EtapeManquante(i32),
    /// Copie de sauvegarde impossible : migration non tentée.
    #[error("sauvegarde avant migration impossible ({chemin}) : {source}")]
    Sauvegarde {
        /// Fichier de sauvegarde visé.
        chemin: PathBuf,
        /// Erreur d'origine.
        #[source]
        source: std::io::Error,
    },
    /// Après migration, le schéma ne correspond pas à la référence.
    #[error("schéma non conforme après migration : {0}")]
    SchemaNonConforme(String),
    /// Erreur SQLite pendant la migration (transaction annulée).
    #[error("erreur SQLite pendant la migration (annulée) : {0}")]
    Sqlite(#[from] rusqlite::Error),
}

/// Étape de migration `de → de + 1`.
struct Etape {
    /// Version de départ.
    de: i32,
    /// Description journalisée.
    description: &'static str,
    /// Application dans la transaction.
    appliquer: fn(&Transaction<'_>) -> Result<(), ErreurMigration>,
}

/// Étapes connues, dans l'ordre. Ajouter ici `vN → vN+1` à chaque évolution
/// du schéma (jamais de `DROP TABLE` sur une table de données).
const ETAPES: &[Etape] = &[Etape {
    de: 1,
    description: "v1 → v2 : tables manquantes (dont transactions récurrentes), vues et déclencheurs",
    appliquer: etape_v1_vers_v2,
}];

/// v1 → v2 : ajout additif des objets manquants. Les tables existantes et
/// leurs lignes sont conservées ; vues et déclencheurs sont recréés.
fn etape_v1_vers_v2(tx: &Transaction<'_>) -> Result<(), ErreurMigration> {
    tx.execute_batch(OBJETS_DERIVES)?;
    tx.execute_batch(SCHEMA_SQL)?;
    Ok(())
}

/// Résultat d'un passage de migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RapportMigration {
    /// Version lue avant migration.
    pub version_initiale: i32,
    /// Version après migration.
    pub version_finale: i32,
    /// Copie de sauvegarde créée, s'il y avait des données à protéger.
    pub sauvegarde: Option<PathBuf>,
}

/// Lit la version du schéma — toute erreur est remontée (fail-closed).
fn lire_version(conn: &Connection) -> Result<i32, ErreurMigration> {
    let v: i32 = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(ErreurMigration::VersionIlisible)?;
    if v < 0 {
        return Err(ErreurMigration::VersionInvalide(v));
    }
    Ok(v)
}

/// Nombre de tables utilisateur (hors objets internes SQLite).
fn nombre_tables(conn: &Connection) -> Result<i64, ErreurMigration> {
    conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
        [],
        |row| row.get(0),
    )
    .map_err(ErreurMigration::VersionIlisible)
}

/// Amène la base à [`VERSION_SCHEMA`].
///
/// - base vierge (version 0, aucune table) : création du schéma de référence ;
/// - base à jour : aucune écriture ;
/// - base ancienne : sauvegarde du fichier `chemin` (si fourni), puis étapes
///   `vN → vN+1` dans **une** transaction, contrôle du schéma, mise à jour de
///   `user_version`. Une base version 0 contenant des tables (antérieure au
///   versionnage) est traitée comme une v1.
///
/// # Errors
///
/// [`ErreurMigration`] : version illisible ou invalide, base plus récente,
/// étape manquante, sauvegarde impossible, schéma non conforme ou erreur
/// SQLite. En cas d'erreur, la transaction est annulée.
pub fn migrer(
    conn: &mut Connection,
    chemin: Option<&Path>,
) -> Result<RapportMigration, ErreurMigration> {
    let version_initiale = lire_version(conn)?;
    if version_initiale > VERSION_SCHEMA {
        return Err(ErreurMigration::BasePlusRecente {
            trouvee: version_initiale,
            supportee: VERSION_SCHEMA,
        });
    }
    if version_initiale == VERSION_SCHEMA {
        return Ok(RapportMigration {
            version_initiale,
            version_finale: VERSION_SCHEMA,
            sauvegarde: None,
        });
    }

    let vierge = version_initiale == 0 && nombre_tables(conn)? == 0;
    let sauvegarde = if vierge {
        None
    } else {
        chemin.map(|c| sauvegarder(c, version_initiale)).transpose()?
    };

    let tx = conn.transaction()?;
    if vierge {
        tx.execute_batch(SCHEMA_SQL)?;
    } else {
        let mut version = version_initiale.max(1);
        while version < VERSION_SCHEMA {
            let etape = ETAPES
                .iter()
                .find(|e| e.de == version)
                .ok_or(ErreurMigration::EtapeManquante(version))?;
            (etape.appliquer)(&tx)?;
            tracing_migration(etape.description);
            version += 1;
        }
    }
    verifier_schema(&tx)?;
    tx.pragma_update(None, "user_version", VERSION_SCHEMA)?;
    tx.commit()?;

    Ok(RapportMigration {
        version_initiale,
        version_finale: VERSION_SCHEMA,
        sauvegarde,
    })
}

/// Journal minimal (le crate n'embarque pas `tracing`).
fn tracing_migration(description: &str) {
    eprintln!("[jaykonta] migration appliquée : {description}");
}

/// Copie le fichier de base avant migration. Le nom porte la version de départ
/// et un horodatage ; un fichier existant n'est jamais écrasé.
fn sauvegarder(chemin: &Path, version: i32) -> Result<PathBuf, ErreurMigration> {
    let nom = chemin
        .file_name()
        .map_or_else(|| "jaykonta.db".to_string(), |n| n.to_string_lossy().into_owned());
    let horodatage = chrono::Utc::now().format("%Y%m%dT%H%M%S%.3fZ");
    let mut cible = chemin.with_file_name(format!("{nom}.v{version}-{horodatage}.bak"));
    let mut n = 1u32;
    while cible.exists() {
        cible = chemin.with_file_name(format!("{nom}.v{version}-{horodatage}-{n}.bak"));
        n += 1;
    }
    let erreur = |source| ErreurMigration::Sauvegarde { chemin: cible.clone(), source };
    std::fs::copy(chemin, &cible).map_err(erreur)?;
    std::fs::File::open(&cible)
        .and_then(|f| f.sync_all())
        .map_err(erreur)?;
    Ok(cible)
}

/// Tables et colonnes d'une connexion (hors objets internes SQLite).
fn tables_et_colonnes(conn: &Connection) -> Result<Vec<(String, BTreeSet<String>)>, ErreurMigration> {
    let tables: Vec<String> = {
        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'
             ORDER BY name",
        )?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        rows.collect::<Result<_, _>>()?
    };
    let mut out = Vec::with_capacity(tables.len());
    for t in tables {
        let mut stmt = conn.prepare("SELECT name FROM pragma_table_info(?1)")?;
        let cols = stmt
            .query_map([&t], |r| r.get::<_, String>(0))?
            .collect::<Result<BTreeSet<_>, _>>()?;
        out.push((t, cols));
    }
    Ok(out)
}

/// Contrôle que toutes les tables et colonnes de `schema.sql` existent.
/// Les tables ou colonnes en plus (héritées) sont tolérées et conservées.
fn verifier_schema(conn: &Connection) -> Result<(), ErreurMigration> {
    let reference = Connection::open_in_memory()?;
    reference.execute_batch(SCHEMA_SQL)?;
    let attendu = tables_et_colonnes(&reference)?;
    let present = tables_et_colonnes(conn)?;
    for (table, colonnes) in attendu {
        let Some((_, cols)) = present.iter().find(|(t, _)| *t == table) else {
            return Err(ErreurMigration::SchemaNonConforme(format!("table {table} absente")));
        };
        let manquantes: Vec<&String> = colonnes.difference(cols).collect();
        if !manquantes.is_empty() {
            return Err(ErreurMigration::SchemaNonConforme(format!(
                "table {table} : colonnes absentes {manquantes:?}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Répertoire temporaire unique (pas de dépendance `tempfile`).
    fn dossier() -> PathBuf {
        let d = std::env::temp_dir().join(format!("jaykonta-migr-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    /// Base v1 : schéma de référence sans `recurring_transactions`, avec données.
    fn creer_base_v1(chemin: &Path) {
        let c = Connection::open(chemin).unwrap();
        let i = SCHEMA_SQL.find("-- ─── Transactions recurrentes").unwrap();
        let j = SCHEMA_SQL.find("-- ─── Rappels JayKoa").unwrap();
        let (avant, apres) = (&SCHEMA_SQL[..i], &SCHEMA_SQL[j..]);
        let declencheur = "CREATE TRIGGER IF NOT EXISTS trg_recurring_updated
AFTER UPDATE ON recurring_transactions FOR EACH ROW
BEGIN
    UPDATE recurring_transactions SET updated_at = datetime('now') WHERE id = NEW.id;
END;";
        assert!(apres.contains(declencheur));
        let ddl = format!("{avant}{}", apres.replace(declencheur, ""));
        c.execute_batch(&ddl).unwrap();
        c.execute_batch(
            "INSERT INTO accounts (id, scope, user_id, label) VALUES ('a1', 'purse', 'u', 'Purse');
             INSERT INTO movements (id, account_id, scope, amount, movement_date)
                VALUES ('m1', 'a1', 'purse', -42.5, '2026-10-01');
             INSERT INTO audit (id, account_id, contract_id, actor_ref, operation, scope, object_ref, result)
                VALUES ('au1', 'a1', 'c', 'u', 'op', 'purse', 'm1', 'ok');
             PRAGMA user_version = 1;",
        )
        .unwrap();
    }

    #[test]
    fn base_vierge_creee_en_version_courante() {
        let mut c = Connection::open_in_memory().unwrap();
        let r = migrer(&mut c, None).unwrap();
        assert_eq!(r.version_initiale, 0);
        assert_eq!(r.version_finale, VERSION_SCHEMA);
        assert!(r.sauvegarde.is_none());
        assert_eq!(lire_version(&c).unwrap(), VERSION_SCHEMA);
        // Réouverture : aucune écriture, aucune sauvegarde.
        let r2 = migrer(&mut c, None).unwrap();
        assert_eq!(r2.version_initiale, VERSION_SCHEMA);
    }

    #[test]
    fn migration_v1_preserve_les_donnees_et_sauvegarde() {
        let d = dossier();
        let chemin = d.join("jaykonta.db");
        creer_base_v1(&chemin);
        let avant = std::fs::read(&chemin).unwrap();

        let mut c = Connection::open(&chemin).unwrap();
        let r = migrer(&mut c, Some(&chemin)).unwrap();
        assert_eq!(r.version_initiale, 1);
        assert_eq!(r.version_finale, VERSION_SCHEMA);

        // Données conservées.
        let montant: f64 = c
            .query_row("SELECT amount FROM movements WHERE id = 'm1'", [], |r| r.get(0))
            .unwrap();
        assert!((montant + 42.5).abs() < f64::EPSILON);
        let audits: i64 = c.query_row("SELECT COUNT(*) FROM audit", [], |r| r.get(0)).unwrap();
        assert_eq!(audits, 1);
        // Table ajoutée par l'étape.
        let rec: i64 = c
            .query_row("SELECT COUNT(*) FROM recurring_transactions", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rec, 0);
        assert_eq!(lire_version(&c).unwrap(), VERSION_SCHEMA);

        // Sauvegarde = copie exacte de la base v1.
        let bak = r.sauvegarde.expect("sauvegarde créée");
        assert!(bak.file_name().unwrap().to_string_lossy().contains(".v1-"));
        assert_eq!(std::fs::read(&bak).unwrap(), avant);
        let cb = Connection::open(&bak).unwrap();
        assert_eq!(lire_version(&cb).unwrap(), 1);

        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn base_legacy_v0_avec_tables_migree_sans_perte() {
        let d = dossier();
        let chemin = d.join("legacy.db");
        {
            let c = Connection::open(&chemin).unwrap();
            c.execute_batch(
                "CREATE TABLE audit_log (id TEXT PRIMARY KEY, payload TEXT);
                 INSERT INTO audit_log VALUES ('x', 'ancien');",
            )
            .unwrap();
        }
        let mut c = Connection::open(&chemin).unwrap();
        let r = migrer(&mut c, Some(&chemin)).unwrap();
        assert_eq!(r.version_initiale, 0);
        assert!(r.sauvegarde.is_some());
        let n: i64 = c.query_row("SELECT COUNT(*) FROM audit_log", [], |r| r.get(0)).unwrap();
        assert_eq!(n, 1, "table héritée conservée");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn version_illisible_erreur_et_base_intacte() {
        let d = dossier();
        let chemin = d.join("corrompue.db");
        let contenu: Vec<u8> = (0..8192u32).map(|i| (i * 31 % 251) as u8).collect();
        std::fs::write(&chemin, &contenu).unwrap();

        let mut c = Connection::open(&chemin).unwrap();
        let err = migrer(&mut c, Some(&chemin)).unwrap_err();
        assert!(matches!(err, ErreurMigration::VersionIlisible(_)), "{err}");
        drop(c);
        assert_eq!(std::fs::read(&chemin).unwrap(), contenu, "base non modifiée");
        let baks = std::fs::read_dir(&d).unwrap().count();
        assert_eq!(baks, 1, "aucune sauvegarde ni fichier annexe");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn base_plus_recente_refusee() {
        let d = dossier();
        let chemin = d.join("future.db");
        {
            let c = Connection::open(&chemin).unwrap();
            c.execute_batch("CREATE TABLE futur (x INTEGER); PRAGMA user_version = 99;")
                .unwrap();
        }
        let avant = std::fs::read(&chemin).unwrap();
        let mut c = Connection::open(&chemin).unwrap();
        let err = migrer(&mut c, Some(&chemin)).unwrap_err();
        assert!(matches!(
            err,
            ErreurMigration::BasePlusRecente { trouvee: 99, supportee: VERSION_SCHEMA }
        ));
        drop(c);
        assert_eq!(std::fs::read(&chemin).unwrap(), avant);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn schema_non_conforme_annule_la_transaction() {
        let d = dossier();
        let chemin = d.join("divergente.db");
        {
            // v1 dont la table `accounts` n'a pas la colonne `currency`.
            let c = Connection::open(&chemin).unwrap();
            c.execute_batch(
                "CREATE TABLE accounts (id TEXT PRIMARY KEY, scope TEXT, user_id TEXT, label TEXT,
                                        created_at TEXT, updated_at TEXT);
                 INSERT INTO accounts VALUES ('a', 'purse', 'u', 'l', '', '');
                 PRAGMA user_version = 1;",
            )
            .unwrap();
        }
        let mut c = Connection::open(&chemin).unwrap();
        let err = migrer(&mut c, Some(&chemin)).unwrap_err();
        assert!(matches!(err, ErreurMigration::SchemaNonConforme(_)), "{err}");
        // Transaction annulée : version et tables inchangées, données présentes.
        assert_eq!(lire_version(&c).unwrap(), 1);
        assert_eq!(nombre_tables(&c).unwrap(), 1);
        let n: i64 = c.query_row("SELECT COUNT(*) FROM accounts", [], |r| r.get(0)).unwrap();
        assert_eq!(n, 1);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[cfg(not(feature = "db-encryption"))]
    #[test]
    fn ouverture_jaykonta_db_migre_sans_perte() {
        let d = dossier();
        let chemin = d.join("jaykonta.db");
        creer_base_v1(&chemin);
        let db = crate::data::JayKontaDb::open(&chemin).unwrap();
        assert!((db.solde_purse().unwrap() + 42.5).abs() < f64::EPSILON);
        assert_eq!(db.audit_count().unwrap(), 1);
        drop(db);
        // Réouverture : déjà à jour, pas de nouvelle sauvegarde.
        let db = crate::data::JayKontaDb::open(&chemin).unwrap();
        assert_eq!(db.audit_count().unwrap(), 1);
        let baks = std::fs::read_dir(&d)
            .unwrap()
            .filter(|e| e.as_ref().unwrap().file_name().to_string_lossy().ends_with(".bak"))
            .count();
        assert_eq!(baks, 1);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[cfg(not(feature = "db-encryption"))]
    #[test]
    fn ouverture_jaykonta_db_refuse_base_illisible() {
        let d = dossier();
        let chemin = d.join("illisible.db");
        let contenu = vec![0xA5u8; 4096];
        std::fs::write(&chemin, &contenu).unwrap();
        assert!(crate::data::JayKontaDb::open(&chemin).is_err());
        assert_eq!(std::fs::read(&chemin).unwrap(), contenu);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn etapes_contigues_jusqu_a_la_version_courante() {
        for v in 1..VERSION_SCHEMA {
            assert!(ETAPES.iter().any(|e| e.de == v), "étape manquante depuis v{v}");
        }
    }
}
