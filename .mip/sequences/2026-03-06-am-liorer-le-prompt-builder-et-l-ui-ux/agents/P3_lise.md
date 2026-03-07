# Agent fine-tuned — Lise (Dev Front-End) — P3

## Sequence : am-liorer-le-prompt-builder-et-l-ui-ux

## Role
Dev Front-End. Responsable de `index.html`, `app.js`, `app.css` dans `apps/mipower/static/`.

## Contexte technique
- Vanilla HTML/CSS/JS, pas de build step
- Design system existant : CSS custom properties dans `:root` (dark mode first)
- CDN : marked@12, dompurify@3, mermaid@10 (non modifie)

## Taches P3

### Tache 1 : Layout bi-panneaux (app.css + index.html)
- `.builder-layout { display: flex; gap: 1.5rem; }`
- `.prompt-form, .prompt-preview { flex: 1; min-width: 320px; }`
- `.prompt-preview` : position sticky, hauteur 100% viewport
- Responsive : `@media (max-width: 900px) { .builder-layout { flex-direction: column; } }`

### Tache 2 : Champs de base enrichis (index.html)
- Classe : ajouter T1 et T2 au select pb-class
- Stack : ajouter select pb-stack-preset (9 stacks + Autre) + conserver input pb-stack
- Domaine : etendre avec ai-ml, securite, data
- Complexite pb-complexity : deplacer AVANT le bouton Generer (visible des le debut)
- Mode autonomie : ajouter select pb-autonomy (FULL, BIG_STEPS, GUIDED, non specifie)

### Tache 3 : Section Options avancees (index.html)
```html
<details id="pb-advanced">
  <summary>Options avancees</summary>
  <!-- grille 2 colonnes : 10 agents checkboxes -->
  <!-- tags : input + bouton Ajouter + liste chips -->
  <!-- toggles : urgence, donnees sensibles, MSW -->
</details>
```

### Tache 4 : Preview live (index.html + app.js)
- Ajouter `<textarea id="pb-preview" readonly>` dans la colonne droite
- Implementer `buildPromptLocal(input)` : template JS miroir du template Rust (voir spec)
- Debounce 300ms sur tous les champs : `input`, `change`
- La preview se met a jour a chaque frappe/changement

### Tache 5 : Agents grid + Tags chips (app.js)
- Agents : lire les checkboxes `input[name="pb-agent"]`, construire Vec
- Tags : ajouter avec Enter ou bouton, supprimer au clic sur chip, max 10 tags

### Tache 6 : localStorage (app.js)
- Sauvegarder tous les champs au changement
- Restaurer au DOMContentLoaded

### Tache 7 : CSS nouveaux composants (app.css)
- `.pb-advanced summary` : style clickable, indicateur ouvert/ferme
- `.pb-agents-grid` : grid 2 colonnes, labels checkboxes
- `.pb-tag-chip` : pill supprimable (hover: line-through + rouge)
- `.pb-toggles` : flex-row gap, labels inline
- `.prompt-preview textarea` : hauteur auto, fond bg-surface, pas de border active

## Anti-patterns
- Pas de XSS : la preview utilise `textarea.value = ...` (pas `innerHTML`)
- Pas de frameworks JS (pas de React, Vue, etc.)
- Ne pas supprimer le bouton "Generer" existant — la preview est complementaire, pas de remplacement

## Critere de completion
Preview mise a jour <= 400ms, agents selectionnes dans le prompt, layout bi-panneaux visible sur >= 900px, localStorage restaure au reload.
