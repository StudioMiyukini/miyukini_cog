//! Service Market — Stockage et API côté Origin.
//!
//! Gère le catalogue des services, le stockage des packages et les endpoints REST.
//!
//! # Routes
//!
//! | Méthode | Route                                | Description                        |
//! |---------|--------------------------------------|------------------------------------|
//! | GET     | /api/market/catalog                  | Catalogue complet (off. + tiers)   |
//! | GET     | /api/market/catalog/{id}             | Détail d'un service                |
//! | GET     | /api/market/catalog/search?q=...     | Recherche                          |
//! | GET     | /api/market/package/{id}/{version}   | Téléchargement du package          |
//! | POST    | /api/market/publish                  | Publication d'un package           |
//! | DELETE  | /api/market/package/{id}/{version}   | Retrait d'un package               |

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Utc;
use rusqlite::Connection;
use tokio::sync::Mutex;

use miyumarket::manifest::{ServiceManifest, ServiceSource, ServiceType};
use miyumarket::package;
use miyumarket::protocol::{
    MarketCatalog, MarketEntry, MarketResponse, MarketSearchResult, PublishResponse,
    UnpublishResponse,
};

// ═══════════════════════════════════════════════════════════════════════════
// MarketStore — Persistence SQLite + stockage fichiers
// ═══════════════════════════════════════════════════════════════════════════

/// Stockage Market sur Origin (catalogue + packages).
pub struct MarketStore {
    /// Base de données SQLite pour le catalogue.
    db: Arc<Mutex<Connection>>,
    /// Répertoire racine pour le stockage des packages.
    packages_dir: PathBuf,
}

impl MarketStore {
    /// Ouvre (ou crée) le MarketStore.
    pub fn open(db_path: impl AsRef<Path>, packages_dir: impl AsRef<Path>) -> Result<Self, String> {
        let conn =
            Connection::open(db_path.as_ref()).map_err(|e| format!("Market DB open: {e}"))?;

        let store = Self {
            db: Arc::new(Mutex::new(conn)),
            packages_dir: packages_dir.as_ref().to_path_buf(),
        };
        store.init_schema_sync()?;
        store.seed_official_catalog_sync()?;
        Ok(store)
    }

    /// Initialise le schéma de la base.
    fn init_schema_sync(&self) -> Result<(), String> {
        // On bloque ici car c'est appelé à l'init (avant le runtime async)
        let conn = self.db.blocking_lock();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS market_services (
                id              TEXT NOT NULL,
                version         TEXT NOT NULL,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL DEFAULT '',
                icon            TEXT NOT NULL DEFAULT '',
                service_type    TEXT NOT NULL DEFAULT 'interne_cog',
                source          TEXT NOT NULL DEFAULT 'officiel',
                developer       TEXT NOT NULL DEFAULT '',
                min_central_ver TEXT NOT NULL DEFAULT '0.1.0',
                dependencies    TEXT NOT NULL DEFAULT '[]',
                permissions     TEXT NOT NULL DEFAULT '[]',
                tags            TEXT NOT NULL DEFAULT '[]',
                homepage        TEXT,
                checksum        TEXT,
                package_size    INTEGER DEFAULT 0,
                download_count  INTEGER NOT NULL DEFAULT 0,
                downloadable    INTEGER NOT NULL DEFAULT 0,
                published_at    TEXT NOT NULL,
                PRIMARY KEY (id, version)
            );

            CREATE INDEX IF NOT EXISTS idx_market_source ON market_services(source);
            CREATE INDEX IF NOT EXISTS idx_market_developer ON market_services(developer);",
        )
        .map_err(|e| format!("Market schema init: {e}"))?;
        Ok(())
    }

    /// Insère le catalogue officiel Miyukini (ne fait rien si les entrées existent déjà).
    fn seed_official_catalog_sync(&self) -> Result<(), String> {
        let conn = self.db.blocking_lock();
        let now = Utc::now().to_rfc3339();

        let services = official_catalog_seed();
        for m in &services {
            let stype = serde_json::to_string(&m.service_type).unwrap_or_default();
            let source = serde_json::to_string(&m.source).unwrap_or_default();
            let deps = serde_json::to_string(&m.dependencies).unwrap_or_default();
            let perms = serde_json::to_string(&m.permissions).unwrap_or_default();
            let tags = serde_json::to_string(&m.tags).unwrap_or_default();

            conn.execute(
                "INSERT OR IGNORE INTO market_services
                 (id, version, name, description, icon, service_type, source, developer,
                  min_central_ver, dependencies, permissions, tags, homepage,
                  checksum, package_size, download_count, downloadable, published_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, 0, 1, ?16)",
                rusqlite::params![
                    m.id, m.version, m.name, m.description, m.icon,
                    stype.trim_matches('"'), source.trim_matches('"'),
                    m.developer, m.min_central_version,
                    deps, perms, tags,
                    m.homepage.as_deref().unwrap_or(""),
                    m.checksum.as_deref().unwrap_or(""),
                    m.package_size.unwrap_or(0) as i64,
                    now,
                ],
            ).map_err(|e| format!("Market seed: {e}"))?;
        }
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Lecture
    // ═══════════════════════════════════════════════════════════════════════

    /// Retourne le catalogue complet.
    pub async fn get_catalog(&self) -> MarketCatalog {
        let conn = self.db.lock().await;
        let mut official = Vec::new();
        let mut community = Vec::new();

        if let Ok(mut stmt) = conn.prepare(
            "SELECT id, version, name, description, icon, service_type, source, developer,
                    min_central_ver, dependencies, permissions, tags, homepage,
                    checksum, package_size, download_count, downloadable, published_at
             FROM market_services ORDER BY name",
        ) {
            if let Ok(rows) = stmt.query_map([], |row| Ok(row_to_entry(row))) {
                for row in rows.flatten() {
                    if let Ok(entry) = row {
                        if entry.manifest.source == ServiceSource::Officiel {
                            official.push(entry);
                        } else {
                            community.push(entry);
                        }
                    }
                }
            }
        }

        MarketCatalog {
            official,
            community,
            generated_at: Utc::now().to_rfc3339(),
        }
    }

    /// Retourne le détail d'un service (dernière version).
    pub async fn get_service(&self, service_id: &str) -> Option<MarketEntry> {
        let conn = self.db.lock().await;
        let mut stmt = conn
            .prepare(
                "SELECT id, version, name, description, icon, service_type, source, developer,
                    min_central_ver, dependencies, permissions, tags, homepage,
                    checksum, package_size, download_count, downloadable, published_at
             FROM market_services WHERE id = ?1 ORDER BY version DESC LIMIT 1",
            )
            .ok()?;

        stmt.query_row(rusqlite::params![service_id], |row| Ok(row_to_entry(row)))
            .ok()?
            .ok()
    }

    /// Recherche dans le catalogue.
    pub async fn search(&self, query: &str) -> MarketSearchResult {
        let conn = self.db.lock().await;
        let pattern = format!("%{query}%");
        let mut results = Vec::new();

        if let Ok(mut stmt) = conn.prepare(
            "SELECT id, version, name, description, icon, service_type, source, developer,
                    min_central_ver, dependencies, permissions, tags, homepage,
                    checksum, package_size, download_count, downloadable, published_at
             FROM market_services
             WHERE name LIKE ?1 OR description LIKE ?1 OR id LIKE ?1 OR tags LIKE ?1
             ORDER BY download_count DESC",
        ) {
            if let Ok(rows) =
                stmt.query_map(rusqlite::params![pattern], |row| Ok(row_to_entry(row)))
            {
                for row in rows.flatten() {
                    if let Ok(entry) = row {
                        results.push(entry);
                    }
                }
            }
        }

        let total = results.len();
        MarketSearchResult {
            results,
            query: query.to_string(),
            total,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Publication
    // ═══════════════════════════════════════════════════════════════════════

    /// Publie un package (manifeste + données brutes).
    pub async fn publish(
        &self,
        manifest: &ServiceManifest,
        package_data: &[u8],
        developer_token: &str,
    ) -> Result<PublishResponse, String> {
        // 1. Valider le manifeste
        manifest
            .validate()
            .map_err(|e| format!("Manifeste invalide : {e}"))?;

        // 2. Vérifier le token développeur
        self.verify_developer_token(developer_token, &manifest.developer)
            .await?;

        // 3. Calculer le checksum et valider le package
        let info = package::validate_package(package_data, manifest)
            .map_err(|e| format!("Package invalide : {e}"))?;

        // 4. Stocker le fichier package
        let full_path = self.packages_dir.join(&info.storage_path);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Création répertoire package : {e}"))?;
        }
        std::fs::write(&full_path, package_data).map_err(|e| format!("Écriture package : {e}"))?;

        // 5. Enregistrer en base
        let conn = self.db.lock().await;
        let stype = serde_json::to_string(&manifest.service_type).unwrap_or_default();
        let source = serde_json::to_string(&manifest.source).unwrap_or_default();
        let deps = serde_json::to_string(&manifest.dependencies).unwrap_or_default();
        let perms = serde_json::to_string(&manifest.permissions).unwrap_or_default();
        let tags = serde_json::to_string(&manifest.tags).unwrap_or_default();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO market_services
             (id, version, name, description, icon, service_type, source, developer,
              min_central_ver, dependencies, permissions, tags, homepage,
              checksum, package_size, download_count, downloadable, published_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, 0, 1, ?16)",
            rusqlite::params![
                manifest.id,
                manifest.version,
                manifest.name,
                manifest.description,
                manifest.icon,
                stype.trim_matches('"'),
                source.trim_matches('"'),
                manifest.developer,
                manifest.min_central_version,
                deps,
                perms,
                tags,
                manifest.homepage.as_deref().unwrap_or(""),
                info.checksum,
                info.size as i64,
                now,
            ],
        )
        .map_err(|e| format!("Insertion Market DB : {e}"))?;

        Ok(PublishResponse {
            service_id: manifest.id.clone(),
            version: manifest.version.clone(),
            checksum: info.checksum,
            package_size: info.size,
        })
    }

    /// Retire un package du Market.
    pub async fn unpublish(
        &self,
        service_id: &str,
        version: &str,
        developer_token: &str,
    ) -> Result<UnpublishResponse, String> {
        // Vérifier que le service existe et que le token est valide
        let entry = self
            .get_service(service_id)
            .await
            .ok_or_else(|| format!("Service '{service_id}' introuvable"))?;
        self.verify_developer_token(developer_token, &entry.manifest.developer)
            .await?;

        // Supprimer le fichier package
        let path = package::storage_path(service_id, version);
        let full_path = self.packages_dir.join(&path);
        if full_path.exists() {
            let _ = std::fs::remove_file(&full_path);
        }

        // Supprimer de la base
        let conn = self.db.lock().await;
        conn.execute(
            "DELETE FROM market_services WHERE id = ?1 AND version = ?2",
            rusqlite::params![service_id, version],
        )
        .map_err(|e| format!("Suppression Market DB : {e}"))?;

        Ok(UnpublishResponse {
            service_id: service_id.to_string(),
            version: version.to_string(),
        })
    }

    /// Lit le contenu binaire d'un package pour le téléchargement.
    pub async fn get_package_bytes(&self, service_id: &str, version: &str) -> Option<Vec<u8>> {
        // Incrémenter le compteur de téléchargements
        {
            let conn = self.db.lock().await;
            let _ = conn.execute(
                "UPDATE market_services SET download_count = download_count + 1 WHERE id = ?1 AND version = ?2",
                rusqlite::params![service_id, version],
            );
        }

        let path = package::storage_path(service_id, version);
        let full_path = self.packages_dir.join(&path);
        std::fs::read(&full_path).ok()
    }

    /// Vérifie un token développeur.
    async fn verify_developer_token(
        &self,
        token: &str,
        expected_developer: &str,
    ) -> Result<(), String> {
        // Phase 1 : vérification simple par token prédéfini
        // Phase 2 : intégration avec le système d'authentification Central/Origin
        if token.is_empty() {
            return Err("Token développeur requis".into());
        }
        // Pour les services officiels Miyukini, le token est vérifié contre la config Origin
        // Pour les tiers, le token est vérifié contre le registre développeurs
        let _ = expected_developer; // sera utilisé en Phase 2
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Convertit une ligne SQLite en `MarketEntry`.
fn row_to_entry(row: &rusqlite::Row<'_>) -> Result<MarketEntry, rusqlite::Error> {
    let id: String = row.get(0)?;
    let version: String = row.get(1)?;
    let name: String = row.get(2)?;
    let description: String = row.get(3)?;
    let icon: String = row.get(4)?;
    let service_type_str: String = row.get(5)?;
    let source_str: String = row.get(6)?;
    let developer: String = row.get(7)?;
    let min_central_version: String = row.get(8)?;
    let dependencies_json: String = row.get(9)?;
    let permissions_json: String = row.get(10)?;
    let tags_json: String = row.get(11)?;
    let homepage: Option<String> = row.get(12)?;
    let checksum: Option<String> = row.get(13)?;
    let package_size: Option<i64> = row.get(14)?;
    let download_count: i64 = row.get(15)?;
    let downloadable: bool = row.get(16)?;
    let published_at: String = row.get(17)?;

    let service_type = match service_type_str.as_str() {
        "surface_web" => ServiceType::SurfaceWeb,
        "inter_cog" => ServiceType::InterCog,
        _ => ServiceType::InterneCog,
    };
    let source = match source_str.as_str() {
        "tiers" => ServiceSource::Tiers,
        _ => ServiceSource::Officiel,
    };
    let dependencies: Vec<String> = serde_json::from_str(&dependencies_json).unwrap_or_default();
    let permissions = serde_json::from_str(&permissions_json).unwrap_or_default();
    let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();

    let homepage = homepage.filter(|h| !h.is_empty());
    let checksum = checksum.filter(|c| !c.is_empty());

    Ok(MarketEntry {
        manifest: ServiceManifest {
            id,
            name,
            description,
            icon,
            version,
            service_type,
            source,
            developer,
            min_central_version,
            dependencies,
            permissions,
            tags,
            homepage,
            checksum,
            package_size: package_size.map(|s| s as u64),
            execution_mode: miyumarket::manifest::ExecutionMode::default(),
            binary_name: None,
            platforms: miyumarket::manifest::PlatformBinaries::default(),
        },
        published_at,
        download_count: download_count as u64,
        downloadable,
    })
}

/// Catalogue officiel Miyukini pré-rempli dans la base Market.
fn official_catalog_seed() -> Vec<ServiceManifest> {
    use miyumarket::manifest::{ExecutionMode, PlatformBinaries, ServicePermission};

    vec![
        ServiceManifest {
            id: "jayxpose".into(), name: "JayXpose".into(),
            description: "Profil exposant, catalogue produits, vitrine, coffre-fort documentaire".into(),
            icon: "\u{1F3EA}".into(), version: "0.1.0".into(),
            service_type: ServiceType::SurfaceWeb, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage, ServicePermission::Webway, ServicePermission::Identity],
            tags: vec!["commerce".into(), "exposant".into(), "vitrine".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("jayxpose.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("jayxpose.exe".into()), ..Default::default() },
        },
        ServiceManifest {
            id: "jayfestival".into(), name: "JayFestival".into(),
            description: "Festivals, éditions, exposants, visiteurs".into(),
            icon: "\u{1F4C5}".into(), version: "0.1.0".into(),
            service_type: ServiceType::SurfaceWeb, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage, ServicePermission::Webway, ServicePermission::Identity, ServicePermission::Notifications],
            tags: vec!["festival".into(), "evenement".into(), "exposant".into(), "visiteur".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("jayfestival.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("jayfestival.exe".into()), ..Default::default() },
        },
        ServiceManifest {
            id: "jaykoa".into(), name: "JayKoa".into(),
            description: "Calendrier universel du COG, récepteur temporel transversal".into(),
            icon: "\u{1F4C6}".into(), version: "0.1.0".into(),
            service_type: ServiceType::InterneCog, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage, ServicePermission::Calendar],
            tags: vec!["calendrier".into(), "agenda".into(), "planning".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("jaykoa.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("jaykoa.exe".into()), ..Default::default() },
        },
        ServiceManifest {
            id: "jaykonta".into(), name: "JayKonta".into(),
            description: "Comptabilité COG unifiée Purse + Account".into(),
            icon: "\u{1F9EE}".into(), version: "0.1.0".into(),
            service_type: ServiceType::InterneCog, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage, ServicePermission::Accounting],
            tags: vec!["comptabilite".into(), "budget".into(), "purse".into(), "account".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("jaykonta.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("jaykonta.exe".into()), ..Default::default() },
        },
        ServiceManifest {
            id: "miyukiniwatch".into(), name: "MiyukiniWatch".into(),
            description: "Tes habitudes et tes mesures — consulte, comprends, efface.".into(),
            icon: "\u{1F441}".into(), version: "0.1.0".into(),
            service_type: ServiceType::InterneCog, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage],
            tags: vec!["habitudes".into(), "mesure".into(), "miou".into(), "usage".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Background,
            binary_name: Some("miyukiniwatch.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("miyukiniwatch.exe".into()), ..Default::default() },
        },
        ServiceManifest {
            id: "jay1tribu".into(), name: "Jay1Tribu".into(),
            description: "Tribus, amis et discussions — chat et tribu pleins uniquement si connecté au Webway.".into(),
            icon: "\u{1F4AC}".into(), version: "0.1.0".into(),
            service_type: ServiceType::InterCog, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage, ServicePermission::Webway, ServicePermission::Identity],
            tags: vec!["chat".into(), "messagerie".into(), "tribu".into(), "amis".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("jay1tribu.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("jay1tribu.exe".into()), ..Default::default() },
        },
        ServiceManifest {
            id: "jaymanga".into(), name: "JayManga".into(),
            description: "Lecture et vente de manga en ligne — catalogue, lecteur, boutique, portail agrégé".into(),
            icon: "\u{1F4DA}".into(), version: "0.1.0".into(),
            service_type: ServiceType::SurfaceWeb, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage, ServicePermission::Webway, ServicePermission::Identity],
            tags: vec!["manga".into(), "lecture".into(), "boutique".into(), "lecteur".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("jaymanga.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("jaymanga.exe".into()), ..Default::default() },
        },
        ServiceManifest {
            id: "miyuclicker".into(), name: "Lord of the Click".into(),
            description: "Premier jeu officiel Miyukini (Idle/Clicker + Carte stratégique)".into(),
            icon: "\u{1F3AE}".into(), version: "0.1.0".into(),
            service_type: ServiceType::InterCog, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage],
            tags: vec!["jeu".into(), "idle".into(), "clicker".into(), "strategie".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("miyuclicker.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("miyuclicker.exe".into()), ..Default::default() },
        },
        ServiceManifest {
            id: "lord_of_the_castle".into(), name: "Miyukini Survivor".into(),
            description: "Jeu Survivor/Tower Defense officiel Miyukini".into(),
            icon: "\u{1F3F0}".into(), version: "0.1.0".into(),
            service_type: ServiceType::InterCog, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage],
            tags: vec!["jeu".into(), "survivor".into(), "tower-defense".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("lord_of_the_castle.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("lord_of_the_castle.exe".into()), ..Default::default() },
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════
// API handlers
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/market/catalog — Catalogue complet.
pub async fn api_market_catalog(store: &MarketStore) -> String {
    let catalog = store.get_catalog().await;
    let resp = MarketResponse::ok(catalog);
    serde_json::to_string_pretty(&resp).unwrap_or_else(|_| r#"{"success":false}"#.into())
}

/// GET /api/market/catalog/{id} — Détail d'un service.
pub async fn api_market_service(store: &MarketStore, service_id: &str) -> Option<String> {
    let entry = store.get_service(service_id).await?;
    let resp = MarketResponse::ok(entry);
    Some(serde_json::to_string_pretty(&resp).unwrap_or_else(|_| r#"{"success":false}"#.into()))
}

/// GET /api/market/catalog/search?q=... — Recherche.
pub async fn api_market_search(store: &MarketStore, query: &str) -> String {
    let results = store.search(query).await;
    let resp = MarketResponse::ok(results);
    serde_json::to_string_pretty(&resp).unwrap_or_else(|_| r#"{"success":false}"#.into())
}

/// GET /api/market/package/{id}/{version} — Téléchargement binaire.
pub async fn api_market_download(
    store: &MarketStore,
    service_id: &str,
    version: &str,
) -> Option<Vec<u8>> {
    store.get_package_bytes(service_id, version).await
}

/// POST /api/market/publish — Publication (body JSON).
pub async fn api_market_publish(store: &MarketStore, body: &str) -> String {
    let req: Result<miyumarket::protocol::PublishRequest, _> = serde_json::from_str(body);
    match req {
        Ok(publish_req) => {
            // Phase 1 : le package data est vide (seul le manifeste est enregistré)
            // Phase 2 : le body sera multipart avec le fichier .msp
            match store
                .publish(&publish_req.manifest, &[], &publish_req.developer_token)
                .await
            {
                Ok(resp) => {
                    let r = MarketResponse::ok(resp);
                    serde_json::to_string_pretty(&r).unwrap_or_default()
                }
                Err(e) => {
                    let r = MarketResponse::<PublishResponse>::err(e);
                    serde_json::to_string_pretty(&r).unwrap_or_default()
                }
            }
        }
        Err(e) => {
            let r = MarketResponse::<PublishResponse>::err(format!("JSON invalide : {e}"));
            serde_json::to_string_pretty(&r).unwrap_or_default()
        }
    }
}

/// DELETE /api/market/package/{id}/{version} — Retrait.
pub async fn api_market_unpublish(
    store: &MarketStore,
    service_id: &str,
    version: &str,
    body: &str,
) -> String {
    // Le body contient le developer_token
    #[derive(serde::Deserialize)]
    struct TokenBody {
        developer_token: String,
    }

    let token = serde_json::from_str::<TokenBody>(body)
        .map(|t| t.developer_token)
        .unwrap_or_default();

    match store.unpublish(service_id, version, &token).await {
        Ok(resp) => {
            let r = MarketResponse::ok(resp);
            serde_json::to_string_pretty(&r).unwrap_or_default()
        }
        Err(e) => {
            let r = MarketResponse::<UnpublishResponse>::err(e);
            serde_json::to_string_pretty(&r).unwrap_or_default()
        }
    }
}
