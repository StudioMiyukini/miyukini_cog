//! Types métier de la mémoire Alicia : `Fact`, `Preference`, `Episode`,
//! `Persona`.
//!
//! Tous les types sont sérialisables (`serde`) pour exposition future via API
//! REST (`miyualicia-api`) et persistance (KindMother, colonnes `TEXT`).

// @id service.alicia.memory.types
// @role memory_types
// @layer 7
// @do declare_memory_types
// @human Modèles de données de la mémoire Alicia.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Un **fait** — énoncé vrai à conserver pour ré-injection dans le prompt
/// du LLM (Hermes-style « persistent context »).
///
/// Exemples :
/// - « L'utilisateur s'appelle Jean »
/// - « Jean préfère prendre le métro »
/// - « Le chat de la maison s'appelle Mochi »
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fact {
    /// UUID stable du fait.
    pub id: Uuid,
    /// Persona à laquelle le fait est rattaché (`work`, `family`, `voice`,
    /// `global`).
    pub persona: String,
    /// Sujet (entité concernée). Ex : `user`, `house`, `cat:mochi`.
    pub subject: String,
    /// Contenu du fait, en langue naturelle (ce qu'on injecte tel quel).
    pub content: String,
    /// Source (qui a appris ce fait). Ex : `user`, `service:jaykoa`,
    /// `inferred`.
    pub source: String,
    /// Confiance (0.0..=1.0). 1.0 = utilisateur l'a déclaré explicitement.
    pub confidence: f32,
    /// Date d'enregistrement (UTC).
    pub created_at: DateTime<Utc>,
    /// Date de dernière mise à jour (UTC).
    pub updated_at: DateTime<Utc>,
}

impl Fact {
    /// Construit un nouveau fait avec `id` et `created_at` générés.
    #[must_use]
    pub fn new(persona: String, subject: String, content: String, source: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            persona,
            subject,
            content,
            source,
            confidence: 1.0,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Une **version** d'un fait (EX-52 : notes versionnées).
///
/// Chaque écriture qui modifie un fait ajoute une version ; les versions
/// antérieures ne sont jamais réécrites. Seul un oubli demandé
/// (`MemoryStore::forget`) efface l'historique, en laissant une pierre tombale.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FactVersion {
    /// Identifiant du fait.
    pub fact_id: Uuid,
    /// Numéro de version (1 = première écriture), strictement croissant.
    pub version: u32,
    /// Persona au moment de cette version.
    pub persona: String,
    /// Sujet au moment de cette version.
    pub subject: String,
    /// Contenu de cette version.
    pub content: String,
    /// Source de cette version.
    pub source: String,
    /// Confiance de cette version.
    pub confidence: f32,
    /// Date d'enregistrement de la version (UTC).
    pub recorded_at: DateTime<Utc>,
}

/// **Pierre tombale** d'un fait oublié à la demande (EX-52, EX-56).
///
/// Ne contient aucun contenu du fait : seulement qui, quand, pourquoi et
/// combien de versions ont été effacées.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FactTombstone {
    /// Identifiant du fait oublié.
    pub fact_id: Uuid,
    /// Date de l'oubli (UTC).
    pub forgotten_at: DateTime<Utc>,
    /// Auteur de la demande d'oubli (ex. `user`, identifiant de persona).
    pub forgotten_by: String,
    /// Motif de l'oubli (texte libre, sans contenu du fait).
    pub reason: String,
    /// Nombre de versions effacées.
    pub versions_erased: u32,
}

/// Une **préférence** — un choix par défaut configurable, avec un scope.
///
/// Exemples :
/// - `(persona=voice, key=notify, value="desktop")`
/// - `(persona=work, key=prefer_train, value=true)`
/// - `(persona=global, key=lang, value="fr")`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Preference {
    /// Scope de la préférence : nom de persona, ou `"global"`.
    pub scope: String,
    /// Clé (slug stable).
    pub key: String,
    /// Valeur arbitraire en JSON.
    pub value: Value,
    /// Date de dernière mise à jour (UTC).
    pub updated_at: DateTime<Utc>,
}

impl Preference {
    /// Construit une préférence avec `updated_at` à maintenant.
    #[must_use]
    pub fn new(scope: String, key: String, value: Value) -> Self {
        Self {
            scope,
            key,
            value,
            updated_at: Utc::now(),
        }
    }
}

/// Issue d'un épisode — résultat de l'exécution d'une intention.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EpisodeOutcome {
    /// L'action a réussi.
    Ok,
    /// L'utilisateur a annulé pendant la confirmation.
    Cancelled,
    /// L'action a échoué (le détail est dans `Episode.error`).
    Failed,
}

/// Un **épisode** — trace d'une interaction utilisateur ↔ Alicia.
///
/// Structure inspirée du `activity_log` actuel d'Alicia (in-memory) mais
/// avec persistance, recherche et corrélation persona/service.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Episode {
    /// UUID stable.
    pub id: Uuid,
    /// Persona active au moment de l'interaction.
    pub persona: String,
    /// Service ciblé (`jaykoa`, `jaykonta`, `unknown`...).
    pub service_id: String,
    /// Intent invoqué (`schedule_event`, `add_expense`...).
    pub intent: String,
    /// Phrase utilisateur d'origine (texte brut, telle qu'écrite/dite).
    pub user_phrase: String,
    /// Paramètres résolus (JSON).
    pub params: Value,
    /// Résultat.
    pub outcome: EpisodeOutcome,
    /// Message d'erreur si `outcome == Failed`.
    pub error: Option<String>,
    /// Date d'enregistrement (UTC).
    pub created_at: DateTime<Utc>,
}

impl Episode {
    /// Construit un nouvel épisode (id + created_at générés).
    #[must_use]
    pub fn new(
        persona: String,
        service_id: String,
        intent: String,
        user_phrase: String,
        params: Value,
        outcome: EpisodeOutcome,
        error: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            persona,
            service_id,
            intent,
            user_phrase,
            params,
            outcome,
            error,
            created_at: Utc::now(),
        }
    }
}

/// Un **persona** — profil contextuel sous lequel Alicia opère.
///
/// Permet de cloisonner faits/préférences/épisodes selon le contexte
/// d'usage (travail / famille / voix-only / public...).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Persona {
    /// Slug stable (`work`, `family`, `voice`...).
    pub id: String,
    /// Nom affiché (« Travail », « Famille »...).
    pub display_name: String,
    /// Description libre (rappelée au LLM).
    pub description: String,
    /// `true` si ce persona est actif par défaut au démarrage.
    pub is_default: bool,
    /// Date de création (UTC).
    pub created_at: DateTime<Utc>,
}

impl Persona {
    /// Construit un nouveau persona.
    #[must_use]
    pub fn new(id: String, display_name: String, description: String, is_default: bool) -> Self {
        Self {
            id,
            display_name,
            description,
            is_default,
            created_at: Utc::now(),
        }
    }
}

/// Statut d'une **skill apprise** dans son cycle de vie de promotion.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LearnedSkillStatus {
    /// Le détecteur de patterns a proposé la skill mais l'utilisateur ne s'est
    /// pas encore prononcé. Visible dans le dialogue de promotion.
    Proposed,
    /// L'utilisateur a accepté — la skill apprise est active et utilisable.
    Accepted,
    /// L'utilisateur a refusé — la skill est conservée comme « no-go » pour
    /// éviter de la re-proposer en boucle.
    Rejected,
}

/// Une **skill apprise** par observation des épisodes de l'utilisateur.
///
/// Hermes-style : quand Alicia détecte qu'un même `qualified_id` est invoqué
/// 3 fois ou plus avec une persona donnée, elle propose à l'utilisateur de
/// le matérialiser comme un raccourci nommé. Les paramètres et la phrase
/// déclencheuse sont remplis à partir de la dernière occurrence observée ;
/// l'utilisateur peut les éditer ou laisser tels quels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LearnedSkill {
    /// UUID stable.
    pub id: Uuid,
    /// Persona à laquelle la skill apprise est rattachée.
    pub persona: String,
    /// Phrase déclencheuse — ce qu'Alicia attend pour activer cette skill.
    /// Initialement remplie depuis le dernier `Episode.user_phrase`.
    pub trigger_phrase: String,
    /// Identifiant qualifié de la capability cible (`service_id.intent`).
    pub qualified_id: String,
    /// Template de paramètres pré-remplis (JSON object — peut être vide).
    pub params_template: Value,
    /// Statut de promotion.
    pub status: LearnedSkillStatus,
    /// Date de création (= proposition).
    pub created_at: DateTime<Utc>,
    /// Date d'acceptation, si applicable.
    pub accepted_at: Option<DateTime<Utc>>,
}

impl LearnedSkill {
    /// Construit une nouvelle proposition (statut `Proposed`).
    #[must_use]
    pub fn proposed(
        persona: String,
        trigger_phrase: String,
        qualified_id: String,
        params_template: Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            persona,
            trigger_phrase,
            qualified_id,
            params_template,
            status: LearnedSkillStatus::Proposed,
            created_at: Utc::now(),
            accepted_at: None,
        }
    }
}

/// Statut d'une `PendingConfirmation` dans le cycle de vie de la queue.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PendingStatus {
    /// En attente de décision utilisateur.
    Pending,
    /// Confirmation réclamée atomiquement, exécution en cours (R-017). Un
    /// enregistrement dans cet état ne peut plus être réclamé : il ne sera
    /// jamais ré-exécuté, même après un arrêt brutal (fail-closed).
    Executing,
    /// L'utilisateur a confirmé — l'exécution a eu lieu.
    Confirmed,
    /// L'exécution a été tentée après confirmation mais a échoué (erreur
    /// dispatcher / gouvernance). Terminal : pas de nouvelle tentative.
    Failed,
    /// L'utilisateur a refusé — pas d'exécution, audit `Cancelled` posé.
    Cancelled,
}

/// Une **confirmation en attente** — instance d'un `Decision::PendingConfirmation`
/// persistée pour qu'une UI / client distant puisse la présenter à
/// l'utilisateur et y répondre, même après un redémarrage du runtime.
///
/// Le runtime Alicia peut auto-enregistrer chaque `Decision::PendingConfirmation`
/// renvoyée par le dispatcher dans cette queue, et exposer une API pour
/// lister les pending et résoudre par `id`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PendingConfirmation {
    /// UUID stable.
    pub id: Uuid,
    /// Identifiant qualifié de la capability (`service_id.intent`).
    pub qualified_id: String,
    /// Paramètres résolus (JSON object ou null) à passer si l'utilisateur
    /// confirme.
    pub params: Value,
    /// Persona active au moment de la demande.
    pub persona: String,
    /// Phrase utilisateur originale (pour audit).
    pub user_phrase: String,
    /// Texte à afficher à l'utilisateur (description de la capability ou
    /// intent).
    pub prompt: String,
    /// Statut courant.
    pub status: PendingStatus,
    /// Date de création (UTC).
    pub created_at: DateTime<Utc>,
    /// Date de résolution (confirmation ou annulation), `None` tant que
    /// `Pending`.
    pub resolved_at: Option<DateTime<Utc>>,
    /// Motif de résolution automatique, ex. `"expired"` quand le balayage
    /// d'expiration a fait passer l'enregistrement en `Failed` (exécution
    /// bloquée) ou `Cancelled` (attente trop longue). `None` sinon.
    #[serde(default)]
    pub reason: Option<String>,
}

/// Motif posé par le balayage d'expiration des confirmations.
pub const PENDING_REASON_EXPIRED: &str = "expired";

/// Bilan d'un balayage d'expiration des `PendingConfirmation`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingExpiryReport {
    /// Enregistrements `Executing` trop anciens passés en `Failed`
    /// (jamais ré-exécutés).
    pub failed: Vec<Uuid>,
    /// Enregistrements `Pending` trop anciens passés en `Cancelled`.
    pub cancelled: Vec<Uuid>,
}

impl PendingExpiryReport {
    /// `true` si aucun enregistrement n'a expiré.
    pub fn is_empty(&self) -> bool {
        self.failed.is_empty() && self.cancelled.is_empty()
    }
}

impl PendingConfirmation {
    /// Construit une nouvelle confirmation en attente (statut `Pending`).
    #[must_use]
    pub fn new(
        qualified_id: String,
        params: Value,
        persona: String,
        user_phrase: String,
        prompt: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            qualified_id,
            params,
            persona,
            user_phrase,
            prompt,
            status: PendingStatus::Pending,
            created_at: Utc::now(),
            resolved_at: None,
            reason: None,
        }
    }
}
