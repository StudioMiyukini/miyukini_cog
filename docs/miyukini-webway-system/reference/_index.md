# MWS â€” Index des RÃ©fÃ©rences

## Contexte

Ce document est l'**index des rÃ©fÃ©rences** dÃ©taillÃ©es du Miyukini Webway System (MWS). Il centralise les liens vers les spÃ©cifications prÃ©cises (normes, protocoles, relay, trackers, contrats) qui se trouvent dans l'arborescence `docs/reference/`, `docs/tools/` et `docs/setup/`.

**Racine documentaire :** [README](../README.md) | **Document fondateur :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

---

## 1. Documentation MWS interne

### Architecture et positionnement

| Document | Contenu |
|----------|---------|
| [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md) | DÃ©finition, subordination aux Cores, acteurs, principes cardinaux |
| [MWS - Architecture et Subordination](../architecture/MWS%20-%20Architecture%20et%20Subordination%20aux%20Cores.md) | Position vs strates, Lois d'Autonomie |
| [MWS - Consommation par les Strates](../strates/MWS%20-%20Consommation%20par%20les%20Strates.md) | Comment Cores, Outils, OpÃ©rateurs consomment le MWS |

### Acteurs du MWS

| Document | Contenu |
|----------|---------|
| [MWS - Origin](../acteurs/MWS%20-%20Origin.md) | Point d'origine, source de vÃ©ritÃ©, Registre de Services, Passeports spÃ©ciaux |
| [MWS - Relays](../acteurs/MWS%20-%20Relays.md) | Duplications d'Origin, vÃ©rification 3 phases, distribution des versions |
| [MWS - Trackers](../acteurs/MWS%20-%20Trackers.md) | Douaniers, pools ; catalogue web = services WEB publics ; Lobbys visibles depuis les services |

### VÃ©rification et identitÃ©

| Document | Contenu |
|----------|---------|
| [MWS - Passeport et Permis](../verification/MWS%20-%20Passeport%20et%20Visa.md) | Structure Passeport COG, types, Permis de circulation (accord relay), accord d'hÃ´te |
| [MWS - Flux de VÃ©rification](../verification/MWS%20-%20Flux%20de%20Verification.md) | Phase A (clÃ© Cores), Phase B (blocs MIP), Phase C (santÃ©), rÃ©sultats |

### SÃ©curitÃ©

| Document | Contenu |
|----------|---------|
| [MWS - Quarantaine et Blacklist](../securite/MWS%20-%20Quarantaine%20et%20Blacklist.md) | Escalade progressive, auto-destruction, alerte rÃ©seau, confinement |
| [MWS - Chiffrement et TLS](../securite/MWS%20-%20Chiffrement%20et%20TLS.md) | TLS obligatoire, exemption temps rÃ©el, authentification, secrets |
| [MWS - Registre de Services et Isolation](../securite/MWS%20-%20Registre%20de%20Services%20et%20Isolation.md) | Registre de Services, services tiers, isolation, suivi des mises Ã  jour |
| [MWS - Audit de SÃ©curitÃ© Complet](../securite/MWS%20-%20Audit%20de%20Securite%20Complet.md) | **AUDIT** â€” Vecteurs d'attaque, simulations, vulnÃ©rabilitÃ©s, plan de remÃ©diation |
| [MWS - Contre-Mesures Prioritaires](../securite/MWS%20-%20Contre-Mesures%20Prioritaires.md) | **REMEDIATION** â€” ImplÃ©mentation dÃ©taillÃ©e des contre-mesures critiques |
| [MWS - Contre-Mesures de SÃ©curitÃ©](../securite/MWS%20-%20Contre-Mesures%20de%20Securite.md) | **REFERENTIEL** â€” Index normatif R-001 Ã  R-015 et renvois vers les docs MWS |
| [MWS - Haute DisponibilitÃ© Origin](../securite/MWS%20-%20Haute%20Disponibilite%20Origin.md) | R-001 â€” Architecture actif-passif, RTO/RPO, rÃ©plication |
| [MWS - ProcÃ©dure de Failover](../securite/MWS%20-%20Procedure%20de%20Failover.md) | R-001 â€” Bascule Origin, relay promu |
| [MWS - Protection DDoS](../securite/MWS%20-%20Protection%20DDoS.md) | R-002 â€” Rate limiting, PoW, anti-DDoS |
| [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md) | Adresse Origin non falsifiable : manifeste signÃ©, pin TLS, rÃ´le des distributions |

### Lobbys et connexions

| Document | Contenu |
|----------|---------|
| [MWS - Lobbys, Favoris et Amis](../lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privÃ©s, surfaces, flow clientâ†’hÃ´te, favoris, amis |

### Protocole

| Document | Contenu |
|----------|---------|
| [MWS - Protocole Relay](../protocole/MWS%20-%20Protocole%20Relay.md) | Format binaire, types de messages, sÃ©quences d'Ã©change |

### Administration

| Document | Contenu |
|----------|---------|
| [MWS - MiyukiniAdmin](../administration/MWS%20-%20MiyukiniAdmin.md) | **ADMIN** â€” Panneau d'administration Origin : authentification, batterie de tests, monitoring, gestion services MWS |

### DÃ©ploiement

| Document | Contenu |
|----------|---------|
| [MWS - Guide de DÃ©ploiement](../deploiement/MWS%20-%20Guide%20de%20Deploiement.md) | Installation, TLS, systemd, monitoring, sÃ©curitÃ© |
| [MWS - ImplÃ©mentation Origin Hostinger](../deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md) | **IMPLEMENTATION** â€” Origin sur VPS Hostinger (Debian 13), config complÃ¨te, MiyukiniAdmin, manifeste, nginx |

---

## 2. RÃ©fÃ©rences conceptuelles (`docs/reference/`)

### SystÃ¨me MWS

| Document | Contenu |
|----------|---------|
| [Miyukini Webway System](_index.md) | DÃ©finition MWS, acteurs (participant, Tracker), annonces, listes de statuts |
| [Miyukini Webway System Complet](_index.md) | Document maÃ®tre consolidÃ© : vue d'ensemble, protocole, relay, sÃ©curitÃ© |
| [Miyukini Webway System Normes et Standards](_index.md) | Formats, norme de dÃ©claration sÃ©curisÃ©e, ports, matrice des statuts |
| [Miyukini Webway System Outils et OpÃ©rateurs](_index.md) | Outils et OpÃ©rateurs MWS (Strate 6 et 7) |

### Relay et Origin

| Document | Contenu |
|----------|---------|
| [Miyukini Webway Relay](_index.md) | Architecture complÃ¨te : Origin, relays, trackers ; flux de vÃ©rification ; Lobbys ; versioning ; Registre |
| [Miyukini Webway Relay Protocol](_index.md) | Protocole binaire : messages, format des trames, TLS, handshake |

### Connexion et glossaire

| Document | Contenu |
|----------|---------|
| [Connexion Inter-COG](_index.md) | Visite gouvernÃ©e, Passeport, Permis de circulation, Bridge |
| [Glossaire](_index.md) | Termes officiels MWS (Origin, Lobby, Permis de circulation, Pool, etc.) |

---

## 3. Outils MWS (`docs/tools/`)

### MiyuWebwayTracker

| Document | Contenu |
|----------|---------|
| [Documentation Fondatrice](../../tools/MiyuWebwayTracker/MiyuWebwayTracker%20-%20Documentation%20Fondatrice.md) | Document fondateur du toolkit Tracker |
| [Reference Outils](../../tools/MiyuWebwayTracker/MiyuWebwayTracker%20-%20Reference%20Outils.md) | RÃ©fÃ©rence technique |
| [Passive Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) | PrÃ©conditions, postconditions, invariants (systÃ¨mes passifs) |
| [Active Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) | DÃ©clencheurs et actions (systÃ¨mes actifs) |
| [Tool Governance Compliance](../../tools/MiyuWebwayTracker/contracts/governance/MiyuWebwayTracker%20-%20Tool%20Governance%20Compliance%20Contract.md) | ConformitÃ© gouvernance |
| [Implementation Guidelines](../../tools/MiyuWebwayTracker/implementation/MiyuWebwayTracker%20-%20Reference%20Implementation%20Guidelines.md) | Guide d'implÃ©mentation |

### MiyuWebwayParticipant

| Document | Contenu |
|----------|---------|
| [Documentation Fondatrice](../../tools/MiyuWebwayParticipant/MiyuWebwayParticipant%20-%20Documentation%20Fondatrice.md) | Document fondateur du toolkit Participant |
| [Reference Outils](../../tools/MiyuWebwayParticipant/MiyuWebwayParticipant%20-%20Reference%20Outils.md) | RÃ©fÃ©rence technique |
| [Tool Governance Compliance](../../tools/MiyuWebwayParticipant/contracts/governance/MiyuWebwayParticipant%20-%20Tool%20Governance%20Compliance%20Contract.md) | ConformitÃ© gouvernance |
| [Implementation Guidelines](../../tools/MiyuWebwayParticipant/implementation/MiyuWebwayParticipant%20-%20Reference%20Implementation%20Guidelines.md) | Guide d'implÃ©mentation |

---

## 4. Setup et dÃ©ploiement (`docs/setup/`)

| Document | Contenu |
|----------|---------|
| [Webway Relay Deployment Guide](..//setup//Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) | Guide de dÃ©ploiement complet (VM, TLS, systemd, tests) |
| [Hostinger VPS Origin Webway](..//setup//Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) | Instance Origin sur Hostinger VPS Debian 13 (Rust, systemd, ufw, monitoring) |

---

## 5. Matrice de couverture

| Sujet | Document principal | RÃ©fÃ©rences dÃ©taillÃ©es |
|-------|-------------------|----------------------|
| **Positionnement MWS** | Document Fondateur | Architecture, Consommation Strates |
| **Origin** | MWS - Origin | Relay (sections 1, 3, 6), MiyukiniAdmin |
| **Relays** | MWS - Relays | Relay, Relay Protocol |
| **Trackers** | MWS - Trackers | MiyuWebwayTracker (contrats) |
| **Passeport / Permis de circulation** | MWS - Passeport et Permis | Relay (accord relay), Tracker (contrÃ´le tracker) |
| **VÃ©rification** | MWS - Flux de VÃ©rification | Relay (section 2) |
| **Quarantaine / Blacklist** | MWS - Quarantaine et Blacklist | Relay (sections 2.8, 2.9, 3.4) |
| **Chiffrement** | MWS - Chiffrement et TLS | Relay (sections 3.3, 10) |
| **Audit SÃ©curitÃ©** | MWS - Audit de SÃ©curitÃ© Complet | Contre-Mesures Prioritaires |
| **Lobbys** | MWS - Lobbys, Favoris et Amis | Relay (sections 8, 9) |
| **Protocole** | MWS - Protocole Relay | Relay Protocol |
| **Administration** | MWS - MiyukiniAdmin | ImplÃ©mentation Origin Hostinger |
| **DÃ©ploiement** | MWS - Guide de DÃ©ploiement | Setup guides, ImplÃ©mentation Origin Hostinger |

---

**Version :** 3.0  
**DerniÃ¨re mise Ã  jour :** 2026-02-13


