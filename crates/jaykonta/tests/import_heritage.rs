//! @id     jaykonta_tests_import_heritage
//! @do     verifier_import_base_heritee_jaykonta
//! @role   test
//! @layer  infra
//! @human  jaykonta — import de la base SQLite héritée dans un KindMother éphémère :
//!         lignes identiques, marqueur, no-op à la relance, fichier intact, `open` accepté.

#![cfg(all(
    feature = "import-heritage",
    feature = "kindmother-only",
    not(feature = "legacy-sqlite")
))]

use std::path::PathBuf;

use jaykonta::data::import_heritage::{importer_base_heritee, plan};
use jaykonta::data::JayKontaDb;
use kindmother_client::heritage::{chemin_marqueur_import, refuser_si_base_heritee};
use kindmother_client::{KindMotherClient, OperatorCredential, SqlValue};
use kindmother_heritage::fabrique::{creer_base_heritee, verifier_dans_kindmother};

/// DDL hérité : `schema.sql` (version 2 du chemin `legacy-sqlite`).
const DDL_HERITE: &str = include_str!("../src/data/schema.sql");

struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn t(s: &str) -> SqlValue {
    SqlValue::Text(s.to_string())
}

fn r(v: f64) -> SqlValue {
    SqlValue::Real(v)
}

/// Lignes héritées. Paiement et mouvement sont insérés avant leur parent : les
/// déclencheurs hérités ne touchent donc pas la facture ni le budget, dont les valeurs
/// (`status = 'cancelled'`, `spent` ≠ somme des mouvements) diffèrent de ce que les
/// déclencheurs KindMother recalculeraient : l'import doit les rétablir.
fn lignes() -> Vec<(&'static str, Vec<(&'static str, SqlValue)>)> {
    let h = |s: &str| t(s);
    vec![
        (
            "accounts",
            vec![
                ("id", t("a1")),
                ("scope", t("purse")),
                ("user_id", t("u1")),
                ("label", t("Budget")),
                ("currency", t("EUR")),
                ("created_at", h("2025-01-01 10:00:00")),
                ("updated_at", h("2025-01-01 10:00:00")),
            ],
        ),
        (
            "accounts",
            vec![
                ("id", t("a2")),
                ("scope", t("account")),
                ("user_id", t("u1")),
                ("label", t("Pro")),
                ("currency", t("EUR")),
                ("created_at", h("2025-01-01 10:00:00")),
                ("updated_at", h("2025-01-01 10:00:00")),
            ],
        ),
        // Enfant avant parent (auto-référence) : l'import doit réordonner.
        (
            "categories",
            vec![
                ("id", t("c2")),
                ("account_id", t("a1")),
                ("name", t("Restaurants")),
                ("icon", SqlValue::Null),
                ("color", t("")),
                ("parent_id", t("c1")),
                ("is_income", SqlValue::Integer(0)),
                ("sort_order", SqlValue::Integer(2)),
                ("created_at", h("2025-01-02 00:00:00")),
            ],
        ),
        (
            "categories",
            vec![
                ("id", t("c1")),
                ("account_id", t("a1")),
                ("name", t("Alimentation")),
                ("icon", t("🍞")),
                ("color", SqlValue::Null),
                ("parent_id", SqlValue::Null),
                ("is_income", SqlValue::Integer(0)),
                ("sort_order", SqlValue::Integer(1)),
                ("created_at", h("2025-01-02 00:00:00")),
            ],
        ),
        (
            "movements",
            vec![
                ("id", t("m1")),
                ("account_id", t("a1")),
                ("scope", t("purse")),
                ("category_id", t("c1")),
                ("amount", r(-120.0)),
                ("currency", t("EUR")),
                ("description", t("Courses")),
                ("movement_date", t("2025-02-01")),
                ("context_ref", SqlValue::Null),
                ("source_service", SqlValue::Null),
                ("budget_id", t("b1")),
                ("created_at", h("2025-02-01 08:00:00")),
                ("updated_at", h("2025-02-01 08:00:00")),
            ],
        ),
        (
            "budgets",
            vec![
                ("id", t("b1")),
                ("account_id", t("a1")),
                ("name", t("Vacances")),
                ("target", r(500.0)),
                ("spent", r(999.5)),
                ("currency", t("EUR")),
                ("start_date", t("2025-01-01")),
                ("end_date", SqlValue::Null),
                ("status", t("active")),
                ("created_at", h("2025-01-01 00:00:00")),
                ("updated_at", h("2025-03-03 03:03:03")),
            ],
        ),
        (
            "counterparties",
            vec![
                ("id", t("cp1")),
                ("account_id", t("a2")),
                ("name", t("Client SA")),
                ("email", SqlValue::Null),
                ("siret", t("")),
                ("created_at", h("2025-01-05 00:00:00")),
                ("updated_at", h("2025-01-05 00:00:00")),
            ],
        ),
        (
            "payments",
            vec![
                ("id", t("pay1")),
                ("account_id", t("a2")),
                ("invoice_id", t("inv1")),
                ("amount", r(50.0)),
                ("currency", t("EUR")),
                ("method", t("virement")),
                ("reference_opaque", SqlValue::Null),
                ("paid_at", t("2025-04-01")),
                ("notes", SqlValue::Null),
                ("created_at", h("2025-04-01 00:00:00")),
            ],
        ),
        (
            "invoices",
            vec![
                ("id", t("inv1")),
                ("account_id", t("a2")),
                ("number", t("FAC-1")),
                ("counterparty_id", t("cp1")),
                ("quote_id", SqlValue::Null),
                ("total_ht", r(100.0)),
                ("total_tva", r(20.0)),
                ("total_ttc", r(120.0)),
                ("paid_amount", r(50.0)),
                ("currency", t("EUR")),
                ("status", t("cancelled")),
                ("issued_at", h("2025-03-30 00:00:00")),
                ("due_at", SqlValue::Null),
                ("created_at", h("2025-03-30 00:00:00")),
                ("updated_at", h("2025-04-02 00:00:00")),
            ],
        ),
        (
            "invoice_lines",
            vec![
                ("id", t("il1")),
                ("invoice_id", t("inv1")),
                ("description", t("Prestation")),
                ("quantity", r(1.0)),
                ("unit_price", r(100.0)),
                ("tva_rate", r(20.0)),
                ("total_ht", r(100.0)),
                ("sort_order", SqlValue::Integer(0)),
            ],
        ),
        (
            "audit",
            vec![
                ("id", t("au1")),
                ("account_id", t("a2")),
                ("contract_id", t("ct")),
                ("actor_ref", t("u1")),
                ("operation", t("create")),
                ("scope", t("account")),
                ("object_ref", t("inv1")),
                ("result", t("ok")),
                ("payload", SqlValue::Null),
                ("created_at", h("2025-04-02 00:00:00")),
            ],
        ),
    ]
}

#[test]
fn import_base_heritee_jaykonta() {
    let tmp =
        TempDir(std::env::temp_dir().join(format!("jaykonta-heritage-{}", uuid::Uuid::new_v4())));
    std::fs::create_dir_all(&tmp.0).expect("tempdir");
    let (addr, creds) =
        kindmother_service::spawn_ephemeral_blocking(tmp.0.join("km")).expect("serveur éphémère");
    let addr = addr.to_string();
    let cred = OperatorCredential::load_from_dir("jaykonta", &creds).expect("justificatif");

    let base = tmp.0.join("jaykonta.db");
    let lignes = lignes();
    creer_base_heritee(&base, DDL_HERITE, &lignes).expect("base héritée");
    let octets = std::fs::read(&base).expect("lecture");

    // Avant import : `open` refuse.
    assert!(JayKontaDb::open(&base).is_err(), "garde de bascule");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    rt.block_on(async {
        let client = KindMotherClient::connect(&addr, &cred, "jaykonta")
            .await
            .expect("client");
        let rapport = importer_base_heritee(&base, &client).await.expect("import");
        println!("{rapport}");
        let table = |n: &str| {
            rapport
                .tables
                .iter()
                .find(|t| t.table == n)
                .expect("table")
                .clone()
        };
        assert_eq!(table("categories").dans_kindmother, 2);
        assert_eq!(
            table("budgets").corrigees,
            1,
            "spent recalculé par le déclencheur puis rétabli"
        );
        assert_eq!(
            table("invoices").corrigees,
            1,
            "statut recalculé par le déclencheur puis rétabli"
        );
        assert_eq!(table("goals").lues, 0);
        assert_eq!(rapport.total_orphelines(), 0);

        // Lignes identiques ; `updated_at` du budget et de la facture rétablis
        // prend la date de l'import (déclencheur de mise à jour, écart toléré).
        let n = verifier_dans_kindmother(
            &client,
            &plan(),
            &lignes,
            &[("budgets", "updated_at"), ("invoices", "updated_at")],
        )
        .await
        .expect("lignes identiques");
        assert_eq!(n, lignes.len());

        // Relance : no-op.
        let encore = importer_base_heritee(&base, &client)
            .await
            .expect("relance");
        assert!(encore.deja_importe);
    });

    assert!(chemin_marqueur_import(&base).is_file());
    assert_eq!(
        std::fs::read(&base).expect("lecture"),
        octets,
        "fichier hérité intact"
    );
    assert!(refuser_si_base_heritee(&base).is_ok());

    // Après import : `open` (client global, environnement) accepte et lit les données.
    std::env::set_var("KINDMOTHER_ADDR", &addr);
    std::env::set_var(kindmother_client::ENV_CREDENTIALS_DIR, &creds);
    let db = JayKontaDb::open(&base).expect("open après import");
    let categories = db.list_categories("a1").expect("catégories");
    assert_eq!(categories.len(), 2);
    let budgets = db.list_budgets("a1").expect("budgets");
    assert!((budgets[0].spent - 999.5).abs() < 1e-9);
}
