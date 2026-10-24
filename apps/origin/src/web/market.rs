//! @id     origin_web_market
//! @do     definir_market_store
//! @role   orchestration
//! @layer  app
//! @human  origin — module « web market ».

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
//!
//! # Sécurité (fail-closed, inventaire #14 : plus de jeton partagé)
//!
//! - publish / unpublish sont des **requêtes signées** ([`miyauth::http`], usages
//!   `origin/market-publication` et `origin/market-retrait`) par une clé d'éditeur
//!   **officiel** épinglée dans `[market] editeurs_officiels` (décision 4 : paquets
//!   officiels uniquement). Liste vide → routes désactivées (503). Les en-têtes
//!   (clé épinglée, horodatage ± 5 min, nonce non rejoué) sont contrôlés **avant** la
//!   lecture du corps ([`MarketStore::controler_entetes`]).
//! - Le corps de publication (`miyumarket::protocol::encoder_corps_publication`)
//!   porte le manifeste du catalogue et le **paquet signé** (`MSPS`) : son sceau
//!   (`ManifestePaquet`) doit être signé par **la même** clé officielle que la
//!   requête, et couvrir l'id, la version, le binaire et l'archive.
//! - Une version publiée ne peut pas être remplacée par un contenu différent
//!   (republication identique : sans effet).
//! - `id` (`[a-z0-9_-]{1,64}`) et `version` (semver strict) sont validés avant
//!   toute construction de chemin disque (anti-traversée).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Utc;
use rusqlite::Connection;
use tokio::sync::Mutex;

use miyauth::http::{
    verifier_requete_signee, EntetesSignees, RefusHttp, RegistreNonces, ENTETE_HORODATAGE, ENTETE_NONCE,
    ENTETE_SIGNATAIRE, ENTETE_SIGNATURE,
};
use miyumarket::manifest::{ServiceManifest, ServiceSource, ServiceType};
use miyumarket::package;
use miyumarket::protocol::{
    decoder_corps_publication, MarketCatalog, MarketEntry, MarketResponse, MarketSearchResult,
    PublishResponse, UnpublishResponse, USAGE_PUBLICATION, USAGE_RETRAIT,
};
use miyumarket::signature::verifier_paquet;
use miyusceau::{AncresDeConfiance, NiveauConfiance};
use std::collections::HashSet;

use super::forum_auth::header_value;

/// Chemin signé de la publication.
pub const CHEMIN_PUBLICATION: &str = "/api/market/publish";

// ═══════════════════════════════════════════════════════════════════════════
// MarketStore — Persistence SQLite + stockage fichiers
// ═══════════════════════════════════════════════════════════════════════════

/// Stockage Market sur Origin (catalogue + packages).
pub struct MarketStore {
    /// Base de données SQLite pour le catalogue.
    db: Arc<Mutex<Connection>>,
    /// Répertoire racine pour le stockage des packages.
    packages_dir: PathBuf,
    /// Clés publiques des éditeurs officiels (vide = publication/retrait désactivés).
    editeurs_officiels: HashSet<[u8; 32]>,
    /// Nonces des requêtes signées déjà acceptées.
    nonces: RegistreNonces,
}

/// Refus d'une opération d'écriture du Market : statut HTTP + message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusMarket {
    /// Statut HTTP (400, 401, 409, 503…).
    pub statut: u16,
    /// Message.
    pub message: String,
}

impl RefusMarket {
    fn new(statut: u16, message: impl Into<String>) -> Self {
        Self { statut, message: message.into() }
    }

    fn auth(e: &RefusHttp) -> Self {
        tracing::warn!("Market : requête signée refusée : {e}");
        if e.statut() == 503 {
            Self::new(503, "Service saturé")
        } else {
            Self::new(401, "Requête non autorisée")
        }
    }
}

/// Valide l'identifiant d'un package : `[a-z0-9_-]{1,64}`.
#[must_use]
pub fn is_valid_service_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

/// Valide une version semver stricte : `X.Y.Z[-prerelease]` (sans métadonnée de build).
/// Pré-release : identifiants `[0-9A-Za-z-]+` séparés par des points.
#[must_use]
pub fn is_valid_version(v: &str) -> bool {
    if v.is_empty() || v.len() > 64 {
        return false;
    }
    let (core, pre) = match v.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (v, None),
    };
    let nums: Vec<&str> = core.split('.').collect();
    if nums.len() != 3 {
        return false;
    }
    let num_ok = |n: &str| {
        !n.is_empty()
            && n.len() <= 9
            && n.bytes().all(|b| b.is_ascii_digit())
            && (n == "0" || !n.starts_with('0'))
    };
    if !nums.iter().all(|n| num_ok(n)) {
        return false;
    }
    match pre {
        None => true,
        Some(p) => p.split('.').all(|id| {
            !id.is_empty() && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        }),
    }
}

impl MarketStore {
    /// Ouvre (ou crée) le MarketStore.
    pub fn open(
        db_path: impl AsRef<Path>,
        packages_dir: impl AsRef<Path>,
        editeurs_officiels: &[[u8; 32]],
    ) -> Result<Self, String> {
        std::fs::create_dir_all(packages_dir.as_ref())
            .map_err(|e| format!("Market packages dir: {e}"))?;
        let conn =
            Connection::open(db_path.as_ref()).map_err(|e| format!("Market DB open: {e}"))?;

        let store = Self {
            db: Arc::new(Mutex::new(conn)),
            packages_dir: packages_dir.as_ref().to_path_buf(),
            editeurs_officiels: editeurs_officiels.iter().copied().collect(),
            nonces: RegistreNonces::default(),
        };
        if store.editeurs_officiels.is_empty() {
            tracing::warn!(
                "[market] editeurs_officiels vide : publication/retrait Market désactivés"
            );
        }
        store.init_schema_sync()?;
        store.seed_official_catalog_sync()?;
        store.sync_package_metadata_sync()?;
        Ok(store)
    }

    /// Publication/retrait activés (au moins un éditeur officiel épinglé) ?
    #[must_use]
    pub fn publish_enabled(&self) -> bool {
        !self.editeurs_officiels.is_empty()
    }

    /// Contrôle des en-têtes signés **avant** lecture du corps : clé d'éditeur
    /// officiel épinglée, horodatage, nonce non déjà vu.
    pub fn controler_entetes(&self, headers: &[String], maintenant: u64) -> Result<EntetesSignees, RefusMarket> {
        if !self.publish_enabled() {
            return Err(RefusMarket::new(503, "Publication désactivée"));
        }
        let entetes = EntetesSignees::analyser(
            header_value(headers, ENTETE_SIGNATAIRE).as_deref(),
            header_value(headers, ENTETE_HORODATAGE).as_deref(),
            header_value(headers, ENTETE_NONCE).as_deref(),
            header_value(headers, ENTETE_SIGNATURE).as_deref(),
        )
        .map_err(|e| RefusMarket::auth(&e))?;
        if !self.editeurs_officiels.contains(&entetes.signataire) {
            return Err(RefusMarket::auth(&RefusHttp::SignataireNonAutorise));
        }
        entetes
            .controler_horodatage(maintenant, self.nonces.fenetre())
            .map_err(|e| RefusMarket::auth(&e))?;
        if self.nonces.deja_vu(&entetes.kid(), &entetes.nonce) {
            return Err(RefusMarket::auth(&RefusHttp::Rejeu));
        }
        Ok(entetes)
    }

    /// Vérifie complètement une requête signée (re-contrôle de l'épinglage inclus).
    fn verifier_requete(
        &self,
        entetes: &EntetesSignees,
        usage: &str,
        methode: &str,
        chemin: &str,
        corps: &[u8],
        maintenant: u64,
    ) -> Result<(), RefusMarket> {
        if !self.publish_enabled() {
            return Err(RefusMarket::new(503, "Publication désactivée"));
        }
        if !self.editeurs_officiels.contains(&entetes.signataire) {
            return Err(RefusMarket::auth(&RefusHttp::SignataireNonAutorise));
        }
        verifier_requete_signee(entetes, usage, methode, chemin, corps, &self.nonces, maintenant)
            .map_err(|e| RefusMarket::auth(&e))
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

    /// Met à jour `downloadable`, `checksum` et `package_size` à partir des packages présents.
    fn sync_package_metadata_sync(&self) -> Result<(), String> {
        let conn = self.db.blocking_lock();
        let mut stmt = conn
            .prepare("SELECT id, version FROM market_services")
            .map_err(|e| format!("Market package scan prepare: {e}"))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| format!("Market package scan query: {e}"))?;

        let mut entries = Vec::new();
        for row in rows {
            let (service_id, version) = row.map_err(|e| format!("Market package scan row: {e}"))?;
            entries.push((service_id, version));
        }
        drop(stmt);

        for (service_id, version) in entries {
            let package_bytes = self
                .package_path(&service_id, &version)
                .and_then(|p| std::fs::read(p).ok());
            if let Some(bytes) = package_bytes {
                let checksum = package::sha256_hex(&bytes);
                conn.execute(
                    "UPDATE market_services
                     SET downloadable = 1, checksum = ?3, package_size = ?4
                     WHERE id = ?1 AND version = ?2",
                    rusqlite::params![service_id, version, checksum, bytes.len() as i64,],
                )
                .map_err(|e| format!("Market package sync update: {e}"))?;
            } else {
                conn.execute(
                    "UPDATE market_services
                     SET downloadable = 0, checksum = '', package_size = 0
                     WHERE id = ?1 AND version = ?2",
                    rusqlite::params![service_id, version],
                )
                .map_err(|e| format!("Market package sync reset: {e}"))?;
            }
        }

        Ok(())
    }

    /// Chemin disque d'un package ; `None` si `id`/`version` sont invalides (anti-traversée).
    fn package_path(&self, service_id: &str, version: &str) -> Option<PathBuf> {
        if !is_valid_service_id(service_id) || !is_valid_version(version) {
            return None;
        }
        Some(
            self.packages_dir
                .join(service_id)
                .join(version)
                .join(package::package_filename(service_id, version)),
        )
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

    /// Publie un paquet signé (requête signée d'un éditeur officiel, voir l'en-tête du module).
    pub async fn publish(
        &self,
        entetes: &EntetesSignees,
        corps: &[u8],
        maintenant: u64,
    ) -> Result<PublishResponse, RefusMarket> {
        // 1. Requête signée par un éditeur officiel épinglé (avant tout décodage).
        self.verifier_requete(entetes, USAGE_PUBLICATION, "POST", CHEMIN_PUBLICATION, corps, maintenant)?;

        // 2. Manifeste du catalogue : valide, officiel, id/version utilisables.
        let (manifest, paquet) = decoder_corps_publication(corps).map_err(|e| RefusMarket::new(400, e))?;
        manifest
            .validate()
            .map_err(|e| RefusMarket::new(400, format!("Manifeste invalide : {e}")))?;
        if manifest.source != ServiceSource::Officiel {
            return Err(RefusMarket::new(400, "Seuls les paquets officiels sont acceptés"));
        }
        let full_path = self
            .package_path(&manifest.id, &manifest.version)
            .ok_or_else(|| RefusMarket::new(400, "Identifiant ou version de package invalide"))?;

        // 3. Sceau du paquet : même clé officielle que la requête, id/version/archive couverts.
        let mut ancres = AncresDeConfiance::new();
        ancres
            .ajouter(&entetes.signataire, NiveauConfiance::Souverain)
            .map_err(|_| RefusMarket::new(401, "Clé d'éditeur invalide"))?;
        let verifie = verifier_paquet(paquet, &manifest.id, Some(&manifest.version), &ancres, maintenant)
            .map_err(|e| RefusMarket::new(400, format!("Paquet refusé : {e}")))?;
        if verifie.manifeste().nom_binaire != manifest.binary_name {
            return Err(RefusMarket::new(400, "binary_name différent de la valeur signée"));
        }
        let checksum = package::sha256_hex(paquet);
        let size = paquet.len() as u64;

        let conn = self.db.lock().await;
        // 4. Pas de substitution d'une version déjà publiée.
        let existant: Option<(bool, String)> = conn
            .query_row(
                "SELECT downloadable, COALESCE(checksum, '') FROM market_services WHERE id = ?1 AND version = ?2",
                rusqlite::params![manifest.id, manifest.version],
                |r| Ok((r.get::<_, bool>(0)?, r.get::<_, String>(1)?)),
            )
            .ok();
        if let Some((true, ancien)) = &existant {
            if ancien != &checksum {
                return Err(RefusMarket::new(409, "Version déjà publiée avec un contenu différent"));
            }
            return Ok(PublishResponse {
                service_id: manifest.id.clone(),
                version: manifest.version.clone(),
                checksum,
                package_size: size,
            });
        }

        // 5. Écriture atomique du paquet signé (tel quel : Central revérifie le sceau).
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| RefusMarket::new(500, format!("Création répertoire package : {e}")))?;
        }
        let tmp = full_path.with_extension("tmp");
        std::fs::write(&tmp, paquet)
            .and_then(|()| std::fs::rename(&tmp, &full_path))
            .map_err(|e| RefusMarket::new(500, format!("Écriture package : {e}")))?;

        // 6. Enregistrement en base.
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
                checksum,
                i64::try_from(size).unwrap_or(i64::MAX),
                now,
            ],
        )
        .map_err(|e| RefusMarket::new(500, format!("Insertion Market DB : {e}")))?;
        tracing::info!(
            "Market : {} {} publié par l'éditeur officiel {}",
            manifest.id,
            manifest.version,
            hex::encode(&entetes.kid()[..8])
        );

        Ok(PublishResponse {
            service_id: manifest.id.clone(),
            version: manifest.version.clone(),
            checksum,
            package_size: size,
        })
    }

    /// Retire un package du Market (requête signée d'un éditeur officiel, corps vide).
    pub async fn unpublish(
        &self,
        entetes: &EntetesSignees,
        service_id: &str,
        version: &str,
        maintenant: u64,
    ) -> Result<UnpublishResponse, RefusMarket> {
        // Les paramètres de chemin ne servent au disque qu'après validation.
        let full_path = self
            .package_path(service_id, version)
            .ok_or_else(|| RefusMarket::new(400, "Identifiant ou version de package invalide"))?;
        let chemin = format!("/api/market/package/{service_id}/{version}");
        self.verifier_requete(entetes, USAGE_RETRAIT, "DELETE", &chemin, b"", maintenant)?;
        self.get_service(service_id)
            .await
            .ok_or_else(|| RefusMarket::new(404, format!("Service '{service_id}' introuvable")))?;

        if full_path.exists() {
            let _ = std::fs::remove_file(&full_path);
        }
        let conn = self.db.lock().await;
        conn.execute(
            "DELETE FROM market_services WHERE id = ?1 AND version = ?2",
            rusqlite::params![service_id, version],
        )
        .map_err(|e| RefusMarket::new(500, format!("Suppression Market DB : {e}")))?;

        Ok(UnpublishResponse {
            service_id: service_id.to_string(),
            version: version.to_string(),
        })
    }

    /// Lit le contenu binaire d'un package pour le téléchargement.
    pub async fn get_package_bytes(&self, service_id: &str, version: &str) -> Option<Vec<u8>> {
        let full_path = self.package_path(service_id, version)?;
        let bytes = std::fs::read(&full_path).ok()?;

        let conn = self.db.lock().await;
        let _ = conn.execute(
            "UPDATE market_services SET download_count = download_count + 1 WHERE id = ?1 AND version = ?2",
            rusqlite::params![service_id, version],
        );

        Some(bytes)
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
            id: "miyucloud".into(), name: "MiyuCloud".into(),
            description: "Cloud privé chiffré du COG — fichiers, portail web authentifié, sync et partage sécurisé".into(),
            icon: "\u{2601}".into(), version: "0.1.0".into(),
            service_type: ServiceType::InterCog, source: ServiceSource::Officiel,
            developer: "Miyukini".into(), min_central_version: "0.2.0".into(),
            dependencies: vec![], permissions: vec![ServicePermission::Storage, ServicePermission::Webway, ServicePermission::Identity],
            tags: vec!["cloud".into(), "fichiers".into(), "sync".into(), "partage".into(), "stockage".into()],
            homepage: None, checksum: None, package_size: None,
            execution_mode: ExecutionMode::Background,
            binary_name: Some("miyucloud-server.exe".into()),
            platforms: PlatformBinaries { windows_x64: Some("miyucloud-server.exe".into()), ..Default::default() },
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

/// Réponse JSON d'une opération d'écriture : `(statut HTTP, corps)`.
fn reponse_ecriture<T: serde::Serialize>(resultat: Result<T, RefusMarket>) -> (u16, String) {
    match resultat {
        Ok(data) => (200, serde_json::to_string_pretty(&MarketResponse::ok(data)).unwrap_or_default()),
        Err(e) => (
            e.statut,
            serde_json::to_string_pretty(&MarketResponse::<()>::err(e.message)).unwrap_or_default(),
        ),
    }
}

/// POST /api/market/publish — Publication (corps binaire signé, en-têtes déjà analysés).
pub async fn api_market_publish(
    store: &MarketStore,
    entetes: &EntetesSignees,
    corps: &[u8],
    maintenant: u64,
) -> (u16, String) {
    reponse_ecriture(store.publish(entetes, corps, maintenant).await)
}

/// DELETE /api/market/package/{id}/{version} — Retrait (requête signée, corps vide).
pub async fn api_market_unpublish(
    store: &MarketStore,
    entetes: &EntetesSignees,
    service_id: &str,
    version: &str,
    maintenant: u64,
) -> (u16, String) {
    reponse_ecriture(store.unpublish(entetes, service_id, version, maintenant).await)
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_market_paths(name: &str) -> (PathBuf, PathBuf, PathBuf) {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("miyukini-market-{name}-{unique}"));
        let db_path = root.join("market.db");
        let packages_dir = root.join("packages");
        (root, db_path, packages_dir)
    }

    #[test]
    fn open_marks_seed_service_as_not_downloadable_without_package() {
        let (root, db_path, packages_dir) = temp_market_paths("missing");
        let store = MarketStore::open(&db_path, &packages_dir, &[]).expect("market store");
        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");

        let entry = runtime
            .block_on(store.get_service("jaykoa"))
            .expect("seeded service");
        assert!(!entry.downloadable);

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn seed_contains_miyucloud_as_official_service() {
        let (root, db_path, packages_dir) = temp_market_paths("miyucloud-seed");
        let store = MarketStore::open(&db_path, &packages_dir, &[]).expect("market store");
        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");

        let entry = runtime
            .block_on(store.get_service("miyucloud"))
            .expect("seeded service");
        assert_eq!(entry.manifest.id, "miyucloud");
        assert_eq!(entry.manifest.service_type, ServiceType::InterCog);
        assert_eq!(entry.manifest.source, ServiceSource::Officiel);
        assert!(!entry.downloadable);

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn open_detects_package_in_packages_root_without_double_prefix() {
        let (root, db_path, packages_dir) = temp_market_paths("present");
        let package_path = packages_dir
            .join("jaykoa")
            .join("0.1.0")
            .join(package::package_filename("jaykoa", "0.1.0"));
        std::fs::create_dir_all(package_path.parent().expect("package parent")).expect("mkdir");
        std::fs::write(&package_path, b"dummy package").expect("write package");

        let store = MarketStore::open(&db_path, &packages_dir, &[]).expect("market store");
        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");

        let entry = runtime
            .block_on(store.get_service("jaykoa"))
            .expect("seeded service");
        assert!(entry.downloadable);
        assert_eq!(entry.manifest.package_size, Some(13));

        let bytes = runtime
            .block_on(store.get_package_bytes("jaykoa", "0.1.0"))
            .expect("package bytes");
        assert_eq!(bytes, b"dummy package");

        let _ = std::fs::remove_dir_all(root);
    }

    use miyauth::http::signer_requete;
    use miyumarket::protocol::encoder_corps_publication;
    use miyumarket::signature::signer_paquet;
    use miyusceau::CleSignature;

    fn maintenant() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
    }

    fn entetes_brutes(e: &EntetesSignees) -> Vec<String> {
        e.en_entetes().iter().map(|(k, v)| format!("{k}: {v}\r\n")).collect()
    }

    fn manifeste(id: &str, version: &str, source: ServiceSource) -> ServiceManifest {
        let mut m = official_catalog_seed().remove(0);
        m.id = id.to_string();
        m.version = version.to_string();
        m.source = source;
        m.binary_name = Some(format!("{id}.exe"));
        m
    }

    /// Corps de publication : manifeste + paquet scellé par `cle_paquet`.
    fn corps(m: &ServiceManifest, archive: &[u8], cle_paquet: &CleSignature) -> Vec<u8> {
        let paquet = signer_paquet(archive, &m.id, &m.version, m.binary_name.as_deref(), cle_paquet).unwrap();
        encoder_corps_publication(m, &paquet).unwrap()
    }

    /// Contrôle des en-têtes puis publication (comme le serveur).
    fn publier(store: &MarketStore, cle_requete: &CleSignature, corps: &[u8]) -> Result<PublishResponse, RefusMarket> {
        let t = maintenant();
        let e = signer_requete(cle_requete, USAGE_PUBLICATION, "POST", CHEMIN_PUBLICATION, corps, t);
        let e = store.controler_entetes(&entetes_brutes(&e), t)?;
        tokio::runtime::Runtime::new().unwrap().block_on(store.publish(&e, corps, t))
    }

    struct Banc {
        root: PathBuf,
        store: MarketStore,
        officiel: CleSignature,
    }

    fn banc(nom: &str) -> Banc {
        let (root, db_path, packages_dir) = temp_market_paths(nom);
        let officiel = CleSignature::generer();
        let store = MarketStore::open(&db_path, &packages_dir, &[officiel.cle_publique()]).expect("market store");
        Banc { root, store, officiel }
    }

    impl Drop for Banc {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn version_et_id_valides_et_invalides() {
        assert!(is_valid_version("1.2.3"));
        assert!(is_valid_version("0.1.0-beta.1"));
        for bad in ["../x", "1.2", "1.2.3/..", "01.2.3", "1.2.3+b", "", "1.2.3-", "a.b.c", "1.2.3-a/b"] {
            assert!(!is_valid_version(bad), "{bad}");
        }
        assert!(is_valid_service_id("miyu_cloud-2"));
        for bad in ["", "../etc", "A", "a/b", "a.b", &"a".repeat(65)] {
            assert!(!is_valid_service_id(bad), "{bad}");
        }
    }

    #[test]
    fn package_path_refuse_la_traversee() {
        let b = banc("traversal");
        assert!(b.store.package_path("jaykoa", "../x").is_none());
        assert!(b.store.package_path("../jaykoa", "1.0.0").is_none());
        assert!(b.store.package_path("jaykoa", "1.0.0").is_some());
        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
        assert!(runtime.block_on(b.store.get_package_bytes("jaykoa", "../x")).is_none());
    }

    #[test]
    fn publication_signee_officielle_acceptee_et_telechargeable() {
        let b = banc("pub-ok");
        let m = manifeste("svcok", "1.0.0", ServiceSource::Officiel);
        let c = corps(&m, b"PK\x03\x04 archive", &b.officiel);
        let r = publier(&b.store, &b.officiel, &c).expect("publication");
        assert_eq!(r.service_id, "svcok");
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let octets = runtime.block_on(b.store.get_package_bytes("svcok", "1.0.0")).expect("paquet stocké");
        assert!(miyumarket::signature::est_paquet_signe(&octets));
        assert_eq!(package::sha256_hex(&octets), r.checksum);
        // Republication identique : sans effet ; contenu différent : refus (pas de substitution).
        assert!(publier(&b.store, &b.officiel, &c).is_ok());
        let autre = corps(&m, b"PK\x03\x04 autre archive", &b.officiel);
        assert_eq!(publier(&b.store, &b.officiel, &autre).unwrap_err().statut, 409);
    }

    #[test]
    fn publication_non_signee_forgee_rejouee_ou_editeur_inconnu_refusee() {
        let b = banc("pub-ko");
        let m = manifeste("svcko", "1.0.0", ServiceSource::Officiel);
        let c = corps(&m, b"PK archive", &b.officiel);
        let t = maintenant();
        // Aucun en-tête signé (ancien schéma à jeton).
        assert_eq!(b.store.controler_entetes(&[], t).unwrap_err().statut, 401);
        // Éditeur non épinglé.
        let inconnu = CleSignature::generer();
        assert_eq!(publier(&b.store, &inconnu, &corps(&m, b"PK archive", &inconnu)).unwrap_err().statut, 401);
        // Requête officielle, paquet scellé par une autre clé.
        assert_eq!(publier(&b.store, &b.officiel, &corps(&m, b"PK archive", &inconnu)).unwrap_err().statut, 400);
        // Paquet tiers.
        let tiers = manifeste("svcko", "1.0.0", ServiceSource::Tiers);
        assert_eq!(publier(&b.store, &b.officiel, &corps(&tiers, b"PK archive", &b.officiel)).unwrap_err().statut, 400);
        // binary_name du catalogue différent de la valeur signée.
        let mut m2 = m.clone();
        let paquet = signer_paquet(b"PK archive", &m.id, &m.version, Some("autre.exe"), &b.officiel).unwrap();
        m2.binary_name = Some("svcko.exe".into());
        let c2 = encoder_corps_publication(&m2, &paquet).unwrap();
        assert_eq!(publier(&b.store, &b.officiel, &c2).unwrap_err().statut, 400);
        // Corps altéré après signature.
        let e = signer_requete(&b.officiel, USAGE_PUBLICATION, "POST", CHEMIN_PUBLICATION, &c, t);
        let mut altere = c.clone();
        let n = altere.len();
        altere[n - 1] ^= 1;
        let rt = tokio::runtime::Runtime::new().unwrap();
        assert_eq!(rt.block_on(b.store.publish(&e, &altere, t)).unwrap_err().statut, 401);
        // Requête valide une fois, puis rejeu refusé (en-têtes et vérification).
        assert!(rt.block_on(b.store.publish(&e, &c, t)).is_ok());
        assert_eq!(b.store.controler_entetes(&entetes_brutes(&e), t).unwrap_err().statut, 401);
        assert_eq!(rt.block_on(b.store.publish(&e, &c, t)).unwrap_err().statut, 401);
        // Requête périmée.
        let vieille = signer_requete(&b.officiel, USAGE_PUBLICATION, "POST", CHEMIN_PUBLICATION, &c, t - 3600);
        assert_eq!(b.store.controler_entetes(&entetes_brutes(&vieille), t).unwrap_err().statut, 401);
        // Signature d'un retrait présentée comme publication (séparation des usages).
        let retrait = signer_requete(&b.officiel, USAGE_RETRAIT, "POST", CHEMIN_PUBLICATION, &c, t);
        assert_eq!(rt.block_on(b.store.publish(&retrait, &c, t)).unwrap_err().statut, 401);
    }

    #[test]
    fn sans_editeur_officiel_publication_desactivee() {
        let (root, db_path, packages_dir) = temp_market_paths("disabled");
        let store = MarketStore::open(&db_path, &packages_dir, &[]).expect("market store");
        assert!(!store.publish_enabled());
        let cle = CleSignature::generer();
        let e = signer_requete(&cle, USAGE_PUBLICATION, "POST", CHEMIN_PUBLICATION, b"", maintenant());
        assert_eq!(store.controler_entetes(&entetes_brutes(&e), maintenant()).unwrap_err().statut, 503);
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn retrait_signe_accepte_et_refus() {
        let b = banc("unpub");
        let m = manifeste("svcdel", "1.0.0", ServiceSource::Officiel);
        publier(&b.store, &b.officiel, &corps(&m, b"PK archive", &b.officiel)).unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let t = maintenant();
        let chemin = "/api/market/package/svcdel/1.0.0";
        // Traversée refusée même signée.
        let e = signer_requete(&b.officiel, USAGE_RETRAIT, "DELETE", "/api/market/package/svcdel/../x", b"", t);
        assert_eq!(rt.block_on(b.store.unpublish(&e, "svcdel", "../x", t)).unwrap_err().statut, 400);
        // Inconnu, puis signature d'un autre chemin.
        let inconnu = signer_requete(&CleSignature::generer(), USAGE_RETRAIT, "DELETE", chemin, b"", t);
        assert_eq!(rt.block_on(b.store.unpublish(&inconnu, "svcdel", "1.0.0", t)).unwrap_err().statut, 401);
        let autre = signer_requete(&b.officiel, USAGE_RETRAIT, "DELETE", "/api/market/package/jaykoa/0.1.0", b"", t);
        assert_eq!(rt.block_on(b.store.unpublish(&autre, "svcdel", "1.0.0", t)).unwrap_err().statut, 401);
        // Valide.
        let e = signer_requete(&b.officiel, USAGE_RETRAIT, "DELETE", chemin, b"", t);
        assert!(rt.block_on(b.store.unpublish(&e, "svcdel", "1.0.0", t)).is_ok());
        assert!(rt.block_on(b.store.get_package_bytes("svcdel", "1.0.0")).is_none());
    }
}
