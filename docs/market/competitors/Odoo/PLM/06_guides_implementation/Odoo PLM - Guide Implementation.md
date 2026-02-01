# Odoo PLM — Guide d'Implémentation

## Contexte

Ce document fournit un **guide d'implémentation technique** pour développer l'équivalent PLM (Product Lifecycle Management) dans Miyukini — ECO, révisions BoM, approbations, versioning, documents.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique des crates Rust
- Schémas de données (ECO, EcoType, BomRevision, Approval, Document)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates (proposition)

```
crates/
├── miyuplm/                          # PlmService — orchestration
│   ├── src/
│   │   ├── lib.rs
│   │   ├── eco.rs                    # EcoOperator (logique ECO)
│   │   ├── eco_type.rs               # EcoTypeOperator
│   │   ├── bom_revision.rs           # BomRevisionOperator
│   │   ├── approval.rs               # EcoApprovalOperator
│   │   ├── document.rs               # EcoDocumentOperator
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuplm-ui/                       # PlmUI
│   ├── src/
│   │   ├── lib.rs
│   │   ├── views/
│   │   │   ├── overview.rs           # Vue d'ensemble par type d'ECO
│   │   │   ├── eco_form.rs           # Formulaire ECO
│   │   │   ├── eco_list.rs           # Liste / Kanban ECO
│   │   │   ├── bom_revision_form.rs  # Révision BoM
│   │   │   ├── bom_changes.rs        # Onglet comparaison composants
│   │   │   ├── operation_changes.rs  # Onglet comparaison opérations
│   │   │   └── document_manager.rs   # Gestion documents ECO
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── (dépendances)
    ├── miyukini-kernel
    ├── miyukini-central               # Cores
    ├── miyumedia                     # Pièces jointes (documents)
    ├── miyunotify                    # Notifications, Chatter
    └── (Manufacturing / BoM)        # Nomenclatures — crate existante ou à définir
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, TAMR

**Kits existants :**
- `miyumedia` : Stockage et liens documents
- `miyunotify` : Notifications, commentaires (Chatter-like)
- Manufacturing / BoM : modèles produit, nomenclature (composants, opérations) — à lier selon architecture existante

---

## 2. Schémas de Données

### 2.1 Modèle Eco

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eco {
    pub id: EcoId,
    pub name: String,                 // ex. ECO005
    pub description: String,
    pub eco_type_id: EcoTypeId,
    pub apply_on: ApplyOn,             // BillOfMaterials | ProductOnly
    pub product_id: ProductId,
    pub bom_id: Option<BomId>,
    pub company_id: Option<CompanyId>,
    pub responsible_id: Option<UserId>,
    pub effective: Effective,          // AsSoonAsPossible | AtDate(DateTime<Utc>)
    pub tag_ids: Vec<TagId>,
    pub stage_id: StageId,
    pub revision_id: Option<BomRevisionId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplyOn {
    BillOfMaterials,
    ProductOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effective {
    AsSoonAsPossible,
    AtDate(DateTime<Utc>),
}
```

### 2.2 Modèle EcoType

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoType {
    pub id: EcoTypeId,
    pub name: String,
    pub stage_ids: Vec<StageId>,
    pub approval_stage_ids: Vec<StageId>,
    pub alias_email: Option<String>,
}
```

### 2.3 Modèle Stage (workflow)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoStage {
    pub id: StageId,
    pub eco_type_id: EcoTypeId,
    pub name: String,
    pub sequence: u32,
    pub requires_approval: bool,
}
```

### 2.4 Modèle BomRevision

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomRevision {
    pub id: BomRevisionId,
    pub eco_id: EcoId,
    pub source_bom_id: BomId,
    pub version: u32,
    pub is_archived: bool,
    pub components: Vec<BomLine>,
    pub operations: Vec<BomOperation>,
    pub created_at: DateTime<Utc>,
}
```

### 2.5 Modèle Approval

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub id: ApprovalId,
    pub eco_id: EcoId,
    pub stage_id: StageId,
    pub approver_id: UserId,
    pub granted: bool,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

### 2.6 Lien Document ↔ ECO / BoM

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoDocument {
    pub id: EcoDocumentId,
    pub eco_id: EcoId,
    pub attachment_id: AttachmentId,
    pub removed: bool,
}
```

---

## 3. API et Contrats

### 3.1 EcoOperator

- `create_eco(intent, mandate) -> Result<Eco>`
- `update_eco(eco_id, intent, mandate) -> Result<Eco>`
- `start_revision(eco_id, mandate) -> Result<(Eco, BomRevision)>`
- `apply_changes(eco_id, mandate) -> Result<Eco>`
- `apply_rebase(eco_id, mandate) -> Result<BomRevision>`
- `move_stage(eco_id, stage_id, mandate) -> Result<Eco>`
- `get_eco(eco_id) -> Result<Eco>`
- `list_ecos_by_type(eco_type_id, filters) -> Result<Vec<Eco>>`

### 3.2 BomRevisionOperator

- `create_revision(intent, mandate) -> Result<BomRevision>`
- `update_revision(revision_id, components | operations, mandate) -> Result<BomRevision>`
- `compare_with_production(revision_id) -> Result<BomChangesDiff>`
- `get_revision_by_eco(eco_id) -> Result<BomRevision>`
- `history(bom_id) -> Result<Vec<(Eco, BomRevision)>>`

### 3.3 EcoApprovalOperator

- `request_approval(eco_id, stage_id, mandate) -> Result<()>`
- `grant_approval(intent, mandate) -> Result<Approval>`
- `all_approvals_granted(eco_id, stage_id) -> Result<bool>`
- `list_pending(eco_id) -> Result<Vec<ApprovalRequest>>`

### 3.4 EcoDocumentOperator

- `attach(eco_id, file, mandate) -> Result<EcoDocument>`
- `remove(eco_id, document_id, mandate) -> Result<()>`
- `sync_to_bom(eco_id, bom_id, mandate) -> Result<()>`
- `list_documents(eco_id) -> Result<Vec<EcoDocument>>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (ECO + Révision + Apply Changes)

- **Objectif :** Créer un ECO, démarrer une révision BoM, modifier composants/opérations (modèle simplifié si pas de Manufacturing complet), appliquer les changements (bascule BoM).
- **Crates :** miyuplm (eco, eco_type, bom_revision), liaison minimale à un modèle BoM existant ou stub.
- **Fonctionnalités :**
  - EcoType avec stages (Nouveau, En cours, Vérification, Clôture)
  - ECO : création, Start Revision, Apply Changes (sans approbation obligatoire en MVP)
  - Révision BoM : copie, modification composants (lignes), comparaison BoM Changes (diff)
- **Pas dans MVP :** Approbations, documents, rebase, effective date, Quality Control Points.

### Phase 2 — Approbations et Documents

- **Objectif :** Stages « vérification » avec approbations ; déblocage Apply Changes ; gestion documents ECO et sync vers BoM.
- **Fonctionnalités :**
  - EcoApprovalOperator : request_approval, grant_approval, all_approvals_granted
  - EcoDocumentOperator : attach, remove, sync_to_bom
  - PlmUI : formulaire approbation, smart button Documents, liste pièces jointes
- **Intégration :** MiyuNotify (notifications approbation), MiyuMedia (fichiers).

### Phase 3 — Rebase et Traçabilité

- **Objectif :** Détection base obsolète, Apply Rebase ; historique versions ; effective date.
- **Fonctionnalités :**
  - Ever Buddy : revision_base_is_obsolete
  - KindMother : rebase_revision (fusion)
  - PlmUI : onglet Previous Eco Bom Changes, bouton Apply Rebase
  - Historique : liste ECO (filtre Done), version BoM, effective date
- **Règles :** Toujours enregistrer une date effective (ex. date Apply Changes) pour traçabilité.

### Phase 4 — Qualité et UX

- **Objectif :** Quality Control Points (Steps) dans les opérations si app Quality existe ; amélioration UX (Overview, indicateurs, breadcrumbs, messages d’aide).
- **Fonctionnalités :**
  - Operation Changes (diff opérations + steps)
  - Réordonnancement steps (glisser-déposer)
  - Overview par type d’ECO avec compteurs
  - Alias email (création ECO par email) si mail/entrée externe disponible

---

## 5. Bornage Fonctionnel

### MVP (Phase 1)

- **In scope :** ECO (création, type, produit, BoM, Apply on), Start Revision, révision BoM (composants, opérations), comparaison BoM Changes, Apply Changes (bascule production), stages simples.
- **Out of scope :** Approbations, documents, rebase, effective date, Quality, alias email.

### Complet (Phases 2–4)

- **In scope :** Tout ce qui précède + approbations (TAMR + StrongFather), documents (attach/remove/sync), rebase, historique versions et effective date, Quality Control Points (si Quality app), Overview, alias email.
- **Out of scope :** Fonctionnalités non documentées Odoo PLM (ex. intégrations métier spécifiques au-delà Manufacturing, Inventory, Quality, Mail).

---

## 6. Risques et Points d'Attention

- **Modèle BoM** : Dépendance à une crate Manufacturing/BoM existante ; définir clairement le contrat (lecture BoM production, écriture après Apply Changes, versioning).
- **Concurrence** : Gestion des ECO concurrents sur la même BoM (rebase) — éviter écrasements ; transactions et verrous si nécessaire.
- **Performances** : Comparaison BoM (grosses nomenclatures) — calcul diff efficace (composants, opérations).
- **Sécurité** : Données sensibles (nomenclatures, documents) — niveau 2, isolation par entreprise/équipe (WorrySentinel).

---

**Document rédigé selon la méthodologie d'analyse Odoo.**
