# Miyukini Framework - UI Header et BottomNav

## Contexte

Ce document décrit les spécifications techniques et visuelles des composants de navigation principaux du Miyukini Framework :
- **Header** : Barre de navigation supérieure (sticky)
- **BottomNav** : Barre de navigation inférieure (mobile-first)

Ces composants sont réutilisables par tous les modules (Front, Back Office, Super Admin) et respectent le système de thèmes dynamiques documenté dans `Miyukini Framework - Atomic Design & Theme Dynamique.md`.

## Portée / Scope

- Composants : `Header.tsx`, `BottomNav.tsx`, `AppShellScreen.tsx`
- Chemins : `src/components/navigation/`, `src/components/layouts/`
- Technologies : React, Next.js, CSS Modules, Theming dynamique
- Dépendances : `useActiveTheme`, `useIsMobile`, `useAuth`, `useRoleSimulation`

---

## 1. Header (Navigation supérieure)

### 1.1 Fichiers sources

| Fichier | Chemin | Description |
|---------|--------|-------------|
| `Header.tsx` | `src/components/navigation/Header.tsx` | Header standard avec navigation |
| `HeaderWithEdition.tsx` | `src/components/navigation/HeaderWithEdition.tsx` | Header avec sélecteur d'édition/mode |

### 1.2 Positionnement

| Propriété | Valeur | Token/Variable | Description |
|-----------|--------|----------------|-------------|
| `position` | `fixed` | — | Fixé en haut de l'écran |
| `top` | `0` | — | Collé au bord supérieur |
| `width` | `100%` | — | Pleine largeur |
| `z-index` | `20` | `--z-header` | Au-dessus du contenu principal |
| `height` | `64px` | `--header-height` | Hauteur fixe |

### 1.3 Apparence visuelle

```css
/* Variables CSS recommandées */
.header {
  background: var(--color-background-90);    /* Fond semi-transparent (90% opacité) */
  backdrop-filter: blur(4px);                /* Flou d'arrière-plan léger */
  border-bottom: 1px solid var(--color-border);
}
```

| Propriété | Token | Valeur par défaut | Effet |
|-----------|-------|-------------------|-------|
| Background | `theme.colors.section.background` | `rgba(bg, 0.9)` | Fond avec 90% d'opacité (glassmorphism) |
| Backdrop | — | `blur(4px)` | Flou sur le contenu derrière |
| Border | `theme.colors.section.border` | `1px solid` | Séparation visuelle avec le contenu |

### 1.4 Dimensions

| Élément | Token/Variable | Valeur | Description |
|---------|----------------|--------|-------------|
| Hauteur | `--header-height` | 64px | Hauteur fixe du header |
| Container | `--container-max` | Variable | Centré avec max-width responsive |
| Padding horizontal | `theme.spacing.medium` | 16px | Espacement intérieur |

### 1.5 Layout interne (Flexbox)

```
┌─────────────────────────────────────────────────────────────────┐
│ [Logo + Nav Desktop]          [Centre]           [Actions]  [☰] │
│     flex + gap-4                                   gap-2        │
└─────────────────────────────────────────────────────────────────┘
```

| Zone | Layout | Contenu |
|------|--------|---------|
| Gauche | `flex items-center gap-4` | Logo + Navigation desktop |
| Centre | `flex items-center gap-3` | Sélecteur d'édition + Badge mode (optionnel) |
| Droite | `flex items-center gap-2` | Boutons connexion/profil |
| Burger | Mobile uniquement | Menu hamburger (visible < 800px) |

### 1.6 Breakpoints responsive

Le Miyukini Framework utilise un **breakpoint unique à 800px** (conformément à `Miyukini Framework - Layouts Responsives.md`) :

| Breakpoint | Largeur | Comportement |
|------------|---------|--------------|
| Mobile | `< 800px` | Menu burger, icônes seules, navigation simplifiée |
| Desktop | `≥ 800px` | Navigation horizontale complète, textes visibles |

#### Éléments masqués par breakpoint

| Élément | Mobile (< 800px) | Desktop (≥ 800px) |
|---------|------------------|-------------------|
| Navigation desktop | Masquée | Visible |
| Textes boutons | Masqués | Visibles |
| Username | Masqué | Visible |
| Menu burger | Visible | Masqué |

### 1.7 Menu mobile (Sheet/Drawer)

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| Composant | `Sheet` ou `Drawer` | Panneau latéral animé |
| Position | `side="right"` | Glisse depuis la droite |
| Largeur | `280px` | Largeur fixe |
| Overlay | `rgba(0, 0, 0, 0.5)` | Fond assombri |
| Animation | `200ms ease-out` | Transition fluide |

---

## 2. BottomNav (Navigation inférieure)

### 2.1 Fichier source

| Fichier | Chemin |
|---------|--------|
| `BottomNav.tsx` | `src/components/navigation/BottomNav.tsx` |

### 2.2 Positionnement

| Propriété | Token/Variable | Valeur | Description |
|-----------|----------------|--------|-------------|
| Position | — | `fixed` | Fixé dans le viewport |
| Bottom | — | `0` | Collé au bord inférieur |
| Left | — | `0` | Depuis le bord gauche |
| Right | — | `0` | Jusqu'au bord droit |
| Z-index | `--z-bottom-nav` | `50` | Au-dessus de tout le contenu |

```tsx
<nav className="bottom-nav">
  {/* position: fixed; bottom: 0; left: 0; right: 0; z-index: 50; */}
</nav>
```

### 2.3 Apparence visuelle

| Propriété | Token | Valeur par défaut | Description |
|-----------|-------|-------------------|-------------|
| Background | `theme.colors.section.card.background` | Variable selon thème | Fond adaptatif |
| Border top | `theme.colors.section.border` | `1px solid` | Séparation avec le contenu |

#### Par thème graphique

| Thème | Background | Border | Texte actif |
|-------|------------|--------|-------------|
| Standard | `#FFFFFF` | `#E5E7EB` | `#6366F1` (indigo) |
| Dark | `#1F2937` | `#374151` | `#818CF8` (indigo clair) |
| Oasis | `#0F766E` | `#14B8A6` | `#5EEAD4` (teal clair) |

### 2.4 Dimensions et espacement

| Élément | Token/Variable | Valeur | Description |
|---------|----------------|--------|-------------|
| Hauteur totale | `--bottom-nav-height` | ~70px | Dépend du contenu |
| Padding vertical item | `theme.spacing.small` | 12px | Espacement haut/bas |
| Padding horizontal item | `theme.spacing.small` | 12px | Espacement gauche/droite |
| Largeur min item | — | 64px | Largeur minimum par bouton |
| Touch target | `--touch-target` | 48×48px | Zone tactile (WCAG) |

### 2.5 Layout interne

```
┌────────────────────────────────────────────────────────────────┐
│  [🏠]     [📚]      [👥]      [📖]      [📄]      [⚙️]       │
│  Cat 1    Cat 2     Cat 3     Cat 4     Cat 5     Cat 6       │
│                                                                │
│  ← ─ ─ ─ ─ ─ justify-around ─ ─ ─ ─ ─ →                       │
└────────────────────────────────────────────────────────────────┘
```

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| Display | `flex` | Conteneur flex |
| Alignement items | `center` | Centrage vertical |
| Distribution | `space-around` | Espacement égal autour des items |
| Overflow-x | `auto` | Scroll horizontal si > 8 items |

### 2.6 Items de navigation (8 catégories par défaut)

#### Structure d'un item

```tsx
<button className="bottom-nav__item">
  {/* Indicateur actif (barre supérieure) */}
  <div className="bottom-nav__indicator" />
  
  {/* Icône */}
  <div className="touch-target">
    <Icon size={24} />
  </div>
  
  {/* Label */}
  <span className="bottom-nav__label">{label}</span>
</button>
```

#### Catégories par défaut

| ID | Label par défaut | Personnalisable |
|----|------------------|-----------------|
| `category_1` | Catégorie 1 | ✅ Oui (Back Office) |
| `category_2` | Catégorie 2 | ✅ Oui (Back Office) |
| `category_3` | Catégorie 3 | ✅ Oui (Back Office) |
| `category_4` | Catégorie 4 | ✅ Oui (Back Office) |
| `category_5` | Catégorie 5 | ✅ Oui (Back Office) |
| `category_6` | Catégorie 6 | ✅ Oui (Back Office) |
| `category_7` | Catégorie 7 | ✅ Oui (Back Office) |
| `category_8` | Catégorie 8 | ✅ Oui (Back Office) |

### 2.7 Taille des icônes

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| Size | `24×24px` | Icônes Lucide ou Heroicons |
| Container | `.touch-target` | min 48×48px (accessibilité) |

### 2.8 Typographie du label

| Propriété | Token | Valeur | Description |
|-----------|-------|--------|-------------|
| Taille | `theme.fonts.body.small` | 12px / 0.75rem | Petit texte |
| Marge top | `theme.spacing.xs` | 4px | Espacement avec l'icône |
| Font-weight | — | 400 (normal) | Par défaut |
| Font-weight actif | — | 500 (medium) | Item sélectionné |

### 2.9 États visuels

#### État inactif

| Propriété | Token | Valeur |
|-----------|-------|--------|
| Couleur texte/icône | `theme.colors.text.secondary` | Variable selon thème |
| Opacité | — | 1 |

#### État actif

| Propriété | Token | Valeur | Description |
|-----------|-------|--------|-------------|
| Couleur texte/icône | `theme.colors.accent` | Variable | Couleur d'accent du thème |
| Indicateur | — | 2px height | Barre horizontale animée |
| Font-weight | — | 500 | Medium |

#### État hover (desktop)

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| Background | `rgba(accent, 0.1)` | Fond léger au survol |
| Transform | `scale(1.05)` | Légère mise en avant |
| Transition | `150ms ease` | Animation fluide |

### 2.10 Animation

```tsx
// Indicateur actif animé
<div 
  className="bottom-nav__indicator"
  style={{
    transform: `translateX(${activeIndex * itemWidth}px)`,
    transition: 'transform 200ms ease-out'
  }}
/>
```

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| Animation | `transform 200ms ease-out` | Déplacement fluide |
| Indicateur | `2px × 100%` du bouton | Barre colorée |

### 2.11 Filtrage par rôle utilisateur

Les items affichés peuvent être filtrés selon le rôle (`useAuth`, `useRoleSimulation`) :

| Rôle | Items visibles |
|------|----------------|
| Visitor | Items publics uniquement |
| User (Gestion) | Tous les items standard |
| Admin | Tous + items admin |
| Super Admin | Tous + items super admin |

---

## 3. AppShellScreen (Wrapper Layout)

### 3.1 Structure triptyque

```
┌────────────────────────────────────────────────────────────────┐
│ HEADER (fixed top-0, z-20, h-16)                               │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│                    BODY / CONTENT                              │
│                    (pt-16 pb-20)                               │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│ BOTTOMNAV (fixed bottom-0, z-50, ~70px)                        │
└────────────────────────────────────────────────────────────────┘
```

### 3.2 Padding de compensation

Le `BottomNav` et `Header` étant fixes, le contenu principal doit avoir des paddings de compensation :

```tsx
// AppShellScreen.tsx
export function AppShellScreen({ children }: { children: React.ReactNode }) {
  return (
    <div className="app-shell">
      {/* Header - 64px */}
      <div className="app-shell__content" style={{ paddingTop: 64, paddingBottom: 80 }}>
        {children}
      </div>
      {/* BottomNav - ~70px */}
    </div>
  )
}
```

| Zone | Padding | Variable | Description |
|------|---------|----------|-------------|
| Top | `64px` | `--header-height` | Compense le Header fixe |
| Bottom | `80px` | `--bottom-nav-height + safe-area` | Compense le BottomNav + safe area iOS |

### 3.3 Safe area (iOS)

Pour les appareils avec encoche ou barre de navigation système :

```css
.safe-area-bottom {
  padding-bottom: calc(var(--bottom-nav-height) + env(safe-area-inset-bottom));
}
```

---

## 4. Variables CSS globales

```css
:root {
  /* Z-index layers */
  --z-content: 1;
  --z-header: 20;
  --z-modal: 40;
  --z-bottom-nav: 50;
  --z-toast: 60;
  
  /* Dimensions */
  --header-height: 64px;
  --bottom-nav-height: 70px;
  --touch-target: 48px;
  --container-max: 1200px;
  
  /* Breakpoint unique */
  --breakpoint-mobile: 800px;
}
```

---

## 5. Classes utilitaires

### 5.1 Touch target (Accessibilité mobile)

```css
.touch-target {
  min-height: 48px;
  min-width: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
}
```

- **Objectif** : Zone tactile minimum de 48×48px (recommandation WCAG 2.1 AA)
- **Usage** : Wrapper autour des icônes cliquables

### 5.2 Glass card (Effet glassmorphism)

```css
.glass-card {
  backdrop-filter: blur(16px);
  background: rgba(var(--color-background-rgb), 0.4);
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}
```

### 5.3 Active indicator

```css
.active-indicator {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--color-accent);
  transition: transform 200ms ease-out;
}
```

---

## 6. Recommandations d'implémentation

### 6.1 Hooks requis

| Hook | Usage | Import |
|------|-------|--------|
| `useActiveTheme` | Couleurs dynamiques | `@/components/hooks/useActiveTheme` |
| `useIsMobile` | Breakpoint 800px | `@/components/hooks/useIsMobile` |
| `useAuth` | Rôle utilisateur | `@/components/hooks/useAuth` |
| `useRoleSimulation` | Test de rôles (dev) | `@/components/hooks/useRoleSimulation` |

### 6.2 Exemple d'intégration

```tsx
'use client'

import { useActiveTheme } from '@/components/hooks/useActiveTheme'
import { useIsMobile } from '@/components/hooks/useIsMobile'

export function BottomNav() {
  const theme = useActiveTheme()
  const isMobile = useIsMobile()
  
  const navStyle = {
    background: theme.colors.section.card.background,
    borderTop: `1px solid ${theme.colors.section.border}`,
  }
  
  return (
    <nav className="bottom-nav" style={navStyle}>
      {/* Items */}
    </nav>
  )
}
```

### 6.3 Accessibilité

| Critère | Implémentation |
|---------|----------------|
| Touch target | Min 48×48px |
| Contraste | Min 4.5:1 (WCAG AA) |
| Focus visible | Outline 2px |
| ARIA | `role="navigation"`, `aria-current` |
| Keyboard | Tab navigation, Enter/Space activation |

---

## 7. Schéma récapitulatif

```
┌────────────────────────────────────────────────────────────────┐
│ HEADER (fixed top-0, z-20, h-16)                               │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │ [🏮 Logo]  [Nav Desktop (≥800px)]        [Actions] [☰]     │ │
│ │            hidden on mobile              gap-2     mobile  │ │
│ └────────────────────────────────────────────────────────────┘ │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│                    CONTENU PRINCIPAL                           │
│                    (ContentStack)                              │
│                    pt: 64px, pb: 80px                          │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│ BOTTOMNAV (fixed bottom-0, z-50, ~70px)                        │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │   [Icon]    [Icon]    [Icon]    [Icon]    ...    [Icon]    │ │
│ │   12px      12px      12px      12px             12px      │ │
│ │   ─────── indicateur actif (2px, accent color) ─────────   │ │
│ │                  + safe-area-inset-bottom                  │ │
│ └────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────┘
```

---

## 8. Stories & Tests

| Composant | Story | Test |
|-----------|-------|------|
| Header | `stories/navigation/header.stories.tsx` | `__tests__/Header.test.tsx` |
| BottomNav | `stories/navigation/bottom-nav.stories.tsx` | `__tests__/BottomNav.test.tsx` |
| AppShellScreen | `stories/layouts/app-shell.stories.tsx` | `__tests__/AppShellScreen.test.tsx` |

### Cas de test prioritaires

- [ ] Rendu correct par thème (Standard, Dark, Oasis)
- [ ] Responsive : affichage mobile vs desktop (breakpoint 800px)
- [ ] Animation de l'indicateur actif
- [ ] Filtrage des items par rôle utilisateur
- [ ] Safe area iOS
- [ ] Accessibilité clavier (Tab, Enter)
- [ ] Touch target ≥ 48px

---

## 9. Changelog

| Date | Version | Modification |
|------|---------|--------------|
| 2026-01-10 | 1.0 | Création initiale inspirée de Catakana |
