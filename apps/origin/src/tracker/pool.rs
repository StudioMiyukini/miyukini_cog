//! Pools de COGs par version.
//!
//! Gère l'isolation des COGs par version des Cores.
//!
//! Note : Code préparé pour fonctionnalités futures (pools étendus).
#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use tracing::{debug, info};

/// Informations sur un COG dans le pool.
#[derive(Debug, Clone)]
pub struct CogEntry {
    /// Identifiant du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
    /// Adresse de connexion.
    pub address: String,
    /// Dernière mise à jour.
    pub last_seen: DateTime<Utc>,
    /// Liste des services exposés.
    pub services: Vec<String>,
    /// Liste des lobbys.
    pub lobbys: Vec<LobbyEntry>,
}

/// Informations sur un Lobby.
#[derive(Debug, Clone)]
pub struct LobbyEntry {
    /// Identifiant du Lobby.
    pub lobby_id: String,
    /// Nom du Lobby.
    pub name: String,
    /// Public ou privé.
    pub is_public: bool,
    /// Mot de passe requis.
    pub password_required: bool,
    /// Hash Argon2 du mot de passe (si password_required).
    pub password_hash: Option<String>,
    /// Nombre de places.
    pub max_players: u32,
    /// Nombre de joueurs actuels.
    pub current_players: u32,
    /// Métadonnées (JSON).
    pub metadata: String,
}

/// Pool de COGs pour une version donnée.
#[derive(Debug)]
pub struct Pool {
    /// Version des Cores.
    pub version: String,
    /// COGs dans ce pool.
    cogs: Arc<RwLock<HashMap<String, CogEntry>>>,
    /// Date de création.
    pub created_at: DateTime<Utc>,
}

impl Pool {
    /// Crée un nouveau pool.
    #[must_use]
    pub fn new(version: String) -> Self {
        Self {
            version,
            cogs: Arc::new(RwLock::new(HashMap::new())),
            created_at: Utc::now(),
        }
    }

    /// Ajoute un COG au pool.
    pub async fn add_cog(&self, entry: CogEntry) {
        let cog_id = entry.cog_id.clone();
        let mut cogs = self.cogs.write().await;
        cogs.insert(cog_id.clone(), entry);
        debug!("Added COG {} to pool {}", cog_id, self.version);
    }

    /// Supprime un COG du pool.
    pub async fn remove_cog(&self, cog_id: &str) {
        let mut cogs = self.cogs.write().await;
        if cogs.remove(cog_id).is_some() {
            debug!("Removed COG {} from pool {}", cog_id, self.version);
        }
    }

    /// Met à jour un COG.
    pub async fn update_cog(&self, cog_id: &str, services: Vec<String>, lobbys: Vec<LobbyEntry>) {
        let mut cogs = self.cogs.write().await;
        if let Some(entry) = cogs.get_mut(cog_id) {
            entry.services = services;
            entry.lobbys = lobbys;
            entry.last_seen = Utc::now();
        }
    }

    /// Récupère un COG.
    pub async fn get_cog(&self, cog_id: &str) -> Option<CogEntry> {
        let cogs = self.cogs.read().await;
        cogs.get(cog_id).cloned()
    }

    /// Liste tous les COGs.
    pub async fn list_cogs(&self) -> Vec<CogEntry> {
        let cogs = self.cogs.read().await;
        cogs.values().cloned().collect()
    }

    /// Liste les lobbys publics.
    pub async fn list_public_lobbys(&self) -> Vec<(String, LobbyEntry)> {
        let cogs = self.cogs.read().await;
        let mut lobbys = Vec::new();
        for (cog_id, entry) in cogs.iter() {
            for lobby in &entry.lobbys {
                if lobby.is_public {
                    lobbys.push((cog_id.clone(), lobby.clone()));
                }
            }
        }
        lobbys
    }

    /// Compte le nombre de COGs.
    pub async fn count(&self) -> usize {
        let cogs = self.cogs.read().await;
        cogs.len()
    }

    /// Nettoie les COGs inactifs.
    pub async fn cleanup_inactive(&self, timeout_seconds: i64) -> usize {
        let now = Utc::now();
        let mut cogs = self.cogs.write().await;
        let before = cogs.len();

        cogs.retain(|_, entry| {
            (now - entry.last_seen).num_seconds() < timeout_seconds
        });

        before - cogs.len()
    }

    /// Met à jour le timestamp last_seen d'un COG.
    pub async fn touch_cog(&self, cog_id: &str) {
        let mut cogs = self.cogs.write().await;
        if let Some(entry) = cogs.get_mut(cog_id) {
            entry.last_seen = Utc::now();
        }
    }

    /// Marque un COG comme absent (sans le supprimer).
    /// Met last_seen dans le passé pour qu'il soit affiché comme "absent" dans le catalogue.
    pub async fn mark_absent(&self, cog_id: &str) -> bool {
        let mut cogs = self.cogs.write().await;
        if let Some(entry) = cogs.get_mut(cog_id) {
            // Met last_seen à une heure dans le passé pour être sûr qu'il soit marqué absent
            entry.last_seen = Utc::now() - chrono::Duration::hours(1);
            debug!("Marked COG {} as absent in pool {}", cog_id, self.version);
            true
        } else {
            false
        }
    }
}

/// Gestionnaire de pools.
#[derive(Debug)]
pub struct PoolManager {
    /// Pools par version.
    pools: Arc<RwLock<HashMap<String, Arc<Pool>>>>,
    /// Activer l'isolation par version.
    enable_version_isolation: bool,
}

impl PoolManager {
    /// Crée un nouveau gestionnaire.
    #[must_use]
    pub fn new(enable_version_isolation: bool) -> Self {
        Self {
            pools: Arc::new(RwLock::new(HashMap::new())),
            enable_version_isolation,
        }
    }

    /// Récupère ou crée un pool pour une version.
    pub async fn get_or_create_pool(&self, version: &str) -> Arc<Pool> {
        let effective_version = if self.enable_version_isolation {
            version.to_string()
        } else {
            "default".to_string()
        };

        let mut pools = self.pools.write().await;

        if let Some(pool) = pools.get(&effective_version) {
            Arc::clone(pool)
        } else {
            let pool = Arc::new(Pool::new(effective_version.clone()));
            pools.insert(effective_version.clone(), Arc::clone(&pool));
            info!("Created pool for version: {}", effective_version);
            pool
        }
    }

    /// Liste toutes les versions de pools.
    pub async fn list_versions(&self) -> Vec<String> {
        let pools = self.pools.read().await;
        pools.keys().cloned().collect()
    }

    /// Récupère un pool existant par version.
    pub async fn get_pool(&self, version: &str) -> Option<Arc<Pool>> {
        let effective_version = if self.enable_version_isolation {
            version.to_string()
        } else {
            "default".to_string()
        };

        let pools = self.pools.read().await;
        pools.get(&effective_version).cloned()
    }

    /// Compte le nombre total de COGs.
    pub async fn total_cog_count(&self) -> usize {
        let pools = self.pools.read().await;
        let mut total = 0;
        for pool in pools.values() {
            total += pool.count().await;
        }
        total
    }

    /// Recherche un COG par ID dans tous les pools.
    pub async fn find_cog(&self, cog_id: &str) -> Option<CogEntry> {
        let pools = self.pools.read().await;
        for pool in pools.values() {
            if let Some(entry) = pool.get_cog(cog_id).await {
                return Some(entry);
            }
        }
        None
    }

    /// Liste tous les lobbys publics.
    pub async fn list_all_public_lobbys(&self) -> Vec<(String, String, LobbyEntry)> {
        let pools = self.pools.read().await;
        let mut all_lobbys = Vec::new();

        for (version, pool) in pools.iter() {
            for (cog_id, lobby) in pool.list_public_lobbys().await {
                all_lobbys.push((version.clone(), cog_id, lobby));
            }
        }

        all_lobbys
    }

    /// Nettoie les COGs inactifs dans tous les pools.
    pub async fn cleanup_all(&self, timeout_seconds: i64) -> usize {
        let pools = self.pools.read().await;
        let mut total = 0;
        for pool in pools.values() {
            total += pool.cleanup_inactive(timeout_seconds).await;
        }
        total
    }

    /// Clone superficiel (partage les pools internes).
    #[must_use]
    pub fn clone_shallow(&self) -> Self {
        Self {
            pools: Arc::clone(&self.pools),
            enable_version_isolation: self.enable_version_isolation,
        }
    }

    /// Liste les pools actifs.
    pub async fn list_pools(&self) -> Vec<String> {
        let pools = self.pools.read().await;
        pools.keys().cloned().collect()
    }

    /// Liste tous les COGs de tous les pools.
    pub async fn list_all_cogs(&self) -> Vec<CogEntry> {
        let pools = self.pools.read().await;
        let mut cogs = Vec::new();
        for pool in pools.values() {
            cogs.extend(pool.list_cogs().await);
        }
        cogs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_operations() {
        let pool = Pool::new("1.0.0".to_string());

        // Ajouter un COG
        let entry = CogEntry {
            cog_id: "test-cog".to_string(),
            core_version: "1.0.0".to_string(),
            address: "127.0.0.1:12345".to_string(),
            last_seen: Utc::now(),
            services: vec!["jayfestival".to_string()],
            lobbys: vec![LobbyEntry {
                lobby_id: "lobby-1".to_string(),
                name: "Test Lobby".to_string(),
                is_public: true,
                password_required: false,
                password_hash: None,
                max_players: 8,
                current_players: 1,
                metadata: "{}".to_string(),
            }],
        };

        pool.add_cog(entry).await;

        // Vérifier
        assert_eq!(pool.count().await, 1);
        assert!(pool.get_cog("test-cog").await.is_some());

        // Lister les lobbys publics
        let lobbys = pool.list_public_lobbys().await;
        assert_eq!(lobbys.len(), 1);

        // Supprimer
        pool.remove_cog("test-cog").await;
        assert_eq!(pool.count().await, 0);
    }

    #[tokio::test]
    async fn test_pool_manager() {
        let manager = PoolManager::new(true);

        // Créer des pools
        let pool1 = manager.get_or_create_pool("1.0.0").await;
        let pool2 = manager.get_or_create_pool("1.1.0").await;

        // Ajouter des COGs
        pool1.add_cog(CogEntry {
            cog_id: "cog-1".to_string(),
            core_version: "1.0.0".to_string(),
            address: "127.0.0.1:12345".to_string(),
            last_seen: Utc::now(),
            services: Vec::new(),
            lobbys: Vec::new(),
        }).await;

        pool2.add_cog(CogEntry {
            cog_id: "cog-2".to_string(),
            core_version: "1.1.0".to_string(),
            address: "127.0.0.1:12346".to_string(),
            last_seen: Utc::now(),
            services: Vec::new(),
            lobbys: Vec::new(),
        }).await;

        // Compter
        assert_eq!(manager.total_cog_count().await, 2);

        // Rechercher
        assert!(manager.find_cog("cog-1").await.is_some());
        assert!(manager.find_cog("cog-2").await.is_some());
        assert!(manager.find_cog("cog-3").await.is_none());
    }

    #[tokio::test]
    async fn test_pool_manager_no_isolation() {
        let manager = PoolManager::new(false);

        // Tous les COGs vont dans le même pool
        let pool1 = manager.get_or_create_pool("1.0.0").await;
        let pool2 = manager.get_or_create_pool("1.1.0").await;

        // C'est le même pool (version "default")
        assert_eq!(pool1.version, pool2.version);
        assert_eq!(pool1.version, "default");
    }
}
