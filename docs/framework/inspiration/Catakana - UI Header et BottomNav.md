# Catakana - UI Header et BottomNav

## Contexte

Ce document décrit les spécifications techniques et visuelles des composants de navigation principaux de l'application Catakana :
- **Header** : Barre de navigation supérieure (sticky)
- **BottomNav** : Barre de navigation inférieure (mobile-first)

## Portée / Scope

- Composants : `Header.tsx`, `HeaderWithEdition.tsx`, `BottomNav.tsx`
- Chemins : `src/components/organisms/`, `src/components/`
- Technologies : React, Tailwind CSS, Framer Motion, Radix UI

---

## 1. Header (Navigation supérieure)

### 1.1 Fichiers sources

| Fichier | Chemin | Description |
|---------|--------|-------------|
| `Header.tsx` | `src/components/organisms/Header.tsx` | Header standard avec navigation |
| `HeaderWithEdition.tsx` | `src/components/organisms/HeaderWithEdition.tsx` | Header avec sélecteur d'édition et mode |

### 1.2 Positionnement

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| `position` | `fixed` | Fixé en haut de l'écran |
| `top` | `0` | Collé au bord supérieur |
| `width` | `100%` (`w-full`) | Pleine largeur |
| `z-index` | `20` (`z-20`) | Au-dessus du contenu principal |

### 1.3 Apparence visuelle

```css
/* Classes Tailwind appliquées */
bg-background/90      /* Fond semi-transparent (90% opacité) */
backdrop-blur-sm      /* Flou d'arrière-plan léger */
border-b              /* Bordure inférieure */
```

| Propriété | Valeur | Effet |
|-----------|--------|-------|
| Background | `bg-background/90` | Fond avec 90% d'opacité pour effet glassmorphism |
| Backdrop | `backdrop-blur-sm` | Flou de 4px sur le contenu derrière |
| Border | `border-b` | Séparation visuelle avec le contenu |

### 1.4 Dimensions

| Élément | Classe | Valeur | Description |
|---------|--------|--------|-------------|
| Hauteur | `h-16` | 64px | Hauteur fixe du header |
| Container | `container mx-auto` | Variable | Centré avec max-width responsive |
| Padding horizontal | `px-4` | 16px | Espacement intérieur |

### 1.5 Layout interne (Flexbox)

```
┌─────────────────────────────────────────────────────────────────┐
│ [Logo + Nav Desktop]          [Centre]           [Actions]  [☰] │
│     flex + gap-4                                   gap-2        │
└─────────────────────────────────────────────────────────────────┘
```

| Zone | Classe | Contenu |
|------|--------|---------|
| Gauche | `flex items-center gap-4` | Logo + Navigation desktop |
| Centre | `flex items-center gap-3` | Sélecteur d'édition + Badge mode (HeaderWithEdition) |
| Droite | `flex items-center gap-2` | Boutons connexion/profil |
| Burger | `md:hidden` | Menu mobile (visible < 768px) |

### 1.6 Breakpoints responsive

| Breakpoint | Largeur | Comportement |
|------------|---------|--------------|
| `< sm` (640px) | Mobile | Icônes seules, textes masqués |
| `sm` - `md` | Tablette | Textes partiels visibles |
| `≥ md` (768px) | Desktop | Navigation horizontale complète |

#### Éléments masqués par breakpoint

| Élément | Classe | Visible à partir de |
|---------|--------|---------------------|
| Navigation desktop | `hidden md:flex` | 768px |
| Texte "Connexion" | `hidden sm:inline` | 640px |
| Texte "Dashboard" | `hidden md:inline` | 768px |
| Username | `hidden md:inline` | 768px |
| Menu burger | `md:hidden` | Masqué à 768px |

### 1.7 Menu mobile (Sheet)

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| Composant | `Sheet` (Radix UI) | Panneau latéral animé |
| Position | `side="right"` | Glisse depuis la droite |
| Largeur | `w-80` | 320px |
| Overlay | Automatique | Fond assombri |

---

## 2. BottomNav (Navigation inférieure)

### 2.1 Fichier source

| Fichier | Chemin |
|---------|--------|
| `BottomNav.tsx` | `src/components/BottomNav.tsx` |

### 2.2 Positionnement

| Propriété | Classe Tailwind | Valeur CSS | Description |
|-----------|-----------------|------------|-------------|
| Position | `fixed` | `position: fixed` | Fixé dans le viewport |
| Bottom | `bottom-0` | `bottom: 0` | Collé au bord inférieur |
| Left | `left-0` | `left: 0` | Depuis le bord gauche |
| Right | `right-0` | `right: 0` | Jusqu'au bord droit |
| Z-index | `z-50` | `z-index: 50` | Au-dessus de tout le contenu |

```tsx
<nav className="fixed bottom-0 left-0 right-0 z-50 bg-black border-t border-gray-800">
```

### 2.3 Apparence visuelle

| Propriété | Classe | Valeur | Description |
|-----------|--------|--------|-------------|
| Background | `bg-black` | `#000000` | Fond noir opaque |
| Border top | `border-t border-gray-800` | 1px solid #1f2937 | Séparation avec le contenu |

### 2.4 Dimensions et espacement

| Élément | Propriété | Valeur | Description |
|---------|-----------|--------|-------------|
| Hauteur totale | Calculée | ~60-70px | Dépend du contenu |
| Padding vertical item | `py-3` | 12px | Espacement haut/bas |
| Padding horizontal item | `px-3` | 12px | Espacement gauche/droite |
| Largeur min item | `min-w-[64px]` | 64px | Largeur minimum par bouton |

### 2.5 Layout interne

```
┌────────────────────────────────────────────────────────────────┐
│  [🏠]     [📚]      [👥]      [📖]      [📄]      [⚙️]       │
│  Accueil  Éditions  Exposants  Annuaire  Documents  Paramètres │
│                                                                │
│  ← ─ ─ ─ ─ ─ justify-around ─ ─ ─ ─ ─ →                       │
└────────────────────────────────────────────────────────────────┘
```

| Propriété | Classe | Description |
|-----------|--------|-------------|
| Display | `flex` | Conteneur flex |
| Alignement items | `items-center` | Centrage vertical |
| Distribution | `justify-around` | Espacement égal autour des items |

### 2.6 Items de navigation

#### Structure d'un item

```tsx
<Link className="relative py-3 px-3 flex flex-col items-center justify-center min-w-[64px]">
  {/* Indicateur actif (barre supérieure) */}
  <motion.div className="absolute top-0 left-0 right-0 h-0.5 bg-catakana-purple" />
  
  {/* Icône */}
  <div className="touch-target">
    <Icon size={24} />
  </div>
  
  {/* Label */}
  <span className="text-xs mt-1">{label}</span>
</Link>
```

#### Taille des icônes

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| Size | `24` | 24×24px (Lucide icons) |
| Container | `.touch-target` | min-h-12 min-w-12 (48×48px) |

#### Typographie du label

| Propriété | Classe | Valeur | Description |
|-----------|--------|--------|-------------|
| Taille | `text-xs` | 12px / 0.75rem | Petit texte |
| Marge top | `mt-1` | 4px | Espacement avec l'icône |
| Font-weight | Default | 400 | Normal |

### 2.7 États visuels

#### État inactif

| Propriété | Classe | Valeur |
|-----------|--------|--------|
| Couleur texte/icône | `text-gray-400` | `#9ca3af` |

#### État actif

| Propriété | Classe | Valeur | Description |
|-----------|--------|--------|-------------|
| Couleur texte/icône | `text-catakana-purple` | `#9b87f5` | Violet Catakana |
| Indicateur | `.h-0.5 bg-catakana-purple` | 2px height | Barre horizontale animée |

### 2.8 Animation (Framer Motion)

```tsx
<motion.div
  layoutId="activeBubble"           // ID pour animation partagée
  className="absolute top-0 left-0 right-0 h-0.5 bg-catakana-purple"
  initial={{ opacity: 0 }}
  animate={{ opacity: 1 }}
  transition={{ duration: 0.2 }}    // 200ms
/>
```

| Propriété | Valeur | Description |
|-----------|--------|-------------|
| `layoutId` | `"activeBubble"` | Animation de déplacement fluide entre items |
| Durée | `0.2s` | Transition rapide |
| Effet | Fade in | Apparition progressive |

### 2.9 Filtrage par rôle utilisateur

Les items affichés dépendent du rôle (`UserRole`) :

| Item | Roles autorisés |
|------|-----------------|
| Accueil | Tous |
| Éditions | admin, manager, exhibitor |
| Exposants | admin, manager |
| Annuaire | Tous |
| Documents | admin, manager |
| Budget | admin, manager |
| Compte | exhibitor uniquement |
| Paramètres | Tous |

---

## 3. Breakpoints Tailwind (Référence)

| Nom | Largeur min | Usage typique |
|-----|-------------|---------------|
| `sm` | 640px | Tablettes portrait |
| `md` | 768px | Tablettes paysage |
| `lg` | 1024px | Laptops |
| `xl` | 1280px | Desktop |
| `2xl` | 1536px | Large desktop |
| `3xl` | 1920px | Full HD |
| `4xl` | 2560px | 4K |

---

## 4. Palette de couleurs Catakana

| Nom | Variable | Hex | Usage |
|-----|----------|-----|-------|
| Purple | `catakana-purple` | `#9b87f5` | Accent principal, items actifs |
| Purple Dark | `catakana-purple-dark` | `#7E69AB` | Hover states |
| Blue | `catakana-blue` | `#1EAEDB` | Liens, hover logo |
| Blue Light | `catakana-blue-light` | `#0EA5E9` | Variante claire |
| Gray | `catakana-gray` | `#403E43` | Texte secondaire |
| Gray Dark | `catakana-gray-dark` | `#222222` | Fonds sombres |
| Gray Light | `catakana-gray-light` | `#8A898C` | Texte muted |

---

## 5. Classes utilitaires personnalisées

### 5.1 Touch target (Accessibilité mobile)

```css
.touch-target {
  @apply min-h-12 min-w-12 flex items-center justify-center;
}
```

- **Objectif** : Zone tactile minimum de 48×48px (recommandation WCAG)
- **Usage** : Wrapper autour des icônes cliquables

### 5.2 Glass card (Effet glassmorphism)

```css
.glass-card {
  @apply backdrop-blur-xl bg-black/40 border border-white/10 shadow-lg;
}
```

### 5.3 Neon border (Effet néon)

```css
.neon-border {
  @apply border border-catakana-purple shadow-[0_0_8px_rgba(155,135,245,0.6)];
}
```

---

## 6. Recommandations d'implémentation

### 6.1 Padding bottom pour le contenu

Le `BottomNav` étant fixé, le contenu principal doit avoir un `padding-bottom` suffisant :

```tsx
<main className="pb-20">  {/* 80px minimum */}
  {/* Contenu */}
</main>
```

### 6.2 Padding top pour le Header

Le `Header` étant fixé avec `h-16` (64px) :

```tsx
<main className="pt-16">  {/* 64px minimum */}
  {/* Contenu */}
</main>
```

### 6.3 Safe area (iOS)

Pour les appareils avec encoche ou barre de navigation système :

```css
.safe-area-bottom {
  padding-bottom: env(safe-area-inset-bottom);
}
```

---

## 7. Schéma récapitulatif

```
┌────────────────────────────────────────────────────────────────┐
│ HEADER (fixed top-0, z-20, h-16)                               │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │ [🏮 Logo]  [Nav Desktop hidden md:flex]    [Actions] [☰]   │ │
│ └────────────────────────────────────────────────────────────┘ │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│                    CONTENU PRINCIPAL                           │
│                    (pt-16 pb-20)                               │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│ BOTTOMNAV (fixed bottom-0, z-50, ~60-70px)                     │
│ ┌────────────────────────────────────────────────────────────┐ │
│ │   [Icon]    [Icon]    [Icon]    [Icon]    [Icon]    [Icon] │ │
│ │   text-xs   text-xs   text-xs   text-xs   text-xs   text-xs│ │
│ │   ─────── indicateur actif (motion, 2px, purple) ───────── │ │
│ └────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────┘
```

---

## 8. Changelog

| Date | Version | Modification |
|------|---------|--------------|
| 2026-01-09 | 1.0 | Création initiale de la documentation |
