# Odoo Rental — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Rental dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Rental
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, Master Butler, WorrySentinel)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
RentalUI → BondingBrother → RentalOrderOperator → StrongFather (décision)
                                              → KindMother (WriteIntent)
                                              → Master Butler (permissions)
                                              → WorrySentinel (sécurité)
                            → RentalPricingOperator (calcul prix)
                            → RentalStockOperator (disponibilité, mouvements)
```

### 1.2 Flux typique : création commande location

1. **Intention utilisateur** → RentalUI (saisie lignes, dates)
2. **Traduction intention** → BondingBrother
3. **Calcul prix** → RentalPricingOperator (option la moins chère)
4. **Vérification disponibilité** → RentalStockOperator (Security Time, chevauchements)
5. **Demande décision** → StrongFather (création commande)
6. **Vérification permissions** → Master Butler
7. **Vérification sécurité** → WorrySentinel
8. **Persistance** → KindMother (WriteIntent commande + lignes)
9. **Notification** → MiyuNotify (optionnel)

### 1.3 Flux typique : confirmation et enlèvement/retour

1. **Confirmation** : StrongFather (décision) → RentalStockOperator (mouvements) → KindMother (WriteIntent mouvements + mise à jour commande)
2. **Enlèvement** : RentalOrderOperator enregistre enlèvement → RentalStockOperator valide livraison → KindMother (WriteIntent)
3. **Retour** : RentalOrderOperator enregistre retour → RentalPricingOperator (pénalités) → RentalStockOperator valide réception → KindMother (WriteIntent)

---

## 2. Patterns d'Intégration

### 2.1 Création commande location (devis)

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreateRentalOrderIntent {
    pub partner_id: Uuid,
    pub lines: Vec<RentalLineIntent>,
}

pub struct RentalLineIntent {
    pub product_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub quantity: Decimal,
}

impl RentalOrderOperator {
    pub async fn create_rental_order(
        &self,
        intent: CreateRentalOrderIntent,
        mandate: Mandate,
    ) -> Result<RentalOrder, RentalError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["rental_order.create"])?;

        // 1. Calcul prix (RentalPricingOperator)
        let priced_lines: Vec<PricedRentalLine> = self.rental_pricing
            .compute_lines(&intent.lines)
            .await?;

        // 2. Vérification disponibilité (RentalStockOperator)
        for line in &priced_lines {
            self.rental_stock
                .check_availability(line.product_id, line.start_date, line.end_date)
                .await?
                .ensure_available()?;
        }

        // 3. Décision StrongFather
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_rental_order",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(RentalError::DecisionDenied);
        }

        // 4. Permissions Master Butler
        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "rental_order.create",
                resource: None,
            })
            .await?;
        if !permission.granted {
            return Err(RentalError::PermissionDenied);
        }

        // 5. Sécurité WorrySentinel
        let security_level = self.worry_sentinel.get_security_level(&intent).await?;
        if security_level > mandate.max_security_level {
            return Err(RentalError::SecurityLevelExceeded);
        }

        // 6. WriteIntent KindMother
        let write_intent = WriteIntent {
            entity_type: "rental.order",
            operation: WriteOperation::Create,
            data: RentalOrderData {
                partner_id: intent.partner_id,
                lines: priced_lines,
                state: RentalOrderState::Draft,
                company_id: self.get_company_id().await?,
            },
            security_level,
        };

        let order = self.kind_mother.execute(write_intent).await?;
        Ok(order)
    }
}
```

### 2.2 Confirmation commande (création mouvements stock)

**Pattern :** WriteIntent en chaîne (commande + mouvements)

```rust
// Pseudo-code Rust
impl RentalOrderOperator {
    pub async fn confirm_rental_order(
        &self,
        order_id: Uuid,
        mandate: Mandate,
    ) -> Result<RentalOrder, RentalError> {
        mandate.validate_flows(&["rental_order.confirm"])?;

        let order = self.get_order(order_id).await?;
        if order.state != RentalOrderState::Draft {
            return Err(RentalError::InvalidState);
        }

        // 1. Décision StrongFather
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "confirm_rental_order",
                context: &order,
            })
            .await?;
        if !decision.allowed {
            return Err(RentalError::DecisionDenied);
        }

        // 2. Création mouvements via RentalStockOperator (WriteIntent internes)
        for line in &order.lines {
            self.rental_stock
                .create_pickup_movement(
                    line.product_id,
                    line.quantity,
                    line.start_date,
                    order_id,
                    mandate.clone(),
                )
                .await?;
        }

        // 3. Mise à jour commande : state = Confirmed
        let write_intent = WriteIntent {
            entity_type: "rental.order",
            operation: WriteOperation::Update,
            data: RentalOrderUpdate {
                id: order_id,
                state: RentalOrderState::Confirmed,
                pickup_scheduled: true,
            },
            security_level: order.security_level,
        };
        self.kind_mother.execute(write_intent).await?;

        Ok(self.get_order(order_id).await?)
    }
}
```

### 2.3 Enregistrement retour et pénalités

**Pattern :** Calcul pénalités (RentalPricingOperator) + WriteIntent (mouvement + commande + lignes facture)

```rust
// Pseudo-code Rust
impl RentalOrderOperator {
    pub async fn register_return(
        &self,
        order_id: Uuid,
        line_ids: Vec<Uuid>,
        actual_return_date: DateTime<Utc>,
        mandate: Mandate,
    ) -> Result<RentalOrder, RentalError> {
        mandate.validate_flows(&["rental_order.return"])?;

        let order = self.get_order(order_id).await?;
        let lines = order.lines.iter().filter(|l| line_ids.contains(&l.id())).collect::<Vec<_>>();

        // 1. Pénalités (RentalPricingOperator)
        let delay_costs = self.rental_pricing
            .compute_delay_costs(&lines, actual_return_date)
            .await?;

        // 2. Mouvements retour (RentalStockOperator → KindMother)
        for line in &lines {
            self.rental_stock
                .create_return_movement(
                    line.product_id,
                    line.quantity,
                    actual_return_date,
                    order_id,
                    mandate.clone(),
                )
                .await?;
        }

        // 3. Mise à jour lignes : state = Returned, delay_costs
        let write_intent = WriteIntent {
            entity_type: "rental.order.line",
            operation: WriteOperation::Update,
            data: RentalLineUpdate {
                ids: line_ids,
                state: RentalLineState::Returned,
                actual_return_date,
                delay_costs: delay_costs.clone(),
            },
            security_level: order.security_level,
        };
        self.kind_mother.execute(write_intent).await?;

        // 4. Si toutes les lignes retournées → state commande = Return
        let all_returned = order.lines.iter().all(|l| line_ids.contains(&l.id()) || l.state == RentalLineState::Returned);
        if all_returned {
            let wi_order = WriteIntent {
                entity_type: "rental.order",
                operation: WriteOperation::Update,
                data: RentalOrderUpdate { id: order_id, state: RentalOrderState::Return, ..default() },
                security_level: order.security_level,
            };
            self.kind_mother.execute(wi_order).await?;
        }

        Ok(self.get_order(order_id).await?)
    }
}
```

### 2.4 Calcul prix (RentalPricingOperator)

**Pattern :** Pas de WriteIntent ; lecture grilles + calcul ; résultat retourné à l'appelant.

```rust
// Pseudo-code Rust
impl RentalPricingOperator {
    /// Règle : une seule ligne de prix utilisée ; option la moins chère pour couvrir la durée.
    pub async fn compute_price(
        &self,
        product_id: Uuid,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        quantity: Decimal,
    ) -> Result<PricedRentalLine, RentalError> {
        let config = self.get_rental_config(product_id).await?;
        let duration = self.duration_in_units(start_date, end_date, config.unit); // heure, jour, semaine, mois
        let (price_line, multiplier) = self.cheapest_combination(&config.prices, duration)?;
        let total = price_line.price * multiplier * quantity;
        Ok(PricedRentalLine {
            product_id,
            start_date,
            end_date,
            quantity,
            unit: config.unit,
            duration,
            unit_price: price_line.price,
            total,
            price_rule_explanation: format!("{} × {} {}", multiplier, price_line.duration, config.unit),
        })
    }
}
```

---

## 3. Gestion des Mandats

### 3.1 Émission Mandat (StrongFather)

- Contexte : utilisateur commercial ou magasinier, session « Location »
- Opérateurs autorisés : RentalOrderOperator, RentalPricingOperator, RentalStockOperator
- Flux autorisés : rental_order.create, rental_order.update, rental_order.confirm, rental_order.pickup, rental_order.return
- Niveau sécurité max : 2 (Sensitive)
- Révocation : fin de session, changement de rôle, alerte WorrySentinel

### 3.2 Validation Mandat (côté Opérateur)

- Vérifier que le Mandat inclut l'Opérateur courant
- Vérifier que le flux demandé est dans la liste des flux autorisés
- Vérifier que le niveau de sécurité du contexte ≤ mandate.max_security_level
- Refuser l'action si Mandat expiré ou révoqué

---

## 4. Récapitulatif des WriteIntent

| Entité | Opérations | Déclencheur |
|--------|------------|-------------|
| rental.order | Create, Update | RentalOrderOperator (création, confirmation, mise à jour statut) |
| rental.order.line | Create, Update | RentalOrderOperator |
| rental.stock.movement | Create | RentalStockOperator (pickup, return) |
| account.move.line (pénalités) | Create | RentalOrderOperator + MiyuInvoice (facturation) |
| rental.product.config | Create, Update | RentalPricingOperator (configuration grilles) |

Toutes les écritures passent par KindMother ; aucun écriture directe en base depuis les Opérateurs.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
