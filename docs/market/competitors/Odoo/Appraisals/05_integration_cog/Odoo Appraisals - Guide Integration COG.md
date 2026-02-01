# Odoo Appraisals — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Appraisals (Évaluations) dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Appraisals
- Patterns WriteIntent et Mandates (création, confirmation, feedback, rating, clôture, goals, 360)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, Master Butler, WorrySentinel, TAMR)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
AppraisalUI → BondingBrother → AppraisalOperator ──► StrongFather (décision create/confirm/done)
                            AppraisalGoalsOperator   KindMother (WriteIntent)
                            Appraisal360Operator      Master Butler (permissions)
                            AppraisalPlanOperator     WorrySentinel (Private Note, rating)
                            AppraisalTemplateOperator TAMR (réunion, décision finale)
                                                      Ever Buddy (cycle de vie)
```

### 1.2 Flux typiques

1. **Création appraisal** : Intention utilisateur → AppraisalUI → BondingBrother → AppraisalOperator → StrongFather (décision) → KindMother (WriteIntent Create).
2. **Confirmation** : AppraisalOperator.confirm → StrongFather (décision) → KindMother (WriteIntent Update statut) → MiyuNotify (notification employé).
3. **Feedback / rating** : AppraisalOperator.update_feedback / rate → Master Butler (permission manager ou self) → KindMother (WriteIntent Update).
4. **Clôture** : AppraisalOperator.done → StrongFather (décision) → KindMother (WriteIntent Update statut + next_appraisal_date via EmployeeOperator) + EmployeeSkillsOperator (skills depuis appraisal).
5. **Goal** : AppraisalGoalsOperator.create / update / done → Master Butler → KindMother (WriteIntent).
6. **360 request** : Appraisal360Operator.request → StrongFather (décision) → MiyuNotify (email Ask Feedback).

---

## 2. Patterns d'Intégration

### 2.1 Création d'appraisal

**Pattern :** WriteIntent Create + Mandate

```rust
// Pseudo-code Rust
pub struct CreateAppraisalIntent {
    pub employee_id: EmployeeId,
    pub appraisal_date: Date,
    pub template_id: TemplateId,
    pub company_id: CompanyId,
}

impl AppraisalOperator {
    pub async fn create_appraisal(
        &self,
        intent: CreateAppraisalIntent,
        mandate: Mandate,
    ) -> Result<Appraisal, AppraisalError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["appraisal.create"])?;

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "appraisal.create",
                resource: Some(Resource::Employee(intent.employee_id)),
            })
            .await?;
        if !permission.granted {
            return Err(AppraisalError::PermissionDenied);
        }

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "appraisal.create",
                context: DecisionContext::Appraisal(intent.clone()),
            })
            .await?;
        if !decision.allowed {
            return Err(AppraisalError::DecisionDenied);
        }

        let write_intent = WriteIntent::Create(AppraisalCreate {
            employee_id: intent.employee_id,
            appraisal_date: intent.appraisal_date,
            template_id: intent.template_id,
            company_id: intent.company_id,
            state: AppraisalState::Draft,
        });
        let appraisal = self.kind_mother.persist(write_intent, mandate).await?;
        Ok(appraisal)
    }
}
```

### 2.2 Confirmation d'appraisal

**Pattern :** WriteIntent Update + Notification

```rust
pub async fn confirm_appraisal(
    &self,
    appraisal_id: AppraisalId,
    mandate: Mandate,
) -> Result<Appraisal, AppraisalError> {
    mandate.validate_flows(&["appraisal.confirm"])?;
    self.master_butler.check_permission(/* appraisal.confirm */).await?;
    self.strong_father.decide(/* appraisal.confirm */).await?;

    let appraisal = self.kind_mother.get::<Appraisal>(appraisal_id).await?;
    if appraisal.state != AppraisalState::Draft {
        return Err(AppraisalError::InvalidState);
    }

    let write_intent = WriteIntent::Update(AppraisalUpdate {
        id: appraisal_id,
        state: AppraisalState::Confirmed,
        confirmed_at: Some(Clock::now()),
    });
    let updated = self.kind_mother.persist(write_intent, mandate).await?;

    // Notification employé (MiyuNotify avec Mandat)
    self.miyu_notify.send(AppraisalConfirmedNotification {
        appraisal_id,
        employee_id: updated.employee_id,
        link: format!("/appraisals/{}", appraisal_id),
    }, mandate).await?;

    Ok(updated)
}
```

### 2.3 Note finale et note privée

**Pattern :** WriteIntent Update + WorrySentinel (niveau sécurité)

```rust
pub async fn set_final_rating(
    &self,
    appraisal_id: AppraisalId,
    rating: EvaluationScaleId,
    private_note: Option<String>,
    mandate: Mandate,
) -> Result<Appraisal, AppraisalError> {
    mandate.validate_flows(&["appraisal.rate"])?;
    self.master_butler.check_permission(/* appraisal.rate, manager */).await?;

    let security_level = self.worry_sentinel
        .get_security_level_for_private_note(&private_note)
        .await?;
    if security_level > mandate.max_security_level {
        return Err(AppraisalError::SecurityLevelExceeded);
    }

    self.strong_father.decide(/* appraisal.rate */).await?;

    let write_intent = WriteIntent::Update(AppraisalUpdate {
        id: appraisal_id,
        final_rating_id: Some(rating),
        private_note: private_note.clone(),
    });
    self.kind_mother.persist(write_intent, mandate).await
}
```

### 2.4 Clôture (Mark as Done)

**Pattern :** WriteIntent Update + mise à jour next_appraisal_date et skills

```rust
pub async fn mark_as_done(
    &self,
    appraisal_id: AppraisalId,
    mandate: Mandate,
) -> Result<Appraisal, AppraisalError> {
    mandate.validate_flows(&["appraisal.done"])?;
    self.master_butler.check_permission(/* appraisal.done, manager */).await?;
    self.strong_father.decide(/* appraisal.done */).await?;

    let appraisal = self.kind_mother.get::<Appraisal>(appraisal_id).await?;
    if appraisal.state != AppraisalState::Confirmed {
        return Err(AppraisalError::InvalidState);
    }

    let write_intent = WriteIntent::Update(AppraisalUpdate {
        id: appraisal_id,
        state: AppraisalState::Done,
        done_at: Some(Clock::now()),
    });
    let updated = self.kind_mother.persist(write_intent, mandate).await?;

    // Mise à jour next_appraisal_date (via EmployeeOperator / KindMother)
    let next_date = self.appraisal_plan_operator
        .compute_next_appraisal_date(updated.employee_id, updated.appraisal_date)
        .await?;
    self.employee_operator.update_next_appraisal_date(
        updated.employee_id,
        next_date,
        mandate,
    ).await?;

    // Report des skills modifiés dans l'appraisal vers la fiche employé (EmployeeSkillsOperator)
    self.employee_skills_operator.sync_skills_from_appraisal(appraisal_id, mandate).await?;

    Ok(updated)
}
```

### 2.5 Création d'un goal

**Pattern :** WriteIntent Create + Mandate

```rust
pub struct CreateGoalIntent {
    pub name: String,
    pub employee_id: EmployeeId,
    pub progress: GoalProgress,  // 0, 25, 50, 75, 100
    pub deadline: Date,
    pub tags: Vec<TagId>,
    pub description: Option<String>,
}

impl AppraisalGoalsOperator {
    pub async fn create_goal(
        &self,
        intent: CreateGoalIntent,
        mandate: Mandate,
    ) -> Result<Goal, AppraisalError> {
        mandate.validate_flows(&["goal.create"])?;
        self.master_butler.check_permission(/* goal.create, manager of employee */).await?;
        self.strong_father.decide(/* goal.create */).await?;

        let write_intent = WriteIntent::Create(GoalCreate {
            name: intent.name,
            employee_id: intent.employee_id,
            manager_id: /* résolu depuis employee */,
            progress: intent.progress,
            deadline: intent.deadline,
            tags: intent.tags,
            description: intent.description,
        });
        self.kind_mother.persist(write_intent, mandate).await
    }
}
```

### 2.6 Demande 360 Feedback

**Pattern :** StrongFather + MiyuNotify (pas d'écriture appraisal directe)

```rust
pub async fn request_360_feedback(
    &self,
    appraisal_id: AppraisalId,
    recipients: Vec<EmployeeId>,
    message: String,
    answer_deadline: Date,
    mandate: Mandate,
) -> Result<(), AppraisalError> {
    mandate.validate_flows(&["360.request"])?;
    self.master_butler.check_permission(/* 360.request, manager */).await?;
    self.strong_father.decide(/* 360.request */).await?;

    let appraisal = self.kind_mother.get::<Appraisal>(appraisal_id).await?;
    if appraisal.state != AppraisalState::Confirmed {
        return Err(AppraisalError::InvalidState);
    }

    self.miyu_notify.send_bulk(AskFeedbackNotification {
        appraisal_id,
        survey_id: appraisal.survey_360_id,
        recipients,
        message,
        answer_deadline,
    }, mandate).await
}
```

---

## 3. Gestion des Erreurs et Rollback

- **PermissionDenied** : Master Butler refuse → retour utilisateur « Droits insuffisants ».
- **DecisionDenied** : StrongFather refuse → retour utilisateur « Action non autorisée ».
- **SecurityLevelExceeded** : WorrySentinel (Private Note) → refus ou élévation Mandat.
- **InvalidState** : Transition interdite (ex. confirm sur déjà confirmed) → message métier clair.
- **Rollback** : KindMother gère la transaction ; en cas d’échec après persist, compenser (ex. annuler notification si possible ou log).

---

## 4. Intégration avec Kits existants

- **MiyuHR (EmployeeOperator, EmployeeSkillsOperator)** : Lecture fiche employé, manager, département ; mise à jour next_appraisal_date et skills après clôture (via Mandat et WriteIntent).
- **MiyuNotify** : Confirmation appraisal, Ask Feedback, invitation réunion (templates email, Mandat).
- **MiyuSurveys** (si utilisé) : Lecture templates appraisal ; enregistrement réponses 360 ; See Results.
- **MiyuPlanning / Calendar** : Création événement Meeting (entretien) ; participants et option vidéocall.

---

**Document** : Odoo Appraisals — Guide Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01
