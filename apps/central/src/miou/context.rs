//! Construction du BotContext depuis les sources de données.

use chrono::{DateTime, Timelike, Utc};
use std::collections::HashMap;

/// Contexte passé au moteur de décision et à l'injecteur de templates.
/// Toutes les sources sont agrégées ici.
#[derive(Debug, Clone)]
pub struct BotContext {
    // --- Session ---
    /// Début de la session.
    pub session_start: Option<DateTime<Utc>>,
    /// Durée de la session en minutes.
    pub session_duration_minutes: u32,
    /// Première connexion de cette session.
    pub is_first_connection_of_session: bool,

    // --- Profil ---
    /// Pseudo de l'utilisateur.
    pub pseudo: String,
    /// Langue (ex: "fr").
    pub langue: String,

    // --- MiyukiniWatch (ou défaut) ---
    /// Jours depuis la dernière visite (None si première connexion).
    pub jours_depuis_derniere_visite: Option<u32>,
    /// Service le plus utilisé (nom, minutes).
    pub service_le_plus_utilise: Option<(String, u32)>,
    /// Service délaissé (nom, jours sans utilisation).
    pub service_delaisse: Option<(String, u32)>,
    /// Ami le plus délaissé (pseudo, jours sans contact).
    pub ami_plus_delaisse: Option<(String, u32)>,
    /// Ami connecté récemment (pseudo).
    pub ami_connecte_recemment: Option<String>,

    // --- JayKoa ---
    /// Événement prochain (titre, date).
    pub evenement_prochain: Option<(String, DateTime<Utc>)>,
    /// Événement dans moins d'une heure.
    pub evenement_dans_moins_d_une_heure: bool,

    // --- Paramètres dérivés des préférences ---
    /// Seuil de pause en minutes.
    pub seuil_pause_minutes: u32,
    /// Maximum de bulles par session.
    pub max_bulles_par_session: u32,
    /// Bulles déjà affichées cette session.
    pub bulles_deja_affichees: u32,
    /// Délai minimum entre deux bulles (secondes).
    pub delai_min_entre_bulles_secs: u32,
    /// Bulles activées.
    pub bulles_actives: bool,
    /// Mode DND actif.
    pub dnd_actif: bool,
    /// Rappels de pause activés.
    pub rappels_pause_actives: bool,

    // --- Specs machine ---
    /// RAM disponible (MB).
    pub ram_available_mb: u32,
    /// Espace disque libre (GB).
    pub disk_free_gb: f32,
    /// Type d'OS.
    pub os_type: String,
    /// Specs améliorées depuis la dernière visite.
    pub specs_upgraded_since_last: bool,

    // --- Réponses utilisateur (optionnel 0.1.x) ---
    /// Réponses utilisateur stockées (clé -> valeur).
    pub user_responses: HashMap<String, String>,

    // --- Relation (0.1.x simplifié) ---
    /// Niveau de relation (0 = inconnue, 1-5 = paliers).
    pub relation_level: u8,
}

impl Default for BotContext {
    fn default() -> Self {
        Self {
            session_start: Some(Utc::now()),
            session_duration_minutes: 0,
            is_first_connection_of_session: true,
            pseudo: "habitant".to_string(),
            langue: "fr".to_string(),
            jours_depuis_derniere_visite: None,
            service_le_plus_utilise: None,
            service_delaisse: None,
            ami_plus_delaisse: None,
            ami_connecte_recemment: None,
            evenement_prochain: None,
            evenement_dans_moins_d_une_heure: false,
            seuil_pause_minutes: 120,
            max_bulles_par_session: 5,
            bulles_deja_affichees: 0,
            delai_min_entre_bulles_secs: 30,
            bulles_actives: true,
            dnd_actif: false,
            rappels_pause_actives: true,
            ram_available_mb: 0,
            disk_free_gb: 0.0,
            os_type: "unknown".to_string(),
            specs_upgraded_since_last: false,
            user_responses: HashMap::new(),
            relation_level: 0,
        }
    }
}

impl BotContext {
    /// Détermine la période de la journée (matin, apres_midi, soir, nuit).
    pub fn periode_journee(&self) -> &'static str {
        let hour = self.session_start.map(|dt| dt.hour()).unwrap_or(12);
        match hour {
            6..=11 => "matin",
            12..=17 => "apres_midi",
            18..=22 => "soir",
            _ => "nuit",
        }
    }

    /// Formate la durée de session pour l'affichage.
    pub fn format_duree(&self) -> String {
        let minutes = self.session_duration_minutes;
        if minutes < 60 {
            format!("{} min", minutes)
        } else {
            let h = minutes / 60;
            let m = minutes % 60;
            if m == 0 {
                format!("{}h", h)
            } else {
                format!("{}h{:02}", h, m)
            }
        }
    }
}

/// Constructeur de contexte depuis les sources de données disponibles.
#[allow(dead_code)]
pub struct BotContextBuilder {
    context: BotContext,
}

#[allow(dead_code)]
impl BotContextBuilder {
    pub fn new() -> Self {
        Self {
            context: BotContext::default(),
        }
    }

    /// Configure le profil utilisateur.
    pub fn with_profile(mut self, pseudo: String, langue: Option<String>) -> Self {
        self.context.pseudo = if pseudo.is_empty() {
            "habitant".to_string()
        } else {
            pseudo
        };
        self.context.langue = langue.unwrap_or_else(|| "fr".to_string());
        self
    }

    /// Configure la session.
    pub fn with_session(
        mut self,
        start: DateTime<Utc>,
        duration_minutes: u32,
        is_first: bool,
    ) -> Self {
        self.context.session_start = Some(start);
        self.context.session_duration_minutes = duration_minutes;
        self.context.is_first_connection_of_session = is_first;
        self
    }

    /// Configure les données MiyukiniWatch.
    pub fn with_watch_data(
        mut self,
        jours_absence: Option<u32>,
        service_prefere: Option<(String, u32)>,
        service_delaisse: Option<(String, u32)>,
        ami_delaisse: Option<(String, u32)>,
        ami_connecte: Option<String>,
    ) -> Self {
        self.context.jours_depuis_derniere_visite = jours_absence;
        self.context.service_le_plus_utilise = service_prefere;
        self.context.service_delaisse = service_delaisse;
        self.context.ami_plus_delaisse = ami_delaisse;
        self.context.ami_connecte_recemment = ami_connecte;
        self
    }

    /// Configure l'événement prochain (JayKoa).
    pub fn with_next_event(mut self, event: Option<(String, DateTime<Utc>)>) -> Self {
        if let Some((_name, dt)) = &event {
            let now = Utc::now();
            let diff = dt.signed_duration_since(now);
            self.context.evenement_dans_moins_d_une_heure =
                diff.num_hours() < 1 && diff.num_seconds() > 0;
        }
        self.context.evenement_prochain = event;
        self
    }

    /// Configure les préférences Miou.
    pub fn with_preferences(
        mut self,
        seuil_pause: u32,
        max_bulles: u32,
        bulles_affichees: u32,
        delai_min: u32,
        bulles_actives: bool,
        dnd: bool,
        rappels_pause: bool,
    ) -> Self {
        self.context.seuil_pause_minutes = seuil_pause;
        self.context.max_bulles_par_session = max_bulles;
        self.context.bulles_deja_affichees = bulles_affichees;
        self.context.delai_min_entre_bulles_secs = delai_min;
        self.context.bulles_actives = bulles_actives;
        self.context.dnd_actif = dnd;
        self.context.rappels_pause_actives = rappels_pause;
        self
    }

    /// Configure les specs machine.
    pub fn with_specs(mut self, ram_mb: u32, disk_gb: f32, os: String, upgraded: bool) -> Self {
        self.context.ram_available_mb = ram_mb;
        self.context.disk_free_gb = disk_gb;
        self.context.os_type = os;
        self.context.specs_upgraded_since_last = upgraded;
        self
    }

    /// Configure les réponses utilisateur.
    pub fn with_user_responses(mut self, responses: HashMap<String, String>) -> Self {
        self.context.user_responses = responses;
        self
    }

    /// Configure le niveau de relation.
    pub fn with_relation(mut self, level: u8) -> Self {
        self.context.relation_level = level;
        self
    }

    /// Construit le BotContext final.
    pub fn build(self) -> BotContext {
        self.context
    }
}

impl Default for BotContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Récupère les informations système (RAM, disque, OS).
#[allow(dead_code)]
pub fn get_system_specs() -> (u32, f32, String) {
    // Implémentation basique avec sysinfo si disponible
    // Pour l'instant, valeurs par défaut
    let os = std::env::consts::OS.to_string();
    // Stub : RAM et disque à 0 (à implémenter avec sysinfo)
    (0, 0.0, os)
}
