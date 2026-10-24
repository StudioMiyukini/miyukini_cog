//! @id     jaykonta_import_heritage
//! @do     importer_base_heritee_jaykonta
//! @role   donnee
//! @layer  infra
//! @human  jaykonta — reprise unique de la base SQLite héritée dans KindMother (K-13),
//!         identifiants conservés, fichier hérité jamais modifié.

//! Import de la base JayKonta héritée (`legacy-sqlite`) dans la base KindMother `jaykonta`.
//!
//! Correspondance : **identique** (le schéma KindMother v1 est `schema.sql`, celui du
//! chemin hérité en version 2). Points explicites :
//! - base héritée en version 1 (avant `etape_v1_vers_v2`) : tables absentes comptées
//!   à 0, colonnes absentes prises à leur valeur par défaut KindMother ;
//! - déclencheurs de `schema.sql` côté KindMother : l'insertion d'un mouvement recalcule
//!   `budgets.spent`, celle d'un paiement `invoices.paid_amount` et `status`. Ces
//!   parents sont rétablis à leur valeur héritée à la vérification ; la réécriture
//!   déclenche `trg_*_updated`, donc `updated_at` de ces lignes prend la date de l'import
//!   (écart toléré et compté ; vaut aussi pour les tables à `updated_at` déclenché lors
//!   d'une reprise) ;
//! - le chemin hérité appliquait déjà les clés étrangères : pas d'orpheline attendue ;
//!   les clés nullables `''` (aucun parent possible) sont normalisées en `NULL`.

use std::path::Path;

use kindmother_client::KindMotherClient;
use kindmother_heritage::{
    ClefEtrangere as Fk, ErreurImport, PlanImport, PlanTable, RapportImport,
};

use super::schema_kindmother::schema_migrations;

/// `updated_at` réécrit par `trg_*_updated` à chaque mise à jour.
const MAJ: &[&str] = &["updated_at"];

const ID: &[&str] = &["id"];
const FK_COMPTE: &[Fk] = &[Fk::requise("account_id", "accounts")];
const FK_CATEGORIES: &[Fk] = &[
    Fk::requise("account_id", "accounts"),
    Fk::nullable("parent_id", "categories"),
];
const FK_MOUVEMENTS: &[Fk] = &[
    Fk::requise("account_id", "accounts"),
    Fk::nullable("category_id", "categories"),
    Fk::nullable("budget_id", "budgets"),
];
const FK_ALERTES: &[Fk] = &[
    Fk::requise("account_id", "accounts"),
    Fk::nullable("category_id", "categories"),
    Fk::nullable("goal_id", "goals"),
    Fk::nullable("budget_id", "budgets"),
];
const FK_DEVIS: &[Fk] = &[
    Fk::requise("account_id", "accounts"),
    Fk::requise("counterparty_id", "counterparties"),
];
const FK_LIGNES_DEVIS: &[Fk] = &[Fk::requise("quote_id", "quotes")];
const FK_FACTURES: &[Fk] = &[
    Fk::requise("account_id", "accounts"),
    Fk::requise("counterparty_id", "counterparties"),
    Fk::nullable("quote_id", "quotes"),
];
const FK_LIGNES_FACTURE: &[Fk] = &[Fk::requise("invoice_id", "invoices")];
const FK_PAIEMENTS: &[Fk] = &[
    Fk::requise("account_id", "accounts"),
    Fk::requise("invoice_id", "invoices"),
];
const FK_RECURRENTES: &[Fk] = &[
    Fk::requise("account_id", "accounts"),
    Fk::nullable("category_id", "categories"),
];

/// Table à clé `id`.
const fn table(
    table: &'static str,
    cles_etrangeres: &'static [Fk],
    colonnes_tolerees: &'static [&'static str],
) -> PlanTable {
    PlanTable {
        table,
        cle: ID,
        cles_etrangeres,
        colonnes_tolerees,
    }
}

/// Plan d'import JayKonta (ordre des clés étrangères : parents d'abord).
#[must_use]
pub fn plan() -> PlanImport {
    PlanImport {
        service: "jaykonta",
        operateur: "jaykonta",
        base: "jaykonta",
        migrations: schema_migrations(),
        tables: vec![
            PlanTable::simple("accounts", ID),
            table("categories", FK_CATEGORIES, &[]),
            table("budgets", FK_COMPTE, MAJ),
            table("goals", FK_COMPTE, MAJ),
            table("movements", FK_MOUVEMENTS, MAJ),
            table("alerts", FK_ALERTES, &[]),
            table("counterparties", FK_COMPTE, &[]),
            table("quotes", FK_DEVIS, MAJ),
            table("quote_lines", FK_LIGNES_DEVIS, &[]),
            table("invoices", FK_FACTURES, MAJ),
            table("invoice_lines", FK_LIGNES_FACTURE, &[]),
            table("payments", FK_PAIEMENTS, &[]),
            table("recurring_transactions", FK_RECURRENTES, MAJ),
            table("reminders", FK_COMPTE, &[]),
            table("audit", FK_COMPTE, &[]),
        ],
        tables_ignorees: &[],
    }
}

/// Importe la base héritée `chemin_heritee` dans la base KindMother `jaykonta`.
///
/// `client` : connecté avec le justificatif de l'opérateur `jaykonta` sur la base
/// `jaykonta`. Idempotent ; marqueur `<base>.importe-kindmother` écrit après succès.
///
/// # Errors
/// [`ErreurImport`] : base héritée illisible ou hors plan, refus ou échec KindMother,
/// vérification échouée, fichier hérité modifié pendant l'import ; aucun marqueur écrit.
pub async fn importer_base_heritee(
    chemin_heritee: &Path,
    client: &KindMotherClient,
) -> Result<RapportImport, ErreurImport> {
    kindmother_heritage::importer(chemin_heritee, client, &plan()).await
}
