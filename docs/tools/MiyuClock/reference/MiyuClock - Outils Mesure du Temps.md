# MiyuClock — Outils de mesure du temps (référence exhaustive)

## Contexte

Ce document constitue la **référence précise et exhaustive** des **outils de mesure du temps** exposés par le kit **MiyuClock** : sémantique (attestation horaire et date IRL), description détaillée de chaque outil, types d'entrée/sortie, relation avec le Kernel (Clock), cas d'usage et conformité LOI-4. Il permet d'exploiter MiyuClock dans les services consommateurs (JayKoa, JayFestival, etc.) en clarifiant le rôle : **MiyuClock atteste l'horaire et la date IRL** ; **JayKoa organise les données et fait l'interface avec l'utilisateur**.

**Références** : [MiyuClock - Documentation Fondatrice](../MiyuClock%20-%20Documentation%20Fondatrice.md), [MiyuClock - Reference Outils](../MiyuClock%20-%20Reference%20Outils.md), [MiyuClock - Runtime Boundary Contract](../contracts/boundaries/MiyuClock%20-%20Runtime%20Boundary%20Contract.md).

## Portée / Scope

- **Périmètre** : Outils de mesure du temps (tool.time.now, tool.time.delta), sémantique « attestation » temps IRL, contrat avec le Kernel Clock, types et unités, cas d'usage, distinction MiyuClock vs JayKoa.
- **Hors périmètre** : Implémentation technique du trait Clock (Kernel), persistance des timestamps (Opérateur + KindMother/MiyuSQL).

---

## 1. Sémantique : attestation horaire et date IRL

### 1.1 Rôle de MiyuClock

**MiyuClock atteste l'horaire et la date IRL** (In Real Life) : il fournit une **référence temporelle** issue de l'horloge locale (Kernel Clock), sans imposer de timezone ni dépendre d'un temps global. Les services consommateurs utilisent cette attestation pour :

- **Afficher** l'heure ou la date courante (ex. « Il est 14h30 »).
- **Comparer** des instants (avant/après, durée écoulée).
- **Auditer** ou tracer un instant (horodatage de trace, sans persistance par MiyuClock).
- **Calculer** une durée entre deux instants fournis dans le flux.

**MiyuClock ne fait pas** : organiser des données agenda, afficher un calendrier, gérer des fuseaux pour l'affichage utilisateur, persister des timestamps. Ces rôles relèvent des **Opérateurs** et des services comme **JayKoa** (organisation des données et interface utilisateur).

### 1.2 MiyuClock vs JayKoa (rôles distincts)

| Composant | Rôle |
|-----------|------|
| **MiyuClock** | **Atteste l'horaire et la date IRL** : fournit l'instant présent (`tool.time.now`) et la durée entre deux instants (`tool.time.delta`). Référentiel temps réel local ; pas de persistance, pas de temps global (LOI-4). |
| **JayKoa** | **Organise les données** (entrées agenda, éditions, participations, créneaux) et **fait l'interface avec l'utilisateur** (vue calendrier, conflits, export, fuseaux d'affichage). Utilise MiyuClock pour l'attestation temporelle lorsque nécessaire (ex. « maintenant » pour filtrer les événements à venir). |

**Référence** : [Miyukini Conceptual References - Interpolarite Services Jay](../../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) (§ 4.1 JayKoa vs MiyuClock).

---

## 2. Outils composants (détail exhaustif)

### 2.1 tool.time.now — Instant présent

| Élément | Description |
|--------|-------------|
| **ToolId** | `tool.time.now` |
| **Nom lisible** | Instant présent |
| **Action** | Retourne l'**instant présent** selon l'horloge locale fournie par le **Kernel (Clock)**. |
| **Entrées** | Aucune (contexte gouverné fourni par le flux). |
| **Sortie** | Valeur d'**instant** (type : horodatage local ; représentation définie par l'implémentation — ex. epoch, ISO 8601, ou structure { secs, nanos }). Aucune timezone imposée. |
| **Niveau de sécurité** | 0 ou 1 |
| **Capability_id** | `time.now` |
| **Contrat Kernel** | MiyuClock appelle le trait **Clock** du Kernel pour obtenir l'instant courant. Le Kernel est la **seule** source de temps ; aucune dépendance à NTP ou serveur de temps externe (LOI-4). |
| **Précision** | Dépend de l'implémentation du Kernel (Clock). La documentation ne fixe pas de précision minimale ; l'usage typique est la trace, l'affichage ou la comparaison d'instants. |
| **Persistance** | MiyuClock **ne persiste pas**. Si l'Opérateur ou un service doit enregistrer un timestamp, il utilise la valeur retournée et la transmet à KindMother (WriteIntent) ou à une table via le flux applicatif. |

**Cas d'usage** : Affichage « Il est 14h30 » ; filtre « événements après maintenant » ; horodatage de trace (logs, audit) côté Opérateur ; référence pour calcul de delta.

---

### 2.2 tool.time.delta — Delta entre instants

| Élément | Description |
|--------|-------------|
| **ToolId** | `tool.time.delta` |
| **Nom lisible** | Delta entre instants |
| **Action** | Retourne la **durée écoulée** entre deux instants fournis **dans le flux** (t_prev, t_now ou références équivalentes). |
| **Entrées** | **Deux instants** (types compatibles avec la sortie de `tool.time.now` ou références) : `t_prev` (instant antérieur), `t_now` (instant postérieur ou courant). L'ordre n'est pas imposé par le contrat : la durée retournée est une **grandeur positive** (abs(différence)). |
| **Sortie** | **Durée** (delta) : représentation définie par l'implémentation (ex. secondes, millisecondes, ou structure { secs, nanos }). Unité cohérente avec les entrées. |
| **Niveau de sécurité** | 0 ou 1 |
| **Capability_id** | `time.delta` |
| **Décision métier** | MiyuClock **ne décide pas** (ex. ne décide pas si une réservation est « expirée »). Il fournit la durée ; l'Opérateur ou le service interprète (ex. « si delta > 24h alors considérer expiré »). |
| **Persistance** | MiyuClock **ne persiste pas**. La durée calculée est fournie dans le flux pour usage par l'Opérateur. |

**Cas d'usage** : Calcul « temps restant avant l'événement » ; durée de session ; comparaison « il y a X minutes » ; audit de durée entre deux événements.

---

## 3. Contrat avec le Kernel (Clock)

| Aspect | Description |
|--------|-------------|
| **Source de temps** | Le **Kernel** fournit le trait **Clock** (trace / horodatage local). MiyuClock s'appuie sur ce trait pour `tool.time.now` et, indirectement, pour les instants fournis à `tool.time.delta` (qui peuvent provenir d'appels antérieurs à `tool.time.now` ou d'autres sources conformes). |
| **Pas de temps global** | Conformité **LOI-4** : aucune dépendance à un temps global (NTP, serveur de temps externe). L'horloge est **locale** au nœud. |
| **Pas de timezone imposée** | MiyuClock n'impose aucune timezone. L'instant retourné par `tool.time.now` est une **référence locale**. L'affichage ou l'interprétation en fuseau (ex. « 14h30 Paris ») relève de l'Opérateur ou du service (ex. JayKoa, MiyuLocale). |
| **Limite** | MiyuClock ne remplace pas le Clock du Kernel ; il **expose** la mesure du temps aux Opérateurs via la gouvernance (Master Butler, BondingBrother). |

**Référence** : [MiyuClock - Runtime Boundary Contract](../contracts/boundaries/MiyuClock%20-%20Runtime%20Boundary%20Contract.md).

---

## 4. Types et unités (orientation implémentation)

| Concept | Orientation |
|---------|-------------|
| **Instant** | Représentation **locale** (epoch, ou ISO 8601 sans timezone, ou structure secs+nanos). Pas de timezone dans la sortie MiyuClock ; l'implémentation peut choisir une représentation interopérable. |
| **Durée (delta)** | Unité **cohérente** avec les entrées (ex. secondes, millisecondes). La sortie est une **grandeur positive** (durée écoulée entre les deux instants). |
| **Précision** | Non fixée par le contrat ; dépend du Kernel (Clock). Suffisante pour les cas d'usage (affichage, comparaison, audit, durée). |

---

## 5. Cas d'usage (résumé)

| Cas d'usage | Outil(s) | Rôle MiyuClock |
|-------------|----------|-----------------|
| Affichage heure courante | `tool.time.now` | Fournir l'instant présent ; l'UI ou JayKoa formate et affiche (fuseau, locale). |
| Événements « à venir » (après maintenant) | `tool.time.now` | Fournir « maintenant » ; le service compare avec les dates des événements (données organisées par JayKoa). |
| Temps restant avant un événement | `tool.time.now` + `tool.time.delta` | Maintenant + delta(now, date_événement) ; le service affiche la durée. |
| Durée écoulée entre deux événements | `tool.time.delta` | Delta(t1, t2) fourni ; le service affiche ou enregistre. |
| Horodatage de trace / audit | `tool.time.now` | L'Opérateur obtient l'instant et le transmet à KindMother ou à un log (sans que MiyuClock persiste). |

---

## 6. Conformité LOI-4 et invariants

| Règle | Application |
|-------|-------------|
| **LOI-4** (pas de temps global requis) | MiyuClock ne dépend d'aucun serveur de temps externe. Horloge locale (Kernel Clock) uniquement. |
| **Pas de persistance** | MiyuClock ne lit ni n'écrit en base. Toute persistance de timestamps = Opérateur + KindMother/MiyuSQL. |
| **Pas de décision métier** | MiyuClock ne décide pas (ALLOW/DENY = StrongFather). Il fournit des valeurs de temps. |
| **Pas de timezone imposée** | L'instant est une référence locale ; l'interprétation fuseau reste au consommateur. |

**Référence** : [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) (LOI-4).

---

## 7. Références

| Document | Rôle |
|----------|------|
| [MiyuClock - Documentation Fondatrice](../MiyuClock%20-%20Documentation%20Fondatrice.md) | Identité, ToolkitId, liste Tools, gouvernance, LOI-4. |
| [MiyuClock - Reference Outils](../MiyuClock%20-%20Reference%20Outils.md) | Tableau ToolIds, action, niveau sécurité. |
| [MiyuClock - Runtime Boundary Contract](../contracts/boundaries/MiyuClock%20-%20Runtime%20Boundary%20Contract.md) | Bornage, frontière Kernel (Clock), interdictions. |
| [MiyuClock - Security and States Contract](../contracts/security/MiyuClock%20-%20Security%20and%20States%20Contract.md) | Niveau sécurité, états autorisés/interdits. |
| [Miyukini Conceptual References - Interpolarite Services Jay](../../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) | § 4.1 JayKoa vs MiyuClock (rôles). |

---

**Document** : MiyuClock — Outils de mesure du temps (référence exhaustive)  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de référence — outils mesure du temps
