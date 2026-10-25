//! Construction du **bloc de contexte mémoire** injecté dans le system prompt
//! du LLM (Hermes-style « persistent context »).
//!
//! L'idée : avant chaque inference, on récupère depuis le `MemoryStore` le
//! profil complet de la persona active (description, faits, préférences,
//! récents épisodes) et on assemble un bloc Markdown que le LLM lit comme
//! des instructions persistantes.
//!
//! Cela donne à Alicia une **continuité** entre conversations : elle se
//! souvient de qui parle, de ses goûts, de ce qu'elle a fait récemment.
//!
//! Format produit (exemple) :
//!
//! ```text
//! # Contexte Alicia — Persona « Voix »
//!
//! Persona vocal par défaut, accès domestique.
//!
//! ## Faits sur l'utilisateur
//! - Jean préfère prendre le métro
//! - Le chat de la maison s'appelle Mochi
//!
//! ## Préférences
//! - notify = "desktop"
//! - lang = "fr"
//!
//! ## Interactions récentes
//! - 2026-05-08 09:12 — jaykoa.schedule_event (ok) « ajoute une réunion lundi »
//! - 2026-05-07 19:45 — jaykonta.add_expense (ok) « note 12€ café »
//! ```
//!
//! Ce module ne décide pas QUAND injecter — c'est le runtime Alicia (P2.3+)
//! qui appellera `build_persona_context` avant chaque tour LLM.

// @id service.alicia.memory.prompt
// @role system_prompt_context_builder
// @layer 7
// @do build_persona_system_context
// @human Sérialise le contexte mémoire d'une persona en Markdown injectable.

use std::fmt::Write;

use crate::errors::MemoryError;
use crate::store::MemoryStore;
use crate::types::EpisodeOutcome;

/// Options de génération du bloc contexte.
#[derive(Debug, Clone)]
pub struct ContextOptions {
    /// Nombre maximum de faits à inclure (les plus récents en premier).
    pub max_facts: usize,
    /// Nombre maximum de préférences à inclure.
    pub max_preferences: usize,
    /// Nombre maximum d'épisodes récents à inclure (toutes personas confondues).
    pub max_recent_episodes: usize,
    /// Inclure les préférences `global` en plus des préférences scope=persona.
    pub include_global_preferences: bool,
    /// Budget du bloc en jetons (EX-50), mesuré par la borne supérieure garantie
    /// « octets UTF-8 + 1 » (tokenizers BPE / SentencePiece à repli octet) : les
    /// dernières lignes (épisodes, puis préférences, puis faits) sont retirées
    /// jusqu'à tenir ; l'en-tête reste toujours.
    pub max_tokens: usize,
}

/// Jetons d'un texte, borne supérieure garantie (octets UTF-8 + 1).
pub fn borne_jetons(texte: &str) -> usize {
    texte.len() + 1
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            max_facts: 12,
            max_preferences: 12,
            max_recent_episodes: 5,
            include_global_preferences: true,
            // Profil « petit modèle » de la mémoire MAIA (§7) : 600 jetons.
            max_tokens: 600,
        }
    }
}

/// Assemble le bloc Markdown de contexte pour la persona donnée.
///
/// Si la persona n'existe pas, le bloc reste assemblé avec ce qui est trouvable
/// (faits + épisodes scope persona, préférences globales si activé). On ne fait
/// pas erreurer — un agent doit pouvoir tourner même sans persona déclarée.
pub fn build_persona_context(
    persona_id: &str,
    store: &MemoryStore,
    opts: &ContextOptions,
) -> Result<String, MemoryError> {
    let mut out = String::new();

    let persona = store.persona(persona_id)?;
    let header_name = persona
        .as_ref()
        .map(|p| p.display_name.clone())
        .unwrap_or_else(|| persona_id.to_string());

    let _ = writeln!(out, "# Contexte Alicia — Persona « {header_name} »");

    if let Some(p) = &persona {
        if !p.description.trim().is_empty() {
            let _ = writeln!(out);
            let _ = writeln!(out, "{}", p.description.trim());
        }
    }

    // ---------- Faits ----------
    let mut facts = store.facts_for_persona(persona_id)?;
    facts.truncate(opts.max_facts);
    if !facts.is_empty() {
        let _ = writeln!(out, "\n## Faits");
        for f in &facts {
            let _ = writeln!(out, "- {}", f.content.trim());
        }
    }

    // ---------- Préférences ----------
    let mut prefs = store.preferences_for_scope(persona_id)?;
    if opts.include_global_preferences {
        let mut globals = store.preferences_for_scope("global")?;
        prefs.append(&mut globals);
    }
    prefs.truncate(opts.max_preferences);
    if !prefs.is_empty() {
        let _ = writeln!(out, "\n## Préférences");
        for p in &prefs {
            let _ = writeln!(out, "- {} = {}", p.key, format_pref_value(&p.value));
        }
    }

    // ---------- Épisodes récents ----------
    let episodes = store.recent_episodes(opts.max_recent_episodes)?;
    if !episodes.is_empty() {
        let _ = writeln!(out, "\n## Interactions récentes");
        for ep in &episodes {
            let outcome = match ep.outcome {
                EpisodeOutcome::Ok => "ok",
                EpisodeOutcome::Cancelled => "annulé",
                EpisodeOutcome::Failed => "échec",
            };
            let _ = writeln!(
                out,
                "- {} — {}.{} ({}) « {} »",
                ep.created_at.format("%Y-%m-%d %H:%M"),
                ep.service_id,
                ep.intent,
                outcome,
                ep.user_phrase.trim()
            );
        }
    }

    Ok(tronquer_au_budget(out, opts.max_tokens))
}

/// Retire les dernières lignes jusqu'à tenir dans `max_tokens` (en-tête conservé).
fn tronquer_au_budget(out: String, max_tokens: usize) -> String {
    if borne_jetons(&out) <= max_tokens {
        return out;
    }
    let mut lignes: Vec<&str> = out.lines().collect();
    while lignes.len() > 1 && borne_jetons(&lignes.join("\n")) > max_tokens {
        lignes.pop();
        // Pas de titre de section orphelin.
        while lignes.len() > 1 && lignes.last().is_some_and(|l| l.is_empty() || l.starts_with("## ")) {
            lignes.pop();
        }
    }
    let mut t = lignes.join("\n");
    t.push('\n');
    t
}

/// Formate une valeur de préférence pour affichage Markdown — string sans
/// guillemets externes redondants, autres types via `to_string()` du JSON.
fn format_pref_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => format!("\"{s}\""),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Episode, EpisodeOutcome, Fact, Persona, Preference};
    use serde_json::json;

    fn store() -> MemoryStore {
        MemoryStore::open_in_memory().unwrap()
    }

    #[test]
    fn build_context_with_full_persona_assembles_all_sections() {
        let s = store();
        s.upsert_persona(&Persona::new(
            "voice".into(),
            "Voix".into(),
            "Persona domestique principal.".into(),
            true,
        ))
        .unwrap();
        s.remember(&Fact::new(
            "voice".into(),
            "user".into(),
            "Jean préfère le métro".into(),
            "user".into(),
        ))
        .unwrap();
        s.prefer(&Preference::new("voice".into(), "notify".into(), json!("desktop")))
            .unwrap();
        s.prefer(&Preference::new("global".into(), "lang".into(), json!("fr")))
            .unwrap();
        s.record_episode(&Episode::new(
            "voice".into(),
            "jaykoa".into(),
            "schedule_event".into(),
            "ajoute une réunion lundi".into(),
            json!({"title": "Sync"}),
            EpisodeOutcome::Ok,
            None,
        ))
        .unwrap();

        let ctx = build_persona_context("voice", &s, &ContextOptions::default()).unwrap();
        assert!(ctx.contains("Persona « Voix »"));
        assert!(ctx.contains("Persona domestique principal."));
        assert!(ctx.contains("## Faits"));
        assert!(ctx.contains("Jean préfère le métro"));
        assert!(ctx.contains("## Préférences"));
        assert!(ctx.contains("notify = \"desktop\""));
        assert!(ctx.contains("lang = \"fr\""));
        assert!(ctx.contains("## Interactions récentes"));
        assert!(ctx.contains("jaykoa.schedule_event (ok)"));
    }

    #[test]
    fn build_context_for_missing_persona_falls_back_to_id_in_header() {
        let s = store();
        let ctx = build_persona_context("ghost", &s, &ContextOptions::default()).unwrap();
        assert!(ctx.contains("Persona « ghost »"));
        // Pas de section faits / préférences / épisodes
        assert!(!ctx.contains("## Faits"));
        assert!(!ctx.contains("## Préférences"));
        assert!(!ctx.contains("## Interactions récentes"));
    }

    #[test]
    fn build_context_respects_max_facts() {
        let s = store();
        s.upsert_persona(&Persona::new(
            "voice".into(),
            "Voix".into(),
            "".into(),
            true,
        ))
        .unwrap();
        for i in 0..10 {
            s.remember(&Fact::new(
                "voice".into(),
                "user".into(),
                format!("Fait {i}"),
                "user".into(),
            ))
            .unwrap();
        }
        let opts = ContextOptions {
            max_facts: 3,
            ..ContextOptions::default()
        };
        let ctx = build_persona_context("voice", &s, &opts).unwrap();
        let count = ctx.matches("- Fait ").count();
        assert_eq!(count, 3);
    }

    #[test]
    fn build_context_can_skip_global_preferences() {
        let s = store();
        s.upsert_persona(&Persona::new(
            "voice".into(),
            "Voix".into(),
            "".into(),
            true,
        ))
        .unwrap();
        s.prefer(&Preference::new("voice".into(), "a".into(), json!(1)))
            .unwrap();
        s.prefer(&Preference::new("global".into(), "b".into(), json!(2)))
            .unwrap();
        let opts = ContextOptions {
            include_global_preferences: false,
            ..ContextOptions::default()
        };
        let ctx = build_persona_context("voice", &s, &opts).unwrap();
        assert!(ctx.contains("a = 1"));
        assert!(!ctx.contains("b = 2"));
    }

    #[test]
    fn build_context_respecte_le_budget_de_jetons() {
        let s = store();
        s.upsert_persona(&Persona::new("voice".into(), "Voix".into(), String::new(), true))
            .unwrap();
        for i in 0..10 {
            s.remember(&Fact::new(
                "voice".into(),
                "user".into(),
                format!("Fait numéro {i} assez long pour peser dans le budget"),
                "user".into(),
            ))
            .unwrap();
        }
        let opts = ContextOptions {
            max_tokens: 200,
            ..ContextOptions::default()
        };
        let ctx = build_persona_context("voice", &s, &opts).unwrap();
        assert!(borne_jetons(&ctx) <= 200, "{}", borne_jetons(&ctx));
        assert!(ctx.starts_with("# Contexte Alicia"));
        assert!(ctx.contains("- Fait numéro"));
        assert!(ctx.matches("- Fait numéro").count() < 10);
        let large = build_persona_context("voice", &s, &ContextOptions::default()).unwrap();
        assert_eq!(large.matches("- Fait numéro").count(), 10);
    }

    #[test]
    fn format_pref_value_quotes_strings_only() {
        assert_eq!(format_pref_value(&json!("desktop")), "\"desktop\"");
        assert_eq!(format_pref_value(&json!(42)), "42");
        assert_eq!(format_pref_value(&json!(true)), "true");
        assert_eq!(format_pref_value(&json!({"k": 1})), r#"{"k":1}"#);
    }

    #[test]
    fn build_context_omits_empty_sections() {
        let s = store();
        s.upsert_persona(&Persona::new(
            "voice".into(),
            "Voix".into(),
            "Description.".into(),
            true,
        ))
        .unwrap();
        let ctx = build_persona_context("voice", &s, &ContextOptions::default()).unwrap();
        assert!(ctx.contains("Description."));
        assert!(!ctx.contains("## Faits"));
        assert!(!ctx.contains("## Préférences"));
        assert!(!ctx.contains("## Interactions récentes"));
    }
}
