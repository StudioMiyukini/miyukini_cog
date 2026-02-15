//! État et préférences Miou.

use std::collections::{VecDeque, HashSet};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::bubble::BulleOutput;

/// Fréquence des bulles Miou.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FrequenceBulles {
    /// Discret : max 2 bulles/session, délai 120s
    Discret,
    /// Normal : max 5 bulles/session, délai 30s
    #[default]
    Normal,
    /// Bavard : max 10 bulles/session, délai 15s
    Bavard,
}

impl FrequenceBulles {
    /// Nombre maximum de bulles par session.
    pub fn max_bulles(&self) -> u32 {
        match self {
            Self::Discret => 2,
            Self::Normal => 5,
            Self::Bavard => 10,
        }
    }

    /// Délai minimum entre deux bulles (secondes).
    #[allow(dead_code)]
    pub fn delai_min_secs(&self) -> u32 {
        match self {
            Self::Discret => 120,
            Self::Normal => 30,
            Self::Bavard => 15,
        }
    }
}

/// Préférences Miou (persistées dans le profil).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiouPreferences {
    /// Bulles activées (master switch).
    pub bulles_actives: bool,
    /// Mode Ne Pas Déranger (aucune bulle sauf exceptions critiques).
    pub dnd_actif: bool,
    /// Fréquence des bulles.
    pub frequence: FrequenceBulles,
    /// Seuil de pause santé (minutes). 0 = désactivé.
    pub seuil_pause_minutes: u32,
    /// Rappels de pause activés.
    pub rappels_pause_actives: bool,
    /// Voix Miou activée (master : MP3 + TTS).
    pub voix_enabled: bool,
    /// TTS eSpeak activé (synthèse vocale pour textes dynamiques).
    pub tts_enabled: bool,
    /// Voix sur le Salon activée (accueil vocal).
    pub voix_salon_enabled: bool,
    /// Son des bulles activé (petit « pop »).
    pub son_bulles_enabled: bool,
    /// Mode LLM activé (non implémenté en 0.1.x, persisté pour compat future).
    pub llm_enabled: bool,
}

impl Default for MiouPreferences {
    fn default() -> Self {
        Self {
            bulles_actives: true,
            dnd_actif: false,
            frequence: FrequenceBulles::Normal,
            seuil_pause_minutes: 120,
            rappels_pause_actives: true,
            voix_enabled: true,
            tts_enabled: false,
            voix_salon_enabled: false,
            son_bulles_enabled: false,
            llm_enabled: false,
        }
    }
}

/// État Miou de la session en cours (non persisté).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MiouState {
    /// Bulle actuellement affichée.
    pub current_bulle: Option<BulleOutput>,
    /// File d'attente des bulles à afficher.
    pub queue: VecDeque<BulleOutput>,
    /// Timestamp de la dernière bulle affichée.
    pub last_shown_at: Option<DateTime<Utc>>,
    /// Nombre de bulles affichées cette session.
    pub bulles_count_this_session: u32,
    /// ID de session (pour historique anti-répétition).
    pub session_id: String,
    /// Variantes utilisées cette session (catégorie:variante_id).
    pub variantes_used: HashSet<String>,
    /// Catégories déjà affichées cette session (pour exclusions).
    pub categories_shown: HashSet<String>,
    /// Timestamp de début de session.
    pub session_start: DateTime<Utc>,
    /// Pause déjà affichée et dismissée (timestamp du dismiss).
    pub pause_dismissed_at: Option<DateTime<Utc>>,
    /// Événements déjà rappelés cette session (event_id).
    pub events_reminded: HashSet<String>,
}

impl Default for MiouState {
    fn default() -> Self {
        Self {
            current_bulle: None,
            queue: VecDeque::new(),
            last_shown_at: None,
            bulles_count_this_session: 0,
            session_id: uuid::Uuid::new_v4().to_string(),
            variantes_used: HashSet::new(),
            categories_shown: HashSet::new(),
            session_start: Utc::now(),
            pause_dismissed_at: None,
            events_reminded: HashSet::new(),
        }
    }
}

impl MiouState {
    /// Crée un nouvel état pour une nouvelle session.
    #[allow(dead_code)]
    pub fn new_session() -> Self {
        Self::default()
    }

    /// Enregistre une variante comme utilisée.
    pub fn record_variante(&mut self, categorie: &str, variante_id: &str) {
        self.variantes_used.insert(format!("{}:{}", categorie, variante_id));
        self.categories_shown.insert(categorie.to_string());
    }

    /// Vérifie si une variante a été utilisée.
    #[allow(dead_code)]
    pub fn is_variante_used(&self, categorie: &str, variante_id: &str) -> bool {
        self.variantes_used.contains(&format!("{}:{}", categorie, variante_id))
    }

    /// Durée de la session en minutes.
    #[allow(dead_code)]
    pub fn session_duration_minutes(&self) -> u32 {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.session_start);
        duration.num_minutes().max(0) as u32
    }

    /// Vérifie si le délai minimum entre deux bulles est respecté.
    #[allow(dead_code)]
    pub fn can_show_bulle(&self, prefs: &MiouPreferences) -> bool {
        if !prefs.bulles_actives || prefs.dnd_actif {
            return false;
        }
        if self.bulles_count_this_session >= prefs.frequence.max_bulles() {
            return false;
        }
        if let Some(last) = self.last_shown_at {
            let elapsed = Utc::now().signed_duration_since(last).num_seconds();
            if elapsed < prefs.frequence.delai_min_secs() as i64 {
                return false;
            }
        }
        true
    }

    /// Enregistre l'affichage d'une bulle.
    pub fn record_bulle_shown(&mut self, categorie: &str, variante_id: &str) {
        self.last_shown_at = Some(Utc::now());
        self.bulles_count_this_session += 1;
        self.record_variante(categorie, variante_id);
    }

    /// Enregistre le dismiss d'une bulle pause.
    pub fn record_pause_dismissed(&mut self) {
        self.pause_dismissed_at = Some(Utc::now());
    }

    /// Vérifie si la pause a été dismissée récemment (< 2h).
    pub fn is_pause_recently_dismissed(&self) -> bool {
        if let Some(dismissed) = self.pause_dismissed_at {
            let elapsed = Utc::now().signed_duration_since(dismissed).num_hours();
            elapsed < 2
        } else {
            false
        }
    }

    /// Enregistre un événement comme rappelé.
    #[allow(dead_code)]
    pub fn record_event_reminded(&mut self, event_id: &str) {
        self.events_reminded.insert(event_id.to_string());
    }

    /// Vérifie si un événement a déjà été rappelé.
    pub fn is_event_reminded(&self, event_id: &str) -> bool {
        self.events_reminded.contains(event_id)
    }
}
