//! @id     miyucloud_infrastructure_repositories_pg_cdr_reclassement_pg_repository
//! @do     stocker_reclassement_cdr
//! @role   securite
//! @layer  app
//! @human  miyucloud — dépôt PostgreSQL + magasin de blobs du reclassement CDR des fichiers hérités `inconnu`.

//! Dépôt du reclassement CDR (voir
//! [`crate::application::services::reclassement_cdr_service`]).
//!
//! Toutes les écritures sont **conditionnelles** (`etat_cdr = 'inconnu'` et
//! blob attendu) : une écriture concurrente de l'utilisateur n'est jamais
//! écrasée. Les fichiers à la corbeille sont reclassés aussi (une
//! restauration ne doit pas rendre servable un fichier non désarmé).

use std::path::Path;
use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::application::services::reclassement_cdr_service::{
    ContenuReclasse, DepotReclassementCdr, FichierInconnu,
};
use crate::common::errors::DomainError;
use crate::domain::entities::etat_cdr::{EtatCdr, MarqueCdr};
use crate::infrastructure::repositories::pg::FileBlobReadRepository;
use crate::infrastructure::services::dedup_service::DedupService;
use crate::infrastructure::services::file_content_cache::FileContentCache;
use crate::infrastructure::services::image_transcode_service::ImageTranscodeService;
use crate::infrastructure::services::thumbnail_service::ThumbnailService;

/// Fichiers `inconnu` par identifiant croissant après un curseur. Prédicat
/// `etat_cdr = 'inconnu'` **littéral**, identique à celui de l'index partiel
/// `idx_files_etat_cdr_inconnu` (migration `20261025000002`).
const SQL_LISTER_INCONNUS: &str = "SELECT id, blob_hash, mime_type FROM storage.files \
     WHERE etat_cdr = 'inconnu' AND ($1::uuid IS NULL OR id > $1::uuid) \
     ORDER BY id LIMIT $2";

/// Décompte des fichiers `inconnu` (même prédicat littéral, même index).
const SQL_COMPTER_INCONNUS: &str = "SELECT COUNT(*) FROM storage.files WHERE etat_cdr = 'inconnu'";

/// Dépôt PostgreSQL du reclassement.
pub struct CdrReclassementPgRepository {
    pool: Arc<PgPool>,
    dedup: Arc<DedupService>,
    lecture: Arc<FileBlobReadRepository>,
    cache_contenu: Arc<FileContentCache>,
    transcodage: Arc<ImageTranscodeService>,
    vignettes: Arc<ThumbnailService>,
}

impl CdrReclassementPgRepository {
    /// Dépôt sur `pool` (pool de maintenance de préférence).
    pub fn new(
        pool: Arc<PgPool>,
        dedup: Arc<DedupService>,
        lecture: Arc<FileBlobReadRepository>,
        cache_contenu: Arc<FileContentCache>,
        transcodage: Arc<ImageTranscodeService>,
        vignettes: Arc<ThumbnailService>,
    ) -> Self {
        Self {
            pool,
            dedup,
            lecture,
            cache_contenu,
            transcodage,
            vignettes,
        }
    }

    fn erreur(contexte: &str, e: impl std::fmt::Display) -> DomainError {
        DomainError::internal_error("ReclassementCdr", format!("{contexte} : {e}"))
    }
}

impl DepotReclassementCdr for CdrReclassementPgRepository {
    async fn lister_inconnus(
        &self,
        apres: Option<Uuid>,
        limite: i64,
    ) -> Result<Vec<FichierInconnu>, DomainError> {
        // Littéral `'inconnu'` (et non un paramètre) : le planificateur ne
        // retient l'index partiel `idx_files_etat_cdr_inconnu` (… WHERE
        // etat_cdr = 'inconnu') que si le prédicat est connu à la préparation.
        let lignes = sqlx::query_as::<_, (Uuid, String, String)>(SQL_LISTER_INCONNUS)
            .bind(apres)
            .bind(limite)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| Self::erreur("liste des fichiers inconnus", e))?;
        Ok(lignes
            .into_iter()
            .map(|(id, blob_hash, mime_type)| FichierInconnu {
                id,
                blob_hash,
                mime_type,
            })
            .collect())
    }

    async fn extraire_blob(&self, hash: &str, vers: &Path) -> Result<(), DomainError> {
        // Hachage hexadécimal attendu (chemin du blob construit à partir de lui).
        if hash.len() < 2 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Self::erreur("hachage de blob invalide", hash));
        }
        // Contenu clair (déchiffré si le blob est chiffré au repos).
        self.dedup
            .copier_blob_clair(hash, vers)
            .await
            .map_err(|e| Self::erreur("lecture du blob", e))?;
        Ok(())
    }

    async fn remplacer_contenu(
        &self,
        id: Uuid,
        ancien_hash: &str,
        depuis: &Path,
        contenu: &ContenuReclasse,
    ) -> Result<Option<String>, DomainError> {
        let taille = i64::try_from(contenu.taille).map_err(|e| Self::erreur("taille", e))?;
        // Blob placé sur disque hors transaction, puis tout le reste en une
        // transaction : ligne conditionnelle, référence du nouveau blob,
        // retrait de celle de l'ancien.
        let prepare = self
            .dedup
            .preparer_blob(depuis, contenu.hachage.clone())
            .await?;
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Self::erreur("transaction", e))?;
        // `updated_at` inchangé : le contenu est le même document, désarmé.
        let n = sqlx::query(
            "UPDATE storage.files \
                SET blob_hash = $1, size = $2, mime_type = $3 \
              WHERE id = $4 AND etat_cdr = $5 AND blob_hash = $6",
        )
        .bind(&prepare.hash)
        .bind(taille)
        .bind(&contenu.mime_type)
        .bind(id)
        .bind(EtatCdr::Inconnu.as_str())
        .bind(ancien_hash)
        .execute(&mut *tx)
        .await
        .map_err(|e| Self::erreur("remplacement du contenu", e))?
        .rows_affected();
        if n != 1 {
            // Écriture concurrente : rien n'est écrit (annulation implicite).
            return Ok(None);
        }
        if prepare.hash != ancien_hash {
            self.dedup
                .referencer_blob(&mut tx, &prepare, Some(contenu.mime_type.clone()))
                .await?;
            DedupService::decrementer_reference(&mut tx, ancien_hash).await?;
        }
        tx.commit()
            .await
            .map_err(|e| Self::erreur("validation du remplacement", e))?;
        self.dedup.liberer_source(&prepare).await;
        Ok(Some(prepare.hash))
    }

    async fn compter_inconnus(&self) -> Result<u64, DomainError> {
        // Littéral : index partiel `idx_files_etat_cdr_inconnu` (voir plus haut).
        let n: i64 = sqlx::query_scalar(SQL_COMPTER_INCONNUS)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Self::erreur("décompte des fichiers inconnus", e))?;
        Ok(u64::try_from(n).unwrap_or(0))
    }

    async fn invalider_derives(&self, id: Uuid) {
        let id = id.to_string();
        self.lecture.oublier_hachage(&id);
        self.cache_contenu.invalidate(&id).await;
        self.transcodage.invalidate(&id).await;
        if let Err(e) = self.vignettes.delete_thumbnails(&id).await {
            tracing::warn!("Reclassement CDR : vignettes de {id} non supprimées : {e}");
        }
    }

    async fn marquer(
        &self,
        id: Uuid,
        hash_attendu: &str,
        marque: &MarqueCdr,
    ) -> Result<bool, DomainError> {
        let n = sqlx::query(
            "UPDATE storage.files SET etat_cdr = $1, cdr_rapport = $2 \
              WHERE id = $3 AND etat_cdr = $4 AND blob_hash = $5",
        )
        .bind(marque.etat.as_str())
        .bind(&marque.rapport)
        .bind(id)
        .bind(EtatCdr::Inconnu.as_str())
        .bind(hash_attendu)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| Self::erreur("marquage de l'état CDR", e))?
        .rows_affected();
        Ok(n == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// L'index partiel n'est utilisable que si le prédicat de la requête est
    /// le littéral de l'index (un paramètre `$1` le rend inutilisable).
    #[test]
    fn requetes_inconnu_utilisent_le_predicat_de_l_index_partiel() {
        let migration = include_str!("../../../../migrations/20261025000002_cdr_reclassement.sql");
        assert!(migration.contains("WHERE etat_cdr = 'inconnu'"));
        for sql in [SQL_LISTER_INCONNUS, SQL_COMPTER_INCONNUS] {
            assert!(sql.contains("etat_cdr = 'inconnu'"), "{sql}");
            assert!(!sql.contains("etat_cdr = $"), "{sql}");
        }
        assert_eq!(EtatCdr::Inconnu.as_str(), "inconnu");
    }
}
