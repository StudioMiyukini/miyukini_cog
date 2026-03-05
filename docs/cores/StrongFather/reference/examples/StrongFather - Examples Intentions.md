# StrongFather â€” Examples Intentions

## Contexte

Ce document illustre StrongFather par des **exemples d'intentions** soumises au moteur de dÃ©cision. Il fait partie de la sÃ©rie de documents d'exemples.

**Documents connexes :**
- [StrongFather - Examples Policies](./StrongFather%20-%20Examples%20Policies.md)
- [StrongFather - Examples Decisions](./StrongFather%20-%20Examples%20Decisions.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 1. Structure d'une intention

Une intention est une demande d'action soumise Ã  StrongFather pour Ã©valuation. Elle contient :

**Composants obligatoires :**
- Identifiant unique
- Action (CRÃ‰ER, MODIFIER, SUPPRIMER, LECTURE, Ã‰VALUATION)
- Type d'entitÃ©
- Contexte (utilisateur, produit, instance)

**Composants optionnels :**
- DonnÃ©es associÃ©es
- PrioritÃ© demandÃ©e
- Contraintes explicites
- MÃ©tadonnÃ©es

---

## 2. Exemples d'intentions

### 2.1. Intention de crÃ©ation de contenu

**Contexte :**
Un adaptateur produit soumet une intention de crÃ©ation d'un article de blog.

**Intention :**
```
Identifiant : intent-2026-01-26-001
Action : CRÃ‰ER
Type d'entitÃ© : ARTICLE
DonnÃ©es :
  - Titre : "Introduction Ã  StrongFather"
  - Auteur : user-alice
  - Statut demandÃ© : PUBLIÃ‰
  - CatÃ©gorie : TECHNIQUE
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
MÃ©tadonnÃ©es :
  - PrioritÃ© demandÃ©e : NORMALE
  - Contraintes : aucune
```

**CaractÃ©ristiques :**
- Intention structurellement valide
- Tous les champs obligatoires prÃ©sents
- Contexte complet fourni
- Action et type d'entitÃ© explicites

---

### 2.2. Intention de modification de contenu

**Contexte :**
Un adaptateur produit soumet une intention de modification d'un article existant.

**Intention :**
```
Identifiant : intent-2026-01-26-002
Action : MODIFIER
Type d'entitÃ© : ARTICLE
Identifiant d'entitÃ© : article-12345
DonnÃ©es :
  - Titre : "Introduction Ã  StrongFather â€” Version mise Ã  jour"
  - Statut demandÃ© : PUBLIÃ‰
Contexte :
  - Utilisateur : user-bob
  - Produit : miyukini-cms
  - Instance : prod-001
MÃ©tadonnÃ©es :
  - PrioritÃ© demandÃ©e : HAUTE
  - Contraintes : aucune
```

**CaractÃ©ristiques :**
- Intention structurellement valide
- RÃ©fÃ©rence Ã  une entitÃ© existante
- Modification partielle (seulement titre et statut)

---

### 2.3. Intention de suppression de contenu

**Contexte :**
Un adaptateur produit soumet une intention de suppression d'un article.

**Intention :**
```
Identifiant : intent-2026-01-26-003
Action : SUPPRIMER
Type d'entitÃ© : ARTICLE
Identifiant d'entitÃ© : article-12345
Contexte :
  - Utilisateur : user-charlie
  - Produit : miyukini-cms
  - Instance : prod-001
MÃ©tadonnÃ©es :
  - PrioritÃ© demandÃ©e : NORMALE
  - Contraintes : aucune
```

**CaractÃ©ristiques :**
- Intention structurellement valide
- Action de suppression
- Pas de donnÃ©es de modification (non applicable)

---

### 2.4. Intention avec prioritÃ© Ã©levÃ©e

**Contexte :**
Un adaptateur produit soumet une intention avec prioritÃ© Ã©levÃ©e pour une publication urgente.

**Intention :**
```
Identifiant : intent-2026-01-26-004
Action : CRÃ‰ER
Type d'entitÃ© : ARTICLE
DonnÃ©es :
  - Titre : "Alerte sÃ©curitÃ© â€” Mise Ã  jour critique"
  - Auteur : user-admin
  - Statut demandÃ© : PUBLIÃ‰
  - CatÃ©gorie : SÃ‰CURITÃ‰
Contexte :
  - Utilisateur : user-admin
  - Produit : miyukini-cms
  - Instance : prod-001
MÃ©tadonnÃ©es :
  - PrioritÃ© demandÃ©e : URGENTE
  - Contraintes : aucune
```

**CaractÃ©ristiques :**
- Intention structurellement valide
- PrioritÃ© URGENTE demandÃ©e
- CatÃ©gorie SÃ‰CURITÃ‰

---

## 3. Intentions invalides (exemples de rejets structurels)

### 3.1. Intention sans action

```
Identifiant : intent-2026-01-26-010
Action : (manquant)
Type d'entitÃ© : ARTICLE
DonnÃ©es :
  - Titre : "Article incomplet"
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
```

**RÃ©sultat attendu :** Rejet structurel â€” Action obligatoire manquante

---

### 3.2. Intention sans identifiant

```
Action : CRÃ‰ER
Type d'entitÃ© : ARTICLE
DonnÃ©es :
  - Titre : "Article sans identifiant"
  - Auteur : user-alice
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
```

**RÃ©sultat attendu :** Rejet structurel â€” Identifiant obligatoire manquant (INV-INT-1)

---

### 3.3. Intention sans contexte utilisateur

```
Identifiant : intent-2026-01-26-006
Action : MODIFIER
Type d'entitÃ© : ARTICLE
Identifiant d'entitÃ© : article-12345
DonnÃ©es :
  - Titre : "Titre modifiÃ©"
Contexte :
  - Utilisateur : (non spÃ©cifiÃ©)
  - Produit : miyukini-cms
  - Instance : prod-001
```

**RÃ©sultat attendu :** DÃ©cision AMBIGUÃ‹ â€” Utilisateur manquant dans le contexte

---

## 4. Points clÃ©s sur les intentions

### 4.1. RÃ¨gles de formation

- **Identifiant unique :** Toute intention DOIT possÃ©der un identifiant unique (INV-INT-1)
- **ImmutabilitÃ© :** Une intention ne peut pas Ãªtre modifiÃ©e aprÃ¨s soumission
- **Contexte complet :** Le contexte (utilisateur, produit, instance) est obligatoire
- **Action explicite :** L'action demandÃ©e doit Ãªtre explicitement spÃ©cifiÃ©e

### 4.2. Cycle de vie

1. **SOUMISE** : Intention reÃ§ue par StrongFather
2. **EN_Ã‰VALUATION** : Ã‰valuation en cours selon les politiques
3. **DÃ‰CIDÃ‰E** : DÃ©cision produite (ACCEPTÃ‰E, REFUSÃ‰E, AMBIGUÃ‹, DIFFÃ‰RÃ‰E)

### 4.3. Ce qu'une intention N'EST PAS

- **Pas une commande d'exÃ©cution** : L'intention est Ã©valuÃ©e, jamais exÃ©cutÃ©e par StrongFather
- **Pas une requÃªte de donnÃ©es** : L'intention ne lit pas de donnÃ©es directement
- **Pas un ordre** : L'intention est une demande d'Ã©valuation, pas un ordre d'exÃ©cution

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.1 (rÃ©organisation)  
**Statut :** Illustratif et pÃ©dagogique  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, StrongFather Intent Model Contract  
**Type :** Exemples et cas d'usage

