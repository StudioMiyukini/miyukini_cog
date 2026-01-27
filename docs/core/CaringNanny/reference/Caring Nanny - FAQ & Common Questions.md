# Caring Nanny - FAQ & Common Questions

## 1. Contexte

Ce document répond aux questions fréquemment posées concernant Caring Nanny, le core d'observation d'état (Strate 4) du Miyukini Core System. Il s'adresse aux développeurs, architectes, et intégrateurs qui cherchent des clarifications rapides sur le rôle, les capacités, et les limites de Caring Nanny.

## 2. Portée / Scope

Ce document couvre :
- Les questions fondamentales sur le rôle de Caring Nanny
- Les questions sur les interactions avec les autres cores
- Les questions sur l'observation et la propagation d'état
- Les questions sur les limites et ce que Caring Nanny ne fait pas
- Les questions sur l'implémentation et l'extension

Ce document **ne couvre pas** :
- Les spécifications techniques détaillées (voir [Architecture et Composants](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md))
- Les contrats formels (voir les documents dans `/contracts/`)
- Les définitions normatives (voir [Documentation Fondatrice](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md))

---

## 3. Questions fondamentales

### Q1 : Qu'est-ce que Caring Nanny ?

**Réponse :** Caring Nanny est l'**observateur d'état** du Miyukini Core System. Elle observe, détecte, classe, et propage les états du système sans jamais modifier, décider, ou exécuter.

Dans la métaphore familiale de Miyukini, Caring Nanny est la **nounou attentive** : elle observe, elle surveille, elle rapporte, mais elle n'agit jamais directement. Son rôle est de savoir ce qui se passe, de détecter les anomalies, et d'informer ceux qui ont l'autorité d'agir.

**Référence :** [Documentation Fondatrice, Section 1](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q2 : Quelle question fondamentale Caring Nanny résout-elle ?

**Réponse :** Caring Nanny répond à la question : **"Dans quel état se trouve le système à un instant donné ?"**

Cette question apparemment simple cache une complexité considérable. L'état d'un système distribué, modulaire, et offline-first n'est pas une valeur unique : c'est une composition d'états partiels, de transitions en cours, de conditions temporaires, et de dépendances croisées. Caring Nanny apporte une réponse structurée, cohérente, et traçable.

**Référence :** [Documentation Fondatrice, Section 1](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q3 : Pourquoi Caring Nanny est-elle nécessaire ?

**Réponse :** Sans Caring Nanny, chaque composant devrait :
- Implémenter sa propre logique de détection d'état
- Gérer ses propres notifications de changement d'état
- Déduire l'état des autres composants de manière indirecte
- Gérer les incohérences entre perceptions d'état différentes

Cette approche dispersée conduit à des erreurs de diagnostic, des réactions tardives aux problèmes, et une complexité accrue. Caring Nanny centralise cette responsabilité en un point unique, cohérent, et fiable.

**Référence :** [Documentation Fondatrice, Section 2](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q4 : Quels sont les cinq états possibles du système ?

**Réponse :** Caring Nanny classe l'état système en cinq catégories :

| État | Description |
|------|-------------|
| **healthy** | Tous les composants fonctionnent normalement, aucune anomalie détectée |
| **degraded** | Certains composants fonctionnent en mode dégradé, le système reste opérationnel |
| **offline** | Le système fonctionne en mode déconnecté, sans accès aux autorités centrales |
| **syncing** | Une synchronisation est en cours, certaines opérations peuvent être différées |
| **error** | Une erreur critique a été détectée, certaines opérations ne sont pas possibles |

**Important :** L'état `offline` est un état **normal** (isolement accepté), distinct de l'état `error` (anomalie). Cette distinction respecte **LOI-2** (le système accepte l'isolement comme état normal).

**Référence :** [State Model Contract](../contracts/observability/Caring%20Nanny%20-%20State%20Model%20Contract.md)

---

### Q5 : Quelle est la différence entre état système et état applicatif ?

**Réponse :**

- **État système** : Condition globale du Miyukini Core System à un instant donné. C'est une synthèse agrégée de tous les états partiels, cohérente et sans contradiction.

- **État applicatif** : Condition d'un module ou composant spécifique. C'est un état partiel qui contribue à l'état système global, avec une sémantique propre au composant.

Caring Nanny collecte les états applicatifs et les agrège en état système global.

**Référence :** [Documentation Fondatrice, Section 4](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

## 4. Questions sur les interactions

### Q6 : Comment Caring Nanny interagit-elle avec KindMother ?

**Réponse :** La relation est une **observation unidirectionnelle**.

**Ce que Caring Nanny observe de KindMother :**
- L'état de santé de la persistance (disponible, dégradé, indisponible)
- L'état de synchronisation (synchronisé, en cours, désynchronisé, conflits)
- L'état des instances (DB Mère accessible, DB Filles connectées)
- L'état des opérations en cours (écritures en attente, deltas non propagés)

**Ce que Caring Nanny ne fait JAMAIS :**
- Modifier des données
- Déclencher des opérations de synchronisation
- Valider ou invalider des WriteIntent
- Accéder directement à la couche de persistance

**Référence :** [KindMother Integration Contract](../contracts/integration/Caring%20Nanny%20-%20KindMother%20Integration%20Contract.md)

---

### Q7 : Comment Caring Nanny interagit-elle avec StrongFather ?

**Réponse :** La relation est une **information**, pas une délégation.

**Ce que Caring Nanny informe StrongFather :**
- L'état actuel du système (healthy, degraded, offline, syncing, error)
- Les transitions d'état en cours
- Les conditions qui pourraient affecter les décisions

**Ce que Caring Nanny ne fait JAMAIS :**
- Prendre une décision basée sur l'état observé
- Modifier une politique ou une contrainte
- Refuser ou accepter une intention
- Influencer le résultat d'une évaluation

StrongFather peut consulter Caring Nanny, mais toute décision reste chez StrongFather.

**Référence :** [StrongFather Integration Contract](../contracts/integration/Caring%20Nanny%20-%20StrongFather%20Integration%20Contract.md)

---

### Q8 : Comment Caring Nanny interagit-elle avec BondingBrother ?

**Réponse :** La relation est une **collaboration passive**.

**Ce que Caring Nanny fournit à BondingBrother :**
- Les notifications de changement d'état à propager
- L'état des composants concernés par une intention
- Les informations de diagnostic pour le filtrage

**Ce que Caring Nanny ne fait JAMAIS :**
- Médiatiser des intentions
- Traduire des demandes de produits
- Filtrer des réponses d'autorités
- Prendre des décisions de routage

Caring Nanny informe, BondingBrother propage. La distinction est fondamentale.

**Référence :** [BondingBrother Integration Contract](../contracts/integration/Caring%20Nanny%20-%20BondingBrother%20Integration%20Contract.md)

---

### Q9 : Caring Nanny peut-elle bloquer un Tool ou un Toolkit ?

**Réponse :** Non directement, mais elle fournit l'information permettant le blocage.

Caring Nanny répond à la question : *"L'état actuel du système permet-il cet appel de Tool ?"*

Si l'environnement est en état `SECURITY_LOCKDOWN` ou `error`, Caring Nanny rapporte cet état. La décision de bloquer ou non le Tool est prise par **StrongFather**, pas par Caring Nanny.

**Référence :** [Documentation Fondatrice, Section 3.4](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

## 5. Questions sur l'observation et la propagation

### Q10 : Comment fonctionne le flux d'observation ?

**Réponse :** Le flux d'observation comporte quatre étapes :

1. **Détection de condition** : Une condition est détectée dans un composant et transmise à Caring Nanny via les canaux d'observation

2. **Évaluation de l'état** : Caring Nanny évalue la condition selon les critères de classification et la traduit en état partiel

3. **Agrégation** : Les états partiels sont agrégés en état système global, avec résolution des contradictions selon les règles de priorité

4. **Détection de transition** : Si l'état global a changé, une transition est enregistrée avec la condition qui l'a provoquée

**Référence :** [Observation Flow Contract](../contracts/observability/Caring%20Nanny%20-%20Observation%20Flow%20Contract.md)

---

### Q11 : Comment fonctionne la propagation des changements d'état ?

**Réponse :** Le flux de propagation comporte quatre étapes :

1. **Identification des destinataires** : Caring Nanny identifie les composants concernés par la transition

2. **Formulation du message** : Le message de notification est construit avec l'état précédent, l'état actuel, et la cause

3. **Délégation à BondingBrother** : Caring Nanny transmet le message à BondingBrother pour propagation

4. **Enregistrement** : La propagation est enregistrée dans l'historique pour traçabilité

**Important :** La propagation est passive et fidèle. Le message transmis est exactement celui observé, sans interprétation ni filtrage.

**Référence :** [Propagation Flow Contract](../contracts/observability/Caring%20Nanny%20-%20Propagation%20Flow%20Contract.md)

---

### Q12 : Comment interroger l'état actuel du système ?

**Réponse :** Les composants interrogent Caring Nanny via le flux de consultation :

1. **Demande d'état** : Un composant demande l'état actuel (global ou spécifique)

2. **Réponse** : Caring Nanny retourne l'état demandé avec l'horodatage de l'observation et le contexte

3. **Aucune modification** : La consultation n'a aucun effet de bord sur le système

**Interfaces disponibles :**
- `IStateQuery` : Interrogation de l'état actuel
- `IHistoryQuery` : Interrogation de l'historique des observations
- `ITransitionQuery` : Interrogation de l'historique des transitions

**Référence :** [Architecture et Composants, Section 3.4](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

### Q13 : Qu'est-ce qu'une transition d'état ?

**Réponse :** Une **transition** est le passage d'un état à un autre. Elle a quatre caractéristiques :

- **Déterministe** : Un état donné conduit à un ensemble fini d'états possibles
- **Observable** : La transition elle-même est un fait observable
- **Traçable** : Chaque transition est enregistrée avec son contexte
- **Causale** : Une transition a toujours une cause identifiable

**Référence :** [Documentation Fondatrice, Section 4](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q14 : Qu'est-ce qu'une condition vs un état ?

**Réponse :**

- **Condition** : Un fait observable brut, avant interprétation. Exemples : "la connexion réseau est disponible", "le temps de réponse dépasse un seuil".

- **État** : Le résultat de l'évaluation et de la classification d'une ou plusieurs conditions. L'état est toujours catégorisé (healthy, degraded, offline, syncing, error).

Caring Nanny collecte les conditions, les évalue, et les traduit en états.

**Référence :** [Glossaire et Terminologie](./Caring%20Nanny%20-%20Glossaire%20et%20Terminologie.md)

---

## 6. Questions sur les limites

### Q15 : Que ne fait PAS Caring Nanny ?

**Réponse :** Caring Nanny a des limites strictes. Elle ne fait **jamais** :

| Action interdite | Responsable |
|-----------------|-------------|
| Modifier des données | KindMother |
| Prendre des décisions | StrongFather |
| Exécuter des actions correctives | Composant concerné ou produit |
| Médiatiser des intentions | BondingBrother |
| Valider des opérations | KindMother ou StrongFather |
| Définir des règles de classification | Produit ou écosystème |
| Bloquer des opérations | StrongFather |

**Référence :** [Documentation Fondatrice, Section 6](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q16 : Caring Nanny peut-elle prendre une décision automatique ?

**Réponse :** **Non, jamais.**

Caring Nanny est un observateur pur. Elle observe, rapporte, et propage des informations d'état, mais elle ne décide jamais. Si une anomalie est détectée, Caring Nanny informe les composants concernés, mais la décision de réagir appartient à StrongFather ou au composant concerné.

**Invariant INV-CN-1 :** "Caring Nanny est exclusivement un observateur."

**Référence :** [Invariants et Garanties](../contracts/governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)

---

### Q17 : Caring Nanny peut-elle corriger automatiquement un problème ?

**Réponse :** **Non, jamais.**

Caring Nanny détecte les anomalies et les rapporte, mais elle n'exécute jamais d'action corrective. La correction est toujours du ressort du composant concerné ou du produit.

**Invariant INV-CN-2 :** "Caring Nanny ne possède aucune capacité d'exécution."

**Référence :** [Invariants et Garanties](../contracts/governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)

---

### Q18 : L'observation peut-elle ralentir le système ?

**Réponse :** **Non.**

L'observation est passive et non bloquante. Caring Nanny ne bloque jamais les opérations du système. Les sondes sont passives et en lecture seule, sans effet de bord.

**Invariant INV-CN-6 :** "Caring Nanny ne bloque jamais les opérations du système."

**Référence :** [Performance & Scalability Contract](../contracts/lifecycle/Caring%20Nanny%20-%20Performance%20&%20Scalability%20Contract.md)

---

### Q19 : Caring Nanny peut-elle altérer les informations qu'elle propage ?

**Réponse :** **Non, jamais.**

La propagation est fidèle. L'information transmise est exactement celle observée, sans interprétation, sans filtrage, sans transformation.

**Invariant INV-CN-7 :** "Caring Nanny propage les changements d'état sans modification."

**Référence :** [Invariants et Garanties](../contracts/governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)

---

## 7. Questions sur la conformité aux Lois d'Autonomie

### Q20 : Caring Nanny fonctionne-t-elle en mode offline ?

**Réponse :** **Oui.**

Caring Nanny respecte **LOI-1** (aucune dépendance externe critique). L'observation d'état fonctionne localement, les observations sont enregistrées localement, et l'absence de connexion ne bloque jamais l'observation.

**Référence :** [Documentation Fondatrice, Section 10](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q21 : L'état "offline" est-il considéré comme une erreur ?

**Réponse :** **Non.**

L'état `offline` est un état **normal** où le système fonctionne sans connexion externe. Il est distinct de l'état `error` qui représente une anomalie. Cette distinction respecte **LOI-2** (le système accepte l'isolement comme état normal).

**Référence :** [Documentation Fondatrice, Section 10](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q22 : Caring Nanny nécessite-t-elle un temps synchronisé ?

**Réponse :** **Non.**

Caring Nanny respecte **LOI-4** (pas de temps global requis). Les observations sont horodatées localement via le kernel Clock, et les transitions d'état sont basées sur des conditions locales, pas sur des timestamps synchronisés.

**Référence :** [Documentation Fondatrice, Section 10](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

## 8. Questions sur l'implémentation

### Q23 : Comment est structurée l'architecture interne de Caring Nanny ?

**Réponse :** Caring Nanny est organisée en **quatre couches** :

1. **Couche Observation** : Collecte des conditions depuis les composants
2. **Couche Classification** : Évaluation, catégorisation et agrégation des états
3. **Couche Propagation** : Diffusion des changements d'état
4. **Couche Consultation** : Interface de lecture pour les consommateurs

Chaque couche est strictement isolée des autres.

**Référence :** [Architecture et Composants](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

### Q24 : Comment ajouter une nouvelle sonde d'observation ?

**Réponse :** Les nouvelles sondes (ComponentProbe) sont un **point d'extension** autorisé.

**Contraintes :**
- La sonde doit être passive et sans effet de bord
- Elle doit suivre le contrat `IConditionReception`
- Elle doit normaliser les conditions via `ConditionNormalizer`
- Elle ne peut jamais modifier le composant observé

**Référence :** [Architecture et Composants, Section 7.1](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

### Q25 : Peut-on ajouter de nouvelles catégories d'état ?

**Réponse :** **Non.**

Les cinq catégories d'état (healthy, degraded, offline, syncing, error) sont **figées** et non extensibles. Cela fait partie des éléments structurels non modifiables de Caring Nanny.

**Référence :** [Architecture et Composants, Section 7.2](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

### Q26 : Comment fonctionne l'historique des observations ?

**Réponse :** Le composant **HistoryStore** maintient l'historique complet :

- Enregistrement chronologique de toutes les observations
- Conservation des transitions avec leur cause
- Indexation pour recherche rapide
- Gestion de la rétention selon les politiques définies

L'historique est local, sans synchronisation externe obligatoire, conforme à **LOI-3**.

**Référence :** [Architecture et Composants, Section 4.1](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

### Q27 : Comment Caring Nanny s'auto-observe-t-elle ?

**Réponse :** Le composant **SelfHealthReporter** rapporte l'état de santé de Caring Nanny elle-même.

**Vérifications :**
- État des sondes d'observation (actives, dégradées, en erreur)
- Capacité de l'historique (espace disponible)
- Connectivité avec BondingBrother pour propagation
- Latence des opérations internes

**Important :** Il n'y a pas d'auto-observation récursive. SelfHealthReporter ne s'observe pas lui-même.

**Référence :** [Architecture et Composants, Section 4.4](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

## 9. Questions de clarification

### Q28 : Quelle est la différence entre Caring Nanny et un système de monitoring traditionnel ?

**Réponse :** Caring Nanny n'est pas un système de monitoring traditionnel :

| Aspect | Monitoring traditionnel | Caring Nanny |
|--------|------------------------|--------------|
| Focus | Métriques et alertes | États et transitions |
| Action | Peut déclencher des actions | Aucune action, observation pure |
| Scope | Technique (CPU, mémoire) | Conceptuel (santé, synchronisation) |
| Intégration | Externe au système | Core intégré au système |
| Décision | Peut prendre des décisions | Jamais de décision |

**Référence :** [Documentation Fondatrice](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q29 : Caring Nanny observe-t-elle les données métier ?

**Réponse :** **Non.**

Caring Nanny observe l'**état** des composants, pas les **données** qu'ils contiennent. Elle sait si KindMother est en état "healthy" ou "syncing", mais elle ne connaît pas le contenu des données gérées par KindMother.

**Référence :** [Documentation Fondatrice, Section 3](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Q30 : Comment Caring Nanny gère-t-elle les contradictions d'état ?

**Réponse :** L'agrégation des états partiels résout les contradictions apparentes selon des **règles de priorité** définies. L'état global est toujours cohérent et sans contradiction interne.

**Invariant INV-CN-4 :** "L'état rapporté par Caring Nanny est toujours cohérent."

**Référence :** [State Model Contract](../contracts/observability/Caring%20Nanny%20-%20State%20Model%20Contract.md)

---

## 10. Statut contractuel

Ce document est **informatif** et ne remplace pas les documents contractuels. Pour les définitions normatives, consultez :
- [Documentation Fondatrice](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) (statut FONDATION)
- [Invariants et Garanties](../contracts/governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) (statut CONTRAT)
- [Architecture et Composants](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md) (statut ARCHITECTURE)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** RÉFÉRENCE — Informatif  
**Dépendance :** Documentation Fondatrice v1.6, Architecture et Composants v1.0
