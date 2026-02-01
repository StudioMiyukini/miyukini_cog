# Odoo Website Builder — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Website Builder dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

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
WebsiteUI → BondingBrother → WebsitePageOperator → StrongFather (décision)
                             → KindMother (WriteIntent)
                             → Master Butler (permissions)
                             → WorrySentinel (sécurité)
                             → Ever Buddy (cycle de vie)

WebsiteFormOperator → BondingBrother → MiyuCRM / MiyuForum / MiyuContacts / MiyuProject / MiyuNotify
```

### 1.2 Flux Typique — Création de page

1. **Intention utilisateur** → WebsiteUI (mode Edit, + New Page)
2. **Traduction intention** → BondingBrother
3. **Demande décision** → StrongFather
4. **Vérification permissions** → Master Butler
5. **Vérification sécurité** → WorrySentinel
6. **Persistance** → KindMother (WriteIntent)
7. **Mise à jour menu** (si « In Menu ») → WebsiteMenuOperator

### 1.3 Flux Typique — Soumission formulaire

1. **Soumission visiteur** → WebsiteUI (frontend)
2. **Réception** → WebsiteFormOperator
3. **Validation** → WebsiteFormOperator (champs, action)
4. **Demande décision** → StrongFather (autoriser création enregistrement)
5. **Traduction intention** → BondingBrother (créer opportunité / ticket / contact / tâche / newsletter)
6. **Opérateur métier** → MiyuCRM / MiyuForum / MiyuContacts / MiyuProject / MiyuNotify (sous Mandat)
7. **Réponse** → Redirection ou message (On Success)

---

## 2. Patterns d'Intégration

### 2.1 Création de page

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreatePageIntent {
    pub title: String,
    pub url: Option<String>,
    pub template_id: Option<BlockTemplateId>,
    pub in_menu: bool,
    pub is_homepage: bool,
    pub visibility: PageVisibility,
}

impl WebsitePageOperator {
    pub async fn create_page(
        &self,
        intent: CreatePageIntent,
        mandate: Mandate,
    ) -> Result<WebsitePage, WebsiteError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["page.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_page",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(WebsiteError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "page.create",
                resource: None,
            })
            .await?;

        if !permission.granted {
            return Err(WebsiteError::PermissionDenied);
        }

        let security_level = self.worry_sentinel
            .get_security_level(&intent)
            .await?;

        if security_level > mandate.max_security_level {
            return Err(WebsiteError::SecurityLevelExceeded);
        }

        let write_intent = WriteIntent {
            entity_type: "website.page",
            operation: WriteOperation::Create,
            data: PageData {
                title: intent.title,
                url: intent.url.unwrap_or_else(|| slugify(&intent.title)),
                in_menu: intent.in_menu,
                is_homepage: intent.is_homepage,
                visibility: intent.visibility,
                published: false,
                website_id: self.get_website_id().await?,
            },
            security_level,
        };

        let page = self.kind_mother.persist(write_intent).await?;

        if intent.in_menu {
            self.website_menu_operator
                .add_page_to_menu(page.id(), mandate.clone())
                .await?;
        }

        Ok(page)
    }
}
```

### 2.2 Publication de page

**Pattern :** WriteIntent (Update) + Mandate

```rust
pub struct PublishPageIntent {
    pub page_id: PageId,
    pub publish: bool,
    pub publishing_date: Option<DateTime>,
}

impl WebsitePageOperator {
    pub async fn publish_page(
        &self,
        intent: PublishPageIntent,
        mandate: Mandate,
    ) -> Result<(), WebsiteError> {
        mandate.validate_flows(&["page.publish"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "publish_page",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(WebsiteError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "website.page",
            operation: WriteOperation::Update,
            data: PageData {
                id: Some(intent.page_id),
                published: intent.publish,
                publishing_date: intent.publishing_date,
                ..Default::default()
            },
            security_level: mandate.max_security_level,
        };

        self.kind_mother.persist(write_intent).await?;
        Ok(())
    }
}
```

### 2.3 Soumission de formulaire (création opportunité CRM)

**Pattern :** Mandate + délégation vers Opérateur métier

```rust
pub struct FormSubmitIntent {
    pub form_id: FormId,
    pub action: FormAction, // CreateOpportunity, CreateTicket, CreateCustomer, etc.
    pub fields: HashMap<String, Value>,
    pub visitor_context: Option<VisitorContext>,
}

impl WebsiteFormOperator {
    pub async fn submit_form(
        &self,
        intent: FormSubmitIntent,
        mandate: Mandate,
    ) -> Result<FormSubmitResult, WebsiteError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["form.submit"])?;

        let form_config = self.get_form_config(intent.form_id).await?;
        if form_config.action != intent.action {
            return Err(WebsiteError::ActionMismatch);
        }

        self.validate_fields(&form_config, &intent.fields)?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "form_submit",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(WebsiteError::DecisionDenied);
        }

        // Délégation vers Opérateur métier via BondingBrother
        let delegate_mandate = self.bonding_brother
            .request_mandate_for_flow(
                "crm.create_opportunity",
                &[MiyuCrmOperator::id()],
                mandate.max_security_level,
            )
            .await?;

        let opportunity_intent = CreateOpportunityIntent {
            name: intent.fields.get("name").cloned(),
            email: intent.fields.get("email").cloned(),
            phone: intent.fields.get("phone").cloned(),
            description: intent.fields.get("message").cloned(),
            source: Some("website_form"),
        };

        let _opportunity = self.bonding_brother
            .call(MiyuCrmOperator::create_opportunity(opportunity_intent), delegate_mandate)
            .await?;

        Ok(FormSubmitResult {
            success: true,
            redirect_url: form_config.on_success_redirect_url.clone(),
            message: form_config.on_success_message.clone(),
        })
    }
}
```

### 2.4 Création de redirection

**Pattern :** WriteIntent + Mandate

```rust
pub struct CreateRedirectIntent {
    pub url_from: String,
    pub url_to: String,
    pub redirect_type: RedirectType, // 301, 302, 308, 404
    pub website_id: Option<WebsiteId>,
    pub sequence: u32,
}

impl WebsiteRedirectOperator {
    pub async fn create_redirect(
        &self,
        intent: CreateRedirectIntent,
        mandate: Mandate,
    ) -> Result<WebsiteRedirect, WebsiteError> {
        mandate.validate_flows(&["redirect.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_redirect",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(WebsiteError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "website.redirect",
            operation: WriteOperation::Create,
            data: RedirectData {
                url_from: intent.url_from,
                url_to: intent.url_to,
                redirect_type: intent.redirect_type,
                website_id: intent.website_id,
                sequence: intent.sequence,
                active: true,
            },
            security_level: mandate.max_security_level,
        };

        self.kind_mother.persist(write_intent).await
    }
}
```

---

## 3. Gestion des Gouvernances

### 3.1 Visibilité des pages (Public / Signed In / Restricted / Password)

- **Public** : pas de vérification ; page exposée via Façade Publique Gouvernée sous Mandat Public d’Accès.
- **Signed In** : vérification session (Utilisateur Visiteur ou citoyen) ; WorrySentinel + Master Butler.
- **Restricted Group** : vérification groupe d’accès ; Master Butler (permissions).
- **With Password** : vérification mot de passe (stocké de façon sécurisée) ; WorrySentinel.

### 3.2 Utilisateur externe (visiteur)

- Ne pénètre jamais dans le COG ; consomme uniquement la Façade Publique Gouvernée.
- Formulaires : soumission via WebsiteFormOperator qui, sous Mandat, délègue aux Opérateurs métier.
- Pas d’identité persistante obligatoire ; traçabilité possible (visitor_id, UTM) selon politique.

### 3.3 Éditeur (utilisateur connecté)

- Accès mode Edit via Mandat de Permission (WebsitePageOperator, WebsiteMenuOperator, WebsiteBlockOperator).
- Toute modification (page, menu, bloc) passe par WriteIntent et StrongFather/KindMother.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
