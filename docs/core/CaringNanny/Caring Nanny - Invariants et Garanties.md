# Caring Nanny - Invariants et Garanties

## 1. Contexte

Ce document formalise les invariants techniques et les garanties de Caring Nanny. Il étend la Section 7 de la [Documentation Fondatrice](./Caring%20Nanny%20-%20Documentation%20Fondatrice.md) en détaillant les propriétés non négociables et les engagements mesurables de l'observateur d'état du Miyukini Core System.

## 2. Portée / Scope

Ce document couvre :
- Les invariants structurels (toujours vrais par construction)
- Les invariants comportementaux (toujours respectés à l'exécution)
- Les garanties envers les consommateurs d'état
- Les garanties envers les autorités (KindMother, StrongFather, BondingBrother)
- Les mécanismes de vérification

Ce document **ne couvre pas** :
- Les violations et anti-patterns (voir document dédié)
- Les détails d'implémentation
- Les cas d'erreur (voir Error & Rejection Model)
- Les flux détaillés (voir documents de flux dédiés)

---

## 3. Définitions

### 3.1 Invariant

Un **invariant** est une propriété qui doit toujours être vraie. Elle ne peut jamais être violée, quelles que soient les circonstances. Un invariant est vérifié par construction (architecture) ou par assertion (code).

**Caractéristiques d'un invariant :**
- Non négociable : aucune exception possible
- Non configurable : pas d'option pour le désactiver
- Non contournable : aucun chemin de code ne peut l'éviter
- Vérifiable : son respect peut être prouvé

### 3.2 Garantie

Une **garantie** est un engagement de Caring Nanny envers ses consommateurs. Elle décrit un comportement promis que les consommateurs peuvent considérer comme acquis.

**Caractéristiques d'une garantie :**
- Contractuelle : formellement documentée
- Mesurable : son respect peut être vérifié
- Stable : ne change pas sans changement de version majeure

### 3.3 Consommateur d'état

Un **consommateur d'état** est tout composant qui interroge Caring Nanny pour connaître l'état du système. Les consommateurs incluent StrongFather, BondingBrother, les modules SPM, et les produits.

---

## 4. Invariants de nature (ce que Caring Nanny EST)

Ces invariants définissent la nature fondamentale de Caring Nanny. Ils sont vrais par définition et ne peuvent être remis en question.

### 4.1 INV-CN-1 : Observateur pur

**Énoncé :** Caring Nanny est **exclusivement** un observateur. Elle observe, elle rapporte, elle propage des informations d'état, mais elle ne modifie jamais l'état du système qu'elle observe.

**Implications :**
- Aucune opération de Caring Nanny ne peut avoir d'effet de bord sur les données
- Aucune opération ne peut modifier l'état des composants observés
- L'observation est strictement passive et non intrusive
- La présence de Caring Nanny n'a aucun impact fonctionnel sur le système

**Vérification :** Revue architecturale. Aucune méthode de Caring Nanny ne possède d'effet de bord sur les données métier ou l'état des autorités.

---

### 4.2 INV-CN-3 : Non-autoritaire

**Énoncé :** Caring Nanny ne détient **aucune autorité** sur aucun aspect du système. Elle ne peut pas valider, invalider, accepter, ou refuser quoi que ce soit.

**Implications :**
- Aucun composant de Caring Nanny ne prend de décision
- Aucun composant de Caring Nanny ne possède de droit de veto
- Aucun composant de Caring Nanny ne peut bloquer une opération
- Les informations d'état sont informatives, jamais prescriptives

**Vérification :** Revue architecturale. Aucune méthode `validate()`, `approve()`, `reject()`, ou `authorize()` n'existe dans Caring Nanny.

---

### 4.3 INV-CN-4 : État cohérent

**Énoncé :** L'état rapporté par Caring Nanny est **toujours cohérent**. Il n'y a jamais de contradiction dans l'état observé : si un composant est rapporté comme "healthy", il ne peut pas être simultanément rapporté comme "error".

**Implications :**
- Un composant ne peut avoir qu'un seul état à un instant donné
- L'état système global est une synthèse cohérente des états partiels
- Les contradictions apparentes sont résolues selon des règles de priorité définies
- Les consommateurs peuvent se fier à la cohérence de l'information fournie

**Vérification :** Tests automatisés vérifiant qu'aucune réponse ne contient de contradiction (état A et non-A simultanés).

---

### 4.4 INV-CN-7 : Propagation fidèle

**Énoncé :** Caring Nanny propage les changements d'état **sans modification**. L'information transmise est exactement celle observée, sans interprétation, sans filtrage, sans transformation.

**Implications :**
- Les destinataires reçoivent une information fiable et non altérée
- La sémantique de l'état est préservée lors de la propagation
- Aucune information essentielle n'est ajoutée ou supprimée
- La traçabilité est maintenue de l'observation à la propagation

**Conformité LOI-1 :** Cette propagation fidèle fonctionne localement sans dépendance externe critique, conforme à **LOI-1** (aucune dépendance externe critique à l'exécution) définie dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

**Vérification :** Comparaison automatisée entre l'état observé et l'état propagé. Les deux doivent être sémantiquement identiques.

---

## 5. Invariants de non-action (ce que Caring Nanny NE FAIT JAMAIS)

Ces invariants définissent les actions que Caring Nanny refuse structurellement d'effectuer.

### 5.1 INV-CN-2 : Aucune capacité d'exécution

**Énoncé :** Caring Nanny ne possède **aucune capacité d'exécution**. Elle ne peut pas déclencher d'action, ni directement ni indirectement.

**Actions interdites :**
- Modifier des données dans KindMother
- Déclencher des opérations de synchronisation
- Exécuter des actions correctives
- Invoquer des méthodes qui modifient l'état du système

**Ce qui est autorisé :**
- Observer l'état des composants
- Enregistrer des observations dans l'historique
- Propager des notifications d'état via BondingBrother
- Répondre aux consultations d'état

**Vérification :** Revue de code. Aucune méthode `execute()`, `modify()`, `update()`, ou `trigger()` n'existe dans Caring Nanny.

---

### 5.2 INV-NEG-CN-01 : Jamais de modification de données

**Énoncé :** Caring Nanny **ne modifie jamais** aucune donnée dans le système.

**Exemples de modifications interdites :**
- Écrire dans la base de données de KindMother
- Créer, modifier, ou supprimer des entités métier
- Valider ou invalider des WriteIntent
- Modifier l'état de synchronisation

**Vérification :** Audit des appels API. Aucun appel d'écriture vers KindMother ou autre source de données.

---

### 5.3 INV-NEG-CN-02 : Jamais de décision

**Énoncé :** Caring Nanny **ne prend jamais** de décision basée sur l'état observé.

**Exemples de décisions interdites :**
- Décider de réagir à une anomalie détectée
- Choisir d'activer ou désactiver un composant
- Autoriser ou refuser une opération basée sur l'état
- Définir une priorité de traitement

**Ce qui est autorisé :**
- Classifier les états selon les catégories définies (healthy, degraded, offline, syncing, error)
- Appliquer des règles d'agrégation prédéfinies
- Déterminer les destinataires d'une propagation selon des règles établies

**Vérification :** Revue de code. Aucune logique conditionnelle basée sur des critères métier qui entraîne une action.

---

### 5.4 INV-NEG-CN-03 : Jamais d'action corrective

**Énoncé :** Caring Nanny **n'exécute jamais** d'action corrective en réponse à une anomalie détectée.

**Actions correctives interdites :**
- Redémarrer un composant défaillant
- Lancer une synchronisation forcée
- Invalider un cache
- Basculer vers un mode de secours

**Ce que Caring Nanny fait :**
- Détecter l'anomalie
- Classifier l'anomalie
- Propager l'information aux composants concernés
- Enregistrer l'anomalie dans l'historique

**Vérification :** Audit du comportement. Aucune action système n'est déclenchée suite à une détection d'anomalie.

---

### 5.5 INV-NEG-CN-04 : Jamais de médiation d'intentions

**Énoncé :** Caring Nanny **ne médiatise jamais** les intentions des produits vers les autorités.

**Actions de médiation interdites :**
- Recevoir des intentions de produits
- Traduire des demandes de produits
- Router des intentions vers les autorités
- Filtrer des réponses d'autorités pour les produits

**Distinction avec BondingBrother :**
- BondingBrother médiatise les intentions
- Caring Nanny observe et informe

**Vérification :** Analyse des interfaces. Aucune interface d'intention n'est exposée par Caring Nanny.

---

### 5.6 INV-NEG-CN-05 : Jamais de définition de règles

**Énoncé :** Caring Nanny **ne définit jamais** de règles pour la classification des états ou la détection des anomalies.

**Ce que Caring Nanny ne fait pas :**
- Définir les seuils de dégradation
- Créer des critères d'anomalie
- Établir des règles de priorité

**Ce que Caring Nanny fait :**
- Appliquer les règles définies par le produit ou l'écosystème
- Classifier selon les critères établis
- Détecter selon les patterns configurés

**Vérification :** Les règles sont chargées depuis une source externe (configuration), jamais générées par Caring Nanny.

---

### 5.7 INV-NEG-CN-06 : Jamais de gestion de persistance

**Énoncé :** Caring Nanny **ne gère jamais** la persistance de ses observations dans un système externe de manière autonome.

**Ce que Caring Nanny ne fait pas :**
- Écrire directement dans une base de données externe
- Gérer des transactions de persistance
- Définir des stratégies de rétention

**Ce que Caring Nanny fait :**
- Maintenir un historique en mémoire
- Déléguer la persistance à KindMother si nécessaire (via les canaux appropriés)
- Exposer l'historique pour consultation

**Vérification :** Audit des dépendances. Aucune connexion directe à un système de persistance externe.

---

## 6. Invariants de flux (comment l'information transite)

Ces invariants définissent les propriétés du transit de l'information d'état à travers Caring Nanny.

### 6.1 INV-CN-5 : Traçabilité complète

**Énoncé :** Chaque observation, chaque transition, chaque propagation est **entièrement traçable**. L'historique permet de reconstituer l'évolution de l'état du système dans le temps.

**Éléments toujours tracés :**
- Observation (timestamp, condition détectée, état résultant)
- Transition (état précédent, état suivant, cause, timestamp)
- Propagation (destinataires, message, timestamp)
- Consultation (demandeur, réponse, timestamp)

**Conformité LOI-3 :** Cette traçabilité complète maintient l'historique local comme source de vérité souveraine, conforme à **LOI-3** (l'état local est souverain) définie dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

**Vérification :** Audit de l'historique. Toute interaction a une entrée correspondante avec contexte complet.

---

### 6.2 INV-CN-6 : Non-bloquant

**Énoncé :** Caring Nanny ne bloque **jamais** les opérations du système. L'observation est passive et n'interfère pas avec le fonctionnement normal.

**Implications :**
- Les consultations sont asynchrones ou à faible latence
- Les propagations sont non bloquantes
- Les observations n'impactent pas les performances
- La présence de Caring Nanny n'a aucun impact sur la disponibilité

**Vérification :** Tests de performance. Les temps de réponse des composants observés ne sont pas affectés par Caring Nanny.

---

### 6.3 INV-FLUX-CN-01 : Séquence d'observation cohérente

**Énoncé :** Toute observation suit une séquence définie, sans saut d'étape.

**Séquence obligatoire d'observation :**
1. Détection de condition
2. Évaluation selon les critères de classification
3. Traduction en état partiel
4. Agrégation en état global (si applicable)
5. Détection de transition (si changement)
6. Enregistrement dans l'historique

**Vérification :** Chaque étape est tracée. Une trace incomplète déclenche une alerte.

---

### 6.4 INV-FLUX-CN-02 : Séquence de propagation cohérente

**Énoncé :** Toute propagation suit une séquence définie, sans saut d'étape.

**Séquence obligatoire de propagation :**
1. Identification des destinataires
2. Formulation du message (état précédent, état actuel, cause)
3. Délégation à BondingBrother
4. Enregistrement de la propagation

**Vérification :** Chaque étape est tracée. Comparaison entre transitions détectées et propagations effectuées.

---

### 6.5 INV-FLUX-CN-03 : Pas de perte d'observation

**Énoncé :** Aucune observation n'est perdue, même en cas de charge élevée ou de conditions anormales.

**Mécanismes de protection :**
- Buffer d'observations en cas de saturation
- Journalisation immédiate avant traitement
- Priorité aux observations critiques (error > degraded > autres)

**Vérification :** Réconciliation périodique entre conditions détectées et observations enregistrées.

---

## 7. Garanties envers les consommateurs d'état

Ces garanties sont les engagements de Caring Nanny envers les composants qui consultent l'état.

### 7.1 GAR-CONS-01 : État toujours disponible

**Engagement :** Caring Nanny fournit toujours une réponse à une demande d'état, même si l'état est incertain.

**Implications :**
- Pas de timeout sans réponse
- En cas d'incertitude, l'état "unknown" ou le dernier état connu est retourné
- Le timestamp de l'observation est toujours inclus

**Mesure :** Taux de réponse à 100% sur les consultations d'état.

---

### 7.2 GAR-CONS-02 : Cohérence garantie

**Engagement :** L'état retourné est toujours cohérent et sans contradiction.

**Implications :**
- Un composant ne peut avoir qu'un seul état
- L'état système est une synthèse valide des états partiels
- Les transitions respectent les règles de validité (pas de saut d'état interdit)

**Mesure :** Tests automatisés de cohérence sur les réponses d'état.

---

### 7.3 GAR-CONS-03 : Historique accessible

**Engagement :** Un consommateur peut obtenir l'historique des états sur une période configurable.

**Accès fourni :**
- Liste des états passés
- Transitions effectuées
- Causes des transitions
- Timestamps précis

**Mesure :** API de consultation de l'historique avec filtrage par composant et période.

---

### 7.4 GAR-CONS-04 : Notifications fiables

**Engagement :** Les notifications de changement d'état sont émises de manière fiable et ordonnée.

**Implications :**
- Toute transition génère une notification
- Les notifications sont ordonnées chronologiquement
- Les notifications ne sont pas dupliquées

**Mesure :** Comparaison entre transitions enregistrées et notifications émises.

---

### 7.5 GAR-CONS-05 : Contexte complet

**Engagement :** Chaque réponse d'état inclut le contexte nécessaire à son interprétation.

**Informations toujours incluses :**
- État courant
- Timestamp de l'observation
- Durée dans l'état actuel
- Cause de la dernière transition (si disponible)

**Mesure :** Validation de la complétude des réponses.

---

## 8. Garanties envers les autorités

Ces garanties sont les engagements de Caring Nanny envers KindMother, StrongFather, et BondingBrother.

### 8.1 GAR-AUTH-01 : Observation non intrusive

**Engagement :** L'observation de Caring Nanny n'interfère jamais avec le fonctionnement des autorités.

**Implications :**
- Pas de charge supplémentaire significative
- Pas de modification d'état
- Pas de verrouillage de ressources

**Conformité LOI-2 :** Cette observation non intrusive permet au système de fonctionner normalement même en isolation, respectant **LOI-2** (le système accepte l'isolement comme état normal) définie dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

**Mesure :** Tests de charge comparant les performances avec et sans Caring Nanny.

---

### 8.2 GAR-AUTH-02 : Respect de la confidentialité

**Engagement :** Caring Nanny ne divulgue jamais d'informations sensibles des autorités aux consommateurs non autorisés.

**Informations protégées :**
- Détails internes de KindMother
- Politiques de StrongFather
- Informations de routage de BondingBrother

**Mesure :** Audits de sécurité sur les réponses d'état.

---

### 8.3 GAR-AUTH-03 : Fidélité de l'observation

**Engagement :** L'état rapporté reflète fidèlement l'état réel des autorités.

**Implications :**
- Pas d'interprétation subjective
- Pas de prédiction ou d'anticipation
- Observation factuelle et objective

**Mesure :** Comparaison périodique entre l'état rapporté et l'état réel des autorités.

---

### 8.4 GAR-AUTH-04 : Propagation via canaux appropriés

**Engagement :** Les notifications d'état sont propagées exclusivement via BondingBrother, jamais directement.

**Implications :**
- Respect de l'architecture de médiation
- Pas de canal de communication parallèle
- Traçabilité complète des propagations

**Mesure :** Audit des flux de communication. Toutes les propagations passent par BondingBrother.

---

## 9. Mécanismes de vérification

### 9.1 Vérification statique (au build)

| Invariant | Mécanisme | Fréquence |
|-----------|-----------|-----------|
| INV-CN-1 | Revue architecturale | Chaque PR |
| INV-CN-2 | Analyse de code (pas d'effet de bord) | CI |
| INV-CN-3 | Vérification des interfaces | CI |
| INV-NEG-CN-01 | Audit des appels API | CI |
| INV-NEG-CN-02 | Analyse de logique conditionnelle | CI |

### 9.2 Vérification dynamique (au runtime)

| Invariant | Mécanisme | Fréquence |
|-----------|-----------|-----------|
| INV-CN-4 | Vérification de cohérence des réponses | Temps réel |
| INV-CN-5 | Trace de chaque étape | Temps réel |
| INV-CN-6 | Monitoring des latences | Temps réel |
| INV-CN-7 | Comparaison observation/propagation | Temps réel |
| INV-FLUX-CN-01 | Validation de séquence | Temps réel |
| INV-FLUX-CN-02 | Validation de séquence | Temps réel |
| INV-FLUX-CN-03 | Réconciliation | Batch |

### 9.3 Vérification périodique (audits)

| Garantie | Mécanisme | Fréquence |
|----------|-----------|-----------|
| GAR-CONS-01 | Analyse des taux de réponse | Quotidien |
| GAR-CONS-02 | Tests de cohérence | Release |
| GAR-AUTH-01 | Tests de charge | Mensuel |
| GAR-AUTH-02 | Audits de sécurité | Mensuel |
| GAR-AUTH-03 | Comparaison état rapporté/réel | Hebdomadaire |

---

## 10. Matrice de couverture

Cette matrice montre quels composants conceptuels sont concernés par chaque invariant.

| Invariant | Observer | StateAggregator | TransitionDetector | Propagator | HistoryKeeper |
|-----------|----------|-----------------|-------------------|------------|---------------|
| INV-CN-1 | ✓ | ✓ | ✓ | ✓ | ✓ |
| INV-CN-2 | ✓ | ✓ | ✓ | ✓ | ✓ |
| INV-CN-3 | ✓ | ✓ | ✓ | ✓ | ✓ |
| INV-CN-4 | - | ✓ | - | - | - |
| INV-CN-5 | ✓ | - | ✓ | ✓ | ✓ |
| INV-CN-6 | ✓ | ✓ | ✓ | ✓ | - |
| INV-CN-7 | - | - | - | ✓ | - |
| INV-NEG-CN-01 | ✓ | - | - | - | ✓ |
| INV-NEG-CN-02 | - | - | ✓ | - | - |
| INV-NEG-CN-03 | - | - | ✓ | - | - |
| INV-NEG-CN-04 | ✓ | - | - | ✓ | - |
| INV-NEG-CN-05 | - | ✓ | ✓ | - | - |
| INV-NEG-CN-06 | - | - | - | - | ✓ |
| INV-FLUX-CN-01 | ✓ | ✓ | ✓ | - | ✓ |
| INV-FLUX-CN-02 | - | - | - | ✓ | ✓ |
| INV-FLUX-CN-03 | ✓ | ✓ | ✓ | - | ✓ |

---

## 11. Correspondance avec la Documentation Fondatrice

Cette section établit la traçabilité entre les invariants de ce document et ceux définis dans la Documentation Fondatrice.

| Invariant Fondateur | Invariant(s) détaillé(s) | Section |
|---------------------|--------------------------|---------|
| INV-CN-1 : Observateur pur | INV-CN-1, INV-NEG-CN-01 | 4.1, 5.2 |
| INV-CN-2 : Aucune capacité d'exécution | INV-CN-2, INV-NEG-CN-03 | 5.1, 5.4 |
| INV-CN-3 : Non-autoritaire | INV-CN-3, INV-NEG-CN-02 | 4.2, 5.3 |
| INV-CN-4 : État cohérent | INV-CN-4, GAR-CONS-02 | 4.3, 7.2 |
| INV-CN-5 : Traçabilité complète | INV-CN-5, INV-FLUX-CN-01, INV-FLUX-CN-02 | 6.1, 6.3, 6.4 |
| INV-CN-6 : Non-bloquant | INV-CN-6, GAR-AUTH-01 | 6.2, 8.1 |
| INV-CN-7 : Propagation fidèle | INV-CN-7, GAR-CONS-04 | 4.4, 7.4 |

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut INVARIANTS**. Il établit les propriétés non négociables de Caring Nanny qui doivent être vraies en toutes circonstances.

Toute implémentation de Caring Nanny doit garantir ces invariants. Toute violation est considérée comme un défaut critique. Toute modification des invariants nécessite une nouvelle version majeure et une revue architecturale complète.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** INVARIANTS — Non négociable  
**Dépendance :** Documentation Fondatrice v1.0 (Section 7)
