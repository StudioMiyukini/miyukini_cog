# Odoo eLearning — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités eLearning (LMS) dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates (cours, contenu, inscription, certification)
- Exemples de code pseudo-Rust
- Gestion des gouvernances

---

## 1. Architecture d'intégration

### 1.1 Vue d'ensemble

```
eLearningUI → BondingBrother → eLearningCourseOperator / eLearningContentOperator / eLearningEnrollmentOperator / eLearningCertificationOperator / eLearningGroupOperator
                                    → StrongFather (décision)
                                    → KindMother (WriteIntent)
                                    → Master Butler (permissions)
                                    → WorrySentinel (sécurité)
                                    → MiyuSurveys (certification)
                                    → MiyuNotify (notifications, invitations)
                                    → MiyuStore (paiement Course)
```

### 1.2 Flux typique (création cours)

1. **Intention utilisateur** → eLearningUI (New course)
2. **Traduction intention** → BondingBrother
3. **Demande décision** → StrongFather
4. **Vérification permissions** → Master Butler
5. **Vérification sécurité** → WorrySentinel
6. **Persistance** → KindMother (WriteIntent)
7. **Retour** → eLearningUI (affichage cours créé)

### 1.3 Flux inscription (On Payment)

1. **Paiement validé** → MiyuStore notifie eLearningEnrollmentOperator
2. **Demande décision** → StrongFather (inscription autorisée pour ce cours après paiement)
3. **Persistance** → KindMother (WriteIntent Enrollment)
4. **Notification** → MiyuNotify (optionnel : email de bienvenue)
5. **Retour** → Utilisateur a accès au cours

---

## 2. Patterns d'intégration

### 2.1 Création de cours

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreateCourseIntent {
    pub title: String,
    pub tag_ids: Vec<TagId>,
    pub responsible_id: UserId,
    pub website_id: Option<WebsiteId>,
    pub show_course_to: ShowCourseTo,  // Everyone | SignedIn | CourseAttendees
    pub enroll_policy: EnrollPolicy,   // Open | OnInvitation | OnPayment
    pub display_mode: DisplayMode,    // Training | Documentation
}

impl eLearningCourseOperator {
    pub async fn create_course(
        &self,
        intent: CreateCourseIntent,
        mandate: Mandate,
    ) -> Result<Course, eLearningError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["course.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_course",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(eLearningError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "course.create",
                resource: None,
            })
            .await?;
        if !permission.granted {
            return Err(eLearningError::PermissionDenied);
        }

        let security_level = self.worry_sentinel.get_security_level(&intent).await?;
        if security_level < 2 {
            return Err(eLearningError::SecurityLevelTooLow);
        }

        let write_intent = CourseWriteIntent {
            action: WriteAction::Create,
            payload: CoursePayload::from_intent(intent),
        };
        let course = self.kind_mother.persist(write_intent, mandate).await?;
        Ok(course)
    }
}
```

### 2.2 Publication d'un cours

**Pattern :** WriteIntent (statut) + Mandate

```rust
pub struct PublishCourseIntent {
    pub course_id: CourseId,
    pub publish: bool,  // true = publish, false = unpublish
}

impl eLearningCourseOperator {
    pub async fn publish_course(
        &self,
        intent: PublishCourseIntent,
        mandate: Mandate,
    ) -> Result<Course, eLearningError> {
        mandate.validate_flows(&["course.publish"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "publish_course",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(eLearningError::DecisionDenied);
        }

        let write_intent = CourseWriteIntent {
            action: WriteAction::Update,
            payload: CoursePayload {
                id: intent.course_id,
                published: Some(intent.publish),
                ..Default::default()
            },
        };
        let course = self.kind_mother.persist(write_intent, mandate).await?;
        Ok(course)
    }
}
```

### 2.3 Inscription (On Invitation)

**Pattern :** Enrollment WriteIntent + MiyuNotify (envoi lien/email)

```rust
pub struct InviteEnrollmentIntent {
    pub course_id: CourseId,
    pub email: Option<String>,
    pub copy_link: bool,
    pub send_by_email: bool,
    pub enroll_message: String,
}

impl eLearningEnrollmentOperator {
    pub async fn invite_attendees(
        &self,
        intent: InviteEnrollmentIntent,
        mandate: Mandate,
    ) -> Result<InviteResult, eLearningError> {
        mandate.validate_flows(&["enrollment.invite"])?;

        let decision = self.strong_father.decide(/* ... */).await?;
        if !decision.allowed {
            return Err(eLearningError::DecisionDenied);
        }

        let invite_link = self.generate_invite_link(intent.course_id).await?;
        if intent.send_by_email {
            self.miyu_notify
                .send_template(
                    intent.email.unwrap(),
                    "elearning_invite",
                    context: { link: invite_link, message: intent.enroll_message },
                    mandate.clone(),
                )
                .await?;
        }
        Ok(InviteResult {
            link: if intent.copy_link { Some(invite_link) } else { None },
        })
    }
}
```

### 2.4 Inscription après paiement (On Payment)

**Pattern :** Événement MiyuStore → StrongFather + KindMother

```rust
// Côté eLearningEnrollmentOperator : écoute événement paiement validé
pub async fn on_payment_validated(
    &self,
    event: PaymentValidatedEvent,
    mandate: Mandate,
) -> Result<Enrollment, eLearningError> {
    if event.product_type != ProductType::Course {
        return Ok(()); // ignore
    }
    let course_id = self.resolve_course_from_product(event.product_id).await?;
    mandate.validate_flows(&["enrollment.create"])?;

    let decision = self.strong_father
        .decide(DecisionRequest {
            action: "enrollment_after_payment",
            context: &EnrollmentContext {
                user_id: event.buyer_id,
                course_id,
                payment_id: event.payment_id,
            },
        })
        .await?;
    if !decision.allowed {
        return Err(eLearningError::DecisionDenied);
    }

    let write_intent = EnrollmentWriteIntent {
        action: WriteAction::Create,
        payload: EnrollmentPayload {
            user_id: event.buyer_id,
            course_id,
            source: EnrollmentSource::Payment(event.payment_id),
        },
    };
    let enrollment = self.kind_mother.persist(write_intent, mandate).await?;
    Ok(enrollment)
}
```

### 2.5 Délivrance certification

**Pattern :** MiyuSurveys (résultat) → StrongFather + KindMother + eLearningEnrollmentOperator (karma)

```rust
pub struct DeliverCertificationIntent {
    pub course_id: CourseId,
    pub user_id: UserId,
    pub survey_result_id: SurveyResultId,
    pub passed: bool,
    pub attempts: u32,
}

impl eLearningCertificationOperator {
    pub async fn deliver_certification(
        &self,
        intent: DeliverCertificationIntent,
        mandate: Mandate,
    ) -> Result<Certification, eLearningError> {
        if !intent.passed {
            return Err(eLearningError::CertificationNotPassed);
        }
        mandate.validate_flows(&["certification.deliver"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "deliver_certification",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(eLearningError::DecisionDenied);
        }

        let write_intent = CertificationWriteIntent {
            action: WriteAction::Create,
            payload: CertificationPayload {
                user_id: intent.user_id,
                course_id: intent.course_id,
                survey_result_id: intent.survey_result_id,
                delivered_at: self.clock.now(),
            },
        };
        let certification = self.kind_mother.persist(write_intent, mandate).await?;

        // Attribuer karma selon tentatives (config cours/contenu)
        let karma_points = self.karma_for_attempts(intent.attempts).await?;
        self.enrollment_operator
            .grant_karma(intent.user_id, intent.course_id, KarmaReason::Certification, karma_points, mandate)
            .await?;

        Ok(certification)
    }
}
```

### 2.6 Vérification d'accès (Show course to)

**Pattern :** Master Butler + contexte utilisateur

```rust
impl eLearningEnrollmentOperator {
    pub async fn can_access_course(
        &self,
        user_id: Option<UserId>,
        course_id: CourseId,
        mandate: Mandate,
    ) -> Result<AccessLevel, eLearningError> {
        let course = self.course_operator.get(course_id, mandate.clone()).await?;
        match course.show_course_to {
            ShowCourseTo::Everyone => Ok(AccessLevel::Browse),
            ShowCourseTo::SignedIn => {
                let user_id = user_id.ok_or(eLearningError::NotSignedIn)?;
                Ok(AccessLevel::Browse)
            }
            ShowCourseTo::CourseAttendees => {
                let user_id = user_id.ok_or(eLearningError::NotSignedIn)?;
                let enrolled = self.is_enrolled(user_id, course_id).await?;
                if enrolled {
                    Ok(AccessLevel::Full)
                } else {
                    Err(eLearningError::NotEnrolled)
                }
            }
        }
    }
}
```

---

## 3. Récapitulatif des patterns

| Action | Pattern | Cores sollicités |
|--------|---------|------------------|
| Création cours | WriteIntent + Mandate | StrongFather, Master Butler, WorrySentinel, KindMother |
| Publication cours | WriteIntent (statut) | StrongFather, KindMother |
| Inscription (Open) | WriteIntent Enrollment | StrongFather, KindMother |
| Inscription (Invitation) | WriteIntent + MiyuNotify | StrongFather, KindMother, MiyuNotify |
| Inscription (Payment) | Événement MiyuStore → WriteIntent | StrongFather, KindMother, MiyuStore |
| Délivrance certification | WriteIntent + MiyuSurveys + karma | StrongFather, KindMother, eLearningEnrollmentOperator |
| Accès cours | Lecture + Master Butler | Master Butler, WorrySentinel |

---

**Document créé le :** 2026-02-01  
**Version :** 1.0
