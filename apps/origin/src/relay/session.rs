//! Session de connexion relay.
//!
//! Représente une connexion active entre un COG et le relay.
//!
//! Note : Code préparé pour fonctionnalités futures (phases de vérification).
#![allow(dead_code)]

use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::protocol::{CogType, OsType, PassportType, SESSION_ID_SIZE};

/// État d'une session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// En attente d'enregistrement.
    Pending,
    /// Vérification Phase A (Clé Cores) en cours.
    VerifyingPhaseA,
    /// Vérification Phase B (Services) en cours.
    VerifyingPhaseB,
    /// Vérification Phase C (Santé) en cours.
    VerifyingPhaseC,
    /// Enregistré et actif.
    Active,
    /// En cours de fermeture.
    Closing,
    /// Fermé.
    Closed,
}

/// Informations du Permis de circulation délivré.
#[derive(Debug, Clone)]
pub struct PermisInfo {
    /// Identifiant du Permis.
    pub permis_id: String,
    /// Date de délivrance.
    pub issued_at: DateTime<Utc>,
    /// Date d'expiration.
    pub expires_at: DateTime<Utc>,
    /// Portée du Permis (JSON).
    pub scope: Bytes,
}

/// Session de connexion relay.
#[derive(Debug)]
pub struct Session {
    /// Identifiant unique de la session.
    pub id: [u8; SESSION_ID_SIZE],
    /// Adresse du client.
    pub peer_addr: SocketAddr,
    /// État de la session.
    pub state: SessionState,
    /// Date de création.
    pub created_at: DateTime<Utc>,
    /// Dernière activité.
    pub last_activity: DateTime<Utc>,
    
    // Informations du COG (remplies après REGISTER)
    /// Identifiant du COG.
    pub cog_id: Option<String>,
    /// Type de COG.
    pub cog_type: Option<CogType>,
    /// Type d'OS.
    pub os_type: Option<OsType>,
    /// Version des Cores.
    pub core_version: Option<String>,
    /// Type de passeport.
    pub passport_type: Option<PassportType>,
    /// cog_id du parent (pour TERMINAL).
    pub parent_cog_id: Option<String>,

    // Informations de vérification
    /// Rapport de santé (REGISTER) pour Phase C en mode strict.
    pub environment_health: Option<Bytes>,
    /// Résultat Phase A.
    pub phase_a_passed: bool,
    /// Résultat Phase B (services vérifiés).
    pub verified_services: Vec<String>,
    /// Résultat Phase C.
    pub phase_c_passed: bool,

    // Permis délivré
    /// Informations du Permis.
    pub permis: Option<PermisInfo>,

    // Compteurs
    /// Numéro de séquence pour les messages DATA.
    pub sequence_number: u32,
    /// Nombre de messages reçus.
    pub messages_received: u64,
    /// Nombre de messages envoyés.
    pub messages_sent: u64,

    // Clé de session pour HMAC
    /// Clé de session dérivée.
    pub session_key: Option<[u8; 32]>,
}

impl Session {
    /// Crée une nouvelle session.
    #[must_use]
    pub fn new(peer_addr: SocketAddr) -> Self {
        let id = Uuid::new_v4().into_bytes();
        let now = Utc::now();

        Self {
            id,
            peer_addr,
            state: SessionState::Pending,
            created_at: now,
            last_activity: now,
            cog_id: None,
            cog_type: None,
            os_type: None,
            core_version: None,
            passport_type: None,
            parent_cog_id: None,
            environment_health: None,
            phase_a_passed: false,
            verified_services: Vec::new(),
            phase_c_passed: false,
            permis: None,
            sequence_number: 0,
            messages_received: 0,
            messages_sent: 0,
            session_key: None,
        }
    }

    /// Met à jour l'heure de dernière activité.
    pub fn touch(&mut self) {
        self.last_activity = Utc::now();
    }

    /// Vérifie si la session est expirée.
    #[must_use]
    pub fn is_expired(&self, timeout_seconds: i64) -> bool {
        let elapsed = Utc::now() - self.last_activity;
        elapsed.num_seconds() > timeout_seconds
    }

    /// Vérifie si la session est active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.state == SessionState::Active
    }

    /// Définit les informations du COG après parsing du REGISTER.
    pub fn set_cog_info(
        &mut self,
        cog_id: String,
        cog_type: CogType,
        os_type: OsType,
        core_version: String,
        passport_type: PassportType,
        parent_cog_id: Option<String>,
    ) {
        self.cog_id = Some(cog_id);
        self.cog_type = Some(cog_type);
        self.os_type = Some(os_type);
        self.core_version = Some(core_version);
        self.passport_type = Some(passport_type);
        self.parent_cog_id = parent_cog_id;
    }

    /// Stocke le rapport de santé (Phase C en mode strict).
    pub fn set_environment_health(&mut self, health: Bytes) {
        self.environment_health = if health.is_empty() {
            None
        } else {
            Some(health)
        };
    }

    /// Passe à la phase de vérification A.
    pub fn start_phase_a(&mut self) {
        self.state = SessionState::VerifyingPhaseA;
    }

    /// Marque la phase A comme réussie.
    pub fn complete_phase_a(&mut self) {
        self.phase_a_passed = true;
        self.state = SessionState::VerifyingPhaseB;
    }

    /// Ajoute un service vérifié (Phase B).
    pub fn add_verified_service(&mut self, service_id: String) {
        self.verified_services.push(service_id);
    }

    /// Passe à la phase C.
    pub fn start_phase_c(&mut self) {
        self.state = SessionState::VerifyingPhaseC;
    }

    /// Marque la phase C comme réussie.
    pub fn complete_phase_c(&mut self) {
        self.phase_c_passed = true;
    }

    /// Active la session avec un Permis.
    pub fn activate(&mut self, permis: PermisInfo, session_key: [u8; 32]) {
        self.permis = Some(permis);
        self.session_key = Some(session_key);
        self.state = SessionState::Active;
    }

    /// Ferme la session.
    pub fn close(&mut self) {
        self.state = SessionState::Closed;
    }

    /// Incrémente le numéro de séquence.
    pub fn next_sequence(&mut self) -> u32 {
        let seq = self.sequence_number;
        self.sequence_number = self.sequence_number.wrapping_add(1);
        seq
    }

    /// Génère un identifiant de Permis.
    #[must_use]
    pub fn generate_permis_id(&self) -> String {
        format!("permis-{}-{}", hex::encode(&self.id[..8]), Utc::now().timestamp())
    }
}

/// Gestionnaire de sessions.
#[derive(Debug)]
pub struct SessionManager {
    /// Sessions actives (par session_id).
    sessions: Arc<RwLock<std::collections::HashMap<[u8; SESSION_ID_SIZE], Arc<RwLock<Session>>>>>,
    /// Index par cog_id.
    cog_index: Arc<RwLock<std::collections::HashMap<String, [u8; SESSION_ID_SIZE]>>>,
    /// Timeout des sessions (secondes).
    timeout_seconds: i64,
}

impl SessionManager {
    /// Crée un nouveau gestionnaire.
    #[must_use]
    pub fn new(timeout_seconds: i64) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
            cog_index: Arc::new(RwLock::new(std::collections::HashMap::new())),
            timeout_seconds,
        }
    }

    /// Crée une nouvelle session.
    pub async fn create(&self, peer_addr: SocketAddr) -> Arc<RwLock<Session>> {
        let session = Session::new(peer_addr);
        let session_id = session.id;
        let session = Arc::new(RwLock::new(session));

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, Arc::clone(&session));

        session
    }

    /// Récupère une session par son ID.
    pub async fn get(&self, session_id: &[u8; SESSION_ID_SIZE]) -> Option<Arc<RwLock<Session>>> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Récupère une session par cog_id.
    pub async fn get_by_cog_id(&self, cog_id: &str) -> Option<Arc<RwLock<Session>>> {
        let index = self.cog_index.read().await;
        if let Some(session_id) = index.get(cog_id) {
            self.get(session_id).await
        } else {
            None
        }
    }

    /// Enregistre le mapping cog_id → session.
    pub async fn register_cog(&self, cog_id: String, session_id: [u8; SESSION_ID_SIZE]) {
        let mut index = self.cog_index.write().await;
        index.insert(cog_id, session_id);
    }

    /// Vérifie si un cog_id est déjà enregistré.
    pub async fn is_cog_registered(&self, cog_id: &str) -> bool {
        let index = self.cog_index.read().await;
        index.contains_key(cog_id)
    }

    /// Supprime une session.
    pub async fn remove(&self, session_id: &[u8; SESSION_ID_SIZE]) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.remove(session_id) {
            let session = session.read().await;
            if let Some(ref cog_id) = session.cog_id {
                let mut index = self.cog_index.write().await;
                index.remove(cog_id);
            }
        }
    }

    /// Nettoie les sessions expirées.
    pub async fn cleanup_expired(&self) -> usize {
        let mut expired = Vec::new();

        {
            let sessions = self.sessions.read().await;
            for (id, session) in sessions.iter() {
                let session = session.read().await;
                if session.is_expired(self.timeout_seconds) {
                    expired.push(*id);
                }
            }
        }

        for id in &expired {
            self.remove(id).await;
        }

        expired.len()
    }

    /// Compte le nombre de sessions actives.
    pub async fn count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_session_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345);
        let session = Session::new(addr);

        assert_eq!(session.state, SessionState::Pending);
        assert!(session.cog_id.is_none());
        assert!(!session.is_active());
    }

    #[test]
    fn test_session_lifecycle() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345);
        let mut session = Session::new(addr);

        // Set COG info
        session.set_cog_info(
            "test-cog".to_string(),
            CogType::Stable,
            OsType::Linux,
            "1.0.0".to_string(),
            PassportType::Standard,
            None,
        );

        assert_eq!(session.cog_id, Some("test-cog".to_string()));

        // Phase A
        session.start_phase_a();
        assert_eq!(session.state, SessionState::VerifyingPhaseA);

        session.complete_phase_a();
        assert!(session.phase_a_passed);
        assert_eq!(session.state, SessionState::VerifyingPhaseB);

        // Phase B
        session.add_verified_service("jayfestival".to_string());
        assert_eq!(session.verified_services.len(), 1);

        // Phase C
        session.start_phase_c();
        session.complete_phase_c();
        assert!(session.phase_c_passed);

        // Activate
        let permis = PermisInfo {
            permis_id: session.generate_permis_id(),
            issued_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
            scope: Bytes::from_static(b"{}"),
        };
        session.activate(permis, [0u8; 32]);

        assert!(session.is_active());
    }

    #[tokio::test]
    async fn test_session_manager() {
        let manager = SessionManager::new(300);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345);

        // Create session
        let session = manager.create(addr).await;
        let session_id = {
            let s = session.read().await;
            s.id
        };

        // Get by ID
        assert!(manager.get(&session_id).await.is_some());

        // Register COG
        {
            let mut s = session.write().await;
            s.set_cog_info(
                "test-cog".to_string(),
                CogType::Stable,
                OsType::Linux,
                "1.0.0".to_string(),
                PassportType::Standard,
                None,
            );
        }
        manager.register_cog("test-cog".to_string(), session_id).await;

        // Get by cog_id
        assert!(manager.get_by_cog_id("test-cog").await.is_some());
        assert!(manager.is_cog_registered("test-cog").await);

        // Count
        assert_eq!(manager.count().await, 1);

        // Remove
        manager.remove(&session_id).await;
        assert!(manager.get(&session_id).await.is_none());
    }
}
