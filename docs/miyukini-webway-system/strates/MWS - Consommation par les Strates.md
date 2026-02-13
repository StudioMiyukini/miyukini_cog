# MWS — Consommation par les strates

## Contexte

Le Miyukini Webway System (MWS) est **consommé à différentes parties par toutes les strates** de l'environnement Miyukini. Ce document décrit **comment** chaque strate (ou niveau) utilise le MWS : Cores, Outils, Opérateurs, Services, BondingBrother.

**Référence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md).

## Portée / Scope

- Identification des **points de consommation** du MWS par strate (ou niveau logique).
- Rôle de chaque strate dans la présence, la découverte, les Lobbys, les permis de circulation et le transport.
- Pas de description détaillée des protocoles (voir références MWS Normes et Standards, Relay, Protocol).

---

## 1. Cores (Strate 4)

Les **Cores** gouvernent la participation au Webway et la conformité.

| Consommation | Description |
|--------------|-------------|
| **Attestation d'environnement** | WorrySentinel (et autres Cores) produisent l'attestation signée (revue interne) utilisée par les relays pour la vérification de conformité. |
| **Clé de conformité des Cores** | Les Cores fournissent la clé cachée dans le code, transmise au relay (Phase A de vérification). |
| **Politique de présence** | Les Cores décident si le COG s'annonce, quelles surfaces exposer, quels Lobbys créer. |
| **Permis de circulation (accord relay)** | Obtenu auprès d'Origin/relay après vérification ; les Cores gèrent la présentation du Passeport et la réception du Permis. Contrôle tracker par les trackers. |
| **Sécurité réseau** | Border Guard, WorrySentinel : listes de statuts, décisions de confiance, réaction aux alertes (quarantaine, blacklist). |

**Documentation détaillée :** [Miyukini Webway Relay](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) (sections 2, 3, 4).

---

## 2. Outils et Kits d'Outils (Strate 6)

Les **Outils** MWS implémentent les rôles Tracker et Participant.

| Outil | Consommation du MWS |
|-------|---------------------|
| **MiyuWebwayTracker** | Expose le rôle Tracker : port 21000 (découverte), port 80 (catalogue et Lobbys) ; gère pools par version, permis de circulation, whitelists/blacklists/quarantaines ; contrats passifs et actifs ; indique les chemins aux clients pour joindre les hôtes. |
| **MiyuWebwayParticipant** | Annonces de présence, déclaration des surfaces (services, ports), création de Lobbys, découverte des autres COGs, consommation des Lobbys (avec accord d'hôte), favoris. |

**Documentation détaillée :**  
- [MiyuWebwayTracker - Documentation Fondatrice](../../tools/MiyuWebwayTracker/MiyuWebwayTracker%20-%20Documentation%20Fondatrice.md)  
- [MiyuWebwayParticipant - Documentation Fondatrice](../../tools/MiyuWebwayParticipant/MiyuWebwayParticipant%20-%20Documentation%20Fondatrice.md)  
- Contrats : [Passive Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md), [Active Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md).

---

## 3. Opérateurs et Services (Strates 7 et au-delà)

Les **Opérateurs** et **Services** utilisent le MWS pour s'exposer et pour consommer les services d'autres COGs.

| Consommation | Description |
|--------------|-------------|
| **Annonces** | Déclaration des services exposés, adresses (IP/ports), sessions hébergées (Host), conformément à la norme de déclaration sécurisée. |
| **Surfaces et Lobbys** | Déclaration des surfaces de connexion (quels services, quels ports, acceptation de connexions) ; création de Lobbys dans le catalogue des trackers. |
| **Découverte** | Requêtes de découverte vers les trackers ; réception des listes de COGs et Lobbys (filtrées par version des Cores, statuts). |
| **Connexion client–hôte** | Le COG client suit les chemins indiqués par le tracker, se connecte au COG hôte, obtient l'accord d'hôte et consomme les services exposés. |
| **Favoris et amis** | Gestion des favoris (retrouver un COG hôte) et de la relation « amis » (contrôles allégés, demande/confirmation humaine). |

**Documentation détaillée :** [Miyukini Webway Relay](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) (sections 8, 9), [Miyukini Webway System](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md).

---

## 4. BondingBrother / Interfaces (Strate 5)

**BondingBrother** est le pont entre l'environnement et l'extérieur (autres COGs, réseau).

| Consommation | Description |
|--------------|-------------|
| **Intention de visite** | Exposition des intentions de visite (vers quels COGs, quels services) et réception des réponses de découverte (chemins, Lobbys). |
| **Bridge** | Le point de contact (Bridge) est annoncé sur le Webway ; les connexions entrantes passent par BondingBrother selon la gouvernance des Cores. |

**Documentation détaillée :** [Connexion Inter-COG](../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md).

---

## 5. Kernel (substrat)

Le **Kernel** fournit le substrat réseau (sockets, TLS, résolution) ; il ne contient pas de logique métier MWS. Les **Outils** et **Cores** utilisent ce substrat pour communiquer avec les trackers, relays et autres COGs.

---

## 6. Synthèse

| Strate / Niveau | Type de consommation |
|-----------------|----------------------|
| **Cores** | Gouvernance, attestation, clé de conformité, politique de présence, réaction aux alertes. |
| **Outils (MiyuWebway*)** | Implémentation Tracker et Participant ; catalogue, Lobbys, chemins, contrats. |
| **Opérateurs / Services** | Annonces, surfaces, Lobbys, découverte, connexion client–hôte, favoris, amis. |
| **BondingBrother** | Pont et Bridge ; intentions de visite, point de contact annoncé. |
| **Kernel** | Substrat réseau (pas de logique MWS). |

---

**Version :** 1.0  
**Références :** [Document Fondateur](../MWS%20-%20Document%20Fondateur.md), [Architecture et Subordination](../architecture/MWS%20-%20Architecture%20et%20Subordination%20aux%20Cores.md), [Index des références](../reference/_index.md)
