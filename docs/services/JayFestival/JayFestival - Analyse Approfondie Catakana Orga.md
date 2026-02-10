# JayFestival - Analyse Approfondie Catakana Orga

## Contexte

Ce document présente l'analyse exhaustive du proto-service **Catakana Orga**, une application TypeScript/React/Supabase de gestion de festivals actuellement en production. Cette analyse sert de référence pour l'implémentation de **JayFestival** dans l'écosystème Miyukini COG (Rust/Dioxus).

**Source analysée** : `/Catakana_Orga` (février 2026)

## Portée / Scope

- **Architecture technique complète** de Catakana Orga
- **Fonctionnalités exhaustives** avec parcours utilisateur
- **Modèles de données** et schémas SQL
- **Patterns UI/UX** et design system
- **Recommandations d'adaptation** vers JayFestival (Rust/Dioxus)
- **Priorisation** des features pour implémentation progressive

## Table des matières

1. [Vue d'ensemble](#1-vue-densemble)
2. [Stack technique](#2-stack-technique)
3. [Modèles de données](#3-modeles-de-donnees)
4. [Architecture UI/UX](#4-architecture-uiux)
5. [Fonctionnalités par module](#5-fonctionnalites-par-module)
6. [Parcours utilisateur](#6-parcours-utilisateur)
7. [Recommandations JayFestival](#7-recommandations-jayfestival)
8. [Priorisation implémentation](#8-priorisation-implementation)

---

## 1. Vue d'ensemble

### 1.1 Présentation

**Catakana Orga** est une application web B2B de gestion complète du festival Catakana. Elle centralise :

- Gestion des **éditions** (cycle complet du festival)
- Gestion des **exposants** (annuaire permanent + participations)
- **Budget** par édition (revenus/dépenses)
- **Plan interactif** avec attribution stands (Fabric.js)
- Système de **réservation générique** (animations, ateliers, jeux)
- **Programme public** et agenda interne
- **Kit de communication** et documents
- Gestion **équipe** et bénévoles

### 1.2 Architecture Globale

```
┌─────────────────────────────────────────────────┐
│            FRONTEND (Vite + React)              │
│  ┌──────────────────────────────────────────┐   │
│  │  Atomic Design (atoms → organisms)       │   │
│  │  Pages publiques + Routes admin         │   │
│  │  React Query (cache & sync)             │   │
│  └──────────────────────────────────────────┘   │
└─────────────────┬───────────────────────────────┘
                  │ Supabase Client
┌─────────────────▼───────────────────────────────┐
│         BACKEND (Supabase PostgreSQL)           │
│  ┌──────────────────────────────────────────┐   │
│  │  Tables avec Row Level Security (RLS)   │   │
│  │  Functions RPC (can_book_slot, etc.)    │   │
│  │  Storage (logos, photos, documents)     │   │
│  │  Auth (email/password)                  │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

### 1.3 Principes Clés

| Principe | Description |
|----------|-------------|
| **Mobile-First** | Breakpoint unique à 800px |
| **Édition-Centric** | Tout gravite autour d'une édition |
| **Annuaire permanent** | Exposants existent indépendamment des éditions |
| **Réservation générique** | Pattern réutilisé pour 4+ systèmes |
| **Confidentialité** | Flags `_public` par donnée exposant |
| **Archivage auto** | Snapshot au jour d'ouverture |

---

## 2. Stack technique

### 2.1 Frontend

| Outil | Version | Usage |
|-------|---------|-------|
| **Vite** | 5.4.1 | Build tool, HMR |
| **React** | 18.3.1 | UI library |
| **TypeScript** | 5.5.3 | Langage (strict mode) |
| **React Router** | 6.26.2 | Routing SPA |
| **TanStack Query** | 5.56.2 | Cache, fetching, sync |
| **React Hook Form** | 7.56.1 | Formulaires |
| **Zod** | 3.24.3 | Validation schémas |

### 2.2 UI & Styling

| Outil | Version | Usage |
|-------|---------|-------|
| **Tailwind CSS** | 3.4.11 | Utility-first CSS |
| **shadcn/ui** | - | Component library |
| **Radix UI** | - | Primitives accessibles |
| **Lucide React** | 0.462.0 | Icônes |
| **Framer Motion** | 11.18.2 | Animations |
| **Fabric.js** | 6.6.4 | Canvas plan interactif |

### 2.3 Backend (Supabase)

| Service | Usage |
|---------|-------|
| **PostgreSQL** | Base de données relationnelle |
| **Row Level Security** | Permissions fine-grained |
| **Functions RPC** | Logique métier côté serveur |
| **Storage** | Fichiers (logos, photos, justificatifs) |
| **Auth** | Authentification email/password |

### 2.4 Organisation du Code

```
src/
├── components/
│   ├── atoms/          # IconWrapper, Badge, Button
│   ├── molecules/      # FeatureCard, DirectoryCard
│   ├── organisms/      # Header, FeaturesGrid
│   ├── auth/           # Login, Signup
│   ├── editions/       # Components spécifiques éditions
│   ├── exhibitors/     # Components exposants
│   ├── floor-plan/     # Plan interactif
│   └── ...
├── pages/              # Pages routing
│   ├── public/         # Pages publiques
│   └── admin/          # Pages admin
├── lib/
│   └── supabase/       # Services métier
│       ├── client.ts
│       ├── editionService.ts
│       ├── exhibitorService.ts
│       └── ...
├── types/              # Types TypeScript
├── context/            # Contextes React
├── features/           # Modules fonctionnels (plan, etc.)
└── config/             # Configuration (thèmes, menus)
```

---

## 3. Modèles de données

### 3.1 Tables Centrales

#### **editions** (Éditions du festival)

```sql
CREATE TABLE editions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    theme TEXT,
    description TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    edition_type TEXT CHECK(edition_type IN ('Intérieur', 'Extérieur', 'Mixte')),
    is_active BOOLEAN NOT NULL DEFAULT false,
    -- Statut calculé
    status TEXT GENERATED ALWAYS AS (
        CASE
            WHEN end_date < CURRENT_DATE THEN 'Terminé'
            WHEN start_date <= CURRENT_DATE AND end_date >= CURRENT_DATE THEN 'En cours'
            WHEN start_date <= CURRENT_DATE + INTERVAL '7 days' THEN 'Imminente'
            ELSE 'Préparation'
        END
    ) STORED,
    -- Inscriptions exposants
    exhibitor_registration_start_date DATE,
    exhibitor_registration_end_date DATE,
    registration_status TEXT,
    -- Lieu et accès
    location_details TEXT,
    parking_info TEXT,
    access_conditions_visitors TEXT,
    access_conditions_exhibitors TEXT,
    -- Horaires visiteurs
    visitor_hours_saturday TEXT, -- Format: "HH:MM - HH:MM"
    visitor_hours_sunday TEXT,
    -- Horaires montage
    setup_start_datetime TIMESTAMPTZ,
    setup_end_datetime TIMESTAMPTZ,
    -- Capacités
    total_stands INTEGER,
    visitor_count_target INTEGER,
    visitor_count_actual INTEGER,
    visitor_count_saturday INTEGER,
    visitor_count_sunday INTEGER,
    -- Besoins
    volunteer_needs JSONB,
    security_needs TEXT,
    -- Points-clés (présentation publique)
    key_point_1 TEXT,
    key_point_2 TEXT,
    key_point_3 TEXT,
    key_point_4 TEXT,
    key_point_5 TEXT,
    key_point_6 TEXT,
    key_point_7 TEXT,
    key_point_8 TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Une seule édition active à la fois
CREATE UNIQUE INDEX idx_editions_active ON editions(is_active) WHERE is_active = true;
```

**Points clés** :
- `is_active` : Une seule édition active (affichée sur pages publiques)
- `status` : Calculé automatiquement (Terminé, En cours, Imminente, Préparation)
- `key_point_1..8` : Points-clés pour communication publique

#### **exposants** (Annuaire permanent)

```sql
CREATE TABLE exposants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- Identification
    company_name TEXT,
    stand_name TEXT NOT NULL,
    contact_first_name TEXT NOT NULL,
    contact_last_name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone TEXT NOT NULL,
    -- Flags confidentialité
    email_public BOOLEAN NOT NULL DEFAULT false,
    phone_public BOOLEAN NOT NULL DEFAULT false,
    address_public BOOLEAN NOT NULL DEFAULT false,
    manager_last_name_public BOOLEAN NOT NULL DEFAULT false,
    -- Identité visuelle
    logo_url TEXT,
    photo_url TEXT,
    website TEXT,
    portfolio TEXT,
    facebook TEXT,
    instagram TEXT,
    twitter TEXT,
    -- Informations légales
    legal_form TEXT,
    status_name TEXT,
    siret TEXT,
    manager_first_name TEXT,
    manager_last_name TEXT,
    -- Adresse
    address TEXT,
    address_number TEXT,
    address_complement TEXT,
    postal_code TEXT,
    city TEXT,
    -- Activité
    activity_type TEXT NOT NULL,
    category TEXT NOT NULL,
    booth_description TEXT,
    products_description TEXT,
    -- Paiement
    credit_card_accepted BOOLEAN NOT NULL DEFAULT false,
    paypal_url TEXT,
    restaurant BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**Points clés** :
- Annuaire **permanent** (indépendant des éditions)
- Flags `*_public` : Contrôle visibilité données sensibles
- Lié à `profiles.id` (authentification)

#### **edition_exposants** (Participations)

```sql
CREATE TABLE edition_exposants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    edition_id UUID NOT NULL REFERENCES editions(id) ON DELETE CASCADE,
    profile_id UUID NOT NULL REFERENCES exposants(id) ON DELETE CASCADE,
    -- Statuts workflow
    status TEXT NOT NULL DEFAULT 'pending',
    is_accepted BOOLEAN NOT NULL DEFAULT false,
    is_validated BOOLEAN NOT NULL DEFAULT false,
    is_paid BOOLEAN NOT NULL DEFAULT false,
    -- Réservation
    amount NUMERIC,
    assigned_stand TEXT,
    size_meters TEXT,
    depth TEXT,
    -- Équipements
    tables INTEGER,
    chairs INTEGER,
    grid BOOLEAN NOT NULL DEFAULT false,
    power_needed BOOLEAN NOT NULL DEFAULT false,
    power_wattage TEXT,
    special_requests TEXT,
    -- Suivi
    exhibitor_pass INTEGER DEFAULT 1,
    last_nfc_scan TIMESTAMPTZ,
    last_qr_scan TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(edition_id, profile_id) -- Un exposant par édition
);
```

**Workflow candidature** :
1. Inscription → `status = 'pending'`
2. Admin valide → `is_accepted = true`
3. Facture générée
4. Paiement → `is_paid = true`
5. Confirmation → `is_validated = true`

#### **budget_entries** (Budget par édition)

```sql
CREATE TABLE budget_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    edition_id UUID NOT NULL REFERENCES editions(id) ON DELETE CASCADE,
    amount NUMERIC NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('income', 'expense')),
    category TEXT NOT NULL,
    subcategory TEXT,
    description TEXT NOT NULL,
    date DATE NOT NULL,
    receipt_url TEXT, -- Supabase Storage
    payment_method TEXT,
    status TEXT NOT NULL DEFAULT 'completed',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**Catégories types** :
- **Revenus** : Stands, Sponsoring, Billetterie
- **Dépenses** : Communication, Matériel, Prestataires, Sécurité

### 3.2 Système de Réservation Générique

**Pattern réutilisé pour** : Animations, Ateliers, Jeux, Prestations invités

#### **Table service (ex: catakana_animations)**

```sql
CREATE TABLE catakana_animations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    is_published BOOLEAN NOT NULL DEFAULT false,
    max_participants_per_slot INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

#### **Table créneaux (ex: catakana_animation_slots)**

```sql
CREATE TABLE catakana_animation_slots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    animation_id UUID NOT NULL REFERENCES catakana_animations(id) ON DELETE CASCADE,
    -- Type de créneau
    schedule_type TEXT NOT NULL CHECK(schedule_type IN ('one_time', 'recurring')),
    -- One-time
    date_start TIMESTAMPTZ,
    date_end TIMESTAMPTZ,
    -- Recurring
    recurring_day_of_week INTEGER, -- 0=dimanche, 6=samedi
    recurring_time_start TIME,
    recurring_time_end TIME,
    first_occurrence_date DATE,
    last_occurrence_date DATE,
    is_paused BOOLEAN NOT NULL DEFAULT false,
    -- Paramètres
    capacity INTEGER,
    price NUMERIC,
    booking_open_offset_hours INTEGER DEFAULT 24,
    booking_open_offset_minutes INTEGER DEFAULT 0,
    booking_close_offset_minutes INTEGER DEFAULT 30,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**Fenêtre de réservation** :
- **Ouverture** : `X heures/minutes` avant début (`booking_open_offset_hours/minutes`)
- **Fermeture** : `Y minutes` avant début (`booking_close_offset_minutes`, défaut 30)

#### **Table réservations (ex: catakana_animation_reservations)**

```sql
CREATE TABLE catakana_animation_reservations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slot_id UUID NOT NULL REFERENCES catakana_animation_slots(id) ON DELETE CASCADE,
    user_id UUID REFERENCES profiles(id), -- Nullable pour anonymes
    contact_name TEXT NOT NULL,
    contact_email TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'confirmed', 'rejected', 'cancelled')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

#### **Fonctions RPC**

```sql
-- Vérifie si fenêtre de réservation ouverte
CREATE FUNCTION can_book_slot(slot_id UUID) RETURNS BOOLEAN AS $$
    -- Logique de vérification des offsets
$$ LANGUAGE plpgsql;

-- Compte participants actifs (pending + confirmed)
CREATE FUNCTION get_slot_active_reservations_count(slot_id UUID) RETURNS INTEGER AS $$
    SELECT COUNT(*) FROM catakana_animation_reservations
    WHERE slot_id = $1 AND status IN ('pending', 'confirmed');
$$ LANGUAGE sql;
```

### 3.3 Autres Tables Importantes

#### **edition_team** (Équipe par édition)

```sql
CREATE TABLE edition_team (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    edition_id UUID NOT NULL REFERENCES editions(id) ON DELETE CASCADE,
    profile_id UUID NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
    local_role TEXT NOT NULL, -- Chef de zone, Accueil, Sécurité
    zone TEXT, -- Entrée, Grande Salle, Halle
    added_by UUID REFERENCES profiles(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

#### **kit_coms** (Kit de communication)

```sql
CREATE TABLE kit_coms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    edition_id UUID NOT NULL REFERENCES editions(id) ON DELETE CASCADE,
    theme TEXT,
    slogan TEXT,
    hashtags TEXT[],
    official_text TEXT,
    version TEXT,
    color_palette JSONB, -- {primary, secondary, accent}
    typography JSONB, -- {primaryFont, secondaryFont, downloadLink}
    logo_guide_url TEXT,
    qr_code_url TEXT,
    visual_assets JSONB[], -- Array de {title, url, type}
    press_releases JSONB[],
    photo_gallery_urls TEXT[],
    video_urls TEXT[],
    legal_mentions TEXT,
    credits TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

#### **docs** (Documents et règlements)

```sql
CREATE TABLE docs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    edition_id UUID REFERENCES editions(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    content TEXT NOT NULL, -- Markdown
    doc_type TEXT NOT NULL CHECK(doc_type IN ('standard', 'custom')),
    is_public BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### 3.4 Archivage Automatique

**Tables d'archive** :
- `archived_emplacements`
- `archived_stands`
- `archived_edition_exposants`

**Trigger** : Au jour `start_date`, snapshot automatique des données.

**RPC** :
```sql
-- Archive une édition
CREATE FUNCTION archive_edition_reservations(edition_id UUID) RETURNS VOID;

-- Restaure vers nouvelle édition
CREATE FUNCTION restore_archived_reservations(
    source_edition_id UUID,
    target_edition_id UUID
) RETURNS VOID;
```

---

## 4. Architecture UI/UX

### 4.1 Atomic Design

```
atoms/              Composants de base
  IconWrapper       Icône avec variantes couleur
  Badge            Pastilles de statut
  Button           Boutons shadcn/ui
  Input            Champs de formulaire
  Label            Labels
    ↓
molecules/          Composants intermédiaires
  FeatureCard       Carte de fonctionnalité (icône + texte)
  DirectoryCard     Carte annuaire avec gradient
  RoleCard          Carte rôle avec pastille
  CTACard           Carte appel à l'action
    ↓
organisms/          Sections complètes
  Header            Navigation responsive
  HeroSection       Section héro avec titre dégradé
  FeaturesGrid      Grille de features avec onglets
  DirectoryBanner   Bannière 2 cartes annuaires
  RolesGrid         Explication 4 rôles
  CTASection        3 cartes CTA
    ↓
pages/              Écrans complets
  HomePage          Page d'accueil publique
  EditionDashboard  Tableau de bord édition (admin)
  ExhibitorsList    Liste exposants
  ...
```

### 4.2 Design System

#### **Breakpoint unique**

```css
/* Mobile */
@media (max-width: 799px) { }

/* Desktop */
@media (min-width: 800px) { }
```

**Règles** :
- Mobile = portrait prioritaire
- Navigation fixe en bas (5 items max)
- Desktop = sidebar droite

#### **Thème dynamique saisonnier**

```typescript
useActiveTheme() → {
  colors: {
    background: { primary, secondary },
    section: { background, border, title, description },
    navigation: { background, text, buttonBackground },
    header: { background, text },
    text: { primary, secondary }
  },
  borders: { radius: { sm, md, lg } },
  shadows: { sm, md, lg },
  spacing: { ... },
  fonts: { primaryFont, secondaryFont, weights }
}
```

**Saisons** :
- Hiver (21 déc → 20 mars) : Bleu
- Printemps (21 mars → 20 juin) : Rose
- Été (21 juin → 22 sept) : Orange/Doré
- Automne (23 sept → 20 déc) : Orange

**Événements spéciaux** :
- Noël (25 déc) : Rouge et vert
- Pâques : Vert pastel et cyan
- Anniversaire (7 fév) : Bleu et rouge

#### **Navigation Mobile**

```
┌─────────────────────────────┐
│      Contenu principal      │
│                             │
│                             │
└─────────────────────────────┘
┌─────────────────────────────┐
│ [🏠] [📅] [👥] [⚙️] [👤]  │ ← Bottom nav fixe
└─────────────────────────────┘
```

**Règles** :
- Z-index : overlay = 34, sidebar = 35
- Un seul menu ouvert à la fois (exclusivité mutuelle)
- Fermeture auto à l'ouverture d'un autre

### 4.3 Accessibilité

| Critère | Implémentation |
|---------|----------------|
| **ARIA** | labels, controls, selected states |
| **Clavier** | focus visible, tabindex cohérent |
| **Contrastes** | Ratios WCAG respectés |
| **Screen readers** | Support VoiceOver/NVDA |

---

## 5. Fonctionnalités par module

### 5.1 Gestion des Éditions

**Tableau de bord édition** avec 14 onglets :

| # | Onglet | Description |
|---|--------|-------------|
| 1 | **Tableau de bord** | Stats (exposants, budget, matériel) |
| 2 | **Informations générales** | Dates, thème, lieu, horaires, points-clés |
| 3 | **Exposants** | Liste avec statuts (accepté, validé, payé) |
| 4 | **Programme** | Événements publics |
| 5 | **Intervenants** | Invités (dessinateurs, doubleurs, concerts) |
| 6 | **Plan** | Plan interactif Fabric.js |
| 7 | **Matériel & Prestataires** | Catalogue, listes, inventaire |
| 8 | **Kit Com** | Identité visuelle, assets, charte |
| 9 | **Budget** | Revenus/dépenses, statistiques |
| 10 | **Agenda** | Agenda interne (réunions, jalons) |
| 11 | **Documents** | Règlements (Markdown) |
| 12 | **Équipe** | Attribution rôles et zones |
| 13 | **Debrief** | Retours post-événement |
| 14 | **Archivage** | Sauvegarde/restauration |

**Statuts automatiques** :
```typescript
if (end_date < today) return 'Terminé';
if (start_date <= today && end_date >= today) return 'En cours';
if (start_date <= today + 7_days) return 'Imminente';
return 'Préparation';
```

**Règle édition active** : Une seule `is_active = true` à la fois.

### 5.2 Gestion des Exposants

#### **Annuaire permanent**

```typescript
// Structure unifiée
interface ExhibitorUnified {
  id: string;
  company_name?: string;
  stand_name: string;
  contact_first_name: string;
  contact_last_name: string;
  email: string;
  phone: string;
  // Flags confidentialité
  email_public: boolean;
  phone_public: boolean;
  address_public: boolean;
  // Identité
  logo_url?: string;
  photo_url?: string;
  website?: string;
  facebook?: string;
  instagram?: string;
  // Activité
  activity_type: string;
  category: string;
  booth_description?: string;
  products_description?: string;
  // ...
}
```

#### **Workflow candidature**

```
1. Inscription compte exposant
   ↓
2. Remplissage profil complet
   ↓
3. Sélection type de stand + options
   │  - Stand Intérieur : 80€ (table + 2 chaises)
   │  - Stand Extérieur : Gratuit
   │  - Stand Restauration : 150€ (FC1-FC5)
   │  + Options : Chaises (+5€), Tables (+10€), Grille, Électricité
   ↓
4. Soumission candidature (is_accepted = false)
   ↓
5. Validation admin (is_accepted = true)
   ↓
6. Génération facture
   ↓
7. Paiement (is_paid = true)
   ↓
8. Confirmation (is_validated = true)
```

#### **Transformateurs de confidentialité**

```typescript
function applyPrivacyRules(exhibitor: ExhibitorUnified): ExhibitorPublic {
  return {
    ...exhibitor,
    email: exhibitor.email_public ? exhibitor.email : null,
    phone: exhibitor.phone_public ? exhibitor.phone : null,
    address: exhibitor.address_public ? exhibitor.address : null,
    manager_last_name: exhibitor.manager_last_name_public
      ? exhibitor.manager_last_name
      : null,
  };
}
```

### 5.3 Plan Interactif (Fabric.js)

**Module** : `src/features/plan/`

**Composants** :
- `PlanCanvas.tsx` : Canvas Fabric.js principal
- `StandCard.tsx` : Info stand
- `StandDraggable.tsx` : Stand déplaçable
- `StandEditor.tsx` : Édition propriétés

**Fonctionnalités** :
- Création stands (rectangles, formes personnalisées)
- Déplacement drag & drop
- Rotation, redimensionnement
- Attribution exposants (liaison `exhibitorId`)
- Sauvegarde positions (JSONB)
- Export PNG
- Gestion zones (Grande Salle, Halle, Extérieurs, Foodtruck)

**Données** :
```typescript
interface StandPosition {
  x: number;
  y: number;
  width: number;
  height: number;
  rotation: number;
  type: 'booth_small' | 'booth_medium' | 'booth_large';
  exhibitorId?: string;
  isLocked: boolean;
}
```

### 5.4 Budget

**Statistiques automatiques** :
```typescript
interface BudgetStats {
  totalIncome: number;
  totalExpense: number;
  balance: number; // income - expense
  byCategory: Record<string, number>;
}
```

**Filtres** :
- Période (date début/fin)
- Type (revenu/dépense)
- Catégorie
- Recherche textuelle

**Justificatifs** : Upload vers Supabase Storage (`receipt_url`).

### 5.5 Système de Réservation

**Workflow public** :
```
1. Liste services publiés (is_published = true)
   ↓
2. Sélection service
   ↓
3. Affichage créneaux disponibles
   │  - Places restantes : capacity - active_count
   │  - Badge "Complet" si capacity atteinte
   │  - Vérification fenêtre : can_book_slot(slot_id)
   ↓
4. Formulaire réservation (nom, email)
   ↓
5. Validation auto ou attente admin
   ↓
6. Confirmation (status = 'confirmed')
```

**Types de créneaux** :
- **One-time** : Date/heure précise (`date_start`, `date_end`)
- **Recurring** : Jour + horaires (`recurring_day_of_week`, `recurring_time_start/end`)

**Fenêtre de réservation** :
```typescript
// Ouverture : X heures/minutes avant
const bookingOpenAt = slotStart - (offset_hours * 3600 + offset_minutes * 60);

// Fermeture : Y minutes avant (défaut 30)
const bookingCloseAt = slotStart - (close_offset_minutes * 60);

const now = Date.now();
return now >= bookingOpenAt && now <= bookingCloseAt;
```

### 5.6 Programme & Agenda

**Programme public** (`events`) :
- Événements et animations publiques
- Filtres : jour (Samedi/Dimanche), catégorie
- Créneaux horaires (`schedule_slots`)
- Affichage calendrier et liste

**Agenda interne** (`agenda_events`) :
- Types : Réunion, Atelier, Rencontre B2B, Grande étape, Autre
- Participants, lieu, horaires
- Synchronisation équipe

### 5.7 Kit de Communication

**Contenu** :
```typescript
interface KitCom {
  theme: string;
  slogan: string;
  hashtags: string[];
  official_text: string;
  version: string;
  color_palette: {
    primary: string;
    secondary: string;
    accent: string;
  };
  typography: {
    primaryFont: string;
    secondaryFont: string;
    downloadLink?: string;
  };
  logo_guide_url?: string;
  qr_code_url?: string;
  visual_assets: Array<{
    title: string;
    url: string;
    type: 'logo' | 'banner' | 'poster' | 'other';
  }>;
  press_releases: Array<{
    title: string;
    date: string;
    content: string;
  }>;
  photo_gallery_urls: string[];
  video_urls: string[];
  legal_mentions?: string;
  credits?: string;
}
```

### 5.8 Documents & Règlements

**Types** :
- `standard` : Règlement exposant, conditions visiteurs
- `custom` : Documents personnalisés

**Gestion** :
- Titre, slug, contenu (Markdown)
- Association à une édition
- Visibilité publique (`is_public`)

**Rendu** : `react-markdown` avec styles Tailwind Typography.

### 5.9 Débriefing Post-Édition

**Sections structurées** :
```typescript
interface EditionDebrief {
  general_comments: string;
  communication: {
    positives: string;
    difficulties: string;
    suggestions: string;
  };
  exhibitors: {
    feedback: string;
    logistics_issues: string;
    improvements: string;
  };
  logistics: {
    setup: string;
    suggestions: string;
  };
  program: {
    highlights: string;
    schedule_feedback: string;
  };
  team: {
    feedback: string;
    issues: string;
    improvements: string;
  };
  security: {
    incidents: string;
  };
  welcome: {
    feedback: string;
    signage: string;
  };
  budget: {
    remarks: string;
  };
  documents: {
    feedback: string;
  };
  summary: {
    positives: string;
    negatives: string;
    next_edition_ideas: string;
    theme_suggestion: string;
  };
}
```

### 5.10 Archivage & Restauration

**Trigger automatique** : Au jour `start_date`, snapshot des données.

**Tables archivées** :
- Emplacements (`archived_emplacements`)
- Stands (`archived_stands`)
- Participations (`archived_edition_exposants`)

**Restauration** :
```typescript
// Copie données archivées vers nouvelle édition
restoreArchivedReservations(sourceEditionId, targetEditionId);

// Réinitialisation statuts
// is_accepted, is_validated, is_paid → false
```

---

## 6. Parcours utilisateur

### 6.1 Visiteur (Public)

**Pages accessibles** :
- `/` : Accueil avec présentation
- `/news` : Actualités
- `/program` : Programme public
- `/exhibitors` : Annuaire exposants (données publiques)
- `/plan` : Plan du festival
- `/rules` : Règlements

**Parcours typique** :
```
1. Consultation page d'accueil
   ↓
2. Découverte programme et animations
   ↓
3. Réservation atelier/animation
   │  - Sélection créneau
   │  - Formulaire (nom, email)
   │  - Confirmation
   ↓
4. Consultation annuaire exposants
   │  - Filtres par catégorie
   │  - Fiche détaillée (données publiques)
   ↓
5. Visualisation plan du festival
```

### 6.2 Exposant

**Parcours inscription** :
```
1. Création compte (type: exhibitor)
   ↓
2. Remplissage profil complet
   │  - Informations générales
   │  - Identité visuelle (logo, photo)
   │  - Description activité
   │  - Configuration confidentialité (flags _public)
   ↓
3. Sélection type de stand + options
   │  - Stand Intérieur : 80€
   │  - Stand Extérieur : Gratuit
   │  - Stand Restauration : 150€
   │  + Options : Chaises, Tables, Grille, Électricité
   ↓
4. Soumission candidature
   │  - status = 'pending'
   │  - is_accepted = false
   ↓
5. Attente validation admin
   ↓
6. Réception facture
   ↓
7. Paiement
   │  - is_paid = true
   ↓
8. Confirmation participation
   │  - is_validated = true
   │  - Attribution stand (assigned_stand)
```

**Pages accessibles** :
- Toutes pages publiques
- `/account` : Gestion profil exposant
- `/my-participation` : Statut participation édition active
- `/my-stand` : Informations stand attribué
- `/my-invoices` : Factures

**Fonctionnalités** :
- Modification profil (logo, photo, description)
- Gestion visibilité données (flags `_public`)
- Upload documents
- Consultation factures
- Suivi statut candidature

### 6.3 Bénévole

**Pages accessibles** :
- Toutes pages publiques
- `/volunteer-zone` : Zones et missions
- `/candidatures` : Lecture candidatures exposants
- `/agenda` : Agenda interne (lecture)

**Fonctionnalités** :
- Lecture candidatures exposants
- Consultation plan et emplacements
- Accès documents internes

### 6.4 Manager

**Pages accessibles** :
- Toutes pages publiques + bénévole
- `/editions/:id` : Tableau de bord édition (14 onglets)
- `/gestion/exposants` : Gestion complète exposants
- `/gestion/budget` : Gestion budget
- `/gestion/programme` : Configuration programme
- `/gestion/equipe` : Attribution rôles et zones

**Fonctionnalités** :
- CRUD exposants
- Validation/rejet candidatures
- Attribution emplacements
- Création/modification entrées budget
- Gestion équipe et bénévoles
- Modération réservations

### 6.5 Admin

**Accès complet** :
- Toutes pages + modules admin
- `/admin` : Dashboard admin général
- `/admin/users` : Gestion utilisateurs (CRUD, modération)
- `/admin/editions` : CRUD éditions
- `/admin/settings` : Paramètres application

**Fonctionnalités exclusives** :
- Création/modification/suppression éditions
- Définition édition active
- Gestion utilisateurs (suspension, bannissement)
- Changement de rôles
- Simulation de rôles (RoleSimulationContext)
- Configuration notifications email
- Accès historiques complets
- Archivage et restauration

---

## 7. Recommandations JayFestival

### 7.1 Architecture Miyukini COG

**Strates concernées** :

| Strate | Composants | Rôle |
|--------|------------|------|
| **Strate 2** | KindMother | Persistance libSQL |
| **Strate 4** | MiyuAuth | Authentification |
| **Strate 6** | Toolkits | MiyuCMS, MiyuBilling, MiyuCalendar |
| **Strate 8** | JayFestival | Service utilisateur final |

**Structure suggérée** :
```
crates/jayfestival/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── data/
│   │   ├── mod.rs
│   │   ├── types.rs           # Edition, Exposant, Budget, etc.
│   │   └── kindmother_db.rs   # Client KindMother
│   ├── services/
│   │   ├── mod.rs
│   │   ├── edition_service.rs
│   │   ├── exposant_service.rs
│   │   ├── budget_service.rs
│   │   └── reservation_service.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── atoms/
│   │   ├── molecules/
│   │   ├── organisms/
│   │   └── screens/
│   └── governance.rs          # Règles StrongFather
```

### 7.2 Mapping Types TypeScript → Rust

```rust
use chrono::{NaiveDate, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Edition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edition {
    pub id: Uuid,
    pub name: String,
    pub theme: Option<String>,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub edition_type: Option<EditionType>,
    pub is_active: bool,
    pub status: EditionStatus, // Calculé
    // Inscriptions exposants
    pub exhibitor_registration_start_date: Option<NaiveDate>,
    pub exhibitor_registration_end_date: Option<NaiveDate>,
    pub registration_status: Option<String>,
    // Lieu et accès
    pub location_details: Option<String>,
    pub parking_info: Option<String>,
    pub access_conditions_visitors: Option<String>,
    pub access_conditions_exhibitors: Option<String>,
    // Horaires
    pub visitor_hours_saturday: Option<String>,
    pub visitor_hours_sunday: Option<String>,
    pub setup_start_datetime: Option<DateTime<Utc>>,
    pub setup_end_datetime: Option<DateTime<Utc>>,
    // Capacités
    pub total_stands: Option<i32>,
    pub visitor_count_target: Option<i32>,
    pub visitor_count_actual: Option<i32>,
    pub visitor_count_saturday: Option<i32>,
    pub visitor_count_sunday: Option<i32>,
    // Besoins
    pub volunteer_needs: Option<serde_json::Value>, // JSON
    pub security_needs: Option<String>,
    // Points-clés
    pub key_points: Vec<Option<String>>, // Vec de 8 éléments
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditionType {
    #[serde(rename = "Intérieur")]
    Interieur,
    #[serde(rename = "Extérieur")]
    Exterieur,
    #[serde(rename = "Mixte")]
    Mixte,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditionStatus {
    #[serde(rename = "Terminé")]
    Termine,
    #[serde(rename = "En cours")]
    EnCours,
    #[serde(rename = "Imminente")]
    Imminente,
    #[serde(rename = "Préparation")]
    Preparation,
}

impl Edition {
    /// Calcule le statut selon les dates
    pub fn calculate_status(&self) -> EditionStatus {
        let today = chrono::Local::now().date_naive();

        if self.end_date < today {
            EditionStatus::Termine
        } else if self.start_date <= today && self.end_date >= today {
            EditionStatus::EnCours
        } else if self.start_date <= today + chrono::Duration::days(7) {
            EditionStatus::Imminente
        } else {
            EditionStatus::Preparation
        }
    }
}

// Exposant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exposant {
    pub id: Uuid,
    // Identification
    pub company_name: Option<String>,
    pub stand_name: String,
    pub contact_first_name: String,
    pub contact_last_name: String,
    pub email: String,
    pub phone: String,
    // Flags confidentialité
    pub email_public: bool,
    pub phone_public: bool,
    pub address_public: bool,
    pub manager_last_name_public: bool,
    // Identité visuelle
    pub logo_url: Option<String>,
    pub photo_url: Option<String>,
    pub website: Option<String>,
    pub portfolio: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub twitter: Option<String>,
    // Légal
    pub legal_form: Option<String>,
    pub status_name: Option<String>,
    pub siret: Option<String>,
    pub manager_first_name: Option<String>,
    pub manager_last_name: Option<String>,
    // Adresse
    pub address: Option<String>,
    pub address_number: Option<String>,
    pub address_complement: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    // Activité
    pub activity_type: String,
    pub category: String,
    pub booth_description: Option<String>,
    pub products_description: Option<String>,
    // Paiement
    pub credit_card_accepted: bool,
    pub paypal_url: Option<String>,
    pub restaurant: bool,
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Exposant Public (après application règles confidentialité)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposantPublic {
    pub id: Uuid,
    pub company_name: Option<String>,
    pub stand_name: String,
    pub contact_first_name: String,
    pub contact_last_name: String,
    pub email: Option<String>, // Masqué si !email_public
    pub phone: Option<String>, // Masqué si !phone_public
    pub logo_url: Option<String>,
    pub photo_url: Option<String>,
    pub website: Option<String>,
    pub portfolio: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub twitter: Option<String>,
    pub address: Option<String>, // Masqué si !address_public
    pub activity_type: String,
    pub category: String,
    pub booth_description: Option<String>,
    pub products_description: Option<String>,
    pub credit_card_accepted: bool,
    pub paypal_url: Option<String>,
    pub restaurant: bool,
}

impl From<Exposant> for ExposantPublic {
    fn from(exp: Exposant) -> Self {
        Self {
            id: exp.id,
            company_name: exp.company_name,
            stand_name: exp.stand_name,
            contact_first_name: exp.contact_first_name,
            contact_last_name: exp.contact_last_name,
            email: if exp.email_public { Some(exp.email) } else { None },
            phone: if exp.phone_public { Some(exp.phone) } else { None },
            logo_url: exp.logo_url,
            photo_url: exp.photo_url,
            website: exp.website,
            portfolio: exp.portfolio,
            facebook: exp.facebook,
            instagram: exp.instagram,
            twitter: exp.twitter,
            address: if exp.address_public {
                exp.address.or_else(|| {
                    // Reconstruction adresse si champs disponibles
                    let parts: Vec<String> = vec![
                        exp.address_number,
                        exp.address_complement,
                        exp.postal_code,
                        exp.city,
                    ]
                    .into_iter()
                    .flatten()
                    .collect();

                    if parts.is_empty() {
                        None
                    } else {
                        Some(parts.join(" "))
                    }
                })
            } else {
                None
            },
            activity_type: exp.activity_type,
            category: exp.category,
            booth_description: exp.booth_description,
            products_description: exp.products_description,
            credit_card_accepted: exp.credit_card_accepted,
            paypal_url: exp.paypal_url,
            restaurant: exp.restaurant,
        }
    }
}

// EditionExposant (Participation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditionExposant {
    pub id: Uuid,
    pub edition_id: Uuid,
    pub profile_id: Uuid,
    // Workflow
    pub status: String,
    pub is_accepted: bool,
    pub is_validated: bool,
    pub is_paid: bool,
    // Réservation
    pub amount: Option<Decimal>,
    pub assigned_stand: Option<String>,
    pub size_meters: Option<String>,
    pub depth: Option<String>,
    // Équipements
    pub tables: Option<i32>,
    pub chairs: Option<i32>,
    pub grid: bool,
    pub power_needed: bool,
    pub power_wattage: Option<String>,
    pub special_requests: Option<String>,
    // Suivi
    pub exhibitor_pass: Option<i32>,
    pub last_nfc_scan: Option<DateTime<Utc>>,
    pub last_qr_scan: Option<DateTime<Utc>>,
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// BudgetEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetEntry {
    pub id: Uuid,
    pub edition_id: Uuid,
    pub amount: Decimal,
    pub type_: BudgetType,
    pub category: String,
    pub subcategory: Option<String>,
    pub description: String,
    pub date: NaiveDate,
    pub receipt_url: Option<String>,
    pub payment_method: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BudgetType {
    Income,
    Expense,
}

// BudgetStats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetStats {
    pub total_income: Decimal,
    pub total_expense: Decimal,
    pub balance: Decimal,
    pub by_category: std::collections::HashMap<String, Decimal>,
}
```

### 7.3 Services Rust

```rust
// crates/jayfestival/src/services/edition_service.rs
use crate::data::{Edition, types::*};
use crate::data::kindmother_db::KindMotherDb;
use std::sync::Arc;
use anyhow::{Result, Context};

pub struct EditionService {
    db: Arc<KindMotherDb>,
}

impl EditionService {
    pub fn new(db: Arc<KindMotherDb>) -> Self {
        Self { db }
    }

    /// Récupère toutes les éditions
    pub async fn get_all(&self) -> Result<Vec<Edition>> {
        self.db.get_all_editions().await
            .context("Failed to fetch editions")
    }

    /// Récupère une édition par ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Edition> {
        self.db.get_edition_by_id(id).await
            .context("Failed to fetch edition by ID")
    }

    /// Récupère l'édition active
    pub async fn get_active(&self) -> Result<Option<Edition>> {
        self.db.get_active_edition().await
            .context("Failed to fetch active edition")
    }

    /// Crée une nouvelle édition
    pub async fn create(&self, data: CreateEditionInput) -> Result<Edition> {
        // Validation
        if data.start_date >= data.end_date {
            anyhow::bail!("start_date must be before end_date");
        }

        self.db.create_edition(data).await
            .context("Failed to create edition")
    }

    /// Met à jour une édition
    pub async fn update(&self, id: Uuid, data: UpdateEditionInput) -> Result<Edition> {
        self.db.update_edition(id, data).await
            .context("Failed to update edition")
    }

    /// Définit une édition comme active (désactive les autres)
    pub async fn set_active(&self, id: Uuid) -> Result<()> {
        self.db.set_active_edition(id).await
            .context("Failed to set active edition")
    }

    /// Supprime une édition
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.db.delete_edition(id).await
            .context("Failed to delete edition")
    }
}

// crates/jayfestival/src/services/exposant_service.rs
use crate::data::{Exposant, ExposantPublic};
use crate::data::kindmother_db::KindMotherDb;
use std::sync::Arc;
use anyhow::Result;

pub struct ExposantService {
    db: Arc<KindMotherDb>,
}

impl ExposantService {
    pub fn new(db: Arc<KindMotherDb>) -> Self {
        Self { db }
    }

    /// Récupère tous les exposants (vue publique)
    pub async fn get_all_public(&self) -> Result<Vec<ExposantPublic>> {
        let exposants = self.db.get_all_exposants().await?;
        Ok(exposants.into_iter().map(|e| e.into()).collect())
    }

    /// Récupère tous les exposants (vue admin, données complètes)
    pub async fn get_all_admin(&self) -> Result<Vec<Exposant>> {
        self.db.get_all_exposants().await
    }

    /// Récupère un exposant par ID (vue publique)
    pub async fn get_public_by_id(&self, id: Uuid) -> Result<ExposantPublic> {
        let exposant = self.db.get_exposant_by_id(id).await?;
        Ok(exposant.into())
    }

    /// Récupère un exposant par ID (vue admin)
    pub async fn get_admin_by_id(&self, id: Uuid) -> Result<Exposant> {
        self.db.get_exposant_by_id(id).await
    }

    /// Crée un nouvel exposant
    pub async fn create(&self, data: CreateExposantInput) -> Result<Exposant> {
        // Validation email unique
        if self.db.exposant_email_exists(&data.email).await? {
            anyhow::bail!("Email already exists");
        }

        self.db.create_exposant(data).await
    }

    /// Met à jour un exposant
    pub async fn update(&self, id: Uuid, data: UpdateExposantInput) -> Result<Exposant> {
        self.db.update_exposant(id, data).await
    }

    /// Supprime un exposant
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.db.delete_exposant(id).await
    }

    /// Recherche exposants par catégorie
    pub async fn get_by_category(&self, category: &str) -> Result<Vec<ExposantPublic>> {
        let exposants = self.db.get_exposants_by_category(category).await?;
        Ok(exposants.into_iter().map(|e| e.into()).collect())
    }
}
```

### 7.4 Intégration KindMother

```rust
// crates/jayfestival/src/data/kindmother_db.rs
use kindmother_client::{KindMotherClient, Query, Row};
use crate::data::types::*;
use anyhow::{Result, Context};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;

pub struct KindMotherDb {
    client: KindMotherClient,
}

impl KindMotherDb {
    pub fn new(database_path: &str) -> Result<Self> {
        let client = KindMotherClient::connect(database_path)
            .context("Failed to connect to KindMother")?;
        Ok(Self { client })
    }

    // === EDITIONS ===

    pub async fn get_all_editions(&self) -> Result<Vec<Edition>> {
        let query = Query::new("SELECT * FROM editions ORDER BY start_date DESC");
        let rows = self.client.execute(query).await?;
        rows.into_iter()
            .map(|row| self.row_to_edition(row))
            .collect()
    }

    pub async fn get_edition_by_id(&self, id: Uuid) -> Result<Edition> {
        let query = Query::new("SELECT * FROM editions WHERE id = ?")
            .bind(id.to_string());
        let rows = self.client.execute(query).await?;
        let row = rows.into_iter().next()
            .context("Edition not found")?;
        self.row_to_edition(row)
    }

    pub async fn get_active_edition(&self) -> Result<Option<Edition>> {
        let query = Query::new("SELECT * FROM editions WHERE is_active = 1 LIMIT 1");
        let rows = self.client.execute(query).await?;
        rows.into_iter()
            .next()
            .map(|row| self.row_to_edition(row))
            .transpose()
    }

    pub async fn create_edition(&self, data: CreateEditionInput) -> Result<Edition> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // Désactiver toutes les éditions actives
        let deactivate = Query::new("UPDATE editions SET is_active = 0 WHERE is_active = 1");
        self.client.execute(deactivate).await?;

        // Insérer nouvelle édition
        let insert = Query::new(
            "INSERT INTO editions (
                id, name, theme, description, start_date, end_date,
                edition_type, is_active, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, 1, ?, ?) RETURNING *"
        )
        .bind(id.to_string())
        .bind(&data.name)
        .bind(data.theme.as_deref())
        .bind(data.description.as_deref())
        .bind(data.start_date.to_string())
        .bind(data.end_date.to_string())
        .bind(data.edition_type.map(|t| match t {
            EditionType::Interieur => "Intérieur",
            EditionType::Exterieur => "Extérieur",
            EditionType::Mixte => "Mixte",
        }))
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339());

        let rows = self.client.execute(insert).await?;
        let row = rows.into_iter().next()
            .context("Failed to retrieve created edition")?;
        self.row_to_edition(row)
    }

    pub async fn set_active_edition(&self, id: Uuid) -> Result<()> {
        // Désactiver toutes
        let deactivate = Query::new("UPDATE editions SET is_active = 0 WHERE is_active = 1");
        self.client.execute(deactivate).await?;

        // Activer celle-ci
        let activate = Query::new("UPDATE editions SET is_active = 1 WHERE id = ?")
            .bind(id.to_string());
        self.client.execute(activate).await?;

        Ok(())
    }

    // === EXPOSANTS ===

    pub async fn get_all_exposants(&self) -> Result<Vec<Exposant>> {
        let query = Query::new("SELECT * FROM exposants ORDER BY stand_name");
        let rows = self.client.execute(query).await?;
        rows.into_iter()
            .map(|row| self.row_to_exposant(row))
            .collect()
    }

    pub async fn get_exposant_by_id(&self, id: Uuid) -> Result<Exposant> {
        let query = Query::new("SELECT * FROM exposants WHERE id = ?")
            .bind(id.to_string());
        let rows = self.client.execute(query).await?;
        let row = rows.into_iter().next()
            .context("Exposant not found")?;
        self.row_to_exposant(row)
    }

    pub async fn exposant_email_exists(&self, email: &str) -> Result<bool> {
        let query = Query::new("SELECT COUNT(*) as count FROM exposants WHERE email = ?")
            .bind(email);
        let rows = self.client.execute(query).await?;
        let count: i64 = rows[0].get("count")?;
        Ok(count > 0)
    }

    pub async fn create_exposant(&self, data: CreateExposantInput) -> Result<Exposant> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let insert = Query::new(
            "INSERT INTO exposants (
                id, company_name, stand_name, contact_first_name, contact_last_name,
                email, phone, email_public, phone_public, address_public,
                activity_type, category, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *"
        )
        .bind(id.to_string())
        .bind(data.company_name.as_deref())
        .bind(&data.stand_name)
        .bind(&data.contact_first_name)
        .bind(&data.contact_last_name)
        .bind(&data.email)
        .bind(&data.phone)
        .bind(data.email_public as i32)
        .bind(data.phone_public as i32)
        .bind(data.address_public as i32)
        .bind(&data.activity_type)
        .bind(&data.category)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339());

        let rows = self.client.execute(insert).await?;
        let row = rows.into_iter().next()
            .context("Failed to retrieve created exposant")?;
        self.row_to_exposant(row)
    }

    // === BUDGET ===

    pub async fn get_budget_entries(&self, edition_id: Uuid) -> Result<Vec<BudgetEntry>> {
        let query = Query::new(
            "SELECT * FROM budget_entries WHERE edition_id = ? ORDER BY date DESC"
        ).bind(edition_id.to_string());

        let rows = self.client.execute(query).await?;
        rows.into_iter()
            .map(|row| self.row_to_budget_entry(row))
            .collect()
    }

    pub async fn get_budget_stats(&self, edition_id: Uuid) -> Result<BudgetStats> {
        let query = Query::new(
            "SELECT
                SUM(CASE WHEN type = 'income' THEN amount ELSE 0 END) as total_income,
                SUM(CASE WHEN type = 'expense' THEN amount ELSE 0 END) as total_expense
             FROM budget_entries
             WHERE edition_id = ?"
        ).bind(edition_id.to_string());

        let rows = self.client.execute(query).await?;
        let row = &rows[0];

        let total_income: Decimal = row.get::<f64>("total_income")?.into();
        let total_expense: Decimal = row.get::<f64>("total_expense")?.into();

        Ok(BudgetStats {
            total_income,
            total_expense,
            balance: total_income - total_expense,
            by_category: std::collections::HashMap::new(), // TODO: Query par catégorie
        })
    }

    // === HELPERS ===

    fn row_to_edition(&self, row: Row) -> Result<Edition> {
        let status_str: String = row.get("status")?;
        let status = match status_str.as_str() {
            "Terminé" => EditionStatus::Termine,
            "En cours" => EditionStatus::EnCours,
            "Imminente" => EditionStatus::Imminente,
            "Préparation" => EditionStatus::Preparation,
            _ => EditionStatus::Preparation,
        };

        Ok(Edition {
            id: Uuid::parse_str(&row.get::<String>("id")?)?,
            name: row.get("name")?,
            theme: row.get("theme").ok(),
            description: row.get("description").ok(),
            start_date: NaiveDate::parse_from_str(&row.get::<String>("start_date")?, "%Y-%m-%d")?,
            end_date: NaiveDate::parse_from_str(&row.get::<String>("end_date")?, "%Y-%m-%d")?,
            edition_type: row.get::<Option<String>>("edition_type")?
                .and_then(|s| match s.as_str() {
                    "Intérieur" => Some(EditionType::Interieur),
                    "Extérieur" => Some(EditionType::Exterieur),
                    "Mixte" => Some(EditionType::Mixte),
                    _ => None,
                }),
            is_active: row.get::<i32>("is_active")? != 0,
            status,
            // ... autres champs
            created_at: DateTime::parse_from_rfc3339(&row.get::<String>("created_at")?)?.into(),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<String>("updated_at")?)?.into(),
        })
    }

    fn row_to_exposant(&self, row: Row) -> Result<Exposant> {
        Ok(Exposant {
            id: Uuid::parse_str(&row.get::<String>("id")?)?,
            company_name: row.get("company_name").ok(),
            stand_name: row.get("stand_name")?,
            contact_first_name: row.get("contact_first_name")?,
            contact_last_name: row.get("contact_last_name")?,
            email: row.get("email")?,
            phone: row.get("phone")?,
            email_public: row.get::<i32>("email_public")? != 0,
            phone_public: row.get::<i32>("phone_public")? != 0,
            address_public: row.get::<i32>("address_public")? != 0,
            manager_last_name_public: row.get::<i32>("manager_last_name_public")? != 0,
            logo_url: row.get("logo_url").ok(),
            photo_url: row.get("photo_url").ok(),
            activity_type: row.get("activity_type")?,
            category: row.get("category")?,
            // ... autres champs
            created_at: DateTime::parse_from_rfc3339(&row.get::<String>("created_at")?)?.into(),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<String>("updated_at")?)?.into(),
        })
    }

    fn row_to_budget_entry(&self, row: Row) -> Result<BudgetEntry> {
        let type_str: String = row.get("type")?;
        let type_ = match type_str.as_str() {
            "income" => BudgetType::Income,
            "expense" => BudgetType::Expense,
            _ => BudgetType::Expense,
        };

        Ok(BudgetEntry {
            id: Uuid::parse_str(&row.get::<String>("id")?)?,
            edition_id: Uuid::parse_str(&row.get::<String>("edition_id")?)?,
            amount: row.get::<f64>("amount")?.into(),
            type_,
            category: row.get("category")?,
            subcategory: row.get("subcategory").ok(),
            description: row.get("description")?,
            date: NaiveDate::parse_from_str(&row.get::<String>("date")?, "%Y-%m-%d")?,
            receipt_url: row.get("receipt_url").ok(),
            payment_method: row.get("payment_method").ok(),
            status: row.get("status")?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<String>("created_at")?)?.into(),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<String>("updated_at")?)?.into(),
        })
    }
}
```

### 7.5 UI Dioxus Suggérée

```rust
// crates/jayfestival/src/ui/screens/edition_dashboard.rs
use dioxus::prelude::*;
use crate::data::{Edition, EditionStatus, EditionStats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditionTab {
    Dashboard,
    Informations,
    Exposants,
    Programme,
    Plan,
    Budget,
    Equipe,
    Documents,
}

#[component]
pub fn EditionDashboard(edition: Edition, stats: EditionStats) -> Element {
    let mut active_tab = use_signal(|| EditionTab::Dashboard);

    rsx! {
        div { class: "edition-dashboard",
            // En-tête
            EditionHeader { edition: edition.clone() }

            hr {}

            // Onglets
            EditionTabs { active_tab }

            hr {}

            // Contenu selon onglet actif
            match *active_tab.read() {
                EditionTab::Dashboard => rsx! { DashboardStats { stats: stats.clone() } },
                EditionTab::Informations => rsx! { EditionInfo { edition: edition.clone() } },
                EditionTab::Exposants => rsx! { ExposantsList {} },
                EditionTab::Programme => rsx! { Programme {} },
                EditionTab::Plan => rsx! { PlanInteractif {} },
                EditionTab::Budget => rsx! { Budget {} },
                EditionTab::Equipe => rsx! { Equipe {} },
                EditionTab::Documents => rsx! { Documents {} },
            }
        }
    }
}

#[component]
fn EditionHeader(edition: Edition) -> Element {
    let status_class = match edition.status {
        EditionStatus::Termine => "badge-gray",
        EditionStatus::EnCours => "badge-green",
        EditionStatus::Imminente => "badge-orange",
        EditionStatus::Preparation => "badge-blue",
    };
    let status_text = match edition.status {
        EditionStatus::Termine => "Termine",
        EditionStatus::EnCours => "En cours",
        EditionStatus::Imminente => "Imminente",
        EditionStatus::Preparation => "Preparation",
    };

    rsx! {
        div { class: "edition-header",
            h1 { "{edition.name}" }
            div { class: "edition-meta",
                span { "{edition.start_date} -> {edition.end_date}" }
                span { class: "badge {status_class}", "{status_text}" }
                if edition.is_active {
                    span { class: "badge badge-gold", "ACTIVE" }
                }
            }
            if let Some(theme) = &edition.theme {
                p { class: "edition-theme", "Theme : {theme}" }
            }
        }
    }
}

#[component]
fn EditionTabs(active_tab: Signal<EditionTab>) -> Element {
    let tabs = [
        (EditionTab::Dashboard, "Tableau de bord"),
        (EditionTab::Informations, "Informations"),
        (EditionTab::Exposants, "Exposants"),
        (EditionTab::Programme, "Programme"),
        (EditionTab::Plan, "Plan"),
        (EditionTab::Budget, "Budget"),
        (EditionTab::Equipe, "Equipe"),
        (EditionTab::Documents, "Documents"),
    ];

    rsx! {
        nav { class: "edition-tabs",
            for (tab, label) in tabs {
                button {
                    class: if *active_tab.read() == tab { "tab active" } else { "tab" },
                    onclick: move |_| active_tab.set(tab),
                    "{label}"
                }
            }
        }
    }
}

#[component]
fn DashboardStats(stats: EditionStats) -> Element {
    rsx! {
        div { class: "dashboard-stats",
            h2 { "Statistiques" }
            div { class: "stats-grid",
                div { class: "stat-card",
                    h3 { "Exposants" }
                    p { class: "stat-value", "{stats.total_exposants}" }
                    p { "Valides : {stats.exposants_valides}" }
                    p { "Payes : {stats.exposants_payes}" }
                }
                div { class: "stat-card",
                    h3 { "Budget" }
                    p { class: "stat-value", "{stats.budget_balance:.2} EUR" }
                    p { "Revenus : {stats.budget_income:.2} EUR" }
                    p { "Depenses : {stats.budget_expense:.2} EUR" }
                }
                div { class: "stat-card",
                    h3 { "Visiteurs" }
                    p { class: "stat-value", "{stats.visitor_count_target}" }
                    p { "(objectif)" }
                }
            }
        }
    }
}

#[component]
fn EditionInfo(edition: Edition) -> Element {
    rsx! {
        div { class: "edition-info",
            h2 { "Informations generales" }
            div { class: "info-card",
                p { "Nom : {edition.name}" }
                if let Some(theme) = &edition.theme {
                    p { "Theme : {theme}" }
                }
                if let Some(desc) = &edition.description {
                    p { "Description : {desc}" }
                }
                p { "Dates : {edition.start_date} -> {edition.end_date}" }
            }
        }
    }
}
```

---

## 8. Priorisation implémentation

### 8.1 Phase 1 : Fondations (MVP)

**Objectif** : Gestion de base des éditions et exposants

| Priorité | Composant | Tâches |
|----------|-----------|--------|
| **P0** | KindMother DB | Migrations SQL, Client Rust |
| **P0** | Types Rust | `Edition`, `Exposant`, `EditionExposant`, `BudgetEntry` |
| **P0** | EditionService | CRUD éditions, calcul statuts, édition active |
| **P0** | ExposantService | CRUD exposants, règles confidentialité |
| **P0** | UI Liste éditions | Tableau avec statuts, filtres, actions |
| **P0** | UI Dashboard édition | En-tête + onglets de base |
| **P0** | UI Liste exposants | Tableau avec recherche, filtres catégorie |
| **P1** | Authentification | Intégration MiyuAuth (Strate 4) |
| **P1** | Permissions | Vérifications rôles (visitor, exhibitor, manager, admin) |

**Livrables MVP** :
- Création/modification/suppression éditions
- Définition édition active
- CRUD exposants avec flags confidentialité
- Dashboard édition (version simplifiée)
- Authentification et permissions basiques

### 8.2 Phase 2 : Fonctionnalités Métier

**Objectif** : Budget, équipe, participations

| Priorité | Composant | Tâches |
|----------|-----------|--------|
| **P0** | Budget | Table `budget_entries`, BudgetService, UI statistiques |
| **P0** | Participations | Table `edition_exposants`, workflow candidature |
| **P0** | UI Exposants (détail) | Fiche exposant complète avec historique participations |
| **P1** | Équipe | Table `edition_team`, attribution rôles/zones |
| **P1** | Documents | Table `docs`, support Markdown, UI viewer |
| **P2** | Kit Com | Table `kit_coms`, version simplifiée (textes + couleurs) |

**Livrables Phase 2** :
- Budget complet avec entrées revenus/dépenses
- Workflow candidature exposant (acceptation → paiement → validation)
- Gestion équipe par édition
- Documents et règlements (Markdown)

### 8.3 Phase 3 : Avancé

**Objectif** : Réservations, plan, programme

| Priorité | Composant | Tâches |
|----------|-----------|--------|
| **P0** | Système réservation générique | Tables animations/ateliers/jeux, créneaux, réservations |
| **P0** | ReservationService | Logique créneaux one-time/recurring, fenêtre réservation |
| **P1** | Plan interactif | Alternative Dioxus à Fabric.js (drag & drop, attribution) |
| **P1** | Programme public | Table `events`, UI calendrier |
| **P2** | Archivage | Tables archivées, trigger automatique, restauration |

**Livrables Phase 3** :
- Système de réservation complet (animations, ateliers, jeux)
- Plan interactif simplifié (version Dioxus)
- Programme public avec filtres
- Archivage automatique au jour d'ouverture

### 8.4 Hors Scope (Simplifications)

**Fonctionnalités à enlever** :

| Fonctionnalité | Raison |
|----------------|--------|
| Système RPG/Gamification | Non essentiel, complexité élevée |
| Thèmes saisonniers dynamiques | UI complexe, peu de valeur métier |
| Invités avec prestations | Workflow trop spécifique, simplifiable |
| Concours | Module isolé, utilisé rarement |
| Notifications email OAuth2 Gmail | Système simplifié suffit (SMTP basique) |
| Debug UI avancée | Outils développeur standard suffisent |

**Fonctionnalités à simplifier** :

| Fonctionnalité | Simplification |
|----------------|----------------|
| Kit de communication | Version réduite : textes + palette couleurs (pas d'assets complexes) |
| Débriefing | Formulaire simple (pas de structure JSON complexe) |
| Matériel & Prestataires | Liste simple (pas d'inventaire global avec catalogue) |
| Plan interactif | Version 2D basique Dioxus (pas de rotation/zoom avancés) |

---

## Conclusion

Cette analyse exhaustive de **Catakana Orga** fournit tous les éléments nécessaires pour l'implémentation de **JayFestival** dans l'écosystème Miyukini COG.

**Points clés à retenir** :

1. **Architecture édition-centrée** : Tout gravite autour d'une édition active
2. **Annuaire permanent** : Exposants existent indépendamment des éditions
3. **Système de réservation générique** : Pattern réutilisable pour animations/ateliers/jeux
4. **Workflow candidature structuré** : Acceptation → Paiement → Validation
5. **Confidentialité fine** : Flags `_public` par donnée exposant
6. **Archivage automatique** : Snapshot au jour d'ouverture

**Recommandations d'adaptation** :

- Prioriser **MVP** (éditions + exposants + authentification)
- Implémenter **système réservation générique** en Phase 3
- **Simplifier** : Kit Com, Débriefing, Notifications
- **Enlever** : RPG, Thèmes saisonniers, Concours
- Adapter **plan interactif** en version Dioxus simplifiée

**Prochaines étapes** :

1. Valider architecture Rust/Dioxus proposée
2. Créer migrations SQL KindMother
3. Implémenter services métier (EditionService, ExposantService)
4. Développer UI MVP (Liste éditions, Dashboard, Liste exposants)
5. Itérer selon retours utilisateurs

---

**Version** : 1.0
**Date** : 9 février 2026
**Source** : Catakana Orga (TypeScript/React/Supabase)
**Cible** : JayFestival (Rust/Dioxus, Miyukini COG)
