# Odoo Recruitment — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Recruitment dans l'architecture COG Miyukini, en respectant la gouvernance, les WriteIntent, et les Mandats de Permission.

**Références :**
- [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Recruitment%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG
- Patterns d'implémentation (WriteIntent, Mandats)
- Exemples de code (pseudo-code Rust)
- Gestion des erreurs et rollback

**Hors scope :**
- Implémentation complète (voir Guide d'Implémentation)
- Spécifications UI/UX

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
┌─────────────────────────────────────────────────────────────────┐
│                 RecruitmentUI (Opérateur Interface)             │
│                 Niveau sécurité: 1                               │
└───────────────────────────┬─────────────────────────────────────┘
                            │
    ┌───────────────────────┼───────────────────────┐
    │                       │                       │
┌───▼────────────┐  ┌───────▼────────┐  ┌──────────▼──────────┐
│ RecruitmentJob │  │RecruitmentApplicant│  │RecruitmentStage   │
│     (S1–S2)    │  │      (S2)         │  │      (S1)          │
└───┬────────────┘  └───────┬────────┘  └──────────┬──────────┘
    │                       │                       │
    └───────────────────────┼───────────────────────┘
                            │
    ┌───────────────────────┼───────────────────────┐
    │                       │                       │
┌───▼────────────┐  ┌───────▼────────┐  ┌──────────▼──────────┐
│ StrongFather   │  │ KindMother      │  │ Master Butler        │
│ (Refuse, Hire) │  │ (WriteIntent)   │  │ (Permissions)        │
└────────────────┘  └────────────────┘  └─────────────────────┘
```

### 1.2 Flux de Données Standard

**Création de candidature :**
```
RecruitmentUI → RecruitmentApplicant → Master Butler (permissions)
             → WorrySentinel (sécurité) → KindMother (WriteIntent)
             → Persistance
```

**Passage de stage :**
```
RecruitmentUI → RecruitmentApplicant → Master Butler (permissions)
             → KindMother (WriteIntent stage_id) → Persistance
             → [Optionnel] Envoi email (template stage) via MiyuNotify
```

**Refus candidat :**
```
RecruitmentUI → RecruitmentApplicant → StrongFather (décision refus)
             → KindMother (WriteIntent refuse_reason_id, state/archive)
             → [Optionnel] Envoi email (template Refuse) via MiyuNotify
```

**Création employé depuis candidature :**
```
RecruitmentUI → RecruitmentApplicant → StrongFather (décision embauche)
             → KindMother (WriteIntent hr.employee) + Intégration MiyuHR
             → Persistance employé, liaison candidature → employé
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : WriteIntent vers KindMother (Candidature)

**Principe :** Toute création ou modification de candidature passe par WriteIntent vers KindMother.

**Pseudo-code Rust :**

```rust
// Dans RecruitmentApplicant
pub async fn create_applicant(
    &self,
    ctx: &OperatorContext,
    draft: ApplicantDraft,
) -> Result<ApplicantId, RecruitmentError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "applicant.create").await?;
    worry_sentinel::check_security_level(ctx.env(), 2).await?;

    let intent = WriteIntent::create_applicant(draft);
    let id = kind_mother::submit(&mandate, intent).await?;
    Ok(id)
}
```

### 2.2 Pattern : Passage de stage (Transition)

**Principe :** Le changement de stage est une mise à jour gouvernée (WriteIntent) ; l'envoi d'email automatique est optionnel (template sur le stage).

```rust
// Dans RecruitmentApplicant
pub async fn move_stage(
    &self,
    ctx: &OperatorContext,
    applicant_id: ApplicantId,
    new_stage_id: StageId,
) -> Result<(), RecruitmentError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "applicant.stage.move").await?;

    let intent = WriteIntent::update_applicant_stage(applicant_id, new_stage_id);
    kind_mother::submit(&mandate, intent).await?;

    // Optionnel : récupérer template du stage, envoyer email via MiyuNotify
    if let Some(template_id) = recruitment_stage::get_email_template(new_stage_id).await? {
        miyu_notify::send_from_template(applicant_id, template_id).await?;
    }
    Ok(())
}
```

### 2.3 Pattern : Refus (Décision StrongFather)

**Principe :** Le refus est une décision (StrongFather) avant mise à jour (KindMother).

```rust
// Dans RecruitmentApplicant
pub async fn refuse_applicant(
    &self,
    ctx: &OperatorContext,
    applicant_id: ApplicantId,
    reason_id: RefuseReasonId,
    send_email: bool,
) -> Result<(), RecruitmentError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "applicant.refuse").await?;

    let decision = strong_father::decide_refuse(applicant_id, reason_id).await?;
    if !decision.allowed {
        return Err(RecruitmentError::RefuseDenied);
    }

    let intent = WriteIntent::refuse_applicant(applicant_id, reason_id);
    kind_mother::submit(&mandate, intent).await?;

    if send_email {
        miyu_notify::send_refuse_template(applicant_id, reason_id).await?;
    }
    Ok(())
}
```

### 2.4 Pattern : Création employé (Décision StrongFather + KindMother)

**Principe :** La création d'employé depuis une candidature requiert une décision StrongFather puis un WriteIntent vers KindMother (et intégration MiyuHR).

```rust
// Dans RecruitmentApplicant (ou module dédié Hire)
pub async fn create_employee_from_applicant(
    &self,
    ctx: &OperatorContext,
    applicant_id: ApplicantId,
) -> Result<EmployeeId, RecruitmentError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "applicant.create_employee").await?;
    worry_sentinel::check_security_level(ctx.env(), 3).await?;

    let decision = strong_father::decide_hire_from_applicant(applicant_id).await?;
    if !decision.allowed {
        return Err(RecruitmentError::HireDenied);
    }

    let applicant = kind_mother::get_applicant(applicant_id).await?;
    let intent = WriteIntent::create_employee_from_applicant(applicant);
    let employee_id = kind_mother::submit(&mandate, intent).await?;

    // Liaison candidature → employé (WriteIntent léger)
    let link_intent = WriteIntent::link_applicant_to_employee(applicant_id, employee_id);
    kind_mother::submit(&mandate, link_intent).await?;

    Ok(employee_id)
}
```

---

## 3. Gestion des Erreurs et Rollback

- **Permission refusée (Master Butler)** : retourner une erreur explicite (capability manquante) ; pas de modification.
- **Décision refusée (StrongFather)** : retourner `RefuseDenied` / `HireDenied` ; pas de WriteIntent.
- **WriteIntent rejeté (KindMother)** : rollback transactionnel si supporté ; log et traçabilité (Maintenance explicable).
- **WorrySentinel (niveau confiance dégradé)** : refuser les opérations sensibles (création employé, accès salaires) selon politique ; retourner erreur « environnement dégradé ».

---

## 4. Intégration avec Kits existants

- **MiyuHR** : Création d'employé (WriteIntent délégué ou appel opérateur HR).
- **MiyuNotify** : Envoi emails (templates, refus, convocations, enquêtes).
- **MiyuDocuments / MiyuMedia** : Stockage et affichage CV (politique confidentialité).
- **MiyuWeb** : Publication postes, formulaire candidature (création candidature gouvernée).
- **MiyuPolls** (ou équivalent) : Enquêtes d'entretien ; envoi lien et liaison candidature.

---

**Document** : Odoo Recruitment — Guide Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01
