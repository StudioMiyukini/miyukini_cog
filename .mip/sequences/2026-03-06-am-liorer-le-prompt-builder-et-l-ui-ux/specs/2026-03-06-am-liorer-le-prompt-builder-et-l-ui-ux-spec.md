# Specification — Ameliorer le Prompt Builder et l'UI/UX

## Statut

- Etat : TERMINE
- Phase : P0 Temps 6
- Responsable principal : Francois

## TL;DR

Prompt Builder v2 : 5 fichiers modifies (models.rs, api.rs, index.html, app.js, app.css). Nouveaux champs : T1-T5 complet, stack select predifini, domaine etendu, complexite visible des le debut, mode autonomie, agents multiselect, tags chips, urgence, donnees sensibles, MSW toggle. Preview live JS locale (debounce 300ms) + sync serveur au submit. Layout bi-panneaux. Section Options avancees retractable. Aucune nouvelle dependance Cargo.

---

## Architecture

- **Backend** : Rust/axum `apps/mipower/src/`
  - `models.rs` — etendre `PromptBuilderInput` (5 champs nouveaux)
  - `api.rs` — enrichir `prompt_handler` + validations longueur/whitelist
- **Frontend** : HTML/CSS/JS vanilla `apps/mipower/static/`
  - `index.html` — nouveaux champs, layout bi-panneaux, section avancee
  - `app.js` — preview live debounce, agents grid, tags chips, localStorage
  - `app.css` — bi-panneaux flex, accordion, tags chips, responsive 900px

---

## Schema de donnees — PromptBuilderInput (etendu)

```rust
pub struct PromptBuilderInput {
    pub title:          String,          // max 200c
    pub task_class:     String,          // T1|T2|T3|T4|T5
    pub domain:         String,          // back|front|fullstack|infra|ai-ml|securite|data|autre
    pub description:    String,          // max 2000c
    pub constraints:    Option<String>,  // max 500c
    pub stack:          Option<String>,  // max 200c
    pub autonomy_mode:  Option<String>,  // FULL|BIG_STEPS|GUIDED|null
    pub agents:         Vec<String>,     // whitelist 10 agents connus, max 10
    pub tags:           Vec<String>,     // max 10 tags, max 50c chacun
    pub urgency:        bool,
    pub sensitive_data: bool,
    pub msw_toggle:     bool,
}
```

---

## API / Interfaces

### POST /api/prompt (modifie)

Validations nouvelles cote serveur :
- `title` : len <= 200, non vide
- `description` : len <= 2000
- `constraints` : len <= 500 si Some
- `stack` : len <= 200 si Some
- `task_class` : whitelist `[T1, T2, T3, T4, T5]`
- `domain` : whitelist `[back, front, fullstack, infra, ai-ml, securite, data, autre]`
- `autonomy_mode` : whitelist `[FULL, BIG_STEPS, GUIDED]` ou None
- `agents` : whitelist 10 agents MIP, max 10 entrees
- `tags` : max 10, longueur max 50c chacun

**Template prompt genere** (format canonique — JS et Rust DOIVENT etre identiques) :
```
Lance une sequence MIP pour : {title}

Classe estimee : {class}
Domaine : {domain}
Stack : {stack}
Contraintes : {constraints}
[Mode autonomie : {mode}]       <- si specifie
[Urgence : Oui]                 <- si urgency=true
[Donnees sensibles : Oui]       <- si sensitive_data=true
[Mode Sans Web : Oui]           <- si msw_toggle=true
[Agents actifs : {liste}]       <- si agents non vide
[Tags : {liste}]                <- si tags non vides

Description :
{description}

---
Maria, classe cette demande et lance P0.
```

### Autres endpoints : inchanges

---

## Composants UI

### Layout bi-panneaux (vue #builder)

```
.builder-layout { display: flex; gap: 1.5rem; }
  .prompt-form   { flex: 1; min-width: 320px; }
  .prompt-preview { flex: 1; min-width: 320px; position: sticky; top: 0; }

@media (max-width: 900px) { .builder-layout { flex-direction: column; } }
```

### Champs de base (toujours visibles)

| ID | Type | Options |
|----|------|---------|
| pb-title | text maxlength=200 | required |
| pb-class | select | T1..T5 |
| pb-domain | select | back, front, fullstack, infra, ai-ml, securite, data, autre |
| pb-complexity | select | C1..C5 (visible des le debut) |
| pb-autonomy | select | FULL, BIG_STEPS, GUIDED, (non specifie) |
| pb-stack-preset | select | 9 stacks + Autre |
| pb-stack | text maxlength=200 | auto-rempli par preset, modifiable |
| pb-desc | textarea maxlength=2000 | required, rows=5 |
| pb-constraints | text maxlength=500 | optionnel |

### Stacks predefinies

Rust + axum + SQLite / Rust + Dioxus / Rust + axum + Dioxus / TypeScript + React / TypeScript + Next.js / Python + FastAPI / Python + Django / Go + Fiber / Kotlin + Spring Boot / Autre (vide)

### Section Options avancees (details HTML natif)

```html
<details id="pb-advanced">
  <summary>Options avancees</summary>
  <!-- agents, tags, toggles -->
</details>
```

Agents : grille 2 colonnes, 10 checkboxes (Maria, Denis, Lise, Victor, Hugo, Fabrice, George, Jean, Arianne, Francois).

Tags : `<input>` + bouton Ajouter + liste de chips cliquables (clic = supprimer).

Toggles : Urgence | Donnees sensibles | MSW (Mode Sans Web).

### Preview live

Zone droite toujours visible : `<textarea id="pb-preview" readonly>`. Mise a jour via `buildPromptLocal()` en debounce 300ms sur tout changement de champ.

---

## Securite

- Validation longueur + whitelist : `api.rs` (400 Bad Request si violation)
- Preview = textarea readonly, pas de XSS
- localStorage : acceptable (usage local, single origin)
- cargo audit en P4

---

## Dependances

Aucune nouvelle dependance Cargo. CDN maintenu : marked@12, dompurify@3, mermaid@10.

---

## Criteres d'acceptance

1. T1/T2/T3/T4/T5 dans le select Classe (T1+T2 etaient absents)
2. Stack preset selectionne → pb-stack pre-rempli (modifiable)
3. Agents selectionnes → apparaissent dans le prompt genere ET dans la preview
4. Preview mise a jour <= 400ms apres toute modification
5. Bouton Copier copie le prompt final (serveur) dans le presse-papier
6. Config sauvegardee localStorage, restauree a la prochaine ouverture
7. cargo test -p mipower : 0 regression + 2 nouveaux tests passes
8. Layout bi-panneaux sur >= 900px, colonne sur < 900px
9. Section Options avancees repliee par defaut
10. Validation 400 si title > 200c
