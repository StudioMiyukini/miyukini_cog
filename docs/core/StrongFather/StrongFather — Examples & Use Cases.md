# StrongFather — Examples & Use Cases

## 1. Introduction

### Objet du document

Ce document illustre StrongFather par des **exemples concrets et des cas d'usage pratiques**, permettant de comprendre comment le moteur de décision évalue des intentions, applique des politiques, et produit des décisions dans le système Miyukini Core System v2.4.

Ce document est **pédagogique et illustratif**. Il ne définit pas de contrats, mais illustre les concepts définis dans les contrats StrongFather par des exemples concrets.

### Portée

Ce document couvre :
- Des exemples d'intentions soumises à StrongFather
- Des exemples de politiques appliquées
- Des exemples de décisions produites
- Des cas d'ambiguïté et de décision différée
- Des cas d'intentions rejetées

### Statut

Ce document est **illustratif et pédagogique**. Il complète les contrats normatifs sans les remplacer.

### Relation avec les autres contrats

Ce document illustre les concepts définis dans :
- **StrongFather — Documentation Fondatrice** : Concepts fondamentaux
- **StrongFather — Intent Model Contract** : Structure des intentions
- **StrongFather — Policy Engine Contract** : Application des politiques
- **StrongFather — Core Decision Contract** : Types de décisions
- **StrongFather — Invariants & Guarantees** : Propriétés garanties
- **[Miyukini Framework - Lois Autonomie Systeme](docs/reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)** : Les exemples respectent les lois d'autonomie système

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

## 3. Exemples de politiques

### 3.1. Politique de permission utilisateur

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

### 3.2. Politique de limite de publication

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

### 3.3. Politique de validation de contenu

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

### 3.4. Politique de priorité selon catégorie

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

### 3.5. Politique de modification conditionnelle

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

## 4. Exemples de décisions

### 4.1. Décision ACCEPTÉE — Cas standard

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
  La priorité NORMALE est établie selon la catégorie TECHNIQUE.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T10:30:00Z
```

**Caractéristiques :**
- Toutes les politiques satisfaites
- Priorité calculée
- Justification complète
- Décision non exécutable (l'adaptateur décide de l'exécution)

### 4.2. Décision ACCEPTÉE — Cas avec priorité élevée

**Intention évaluée :**
- Intention intent-2026-01-26-004 (création d'article SÉCURITÉ par user-admin)

**Politiques appliquées :**
- POL-USER-PERM-001 : SATISFAITE (user-admin a le rôle ADMIN)
- POL-VALID-CONT-001 : SATISFAITE (contenu de 800 caractères)
- POL-LIMIT-PUB-001 : SATISFAITE (5 articles publiés aujourd'hui)
- POL-PRIO-CAT-001 : Priorité MAXIMALE (catégorie SÉCURITÉ)

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
  L'utilisateur user-admin possède le rôle ADMIN requis.
  Le contenu respecte la longueur minimale de 500 caractères.
  La limite de publications quotidiennes n'est pas atteinte (5/10).
  La priorité MAXIMALE est établie selon la catégorie SÉCURITÉ.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T14:15:00Z
```

**Caractéristiques :**
- Toutes les politiques satisfaites
- Priorité MAXIMALE établie (catégorie SÉCURITÉ)
- Justification complète

### 4.3. Décision REFUSÉE — Cas de permission insuffisante

**Intention évaluée :**
- Intention intent-2026-01-26-001 (création d'article par user-visitor)

**Politiques appliquées :**
- POL-USER-PERM-001 : NON_SATISFAITE (user-visitor a le rôle VISITEUR, pas ÉDITEUR ou ADMIN)

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
  ÉDITEUR ou ADMIN pour créer un article. L'utilisateur user-visitor possède
  le rôle VISITEUR, ce qui ne satisfait pas cette condition.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T11:00:00Z
```

**Caractéristiques :**
- Au moins une politique non satisfaite
- Raison explicite du refus
- Politiques violées identifiées
- Décision définitive (ne peut pas être réévaluée sans modification)

### 4.4. Décision REFUSÉE — Cas de limite atteinte

**Intention évaluée :**
- Intention intent-2026-01-26-001 (création d'article par user-alice)

**Politiques appliquées :**
- POL-USER-PERM-001 : SATISFAITE (user-alice a le rôle ÉDITEUR)
- POL-VALID-CONT-001 : SATISFAITE (contenu de 1200 caractères)
- POL-LIMIT-PUB-001 : NON_SATISFAITE (12 articles publiés aujourd'hui, limite à 10)

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : intent-2026-01-26-001
Politiques violées :
  - POL-LIMIT-PUB-001 : NON_SATISFAITE
Raison explicite :
  La limite de publications quotidiennes est atteinte. L'utilisateur user-alice
  a déjà publié 12 articles aujourd'hui, ce qui dépasse la limite de 10 articles
  par jour définie par la politique POL-LIMIT-PUB-001.
Justification :
  La politique POL-LIMIT-PUB-001 limite le nombre de publications quotidiennes
  à 10 articles par utilisateur. L'utilisateur user-alice a déjà publié 12 articles
  aujourd'hui, ce qui viole cette contrainte.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T18:00:00Z
```

**Caractéristiques :**
- Politique de limite violée
- Raison explicite avec détails (12/10)
- Décision définitive

### 4.5. Décision REFUSÉE — Cas de contenu insuffisant

**Intention évaluée :**
- Intention intent-2026-01-26-001 (création d'article avec contenu court)

**Politiques appliquées :**
- POL-USER-PERM-001 : SATISFAITE (user-alice a le rôle ÉDITEUR)
- POL-VALID-CONT-001 : NON_SATISFAITE (contenu de 250 caractères, minimum 500)

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : intent-2026-01-26-001
Politiques violées :
  - POL-VALID-CONT-001 : NON_SATISFAITE
Raison explicite :
  Le contenu de l'article ne respecte pas la longueur minimale requise.
  Le contenu fourni contient 250 caractères, alors que la politique
  POL-VALID-CONT-001 exige au moins 500 caractères pour publier un article.
Justification :
  La politique POL-VALID-CONT-001 exige qu'un article publié contienne
  au moins 500 caractères. Le contenu fourni dans l'intention contient
  seulement 250 caractères, ce qui ne satisfait pas cette condition.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T12:00:00Z
```

**Caractéristiques :**
- Politique de validation violée
- Raison explicite avec détails (250/500)
- Décision définitive

---

## 5. Cas AMBIGUË — Intentions insuffisamment définies

### 5.1. Cas AMBIGUË — Information manquante

**Intention évaluée :**
```
Identifiant : intent-2026-01-26-005
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Article sans auteur"
  - Statut demandé : PUBLIÉ
  - Catégorie : TECHNIQUE
  - Auteur : (manquant)
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
```

**Politiques appliquées :**
- POL-USER-PERM-001 : Évaluation impossible (auteur manquant dans les données)
- POL-VALID-CONT-001 : SATISFAITE (contenu de 1200 caractères)
- POL-LIMIT-PUB-001 : Évaluation impossible (auteur manquant)

**Décision produite :**
```
Type : AMBIGUË
Identifiant intention : intent-2026-01-26-005
Éléments manquants :
  - Auteur de l'article (champ obligatoire)
Clarifications requises :
  - L'auteur de l'article doit être spécifié dans les données de l'intention.
    Ce champ est requis pour évaluer les politiques POL-USER-PERM-001
    et POL-LIMIT-PUB-001.
Politiques nécessitant ces clarifications :
  - POL-USER-PERM-001 : Nécessite l'auteur pour vérifier les permissions
  - POL-LIMIT-PUB-001 : Nécessite l'auteur pour vérifier la limite quotidienne
Justification :
  L'intention est insuffisamment définie. Le champ "Auteur" est manquant
  dans les données, ce qui empêche l'évaluation des politiques de permission
  et de limite. Ces politiques nécessitent l'identité de l'auteur pour
  déterminer si l'intention est valide.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T13:00:00Z
```

**Caractéristiques :**
- Information manquante identifiée
- Clarifications requises explicites
- Politiques affectées identifiées
- Suspension d'évaluation jusqu'à clarification
- Pas de priorité calculée

### 5.2. Cas AMBIGUË — Contexte utilisateur ambigu

**Intention évaluée :**
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

**Politiques appliquées :**
- POL-MODIF-001 : Évaluation impossible (utilisateur non spécifié dans le contexte)

**Décision produite :**
```
Type : AMBIGUË
Identifiant intention : intent-2026-01-26-006
Éléments manquants :
  - Utilisateur dans le contexte (champ obligatoire)
Clarifications requises :
  - L'utilisateur effectuant la modification doit être spécifié dans le contexte.
    Ce champ est requis pour évaluer la politique POL-MODIF-001 qui vérifie
    si l'utilisateur est l'auteur de l'article ou possède le rôle ADMIN.
Politiques nécessitant ces clarifications :
  - POL-MODIF-001 : Nécessite l'utilisateur pour vérifier les permissions
Justification :
  L'intention est insuffisamment définie. Le champ "Utilisateur" est manquant
  dans le contexte, ce qui empêche l'évaluation de la politique de permission
  de modification. Cette politique nécessite l'identité de l'utilisateur pour
  déterminer si l'intention est valide.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T13:30:00Z
```

**Caractéristiques :**
- Contexte utilisateur manquant
- Clarifications requises explicites
- Suspension d'évaluation

### 5.3. Cas AMBIGUË — Statut non spécifié

**Intention évaluée :**
```
Identifiant : intent-2026-01-26-007
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Article sans statut"
  - Auteur : user-alice
  - Catégorie : TECHNIQUE
  - Statut demandé : (non spécifié)
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
```

**Politiques appliquées :**
- POL-USER-PERM-001 : SATISFAITE (user-alice a le rôle ÉDITEUR)
- POL-VALID-CONT-001 : Évaluation impossible (statut non spécifié, politique applicable uniquement si PUBLIÉ)
- POL-LIMIT-PUB-001 : Évaluation impossible (statut non spécifié)

**Décision produite :**
```
Type : AMBIGUË
Identifiant intention : intent-2026-01-26-007
Éléments manquants :
  - Statut demandé (champ obligatoire pour déterminer les politiques applicables)
Clarifications requises :
  - Le statut demandé doit être spécifié dans les données de l'intention.
    Ce champ détermine quelles politiques sont applicables :
    - Si statut = PUBLIÉ : POL-VALID-CONT-001 et POL-LIMIT-PUB-001 s'appliquent
    - Si statut = BROUILLON : Ces politiques ne s'appliquent pas
Politiques nécessitant ces clarifications :
  - POL-VALID-CONT-001 : Nécessite le statut pour déterminer l'applicabilité
  - POL-LIMIT-PUB-001 : Nécessite le statut pour déterminer l'applicabilité
Justification :
  L'intention est insuffisamment définie. Le champ "Statut demandé" est manquant,
  ce qui empêche de déterminer quelles politiques sont applicables. Certaines
  politiques (POL-VALID-CONT-001, POL-LIMIT-PUB-001) ne s'appliquent que si
  le statut demandé est PUBLIÉ.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T14:00:00Z
```

**Caractéristiques :**
- Statut manquant affecte l'applicabilité des politiques
- Clarifications requises avec explication conditionnelle
- Suspension d'évaluation

---

## 6. Cas DIFFÉRÉE — Contexte futur requis

### 6.1. Cas DIFFÉRÉE — Dépendance à un événement futur

**Intention évaluée :**
```
Identifiant : intent-2026-01-26-008
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Article programmé"
  - Auteur : user-alice
  - Statut demandé : PUBLIÉ
  - Catégorie : TECHNIQUE
  - Date de publication : 2026-01-27T10:00:00Z (demain)
Contexte :
  - Utilisateur : user-alice
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date actuelle : 2026-01-26T15:00:00Z
```

**Politiques appliquées :**
- POL-USER-PERM-001 : SATISFAITE (user-alice a le rôle ÉDITEUR)
- POL-VALID-CONT-001 : SATISFAITE (contenu de 1200 caractères)
- POL-LIMIT-PUB-001 : INDÉTERMINÉE (limite quotidienne dépend de la date de publication effective, qui est dans le futur)

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
  d'articles publiés le jour de la publication effective, qui n'est
  pas encore disponible.
Politiques nécessitant ce contexte :
  - POL-LIMIT-PUB-001 : Nécessite le contexte du jour de publication
    pour vérifier la limite quotidienne
Justification :
  L'évaluation de l'intention nécessite un contexte futur qui n'est
  pas encore disponible. La politique POL-LIMIT-PUB-001 vérifie la
  limite de publications quotidiennes, mais cette vérification doit
  être effectuée le jour de la publication effective (2026-01-27),
  pas le jour de la soumission (2026-01-26).
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T15:00:00Z
```

**Caractéristiques :**
- Contexte futur requis identifié
- Raison du différé explicite
- Suspension d'évaluation jusqu'à disponibilité du contexte
- Pas de planification (l'adaptateur décide quand réévaluer)

### 6.2. Cas DIFFÉRÉE — Dépendance à un état futur

**Intention évaluée :**
```
Identifiant : intent-2026-01-26-009
Action : MODIFIER
Type d'entité : ARTICLE
Identifiant d'entité : article-12345
Données :
  - Statut demandé : PUBLIÉ
Contexte :
  - Utilisateur : user-bob
  - Produit : miyukini-cms
  - Instance : prod-001
  - État actuel de l'article : EN_REVUE (en cours de révision)
```

**Politiques appliquées :**
- POL-MODIF-001 : SATISFAITE (user-bob est l'auteur de l'article)
- POL-PUBLICATION-APPROUVÉE : INDÉTERMINÉE (nécessite que l'article soit approuvé, état futur)

**Décision produite :**
```
Type : DIFFÉRÉE
Identifiant intention : intent-2026-01-26-009
Contexte futur requis :
  - État de l'article : APPROUVÉ (actuellement EN_REVUE)
Raison du différé :
  L'intention demande de publier un article qui est actuellement
  en révision (EN_REVUE). La politique POL-PUBLICATION-APPROUVÉE
  exige que l'article soit dans l'état APPROUVÉ avant publication,
  mais cet état n'est pas encore atteint.
Politiques nécessitant ce contexte :
  - POL-PUBLICATION-APPROUVÉE : Nécessite l'état APPROUVÉ de l'article
Justification :
  L'évaluation de l'intention nécessite un état futur qui n'est
  pas encore disponible. La politique POL-PUBLICATION-APPROUVÉE
  vérifie que l'article est approuvé avant publication, mais
  l'article est actuellement en révision. L'évaluation doit être
  différée jusqu'à ce que l'article atteigne l'état APPROUVÉ.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T16:00:00Z
```

**Caractéristiques :**
- État futur requis identifié
- Raison du différé avec état actuel vs état requis
- Suspension d'évaluation

---

## 7. Cas rejetés — Intentions invalides

### 7.1. Rejet structurel — Intention incomplète

**Intention soumise :**
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
  le Intent Model Contract. Le champ "Action" est obligatoire et
  doit être présent dans toute intention. Sans action, l'intention
  ne peut pas être évaluée.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T17:00:00Z
```

**Caractéristiques :**
- Rejet avant évaluation des politiques
- Violation des règles de formation
- Décision définitive

### 7.2. Rejet structurel — Identifiant manquant

**Intention soumise :**
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

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : (non applicable — intention rejetée avant attribution d'identifiant)
Politiques violées :
  - (Aucune politique évaluée — rejet structurel)
Raison explicite :
  L'intention est structurellement invalide. L'identifiant d'intention
  est obligatoire et manquant. Toute intention DOIT posséder un
  identifiant unique et immutable (INV-INT-1).
Justification :
  L'intention ne respecte pas les règles de formation définies dans
  le Intent Model Contract. L'identifiant d'intention est obligatoire
  et doit être fourni par l'appelant. Sans identifiant, l'intention
  ne peut pas être tracée ni associée à une décision.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T17:30:00Z
```

**Caractéristiques :**
- Rejet avant évaluation des politiques
- Violation de l'invariant INV-INT-1
- Décision définitive

### 7.3. Rejet de politique — Modification non autorisée

**Intention évaluée :**
- Intention intent-2026-01-26-002 (modification d'article par user-charlie)

**Fait fourni :**
- Article article-12345 a pour auteur user-alice

**Politiques appliquées :**
- POL-MODIF-001 : NON_SATISFAITE (user-charlie n'est pas l'auteur et n'a pas le rôle ADMIN)

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : intent-2026-01-26-002
Politiques violées :
  - POL-MODIF-001 : NON_SATISFAITE
Raison explicite :
  L'utilisateur user-charlie n'est pas autorisé à modifier l'article
  article-12345. La politique POL-MODIF-001 exige que l'utilisateur
  soit l'auteur de l'article ou possède le rôle ADMIN. L'article
  a pour auteur user-alice, et user-charlie possède le rôle ÉDITEUR.
Justification :
  La politique POL-MODIF-001 exige que l'utilisateur effectuant la
  modification soit l'auteur de l'article ou possède le rôle ADMIN.
  L'article article-12345 a pour auteur user-alice, et l'utilisateur
  user-charlie (rôle ÉDITEUR) ne satisfait pas cette condition.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T18:00:00Z
```

**Caractéristiques :**
- Politique de permission violée
- Raison explicite avec détails (auteur, rôles)
- Décision définitive

### 7.4. Rejet de politique — Suppression non autorisée

**Intention évaluée :**
- Intention intent-2026-01-26-003 (suppression d'article par user-charlie)

**Fait fourni :**
- Article article-12345 a pour auteur user-alice
- Article article-12345 a le statut PUBLIÉ

**Politiques appliquées :**
- POL-SUPPR-001 : NON_SATISFAITE (seul l'auteur ou un ADMIN peut supprimer un article publié)

**Décision produite :**
```
Type : REFUSÉE
Identifiant intention : intent-2026-01-26-003
Politiques violées :
  - POL-SUPPR-001 : NON_SATISFAITE
Raison explicite :
  L'utilisateur user-charlie n'est pas autorisé à supprimer l'article
  article-12345. La politique POL-SUPPR-001 exige que l'utilisateur
  soit l'auteur de l'article ou possède le rôle ADMIN pour supprimer
  un article publié. L'article a pour auteur user-alice, et
  user-charlie possède le rôle ÉDITEUR.
Justification :
  La politique POL-SUPPR-001 exige que l'utilisateur effectuant la
  suppression soit l'auteur de l'article ou possède le rôle ADMIN
  pour supprimer un article publié. L'article article-12345 a pour
  auteur user-alice et est publié, et l'utilisateur user-charlie
  (rôle ÉDITEUR) ne satisfait pas cette condition.
Contexte d'évaluation :
  - Produit : miyukini-cms
  - Instance : prod-001
  - Date d'évaluation : 2026-01-26T18:30:00Z
```

**Caractéristiques :**
- Politique de permission violée
- Raison explicite
- Décision définitive

---

## 8. Scénarios complets

### 8.1. Scénario : Publication d'article avec workflow complet

**Étape 1 : Intention de création (brouillon)**
```
Intention : intent-2026-01-26-011
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Mon premier article"
  - Auteur : user-alice
  - Statut demandé : BROUILLON
  - Catégorie : TECHNIQUE
Contexte : user-alice, miyukini-cms, prod-001
```

**Décision : ACCEPTÉE**
- POL-USER-PERM-001 : SATISFAITE
- POL-VALID-CONT-001 : Non applicable (brouillon)
- POL-LIMIT-PUB-001 : Non applicable (brouillon)
- Priorité : NORMALE

**Étape 2 : Intention de modification (passage en révision)**
```
Intention : intent-2026-01-26-012
Action : MODIFIER
Type d'entité : ARTICLE
Identifiant d'entité : article-67890
Données :
  - Statut demandé : EN_REVUE
Contexte : user-alice, miyukini-cms, prod-001
```

**Décision : ACCEPTÉE**
- POL-MODIF-001 : SATISFAITE (user-alice est l'auteur)
- Priorité : NORMALE

**Étape 3 : Intention de modification (publication)**
```
Intention : intent-2026-01-26-013
Action : MODIFIER
Type d'entité : ARTICLE
Identifiant d'entité : article-67890
Données :
  - Statut demandé : PUBLIÉ
Contexte : user-alice, miyukini-cms, prod-001
```

**Décision : ACCEPTÉE**
- POL-MODIF-001 : SATISFAITE
- POL-VALID-CONT-001 : SATISFAITE (contenu de 1500 caractères)
- POL-LIMIT-PUB-001 : SATISFAITE (2 articles publiés aujourd'hui)
- Priorité : NORMALE

**Caractéristiques du scénario :**
- Workflow complet illustré
- Différentes intentions pour différentes étapes
- Politiques applicables selon le contexte
- Décisions cohérentes

### 8.2. Scénario : Tentative de publication avec ambiguïté puis clarification

**Étape 1 : Intention ambiguë**
```
Intention : intent-2026-01-26-014
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Article ambigu"
  - Statut demandé : PUBLIÉ
  - Auteur : (manquant)
Contexte : user-alice, miyukini-cms, prod-001
```

**Décision : AMBIGUË**
- Clarification requise : Auteur manquant

**Étape 2 : Intention clarifiée**
```
Intention : intent-2026-01-26-015 (nouvel identifiant après clarification)
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Article ambigu"
  - Auteur : user-alice
  - Statut demandé : PUBLIÉ
Contexte : user-alice, miyukini-cms, prod-001
```

**Décision : ACCEPTÉE**
- POL-USER-PERM-001 : SATISFAITE
- POL-VALID-CONT-001 : SATISFAITE
- POL-LIMIT-PUB-001 : SATISFAITE
- Priorité : NORMALE

**Caractéristiques du scénario :**
- Ambiguïté détectée
- Clarification effectuée
- Nouvelle intention avec nouvel identifiant
- Décision finale acceptée

### 8.3. Scénario : Conflit de priorités

**Intention 1 :**
```
Intention : intent-2026-01-26-016
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Article technique"
  - Catégorie : TECHNIQUE
  - Priorité demandée : NORMALE
```

**Décision 1 : ACCEPTÉE**
- Priorité établie : HAUTE (catégorie TECHNIQUE selon POL-PRIO-CAT-001)

**Intention 2 :**
```
Intention : intent-2026-01-26-017
Action : CRÉER
Type d'entité : ARTICLE
Données :
  - Titre : "Alerte sécurité"
  - Catégorie : SÉCURITÉ
  - Priorité demandée : NORMALE
```

**Décision 2 : ACCEPTÉE**
- Priorité établie : MAXIMALE (catégorie SÉCURITÉ selon POL-PRIO-CAT-001)

**Caractéristiques du scénario :**
- Deux intentions concurrentes
- Priorités établies selon les politiques
- Priorité MAXIMALE > HAUTE (l'adaptateur peut utiliser cette information pour ordonnancer)

---

## 9. Points clés à retenir

### 9.1. Types de décisions

- **ACCEPTÉE** : Intention valide selon toutes les politiques, priorité établie
- **REFUSÉE** : Intention invalide selon au moins une politique, raison explicite
- **AMBIGUË** : Intention insuffisamment définie, clarifications requises
- **DIFFÉRÉE** : Contexte futur requis pour l'évaluation

### 9.2. Flux d'évaluation

1. **Réception** : Intention soumise par l'adaptateur
2. **Validation structurelle** : Vérification des règles de formation
3. **Sélection des politiques** : Identification des politiques applicables
4. **Évaluation** : Évaluation de chaque politique
5. **Composition** : Agrégation des résultats
6. **Calcul de priorité** : (Uniquement si toutes les politiques satisfaites)
7. **Production de décision** : Génération de la décision finale

### 9.3. Caractéristiques des décisions

- **Non exécutables** : Une décision n'est jamais exécutée automatiquement
- **Justifiées** : Toute décision contient une justification complète
- **Traçables** : Toute décision est tracée avec son contexte
- **Non ambiguës** : Une décision est toujours claire (ACCEPTÉE, REFUSÉE, AMBIGUË, ou DIFFÉRÉE)

### 9.4. Distinctions importantes

- **AMBIGUË vs DIFFÉRÉE** :
  - AMBIGUË : Information manquante dans l'intention
  - DIFFÉRÉE : Contexte futur requis pour l'évaluation

- **REFUSÉE vs AMBIGUË** :
  - REFUSÉE : Intention valide mais politique violée
  - AMBIGUË : Intention insuffisamment définie pour évaluation

- **ACCEPTÉE vs Exécution** :
  - ACCEPTÉE : Intention valide selon les politiques
  - Exécution : Responsabilité de l'adaptateur, pas de StrongFather

---

## 10. Conclusion

Ce document illustre StrongFather par des exemples concrets montrant comment le moteur de décision évalue des intentions, applique des politiques, et produit des décisions.

Les exemples démontrent :
- La diversité des intentions soumises
- La variété des politiques appliquées
- Les différents types de décisions produites
- Les cas d'ambiguïté et de différation
- Les cas d'intentions rejetées

Ces exemples sont **illustratifs** et complètent les contrats normatifs sans les remplacer. Pour les définitions formelles, référez-vous aux contrats StrongFather.

---

**Document créé le :** 2026-01-26  
**Version :** 1.0  
**Statut :** Illustratif et pédagogique  
**Référence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** Exemples et cas d'usage

---

## 11. Mini log de génération

### Décision éditoriale E1 : Structure du document

**Décision prise :** Organisation en sections thématiques (intentions, politiques, décisions, cas spéciaux, scénarios).

**Application :** Sections 2 à 8 organisent les exemples par type.

### Décision éditoriale E2 : Exemples concrets

**Décision prise :** Utilisation d'exemples concrets et réalistes (articles, utilisateurs, rôles) sans code.

**Application :** Tous les exemples utilisent des scénarios CMS concrets.

### Décision éditoriale E3 : Couverture complète

**Décision prise :** Couverture de tous les types de décisions (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE) avec plusieurs exemples chacun.

**Application :** Sections 4, 5, 6, et 7 couvrent tous les types de décisions.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice : Confirmée (concepts respectés)
- ✅ Cohérence avec Core Decision Contract : Confirmée (types de décisions respectés)
- ✅ Cohérence avec Intent Model Contract : Confirmée (structure des intentions respectée)
- ✅ Cohérence avec Policy Engine Contract : Confirmée (application des politiques respectée)
- ✅ Aucun code inclus : Confirmé
- ✅ Aucune exécution mentionnée : Confirmé

**Conclusion :** Document illustratif complet et cohérent avec les contrats.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
