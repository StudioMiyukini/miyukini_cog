# StrongFather — Examples Decisions

## Contexte

Ce document illustre StrongFather par des **exemples de décisions** produites par le moteur de décision. Il fait partie de la série de documents d'exemples.

**Documents connexes :**
- [StrongFather - Examples Intentions](./StrongFather%20-%20Examples%20Intentions.md)
- [StrongFather - Examples Policies](./StrongFather%20-%20Examples%20Policies.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 1. Types de décisions

StrongFather produit 4 types de décisions :

| Type | Description | Conséquence |
|------|-------------|-------------|
| **ACCEPTÉE** | Intention valide selon toutes les politiques | L'adaptateur peut procéder |
| **REFUSÉE** | Au moins une politique violée | L'intention ne doit pas être exécutée |
| **AMBIGUË** | Information manquante dans l'intention | Clarifications requises |
| **DIFFÉRÉE** | Contexte futur requis | Réévaluation nécessaire |

---

## 2. Décisions ACCEPTÉE

### 2.1. Décision ACCEPTÉE — Cas standard

**Intention évaluée :**
- Intention intent-2026-01-26-001 (création d'article par user-alice)

**Politiques appliquées :**
- POL-USER-PERM-001 : SATISFAITE (user-alice a le rôle ÉDITEUR)
- POL-VALID-CONT-001 : SATISFAITE (contenu de 1200 caractères)
- POL-LIMIT-PUB-001 : SATISFAITE (3 articles publiés aujourd'hui)

**Décision produite :**
```
Type : ACCEPTÉE
Identifiant intention : intent-2026-01-26-001
Priorité établie : NORMALE
Politiques appliquées :
  - POL-USER-PERM-001 : SATISFAITE
  - POL-VALID-CONT-001 : SATISFAITE
  - POL-LIMIT-PUB-001 : SATISFAITE
Justification :
  L'intention est valide selon toutes les politiques applicables.
  L'utilisateur user-alice possède le rôle ÉDITEUR requis.
  Le contenu respecte la longueur minimale de 500 caractères.
  La limite de publications quotidiennes n'est pas atteinte (3/10).
```

**Caractéristiques :**
- Toutes les politiques satisfaites
- Priorité calculée
- Justification complète
- Décision non exécutable (l'adaptateur décide de l'exécution)

---

### 2.2. Décision ACCEPTÉE — Cas avec priorité élevée

**Intention évaluée :**
- Intention intent-2026-01-26-004 (création d'article SÉCURITÉ par user-admin)

**Décision produite :**
```
Type : ACCEPTÉE
Identifiant intention : intent-2026-01-26-004
Priorité établie : MAXIMALE
Politiques appliquées :
  - POL-USER-PERM-001 : SATISFAITE
  - POL-VALID-CONT-001 : SATISFAITE
  - POL-LIMIT-PUB-001 : SATISFAITE
  - POL-PRIO-CAT-001 : Priorité MAXIMALE
Justification :
  L'intention est valide selon toutes les politiques applicables.
  La priorité MAXIMALE est établie selon la catégorie SÉCURITÉ.
```

---

## 3. Décisions REFUSÉE

### 3.1. Décision REFUSÉE — Permission insuffisante

**Intention évaluée :**
- Intention intent-2026-01-26-001 (création d'article par user-visitor)

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : intent-2026-01-26-001
Politiques violées :
  - POL-USER-PERM-001 : NON_SATISFAITE
Raison explicite :
  L'utilisateur user-visitor ne possède pas le rôle requis (ÉDITEUR ou ADMIN)
  pour créer un article. Le rôle actuel (VISITEUR) ne permet pas cette action.
Justification :
  La politique POL-USER-PERM-001 exige que l'utilisateur possède le rôle
  ÉDITEUR ou ADMIN pour créer un article.
```

**Caractéristiques :**
- Au moins une politique non satisfaite
- Raison explicite du refus
- Politiques violées identifiées
- Décision définitive

---

### 3.2. Décision REFUSÉE — Limite atteinte

**Intention évaluée :**
- Intention de création d'article par user-alice (12 articles déjà publiés aujourd'hui)

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : intent-2026-01-26-001
Politiques violées :
  - POL-LIMIT-PUB-001 : NON_SATISFAITE
Raison explicite :
  La limite de publications quotidiennes est atteinte. L'utilisateur user-alice
  a déjà publié 12 articles aujourd'hui, ce qui dépasse la limite de 10 articles.
Justification :
  La politique POL-LIMIT-PUB-001 limite le nombre de publications quotidiennes
  à 10 articles par utilisateur. Nombre actuel : 12/10.
```

---

### 3.3. Décision REFUSÉE — Contenu insuffisant

**Intention évaluée :**
- Intention de création d'article avec contenu de 250 caractères (minimum 500)

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : intent-2026-01-26-001
Politiques violées :
  - POL-VALID-CONT-001 : NON_SATISFAITE
Raison explicite :
  Le contenu de l'article ne respecte pas la longueur minimale requise.
  Contenu fourni : 250 caractères. Minimum requis : 500 caractères.
```

---

### 3.4. Décision REFUSÉE — Rejet structurel

**Intention évaluée :**
- Intention sans action spécifiée

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : intent-2026-01-26-010
Politiques violées :
  - (Aucune politique évaluée — rejet structurel)
Raison explicite :
  L'intention est structurellement invalide. Le champ "Action" est
  obligatoire et manquant. Une intention doit toujours spécifier
  l'action demandée (CRÉER, MODIFIER, SUPPRIMER, etc.).
Justification :
  L'intention ne respecte pas les règles de formation définies dans
  le Intent Model Contract.
```

---

## 4. Décisions AMBIGUË

### 4.1. Cas AMBIGUË — Information manquante

**Intention évaluée :**
```
Identifiant : intent-2026-01-26-005
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Article sans auteur"
  - Statut demandé : PUBLIÉ
  - Auteur : (manquant)
```

**Décision produite :**
```
Type : AMBIGUË
Identifiant intention : intent-2026-01-26-005
Éléments manquants :
  - Auteur de l'article (champ obligatoire)
Clarifications requises :
  - L'auteur de l'article doit être spécifié dans les données de l'intention.
Politiques nécessitant ces clarifications :
  - POL-USER-PERM-001 : Nécessite l'auteur pour vérifier les permissions
  - POL-LIMIT-PUB-001 : Nécessite l'auteur pour vérifier la limite quotidienne
Justification :
  L'intention est insuffisamment définie. Le champ "Auteur" est manquant,
  ce qui empêche l'évaluation des politiques de permission et de limite.
```

**Caractéristiques :**
- Information manquante identifiée
- Clarifications requises explicites
- Politiques affectées identifiées
- Suspension d'évaluation jusqu'à clarification

---

### 4.2. Cas AMBIGUË — Contexte utilisateur ambigu

**Intention évaluée :**
```
Identifiant : intent-2026-01-26-006
Action : MODIFIER
Type d'entité : ARTICLE
Contexte :
  - Utilisateur : (non spécifié)
```

**Décision produite :**
```
Type : AMBIGUË
Identifiant intention : intent-2026-01-26-006
Éléments manquants :
  - Utilisateur dans le contexte (champ obligatoire)
Clarifications requises :
  - L'utilisateur effectuant la modification doit être spécifié dans le contexte.
```

---

## 5. Décisions DIFFÉRÉE

### 5.1. Cas DIFFÉRÉE — Dépendance à un événement futur

**Intention évaluée :**
```
Identifiant : intent-2026-01-26-008
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Date de publication : 2026-01-27T10:00:00Z (demain)
Contexte :
  - Date actuelle : 2026-01-26T15:00:00Z
```

**Décision produite :**
```
Type : DIFFÉRÉE
Identifiant intention : intent-2026-01-26-008
Contexte futur requis :
  - Date effective de publication (2026-01-27T10:00:00Z) pour évaluer
    la limite quotidienne de publications
Raison du différé :
  L'intention spécifie une date de publication future (2026-01-27).
  La politique POL-LIMIT-PUB-001 nécessite de connaître le nombre
  d'articles publiés le jour de la publication effective.
Politiques nécessitant ce contexte :
  - POL-LIMIT-PUB-001 : Nécessite le contexte du jour de publication
Justification :
  L'évaluation nécessite un contexte futur qui n'est pas encore disponible.
```

**Caractéristiques :**
- Contexte futur requis identifié
- Raison du différé explicite
- Pas de planification (l'adaptateur décide quand réévaluer)

---

### 5.2. Cas DIFFÉRÉE — Dépendance à un état futur

**Intention évaluée :**
```
Identifiant : intent-2026-01-26-009
Action : MODIFIER
Type d'entité : ARTICLE
Données :
  - Statut demandé : PUBLIÉ
Contexte :
  - État actuel de l'article : EN_REVUE (en cours de révision)
```

**Décision produite :**
```
Type : DIFFÉRÉE
Identifiant intention : intent-2026-01-26-009
Contexte futur requis :
  - État de l'article : APPROUVÉ (actuellement EN_REVUE)
Raison du différé :
  L'intention demande de publier un article qui est actuellement en révision.
  La politique POL-PUBLICATION-APPROUVÉE exige que l'article soit APPROUVÉ.
```

---

## 6. Scénarios complets

### 6.1. Scénario : Workflow de publication

**Étape 1 : Création brouillon**
- Intention : Créer article en BROUILLON
- Décision : **ACCEPTÉE** (politiques de publication non applicables)

**Étape 2 : Passage en révision**
- Intention : Modifier article → EN_REVUE
- Décision : **ACCEPTÉE**

**Étape 3 : Publication**
- Intention : Modifier article → PUBLIÉ
- Décision : **ACCEPTÉE** (toutes les politiques satisfaites)

---

### 6.2. Scénario : Clarification d'ambiguïté

**Étape 1 : Intention ambiguë**
- Intention : Créer article sans auteur
- Décision : **AMBIGUË** (auteur manquant)

**Étape 2 : Intention clarifiée**
- Intention : Créer article avec auteur spécifié
- Décision : **ACCEPTÉE**

---

## 7. Points clés sur les décisions

### 7.1. Caractéristiques des décisions

- **Non exécutables :** Une décision n'est jamais exécutée automatiquement
- **Justifiées :** Toute décision contient une justification complète
- **Traçables :** Toute décision est tracée avec son contexte
- **Non ambiguës :** Une décision est toujours claire

### 7.2. Distinctions importantes

| Distinction | Explication |
|-------------|-------------|
| **AMBIGUË vs DIFFÉRÉE** | AMBIGUË = information manquante, DIFFÉRÉE = contexte futur requis |
| **REFUSÉE vs AMBIGUË** | REFUSÉE = politique violée, AMBIGUË = évaluation impossible |
| **ACCEPTÉE vs Exécution** | ACCEPTÉE = décision, Exécution = responsabilité adaptateur |

### 7.3. Ce qu'une décision N'EST PAS

- **Pas une commande** : La décision ne déclenche aucune action
- **Pas un ordre** : L'adaptateur décide de l'exécution
- **Pas une planification** : DIFFÉRÉE ne planifie pas de réévaluation

---

**Document créé le :** 2026-01-27  
**Version :** 1.1 (réorganisation)  
**Statut :** Illustratif et pédagogique  
**Référence :** Miyukini Core System v2.4, StrongFather Core Decision Contract  
**Type :** Exemples et cas d'usage
