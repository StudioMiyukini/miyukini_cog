# P0 Temps 2 - Ideation

## Statut

- Etat : TERMINE
- Phase : P0 Temps 2
- Responsable principal : Maria/Lise

## TL;DR

Le Prompt Builder v2 est decoupage en 4 blocs : (A) Champs de base enrichis, (B) Section Options avancees, (C) Preview live, (D) Backend enrichi. Solution retenue : layout bi-panneaux avec preview live a droite, sections retractables, sauvegarde localStorage. Complexite C4 confirmee.

## Perimetre

### IN (inclus dans la sequence)

- Classe de tache etendue : T1, T2, T3, T4, T5 (T1/T2 manquants actuellement)
- Stack : select predifini (10+ options courantes) + champ "Autre" libre
- Domaine etendu : back, front, fullstack, infra, AI/ML, securite, data, autre
- Complexite : visible des le debut du formulaire (pas apres generation)
- Mode autonomie : select FULL / BIG_STEPS / GUIDED / (non specifie)
- Tags : multiselect + saisie libre (add tag)
- Agents actifs : multiselect checkboxes (10 agents MIP)
- Urgence : toggle boolean (flag "urgent" dans le prompt)
- Donnees sensibles : toggle boolean (flag pour Victor en P0 T5)
- MSW toggle : a specifier en T6 (hypothese : toggle "Mode Sans Web" pour indiquer absence connectivite)
- Preview live : textarea droite mise a jour en debounce 300ms
- Options avancees : section retractable (accordion)
- Sauvegarde config : localStorage (pas de DB)
- Copier en 1 clic : conserve et ameliore
- Init sequence : conserve et ameliore (complexite choisie des le debut)
- Backend : PromptBuilderInput etendu (autonomy_mode, agents[], tags[], urgency, sensitive_data)
- Template prompt enrichi : inclut mode autonomie, agents, tags, urgence, donnees sensibles
- Tests Rust : mise a jour test_generate_prompt_non_empty + nouveaux cas

### OUT (exclus de la sequence)

- Editeur de template Jinja/Mustache custom
- Gestion multi-profils utilisateurs
- Sauvegarde configs cote serveur (DB)
- Export PDF/Word du prompt
- Historique des prompts generes
- Partage de config entre utilisateurs

## Decoupe fonctionnelle

| Bloc | Description | Priorite | Agents pressentis |
|------|-------------|----------|------------------|
| A — Champs enrichis | Classe T1-T5, stack select+libre, domaine etendu, complexite visible, mode autonomie | MVP | Lise (front) |
| B — Options avancees | Agents multiselect, tags multiselect, urgence toggle, donnees sensibles toggle, MSW toggle | MVP | Lise (front) |
| C — Preview live | Layout bi-panneaux, debounce 300ms, textarea preview readonly synchronisee | MVP | Lise (front) |
| D — Backend enrichi | PromptBuilderInput etendu, prompt_handler enrichi, tests mis a jour | MVP | Francois (back) |
| E — Persistance | Sauvegarde/restauration config via localStorage | V1 | Lise (front) |

## MVP — Definition minimale viable

Blocs A + B + C + D : formulaire enrichi (T1-T5, stack select, domaine, complexite visible, mode autonomie, agents checkboxes, tags, urgence, donnees sensibles) + preview live + backend etendu. La config se retrouve dans le prompt genere copie en 1 clic.

## Dependances identifiees

| Dependance | Type | Statut |
|-----------|------|--------|
| `apps/mipower/static/index.html` | interne | existant (a modifier) |
| `apps/mipower/static/app.js` | interne | existant (a modifier) |
| `apps/mipower/static/app.css` | interne | existant (a modifier) |
| `apps/mipower/src/api.rs` | interne | existant (a modifier) |
| `apps/mipower/src/models.rs` | interne | existant (a modifier) |
| marked.js, DOMPurify, mermaid | externe (CDN) | existant (non modifie) |
| localStorage browser API | externe (navigateur) | standard, dispo |

## Complexite estimee

- Complexite sequence : **C4 — elevee**
- Justification : 5 fichiers modifies (HTML, CSS, JS, api.rs, models.rs) + nouveaux tests. State management JS plus complexe (grille agents, debounce). Layout bi-panneaux CSS a concevoir. Le tout dans un seul service existant sans nouveau crate.
- Sera confirme par Denis en T8.

## Risques principaux

| Risque | Impact | Mitigation envisagee |
|--------|--------|---------------------|
| Debounce preview trop reactif (appels /api/prompt trop frequents) | moyen | Generation locale JS sans appel API (template string cote client) ou debounce 500ms |
| CSS layout bi-panneaux casse sur petits ecrans | faible | Responsive avec flex-column en dessous de 900px |
| localStorage taille limite (5MB) | faible | Ne sauvegarder que les champs texte + selects (pas le prompt genere) |
| MSW toggle semantique floue | faible | Specifier en T6, toggle optionnel |

## Solutions envisagees

| Solution | Avantages | Inconvenients | Score |
|----------|-----------|--------------|-------|
| Preview via appel /api/prompt a chaque frappe | Prompt identique au serveur | Trop d'appels reseau, latence | 5/10 |
| Preview locale (template JS pur) | Instantane, 0 appel reseau | Template duplique (JS + Rust) | 7/10 |
| Preview locale + sync serveur au submit | Meilleur des deux : preview rapide, prompt final certifie serveur | Legere difference visuelle possible | 9/10 |

## Solution retenue

**Preview locale (template JS) + sync serveur au submit.** Le bouton "Generer" reste le seul point d'appel API pour le prompt final. La preview live utilise un template JS miroir du template Rust — acceptable car le template est simple et stable.
