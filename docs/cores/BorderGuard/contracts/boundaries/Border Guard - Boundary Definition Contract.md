# Border Guard - Boundary Definition Contract

## 1. Contexte

Ce document dÃ©finit les **types de frontiÃ¨res** reconnus par Border Guard dans l'Ã©cosystÃ¨me Miyukini. Il spÃ©cifie formellement ce qu'est une frontiÃ¨re, ses propriÃ©tÃ©s, ses caractÃ©ristiques, et la taxonomie complÃ¨te des types de frontiÃ¨res.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non nÃ©gociable**. Il dÃ©rive directement de la Documentation Fondatrice (Section 4 - Concepts fondamentaux : FrontiÃ¨re).

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toute dÃ©marcation conceptuelle entre zones de confiance dans l'Ã©cosystÃ¨me Miyukini
- **Responsable :** Border Guard (responsabilitÃ© exclusive de dÃ©finition des frontiÃ¨res - INV-BG-5)
- **Consommateurs :** StrongFather (contexte de confiance), BondingBrother (application des rÃ¨gles), CaringNanny (Ã©tat des frontiÃ¨res)
- **Ne couvre pas :** L'application technique des frontiÃ¨res (responsabilitÃ© de BondingBrother)

---

## 3. DÃ©finition canonique de la frontiÃ¨re

### 3.1 Qu'est-ce qu'une frontiÃ¨re ?

Une **frontiÃ¨re** est une dÃ©marcation conceptuelle qui sÃ©pare deux zones de confiance diffÃ©rentes. Elle reprÃ©sente le point de transition entre un niveau de confiance et un autre.

**CaractÃ©ristiques fondamentales :**

1. **Conceptuelle** â€” Une frontiÃ¨re est une abstraction, pas une implÃ©mentation technique
2. **Explicite** â€” Toute frontiÃ¨re doit Ãªtre formellement dÃ©finie et documentÃ©e (INV-BG-5)
3. **Stable** â€” Une frontiÃ¨re possÃ¨de une identitÃ© unique et pÃ©renne
4. **OrientÃ©e** â€” Une frontiÃ¨re a une direction (entrÃ©e, sortie, bidirectionnelle)
5. **PermÃ©able** â€” Une frontiÃ¨re a un niveau de permÃ©abilitÃ© dÃ©finissant sa propension au franchissement

**Ce qu'une frontiÃ¨re n'est PAS :**

- âŒ Un firewall technique
- âŒ Une rÃ¨gle de filtrage
- âŒ Un point de contrÃ´le d'authentification
- âŒ Une implÃ©mentation de sÃ©curitÃ©

### 3.2 ResponsabilitÃ© de Border Guard

Border Guard est **exclusivement responsable** de la dÃ©finition formelle des frontiÃ¨res du systÃ¨me. Cette responsabilitÃ© inclut :

- Identifier et nommer chaque frontiÃ¨re
- Classifier la nature de chaque frontiÃ¨re (externe, interne, intÃ©gration)
- DÃ©finir la direction de chaque frontiÃ¨re (entrÃ©e, sortie, bidirectionnelle)
- Ã‰tablir le niveau de permÃ©abilitÃ© de chaque frontiÃ¨re
- Maintenir le registre exhaustif des frontiÃ¨res du systÃ¨me

**Invariant associÃ© :** INV-BG-5 â€” Toute frontiÃ¨re **doit** Ãªtre explicitement dÃ©finie et documentÃ©e. Aucune frontiÃ¨re implicite n'est autorisÃ©e.

---

## 4. PropriÃ©tÃ©s d'une frontiÃ¨re

Toute frontiÃ¨re possÃ¨de les propriÃ©tÃ©s obligatoires suivantes :

### 4.1 IdentitÃ©

| PropriÃ©tÃ© | Description | Obligatoire |
|-----------|-------------|-------------|
| **Identifiant** | Identifiant unique et stable dans le systÃ¨me | âœ… Oui |
| **Nom** | Nom descriptif et non ambigu | âœ… Oui |
| **Description** | Description de la frontiÃ¨re et de sa raison d'Ãªtre | âœ… Oui |
| **Date de crÃ©ation** | Horodatage de crÃ©ation de la frontiÃ¨re | âœ… Oui |

### 4.2 Classification

| PropriÃ©tÃ© | Description | Obligatoire |
|-----------|-------------|-------------|
| **Type** | Type de frontiÃ¨re (externe, interne, intÃ©gration) | âœ… Oui |
| **Zone source** | Zone de confiance cÃ´tÃ© source | âœ… Oui |
| **Zone destination** | Zone de confiance cÃ´tÃ© destination | âœ… Oui |

### 4.3 Comportement

| PropriÃ©tÃ© | Description | Obligatoire |
|-----------|-------------|-------------|
| **Direction** | Direction du flux autorisÃ© (entrÃ©e, sortie, bidirectionnelle) | âœ… Oui |
| **PermÃ©abilitÃ©** | Niveau de permÃ©abilitÃ© (ouverte, contrÃ´lÃ©e, fermÃ©e) | âœ… Oui |
| **RÃ¨gles associÃ©es** | RÃ©fÃ©rences aux rÃ¨gles de franchissement applicables | âœ… Oui |

### 4.4 TraÃ§abilitÃ©

| PropriÃ©tÃ© | Description | Obligatoire |
|-----------|-------------|-------------|
| **Origine** | Qui a crÃ©Ã© cette frontiÃ¨re | âœ… Oui |
| **Justification** | Pourquoi cette frontiÃ¨re existe | âœ… Oui |
| **Historique** | Historique des modifications | âœ… Oui |

**Invariant associÃ© :** INV-BG-8 â€” Toute dÃ©finition de frontiÃ¨re est **traÃ§able** avec son origine, sa date, et sa justification.

---

## 5. Taxonomie des types de frontiÃ¨res

Border Guard reconnaÃ®t trois types canoniques de frontiÃ¨res.

### 5.1 FrontiÃ¨re externe

**DÃ©finition :** SÃ©pare l'Ã©cosystÃ¨me Miyukini du monde extÃ©rieur (internet, systÃ¨mes tiers, utilisateurs non authentifiÃ©s). C'est la limite entre le "dehors" et le "dedans".

| Aspect | SpÃ©cification |
|--------|---------------|
| **Zone source** | Monde extÃ©rieur (unknown ou hostile par dÃ©faut) |
| **Zone destination** | Ã‰cosystÃ¨me Miyukini |
| **Confiance par dÃ©faut** | Unknown (aucune confiance accordÃ©e a priori) |
| **Direction typique** | EntrÃ©e (flux venant de l'extÃ©rieur vers l'intÃ©rieur) |
| **PermÃ©abilitÃ© typique** | ContrÃ´lÃ©e (vÃ©rifications systÃ©matiques) |

**Exemples de frontiÃ¨res externes :**

- FrontiÃ¨re API publique â€” Point d'entrÃ©e des requÃªtes HTTP externes
- FrontiÃ¨re utilisateur non authentifiÃ© â€” Point d'entrÃ©e des utilisateurs anonymes
- FrontiÃ¨re webhook â€” Point d'entrÃ©e des notifications externes
- FrontiÃ¨re rÃ©seau â€” Point d'entrÃ©e des connexions rÃ©seau

**Implications :**

- Tout ce qui traverse une frontiÃ¨re externe est prÃ©sumÃ© "unknown" jusqu'Ã  classification
- Les rÃ¨gles de franchissement sont restrictives par dÃ©faut
- Bonding Brother applique des contrÃ´les systÃ©matiques

### 5.2 FrontiÃ¨re interne

**DÃ©finition :** SÃ©pare diffÃ©rentes zones de confiance au sein de l'Ã©cosystÃ¨me (zone admin vs zone utilisateur, module sensible vs module standard, donnÃ©es critiques vs donnÃ©es publiques).

| Aspect | SpÃ©cification |
|--------|---------------|
| **Zone source** | Zone interne avec niveau de confiance X |
| **Zone destination** | Zone interne avec niveau de confiance Y (X â‰  Y) |
| **Confiance par dÃ©faut** | HÃ©ritÃ©e de la zone source |
| **Direction typique** | Bidirectionnelle (selon les rÃ¨gles) |
| **PermÃ©abilitÃ© typique** | Variable (selon les zones) |

**Exemples de frontiÃ¨res internes :**

- FrontiÃ¨re admin/utilisateur â€” Entre l'espace d'administration et l'espace utilisateur
- FrontiÃ¨re donnÃ©es sensibles â€” Entre les donnÃ©es critiques et les donnÃ©es standard
- FrontiÃ¨re cores â€” Entre diffÃ©rents cores du systÃ¨me (sauf pour les flux explicites)
- FrontiÃ¨re module critique â€” Autour d'un module Ã  haute sÃ©curitÃ© (niveau 3-4)

**Implications :**

- Les frontiÃ¨res internes permettent la dÃ©fense en profondeur
- Chaque zone interne peut avoir ses propres rÃ¨gles de franchissement
- La confiance peut varier entre zones internes

### 5.3 FrontiÃ¨re d'intÃ©gration

**DÃ©finition :** SÃ©pare l'Ã©cosystÃ¨me d'un systÃ¨me externe avec lequel il interagit de maniÃ¨re contrÃ´lÃ©e (API partenaire, service tiers, base de donnÃ©es externe).

| Aspect | SpÃ©cification |
|--------|---------------|
| **Zone source** | Ã‰cosystÃ¨me Miyukini ou systÃ¨me externe intÃ©grÃ© |
| **Zone destination** | SystÃ¨me externe intÃ©grÃ© ou Ã©cosystÃ¨me Miyukini |
| **Confiance par dÃ©faut** | Selon classification de l'intÃ©gration (verified typiquement) |
| **Direction typique** | Bidirectionnelle (Ã©changes avec le systÃ¨me intÃ©grÃ©) |
| **PermÃ©abilitÃ© typique** | ContrÃ´lÃ©e (protocoles d'intÃ©gration) |

**Exemples de frontiÃ¨res d'intÃ©gration :**

- FrontiÃ¨re Supabase â€” Avec le backend Supabase
- FrontiÃ¨re API partenaire â€” Avec une API tierce certifiÃ©e
- FrontiÃ¨re service de paiement â€” Avec un processeur de paiement (Stripe, etc.)
- FrontiÃ¨re service d'authentification â€” Avec un IdP externe (OAuth, SAML)

**Implications :**

- Une intÃ©gration peut Ãªtre classifiÃ©e "verified" si elle respecte les protocoles
- L'Ã©tat de l'intÃ©gration peut Ãªtre signalÃ© Ã  CaringNanny
- Les rÃ¨gles de franchissement sont spÃ©cifiques Ã  chaque intÃ©gration

---

## 6. Niveaux de permÃ©abilitÃ©

La permÃ©abilitÃ© caractÃ©rise la propension d'une frontiÃ¨re Ã  autoriser le franchissement.

### 6.1 PermÃ©abilitÃ© ouverte

**DÃ©finition :** Franchissement libre sous conditions minimales.

| Aspect | SpÃ©cification |
|--------|---------------|
| **VÃ©rification** | Minimale (validation structurelle uniquement) |
| **Blocage** | Rare (uniquement en cas d'anomalie Ã©vidente) |
| **Usage typique** | FrontiÃ¨res vers des zones publiques |
| **Niveau de sÃ©curitÃ© associÃ©** | 0 (PUBLIC / DISPLAY) |

**Exemples :**

- FrontiÃ¨re vers une API publique en lecture seule
- FrontiÃ¨re vers des ressources statiques
- FrontiÃ¨re vers des donnÃ©es publiques

### 6.2 PermÃ©abilitÃ© contrÃ´lÃ©e

**DÃ©finition :** Franchissement soumis Ã  vÃ©rification selon les rÃ¨gles dÃ©finies.

| Aspect | SpÃ©cification |
|--------|---------------|
| **VÃ©rification** | SystÃ©matique (selon rÃ¨gles de franchissement) |
| **Blocage** | Conditionnel (si rÃ¨gles non respectÃ©es) |
| **Usage typique** | FrontiÃ¨res standard, intÃ©grations |
| **Niveau de sÃ©curitÃ© associÃ©** | 1-3 (STANDARD Ã  CRITICAL) |

**Exemples :**

- FrontiÃ¨re utilisateur authentifiÃ©
- FrontiÃ¨re d'intÃ©gration avec API partenaire
- FrontiÃ¨re vers des donnÃ©es sensibles

### 6.3 PermÃ©abilitÃ© fermÃ©e

**DÃ©finition :** Franchissement interdit sauf conditions exceptionnelles.

| Aspect | SpÃ©cification |
|--------|---------------|
| **VÃ©rification** | Maximale (toutes les conditions doivent Ãªtre satisfaites) |
| **Blocage** | Par dÃ©faut (franchissement exceptionnel) |
| **Usage typique** | FrontiÃ¨res vers des zones critiques, isolement |
| **Niveau de sÃ©curitÃ© associÃ©** | 4 (HARDENED / ISOLATED) |

**Exemples :**

- FrontiÃ¨re vers des clÃ©s cryptographiques
- FrontiÃ¨re en mode quarantaine
- FrontiÃ¨re vers des zones isolÃ©es en mode survie

---

## 7. Direction de franchissement

### 7.1 EntrÃ©e (Inbound)

**DÃ©finition :** Flux autorisÃ© uniquement de l'extÃ©rieur vers l'intÃ©rieur (par rapport Ã  la zone de confiance supÃ©rieure).

| Aspect | SpÃ©cification |
|--------|---------------|
| **Flux autorisÃ©** | Source â†’ Destination uniquement |
| **Usage typique** | FrontiÃ¨res externes, rÃ©ception de donnÃ©es |
| **ContrÃ´le** | Sur ce qui entre |

### 7.2 Sortie (Outbound)

**DÃ©finition :** Flux autorisÃ© uniquement de l'intÃ©rieur vers l'extÃ©rieur.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Flux autorisÃ©** | Destination â†’ Source uniquement |
| **Usage typique** | Envoi de donnÃ©es vers l'extÃ©rieur |
| **ContrÃ´le** | Sur ce qui sort |

### 7.3 Bidirectionnel

**DÃ©finition :** Flux autorisÃ© dans les deux sens, chaque direction pouvant avoir ses propres rÃ¨gles.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Flux autorisÃ©** | Source â†” Destination |
| **Usage typique** | IntÃ©grations, frontiÃ¨res internes |
| **ContrÃ´le** | RÃ¨gles distinctes par direction |

---

## 8. Zones de confiance

### 8.1 DÃ©finition

Une **zone de confiance** est un espace conceptuel dÃ©limitÃ© par des frontiÃ¨res, oÃ¹ tous les Ã©lÃ©ments partagent un mÃªme niveau de confiance.

### 8.2 PropriÃ©tÃ©s d'une zone

| PropriÃ©tÃ© | Description |
|-----------|-------------|
| **Identifiant** | Identifiant unique de la zone |
| **Niveau de confiance** | Niveau de confiance homogÃ¨ne (trusted, verified, unknown, hostile) |
| **FrontiÃ¨res** | Liste des frontiÃ¨res dÃ©limitant la zone |
| **Contenu** | Composants, donnÃ©es, services contenus dans la zone |

### 8.3 HiÃ©rarchie des zones

Les zones de confiance sont organisÃ©es hiÃ©rarchiquement :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ ZONE EXTERNE (hostile/unknown)                              â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚   â”‚ ZONE PÃ‰RIPHÃ‰RIQUE (unknown/verified)                â”‚   â”‚
â”‚   â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚   â”‚
â”‚   â”‚   â”‚ ZONE UTILISATEUR (verified)                 â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”‚ ZONE ADMIN (verified+)              â”‚   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”‚   â”‚ ZONE SYSTÃˆME (trusted)      â”‚   â”‚   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”‚   â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚   â”‚   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”‚   â”‚   â”‚ ZONE CRITIQUE       â”‚   â”‚   â”‚   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”‚   â”‚   â”‚ (trusted isolÃ©)     â”‚   â”‚   â”‚   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”‚   â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚   â”‚   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚   â”‚   â”‚   â”‚
â”‚   â”‚   â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚   â”‚   â”‚
â”‚   â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚   â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**RÃ¨gle fondamentale :** Chaque frontiÃ¨re sÃ©pare exactement deux zones de niveaux de confiance diffÃ©rents.

---

## 9. Adaptation selon les niveaux de sÃ©curitÃ©

Les frontiÃ¨res s'adaptent selon le niveau de sÃ©curitÃ© dÃ©clarÃ© par l'OpÃ©rateur.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md)

### 9.1 Adaptation par niveau

| Niveau | PermÃ©abilitÃ© par dÃ©faut | FrontiÃ¨res internes | FrontiÃ¨res externes |
|--------|-------------------------|---------------------|---------------------|
| **0 - PUBLIC** | Ouverte | Minimales | Assouplies |
| **1 - STANDARD** | ContrÃ´lÃ©e | Standard | Standard |
| **2 - SENSITIVE** | ContrÃ´lÃ©e renforcÃ©e | RenforcÃ©es | RenforcÃ©es |
| **3 - CRITICAL** | Strictement contrÃ´lÃ©e | Strictes | Strictes |
| **4 - HARDENED** | FermÃ©e par dÃ©faut | Maximales, isolement | Maximales, isolement |

### 9.2 RÃ¨gles d'adaptation

| RÃ¨gle | Description |
|-------|-------------|
| **RÃˆGLE-ADAPT-1** | Le niveau de sÃ©curitÃ© influence la permÃ©abilitÃ© par dÃ©faut des nouvelles frontiÃ¨res |
| **RÃˆGLE-ADAPT-2** | Une frontiÃ¨re peut Ãªtre plus restrictive que le niveau, jamais moins |
| **RÃˆGLE-ADAPT-3** | L'Ã©lÃ©vation du niveau resserre automatiquement les frontiÃ¨res existantes |
| **RÃˆGLE-ADAPT-4** | La rÃ©duction du niveau ne desserre pas automatiquement les frontiÃ¨res |

---

## 10. RÃ¨gles de dÃ©finition

### 10.1 RÃ¨gles obligatoires

| RÃ¨gle | Description |
|-------|-------------|
| **RÃˆGLE-DEF-1** | Toute frontiÃ¨re doit Ãªtre explicitement dÃ©finie (INV-BG-5) |
| **RÃˆGLE-DEF-2** | Toute frontiÃ¨re doit avoir une identitÃ© unique et stable |
| **RÃˆGLE-DEF-3** | Toute frontiÃ¨re doit sÃ©parer exactement deux zones de confiance |
| **RÃˆGLE-DEF-4** | Toute frontiÃ¨re doit avoir au moins une rÃ¨gle de franchissement associÃ©e |
| **RÃˆGLE-DEF-5** | Toute dÃ©finition de frontiÃ¨re est traÃ§able (INV-BG-8) |

### 10.2 Anti-patterns de dÃ©finition

| Anti-pattern | Description | Pourquoi c'est interdit |
|--------------|-------------|-------------------------|
| **FrontiÃ¨re implicite** | FrontiÃ¨re non dÃ©clarÃ©e formellement | Viole INV-BG-5 |
| **FrontiÃ¨re flottante** | FrontiÃ¨re sans zones clairement dÃ©finies | Viole RÃˆGLE-DEF-3 |
| **FrontiÃ¨re sans rÃ¨gles** | FrontiÃ¨re sans rÃ¨gles de franchissement | Viole RÃˆGLE-DEF-4 |
| **FrontiÃ¨re technique** | FrontiÃ¨re dÃ©finie par l'implÃ©mentation | Viole INV-BG-10 |

---

## 11. Interactions avec les autres cores

### 11.1 Flux vers StrongFather

Border Guard fournit Ã  StrongFather le **contexte de frontiÃ¨re** pour ses dÃ©cisions :

- Quelles frontiÃ¨res sont traversÃ©es par une intention
- Quelle est la zone source de l'intention
- Quelle est la zone destination de l'intention
- Quel niveau de confiance est associÃ© aux zones

### 11.2 Flux vers BondingBrother

Border Guard fournit Ã  BondingBrother les **dÃ©finitions de frontiÃ¨res** :

- Type, direction, permÃ©abilitÃ© de chaque frontiÃ¨re
- RÃ¨gles de franchissement applicables
- Ã‰tat actuel des frontiÃ¨res (via CaringNanny)

### 11.3 Flux vers CaringNanny

Border Guard informe CaringNanny de l'**Ã©tat des frontiÃ¨res** :

- CrÃ©ation, modification, suppression de frontiÃ¨res
- Changement d'Ã©tat d'une frontiÃ¨re (ex: passage en mode fermÃ©)
- Anomalies dÃ©tectÃ©es sur une frontiÃ¨re

---

## 12. RÃ©fÃ©rences croisÃ©es

### Invariants associÃ©s (Documentation Fondatrice - Section 7)

| Invariant | Ã‰noncÃ© | Relation |
|-----------|--------|----------|
| INV-BG-5 | FrontiÃ¨res explicites | Fondement de ce contrat |
| INV-BG-8 | TraÃ§abilitÃ© complÃ¨te | Toute frontiÃ¨re est traÃ§able |
| INV-BG-9 | CohÃ©rence globale | Pas de contradiction entre frontiÃ¨res |
| INV-BG-10 | NeutralitÃ© conceptuelle | Pas de supposition technique |

### Documents associÃ©s

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Border Guard - Trust Level Classification Contract](./Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Niveaux de confiance des zones |
| [Border Guard - Crossing Rules Contract](./Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | RÃ¨gles de franchissement |
| [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) | Adaptation selon niveau sÃ©curitÃ© |

### RÃ©fÃ©rences glossaire

| Terme | DÃ©finition |
|-------|------------|
| **FrontiÃ¨re** | DÃ©marcation conceptuelle entre deux zones de confiance diffÃ©rentes |
| **Zone de confiance** | Espace conceptuel oÃ¹ tous les Ã©lÃ©ments partagent un niveau de confiance homogÃ¨ne |
| **PermÃ©abilitÃ©** | Propension d'une frontiÃ¨re Ã  autoriser le franchissement |
| **FrontiÃ¨re externe** | SÃ©pare l'Ã©cosystÃ¨me du monde extÃ©rieur |
| **FrontiÃ¨re interne** | SÃ©pare diffÃ©rentes zones au sein de l'Ã©cosystÃ¨me |
| **FrontiÃ¨re d'intÃ©gration** | SÃ©pare l'Ã©cosystÃ¨me d'un systÃ¨me externe intÃ©grÃ© |

**Source :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 13. SynthÃ¨se contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Les frontiÃ¨res sont dÃ©finies** â€” Trois types canoniques avec propriÃ©tÃ©s explicites
2. **Les zones sont claires** â€” Chaque frontiÃ¨re sÃ©pare exactement deux zones
3. **La permÃ©abilitÃ© est classifiÃ©e** â€” Trois niveaux (ouverte, contrÃ´lÃ©e, fermÃ©e)
4. **L'adaptation est automatique** â€” Les frontiÃ¨res s'adaptent au niveau de sÃ©curitÃ©
5. **La traÃ§abilitÃ© est complÃ¨te** â€” Toute frontiÃ¨re est documentÃ©e et traÃ§able

### Phrase de synthÃ¨se

> **Une frontiÃ¨re est une dÃ©marcation conceptuelle, explicite et traÃ§able, qui sÃ©pare deux zones de confiance diffÃ©rentes et dont la permÃ©abilitÃ© s'adapte au niveau de sÃ©curitÃ© du systÃ¨me.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat â€” Normatif  
**RÃ©fÃ©rence :** Border Guard v1.5, Documentation Fondatrice Section 4  
**Type :** Contrat de dÃ©finition de frontiÃ¨res

