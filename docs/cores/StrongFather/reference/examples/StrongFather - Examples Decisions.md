# StrongFather â€” Examples Decisions

## Contexte

Ce document illustre StrongFather par des **exemples de dÃ©cisions** produites par le moteur de dÃ©cision. Il fait partie de la sÃ©rie de documents d'exemples.

**Documents connexes :**
- [StrongFather - Examples Intentions](./StrongFather%20-%20Examples%20Intentions.md)
- [StrongFather - Examples Policies](./StrongFather%20-%20Examples%20Policies.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 1. Types de dÃ©cisions

StrongFather produit 4 types de dÃ©cisions :

| Type | Description | ConsÃ©quence |
|------|-------------|-------------|
| **ACCEPTÃ‰E** | Intention valide selon toutes les politiques | L'adaptateur peut procÃ©der |
| **REFUSÃ‰E** | Au moins une politique violÃ©e | L'intention ne doit pas Ãªtre exÃ©cutÃ©e |
| **AMBIGUÃ‹** | Information manquante dans l'intention | Clarifications requises |
| **DIFFÃ‰RÃ‰E** | Contexte futur requis | RÃ©Ã©valuation nÃ©cessaire |

---

## 2. DÃ©cisions ACCEPTÃ‰E

### 2.1. DÃ©cision ACCEPTÃ‰E â€” Cas standard

**Intention Ã©valuÃ©e :**
- Intention intent-2026-01-26-001 (crÃ©ation d'article par user-alice)

**Politiques appliquÃ©es :**
- POL-USER-PERM-001 : SATISFAITE (user-alice a le rÃ´le Ã‰DITEUR)
- POL-VALID-CONT-001 : SATISFAITE (contenu de 1200 caractÃ¨res)
- POL-LIMIT-PUB-001 : SATISFAITE (3 articles publiÃ©s aujourd'hui)

**DÃ©cision produite :**
```
Type : ACCEPTÃ‰E
Identifiant intention : intent-2026-01-26-001
PrioritÃ© Ã©tablie : NORMALE
Politiques appliquÃ©es :
  - POL-USER-PERM-001 : SATISFAITE
  - POL-VALID-CONT-001 : SATISFAITE
  - POL-LIMIT-PUB-001 : SATISFAITE
Justification :
  L'intention est valide selon toutes les politiques applicables.
  L'utilisateur user-alice possÃ¨de le rÃ´le Ã‰DITEUR requis.
  Le contenu respecte la longueur minimale de 500 caractÃ¨res.
  La limite de publications quotidiennes n'est pas atteinte (3/10).
```

**CaractÃ©ristiques :**
- Toutes les politiques satisfaites
- PrioritÃ© calculÃ©e
- Justification complÃ¨te
- DÃ©cision non exÃ©cutable (l'adaptateur dÃ©cide de l'exÃ©cution)

---

### 2.2. DÃ©cision ACCEPTÃ‰E â€” Cas avec prioritÃ© Ã©levÃ©e

**Intention Ã©valuÃ©e :**
- Intention intent-2026-01-26-004 (crÃ©ation d'article SÃ‰CURITÃ‰ par user-admin)

**DÃ©cision produite :**
```
Type : ACCEPTÃ‰E
Identifiant intention : intent-2026-01-26-004
PrioritÃ© Ã©tablie : MAXIMALE
Politiques appliquÃ©es :
  - POL-USER-PERM-001 : SATISFAITE
  - POL-VALID-CONT-001 : SATISFAITE
  - POL-LIMIT-PUB-001 : SATISFAITE
  - POL-PRIO-CAT-001 : PrioritÃ© MAXIMALE
Justification :
  L'intention est valide selon toutes les politiques applicables.
  La prioritÃ© MAXIMALE est Ã©tablie selon la catÃ©gorie SÃ‰CURITÃ‰.
```

---

## 3. DÃ©cisions REFUSÃ‰E

### 3.1. DÃ©cision REFUSÃ‰E â€” Permission insuffisante

**Intention Ã©valuÃ©e :**
- Intention intent-2026-01-26-001 (crÃ©ation d'article par user-visitor)

**DÃ©cision produite :**
```
Type : REFUSÃ‰E
Identifiant intention : intent-2026-01-26-001
Politiques violÃ©es :
  - POL-USER-PERM-001 : NON_SATISFAITE
Raison explicite :
  L'utilisateur user-visitor ne possÃ¨de pas le rÃ´le requis (Ã‰DITEUR ou ADMIN)
  pour crÃ©er un article. Le rÃ´le actuel (VISITEUR) ne permet pas cette action.
Justification :
  La politique POL-USER-PERM-001 exige que l'utilisateur possÃ¨de le rÃ´le
  Ã‰DITEUR ou ADMIN pour crÃ©er un article.
```

**CaractÃ©ristiques :**
- Au moins une politique non satisfaite
- Raison explicite du refus
- Politiques violÃ©es identifiÃ©es
- DÃ©cision dÃ©finitive

---

### 3.2. DÃ©cision REFUSÃ‰E â€” Limite atteinte

**Intention Ã©valuÃ©e :**
- Intention de crÃ©ation d'article par user-alice (12 articles dÃ©jÃ  publiÃ©s aujourd'hui)

**DÃ©cision produite :**
```
Type : REFUSÃ‰E
Identifiant intention : intent-2026-01-26-001
Politiques violÃ©es :
  - POL-LIMIT-PUB-001 : NON_SATISFAITE
Raison explicite :
  La limite de publications quotidiennes est atteinte. L'utilisateur user-alice
  a dÃ©jÃ  publiÃ© 12 articles aujourd'hui, ce qui dÃ©passe la limite de 10 articles.
Justification :
  La politique POL-LIMIT-PUB-001 limite le nombre de publications quotidiennes
  Ã  10 articles par utilisateur. Nombre actuel : 12/10.
```

---

### 3.3. DÃ©cision REFUSÃ‰E â€” Contenu insuffisant

**Intention Ã©valuÃ©e :**
- Intention de crÃ©ation d'article avec contenu de 250 caractÃ¨res (minimum 500)

**DÃ©cision produite :**
```
Type : REFUSÃ‰E
Identifiant intention : intent-2026-01-26-001
Politiques violÃ©es :
  - POL-VALID-CONT-001 : NON_SATISFAITE
Raison explicite :
  Le contenu de l'article ne respecte pas la longueur minimale requise.
  Contenu fourni : 250 caractÃ¨res. Minimum requis : 500 caractÃ¨res.
```

---

### 3.4. DÃ©cision REFUSÃ‰E â€” Rejet structurel

**Intention Ã©valuÃ©e :**
- Intention sans action spÃ©cifiÃ©e

**DÃ©cision produite :**
```
Type : REFUSÃ‰E
Identifiant intention : intent-2026-01-26-010
Politiques violÃ©es :
  - (Aucune politique Ã©valuÃ©e â€” rejet structurel)
Raison explicite :
  L'intention est structurellement invalide. Le champ "Action" est
  obligatoire et manquant. Une intention doit toujours spÃ©cifier
  l'action demandÃ©e (CRÃ‰ER, MODIFIER, SUPPRIMER, etc.).
Justification :
  L'intention ne respecte pas les rÃ¨gles de formation dÃ©finies dans
  le Intent Model Contract.
```

---

## 4. DÃ©cisions AMBIGUÃ‹

### 4.1. Cas AMBIGUÃ‹ â€” Information manquante

**Intention Ã©valuÃ©e :**
```
Identifiant : intent-2026-01-26-005
Action : CRÃ‰ER
Type d'entitÃ© : ARTICLE
DonnÃ©es :
  - Titre : "Article sans auteur"
  - Statut demandÃ© : PUBLIÃ‰
  - Auteur : (manquant)
```

**DÃ©cision produite :**
```
Type : AMBIGUÃ‹
Identifiant intention : intent-2026-01-26-005
Ã‰lÃ©ments manquants :
  - Auteur de l'article (champ obligatoire)
Clarifications requises :
  - L'auteur de l'article doit Ãªtre spÃ©cifiÃ© dans les donnÃ©es de l'intention.
Politiques nÃ©cessitant ces clarifications :
  - POL-USER-PERM-001 : NÃ©cessite l'auteur pour vÃ©rifier les permissions
  - POL-LIMIT-PUB-001 : NÃ©cessite l'auteur pour vÃ©rifier la limite quotidienne
Justification :
  L'intention est insuffisamment dÃ©finie. Le champ "Auteur" est manquant,
  ce qui empÃªche l'Ã©valuation des politiques de permission et de limite.
```

**CaractÃ©ristiques :**
- Information manquante identifiÃ©e
- Clarifications requises explicites
- Politiques affectÃ©es identifiÃ©es
- Suspension d'Ã©valuation jusqu'Ã  clarification

---

### 4.2. Cas AMBIGUÃ‹ â€” Contexte utilisateur ambigu

**Intention Ã©valuÃ©e :**
```
Identifiant : intent-2026-01-26-006
Action : MODIFIER
Type d'entitÃ© : ARTICLE
Contexte :
  - Utilisateur : (non spÃ©cifiÃ©)
```

**DÃ©cision produite :**
```
Type : AMBIGUÃ‹
Identifiant intention : intent-2026-01-26-006
Ã‰lÃ©ments manquants :
  - Utilisateur dans le contexte (champ obligatoire)
Clarifications requises :
  - L'utilisateur effectuant la modification doit Ãªtre spÃ©cifiÃ© dans le contexte.
```

---

## 5. DÃ©cisions DIFFÃ‰RÃ‰E

### 5.1. Cas DIFFÃ‰RÃ‰E â€” DÃ©pendance Ã  un Ã©vÃ©nement futur

**Intention Ã©valuÃ©e :**
```
Identifiant : intent-2026-01-26-008
Action : CRÃ‰ER
Type d'entitÃ© : ARTICLE
DonnÃ©es :
  - Date de publication : 2026-01-27T10:00:00Z (demain)
Contexte :
  - Date actuelle : 2026-01-26T15:00:00Z
```

**DÃ©cision produite :**
```
Type : DIFFÃ‰RÃ‰E
Identifiant intention : intent-2026-01-26-008
Contexte futur requis :
  - Date effective de publication (2026-01-27T10:00:00Z) pour Ã©valuer
    la limite quotidienne de publications
Raison du diffÃ©rÃ© :
  L'intention spÃ©cifie une date de publication future (2026-01-27).
  La politique POL-LIMIT-PUB-001 nÃ©cessite de connaÃ®tre le nombre
  d'articles publiÃ©s le jour de la publication effective.
Politiques nÃ©cessitant ce contexte :
  - POL-LIMIT-PUB-001 : NÃ©cessite le contexte du jour de publication
Justification :
  L'Ã©valuation nÃ©cessite un contexte futur qui n'est pas encore disponible.
```

**CaractÃ©ristiques :**
- Contexte futur requis identifiÃ©
- Raison du diffÃ©rÃ© explicite
- Pas de planification (l'adaptateur dÃ©cide quand rÃ©Ã©valuer)

---

### 5.2. Cas DIFFÃ‰RÃ‰E â€” DÃ©pendance Ã  un Ã©tat futur

**Intention Ã©valuÃ©e :**
```
Identifiant : intent-2026-01-26-009
Action : MODIFIER
Type d'entitÃ© : ARTICLE
DonnÃ©es :
  - Statut demandÃ© : PUBLIÃ‰
Contexte :
  - Ã‰tat actuel de l'article : EN_REVUE (en cours de rÃ©vision)
```

**DÃ©cision produite :**
```
Type : DIFFÃ‰RÃ‰E
Identifiant intention : intent-2026-01-26-009
Contexte futur requis :
  - Ã‰tat de l'article : APPROUVÃ‰ (actuellement EN_REVUE)
Raison du diffÃ©rÃ© :
  L'intention demande de publier un article qui est actuellement en rÃ©vision.
  La politique POL-PUBLICATION-APPROUVÃ‰E exige que l'article soit APPROUVÃ‰.
```

---

## 6. ScÃ©narios complets

### 6.1. ScÃ©nario : Workflow de publication

**Ã‰tape 1 : CrÃ©ation brouillon**
- Intention : CrÃ©er article en BROUILLON
- DÃ©cision : **ACCEPTÃ‰E** (politiques de publication non applicables)

**Ã‰tape 2 : Passage en rÃ©vision**
- Intention : Modifier article â†’ EN_REVUE
- DÃ©cision : **ACCEPTÃ‰E**

**Ã‰tape 3 : Publication**
- Intention : Modifier article â†’ PUBLIÃ‰
- DÃ©cision : **ACCEPTÃ‰E** (toutes les politiques satisfaites)

---

### 6.2. ScÃ©nario : Clarification d'ambiguÃ¯tÃ©

**Ã‰tape 1 : Intention ambiguÃ«**
- Intention : CrÃ©er article sans auteur
- DÃ©cision : **AMBIGUÃ‹** (auteur manquant)

**Ã‰tape 2 : Intention clarifiÃ©e**
- Intention : CrÃ©er article avec auteur spÃ©cifiÃ©
- DÃ©cision : **ACCEPTÃ‰E**

---

## 7. Points clÃ©s sur les dÃ©cisions

### 7.1. CaractÃ©ristiques des dÃ©cisions

- **Non exÃ©cutables :** Une dÃ©cision n'est jamais exÃ©cutÃ©e automatiquement
- **JustifiÃ©es :** Toute dÃ©cision contient une justification complÃ¨te
- **TraÃ§ables :** Toute dÃ©cision est tracÃ©e avec son contexte
- **Non ambiguÃ«s :** Une dÃ©cision est toujours claire

### 7.2. Distinctions importantes

| Distinction | Explication |
|-------------|-------------|
| **AMBIGUÃ‹ vs DIFFÃ‰RÃ‰E** | AMBIGUÃ‹ = information manquante, DIFFÃ‰RÃ‰E = contexte futur requis |
| **REFUSÃ‰E vs AMBIGUÃ‹** | REFUSÃ‰E = politique violÃ©e, AMBIGUÃ‹ = Ã©valuation impossible |
| **ACCEPTÃ‰E vs ExÃ©cution** | ACCEPTÃ‰E = dÃ©cision, ExÃ©cution = responsabilitÃ© adaptateur |

### 7.3. Ce qu'une dÃ©cision N'EST PAS

- **Pas une commande** : La dÃ©cision ne dÃ©clenche aucune action
- **Pas un ordre** : L'adaptateur dÃ©cide de l'exÃ©cution
- **Pas une planification** : DIFFÃ‰RÃ‰E ne planifie pas de rÃ©Ã©valuation

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.1 (rÃ©organisation)  
**Statut :** Illustratif et pÃ©dagogique  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, StrongFather Core Decision Contract  
**Type :** Exemples et cas d'usage

