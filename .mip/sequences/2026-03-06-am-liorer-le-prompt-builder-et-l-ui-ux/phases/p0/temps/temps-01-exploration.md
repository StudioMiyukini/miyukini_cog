# P0 Temps 1 - Exploration et brainstorming

## Statut

- Etat : TERMINE
- Phase : P0 Temps 1
- Responsable principal : Maria

## TL;DR

Prompt Builder T4/C4. Le builder actuel est trop lent, trop pauvre en options et trop rudimentaire visuellement. L'utilisateur veut les trois a la fois : vitesse (listes predefinies), puissance (multiselect agents, mode autonomie, tags, MSW toggle, donnees sensibles) et ergonomie (preview live, sections avancees retractables).

## Section 0 — Orientation (deduite depuis le premier prompt)

> Maria remplit ce tableau SEULE, avant de poser des questions a l'utilisateur.

| Question | Reponse deduite | Confiance |
|----------|----------------|-----------|
| Pourquoi exactement ? | Le Prompt Builder actuel est trop basique (6 champs texte/select simples). Il manque de fonctionnalites avancees pour configurer finement les sequences MIP : pas de multiselect, pas de listes predefinies, pas d'options de style, pas de tags. | haute |
| Exemple concret d'usage attendu ? | Choisir la stack via liste deroulante predefinies (Rust+axum, Dioxus, etc.), activer des agents par checkboxes, ajouter des tags par multiselect, previsualiser le prompt en live | haute |
| Solution existante proche dans le projet ? | `apps/mipower/static/index.html` + `app.js` — vue "Prompt Builder" avec `<form id="promptForm">` (6 champs : title, task_class, domain, desc, constraints, stack) | haute |
| Pour qui ? | Miyukini (l'utilisateur principal, developpeur/architecte) utilisant MIPOWER pour generer des prompts MIP et lancer des sequences | haute |
| Fonction Online / MWS requise ? | Non — application locale Rust/axum, frontend HTML/JS vanilla | haute |
| Open-source / forkable ou from scratch ? | Amelioration du code existant dans `apps/mipower/` (from-scratch pour les nouveaux composants UI) | haute |
| Classification estimee (T1-T5) | T4 — Feature majeure (confirmee par l'utilisateur : touche HTML, CSS, JS, backend Rust, modeles, tests) | haute |

## Etat actuel du Prompt Builder (Exploration codebase)

### Frontend (`apps/mipower/static/index.html` + `app.js`)
- Champs actuels : titre (text), classe (select T3/T4/T5), domaine (select 5 options), description (textarea), contraintes (text optionnel), stack (text optionnel)
- Apres generation : copier, choisir complexite C1-C5, bouton "Init sequence"
- Manques identifies :
  - T1/T2 absents du select "Classe de tache"
  - Stack en saisie libre (pas de liste predifinie ni d'autocompletion)
  - Aucun champ tags/mots-cles
  - Aucun champ mode d'autonomie (FULL/BIG_STEPS/GUIDED)
  - Complexite cachee jusqu'apres generation (UX confuse)
  - Pas de preview live du prompt
  - Pas d'options avancees (lois d'autonomie, agents actives, etc.)
  - `tags: []` envoye au backend mais ignores cote serveur ET frontend

### Backend (`apps/mipower/src/api.rs` + `models.rs`)
- `PromptBuilderInput` : title, task_class, domain, description, constraints?, stack?, tags[]
- `prompt_handler` : genere template texte fixe — tags completement ignores
- Pas de champ : autonomy_mode, agents, urgency, keywords, complexity_hint

## Brainstorming (reponses utilisateur)

### Section 1 — COMPRENDRE
- 1.1-1.4 : **Les trois a la fois** — Vitesse (saisie repetitive trop lente), puissance (pas assez de contr\xf4le sur le contenu du prompt), ergonomie (UI trop rudimentaire).
- Flux actuel : ouvrir MIPOWER > onglet Prompt Builder > remplir 6 champs manuellement > clic Generer > copier > coller dans le LLM. Frictions : stack en texte libre, classe T1/T2 absente, tags ignores, complexite cachee, pas de preview.

### Section 2 — CADRER
- Perimetre : **Complet + sections avancees**
  - INCLUS : menus deroulants predifinis stack, multiselect tags/agents, mode autonomie, T1/T2 dans la classe, complexite visible des le debut, preview live, sauvegarde configs favorites, section Options avancees retractable (lois d'autonomie, agents fine-tuned, urgency flag, keywords), MSW toggle [?], toggle donnees sensibles
  - EXCLUS : editeur de template custom, gestion multi-profils multi-utilisateurs
- Priorite : prompt complet + valide + copiable en 1 clic (LA chose qui DOIT fonctionner)

### Section 3 — IMAGINER
Toutes les options choisies :
- Listes predefinies stack (select + Autre libre)
- Multiselect agents actifs (checkboxes/tags : Maria, Denis, Lise, Victor, Hugo, Fabrice, George, Jean, Arianne, Francois)
- Preview live du prompt (mise a jour en temps reel sans clic Generer)
- Mode autonomie + tags + urgence (FULL/BIG_STEPS/GUIDED + multiselect tags + flag urgence)
- Options generales de secteur (domaine etendu : back, front, fullstack, infra, AI/ML, securite, data, autre)
- MSW toggle [?] — a clarifier en T6 spec
- Toggle donnees sensibles [?] — marque la sequence comme sensible (impacte Victor en P0 T5)

### Section 4 — EVALUER
- Benefice principal : prompt MIP complet et valide, copiable en 1 clic, avec toutes les options configurees en quelques secondes (vs. 2min de saisie manuelle actuelle)
- Risques : debounce pour le preview live, state management JS (grille d'agents), CSS layout bi-panneaux a refaire
- Complexite : **C4 — Petit service enrichi** (confirme par l'utilisateur)

## Hypotheses retenues

1. Preview live via debounce 300ms cote JS — pas d'appel API supplementaire (generation locale ou endpoint leger)
2. Agents : liste fixe des 10 agents MIP connus, cases a cocher, envoyes dans le prompt
3. Stack : liste predifinie + champ "Autre" libre
4. Mode autonomie inclus dans le prompt genere (section separee)
5. Section "Options avancees" retractable (accordion HTML natif ou toggle CSS)
6. Complexite C4 → artefacts agents/ generes (index + manifest)

## Hypotheses ecartees

1. Editeur de template Jinja/Mustache — trop complexe, hors scope
2. Sauvegarde configs cote serveur (DB) — trop lourd pour C4 ; on utilisera localStorage
3. Multi-profils utilisateurs — hors scope

## Classification post-T1

- Classe tache (T1-T5) : **T4** — feature majeure (HTML, CSS, JS, Rust backend+models, tests)
- Complexite sequence estimee (C1-C5) : **C4** — confirme par l'utilisateur (sera valide par Denis en T8)
