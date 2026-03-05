# MiyuClock â€” Outils de mesure du temps (rÃ©fÃ©rence exhaustive)

## Contexte

Ce document constitue la **rÃ©fÃ©rence prÃ©cise et exhaustive** des **outils de mesure du temps** exposÃ©s par le kit **MiyuClock** : sÃ©mantique (attestation horaire et date IRL), description dÃ©taillÃ©e de chaque outil, types d'entrÃ©e/sortie, relation avec le Kernel (Clock), cas d'usage et conformitÃ© LOI-4. Il permet d'exploiter MiyuClock dans les services consommateurs (JayKoa, JayFestival, etc.) en clarifiant le rÃ´le : **MiyuClock atteste l'horaire et la date IRL** ; **JayKoa organise les donnÃ©es et fait l'interface avec l'utilisateur**.

**RÃ©fÃ©rences** : [MiyuClock - Documentation Fondatrice](../MiyuClock%20-%20Documentation%20Fondatrice.md), [MiyuClock - Reference Outils](../MiyuClock%20-%20Reference%20Outils.md), [MiyuClock - Runtime Boundary Contract](../contracts/boundaries/MiyuClock%20-%20Runtime%20Boundary%20Contract.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Outils de mesure du temps (tool.time.now, tool.time.delta), sÃ©mantique Â« attestation Â» temps IRL, contrat avec le Kernel Clock, types et unitÃ©s, cas d'usage, distinction MiyuClock vs JayKoa.
- **Hors pÃ©rimÃ¨tre** : ImplÃ©mentation technique du trait Clock (Kernel), persistance des timestamps (OpÃ©rateur + KindMother/MiyuSQL).

---

## 1. SÃ©mantique : attestation horaire et date IRL

### 1.1 RÃ´le de MiyuClock

**MiyuClock atteste l'horaire et la date IRL** (In Real Life) : il fournit une **rÃ©fÃ©rence temporelle** issue de l'horloge locale (Kernel Clock), sans imposer de timezone ni dÃ©pendre d'un temps global. Les services consommateurs utilisent cette attestation pour :

- **Afficher** l'heure ou la date courante (ex. Â« Il est 14h30 Â»).
- **Comparer** des instants (avant/aprÃ¨s, durÃ©e Ã©coulÃ©e).
- **Auditer** ou tracer un instant (horodatage de trace, sans persistance par MiyuClock).
- **Calculer** une durÃ©e entre deux instants fournis dans le flux.

**MiyuClock ne fait pas** : organiser des donnÃ©es agenda, afficher un calendrier, gÃ©rer des fuseaux pour l'affichage utilisateur, persister des timestamps. Ces rÃ´les relÃ¨vent des **OpÃ©rateurs** et des services comme **JayKoa** (organisation des donnÃ©es et interface utilisateur).

### 1.2 MiyuClock vs JayKoa (rÃ´les distincts)

| Composant | RÃ´le |
|-----------|------|
| **MiyuClock** | **Atteste l'horaire et la date IRL** : fournit l'instant prÃ©sent (`tool.time.now`) et la durÃ©e entre deux instants (`tool.time.delta`). RÃ©fÃ©rentiel temps rÃ©el local ; pas de persistance, pas de temps global (LOI-4). |
| **JayKoa** | **Organise les donnÃ©es** (entrÃ©es agenda, Ã©ditions, participations, crÃ©neaux) et **fait l'interface avec l'utilisateur** (vue calendrier, conflits, export, fuseaux d'affichage). Utilise MiyuClock pour l'attestation temporelle lorsque nÃ©cessaire (ex. Â« maintenant Â» pour filtrer les Ã©vÃ©nements Ã  venir). |

**RÃ©fÃ©rence** : [Miyukini Conceptual References - Interpolarite Services Jay](..//..//..//miyukini-webway-system//reference//_index.md) (Â§ 4.1 JayKoa vs MiyuClock).

---

## 2. Outils composants (dÃ©tail exhaustif)

### 2.1 tool.time.now â€” Instant prÃ©sent

| Ã‰lÃ©ment | Description |
|--------|-------------|
| **ToolId** | `tool.time.now` |
| **Nom lisible** | Instant prÃ©sent |
| **Action** | Retourne l'**instant prÃ©sent** selon l'horloge locale fournie par le **Kernel (Clock)**. |
| **EntrÃ©es** | Aucune (contexte gouvernÃ© fourni par le flux). |
| **Sortie** | Valeur d'**instant** (type : horodatage local ; reprÃ©sentation dÃ©finie par l'implÃ©mentation â€” ex. epoch, ISO 8601, ou structure { secs, nanos }). Aucune timezone imposÃ©e. |
| **Niveau de sÃ©curitÃ©** | 0 ou 1 |
| **Capability_id** | `time.now` |
| **Contrat Kernel** | MiyuClock appelle le trait **Clock** du Kernel pour obtenir l'instant courant. Le Kernel est la **seule** source de temps ; aucune dÃ©pendance Ã  NTP ou serveur de temps externe (LOI-4). |
| **PrÃ©cision** | DÃ©pend de l'implÃ©mentation du Kernel (Clock). La documentation ne fixe pas de prÃ©cision minimale ; l'usage typique est la trace, l'affichage ou la comparaison d'instants. |
| **Persistance** | MiyuClock **ne persiste pas**. Si l'OpÃ©rateur ou un service doit enregistrer un timestamp, il utilise la valeur retournÃ©e et la transmet Ã  KindMother (WriteIntent) ou Ã  une table via le flux applicatif. |

**Cas d'usage** : Affichage Â« Il est 14h30 Â» ; filtre Â« Ã©vÃ©nements aprÃ¨s maintenant Â» ; horodatage de trace (logs, audit) cÃ´tÃ© OpÃ©rateur ; rÃ©fÃ©rence pour calcul de delta.

---

### 2.2 tool.time.delta â€” Delta entre instants

| Ã‰lÃ©ment | Description |
|--------|-------------|
| **ToolId** | `tool.time.delta` |
| **Nom lisible** | Delta entre instants |
| **Action** | Retourne la **durÃ©e Ã©coulÃ©e** entre deux instants fournis **dans le flux** (t_prev, t_now ou rÃ©fÃ©rences Ã©quivalentes). |
| **EntrÃ©es** | **Deux instants** (types compatibles avec la sortie de `tool.time.now` ou rÃ©fÃ©rences) : `t_prev` (instant antÃ©rieur), `t_now` (instant postÃ©rieur ou courant). L'ordre n'est pas imposÃ© par le contrat : la durÃ©e retournÃ©e est une **grandeur positive** (abs(diffÃ©rence)). |
| **Sortie** | **DurÃ©e** (delta) : reprÃ©sentation dÃ©finie par l'implÃ©mentation (ex. secondes, millisecondes, ou structure { secs, nanos }). UnitÃ© cohÃ©rente avec les entrÃ©es. |
| **Niveau de sÃ©curitÃ©** | 0 ou 1 |
| **Capability_id** | `time.delta` |
| **DÃ©cision mÃ©tier** | MiyuClock **ne dÃ©cide pas** (ex. ne dÃ©cide pas si une rÃ©servation est Â« expirÃ©e Â»). Il fournit la durÃ©e ; l'OpÃ©rateur ou le service interprÃ¨te (ex. Â« si delta > 24h alors considÃ©rer expirÃ© Â»). |
| **Persistance** | MiyuClock **ne persiste pas**. La durÃ©e calculÃ©e est fournie dans le flux pour usage par l'OpÃ©rateur. |

**Cas d'usage** : Calcul Â« temps restant avant l'Ã©vÃ©nement Â» ; durÃ©e de session ; comparaison Â« il y a X minutes Â» ; audit de durÃ©e entre deux Ã©vÃ©nements.

---

## 3. Contrat avec le Kernel (Clock)

| Aspect | Description |
|--------|-------------|
| **Source de temps** | Le **Kernel** fournit le trait **Clock** (trace / horodatage local). MiyuClock s'appuie sur ce trait pour `tool.time.now` et, indirectement, pour les instants fournis Ã  `tool.time.delta` (qui peuvent provenir d'appels antÃ©rieurs Ã  `tool.time.now` ou d'autres sources conformes). |
| **Pas de temps global** | ConformitÃ© **LOI-4** : aucune dÃ©pendance Ã  un temps global (NTP, serveur de temps externe). L'horloge est **locale** au nÅ“ud. |
| **Pas de timezone imposÃ©e** | MiyuClock n'impose aucune timezone. L'instant retournÃ© par `tool.time.now` est une **rÃ©fÃ©rence locale**. L'affichage ou l'interprÃ©tation en fuseau (ex. Â« 14h30 Paris Â») relÃ¨ve de l'OpÃ©rateur ou du service (ex. JayKoa, MiyuLocale). |
| **Limite** | MiyuClock ne remplace pas le Clock du Kernel ; il **expose** la mesure du temps aux OpÃ©rateurs via la gouvernance (Master Butler, BondingBrother). |

**RÃ©fÃ©rence** : [MiyuClock - Runtime Boundary Contract](../contracts/boundaries/MiyuClock%20-%20Runtime%20Boundary%20Contract.md).

---

## 4. Types et unitÃ©s (orientation implÃ©mentation)

| Concept | Orientation |
|---------|-------------|
| **Instant** | ReprÃ©sentation **locale** (epoch, ou ISO 8601 sans timezone, ou structure secs+nanos). Pas de timezone dans la sortie MiyuClock ; l'implÃ©mentation peut choisir une reprÃ©sentation interopÃ©rable. |
| **DurÃ©e (delta)** | UnitÃ© **cohÃ©rente** avec les entrÃ©es (ex. secondes, millisecondes). La sortie est une **grandeur positive** (durÃ©e Ã©coulÃ©e entre les deux instants). |
| **PrÃ©cision** | Non fixÃ©e par le contrat ; dÃ©pend du Kernel (Clock). Suffisante pour les cas d'usage (affichage, comparaison, audit, durÃ©e). |

---

## 5. Cas d'usage (rÃ©sumÃ©)

| Cas d'usage | Outil(s) | RÃ´le MiyuClock |
|-------------|----------|-----------------|
| Affichage heure courante | `tool.time.now` | Fournir l'instant prÃ©sent ; l'UI ou JayKoa formate et affiche (fuseau, locale). |
| Ã‰vÃ©nements Â« Ã  venir Â» (aprÃ¨s maintenant) | `tool.time.now` | Fournir Â« maintenant Â» ; le service compare avec les dates des Ã©vÃ©nements (donnÃ©es organisÃ©es par JayKoa). |
| Temps restant avant un Ã©vÃ©nement | `tool.time.now` + `tool.time.delta` | Maintenant + delta(now, date_Ã©vÃ©nement) ; le service affiche la durÃ©e. |
| DurÃ©e Ã©coulÃ©e entre deux Ã©vÃ©nements | `tool.time.delta` | Delta(t1, t2) fourni ; le service affiche ou enregistre. |
| Horodatage de trace / audit | `tool.time.now` | L'OpÃ©rateur obtient l'instant et le transmet Ã  KindMother ou Ã  un log (sans que MiyuClock persiste). |

---

## 6. ConformitÃ© LOI-4 et invariants

| RÃ¨gle | Application |
|-------|-------------|
| **LOI-4** (pas de temps global requis) | MiyuClock ne dÃ©pend d'aucun serveur de temps externe. Horloge locale (Kernel Clock) uniquement. |
| **Pas de persistance** | MiyuClock ne lit ni n'Ã©crit en base. Toute persistance de timestamps = OpÃ©rateur + KindMother/MiyuSQL. |
| **Pas de dÃ©cision mÃ©tier** | MiyuClock ne dÃ©cide pas (ALLOW/DENY = StrongFather). Il fournit des valeurs de temps. |
| **Pas de timezone imposÃ©e** | L'instant est une rÃ©fÃ©rence locale ; l'interprÃ©tation fuseau reste au consommateur. |

**RÃ©fÃ©rence** : [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md) (LOI-4).

---

## 7. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [MiyuClock - Documentation Fondatrice](../MiyuClock%20-%20Documentation%20Fondatrice.md) | IdentitÃ©, ToolkitId, liste Tools, gouvernance, LOI-4. |
| [MiyuClock - Reference Outils](../MiyuClock%20-%20Reference%20Outils.md) | Tableau ToolIds, action, niveau sÃ©curitÃ©. |
| [MiyuClock - Runtime Boundary Contract](../contracts/boundaries/MiyuClock%20-%20Runtime%20Boundary%20Contract.md) | Bornage, frontiÃ¨re Kernel (Clock), interdictions. |
| [MiyuClock - Security and States Contract](../contracts/security/MiyuClock%20-%20Security%20and%20States%20Contract.md) | Niveau sÃ©curitÃ©, Ã©tats autorisÃ©s/interdits. |
| [Miyukini Conceptual References - Interpolarite Services Jay](..//..//..//miyukini-webway-system//reference//_index.md) | Â§ 4.1 JayKoa vs MiyuClock (rÃ´les). |

---

**Document** : MiyuClock â€” Outils de mesure du temps (rÃ©fÃ©rence exhaustive)  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de rÃ©fÃ©rence â€” outils mesure du temps

