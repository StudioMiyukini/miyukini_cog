# MWS — Index des Références

## Contexte

Ce document est l'**index des références** détaillées du Miyukini Webway System (MWS). Il centralise les liens vers les spécifications précises (normes, protocoles, relay, trackers, contrats) qui se trouvent dans l'arborescence `docs/reference/`, `docs/tools/` et `docs/setup/`.

**Racine documentaire :** [README](../README.md) | **Document fondateur :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

---

## 1. Documentation MWS interne

### Architecture et positionnement

| Document | Contenu |
|----------|---------|
| [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md) | Définition, subordination aux Cores, acteurs, principes cardinaux |
| [MWS - Architecture et Subordination](../architecture/MWS%20-%20Architecture%20et%20Subordination%20aux%20Cores.md) | Position vs strates, Lois d'Autonomie |
| [MWS - Consommation par les Strates](../strates/MWS%20-%20Consommation%20par%20les%20Strates.md) | Comment Cores, Outils, Opérateurs consomment le MWS |

### Acteurs du MWS

| Document | Contenu |
|----------|---------|
| [MWS - Origin](../acteurs/MWS%20-%20Origin.md) | Point d'origine, source de vérité, Registre de Services, Passeports spéciaux |
| [MWS - Relays](../acteurs/MWS%20-%20Relays.md) | Duplications d'Origin, vérification 3 phases, distribution des versions |
| [MWS - Trackers](../acteurs/MWS%20-%20Trackers.md) | Douaniers, pools ; catalogue web = services WEB publics ; Lobbys visibles depuis les services |

### Vérification et identité

| Document | Contenu |
|----------|---------|
| [MWS - Passeport et Permis](../verification/MWS%20-%20Passeport%20et%20Visa.md) | Structure Passeport COG, types, Permis de circulation (accord relay), accord d'hôte |
| [MWS - Flux de Vérification](../verification/MWS%20-%20Flux%20de%20Verification.md) | Phase A (clé Cores), Phase B (blocs MIP), Phase C (santé), résultats |

### Sécurité

| Document | Contenu |
|----------|---------|
| [MWS - Quarantaine et Blacklist](../securite/MWS%20-%20Quarantaine%20et%20Blacklist.md) | Escalade progressive, auto-destruction, alerte réseau, confinement |
| [MWS - Chiffrement et TLS](../securite/MWS%20-%20Chiffrement%20et%20TLS.md) | TLS obligatoire, exemption temps réel, authentification, secrets |
| [MWS - Registre de Services et Isolation](../securite/MWS%20-%20Registre%20de%20Services%20et%20Isolation.md) | Registre de Services, services tiers, isolation, suivi des mises à jour |
| [MWS - Audit de Sécurité Complet](../securite/MWS%20-%20Audit%20de%20Securite%20Complet.md) | **AUDIT** — Vecteurs d'attaque, simulations, vulnérabilités, plan de remédiation |
| [MWS - Contre-Mesures Prioritaires](../securite/MWS%20-%20Contre-Mesures%20Prioritaires.md) | **REMEDIATION** — Implémentation détaillée des contre-mesures critiques |
| [MWS - Contre-Mesures de Sécurité](../securite/MWS%20-%20Contre-Mesures%20de%20Securite.md) | **REFERENTIEL** — Index normatif R-001 à R-015 et renvois vers les docs MWS |
| [MWS - Haute Disponibilité Origin](../securite/MWS%20-%20Haute%20Disponibilite%20Origin.md) | R-001 — Architecture actif-passif, RTO/RPO, réplication |
| [MWS - Procédure de Failover](../securite/MWS%20-%20Procedure%20de%20Failover.md) | R-001 — Bascule Origin, relay promu |
| [MWS - Protection DDoS](../securite/MWS%20-%20Protection%20DDoS.md) | R-002 — Rate limiting, PoW, anti-DDoS |
| [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md) | Adresse Origin non falsifiable : manifeste signé, pin TLS, rôle des distributions |

### Lobbys et connexions

| Document | Contenu |
|----------|---------|
| [MWS - Lobbys, Favoris et Amis](../lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privés, surfaces, flow client→hôte, favoris, amis |

### Protocole

| Document | Contenu |
|----------|---------|
| [MWS - Protocole Relay](../protocole/MWS%20-%20Protocole%20Relay.md) | Format binaire, types de messages, séquences d'échange |

### Administration

| Document | Contenu |
|----------|---------|
| [MWS - MiyukiniAdmin](../administration/MWS%20-%20MiyukiniAdmin.md) | **ADMIN** — Panneau d'administration Origin : authentification, batterie de tests, monitoring, gestion services MWS |

### Déploiement

| Document | Contenu |
|----------|---------|
| [MWS - Guide de Déploiement](../deploiement/MWS%20-%20Guide%20de%20Deploiement.md) | Installation, TLS, systemd, monitoring, sécurité |
| [MWS - Implémentation Origin Hostinger](../deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md) | **IMPLEMENTATION** — Origin sur VPS Hostinger (Debian 13), config complète, MiyukiniAdmin, manifeste, nginx |

---

## 2. Références conceptuelles (`docs/reference/`)

### Système MWS

| Document | Contenu |
|----------|---------|
| [Miyukini Webway System](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) | Définition MWS, acteurs (participant, Tracker), annonces, listes de statuts |
| [Miyukini Webway System Complet](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Complet.md) | Document maître consolidé : vue d'ensemble, protocole, relay, sécurité |
| [Miyukini Webway System Normes et Standards](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) | Formats, norme de déclaration sécurisée, ports, matrice des statuts |
| [Miyukini Webway System Outils et Opérateurs](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) | Outils et Opérateurs MWS (Strate 6 et 7) |

### Relay et Origin

| Document | Contenu |
|----------|---------|
| [Miyukini Webway Relay](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) | Architecture complète : Origin, relays, trackers ; flux de vérification ; Lobbys ; versioning ; Registre |
| [Miyukini Webway Relay Protocol](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) | Protocole binaire : messages, format des trames, TLS, handshake |

### Connexion et glossaire

| Document | Contenu |
|----------|---------|
| [Connexion Inter-COG](../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) | Visite gouvernée, Passeport, Permis de circulation, Bridge |
| [Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Termes officiels MWS (Origin, Lobby, Permis de circulation, Pool, etc.) |

---

## 3. Outils MWS (`docs/tools/`)

### MiyuWebwayTracker

| Document | Contenu |
|----------|---------|
| [Documentation Fondatrice](../../tools/MiyuWebwayTracker/MiyuWebwayTracker%20-%20Documentation%20Fondatrice.md) | Document fondateur du toolkit Tracker |
| [Reference Outils](../../tools/MiyuWebwayTracker/MiyuWebwayTracker%20-%20Reference%20Outils.md) | Référence technique |
| [Passive Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) | Préconditions, postconditions, invariants (systèmes passifs) |
| [Active Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) | Déclencheurs et actions (systèmes actifs) |
| [Tool Governance Compliance](../../tools/MiyuWebwayTracker/contracts/governance/MiyuWebwayTracker%20-%20Tool%20Governance%20Compliance%20Contract.md) | Conformité gouvernance |
| [Implementation Guidelines](../../tools/MiyuWebwayTracker/implementation/MiyuWebwayTracker%20-%20Reference%20Implementation%20Guidelines.md) | Guide d'implémentation |

### MiyuWebwayParticipant

| Document | Contenu |
|----------|---------|
| [Documentation Fondatrice](../../tools/MiyuWebwayParticipant/MiyuWebwayParticipant%20-%20Documentation%20Fondatrice.md) | Document fondateur du toolkit Participant |
| [Reference Outils](../../tools/MiyuWebwayParticipant/MiyuWebwayParticipant%20-%20Reference%20Outils.md) | Référence technique |
| [Tool Governance Compliance](../../tools/MiyuWebwayParticipant/contracts/governance/MiyuWebwayParticipant%20-%20Tool%20Governance%20Compliance%20Contract.md) | Conformité gouvernance |
| [Implementation Guidelines](../../tools/MiyuWebwayParticipant/implementation/MiyuWebwayParticipant%20-%20Reference%20Implementation%20Guidelines.md) | Guide d'implémentation |

---

## 4. Setup et déploiement (`docs/setup/`)

| Document | Contenu |
|----------|---------|
| [Webway Relay Deployment Guide](../../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) | Guide de déploiement complet (VM, TLS, systemd, tests) |
| [Hostinger VPS Origin Webway](../../setup/Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) | Instance Origin sur Hostinger VPS Debian 13 (Rust, systemd, ufw, monitoring) |

---

## 5. Matrice de couverture

| Sujet | Document principal | Références détaillées |
|-------|-------------------|----------------------|
| **Positionnement MWS** | Document Fondateur | Architecture, Consommation Strates |
| **Origin** | MWS - Origin | Relay (sections 1, 3, 6), MiyukiniAdmin |
| **Relays** | MWS - Relays | Relay, Relay Protocol |
| **Trackers** | MWS - Trackers | MiyuWebwayTracker (contrats) |
| **Passeport / Permis de circulation** | MWS - Passeport et Permis | Relay (accord relay), Tracker (contrôle tracker) |
| **Vérification** | MWS - Flux de Vérification | Relay (section 2) |
| **Quarantaine / Blacklist** | MWS - Quarantaine et Blacklist | Relay (sections 2.8, 2.9, 3.4) |
| **Chiffrement** | MWS - Chiffrement et TLS | Relay (sections 3.3, 10) |
| **Audit Sécurité** | MWS - Audit de Sécurité Complet | Contre-Mesures Prioritaires |
| **Lobbys** | MWS - Lobbys, Favoris et Amis | Relay (sections 8, 9) |
| **Protocole** | MWS - Protocole Relay | Relay Protocol |
| **Administration** | MWS - MiyukiniAdmin | Implémentation Origin Hostinger |
| **Déploiement** | MWS - Guide de Déploiement | Setup guides, Implémentation Origin Hostinger |

---

**Version :** 3.0  
**Dernière mise à jour :** 2026-02-13
