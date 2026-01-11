# Miyukini Framework - Standards Modules Fonctions Couplage

## Objectif

Clarifier la **mécanique de couplage** entre :
- Les **modules métiers** (vertical slices) qui scellent des besoins business (Booking Pro, Festival Orga, Abonnement, BackOffice, etc.)
- Les **functionalités/capabilities** (agenda, paiement, documents, communication, notifications, contexte édition, etc.)

L’objectif est de rendre la **composition** explicite, **décentralisée** (chaque module sait qui appeler) et **centralisable** (via manifest, contracts, events) pour éviter la "toile confusante" actuelle. Ce doc propose un standard lisible, léger, pas un framework rigide.

## 1. Rôles

| Rôle | Responsabilités | Comment |
|------|----------------|---------|
| **Capability Owner** | Définit l’interface (ports/hooks), events, RLS, documentation. | Fournit un **manifest de capability** (id, exports, events). |
| **Module Owner** | Compose les capabilities nécessaires, expose des routes + UI + workflows. | Déclare les capabilities utilisées via son manifest de module. |
| **Platform Coordinator** (Core team) | Gère les contrats communs, IDs, naming, événements globaux. | Maintient docs comme `docs/features/agenda/...`, `docs/framework/...`. |

## 2. Standard de couplage

1. **Capability Manifest**
   - Fichier `capabilities/<id>/manifest.ts` (ou JSON) exposant :
     - `id`, `name`, `version`
     - `exports`: hooks/events/public API (ex: `bookSlot`, `onSlotConfirmed`)
     - `eventsEmitted` / `eventsConsumed`
     - `dataContracts`: noms de tables ou supabase/views si nécessaires
     - `policies`: résumé des policies (ex: `agendas` SELECT/UPDATE)
2. **Module Manifest**
   - `modules/<id>/manifest.ts` avec :
     - `capabilitiesUsed`: liste d’IDs (ex: `['auth', 'agenda', 'billing']`)
     - `events`: énumère `published` et `subscribed`
     - `navigation`: routes/screen definitions
     - `permissions`: rôles requis
3. **Couplage par ports/events**
   - Chaque capability fournit un **port** (interface) consommable par module.
   - Les events (simple pub/sub) sont déclarés dans `events/agenda.ts` etc. Les modules peuvent s’abonner sans importer l’implémentation.

## 3. Mécanique : qui appelle qui ?

1. **Module -> Capability**
   - Le module importe uniquement les **ports** (ex: `AgendaPort`) et utilise `capabilities/agenda/adapters/supabaseAdapter`.
   - Exemple : module `Booking Pro` appelle `AgendaPort.bookSlot(payload)` puis `BillingPort.createInvoice`.
2. **Capability -> Module (events)**
   - Le module s’inscrit (`onAgendaSlotConfirmed`) via un event bus central ou hook `useAgendaEvents()`.
   - Exemple : `Agenda capability` émet `agenda.slot.confirmed` ; module `Festival Orga` réagit en notifiant les assistants.
3. **Centralisation limitée**
   - Un registry global (ex: `modulesRegistry.ts`) recense les manifest et expose un helper pour builder navigation, menus.
   - L’event bus n’a besoin que d’un **router simple** : `eventBus.publish('agenda.slot.confirmed', payload)`.

## 4. Gestion des politiques croisées

- Chaque capability expose sa politique RLS.
- Si un module étend une capability (ex: Agenda ajoute `booking_limit`), il peut **exposer ses propres policies** qui **mappent** aux tables de capability.
- Exemple : module “BackOffice” peut dérouler `Agenda.capabilities` mais créer politiques supplémentaires pour `workspace_id`.

## 5. Documentation et contrats

- Tous les manifests sont référencés dans un index (ex: `docs/framework/module-manifest-index.md`) pour la visibilité.
- Les events doivent être listés dans `docs/events/agenda-events.md`.
- Les `dataContracts` (tables, API) sont décrits dans les docs spécifiques (ex: `docs/features/agenda/..`).

## 6. À retenir

- **Modules = Compose**; capabilities = “couches réutilisables”.
- **Standard = manifest + ports + events**.
- **Couplage léger** : imports de ports + événements, pas de dépendances directes.
- **RLS & policies** suivent la capability, le module ajoute ses filtres si besoin.

---

## 7. Exemple concret : Module « Agenda Pro » (manifest + event map)

### 7.1 Manifest de capability : `agenda`

But : décrire **ce que la capability expose** (API, events, contrats de données, policies).  
Ce manifest est maintenu par le **Capability Owner** (Agenda).

```ts
// src/capabilities/agenda/manifest.ts
export const agendaCapabilityManifest = {
  id: 'agenda',
  name: 'Agenda',
  version: '0.1.0',

  // Exports publics (API / hooks). Le module ne doit pas importer d'implémentation DB directement.
  exports: {
    ports: ['AgendaPort'],
    hooks: ['useAgenda', 'useAgendaEvents'],
  },

  // Contrats de données (source of truth fonctionnelle).
  dataContracts: {
    tables: ['agendas', 'slots', 'slot_participants', 'slot_resources', 'slot_events'],
  },

  // Résumé RLS (la politique est portée par la capability).
  policies: {
    agendas: ['select:workspace_or_role', 'write:admin_planner_scheduler'],
    slots: ['select:workspace_or_participant', 'write:owner_or_admin', 'delete:draft_or_admin'],
    slot_participants: ['select:participant_or_admin', 'write:scheduler_or_invited'],
  },

  // Events publiés par la capability (domaine agenda).
  eventsEmitted: [
    'agenda.slot.requested',
    'agenda.slot.confirmed',
    'agenda.slot.cancelled',
    'agenda.slot.rescheduled',
    'agenda.slot.paid',
    'agenda.capacity.threshold',
    'agenda.schedule.updated',
    'agenda.slot.document.generated',
  ],

  // Events que la capability peut consommer (cross-capability).
  // (Optionnel : une capability peut rester "passive" et laisser les modules réagir.)
  eventsConsumed: [
    'payment.invoice.paid',
    'documents.file.uploaded',
    'notifications.preference.changed',
  ],
} as const
```

### 7.2 Manifest de module : `agenda-pro`

But : décrire **comment le module compose** l’Agenda (routes, permissions, wiring events).  
Ce manifest est maintenu par le **Module Owner** (Agenda Pro).

```ts
// src/modules/agenda-pro/manifest.ts
export const agendaProModuleManifest = {
  id: 'agenda-pro',
  name: 'Agenda Pro',
  version: '0.1.0',

  capabilitiesUsed: [
    'auth',
    'agenda',
    'communication',
    'notifications',
    'documents',
    'billing',
  ],

  permissions: {
    // Le module définit les "rôles" d'accès au module. La capability assure le filtrage fin via RLS.
    requiredRoles: ['admin', 'planner', 'manager'],
  },

  navigation: {
    group: 'backoffice',
    entries: [
      { label: 'Agenda', href: '/admin/agenda', icon: 'tabler--calendar' },
      { label: 'Agendas', href: '/admin/agenda/calendars', icon: 'tabler--calendar-cog' },
      { label: 'Ressources', href: '/admin/agenda/resources', icon: 'tabler--users' },
      { label: 'Réservations', href: '/admin/agenda/bookings', icon: 'tabler--clipboard-list' },
      { label: 'Paramètres', href: '/admin/agenda/settings', icon: 'tabler--settings' },
    ],
  },

  // Le module choisit comment réagir aux événements (workflow).
  events: {
    published: [
      // Exemple : le module peut publier des events "module.*" si nécessaire
      'module.agenda-pro.booking.validated',
    ],
    subscribed: [
      'agenda.slot.requested',
      'agenda.slot.confirmed',
      'agenda.slot.cancelled',
      'agenda.slot.rescheduled',
      'payment.invoice.paid',
    ],
  },
} as const
```

### 7.3 Event map (lisible et “mécanique”)

Principe : l’**Agenda** publie des événements “domaine agenda”.  
Le **module Agenda Pro** (ou d’autres modules) s’abonnent et orchestrent les effets (emails, factures, documents, validations).

| Event | Émis par | Consommé par | Effet attendu (résumé) |
|------|----------|--------------|-------------------------|
| `agenda.slot.requested` | `agenda` | `agenda-pro` | Vérif conflits/capacité + éventuellement mise en `pending` + notification admin. |
| `agenda.slot.confirmed` | `agenda` ou `agenda-pro` (selon choix) | `communication`, `notifications`, `documents` (via `agenda-pro`) | Envoi confirmation, génération doc, programmation rappels. |
| `payment.invoice.paid` | `billing` | `agenda-pro` | Marquer le slot `paid`, auto-validation si règle active. |
| `agenda.slot.paid` | `agenda` | `agenda-pro` | Déclencher actions post-paiement (accès, badges, check-in, etc.). |
| `agenda.slot.cancelled` | `agenda` | `agenda-pro` | Envoi annulation, policy remboursement via billing, libérer capacité. |
| `agenda.slot.rescheduled` | `agenda` | `agenda-pro` | Notifier participants, re-check conflits, recalcul rappels. |
| `agenda.capacity.threshold` | `agenda` | `agenda-pro` | Empêcher surbooking / alerter / proposer alternatives. |
| `agenda.schedule.updated` | `agenda` | `agenda-pro` | Recalculer les disponibilités (UI + cache), notifier si nécessaire. |

### 7.4 Qui “gère” concrètement ?

- **La capability Agenda gère** : modèle de données, règles de conflits, calcul de disponibilité, RLS, statuts de slot, émission d’événements métier “agenda.*”.
- **Le module Agenda Pro gère** : l’UX, les routes, et l’orchestration inter-capabilities (emails, factures, documents, rappels) via abonnements d’événements.
- **Le core framework gère** : registry (listes des manifests), navigation agrégée, et une implémentation simple d’event bus.
