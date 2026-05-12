# JayCloud — Spec MSCM / MIP Conformité

> Document **P1** du protocole MIP. Prérequis : *JayCloud - Document
> Fondateur.md* validé (DS-01 → DS-06).

## 0. Cadre / Méthode

### 0.1 Position dans le protocole MIP

| Étape | Livrable | État |
|-------|----------|------|
| P0 | *Document Fondateur* | ✅ validé (commit `dad5dd3a`) |
| **P1** | **Ce document — Spec MSCM / MIP Conformité** | 🟢 en cours |
| P2 | Skeleton crates + manifest + workspace entry | à venir |
| P3 | Drive web + WebDAV (Litmus pass) | à venir |
| P4 | CalDAV + CardDAV (clients DAVx⁵ / Thunderbird / Apple) | à venir |
| P5 | Webmail | à venir |
| P6 | Migration MiyuCloud | à venir |
| P7 | Intégration Alicia (capacités exposées) | à venir |
| P8 | Polish & extensions | à venir |

### 0.2 Périmètre du document

| In scope | Hors scope |
|----------|------------|
| Architecture en couches (Opérateurs / Kits / Adaptateurs) | Implémentation détaillée des handlers (les crates les détaillent) |
| Choix de bibliothèques opensource (matrice + licences) | Choix d'UI (HTML/HTMX) — référencés par *JayCloud - UI Web Portail* (futur P3) |
| Contrats d'API entre JayCloud et les services consommés (Files, JayKoa, JayContact, JayMail) | Renégociation des Docs Fondateurs des services consommés |
| Schémas de persistance JayCloud-propre (libSQL chiffré) | Schémas internes des services consommés |
| Plan d'implémentation crate par crate avec dépendances ordonnées | Roadmap de jalons / dates calendaires |
| Tests de conformité (Litmus, CalDAV/CardDAV CTS) | Procédures CI / pipelines (futur *JayCloud - Ops*) |

### 0.3 Contraintes absolues

| Contrainte | Description |
|------------|-------------|
| ❌ **Ne pas dupliquer les sources de vérité** | Aucune donnée n'est stockée en double dans JayCloud quand un service consommé la détient déjà. JayCloud ne stocke en propre que : sessions, jetons applicatifs, métadonnées de partage publique, vues matérialisées de cache. |
| ❌ **Ne pas dépendre d'une lib AGPL** | La double licence Miyukini est incompatible avec AGPL viral. RustiCal est référence d'architecture, pas dépendance. |
| ❌ **Ne pas exposer de protocole sans test de conformité** | Tout protocole DAV livré doit passer le test suite de référence (Litmus pour WebDAV, CalDAVTester pour CalDAV/CardDAV). |
| ✅ **Source de vérité** | Ce document est la **référence** pour l'architecture technique, les choix de bibliothèques et les contrats d'intégration. |

### 0.4 Décisions structurantes (mini log)

| Id | Décision | Justification |
|----|----------|---------------|
| **DT-01** | **`dav-server` 0.11.0 (Apache-2.0) comme base WebDAV.** | Mature (Litmus pass, RFC 4918), pluggable via `DavFileSystem` / `GuardedFileSystem`, compatible Axum. Évite de réécrire 6 mois de plomberie HTTP/XML. |
| **DT-02** | **CalDAV/CardDAV construits AU-DESSUS de `dav-server` avec backends custom.** | RustiCal est AGPL-3.0 (viral) et standalone-binary, non utilisable comme lib. On réutilise `dav-server` pour les méthodes communes (PROPFIND, REPORT) et on implémente les *reports* spécifiques CalDAV (calendar-query) et CardDAV (addressbook-query) en propre. |
| **DT-03** | **iCalendar via crate `icalendar` (Apache-2.0/MIT) ; vCard via `vcard4` (vérifier licence, sinon fork interne).** | Évite l'écriture d'un parser RFC 5545 / 6350 complet (chronophage et risqué). |
| **DT-04** | **Webmail via `async-imap` + `lettre` + `mail-parser` (tous Apache-2.0/MIT).** | Stack Rust mature, soutenue (Stalwart Labs pour mail-parser). |
| **DT-05** | **Architecture MSCM en 3 couches : Adaptateurs (protocoles) → Opérateurs (logique métier) → Kits (transversaux).** | Cohérent avec les autres services Miyukini (cf. JayKonta, JayKoa). Permet réutilisation des Kits dans les services frères. |
| **DT-06** | **Persistance JayCloud-propre via libSQL chiffré ; aucun moteur DB externe.** | Cohérent avec KindMother + le reste du COG. Permet sauvegarde / restauration unitaire. |
| **DT-07** | **Authentification = délégation à KindMother + jetons applicatifs JWT-like signés.** | Évite stockage de mots de passe ; chaque appareil DAV reçoit un app-password révocable scopé. |
| **DT-08** | **`Files Opérateur` est *intégré* à JayCloud (pas un crate séparé) — il est l'héritier direct de MiyuCloud.** | Évite la double couche d'indirection ; MiyuCloud étant retiré, son code est rapatrié dans `crates/jaycloud/src/files/`. |
| **DT-09** | **JayKoa / JayContact / JayMail restent autonomes ; JayCloud les appelle via leurs *clients* existants (jaykoa-client, jaycontact-client, jaymail-client).** | Cohérent avec l'architecture Alicia : les clients sont des adaptateurs `ServiceClient`. JayCloud devient un consommateur de plus, sans coupling fort. |
| **DT-10** | **Pas de cache de second niveau en P3-P5.** | Les services consommés ont leurs propres caches (KindMother, libSQL). Ajouter du cache JayCloud → risque d'incohérence. Évalué en P8 si nécessaire. |

---

## 1. Architecture en couches

### 1.1 Vue globale

```
┌────────────────────────────────────────────────────────────────────────┐
│                      JayCloud (crates/jaycloud)                        │
│                                                                        │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  COUCHE 1 — ADAPTATEURS (protocoles exposés)                     │  │
│  │  ──────────────────────────────────────────────────────────────  │  │
│  │  webdav_adapter   caldav_adapter   carddav_adapter   webmail_ui  │  │
│  │  (dav-server)     (custom on       (custom on       (axum +      │  │
│  │                    dav-server)      dav-server)      HTMX)       │  │
│  └──────────┬───────────────┬───────────────┬──────────┬────────────┘  │
│             │               │               │          │               │
│  ┌──────────┴───────────────┴───────────────┴──────────┴────────────┐  │
│  │  COUCHE 2 — OPÉRATEURS (logique métier)                          │  │
│  │  ──────────────────────────────────────────────────────────────  │  │
│  │  files_op  calendar_op  contact_op   mail_op   share_op   auth_op│  │
│  │  (intern)  (→jaykoa)    (→jaycontact) (→jaymail) (intern) (intern)│ │
│  └──────────┬───────────────┬───────────────┬──────────┬────────────┘  │
│             │               │               │          │               │
│  ┌──────────┴───────────────┴───────────────┴──────────┴────────────┐  │
│  │  COUCHE 3 — KITS (transversaux)                                  │  │
│  │  ──────────────────────────────────────────────────────────────  │  │
│  │  ical_kit    vcard_kit    dav_xml_kit   token_kit    storage_kit │  │
│  │  (icalendar) (vcard4)     (quick-xml)   (jwt-simple) (libSQL)    │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                            ↓                                           │
│             ┌──────────────────────────────────────┐                   │
│             │  KindMother (identité, clés, sqlite) │                   │
│             └──────────────────────────────────────┘                   │
└────────────────────────────────────────────────────────────────────────┘
            ↓ (clients ServiceClient pour les services frères)
       ┌────────────┐ ┌────────────┐ ┌──────────────┐
       │ jaykoa-    │ │ jaycontact-│ │ jaymail-     │
       │ client     │ │ client     │ │ client       │
       └────────────┘ └────────────┘ └──────────────┘
```

### 1.2 Couches détaillées

#### Couche 1 — Adaptateurs (protocoles)

Chaque adaptateur **expose un protocole standard** vers l'extérieur et
**délègue toute logique métier** à un ou plusieurs Opérateurs. Le code
spécifique au protocole (parsing XML DAV, construction des réponses
multistatus, gestion des locks, etc.) vit ici.

| Adaptateur | Fichier | RFC ciblées | Lib base |
|------------|---------|-------------|----------|
| `webdav_adapter` | `src/adapters/webdav.rs` | 4918 (+ 3253 en P8) | `dav-server` |
| `caldav_adapter` | `src/adapters/caldav.rs` | 4791 | `dav-server` (méthodes communes) + impl custom des reports |
| `carddav_adapter` | `src/adapters/carddav.rs` | 6352 | idem |
| `webmail_ui` | `src/adapters/webmail.rs` | — (HTML/HTMX) | axum + askama |

#### Couche 2 — Opérateurs (logique métier)

Un Opérateur **détient ou orchestre** une logique métier identifiable.
Soit il **détient** sa donnée en propre (cas `files_op`, `share_op`,
`auth_op`), soit il **délègue** à un service Miyukini frère via son
client (`calendar_op → jaykoa-client`, `contact_op → jaycontact-client`,
`mail_op → jaymail-client`).

| Opérateur | Détient ? | Délègue à | Responsabilités |
|-----------|-----------|-----------|-----------------|
| `files_op` | ✅ oui | — | Arborescence fichiers, versionning, corbeille, héritier MiyuCloud. |
| `calendar_op` | ❌ non | `jaykoa-client` | Mapping CalDAV ↔ JayKoa, gestion ETags, calendars-query. |
| `contact_op` | ❌ non | `jaycontact-client` | Mapping CardDAV ↔ JayContact, gestion ETags, addressbook-query. |
| `mail_op` | ❌ non | `jaymail-client` | Façade webmail vers JayMail (lecture, écriture, recherche). |
| `share_op` | ✅ oui | — | Liens publics signés, expirations, mots de passe, redirection 308 ex-MiyuCloud. |
| `auth_op` | ✅ oui | KindMother (clés) | Sessions web, jetons applicatifs DAV, scoping par appareil, révocation. |

#### Couche 3 — Kits (transversaux)

Un Kit est une **bibliothèque interne réutilisable** sans état (pure fns
ou state local). Pas de dépendance circulaire entre Kits.

| Kit | Wrapper sur | Rôle |
|-----|-------------|------|
| `ical_kit` | `icalendar` | Parse/render iCalendar, conversion `iCal Event ↔ jaykoa::EventSummary`. |
| `vcard_kit` | `vcard4` ou fork interne | Parse/render vCard 4.0, conversion `vCard ↔ jaycontact::ContactSummary`. |
| `dav_xml_kit` | `quick-xml` | Templates XML DAV (multistatus, propfind, calendar-query, addressbook-query). |
| `token_kit` | `jwt-simple` ou `jose-jwt` | Génération / vérification / révocation des jetons applicatifs. |
| `storage_kit` | `libsql` chiffré + KindMother | Wrapper persistance JayCloud (sessions, tokens, partages, métadonnées Files). |

---

## 2. Sélection des bibliothèques opensource

### 2.1 Matrice complète

| Lib | Version cible | Licence | Compatible Miyukini ? | Justification |
|-----|---------------|---------|-----------------------|---------------|
| **`dav-server`** | ^0.11 | Apache-2.0 | ✅ | Fondation WebDAV. RFC 4918, Litmus pass, pluggable filesystem. |
| **`axum`** | workspace | MIT | ✅ | Déjà standard du workspace. |
| **`tokio`** | workspace | MIT | ✅ | Déjà standard. |
| **`icalendar`** | ^0.16 | Apache-2.0/MIT | ✅ | iCalendar (RFC 5545) parse + render. |
| **`vcard4`** | ^0.7 | Apache-2.0/MIT | ✅ (à confirmer en P2) | vCard 4.0 (RFC 6350). Sinon fork interne. |
| **`quick-xml`** | ^0.36 | MIT | ✅ | Parse/render XML DAV (PROPFIND multistatus, etc.). |
| **`jwt-simple`** | ^0.12 | ISC | ✅ | Jetons applicatifs DAV. |
| **`async-imap`** | ^0.10 | Apache-2.0/MIT | ✅ | Client IMAP pour webmail. |
| **`lettre`** | ^0.11 | MIT/Apache-2.0 | ✅ | Client SMTP pour webmail. |
| **`mail-parser`** | ^0.10 | Apache-2.0 | ✅ | MIME parsing (RFC 5322). Maintenu par Stalwart. |
| **`libsql`** | workspace | MIT | ✅ | Persistance chiffrée (cf. kindmother-db-key). |
| **`askama`** | ^0.12 | MIT/Apache-2.0 | ✅ | Templates HTML compilés pour le portail Drive et Webmail. |
| **`tower-sessions`** | ^0.13 | MIT | ✅ | Sessions web pour le portail. |
| **`reqwest`** | workspace | Apache-2.0/MIT | ✅ | Appels HTTP sortants (jamais sur chemin critique en P0). |
| **`tracing`** | workspace | MIT | ✅ | Logs structurés. |

### 2.2 Référence d'architecture (non-link)

| Projet | Licence | Statut | Usage |
|--------|---------|--------|-------|
| **RustiCal** | AGPL-3.0 | non lié | Référence pour l'organisation CalDAV/CardDAV au-dessus de `dav-server`, les chemins URL DAV (`/dav/calendars/{user}/{cal}/`, `/dav/addressbooks/{user}/{book}/`), les principal-URLs et current-user-principal. Code recopié = ❌. |
| **Radicale** (Python) | GPL-3.0 | non lié | Référence pour la robustesse face aux clients tordus (iOS, Outlook). |
| **SabreDAV** (PHP) | New BSD | non lié | Référence canonique pour l'interprétation des RFC DAV (test suite, edge cases). |

### 2.3 Bibliothèques rejetées

| Lib | Raison |
|-----|--------|
| `rustical` (en tant que lib) | AGPL-3.0 viral. |
| `rust-icalendar` antérieur à `icalendar` v0.16 | Obsolète, non maintenu. |
| Implémenter WebDAV from scratch | 6 mois de plomberie, risque de bugs RFC, pas notre cœur de métier. |
| `kalk` / autres CalDAV expérimentaux | Non maintenus, non test-couverts. |

---

## 3. Modules (crates) à produire

### 3.1 Liste des crates JayCloud

| Crate | Type | Rôle | Workspace entry |
|-------|------|------|-----------------|
| `crates/jaycloud/` | binary + lib | Le service JayCloud lui-même (Adaptateurs + Opérateurs + Kits internes). Sert le portail HTTPS et les protocoles DAV. | `crates/jaycloud` |
| `crates/jaycloud-client/` | lib | Adaptateur `ServiceClient` pour Alicia (intents : `share_file`, `list_recent_uploads`, `revoke_app_password`, etc.). | `crates/jaycloud-client` |
| `crates/jaycloud-migrate/` | binary | Outil one-shot de migration depuis MiyuCloud (rapatrie l'arborescence, génère les redirections de liens publics). Optionnel, peut être un sous-binaire de jaycloud. | `crates/jaycloud-migrate` |

### 3.2 Structure interne de `crates/jaycloud/`

```
crates/jaycloud/
├── Cargo.toml
├── service.manifest.json
├── README.md
├── src/
│   ├── lib.rs                          # exposition pub des Opérateurs réutilisables
│   ├── main.rs                         # binaire de service (bootstrap axum + DAV)
│   ├── config.rs                       # JayCloudConfig (chemin storage, ports, etc.)
│   ├── adapters/
│   │   ├── mod.rs
│   │   ├── webdav.rs                   # adapter dav-server → files_op
│   │   ├── caldav.rs                   # adapter DAV → calendar_op
│   │   ├── carddav.rs                  # adapter DAV → contact_op
│   │   ├── webmail.rs                  # routes axum portail webmail
│   │   ├── drive_web.rs                # routes axum portail Drive (HTMX)
│   │   └── auth_routes.rs              # login / app-password / révocation
│   ├── operators/
│   │   ├── mod.rs
│   │   ├── files_op.rs                 # héritier MiyuCloud
│   │   ├── calendar_op.rs              # via jaykoa-client
│   │   ├── contact_op.rs               # via jaycontact-client
│   │   ├── mail_op.rs                  # via jaymail-client + async-imap/lettre
│   │   ├── share_op.rs                 # liens publics + redirections MiyuCloud
│   │   └── auth_op.rs                  # KindMother + jetons applicatifs
│   ├── kits/
│   │   ├── mod.rs
│   │   ├── ical_kit.rs                 # wrap icalendar
│   │   ├── vcard_kit.rs                # wrap vcard4
│   │   ├── dav_xml_kit.rs              # templates XML DAV
│   │   ├── token_kit.rs                # JWT-like jetons applicatifs
│   │   └── storage_kit.rs              # libSQL chiffré
│   ├── files/                          # ex-MiyuCloud (DT-08)
│   │   ├── mod.rs
│   │   ├── davfs.rs                    # impl DavFileSystem pour dav-server
│   │   ├── tree.rs                     # arborescence virtuelle
│   │   ├── versioning.rs               # versions / corbeille
│   │   └── encryption.rs               # chiffrement au repos
│   └── errors.rs                       # JayCloudError unifié
└── templates/                          # askama
    ├── drive/
    │   ├── index.html
    │   ├── folder.html
    │   └── share.html
    ├── webmail/
    │   ├── inbox.html
    │   └── compose.html
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
        # ShareLink / AppPassword DTOs
        # FakeJayCloudBackend
        # JayCloudClient (impl ServiceClient)
```

Intents exposés à Alicia :

| Intent | Type | Confirmation |
|--------|------|--------------|
| `share_file` | write | `always_for_writes` |
| `list_recent_uploads` | read | `never` |
| `list_app_passwords` | read | `never` |
| `revoke_app_password` | write | `always_for_writes` |
| `find_file_by_name` | read | `never` |

### 3.4 Workspace Cargo.toml

Section à insérer (après `crates/jaycontact-client`) :

```toml
    # === JayCloud (cloud souverain, fork Nextcloud) ===
    "crates/jaycloud",
    "crates/jaycloud-client",
    "crates/jaycloud-migrate",
```

---

## 4. Contrats d'intégration avec les services Miyukini

### 4.1 JayCloud ↔ Files (intégré)

Pas de contrat externe : `files_op` est interne à `crates/jaycloud`,
héritier direct de MiyuCloud.

### 4.2 JayCloud ↔ JayKoa (via jaykoa-client)

| Direction | Méthode | Mapping |
|-----------|---------|---------|
| Read | `JayKoaBackend::list_events(from, to)` | `iCal VEVENT` × N → réponse CalDAV `calendar-data` × N. |
| Read | `JayKoaBackend::next_event()` | Pas exposé directement (CalDAV est range-based). |
| Write (P4) | extension `JayKoaBackend::create_event(EventSummary)` | À ajouter à `jaykoa-client`. Le CalDAV `PUT` d'un iCal → `EventSummary` → JayKoa. |
| Write (P4) | extension `JayKoaBackend::update_event(id, EventSummary)` | À ajouter. |
| Write (P4) | extension `JayKoaBackend::delete_event(id)` | À ajouter. |
| Metadata | `ETag` calculé par JayCloud (hash stable du `EventSummary`) | Stocké dans `storage_kit` pour invalidation rapide. |

**Décision** : `jaykoa-client` doit être étendu avec create/update/delete
events en P4. Ces extensions sont mineures (3 méthodes du trait, 3 fake
impls).

### 4.3 JayCloud ↔ JayContact (via jaycontact-client)

Symétrique à JayKoa, avec ajout de :

| Direction | Méthode existante | À étendre ? |
|-----------|-------------------|--------------|
| Read | `JayContactBackend::list_contacts(kind, limit)` | ✅ déjà OK |
| Read | `JayContactBackend::find_contact(query)` | ✅ déjà OK |
| Write | `JayContactBackend::add_contact(id, name, kind)` | ✅ déjà OK |
| Write | `JayContactBackend::remove_contact(id)` | ✅ déjà OK |
| Write | `JayContactBackend::update_contact(id, ...)` | ❌ **à ajouter en P4** |
| Metadata | `ETag` calculé par JayCloud | — |

### 4.4 JayCloud ↔ JayMail (via jaymail-client)

Plus dense car le webmail expose une interface complète.

| Endpoint webmail | Méthode jaymail-client | À ajouter ? |
|------------------|------------------------|-------------|
| GET `/webmail/inbox` | `list_recent_inbox(limit, unread_only)` | ✅ déjà OK |
| GET `/webmail/search` | `search_mail(query, limit)` | ✅ déjà OK |
| GET `/webmail/message/{id}` | `get_mail(id)` | ❌ à ajouter |
| GET `/webmail/message/{id}/attachment/{idx}` | `get_attachment(id, idx)` | ❌ à ajouter |
| POST `/webmail/compose` | `send_mail(to, subject, body, cc, bcc, attachments)` | ✅ déjà OK (étendre attachments) |
| POST `/webmail/message/{id}/flag` | `flag_mail(id, flag)` | ❌ à ajouter |
| DELETE `/webmail/message/{id}` | `delete_mail(id)` | ❌ à ajouter |

**Décision** : `jaymail-client` doit être enrichi de 4 méthodes en P5
(`get_mail`, `get_attachment`, `flag_mail`, `delete_mail`). `send_mail`
existant à étendre pour supporter les pièces jointes.

### 4.5 JayCloud ↔ KindMother

| Usage | API KindMother |
|-------|----------------|
| Authentification | `kindmother::verify_identity(credentials) -> CogIdentity` |
| Dérivation clé chiffrement files | `kindmother::derive_key("jaycloud_files_v1") -> Key` |
| Signature liens publics | `kindmother::sign(payload) -> Signature` |
| Vérification jeton applicatif | `kindmother::verify_signature(token, sig) -> bool` |
| Stockage chiffré sessions | via `kindmother-client` libSQL handle |

Aucune extension de KindMother nécessaire — l'API actuelle suffit.

---

## 5. Schémas de données (libSQL chiffré JayCloud)

JayCloud persiste **uniquement** ce qui n'appartient à aucun autre
service.

### 5.1 Tables

```sql
-- Sessions web du portail
CREATE TABLE sessions (
    id            TEXT PRIMARY KEY,            -- UUID
    user_id       TEXT NOT NULL,                -- identité COG
    created_at    INTEGER NOT NULL,             -- epoch ms
    expires_at    INTEGER NOT NULL,
    user_agent    TEXT,
    ip_hash       TEXT                          -- HMAC-SHA256 de l'IP (anti-replay)
);
CREATE INDEX idx_sessions_user ON sessions(user_id);
CREATE INDEX idx_sessions_expiry ON sessions(expires_at);

-- Jetons applicatifs DAV (app-passwords)
CREATE TABLE app_passwords (
    id            TEXT PRIMARY KEY,             -- UUID
    user_id       TEXT NOT NULL,
    name          TEXT NOT NULL,                -- "DAVx5 Android", "Thunderbird"
    token_hash    TEXT NOT NULL,                -- SHA256(token) — le token brut n'est jamais stocké
    scopes        TEXT NOT NULL,                -- JSON ["webdav","caldav","carddav","webmail"]
    created_at    INTEGER NOT NULL,
    last_used_at  INTEGER,
    revoked_at    INTEGER                       -- NULL = actif
);
CREATE INDEX idx_app_passwords_user ON app_passwords(user_id);

-- Liens publics de partage
CREATE TABLE share_links (
    token         TEXT PRIMARY KEY,             -- short token URL-safe
    resource_type TEXT NOT NULL,                -- "file" | "folder"
    resource_path TEXT NOT NULL,                -- chemin canonique côté files_op
    owner_user_id TEXT NOT NULL,
    permissions   TEXT NOT NULL,                -- "read" | "read_write"
    password_hash TEXT,                          -- argon2id si protégé
    created_at    INTEGER NOT NULL,
    expires_at    INTEGER,                      -- NULL = pas d'expiration
    download_count INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX idx_share_links_owner ON share_links(owner_user_id);

-- Redirections legacy MiyuCloud → JayCloud (migration M-3)
CREATE TABLE miyucloud_redirects (
    legacy_token  TEXT PRIMARY KEY,             -- token MiyuCloud original
    new_token     TEXT NOT NULL,                -- token JayCloud actuel
    expires_at    INTEGER                       -- fin de fenêtre de transition (90j par défaut)
);

-- Cache ETags des ressources DAV (invalidation)
CREATE TABLE dav_etags (
    resource_path TEXT PRIMARY KEY,             -- "calendars/user/cal1/event-uid.ics" etc.
    etag          TEXT NOT NULL,
    last_modified INTEGER NOT NULL,
    last_seen_at  INTEGER NOT NULL
);
```

### 5.2 Politique de chiffrement

- libSQL avec **chiffrement au repos** via clé dérivée par
  `kindmother::derive_key("jaycloud_storage_v1")`.
- Aucun champ "raw token" stocké : on stocke `token_hash = SHA256(token)`.
  Le token brut n'est connu de l'utilisateur qu'au moment de la création.
- `password_hash` des liens publics : Argon2id (paramètres standards
  Miyukini).

---

## 6. Protocole WebDAV — détails

### 6.1 Méthodes supportées en P3

| Méthode | Statut P3 | Notes |
|---------|-----------|-------|
| `OPTIONS` | ✅ | Annonce DAV: 1, 2 dans la réponse. |
| `PROPFIND` | ✅ | Depth 0 / 1 / infinity (limite infinity à 1000 entrées). |
| `PROPPATCH` | ⚠ | Propriétés "live" en lecture seule ; properties "dead" stockées dans `dav_etags`. |
| `GET` / `HEAD` | ✅ | Avec support Range. |
| `PUT` | ✅ | Support Content-Range / PATCH X-Update-Range (RFC 4918 + extension). |
| `DELETE` | ✅ | Soft delete → corbeille (`files_op::trash`). |
| `MKCOL` | ✅ | Création de dossier. |
| `COPY` | ✅ | Avec gestion Depth / Overwrite. |
| `MOVE` | ✅ | Atomic via rename si même filesystem. |
| `LOCK` / `UNLOCK` | ✅ | LockTokens stockés en mémoire (TTL 1h). |

### 6.2 Backend `DavFileSystem` custom

`dav-server` exige une impl du trait `DavFileSystem`. JayCloud fournit
`files::davfs::JayCloudDavFs` qui :

- Map les chemins DAV vers l'arborescence `files_op`.
- Intercepte les opérations d'écriture pour passer par `borderguard`
  (gate Cores) cohérent avec la gouvernance Alicia.
- Encrypt/decrypt au repos via `files::encryption`.
- Émet des événements vers MiyukiniNotify pour le bandeau d'activité.

### 6.3 Réponses XML

Construites avec `quick-xml` + templates `dav_xml_kit`. Pas de
sérialisation `serde-xml-rs` (instable). Exemple PROPFIND response :

```xml
<?xml version="1.0" encoding="utf-8"?>
<D:multistatus xmlns:D="DAV:">
  <D:response>
    <D:href>/dav/files/folder/file.txt</D:href>
    <D:propstat>
      <D:prop>
        <D:displayname>file.txt</D:displayname>
        <D:getcontentlength>1234</D:getcontentlength>
        <D:getcontenttype>text/plain</D:getcontenttype>
        <D:getetag>"a1b2c3..."</D:getetag>
        <D:resourcetype/>
      </D:prop>
      <D:status>HTTP/1.1 200 OK</D:status>
    </D:propstat>
  </D:response>
</D:multistatus>
```

---

## 7. Protocole CalDAV — détails

### 7.1 Endpoints

| Endpoint | Méthode | Rôle |
|----------|---------|------|
| `/dav/principals/users/{user}/` | PROPFIND | Principal URL. |
| `/dav/calendars/{user}/` | PROPFIND | Liste des calendriers de l'utilisateur. |
| `/dav/calendars/{user}/{cal}/` | PROPFIND, REPORT | Métadonnées + calendar-query / calendar-multiget. |
| `/dav/calendars/{user}/{cal}/{uid}.ics` | GET, PUT, DELETE | CRUD événement. |
| `.well-known/caldav` | redirection 301 | Pour autodiscovery (DAVx⁵). |

### 7.2 Reports CalDAV (RFC 4791 §7)

| Report | Statut P4 | Impl |
|--------|-----------|------|
| `calendar-query` | ✅ | Parse XML filter via `dav_xml_kit`, traduit en `JayKoaBackend::list_events(from, to)` puis filtre côté JayCloud. |
| `calendar-multiget` | ✅ | Récupère un set d'UIDs en un appel. |
| `free-busy-query` | ⚠ P7 | Reporté. |
| `sync-collection` | ⚠ P7 | Reporté (RFC 6578). |

### 7.3 Mapping iCal ↔ JayKoa EventSummary

| iCal property | JayKoa field | Conversion |
|---------------|--------------|------------|
| `UID` | `EventSummary.id` | direct |
| `SUMMARY` | `EventSummary.title` | direct |
| `DTSTART` | `EventSummary.start` | parse iCal datetime → RFC 3339 |
| `DTEND` | `EventSummary.end` | parse iCal datetime → RFC 3339 |
| `LOCATION` | `EventSummary.location` | direct |
| `X-ALL-DAY` ou DTSTART date-only | `EventSummary.all_day` | déduction |
| `RRULE` | (étendre EventSummary) | **à ajouter dans jaykoa-client P4** : champ `recurrence_rule: Option<String>` |
| `X-MWS-SOURCE` | `EventSummary.source_service` | propriété custom Miyukini |

### 7.4 Gestion des ETags

ETag = SHA-256 du `EventSummary` sérialisé en JSON canonique (clés
triées). Stocké dans `dav_etags`. Invalidé au PUT / DELETE.

---

## 8. Protocole CardDAV — détails

### 8.1 Endpoints

Symétriques à CalDAV : `/dav/addressbooks/{user}/{book}/{uid}.vcf`.
`.well-known/carddav` → 301.

### 8.2 Reports CardDAV (RFC 6352 §8)

| Report | Statut P4 | Impl |
|--------|-----------|------|
| `addressbook-query` | ✅ | Filter XML → `JayContactBackend::list_contacts(kind, limit)` + filtre côté JayCloud. |
| `addressbook-multiget` | ✅ | Batch get. |
| `sync-collection` | ⚠ P7 | Reporté. |

### 8.3 Mapping vCard ↔ JayContact ContactSummary

| vCard 4.0 property | ContactSummary field | Conversion |
|--------------------|----------------------|------------|
| `UID` | `id` | direct |
| `FN` (formatted name) | `display_name` | direct |
| `N` (structured) | — | non utilisé en P4, on garde dans `extra` |
| `EMAIL` | (étendre ContactSummary) | **à ajouter dans jaycontact-client P4** : champ `emails: Vec<String>` |
| `TEL` | (étendre) | **à ajouter** : `phones: Vec<String>` |
| `ADR` | (étendre) | **à ajouter** : `addresses: Vec<Address>` |
| `X-MWS-KIND` | `kind` (Friend/Foe) | direct |

**Décision** : `jaycontact-client` doit être étendu en P4 avec les
champs vCard standards (emails, phones, addresses). Sinon le round-trip
CardDAV → JayCloud → CardDAV perd des données.

---

## 9. Webmail — architecture

### 9.1 Stack

| Couche | Lib | Rôle |
|--------|-----|------|
| Lecture IMAP | `async-imap` | Liste boîtes, fetch messages, recherche. |
| Envoi SMTP | `lettre` | Compose + envoi. |
| Parse MIME | `mail-parser` | Décodage messages, pièces jointes, encodages. |
| Build MIME | `lettre::message::Message` | Encodage sortant. |
| UI | axum + `askama` | Templates HTML server-rendered + HTMX pour interactivité. |
| Storage | aucun (proxy stateless) | Tout vit côté JayMail. JayCloud ne stocke pas de mail. |

### 9.2 Modèle d'accès

```
Navigateur ─── HTTPS portail ─── jaycloud (mail_op)
                                       │
                            ┌──────────┴──────────┐
                            │ jaymail-client      │
                            │ (méthodes étendues  │
                            │  get_mail,          │
                            │  get_attachment,    │
                            │  flag_mail,         │
                            │  delete_mail)       │
                            └──────────┬──────────┘
                                       │
                            ┌──────────┴──────────┐
                            │ JayMail backend     │
                            │ (async-imap +       │
                            │  lettre côté        │
                            │  service)           │
                            └─────────────────────┘
```

**Note** : `async-imap` / `lettre` ne sont **pas** dans `jaycloud` mais
dans **JayMail** (le service consommé). JayCloud reste un proxy via
`jaymail-client`. C'est cohérent avec DT-09.

### 9.3 Pièces jointes

- Petites (<1 MB) : streamées via `jaymail-client::get_attachment(id, idx)`.
- Grandes (>1 MB) : pop-up "Enregistrer dans Drive" → écriture dans
  `files_op` puis lien interne.
- Limite globale d'upload = 50 MB par défaut, configurable.

---

## 10. Authentification & jetons applicatifs

### 10.1 Authentification web

1. L'utilisateur navigue vers `https://<cog>/jaycloud/login`.
2. JayCloud délègue à KindMother (via `kindmother-client`).
3. Sur succès, session créée dans `sessions` table avec TTL 24h.
4. Cookie `Secure; HttpOnly; SameSite=Strict`.

### 10.2 Jetons applicatifs (app-passwords)

Génération d'un jeton par appareil / client DAV. Format :

```
mws-jc-<base32(random_16_bytes)>-<hmac>
       └─ token brut ─────────┘   └─ KindMother sig ┘
```

- Stocké côté JayCloud uniquement comme `SHA256(token)`.
- Affiché à l'utilisateur **une seule fois** au moment de la création.
- Révocation = `revoked_at = NOW()` ; cohérent immédiatement.
- HTTP Basic Auth avec `username = <user>, password = <token>` sur
  toutes les endpoints DAV.

### 10.3 Intégration Alicia (P7)

| Intent | Action |
|--------|--------|
| `share_file(path)` | crée un `share_link`, renvoie l'URL. |
| `list_app_passwords()` | liste avec dernière utilisation. |
| `revoke_app_password(name)` | révoque par nom (fuzzy match). |

---

## 11. Migration MiyuCloud — détails techniques

### 11.1 Pipeline de migration

```
Détection MiyuCloud
       │
       ▼
Validation cohérence (inventaire fichiers, taille, sha256)
       │
       ▼
Init storage JayCloud (libSQL + chiffrement key derived)
       │
       ▼
Migration arborescence
   ├─ Si même filesystem → rename (atomique, instant)
   └─ Sinon → copie streamée avec reprise sur erreur
       │
       ▼
Migration liens publics
   ├─ Génère JayCloud token équivalent
   └─ Insert dans miyucloud_redirects (legacy_token → new_token)
       │
       ▼
MiyuCloud → mode lecture seule + redirection 308 active
       │
       ▼
Fenêtre de transition (90j par défaut)
       │
       ▼
Désinstallation MiyuCloud par MasterButler
       │
       ▼
docs/services/DEPRECATED.md mis à jour
```

### 11.2 Compatibilité des liens

Une URL MiyuCloud `/cloud/files/<legacy_token>` → JayCloud regarde
`miyucloud_redirects.legacy_token` :

- Match trouvé : `308 Permanent Redirect` vers `/jaycloud/share/<new_token>`.
- Pas de match : 404.

L'utilisateur final ne voit aucune coupure tant que les liens existent
dans la table.

### 11.3 Catalogue services

Lors de la livraison P6, **dans le même commit** :

- `apps/origin/src/web/content.rs::load_services()` : retirer
  l'entrée `miyucloud`, ajouter `jaycloud`.
- `docs/services/DEPRECATED.md` : section MiyuCloud (date,
  périmètre, conséquences pour les services restants).
- Tests d'intégration MWS / Service Market : passer les références
  `miyucloud` à `jaycloud`.

---

## 12. Sécurité — détails par couche

| Couche | Mesure | Lib / mécanisme |
|--------|--------|-----------------|
| Transport | TLS 1.3 only, HSTS | `axum-server` + `rustls` |
| Auth web | Sessions courtes (24h), CSRF token, SameSite=Strict | `tower-sessions` |
| Auth DAV | App-passwords scopés, révocables, jamais stockés en clair | `token_kit` + `kindmother` |
| Chiffrement au repos | libSQL chiffré, clé dérivée KindMother | `kindmother::derive_key` |
| Stockage fichiers | Chiffrement par bloc côté `files::encryption` | `chacha20poly1305` (workspace) |
| Audit | Logs structurés (auth events, partages, DAV writes) | `tracing` |
| Rate-limit | Par IP + par jeton app sur chaque adapter DAV | `tower::limit::RateLimitLayer` |
| Anti-CSRF | Tokens sur écritures portail | session |
| Anti-SSRF | Pas d'appels HTTP sortants en chemin critique | revue de code |
| Anti-injection XML | `quick-xml` mode strict (rejette entités externes) | `quick-xml` |
| Anti-path traversal | Sanitisation systématique des chemins DAV via `files::tree::canonical_path` | impl interne |

---

## 13. Performance & caching

### 13.1 Cibles P3

| Métrique | Cible | Mesure |
|----------|-------|--------|
| Cold start | <1 s | binaire jaycloud → premier `200 OK` sur `/` |
| PROPFIND Depth 1 sur 100 fichiers | <200 ms | bench local |
| GET fichier 10 MB | >50 MB/s | bench local |
| CalDAV calendar-query sur 1 an | <300 ms | bench local |
| Mémoire au repos | <50 MB | RSS du process |

### 13.2 Caches

- **ETags** : `dav_etags` table, invalidation au PUT/DELETE.
- **Sessions** : libSQL, indexed `expires_at`.
- **Pas de cache de fichiers** en RAM en P3 (laisse le filesystem
  OS s'en charger).
- **Cache d'auth** : 5 min en RAM, invalidé sur révocation.

### 13.3 Hot paths

- `webdav_adapter::handle_get` : streaming direct depuis `files::davfs`.
- `caldav_adapter::handle_report` : passe par `dav_etags` avant tout
  call vers `jaykoa-client` (économie de round-trip si If-None-Match
  match).

---

## 14. Tests de conformité

### 14.1 WebDAV — Litmus

Litmus est la test suite de référence pour WebDAV (RFC 4918). Cible :

- `basic` ✅
- `copymove` ✅
- `props` ✅
- `locks` ✅
- `http` ✅

Intégration CI : script `scripts/test-jaycloud-litmus.sh` qui lance
JayCloud en mode test + litmus, et publie un rapport JUnit.

### 14.2 CalDAV / CardDAV — CalDAVTester

`CalDAVTester` (caldavtester.org) est la référence Apple. Cible en
P4 : profil "calendarserver-trial-basic". Subset minimal :

- ACLs basiques
- calendar-query avec filter time-range
- calendar-multiget
- addressbook-query / addressbook-multiget

### 14.3 Clients réels

Matrice de validation P4 :

| Client | Plateforme | Test |
|--------|------------|------|
| DAVx⁵ | Android | Sync agenda + contacts ; création / modification / suppression ; conflit. |
| Thunderbird | Linux/macOS/Windows | Lecture iCalendar + écriture event. |
| Apple Calendar | macOS / iOS | Sync agenda. |
| Apple Contacts | macOS / iOS | Sync contacts. |
| GNOME Calendar | Linux | Sync agenda via Evolution. |

### 14.4 Webmail — tests manuels

Pas de CTS pour webmail. Tests E2E via Playwright en P5 :

- Login → inbox → ouverture mail → réponse → envoi.
- Recherche.
- Pièces jointes upload + download.

---

## 15. Plan d'implémentation crate par crate

### 15.1 Ordre des PRs

| PR | Périmètre | Bloque |
|----|-----------|--------|
| **PR-1 (P2)** | Skeleton `crates/jaycloud/`, `crates/jaycloud-client/`, `service.manifest.json`, workspace entries. Cargo check vert. | Tout le reste. |
| **PR-2 (P3.a)** | `files_op` héritier MiyuCloud (rapatriement code) + `storage_kit` + `auth_op` minimal. | PR-3, PR-4. |
| **PR-3 (P3.b)** | `webdav_adapter` + portail Drive web (HTMX). Litmus pass. | PR-4 (partiellement). |
| **PR-4 (P4.a)** | Extension `jaykoa-client` : create/update/delete events + champ `recurrence_rule`. `calendar_op` + `caldav_adapter`. `ical_kit`. | PR-5 (a). |
| **PR-5 (P4.b)** | Extension `jaycontact-client` : update_contact + champs emails/phones/addresses. `contact_op` + `carddav_adapter`. `vcard_kit`. | PR-6. |
| **PR-6 (P5)** | Extension `jaymail-client` : get_mail / get_attachment / flag_mail / delete_mail. `mail_op` + `webmail_ui`. | PR-7. |
| **PR-7 (P6)** | `crates/jaycloud-migrate` + redirections share_op + entrées DEPRECATED.md + catalogue services MAJ. | PR-8. |
| **PR-8 (P7)** | `crates/jaycloud-client` (intents Alicia). | — |
| **PR-9 (P8)** | Polish : RFC 3253 versioning, iCalendar publish, sharing extensions, sync-collection. | — |

### 15.2 Extensions requises sur les services frères

Récapitulatif (à intégrer en PR-4 / PR-5 / PR-6) :

| Crate | Extension |
|-------|-----------|
| `jaykoa-client` | `create_event` / `update_event` / `delete_event` ; champ `recurrence_rule: Option<String>` dans `EventSummary`. |
| `jaycontact-client` | `update_contact(id, name, kind, emails, phones, addresses)` ; champs `emails: Vec<String>`, `phones: Vec<String>`, `addresses: Vec<Address>` dans `ContactSummary` ; nouveau type `Address`. |
| `jaymail-client` | `get_mail(id)` / `get_attachment(id, idx)` / `flag_mail(id, flag)` / `delete_mail(id)` ; `send_mail` étendu avec `Vec<Attachment>`. |

Ces extensions sont **rétrocompatibles** (ajout pur) et ne cassent
aucun consommateur existant.

### 15.3 Suppression progressive de MiyuCloud

| Phase | Action MiyuCloud |
|-------|------------------|
| PR-1 → PR-5 | MiyuCloud reste tel quel. JayCloud cohabite. |
| PR-2 (P3.a) | Le code de MiyuCloud est **dupliqué** dans `crates/jaycloud/src/files/`. MiyuCloud original conservé pour servir les utilisateurs non encore migrés. |
| PR-7 (P6) | MiyuCloud passe en lecture seule (via flag config). Catalogue retiré. |
| Après transition (90j) | `crates/miyucloud` et `crates/miyucloud-dav` supprimés du workspace. Entrée DEPRECATED.md ajoutée. |

---

## 16. Risques techniques

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Conflit CalDAV bidirectionnel (édition simultanée client + Central) | élevée | élevé | Last-write-wins en P4 ; ETags + If-Match obligatoires ; conflit signalé dans le bandeau Activity. CRDT évalué P8. |
| Clients DAV exotiques (vieux Outlook, certains Android OEM) | moyenne | moyen | Matrice de clients officiellement supportés ; bugs hors matrice = backlog non-bloquant. |
| Volume MiyuCloud >100 Go à migrer | moyenne | moyen | Migration par rename si même filesystem ; sinon copie streamée avec reprise. |
| Latence IMAP/SMTP variable (webmail) | moyenne | moyen | Timeouts agressifs (5 s par appel) + UI optimiste avec indicateur de progression. |
| Limite mémoire sur PROPFIND profond | faible | élevé | Limite Depth infinity = 1000 entrées ; au-delà → réponse partielle + `Preconditions Failed`. |
| Bug RFC dans `dav-server` (rare mais possible) | faible | élevé | Pin version + tests Litmus à chaque MAJ ; fork interne si bloqué. |
| `vcard4` crate non maintenu | moyenne | moyen | Fork interne `crates/vcard4-mws/` en fallback ; déjà budgété en P4. |
| Conformité CalDAVTester non passée | moyenne | élevé | Lots d'itérations P4 prévus ; subset minimum d'abord, extensions ensuite. |

---

## Annexes

### A. Mapping RFC ↔ Module

| RFC | Module |
|-----|--------|
| RFC 4918 (WebDAV) | `adapters/webdav.rs` + `dav-server` |
| RFC 4791 (CalDAV) | `adapters/caldav.rs` + `kits/ical_kit.rs` + `kits/dav_xml_kit.rs` |
| RFC 6352 (CardDAV) | `adapters/carddav.rs` + `kits/vcard_kit.rs` + `kits/dav_xml_kit.rs` |
| RFC 5545 (iCalendar) | `kits/ical_kit.rs` (via `icalendar` crate) |
| RFC 6350 (vCard 4.0) | `kits/vcard_kit.rs` (via `vcard4` crate) |
| RFC 3501 (IMAP4) | délégué à JayMail backend |
| RFC 5321 (SMTP) | délégué à JayMail backend |
| RFC 5322 (Internet Messages) | `mail-parser` (côté JayMail) |
| RFC 4226 / 6238 (HOTP/TOTP) | délégué à KindMother (MFA optionnel P7) |

### B. Glossaire MSCM JayCloud

| Terme | Définition |
|-------|------------|
| **Adaptateur** | Couche 1 — expose un protocole vers l'extérieur. Stateless, parsing/sérialisation uniquement. |
| **Opérateur** | Couche 2 — détient ou orchestre une logique métier. Stateful (cache, ETags). |
| **Kit** | Couche 3 — bibliothèque interne réutilisable, sans état persistant. |
| **App-password** | Jeton applicatif révocable, scopé par appareil/client, utilisé en HTTP Basic Auth sur les DAV. |
| **Fenêtre de transition** | Période (90j par défaut) pendant laquelle MiyuCloud reste accessible en lecture seule pour servir les liens existants. |
| **CTS** | Compatibility Test Suite — Litmus pour WebDAV, CalDAVTester pour CalDAV/CardDAV. |
| **Live property** | Propriété DAV calculée à la volée (ex: `getcontentlength`). |
| **Dead property** | Propriété DAV stockée en propre (custom). Limité dans JayCloud P3. |

### C. Variables de configuration

| Clé | Type | Défaut | Rôle |
|-----|------|--------|------|
| `JAYCLOUD_STORAGE_PATH` | string | `~/.miyukini/jaycloud/` | Racine du storage local. |
| `JAYCLOUD_HTTP_PORT` | int | 8443 | Port HTTPS du portail. |
| `JAYCLOUD_DAV_DEPTH_LIMIT` | int | 1000 | Limite PROPFIND Depth infinity. |
| `JAYCLOUD_SESSION_TTL_SECONDS` | int | 86400 | TTL session web. |
| `JAYCLOUD_LOCK_TTL_SECONDS` | int | 3600 | TTL DAV LOCK. |
| `JAYCLOUD_SHARE_LINK_DEFAULT_EXPIRY_DAYS` | int | 30 | Expiration par défaut liens publics. |
| `JAYCLOUD_MIYUCLOUD_TRANSITION_DAYS` | int | 90 | Fenêtre de transition migration. |
| `JAYCLOUD_RATE_LIMIT_PER_TOKEN_PER_MIN` | int | 600 | Rate limit par app-password. |

### D. Dette technique acceptée P0→P5

| Item | Reporté à | Justification |
|------|-----------|---------------|
| Sync-collection (RFC 6578) | P7 | Permet sync incrémentale ; clients DAV s'en passent par défaut. |
| Free-busy CalDAV | P7 | Pas d'usage métier en P0. |
| Federation OIDC inter-COG | P8 | Périmètre MWS séparé. |
| Versioning WebDAV (RFC 3253) | P8 | `files_op::versioning` interne suffit en attendant. |
| Sharing WebDAV extensions | P8 | Liens publics propres suffisent en P3. |
| CRDT pour résolution de conflit CalDAV | P8 | Last-write-wins acceptable au lancement. |
| MFA / TOTP login portail | P7 | Sessions web suffisent en P0 ; MFA pour comptes sensibles ensuite. |
| iCalendar publish (calendrier public) | P8 | Pas demandé par les early adopters. |

---

> **Prochaine étape MIP** : sur validation de cette spec, P2 = skeleton
> crates (`crates/jaycloud/`, `crates/jaycloud-client/`,
> `crates/jaycloud-migrate/`) avec `Cargo.toml`, `service.manifest.json`,
> entrées workspace, `cargo check` vert. Aucune fonctionnalité — juste
> le squelette pour que les PR suivantes (P3 → P8) puissent atterrir
> proprement.
