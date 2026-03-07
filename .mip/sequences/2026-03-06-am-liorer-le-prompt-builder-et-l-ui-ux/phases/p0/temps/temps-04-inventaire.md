# P0 Temps 4 - Inventaire prerequis

## Statut

- Etat : TERMINE
- Phase : P0 Temps 4
- Responsable principal : Denis/Hugo/Jean

## TL;DR

5 fichiers modifies (models.rs, api.rs, index.html, app.js, app.css). Aucune nouvelle dependance Cargo. 1 test Rust a mettre a jour, 2 nouveaux cas de test a ajouter. Pas de prerequis infra. Blocage potentiel : template JS miroir du template Rust (a synchroniser manuellement).

## Crates / modules a modifier

| Crate | Fichier(s) cible(s) | Type de modification | Raison |
|-------|-------------------|---------------------|--------|
| mipower | `src/models.rs` | modify | Etendre PromptBuilderInput : autonomy_mode, agents[], urgency, sensitive_data, msw_toggle |
| mipower | `src/api.rs` | modify | prompt_handler : utiliser nouveaux champs, enrichir template prompt |
| mipower | `static/index.html` | modify | Ajouter champs UI : T1/T2, stack select, domaine etendu, complexite, autonomie, agents, tags, urgence, donnees sensibles, MSW toggle |
| mipower | `static/app.js` | modify | Preview live (debounce 300ms, template JS), handler agents checkboxes, localStorage save/restore |
| mipower | `static/app.css` | modify | Layout bi-panneaux, accordion sections avancees, tags multiselect, responsive |

## Nouvelles dependances Cargo

Aucune — la stack actuelle (axum 0.8, serde, serde_json, tokio) suffit. Pas de crate supplementaire.

## Tests existants concernes

| Test | Fichier | Impact attendu |
|------|---------|---------------|
| test_generate_prompt_non_empty | `src/api.rs` | A modifier : ajouter les nouveaux champs au PromptBuilderInput de test |
| test_init_sequence_slug_validation | `src/api.rs` | Compatible — non impacte |
| test_path_traversal_rejected | `src/api.rs` | Compatible — non impacte |
| test_sequences_index_parse | `src/api.rs` | Compatible — non impacte |

Nouveaux tests a ajouter :
- `test_generate_prompt_with_agents` : verifier que les agents selectionnes apparaissent dans le prompt
- `test_generate_prompt_with_autonomy_mode` : verifier que le mode autonomie apparait

## Prerequis infrastructure

Aucun prerequis supplementaire. L'application tourne en local sur le port configure dans `src/main.rs`. localStorage navigateur disponible nativement.

## Blocages potentiels

| Blocage | Probabilite | Mitigation |
|---------|------------|------------|
| Template JS miroir du Rust desynchronise | moyenne | Definir le format prompt comme constante documentee en T6 spec |
| CSS bi-panneaux cassant le layout existant | faible | Utiliser flexbox avec breakpoint 900px, tester avant merge |
| serde Deserialize sur Vec<String> pour agents[] | faible | Deja utilise (tags: Vec<String>) dans models.rs — pattern connu |

