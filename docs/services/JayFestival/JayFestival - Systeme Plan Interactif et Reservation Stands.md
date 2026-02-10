# JayFestival - Systeme Plan Interactif et Reservation Stands

## Contexte

Ce document specifie le systeme de **plan interactif** et de **reservation de stands** pour JayFestival, extrait du proto-service Catakana Orga (React/TypeScript/Supabase) et adapte a l'ecosysteme Miyukini COG (Rust/Dioxus). Il couvre :

- La **construction de plans** (editeur avec grille, positionnement relatif des stands)
- La **reservation de stands** (workflow exposant/admin, validation, limites)
- La **mise a jour en temps reel** (synchronisation multi-utilisateur)
- L'**administration manuelle** (reservation/validation/liberation par l'organisateur)

**Source analysee** : `Catakana_Orga/` — fevrier 2026 (Fabric.js, Supabase Realtime, React Query)

## Portee / Scope

| Inclus | Exclu |
|--------|-------|
| Editeur de plan (constructeur) avec grille | Visualisation 3D |
| Positionnement relatif sur surface | Rotation/zoom avances (Phase 3+) |
| Types de stands et elements d'infrastructure | Systeme de facturation (→ MiyuBilling) |
| Reservation par exposant et admin | Paiement en ligne (→ MiyuInvoice) |
| Validation workflow (3 etats) | Kit de communication |
| Temps reel multi-utilisateur | Programme / animations |
| Archivage automatique par edition | Plan multi-etage |

---

## Table des matieres

1. [Vue d'ensemble du systeme](#1-vue-densemble-du-systeme)
2. [Modele de donnees](#2-modele-de-donnees)
3. [Constructeur de plan (editeur)](#3-constructeur-de-plan-editeur)
4. [Systeme de reservation de stands](#4-systeme-de-reservation-de-stands)
5. [Temps reel et synchronisation](#5-temps-reel-et-synchronisation)
6. [Administration manuelle](#6-administration-manuelle)
7. [Architecture Rust/Dioxus](#7-architecture-rustdioxus)
8. [Schemas SQL KindMother](#8-schemas-sql-kindmother)
9. [UI Dioxus — Composants](#9-ui-dioxus--composants)
10. [Parcours utilisateur](#10-parcours-utilisateur)
11. [Regles metier et contraintes](#11-regles-metier-et-contraintes)
12. [Plan d'implementation](#12-plan-dimplementation)

---

## 1. Vue d'ensemble du systeme

### 1.1 Architecture fonctionnelle

Le systeme Plan Interactif se decompose en **deux modes** distincts :

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    PLAN INTERACTIF JayFestival                         │
├──────────────────────────────┬──────────────────────────────────────────┤
│                              │                                         │
│   MODE CONSTRUCTEUR          │   MODE EXPLOITATION                     │
│   (Organisateur)             │   (Organisateur + Exposant + Public)    │
│                              │                                         │
│   ┌──────────────────────┐   │   ┌─────────────────────────────────┐   │
│   │ Editeur de plan      │   │   │ Visualisation plan publie       │   │
│   │ ─ Grille visible     │   │   │ ─ Grille masquee               │   │
│   │ ─ Drag & drop        │   │   │ ─ Statuts colores              │   │
│   │ ─ Redimensionnement  │   │   │ ─ Info-bulles stands           │   │
│   │ ─ Types d'elements   │   │   │ ─ Temps reel                   │   │
│   │ ─ Zones nommees      │   │   │ ─ Actions selon role           │   │
│   └──────────────────────┘   │   └─────────────────────────────────┘   │
│                              │                                         │
│   ┌──────────────────────┐   │   ┌─────────────────────────────────┐   │
│   │ Proprietes element   │   │   │ Reservation / Attribution       │   │
│   │ ─ Position (x, y)    │   │   │ ─ Exposant reserve              │   │
│   │ ─ Taille (w, h)      │   │   │ ─ Admin valide/bloque          │   │
│   │ ─ Rotation           │   │   │ ─ Max 2 emplacements           │   │
│   │ ─ Verrouillage       │   │   │ ─ Changement d'emplacement     │   │
│   │ ─ Label / couleur    │   │   │ ─ Liberation                   │   │
│   └──────────────────────┘   │   └─────────────────────────────────┘   │
│                              │                                         │
└──────────────────────────────┴──────────────────────────────────────────┘
```

### 1.2 Roles et permissions

| Role | Constructeur | Reservation | Validation | Visualisation |
|------|:------------:|:-----------:|:----------:|:-------------:|
| **Organisateur (Admin)** | Oui | Oui (pour tout exposant) | Oui | Oui |
| **Organisateur (Manager)** | Oui | Oui (pour tout exposant) | Oui | Oui |
| **Exposant** | Non | Oui (pour soi, max 2) | Non | Oui |
| **Visiteur** | Non | Non | Non | Oui (plan publie) |
| **Non connecte** | Non | Non | Non | Oui (plan publie) |

### 1.3 Differrence Catakana → JayFestival

| Aspect | Catakana Orga | JayFestival |
|--------|---------------|-------------|
| Canvas | Fabric.js (JavaScript) | Dioxus Canvas 2D natif |
| Grille | Affichee en permanence | Visible **uniquement en mode constructeur** |
| Positionnement | Absolu (pixels) | **Relatif a la surface** (% ou unites metriques) |
| Temps reel | Supabase Realtime | KindMother events (libSQL + broadcast local) |
| Stockage positions | JSONB Supabase | JSONB KindMother (libSQL) |
| Plan images | Images statiques hard-codees | Plans construits dynamiquement |
| Export | PNG (simule) | PNG/SVG natif |

---

## 2. Modele de donnees

### 2.1 Diagramme entite-relation

```
┌──────────────┐       ┌──────────────────┐       ┌──────────────────┐
│   editions   │1─────N│   floor_plans    │1─────N│  plan_elements   │
│              │       │                  │       │                  │
│  id          │       │  id              │       │  id              │
│  name        │       │  edition_id  (FK)│       │  floor_plan_id   │
│  start_date  │       │  name            │       │  element_type    │
│  end_date    │       │  surface_width   │       │  label           │
│  is_active   │       │  surface_height  │       │  pos_x_pct       │
│  status      │       │  surface_unit    │       │  pos_y_pct       │
│              │       │  grid_size       │       │  width_pct       │
└──────────────┘       │  grid_visible    │       │  height_pct      │
                       │  is_published    │       │  rotation_deg    │
                       │  version         │       │  color           │
                       │  created_by      │       │  is_locked       │
                       │  created_at      │       │  z_index         │
                       │  updated_at      │       │  metadata (JSON) │
                       └──────────────────┘       │  created_at      │
                                                  │  updated_at      │
                                                  └────────┬─────────┘
                                                           │
                                                  (Si type = stand)
                                                           │
                       ┌──────────────────┐       ┌────────▼─────────┐
                       │    exposants     │1─────N│    stands        │
                       │                  │       │                  │
                       │  id              │       │  id              │
                       │  stand_name      │       │  plan_element_id │
                       │  ...             │       │  edition_id      │
                       └──────────────────┘       │  code            │
                                                  │  stand_type      │
                       ┌──────────────────┐       │  prix            │
                       │edition_exposants │       │  largeur_m       │
                       │                  │       │  longueur_m      │
                       │  id              │       │  zone            │
                       │  edition_id      │       │  equipements[]   │
                       │  exposant_id     │       │  electricite     │
                       │  assigned_stand  │───FK──│  statut          │
                       │  ...             │       │  exposant_id     │
                       └──────────────────┘       │  reserved_by     │
                                                  │  reserved_at     │
                                                  │  validated_by    │
                                                  │  validated_at    │
                                                  │  created_at      │
                                                  │  updated_at      │
                                                  └──────────────────┘
```

### 2.2 Table `floor_plans` — Plans d'implantation

| Colonne | Type | Description |
|---------|------|-------------|
| `id` | TEXT (ULID) | Identifiant unique du plan |
| `edition_id` | TEXT (FK → editions) | Edition proprietaire |
| `name` | TEXT NOT NULL | Nom du plan ("Grande Salle", "Exterieur Nord") |
| `surface_width` | REAL NOT NULL | Largeur de la surface (en `surface_unit`) |
| `surface_height` | REAL NOT NULL | Hauteur de la surface (en `surface_unit`) |
| `surface_unit` | TEXT NOT NULL DEFAULT 'm' | Unite de mesure : `m` (metres), `cm`, `ft` |
| `grid_size` | REAL NOT NULL DEFAULT 1.0 | Taille d'une cellule de grille (en `surface_unit`) |
| `grid_visible` | BOOLEAN NOT NULL DEFAULT true | Grille affichee (mode constructeur uniquement) |
| `is_published` | BOOLEAN NOT NULL DEFAULT false | Plan visible par les exposants/visiteurs |
| `background_image` | BLOB | Image de fond optionnelle (plan architecte) |
| `version` | INTEGER NOT NULL DEFAULT 1 | Numero de version (incremente a chaque sauvegarde) |
| `created_by` | TEXT (FK → users) | Createur du plan |
| `created_at` | TEXT (ISO 8601) | Date de creation |
| `updated_at` | TEXT (ISO 8601) | Derniere modification |

**Contraintes** :
- `UNIQUE(edition_id, name)` — Un seul plan par nom par edition
- Une edition peut avoir **plusieurs plans** (un par zone/batiment)

### 2.3 Table `plan_elements` — Elements du plan

| Colonne | Type | Description |
|---------|------|-------------|
| `id` | TEXT (ULID) | Identifiant unique |
| `floor_plan_id` | TEXT (FK → floor_plans) | Plan parent |
| `element_type` | TEXT NOT NULL | Type d'element (voir enum ci-dessous) |
| `label` | TEXT | Etiquette affichee |
| `pos_x_pct` | REAL NOT NULL | Position X en **pourcentage** de la surface (0.0–100.0) |
| `pos_y_pct` | REAL NOT NULL | Position Y en **pourcentage** de la surface (0.0–100.0) |
| `width_pct` | REAL NOT NULL | Largeur en **pourcentage** de la surface |
| `height_pct` | REAL NOT NULL | Hauteur en **pourcentage** de la surface |
| `rotation_deg` | REAL NOT NULL DEFAULT 0.0 | Rotation en degres (0–360) |
| `color` | TEXT | Couleur hexadecimale (#RRGGBB) |
| `is_locked` | BOOLEAN NOT NULL DEFAULT false | Verrouille (non deplacable) |
| `z_index` | INTEGER NOT NULL DEFAULT 0 | Ordre d'empilement |
| `metadata` | TEXT (JSON) | Donnees supplementaires par type |
| `created_at` | TEXT (ISO 8601) | Date de creation |
| `updated_at` | TEXT (ISO 8601) | Derniere modification |

**Types d'elements** (`element_type`) :

| Type | Categorie | Description | Couleur par defaut |
|------|-----------|-------------|---------------------|
| `stand_small` | Stand | Petit stand (1x1 unite) | `#8A2BE2` (violet) |
| `stand_medium` | Stand | Stand moyen (2x1 unites) | `#8A2BE2` |
| `stand_large` | Stand | Grand stand (2x2 unites) | `#8A2BE2` |
| `stand_xl` | Stand | Stand XL (3x2 unites) | `#8A2BE2` |
| `stand_custom` | Stand | Stand forme libre | `#8A2BE2` |
| `wall` | Infrastructure | Mur | `#4A4A4A` (gris fonce) |
| `door` | Infrastructure | Porte | `#9CA3AF` (gris clair) |
| `entrance` | Infrastructure | Entree | `#22C55E` (vert) |
| `exit` | Infrastructure | Sortie | `#EF4444` (rouge) |
| `bar` | Equipement | Bar / buvette | `#F59E0B` (jaune) |
| `stage` | Equipement | Scene | `#3B82F6` (bleu) |
| `bathroom` | Equipement | Toilettes | `#6B7280` |
| `table` | Mobilier | Table | `#D4A574` (bois) |
| `chair` | Mobilier | Chaise | `#D4A574` |
| `info_point` | Equipement | Point information | `#06B6D4` (cyan) |
| `technical_area` | Zone | Zone technique | `#F97316` (orange) |
| `reserved_area` | Zone | Zone reservee | `#DC2626` |
| `foodtruck_zone` | Zone | Zone food trucks | `#84CC16` (lime) |
| `parking` | Zone | Parking | `#78716C` |

### 2.4 Table `stands` — Stands reservables

| Colonne | Type | Description |
|---------|------|-------------|
| `id` | TEXT (ULID) | Identifiant unique |
| `plan_element_id` | TEXT (FK → plan_elements) | Element visuel sur le plan |
| `edition_id` | TEXT (FK → editions) | Edition proprietaire |
| `code` | TEXT NOT NULL | Code stand (GS_01, H1, E1, FC1) |
| `stand_type` | TEXT NOT NULL | `interieur`, `exterieur`, `restauration` |
| `prix` | REAL NOT NULL DEFAULT 0.0 | Prix en euros |
| `largeur_m` | REAL | Largeur reelle en metres |
| `longueur_m` | REAL | Longueur reelle en metres |
| `zone` | TEXT | Zone du plan ("Grande Salle", "Halle", "Exterieur") |
| `equipements` | TEXT (JSON array) | `["table", "chaises_x2", "grille", "electricite"]` |
| `electricite` | BOOLEAN NOT NULL DEFAULT false | Alimentation electrique disponible |
| `accessibilite` | TEXT | Notes accessibilite PMR |
| `statut` | TEXT NOT NULL DEFAULT 'disponible' | `disponible`, `reserve`, `valide` |
| `exposant_id` | TEXT (FK → exposants, nullable) | Exposant qui a reserve |
| `reserved_by` | TEXT (FK → users, nullable) | Qui a fait la reservation (exposant ou admin) |
| `reserved_at` | TEXT (ISO 8601, nullable) | Date de reservation |
| `validated_by` | TEXT (FK → users, nullable) | Admin qui a valide |
| `validated_at` | TEXT (ISO 8601, nullable) | Date de validation |
| `commentaire` | TEXT | Notes internes |
| `created_at` | TEXT (ISO 8601) | Date de creation |
| `updated_at` | TEXT (ISO 8601) | Derniere modification |

**Contraintes** :
- `UNIQUE(edition_id, code)` — Un code unique par edition
- `CHECK(statut IN ('disponible', 'reserve', 'valide'))` — 3 etats seulement
- `CHECK(stand_type IN ('interieur', 'exterieur', 'restauration'))` — 3 types

### 2.5 Table `stand_reservations_archive` — Archivage

| Colonne | Type | Description |
|---------|------|-------------|
| `id` | TEXT (ULID) | Identifiant unique |
| `source_stand_id` | TEXT | ID du stand original |
| `edition_id` | TEXT (FK → editions) | Edition archivee |
| `code` | TEXT | Code stand |
| `stand_type` | TEXT | Type |
| `exposant_id` | TEXT | Exposant assigne |
| `statut` | TEXT | Statut au moment de l'archivage |
| `prix` | REAL | Prix applique |
| `snapshot_data` | TEXT (JSON) | Copie integrale du stand + element du plan |
| `archived_at` | TEXT (ISO 8601) | Date d'archivage |
| `archived_by` | TEXT | Operateur ou systeme |

---

## 3. Constructeur de plan (editeur)

### 3.1 Principe du constructeur

Le constructeur est un **editeur 2D** accessible uniquement aux organisateurs (Admin/Manager). Il permet de :

1. **Definir une surface** — Dimensions reelles (metres), unite de mesure
2. **Afficher une grille d'aide** — Visible uniquement en mode edition, masquee a la publication
3. **Placer des elements** — Stands, murs, portes, equipements par drag & drop
4. **Positionner relativement** — Toutes les positions sont en **pourcentage de la surface**, pas en pixels
5. **Redimensionner et pivoter** — Poignees de redimensionnement, rotation libre ou par pas de 15°
6. **Verrouiller des elements** — Empecher le deplacement accidentel
7. **Sauvegarder par version** — Chaque sauvegarde incremente la version

### 3.2 Grille du constructeur

La grille est un **outil d'aide au positionnement**, visible **uniquement en mode constructeur** :

```
┌─────────────────────────────────────────────────────────────────────┐
│ MODE CONSTRUCTEUR : Grille visible                                  │
│                                                                     │
│  ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐   │
│  │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │
│  ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤   │
│  │   │   │ ╔═══╗ │   │   │   │   │   │   │   │   │   │   │   │   │
│  ├───┼───┼─║ S ║─┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤   │
│  │   │   │ ║ 01║ │   │   │   │   │   │   │   │   │   │   │   │   │
│  ├───┼───┼─╚═══╝─┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤   │
│  │   │   │   │   │ ╔═══════╗ │   │   │   │   │   │   │   │   │   │
│  ├───┼───┼───┼───┼─║ S02   ║─┼───┼───┼───┼───┼───┼───┼───┼───┤   │
│  │   │   │   │   │ ╚═══════╝ │   │   │   │   │   │   │   │   │   │
│  ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤   │
│  │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │
│  └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘   │
│                                                                     │
│ Grille : 1m x 1m │ Surface : 15m x 8m │ Snap: ON │ Version: 3     │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ MODE EXPLOITATION : Grille masquee                                  │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                                                             │   │
│  │       ╔═══╗                                                 │   │
│  │       ║S01║  ← Disponible (vert)                            │   │
│  │       ╚═══╝                                                 │   │
│  │               ╔═══════╗                                     │   │
│  │               ║  S02  ║  ← Reserve (orange)                 │   │
│  │               ╚═══════╝                                     │   │
│  │                                                             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│ Legende : ■ Disponible  ■ Reserve  ■ Valide                        │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.3 Positionnement relatif

**Principe fondamental** : toutes les positions sont stockees en **pourcentage de la surface**, pas en pixels. Cela garantit que le plan s'affiche correctement quelle que soit la taille de l'ecran.

| Stockage | Calcul | Exemple |
|----------|--------|---------|
| `pos_x_pct = 25.0` | Stand a 25% de la largeur depuis la gauche | Surface 15m → stand a 3.75m du bord gauche |
| `pos_y_pct = 60.0` | Stand a 60% de la hauteur depuis le haut | Surface 8m → stand a 4.8m du bord haut |
| `width_pct = 6.67` | Stand occupe 6.67% de la largeur | Surface 15m → stand de 1m de large |
| `height_pct = 12.5` | Stand occupe 12.5% de la hauteur | Surface 8m → stand de 1m de profondeur |

**Conversion position ↔ metres** :

```
position_m = (pourcentage / 100.0) × surface_dimension
pourcentage = (position_m / surface_dimension) × 100.0
```

### 3.4 Snap-to-grid (magnetisme)

En mode constructeur, l'option **Snap-to-grid** aligne automatiquement les elements sur la grille :

```
Snap ON  : pos_x_pct = round(pos_x_pct / grid_step_pct) × grid_step_pct
Snap OFF : pos_x_pct = valeur libre (precision au 0.01%)

grid_step_pct = (grid_size / surface_width) × 100.0
```

**Exemple** : Surface 15m, grille 1m → `grid_step_pct = 6.67%`

### 3.5 Outils du constructeur

| Outil | Icone | Action | Raccourci |
|-------|-------|--------|-----------|
| **Selection** | Curseur | Selectionner, deplacer, redimensionner | `V` |
| **Ajouter stand** | + Stand | Clic pour placer un nouveau stand | `S` |
| **Ajouter mur** | Ligne | Tracer un mur (clic-clic) | `W` |
| **Ajouter element** | + | Menu deroulant des types d'elements | `E` |
| **Panoramique** | Main | Alt+clic ou clic molette pour deplacer la vue | `H` |
| **Zoom** | Loupe | Molette souris, limites 10%–2000% | `Z` |
| **Verrouiller** | Cadenas | Verrouiller/deverrouiller l'element selectionne | `L` |
| **Supprimer** | Corbeille | Supprimer l'element selectionne | `Del` |
| **Dupliquer** | Copie | Dupliquer l'element selectionne | `Ctrl+D` |

### 3.6 Panneau de proprietes

Quand un element est selectionne, le panneau de proprietes affiche :

| Propriete | Controle | Description |
|-----------|----------|-------------|
| **Label** | Champ texte | Nom affiche sur l'element |
| **Position X** | Numerique + curseur | Position horizontale (m ou %) |
| **Position Y** | Numerique + curseur | Position verticale (m ou %) |
| **Largeur** | Numerique | En metres ou en cellules de grille |
| **Hauteur** | Numerique | En metres ou en cellules de grille |
| **Rotation** | Curseur 0–360° | Pas de 15° (ou libre si Shift maintenu) |
| **Couleur** | Selecteur couleur | Pour les elements non-stand |
| **Verrouille** | Toggle | Empecher le deplacement |
| **Z-index** | +/- | Ordre de superposition |
| *Si stand :* | | |
| **Code** | Texte | Code unique (GS_01, H1, etc.) |
| **Type** | Select | Interieur / Exterieur / Restauration |
| **Prix** | Numerique | Prix en euros |
| **Equipements** | Checkboxes | Table, chaises, grille, electricite |
| **Exposant** | Select / recherche | Assigner un exposant (admin) |

---

## 4. Systeme de reservation de stands

### 4.1 Etats d'un stand

```
                    ┌─────────────┐
                    │ DISPONIBLE  │  Couleur : vert (#10B981)
                    │             │  Le stand est libre
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │                         │
              ▼                         ▼
    ┌──────────────────┐     ┌──────────────────┐
    │ Exposant reserve │     │ Admin reserve    │
    │ (self-service)   │     │ (pour exposant)  │
    └────────┬─────────┘     └────────┬─────────┘
             │                        │
             ▼                        ▼
        ┌─────────────┐
        │   RESERVE   │  Couleur : orange (#F59E0B)
        │             │  En attente de validation admin
        └──────┬──────┘
               │
               ├──── Admin valide ────────────┐
               │                              │
               ▼                              │
        ┌─────────────┐                       │
        │   VALIDE    │  Couleur : bleu (#3B82F6)
        │             │  Confirmation definitive
        └─────────────┘                       │
                                              │
    ┌─────────────────────────────────────────┘
    │ A tout moment (sauf VALIDE) :
    │ ─ Exposant annule → retour DISPONIBLE
    │ ─ Admin libere → retour DISPONIBLE
    └─────────────────────────────────────────
```

### 4.2 Transitions d'etat

| Transition | Declencheur | Conditions | Effets |
|------------|-------------|------------|--------|
| Disponible → Reserve | Exposant clique "Reserver" | Max 2 stands par exposant, stand disponible | `reserved_by` = exposant, `reserved_at` = now |
| Disponible → Reserve | Admin assigne manuellement | Stand disponible | `reserved_by` = admin, `exposant_id` = cible |
| Reserve → Valide | Admin valide | Stand reserve | `validated_by` = admin, `validated_at` = now |
| Reserve → Disponible | Exposant annule | L'exposant est le reservant | Reset `exposant_id`, `reserved_by`, `reserved_at` |
| Reserve → Disponible | Admin libere | Role admin/manager | Reset complet |
| Valide → Disponible | Admin libere (exceptionnel) | Role admin | Reset complet, log audit |

### 4.3 Contrainte : maximum 2 stands par exposant

Extraite de Catakana Orga, cette regle metier limite chaque exposant a **2 emplacements maximum** par edition :

```
AVANT reservation :
  SELECT COUNT(*) FROM stands
  WHERE edition_id = ? AND exposant_id = ? AND statut IN ('reserve', 'valide')

  Si count >= 2 → REFUS avec message explicite
  Si count < 2  → AUTORISER
```

### 4.4 Changement d'emplacement

Un exposant peut changer de stand (transfert atomique) :

```
TRANSACTION :
  1. Verifier que le nouveau stand est 'disponible'
  2. Liberer l'ancien stand → statut = 'disponible', exposant_id = NULL
  3. Reserver le nouveau stand → statut = 'reserve', exposant_id = exposant
  4. Si erreur a l'etape 3 → rollback (ancien stand restaure)
```

### 4.5 Verification de disponibilite (batch)

Avant toute reservation, verifier la disponibilite en temps reel :

```
verifier_disponibilite(stand_ids: Vec<String>) → Result<DisponibiliteResult>

DisponibiliteResult {
    all_available: bool,
    unavailable: Vec<{ id, code, statut }>
}
```

---

## 5. Temps reel et synchronisation

### 5.1 Architecture temps reel

JayFestival utilise **KindMother events** pour la synchronisation en temps reel :

```
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  Organisateur A  │     │   Exposant B     │     │   Exposant C     │
│  (navigateur)    │     │   (navigateur)   │     │   (navigateur)   │
└────────┬─────────┘     └────────┬─────────┘     └────────┬─────────┘
         │                        │                        │
         │  reserve stand S01     │                        │
         ▼                        │                        │
┌────────────────────────────────────────────────────────────────────┐
│                    KindMother Service                               │
│                                                                    │
│  1. Ecrire en DB (libSQL)                                          │
│  2. Emettre evenement : StandStatusChanged { stand_id, new_status }│
│  3. Broadcast a tous les abonnes de l'edition                      │
│                                                                    │
└────────────────────────────┬───────────────────────────────────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
      ┌──────────┐   ┌──────────┐   ┌──────────┐
      │  UI A    │   │  UI B    │   │  UI C    │
      │ maj plan │   │ maj plan │   │ maj plan │
      │ (admin)  │   │ (expos.) │   │ (expos.) │
      └──────────┘   └──────────┘   └──────────┘
```

### 5.2 Evenements temps reel

| Evenement | Payload | Declencheur |
|-----------|---------|-------------|
| `StandStatusChanged` | `{ stand_id, edition_id, old_status, new_status, exposant_id }` | Reservation, validation, liberation |
| `StandPositionChanged` | `{ stand_id, floor_plan_id, pos_x_pct, pos_y_pct, width_pct, height_pct }` | Deplacement en mode constructeur |
| `PlanElementAdded` | `{ element_id, floor_plan_id, element_type }` | Ajout d'un element |
| `PlanElementRemoved` | `{ element_id, floor_plan_id }` | Suppression d'un element |
| `PlanPublished` | `{ floor_plan_id, edition_id, is_published }` | Publication/depublication |

### 5.3 Strategie de mise a jour UI

**Mise a jour granulaire** (pas de rechargement complet) :

```
On StandStatusChanged(payload) :
    stands.iter_mut()
        .find(|s| s.id == payload.stand_id)
        .map(|s| {
            s.statut = payload.new_status;
            s.exposant_id = payload.exposant_id;
        });
    // Re-render uniquement le stand concerne
```

### 5.4 Gestion des conflits

Si deux utilisateurs reservent le meme stand simultanement :

```
1. Premier arrive : reservation reussie
2. Second arrive : la DB refuse (stand plus disponible)
3. Le second recoit l'evenement StandStatusChanged
4. Son UI met a jour le stand en "Reserve" (orange)
5. Toast : "Ce stand vient d'etre reserve par un autre exposant"
```

---

## 6. Administration manuelle

### 6.1 Actions admin sur un stand

| Action | Conditions | Resultat |
|--------|------------|----------|
| **Reserver pour un exposant** | Stand disponible | Stand passe a "reserve", exposant_id assigne |
| **Valider** | Stand reserve | Stand passe a "valide", validated_by/at remplis |
| **Liberer** | Stand reserve ou valide | Stand retourne a "disponible", nettoyage complet |
| **Bloquer** | Stand disponible | Stand passe a "reserve" sans exposant (bloque) |
| **Debloquer** | Stand bloque (reserve, exposant_id = NULL) | Stand retourne a "disponible" |
| **Changer exposant** | Stand reserve ou valide | Transfert atomique vers un autre exposant |
| **Modifier proprietes** | Tout etat | Modifier prix, equipements, code, zone |

### 6.2 Interface admin du plan

L'admin dispose de controles supplementaires sur la vue plan :

```
┌─────────────────────────────────────────────────────────────────────┐
│  Plan Interactif — Edition "Catakana 2026"                         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  [Barre d'outils admin]                                             │
│  ┌─────┬──────────┬──────────┬───────────┬──────────┬────────────┐ │
│  │ Vue │ Filtrer   │ Legende  │ Exporter  │ Archiver │ Publier    │ │
│  └─────┴──────────┴──────────┴───────────┴──────────┴────────────┘ │
│                                                                     │
│  [Filtres actifs]                                                    │
│  Zone: [Toutes ▼]  Statut: [Tous ▼]  Type: [Tous ▼]                │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                                                             │   │
│  │              ╔═══╗ ← clic droit : menu contextuel           │   │
│  │              ║S01║    ├ Reserver pour...                     │   │
│  │              ╚═══╝    ├ Valider                              │   │
│  │                       ├ Liberer                              │   │
│  │        ╔═══════╗      ├ Bloquer                              │   │
│  │        ║  S02  ║      ├ Modifier proprietes                  │   │
│  │        ╚═══════╝      └ Voir historique                      │   │
│  │                                                             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  [Panneau lateral : details stand selectionne]                      │
│  ┌──────────────────────────────────┐                              │
│  │ Stand GS_01                      │                              │
│  │ Type : Interieur                 │                              │
│  │ Prix : 80 EUR                    │                              │
│  │ Statut : Reserve ■               │                              │
│  │ Exposant : Nom Exposant          │                              │
│  │ Reserve le : 2026-01-15          │                              │
│  │                                  │                              │
│  │ [Valider] [Liberer] [Changer]   │                              │
│  └──────────────────────────────────┘                              │
│                                                                     │
│  [Statistiques en bas]                                              │
│  Total: 42  │  Disponibles: 18  │  Reserves: 15  │  Valides: 9    │
└─────────────────────────────────────────────────────────────────────┘
```

### 6.3 Tableau de bord des reservations

En complement du plan visuel, un **tableau liste** pour la gestion en masse :

| Code | Zone | Type | Statut | Exposant | Prix | Reserve le | Valide le | Actions |
|------|------|------|--------|----------|------|------------|-----------|---------|
| GS_01 | Grande Salle | Interieur | Reserve | Exposant A | 80€ | 15/01 | — | [Valider] [Liberer] |
| GS_02 | Grande Salle | Interieur | Disponible | — | 80€ | — | — | [Reserver pour...] |
| H1 | Halle | Interieur | Valide | Exposant B | 80€ | 10/01 | 12/01 | [Liberer] |
| E1 | Exterieur | Exterieur | Disponible | — | 0€ | — | — | [Reserver pour...] |
| FC1 | Food Trucks | Restauration | Reserve | Traiteur C | 150€ | 20/01 | — | [Valider] [Liberer] |

**Actions en masse** :
- Selectionner plusieurs stands → Valider tous / Liberer tous
- Filtrer par zone, type, statut
- Export CSV / PDF

---

## 7. Architecture Rust/Dioxus

### 7.1 Structure des modules

```
crates/jayfestival/src/
├── plan/
│   ├── mod.rs                    # Re-exports
│   ├── types.rs                  # FloorPlan, PlanElement, Stand, enums
│   ├── plan_service.rs           # CRUD plans + elements
│   ├── stand_service.rs          # Reservation, validation, liberation
│   ├── realtime.rs               # Evenements temps reel
│   └── archive.rs                # Archivage par edition
├── ui/
│   ├── plan/
│   │   ├── mod.rs
│   │   ├── plan_canvas.rs        # Canvas 2D (rendu + interactions)
│   │   ├── plan_grid.rs          # Grille d'aide (mode constructeur)
│   │   ├── plan_element.rs       # Rendu element individuel
│   │   ├── plan_toolbar.rs       # Barre d'outils constructeur
│   │   ├── plan_properties.rs    # Panneau proprietes laterale
│   │   ├── plan_viewer.rs        # Vue lecture seule (exploitatiom)
│   │   ├── stand_info.rs         # Info-bulle stand (hover)
│   │   ├── stand_reservation.rs  # Dialog reservation exposant
│   │   ├── stand_admin.rs        # Actions admin sur stand
│   │   └── stand_table.rs        # Tableau liste des stands
│   └── ...
├── data/
│   ├── mod.rs
│   ├── plan_db.rs                # Requetes KindMother pour plans
│   └── stand_db.rs               # Requetes KindMother pour stands
└── ...
```

### 7.2 Types Rust

```rust
// crates/jayfestival/src/plan/types.rs

use serde::{Deserialize, Serialize};

// ─── Plan ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorPlan {
    pub id: String,
    pub edition_id: String,
    pub name: String,
    pub surface_width: f64,
    pub surface_height: f64,
    pub surface_unit: SurfaceUnit,
    pub grid_size: f64,
    pub grid_visible: bool,
    pub is_published: bool,
    pub background_image: Option<Vec<u8>>,
    pub version: u32,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceUnit {
    Meters,
    Centimeters,
    Feet,
}

// ─── Element du plan ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanElement {
    pub id: String,
    pub floor_plan_id: String,
    pub element_type: ElementType,
    pub label: Option<String>,
    pub pos_x_pct: f64,
    pub pos_y_pct: f64,
    pub width_pct: f64,
    pub height_pct: f64,
    pub rotation_deg: f64,
    pub color: Option<String>,
    pub is_locked: bool,
    pub z_index: i32,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementType {
    // Stands
    StandSmall,
    StandMedium,
    StandLarge,
    StandXl,
    StandCustom,
    // Infrastructure
    Wall,
    Door,
    Entrance,
    Exit,
    // Equipement
    Bar,
    Stage,
    Bathroom,
    Table,
    Chair,
    InfoPoint,
    // Zones
    TechnicalArea,
    ReservedArea,
    FoodtruckZone,
    Parking,
}

impl ElementType {
    pub fn is_stand(&self) -> bool {
        matches!(
            self,
            Self::StandSmall
                | Self::StandMedium
                | Self::StandLarge
                | Self::StandXl
                | Self::StandCustom
        )
    }

    pub fn default_color(&self) -> &str {
        match self {
            Self::StandSmall | Self::StandMedium | Self::StandLarge
            | Self::StandXl | Self::StandCustom => "#8A2BE2",
            Self::Wall => "#4A4A4A",
            Self::Door => "#9CA3AF",
            Self::Entrance => "#22C55E",
            Self::Exit => "#EF4444",
            Self::Bar => "#F59E0B",
            Self::Stage => "#3B82F6",
            Self::Bathroom => "#6B7280",
            Self::Table | Self::Chair => "#D4A574",
            Self::InfoPoint => "#06B6D4",
            Self::TechnicalArea => "#F97316",
            Self::ReservedArea => "#DC2626",
            Self::FoodtruckZone => "#84CC16",
            Self::Parking => "#78716C",
        }
    }

    /// Dimensions par defaut en pourcentage pour une surface standard
    pub fn default_size_pct(&self) -> (f64, f64) {
        match self {
            Self::StandSmall => (5.0, 5.0),
            Self::StandMedium => (10.0, 5.0),
            Self::StandLarge => (10.0, 10.0),
            Self::StandXl => (15.0, 10.0),
            Self::StandCustom => (8.0, 8.0),
            Self::Wall => (20.0, 1.0),
            Self::Door => (5.0, 1.0),
            Self::Entrance | Self::Exit => (5.0, 3.0),
            Self::Bar => (12.0, 6.0),
            Self::Stage => (20.0, 12.0),
            Self::Bathroom => (8.0, 6.0),
            Self::Table => (3.0, 3.0),
            Self::Chair => (2.0, 2.0),
            Self::InfoPoint => (4.0, 4.0),
            Self::TechnicalArea => (15.0, 10.0),
            Self::ReservedArea => (12.0, 8.0),
            Self::FoodtruckZone => (20.0, 15.0),
            Self::Parking => (25.0, 20.0),
        }
    }
}

// ─── Stand (reservation) ────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stand {
    pub id: String,
    pub plan_element_id: String,
    pub edition_id: String,
    pub code: String,
    pub stand_type: StandType,
    pub prix: f64,
    pub largeur_m: Option<f64>,
    pub longueur_m: Option<f64>,
    pub zone: Option<String>,
    pub equipements: Vec<String>,
    pub electricite: bool,
    pub accessibilite: Option<String>,
    pub statut: StandStatut,
    pub exposant_id: Option<String>,
    pub reserved_by: Option<String>,
    pub reserved_at: Option<String>,
    pub validated_by: Option<String>,
    pub validated_at: Option<String>,
    pub commentaire: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandType {
    Interieur,
    Exterieur,
    Restauration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandStatut {
    Disponible,
    Reserve,
    Valide,
}

impl StandStatut {
    pub fn color(&self) -> &str {
        match self {
            Self::Disponible => "#10B981",  // vert
            Self::Reserve => "#F59E0B",     // orange
            Self::Valide => "#3B82F6",      // bleu
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Disponible => "Disponible",
            Self::Reserve => "Reserve",
            Self::Valide => "Valide",
        }
    }
}

// ─── Evenements temps reel ──────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanEvent {
    StandStatusChanged {
        stand_id: String,
        edition_id: String,
        old_status: StandStatut,
        new_status: StandStatut,
        exposant_id: Option<String>,
    },
    StandPositionChanged {
        stand_id: String,
        floor_plan_id: String,
        pos_x_pct: f64,
        pos_y_pct: f64,
        width_pct: f64,
        height_pct: f64,
    },
    PlanElementAdded {
        element_id: String,
        floor_plan_id: String,
        element_type: ElementType,
    },
    PlanElementRemoved {
        element_id: String,
        floor_plan_id: String,
    },
    PlanPublished {
        floor_plan_id: String,
        edition_id: String,
        is_published: bool,
    },
}

// ─── Statistiques ────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StandStats {
    pub total: u32,
    pub disponibles: u32,
    pub reserves: u32,
    pub valides: u32,
    pub par_zone: Vec<(String, u32, u32, u32)>,  // (zone, dispo, reserve, valide)
    pub par_type: Vec<(StandType, u32, u32, u32)>,
}

// ─── Resultat de disponibilite ──────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisponibiliteResult {
    pub all_available: bool,
    pub unavailable: Vec<UnavailableStand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnavailableStand {
    pub id: String,
    pub code: String,
    pub statut: StandStatut,
}
```

### 7.3 StandService — Logique metier

```rust
// crates/jayfestival/src/plan/stand_service.rs

use crate::plan::types::*;
use crate::data::stand_db::StandDb;
use std::sync::Arc;

pub struct StandService {
    db: Arc<StandDb>,
}

impl StandService {
    pub fn new(db: Arc<StandDb>) -> Self {
        Self { db }
    }

    /// Reserver un stand pour un exposant
    /// Verifie : disponibilite + max 2 par exposant
    pub async fn reserver(
        &self,
        stand_id: &str,
        exposant_id: &str,
        reserved_by: &str,
    ) -> Result<Stand, StandError> {
        // 1. Verifier que le stand est disponible
        let stand = self.db.get_stand(stand_id).await?;
        if stand.statut != StandStatut::Disponible {
            return Err(StandError::StandNonDisponible {
                code: stand.code,
                statut: stand.statut,
            });
        }

        // 2. Verifier la limite de 2 stands par exposant
        let count = self.db.count_stands_exposant(
            &stand.edition_id,
            exposant_id,
        ).await?;
        if count >= 2 {
            return Err(StandError::LimiteAtteinte {
                exposant_id: exposant_id.to_string(),
                max: 2,
            });
        }

        // 3. Effectuer la reservation
        self.db.reserver_stand(stand_id, exposant_id, reserved_by).await
    }

    /// Admin valide un stand reserve
    pub async fn valider(
        &self,
        stand_id: &str,
        validated_by: &str,
    ) -> Result<Stand, StandError> {
        let stand = self.db.get_stand(stand_id).await?;
        if stand.statut != StandStatut::Reserve {
            return Err(StandError::TransitionInvalide {
                from: stand.statut,
                to: StandStatut::Valide,
            });
        }
        self.db.valider_stand(stand_id, validated_by).await
    }

    /// Liberer un stand (retour a disponible)
    pub async fn liberer(
        &self,
        stand_id: &str,
        liberated_by: &str,
    ) -> Result<Stand, StandError> {
        let stand = self.db.get_stand(stand_id).await?;
        if stand.statut == StandStatut::Disponible {
            return Err(StandError::DejaDisponible { code: stand.code });
        }
        self.db.liberer_stand(stand_id, liberated_by).await
    }

    /// Changer l'emplacement d'un exposant (transaction atomique)
    pub async fn changer_emplacement(
        &self,
        ancien_stand_id: &str,
        nouveau_stand_id: &str,
        exposant_id: &str,
    ) -> Result<(Stand, Stand), StandError> {
        // Verification prealable
        let nouveau = self.db.get_stand(nouveau_stand_id).await?;
        if nouveau.statut != StandStatut::Disponible {
            return Err(StandError::StandNonDisponible {
                code: nouveau.code,
                statut: nouveau.statut,
            });
        }

        // Transaction atomique : liberer ancien + reserver nouveau
        self.db.changer_emplacement(
            ancien_stand_id,
            nouveau_stand_id,
            exposant_id,
        ).await
    }

    /// Verifier la disponibilite de plusieurs stands
    pub async fn verifier_disponibilite(
        &self,
        stand_ids: &[String],
    ) -> Result<DisponibiliteResult, StandError> {
        self.db.verifier_disponibilite(stand_ids).await
    }

    /// Statistiques des stands pour une edition
    pub async fn get_stats(
        &self,
        edition_id: &str,
    ) -> Result<StandStats, StandError> {
        self.db.get_stats(edition_id).await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StandError {
    #[error("Stand {code} non disponible (statut: {statut:?})")]
    StandNonDisponible { code: String, statut: StandStatut },

    #[error("Exposant {exposant_id} a atteint la limite de {max} stands")]
    LimiteAtteinte { exposant_id: String, max: u32 },

    #[error("Transition invalide : {from:?} -> {to:?}")]
    TransitionInvalide { from: StandStatut, to: StandStatut },

    #[error("Stand {code} est deja disponible")]
    DejaDisponible { code: String },

    #[error("Stand non trouve : {0}")]
    NotFound(String),

    #[error("Erreur base de donnees : {0}")]
    DbError(#[from] crate::data::DbError),
}
```

### 7.4 PlanService — Gestion des plans

```rust
// crates/jayfestival/src/plan/plan_service.rs

use crate::plan::types::*;
use crate::data::plan_db::PlanDb;
use std::sync::Arc;

pub struct PlanService {
    db: Arc<PlanDb>,
}

impl PlanService {
    pub fn new(db: Arc<PlanDb>) -> Self {
        Self { db }
    }

    /// Creer un nouveau plan pour une edition
    pub async fn create_plan(
        &self,
        edition_id: &str,
        name: &str,
        surface_width: f64,
        surface_height: f64,
        surface_unit: SurfaceUnit,
        grid_size: f64,
        created_by: &str,
    ) -> Result<FloorPlan, PlanError> {
        self.db.create_plan(
            edition_id, name, surface_width, surface_height,
            surface_unit, grid_size, created_by,
        ).await
    }

    /// Ajouter un element au plan
    pub async fn add_element(
        &self,
        floor_plan_id: &str,
        element_type: ElementType,
        pos_x_pct: f64,
        pos_y_pct: f64,
    ) -> Result<PlanElement, PlanError> {
        let (default_w, default_h) = element_type.default_size_pct();
        self.db.add_element(
            floor_plan_id, element_type,
            pos_x_pct, pos_y_pct,
            default_w, default_h,
        ).await
    }

    /// Deplacer un element (snap-to-grid optionnel)
    pub async fn move_element(
        &self,
        element_id: &str,
        pos_x_pct: f64,
        pos_y_pct: f64,
        snap_to_grid: bool,
    ) -> Result<PlanElement, PlanError> {
        let (x, y) = if snap_to_grid {
            let plan = self.db.get_plan_for_element(element_id).await?;
            let grid_step_x = (plan.grid_size / plan.surface_width) * 100.0;
            let grid_step_y = (plan.grid_size / plan.surface_height) * 100.0;
            (
                (pos_x_pct / grid_step_x).round() * grid_step_x,
                (pos_y_pct / grid_step_y).round() * grid_step_y,
            )
        } else {
            (pos_x_pct, pos_y_pct)
        };
        self.db.move_element(element_id, x, y).await
    }

    /// Redimensionner un element
    pub async fn resize_element(
        &self,
        element_id: &str,
        width_pct: f64,
        height_pct: f64,
    ) -> Result<PlanElement, PlanError> {
        self.db.resize_element(element_id, width_pct, height_pct).await
    }

    /// Publier/depublier un plan
    pub async fn set_published(
        &self,
        floor_plan_id: &str,
        is_published: bool,
    ) -> Result<FloorPlan, PlanError> {
        self.db.set_published(floor_plan_id, is_published).await
    }

    /// Sauvegarder le plan (incremente la version)
    pub async fn save_plan(
        &self,
        floor_plan_id: &str,
    ) -> Result<FloorPlan, PlanError> {
        self.db.increment_version(floor_plan_id).await
    }

    /// Promouvoir un element stand en stand reservable
    pub async fn create_stand_from_element(
        &self,
        plan_element_id: &str,
        edition_id: &str,
        code: &str,
        stand_type: StandType,
        prix: f64,
    ) -> Result<Stand, PlanError> {
        // Verifier que l'element est bien un type stand
        let element = self.db.get_element(plan_element_id).await?;
        if !element.element_type.is_stand() {
            return Err(PlanError::NotAStandElement {
                element_type: element.element_type,
            });
        }
        self.db.create_stand(
            plan_element_id, edition_id, code, stand_type, prix,
        ).await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PlanError {
    #[error("Plan non trouve : {0}")]
    NotFound(String),

    #[error("Element non trouve : {0}")]
    ElementNotFound(String),

    #[error("L'element de type {element_type:?} n'est pas un stand")]
    NotAStandElement { element_type: ElementType },

    #[error("Nom de plan duplique pour cette edition : {0}")]
    DuplicateName(String),

    #[error("Erreur base de donnees : {0}")]
    DbError(#[from] crate::data::DbError),
}
```

---

## 8. Schemas SQL KindMother

```sql
-- ═══════════════════════════════════════════════════════════════
-- JayFestival — Plan Interactif & Reservation Stands
-- Migration KindMother (libSQL)
-- ═══════════════════════════════════════════════════════════════

-- ─── Plans d'implantation ────────────────────────────────────

CREATE TABLE IF NOT EXISTS floor_plans (
    id              TEXT PRIMARY KEY,
    edition_id      TEXT NOT NULL REFERENCES editions(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    surface_width   REAL NOT NULL,
    surface_height  REAL NOT NULL,
    surface_unit    TEXT NOT NULL DEFAULT 'm'
                    CHECK(surface_unit IN ('m', 'cm', 'ft')),
    grid_size       REAL NOT NULL DEFAULT 1.0,
    grid_visible    INTEGER NOT NULL DEFAULT 1,  -- boolean
    is_published    INTEGER NOT NULL DEFAULT 0,  -- boolean
    background_image BLOB,
    version         INTEGER NOT NULL DEFAULT 1,
    created_by      TEXT NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(edition_id, name)
);

CREATE INDEX idx_floor_plans_edition ON floor_plans(edition_id);

-- ─── Elements du plan ────────────────────────────────────────

CREATE TABLE IF NOT EXISTS plan_elements (
    id              TEXT PRIMARY KEY,
    floor_plan_id   TEXT NOT NULL REFERENCES floor_plans(id) ON DELETE CASCADE,
    element_type    TEXT NOT NULL
                    CHECK(element_type IN (
                        'stand_small', 'stand_medium', 'stand_large',
                        'stand_xl', 'stand_custom',
                        'wall', 'door', 'entrance', 'exit',
                        'bar', 'stage', 'bathroom', 'table', 'chair',
                        'info_point', 'technical_area', 'reserved_area',
                        'foodtruck_zone', 'parking'
                    )),
    label           TEXT,
    pos_x_pct       REAL NOT NULL CHECK(pos_x_pct >= 0.0 AND pos_x_pct <= 100.0),
    pos_y_pct       REAL NOT NULL CHECK(pos_y_pct >= 0.0 AND pos_y_pct <= 100.0),
    width_pct       REAL NOT NULL CHECK(width_pct > 0.0 AND width_pct <= 100.0),
    height_pct      REAL NOT NULL CHECK(height_pct > 0.0 AND height_pct <= 100.0),
    rotation_deg    REAL NOT NULL DEFAULT 0.0
                    CHECK(rotation_deg >= 0.0 AND rotation_deg < 360.0),
    color           TEXT,
    is_locked       INTEGER NOT NULL DEFAULT 0,  -- boolean
    z_index         INTEGER NOT NULL DEFAULT 0,
    metadata        TEXT,  -- JSON
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_plan_elements_plan ON plan_elements(floor_plan_id);
CREATE INDEX idx_plan_elements_type ON plan_elements(element_type);

-- ─── Stands reservables ──────────────────────────────────────

CREATE TABLE IF NOT EXISTS stands (
    id              TEXT PRIMARY KEY,
    plan_element_id TEXT NOT NULL REFERENCES plan_elements(id) ON DELETE CASCADE,
    edition_id      TEXT NOT NULL REFERENCES editions(id) ON DELETE CASCADE,
    code            TEXT NOT NULL,
    stand_type      TEXT NOT NULL
                    CHECK(stand_type IN ('interieur', 'exterieur', 'restauration')),
    prix            REAL NOT NULL DEFAULT 0.0,
    largeur_m       REAL,
    longueur_m      REAL,
    zone            TEXT,
    equipements     TEXT DEFAULT '[]',  -- JSON array
    electricite     INTEGER NOT NULL DEFAULT 0,  -- boolean
    accessibilite   TEXT,
    statut          TEXT NOT NULL DEFAULT 'disponible'
                    CHECK(statut IN ('disponible', 'reserve', 'valide')),
    exposant_id     TEXT REFERENCES exposants(id) ON DELETE SET NULL,
    reserved_by     TEXT,
    reserved_at     TEXT,
    validated_by    TEXT,
    validated_at    TEXT,
    commentaire     TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(edition_id, code)
);

CREATE INDEX idx_stands_edition ON stands(edition_id);
CREATE INDEX idx_stands_exposant ON stands(exposant_id);
CREATE INDEX idx_stands_statut ON stands(statut);

-- ─── Archivage des reservations ──────────────────────────────

CREATE TABLE IF NOT EXISTS stand_reservations_archive (
    id              TEXT PRIMARY KEY,
    source_stand_id TEXT NOT NULL,
    edition_id      TEXT NOT NULL REFERENCES editions(id),
    code            TEXT NOT NULL,
    stand_type      TEXT NOT NULL,
    exposant_id     TEXT,
    statut          TEXT NOT NULL,
    prix            REAL NOT NULL,
    snapshot_data   TEXT NOT NULL,  -- JSON : copie integrale stand + element plan
    archived_at     TEXT NOT NULL DEFAULT (datetime('now')),
    archived_by     TEXT NOT NULL
);

CREATE INDEX idx_archive_edition ON stand_reservations_archive(edition_id);
CREATE INDEX idx_archive_exposant ON stand_reservations_archive(exposant_id);

-- ─── Triggers de mise a jour automatique ─────────────────────

CREATE TRIGGER trg_floor_plans_updated_at
AFTER UPDATE ON floor_plans
FOR EACH ROW
BEGIN
    UPDATE floor_plans SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER trg_plan_elements_updated_at
AFTER UPDATE ON plan_elements
FOR EACH ROW
BEGIN
    UPDATE plan_elements SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER trg_stands_updated_at
AFTER UPDATE ON stands
FOR EACH ROW
BEGIN
    UPDATE stands SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- ─── Vues utilitaires ────────────────────────────────────────

-- Vue : stands avec infos plan_element et exposant
CREATE VIEW IF NOT EXISTS v_stands_complets AS
SELECT
    s.*,
    pe.pos_x_pct,
    pe.pos_y_pct,
    pe.width_pct,
    pe.height_pct,
    pe.rotation_deg,
    pe.is_locked,
    fp.name AS plan_name,
    fp.is_published AS plan_published,
    e.stand_name AS exposant_name
FROM stands s
JOIN plan_elements pe ON s.plan_element_id = pe.id
JOIN floor_plans fp ON pe.floor_plan_id = fp.id
LEFT JOIN exposants e ON s.exposant_id = e.id;

-- Vue : statistiques par edition
CREATE VIEW IF NOT EXISTS v_stand_stats AS
SELECT
    s.edition_id,
    s.zone,
    s.stand_type,
    COUNT(*) AS total,
    SUM(CASE WHEN s.statut = 'disponible' THEN 1 ELSE 0 END) AS disponibles,
    SUM(CASE WHEN s.statut = 'reserve' THEN 1 ELSE 0 END) AS reserves,
    SUM(CASE WHEN s.statut = 'valide' THEN 1 ELSE 0 END) AS valides,
    SUM(s.prix) AS total_prix,
    SUM(CASE WHEN s.statut IN ('reserve', 'valide') THEN s.prix ELSE 0 END) AS prix_engage
FROM stands s
GROUP BY s.edition_id, s.zone, s.stand_type;
```

---

## 9. UI Dioxus — Composants

### 9.1 Canvas du plan (rendu 2D)

```rust
// crates/jayfestival/src/ui/plan/plan_canvas.rs
use dioxus::prelude::*;
use crate::plan::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasTool {
    Select,
    AddStand(ElementType),
    AddElement(ElementType),
    Pan,
}

#[component]
pub fn PlanCanvas(
    floor_plan: FloorPlan,
    elements: Vec<PlanElement>,
    stands: Vec<Stand>,
    selected_id: Signal<Option<String>>,
    active_tool: Signal<CanvasTool>,
    is_constructor: bool,
) -> Element {
    let mut zoom = use_signal(|| 1.0_f64);
    let mut pan_offset = use_signal(|| (0.0_f64, 0.0_f64));

    // Dimensions du canvas en pixels (responsive)
    let canvas_width = 800.0;
    let canvas_height = canvas_width
        * (floor_plan.surface_height / floor_plan.surface_width);

    rsx! {
        div {
            class: "plan-canvas-container",
            style: "position: relative; overflow: hidden; width: 100%; aspect-ratio: {floor_plan.surface_width}/{floor_plan.surface_height};",

            // Fond du plan
            div {
                class: "plan-surface",
                style: "transform: scale({zoom}) translate({pan_offset.read().0}px, {pan_offset.read().1}px); transform-origin: 0 0; width: 100%; height: 100%; position: relative; background: #f8f8f8;",

                // Grille (visible UNIQUEMENT en mode constructeur)
                if is_constructor && floor_plan.grid_visible {
                    PlanGrid {
                        surface_width: floor_plan.surface_width,
                        surface_height: floor_plan.surface_height,
                        grid_size: floor_plan.grid_size,
                    }
                }

                // Elements du plan
                for element in &elements {
                    PlanElementView {
                        key: "{element.id}",
                        element: element.clone(),
                        stand: stands.iter().find(|s| s.plan_element_id == element.id).cloned(),
                        is_selected: *selected_id.read() == Some(element.id.clone()),
                        is_constructor,
                        on_select: move |id| selected_id.set(Some(id)),
                    }
                }
            }

            // Barre de zoom
            div {
                class: "zoom-controls",
                style: "position: absolute; bottom: 8px; right: 8px; display: flex; gap: 4px;",
                button {
                    onclick: move |_| zoom.set((*zoom.read() * 1.2).min(20.0)),
                    "+"
                }
                span { "{(*zoom.read() * 100.0) as u32}%" }
                button {
                    onclick: move |_| zoom.set((*zoom.read() / 1.2).max(0.1)),
                    "-"
                }
                button {
                    onclick: move |_| { zoom.set(1.0); pan_offset.set((0.0, 0.0)); },
                    "Reset"
                }
            }
        }
    }
}
```

### 9.2 Grille du constructeur

```rust
// crates/jayfestival/src/ui/plan/plan_grid.rs
use dioxus::prelude::*;

#[component]
pub fn PlanGrid(
    surface_width: f64,
    surface_height: f64,
    grid_size: f64,
) -> Element {
    let cols = (surface_width / grid_size).ceil() as usize;
    let rows = (surface_height / grid_size).ceil() as usize;
    let cell_w_pct = (grid_size / surface_width) * 100.0;
    let cell_h_pct = (grid_size / surface_height) * 100.0;

    rsx! {
        svg {
            class: "plan-grid",
            style: "position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none;",
            // Lignes verticales
            for i in 0..=cols {
                line {
                    x1: "{i as f64 * cell_w_pct}%",
                    y1: "0%",
                    x2: "{i as f64 * cell_w_pct}%",
                    y2: "100%",
                    stroke: "#d1d5db",
                    stroke_width: "0.5",
                    stroke_dasharray: "4,4",
                }
            }
            // Lignes horizontales
            for j in 0..=rows {
                line {
                    x1: "0%",
                    y1: "{j as f64 * cell_h_pct}%",
                    x2: "100%",
                    y2: "{j as f64 * cell_h_pct}%",
                    stroke: "#d1d5db",
                    stroke_width: "0.5",
                    stroke_dasharray: "4,4",
                }
            }
        }
    }
}
```

### 9.3 Element du plan (stand, mur, porte...)

```rust
// crates/jayfestival/src/ui/plan/plan_element.rs
use dioxus::prelude::*;
use crate::plan::types::*;

#[component]
pub fn PlanElementView(
    element: PlanElement,
    stand: Option<Stand>,
    is_selected: bool,
    is_constructor: bool,
    on_select: EventHandler<String>,
) -> Element {
    let color = element.color.as_deref()
        .unwrap_or(element.element_type.default_color());

    let border_style = if is_selected {
        "border: 2px solid #7C3AED; box-shadow: 0 0 0 2px rgba(124,58,237,0.3);"
    } else if element.element_type.is_stand() {
        if stand.as_ref().map_or(false, |s| s.exposant_id.is_some()) {
            "border: 2px solid currentColor;"
        } else {
            "border: 2px dashed currentColor; opacity: 0.7;"
        }
    } else {
        "border: 1px solid rgba(0,0,0,0.2);"
    };

    // Couleur selon statut du stand (si applicable)
    let bg_color = stand.as_ref().map_or(
        format!("{color}33"),  // 20% opacity
        |s| format!("{}99", s.statut.color()),  // 60% opacity
    );

    rsx! {
        div {
            class: "plan-element",
            style: "position: absolute; left: {element.pos_x_pct}%; top: {element.pos_y_pct}%; width: {element.width_pct}%; height: {element.height_pct}%; transform: rotate({element.rotation_deg}deg); background: {bg_color}; {border_style} border-radius: 4px; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 0.65rem; overflow: hidden; z-index: {element.z_index};",
            onclick: {
                let id = element.id.clone();
                move |_| on_select.call(id.clone())
            },

            // Label de l'element
            if let Some(label) = &element.label {
                span {
                    class: "element-label",
                    style: "color: white; font-weight: bold; text-shadow: 0 1px 2px rgba(0,0,0,0.5);",
                    "{label}"
                }
            }

            // Info stand (code + exposant)
            if let Some(stand) = &stand {
                div {
                    style: "text-align: center;",
                    div {
                        style: "font-weight: bold; color: white;",
                        "{stand.code}"
                    }
                    if let Some(ref _expo_id) = stand.exposant_id {
                        div {
                            style: "font-size: 0.55rem; color: rgba(255,255,255,0.8);",
                            // TODO: resoudre nom exposant
                            "Assigne"
                        }
                    }
                }
            }

            // Indicateur de verrouillage
            if element.is_locked && is_constructor {
                div {
                    style: "position: absolute; top: 2px; right: 2px; font-size: 0.5rem;",
                    "🔒"
                }
            }

            // Poignees de redimensionnement (mode constructeur, element selectionne, non verrouille)
            if is_constructor && is_selected && !element.is_locked {
                // Coin bas-droit
                div {
                    class: "resize-handle",
                    style: "position: absolute; bottom: -3px; right: -3px; width: 6px; height: 6px; background: #7C3AED; cursor: se-resize;",
                }
                // Coin haut-droit
                div {
                    class: "resize-handle",
                    style: "position: absolute; top: -3px; right: -3px; width: 6px; height: 6px; background: #7C3AED; cursor: ne-resize;",
                }
            }
        }
    }
}
```

### 9.4 Toolbar du constructeur

```rust
// crates/jayfestival/src/ui/plan/plan_toolbar.rs
use dioxus::prelude::*;
use crate::plan::types::*;
use super::plan_canvas::CanvasTool;

#[component]
pub fn PlanToolbar(
    active_tool: Signal<CanvasTool>,
    on_save: EventHandler<()>,
    on_publish: EventHandler<bool>,
    is_published: bool,
    version: u32,
) -> Element {
    rsx! {
        div {
            class: "plan-toolbar",
            style: "display: flex; gap: 4px; padding: 8px; border-bottom: 1px solid #e5e7eb; flex-wrap: wrap; align-items: center;",

            // Outils de selection
            ToolButton { tool: CanvasTool::Select, active: active_tool, label: "Selection" }
            ToolButton { tool: CanvasTool::Pan, active: active_tool, label: "Deplacer vue" }

            div { style: "width: 1px; height: 24px; background: #d1d5db; margin: 0 4px;" }

            // Ajout de stands
            ToolButton { tool: CanvasTool::AddStand(ElementType::StandSmall), active: active_tool, label: "Stand S" }
            ToolButton { tool: CanvasTool::AddStand(ElementType::StandMedium), active: active_tool, label: "Stand M" }
            ToolButton { tool: CanvasTool::AddStand(ElementType::StandLarge), active: active_tool, label: "Stand L" }
            ToolButton { tool: CanvasTool::AddStand(ElementType::StandXl), active: active_tool, label: "Stand XL" }

            div { style: "width: 1px; height: 24px; background: #d1d5db; margin: 0 4px;" }

            // Ajout d'elements
            ToolButton { tool: CanvasTool::AddElement(ElementType::Wall), active: active_tool, label: "Mur" }
            ToolButton { tool: CanvasTool::AddElement(ElementType::Door), active: active_tool, label: "Porte" }
            ToolButton { tool: CanvasTool::AddElement(ElementType::Entrance), active: active_tool, label: "Entree" }
            ToolButton { tool: CanvasTool::AddElement(ElementType::Bar), active: active_tool, label: "Bar" }
            ToolButton { tool: CanvasTool::AddElement(ElementType::Stage), active: active_tool, label: "Scene" }

            // Spacer
            div { style: "flex: 1;" }

            // Actions
            span {
                style: "font-size: 0.75rem; color: #6b7280;",
                "v{version}"
            }
            button {
                class: "btn-secondary",
                onclick: move |_| on_save.call(()),
                "Sauvegarder"
            }
            button {
                class: if is_published { "btn-warning" } else { "btn-primary" },
                onclick: move |_| on_publish.call(!is_published),
                if is_published { "Depublier" } else { "Publier" }
            }
        }
    }
}

#[component]
fn ToolButton(
    tool: CanvasTool,
    active: Signal<CanvasTool>,
    label: String,
) -> Element {
    let is_active = *active.read() == tool;
    rsx! {
        button {
            class: if is_active { "tool-btn active" } else { "tool-btn" },
            style: "padding: 4px 8px; border-radius: 4px; font-size: 0.75rem; border: 1px solid {if is_active { \"#7C3AED\" } else { \"#d1d5db\" }}; background: {if is_active { \"#EDE9FE\" } else { \"white\" }};",
            onclick: move |_| active.set(tool),
            "{label}"
        }
    }
}
```

### 9.5 Vue exploitation (lecture seule)

```rust
// crates/jayfestival/src/ui/plan/plan_viewer.rs
use dioxus::prelude::*;
use crate::plan::types::*;

#[component]
pub fn PlanViewer(
    floor_plan: FloorPlan,
    elements: Vec<PlanElement>,
    stands: Vec<Stand>,
    user_role: UserRole,
    user_exposant_id: Option<String>,
) -> Element {
    let mut selected_stand = use_signal(|| Option::<Stand>::None);

    rsx! {
        div {
            class: "plan-viewer",

            // Legende
            div {
                class: "plan-legend",
                style: "display: flex; gap: 16px; padding: 8px; font-size: 0.75rem;",
                LegendItem { color: "#10B981", label: "Disponible" }
                LegendItem { color: "#F59E0B", label: "Reserve" }
                LegendItem { color: "#3B82F6", label: "Valide" }
            }

            // Canvas en lecture seule (grille masquee)
            PlanCanvas {
                floor_plan: floor_plan.clone(),
                elements,
                stands: stands.clone(),
                selected_id: use_signal(|| None),
                active_tool: use_signal(|| super::plan_canvas::CanvasTool::Select),
                is_constructor: false,
            }

            // Statistiques
            div {
                class: "plan-stats",
                style: "display: flex; gap: 12px; padding: 8px; font-size: 0.8rem; border-top: 1px solid #e5e7eb;",
                {
                    let total = stands.len();
                    let dispo = stands.iter().filter(|s| s.statut == StandStatut::Disponible).count();
                    let reserve = stands.iter().filter(|s| s.statut == StandStatut::Reserve).count();
                    let valide = stands.iter().filter(|s| s.statut == StandStatut::Valide).count();
                    rsx! {
                        span { "Total: {total}" }
                        span { style: "color: #10B981;", "Disponibles: {dispo}" }
                        span { style: "color: #F59E0B;", "Reserves: {reserve}" }
                        span { style: "color: #3B82F6;", "Valides: {valide}" }
                    }
                }
            }

            // Detail du stand selectionne
            if let Some(stand) = selected_stand.read().as_ref() {
                StandDetail {
                    stand: stand.clone(),
                    user_role,
                    user_exposant_id: user_exposant_id.clone(),
                }
            }
        }
    }
}

#[component]
fn LegendItem(color: String, label: String) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px;",
            div {
                style: "width: 12px; height: 12px; border-radius: 2px; background: {color};",
            }
            span { "{label}" }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Manager,
    Exposant,
    Visiteur,
    NonConnecte,
}
```

---

## 10. Parcours utilisateur

### 10.1 Organisateur — Construire un plan

```
1. Acceder a Edition > Plan
2. Cliquer "Nouveau plan"
3. Renseigner :
   ─ Nom du plan (ex: "Grande Salle")
   ─ Dimensions surface (ex: 15m x 8m)
   ─ Taille de la grille (ex: 1m)
4. Le constructeur s'ouvre avec la grille visible

5. Placer les elements :
   a. Selectionner un outil (Stand S, Mur, Porte...)
   b. Cliquer sur le plan pour placer
   c. Deplacer par drag & drop (snap-to-grid optionnel)
   d. Redimensionner via les poignees
   e. Configurer dans le panneau proprietes

6. Pour chaque stand place :
   a. Attribuer un code unique (GS_01, GS_02...)
   b. Definir le type (Interieur, Exterieur, Restauration)
   c. Definir le prix
   d. Configurer les equipements (table, chaises, electricite...)
   e. Promouvoir en "stand reservable"

7. Sauvegarder (version incrementee)
8. Optionnel : importer une image de fond (plan architecte)
9. Publier le plan → visible par les exposants et visiteurs
```

### 10.2 Organisateur — Gerer les reservations

```
1. Acceder au plan publie (mode exploitation)
2. Voir les stands colores par statut (vert/orange/bleu)
3. Clic sur un stand → details dans le panneau lateral

4a. Reserver pour un exposant :
    ─ Clic droit > "Reserver pour..."
    ─ Chercher l'exposant (nom, stand_name)
    ─ Confirmer → stand passe en "Reserve" (orange)

4b. Valider une reservation :
    ─ Clic sur stand reserve
    ─ Bouton "Valider"
    ─ → stand passe en "Valide" (bleu)

4c. Liberer un stand :
    ─ Clic sur stand reserve ou valide
    ─ Bouton "Liberer"
    ─ Confirmation requise si statut = "Valide"
    ─ → stand retourne en "Disponible" (vert)

5. Vue tableau complementaire :
   ─ Filtrer par zone, type, statut
   ─ Actions en masse (valider/liberer selection)
   ─ Export CSV
```

### 10.3 Exposant — Reserver un stand

```
1. Acceder au plan de l'edition (via dashboard exposant)
2. Voir les stands disponibles (vert)
3. Cliquer sur un stand disponible
4. Info-bulle : code, type, prix, equipements
5. Bouton "Reserver"
6. Confirmation :
   ─ "Vous allez reserver le stand GS_01 (Interieur, 80€). Confirmer ?"
   ─ Si deja 2 stands : "Vous avez atteint la limite de 2 emplacements."
7. Stand passe en "Reserve" (orange)
8. L'exposant peut annuler sa reservation (avant validation admin)
9. Quand l'admin valide → stand passe en "Valide" (bleu)
   ─ Notification a l'exposant
```

### 10.4 Visiteur — Consulter le plan

```
1. Acceder a la fiche evenement publique
2. Section "Plan" (si publie par l'organisateur)
3. Voir le plan sans grille, avec les zones et stands
4. Stands affiches avec noms des exposants (si valides)
5. Clic sur un stand → info-bulle avec nom exposant, categorie
6. Pas d'actions de reservation
```

---

## 11. Regles metier et contraintes

### 11.1 Invariants

| Regle | Description | Enforcement |
|-------|-------------|-------------|
| **R1** | Un stand a exactement 1 des 3 statuts | `CHECK` SQL |
| **R2** | Max 2 stands par exposant par edition | Verification applicative avant `INSERT`/`UPDATE` |
| **R3** | Un code de stand est unique par edition | `UNIQUE` SQL |
| **R4** | Seuls Admin/Manager peuvent valider/liberer | Verification role applicative |
| **R5** | La grille est invisible en mode exploitation | Logique UI (`is_constructor` flag) |
| **R6** | Les positions sont en pourcentage (0–100) | `CHECK` SQL |
| **R7** | Un element verrouille ne peut pas etre deplace | Verification applicative |
| **R8** | La publication rend le plan visible aux exposants/visiteurs | Flag `is_published` |
| **R9** | Le changement d'emplacement est atomique | Transaction SQL |
| **R10** | L'archivage preserve l'etat complet du stand | Snapshot JSON |

### 11.2 Permissions (Row Level Security equivalent)

```
floor_plans :
  SELECT → Tout utilisateur (si is_published) OU Admin/Manager (toujours)
  INSERT → Admin/Manager
  UPDATE → Admin/Manager
  DELETE → Admin uniquement

plan_elements :
  SELECT → Meme regle que floor_plans parent
  INSERT → Admin/Manager
  UPDATE → Admin/Manager (sauf elements verrouilles sans flag admin)
  DELETE → Admin/Manager

stands :
  SELECT → Tout utilisateur (via plan publie)
  INSERT → Admin/Manager (creation stand reservable)
  UPDATE statut → Admin/Manager (valider/liberer) OU Exposant (reserver/annuler)
  DELETE → Admin uniquement
```

### 11.3 Performance

| Aspect | Strategie |
|--------|-----------|
| Nombre d'elements par plan | Limite suggeree : 500 elements max |
| Taille JSONB snapshot | Compression gzip pour archivage |
| Temps reel | Evenements granulaires (un stand a la fois, pas tout le plan) |
| Rendu canvas | Rendu CSS/SVG (pas WebGL), suffisant pour 500 elements |
| Cache | Plans et elements caches en memoire, invalides sur evenement |

---

## 12. Plan d'implementation

### 12.1 Phase A : Constructeur de plan (prerequis)

| Priorite | Tache | Estimation |
|----------|-------|------------|
| **P0** | Tables SQL `floor_plans` + `plan_elements` | 1j |
| **P0** | Types Rust `FloorPlan`, `PlanElement`, `ElementType` | 0.5j |
| **P0** | `PlanService` : CRUD plan + elements | 1j |
| **P0** | Canvas 2D Dioxus (rendu SVG/CSS) | 2j |
| **P0** | Grille du constructeur (SVG, visible/masquee) | 0.5j |
| **P1** | Drag & drop elements | 1.5j |
| **P1** | Snap-to-grid | 0.5j |
| **P1** | Panneau proprietes | 1j |
| **P1** | Toolbar avec outils | 0.5j |
| **P2** | Redimensionnement (poignees) | 1j |
| **P2** | Rotation | 0.5j |
| **P2** | Import image de fond | 0.5j |
| **P2** | Export PNG/SVG | 1j |

### 12.2 Phase B : Stands reservables

| Priorite | Tache | Estimation |
|----------|-------|------------|
| **P0** | Table SQL `stands` | 0.5j |
| **P0** | Types Rust `Stand`, `StandStatut`, `StandType` | 0.5j |
| **P0** | `StandService` : reserver, valider, liberer | 1j |
| **P0** | Promotion element → stand reservable | 0.5j |
| **P0** | Contrainte max 2 stands par exposant | 0.5j |
| **P1** | Changement d'emplacement (transaction) | 0.5j |
| **P1** | Verification disponibilite batch | 0.5j |
| **P1** | UI info-bulle stand (hover) | 0.5j |
| **P1** | Dialog reservation exposant | 1j |
| **P1** | Actions admin (panneau lateral) | 1j |

### 12.3 Phase C : Temps reel et administration

| Priorite | Tache | Estimation |
|----------|-------|------------|
| **P0** | Evenements `PlanEvent` + broadcast | 1j |
| **P0** | Mise a jour granulaire UI sur evenement | 1j |
| **P0** | Gestion conflits de reservation | 0.5j |
| **P1** | Vue tableau des stands (liste + filtres) | 1j |
| **P1** | Actions en masse (valider/liberer) | 0.5j |
| **P1** | Statistiques en temps reel | 0.5j |
| **P2** | Archivage par edition | 1j |
| **P2** | Export CSV/PDF des reservations | 0.5j |
| **P2** | Historique d'actions par stand | 1j |

### 12.4 Phase D : Publication et acces public

| Priorite | Tache | Estimation |
|----------|-------|------------|
| **P0** | Mode exploitation (lecture seule) | 1j |
| **P0** | Publication/depublication du plan | 0.5j |
| **P1** | Vue visiteur (plan avec noms exposants) | 0.5j |
| **P1** | Vue exposant (reservation + annulation) | 1j |
| **P2** | Legende interactive | 0.5j |
| **P2** | Filtres par zone/type | 0.5j |

---

## Dependances inter-services

| Service / Kit | Role dans le plan interactif |
|---------------|------------------------------|
| **KindMother** | Persistance (libSQL) + broadcast evenements |
| **MiyuAuth** | Verification roles (Admin, Manager, Exposant) |
| **JayFestival (Editions)** | Liaison edition → plans → stands |
| **JayFestival (Exposants)** | Liaison exposant → reservation stand |
| **MiyuBilling** | Generation facture apres validation stand |
| **StrongFather** | Gouvernance : qui peut publier, archiver |

---

**Version** : 1.0
**Date** : 10 fevrier 2026
**Source** : Catakana Orga (Fabric.js, Supabase Realtime)
**Cible** : JayFestival (Rust/Dioxus, KindMother libSQL)
