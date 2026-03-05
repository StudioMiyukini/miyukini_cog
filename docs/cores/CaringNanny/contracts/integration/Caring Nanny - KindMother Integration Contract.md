# Caring Nanny - KindMother Integration Contract

## 1. Contexte

Ce document dÃ©finit le **contrat d'intÃ©gration entre Caring Nanny et KindMother**. Il spÃ©cifie l'interface d'observation, le protocole, les types de donnÃ©es observÃ©es, et les garanties associÃ©es Ã  l'observation de KindMother en tant qu'autoritÃ© des donnÃ©es.

Ce document complÃ¨te la Section 3 de la [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [Caring Nanny - Architecture et Composants](../../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md) pour l'architecture d'observation
- [Caring Nanny - Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) pour les invariants d'observation
- [KindMother - Documentation Fondatrice](../../../KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md) pour la nature de KindMother
- [Connexion Inter-COG](..//..//..//..//miyukini-webway-system//reference//_index.md) pour le contexte inter-composants

L'intÃ©gration respecte les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) : l'observation fonctionne localement sans dÃ©pendance externe (**LOI-1**), et l'Ã©tat offline est reconnu comme normal (**LOI-2**).

## 2. PortÃ©e / Scope

Ce document couvre :
- L'interface d'observation entre Caring Nanny et KindMother
- Le protocole d'observation (unidirectionnel, passif)
- Les types de donnÃ©es observÃ©es depuis KindMother
- Les Ã©tats dÃ©rivÃ©s de l'observation de KindMother
- Les rÃ¨gles d'intÃ©gration spÃ©cifiques
- La gestion des Ã©tats et transitions
- Les garanties de l'intÃ©gration

Ce document **ne couvre pas** :
- Les dÃ©tails internes de KindMother (voir documentation KindMother)
- Les flux de propagation vers BondingBrother (voir BondingBrother Integration Contract)
- Les dÃ©cisions basÃ©es sur l'Ã©tat observÃ© (voir StrongFather Integration Contract)
- Le modÃ¨le d'Ã©tat global (voir State Model Contract)

---

## 3. Principe fondamental

**Caring Nanny observe KindMother sans jamais interagir avec elle. La relation est strictement unidirectionnelle : KindMother produit des faits sur les donnÃ©es, Caring Nanny observe l'Ã©tat de ces donnÃ©es. Caring Nanny ne modifie jamais, ne dÃ©clenche jamais, ne valide jamais.**

La relation est asymÃ©trique : KindMother gÃ¨re les donnÃ©es et leur persistance, Caring Nanny observe passivement les signaux d'Ã©tat Ã©mis par KindMother sans jamais influencer son fonctionnement.

---

## 4. Nature de la relation Caring Nanny â€” KindMother

### 4.1 Relation d'observation pure

**Caring Nanny est un observateur passif de KindMother :**
- Elle observe les signaux d'Ã©tat Ã©mis par KindMother
- Elle dÃ©tecte les transitions d'Ã©tat de la persistance
- Elle agrÃ¨ge les informations en Ã©tat cohÃ©rent
- Elle ne sollicite jamais KindMother pour des opÃ©rations

**RÃ¨gle CN-KM-01 : Observation sans interaction**

Caring Nanny ne produit jamais de demande vers KindMother. Elle observe les signaux Ã©mis, elle ne provoque pas d'Ã©mission.

**RÃ¨gle CN-KM-02 : Aucune capacitÃ© d'Ã©criture**

Caring Nanny ne peut jamais modifier les donnÃ©es gÃ©rÃ©es par KindMother. Aucun WriteIntent, aucune modification, aucun delta ne peut Ãªtre Ã©mis par Caring Nanny.

**RÃ¨gle CN-KM-03 : Aucune influence sur la synchronisation**

Caring Nanny ne peut jamais dÃ©clencher, suspendre, ou modifier une synchronisation entre DB MÃ¨re et DB Filles. Elle observe l'Ã©tat de synchronisation, elle n'agit jamais sur lui.

### 4.2 SÃ©paration des responsabilitÃ©s

| ResponsabilitÃ© | Caring Nanny | KindMother |
|----------------|--------------|------------|
| **GÃ©rer les donnÃ©es** | âŒ Jamais | âœ… Exclusif |
| **Observer l'Ã©tat des donnÃ©es** | âœ… Exclusif | âŒ |
| **DÃ©clencher la synchronisation** | âŒ Jamais | âœ… Exclusif |
| **Observer l'Ã©tat de synchronisation** | âœ… Exclusif | âŒ |
| **Valider les WriteIntent** | âŒ Jamais | âœ… Exclusif |
| **DÃ©tecter les anomalies de persistance** | âœ… Exclusif | âŒ |
| **Propager l'Ã©tat observÃ©** | âœ… Exclusif | âŒ |

**RÃ¨gle CN-KM-04 : Aucun chevauchement d'autoritÃ©**

Caring Nanny n'a aucune autoritÃ© sur les donnÃ©es. KindMother n'a aucune responsabilitÃ© de propagation d'Ã©tat. Les domaines sont strictement sÃ©parÃ©s.

---

## 5. DonnÃ©es observÃ©es depuis KindMother

### 5.1 Ã‰tat de santÃ© de la persistance

**PERSISTENCE_HEALTH**
- **Objet d'observation :** DisponibilitÃ© et fonctionnement de la couche de persistance
- **Valeurs possibles :** `available`, `degraded`, `unavailable`
- **Signaux observÃ©s :** Temps de rÃ©ponse, erreurs de lecture/Ã©criture, intÃ©gritÃ© des fichiers

**RÃ¨gle CN-KM-OBS-01 : Observation non intrusive**

L'observation de la santÃ© de la persistance n'interfÃ¨re pas avec les opÃ©rations de KindMother. Caring Nanny observe les mÃ©triques exposÃ©es, elle ne provoque pas de requÃªte de diagnostic.

### 5.2 Ã‰tat de synchronisation

**SYNC_STATUS**
- **Objet d'observation :** Ã‰tat de la synchronisation entre DB MÃ¨re et DB Filles
- **Valeurs possibles :** `synchronized`, `syncing`, `desynchronized`, `conflict`
- **Signaux observÃ©s :** Deltas en attente, Ã©tat de connexion, conflits dÃ©tectÃ©s

**RÃ¨gle CN-KM-OBS-02 : Ã‰tat de synchronisation, pas action de synchronisation**

Caring Nanny observe si la synchronisation est en cours, rÃ©ussie, ou en Ã©chec. Elle ne peut jamais initier, annuler, ou modifier une synchronisation.

### 5.3 Ã‰tat des instances

**INSTANCE_STATUS**
- **Objet d'observation :** DisponibilitÃ© et connectivitÃ© des instances DB
- **DonnÃ©es observÃ©es :**
  - DB MÃ¨re : accessible, inaccessible
  - DB Filles : connectÃ©es, dÃ©connectÃ©es, nombre de filles actives
  - Latence de communication entre instances

**RÃ¨gle CN-KM-OBS-03 : Observation globale des instances**

Caring Nanny observe l'Ã©tat de toutes les instances connues. Elle agrÃ¨ge cette information en vue d'ensemble cohÃ©rente.

### 5.4 Ã‰tat des opÃ©rations en cours

**OPERATION_STATUS**
- **Objet d'observation :** WriteIntent en attente, deltas non propagÃ©s
- **DonnÃ©es observÃ©es :**
  - Nombre de WriteIntent en attente de validation
  - Nombre de deltas non propagÃ©s
  - Ã‚ge des opÃ©rations en attente
  - OpÃ©rations en Ã©chec ou en retry

**RÃ¨gle CN-KM-OBS-04 : Observation quantitative, pas qualitative**

Caring Nanny observe le volume et l'Ã©tat des opÃ©rations en cours. Elle ne connaÃ®t pas le contenu des WriteIntent ni des deltas.

### 5.5 Tableau rÃ©capitulatif des observations

| CatÃ©gorie | DonnÃ©es observÃ©es | FrÃ©quence | Impact sur Ã©tat systÃ¨me |
|-----------|-------------------|-----------|------------------------|
| **PERSISTENCE_HEALTH** | DisponibilitÃ©, temps de rÃ©ponse, erreurs | Continue | `healthy` â†’ `degraded` â†’ `error` |
| **SYNC_STATUS** | Ã‰tat sync, deltas, conflits | Continue | `syncing`, `conflict` |
| **INSTANCE_STATUS** | DB MÃ¨re, DB Filles, latence | Continue | `offline` si DB MÃ¨re inaccessible |
| **OPERATION_STATUS** | WriteIntent, deltas, Ã¢ge | Continue | `degraded` si accumulation |

---

## 6. Ã‰tats dÃ©rivÃ©s de l'observation de KindMother

### 6.1 Contribution aux catÃ©gories d'Ã©tat systÃ¨me

L'observation de KindMother contribue directement aux catÃ©gories d'Ã©tat systÃ¨me dÃ©finies dans la Documentation Fondatrice :

**healthy**
- Persistance disponible et fonctionnelle
- Synchronisation Ã  jour (si applicable)
- Toutes les instances connectÃ©es
- Aucune opÃ©ration en Ã©chec

**degraded**
- Temps de rÃ©ponse de la persistance Ã©levÃ©
- Deltas en attente depuis longtemps
- Certaines DB Filles dÃ©connectÃ©es
- WriteIntent en retry

**offline**
- DB MÃ¨re inaccessible
- Mode offline actif sur la DB Fille locale
- Synchronisation impossible

**syncing**
- Synchronisation en cours entre instances
- Deltas en transfert
- Ã‰tat temporaire pendant la rÃ©conciliation

**error**
- Persistance indisponible
- Conflits de synchronisation non rÃ©solus
- Erreurs critiques sur la couche de stockage

### 6.2 RÃ¨gles de dÃ©rivation d'Ã©tat

**RÃ¨gle CN-KM-STATE-01 : PrioritÃ© des Ã©tats**

En cas de conditions multiples, l'Ã©tat le plus critique prime :
`error` > `offline` > `syncing` > `degraded` > `healthy`

**RÃ¨gle CN-KM-STATE-02 : Ã‰tat offline reconnu comme normal**

ConformÃ©ment Ã  LOI-2, l'Ã©tat `offline` est un Ã©tat normal, pas une erreur. La DB Fille fonctionne de maniÃ¨re autonome.

**RÃ¨gle CN-KM-STATE-03 : Transition traÃ§able**

Chaque transition d'Ã©tat dÃ©rivÃ©e de l'observation de KindMother est traÃ§able : cause, timestamp, Ã©tat prÃ©cÃ©dent, Ã©tat nouveau.

---

## 7. Protocole d'observation

### 7.1 ModÃ¨le d'observation

L'observation suit un modÃ¨le **push passif** : KindMother Ã©met des signaux d'Ã©tat, Caring Nanny les reÃ§oit et les traite.

**CaractÃ©ristiques :**
- Unidirectionnel : KindMother â†’ Caring Nanny
- Passif : Caring Nanny reÃ§oit, elle ne demande pas
- Non bloquant : L'observation n'interfÃ¨re pas avec KindMother
- Continue : Observation permanente, pas ponctuelle

### 7.2 Format des signaux observÃ©s

**Structure conceptuelle d'un signal :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `signal_id` | Identifiant unique du signal | âœ… Oui |
| `source` | Origine du signal (kindmother) | âœ… Oui |
| `type` | Type de signal (health, sync, instance, operation) | âœ… Oui |
| `donnÃ©es` | DonnÃ©es spÃ©cifiques au type | âœ… Oui |
| `timestamp` | Horodatage du signal | âœ… Oui |
| `instance_id` | Instance concernÃ©e (si applicable) | âŒ Optionnel |

**RÃ¨gle CN-KM-PROT-01 : RÃ©ception fidÃ¨le**

Le signal est reÃ§u intÃ©gralement, sans modification ni interprÃ©tation initiale.

**RÃ¨gle CN-KM-PROT-02 : Pas de filtrage Ã  la source**

Caring Nanny reÃ§oit tous les signaux de KindMother. Le filtrage Ã©ventuel est fait cÃ´tÃ© Caring Nanny, jamais cÃ´tÃ© KindMother.

### 7.3 Traitement des signaux

**SÃ©quence de traitement :**

1. **RÃ©ception** â€” Le signal est reÃ§u depuis KindMother
2. **Validation** â€” Le signal est validÃ© (format, cohÃ©rence)
3. **Classification** â€” Le signal est classÃ© par type
4. **Ã‰valuation** â€” La condition observÃ©e est Ã©valuÃ©e
5. **AgrÃ©gation** â€” L'Ã©tat partiel est agrÃ©gÃ© Ã  l'Ã©tat global
6. **Transition** â€” Si l'Ã©tat global change, une transition est enregistrÃ©e
7. **Propagation** â€” Le changement d'Ã©tat est propagÃ© (via BondingBrother)

**RÃ¨gle CN-KM-PROT-03 : Traitement sÃ©quentiel**

Les signaux sont traitÃ©s dans l'ordre de rÃ©ception. Aucun signal n'est sautÃ© ou traitÃ© hors sÃ©quence.

**RÃ¨gle CN-KM-PROT-04 : Pas d'effet de bord**

Le traitement d'un signal ne produit jamais d'effet de bord sur KindMother.

---

## 8. Flux d'observation

### 8.1 Flux d'observation de santÃ© de persistance

**DÃ©clencheur :** Signal de santÃ© Ã©mis par KindMother

**SÃ©quence :**
1. KindMother dÃ©tecte un changement de santÃ© (disponibilitÃ©, latence)
2. KindMother Ã©met un signal `PERSISTENCE_HEALTH`
3. Caring Nanny reÃ§oit le signal
4. Caring Nanny Ã©value la condition (healthy, degraded, unavailable)
5. Caring Nanny met Ã  jour l'Ã©tat partiel de la persistance
6. Si l'Ã©tat global change, Caring Nanny enregistre la transition
7. Caring Nanny propage le changement d'Ã©tat

### 8.2 Flux d'observation de synchronisation

**DÃ©clencheur :** Changement d'Ã©tat de synchronisation

**SÃ©quence :**
1. KindMother dÃ©marre, progresse, ou termine une synchronisation
2. KindMother Ã©met un signal `SYNC_STATUS`
3. Caring Nanny reÃ§oit le signal
4. Caring Nanny Ã©value l'Ã©tat (synchronized, syncing, desynchronized, conflict)
5. Caring Nanny met Ã  jour l'Ã©tat partiel de la synchronisation
6. Si l'Ã©tat global change, Caring Nanny enregistre la transition
7. Caring Nanny propage le changement d'Ã©tat

### 8.3 Flux d'observation d'instances

**DÃ©clencheur :** Changement de connectivitÃ© d'une instance

**SÃ©quence :**
1. Une instance DB (MÃ¨re ou Fille) change d'Ã©tat de connexion
2. KindMother Ã©met un signal `INSTANCE_STATUS`
3. Caring Nanny reÃ§oit le signal
4. Caring Nanny met Ã  jour la cartographie des instances
5. Caring Nanny Ã©value l'impact sur l'Ã©tat global (notamment offline)
6. Si l'Ã©tat global change, Caring Nanny enregistre la transition
7. Caring Nanny propage le changement d'Ã©tat

### 8.4 Diagramme de sÃ©quence

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚   KindMother    â”‚                      â”‚  Caring Nanny   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚                                        â”‚
         â”‚                                        â”‚
         â”œâ”€â”€ Signal (health/sync/instance) â”€â”€â”€â”€â”€â–ºâ”‚
         â”‚                                        â”‚
         â”‚                                        â”œâ”€â”€ RÃ©ception
         â”‚                                        â”‚
         â”‚                                        â”œâ”€â”€ Validation
         â”‚                                        â”‚
         â”‚                                        â”œâ”€â”€ Classification
         â”‚                                        â”‚
         â”‚                                        â”œâ”€â”€ Ã‰valuation
         â”‚                                        â”‚
         â”‚                                        â”œâ”€â”€ AgrÃ©gation
         â”‚                                        â”‚
         â”‚                                        â”œâ”€â”€ Transition?
         â”‚                                        â”‚
         â”‚                                        â”œâ”€â”€ Propagation
         â”‚                                        â”‚   (vers BondingBrother)
         â”‚                                        â”‚
```

---

## 9. RÃ¨gles d'intÃ©gration

### 9.1 RÃ¨gles de communication

**RÃ¨gle CN-KM-INT-01 : KindMother Ã©met, Caring Nanny reÃ§oit**

La direction de communication est toujours KindMother â†’ Caring Nanny. Caring Nanny ne sollicite jamais KindMother.

**RÃ¨gle CN-KM-INT-02 : Pas de callback vers KindMother**

Caring Nanny ne fournit jamais de callback ou de point d'entrÃ©e pour que KindMother l'interroge.

**RÃ¨gle CN-KM-INT-03 : Observation continue**

L'observation est continue et permanente. Il n'y a pas de mode "observation dÃ©sactivÃ©e".

### 9.2 RÃ¨gles de donnÃ©es

**RÃ¨gle CN-KM-INT-04 : DonnÃ©es d'Ã©tat uniquement**

Caring Nanny observe uniquement les donnÃ©es d'Ã©tat de KindMother, jamais les donnÃ©es mÃ©tier (contenu, hiÃ©rarchie, etc.).

**RÃ¨gle CN-KM-INT-05 : Pas d'accÃ¨s Ã  SQLite**

Caring Nanny n'a jamais accÃ¨s Ã  la couche SQLite interne de KindMother. L'abstraction de KindMother est respectÃ©e.

**RÃ¨gle CN-KM-INT-06 : Pas de connaissance des WriteIntent**

Caring Nanny connaÃ®t le nombre et l'Ã¢ge des WriteIntent en attente, mais jamais leur contenu.

### 9.3 RÃ¨gles de traÃ§abilitÃ©

**RÃ¨gle CN-KM-INT-07 : TraÃ§abilitÃ© des signaux**

Tous les signaux reÃ§us de KindMother sont enregistrÃ©s dans l'historique de Caring Nanny.

**RÃ¨gle CN-KM-INT-08 : CorrÃ©lation signal-transition**

Chaque transition d'Ã©tat est corrÃ©lÃ©e au(x) signal(aux) qui l'a(ont) provoquÃ©e.

---

## 10. Gestion des Ã©tats spÃ©ciaux

### 10.1 Ã‰tat offline

**Comportement :**
- Caring Nanny dÃ©tecte l'inaccessibilitÃ© de la DB MÃ¨re
- L'Ã©tat global passe Ã  `offline`
- L'observation continue sur les signaux locaux (DB Fille)
- La transition est enregistrÃ©e avec la cause

**RÃ¨gle CN-KM-OFFLINE-01 : Offline est un Ã©tat normal**

ConformÃ©ment Ã  LOI-2, l'Ã©tat `offline` n'est pas une erreur. C'est un Ã©tat normal de fonctionnement autonome.

**RÃ¨gle CN-KM-OFFLINE-02 : Observation locale maintenue**

En mode offline, Caring Nanny continue d'observer les signaux de la DB Fille locale.

### 10.2 Ã‰tat de conflit

**Comportement :**
- KindMother dÃ©tecte un conflit de synchronisation
- Caring Nanny reÃ§oit un signal `SYNC_STATUS` avec `conflict`
- L'Ã©tat global inclut `conflict` dans son Ã©valuation
- Caring Nanny propage l'information, mais ne rÃ©sout pas le conflit

**RÃ¨gle CN-KM-CONFLICT-01 : Observation du conflit, pas rÃ©solution**

Caring Nanny observe l'existence d'un conflit. La rÃ©solution appartient Ã  KindMother ou au produit.

### 10.3 Ã‰tat d'erreur de persistance

**Comportement :**
- KindMother dÃ©tecte une erreur critique de persistance
- Caring Nanny reÃ§oit un signal `PERSISTENCE_HEALTH` avec `unavailable`
- L'Ã©tat global passe Ã  `error`
- Caring Nanny propage l'information immÃ©diatement

**RÃ¨gle CN-KM-ERROR-01 : Propagation immÃ©diate des erreurs critiques**

Les erreurs critiques de persistance sont propagÃ©es immÃ©diatement, sans dÃ©lai d'agrÃ©gation.

---

## 11. Garanties de l'intÃ©gration

### 11.1 Garantie de passivitÃ©

**Engagement :** Caring Nanny n'a jamais d'effet sur KindMother. L'observation est strictement passive et unidirectionnelle.

### 11.2 Garantie de fidÃ©litÃ©

**Engagement :** Les signaux observÃ©s sont traitÃ©s fidÃ¨lement. Caring Nanny ne modifie pas, n'interprÃ¨te pas subjectivement, ne filtre pas arbitrairement les signaux.

### 11.3 Garantie de complÃ©tude

**Engagement :** Tous les signaux Ã©mis par KindMother sont observÃ©s. Aucun signal n'est ignorÃ© ou perdu.

### 11.4 Garantie de traÃ§abilitÃ©

**Engagement :** Toute observation de KindMother est traÃ§able de bout en bout. L'historique permet de reconstituer l'Ã©volution de l'Ã©tat observÃ©.

### 11.5 Garantie de cohÃ©rence

**Engagement :** L'Ã©tat dÃ©rivÃ© de l'observation de KindMother est cohÃ©rent avec les autres sources d'observation. Pas de contradiction dans l'Ã©tat global.

### 11.6 Garantie de non-blocage

**Engagement :** L'observation de KindMother ne bloque jamais le fonctionnement de KindMother ou du systÃ¨me. ConformitÃ© Ã  INV-CN-6.

---

## 12. Invariants de l'intÃ©gration

### 12.1 Invariants de relation

**INV-CN-KM-1 : Observation unidirectionnelle**

KindMother Ã©met des signaux. Caring Nanny reÃ§oit et observe. La direction est toujours KindMother â†’ Caring Nanny.

**INV-CN-KM-2 : Aucune capacitÃ© de modification**

Caring Nanny ne peut jamais modifier l'Ã©tat ou les donnÃ©es de KindMother. Aucune exception.

**INV-CN-KM-3 : Respect de l'abstraction KindMother**

Caring Nanny n'accÃ¨de jamais aux dÃ©tails internes de KindMother (SQLite, schÃ©mas, etc.). Elle observe uniquement les signaux d'Ã©tat exposÃ©s.

### 12.2 Invariants de donnÃ©es

**INV-CN-KM-4 : Observation d'Ã©tat, pas de contenu**

Caring Nanny observe l'Ã©tat des donnÃ©es (santÃ©, synchronisation, disponibilitÃ©), jamais le contenu des donnÃ©es.

**INV-CN-KM-5 : Signaux complets**

Tous les signaux de KindMother sont reÃ§us et traitÃ©s. Aucun signal n'est filtrÃ© Ã  la source.

### 12.3 Invariants de protocole

**INV-CN-KM-6 : Traitement sÃ©quentiel**

Les signaux sont traitÃ©s dans l'ordre de rÃ©ception. La sÃ©quence est prÃ©servÃ©e.

**INV-CN-KM-7 : TraÃ§abilitÃ© complÃ¨te**

Chaque signal reÃ§u est enregistrÃ© dans l'historique avec son contexte complet.

---

## 13. Exemples

### 13.1 Observation de santÃ© normale

**Signal KindMother :**
```
{
  "signal_id": "sig-km-001",
  "source": "kindmother",
  "type": "PERSISTENCE_HEALTH",
  "donnÃ©es": {
    "status": "available",
    "latency_ms": 5,
    "error_count": 0
  },
  "timestamp": "2026-01-27T14:00:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- RÃ©ception du signal
- Ã‰valuation : persistance saine (latency < 50ms, error_count = 0)
- Ã‰tat partiel : `healthy`
- Pas de transition (Ã©tat stable)
- Enregistrement dans l'historique

### 13.2 DÃ©tection de dÃ©gradation

**Signal KindMother :**
```
{
  "signal_id": "sig-km-002",
  "source": "kindmother",
  "type": "PERSISTENCE_HEALTH",
  "donnÃ©es": {
    "status": "available",
    "latency_ms": 250,
    "error_count": 3
  },
  "timestamp": "2026-01-27T14:05:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- RÃ©ception du signal
- Ã‰valuation : latence Ã©levÃ©e (> 100ms), erreurs prÃ©sentes
- Ã‰tat partiel : `degraded`
- Transition : `healthy` â†’ `degraded`
- Cause : "latence Ã©levÃ©e (250ms), erreurs (3)"
- Propagation du changement d'Ã©tat

### 13.3 Passage en mode offline

**Signal KindMother :**
```
{
  "signal_id": "sig-km-003",
  "source": "kindmother",
  "type": "INSTANCE_STATUS",
  "donnÃ©es": {
    "db_mere": {
      "status": "unreachable",
      "last_seen": "2026-01-27T13:55:00Z"
    },
    "db_fille_local": {
      "status": "active",
      "mode": "offline"
    }
  },
  "timestamp": "2026-01-27T14:10:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- RÃ©ception du signal
- Ã‰valuation : DB MÃ¨re inaccessible, DB Fille en mode offline
- Ã‰tat partiel : `offline`
- Transition : `degraded` â†’ `offline`
- Cause : "DB MÃ¨re inaccessible depuis 15 minutes"
- Ã‰tat `offline` reconnu comme normal (LOI-2)
- Propagation du changement d'Ã©tat

### 13.4 Synchronisation en cours

**Signal KindMother :**
```
{
  "signal_id": "sig-km-004",
  "source": "kindmother",
  "type": "SYNC_STATUS",
  "donnÃ©es": {
    "status": "syncing",
    "deltas_pending": 42,
    "progress_percent": 65,
    "estimated_completion": "2026-01-27T14:20:00Z"
  },
  "timestamp": "2026-01-27T14:15:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- RÃ©ception du signal
- Ã‰valuation : synchronisation en cours
- Ã‰tat partiel : `syncing`
- Transition : `offline` â†’ `syncing`
- Cause : "Reconnexion DB MÃ¨re, synchronisation dÃ©marrÃ©e"
- Propagation du changement d'Ã©tat

### 13.5 DÃ©tection de conflit

**Signal KindMother :**
```
{
  "signal_id": "sig-km-005",
  "source": "kindmother",
  "type": "SYNC_STATUS",
  "donnÃ©es": {
    "status": "conflict",
    "conflict_count": 3,
    "conflict_types": ["write_intent_collision", "version_mismatch"],
    "requires_resolution": true
  },
  "timestamp": "2026-01-27T14:18:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- RÃ©ception du signal
- Ã‰valuation : conflits de synchronisation dÃ©tectÃ©s
- Ã‰tat partiel : `syncing` avec `conflict`
- Information propagÃ©e : conflits Ã  rÃ©soudre
- Caring Nanny n'intervient pas dans la rÃ©solution
- Propagation de l'Ã©tat incluant les informations de conflit

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit l'interface et le protocole que Caring Nanny doit respecter pour observer KindMother.

Toute implÃ©mentation de l'intÃ©gration avec KindMother doit respecter ce contrat. Toute violation (tentative de modification, d'interaction bidirectionnelle, d'accÃ¨s aux donnÃ©es) constitue une rupture de contrat grave.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :**
- Caring Nanny - Documentation Fondatrice v1.6 (Section 3)
- Caring Nanny - Architecture et Composants v1.0
- Caring Nanny - Invariants et Garanties v1.0
- KindMother - Documentation Fondatrice v1.2

---

## 15. Mini log de gÃ©nÃ©ration

### DÃ©cision Ã©ditoriale E1 : Nature de la relation

**DÃ©cision prise :** La relation est strictement unidirectionnelle d'observation : KindMother Ã©met des signaux d'Ã©tat, Caring Nanny les observe passivement. Cette approche diffÃ¨re des contrats d'intÃ©gration BondingBrother (dÃ©lÃ©gation bidirectionnelle) ou Master Butler (consultation).

**Application :** Tout le document est structurÃ© autour de l'observation passive sans interaction.

### DÃ©cision Ã©ditoriale E2 : Types de donnÃ©es observÃ©es

**DÃ©cision prise :** Les donnÃ©es observÃ©es sont catÃ©gorisÃ©es en 4 types : santÃ© de la persistance, Ã©tat de synchronisation, Ã©tat des instances, Ã©tat des opÃ©rations. Ces catÃ©gories correspondent aux informations mentionnÃ©es dans la Documentation Fondatrice.

**Application :** Section 5 dÃ©finit exhaustivement chaque type d'observation.

### Warning W1 : Risque de confusion observation/action

**Warning rencontrÃ© :** Risque que l'observation soit interprÃ©tÃ©e comme permettant une action corrective.

**DÃ©cision prise :** Renforcement explicite dans toutes les sections que Caring Nanny ne peut jamais agir sur KindMother. RÃ¨gles CN-KM-01, CN-KM-02, CN-KM-03 Ã©tablissent l'impossibilitÃ© d'action.

**Correction effectuÃ©e :** Ajout d'invariants INV-CN-KM-1, INV-CN-KM-2, INV-CN-KM-3 pour formaliser cette impossibilitÃ©.

### Warning W2 : Ã‰tat offline

**Warning rencontrÃ© :** Risque que l'Ã©tat offline soit traitÃ© comme une erreur.

**DÃ©cision prise :** ConformÃ©ment Ã  LOI-2, l'Ã©tat offline est explicitement reconnu comme un Ã©tat normal. RÃ¨gles CN-KM-OFFLINE-01 et CN-KM-STATE-02 clarifient ce point.

**Correction effectuÃ©e :** Section 10.1 dÃ©diÃ©e Ã  l'Ã©tat offline.

### VÃ©rification de cohÃ©rence

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec Caring Nanny - Documentation Fondatrice : ConfirmÃ©e (Section 3, relation avec KindMother)
- âœ… CohÃ©rence avec KindMother - Documentation Fondatrice : ConfirmÃ©e (relation d'observation mentionnÃ©e Section 7)
- âœ… ConformitÃ© LOI-1 : ConfirmÃ©e (observation locale, aucune dÃ©pendance externe)
- âœ… ConformitÃ© LOI-2 : ConfirmÃ©e (offline reconnu comme Ã©tat normal)
- âœ… ConformitÃ© INV-CN-1 Ã  INV-CN-7 : ConfirmÃ©e (observateur pur, aucune modification, non-bloquant)

**Conclusion :** Aucune contradiction dÃ©tectÃ©e. Le document est cohÃ©rent avec l'Ã©cosystÃ¨me documentaire existant.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

