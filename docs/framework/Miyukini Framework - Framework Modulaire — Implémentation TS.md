Miyukini Framework Modulaire — Implémentation TypeScript

> ⚠️ DOCUMENT TECHNIQUE D'IMPLÉMENTATION  
> Ce document décrit l'implémentation concrète du Framework Modulaire en TypeScript/React.  
> Complète le document "Framework Modulaire (Extension & Scalabilité)".

---

## 0. Vision mobile-first & écrans adaptatifs

### 0.1 Principes mobiles

- Le Framework Modulaire priorise les écrans mobiles : toutes les vues sont conçues pour un viewport ≤ 480px, puis enrichies par des breakpoints tablets (768px) et desktop (1200px) sans modifier l’ordre logique des sections.
- Chaque écran est pensé comme une "carte" complète qui se déroule du haut vers le bas, de la barre supérieure à la barre de navigation inférieure, avec des marges et espacement alignés sur les tokens (cartes 5‑10 % d’opacité, sections 40 % d’opacité, etc.).
- Les styles utilisent `useActiveTheme()`, `colorToRgba()` et `getModalOverlayStyle()` pour garantir l’adaptation aux thèmes sans jamais hardcoder de couleurs ou de mesures fixes.
- Le layout est responsive grâce à des helpers (`useResponsiveLayout`, `useWindowSize`) qui réorganisent les organismes quand l’espace horizontal augmente : colonnes, panneaux collables, modales full-screen → card-layout.

### 0.2 Écrans et parcours

- Chaque module expose ses écrans par `ModuleContract.routes()` ; chaque écran est décrit par :
  - un **header** (menu, titre, actions globales)
  - un **body** (contenu principal, listes, formulaires, graphiques)
  - un **bottom** (navigation locale, CTA, FAB, tab-bar)
- Les écrans mobiles utilisent un seul flux vertical ; les versions desktop redistribuent body en colonnes ou panneaux selon les organes disponibles.
- Tous les écrans documentent leur FAB, leur toast/alert, les skeleton loaders mobiles, et s’appuient sur les organismes listés dans ce document pour assurer la cohérence.

## 1. Architecture générale

### 1.1 Stack technique
- **Frontend** : React 18 + TypeScript strict
- **Routing** : React Router v7
- **État** : TanStack Query + Zustand
- **Offline** : IndexedDB via Dexie
- **Backend** : Supabase (PostgreSQL + RLS)

### 1.2 Structure de base
```
src/
├── core/                    # Code partagé
│   ├── modules/            # Système modulaire
│   │   ├── ModuleContract.ts      # Interface de contrat
│   │   ├── ModuleRegistry.ts      # Registre des modules
│   │   └── index.ts               # Enregistrement des modules
│   ├── events/             # Event Bus
│   │   ├── EventBus.ts            # Implémentation Event Bus
│   │   └── events.ts              # Définitions d'événements
│   ├── ui/                 # UI Kit (Atomic Design)
│   │   ├── atoms/          # Composants de base
│   │   ├── molecules/      # Composants composés
│   │   └── organisms/      # Sections complexes
│   └── services/          # Services partagés
├── features/               # Modules métier
│   ├── todo/
│   ├── shopping/
│   ├── budget/
│   └── medical/
└── app/                    # Configuration app
    └── router/            # Routeur principal
```

---

## 2. Système de contrats modulaires

### 2.1 Interface ModuleContract

Chaque module DOIT implémenter `ModuleContract` :

```typescript
interface ModuleContract {
  readonly name: ModuleName
  routes(): ModuleRoute[]
  permissions(): ModulePermission[]
  isEnabled(): boolean
  initialize?(): Promise<void>
  cleanup?(): Promise<void>
}
```

### 2.2 Interface du framework central

`ModuleRegistry` (dans `src/core/modules/ModuleRegistry.ts`) expose l’unique porte d’entrée pour tous les modules :
- `register(module: ModuleContract)`
- `get(name: string)` / `getAllEnabled()` pour construire dynamiquement la navigation
- `initializeAll()` et `cleanupAll()` utilisés depuis `App.tsx` pour piloter le cycle de vie
- Garantit l’isolation (pas d’accès direct entre modules) tout en permettant de découvrir les routes/permissions via leur `ModuleContract`

### 2.3 Exemple d'implémentation

```typescript
// features/todo/api/TodoContract.ts
export class TodoContract implements ModuleContract {
  readonly name = 'todo' as const

  routes(): ModuleRoute[] {
    return [
      { path: '/todo', name: 'TodoList', component: TodoListScreen },
      { path: '/todo/:id', name: 'TodoDetail', component: TodoDetailScreen },
    ]
  }

  permissions(): ModulePermission[] {
    return [
      { name: 'todo:read', description: 'Lire les tâches' },
      { name: 'todo:write', description: 'Créer et modifier les tâches' },
    ]
  }

  isEnabled(): boolean {
    return true
  }
}
```

### 2.3 Enregistrement des modules

Tous les modules sont enregistrés dans `src/core/modules/index.ts` :

```typescript
export function registerAllModules(): void {
  moduleRegistry.register(new TodoContract())
  moduleRegistry.register(new ShoppingContract())
  moduleRegistry.register(new BudgetContract())
  moduleRegistry.register(new MedicalContract())
}
```

L'enregistrement se fait au démarrage de l'application dans `App.tsx`.

---

## 3. Event Bus central

### 3.1 Implémentation

L'Event Bus est un singleton qui permet la communication inter-modules :

```typescript
// core/events/EventBus.ts
class EventBus {
  on<T>(eventType: string, handler: EventHandler<T>): () => void
  emit<T>(eventType: string, event: T): Promise<void>
  once<T>(eventType: string, handler: EventHandler<T>): void
  off(eventType: string): void
  clear(): void
}

export const eventBus = new EventBus()
```

### 3.2 Définition des événements

Tous les événements sont définis dans `core/events/events.ts` :

```typescript
export interface ShoppingItemBoughtEvent {
  eventId: string
  occurredAt: string
  itemId: string
  userId: string
  listId: string
  price: number
}

export const EventTypes = {
  SHOPPING_ITEM_BOUGHT: 'shopping:item:bought',
  // ...
} as const
```

### 3.3 Utilisation

**Émission d'un événement** :
```typescript
import { eventBus } from '@/core/events/EventBus'
import { EventTypes, type ShoppingItemBoughtEvent } from '@/core/events/events'

await eventBus.emit(EventTypes.SHOPPING_ITEM_BOUGHT, {
  eventId: generateId(),
  occurredAt: new Date().toISOString(),
  itemId: '...',
  userId: '...',
  listId: '...',
  price: 10.50,
})
```

**Écoute d'un événement** :
```typescript
useEffect(() => {
  const unsubscribe = eventBus.on<ShoppingItemBoughtEvent>(
    EventTypes.SHOPPING_ITEM_BOUGHT,
    async (event) => {
      // Créer une transaction budget automatiquement
      await createTransactionFromPurchase(event)
    }
  )

  return unsubscribe
}, [])
```

---

## 4. Format des modules

### 4.1 Structure de dossiers (OBLIGATOIRE)

```
features/<module_name>/
├── api/
│   └── <Module>Contract.ts          # Contrat du module
├── ui/
│   ├── screens/                     # Écrans du module
│   ├── components/                  # Composants spécifiques au module
│   └── <Module>NavGraph.tsx         # (Optionnel) NavGraph React Router
├── domain/
│   ├── model/                       # Modèles domain
│   ├── usecase/                     # Cas d'usage
│   └── repository/                  # Interface repository
├── data/
│   ├── local/                       # Data source locale (Dexie)
│   │   └── <Module>LocalDataSource.ts
│   ├── remote/                      # Data source distante (Supabase)
│   │   └── <Module>RemoteDataSource.ts
│   └── <Module>RepositoryImpl.ts    # Implémentation repository
└── di/                              # (Optionnel) Dependency Injection
```

### 4.2 Exemple complet : Module Todo

**Modèle domain** (`domain/model/TodoTask.ts`) :
```typescript
export interface TodoTask extends BaseEntity {
  title: string
  description?: string
  status: TodoTaskStatus
  priority?: number
  dueDate?: string
  deadline?: string
  tags?: string[]
  budgetId?: string
  childProfileId?: string
  googleEventId?: string
  userId: string
}
```

**Repository interface** (`domain/repository/TodoRepository.ts`) :
```typescript
export interface TodoRepository {
  getAll(userId: string): Promise<TodoTask[]>
  getById(id: string): Promise<TodoTask | null>
  create(task: Omit<TodoTask, 'id' | 'createdAt' | 'updatedAt'>): Promise<TodoTask>
  update(id: string, updates: Partial<TodoTask>): Promise<TodoTask>
  delete(id: string): Promise<void>
  // ...
}
```

**Repository implémentation** (`data/TodoRepositoryImpl.ts`) :
```typescript
export class TodoRepositoryImpl implements TodoRepository {
  private localDataSource = new TodoLocalDataSource()
  private remoteDataSource = new TodoRemoteDataSource()

  async getAll(userId: string): Promise<TodoTask[]> {
    // Offline-first : retourner d'abord les données locales
    const localData = await this.localDataSource.getAll?.(userId) || []
    
    // Sync en arrière-plan
    this.sync().catch(console.error)
    
    return localData
  }

  async sync(): Promise<void> {
    // Synchronisation bidirectionnelle
    await this.remoteDataSource.sync?.()
  }
}
```

---

## 5. Navigation modulaire

### 5.1 Routeur principal

Le routeur principal (`app/router/AppRouter.tsx`) agrège toutes les routes des modules :

```typescript
export function AppRouter() {
  const enabledModules = moduleRegistry.getAllEnabled()
  const allRoutes = enabledModules.flatMap((module) => module.routes())

  return (
    <>
      <Routes>
        <Route path="/" element={<Navigate to="/todo" replace />} />
        {allRoutes.map((route) => (
          <Route key={route.path} path={route.path} element={<route.component />} />
        ))}
      </Routes>
      <BottomNavigation />
    </>
  )
}
```

### 5.2 Navigation bottom

La navigation bottom (`core/ui/organisms/BottomNavigation.tsx`) affiche exactement 4 onglets :
- Todo
- Courses (shopping)
- Budget
- Médical

---

## 6. UI Kit (Atomic Design)

### 6.0 Utilisation transverse

- Importer toujours depuis `src/core/ui/index.ts` pour garantir la cohérence : `import { Button, Card, List } from '@/core/ui'`.
- Tout composant visuel doit combiner atoms/molecules/organisms, jamais de styles inline fixes ni de couleurs hardcodées.
- Les hooks UI (`useActiveTheme()`) sont obligatoires dès qu’un composant lit le thème ; aucun composant ne doit manipuler directement les tokens.
- Les modules documentent l’interaction principale, la FAB et les organismes utilisés (List, FilterBar, Toast, etc.).
- La navigation reste modulée via le `ModuleRegistry` et la bottom navigation : aucun autre composant n’interagit avec la barre globale.

### 6.1 Atoms

Composants de base réutilisables dans `core/ui/atoms/ui/` :
- `Button.tsx` - Bouton avec variantes (primary, secondary, tertiary)
- `Card.tsx` - Carte avec style dynamique
- `Input.tsx` - Champ de saisie avec label et erreur

**Règles strictes** :
- ✅ Utilisation obligatoire de `useActiveTheme()`
- ✅ Aucune couleur hardcodée
- ✅ Zone cliquable ≥ 48dp
- ✅ Styles via `themeUtils` helpers

### 6.2 Molecules

Composants composés dans `core/ui/molecules/` :
- `FAB.tsx` - Floating Action Button (1 FAB visible par écran)

### 6.3 Organisms

### 6.4 Catalogue d’organismes

- **AppShellScreen** (organisme layout) : encapsule header, body et bottom pour chaque écran ; applique les styles mobile-first et les transitions adaptatives desktop.
- **BottomNavigation** : barre fixe à 4 onglets, switch sur modules et adaptation tactile / clavier.
- **ModuleToolbar** : header compact avec titre, fil d’Ariane et actions globales (menu, notifications, filtre rapide).
- **ContentStack** : body scrollable vertical qui s’appuie sur des listes et cards responsives ; devient grille (2-3 colonnes) au-delà de 1024px.
- **ActionTray** : bottom fixe avec CTA principal/FAB, état loading et badges de notification, capable de basculer en drawer pour desktop.
- **AdaptivePanel** : panneau latéral qui peut se cacher/collapse en mobile et s’ancre sur desktop pour afficher détails/contextuels.
- **ToastStack** : conteneur fixe en bas de l’écran (mobile) ou en bas à droite (desktop) pour les retours asynchrones.
- **ModuleSkeleton** : fallback plein écran pour l’initialisation module ; reprend les proportions mobile pour éviter les jumps.
- **ModalDialog** : overlay géré via `getModalOverlayStyle(theme, 0.5)` et contenu centré avec padding responsive.
- **FilterSheet** : drawer mobile déroulant depuis le bas, se transforme en sidebar sur desktop (pivote en `AdaptivePanel`).

---

## 7. Gestion des données (Offline-first)

### 7.1 Stratégie

1. **Local d'abord** : Les données sont toujours lues depuis IndexedDB (Dexie)
2. **Sync en arrière-plan** : La synchronisation avec Supabase se fait automatiquement
3. **Résolution de conflits** : Last-write-wins (à améliorer en bêta)

### 7.2 Structure des data sources

**LocalDataSource** (Dexie) :

```typescript
export class TodoLocalDataSource {
  async getAll(userId: string): Promise<TodoTask[]> {
    // Requête IndexedDB
  }
}
```

**RemoteDataSource** (Supabase) :

```typescript
export class TodoRemoteDataSource {
  async getAll(userId: string): Promise<TodoTask[]> {
    // Requête Supabase
  }
}
```

**Repository** (orchestration) :

```typescript
export class TodoRepositoryImpl implements TodoRepository {
  async getAll(userId: string): Promise<TodoTask[]> {
    const local = await this.localDataSource.getAll(userId)
    this.sync().catch(console.error) // Sync en arrière-plan
    return local
  }
}
```

## 8. Écran type

### 8.1 Header / Body / Bottom

- Le **header** reçoit le `ModuleToolbar` avec logo, titre dynamique, accès aux actions globales, fil d’Ariane et indicateurs de statut ; il reste collé en haut sur mobile (sticky) et se déploie en desktop avec sous-header quand nécessaire.
- Le **body** est une `ContentStack` scrollable contenant listes, cards, formulaires, graphiques ; il utilise les tokens d’espacement (32px mobile, 48px desktop) et module les colonnes via `Grid` adaptatif.
- Le **bottom** regroupe la `BottomNavigation`, le `ActionTray` ou le `FAB` selon l’écran ; en mobile, il couvre toute la largeur et se fixe, en desktop il peut se transformer en sidebar contextuelle ou en overlay minimal.
- Les transitions entre header/body/bottom sont gérées via des animations CSS légères (opacity/translate) calibrées pour les interactions tactiles et souris.
- Les états (loading, error, empty) sont visibles sur l’écran complet grâce à `ModuleSkeleton`, `ToastStack` et `ModalDialog` pour rester accessibles sur mobile sans chevauchement.

### 8.2 Adaptation desktop

- Lorsque l’écran dépasse 1024px, les `AdaptivePanel` et `FilterSheet` se positionnent en colonnes, les listes s’ouvrent en tableaux, et les `BottomNavigation` se cachent au profit d’un menu latéral sticky qui conserve l’ordre des écrans mobiles.
- La logique métier (module, permissions, eventBus) reste inchangée ; seules les couches UI pivotent via des hooks responsive. Chaque module décrit son comportement par écran dans `ui/screens/<Module>Screen.tsx`.

---

## 9. Back-end

### 9.1 Principes

- Le backend est entièrement capté par Supabase (PostgreSQL + RLS + Storage + Edge Functions) et expose les tables/modules listés en section 10.
- Les fonctions edge et hooks TanStack Query sont multi-tenants et respectent les policies `module_table_action`. Chaque module consomme son propre `supabaseClient` configuré avec les clés `sb_xxx`.
- Le flux de données est event-driven : les events émis (`eventBus.emit`) déclenchent des triggers Supabase (`pg_notify`) qui, si nécessaire, alimentent des Edge Functions (notifications, sync cross-modules).

### 9.2 Settlement des services

- `DatabaseService` orchestre les sources Dexie et Supabase (cache, sync, conflict handling) pour assurer l’offline-first.
- `NotificationService` (core/services) expose les hooks `usePushNotifications` (via Supabase Realtime) nécessaires aux écrans mobiles pour être notifiés en background.
- `SupabaseService` centralise les helpers (RPC, Storage, Auth) et documente les migrations SQL (`supabase/migrations`) adoptant la nomenclature `module_table_v1`.

---

## 10. Nomenclature SQL & services

### 10.1 Tables et colonnes

- Préfixe par module (`todo_tasks`, `shopping_items`, `budget_transactions`, `medical_profiles`)
- Colonnes communes : `id UUID PRIMARY KEY`, `created_at`, `updated_at`, `deleted_at` (soft delete)
- Relations définies avec `<entity>_id` (`shopping_list_id`, `child_profile_id`, `medical_profile_id`)
- ENUMs obligatoires : `shopping_item_status`, `todo_task_status`, `budget_transaction_type`, `budget_transaction_source`
- RLS policies nommées `<module>_<table>_<verb>` (`shopping_items_select_own`, `budget_transactions_update_owner`)
- Snake_case SQL strict, pas de majuscules ni d’abréviations ambigües

### 10.2 Services & hooks partagés

- `DatabaseService` (IndexedDB via Dexie) dans `src/core/services/DatabaseService.ts`
- `eventBus` (singleton) pour la communication inter-modules
- `queryClient` (TanStack Query) et `supabaseClient` (backend)
- Hooks UI : `useActiveTheme()` et, à terme, `useModuleEvents()` autour de `eventBus`
- Repositories orchestrent les data sources locales + distantes, exposent `sync()` pour résolution de conflits

### 10.3 Nomenclature des hooks/services

- Hooks spécifiques aux modules vivent sous `features/<module>/hooks/`
- Services partagés respectent la convention `XService` (`DatabaseService`, `NotificationService`)
- Modules ne consomment jamais directement les services d’un autre module ; une interaction passe par `ModuleContract` / `eventBus` / l’API distante

---

## 11. Communication inter-modules

### 11.1 Règles strictes

- ❌ **JAMAIS** d'appel direct entre modules
- ✅ **TOUJOURS** via Event Bus
- ✅ Un module ÉMET un événement
- ✅ Un autre module ÉCOUTE l'événement

### 11.2 Exemple : Shopping → Budget

**Module Shopping émet** :

```typescript
// Dans ShoppingItemBoughtScreen.tsx
await eventBus.emit(EventTypes.SHOPPING_ITEM_BOUGHT, {
  eventId: generateId(),
  occurredAt: new Date().toISOString(),
  itemId: item.id,
  userId: currentUser.id,
  listId: list.id,
  price: item.realPrice || item.estimatedPrice,
})
```

**Module Budget écoute** :

```typescript
// Dans BudgetContract.initialize()
useEffect(() => {
  const unsubscribe = eventBus.on<ShoppingItemBoughtEvent>(
    EventTypes.SHOPPING_ITEM_BOUGHT,
    async (event) => {
      await budgetRepository.createTransaction({
        type: 'expense',
        source: 'shopping',
        amount: event.price,
        linkedItemId: event.itemId,
        userId: event.userId,
      })
    }
  )

  return unsubscribe
}, [])
```

---

## 12. Activation / Désactivation de module

### 12.1 Mécanisme

Chaque module expose `isEnabled()` :

```typescript
isEnabled(): boolean {
  return true // ou false pour désactiver
}
```

Le routeur ne charge que les modules activés :

```typescript
const enabledModules = moduleRegistry.getAllEnabled()
```

### 12.2 Cas d'usage

- Features expérimentales
- Plans futurs (habits, meal-planner)
- Désactivation temporaire pour maintenance

---

## 13. Ajout d'un nouveau module — Processus

### Étape 1 : Créer la structure

```bash
mkdir -p features/new-module/{api,ui/{screens,components},domain/{model,usecase,repository},data/{local,remote}}
```

### Étape 2 : Implémenter le contrat

```typescript
// features/new-module/api/NewModuleContract.ts
export class NewModuleContract implements ModuleContract {
  readonly name = 'new-module' as const
  // ...
}
```

### Étape 3 : Créer les modèles domain

```typescript
// features/new-module/domain/model/NewEntity.ts
export interface NewEntity extends BaseEntity {
  // ...
}
```

### Étape 4 : Implémenter repository

```typescript
// features/new-module/data/NewModuleRepositoryImpl.ts
export class NewModuleRepositoryImpl implements NewModuleRepository {
  // ...
}
```

### Étape 5 : Créer les écrans UI

```typescript
// features/new-module/ui/screens/NewModuleScreen.tsx
export function NewModuleScreen() {
  // Utiliser uniquement le UI Kit
}
```

### Étape 6 : Enregistrer le module

```typescript
// core/modules/index.ts
moduleRegistry.register(new NewModuleContract())
```

---

## 14. Règles de validation

### 14.1 Checklist avant commit

- [ ] Le module implémente `ModuleContract`
- [ ] Le module est enregistré dans `core/modules/index.ts`
- [ ] Les routes sont exposées via `routes()`
- [ ] Les composants UI utilisent uniquement le UI Kit
- [ ] Aucune couleur hardcodée (tout via thème)
- [ ] Les événements inter-modules passent par Event Bus
- [ ] Offline-first respecté (local d'abord, sync arrière-plan)
- [ ] Structure de dossiers conforme

### 14.2 Anti-patterns interdits

❌ **Appel direct entre modules** :

```typescript
// INTERDIT
import { budgetRepository } from '@/features/budget/data/BudgetRepositoryImpl'
budgetRepository.createTransaction(...)
```

✅ **Via Event Bus** :

```typescript
// CORRECT
eventBus.emit(EventTypes.SHOPPING_ITEM_BOUGHT, event)
```

❌ **Couleurs hardcodées** :

```typescript
// INTERDIT
<div style={{ color: '#ffffff' }}>
```

✅ **Via thème** :

```typescript
// CORRECT
const style = getTextPrimaryStyle(theme)
<div style={style}>
```

---

## 15. Principe final

> Le Framework Modulaire Famitura privilégie :
> - La simplicité d'ajout d'un module
> - La cohérence architecturale
> - La découplage strict entre modules
> - L'offline-first par défaut
>
> Toute déviation doit être documentée et justifiée.
