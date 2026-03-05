# Miyukini COG â€” Ã‰tude App Android Terminal

## Contexte

Lâ€™Ã©cosystÃ¨me Miyukini COG prÃ©voit, pour les **COGs TERMINAL** (type `TERMINAL`, `os_type` `ANDROID`), un client mobile embarquÃ© **enfant dâ€™un COG STABLE** du mÃªme utilisateur. Cette Ã©tude pose les bases dâ€™une **app Android** permettant aux utilisateurs en mobilitÃ© dâ€™interagir avec leur environnement COG via un terminal lÃ©ger.

**RÃ©fÃ©rences :**

- [MWS - Passeport et Visa](../miyukini-webway-system/verification/MWS%20-%20Passeport%20et%20Visa.md)
- [Glossaire](..//miyukini-webway-system//reference//_index.md)
- [Comportement COG Environnements](..//miyukini-webway-system//reference//_index.md)

---

## PortÃ©e / Scope

- DÃ©finition architecturale du COG TERMINAL Android
- Contraintes MWS (relation parent-enfant, Passeport, Permis)
- Options techniques (stack UI, binaires, IPC)
- FonctionnalitÃ©s cibles pour utilisateurs nomades
- ScÃ©narios de synchronisation et offline
- Pistes de roadmap MVP

**Public :** Architectes, dÃ©veloppeurs, dÃ©cisions produit.

---

## 1. DÃ©finition du COG Terminal Android

### 1.1 RÃ´le

| Aspect | Description |
|--------|-------------|
| **Type** | `TERMINAL` (cog_type = 0x05) |
| **OS** | `ANDROID` (os_type) |
| **RÃ´le** | Extension mobile du COG Stable ; capacitÃ©s rÃ©duites |
| **Parent** | Obligatoire : `parent_cog_id` = cog_id du STABLE |
| **Limite** | 5 terminaux max par COG Stable (MWS) |

### 1.2 Contraintes MWS

| RÃ¨gle | Description |
|-------|-------------|
| `parent_cog_id` | Toujours prÃ©sent dans le Passeport |
| MÃªme utilisateur | Parent et enfant = mÃªme identitÃ© utilisateur |
| Connexion | Via parent ou directe avec dÃ©pendance au parent |
| Blacklist | Si parent blacklistÃ© â†’ terminaux blacklistÃ©s |
| Passeport | STANDARD (Ã©mission automatique, comme STABLE) |

### 1.3 Flux de provisionnement (depuis le STABLE)

```mermaid
sequenceDiagram
    participant U as Utilisateur
    participant C as Miyukini Central (STABLE)
    participant T as App Android (TERMINAL)
    participant R as Relay MWS

    U->>C: "CrÃ©er un Terminal mobile"
    C->>C: VÃ©rifier limite 5 terminaux
    C->>C: GÃ©nÃ©rer cog_id pour TERMINAL
    C->>C: CrÃ©er liaison parent_cog_id
    C->>T: QR / lien de liaison (token temporaire)
    T->>T: Premier lancement : saisie token
    T->>T: Charger parent_cog_id, cog_id
    T->>R: Passeport (cog_type=TERMINAL, parent_cog_id)
    R->>R: VÃ©rifier parent valide
    R->>T: Permis de circulation
    T->>C: Synchronisation initiale (services, prÃ©fÃ©rences)
```

---

## 2. Options techniques

### 2.1 Stack UI â€” Dioxus (retenue)

**DÃ©cision :** Stack **Dioxus** retenue pour compatibilitÃ© maximale avec Miyukini Central (Rust partagÃ©, patterns UI, rÃ©utilisation du thÃ¨me).

| Option | Avantages | InconvÃ©nients |
|--------|-----------|---------------|
| **Dioxus Mobile** âœ“ | Code Rust partagÃ© avec Central ; patterns connus ; hot-reload ; compatibilitÃ© maximale | ExpÃ©rimental Android ; WebView ou WGPU ; config NDK/SDK lourde |
| ~~Kotlin + Compose~~ | â€” | Pas de partage avec Central ; deux codebases UI |

**RÃ©fÃ©rence :** [Dioxus Mobile](https://dioxus.dev/learn/0.6/guides/mobile/) â€” rendu WebView ou WGPU, `dx serve` pour Ã©mulateurs.

### 2.2 Binaires et dÃ©pendances

| Composant | Option 1 | Option 2 |
|-----------|----------|----------|
| **Logique MWS** | RÃ©utiliser `miyuwebway_participant` (Rust) via JNI | Port Kotlin des protocoles MWS |
| **Base locale** | SQLite / libSQL (KindMother) via FFI | Room (Kotlin) |
| **RÃ©seau** | TCP/TLS via Rust (relay port 7000) | OkHttp / Ktor |
| **Authentification** | Token fourni par Central via lien/QR | Idem |

### 2.3 Architecture proposÃ©e (Dioxus)

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  App Android (COG TERMINAL)                          â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  UI Dioxus Mobile                                    â”‚
â”‚  - Ã‰crans : liaison, salon, services, paramÃ¨tres    â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  Couche Services                                     â”‚
â”‚  - Provisionnement (parent_cog_id, Passeport)        â”‚
â”‚  - Synchronisation avec parent                      â”‚
â”‚  - Consommation services (JayKonta, JayKoa, etc.)   â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  MWS Client (Rust, miyuwebway_participant)           â”‚
â”‚  - MiyuWebwayParticipant (allÃ©gÃ©)                    â”‚
â”‚  - Relay connect (port 7000)                        â”‚
â”‚  - Tracker discovery (port 21000)                   â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  Stockage local (SQLite / KindMother)                â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 3. FonctionnalitÃ©s cibles (utilisateurs nomades)

### 3.1 PrioritÃ© haute (MVP)

| FonctionnalitÃ© | Description |
|----------------|-------------|
| **Liaison au parent** | Scanner QR / saisir token depuis Central ; stockage sÃ©curisÃ© `parent_cog_id` |
| **Connexion MWS** | PrÃ©sentation Passeport â†’ obtention Permis ; heartbeats |
| **Vue services limitÃ©e** | AccÃ¨s consultatif aux services du parent (JayKonta, JayKoa) |
| **Mode offline** | Cache local ; queue des actions diffÃ©rÃ©es ; sync Ã  la reconnexion |
| **Notifications** | Alertes importantes (ex. rappels JayKoa, seuils JayKonta) |

### 3.2 PrioritÃ© moyenne

| FonctionnalitÃ© | Description |
|----------------|-------------|
| **Actions simples** | Saisie dÃ©pense, crÃ©ation Ã©vÃ©nement (dÃ©lÃ©guÃ© au parent) |
| **DÃ©couverte rÃ©seau** | Voir COGs accessibles (Lobbys, amis) via Tracker |
| **SÃ©curitÃ©** | Verrouillage app ; biomÃ©trie optionnelle |

### 3.3 PrioritÃ© basse

| FonctionnalitÃ© | Description |
|----------------|-------------|
| **Multi-compte** | Plusieurs COG Stable liÃ©s (changement de contexte) |
| **Jeux mobiles** | IntÃ©gration Miyukini Survivor / Clicker (lÃ©ger) |

---

## 4. ScÃ©narios de synchronisation

### 4.1 Connexion directe au Relay

Le TERMINAL peut se connecter directement au Relay (comme un STABLE), en prÃ©sentant un Passeport avec `parent_cog_id`. Le Relay vÃ©rifie que le parent est valide et dÃ©livre le Permis.

### 4.2 Connexion via parent (tunnel)

Alternative : le TERMINAL se connecte au parent STABLE (mÃªme rÃ©seau local ou tunnel), et le parent transmet les requÃªtes MWS. Moins de surface dâ€™attaque, mais dÃ©pendance forte Ã  la disponibilitÃ© du parent.

### 4.3 Mode offline

| Ã‰tat | Comportement |
|------|--------------|
| **DonnÃ©es en cache** | Lecture seule sur cache local (derniÃ¨re sync) |
| **Actions diffÃ©rÃ©es** | Ã‰crire en local ; queue sync ; rejouer Ã  la reconnexion |
| **Conflict** | Politique : dernier Ã©crit gagnant ou merge manuel (TAMR) |

---

## 5. Respect des Lois dâ€™Autonomie

| Loi | Application au Terminal |
|-----|-------------------------|
| **LOI-1** | Pas de dÃ©pendance externe critique : fonctionne en offline limitÃ© |
| **LOI-2** | Isolement acceptÃ© : sync diffÃ©rÃ©e, pas de blocage |
| **LOI-3** | Ã‰tat local souverain : cache et queue locaux |
| **LOI-4** | Pas de temps global : horodatage local, sync asynchrone |
| **LOI-5** | CoÃ»t proportionnel : app lÃ©gÃ¨re, pas de services cloud obligatoires |
| **LOI-6** | FÃ©dÃ©ration : connexion MWS via Relay/Tracker |
| **LOI-7** | Cores immuables : Terminal = client dâ€™un environnement versionnÃ© |
| **LOI-8** | Migration : via le parent ; pas de migration directe Terminal |

---

## 6. DÃ©pendances projet existant

### 6.1 Crates Ã  rÃ©utiliser ou adapter

| Crate | Usage |
|-------|-------|
| `miyuwebway_participant` | Transport, dÃ©claration, discovery (port Android/JNI si Rust) |
| `apps/origin` (protocol) | Types `CogType`, `OsType`, messages Relay |
| `kindmother`, `kindmother-client` | Persistance optionnelle (si Rust backend) |

### 6.2 Modifications Ã  prÃ©voir

| Composant | Modification |
|-----------|--------------|
| **Relay (Origin)** | DÃ©jÃ  supporte `parent_cog_id` ; vÃ©rification parent valide Ã  confirmer |
| **Central** | Nouvel Ã©cran "GÃ©rer mes Terminaux" ; gÃ©nÃ©ration token/QR liaison |
| **Passeport** | GÃ©nÃ©ration cÃ´tÃ© STABLE pour le TERMINAL ; transmission sÃ©curisÃ©e |

---

## 7. Pistes de roadmap

### Phase 0 â€” PrÃ©paration (Ã©tude actuelle)

- [x] Document dâ€™Ã©tude
- [x] Stack Dioxus retenue
- [ ] Documentation exhaustive (voir Plan de dÃ©veloppement Phase 1)
- [ ] SpÃ©cification flux liaison Central â†’ Terminal

### Phase 1 â€” Proof of concept

- [ ] App Android minimale (Ã©cran liaison + affichage statut)
- [ ] Connexion Relay avec Passeport TERMINAL
- [ ] Obtention Permis de circulation
- [ ] Stockage local `parent_cog_id`, `cog_id`

### Phase 2 â€” MVP

- [ ] Ã‰cran "Salon" avec liste services du parent
- [ ] Vue consultative (ex. JayKonta, JayKoa)
- [ ] Mode offline basique (cache)
- [ ] IntÃ©gration Central : crÃ©ation Terminal, QR/lien

### Phase 3 â€” Production

- [ ] Notifications push
- [ ] Actions simples (dÃ©penses, Ã©vÃ©nements)
- [ ] Tests E2E, signature release
- [ ] Documentation utilisateur

---

## 8. Risques et points de vigilance

| Risque | Mitigation |
|--------|------------|
| Dioxus Android instable | POC rapide (Phase 3) ; documentation Env Dev pour dÃ©pannage |
| Fatigue batterie (sync continue) | Sync par batch ; intervalles adaptatifs |
| SÃ©curitÃ© token liaison | Token Ã  usage unique ; expiration courte ; chiffrement stockage |
| Fragmentation Android | Cibler API 24+ (Android 7) ; tests sur Ã©mulateurs multiples |

---

## 9. RÃ©fÃ©rences

- **MWS Passeport/Visa** : `docs/miyukini-webway-system/verification/MWS - Passeport et Visa.md`
- **Protocole Relay** : `docs/miyukini-webway-system/protocole/MWS - Protocole Relay.md`
- **Architecture** : `.cursor/skills/miyukini-architecture/SKILL.md`
- **Glossaire** : `.cursor/skills/miyukini-glossary/SKILL.md`
- **Dioxus Mobile** : [dioxus.dev/learn/0.6/guides/mobile](https://dioxus.dev/learn/0.6/guides/mobile/)

