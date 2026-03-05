# MWS â€” Relays (Duplications d'Origin)

## Contexte

Les **relays** sont des **duplications d'Origin**, placÃ©s sous l'autoritÃ© d'Origin. Ils garantissent la **conformitÃ© en substance** des COGs, la **maintenance des environnements** et la **distribution des versions** (mises Ã  jour). Chaque relay hÃ©berge une copie de la vÃ©ritÃ© d'Origin et peut effectuer les mÃªmes vÃ©rifications de conformitÃ©.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- Position des relays dans l'architecture MWS
- Fonctions de vÃ©rification de conformitÃ©
- Distribution des versions et mises Ã  jour
- Transport et routing des tunnels
- Synchronisation avec Origin
- Services web des relays

---

## 1. Position dans l'architecture

Les relays sont des **extensions d'Origin** rÃ©parties gÃ©ographiquement ou par capacitÃ©. Ils permettent de :

- **DÃ©charger Origin** : absorber le trafic de vÃ©rification
- **Rapprocher gÃ©ographiquement** : rÃ©duire la latence pour les COGs
- **Assurer la continuitÃ©** : maintenir le service si Origin est temporairement saturÃ©

```mermaid
flowchart TB
    subgraph Origin["Origin"]
        O[Origin]
    end

    subgraph Relays["Relays (duplications)"]
        R1[Relay A<br>Europe]
        R2[Relay B<br>AmÃ©rique]
        R3[Relay C<br>Asie]
    end

    subgraph COGs["COGs"]
        C1[COG EU]
        C2[COG US]
        C3[COG ASIA]
    end

    O -->|VÃ©ritÃ©| R1
    O -->|VÃ©ritÃ©| R2
    O -->|VÃ©ritÃ©| R3

    C1 -->|VÃ©rification| R1
    C2 -->|VÃ©rification| R2
    C3 -->|VÃ©rification| R3
```

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **Subordination** | Tous les relays sont sous l'autoritÃ© d'Origin. |
| **VÃ©ritÃ© hÃ©ritÃ©e** | Chaque relay possÃ¨de une copie synchronisÃ©e de la vÃ©ritÃ© d'Origin. |
| **Autonomie de vÃ©rification** | Un relay peut vÃ©rifier un COG sans contacter Origin en temps rÃ©el. |
| **Pas de divergence** | Un relay ne peut pas modifier la vÃ©ritÃ© ; il l'applique telle que reÃ§ue d'Origin. |

---

## 2. Fonctions de vÃ©rification

Les relays effectuent la **vÃ©rification de conformitÃ©** des COGs selon le mÃªme protocole qu'Origin.

### 2.1 RÃ©ception du Passeport COG

Le COG transmet son **Passeport complet** :

| Champ | Description |
|-------|-------------|
| `cog_id` | Identifiant unique du COG |
| `core_version` | Version des Cores (`MAJOR.MINOR`) |
| `service_list` | Services installÃ©s avec versions et checksums |
| `environment_health` | Rapport de santÃ© (WorrySentinel, KindMother) |
| `previous_permis` | Historique des Permis de circulation prÃ©cÃ©dents |
| `passport_type` | `STANDARD` ou `SPECIAL` |
| `special_key` | (Passeports spÃ©ciaux) ClÃ© dÃ©livrÃ©e par Origin |

### 2.2 VÃ©rification en trois phases

#### Phase A : ClÃ© de conformitÃ© des Cores

| Ã‰tape | Description |
|-------|-------------|
| Le relay possÃ¨de la clÃ© attendue | HÃ©ritÃ©e d'Origin, stockÃ©e dans le cache du relay |
| Le COG transmet la clÃ© cachÃ©e | GÃ©nÃ©rÃ©e par les Cores, non accessible de l'extÃ©rieur |
| **Concordance** | Cores authentiques â†’ passer Ã  Phase B |
| **Discordance** | Cores potentiellement corrompus â†’ quarantaine |

#### Phase B : Blocs de code des Services

1. Le relay sÃ©lectionne un **bloc de code alÃ©atoire** (au sens MIP) pour chaque Service.
2. Le Service envoie un **paquet chiffrÃ©** contenant ce bloc.
3. Le relay **dÃ©chiffre** en utilisant les rÃ©fÃ©rences d'Origin.
4. **Concordance** â†’ Service authentique. **Discordance** â†’ Service suspect.
5. En cas de doute â†’ vÃ©rification Ã©tendue Ã  tout le code (sÃ©curitÃ© renforcÃ©e).

> **Note :** Si la version du Service est simplement en retard (valide mais non-courante), le relay Ã©met une **notification de mise Ã  jour**, pas une alerte de non-conformitÃ©.

#### Phase C : SantÃ© de l'environnement

Le relay vÃ©rifie le rapport `environment_health` :

| VÃ©rification | Description |
|--------------|-------------|
| IntÃ©gritÃ© du stockage | Pas de corruption dÃ©tectÃ©e par KindMother |
| Configuration cohÃ©rente | Strates intactes, configuration valide |
| Attestation signÃ©e | Rapport signÃ© par WorrySentinel |

### 2.3 RÃ©sultat de la vÃ©rification

| RÃ©sultat | Action |
|----------|--------|
| **Conforme** | Permis de circulation dÃ©livrÃ© (accord relay) |
| **Version en retard** | Notification de mise Ã  jour (pas d'alerte) |
| **Non-conforme** | Quarantaine (voir [MWS - Quarantaine et Blacklist](../securite/MWS%20-%20Quarantaine%20et%20Blacklist.md)) |

---

## 3. DÃ©livrance du Permis de circulation (accord relay)

En cas de conformitÃ©, le relay dÃ©livre un **Permis de circulation** (accord relay) :

| Champ | Description |
|-------|-------------|
| `permis_id` | Identifiant unique du permis |
| `cog_id` | COG concernÃ© |
| `issued_by` | Relay (ou Origin) Ã©metteur |
| `issued_at` | Date et heure d'Ã©mission |
| `expires_at` | Date et heure d'expiration |
| `scope` | PortÃ©e (intentions dÃ©clarÃ©es par le COG) |
| `core_version` | Version des Cores validÃ©e |
| `passport_type` | `STANDARD` ou `SPECIAL` |

Le COG peut alors se connecter au Webway par les **trackers officiels** dont les adresses sont remises avec le Permis (liste fournie par le relay, trackers connus d'Origin). Le Permis est valable sur tout le rÃ©seau accessible au COG. Un COG ne peut et ne doit pas se connecter Ã  un tracker inconnu d'Origin. Les trackers effectuent le contrÃ´le tracker (vÃ©rification du Permis de circulation) avant d'autoriser les connexions.

---

## 4. Distribution des versions et mises Ã  jour

Les relays sont la **source de distribution** des versions pour les COGs.

### 4.1 Liste officielle des Services

Chaque relay possÃ¨de une copie du **Registre de Services** d'Origin :

| Contenu | Description |
|---------|-------------|
| Services officiels | `service_id`, `current_version`, `min_version`, `checksum`, `download_url` |
| Services tiers | `service_id`, `publisher`, `official_source_url`, `review_status` |

### 4.2 Versions des Cores

| Contenu | Description |
|---------|-------------|
| `core_version` courante | DerniÃ¨re version stable |
| ClÃ©s de conformitÃ© | ClÃ©s attendues pour chaque version |
| Historique | Versions prÃ©cÃ©dentes et changelogs |

### 4.3 Notifications de mise Ã  jour

Le relay peut informer un COG d'une mise Ã  jour disponible :

| MÃ©canisme | Description |
|-----------|-------------|
| **Dans REGISTER_OK** | Inclure `UPDATE_RECOMMENDED` avec les versions disponibles |
| **Message UPDATE_AVAILABLE** | Notification push via le tunnel actif |

Contenu de la notification :

| Champ | Description |
|-------|-------------|
| `service_id` | Service concernÃ© |
| `current_version` | Version installÃ©e sur le COG |
| `available_version` | Version disponible |
| `severity` | `critical`, `recommended`, `optional` |
| `download_url` | URL de tÃ©lÃ©chargement |
| `checksum` | Hash SHA-256 |

### 4.4 Redirection vers les sources

| Type de service | Action du relay |
|-----------------|-----------------|
| Service officiel | Fournit l'URL de tÃ©lÃ©chargement Miyukini |
| Service tiers | Redirige vers la source officielle de l'Ã©diteur |

Le relay **ne distribue pas** les binaires tiers ; il redirige vers les sources officielles.

---

## 5. Transport et routing

### 5.1 Enregistrement du tunnel

```mermaid
sequenceDiagram
    participant COG as COG
    participant R as Relay :7000

    COG->>R: TCP (TLS) connect
    R->>COG: (TLS handshake)
    COG->>R: REGISTER (token, cog_id, Passeport)
    R->>R: VÃ©rification (Phase A, B, C)
    R->>COG: REGISTER_OK (permis_id, session_id)
    loop Persistance
        COG->>R: HEARTBEAT
        R->>COG: HEARTBEAT_ACK
    end
```

### 5.2 Table de routage

Le relay maintient une **table de routage** :

| ClÃ© | Valeur |
|-----|--------|
| `cog_id` | Tunnel actif, empreinte de version |

Lorsqu'une connexion arrive avec une cible `cog_id` :

1. Consulter la table de routage
2. Si tunnel enregistrÃ© â†’ transmettre les donnÃ©es
3. Sinon â†’ erreur (COG non enregistrÃ©)

### 5.3 Multi-COG et isolation

| Principe | Description |
|----------|-------------|
| **Multi-COG** | Plusieurs `cog_id` peuvent s'enregistrer sur le mÃªme relay |
| **Isolation stricte** | Le trafic d'un `cog_id` ne doit jamais Ãªtre routÃ© vers un autre |
| **Pas d'Ã©numÃ©ration** | Le relay ne rÃ©vÃ¨le pas la liste des `cog_id` enregistrÃ©s |

---

## 6. Synchronisation avec Origin

### 6.1 MÃ©canisme de synchronisation

| Aspect | Description |
|--------|-------------|
| **Push depuis Origin** | Origin pousse les mises Ã  jour (nouvelles versions, politiques) vers les relays |
| **Pull pÃ©riodique** | Les relays interrogent Origin pÃ©riodiquement pour vÃ©rifier les mises Ã  jour |
| **Invalidation** | Si une entrÃ©e du Registre est modifiÃ©e, Origin notifie tous les relays |

### 6.2 CohÃ©rence

| Principe | Description |
|----------|-------------|
| **CohÃ©rence Ã©ventuelle** | Les relays peuvent avoir un lÃ©ger retard sur Origin (quelques secondes) |
| **Pas de divergence** | Un relay ne peut pas avoir une vÃ©ritÃ© diffÃ©rente ; seulement en retard |
| **Fallback vers Origin** | En cas de doute, le relay peut interroger Origin en temps rÃ©el |

---

## 7. Services web des relays

Chaque relay expose un **serveur web** (port 80/443) :

| Contenu | Description |
|---------|-------------|
| **PrÃ©sentation du projet** | PrÃ©sentation globale de Miyukini COG |
| **Documentation** | Documentation officielle |
| **TÃ©lÃ©chargement** | Versions des COGs (hÃ©ritÃ©es d'Origin) |
| **Dev blog** | Blog de dÃ©veloppement |
| **Annonces globales** | Nouvelles versions, alertes |

Les relays sont **source de vÃ©ritÃ© pour ces contenus** mais restent **subordonnÃ©s Ã  Origin**.

---

## 8. SÃ©curitÃ© des relays

### 8.1 TLS obligatoire

| Exigence | Description |
|----------|-------------|
| TLS 1.2+ | Minimum TLS 1.2, recommandÃ© TLS 1.3 |
| PFS | Perfect Forward Secrecy obligatoire |
| Certificat validÃ© | SignÃ© par une CA reconnue |

### 8.2 Authentification

| MÃ©canisme | Description |
|-----------|-------------|
| Token 256+ bits | Entropie minimale pour l'authentification |
| Replay protection | Nonce + timestamp obligatoires |
| Rotation possible | Tokens rÃ©vocables et renouvelables |

### 8.3 Rate limiting

| Seuil | Description |
|-------|-------------|
| Enregistrements | Limiter par adresse source et par token |
| Connexions | Limiter les connexions simultanÃ©es |
| DÃ©bit par tunnel | Limiter bytes/s et connexions/s |

---

## 9. Ports et dÃ©ploiement

| Port | Usage |
|------|-------|
| **7000** | Protocole relay (TCP + TLS) |
| **80/443** | Services web (site, tÃ©lÃ©chargements) |

Voir [Miyukini - Webway Relay Deployment Guide](..//setup//Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) pour le dÃ©ploiement complet.

---

## 10. SchÃ©ma rÃ©capitulatif

```
+------------------------+
|        RELAY           |
|------------------------|
| VÃ‰RIFICATION :         |
| - Phase A : ClÃ© Cores  |
| - Phase B : Blocs MIP  |
| - Phase C : SantÃ© env. |
| - Permis de circulation  |
|------------------------|
| DISTRIBUTION :         |
| - Versions Cores       |
| - Services officiels   |
| - Redirection tiers    |
| - Notifications MAJ    |
|------------------------|
| TRANSPORT :            |
| - Table de routage     |
| - Multi-COG            |
| - Isolation stricte    |
|------------------------|
| SERVICES WEB :         |
| - Documentation        |
| - TÃ©lÃ©chargements      |
| - Annonces             |
+------------------------+
         ^
         | VÃ©ritÃ© hÃ©ritÃ©e
         |
+--------+--------+
|     ORIGIN      |
+-----------------+
```

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](./MWS%20-%20Origin.md)
- [MWS - Trackers](./MWS%20-%20Trackers.md)
- [Miyukini Webway Relay](..//reference//_index.md)
- [Miyukini Webway Relay Protocol](..//reference//_index.md)

---

**Version :** 1.0  
**Classification :** Documentation MWS â€” Acteurs


