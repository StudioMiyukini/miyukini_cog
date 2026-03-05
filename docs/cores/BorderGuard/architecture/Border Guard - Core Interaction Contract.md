# Border Guard - Core Interaction Contract

## 1. Contexte

Ce document formalise les **interactions de Border Guard avec les autres Cores** du Miyukini Core System. Il dÃ©finit les contrats d'interface, les flux d'Ã©change, et les responsabilitÃ©s de chaque partie dans les interactions.

Border Guard, en tant que **core de dÃ©finition des frontiÃ¨res et classification de confiance** (Strate 2 - FrontiÃ¨re), interagit avec tous les autres cores pour fournir le contexte de frontiÃ¨re nÃ©cessaire aux dÃ©cisions et aux opÃ©rations du systÃ¨me.

**Document de rÃ©fÃ©rence :** [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toute interaction entre Border Guard et les autres cores
- **Audience :** Architectes, dÃ©veloppeurs, intÃ©grateurs
- **Statut :** Document contractuel normatif â€” CONTRAT D'INTERACTION

---

## 3. Principes gÃ©nÃ©raux d'interaction

### 3.1 Nature des relations

Border Guard entretient des relations avec les autres cores qui suivent des patterns spÃ©cifiques :

| Pattern | Description | Cores concernÃ©s |
|---------|-------------|-----------------|
| **Conseil** | Border Guard fournit un contexte informatif | StrongFather |
| **ComplÃ©mentaritÃ©** | Les responsabilitÃ©s se complÃ¨tent sans chevauchement | KindMother |
| **DÃ©finition/Application** | Border Guard dÃ©finit, l'autre applique | BondingBrother |
| **Information** | Border Guard signale des changements d'Ã©tat | Caring Nanny |
| **Normative** | Border Guard reÃ§oit des rÃ¨gles de compatibilitÃ© | Ever Buddy |
| **Consultation** | Border Guard fournit des informations de frontiÃ¨re | Master Butler |
| **Escalade** | Border Guard signale le besoin d'intervention | TAMR |

### 3.2 Invariants d'interaction

**INV-INT-BG-1 : Border Guard ne dÃ©cide jamais**

Border Guard informe, classifie, dÃ©finit, mais la dÃ©cision finale appartient toujours au core appropriÃ© (StrongFather pour les dÃ©cisions stratÃ©giques).

**INV-INT-BG-2 : Border Guard n'exÃ©cute jamais**

Border Guard ne filtre pas, ne bloque pas, n'applique pas. L'exÃ©cution est du ressort de BondingBrother et des autres cores opÃ©rationnels.

**INV-INT-BG-3 : Flux explicites et traÃ§ables**

Chaque interaction a une direction explicite. Les flux bidirectionnels sont documentÃ©s comme deux flux unidirectionnels distincts.

**INV-INT-BG-4 : Aucune modification d'Ã©tat par Border Guard**

Border Guard ne modifie jamais l'Ã©tat des autres cores. Il observe, dÃ©finit, conseille, mais la modification d'Ã©tat reste sous l'autoritÃ© du core concernÃ©.

---

## 4. Relations avec chaque Core

### 4.1 Relation avec StrongFather

**Type de relation :** Conseil

**Principe fondamental :**

> StrongFather dÃ©cide si une action est autorisÃ©e. Border Guard fournit le contexte de confiance nÃ©cessaire Ã  la dÃ©cision.

**ResponsabilitÃ©s respectives :**

| Aspect | StrongFather | Border Guard |
|--------|--------------|--------------|
| DÃ©cision d'autorisation | âœ… AutoritÃ© | âŒ Aucune |
| Contexte de confiance | âŒ Consommateur | âœ… Fournisseur |
| Ã‰valuation du risque | âœ… DÃ©cision finale | âœ… Information sur la confiance |
| Ã‰valuation de l'intention | âœ… AutoritÃ© | âŒ Aucune |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚StrongFather â”‚  Demande contexte    â”‚ Border Guardâ”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Contexte frontiÃ¨re  â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚  DÃ‰CISION   â”‚                      â”‚  (aucune)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| SF â†’ BG | Demande de contexte pour une interaction | `BoundaryContextRequest` |
| BG â†’ SF | Niveau de confiance de la source | `TrustLevel` |
| BG â†’ SF | FrontiÃ¨res traversÃ©es | `CrossedBoundaries` |
| BG â†’ SF | RÃ¨gles applicables | `ApplicableRules` |

**Informations fournies par Border Guard Ã  StrongFather :**

| Information | Description | Usage par StrongFather |
|-------------|-------------|------------------------|
| `source_trust_level` | trusted, verified, unknown, hostile | Ã‰valuer la fiabilitÃ© de l'intention |
| `crossed_boundaries` | Liste des frontiÃ¨res traversÃ©es | Ã‰valuer le risque du franchissement |
| `applicable_rules` | RÃ¨gles de franchissement en vigueur | VÃ©rifier la conformitÃ© de l'intention |
| `integration_state` | Ã‰tat de l'intÃ©gration source (si applicable) | Ã‰valuer si la source est autorisÃ©e |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-SF-1** | StrongFather peut consulter Border Guard mais la dÃ©cision finale lui appartient |
| **COL-SF-2** | Border Guard ne prend jamais de dÃ©cision Ã  la place de StrongFather |
| **COL-SF-3** | StrongFather peut ignorer les informations de Border Guard (mais c'est tracÃ©) |
| **COL-SF-4** | Border Guard fournit le contexte dans un dÃ©lai garanti (non-bloquant) |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 3.2 (Relation avec Strong Father) et Section 8.1 (Flux d'information vers Strong Father)

---

### 4.2 Relation avec KindMother

**Type de relation :** ComplÃ©mentaritÃ©

**Principe fondamental :**

> KindMother gouverne les donnÃ©es et leur persistance. Border Guard gouverne les frontiÃ¨res et les niveaux de confiance. Ce qui vient de l'extÃ©rieur passe par Border Guard avant d'Ãªtre traitÃ© par KindMother.

**ResponsabilitÃ©s respectives :**

| Aspect | KindMother | Border Guard |
|--------|------------|--------------|
| Persistance des donnÃ©es | âœ… AutoritÃ© | âŒ Aucune |
| Conditions d'entrÃ©e des donnÃ©es | âŒ Non concernÃ© | âœ… DÃ©finition |
| Synchronisation | âœ… ExÃ©cution | âŒ Aucune |
| Persistance des dÃ©finitions de frontiÃ¨res | âœ… Stockage | âœ… DÃ©finition |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Border Guardâ”‚  DÃ©finitions Ã        â”‚ KindMother  â”‚
â”‚             â”‚  persister           â”‚             â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚  (dÃ©finit)  â”‚                      â”‚ (stocke)    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| BG â†’ KM | DÃ©finitions de frontiÃ¨res Ã  persister | `BoundaryDefinition` |
| BG â†’ KM | Classifications Ã  persister | `TrustClassification` |
| BG â†’ KM | RÃ¨gles Ã  persister | `CrossingRule` |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-KM-1** | Border Guard ne persiste jamais directement (INV-BG-2) |
| **COL-KM-2** | KindMother stocke les dÃ©finitions de Border Guard sans les modifier |
| **COL-KM-3** | Border Guard traite les donnÃ©es une fois qu'elles sont "Ã  l'intÃ©rieur" est du ressort de KindMother |
| **COL-KM-4** | La synchronisation des dÃ©finitions de frontiÃ¨res est gÃ©rÃ©e par KindMother |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 3.1 (Relation avec Kind Mother)

---

### 4.3 Relation avec BondingBrother

**Type de relation :** DÃ©finition/Application

**Principe fondamental :**

> Border Guard dÃ©finit les rÃ¨gles de franchissement des frontiÃ¨res. BondingBrother applique ces rÃ¨gles lors de la mÃ©diation entre les produits et l'Ã©cosystÃ¨me.

**ResponsabilitÃ©s respectives :**

| Aspect | BondingBrother | Border Guard |
|--------|----------------|--------------|
| DÃ©finition des rÃ¨gles | âŒ Consommateur | âœ… AutoritÃ© |
| Application des rÃ¨gles | âœ… ExÃ©cution | âŒ Aucune |
| MÃ©diation produits â†” cores | âœ… AutoritÃ© | âŒ Aucune |
| Filtrage aux frontiÃ¨res | âœ… ExÃ©cution | âŒ Aucune |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Border Guardâ”‚  RÃ¨gles franchissementâ”‚ BondingBrotherâ”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚               â”‚
â”‚             â”‚                      â”‚               â”‚
â”‚             â”‚  Demande rÃ¨gles     â”‚               â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚               â”‚
â”‚             â”‚                      â”‚               â”‚
â”‚  (dÃ©finit)  â”‚                      â”‚  (applique)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| BB â†’ BG | Demande de rÃ¨gles pour une frontiÃ¨re | `RulesRequest` |
| BG â†’ BB | RÃ¨gles de franchissement applicables | `CrossingRules` |
| BG â†’ BB | Niveau de confiance d'une source | `TrustLevel` |
| BB â†’ BG | Notification de franchissement effectuÃ© | `CrossingNotification` |

**Relation fondamentale et asymÃ©trique :**

Cette relation est **non nÃ©gociable** selon la Documentation Fondatrice :

- BondingBrother ne dÃ©finit **jamais** de frontiÃ¨re
- Border Guard n'applique **jamais** de rÃ¨gle
- La sÃ©paration est **absolue**

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-BB-1** | BondingBrother consulte Border Guard avant tout franchissement de frontiÃ¨re |
| **COL-BB-2** | Border Guard fournit les rÃ¨gles, BondingBrother les applique |
| **COL-BB-3** | BondingBrother notifie Border Guard des franchissements effectuÃ©s (traÃ§abilitÃ©) |
| **COL-BB-4** | Les produits ne parlent jamais directement Ã  Border Guard |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 3.3 (Relation avec Bonding Brother) et Section 8.2 (Flux de rÃ¨gles vers Bonding Brother)

---

### 4.4 Relation avec Caring Nanny

**Type de relation :** Information

**Principe fondamental :**

> Caring Nanny observe l'Ã©tat global du systÃ¨me. Border Guard informe Caring Nanny de l'Ã©tat des frontiÃ¨res pour enrichir cette observation.

**ResponsabilitÃ©s respectives :**

| Aspect | Caring Nanny | Border Guard |
|--------|--------------|--------------|
| Observation d'Ã©tat global | âœ… AutoritÃ© | âŒ Aucune |
| Ã‰tat des frontiÃ¨res | âŒ Consommateur | âœ… Fournisseur |
| Rapport de santÃ© | âœ… Production | âŒ Contribution |
| DÃ©tection d'anomalies | âœ… AutoritÃ© | âœ… Source d'information |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Border Guardâ”‚  Ã‰tat des frontiÃ¨res â”‚Caring Nanny â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚ (signale)   â”‚                      â”‚  RAPPORT    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| BG â†’ CN | Changement d'Ã©tat d'une frontiÃ¨re | `BoundaryStateChange` |
| BG â†’ CN | IntÃ©gration dÃ©faillante | `IntegrationFailure` |
| BG â†’ CN | Passage d'une source vers "hostile" | `HostileDetection` |
| BG â†’ CN | Indicateurs de santÃ© des frontiÃ¨res | `BoundaryHealthMetrics` |

**Indicateurs fournis par Border Guard :**

| Indicateur | Description | Impact sur la santÃ© |
|------------|-------------|---------------------|
| `hostile_detections` | Nombre de sources passÃ©es Ã  "hostile" | Risque de sÃ©curitÃ© |
| `unknown_sources_ratio` | Ratio de sources non classifiÃ©es | Couverture de classification |
| `integration_failures` | IntÃ©grations dÃ©faillantes | ConnectivitÃ© externe |
| `closed_boundaries` | FrontiÃ¨res fermÃ©es | Ã‰tat de verrouillage |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-CN-1** | Border Guard notifie Caring Nanny de tout changement d'Ã©tat significatif |
| **COL-CN-2** | Caring Nanny intÃ¨gre ces informations dans son rapport de santÃ© |
| **COL-CN-3** | Border Guard ne demande jamais Ã  Caring Nanny de modifier un Ã©tat |
| **COL-CN-4** | La frÃ©quence de notification est dÃ©finie par Border Guard |

**RÃ©fÃ©rence Documentation Fondatrice :** Section 3.4 (Relation avec Caring Nanny) et Section 8.3 (Flux d'Ã©tat vers Caring Nanny)

---

### 4.5 Relation avec Ever Buddy

**Type de relation :** Normative (Ever Buddy â†’ Border Guard)

**Principe fondamental :**

> Ever Buddy dÃ©finit les rÃ¨gles de compatibilitÃ© et d'Ã©volution. Border Guard applique ces rÃ¨gles aux frontiÃ¨res pour les intÃ©grations et les versions supportÃ©es.

**ResponsabilitÃ©s respectives :**

| Aspect | Ever Buddy | Border Guard |
|--------|------------|--------------|
| RÃ¨gles de compatibilitÃ© | âœ… DÃ©finition | âŒ Consommateur |
| Versions supportÃ©es aux frontiÃ¨res | âœ… DÃ©finition | âœ… Application |
| VÃ©rification d'intÃ©gration | âœ… CritÃ¨res | âœ… Contexte de frontiÃ¨re |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Ever Buddy  â”‚  RÃ¨gles compatibilitÃ©â”‚ Border Guardâ”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Rejets incompatib.  â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚  (gouverne) â”‚                      â”‚ (applique)  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| EB â†’ BG | Versions supportÃ©es par interface | `SupportedVersions` |
| EB â†’ BG | RÃ¨gles de compatibilitÃ© en vigueur | `CompatibilityRules` |
| EB â†’ BG | FenÃªtres de compatibilitÃ© | `CompatibilityWindows` |
| BG â†’ EB | IntÃ©grations refusÃ©es pour incompatibilitÃ© | `RejectionReport` |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-EB-1** | Ever Buddy dÃ©finit les versions acceptables aux frontiÃ¨res |
| **COL-EB-2** | Border Guard intÃ¨gre ces rÃ¨gles dans les conditions de franchissement |
| **COL-EB-3** | Border Guard notifie Ever Buddy des rejets pour incompatibilitÃ© |
| **COL-EB-4** | Les fenÃªtres de compatibilitÃ© sont non nÃ©gociables |

---

### 4.6 Relation avec Master Butler

**Type de relation :** Consultation

**Principe fondamental :**

> Master Butler expose les capacitÃ©s disponibles. Border Guard informe sur le niveau de confiance requis pour accÃ©der Ã  certaines capacitÃ©s selon leur sensibilitÃ©.

**ResponsabilitÃ©s respectives :**

| Aspect | Master Butler | Border Guard |
|--------|---------------|--------------|
| Catalogue des capacitÃ©s | âœ… AutoritÃ© | âŒ Aucune |
| Niveau de confiance requis | âŒ Consommateur | âœ… DÃ©finition |
| Exposition des capacitÃ©s | âœ… ExÃ©cution | âŒ Aucune |
| Filtrage selon confiance | âœ… Application | âœ… RÃ¨gles |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Border Guardâ”‚  Niveaux requis      â”‚Master Butlerâ”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Demande contexte    â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚  (dÃ©finit)  â”‚                      â”‚  (expose)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| MB â†’ BG | Demande de niveau de confiance requis pour capacitÃ© | `CapabilityTrustRequest` |
| BG â†’ MB | Niveau de confiance requis | `RequiredTrustLevel` |
| BG â†’ MB | RÃ¨gles d'accÃ¨s aux capacitÃ©s sensibles | `CapabilityAccessRules` |

**Impact sur l'exposition des capacitÃ©s :**

| Niveau de confiance source | CapacitÃ©s accessibles |
|---------------------------|----------------------|
| **Trusted** | Toutes les capacitÃ©s |
| **Verified** | CapacitÃ©s standard + certaines sensibles |
| **Unknown** | CapacitÃ©s publiques uniquement |
| **Hostile** | Aucune capacitÃ© |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-MB-1** | Master Butler peut consulter Border Guard pour les niveaux requis |
| **COL-MB-2** | Border Guard dÃ©finit les rÃ¨gles d'accÃ¨s aux capacitÃ©s sensibles |
| **COL-MB-3** | Master Butler adapte son exposition selon les rÃ¨gles de Border Guard |
| **COL-MB-4** | Les capacitÃ©s critiques sont inaccessibles pour les sources "unknown" ou "hostile" |

---

### 4.7 Relation avec TAMR

**Type de relation :** Escalade

**Principe fondamental :**

> TAMR dÃ©finit quand l'humain intervient. Border Guard signale les situations de frontiÃ¨re qui nÃ©cessitent une intervention humaine.

**ResponsabilitÃ©s respectives :**

| Aspect | TAMR | Border Guard |
|--------|------|--------------|
| Points d'intervention humaine | âœ… AutoritÃ© | âŒ Aucune |
| Signalement de besoin d'intervention | âŒ Destinataire | âœ… Ã‰metteur |
| Validation humaine des classifications | âœ… ExÃ©cution | âŒ Aucune |
| Passage vers "hostile" manuel | âœ… Validation finale | âœ… Proposition |

**Flux d'interaction :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Border Guardâ”‚  Besoin intervention â”‚    TAMR     â”‚
â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚             â”‚  Validation humaine  â”‚             â”‚
â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
â”‚             â”‚                      â”‚             â”‚
â”‚ (signale)   â”‚                      â”‚ (valide)    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Contrat d'interface :**

| Direction | DonnÃ©es Ã©changÃ©es | Format |
|-----------|-------------------|--------|
| BG â†’ TAMR | Demande de classification manuelle | `ManualClassificationRequest` |
| BG â†’ TAMR | Signalement de source suspecte | `SuspiciousSourceAlert` |
| TAMR â†’ BG | Validation de classification | `HumanClassificationValidation` |
| TAMR â†’ BG | Refus avec justification | `HumanRejection` |

**Cas nÃ©cessitant une escalade vers TAMR :**

| Cas | Description | SÃ©vÃ©ritÃ© |
|-----|-------------|----------|
| Classification ambiguÃ« | Source difficile Ã  classifier automatiquement | Moyenne |
| Passage vers "hostile" | Confirmation humaine avant blacklist | Ã‰levÃ©e |
| RÃ©vocation d'intÃ©gration | DÃ©cision de rÃ©voquer une intÃ©gration | Ã‰levÃ©e |
| Nouvelle intÃ©gration critique | IntÃ©gration avec un systÃ¨me externe sensible | Critique |
| Modification de frontiÃ¨re FONDATION | Changement de frontiÃ¨re critique | Critique |

**RÃ¨gles de collaboration :**

| ID | RÃ¨gle |
|----|-------|
| **COL-TAMR-1** | Border Guard signale automatiquement les cas d'escalade Ã  TAMR |
| **COL-TAMR-2** | TAMR peut valider ou refuser une classification proposÃ©e |
| **COL-TAMR-3** | Une classification refusÃ©e par TAMR ne peut Ãªtre forcÃ©e par Border Guard |
| **COL-TAMR-4** | La validation TAMR est enregistrÃ©e dans l'historique (traÃ§abilitÃ©) |

---

## 5. Flux d'interaction transversaux

### 5.1 Flux de contexte de confiance

Ce flux dÃ©crit comment le contexte de confiance circule de Border Guard vers les consommateurs.

**SÃ©quence :**

```
1. Source externe / Interaction
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ Source      â”‚
   â”‚ externe     â”‚
   â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”˜
          â”‚
          â–¼
2. Classification par Border Guard
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ Border Guardâ”‚
   â”‚ (classifie) â”‚
   â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”˜
          â”‚
          â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
          â–¼                 â–¼                 â–¼
3. Distribution aux consommateurs
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚StrongFather â”‚  â”‚ BondingBrotherâ”‚  â”‚Caring Nanny â”‚
   â”‚ (dÃ©cide)    â”‚  â”‚  (applique)   â”‚  â”‚ (observe)   â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**DonnÃ©es du contexte :**

| DonnÃ©e | Consommateur | Usage |
|--------|--------------|-------|
| `trust_level` | StrongFather | Facteur de dÃ©cision |
| `trust_level` | BondingBrother | RÃ¨gles de filtrage |
| `trust_level` | Caring Nanny | Indicateur de santÃ© |
| `crossed_boundaries` | StrongFather | Ã‰valuation du risque |
| `applicable_rules` | BondingBrother | Application |
| `boundary_state` | Caring Nanny | Ã‰tat global |

### 5.2 Flux de dÃ©finition de frontiÃ¨re

Ce flux dÃ©crit comment une nouvelle frontiÃ¨re est dÃ©finie et propagÃ©e.

**SÃ©quence :**

```
1. Identification du besoin
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ Nouveau besoin de frontiÃ¨re dÃ©tectÃ©     â”‚
   â”‚ (architecture, nouvelle intÃ©gration...) â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
2. DÃ©finition par Border Guard
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ Border Guard dÃ©finit :                  â”‚
   â”‚ - FrontiÃ¨re (type, direction, perm.)    â”‚
   â”‚ - RÃ¨gles de franchissement              â”‚
   â”‚ - Niveau de confiance requis            â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                       â–¼                 â–¼
3. Persistance et propagation
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ KindMother  â”‚            â”‚ BondingBrotherâ”‚
   â”‚ (persiste)  â”‚            â”‚ (reÃ§oit rÃ¨gles)â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.3 Flux de dÃ©tection hostile

Ce flux dÃ©crit comment une source est identifiÃ©e comme hostile.

**SÃ©quence :**

```
1. DÃ©tection de pattern malveillant
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ Pattern d'attaque dÃ©tectÃ©               â”‚
   â”‚ (via BondingBrother ou Caring Nanny)    â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
2. Proposition de classification hostile
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ Border Guard propose : source â†’ hostile â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
3. Escalade vers TAMR (si nÃ©cessaire)
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ Border Guardâ”‚  Demande validation  â”‚    TAMR     â”‚
   â”‚             â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚             â”‚
   â”‚             â”‚                      â”‚             â”‚
   â”‚             â”‚  Validation humaine  â”‚             â”‚
   â”‚             â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚             â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
4. Notification
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ Border Guard notifie :                  â”‚
   â”‚ - BondingBrother (blocage)              â”‚
   â”‚ - Caring Nanny (Ã©tat)                   â”‚
   â”‚ - StrongFather (contexte)               â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 6. Relation avec les produits

### 6.1 Principe fondamental

**Les produits ne parlent jamais directement Ã  Border Guard.**

Toute interaction passe par BondingBrother qui traduit et filtre les Ã©changes.

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Produits   â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚ Border Guardâ”‚
â”‚             â”‚              âŒ INTERDIT            â”‚             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    via     â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Produits   â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚BondingBrother â”‚ â”€â”€â”€â”€â–º â”‚ Border Guardâ”‚
â”‚             â”‚            â”‚               â”‚       â”‚             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
               âœ… AUTORISÃ‰
```

### 6.2 Ce que les produits peuvent demander (via BondingBrother)

| Demande | RÃ©ponse de Border Guard |
|---------|------------------------|
| "Quelle est ma classification ?" | Niveau de confiance actuel |
| "Puis-je accÃ©der Ã  X ?" | Niveau de confiance requis pour X |
| "L'intÃ©gration Y est-elle active ?" | Ã‰tat de l'intÃ©gration |

### 6.3 Ce que les produits reÃ§oivent (via BondingBrother)

| Type | Description |
|------|-------------|
| Niveau de confiance | Classification actuelle du produit |
| RÃ¨gles applicables | RÃ¨gles de franchissement qui s'appliquent |
| Alertes | Notifications de changement de classification |

---

## 7. Diagramme d'interaction globale

```
                              â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                              â”‚                BORDER GUARD                  â”‚
                              â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
                              â”‚  â”‚ Registre  â”‚  â”‚Classificatâ”‚  â”‚ RÃ¨gles    â”‚ â”‚
                              â”‚  â”‚ frontiÃ¨resâ”‚  â”‚eur confianâ”‚  â”‚ franchis. â”‚ â”‚
                              â”‚  â””â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜ â”‚
                              â”‚        â”‚              â”‚              â”‚       â”‚
                              â”‚        â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜       â”‚
                              â”‚                       â”‚                      â”‚
                              â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                                      â”‚
        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
        â”‚                                             â”‚                                             â”‚
        â–¼                                             â–¼                                             â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  KindMother   â”‚                            â”‚ StrongFather  â”‚                            â”‚BondingBrother â”‚
â”‚(complÃ©mentaire)â”‚                            â”‚   (conseil)   â”‚                            â”‚(dÃ©f./applic.) â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                            â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜
                                                                                                  â”‚
        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”         â”‚
        â”‚                                             â”‚                                 â”‚         â”‚
        â–¼                                             â–¼                                 â–¼         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Caring Nanny  â”‚                            â”‚  Ever Buddy   â”‚                  â”‚      PRODUITS         â”‚
â”‚ (information) â”‚                            â”‚  (normatif)   â”‚                  â”‚ (via BondingBrother)  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
        â”‚                                             â”‚                                 â”‚
        â–¼                                             â–¼                                 â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                  
â”‚ Master Butler â”‚                            â”‚     TAMR      â”‚                  
â”‚(consultation) â”‚                            â”‚  (escalade)   â”‚                  
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                  
```

---

## 8. SynthÃ¨se des contrats d'interface

### 8.1 Matrice des interactions

| Core | Direction | Nature | DonnÃ©es Ã©changÃ©es |
|------|-----------|--------|-------------------|
| **StrongFather** | SF â†’ BG â†’ SF | Conseil | Demande contexte â†” Contexte confiance |
| **KindMother** | BG â†’ KM | ComplÃ©mentaire | DÃ©finitions Ã  persister |
| **BondingBrother** | BB â†” BG | DÃ©finition/Application | Demande rÃ¨gles â†” RÃ¨gles Ã  appliquer |
| **Caring Nanny** | BG â†’ CN | Information | Ã‰tat frontiÃ¨res (unidirectionnel) |
| **Ever Buddy** | EB â†’ BG | Normative | RÃ¨gles compatibilitÃ© (unidirectionnel) |
| **Master Butler** | MB â†’ BG | Consultation | Demande niveaux requis |
| **TAMR** | BG â†” TAMR | Escalade | Besoin validation â†” Validation humaine |

### 8.2 Garanties de service

| Garantie | Valeur | Condition |
|----------|--------|-----------|
| Temps de rÃ©ponse consultation | < 50ms | Ã‰tat systÃ¨me normal |
| DisponibilitÃ© des dÃ©finitions | 99.9% | Hors maintenance |
| Non-blocage des flux | 100% | Invariant structural |
| TraÃ§abilitÃ© des interactions | 100% | Invariant INV-BG-8 |

---

## 9. ConformitÃ© aux Lois d'Autonomie

### 9.1 LOI-1 : Aucune dÃ©pendance externe critique

Toutes les interactions sont locales. Border Guard n'a pas besoin de service externe pour interagir avec les autres cores.

### 9.2 LOI-2 : Le systÃ¨me accepte l'isolement

En mode isolÃ©, Border Guard continue d'interagir avec les cores locaux. Les frontiÃ¨res sont dÃ©finies localement.

### 9.3 LOI-6 : L'autonomie n'empÃªche pas la fÃ©dÃ©ration

Les informations de frontiÃ¨re peuvent Ãªtre partagÃ©es entre COG via BondingBrother, avec validation explicite de Border Guard.

---

## 10. RÃ©fÃ©rences

### Documents fondateurs

- [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

### Contrats associÃ©s

- [Border Guard - Architecture & Flows](./Border%20Guard%20-%20Architecture%20&%20Flows.md)

### Documents de rÃ©fÃ©rence

- [Miyukini Conceptual References - Security Protocols](..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat normatif â€” ARCHITECTURE  
**RÃ©fÃ©rence :** Border Guard - Documentation Fondatrice v1.5, Sections 3 et 8

