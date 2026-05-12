# JayCloud — Spec MSCM / MIP Conformité

> Document **P1** du protocole MIP. Prérequis : *JayCloud - Document
> Fondateur.md* validé.

## 0. Cadre / Méthode

### 0.1 Position dans le protocole MIP

| Étape | Livrable | État |
|-------|----------|------|
| P0 | *Document Fondateur* | ✅ validé |
| **P1** | **Ce document — Spec MSCM / MIP Conformité** | 🟢 en cours |
| P2 | Skeleton crates + manifest + workspace entry | à venir |
| P3 | Backup core + WebDAV (Litmus pass) | à venir |
| P4 | UI web backup + scheduling | à venir |
| P5 | Migration MiyuCloud | à venir |
| P6 | Intégration Alicia + sauvegarde MWS inter-COG | à venir |
| P7 | Polish (intégrité périodique, RFC 3253, sharing extensions) | à venir |

### 0.2 Périmètre du document

| In scope | Hors scope |
|----------|------------|
| Architecture interne (Opérateurs / Kits) du service de sauvegarde | Tout ce qui n'est pas sauvegarde cloud — voir docs des services Jay correspondants |
| Choix des bibliothèques opensource (matrice + licences) | UI Dioxus de Central |
| Schémas de persistance JayCloud (libSQL chiffré) | Schémas internes des services Jay (qui ne sont pas consommés) |
| WebDAV (seul protocole standard exposé) | CalDAV, CardDAV, IMAP, SMTP (hors scope JayCloud) |
| Plan d'implémentation crate par crate | Roadmap calendaire |
| Tests de conformité (Litmus) | CI/CD pipelines (futur *JayCloud - Ops*) |

### 0.3 Contraintes absolues

| Contrainte | Description |
|------------|-------------|
| ❌ **Une seule responsabilité** | Sauvegarde cloud. Toute fonctionnalité hors backup → rejet. |
| ❌ **Aucun couplage aux services Jay** | JayCloud ne consomme `jaykoa-client` / `jaycontact-client` / `jaymail-client` ni aucun autre client Jay. |
| ❌ **Pas de lib AGPL** | Double licence Miyukini incompatible avec AGPL viral. |
| ❌ **Pas de protocole sans test de conformité** | WebDAV doit passer Litmus en P3. |
| ✅ **Source de vérité** | Ce document est la référence pour l'architecture et les choix techniques. |

### 0.4 Décisions structurantes (mini log)

| Id | Décision | Justification |
|----|----------|---------------|
| **DT-01** | **`dav-server` 0.11.0 (Apache-2.0) comme base WebDAV.** | Mature, RFC 4918, Litmus pass, pluggable filesystem trait, compatible Axum. Évite 6 mois de plomberie HTTP/XML. |
| **DT-02** | **WebDAV est le SEUL protocole standard exposé.** Pas de CalDAV, pas de CardDAV, pas d'IMAP. | Cohérent avec DS-02/DS-05 du Doc Fondateur : JayCloud = backup, pas synchronisation live. Si une exposition CalDAV/CardDAV est un jour nécessaire, elle appartiendra à JayKoa/JayContact eux-mêmes. |
| **DT-03** | **Architecture en 3 couches : Adaptateur WebDAV → Opérateurs → Kits transversaux.** | Cohérent avec les autres services Miyukini (JayKonta, JayKoa). |
| **DT-04** | **Aucune dépendance vers un autre service Jay.** Pas de `jaykoa-client`, `jaycontact-client`, `jaymail-client` etc. dans `crates/jaycloud/Cargo.toml`. | Garantit l'absence de doublon et l'autonomie totale du service de sauvegarde. |
| **DT-05** | **Persistance via libSQL chiffré (clé dérivée KindMother) ; filesystem chiffré au repos pour les blobs.** | Cohérent avec le reste du COG. |
| **DT-06** | **Authentification = délégation KindMother + jetons applicatifs WebDAV signés.** | Évite tout stockage de mot de passe ; chaque outil de backup tiers reçoit son app-password scopé. |
| **DT-07** | **`files_op` est l'héritier direct de MiyuCloud**, intégré au crate `jaycloud` (pas un crate séparé). | MiyuCloud étant retiré, son code est rapatrié dans `crates/jaycloud/src/files/`. |
| **DT-08** | **Snapshots stockés en content-addressed storage (CAS) + manifest libSQL.** | Évite la duplication entre snapshots incrémentaux ; dédup naturelle via hash de bloc. |
| **DT-09** | **Pas de cache de second niveau en P3-P4.** | Le filesystem OS et libSQL suffisent. Cache évalué en P7 si nécessaire. |

---

## 1. Architecture en couches

### 1.1 Vue globale

```
┌─────────────────────────────────────────────────────────────────────┐
│                  JayCloud (crates/jaycloud)                         │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  COUCHE 1 — ADAPTATEURS (protocoles exposés)                │   │
│  │  ─────────────────────────────────────────────────────────  │   │
│  │  webdav_adapter     backup_api      ui_web_backup           │   │
│  │  (dav-server)       (REST interne)  (axum + askama HTMX)    │   │
│  └─────────────┬──────────────┬──────────────┬──────────────────┘   │
│                │              │              │                       │
│  ┌─────────────┴──────────────┴──────────────┴──────────────────┐   │
│  │  COUCHE 2 — OPÉRATEURS                                       │   │
│  │  ─────────────────────────────────────────────────────────  │   │
│  │  files_op      snapshots_op    restore_op   share_op   auth_op│  │
│  │  (storage)     (politique)     (récupération)(liens)  (KindM)│  │
│  └─────────────┬──────────────┬──────────────┬───────────────────┘   │
│                │              │              │                       │
│  ┌─────────────┴──────────────┴──────────────┴──────────────────┐   │
│  │  COUCHE 3 — KITS (transversaux)                              │   │
│  │  ─────────────────────────────────────────────────────────  │   │
│  │  cas_kit       crypto_kit     dav_xml_kit   token_kit       │   │
│  │  (content-addr (chacha20      (quick-xml)   (jwt-simple)    │   │
│  │   storage)     poly1305)                                    │   │
│  │                                                              │   │
│  │  storage_kit                                                 │   │
│  │  (libSQL chiffré)                                            │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                            ↓                                         │
│             ┌──────────────────────────────────────┐                │
│             │  KindMother (identité, clés)         │                │
│             └──────────────────────────────────────┘                │
└─────────────────────────────────────────────────────────────────────┘

(Pas de consommation de jaykoa-client / jaycontact-client / jaymail-client.
 JayCloud est strictement autonome.)
```

### 1.2 Couches détaillées

#### Couche 1 — Adaptateurs

| Adaptateur | Fichier | Rôle |
|------------|---------|------|
| `webdav_adapter` | `src/adapters/webdav.rs` | WebDAV (RFC 4918) via `dav-server`. |
| `backup_api` | `src/adapters/api.rs` | REST JSON interne pour Central + UI (déclencher backup, lancer restore, lister snapshots, gérer cibles). |
| `ui_web_backup` | `src/adapters/ui.rs` | Pages HTML minimalistes (askama + HTMX) : liste snapshots, déclencher backup, restore, partages. |

#### Couche 2 — Opérateurs

| Opérateur | Détient | Responsabilités |
|-----------|---------|-----------------|
| `files_op` | ✅ filesystem chiffré | Lecture / écriture des fichiers, héritier MiyuCloud. |
| `snapshots_op` | ✅ index libSQL | Création de snapshots (complets ou incrémentaux), politique de rétention, listing. |
| `restore_op` | ❌ orchestrateur | Récupération de fichiers depuis un snapshot vers un emplacement choisi. |
| `share_op` | ✅ libSQL | Liens publics signés (expiration, mot de passe), redirection 308 ex-MiyuCloud. |
| `auth_op` | ✅ sessions + tokens | Sessions web, jetons applicatifs WebDAV (KindMother pour les clés). |

#### Couche 3 — Kits

| Kit | Wrapper sur | Rôle |
|-----|-------------|------|
| `cas_kit` | filesystem + libSQL | Content-Addressed Storage : blobs adressés par leur SHA-256, dédup naturelle. |
| `crypto_kit` | `chacha20poly1305` | Chiffrement de blocs au repos, clés dérivées KindMother. |
| `dav_xml_kit` | `quick-xml` | Templates XML DAV (multistatus, propfind). |
| `token_kit` | `jwt-simple` | Génération / vérification / révocation jetons applicatifs. |
| `storage_kit` | `libsql` chiffré | Wrapper persistance (sessions, app_passwords, share_links, snapshots index, redirects). |

---

## 2. Sélection des bibliothèques opensource

### 2.1 Matrice

| Lib | Version cible | Licence | Justification |
|-----|---------------|---------|---------------|
| **`dav-server`** | ^0.11 | Apache-2.0 | Fondation WebDAV. RFC 4918, Litmus pass, pluggable filesystem trait. |
| **`axum`** | workspace | MIT | Standard du workspace. |
| **`tokio`** | workspace | MIT | Standard du workspace. |
| **`quick-xml`** | ^0.36 | MIT | Parse/render XML DAV. |
| **`jwt-simple`** | ^0.12 | ISC | Jetons applicatifs WebDAV. |
| **`chacha20poly1305`** | workspace | MIT/Apache-2.0 | Chiffrement de blocs (CAS + métadonnées sensibles). |
| **`libsql`** | workspace | MIT | Persistance chiffrée (index snapshots, sessions, etc.). |
| **`askama`** | ^0.12 | MIT/Apache-2.0 | Templates HTML compilés (UI web minimaliste). |
| **`tower-sessions`** | ^0.13 | MIT | Sessions web du portail. |
| **`sha2`** | workspace | MIT/Apache-2.0 | Checksums SHA-256 (intégrité, CAS). |
| **`zstd`** | ^0.13 | MIT/Apache-2.0 | Compression optionnelle des blobs CAS. |
| **`tracing`** | workspace | MIT | Logs structurés. |

### 2.2 Bibliothèques rejetées

| Lib | Raison |
|-----|--------|
| `rustical` | AGPL-3.0 viral. Et hors scope (sauf si on faisait du CalDAV — ce qu'on ne fait pas, DT-02). |
| `async-imap` / `lettre` / `mail-parser` | Hors scope (mail = JayMail, DT-04). |
| `icalendar` / `vcard4` | Hors scope (CalDAV/CardDAV exclus, DT-02). |
| Implémenter WebDAV from scratch | 6 mois de plomberie inutile alors que `dav-server` est mature. |

---

## 3. Modules (crates) à produire

### 3.1 Liste

| Crate | Type | Rôle |
|-------|------|------|
| `crates/jaycloud/` | binary + lib | Service JayCloud (Adaptateurs + Opérateurs + Kits). Sert portail HTTPS + WebDAV. |
| `crates/jaycloud-client/` | lib | Adaptateur `ServiceClient` pour Alicia (intents `trigger_backup`, `list_snapshots`, `restore_file`, `share_file`, `revoke_app_password`). |
| `crates/jaycloud-migrate/` | binary | Outil one-shot de migration MiyuCloud → JayCloud. |

### 3.2 Structure interne de `crates/jaycloud/`

```
crates/jaycloud/
├── Cargo.toml
├── service.manifest.json
├── README.md
├── src/
│   ├── lib.rs                          # exposition pub
│   ├── main.rs                         # bootstrap axum + DAV
│   ├── config.rs                       # JayCloudConfig
│   ├── adapters/
│   │   ├── mod.rs
│   │   ├── webdav.rs                   # dav-server → files_op
│   │   ├── api.rs                      # REST interne (backup/restore/snapshots/shares)
│   │   ├── ui.rs                       # routes UI web minimaliste
│   │   └── auth_routes.rs              # login + app-passwords + révocation
│   ├── operators/
│   │   ├── mod.rs
│   │   ├── files_op.rs                 # héritier MiyuCloud
│   │   ├── snapshots_op.rs             # création + rétention + listing
│   │   ├── restore_op.rs               # récupération sélective
│   │   ├── share_op.rs                 # liens publics + redirections legacy
│   │   └── auth_op.rs                  # sessions + tokens
│   ├── kits/
│   │   ├── mod.rs
│   │   ├── cas_kit.rs                  # content-addressed storage
│   │   ├── crypto_kit.rs               # chacha20poly1305
│   │   ├── dav_xml_kit.rs              # templates XML DAV
│   │   ├── token_kit.rs                # JWT-like
│   │   └── storage_kit.rs              # libSQL chiffré
│   ├── files/                          # ex-MiyuCloud (DT-07)
│   │   ├── mod.rs
│   │   ├── davfs.rs                    # impl DavFileSystem
│   │   ├── tree.rs                     # arborescence virtuelle
│   │   └── encryption.rs               # chiffrement bloc
│   └── errors.rs
└── templates/
    ├── snapshots/
    │   ├── list.html
    │   └── detail.html
    ├── backup/
    │   └── trigger.html
    ├── restore/
    │   └── pick.html
    ├── shares/
    │   └── list.html
    └── auth/
        ├── login.html
        └── app_passwords.html
```

### 3.3 Structure de `crates/jaycloud-client/`

Réplique du pattern `jaykoa-client` / `jaycontact-client` :

```
crates/jaycloud-client/
├── Cargo.toml
└── src/
    └── lib.rs
        # JayCloudBackend trait
        # SnapshotSummary / ShareLink / AppPassword DTOs
        # FakeJayCloudBackend
        # JayCloudClient (impl ServiceClient)
```

Intents exposés à Alicia :

| Intent | Type | Confirmation |
|--------|------|--------------|
| `trigger_backup(target_name?)` | write | `always_for_writes` |
| `list_snapshots(target_name?, limit?)` | read | `never` |
| `restore_file(snapshot_id, file_path, destination?)` | write | `always_for_writes` |
| `share_file(file_path, expires_in?, password?)` | write | `always_for_writes` |
| `list_app_passwords` | read | `never` |
| `revoke_app_password(name)` | write | `always_for_writes` |

### 3.4 Workspace Cargo.toml

```toml
    # === JayCloud (sauvegarde cloud souveraine) ===
    "crates/jaycloud",
    "crates/jaycloud-client",
    "crates/jaycloud-migrate",
```

---

## 4. Contrats d'intégration

### 4.1 JayCloud ↔ KindMother

| Usage | API KindMother |
|-------|----------------|
| Authentification | `verify_identity(credentials) -> CogIdentity` |
| Dérivation clé chiffrement | `derive_key("jaycloud_files_v1") -> Key` |
| Signature liens publics | `sign(payload) -> Signature` |
| Vérification jeton applicatif | `verify_signature(token, sig) -> bool` |

### 4.2 JayCloud ↔ services Jay

**Aucun contrat.** JayCloud ne consomme aucun service Jay (DT-04).

Si l'utilisateur veut sauvegarder son fichier `jaykonta.db`, il
configure une **cible de backup** pointant vers le chemin du fichier.
JayCloud ne sait pas ce qu'est JayKonta, il sait juste sauvegarder un
fichier.

### 4.3 JayCloud ↔ MiyukiniNotify (optionnel)

Émission d'événements de notification :

| Événement | Niveau |
|-----------|--------|
| `backup_started(target_name)` | info |
| `backup_completed(target_name, snapshot_id, files_count, size_bytes)` | info |
| `backup_failed(target_name, reason)` | warning |
| `restore_started(snapshot_id, files_count)` | info |
| `restore_completed(snapshot_id, files_count)` | info |
| `integrity_check_failed(snapshot_id, corrupted_files)` | error |

---

## 5. Schémas de données (libSQL chiffré JayCloud)

### 5.1 Tables

```sql
-- Sessions web du portail
CREATE TABLE sessions (
    id            TEXT PRIMARY KEY,
    user_id       TEXT NOT NULL,
    created_at    INTEGER NOT NULL,
    expires_at    INTEGER NOT NULL,
    user_agent    TEXT,
    ip_hash       TEXT
);

-- Jetons applicatifs WebDAV (app-passwords)
CREATE TABLE app_passwords (
    id            TEXT PRIMARY KEY,
    user_id       TEXT NOT NULL,
    name          TEXT NOT NULL,                -- "rclone", "Duplicati", etc.
    token_hash    TEXT NOT NULL,                -- SHA256(token)
    scopes        TEXT NOT NULL,                -- JSON ["webdav","backup_api"]
    created_at    INTEGER NOT NULL,
    last_used_at  INTEGER,
    revoked_at    INTEGER
);

-- Cibles de backup (configurations nommées)
CREATE TABLE backup_targets (
    id            TEXT PRIMARY KEY,
    name          TEXT NOT NULL UNIQUE,
    source_path   TEXT NOT NULL,                -- chemin source à sauvegarder
    schedule_cron TEXT,                          -- planning (NULL = manuel uniquement)
    retention     TEXT NOT NULL,                -- JSON {daily:7, weekly:4, monthly:12}
    encryption    INTEGER NOT NULL DEFAULT 1,   -- bool
    created_at    INTEGER NOT NULL,
    last_run_at   INTEGER,
    enabled       INTEGER NOT NULL DEFAULT 1
);

-- Snapshots (index)
CREATE TABLE snapshots (
    id            TEXT PRIMARY KEY,             -- UUID
    target_id     TEXT NOT NULL,
    kind          TEXT NOT NULL,                -- "full" | "incremental"
    parent_id     TEXT,                          -- snapshot précédent si incrémental
    created_at    INTEGER NOT NULL,
    files_count   INTEGER NOT NULL,
    size_bytes    INTEGER NOT NULL,
    manifest_path TEXT NOT NULL,                -- chemin CAS du manifest
    status        TEXT NOT NULL,                -- "in_progress" | "complete" | "failed" | "corrupted"
    FOREIGN KEY (target_id) REFERENCES backup_targets(id)
);
CREATE INDEX idx_snapshots_target_date ON snapshots(target_id, created_at DESC);

-- Liens publics de partage
CREATE TABLE share_links (
    token         TEXT PRIMARY KEY,
    resource_type TEXT NOT NULL,                -- "snapshot_file" | "snapshot"
    snapshot_id   TEXT NOT NULL,
    resource_path TEXT,                          -- chemin dans le snapshot (NULL = snapshot complet)
    owner_user_id TEXT NOT NULL,
    password_hash TEXT,
    created_at    INTEGER NOT NULL,
    expires_at    INTEGER,
    download_count INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (snapshot_id) REFERENCES snapshots(id)
);

-- Redirections legacy MiyuCloud
CREATE TABLE miyucloud_redirects (
    legacy_token  TEXT PRIMARY KEY,
    new_token     TEXT NOT NULL,
    expires_at    INTEGER
);

-- Cache ETags WebDAV
CREATE TABLE dav_etags (
    resource_path TEXT PRIMARY KEY,
    etag          TEXT NOT NULL,
    last_modified INTEGER NOT NULL,
    last_seen_at  INTEGER NOT NULL
);
```

### 5.2 Stockage CAS (Content-Addressed Storage)

Les blobs (contenus de fichiers sauvegardés, manifestes de snapshots) sont
stockés dans un répertoire CAS structuré par hash :

```
~/.miyukini/jaycloud/cas/
├── ab/
│   ├── ab12cd34.../              # blob (chacha20poly1305 + nonce inline)
│   └── ab78ef90.../
└── cd/
    └── cd56...
```

Chaque blob est :
- chiffré avec une clé dérivée de KindMother,
- adressé par `SHA-256(plaintext)` (avant chiffrement) → dédup naturelle,
- compressé optionnellement avec `zstd`.

### 5.3 Manifests de snapshot

Un manifest est un fichier JSON (lui-même stocké dans le CAS) listant les
entrées du snapshot :

```json
{
  "snapshot_id": "snap_abc123",
  "target_id": "target_jaykonta",
  "created_at": 1747044000,
  "kind": "incremental",
  "parent_id": "snap_xyz789",
  "files": [
    {"path": "data/db.sqlite", "size": 1234567, "blob": "ab12cd34...", "mode": 644, "mtime": 1747043900},
    {"path": "data/logs/app.log", "size": 8192, "blob": "cd56ef78...", "mode": 644, "mtime": 1747042000}
  ]
}
```

---

## 6. Protocole WebDAV (RFC 4918)

### 6.1 Méthodes supportées en P3

| Méthode | Statut P3 |
|---------|-----------|
| `OPTIONS` | ✅ |
| `PROPFIND` | ✅ (Depth 0/1/infinity, limite 1000 entrées) |
| `PROPPATCH` | ⚠ propriétés live read-only ; dead properties stockées dans `dav_etags` |
| `GET` / `HEAD` | ✅ avec support Range |
| `PUT` | ✅ |
| `DELETE` | ✅ |
| `MKCOL` | ✅ |
| `COPY` / `MOVE` | ✅ |
| `LOCK` / `UNLOCK` | ✅ |

### 6.2 Backend `DavFileSystem` custom

`crates/jaycloud/src/files/davfs.rs` implémente `dav-server::DavFileSystem` :

- Mappe les chemins WebDAV vers le CAS via `files_op`.
- Toute écriture passe par `borderguard` (gate Cores).
- Chiffrement/déchiffrement transparent via `crypto_kit`.
- Émission d'événements MiyukiniNotify pour les écritures importantes.

### 6.3 Exposition CAS via WebDAV

Le WebDAV est utile pour deux cas :

1. **Backup tiers** (rclone, restic, Duplicati) : ces outils écrivent
   leurs propres archives dans JayCloud comme dans un cloud quelconque.
2. **Restore manuel** : l'utilisateur peut monter le WebDAV et naviguer
   dans les snapshots (chemin `/dav/snapshots/<snap_id>/...`) en
   lecture seule.

---

## 7. Workflow Backup

### 7.1 Backup ad-hoc

```
POST /api/backup/trigger { target_id: "target_jaykonta" }
                │
                ▼
       backup_api → snapshots_op::create()
                │
                ▼
       snapshots_op :
       1. Détermine kind (full / incremental)
       2. Parent_id = dernier snapshot complet ou récent
                │
                ▼
       files_op : parcourt source_path
                │
                ▼
       Pour chaque fichier :
       - Hash plaintext (SHA-256)
       - Si hash existe déjà dans CAS → réutilise (dédup)
       - Sinon : chiffre + zstd compress + store dans CAS
                │
                ▼
       Construit manifest.json → store dans CAS
                │
                ▼
       INSERT snapshots (id, target_id, kind, parent_id, manifest_path, status="complete")
                │
                ▼
       MiyukiniNotify : backup_completed(...)
```

### 7.2 Backup planifié

Worker tokio interne lit `backup_targets WHERE schedule_cron IS NOT NULL`
toutes les minutes ; déclenche les backups dus selon leur expression
cron. Pas de dépendance externe (pas de systemd timer, pas de cron OS).

### 7.3 Politique de rétention

À chaque backup réussi, `snapshots_op::apply_retention(target_id)` :

1. Lit `retention` JSON (ex: `{daily:7, weekly:4, monthly:12}`).
2. Pour chaque snapshot existant, détermine s'il est conservé (par
   bucket calendaire).
3. Supprime les snapshots non conservés.
4. Garbage-collect les blobs CAS orphelins (référencés par aucun
   manifest restant).

---

## 8. Workflow Restore

### 8.1 Restore d'un fichier

```
POST /api/restore { snapshot_id, file_path, destination?: "..." }
                │
                ▼
       restore_op → snapshots_op::open(snapshot_id)
                │
                ▼
       Lit manifest.json depuis le CAS
                │
                ▼
       Trouve l'entrée file_path → blob hash
                │
                ▼
       cas_kit::read(blob_hash) → décrypte → décompresse
                │
                ▼
       Écrit vers destination (default = source_path d'origine)
                │
                ▼
       MiyukiniNotify : restore_completed(...)
```

### 8.2 Restore complet

Iter sur toutes les entrées du manifest, restaure chacune.

### 8.3 Restore-as-snapshot (point in time)

Si l'utilisateur veut une copie complète d'un snapshot vers un nouvel
emplacement, restore_op le traite comme un restore complet vers
`destination`.

---

## 9. Authentification & jetons applicatifs

### 9.1 Authentification web

1. `https://<cog>/jaycloud/login` → délégation KindMother.
2. Session créée dans `sessions` (TTL 24h).
3. Cookie `Secure; HttpOnly; SameSite=Strict`.

### 9.2 Jetons applicatifs (app-passwords)

Format `mws-jc-<base32(random_16_bytes)>-<hmac>`.

- Stocké comme `SHA256(token)` uniquement.
- Affiché à l'utilisateur une seule fois à la création.
- Révocation = `revoked_at = NOW()`, effet immédiat.
- HTTP Basic Auth sur les endpoints WebDAV et API.

### 9.3 Intégration Alicia (P6)

Liste des intents (cf. §3.3).

---

## 10. Migration depuis MiyuCloud

### 10.1 Pipeline

```
Détection MiyuCloud
       ↓
Validation cohérence (inventaire, sha256)
       ↓
Init storage JayCloud (libSQL + CAS dir)
       ↓
Migration : import de l'arborescence comme premier snapshot full
       ├─ Hash chaque fichier MiyuCloud
       ├─ Stocke dans CAS
       └─ Construit manifest
       ↓
Insert snapshots (kind=full, target=migration, status=complete)
       ↓
Migration liens publics MiyuCloud → entries dans miyucloud_redirects
       ↓
MiyuCloud → mode lecture seule
       ↓
Fenêtre de transition (90j)
       ↓
Désinstallation MiyuCloud (MasterButler)
       ↓
docs/services/DEPRECATED.md MAJ
```

### 10.2 Catalogue services

Lors de la livraison P5, dans le même commit :

- `apps/origin/src/web/content.rs` : retirer `miyucloud`, ajouter `jaycloud`.
- `docs/services/DEPRECATED.md` : section MiyuCloud.

---

## 11. Sécurité (détails)

| Couche | Mesure |
|--------|--------|
| Transport | TLS 1.3 only, HSTS forcé. |
| Auth web | Sessions courtes, CSRF token, SameSite=Strict. |
| Auth WebDAV | App-passwords scopés, révocables, jamais stockés en clair. |
| Chiffrement au repos | libSQL chiffré + blobs CAS chiffrés (chacha20poly1305), clés dérivées KindMother. |
| Intégrité | SHA-256 à l'écriture et à la lecture ; check périodique configurable. |
| Audit | Logs structurés (auth, partages, backups, restores). |
| Rate-limit | Par IP + par jeton app. |
| Anti-CSRF | Tokens sur écritures portail. |
| Anti-injection XML | `quick-xml` mode strict. |
| Anti-path traversal | Sanitisation systématique via `files::tree::canonical_path`. |

---

## 12. Performance & cibles P3

| Métrique | Cible |
|----------|-------|
| Cold start | <1 s |
| PROPFIND Depth 1 sur 100 entrées | <200 ms |
| GET fichier 10 MB | >50 MB/s |
| Backup d'un dossier de 1 Go (full) | <30 s sur SSD local |
| Backup incrémental (0 fichier modifié) | <2 s |
| Restore d'un fichier 100 MB | <10 s |
| Mémoire au repos | <50 MB |

---

## 13. Tests de conformité

### 13.1 WebDAV — Litmus

Tests Litmus en P3 : `basic`, `copymove`, `props`, `locks`, `http`.
Intégration CI : `scripts/test-jaycloud-litmus.sh`.

### 13.2 Outils tiers réels

Matrice de validation P3 :

| Outil | Type | Test |
|-------|------|------|
| rclone | sync/backup | Push d'un dossier vers JayCloud + restore. |
| restic (WebDAV backend) | backup | Init repo + backup + restore + check. |
| Duplicati | backup | Job complet de bout en bout. |
| Cyberduck | client | Browse + upload + download. |
| Windows Explorer / macOS Finder | client OS | Monter + parcourir + lire/écrire. |

### 13.3 Tests internes E2E

- Backup full + incrémental + rétention → restore intégrale.
- Restore sélectif d'un fichier dans un snapshot ancien.
- Détection de corruption (modification arbitraire d'un blob CAS) →
  rapport d'intégrité.
- Migration MiyuCloud → vérification ETag de tous les fichiers post-migration.

---

## 14. Plan d'implémentation

### 14.1 Ordre des PRs

| PR | Périmètre | Bloque |
|----|-----------|--------|
| **PR-1 (P2)** | Skeleton crates jaycloud + jaycloud-client + jaycloud-migrate, manifest, workspace. `cargo check` vert. | Tout. |
| **PR-2 (P3.a)** | `files_op` héritier MiyuCloud + `cas_kit` + `crypto_kit` + `storage_kit` + `auth_op` minimal. | PR-3, PR-4. |
| **PR-3 (P3.b)** | `snapshots_op` + `restore_op` + `share_op` + `backup_api` + tests E2E backup/restore. | PR-4. |
| **PR-4 (P3.c)** | `webdav_adapter` (dav-server intégration) + Litmus pass + matrice outils tiers. | PR-5. |
| **PR-5 (P4)** | `ui_web_backup` (templates askama HTMX) + scheduler interne tokio pour les backups planifiés. | PR-6. |
| **PR-6 (P5)** | `crates/jaycloud-migrate` + redirections share_op + catalogue services MAJ + DEPRECATED.md. | PR-7. |
| **PR-7 (P6)** | `crates/jaycloud-client` (intents Alicia). Sauvegarde MWS inter-COG (copie distante chiffrée vers un autre COG via webway). | PR-8. |
| **PR-8 (P7)** | Polish : vérification intégrité périodique, RFC 3253 versioning, sharing extensions, compression différentielle des blobs. | — |

### 14.2 Extensions requises sur les services frères

**Aucune.** JayCloud n'extension aucune API d'aucun service Jay (DT-04).

### 14.3 Suppression progressive de MiyuCloud

| Étape | Action MiyuCloud |
|-------|------------------|
| PR-1 → PR-4 | MiyuCloud reste tel quel. JayCloud cohabite. |
| PR-2 (P3.a) | Code MiyuCloud dupliqué dans `crates/jaycloud/src/files/`. MiyuCloud original conservé. |
| PR-6 (P5) | MiyuCloud passe en lecture seule. Catalogue retiré. |
| Après transition (90j) | `crates/miyucloud` et `crates/miyucloud-dav` supprimés du workspace. |

---

## 15. Risques techniques

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Volume MiyuCloud >100 Go à migrer | moyenne | moyen | Migration streamée avec reprise sur erreur ; hash incrémental pour reprendre depuis le dernier blob OK. |
| Corruption silencieuse de blobs CAS anciens | faible | élevé | Vérif périodique SHA-256 + notif utilisateur ; redondance configurable. |
| Rétention mal configurée → perte de données | moyenne | élevé | Valeurs par défaut conservatrices ; confirmation explicite avant suppression de snapshot ; possibilité de marquer un snapshot "ne pas supprimer". |
| Outils WebDAV exotiques mal supportés | faible | faible | Matrice officielle (rclone, restic, Duplicati, Cyberduck, Windows, macOS). |
| Limite mémoire sur PROPFIND profond | faible | élevé | Limite Depth infinity = 1000 entrées. |
| Bug RFC dans `dav-server` (rare) | faible | élevé | Pin version + tests Litmus à chaque MAJ. |
| Saturation espace disque CAS (snapshots accumulés) | élevée | moyen | Rétention bien configurée + alertes seuil + GC blobs orphelins. |
| Conflit d'écriture WebDAV concurrent | moyenne | moyen | LOCK/UNLOCK supportés ; sinon ETags + If-Match. |

---

## Annexes

### A. Mapping RFC ↔ Module

| RFC | Module |
|-----|--------|
| RFC 4918 (WebDAV) | `adapters/webdav.rs` + `dav-server` |
| RFC 3253 (Versioning WebDAV) | Reporté P7. |
| RFC 4226 / 6238 (HOTP/TOTP) | Délégué à KindMother (MFA P6). |

### B. Glossaire MSCM JayCloud

| Terme | Définition |
|-------|------------|
| **Backup** | Copie chiffrée à un instant T d'un fichier ou dossier, stockée dans le CAS. |
| **Snapshot** | Backup nommé avec manifest et politique de rétention. Complet ou incrémental. |
| **Restore** | Récupération depuis un snapshot vers un emplacement choisi. |
| **CAS** | Content-Addressed Storage — stockage par hash de contenu, dédup naturelle. |
| **Cible de backup** | Config nommée (source, planning, rétention). |
| **App-password** | Jeton applicatif révocable pour outils WebDAV tiers. |
| **Fenêtre de transition** | Période (90j) pendant laquelle MiyuCloud reste en lecture seule. |
| **Adaptateur** | Couche 1 — protocole exposé (WebDAV, API REST, UI). |
| **Opérateur** | Couche 2 — logique métier (files, snapshots, restore, share, auth). |
| **Kit** | Couche 3 — bibliothèque interne (CAS, crypto, XML DAV, tokens, storage). |

### C. Variables de configuration

| Clé | Type | Défaut | Rôle |
|-----|------|--------|------|
| `JAYCLOUD_STORAGE_PATH` | string | `~/.miyukini/jaycloud/` | Racine storage. |
| `JAYCLOUD_HTTP_PORT` | int | 8443 | Port HTTPS portail. |
| `JAYCLOUD_DAV_DEPTH_LIMIT` | int | 1000 | Limite PROPFIND infinity. |
| `JAYCLOUD_SESSION_TTL_SECONDS` | int | 86400 | TTL session web. |
| `JAYCLOUD_LOCK_TTL_SECONDS` | int | 3600 | TTL DAV LOCK. |
| `JAYCLOUD_SHARE_LINK_DEFAULT_EXPIRY_DAYS` | int | 30 | Expiration par défaut liens. |
| `JAYCLOUD_MIYUCLOUD_TRANSITION_DAYS` | int | 90 | Fenêtre de transition. |
| `JAYCLOUD_RATE_LIMIT_PER_TOKEN_PER_MIN` | int | 600 | Rate limit par app-password. |
| `JAYCLOUD_DEFAULT_RETENTION` | string | `{daily:7,weekly:4,monthly:12}` | Politique de rétention par défaut. |
| `JAYCLOUD_INTEGRITY_CHECK_CRON` | string | `0 4 * * 0` | Planning vérif intégrité (par défaut dimanche 4h). |
| `JAYCLOUD_CAS_COMPRESSION_LEVEL` | int | 3 | Niveau zstd (0 = pas de compression). |

### D. Dette technique acceptée P0→P4

| Item | Reporté à | Justification |
|------|-----------|---------------|
| Sauvegarde MWS inter-COG (copie distante vers un autre COG) | P6 | Périmètre Alicia + MWS plus large. |
| Versioning WebDAV RFC 3253 | P7 | snapshots_op interne suffit. |
| Sharing WebDAV extensions | P7 | Liens publics propres suffisent. |
| MFA / TOTP login portail | P6 | Sessions web suffisent en P0. |
| Compression différentielle entre blobs proches | P7 | zstd block-level suffit en P0. |

---

> **Prochaine étape MIP** : sur validation de cette spec, P2 = skeleton
> crates (`crates/jaycloud/`, `crates/jaycloud-client/`,
> `crates/jaycloud-migrate/`) avec `Cargo.toml`, `service.manifest.json`,
> entrées workspace, `cargo check` vert. Aucune fonctionnalité — juste
> le squelette.
