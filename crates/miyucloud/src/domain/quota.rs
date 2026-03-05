//! Gestion des quotas de stockage par utilisateur.
//!
//! @id: miyucloud_domain_quota
//! @do: check_quota_before_upload_get_usage_set_max
//! @role: domain
//! @layer: domain
//!
//! Le quota 0 signifie illimite. Le quota est verifie AVANT l'upload (pas apres).
//! `used_bytes` est mis a jour apres chaque upload et chaque purge.

use crate::data::types::UserQuota;
use crate::errors::MiyucloudError;

#[cfg(feature = "legacy-sqlite")]
use crate::data::MiyucloudDb;

/// Operations sur les quotas.
pub struct QuotaOps;

impl QuotaOps {
    /// Verifie si un upload de `additional_bytes` est autorise.
    ///
    /// Retourne `Ok(())` si le quota n'est pas depasse, ou `QuotaExceeded` sinon.
    /// Un quota `max_bytes == 0` signifie illimite et passe toujours.
    #[cfg(feature = "legacy-sqlite")]
    pub fn check_quota(
        db: &MiyucloudDb,
        owner_id: &str,
        additional_bytes: u64,
    ) -> Result<(), MiyucloudError> {
        let quota = db.quota_get(owner_id)?;

        // 0 = unlimited
        if quota.max_bytes == 0 {
            return Ok(());
        }

        let projected = quota.used_bytes.saturating_add(additional_bytes);
        if projected > quota.max_bytes {
            return Err(MiyucloudError::QuotaExceeded(format!(
                "Upload of {} bytes would exceed quota ({} / {} bytes used)",
                additional_bytes, quota.used_bytes, quota.max_bytes
            )));
        }

        Ok(())
    }

    /// Retourne l'usage courant pour un proprietaire.
    #[cfg(feature = "legacy-sqlite")]
    pub fn get_usage(db: &MiyucloudDb, owner_id: &str) -> Result<UserQuota, MiyucloudError> {
        db.quota_get(owner_id)
    }

    /// Definit la limite maximale du quota pour un proprietaire.
    ///
    /// `max_bytes == 0` signifie illimite.
    #[cfg(feature = "legacy-sqlite")]
    pub fn set_max(db: &MiyucloudDb, owner_id: &str, max_bytes: u64) -> Result<(), MiyucloudError> {
        db.quota_set_max(owner_id, max_bytes)
    }
}

#[cfg(all(test, feature = "legacy-sqlite"))]
mod tests {
    use super::*;
    use crate::data::MiyucloudDb;

    fn mem_db() -> MiyucloudDb {
        MiyucloudDb::open(":memory:").unwrap()
    }

    #[test]
    fn test_quota_check_within_limit() {
        let db = mem_db();
        QuotaOps::set_max(&db, "owner1", 1_000_000).unwrap();
        let result = QuotaOps::check_quota(&db, "owner1", 500_000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quota_check_exceeds_limit() {
        let db = mem_db();
        QuotaOps::set_max(&db, "owner1", 1_000).unwrap();
        // Simulate existing usage
        db.quota_update_used("owner1", 900).unwrap();
        let result = QuotaOps::check_quota(&db, "owner1", 200);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Quota exceeded"));
    }

    #[test]
    fn test_quota_unlimited_always_passes() {
        let db = mem_db();
        // Default max_bytes is 0 = unlimited
        let result = QuotaOps::check_quota(&db, "owner1", 1_000_000_000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_used_bytes() {
        let db = mem_db();
        db.quota_update_used("owner1", 500).unwrap();
        let quota = QuotaOps::get_usage(&db, "owner1").unwrap();
        assert_eq!(quota.used_bytes, 500);

        db.quota_update_used("owner1", 300).unwrap();
        let quota = QuotaOps::get_usage(&db, "owner1").unwrap();
        assert_eq!(quota.used_bytes, 800);

        // Negative delta (purge)
        db.quota_update_used("owner1", -200).unwrap();
        let quota = QuotaOps::get_usage(&db, "owner1").unwrap();
        assert_eq!(quota.used_bytes, 600);
    }

    #[test]
    fn test_set_max() {
        let db = mem_db();
        QuotaOps::set_max(&db, "owner1", 5_000_000).unwrap();
        let quota = QuotaOps::get_usage(&db, "owner1").unwrap();
        assert_eq!(quota.max_bytes, 5_000_000);
    }

    #[test]
    fn test_quota_exact_limit() {
        let db = mem_db();
        QuotaOps::set_max(&db, "owner1", 1_000).unwrap();
        db.quota_update_used("owner1", 500).unwrap();
        // Exactly at the limit should pass
        let result = QuotaOps::check_quota(&db, "owner1", 500);
        assert!(result.is_ok());
        // One byte over should fail
        let result = QuotaOps::check_quota(&db, "owner1", 501);
        assert!(result.is_err());
    }
}
