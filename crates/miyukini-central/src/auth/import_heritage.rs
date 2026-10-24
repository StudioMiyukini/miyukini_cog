//! @id     central_auth_import_heritage
//! @do     importer_base_heritee_central
//! @role   securite
//! @layer  infra
//! @human  miyukini-central — reprise unique de la base auth SQLite héritée dans
//!         KindMother (K-13), identifiants conservés, fichier hérité jamais modifié.

//! Import de la base auth Central héritée (`legacy-sqlite`, `db.rs`) dans la base
//! KindMother `central` (opérateur `miyukini-central`).
//!
//! Correspondance **identique** (schéma KindMother v1 = schéma final hérité). Points
//! explicites :
//! - base héritée ancienne : colonnes de profil ajoutées plus tard par
//!   `migrate_add_profile_columns` éventuellement absentes → valeur par défaut
//!   KindMother (`is_admin` = 0, autres `NULL`), comme l'aurait fait l'`ALTER TABLE`
//!   hérité ;
//! - hachages de mot de passe copiés tels quels (SHA-256 hérité ou Argon2id) : la
//!   migration progressive vers Argon2id à la connexion continue côté KindMother ;
//! - `cog_meta` copié tel quel (`cog_virgin`, `current_profile_id`). Le jeton
//!   `first_admin` du chemin KindMother n'est **pas** synthétisé : il ne sert qu'à
//!   départager deux inscriptions concurrentes sur une base sans profil, et sa présence
//!   empêcherait toute attribution d'admin si la table des profils redevenait vide
//!   (même comportement que le chemin hérité) ;
//! - les types partagés (`CentralProfile`…, déplacés dans `auth::modele`) ne changent pas
//!   le format stocké ;
//! - références, sauvegardes (BLOB copiés à l'octet) et hash Connect d'un profil absent
//!   (orphelins) ne sont pas importés : KindMother applique les clés étrangères ; ils
//!   sont comptés dans le rapport et le marqueur.

use std::path::Path;

use kindmother_client::KindMotherClient;
use kindmother_heritage::{
    ClefEtrangere as Fk, ErreurImport, PlanImport, PlanTable, RapportImport,
};

use super::schema_kindmother::migrations;

const FK_PROFIL: &[Fk] = &[Fk::requise("profile_id", "central_profiles")];

/// Plan d'import de la base `central` (profils avant tables liées).
#[must_use]
pub fn plan() -> PlanImport {
    let liee = |table, cle| PlanTable {
        table,
        cle,
        cles_etrangeres: FK_PROFIL,
        colonnes_tolerees: &[],
    };
    PlanImport {
        service: "miyukini-central",
        operateur: "miyukini-central",
        base: "central",
        migrations: migrations(),
        tables: vec![
            PlanTable::simple("central_profiles", &["id"]),
            PlanTable::simple("cog_meta", &["key"]),
            liee("profile_service_refs", &["profile_id", "service_key"]),
            liee("central_profile_saves", &["id"]),
            liee("connect_credentials", &["profile_id"]),
        ],
        tables_ignorees: &[],
    }
}

/// Importe la base héritée `chemin_heritee` dans la base KindMother `central`.
///
/// `client` : connecté avec le justificatif de l'opérateur `miyukini-central` sur la
/// base `central`. Idempotent ; marqueur `<base>.importe-kindmother` écrit après succès.
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
