# StrongFather â€” Examples Policies

## Contexte

Ce document illustre StrongFather par des **exemples de politiques** appliquÃ©es par le moteur de dÃ©cision. Il fait partie de la sÃ©rie de documents d'exemples.

**Documents connexes :**
- [StrongFather - Examples Intentions](./StrongFather%20-%20Examples%20Intentions.md)
- [StrongFather - Examples Decisions](./StrongFather%20-%20Examples%20Decisions.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 1. Types de politiques

Les politiques sont des rÃ¨gles dÃ©claratives qui dÃ©terminent la validitÃ© d'une intention. StrongFather supporte 5 types de politiques :

| Type | Description | Exemple d'usage |
|------|-------------|-----------------|
| **PERMISSION** | DÃ©termine si un acteur est autorisÃ© | RÃ´le requis pour crÃ©er un article |
| **CONTRAINTE** | DÃ©finit des conditions Ã  satisfaire | Limite de publications par jour |
| **PRIORITÃ‰** | DÃ©termine l'ordre d'importance | PrioritÃ© selon la catÃ©gorie |
| **VALIDATION** | Valide la cohÃ©rence ou conformitÃ© | Longueur minimale de contenu |
| **COMPOSITE** | Combine plusieurs politiques | ET, OU, NON logiques |

---

## 2. Exemples de politiques de permission

### 2.1. Politique de permission utilisateur

**DÃ©finition :**
```
Identifiant : POL-USER-PERM-001
Nom : Permission de crÃ©ation d'article
Description : Un utilisateur peut crÃ©er un article uniquement s'il possÃ¨de le rÃ´le Ã‰DITEUR ou ADMIN
Type : PERMISSION
PortÃ©e : CRÃ‰ER + ARTICLE
Condition :
  - Utilisateur.role IN [Ã‰DITEUR, ADMIN]
```

**Application :**
- Ã‰valuÃ©e pour toute intention CRÃ‰ER + ARTICLE
- VÃ©rifie le rÃ´le de l'utilisateur dans le contexte
- RÃ©sultat : SATISFAITE ou NON_SATISFAITE

---

### 2.2. Politique de modification conditionnelle

**DÃ©finition :**
```
Identifiant : POL-MODIF-001
Nom : Permission de modification
Description : Un utilisateur peut modifier un article uniquement s'il en est l'auteur ou s'il possÃ¨de le rÃ´le ADMIN
Type : PERMISSION
PortÃ©e : MODIFIER + ARTICLE
Condition :
  - Utilisateur == Article.auteur OU Utilisateur.role == ADMIN
```

**Application :**
- Ã‰valuÃ©e pour toute intention MODIFIER + ARTICLE
- NÃ©cessite un fait : auteur de l'article
- RÃ©sultat : SATISFAITE, NON_SATISFAITE, ou INDÃ‰TERMINÃ‰E (si fait manquant)

---

### 2.3. Politique de suppression

**DÃ©finition :**
```
Identifiant : POL-SUPPR-001
Nom : Permission de suppression
Description : Seul l'auteur ou un ADMIN peut supprimer un article publiÃ©
Type : PERMISSION
PortÃ©e : SUPPRIMER + ARTICLE + Statut PUBLIÃ‰
Condition :
  - Utilisateur == Article.auteur OU Utilisateur.role == ADMIN
```

**Application :**
- Ã‰valuÃ©e pour toute intention SUPPRIMER + ARTICLE avec statut PUBLIÃ‰
- Protection des articles publiÃ©s

---

## 3. Exemples de politiques de contrainte

### 3.1. Politique de limite de publication

**DÃ©finition :**
```
Identifiant : POL-LIMIT-PUB-001
Nom : Limite de publications simultanÃ©es
Description : Un utilisateur ne peut pas publier plus de 10 articles par jour
Type : CONTRAINTE
PortÃ©e : CRÃ‰ER + ARTICLE + Statut PUBLIÃ‰
Condition :
  - COUNT(Articles publiÃ©s aujourd'hui par Utilisateur) < 10
```

**Application :**
- Ã‰valuÃ©e pour toute intention CRÃ‰ER + ARTICLE avec statut PUBLIÃ‰
- NÃ©cessite un fait : nombre d'articles publiÃ©s aujourd'hui
- RÃ©sultat : SATISFAITE, NON_SATISFAITE, ou INDÃ‰TERMINÃ‰E (si fait manquant)

---

### 3.2. Politique de taille maximale

**DÃ©finition :**
```
Identifiant : POL-SIZE-MAX-001
Nom : Taille maximale d'article
Description : Un article ne peut pas dÃ©passer 50 000 caractÃ¨res
Type : CONTRAINTE
PortÃ©e : CRÃ‰ER + ARTICLE
Condition :
  - LENGTH(Article.contenu) <= 50000
```

**Application :**
- Ã‰valuÃ©e pour toute intention CRÃ‰ER + ARTICLE
- VÃ©rifie la taille du contenu dans les donnÃ©es de l'intention

---

## 4. Exemples de politiques de validation

### 4.1. Politique de validation de contenu

**DÃ©finition :**
```
Identifiant : POL-VALID-CONT-001
Nom : Validation de contenu obligatoire
Description : Un article ne peut Ãªtre publiÃ© que s'il contient au moins 500 caractÃ¨res
Type : VALIDATION
PortÃ©e : CRÃ‰ER + ARTICLE + Statut PUBLIÃ‰
Condition :
  - LENGTH(Article.contenu) >= 500
```

**Application :**
- Ã‰valuÃ©e pour toute intention CRÃ‰ER + ARTICLE avec statut PUBLIÃ‰
- VÃ©rifie la longueur du contenu dans les donnÃ©es de l'intention
- RÃ©sultat : SATISFAITE ou NON_SATISFAITE

---

### 4.2. Politique de validation de titre

**DÃ©finition :**
```
Identifiant : POL-VALID-TITLE-001
Nom : Validation de titre
Description : Le titre d'un article doit contenir entre 10 et 200 caractÃ¨res
Type : VALIDATION
PortÃ©e : CRÃ‰ER + ARTICLE
Condition :
  - LENGTH(Article.titre) >= 10 AND LENGTH(Article.titre) <= 200
```

**Application :**
- Ã‰valuÃ©e pour toute intention CRÃ‰ER + ARTICLE
- VÃ©rifie la longueur du titre

---

## 5. Exemples de politiques de prioritÃ©

### 5.1. Politique de prioritÃ© selon catÃ©gorie

**DÃ©finition :**
```
Identifiant : POL-PRIO-CAT-001
Nom : PrioritÃ© selon catÃ©gorie
Description : Les articles de catÃ©gorie SÃ‰CURITÃ‰ ont prioritÃ© maximale
Type : PRIORITÃ‰
PortÃ©e : CRÃ‰ER + ARTICLE
RÃ¨gle :
  - Si CatÃ©gorie == SÃ‰CURITÃ‰ : PrioritÃ© = MAXIMALE
  - Si CatÃ©gorie == TECHNIQUE : PrioritÃ© = HAUTE
  - Sinon : PrioritÃ© = NORMALE
```

**Application :**
- Ã‰valuÃ©e uniquement si toutes les politiques de validation sont satisfaites
- DÃ©termine la prioritÃ© relative de l'intention
- RÃ©sultat : PrioritÃ© calculÃ©e

---

### 5.2. Politique de prioritÃ© selon urgence

**DÃ©finition :**
```
Identifiant : POL-PRIO-URG-001
Nom : PrioritÃ© selon urgence dÃ©clarÃ©e
Description : Si une intention dÃ©clare une urgence, augmente la prioritÃ©
Type : PRIORITÃ‰
PortÃ©e : Toutes les intentions
RÃ¨gle :
  - Si MÃ©tadonnÃ©es.urgence == true : PrioritÃ© += 1
```

**Application :**
- Ã‰valuÃ©e pour toute intention avec mÃ©tadonnÃ©e urgence
- Modifie la prioritÃ© calculÃ©e

---

## 6. Exemples de politiques composites

### 6.1. Politique composite ET

**DÃ©finition :**
```
Identifiant : POL-COMP-AND-001
Nom : AccÃ¨s premium requis
Description : L'utilisateur doit avoir le rÃ´le PREMIUM ET Ãªtre vÃ©rifiÃ©
Type : COMPOSITE
OpÃ©rateur : ET
Politiques composantes :
  - POL-ROLE-PREMIUM
  - POL-USER-VERIFIED
```

**Application :**
- Les deux politiques composantes doivent Ãªtre satisfaites

---

### 6.2. Politique composite OU

**DÃ©finition :**
```
Identifiant : POL-COMP-OR-001
Nom : AccÃ¨s Ã©ditorial
Description : L'utilisateur doit avoir le rÃ´le Ã‰DITEUR OU le rÃ´le ADMIN
Type : COMPOSITE
OpÃ©rateur : OU
Politiques composantes :
  - POL-ROLE-EDITOR
  - POL-ROLE-ADMIN
```

**Application :**
- Au moins une des politiques composantes doit Ãªtre satisfaite

---

## 7. Points clÃ©s sur les politiques

### 7.1. CaractÃ©ristiques des politiques

- **Explicites :** DÃ©finies de maniÃ¨re dÃ©clarative, sans logique implicite
- **DÃ©claratives :** Expriment ce qui est autorisÃ©/interdit, pas comment Ã©valuer
- **CentralisÃ©es :** DÃ©finies une fois, appliquÃ©es de maniÃ¨re cohÃ©rente
- **VersionnÃ©es :** Peuvent Ã©voluer dans le temps avec traÃ§abilitÃ©

### 7.2. Ce qu'une politique N'EST PAS

- **Pas de logique d'exÃ©cution :** Une politique ne dÃ©clenche jamais d'action
- **Pas de logique mÃ©tier spÃ©cifique :** RÃ¨gles gÃ©nÃ©rales, pas mÃ©tier
- **Pas de validation technique :** Pas de vÃ©rification de schÃ©mas de donnÃ©es

### 7.3. RÃ©solution de conflits

Lorsque plusieurs politiques produisent des rÃ©sultats contradictoires :
1. Les politiques de prioritÃ© supÃ©rieure priment
2. L'interdiction prime sur l'autorisation
3. Les politiques spÃ©cifiques priment sur les gÃ©nÃ©rales

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.1 (rÃ©organisation)  
**Statut :** Illustratif et pÃ©dagogique  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, StrongFather Policy Engine Contract  
**Type :** Exemples et cas d'usage

