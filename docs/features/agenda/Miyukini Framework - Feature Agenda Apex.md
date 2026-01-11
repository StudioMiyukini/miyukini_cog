# Miyukini Framework - Feature Agenda Apex

## Contexte

L’objectif est de définir une **fonctionnalité Agenda** suffisamment générique et complète pour être utilisée par plusieurs modules (Booking Pro, Abonnement, Festival Orga, SuperAdmin, …).  
Cette capacité s’inspire des agendas grands publics (Doctolib, Planity) et du catalogue interne de Catakana Orga (planning d’éditions, interventions, ateliers, stands) pour couvrir l’ensemble des usages professionnels.

Ce document APEX (Architecture, Processus, Expérience) alignera :
- Les éléments fonctionnels (créneaux, ressources, notifications, paiements, signatures, etc.).
- Les connexions aux autres capabilities (auth/RLS, documents, communication, billing…).
- Le standard d’intégration (ports, events, data contracts) pour un module Agenda réutilisable.

## Benchmark agenda pro

| Plateforme | Points forts | Différences détectées | Inspiration pour Miyukini |
|------------|--------------|------------------------|----------------------------|
| **Doctolib** | Interface patient/pro, multi-praticiens, accès à la disponibilité, confirmation, rappels (SMS/email), prise en charge tiers (CPAM). | FS orienté santé (créneaux 24h, dossiers médicaux). | Sessions multi-praticiens + tranches horaires régulières, statuts (disponible/occupé), notifications, lien documents/paiements. |
| **Planity** | Gestion salons/besoins multi-ressources, synchronisation calendrier, paiement en ligne, reporting, plan de production. | Forte intégration avec fournisseurs externes (calendar sync). | Ressources (cabines, salles), événements réseaux (workflows), calcul de capacité, exports. |
| **Catakana Orga** | Planning d’édition, créneaux d’animations, ateliers, concurrences, invités, slots, ressources (stands), multi-rôles (admin, manager, visitor). | Focus événementiel et salaires, includes modules “program”, “material management”, “budget”. | Contexte “édition active” = shared calendar, interventions avec ressources (matériel, intervenants), tickets, check-in, communication. |

## Fonctionnalités remontées (capabilities)

1. **Gestion complète CRUD des ressources / calendriers / créneaux**
   - Permettre aux modules ou utilisateurs autorisés de créer, voir, mettre à jour et supprimer des agendas (calendriers) et leurs slots.
   - Configuration multi-dimensionnelle : taille du créneau, buffer/minimum entre créneaux, nombre de participants max, nombre de ressources assignables, tags, couleurs.
   - Affectation d’une ou plusieurs ressources (salle, intervenant, stand, matériel, équipe) avec règles de conflits.
   - Possibilité de workflow “provisoire → confirmé → facturé” ou “brouillon → publié”, avec statuts explicites (Draft / Pending / Confirmed / Paid / Cancelled).
   - Rôles et scopes : uniquement certains rôles (admin, planner, manager) peuvent modifier ; lecture possible pour visiteurs selon configuration.
2. **Disponibilité et visibilité publique**
   - Vue calendrier (agenda, grille, UIs mobiles/desktop).
   - Export ICS / synchronisation calendar externe.
3. **Boucle de réservation + workflows**
   - Formulaire de sélection avec filtres multi-tags, ressources, créneaux, zones, disponibilités.
   - Validation modulable (auto, require approbation, require document) puis déclenchement événement `agenda.slot.confirmed`.
   - Génération d’un **résumé** (PDF, email) + pièces attachées (contrat, règlement, fiche intervenant) via Document capability.
   - Marquage des slots par tags (ex : “atelier”, “VIP”, “audio-visuel”) pour filtrage avancé (inspiration Catakana).
4. **Paiement / Facturation**
   - Possibilité de lier un créneau à une facture (dans `Payment capability`).
   - Validation automatique du slot à réception du paiement ou de la confirmation.
5. **Notifications & rappels**
   - Envoi d’email / SMS (via Communication capability).
   - Relances de confirmation (24h avant, jour J).
6. **Gestion des modifications / annulations**
   - Historique de changements, audit `slots_history`, capacités d’undo/restauration.
   - Workflow “re-planification” avec changement de date/ressource et notification aux participants.
   - Politiques d’annulation (pénalités, remboursement partiel, notes, freeze du slot).
   - Contrôles de rôle : seuls les rôles permis peuvent supprimer/annuler, les autres peuvent demander un changement.
7. **Infos contextuelles**
   - Un “editionId” (ou workspace/context) associe chaque slot.
   - Étiquettes, tags, catégories (ex : atelier, intervention, visite guidée).
8. **Indicateurs + reporting + observabilité**
   - Dashboards : taux d’occupation, créneaux confirmés vs provisoires, revenu lié, anticipations.
   - Export CSV / integration BI (sgd, Supabase, API).
   - Alerts proactives (ex: capacité dépassée, slot non confirmé, paiement en attente).
9. **API & hooks**
   - `getSlots(filters, context)` avec tags, ressources, status, pagination, context workspace.
   - `bookSlot(payload)`, `cancelSlot(slotId)`, `updateSlot(slotId, payload)`, `cloneSlot(slotId)` (CRUD complet).
   - `createCalendar(payload)` / `updateCalendar` / `deleteCalendar`.
   - Hooks/events : `onSlotConfirmed`, `onSlotCancelled`, `onSlotReminder`, `onCapacityThreshold`, `onSlotPaid`.
   - Portabilité multi-module : agenda peut être utilisé par Booking Pro, Abonnement, Festival Orga, Support.

## Architecture APEX proposée

### A = Architecture

- **Ports** :
  - `SlotRepository` (CRUD slots, ressources, conflicts).
  - `NotificationPublisher` (via Communication capability).
  - `BillingAdapter` (paiement, facturation).
  - `DocumentGenerator` (contrats, confirmations).
- **Adapters** :
  - Supabase (via services `scheduleService`, `agendaService`).
  - Mock / Local (pour tests de modules).
- **Events** :
  - `agenda.slot.requested`
  - `agenda.slot.confirmed`
  - `agenda.slot.cancelled`
  - `agenda.slot.rescheduled`
  - `agenda.slot.paid`
  - `agenda.available.update`
- **UI** :
  - `CalendarView`, `SlotForm`, `SlotTimeline`, `SlotList`.
  - `ResourcePicker`, `ConflictWarning`, `ReassignModal`.

### P = Processus

1. Utilisateur sélectionne un contexte (module, edition, resource).
2. Agenda montre les **slots disponibles** (règles de buffer, disponibilité).
3. Créneau en “draft” -> l’event `agenda.slot.requested`.
4. Validation (auto ou humain) -> `agenda.slot.confirmed`.
5. Paiement connecté `payment.invoice.paid` déclenche `agenda.slot.paid`.
6. Rappels + notifications automatisés.
7. Modification/annulation -> `agenda.slot.cancelled`.

### E = Expérience / UX

- **Responsiveness** (mobile/desktop).
- **Points d’entrée** : calendrier global, search by ressource, timeline, quick book.
- **Automations** : mails de rappel, detection de double booking, suggestions de replanification.
- **Transparence** : badge statut (provisoire, confirmé, en attente) + historique.

## Intégration avec d’autres capabilities

- **Auth/RLS** : slots filtrés selon rôle (admin peut override).
- **Contexts** : `EditionContext`→ filtre les slots (ex : editionId).
- **Communication** : hook `onSlotConfirmed` → envoi email/rappel.
- **Documents** : attaché à chaque slot (contrat, fiche intervenant).
- **Billing** : création de facture liée (via event `agenda.slot.paid`).
- **Notifications** : création de préférences (SMS/email) pour rappels.

## Spécification : data contracts

### Tables principales

1. `agendas`
   - `id`, `module_id`, `name`, `description`, `workspace_id`, `created_by`.
   - `default_slot_duration`, `slot_buffer_before`, `slot_buffer_after`.
   - `max_participants_per_slot`, `allow_overbooking`, `timezone`, `is_public`.
2. `slots`
   - `id`, `agenda_id`, `resource_id`, `start_at`, `end_at`.
   - `status` ENUM (`draft`, `pending`, `confirmed`, `paid`, `cancelled`).
   - `capacity`, `participants_count`, `price`, `currency`, `payment_link`.
   - `tags` ARRAY, `metadata` JSONB.
3. `slot_resources`
   - `slot_id`, `resource_id`, `resource_type`, `assigned_at`.
4. `slot_participants`
   - `slot_id`, `user_id`, `role`, `status`, `confirmed_at`.
5. `slot_events`
   - `slot_id`, `event_type`, `actor_id`, `payload`, `created_at`.

### API payloads

- `bookSlot(payload)` expects `{ agendaId, resourceId, startAt, endAt, tags?, capacity?, context?, attachments? }`
- `updateSlot`: partial updates including status transitions, tags, capacity.
- `getSlots(filters)`: `{ agendaIds?, resourceIds?, statuses?, tags?, workspaceId?, startRange?, endRange?, page?, limit? }`
- `cloneSlot(slotId, overrides)`: duplicates slot with new times/resolution.
- `createCalendar(payload)`: { name, workspaceId, tags, configuration }.

## Événements détaillés

- `agenda.slot.requested` – payload `{ slotId, agendaId, requestedBy, context, meta }`
- `agenda.slot.confirmed` – triggers notifications, optionally payment.
- `agenda.slot.paid` – payload includes `{ invoiceId, amount, currency }` for billing ledger.
- `agenda.slot.cancelled` – payload `{ reason, cancelledBy, refundGiven }`.
- `agenda.slot.rescheduled` – new start/end plus previous.
- `agenda.capacity.threshold` – emitted when participants_count == capacity.
- `agenda.schedule.updated` – when configuration (buffers, duration) change.
- `agenda.slot.document.generated` – for trackers documents attachments.

## Policies & RLS

1. `agendas`: 
   - `SELECT` allowed if `workspace_id` matches user workspace or user role admin/global.
   - `INSERT/UPDATE/DELETE` restricted to roles `admin`, `planner`, `scheduler`.
2. `slots`: 
   - `SELECT` for roles on same workspace or participants.
   - `UPDATE` only when `status` in (`draft`, `pending`, `confirmed`) and agent is owner or admin.
   - `DELETE` only admin or when slot is `draft`.
3. `slot_participants`: 
   - `INSERT` allowed for scheduler/admin or when invited.
   - `SELECT` for participants and admins.

## Prochaines étapes
## Prochaines étapes

1. Créer un capability `agenda` avec `domain`, `ports`, `adapters`, `ui`.
2. Concevoir un module “Agenda Pro” (routes + manifest) qui peut être branché sur Booking Pro & Festival Orga.
3. Documenter API (hooks, events, data contracts) dans `docs/features/agenda/` (ce même dossier).

--- 

*Document à garder dans la bibliothèque d’inspiration (nommage Miyukini Framework).*  
