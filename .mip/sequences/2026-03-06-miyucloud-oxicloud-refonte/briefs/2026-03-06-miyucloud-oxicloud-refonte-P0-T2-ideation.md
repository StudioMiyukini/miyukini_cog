# P0 Temps 2 -- Ideation (Maria + Lise)

**Sequence** : 2026-03-06-miyucloud-oxicloud-refonte
**Debut** : 2026-03-06T13:55:20Z | **Fin** : 2026-03-06T13:59:19Z

## TL;DR
Fork complet OxiCloud (Approche A recommandee). Migration progressive actix->axum, adaptateur SQLite, conservation crypto+sync existants. 9 risques identifies. UI: migration vers miyuki-ui-dioxus (26 composants reutilisables, 13 a creer), 3 phases front. CalDAV/CardDAV UI differee Phase 3.

## Maria -- Cadrage fonctionnel

### Objectif principal
Remplacer le backend MiyuCloud actuel par un fork OxiCloud migre vers axum, integre a Central via Miyukini Connect. Demonstrateur certif-ready.

### Objectifs secondaires
1. Uniformisation framework HTTP (axum)
2. Gain fonctionnel : WebDAV/CalDAV/CardDAV
3. Deduplication SHA-256 + chunked upload TUS-like
4. Scalabilite DB via adaptateur SQLite (architecture Clean)
5. Conservation acquis : chiffrement E2E + sync P2P
6. Integration Central native enrichie

### Perimetre IN
- Fork OxiCloud complet dans monorepo COG
- Migration actix-web -> axum (couche interfaces)
- Adaptateur SQLite (couche infrastructure)
- Auth Miyukini Connect
- WebDAV/CalDAV/CardDAV adaptes axum
- Dedup + chunked upload + cache moka
- API REST MiyuCloud conservee (compat MiyuCloudClient)
- Surface web HTTPS (portail Dioxus SSR/HTML)
- UI Central complete (12+ composants existants)
- Crypto at-rest (ChaCha20-Poly1305, Argon2id, X25519)
- Sync P2P (vector clock, conflict resolution, peer discovery)

### Perimetre OUT
- Front-end React d'OxiCloud
- PostgreSQL (remplace par SQLite)
- Docker/infra OxiCloud
- Multi-tenant
- CI/CD OxiCloud (COG a sa propre CI)
- Federation inter-instances
- Application mobile
- Migration donnees v1

### Approche recommandee : A (Fork complet + migration progressive)
Migration module par module de la couche interfaces (auth -> files -> folders -> shares -> DAV). Tests OxiCloud existants restent fonctionnels pendant la migration. MiyuCloud actuel operationnel en parallele.

### Risques
| # | Risque | Prob | Impact | Mitigation |
|---|--------|------|--------|------------|
| R1 | Migration actix->axum | Elevee | Eleve | Architecture Clean isole la couche. Migration module par module. |
| R2 | Adaptateur SQLite | Elevee | Eleve | Traits repository dans domain/. SqliteRepository dans infrastructure/. |
| R3 | Volume code OxiCloud | Moyenne | Moyen | Import selectif des modules necessaires. |
| R4 | Compat WebDAV | Moyenne | Eleve | Suite tests litmus/cadaver apres migration. |
| R5 | Regression UI Central | Moyenne | Moyen | Maintien contrat API REST MiyuCloudClient. |
| R6 | Perte chiffrement E2E | Faible | Critique | Conserver crate crypto/ intact. Integration middleware. |
| R7 | Perte sync P2P | Faible | Critique | Conserver crate sync/ intact. Memes endpoints API. |
| R8 | Calendrier/Contacts UI | Elevee | Faible | Backend IN, UI differee Phase 3. |
| R9 | Maintenance fork | Moyenne | Moyen | Documenter chaque modif vs upstream. Cherry-pick selectif. |

## Lise -- Direction visuelle

### Composants a reutiliser depuis miyuki-ui-dioxus : 26
### Composants a creer : 13 (FileCard, FileRow, FileIcon, QuotaBar, UploadProgressBar, SyncStatusIndicator, PeerCard, ConflictCard, ShareLinkCard, CalendarView, ContactList, ContactCard, EmptyState)

### Migration UI en 3 phases
- Phase 1 (immediate) : Boutons, Breadcrumb, Modals, FileCard, FileRow, EmptyState, Icons
- Phase 2 (stabilisation) : Inputs, Sidebar, DataTable, QuotaBar, ShareLinkCard, Toast
- Phase 3 (enrichissement) : CalendarView, ContactList, ContactCard
