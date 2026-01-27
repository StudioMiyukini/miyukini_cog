# StrongFather — Examples Intentions

## Contexte

Ce document illustre StrongFather par des **exemples d'intentions** soumises au moteur de décision. Il fait partie de la série de documents d'exemples.

**Documents connexes :**
- [StrongFather - Examples Policies](./StrongFather%20-%20Examples%20Policies.md)
- [StrongFather - Examples Decisions](./StrongFather%20-%20Examples%20Decisions.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 1. Structure d'une intention

Une intention est une demande d'action soumise à StrongFather pour évaluation. Elle contient :

**Composants obligatoires :**
- Identifiant unique
- Action (CRÉER, MODIFIER, SUPPRIMER, LECTURE, ÉVALUATION)
- Type d'entité
- Contexte (utilisateur, produit, instance)

**Composants optionnels :**
- Données associées
- Priorité demandée
- Contraintes explicites
- Métadonnées

---

## 2. Exemples d'intentions

### 2.1. Intention de création de contenu

**Contexte :**
Un adaptateur produit soumet une intention de création d'un article de blog.

**Intention :**
```
Identifiant : intent-2026-01-26-001
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Introduction à StrongFather"
  - Auteur : user-alice
  - Statut demandé : PUBLIÉ
  - Catégorie : TECHNIQUE
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
Métadonnées :
  - Priorité demandée : NORMALE
  - Contraintes : aucune
```

**Caractéristiques :**
- Intention structurellement valide
- Tous les champs obligatoires présents
- Contexte complet fourni
- Action et type d'entité explicites

---

### 2.2. Intention de modification de contenu

**Contexte :**
Un adaptateur produit soumet une intention de modification d'un article existant.

**Intention :**
```
Identifiant : intent-2026-01-26-002
Action : MODIFIER
Type d'entité : ARTICLE
Identifiant d'entité : article-12345
Données :
  - Titre : "Introduction à StrongFather — Version mise à jour"
  - Statut demandé : PUBLIÉ
Contexte :
  - Utilisateur : user-bob
  - Produit : miyukini-cms
  - Instance : prod-001
Métadonnées :
  - Priorité demandée : HAUTE
  - Contraintes : aucune
```

**Caractéristiques :**
- Intention structurellement valide
- Référence à une entité existante
- Modification partielle (seulement titre et statut)

---

### 2.3. Intention de suppression de contenu

**Contexte :**
Un adaptateur produit soumet une intention de suppression d'un article.

**Intention :**
```
Identifiant : intent-2026-01-26-003
Action : SUPPRIMER
Type d'entité : ARTICLE
Identifiant d'entité : article-12345
Contexte :
  - Utilisateur : user-charlie
  - Produit : miyukini-cms
  - Instance : prod-001
Métadonnées :
  - Priorité demandée : NORMALE
  - Contraintes : aucune
```

**Caractéristiques :**
- Intention structurellement valide
- Action de suppression
- Pas de données de modification (non applicable)

---

### 2.4. Intention avec priorité élevée

**Contexte :**
Un adaptateur produit soumet une intention avec priorité élevée pour une publication urgente.

**Intention :**
```
Identifiant : intent-2026-01-26-004
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Alerte sécurité — Mise à jour critique"
  - Auteur : user-admin
  - Statut demandé : PUBLIÉ
  - Catégorie : SÉCURITÉ
Contexte :
  - Utilisateur : user-admin
  - Produit : miyukini-cms
  - Instance : prod-001
Métadonnées :
  - Priorité demandée : URGENTE
  - Contraintes : aucune
```

**Caractéristiques :**
- Intention structurellement valide
- Priorité URGENTE demandée
- Catégorie SÉCURITÉ

---

## 3. Intentions invalides (exemples de rejets structurels)

### 3.1. Intention sans action

```
Identifiant : intent-2026-01-26-010
Action : (manquant)
Type d'entité : ARTICLE
Données :
  - Titre : "Article incomplet"
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
```

**Résultat attendu :** Rejet structurel — Action obligatoire manquante

---

### 3.2. Intention sans identifiant

```
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Article sans identifiant"
  - Auteur : user-alice
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
```

**Résultat attendu :** Rejet structurel — Identifiant obligatoire manquant (INV-INT-1)

---

### 3.3. Intention sans contexte utilisateur

```
Identifiant : intent-2026-01-26-006
Action : MODIFIER
Type d'entité : ARTICLE
Identifiant d'entité : article-12345
Données :
  - Titre : "Titre modifié"
Contexte :
  - Utilisateur : (non spécifié)
  - Produit : miyukini-cms
  - Instance : prod-001
```

**Résultat attendu :** Décision AMBIGUË — Utilisateur manquant dans le contexte

---

## 4. Points clés sur les intentions

### 4.1. Règles de formation

- **Identifiant unique :** Toute intention DOIT posséder un identifiant unique (INV-INT-1)
- **Immutabilité :** Une intention ne peut pas être modifiée après soumission
- **Contexte complet :** Le contexte (utilisateur, produit, instance) est obligatoire
- **Action explicite :** L'action demandée doit être explicitement spécifiée

### 4.2. Cycle de vie

1. **SOUMISE** : Intention reçue par StrongFather
2. **EN_ÉVALUATION** : Évaluation en cours selon les politiques
3. **DÉCIDÉE** : Décision produite (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE)

### 4.3. Ce qu'une intention N'EST PAS

- **Pas une commande d'exécution** : L'intention est évaluée, jamais exécutée par StrongFather
- **Pas une requête de données** : L'intention ne lit pas de données directement
- **Pas un ordre** : L'intention est une demande d'évaluation, pas un ordre d'exécution

---

**Document créé le :** 2026-01-27  
**Version :** 1.1 (réorganisation)  
**Statut :** Illustratif et pédagogique  
**Référence :** Miyukini Core System v2.4, StrongFather Intent Model Contract  
**Type :** Exemples et cas d'usage
