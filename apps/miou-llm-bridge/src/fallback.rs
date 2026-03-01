//! Proto-IA de secours — mode hors-ligne / isolement.
//!
//! Quand aucun LLM n'est disponible (ni distant, ni local), ce module
//! fournit un système de réponses bornées avec anti-hallucination strict.
//!
//! Principe : mieux vaut refuser de répondre que d'halluciner.

use serde::{Deserialize, Serialize};

// ── Types ────────────────────────────────────────────────────────────

/// État de disponibilité des LLM.
///
/// Chaîne de dégradation : NativeOnline → UpstreamOnline → Fallback → Offline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmAvailability {
    /// Inférence GGUF native opérationnelle (moteur principal).
    NativeOnline,
    /// Upstream (LM Studio / Ollama) opérationnel (backup).
    UpstreamOnline,
    /// Aucun LLM disponible — proto-IA de secours active.
    Fallback,
    /// Tout est down, même le fallback (état critique).
    Offline,
}

impl LlmAvailability {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NativeOnline => "Natif (GGUF local — full power)",
            Self::UpstreamOnline => "Upstream (LM Studio — backup)",
            Self::Fallback => "Secours (réponses bornées)",
            Self::Offline => "Hors-ligne (critique)",
        }
    }

    pub fn is_degraded(&self) -> bool {
        matches!(self, Self::Fallback | Self::Offline)
    }

    /// Vérifie si un LLM est effectivement disponible (natif ou upstream).
    pub fn has_llm(&self) -> bool {
        matches!(self, Self::NativeOnline | Self::UpstreamOnline)
    }
}

/// Réponse du proto-IA de secours.
#[derive(Debug, Clone, Serialize)]
pub struct FallbackResponse {
    /// Texte de réponse.
    pub content: String,
    /// Le proto-IA a-t-il pu répondre ?
    pub answered: bool,
    /// Raison si refus.
    pub refusal_reason: Option<String>,
    /// Niveau de confiance (0.0 = aucune, 1.0 = sûr).
    pub confidence: f32,
    /// Mode actif.
    pub availability: LlmAvailability,
}

// ── Catégories de questions reconnues ────────────────────────────────

/// Le proto-IA ne répond qu'à des catégories très limitées et bornées.
#[derive(Debug, Clone, Copy)]
enum QuestionCategory {
    /// Salutation / politesse.
    Greeting,
    /// Question sur le statut du service.
    StatusQuery,
    /// Demande d'aide / navigation COG.
    HelpRequest,
    /// Question hors périmètre — REFUS.
    OutOfScope,
}

/// Classifie une question (heuristique simple, pas de ML).
fn classify_question(input: &str) -> QuestionCategory {
    let lower = input.to_lowercase();

    // Salutations
    let greetings = ["bonjour", "salut", "hello", "bonsoir", "hey", "coucou", "yo"];
    if greetings.iter().any(|g| lower.starts_with(g) || lower == *g) {
        return QuestionCategory::Greeting;
    }

    // Status
    let status_kw = ["status", "état", "etat", "connecté", "modèle", "llm", "disponible"];
    if status_kw.iter().any(|k| lower.contains(k)) {
        return QuestionCategory::StatusQuery;
    }

    // Aide / navigation
    let help_kw = ["aide", "help", "comment", "service", "agent", "équipe", "miou"];
    if help_kw.iter().any(|k| lower.contains(k)) {
        return QuestionCategory::HelpRequest;
    }

    QuestionCategory::OutOfScope
}

// ── Proto-IA ─────────────────────────────────────────────────────────

/// Génère une réponse de secours pour un message utilisateur.
/// Anti-hallucination : refuse si la question est hors périmètre.
pub fn fallback_respond(user_message: &str) -> FallbackResponse {
    let category = classify_question(user_message);

    match category {
        QuestionCategory::Greeting => FallbackResponse {
            content: "Bonjour ! Je suis Miou en mode secours. \
                     Les modèles IA ne sont pas disponibles actuellement. \
                     Je ne peux répondre qu'à des questions basiques sur le service. \
                     Dès qu'un modèle sera de nouveau accessible, je retrouverai \
                     toutes mes capacités ! 🐱"
                .into(),
            answered: true,
            refusal_reason: None,
            confidence: 1.0,
            availability: LlmAvailability::Fallback,
        },

        QuestionCategory::StatusQuery => FallbackResponse {
            content: "⚠️ Mode secours actif.\n\n\
                     • Inférence native : ❌ Aucun modèle GGUF chargé\n\
                     • Upstream (backup) : ❌ Injoignable\n\
                     • Proto-IA : ✅ Active (capacités très limitées)\n\n\
                     Pour retrouver les capacités complètes :\n\
                     1. Placez un fichier .gguf dans le dossier models/\n\
                     2. Ou lancez LM Studio / Ollama en backup\n\
                     3. Alicia vous guidera dès que possible"
                .into(),
            answered: true,
            refusal_reason: None,
            confidence: 1.0,
            availability: LlmAvailability::Fallback,
        },

        QuestionCategory::HelpRequest => FallbackResponse {
            content: "📋 Miyukini AI Studio — Mode secours\n\n\
                     Services disponibles même sans IA :\n\
                     • Navigation dans les services COG\n\
                     • Status du système\n\
                     • Liste des agents disponibles\n\n\
                     Agents de l'équipe (actifs dès qu'un LLM est connecté) :\n\
                     🐱 Miou — Guide COG\n\
                     👩‍💼 Alicia — Assistante personnelle\n\
                     📋 Maria — Chef de projet\n\
                     🔍 Fabrice — Analyste PR\n\
                     👨‍💻 Denis — Chef Dev Senior\n\
                     ⚙️ François — Dev Back-End\n\
                     🎨 Lise — Dev Front-End\n\
                     🧠 Arianne — Team Manager\n\
                     📢 Sofia — Marketing\n\
                     🔎 George — Audit Expert\n\
                     📚 Julie — Formatrice\n\
                     🧮 Hugo — Comptable\n\
                     💬 Clara — Community Manager\n\
                     🤝 Victor — Commercial B2B\n\
                     🛒 Emma — Commerciale B2C\n\
                     ⚖️ Louis — Conseil Juridique\n\
                     🌿 Camille — Coach Bien-être"
                .into(),
            answered: true,
            refusal_reason: None,
            confidence: 0.9,
            availability: LlmAvailability::Fallback,
        },

        QuestionCategory::OutOfScope => FallbackResponse {
            content: String::new(),
            answered: false,
            refusal_reason: Some(
                "Je suis en mode secours (aucun modèle IA disponible). \
                 Je ne peux pas répondre à cette question car je risquerais \
                 de donner une réponse incorrecte ou trompeuse. \
                 Dès qu'un modèle sera chargé, je pourrai vous aider. \
                 En attendant, vérifiez que LM Studio est lancé ou qu'un modèle \
                 local est disponible."
                    .into(),
            ),
            confidence: 0.0,
            availability: LlmAvailability::Fallback,
        },
    }
}

/// Vérifie la disponibilité des LLM.
///
/// Chaîne : NativeOnline (GGUF chargé) → UpstreamOnline (LM Studio / Ollama) → Fallback.
pub async fn probe_availability(
    client: &reqwest::Client,
    upstream_url: &str,
    native_loaded: bool,
) -> LlmAvailability {
    // 1. Vérifier si un modèle GGUF est chargé nativement (moteur principal)
    if native_loaded {
        return LlmAvailability::NativeOnline;
    }

    // 2. Tester le upstream configuré
    let upstream_probe = format!("{upstream_url}/v1/models");
    let upstream_ok = client
        .get(&upstream_probe)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    if upstream_ok {
        return LlmAvailability::UpstreamOnline;
    }

    // 3. Tester les backends locaux connus (LM Studio, Ollama)
    let local_urls = [
        "http://localhost:1234/v1/models",
        "http://localhost:11434/api/tags", // Ollama
    ];

    for url in &local_urls {
        // Ne pas re-tester si c'est la même URL que l'upstream
        if upstream_url.contains("localhost:1234") && url.contains("localhost:1234") {
            continue;
        }
        if upstream_url.contains("localhost:11434") && url.contains("localhost:11434") {
            continue;
        }

        let ok = client
            .get(*url)
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);

        if ok {
            return LlmAvailability::UpstreamOnline;
        }
    }

    // 4. Aucun LLM disponible → mode secours
    LlmAvailability::Fallback
}
