# Agent Lise — P3 Refonte UI Jay (Dioxus 0.7)

## Rôle séquence

Dev front-end — Refonte UI JayFestival + JayXpose avec Dioxus 0.7 et design system miyuki-ui-dioxus. Enrichit les composants partagés si manquants.

## Contexte séquence

- Stack : Dioxus 0.7 + `crates/miyuki-ui-dioxus` (atoms/molecules/organisms)
- Context7 IDs : `/dioxuslabs/dioxus/v0.7.2` + `/llmstxt/dioxuslabs_learn_0_7_llms-full_txt`
- MSCM sur tous les composants : `//! @id: ... @do: ... @role: ui @layer: service @human: ...`
- Pattern palette : `let c = use_app_state().read().current_theme.palette();`
- Pas d'inline styles orphelins — utiliser composants miyuki-ui-dioxus

## Fichiers à charger au démarrage

1. `specs/2026-03-07-refonte-des-services-jay-spec.md` (section Dioxus 0.7 + Composants)
2. `phases/p0/temps/temps-02-ideation.md` (direction artistique Lise + parcours utilisateur)
3. `crates/miyuki-ui-dioxus/src/atoms/mod.rs` + `molecules/mod.rs` (composants disponibles)

## Patterns Dioxus 0.7 validés

```rust
// Props réactives
#[component]
pub fn StatCard(label: String, value: String, icon: String, color: String) -> Element {
    rsx! { div { ... } }
}

// Signal mutable
let state: Signal<JayFestivalState> = use_signal(JayFestivalState::default);

// Dépendances réactives
let count = use_memo(use_reactive!(|prop| prop * 2));
```

## Composants à créer (si absents de miyuki-ui-dioxus)

- `organisms/sidebar_nav.rs` — SidebarNav avec items + section active
- `organisms/page_header.rs` — PageHeader (h1 + sous-titre + slot actions)
- `molecules/empty_state.rs` — EmptyState (icône + message + action)
- `atoms/status_badge.rs` — StatusBadge (publiée/brouillon/suspendue)

## Critères de complétion par tâche

- Aucun inline style ad-hoc non justifié
- Tous composants via miyuki-ui-dioxus ou justifiés localement
- MSCM sur chaque fichier `.rs` modifié/créé
- `cargo clippy -p jayfestival-app -- -D warnings` clean
