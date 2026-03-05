# MWS â€” Origin (Source de VÃ©ritÃ©)

## Contexte

**Origin** est le **point d'origine** du Miyukini Webway System (MWS). Il cumule les fonctions de **relay** et de **tracker**, et constitue la **source de vÃ©ritÃ© unique** de l'Ã©cosystÃ¨me. Toute information officielle (versions des Cores, Services, politiques de conformitÃ©, Passeports spÃ©ciaux) Ã©mane d'Origin.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- DÃ©finition et rÃ´le d'Origin dans l'architecture MWS
- Fonctions relay d'Origin (vÃ©rification, distribution)
- Fonctions tracker d'Origin (pools, catalogue, connexions)
- Source de vÃ©ritÃ© (Registre de Services, versions, politiques)
- DÃ©livrance des Passeports spÃ©ciaux
- Protocole de prÃ©sentation et redirection

---

## 1. Position d'Origin dans l'architecture

Origin est l'**unique point d'origine** du MWS. Tous les autres acteurs (relays, trackers, COGs) sont **subordonnÃ©s** Ã  Origin, directement ou indirectement.

```mermaid
flowchart TB
    subgraph Origin["Origin (relay + tracker)"]
        O[Origin]
        REG[Registre de Services]
        VER[Versions Cores]
        POL[Politiques conformitÃ©]
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

    O -->|VÃ©ritÃ© distribuÃ©e| R1
    O -->|VÃ©ritÃ© distribuÃ©e| R2
    R1 -->|CritÃ¨res| T1
    R2 -->|CritÃ¨res| T2
```

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **Point d'entrÃ©e initial** | Tout COG se prÃ©sente **d'abord Ã  Origin** pour sa premiÃ¨re vÃ©rification. |
| **Fonction duale** | Origin cumule les capacitÃ©s **relay** (vÃ©rification de conformitÃ©) et **tracker** (connexions, pools). |
| **Source de vÃ©ritÃ©** | Toutes les informations officielles (versions, checksums, politiques) Ã©manent d'Origin. |
| **Subordination** | Les relays et trackers hÃ©ritent leur vÃ©ritÃ© d'Origin et lui sont subordonnÃ©s. |

### 1.2 Comment les clients obtiennent l'adresse d'Origin

L'adresse d'Origin **ne doit pas Ãªtre falsifiable**. Les distributions (packages, installateurs) fournissent cette adresse via un **Manifeste Origin signÃ©** : un fichier contenant l'URL canonique et le pin TLS, signÃ© par l'autoritÃ© MWS. La clÃ© publique de vÃ©rification est intÃ©grÃ©e dans le client. Toute modification du manifeste invalide la signature et est rejetÃ©e. Voir [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md).

---

## 2. Fonctions relay d'Origin

En tant que relay, Origin effectue les opÃ©rations suivantes :

### 2.1 VÃ©rification de conformitÃ©

| Ã‰tape | Description |
|-------|-------------|
| **RÃ©ception du Passeport COG** | Origin reÃ§oit le Passeport complet du COG (`cog_id`, `core_version`, `service_list`, `environment_health`, `previous_permis`, `passport_type`). |
| **Phase A : ClÃ© de conformitÃ© des Cores** | Origin compare la clÃ© cachÃ©e dans le code des Cores avec la clÃ© attendue pour la version dÃ©clarÃ©e. |
| **Phase B : Blocs de code des Services** | Origin demande des blocs de code MIP alÃ©atoires pour chaque Service et vÃ©rifie le dÃ©chiffrement. |
| **Phase C : SantÃ© de l'environnement** | Origin vÃ©rifie le rapport de santÃ© produit par WorrySentinel et KindMother. |
| **DÃ©cision** | Conforme â†’ Permis de circulation (accord relay). Non-conforme â†’ Quarantaine. |

### 2.2 Distribution des versions

| CapacitÃ© | Description |
|----------|-------------|
| **Versions des Cores** | Origin publie toutes les versions officielles des Cores (`core_version`, changelog, seuils minimaux). |
| **Services officiels** | Origin distribue les Services officiels Miyukini avec checksums et URLs de tÃ©lÃ©chargement. |
| **Services tiers** | Origin maintient un Registre des services tiers rÃ©pertoriÃ©s et redirige vers leurs sources officielles. |

### 2.3 Redirection en cas de saturation

Si Origin est **saturÃ©** (puissance de calcul insuffisante, trop de requÃªtes simultanÃ©es) :

1. Origin Ã©value la requÃªte entrante.
2. Si impossible de traiter â†’ Origin renvoie un message `REDIRECT` vers un relay disponible.
3. Le COG se reconnecte au relay dÃ©signÃ© pour poursuivre la vÃ©rification.

```mermaid
sequenceDiagram
    participant COG as COG
    participant O as Origin
    participant R as Relay A

    COG->>O: RequÃªte de vÃ©rification (cog_id)
    O->>O: Ã‰valuer capacitÃ©
    alt Origin saturÃ©
        O->>COG: REDIRECT (relay_A_host:7000)
        COG->>R: RequÃªte de vÃ©rification (cog_id)
        R->>COG: Acceptation, vÃ©rification...
    else Origin disponible
        O->>COG: Acceptation directe
        O->>O: VÃ©rification (Phase A, B, C)
        O->>COG: Permis de circulation
    end
```

---

## 3. Fonctions tracker d'Origin

En tant que tracker, Origin gÃ¨re :

| CapacitÃ© | Description |
|----------|-------------|
| **Pools par version des Cores** | Dirige les COGs vers des pools isolÃ©s par `core_version.MAJOR`. |
| **ContrÃ´le d'identitÃ© et contrÃ´le tracker** | VÃ©rifie le Permis de circulation avant connexion au maillage. |
| **Whitelists / Blacklists / Quarantaines** | Maintient les listes maÃ®tres (partagÃ©es avec les trackers). |
| **Catalogue web (port 80)** | Catalogue des services WEB publics (URLs, recherche) ; catalogue de Lobbys tenu mais visible depuis les services COG, pas depuis le portail web. |
| **Monitoring rÃ©seau** | Surveille l'Ã©tat du rÃ©seau, dÃ©tecte les congestions. |

---

## 4. Source de vÃ©ritÃ© unique

### 4.1 Registre de Services

Le **Registre de Services** d'Origin contient :

#### Services officiels Miyukini

| Champ | Description |
|-------|-------------|
| `service_id` | Identifiant unique (ex. `webway.tracker`, `bridge`) |
| `current_version` | Version courante officielle (`MAJOR.MINOR.PATCH`) |
| `min_version` | Version minimale acceptÃ©e sur le rÃ©seau |
| `checksum` | Hash SHA-256 du binaire/package |
| `download_url` | URL de tÃ©lÃ©chargement officielle |
| `changelog_url` | URL du journal des modifications |
| `core_compatibility` | Liste des `core_version.MAJOR` compatibles |
| `status` | `ACTIVE`, `DEPRECATED`, `RETIRED` |

#### Services tiers rÃ©pertoriÃ©s

| Champ | Description |
|-------|-------------|
| `service_id` | Identifiant unique (prÃ©fixe `third.` ou namespace Ã©diteur) |
| `publisher` | Nom de l'Ã©diteur |
| `official_source_url` | URL de la source officielle de l'Ã©diteur |
| `current_version` | DerniÃ¨re version connue |
| `checksum` | Hash SHA-256 de la version rÃ©pertoriÃ©e |
| `core_compatibility` | `core_version.MAJOR` compatibles |
| `review_status` | `APPROVED`, `PENDING_REVIEW`, `SUSPENDED` |
| `registration_date` | Date d'enregistrement |

### 4.2 Versions des Cores

Origin publie et maintient :

| Ã‰lÃ©ment | Description |
|---------|-------------|
| **Version courante** | La derniÃ¨re version stable des Cores |
| **Historique** | Toutes les versions prÃ©cÃ©dentes avec changelogs |
| **ClÃ©s de conformitÃ©** | ClÃ©s cachÃ©es associÃ©es Ã  chaque `core_version` |
| **Seuils minimaux** | `min_core_version` pour le rÃ©seau |

### 4.3 Politiques de conformitÃ©

| Politique | Description |
|-----------|-------------|
| **CritÃ¨res de sÃ©curitÃ©** | RÃ¨gles de vÃ©rification (Phase A, B, C) |
| **Seuils de quarantaine** | DÃ©lais et escalade (1h, 2h, blacklist) |
| **RÃ¨gles de blacklistage** | Conditions de mise en blacklist |
| **Passeports spÃ©ciaux** | Registre des COGs avec Passeport spÃ©cial |

---

## 5. Passeports spÃ©ciaux

**Origin est le seul** Ã  pouvoir dÃ©livrer des **Passeports spÃ©ciaux**. Ces passeports concernent les COGs Ã  usage **professionnel** ou Ã  **fort trafic**.

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **ID spÃ©ciale** | Identifiant unique renforcÃ© |
| **ClÃ© spÃ©ciale** | ClÃ© cryptographique attestant le statut professionnel |
| **ContrÃ´le allÃ©gÃ© quotidien** | VÃ©rification simplifiÃ©e au quotidien |
| **ContrÃ´le renforcÃ© lors des audits** | VÃ©rifications approfondies planifiÃ©es |
| **Cas d'usage** | Sites de grandes entreprises, serveurs de services, jeux MMO |
| **Protocole de dÃ©livrance** | Audit prÃ©alable, attestation, processus spÃ©cifique avec Origin |

### 5.1 Protocole de dÃ©livrance

1. **Demande** : Le COG soumet une demande de Passeport spÃ©cial Ã  Origin.
2. **Audit prÃ©alable** : Origin audite le COG (conformitÃ©, historique, cas d'usage).
3. **Attestation** : Si approuvÃ©, Origin gÃ©nÃ¨re une clÃ© spÃ©ciale et l'enregistre.
4. **DÃ©livrance** : Le COG reÃ§oit son Passeport spÃ©cial avec l'ID et la clÃ©.
5. **Renouvellement** : Audits pÃ©riodiques pour maintenir le statut.

---

## 6. Services sur Origin (exclusivement MWS)

Origin est **exclusivement dÃ©diÃ© au MWS**. Aucun service hors pÃ©rimÃ¨tre MWS n'est installÃ© ni exÃ©cutÃ© sur la VM Origin.

### 6.1 Services MWS prÃ©sents

| Service | Port | Description |
|---------|------|-------------|
| **Origin Relay** | 7000 | VÃ©rification de conformitÃ©, dÃ©livrance de Permis |
| **Origin Tracker** | 21000 | Pools, dÃ©couverte, Lobbys, catalogue |
| **Portail web Origin** | 80/443 (`/`) | Site public MWS (voir Â§ 6.2) + bouton d'accÃ¨s MiyukiniAdmin Origin |
| **MiyukiniAdmin Origin** | 443 (`/admin`) | Panneau d'administration spÃ©cifique Ã  Origin : tests, monitoring, gestion (accÃ¨s restreint) |
| **Registre de Services** | interne | Liste officielle des services autorisÃ©s |

### 6.2 Portail web Origin (racine `/`)

La racine du serveur web d'Origin (`https://origin.miyukini.com/` ou l'URL du VPS Hostinger) affiche le **portail public MWS** avec le contenu suivant :

| Contenu | Description |
|---------|-------------|
| **PrÃ©sentation du projet** | PrÃ©sentation globale de Miyukini COG |
| **Documentation** | Documentation officielle complÃ¨te |
| **TÃ©lÃ©chargement** | Versions des COGs, Cores, packages officiels |
| **Dev blog** | Blog de dÃ©veloppement et actualitÃ©s |
| **Annonces globales** | Nouvelles versions, alertes, communications officielles |

En bas ou en en-tÃªte du portail, un **bouton d'accÃ¨s** renvoie vers la page d'authentification de **MiyukiniAdmin Origin** (`/admin`). Ce bouton est visible publiquement mais la page `/admin` elle-mÃªme est protÃ©gÃ©e par le protocole d'identification (e-mail + mot de passe Argon2id).

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  origin.miyukini.com                                         â”‚
â”‚â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â”‚
â”‚                                                              â”‚
â”‚   â•”â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•—   â”‚
â”‚   â•‘            Miyukini COG â€” Webway System              â•‘   â”‚
â”‚   â•šâ•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•   â”‚
â”‚                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚   â”‚ PrÃ©sentation â”‚  â”‚ Documentationâ”‚  â”‚  TÃ©lÃ©chargement  â”‚  â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚   â”‚   Dev Blog   â”‚  â”‚       Annonces globales              â”‚ â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                                              â”‚
â”‚                              â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚                              â”‚  MiyukiniAdmin Origin âžœ  â”‚    â”‚
â”‚                              â”‚     (Authentification)    â”‚    â”‚
â”‚                              â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚
â”‚                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 6.3 Services exclus

| Exclusion | Raison |
|-----------|--------|
| Services applicatifs tiers | S'exÃ©cutent sur les COGs, pas sur Origin |
| Jeux, streaming, messagerie | Services utilisateurs â€” hors pÃ©rimÃ¨tre |
| CDN de contenu | Origin ne sert pas de CDN (sauf catalogue MWS) |
| CI/CD, monitoring externe | Le monitoring est intÃ©grÃ© dans MiyukiniAdmin |
| Base de donnÃ©es externe | Origin utilise son propre stockage embarquÃ© |

### 6.4 MiyukiniAdmin Origin

**MiyukiniAdmin Origin** est le panneau d'administration **spÃ©cifique Ã  Origin**, accessible uniquement Ã  l'administrateur authentifiÃ© (dÃ©tenteur de la distribution stable). Il fournit :

- **Batterie complÃ¨te de tests** (connectivitÃ©, fonctionnel MWS, sÃ©curitÃ©, rÃ©seau)
- **Monitoring en temps rÃ©el** (mÃ©triques systÃ¨me et MWS, alertes 3 niveaux)
- **Gestion des services** (restart, Registre, versions Cores, quarantaines, blacklists, alertes rÃ©seau)

> **Note :** MiyukiniAdmin est un concept gÃ©nÃ©rique ; chaque acteur MWS peut disposer de son propre MiyukiniAdmin adaptÃ© Ã  son rÃ´le. Celui d'Origin est le plus complet car Origin est la source de vÃ©ritÃ©.

Voir [MWS - MiyukiniAdmin](../administration/MWS%20-%20MiyukiniAdmin.md) pour la documentation complÃ¨te.

---

## 7. RÃ©silience et haute disponibilitÃ©

| Aspect | Description |
|--------|-------------|
| **Point unique** | Origin est unique ; sa disponibilitÃ© est critique. |
| **Redirection** | En cas de saturation, redirection vers les relays. |
| **Mode lecture seule** | En cas d'alerte rÃ©seau, Origin reste accessible en lecture seule. |
| **Reconstruction** | En cas de dÃ©faillance, les relays maintiennent la vÃ©ritÃ© hÃ©ritÃ©e jusqu'Ã  restauration. |

### 7.1 ImplÃ©mentation actuelle

Origin est hÃ©bergÃ© sur un **VPS Hostinger** (Debian 13) :

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **IP publique** | `46.202.129.65` |
| **Domaine** | `origin.miyukini.com` (Ã  configurer) |
| **Port relay** | 7000 |
| **Port tracker** | 21000 |
| **Port web** | 80 / 443 |

Pour le guide complet d'installation et de configuration, voir [MWS - ImplÃ©mentation Origin Hostinger](../deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md).

---

## 8. SchÃ©ma rÃ©capitulatif

```
+------------------------+
|        ORIGIN          |
|------------------------|
| Fonction RELAY :       |
| - VÃ©rification (A,B,C) |
| - Permis de circulation  |
| - Distribution versions|
| - Passeports spÃ©ciaux  |
|------------------------|
| Fonction TRACKER :     |
| - Pools par version    |
| - Catalogue web (services WEB publics) ; catalogue de Lobbys (visible depuis les services) |
| - Whitelists/Blacklists|
| - Monitoring rÃ©seau    |
|------------------------|
| Source de VÃ‰RITÃ‰ :     |
| - Registre de Services |
| - Versions des Cores   |
| - Politiques conformitÃ©|
| - ClÃ©s de conformitÃ©   |
+------------------------+
         |
         | VÃ©ritÃ© distribuÃ©e
         v
+--------+--------+
| Relays | Trackers|
+--------+--------+
```

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Relays](./MWS%20-%20Relays.md)
- [MWS - Trackers](./MWS%20-%20Trackers.md)
- [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md)
- [MWS - MiyukiniAdmin](../administration/MWS%20-%20MiyukiniAdmin.md) â€” panneau d'administration Origin
- [MWS - ImplÃ©mentation Origin Hostinger](../deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md) â€” guide complet de dÃ©ploiement
- [MWS - Haute DisponibilitÃ© Origin](../securite/MWS%20-%20Haute%20Disponibilite%20Origin.md) â€” architecture actif-passif, failover
- [Miyukini Webway Relay](..//reference//_index.md) â€” sections 1, 3, 6

---

**Version :** 3.0  
**Mise Ã  jour :** Services MWS uniquement, MiyukiniAdmin, scope restreint  
**Classification :** Documentation MWS â€” Acteurs

