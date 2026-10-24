//! @id     jay1tribu_import_heritage
//! @do     importer_base_heritee_jay1tribu
//! @role   donnee
//! @layer  infra
//! @human  jay1tribu — reprise unique de la base SQLite héritée dans KindMother (K-13),
//!         identifiants conservés, fichier hérité jamais modifié.

//! Import de la base Jay1Tribu héritée (`legacy-sqlite`) dans la base KindMother `jay1tribu`.
//!
//! Correspondance : **identique** (schéma KindMother v1 = DDL du chemin hérité). Points
//! explicites :
//! - le chemin hérité supprimait tribus et salons sans leurs dépendances (livraisons,
//!   pièces jointes, invitations) ; si sa build n'appliquait pas les clés étrangères,
//!   ces lignes orphelines existent. KindMother les refuserait : elles ne sont pas
//!   importées, elles sont comptées dans le rapport et le marqueur ;
//! - `salons.tribe_id` (nullable) valant `''` est normalisé en `NULL` (salon direct) ;
//! - `NULL` et chaîne vide sont conservés tels quels ailleurs (le chemin hérité écrivait
//!   `NULL` pour `None`, comme le chemin KindMother).

use std::path::Path;

use kindmother_client::KindMotherClient;
use kindmother_heritage::{
    ClefEtrangere as Fk, ErreurImport, PlanImport, PlanTable, RapportImport,
};

use super::schema_kindmother::migrations;

const ID: &[&str] = &["id"];
const FK_TRIBU: &[Fk] = &[Fk::requise("tribe_id", "tribes")];
const FK_MEMBRES: &[Fk] = &[
    Fk::requise("tribe_id", "tribes"),
    Fk::requise("role_id", "tribe_roles"),
];
const FK_SALON_TRIBU: &[Fk] = &[Fk::nullable("tribe_id", "tribes")];
const FK_SALON: &[Fk] = &[Fk::requise("salon_id", "salons")];
const FK_MESSAGE: &[Fk] = &[Fk::requise("message_id", "messages")];

/// Table à clé `id`.
const fn table(table: &'static str, cles_etrangeres: &'static [Fk]) -> PlanTable {
    PlanTable {
        table,
        cle: ID,
        cles_etrangeres,
        colonnes_tolerees: &[],
    }
}

/// Plan d'import Jay1Tribu (ordre des clés étrangères : parents d'abord).
#[must_use]
pub fn plan() -> PlanImport {
    PlanImport {
        service: "jay1tribu",
        operateur: "jay1tribu",
        base: "jay1tribu",
        migrations: migrations(),
        tables: vec![
            PlanTable::simple("friends", ID),
            PlanTable::simple("tribes", ID),
            table("tribe_roles", FK_TRIBU),
            table("tribe_members", FK_MEMBRES),
            table("salons", FK_SALON_TRIBU),
            PlanTable {
                table: "salon_members",
                cle: &["salon_id", "cog_id"],
                cles_etrangeres: FK_SALON,
                colonnes_tolerees: &[],
            },
            table("messages", FK_SALON),
            table("pending_deliveries", FK_MESSAGE),
            table("tribe_invitations", FK_TRIBU),
            table("message_attachments", FK_MESSAGE),
        ],
        tables_ignorees: &[],
    }
}

/// Importe la base héritée `chemin_heritee` dans la base KindMother `jay1tribu`.
///
/// `client` : connecté avec le justificatif de l'opérateur `jay1tribu` sur la base
/// `jay1tribu`. Idempotent ; marqueur `<base>.importe-kindmother` écrit après succès.
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
