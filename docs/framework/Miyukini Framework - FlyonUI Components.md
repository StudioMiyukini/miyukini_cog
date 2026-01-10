# Miyukini Framework - FlyonUI Components

## Contexte

Ce document catalogue les composants FlyonUI intégrés au Miyukini Framework, extraits du template `flyonui-dashboard-free`. Ces composants suivent l'Atomic Design et sont prêts pour la réutilisation dans tous les modules (Front, Back Office, Super Admin).

## Portée / Scope

- Composants FlyonUI intégrés en React/TypeScript
- Thème personnalisé `miyukini` avec couleurs du framework
- Icônes via `@iconify-json/tabler`
- Compatible avec le système de theming dynamique existant

---

## 1. Atoms (Composants de base)

### Badge

**Chemin**: `src/components/atoms/badge/Badge.tsx`

| Prop | Type | Description |
|------|------|-------------|
| `variant` | `'primary' \| 'success' \| 'error' \| 'warning' \| 'info' \| 'neutral'` | Couleur du badge |
| `style` | `'solid' \| 'soft' \| 'outline'` | Style visuel |
| `size` | `'xs' \| 'sm' \| 'md' \| 'lg'` | Taille |
| `rounded` | `boolean` | Coins arrondis |
| `icon` | `ReactNode` | Icône optionnelle |

```tsx
import { Badge } from '@/components/atoms/badge'

<Badge variant="success" style="soft">Active</Badge>
<Badge variant="error" size="sm">Rejected</Badge>
```

---

### Avatar

**Chemin**: `src/components/atoms/avatar/Avatar.tsx`

| Prop | Type | Description |
|------|------|-------------|
| `src` | `string` | URL de l'image |
| `size` | `'xs' \| 'sm' \| 'md' \| 'lg' \| 'xl'` | Taille |
| `status` | `'online' \| 'offline' \| 'away' \| 'busy' \| 'none'` | Indicateur de statut |
| `placeholder` | `ReactNode` | Contenu si pas d'image |
| `rounded` | `'full' \| 'field'` | Forme |

```tsx
import { Avatar } from '@/components/atoms/avatar'

<Avatar src="/avatar.png" size="md" status="online" />
<Avatar placeholder={<span>JD</span>} size="lg" />
```

---

### Card

**Chemin**: `src/components/atoms/card/Card.tsx`

Composants: `Card`, `CardHeader`, `CardBody`, `CardFooter`

```tsx
import { Card, CardHeader, CardBody, CardFooter } from '@/components/atoms/card'

<Card>
  <CardHeader>Titre</CardHeader>
  <CardBody>Contenu de la carte</CardBody>
  <CardFooter>Actions</CardFooter>
</Card>
```

---

### StatsCard

**Chemin**: `src/components/atoms/stats-card/StatsCard.tsx`

| Prop | Type | Description |
|------|------|-------------|
| `title` | `string` | Titre de la statistique |
| `value` | `string \| number` | Valeur principale |
| `icon` | `ReactNode` | Icône représentative |
| `trend` | `{ value: number, direction: 'up' \| 'down' \| 'neutral' }` | Tendance |
| `subtitle` | `string` | Information supplémentaire |

```tsx
import { StatsCard } from '@/components/atoms/stats-card'

<StatsCard 
  title="Pageviews" 
  value="17,356" 
  icon={<span className="icon-[tabler--eye] size-5" />}
  trend={{ value: 25.6, direction: 'up' }}
  subtitle="EPC: 308.20"
/>
```

---

## 2. Molecules (Composants composés)

### Accordion

**Chemin**: `src/components/molecules/accordion/Accordion.tsx`

| Prop | Type | Description |
|------|------|-------------|
| `items` | `AccordionItem[]` | Liste des items |
| `allowMultiple` | `boolean` | Permettre plusieurs items ouverts |

```tsx
import { Accordion } from '@/components/molecules/accordion'

<Accordion 
  items={[
    { title: 'Question 1', content: 'Réponse 1', defaultOpen: true },
    { title: 'Question 2', content: 'Réponse 2' },
  ]} 
/>
```

---

### DataTable

**Chemin**: `src/components/molecules/data-table/DataTable.tsx`

| Prop | Type | Description |
|------|------|-------------|
| `data` | `T[]` | Données du tableau |
| `columns` | `TableColumn<T>[]` | Configuration des colonnes |
| `actions` | `TableAction[]` | Actions par ligne |

```tsx
import { DataTable, StatusBadge } from '@/components/molecules/data-table'

<DataTable 
  data={users}
  columns={[
    { key: 'name', header: 'Nom' },
    { key: 'email', header: 'Email' },
    { key: 'status', header: 'Statut', render: (item) => <StatusBadge status={item.status} /> }
  ]}
  actions={[
    { icon: 'icon-[tabler--pencil]', label: 'Modifier', onClick: (item) => edit(item) },
    { icon: 'icon-[tabler--trash]', label: 'Supprimer', onClick: (item) => delete(item) }
  ]}
/>
```

---

### MeetingList

**Chemin**: `src/components/molecules/meeting-list/MeetingList.tsx`

```tsx
import { MeetingList } from '@/components/molecules/meeting-list'

<MeetingList 
  meetings={[
    { 
      id: '1', 
      title: 'Call with John', 
      date: '1 Jul', 
      time: '08:20-10:20',
      avatar: '/avatars/1.png',
      category: { label: 'Business', variant: 'primary' }
    }
  ]}
  onMeetingClick={(meeting) => console.log(meeting)}
/>
```

---

### ProfileDropdown

**Chemin**: `src/components/molecules/profile-dropdown/ProfileDropdown.tsx`

```tsx
import { ProfileDropdown } from '@/components/molecules/profile-dropdown'

<ProfileDropdown 
  user={{ 
    name: 'John Doe', 
    email: 'john@example.com',
    role: 'Admin',
    avatar: '/avatar.png' 
  }}
  menuItems={[
    { icon: 'icon-[tabler--user]', label: 'Mon compte', href: '/account' },
    { icon: 'icon-[tabler--settings]', label: 'Paramètres', href: '/settings' },
    { divider: true },
    { icon: 'icon-[tabler--logout]', label: 'Déconnexion', onClick: () => logout(), danger: true }
  ]}
/>
```

---

## 3. Organisms (Composants complexes)

### Sidebar

**Chemin**: `src/components/organisms/sidebar/Sidebar.tsx`

Navigation latérale avec accordion, profil utilisateur et sections.

```tsx
import { Sidebar } from '@/components/organisms/sidebar'

<Sidebar 
  profile={{ 
    name: 'John Doe', 
    email: 'john@example.com',
    avatar: '/avatar.png',
    socialLinks: [
      { icon: 'icon-[tabler--brand-github]', href: 'https://github.com', label: 'GitHub' }
    ]
  }}
  sections={[
    { 
      label: 'Main',
      items: [
        { 
          id: 'dashboard', 
          icon: 'icon-[tabler--dashboard]', 
          label: 'Dashboard',
          active: true,
          children: [
            { id: 'analytics', label: 'Analytics', href: '/analytics' },
            { id: 'reports', label: 'Reports', href: '/reports' }
          ]
        }
      ]
    },
    {
      label: 'Settings',
      items: [
        { id: 'settings', icon: 'icon-[tabler--settings]', label: 'Settings', href: '/settings' }
      ]
    }
  ]}
  onNavigate={(href) => router.push(href)}
/>
```

---

## 4. Configuration Tailwind + FlyonUI

### tailwind.config.js

```js
module.exports = {
  content: [
    './src/**/*.{js,ts,jsx,tsx,mdx}',
    './node_modules/flyonui/dist/js/*.js',
  ],
  plugins: [
    require('flyonui'),
    require('flyonui/plugin'),
    require('@iconify/tailwind4').default(),
  ],
  flyonui: {
    themes: ['dark', 'light', {
      miyukini: {
        'primary': '#6366f1',
        'secondary': '#818cf8',
        'accent': '#5eead4',
        'neutral': '#1f2937',
        'base-100': '#05070d',
        'base-200': '#0f172a',
        'base-300': '#1e293b',
        'base-content': '#f8fafc',
      }
    }]
  }
}
```

---

## 5. Provider FlyonUI

### FlyonUIProvider

**Chemin**: `src/components/providers/FlyonUIProvider.tsx`

Wrapper obligatoire pour initialiser FlyonUI JS (composants interactifs).

```tsx
// src/app/layout.tsx
import { FlyonUIProvider } from '@/components/providers/FlyonUIProvider'

export default function RootLayout({ children }) {
  return (
    <html lang="fr" data-theme="miyukini">
      <body>
        <FlyonUIProvider>
          {children}
        </FlyonUIProvider>
      </body>
    </html>
  )
}
```

---

## 6. Icônes Tabler

Collection d'icônes via `@iconify-json/tabler`. Format: `icon-[tabler--nom-icone]`

### Exemples courants

| Icône | Classe |
|-------|--------|
| User | `icon-[tabler--user]` |
| Settings | `icon-[tabler--settings]` |
| Dashboard | `icon-[tabler--dashboard]` |
| Calendar | `icon-[tabler--calendar]` |
| Search | `icon-[tabler--search]` |
| Plus | `icon-[tabler--plus]` |
| Minus | `icon-[tabler--minus]` |
| Chevron Right | `icon-[tabler--chevron-right]` |
| Arrow Up | `icon-[tabler--arrow-up]` |
| Arrow Down | `icon-[tabler--arrow-down]` |
| Trash | `icon-[tabler--trash]` |
| Pencil | `icon-[tabler--pencil]` |
| Logout | `icon-[tabler--logout]` |

```tsx
<span className="icon-[tabler--user] size-5" />
<span className="icon-[tabler--settings] size-6 text-primary" />
```

---

## 7. Classes FlyonUI utiles

### Boutons

```html
<button class="btn btn-primary">Primary</button>
<button class="btn btn-secondary btn-soft">Secondary Soft</button>
<button class="btn btn-error btn-outline">Error Outline</button>
<button class="btn btn-circle btn-sm">●</button>
```

### Badges

```html
<span class="badge badge-success badge-soft">Success</span>
<span class="badge badge-error badge-sm">Error</span>
```

### Cards

```html
<div class="card shadow-md">
  <div class="card-header">Header</div>
  <div class="card-body">Body</div>
</div>
```

### Dropdown

```html
<div class="dropdown">
  <button class="dropdown-toggle">Menu</button>
  <ul class="dropdown-menu">
    <li><a class="dropdown-item">Item 1</a></li>
  </ul>
</div>
```

---

## 8. Changelog

| Date | Version | Modification |
|------|---------|--------------|
| 2026-01-10 | 1.0 | Extraction et intégration des composants FlyonUI Dashboard Free |
