# MWS â€” Passeport COG et Permis de Circulation

## Contexte

Le **Passeport COG** et le **Permis de circulation** sont les deux documents fondamentaux qui permettent Ã  un COG de participer au rÃ©seau Miyukini Webway. Le Passeport est l'identitÃ© complÃ¨te du COG ; le Permis de circulation (accord relay) est l'autorisation de circuler sur le rÃ©seau, dÃ©livrÃ©e par un relay aprÃ¨s vÃ©rification de conformitÃ©, et contrÃ´lÃ©e par les trackers (contrÃ´le tracker).

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- Structure et contenu du Passeport COG
- Types de COG (Origin, Relay, Tracker, Stable, Special, Terminal, Lone)
- Types d'OS (Windows, Linux, macOS, Android, iOS)
- Types de Passeport (Standard, SpÃ©cial)
- Relation parent-enfant (Terminal)
- Permis de circulation : Ã©mission (accord relay), contenu, durÃ©e
- Accord d'hÃ´te (distinct du Permis de circulation)
- Cycle de vie des documents

---

## 1. Passeport COG

### 1.1 DÃ©finition

Le **Passeport COG** est le document d'identitÃ© complet d'un COG. Il contient toutes les informations nÃ©cessaires pour que les relays puissent vÃ©rifier la conformitÃ© du COG.

### 1.2 Structure du Passeport

| Champ | Type | Description |
|-------|------|-------------|
| `cog_id` | string | Identifiant unique du COG (UUID ou LSI) |
| `cog_type` | enum | Type de COG : `ORIGIN`, `RELAY`, `TRACKER`, `STABLE`, `SPECIAL`, `TERMINAL`, `LONE` |
| `os_type` | enum | SystÃ¨me d'exploitation : `WINDOWS`, `LINUX`, `MACOS`, `ANDROID`, `IOS` |
| `core_version` | string | Version des Cores (`MAJOR.MINOR`, ex. `1.0`) |
| `service_list` | array | Liste des Services installÃ©s |
| `environment_health` | object | Rapport de santÃ© de l'environnement |
| `previous_permis` | array | Historique des Permis de circulation prÃ©cÃ©dents |
| `passport_type` | enum | `STANDARD` ou `SPECIAL` |
| `special_key` | string | (Passeports spÃ©ciaux uniquement) ClÃ© dÃ©livrÃ©e par Origin |
| `parent_cog_id` | string | (Terminaux uniquement) `cog_id` du COG Stable parent |

### 1.3 DÃ©tail des champs

#### cog_id

| PropriÃ©tÃ© | Description |
|-----------|-------------|
| **Format** | UUID v4 ou LSI (Local Sovereignty Identifier) |
| **UnicitÃ©** | Unique dans tout le rÃ©seau MWS |
| **ImmuabilitÃ©** | Ne change jamais pour un COG donnÃ© |

#### cog_type

Le **type de COG** dÃ©finit le rÃ´le et les capacitÃ©s du COG dans l'Ã©cosystÃ¨me MWS :

| Valeur | Description | CaractÃ©ristiques |
|--------|-------------|------------------|
| `ORIGIN` | Point central de vÃ©ritÃ© du MWS | Unique ; une seule adresse IP et/ou URL ; source de vÃ©ritÃ© pour tout le rÃ©seau |
| `RELAY` | COG de contrÃ´le d'intÃ©gritÃ© | VÃ©rification de conformitÃ© en 3 phases ; distribution des versions ; subordination Ã  Origin |
| `TRACKER` | Mapping et contrÃ´le | Douanier du rÃ©seau ; pools par version ; catalogue ; connexions inter-COG |
| `STABLE` | COG d'utilisateur commun | Usage courant ; environnement personnel ou professionnel standard |
| `SPECIAL` | COG professionnel Ã  forte utilisation | Fort trafic rÃ©seau et/ou services larges ; contrÃ´les allÃ©gÃ©s ; audit renforcÃ© |
| `TERMINAL` | COG embarquÃ© mobile | Enfant d'un COG Stable ; mÃªme utilisateur ; capacitÃ©s rÃ©duites ; dÃ©pendance au parent |
| `LONE` | COG isolÃ© volontairement | Structurellement et volontairement isolÃ© du rÃ©seau ; souverainetÃ© totale |

**RÃ¨gles par type :**

| Type | Connexion rÃ©seau | Passeport | ParticularitÃ©s |
|------|------------------|-----------|----------------|
| `ORIGIN` | Obligatoire (IP/URL fixe) | N/A (source) | Un seul Origin par rÃ©seau MWS |
| `RELAY` | Obligatoire | SPECIAL | Doit Ãªtre enregistrÃ© auprÃ¨s d'Origin |
| `TRACKER` | Obligatoire | SPECIAL | Doit Ãªtre listÃ© par Origin |
| `STABLE` | Optionnelle | STANDARD | Type par dÃ©faut pour utilisateurs |
| `SPECIAL` | Obligatoire | SPECIAL | Audit prÃ©alable par Origin |
| `TERMINAL` | Via parent | STANDARD | `parent_cog_id` obligatoire |
| `LONE` | Aucune | N/A | Pas de participation au MWS |

#### os_type

Le **type de systÃ¨me d'exploitation** sur lequel le COG s'exÃ©cute :

| Valeur | Description |
|--------|-------------|
| `WINDOWS` | Microsoft Windows (10, 11, Server) |
| `LINUX` | Distributions Linux (Ubuntu, Debian, Fedora, etc.) |
| `MACOS` | Apple macOS |
| `ANDROID` | Google Android (pour COGs TERMINAL) |
| `IOS` | Apple iOS (pour COGs TERMINAL) |

**RÃ¨gles par OS :**

| OS | Types de COG autorisÃ©s | Notes |
|----|------------------------|-------|
| `WINDOWS` | STABLE, SPECIAL, LONE | Environnement desktop/serveur |
| `LINUX` | Tous types | Environnement serveur privilÃ©giÃ© pour ORIGIN, RELAY, TRACKER |
| `MACOS` | STABLE, SPECIAL, LONE | Environnement desktop |
| `ANDROID` | TERMINAL, LONE | Mobile uniquement ; doit avoir un parent STABLE |
| `IOS` | TERMINAL, LONE | Mobile uniquement ; doit avoir un parent STABLE |

#### core_version

| PropriÃ©tÃ© | Description |
|-----------|-------------|
| **Format** | `MAJOR.MINOR` (ex. `1.0`, `2.3`) |
| **Signification MAJOR** | Version majeure des Cores ; dÃ©termine la compatibilitÃ© inter-COG |
| **Signification MINOR** | Ajustements internes compatibles |
| **ImmuabilitÃ©** | Les Cores sont immuables Ã  version donnÃ©e |

#### service_list

Liste des Services installÃ©s sur le COG :

| Champ | Description |
|-------|-------------|
| `service_id` | Identifiant unique du Service |
| `version` | Version du Service (`MAJOR.MINOR.PATCH`) |
| `checksum` | Hash SHA-256 du binaire/package |

Exemple :
```json
[
  {"service_id": "webway.tracker", "version": "1.2.3", "checksum": "abc123..."},
  {"service_id": "bridge", "version": "2.0.1", "checksum": "def456..."}
]
```

#### environment_health

Rapport de santÃ© gÃ©nÃ©rÃ© par les Cores (WorrySentinel, KindMother) :

| Champ | Description |
|-------|-------------|
| `storage_integrity` | IntÃ©gritÃ© du stockage (OK / DEGRADED / CORRUPTED) |
| `config_valid` | Configuration valide (true / false) |
| `strata_intact` | Strates intactes (true / false) |
| `attestation_signature` | Signature du rapport par WorrySentinel |
| `generated_at` | Date de gÃ©nÃ©ration |

#### previous_permis

Historique des Permis de circulation prÃ©cÃ©dents :

| Champ | Description |
|-------|-------------|
| `permis_id` | Identifiant du Permis de circulation |
| `issued_by` | Relay ou Origin Ã©metteur |
| `issued_at` | Date d'Ã©mission |
| `expired_at` | Date d'expiration |
| `scope` | PortÃ©e du Permis |

#### passport_type

| Valeur | Description |
|--------|-------------|
| `STANDARD` | Passeport standard, contrÃ´les normaux |
| `SPECIAL` | Passeport spÃ©cial (professionnel/fort trafic), contrÃ´les allÃ©gÃ©s au quotidien |

---

## 2. Types de COG

### 2.1 Vue d'ensemble des types

```mermaid
flowchart TB
    subgraph Infrastructure["Infrastructure MWS"]
        ORIGIN[ORIGIN<br>Source de vÃ©ritÃ©]
        RELAY[RELAY<br>ContrÃ´le d'intÃ©gritÃ©]
        TRACKER[TRACKER<br>Mapping et contrÃ´le]
    end

    subgraph Utilisateurs["COGs Utilisateurs"]
        STABLE[STABLE<br>Utilisateur commun]
        SPECIAL[SPECIAL<br>Professionnel]
        TERMINAL[TERMINAL<br>Mobile]
        LONE[LONE<br>IsolÃ©]
    end

    ORIGIN --> RELAY
    RELAY --> TRACKER
    TRACKER --> STABLE
    TRACKER --> SPECIAL
    STABLE --> TERMINAL
    LONE -.->|IsolÃ©| LONE
```

### 2.2 COG Origin

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **RÃ´le** | Point central de vÃ©ritÃ© unique du MWS |
| **UnicitÃ©** | Un seul Origin par rÃ©seau MWS |
| **Adressage** | Une seule adresse IP fixe et/ou une URL unique |
| **Fonctions** | Relay + Tracker + Registre de Services + Politiques |
| **OS recommandÃ©** | `LINUX` (serveur) |
| **Passeport** | N/A â€” Origin est la source, pas un participant |

### 2.3 COG Relay

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **RÃ´le** | Duplication d'Origin pour contrÃ´le d'intÃ©gritÃ© |
| **Fonctions** | VÃ©rification 3 phases, distribution des versions |
| **Subordination** | Ã€ Origin uniquement |
| **Enregistrement** | Doit Ãªtre enregistrÃ© et approuvÃ© par Origin |
| **OS recommandÃ©** | `LINUX` (serveur) |
| **Passeport** | SPECIAL (dÃ©livrÃ© par Origin) |

### 2.4 COG Tracker

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **RÃ´le** | Douanier du rÃ©seau â€” mapping et contrÃ´le |
| **Fonctions** | Pools par version, catalogue, connexions inter-COG |
| **Subordination** | Ã€ Origin et Relays |
| **Enregistrement** | Doit Ãªtre listÃ© par Origin dans la liste des trackers officiels |
| **OS recommandÃ©** | `LINUX` (serveur) |
| **Passeport** | SPECIAL (dÃ©livrÃ© par Origin) |

### 2.5 COG Stable

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **RÃ´le** | COG d'utilisateur commun |
| **Fonctions** | Usage personnel ou professionnel standard |
| **Connexion** | Optionnelle (peut fonctionner hors rÃ©seau) |
| **Terminaux** | Peut avoir des COGs TERMINAL enfants |
| **OS supportÃ©s** | `WINDOWS`, `LINUX`, `MACOS` |
| **Passeport** | STANDARD (automatique) |

### 2.6 COG Special

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **RÃ´le** | COG professionnel Ã  forte utilisation rÃ©seau |
| **Fonctions** | Services larges, fort trafic, haute disponibilitÃ© |
| **Connexion** | Obligatoire |
| **ContrÃ´les** | AllÃ©gÃ©s au quotidien, audits renforcÃ©s pÃ©riodiques |
| **OS supportÃ©s** | `WINDOWS`, `LINUX`, `MACOS` |
| **Passeport** | SPECIAL (aprÃ¨s audit Origin) |

### 2.7 COG Terminal

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **RÃ´le** | COG embarquÃ© sur mobile, extension d'un COG Stable |
| **ParentÃ©** | Enfant obligatoire d'un COG Stable du mÃªme utilisateur |
| **Fonctions** | CapacitÃ©s rÃ©duites, synchronisation avec le parent |
| **Connexion** | Via le parent ou directe avec dÃ©pendance |
| **OS supportÃ©s** | `ANDROID`, `IOS` |
| **Passeport** | STANDARD (avec `parent_cog_id` obligatoire) |

#### Relation Parent-Enfant (Terminal)

```mermaid
sequenceDiagram
    participant T as COG Terminal (Android)
    participant S as COG Stable (Parent)
    participant R as Relay

    Note over T,S: Premier enregistrement
    T->>S: Demande de liaison (user_credentials)
    S->>S: VÃ©rifier identitÃ© utilisateur
    S->>T: Acceptation + parent_cog_id
    T->>T: Stocker parent_cog_id dans Passeport

    Note over T,R: VÃ©rification rÃ©seau
    T->>R: Passeport (avec parent_cog_id)
    R->>R: VÃ©rifier que parent_cog_id est un STABLE valide
    R->>R: VÃ©rifier mÃªme utilisateur
    alt Parent valide
        R->>T: Permis de circulation
    else Parent invalide/blacklistÃ©
        R->>T: Refus (parent_invalid)
    end
```

| RÃ¨gle | Description |
|-------|-------------|
| `parent_cog_id` obligatoire | Un TERMINAL doit toujours dÃ©clarer son parent |
| MÃªme utilisateur | Parent et enfant doivent appartenir au mÃªme utilisateur |
| Propagation blacklist | Si le parent est blacklistÃ©, tous ses terminaux le sont |
| Limite de terminaux | Un STABLE peut avoir au maximum 5 terminaux |

### 2.8 COG Lone

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **RÃ´le** | COG structurellement et volontairement isolÃ© |
| **Fonctions** | SouverainetÃ© totale, aucune dÃ©pendance rÃ©seau |
| **Connexion** | Aucune â€” refus explicite du MWS |
| **Cas d'usage** | Environnements air-gapped, donnÃ©es sensibles, souverainetÃ© absolue |
| **OS supportÃ©s** | Tous |
| **Passeport** | N/A â€” pas de participation au MWS |

> **Note :** Un COG Lone peut dÃ©cider de rejoindre le rÃ©seau ultÃ©rieurement en changeant son `cog_type` vers `STABLE` et en passant la vÃ©rification initiale.

---

## 3. Types de Passeport

### 3.1 Passeport Standard

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **Ã‰mission** | Automatique lors de la crÃ©ation du COG |
| **ContrÃ´les** | VÃ©rification complÃ¨te Ã  chaque prÃ©sentation |
| **Limite de connexions** | 100 connexions simultanÃ©es (hors ports 80/8080) |
| **Cas d'usage** | COGs personnels, petits services, usage courant |

### 3.2 Passeport SpÃ©cial

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **Ã‰mission** | Uniquement par Origin, aprÃ¨s audit prÃ©alable |
| **ID spÃ©ciale** | Identifiant renforcÃ© unique |
| **ClÃ© spÃ©ciale** | ClÃ© cryptographique attestant le statut |
| **ContrÃ´les quotidiens** | AllÃ©gÃ©s pour optimiser les performances |
| **ContrÃ´les pÃ©riodiques** | Audits renforcÃ©s planifiÃ©s |
| **Limite de connexions** | SupÃ©rieure (configurable) |
| **Cas d'usage** | Sites de grandes entreprises, serveurs MMO, services Ã  fort trafic |

### 3.3 Protocole de dÃ©livrance du Passeport SpÃ©cial

```mermaid
sequenceDiagram
    participant COG as COG
    participant O as Origin

    COG->>O: Demande de Passeport spÃ©cial
    O->>O: Audit du COG (historique, conformitÃ©, cas d'usage)
    alt Audit rÃ©ussi
        O->>O: GÃ©nÃ©ration de l'ID et clÃ© spÃ©ciales
        O->>COG: Passeport spÃ©cial dÃ©livrÃ©
        Note over COG,O: Audits pÃ©riodiques pour maintien du statut
    else Audit Ã©chouÃ©
        O->>COG: Demande refusÃ©e (raison)
    end
```

---

## 4. Permis de Circulation (accord relay)

### 4.1 DÃ©finition

Le **Permis de circulation** est l'autorisation officielle de circuler sur le rÃ©seau MWS. Il est dÃ©livrÃ© par un **relay** (ou Origin) aprÃ¨s vÃ©rification de conformitÃ© du COG (accord relay) et vÃ©rifiÃ© par les trackers (contrÃ´le tracker).

**ValiditÃ© et trackers officiels :**

- Le Permis de circulation dÃ©livrÃ© par un relay est **valable sur tout le rÃ©seau** accessible au COG qui le prÃ©sente (maillage MWS couvert par Origin et les relays).
- Lors de la dÃ©livrance du Permis, le relay remet au COG les **adresses des trackers sÃ»rs/officiels** (trackers connus et reconnus par Origin). Le COG ne peut et **ne doit pas** se connecter Ã  un tracker inconnu d'Origin : seuls les trackers figurant sur cette liste sont autorisÃ©s pour la connexion au maillage.

### 4.2 Structure du Permis de circulation

| Champ | Type | Description |
|-------|------|-------------|
| `permis_id` | string | Identifiant unique du Permis de circulation |
| `cog_id` | string | COG concernÃ© |
| `issued_by` | string | Relay ou Origin Ã©metteur |
| `issued_at` | datetime | Date et heure d'Ã©mission |
| `expires_at` | datetime | Date et heure d'expiration |
| `scope` | object | PortÃ©e du Permis |
| `core_version` | string | Version des Cores validÃ©e |
| `passport_type` | enum | `STANDARD` ou `SPECIAL` |
| `tracker_addresses` | array | Adresses des trackers officiels/sÃ»rs (remises par le relay avec le Permis ; le COG ne doit se connecter qu'Ã  ces trackers). |

### 4.3 PortÃ©e (scope) du Permis de circulation

Le champ `scope` dÃ©finit les **intentions** dÃ©clarÃ©es par le COG :

| Champ | Description |
|-------|-------------|
| `services_to_use` | Services que le COG souhaite consommer |
| `cogs_to_contact` | COGs que le COG souhaite joindre (optionnel) |
| `expose_services` | Services que le COG expose (si hÃ´te) |
| `accept_connections` | Accepte des connexions entrantes (true/false) |

### 4.4 DurÃ©e de validitÃ©

| Type de Passeport | DurÃ©e typique | Renouvellement |
|-------------------|---------------|----------------|
| Standard | 1 Ã  24 heures | Automatique Ã  expiration si toujours conforme |
| SpÃ©cial | Jusqu'Ã  7 jours | Renouvellement simplifiÃ© |

### 4.5 Ã‰mission du Permis de circulation (accord relay)

```mermaid
sequenceDiagram
    participant COG as COG
    participant R as Relay

    COG->>R: Passeport COG complet
    R->>R: Phase A : VÃ©rification clÃ© Cores
    R->>R: Phase B : VÃ©rification blocs de code Services
    R->>R: Phase C : VÃ©rification santÃ© environnement
    alt Conforme
        R->>COG: Permis de circulation (permis_id, expires_at, scope)
    else Non-conforme
        R->>COG: Quarantaine (durÃ©e, raison)
    end
```

---

## 5. Accord d'hÃ´te

### 5.1 DÃ©finition

Distinct du Permis de circulation, l'**accord d'hÃ´te** est dÃ©livrÃ© par un **COG hÃ´te** Ã  un COG client pour autoriser la consommation de services spÃ©cifiques.

### 5.2 Structure

| Champ | Type | Description |
|-------|------|-------------|
| `accord_id` | string | Identifiant unique (accord d'hÃ´te) |
| `client_cog_id` | string | COG client autorisÃ© |
| `host_cog_id` | string | COG hÃ´te |
| `services_authorized` | array | Services accessibles |
| `issued_at` | datetime | Date d'Ã©mission |
| `expires_at` | datetime | Date d'expiration |
| `lobby_id` | string | Lobby concernÃ© (optionnel) |

### 5.3 Distinction avec le Permis de circulation

| Aspect | Permis de circulation (accord relay) | Accord d'hÃ´te |
|--------|--------------------------------------|---------------|
| **Ã‰metteur** | Relay / Origin | COG hÃ´te |
| **Autorisation** | Circuler sur le rÃ©seau MWS | Consommer les services du hÃ´te |
| **VÃ©rification** | ConformitÃ© du COG (relay) ; contrÃ´le tracker | Autorisation du hÃ´te |
| **DurÃ©e** | Heures Ã  jours | Session ou dÃ©finie par le hÃ´te |

### 5.4 Flow de dÃ©livrance

```mermaid
sequenceDiagram
    participant Client as COG Client
    participant Host as COG HÃ´te

    Note over Client: PossÃ¨de un Permis de circulation valide
    Client->>Host: Demande d'accÃ¨s (Permis circulation, services souhaitÃ©s)
    Host->>Host: VÃ©rifier Permis de circulation
    Host->>Host: VÃ©rifier autorisation (Lobby, politique)
    alt AutorisÃ©
        Host->>Client: Accord d'hÃ´te (services_authorized)
        Client->>Host: Consommation des services
    else Non autorisÃ©
        Host->>Client: Refus (raison)
    end
```

---

## 6. Cycle de vie des documents

### 6.1 Cycle du Passeport

```mermaid
stateDiagram-v2
    [*] --> CrÃ©Ã©: CrÃ©ation du COG
    CrÃ©Ã© --> Actif: PremiÃ¨re vÃ©rification OK
    Actif --> Quarantaine: Non-conformitÃ©
    Quarantaine --> Actif: ConformitÃ© restaurÃ©e
    Quarantaine --> BlacklistÃ©: 3 Ã©checs
    BlacklistÃ© --> Destruction: Auto-destruction
    Destruction --> [*]
    Actif --> SpÃ©cial: Demande approuvÃ©e par Origin
    SpÃ©cial --> Actif: Statut rÃ©voquÃ©
```

### 6.2 Cycle du Permis de circulation

```mermaid
stateDiagram-v2
    [*] --> Demande: PrÃ©sentation au relay
    Demande --> Ã‰mis: VÃ©rification OK
    Demande --> RefusÃ©: Non-conformitÃ©
    Ã‰mis --> Valide: Dans la pÃ©riode de validitÃ©
    Valide --> ExpirÃ©: Fin de validitÃ©
    ExpirÃ© --> RenouvelÃ©: Re-vÃ©rification OK
    Valide --> RÃ©voquÃ©: Alerte sÃ©curitÃ©
    RÃ©voquÃ© --> [*]
    RenouvelÃ© --> Valide
```

---

## 7. ContrÃ´le tracker : vÃ©rification des Permis de circulation

### 7.1 Points de vÃ©rification (contrÃ´le tracker)

Quand un COG se prÃ©sente Ã  un Tracker :

| VÃ©rification | Description |
|--------------|-------------|
| **Existence** | Le Permis de circulation existe-t-il ? |
| **Expiration** | Le Permis n'est-il pas expirÃ© ? |
| **Ã‰metteur** | Le relay Ã©metteur est-il reconnu ? |
| **CohÃ©rence** | Le scope est-il cohÃ©rent avec la requÃªte ? |
| **Blacklist** | Le `cog_id` n'est-il pas blacklistÃ© ? |

### 7.2 Actions selon le rÃ©sultat

| RÃ©sultat | Action |
|----------|--------|
| Permis valide | Accepter la connexion, assigner au pool |
| Permis expirÃ© | Rediriger vers relay pour renouvellement |
| Permis invalide | Refuser, journaliser, potentiel signalement |
| COG blacklistÃ© | Refuser, ignorer le Permis |

---

## 8. Exemples de Passeport et Permis de circulation

### 8.1 Exemple de Passeport Standard (COG Stable)

```json
{
  "cog_id": "550e8400-e29b-41d4-a716-446655440000",
  "cog_type": "STABLE",
  "os_type": "WINDOWS",
  "core_version": "1.0",
  "service_list": [
    {"service_id": "webway.participant", "version": "1.2.0", "checksum": "sha256:abc..."},
    {"service_id": "bridge", "version": "1.0.0", "checksum": "sha256:def..."}
  ],
  "environment_health": {
    "storage_integrity": "OK",
    "config_valid": true,
    "strata_intact": true,
    "attestation_signature": "sig:...",
    "generated_at": "2026-02-13T10:00:00Z"
  },
  "previous_permis": [
    {"permis_id": "permis-001", "issued_by": "relay-eu", "issued_at": "2026-02-12T08:00:00Z"}
  ],
  "passport_type": "STANDARD",
  "special_key": null,
  "parent_cog_id": null
}
```

### 8.2 Exemple de Passeport Terminal (COG mobile enfant)

```json
{
  "cog_id": "660f9500-f30c-52e5-b827-557766550111",
  "cog_type": "TERMINAL",
  "os_type": "ANDROID",
  "core_version": "1.0",
  "service_list": [
    {"service_id": "webway.participant.lite", "version": "1.2.0", "checksum": "sha256:ghi..."}
  ],
  "environment_health": {
    "storage_integrity": "OK",
    "config_valid": true,
    "strata_intact": true,
    "attestation_signature": "sig:...",
    "generated_at": "2026-02-13T10:00:00Z"
  },
  "previous_permis": [],
  "passport_type": "STANDARD",
  "special_key": null,
  "parent_cog_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### 8.3 Exemple de Passeport SpÃ©cial (COG serveur professionnel)

```json
{
  "cog_id": "770a0600-g41d-63f6-c938-668877661222",
  "cog_type": "SPECIAL",
  "os_type": "LINUX",
  "core_version": "1.0",
  "service_list": [
    {"service_id": "webway.participant", "version": "1.2.0", "checksum": "sha256:jkl..."},
    {"service_id": "game.server.mmo", "version": "3.5.2", "checksum": "sha256:mno..."},
    {"service_id": "chat.enterprise", "version": "2.1.0", "checksum": "sha256:pqr..."}
  ],
  "environment_health": {
    "storage_integrity": "OK",
    "config_valid": true,
    "strata_intact": true,
    "attestation_signature": "sig:...",
    "generated_at": "2026-02-13T10:00:00Z"
  },
  "previous_permis": [
    {"permis_id": "permis-sp-001", "issued_by": "origin", "issued_at": "2026-02-01T00:00:00Z"}
  ],
  "passport_type": "SPECIAL",
  "special_key": "sk_live_abc123xyz789...",
  "parent_cog_id": null
}
```

### 8.4 Exemple de Permis de circulation

```json
{
  "permis_id": "permis-002",
  "cog_id": "550e8400-e29b-41d4-a716-446655440000",
  "cog_type": "STABLE",
  "os_type": "WINDOWS",
  "issued_by": "relay-eu",
  "issued_at": "2026-02-13T10:05:00Z",
  "expires_at": "2026-02-14T10:05:00Z",
  "scope": {
    "services_to_use": ["game.server", "chat"],
    "cogs_to_contact": [],
    "expose_services": ["game.server"],
    "accept_connections": true
  },
  "core_version": "1.0",
  "passport_type": "STANDARD",
  "tracker_addresses": [
    {"host": "tracker-eu.miyukini.com", "port": 21000},
    {"host": "tracker-us.miyukini.com", "port": 21000}
  ]
}
```

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Flux de VÃ©rification](./MWS%20-%20Flux%20de%20Verification.md)
- [MWS - Relays](../acteurs/MWS%20-%20Relays.md)
- [Miyukini Webway Relay](..//reference//_index.md) â€” sections 2.2 Ã  2.7

---

**Version :** 2.0  
**Mise Ã  jour :** Ajout cog_type, os_type, relation Terminal-Stable  
**Classification :** Documentation MWS â€” VÃ©rification

