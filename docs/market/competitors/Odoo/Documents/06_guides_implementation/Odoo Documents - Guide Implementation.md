# Odoo Documents — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Documents (DMS) dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust (ou modules)
- Schémas de données (dossier, document, tag, accès)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des crates (proposition)

```
crates/
├── miyukini-documents-folder/     # DocumentsFolderOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── folder.rs               # Modèle Folder, sections
│   │   ├── hierarchy.rs            # Hiérarchie, sous-dossiers
│   │   ├── alias.rs                # Alias email
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-documents-file/        # DocumentsFileOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── document.rs             # Modèle Document (fichier, lien, tableur, placeholder)
│   │   ├── version.rs              # Versions
│   │   ├── pdf.rs                  # Split / Merge PDF
│   │   ├── shortcut.rs             # Raccourcis
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-documents-tag/         # DocumentsTagOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── tag.rs                  # Modèle Tag
│   │   ├── assignment.rs           # Affectation tag ↔ fichier/dossier
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-documents-share/       # DocumentsShareOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── access.rs               # Règles d'accès (Viewer/Editor, expiration)
│   │   ├── portal.rs               # Exposition portail (Façade Publique Gouvernée)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-documents-request/    # DocumentsRequestOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── request.rs               # Demande de document, placeholder
│   │   ├── reminder.rs             # Rappels (MiyuNotify)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-documents-ui/          # DocumentsUI
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── tree.rs              # Arborescence sections + dossiers
    │   │   ├── list.rs              # Vue liste
    │   │   ├── grid.rs              # Vue grille
    │   │   ├── preview.rs           # Prévisualisation (PDF, images, vidéos)
    │   │   ├── details_panel.rs     # Info & Tags + chatter
    │   │   └── share_dialog.rs      # Dialogue partage
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyunotify` : Activités, chatter, rappels
- `miyumedia` : Stockage blobs (optionnel)
- `miyucontacts` : Contacts (Owner, Contact)
- `miyuportal` : Accès portail (Façade Publique Gouvernée)
- `miyuclock` : Dates (expiration, due date)

---

## 2. Schémas de données

### 2.1 Modèle Folder

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: FolderId,
    pub name: String,
    pub parent_id: Option<FolderId>,
    pub section: DocumentSection,
    pub owner_id: UserId,
    pub contact_id: Option<PartnerId>,
    pub company_id: CompanyId,
    pub email_alias: Option<String>,
    pub email_domain_id: Option<DomainId>,
    pub activity_type_on_receive: Option<String>,
    pub assignee_id: Option<UserId>,
    pub tag_ids_on_receive: Vec<TagId>,
    pub is_starred: bool, // par utilisateur (vue calculée)
    pub item_count: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentSection {
    All,
    Company,
    MyDrive,
    SharedWithMe,
    Recent,
    Trash,
}
```

### 2.2 Modèle Document

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub name: String,
    pub folder_id: FolderId,
    pub document_type: DocumentType,
    pub owner_id: UserId,
    pub contact_id: Option<PartnerId>,
    pub tag_ids: Vec<TagId>,
    pub is_locked: bool,
    pub size_bytes: Option<u64>,
    pub content_type: Option<String>,
    pub url: Option<String>,           // pour type Link
    pub request_to_id: Option<UserId>, // pour type RequestPlaceholder
    pub due_date: Option<Date>,
    pub version_count: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    Uploaded,
    Link,
    Spreadsheet,
    RequestPlaceholder,
}

pub struct DocumentVersion {
    pub id: VersionId,
    pub document_id: DocumentId,
    pub version_number: u32,
    pub created_at: DateTime,
    pub size_bytes: u64,
}
```

### 2.3 Modèle Tag

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub color: Option<u32>,
    pub tooltip: Option<String>,
    pub company_id: CompanyId,
}
```

### 2.4 Modèle Access (partage)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderAccess {
    pub id: AccessId,
    pub folder_id: FolderId,
    pub principal_type: PrincipalType, // User | Contact
    pub principal_id: Uuid,
    pub role: ShareRole,
    pub expires_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentAccess {
    pub id: AccessId,
    pub document_id: DocumentId,
    pub principal_type: PrincipalType,
    pub principal_id: Uuid,
    pub role: ShareRole,
    pub expires_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShareRole {
    Viewer,
    Editor,
}

pub struct GeneralAccess {
    pub folder_id: Option<FolderId>,
    pub document_id: Option<DocumentId>,
    pub scope: GeneralAccessScope, // InternalUsers | AnyoneWithLink
    pub role: ShareRole,
    pub discoverable: bool,
}
```

### 2.5 Modèle DocumentRequest

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRequest {
    pub id: RequestId,
    pub placeholder_document_id: DocumentId,
    pub request_to_id: UserId,
    pub due_date: Option<Date>,
    pub message: Option<String>,
    pub activity_id: Option<ActivityId>,
    pub status: RequestStatus, // Pending | Fulfilled | Cancelled
    pub fulfilled_at: Option<DateTime>,
}
```

---

## 3. API et contrats

### 3.1 DocumentsFolderOperator

- `create_folder(intent, mandate) -> Result<Folder>`
- `update_folder(folder_id, intent, mandate) -> Result<Folder>`
- `move_to_trash(folder_id, mandate) -> Result<()>`
- `download_zip(folder_id, mandate) -> Result<Stream>`
- `set_email_alias(folder_id, alias, domain_id, activity_type, assignee_id, tag_ids, mandate) -> Result<()>`
- `add_shortcut(folder_id, target_folder_id, mandate) -> Result<Folder>`

### 3.2 DocumentsFileOperator

- `upload_file(intent, mandate) -> Result<Document>`
- `create_link(url, name, folder_id, mandate) -> Result<Document>`
- `update_document(document_id, intent, mandate) -> Result<Document>`
- `move_to_trash(document_id, mandate) -> Result<()>`
- `lock(document_id, mandate) -> Result<()>` / `unlock(document_id, mandate) -> Result<()>`
- `upload_version(document_id, blob, mandate) -> Result<DocumentVersion>`
- `create_shortcut(document_id, folder_id, mandate) -> Result<Document>`
- `split_pdf(document_id, page_splits, mandate) -> Result<Vec<Document>>`
- `merge_pdfs(document_ids, order, mandate) -> Result<Document>`

### 3.3 DocumentsTagOperator

- `create_tag(intent, mandate) -> Result<Tag>`
- `update_tag(tag_id, intent, mandate) -> Result<Tag>`
- `assign_to_document(document_id, tag_ids, mandate) -> Result<()>`
- `assign_to_folder(folder_id, tag_ids_for_alias, mandate) -> Result<()>`

### 3.4 DocumentsShareOperator

- `share_folder(folder_id, grants, general_access, mandate) -> Result<()>`
- `share_document(document_id, grants, general_access, mandate) -> Result<()>`
- `revoke_access(resource, principal_id, mandate) -> Result<()>`
- `check_access(resource, user_id, required_role) -> Result<bool>`

### 3.5 DocumentsRequestOperator

- `create_request(intent, mandate) -> Result<DocumentRequest>`
- `upload_on_placeholder(placeholder_id, blob, mandate) -> Result<Document>`
- `send_reminder(request_id, mandate) -> Result<()>`
- `send_reminder_bulk(mandate) -> Result<()>`
- `cancel_request(request_id, mandate) -> Result<()>`

---

## 4. Plan de développement par phases

### Phase 1 — MVP (Dossiers + Fichiers + Partage basique)

- **Jalons :**
  - Création/modification/suppression dossiers (sections Company, My Drive, Trash)
  - Upload fichier, lien URL, métadonnées (nom, dossier, propriétaire, contact, tags)
  - Partage dossier/fichier : Viewer/Editor, utilisateurs internes, expiration
  - Vue arborescence + liste/grille + prévisualisation (PDF, images)
  - Panneau Info & Tags (détails, chatter via MiyuNotify)
- **Crates :** miyukini-documents-folder, miyukini-documents-file, miyukini-documents-share, miyukini-documents-tag (minimal), miyukini-documents-ui (tree, list, grid, preview, details, share dialog)
- **Estimation :** 8–12 semaines

### Phase 2 — Tags, Trash, Raccourcis, Versions

- **Jalons :**
  - Tags (création, affectation, configuration centrale)
  - Corbeille : délai de suppression configurable, purge planifiée (Ever Buddy)
  - Raccourcis (dossier, fichier)
  - Versions (historique, téléchargement, upload nouvelle version)
  - Lock/Unlock fichier
- **Estimation :** 4–6 semaines

### Phase 3 — Demandes de documents, Alias email

- **Jalons :**
  - Demandes de documents (placeholder, Request To, Due Date, Folder, Tags, Message)
  - Activité « document demandé » et vue Activités (colonne Requested Document)
  - Rappels unitaires et groupés (MiyuNotify)
  - Alias email par dossier (domaine, activité, assignation, tags)
- **Crates :** miyukini-documents-request (renforcé), intégration MiyuNotify
- **Estimation :** 4–6 semaines

### Phase 4 — PDF (Split/Merge), Centralisation, Portail

- **Jalons :**
  - Split PDF (découpage en plusieurs documents)
  - Merge PDF (fusion de plusieurs PDF)
  - Centralisation par « app » Miyukini (dossier cible, tags, sous-dossiers) ; suppression entité source → déplacement pièces jointes vers Trash
  - Exposition portail (Façade Publique Gouvernée, Mandat Public d’Accès, carte Documents)
- **Estimation :** 6–8 semaines

### Phase 5 — Numérisation IA, Spreadsheet, Favoris

- **Jalons :**
  - Numérisation IA (dossier Finance : Create Vendor Bill / Customer Invoice / Credit Note ‣ Send for Digitization) — intégration MiyuInvoice (ou équivalent)
  - Création tableur depuis Documents (New ‣ Spreadsheet) — intégration Opérateur Spreadsheet si existant
  - Favoris (étoile) par utilisateur, filtre Starred
- **Estimation :** 4–6 semaines

---

## 5. Bornage fonctionnel

### MVP (Phase 1)

- **Inclus :** Dossiers (Company, My Drive), sous-dossiers, upload fichier, lien URL, tags basiques, partage Viewer/Editor (internes + expiration), arbre + liste/grille + prévisualisation, Info & Tags + chatter
- **Exclu :** Trash avec délai, versions, lock, raccourcis, demandes de documents, alias email, split/merge PDF, centralisation, portail, IA, spreadsheet, favoris

### Complet (Phases 1–5)

- **Inclus :** Tout ce qui est décrit dans les 7 documents d’analyse (logique métier, parcours, UI/UX, intégrations, spécifications Opérateurs, COG, implémentation)
- **Exclu :** Fonctionnalités non documentées Odoo Documents (ex. extensions métier spécifiques)

### Critères d’acceptation

- Toute création/modification/suppression/partage passe par StrongFather (décision) et KindMother (WriteIntent)
- Permissions vérifiées via Master Butler ; niveau sécurité via WorrySentinel
- Partage externe aligné sur Façade Publique Gouvernée et Mandat Public d’Accès
- Demandes de documents avec activités et rappels via MiyuNotify
- Corbeille avec délai configurable et purge planifiée (Ever Buddy)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
