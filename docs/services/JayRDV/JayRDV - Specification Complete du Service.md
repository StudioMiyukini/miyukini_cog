# JayRDV â€” SpÃ©cification ComplÃ¨te du Service

## Contexte

**JayRDV** est le service Miyukini dÃ©diÃ© Ã  la **prise de rendez-vous et Ã  la rÃ©servation en ligne pour les professionnels du service**. Il couvre l'ensemble du cycle de vie d'un rendez-vous : exposition des crÃ©neaux, flux de rÃ©servation, gestion des annulations, rappels, synchronisation avec les agendas et la vitrine du professionnel.

Ce document est la **spÃ©cification exhaustive** du service : vision, pÃ©rimÃ¨tre, modÃ¨le de donnÃ©es, flux, Ã©crans, intÃ©grations, gouvernance, contraintes et roadmap.

**Exclusion formelle :** Le domaine **mÃ©dical** (praticiens de santÃ©, dossier patient, ordonnances, tÃ©lÃ©consultation mÃ©dicale, conformitÃ© RGPD santÃ©, HDS) est **exclu** de JayRDV. Un service dÃ©diÃ©, **JayBobo**, couvrira les spÃ©cificitÃ©s mÃ©dicales (fiche patient, historique mÃ©dical, consentement Ã©clairÃ©, hÃ©bergement donnÃ©es de santÃ©, intÃ©gration carte Vitale, etc.).

---

## PortÃ©e / Scope

| Ã‰lÃ©ment | Couvert | Exclus |
|---------|---------|--------|
| **Secteurs** | BeautÃ©, bien-Ãªtre, coaching, conseil, artisanat, restauration, immobilier, services Ã  la personne, formation, Ã©vÃ©nementiel, services B2B | MÃ©dical (â†’ JayBobo) |
| **Publics** | Professionnels, Clients (avec ou sans compte), Utilisateurs non connectÃ©s (guest) | Patients (â†’ JayBobo) |
| **Fonctionnel** | RÃ©servation, crÃ©neaux, plannings, rappels, annulations, paiement, statistiques, Ã©quipe, widget, vitrine | Dossier patient, ordonnance, tÃ©lÃ©consultation mÃ©dicale, carte Vitale |
| **Architecture** | Crate `jayrdv`, intÃ©gration JayXpose, JayKoa, MiyuBooking, MiyuNotify, Cores COG | UI finale (â†’ design system Central / Tauri) |

**Audience :** Ã‰quipes produit, architecture, dÃ©veloppement, design.

**RÃ©fÃ©rences :**
- [Document Fondateur](./JayRDV%20-%20Document%20Fondateur.md)
- [Equivalents RÃ©servation RDV en Ligne](..//..//miyukini-webway-system//reference//_index.md)
- [AccessibilitÃ© Client et Parcours de RÃ©servation](./JayRDV%20-%20Accessibilite%20Client%20et%20Parcours%20de%20Reservation.md)
- [Professionnels â€” Analyse des besoins](./publics/Professionnels/Professionnels%20-%20Analyse%20des%20besoins.md)
- [Clients â€” Analyse des besoins](./publics/Clients/Clients%20-%20Analyse%20des%20besoins.md)
- [Professionnels â€” OpÃ©rateurs et Toolkits](./publics/Professionnels/Professionnels%20-%20Operateurs%20et%20Toolkits.md)

---

## 1. Vision et positionnement

### 1.1 Proposition de valeur

JayRDV permet Ã  tout **professionnel du service** de :

1. **Proposer des crÃ©neaux de rÃ©servation en ligne** accessibles 24h/24 7j/7, sans intervention manuelle.
2. **GÃ©rer son calendrier, ses services et ses ressources** (personnes, salles, Ã©quipements) depuis un espace professionnel gouvernÃ©.
3. **RÃ©duire les rendez-vous manquÃ©s** (no-show) grÃ¢ce Ã  des rappels multi-niveaux (SMS, email, push).
4. **Offrir un parcours client en 3 clics** : choisir un service â†’ sÃ©lectionner un crÃ©neau â†’ confirmer.
5. **Garder la souverainetÃ© de ses donnÃ©es** : toute la donnÃ©e reste dans le COG du professionnel (KindMother), pas dans un cloud tiers.

### 1.2 Positionnement concurrentiel

| Concurrent | ModÃ¨le | JayRDV se diffÃ©rencie par |
|-----------|--------|--------------------------|
| Planity | SaaS cloud, beautÃ©/bien-Ãªtre | SouverainetÃ© des donnÃ©es, multi-secteur (pas limitÃ© beautÃ©) |
| Calendly | SaaS cloud, B2B/freelance | InterpolaritÃ© COG (JayKoa, JayXpose, JayKonta), offline-first |
| Fresha | SaaS gratuit, beautÃ© | Pas de dÃ©pendance cloud, gouvernance Cores |
| Treatwell | Marketplace + SaaS | Pas de commission, donnÃ©es chez le pro |
| Cal.com | Open source, self-hosted | IntÃ©gration native Ã©cosystÃ¨me Miyukini, Cores de gouvernance |

### 1.3 Secteurs cibles

| Secteur | Exemples de professionnels |
|---------|---------------------------|
| **BeautÃ© / Bien-Ãªtre** | Coiffeurs, barbiers, esthÃ©ticiennes, masseurs, spas |
| **Coaching / Conseil** | Coachs sportifs, consultants, avocats, experts-comptables |
| **Artisanat / Services** | Plombiers, Ã©lectriciens, photographes, architectes d'intÃ©rieur |
| **Restauration** | Restaurants (tables), traiteurs, food trucks |
| **Immobilier** | Agents immobiliers (visites), diagnostiqueurs |
| **Formation** | Formateurs, Ã©coles (cours, ateliers, stages) |
| **Ã‰vÃ©nementiel** | Organisateurs (crÃ©neaux de participation), prestataires |
| **Services B2B** | Agences, studios, prestataires IT (dÃ©mos, appels) |

---

## 2. SÃ©paration JayRDV / JayBobo

### 2.1 Principe

Le domaine mÃ©dical impose des **contraintes rÃ©glementaires, sÃ©curitaires et fonctionnelles** qui justifient un service dÃ©diÃ© :

| Dimension | JayRDV (Service) | JayBobo (MÃ©dical) |
|-----------|------------------|-------------------|
| **DonnÃ©es** | Fiche client (nom, email, tÃ©lÃ©phone, prÃ©fÃ©rences) | Fiche patient (antÃ©cÃ©dents, allergies, prescriptions, consentement Ã©clairÃ©) |
| **RÃ©glementation** | RGPD standard | RGPD santÃ© + HDS (HÃ©bergement DonnÃ©es de SantÃ©) + codes de dÃ©ontologie |
| **IdentitÃ©** | Compte Miyauth ou guest (tÃ©lÃ©phone + OTP) | IdentitÃ© vÃ©rifiÃ©e (carte Vitale, INS, identifiant national) |
| **TÃ©lÃ©consultation** | Visio gÃ©nÃ©raliste (optionnel) | TÃ©lÃ©consultation mÃ©dicale (cadre lÃ©gal, prescriptions, compte-rendu) |
| **Rappels** | SMS/email/push configurables | Rappels + consignes prÃ©-consultation (jeÃ»ne, documents Ã  apporter) |
| **Paiement** | Acompte / paiement libre | Tiers payant, mutuelle, dÃ©passement d'honoraires |
| **IntÃ©grations** | JayXpose (vitrine), JayKoa (agenda), MiyuBooking, MiyuNotify | Logiciels mÃ©tier santÃ©, DMP, protocoles HL7/FHIR |
| **SÃ©curitÃ©** | WorrySentinel standard (S1-S3) | Niveaux de sÃ©curitÃ© renforcÃ©s (S3-S5), chiffrement bout-en-bout obligatoire |

### 2.2 Interfaces communes

JayRDV et JayBobo **partagent** :
- Le moteur de crÃ©neaux (MiyuBooking) : slots, disponibilitÃ©s, hold, anti-double-booking.
- Le moteur de rappels (MiyuNotify) : SMS, email, push.
- L'agenda unifiÃ© (JayKoa) : reflets des RDV confirmÃ©s.
- La vitrine (JayXpose) : page publique du professionnel.

JayBobo **ajoute** une couche de conformitÃ© santÃ© au-dessus de ces briques communes.

---

## 3. Architecture et intÃ©grations inter-services

### 3.1 Vue d'ensemble

```
JayXpose (vitrine professionnel)
    â”‚ Alimente : profil pro, services proposÃ©s, page publique, widget
    â–¼
JayRDV (service de rÃ©servation)
    â”‚ Orchestre : Ã©crans, vues, flux de rÃ©servation
    â”‚ CRUD : services, crÃ©neaux, rendez-vous
    â”‚ GÃ¨re : annulations, modifications, rappels, paiement
    â”‚ Utilise : MiyuBooking (moteur crÃ©neaux), MiyuNotify (rappels)
    â”‚           MiyuAuth (identitÃ©), MiyuProfile (profil client)
    â”‚           MiyuInvoice (paiement/acompte)
    â–¼
JayKoa (agenda universel)
    â”‚ ReÃ§oit : reflets des RDV confirmÃ©s (lecture seule)
    â”‚ Organise : agenda unifiÃ© par utilisateur
    â–¼
JayKonta (comptabilitÃ© â€” optionnel)
    â””â”€â”€ ReÃ§oit : Ã©critures comptables si paiement (factures, acomptes)
```

### 3.2 RÃ´le de chaque brique

| Brique | Strate | RÃ´le dans JayRDV |
|--------|--------|-----------------|
| **JayXpose** | Service (7) | Alimente JayRDV avec les **informations du professionnel** : prÃ©sentation, offre de services, photos, horaires, lien public, widget. JayXpose est la **vitrine** ; JayRDV en consomme les donnÃ©es pour configurer ce que le client voit. |
| **JayKoa** | Service (7) | **Organise les agendas** de chacun. AgrÃ¨ge et reflÃ¨te les engagements temporels. ReÃ§oit les RDV confirmÃ©s de JayRDV en reflets (lecture seule) via `JayRDVAdapter::sync_appointments_from_store`. |
| **MiyuBooking** | Toolkit (6) | **Moteur de crÃ©neaux** : calcul des disponibilitÃ©s, hold temporaire, anti-double-booking, buffers, prÃ©avis min/max. |
| **MiyuNotify** | Toolkit (6) | **Rappels et notifications** : SMS, email, push. Envoi de confirmations, rappels J-7/J-1/H-2, alertes pro (nouveau RDV, annulation). |
| **MiyuAuth** | Toolkit (6) | **IdentitÃ©** : rÃ©solution, vÃ©rification, attestation. Gestion du compte client ou guest (OTP tÃ©lÃ©phone). |
| **MiyuProfile** | Toolkit (6) | **Profil** : donnÃ©es client (nom, prÃ©fÃ©rences, historique) et profil professionnel. |
| **MiyuInvoice** | Toolkit (6) | **Paiement** : gÃ©nÃ©ration de factures, capture d'acomptes, remboursements. |
| **MiyuStore** | Toolkit (6) | **Boutique** : cartes cadeaux, cures, packs de sÃ©ances (optionnel). |
| **KindMother** | Core (4) | **Persistance** : toutes les donnÃ©es (RDV, crÃ©neaux, clients, services) stockÃ©es localement dans le COG du pro. |
| **StrongFather** | Core (4) | **Gouvernance** : Ã©valuation des intentions (rÃ©servation, annulation), mandats de permission. |
| **BorderGuard** | Core (4) | **SÃ©curitÃ© pÃ©rimÃ©trique** : filtrage des requÃªtes publiques, niveaux S1-S2, rate limiting, anti-spam. |
| **WorrySentinel** | Core (4) | **SÃ©curitÃ©** : niveaux de sÃ©curitÃ©, dÃ©tection de menaces, dÃ©gradation gracieuse. |

### 3.3 Flux JayXpose â†’ JayRDV

JayXpose fournit Ã  JayRDV :

| DonnÃ©e | Source JayXpose | Usage JayRDV |
|--------|----------------|-------------|
| Profil professionnel | Nom, photo, description, coordonnÃ©es | AffichÃ© sur la page de rÃ©servation |
| Services proposÃ©s | Catalogue de prestations (nom, durÃ©e, prix, photo) | Liste des services rÃ©servables |
| Horaires d'ouverture | Plages horaires par jour | Base pour le calcul des crÃ©neaux disponibles |
| Page publique | URL, template, branding (couleurs, logo) | Habillage de la page de rÃ©servation |
| Widget | Code embed, paramÃ¨tres de personnalisation | Widget intÃ©grable sur le site du pro |

**Synchronisation :** Lecture rÃ©flÃ©chie (adaptateur JayXpose â†’ JayRDV) ou API directe. JayRDV ne modifie jamais les donnÃ©es JayXpose ; JayXpose est la source de vÃ©ritÃ© pour la prÃ©sentation du pro.

### 3.4 Flux JayRDV â†’ JayKoa

JayRDV envoie Ã  JayKoa :

| Ã‰vÃ©nement | DonnÃ©es | Type de reflet |
|-----------|---------|----------------|
| RDV confirmÃ© | id, titre, start_at, end_at, location, client_name | `EntryType::ReflectJayRDV` |
| RDV annulÃ© | id, statut annulÃ© | Suppression ou mise Ã  jour du reflet |
| RDV modifiÃ© | id, nouvelles dates | Mise Ã  jour du reflet |

**Principe :** JayKoa ne crÃ©e jamais de RDV ; il reflÃ¨te. JayRDV est la source de vÃ©ritÃ© pour les rendez-vous.

---

## 4. ModÃ¨le de donnÃ©es

### 4.1 EntitÃ©s principales

#### 4.1.1 Professional (profil professionnel)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique |
| name | String | Nom affichÃ© (personne ou entreprise) |
| slug | String | Identifiant URL-friendly (ex. Â« marie-coiffure-paris Â») |
| description | Option\<String\> | PrÃ©sentation courte |
| sector | String | Secteur d'activitÃ© (beautÃ©, coaching, artisanatâ€¦) |
| photo_url | Option\<String\> | URL photo de profil |
| contact_email | String | Email de contact |
| contact_phone | Option\<String\> | TÃ©lÃ©phone |
| address | Option\<String\> | Adresse physique |
| timezone | String | Fuseau horaire (ex. Â« Europe/Paris Â») |
| settings | ProfessionalSettings | ParamÃ¨tres (politique annulation, prÃ©avis, bufferâ€¦) |
| created_at | DateTime | Horodatage de crÃ©ation |
| updated_at | DateTime | Horodatage de derniÃ¨re modification |

#### 4.1.2 Service (prestation proposÃ©e)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique |
| professional_id | UUID | RÃ©fÃ©rence au professionnel |
| name | String | Nom de la prestation (ex. Â« Coupe homme Â») |
| description | Option\<String\> | Description dÃ©taillÃ©e |
| duration_min | u32 | DurÃ©e en minutes |
| price | Option\<Decimal\> | Tarif (optionnel) |
| deposit | Option\<Decimal\> | Montant de l'acompte (optionnel) |
| category | Option\<String\> | CatÃ©gorie (ex. Â« Coiffure Â», Â« Soin Â») |
| photo_url | Option\<String\> | Photo illustrative |
| buffer_before_min | u32 | Temps tampon avant (minutes) |
| buffer_after_min | u32 | Temps tampon aprÃ¨s (minutes) |
| min_notice_hours | u32 | PrÃ©avis minimum (heures) |
| max_advance_days | u32 | RÃ©servation maximum Ã  l'avance (jours) |
| active | bool | Actif / inactif |
| resource_ids | Vec\<UUID\> | Ressources requises |
| practitioner_ids | Vec\<UUID\> | Praticiens habilitÃ©s |
| created_at | DateTime | |
| updated_at | DateTime | |

#### 4.1.3 Practitioner (praticien / collaborateur)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique |
| professional_id | UUID | RÃ©fÃ©rence au professionnel (Ã©tablissement) |
| name | String | Nom affichÃ© |
| role | PractitionerRole | Admin, Gestionnaire, Praticien |
| photo_url | Option\<String\> | Photo |
| service_ids | Vec\<UUID\> | Services que ce praticien peut rÃ©aliser |
| created_at | DateTime | |
| updated_at | DateTime | |

#### 4.1.4 Resource (salle, Ã©quipement)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique |
| professional_id | UUID | |
| name | String | Nom (ex. Â« Salle 1 Â», Â« Appareil laser Â») |
| kind | Option\<String\> | Type (room, equipment, vehicleâ€¦) |
| capacity | Option\<u32\> | CapacitÃ© (pour cours/ateliers) |
| created_at | DateTime | |
| updated_at | DateTime | |

#### 4.1.5 Schedule (planning rÃ©current)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| owner_type | ScheduleOwner | Practitioner ou Resource |
| owner_id | UUID | RÃ©fÃ©rence au praticien ou Ã  la ressource |
| day_of_week | u8 | 0 (lundi) Ã  6 (dimanche) |
| start_time | Time | Heure de dÃ©but |
| end_time | Time | Heure de fin |
| active | bool | |

#### 4.1.6 Exception (congÃ©s, absences, fermetures)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| owner_type | ScheduleOwner | Practitioner, Resource ou Professional (global) |
| owner_id | UUID | |
| date | Date | Jour de l'exception |
| start_time | Option\<Time\> | DÃ©but (si partiel) ; None = journÃ©e entiÃ¨re |
| end_time | Option\<Time\> | Fin (si partiel) |
| reason | Option\<String\> | Motif (congÃ©s, formation, maintenanceâ€¦) |

#### 4.1.7 Slot (crÃ©neau calculÃ© / proposÃ©)

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
| slot_id | UUID | CrÃ©neau rÃ©servÃ© |
| client_id | Option\<UUID\> | Si client avec compte |
| client_name | String | |
| client_email | Option\<String\> | |
| client_phone | String | |
| notes | Option\<String\> | Remarques client |
| status | AppointmentStatus | Pending, Confirmed, Cancelled, Completed, NoShow |
| cancellation_reason | Option\<String\> | Motif d'annulation (si applicable) |
| cancelled_by | Option\<CancelledBy\> | Client ou Professional |
| paid_amount | Option\<Decimal\> | Montant payÃ© (acompte ou total) |
| cancel_token | String | Token unique pour annulation par lien |
| created_at | DateTime | |
| updated_at | DateTime | |

#### 4.1.9 Reminder (rappel programmÃ©)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| appointment_id | UUID | |
| channel | ReminderChannel | SMS, Email, Push |
| scheduled_at | DateTime | Date/heure d'envoi prÃ©vue |
| sent | bool | DÃ©jÃ  envoyÃ© |
| sent_at | Option\<DateTime\> | Date/heure effective d'envoi |

#### 4.1.10 Client (fiche client)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | |
| professional_id | UUID | Fiche propre Ã  chaque pro (isolation) |
| name | String | |
| email | Option\<String\> | |
| phone | String | |
| notes | Option\<String\> | Notes internes du pro |
| tags | Vec\<String\> | Ã‰tiquettes (VIP, fidÃ¨leâ€¦) |
| total_appointments | u32 | Compteur de RDV |
| last_appointment_at | Option\<DateTime\> | Dernier RDV |
| no_show_count | u32 | Compteur de no-show |
| created_at | DateTime | |
| updated_at | DateTime | |

### 4.2 Ã‰numÃ©rations

```rust
enum AppointmentStatus { Pending, Confirmed, Cancelled, Completed, NoShow }
enum SlotStatus { Available, Held, Booked, Blocked }
enum CancelledBy { Client, Professional }
enum ReminderChannel { Sms, Email, Push }
enum PractitionerRole { Admin, Manager, Practitioner }
enum ScheduleOwner { Practitioner, Resource, Professional }
```

### 4.3 ParamÃ¨tres professionnels (ProfessionalSettings)

| ParamÃ¨tre | Type | DÃ©faut | Description |
|-----------|------|--------|-------------|
| cancellation_policy_hours | u32 | 24 | DÃ©lai minimum avant RDV pour annuler gratuitement |
| allow_guest_booking | bool | true | Autoriser la rÃ©servation sans compte |
| require_phone_verification | bool | true | OTP tÃ©lÃ©phone obligatoire |
| hold_duration_min | u32 | 10 | DurÃ©e du verrouillage de crÃ©neau (minutes) |
| default_buffer_before_min | u32 | 0 | Buffer par dÃ©faut avant chaque RDV |
| default_buffer_after_min | u32 | 0 | Buffer par dÃ©faut aprÃ¨s chaque RDV |
| default_min_notice_hours | u32 | 2 | PrÃ©avis minimum par dÃ©faut |
| default_max_advance_days | u32 | 90 | RÃ©servation max Ã  l'avance |
| reminder_j7 | bool | false | Rappel Ã  J-7 |
| reminder_j1 | bool | true | Rappel Ã  J-1 |
| reminder_h2 | bool | true | Rappel Ã  H-2 |
| reminder_channels | Vec\<ReminderChannel\> | [Sms, Email] | Canaux de rappel |
| round_robin_enabled | bool | false | Distribution Round Robin |
| round_robin_strategy | String | "balanced" | StratÃ©gie RR (balanced, ordered, weighted) |
| waitlist_enabled | bool | false | Liste d'attente si crÃ©neaux pleins |
| deposit_required | bool | false | Acompte obligatoire |
| payment_at_booking | bool | false | Paiement intÃ©gral Ã  la rÃ©servation |

---

## 5. Flux de rÃ©servation

### 5.1 Parcours client (3 clics)

```
1. DÃ‰COUVERTE
   Client accÃ¨de Ã  la page de rÃ©servation via :
   - Lien direct (email, SMS, rÃ©seaux sociaux)
   - Widget sur le site du pro (via JayXpose)
   - QR code (carte de visite, salle d'attente)
   - Recherche sur la marketplace (si activÃ©e)

2. SÃ‰LECTION DU SERVICE (Clic 1)
   - Liste des services actifs du pro (nom, durÃ©e, prix, photo)
   - CatÃ©gories pour organiser (si nombreux services)
   - SÃ©lection d'un service

3. SÃ‰LECTION DU CRÃ‰NEAU (Clic 2)
   - Calendrier avec crÃ©neaux disponibles uniquement
   - Choix du praticien (si multi-praticiens) ou Â« Premier disponible Â»
   - Vue jour ou semaine
   - Hold automatique (10 min par dÃ©faut) dÃ¨s la sÃ©lection
   - Si crÃ©neau expirÃ© : notification Â« Ce crÃ©neau n'est plus disponible Â»
     + proposition des 3 prochains crÃ©neaux

4. IDENTIFICATION + CONFIRMATION (Clic 3)
   - Formulaire minimal : nom, tÃ©lÃ©phone (obligatoire), email (optionnel)
   - Si client avec compte : prÃ©-rempli, connexion en 1 clic
   - Si guest : OTP tÃ©lÃ©phone pour vÃ©rification
   - Remarques optionnelles
   - Acompte / paiement si configurÃ© par le pro
   - Bouton Â« Confirmer le rendez-vous Â»

5. POST-CONFIRMATION
   - Ã‰cran de confirmation (rÃ©capitulatif complet)
   - Email de confirmation avec :
     - RÃ©capitulatif (service, date, heure, lieu, praticien)
     - Lien Â« Ajouter Ã  mon agenda Â» (iCal / Google / Outlook)
     - Lien Â« Modifier ou annuler mon RDV Â» (token unique)
   - SMS de confirmation (si tÃ©lÃ©phone fourni)
   - Notification au professionnel (nouveau RDV)
```

### 5.2 Verrouillage de crÃ©neau (Hold)

```
Client sÃ©lectionne crÃ©neau
    â†“
Slot.status = Held, held_until = now + hold_duration_min
    â†“
Client a 10 min (configurable) pour confirmer
    â†“
Si confirmation â†’ Slot.status = Booked, Appointment crÃ©Ã©
Si expiration   â†’ Slot.status = Available (libÃ©rÃ© automatiquement)
Si erreur       â†’ Slot.status = Available, client notifiÃ©
```

**Anti-double-booking :** Le hold est atomique (transaction KindMother). Deux clients ne peuvent pas hold le mÃªme crÃ©neau. Le second reÃ§oit Â« CrÃ©neau indisponible Â» + alternatives.

### 5.3 Annulation

#### Par le client

```
Client clique lien Â« Annuler Â» (dans email/SMS, token unique)
    â†“
VÃ©rification token (BorderGuard)
    â†“
VÃ©rification dÃ©lai (> cancellation_policy_hours avant le RDV ?)
    â†“
OUI â†’ Appointment.status = Cancelled, cancelled_by = Client
       Slot.status = Available (libÃ©rÃ©)
       SMS/email confirmation annulation au client
       Notification au pro (crÃ©neau libÃ©rÃ©)
       Si waitlist_enabled â†’ notification au premier de la liste d'attente

NON â†’ Message Â« Annulation impossible, dÃ©lai dÃ©passÃ©.
       Contactez le professionnel directement. Â»
       OU facturation de frais d'annulation (si configurÃ©)
```

#### Par le professionnel

```
Pro annule depuis son agenda (Ã©cran pro)
    â†“
Appointment.status = Cancelled, cancelled_by = Professional
Slot.status = Available (libÃ©rÃ©)
    â†“
SMS/email au client : Â« Votre RDV du [date] a Ã©tÃ© annulÃ© par [pro].
    Souhaitez-vous reprogrammer ? [Lien rÃ©servation] Â»
Remboursement acompte si applicable
```

### 5.4 Modification

```
Client clique lien Â« Modifier Â» (dans email/SMS, token unique)
    â†“
VÃ©rification token + dÃ©lai
    â†“
Affichage du calendrier avec crÃ©neaux disponibles (comme nouvelle rÃ©servation)
    â†“
Client sÃ©lectionne nouveau crÃ©neau â†’ Hold
    â†“
Confirmation â†’ Ancien slot libÃ©rÃ©, nouveau slot rÃ©servÃ©
               Appointment mis Ã  jour (nouvelles dates)
               Email/SMS de confirmation modification
               Reflet JayKoa mis Ã  jour
```

### 5.5 Rappels

| Timing | Canal | Contenu |
|--------|-------|---------|
| **ImmÃ©diat** | Email + SMS | Confirmation de RDV (rÃ©capitulatif + liens annuler/modifier + ajout agenda) |
| **J-7** (optionnel) | Email | Â« Rappel : RDV dans 7 jours avec [pro] le [date] Ã  [heure] Â» |
| **J-1** | SMS + Email | Â« Rappel : RDV demain Ã  [heure] avec [pro]. [Lien annuler] Â» |
| **H-2** | SMS ou Push | Â« RDV dans 2 heures avec [pro] Ã  [lieu] Â» |
| **Post-RDV** (optionnel) | Email | Â« Merci pour votre visite ! [Lien avis] [Lien reprendre RDV] Â» |

Tous les rappels sont programmÃ©s via **MiyuNotify** et stockÃ©s dans l'entitÃ© `Reminder`. Le timing et les canaux sont configurables par le professionnel dans `ProfessionalSettings`.

---

## 6. Ã‰crans et vues

### 6.1 Ã‰crans client (FaÃ§ade publique)

| Ã‰cran | Description | DonnÃ©es affichÃ©es |
|-------|-------------|-------------------|
| **Page de rÃ©servation** | Page publique du pro (via JayXpose) avec bouton Â« RÃ©server Â» | Nom, photo, description, adresse, horaires |
| **SÃ©lection service** | Liste des prestations | Nom, durÃ©e, prix, photo, catÃ©gorie |
| **SÃ©lection crÃ©neau** | Calendrier interactif | CrÃ©neaux disponibles (vue jour/semaine), choix praticien |
| **Formulaire rÃ©servation** | Saisie infos client + confirmation | Nom, tÃ©lÃ©phone, email, remarques, acompte |
| **Confirmation** | RÃ©capitulatif + liens | Service, date, heure, lieu, praticien, liens annuler/modifier/agenda |
| **Mes RDV** (si compte) | Liste des RDV passÃ©s et Ã  venir | Statut, date, service, praticien, actions (annuler, modifier, re-rÃ©server) |
| **Annulation** | Ã‰cran d'annulation via token | Confirmation annulation + proposition de reprogrammation |
| **Modification** | SÃ©lection nouveau crÃ©neau | MÃªme flux que sÃ©lection crÃ©neau |

### 6.2 Ã‰crans professionnel (Espace pro)

| Ã‰cran | Description | FonctionnalitÃ©s |
|-------|-------------|-----------------|
| **Dashboard** | Vue d'ensemble de l'activitÃ© | RDV du jour, de la semaine ; indicateurs (total, no-show, taux remplissage) ; alertes |
| **Agenda** | Calendrier complet | Vue jour/semaine/mois ; filtres par praticien/ressource ; drag & drop ; code couleur par service/statut |
| **Services** | Gestion des prestations | CRUD (nom, durÃ©e, prix, photo, catÃ©gorie) ; activation/dÃ©sactivation ; association praticiens/ressources |
| **Plannings** | Horaires et exceptions | Horaires rÃ©currents par praticien/ressource ; exceptions (congÃ©s, pauses) ; jours fÃ©riÃ©s |
| **Clients** | Fichier client | Liste, recherche, fiche dÃ©taillÃ©e (historique, notes, tags, no-show) |
| **Ã‰quipe** | Gestion des collaborateurs | Invitation, rÃ´les (Admin/Gestionnaire/Praticien), association services |
| **Ressources** | Salles et Ã©quipements | CRUD ; plannings ; association services |
| **Notifications** | Configuration des rappels | Canaux (SMS, email, push), timing, modÃ¨les personnalisables |
| **IntÃ©grations** | Lien de rÃ©servation et widget | GÃ©nÃ©ration URL, code embed, bouton rÃ©seaux sociaux, prÃ©visualisation |
| **Statistiques** | Pilotage | RDV par pÃ©riode, taux de remplissage, taux de no-show, crÃ©neaux populaires ; export CSV/PDF |
| **ParamÃ¨tres** | Configuration gÃ©nÃ©rale | Politique d'annulation, prÃ©avis, buffers, paiement, round-robin, liste d'attente |

---

## 7. Gouvernance COG

### 7.1 Niveaux de sÃ©curitÃ©

| Niveau | Qui | Actions autorisÃ©es |
|--------|-----|--------------------|
| **S1** (Observation) | Utilisateur non connectÃ© | Consulter la page de rÃ©servation, voir les crÃ©neaux disponibles |
| **S2** (Interaction contrÃ´lÃ©e) | Client (guest ou avec compte) | RÃ©server, annuler, modifier (avec OTP ou token) |
| **S3** (OpÃ©ration gouvernÃ©e) | Professionnel (Praticien) | GÃ©rer ses propres RDV, consulter ses statistiques |
| **S4** (Administration) | Professionnel (Gestionnaire, Admin) | CRUD services, plannings, Ã©quipe, paramÃ¨tres, statistiques globales |

### 7.2 Flux gouvernÃ© (rÃ©servation)

```
Intention de rÃ©servation (client)
    â†“
BorderGuard : rate limiting, anti-spam, validation niveau S2
    â†“
StrongFather : Ã©valuation de l'intention (crÃ©neau disponible ? politique respectÃ©e ?)
    â†“
MiyuBooking : vÃ©rification slot + hold atomique
    â†“
KindMother : persistance Appointment + Slot + Client + Reminder
    â†“
MiyuNotify : envoi confirmation (SMS + email)
    â†“
JayKoa : crÃ©ation reflet (sync_appointments_from_store)
```

### 7.3 Anti-spam et anti-abus

| Menace | Protection |
|--------|-----------|
| Spam de rÃ©servations | Rate limiting : max 3 rÃ©servations / IP / heure |
| Fausses rÃ©servations | OTP tÃ©lÃ©phone (SMS) avant confirmation |
| Bot scraping | CAPTCHA optionnel ; crÃ©neaux visibles uniquement aprÃ¨s interaction |
| Double booking | Hold atomique (transaction KindMother) |
| Abus d'annulation | Compteur no-show par client ; blocage temporaire si > seuil |
| DDoS | Cloudflare / reverse proxy devant le COG (si exposÃ© web) |

---

## 8. Cas d'usage dÃ©taillÃ©s

### 8.1 RÃ©servation standard (guest)

1. Paul ouvre `https://marie-coiffure.jayrdv.fr` (lien partagÃ© sur Instagram).
2. Voit la page vitrine de Marie (photo, services, avis). Clique Â« RÃ©server Â».
3. SÃ©lectionne Â« Coupe homme â€” 30 min â€” 25 EUR Â».
4. Voit le calendrier : mercredi 10h30 disponible. SÃ©lectionne. Hold de 10 min.
5. Saisit : Paul Dupont, 06 12 34 56 78. ReÃ§oit un SMS OTP. Le valide.
6. Clique Â« Confirmer Â». RDV confirmÃ©.
7. ReÃ§oit : SMS + email de confirmation avec lien annulation et ajout agenda.
8. J-1 : reÃ§oit un rappel SMS.
9. Paul se prÃ©sente au salon. Marie valide le RDV (statut â†’ Completed).

### 8.2 RÃ©servation avec choix de praticien

1. Sophie ouvre la page de Â« Centre Zen â€” Spa & Massages Â».
2. SÃ©lectionne Â« Massage relaxant â€” 60 min â€” 70 EUR Â».
3. Voit la liste des praticiens : Julie, Marc, Â« Premier disponible Â».
4. Choisit Julie. Voit les crÃ©neaux de Julie uniquement.
5. SÃ©lectionne vendredi 14h. Hold.
6. Confirme (a un compte client : formulaire prÃ©-rempli).
7. Paiement d'un acompte de 20 EUR (si configurÃ©).
8. Confirmation avec reÃ§u de paiement.

### 8.3 Annulation par le client

1. Paul reÃ§oit le SMS rappel J-1 : Â« RDV demain 10h30. Pour annuler : [lien] Â».
2. Paul clique le lien. Page d'annulation affichÃ©e.
3. DÃ©lai > 24h â†’ annulation autorisÃ©e.
4. Paul confirme l'annulation.
5. CrÃ©neau libÃ©rÃ©. Marie notifiÃ©e (Â« Paul a annulÃ© son RDV de demain Â»).
6. Si liste d'attente active : premier de la liste notifiÃ© (Â« Un crÃ©neau s'est libÃ©rÃ© ! Â»).

### 8.4 Round Robin

1. Cabinet de 3 consultants (A, B, C). Round robin activÃ© (stratÃ©gie Â« balanced Â»).
2. Client rÃ©serve Â« Consultation 45 min Â». Ne choisit pas de praticien.
3. JayRDV vÃ©rifie les compteurs : A a 12 RDV cette semaine, B a 10, C a 11.
4. Attribution Ã  B (le moins chargÃ©).
5. Client voit : Â« RDV avec B le [date] Ã  [heure] Â».

### 8.5 Cours / atelier collectif

1. Formateur propose Â« Atelier aquarelle â€” 2h â€” 8 places max Â».
2. 5 clients ont dÃ©jÃ  rÃ©servÃ©. Client 6 voit Â« 3 places restantes Â».
3. Client 6 rÃ©serve. Places restantes : 2.
4. Client 9 arrive : Â« Complet â€” Rejoindre la liste d'attente ? Â».
5. Si place se libÃ¨re (annulation) â†’ notification au premier de la liste d'attente.

### 8.6 Gestion offline (COG hors ligne)

1. COG du pro est hors ligne (coupure internet).
2. Client accÃ¨de Ã  la page de rÃ©servation via le buffer (Cloudflare Workers).
3. Voit les crÃ©neaux snapshot (derniÃ¨re synchro).
4. Soumet une demande de rÃ©servation â†’ mise en buffer.
5. SMS : Â« Demande reÃ§ue. Confirmation sous quelques heures. Â»
6. COG revient en ligne â†’ sync buffer â†’ Ã©valuation StrongFather â†’ confirmation ou conflit.
7. SMS final : confirmation ou proposition alternative.

---

## 9. Roadmap d'implÃ©mentation

### 9.1 Phase 1 â€” MVP (v0.1)

| Livrable | DÃ©tail |
|----------|--------|
| Types domaine | Appointment, Slot, Resource, Service, Client, Reminder, AppointmentStatus |
| Store mÃ©moire | CRUD en mÃ©moire (v0), migration KindMother/legacy-sqlite ensuite |
| Domain layer | appointment_create, appointment_set_status, slot_create, resource_create, reminder_create |
| IntÃ©gration JayKoa | sync_appointments_from_store (reflets RDV confirmÃ©s) |
| Balisage MSCM | @id, @do, @layer sur tous les modules |
| Tests | Unitaires (store + domain) + intÃ©gration (sync JayKoa) |

**Statut actuel :** Types, store mÃ©moire, domain, intÃ©gration JayKoa = **fait**. MSCM et tests = **Ã  faire**.

### 9.2 Phase 2 â€” Persistance et services

| Livrable | DÃ©tail |
|----------|--------|
| Feature `legacy-sqlite` | Structure data/ alignÃ©e sur les autres services (kindmother_db.rs) |
| Feature `kindmother-only` | DÃ©lÃ©gation KindMother Client |
| EntitÃ©s Ã©tendues | Professional, Service, Practitioner, Schedule, Exception, Client |
| Hold de crÃ©neau | SlotStatus::Held, expiration automatique |
| IntÃ©gration MiyuBooking | Calcul des crÃ©neaux disponibles, anti-double-booking |
| IntÃ©gration MiyuNotify | Rappels SMS/email, confirmation, alertes pro |
| IntÃ©gration JayXpose | Lecture rÃ©flÃ©chie du profil pro et des services |

### 9.3 Phase 3 â€” Interface web publique (StratÃ©gie 1)

| Livrable | DÃ©tail |
|----------|--------|
| Page de rÃ©servation | Interface web responsive (MiyuWeb + JayRDV) |
| Parcours Â« 3 clics Â» | SÃ©lection service â†’ crÃ©neau â†’ confirmation |
| OTP tÃ©lÃ©phone | VÃ©rification SMS (MiyuAuth) |
| Widget / embed | Code intÃ©grable sur le site du pro (JayXpose) |
| Lien de rÃ©servation | URL unique par professionnel |
| Buffer offline | Cloudflare Workers / snapshot JSON si COG offline |

### 9.4 Phase 4 â€” FonctionnalitÃ©s avancÃ©es

| Livrable | DÃ©tail |
|----------|--------|
| Multi-praticien | Round Robin (balanced, ordered, weighted) |
| Ressources | Association service â†” ressource, anti-double-booking ressource |
| Cours / ateliers | Places limitÃ©es, liste d'attente |
| Statistiques | Dashboard pro (RDV, remplissage, no-show, export) |
| Paiement | Acompte / paiement intÃ©gral (MiyuInvoice) |
| Cartes cadeaux / cures | MiyuStore (packs de sÃ©ances) |
| Offres heures creuses | CrÃ©neaux Ã  tarif rÃ©duit / derniÃ¨re minute |

### 9.5 Phase 5 â€” PWA et mobile

| Livrable | DÃ©tail |
|----------|--------|
| PWA installable | manifest.json, Service Worker, cache offline |
| Notifications push | Web Push API |
| App mobile pro | Gestion agenda + RDV depuis smartphone |

---

## 10. Contraintes et invariants

### 10.1 Invariants fonctionnels

| Id | Invariant | Description |
|----|-----------|-------------|
| INV-RDV-1 | **Pas de double booking** | Un crÃ©neau ne peut Ãªtre rÃ©servÃ© qu'une seule fois par praticien ET par ressource. |
| INV-RDV-2 | **Hold atomique** | Le verrouillage d'un crÃ©neau est transactionnel ; deux clients ne peuvent pas hold le mÃªme crÃ©neau. |
| INV-RDV-3 | **DÃ©lai d'annulation** | Le client ne peut annuler que si le dÃ©lai configurÃ© par le pro est respectÃ©. |
| INV-RDV-4 | **DonnÃ©es chez le pro** | Toutes les donnÃ©es (RDV, clients, plannings) sont persistÃ©es dans le COG du professionnel (KindMother), jamais dans un cloud tiers. |
| INV-RDV-5 | **JayRDV ne dÃ©cide pas du temps** | JayRDV gÃ¨re les crÃ©neaux et les rÃ©servations ; l'agenda unifiÃ© est gÃ©rÃ© par JayKoa (reflets uniquement). |
| INV-RDV-6 | **JayXpose est la source de vÃ©ritÃ© du pro** | JayRDV consomme les donnÃ©es professionnel de JayXpose (profil, services, page) sans les modifier. |
| INV-RDV-7 | **Pas de donnÃ©e mÃ©dicale** | JayRDV ne stocke ni ne traite de donnÃ©es de santÃ©. Le domaine mÃ©dical est rÃ©servÃ© Ã  JayBobo. |
| INV-RDV-8 | **Rappels configurables** | Le professionnel contrÃ´le les canaux et le timing des rappels. Aucun rappel n'est envoyÃ© sans sa configuration. |
| INV-RDV-9 | **Guest autorisÃ©** | La rÃ©servation sans compte est autorisÃ©e par dÃ©faut (configurable). |
| INV-RDV-10 | **Isolation des donnÃ©es** | Les donnÃ©es d'un professionnel ne sont jamais visibles par un autre professionnel. |

### 10.2 Contraintes techniques

| Id | Contrainte | Description |
|----|-----------|-------------|
| C-RDV-1 | **Persistance via KindMother** | Toute Ã©criture passe par KindMother (legacy-sqlite ou kindmother-only). |
| C-RDV-2 | **Gouvernance Cores** | Toute action publique (rÃ©servation, annulation) passe par BorderGuard â†’ StrongFather. |
| C-RDV-3 | **ConformitÃ© MSCM** | Tout fichier du crate doit Ãªtre balisÃ© MSCM (@id, @do, @layer). |
| C-RDV-4 | **unsafe interdit** | `unsafe_code = "forbid"` dans Cargo.toml. |
| C-RDV-5 | **Timestamps ISO 8601** | Toutes les dates en ISO 8601 (chrono::Utc). |
| C-RDV-6 | **IDs UUID v4** | Tous les identifiants primaires en UUID v4. |

---

## 11. MÃ©triques de succÃ¨s

| MÃ©trique | Objectif |
|----------|---------|
| Taux de conversion (visiteur â†’ rÃ©servation) | > 60 % |
| Taux d'annulation | < 15 % |
| RÃ©duction des no-show (avec rappels) | > 70 % |
| Temps de rÃ©servation (3 clics) | < 45 secondes |
| Satisfaction client | > 4 / 5 |
| Satisfaction pro | > 4 / 5 |
| Uptime de la page de rÃ©servation | > 99,5 % |

---

## 12. Glossaire JayRDV

| Terme | DÃ©finition |
|-------|-----------|
| **Service** | Prestation proposÃ©e par le professionnel (ex. Â« Coupe homme 30 min Â») |
| **CrÃ©neau (Slot)** | FenÃªtre temporelle disponible pour un service donnÃ© |
| **Hold** | Verrouillage temporaire d'un crÃ©neau pendant que le client confirme |
| **Rendez-vous (Appointment)** | RÃ©servation confirmÃ©e d'un crÃ©neau par un client |
| **Praticien (Practitioner)** | Collaborateur du professionnel qui rÃ©alise la prestation |
| **Ressource** | Salle, Ã©quipement ou vÃ©hicule nÃ©cessaire Ã  une prestation |
| **Round Robin** | Distribution automatique des RDV entre praticiens |
| **No-show** | Client qui ne se prÃ©sente pas au RDV |
| **Guest** | Client qui rÃ©serve sans crÃ©er de compte (identification par tÃ©lÃ©phone + OTP) |
| **Buffer** | Temps tampon entre deux RDV (prÃ©paration, nettoyage) |
| **Liste d'attente (Waitlist)** | File de clients en attente d'un crÃ©neau libÃ©rÃ© |
| **JayBobo** | Service dÃ©diÃ© au domaine mÃ©dical (hors pÃ©rimÃ¨tre JayRDV) |

---

## 13. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayRDV - Document Fondateur](./JayRDV%20-%20Document%20Fondateur.md) | Vision, raison d'Ãªtre |
| [Equivalents RÃ©servation RDV en Ligne](..//..//miyukini-webway-system//reference//_index.md) | Benchmark concurrentiel (Planity, Doctolib, Treatwell, Booksy, Fresha, Calendly, Cal.com, SimplyBook) |
| [AccessibilitÃ© Client et Parcours](./JayRDV%20-%20Accessibilite%20Client%20et%20Parcours%20de%20Reservation.md) | StratÃ©gies d'accÃ¨s (web, PWA, mini COG, fÃ©dÃ©ration) |
| [Professionnels â€” Analyse des besoins](./publics/Professionnels/Professionnels%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels dÃ©taillÃ©s (PRO-01 Ã  PRO-50+) |
| [Clients â€” Analyse des besoins](./publics/Clients/Clients%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels dÃ©taillÃ©s (CLI-01 Ã  CLI-40+) |
| [Professionnels â€” OpÃ©rateurs et Toolkits](./publics/Professionnels/Professionnels%20-%20Operateurs%20et%20Toolkits.md) | JayRDV Pro, JayRDV Exposition, Kits (Calendrier, Services, Notifications, Stats, Ã‰quipe) |
| [Jay1Tribu â€” Todo Audit](../Jay1Tribu/Jay1Tribu%20-%20Todo%20Audit.md) | Sync JayXpose / JayKoa / JayRDV |
| [Audit JayRDV](../../implementation/Miyukini%20COG%20-%20Audit%20JayRDV.md) | Ã‰tat actuel du crate jayrdv |
| [Suivi Audit et Todo](..//..//_index.md) | Suivi global du projet |

---

**Document** : JayRDV â€” SpÃ©cification ComplÃ¨te du Service  
**Version** : 1.0  
**Date** : 2026-02-16  
**Statut** : Document de rÃ©fÃ©rence normatif â€” source de vÃ©ritÃ© pour l'implÃ©mentation


