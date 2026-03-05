# MWS â€” Quarantaine et Blacklist

## Contexte

La **quarantaine** et la **blacklist** sont les mÃ©canismes de sÃ©curitÃ© du MWS qui permettent d'isoler les COGs non-conformes et de protÃ©ger le rÃ©seau contre les COGs malveillants ou corrompus. Ce systÃ¨me d'escalade progressive garantit une rÃ©ponse proportionnÃ©e aux non-conformitÃ©s.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- Quarantaine : dÃ©finition, dÃ©clencheurs, durÃ©es, escalade
- Blacklist : conditions, consÃ©quences, auto-destruction
- LevÃ©e de quarantaine et reconstruction
- Alerte rÃ©seau et confinement
- Synchronisation des listes entre acteurs MWS

---

## 1. Quarantaine

### 1.1 DÃ©finition

La **quarantaine** est un Ã©tat d'**isolement temporaire** d'un COG qui n'a pas passÃ© la vÃ©rification de conformitÃ©. Un COG en quarantaine :

- Ne peut pas obtenir de Permis de circulation
- Ne peut pas se connecter aux trackers
- Ne peut pas participer au maillage MWS
- Peut retenter la vÃ©rification aprÃ¨s le dÃ©lai de quarantaine

### 1.2 DÃ©clencheurs de quarantaine

| DÃ©clencheur | Phase | Description |
|-------------|-------|-------------|
| **Ã‰chec Phase A** | ClÃ© Cores | ClÃ© de conformitÃ© incorrecte |
| **Ã‰chec Phase B** | Blocs Services | Un ou plusieurs Services suspects |
| **Ã‰chec Phase C** | SantÃ© | Environnement dÃ©gradÃ© ou corrompu |
| **Permis expirÃ©/invalide** | Tracker | Tentative de connexion avec Permis invalide (contrÃ´le tracker) |
| **Service non rÃ©pertoriÃ©** | Registre | Service absent du Registre Origin |

### 1.3 Escalade des durÃ©es

```mermaid
stateDiagram-v2
    [*] --> NonConformitÃ©1: 1Ã¨re non-conformitÃ©
    NonConformitÃ©1 --> Quarantaine1h: DÃ©lai 1 heure
    Quarantaine1h --> Retentative: AprÃ¨s 1h
    Retentative --> Conforme: VÃ©rification OK
    Retentative --> NonConformitÃ©2: 2Ã¨me Ã©chec
    NonConformitÃ©2 --> Quarantaine2h: DÃ©lai 2 heures (x2)
    Quarantaine2h --> Retentative2: AprÃ¨s 2h
    Retentative2 --> Conforme: VÃ©rification OK
    Retentative2 --> NonConformitÃ©3: 3Ã¨me Ã©chec
    NonConformitÃ©3 --> Blacklist: Blacklistage
    Conforme --> [*]
    Blacklist --> AutoDestruction
```

| Tentative | DurÃ©e | Action |
|-----------|-------|--------|
| **1Ã¨re non-conformitÃ©** | 1 heure | Isolation, journalisation, notification utilisateur |
| **2Ã¨me non-conformitÃ©** | 2 heures (x2) | Isolation, alerte rÃ©seau, surveillance renforcÃ©e |
| **3Ã¨me non-conformitÃ©** | **Blacklist** | COG et IP blacklistÃ©s pour tout le rÃ©seau |

### 1.4 Informations stockÃ©es

| Champ | Description |
|-------|-------------|
| `cog_id` | Identifiant du COG en quarantaine |
| `reason` | Raison de la quarantaine (phase Ã©chouÃ©e, dÃ©clencheur) |
| `started_at` | Date et heure de dÃ©but |
| `duration` | DurÃ©e de la quarantaine |
| `attempt` | NumÃ©ro de tentative (1, 2, 3) |
| `relay_id` | Relay ayant appliquÃ© la quarantaine |

### 1.5 Notification utilisateur

Quand un COG est mis en quarantaine :

| Contenu | Description |
|---------|-------------|
| **Raison** | Quelle phase a Ã©chouÃ© et pourquoi |
| **DurÃ©e** | Combien de temps dure la quarantaine |
| **Actions recommandÃ©es** | Comment corriger la non-conformitÃ© |
| **Historique** | Nombre de tentatives prÃ©cÃ©dentes |

---

## 2. Blacklist

### 2.1 DÃ©finition

La **blacklist** est la liste des COGs (et adresses IP associÃ©es) **dÃ©finitivement exclus** du rÃ©seau MWS. Un COG blacklistÃ© :

- Est identifiÃ© comme **corrompu**
- Doit s'**auto-dÃ©truire**
- Ne peut plus participer au MWS sous cette identitÃ©
- Peut potentiellement Ãªtre restaurÃ© aprÃ¨s reconstruction complÃ¨te

### 2.2 Conditions de blacklistage

| Condition | Description |
|-----------|-------------|
| **3 non-conformitÃ©s** | AprÃ¨s 3 Ã©checs de vÃ©rification consÃ©cutifs |
| **Comportement malveillant** | DÃ©tection d'attaque, usurpation, injection |
| **DÃ©cision Origin** | DÃ©cision explicite d'Origin pour des raisons de sÃ©curitÃ© |

### 2.3 Contenu de la blacklist

| Champ | Description |
|-------|-------------|
| `cog_id` | Identifiant du COG blacklistÃ© |
| `ip_addresses` | Adresses IP associÃ©es |
| `reason` | Raison du blacklistage |
| `blacklisted_at` | Date et heure du blacklistage |
| `source` | Origin, relay, ou tracker ayant initiÃ© |
| `status` | `ACTIVE`, `PENDING_REVIEW`, `REMOVED` |

### 2.4 Auto-destruction

Un COG dont l'ID est blacklistÃ©e **doit** suivre le protocole d'auto-destruction :

```mermaid
sequenceDiagram
    participant COG as COG
    participant Cores as Cores (WorrySentinel)
    participant O as Origin

    Note over COG: ID blacklistÃ©e dÃ©tectÃ©e
    COG->>COG: Identification comme corrompu
    COG->>COG: Suppression de toutes les strates (9 â†’ 0)
    COG->>COG: Conservation des Cores uniquement
    Note over Cores: En attente de connexion Internet
    Cores->>O: Ping (cog_id, Ã©tat actuel)
    O->>Cores: Instructions de remise en conformitÃ©
    Cores->>COG: Reconstruction depuis la version Origin
    Note over COG: Si conformitÃ© restaurÃ©e
    O->>O: Retrait de la blacklist
```

### 2.5 Ã‰tapes de l'auto-destruction

| Ã‰tape | Action |
|-------|--------|
| 1 | Le COG s'identifie comme **corrompu** |
| 2 | Suppression de **toutes les strates** (du haut vers le bas) |
| 3 | Le contenu est **vidÃ©** (donnÃ©es utilisateur, Services) |
| 4 | Seuls les **Cores** restent (Border Guard, WorrySentinel, etc.) |
| 5 | Le Core de sÃ©curitÃ© **ping Origin** dÃ¨s qu'une connexion Internet est disponible |
| 6 | Origin fournit les **instructions de reconstruction** |
| 7 | Le COG est **reconstruit** dans sa version d'origine |
| 8 | Si la conformitÃ© est **restaurÃ©e**, le COG est **retirÃ© de la blacklist** |

---

## 3. LevÃ©e de quarantaine

### 3.1 Conditions

Un COG peut sortir de quarantaine si :

| Condition | Description |
|-----------|-------------|
| **DÃ©lai Ã©coulÃ©** | La durÃ©e de quarantaine est terminÃ©e |
| **Re-vÃ©rification rÃ©ussie** | Les 3 phases de vÃ©rification passent |
| **Correction effectuÃ©e** | La cause de non-conformitÃ© a Ã©tÃ© corrigÃ©e |

### 3.2 Processus

```mermaid
sequenceDiagram
    participant COG as COG
    participant R as Relay

    Note over COG: En quarantaine, dÃ©lai Ã©coulÃ©
    COG->>R: Nouvelle requÃªte de vÃ©rification
    R->>R: VÃ©rification Phase A, B, C
    alt Conforme
        R->>R: RÃ©initialiser compteur de tentatives
        R->>COG: Permis de circulation dÃ©livrÃ©
        Note over COG: Sortie de quarantaine
    else Non-conforme
        R->>R: IncrÃ©menter compteur de tentatives
        R->>COG: Quarantaine (durÃ©e x2)
    end
```

---

## 4. Alerte rÃ©seau

### 4.1 DÃ©clenchement

Une **alerte rÃ©seau** est dÃ©clenchÃ©e si **plusieurs COGs sont rejetÃ©s** dans un **trÃ¨s court laps de temps** :

| Seuil | Action |
|-------|--------|
| > N rejets en < T secondes | Alerte envoyÃ©e Ã  tout le rÃ©seau |

Les seuils N et T sont configurables par Origin.

### 4.2 ConsÃ©quences de l'alerte

```mermaid
flowchart TB
    A[Alerte rÃ©seau] --> B[Relays : contrÃ´le renforcÃ©]
    A --> C[Trackers : surveillance]
    B --> D{Attaque confirmÃ©e ?}
    D -->|Oui| E[Confinement rÃ©seau]
    D -->|Non| F[Retour Ã  la normale]
    E --> G[Fermeture connexions inter-COG]
    E --> H[Origin/Relays en lecture seule]
    E --> I[Reconstruction progressive]
```

### 4.3 Actions immÃ©diates

| Acteur | Action |
|--------|--------|
| **Relays** | Renforcement immÃ©diat des contrÃ´les |
| **Trackers** | Surveillance renforcÃ©e, fermeture possible des connexions |
| **COGs** | Peuvent Ãªtre soumis Ã  re-vÃ©rification obligatoire |

---

## 5. Confinement rÃ©seau

### 5.1 DÃ©finition

Le **confinement rÃ©seau** est l'Ã©tat d'urgence du MWS oÃ¹ les connexions inter-COG sont **fermÃ©es** pour circonscrire une attaque ou une corruption massive.

### 5.2 Phases du confinement

| Phase | Ã‰tat | Description |
|-------|------|-------------|
| **Alerte** | DÃ©tection | Multiples rejets dÃ©tectÃ©s, alerte envoyÃ©e |
| **Confinement** | ExÃ©cution | Les trackers ferment tout ou partie des connexions |
| **Lecture seule** | Maintenance | Origin et relays accessibles en lecture seule, vÃ©rification uniquement |
| **Reconstruction** | RÃ©cupÃ©ration | Les COGs valides reconstruisent le rÃ©seau progressivement |

### 5.3 Comportement des acteurs

| Acteur | Pendant le confinement |
|--------|------------------------|
| **Origin** | Accessible en lecture seule, fonctions de vÃ©rification actives |
| **Relays** | Accessibles en lecture seule, peuvent vÃ©rifier les COGs |
| **Trackers** | Ferment les connexions, n'acceptent que les COGs re-vÃ©rifiÃ©s |
| **COGs** | Ne peuvent plus Ã©changer de donnÃ©es, peuvent se re-vÃ©rifier |

### 5.4 Reconstruction progressive

1. Les COGs se re-prÃ©sentent aux relays
2. Re-vÃ©rification complÃ¨te (Phase A, B, C)
3. Si conforme â†’ nouveau Permis de circulation
4. Connexion aux trackers avec le nouveau Permis
5. Reconstruction progressive du maillage

---

## 5.5 RÃ©vocation de Permis en temps rÃ©el (contremesure R-009)

Pour rÃ©agir rapidement Ã  un COG malveillant sans attendre l'expiration de son Permis, le MWS prÃ©voit une **rÃ©vocation de Permis en temps rÃ©el**.

### DÃ©clenchement

| DÃ©clencheur | Description |
|-------------|-------------|
| **Alerte sÃ©curitÃ©** | Comportement suspect dÃ©tectÃ© par un tracker ou un relay |
| **DÃ©cision administrative** | Origin ou relay dÃ©cide de rÃ©voquer un Permis |
| **Blacklistage** | Le COG est blacklistÃ© â†’ tous ses Permis sont rÃ©voquÃ©s |

### Propagation

| Ã‰tape | Description |
|-------|-------------|
| 1 | Le relay (ou Origin) Ã©met un message **PERMIT_REVOKE** (permis_id, raison, signature) |
| 2 | Origin diffuse la rÃ©vocation Ã  tous les trackers en **moins de 1 minute** |
| 3 | Chaque tracker met Ã  jour son **cache de rÃ©vocation** et ferme les connexions concernÃ©es |
| 4 | Le COG rÃ©voquÃ© reÃ§oit **CLOSE** avec la raison `permit_revoked` |

### Cache de rÃ©vocation

Les trackers maintiennent un cache des Permis rÃ©voquÃ©s (TTL au moins Ã©gal Ã  la durÃ©e max d'un Permis, ex. 8 jours). Toute connexion prÃ©sentant un Permis rÃ©voquÃ© est refusÃ©e.

### Journalisation

| Ã‰vÃ©nement | DonnÃ©es |
|-----------|---------|
| RÃ©vocation Ã©mise | `permis_id`, `cog_id`, `reason`, `revoked_by`, `revoked_at` |
| RÃ©vocation appliquÃ©e | `tracker_id`, `permis_id`, `connections_closed` |

---

## 6. Synchronisation des listes

### 6.1 Architecture de synchronisation

```mermaid
flowchart TB
    subgraph Origin["Origin"]
        OW[Whitelist maÃ®tre]
        OB[Blacklist maÃ®tre]
        OQ[Quarantaines maÃ®tre]
    end

    subgraph Relays["Relays"]
        RW1[Whitelist locale]
        RB1[Blacklist locale]
        RQ1[Quarantaines locales]
    end

    subgraph Trackers["Trackers"]
        TW1[Whitelist locale]
        TB1[Blacklist locale]
        TQ1[Quarantaines locales]
    end

    OW -->|Push| RW1
    OB -->|Push| RB1
    OQ -->|Push| RQ1

    RW1 -->|Push| TW1
    RB1 -->|Push| TB1
    RQ1 -->|Push| TQ1
```

### 6.2 MÃ©canismes

| MÃ©canisme | Description |
|-----------|-------------|
| **Push depuis Origin** | Origin pousse les mises Ã  jour vers tous les relays |
| **Push depuis Relays** | Les relays propagent vers les trackers |
| **Pull pÃ©riodique** | Les acteurs peuvent interroger pour synchronisation |
| **Invalidation** | Notification immÃ©diate en cas de modification critique |

### 6.3 CohÃ©rence

| Principe | Description |
|----------|-------------|
| **Origin fait autoritÃ©** | La liste d'Origin est la vÃ©ritÃ© |
| **CohÃ©rence Ã©ventuelle** | Un lÃ©ger retard est acceptable (< 1 minute) |
| **Pas de divergence** | Un acteur ne peut pas avoir une liste diffÃ©rente d'Origin |

---

## 7. Cas particuliers

### 7.1 Passeport spÃ©cial

| Aspect | Comportement |
|--------|--------------|
| **Quarantaine** | MÃªme processus, mais notification prioritaire |
| **Blacklist** | MÃªme processus, mais audit approfondi avant auto-destruction |
| **Alerte** | Un Passeport spÃ©cial blacklistÃ© dÃ©clenche une alerte rÃ©seau |

### 7.2 COG avec parentÃ©

| Aspect | Comportement |
|--------|--------------|
| **Quarantaine** | Le COG parent est notifiÃ© |
| **Blacklist** | Le COG parent n'est pas automatiquement blacklistÃ© (mais surveillÃ©) |
| **Confiance hÃ©ritÃ©e** | La confiance du parent peut accÃ©lÃ©rer la sortie de quarantaine |

### 7.3 Faux positif

Si un COG lÃ©gitime est mis en quarantaine par erreur :

1. L'utilisateur peut contacter Origin
2. Audit manuel de la situation
3. Si erreur confirmÃ©e : levÃ©e de quarantaine + whitelist temporaire
4. Investigation de la cause du faux positif

---

## 8. Journalisation

### 8.1 Ã‰vÃ©nements journalisÃ©s

| Ã‰vÃ©nement | DonnÃ©es |
|-----------|---------|
| Mise en quarantaine | `cog_id`, `reason`, `attempt`, `duration`, `relay_id` |
| Sortie de quarantaine | `cog_id`, `after_verification` (bool) |
| Blacklistage | `cog_id`, `ip_addresses`, `reason`, `source` |
| Auto-destruction | `cog_id`, `stages_cleared`, `timestamp` |
| Retrait de blacklist | `cog_id`, `reason`, `verified_by` |
| Alerte rÃ©seau | `trigger_count`, `time_window`, `initiator` |
| Confinement | `phase`, `connections_closed`, `timestamp` |
| RÃ©vocation Permis | `permis_id`, `cog_id`, `reason`, `revoked_by` |

### 8.2 RÃ©tention

| Type | DurÃ©e recommandÃ©e |
|------|-------------------|
| Quarantaines | 90 jours |
| Blacklists | IndÃ©fini (historique) |
| Alertes rÃ©seau | 1 an |
| Confinements | IndÃ©fini (incidents critiques) |

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Flux de VÃ©rification](../verification/MWS%20-%20Flux%20de%20Verification.md)
- [MWS - Relays](../acteurs/MWS%20-%20Relays.md)
- [MWS - Trackers](../acteurs/MWS%20-%20Trackers.md)
- [MWS - Contre-Mesures de SÃ©curitÃ©](./MWS%20-%20Contre-Mesures%20de%20Securite.md) â€” R-009
- [Miyukini Webway Relay](..//reference//_index.md) â€” sections 2.8, 2.9, 3.4

---

**Version :** 2.0  
**Mise Ã  jour :** RÃ©vocation Permis temps rÃ©el (R-009)  
**Classification :** Documentation MWS â€” SÃ©curitÃ©

