# MWS â€” Flux de VÃ©rification (3 Phases)

## Contexte

La **vÃ©rification de conformitÃ©** est le processus par lequel un relay (ou Origin) s'assure qu'un COG est **authentique**, **intÃ¨gre** et **sain** avant de lui dÃ©livrer un Permis de circulation. Cette vÃ©rification se dÃ©roule en **trois phases** distinctes et complÃ©mentaires.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- Phase A : VÃ©rification de la clÃ© de conformitÃ© des Cores
- Phase B : VÃ©rification par blocs de code des Services
- Phase C : VÃ©rification de la santÃ© de l'environnement
- RÃ©sultats possibles et actions
- Diagrammes de sÃ©quence dÃ©taillÃ©s
- SÃ©curitÃ© renforcÃ©e (vÃ©rification Ã©tendue)

---

## 1. Vue d'ensemble

```mermaid
flowchart TB
    subgraph PrÃ©sentation["PrÃ©sentation"]
        P[COG se prÃ©sente avec Passeport]
    end

    subgraph PhaseA["Phase A : ClÃ© Cores"]
        A1[Cores envoient clÃ© cachÃ©e]
        A2{ClÃ© correcte ?}
    end

    subgraph PhaseB["Phase B : Blocs Services"]
        B1[Pour chaque Service]
        B2[Bloc de code alÃ©atoire chiffrÃ©]
        B3{DÃ©chiffrement OK ?}
    end

    subgraph PhaseC["Phase C : SantÃ©"]
        C1[VÃ©rifier environment_health]
        C2{SantÃ© OK ?}
    end

    subgraph RÃ©sultat["RÃ©sultat"]
        R1[Permis de circulation]
        R2[Quarantaine]
    end

    P --> A1
    A1 --> A2
    A2 -->|Oui| B1
    A2 -->|Non| R2
    B1 --> B2
    B2 --> B3
    B3 -->|Oui| C1
    B3 -->|Non| R2
    C1 --> C2
    C2 -->|Oui| R1
    C2 -->|Non| R2
```

---

## 2. Phase A : VÃ©rification de la clÃ© de conformitÃ© des Cores

### 2.1 Principe

Les Cores d'un COG contiennent une **clÃ© de conformitÃ© cachÃ©e** dans leur code. Puisque les Cores proviennent normalement d'Origin (et sont immuables), cette clÃ© est connue du relay et permet de vÃ©rifier l'authenticitÃ© des Cores.

### 2.2 MÃ©canisme

```mermaid
sequenceDiagram
    participant Cores as Cores (WorrySentinel)
    participant COG as COG
    participant R as Relay

    COG->>R: Passeport (cog_id, core_version)
    R->>R: RÃ©cupÃ©rer clÃ© attendue pour core_version
    R->>COG: Demande de clÃ© de conformitÃ©
    Cores->>COG: GÃ©nÃ©rer clÃ© cachÃ©e
    COG->>R: ClÃ© de conformitÃ© transmise
    R->>R: Comparer clÃ© reÃ§ue vs clÃ© attendue
    alt ClÃ© correcte
        R->>R: Phase A OK â†’ passer Ã  Phase B
    else ClÃ© incorrecte
        R->>COG: Non-conformitÃ© Cores â†’ Quarantaine
    end
```

### 2.3 DÃ©tails techniques

| Ã‰lÃ©ment | Description |
|---------|-------------|
| **ClÃ© cachÃ©e** | IntÃ©grÃ©e dans le code des Cores Ã  la compilation, non accessible de l'extÃ©rieur |
| **Stockage relay** | Le relay possÃ¨de toutes les clÃ©s attendues (hÃ©ritÃ©es d'Origin) pour chaque `core_version` |
| **Comparaison** | Comparaison cryptographique constante-time pour Ã©viter les timing attacks |
| **RÃ©sultat** | Concordance = Cores authentiques ; Discordance = Cores potentiellement falsifiÃ©s |

### 2.4 Causes d'Ã©chec

| Cause | Description |
|-------|-------------|
| **Cores modifiÃ©s** | Le code des Cores a Ã©tÃ© altÃ©rÃ© |
| **Cores contrefaits** | Les Cores ne proviennent pas d'Origin |
| **Version dÃ©clarÃ©e incorrecte** | Le COG dÃ©clare une `core_version` diffÃ©rente de celle installÃ©e |
| **Corruption** | Corruption du fichier des Cores |

---

## 3. Phase B : VÃ©rification par blocs de code des Services

### 3.1 Principe

Chaque Service du COG doit prouver qu'il exÃ©cute du **code authentique et non corrompu**. Pour cela, le relay demande un **bloc de code alÃ©atoire** (au sens du MSCM/MIP) et vÃ©rifie qu'il correspond Ã  la version officielle.

### 3.2 MÃ©canisme

```mermaid
sequenceDiagram
    participant S as Service
    participant COG as COG
    participant R as Relay

    R->>R: SÃ©lectionner bloc de code alÃ©atoire (MIP index)
    R->>COG: Demande bloc #X du Service Y
    COG->>S: Extraire bloc #X
    S->>S: Chiffrer bloc avec clÃ© de vÃ©rification
    S->>COG: Bloc chiffrÃ©
    COG->>R: Paquet chiffrÃ© contenant le bloc
    R->>R: DÃ©chiffrer avec rÃ©fÃ©rence Origin
    alt Bloc correct
        R->>R: Service Y OK
    else Bloc incorrect
        R->>R: Service Y suspect
        Note over R: Option : vÃ©rification Ã©tendue
    end
    Note over R: RÃ©pÃ©ter pour chaque Service
```

### 3.3 DÃ©tails techniques

| Ã‰lÃ©ment | Description |
|---------|-------------|
| **Bloc de code MIP** | Segment de code indexÃ© dans le MSCM Index Protocol |
| **SÃ©lection alÃ©atoire** | Le relay choisit un bloc au hasard (imprÃ©visible) |
| **Chiffrement** | Le Service chiffre le bloc avec une clÃ© de vÃ©rification |
| **RÃ©fÃ©rence Origin** | Le relay possÃ¨de les blocs de rÃ©fÃ©rence de toutes les versions officielles |
| **DÃ©chiffrement** | Le relay dÃ©chiffre et compare avec la rÃ©fÃ©rence |

### 3.4 Gestion des versions

| Situation | Action du relay |
|-----------|-----------------|
| **Version courante** | VÃ©rification normale |
| **Version antÃ©rieure valide** | Pas d'alerte ; notification de mise Ã  jour |
| **Version inconnue** | Non-conformitÃ© ; vÃ©rification impossible |
| **Service non rÃ©pertoriÃ©** | Isolation (voir [MWS - Isolation des Services](..//README.md)) |

### 3.5 VÃ©rification Ã©tendue

En cas de **doute** (bloc suspect mais pas clairement corrompu), le relay peut demander une **vÃ©rification Ã©tendue** :

| Mode | Description |
|------|-------------|
| **Standard** | 1 bloc alÃ©atoire par Service |
| **Ã‰tendue** | Plusieurs blocs ou tout le code du Service |
| **Maximale** | VÃ©rification de tous les blocs de tous les Services |

La vÃ©rification Ã©tendue est **plus lente** mais offre une **garantie renforcÃ©e**.

---

## 4. Phase C : VÃ©rification de la santÃ© de l'environnement

### 4.1 Principe

Le relay vÃ©rifie le rapport de **santÃ© de l'environnement** (`environment_health`) produit par les Cores pour s'assurer que l'environnement global du COG est **sain et intÃ¨gre**.

### 4.2 MÃ©canisme

```mermaid
sequenceDiagram
    participant Cores as Cores (WorrySentinel, KindMother)
    participant COG as COG
    participant R as Relay

    Note over Cores: GÃ©nÃ©ration du rapport de santÃ©
    Cores->>Cores: VÃ©rifier intÃ©gritÃ© stockage
    Cores->>Cores: VÃ©rifier configuration
    Cores->>Cores: VÃ©rifier strates intactes
    Cores->>COG: Rapport signÃ© (environment_health)
    COG->>R: Passeport incluant environment_health
    R->>R: VÃ©rifier signature du rapport
    R->>R: Analyser les indicateurs
    alt SantÃ© OK
        R->>R: Phase C OK â†’ Permis dÃ©livrÃ©
    else SantÃ© dÃ©gradÃ©e/corrompue
        R->>COG: Non-conformitÃ© â†’ Quarantaine
    end
```

### 4.3 Contenu du rapport de santÃ©

| Indicateur | Valeurs | Description |
|------------|---------|-------------|
| `storage_integrity` | `OK`, `DEGRADED`, `CORRUPTED` | IntÃ©gritÃ© du stockage vÃ©rifiÃ©e par KindMother |
| `config_valid` | `true`, `false` | Configuration valide et cohÃ©rente |
| `strata_intact` | `true`, `false` | Strates 0-9 intactes |
| `attestation_signature` | signature | Signature cryptographique par WorrySentinel |
| `generated_at` | datetime | Date de gÃ©nÃ©ration du rapport |

### 4.4 CritÃ¨res de conformitÃ©

| Indicateur | Conforme | Non-conforme |
|------------|----------|--------------|
| `storage_integrity` | `OK` ou `DEGRADED` (avec avertissement) | `CORRUPTED` |
| `config_valid` | `true` | `false` |
| `strata_intact` | `true` | `false` |
| Signature | Valide | Invalide ou absente |
| AnciennetÃ© | < 5 minutes | > 5 minutes (rapport pÃ©rimÃ©) |

---

## 5. RÃ©sultats de la vÃ©rification

### 5.1 ConformitÃ© totale

Si les **trois phases** rÃ©ussissent :

| Action | Description |
|--------|-------------|
| **Permis de circulation** | DÃ©livrÃ© avec `permis_id`, `expires_at`, `scope` |
| **Enregistrement** | COG enregistrÃ© dans la table de routage du relay |
| **Notification** | Si version en retard : notification de mise Ã  jour |

### 5.2 Non-conformitÃ©

Si une ou plusieurs phases Ã©chouent :

| Phase Ã©chouÃ©e | Signification | Action |
|---------------|---------------|--------|
| **Phase A** | Cores falsifiÃ©s ou corrompus | Quarantaine immÃ©diate |
| **Phase B** | Service suspect ou non rÃ©pertoriÃ© | Quarantaine ou isolation |
| **Phase C** | Environnement dÃ©gradÃ© | Quarantaine |

### 5.3 Escalade de quarantaine

| Tentative | DurÃ©e de quarantaine | Action |
|-----------|----------------------|--------|
| 1Ã¨re | 1 heure | Isolation, journalisation |
| 2Ã¨me | 2 heures (x2) | Isolation, alerte |
| 3Ã¨me | Blacklist | COG et IP blacklistÃ©s, auto-destruction dÃ©clenchÃ©e |

Voir [MWS - Quarantaine et Blacklist](../securite/MWS%20-%20Quarantaine%20et%20Blacklist.md).

---

## 6. Diagramme de sÃ©quence complet

```mermaid
sequenceDiagram
    participant COG as COG
    participant Cores as Cores
    participant Services as Services
    participant R as Relay

    %% PrÃ©sentation
    COG->>R: RequÃªte de vÃ©rification (cog_id)
    R->>R: Ã‰valuer capacitÃ©
    R->>COG: Acceptation

    %% Transmission du Passeport
    COG->>R: Passeport complet

    %% Phase A
    Note over R: Phase A : ClÃ© Cores
    R->>COG: Demande clÃ© de conformitÃ©
    Cores->>COG: ClÃ© cachÃ©e gÃ©nÃ©rÃ©e
    COG->>R: ClÃ© transmise
    R->>R: VÃ©rifier clÃ© vs rÃ©fÃ©rence Origin
    alt ClÃ© incorrecte
        R->>COG: QUARANTINE (phase_a_failed)
    end

    %% Phase B
    Note over R: Phase B : Blocs Services
    loop Pour chaque Service
        R->>COG: Demande bloc alÃ©atoire
        Services->>COG: Bloc chiffrÃ©
        COG->>R: Paquet chiffrÃ©
        R->>R: DÃ©chiffrer et comparer
        alt Bloc incorrect
            R->>R: Marquer suspect
        end
    end
    alt Un ou plusieurs Services suspects
        R->>COG: QUARANTINE (phase_b_failed)
    end

    %% Phase C
    Note over R: Phase C : SantÃ© environnement
    R->>R: VÃ©rifier environment_health
    R->>R: VÃ©rifier signature
    alt SantÃ© non conforme
        R->>COG: QUARANTINE (phase_c_failed)
    end

    %% SuccÃ¨s
    R->>COG: PERMIS DE CIRCULATION (permis_id, expires_at)
```

---

## 7. Performances et optimisations

### 7.1 Temps de vÃ©rification typique

| Phase | Temps estimÃ© | Facteurs |
|-------|--------------|----------|
| Phase A | < 100 ms | Comparaison cryptographique |
| Phase B | 100-500 ms par Service | Nombre de Services, taille des blocs |
| Phase C | < 50 ms | VÃ©rification de signature |
| **Total** | 500 ms - 2 s | Selon le nombre de Services |

### 7.2 Optimisations possibles

| Optimisation | Description |
|--------------|-------------|
| **Cache de clÃ©s** | Le relay met en cache les clÃ©s de conformitÃ© |
| **ParallÃ©lisation Phase B** | VÃ©rifier plusieurs Services en parallÃ¨le |
| **Skip si rÃ©cent** | Ne pas re-vÃ©rifier si derniÃ¨re vÃ©rification < X minutes (Passeports spÃ©ciaux) |

---

## 8. Cas particuliers

### 8.1 Passeport spÃ©cial

| Aspect | Comportement |
|--------|--------------|
| **VÃ©rification quotidienne** | AllÃ©gÃ©e (phases simplifiÃ©es) |
| **Audit pÃ©riodique** | VÃ©rification complÃ¨te et renforcÃ©e |
| **PrioritÃ©** | Traitement prioritaire par le relay |

### 8.2 Service non rÃ©pertoriÃ© dÃ©tectÃ©

Si un `service_id` dans le `service_manifest` n'est pas dans le Registre :

1. Phase B Ã©choue pour ce Service
2. COG isolÃ© du rÃ©seau (pas en quarantaine classique)
3. Notification utilisateur
4. LevÃ©e d'isolation aprÃ¨s correction

### 8.3 Version des Cores obsolÃ¨te mais valide

| Situation | Action |
|-----------|--------|
| `core_version` obsolÃ¨te | VÃ©rification OK, mais notification de mise Ã  jour |
| `core_version` inconnue | Phase A Ã©choue (clÃ© inconnue) |
| `core_version` incompatible avec le rÃ©seau | Refus (politique de version minimale) |

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Passeport et Visa](./MWS%20-%20Passeport%20et%20Visa.md)
- [MWS - Quarantaine et Blacklist](../securite/MWS%20-%20Quarantaine%20et%20Blacklist.md)
- [MWS - Relays](../acteurs/MWS%20-%20Relays.md)
- [Miyukini Webway Relay](..//reference//_index.md) â€” section 2

---

**Version :** 1.0  
**Classification :** Documentation MWS â€” VÃ©rification


