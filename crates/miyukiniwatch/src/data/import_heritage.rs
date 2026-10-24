//! @id     miyukiniwatch_import_heritage
//! @do     importer_base_heritee_miyukiniwatch
//! @role   donnee
//! @layer  infra
//! @human  miyukiniwatch — reprise unique de la base SQLite héritée dans KindMother
//!         (K-13), identifiants conservés, fichier hérité jamais modifié.

//! Import de la base MiyukiniWatch héritée (`legacy-sqlite`, `db.rs`) dans la base
//! KindMother `miyukiniwatch`.
//!
//! Correspondance **identique** (schéma KindMother v1 = schéma hérité). Points explicites :
//! - les tables à `INTEGER PRIMARY KEY AUTOINCREMENT` (`daily`, `weekly`, `audit`) gardent
//!   leurs identifiants ; `sqlite_sequence` côté KindMother suit le maximum inséré, les
//!   identifiants futurs ne réutilisent donc pas d'anciens numéros ;
//! - aucune clé étrangère ; `NULL` des colonnes `service_id`/`friend_id` conservé (il
//!   entre dans les contraintes d'unicité des agrégats).

use std::path::Path;

use kindmother_client::KindMotherClient;
use kindmother_heritage::{ErreurImport, PlanImport, PlanTable, RapportImport};

use super::schema_kindmother::migrations;

/// Plan d'import MiyukiniWatch.
#[must_use]
pub fn plan() -> PlanImport {
    const ID: &[&str] = &["id"];
    PlanImport {
        service: "miyukiniwatch",
        operateur: "miyukiniwatch",
        base: "miyukiniwatch",
        migrations: migrations(),
        tables: vec![
            PlanTable::simple("miyukiniwatch_metrics", &["record_id"]),
            PlanTable::simple("miyukiniwatch_daily", ID),
            PlanTable::simple("miyukiniwatch_weekly", ID),
            PlanTable::simple("miyukiniwatch_globals", &["profile_id", "key"]),
            PlanTable::simple("miyukiniwatch_audit", ID),
            PlanTable::simple("miyukiniwatch_prefs", &["profile_id"]),
        ],
        tables_ignorees: &[],
    }
}

/// Importe la base héritée `chemin_heritee` dans la base KindMother `miyukiniwatch`.
///
/// `client` : connecté avec le justificatif de l'opérateur `miyukiniwatch` sur la base
/// `miyukiniwatch`. Idempotent ; marqueur `<base>.importe-kindmother` écrit après succès.
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
