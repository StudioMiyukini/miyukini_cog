# MWS — Origin (Source de Vérité)

## Contexte

**Origin** est le **point d'origine** du Miyukini Webway System (MWS). Il cumule les fonctions de **relay** et de **tracker**, et constitue la **source de vérité unique** de l'écosystème. Toute information officielle (versions des Cores, Services, politiques de conformité, Passeports spéciaux) émane d'Origin.

**Référence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## Portée / Scope

- Définition et rôle d'Origin dans l'architecture MWS
- Fonctions relay d'Origin (vérification, distribution)
- Fonctions tracker d'Origin (pools, catalogue, connexions)
- Source de vérité (Registre de Services, versions, politiques)
- Délivrance des Passeports spéciaux
- Protocole de présentation et redirection

---

## 1. Position d'Origin dans l'architecture

Origin est l'**unique point d'origine** du MWS. Tous les autres acteurs (relays, trackers, COGs) sont **subordonnés** à Origin, directement ou indirectement.

```mermaid
flowchart TB
    subgraph Origin["Origin (relay + tracker)"]
        O[Origin]
        REG[Registre de Services]
        VER[Versions Cores]
        POL[Politiques conformité]
        O --- REG
        O --- VER
        O --- POL
    end

    subgraph Relays["Relays"]
        R1[Relay A]
        R2[Relay B]
    end

    subgraph Trackers["Trackers"]
        T1[Tracker 1]
        T2[Tracker 2]
    end

    O -->|Vérité distribuée| R1
    O -->|Vérité distribuée| R2
    R1 -->|Critères| T1
    R2 -->|Critères| T2
```

| Caractéristique | Description |
|-----------------|-------------|
| **Point d'entrée initial** | Tout COG se présente **d'abord à Origin** pour sa première vérification. |
| **Fonction duale** | Origin cumule les capacités **relay** (vérification de conformité) et **tracker** (connexions, pools). |
| **Source de vérité** | Toutes les informations officielles (versions, checksums, politiques) émanent d'Origin. |
| **Subordination** | Les relays et trackers héritent leur vérité d'Origin et lui sont subordonnés. |

---

## 2. Fonctions relay d'Origin

En tant que relay, Origin effectue les opérations suivantes :

### 2.1 Vérification de conformité

| Étape | Description |
|-------|-------------|
| **Réception du Passeport COG** | Origin reçoit le Passeport complet du COG (`cog_id`, `core_version`, `service_list`, `environment_health`, `previous_visas`, `passport_type`). |
| **Phase A : Clé de conformité des Cores** | Origin compare la clé cachée dans le code des Cores avec la clé attendue pour la version déclarée. |
| **Phase B : Blocs de code des Services** | Origin demande des blocs de code MIP aléatoires pour chaque Service et vérifie le déchiffrement. |
| **Phase C : Santé de l'environnement** | Origin vérifie le rapport de santé produit par WorrySentinel et KeeperOfStorage. |
| **Décision** | Conforme → Visa de circulation. Non-conforme → Quarantaine. |

### 2.2 Distribution des versions

| Capacité | Description |
|----------|-------------|
| **Versions des Cores** | Origin publie toutes les versions officielles des Cores (`core_version`, changelog, seuils minimaux). |
| **Services officiels** | Origin distribue les Services officiels Miyukini avec checksums et URLs de téléchargement. |
| **Services tiers** | Origin maintient un Registre des services tiers répertoriés et redirige vers leurs sources officielles. |

### 2.3 Redirection en cas de saturation

Si Origin est **saturé** (puissance de calcul insuffisante, trop de requêtes simultanées) :

1. Origin évalue la requête entrante.
2. Si impossible de traiter → Origin renvoie un message `REDIRECT` vers un relay disponible.
3. Le COG se reconnecte au relay désigné pour poursuivre la vérification.

```mermaid
sequenceDiagram
    participant COG as COG
    participant O as Origin
    participant R as Relay A

    COG->>O: Requête de vérification (cog_id)
    O->>O: Évaluer capacité
    alt Origin saturé
        O->>COG: REDIRECT (relay_A_host:7000)
        COG->>R: Requête de vérification (cog_id)
        R->>COG: Acceptation, vérification...
    else Origin disponible
        O->>COG: Acceptation directe
        O->>O: Vérification (Phase A, B, C)
        O->>COG: Visa de circulation
    end
```

---

## 3. Fonctions tracker d'Origin

En tant que tracker, Origin gère :

| Capacité | Description |
|----------|-------------|
| **Pools par version des Cores** | Dirige les COGs vers des pools isolés par `core_version.MAJOR`. |
| **Contrôle d'identité et de Visa** | Vérifie le Visa de circulation avant connexion au maillage. |
| **Whitelists / Blacklists / Quarantaines** | Maintient les listes maîtres (partagées avec les trackers). |
| **Catalogue et Lobbys** | Gère le catalogue des COGs connectés et leurs Lobbys (port 80). |
| **Monitoring réseau** | Surveille l'état du réseau, détecte les congestions. |

---

## 4. Source de vérité unique

### 4.1 Registre de Services

Le **Registre de Services** d'Origin contient :

#### Services officiels Miyukini

| Champ | Description |
|-------|-------------|
| `service_id` | Identifiant unique (ex. `webway.tracker`, `bridge`) |
| `current_version` | Version courante officielle (`MAJOR.MINOR.PATCH`) |
| `min_version` | Version minimale acceptée sur le réseau |
| `checksum` | Hash SHA-256 du binaire/package |
| `download_url` | URL de téléchargement officielle |
| `changelog_url` | URL du journal des modifications |
| `core_compatibility` | Liste des `core_version.MAJOR` compatibles |
| `status` | `ACTIVE`, `DEPRECATED`, `RETIRED` |

#### Services tiers répertoriés

| Champ | Description |
|-------|-------------|
| `service_id` | Identifiant unique (préfixe `third.` ou namespace éditeur) |
| `publisher` | Nom de l'éditeur |
| `official_source_url` | URL de la source officielle de l'éditeur |
| `current_version` | Dernière version connue |
| `checksum` | Hash SHA-256 de la version répertoriée |
| `core_compatibility` | `core_version.MAJOR` compatibles |
| `review_status` | `APPROVED`, `PENDING_REVIEW`, `SUSPENDED` |
| `registration_date` | Date d'enregistrement |

### 4.2 Versions des Cores

Origin publie et maintient :

| Élément | Description |
|---------|-------------|
| **Version courante** | La dernière version stable des Cores |
| **Historique** | Toutes les versions précédentes avec changelogs |
| **Clés de conformité** | Clés cachées associées à chaque `core_version` |
| **Seuils minimaux** | `min_core_version` pour le réseau |

### 4.3 Politiques de conformité

| Politique | Description |
|-----------|-------------|
| **Critères de sécurité** | Règles de vérification (Phase A, B, C) |
| **Seuils de quarantaine** | Délais et escalade (1h, 2h, blacklist) |
| **Règles de blacklistage** | Conditions de mise en blacklist |
| **Passeports spéciaux** | Registre des COGs avec Passeport spécial |

---

## 5. Passeports spéciaux

**Origin est le seul** à pouvoir délivrer des **Passeports spéciaux**. Ces passeports concernent les COGs à usage **professionnel** ou à **fort trafic**.

| Caractéristique | Description |
|-----------------|-------------|
| **ID spéciale** | Identifiant unique renforcé |
| **Clé spéciale** | Clé cryptographique attestant le statut professionnel |
| **Contrôle allégé quotidien** | Vérification simplifiée au quotidien |
| **Contrôle renforcé lors des audits** | Vérifications approfondies planifiées |
| **Cas d'usage** | Sites de grandes entreprises, serveurs de services, jeux MMO |
| **Protocole de délivrance** | Audit préalable, attestation, processus spécifique avec Origin |

### 5.1 Protocole de délivrance

1. **Demande** : Le COG soumet une demande de Passeport spécial à Origin.
2. **Audit préalable** : Origin audite le COG (conformité, historique, cas d'usage).
3. **Attestation** : Si approuvé, Origin génère une clé spéciale et l'enregistre.
4. **Délivrance** : Le COG reçoit son Passeport spécial avec l'ID et la clé.
5. **Renouvellement** : Audits périodiques pour maintenir le statut.

---

## 6. Services web d'Origin

Origin expose un **serveur web** (port 80/443) avec :

| Contenu | Description |
|---------|-------------|
| **Présentation du projet** | Présentation globale de Miyukini COG |
| **Documentation** | Documentation officielle complète |
| **Téléchargement** | Versions des COGs, Cores, packages officiels |
| **Dev blog** | Blog de développement et actualités |
| **Annonces globales** | Nouvelles versions, alertes, communications officielles |

---

## 7. Résilience et haute disponibilité

| Aspect | Description |
|--------|-------------|
| **Point unique** | Origin est unique ; sa disponibilité est critique. |
| **Redirection** | En cas de saturation, redirection vers les relays. |
| **Mode lecture seule** | En cas d'alerte réseau, Origin reste accessible en lecture seule. |
| **Reconstruction** | En cas de défaillance, les relays maintiennent la vérité héritée jusqu'à restauration. |

---

## 8. Schéma récapitulatif

```
+------------------------+
|        ORIGIN          |
|------------------------|
| Fonction RELAY :       |
| - Vérification (A,B,C) |
| - Visa de circulation  |
| - Distribution versions|
| - Passeports spéciaux  |
|------------------------|
| Fonction TRACKER :     |
| - Pools par version    |
| - Catalogue et Lobbys  |
| - Whitelists/Blacklists|
| - Monitoring réseau    |
|------------------------|
| Source de VÉRITÉ :     |
| - Registre de Services |
| - Versions des Cores   |
| - Politiques conformité|
| - Clés de conformité   |
+------------------------+
         |
         | Vérité distribuée
         v
+--------+--------+
| Relays | Trackers|
+--------+--------+
```

---

## Références

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Relays](./MWS%20-%20Relays.md)
- [MWS - Trackers](./MWS%20-%20Trackers.md)
- [Miyukini Webway Relay](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) — sections 1, 3, 6

---

**Version :** 1.0  
**Classification :** Documentation MWS — Acteurs
