# Odoo Email Marketing — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Email Marketing dans l'architecture COG Miyukini, en respectant la gouvernance, les WriteIntent et les Mandats de Permission.

**Références :**
- [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Email%20Marketing%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Email Marketing
- Patterns d'implémentation (WriteIntent, Mandats)
- Exemples de code (pseudo-code Rust)
- Gestion des erreurs et rollback

**Hors scope :**
- Implémentation complète (voir Guide d'Implémentation)
- Spécifications UI/UX

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────┐
│                    MailingUI (Opérateur Interface)         │
│                    Niveau sécurité: 1                       │
└───────────────────────┬─────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌─────▼──────┐
│MailingCampaign│ │MailingList  │ │MailingContact│
│   Operator    │ │  Operator   │ │  Operator   │
│   (S2)        │ │   (S2)      │ │   (S2)      │
└───────┬──────┘ └──────┬──────┘ └─────┬──────┘
        │               │               │
        └───────────────┼───────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌─────▼──────┐
│StrongFather  │ │ KindMother  │ │WorrySentinel│
│  (Décision)  │ │ (Persistance)│ │(Quota/Blacklist)│
└──────────────┘ └──────────────┘ └────────────┘
```

### 1.2 Flux de données standard

**Création d’un mailing (brouillon) :**
```
MailingUI → MailingCampaignOperator → Master Butler (permissions)
         → WorrySentinel (niveau données) → StrongFather (décision)
         → KindMother (WriteIntent) → Persistance
```

**Envoi ou planification :**
```
MailingUI → MailingCampaignOperator → WorrySentinel (quota, blacklist)
         → StrongFather (décision envoi) → KindMother (WriteIntent traces)
         → Opérateur envoi SMTP (exécution)
```

**Désabonnement (portail) :**
```
Façade publique → MailingContactOperator (unsubscribe)
         → MailingBlacklistOperator (option blacklist)
         → KindMother (WriteIntent) → Persistance
```

---

## 2. Patterns d'implémentation

### 2.1 Pattern : WriteIntent vers KindMother (création mailing)

**Principe :** Toute création ou modification de mailing, liste, contact, blacklist ou trace passe par WriteIntent vers KindMother.

**Pseudo-code Rust (création mailing) :**

```rust
// Dans MailingCampaignOperator
pub async fn create_mailing(
    &self,
    ctx: &OperatorContext,
    draft: MailingDraft,
) -> Result<MailingId, EmailMarketingError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "mailing.create").await?;
    worry_sentinel::check_security_level(ctx.environment_id(), SecurityLevel::Sensitive).await?;

    let decision = strong_father::decide(
        ctx.environment_id(),
        DecisionRequest::CreateMailing { draft: &draft },
    ).await?;

    match decision {
        Decision::Approved => {
            let write_intent = WriteIntent {
                operation: WriteOperation::CreateMailing { draft: draft.clone() },
                source: ctx.operator_id().clone(),
                environment_id: ctx.environment_id().clone(),
            };
            let id = kind_mother::submit(write_intent).await?;
            Ok(id)
        }
        Decision::Rejected(reason) => Err(EmailMarketingError::GovernanceRejected(reason)),
    }
}
```

### 2.2 Pattern : Envoi massif (Mandat StrongFather + WorrySentinel)

**Principe :** L’envoi (ou la planification) est un acte de gouvernance : StrongFather décide, WorrySentinel vérifie quota et blacklist.

**Pseudo-code Rust (envoi) :**

```rust
// Dans MailingCampaignOperator
pub async fn send_mailing(
    &self,
    ctx: &OperatorContext,
    mailing_id: MailingId,
) -> Result<(), EmailMarketingError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "mailing.send").await?;

    let mailing = kind_mother::read::<Mailing>(mailing_id).await?;
    if mailing.state != MailingState::Draft && mailing.state != MailingState::InQueue {
        return Err(EmailMarketingError::InvalidState(mailing.state));
    }

    // WorrySentinel : quota et blacklist
    worry_sentinel::check_mailing_quota(ctx.environment_id(), mailing.recipient_count()).await?;
    let excluded = mailing_blacklist_operator::excluded_emails(&mailing.recipient_emails()).await?;
    let effective_count = mailing.recipient_count() - excluded.len();

    let decision = strong_father::decide(
        ctx.environment_id(),
        DecisionRequest::SendMailing {
            mailing_id,
            recipient_count: effective_count,
        },
    ).await?;

    match decision {
        Decision::Approved => {
            // WriteIntent : mise à jour état + création traces
            let write_intent = WriteIntent {
                operation: WriteOperation::SendMailing {
                    mailing_id,
                    excluded_emails: excluded,
                },
                source: ctx.operator_id().clone(),
                environment_id: ctx.environment_id().clone(),
            };
            kind_mother::submit(write_intent).await?;
            // Déclencher l’exécution SMTP (Opérateur ou Kit dédié, hors Core)
            mail_send_operator::enqueue_mailing(mailing_id).await?;
            Ok(())
        }
        Decision::Rejected(reason) => Err(EmailMarketingError::GovernanceRejected(reason)),
    }
}
```

### 2.3 Pattern : Désabonnement (portail, Façade publique)

**Principe :** Le portail désabonnement est une Façade publique gouvernée ; pas d’entrée dans le COG ; l’utilisateur soumet une intention (unsubscribe, option blacklist) ; un Mandat Public d’Accès encadre l’action ; l’écriture passe par WriteIntent.

**Pseudo-code Rust (côté COG, traitement de l’intention) :**

```rust
// Traitement de l’intention « unsubscribe » (depuis BondingBrother / Façade)
pub async fn process_unsubscribe_intent(
    &self,
    ctx: &OperatorContext,
    email: EmailAddress,
    list_ids: Vec<ListId>,
    add_to_blacklist: bool,
) -> Result<(), EmailMarketingError> {
    let mandate = ctx.mandate()?;
    // Mandat Public ou Mandat limité au désabonnement
    master_butler::check_capability(&mandate, "mailing.unsubscribe").await?;

    let write_intent_contact = WriteIntent {
        operation: WriteOperation::UnsubscribeContacts {
            email: email.clone(),
            list_ids,
        },
        source: ctx.operator_id().clone(),
        environment_id: ctx.environment_id().clone(),
    };
    kind_mother::submit(write_intent_contact).await?;

    if add_to_blacklist {
        let write_intent_blacklist = WriteIntent {
            operation: WriteOperation::AddToBlacklist { email },
            source: ctx.operator_id().clone(),
            environment_id: ctx.environment_id().clone(),
        };
        kind_mother::submit(write_intent_blacklist).await?;
    }
    Ok(())
}
```

### 2.4 Pattern : Résolution des destinataires (filtre dynamique)

**Principe :** MailingFilterOperator résout les destinataires en lecture seule sur les modèles cibles (Contacts, CRM, Events, Sales) ; pas d’écriture sur ces modèles ; BondingBrother médie les lectures selon Mandat.

**Pseudo-code Rust (résolution filtre) :**

```rust
// Dans MailingFilterOperator
pub async fn resolve_recipients(
    &self,
    ctx: &OperatorContext,
    filter_id: FilterId,
) -> Result<Vec<EmailAddress>, EmailMarketingError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "mailing_filter.resolve").await?;

    let filter = kind_mother::read::<MailingFilter>(filter_id).await?;
    // Lecture seule sur le modèle cible (Contact, CRM, Event, Sale) via BondingBrother
    let records = bonding_brother::query(
        ctx.environment_id(),
        &filter.model_id,
        &filter.mailing_domain,
        QueryIntent::ReadOnly,
    ).await?;
    let emails = records.extract_emails(); // selon schéma du modèle
    Ok(emails)
}
```

---

## 3. Gestion des erreurs et rollback

- **GovernanceRejected :** Si StrongFather ou WorrySentinel rejette (quota, blacklist, politique), retourner une erreur explicite sans écrire.
- **WriteIntent en échec :** Si KindMother rejette ou échoue (conflit, contrainte), ne pas appliquer l’action ; rollback implicite (pas de persistance partielle).
- **Envoi SMTP en échec :** Les traces (MailingTraceOperator) doivent refléter l’état (exception, bounce) via WriteIntent ; pas de correction automatique par le Core (Maintenance explicable).

---

## 4. Intégration avec Kits existants

- **MiyuContacts :** MailingFilterOperator et MailingContactOperator consomment en lecture (ciblage, lien partner_id) ; pas d’écriture sur Contacts depuis Email Marketing sauf via contrats explicites (ex. préférences).
- **MiyuCRM / MiyuSales / MiyuEvents :** MailingFilterOperator consomme en lecture pour filtres dynamiques ; métriques campagnes (Opportunities, Revenues, Quotations) via Contrat d’équipe et agrégation par MailingCampaignUTMOperator.
- **Envoi SMTP :** Déléguer à un Kit ou Opérateur dédié (hors Cores) ; MailingCampaignOperator déclenche l’envoi après validation StrongFather et WriteIntent traces.

---

**Document :** Odoo Email Marketing — Guide d'Intégration COG  
**Version :** 1.0  
**Date :** 2026-02-01
