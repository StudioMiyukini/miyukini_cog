# Odoo Project — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Project dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust
- Gestion des gouvernances

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
ProjectUI → BondingBrother → ProjectOperator → StrongFather (décision)
                                              → KindMother (WriteIntent)
                                              → Master Butler (permissions)
                                              → WorrySentinel (sécurité)
```

### 1.2 Flux Typique

1. **Intention utilisateur** → ProjectUI
2. **Traduction intention** → BondingBrother
3. **Demande décision** → StrongFather
4. **Vérification permissions** → Master Butler
5. **Vérification sécurité** → WorrySentinel
6. **Persistance** → KindMother (WriteIntent)
7. **Notification** → MiyuNotify

---

## 2. Patterns d'Intégration

### 2.1 Création Projet

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreateProjectIntent {
    pub name: String,
    pub partner_id: Option<Uuid>,
    pub date_start: Option<Date>,
    pub date_end: Option<Date>,
    pub privacy_visibility: PrivacyVisibility,
}

impl ProjectOperator {
    pub async fn create_project(
        &self,
        intent: CreateProjectIntent,
        mandate: Mandate,
    ) -> Result<Project, ProjectError> {
        // 1. Vérification Mandate
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["project.create"])?;
        
        // 2. Demande décision StrongFather
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_project",
                context: &intent,
            })
            .await?;
        
        if !decision.allowed {
            return Err(ProjectError::DecisionDenied);
        }
        
        // 3. Vérification permissions Master Butler
        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "project.create",
                resource: None,
            })
            .await?;
        
        if !permission.granted {
            return Err(ProjectError::PermissionDenied);
        }
        
        // 4. Vérification sécurité WorrySentinel
        let security_level = self.worry_sentinel
            .get_security_level(&intent)
            .await?;
        
        if security_level > mandate.max_security_level {
            return Err(ProjectError::SecurityLevelExceeded);
        }
        
        // 5. Création WriteIntent
        let write_intent = WriteIntent {
            entity_type: "project.project",
            operation: WriteOperation::Create,
            data: ProjectData {
                name: intent.name,
                partner_id: intent.partner_id,
                date_start: intent.date_start,
                date: intent.date_end,
                privacy_visibility: intent.privacy_visibility,
                company_id: self.get_company_id().await?,
            },
            security_level,
        };
        
        // 6. Persistance via KindMother
        let project = self.kind_mother
            .persist(write_intent)
            .await?;
        
        // 7. Création compte analytique (si applicable)
        if let Some(account_id) = self.create_analytic_account(&project).await? {
            let update_intent = WriteIntent {
                entity_type: "project.project",
                operation: WriteOperation::Update,
                data: ProjectData {
                    account_id: Some(account_id),
                    ..project.data
                },
                security_level,
            };
            
            self.kind_mother.persist(update_intent).await?;
        }
        
        Ok(project)
    }
}
```

### 2.2 Création Tâche

**Pattern :** WriteIntent + Mandate + Notification

```rust
pub struct CreateTaskIntent {
    pub name: String,
    pub project_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub user_ids: Vec<Uuid>,
    pub date_deadline: Option<DateTime>,
    pub description: Option<String>,
}

impl TaskOperator {
    pub async fn create_task(
        &self,
        intent: CreateTaskIntent,
        mandate: Mandate,
    ) -> Result<Task, TaskError> {
        // 1. Vérification Mandate
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["task.create"])?;
        
        // 2. Demande décision StrongFather
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_task",
                context: &intent,
            })
            .await?;
        
        if !decision.allowed {
            return Err(TaskError::DecisionDenied);
        }
        
        // 3. Vérification projet (si fourni)
        let project = if let Some(project_id) = intent.project_id {
            Some(self.project_operator.get_project(project_id, &mandate).await?)
        } else {
            None
        };
        
        // 4. Calcul état initial (selon dépendances)
        let state = if intent.depend_on_ids.is_empty() {
            TaskState::InProgress
        } else {
            TaskState::Waiting
        };
        
        // 5. Création WriteIntent
        let write_intent = WriteIntent {
            entity_type: "project.task",
            operation: WriteOperation::Create,
            data: TaskData {
                name: intent.name,
                project_id: intent.project_id,
                parent_id: intent.parent_id,
                user_ids: intent.user_ids.clone(),
                date_deadline: intent.date_deadline,
                description: intent.description,
                state,
                stage_id: self.get_default_stage(&project).await?,
            },
            security_level: 2, // Sensitive
        };
        
        // 6. Persistance via KindMother
        let task = self.kind_mother
            .persist(write_intent)
            .await?;
        
        // 7. Notification assignés (si assignation)
        if !intent.user_ids.is_empty() {
            let notify_intent = NotifyIntent {
                recipients: intent.user_ids,
                subject: format!("Task assigned: {}", task.name),
                body: format!("You have been assigned to task: {}", task.name),
                template: Some("task_assigned"),
            };
            
            self.miyu_notify.notify(notify_intent).await?;
        }
        
        Ok(task)
    }
}
```

### 2.3 Assignation Tâche

**Pattern :** WriteIntent + Notification

```rust
impl TaskOperator {
    pub async fn assign_task(
        &self,
        task_id: Uuid,
        user_ids: Vec<Uuid>,
        mandate: Mandate,
    ) -> Result<Task, TaskError> {
        // 1. Vérification Mandate
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["task.assign"])?;
        
        // 2. Demande décision StrongFather
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "assign_task",
                context: &AssignContext { task_id, user_ids: user_ids.clone() },
            })
            .await?;
        
        if !decision.allowed {
            return Err(TaskError::DecisionDenied);
        }
        
        // 3. Récupération tâche
        let task = self.get_task(task_id, &mandate).await?;
        
        // 4. Calcul date assignation
        let date_assign = self.miyu_clock.now().await?;
        
        // 5. Création WriteIntent
        let write_intent = WriteIntent {
            entity_type: "project.task",
            operation: WriteOperation::Update,
            data: TaskData {
                user_ids: user_ids.clone(),
                date_assign: Some(date_assign),
                ..task.data
            },
            security_level: 2,
        };
        
        // 6. Persistance via KindMother
        let updated_task = self.kind_mother
            .persist(write_intent)
            .await?;
        
        // 7. Notification nouveaux assignés
        let new_assignees: Vec<Uuid> = user_ids
            .iter()
            .filter(|uid| !task.user_ids.contains(uid))
            .cloned()
            .collect();
        
        if !new_assignees.is_empty() {
            let notify_intent = NotifyIntent {
                recipients: new_assignees,
                subject: format!("Task assigned: {}", updated_task.name),
                body: format!("You have been assigned to task: {}", updated_task.name),
                template: Some("task_assigned"),
            };
            
            self.miyu_notify.notify(notify_intent).await?;
        }
        
        Ok(updated_task)
    }
}
```

### 2.4 Fermeture Tâche

**Pattern :** WriteIntent + Gestion Dépendances + Récurrence

```rust
impl TaskOperator {
    pub async fn close_task(
        &self,
        task_id: Uuid,
        state: TaskState, // Done ou Canceled
        mandate: Mandate,
    ) -> Result<Task, TaskError> {
        // 1. Vérification Mandate
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["task.close"])?;
        
        // 2. Demande décision StrongFather
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "close_task",
                context: &CloseContext { task_id, state },
            })
            .await?;
        
        if !decision.allowed {
            return Err(TaskError::DecisionDenied);
        }
        
        // 3. Récupération tâche
        let task = self.get_task(task_id, &mandate).await?;
        
        // 4. Calcul date fin
        let date_end = self.miyu_clock.now().await?;
        
        // 5. Création WriteIntent
        let write_intent = WriteIntent {
            entity_type: "project.task",
            operation: WriteOperation::Update,
            data: TaskData {
                state,
                date_end: Some(date_end),
                date_last_stage_update: Some(date_end),
                ..task.data
            },
            security_level: 2,
        };
        
        // 6. Persistance via KindMother
        let closed_task = self.kind_mother
            .persist(write_intent)
            .await?;
        
        // 7. Mise à jour tâches dépendantes (si dépendances activées)
        if task.allow_task_dependencies {
            let dependent_tasks = self.get_dependent_tasks(task_id, &mandate).await?;
            
            for dependent_task in dependent_tasks {
                // Vérifier si toutes dépendances fermées
                let all_dependencies_closed = self
                    .check_all_dependencies_closed(dependent_task.id, &mandate)
                    .await?;
                
                if all_dependencies_closed {
                    // Passer en InProgress
                    let update_intent = WriteIntent {
                        entity_type: "project.task",
                        operation: WriteOperation::Update,
                        data: TaskData {
                            state: TaskState::InProgress,
                            ..dependent_task.data
                        },
                        security_level: 2,
                    };
                    
                    self.kind_mother.persist(update_intent).await?;
                }
            }
        }
        
        // 8. Gestion récurrence (si dernière tâche récurrence)
        if task.recurring_task {
            if self.is_last_task_in_recurrence(task_id, &mandate).await? {
                self.create_next_recurrence_task(&task, &mandate).await?;
            }
        }
        
        Ok(closed_task)
    }
}
```

### 2.5 Partage Projet

**Pattern :** WriteIntent + Portal Access + Notification

```rust
impl ProjectOperator {
    pub async fn share_project(
        &self,
        project_id: Uuid,
        collaborator_ids: Vec<Uuid>,
        limited_access: bool,
        mandate: Mandate,
    ) -> Result<Project, ProjectError> {
        // 1. Vérification Mandate
        mandate.validate_operators(&[self.id(), self.collaborator_operator.id()])?;
        mandate.validate_flows(&["project.share"])?;
        
        // 2. Demande décision StrongFather
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "share_project",
                context: &ShareContext { project_id, collaborator_ids: collaborator_ids.clone() },
            })
            .await?;
        
        if !decision.allowed {
            return Err(ProjectError::DecisionDenied);
        }
        
        // 3. Récupération projet
        let project = self.get_project(project_id, &mandate).await?;
        
        // 4. Vérification visibilité projet
        if !project.privacy_visibility.allows_portal() {
            return Err(ProjectError::PrivacyVisibilityNotCompatible);
        }
        
        // 5. Ajout collaborateurs via ProjectCollaboratorOperator
        for collaborator_id in collaborator_ids {
            let collaborator_intent = CreateCollaboratorIntent {
                project_id,
                partner_id: collaborator_id,
                limited_access,
            };
            
            self.collaborator_operator
                .create_collaborator(collaborator_intent, &mandate)
                .await?;
            
            // 6. Génération token accès portail
            let access_token = self.miyu_portal
                .generate_access_token(collaborator_id, project_id)
                .await?;
            
            // 7. Envoi invitation
            let notify_intent = NotifyIntent {
                recipients: vec![collaborator_id],
                subject: format!("Project shared: {}", project.name),
                body: format!("You have been invited to collaborate on project: {}", project.name),
                template: Some("project_shared"),
                data: Some(json!({
                    "access_token": access_token,
                    "project_url": format!("/my/projects/{}", project_id),
                })),
            };
            
            self.miyu_notify.notify(notify_intent).await?;
        }
        
        Ok(project)
    }
}
```

---

## 3. Gestion des WriteIntents

### 3.1 Structure WriteIntent

```rust
pub struct WriteIntent {
    pub entity_type: &'static str, // "project.project", "project.task", etc.
    pub operation: WriteOperation,
    pub data: EntityData,
    pub security_level: SecurityLevel,
    pub metadata: Option<WriteMetadata>,
}

pub enum WriteOperation {
    Create,
    Update,
    Delete,
}

pub struct WriteMetadata {
    pub operator_id: Uuid,
    pub mandate_id: Uuid,
    pub timestamp: DateTime,
    pub reason: Option<String>,
}
```

### 3.2 Validation WriteIntent

```rust
impl KindMother {
    pub async fn validate_write_intent(
        &self,
        intent: &WriteIntent,
    ) -> Result<(), WriteIntentError> {
        // 1. Vérification sécurité WorrySentinel
        let allowed = self.worry_sentinel
            .check_write_allowed(intent)
            .await?;
        
        if !allowed {
            return Err(WriteIntentError::SecurityDenied);
        }
        
        // 2. Vérification cohérence données
        self.validate_data_coherence(&intent.data).await?;
        
        // 3. Vérification contraintes
        self.validate_constraints(&intent.data).await?;
        
        Ok(())
    }
}
```

---

## 4. Gestion des Mandates

### 4.1 Émission Mandate

```rust
impl StrongFather {
    pub async fn emit_mandate(
        &self,
        request: MandateRequest,
    ) -> Result<Mandate, MandateError> {
        // 1. Validation requête
        request.validate()?;
        
        // 2. Vérification règles métier
        let rules_check = self.check_business_rules(&request).await?;
        
        if !rules_check.allowed {
            return Err(MandateError::RulesViolation);
        }
        
        // 3. Création Mandate
        let mandate = Mandate {
            id: Uuid::new_v4(),
            operators: request.operators,
            flows: request.flows,
            max_security_level: request.max_security_level,
            valid_until: request.valid_until,
            conditions: request.conditions,
            issued_at: self.miyu_clock.now().await?,
        };
        
        // 4. Persistance Mandate (via KindMother)
        let write_intent = WriteIntent {
            entity_type: "mandate",
            operation: WriteOperation::Create,
            data: mandate.clone().into(),
            security_level: 3, // Critical
        };
        
        self.kind_mother.persist(write_intent).await?;
        
        Ok(mandate)
    }
}
```

### 4.2 Validation Mandate

```rust
impl Mandate {
    pub fn validate_operators(&self, operators: &[Uuid]) -> Result<(), MandateError> {
        for operator in operators {
            if !self.operators.contains(operator) {
                return Err(MandateError::OperatorNotAuthorized);
            }
        }
        Ok(())
    }
    
    pub fn validate_flows(&self, flows: &[&str]) -> Result<(), MandateError> {
        for flow in flows {
            if !self.flows.contains(&flow.to_string()) {
                return Err(MandateError::FlowNotAuthorized);
            }
        }
        Ok(())
    }
    
    pub fn is_valid(&self, now: DateTime) -> bool {
        // Vérification expiration
        if let Some(valid_until) = self.valid_until {
            if now > valid_until {
                return false;
            }
        }
        
        // Vérification conditions
        for condition in &self.conditions {
            if !condition.check() {
                return false;
            }
        }
        
        true
    }
}
```

---

## 5. Recommandations

### 5.1 Patterns à Suivre

- **Toujours valider Mandate** avant action
- **Toujours demander décision StrongFather** pour actions importantes
- **Toujours utiliser WriteIntent** pour persistance
- **Toujours vérifier permissions Master Butler**
- **Toujours vérifier sécurité WorrySentinel**

### 5.2 Gestion Erreurs

- Erreurs de décision → Retourner erreur claire
- Erreurs de permission → Logger et notifier
- Erreurs de sécurité → Logger et alerter WorrySentinel
- Erreurs de persistance → Rollback et notifier

### 5.3 Performance

- Cache des décisions StrongFather (si applicable)
- Batch WriteIntents pour opérations multiples
- Async/await pour opérations I/O
- Pagination pour listes importantes

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
