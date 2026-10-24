//! @id     jaykonta_tests_kindmother_only
//! @do     verifier_persistance_via_kindmother
//! @role   test
//! @layer  infra
//! @human  jaykonta — tests du chemin KindMother (seul écrivain, K-13) sur serveur éphémère.

//! Tests d'intégration du chemin `kindmother-only` (défaut) : un serveur KindMother
//! éphémère est démarré en processus, `JayKontaDb` s'y connecte par injection.

#![cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]

use jaykonta::backend::service::JayKontaBackend;
use jaykonta::data::{
    AuditRecord, InvoiceRecord, JayKontaDb, MovementRecord, PaymentRecord, QuoteRecord,
    ReminderRecord,
};
use jaykonta::domain::model::EntryPoint;
use jaykonta::domain::purse::{MovementFilters, MovementType};
use kindmother_client::OperatorCredential;
use std::path::PathBuf;
use std::sync::Arc;

/// Répertoire temporaire unique, supprimé en fin de test.
struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!("jaykonta-km-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("création du répertoire temporaire");
        Self(dir)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Serveur éphémère + adresse + justificatif `jaykonta`.
fn serveur() -> (TempDir, String, OperatorCredential) {
    let tmp = TempDir::new();
    let (addr, creds) =
        kindmother_service::spawn_ephemeral_blocking(&tmp.0).expect("serveur KindMother éphémère");
    let cred = OperatorCredential::load_from_dir("jaykonta", &creds).expect("justificatif jaykonta");
    (tmp, addr.to_string(), cred)
}

fn ouvrir() -> (TempDir, String, OperatorCredential, JayKontaDb) {
    let (tmp, addr, cred) = serveur();
    let db = JayKontaDb::connect_with(&addr, &cred).expect("connexion + migrations");
    (tmp, addr, cred, db)
}

fn mois_courant() -> String {
    chrono::Local::now().format("%Y-%m").to_string()
}

fn jour_courant() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

#[test]
fn schema_applique_et_reouverture_idempotente() {
    let (_tmp, addr, cred, db) = ouvrir();
    // Migrations déjà appliquées : v1 -> v1.
    assert_eq!(db.init_schema_sync().expect("idempotent"), (1, 1));
    assert!(db.is_bootstrap_needed().expect("lecture audit"));
    let account = db.ensure_account("purse", "u1", "Budget").expect("compte");

    // Réouverture : aucune perte, même compte.
    let db2 = JayKontaDb::connect_with(&addr, &cred).expect("réouverture");
    assert_eq!(db2.ensure_account("purse", "u1", "Budget").expect("compte"), account);
    assert!(db2.health_check().expect("santé"));
    assert_eq!(JayKontaDb::SCHEMA_VERSION, 1);
}

#[test]
fn ecritures_purse_et_agregats() {
    let (_tmp, _addr, _cred, db) = ouvrir();
    let account = db.ensure_account("purse", "u1", "Budget").expect("compte");
    db.ensure_default_categories(&account).expect("catégories par défaut");
    let n_defaut = db.list_categories(&account).expect("liste").len();
    assert!(n_defaut > 0);
    // Idempotent.
    db.ensure_default_categories(&account).expect("catégories par défaut x2");
    assert_eq!(db.list_categories(&account).expect("liste").len(), n_defaut);

    // create_category : INSERT OR IGNORE (id ou nom déjà pris = ignoré).
    db.create_category("cat-x", &account, "Voyages", Some("✈"), Some("#00f"), false, 99)
        .expect("catégorie");
    db.create_category("cat-x", &account, "Autre nom", None, None, false, 1)
        .expect("doublon d'id ignoré");
    db.create_category("cat-y", &account, "Voyages", None, None, false, 1)
        .expect("doublon de nom ignoré");
    let cats = db.list_categories(&account).expect("liste");
    assert_eq!(cats.len(), n_defaut + 1);
    let voyage = cats.iter().find(|c| c.id == "cat-x").expect("cat-x");
    assert_eq!(voyage.name, "Voyages");
    assert_eq!(voyage.sort_order, 99);
    assert_eq!(voyage.icon.as_deref(), Some("✈"));

    // Budget + mouvements (déclencheur de cumul côté service).
    let jour = jour_courant();
    db.create_budget("b1", &account, "Vacances", 500.0, &jour, None).expect("budget");
    db.create_purse_movement(&account, "m1", -120.0, Some("cat-x"), Some("Train"), &jour, Some("b1"))
        .expect("mvt 1");
    db.create_purse_movement(&account, "m2", -30.0, None, None, &jour, None).expect("mvt 2");
    db.create_purse_movement(&account, "m3", 1000.0, None, Some("Salaire"), &jour, None)
        .expect("mvt 3");

    let budgets = db.list_budgets(&account).expect("budgets");
    assert_eq!(budgets.len(), 1);
    assert!((budgets[0].spent - 120.0).abs() < 1e-9, "cumul du déclencheur");
    assert_eq!(budgets[0].movements_count, 1);
    db.close_budget("b1").expect("clôture");
    assert_eq!(
        db.list_budgets(&account).expect("budgets")[0].status,
        jaykonta::domain::purse::BudgetStatus::from_str("closed")
    );

    // Listes filtrées et paginées.
    let (all, total) = db
        .list_purse_movements(&account, &MovementFilters::default())
        .expect("liste");
    assert_eq!(total, 3);
    assert_eq!(all.len(), 3);
    let filtres = MovementFilters {
        month: Some(mois_courant()),
        movement_type: Some(MovementType::Expense),
        ..MovementFilters::default()
    };
    let (depenses, n) = db.list_purse_movements(&account, &filtres).expect("dépenses");
    assert_eq!(n, 2);
    assert!(depenses.iter().all(|m| m.amount < 0.0));
    let train = all.iter().find(|m| m.id == "m1").expect("m1");
    assert_eq!(train.category.as_ref().map(|c| c.name.as_str()), Some("Voyages"));
    assert_eq!(train.budget.as_ref().map(|b| b.name.as_str()), Some("Vacances"));
    let page = MovementFilters {
        per_page: 2,
        page: 1,
        ..MovementFilters::default()
    };
    let (p2, n) = db.list_purse_movements(&account, &page).expect("page 2");
    assert_eq!((p2.len(), n), (1, 3));

    // Agrégats.
    assert!((db.solde_purse().expect("solde") - 850.0).abs() < 1e-9);
    let (inc, exp) = db.purse_month_totals(&account, &mois_courant()).expect("totaux");
    assert!((inc - 1000.0).abs() < 1e-9 && (exp - 150.0).abs() < 1e-9);
    let rep = db
        .categories_breakdown_purse(&account, &mois_courant())
        .expect("répartition");
    assert_eq!(rep.len(), 2);
    assert_eq!(rep[0].category_name, "Voyages");
    assert!((rep[0].percentage - 80.0).abs() < 1e-9);
    let stats = db.purse_stats().expect("stats purse");
    assert!((stats.monthly_expenses - 150.0).abs() < 1e-9);
    assert!((stats.budget_usage_pct - 15.0).abs() < 1e-9);
    assert!((stats.categorized_pct - 100.0 / 3.0).abs() < 1e-9);
    assert_eq!(db.movements_recent(2).expect("récents").len(), 2);
}

#[test]
fn objectifs_progression_et_atteinte() {
    let (_tmp, _addr, _cred, db) = ouvrir();
    let account = db.ensure_account("purse", "u1", "Budget").expect("compte");
    db.create_goal("g1", &account, "Epargne", 100.0, "savings", None).expect("objectif");
    assert_eq!(db.purse_stats().expect("stats").active_goals, 1);
    db.update_goal_progress("g1", 40.0).expect("progression");
    let g = &db.list_goals(&account).expect("objectifs")[0];
    assert!((g.current - 40.0).abs() < 1e-9);
    assert_eq!(g.status, jaykonta::domain::purse::GoalStatus::from_str("active"));
    db.update_goal_progress("g1", 60.0).expect("progression");
    let g = &db.list_goals(&account).expect("objectifs")[0];
    assert!((g.progress_pct - 100.0).abs() < 1e-9);
    assert_eq!(g.status, jaykonta::domain::purse::GoalStatus::from_str("reached"));
    // Objectif inconnu : sans effet (comme l'UPDATE hérité).
    db.update_goal_progress("inconnu", 1.0).expect("sans effet");
}

#[test]
fn recurrents_creation_bascule_suppression() {
    let (_tmp, _addr, _cred, db) = ouvrir();
    let account = db.ensure_account("purse", "u1", "Budget").expect("compte");
    let jour = jour_courant();
    db.create_recurring(
        "r1", &account, "expense", 30.0, Some(""), "Abonnement", "monthly", Some(5), None,
        &jour, None, &jour, false, None,
    )
    .expect("récurrent 1");
    db.create_recurring(
        "r2", &account, "income", 1200.0, None, "Salaire", "yearly", None, None, &jour, None,
        &jour, true, Some("note"),
    )
    .expect("récurrent 2");
    assert!(db
        .create_recurring(
            "r3", "", "expense", 1.0, None, "x", "monthly", None, None, &jour, None, &jour, false,
            None,
        )
        .is_err());

    let (inc, exp) = db.recurring_monthly_totals(&account).expect("totaux");
    assert!((inc - 100.0).abs() < 1e-9 && (exp - 30.0).abs() < 1e-9);

    db.toggle_recurring("r1", false).expect("suspension");
    assert_eq!(db.list_recurring(&account, true).expect("actifs").len(), 1);
    let tous = db.list_recurring(&account, false).expect("tous");
    assert_eq!(tous.len(), 2);
    assert!(!tous.iter().find(|r| r.id == "r1").expect("r1").is_active);
    let r2 = tous.iter().find(|r| r.id == "r2").expect("r2");
    assert!(r2.auto_create && r2.notes.as_deref() == Some("note"));
    let (_, exp) = db.recurring_monthly_totals(&account).expect("totaux");
    assert!(exp.abs() < 1e-9);

    db.toggle_recurring("r1", true).expect("réactivation");
    assert_eq!(db.list_recurring(&account, true).expect("actifs").len(), 2);
    db.delete_recurring("r1").expect("suppression");
    let restants = db.list_recurring(&account, false).expect("tous");
    assert_eq!(restants.len(), 1);
    assert_eq!(restants[0].id, "r2");
}

#[test]
fn flux_account_paiement_facture_atomique() {
    let (_tmp, _addr, _cred, db) = ouvrir();
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    db.insert_quote(&QuoteRecord {
        id: "quote-0001".into(),
        scope: "account".into(),
        context_ref: "pro:x".into(),
        counterparty_ref: "client:42".into(),
        total: 100.0,
        currency: "EUR".into(),
        status: "sent".into(),
        created_at: now.clone(),
    })
    .expect("devis");
    db.insert_invoice(&InvoiceRecord {
        id: "invoice-0001".into(),
        scope: "account".into(),
        context_ref: "pro:x".into(),
        counterparty_ref: "client:42".into(),
        quote_id: Some("quote-0001".into()),
        total: 100.0,
        paid_amount: 0.0,
        currency: "EUR".into(),
        status: "issued".into(),
        issued_at: now.clone(),
        due_at: None,
    })
    .expect("facture");

    let paiement = |id: &str, montant: f64| PaymentRecord {
        id: id.into(),
        invoice_id: "invoice-0001".into(),
        amount: montant,
        currency: "EUR".into(),
        paid_at: now.clone(),
        method: "virement".into(),
        reference_opaque: "tok".into(),
    };
    // Nouveau paiement partiel : déclencheur côté service.
    db.insert_payment_and_update_invoice(&paiement("pay-1", 40.0)).expect("paiement 1");
    let inv = &db.invoices_recent(10).expect("factures")[0];
    assert!((inv.paid_amount - 40.0).abs() < 1e-9);
    assert_eq!(inv.status, "partial");
    assert_eq!(inv.counterparty_ref, "client:42");
    let stats = db.account_stats().expect("stats");
    assert!((stats.impayes - 60.0).abs() < 1e-9);
    assert!((stats.ca_mensuel - 100.0).abs() < 1e-9);
    assert_eq!(stats.invoices_count, 1);

    // Paiement rejoué avec montant corrigé : lot atomique paiement + facture.
    db.insert_payment_and_update_invoice(&paiement("pay-1", 100.0)).expect("paiement rejoué");
    let inv = &db.invoices_recent(10).expect("factures")[0];
    assert!((inv.paid_amount - 100.0).abs() < 1e-9);
    assert_eq!(inv.status, "paid");
    assert!((db.account_stats().expect("stats").payment_rate_pct - 100.0).abs() < 1e-9);

    // Lot refusé (méthode hors CHECK) : ni le paiement ni la facture ne bougent.
    let mut invalide = paiement("pay-1", 10.0);
    invalide.method = "card_token".into();
    assert!(db.insert_payment_and_update_invoice(&invalide).is_err());
    let inv = &db.invoices_recent(10).expect("factures")[0];
    assert!((inv.paid_amount - 100.0).abs() < 1e-9, "lot tout-ou-rien");
    assert_eq!(inv.status, "paid");

    // Rappel, mouvement legacy, audit.
    db.insert_reminder(&ReminderRecord {
        id: "rem-1".into(),
        deadline_ref: "invoice:1".into(),
        due_at: now.clone(),
        label: "Relance".into(),
        context_ref: "ctx".into(),
        source_service: "jaykoa".into(),
    })
    .expect("rappel");
    let mvt = MovementRecord {
        id: "mv-legacy".into(),
        scope: "purse".into(),
        context_ref: "ctx".into(),
        category: "Courses".into(),
        amount: -12.5,
        currency: "EUR".into(),
        movement_date: jour_courant(),
        source_service: "manual".into(),
    };
    db.insert_movement(&mvt).expect("mouvement");
    db.insert_movement(&MovementRecord { amount: -15.0, ..mvt }).expect("mouvement remplacé");
    let recents = db.movements_recent(10).expect("récents");
    assert_eq!(recents.len(), 1);
    assert!((recents[0].amount + 15.0).abs() < 1e-9);
    assert_eq!(recents[0].category, "Courses");

    assert!(db.is_bootstrap_needed().expect("audit vide"));
    db.insert_audit(&AuditRecord {
        id: "aud-1".into(),
        contract_id: "CK-INT-02".into(),
        actor_ref: "test".into(),
        operation: "op".into(),
        scope: "account".into(),
        object_ref: "obj".into(),
        result: "ok".into(),
        payload_json: "{}".into(),
        created_at: now,
    })
    .expect("audit");
    assert_eq!(db.audit_count().expect("compte audit"), 1);
    assert!(!db.is_bootstrap_needed().expect("audit non vide"));
}

#[test]
fn backend_injecte_construit_les_vues() {
    let (_tmp, _addr, _cred, db) = ouvrir();
    let db = Arc::new(db);
    let mut backend = JayKontaBackend::new_with_store(db.clone()).expect("backend");
    // Le bootstrap CK-INT-02/03 a alimenté la base via le pipeline.
    assert!(db.audit_count().expect("audit") > 0);
    assert!(!db.invoices_recent(5).expect("factures").is_empty());
    let vm = backend.view_model();
    assert!(!vm.kpis.is_empty());
    backend.set_entry_point(EntryPoint::Purse);
    assert!(!backend.view_model().kpis.is_empty());
    // Second backend sur la même base : pas de second bootstrap.
    let avant = db.audit_count().expect("audit");
    let _ = JayKontaBackend::new_with_store(db.clone()).expect("backend 2");
    assert_eq!(db.audit_count().expect("audit"), avant);
}

#[tokio::test(flavor = "multi_thread")]
async fn api_async_et_sync_depuis_un_runtime() {
    let tmp = TempDir::new();
    let (addr, creds) = kindmother_service::spawn_ephemeral(&tmp.0)
        .await
        .expect("serveur KindMother éphémère");
    let cred = OperatorCredential::load_from_dir("jaykonta", &creds).expect("justificatif");
    let db = JayKontaDb::connect_with_async(&addr.to_string(), &cred)
        .await
        .expect("connexion async");
    let account = db
        .ensure_account_async("purse", "u-async", "Budget")
        .await
        .expect("compte");
    // API synchrone appelée depuis un runtime multi-fils (block_in_place + fil dédié).
    db.create_goal("g-async", &account, "Objectif", 10.0, "savings", None)
        .expect("objectif (sync)");
    assert_eq!(db.list_goals_async(&account).await.expect("objectifs").len(), 1);
}

#[tokio::test(flavor = "current_thread")]
async fn api_sync_depuis_un_runtime_current_thread() {
    let (_tmp, addr, cred) = tokio::task::spawn_blocking(serveur).await.expect("serveur");
    // block_in_place paniquerait ici : l'appel passe par un fil dédié.
    let db = JayKontaDb::connect_with(&addr, &cred).expect("connexion sync");
    assert!(db.is_bootstrap_needed().expect("lecture"));
}
