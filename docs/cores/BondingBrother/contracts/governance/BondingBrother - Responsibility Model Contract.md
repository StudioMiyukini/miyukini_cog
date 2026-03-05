# BondingBrother - Responsibility Model Contract

## 1. Contexte

Ce document dÃ©finit le modÃ¨le de responsabilitÃ© de Bonding Brother. Il spÃ©cifie qui est responsable de quoi dans le cycle de vie d'une intention, comment les responsabilitÃ©s sont attribuÃ©es, et comment les erreurs et les dÃ©cisions sont imputÃ©es aux bons acteurs.

Ce document complÃ¨te la Section 9 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) sur la traÃ§abilitÃ© et la responsabilitÃ©, et s'appuie sur le [Audit & Traceability Contract](./BondingBrother%20-%20Audit%20&%20Traceability%20Contract.md) pour dÃ©finir l'attribution formelle des responsabilitÃ©s.

Le modÃ¨le de responsabilitÃ© garantit **LOI-3** (Ã©tat local souverain) : les responsabilitÃ©s sont traÃ§ables localement mÃªme en mode offline, conformÃ©ment aux [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md).

## 2. PortÃ©e / Scope

Ce document couvre :
- La dÃ©finition formelle de la responsabilitÃ©
- L'attribution des responsabilitÃ©s par acteur
- Les responsabilitÃ©s de Bonding Brother
- Les responsabilitÃ©s des produits
- Les responsabilitÃ©s des autoritÃ©s
- L'imputation des erreurs
- Les limites de responsabilitÃ©

Ce document **ne couvre pas** :
- Les dÃ©tails d'implÃ©mentation de l'attribution
- Les mÃ©canismes lÃ©gaux de responsabilitÃ©
- Les garanties contractuelles commerciales

---

## 3. Principe fondamental

**Chaque acteur est responsable de ses propres actions et dÃ©cisions. Bonding Brother est responsable de la mÃ©diation, pas des dÃ©cisions des autoritÃ©s ni des intentions des produits.**

La responsabilitÃ© est claire, attribuable, et traÃ§able. Chaque erreur, chaque dÃ©cision, chaque rÃ©sultat peut Ãªtre imputÃ© Ã  l'acteur responsable.

---

## 4. DÃ©finitions

### 4.1 ResponsabilitÃ©

La **responsabilitÃ©** est l'obligation d'un acteur de rÃ©pondre de ses actions, dÃ©cisions, et rÃ©sultats dans le cadre de ses attributions.

### 4.2 Imputation

L'**imputation** est l'attribution formelle d'une responsabilitÃ© Ã  un acteur spÃ©cifique pour un Ã©vÃ©nement ou un rÃ©sultat donnÃ©.

### 4.3 Domaine de responsabilitÃ©

Un **domaine de responsabilitÃ©** est un pÃ©rimÃ¨tre d'actions et de dÃ©cisions pour lequel un acteur est responsable.

---

## 5. Acteurs et leurs responsabilitÃ©s

### 5.1 Produit

**Domaine de responsabilitÃ© :**
- Expression des intentions
- Format et structure des intentions
- Contexte fourni avec les intentions
- Gestion des rÃ©sultats reÃ§us

**ResponsabilitÃ©s spÃ©cifiques :**

| ResponsabilitÃ© | Description | Imputation |
|----------------|------------|------------|
| **RES-PROD-01** | Expression d'intentions valides structurellement | Produit responsable si intention invalide |
| **RES-PROD-02** | Fourniture d'un contexte complet et vÃ©ridique | Produit responsable si contexte falsifiÃ© |
| **RES-PROD-03** | Respect du vocabulaire et des formats | Produit responsable si format incorrect |
| **RES-PROD-04** | Gestion des rÃ©sultats reÃ§us | Produit responsable de l'utilisation des rÃ©sultats |

**Ce que le produit n'est PAS responsable :**
- DÃ©cisions des autoritÃ©s
- Traduction des intentions
- Filtrage des rÃ©sultats
- Erreurs de transmission

### 5.2 Bonding Brother

**Domaine de responsabilitÃ© :**
- Validation structurelle des intentions
- Traduction des intentions en demandes
- Traduction des rÃ©ponses en rÃ©sultats
- Filtrage des demandes et rÃ©sultats
- Transmission aux autoritÃ©s
- Journalisation
- Synchronisation

**ResponsabilitÃ©s spÃ©cifiques :**

| ResponsabilitÃ© | Description | Imputation |
|----------------|------------|------------|
| **RES-BB-01** | Validation structurelle correcte | BB responsable si validation incorrecte |
| **RES-BB-02** | Traduction fidÃ¨le (sÃ©mantique prÃ©servÃ©e) | BB responsable si traduction incorrecte |
| **RES-BB-03** | Filtrage selon rÃ¨gles dÃ©finies | BB responsable si filtrage incorrect |
| **RES-BB-04** | Transmission fidÃ¨le aux autoritÃ©s | BB responsable si transmission incorrecte |
| **RES-BB-05** | Journalisation complÃ¨te | BB responsable si journalisation incomplÃ¨te |
| **RES-BB-06** | Synchronisation sans perte | BB responsable si intention perdue en sync |
| **RES-BB-07** | PrÃ©servation de l'ordre | BB responsable si ordre non prÃ©servÃ© |

**Ce que Bonding Brother n'est PAS responsable :**
- DÃ©cisions des autoritÃ©s (refus, acceptation)
- ValiditÃ© mÃ©tier des intentions
- Permissions rÃ©elles des utilisateurs
- CohÃ©rence des donnÃ©es
- Erreurs des autoritÃ©s

### 5.3 AutoritÃ© (Kind Mother / Strong Father)

**Domaine de responsabilitÃ© :**
- Ã‰valuation des demandes
- DÃ©cisions d'acceptation ou de refus
- Validation mÃ©tier
- CohÃ©rence et intÃ©gritÃ©
- Gestion des erreurs internes

**ResponsabilitÃ©s spÃ©cifiques :**

| ResponsabilitÃ© | Description | Imputation |
|----------------|------------|------------|
| **RES-AUTH-01** | DÃ©cision correcte selon rÃ¨gles | AutoritÃ© responsable si dÃ©cision incorrecte |
| **RES-AUTH-02** | Validation mÃ©tier appropriÃ©e | AutoritÃ© responsable si validation incorrecte |
| **RES-AUTH-03** | CohÃ©rence et intÃ©gritÃ© | AutoritÃ© responsable si incohÃ©rence |
| **RES-AUTH-04** | RÃ©ponse dans dÃ©lai raisonnable | AutoritÃ© responsable si timeout |
| **RES-AUTH-05** | Gestion des erreurs internes | AutoritÃ© responsable si erreur interne |

**Ce que l'autoritÃ© n'est PAS responsable :**
- Format des intentions (responsabilitÃ© du produit)
- Traduction des intentions (responsabilitÃ© de BB)
- Filtrage des rÃ©sultats (responsabilitÃ© de BB)

---

## 6. Imputation des erreurs

### 6.1 Erreurs de validation

**Erreur :** Intention rejetÃ©e pour format invalide

**Imputation :** Bonding Brother (RES-BB-01)

**Justification :** BB est responsable de la validation structurelle. Si une intention valide est rejetÃ©e, c'est la responsabilitÃ© de BB.

**Exception :** Si l'intention est effectivement invalide selon le schÃ©ma, le produit est responsable (RES-PROD-01).

### 6.2 Erreurs de traduction

**Erreur :** Traduction incorrecte (sÃ©mantique perdue ou altÃ©rÃ©e)

**Imputation :** Bonding Brother (RES-BB-02)

**Justification :** BB est responsable de la traduction fidÃ¨le. Toute perte ou altÃ©ration de sÃ©mantique est la responsabilitÃ© de BB.

### 6.3 Erreurs de filtrage

**Erreur :** Filtrage incorrect (information filtrÃ©e Ã  tort ou non filtrÃ©e)

**Imputation :** Bonding Brother (RES-BB-03)

**Justification :** BB est responsable de l'application correcte des rÃ¨gles de filtrage.

### 6.4 Erreurs d'autoritÃ©

**Erreur :** DÃ©cision incorrecte de l'autoritÃ©

**Imputation :** AutoritÃ© (RES-AUTH-01)

**Justification :** L'autoritÃ© est responsable de ses dÃ©cisions. BB transmet fidÃ¨lement, mais ne dÃ©cide pas.

### 6.5 Erreurs de transmission

**Erreur :** Transmission Ã©chouÃ©e ou incorrecte

**Imputation :** Bonding Brother (RES-BB-04)

**Justification :** BB est responsable de la transmission fidÃ¨le aux autoritÃ©s.

**Exception :** Si l'erreur est due Ã  une indisponibilitÃ© de l'autoritÃ©, l'autoritÃ© est responsable (RES-AUTH-04).

### 6.6 Erreurs de synchronisation

**Erreur :** Intention perdue lors de la synchronisation

**Imputation :** Bonding Brother (RES-BB-06)

**Justification :** BB est responsable de la synchronisation sans perte.

### 6.7 Erreurs de journalisation

**Erreur :** Journalisation incomplÃ¨te ou incorrecte

**Imputation :** Bonding Brother (RES-BB-05)

**Justification :** BB est responsable de la journalisation complÃ¨te.

---

## 7. Limites de responsabilitÃ©

### 7.1 ResponsabilitÃ© limitÃ©e Ã  la mÃ©diation

**RÃ¨gle LIM-01 : Pas de responsabilitÃ© sur les dÃ©cisions**

Bonding Brother n'est pas responsable des dÃ©cisions des autoritÃ©s :
- Refus d'une intention : ResponsabilitÃ© de l'autoritÃ©
- Acceptation d'une intention : ResponsabilitÃ© de l'autoritÃ©
- Validation mÃ©tier : ResponsabilitÃ© de l'autoritÃ©

**RÃ¨gle LIM-02 : Pas de responsabilitÃ© sur les intentions**

Bonding Brother n'est pas responsable du contenu des intentions :
- ValiditÃ© mÃ©tier : ResponsabilitÃ© du produit
- ConformitÃ© aux rÃ¨gles : ResponsabilitÃ© du produit
- CohÃ©rence sÃ©mantique : ResponsabilitÃ© du produit

**RÃ¨gle LIM-03 : Pas de responsabilitÃ© sur les rÃ©sultats**

Bonding Brother n'est pas responsable de l'utilisation des rÃ©sultats par les produits :
- InterprÃ©tation des rÃ©sultats : ResponsabilitÃ© du produit
- Actions basÃ©es sur les rÃ©sultats : ResponsabilitÃ© du produit

### 7.2 ResponsabilitÃ© limitÃ©e aux rÃ¨gles dÃ©finies

**RÃ¨gle LIM-04 : Application des rÃ¨gles, pas leur dÃ©finition**

Bonding Brother applique les rÃ¨gles dÃ©finies par les autoritÃ©s, mais n'est pas responsable de leur dÃ©finition :
- RÃ¨gles de filtrage : DÃ©finies par autoritÃ©, appliquÃ©es par BB
- RÃ¨gles de traduction : DÃ©finies par architecture, appliquÃ©es par BB

**RÃ¨gle LIM-05 : Pas de responsabilitÃ© sur les rÃ¨gles incorrectes**

Si une rÃ¨gle est incorrecte, la responsabilitÃ© est de l'autoritÃ© qui l'a dÃ©finie, pas de BB qui l'applique.

### 7.3 ResponsabilitÃ© limitÃ©e aux capacitÃ©s

**RÃ¨gle LIM-06 : ResponsabilitÃ© dans les limites des capacitÃ©s**

Bonding Brother est responsable uniquement dans les limites de ses capacitÃ©s :
- Transmission : Responsable si erreur de transmission, pas si autoritÃ© indisponible
- Traduction : Responsable si erreur de traduction, pas si mapping manquant (configuration)
- Filtrage : Responsable si erreur d'application, pas si rÃ¨gle manquante (configuration)

---

## 8. TraÃ§abilitÃ© de la responsabilitÃ©

### 8.1 Attribution dans les traces

**RÃ¨gle TRACE-01 : Acteur responsable tracÃ©**

Chaque trace d'audit inclut l'acteur responsable :
- `acteur_responsable` : Acteur (PRODUIT, BONDING_BROTHER, AUTORITÃ‰)
- `responsabilitÃ©_code` : Code de responsabilitÃ© (ex: RES-BB-01)

**RÃ¨gle TRACE-02 : Imputation tracÃ©e**

L'imputation d'une erreur est tracÃ©e :
- `erreur_id` : ID de l'erreur
- `acteur_imputÃ©` : Acteur responsable
- `justification` : Justification de l'imputation

### 8.2 Consultation de la responsabilitÃ©

**RÃ¨gle CONSULT-01 : API de consultation**

Un produit peut consulter la responsabilitÃ© de ses propres interactions :
- `GET /responsibility/intentions/{intention_id}` : ResponsabilitÃ©s pour une intention
- `GET /responsibility/erreurs/{erreur_id}` : Imputation d'une erreur

**RÃ¨gle CONSULT-02 : Rapport de responsabilitÃ©**

Les administrateurs peuvent gÃ©nÃ©rer des rapports de responsabilitÃ© :
- Par acteur
- Par type d'erreur
- Par pÃ©riode

---

## 9. Exemples

### 9.1 Intention rejetÃ©e par validation

**ScÃ©nario :** Intention avec format invalide rejetÃ©e par BB.

**Imputation :**
- Si intention effectivement invalide : **PRODUIT** (RES-PROD-01)
- Si intention valide mais rejetÃ©e Ã  tort : **BONDING_BROTHER** (RES-BB-01)

**Trace :**
```json
{
  "trace_id": "trace-001",
  "type_Ã©vÃ©nement": "INTENTION_REJETÃ‰E",
  "acteur_responsable": "PRODUIT",
  "responsabilitÃ©_code": "RES-PROD-01",
  "justification": "Intention invalide : champ 'payload' manquant"
}
```

### 9.2 Intention refusÃ©e par autoritÃ©

**ScÃ©nario :** Intention valide et traduite, mais refusÃ©e par Strong Father pour permissions insuffisantes.

**Imputation :**
- Validation : **BONDING_BROTHER** (correcte)
- Traduction : **BONDING_BROTHER** (correcte)
- DÃ©cision de refus : **STRONG_FATHER** (RES-AUTH-01)

**Trace :**
```json
{
  "trace_id": "trace-005",
  "type_Ã©vÃ©nement": "RÃ‰PONSE_REÃ‡UE",
  "acteur_responsable": "STRONG_FATHER",
  "responsabilitÃ©_code": "RES-AUTH-01",
  "donnÃ©es_Ã©vÃ©nement": {
    "rÃ©ponse": {
      "statut": "REFUSÃ‰",
      "raison": "Permissions insuffisantes"
    }
  }
}
```

### 9.3 Erreur de traduction

**ScÃ©nario :** Intention valide mais traduction incorrecte (sÃ©mantique perdue).

**Imputation :** **BONDING_BROTHER** (RES-BB-02)

**Trace :**
```json
{
  "trace_id": "trace-003",
  "type_Ã©vÃ©nement": "ERREUR_SURVENUE",
  "acteur_responsable": "BONDING_BROTHER",
  "responsabilitÃ©_code": "RES-BB-02",
  "donnÃ©es_Ã©vÃ©nement": {
    "erreur": {
      "code": "TRAD-002",
      "message": "Champ non mappable lors de la traduction"
    }
  }
}
```

### 9.4 Intention perdue en synchronisation

**ScÃ©nario :** Intention buffÃ©e en offline, mais perdue lors de la synchronisation.

**Imputation :** **BONDING_BROTHER** (RES-BB-06)

**Trace :**
```json
{
  "trace_id": "trace-sync-001",
  "type_Ã©vÃ©nement": "ERREUR_SURVENUE",
  "acteur_responsable": "BONDING_BROTHER",
  "responsabilitÃ©_code": "RES-BB-06",
  "donnÃ©es_Ã©vÃ©nement": {
    "erreur": {
      "code": "SYNC-005",
      "message": "Intention perdue lors de la synchronisation"
    }
  }
}
```

---

## 10. Matrice de responsabilitÃ©

Cette matrice rÃ©sume les responsabilitÃ©s par type d'Ã©vÃ©nement :

| Ã‰vÃ©nement | Acteur responsable | Code |
|-----------|-------------------|------|
| Intention invalide | Produit | RES-PROD-01 |
| Validation incorrecte | Bonding Brother | RES-BB-01 |
| Traduction incorrecte | Bonding Brother | RES-BB-02 |
| Filtrage incorrect | Bonding Brother | RES-BB-03 |
| Transmission incorrecte | Bonding Brother | RES-BB-04 |
| Journalisation incomplÃ¨te | Bonding Brother | RES-BB-05 |
| Perte en synchronisation | Bonding Brother | RES-BB-06 |
| Ordre non prÃ©servÃ© | Bonding Brother | RES-BB-07 |
| DÃ©cision incorrecte | AutoritÃ© | RES-AUTH-01 |
| Validation mÃ©tier incorrecte | AutoritÃ© | RES-AUTH-02 |
| IncohÃ©rence | AutoritÃ© | RES-AUTH-03 |
| Timeout autoritÃ© | AutoritÃ© | RES-AUTH-04 |
| Erreur interne autoritÃ© | AutoritÃ© | RES-AUTH-05 |

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit le modÃ¨le de responsabilitÃ© que Bonding Brother doit respecter pour garantir l'attribution claire des responsabilitÃ©s.

Toute erreur, toute dÃ©cision, tout rÃ©sultat doit pouvoir Ãªtre imputÃ© selon ce modÃ¨le. Toute dÃ©viation est considÃ©rÃ©e comme une violation.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** 
- [Documentation Fondatrice v1.0](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) (Section 9)
- [Audit & Traceability Contract v1.0](./BondingBrother%20-%20Audit%20&%20Traceability%20Contract.md)
- [Intent Model Contract v1.0](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md)
- [Error & Rejection Model v1.0](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md)
- [Invariants & Guarantees v1.0](./BondingBrother%20-%20Invariants%20&%20Guarantees.md)

