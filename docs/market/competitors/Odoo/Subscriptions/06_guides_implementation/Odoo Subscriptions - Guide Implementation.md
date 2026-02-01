# Odoo Subscriptions — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Subscriptions (Abonnements) dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates)
- Spécifications des crates Rust
- Schémas de données (RecurringPlan, SubscriptionOrder, lignes, facturation, exception)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyupm/                              # SubscriptionOperator + RecurringPlanOperator (optionnel séparé)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── subscription.rs               # Modèle SubscriptionOrder, états, liens
│   │   ├── renew.rs                      # Renouvellement
│   │   ├── upsell.rs                     # Upsell
│   │   ├── close.rs                     # Clôture, raisons
│   │   ├── exception.rs                 # Contract in exception, résolution
│   │   ├── recurring_plan.rs            # Plan récurrent, pricing, self-service
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupm_billing/                       # SubscriptionBillingOperator (ou intégré miyuinvoice)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── schedule.rs                   # Planification génération factures
│   │   ├── prorata.rs                    # Prorata services
│   │   ├── invoice_generate.rs           # Génération facture depuis abonnement
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupm_payment/                      # SubscriptionPaymentOperator (ou intégré miyubilling)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── tokenize.rs                   # Enregistrement moyen de paiement
│   │   ├── charge.rs                     # Prélèvement récurrent
│   │   ├── failure.rs                    # Gestion échec + exception
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyupm_ui/                           # SubscriptionUI (frontend selon stack)
    ├── src/
    │   ├── lib.rs
    │   ├── plans.rs                      # Vues plans récurrents
    │   ├── subscriptions.rs             # Liste / formulaire abonnements
    │   ├── portal.rs                     # Portail client (Renew, Upsell, Close)
    │   └── admin_cell.rs
    └── Cargo.toml
```

**Note :** Le crate `miyupm` (MiyuPM) correspond au nommage Miyu* pour les modules ; équivalent possible **miyukini-subscriptions** si le service est exposé au niveau COG sous ce nom. Ajuster selon la nomenclature retenue (MiyuPM vs Miyukini Subscriptions).

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, TAMR)

**Kits existants :**
- `miyustore` / équivalent Sales : Devis, commandes (sale.order)
- `miyuinvoice` : Factures, écritures, paiements
- `miyubilling` : Prestataires de paiement, tokenisation (si existant)
- `miyunotify` : Notifications, templates email
- `miyucontacts` : Partenaires, multi-société
- `miyuvalidate` : Validation montants, devises, dates

---

## 2. Schémas de Données

### 2.1 RecurringPlan

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPlan {
    pub id: RecurringPlanId,
    pub name: String,
    pub billing_period_unit: BillingPeriodUnit, // Weeks, Months, Years (pas Days)
    pub billing_period_value: u32,
    pub automatic_closing_days: Option<u32>,
    pub align_to_period_start: bool,
    pub company_id: Option<CompanyId>,
    pub invoice_email_template_id: Option<TemplateId>,
    pub closable: bool,
    pub add_products: bool,
    pub renew: bool,
    pub optional_plan_ids: Vec<RecurringPlanId>,
    pub pricing_line_ids: Vec<RecurringPlanPricingLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPlanPricingLine {
    pub product_id: Option<ProductId>,
    pub product_variant_id: Option<ProductVariantId>,
    pub pricelist_id: Option<PricelistId>,
    pub recurring_price: Decimal,
}
```

### 2.2 SubscriptionOrder (extension sale.order)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionOrder {
    pub id: OrderId,
    pub name: String,
    pub partner_id: PartnerId,
    pub recurring_plan_id: RecurringPlanId,
    pub state: OrderState,           // draft, sent, sale, cancel (Sales)
    pub subscription_status: SubscriptionStatus, // Quotation, InProgress, RenewalQuotation, Churned, PaymentFailure, Closed
    pub next_invoice_date: Option<Date>,
    pub close_reason_id: Option<CloseReasonId>,
    pub close_reason_text: Option<String>,
    pub contract_in_exception: bool,
    pub payment_failure_tag: bool,
    pub order_line_ids: Vec<SubscriptionOrderLine>,
    pub renewal_order_ids: Vec<OrderId>,
    pub upsell_order_ids: Vec<OrderId>,
    pub parent_subscription_id: Option<OrderId>, // si cet ordre est un renouvellement ou upsell
    pub create_date: DateTime,
    pub write_date: DateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Quotation,
    InProgress,
    RenewalQuotation,
    Churned,
    PaymentFailure,
    Closed,
}
```

### 2.3 SubscriptionOrderLine

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionOrderLine {
    pub id: OrderLineId,
    pub order_id: OrderId,
    pub product_id: ProductId,
    pub product_uom_id: UomId,
    pub quantity: Decimal,
    pub price_unit: Decimal,
    pub price_subtotal: Decimal,
    pub is_recurring: bool,
    pub prorata_applied: bool, // pour affichage (services uniquement)
}
```

### 2.4 CloseReason

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseReason {
    pub id: CloseReasonId,
    pub name: String,
    pub available_for_portal: bool, // client peut choisir cette raison
}
```

---

## 3. API et Contrats (résumé)

**SubscriptionOperator :**
- `confirm_subscription(intent, mandate) -> Result<SubscriptionOrder>`
- `renew(intent, mandate) -> Result<RenewalQuotation>`
- `create_upsell_quotation(intent, mandate) -> Result<UpsellQuotation>`
- `confirm_upsell(order_id, mandate) -> Result<()>`
- `close(intent, mandate) -> Result<()>`
- `resolve_exception(intent, mandate) -> Result<()>`

**RecurringPlanOperator :**
- `create_plan(data, mandate) -> Result<RecurringPlan>`
- `update_plan(id, data, mandate) -> Result<RecurringPlan>`
- `list_close_reasons() -> Result<Vec<CloseReason>>`

**SubscriptionBillingOperator :**
- `generate_scheduled_invoice(subscription_id, mandate) -> Result<InvoiceId>`
- `prorata_compute(order_line_ids, period_start, period_end) -> Result<Decimal>`

**SubscriptionPaymentOperator :**
- `tokenize(subscription_id, payment_method_id, mandate) -> Result<TokenId>`
- `charge_recurring(subscription_id, invoice_id, mandate) -> Result<PaymentId>`
- `resolve_failure(intent, mandate) -> Result<()>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (3–4 semaines)

- **RecurringPlan** : Modèle + CRUD (sans optional_plan_ids ni pricing avancé) ; champs DETAILS et SELF-SERVICE de base.
- **SubscriptionOrder** : Extension commande (recurring_plan_id, subscription_status, next_invoice_date) ; création abonnement par confirmation de devis.
- **Renew** : Création manuelle du devis de renouvellement ; lien parent_subscription_id ; pas encore de planification auto.
- **Close** : Clôture admin avec CloseReason (liste prédéfinie + texte libre) ; pas encore de portail client.
- **Facturation** : Génération manuelle de la facture depuis l’abonnement ; pas encore de prorata ni de planification.
- **UI** : Liste abonnements, formulaire abonnement, formulaire plan, boutons Renew et Close, wizard Close Reason.

**Livrable Phase 1 :** Abonnements créés manuellement, renouvellement manuel, clôture admin, facturation manuelle.

### Phase 2 — Facturation et prorata (2–3 semaines)

- **Planification** : Ever Buddy (ou cron) pour next_invoice_date ; génération automatique des factures à échéance.
- **Prorata** : Calcul prorata pour les produits de type Service (upsell / renouvellement en cours de période).
- **Upsell** : Création devis upsell, confirmation → fusion des lignes ; prorata appliqué aux services.
- **Close Reasons** : Configuration Close Reasons (admin + portail) ; portail client Close Subscription (liste fermée).
- **Contract in exception** : Marquage Payment Failure + contract_in_exception ; blocage des actions planifiées ; résolution manuelle (flux métier sans mode développeur).

**Livrable Phase 2 :** Facturation planifiée, upsell avec prorata, clôture client self-service, gestion exception.

### Phase 3 — Paiements récurrents et eCommerce (2–3 semaines)

- **Tokenisation** : Intégration MiyuBilling (ou prestataire) pour enregistrement du moyen de paiement (checkout + portail).
- **Prélèvement** : À l’échéance, tentative de prélèvement ; en échec → Payment Failure + Contract in exception.
- **eCommerce** : Produits abonnement publiés ; création et confirmation automatiques des devis abonnement après achat en ligne.
- **Portail** : Renew et Add Products en self-service (selon plan) ; affichage Sales History.

**Livrable Phase 3 :** Paiements récurrents (tokenisation + prélèvement), eCommerce abonnements, portail self-service complet.

### Phase 4 — Consolidation (1–2 semaines)

- **Plans** : Optional plans, pricing avancé (lignes par plan), template email facture.
- **Rapports** : MRR, abonnements actifs, résiliations, Payment Failure (équivalents Odoo Subscriptions Reports).
- **Multi-société** : Company sur le plan ; filtres et droits par société.
- **Tests** : Couverture unitaires et d’intégration (création, renew, upsell, close, facturation, exception, prélèvement).

**Livrable Phase 4 :** Fonctionnalités avancées plans, rapports, multi-société, tests.

---

## 5. Bornage Fonctionnel (MVP → Complet)

| Fonctionnalité | MVP | Complet |
|----------------|-----|---------|
| Plans récurrents (DETAILS, SELF-SERVICE) | Oui (base) | Oui (pricing, optional plans, template email) |
| Création abonnement (devis + confirm) | Oui | Oui |
| Renouvellement manuel | Oui | Oui + self-service portail |
| Renouvellement automatique (planifié) | Non | Oui |
| Upsell | Non | Oui (avec prorata services) |
| Clôture admin | Oui | Oui |
| Clôture client (self-service) | Non | Oui |
| Close Reasons (liste + portail) | Liste admin | Liste + portail (liste fermée) |
| Facturation manuelle | Oui | Oui |
| Facturation planifiée | Non | Oui |
| Prorata (services) | Non | Oui |
| Contract in exception | Non | Oui + résolution métier |
| Tokenisation / prélèvement récurrent | Non | Oui |
| eCommerce abonnements | Non | Oui |
| Sales History | Oui (liens) | Oui |
| Rapports (MRR, actifs, résiliations) | Non | Oui |
| Multi-société | Non | Oui |

---

## 6. Critères d'Acceptation (MVP)

- Un plan récurrent peut être créé avec Billing Period (Weeks/Months/Years), Automatic Closing, Align to Period Start, options self-service (Closable, Add Products, Renew).
- Une commande avec plan récurrent et lignes produits abonnement devient un abonnement à la confirmation (statut In Progress après facturation/paiement).
- Le bouton Renew crée un devis de renouvellement lié ; confirmation et facturation suivent le flux standard.
- Le bouton Close ouvre un wizard Close Reason ; après validation, l’abonnement passe en Churned avec motif enregistré.
- Au moins une raison de clôture est configurable ; utilisée dans le wizard Close.
- La facture peut être générée manuellement depuis l’abonnement (sans prorata en MVP).
- La liste et le formulaire abonnement affichent le statut d’abonnement, le plan, la date de prochaine facture (si calculée), et les boutons Renew et Close selon les prérequis.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
