# JayRDV — Spécification Complète du Service

## Contexte

**JayRDV** est le service Miyukini dédié à la **prise de rendez-vous et à la réservation en ligne pour les professionnels du service**. Il couvre l'ensemble du cycle de vie d'un rendez-vous : exposition des créneaux, flux de réservation, gestion des annulations, rappels, synchronisation avec les agendas et la vitrine du professionnel.

Ce document est la **spécification exhaustive** du service : vision, périmètre, modèle de données, flux, écrans, intégrations, gouvernance, contraintes et roadmap.

**Exclusion formelle :** Le domaine **médical** (praticiens de santé, dossier patient, ordonnances, téléconsultation médicale, conformité RGPD santé, HDS) est **exclu** de JayRDV. Un service dédié, **JayBobo**, couvrira les spécificités médicales (fiche patient, historique médical, consentement éclairé, hébergement données de santé, intégration carte Vitale, etc.).

---

## Portée / Scope

| Élément | Couvert | Exclus |
|---------|---------|--------|
| **Secteurs** | Beauté, bien-être, coaching, conseil, artisanat, restauration, immobilier, services à la personne, formation, événementiel, services B2B | Médical (→ JayBobo) |
| **Publics** | Professionnels, Clients (avec ou sans compte), Utilisateurs non connectés (guest) | Patients (→ JayBobo) |
| **Fonctionnel** | Réservation, créneaux, plannings, rappels, annulations, paiement, statistiques, équipe, widget, vitrine | Dossier patient, ordonnance, téléconsultation médicale, carte Vitale |
| **Architecture** | Crate `jayrdv`, intégration JayXpose, JayKoa, MiyuBooking, MiyuNotify, Cores COG | UI finale (→ design system Central / Tauri) |

**Audience :** Équipes produit, architecture, développement, design.

**Références :**
- [Document Fondateur](./JayRDV%20-%20Document%20Fondateur.md)
- [Equivalents Réservation RDV en Ligne](../../reference/equivalents/Miyukini%20Conceptual%20References%20-%20Equivalents%20Reservation%20RDV%20en%20Ligne.md)
- [Accessibilité Client et Parcours de Réservation](./JayRDV%20-%20Accessibilite%20Client%20et%20Parcours%20de%20Reservation.md)
- [Professionnels — Analyse des besoins](./publics/Professionnels/Professionnels%20-%20Analyse%20des%20besoins.md)
- [Clients — Analyse des besoins](./publics/Clients/Clients%20-%20Analyse%20des%20besoins.md)
- [Professionnels — Opérateurs et Toolkits](./publics/Professionnels/Professionnels%20-%20Operateurs%20et%20Toolkits.md)

---

## 1. Vision et positionnement

### 1.1 Proposition de valeur

JayRDV permet à tout **professionnel du service** de :

1. **Proposer des créneaux de réservation en ligne** accessibles 24h/24 7j/7, sans intervention manuelle.
2. **Gérer son calendrier, ses services et ses ressources** (personnes, salles, équipements) depuis un espace professionnel gouverné.
3. **Réduire les rendez-vous manqués** (no-show) grâce à des rappels multi-niveaux (SMS, email, push).
4. **Offrir un parcours client en 3 clics** : choisir un service → sélectionner un créneau → confirmer.
5. **Garder la souveraineté de ses données** : toute la donnée reste dans le COG du professionnel (KindMother), pas dans un cloud tiers.

### 1.2 Positionnement concurrentiel

| Concurrent | Modèle | JayRDV se différencie par |
|-----------|--------|--------------------------|
| Planity | SaaS cloud, beauté/bien-être | Souveraineté des données, multi-secteur (pas limité beauté) |
| Calendly | SaaS cloud, B2B/freelance | Interpolarité COG (JayKoa, JayXpose, JayKonta), offline-first |
| Fresha | SaaS gratuit, beauté | Pas de dépendance cloud, gouvernance Cores |
| Treatwell | Marketplace + SaaS | Pas de commission, données chez le pro |
| Cal.com | Open source, self-hosted | Intégration native écosystème Miyukini, Cores de gouvernance |

### 1.3 Secteurs cibles

| Secteur | Exemples de professionnels |
|---------|---------------------------|
| **Beauté / Bien-être** | Coiffeurs, barbiers, esthéticiennes, masseurs, spas |
| **Coaching / Conseil** | Coachs sportifs, consultants, avocats, experts-comptables |
| **Artisanat / Services** | Plombiers, électriciens, photographes, architectes d'intérieur |
| **Restauration** | Restaurants (tables), traiteurs, food trucks |
| **Immobilier** | Agents immobiliers (visites), diagnostiqueurs |
| **Formation** | Formateurs, écoles (cours, ateliers, stages) |
| **Événementiel** | Organisateurs (créneaux de participation), prestataires |
| **Services B2B** | Agences, studios, prestataires IT (démos, appels) |

---

## 2. Séparation JayRDV / JayBobo

### 2.1 Principe

Le domaine médical impose des **contraintes réglementaires, sécuritaires et fonctionnelles** qui justifient un service dédié :

| Dimension | JayRDV (Service) | JayBobo (Médical) |
|-----------|------------------|-------------------|
| **Données** | Fiche client (nom, email, téléphone, préférences) | Fiche patient (antécédents, allergies, prescriptions, consentement éclairé) |
| **Réglementation** | RGPD standard | RGPD santé + HDS (Hébergement Données de Santé) + codes de déontologie |
| **Identité** | Compte Miyauth ou guest (téléphone + OTP) | Identité vérifiée (carte Vitale, INS, identifiant national) |
| **Téléconsultation** | Visio généraliste (optionnel) | Téléconsultation médicale (cadre légal, prescriptions, compte-rendu) |
| **Rappels** | SMS/email/push configurables | Rappels + consignes pré-consultation (jeûne, documents à apporter) |
| **Paiement** | Acompte / paiement libre | Tiers payant, mutuelle, dépassement d'honoraires |
| **Intégrations** | JayXpose (vitrine), JayKoa (agenda), MiyuBooking, MiyuNotify | Logiciels métier santé, DMP, protocoles HL7/FHIR |
| **Sécurité** | WorrySentinel standard (S1-S3) | Niveaux de sécurité renforcés (S3-S5), chiffrement bout-en-bout obligatoire |

### 2.2 Interfaces communes

JayRDV et JayBobo **partagent** :
- Le moteur de créneaux (MiyuBooking) : slots, disponibilités, hold, anti-double-booking.
- Le moteur de rappels (MiyuNotify) : SMS, email, push.
- L'agenda unifié (JayKoa) : reflets des RDV confirmés.
- La vitrine (JayXpose) : page publique du professionnel.

JayBobo **ajoute** une couche de conformité santé au-dessus de ces briques communes.

---

## 3. Architecture et intégrations inter-services

### 3.1 Vue d'ensemble

```
JayXpose (vitrine professionnel)
    │ Alimente : profil pro, services proposés, page publique, widget
    ▼
JayRDV (service de réservation)
    │ Orchestre : écrans, vues, flux de réservation
    │ CRUD : services, créneaux, rendez-vous
    │ Gère : annulations, modifications, rappels, paiement
    │ Utilise : MiyuBooking (moteur créneaux), MiyuNotify (rappels)
    │           MiyuAuth (identité), MiyuProfile (profil client)
    │           MiyuInvoice (paiement/acompte)
    ▼
JayKoa (agenda universel)
    │ Reçoit : reflets des RDV confirmés (lecture seule)
    │ Organise : agenda unifié par utilisateur
    ▼
JayKonta (comptabilité — optionnel)
    └── Reçoit : écritures comptables si paiement (factures, acomptes)
```

### 3.2 Rôle de chaque brique

| Brique | Strate | Rôle dans JayRDV |
|--------|--------|-----------------|
| **JayXpose** | Service (7) | Alimente JayRDV avec les **informations du professionnel** : présentation, offre de services, photos, horaires, lien public, widget. JayXpose est la **vitrine** ; JayRDV en consomme les données pour configurer ce que le client voit. |
| **JayKoa** | Service (7) | **Organise les agendas** de chacun. Agrège et reflète les engagements temporels. Reçoit les RDV confirmés de JayRDV en reflets (lecture seule) via `JayRDVAdapter::sync_appointments_from_store`. |
| **MiyuBooking** | Toolkit (6) | **Moteur de créneaux** : calcul des disponibilités, hold temporaire, anti-double-booking, buffers, préavis min/max. |
| **MiyuNotify** | Toolkit (6) | **Rappels et notifications** : SMS, email, push. Envoi de confirmations, rappels J-7/J-1/H-2, alertes pro (nouveau RDV, annulation). |
| **MiyuAuth** | Toolkit (6) | **Identité** : résolution, vérification, attestation. Gestion du compte client ou guest (OTP téléphone). |
| **MiyuProfile** | Toolkit (6) | **Profil** : données client (nom, préférences, historique) et profil professionnel. |
| **MiyuInvoice** | Toolkit (6) | **Paiement** : génération de factures, capture d'acomptes, remboursements. |
| **MiyuStore** | Toolkit (6) | **Boutique** : cartes cadeaux, cures, packs de séances (optionnel). |
| **KindMother** | Core (4) | **Persistance** : toutes les données (RDV, créneaux, clients, services) stockées localement dans le COG du pro. |
| **StrongFather** | Core (4) | **Gouvernance** : évaluation des intentions (réservation, annulation), mandats de permission. |
| **BorderGuard** | Core (4) | **Sécurité périmétrique** : filtrage des requêtes publiques, niveaux S1-S2, rate limiting, anti-spam. |
| **WorrySentinel** | Core (4) | **Sécurité** : niveaux de sécurité, détection de menaces, dégradation gracieuse. |

### 3.3 Flux JayXpose → JayRDV

JayXpose fournit à JayRDV :

| Donnée | Source JayXpose | Usage JayRDV |
|--------|----------------|-------------|
| Profil professionnel | Nom, photo, description, coordonnées | Affiché sur la page de réservation |
| Services proposés | Catalogue de prestations (nom, durée, prix, photo) | Liste des services réservables |
| Horaires d'ouverture | Plages horaires par jour | Base pour le calcul des créneaux disponibles |
| Page publique | URL, template, branding (couleurs, logo) | Habillage de la page de réservation |
| Widget | Code embed, paramètres de personnalisation | Widget intégrable sur le site du pro |

**Synchronisation :** Lecture réfléchie (adaptateur JayXpose → JayRDV) ou API directe. JayRDV ne modifie jamais les données JayXpose ; JayXpose est la source de vérité pour la présentation du pro.

### 3.4 Flux JayRDV → JayKoa

JayRDV envoie à JayKoa :

| Événement | Données | Type de reflet |
|-----------|---------|----------------|
| RDV confirmé | id, titre, start_at, end_at, location, client_name | `EntryType::ReflectJayRDV` |
| RDV annulé | id, statut annulé | Suppression ou mise à jour du reflet |
| RDV modifié | id, nouvelles dates | Mise à jour du reflet |

**Principe :** JayKoa ne crée jamais de RDV ; il reflète. JayRDV est la source de vérité pour les rendez-vous.

---

## 4. Modèle de données

### 4.1 Entités principales

#### 4.1.1 Professional (profil professionnel)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique |
| name | String | Nom affiché (personne ou entreprise) |
| slug | String | Identifiant URL-friendly (ex. « marie-coiffure-paris ») |
| description | Option\<String\> | Présentation courte |
| sector | String | Secteur d'activité (beauté, coaching, artisanat…) |
| photo_url | Option\<String\> | URL photo de profil |
| contact_email | String | Email de contact |
| contact_phone | Option\<String\> | Téléphone |
| address | Option\<String\> | Adresse physique |
| timezone | String | Fuseau horaire (ex. « Europe/Paris ») |
| settings | ProfessionalSettings | Paramètres (politique annulation, préavis, buffer…) |
| created_at | DateTime | Horodatage de création |
| updated_at | DateTime | Horodatage de dernière modification |

#### 4.1.2 Service (prestation proposée)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique |
| professional_id | UUID | Référence au professionnel |
| name | String | Nom de la prestation (ex. « Coupe homme ») |
| description | Option\<String\> | Description détaillée |
| duration_min | u32 | Durée en minutes |
| price | Option\<Decimal\> | Tarif (optionnel) |
| deposit | Option\<Decimal\> | Montant de l'acompte (optionnel) |
| category | Option\<String\> | Catégorie (ex. « Coiffure », « Soin ») |
| photo_url | Option\<String\> | Photo illustrative |
| buffer_before_min | u32 | Temps tampon avant (minutes) |
| buffer_after_min | u32 | Temps tampon après (minutes) |
| min_notice_hours | u32 | Préavis minimum (heures) |
| max_advance_days | u32 | Réservation maximum à l'avance (jours) |
| active | bool | Actif / inactif |
| resource_ids | Vec\<UUID\> | Ressources requises |
| practitioner_ids | Vec\<UUID\> | Praticiens habilités |
| created_at | DateTime | |
| updated_at | DateTime | |

#### 4.1.3 Practitioner (praticien / collaborateur)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique |
| professional_id | UUID | Référence au professionnel (établissement) |
| name | String | Nom affiché |
| role | PractitionerRole | Admin, Gestionnaire, Praticien |
| photo_url | Option\<String\> | Photo |
| service_ids | Vec\<UUID\> | Services que ce praticien peut réaliser |
| created_at | DateTime | |
| updated_at | DateTime | |

#### 4.1.4 Resource (salle, équipement)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique |
| professional_id | UUID | |
| name | String | Nom (ex. « Salle 1 », « Appareil laser ») |
| kind | Option\<String\> | Type (room, equipment, vehicle…) |
| capacity | Option\<u32\> | Capacité (pour cours/ateliers) |
| created_at | DateTime | |
| updated_at | DateTime | |

#### 4.1.5 Schedule (planning récurrent)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| owner_type | ScheduleOwner | Practitioner ou Resource |
| owner_id | UUID | Référence au praticien ou à la ressource |
| day_of_week | u8 | 0 (lundi) à 6 (dimanche) |
| start_time | Time | Heure de début |
| end_time | Time | Heure de fin |
| active | bool | |

#### 4.1.6 Exception (congés, absences, fermetures)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| owner_type | ScheduleOwner | Practitioner, Resource ou Professional (global) |
| owner_id | UUID | |
| date | Date | Jour de l'exception |
| start_time | Option\<Time\> | Début (si partiel) ; None = journée entière |
| end_time | Option\<Time\> | Fin (si partiel) |
| reason | Option\<String\> | Motif (congés, formation, maintenance…) |

#### 4.1.7 Slot (créneau calculé / proposé)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| service_id | UUID | |
| practitioner_id | Option\<UUID\> | |
| resource_id | Option\<UUID\> | |
| start_at | DateTime | |
| end_at | DateTime | |
| status | SlotStatus | Available, Held, Booked, Blocked |
| held_until | Option\<DateTime\> | Si Held : expiration du verrouillage |
| held_by | Option\<String\> | Identifiant de session du client qui a pris le hold |

#### 4.1.8 Appointment (rendez-vous)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| professional_id | UUID | |
| service_id | UUID | |
| practitioner_id | Option\<UUID\> | |
| resource_id | Option\<UUID\> | |
| slot_id | UUID | Créneau réservé |
| client_id | Option\<UUID\> | Si client avec compte |
| client_name | String | |
| client_email | Option\<String\> | |
| client_phone | String | |
| notes | Option\<String\> | Remarques client |
| status | AppointmentStatus | Pending, Confirmed, Cancelled, Completed, NoShow |
| cancellation_reason | Option\<String\> | Motif d'annulation (si applicable) |
| cancelled_by | Option\<CancelledBy\> | Client ou Professional |
| paid_amount | Option\<Decimal\> | Montant payé (acompte ou total) |
| cancel_token | String | Token unique pour annulation par lien |
| created_at | DateTime | |
| updated_at | DateTime | |

#### 4.1.9 Reminder (rappel programmé)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| appointment_id | UUID | |
| channel | ReminderChannel | SMS, Email, Push |
| scheduled_at | DateTime | Date/heure d'envoi prévue |
| sent | bool | Déjà envoyé |
| sent_at | Option\<DateTime\> | Date/heure effective d'envoi |

#### 4.1.10 Client (fiche client)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| professional_id | UUID | Fiche propre à chaque pro (isolation) |
| name | String | |
| email | Option\<String\> | |
| phone | String | |
| notes | Option\<String\> | Notes internes du pro |
| tags | Vec\<String\> | Étiquettes (VIP, fidèle…) |
| total_appointments | u32 | Compteur de RDV |
| last_appointment_at | Option\<DateTime\> | Dernier RDV |
| no_show_count | u32 | Compteur de no-show |
| created_at | DateTime | |
| updated_at | DateTime | |

### 4.2 Énumérations

```rust
enum AppointmentStatus { Pending, Confirmed, Cancelled, Completed, NoShow }
enum SlotStatus { Available, Held, Booked, Blocked }
enum CancelledBy { Client, Professional }
enum ReminderChannel { Sms, Email, Push }
enum PractitionerRole { Admin, Manager, Practitioner }
enum ScheduleOwner { Practitioner, Resource, Professional }
```

### 4.3 Paramètres professionnels (ProfessionalSettings)

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| cancellation_policy_hours | u32 | 24 | Délai minimum avant RDV pour annuler gratuitement |
| allow_guest_booking | bool | true | Autoriser la réservation sans compte |
| require_phone_verification | bool | true | OTP téléphone obligatoire |
| hold_duration_min | u32 | 10 | Durée du verrouillage de créneau (minutes) |
| default_buffer_before_min | u32 | 0 | Buffer par défaut avant chaque RDV |
| default_buffer_after_min | u32 | 0 | Buffer par défaut après chaque RDV |
| default_min_notice_hours | u32 | 2 | Préavis minimum par défaut |
| default_max_advance_days | u32 | 90 | Réservation max à l'avance |
| reminder_j7 | bool | false | Rappel à J-7 |
| reminder_j1 | bool | true | Rappel à J-1 |
| reminder_h2 | bool | true | Rappel à H-2 |
| reminder_channels | Vec\<ReminderChannel\> | [Sms, Email] | Canaux de rappel |
| round_robin_enabled | bool | false | Distribution Round Robin |
| round_robin_strategy | String | "balanced" | Stratégie RR (balanced, ordered, weighted) |
| waitlist_enabled | bool | false | Liste d'attente si créneaux pleins |
| deposit_required | bool | false | Acompte obligatoire |
| payment_at_booking | bool | false | Paiement intégral à la réservation |

---

## 5. Flux de réservation

### 5.1 Parcours client (3 clics)

```
1. DÉCOUVERTE
   Client accède à la page de réservation via :
   - Lien direct (email, SMS, réseaux sociaux)
   - Widget sur le site du pro (via JayXpose)
   - QR code (carte de visite, salle d'attente)
   - Recherche sur la marketplace (si activée)

2. SÉLECTION DU SERVICE (Clic 1)
   - Liste des services actifs du pro (nom, durée, prix, photo)
   - Catégories pour organiser (si nombreux services)
   - Sélection d'un service

3. SÉLECTION DU CRÉNEAU (Clic 2)
   - Calendrier avec créneaux disponibles uniquement
   - Choix du praticien (si multi-praticiens) ou « Premier disponible »
   - Vue jour ou semaine
   - Hold automatique (10 min par défaut) dès la sélection
   - Si créneau expiré : notification « Ce créneau n'est plus disponible »
     + proposition des 3 prochains créneaux

4. IDENTIFICATION + CONFIRMATION (Clic 3)
   - Formulaire minimal : nom, téléphone (obligatoire), email (optionnel)
   - Si client avec compte : pré-rempli, connexion en 1 clic
   - Si guest : OTP téléphone pour vérification
   - Remarques optionnelles
   - Acompte / paiement si configuré par le pro
   - Bouton « Confirmer le rendez-vous »

5. POST-CONFIRMATION
   - Écran de confirmation (récapitulatif complet)
   - Email de confirmation avec :
     - Récapitulatif (service, date, heure, lieu, praticien)
     - Lien « Ajouter à mon agenda » (iCal / Google / Outlook)
     - Lien « Modifier ou annuler mon RDV » (token unique)
   - SMS de confirmation (si téléphone fourni)
   - Notification au professionnel (nouveau RDV)
```

### 5.2 Verrouillage de créneau (Hold)

```
Client sélectionne créneau
    ↓
Slot.status = Held, held_until = now + hold_duration_min
    ↓
Client a 10 min (configurable) pour confirmer
    ↓
Si confirmation → Slot.status = Booked, Appointment créé
Si expiration   → Slot.status = Available (libéré automatiquement)
Si erreur       → Slot.status = Available, client notifié
```

**Anti-double-booking :** Le hold est atomique (transaction KindMother). Deux clients ne peuvent pas hold le même créneau. Le second reçoit « Créneau indisponible » + alternatives.

### 5.3 Annulation

#### Par le client

```
Client clique lien « Annuler » (dans email/SMS, token unique)
    ↓
Vérification token (BorderGuard)
    ↓
Vérification délai (> cancellation_policy_hours avant le RDV ?)
    ↓
OUI → Appointment.status = Cancelled, cancelled_by = Client
       Slot.status = Available (libéré)
       SMS/email confirmation annulation au client
       Notification au pro (créneau libéré)
       Si waitlist_enabled → notification au premier de la liste d'attente

NON → Message « Annulation impossible, délai dépassé.
       Contactez le professionnel directement. »
       OU facturation de frais d'annulation (si configuré)
```

#### Par le professionnel

```
Pro annule depuis son agenda (écran pro)
    ↓
Appointment.status = Cancelled, cancelled_by = Professional
Slot.status = Available (libéré)
    ↓
SMS/email au client : « Votre RDV du [date] a été annulé par [pro].
    Souhaitez-vous reprogrammer ? [Lien réservation] »
Remboursement acompte si applicable
```

### 5.4 Modification

```
Client clique lien « Modifier » (dans email/SMS, token unique)
    ↓
Vérification token + délai
    ↓
Affichage du calendrier avec créneaux disponibles (comme nouvelle réservation)
    ↓
Client sélectionne nouveau créneau → Hold
    ↓
Confirmation → Ancien slot libéré, nouveau slot réservé
               Appointment mis à jour (nouvelles dates)
               Email/SMS de confirmation modification
               Reflet JayKoa mis à jour
```

### 5.5 Rappels

| Timing | Canal | Contenu |
|--------|-------|---------|
| **Immédiat** | Email + SMS | Confirmation de RDV (récapitulatif + liens annuler/modifier + ajout agenda) |
| **J-7** (optionnel) | Email | « Rappel : RDV dans 7 jours avec [pro] le [date] à [heure] » |
| **J-1** | SMS + Email | « Rappel : RDV demain à [heure] avec [pro]. [Lien annuler] » |
| **H-2** | SMS ou Push | « RDV dans 2 heures avec [pro] à [lieu] » |
| **Post-RDV** (optionnel) | Email | « Merci pour votre visite ! [Lien avis] [Lien reprendre RDV] » |

Tous les rappels sont programmés via **MiyuNotify** et stockés dans l'entité `Reminder`. Le timing et les canaux sont configurables par le professionnel dans `ProfessionalSettings`.

---

## 6. Écrans et vues

### 6.1 Écrans client (Façade publique)

| Écran | Description | Données affichées |
|-------|-------------|-------------------|
| **Page de réservation** | Page publique du pro (via JayXpose) avec bouton « Réserver » | Nom, photo, description, adresse, horaires |
| **Sélection service** | Liste des prestations | Nom, durée, prix, photo, catégorie |
| **Sélection créneau** | Calendrier interactif | Créneaux disponibles (vue jour/semaine), choix praticien |
| **Formulaire réservation** | Saisie infos client + confirmation | Nom, téléphone, email, remarques, acompte |
| **Confirmation** | Récapitulatif + liens | Service, date, heure, lieu, praticien, liens annuler/modifier/agenda |
| **Mes RDV** (si compte) | Liste des RDV passés et à venir | Statut, date, service, praticien, actions (annuler, modifier, re-réserver) |
| **Annulation** | Écran d'annulation via token | Confirmation annulation + proposition de reprogrammation |
| **Modification** | Sélection nouveau créneau | Même flux que sélection créneau |

### 6.2 Écrans professionnel (Espace pro)

| Écran | Description | Fonctionnalités |
|-------|-------------|-----------------|
| **Dashboard** | Vue d'ensemble de l'activité | RDV du jour, de la semaine ; indicateurs (total, no-show, taux remplissage) ; alertes |
| **Agenda** | Calendrier complet | Vue jour/semaine/mois ; filtres par praticien/ressource ; drag & drop ; code couleur par service/statut |
| **Services** | Gestion des prestations | CRUD (nom, durée, prix, photo, catégorie) ; activation/désactivation ; association praticiens/ressources |
| **Plannings** | Horaires et exceptions | Horaires récurrents par praticien/ressource ; exceptions (congés, pauses) ; jours fériés |
| **Clients** | Fichier client | Liste, recherche, fiche détaillée (historique, notes, tags, no-show) |
| **Équipe** | Gestion des collaborateurs | Invitation, rôles (Admin/Gestionnaire/Praticien), association services |
| **Ressources** | Salles et équipements | CRUD ; plannings ; association services |
| **Notifications** | Configuration des rappels | Canaux (SMS, email, push), timing, modèles personnalisables |
| **Intégrations** | Lien de réservation et widget | Génération URL, code embed, bouton réseaux sociaux, prévisualisation |
| **Statistiques** | Pilotage | RDV par période, taux de remplissage, taux de no-show, créneaux populaires ; export CSV/PDF |
| **Paramètres** | Configuration générale | Politique d'annulation, préavis, buffers, paiement, round-robin, liste d'attente |

---

## 7. Gouvernance COG

### 7.1 Niveaux de sécurité

| Niveau | Qui | Actions autorisées |
|--------|-----|--------------------|
| **S1** (Observation) | Utilisateur non connecté | Consulter la page de réservation, voir les créneaux disponibles |
| **S2** (Interaction contrôlée) | Client (guest ou avec compte) | Réserver, annuler, modifier (avec OTP ou token) |
| **S3** (Opération gouvernée) | Professionnel (Praticien) | Gérer ses propres RDV, consulter ses statistiques |
| **S4** (Administration) | Professionnel (Gestionnaire, Admin) | CRUD services, plannings, équipe, paramètres, statistiques globales |

### 7.2 Flux gouverné (réservation)

```
Intention de réservation (client)
    ↓
BorderGuard : rate limiting, anti-spam, validation niveau S2
    ↓
StrongFather : évaluation de l'intention (créneau disponible ? politique respectée ?)
    ↓
MiyuBooking : vérification slot + hold atomique
    ↓
KindMother : persistance Appointment + Slot + Client + Reminder
    ↓
MiyuNotify : envoi confirmation (SMS + email)
    ↓
JayKoa : création reflet (sync_appointments_from_store)
```

### 7.3 Anti-spam et anti-abus

| Menace | Protection |
|--------|-----------|
| Spam de réservations | Rate limiting : max 3 réservations / IP / heure |
| Fausses réservations | OTP téléphone (SMS) avant confirmation |
| Bot scraping | CAPTCHA optionnel ; créneaux visibles uniquement après interaction |
| Double booking | Hold atomique (transaction KindMother) |
| Abus d'annulation | Compteur no-show par client ; blocage temporaire si > seuil |
| DDoS | Cloudflare / reverse proxy devant le COG (si exposé web) |

---

## 8. Cas d'usage détaillés

### 8.1 Réservation standard (guest)

1. Paul ouvre `https://marie-coiffure.jayrdv.fr` (lien partagé sur Instagram).
2. Voit la page vitrine de Marie (photo, services, avis). Clique « Réserver ».
3. Sélectionne « Coupe homme — 30 min — 25 EUR ».
4. Voit le calendrier : mercredi 10h30 disponible. Sélectionne. Hold de 10 min.
5. Saisit : Paul Dupont, 06 12 34 56 78. Reçoit un SMS OTP. Le valide.
6. Clique « Confirmer ». RDV confirmé.
7. Reçoit : SMS + email de confirmation avec lien annulation et ajout agenda.
8. J-1 : reçoit un rappel SMS.
9. Paul se présente au salon. Marie valide le RDV (statut → Completed).

### 8.2 Réservation avec choix de praticien

1. Sophie ouvre la page de « Centre Zen — Spa & Massages ».
2. Sélectionne « Massage relaxant — 60 min — 70 EUR ».
3. Voit la liste des praticiens : Julie, Marc, « Premier disponible ».
4. Choisit Julie. Voit les créneaux de Julie uniquement.
5. Sélectionne vendredi 14h. Hold.
6. Confirme (a un compte client : formulaire pré-rempli).
7. Paiement d'un acompte de 20 EUR (si configuré).
8. Confirmation avec reçu de paiement.

### 8.3 Annulation par le client

1. Paul reçoit le SMS rappel J-1 : « RDV demain 10h30. Pour annuler : [lien] ».
2. Paul clique le lien. Page d'annulation affichée.
3. Délai > 24h → annulation autorisée.
4. Paul confirme l'annulation.
5. Créneau libéré. Marie notifiée (« Paul a annulé son RDV de demain »).
6. Si liste d'attente active : premier de la liste notifié (« Un créneau s'est libéré ! »).

### 8.4 Round Robin

1. Cabinet de 3 consultants (A, B, C). Round robin activé (stratégie « balanced »).
2. Client réserve « Consultation 45 min ». Ne choisit pas de praticien.
3. JayRDV vérifie les compteurs : A a 12 RDV cette semaine, B a 10, C a 11.
4. Attribution à B (le moins chargé).
5. Client voit : « RDV avec B le [date] à [heure] ».

### 8.5 Cours / atelier collectif

1. Formateur propose « Atelier aquarelle — 2h — 8 places max ».
2. 5 clients ont déjà réservé. Client 6 voit « 3 places restantes ».
3. Client 6 réserve. Places restantes : 2.
4. Client 9 arrive : « Complet — Rejoindre la liste d'attente ? ».
5. Si place se libère (annulation) → notification au premier de la liste d'attente.

### 8.6 Gestion offline (COG hors ligne)

1. COG du pro est hors ligne (coupure internet).
2. Client accède à la page de réservation via le buffer (Cloudflare Workers).
3. Voit les créneaux snapshot (dernière synchro).
4. Soumet une demande de réservation → mise en buffer.
5. SMS : « Demande reçue. Confirmation sous quelques heures. »
6. COG revient en ligne → sync buffer → évaluation StrongFather → confirmation ou conflit.
7. SMS final : confirmation ou proposition alternative.

---

## 9. Roadmap d'implémentation

### 9.1 Phase 1 — MVP (v0.1)

| Livrable | Détail |
|----------|--------|
| Types domaine | Appointment, Slot, Resource, Service, Client, Reminder, AppointmentStatus |
| Store mémoire | CRUD en mémoire (v0), migration KindMother/legacy-sqlite ensuite |
| Domain layer | appointment_create, appointment_set_status, slot_create, resource_create, reminder_create |
| Intégration JayKoa | sync_appointments_from_store (reflets RDV confirmés) |
| Balisage MSCM | @id, @do, @layer sur tous les modules |
| Tests | Unitaires (store + domain) + intégration (sync JayKoa) |

**Statut actuel :** Types, store mémoire, domain, intégration JayKoa = **fait**. MSCM et tests = **à faire**.

### 9.2 Phase 2 — Persistance et services

| Livrable | Détail |
|----------|--------|
| Feature `legacy-sqlite` | Structure data/ alignée sur les autres services (kindmother_db.rs) |
| Feature `kindmother-only` | Délégation KindMother Client |
| Entités étendues | Professional, Service, Practitioner, Schedule, Exception, Client |
| Hold de créneau | SlotStatus::Held, expiration automatique |
| Intégration MiyuBooking | Calcul des créneaux disponibles, anti-double-booking |
| Intégration MiyuNotify | Rappels SMS/email, confirmation, alertes pro |
| Intégration JayXpose | Lecture réfléchie du profil pro et des services |

### 9.3 Phase 3 — Interface web publique (Stratégie 1)

| Livrable | Détail |
|----------|--------|
| Page de réservation | Interface web responsive (MiyuWeb + JayRDV) |
| Parcours « 3 clics » | Sélection service → créneau → confirmation |
| OTP téléphone | Vérification SMS (MiyuAuth) |
| Widget / embed | Code intégrable sur le site du pro (JayXpose) |
| Lien de réservation | URL unique par professionnel |
| Buffer offline | Cloudflare Workers / snapshot JSON si COG offline |

### 9.4 Phase 4 — Fonctionnalités avancées

| Livrable | Détail |
|----------|--------|
| Multi-praticien | Round Robin (balanced, ordered, weighted) |
| Ressources | Association service ↔ ressource, anti-double-booking ressource |
| Cours / ateliers | Places limitées, liste d'attente |
| Statistiques | Dashboard pro (RDV, remplissage, no-show, export) |
| Paiement | Acompte / paiement intégral (MiyuInvoice) |
| Cartes cadeaux / cures | MiyuStore (packs de séances) |
| Offres heures creuses | Créneaux à tarif réduit / dernière minute |

### 9.5 Phase 5 — PWA et mobile

| Livrable | Détail |
|----------|--------|
| PWA installable | manifest.json, Service Worker, cache offline |
| Notifications push | Web Push API |
| App mobile pro | Gestion agenda + RDV depuis smartphone |

---

## 10. Contraintes et invariants

### 10.1 Invariants fonctionnels

| Id | Invariant | Description |
|----|-----------|-------------|
| INV-RDV-1 | **Pas de double booking** | Un créneau ne peut être réservé qu'une seule fois par praticien ET par ressource. |
| INV-RDV-2 | **Hold atomique** | Le verrouillage d'un créneau est transactionnel ; deux clients ne peuvent pas hold le même créneau. |
| INV-RDV-3 | **Délai d'annulation** | Le client ne peut annuler que si le délai configuré par le pro est respecté. |
| INV-RDV-4 | **Données chez le pro** | Toutes les données (RDV, clients, plannings) sont persistées dans le COG du professionnel (KindMother), jamais dans un cloud tiers. |
| INV-RDV-5 | **JayRDV ne décide pas du temps** | JayRDV gère les créneaux et les réservations ; l'agenda unifié est géré par JayKoa (reflets uniquement). |
| INV-RDV-6 | **JayXpose est la source de vérité du pro** | JayRDV consomme les données professionnel de JayXpose (profil, services, page) sans les modifier. |
| INV-RDV-7 | **Pas de donnée médicale** | JayRDV ne stocke ni ne traite de données de santé. Le domaine médical est réservé à JayBobo. |
| INV-RDV-8 | **Rappels configurables** | Le professionnel contrôle les canaux et le timing des rappels. Aucun rappel n'est envoyé sans sa configuration. |
| INV-RDV-9 | **Guest autorisé** | La réservation sans compte est autorisée par défaut (configurable). |
| INV-RDV-10 | **Isolation des données** | Les données d'un professionnel ne sont jamais visibles par un autre professionnel. |

### 10.2 Contraintes techniques

| Id | Contrainte | Description |
|----|-----------|-------------|
| C-RDV-1 | **Persistance via KindMother** | Toute écriture passe par KindMother (legacy-sqlite ou kindmother-only). |
| C-RDV-2 | **Gouvernance Cores** | Toute action publique (réservation, annulation) passe par BorderGuard → StrongFather. |
| C-RDV-3 | **Conformité MSCM** | Tout fichier du crate doit être balisé MSCM (@id, @do, @layer). |
| C-RDV-4 | **unsafe interdit** | `unsafe_code = "forbid"` dans Cargo.toml. |
| C-RDV-5 | **Timestamps ISO 8601** | Toutes les dates en ISO 8601 (chrono::Utc). |
| C-RDV-6 | **IDs UUID v4** | Tous les identifiants primaires en UUID v4. |

---

## 11. Métriques de succès

| Métrique | Objectif |
|----------|---------|
| Taux de conversion (visiteur → réservation) | > 60 % |
| Taux d'annulation | < 15 % |
| Réduction des no-show (avec rappels) | > 70 % |
| Temps de réservation (3 clics) | < 45 secondes |
| Satisfaction client | > 4 / 5 |
| Satisfaction pro | > 4 / 5 |
| Uptime de la page de réservation | > 99,5 % |

---

## 12. Glossaire JayRDV

| Terme | Définition |
|-------|-----------|
| **Service** | Prestation proposée par le professionnel (ex. « Coupe homme 30 min ») |
| **Créneau (Slot)** | Fenêtre temporelle disponible pour un service donné |
| **Hold** | Verrouillage temporaire d'un créneau pendant que le client confirme |
| **Rendez-vous (Appointment)** | Réservation confirmée d'un créneau par un client |
| **Praticien (Practitioner)** | Collaborateur du professionnel qui réalise la prestation |
| **Ressource** | Salle, équipement ou véhicule nécessaire à une prestation |
| **Round Robin** | Distribution automatique des RDV entre praticiens |
| **No-show** | Client qui ne se présente pas au RDV |
| **Guest** | Client qui réserve sans créer de compte (identification par téléphone + OTP) |
| **Buffer** | Temps tampon entre deux RDV (préparation, nettoyage) |
| **Liste d'attente (Waitlist)** | File de clients en attente d'un créneau libéré |
| **JayBobo** | Service dédié au domaine médical (hors périmètre JayRDV) |

---

## 13. Références

| Document | Rôle |
|----------|------|
| [JayRDV - Document Fondateur](./JayRDV%20-%20Document%20Fondateur.md) | Vision, raison d'être |
| [Equivalents Réservation RDV en Ligne](../../reference/equivalents/Miyukini%20Conceptual%20References%20-%20Equivalents%20Reservation%20RDV%20en%20Ligne.md) | Benchmark concurrentiel (Planity, Doctolib, Treatwell, Booksy, Fresha, Calendly, Cal.com, SimplyBook) |
| [Accessibilité Client et Parcours](./JayRDV%20-%20Accessibilite%20Client%20et%20Parcours%20de%20Reservation.md) | Stratégies d'accès (web, PWA, mini COG, fédération) |
| [Professionnels — Analyse des besoins](./publics/Professionnels/Professionnels%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels détaillés (PRO-01 à PRO-50+) |
| [Clients — Analyse des besoins](./publics/Clients/Clients%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels détaillés (CLI-01 à CLI-40+) |
| [Professionnels — Opérateurs et Toolkits](./publics/Professionnels/Professionnels%20-%20Operateurs%20et%20Toolkits.md) | JayRDV Pro, JayRDV Exposition, Kits (Calendrier, Services, Notifications, Stats, Équipe) |
| [Jay1Tribu — Todo Audit](../Jay1Tribu/Jay1Tribu%20-%20Todo%20Audit.md) | Sync JayXpose / JayKoa / JayRDV |
| [Audit JayRDV](../../implementation/Miyukini%20COG%20-%20Audit%20JayRDV.md) | État actuel du crate jayrdv |
| [Suivi Audit et Todo](../../implementation/Miyukini%20COG%20-%20Suivi%20Audit%20et%20Todo.md) | Suivi global du projet |

---

**Document** : JayRDV — Spécification Complète du Service  
**Version** : 1.0  
**Date** : 2026-02-16  
**Statut** : Document de référence normatif — source de vérité pour l'implémentation
