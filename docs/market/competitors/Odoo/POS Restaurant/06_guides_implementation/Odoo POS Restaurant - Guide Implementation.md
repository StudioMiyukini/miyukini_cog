# Odoo POS Restaurant — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique** pour développer l'équivalent POS Restaurant dans Miyukini : architecture crates Rust, schémas de données, API, plan de développement par phases et bornage fonctionnel (MVP → Complet).

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique (crates Rust)
- Schémas de données (Floor, Table, Binding, Course, SubOrder, Booking)
- API et contrats
- Plan de développement en phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates (proposition)

```
crates/
├── miyupos/                      # POS de base (existant ou à créer)
│   └── ...
├── miyupos-restaurant-floor/      # FloorManager
│   ├── src/
│   │   ├── lib.rs
│   │   ├── floor.rs
│   │   ├── table.rs
│   │   ├── plan_state.rs         # État temps réel (disponible, occupée, réservée)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupos-restaurant-binding/   # TableOrderBinding
│   ├── src/
│   │   ├── lib.rs
│   │   ├── binding.rs
│   │   ├── set_table.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupos-restaurant-transfer/  # OrderTransfer
│   ├── src/
│   │   ├── lib.rs
│   │   ├── transfer.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupos-restaurant-course/    # CourseManager
│   ├── src/
│   │   ├── lib.rs
│   │   ├── course.rs
│   │   ├── fire.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupos-restaurant-prepare/   # PreparationPrint
│   ├── src/
│   │   ├── lib.rs
│   │   ├── channel.rs            # Canal (imprimante, écran, webhook)
│   │   ├── routing.rs            # Catégorie → canal
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupos-restaurant-split/     # BillSplit
│   ├── src/
│   │   ├── lib.rs
│   │   ├── suborder.rs
│   │   ├── split.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupos-restaurant-presets/   # RestaurantPresets
│   ├── src/
│   │   ├── lib.rs
│   │   ├── preset.rs             # DineIn, Takeout, Delivery
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupos-restaurant-booking/   # RestaurantBooking
│   ├── src/
│   │   ├── lib.rs
│   │   ├── booking.rs
│   │   ├── agenda_adapter.rs      # Optionnel : Miyukini Agenda
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-central/             # Intégration RestaurantUI (écrans POS)
    └── ... (utilisation des opérateurs ci-dessus)
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, Caring Nanny)

**Kits existants :**
- `miyupos` (ou équivalent) : Ordres POS, paiements, produits
- `miyukini-agenda` ou `miyubooking` (optionnel) : Ressources et créneaux pour réservations
- `miyustore` : Produits et catégories (routage préparation)
- `miyuclock` : Dates et créneaux

---

## 2. Schémas de Données

### 2.1 Floor et Table

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floor {
    pub id: FloorId,
    pub name: String,
    pub pos_config_id: PosConfigId,
    pub background_image_url: Option<String>,
    pub sequence: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: TableId,
    pub floor_id: FloorId,
    pub name: String,
    pub seats: u32,
    pub shape: TableShape,  // Square | Round
    pub position_h: f64,
    pub position_v: f64,
    pub width: f64,
    pub height: f64,
    pub color: String,
    pub active: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum TableState {
    Available,
    Occupied,
    Reserved { until: DateTime<Utc> },
    Late,  // Réservation en retard
}
```

### 2.2 Binding (ordre ↔ table / tab)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTableBinding {
    pub order_id: OrderId,
    pub table_id: Option<TableId>,
    pub tab_name: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

### 2.3 Course

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub order_id: OrderId,
    pub course_number: u32,
    pub line_ids: Vec<LineId>,
    pub sent_at: Option<DateTime<Utc>>,
}
```

### 2.4 SubOrder (split)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubOrder {
    pub id: SubOrderId,
    pub parent_order_id: OrderId,
    pub line_ids: Vec<LineId>,
    pub state: SubOrderState,  // Pending | Paid
    pub paid_at: Option<DateTime<Utc>>,
}
```

### 2.5 Booking

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: BookingId,
    pub table_ids: Vec<TableId>,
    pub guest_name: String,
    pub start: DateTime<Utc>,
    pub duration_minutes: u32,
    pub guests_count: u32,
    pub phone: Option<String>,
    pub stage: BookingStage,  // Booked | CheckedIn | NoShow
}
```

---

## 3. API et Contrats (résumé)

- **FloorManager** : `list_floors`, `create_floor`, `update_table`, `get_plan_state`
- **TableOrderBinding** : `set_table`, `set_tab`, `release`, `get_binding`
- **OrderTransfer** : `transfer_to_table`, `merge_into_order`
- **CourseManager** : `add_course`, `fire_course`, `transfer_course`
- **PreparationPrint** : `send_order`, `send_course`, `cancel_order`
- **BillSplit** : `split_suborder`, `pay_suborder`, `transfer_lines`
- **RestaurantPresets** : `list_presets`, `validate_order_for_preset`
- **RestaurantBooking** : `create_booking`, `update_stage`, `list_by_floor`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (Plan + liaison + paiement)

- **Objectif** : Plan des tables, liaison ordre–table/tab, libération, paiement standard (sans split).
- **Crates** : miyupos-restaurant-floor, miyupos-restaurant-binding.
- **Livrables** : CRUD sols/tables, affichage plan avec état (disponible/occupée), Set Table, Set Tab, Release table, intégration au registre POS.
- **Durée estimée** : 2–3 semaines.

### Phase 2 — Transfert et cours

- **Objectif** : Transfert et fusion d’ordres ; découpage en cours et envoi préparation.
- **Crates** : miyupos-restaurant-transfer, miyupos-restaurant-course, miyupos-restaurant-prepare.
- **Livrables** : Transfer/Merge, Course, Fire Course, routage préparation par catégorie (au moins un canal : écran ou webhook).
- **Durée estimée** : 2–3 semaines.

### Phase 3 — Split et presets

- **Objectif** : Split d’addition (sous-commandes, paiement par part) ; presets Dine In / Takeout / Delivery.
- **Crates** : miyupos-restaurant-split, miyupos-restaurant-presets.
- **Livrables** : Split, sous-commande, paiement partiel ; validation des champs selon preset.
- **Durée estimée** : 2 semaines.

### Phase 4 — Réservations et polish

- **Objectif** : Réservations (tables comme ressources), intégration Agenda si existant ; pourboires, early receipt, tests et documentation.
- **Crates** : miyupos-restaurant-booking (+ adaptateur Agenda).
- **Livrables** : Booking create/update/stages, affichage réservations sur le plan ; options tips/receipt ; tests E2E, doc opérationnelle.
- **Durée estimée** : 2–3 semaines.

---

## 5. Bornage Fonctionnel

### MVP (Phase 1)

- Un point de vente peut avoir plusieurs sols et tables.
- Un ordre peut être lié à une table ou à un tab (nom).
- Le plan affiche les tables et leur état (disponible / occupée).
- Libération de table lorsque l’ordre est réglé et panier vide.
- Pas de transfert, pas de cours, pas de split, pas de réservation.

### Complet (fin Phase 4)

- Tout le périmètre Odoo POS Restaurant couvert : plan, liaison, transfert, fusion, cours, préparation, split, presets, réservations.
- Optionnel : impression physique (driver imprimante ou IoT), pourboires après paiement (terminaux), early receipt.
- Interopération avec Miyukini Agenda pour les ressources (tables) et créneaux.

### Critères d'acceptation (exemples)

- AC1 : Création d’un sol et de 5 tables depuis l’interface ; affichage sur le plan avec états corrects après prise de commande et libération.
- AC2 : Transfert d’un ordre d’une table A vers une table B libre ; occupation mise à jour.
- AC3 : Commande avec 2 cours ; Order envoie le cours 1 ; Fire Course 2 envoie le cours 2 au canal préparation configuré.
- AC4 : Split d’une commande en 2 sous-commandes ; paiement de chaque sous-commande ; retour à l’ordre principal possible uniquement lorsque les deux sont réglées.
- AC5 : Création d’une réservation pour une table à une heure donnée ; affichage « Réservé » sur le plan ; passage en Checked-In depuis le POS.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
