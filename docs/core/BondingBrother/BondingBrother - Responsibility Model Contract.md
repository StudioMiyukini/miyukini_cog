# BondingBrother - Responsibility Model Contract

## 1. Contexte

Ce document définit le modèle de responsabilité de Bonding Brother. Il spécifie qui est responsable de quoi dans le cycle de vie d'une intention, comment les responsabilités sont attribuées, et comment les erreurs et les décisions sont imputées aux bons acteurs.

Ce document complète la Section 9 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) sur la traçabilité et la responsabilité, et s'appuie sur l'Audit & Traceability Contract pour définir l'attribution formelle des responsabilités.

Le modèle de responsabilité garantit **LOI-3** (état local souverain) : les responsabilités sont traçables localement même en mode offline, conformément aux [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

## 2. Portée / Scope

Ce document couvre :
- La définition formelle de la responsabilité
- L'attribution des responsabilités par acteur
- Les responsabilités de Bonding Brother
- Les responsabilités des produits
- Les responsabilités des autorités
- L'imputation des erreurs
- Les limites de responsabilité

Ce document **ne couvre pas** :
- Les détails d'implémentation de l'attribution
- Les mécanismes légaux de responsabilité
- Les garanties contractuelles commerciales

---

## 3. Principe fondamental

**Chaque acteur est responsable de ses propres actions et décisions. Bonding Brother est responsable de la médiation, pas des décisions des autorités ni des intentions des produits.**

La responsabilité est claire, attribuable, et traçable. Chaque erreur, chaque décision, chaque résultat peut être imputé à l'acteur responsable.

---

## 4. Définitions

### 4.1 Responsabilité

La **responsabilité** est l'obligation d'un acteur de répondre de ses actions, décisions, et résultats dans le cadre de ses attributions.

### 4.2 Imputation

L'**imputation** est l'attribution formelle d'une responsabilité à un acteur spécifique pour un événement ou un résultat donné.

### 4.3 Domaine de responsabilité

Un **domaine de responsabilité** est un périmètre d'actions et de décisions pour lequel un acteur est responsable.

---

## 5. Acteurs et leurs responsabilités

### 5.1 Produit

**Domaine de responsabilité :**
- Expression des intentions
- Format et structure des intentions
- Contexte fourni avec les intentions
- Gestion des résultats reçus

**Responsabilités spécifiques :**

| Responsabilité | Description | Imputation |
|----------------|------------|------------|
| **RES-PROD-01** | Expression d'intentions valides structurellement | Produit responsable si intention invalide |
| **RES-PROD-02** | Fourniture d'un contexte complet et véridique | Produit responsable si contexte falsifié |
| **RES-PROD-03** | Respect du vocabulaire et des formats | Produit responsable si format incorrect |
| **RES-PROD-04** | Gestion des résultats reçus | Produit responsable de l'utilisation des résultats |

**Ce que le produit n'est PAS responsable :**
- Décisions des autorités
- Traduction des intentions
- Filtrage des résultats
- Erreurs de transmission

### 5.2 Bonding Brother

**Domaine de responsabilité :**
- Validation structurelle des intentions
- Traduction des intentions en demandes
- Traduction des réponses en résultats
- Filtrage des demandes et résultats
- Transmission aux autorités
- Journalisation
- Synchronisation

**Responsabilités spécifiques :**

| Responsabilité | Description | Imputation |
|----------------|------------|------------|
| **RES-BB-01** | Validation structurelle correcte | BB responsable si validation incorrecte |
| **RES-BB-02** | Traduction fidèle (sémantique préservée) | BB responsable si traduction incorrecte |
| **RES-BB-03** | Filtrage selon règles définies | BB responsable si filtrage incorrect |
| **RES-BB-04** | Transmission fidèle aux autorités | BB responsable si transmission incorrecte |
| **RES-BB-05** | Journalisation complète | BB responsable si journalisation incomplète |
| **RES-BB-06** | Synchronisation sans perte | BB responsable si intention perdue en sync |
| **RES-BB-07** | Préservation de l'ordre | BB responsable si ordre non préservé |

**Ce que Bonding Brother n'est PAS responsable :**
- Décisions des autorités (refus, acceptation)
- Validité métier des intentions
- Permissions réelles des utilisateurs
- Cohérence des données
- Erreurs des autorités

### 5.3 Autorité (Kind Mother / Strong Father)

**Domaine de responsabilité :**
- Évaluation des demandes
- Décisions d'acceptation ou de refus
- Validation métier
- Cohérence et intégrité
- Gestion des erreurs internes

**Responsabilités spécifiques :**

| Responsabilité | Description | Imputation |
|----------------|------------|------------|
| **RES-AUTH-01** | Décision correcte selon règles | Autorité responsable si décision incorrecte |
| **RES-AUTH-02** | Validation métier appropriée | Autorité responsable si validation incorrecte |
| **RES-AUTH-03** | Cohérence et intégrité | Autorité responsable si incohérence |
| **RES-AUTH-04** | Réponse dans délai raisonnable | Autorité responsable si timeout |
| **RES-AUTH-05** | Gestion des erreurs internes | Autorité responsable si erreur interne |

**Ce que l'autorité n'est PAS responsable :**
- Format des intentions (responsabilité du produit)
- Traduction des intentions (responsabilité de BB)
- Filtrage des résultats (responsabilité de BB)

---

## 6. Imputation des erreurs

### 6.1 Erreurs de validation

**Erreur :** Intention rejetée pour format invalide

**Imputation :** Bonding Brother (RES-BB-01)

**Justification :** BB est responsable de la validation structurelle. Si une intention valide est rejetée, c'est la responsabilité de BB.

**Exception :** Si l'intention est effectivement invalide selon le schéma, le produit est responsable (RES-PROD-01).

### 6.2 Erreurs de traduction

**Erreur :** Traduction incorrecte (sémantique perdue ou altérée)

**Imputation :** Bonding Brother (RES-BB-02)

**Justification :** BB est responsable de la traduction fidèle. Toute perte ou altération de sémantique est la responsabilité de BB.

### 6.3 Erreurs de filtrage

**Erreur :** Filtrage incorrect (information filtrée à tort ou non filtrée)

**Imputation :** Bonding Brother (RES-BB-03)

**Justification :** BB est responsable de l'application correcte des règles de filtrage.

### 6.4 Erreurs d'autorité

**Erreur :** Décision incorrecte de l'autorité

**Imputation :** Autorité (RES-AUTH-01)

**Justification :** L'autorité est responsable de ses décisions. BB transmet fidèlement, mais ne décide pas.

### 6.5 Erreurs de transmission

**Erreur :** Transmission échouée ou incorrecte

**Imputation :** Bonding Brother (RES-BB-04)

**Justification :** BB est responsable de la transmission fidèle aux autorités.

**Exception :** Si l'erreur est due à une indisponibilité de l'autorité, l'autorité est responsable (RES-AUTH-04).

### 6.6 Erreurs de synchronisation

**Erreur :** Intention perdue lors de la synchronisation

**Imputation :** Bonding Brother (RES-BB-06)

**Justification :** BB est responsable de la synchronisation sans perte.

### 6.7 Erreurs de journalisation

**Erreur :** Journalisation incomplète ou incorrecte

**Imputation :** Bonding Brother (RES-BB-05)

**Justification :** BB est responsable de la journalisation complète.

---

## 7. Limites de responsabilité

### 7.1 Responsabilité limitée à la médiation

**Règle LIM-01 : Pas de responsabilité sur les décisions**

Bonding Brother n'est pas responsable des décisions des autorités :
- Refus d'une intention : Responsabilité de l'autorité
- Acceptation d'une intention : Responsabilité de l'autorité
- Validation métier : Responsabilité de l'autorité

**Règle LIM-02 : Pas de responsabilité sur les intentions**

Bonding Brother n'est pas responsable du contenu des intentions :
- Validité métier : Responsabilité du produit
- Conformité aux règles : Responsabilité du produit
- Cohérence sémantique : Responsabilité du produit

**Règle LIM-03 : Pas de responsabilité sur les résultats**

Bonding Brother n'est pas responsable de l'utilisation des résultats par les produits :
- Interprétation des résultats : Responsabilité du produit
- Actions basées sur les résultats : Responsabilité du produit

### 7.2 Responsabilité limitée aux règles définies

**Règle LIM-04 : Application des règles, pas leur définition**

Bonding Brother applique les règles définies par les autorités, mais n'est pas responsable de leur définition :
- Règles de filtrage : Définies par autorité, appliquées par BB
- Règles de traduction : Définies par architecture, appliquées par BB

**Règle LIM-05 : Pas de responsabilité sur les règles incorrectes**

Si une règle est incorrecte, la responsabilité est de l'autorité qui l'a définie, pas de BB qui l'applique.

### 7.3 Responsabilité limitée aux capacités

**Règle LIM-06 : Responsabilité dans les limites des capacités**

Bonding Brother est responsable uniquement dans les limites de ses capacités :
- Transmission : Responsable si erreur de transmission, pas si autorité indisponible
- Traduction : Responsable si erreur de traduction, pas si mapping manquant (configuration)
- Filtrage : Responsable si erreur d'application, pas si règle manquante (configuration)

---

## 8. Traçabilité de la responsabilité

### 8.1 Attribution dans les traces

**Règle TRACE-01 : Acteur responsable tracé**

Chaque trace d'audit inclut l'acteur responsable :
- `acteur_responsable` : Acteur (PRODUIT, BONDING_BROTHER, AUTORITÉ)
- `responsabilité_code` : Code de responsabilité (ex: RES-BB-01)

**Règle TRACE-02 : Imputation tracée**

L'imputation d'une erreur est tracée :
- `erreur_id` : ID de l'erreur
- `acteur_imputé` : Acteur responsable
- `justification` : Justification de l'imputation

### 8.2 Consultation de la responsabilité

**Règle CONSULT-01 : API de consultation**

Un produit peut consulter la responsabilité de ses propres interactions :
- `GET /responsibility/intentions/{intention_id}` : Responsabilités pour une intention
- `GET /responsibility/erreurs/{erreur_id}` : Imputation d'une erreur

**Règle CONSULT-02 : Rapport de responsabilité**

Les administrateurs peuvent générer des rapports de responsabilité :
- Par acteur
- Par type d'erreur
- Par période

---

## 9. Exemples

### 9.1 Intention rejetée par validation

**Scénario :** Intention avec format invalide rejetée par BB.

**Imputation :**
- Si intention effectivement invalide : **PRODUIT** (RES-PROD-01)
- Si intention valide mais rejetée à tort : **BONDING_BROTHER** (RES-BB-01)

**Trace :**
```json
{
  "trace_id": "trace-001",
  "type_événement": "INTENTION_REJETÉE",
  "acteur_responsable": "PRODUIT",
  "responsabilité_code": "RES-PROD-01",
  "justification": "Intention invalide : champ 'payload' manquant"
}
```

### 9.2 Intention refusée par autorité

**Scénario :** Intention valide et traduite, mais refusée par Strong Father pour permissions insuffisantes.

**Imputation :**
- Validation : **BONDING_BROTHER** (correcte)
- Traduction : **BONDING_BROTHER** (correcte)
- Décision de refus : **STRONG_FATHER** (RES-AUTH-01)

**Trace :**
```json
{
  "trace_id": "trace-005",
  "type_événement": "RÉPONSE_REÇUE",
  "acteur_responsable": "STRONG_FATHER",
  "responsabilité_code": "RES-AUTH-01",
  "données_événement": {
    "réponse": {
      "statut": "REFUSÉ",
      "raison": "Permissions insuffisantes"
    }
  }
}
```

### 9.3 Erreur de traduction

**Scénario :** Intention valide mais traduction incorrecte (sémantique perdue).

**Imputation :** **BONDING_BROTHER** (RES-BB-02)

**Trace :**
```json
{
  "trace_id": "trace-003",
  "type_événement": "ERREUR_SURVENUE",
  "acteur_responsable": "BONDING_BROTHER",
  "responsabilité_code": "RES-BB-02",
  "données_événement": {
    "erreur": {
      "code": "TRAD-002",
      "message": "Champ non mappable lors de la traduction"
    }
  }
}
```

### 9.4 Intention perdue en synchronisation

**Scénario :** Intention buffée en offline, mais perdue lors de la synchronisation.

**Imputation :** **BONDING_BROTHER** (RES-BB-06)

**Trace :**
```json
{
  "trace_id": "trace-sync-001",
  "type_événement": "ERREUR_SURVENUE",
  "acteur_responsable": "BONDING_BROTHER",
  "responsabilité_code": "RES-BB-06",
  "données_événement": {
    "erreur": {
      "code": "SYNC-005",
      "message": "Intention perdue lors de la synchronisation"
    }
  }
}
```

---

## 10. Matrice de responsabilité

Cette matrice résume les responsabilités par type d'événement :

| Événement | Acteur responsable | Code |
|-----------|-------------------|------|
| Intention invalide | Produit | RES-PROD-01 |
| Validation incorrecte | Bonding Brother | RES-BB-01 |
| Traduction incorrecte | Bonding Brother | RES-BB-02 |
| Filtrage incorrect | Bonding Brother | RES-BB-03 |
| Transmission incorrecte | Bonding Brother | RES-BB-04 |
| Journalisation incomplète | Bonding Brother | RES-BB-05 |
| Perte en synchronisation | Bonding Brother | RES-BB-06 |
| Ordre non préservé | Bonding Brother | RES-BB-07 |
| Décision incorrecte | Autorité | RES-AUTH-01 |
| Validation métier incorrecte | Autorité | RES-AUTH-02 |
| Incohérence | Autorité | RES-AUTH-03 |
| Timeout autorité | Autorité | RES-AUTH-04 |
| Erreur interne autorité | Autorité | RES-AUTH-05 |

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit le modèle de responsabilité que Bonding Brother doit respecter pour garantir l'attribution claire des responsabilités.

Toute erreur, toute décision, tout résultat doit pouvoir être imputé selon ce modèle. Toute déviation est considérée comme une violation.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 9)
- Audit & Traceability Contract v1.0
- Intent Model Contract v1.0
- Error and Rejection Model v1.0
- Invariants et Garanties v1.0
