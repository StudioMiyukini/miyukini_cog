# Caring Nanny - Glossaire et Terminologie

## 1. Contexte

Ce document étend et précise le vocabulaire canonique introduit dans la Section 9 de la [Documentation Fondatrice](./Caring%20Nanny%20-%20Documentation%20Fondatrice.md). Il établit le dictionnaire complet et définitif de tous les termes utilisés dans l'écosystème Caring Nanny.

## 2. Portée / Scope

Ce document couvre :
- Les termes fondamentaux hérités du document fondateur
- Les termes architecturaux dérivés de la structure technique
- Les termes opérationnels utilisés dans les flux
- Les termes contractuels utilisés dans les spécifications

Ce document **établit** :
- La définition canonique et unique de chaque terme
- Les relations entre termes
- Les usages autorisés et interdits

---

## 3. Règles terminologiques

### 3.1 Règle d'unicité

Chaque concept a **un seul terme** autorisé. Les synonymes sont interdits dans la documentation officielle.

### 3.2 Règle de précision

Chaque terme a **une seule définition**. Aucune interprétation contextuelle n'est autorisée.

### 3.3 Règle de stabilité

Les termes sont **versionnés** avec la documentation. Un terme ne peut changer de sens qu'avec un changement de version majeure.

### 3.4 Règle d'usage

L'usage d'un terme non défini dans ce glossaire est **interdit** dans la documentation contractuelle.

---

## 4. Termes fondamentaux

### 4.1 État

**Définition :** Condition observable d'un composant ou du système à un instant donné. Un état représente une photographie de la situation à un moment précis.

**Caractéristiques :**
- Toujours catégorisé selon les catégories définies (healthy, degraded, offline, syncing, error)
- Toujours daté avec un horodatage précis (horodatage local, conforme à **LOI-4** : pas de temps global requis, voir [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md))
- Toujours contextualisé avec son domaine d'observation
- Toujours cohérent (pas de contradiction interne)

**Forme canonique :**
```
État {
    catégorie: CatégorieÉtat,
    timestamp: Horodatage,
    contexte: ContexteObservation,
    source: IdentitéComposant
}
```

**Types d'états :**

| Type | Portée | Description |
|------|--------|-------------|
| État système | Global | Synthèse de tous les états partiels du système |
| État applicatif | Partiel | Condition d'un module ou composant spécifique |

**Termes apparentés :**
- Condition (fait observable qui peut influencer l'état)
- Catégorie d'état (classification de l'état)

---

### 4.2 Observation

**Définition :** Acte par lequel Caring Nanny détecte et enregistre une condition ou un état. L'observation est le mécanisme fondamental de Caring Nanny.

**Caractéristiques :**
- Passive : n'influence pas ce qui est observé
- Non intrusive : ne perturbe pas le fonctionnement normal
- Sans effet de bord : ne modifie aucune donnée
- Traçable : enregistrée avec son contexte

**Forme canonique :**
```
Observation {
    cible: IdentitéComposant,
    état_observé: État,
    timestamp: Horodatage,
    méthode: MéthodeObservation
}
```

**Propriétés de l'observation :**
- **Fidélité** : L'observation reflète exactement ce qui est observé
- **Neutralité** : L'observation ne juge pas, ne décide pas
- **Complétude** : L'observation capture toutes les informations pertinentes

**Ce que l'observation n'est pas :**
- Une action (pas de modification)
- Une décision (pas de jugement)
- Une validation (pas d'approbation/rejet)

---

### 4.3 Transition

**Définition :** Passage d'un état à un autre. Une transition représente un changement observable dans le système.

**Caractéristiques :**
- Déterministe : Un état donné conduit à un ensemble fini d'états possibles
- Observable : La transition elle-même est un fait observable
- Traçable : Enregistrée avec son contexte complet
- Causale : Toujours provoquée par une condition identifiable

**Forme canonique :**
```
Transition {
    état_précédent: État,
    état_actuel: État,
    cause: Condition,
    timestamp: Horodatage
}
```

**Propriétés d'une transition :**
- **Atomicité** : Une transition est indivisible
- **Irréversibilité** : Une transition ne peut pas être annulée (mais une transition inverse peut survenir)
- **Traçabilité** : L'historique des transitions est conservé

**Termes apparentés :**
- Cause (condition qui provoque la transition)
- État précédent / État actuel

---

### 4.4 Propagation

**Définition :** Mécanisme par lequel un changement d'état est communiqué aux composants concernés. La propagation est une diffusion d'information, pas une modification d'état.

**Caractéristiques :**
- Passive : Caring Nanny informe, elle ne modifie pas
- Sélective : Seuls les composants concernés sont informés
- Traçable : Chaque propagation est enregistrée
- Non bloquante : La propagation n'attend pas de confirmation d'action
- Fidèle : L'information transmise n'est pas altérée

**Forme canonique :**
```
Propagation {
    transition: Transition,
    destinataires: Liste<IdentitéComposant>,
    timestamp: Horodatage,
    canal: CanalPropagation
}
```

**Ce que la propagation ne fait pas :**
- Ne déclenche pas d'action
- Ne modifie pas l'état transmis
- Ne filtre pas l'information (sauf le périmètre des destinataires)
- N'attend pas de réponse

---

### 4.5 Condition

**Définition :** Fait observable qui peut influencer l'état. Une condition est un élément d'information brut, avant interprétation en termes d'état.

**Caractéristiques :**
- Factuelle : Représente un fait, pas une interprétation
- Observable : Peut être détectée par Caring Nanny
- Temporelle : Valide à un moment donné
- Contextuelle : A un contexte d'observation

**Forme canonique :**
```
Condition {
    type: TypeCondition,
    valeur: Valeur,
    timestamp: Horodatage,
    source: IdentitéComposant
}
```

**Exemples de conditions :**
- La connexion réseau est disponible
- Le temps de réponse dépasse un seuil
- Un composant ne répond pas
- Une synchronisation a échoué
- L'espace disque est insuffisant

**Différence avec l'état :**
- Une condition est un fait brut
- Un état est une classification d'un ensemble de conditions

---

### 4.6 Anomalie

**Définition :** Condition qui s'écarte du comportement attendu. Une anomalie signale un écart par rapport à la norme, mais n'est pas nécessairement une erreur.

**Caractéristiques :**
- Détectée par Caring Nanny
- Rapportée aux composants concernés
- Jamais corrigée par Caring Nanny
- Peut précéder une transition vers un état dégradé ou d'erreur

**Forme canonique :**
```
Anomalie {
    type: TypeAnomalie,
    condition_anormale: Condition,
    seuil_attendu: Valeur,
    valeur_observée: Valeur,
    timestamp: Horodatage
}
```

**Types d'anomalies :**

| Type | Description | Exemple |
|------|-------------|---------|
| Seuil dépassé | Une valeur dépasse une limite | Temps de réponse > 500ms |
| Pattern anormal | Un comportement inhabituel | Pic de requêtes anormal |
| Absence de signal | Un composant ne répond plus | Timeout de healthcheck |
| Incohérence | Données contradictoires | États conflictuels |

**Ce que Caring Nanny fait avec une anomalie :**
- La détecte
- L'enregistre
- La propage aux composants concernés

**Ce que Caring Nanny ne fait JAMAIS avec une anomalie :**
- La corriger
- Prendre une décision corrective
- Bloquer des opérations

---

### 4.7 Santé

**Définition :** Catégorie d'état qui indique le niveau de fonctionnement d'un composant ou du système. La santé est l'interprétation synthétique de l'état.

**Catégories de santé :**

| Catégorie | Signification | Caractéristiques |
|-----------|---------------|------------------|
| healthy | Fonctionnement normal | Aucune anomalie, toutes conditions nominales |
| degraded | Mode dégradé | Certaines anomalies, fonctionnement partiel |
| offline | Mode déconnecté | Sans accès aux autorités centrales |
| syncing | Synchronisation en cours | Opérations potentiellement différées |
| error | Erreur critique | Certaines opérations impossibles |

**Règles de catégorisation :**
- Les catégories sont mutuellement exclusives (un composant n'a qu'une seule catégorie à un instant donné)
- La catégorie est déterminée par des règles de classification explicites
- La catégorie d'un état système est agrégée depuis les catégories des états applicatifs

---

### 4.8 Diagnostic

**Définition :** Analyse de l'historique d'observations pour identifier la cause d'un problème. Le diagnostic utilise les données collectées par Caring Nanny mais n'est pas réalisé par Caring Nanny.

**Caractéristiques :**
- Utilise l'historique des observations
- Utilise l'historique des transitions
- Recherche les causes racines
- Identifie les patterns

**Ce que Caring Nanny fournit pour le diagnostic :**
- L'historique complet des observations
- Les transitions avec leurs causes
- Les conditions observées
- Les anomalies détectées

**Ce que Caring Nanny ne fait pas :**
- Réaliser le diagnostic elle-même
- Interpréter les données
- Proposer des solutions
- Prendre des décisions correctives

---

### 4.9 Agrégation

**Définition :** Opération par laquelle Caring Nanny synthétise les états partiels des composants en état système global.

**Caractéristiques :**
- Déterministe : Mêmes entrées = même résultat
- Cohérente : Pas de contradiction dans le résultat
- Reproductible : Peut être recalculée à l'identique
- Documentée : Les règles d'agrégation sont explicites

**Forme canonique :**
```
Agrégation {
    états_partiels: Liste<ÉtatApplicatif>,
    règles: RèglesAgrégation,
    résultat: ÉtatSystème,
    timestamp: Horodatage
}
```

**Règles d'agrégation par défaut :**
- Si un état partiel est "error", l'état global est au minimum "degraded"
- Si tous les états partiels sont "healthy", l'état global est "healthy"
- Si un état partiel est "offline", l'état global reflète le mode offline
- Si un état partiel est "syncing", l'état global peut être "syncing"

---

### 4.10 Historique

**Définition :** Ensemble des observations enregistrées par Caring Nanny. L'historique est la mémoire de l'évolution du système dans le temps.

**Caractéristiques :**
- Complet : Toutes les observations sont enregistrées
- Ordonné : L'ordre chronologique est préservé
- Immuable : L'historique n'est jamais modifié après enregistrement
- Accessible : L'historique est consultable pour audit et diagnostic

**Éléments de l'historique :**
- Toutes les observations
- Toutes les transitions
- Toutes les propagations
- Toutes les anomalies détectées
- Tous les états calculés

**Propriétés de l'historique :**
- **Intégrité** : Aucune perte d'information
- **Authenticité** : Aucune modification possible
- **Traçabilité** : Chaque entrée est horodatée et contextualisée

---

## 5. Termes des catégories d'état

### 5.1 Healthy

**Définition :** Catégorie d'état indiquant un fonctionnement normal. Aucune anomalie n'a été détectée, toutes les conditions sont nominales.

**Caractéristiques :**
- Tous les composants observés fonctionnent normalement
- Aucun seuil n'est dépassé
- Aucune anomalie n'est active
- Toutes les dépendances sont disponibles

**Transition depuis healthy :**
- Vers degraded : si une anomalie non critique est détectée
- Vers offline : si la connexion aux autorités est perdue
- Vers syncing : si une synchronisation est déclenchée
- Vers error : si une erreur critique survient

---

### 5.2 Degraded

**Définition :** Catégorie d'état indiquant un fonctionnement partiel ou dégradé. Le système reste opérationnel mais certaines anomalies ont été détectées.

**Caractéristiques :**
- Le système est toujours fonctionnel
- Certaines fonctionnalités peuvent être limitées
- Des anomalies sont actives
- Une intervention peut être nécessaire

**Transition depuis degraded :**
- Vers healthy : si les anomalies sont résolues
- Vers offline : si la connexion aux autorités est perdue
- Vers syncing : si une synchronisation est déclenchée
- Vers error : si une erreur critique survient

---

### 5.3 Offline

**Définition :** Catégorie d'état indiquant un fonctionnement en mode déconnecté. Le système fonctionne sans accès aux autorités centrales.

**Caractéristiques :**
- Le système fonctionne localement
- Les autorités centrales ne sont pas accessibles
- Certaines opérations sont différées
- Les données locales sont utilisées

**Conformité LOI-2 :** L'état `offline` est reconnu comme un **état normal**, pas comme une erreur. Cette distinction respecte **LOI-2** (le système accepte l'isolement comme état normal) définie dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

**Transition depuis offline :**
- Vers syncing : si la connexion est rétablie
- Vers degraded : si des problèmes locaux surviennent
- Vers error : si une erreur critique survient

---

### 5.4 Syncing

**Définition :** Catégorie d'état indiquant une synchronisation en cours. Le système est en train de réconcilier son état avec les autorités centrales.

**Caractéristiques :**
- Une synchronisation est active
- Certaines opérations peuvent être différées
- L'état peut être temporairement incohérent
- La durée est normalement limitée

**Transition depuis syncing :**
- Vers healthy : si la synchronisation réussit sans anomalie
- Vers degraded : si la synchronisation révèle des problèmes
- Vers offline : si la connexion est perdue pendant la synchronisation
- Vers error : si la synchronisation échoue de manière critique

---

### 5.5 Error

**Définition :** Catégorie d'état indiquant une erreur critique. Certaines opérations ne sont pas possibles et une intervention est requise.

**Caractéristiques :**
- Une erreur critique a été détectée
- Certaines opérations sont impossibles
- Une intervention est requise
- Le système peut être partiellement non fonctionnel

**Transition depuis error :**
- Vers degraded : si l'erreur est partiellement résolue
- Vers healthy : si l'erreur est complètement résolue
- Vers offline : si la connexion est perdue

---

## 6. Termes architecturaux

### 6.1 Observateur

**Définition :** Entité responsable de la collecte des conditions et de la détection des changements d'état. Caring Nanny est l'observateur d'état privilégié du système.

**Caractéristiques :**
- Passif : N'influence pas ce qu'il observe
- Privilégié : A accès à l'information d'état de tous les composants
- Unique : Il n'y a qu'un seul observateur d'état système
- Non autoritaire : N'a aucun pouvoir de décision

**Propriétés d'un observateur :**
- **Pureté** : Aucun effet de bord
- **Fidélité** : Observation exacte de la réalité
- **Exhaustivité** : Observation de tous les aspects pertinents

---

### 6.2 Source d'état

**Définition :** Composant qui produit des informations d'état observables par Caring Nanny.

**Sources d'état dans l'écosystème :**

| Source | Type d'information | Exemple |
|--------|-------------------|---------|
| KindMother | État de persistance et synchronisation | DB disponible, en sync |
| StrongFather | État des politiques et permissions | Politique active, suspendue |
| BondingBrother | État de la médiation | Canal actif, congestionné |
| Modules SPM | État applicatif | Module chargé, erreur de schéma |

**Ce qu'une source d'état fournit :**
- Des conditions observables
- Des métriques de santé
- Des signaux de changement

---

### 6.3 Canal d'observation

**Définition :** Voie par laquelle les conditions transitent depuis les sources d'état vers Caring Nanny.

**Types de canaux :**

| Type | Mode | Description |
|------|------|-------------|
| Push | Actif | La source envoie les conditions |
| Pull | Passif | Caring Nanny interroge la source |
| Event | Réactif | La source émet des événements |

**Propriétés d'un canal :**
- Fiable : Pas de perte d'information
- Ordonné : L'ordre des conditions est préservé
- Non intrusif : N'impacte pas les performances

---

### 6.4 Destinataire

**Définition :** Composant qui reçoit les notifications de changement d'état propagées par Caring Nanny.

**Types de destinataires :**
- StrongFather (pour enrichir le contexte des décisions)
- BondingBrother (pour propager aux produits)
- Modules SPM (pour réagir aux changements)
- Produits (via BondingBrother)

**Ce qu'un destinataire reçoit :**
- La transition d'état
- L'état précédent et l'état actuel
- La cause de la transition
- L'horodatage

**Ce qu'un destinataire ne reçoit pas :**
- D'instructions d'action
- De décisions
- D'informations hors de son périmètre

---

## 7. Termes opérationnels

### 7.1 Flux d'observation

**Définition :** Séquence d'étapes par laquelle Caring Nanny collecte et traite l'information d'état.

**Étapes du flux :**
1. Détection de condition
2. Évaluation de l'état
3. Agrégation
4. Détection de transition

**Propriétés du flux :**
- Ordre strict des étapes
- Pas de saut d'étape
- Traçabilité à chaque étape
- Non bloquant

---

### 7.2 Flux de propagation

**Définition :** Séquence d'étapes par laquelle Caring Nanny communique les changements d'état.

**Étapes du flux :**
1. Identification des destinataires
2. Formulation du message
3. Délégation à BondingBrother
4. Enregistrement dans l'historique

**Propriétés du flux :**
- Sélectif (seuls les destinataires concernés)
- Fidèle (pas d'altération du message)
- Traçable (enregistrement complet)

---

### 7.3 Flux de consultation

**Définition :** Séquence d'étapes par laquelle un composant interroge Caring Nanny sur l'état actuel.

**Étapes du flux :**
1. Réception de la demande d'état
2. Récupération de l'état demandé
3. Retour de l'état avec contexte

**Propriétés du flux :**
- Sans effet de bord (la consultation ne modifie rien)
- Synchrone (réponse immédiate)
- Contextualisé (horodatage inclus)

---

### 7.4 Classification

**Définition :** Processus par lequel une condition ou un ensemble de conditions est traduit en catégorie d'état.

**Caractéristiques :**
- Basée sur des règles explicites
- Déterministe (mêmes conditions = même catégorie)
- Documentée (règles consultables)

**Éléments de classification :**
- Seuils (valeurs limites)
- Patterns (combinaisons de conditions)
- Priorités (en cas de conflit)

---

### 7.5 Notification

**Définition :** Message envoyé par Caring Nanny pour informer d'un changement d'état.

**Forme canonique :**
```
Notification {
    type: TypeNotification,
    transition: Transition,
    destinataires: Liste<IdentitéComposant>,
    timestamp: Horodatage
}
```

**Types de notifications :**
- Transition d'état système
- Transition d'état applicatif
- Anomalie détectée
- Retour à la normale

**Propriétés d'une notification :**
- Informative (pas directive)
- Complète (toutes les informations nécessaires)
- Traçable (enregistrée dans l'historique)

---

### 7.6 Seuil

**Définition :** Valeur limite qui détermine si une condition est normale ou anormale.

**Types de seuils :**

| Type | Description | Exemple |
|------|-------------|---------|
| Seuil d'alerte | Valeur de vigilance | Temps de réponse > 200ms |
| Seuil critique | Valeur de dégradation | Temps de réponse > 500ms |
| Seuil d'erreur | Valeur d'erreur | Temps de réponse > 2000ms |

**Propriétés d'un seuil :**
- Configurable (défini par le produit ou l'écosystème)
- Documenté (valeur et unité explicites)
- Versionné (historique des changements)

---

## 8. Termes contractuels

### 8.1 Contrat

**Définition :** Document normatif qui définit les règles, interfaces, ou comportements que Caring Nanny s'engage à respecter.

**Types de contrats :**

| Type | Portée | Exemple |
|------|--------|---------|
| Contrat de modèle | Structure des données | State Model Contract |
| Contrat de flux | Comportement des flux | Observation Flow Contract |
| Contrat d'intégration | Interactions avec autres composants | KindMother Integration Contract |
| Contrat opérationnel | Propriétés de fonctionnement | Performance Contract |

---

### 8.2 Invariant

**Définition :** Propriété qui doit toujours être vraie, quelles que soient les circonstances, et qui ne peut jamais être violée.

**Invariants de Caring Nanny (INV-CN) :**

| ID | Invariant | Description |
|----|-----------|-------------|
| INV-CN-1 | Observateur pur | Caring Nanny observe mais ne modifie jamais |
| INV-CN-2 | Aucune capacité d'exécution | Caring Nanny ne peut déclencher aucune action |
| INV-CN-3 | Non-autoritaire | Caring Nanny ne détient aucune autorité |
| INV-CN-4 | État cohérent | L'état rapporté est toujours sans contradiction |
| INV-CN-5 | Traçabilité complète | Tout est enregistré et auditable |
| INV-CN-6 | Non-bloquant | Caring Nanny ne bloque jamais les opérations |
| INV-CN-7 | Propagation fidèle | L'information transmise n'est jamais altérée |

**Propriétés d'un invariant :**
- Non négociable
- Non configurable
- Non désactivable
- Vérifié structurellement

---

### 8.3 Garantie

**Définition :** Engagement de Caring Nanny envers ses consommateurs (composants ou produits) sur un comportement ou une propriété.

**Exemples de garanties :**
- Vision cohérente de l'état (pas de contradiction)
- Observation sans effet de bord (pas de modification)
- Transitions traçables et auditables (historique complet)
- Propagation fidèle et non altérée (information exacte)

**Différence avec l'invariant :**
- L'invariant est interne (Caring Nanny s'impose à elle-même)
- La garantie est externe (Caring Nanny promet aux autres)

---

### 8.4 Violation

**Définition :** Situation où une règle, un invariant, ou un contrat n'est pas respecté.

**Traitement des violations :**
- Violations d'invariant : Impossible par construction (erreur de conception si détectée)
- Violations de contrat : Journalisation, notification, signalement

**Note :** Caring Nanny ne prend aucune action corrective en cas de violation. Elle se limite à observer et rapporter.

---

## 9. Termes interdits

Les termes suivants sont **interdits** dans la documentation de Caring Nanny car ils sont ambigus ou porteurs de mauvaises connotations :

| Terme interdit | Raison | Terme à utiliser |
|----------------|--------|------------------|
| Décision | Caring Nanny ne décide pas | Classification ou Catégorisation |
| Action | Caring Nanny n'agit pas | Observation ou Propagation |
| Correction | Caring Nanny ne corrige pas | Détection ou Signalement |
| Commande | Caring Nanny ne commande pas | Notification |
| Contrôle | Implique une autorité | Observation |
| Validation | Caring Nanny ne valide pas | Classification |
| Blocage | Caring Nanny ne bloque pas | (aucun équivalent — concept interdit) |
| Exécution | Caring Nanny n'exécute pas | (aucun équivalent — concept interdit) |
| Modification | Caring Nanny ne modifie pas | (aucun équivalent — concept interdit) |
| Cache | Implique un stockage actif | Historique |

---

## 10. Index alphabétique

| Terme | Section | Catégorie |
|-------|---------|-----------|
| Agrégation | 4.9 | Fondamental |
| Anomalie | 4.6 | Fondamental |
| Canal d'observation | 6.3 | Architectural |
| Classification | 7.4 | Opérationnel |
| Condition | 4.5 | Fondamental |
| Contrat | 8.1 | Contractuel |
| Degraded | 5.2 | Catégorie d'état |
| Destinataire | 6.4 | Architectural |
| Diagnostic | 4.8 | Fondamental |
| Error | 5.5 | Catégorie d'état |
| État | 4.1 | Fondamental |
| Flux d'observation | 7.1 | Opérationnel |
| Flux de consultation | 7.3 | Opérationnel |
| Flux de propagation | 7.2 | Opérationnel |
| Garantie | 8.3 | Contractuel |
| Healthy | 5.1 | Catégorie d'état |
| Historique | 4.10 | Fondamental |
| Invariant | 8.2 | Contractuel |
| Notification | 7.5 | Opérationnel |
| Observateur | 6.1 | Architectural |
| Observation | 4.2 | Fondamental |
| Offline | 5.3 | Catégorie d'état |
| Propagation | 4.4 | Fondamental |
| Santé | 4.7 | Fondamental |
| Seuil | 7.6 | Opérationnel |
| Source d'état | 6.2 | Architectural |
| Syncing | 5.4 | Catégorie d'état |
| Transition | 4.3 | Fondamental |
| Violation | 8.4 | Contractuel |

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut RÉFÉRENCE**. Il établit le vocabulaire officiel de Caring Nanny qui doit être utilisé dans toute documentation, code, et communication.

Tout terme utilisé dans un document contractuel de Caring Nanny doit être défini dans ce glossaire. Toute modification terminologique nécessite une nouvelle version de ce document.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** RÉFÉRENCE — Normatif  
**Dépendance :** Documentation Fondatrice v1.0 (Section 9)
