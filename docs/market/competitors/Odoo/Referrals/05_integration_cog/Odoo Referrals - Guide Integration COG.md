# Odoo Referrals — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Referrals dans l'architecture COG Miyukini, en respectant la gouvernance, les WriteIntent et les Mandats de Permission.

**Références :**
- [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Referrals%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Referrals
- Patterns d'implémentation (WriteIntent, Mandats, crédit points, achat récompense)
- Exemples de code (pseudo-code Rust)
- Gestion des erreurs et rollback

**Hors scope :**
- Implémentation complète (voir Guide d'Implémentation)
- Spécifications UI/UX

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────┐
│                 ReferralsUI (Opérateur Interface)               │
│                 Niveau sécurité: 1                              │
└───────────────────────────┬─────────────────────────────────────┘
                            │
    ┌───────────────────────┼───────────────────────┬─────────────────────┐
    │                       │                       │                     │
┌───▼────────────┐  ┌───────▼────────┐  ┌──────────▼──────────┐  ┌───────▼────────┐
│ReferralsPoints │  │ReferralsRewards │  │ ReferralsShare       │  │ReferralsReporting│
│    (S1–S2)     │  │    (S1–S2)      │  │     (S1)            │  │     (S2)        │
└───┬────────────┘  └───────┬────────┘  └──────────┬──────────┘  └───────┬────────┘
    │                       │                       │                     │
    └───────────────────────┼───────────────────────┘                     │
                            │                                              │
    ┌───────────────────────┼───────────────────────┐                     │
    │                       │                       │                     │
┌───▼────────────┐  ┌───────▼────────┐  ┌──────────▼──────────┐        │
│ KindMother     │  │ Master Butler   │  │ MiyuRecruitment      │        │
│ (WriteIntent)  │  │ (Permissions)   │  │ (events stage)       │◄───────┘
└────────────────┘  └────────────────┘  └──────────────────────┘
```

### 1.2 Flux de Données Standard

**Crédit de points (événement Recruitment) :**
```
MiyuRecruitment (candidature change de stage, referrer_id présent)
  → ReferralsPoints.credit_for_stage(referrer_id, applicant_id, stage_id)
  → Master Butler (vérif permission / contexte)
  → KindMother (WriteIntent points movement + balance)
  → Persistance
```

**Achat récompense :**
```
ReferralsUI → ReferralsRewards.buy(reward_id)
  → ReferralsPoints.check_balance(user) puis debit(amount)
  → Master Butler (Mandat ReferralsUser)
  → KindMother (WriteIntent reward_purchase, WriteIntent points debit)
  → MiyuNotify (alerte Gift Responsible)
  → Persistance
```

**Partage poste (email) :**
```
ReferralsUI → ReferralsShare.send_email(job_id, recipient_emails, template_context)
  → Master Butler (Mandat ReferralsUser)
  → Génération lien de suivi (referrer_id dans token/URL)
  → MiyuNotify (envoi email avec lien)
  → Pas de WriteIntent Referrals (lecture seule + envoi)
```

**Level up :**
```
ReferralsUI → ReferralsLevels.level_up(user_id)
  → ReferralsPoints.total_earned(user_id) >= next_level_required
  → Master Butler (Mandat ReferralsUser)
  → KindMother (WriteIntent current_level update)
  → Pas de débit de points
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : WriteIntent vers KindMother (Mouvement de points)

**Principe :** Tout crédit ou débit de points passe par WriteIntent vers KindMother.

**Pseudo-code Rust :**

```rust
// Dans ReferralsPoints
pub async fn credit_for_stage(
    &self,
    ctx: &OperatorContext,
    referrer_id: EmployeeId,
    applicant_id: ApplicantId,
    stage_id: StageId,
    points: u32,
) -> Result<(), ReferralsError> {
    let mandate = ctx.mandate().ok_or(ReferralsError::MandateRequired)?;
    self.master_butler.check(mandate, Permission::ReferralsCredit).await?;

    let intent = WriteIntent::PointsCredit {
        referrer_id,
        applicant_id,
        stage_id,
        points,
        at: ctx.clock().now(),
    };
    self.kind_mother.submit(intent).await?;
    Ok(())
}

pub async fn debit_for_reward(
    &self,
    ctx: &OperatorContext,
    referrer_id: EmployeeId,
    reward_id: RewardId,
    cost: u32,
) -> Result<(), ReferralsError> {
    let balance = self.balance(ctx, referrer_id).await?.to_spend;
    if balance < cost {
        return Err(ReferralsError::InsufficientPoints);
    }
    let mandate = ctx.mandate().ok_or(ReferralsError::MandateRequired)?;
    self.master_butler.check(mandate, Permission::ReferralsBuy).await?;

    let intent = WriteIntent::PointsDebit {
        referrer_id,
        reward_id,
        cost,
        at: ctx.clock().now(),
    };
    self.kind_mother.submit(intent).await?;
    Ok(())
}
```

### 2.2 Pattern : Achat récompense (coordination ReferralsRewards + ReferralsPoints)

**Principe :** ReferralsRewards orchestre la vérification du solde, le débit et l’enregistrement de l’achat ; ReferralsPoints exécute le débit via KindMother.

**Pseudo-code Rust :**

```rust
// Dans ReferralsRewards
pub async fn buy(
    &self,
    ctx: &OperatorContext,
    reward_id: RewardId,
) -> Result<RewardPurchaseId, ReferralsError> {
    let user = ctx.current_employee()?;
    let reward = self.get_reward(reward_id).await?;
    let cost = reward.cost;

    self.referrals_points.debit_for_reward(ctx, user.id, reward_id, cost).await?;

    let intent = WriteIntent::RewardPurchase {
        referrer_id: user.id,
        reward_id,
        cost,
        gift_responsible_id: reward.gift_responsible_id,
        at: ctx.clock().now(),
    };
    let purchase_id = self.kind_mother.submit(intent).await?;

    if let Some(resp_id) = reward.gift_responsible_id {
        self.miyu_notify.notify_reward_purchased(ctx, purchase_id, resp_id).await?;
    }
    Ok(purchase_id)
}
```

### 2.3 Pattern : Réception événement Recruitment (crédit points)

**Principe :** Lorsqu’un candidat parrainé change de stage, Recruitment (ou un bridge) émet un événement ; ReferralsPoints écoute et crédite les points selon la config des stages.

**Pseudo-code Rust :**

```rust
// Abonnement événement (côté Referrals ou Central)
async fn on_applicant_stage_changed(event: ApplicantStageChanged) {
    let referrer_id = event.referrer_id?; // si absent, ignorer
    let stage_points = get_stage_points(event.stage_id).await?; // config Recruitment
    referrals_points
        .credit_for_stage(ctx, referrer_id, event.applicant_id, event.stage_id, stage_points)
        .await?;
}
```

### 2.4 Pattern : Génération lien de suivi (ReferralsShare)

**Principe :** Génération d’un token ou paramètre sécurisé (signé, non falsifiable) contenant l’identifiant du référent ; URL = base Website + chemin poste/liste + token.

**Pseudo-code Rust :**

```rust
// Dans ReferralsShare
pub fn build_tracking_link(
    &self,
    referrer_id: EmployeeId,
    job_id: Option<JobId>,
    base_url: &str,
) -> Result<String, ReferralsError> {
    let payload = TrackingPayload {
        referrer_id,
        job_id,
        exp: self.clock().now() + TRACKING_LINK_TTL,
    };
    let token = self.sign_and_encode(&payload)?;
    let path = job_id
        .map(|id| format!("/jobs/{}", id))
        .unwrap_or_else(|| "/jobs".to_string());
    Ok(format!("{}{}?ref={}", base_url, path, token))
}
```

---

## 3. Gestion des Erreurs et Rollback

- **InsufficientPoints** : refuser l’achat, pas de débit. Pas de rollback côté Referrals si la vérification du solde est faite avant WriteIntent.
- **MandateRequired / PermissionDenied** : retour 403, pas d’écriture.
- **KindMother reject** : rollback local si une partie des WriteIntent a déjà été envoyée (idéalement une seule WriteIntent atomique par opération métier).
- **Notify failure** (Gift Responsible) : l’achat reste enregistré ; log + retry ou file pour notification.

---

## 4. Intégration avec Kits Existants

| Kit / Opérateur | Usage |
|----------------|--------|
| **MiyuHR** | Identité du référent (employé courant), droits |
| **MiyuRecruitment** | Candidatures, stages, referrer_id, événements passage de stage, config points par stage |
| **MiyuWeb** | Postes publiés, URLs, formulaire candidature avec récupération referrer_id depuis lien |
| **MiyuNotify** | Envoi email/SMS/WhatsApp partage postes, notification achat récompense |
| **Miyukini-kernel** | Id, Logger, Clock |

---

**Document** : Odoo Referrals — Guide d'Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01
