//! Etat local de l'UI MiyuCloud dans Central.
//!
//! @id: miyucloud_ui_state
//! @do: manage_miyucloud_ui_state
//! @role: state
//! @layer: presentation
//!
//! Les types domaine sont definis localement en attendant que le crate `miyucloud`
//! exporte ses types publics. Chaque type porte un commentaire de remplacement futur.

use serde::{Deserialize, Serialize};

// ════════════════════════════════════════════════════════════════════════════
// Types domaine locaux (replicas du crate miyucloud)
// TODO: replace with miyucloud::data::types::* when crate is ready
// ════════════════════════════════════════════════════════════════════════════

/// Entree fichier dans le cloud.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FileEntry {
    pub id: String,
    pub parent_id: Option<String>,
    pub owner_id: String,
    pub name: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub checksum_sha256: String,
    pub encryption_iv: Option<String>,
    pub chunk_count: u32,
    pub is_trashed: bool,
    pub trashed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Entree dossier dans le cloud.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FolderEntry {
    pub id: String,
    pub parent_id: Option<String>,
    pub owner_id: String,
    pub name: String,
    pub is_trashed: bool,
    pub trashed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Contenu d'un dossier (fichiers + sous-dossiers).
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct FolderContents {
    pub folder: Option<FolderEntry>,
    pub subfolders: Vec<FolderEntry>,
    pub files: Vec<FileEntry>,
}

/// Lien de partage externe.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ShareLink {
    /// UUID v4 du lien.
    pub id: String,
    /// ID du fichier partage (exclusif avec folder_id).
    pub file_id: Option<String>,
    /// ID du dossier partage (exclusif avec file_id).
    pub folder_id: Option<String>,
    /// ID du profil createur.
    pub creator_id: String,
    /// Token unique du lien (64 hex chars).
    pub token: String,
    /// True si un mot de passe est requis.
    #[serde(default)]
    pub has_password: bool,
    /// Date d'expiration ISO 8601.
    pub expires_at: String,
    /// Nombre max de telechargements (None = illimite).
    pub max_downloads: Option<u32>,
    /// Nombre de telechargements effectues.
    #[serde(default)]
    pub download_count: u32,
    /// True si le lien a ete revoque.
    #[serde(default)]
    pub is_revoked: bool,
    /// Date de creation ISO 8601.
    pub created_at: String,
}

/// Permission de partage interne (Tribu).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SharePermission {
    /// UUID v4.
    pub id: String,
    /// ID du fichier concerne.
    pub file_id: Option<String>,
    /// ID du dossier concerne.
    pub folder_id: Option<String>,
    /// ID du profil ou tribu beneficiaire.
    pub grantee_id: String,
    /// Type de beneficiaire.
    pub grantee_type: GranteeType,
    /// Niveau de permission.
    pub permission: PermissionLevel,
    /// Date de creation ISO 8601.
    pub created_at: String,
}

/// Type de beneficiaire d'un partage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GranteeType {
    /// Profil individuel.
    Profile,
    /// Tribu entiere.
    Tribe,
}

/// Niveau de permission de partage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionLevel {
    /// Lecture seule.
    Read,
    /// Lecture et ecriture.
    ReadWrite,
}

/// Element partage avec moi (fichier ou dossier + permission).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SharedWithMeEntry {
    /// Permission de partage.
    pub permission: SharePermission,
    /// Nom de la ressource partagee.
    pub resource_name: String,
    /// Nom du proprietaire.
    pub owner_name: String,
}

/// Requete de creation de lien de partage.
#[derive(Debug, Clone, Serialize)]
pub struct CreateShareLinkRequest {
    /// ID du fichier a partager (exclusif avec folder_id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// ID du dossier a partager (exclusif avec file_id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// Mot de passe optionnel pour proteger le lien.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Duree de validite en heures.
    pub expires_in_hours: u32,
    /// Nombre max de telechargements (None = illimite).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_downloads: Option<u32>,
}

/// Requete de partage avec un profil ou tribu.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSharePermissionRequest {
    /// ID du fichier a partager (exclusif avec folder_id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// ID du dossier a partager (exclusif avec file_id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// ID du beneficiaire (profil ou tribu).
    pub grantee_id: String,
    /// Type de beneficiaire.
    pub grantee_type: GranteeType,
    /// Niveau de permission.
    pub permission: PermissionLevel,
}

/// Quota utilisateur.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UserQuota {
    /// ID du proprietaire.
    pub owner_id: String,
    /// Quota maximal en octets (0 = illimite).
    pub max_bytes: u64,
    /// Espace utilise en octets.
    pub used_bytes: u64,
    /// Date de derniere mise a jour.
    pub updated_at: String,
}

/// Statistiques de stockage.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
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
    /// Nombre de fichiers en corbeille.
    #[serde(default)]
    pub trashed_files: u64,
    /// Taille des fichiers en corbeille.
    #[serde(default)]
    pub trashed_size_bytes: u64,
    /// Nombre de partages actifs.
    #[serde(default)]
    pub active_shares: u64,
    /// Nombre de pairs sync.
    #[serde(default)]
    pub sync_peers: u64,
}

/// Session d'authentification active.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AuthSession {
    /// UUID v4 de session.
    pub id: String,
    /// ID du proprietaire.
    pub owner_id: String,
    /// Date de creation ISO 8601.
    pub created_at: String,
    /// Date d'expiration ISO 8601.
    pub expires_at: String,
    /// Date de derniere activite ISO 8601.
    pub last_activity_at: String,
    /// True si la session est verifiee 2FA.
    pub totp_verified: bool,
    /// Hash IP (optionnel).
    #[serde(default)]
    pub created_ip_hash: Option<String>,
    /// User-Agent tronque (optionnel).
    #[serde(default)]
    pub created_ua: Option<String>,
}

/// Reponse de setup TOTP.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TotpSetupData {
    /// URI otpauth.
    pub otpauth_uri: String,
    /// Codes de recuperation affiches une seule fois.
    pub recovery_codes: Vec<String>,
}

/// Statut onboarding.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct OnboardingStatus {
    /// Etape courante (1..4).
    pub current_step: u32,
    /// Passphrase configuree.
    pub passphrase_set: bool,
    /// 2FA configuree.
    pub totp_set: bool,
    /// Stockage valide.
    pub storage_verified: bool,
    /// Onboarding termine.
    pub completed: bool,
    /// Date de completion.
    #[serde(default)]
    pub completed_at: Option<String>,
}

/// Metriques de health.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HealthMetrics {
    pub total_files: u64,
    pub total_folders: u64,
    pub total_size_bytes: u64,
    pub active_share_links: u64,
    pub active_sessions: u64,
    pub sync_peers: u64,
}

/// Statut de sante detaille.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub db_accessible: bool,
    pub storage_accessible: bool,
    pub storage_path: String,
    pub disk_free_bytes: u64,
    pub disk_total_bytes: u64,
    pub file_count: u64,
    pub total_size_bytes: u64,
    pub uptime_seconds: u64,
    pub metrics: HealthMetrics,
}

// ════════════════════════════════════════════════════════════════════════════
// Types UI specifiques a Central
// ════════════════════════════════════════════════════════════════════════════

/// Mode d'affichage des fichiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    /// Affichage en grille (icones larges).
    #[default]
    Grid,
    /// Affichage en liste (tableau detaille).
    List,
}

/// Champ de tri pour la liste de fichiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortField {
    /// Tri alphabetique par nom.
    #[default]
    Name,
    /// Tri par taille.
    Size,
    /// Tri par date de modification.
    Date,
    /// Tri par type MIME.
    Type,
}

/// Progression d'un televersement en cours.
#[derive(Debug, Clone)]
pub struct UploadProgress {
    /// Nom du fichier en cours d'upload.
    pub file_name: String,
    /// Octets envoyes.
    pub bytes_sent: u64,
    /// Taille totale en octets.
    pub bytes_total: u64,
}

/// Element du fil d'Ariane (breadcrumb).
#[derive(Debug, Clone, PartialEq)]
pub struct BreadcrumbItem {
    /// ID du dossier (None = racine).
    pub id: Option<String>,
    /// Nom affiche.
    pub name: String,
}

/// Pair de synchronisation P2P.
///
/// Les champs utilisent `serde(alias)` pour accepter les noms du crate `miyucloud`
/// (ex. `display_name`, `last_seen_at`) et les noms courts de l'UI.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SyncPeer {
    /// UUID v4 du pair.
    pub id: String,
    /// Identifiant COG du pair.
    pub cog_id: String,
    /// Nom lisible du pair (optionnel).
    #[serde(default, alias = "display_name")]
    pub name: Option<String>,
    /// Cle publique du pair (hex).
    pub public_key: String,
    /// True si le pair est approuve (trusted).
    #[serde(default)]
    pub is_trusted: bool,
    /// True si le pair est actuellement en ligne (calcule par l'UI, absent du serveur).
    #[serde(default)]
    pub is_online: bool,
    /// Date ISO 8601 de derniere connexion.
    #[serde(default, alias = "last_seen_at")]
    pub last_seen: Option<String>,
    /// Date ISO 8601 de derniere synchronisation.
    #[serde(default, alias = "last_sync_at")]
    pub last_sync: Option<String>,
    /// Date de creation du pair.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Statut global de la synchronisation P2P.
///
/// Accepte les noms de champs du crate `miyucloud` via `serde(alias)`.
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct SyncStatus {
    /// True si une synchronisation est en cours.
    #[serde(default)]
    pub is_syncing: bool,
    /// Nombre de pairs actuellement en ligne.
    #[serde(default)]
    pub peers_online: u32,
    /// Nombre total de pairs enregistres.
    #[serde(default)]
    pub peers_total: u32,
    /// Nombre de conflits en attente de resolution.
    #[serde(default, alias = "conflicts_pending")]
    pub pending_conflicts: u32,
    /// Uploads en attente (fourni par le serveur).
    #[serde(default)]
    pub pending_uploads: u32,
    /// Downloads en attente (fourni par le serveur).
    #[serde(default)]
    pub pending_downloads: u32,
    /// Date ISO 8601 de la derniere synchronisation reussie.
    #[serde(default, alias = "last_sync_at")]
    pub last_sync: Option<String>,
    /// Progression de la sync en cours (0.0 - 1.0, calcule par l'UI).
    #[serde(default)]
    pub sync_progress: Option<f64>,
}

/// Conflit de synchronisation entre ce noeud et un pair.
///
/// Les champs `local_version` et `remote_version` du crate sont aliases
/// vers `local_modified` et `remote_modified` pour l'affichage UI.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SyncConflict {
    /// UUID v4 du conflit.
    pub id: String,
    /// Nom du fichier en conflit (calcule par l'UI ou fourni par le serveur).
    #[serde(default)]
    pub file_name: String,
    /// UUID v4 du fichier concerne.
    pub file_id: String,
    /// Version locale (chemin ou date ISO 8601).
    #[serde(default, alias = "local_version")]
    pub local_modified: String,
    /// Version distante (chemin ou date ISO 8601).
    #[serde(default, alias = "remote_version")]
    pub remote_modified: String,
    /// ID du noeud distant.
    pub remote_node_id: String,
    /// Statut du conflit : "pending", "resolved_local", "resolved_remote", "resolved_both".
    pub status: String,
    /// Date de creation (ISO 8601).
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de resolution (ISO 8601).
    #[serde(default)]
    pub resolved_at: Option<String>,
}

/// Requete d'enregistrement d'un pair.
///
/// Le serveur attend `display_name`, donc on serialise `name` sous ce nom.
#[derive(Debug, Clone, Serialize)]
pub struct RegisterPeerRequest {
    /// Identifiant COG du pair.
    pub cog_id: String,
    /// Cle publique du pair (hex).
    pub public_key: String,
    /// Nom lisible du pair (optionnel). Serialise en `display_name` pour le serveur.
    #[serde(skip_serializing_if = "Option::is_none", rename = "display_name")]
    pub name: Option<String>,
}

/// Requete de resolution d'un conflit.
#[derive(Debug, Clone, Serialize)]
pub struct ResolveConflictRequest {
    /// Strategie de resolution : "keep_local" ou "keep_remote".
    pub resolution: String,
}

/// Statut de synchronisation P2P pour l'UI.
#[derive(Debug, Clone, Default)]
pub struct SyncStatusInfo {
    /// Nombre de pairs connectes.
    pub peers_online: u32,
    /// True si une sync est en cours.
    pub syncing: bool,
    /// Nombre de conflits en attente.
    pub pending_conflicts: u32,
    /// Derniere date de sync reussie.
    pub last_sync: Option<String>,
}

/// Section active dans la sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CloudSection {
    /// Explorateur de fichiers (par defaut).
    #[default]
    Files,
    /// Partages (liens actifs + partages Tribu).
    Shares,
    /// Corbeille.
    Trash,
    /// Parametres MiyuCloud.
    Settings,
}

/// Etat local de l'UI MiyuCloud dans Central.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools, dead_code)]
pub struct MiyuCloudState {
    /// Dossier actuellement affiche (None = racine).
    pub current_folder_id: Option<String>,
    /// Contenu du dossier courant.
    pub current_contents: FolderContents,
    /// Fichier selectionne (pour le panel detail).
    pub selected_file: Option<FileEntry>,
    /// Mode d'affichage (grille ou liste).
    pub view_mode: ViewMode,
    /// Tri actif.
    pub sort_by: SortField,
    /// Sens du tri (true = ascendant).
    pub sort_asc: bool,
    /// Upload en cours.
    pub upload_progress: Option<UploadProgress>,
    /// Etat de la sync (placeholder Phase A).
    pub sync_status: SyncStatusInfo,
    /// Dialog de partage ouvert.
    pub share_dialog_open: bool,
    /// Breadcrumb (chemin du dossier courant).
    pub breadcrumb: Vec<BreadcrumbItem>,
    /// Section active (fichiers ou corbeille).
    pub section: CloudSection,
    /// Message d'erreur a afficher.
    pub error_message: Option<String>,
    /// Chargement en cours.
    pub loading: bool,
    /// Modal de creation de dossier ouverte.
    pub new_folder_dialog: bool,
    /// Nom saisi pour le nouveau dossier.
    pub new_folder_name: String,
    /// Modal de renommage ouverte.
    pub rename_dialog: bool,
    /// Nom saisi pour le renommage.
    pub rename_value: String,
    /// Dossiers connus pour la sidebar (arborescence).
    pub folder_tree: Vec<FolderEntry>,
    /// Dossiers deplies dans la sidebar.
    pub expanded_folders: Vec<String>,
    /// Liens de partage actifs.
    pub share_links: Vec<ShareLink>,
    /// Fichier ou dossier a partager (ID pour le dialog).
    pub share_target_file_id: Option<String>,
    /// Dossier cible du partage (pour le dialog).
    pub share_target_folder_id: Option<String>,
    /// Lien de partage genere (affiche dans le dialog).
    pub generated_share_url: Option<String>,
    /// Confirmation de suppression definitive en cours (ID de l'element).
    pub confirm_purge_id: Option<String>,
    /// Statistiques de stockage (pour Settings).
    pub storage_stats: StorageStats,
    /// Quota utilisateur (pour Settings).
    pub user_quota: Option<UserQuota>,
    /// Chemin de stockage (pour Settings).
    pub storage_path: String,
    /// Surface web activee (pour Settings).
    pub web_surface_enabled: bool,
    /// Port de la surface web (pour Settings).
    pub web_surface_port: u16,
    /// Pairs de synchronisation connus.
    pub sync_peers: Vec<SyncPeer>,
    /// Conflits de synchronisation en cours.
    pub sync_conflicts: Vec<SyncConflict>,
    /// Panel sync ouvert (clic sur le badge).
    pub sync_panel_open: bool,
    /// Formulaire d'ajout de pair ouvert.
    pub add_peer_form_open: bool,
    /// Saisie IP pour ajout de pair.
    pub add_peer_ip: String,
    /// Saisie port pour ajout de pair.
    pub add_peer_port: String,
    /// Sessions d'auth actives.
    pub auth_sessions: Vec<AuthSession>,
    /// 2FA activee.
    pub totp_enabled: bool,
    /// URI otpauth du setup courant.
    pub totp_otpauth_uri: Option<String>,
    /// Codes de recuperation recents.
    pub recovery_codes: Vec<String>,
    /// Modal recovery codes ouverte.
    pub recovery_modal_open: bool,
    /// Compte pour le setup TOTP.
    pub totp_account_name: String,
    /// Code TOTP saisi.
    pub totp_code_input: String,
    /// Code recovery saisi.
    pub recovery_code_input: String,
    /// Statut onboarding.
    pub onboarding_status: Option<OnboardingStatus>,
    /// Statut health detaille.
    pub health_status: Option<HealthStatus>,
}

impl Default for MiyuCloudState {
    fn default() -> Self {
        Self {
            current_folder_id: None,
            current_contents: FolderContents::default(),
            selected_file: None,
            view_mode: ViewMode::Grid,
            sort_by: SortField::Name,
            sort_asc: true,
            upload_progress: None,
            sync_status: SyncStatusInfo::default(),
            share_dialog_open: false,
            breadcrumb: vec![BreadcrumbItem {
                id: None,
                name: "Mon Cloud".to_string(),
            }],
            section: CloudSection::Files,
            error_message: None,
            loading: false,
            new_folder_dialog: false,
            new_folder_name: String::new(),
            rename_dialog: false,
            rename_value: String::new(),
            folder_tree: Vec::new(),
            expanded_folders: Vec::new(),
            share_links: Vec::new(),
            share_target_file_id: None,
            share_target_folder_id: None,
            generated_share_url: None,
            confirm_purge_id: None,
            storage_stats: StorageStats::default(),
            user_quota: None,
            storage_path: String::new(),
            web_surface_enabled: false,
            web_surface_port: 11442,
            sync_peers: Vec::new(),
            sync_conflicts: Vec::new(),
            sync_panel_open: false,
            add_peer_form_open: false,
            add_peer_ip: String::new(),
            add_peer_port: "11440".to_string(),
            auth_sessions: Vec::new(),
            totp_enabled: false,
            totp_otpauth_uri: None,
            recovery_codes: Vec::new(),
            recovery_modal_open: false,
            totp_account_name: "owner@miyucloud.local".to_string(),
            totp_code_input: String::new(),
            recovery_code_input: String::new(),
            onboarding_status: None,
            health_status: None,
        }
    }
}
