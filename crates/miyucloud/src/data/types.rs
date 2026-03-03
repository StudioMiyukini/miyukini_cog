//! Types domaine MiyuCloud.
//!
//! @id: miyucloud_domain_types
//! @do: define_miyucloud_domain_model
//! @role: data
//! @layer: domain
//!
//! Tous les types domaine pour les fichiers, dossiers, partages, pairs de sync,
//! horloges vectorielles, conflits, journal d'acces, quotas et statistiques.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Fichiers et dossiers
// ---------------------------------------------------------------------------

/// Entree fichier dans le cloud.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileEntry {
    /// UUID v4.
    pub id: String,
    /// UUID du dossier parent (None = racine).
    #[serde(default)]
    pub parent_id: Option<String>,
    /// UUID du profil proprietaire.
    pub owner_id: String,
    /// Nom du fichier.
    pub name: String,
    /// Type MIME.
    #[serde(default = "default_mime_type")]
    pub mime_type: String,
    /// Taille originale en octets.
    #[serde(default)]
    pub size_bytes: u64,
    /// SHA-256 du contenu original (hex lowercase).
    #[serde(default)]
    pub checksum_sha256: String,
    /// IV/Nonce du chiffrement at-rest (base64).
    #[serde(default)]
    pub encryption_iv: Option<String>,
    /// Nombre de chunks.
    #[serde(default = "default_chunk_count")]
    pub chunk_count: u32,
    /// Fichier dans la corbeille.
    #[serde(default)]
    pub is_trashed: bool,
    /// Date de mise en corbeille (ISO 8601).
    #[serde(default)]
    pub trashed_at: Option<String>,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Date de derniere modification (ISO 8601).
    pub updated_at: String,
}

fn default_mime_type() -> String {
    "application/octet-stream".to_string()
}

const fn default_chunk_count() -> u32 {
    1
}

/// Entree dossier dans le cloud.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FolderEntry {
    /// UUID v4.
    pub id: String,
    /// UUID du dossier parent (None = racine).
    #[serde(default)]
    pub parent_id: Option<String>,
    /// UUID du profil proprietaire.
    pub owner_id: String,
    /// Nom du dossier.
    pub name: String,
    /// Dossier dans la corbeille.
    #[serde(default)]
    pub is_trashed: bool,
    /// Date de mise en corbeille (ISO 8601).
    #[serde(default)]
    pub trashed_at: Option<String>,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Date de derniere modification (ISO 8601).
    pub updated_at: String,
}

/// Contenu d'un dossier (fichiers + sous-dossiers).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FolderContents {
    /// Le dossier lui-meme (None si racine).
    #[serde(default)]
    pub folder: Option<FolderEntry>,
    /// Sous-dossiers.
    #[serde(default)]
    pub subfolders: Vec<FolderEntry>,
    /// Fichiers.
    #[serde(default)]
    pub files: Vec<FileEntry>,
}

// ---------------------------------------------------------------------------
// Partage
// ---------------------------------------------------------------------------

/// Lien de partage externe (Decision D2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareLink {
    /// UUID v4.
    pub id: String,
    /// UUID du fichier partage (exclusif avec `folder_id`).
    #[serde(default)]
    pub file_id: Option<String>,
    /// UUID du dossier partage (exclusif avec `file_id`).
    #[serde(default)]
    pub folder_id: Option<String>,
    /// UUID du profil createur.
    pub creator_id: String,
    /// Token aleatoire (32 bytes hex).
    pub token: String,
    /// Indique si un mot de passe est requis.
    #[serde(default)]
    pub has_password: bool,
    /// Date d'expiration (ISO 8601). OBLIGATOIRE.
    pub expires_at: String,
    /// Nombre max de telechargements (None = illimite).
    #[serde(default)]
    pub max_downloads: Option<u32>,
    /// Compteur de telechargements effectues.
    #[serde(default)]
    pub download_count: u32,
    /// Lien revoque.
    #[serde(default)]
    pub is_revoked: bool,
    /// Date de creation (ISO 8601).
    pub created_at: String,
}

/// Permission de partage interne (Tribu).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharePermission {
    /// UUID v4.
    pub id: String,
    /// UUID du fichier concerne.
    #[serde(default)]
    pub file_id: Option<String>,
    /// UUID du dossier concerne.
    #[serde(default)]
    pub folder_id: Option<String>,
    /// UUID du profil ou tribu beneficiaire.
    pub grantee_id: String,
    /// Type de beneficiaire.
    pub grantee_type: GranteeType,
    /// Niveau de permission.
    pub permission: PermissionLevel,
    /// Date de creation (ISO 8601).
    pub created_at: String,
}

/// Type de beneficiaire d'un partage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GranteeType {
    /// Profil individuel.
    Profile,
    /// Tribu (groupe).
    Tribe,
}

impl GranteeType {
    /// Convertit depuis une chaine.
    pub fn from_str_value(s: &str) -> Self {
        match s {
            "tribe" => Self::Tribe,
            _ => Self::Profile,
        }
    }

    /// Convertit en chaine.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Profile => "profile",
            Self::Tribe => "tribe",
        }
    }
}

/// Niveau de permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionLevel {
    /// Lecture seule.
    Read,
    /// Lecture et ecriture.
    ReadWrite,
}

impl PermissionLevel {
    /// Convertit depuis une chaine.
    pub fn from_str_value(s: &str) -> Self {
        match s {
            "read_write" => Self::ReadWrite,
            _ => Self::Read,
        }
    }

    /// Convertit en chaine.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::ReadWrite => "read_write",
        }
    }
}

// ---------------------------------------------------------------------------
// Synchronisation P2P
// ---------------------------------------------------------------------------

/// Pair de synchronisation (Decision D1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncPeer {
    /// UUID v4.
    pub id: String,
    /// COG ID du pair.
    pub cog_id: String,
    /// Nom affiche du pair.
    #[serde(default)]
    pub display_name: Option<String>,
    /// Cle publique X25519 (base64).
    pub public_key: String,
    /// Derniere connexion (ISO 8601).
    #[serde(default)]
    pub last_seen_at: Option<String>,
    /// Derniere sync reussie (ISO 8601).
    #[serde(default)]
    pub last_sync_at: Option<String>,
    /// Pair de confiance (approuve manuellement).
    #[serde(default)]
    pub is_trusted: bool,
    /// Date de creation (ISO 8601).
    pub created_at: String,
}

/// Etat d'un fichier dans l'horloge vectorielle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorClockEntry {
    /// UUID du fichier.
    pub file_id: String,
    /// COG ID du noeud.
    pub node_id: String,
    /// Compteur logique.
    pub counter: u64,
    /// Date de mise a jour (ISO 8601).
    pub updated_at: String,
}

/// Conflit de synchronisation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncConflict {
    /// UUID v4.
    pub id: String,
    /// UUID du fichier en conflit.
    pub file_id: String,
    /// Chemin de la version locale.
    pub local_version: String,
    /// Chemin de la version distante.
    pub remote_version: String,
    /// COG ID du noeud distant.
    pub remote_node_id: String,
    /// Statut de resolution.
    pub status: ConflictStatus,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Date de resolution (ISO 8601).
    #[serde(default)]
    pub resolved_at: Option<String>,
}

/// Statut de resolution d'un conflit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictStatus {
    /// En attente de resolution.
    Pending,
    /// Resolu en faveur de la version locale.
    ResolvedLocal,
    /// Resolu en faveur de la version distante.
    ResolvedRemote,
    /// Les deux versions conservees.
    ResolvedBoth,
}

impl ConflictStatus {
    /// Convertit depuis une chaine.
    pub fn from_str_value(s: &str) -> Self {
        match s {
            "resolved_local" => Self::ResolvedLocal,
            "resolved_remote" => Self::ResolvedRemote,
            "resolved_both" => Self::ResolvedBoth,
            _ => Self::Pending,
        }
    }

    /// Convertit en chaine.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::ResolvedLocal => "resolved_local",
            Self::ResolvedRemote => "resolved_remote",
            Self::ResolvedBoth => "resolved_both",
        }
    }
}

// ---------------------------------------------------------------------------
// Journal d'acces
// ---------------------------------------------------------------------------

/// Entree du journal d'acces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessLogEntry {
    /// UUID v4.
    pub id: String,
    /// UUID du lien de partage concerne (None si acces interne).
    #[serde(default)]
    pub share_link_id: Option<String>,
    /// IP de l'accedant (hashee pour RGPD).
    #[serde(default)]
    pub accessor_ip: Option<String>,
    /// User-Agent (tronque a 100 caracteres).
    #[serde(default)]
    pub accessor_ua: Option<String>,
    /// Type d'action.
    pub action: AccessAction,
    /// UUID du fichier concerne.
    #[serde(default)]
    pub file_id: Option<String>,
    /// Succes de l'action.
    #[serde(default = "default_true")]
    pub success: bool,
    /// Date de creation (ISO 8601).
    pub created_at: String,
}

const fn default_true() -> bool {
    true
}

/// Type d'action dans le journal d'acces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessAction {
    /// Telechargement.
    Download,
    /// Apercu.
    Preview,
    /// Tentative d'authentification.
    AuthAttempt,
    /// Echec d'authentification.
    AuthFail,
}

impl AccessAction {
    /// Convertit depuis une chaine.
    pub fn from_str_value(s: &str) -> Self {
        match s {
            "preview" => Self::Preview,
            "auth_attempt" => Self::AuthAttempt,
            "auth_fail" => Self::AuthFail,
            // "download" and any unknown value default to Download
            _ => Self::Download,
        }
    }

    /// Convertit en chaine.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Download => "download",
            Self::Preview => "preview",
            Self::AuthAttempt => "auth_attempt",
            Self::AuthFail => "auth_fail",
        }
    }
}

// ---------------------------------------------------------------------------
// Quotas et statistiques
// ---------------------------------------------------------------------------

/// Quota utilisateur.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserQuota {
    /// UUID du profil proprietaire.
    pub owner_id: String,
    /// Limite en octets (0 = illimite).
    #[serde(default)]
    pub max_bytes: u64,
    /// Espace utilise en octets.
    #[serde(default)]
    pub used_bytes: u64,
    /// Date de mise a jour (ISO 8601).
    pub updated_at: String,
}

/// Dossier synchronise (sync selective).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncFolder {
    /// UUID v4.
    pub id: String,
    /// UUID du dossier a synchroniser.
    pub folder_id: String,
    /// Chemin local sur cette machine.
    pub local_path: String,
    /// Sync active.
    #[serde(default = "default_true")]
    pub is_active: bool,
    /// Date de creation (ISO 8601).
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Types supplementaires (spec P1 section 3.2)
// ---------------------------------------------------------------------------

/// Etat global de la synchronisation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    /// Sync en cours.
    #[serde(default)]
    pub is_syncing: bool,
    /// Pairs en ligne.
    #[serde(default)]
    pub peers_online: u32,
    /// Pairs total.
    #[serde(default)]
    pub peers_total: u32,
    /// Uploads en attente.
    #[serde(default)]
    pub pending_uploads: u32,
    /// Downloads en attente.
    #[serde(default)]
    pub pending_downloads: u32,
    /// Conflits en attente.
    #[serde(default)]
    pub conflicts_pending: u32,
    /// Derniere sync reussie (ISO 8601).
    #[serde(default)]
    pub last_sync_at: Option<String>,
}

/// Statistiques de stockage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    /// Nombre total de fichiers.
    #[serde(default)]
    pub total_files: u64,
    /// Nombre total de dossiers.
    #[serde(default)]
    pub total_folders: u64,
    /// Taille totale en octets.
    #[serde(default)]
    pub total_size_bytes: u64,
    /// Fichiers en corbeille.
    #[serde(default)]
    pub trashed_files: u64,
    /// Taille des fichiers en corbeille.
    #[serde(default)]
    pub trashed_size_bytes: u64,
    /// Liens de partage actifs.
    #[serde(default)]
    pub active_shares: u64,
    /// Pairs sync.
    #[serde(default)]
    pub sync_peers: u64,
}

/// Progression d'un upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadProgress {
    /// Nom du fichier.
    pub file_name: String,
    /// Octets envoyes.
    #[serde(default)]
    pub bytes_sent: u64,
    /// Taille totale.
    #[serde(default)]
    pub total_bytes: u64,
    /// Pourcentage (0-100).
    #[serde(default)]
    pub percent: u8,
}

/// Element de breadcrumb pour la navigation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreadcrumbItem {
    /// UUID du dossier (None = racine).
    #[serde(default)]
    pub id: Option<String>,
    /// Nom affiche.
    pub name: String,
}

/// Mode d'affichage de l'explorateur.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewMode {
    /// Affichage en grille.
    #[default]
    Grid,
    /// Affichage en liste.
    List,
}

/// Champ de tri.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    /// Tri par nom.
    #[default]
    Name,
    /// Tri par taille.
    Size,
    /// Tri par date de modification.
    Date,
    /// Tri par type MIME.
    Type,
}

/// Configuration cryptographique stockee en DB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CryptoConfig {
    /// UUID du proprietaire.
    pub owner_id: String,
    /// Sel Argon2 (base64, 16 bytes).
    pub argon2_salt: String,
    /// Canary chiffre (base64) pour verification de passphrase.
    #[serde(default)]
    pub canary_ciphertext: Option<String>,
    /// Nonce du canary (base64).
    #[serde(default)]
    pub canary_nonce: Option<String>,
    /// Version de cle (pour rotation future).
    #[serde(default = "default_key_version")]
    pub key_version: u32,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Date de mise a jour (ISO 8601).
    pub updated_at: String,
}

const fn default_key_version() -> u32 {
    1
}

// ---------------------------------------------------------------------------
// V2: Sessions, TOTP, Recovery Codes, Onboarding (raw DB rows)
// ---------------------------------------------------------------------------

/// Raw DB row for sessions.
#[derive(Debug, Clone)]
pub struct SessionRow {
    /// UUID v4 de la session.
    pub id: String,
    /// UUID du profil proprietaire.
    pub owner_id: String,
    /// Token de session (opaque, unique).
    pub token: String,
    /// Hash SHA-256 de l'IP de creation (RGPD).
    pub created_ip_hash: Option<String>,
    /// User-Agent du navigateur a la creation.
    pub created_ua: Option<String>,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Date d'expiration (ISO 8601).
    pub expires_at: String,
    /// Derniere activite enregistree (ISO 8601).
    pub last_activity_at: String,
    /// Session revoquee.
    pub is_revoked: bool,
    /// TOTP verifie pour cette session.
    pub totp_verified: bool,
}

/// Raw DB row for TOTP secrets.
#[derive(Debug, Clone)]
pub struct TotpRow {
    /// UUID v4 de l'enregistrement.
    pub id: String,
    /// UUID du profil proprietaire.
    pub owner_id: String,
    /// Secret TOTP chiffre (base64).
    pub encrypted_secret: String,
    /// Nonce de chiffrement (base64).
    pub encryption_nonce: String,
    /// Nom de l'emetteur (ex: "MiyuCloud").
    pub issuer: String,
    /// Nom du compte utilisateur.
    pub account_name: String,
    /// Algorithme HMAC (SHA1, SHA256, SHA512).
    pub algorithm: String,
    /// Nombre de chiffres du code OTP.
    pub digits: u32,
    /// Periode de validite en secondes.
    pub period: u32,
    /// Secret TOTP active et fonctionnel.
    pub is_active: bool,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Derniere utilisation du code TOTP (ISO 8601).
    pub last_used_at: Option<String>,
}

/// Raw DB row for recovery codes.
#[derive(Debug, Clone)]
pub struct RecoveryCodeRow {
    /// UUID v4 du code.
    pub id: String,
    /// UUID du profil proprietaire.
    pub owner_id: String,
    /// Hash du code de recuperation.
    pub code_hash: String,
    /// Code deja utilise.
    pub is_used: bool,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Date d'utilisation (ISO 8601).
    pub used_at: Option<String>,
}

/// Raw DB row for onboarding.
#[derive(Debug, Clone)]
pub struct OnboardingRow {
    /// UUID du profil proprietaire.
    pub owner_id: String,
    /// Etape courante de l'onboarding (1-based).
    pub current_step: u32,
    /// Passphrase definie.
    pub passphrase_set: bool,
    /// TOTP configure.
    pub totp_set: bool,
    /// Stockage verifie.
    pub storage_verified: bool,
    /// Onboarding termine.
    pub completed: bool,
    /// Date de completion (ISO 8601).
    pub completed_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_entry_serialize_roundtrip() {
        let entry = FileEntry {
            id: "test-id".into(),
            parent_id: Some("parent-id".into()),
            owner_id: "owner-id".into(),
            name: "photo.jpg".into(),
            mime_type: "image/jpeg".into(),
            size_bytes: 1024,
            checksum_sha256: "abc123".into(),
            encryption_iv: Some("iv-base64".into()),
            chunk_count: 1,
            is_trashed: false,
            trashed_at: None,
            created_at: "2026-03-01T00:00:00Z".into(),
            updated_at: "2026-03-01T00:00:00Z".into(),
        };
        let json = serde_json::to_string(&entry).unwrap();
        let restored: FileEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, restored);
    }

    #[test]
    fn folder_entry_serialize_roundtrip() {
        let entry = FolderEntry {
            id: "folder-id".into(),
            parent_id: None,
            owner_id: "owner-id".into(),
            name: "Photos".into(),
            is_trashed: false,
            trashed_at: None,
            created_at: "2026-03-01T00:00:00Z".into(),
            updated_at: "2026-03-01T00:00:00Z".into(),
        };
        let json = serde_json::to_string(&entry).unwrap();
        let restored: FolderEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, restored);
    }

    #[test]
    fn share_link_serialize_roundtrip() {
        let link = ShareLink {
            id: "share-id".into(),
            file_id: Some("file-id".into()),
            folder_id: None,
            creator_id: "creator-id".into(),
            token: "abcdef1234567890".into(),
            has_password: false,
            expires_at: "2026-04-01T00:00:00Z".into(),
            max_downloads: Some(10),
            download_count: 0,
            is_revoked: false,
            created_at: "2026-03-01T00:00:00Z".into(),
        };
        let json = serde_json::to_string(&link).unwrap();
        let restored: ShareLink = serde_json::from_str(&json).unwrap();
        assert_eq!(link, restored);
    }

    #[test]
    fn share_permission_serialize_roundtrip() {
        let perm = SharePermission {
            id: "perm-id".into(),
            file_id: Some("file-id".into()),
            folder_id: None,
            grantee_id: "grantee-id".into(),
            grantee_type: GranteeType::Profile,
            permission: PermissionLevel::Read,
            created_at: "2026-03-01T00:00:00Z".into(),
        };
        let json = serde_json::to_string(&perm).unwrap();
        let restored: SharePermission = serde_json::from_str(&json).unwrap();
        assert_eq!(perm, restored);
    }

    #[test]
    fn grantee_type_roundtrip() {
        assert_eq!(GranteeType::from_str_value("profile"), GranteeType::Profile);
        assert_eq!(GranteeType::from_str_value("tribe"), GranteeType::Tribe);
        assert_eq!(GranteeType::Profile.as_str(), "profile");
        assert_eq!(GranteeType::Tribe.as_str(), "tribe");
    }

    #[test]
    fn permission_level_roundtrip() {
        assert_eq!(PermissionLevel::from_str_value("read"), PermissionLevel::Read);
        assert_eq!(
            PermissionLevel::from_str_value("read_write"),
            PermissionLevel::ReadWrite
        );
        assert_eq!(PermissionLevel::Read.as_str(), "read");
        assert_eq!(PermissionLevel::ReadWrite.as_str(), "read_write");
    }

    #[test]
    fn conflict_status_roundtrip() {
        assert_eq!(
            ConflictStatus::from_str_value("pending"),
            ConflictStatus::Pending
        );
        assert_eq!(
            ConflictStatus::from_str_value("resolved_local"),
            ConflictStatus::ResolvedLocal
        );
        assert_eq!(ConflictStatus::Pending.as_str(), "pending");
    }

    #[test]
    fn access_action_roundtrip() {
        assert_eq!(
            AccessAction::from_str_value("download"),
            AccessAction::Download
        );
        assert_eq!(
            AccessAction::from_str_value("auth_fail"),
            AccessAction::AuthFail
        );
        assert_eq!(AccessAction::Download.as_str(), "download");
    }

    #[test]
    fn view_mode_default_is_grid() {
        assert_eq!(ViewMode::default(), ViewMode::Grid);
    }

    #[test]
    fn sort_field_default_is_name() {
        assert_eq!(SortField::default(), SortField::Name);
    }

    #[test]
    fn folder_contents_serialize_roundtrip() {
        let contents = FolderContents {
            folder: None,
            subfolders: vec![],
            files: vec![],
        };
        let json = serde_json::to_string(&contents).unwrap();
        let restored: FolderContents = serde_json::from_str(&json).unwrap();
        assert_eq!(contents, restored);
    }

    #[test]
    fn crypto_config_serialize_roundtrip() {
        let cfg = CryptoConfig {
            owner_id: "owner".into(),
            argon2_salt: "c2FsdA==".into(),
            canary_ciphertext: Some("encrypted".into()),
            canary_nonce: Some("nonce".into()),
            key_version: 1,
            created_at: "2026-03-01T00:00:00Z".into(),
            updated_at: "2026-03-01T00:00:00Z".into(),
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let restored: CryptoConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg, restored);
    }
}
