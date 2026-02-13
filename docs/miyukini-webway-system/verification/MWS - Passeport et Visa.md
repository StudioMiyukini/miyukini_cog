# MWS — Passeport COG et Permis de Circulation

## Contexte

Le **Passeport COG** et le **Permis de circulation** sont les deux documents fondamentaux qui permettent à un COG de participer au réseau Miyukini Webway. Le Passeport est l'identité complète du COG ; le Permis de circulation (accord relay) est l'autorisation de circuler sur le réseau, délivrée par un relay après vérification de conformité, et contrôlée par les trackers (contrôle tracker).

**Référence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## Portée / Scope

- Structure et contenu du Passeport COG
- Types de Passeport (Standard, Spécial)
- Permis de circulation : émission (accord relay), contenu, durée
- Accord d'hôte (distinct du Permis de circulation)
- Cycle de vie des documents

---

## 1. Passeport COG

### 1.1 Définition

Le **Passeport COG** est le document d'identité complet d'un COG. Il contient toutes les informations nécessaires pour que les relays puissent vérifier la conformité du COG.

### 1.2 Structure du Passeport

| Champ | Type | Description |
|-------|------|-------------|
| `cog_id` | string | Identifiant unique du COG (UUID ou LSI) |
| `core_version` | string | Version des Cores (`MAJOR.MINOR`, ex. `1.0`) |
| `service_list` | array | Liste des Services installés |
| `environment_health` | object | Rapport de santé de l'environnement |
| `previous_permis` | array | Historique des Permis de circulation précédents |
| `passport_type` | enum | `STANDARD` ou `SPECIAL` |
| `special_key` | string | (Passeports spéciaux uniquement) Clé délivrée par Origin |

### 1.3 Détail des champs

#### cog_id

| Propriété | Description |
|-----------|-------------|
| **Format** | UUID v4 ou LSI (Local Sovereignty Identifier) |
| **Unicité** | Unique dans tout le réseau MWS |
| **Immuabilité** | Ne change jamais pour un COG donné |

#### core_version

| Propriété | Description |
|-----------|-------------|
| **Format** | `MAJOR.MINOR` (ex. `1.0`, `2.3`) |
| **Signification MAJOR** | Version majeure des Cores ; détermine la compatibilité inter-COG |
| **Signification MINOR** | Ajustements internes compatibles |
| **Immuabilité** | Les Cores sont immuables à version donnée |

#### service_list

Liste des Services installés sur le COG :

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

Rapport de santé généré par les Cores (WorrySentinel, KeeperOfStorage) :

| Champ | Description |
|-------|-------------|
| `storage_integrity` | Intégrité du stockage (OK / DEGRADED / CORRUPTED) |
| `config_valid` | Configuration valide (true / false) |
| `strata_intact` | Strates intactes (true / false) |
| `attestation_signature` | Signature du rapport par WorrySentinel |
| `generated_at` | Date de génération |

#### previous_permis

Historique des Permis de circulation précédents :

| Champ | Description |
|-------|-------------|
| `permis_id` | Identifiant du Permis de circulation |
| `issued_by` | Relay ou Origin émetteur |
| `issued_at` | Date d'émission |
| `expired_at` | Date d'expiration |
| `scope` | Portée du Permis |

#### passport_type

| Valeur | Description |
|--------|-------------|
| `STANDARD` | Passeport standard, contrôles normaux |
| `SPECIAL` | Passeport spécial (professionnel/fort trafic), contrôles allégés au quotidien |

---

## 2. Types de Passeport

### 2.1 Passeport Standard

| Caractéristique | Description |
|-----------------|-------------|
| **Émission** | Automatique lors de la création du COG |
| **Contrôles** | Vérification complète à chaque présentation |
| **Limite de connexions** | 100 connexions simultanées (hors ports 80/8080) |
| **Cas d'usage** | COGs personnels, petits services, usage courant |

### 2.2 Passeport Spécial

| Caractéristique | Description |
|-----------------|-------------|
| **Émission** | Uniquement par Origin, après audit préalable |
| **ID spéciale** | Identifiant renforcé unique |
| **Clé spéciale** | Clé cryptographique attestant le statut |
| **Contrôles quotidiens** | Allégés pour optimiser les performances |
| **Contrôles périodiques** | Audits renforcés planifiés |
| **Limite de connexions** | Supérieure (configurable) |
| **Cas d'usage** | Sites de grandes entreprises, serveurs MMO, services à fort trafic |

### 2.3 Protocole de délivrance du Passeport Spécial

```mermaid
sequenceDiagram
    participant COG as COG
    participant O as Origin

    COG->>O: Demande de Passeport spécial
    O->>O: Audit du COG (historique, conformité, cas d'usage)
    alt Audit réussi
        O->>O: Génération de l'ID et clé spéciales
        O->>COG: Passeport spécial délivré
        Note over COG,O: Audits périodiques pour maintien du statut
    else Audit échoué
        O->>COG: Demande refusée (raison)
    end
```

---

## 3. Permis de Circulation (accord relay)

### 3.1 Définition

Le **Permis de circulation** est l'autorisation officielle de circuler sur le réseau MWS. Il est délivré par un **relay** (ou Origin) après vérification de conformité du COG (accord relay) et vérifié par les trackers (contrôle tracker).

**Validité et trackers officiels :**

- Le Permis de circulation délivré par un relay est **valable sur tout le réseau** accessible au COG qui le présente (maillage MWS couvert par Origin et les relays).
- Lors de la délivrance du Permis, le relay remet au COG les **adresses des trackers sûrs/officiels** (trackers connus et reconnus par Origin). Le COG ne peut et **ne doit pas** se connecter à un tracker inconnu d'Origin : seuls les trackers figurant sur cette liste sont autorisés pour la connexion au maillage.

### 3.2 Structure du Permis de circulation

| Champ | Type | Description |
|-------|------|-------------|
| `permis_id` | string | Identifiant unique du Permis de circulation |
| `cog_id` | string | COG concerné |
| `issued_by` | string | Relay ou Origin émetteur |
| `issued_at` | datetime | Date et heure d'émission |
| `expires_at` | datetime | Date et heure d'expiration |
| `scope` | object | Portée du Permis |
| `core_version` | string | Version des Cores validée |
| `passport_type` | enum | `STANDARD` ou `SPECIAL` |
| `tracker_addresses` | array | Adresses des trackers officiels/sûrs (remises par le relay avec le Permis ; le COG ne doit se connecter qu'à ces trackers). |

### 3.3 Portée (scope) du Permis de circulation

Le champ `scope` définit les **intentions** déclarées par le COG :

| Champ | Description |
|-------|-------------|
| `services_to_use` | Services que le COG souhaite consommer |
| `cogs_to_contact` | COGs que le COG souhaite joindre (optionnel) |
| `expose_services` | Services que le COG expose (si hôte) |
| `accept_connections` | Accepte des connexions entrantes (true/false) |

### 3.4 Durée de validité

| Type de Passeport | Durée typique | Renouvellement |
|-------------------|---------------|----------------|
| Standard | 1 à 24 heures | Automatique à expiration si toujours conforme |
| Spécial | Jusqu'à 7 jours | Renouvellement simplifié |

### 3.5 Émission du Permis de circulation (accord relay)

```mermaid
sequenceDiagram
    participant COG as COG
    participant R as Relay

    COG->>R: Passeport COG complet
    R->>R: Phase A : Vérification clé Cores
    R->>R: Phase B : Vérification blocs de code Services
    R->>R: Phase C : Vérification santé environnement
    alt Conforme
        R->>COG: Permis de circulation (permis_id, expires_at, scope)
    else Non-conforme
        R->>COG: Quarantaine (durée, raison)
    end
```

---

## 4. Accord d'hôte

### 4.1 Définition

Distinct du Permis de circulation, l'**accord d'hôte** est délivré par un **COG hôte** à un COG client pour autoriser la consommation de services spécifiques.

### 4.2 Structure

| Champ | Type | Description |
|-------|------|-------------|
| `accord_id` | string | Identifiant unique (accord d'hôte) |
| `client_cog_id` | string | COG client autorisé |
| `host_cog_id` | string | COG hôte |
| `services_authorized` | array | Services accessibles |
| `issued_at` | datetime | Date d'émission |
| `expires_at` | datetime | Date d'expiration |
| `lobby_id` | string | Lobby concerné (optionnel) |

### 4.3 Distinction avec le Permis de circulation

| Aspect | Permis de circulation (accord relay) | Accord d'hôte |
|--------|--------------------------------------|---------------|
| **Émetteur** | Relay / Origin | COG hôte |
| **Autorisation** | Circuler sur le réseau MWS | Consommer les services du hôte |
| **Vérification** | Conformité du COG (relay) ; contrôle tracker | Autorisation du hôte |
| **Durée** | Heures à jours | Session ou définie par le hôte |

### 4.4 Flow de délivrance

```mermaid
sequenceDiagram
    participant Client as COG Client
    participant Host as COG Hôte

    Note over Client: Possède un Permis de circulation valide
    Client->>Host: Demande d'accès (Permis circulation, services souhaités)
    Host->>Host: Vérifier Permis de circulation
    Host->>Host: Vérifier autorisation (Lobby, politique)
    alt Autorisé
        Host->>Client: Accord d'hôte (services_authorized)
        Client->>Host: Consommation des services
    else Non autorisé
        Host->>Client: Refus (raison)
    end
```

---

## 5. Cycle de vie des documents

### 5.1 Cycle du Passeport

```mermaid
stateDiagram-v2
    [*] --> Créé: Création du COG
    Créé --> Actif: Première vérification OK
    Actif --> Quarantaine: Non-conformité
    Quarantaine --> Actif: Conformité restaurée
    Quarantaine --> Blacklisté: 3 échecs
    Blacklisté --> Destruction: Auto-destruction
    Destruction --> [*]
    Actif --> Spécial: Demande approuvée par Origin
    Spécial --> Actif: Statut révoqué
```

### 5.2 Cycle du Permis de circulation

```mermaid
stateDiagram-v2
    [*] --> Demande: Présentation au relay
    Demande --> Émis: Vérification OK
    Demande --> Refusé: Non-conformité
    Émis --> Valide: Dans la période de validité
    Valide --> Expiré: Fin de validité
    Expiré --> Renouvelé: Re-vérification OK
    Valide --> Révoqué: Alerte sécurité
    Révoqué --> [*]
    Renouvelé --> Valide
```

---

## 6. Contrôle tracker : vérification des Permis de circulation

### 6.1 Points de vérification (contrôle tracker)

Quand un COG se présente à un Tracker :

| Vérification | Description |
|--------------|-------------|
| **Existence** | Le Permis de circulation existe-t-il ? |
| **Expiration** | Le Permis n'est-il pas expiré ? |
| **Émetteur** | Le relay émetteur est-il reconnu ? |
| **Cohérence** | Le scope est-il cohérent avec la requête ? |
| **Blacklist** | Le `cog_id` n'est-il pas blacklisté ? |

### 6.2 Actions selon le résultat

| Résultat | Action |
|----------|--------|
| Permis valide | Accepter la connexion, assigner au pool |
| Permis expiré | Rediriger vers relay pour renouvellement |
| Permis invalide | Refuser, journaliser, potentiel signalement |
| COG blacklisté | Refuser, ignorer le Permis |

---

## 7. Exemples de Passeport et Permis de circulation

### 7.1 Exemple de Passeport Standard

```json
{
  "cog_id": "550e8400-e29b-41d4-a716-446655440000",
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
  "special_key": null
}
```

### 7.2 Exemple de Permis de circulation

```json
{
  "permis_id": "permis-002",
  "cog_id": "550e8400-e29b-41d4-a716-446655440000",
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
  "passport_type": "STANDARD"
}
```

---

## Références

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Flux de Vérification](./MWS%20-%20Flux%20de%20Verification.md)
- [MWS - Relays](../acteurs/MWS%20-%20Relays.md)
- [Miyukini Webway Relay](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) — sections 2.2 à 2.7

---

**Version :** 1.0  
**Classification :** Documentation MWS — Vérification
