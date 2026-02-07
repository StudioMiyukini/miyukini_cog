# StrongFather — Examples Policies

## Contexte

Ce document illustre StrongFather par des **exemples de politiques** appliquées par le moteur de décision. Il fait partie de la série de documents d'exemples.

**Documents connexes :**
- [StrongFather - Examples Intentions](./StrongFather%20-%20Examples%20Intentions.md)
- [StrongFather - Examples Decisions](./StrongFather%20-%20Examples%20Decisions.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 1. Types de politiques

Les politiques sont des règles déclaratives qui déterminent la validité d'une intention. StrongFather supporte 5 types de politiques :

| Type | Description | Exemple d'usage |
|------|-------------|-----------------|
| **PERMISSION** | Détermine si un acteur est autorisé | Rôle requis pour créer un article |
| **CONTRAINTE** | Définit des conditions à satisfaire | Limite de publications par jour |
| **PRIORITÉ** | Détermine l'ordre d'importance | Priorité selon la catégorie |
| **VALIDATION** | Valide la cohérence ou conformité | Longueur minimale de contenu |
| **COMPOSITE** | Combine plusieurs politiques | ET, OU, NON logiques |

---

## 2. Exemples de politiques de permission

### 2.1. Politique de permission utilisateur

**Définition :**
```
Identifiant : POL-USER-PERM-001
Nom : Permission de création d'article
Description : Un utilisateur peut créer un article uniquement s'il possède le rôle ÉDITEUR ou ADMIN
Type : PERMISSION
Portée : CRÉER + ARTICLE
Condition :
  - Utilisateur.role IN [ÉDITEUR, ADMIN]
```

**Application :**
- Évaluée pour toute intention CRÉER + ARTICLE
- Vérifie le rôle de l'utilisateur dans le contexte
- Résultat : SATISFAITE ou NON_SATISFAITE

---

### 2.2. Politique de modification conditionnelle

**Définition :**
```
Identifiant : POL-MODIF-001
Nom : Permission de modification
Description : Un utilisateur peut modifier un article uniquement s'il en est l'auteur ou s'il possède le rôle ADMIN
Type : PERMISSION
Portée : MODIFIER + ARTICLE
Condition :
  - Utilisateur == Article.auteur OU Utilisateur.role == ADMIN
```

**Application :**
- Évaluée pour toute intention MODIFIER + ARTICLE
- Nécessite un fait : auteur de l'article
- Résultat : SATISFAITE, NON_SATISFAITE, ou INDÉTERMINÉE (si fait manquant)

---

### 2.3. Politique de suppression

**Définition :**
```
Identifiant : POL-SUPPR-001
Nom : Permission de suppression
Description : Seul l'auteur ou un ADMIN peut supprimer un article publié
Type : PERMISSION
Portée : SUPPRIMER + ARTICLE + Statut PUBLIÉ
Condition :
  - Utilisateur == Article.auteur OU Utilisateur.role == ADMIN
```

**Application :**
- Évaluée pour toute intention SUPPRIMER + ARTICLE avec statut PUBLIÉ
- Protection des articles publiés

---

## 3. Exemples de politiques de contrainte

### 3.1. Politique de limite de publication

**Définition :**
```
Identifiant : POL-LIMIT-PUB-001
Nom : Limite de publications simultanées
Description : Un utilisateur ne peut pas publier plus de 10 articles par jour
Type : CONTRAINTE
Portée : CRÉER + ARTICLE + Statut PUBLIÉ
Condition :
  - COUNT(Articles publiés aujourd'hui par Utilisateur) < 10
```

**Application :**
- Évaluée pour toute intention CRÉER + ARTICLE avec statut PUBLIÉ
- Nécessite un fait : nombre d'articles publiés aujourd'hui
- Résultat : SATISFAITE, NON_SATISFAITE, ou INDÉTERMINÉE (si fait manquant)

---

### 3.2. Politique de taille maximale

**Définition :**
```
Identifiant : POL-SIZE-MAX-001
Nom : Taille maximale d'article
Description : Un article ne peut pas dépasser 50 000 caractères
Type : CONTRAINTE
Portée : CRÉER + ARTICLE
Condition :
  - LENGTH(Article.contenu) <= 50000
```

**Application :**
- Évaluée pour toute intention CRÉER + ARTICLE
- Vérifie la taille du contenu dans les données de l'intention

---

## 4. Exemples de politiques de validation

### 4.1. Politique de validation de contenu

**Définition :**
```
Identifiant : POL-VALID-CONT-001
Nom : Validation de contenu obligatoire
Description : Un article ne peut être publié que s'il contient au moins 500 caractères
Type : VALIDATION
Portée : CRÉER + ARTICLE + Statut PUBLIÉ
Condition :
  - LENGTH(Article.contenu) >= 500
```

**Application :**
- Évaluée pour toute intention CRÉER + ARTICLE avec statut PUBLIÉ
- Vérifie la longueur du contenu dans les données de l'intention
- Résultat : SATISFAITE ou NON_SATISFAITE

---

### 4.2. Politique de validation de titre

**Définition :**
```
Identifiant : POL-VALID-TITLE-001
Nom : Validation de titre
Description : Le titre d'un article doit contenir entre 10 et 200 caractères
Type : VALIDATION
Portée : CRÉER + ARTICLE
Condition :
  - LENGTH(Article.titre) >= 10 AND LENGTH(Article.titre) <= 200
```

**Application :**
- Évaluée pour toute intention CRÉER + ARTICLE
- Vérifie la longueur du titre

---

## 5. Exemples de politiques de priorité

### 5.1. Politique de priorité selon catégorie

**Définition :**
```
Identifiant : POL-PRIO-CAT-001
Nom : Priorité selon catégorie
Description : Les articles de catégorie SÉCURITÉ ont priorité maximale
Type : PRIORITÉ
Portée : CRÉER + ARTICLE
Règle :
  - Si Catégorie == SÉCURITÉ : Priorité = MAXIMALE
  - Si Catégorie == TECHNIQUE : Priorité = HAUTE
  - Sinon : Priorité = NORMALE
```

**Application :**
- Évaluée uniquement si toutes les politiques de validation sont satisfaites
- Détermine la priorité relative de l'intention
- Résultat : Priorité calculée

---

### 5.2. Politique de priorité selon urgence

**Définition :**
```
Identifiant : POL-PRIO-URG-001
Nom : Priorité selon urgence déclarée
Description : Si une intention déclare une urgence, augmente la priorité
Type : PRIORITÉ
Portée : Toutes les intentions
Règle :
  - Si Métadonnées.urgence == true : Priorité += 1
```

**Application :**
- Évaluée pour toute intention avec métadonnée urgence
- Modifie la priorité calculée

---

## 6. Exemples de politiques composites

### 6.1. Politique composite ET

**Définition :**
```
Identifiant : POL-COMP-AND-001
Nom : Accès premium requis
Description : L'utilisateur doit avoir le rôle PREMIUM ET être vérifié
Type : COMPOSITE
Opérateur : ET
Politiques composantes :
  - POL-ROLE-PREMIUM
  - POL-USER-VERIFIED
```

**Application :**
- Les deux politiques composantes doivent être satisfaites

---

### 6.2. Politique composite OU

**Définition :**
```
Identifiant : POL-COMP-OR-001
Nom : Accès éditorial
Description : L'utilisateur doit avoir le rôle ÉDITEUR OU le rôle ADMIN
Type : COMPOSITE
Opérateur : OU
Politiques composantes :
  - POL-ROLE-EDITOR
  - POL-ROLE-ADMIN
```

**Application :**
- Au moins une des politiques composantes doit être satisfaite

---

## 7. Points clés sur les politiques

### 7.1. Caractéristiques des politiques

- **Explicites :** Définies de manière déclarative, sans logique implicite
- **Déclaratives :** Expriment ce qui est autorisé/interdit, pas comment évaluer
- **Centralisées :** Définies une fois, appliquées de manière cohérente
- **Versionnées :** Peuvent évoluer dans le temps avec traçabilité

### 7.2. Ce qu'une politique N'EST PAS

- **Pas de logique d'exécution :** Une politique ne déclenche jamais d'action
- **Pas de logique métier spécifique :** Règles générales, pas métier
- **Pas de validation technique :** Pas de vérification de schémas de données

### 7.3. Résolution de conflits

Lorsque plusieurs politiques produisent des résultats contradictoires :
1. Les politiques de priorité supérieure priment
2. L'interdiction prime sur l'autorisation
3. Les politiques spécifiques priment sur les générales

---

**Document créé le :** 2026-01-27  
**Version :** 1.1 (réorganisation)  
**Statut :** Illustratif et pédagogique  
**Référence :** Miyukini Core System v2.4, StrongFather Policy Engine Contract  
**Type :** Exemples et cas d'usage
