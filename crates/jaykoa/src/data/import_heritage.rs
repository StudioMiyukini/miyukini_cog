//! @id     jaykoa_import_heritage
//! @do     importer_base_heritee_jaykoa
//! @role   donnee
//! @layer  infra
//! @human  jaykoa — reprise unique de la base SQLite héritée dans KindMother (K-13),
//!         identifiants conservés, fichier hérité jamais modifié.

//! Import de la base JayKoa héritée dans la base KindMother `jaykoa`.
//!
//! Le chemin SQLite direct de JayKoa (`kindmother_db.rs`, retiré au lot C) écrivait
//! `agendas`, `temporal_entries` et `user_settings` ; le schéma KindMother v1 est le
//! même DDL. Correspondance **identique** ; points explicites :
//! - `NULL` et chaîne vide conservés (l'ancien chemin écrivait `NULL` pour `None`) ;
//! - un engagement dont l'agenda n'existe plus (orphelin, possible si la build
//!   héritée n'appliquait pas les clés étrangères) n'est pas importé : KindMother le
//!   refuserait ; il est compté dans le rapport et le marqueur.

use std::path::Path;

use kindmother_client::KindMotherClient;
use kindmother_heritage::{
    ClefEtrangere as Fk, ErreurImport, PlanImport, PlanTable, RapportImport,
};

use super::schema_kindmother::migrations;

const FK_AGENDA: &[Fk] = &[Fk::requise("agenda_id", "agendas")];

/// Plan d'import JayKoa (agendas avant engagements).
#[must_use]
pub fn plan() -> PlanImport {
    PlanImport {
        service: "jaykoa",
        operateur: "jaykoa",
        base: "jaykoa",
        migrations: migrations(),
        tables: vec![
            PlanTable::simple("agendas", &["id"]),
            PlanTable {
                table: "temporal_entries",
                cle: &["id"],
                cles_etrangeres: FK_AGENDA,
                colonnes_tolerees: &[],
            },
            PlanTable::simple("user_settings", &["profile_id"]),
        ],
        tables_ignorees: &[],
    }
}

/// Importe la base héritée `chemin_heritee` dans la base KindMother `jaykoa`.
///
/// `client` : connecté avec le justificatif de l'opérateur `jaykoa` sur la base
/// `jaykoa`. Idempotent ; marqueur `<base>.importe-kindmother` écrit après succès.
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
