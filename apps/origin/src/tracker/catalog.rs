//! Catalogue web MWS.
//!
//! Expose les services et lobbys via HTTP.

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::pool::PoolManager;

/// Entrée du catalogue de services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogServiceEntry {
    /// Identifiant du service.
    pub service_id: String,
    /// Version du service.
    pub version: String,
    /// Description.
    pub description: String,
    /// Catégorie.
    pub category: String,
    /// URL de téléchargement.
    pub download_url: Option<String>,
    /// Hash SHA-256.
    pub checksum: Option<String>,
    /// Date de publication.
    pub published_at: String,
}

/// Entrée du catalogue de lobbys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogLobbyEntry {
    /// Version du pool.
    pub pool_version: String,
    /// Identifiant du COG hôte.
    pub host_cog_id: String,
    /// Identifiant du Lobby.
    pub lobby_id: String,
    /// Nom du Lobby.
    pub name: String,
    /// Mot de passe requis.
    pub password_required: bool,
    /// Nombre de places.
    pub max_players: u32,
    /// Nombre de joueurs actuels.
    pub current_players: u32,
    /// Métadonnées.
    pub metadata: serde_json::Value,
}

/// Catalogue MWS.
pub struct Catalog {
    /// Gestionnaire de pools.
    pool_manager: Arc<PoolManager>,
    /// Services enregistrés.
    services: Arc<RwLock<Vec<CatalogServiceEntry>>>,
}

impl Catalog {
    /// Crée un nouveau catalogue.
    #[must_use]
    pub fn new(pool_manager: Arc<PoolManager>) -> Self {
        Self {
            pool_manager,
            services: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Enregistre un service.
    pub async fn register_service(&self, entry: CatalogServiceEntry) {
        let mut services = self.services.write().await;
        // Remplacer si existe déjà
        services.retain(|s| s.service_id != entry.service_id);
        services.push(entry);
    }

    /// Liste tous les services.
    pub async fn list_services(&self) -> Vec<CatalogServiceEntry> {
        let services = self.services.read().await;
        services.clone()
    }

    /// Recherche des services.
    pub async fn search_services(&self, query: &str) -> Vec<CatalogServiceEntry> {
        let services = self.services.read().await;
        let query_lower = query.to_lowercase();
        services
            .iter()
            .filter(|s| {
                s.service_id.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
                    || s.category.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Récupère un service par ID.
    pub async fn get_service(&self, service_id: &str) -> Option<CatalogServiceEntry> {
        let services = self.services.read().await;
        services.iter().find(|s| s.service_id == service_id).cloned()
    }

    /// Liste tous les lobbys publics.
    pub async fn list_lobbys(&self) -> Vec<CatalogLobbyEntry> {
        let all_lobbys = self.pool_manager.list_all_public_lobbys().await;
        all_lobbys
            .into_iter()
            .map(|(version, cog_id, lobby)| CatalogLobbyEntry {
                pool_version: version,
                host_cog_id: cog_id,
                lobby_id: lobby.lobby_id,
                name: lobby.name,
                password_required: lobby.password_required,
                max_players: lobby.max_players,
                current_players: lobby.current_players,
                metadata: serde_json::from_str(&lobby.metadata).unwrap_or(serde_json::Value::Null),
            })
            .collect()
    }

    /// Recherche des lobbys.
    pub async fn search_lobbys(&self, query: &str) -> Vec<CatalogLobbyEntry> {
        let all_lobbys = self.list_lobbys().await;
        let query_lower = query.to_lowercase();
        all_lobbys
            .into_iter()
            .filter(|l| l.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Génère le JSON du catalogue complet.
    pub async fn to_json(&self) -> serde_json::Value {
        let services = self.list_services().await;
        let lobbys = self.list_lobbys().await;

        serde_json::json!({
            "services": services,
            "lobbys": lobbys,
            "pool_stats": {
                "total_cogs": self.pool_manager.total_cog_count().await,
                "versions": self.pool_manager.list_versions().await,
            },
            "generated_at": chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Crée un catalogue à partir d'un PoolManager.
    pub async fn from_pools(pool_manager: &PoolManager) -> Self {
        Self {
            pool_manager: Arc::new(pool_manager.clone_shallow()),
            services: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Génère la page HTML du catalogue.
    pub fn to_html(&self) -> String {
        // Version synchrone avec des données vides pour le moment
        // Le contenu réel est chargé via JS/API
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Catalogue — Miyukini Webway</title>
    <style>
        :root {
            --primary: #8b5cf6;
            --bg: #0a0a0f;
            --bg-surface: #12121a;
            --text: #f0f0f5;
            --text-muted: #9ca3af;
            --border: rgba(139, 92, 246, 0.2);
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: 'Inter', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            padding: 2rem;
        }
        .container { max-width: 1200px; margin: 0 auto; }
        h1 {
            font-size: 2rem;
            margin-bottom: 0.5rem;
            background: linear-gradient(135deg, var(--primary), #06b6d4);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        .subtitle { color: var(--text-muted); margin-bottom: 2rem; }
        .card {
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 0.75rem;
            padding: 1.5rem;
            margin-bottom: 1.5rem;
        }
        .card h2 { font-size: 1.25rem; margin-bottom: 1rem; }
        table { width: 100%; border-collapse: collapse; }
        th, td { padding: 0.75rem; text-align: left; }
        th { color: var(--text-muted); font-weight: 500; border-bottom: 1px solid var(--border); }
        tr:hover { background: rgba(139, 92, 246, 0.1); }
        .empty { color: var(--text-muted); text-align: center; padding: 2rem; }
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }
        .stat {
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 0.5rem;
            padding: 1rem;
            text-align: center;
        }
        .stat-value { font-size: 2rem; font-weight: 600; color: var(--primary); }
        .stat-label { color: var(--text-muted); font-size: 0.875rem; }
        .api-links { margin-top: 2rem; }
        .api-links a {
            color: var(--primary);
            text-decoration: none;
            margin-right: 1rem;
        }
        .api-links a:hover { text-decoration: underline; }
        .back-link { margin-bottom: 2rem; }
        .back-link a { color: var(--text-muted); }
    </style>
</head>
<body>
    <div class="container">
        <div class="back-link">
            <a href="/">← Retour à l'accueil</a>
        </div>
        <h1>📡 Catalogue MWS</h1>
        <p class="subtitle">Services et lobbys disponibles sur le réseau</p>

        <div class="stats">
            <div class="stat">
                <div class="stat-value" id="services-count">-</div>
                <div class="stat-label">Services</div>
            </div>
            <div class="stat">
                <div class="stat-value" id="lobbys-count">-</div>
                <div class="stat-label">Lobbys publics</div>
            </div>
            <div class="stat">
                <div class="stat-value" id="cogs-count">-</div>
                <div class="stat-label">COGs connectés</div>
            </div>
        </div>

        <div class="card">
            <h2>📦 Services enregistrés</h2>
            <div id="services-table">
                <p class="empty">Chargement...</p>
            </div>
        </div>

        <div class="card">
            <h2>🎮 Lobbys publics</h2>
            <div id="lobbys-table">
                <p class="empty">Chargement...</p>
            </div>
        </div>

        <div class="api-links">
            <strong>API :</strong>
            <a href="/api/catalog">Catalogue complet</a>
            <a href="/api/status">Statut serveur</a>
        </div>
    </div>

    <script>
        async function loadCatalog() {
            try {
                const resp = await fetch('/api/catalog');
                const data = await resp.json();
                
                document.getElementById('services-count').textContent = data.services?.length || 0;
                document.getElementById('lobbys-count').textContent = data.lobbys?.length || 0;
                document.getElementById('cogs-count').textContent = data.pool_stats?.total_cogs || 0;

                const servicesEl = document.getElementById('services-table');
                if (data.services && data.services.length > 0) {
                    servicesEl.innerHTML = `<table>
                        <thead><tr><th>ID</th><th>Version</th><th>Catégorie</th><th>Description</th></tr></thead>
                        <tbody>${data.services.map(s => 
                            `<tr><td>${s.service_id}</td><td>${s.version}</td><td>${s.category}</td><td>${s.description}</td></tr>`
                        ).join('')}</tbody>
                    </table>`;
                } else {
                    servicesEl.innerHTML = '<p class="empty">Aucun service enregistré</p>';
                }

                const lobbysEl = document.getElementById('lobbys-table');
                if (data.lobbys && data.lobbys.length > 0) {
                    lobbysEl.innerHTML = `<table>
                        <thead><tr><th>Nom</th><th>Hôte</th><th>Joueurs</th><th>Accès</th></tr></thead>
                        <tbody>${data.lobbys.map(l => 
                            `<tr><td>${l.name}</td><td>${l.host_cog_id}</td><td>${l.current_players}/${l.max_players}</td><td>${l.password_required ? '🔒' : '🌐'}</td></tr>`
                        ).join('')}</tbody>
                    </table>`;
                } else {
                    lobbysEl.innerHTML = '<p class="empty">Aucun lobby actif</p>';
                }
            } catch (e) {
                console.error('Failed to load catalog:', e);
            }
        }
        loadCatalog();
        setInterval(loadCatalog, 30000);
    </script>
</body>
</html>"#.to_string()
    }
}

/// Réponse de recherche.
#[derive(Debug, Serialize)]
pub struct SearchResponse<T> {
    /// Résultats.
    pub results: Vec<T>,
    /// Nombre total.
    pub total: usize,
    /// Requête.
    pub query: String,
}

impl<T> SearchResponse<T> {
    /// Crée une nouvelle réponse de recherche.
    pub fn new(results: Vec<T>, query: String) -> Self {
        let total = results.len();
        Self {
            results,
            total,
            query,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_catalog_services() {
        let pool_manager = Arc::new(PoolManager::new(true));
        let catalog = Catalog::new(pool_manager);

        // Enregistrer un service
        catalog.register_service(CatalogServiceEntry {
            service_id: "jayfestival".to_string(),
            version: "1.0.0".to_string(),
            description: "Music festival management".to_string(),
            category: "entertainment".to_string(),
            download_url: None,
            checksum: None,
            published_at: chrono::Utc::now().to_rfc3339(),
        }).await;

        // Lister
        let services = catalog.list_services().await;
        assert_eq!(services.len(), 1);

        // Rechercher
        let results = catalog.search_services("festival").await;
        assert_eq!(results.len(), 1);

        let results = catalog.search_services("unknown").await;
        assert_eq!(results.len(), 0);

        // Récupérer
        assert!(catalog.get_service("jayfestival").await.is_some());
        assert!(catalog.get_service("unknown").await.is_none());
    }

    #[tokio::test]
    async fn test_catalog_json() {
        let pool_manager = Arc::new(PoolManager::new(true));
        let catalog = Catalog::new(pool_manager);

        let json = catalog.to_json().await;
        assert!(json.get("services").is_some());
        assert!(json.get("lobbys").is_some());
        assert!(json.get("pool_stats").is_some());
    }
}
