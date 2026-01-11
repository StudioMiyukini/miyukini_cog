# Miyukini Framework - Inspiration Catakana Orga Modules

## Contexte

Le projet **Catakana Orga** (dans `docs/framework/inspiration/catakana_orga/catakana-orga`) est une application en production pour la gestion d’un festival, construite avec **Vite + React Router + Supabase + TanStack Query + shadcn/ui**.

Ce document “inspiration” sert à :
- Décortiquer Catakana Orga pour identifier des **modules indépendants** (isolés) mais **connectables** (composables).
- Distinguer ce qui relève de **fonctionnalités transverses** (réutilisables) vs **modules métiers** (vertical slices).
- Proposer un **standard Miyukini** pour la création future de modules.

## Portée / Scope

- **Dans Catakana** : cartographie des domaines fonctionnels visibles dans `src/pages`, `src/components/sections`, `src/features`, `src/lib/*`, `supabase/migrations`, `supabase/functions`.
- **Dans Miyukini Framework** : proposition de standard (arborescence, contrats, conventions de connectabilité).
- **Hors scope** : réécrire Catakana, migrer Catakana vers Next.js, ou “finir” les features Catakana.

---

## Lecture rapide (TL;DR décisionnel)

- **Catakana montre une bonne logique de “sections”** (UI) par domaine, mais la séparation **Module** vs **Fonctionnalité transverse** est parfois mélangée.
- Le **pivot architecturel** le plus important est la notion d’**Edition active** (festival edition) qui traverse beaucoup de domaines (planning, exposants, budget, documents, programme, etc.).
- Pour Miyukini, il faut standardiser un modèle :
  - **Fonctionnalité (capability)** = composant réutilisable (agenda, paiement, emailing, documents…)
  - **Module (vertical slice)** = produit/cas d’usage (Booking Pro, Abonnement, Festival Orga…)
  - Les modules “branchent” des fonctionnalités via des **ports/adapters** et un **manifest**.

---

## Cartographie Catakana : structure et signaux

### Stack et routage

- **Routing** : `react-router-dom` avec routes publiques + route protégée `/dashboard` (RequireAuth).
- **Auth** : `AuthContext` (session Supabase + “profile” en table `profiles`).
- **State transverses** :
  - `EditionContext` : charge une `activeEdition` (sinon fallback sur édition la plus récente).
  - `LayoutModeContext` : mode d’affichage (mobile/desktop).
  - `RoleSimulationContext` : simulation de rôles.

### Organisation du code (indices d’architecture)

- **UI “sectionnée” par domaine** : `src/components/sections/*`
  - `admin/*` (users, invoices, editions, notifications, themes, app settings…)
  - `exhibitors/*` (candidatures, emplacements, annuaire, plan…)
  - `activites/*` (animations, ateliers, jeux, concours, guests…)
  - `communication/*` (emailing, templates, campagnes)
  - `informations/*` (programme, plan, reglements, galleries)
  - `gestion/*` (budget/report, team, postes, material management)
  - `organisation/*` (intervenants, demandes)
  - `gamification/*` (rewards, leaderboards)

- **Services Supabase** :
  - “Services métier” : `src/lib/services/*`
  - “Services DB” : `src/lib/supabase/*` (agenda, budget, documents, editions, exposants, email, factures…)

- **Schéma DB** : migrations nombreuses dans `supabase/migrations/` (agenda/schedule, events, stand reservations, invoice system, material management, emailing system, notifications…)
- **Edge functions** : `supabase/functions/send-invoice-email`, `send-campaign-email`, `process-scheduled-campaigns`.

---

## Extraction : Fonctionnalités transverses (Capabilities)

Ces éléments doivent devenir des **“fonctionnalités” Miyukini** réutilisables entre modules :

### 1) Auth & Profils & Rôles (RBAC)

- **But** : session, profil utilisateur, typage rôle (`admin/manager/volunteer/visitor/exhibitor` côté Catakana).
- **Réutilisation** : tous les modules.
- **Connecteurs** :
  - tables `profiles`
  - policies RLS et helpers de rôle
  - UI : login/reset password, compte.

### 2) “Edition active” (Contexte global)

- **But** : déterminer le contexte “édition en cours” qui filtre les données et l’UI.
- **Réutilisation** : modules festival, booking, planning, etc.
- **Standard Miyukini** : une capability “Context” (EditionContext / WorkspaceContext / TenantContext) avec :
  - `getActiveContext()`, `setActiveContext()`, `subscribe()`
  - cache + refresh interval (ex: 5 min)

### 3) Agenda / Planning / Slots

- **But** : gérer des créneaux, événements, interventions, etc.
- **Réutilisation** :
  - module “Booking Pro” (rdv, planning prestataire)
  - module “Festival Orga” (programme, planning bénévoles)
  - module “Abonnement” (renouvellements, cycles, fenêtres d’accès)
- **Signal Catakana** : migrations `schedule_slots`, `events`, `program_events`, services `agendaService`, composants `ScheduleGrid`.

### 4) Paiement / Facturation / Devis

- **But** : factures, encaissements, remises, TVA, génération, email.
- **Réutilisation** :
  - module “Booking Pro”
  - module “Abonnement”
  - module “Festival Orga” (stands, réservations)
- **Signal Catakana** : `invoiceService`, migrations `create_invoice_system`, `invoice_discounts`, edge `send-invoice-email`.

### 5) Documents & Uploads (KYC, justificatifs, dossiers)

- **But** : upload, stockage, statut, visibilité (public/privé), organisation.
- **Réutilisation** : exposants, users, réservation, admin compliance.
- **Signal Catakana** : `documentsService`, migrations `exposant_documents`, `is_public_to_docs`.

### 6) Communication / Emailing / Campagnes

- **But** : templates, campagnes, planification, logs, providers (SMTP/Gmail).
- **Réutilisation** : relances, marketing, notifications, factures.
- **Signal Catakana** : `emailingService`, migrations emailing + edge functions.

### 7) Notifications & Préférences

- **But** : centre de notification + réglages de réception.
- **Réutilisation** : tous les modules.
- **Signal Catakana** : migrations `notification_settings`, `email_logs`, UI admin `NotificationsManager`.

### 8) CMS léger (News / Texts / Pages)

- **But** : actualités, contenus éditoriaux, textes paramétrables.
- **Réutilisation** : onboarding, home, pages info.
- **Signal Catakana** : `newsService`, sections admin news/texts.

### 9) Plan / Floor plan / Map interactive (optionnelle)

- **But** : plan interactif (stands, zones), drag/drop, cartes.
- **Réutilisation** : festival, salons, réservation d’emplacements.
- **Signal Catakana** : `features/plan`, `fabric`, `mockFloorPlan`, `PlanCanvas`, `FloorPlanViewer`.

### 10) Gamification (optionnelle)

- **But** : rewards, leaderboard, claims.
- **Réutilisation** : marketing, engagement, fidélité.
- **Signal Catakana** : migrations + services `gamification*`.

---

## Extraction : Modules métiers (Vertical Slices)

Un **module** est un assemblage de capabilities orienté “produit” / “cas d’usage”.

### Module A — Festival Orga (Catakana)

**Objectif** : organiser une édition (programme, exposants, stands, docs, équipe, budget, communication).

**Capabilities utilisées** :
- Auth/Rôles
- Edition active
- Agenda/Programme
- Documents
- Paiement/Facturation (stands)
- Communication/Emailing
- Notifications
- Plan interactif (emplacements)
- Gestion (budget, matériel, postes)

### Module B — Booking Pro (Miyukini)

**Objectif** : permettre à un pro de gérer des réservations et paiements.

**Capabilities utilisées** :
- Auth/Rôles
- Agenda (slots/rdv)
- Paiement/Facturation
- Notifications
- Documents (contrats, justificatifs) (optionnel)

### Module C — Abonnement (Miyukini)

**Objectif** : gérer plans, paiements récurrents, droits, cycles.

**Capabilities utilisées** :
- Auth/Rôles
- Paiement (récurrence)
- Notifications
- CMS (pages légales, CGV, etc.)

### Module D — Exposants / Marketplace (Miyukini)

**Objectif** : candidatures, dossiers, validation, réservation.

**Capabilities utilisées** :
- Auth/Rôles
- Documents
- Paiement/Facturation
- Notifications
- Edition active (si “salon / édition”)

---

## Standard Miyukini proposé : “Modules composables”

### Définitions

- **Fonctionnalité (Capability)** : brique réutilisable (agenda, paiement, documents, emailing…).
  - Doit être **stateless** côté UI quand possible et dépendre d’un **port** (interface) plutôt que d’un provider unique.
- **Module (Vertical slice)** : paquet orienté besoin métier (booking, abonnement, festival…).
  - Assemble 1..N capabilities, définit l’UX, et porte les règles métier spécifiques.

### Contrats de connectabilité (indispensables)

Chaque module doit exposer un “manifest” (TS/JSON) décrivant :
- `id`, `name`, `version`
- `routes` (pages/screens + guards + navigation group)
- `navigation` (sidebar entries, bottom nav, ordre, badges)
- `permissions` (rôles requis, claims)
- `capabilitiesUsed` (ex: `agenda`, `payment`, `documents`)
- `dataContracts` (tables, vues, API endpoints, types)
- `migrations` (SQL) + `rlsPolicies` (résumé)
- `edgeFunctions` / `jobs` (si applicable)
- `buckets` (storage)
- `events` publiés/consommés (voir ci-dessous)

### Communication inter-modules : événements (event-driven, simple)

Standardiser une couche “events” (peu importe l’implémentation, même locale au début) :
- `agenda.slot.created`
- `payment.invoice.paid`
- `documents.file.uploaded`
- `user.profile.updated`
- `notifications.preference.changed`

Un module peut :
- **publier** un événement après action
- **s’abonner** pour déclencher une action (ex: “invoice paid” -> “auto-validate reservation”)

### Arborescence recommandée (Miyukini)

Proposition (adaptable à Next.js App Router) :

- `src/capabilities/<capabilityId>/`
  - `domain/` (types, règles métier pures)
  - `ports/` (interfaces, contracts)
  - `adapters/` (supabase, stripe, mock, etc.)
  - `ui/` (components/screens réutilisables)
  - `index.ts` (exports publics)

- `src/modules/<moduleId>/`
  - `manifest.ts`
  - `domain/` (règles spécifiques)
  - `ui/` (screens/pages du module)
  - `data/` (queries, mappers)
  - `integration/` (wiring vers capabilities + events)

### Règles de qualité (extraction depuis Catakana)

- **Ne pas appeler Supabase directement depuis l’UI** : passer par `ports` -> `adapters`.
- **Un contexte global maximum** : Auth + (Optionnel) Tenant/Edition. Le reste = hooks/capabilities.
- **Types stricts** : `database.types.ts` généré + types métiers séparés des types DB.
- **RLS first** : tout module doit livrer ses policies + helpers anti-récursion.
- **Feature flags** : permettre d’activer/désactiver un module (comme Catakana a des sections “BlankCategory”).

---

## Plan d’extraction “organique” (pratique)

1) **Identifier le “pivot”** du produit (Catakana : Edition active).
2) Extraire une première capability “EditionContext” utilisable partout.
3) Extraire ensuite les capabilities à fort ROI (Auth/RBAC, Documents, Paiement, Agenda).
4) Pour chaque module (Booking Pro, Abonnement, Festival Orga) :
   - brancher les capabilities via `manifest.ts`
   - exposer les routes et les entrées de navigation
   - publier/consommer des events

---

## Annexes (signaux Catakana utiles)

- **Routage protégé** : `/dashboard` + `RequireAuth` (React Router).
- **Edition active** : `EditionContext` charge `is_active=true` ou fallback sur la plus récente.
- **Emailing** : edge functions + migrations (templates, campagnes, logs, SMTP/Gmail).
- **Facturation** : migrations dédiées + edge d’envoi de facture.

