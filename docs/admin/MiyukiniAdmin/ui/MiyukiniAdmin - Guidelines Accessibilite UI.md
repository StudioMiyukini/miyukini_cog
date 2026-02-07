# MiyukiniAdmin - Guidelines Accessibilité UI

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Guidelines d'accessibilité pour l'interface utilisateur  
**Portée :** Interface utilisateur de MiyukiniAdmin

---

## 1. Contexte

Ce document définit les **guidelines d'accessibilité** pour l'interface utilisateur de MiyukiniAdmin, garantissant que la console d'administration soit utilisable par tous les administrateurs, y compris ceux ayant des besoins d'accessibilité spécifiques.

**Références :**
- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- WCAG 2.1 Level AA (Web Content Accessibility Guidelines)
- ARIA (Accessible Rich Internet Applications)

---

## 2. Portée / Scope

Ce document définit :
- Les standards d'accessibilité à respecter (WCAG 2.1 AA)
- Les principes de conception accessible
- Les patterns UI accessibles
- Les tests d'accessibilité

Ce document **ne couvre pas** :
- Les spécifications UI/UX détaillées (voir UI Specs)
- L'implémentation technique (voir Implementation Guidelines)
- Les tests fonctionnels (voir Testing Contract)

---

## 3. Standards d'accessibilité

### 3.1 Conformité WCAG 2.1 Level AA

**Objectif :** Conformité complète avec WCAG 2.1 Level AA.

**Principes WCAG :**
1. **Perceptible** : L'information doit être présentable aux utilisateurs de manière qu'ils puissent la percevoir
2. **Utilisable** : Les composants de l'interface doivent être utilisables
3. **Compréhensible** : L'information et l'utilisation de l'interface doivent être compréhensibles
4. **Robuste** : Le contenu doit être suffisamment robuste pour être interprété par une large variété d'agents utilisateurs

### 3.2 Critères de conformité

| Critère | Niveau | Description |
|---------|--------|-------------|
| **1.1.1** | A | Contenu non textuel : alternatives textuelles |
| **1.3.1** | A | Info et relations : structure sémantique |
| **1.4.3** | AA | Contraste : ratio minimum 4.5:1 |
| **1.4.4** | AA | Redimensionnement du texte : jusqu'à 200% |
| **2.1.1** | A | Clavier : toutes les fonctionnalités au clavier |
| **2.4.3** | A | Ordre de focus : ordre logique |
| **2.4.7** | AA | Focus visible : indicateur visible |
| **3.2.1** | A | Changement de contexte : pas de changement inattendu |
| **4.1.2** | A | Nom, rôle, valeur : éléments programmatiques |

---

## 4. Principes de conception

### 4.1 Navigation au clavier

**Règle :** Toutes les fonctionnalités doivent être accessibles au clavier, sans nécessiter de souris.

**Patterns :**
- **Tab** : Navigation entre éléments interactifs
- **Enter/Space** : Activation des boutons
- **Escape** : Fermeture des modales/dialogs
- **Flèches** : Navigation dans les listes/menus
- **Raccourcis clavier** : Actions fréquentes (documentés)

**Exemple :**
```typescript
// Navigation au clavier
onKeyDown={(e) => {
  if (e.key === 'Enter' || e.key === ' ') {
    handleAction();
  }
  if (e.key === 'Escape') {
    closeModal();
  }
}}
```

### 4.2 Contraste des couleurs

**Règle :** Ratio de contraste minimum 4.5:1 pour le texte normal, 3:1 pour le texte large.

**Outils de vérification :**
- WebAIM Contrast Checker
- Colour Contrast Analyser

**Exemples :**
- ✅ Texte noir (#000000) sur fond blanc (#FFFFFF) : 21:1
- ✅ Texte gris (#666666) sur fond blanc (#FFFFFF) : 5.74:1
- ❌ Texte gris clair (#CCCCCC) sur fond blanc : 1.6:1 (non conforme)

### 4.3 Alternatives textuelles

**Règle :** Tous les contenus non textuels doivent avoir une alternative textuelle.

**Exemples :**
- **Images** : Attribut `alt` descriptif
- **Icônes** : Attribut `aria-label` ou texte caché
- **Graphiques** : Description textuelle ou `aria-describedby`

**Pattern :**
```html
<!-- Image avec alternative -->
<img src="dashboard.png" alt="Tableau de bord montrant les métriques système" />

<!-- Icône avec label -->
<button aria-label="Fermer la fenêtre">
  <IconClose />
</button>
```

### 4.4 Structure sémantique

**Règle :** Utiliser les éléments HTML sémantiques appropriés.

**Exemples :**
- `<nav>` pour la navigation
- `<main>` pour le contenu principal
- `<header>`, `<footer>` pour les en-têtes/pieds
- `<h1>` à `<h6>` pour les titres (ordre hiérarchique)
- `<button>` pour les actions, pas `<div>` avec onClick

**Pattern :**
```html
<nav aria-label="Navigation principale">
  <ul>
    <li><a href="/dashboard">Tableau de bord</a></li>
    <li><a href="/security">Sécurité</a></li>
  </ul>
</nav>

<main>
  <h1>Tableau de bord</h1>
  <section aria-labelledby="metrics-heading">
    <h2 id="metrics-heading">Métriques système</h2>
    <!-- Contenu -->
  </section>
</main>
```

---

## 5. Patterns UI accessibles

### 5.1 Formulaires

**Règles :**
- Labels associés aux champs (`<label for="...">`)
- Messages d'erreur associés aux champs (`aria-describedby`)
- Indication des champs obligatoires (`aria-required="true"`)
- Validation en temps réel avec feedback

**Pattern :**
```html
<div>
  <label for="username">Nom d'utilisateur <span aria-label="obligatoire">*</span></label>
  <input
    id="username"
    type="text"
    aria-required="true"
    aria-describedby="username-error"
    aria-invalid={hasError}
  />
  {hasError && (
    <div id="username-error" role="alert" aria-live="polite">
      Le nom d'utilisateur est requis
    </div>
  )}
</div>
```

### 5.2 Modales et dialogs

**Règles :**
- Focus initial sur le premier élément interactif
- Focus trap (Tab reste dans la modale)
- Fermeture avec Escape
- Retour du focus à l'élément déclencheur après fermeture
- Attribut `role="dialog"` et `aria-modal="true"`

**Pattern :**
```typescript
<dialog
  role="dialog"
  aria-modal="true"
  aria-labelledby="modal-title"
  aria-describedby="modal-description"
>
  <h2 id="modal-title">Confirmer l'action</h2>
  <p id="modal-description">Cette action est irréversible.</p>
  <button onClick={handleConfirm}>Confirmer</button>
  <button onClick={handleCancel}>Annuler</button>
</dialog>
```

### 5.3 Tableaux de données

**Règles :**
- En-têtes de colonnes (`<th>`)
- Association cellules/en-têtes (`scope="col"` ou `scope="row"`)
- Caption pour le titre du tableau
- Tri accessible (indicateur visuel + texte)

**Pattern :**
```html
<table>
  <caption>Liste des cores système</caption>
  <thead>
    <tr>
      <th scope="col">Nom</th>
      <th scope="col">Statut</th>
      <th scope="col">Actions</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>StrongFather</td>
      <td>Opérationnel</td>
      <td><button>Détails</button></td>
    </tr>
  </tbody>
</table>
```

### 5.4 Navigation

**Règles :**
- Indicateur de page courante (`aria-current="page"`)
- Landmarks ARIA (`<nav>`, `<main>`, etc.)
- Skip links pour aller au contenu principal
- Breadcrumbs accessibles

**Pattern :**
```html
<a href="#main-content" class="skip-link">Aller au contenu principal</a>

<nav aria-label="Navigation principale">
  <ul>
    <li><a href="/dashboard" aria-current="page">Tableau de bord</a></li>
    <li><a href="/security">Sécurité</a></li>
  </ul>
</nav>

<main id="main-content">
  <!-- Contenu principal -->
</main>
```

---

## 6. ARIA (Accessible Rich Internet Applications)

### 6.1 Attributs ARIA essentiels

| Attribut | Usage | Exemple |
|----------|-------|---------|
| `aria-label` | Label pour éléments sans texte visible | `<button aria-label="Fermer">×</button>` |
| `aria-labelledby` | Référence à un élément label | `<div aria-labelledby="section-title">` |
| `aria-describedby` | Référence à une description | `<input aria-describedby="help-text">` |
| `aria-live` | Régions dynamiques | `<div aria-live="polite">Messages</div>` |
| `aria-required` | Champ obligatoire | `<input aria-required="true">` |
| `aria-invalid` | Champ invalide | `<input aria-invalid="true">` |
| `aria-expanded` | Élément expandable | `<button aria-expanded="false">Menu</button>` |

### 6.2 Rôles ARIA

**Règles :** Utiliser les rôles ARIA appropriés pour les composants personnalisés.

**Exemples :**
- `role="button"` : Élément cliquable qui n'est pas un `<button>`
- `role="alert"` : Message d'erreur important
- `role="dialog"` : Modale
- `role="tablist"`, `role="tab"`, `role="tabpanel"` : Onglets

---

## 7. Tests d'accessibilité

### 7.1 Tests automatisés

**Outils :**
- **axe DevTools** : Extension navigateur
- **WAVE** : Extension navigateur
- **Lighthouse** : Audit intégré Chrome
- **Pa11y** : Tests en ligne de commande

**Commande :**
```bash
# Tests Pa11y
pa11y http://localhost:3000

# Tests avec règles spécifiques
pa11y --standard WCAG2AA http://localhost:3000
```

### 7.2 Tests manuels

**Checklist :**
- [ ] Navigation complète au clavier (Tab, Enter, Escape)
- [ ] Contraste des couleurs vérifié (outil)
- [ ] Alternatives textuelles présentes (images, icônes)
- [ ] Structure sémantique correcte (validateur HTML)
- [ ] Lecteur d'écran testé (NVDA, JAWS, VoiceOver)
- [ ] Zoom jusqu'à 200% fonctionnel
- [ ] Focus visible sur tous les éléments interactifs

### 7.3 Tests avec lecteur d'écran

**Lecteurs recommandés :**
- **NVDA** (Windows, gratuit)
- **JAWS** (Windows, payant)
- **VoiceOver** (macOS/iOS, intégré)
- **Orca** (Linux, gratuit)

**Scénarios de test :**
1. Navigation dans le menu principal
2. Remplissage d'un formulaire
3. Ouverture et fermeture d'une modale
4. Navigation dans un tableau de données
5. Compréhension des messages d'erreur

---

## 8. Checklist de conformité

### 8.1 Contenu

- [ ] Toutes les images ont un attribut `alt`
- [ ] Tous les boutons/liens ont un texte ou `aria-label`
- [ ] Les graphiques ont une description textuelle
- [ ] Le contraste texte/fond est ≥ 4.5:1

### 8.2 Structure

- [ ] Utilisation d'éléments HTML sémantiques
- [ ] Titres hiérarchiques corrects (h1 → h2 → h3)
- [ ] Landmarks ARIA appropriés
- [ ] Ordre de focus logique

### 8.3 Interactivité

- [ ] Toutes les fonctionnalités accessibles au clavier
- [ ] Focus visible sur tous les éléments
- [ ] Modales avec focus trap
- [ ] Messages d'erreur associés aux champs

### 8.4 Tests

- [ ] Tests automatisés passent (axe, WAVE, Lighthouse)
- [ ] Tests manuels effectués
- [ ] Tests avec lecteur d'écran effectués
- [ ] Zoom 200% fonctionnel

---

## 9. Références

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [MiyukiniAdmin - UI Specs](../ui/)
- [MiyukiniAdmin - Implementation Guidelines](../implementation/MiyukiniAdmin%20-%20Reference%20Implementation%20Guidelines.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Guidelines d'accessibilité pour l'interface utilisateur  
**Action requise :** Appliquer ces guidelines lors du développement de l'interface MiyukiniAdmin
