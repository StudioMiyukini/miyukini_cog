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
        let lignes = sqlx::query_as::<_, (Uuid, String, String)>(
            "SELECT id, blob_hash, mime_type FROM storage.files \
              WHERE etat_cdr = $1 AND ($2::uuid IS NULL OR id > $2::uuid) \
              ORDER BY id LIMIT $3",
        )
        .bind(EtatCdr::Inconnu.as_str())
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
        tokio::fs::copy(self.dedup.blob_path(hash), vers)
            .await
            .map_err(|e| Self::erreur("lecture du blob", e))?;
        Ok(())
    }

    async fn ranger_blob(
        &self,
        depuis: &Path,
        mime_type: &str,
        hachage: Option<String>,
    ) -> Result<String, DomainError> {
        let r = self
            .dedup
            .store_from_file(depuis, Some(mime_type.to_string()), hachage)
            .await?;
        Ok(r.hash().to_string())
    }

    async fn retirer_reference(&self, hash: &str) -> Result<(), DomainError> {
        self.dedup.remove_reference(hash).await.map(|_| ())
    }

    async fn remplacer_contenu(
        &self,
        id: Uuid,
        ancien_hash: &str,
        contenu: &ContenuReclasse,
    ) -> Result<bool, DomainError> {
        // `updated_at` inchangé : le contenu est le même document, désarmé.
        let n = sqlx::query(
            "UPDATE storage.files \
                SET blob_hash = $1, size = $2, mime_type = $3 \
              WHERE id = $4 AND etat_cdr = $5 AND blob_hash = $6",
        )
        .bind(&contenu.blob_hash)
        .bind(i64::try_from(contenu.taille).map_err(|e| Self::erreur("taille", e))?)
        .bind(&contenu.mime_type)
        .bind(id)
        .bind(EtatCdr::Inconnu.as_str())
        .bind(ancien_hash)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| Self::erreur("remplacement du contenu", e))?
        .rows_affected();
        Ok(n == 1)
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
