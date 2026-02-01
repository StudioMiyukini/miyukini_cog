# Odoo Documents — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Documents (DMS) dans l’architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d’intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (dossier, fichier, partage, demande)
- Gestion des gouvernances

---

## 1. Architecture d’intégration

### 1.1 Vue d’ensemble

```
DocumentsUI → BondingBrother → DocumentsFolderOperator / DocumentsFileOperator / DocumentsShareOperator / DocumentsRequestOperator
                                    → StrongFather (décision)
                                    → KindMother (WriteIntent)
                                    → Master Butler (permissions)
                                    → WorrySentinel (sécurité)
```

### 1.2 Flux typique (création dossier)

1. **Intention utilisateur** → DocumentsUI (New ‣ Folder)
2. **Traduction intention** → BondingBrother
3. **Demande décision** → StrongFather
4. **Vérification permissions** → Master Butler
5. **Vérification sécurité** → WorrySentinel
6. **Persistance** → KindMother (WriteIntent)
7. **Retour** → DocumentsUI (affichage nouvel élément)

---

## 2. Patterns d’intégration

### 2.1 Création de dossier

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreateFolderIntent {
    pub name: String,
    pub parent_id: Option<FolderId>,
    pub section: DocumentSection, // Company | MyDrive
    pub owner_id: UserId,
}

impl DocumentsFolderOperator {
    pub async fn create_folder(
        &self,
        intent: CreateFolderIntent,
        mandate: Mandate,
    ) -> Result<Folder, DocumentsError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["folder.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_folder",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(DocumentsError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "folder.create",
                resource: None,
            })
            .await?;
        if !permission.granted {
            return Err(DocumentsError::PermissionDenied);
        }

        let security_level = self.worry_sentinel.get_security_level(&intent).await?;
        if security_level > mandate.max_security_level {
            return Err(DocumentsError::SecurityLevelExceeded);
        }

        let write_intent = WriteIntent {
            entity_type: "documents.folder",
            operation: WriteOperation::Create,
            data: FolderData {
                name: intent.name,
                parent_id: intent.parent_id,
                section: intent.section,
                owner_id: intent.owner_id,
                company_id: self.get_company_id().await?,
            },
            security_level,
        };

        let folder = self.kind_mother.persist(write_intent).await?;
        Ok(folder)
    }
}
```

### 2.2 Upload de fichier

**Pattern :** WriteIntent + Mandate (avec blob ou référence stockage)

```rust
pub struct UploadFileIntent {
    pub name: String,
    pub folder_id: FolderId,
    pub content_type: String,
    pub size_bytes: u64,
    pub owner_id: UserId,
    // blob ou stream géré par MiyuMedia / storage
}

impl DocumentsFileOperator {
    pub async fn upload_file(
        &self,
        intent: UploadFileIntent,
        mandate: Mandate,
    ) -> Result<Document, DocumentsError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["file.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "upload_file",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(DocumentsError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "file.create",
                resource: Some(ResourceRef::Folder(intent.folder_id)),
            })
            .await?;
        if !permission.granted {
            return Err(DocumentsError::PermissionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "documents.document",
            operation: WriteOperation::Create,
            data: DocumentData {
                name: intent.name,
                folder_id: intent.folder_id,
                document_type: DocumentType::Uploaded,
                owner_id: intent.owner_id,
                content_type: intent.content_type,
                size_bytes: intent.size_bytes,
            },
            security_level: self.worry_sentinel.get_security_level(&intent).await?,
        };

        let document = self.kind_mother.persist(write_intent).await?;
        // Stockage blob délégué à MiyuMedia ou adaptateur storage
        self.store_blob(document.id, &intent).await?;
        Ok(document)
    }
}
```

### 2.3 Partager un dossier

**Pattern :** WriteIntent (règles d’accès) + Mandate

```rust
pub struct ShareFolderIntent {
    pub folder_id: FolderId,
    pub grants: Vec<ShareGrant>,
    pub general_access: Option<GeneralAccess>, // InternalUsers | AnyoneWithLink
    pub discoverable: bool,
}

pub struct ShareGrant {
    pub principal_id: Uuid, // UserId ou ContactId selon principal_type
    pub role: ShareRole,    // Viewer | Editor
    pub expires_at: Option<DateTime>,
}

impl DocumentsShareOperator {
    pub async fn share_folder(
        &self,
        intent: ShareFolderIntent,
        mandate: Mandate,
    ) -> Result<(), DocumentsError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["share.folder"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "share_folder",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(DocumentsError::DecisionDenied);
        }

        for grant in &intent.grants {
            let write_intent = WriteIntent {
                entity_type: "documents.folder_access",
                operation: WriteOperation::Create,
                data: FolderAccessData {
                    folder_id: intent.folder_id,
                    principal_id: grant.principal_id,
                    role: grant.role,
                    expires_at: grant.expires_at,
                },
                security_level: 2,
            };
            self.kind_mother.persist(write_intent).await?;
        }

        if let Some(general) = intent.general_access {
            let write_intent = WriteIntent {
                entity_type: "documents.folder_access",
                operation: WriteOperation::Create,
                data: FolderAccessData {
                    folder_id: intent.folder_id,
                    general_access: general,
                    discoverable: intent.discoverable,
                },
                security_level: 2,
            };
            self.kind_mother.persist(write_intent).await?;
        }

        Ok(())
    }
}
```

### 2.4 Demande de document

**Pattern :** WriteIntent (placeholder + activité) + Mandate

```rust
pub struct CreateDocumentRequestIntent {
    pub document_name: String,
    pub request_to_id: UserId,
    pub due_date: Option<Date>,
    pub folder_id: FolderId,
    pub tag_ids: Vec<TagId>,
    pub message: Option<String>,
}

impl DocumentsRequestOperator {
    pub async fn create_request(
        &self,
        intent: CreateDocumentRequestIntent,
        mandate: Mandate,
    ) -> Result<DocumentRequest, DocumentsError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["request.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_document_request",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(DocumentsError::DecisionDenied);
        }

        // Création placeholder (document de type Request)
        let placeholder_intent = WriteIntent {
            entity_type: "documents.document",
            operation: WriteOperation::Create,
            data: DocumentData {
                name: intent.document_name,
                folder_id: intent.folder_id,
                document_type: DocumentType::RequestPlaceholder,
                request_to_id: Some(intent.request_to_id),
                due_date: intent.due_date,
                tag_ids: intent.tag_ids,
            },
            security_level: 2,
        };
        let placeholder = self.kind_mother.persist(placeholder_intent).await?;

        // Création activité via MiyuNotify
        self.miyu_notify
            .schedule_activity(ActivityIntent {
                model: "documents.document",
                res_id: placeholder.id,
                activity_type: "document_request",
                user_id: intent.request_to_id,
                due_date: intent.due_date,
                summary: intent.message,
            })
            .await?;

        Ok(DocumentRequest { placeholder_id: placeholder.id, .. })
    }
}
```

### 2.5 Déplacement vers Trash (délai de suppression)

**Pattern :** WriteIntent (changement d’état) + Ever Buddy (cycle de vie)

```rust
impl DocumentsFolderOperator {
    pub async fn move_to_trash(
        &self,
        folder_id: FolderId,
        mandate: Mandate,
    ) -> Result<(), DocumentsError> {
        mandate.validate_flows(&["folder.delete"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "move_folder_to_trash",
                context: &folder_id,
            })
            .await?;
        if !decision.allowed {
            return Err(DocumentsError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "documents.folder",
            operation: WriteOperation::Update,
            data: FolderData {
                id: folder_id,
                section: DocumentSection::Trash,
                deleted_at: Some(Utc::now()),
            },
            security_level: 2,
        };
        self.kind_mother.persist(write_intent).await?;

        // Ever Buddy : planification purge après délai (config)
        let delay_days = self.get_deletion_delay_days().await?;
        self.ever_buddy
            .schedule_retirement(RetirementIntent {
                entity_type: "documents.folder",
                entity_id: folder_id,
                at: Utc::now() + Duration::days(delay_days as i64),
            })
            .await?;

        Ok(())
    }
}
```

---

## 3. Vérification d’accès (partage)

**Pattern :** Master Butler + règles DocumentsShareOperator

```rust
impl DocumentsShareOperator {
    pub async fn check_access(
        &self,
        resource: ResourceRef, // Folder(id) | File(id)
        user_id: UserId,
        required_role: ShareRole, // Viewer | Editor
    ) -> Result<bool, DocumentsError> {
        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: match required_role {
                    ShareRole::Viewer => "documents.view",
                    ShareRole::Editor => "documents.edit",
                },
                resource: Some(resource),
                principal: user_id,
            })
            .await?;

        if !permission.granted {
            return Ok(false);
        }

        // Vérification expiration des grants
        let access = self.kind_mother
            .get_access_rules(resource).await?;
        let valid = access.iter().any(|r| {
            r.principal_id == user_id && r.role >= required_role && r.is_valid_now()
        });
        Ok(valid)
    }
}
```

---

## 4. Résumé des flux COG

| Action | StrongFather | KindMother | Master Butler | WorrySentinel | Ever Buddy |
|--------|--------------|------------|---------------|---------------|------------|
| Créer dossier | Décision | WriteIntent folder | Permission folder.create | Niveau sécurité | — |
| Upload fichier | Décision | WriteIntent document + blob | Permission file.create | Niveau sécurité | — |
| Partager dossier/fichier | Décision | WriteIntent access | Permission share | Niveau lien public | — |
| Demande document | Décision | WriteIntent placeholder + activité | Permission request.create | — | — |
| Déplacer vers Trash | Décision | WriteIntent section=Trash | Permission delete | — | Planification purge |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
