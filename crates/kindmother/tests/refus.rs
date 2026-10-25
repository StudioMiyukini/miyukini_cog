//! @id     kindmother_tests_refus
//! @do     verifier_refus_intentions_et_pont
//! @role   test
//! @layer  core
//! @human  kindmother — chemins de refus : forme des intentions, cycle de vie, pont MiyuSQL non signé (C-62).

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use kindmother::intent::{MAX_DECISION_DIGEST_LEN, MAX_INTENT_COLUMNS};
use kindmother::{
    APIError, InstanceIdentity, InstanceType, IntentLifecycle, IntentState, LifecycleError,
    MemoryStorage, MiyuSQLBridgeImpl, MiyuSQLExecutionBridge, SqlValue, Storage, StorageError,
    SyncError, TypedSelect, TypedWriteIntent,
};
use miyusql::{
    GovernedContext, MiyuSQLError, QueryExecutor, QueryResult, SchemaExecutor, SchemaResult,
    TableMeta, TransactionExecutor,
};

// ─── Forme des intentions ──────────────────────────────────────────────────

fn refus(intent: &TypedWriteIntent, motif: &str) {
    let e = intent.check_shape().unwrap_err();
    assert!(e.contains(motif), "« {motif} » absent de « {e} »");
}

#[test]
fn intention_trop_large_refusee() {
    let mut i = TypedWriteIntent::insert("notes");
    for c in 0..=MAX_INTENT_COLUMNS {
        i = i.value(format!("c{c}"), 1u32);
    }
    refus(&i, "too many columns");
    // Exactement à la borne : admis.
    let mut i = TypedWriteIntent::insert("notes");
    for c in 0..MAX_INTENT_COLUMNS {
        i = i.value(format!("c{c}"), 1u32);
    }
    assert!(i.check_shape().is_ok());
}

#[test]
fn empreinte_de_decision_bornee() {
    let base = TypedWriteIntent::insert("notes").value("titre", "x");
    refus(&base.clone().decision(""), "invalid decision digest length");
    refus(
        &base
            .clone()
            .decision("a".repeat(MAX_DECISION_DIGEST_LEN + 1)),
        "invalid decision digest length",
    );
    assert!(base
        .decision("a".repeat(MAX_DECISION_DIGEST_LEN))
        .check_shape()
        .is_ok());
}

#[test]
fn operations_mal_formees_refusees() {
    refus(
        &TypedWriteIntent::insert("t")
            .value("a", 1u32)
            .key("id", 1u32),
        "insert accepts values only",
    );
    refus(
        &TypedWriteIntent::insert("t")
            .value("a", 1u32)
            .increment("n", 1u32),
        "insert accepts values only",
    );
    refus(
        &TypedWriteIntent::update("t").key("id", 1u32),
        "update requires values or increments",
    );
    refus(
        &TypedWriteIntent::delete("t")
            .key("id", 1u32)
            .value("a", 1u32),
        "delete accepts a key only",
    );
    refus(
        &TypedWriteIntent::delete("t")
            .key("id", 1u32)
            .increment("n", 1u32),
        "delete accepts a key only",
    );
    refus(
        &TypedWriteIntent::upsert("t").value("a", 1u32),
        "upsert requires the conflict key",
    );
    refus(
        &TypedWriteIntent::upsert("t")
            .key("id", 1u32)
            .increment("n", 1u32),
        "upsert does not accept increments",
    );
    // Colonne d'incrément ou de clé invalide : refusée comme une colonne de valeur.
    refus(
        &TypedWriteIntent::update("t")
            .key("id; --", 1u32)
            .value("a", 1u32),
        "invalid column name",
    );
    refus(
        &TypedWriteIntent::update("t")
            .key("id", 1u32)
            .increment("n n", 1u32),
        "invalid column name",
    );
}

#[test]
fn lecture_typee_refusee() {
    let e = TypedSelect::from("notes")
        .order_by("x) --", false)
        .check_shape()
        .unwrap_err();
    assert!(e.contains("invalid column name"));
    let e = TypedSelect::from("notes")
        .filter("1=1", 1u32)
        .check_shape()
        .unwrap_err();
    assert!(e.contains("invalid column name"));
    let e = TypedSelect::from("_kindmother_audit")
        .check_shape()
        .unwrap_err();
    assert!(e.contains("reserved table"));
    let large =
        TypedSelect::from("notes").columns((0..=MAX_INTENT_COLUMNS).map(|c| format!("c{c}")));
    assert_eq!(large.check_shape().unwrap_err(), "too many columns");
}

#[test]
fn empreinte_canonique_sans_collision_de_type() {
    let empreinte = |v: SqlValue| {
        TypedWriteIntent::insert("t")
            .value("a", v)
            .canonical_params()
    };
    let valeurs = [
        SqlValue::Null,
        SqlValue::Integer(1),
        SqlValue::Real(1.0),
        SqlValue::Text("a".into()),
        SqlValue::Blob(b"a".to_vec()),
        SqlValue::Real(-0.0),
        SqlValue::Real(0.0),
    ];
    let empreintes: std::collections::HashSet<Vec<u8>> =
        valeurs.iter().cloned().map(empreinte).collect();
    assert_eq!(empreintes.len(), valeurs.len());
    // Conversions : &String, i32, Option.
    let s = String::from("x");
    assert_eq!(SqlValue::from(&s), SqlValue::Text("x".into()));
    assert_eq!(SqlValue::from(-3i32), SqlValue::Integer(-3));
    assert_eq!(SqlValue::from(None::<i32>), SqlValue::Null);
}

// ─── Cycle de vie ──────────────────────────────────────────────────────────

#[test]
fn cycle_de_vie_refuse_les_sauts_et_retours() {
    let mut l = IntentLifecycle::default();
    assert_eq!(l.state(), IntentState::Creee);
    // Créée → Appliquée sans validation : refus, état inchangé.
    let e = l.transition(IntentState::Appliquee).unwrap_err();
    assert_eq!(
        e,
        LifecycleError {
            from: IntentState::Creee,
            to: IntentState::Appliquee
        }
    );
    assert_eq!(
        e.to_string(),
        "invalid intent transition creee -> appliquee"
    );
    assert_eq!(l.history(), &[IntentState::Creee]);
    assert!(l.transition(IntentState::Archivee).is_err());
    l.transition(IntentState::Validee).unwrap();
    l.transition(IntentState::Echouee).unwrap();
    assert!(l.transition(IntentState::Appliquee).is_err());
    l.transition(IntentState::Archivee).unwrap();
    // État final : plus aucune transition.
    for s in [
        IntentState::Creee,
        IntentState::Validee,
        IntentState::Refusee,
        IntentState::Appliquee,
        IntentState::Echouee,
        IntentState::Archivee,
    ] {
        assert!(l.transition(s).is_err(), "archivee -> {}", s.as_str());
    }
    assert_eq!(l.terminal(), Some(IntentState::Echouee));
    assert_eq!(
        l.history(),
        &[
            IntentState::Creee,
            IntentState::Validee,
            IntentState::Echouee,
            IntentState::Archivee
        ]
    );
}

// ─── Pont MiyuSQL : mandat non signé ⇒ refus, exécuteur jamais atteint ────

/// Exécuteur espion : compte les appels qui lui parviennent (et les refuse).
#[derive(Default)]
struct Espion {
    appels: AtomicUsize,
}

impl Espion {
    fn compter<T>(&self) -> Result<T, MiyuSQLError> {
        self.appels.fetch_add(1, Ordering::SeqCst);
        Err(MiyuSQLError::Execution("espion atteint".into()))
    }
}

impl QueryExecutor for Espion {
    fn execute(
        &self,
        _: &GovernedContext,
        _: &str,
        _: &[String],
    ) -> Result<QueryResult, MiyuSQLError> {
        self.compter()
    }
    fn prepare(&self, _: &GovernedContext, _: &str, _: &[String]) -> Result<(), MiyuSQLError> {
        self.compter()
    }
}

impl TransactionExecutor for Espion {
    fn begin(&self, _: &GovernedContext) -> Result<String, MiyuSQLError> {
        self.compter()
    }
    fn commit(&self, _: &GovernedContext, _: &str) -> Result<(), MiyuSQLError> {
        self.compter()
    }
    fn rollback(&self, _: &GovernedContext, _: &str) -> Result<(), MiyuSQLError> {
        self.compter()
    }
}

impl SchemaExecutor for Espion {
    fn read_tables(&self, _: &GovernedContext) -> Result<SchemaResult, MiyuSQLError> {
        self.compter()
    }
    fn read_table(&self, _: &GovernedContext, _: &str) -> Result<TableMeta, MiyuSQLError> {
        self.compter()
    }
}

fn non_signe<T: std::fmt::Debug>(r: Result<T, MiyuSQLError>) {
    assert!(matches!(r, Err(MiyuSQLError::MandatNonSigne)), "{r:?}");
}

#[test]
fn pont_miyusql_refuse_tout_mandat_non_signe() {
    let espion = Arc::new(Espion::default());
    let pont = MiyuSQLBridgeImpl::new(Arc::clone(&espion));
    let p = ["1".to_string()];
    non_signe(pont.execute_query("mandat", 3, "SELECT 1", &p));
    non_signe(pont.prepare_query("mandat", 3, "SELECT 1", &p));
    non_signe(pont.begin_transaction("mandat", 3));
    non_signe(pont.commit_transaction("mandat", 3, "tx"));
    non_signe(pont.rollback_transaction("mandat", 3, "tx"));
    non_signe(pont.read_schema("mandat", 3));
    non_signe(pont.read_table_schema("mandat", 3, "notes"));
    // Mandat vide : refus plus tôt encore.
    assert!(matches!(
        pont.execute_query(" ", 3, "SELECT 1", &p),
        Err(MiyuSQLError::NoMandate)
    ));
    assert_eq!(
        espion.appels.load(Ordering::SeqCst),
        0,
        "l'exécuteur a été atteint"
    );
}

// ─── Stockage et erreurs ───────────────────────────────────────────────────

#[test]
fn stockage_cloisonne_par_instance() {
    let mere = InstanceIdentity::new(InstanceType::Mother);
    let fille = InstanceIdentity::new(InstanceType::Daughter);
    let mut s = MemoryStorage::new();
    s.write(&mere, "e1", b"secret").unwrap();
    assert_eq!(s.read(&fille, "e1"), None);
    // Suppression depuis une instance sans données : refus, données de la mère intactes.
    assert_eq!(s.delete(&fille, "e1"), Err(StorageError::NotFound));
    assert_eq!(s.read(&mere, "e1").as_deref(), Some(&b"secret"[..]));
}

#[test]
fn messages_d_erreur() {
    assert_eq!(StorageError::NotFound.to_string(), "Entity not found");
    assert!(StorageError::Corruption.to_string().contains("corruption"));
    assert!(StorageError::Io("disque".into())
        .to_string()
        .contains("disque"));
    assert!(APIError::PermissionDenied("p".into())
        .to_string()
        .starts_with("Permission denied"));
    assert!(APIError::InvalidIntent("i".into())
        .to_string()
        .starts_with("Invalid intent"));
    assert_eq!(APIError::NotFound.to_string(), "Entity not found");
    assert!(SyncError::Conflict("c".into())
        .to_string()
        .starts_with("Sync conflict"));
    assert!(SyncError::InvalidDelta("d".into())
        .to_string()
        .starts_with("Invalid delta"));
    assert!(SyncError::Io("e".into())
        .to_string()
        .starts_with("IO error"));
}
