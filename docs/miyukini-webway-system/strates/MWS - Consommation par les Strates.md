# MWS â€” Consommation par les strates

## Contexte

Le Miyukini Webway System (MWS) est **consommÃ© Ã  diffÃ©rentes parties par toutes les strates** de l'environnement Miyukini. Ce document dÃ©crit **comment** chaque strate (ou niveau) utilise le MWS : Cores, Outils, OpÃ©rateurs, Services, BondingBrother.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md).

## PortÃ©e / Scope

- Identification des **points de consommation** du MWS par strate (ou niveau logique).
- RÃ´le de chaque strate dans la prÃ©sence, la dÃ©couverte, les Lobbys, les permis de circulation et le transport.
- Pas de description dÃ©taillÃ©e des protocoles (voir rÃ©fÃ©rences MWS Normes et Standards, Relay, Protocol).

---

## 1. Cores (Strate 4)

Les **Cores** gouvernent la participation au Webway et la conformitÃ©.

| Consommation | Description |
|--------------|-------------|
| **Attestation d'environnement** | WorrySentinel (et autres Cores) produisent l'attestation signÃ©e (revue interne) utilisÃ©e par les relays pour la vÃ©rification de conformitÃ©. |
| **ClÃ© de conformitÃ© des Cores** | Les Cores fournissent la clÃ© cachÃ©e dans le code, transmise au relay (Phase A de vÃ©rification). |
| **Politique de prÃ©sence** | Les Cores dÃ©cident si le COG s'annonce, quelles surfaces exposer, quels Lobbys crÃ©er. |
| **Permis de circulation (accord relay)** | Obtenu auprÃ¨s d'Origin/relay aprÃ¨s vÃ©rification ; les Cores gÃ¨rent la prÃ©sentation du Passeport et la rÃ©ception du Permis. ContrÃ´le tracker par les trackers. |
| **SÃ©curitÃ© rÃ©seau** | Border Guard, WorrySentinel : listes de statuts, dÃ©cisions de confiance, rÃ©action aux alertes (quarantaine, blacklist). |

**Documentation dÃ©taillÃ©e :** [Miyukini Webway Relay](..//reference//_index.md) (sections 2, 3, 4).

---

## 2. Outils et Kits d'Outils (Strate 6)

Les **Outils** MWS implÃ©mentent les rÃ´les Tracker et Participant.

| Outil | Consommation du MWS |
|-------|---------------------|
| **MiyuWebwayTracker** | Expose le rÃ´le Tracker : port 21000 (dÃ©couverte), port 80 (catalogue et Lobbys) ; gÃ¨re pools par version, permis de circulation, whitelists/blacklists/quarantaines ; contrats passifs et actifs ; indique les chemins aux clients pour joindre les hÃ´tes. |
| **MiyuWebwayParticipant** | Annonces de prÃ©sence, dÃ©claration des surfaces (services, ports), crÃ©ation de Lobbys, dÃ©couverte des autres COGs, consommation des Lobbys (avec accord d'hÃ´te), favoris. |

**Documentation dÃ©taillÃ©e :**  
- [MiyuWebwayTracker - Documentation Fondatrice](../../tools/MiyuWebwayTracker/MiyuWebwayTracker%20-%20Documentation%20Fondatrice.md)  
- [MiyuWebwayParticipant - Documentation Fondatrice](../../tools/MiyuWebwayParticipant/MiyuWebwayParticipant%20-%20Documentation%20Fondatrice.md)  
- Contrats : [Passive Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md), [Active Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md).

---

## 3. OpÃ©rateurs et Services (Strates 7 et au-delÃ )

Les **OpÃ©rateurs** et **Services** utilisent le MWS pour s'exposer et pour consommer les services d'autres COGs.

| Consommation | Description |
|--------------|-------------|
| **Annonces** | DÃ©claration des services exposÃ©s, adresses (IP/ports), sessions hÃ©bergÃ©es (Host), conformÃ©ment Ã  la norme de dÃ©claration sÃ©curisÃ©e. |
| **Surfaces et Lobbys** | DÃ©claration des surfaces de connexion (quels services, quels ports, acceptation de connexions) ; crÃ©ation de Lobbys dans le catalogue des trackers. |
| **DÃ©couverte** | RequÃªtes de dÃ©couverte vers les trackers ; rÃ©ception des listes de COGs et Lobbys (filtrÃ©es par version des Cores, statuts). |
| **Connexion clientâ€“hÃ´te** | Le COG client suit les chemins indiquÃ©s par le tracker, se connecte au COG hÃ´te, obtient l'accord d'hÃ´te et consomme les services exposÃ©s. |
| **Favoris et amis** | Gestion des favoris (retrouver un COG hÃ´te) et de la relation Â« amis Â» (contrÃ´les allÃ©gÃ©s, demande/confirmation humaine). |

**Documentation dÃ©taillÃ©e :** [Miyukini Webway Relay](..//reference//_index.md) (sections 8, 9), [Miyukini Webway System](..//reference//_index.md).

---

## 4. BondingBrother / Interfaces (Strate 5)

**BondingBrother** est le pont entre l'environnement et l'extÃ©rieur (autres COGs, rÃ©seau).

| Consommation | Description |
|--------------|-------------|
| **Intention de visite** | Exposition des intentions de visite (vers quels COGs, quels services) et rÃ©ception des rÃ©ponses de dÃ©couverte (chemins, Lobbys). |
| **Bridge** | Le point de contact (Bridge) est annoncÃ© sur le Webway ; les connexions entrantes passent par BondingBrother selon la gouvernance des Cores. |

**Documentation dÃ©taillÃ©e :** [Connexion Inter-COG](..//reference//_index.md).

---

## 5. Kernel (substrat)

Le **Kernel** fournit le substrat rÃ©seau (sockets, TLS, rÃ©solution) ; il ne contient pas de logique mÃ©tier MWS. Les **Outils** et **Cores** utilisent ce substrat pour communiquer avec les trackers, relays et autres COGs.

---

## 6. SynthÃ¨se

| Strate / Niveau | Type de consommation |
|-----------------|----------------------|
| **Cores** | Gouvernance, attestation, clÃ© de conformitÃ©, politique de prÃ©sence, rÃ©action aux alertes. |
| **Outils (MiyuWebway*)** | ImplÃ©mentation Tracker et Participant ; catalogue, Lobbys, chemins, contrats. |
| **OpÃ©rateurs / Services** | Annonces, surfaces, Lobbys, dÃ©couverte, connexion clientâ€“hÃ´te, favoris, amis. |
| **BondingBrother** | Pont et Bridge ; intentions de visite, point de contact annoncÃ©. |
| **Kernel** | Substrat rÃ©seau (pas de logique MWS). |

---

**Version :** 1.0  
**RÃ©fÃ©rences :** [Document Fondateur](../MWS%20-%20Document%20Fondateur.md), [Architecture et Subordination](../architecture/MWS%20-%20Architecture%20et%20Subordination%20aux%20Cores.md), [Index des rÃ©fÃ©rences](../reference/_index.md)

