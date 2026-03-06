# Brief : Refonte MiyuCloud sur fork OxiCloud

## TL;DR (5 lignes)
Refonte complete de MiyuCloud en forkant OxiCloud (MIT, Rust, axum 0.8.8) pour obtenir WebDAV/CalDAV/CardDAV natifs, dedup SHA-256 et chunked uploads. Nouveau crate `miyucloud-dav` + extensions de l'existant. 88 taches en 10 etapes, 5 agents, ~2680 LOC nouvelles, 6 tables SQLite. Score securite 82->96/100. Budget estime $60-90 USD.

## Metadonnees
- Classe : T5 (Chantier strategique)
- Date : 2026-03-06
- Responsable P0 : Maria
- Duree P0 : 36 min (13:43 -> 14:20 UTC)

## Contexte
MiyuCloud actuel (KindMother/SQLite, ChaCha20-Poly1305, Argon2id, X25519 E2E, P2P sync) est fonctionnel mais limite en features. OxiCloud (DioCrafts, MIT, 2.6k stars) offre WebDAV/CalDAV/CardDAV, dedup SHA-256, chunked uploads TUS-like, moka cache -- des fonctionnalites qui prendraient des mois a reimplementer. Le fork permet d'accelerer massivement le developpement tout en conservant notre couche securite renforcee.

**ALERTE T4** : OxiCloud est deja sur axum 0.8.8. Pas de migration de framework necessaire.

## Objectifs
- **Principal** : Integration Central fluide -- naviguer, uploader, partager, calendrier, contacts depuis l'UI Dioxus
- **Secondaires** :
  1. WebDAV/CalDAV/CardDAV natifs (interop Thunderbird, macOS, iOS, DAVx5)
  2. Deduplication content-addressable SHA-256
  3. Score securite >95/100 (certif-ready)
  4. Vitrine technique de l'ecosysteme Miyukini
- **Criteres de succes** : build OK, tests E2E passes, 3 clients WebDAV valides, audit Victor >95/100

## Perimetre

### Inclus
- Fork OxiCloud complet integre dans le workspace COG
- Nouveau crate `crates/miyucloud-dav/` (WebDAV, CalDAV, CardDAV, WOPI)
- Extensions `crates/miyucloud/` (dedup, compression, thumbnails)
- 6 nouvelles tables SQLite (calendars, events, addressbooks, contacts, content_blobs, file_blobs)
- Adaptateur SQLite (remplace PostgreSQL d'OxiCloud)
- Auth Miyukini Connect (remplace OIDC/SSO OxiCloud)
- Conservation crypto existante (ChaCha20-Poly1305, Argon2id, X25519 E2E)
- Conservation sync P2P existante
- Surface web HTTPS existante
- Integration UI Central (12+ composants existants + migration miyuki-ui-dioxus)
- Hardening securite (CVE rusqlite, zeroize, XML defense, path validation)

### Exclus
- Front-end React d'OxiCloud (on garde Dioxus)
- PostgreSQL (remplace par SQLite via adaptateur)
- Docker/infra OxiCloud (COG a son propre systeme)
- Multi-tenant (mono-utilisateur)
- Application mobile
- Federation inter-instances
- Migration donnees v1

## Approches proposees

### Approche A -- Fork complet + migration progressive (RECOMMANDEE)
- **Description** : Importer le code OxiCloud dans un nouveau crate `miyucloud-dav`. Etendre le crate existant pour dedup/compression/thumbnails. Migration module par module.
- **Pour** : Progression mesurable, tests OxiCloud reutilisables, architecture Clean facilite le remplacement couche par couche, MiyuCloud actuel operationnel pendant la transition
- **Contre** : Volume de code important (~2680 LOC), coexistence temporaire de deux architectures
- **Effort** : 88 taches, ~$60-90

### Approche B -- Extraction selective
- **Description** : Extraire uniquement les modules domaine/application d'OxiCloud, reimplementer dans l'architecture existante
- **Pour** : Moins de code, pas de nouveau crate
- **Contre** : Perte architecture Clean/DDD, re-implementation manuelle, pas de benefice communaute
- **Non recommandee** : contradictoire avec la decision "fork"

### Approche C -- Wrapper/Proxy
- **Description** : Executer OxiCloud tel quel, proxier depuis MiyuCloud
- **Pour** : Zero portage
- **Contre** : Deux processus, dependance PostgreSQL, double latence, impossible d'integrer crypto E2E
- **Non recommandee** : contradictoire avec toutes les decisions figees

## Direction visuelle (Lise)
Conservation du theme COG sombre. Migration des composants locaux vers miyuki-ui-dioxus (26 reutilisables, 13 a creer). Elimination des emojis Unicode au profit d'icones vectorielles. Migration tokens typographiques et espacement. UI CalDAV/CardDAV differee Phase 3 front.

## Analyse concurrentielle (Fabrice)
6 concurrents analyses. Positionnement : *"Le cloud prive qui tient dans 128 Mo"*. Avantages : Rust+axum+SQLite (binaire unique), integration ecosysteme COG (unique), UI native Dioxus, securite certif-ready, protocoles complets. Faiblesses concurrents exploitees : Nextcloud lourd, Seafile/oCIS sans CalDAV/CardDAV, Syncthing sans web UI.

## Analyse de securite (Victor)

### Modele de menace
12 actifs, 5 acteurs, 7 surfaces d'attaque, 10 scenarios.

### Niveau : DURCI (2/3)
Score actuel 82/100, cible 96/100.

### Recommandations critiques
| # | Recommandation | Impact |
|---|---------------|--------|
| R1 | Verifier rusqlite pour CVE-2025-6965 | Securite critique |
| R2 | Validation WebDAV paths (whitelist, rejet ..) | Previent path traversal |
| R3 | Defenses XML CalDAV/CardDAV (desactiver DTD) | Previent XXE |
| R4 | Zeroize sur KeyManager.master_key | Nettoyage memoire |

### Checklist securite transmise a Francois
Auth, autorisation, validation entrees, chiffrement, secrets, logging, rate limiting, CORS -- detaillee dans RPS T5.

## Pipeline CI/CD (Hugo)
- 3 modifications obligatoires : paths triggers, clippy -p, test -p pour `miyucloud-dav`
- Temps build supplementaire : +1m50 (cold), +20s (cache)
- Pas de nouveaux secrets CI

## Inventaire des prerequis (Denis + Hugo + Jean)
- 12 crates a ajouter, 5 crates COG reutilises
- OxiCloud deja sur axum 0.8.8 (pas de migration framework)
- 10 etapes macro, 88 taches, 5 agents
- Budget : ~$60-90 USD (6.6M tokens, Sonnet 4 + Opus Denis)

## Specification technique (Francois)
- Nouveau crate `miyucloud-dav` (22 fichiers, WebDAV/CalDAV/CardDAV/WOPI/common)
- Extensions `miyucloud` (4 fichiers : dedup, compression, dedup_ops, thumbnail_ops)
- 12 fichiers existants modifies
- Types : Calendar, CalendarEvent, AddressBook, Contact, ContentBlob, ContentAddressableStorage trait
- Schema SQL : 6 tables avec index
- WAL mode SQLite pour concurrence

## Plan de developpement (Denis)
88 taches en 10 etapes + buffer 20%. Voir `plans_p3/2026-03-06-miyucloud-oxicloud-refonte-plan.md`.

| Etape | Titre | Taches |
|-------|-------|--------|
| E0 | Fork & Fondations | 8 |
| E1 | Schema & Types | 10 |
| E2 | Dedup & Compression | 8 |
| E3 | WebDAV Core | 12 |
| E4 | CalDAV | 8 |
| E5 | CardDAV | 7 |
| E6 | Thumbnails | 5 |
| E7 | WOPI | 6 |
| E8 | Integration Central | 6 |
| E9 | Integration & Tests E2E | 6 |
| E10 | Hardening & Audit | 8 |
| BUF | Buffer corrections | 4 |

## Audit de faisabilite (Arianne + Jean)
**Verdict : FAISABLE -- MANQUES MINEURS**

| # | Nature | Action |
|---|--------|--------|
| M1 | Agents Lise/Hugo non generes | Regenerer avant P3 |
| M2 | George genere mais absent du plan | Clarifier (audit P4 only) |
| M3 | http-range-header et fs2 en maintenance faible | Non bloquant, alternatives possibles |
| M4 | security-patterns.md et stack-patterns.md vides | Remplir en E0 |
| M5 | Plan detaille non materialise en fichier | Materialiser avant P3 |

SKILLs : 9 identifies, tous existants. Budget : $60-90 USD confirme.

## Risques

| # | Risque | Prob | Impact | Mitigation |
|---|--------|------|--------|------------|
| R1 | Adaptateur SQLite : incompatibilites PostgreSQL->SQLite | Elevee | Moyen | Tester chaque query. Eviter RETURNING. WAL mode. |
| R2 | Conformite WebDAV/CalDAV/CardDAV clients | Haute | Eleve | Suite tests litmus, 3 clients minimum |
| R3 | Volume code (~2680 LOC) | Moyenne | Moyen | Taches atomiques, 88 taches tracees |
| R4 | CVE-2025-6965 rusqlite | Faible | Critique | Bumper rusqlite en E0 |
| R5 | XXE via CalDAV/CardDAV XML | Haute | Eleve | xml_security.rs : pas de DTD, limite taille/profondeur |
| R6 | Perte crypto E2E lors integration | Faible | Critique | Conserver crate crypto intact, tests round-trip |
| R7 | Perte sync P2P | Faible | Critique | Conserver crate sync intact, memes endpoints |
| R8 | Maintenance fork vs upstream | Moyenne | Moyen | Cherry-pick selectif, documenter deltas |
| R9 | Concurrence SQLite single-writer | Haute | Moyen | WAL mode + busy_timeout=5000 |

## Decision
En attente de l'approbation utilisateur.

## Artefacts P0 produits

| Temps | Artefact | Chemin |
|-------|----------|--------|
| T1 | Brainstorming (5 sections, 15 questions) | Dans le contexte de conversation |
| T2 | Ideation (Maria + Lise) | `briefs/...-P0-T2-ideation.md` |
| T3 | Analyse concurrentielle (Fabrice) | `briefs/...-P0-T3-concurrence.md` |
| T4 | Inventaire prerequis (Denis + Hugo + Jean) | `briefs/...-P0-T4-inventaire.md` |
| T5 | RPS securite (Victor) | `briefs/...-P0-T5-securite.md` |
| T6 | Specification technique (Francois) | `specs/...-spec.md` |
| T7 | Agents fine-tuned (Maria) | `agents/` (20 fichiers + index + manifest) |
| T8 | Plan exhaustif (Denis) | `plans_p3/...-plan.md` |
| T9 | Audit faisabilite (Arianne + Jean) | Dans le contexte de conversation |
| T10 | CI/CD verification (Hugo) | Dans le contexte de conversation |
| T11 | Brief final (Maria) | Ce document |
| -- | Trace P0 | `phases/p0-trace.md` |
| -- | Metriques | `metrics/...-metrics.json` |
| -- | Ressources | `ressources/index.md` |
