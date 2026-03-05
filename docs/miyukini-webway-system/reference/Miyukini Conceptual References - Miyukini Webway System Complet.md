# Miyukini Conceptual References - Miyukini Webway System Complet

## Contexte

Ce document est le **document maÃ®tre consolidÃ©** du **Miyukini Webway System (MWS)**. Il offre une vision unifiÃ©e du systÃ¨me en un seul point d'entrÃ©e : vue d'ensemble, acteurs, protocole, relay, transport, dÃ©couverte, sÃ©curitÃ© et dÃ©ploiement. Il ne remplace pas les documents de rÃ©fÃ©rence dÃ©taillÃ©s mais en synthÃ©tise le contenu et en assure la navigation.

**Principe fondamental :**

> **Le Webway normalise la prÃ©sence et facilite l'Ã©change entre environnements ; il ne transporte pas la gouvernance ni les donnÃ©es â€” il permet de savoir oÃ¹ et comment initier une visite gouvernÃ©e.**

## PortÃ©e / Scope

- **EntrÃ©e unique** pour comprendre l'ensemble du MWS (prÃ©sence, dÃ©couverte, relay, sÃ©curitÃ©, dÃ©ploiement).
- **SynthÃ¨se** des trois rÃ©fÃ©rences conceptuelles MWS (document principal, Normes et Standards, Outils et OpÃ©rateurs) et du guide d'instance Hostinger VPS.
- **Diagrammes** : flux de connexion, topologie rÃ©seau, sÃ©quence d'enregistrement.
- **RÃ©fÃ©rences croisÃ©es** vers tous les documents MWS et connexes.

Ce document **ne remplace pas** les spÃ©cifications dÃ©taillÃ©es ; pour les normes, formats, contrats et implÃ©mentations, se reporter aux documents indiquÃ©s dans les rÃ©fÃ©rences croisÃ©es.

---

## 1. Vue d'ensemble du Miyukini Webway System

### 1.1 RÃ´le du MWS

Le **Miyukini Webway System (MWS)** est la couche de **prÃ©sence et de dÃ©couverte** des environnements COG disposant d'un accÃ¨s rÃ©seau. Il permet aux COGs de :

| CapacitÃ© | Description |
|----------|-------------|
| **Se dÃ©clarer** | Annoncer sa prÃ©sence (identitÃ© COG, adresse du Bridge / point de contact) |
| **DÃ©couvrir** | Savoir quels COGs sont prÃ©sents et oÃ¹ les joindre |
| **Faciliter l'Ã©change** | Donner le point d'entrÃ©e pour initier une visite gouvernÃ©e (Passeport â†’ Permis de circulation â†’ Bridge â†’ Visa de Connexion) |

**Le MWS ne sert pas Ã  transfÃ©rer des donnÃ©es mÃ©tier.** Il est la transcription concrÃ¨te des concepts de prÃ©sence : il normalise *qui est lÃ * et *oÃ¹ se prÃ©senter* pour demander un Permis de circulation (relay) ou un Visa de Connexion / accord d'hÃ´te (COG hÃ´te).

**Analogie :** Ã  la maniÃ¨re d'un rÃ©seau de type BitTorrent, les COGs peuvent s'annoncer et interroger des **Trackers** (points de rendez-vous pour la dÃ©couverte) ; le transfert rÃ©el et la gouvernance restent dans le cadre de la visite gouvernÃ©e (Bridge, Visa de Connexion).

### 1.2 Principes cardinaux

- **Le maillage ne fait pas confiance** â€” il transporte et expose des informations de prÃ©sence.
- **La gouvernance (Passeport, Permis de circulation, Visa de Connexion) reste souveraine** ; le Webway ne gouverne pas.
- **Optionnel** : les environnements sans rÃ©seau ou qui refusent la dÃ©couverte restent souverains (LOI-2, LOI-6).
- **Aucun core partagÃ©** : la prÃ©sence ne donne aucun accÃ¨s aux Cores ; elle indique oÃ¹ aller pour initier une visite.
- **Une seule gouvernance active** : c'est toujours le COG HÃ©bergeur qui dÃ©cide (Visa de Connexion / accord d'hÃ´te, refus, rÃ©vocation) ; Origin/relays pour le Permis de circulation.

---

## 2. Acteurs du Webway

### 2.1 COG participant (Webway Participant)

Tout COG qui choisit de participer au maillage MWS (accÃ¨s rÃ©seau et dÃ©claration activÃ©e).

**RÃ´le :** se dÃ©clarer auprÃ¨s d'un ou plusieurs Trackers, exposer les informations minimales de prÃ©sence (identitÃ© COG, adresse du Bridge), consulter la prÃ©sence d'autres COGs, participer Ã  l'Ã©change de listes de COGs avec statuts.

**ResponsabilitÃ©s :** ne pas exposer de donnÃ©es mÃ©tier ni de gouvernance via le Webway ; respecter les rÃ¨gles de sÃ©curitÃ© du maillage.

### 2.2 COG Tracker (Webway Tracker)

COG dont l'administrateur a choisi d'endosser le rÃ´le de **Tracker** : exposer volontairement une adresse pour servir de point de rendez-vous pour la dÃ©couverte.

**Port officiel :** les COGs Tracker MWS exposent leur endpoint sur le **port 21000** (voir [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) section 2.7.4).

**RÃ´le :** point de rendez-vous (enregistrement des annonces, rÃ©ponse aux requÃªtes de dÃ©couverte), **protection du rÃ©seau** par des mÃ©canismes **passifs** et **actifs**.

**Devoir fondamental :** les COGs Tracker ont le devoir de protÃ©ger le rÃ©seau par des systÃ¨mes passifs et actifs (observer, signaler, filtrer, rejeter selon les contrats dÃ©diÃ©s).

---

## 3. Protocole MWS

### 3.1 Types de messages

| Type | Direction | Description |
|------|-----------|-------------|
| `presence_announcement` | COG â†’ Tracker / maillage | Annonce de prÃ©sence |
| `service_announcement` | COG â†’ Tracker / maillage | Annonce de services et adresses |
| `host_session_declaration` | COG HÃ©bergeur â†’ Tracker / maillage | DÃ©claration d'hÃ©bergement de session |
| `discovery_request` | COG / Tracker â†’ Tracker | RequÃªte de dÃ©couverte |
| `discovery_response` | Tracker â†’ COG | RÃ©ponse (liste de COGs, services, sessions) |
| `cog_list` / `status_update` | COG â†” COG, Tracker â†” COG | Ã‰change de listes de statuts |

### 3.2 SÃ©quences typiques

- **Annonce :** le COG construit la dÃ©claration conforme (format, signature/intÃ©gritÃ©), envoie vers un ou plusieurs Trackers ; le(s) Tracker(s) vÃ©rifient conformitÃ© et intÃ©gritÃ©.
- **DÃ©couverte :** le COG envoie une `discovery_request` ; le(s) Tracker(s) rÃ©pondent par `discovery_response` en respectant les listes de statuts (ex. exclure Rejected).
- **Ã‰change de statuts :** les COGs et Trackers s'Ã©changent des `cog_list` ou `status_update` ; chaque COG met Ã  jour sa liste locale et applique ses rÃ¨gles (filtrer, dÃ©grader, rejeter).

DÃ©tails des formats, champs obligatoires/optionnels et ports : [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md).

---

## 4. Relay Webway

Le **relay Miyukini Webway** est un composant de **transport** (tunnel Ã©tendu multi-tenant) qui permet aux COGs derriÃ¨re NAT ou sans IP publique d'Ãªtre joignables : ils s'enregistrent auprÃ¨s du relay avec un **token d'authentification** et une adresse logique (`cog_id`), et le relay route le trafic entrant vers le bon tunnel.

### 4.1 RÃ´le du relay

- **Enregistrement** : un COG se connecte au relay (ex. `relay_host:7000`), s'authentifie par token/secret et enregistre son tunnel associÃ© Ã  son `cog_id`.
- **Routing** : le relay route les connexions entrantes (ou les donnÃ©es) vers le tunnel du COG concernÃ© (multi-COG, multi-service).
- **IntÃ©gration MWS** : les adresses annoncÃ©es sur le Webway peuvent Ãªtre **relay_host:port + token** (ou identifiant dÃ©rivÃ©), permettant Ã  d'autres COGs de joindre un COG via le relay sans exposition directe d'IP.

### 4.2 Port et dÃ©ploiement

- **Port relay (orientation)** : **7000** (TCP) â€” modifiable selon l'implÃ©mentation.
- **Port Tracker MWS** : **21000** (dÃ©couverte) â€” peut Ãªtre hÃ©bergÃ© sur la mÃªme machine que le relay (ex. VPS Hostinger).

Documentation dÃ©taillÃ©e du relay : [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) (architecture, sÃ©curitÃ©) et [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) (protocole binaire, handshake). Guide d'instance : [Hostinger VPS Origin Webway](..//setup//Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md). Guide de dÃ©ploiement complet : [Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md).

---

## 5. Transport

- **Bindings** : le protocole MWS ne impose pas un transport unique ; la norme peut spÃ©cifier un ou plusieurs bindings (transport + encodage). **TCP + TLS** est le binding principal recommandÃ© pour confidentialitÃ© et intÃ©gritÃ© en transit.
- **Port officiel Trackers** : **21000** (les participants se connectent aux Trackers sur `host:21000` par dÃ©faut).
- **Ports exclus** : les annonces MWS ne doivent pas utiliser les ports exclus (plage IANA 0â€“1023 + ports courants web/dev/DB) ; liste normative dans [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) section 2.7.
- **DÃ©couverte des Trackers** : par configuration locale, bootstrap ou annuaire connu ; port par dÃ©faut **21000**.

---

## 6. DÃ©couverte

- **Annonces** : prÃ©sence, services/adresses (IP/ports), dÃ©claration d'hÃ©bergement de session (Host) â€” format et norme de dÃ©claration sÃ©curisÃ©e dans [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md).
- **RequÃªtes** : un COG envoie une `discovery_request` (critÃ¨res : service_id, cog_id, sessions) ; les Trackers rÃ©pondent par `discovery_response` (COGs, services, sessions) en appliquant les listes de statuts (ex. exclure Rejected).
- **Outils et OpÃ©rateurs** : construction et envoi des requÃªtes/rÃ©ponses via les Outils MWS (Kit Participant, Kit Tracker) et les OpÃ©rateurs Participant Webway / Tracker Webway â€” voir [MWS Outils et OpÃ©rateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md).

---

## 7. SÃ©curitÃ©

### 7.1 Liste de COGs avec statuts (Webway COG List)

Chaque COG participant (et chaque Tracker) maintient une **liste de COGs** avec un **statut** par entrÃ©e : `cog_id`, `status`, `source`, `updated_at`. Les COGs s'Ã©changent des listes ou mises Ã  jour pour analyser et, le cas Ã©chÃ©ant, rejeter un COG ou une connexion considÃ©rÃ©e comme malveillante ou non fiable.

### 7.2 Statuts normatifs

| Statut | Code | Signification | Usage typique |
|--------|------|---------------|----------------|
| **Trusted** | `trusted` | COG fiable pour prÃ©sence/dÃ©couverte | Annonces acceptÃ©es, relayÃ©es |
| **Neutral** | `neutral` | Aucun signal positif ou nÃ©gatif | TraitÃ© par dÃ©faut |
| **Under review** | `under_review` | En cours d'analyse | Limitation ou surveillance |
| **Distrusted** | `distrusted` | COG non fiable | Annonces/connexions dÃ©gradÃ©es ou filtrÃ©es |
| **Rejected** | `rejected` | COG ou connexion rejetÃ©e | Refus d'annonce, blocage Webway |

RÃ¨gles de transition et comportement attendu par statut : [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) section 4.

### 7.3 Norme de dÃ©claration sÃ©curisÃ©e

Pour les annonces (prÃ©sence, services, adresses, sessions hÃ©bergÃ©es) : **authentification** de l'origine, **intÃ©gritÃ©** (signature/MAC), **format unifiÃ©**, **limitation des abus**. Cadre dÃ©fini dans [MWS](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) section 3.3 et [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) section 1.

### 7.4 SystÃ¨mes passifs et actifs des Trackers

- **Passifs** : observer, enregistrer, signaler, alimenter les listes de statuts â€” sans modifier le flux par eux-mÃªmes. Contrat : [MiyuWebwayTracker - Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md).
- **Actifs** : filtrer, dÃ©grader, rejeter (annonces/connexions) selon listes de statuts et politiques. Contrat : [MiyuWebwayTracker - Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md).

---

## 8. DÃ©ploiement

- **VPS Hostinger (Debian 13)** : instance pour hÃ©berger **Origin** (relay 7000, Tracker 21000, catalogue web). RÃ¨gles de sÃ©curitÃ© : ouvrir TCP 22, 80, 443, 7000, 21000. Voir [Miyukini - Hostinger VPS Origin Webway](..//setup//Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md).
- **Relay** : dÃ©ploiement du binaire relay (crate Rust), configuration TLS et authentification par token/secret, enregistrement des tunnels par `cog_id`. Guide de dÃ©ploiement complet : [Miyukini - Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md).
- **DNS (optionnel)** : nom de domaine pointant vers l'IP publique (ex. `webway.studiomiyukini.com`) pour une adresse stable du relay et du Tracker.

---

## 9. Diagrammes

### 9.1 Flux de connexion (MWS â†’ Visite gouvernÃ©e)

```mermaid
sequenceDiagram
    participant P as COG Participant
    participant T as COG Tracker
    participant H as COG HÃ©bergeur
    participant V as Visiteur

    P->>T: presence_announcement / service_announcement
    T->>T: Enregistrement, listes statuts
    V->>T: discovery_request
    T->>V: discovery_response (adresse Bridge H)
    V->>H: Passeport + Demande de Visite (Connexion Inter-COG)
    H->>H: Douane, Ã©mission Visa de Connexion / accord d'hÃ´te
    H->>V: Visa de Connexion, session gouvernÃ©e
```

### 9.2 Topologie rÃ©seau (Participants, Trackers, Relay)

```mermaid
flowchart LR
    subgraph COGs["COGs Participants"]
        P1[Participant 1]
        P2[Participant 2]
        P3[Participant 3]
    end

    subgraph Relay["Relay Webway"]
        R[Relay :7000]
    end

    subgraph Trackers["Trackers MWS"]
        T1[Tracker :21000]
    end

    P1 <-->|annonces / dÃ©couverte| T1
    P2 <-->|annonces / dÃ©couverte| T1
    P3 <-->|tunnel + token| R
    R -.->|routing cog_id| P3
    T1 -.->|dÃ©couverte relay_host:7000| P1
```

### 9.3 SÃ©quence d'enregistrement (Participant â†’ Tracker)

```mermaid
sequenceDiagram
    participant COG as COG Participant
    participant Op as OpÃ©rateur Participant Webway
    participant Tools as Outils MWS (build, sign, send)
    participant Tracker as COG Tracker :21000

    COG->>Op: Intention (annoncer prÃ©sence) via BondingBrother
    Op->>Tools: mws.declaration.build + sign
    Tools->>Op: message signÃ©
    Op->>Tools: mws.transport.send(message, tracker_host:21000)
    Tools->>Tracker: presence_announcement (TCP/TLS)
    Tracker->>Tracker: validate, verify, enregistrement
    Tracker-->>COG: (ack ou erreur selon protocole)
```

---

## 10. Relation avec la Connexion Inter-COG

Le MWS **ne remplace pas** la visite gouvernÃ©e ; il la **prÃ©cÃ¨de** et la **rend possible** en environnement connectÃ©.

| Ã‰tape | Couche | RÃ´le |
|-------|--------|------|
| 1 | **MWS** | DÃ©couverte : quels COGs sont prÃ©sents, oÃ¹ contacter le Bridge du COG HÃ©bergeur |
| 2 | **Connexion Inter-COG** | PrÃ©-validation locale (COG Origine), Ã©mission du Passeport Utilisateur |
| 3 | **Connexion Inter-COG** | PrÃ©sentation au Bridge (Passeport + Demande de Visite) |
| 4 | **Connexion Inter-COG** | Douane du Host COG, Ã©mission du Visa de Connexion / accord d'hÃ´te, session gouvernÃ©e |

RÃ©fÃ©rence : [Connexion Inter-COG](_index.md).

---

## 11. Positionnement dans l'architecture

- **Border Guard** : rÃ¨gles de qui peut s'annoncer ou interroger le maillage ; politique des listes de statuts.
- **Bridge inter-COG** : une fois l'adresse connue via le MWS, le Bridge reste le canal diplomatique ; le MWS ne remplace pas le Bridge.
- **WorrySentinel** : peut Ãªtre sollicitÃ© pour surveiller les signaux issus du Webway (statuts, alertes).
- **Outils et OpÃ©rateurs** : Kit Participant Webway, Kit Tracker Webway ; OpÃ©rateurs Participant Webway et Tracker Webway (Strates 6â€“7), gouvernÃ©s par les Cores. Voir [MWS Outils et OpÃ©rateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md).

Le MWS est une **couche de dÃ©couverte et de prÃ©sence** sous le contrÃ´le des Cores existants ; il n'introduit pas de nouveau Core mÃ©tier.

---

## RÃ©fÃ©rences croisÃ©es

### Documents MWS principaux

- [Miyukini Webway System](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) â€” document conceptuel principal (acteurs, annonces, sÃ©curitÃ©, devoir Trackers)
- [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) â€” formats, protocole, ports, matrice des statuts, conformitÃ© Trackers
- [Miyukini Webway System - Outils et OpÃ©rateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) â€” Outils MWS, Kits Participant/Tracker, OpÃ©rateurs

### Relay et dÃ©ploiement

- [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) â€” architecture du relay custom (tunnel Ã©tendu multi-tenant)
- [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) â€” protocole relay (messages, handshake, TLS)
- [Miyukini - Hostinger VPS Origin Webway](..//setup//Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) â€” instance Origin (Debian 13), Rust, systemd, ufw, monitoring
- [Miyukini - Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) â€” guide de dÃ©ploiement complet (VM, TLS, systemd, tests)

### Toolkits MWS

- [MiyuWebwayParticipant](../README.md) â€” Documentation fondatrice, rÃ©fÃ©rence outils, implÃ©mentation, gouvernance
- [MiyuWebwayTracker](../README.md) â€” Documentation fondatrice, rÃ©fÃ©rence outils, implÃ©mentation, gouvernance
- Contrats systÃ¨mes passifs/actifs Tracker : [Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md), [Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md)

### RÃ©fÃ©rences conceptuelles connexes

- [Connexion Inter-COG](_index.md)
- [Definition COG](_index.md)
- [Souverainete Environnement](_index.md)
- [Lois Autonomie Systeme](_index.md)
- [Doctrine Securite Fondamentale](_index.md)
- [Glossaire](_index.md) (Passeport Utilisateur, Visa de Connexion, Bridge inter-COG, COG Tracker, MWS)

---

*Document crÃ©Ã© le 12/02/2026*  
*Classification : Reference conceptuelle â€” Document maÃ®tre MWS (Miyukini Webway System Complet)*



