# WorrySentinel - Core Interaction Contract

## 1. Contexte

Ce document formalise les **interactions de WorrySentinel avec les autres Cores** du Miyukini Core System. Il dÃ©finit les contrats d'interface, les flux d'Ã©change, et les responsabilitÃ©s de chaque partie dans les interactions impliquant la gouvernance de sÃ©curitÃ©.

WorrySentinel, en tant que **core de gouvernance transversale** (Strate 4 â€” Gouvernance de sÃ©curitÃ©), interagit avec tous les autres cores selon deux flux distincts :
- **Flux descendant (gouvernance)** : WorrySentinel impose des contraintes verticales sur les cores fonctionnels
- **Flux montant (observation)** : WorrySentinel observe et corrÃ¨le les signaux remontant des cores

**Document de rÃ©fÃ©rence :** [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toute interaction entre WorrySentinel et les autres cores
- **Audience :** Architectes, dÃ©veloppeurs, intÃ©grateurs
- **Statut :** Document contractuel normatif â€” CONTRAT D'INTERACTION

---

## 3. Principes gÃ©nÃ©raux d'interaction

### 3.1 Nature des relations

WorrySentinel entretient des relations avec les autres cores qui suivent des patterns spÃ©cifiques :

| Pattern | Description | Cores concernÃ©s |
|---------|-------------|-----------------|
| **Gouvernance** | WorrySentinel impose des contraintes sÃ©curitaires | StrongFather, MasterButler, BorderGuard, LogisticsSteward |
| **Observation** | WorrySentinel reÃ§oit des signaux pour Ã©valuer l'Ã©tat | Kernel, CaringNanny, KindMother, BondingBrother |
| **Escalade** | WorrySentinel signale le besoin d'intervention humaine | TAMR |
| **Exposition** | WorrySentinel expose la gouvernance pour consultation | MiyukiniAdmin |

### 3.2 Invariants d'interaction

**INV-INT-WS-1 : WorrySentinel gouverne mais n'exÃ©cute jamais**

WorrySentinel impose des contraintes, dÃ©finit des niveaux, dÃ©clare des Ã©tats, mais n'exÃ©cute jamais d'action technique. L'exÃ©cution est toujours du ressort des cores fonctionnels.

**INV-INT-WS-2 : WorrySentinel n'implÃ©mente jamais**

WorrySentinel ne dÃ©finit jamais de mÃ©canisme cryptographique, d'algorithme de sÃ©curitÃ©, ou de contrÃ´le technique. Il gouverne le "quoi" mais jamais le "comment".

**INV-INT-WS-3 : Flux explicites et traÃ§ables**

Chaque interaction a une direction explicite. Les flux bidirectionnels sont documentÃ©s comme deux flux unidirectionnels distincts avec traÃ§abilitÃ© complÃ¨te.

**INV-INT-WS-4 : Aucune modification d'Ã©tat par WorrySentinel**

WorrySentinel ne modifie jamais directement l'Ã©tat des autres cores. Il dÃ©clare des contraintes que les cores doivent appliquer eux-mÃªmes.

**INV-INT-WS-5 : Pression verticale, pas remplacement**

WorrySentinel agit comme une pression verticale sur les cores fonctionnels. Il contraint sans remplacer, gouverne sans se substituer aux responsabilitÃ©s des autres cores.

---

## 4. Flux d'interaction globaux

### 4.1 Flux descendant â€” Gouvernance

WorrySentinel impose des contraintes verticales sur tous les cores fonctionnels :

```
                    WorrySentinel
                         â”‚
                         â”‚ impose contraintes
                         â–¼
    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
    â”‚                    â”‚                    â”‚
    â–¼                    â–¼                    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚StrongFatherâ”‚     â”‚MasterButlerâ”‚    â”‚BorderGuardâ”‚
â”‚ sÃ©vÃ©ritÃ©  â”‚     â”‚permissions â”‚    â”‚durcissementâ”‚
â”‚ dÃ©cisions â”‚     â”‚ actives    â”‚    â”‚   I/O      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
    â”‚                    â”‚                    â”‚
    â–¼                    â–¼                    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚Logistics â”‚      â”‚   TAMR    â”‚      â”‚  Kernel  â”‚
â”‚ Steward  â”‚      â”‚  droits   â”‚      â”‚ frÃ©quenceâ”‚
â”‚ quotas   â”‚      â”‚  humains  â”‚      â”‚  sondes  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Principe :** WorrySentinel ne remplace rien. Il contraint tout.

### 4.2 Flux montant â€” Observation

WorrySentinel observe et corrÃ¨le les signaux remontant des cores :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Kernel  â”‚      â”‚BorderGuardâ”‚     â”‚StrongFatherâ”‚
â”‚ signaux  â”‚      â”‚ anomalies â”‚     â”‚ dÃ©cisions â”‚
â”‚clock, id â”‚      â”‚    I/O    â”‚     â”‚  refusÃ©es â”‚
â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜      â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜      â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜
     â”‚                 â”‚                 â”‚
     â”‚                 â–¼                 â”‚
     â”‚          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”‚
     â”‚          â”‚KindMother â”‚            â”‚
     â”‚          â”‚incohÃ©rencesâ”‚           â”‚
     â”‚          â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜            â”‚
     â”‚               â”‚                   â”‚
     â–¼               â–¼                   â–¼
    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
    â”‚                â”‚                    â”‚
    â”‚                â–¼                    â”‚
    â”‚     â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”         â”‚
    â”‚     â”‚  BondingBrother    â”‚         â”‚
    â”‚     â”‚  comportements     â”‚         â”‚
    â”‚     â”‚    produits        â”‚         â”‚
    â”‚     â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜         â”‚
    â”‚              â”‚                      â”‚
    â”‚              â–¼                      â”‚
    â”‚     â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”         â”‚
    â”‚     â”‚   CaringNanny      â”‚         â”‚
    â”‚     â”‚   consolidation    â”‚         â”‚
    â”‚     â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜         â”‚
    â”‚              â”‚                      â”‚
    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                   â”‚
                   â–¼
              WorrySentinel
           observe, corrÃ¨le,
           dÃ©clare un Ã©tat
```

**Principe :** WorrySentinel observe, corrÃ¨le, et dÃ©clare un Ã©tat global basÃ© sur les signaux consolidÃ©s.

---

## 5. Relations avec chaque Core

### 5.1 Relation avec le Kernel

**Type de relation :** Observation

**Principe fondamental :**

> Le Kernel fournit les signaux de base (clock, id, traces). WorrySentinel observe ces signaux pour Ã©valuer l'Ã©tat du systÃ¨me mais n'utilise jamais le Kernel directement pour sa logique de gouvernance.

**ResponsabilitÃ©s respectives :**

| Aspect | Kernel | WorrySentinel |
|--------|--------|---------------|
| Fourniture de signaux | âœ… AutoritÃ© | âŒ Consommateur |
| Horloge logique | âœ… Source | âŒ Utilisateur (traÃ§abilitÃ©) |
| GÃ©nÃ©ration d'identifiants | âœ… AutoritÃ© | âŒ Utilisateur (audit) |
| FrÃ©quence des sondes | âœ… ExÃ©cution | âœ… Gouvernance |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚   Kernel    â”‚  Signaux systeme    â”‚WorrySentinelâ”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Contrainte frÃ©quenceâ”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚ (exÃ©cute)   â”‚                      â”‚ (gouverne)  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| K â†’ WS | Signaux systÃ¨me (anomalies, mÃ©triques) | `SystemSignal` |
| K â†’ WS | Ã‰tat des sondes | `ProbeStatus` |
| WS â†’ K | FrÃ©quence de sondage requise | `ProbeFrequencyConstraint` |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-K-1** | WorrySentinel n'appelle jamais directement le Kernel pour ses dÃ©cisions de gouvernance |
| **COL-K-2** | Le Kernel exÃ©cute les contraintes de frÃ©quence imposÃ©es par WorrySentinel |
| **COL-K-3** | Les signaux du Kernel sont une source d'observation, pas une dÃ©pendance fonctionnelle |
| **COL-K-4** | En mode isolÃ©, WorrySentinel fonctionne sans signaux du Kernel (dÃ©gradation gracieuse) |

---

### 5.2 Relation avec StrongFather

**Type de relation :** Gouvernance

**Principe fondamental :**

> StrongFather dÃ©cide si une action est autorisÃ©e. WorrySentinel gouverne la sÃ©vÃ©ritÃ© selon laquelle StrongFather doit dÃ©cider, en fonction du niveau de sÃ©curitÃ© et de l'Ã©tat de confiance.

**ResponsabilitÃ©s respectives :**

| Aspect | StrongFather | WorrySentinel |
|--------|--------------|---------------|
| DÃ©cision d'autorisation | âœ… AutoritÃ© | âŒ Aucune |
| SÃ©vÃ©ritÃ© des politiques | âŒ ExÃ©cution | âœ… Gouvernance |
| Ã‰valuation des intentions | âœ… AutoritÃ© | âŒ Aucune |
| Niveau de sÃ©curitÃ© applicable | âŒ Consommateur | âœ… Fournisseur |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚WorrySentinelâ”‚  Niveau sÃ©curitÃ© +   â”‚ StrongFatherâ”‚
â”‚             â”‚  Ã‰tat confiance      â”‚             â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  DÃ©cisions refusÃ©es  â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚ (gouverne)  â”‚                      â”‚  (dÃ©cide)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| WS â†’ SF | Niveau de sÃ©curitÃ© applicable | `SecurityLevel` (0-4) |
| WS â†’ SF | Ã‰tat de confiance du systÃ¨me | `TrustState` (T0-T4) |
| WS â†’ SF | SÃ©vÃ©ritÃ© requise | `SeverityConstraint` |
| SF â†’ WS | DÃ©cisions refusÃ©es (pour observation) | `DecisionRejectionSignal` |

**Impact de la gouvernance sur StrongFather :**

| Ã‰tat de confiance | Impact sur les dÃ©cisions StrongFather |
|-------------------|--------------------------------------|
| **T0 (Normal)** | DÃ©cisions normales, sÃ©vÃ©ritÃ© standard |
| **T1 (Instable)** | Logging renforcÃ©, sÃ©vÃ©ritÃ© lÃ©gÃ¨rement accrue |
| **T2 (DÃ©gradÃ©)** | DÃ©cisions plus strictes, capacitÃ©s non essentielles refusÃ©es |
| **T3 (Restreint)** | DÃ©cisions critiques â†’ AMBIGUÃ‹ / DIFFÃ‰RÃ‰E, TAMR requis |
| **T4 (BloquÃ©)** | Plus aucune dÃ©cision opÃ©rationnelle autorisÃ©e |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-SF-1** | WorrySentinel ne prend jamais de dÃ©cision Ã  la place de StrongFather |
| **COL-SF-2** | StrongFather adapte sa sÃ©vÃ©ritÃ© selon les contraintes de WorrySentinel |
| **COL-SF-3** | Les dÃ©cisions refusÃ©es par StrongFather sont observÃ©es par WorrySentinel pour corrÃ©lation |
| **COL-SF-4** | StrongFather ne peut pas ignorer un Ã©tat de confiance T3+ |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 9.2 (Relation avec StrongFather)

---

### 5.3 Relation avec KindMother

**Type de relation :** Observation indirecte

**Principe fondamental :**

> KindMother persiste les donnÃ©es. WorrySentinel observe les incohÃ©rences dÃ©tectÃ©es par KindMother comme signaux d'intÃ©gritÃ©, mais n'accÃ¨de jamais directement Ã  KindMother.

**ResponsabilitÃ©s respectives :**

| Aspect | KindMother | WorrySentinel |
|--------|------------|---------------|
| Persistance des donnÃ©es | âœ… AutoritÃ© | âŒ Aucune |
| DÃ©tection d'incohÃ©rences | âœ… Source | âŒ Observateur |
| AccÃ¨s aux donnÃ©es | âœ… AutoritÃ© | âŒ INTERDIT |
| Signalement d'anomalies | âœ… Ã‰metteur | âŒ Destinataire (via CaringNanny) |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ KindMother  â”‚  IncohÃ©rences       â”‚  CaringNanny â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚  (consolide) â”‚
â”‚             â”‚                      â”‚              â”‚
â”‚             â”‚                      â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜
â”‚             â”‚                             â”‚
â”‚             â”‚                             â–¼
â”‚             â”‚                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚             â”‚                      â”‚WorrySentinelâ”‚
â”‚ (persiste)  â”‚                      â”‚ (observe)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-KM-1** | WorrySentinel n'appelle jamais KindMother directement (INV-WS-3) |
| **COL-KM-2** | Les incohÃ©rences dÃ©tectÃ©es par KindMother sont relayÃ©es via CaringNanny |
| **COL-KM-3** | WorrySentinel ne peut jamais lire ou modifier des donnÃ©es persistÃ©es |
| **COL-KM-4** | Les signaux d'incohÃ©rence contribuent Ã  l'Ã©valuation de l'Ã©tat de confiance |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 9.3 (Relation avec KindMother) â€” INV-WS-3

---

### 5.4 Relation avec CaringNanny

**Type de relation :** Observation consolidÃ©e + Proposition

**Principe fondamental :**

> CaringNanny consolide les signaux d'intÃ©gritÃ© du systÃ¨me. WorrySentinel observe ces signaux consolidÃ©s et CaringNanny peut proposer des transitions d'Ã©tat de confiance.

**ResponsabilitÃ©s respectives :**

| Aspect | CaringNanny | WorrySentinel |
|--------|-------------|---------------|
| Consolidation des signaux | âœ… AutoritÃ© | âŒ Consommateur |
| Ã‰valuation de l'intÃ©gritÃ© | âœ… Production | âœ… DÃ©cision finale |
| Proposition de transition | âœ… Ã‰metteur | âŒ Destinataire |
| DÃ©cision de transition | âŒ Aucune | âœ… AutoritÃ© |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ CaringNanny â”‚  Signaux consolidÃ©s  â”‚WorrySentinelâ”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Proposition transit.â”‚             â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Ã‰tat global actuel  â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚(consolide)  â”‚                      â”‚ (gouverne)  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| CN â†’ WS | Signaux d'intÃ©gritÃ© consolidÃ©s | `IntegritySignalBundle` |
| CN â†’ WS | Proposition de transition d'Ã©tat | `TransitionProposal` |
| CN â†’ WS | Indicateurs de santÃ© | `HealthIndicators` |
| WS â†’ CN | Ã‰tat de confiance actuel | `CurrentTrustState` |
| WS â†’ CN | RÃ¨gles de consolidation | `ConsolidationRules` |

**Structure des propositions de transition :**

```typescript
interface TransitionProposal {
  // Identification
  proposal_id: UUID;
  
  // Transition proposÃ©e
  current_state: TrustState;        // T0-T4
  proposed_state: TrustState;       // T0-T4
  
  // Justification
  signals: ConsolidatedSignal[];
  confidence_score: NormalizedScore; // 0-100
  
  // Metadata
  timestamp: LogicalClock;
}
```

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-CN-1** | CaringNanny consolide les signaux de tous les cores et les transmet Ã  WorrySentinel |
| **COL-CN-2** | CaringNanny peut proposer des transitions mais WorrySentinel dÃ©cide |
| **COL-CN-3** | WorrySentinel gouverne les rÃ¨gles selon lesquelles CaringNanny consolide |
| **COL-CN-4** | Une proposition refusÃ©e par WorrySentinel n'est pas appliquÃ©e |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 9.4 (Relation avec CaringNanny)

---

### 5.5 Relation avec BorderGuard

**Type de relation :** Gouvernance

**Principe fondamental :**

> BorderGuard dÃ©finit les frontiÃ¨res d'intÃ©gration. WorrySentinel gouverne le durcissement de ces frontiÃ¨res selon le niveau de sÃ©curitÃ© et l'Ã©tat de confiance.

**ResponsabilitÃ©s respectives :**

| Aspect | BorderGuard | WorrySentinel |
|--------|-------------|---------------|
| DÃ©finition des frontiÃ¨res | âœ… AutoritÃ© | âŒ Aucune |
| Classification de confiance | âœ… AutoritÃ© | âŒ Aucune |
| Durcissement des frontiÃ¨res | âœ… ExÃ©cution | âœ… Gouvernance |
| Signalement d'anomalies I/O | âœ… Ã‰metteur | âŒ Observateur |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚WorrySentinelâ”‚  Niveau durcissement â”‚ BorderGuard â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Anomalies I/O       â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚ (gouverne)  â”‚                      â”‚ (dÃ©finit)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| WS â†’ BG | Niveau de durcissement requis | `HardeningLevel` |
| WS â†’ BG | FrontiÃ¨res Ã  bloquer (Ã©tat T3+) | `BlockedBoundaries` |
| BG â†’ WS | Anomalies I/O dÃ©tectÃ©es | `IOAnomalySignal` |
| BG â†’ WS | Passages vers "hostile" | `HostileDetectionSignal` |

**Impact de la gouvernance sur BorderGuard :**

| Ã‰tat de confiance | Impact sur BorderGuard |
|-------------------|------------------------|
| **T0 (Normal)** | FrontiÃ¨res normales, classification standard |
| **T1 (Instable)** | Surveillance accrue des passages |
| **T2 (DÃ©gradÃ©)** | Durcissement des rÃ¨gles de franchissement |
| **T3 (Restreint)** | Fermeture des frontiÃ¨res non essentielles |
| **T4 (BloquÃ©)** | Toutes les frontiÃ¨res fermÃ©es (mode isolation) |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-BG-1** | WorrySentinel ne dÃ©finit jamais de frontiÃ¨re (responsabilitÃ© de BorderGuard) |
| **COL-BG-2** | BorderGuard applique le durcissement imposÃ© par WorrySentinel |
| **COL-BG-3** | Les anomalies I/O dÃ©tectÃ©es par BorderGuard sont observÃ©es par WorrySentinel |
| **COL-BG-4** | En Ã©tat T3+, BorderGuard doit fermer les frontiÃ¨res non essentielles |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 9.5 (Relation avec BorderGuard)

---

### 5.6 Relation avec MasterButler

**Type de relation :** Gouvernance

**Principe fondamental :**

> MasterButler expose les capacitÃ©s disponibles. WorrySentinel gouverne les permissions actives en limitant les capacitÃ©s accessibles selon le niveau de sÃ©curitÃ© et l'Ã©tat de confiance.

**ResponsabilitÃ©s respectives :**

| Aspect | MasterButler | WorrySentinel |
|--------|--------------|---------------|
| Catalogue des capacitÃ©s | âœ… AutoritÃ© | âŒ Aucune |
| Exposition des capacitÃ©s | âœ… ExÃ©cution | âŒ Aucune |
| Limitation des capacitÃ©s | âœ… ExÃ©cution | âœ… Gouvernance |
| Permissions actives | âŒ Consommateur | âœ… DÃ©finition |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚WorrySentinelâ”‚  CapacitÃ©s limitÃ©es  â”‚MasterButler â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Ã‰tat permissions    â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚ (gouverne)  â”‚                      â”‚ (expose)    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| WS â†’ MB | CapacitÃ©s Ã  limiter | `CapabilityLimitations` |
| WS â†’ MB | CapacitÃ©s Ã  bloquer (Ã©tat T2+) | `BlockedCapabilities` |
| MB â†’ WS | Ã‰tat des permissions actives | `PermissionStateReport` |

**Impact de la gouvernance sur MasterButler :**

| Ã‰tat de confiance | Impact sur MasterButler |
|-------------------|-------------------------|
| **T0 (Normal)** | Toutes les capacitÃ©s disponibles |
| **T1 (Instable)** | Logging renforcÃ© des usages de capacitÃ©s |
| **T2 (DÃ©gradÃ©)** | CapacitÃ©s sensibles limitÃ©es |
| **T3 (Restreint)** | Seules capacitÃ©s essentielles disponibles |
| **T4 (BloquÃ©)** | Aucune capacitÃ© disponible (mode diagnostic) |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-MB-1** | WorrySentinel ne modifie jamais le catalogue de capacitÃ©s |
| **COL-MB-2** | MasterButler applique les limitations imposÃ©es par WorrySentinel |
| **COL-MB-3** | MasterButler peut consulter WorrySentinel pour connaÃ®tre les permissions actives |
| **COL-MB-4** | Les capacitÃ©s critiques sont bloquÃ©es automatiquement en Ã©tat T3+ |

---

### 5.7 Relation avec BondingBrother

**Type de relation :** Observation

**Principe fondamental :**

> BondingBrother mÃ©diatise les Ã©changes entre produits et Ã©cosystÃ¨me. WorrySentinel observe les comportements des produits via BondingBrother pour dÃ©tecter des anomalies.

**ResponsabilitÃ©s respectives :**

| Aspect | BondingBrother | WorrySentinel |
|--------|----------------|---------------|
| MÃ©diation produits â†” cores | âœ… AutoritÃ© | âŒ Aucune |
| Transport des dÃ©cisions | âœ… ExÃ©cution | âŒ Aucune |
| Observation comportements | âœ… Source | âŒ Consommateur |
| Signalement d'anomalies | âœ… Ã‰metteur | âŒ Observateur |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚BondingBrotherâ”‚ Comportements       â”‚ WorrySentinel â”‚
â”‚             â”‚ produits             â”‚               â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚               â”‚
â”‚             â”‚                      â”‚               â”‚
â”‚             â”‚ Contraintes Ã©tat     â”‚               â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚               â”‚
â”‚             â”‚                      â”‚               â”‚
â”‚ (transporte)â”‚                      â”‚  (observe)    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| BB â†’ WS | Comportements anormaux des produits | `ProductBehaviorSignal` |
| BB â†’ WS | Patterns d'usage suspects | `SuspiciousPatternSignal` |
| WS â†’ BB | Contraintes liÃ©es Ã  l'Ã©tat global | `StateConstraints` |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-BB-1** | WorrySentinel n'interagit jamais directement avec les produits |
| **COL-BB-2** | BondingBrother remonte les comportements anormaux Ã  WorrySentinel |
| **COL-BB-3** | WorrySentinel peut imposer des contraintes sur les Ã©changes en Ã©tat dÃ©gradÃ© |
| **COL-BB-4** | BondingBrother informe les produits de l'Ã©tat global (via contraintes) |

---

### 5.8 Relation avec LogisticsSteward

**Type de relation :** Supervision + Gouvernance

**Principe fondamental :**

> LogisticsSteward gouverne l'allocation des ressources. WorrySentinel supervise LogisticsSteward pour dÃ©tecter les dÃ©rives et peut imposer un durcissement des rÃ¨gles d'arbitrage.

**ResponsabilitÃ©s respectives :**

| Aspect | LogisticsSteward | WorrySentinel |
|--------|------------------|---------------|
| Gouvernance des ressources | âœ… AutoritÃ© | âŒ Aucune |
| Arbitrage de quotas | âœ… AutoritÃ© | âŒ Aucune |
| Durcissement des quotas | âœ… ExÃ©cution | âœ… DÃ©clenchement |
| DÃ©tection de dÃ©rives | âŒ Source | âœ… Observateur |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚WorrySentinelâ”‚                      â”‚LogisticsStewardâ”‚
â”‚             â”‚ â†â”€â”€ signaux alloc â”€â”€ â”‚               â”‚
â”‚             â”‚                      â”‚               â”‚
â”‚             â”‚ â”€â”€ contraintes â”€â”€â”€â†’ â”‚               â”‚
â”‚             â”‚                      â”‚               â”‚
â”‚             â”‚ â”€â”€ durcissement â”€â”€â†’ â”‚               â”‚
â”‚             â”‚    (si T1+)          â”‚               â”‚
â”‚             â”‚                      â”‚               â”‚
â”‚             â”‚ â†â”€â”€ confirmation â”€â”€â”€ â”‚               â”‚
â”‚             â”‚                      â”‚               â”‚
â”‚(supervise)  â”‚                      â”‚ (arbitre)     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| LS â†’ WS | Signaux d'allocation | `AllocationSignal` |
| LS â†’ WS | DÃ©rives dÃ©tectÃ©es | `AllocationDrift` |
| WS â†’ LS | Contraintes sÃ©curitaires | `SecurityConstraints` |
| WS â†’ LS | Directive de durcissement | `HardeningDirective` |
| LS â†’ WS | Confirmation d'application | `ApplicationConfirmation` |

**RÃ¨gles d'interaction (RÃˆGLE-WS-LS-*) :**

| ID | RÃ¨gle |
|----|-------|
| **RÃˆGLE-WS-LS-1** | WorrySentinel peut imposer des contraintes sÃ©curitaires sur les dÃ©cisions d'arbitrage de LogisticsSteward |
| **RÃˆGLE-WS-LS-2** | En Ã©tat T2+, LogisticsSteward doit appliquer des quotas plus restrictifs selon les directives de WorrySentinel |
| **RÃˆGLE-WS-LS-3** | WorrySentinel observe les patterns d'allocation de ressources pour dÃ©tecter des anomalies sÃ©curitaires |
| **RÃˆGLE-WS-LS-4** | Toute dÃ©rive d'allocation signalÃ©e par WorrySentinel doit Ãªtre traitÃ©e par LogisticsSteward |

**Impact de la gouvernance sur LogisticsSteward :**

| Ã‰tat de confiance | Impact sur LogisticsSteward |
|-------------------|----------------------------|
| **T0 (Normal)** | Arbitrage normal, quotas standards |
| **T1 (Instable)** | Surveillance accrue des allocations |
| **T2 (DÃ©gradÃ©)** | Quotas rÃ©duits, prioritÃ©s aplaties |
| **T3 (Restreint)** | Quotas minimaux, ressources essentielles uniquement |
| **T4 (BloquÃ©)** | Gel des allocations, mode maintenance |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-LS-1** | WorrySentinel ne se substitue jamais Ã  LogisticsSteward pour l'arbitrage |
| **COL-LS-2** | LogisticsSteward reste souverain sur les dÃ©cisions d'allocation |
| **COL-LS-3** | WorrySentinel supervise sans remplacer |
| **COL-LS-4** | Les directives de durcissement sont obligatoires en Ã©tat T2+ |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 9.6 (Relation avec LogisticsSteward)

---

### 5.9 Relation avec TAMR

**Type de relation :** Escalade + Gouvernance

**Principe fondamental :**

> TAMR dÃ©finit quand l'humain intervient. WorrySentinel gouverne les droits humains selon l'Ã©tat de confiance et signale les situations nÃ©cessitant une intervention.

**ResponsabilitÃ©s respectives :**

| Aspect | TAMR | WorrySentinel |
|--------|------|---------------|
| Points d'intervention humaine | âœ… AutoritÃ© | âŒ Aucune |
| Validation humaine | âœ… ExÃ©cution | âŒ Aucune |
| Droits humains actifs | âŒ ExÃ©cution | âœ… Gouvernance |
| Signalement besoin intervention | âŒ Destinataire | âœ… Ã‰metteur |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚WorrySentinelâ”‚  Droits humains      â”‚    TAMR     â”‚
â”‚             â”‚  applicables         â”‚             â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Besoin intervention â”‚             â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Override Ã©tat       â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚ (gouverne)  â”‚                      â”‚ (valide)    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| WS â†’ TAMR | Droits humains applicables selon Ã©tat | `HumanRightsConstraints` |
| WS â†’ TAMR | Demande d'intervention | `InterventionRequest` |
| TAMR â†’ WS | Override d'Ã©tat de confiance | `TrustStateOverride` |
| TAMR â†’ WS | Validation de transition | `TransitionValidation` |

**Cas nÃ©cessitant une escalade vers TAMR :**

| Cas | Description | SÃ©vÃ©ritÃ© |
|-----|-------------|----------|
| Transition vers T3 | Confirmation humaine requise | Ã‰levÃ©e |
| Transition vers T4 | Confirmation humaine obligatoire | Critique |
| Override d'Ã©tat | Humain souhaite modifier l'Ã©tat | Variable |
| AmbiguÃ¯tÃ© sÃ©curitaire | Signaux contradictoires | Moyenne |
| Sortie de T4 | Restauration du systÃ¨me | Critique |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-TAMR-1** | WorrySentinel signale automatiquement les cas d'escalade Ã  TAMR |
| **COL-TAMR-2** | TAMR peut valider ou refuser une transition d'Ã©tat proposÃ©e |
| **COL-TAMR-3** | En Ã©tat T3+, TAMR est requis pour toute dÃ©cision critique |
| **COL-TAMR-4** | TAMR peut forcer un override d'Ã©tat (traÃ§abilitÃ© obligatoire) |
| **COL-TAMR-5** | La sortie de T4 nÃ©cessite obligatoirement une validation TAMR |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 9.5 (Relation avec TAMR)

---

### 5.10 Relation avec MiyukiniAdmin

**Type de relation :** Exposition + Configuration

**Principe fondamental :**

> MiyukiniAdmin est l'interface d'administration. WorrySentinel expose la gouvernance de sÃ©curitÃ© pour consultation et permet une configuration limitÃ©e sous validation StrongFather.

**ResponsabilitÃ©s respectives :**

| Aspect | MiyukiniAdmin | WorrySentinel |
|--------|---------------|---------------|
| Interface d'administration | âœ… AutoritÃ© | âŒ Aucune |
| Consultation gouvernance | âœ… Client | âœ… Fournisseur |
| Configuration gouvernance | âœ… Demandeur | âœ… Sous validation SF |
| Modification directe | âŒ INTERDIT | âŒ N/A |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚MiyukiniAdminâ”‚  Consultation Ã©tat   â”‚WorrySentinelâ”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Ã‰tat + historique   â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Demande config      â”‚             â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚       â”‚              â”‚             â”‚
â”‚             â”‚       â–¼              â”‚             â”‚
â”‚             â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚             â”‚
â”‚             â”‚  â”‚ StrongFatherâ”‚    â”‚             â”‚
â”‚             â”‚  â”‚ (validation)â”‚    â”‚             â”‚
â”‚             â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚ (administre)â”‚                      â”‚ (expose)    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| MA â†’ WS | Demande Ã©tat actuel | `StateQuery` |
| WS â†’ MA | Ã‰tat de confiance + niveau sÃ©curitÃ© | `GovernanceState` |
| WS â†’ MA | Historique des transitions | `TransitionHistory` |
| MA â†’ WS | Demande de configuration | `ConfigurationRequest` |
| WS â†’ MA | RÃ©sultat de configuration (aprÃ¨s validation SF) | `ConfigurationResult` |

**Interactions autorisÃ©es (INTERACTION-ADMIN-*) :**

| ID | Interaction | Validation requise |
|----|-------------|-------------------|
| **INTERACTION-ADMIN-1** | Consultation des niveaux de sÃ©curitÃ© | Non |
| **INTERACTION-ADMIN-2** | Consultation des Ã©tats de confiance | Non |
| **INTERACTION-ADMIN-3** | Configuration de la gouvernance | Oui (StrongFather) |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **RÃˆGLE-ADMIN-1** | Toute configuration de gouvernance par MiyukiniAdmin doit Ãªtre validÃ©e par StrongFather |
| **RÃˆGLE-ADMIN-2** | Toute interaction avec MiyukiniAdmin concernant la gouvernance de sÃ©curitÃ© est tracÃ©e avec identitÃ©, moment, et justification |
| **COL-MA-1** | MiyukiniAdmin peut consulter librement l'Ã©tat de gouvernance |
| **COL-MA-2** | MiyukiniAdmin ne peut jamais modifier directement l'Ã©tat de confiance |
| **COL-MA-3** | Les configurations sont soumises Ã  validation, pas imposÃ©es |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 11 (Interaction avec MiyukiniAdmin)

---

## 6. Relation avec les produits

### 6.1 Principe fondamental

**Les produits ne parlent jamais directement Ã  WorrySentinel.**

Toute interaction passe par BondingBrother qui mÃ©diatise les Ã©changes.

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Produits   â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚WorrySentinelâ”‚
â”‚             â”‚              âŒ INTERDIT            â”‚             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    via     â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Produits   â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚BondingBrother â”‚ â”€â”€â”€â”€â–º â”‚WorrySentinelâ”‚
â”‚             â”‚            â”‚               â”‚       â”‚ (observation)â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
               âœ… AUTORISÃ‰
```

### 6.2 Ce que les produits reÃ§oivent (via BondingBrother)

| Type | Description |
|------|-------------|
| Ã‰tat global | Ã‰tat de confiance actuel (T0-T4) |
| Contraintes | Limitations actives dues Ã  l'Ã©tat |
| Alertes | Notifications de changement d'Ã©tat |

### 6.3 Ce que les produits ne peuvent pas demander

| Demande | Statut |
|---------|--------|
| Modification de l'Ã©tat de confiance | âŒ INTERDIT |
| Bypass des contraintes de sÃ©curitÃ© | âŒ INTERDIT |
| Configuration directe de WorrySentinel | âŒ INTERDIT |

---

## 7. Diagramme d'interaction globale

```
                                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                                    â”‚              WORRY SENTINEL                     â”‚
                                    â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
                                    â”‚  â”‚ Niveaux   â”‚  â”‚  Ã‰tats    â”‚  â”‚DÃ©gradationâ”‚   â”‚
                                    â”‚  â”‚ sÃ©curitÃ©  â”‚  â”‚ confiance â”‚  â”‚progressiveâ”‚   â”‚
                                    â”‚  â”‚   (0-4)   â”‚  â”‚  (T0-T4)  â”‚  â”‚           â”‚   â”‚
                                    â”‚  â””â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜   â”‚
                                    â”‚        â”‚              â”‚              â”‚         â”‚
                                    â”‚        â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜         â”‚
                                    â”‚                       â”‚                        â”‚
                                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                                            â”‚
           â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
           â”‚                         â”‚                      â”‚                       â”‚                        â”‚
           â”‚ FLUX DESCENDANT         â”‚                      â”‚                       â”‚         FLUX MONTANT   â”‚
           â”‚ (gouvernance)           â–¼                      â”‚                       â–¼         (observation)  â”‚
           â”‚                 â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚               â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                â”‚
           â”‚                 â”‚ StrongFather  â”‚              â”‚               â”‚    Kernel     â”‚                â”‚
           â”‚                 â”‚  (sÃ©vÃ©ritÃ©)   â”‚              â”‚               â”‚  (signaux)    â”‚                â”‚
           â”‚                 â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚               â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                â”‚
           â”‚                         â”‚                      â”‚                       â”‚                        â”‚
           â–¼                         â–¼                      â”‚                       â–¼                        â”‚
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”         â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚               â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                â”‚
   â”‚ MasterButler  â”‚         â”‚  BorderGuard  â”‚              â”‚               â”‚  KindMother   â”‚                â”‚
   â”‚ (permissions) â”‚         â”‚(durcissement) â”‚              â”‚               â”‚(incohÃ©rences) â”‚                â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚               â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜                â”‚
           â”‚                         â”‚                      â”‚                       â”‚                        â”‚
           â–¼                         â–¼                      â”‚                       â–¼                        â”‚
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”         â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚               â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                â”‚
   â”‚Logistics      â”‚         â”‚     TAMR      â”‚              â”‚               â”‚BondingBrother â”‚                â”‚
   â”‚Steward(quotas)â”‚         â”‚(droits humain)â”‚              â”‚               â”‚ (comportementsâ”‚                â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚               â”‚   produits)   â”‚                â”‚
                                                            â”‚               â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜                â”‚
                                                            â”‚                       â”‚                        â”‚
                                                            â”‚                       â–¼                        â”‚
                                                            â”‚               â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                â”‚
                                                            â”‚               â”‚  CaringNanny  â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                                            â”‚               â”‚(consolidation)â”‚
                                                            â”‚               â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜
                                                            â”‚                       â”‚
                                                            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                                                    Propositions
                                                                    de transition
```

---

## 8. SynthÃ¨se des contrats d'interface

### 8.1 Matrice des interactions

| Core | Direction | Nature | DonnÃ©es Ã©changÃ©es |
|------|-----------|--------|-------------------|
| **Kernel** | K â†’ WS, WS â†’ K | Observation | Signaux â†” Contraintes sondes |
| **StrongFather** | WS â†’ SF, SF â†’ WS | Gouvernance | Niveaux/Ã©tats â†’ DÃ©cisions refusÃ©es |
| **KindMother** | KM â†’ CN â†’ WS | Observation indirecte | IncohÃ©rences (via CaringNanny) |
| **CaringNanny** | CN â†” WS | Observation + Proposition | Signaux consolidÃ©s â†” Ã‰tat actuel |
| **BorderGuard** | WS â†’ BG, BG â†’ WS | Gouvernance | Durcissement â†” Anomalies I/O |
| **MasterButler** | WS â†’ MB, MB â†’ WS | Gouvernance | CapacitÃ©s limitÃ©es â†” Ã‰tat permissions |
| **BondingBrother** | BB â†’ WS, WS â†’ BB | Observation | Comportements â†” Contraintes |
| **LogisticsSteward** | LS â†” WS | Supervision + Gouvernance | Signaux alloc â†” Contraintes/Durcissement |
| **TAMR** | WS â†’ TAMR, TAMR â†’ WS | Escalade + Gouvernance | Droits/Interventions â†” Overrides |
| **MiyukiniAdmin** | MA â†’ WS, WS â†’ MA | Exposition | Consultation â†” Ã‰tat/Configuration |

### 8.2 Garanties de service

| Garantie | Valeur | Condition |
|----------|--------|-----------|
| DisponibilitÃ© de la gouvernance | 100% | Invariant structural |
| TraÃ§abilitÃ© des interactions | 100% | Invariant INV-WS-8 |
| Non-blocage des flux | 100% | Invariant structural |
| CohÃ©rence inter-Ã©tats | 100% | Invariant INV-GOV-2 |

---

## 9. ConformitÃ© aux Lois d'Autonomie

### 9.1 LOI-1 : Aucune dÃ©pendance externe critique

Toutes les interactions sont locales. WorrySentinel n'a pas besoin de service externe pour interagir avec les autres cores.

### 9.2 LOI-2 : Le systÃ¨me accepte l'isolement

En mode isolÃ©, WorrySentinel continue de gouverner la sÃ©curitÃ© localement. Les Ã©tats de confiance sont maintenus sans dÃ©pendance externe.

### 9.3 LOI-6 : L'autonomie n'empÃªche pas la fÃ©dÃ©ration

Les informations de gouvernance peuvent Ãªtre partagÃ©es entre COG via BondingBrother, avec contraintes de WorrySentinel.

---

## 10. RÃ©fÃ©rences

### Documents fondateurs

- [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

### Contrats associÃ©s

- [WorrySentinel - Architecture & Flows](./WorrySentinel%20-%20Architecture%20&%20Flows.md)

### Documents de rÃ©fÃ©rence

- [Miyukini Conceptual References - Security Levels](..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Integrity Degradation System](..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 11. Mini log de gÃ©nÃ©ration

### DÃ©cision structurelle D1 : SÃ©paration flux descendant/montant

**DÃ©cision prise :** Le document est structurÃ© autour des deux flux fondamentaux de WorrySentinel (gouvernance et observation) pour reflÃ©ter sa nature de pression verticale transversale.

**Application :** Section 4 dÃ©diÃ©e aux flux globaux, et chaque relation avec un core prÃ©cise sa direction principale.

### DÃ©cision structurelle D2 : Relations multiples avec certains cores

**DÃ©cision prise :** Certains cores ont des relations bidirectionnelles avec WorrySentinel (ex: CaringNanny, LogisticsSteward). Chaque direction est documentÃ©e comme flux distinct.

**Application :** Contrats d'interface avec directions explicites pour chaque Ã©change.

### VÃ©rification de cohÃ©rence

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec Documentation Fondatrice : Toutes les relations documentÃ©es en Section 9
- âœ… Respect INV-WS-1 : Aucune autoritÃ© sur l'implÃ©mentation
- âœ… Respect INV-WS-2 : Aucune autoritÃ© sur l'exÃ©cution
- âœ… Respect INV-WS-3 : Aucune autoritÃ© sur la persistance
- âœ… Respect INV-WS-4 : Aucune modification d'Ã©tat directe
- âœ… Respect INV-WS-5 : Aucune logique temporelle technique
- âœ… Flux descendant conforme : Section 9 (gouvernance)
- âœ… Flux montant conforme : Section 9 (observation)
- âœ… Relation LogisticsSteward conforme : RÃˆGLE-WS-LS-1 Ã  RÃˆGLE-WS-LS-4

**Conclusion :** Aucune contradiction dÃ©tectÃ©e. Le document est cohÃ©rent avec la Documentation Fondatrice.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat normatif â€” ARCHITECTURE  
**RÃ©fÃ©rence :** WorrySentinel - Documentation Fondatrice v1.2, Sections 9 et 11

