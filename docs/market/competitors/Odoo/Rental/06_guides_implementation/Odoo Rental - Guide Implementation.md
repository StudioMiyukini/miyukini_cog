# Odoo Rental — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Rental dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates, modules)
- Schémas de données (commandes, lignes, tarification, stock)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des crates proposée

```
crates/
├── miyurental/                        # RentalOrderOperator + cœur métier
│   ├── src/
│   │   ├── lib.rs
│   │   ├── order.rs                    # Modèle RentalOrder
│   │   ├── line.rs                     # Modèle RentalOrderLine
│   │   ├── state.rs                    # États et transitions
│   │   ├── pricing.rs                  # Délégation RentalPricing
│   │   ├── stock.rs                    # Délégation RentalStock
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyurental_pricing/                 # RentalPricingOperator (optionnel : peut être dans miyurental)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── config.rs                   # Configuration produit (grilles)
│   │   ├── compute.rs                  # Calcul prix, pénalités
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyurental_stock/                   # RentalStockOperator (optionnel : peut être dans miyurental)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── locations.rs               # Rental In / Rental Out
│   │   ├── availability.rs            # Security Time, chevauchements
│   │   ├── movements.rs               # Pickup / Return
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyurental_ui/                     # RentalUI (optionnel en phase 1)
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── order_form.rs
    │   │   ├── planning.rs
    │   │   └── product_rental_tab.rs
    │   └── admin_cell.rs
    └── Cargo.toml
```

**Alternative MVP :** Une seule crate `miyurental` contenant order, line, pricing, stock (sans découpage Opérateurs séparés), puis découpage en crates distinctes en phase 2.

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyucontacts` : Client (partenaire)
- `miyustore` ou équivalent : Produits, stock de base
- `miyuinvoice` : Facturation (lignes location + pénalités)
- `miyuclock` : Dates et calendrier
- `miyunotify` : Notifications
- `miyusign` ou équivalent : Signature (optionnel)

---

## 2. Schémas de Données

### 2.1 Modèle RentalOrder

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalOrder {
    pub id: RentalOrderId,
    pub partner_id: PartnerId,
    pub company_id: CompanyId,
    pub currency_id: CurrencyId,
    pub state: RentalOrderState,
    pub lines: Vec<RentalOrderLine>,
    pub pickup_scheduled: bool,
    pub return_scheduled: bool,
    pub security_level: SecurityLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RentalOrderState {
    Draft,
    Confirmed,
    Pickup,
    Return,
    Invoiced,
}
```

### 2.2 Modèle RentalOrderLine

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalOrderLine {
    pub id: RentalOrderLineId,
    pub order_id: RentalOrderId,
    pub product_id: ProductId,
    pub quantity: Decimal,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub duration_unit: RentalDurationUnit, // Hour, Day, Week, Month
    pub duration_value: u32,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    pub delay_costs: Option<Decimal>,
    pub state: RentalLineState,
    pub price_rule_explanation: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RentalLineState {
    Planned,
    PickedUp,
    Returned,
    Invoiced,
}
```

### 2.3 Modèle RentalProductConfig (tarification)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalProductConfig {
    pub product_id: ProductId,
    pub can_be_rented: bool,
    pub price_lines: Vec<RentalPriceLine>,
    pub extra_hour: Option<Decimal>,
    pub extra_day: Option<Decimal>,
    pub security_time_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalPriceLine {
    pub unit: RentalDurationUnit,
    pub duration: u32,        // ex. 3 pour "3 days"
    pub price: Decimal,
}
```

### 2.4 Emplacements et mouvements

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalLocation {
    pub warehouse_id: WarehouseId,
    pub rental_in_location_id: LocationId,
    pub rental_out_location_id: LocationId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalMovement {
    pub id: RentalMovementId,
    pub order_id: RentalOrderId,
    pub line_id: RentalOrderLineId,
    pub product_id: ProductId,
    pub quantity: Decimal,
    pub movement_type: RentalMovementType, // Pickup | Return
    pub scheduled_date: DateTime<Utc>,
    pub actual_date: Option<DateTime<Utc>>,
    pub state: MovementState, // Draft, Done
}
```

---

## 3. API et Contrats

### 3.1 RentalOrderOperator (API publique conceptuelle)

```rust
// Création
pub async fn create_rental_order(intent: CreateRentalOrderIntent, mandate: Mandate) -> Result<RentalOrder, RentalError>;

// Mise à jour (devis)
pub async fn update_rental_order(order_id: RentalOrderId, intent: UpdateRentalOrderIntent, mandate: Mandate) -> Result<RentalOrder, RentalError>;

// Confirmation
pub async fn confirm_rental_order(order_id: RentalOrderId, mandate: Mandate) -> Result<RentalOrder, RentalError>;

// Enlèvement
pub async fn register_pickup(order_id: RentalOrderId, line_ids: Vec<RentalOrderLineId>, actual_date: DateTime<Utc>, mandate: Mandate) -> Result<RentalOrder, RentalError>;

// Retour
pub async fn register_return(order_id: RentalOrderId, line_ids: Vec<RentalOrderLineId>, actual_return_date: DateTime<Utc>, mandate: Mandate) -> Result<RentalOrder, RentalError>;

// Facturation (délégation MiyuInvoice)
pub async fn invoice_rental_order(order_id: RentalOrderId, mandate: Mandate) -> Result<InvoiceId, RentalError>;
```

### 3.2 RentalPricingOperator

```rust
pub async fn compute_price(product_id: ProductId, start: DateTime<Utc>, end: DateTime<Utc>, quantity: Decimal) -> Result<PricedRentalLine, RentalError>;
pub async fn compute_delay_costs(lines: &[RentalOrderLine], actual_return_date: DateTime<Utc>) -> Result<Decimal, RentalError>;
pub async fn get_config(product_id: ProductId) -> Result<RentalProductConfig, RentalError>;
```

### 3.3 RentalStockOperator

```rust
pub async fn check_availability(product_id: ProductId, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<AvailabilityResult, RentalError>;
pub async fn create_pickup_movement(product_id: ProductId, quantity: Decimal, scheduled_date: DateTime<Utc>, order_id: RentalOrderId, mandate: Mandate) -> Result<(), RentalError>;
pub async fn create_return_movement(product_id: ProductId, quantity: Decimal, actual_date: DateTime<Utc>, order_id: RentalOrderId, mandate: Mandate) -> Result<(), RentalError>;
```

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (2–3 sprints)

**Objectif :** Devis location, confirmation, enlèvement/retour, calcul prix et pénalités, pas de facturation ni Sign.

**Livrables :**
- Crate `miyurental` (ou miyurental + miyurental_pricing dans une crate)
- Modèles : RentalOrder, RentalOrderLine, RentalProductConfig
- Calcul prix (règle option la moins chère), pénalités Extra Hour / Extra Day
- CRUD commande (draft), confirmation (création mouvements conceptuels ou réels selon stock existant)
- Enregistrement enlèvement et retour (mise à jour statuts + calcul pénalités)
- Pas d’UI dédiée : utilisation API / tests uniquement, ou formulaire minimal

**Bornage :**
- Pas de facturation automatique (hors scope MVP)
- Pas d’intégration Sign (hors scope MVP)
- Stock : soit emplacements Rental In/Out réels, soit simulation (réservation logique)

### Phase 2 — Stock et facturation (1–2 sprints)

**Objectif :** Emplacements Rental In/Out réels, mouvements de stock, facturation des lignes et pénalités.

**Livrables :**
- Crate `miyurental_stock` (ou module dans miyurental) : emplacements, disponibilité, Security Time, mouvements pickup/return
- Intégration MiyuInvoice : facturation commande location (lignes + lignes pénalités)
- Contrôle disponibilité avant confirmation (chevauchements, Security Time)
- Tests de charge sur disponibilité et mouvements

**Bornage :**
- Sign toujours hors scope
- UI : formulaire commande location minimal (liste + formulaire)

### Phase 3 — UI et planning (1–2 sprints)

**Objectif :** Interface complète : commandes, planning, reçu PDF, configuration produits.

**Livrables :**
- Crate `miyurental_ui` (ou intégration dans miyukini-central / app existante)
- Formulaire commande location (lignes, dates, prix, statuts)
- Vue planning (calendrier ou timeline par produit)
- Impression reçu enlèvement/retour (PDF)
- Configuration produits : onglet Location (grilles, Extra Hour/Day, Security Time)
- Paramètres Rental (optionnel : Digital Documents pour phase 4)

**Bornage :**
- Signature électronique (Sign) en phase 4

### Phase 4 — Signature et finition (1 sprint)

**Objectif :** Intégration signature (contrat location), rapports, optimisations.

**Livrables :**
- Intégration MiyuSign (ou équivalent) : demande signature sur commande, modèle « Rental Agreement »
- Paramètres : activer Digital Documents, choix du modèle
- Rapports optionnels : revenus location, taux d’occupation, retards
- Documentation utilisateur et technique
- Revue sécurité et performances

---

## 5. Bornage Fonctionnel (MVP → Complet)

| Fonctionnalité | MVP | Phase 2 | Phase 3 | Phase 4 |
|----------------|-----|--------|--------|--------|
| Devis location (lignes, dates) | ✅ | ✅ | ✅ | ✅ |
| Calcul prix (option la moins chère) | ✅ | ✅ | ✅ | ✅ |
| Pénalités Extra Hour/Day | ✅ | ✅ | ✅ | ✅ |
| Security Time (config + vérification) | ✅ | ✅ | ✅ | ✅ |
| Confirmation commande | ✅ | ✅ | ✅ | ✅ |
| Mouvements stock Rental In/Out | Simulé ou basique | ✅ Réel | ✅ | ✅ |
| Disponibilité (chevauchements) | Basique | ✅ | ✅ | ✅ |
| Enlèvement / retour (statuts) | ✅ | ✅ | ✅ | ✅ |
| Facturation lignes + pénalités | ❌ | ✅ | ✅ | ✅ |
| UI formulaire commande | Minimal | Minimal | ✅ Complète | ✅ |
| Planning / calendrier | ❌ | ❌ | ✅ | ✅ |
| Reçu PDF enlèvement/retour | ❌ | ❌ | ✅ | ✅ |
| Configuration produits (grilles) | ✅ | ✅ | ✅ (UI) | ✅ |
| Signature contrat (Sign) | ❌ | ❌ | ❌ | ✅ |
| Rapports location | ❌ | ❌ | ❌ | Optionnel |

---

## 6. Risques et Points d'Attention

### 6.1 Technique

- **Stock** : S’aligner sur le modèle existant MiyuStore/Inventory (emplacements, mouvements) pour éviter doublons et incohérences.
- **Prix** : Règle « option la moins chère » à documenter et tester (cas limites : durée 0, plusieurs grilles, unités différentes).
- **Dates** : Fuseaux et calendrier (heures ouvrables vs 24/24) à clarifier pour Security Time et pénalités.

### 6.2 Métier

- **Prolongations** : Gérer extension de location (nouvelle ligne ou modification date fin) et impact disponibilité.
- **Vente du loué** : Hors scope initial ; prévoir extension possible (conversion location → vente).
- **Multi-entrepôts** : Chaque entrepôt doit avoir ses Rental In/Out ; bien documenter la configuration.

### 6.3 Sécurité

- Niveau sécurité 2 (Sensitive) pour commandes et stock ; audit des actions (création, confirmation, enlèvement, retour, facturation).
- Mandats avec révocation automatique (fin de session, alerte WorrySentinel).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
