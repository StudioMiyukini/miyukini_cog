# Miyukini Central — Packs UI CSS Style Chrome

**Date de création :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document de référence

---

## Contexte

Recherche de packs UI CSS pour créer une interface style Chrome, complémentaire aux assets pixel art. Ces packs peuvent être utilisés pour des versions web ou comme référence pour le style.

---

## Portée / Scope

- **Périmètre :** Packs UI CSS style Chrome ; frameworks CSS ; composants réutilisables.
- **Hors périmètre :** Implémentation spécifique (sera décidée après sélection).

---

## 1. Packs CSS Style Chrome (Recommandés)

### 1.1 chrome-tabs ⭐ **MEILLEURE OPTION**

**Source :** [github.com/adamschwartz/chrome-tabs](https://github.com/adamschwartz/chrome-tabs)  
**Licence :** MIT  
**Stars :** 1.7k+  
**Demo :** [adamschwartz.co/chrome-tabs](https://adamschwartz.co/chrome-tabs/)

**Description :**
- Onglets style Chrome en HTML/CSS/ES6
- Implémentation complète et fidèle
- Pas de dépendances lourdes
- Code propre et maintenu

**Caractéristiques :**
- ✅ Onglets avec coins arrondis en haut
- ✅ Courbe concave en bas (onglet actif)
- ✅ Drag & drop des onglets
- ✅ Fermeture avec animation
- ✅ Responsive
- ✅ Accessible (clavier)

**Fichiers inclus :**
- `chrome-tabs.css` — Styles CSS
- `chrome-tabs.js` — Logique JavaScript
- Exemples HTML

**Installation :**
```bash
# Via npm
npm install chrome-tabs

# Ou téléchargement direct depuis GitHub
git clone https://github.com/adamschwartz/chrome-tabs.git
```

**Utilisation :**
```html
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="chrome-tabs.css">
</head>
<body>
    <chrome-tabs>
        <chrome-tab title="HUB" favicon="🔄" active></chrome-tab>
        <chrome-tab title="Calculatrice" favicon="🔢"></chrome-tab>
        <chrome-tab title="Jeu" favicon="🎮"></chrome-tab>
    </chrome-tabs>
    <script src="chrome-tabs.js"></script>
</body>
</html>
```

**Avantages :**
- ✅ **Style Chrome exact** — reproduction fidèle
- ✅ **Léger** — CSS + JS minimal
- ✅ **Documenté** — exemples et documentation
- ✅ **Actif** — projet maintenu
- ✅ **MIT** — libre d'utilisation

---

### 1.2 chrome-like-tabs

**Source :** [github.com/while0pass/chrome-like-tabs](https://github.com/while0pass/chrome-like-tabs)  
**Licence :** Open source  
**Stars :** 1

**Description :**
- Implémentation CSS3 pure
- Plus simple que chrome-tabs
- Focus sur le style visuel

**Caractéristiques :**
- ✅ CSS3 uniquement (pas de JS requis)
- ✅ Style Chrome simplifié
- ✅ Léger

**Avantages :**
- ✅ **Simple** — CSS uniquement
- ✅ **Léger** — pas de JavaScript
- ⚠️ **Moins de fonctionnalités** — pas de drag & drop

---

### 1.3 Chrome-in-Html

**Source :** [github.com/glitch128/Chrome-in-Html](https://github.com/glitch128/Chrome-in-Html)  
**Licence :** MIT  
**Stars :** 3

**Description :**
- Recréation complète de Chrome en HTML/CSS/JS
- Basé sur chrome-tabs
- Interface complète (barre d'adresse, etc.)

**Caractéristiques :**
- ✅ Interface Chrome complète
- ✅ Barre d'adresse
- ✅ Boutons de navigation
- ✅ Menu

**Avantages :**
- ✅ **Complet** — interface entière
- ✅ **Référence** — pour comprendre Chrome UI
- ⚠️ **Lourd** — plus que nécessaire si on veut juste les onglets

---

## 2. Frameworks CSS Pixel Art

### 2.1 Pixel UI (8-Bit Retro) — Composants React + CSS

**Source :** [pixel-art-8-bit.mishrashardendu22.is-a.dev](https://pixel-art-8-bit.mishrashardendu22.is-a.dev/)  
**GitHub :** Disponible  
**Licence :** Open source

**Description :**
- 100+ composants pixel art
- React + CSS
- Style 8-bit authentique

**Composants inclus :**
- Navigation tabs
- Buttons
- Forms
- Display elements
- Cards
- Modals

**CSS disponible :**
- Les styles CSS peuvent être extraits pour usage standalone
- Variables CSS personnalisables

**Avantages :**
- ✅ **Complet** — 100+ composants
- ✅ **Pixel art authentique**
- ✅ **CSS modulaire**
- ⚠️ **React** — mais CSS réutilisable

---

### 2.2 SNES.CSS

**Source :** [snes-css.sadlative.com](https://snes-css.sadlative.com/)  
**GitHub :** Disponible  
**Licence :** Open source

**Description :**
- Framework CSS rétro style années 90
- Style 16-bit
- Pas de JavaScript requis

**Caractéristiques :**
- ✅ Variables CSS personnalisables
- ✅ Compatible React/Tailwind
- ✅ Style pixel art
- ✅ Textes, liens, boutons, listes

**Avantages :**
- ✅ **Framework complet**
- ✅ **Style rétro authentique**
- ✅ **CSS pur** — pas de JS

---

### 2.3 pixelCSS

**Source :** [github.com/gwannon/pixelCSS](https://github.com/gwannon/pixelCSS)  
**Licence :** Open source

**Description :**
- Mini-framework CSS3 pour pixel art
- Léger et configurable
- Création de pixel art avec CSS

**Avantages :**
- ✅ **Léger**
- ✅ **Configurable**
- ✅ **CSS3 pur**

---

## 3. Packs UI CSS Généraux

### 3.1 CSSUI

**Source :** [cssui.dev/docs/components/tabs](https://cssui.dev/docs/components/tabs)  
**Licence :** Open source

**Description :**
- Framework CSS moderne
- Composant tabs inclus
- Variables CSS personnalisables

**Caractéristiques :**
- ✅ Navigation horizontale
- ✅ Accessible (clavier)
- ✅ Variables CSS
- ✅ Design propre

**Avantages :**
- ✅ **Moderne**
- ✅ **Accessible**
- ⚠️ **Pas spécifiquement Chrome** — style générique

---

## 4. Recommandation : chrome-tabs

### 4.1 Pourquoi cette option ?

1. **Style Chrome exact** — reproduction fidèle de l'interface Chrome
2. **Populaire** — 1.7k+ stars, bien maintenu
3. **Complet** — fonctionnalités complètes (drag & drop, animations)
4. **Documenté** — exemples et documentation claire
5. **MIT** — libre d'utilisation commerciale
6. **Léger** — CSS + JS minimal

### 4.2 Plan d'intégration

#### Option A : Utilisation directe (Web)

1. Télécharger chrome-tabs depuis GitHub
2. Intégrer dans une version web de Miyukini Central
3. Adapter les styles pour correspondre à nos besoins

#### Option B : Inspiration pour egui (Rust)

1. Analyser le CSS de chrome-tabs
2. Comprendre les techniques utilisées (coins arrondis, courbe concave)
3. Adapter dans egui avec des formes personnalisées

#### Option C : Génération de sprites

1. Utiliser chrome-tabs pour générer des captures d'écran
2. Convertir en sprites pixel art
3. Intégrer dans egui comme textures

---

## 5. Combinaison recommandée

### Stack complète

1. **Assets pixel art** (Kenney's Pack) — Sprites PNG universels
2. **CSS chrome-tabs** — Référence/style pour web
3. **Adaptation egui** — Implémentation Rust avec les concepts CSS

**Avantages :**
- ✅ **Multi-plateforme** — Web (CSS) + Desktop (egui)
- ✅ **Style cohérent** — même apparence partout
- ✅ **Réutilisable** — assets PNG + concepts CSS

---

## 6. Installation et utilisation

### chrome-tabs

```bash
# Installation npm
npm install chrome-tabs

# Ou téléchargement GitHub
git clone https://github.com/adamschwartz/chrome-tabs.git
cd chrome-tabs
```

**Structure des fichiers :**
```
chrome-tabs/
├── chrome-tabs.css      # Styles CSS
├── chrome-tabs.js       # Logique JavaScript
├── chrome-tabs.html     # Exemple HTML
└── README.md            # Documentation
```

**Intégration dans un projet web :**
```html
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Miyukini Central</title>
    <link rel="stylesheet" href="chrome-tabs/chrome-tabs.css">
    <style>
        /* Styles personnalisés */
        :root {
            --chrome-tab-height: 36px;
            --chrome-tab-background: #f2f2f4;
        }
    </style>
</head>
<body>
    <chrome-tabs>
        <chrome-tab title="HUB" favicon="🔄" active></chrome-tab>
        <chrome-tab title="Calculatrice" favicon="🔢"></chrome-tab>
        <chrome-tab title="Jeu" favicon="🎮"></chrome-tab>
    </chrome-tabs>
    
    <script src="chrome-tabs/chrome-tabs.js"></script>
</body>
</html>
```

---

## 7. Adaptation pour egui (Rust)

### Analyse du CSS chrome-tabs

Les techniques CSS utilisées peuvent être adaptées en Rust/egui :

**Coins arrondis :**
```css
/* CSS */
border-radius: 8px 8px 0 0;
```

```rust
// Rust/egui équivalent
egui::CornerRadius {
    nw: 8, ne: 8, sw: 0, se: 0
}
```

**Courbe concave :**
```css
/* CSS - utilise clip-path ou SVG */
clip-path: ellipse(100% 50% at 50% 100%);
```

```rust
// Rust/egui - polygone avec courbe
let mut points = Vec::new();
// Points pour créer la courbe concave
```

---

## 8. Ressources supplémentaires

### Documentation

- [chrome-tabs GitHub](https://github.com/adamschwartz/chrome-tabs)
- [Demo chrome-tabs](https://adamschwartz.co/chrome-tabs/)
- [CSSUI Documentation](https://cssui.dev/docs/components/tabs)

### Communautés

- [CodePen Chrome Tabs](https://codepen.io/search/pens?q=chrome+tabs)
- [CSS-Tricks Browser UI](https://css-tricks.com/)

---

## 9. Prochaines étapes

1. ✅ **Télécharger chrome-tabs** depuis GitHub
2. ✅ **Analyser le CSS** pour comprendre les techniques
3. ✅ **Tester** dans un projet web de démonstration
4. ✅ **Adapter** les concepts pour egui (Rust)
5. ✅ **Combiner** avec les assets pixel art (Kenney's Pack)

---

**Date de dernière mise à jour :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document de référence
