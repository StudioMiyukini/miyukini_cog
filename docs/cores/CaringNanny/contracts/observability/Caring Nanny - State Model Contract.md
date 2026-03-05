# Caring Nanny - State Model Contract

## 1. Contexte

Ce document dÃ©finit le **modÃ¨le formel des Ã©tats** observÃ©s et rapportÃ©s par Caring Nanny. Il Ã©tablit les catÃ©gories d'Ã©tats canoniques, leurs caractÃ©ristiques, leurs conditions d'entrÃ©e/sortie, et les rÃ¨gles qui gouvernent leur usage dans le Miyukini Core System.

Ce contrat Ã©tend la Section 4 de la [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) en formalisant le modÃ¨le d'Ã©tat comme spÃ©cification normative.

## 2. PortÃ©e / Scope

Ce document couvre :
- Les catÃ©gories d'Ã©tat systÃ¨me (healthy, degraded, offline, syncing, error)
- Les catÃ©gories d'Ã©tat applicatif (Ã©tats partiels des composants)
- Les propriÃ©tÃ©s formelles de chaque Ã©tat
- Les conditions d'entrÃ©e et de sortie de chaque Ã©tat
- Les rÃ¨gles d'agrÃ©gation des Ã©tats partiels en Ã©tat global
- Les transitions valides entre Ã©tats
- Les Ã©tats d'isolement conformes Ã  LOI-2

Ce document **ne couvre pas** :
- Les flux d'observation (voir Observation Flow Contract)
- Les flux de propagation (voir Propagation Flow Contract)
- Les dÃ©tails d'implÃ©mentation
- Les mÃ©canismes de dÃ©tection (voir Architecture et Composants)

---

## 3. DÃ©finitions

### 3.1 Ã‰tat

Un **Ã©tat** est une condition observable et classifiable d'un composant ou du systÃ¨me Ã  un instant donnÃ©. Un Ã©tat est :
- **CatÃ©gorisÃ©** : appartient Ã  une catÃ©gorie canonique dÃ©finie
- **HorodatÃ©** : associÃ© Ã  un instant de temps local (via le kernel Clock)
- **ContextualisÃ©** : accompagnÃ© d'informations de contexte
- **Non-interprÃ©table** : Caring Nanny ne donne pas d'opinion sur l'Ã©tat, elle le rapporte

### 3.2 Ã‰tat systÃ¨me

L'**Ã©tat systÃ¨me** est la condition globale du Miyukini Core System Ã  un instant donnÃ©. C'est une synthÃ¨se agrÃ©gÃ©e de tous les Ã©tats partiels des composants.

**PropriÃ©tÃ©s :**
- Unique : un seul Ã©tat systÃ¨me Ã  un instant T
- CohÃ©rent : sans contradiction interne
- Observable : accessible par interrogation
- InstantanÃ© : valide Ã  un moment prÃ©cis

### 3.3 Ã‰tat applicatif

L'**Ã©tat applicatif** est la condition d'un module ou composant spÃ©cifique au sein du systÃ¨me. C'est un Ã©tat partiel qui contribue Ã  l'Ã©tat systÃ¨me global.

**PropriÃ©tÃ©s :**
- Partiel : concerne un composant spÃ©cifique
- Contributif : participe Ã  l'agrÃ©gation de l'Ã©tat systÃ¨me
- Autonome : peut Ãªtre observÃ© indÃ©pendamment
- SpÃ©cialisÃ© : sÃ©mantique propre au composant

### 3.4 Transition

Une **transition** est le passage d'un Ã©tat Ã  un autre. Elle est :
- **Causale** : provoquÃ©e par une ou plusieurs conditions
- **InstantanÃ©e** : se produit Ã  un moment prÃ©cis
- **TraÃ§able** : enregistrÃ©e avec son contexte
- **Validable** : conforme aux rÃ¨gles de transition

---

## 4. CatÃ©gories d'Ã©tat systÃ¨me

Caring Nanny reconnaÃ®t exactement **cinq catÃ©gories d'Ã©tat systÃ¨me**. Ces catÃ©gories sont mutuellement exclusives : Ã  tout instant, le systÃ¨me est dans exactement une de ces catÃ©gories.

### 4.1 Ã‰tat : healthy

**DÃ©finition :** Tous les composants fonctionnent normalement, aucune anomalie n'est dÃ©tectÃ©e, toutes les fonctionnalitÃ©s sont disponibles.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Code | `healthy` |
| SÃ©vÃ©ritÃ© | 0 (normale) |
| OpÃ©rations permises | Toutes |
| Notifications | Aucune (Ã©tat nominal) |
| DurÃ©e typique | IndÃ©finie (Ã©tat cible) |

**Conditions d'entrÃ©e :**
- Tous les composants critiques rapportent un Ã©tat nominal
- Aucune anomalie active
- Aucune synchronisation en cours
- Connexion disponible (si mode connectÃ©)

**Conditions de sortie :**
- DÃ©tection d'une anomalie â†’ `degraded` ou `error`
- Perte de connexion â†’ `offline`
- DÃ©marrage de synchronisation â†’ `syncing`

**ConformitÃ© LOI-1 :** L'Ã©tat `healthy` est atteignable sans dÃ©pendance externe. Un systÃ¨me isolÃ© peut Ãªtre `healthy` s'il fonctionne correctement en mode autonome.

---

### 4.2 Ã‰tat : degraded

**DÃ©finition :** Certains composants fonctionnent en mode dÃ©gradÃ©, le systÃ¨me reste opÃ©rationnel mais avec des fonctionnalitÃ©s rÃ©duites ou des performances diminuÃ©es.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Code | `degraded` |
| SÃ©vÃ©ritÃ© | 1 (avertissement) |
| OpÃ©rations permises | OpÃ©rations essentielles |
| Notifications | Changement d'Ã©tat |
| DurÃ©e typique | Variable |

**Conditions d'entrÃ©e :**
- Un ou plusieurs composants non-critiques dysfonctionnent
- Performance dÃ©gradÃ©e (latence, dÃ©bit)
- Ressources limitÃ©es (mÃ©moire, CPU)
- Certaines fonctionnalitÃ©s indisponibles

**Conditions de sortie :**
- RÃ©solution de toutes les dÃ©gradations â†’ `healthy`
- Aggravation critique â†’ `error`
- Perte de connexion (si connectÃ©) â†’ `offline`

**Sous-catÃ©gories informatives (non-canoniques) :**
- `degraded:performance` : dÃ©gradation de performance
- `degraded:feature` : fonctionnalitÃ© indisponible
- `degraded:resource` : ressources limitÃ©es

**ConformitÃ© LOI-2 :** L'Ã©tat `degraded` est un Ã©tat normal, pas une erreur. Le systÃ¨me fonctionne avec ce qu'il a disponible, conformÃ©ment Ã  LOI-2 (le systÃ¨me accepte l'isolement comme Ã©tat normal).

---

### 4.3 Ã‰tat : offline

**DÃ©finition :** Le systÃ¨me fonctionne en mode dÃ©connectÃ©, sans accÃ¨s aux autoritÃ©s centrales (DB MÃ¨re, nÅ“uds fÃ©dÃ©rÃ©s). C'est un **Ã©tat normal**, pas une erreur.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Code | `offline` |
| SÃ©vÃ©ritÃ© | 0 (normale) |
| OpÃ©rations permises | OpÃ©rations locales |
| Notifications | Transition d'Ã©tat uniquement |
| DurÃ©e typique | Variable (Ã©tat normal) |

**Conditions d'entrÃ©e :**
- Perte de connexion rÃ©seau
- IndisponibilitÃ© de la DB MÃ¨re
- DÃ©cision explicite de fonctionnement isolÃ©
- DÃ©marrage sans connexion disponible

**Conditions de sortie :**
- RÃ©tablissement de la connexion â†’ `syncing` (puis `healthy`)
- DÃ©tection d'anomalie locale â†’ `degraded` ou `error`

**Distinctions critiques :**

| Aspect | offline (normal) | error (problÃ¨me) |
|--------|------------------|------------------|
| Nature | Ã‰tat souhaitÃ© ou acceptÃ© | Condition anormale |
| Fonctionnement | Complet en mode local | LimitÃ© ou bloquÃ© |
| RÃ©action | Aucune correction requise | Diagnostic/correction |
| DurÃ©e | IndÃ©finie acceptable | Ã€ rÃ©soudre |

**ConformitÃ© LOI-2 :** L'Ã©tat `offline` implÃ©mente directement LOI-2 (le systÃ¨me accepte l'isolement comme Ã©tat normal). L'isolement n'est pas une erreur, c'est un mode de fonctionnement valide et explicitement reconnu.

**ConformitÃ© LOI-3 :** En Ã©tat `offline`, l'Ã©tat local est souverain. Les dÃ©cisions prises sont valables localement, les donnÃ©es locales constituent la vÃ©ritÃ© locale.

---

### 4.4 Ã‰tat : syncing

**DÃ©finition :** Une synchronisation est en cours entre la source locale et une source distante (DB MÃ¨re, nÅ“ud fÃ©dÃ©rÃ©). Certaines opÃ©rations peuvent Ãªtre diffÃ©rÃ©es ou contraintes.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Code | `syncing` |
| SÃ©vÃ©ritÃ© | 0 (normale) |
| OpÃ©rations permises | Lectures, Ã©critures locales |
| Notifications | Progression, conflits Ã©ventuels |
| DurÃ©e typique | Transitoire |

**Conditions d'entrÃ©e :**
- Reconnexion aprÃ¨s mode offline
- RÃ©conciliation programmÃ©e
- RÃ©ception de deltas Ã  traiter
- Demande explicite de synchronisation

**Conditions de sortie :**
- Synchronisation terminÃ©e avec succÃ¨s â†’ `healthy`
- Synchronisation terminÃ©e avec rÃ©sidus â†’ `degraded`
- Perte de connexion pendant sync â†’ `offline`
- Erreur critique de synchronisation â†’ `error`

**Sous-Ã©tats informatifs (non-canoniques) :**
- `syncing:receiving` : rÃ©ception de deltas
- `syncing:applying` : application des changements
- `syncing:reconciling` : rÃ©solution de conflits
- `syncing:sending` : envoi de deltas locaux

**ConformitÃ© LOI-4 :** La synchronisation ne dÃ©pend pas d'un temps global. Les comparaisons utilisent des horloges logiques ou des points de synchronisation, conformÃ©ment Ã  LOI-4 (pas de temps global requis).

---

### 4.5 Ã‰tat : error

**DÃ©finition :** Une erreur critique a Ã©tÃ© dÃ©tectÃ©e. Certaines opÃ©rations ne sont pas possibles. Le systÃ¨me nÃ©cessite une attention ou une intervention.

**CaractÃ©ristiques :**

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Code | `error` |
| SÃ©vÃ©ritÃ© | 2 (critique) |
| OpÃ©rations permises | LimitÃ©es (diagnostic, lecture) |
| Notifications | Alerte, dÃ©tails d'erreur |
| DurÃ©e typique | Ã€ rÃ©soudre |

**Conditions d'entrÃ©e :**
- Ã‰chec d'un composant critique
- Corruption de donnÃ©es dÃ©tectÃ©e
- IncohÃ©rence non rÃ©solvable
- Erreur systÃ¨me critique

**Conditions de sortie :**
- RÃ©solution de l'erreur â†’ `healthy` ou `degraded`
- RedÃ©marrage â†’ Ã©tat initial selon contexte

**Sous-catÃ©gories informatives (non-canoniques) :**
- `error:critical` : composant critique dÃ©faillant
- `error:data` : problÃ¨me de donnÃ©es
- `error:system` : erreur systÃ¨me
- `error:unrecoverable` : erreur non rÃ©cupÃ©rable

**Distinction avec offline :**

L'Ã©tat `error` reprÃ©sente un **problÃ¨me** Ã  rÃ©soudre, tandis que `offline` reprÃ©sente un **mode de fonctionnement** valide. Cette distinction est fondamentale pour la conformitÃ© LOI-2.

---

## 5. CatÃ©gories d'Ã©tat applicatif

Les Ã©tats applicatifs sont les Ã©tats partiels des composants individuels. Ils contribuent Ã  l'Ã©tat systÃ¨me global par agrÃ©gation.

### 5.1 Ã‰tats KindMother

| Ã‰tat | Description | Contribution Ã  l'Ã©tat systÃ¨me |
|------|-------------|------------------------------|
| `km:available` | Persistance disponible | â†’ healthy |
| `km:degraded` | Performance rÃ©duite | â†’ degraded |
| `km:syncing` | Synchronisation en cours | â†’ syncing |
| `km:offline` | Mode local uniquement | â†’ offline |
| `km:error` | Erreur de persistance | â†’ error |

### 5.2 Ã‰tats StrongFather

| Ã‰tat | Description | Contribution Ã  l'Ã©tat systÃ¨me |
|------|-------------|------------------------------|
| `sf:ready` | Moteur de dÃ©cision prÃªt | â†’ healthy |
| `sf:degraded` | Certaines politiques non disponibles | â†’ degraded |
| `sf:error` | Erreur du moteur de dÃ©cision | â†’ error |

### 5.3 Ã‰tats BondingBrother

| Ã‰tat | Description | Contribution Ã  l'Ã©tat systÃ¨me |
|------|-------------|------------------------------|
| `bb:available` | MÃ©diation disponible | â†’ healthy |
| `bb:degraded` | Canaux partiellement disponibles | â†’ degraded |
| `bb:offline` | MÃ©diation locale uniquement | â†’ offline |
| `bb:error` | Erreur de mÃ©diation | â†’ error |

### 5.4 Ã‰tats Module SPM

| Ã‰tat | Description | Contribution Ã  l'Ã©tat systÃ¨me |
|------|-------------|------------------------------|
| `mod:ready` | Module opÃ©rationnel | â†’ healthy |
| `mod:loading` | Module en chargement | â†’ syncing |
| `mod:degraded` | FonctionnalitÃ©s rÃ©duites | â†’ degraded |
| `mod:unavailable` | Module non disponible | â†’ degraded |
| `mod:error` | Erreur de module | â†’ error |

---

## 6. RÃ¨gles d'agrÃ©gation

L'Ã©tat systÃ¨me global est dÃ©terminÃ© par l'agrÃ©gation des Ã©tats applicatifs selon des rÃ¨gles de prioritÃ© dÃ©finies.

### 6.1 RÃ¨gle de prioritÃ©

Caring Nanny applique la rÃ¨gle de **prioritÃ© par sÃ©vÃ©ritÃ© maximale** :

```
Ã‰tat systÃ¨me = max(sÃ©vÃ©ritÃ©(Ã©tats applicatifs))
```

**Ordre de prioritÃ© (du plus prioritaire au moins prioritaire) :**
1. `error` (sÃ©vÃ©ritÃ© 2) : si un composant critique est en erreur
2. `syncing` (sÃ©vÃ©ritÃ© 0, mais transitoire prioritaire)
3. `degraded` (sÃ©vÃ©ritÃ© 1)
4. `offline` (sÃ©vÃ©ritÃ© 0, mode)
5. `healthy` (sÃ©vÃ©ritÃ© 0, nominal)

### 6.2 RÃ¨gles d'agrÃ©gation spÃ©cifiques

| Condition | Ã‰tat systÃ¨me rÃ©sultant |
|-----------|------------------------|
| Au moins un composant critique en `error` | `error` |
| Synchronisation en cours | `syncing` |
| Au moins un composant en `degraded`, aucun `error` | `degraded` |
| Tous les composants `offline` ou mode dÃ©connectÃ© | `offline` |
| Tous les composants `healthy` | `healthy` |

### 6.3 Composants critiques vs non-critiques

La distinction entre composants **critiques** et **non-critiques** influence l'agrÃ©gation :

**Composants critiques :**
- KindMother (persistance)
- StrongFather (dÃ©cisions)

**Composants non-critiques :**
- Modules SPM individuels
- Canaux de mÃ©diation optionnels

**RÃ¨gle :** Une erreur sur un composant critique entraÃ®ne `error` systÃ¨me. Une erreur sur un composant non-critique entraÃ®ne `degraded` (sauf si bloquante).

### 6.4 RÃ©solution des contradictions

En cas de contradiction apparente, Caring Nanny applique :

1. **CohÃ©rence temporelle** : l'observation la plus rÃ©cente prÃ©vaut
2. **CohÃ©rence de sÃ©vÃ©ritÃ©** : la sÃ©vÃ©ritÃ© maximale prÃ©vaut
3. **CohÃ©rence de source** : les composants critiques prÃ©valent

**Exemple :**
- KindMother rapporte `km:available`
- Module A rapporte `mod:error`
- RÃ©sultat : `degraded` (module non-critique en erreur)

---

## 7. Matrice de transitions valides

### 7.1 Transitions entre Ã©tats systÃ¨me

| Ã‰tat source | Ã‰tats cibles valides | Transitions directes interdites |
|-------------|---------------------|--------------------------------|
| `healthy` | `degraded`, `offline`, `syncing`, `error` | â€” |
| `degraded` | `healthy`, `offline`, `error` | `syncing` sans passer par `healthy` |
| `offline` | `syncing`, `degraded`, `error` | `healthy` sans passer par `syncing` |
| `syncing` | `healthy`, `degraded`, `offline`, `error` | â€” |
| `error` | `healthy`, `degraded`, `offline` | `syncing` (correction requise d'abord) |

### 7.2 Diagramme de transitions

```
                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                    â”‚                                         â”‚
                    â–¼                                         â”‚
    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
    â”‚ offline â”‚â—„â”€â”€â”‚ healthy â”‚â”€â”€â–ºâ”‚degraded â”‚â”€â”€â–ºâ”‚  error  â”‚â”€â”€â”€â”˜
    â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜   â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜   â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜   â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜
         â”‚             â”‚             â”‚             â”‚
         â”‚             â”‚             â”‚             â”‚
         â”‚        â”Œâ”€â”€â”€â”€â–¼â”€â”€â”€â”€â”        â”‚             â”‚
         â””â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚ syncing â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”˜             â”‚
                  â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜                      â”‚
                       â”‚                           â”‚
                       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 7.3 Conditions de transition

| Transition | Condition de dÃ©clenchement |
|------------|---------------------------|
| `healthy` â†’ `degraded` | DÃ©tection de dÃ©gradation non-critique |
| `healthy` â†’ `offline` | Perte de connexion |
| `healthy` â†’ `syncing` | DÃ©marrage de synchronisation |
| `healthy` â†’ `error` | Erreur critique dÃ©tectÃ©e |
| `degraded` â†’ `healthy` | RÃ©solution de toutes les dÃ©gradations |
| `degraded` â†’ `error` | Aggravation critique |
| `degraded` â†’ `offline` | Perte de connexion |
| `offline` â†’ `syncing` | RÃ©tablissement de connexion |
| `offline` â†’ `degraded` | Anomalie locale dÃ©tectÃ©e |
| `offline` â†’ `error` | Erreur critique locale |
| `syncing` â†’ `healthy` | Synchronisation rÃ©ussie |
| `syncing` â†’ `degraded` | Synchronisation avec rÃ©sidus |
| `syncing` â†’ `offline` | Perte de connexion pendant sync |
| `syncing` â†’ `error` | Erreur de synchronisation |
| `error` â†’ `healthy` | RÃ©solution complÃ¨te |
| `error` â†’ `degraded` | RÃ©solution partielle |
| `error` â†’ `offline` | Passage en mode isolÃ© aprÃ¨s erreur |

---

## 8. PropriÃ©tÃ©s formelles des Ã©tats

### 8.1 PropriÃ©tÃ© d'exclusivitÃ© mutuelle

**PF-SM-01 :** Ã€ tout instant T, le systÃ¨me est dans exactement un Ã©tat :

```
âˆ€T : |{s âˆˆ {healthy, degraded, offline, syncing, error} : Ã©tat(T) = s}| = 1
```

### 8.2 PropriÃ©tÃ© de complÃ©tude

**PF-SM-02 :** Toute condition observable peut Ãªtre classifiÃ©e dans une catÃ©gorie d'Ã©tat :

```
âˆ€c âˆˆ Conditions : âˆƒs âˆˆ Ã‰tats : classifie(c) = s
```

### 8.3 PropriÃ©tÃ© de dÃ©terminisme

**PF-SM-03 :** L'agrÃ©gation des Ã©tats partiels produit toujours le mÃªme Ã©tat global :

```
âˆ€(epâ‚, epâ‚‚, ..., epâ‚™) : agrÃ¨ge(epâ‚, epâ‚‚, ..., epâ‚™) = Ã©tat_unique
```

### 8.4 PropriÃ©tÃ© de transition valide

**PF-SM-04 :** Toute transition respecte la matrice de transitions valides :

```
âˆ€(sâ‚, sâ‚‚) : transition(sâ‚, sâ‚‚) âŸ¹ (sâ‚, sâ‚‚) âˆˆ TransitionsValides
```

### 8.5 PropriÃ©tÃ© de traÃ§abilitÃ©

**PF-SM-05 :** Toute transition est associÃ©e Ã  une cause identifiable :

```
âˆ€transition(sâ‚, sâ‚‚) : âˆƒcause : provoquÃ©e_par(transition, cause)
```

---

## 9. Ã‰tats d'isolement (conformitÃ© LOI-2)

Ce contrat implÃ©mente explicitement la conformitÃ© Ã  **LOI-2** (le systÃ¨me accepte l'isolement comme Ã©tat normal).

### 9.1 Reconnaissance des Ã©tats d'isolement

Caring Nanny reconnaÃ®t les Ã©tats d'isolement suivants comme **Ã©tats normaux** :

| Ã‰tat d'isolement | Code Caring Nanny | Nature |
|------------------|-------------------|--------|
| ConnectÃ© | `healthy` ou autre selon Ã©tat | Ã‰tat nominal |
| IsolÃ© | `offline` | **Ã‰tat normal** |
| Partiellement synchronisÃ© | `syncing` | Ã‰tat transitoire |
| DÃ©gradÃ© | `degraded` | Ã‰tat normal |
| FÃ©dÃ©rÃ© | `healthy` avec flag fÃ©dÃ©ration | Ã‰tat nominal |

### 9.2 Distinction isolÃ© vs erreur

**RÃ¨gle fondamentale (conformitÃ© LOI-2) :**

> L'isolement (`offline`) n'est **jamais** classifiÃ© comme erreur (`error`).

Cette distinction est non-nÃ©gociable :

| Situation | Ã‰tat correct | Ã‰tat INTERDIT |
|-----------|--------------|---------------|
| Pas de connexion rÃ©seau | `offline` | `error` |
| DB MÃ¨re injoignable | `offline` | `error` |
| DÃ©marrage sans rÃ©seau | `offline` | `error` |
| Fonctionnement volontaire isolÃ© | `offline` | `error` |

### 9.3 CritÃ¨res de distinction

Pour classifier une situation :

| CritÃ¨re | â†’ offline | â†’ error |
|---------|-----------|---------|
| Fonctionnement local possible | âœ“ | â€” |
| Fonctionnement local impossible | â€” | âœ“ |
| Absence de connexion | âœ“ | â€” |
| Composant critique dÃ©faillant | â€” | âœ“ |
| Mode choisi explicitement | âœ“ | â€” |
| Condition anormale | â€” | âœ“ |

---

## 10. ConformitÃ© aux Lois d'Autonomie

Ce contrat garantit la conformitÃ© aux Lois d'Autonomie dÃ©finies dans [Miyukini Conceptual References - Lois Autonomie Systeme.md](..//..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dÃ©pendance externe critique

**ConformitÃ© :** âœ… Le modÃ¨le d'Ã©tat fonctionne localement. La classification des Ã©tats ne nÃ©cessite aucun appel externe.

### LOI-2 : Le systÃ¨me accepte l'isolement comme Ã©tat normal

**ConformitÃ© :** âœ… L'Ã©tat `offline` est explicitement dÃ©fini comme un Ã©tat normal (sÃ©vÃ©ritÃ© 0), distinct de l'Ã©tat `error`.

### LOI-3 : L'Ã©tat local est souverain

**ConformitÃ© :** âœ… Les Ã©tats sont dÃ©terminÃ©s Ã  partir de conditions locales. L'Ã©tat local est la source de vÃ©ritÃ© pour Caring Nanny.

### LOI-4 : Pas de temps global requis

**ConformitÃ© :** âœ… Les horodatages sont locaux (kernel Clock). Aucune comparaison inter-nÅ“uds basÃ©e sur un temps global.

### LOI-5 : Le coÃ»t doit Ãªtre proportionnel au hardware

**ConformitÃ© :** âœ… Le modÃ¨le d'Ã©tat est lÃ©ger (5 catÃ©gories, rÃ¨gles simples). Pas de structure complexe en mÃ©moire.

### LOI-6 : L'autonomie n'empÃªche pas la fÃ©dÃ©ration

**ConformitÃ© :** âœ… Les Ã©tats `syncing` et la transition `offline` â†’ `syncing` supportent la fÃ©dÃ©ration optionnelle.

---

## 11. Correspondance avec la Documentation Fondatrice

| Section Fondatrice | Couverture dans ce contrat |
|-------------------|---------------------------|
| Â§4 Ã‰tat systÃ¨me | Section 4 (CatÃ©gories d'Ã©tat systÃ¨me) |
| Â§4 Ã‰tat applicatif | Section 5 (CatÃ©gories d'Ã©tat applicatif) |
| Â§4 Transition d'Ã©tat | Section 7 (Matrice de transitions) |
| Â§4 Condition | Section 3.1 (DÃ©finitions) |
| Â§10 ConformitÃ© LOI-2 | Section 9 (Ã‰tats d'isolement) |

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit le modÃ¨le formel des Ã©tats qui doit Ãªtre respectÃ© par toute implÃ©mentation de Caring Nanny.

Les catÃ©gories d'Ã©tat, les rÃ¨gles d'agrÃ©gation, et les transitions valides sont **non-nÃ©gociables**. Toute modification nÃ©cessite une nouvelle version majeure du contrat.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT â€” ModÃ¨le d'Ã©tat normatif  
**DÃ©pendances :**  
- [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) v1.6 (Section 4)
- [Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) v1.0
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) v1.1

