use std::env;
use std::path::PathBuf;
use std::time::Duration;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// TTL for file cache entries (ms)
    pub file_ttl_ms: u64,
    /// TTL for directory cache entries (ms)
    pub directory_ttl_ms: u64,
    /// Maximum number of cache entries
    pub max_entries: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            file_ttl_ms: 60_000,       // 1 minute
            directory_ttl_ms: 120_000, // 2 minutes
            max_entries: 10_000,       // 10,000 entries
        }
    }
}

/// Timeout configuration for different operations
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Timeout for file operations (ms)
    pub file_operation_ms: u64,
    /// Timeout for directory operations (ms)
    pub dir_operation_ms: u64,
    /// Timeout for lock acquisition (ms)
    pub lock_acquisition_ms: u64,
    /// Timeout for network operations (ms)
    pub network_operation_ms: u64,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            file_operation_ms: 10000,    // 10 seconds
            dir_operation_ms: 30000,     // 30 seconds
            lock_acquisition_ms: 5000,   // 5 seconds
            network_operation_ms: 15000, // 15 seconds
        }
    }
}

impl TimeoutConfig {
    /// Gets a Duration for file operations
    pub fn file_timeout(&self) -> Duration {
        Duration::from_millis(self.file_operation_ms)
    }

    /// Gets a Duration for file write operations
    pub fn file_write_timeout(&self) -> Duration {
        Duration::from_millis(self.file_operation_ms)
    }

    /// Gets a Duration for file read operations
    pub fn file_read_timeout(&self) -> Duration {
        Duration::from_millis(self.file_operation_ms)
    }

    /// Gets a Duration for file delete operations
    pub fn file_delete_timeout(&self) -> Duration {
        Duration::from_millis(self.file_operation_ms)
    }

    /// Gets a Duration for directory operations
    pub fn dir_timeout(&self) -> Duration {
        Duration::from_millis(self.dir_operation_ms)
    }

    /// Gets a Duration for lock acquisition
    pub fn lock_timeout(&self) -> Duration {
        Duration::from_millis(self.lock_acquisition_ms)
    }

    /// Gets a Duration for network operations
    pub fn network_timeout(&self) -> Duration {
        Duration::from_millis(self.network_operation_ms)
    }
}

/// Configuration for large resource handling
#[derive(Debug, Clone)]
pub struct ResourceConfig {
    /// Threshold in MB to consider a file as large
    pub large_file_threshold_mb: u64,
    /// Entry threshold to consider a directory as large
    pub large_dir_threshold_entries: usize,
    /// Chunk size for large file processing (bytes)
    pub chunk_size_bytes: usize,
    /// File size limit for loading into memory (MB)
    pub max_in_memory_file_size_mb: u64,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            large_file_threshold_mb: 100,      // 100 MB
            large_dir_threshold_entries: 1000, // 1000 entries
            chunk_size_bytes: 1024 * 1024,     // 1 MB
            max_in_memory_file_size_mb: 50,    // 50 MB
        }
    }
}

impl ResourceConfig {
    /// Converts a size in bytes to MB
    pub fn bytes_to_mb(&self, bytes: u64) -> u64 {
        bytes / (1024 * 1024)
    }

    /// Determines if a file is considered large
    pub fn is_large_file(&self, size_bytes: u64) -> bool {
        self.bytes_to_mb(size_bytes) >= self.large_file_threshold_mb
    }

    /// Determines if a file is large enough for parallel processing
    pub fn needs_parallel_processing(&self, size_bytes: u64, config: &ConcurrencyConfig) -> bool {
        self.bytes_to_mb(size_bytes) >= config.min_size_for_parallel_chunks_mb
    }

    /// Determines if a file can be fully loaded into memory
    pub fn can_load_in_memory(&self, size_bytes: u64) -> bool {
        self.bytes_to_mb(size_bytes) <= self.max_in_memory_file_size_mb
    }

    /// Determines if a directory is considered large
    pub fn is_large_directory(&self, entry_count: usize) -> bool {
        entry_count >= self.large_dir_threshold_entries
    }

    /// Calculates the number of chunks for parallel processing
    pub fn calculate_optimal_chunks(&self, size_bytes: u64, config: &ConcurrencyConfig) -> usize {
        // If the file is not large enough, return 1
        if !self.needs_parallel_processing(size_bytes, config) {
            return 1;
        }

        // Calculate the number of chunks based on size
        let chunk_count = (size_bytes as usize).div_ceil(config.parallel_chunk_size_bytes);

        // Limit to the maximum number of parallel chunks
        chunk_count.min(config.max_parallel_chunks)
    }

    /// Calculates the optimal size of each chunk for parallel processing
    pub fn calculate_chunk_size(&self, file_size: u64, chunk_count: usize) -> usize {
        if chunk_count <= 1 {
            return file_size as usize;
        }

        // Distribute the size evenly among the chunks
        (file_size as usize).div_ceil(chunk_count)
    }
}

/// Configuration for concurrent operations
#[derive(Debug, Clone)]
pub struct ConcurrencyConfig {
    /// Maximum concurrent file tasks
    pub max_concurrent_files: usize,
    /// Maximum concurrent directory tasks
    pub max_concurrent_dirs: usize,
    /// Maximum concurrent IO operations
    pub max_concurrent_io: usize,
    /// Maximum chunks to process in parallel per file
    pub max_parallel_chunks: usize,
    /// Minimum file size (MB) to apply parallel chunk processing
    pub min_size_for_parallel_chunks_mb: u64,
    /// Chunk size for parallel processing (bytes)
    pub parallel_chunk_size_bytes: usize,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_files: 10,
            max_concurrent_dirs: 5,
            max_concurrent_io: 20,
            max_parallel_chunks: 8,
            min_size_for_parallel_chunks_mb: 200,       // 200 MB
            parallel_chunk_size_bytes: 8 * 1024 * 1024, // 8 MB
        }
    }
}

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Root directory for storage
    pub root_dir: String,
    /// Chunk size for file processing
    pub chunk_size: usize,
    /// Threshold for parallel processing
    pub parallel_threshold: usize,
    /// Retention days for files in the trash
    pub trash_retention_days: u32,
    /// Maximum upload file size in bytes (default: 10 GB).
    /// Applied as a hard limit to WebDAV PUT and streaming uploads.
    pub max_upload_size: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        // Architecture-appropriate max upload size to avoid overflow on 32-bit systems
        const MAX_UPLOAD_SIZE: usize = if cfg!(target_pointer_width = "64") {
            10 * 1024 * 1024 * 1024 // 10 GB on 64-bit
        } else {
            1024 * 1024 * 1024 // 1 GB on 32-bit
        };
        Self {
            root_dir: "storage".to_string(),
            chunk_size: 1024 * 1024,                  // 1 MB
            parallel_threshold: 100 * 1024 * 1024,    // 100 MB
            trash_retention_days: 30,                 // 30 days
            max_upload_size: MAX_UPLOAD_SIZE,
        }
    }
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    /// Maximum connections for the maintenance pool (background/batch tasks).
    /// Defaults to 25% of `max_connections` (minimum 2).
    pub maintenance_max_connections: u32,
    /// Minimum connections for the maintenance pool.
    /// Defaults to 1.
    pub maintenance_min_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            // Updated connection string with default credentials that PostgreSQL often uses
            connection_string: "postgres://postgres:postgres@localhost:5432/miyucloud".to_string(),
            max_connections: 20,
            min_connections: 5,
            connect_timeout_secs: 10,
            idle_timeout_secs: 300,
            max_lifetime_secs: 1800,
            maintenance_max_connections: 5,
            maintenance_min_connections: 1,
        }
    }
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub access_token_expiry_secs: i64,
    pub refresh_token_expiry_secs: i64,
    /// Argon2id memory cost in KiB (default 65536 = 64 MiB)
    pub hash_memory_cost: u32,
    /// Argon2id time cost / iterations (default 3)
    pub hash_time_cost: u32,
    /// Argon2id parallelism lanes (default 2)
    pub hash_parallelism: u32,
    /// Rate limiting / account lockout configuration
    pub rate_limit: RateLimitConfig,
}

/// Rate limiting and brute-force protection configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Max login attempts per IP per window (default: 10)
    pub login_max_requests: u32,
    /// Login rate-limit window in seconds (default: 60)
    pub login_window_secs: u64,
    /// Max registration attempts per IP per window (default: 5)
    pub register_max_requests: u32,
    /// Registration rate-limit window in seconds (default: 3600)
    pub register_window_secs: u64,
    /// Max token refresh attempts per IP per window (default: 20)
    pub refresh_max_requests: u32,
    /// Refresh rate-limit window in seconds (default: 60)
    pub refresh_window_secs: u64,
    /// Consecutive failed logins before account lockout (default: 5)
    pub lockout_max_failures: u32,
    /// Account lockout duration in seconds (default: 900 = 15 min)
    pub lockout_duration_secs: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            login_max_requests: 10,
            login_window_secs: 60,
            register_max_requests: 5,
            register_window_secs: 3600,
            refresh_max_requests: 20,
            refresh_window_secs: 60,
            lockout_max_failures: 5,
            lockout_duration_secs: 900,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            // SECURITY: This default is intentionally insecure to force operators
            // to set MIYUCLOUD_JWT_SECRET in production. The from_env() method
            // will validate this and warn/panic if not configured.
            jwt_secret: String::new(),
            access_token_expiry_secs: 3600,     // 1 hour
            refresh_token_expiry_secs: 2592000, // 30 days
            hash_memory_cost: 65536,            // 64 MiB
            hash_time_cost: 3,
            hash_parallelism: 2,
            rate_limit: RateLimitConfig::default(),
        }
    }
}

/// OpenID Connect (OIDC) configuration
#[derive(Debug, Clone)]
pub struct OidcConfig {
    /// Whether OIDC authentication is enabled
    pub enabled: bool,
    /// OIDC Issuer URL (e.g. https://authentik.example.com/application/o/miyucloud/)
    pub issuer_url: String,
    /// OIDC Client ID
    pub client_id: String,
    /// OIDC Client Secret
    pub client_secret: String,
    /// Redirect URI after OIDC authentication (must match IdP config)
    pub redirect_uri: String,
    /// OIDC scopes to request
    pub scopes: String,
    /// Frontend URL to redirect after successful OIDC login (tokens appended as fragment)
    pub frontend_url: String,
    /// Whether to auto-create users on first OIDC login (JIT provisioning)
    pub auto_provision: bool,
    /// Comma-separated list of OIDC groups that map to admin role
    pub admin_groups: String,
    /// Whether to disable password-based login entirely
    pub disable_password_login: bool,
    /// OIDC provider display name (shown in UI)
    pub provider_name: String,
}

impl Default for OidcConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            issuer_url: String::new(),
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: "http://localhost:8086/api/auth/oidc/callback".to_string(),
            scopes: "openid profile email".to_string(),
            frontend_url: "http://localhost:8086".to_string(),
            auto_provision: true,
            admin_groups: String::new(),
            disable_password_login: false,
            provider_name: "SSO".to_string(),
        }
    }
}

impl OidcConfig {
    /// Load OIDC configuration from environment variables only
    pub fn from_env() -> Self {
        use std::env;
        let mut cfg = Self::default();
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_ENABLED") {
            cfg.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_ISSUER_URL") {
            cfg.issuer_url = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_CLIENT_ID") {
            cfg.client_id = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_CLIENT_SECRET") {
            cfg.client_secret = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_REDIRECT_URI") {
            cfg.redirect_uri = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_SCOPES") {
            cfg.scopes = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_FRONTEND_URL") {
            cfg.frontend_url = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_AUTO_PROVISION") {
            cfg.auto_provision = v.parse::<bool>().unwrap_or(true);
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_ADMIN_GROUPS") {
            cfg.admin_groups = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_DISABLE_PASSWORD_LOGIN") {
            cfg.disable_password_login = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_PROVIDER_NAME") {
            cfg.provider_name = v;
        }
        cfg
    }
}

/// WOPI (Web Application Open Platform Interface) configuration
#[derive(Debug, Clone)]
pub struct WopiConfig {
    /// Whether WOPI integration is enabled
    pub enabled: bool,
    /// URL to the WOPI client's discovery endpoint
    /// e.g., "http://collabora:9980/hosting/discovery"
    pub discovery_url: String,
    /// Secret key for signing WOPI access tokens
    /// Falls back to JWT secret if empty
    pub secret: String,
    /// Access token TTL in seconds (default: 86400 = 24 hours)
    pub token_ttl_secs: i64,
    /// Lock expiration in seconds (default: 1800 = 30 minutes)
    pub lock_ttl_secs: u64,
}

impl Default for WopiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            discovery_url: String::new(),
            secret: String::new(),
            token_ttl_secs: 86400,
            lock_ttl_secs: 1800,
        }
    }
}

/// Nextcloud compatibility configuration
#[derive(Debug, Clone)]
pub struct NextcloudConfig {
    /// Whether the Nextcloud compatibility layer is enabled
    pub enabled: bool,
    /// Instance ID suffix for oc:id formatting (e.g., "ocnca")
    pub instance_id: String,
    /// Emulated Nextcloud version (major.minor.patch).
    /// Clients use this to decide which features to enable.
    pub emulated_version: (u32, u32, u32),
    /// Login Flow v2 token TTL in seconds (default: 600 = 10 minutes)
    pub login_flow_ttl_secs: u64,
}

impl Default for NextcloudConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            instance_id: "ocnca".to_string(),
            emulated_version: (28, 0, 4),
            login_flow_ttl_secs: 600,
        }
    }
}

impl NextcloudConfig {
    /// Version string, e.g. "28.0.4".
    pub fn version_string(&self) -> String {
        let (maj, min, pat) = self.emulated_version;
        format!("{}.{}.{}", maj, min, pat)
    }
}

/// Feature configuration (feature flags)
#[derive(Debug, Clone)]
pub struct FeaturesConfig {
    pub enable_auth: bool,
    pub enable_user_storage_quotas: bool,
    pub enable_file_sharing: bool,
    pub enable_trash: bool,
    pub enable_search: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            enable_auth: true, // Enable authentication by default
            enable_user_storage_quotas: false,
            enable_file_sharing: true, // Enable file sharing by default
            enable_trash: true,        // Enable trash feature
            enable_search: true,       // Enable search feature
        }
    }
}

/// Security configuration for upload validation, cookie hardening, and bandwidth limits.
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Comma-separated list of blocked MIME types (e.g. "application/x-executable,application/x-dosexec").
    /// Empty = no blocklist enforcement (all types accepted).
    pub blocked_mime_types: Vec<String>,
    /// Force `Secure` flag on all cookies, regardless of detected scheme.
    pub force_secure_cookies: bool,
    /// Per-user bandwidth limit in MB per minute. 0 = unlimited.
    pub bandwidth_limit_mb_per_min: u64,
    /// Enable HSTS header (Strict-Transport-Security).
    pub enable_hsts: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            blocked_mime_types: vec![
                "application/x-executable".into(),
                "application/x-dosexec".into(),
                "application/x-msdos-program".into(),
                "application/x-msdownload".into(),
                "application/x-mach-binary".into(),
                "application/vnd.microsoft.portable-executable".into(),
                "application/x-elf".into(),
                "application/x-sharedlib".into(),
            ],
            force_secure_cookies: false,
            bandwidth_limit_mb_per_min: 0,
            enable_hsts: false,
        }
    }
}

/// Encryption at rest configuration (ISO 27001 A.8.24, RGPD Art.32, HDS mandatory).
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// Enable file blob encryption at rest.
    pub enabled: bool,
    /// Master encryption key (base64-encoded, 32 bytes for AES-256-GCM).
    /// MUST be set when encryption is enabled.
    pub key_base64: String,
    /// Algorithm identifier (for future rotation support).
    pub algorithm: String,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            key_base64: String::new(),
            algorithm: "aes-256-gcm".into(),
        }
    }
}

/// TLS configuration for native HTTPS.
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Enable native HTTPS.
    pub enabled: bool,
    /// Path to PEM certificate file.
    pub cert_path: String,
    /// Path to PEM private key file.
    pub key_path: String,
    /// HTTPS port (default: 443 or 8443 for non-root).
    pub port: u16,
    /// Auto-generate a self-signed certificate if cert/key files don't exist.
    pub auto_generate: bool,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: "./storage/tls/cert.pem".into(),
            key_path: "./storage/tls/key.pem".into(),
            port: 8443,
            auto_generate: true,
        }
    }
}

/// DDNS provider for dynamic DNS updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DdnsProvider {
    NoIp,
    DuckDns,
    Generic,
}

/// DDNS (Dynamic DNS) configuration for direct internet access.
#[derive(Debug, Clone)]
pub struct DdnsConfig {
    pub enabled: bool,
    pub provider: DdnsProvider,
    pub hostname: String,
    pub token: String,
    pub username: Option<String>,
    pub update_url: Option<String>,
    pub update_interval_secs: u64,
    pub detect_ip_url: String,
}

impl Default for DdnsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: DdnsProvider::NoIp,
            hostname: String::new(),
            token: String::new(),
            username: None,
            update_url: None,
            update_interval_secs: 300,
            detect_ip_url: "https://api.ipify.org".into(),
        }
    }
}

/// MWS Tunnel configuration for Miyukini Webway System connectivity.
#[derive(Debug, Clone)]
pub struct MwsTunnelConfig {
    pub enabled: bool,
    pub relay_address: String,
    pub tracker_address: String,
    pub cog_id: String,
    pub subdomain_slug: Option<String>,
    pub auto_reconnect: bool,
    pub reconnect_delay_secs: u64,
    pub max_reconnect_attempts: u32,
}

impl Default for MwsTunnelConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            relay_address: "miyukini.com:7000".into(),
            tracker_address: "miyukini.com:21000".into(),
            cog_id: format!("miyucloud-{}", gethostname()),
            subdomain_slug: None,
            auto_reconnect: true,
            reconnect_delay_secs: 10,
            max_reconnect_attempts: 5,
        }
    }
}

/// Central (Miyukini COG) client configuration.
#[derive(Debug, Clone)]
pub struct CentralClientConfig {
    pub enabled: bool,
    pub central_url: String,
    pub service_id: String,
    pub report_interval_secs: u64,
}

impl Default for CentralClientConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            central_url: "http://127.0.0.1:8090".into(),
            service_id: "miyucloud".into(),
            report_interval_secs: 30,
        }
    }
}

/// Web connection management — MWS tunnel, DDNS, and Central client.
#[derive(Debug, Clone)]
pub struct WebConnectionConfig {
    pub mws: MwsTunnelConfig,
    pub ddns: DdnsConfig,
    pub central: CentralClientConfig,
}

impl Default for WebConnectionConfig {
    fn default() -> Self {
        Self {
            mws: MwsTunnelConfig::default(),
            ddns: DdnsConfig::default(),
            central: CentralClientConfig::default(),
        }
    }
}

/// Best-effort hostname detection for default COG ID.
fn gethostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".into())
}

/// Global application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Storage directory path
    pub storage_path: PathBuf,
    /// Static files directory path
    pub static_path: PathBuf,
    /// Server port
    pub server_port: u16,
    /// Server host
    pub server_host: String,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Timeout configuration
    pub timeouts: TimeoutConfig,
    /// Resource configuration
    pub resources: ResourceConfig,
    /// Concurrency configuration
    pub concurrency: ConcurrencyConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Feature configuration
    pub features: FeaturesConfig,
    /// OIDC configuration
    pub oidc: OidcConfig,
    /// WOPI configuration
    pub wopi: WopiConfig,
    /// Nextcloud compatibility configuration
    pub nextcloud: NextcloudConfig,
    /// Security configuration (MIME blocklist, cookie hardening, bandwidth, HSTS)
    pub security: SecurityConfig,
    /// Encryption at rest configuration
    pub encryption: EncryptionConfig,
    /// Web connection management (MWS tunnel, DDNS, Central client)
    pub web_connection: WebConnectionConfig,
    /// TLS/HTTPS configuration
    pub tls: TlsConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from("./storage"),
            static_path: PathBuf::from("./static"),
            server_port: 8086,
            server_host: "127.0.0.1".to_string(),
            cache: CacheConfig::default(),
            timeouts: TimeoutConfig::default(),
            resources: ResourceConfig::default(),
            concurrency: ConcurrencyConfig::default(),
            storage: StorageConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            features: FeaturesConfig::default(),
            oidc: OidcConfig::default(),
            wopi: WopiConfig::default(),
            nextcloud: NextcloudConfig::default(),
            security: SecurityConfig::default(),
            encryption: EncryptionConfig::default(),
            web_connection: WebConnectionConfig::default(),
            tls: TlsConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Use environment variables to override default values
        if let Ok(storage_path) = env::var("MIYUCLOUD_STORAGE_PATH") {
            config.storage_path = PathBuf::from(storage_path);
        }

        if let Ok(static_path) = env::var("MIYUCLOUD_STATIC_PATH") {
            config.static_path = PathBuf::from(static_path);
        }

        if let Ok(server_port) = env::var("MIYUCLOUD_SERVER_PORT")
            && let Ok(port) = server_port.parse::<u16>()
        {
            config.server_port = port;
        }

        if let Ok(server_host) = env::var("MIYUCLOUD_SERVER_HOST") {
            config.server_host = server_host;
        }

        // Database configuration
        if let Ok(connection_string) = env::var("MIYUCLOUD_DB_CONNECTION_STRING") {
            config.database.connection_string = connection_string;
        }

        if let Ok(max_connections) =
            env::var("MIYUCLOUD_DB_MAX_CONNECTIONS").map(|v| v.parse::<u32>())
            && let Ok(val) = max_connections
        {
            config.database.max_connections = val;
        }

        if let Ok(min_connections) =
            env::var("MIYUCLOUD_DB_MIN_CONNECTIONS").map(|v| v.parse::<u32>())
            && let Ok(val) = min_connections
        {
            config.database.min_connections = val;
        }

        if let Ok(max_conn) =
            env::var("MIYUCLOUD_DB_MAINTENANCE_MAX_CONNECTIONS").map(|v| v.parse::<u32>())
            && let Ok(val) = max_conn
        {
            config.database.maintenance_max_connections = val;
        }

        if let Ok(min_conn) =
            env::var("MIYUCLOUD_DB_MAINTENANCE_MIN_CONNECTIONS").map(|v| v.parse::<u32>())
            && let Ok(val) = min_conn
        {
            config.database.maintenance_min_connections = val;
        }

        // Auth configuration
        if let Some(jwt_secret) = env::var("MIYUCLOUD_JWT_SECRET")
            .ok()
            .filter(|s| !s.is_empty())
        {
            // SECURITY: Validate JWT secret minimum entropy (RFC 7518 §3.2
            // recommends ≥256 bits for HS256). Panic on dangerously short
            // secrets, warn on sub-optimal ones.
            let len = jwt_secret.len();
            if config.features.enable_auth && len < 16 {
                panic!(
                    "FATAL: MIYUCLOUD_JWT_SECRET is dangerously short ({} bytes). \
                     Minimum: 32 bytes (256 bits) for HS256. \
                     Generate a secure secret with: openssl rand -hex 32",
                    len
                );
            } else if config.features.enable_auth && len < 32 {
                tracing::warn!("==========================================================");
                tracing::warn!(
                    "MIYUCLOUD_JWT_SECRET is only {} bytes — recommended minimum is 32 (256 bits).",
                    len
                );
                tracing::warn!("Generate a stronger secret with: openssl rand -hex 32");
                tracing::warn!("==========================================================");
            }
            config.auth.jwt_secret = jwt_secret;
        }

        // SECURITY: Auto-persist JWT secret to storage so it survives restarts.
        // Priority: env var > persisted file > generate new.
        if config.features.enable_auth && config.auth.jwt_secret.is_empty() {
            let secret_file = config.storage_path.join(".jwt_secret");

            if secret_file.exists() {
                // Read persisted secret from previous run
                match std::fs::read_to_string(&secret_file) {
                    Ok(persisted) => {
                        let persisted = persisted.trim().to_string();
                        if persisted.len() >= 32 {
                            config.auth.jwt_secret = persisted;
                            tracing::info!("JWT secret loaded from {}", secret_file.display());
                        } else {
                            tracing::warn!(
                                "Persisted JWT secret too short ({}B), regenerating",
                                persisted.len()
                            );
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to read {}: {}", secret_file.display(), e);
                    }
                }
            }

            // Still empty → generate and persist
            if config.auth.jwt_secret.is_empty() {
                use rand_core::{OsRng, RngCore};
                let mut key = [0u8; 32];
                OsRng.fill_bytes(&mut key);
                let generated_secret: String = key.iter().map(|b| format!("{:02x}", b)).collect();

                // Persist to storage volume so it survives container restarts
                if let Err(e) = std::fs::write(&secret_file, &generated_secret) {
                    tracing::error!(
                        "Failed to persist JWT secret to {}: {}. \
                         Tokens will be invalidated on restart!",
                        secret_file.display(),
                        e
                    );
                } else {
                    // Restrict file permissions (owner-only read/write)
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let _ = std::fs::set_permissions(
                            &secret_file,
                            std::fs::Permissions::from_mode(0o600),
                        );
                    }
                    tracing::info!(
                        "JWT secret auto-generated and persisted to {}",
                        secret_file.display()
                    );
                }

                config.auth.jwt_secret = generated_secret;
            }
        }

        if let Ok(access_token_expiry) =
            env::var("MIYUCLOUD_ACCESS_TOKEN_EXPIRY_SECS").map(|v| v.parse::<i64>())
            && let Ok(val) = access_token_expiry
        {
            config.auth.access_token_expiry_secs = val;
        }

        if let Ok(refresh_token_expiry) =
            env::var("MIYUCLOUD_REFRESH_TOKEN_EXPIRY_SECS").map(|v| v.parse::<i64>())
            && let Ok(val) = refresh_token_expiry
        {
            config.auth.refresh_token_expiry_secs = val;
        }

        // Argon2 hashing parameters
        if let Ok(v) = env::var("MIYUCLOUD_HASH_MEMORY_COST").map(|v| v.parse::<u32>())
            && let Ok(val) = v
        {
            config.auth.hash_memory_cost = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_HASH_TIME_COST").map(|v| v.parse::<u32>())
            && let Ok(val) = v
        {
            config.auth.hash_time_cost = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_HASH_PARALLELISM").map(|v| v.parse::<u32>())
            && let Ok(val) = v
        {
            config.auth.hash_parallelism = val;
        }

        // Rate limiting / account lockout
        if let Ok(v) = env::var("MIYUCLOUD_RATE_LIMIT_LOGIN_MAX").map(|v| v.parse::<u32>())
            && let Ok(val) = v
        {
            config.auth.rate_limit.login_max_requests = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_RATE_LIMIT_LOGIN_WINDOW_SECS").map(|v| v.parse::<u64>())
            && let Ok(val) = v
        {
            config.auth.rate_limit.login_window_secs = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_RATE_LIMIT_REGISTER_MAX").map(|v| v.parse::<u32>())
            && let Ok(val) = v
        {
            config.auth.rate_limit.register_max_requests = val;
        }
        if let Ok(v) =
            env::var("MIYUCLOUD_RATE_LIMIT_REGISTER_WINDOW_SECS").map(|v| v.parse::<u64>())
            && let Ok(val) = v
        {
            config.auth.rate_limit.register_window_secs = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_RATE_LIMIT_REFRESH_MAX").map(|v| v.parse::<u32>())
            && let Ok(val) = v
        {
            config.auth.rate_limit.refresh_max_requests = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_RATE_LIMIT_REFRESH_WINDOW_SECS").map(|v| v.parse::<u64>())
            && let Ok(val) = v
        {
            config.auth.rate_limit.refresh_window_secs = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_LOCKOUT_MAX_FAILURES").map(|v| v.parse::<u32>())
            && let Ok(val) = v
        {
            config.auth.rate_limit.lockout_max_failures = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_LOCKOUT_DURATION_SECS").map(|v| v.parse::<u64>())
            && let Ok(val) = v
        {
            config.auth.rate_limit.lockout_duration_secs = val;
        }

        // Feature flags
        if let Ok(enable_auth) = env::var("MIYUCLOUD_ENABLE_AUTH").map(|v| v.parse::<bool>())
            && let Ok(val) = enable_auth
        {
            config.features.enable_auth = val;
        }

        if let Ok(enable_user_storage_quotas) =
            env::var("MIYUCLOUD_ENABLE_USER_STORAGE_QUOTAS").map(|v| v.parse::<bool>())
            && let Ok(val) = enable_user_storage_quotas
        {
            config.features.enable_user_storage_quotas = val;
        }

        if let Ok(enable_file_sharing) =
            env::var("MIYUCLOUD_ENABLE_FILE_SHARING").map(|v| v.parse::<bool>())
            && let Ok(val) = enable_file_sharing
        {
            config.features.enable_file_sharing = val;
        }

        if let Ok(enable_trash) = env::var("MIYUCLOUD_ENABLE_TRASH").map(|v| v.parse::<bool>())
            && let Ok(val) = enable_trash
        {
            config.features.enable_trash = val;
        }

        if let Ok(enable_search) = env::var("MIYUCLOUD_ENABLE_SEARCH").map(|v| v.parse::<bool>())
            && let Ok(val) = enable_search
        {
            config.features.enable_search = val;
        }

        // Storage limits
        if let Ok(max_upload) = env::var("MIYUCLOUD_MAX_UPLOAD_SIZE").map(|v| v.parse::<usize>())
            && let Ok(val) = max_upload
        {
            config.storage.max_upload_size = val;
        }

        // OIDC configuration
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_ENABLED") {
            config.oidc.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_ISSUER_URL") {
            config.oidc.issuer_url = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_CLIENT_ID") {
            config.oidc.client_id = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_CLIENT_SECRET") {
            config.oidc.client_secret = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_REDIRECT_URI") {
            config.oidc.redirect_uri = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_SCOPES") {
            config.oidc.scopes = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_FRONTEND_URL") {
            config.oidc.frontend_url = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_AUTO_PROVISION") {
            config.oidc.auto_provision = v.parse::<bool>().unwrap_or(true);
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_ADMIN_GROUPS") {
            config.oidc.admin_groups = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_DISABLE_PASSWORD_LOGIN") {
            config.oidc.disable_password_login = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_OIDC_PROVIDER_NAME") {
            config.oidc.provider_name = v;
        }

        // Security configuration
        if let Ok(v) = env::var("MIYUCLOUD_BLOCKED_MIME_TYPES") {
            config.security.blocked_mime_types = v
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
        if let Ok(v) = env::var("MIYUCLOUD_FORCE_SECURE_COOKIES") {
            config.security.force_secure_cookies = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_BANDWIDTH_LIMIT_MB_PER_MIN")
            && let Ok(val) = v.parse::<u64>()
        {
            config.security.bandwidth_limit_mb_per_min = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_ENABLE_HSTS") {
            config.security.enable_hsts = v.parse::<bool>().unwrap_or(false);
        }

        // Encryption at rest configuration
        if let Ok(v) = env::var("MIYUCLOUD_ENCRYPTION_ENABLED") {
            config.encryption.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_ENCRYPTION_KEY") {
            config.encryption.key_base64 = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_ENCRYPTION_ALGORITHM") {
            config.encryption.algorithm = v;
        }
        // Validate encryption config
        if config.encryption.enabled && config.encryption.key_base64.is_empty() {
            panic!(
                "FATAL: MIYUCLOUD_ENCRYPTION_ENABLED=true but MIYUCLOUD_ENCRYPTION_KEY is not set. \
                 Generate a key with: openssl rand -base64 32"
            );
        }

        // TLS configuration
        if let Ok(v) = env::var("MIYUCLOUD_TLS_ENABLED") {
            config.tls.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_TLS_CERT") {
            config.tls.cert_path = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_TLS_KEY") {
            config.tls.key_path = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_TLS_PORT")
            && let Ok(val) = v.parse::<u16>()
        {
            config.tls.port = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_TLS_AUTO_GENERATE") {
            config.tls.auto_generate = v.parse::<bool>().unwrap_or(true);
        }

        // Web connection configuration (MWS tunnel, DDNS, Central client)
        if let Ok(v) = env::var("MIYUCLOUD_MWS_ENABLED") {
            config.web_connection.mws.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_MWS_RELAY_ADDRESS") {
            config.web_connection.mws.relay_address = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_MWS_TRACKER_ADDRESS") {
            config.web_connection.mws.tracker_address = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_MWS_COG_ID") {
            config.web_connection.mws.cog_id = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_MWS_SUBDOMAIN") {
            config.web_connection.mws.subdomain_slug = Some(v);
        }

        if let Ok(v) = env::var("MIYUCLOUD_DDNS_ENABLED") {
            config.web_connection.ddns.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_DDNS_PROVIDER") {
            config.web_connection.ddns.provider = match v.to_lowercase().as_str() {
                "duckdns" => DdnsProvider::DuckDns,
                "generic" => DdnsProvider::Generic,
                _ => DdnsProvider::NoIp,
            };
        }
        if let Ok(v) = env::var("MIYUCLOUD_DDNS_HOSTNAME") {
            config.web_connection.ddns.hostname = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_DDNS_TOKEN") {
            config.web_connection.ddns.token = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_DDNS_USERNAME") {
            config.web_connection.ddns.username = Some(v);
        }
        if let Ok(v) = env::var("MIYUCLOUD_DDNS_UPDATE_URL") {
            config.web_connection.ddns.update_url = Some(v);
        }
        if let Ok(v) = env::var("MIYUCLOUD_DDNS_INTERVAL")
            && let Ok(val) = v.parse::<u64>()
        {
            config.web_connection.ddns.update_interval_secs = val;
        }

        if let Ok(v) = env::var("MIYUCLOUD_CENTRAL_ENABLED") {
            config.web_connection.central.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_CENTRAL_URL") {
            config.web_connection.central.central_url = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_CENTRAL_SERVICE_ID") {
            config.web_connection.central.service_id = v;
        }

        // Validate OIDC config when enabled
        if config.oidc.enabled
            && (config.oidc.issuer_url.is_empty()
                || config.oidc.client_id.is_empty()
                || config.oidc.client_secret.is_empty())
        {
            tracing::error!(
                "OIDC is enabled but MIYUCLOUD_OIDC_ISSUER_URL, MIYUCLOUD_OIDC_CLIENT_ID, or MIYUCLOUD_OIDC_CLIENT_SECRET are not set"
            );
            config.oidc.enabled = false;
        }

        // WOPI configuration
        if let Ok(v) = env::var("MIYUCLOUD_WOPI_ENABLED") {
            config.wopi.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_WOPI_DISCOVERY_URL") {
            config.wopi.discovery_url = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_WOPI_SECRET") {
            config.wopi.secret = v;
        }
        if let Ok(v) = env::var("MIYUCLOUD_WOPI_TOKEN_TTL_SECS")
            && let Ok(val) = v.parse::<i64>()
        {
            config.wopi.token_ttl_secs = val;
        }
        if let Ok(v) = env::var("MIYUCLOUD_WOPI_LOCK_TTL_SECS")
            && let Ok(val) = v.parse::<u64>()
        {
            config.wopi.lock_ttl_secs = val;
        }

        // WOPI secret fallback: use JWT secret if WOPI secret not set
        if config.wopi.enabled && config.wopi.secret.is_empty() {
            config.wopi.secret = config.auth.jwt_secret.clone();
            tracing::info!("WOPI secret not set, falling back to JWT secret");
        }

        // Nextcloud compatibility configuration
        if let Ok(v) = env::var("MIYUCLOUD_NEXTCLOUD_ENABLED") {
            config.nextcloud.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("MIYUCLOUD_NEXTCLOUD_INSTANCE_ID") {
            let trimmed = v.trim();
            if !trimmed.is_empty() {
                config.nextcloud.instance_id = trimmed.to_string();
            }
        }
        if let Ok(v) = env::var("MIYUCLOUD_NEXTCLOUD_VERSION") {
            // Expected format: "28.0.4"
            let parts: Vec<&str> = v.trim().splitn(3, '.').collect();
            if parts.len() == 3
                && let (Ok(maj), Ok(min), Ok(pat)) = (
                    parts[0].parse::<u32>(),
                    parts[1].parse::<u32>(),
                    parts[2].parse::<u32>(),
                )
            {
                config.nextcloud.emulated_version = (maj, min, pat);
            }
        }

        config
    }

    pub fn with_features(mut self, features: FeaturesConfig) -> Self {
        self.features = features;
        self
    }

    pub fn db_enabled(&self) -> bool {
        self.features.enable_auth
    }

    pub fn auth_enabled(&self) -> bool {
        self.features.enable_auth
    }

    /// Build the public base URL for generating share links and other external URLs.
    ///
    /// Priority:
    /// 1. `MIYUCLOUD_BASE_URL` env var (used as-is)
    /// 2. If `server_host` already contains a scheme (`http://` or `https://`),
    ///    treat it as a full origin and do **not** prepend a scheme or append a port.
    /// 3. Otherwise, fall back to `http://{server_host}:{server_port}`.
    pub fn base_url(&self) -> String {
        if let Ok(explicit) = std::env::var("MIYUCLOUD_BASE_URL") {
            return explicit.trim_end_matches('/').to_string();
        }

        let host = self.server_host.trim_end_matches('/');

        if host.starts_with("http://") || host.starts_with("https://") {
            // The user already provided a full origin — use it directly.
            host.to_string()
        } else {
            format!("http://{}:{}", host, self.server_port)
        }
    }
}

/// Gets a default global configuration
pub fn default_config() -> AppConfig {
    AppConfig::default()
}
