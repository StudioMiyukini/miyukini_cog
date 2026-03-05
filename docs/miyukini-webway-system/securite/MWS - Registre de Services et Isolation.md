# MWS â€” Registre de Services et Isolation

## Contexte

Le **Registre de Services** est la liste officielle de tous les Services autorisÃ©s sur le rÃ©seau MWS. Maintenu par **Origin**, il garantit que seuls des Services **vÃ©rifiÃ©s et rÃ©pertoriÃ©s** peuvent Ãªtre installÃ©s dans les COGs connectÃ©s. L'**isolation** est le mÃ©canisme appliquÃ© aux COGs possÃ©dant des Services non rÃ©pertoriÃ©s.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- Registre de Services : structure, services officiels, services tiers
- VÃ©rification des Services lors de l'enregistrement
- DÃ©tection des Services non rÃ©pertoriÃ©s
- Protocole d'isolation et de levÃ©e d'isolation
- Suivi des mises Ã  jour

---

## 1. Registre de Services

### 1.1 Principe fondamental

> **Un Service ne peut pas Ãªtre installÃ© dans un COG connectÃ© au Webway sans Ãªtre prÃ©sent dans le Registre de Services du Relay Origin.**

Le Registre garantit :

| Garantie | Description |
|----------|-------------|
| **AuthenticitÃ©** | Seuls les Services vÃ©rifiÃ©s sont acceptÃ©s |
| **IntÃ©gritÃ©** | Les checksums permettent de vÃ©rifier l'intÃ©gritÃ© |
| **TraÃ§abilitÃ©** | L'origine de chaque Service est connue |
| **CompatibilitÃ©** | Les versions compatibles avec les Cores sont documentÃ©es |

### 1.2 Structure du Registre

Le Registre est maintenu par **Origin** et contient deux catÃ©gories :

#### Services officiels Miyukini

| Champ | Description |
|-------|-------------|
| `service_id` | Identifiant unique (ex. `webway.tracker`, `bridge`) |
| `current_version` | Version courante officielle (`MAJOR.MINOR.PATCH`) |
| `min_version` | Version minimale acceptÃ©e sur le rÃ©seau |
| `checksum` | Hash SHA-256 du binaire/package |
| `signature` | **Signature Ed25519 ou GPG** du binaire (contremesure R-005 â€” supply chain). Obligatoire pour toute installation. |
| `signing_key_id` | RÃ©fÃ©rence Ã  la clÃ© publique de signature (Registre des clÃ©s). |
| `build_reproducible` | Optionnel : boolÃ©en + hash de build pour reproducible builds. |
| `download_url` | URL de tÃ©lÃ©chargement officielle Miyukini |
| `changelog_url` | URL du journal des modifications |
| `core_compatibility` | Liste des `core_version.MAJOR` compatibles |
| `status` | `ACTIVE`, `DEPRECATED`, `RETIRED` |

#### Services tiers rÃ©pertoriÃ©s

| Champ | Description |
|-------|-------------|
| `service_id` | Identifiant unique (prÃ©fixe `third.` ou namespace Ã©diteur) |
| `publisher` | Nom de l'Ã©diteur du service tiers |
| `official_source_url` | URL de la source officielle de l'Ã©diteur |
| `current_version` | DerniÃ¨re version connue dans le Registre |
| `checksum` | Hash SHA-256 de la version rÃ©pertoriÃ©e |
| `signature` | **Signature** du binaire par l'Ã©diteur (contremesure R-005). VÃ©rification obligatoire avant installation. |
| `signing_key` | ClÃ© publique de l'Ã©diteur (enregistrÃ©e et vÃ©rifiÃ©e par Origin). |
| `core_compatibility` | Liste des `core_version.MAJOR` compatibles |
| `review_status` | `APPROVED`, `PENDING_REVIEW`, `SUSPENDED` |
| `registration_date` | Date d'enregistrement dans le Registre |

**RÃ¨gle R-005 :** Avant d'installer un Service, le COG doit vÃ©rifier la signature du binaire avec la clÃ© publique enregistrÃ©e. En cas d'Ã©chec de vÃ©rification, l'installation est refusÃ©e.

### 1.3 Synchronisation

```mermaid
flowchart TB
    subgraph Origin["Origin"]
        REG[Registre maÃ®tre]
    end

    subgraph Relays["Relays"]
        R1[Cache local Relay A]
        R2[Cache local Relay B]
    end

    subgraph Trackers["Trackers"]
        T1[Cache local Tracker 1]
    end

    REG -->|Push/Pull| R1
    REG -->|Push/Pull| R2
    R1 -->|Propagation| T1
```

| MÃ©canisme | Description |
|-----------|-------------|
| **Push depuis Origin** | Origin pousse les mises Ã  jour vers les relays |
| **Pull pÃ©riodique** | Les relays interrogent Origin pÃ©riodiquement |
| **Cache local** | Chaque relay/tracker maintient un cache du Registre |

---

## 2. VÃ©rification lors de l'enregistrement

### 2.1 Processus

Quand un COG se prÃ©sente avec son `service_manifest` :

```mermaid
sequenceDiagram
    participant COG as COG
    participant R as Relay
    participant REG as Registre Origin

    COG->>R: REGISTER (service_manifest)
    loop Pour chaque service_id
        R->>R: Consulter cache local
        alt Cache manquant ou pÃ©rimÃ©
            R->>REG: REGISTRY_QUERY (service_id)
            REG->>R: REGISTRY_RESPONSE (status)
        end
        alt Service FOUND
            R->>R: Marquer OK
        else Service NOT_FOUND
            R->>R: Marquer non rÃ©pertoriÃ©
        else Service SUSPENDED
            R->>R: Marquer suspendu
        end
    end
    alt Tous les services OK
        R->>COG: REGISTER_OK
    else Un ou plusieurs non rÃ©pertoriÃ©s
        R->>COG: Isolation (raison)
    end
```

### 2.2 RÃ©sultats possibles

| Statut Registre | Action |
|-----------------|--------|
| `FOUND` + `ACTIVE` | Service acceptÃ© |
| `FOUND` + `DEPRECATED` | AcceptÃ© avec avertissement |
| `FOUND` + `SUSPENDED` | Service temporairement retirÃ© â†’ isolation |
| `NOT_FOUND` | Service non rÃ©pertoriÃ© â†’ isolation |

---

## 3. DÃ©tection des Services non rÃ©pertoriÃ©s

### 3.1 ScÃ©narios de dÃ©tection

| ScÃ©nario | Moment de dÃ©tection |
|----------|---------------------|
| **Installation hors ligne** | Le COG installe un Service sans connexion rÃ©seau, puis se prÃ©sente au relay |
| **Service retirÃ©** | Un Service prÃ©cÃ©demment rÃ©pertoriÃ© est retirÃ© du Registre |
| **ContrefaÃ§on** | Un service_id falsifiÃ© est dÃ©tectÃ© |
| **Erreur de configuration** | Un service_id mal orthographiÃ© ou invalide |

### 3.2 Informations journalisÃ©es

| Champ | Description |
|-------|-------------|
| `cog_id` | COG concernÃ© |
| `service_id` | Service non rÃ©pertoriÃ© |
| `detected_at` | Date de dÃ©tection |
| `source_ip` | Adresse IP du COG |
| `relay_id` | Relay ayant dÃ©tectÃ© |
| `reason` | `NOT_FOUND` ou `SUSPENDED` |

---

## 4. Protocole d'isolation

### 4.1 DÃ©finition

L'**isolation** est un Ã©tat oÃ¹ le COG est **exclu du maillage MWS actif** mais maintenu en **surveillance**. Il ne s'agit pas d'une quarantaine classique (non-conformitÃ©) mais d'une attente de mise en conformitÃ© du `service_manifest`.

### 4.2 Comportement du COG isolÃ©

| AutorisÃ© | Interdit |
|----------|----------|
| Recevoir des notifications | Participer aux annonces de prÃ©sence |
| Consulter le Registre | ApparaÃ®tre dans les rÃ©ponses de dÃ©couverte |
| Maintenir le tunnel (HEARTBEAT) | Recevoir des connexions inter-COG |
| Se re-enregistrer | Router des donnÃ©es vers d'autres COGs |

### 4.3 Ã‰tapes de l'isolation

```mermaid
sequenceDiagram
    participant COG as COG
    participant R as Relay
    participant T as Tracker
    participant User as Utilisateur

    R->>R: DÃ©tecter service non rÃ©pertoriÃ©
    R->>COG: REGISTER_OK (status = ISOLATED, reason)
    R->>T: ALERT (cog_id, unregistered_service)
    T->>T: Exclure des rÃ©ponses de dÃ©couverte
    R->>COG: Notification utilisateur
    COG->>User: Afficher notification
    Note over COG: Mode surveillance
    loop Surveillance
        COG->>R: HEARTBEAT
        R->>COG: HEARTBEAT_ACK
    end
```

### 4.4 Notification utilisateur

La notification envoyÃ©e Ã  l'utilisateur contient :

| Champ | Description |
|-------|-------------|
| **service_id** | Service non rÃ©pertoriÃ© |
| **Raison** | Absent du Registre de Services |
| **Actions recommandÃ©es** | |
| | 1. Soumettre le service au Registre (processus d'enregistrement tiers) |
| | 2. DÃ©sinstaller le service non rÃ©pertoriÃ© |
| | 3. Se re-enregistrer avec un manifest conforme |

---

## 5. LevÃ©e d'isolation

### 5.1 Conditions

L'isolation est levÃ©e **automatiquement** quand :

| Condition | Description |
|-----------|-------------|
| **Manifest corrigÃ©** | Le COG se re-enregistre avec un `service_manifest` entiÃ¨rement conforme |
| **Service ajoutÃ© au Registre** | Le service non rÃ©pertoriÃ© est ajoutÃ© au Registre (processus d'enregistrement tiers approuvÃ©) |

### 5.2 Processus de levÃ©e

```mermaid
sequenceDiagram
    participant COG as COG
    participant R as Relay
    participant T as Tracker

    COG->>R: REGISTER (manifest corrigÃ©)
    R->>R: VÃ©rifier tous les services dans le Registre
    alt Tous conformes
        R->>COG: REGISTER_OK (status = ACTIVE)
        R->>T: UPDATE (cog_id, status = ACTIVE)
        T->>T: RÃ©intÃ©grer dans les rÃ©ponses de dÃ©couverte
    else Encore non conforme
        R->>COG: REGISTER_OK (status = ISOLATED)
    end
```

---

## 6. Suivi des mises Ã  jour

### 6.1 MÃ©canismes

Le COG dispose de capacitÃ©s de **suivi des mises Ã  jour** :

| MÃ©canisme | Description |
|-----------|-------------|
| **VÃ©rification pÃ©riodique** | Le COG interroge le Registre pour comparer son manifest |
| **Notification push** | Le relay envoie `UPDATE_AVAILABLE` via le tunnel actif |
| **Registre local** | Le COG maintient un registre local des versions installÃ©es |

### 6.2 Contenu de UPDATE_AVAILABLE

| Champ | Description |
|-------|-------------|
| `service_id` | Service concernÃ© |
| `current_version` | Version installÃ©e sur le COG |
| `available_version` | Version disponible |
| `severity` | `critical`, `recommended`, `optional` |
| `download_url` | URL de tÃ©lÃ©chargement |
| `checksum` | Hash SHA-256 |
| `changelog_url` | Journal des modifications |

### 6.3 Mises Ã  jour critiques

Si une mise Ã  jour est marquÃ©e `critical` (faille de sÃ©curitÃ©) :

| DÃ©lai | Action |
|-------|--------|
| ImmÃ©diat | Signalement vers WorrySentinel |
| AprÃ¨s 24h | Throttling du COG |
| AprÃ¨s 72h | Isolation du COG (si configurable) |

> La dÃ©cision de mise Ã  jour reste **souveraine au COG**. Le relay/Tracker ne force pas la mise Ã  jour, mais applique des mesures progressives.

---

## 7. Enregistrement de services tiers

### 7.1 Processus

Pour qu'un service tiers soit ajoutÃ© au Registre :

1. **Soumission** : L'Ã©diteur soumet le service Ã  Origin avec documentation
2. **Audit** : Origin audite le service (sÃ©curitÃ©, conformitÃ©, compatibilitÃ©)
3. **Review** : Statut `PENDING_REVIEW` pendant l'audit
4. **Approbation** : Si approuvÃ© â†’ statut `APPROVED`
5. **Publication** : Le service est ajoutÃ© au Registre et synchronisÃ©

### 7.2 ResponsabilitÃ©s

| Acteur | ResponsabilitÃ© |
|--------|----------------|
| **Ã‰diteur** | Maintenir le service, fournir les mises Ã  jour |
| **Origin** | Auditer, approuver, maintenir le Registre |
| **Relay** | VÃ©rifier, rediriger vers la source officielle |
| **COG** | TÃ©lÃ©charger depuis la source officielle |

> **Important :** Le relay **ne distribue pas** les binaires tiers ; il redirige vers la source officielle de l'Ã©diteur.

---

## 8. SchÃ©ma rÃ©capitulatif

```
+--------------------------------+
|       REGISTRE DE SERVICES     |
|         (maintenu par Origin)  |
|--------------------------------|
| Services officiels Miyukini    |
| Services tiers rÃ©pertoriÃ©s     |
+--------------------------------+
              |
              | Synchronisation
              v
+--------------------------------+
|        RELAY / TRACKER         |
|--------------------------------|
| Cache local du Registre        |
| VÃ©rification lors de REGISTER  |
+--------------------------------+
              |
              | VÃ©rification
              v
+--------------------------------+
|             COG                |
|--------------------------------|
| service_manifest               |
| Tous les services doivent Ãªtre |
| prÃ©sents dans le Registre      |
+--------------------------------+
              |
              | Si non rÃ©pertoriÃ©
              v
+--------------------------------+
|          ISOLATION             |
|--------------------------------|
| Tunnel maintenu (surveillance) |
| Exclu du maillage actif        |
| Notification utilisateur       |
| LevÃ©e aprÃ¨s correction         |
+--------------------------------+
```

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](../acteurs/MWS%20-%20Origin.md)
- [MWS - Flux de VÃ©rification](../verification/MWS%20-%20Flux%20de%20Verification.md)
- [MWS - Contre-Mesures de SÃ©curitÃ©](./MWS%20-%20Contre-Mesures%20de%20Securite.md) â€” R-005
- [Miyukini Webway Relay](..//reference//_index.md) â€” section 6

---

**Version :** 2.0  
**Mise Ã  jour :** Signature des binaires (R-005)  
**Classification :** Documentation MWS â€” SÃ©curitÃ©

