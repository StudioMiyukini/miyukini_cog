# Miyukini Framework - Module Reservation Booking

## Contexte

Le module **Réservation (Booking)** est une alternative “framework-ready” à Planity/Doctolib :  
des **visiteurs/clients** réservent des créneaux de prestations, et des **prestataires** gèrent leurs disponibilités (CRUD à l’unité, en masse, semaines types, vacances), leurs réservations (validation/annulation) et des créneaux à **plusieurs places**.

Ce module s’appuie sur la **capability Agenda** (créneaux, ressources, tags, règles de disponibilité) et ajoute la couche métier “réservation”.

## Portée / Scope

- **Inclus** :
  - disponibilités (créneaux) par prestataire,
  - réservation client, annulation et replanification,
  - capacité multi-places,
  - création en masse (semaine type) + exceptions,
  - mode vacances (du… au…),
  - back-office pour administration et modération.
- **Exclus** :
  - paiement en ligne (PSP/checkout),
  - dossiers médicaux / exigences “santé” (Doctolib) (hors scope).

---

## 1) Acteurs & rôles

- **Client / Visiteur** : recherche, réserve, annule selon règles.
- **Prestataire** : configure son offre, gère créneaux et réservations.
- **Manager** (optionnel) : supervise plusieurs prestataires.
- **Admin / Super Admin** : accès global, modération, override.

---

## 2) Standard de composition (capabilities utilisées)

- **Auth/RBAC** : identité + rôles + RLS.
- **Agenda** : stockage/calcul des créneaux + conflits + statuts de slots.
- **Ma Page** (optionnel) : bloc `agenda_booking` sur un profil public.
- **Emailing** (optionnel mais recommandé) : confirmations, rappels, annulations.
- **Documents** (optionnel) : CGV, confirmation PDF, preuves.
- **Budget** (optionnel) : si paiement confirmé manuellement (hors ligne).

> Règle : le module orchestre et publie des événements `booking.*`. La capability Agenda reste la source de vérité des créneaux.

---

## 3) Modèle métier

### 3.1 Entités

- **Provider (Prestataire)** : profil, tags, timezone, règles.
- **Service (Prestation)** : durée, prix indicatif, règles d’annulation, capacité.
- **Calendar (Agenda)** : un ou plusieurs agendas par prestataire (ex: “Cabine 1”, “Visio”).
- **Slot (Créneau)** : intervalle temps + capacité, statut.
- **SlotServices (Disponibilités par prestation)** : quelles prestations sont autorisées sur un créneau.
- **Booking (Réservation)** : lien client ↔ slot (+ service) + statut.

### 3.2 Statuts de réservation (proposition)

- `requested` (demande faite)
- `confirmed` (confirmée automatiquement ou par prestataire)
- `cancelled_by_client`
- `cancelled_by_provider`
- `no_show` (optionnel)
- `completed` (optionnel)

> Le statut du slot suit la capacité (ex: remaining seats).

---

## 4) Fonctionnalités clés

### 4.1 Côté client

- recherche prestataire / service / date / tags
- visualisation disponibilités (calendrier + slots)
- réservation (choix service + info contact + consent)
- annulation selon politique (délai, pénalité éventuelle)
- replanification (si autorisé)

### 4.2 Côté prestataire

#### 0) Catalogue de prestations
- CRUD des **prestations** (durée, prix indicatif, capacité par défaut, tags, nécessite validation, politique d’annulation).
- Activation/désactivation d’une prestation (`is_active`).

#### A) CRUD créneaux “à l’unité”
- créer/modifier/supprimer un slot
- config : date, heure, durée, buffer, capacité, tags, visibilité (public/unlisted)
- config : **prestations disponibles sur ce créneau** (une ou plusieurs)  
  Exemple : un créneau “Cabine 1” peut autoriser “Coupe” et “Barbe” mais pas “Coloration”.

#### B) CRUD “en masse”
- **semaine type** (templates) :
  - ex: Lun 9-12 + 14-18, Mar 10-16…
  - génération sur une plage (ex: 8 semaines)
- génération en masse avec **règles de prestations** (par plage horaire / par ressource / par jour)
- **exceptions** :
  - jours fériés, événements, indisponibilités partielles
- **vacances** :
  - intervalle [start, end] = indisponible
  - option : annuler automatiquement les réservations futures ou demander validation manuelle

#### C) Gestion des réservations
- confirmer/refuser (si mode “approval”)
- annuler (avec motif)
- marquer no-show / completed
- export (CSV/ICS)

### 4.3 Multi-places (capacité)

Un slot a :
- `capacity_total`
- `capacity_remaining`
- bookings multiples jusqu’au plafond

Règles :
- surbooking désactivé par défaut,
- support “group booking” (ex: 3 places) optionnel.

---

## 5) UX / Écrans

### 5.1 Client (front)

- **Recherche**
  - filtres : tags, date, service, distance (futur)
- **Page prestataire**
  - présentation + CTA “Réserver”
  - vue disponibilités
- **Réservation**
  - formulaire + récapitulatif + confirmation
- **Mes réservations**
  - liste + annulation + replanification

### 5.2 Prestataire

- **Dashboard planning**
  - calendrier + liste
  - “Créer créneau” / “Générer semaine type”
- **Prestations**
  - liste + CRUD
  - tags, durée, capacité, règles (approval/annulation)
  - option : “rendre disponible par défaut sur les nouveaux créneaux”
- **Agenda des réservations (back-office prestataire)**
  - vue “planning” + vue “liste” des réservations
  - actions : confirmer (si approval), annuler, replanifier, marquer no-show/completed
  - visibilité : n’affiche **que** les réservations/slots/services du prestataire (filtrage **RLS**)
- **Semaine type**
  - éditeur template (plages horaires)
  - génération sur période
- **Vacances**
  - date début/fin, options de gestion des bookings existants
- **Réservations**
  - liste (statuts) + détails + actions

### 5.3 Admin

- supervision multi-prestataires (vision “plateforme”)
- overrides (annulation, création slots) via outils admin dédiés
- audit & logs

> Isolation : l’**admin n’a pas accès** au back-office “privé” de chaque prestataire.  
> Le back-office prestataire est un espace “tenant-like”, filtré par RLS. L’admin travaille via son propre back-office global (modération, outils plateforme), sans navigation vers l’espace interne d’un prestataire.

---

## 6) Data contract (proposition)

### 6.1 Tables du module (Booking)

#### `booking_providers`
- `id` UUID (profiles.id ou entité dédiée)
- `display_name`
- `timezone`
- `tags` TEXT[]
- `is_active` BOOLEAN
- `created_at`, `updated_at`

#### `booking_services`
- `id` UUID
- `provider_id` UUID
- `name`
- `description`
- `duration_minutes`
- `price_hint` NUMERIC (optionnel)
- `currency` TEXT
- `requires_approval` BOOLEAN
- `cancellation_policy` JSONB (délai min, pénalité)
- `default_capacity` INT (ex: 1)
- `tags` TEXT[]
- `is_active`

#### `booking_slot_services` (disponibilités par créneau)
Table de jointure explicite “créneau ↔ prestations autorisées”.
- `id` UUID
- `provider_id` UUID
- `slot_id` UUID/TEXT (référence slot Agenda)
- `service_id` UUID (référence `booking_services`)
- `created_at`

> Alternative légère (moins robuste) : stocker `service_ids` dans `slots.metadata` ou `slots.tags`.  
> Recommandé Miyukini : **table dédiée** (`booking_slot_services`) pour requêtes, RLS, audit et intégrité.

#### `booking_bookings`
- `id` UUID
- `provider_id` UUID
- `service_id` UUID
- `agenda_id` TEXT/UUID (si Agenda capability)
- `slot_id` UUID/TEXT (référence slot Agenda)
- `customer_id` UUID NULL (profiles.id si connecté)
- `customer_email` TEXT (fallback)
- `customer_phone` TEXT (optionnel)
- `status` ENUM (`requested`,`confirmed`,`cancelled_by_client`,`cancelled_by_provider`,`no_show`,`completed`)
- `quantity` INT DEFAULT 1 (places réservées)
- `notes_customer` TEXT
- `notes_internal` TEXT
- `cancel_reason` TEXT
- `created_at`, `updated_at`

#### `booking_week_templates`
- `id` UUID
- `provider_id` UUID
- `name` TEXT
- `timezone` TEXT
- `rules` JSONB
  - ex: `{ "monday":[{"start":"09:00","end":"12:00","slot":30,"capacity":1}, ...], ... }`
- `is_active` BOOLEAN
- `created_at`, `updated_at`

#### `booking_time_off`
- `id` UUID
- `provider_id` UUID
- `start_at` TIMESTAMPTZ
- `end_at` TIMESTAMPTZ
- `mode` ENUM (`block_slots`,`cancel_bookings`,`request_reschedule`)
- `reason` TEXT
- `created_at`

### 6.2 Tables de la capability Agenda (réutilisées)

Référence au doc Agenda :
- `agendas`, `slots`, `slot_participants`, `slot_resources`, `slot_events`

Recommandation :
- le module Booking n’invente pas un second modèle de créneaux : il **référence** `slots`.
- la capacité multi-places est portée par le slot (`capacity`) + somme des bookings.

---

## 7) Event map (booking.* + agenda.* + emailing.*)

### 7.1 Events émis par Booking

- `booking.provider.updated`
- `booking.service.created`
- `booking.service.updated`
- `booking.slot.generated` (bulk create)
- `booking.booking.requested`
- `booking.booking.confirmed`
- `booking.booking.cancelled`
- `booking.booking.rescheduled`

### 7.2 Events émis/consommés via Agenda

- Booking consomme :
  - `agenda.slot.confirmed` (si un slot devient indisponible)
  - `agenda.slot.cancelled` (si slot supprimé)
- Booking émet :
  - `agenda.slot.requested` (création)
  - `agenda.slot.updated` (capacity)
  - `agenda.slot.cancelled` (suppression)

### 7.3 Templates transactionnels (Emailing)

- `booking.booking.requested` → confirmation de demande
- `booking.booking.confirmed` → confirmation + détails
- `booking.booking.cancelled` → annulation + prochaines actions
- `booking.time_off.created` (optionnel) → informer les clients impactés

---

## 8) Policies / RLS (modèle recommandé)

### 8.1 Principes

- **Client** : accès à ses réservations uniquement.
- **Prestataire** : accès à ses services/slots/bookings uniquement.
- **Admin** : accès global “plateforme” **sans** accès au back-office privé prestataire.
- **SuperAdmin** : accès global “plateforme” **avec** accès au back-office privé prestataire.
- **Slots publics** : lecture des disponibilités “publiques” uniquement.

### 8.2 Règles (pseudo)

- `booking_services`
  - `SELECT`: public si `is_active` et provider public ; sinon provider/admin
  - `INSERT/UPDATE/DELETE`: provider/admin
- `booking_slot_services`
  - `SELECT`: provider_id = auth.uid() OR admin (plateforme)
  - `INSERT/UPDATE/DELETE`: provider/admin
- `booking_bookings`
  - `SELECT`: customer_id = auth.uid() OR provider_id = auth.uid() OR admin (plateforme)
  - `INSERT`: public autorisé (avec anti-spam) OU authenticated
  - `UPDATE`: provider/admin (confirm/cancel) ; customer (cancel selon policy)
- `booking_week_templates`, `booking_time_off`
  - `SELECT/WRITE`: provider/admin

> Important : éviter la récursivité dans les policies ; utiliser helpers `is_admin_user()` / `is_super_admin()`.

---

## 9) Robustesse (anti-bugs type “vibe-code”)

- **Idempotence** sur génération en masse : `dedupe_key` par (provider, week_template, date_range).
- **Conflits** : empêcher double booking (capacity_remaining >= quantity).
- **Transactions** : réserver = “lock” logique (ou RPC) pour éviter race conditions.
- **Audit** : journaliser changements (slots, bookings, cancellations).
- **Timezones** : stocker en UTC, afficher en timezone provider.

---

## Prochaines étapes

1) Valider le périmètre exact : mode “approval” par défaut ou confirmation auto ?
2) Rédiger la migration SQL (tables booking + relations avec agenda slots) + policies RLS.
3) Définir les écrans back-office (prestataire) dans le framework (routes `/admin/booking/...`).
4) Connecter Emailing (templates) pour confirmations/annulations.

