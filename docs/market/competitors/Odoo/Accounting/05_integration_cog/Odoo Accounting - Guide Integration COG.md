# Odoo Accounting — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Accounting dans l'architecture COG Miyukini, en respectant la gouvernance, les WriteIntent, et les Mandats de Permission.

**Références :**
- [Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Accounting%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Miyukini Account - Document Fondateur](../../../../services/MiyukiniAccount/Miyukini%20Account%20-%20Document%20Fondateur.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG
- Patterns d'implémentation
- Exemples de code (pseudo-code Rust)
- Gestion des WriteIntent
- Gestion des Mandats
- Gestion des erreurs et rollback

**Hors scope :**
- Implémentation complète (voir Guide d'Implémentation)
- Spécifications UI/UX

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
┌─────────────────────────────────────────────────────────────┐
│                    AccountUI (Opérateur Interface)          │
│                    Niveau sécurité: 1                       │
└───────────────────────┬─────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌─────▼──────┐
│ AccountLedger│ │AccountJournal│ │AccountChart│
│   (S2)       │ │    (S2)      │ │   (S2)     │
└───────┬──────┘ └──────┬──────┘ └─────┬──────┘
        │               │               │
        └───────────────┼───────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌─────▼──────┐
│StrongFather  │ │ KindMother  │ │MasterButler│
│  (Décision)  │ │ (Persistance)│ │(Permissions)│
└──────────────┘ └──────────────┘ └────────────┘
```

### 1.2 Flux de Données Standard

**Création d'écriture :**
```
AccountUI → AccountLedger → Master Butler (permissions)
         → WorrySentinel (sécurité) → StrongFather (décision)
         → KindMother (WriteIntent) → Persistance
```

**Validation d'écriture :**
```
AccountUI → AccountLedger → StrongFather (décision validation)
         → Ever Buddy (séquence) → KindMother (WriteIntent)
         → Persistance
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : WriteIntent vers KindMother

**Principe :** Toute écriture comptable passe par WriteIntent vers KindMother.

**Pseudo-code Rust :**

```rust
// Dans AccountLedger
pub async fn create_entry(
    &self,
    ctx: &OperatorContext,
    entry: EntryDraft,
) -> Result<EntryId, AccountError> {
    // 1. Vérification permissions (Master Butler)
    let mandate = ctx.mandate()?;
    master_butler::check_capability(
        &mandate,
        "ledger.entry.create",
    ).await?;
    
    // 2. Vérification sécurité (WorrySentinel)
    worry_sentinel::check_security_level(
        &ctx.environment_id(),
        SecurityLevel::Sensitive,
    ).await?;
    
    // 3. Décision StrongFather
    let decision = strong_father::decide(
        &ctx.environment_id(),
        DecisionRequest::CreateEntry {
            entry: &entry,
        },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 4. WriteIntent vers KindMother
            let write_intent = WriteIntent {
                operation: WriteOperation::CreateEntry {
                    entry: entry.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            
            let entry_id = kind_mother::write(
                &ctx.environment_id(),
                write_intent,
            ).await?;
            
            Ok(entry_id)
        }
        Decision::Rejected { reason } => {
            Err(AccountError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.2 Pattern : Validation avec Séquence

**Principe :** La validation d'écriture génère un numéro de séquence via Ever Buddy.

**Pseudo-code Rust :**

```rust
// Dans AccountLedger
pub async fn validate_entry(
    &self,
    ctx: &OperatorContext,
    entry_id: EntryId,
) -> Result<ValidatedEntry, AccountError> {
    // 1. Lecture de l'écriture (KindMother)
    let entry = kind_mother::read::<Entry>(
        &ctx.environment_id(),
        entry_id.clone(),
    ).await?;
    
    // 2. Vérification équilibre comptable
    if !entry.is_balanced() {
        return Err(AccountError::UnbalancedEntry);
    }
    
    // 3. Décision StrongFather
    let decision = strong_father::decide(
        &ctx.environment_id(),
        DecisionRequest::ValidateEntry {
            entry_id: entry_id.clone(),
        },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 4. Génération numéro séquence (Ever Buddy)
            let journal = account_journal::get(
                &ctx.environment_id(),
                entry.journal_id(),
            ).await?;
            
            let sequence_number = ever_buddy::generate_sequence(
                &ctx.environment_id(),
                journal.sequence_id(),
                entry.date(),
            ).await?;
            
            // 5. WriteIntent validation vers KindMother
            let write_intent = WriteIntent {
                operation: WriteOperation::ValidateEntry {
                    entry_id: entry_id.clone(),
                    sequence_number: sequence_number.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            
            let validated_entry = kind_mother::write(
                &ctx.environment_id(),
                write_intent,
            ).await?;
            
            Ok(validated_entry)
        }
        Decision::Rejected { reason } => {
            Err(AccountError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.3 Pattern : Réconciliation avec Validation

**Principe :** La réconciliation nécessite une décision StrongFather avant WriteIntent.

**Pseudo-code Rust :**

```rust
// Dans AccountReconciliation
pub async fn reconcile(
    &self,
    ctx: &OperatorContext,
    debit_line_id: LineId,
    credit_line_id: LineId,
    amount: Amount,
) -> Result<ReconciliationId, AccountError> {
    // 1. Lecture des lignes (KindMother)
    let debit_line = kind_mother::read::<MoveLine>(
        &ctx.environment_id(),
        debit_line_id.clone(),
    ).await?;
    
    let credit_line = kind_mother::read::<MoveLine>(
        &ctx.environment_id(),
        credit_line_id.clone(),
    ).await?;
    
    // 2. Vérification compatibilité
    if !debit_line.is_reconcilable() || !credit_line.is_reconcilable() {
        return Err(AccountError::NotReconcilable);
    }
    
    // 3. Décision StrongFather (validation réconciliation)
    let decision = strong_father::decide(
        &ctx.environment_id(),
        DecisionRequest::Reconcile {
            debit_line_id: debit_line_id.clone(),
            credit_line_id: credit_line_id.clone(),
            amount: amount.clone(),
        },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 4. WriteIntent réconciliation vers KindMother
            let write_intent = WriteIntent {
                operation: WriteOperation::Reconcile {
                    debit_line_id: debit_line_id.clone(),
                    credit_line_id: credit_line_id.clone(),
                    amount: amount.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            
            let reconciliation_id = kind_mother::write(
                &ctx.environment_id(),
                write_intent,
            ).await?;
            
            Ok(reconciliation_id)
        }
        Decision::Rejected { reason } => {
            Err(AccountError::RejectedByGovernance(reason))
        }
    }
}
```

---

## 3. Gestion des Mandats de Permission

### 3.1 Obtention d'un Mandat

**Pattern standard :**

```rust
// Dans AccountLedger
pub async fn ensure_mandate(
    &self,
    ctx: &OperatorContext,
) -> Result<Mandate, AccountError> {
    // Vérification mandat existant
    if let Some(mandate) = ctx.mandate() {
        // Vérification validité
        if mandate.is_valid() {
            return Ok(mandate);
        }
    }
    
    // Demande nouveau mandat (StrongFather)
    let mandate = strong_father::request_mandate(
        &ctx.environment_id(),
        MandateRequest {
            operators: vec![
                ctx.operator_id().clone(),
                OperatorId::from("account.ledger"),
            ],
            capabilities: vec![
                "ledger.entry.create".into(),
                "ledger.entry.validate".into(),
            ],
            security_level: SecurityLevel::Sensitive,
            duration: Duration::hours(8),
        },
    ).await?;
    
    Ok(mandate)
}
```

### 3.2 Utilisation d'un Mandat

**Pattern dans les méthodes :**

```rust
pub async fn create_entry(
    &self,
    ctx: &OperatorContext,
    entry: EntryDraft,
) -> Result<EntryId, AccountError> {
    // Obtention mandat
    let mandate = self.ensure_mandate(ctx).await?;
    
    // Vérification permissions (Master Butler)
    master_butler::check_capability_with_mandate(
        &mandate,
        "ledger.entry.create",
    ).await?;
    
    // ... reste de l'implémentation
}
```

---

## 4. Gestion des Erreurs et Rollback

### 4.1 Pattern : Rollback sur Erreur

**Principe :** En cas d'erreur après WriteIntent, KindMother gère le rollback automatiquement.

**Pseudo-code Rust :**

```rust
pub async fn create_entry_with_validation(
    &self,
    ctx: &OperatorContext,
    entry: EntryDraft,
) -> Result<EntryId, AccountError> {
    // 1. WriteIntent création
    let write_intent = WriteIntent {
        operation: WriteOperation::CreateEntry {
            entry: entry.clone(),
        },
        source: ctx.operator_id().clone(),
        mandate_id: ctx.mandate()?.id().clone(),
    };
    
    let entry_id = match kind_mother::write(
        &ctx.environment_id(),
        write_intent,
    ).await {
        Ok(id) => id,
        Err(e) => {
            // KindMother gère le rollback automatiquement
            return Err(AccountError::WriteFailed(e));
        }
    };
    
    // 2. Validation automatique (si configuré)
    if entry.auto_validate {
        match self.validate_entry(ctx, entry_id.clone()).await {
            Ok(_) => Ok(entry_id),
            Err(e) => {
                // Rollback géré par KindMother si WriteIntent échoue
                // L'écriture reste en draft
                Err(AccountError::ValidationFailed(e))
            }
        }
    } else {
        Ok(entry_id)
    }
}
```

### 4.2 Pattern : Gestion d'Erreurs Gouvernance

**Types d'erreurs :**

```rust
#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Rejected by governance: {0}")]
    RejectedByGovernance(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Security level insufficient: required {0}, got {1}")]
    SecurityLevelInsufficient(SecurityLevel, SecurityLevel),
    
    #[error("Write failed: {0}")]
    WriteFailed(#[from] KindMotherError),
    
    #[error("Unbalanced entry")]
    UnbalancedEntry,
    
    #[error("Entry not found: {0}")]
    EntryNotFound(EntryId),
    
    #[error("Not reconcilable")]
    NotReconcilable,
    
    // ... autres erreurs
}
```

---

## 5. Intégration avec les Kits Existants

### 5.1 Utilisation de MiyuInvoice

**Pattern :** AccountInvoice utilise MiyuInvoice pour la facturation, puis AccountLedger pour la comptabilisation.

```rust
// Dans AccountInvoice
pub async fn create_invoice(
    &self,
    ctx: &OperatorContext,
    invoice_data: InvoiceData,
) -> Result<InvoiceId, AccountError> {
    // 1. Création facture via MiyuInvoice
    let invoice_id = miyu_invoice::create(
        &ctx.environment_id(),
        invoice_data.clone(),
    ).await?;
    
    // 2. Comptabilisation automatique (si configuré)
    if invoice_data.auto_post {
        let entry = self.invoice_to_entry(
            invoice_id.clone(),
        ).await?;
        
        account_ledger::create_entry(
            ctx,
            entry,
        ).await?;
    }
    
    Ok(invoice_id)
}
```

### 5.2 Utilisation de MiyuComptaLedger

**Pattern :** AccountReconciliation utilise MiyuComptaLedger pour les outils de réconciliation.

```rust
// Dans AccountReconciliation
pub async fn suggest_reconciliations(
    &self,
    ctx: &OperatorContext,
    statement_lines: Vec<StatementLine>,
) -> Result<Vec<ReconciliationSuggestion>, AccountError> {
    // Utilisation outil MiyuComptaLedger
    let suggestions = miyu_compta_ledger::reconciliation_suggest(
        &ctx.environment_id(),
        statement_lines,
    ).await?;
    
    Ok(suggestions)
}
```

---

## 6. Tests d'Intégration COG

### 6.1 Test : Création d'Écriture avec Gouvernance

```rust
#[tokio::test]
async fn test_create_entry_with_governance() {
    let ctx = create_test_context().await;
    
    // Création écriture
    let entry = EntryDraft {
        journal_id: JournalId::from("journal.001"),
        date: Date::today(),
        lines: vec![
            LineDraft {
                account_id: AccountId::from("account.701000"),
                debit: Amount::from(1000.0),
                credit: Amount::zero(),
            },
            LineDraft {
                account_id: AccountId::from("account.411000"),
                debit: Amount::zero(),
                credit: Amount::from(1000.0),
            },
        ],
    };
    
    // Création via AccountLedger
    let entry_id = account_ledger::create_entry(
        &ctx,
        entry,
    ).await.unwrap();
    
    // Vérification WriteIntent vers KindMother
    let persisted_entry = kind_mother::read::<Entry>(
        &ctx.environment_id(),
        entry_id.clone(),
    ).await.unwrap();
    
    assert_eq!(persisted_entry.state(), EntryState::Draft);
    assert!(persisted_entry.is_balanced());
}
```

### 6.2 Test : Validation avec Séquence

```rust
#[tokio::test]
async fn test_validate_entry_with_sequence() {
    let ctx = create_test_context().await;
    
    // Création écriture
    let entry_id = create_test_entry(&ctx).await;
    
    // Validation
    let validated_entry = account_ledger::validate_entry(
        &ctx,
        entry_id.clone(),
    ).await.unwrap();
    
    // Vérification numéro séquence généré
    assert!(validated_entry.sequence_number().is_some());
    assert_eq!(validated_entry.state(), EntryState::Posted);
    
    // Vérification WriteIntent vers KindMother
    let persisted_entry = kind_mother::read::<Entry>(
        &ctx.environment_id(),
        entry_id.clone(),
    ).await.unwrap();
    
    assert_eq!(persisted_entry.state(), EntryState::Posted);
}
```

---

## 7. Checklist d'Intégration

### 7.1 Avant Implémentation

- [ ] Opérateurs identifiés et spécifiés
- [ ] Contrat d'Équipe défini
- [ ] Mandats de Permission spécifiés
- [ ] Niveaux de sécurité définis
- [ ] Intégration avec Cores planifiée

### 7.2 Pendant Implémentation

- [ ] WriteIntent vers KindMother implémenté
- [ ] Décisions StrongFather implémentées
- [ ] Vérifications Master Butler implémentées
- [ ] Vérifications WorrySentinel implémentées
- [ ] Gestion des Mandats implémentée
- [ ] Gestion des erreurs implémentée

### 7.3 Après Implémentation

- [ ] Tests d'intégration COG passés
- [ ] Tests de gouvernance passés
- [ ] Tests de sécurité passés
- [ ] Documentation mise à jour

---

## 8. Conclusion

Le **guide d'intégration COG** fournit :

- **Patterns d'implémentation** pour WriteIntent, Mandats, Gouvernance
- **Exemples de code** (pseudo-code Rust) pour chaque pattern
- **Gestion des erreurs** et rollback
- **Intégration avec Kits existants** (MiyuInvoice, MiyuComptaLedger)
- **Tests d'intégration** pour valider la gouvernance

**Prochaines étapes :** Voir [Guide d'Implémentation](./06_guides_implementation/Odoo%20Accounting%20-%20Guide%20Implementation.md) pour les spécifications techniques détaillées.

---

**Document** : Odoo Accounting — Guide d'Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Guide pratique — référence pour implémentation
