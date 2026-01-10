Miyukini Framework - Atomic Design & Theme Dynamique

## Contexte

Ce document formalise l’UI Kit du Miyukini Framework avec les principes d’**Atomic Design** et les règles strictes d’utilisation du **thème dynamique**. Il sert de guide pour chaque développeur IA ou humain qui construit un composant sur la plateforme : l’objectif est d’assurer une cohérence visuelle, une maintenance facilitée et une conformité totale avec la stratégie thématique en cours.

## Portée / Scope

Appliquez ce guide à chaque nouveau composant dans `src/components` (atoms → pages), ainsi qu’aux pages existantes qui subissent des refontes graphiques. Il décrit :

- les interdictions absolues (couleurs hardcodées, styles inline, composants sans thème),
- la hiérarchie Atomic design imposée pour structurer les composants,
- les helpers et patterns thématiques obligatoires,
- la checklist de validation pré-commit,
- les exemples types permettant d’appliquer la règle.


## 1. Interdictions strictes

### 1.1 Couleurs

- ❌ **Couleurs** hex, rgb, rgba, ou classes Tailwind colorées inline sont interdites.
- ✅ Toujours déduire la couleur via `useActiveTheme()` et les tokens : `theme.colors.section.*`, `theme.colors.text.*`.
- Les tokens peuvent être transformés en `rgba` via `colorToRgba`.

### 1.2 Styles inline

- ❌ Aucun style inline avec valeur fixe (`color: '#fff'`, etc.).
- ✅ Seuls des objets de style dérivés du thème (`textPrimaryStyle`, `cardStyle`, etc.) sont autorisés.

### 1.3 Composants non-thémés

- ❌ Les UI components (Button, Card, Input, etc.) doivent impérativement utiliser le thème actif.
- ✅ Les composants sans accès au thème n’existent pas : chaque UI component doit importer `useActiveTheme` (ou la recevoir en prop).


## 2. Atomisation obligatoire

Structure hiérarchique :

```
atoms/          → composants de base (Button, Input, Badge, Icon)
molecules/      → composants composés (Card, FormField, SearchBar)
organisms/      → sections complexes (UserList, EventCard, InvoiceTable)
templates/      → layouts (AdminLayout, PublicLayout)
pages/          → pages complètes (HomePage, AdminPage)
```

- Les **atoms** sont 100 % réutilisables et consomment uniquement les tokens du thème.
- Les **molecules** combinent des atoms sans styles hardcodés.
- Les **organisms** joignent molecules/atoms et respectent les règles d’opacité et de thème.
- Les **templates** définissent les grilles/layouts via des organismes thémés.
- Les **pages** assemblent les templates et les organisms sans styles inline.


## 3. Thème dynamique : import & patterns obligatoires

### 3.1 Imports standards

```
import { useActiveTheme } from '@/components/hooks/useAdminConfig'
import { colorToRgba, getModalOverlayStyle } from '@/lib/utils/themeUtils'
```

### 3.2 Helpers obligatoires

```
const theme = useActiveTheme()

const textPrimaryStyle = {
  color: theme?.colors?.section?.title || theme?.colors?.text?.primary || '#ffffff'
}

const textSecondaryStyle = {
  color: theme?.colors?.section?.description || theme?.colors?.text?.secondary || '#ffffff'
}

const cardStyle = {
  background: colorToRgba(
    theme?.colors?.section?.card?.background || theme?.colors?.section?.background,
    0.1
  ),
  borderColor: theme?.colors?.section?.border || theme?.colors?.section?.card?.border
}

const buttonStyle = {
  background: colorToRgba(
    theme?.colors?.section?.card?.background || theme?.colors?.section?.background,
    0.1
  ),
  borderColor: theme?.colors?.section?.border || theme?.colors?.section?.card?.border,
  color: theme?.colors?.text?.primary || '#ffffff'
}
```

### 3.3 Patterns obligatoires

- **Textes** : `textPrimaryStyle` / `textSecondaryStyle` obligatoires.
- **Backgrounds** : `colorToRgba` avec `section.background` ou `section.card.background` (opacité 0.05-0.4 selon contexte).
- **Bordures** : `theme.colors.section.border` ou `section.card.border`.
- **Overlays modales** : `style={getModalOverlayStyle(theme, 0.5)}`.
- **Inputs / Selects / Textareas** : `colorToRgba` + tokens et `color` fallback.
- **Boutons** : `className="transition-colors"`, `style={buttonStyle}` et hover via `onMouseEnter` / `onMouseLeave`.

### 3.4 Opacités

- Fonds principaux (body, sections) : `colorToRgba(..., 0.4)`
- Cartes :
  - discrètes : `colorToRgba(..., 0.05)`
  - normales : `colorToRgba(..., 0.1)`
- Hover : opacité élevée (ex. `0.2` au lieu de `0.1`).
- Overlays : `getModalOverlayStyle(theme, 0.5)`


## 4. Structure de fichiers requise

```
src/
  components/
    atoms/
      ui/
      icons/
    molecules/
      cards/
      forms/
    organisms/
      sections/
      modals/
    templates/
    pages/
```

- Vérifier que chaque composant réside dans le dossier atomique correspondant.
- Les composants Shadcn/ui vivront sous `atoms/ui/`.
- Les organismes complexes (ex : modals) restent dans `organisms/modals/`.


## 5. Helpers disponible

- `colorToRgba(color, opacity)` — `src/lib/utils/themeUtils.ts`
- `getModalOverlayStyle(theme, opacity?)` — `src/lib/utils/themeUtils.ts`
- `useActiveTheme()` — `src/components/hooks/useAdminConfig.ts`


## 6. Checklist avant commit

- ✅ `useActiveTheme()` importé si composant graphique.
- ✅ Pas de couleur hardcodée (hex, rgb, rgba, classes Tailwind colorées).
- ✅ Textes utilisent `textPrimaryStyle` / `textSecondaryStyle`.
- ✅ Backgrounds passent par `colorToRgba` + tokens.
- ✅ Bordures utilisent `theme.colors.section.border`.
- ✅ Overlays passent par `getModalOverlayStyle()`.
- ✅ Boutons avec hover gérés via `onMouseEnter`/`onMouseLeave`.
- ✅ FallBacks présents (`|| theme?.colors?.text?.primary || '#ffffff'`).
- ✅ Hiérarchie Atomic respectée.
- ✅ Placement dossier correct (atoms/molecules/organisms...).
- ✅ Aucun lint error détecté après modification.


## 7. Exemples complets

### 7.1 Composant Atom : `ThemedButton`

```
'use client'

import { useActiveTheme } from '@/components/hooks/useAdminConfig'
import { colorToRgba } from '@/lib/utils/themeUtils'
import { Button } from '@/components/ui/button'

export function ThemedButton({ children, ...props }) {
  const theme = useActiveTheme()

  const buttonStyle = {
    background: colorToRgba(
      theme?.colors?.section?.card?.background || theme?.colors?.section?.background,
      0.1
    ),
    borderColor: theme?.colors?.section?.border || theme?.colors?.section?.card?.border,
    color: theme?.colors?.text?.primary || '#ffffff'
  }

  return (
    <Button
      {...props}
      className="transition-colors"
      style={buttonStyle}
      onMouseEnter={(e) => {
        e.currentTarget.style.background = colorToRgba(
          theme?.colors?.section?.card?.background || theme?.colors?.section?.background,
          0.2
        )
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.background = colorToRgba(
          theme?.colors?.section?.card?.background || theme?.colors?.section?.background,
          0.1
        )
      }}
    >
      {children}
    </Button>
  )
}
```

### 7.2 Composant Molecule : `ThemedCard`

```
'use client'

import { useActiveTheme } from '@/components/hooks/useAdminConfig'
import { colorToRgba } from '@/lib/utils/themeUtils'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

export function ThemedCard({ title, children }) {
  const theme = useActiveTheme()

  const cardStyle = {
    background: colorToRgba(
      theme?.colors?.section?.card?.background || theme?.colors?.section?.background,
      0.1
    ),
    borderColor: theme?.colors?.section?.border || theme?.colors?.section?.card?.border
  }

  const textPrimaryStyle = {
    color: theme?.colors?.section?.title || theme?.colors?.text?.primary
  }

  return (
    <Card style={cardStyle}>
      <CardHeader>
        <CardTitle style={textPrimaryStyle}>{title}</CardTitle>
      </CardHeader>
      <CardContent>
        {children}
      </CardContent>
    </Card>
  )
}
```


## 8. Validation & refus

Si un code viole ces règles :

1. Refuser immédiatement la proposition.
2. Demander correction en expliquant la règle non respectée.
3. Proposer le pattern (`textPrimaryStyle`, helpers de thème) comme solution.
4. Confirmer la non-négociabilité de la règle avant validation finale.

> Une page livre, un écran conçoit : chaque document doit guider une décision.
