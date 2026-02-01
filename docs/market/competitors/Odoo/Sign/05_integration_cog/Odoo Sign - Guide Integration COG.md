# Odoo Sign — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Sign dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Sign
- Patterns WriteIntent et Mandates (création demande, envoi, signature)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, WorrySentinel)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
SignUI → BondingBrother → SignRequestOperator → StrongFather (décision)
                                              → KindMother (WriteIntent)
                                              → Master Butler (permissions)
                                              → WorrySentinel (sécurité)
                                              → SignComplianceOperator (preuves)
```

### 1.2 Flux typiques

**Envoi d'une demande :**
1. Intention utilisateur (SignUI) → BondingBrother
2. Demande décision → StrongFather
3. Vérification permissions → Master Butler
4. Vérification sécurité → WorrySentinel
5. Persistance demande → KindMother (WriteIntent)
6. Envoi emails (liens signataires) → MiyuNotify
7. Planification relances et expiration (jobs)

**Signature (côté signataire) :**
1. Accès par token (page signataire) → validation token
2. Récupération demande + champs à remplir pour ce signataire → SignRequestOperator
3. Soumission des champs remplis → StrongFather (décision enregistrement) + KindMother (WriteIntent)
4. Génération hash et enregistrement preuve → SignComplianceOperator
5. Si dernier signataire : génération PDF signé, archivage → MiyuDocuments ; notification initiateur → MiyuNotify
6. Sinon : envoi au signataire suivant → MiyuNotify

---

## 2. Patterns d'Intégration

### 2.1 Création et envoi d'une demande

**Pattern :** WriteIntent (demande) + Mandate + MiyuNotify (envoi)

```rust
// Pseudo-code Rust
pub struct CreateAndSendRequestIntent {
    pub template_id: Uuid,
    pub signers: Vec<SignerAssignment>, // (role_id, partner_id, order?)
    pub valid_until: Option<Date>,
    pub reminder: bool,
    pub reminder_days: u32,
    pub subject: String,
    pub message: Option<String>,
}

impl SignRequestOperator {
    pub async fn create_and_send(
        &self,
        intent: CreateAndSendRequestIntent,
        mandate: Mandate,
    ) -> Result<SignRequest, SignError> {
        // 1. Vérification Mandate
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["sign_request.send"])?;

        // 2. Décision StrongFather
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "send_sign_request",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(SignError::DecisionDenied);
        }

        // 3. Permissions Master Butler
        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "sign_request.send",
                resource: Some(intent.template_id),
            })
            .await?;
        if !permission.granted {
            return Err(SignError::PermissionDenied);
        }

        // 4. Sécurité WorrySentinel
        let security_level = self.worry_sentinel
            .get_security_level(&intent)
            .await?;
        if security_level > mandate.max_security_level {
            return Err(SignError::SecurityLevelExceeded);
        }

        // 5. Vérification un signataire par rôle (règle métier)
        self.ensure_one_signer_per_role(&intent)?;

        // 6. WriteIntent demande (état shared puis sent)
        let write_intent = WriteIntent {
            entity_type: "sign.request",
            operation: WriteOperation::Create,
            data: SignRequestData {
                template_id: intent.template_id,
                signers: intent.signers.clone(),
                valid_until: intent.valid_until,
                reminder: intent.reminder,
                reminder_days: intent.reminder_days,
                state: SignRequestState::Sent,
                company_id: self.get_company_id().await?,
            },
            security_level,
        };
        let request = self.kind_mother.persist(write_intent).await?;

        // 7. Génération liens signataires (tokens)
        let signer_links = self.generate_signer_links(&request).await?;

        // 8. Envoi emails via MiyuNotify
        for (signer, link) in signer_links {
            self.miyu_notify
                .send(SendRequest {
                    recipient: signer.partner_id,
                    subject: intent.subject.clone(),
                    body: intent.message.clone().unwrap_or_default(),
                    template: Some("sign_request_invitation"),
                    link: Some(link),
                })
                .await?;
        }

        // 9. Planification relances et expiration (job)
        self.schedule_reminders_and_expiration(&request).await?;

        Ok(request)
    }
}
```

### 2.2 Enregistrement d'une signature (signataire)

**Pattern :** WriteIntent (champs remplis) + SignComplianceOperator (hash, preuve)

```rust
pub struct SubmitSignatureIntent {
    pub request_id: Uuid,
    pub token: String,           // token signataire
    pub filled_fields: Vec<FilledField>, // (item_id, value: Signature|Text|Checkbox|...)
}

impl SignRequestOperator {
    pub async fn submit_signature(
        &self,
        intent: SubmitSignatureIntent,
    ) -> Result<SignRequest, SignError> {
        // 1. Validation token (pas de Mandate côté signataire ; token = preuve d'autorisation)
        let signer_context = self.validate_signer_token(&intent.token).await?;
        if signer_context.request_id != intent.request_id {
            return Err(SignError::InvalidToken);
        }

        // 2. Décision StrongFather (enregistrement signature)
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "record_signature",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(SignError::DecisionDenied);
        }

        // 3. WriteIntent : mise à jour demande (champs remplis pour ce signataire)
        let write_intent = WriteIntent {
            entity_type: "sign.request",
            operation: WriteOperation::Update,
            data: SignRequestData {
                id: intent.request_id,
                filled_fields: intent.filled_fields,
                signer_completed: Some(signer_context.role_id),
                security_level: 3,
            },
            security_level: 3,
        };
        let request = self.kind_mother.persist(write_intent).await?;

        // 4. SignComplianceOperator : hash et preuve
        let proof = self.sign_compliance
            .record_signature_evidence(RecordEvidenceRequest {
                request_id: intent.request_id,
                signer_role_id: signer_context.role_id,
                filled_fields: intent.filled_fields.clone(),
                ip: intent.client_ip,
                timestamp: self.miyu_clock.now().await?,
            })
            .await?;

        // 5. Si dernier signataire : signed, PDF final, archivage, notification
        if request.all_signers_completed() {
            let signed_pdf = self.generate_signed_pdf(&request, &proof).await?;
            self.miyu_documents
                .archive(ArchiveRequest {
                    file: signed_pdf,
                    workspace_id: request.template.signed_document_workspace_id,
                    tag_ids: request.template.signed_document_tag_ids.clone(),
                })
                .await?;

            let update_signed = WriteIntent {
                entity_type: "sign.request",
                operation: WriteOperation::Update,
                data: SignRequestData {
                    id: request.id,
                    state: SignRequestState::Signed,
                    security_level: 3,
                },
                security_level: 3,
            };
            self.kind_mother.persist(update_signed).await?;

            self.miyu_notify
                .notify_initiator(&request, "sign_request_signed")
                .await?;
        } else {
            // Envoi au signataire suivant
            let next_signer_link = self.get_signer_link(&request, request.next_signer()).await?;
            self.miyu_notify
                .send_to_partner(request.next_signer().partner_id, next_signer_link)
                .await?;
        }

        Ok(request)
    }
}
```

### 2.3 Annulation d'une demande

**Pattern :** StrongFather (décision) + KindMother (WriteIntent état) + invalidation tokens

```rust
impl SignRequestOperator {
    pub async fn cancel_request(
        &self,
        request_id: Uuid,
        mandate: Mandate,
    ) -> Result<(), SignError> {
        mandate.validate_flows(&["sign_request.cancel"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "cancel_sign_request",
                context: &CancelContext { request_id },
            })
            .await?;
        if !decision.allowed {
            return Err(SignError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "sign.request",
            operation: WriteOperation::Update,
            data: SignRequestData {
                id: request_id,
                state: SignRequestState::Canceled,
                security_level: 3,
            },
            security_level: 3,
        };
        self.kind_mother.persist(write_intent).await?;

        self.invalidate_signer_tokens(request_id).await?;
        Ok(())
    }
}
```

---

## 3. Gestion des Gouvernances

### 3.1 WorrySentinel — Niveau de sécurité

- **Demandes et documents signés** : niveau 3 (Critical) — données sensibles, conformité juridique.
- **Templates** : niveau 2–3 selon contenu.
- **Page signataire** : accès par token uniquement ; pas d’élévation de privilège ; audit des accès (IP, date).
- **Export preuves** : réservé aux utilisateurs avec permission audit ; WorrySentinel valide le niveau.

### 3.2 KindMother — WriteIntent

- Toute création/modification de demande, template, item, rôle, et enregistrement de preuve passe par **WriteIntent**.
- Pas d’écriture directe sur les entités Sign en dehors du flux COG.
- Document signé (PDF final) : généré côté serveur puis archivé via MiyuDocuments (WriteIntent côté Documents si applicable).

### 3.3 StrongFather — Décisions

- **Envoi** : décision « send_sign_request » avec contexte (template, signataires, options).
- **Signature** : décision « record_signature » avec contexte (request_id, signataire, champs).
- **Annulation** : décision « cancel_sign_request » avec request_id.
- **Refus signataire** : décision « refuse_sign_request » avec request_id et signataire.

### 3.4 SignComplianceOperator — Preuves

- **Ne jamais modifier** le contenu signé après enregistrement.
- Génération hash et enregistrement des métadonnées (IP, date, type d’auth) en **WriteIntent limité** (logs de preuve).
- Export preuve : lecture + assemblage document + métadonnées ; Mandat avec permission audit.

---

## 4. Résumé des patterns

| Cas | Pattern |
|-----|--------|
| Création + envoi demande | WriteIntent (Create demande) + Mandate + MiyuNotify (liens) + jobs (relances, expiration) |
| Signature signataire | Validation token + WriteIntent (Update champs) + SignComplianceOperator (hash, preuve) + MiyuDocuments (archivage si dernier) + MiyuNotify |
| Annulation | StrongFather (décision) + WriteIntent (état canceled) + invalidation tokens |
| Création template | WriteIntent (Create template) + StrongFather + Master Butler (authorized users) |
| Export preuve | Mandate (permission audit) + SignComplianceOperator (lecture + export) + WorrySentinel |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
