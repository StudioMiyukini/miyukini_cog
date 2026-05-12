//! `storage_kit` — wrapper libSQL chiffré.
//!
//! Implémentation en PR-2 (P3.a). Persiste les tables listées au §5
//! de la Spec : `sessions`, `app_passwords`, `backup_targets`,
//! `snapshots`, `share_links`, `miyucloud_redirects`, `dav_etags`.
//!
//! Clé de chiffrement dérivée par `kindmother::derive_key("jaycloud_storage_v1")`.
