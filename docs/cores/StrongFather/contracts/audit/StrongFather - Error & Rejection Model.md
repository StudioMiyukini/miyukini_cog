# StrongFather — Error & Rejection Model

## 1. Introduction

### Objet du contrat

Ce document définit le **StrongFather — Error & Rejection Model** : un contrat normatif, non négociable, et de statut FONDATION qui établit le modèle conceptuel des erreurs et des rejets dans StrongFather, définissant comment les erreurs sont représentées, comment les rejets sont communiqués, les catégories d'erreurs, et les règles de gestion des situations exceptionnelles dans le système Miyukini Core System v2.4.

Ce contrat précise la nature des erreurs dans StrongFather, la distinction entre erreur et rejet, les catégories de rejet, et les garanties associées.

### Portée

Ce contrat s'applique à **toutes les situations d'erreur et de rejet dans StrongFather** et définit de manière absolue :
- la définition formelle d'une erreur StrongFather,
- la distinction entre erreur et rejet,
- les catégories d'erreurs et de rejets,
- la structure des messages d'erreur et de rejet,
- les règles de propagation,
- les invariants de gestion d'erreur.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **StrongFather — Documentation Fondatrice** : Définition philosophique de StrongFather
- **StrongFather — Core Decision Contract** : Les décisions refusées sont formalisées ici
- **StrongFather — Intent Model Contract** : Les intentions invalides produisent des rejets
- **StrongFather — Policy Engine Contract** : Les politiques non satisfaites produisent des rejets
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité aux lois d'autonomie, notamment **LOI-2** (le système accepte l'isolement comme état normal) : les erreurs ne bloquent jamais le système en attente d'une ressource externe

Il n'introduit aucune contradiction, et constitue la définition formelle de la gestion des erreurs et rejets dans StrongFather.

---

## 2. Distinction erreur/rejet

### 2.1. Définition d'une erreur

Une **erreur** dans StrongFather est une situation anormale qui empêche le fonctionnement correct du moteur de décision. Une erreur représente un dysfonctionnement interne, pas un résultat d'évaluation.

**Caractéristiques d'une erreur :**

- **Interne** : L'erreur provient du moteur de décision, pas de l'intention
- **Inattendue** : L'erreur n'est pas un résultat d'évaluation prévisible
- **Bloquante** : L'erreur empêche la production d'une décision
- **Technique** : L'erreur concerne le fonctionnement technique (conceptuel dans ce contrat)

### 2.2. Définition d'un rejet

Un **rejet** dans StrongFather est le résultat d'une évaluation qui détermine qu'une intention ne peut pas être acceptée. Un rejet est un résultat normal de l'évaluation, pas un dysfonctionnement.

**Caractéristiques d'un rejet :**

- **Externe** : Le rejet est causé par l'intention ou son contexte
- **Prévisible** : Le rejet est un résultat d'évaluation selon les politiques
- **Non-bloquant** : Le rejet produit une décision (REFUSÉE, AMBIGUË, DIFFÉRÉE)
- **Conceptuel** : Le rejet concerne la validité stratégique et politique

### 2.3. Distinction fondamentale

| Aspect | Erreur | Rejet |
|--------|--------|-------|
| Source | Interne à StrongFather | Externe (intention, contexte) |
| Nature | Dysfonctionnement | Résultat d'évaluation |
| Résultat | Pas de décision produite | Décision produite (refusée, ambiguë, différée) |
| Traitement | Correction technique requise | Information à l'appelant |
| Traçabilité | Log d'erreur | Décision avec justification |

---

## 3. Catégories d'erreurs

### 3.1. Erreurs de structure

**Définition :**

Les **erreurs de structure** sont des erreurs causées par une incohérence ou une malformation dans la structure des données internes de StrongFather.

**Exemples conceptuels :**

- Politique malformée dans le moteur
- Règle de composition incohérente
- Référence circulaire dans les politiques

**Gravité :** Critique

**Conséquence :** Arrêt de l'évaluation, signalement d'erreur interne

### 3.2. Erreurs de cohérence

**Définition :**

Les **erreurs de cohérence** sont des erreurs causées par une violation des invariants internes de StrongFather.

**Exemples conceptuels :**

- Violation d'un invariant de politique
- Incohérence dans l'état du moteur
- Contradiction détectée dans les règles

**Gravité :** Critique

**Conséquence :** Arrêt de l'évaluation, signalement d'erreur de cohérence

### 3.3. Erreurs de ressource

**Définition :**

Les **erreurs de ressource** sont des erreurs causées par l'indisponibilité de ressources nécessaires à l'évaluation.

**Exemples conceptuels :**

- Politiques non disponibles
- Contexte d'évaluation incomplet côté moteur
- Capacité d'évaluation dépassée

**Gravité :** Haute

**Conséquence :** Échec de l'évaluation, possibilité de réessai

---

## 4. Catégories de rejets

### 4.1. Rejet structurel

**Définition :**

Un **rejet structurel** se produit lorsque l'intention soumise est structurellement invalide selon les règles de formation définies dans le Intent Model Contract.

**Causes :**

- Absence d'un composant obligatoire
- Type d'action non reconnu
- Structure de l'intention incohérente
- Contexte d'appel incomplet

**Décision produite :** REFUSÉE

**Contenu du rejet :**

- Type de rejet : STRUCTUREL
- Composants manquants ou invalides
- Règles de formation violées
- Aucune politique n'est évaluée (rejet avant évaluation)

### 4.2. Rejet de contenu

**Définition :**

Un **rejet de contenu** se produit lorsque l'intention contient des éléments interdits selon le Intent Model Contract.

**Causes :**

- Présence de commandes d'exécution
- Présence de logique temporelle technique
- Présence d'appels système
- Contenu ambigu ou contradictoire

**Décision produite :** REFUSÉE

**Contenu du rejet :**

- Type de rejet : CONTENU
- Éléments interdits identifiés
- Règles de contenu violées
- Aucune politique n'est évaluée (rejet avant évaluation)

### 4.3. Rejet de politique

**Définition :**

Un **rejet de politique** se produit lorsque l'intention est structurellement valide mais viole une ou plusieurs politiques.

**Causes :**

- Politique de permission non satisfaite
- Politique de contrainte violée
- Politique de validation échouée
- Politique de dépendance non respectée

**Décision produite :** REFUSÉE

**Contenu du rejet :**

- Type de rejet : POLITIQUE
- Politiques violées (identifiants et descriptions)
- Résultats d'évaluation par politique
- Justification détaillée du rejet

### 4.4. Rejet pour ambiguïté

**Définition :**

Un **rejet pour ambiguïté** se produit lorsque l'intention ne peut pas être évaluée complètement car des informations sont manquantes ou insuffisamment définies.

**Causes :**

- Éléments de l'intention insuffisamment définis
- Contexte insuffisant pour certaines politiques
- Clarifications nécessaires pour l'évaluation

**Décision produite :** AMBIGUË

**Contenu du rejet :**

- Type de rejet : AMBIGUÏTÉ
- Éléments manquants ou insuffisants
- Clarifications requises
- Politiques nécessitant ces clarifications

**Particularités :**

- **Suspension d'évaluation** : L'évaluation ultérieure de l'intention est suspendue jusqu'à clarification
- **Pas de calcul de priorité** : Aucune priorité ne peut être calculée pour une intention ambiguë
- **Non-définitif** : L'ambiguïté n'est pas un refus définitif ; l'intention peut être clarifiée et réévaluée

### 4.5. Rejet pour contexte futur

**Définition :**

Un **rejet pour contexte futur** se produit lorsque l'intention dépend d'un contexte qui n'est pas encore disponible.

**Causes :**

- Dépendance à un événement futur
- Dépendance à un état non encore atteint
- Contexte requis non disponible

**Décision produite :** DIFFÉRÉE

**Contenu du rejet :**

- Type de rejet : CONTEXTE_FUTUR
- Contexte futur requis
- Raison de la différation
- Politiques nécessitant ce contexte

**Particularités :**

- **Distinction avec ambiguïté** : L'ambiguïté concerne des informations manquantes dans l'intention ; le contexte futur concerne des informations qui n'existent pas encore dans le système
- **Réévaluation possible** : Une fois le contexte disponible, l'intention peut être réévaluée

---

## 5. Structure des messages d'erreur

### 5.1. Composants obligatoires

Tout message d'erreur DOIT contenir :

**Identifiant d'erreur :**

Un identifiant unique permettant de référencer l'erreur.

**Catégorie d'erreur :**

La catégorie de l'erreur (STRUCTURE, COHÉRENCE, RESSOURCE).

**Description :**

Une description conceptuelle de l'erreur.

**Contexte d'erreur :**

Le contexte dans lequel l'erreur s'est produite.

### 5.2. Composants optionnels

Les composants suivants sont optionnels :

**Cause racine :**

La cause conceptuelle identifiée de l'erreur.

**Recommandation :**

Une recommandation conceptuelle pour résoudre l'erreur.

**Références :**

Des références vers des documents ou des contrats pertinents.

---

## 6. Structure des messages de rejet

### 6.1. Composants obligatoires

Tout message de rejet DOIT contenir :

**Identifiant de l'intention :**

L'identifiant de l'intention rejetée.

**Type de décision :**

Le type de décision (REFUSÉE, AMBIGUË, DIFFÉRÉE).

**Type de rejet :**

La catégorie de rejet (STRUCTUREL, CONTENU, POLITIQUE, AMBIGUÏTÉ, CONTEXTE_FUTUR).

**Justification :**

La justification détaillée du rejet.

**Contexte d'évaluation :**

Le contexte utilisé pour l'évaluation.

### 6.2. Composants spécifiques par type

**Pour rejet STRUCTUREL :**

- Composants manquants
- Règles de formation violées

**Pour rejet CONTENU :**

- Éléments interdits identifiés
- Règles de contenu violées

**Pour rejet POLITIQUE :**

- Politiques violées (identifiants, descriptions)
- Résultats d'évaluation par politique

**Pour rejet AMBIGUÏTÉ :**

- Éléments manquants ou insuffisants
- Clarifications requises
- Politiques nécessitant ces clarifications

**Pour rejet CONTEXTE_FUTUR :**

- Contexte futur requis
- Raison de la différation
- Politiques nécessitant ce contexte

---

## 7. Règles de propagation

### 7.1. Propagation des erreurs

**R-PROP-ERR-1 : Non-absorption**

Les erreurs ne sont jamais absorbées silencieusement. Toute erreur doit être signalée.

**R-PROP-ERR-2 : Remontée**

Les erreurs sont remontées à l'appelant avec leur contexte complet.

**R-PROP-ERR-3 : Pas de transformation en rejet**

Une erreur ne peut jamais être transformée en rejet. Les erreurs et les rejets sont distincts.

**R-PROP-ERR-4 : Arrêt d'évaluation**

Une erreur arrête l'évaluation. Aucune décision n'est produite suite à une erreur.

### 7.2. Propagation des rejets

**R-PROP-REJ-1 : Décision produite**

Un rejet produit toujours une décision (REFUSÉE, AMBIGUË, ou DIFFÉRÉE).

**R-PROP-REJ-2 : Justification complète**

Un rejet est toujours accompagné d'une justification complète.

**R-PROP-REJ-3 : Traçabilité**

Un rejet est toujours traçable avec les politiques évaluées et les résultats.

**R-PROP-REJ-4 : Non-blocage**

Un rejet ne bloque pas StrongFather. D'autres intentions peuvent être évaluées.

---

## 8. Invariants de gestion d'erreur

### 8.1. Invariants de distinction

**INV-ERR-1 : Distinction erreur/rejet**

Toute situation est soit une erreur, soit un rejet, jamais les deux. La distinction est absolue.

**INV-ERR-2 : Erreur sans décision**

Une erreur ne produit jamais de décision. Les erreurs et les décisions sont mutuellement exclusives.

**INV-ERR-3 : Rejet avec décision**

Un rejet produit toujours une décision. Pas de rejet sans décision associée.

### 8.2. Invariants de traçabilité

**INV-ERR-4 : Traçabilité des erreurs**

Toute erreur est traçable avec son contexte et sa cause.

**INV-ERR-5 : Traçabilité des rejets**

Tout rejet est traçable avec les politiques évaluées et les résultats.

### 8.3. Invariants de comportement

**INV-ERR-6 : Pas d'effet de bord sur erreur**

Une erreur ne produit jamais d'effet de bord sur le système.

**INV-ERR-7 : Pas d'effet de bord sur rejet**

Un rejet ne produit jamais d'effet de bord sur le système (conformément au Execution Prohibition Contract).

---

## 9. Règles de fermeture du contrat

### 9.1. Contrat fermé

Ce contrat est **fermé**. Seules les catégories d'erreurs, les catégories de rejets, les structures, et les règles explicitement définies dans ce contrat sont autorisées.

### 9.2. Interdiction d'extension implicite

Aucune extension implicite de ce contrat n'est autorisée :

- **INTERD-ERR-1** : Aucune catégorie d'erreur non définie n'est reconnue
- **INTERD-ERR-2** : Aucune catégorie de rejet non définie n'est reconnue
- **INTERD-ERR-3** : Aucune règle de propagation non définie n'est applicable
- **INTERD-ERR-4** : Aucun invariant non défini n'est garanti

---

## 10. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable le modèle d'erreurs et de rejets de StrongFather.

Il garantit que :
- la distinction erreur/rejet est absolue,
- les catégories d'erreurs et de rejets sont définies et fermées,
- les structures de messages sont standardisées,
- les règles de propagation sont explicites,
- les invariants sont respectés,
- le contrat est fermé et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 11. Validation conceptuelle

### 11.1. Cas valides

Les cas suivants sont **valides** selon ce contrat :

1. **Rejet structurel** : Une intention sans identifiant produit une décision REFUSÉE de type STRUCTUREL.

2. **Rejet de politique** : Une intention valide mais violant une politique produit une décision REFUSÉE de type POLITIQUE.

3. **Rejet pour ambiguïté** : Une intention avec contexte insuffisant produit une décision AMBIGUË de type AMBIGUÏTÉ.

### 11.2. Cas de violation

Les cas suivants **violent** ce contrat :

1. **Erreur transformée en rejet** : Une erreur de structure transformée en décision REFUSÉE. Viole R-PROP-ERR-3.

2. **Rejet sans décision** : Un rejet qui ne produit pas de décision. Viole INV-ERR-3.

3. **Erreur absorbée** : Une erreur qui n'est pas signalée. Viole R-PROP-ERR-1.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** Contrat de modèle d'erreur et de rejet non négociable

---

## 12. Mini log de génération

### Warning W1 : Distinction erreur/rejet

**Warning rencontré :** Risque de confusion entre erreur et rejet.

**Décision prise :** Section 2 entièrement dédiée à la distinction avec tableau comparatif et caractéristiques explicites.

**Correction effectuée :** Section 2 rédigée avec distinction claire et invariants INV-ERR-1, INV-ERR-2, INV-ERR-3.

### Warning W2 : Ambiguïté et suspension

**Warning rencontré :** La Documentation Fondatrice mentionne que les détails de l'ambiguïté seront précisés dans ce document.

**Décision prise :** Section 4.4 détaille les particularités de l'ambiguïté : suspension d'évaluation, pas de calcul de priorité, non-définitif.

**Correction effectuée :** Section 4.4 rédigée avec particularités de l'ambiguïté conformément à la Documentation Fondatrice.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice : Confirmée (ambiguïté détaillée)
- ✅ Cohérence avec Core Decision Contract : Confirmée (types de décisions)
- ✅ Cohérence avec Intent Model Contract : Confirmée (rejets structurels et de contenu)
- ✅ Cohérence avec Policy Engine Contract : Confirmée (rejets de politique)
- ✅ Cohérence avec Execution Prohibition Contract : Confirmée (INV-ERR-6, INV-ERR-7)

**Conclusion :** Aucune contradiction détectée.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
