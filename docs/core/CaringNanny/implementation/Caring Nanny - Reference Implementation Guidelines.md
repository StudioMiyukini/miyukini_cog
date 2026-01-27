# Caring Nanny — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter Caring Nanny correctement, sans violer les contrats FONDATION.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment traduire les contrats FONDATION en implémentation, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas être interprété abusivement. Il ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implémenter Caring Nanny de manière conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implémentation sans interprétation abusive.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats FONDATION.

### 1.3. Rappel de la mission de Caring Nanny

Caring Nanny est le **core d'observation d'état** (Strate 4). Il répond à la question fondamentale :

> **"Dans quel état se trouve le système à un instant donné ?"**

Caring Nanny **observe, détecte, classe, et propage** les états du système. Il **ne modifie jamais**, **ne décide jamais**, et **n'exécute jamais**.

### 1.4. Sources contractuelles

Ce document se base sur les contrats FONDATION, avec un focus particulier sur :

- **Documentation Fondatrice** : Invariants INV-CN-1 à INV-CN-7, responsabilités exclusives, interdictions
- **Architecture et Composants** : Structure en 4 couches, composants internes
- **State Model Contract** : Modèle formel des états (healthy, degraded, offline, syncing, error)
- **Observation Flow Contract** : Flux d'observation détection → évaluation → agrégation → transition
- **Propagation Flow Contract** : Flux de propagation changement → destinataires → message → dispatch
- **Invariants & Garanties** : Garanties structurelles non négociables
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Les lignes directrices d'implémentation doivent respecter les 6 lois d'autonomie, notamment **LOI-1** (aucune dépendance externe critique), **LOI-2** (isolation acceptée comme état normal), **LOI-4** (pas de temps global requis).

---

## 2. Principes généraux à respecter absolument

### 2.1. Observateur pur (INV-CN-1)

**Principe contractuel :**

L'invariant INV-CN-1 établit que Caring Nanny est **exclusivement** un observateur. Elle observe, elle rapporte, elle propage des informations d'état, mais elle ne modifie jamais l'état du système qu'elle observe.

**Traduction en logique d'implémentation :**

- **Caring Nanny OBSERVE** : Elle détecte et enregistre les conditions du système.
- **Caring Nanny CLASSE** : Elle catégorise les états selon les cinq catégories définies.
- **Caring Nanny PROPAGE** : Elle diffuse les changements d'état via BondingBrother.
- **Caring Nanny NE FAIT JAMAIS** : Elle ne modifie aucune donnée, aucun état, aucune configuration.

**Ce que cela signifie concrètement :**

- Aucun mécanisme d'écriture de données métier ne doit être accessible à Caring Nanny
- Les observations sont stockées dans un historique propre à Caring Nanny, pas dans les données métier
- Caring Nanny n'a aucun effet de bord sur le système qu'elle observe

### 2.2. Aucune capacité d'exécution (INV-CN-2)

**Principe contractuel :**

L'invariant INV-CN-2 établit que Caring Nanny ne possède **aucune capacité d'exécution**. Elle ne peut pas déclencher d'action, ni directement ni indirectement. Si une action est nécessaire en réponse à un état observé, cette action doit être décidée et exécutée par un autre composant.

**Traduction en logique d'implémentation :**

- **Information seulement** : Caring Nanny fournit de l'information, jamais des ordres.
- **Pas de callbacks d'action** : Aucun mécanisme ne permet de lier une observation à une action.
- **Délégation obligatoire** : Toute réaction à un état est décidée par StrongFather et exécutée par d'autres.

**Ce que cela signifie concrètement :**

- Caring Nanny ne peut jamais être la cause d'une modification du système
- Les notifications envoyées via BondingBrother sont informatives, pas directives
- Aucun "trigger automatique" ne peut être implémenté dans Caring Nanny

### 2.3. Non-autoritaire (INV-CN-3)

**Principe contractuel :**

L'invariant INV-CN-3 établit que Caring Nanny ne détient **aucune autorité** sur aucun aspect du système. Elle ne peut pas valider, invalider, accepter, ou refuser quoi que ce soit.

**Traduction en logique d'implémentation :**

- **Pas de veto** : Caring Nanny ne peut jamais bloquer une opération.
- **Pas de validation** : Caring Nanny ne valide pas les actions avant exécution.
- **Rôle consultatif** : Les autres cores peuvent consulter Caring Nanny, mais elle ne leur impose rien.

**Ce que cela signifie concrètement :**

- Caring Nanny ne peut jamais bloquer une opération ou imposer une contrainte
- StrongFather peut ignorer les informations de Caring Nanny sans violer aucun contrat
- Caring Nanny est un service d'information, pas une autorité de contrôle

### 2.4. État cohérent (INV-CN-4)

**Principe contractuel :**

L'invariant INV-CN-4 établit que l'état rapporté par Caring Nanny est **toujours cohérent**. Il n'y a jamais de contradiction dans l'état observé : si un composant est rapporté comme "healthy", il ne peut pas être simultanément rapporté comme "error".

**Traduction en logique d'implémentation :**

- **Unicité de l'état** : Un composant a exactement UN état à tout instant.
- **Agrégation déterministe** : L'état système global est calculé de manière déterministe.
- **Pas de contradiction** : Aucun consommateur ne peut recevoir des informations contradictoires.

**Ce que cela signifie concrètement :**

- L'agrégation des états partiels suit des règles de priorité strictes et documentées
- Les transitions d'état sont atomiques — pas d'état intermédiaire
- Les consommateurs de l'état peuvent se fier à la cohérence de l'information fournie

### 2.5. Traçabilité complète (INV-CN-5)

**Principe contractuel :**

L'invariant INV-CN-5 établit que chaque observation, chaque transition, chaque propagation est **entièrement traçable**. L'historique permet de reconstituer l'évolution de l'état du système dans le temps.

**Traduction en logique d'implémentation :**

- **Traçabilité systématique** : Chaque observation DOIT être enregistrée avec son contexte complet.
- **Historique complet** : L'historique conserve toutes les transitions et leurs causes.
- **Accessibilité audit** : L'historique DOIT être accessible pour audit et diagnostic.

**Ce que cela signifie concrètement :**

- Toute observation est tracée avec : source, timestamp, condition, état résultant
- L'audit et le diagnostic sont toujours possibles a posteriori
- L'historique est la mémoire fidèle de l'évolution du système

### 2.6. Non-bloquant (INV-CN-6)

**Principe contractuel :**

L'invariant INV-CN-6 établit que Caring Nanny ne bloque **jamais** les opérations du système. L'observation est passive et n'interfère pas avec le fonctionnement normal.

**Traduction en logique d'implémentation :**

- **Observation asynchrone** : L'observation ne doit pas bloquer les opérations observées.
- **Latence minimale** : L'impact de Caring Nanny sur les performances doit être négligeable.
- **Dégradation gracieuse** : Si Caring Nanny est indisponible, le système continue de fonctionner.

**Ce que cela signifie concrètement :**

- La présence de Caring Nanny n'a aucun impact sur les performances ou la disponibilité du système
- Les sondes d'observation sont passives et non intrusives
- Le système ne dépend pas de Caring Nanny pour fonctionner

### 2.7. Propagation fidèle (INV-CN-7)

**Principe contractuel :**

L'invariant INV-CN-7 établit que Caring Nanny propage les changements d'état **sans modification**. L'information transmise est exactement celle observée, sans interprétation, sans filtrage, sans transformation.

**Traduction en logique d'implémentation :**

- **Fidélité absolue** : Le message transmis est exactement celui observé.
- **Pas d'interprétation** : Caring Nanny ne traduit pas, n'interprète pas, ne filtre pas.
- **Transparence** : Les destinataires reçoivent l'information brute, pas une version éditoriale.

**Ce que cela signifie concrètement :**

- Les destinataires reçoivent une information fiable et non altérée
- Caring Nanny est un canal de transmission, pas un éditeur
- L'interprétation des états est la responsabilité des consommateurs

---

## 3. Comment traduire les contrats en logique sans interprétation abusive

### 3.1. Respecter les invariants comme contraintes absolues

**Principe :**

Les invariants contractuels (INV-CN-*) sont des contraintes absolues qui DOIVENT toujours être vraies. Ils ne sont pas des suggestions ou des recommandations.

**Traduction :**

- **Vérification systématique** : Chaque invariant DOIT être vérifié à chaque opération.
- **Préservation garantie** : Toute opération DOIT garantir que les invariants sont préservés après exécution.
- **Pas d'interprétation** : Les invariants ne peuvent pas être interprétés ou adaptés.

**Exemple conceptuel :**

Si l'invariant INV-CN-6 (non-bloquant) exige que Caring Nanny ne bloque jamais, alors aucune opération d'observation ne peut attendre une confirmation ou un acquittement avant de se terminer.

### 3.2. Implémenter l'observation comme acte passif

**Principe :**

L'observation est un acte **strictement passif**. Observer signifie détecter et enregistrer, jamais interagir ou modifier.

**Traduction :**

- **Sondes passives** : Les mécanismes d'observation n'interfèrent pas avec les composants observés.
- **Lecture seule** : Caring Nanny accède aux informations en lecture seule, jamais en écriture.
- **Sans effet de bord** : Aucune observation ne peut avoir d'effet sur le système observé.

**Exemple conceptuel :**

Observer l'état de KindMother signifie lire des métriques ou des indicateurs exposés par KindMother, pas interroger activement la base de données ou déclencher des opérations de diagnostic.

### 3.3. Traiter les cinq états comme exhaustifs et exclusifs

**Principe :**

Les cinq états (healthy, degraded, offline, syncing, error) sont exhaustifs et mutuellement exclusifs. Tout composant est dans exactement UN de ces états.

**Traduction :**

- **Exhaustivité** : Tout état observable DOIT être classifiable dans l'une des cinq catégories.
- **Exclusivité** : Aucun composant ne peut être dans deux états simultanément.
- **Pas d'extension** : Aucun nouvel état ne peut être ajouté sans modification du contrat FONDATION.

**Exemple conceptuel :**

Si un composant présente à la fois des symptômes de "degraded" et "syncing", les règles de priorité déterminent l'état unique à rapporter. Il n'y a pas d'état "degraded+syncing".

### 3.4. Implémenter la propagation comme transmission fidèle

**Principe :**

La propagation est une **transmission fidèle** d'information, pas une interprétation ou une recommandation.

**Traduction :**

- **Fidélité** : Le message propagé est exactement l'observation effectuée.
- **Pas de recommandation** : Caring Nanny ne suggère pas d'action, elle informe d'un état.
- **Traçabilité** : Chaque propagation est enregistrée avec ses destinataires.

**Exemple conceptuel :**

Quand Caring Nanny propage "KindMother est passée de healthy à degraded", elle ne dit pas "il faudrait vérifier KindMother" ou "l'utilisateur devrait être notifié". Elle transmet le fait brut.

---

## 4. Ce qu'un développeur ne doit jamais faire

### 4.1. Modifier des données (INV-CN-1)

**Interdiction contractuelle :**

L'invariant INV-CN-1 établit que Caring Nanny ne modifie **jamais** l'état du système qu'elle observe.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des mécanismes d'écriture de données dans Caring Nanny
- Permettre à Caring Nanny de modifier directement des configurations ou des états
- Créer des "corrections automatiques" exécutées par Caring Nanny
- Exposer des APIs de modification accessibles à Caring Nanny

**Conséquence de la violation :**

- Violation de l'invariant INV-CN-1 (observateur pur)
- Compromission de la séparation observation / action
- Perte de confiance dans la neutralité de Caring Nanny

### 4.2. Déclencher des actions (INV-CN-2)

**Interdiction contractuelle :**

L'invariant INV-CN-2 établit que Caring Nanny ne possède **aucune capacité d'exécution**.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer des triggers qui exécutent des actions basées sur les observations
- Implémenter des "réactions automatiques" aux changements d'état
- Lier des observations à des callbacks qui modifient le système
- Permettre à Caring Nanny de "réparer" automatiquement des anomalies

**Conséquence de la violation :**

- Violation de l'invariant INV-CN-2 (aucune capacité d'exécution)
- Caring Nanny devient une cause de modifications du système
- Confusion entre observation et action

### 4.3. Bloquer des opérations (INV-CN-3, INV-CN-6)

**Interdiction contractuelle :**

Les invariants INV-CN-3 et INV-CN-6 établissent que Caring Nanny ne détient aucune autorité et ne bloque jamais.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des mécanismes de validation obligatoire par Caring Nanny
- Créer des "gates" qui bloquent les opérations en attente d'observation
- Permettre à Caring Nanny de refuser ou d'invalider des actions
- Rendre le système dépendant de la disponibilité de Caring Nanny

**Conséquence de la violation :**

- Violation des invariants INV-CN-3 et INV-CN-6
- Caring Nanny devient un point de blocage du système
- Compromission de la disponibilité globale

### 4.4. Créer des états ambigus (INV-CN-4)

**Interdiction contractuelle :**

L'invariant INV-CN-4 établit que l'état rapporté est **toujours cohérent**, sans contradiction.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer des états intermédiaires ou de transition
- Permettre des états contradictoires simultanés
- Exposer des états "indéterminés" ou "inconnus"
- Implémenter des transitions non atomiques

**Conséquence de la violation :**

- Violation de l'invariant INV-CN-4 (état cohérent)
- Ambiguïté sur l'état réel du système
- Perte de confiance dans les informations fournies par Caring Nanny

### 4.5. Altérer l'historique (INV-CN-5)

**Interdiction contractuelle :**

L'invariant INV-CN-5 établit que l'historique est **entièrement traçable** et permet la reconstitution.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des mécanismes de modification de l'historique
- Permettre la suppression de traces, même "obsolètes"
- Créer des mécanismes de "correction" de l'historique
- Exposer des APIs de modification des enregistrements passés

**Conséquence de la violation :**

- Violation de l'invariant INV-CN-5 (traçabilité complète)
- Compromission de l'auditabilité du système
- Perte de confiance dans l'historique des observations

### 4.6. Prendre des décisions à la place de StrongFather

**Interdiction contractuelle :**

Caring Nanny fournit le contexte d'état, mais la décision d'agir appartient à StrongFather.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Implémenter des décisions d'autorisation dans Caring Nanny
- Créer des règles de blocage basées sur l'état observé
- Permettre à Caring Nanny de refuser des intentions
- Confondre "information d'état" et "décision d'autorisation"

**Conséquence de la violation :**

- Violation de la séparation des autorités entre cores
- Conflit d'autorité avec StrongFather
- Compromission de l'architecture de gouvernance

### 4.7. Modifier les données de KindMother

**Interdiction contractuelle :**

Caring Nanny **ne modifie jamais** les données gérées par KindMother.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Permettre à Caring Nanny d'écrire dans les données de KindMother
- Créer des "corrections de données" exécutées par Caring Nanny
- Accéder directement aux mécanismes de persistance de KindMother
- Implémenter des "mises à jour automatiques" basées sur les observations

**Conséquence de la violation :**

- Violation de l'autorité exclusive de KindMother sur les données
- Violation de l'invariant INV-CN-1 (observateur pur)
- Compromission de l'intégrité des données

---

## 5. Anti-patterns classiques

### 5.1. Anti-pattern 1 : Observation intrusive

**Description :**

Tentative d'implémenter des sondes d'observation qui interfèrent avec les composants observés ou qui nécessitent leur coopération active.

**Exemple conceptuel :**

Un développeur crée un mécanisme où Caring Nanny "ping" activement KindMother et attend une réponse synchrone, bloquant l'observation si KindMother est lente.

**Conséquence :**

- Violation de l'invariant INV-CN-6 (non-bloquant)
- Impact sur les performances des composants observés
- Dépendance de Caring Nanny à la disponibilité des composants

**Correction :**

Les sondes sont strictement passives. Elles lisent des métriques exposées ou des événements publiés, sans jamais interroger activement ni attendre de réponse.

### 5.2. Anti-pattern 2 : Réaction automatique

**Description :**

Tentative d'implémenter des réactions automatiques aux observations, comme redémarrer un composant ou notifier automatiquement un utilisateur.

**Exemple conceptuel :**

Un développeur crée un mécanisme où quand Caring Nanny détecte l'état "error", elle déclenche automatiquement un redémarrage du composant concerné.

**Conséquence :**

- Violation de l'invariant INV-CN-2 (aucune capacité d'exécution)
- Caring Nanny devient une cause de modifications du système
- Confusion entre observation et action

**Correction :**

Caring Nanny observe et propage l'information. La décision de redémarrer est prise par StrongFather, l'exécution est effectuée par le composant concerné ou un mécanisme dédié.

### 5.3. Anti-pattern 3 : États de transition

**Description :**

Tentative de créer des états intermédiaires pour gérer les transitions complexes entre états.

**Exemple conceptuel :**

Un développeur crée un état "transitioning" entre "healthy" et "degraded" pour représenter "en cours de dégradation".

**Conséquence :**

- Violation de l'invariant INV-CN-4 (état cohérent)
- Ambiguïté sur l'état réel du système
- Complexité inutile et risque d'états bloqués

**Correction :**

Les transitions sont atomiques. Un composant est healthy, puis instantanément degraded. Il n'y a pas d'état intermédiaire. La transition est un événement, pas un état.

### 5.4. Anti-pattern 4 : Validation obligatoire

**Description :**

Tentative de créer des mécanismes où les opérations doivent être "validées" par Caring Nanny avant exécution.

**Exemple conceptuel :**

Un développeur crée un mécanisme où une opération de KindMother ne peut s'exécuter que si Caring Nanny confirme que l'état est "healthy".

**Conséquence :**

- Violation de l'invariant INV-CN-3 (non-autoritaire) et INV-CN-6 (non-bloquant)
- Caring Nanny devient un point de blocage
- Dépendance à la disponibilité de Caring Nanny

**Correction :**

Caring Nanny informe de l'état, elle ne valide pas les opérations. StrongFather peut consulter l'état fourni par Caring Nanny pour décider, mais Caring Nanny ne peut pas bloquer.

### 5.5. Anti-pattern 5 : Filtrage de propagation

**Description :**

Tentative de filtrer ou modifier les informations d'état avant propagation pour "protéger" les consommateurs ou "simplifier" le message.

**Exemple conceptuel :**

Un développeur crée un mécanisme où Caring Nanny ne propage pas les transitions "mineures" ou édulcore les messages d'erreur pour ne pas "alarmer" les produits.

**Conséquence :**

- Violation de l'invariant INV-CN-7 (propagation fidèle)
- Perte d'information critique pour les consommateurs
- Compromission de la traçabilité

**Correction :**

La propagation est fidèle et complète. Caring Nanny transmet exactement ce qu'elle observe. Le filtrage, si nécessaire, est la responsabilité des consommateurs.

### 5.6. Anti-pattern 6 : Historique modifiable

**Description :**

Tentative de permettre la modification de l'historique pour "corriger des erreurs" ou "nettoyer les données obsolètes".

**Exemple conceptuel :**

Un développeur implémente une fonction "cleanHistory()" pour supprimer les anciennes observations jugées "inutiles".

**Conséquence :**

- Violation de l'invariant INV-CN-5 (traçabilité complète)
- Perte de la capacité d'audit
- Compromission de la confiance dans l'historique

**Correction :**

L'historique est strictement append-only. Les traces ne sont jamais modifiées ni supprimées. Si l'espace devient un problème, des mécanismes d'archivage (pas de suppression) peuvent être envisagés.

---

## 6. Bonnes pratiques conceptuelles

### 6.1. Sondes passives et non intrusives

**Pratique :**

Implémenter des sondes d'observation strictement passives qui n'interfèrent pas avec les composants observés.

**Justification :**

- Respecte l'invariant INV-CN-1 (observateur pur)
- Respecte l'invariant INV-CN-6 (non-bloquant)
- Garantit que l'observation n'a aucun effet de bord

**Implémentation conceptuelle :**

- Lecture de métriques exposées par les composants
- Écoute d'événements publiés par les composants
- Pas de requêtes actives ni d'interrogations synchrones
- Timeout courts pour éviter tout blocage

### 6.2. Agrégation déterministe avec règles de priorité

**Pratique :**

Implémenter l'agrégation des états partiels avec des règles de priorité claires et déterministes.

**Justification :**

- Respecte l'invariant INV-CN-4 (état cohérent)
- Garantit que le même ensemble de conditions produit toujours le même état
- Facilite l'audit et la compréhension

**Implémentation conceptuelle :**

- Règles de priorité documentées (ex: error > degraded > syncing > offline > healthy)
- Agrégation déterministe des états partiels
- Aucune ambiguïté dans le résultat
- Tests de reproductibilité

### 6.3. Historique append-only avec horodatage local

**Pratique :**

Implémenter l'historique comme une structure append-only avec horodatage local (conforme à LOI-4).

**Justification :**

- Respecte l'invariant INV-CN-5 (traçabilité complète)
- Respecte LOI-4 (pas de temps global requis)
- Garantit l'immuabilité de l'historique

**Implémentation conceptuelle :**

- Structure de données append-only (log immuable)
- Horodatage via le kernel Clock (local, pas synchronisé)
- Pas de mécanisme de suppression ou modification
- Indexation pour recherche rapide

### 6.4. Propagation asynchrone et non bloquante

**Pratique :**

Implémenter la propagation de manière asynchrone, sans attendre de confirmation des destinataires.

**Justification :**

- Respecte l'invariant INV-CN-6 (non-bloquant)
- Respecte l'invariant INV-CN-7 (propagation fidèle)
- Garantit que la propagation n'impacte pas les performances

**Implémentation conceptuelle :**

- Délégation à BondingBrother pour la distribution
- Pas d'attente de confirmation
- Enregistrement de la propagation pour traçabilité
- Fire-and-forget (avec traçabilité)

### 6.5. Distinction explicite offline vs error (LOI-2)

**Pratique :**

Distinguer explicitement l'état "offline" (isolement normal) de l'état "error" (anomalie).

**Justification :**

- Respecte LOI-2 (le système accepte l'isolement comme état normal)
- Évite de traiter l'isolation comme une erreur
- Facilite la gestion du mode déconnecté

**Implémentation conceptuelle :**

- L'état "offline" indique un fonctionnement normal sans connexion externe
- L'état "error" indique une anomalie qui empêche le fonctionnement correct
- Critères de classification documentés et sans ambiguïté
- Pas de confusion entre "isolé" et "en erreur"

### 6.6. Consultation sans effet de bord

**Pratique :**

Implémenter les interfaces de consultation de manière à garantir qu'aucune consultation ne modifie l'état.

**Justification :**

- Respecte l'invariant INV-CN-1 (observateur pur)
- Garantit la sécurité des consultations répétées
- Facilite la mise en cache

**Implémentation conceptuelle :**

- Interfaces de lecture seule pour toutes les consultations
- Aucun effet de bord lors de la lecture
- Réponses avec contexte (timestamp, source)
- Idempotence garantie

### 6.7. Métriques de fonctionnement légères (LOI-5)

**Pratique :**

Collecter des métriques de fonctionnement de Caring Nanny de manière légère et optimisée.

**Justification :**

- Respecte LOI-5 (le coût doit être proportionnel au hardware)
- Permet la supervision sans impact sur les performances
- Facilite le diagnostic de Caring Nanny elle-même

**Implémentation conceptuelle :**

- Métriques collectées de manière asynchrone
- Agrégation plutôt que logging exhaustif
- Rétention configurable selon les ressources disponibles
- Impact négligeable sur les performances

---

## 7. Check-list mentale avant toute feature

Avant d'implémenter une nouvelle fonctionnalité liée à Caring Nanny, un développeur DOIT vérifier mentalement :

### 7.1. Vérification des invariants

- **INV-CN-1 est-il préservé ?** : La fonctionnalité n'observe-t-elle que passivement, sans modifier ?
- **INV-CN-2 est-il préservé ?** : Aucune action n'est-elle déclenchée par la fonctionnalité ?
- **INV-CN-3 est-il préservé ?** : La fonctionnalité n'impose-t-elle aucune autorité ou blocage ?
- **INV-CN-4 est-il préservé ?** : L'état reste-t-il toujours cohérent et sans ambiguïté ?
- **INV-CN-5 est-il préservé ?** : La traçabilité est-elle complète et l'historique immuable ?
- **INV-CN-6 est-il préservé ?** : La fonctionnalité est-elle non bloquante ?
- **INV-CN-7 est-il préservé ?** : La propagation est-elle fidèle, sans altération ?

### 7.2. Vérification de la séparation des responsabilités

- **Caring Nanny reste-t-elle observatrice ?** : La fonctionnalité n'exécute-t-elle rien ?
- **L'autorité de KindMother est-elle respectée ?** : Aucune modification de données ?
- **L'autorité de StrongFather est-elle respectée ?** : Aucune décision d'autorisation ?
- **La collaboration avec BondingBrother est-elle passive ?** : Information seulement, pas de médiation ?

### 7.3. Vérification de la conformité aux Lois d'Autonomie

- **LOI-1 respectée ?** : Aucune dépendance externe critique pour l'observation ?
- **LOI-2 respectée ?** : L'isolation est-elle reconnue comme état normal (offline ≠ error) ?
- **LOI-4 respectée ?** : L'horodatage est-il local, sans temps global requis ?
- **LOI-5 respectée ?** : Le coût est-il proportionnel aux ressources disponibles ?

### 7.4. Vérification de la traçabilité

- **Toutes les observations sont-elles tracées ?** : Aucune observation silencieuse ?
- **Les traces sont-elles immuables ?** : Aucune modification possible ?
- **Les traces sont-elles accessibles ?** : Audit possible ?

### 7.5. Vérification des flux

- **Le flux d'observation est-il respecté ?** : Détection → Évaluation → Agrégation → Transition ?
- **Le flux de propagation est-il respecté ?** : Changement → Destinataires → Message → Dispatch ?
- **Le flux de consultation est-il respecté ?** : Demande → Réponse → Aucune modification ?

---

## 8. Conclusion

Ce document fournit des lignes directrices pour implémenter Caring Nanny de manière conforme aux contrats FONDATION.

**Points clés :**

- Caring Nanny **observe, détecte, classe, et propage** — elle **ne modifie jamais, ne décide jamais, n'exécute jamais**
- Les invariants INV-CN-1 à INV-CN-7 sont des **contraintes absolues**
- La **traçabilité est immuable** et la **propagation est fidèle**
- La **séparation observation / action** est fondamentale
- Les **Lois d'Autonomie** doivent être respectées
- L'état **offline** (isolation) est normal, distinct de **error** (anomalie)

**Nature informative :**

Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à guider la compréhension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se référer à la Documentation Fondatrice et aux contrats spécifiques.

**Phrase fondatrice à garder en mémoire :**

> **Caring Nanny est l'observateur d'état privilégié du système, fournissant une vision cohérente et traçable de l'état global et des transitions, sans jamais modifier, décider, ou exécuter.**

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System, Caring Nanny Documentation Fondatrice, Tous les contrats FONDATION  
**Type :** Guide d'implémentation informatif

---

## 9. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Niveau de détail des exemples

**Arbitrage rencontré :** Quel niveau de détail donner aux exemples sans prescrire d'implémentation technique ?

**Décision prise :** Les exemples restent purement conceptuels et narratifs. Aucun code, aucune structure de données spécifique.

**Justification :** Ce document est informatif et non normatif. Les choix techniques appartiennent aux équipes d'implémentation.

**Documentation :** Sections 5 (anti-patterns) et 6 (bonnes pratiques) avec exemples conceptuels uniquement.

### Arbitrage A2 : Références aux Lois d'Autonomie

**Arbitrage rencontré :** Quelles lois d'autonomie sont les plus pertinentes pour Caring Nanny ?

**Décision prise :** Emphase sur LOI-1 (aucune dépendance externe), LOI-2 (isolation acceptée), LOI-4 (pas de temps global), et LOI-5 (coût proportionnel).

**Justification :** Ces quatre lois sont les plus directement applicables à la nature d'observateur passif de Caring Nanny.

**Documentation :** Sections 1.4, 6.3, 6.5, 6.7 et 7.3.

### Arbitrage A3 : Distinction offline vs error

**Arbitrage rencontré :** Comment traiter l'extension requise pour LOI-2 mentionnée dans la Documentation Fondatrice ?

**Décision prise :** Inclure cette distinction comme bonne pratique explicite et élément de vérification.

**Justification :** L'extension est documentée dans la Documentation Fondatrice comme nécessaire. Ce guide doit en faciliter l'application.

**Documentation :** Section 6.5 dédiée à cette distinction.

### Arbitrage A4 : Exhaustivité de la check-list

**Arbitrage rencontré :** La check-list avec tous les invariants et vérifications est-elle trop longue ?

**Décision prise :** Conserver la liste complète car chaque vérification est importante. Organiser par catégorie pour faciliter la lecture.

**Justification :** Omettre des vérifications de la check-list risquerait de les faire oublier. L'organisation par catégorie aide à la mémorisation.

**Documentation :** Section 7 avec vérifications organisées par thème.

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
