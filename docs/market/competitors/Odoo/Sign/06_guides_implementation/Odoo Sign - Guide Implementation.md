# Odoo Sign — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Sign dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates)
- Spécifications des modèles et schémas de données
- API et contrats des Opérateurs
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates (proposition)

```
crates/
├── miyusign/                          # SignRequestOperator + SignTemplateOperator (noyau)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── request.rs                 # Modèle SignRequest, états, envoi
│   │   ├── template.rs                # Modèle SignTemplate, items
│   │   ├── role.rs                    # Rôles, auth renforcée
│   │   ├── item_type.rs               # Types de champs, auto-fill
│   │   ├── token.rs                   # Génération / validation tokens signataires
│   │   ├── compliance.rs              # Hash, preuves (ou crate dédiée)
│   │   ├── admin_cell.rs
│   │   └── errors.rs
│   └── Cargo.toml
│
├── miyusign_compliance/               # SignComplianceOperator (optionnel, séparation)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── hash.rs                    # Génération hash signataire
│   │   ├── proof.rs                   # Enregistrement et export preuves
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyusign_ui/                       # SignUI (dashboard, éditeur, page signataire)
    ├── src/
    │   ├── lib.rs
    │   ├── dashboard.rs
    │   ├── editor.rs                  # Éditeur PDF + champs
    │   ├── send_wizard.rs
    │   ├── signer_page.rs             # Page signataire (portail/public)
    │   ├── config.rs                  # Rôles, types de champs, tags
    │   └── admin_cell.rs
    └── Cargo.toml
```

**Alternative :** Tout regrouper dans `miyusign` (request, template, role, item_type, compliance) + `miyusign_ui` si on veut limiter le nombre de crates.

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyucontacts` : Signataires (res.partner), auto-fill
- `miyunotify` : Envoi emails, relances, notifications
- `miyuexport` / stockage : Archivage PDF signés (workspace, tags) — ou module Documents équivalent
- `miyuclock` : Horodatage
- `miyuauth` ou équivalent : Tokens sécurisés (optionnel selon implémentation)

---

## 2. Schémas de Données

### 2.1 Modèle SignRequest

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRequest {
    pub id: SignRequestId,
    pub template_id: SignTemplateId,
    pub state: SignRequestState,
    pub reference: Option<String>,
    
    // Signataires (role_id, partner_id, order)
    pub signers: Vec<SignerAssignment>,
    pub sign_order_enabled: bool,
    
    // Options
    pub valid_until: Option<Date>,
    pub reminder: bool,
    pub reminder_days: u32,
    pub subject: String,
    pub message: Option<String>,
    pub redirect_url: Option<String>,
    
    // Progression
    pub completed_signer_ids: Vec<RoleId>,
    pub refused_by: Option<RoleId>,
    
    // Lien optionnel métier
    pub sale_order_id: Option<SaleOrderId>,
    pub crm_lead_id: Option<CrmLeadId>,
    
    pub company_id: CompanyId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignRequestState {
    Shared,   // brouillon
    Sent,
    Signed,
    Refused,
    Canceled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerAssignment {
    pub role_id: RoleId,
    pub partner_id: PartnerId,
    pub order: Option<u32>,
}
```

### 2.2 Modèle SignTemplate

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignTemplate {
    pub id: SignTemplateId,
    pub name: String,
    pub attachment_id: AttachmentId,   // PDF
    pub active: bool,
    
    // Champs (items) : type, rôle, position, page, auto_fill_partner_field, tip, placeholder
    pub item_ids: Vec<SignItemId>,
    
    // Propriétés
    pub tag_ids: Vec<TagId>,
    pub signed_document_workspace_id: Option<WorkspaceId>,
    pub signed_document_tag_ids: Vec<TagId>,
    pub redirect_link: Option<String>,
    pub authorized_user_ids: Vec<UserId>,
    
    pub company_id: CompanyId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.3 Modèle SignItem (champ sur template)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignItem {
    pub id: SignItemId,
    pub template_id: SignTemplateId,
    pub item_type_id: SignItemTypeId,
    pub responsible_id: RoleId,
    pub page: u32,
    pub pos_x: f64,
    pub pos_y: f64,
    pub width: f64,
    pub height: f64,
    pub tip: Option<String>,
    pub placeholder: Option<String>,
    pub auto_fill_partner_field: Option<String>,  // nom technique res.partner
}
```

### 2.4 Modèle SignItemType (type de champ)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignItemType {
    pub id: SignItemTypeId,
    pub name: String,
    pub field_type: SignFieldType,
    pub auto_fill_partner_field: Option<String>,
    pub default_width: f64,
    pub default_height: f64,
    pub tip: Option<String>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignFieldType {
    Signature,
    Initial,
    Text,
    MultilineText,
    Checkbox,
    Selection,
}
```

### 2.5 Modèle SignRole

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRole {
    pub id: RoleId,
    pub name: String,
    pub color: Option<u32>,
    pub change_authorized: bool,
    pub extra_authentication: ExtraAuthentication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtraAuthentication {
    None,
    Sms,
    Itsme,
    AadhaarESign,
}
```

### 2.6 Modèle SignerToken (accès signataire)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerToken {
    pub token: String,
    pub request_id: SignRequestId,
    pub role_id: RoleId,
    pub partner_id: PartnerId,
    pub valid_until: Option<DateTime>,
    pub used_at: Option<DateTime>,
}
```

### 2.7 Modèle SignatureProof (preuve)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureProof {
    pub id: ProofId,
    pub request_id: SignRequestId,
    pub role_id: RoleId,
    pub signatory_hash: String,
    pub ip: Option<String>,
    pub timestamp: DateTime,
    pub auth_type: Option<ExtraAuthentication>,
}
```

---

## 3. API et Contrats

### 3.1 SignRequestOperator API

```rust
#[async_trait]
pub trait SignRequestOperatorTrait {
    async fn create_and_send(
        &self,
        intent: CreateAndSendRequestIntent,
        mandate: Mandate,
    ) -> Result<SignRequest, SignError>;
    
    async fn get_request(
        &self,
        request_id: SignRequestId,
        mandate: Mandate,
    ) -> Result<SignRequest, SignError>;
    
    async fn list_requests(
        &self,
        filters: SignRequestFilters,
        mandate: Mandate,
    ) -> Result<Vec<SignRequest>, SignError>;
    
    async fn cancel_request(
        &self,
        request_id: SignRequestId,
        mandate: Mandate,
    ) -> Result<(), SignError>;
    
    // Côté signataire (token, pas Mandate)
    async fn get_signer_page(
        &self,
        token: String,
    ) -> Result<SignerPageData, SignError>;
    
    async fn submit_signature(
        &self,
        intent: SubmitSignatureIntent,
    ) -> Result<SignRequest, SignError>;
    
    async fn refuse_signature(
        &self,
        request_id: SignRequestId,
        token: String,
    ) -> Result<(), SignError>;
}
```

### 3.2 SignTemplateOperator API

```rust
#[async_trait]
pub trait SignTemplateOperatorTrait {
    async fn create_template(
        &self,
        intent: CreateTemplateIntent,
        mandate: Mandate,
    ) -> Result<SignTemplate, SignError>;
    
    async fn update_template(
        &self,
        template_id: SignTemplateId,
        updates: TemplateUpdates,
        mandate: Mandate,
    ) -> Result<SignTemplate, SignError>;
    
    async fn get_template(
        &self,
        template_id: SignTemplateId,
        mandate: Mandate,
    ) -> Result<SignTemplate, SignError>;
    
    async fn list_templates(
        &self,
        filters: TemplateFilters,
        mandate: Mandate,
    ) -> Result<Vec<SignTemplate>, SignError>;
    
    async fn duplicate_from_request(
        &self,
        request_id: SignRequestId,
        mandate: Mandate,
    ) -> Result<SignTemplate, SignError>;
}
```

### 3.3 SignComplianceOperator API

```rust
#[async_trait]
pub trait SignComplianceOperatorTrait {
    async fn record_signature_evidence(
        &self,
        request: RecordEvidenceRequest,
    ) -> Result<SignatureProof, SignError>;
    
    async fn verify_integrity(
        &self,
        request_id: SignRequestId,
    ) -> Result<IntegrityResult, SignError>;
    
    async fn export_proof(
        &self,
        request_id: SignRequestId,
        mandate: Mandate,
    ) -> Result<ProofExport, SignError>;
}
```

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (signature simple)

- **Objectif :** One-time signature, un signataire, pas d’ordre, pas d’auth renforcée.
- **Contenu :**
  - Modèles : SignRequest, SignTemplate (simplifié), SignItem, SignRole (sans extra_authentication).
  - SignRequestOperator : create_and_send, get_signer_page (token), submit_signature.
  - SignTemplateOperator : create_template, get_template, list_templates.
  - SignUI : dashboard (upload PDF, envoi), page signataire (PDF + champs signature/texte/checkbox).
  - MiyuNotify : envoi lien signataire, confirmation après signature.
  - Tokens : génération et validation.
- **Hors scope MVP :** Templates réutilisables avancés, ordre de signature, relances, valid_until, SMS/itsme®/Aadhaar, archivage Documents.

### Phase 2 — Templates et ordre

- Templates réutilisables avec propriétés (tags, workspace, redirect, authorized users).
- Ordre de signature (sign_order_enabled, envoi au signataire suivant après signature).
- Validité (valid_until) et expiration (job).
- Relances (reminder, reminder_days, job).
- Archivage des documents signés (workspace + tags) via MiyuDocuments ou équivalent.

### Phase 3 — Conformité et auth renforcée

- SignComplianceOperator : hash, enregistrement preuve, export preuve.
- SignRoleOperator : extra_authentication (SMS, itsme®, Aadhaar selon disponibilité).
- Intégration fournisseurs (SMS, itsme®, Aadhaar) avec abstraction.
- Gestion crédits / quotas (admin).
- Option Frame (affichage hash sur signature).

### Phase 4 — Intégrations métier

- Lien optionnel SignRequest ↔ Sale Order / CRM Lead.
- Actions contextuelles « Envoyer à signer » depuis commande / opportunité.
- Événements signed / refused / expired pour autres modules.

---

## 5. Bornage Fonctionnel

### MVP (Phase 1)

| Fonctionnalité | Inclus |
|----------------|--------|
| Upload PDF one-time | Oui |
| Champs : Signature, Text, Checkbox | Oui |
| Un signataire | Oui |
| Envoi par email (lien unique) | Oui |
| Page signataire (remplir + valider) | Oui |
| Confirmation + copie signataire (optionnel) | Oui |
| Templates réutilisables | Non |
| Ordre de signature | Non |
| Validité / relances | Non |
| Auth renforcée (SMS, itsme®, Aadhaar) | Non |
| Archivage Documents (workspace, tags) | Non |
| Hash / preuves / export preuve | Non |

### Complet (Phases 2–4)

| Fonctionnalité | Inclus |
|----------------|--------|
| Templates avec propriétés (workspace, tags, redirect, authorized users) | Oui |
| Ordre de signature | Oui |
| Validité (valid_until) et expiration | Oui |
| Relances automatiques | Oui |
| Archivage Documents | Oui |
| Hash et preuves d’intégrité | Oui |
| Export preuve (audit) | Oui |
| Auth renforcée (SMS, itsme®, Aadhaar) | Oui (selon fournisseurs) |
| Lien Sales / CRM | Oui |
| Événements signed / refused / expired | Oui |

---

## 6. Risques et Points d’Attention

- **Conformité juridique :** Documenter « à usage informatif » ; ne pas garantir validité juridique par pays ; conseil juridique recommandé.
- **Tokens :** Stockage sécurisé, révocation à l’annulation, durée de vie optionnelle.
- **PDF :** Génération du PDF signé (fusion champs + signatures + hash) peut nécessiter une librairie PDF (ex. Rust: pdf, printpdf, ou appel à un service).
- **Coûts SMS / Aadhaar :** Gestion des crédits et alertes pour éviter les abus ou les échecs silencieux.
- **Performance :** Jobs de relances et d’expiration à planifier (cron ou queue) sans surcharge.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
