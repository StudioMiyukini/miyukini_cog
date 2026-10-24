//! @id     miyucloud_interfaces_upload_spool
//! @do     spooler_corps_envoi_vers_disque_borne
//! @role   securite
//! @layer  app
//! @human  miyucloud — envoi en flux vers un fichier temporaire, taille maximale et quota vérifiés pendant l'écriture (M-04).

//! Envoi en flux borné (M-04, C-13).
//!
//! Aucun corps d'envoi n'est accumulé en mémoire : chaque tranche est écrite
//! sur disque dès sa réception. La taille maximale configurée
//! (`MIYUCLOUD_MAX_UPLOAD_SIZE`) et l'espace restant du quota de
//! l'utilisateur sont vérifiés **avant** la lecture (en-tête
//! `Content-Length` déclaré) puis **pendant** l'écriture. Au premier
//! dépassement, la lecture s'arrête et le fichier partiel est supprimé.

use std::fmt::Display;
use std::path::Path;

use axum::http::StatusCode;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::application::ports::storage_ports::StorageUsagePort;
use crate::common::di::AppState;
use crate::interfaces::errors::AppError;

/// Nombre d'octets conservés pour la détection du type MIME.
pub const MIME_SNIFF_BYTES: usize = 8192;

/// Refus ou échec d'un envoi en flux.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SpoolError {
    /// Taille maximale configurée dépassée (413).
    #[error("Upload exceeds maximum size of {limit} bytes")]
    TooLarge { limit: u64 },
    /// Quota de l'utilisateur dépassé (507).
    #[error("Storage quota exceeded ({available} bytes available)")]
    QuotaExceeded { available: u64 },
    /// Lecture du corps impossible (client déconnecté, flux invalide).
    #[error("Failed to read request body: {0}")]
    Body(String),
    /// Erreur d'écriture disque.
    #[error("Upload I/O error: {0}")]
    Io(String),
}

impl From<SpoolError> for AppError {
    fn from(err: SpoolError) -> Self {
        match err {
            SpoolError::TooLarge { .. } => AppError::payload_too_large(err.to_string()),
            SpoolError::QuotaExceeded { .. } => AppError::new(
                StatusCode::INSUFFICIENT_STORAGE,
                err.to_string(),
                "QuotaExceeded",
            ),
            SpoolError::Body(_) => AppError::bad_request(err.to_string()),
            SpoolError::Io(_) => AppError::internal_error(err.to_string()),
        }
    }
}

/// Budget d'un envoi : taille maximale et espace de quota disponible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UploadBudget {
    /// Taille maximale d'un envoi (configuration).
    pub max_size: u64,
    /// Octets encore disponibles dans le quota ; `None` = quota illimité.
    pub quota_available: Option<u64>,
}

impl UploadBudget {
    /// Budget sans quota (quota désactivé ou illimité).
    pub fn unlimited_quota(max_size: u64) -> Self {
        Self {
            max_size,
            quota_available: None,
        }
    }

    /// Construit le budget depuis l'usage (`used`), le quota (`quota`, 0 ou
    /// négatif = illimité) et la taille du contenu remplacé (`replaced`,
    /// libéré par l'écrasement d'un fichier existant).
    pub fn from_usage(max_size: u64, used: i64, quota: i64, replaced: u64) -> Self {
        let quota_available = if quota <= 0 {
            None
        } else {
            let used = u64::try_from(used.max(0)).unwrap_or(0);
            let quota = u64::try_from(quota).unwrap_or(0);
            Some(quota.saturating_add(replaced).saturating_sub(used))
        };
        Self {
            max_size,
            quota_available,
        }
    }

    /// Réduit le budget des octets déjà engagés (ex. tranches déjà reçues
    /// d'un envoi par morceaux).
    pub fn minus(self, already: u64) -> Self {
        Self {
            max_size: self.max_size.saturating_sub(already),
            quota_available: self.quota_available.map(|q| q.saturating_sub(already)),
        }
    }

    /// Vérifie qu'un total de `total` octets reste dans le budget.
    pub fn check(&self, total: u64) -> Result<(), SpoolError> {
        if total > self.max_size {
            return Err(SpoolError::TooLarge {
                limit: self.max_size,
            });
        }
        if let Some(available) = self.quota_available
            && total > available
        {
            return Err(SpoolError::QuotaExceeded { available });
        }
        Ok(())
    }

    /// Vérification préalable sur la taille déclarée (`Content-Length`).
    pub fn check_declared(&self, declared: Option<u64>) -> Result<(), SpoolError> {
        match declared {
            Some(len) => self.check(len),
            None => Ok(()),
        }
    }
}

/// Résultat d'un envoi écrit sur disque.
#[derive(Debug, Clone)]
pub struct Spooled {
    /// Octets écrits.
    pub size: u64,
    /// Empreinte BLAKE3 (hex) du contenu, calculée pendant l'écriture.
    pub hash: String,
    /// Premiers octets, pour la détection du type MIME.
    pub head: Vec<u8>,
}

/// Écrit `stream` dans `dest` en vérifiant `budget` à chaque tranche.
///
/// En cas de dépassement ou d'erreur, la lecture s'arrête immédiatement
/// (le reste du flux n'est jamais consommé) et `dest` est supprimé.
pub async fn spool_to_path<S, E>(
    stream: S,
    dest: &Path,
    budget: &UploadBudget,
) -> Result<Spooled, SpoolError>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Display,
{
    let result = spool_inner(stream, dest, budget).await;
    if result.is_err() {
        let _ = tokio::fs::remove_file(dest).await;
    }
    result
}

async fn spool_inner<S, E>(
    stream: S,
    dest: &Path,
    budget: &UploadBudget,
) -> Result<Spooled, SpoolError>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Display,
{
    // Rien n'est accepté si le budget est déjà épuisé.
    budget.check(0)?;

    let mut file = tokio::fs::File::create(dest)
        .await
        .map_err(|e| SpoolError::Io(e.to_string()))?;
    let mut hasher = blake3::Hasher::new();
    let mut head: Vec<u8> = Vec::new();
    let mut total: u64 = 0;

    let mut stream = std::pin::pin!(stream);
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| SpoolError::Body(e.to_string()))?;
        let len = u64::try_from(chunk.len()).unwrap_or(u64::MAX);
        total = total.saturating_add(len);
        budget.check(total)?;

        if head.len() < MIME_SNIFF_BYTES {
            let take = (MIME_SNIFF_BYTES - head.len()).min(chunk.len());
            head.extend_from_slice(&chunk[..take]);
        }
        hasher.update(&chunk);
        file.write_all(&chunk)
            .await
            .map_err(|e| SpoolError::Io(e.to_string()))?;
    }
    file.flush()
        .await
        .map_err(|e| SpoolError::Io(e.to_string()))?;

    Ok(Spooled {
        size: total,
        hash: hasher.finalize().to_hex().to_string(),
        head,
    })
}

/// Taille déclarée par l'en-tête `Content-Length`, si valide.
pub fn declared_length(headers: &axum::http::HeaderMap) -> Option<u64> {
    headers
        .get(axum::http::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse::<u64>().ok())
}

/// Budget d'envoi de `user_id` : taille maximale configurée et quota restant.
///
/// Fail-closed (C-12) : si le quota ne peut pas être lu, l'envoi est refusé.
pub async fn upload_budget_for(
    state: &AppState,
    user_id: Uuid,
    replaced: u64,
) -> Result<UploadBudget, AppError> {
    let max_size = u64::try_from(state.core.config.storage.max_upload_size).unwrap_or(u64::MAX);
    let Some(storage_svc) = state.storage_usage_service.as_ref() else {
        return Ok(UploadBudget::unlimited_quota(max_size));
    };
    let (used, quota) = storage_svc
        .get_user_storage_info(user_id)
        .await
        .map_err(|e| {
            tracing::warn!("upload refused: quota lookup failed for {}: {}", user_id, e);
            AppError::internal_error("Unable to verify storage quota")
        })?;
    Ok(UploadBudget::from_usage(max_size, used, quota, replaced))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const CHUNK: usize = 64 * 1024;

    /// Flux de `count` tranches de 64 Kio qui compte les tranches consommées.
    fn counting_stream(
        count: usize,
        pulled: Arc<AtomicUsize>,
    ) -> impl Stream<Item = Result<Bytes, std::io::Error>> {
        futures::stream::iter(0..count).map(move |_| {
            pulled.fetch_add(1, Ordering::SeqCst);
            Ok(Bytes::from(vec![0xAB; CHUNK]))
        })
    }

    #[tokio::test]
    async fn oversize_stream_is_rejected_without_consuming_it() {
        let dir = tempfile::tempdir().expect("tempdir");
        let dest = dir.path().join("upload.part");
        let pulled = Arc::new(AtomicUsize::new(0));
        // 4096 tranches de 64 Kio = 256 Mio annoncés, limite 1 Mio.
        let stream = counting_stream(4096, pulled.clone());
        let budget = UploadBudget::unlimited_quota(1024 * 1024);

        let err = spool_to_path(stream, &dest, &budget)
            .await
            .expect_err("must be refused");
        assert_eq!(err, SpoolError::TooLarge { limit: 1024 * 1024 });
        // Arrêt à la première tranche qui dépasse : 17 tranches lues sur 4096.
        assert_eq!(pulled.load(Ordering::SeqCst), 17);
        // Le fichier partiel est supprimé.
        assert!(!dest.exists());
    }

    #[tokio::test]
    async fn oversize_axum_body_is_rejected() {
        let dir = tempfile::tempdir().expect("tempdir");
        let dest = dir.path().join("body.part");
        let pulled = Arc::new(AtomicUsize::new(0));
        let body = axum::body::Body::from_stream(counting_stream(1024, pulled.clone()));
        let budget = UploadBudget::unlimited_quota(200 * 1024);

        let err = spool_to_path(body.into_data_stream(), &dest, &budget)
            .await
            .expect_err("must be refused");
        assert!(matches!(err, SpoolError::TooLarge { .. }));
        assert!(pulled.load(Ordering::SeqCst) < 10);
        assert!(!dest.exists());
        assert_eq!(
            AppError::from(err).status_code,
            StatusCode::PAYLOAD_TOO_LARGE
        );
    }

    #[tokio::test]
    async fn quota_exceeded_during_write_is_refused() {
        let dir = tempfile::tempdir().expect("tempdir");
        let dest = dir.path().join("quota.part");
        let pulled = Arc::new(AtomicUsize::new(0));
        // Quota 1 Gio, 999,9 Mio utilisés → ~100 Kio disponibles.
        let budget = UploadBudget::from_usage(
            10 * 1024 * 1024 * 1024,
            1024 * 1024 * 1024 - 100 * 1024,
            1024 * 1024 * 1024,
            0,
        );
        let err = spool_to_path(counting_stream(100, pulled.clone()), &dest, &budget)
            .await
            .expect_err("must be refused");
        assert_eq!(
            err,
            SpoolError::QuotaExceeded {
                available: 100 * 1024
            }
        );
        assert_eq!(pulled.load(Ordering::SeqCst), 2);
        assert!(!dest.exists());
        assert_eq!(
            AppError::from(err).status_code,
            StatusCode::INSUFFICIENT_STORAGE
        );
    }

    #[tokio::test]
    async fn upload_within_budget_is_written_and_hashed() {
        let dir = tempfile::tempdir().expect("tempdir");
        let dest = dir.path().join("ok.part");
        let pulled = Arc::new(AtomicUsize::new(0));
        let budget = UploadBudget::from_usage(1024 * 1024, 0, 10 * 1024 * 1024, 0);
        let spooled = spool_to_path(counting_stream(3, pulled), &dest, &budget)
            .await
            .expect("accepted");
        assert_eq!(spooled.size, 3 * CHUNK as u64);
        assert_eq!(spooled.head.len(), MIME_SNIFF_BYTES);
        let content = tokio::fs::read(&dest).await.expect("read");
        assert_eq!(content.len(), 3 * CHUNK);
        assert_eq!(spooled.hash, blake3::hash(&content).to_hex().to_string());
    }

    #[test]
    fn declared_length_is_checked_before_reading() {
        let budget = UploadBudget::from_usage(1000, 900, 1000, 0);
        assert_eq!(
            budget.check_declared(Some(5000)),
            Err(SpoolError::TooLarge { limit: 1000 })
        );
        assert_eq!(
            budget.check_declared(Some(200)),
            Err(SpoolError::QuotaExceeded { available: 100 })
        );
        assert!(budget.check_declared(Some(100)).is_ok());
        assert!(budget.check_declared(None).is_ok());
    }

    #[test]
    fn budget_accounts_for_replaced_content_and_unlimited_quota() {
        // Écraser un fichier de 300 octets libère 300 octets.
        let b = UploadBudget::from_usage(10_000, 1000, 1000, 300);
        assert_eq!(b.quota_available, Some(300));
        // Quota 0 = illimité.
        let b = UploadBudget::from_usage(10_000, 5_000_000, 0, 0);
        assert_eq!(b.quota_available, None);
        // Usage déjà au-delà du quota : rien de disponible.
        let b = UploadBudget::from_usage(10_000, 2000, 1000, 0);
        assert_eq!(b.quota_available, Some(0));
        assert!(b.check(1).is_err());
    }

    #[test]
    fn budget_minus_already_received_chunks() {
        let b = UploadBudget::from_usage(1000, 0, 800, 0).minus(700);
        assert_eq!(b.max_size, 300);
        assert_eq!(b.quota_available, Some(100));
        assert!(b.check(150).is_err());
    }
}
