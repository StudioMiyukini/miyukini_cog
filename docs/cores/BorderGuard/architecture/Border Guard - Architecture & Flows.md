# Border Guard - Architecture & Flows

## 1. Contexte

Ce document dÃ©finit l'**architecture conceptuelle** de Border Guard et les **flux de dÃ©finition** qu'il orchestre dans l'Ã©cosystÃ¨me Miyukini. Border Guard est le core de dÃ©finition des frontiÃ¨res et des rÃ¨gles d'entrÃ©e/sortie.

Border Guard orchestre trois flux principaux :

1. **Flux de classification** â€” Attribution des niveaux de confiance
2. **Flux de dÃ©finition** â€” Ã‰tablissement des frontiÃ¨res et rÃ¨gles
3. **Flux de conseil** â€” Communication du contexte aux autres cores

Ce document est **dÃ©rivÃ© de la Documentation Fondatrice de Border Guard** et constitue la rÃ©fÃ©rence architecturale pour la structure et les flux de dÃ©finition.

**Document source :** [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toute dÃ©finition de frontiÃ¨re et classification de confiance dans l'Ã©cosystÃ¨me Miyukini
- **Audience :** Architectes, dÃ©veloppeurs, intÃ©grateurs
- **Statut :** Documentation architecturale â€” Normative
- **DÃ©pendances :** Documentation Fondatrice Border Guard, Security Protocols, Lois Autonomie SystÃ¨me

---

## 3. Vue d'ensemble architecturale

### 3.1 Composants conceptuels

Border Guard est structurÃ© en **composants conceptuels** distincts. Ces composants n'ont aucune capacitÃ© d'exÃ©cution â€” ils dÃ©finissent, classifient et Ã©tablissent des rÃ¨gles.

```mermaid
graph TB
    subgraph BorderGuard[Border Guard]
        REG[Registre<br/>des FrontiÃ¨res]
        CLASS[Classificateur<br/>de Confiance]
        RULES[DÃ©finisseur<br/>de RÃ¨gles]
        INTEG[Gouverneur<br/>d'IntÃ©grations]
    end

    subgraph Consommateurs
        SF[StrongFather<br/>contexte]
        BB[BondingBrother<br/>rÃ¨gles]
        CN[Caring Nanny<br/>Ã©tat]
    end

    REG --> RULES
    CLASS --> RULES
    RULES --> BB
    CLASS --> SF
    REG --> CN
    INTEG --> CLASS
    INTEG --> REG

    classDef core fill:#ede7f6
    classDef consumer fill:#e1f5fe
    class REG,CLASS,RULES,INTEG core
    class SF,BB,CN consumer
```

### 3.2 Composants dÃ©taillÃ©s

#### 3.2.1 Registre des FrontiÃ¨res

Le **Registre des FrontiÃ¨res** maintient la dÃ©finition formelle de toutes les frontiÃ¨res du systÃ¨me.

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **Identification** | Nommer et identifier chaque frontiÃ¨re de maniÃ¨re unique |
| **Classification** | Classifier la nature (externe, interne, intÃ©gration) |
| **Direction** | DÃ©finir la direction (entrÃ©e, sortie, bidirectionnelle) |
| **PermÃ©abilitÃ©** | Ã‰tablir le niveau de permÃ©abilitÃ© (ouverte, contrÃ´lÃ©e, fermÃ©e) |
| **Persistance** | Transmettre Ã  KindMother pour stockage |

**DonnÃ©es gÃ©rÃ©es :**

| DonnÃ©e | Description |
|--------|-------------|
| `boundary_id` | Identifiant unique de la frontiÃ¨re |
| `boundary_type` | external, internal, integration |
| `direction` | inbound, outbound, bidirectional |
| `permeability` | open, controlled, closed |
| `description` | Description de la frontiÃ¨re |
| `created_at` | Horodatage de crÃ©ation (local) |

#### 3.2.2 Classificateur de Confiance

Le **Classificateur de Confiance** attribue les niveaux de confiance aux sources, destinations et interactions.

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **Attribution** | Attribuer un niveau de confiance Ã  chaque source/destination |
| **Transition** | GÃ©rer les transitions entre niveaux de confiance |
| **DÃ©faut** | Appliquer le niveau "unknown" par dÃ©faut |
| **RÃ©vocation** | Signaler les passages vers "hostile" |

**Niveaux de confiance (canoniques) :**

| Niveau | Description | CritÃ¨res |
|--------|-------------|----------|
| **Trusted** | Confiance totale | Composants internes validÃ©s, autoritÃ©s systÃ¨me |
| **Verified** | Confiance vÃ©rifiÃ©e | Authentification rÃ©ussie, intÃ©grations certifiÃ©es |
| **Unknown** | Confiance inconnue | DÃ©faut pour tout ce qui arrive de l'extÃ©rieur |
| **Hostile** | Confiance nulle | Sources blacklistÃ©es, patterns d'attaque dÃ©tectÃ©s |

#### 3.2.3 DÃ©finisseur de RÃ¨gles

Le **DÃ©finisseur de RÃ¨gles** Ã©tablit les rÃ¨gles de franchissement des frontiÃ¨res.

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **DÃ©finition** | DÃ©finir les rÃ¨gles associÃ©es Ã  chaque frontiÃ¨re |
| **Conditions** | SpÃ©cifier les conditions de franchissement |
| **Exceptions** | Ã‰tablir les exceptions et cas particuliers |
| **CohÃ©rence** | Maintenir la cohÃ©rence entre rÃ¨gles |

**Structure d'une rÃ¨gle :**

| Ã‰lÃ©ment | Description |
|---------|-------------|
| `rule_id` | Identifiant unique de la rÃ¨gle |
| `boundary_id` | FrontiÃ¨re associÃ©e |
| `trust_level_required` | Niveau de confiance minimal requis |
| `conditions` | Conditions dÃ©claratives de franchissement |
| `exceptions` | Exceptions autorisÃ©es |
| `documentation` | RÃ©fÃ©rence vers la documentation |

**Principe fondamental :** Les rÃ¨gles sont **dÃ©claratives**, pas procÃ©durales. Elles expriment ce qui est requis, pas comment le vÃ©rifier techniquement.

#### 3.2.4 Gouverneur d'IntÃ©grations

Le **Gouverneur d'IntÃ©grations** gÃ¨re conceptuellement les relations avec les systÃ¨mes externes.

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **Classification** | Classifier chaque intÃ©gration selon sa nature et son risque |
| **Cadre** | DÃ©finir le cadre d'interaction avec chaque systÃ¨me externe |
| **Suspension** | Ã‰tablir les conditions de suspension ou rÃ©vocation |
| **Registre** | Maintenir le registre des intÃ©grations et leur Ã©tat |

**Ã‰tats d'une intÃ©gration :**

| Ã‰tat | Description |
|------|-------------|
| **Active** | IntÃ©gration fonctionnelle et autorisÃ©e |
| **Suspendue** | IntÃ©gration temporairement dÃ©sactivÃ©e |
| **RÃ©voquÃ©e** | IntÃ©gration dÃ©finitivement interdite |

---

## 4. Architecture des flux

### 4.1 Vue d'ensemble des flux

Border Guard orchestre trois flux principaux qui fonctionnent de maniÃ¨re coordonnÃ©e mais indÃ©pendante.

```mermaid
graph TB
    subgraph FluxClass[Flux de Classification]
        CL1[RÃ©ception source]
        CL2[Ã‰valuation critÃ¨res]
        CL3[Attribution niveau]
        CL4[Notification]
    end

    subgraph FluxDef[Flux de DÃ©finition]
        DF1[Identification frontiÃ¨re]
        DF2[DÃ©finition rÃ¨gles]
        DF3[Validation cohÃ©rence]
        DF4[Enregistrement]
    end

    subgraph FluxCons[Flux de Conseil]
        CO1[Demande contexte]
        CO2[Recherche informations]
        CO3[Construction contexte]
        CO4[Fourniture]
    end

    CL1 --> CL2 --> CL3 --> CL4
    DF1 --> DF2 --> DF3 --> DF4
    CO1 --> CO2 --> CO3 --> CO4

    CL4 --> CO2
    DF4 --> CO2
```

### 4.2 CaractÃ©ristiques communes

| CaractÃ©ristique | Valeur |
|-----------------|--------|
| **SynchronicitÃ©** | Tous les flux sont non bloquants |
| **TraÃ§abilitÃ©** | Chaque opÃ©ration est enregistrÃ©e (INV-BG-8) |
| **CohÃ©rence** | Les flux maintiennent la cohÃ©rence globale (INV-BG-9) |
| **NeutralitÃ©** | Aucune supposition sur la technologie (INV-BG-10) |
| **PrioritÃ©** | Classification > DÃ©finition > Conseil |

---

## 5. Flux de classification

### 5.1 Description

Le flux de classification est le flux par lequel Border Guard attribue les niveaux de confiance. Ce flux est **rÃ©actif** : il se dÃ©clenche Ã  la demande de classification d'une source ou interaction.

### 5.2 Ã‰tapes du flux

```mermaid
sequenceDiagram
    participant Source as Demandeur
    participant BG as Border Guard
    participant Class as Classificateur
    participant Notif as Notification

    Source->>BG: 1. Soumet source Ã  classifier
    BG->>Class: 2. Ã‰value selon critÃ¨res
    Class-->>BG: Niveau dÃ©terminÃ©
    BG->>BG: 3. Attribue niveau de confiance
    BG->>Notif: 4. Notifie les cores concernÃ©s
    Notif-->>Source: Confirmation de classification
```

#### Ã‰tape 1 : RÃ©ception de la source

Border Guard **reÃ§oit** une demande de classification d'une source, destination ou interaction.

| Information fournie | Description |
|---------------------|-------------|
| **Identifiant** | ID unique de la source/interaction |
| **Type** | Source, destination, ou interaction |
| **Origine** | D'oÃ¹ vient la demande |
| **Contexte** | Informations de contexte disponibles |

**Sources typiques de demande :**

| Demandeur | Cas d'usage |
|-----------|-------------|
| **BondingBrother** | Nouvelle intÃ©gration, nouvelle source externe |
| **Produit (via BB)** | Connexion utilisateur, appel API externe |
| **Caring Nanny** | Source dÃ©tectÃ©e sans classification |

#### Ã‰tape 2 : Ã‰valuation selon critÃ¨res

Border Guard **Ã©value** la source selon des critÃ¨res dÃ©finis.

| CritÃ¨re | Description |
|---------|-------------|
| **Authentification** | La source est-elle authentifiÃ©e ? |
| **Historique** | Y a-t-il un historique de comportement ? |
| **Certification** | La source est-elle certifiÃ©e/validÃ©e ? |
| **Pattern** | Des patterns d'attaque sont-ils dÃ©tectÃ©s ? |

**Matrice de classification :**

| Authentification | Certification | Historique | Pattern malveillant | â†’ Niveau |
|------------------|---------------|------------|---------------------|----------|
| âœ… Interne validÃ© | âœ… | N/A | âŒ | **Trusted** |
| âœ… AuthentifiÃ©e | âœ… | âœ… Positif | âŒ | **Verified** |
| âŒ Non authentifiÃ©e | âŒ | ? | âŒ | **Unknown** |
| ? | ? | âŒ NÃ©gatif | âœ… | **Hostile** |

#### Ã‰tape 3 : Attribution du niveau de confiance

Border Guard **attribue** le niveau de confiance dÃ©terminÃ©.

| DonnÃ©e enregistrÃ©e | Description |
|--------------------|-------------|
| **source_id** | Identifiant de la source |
| **trust_level** | Niveau attribuÃ© (trusted, verified, unknown, hostile) |
| **reason** | Justification de la classification |
| **classified_at** | Horodatage local |
| **valid_until** | DurÃ©e de validitÃ© (si applicable) |

**Invariant applicable :** INV-BG-4 (Classification exhaustive)

#### Ã‰tape 4 : Notification

Border Guard **notifie** les cores concernÃ©s de la nouvelle classification.

| Destinataire | Information envoyÃ©e |
|--------------|---------------------|
| **StrongFather** | Contexte de confiance pour les dÃ©cisions futures |
| **BondingBrother** | RÃ¨gles applicables selon le niveau |
| **Caring Nanny** | Mise Ã  jour de l'Ã©tat des frontiÃ¨res |

### 5.3 Garanties du flux de classification

| Garantie | Description |
|----------|-------------|
| **ExhaustivitÃ©** | Toute source non classifiÃ©e est "unknown" (INV-BG-4) |
| **TraÃ§abilitÃ©** | Chaque classification est enregistrÃ©e avec justification |
| **CohÃ©rence** | Pas de classification contradictoire |
| **Non-autoritÃ©** | Border Guard classifie mais ne bloque pas lui-mÃªme |

---

## 6. Flux de dÃ©finition

### 6.1 Description

Le flux de dÃ©finition est le flux par lequel Border Guard Ã©tablit les frontiÃ¨res et leurs rÃ¨gles. Ce flux est **proactif** : Border Guard initie la dÃ©finition selon les besoins architecturaux.

### 6.2 Ã‰tapes du flux

```mermaid
sequenceDiagram
    participant Archi as Architecte/SystÃ¨me
    participant BG as Border Guard
    participant Reg as Registre
    participant Valid as Validation

    Archi->>BG: 1. Identifie nouvelle frontiÃ¨re
    BG->>Reg: 2. DÃ©finit frontiÃ¨re et rÃ¨gles
    Reg-->>BG: DÃ©finition enregistrÃ©e
    BG->>Valid: 3. Valide cohÃ©rence globale
    alt CohÃ©rent
        Valid-->>BG: Validation OK
        BG->>BG: 4. Enregistre dÃ©finition
    else IncohÃ©rence dÃ©tectÃ©e
        Valid-->>BG: IncohÃ©rence signalÃ©e
        BG-->>Archi: Demande clarification
    end
```

#### Ã‰tape 1 : Identification de la frontiÃ¨re

Border Guard **identifie** une nouvelle frontiÃ¨re Ã  formaliser.

| Source d'identification | Exemple |
|-------------------------|---------|
| **Architecture** | Nouvelle zone de confiance dÃ©finie |
| **IntÃ©gration** | Nouveau systÃ¨me externe Ã  connecter |
| **Ã‰volution** | Modification de pÃ©rimÃ¨tre d'une zone existante |

**DonnÃ©es d'identification :**

| DonnÃ©e | Description |
|--------|-------------|
| **Nom** | Nom explicite de la frontiÃ¨re |
| **Type** | Externe, interne, ou intÃ©gration |
| **Zones sÃ©parÃ©es** | Quelles zones de confiance sont sÃ©parÃ©es |
| **Justification** | Pourquoi cette frontiÃ¨re existe |

#### Ã‰tape 2 : DÃ©finition des rÃ¨gles

Border Guard **dÃ©finit** les rÃ¨gles de franchissement associÃ©es Ã  la frontiÃ¨re.

| Ã‰lÃ©ment dÃ©fini | Description |
|----------------|-------------|
| **Direction** | EntrÃ©e, sortie, bidirectionnelle |
| **PermÃ©abilitÃ©** | Ouverte, contrÃ´lÃ©e, fermÃ©e |
| **Niveau requis** | Niveau de confiance minimal pour franchir |
| **Conditions** | Conditions dÃ©claratives supplÃ©mentaires |

**Exemple de rÃ¨gle dÃ©clarative :**

```
FrontiÃ¨re : BND-EXT-API
Direction : EntrÃ©e
PermÃ©abilitÃ© : ContrÃ´lÃ©e
Niveau requis : Verified
Conditions :
  - Authentification valide
  - Origine dans la liste blanche
  - Quota non dÃ©passÃ©
```

**Invariant applicable :** INV-BG-6 (RÃ¨gles dÃ©claratives)

#### Ã‰tape 3 : Validation de cohÃ©rence

Border Guard **valide** que la nouvelle dÃ©finition est cohÃ©rente avec l'existant.

| VÃ©rification | Description |
|--------------|-------------|
| **Pas de contradiction** | La rÃ¨gle ne contredit pas une rÃ¨gle existante |
| **Couverture complÃ¨te** | Pas de "trou" dans la dÃ©finition des frontiÃ¨res |
| **HiÃ©rarchie respectÃ©e** | Les zones de confiance restent cohÃ©rentes |

**Invariant applicable :** INV-BG-9 (CohÃ©rence globale)

#### Ã‰tape 4 : Enregistrement de la dÃ©finition

Border Guard **enregistre** la dÃ©finition validÃ©e.

| DonnÃ©e enregistrÃ©e | Description |
|--------------------|-------------|
| **boundary_definition** | DÃ©finition complÃ¨te de la frontiÃ¨re |
| **associated_rules** | RÃ¨gles de franchissement |
| **created_at** | Horodatage local |
| **created_by** | Source de la dÃ©finition |
| **documentation** | RÃ©fÃ©rence vers la documentation |

**Invariants applicables :** INV-BG-5 (FrontiÃ¨res explicites), INV-BG-8 (TraÃ§abilitÃ© complÃ¨te)

### 6.3 Garanties du flux de dÃ©finition

| Garantie | Description |
|----------|-------------|
| **Explicite** | Aucune frontiÃ¨re implicite (INV-BG-5) |
| **CohÃ©rent** | Validation de cohÃ©rence obligatoire (INV-BG-9) |
| **TraÃ§able** | Toute dÃ©finition est traÃ§able (INV-BG-8) |
| **DÃ©claratif** | RÃ¨gles dÃ©claratives uniquement (INV-BG-6) |

---

## 7. Flux de conseil

### 7.1 Description

Le flux de conseil est le flux par lequel Border Guard fournit le contexte de frontiÃ¨re aux autres cores. Ce flux est **passif** : Border Guard rÃ©pond aux demandes.

### 7.2 Ã‰tapes du flux

```mermaid
sequenceDiagram
    participant Core as Core demandeur
    participant BG as Border Guard
    participant Ctx as Construction contexte

    Core->>BG: 1. Demande contexte de frontiÃ¨re
    BG->>BG: 2. Recherche informations
    BG->>Ctx: 3. Construit contexte
    Ctx-->>BG: Contexte complet
    BG-->>Core: 4. Fournit contexte
    Core->>Core: Utilise pour dÃ©cision/action
```

#### Ã‰tape 1 : Demande de contexte

Un core **demande** le contexte de frontiÃ¨re pour une interaction.

| Demandeur | Type de demande |
|-----------|-----------------|
| **StrongFather** | Contexte de confiance pour Ã©valuer une intention |
| **BondingBrother** | RÃ¨gles de franchissement Ã  appliquer |
| **Caring Nanny** | Ã‰tat des frontiÃ¨res pour observation |

**ParamÃ¨tres de demande :**

| ParamÃ¨tre | Description |
|-----------|-------------|
| **interaction_id** | ID de l'interaction concernÃ©e |
| **source_id** | Source de l'interaction |
| **boundary_id** | FrontiÃ¨re traversÃ©e (si connue) |
| **context_depth** | Profondeur du contexte demandÃ© |

#### Ã‰tape 2 : Recherche des informations

Border Guard **recherche** les informations pertinentes.

| Source | Information extraite |
|--------|----------------------|
| **Registre des frontiÃ¨res** | DÃ©finition de la frontiÃ¨re |
| **Classificateur** | Niveau de confiance de la source |
| **DÃ©finisseur de rÃ¨gles** | RÃ¨gles applicables |
| **Gouverneur d'intÃ©grations** | Ã‰tat de l'intÃ©gration (si applicable) |

#### Ã‰tape 3 : Construction du contexte

Border Guard **construit** le contexte de frontiÃ¨re.

| Ã‰lÃ©ment du contexte | Description |
|---------------------|-------------|
| **boundary_info** | DÃ©finition de la frontiÃ¨re traversÃ©e |
| **source_trust_level** | Niveau de confiance de la source |
| **applicable_rules** | RÃ¨gles de franchissement applicables |
| **integration_state** | Ã‰tat de l'intÃ©gration (si applicable) |
| **recommendations** | Recommandations (informatives) |

#### Ã‰tape 4 : Fourniture du contexte

Border Guard **fournit** le contexte au demandeur.

| Destinataire | Utilisation du contexte |
|--------------|-------------------------|
| **StrongFather** | IntÃ¨gre dans l'Ã©valuation de l'intention |
| **BondingBrother** | Applique les rÃ¨gles lors de la mÃ©diation |
| **Caring Nanny** | Inclut dans l'Ã©tat global observÃ© |

**Invariant applicable :** INV-BG-3 (Aucune dÃ©cision autonome) â€” Border Guard informe, ne dÃ©cide pas.

### 7.3 Garanties du flux de conseil

| Garantie | Description |
|----------|-------------|
| **Non-bloquant** | La fourniture de contexte n'impose pas de dÃ©cision |
| **Complet** | Le contexte inclut toutes les informations pertinentes |
| **ActualisÃ©** | Le contexte reflÃ¨te l'Ã©tat actuel |
| **TraÃ§able** | Les consultations peuvent Ãªtre auditÃ©es |

---

## 8. IntÃ©gration avec les Security Protocols

Border Guard joue un rÃ´le clÃ© dans les [Security Protocols](..//..//..//miyukini-webway-system//reference//_index.md).

### 8.1 Protocoles temps rÃ©el (Online / Sync)

| Protocole | RÃ´le de Border Guard |
|-----------|----------------------|
| **RT-SEC-1** (Session Ã©phÃ©mÃ¨re) | Classification de la session selon l'origine |
| **RT-SEC-2** (Authentification en couches) | Fournit la classification de la source dans le flux d'authentification |
| **RT-SEC-4** (DÃ©tection d'anomalie) | Classification des patterns dÃ©tectÃ©s comme "hostile" |

```
RequÃªte
    â†“
Border Guard (classification source)
    â†“
Master Butler (capacitÃ©s ?)
    â†“
Caring Nanny (Ã©tat systÃ¨me ?)
    â†“
StrongFather (dÃ©cision finale)
```

### 8.2 Protocoles asynchrones (Offline / Async)

| Protocole | RÃ´le de Border Guard |
|-----------|----------------------|
| **AS-SEC-2** (Signature locale faible) | Classification du risque des intentions asynchrones |
| **NET-SEC-1** (Handshake conformitÃ©) | Validation de l'Ã©tat des frontiÃ¨res Ã  la reconnexion |
| **NET-SEC-2** (Mise Ã  jour sÃ©curisÃ©e) | Validation des frontiÃ¨res pour les mises Ã  jour |

### 8.3 Invariants de sÃ©curitÃ© portÃ©s

Border Guard est porteur des invariants de sÃ©curitÃ© suivants :

| Invariant | ResponsabilitÃ© Border Guard |
|-----------|----------------------------|
| **Aucun client n'est source de vÃ©ritÃ©** | Classification systÃ©matique des sources |
| **Toute action justifiÃ©e et traÃ§able** | TraÃ§abilitÃ© des classifications et dÃ©finitions |
| **Tout est rÃ©vocable** | CapacitÃ© de passage vers "hostile" |

---

## 9. Diagramme d'architecture complÃ¨te

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                              BORDER GUARD                                        â”‚
â”‚                                                                                  â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚
â”‚  â”‚    REGISTRE      â”‚  â”‚  CLASSIFICATEUR  â”‚  â”‚   DÃ‰FINISSEUR    â”‚              â”‚
â”‚  â”‚  DES FRONTIÃˆRES  â”‚  â”‚   DE CONFIANCE   â”‚  â”‚    DE RÃˆGLES     â”‚              â”‚
â”‚  â”‚                  â”‚  â”‚                  â”‚  â”‚                  â”‚              â”‚
â”‚  â”‚ â€¢ FrontiÃ¨res     â”‚  â”‚ â€¢ Niveaux        â”‚  â”‚ â€¢ RÃ¨gles         â”‚              â”‚
â”‚  â”‚ â€¢ Types          â”‚  â”‚ â€¢ Transitions    â”‚  â”‚ â€¢ Conditions     â”‚              â”‚
â”‚  â”‚ â€¢ PermÃ©abilitÃ©   â”‚  â”‚ â€¢ CritÃ¨res       â”‚  â”‚ â€¢ Exceptions     â”‚              â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚
â”‚           â”‚                     â”‚                     â”‚                         â”‚
â”‚           â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                         â”‚
â”‚                                 â”‚                                               â”‚
â”‚                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                  â”‚
â”‚                    â”‚     GOUVERNEUR          â”‚                                  â”‚
â”‚                    â”‚    D'INTÃ‰GRATIONS       â”‚                                  â”‚
â”‚                    â”‚                         â”‚                                  â”‚
â”‚                    â”‚ â€¢ Classifications       â”‚                                  â”‚
â”‚                    â”‚ â€¢ Ã‰tats                 â”‚                                  â”‚
â”‚                    â”‚ â€¢ Conditions            â”‚                                  â”‚
â”‚                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                  â”‚
â”‚                                 â”‚                                               â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”           â”‚
â”‚  â”‚                              â”‚                                   â”‚           â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚           â”‚
â”‚  â”‚  â”‚   FLUX     â”‚  â”‚        FLUX          â”‚  â”‚      FLUX      â”‚  â”‚           â”‚
â”‚  â”‚  â”‚CLASSIFICATIONâ”‚  â”‚    DÃ‰FINITION       â”‚  â”‚    CONSEIL     â”‚  â”‚           â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚           â”‚
â”‚  â”‚         â”‚                   â”‚                      â”‚           â”‚           â”‚
â”‚  â”‚         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜           â”‚           â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜           â”‚
â”‚                                â”‚                                               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                 â”‚
              â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
              â”‚                  â”‚                  â”‚
              â–¼                  â–¼                  â–¼
    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
    â”‚  StrongFather   â”‚ â”‚  BondingBrother â”‚ â”‚  Caring Nanny   â”‚
    â”‚  (contexte)     â”‚ â”‚  (rÃ¨gles)       â”‚ â”‚  (Ã©tat)         â”‚
    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                 â”‚
                                 â–¼
                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                      â”‚     FRONTIÃˆRES      â”‚
                      â”‚     DU SYSTÃˆME      â”‚
                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                 â”‚
                                 â–¼
                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                      â”‚   MONDE EXTÃ‰RIEUR   â”‚
                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 10. ConformitÃ© aux Lois d'Autonomie

Les flux de Border Guard respectent les [Lois d'Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md) :

| Loi | ConformitÃ© | MÃ©canisme dans l'architecture |
|-----|------------|-------------------------------|
| **LOI-1** | âœ… RÃ´le critique | DÃ©finitions locales, aucun appel externe requis |
| **LOI-2** | âœ… | L'isolement est un Ã©tat normal des frontiÃ¨res |
| **LOI-3** | âœ… | Ã‰tat local souverain des dÃ©finitions |
| **LOI-4** | âœ… | Horodatage local, pas de temps global requis |
| **LOI-5** | âœ… | Core conceptuel lÃ©ger, sans exÃ©cution |
| **LOI-6** | âœ… RÃ´le critique | ContrÃ´le explicite des Ã©changes fÃ©dÃ©rÃ©s |

**Border Guard est critique pour l'autonomie** car :

- Il contrÃ´le tout ce qui entre et sort du systÃ¨me
- Les rÃ¨gles de franchissement sont locales et chargÃ©es au dÃ©marrage
- Il valide explicitement les Ã©changes fÃ©dÃ©rÃ©s (LOI-6)

---

## 11. Invariants architecturaux

Ce document est gouvernÃ© par les invariants de la Documentation Fondatrice :

| Invariant | Ã‰noncÃ© | Application architecturale |
|-----------|--------|---------------------------|
| **INV-BG-1** | Aucune capacitÃ© d'exÃ©cution | Les composants dÃ©finissent, ils n'exÃ©cutent pas |
| **INV-BG-4** | Classification exhaustive | Le Classificateur classifie toute source |
| **INV-BG-5** | FrontiÃ¨res explicites | Le Registre formalise toute frontiÃ¨re |
| **INV-BG-6** | RÃ¨gles dÃ©claratives | Le DÃ©finisseur utilise des rÃ¨gles dÃ©claratives |
| **INV-BG-7** | SÃ©paration dÃ©finition/application | Border Guard dÃ©finit, BondingBrother applique |
| **INV-BG-8** | TraÃ§abilitÃ© complÃ¨te | Tous les flux sont traÃ§ables |
| **INV-BG-9** | CohÃ©rence globale | Validation de cohÃ©rence obligatoire |
| **INV-BG-10** | NeutralitÃ© conceptuelle | Architecture indÃ©pendante de la technologie |

---

## 12. RÃ©fÃ©rences

### Documents fondateurs

- [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

### Contrats associÃ©s

- [Border Guard - Core Interaction Contract](./Border%20Guard%20-%20Core%20Interaction%20Contract.md)

### Documents de rÃ©fÃ©rence

- [Miyukini Conceptual References - Security Protocols](..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Security Levels](..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Documentation architecturale â€” Normative  
**DÃ©rivÃ© de :** Border Guard - Documentation Fondatrice v1.5, Sections 4, 5 et 8

