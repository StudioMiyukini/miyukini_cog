# Agent fine-tuned — Francois (Dev Back-End) — P3

## Sequence : am-liorer-le-prompt-builder-et-l-ui-ux

## Role
Dev Back-End Rust. Responsable de `models.rs` et `api.rs` dans `apps/mipower/src/`.

## Contexte technique
- Crate : `mipower` (apps/mipower/)
- Stack : Rust/axum 0.8, serde, serde_json
- Aucune nouvelle dependance Cargo

## Taches P3

### Tache 1 : Etendre PromptBuilderInput (models.rs)
Ajouter a la struct :
```rust
pub autonomy_mode:  Option<String>,
pub urgency:        bool,
pub sensitive_data: bool,
pub msw_toggle:     bool,
```
Note : `agents: Vec<String>` et `tags: Vec<String>` existent deja — ne pas dupliquer.

### Tache 2 : Enrichir prompt_handler (api.rs)
- Ajouter les validations : longueur title<=200, desc<=2000, constraints<=500, stack<=200
- Ajouter whitelist agents : `const VALID_AGENTS: &[&str] = &["Maria","Denis","Lise","Victor","Hugo","Fabrice","George","Jean","Arianne","Francois"];`
- Valider `task_class` ∈ [T1..T5], `domain` ∈ liste spec, `autonomy_mode` ∈ [FULL, BIG_STEPS, GUIDED] ou None
- Enrichir le template prompt selon spec (lignes optionnelles si valeur non vide/false)

### Tache 3 : Mettre a jour les tests (api.rs)
- `test_generate_prompt_non_empty` : ajouter les nouveaux champs
- `test_generate_prompt_with_agents` : verifier que les agents selectionnes apparaissent dans le prompt
- `test_generate_prompt_with_autonomy_mode` : verifier le mode autonomie dans le prompt

## Anti-patterns
- Ne pas utiliser `unwrap()` — utiliser `?` ou `unwrap_or`
- Ne pas modifier les autres handlers (sequences, artefact, progress, init-sequence, settings)
- Ne pas ajouter de nouvelles dependances Cargo

## Critere de completion
`cargo test -p mipower` : 0 regression + 2 nouveaux tests passes.
