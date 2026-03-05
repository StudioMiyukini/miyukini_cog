# Caring Nanny - Glossaire et Terminologie

## 1. Contexte

Ce document Ã©tend et prÃ©cise le vocabulaire canonique introduit dans la Section 9 de la [Documentation Fondatrice](..//foundation//Caring%20Nanny%20-%20Documentation%20Fondatrice.md). Il Ã©tablit le dictionnaire complet et dÃ©finitif de tous les termes utilisÃ©s dans l'Ã©cosystÃ¨me Caring Nanny.

## 2. PortÃ©e / Scope

Ce document couvre :
- Les termes fondamentaux hÃ©ritÃ©s du document fondateur
- Les termes architecturaux dÃ©rivÃ©s de la structure technique
- Les termes opÃ©rationnels utilisÃ©s dans les flux
- Les termes contractuels utilisÃ©s dans les spÃ©cifications

Ce document **Ã©tablit** :
- La dÃ©finition canonique et unique de chaque terme
- Les relations entre termes
- Les usages autorisÃ©s et interdits

---

## 3. RÃ¨gles terminologiques

### 3.1 RÃ¨gle d'unicitÃ©

Chaque concept a **un seul terme** autorisÃ©. Les synonymes sont interdits dans la documentation officielle.

### 3.2 RÃ¨gle de prÃ©cision

Chaque terme a **une seule dÃ©finition**. Aucune interprÃ©tation contextuelle n'est autorisÃ©e.

### 3.3 RÃ¨gle de stabilitÃ©

Les termes sont **versionnÃ©s** avec la documentation. Un terme ne peut changer de sens qu'avec un changement de version majeure.

### 3.4 RÃ¨gle d'usage

L'usage d'un terme non dÃ©fini dans ce glossaire est **interdit** dans la documentation contractuelle.

---

## 4. Termes fondamentaux

### 4.1 Ã‰tat

**DÃ©finition :** Condition observable d'un composant ou du systÃ¨me Ã  un instant donnÃ©. Un Ã©tat reprÃ©sente une photographie de la situation Ã  un moment prÃ©cis.

**CaractÃ©ristiques :**
- Toujours catÃ©gorisÃ© selon les catÃ©gories dÃ©finies (healthy, degraded, offline, syncing, error)
- Toujours datÃ© avec un horodatage prÃ©cis (horodatage local, conforme Ã  **LOI-4** : pas de temps global requis, voir [Miyukini Framework - Lois Autonomie Systeme.md](..//..//..//miyukini-webway-system//reference//_index.md))
- Toujours contextualisÃ© avec son domaine d'observation
- Toujours cohÃ©rent (pas de contradiction interne)

**Forme canonique :**
```
Ã‰tat {
    catÃ©gorie: CatÃ©gorieÃ‰tat,
    timestamp: Horodatage,
    contexte: ContexteObservation,
    source: IdentitÃ©Composant
}
```

**Types d'Ã©tats :**

| Type | PortÃ©e | Description |
|------|--------|-------------|
| Ã‰tat systÃ¨me | Global | SynthÃ¨se de tous les Ã©tats partiels du systÃ¨me |
| Ã‰tat applicatif | Partiel | Condition d'un module ou composant spÃ©cifique |

**Termes apparentÃ©s :**
- Condition (fait observable qui peut influencer l'Ã©tat)
- CatÃ©gorie d'Ã©tat (classification de l'Ã©tat)

---

### 4.2 Observation

**DÃ©finition :** Acte par lequel Caring Nanny dÃ©tecte et enregistre une condition ou un Ã©tat. L'observation est le mÃ©canisme fondamental de Caring Nanny.

**CaractÃ©ristiques :**
- Passive : n'influence pas ce qui est observÃ©
- Non intrusive : ne perturbe pas le fonctionnement normal
- Sans effet de bord : ne modifie aucune donnÃ©e
- TraÃ§able : enregistrÃ©e avec son contexte

**Forme canonique :**
```
Observation {
    cible: IdentitÃ©Composant,
    Ã©tat_observÃ©: Ã‰tat,
    timestamp: Horodatage,
    mÃ©thode: MÃ©thodeObservation
}
```

**PropriÃ©tÃ©s de l'observation :**
- **FidÃ©litÃ©** : L'observation reflÃ¨te exactement ce qui est observÃ©
- **NeutralitÃ©** : L'observation ne juge pas, ne dÃ©cide pas
- **ComplÃ©tude** : L'observation capture toutes les informations pertinentes

**Ce que l'observation n'est pas :**
- Une action (pas de modification)
- Une dÃ©cision (pas de jugement)
- Une validation (pas d'approbation/rejet)

---

### 4.3 Transition

**DÃ©finition :** Passage d'un Ã©tat Ã  un autre. Une transition reprÃ©sente un changement observable dans le systÃ¨me.

**CaractÃ©ristiques :**
- DÃ©terministe : Un Ã©tat donnÃ© conduit Ã  un ensemble fini d'Ã©tats possibles
- Observable : La transition elle-mÃªme est un fait observable
- TraÃ§able : EnregistrÃ©e avec son contexte complet
- Causale : Toujours provoquÃ©e par une condition identifiable

**Forme canonique :**
```
Transition {
    Ã©tat_prÃ©cÃ©dent: Ã‰tat,
    Ã©tat_actuel: Ã‰tat,
    cause: Condition,
    timestamp: Horodatage
}
```

**PropriÃ©tÃ©s d'une transition :**
- **AtomicitÃ©** : Une transition est indivisible
- **IrrÃ©versibilitÃ©** : Une transition ne peut pas Ãªtre annulÃ©e (mais une transition inverse peut survenir)
- **TraÃ§abilitÃ©** : L'historique des transitions est conservÃ©

**Termes apparentÃ©s :**
- Cause (condition qui provoque la transition)
- Ã‰tat prÃ©cÃ©dent / Ã‰tat actuel

---

### 4.4 Propagation

**DÃ©finition :** MÃ©canisme par lequel un changement d'Ã©tat est communiquÃ© aux composants concernÃ©s. La propagation est une diffusion d'information, pas une modification d'Ã©tat.

**CaractÃ©ristiques :**
- Passive : Caring Nanny informe, elle ne modifie pas
- SÃ©lective : Seuls les composants concernÃ©s sont informÃ©s
- TraÃ§able : Chaque propagation est enregistrÃ©e
- Non bloquante : La propagation n'attend pas de confirmation d'action
- FidÃ¨le : L'information transmise n'est pas altÃ©rÃ©e

**Forme canonique :**
```
Propagation {
    transition: Transition,
    destinataires: Liste<IdentitÃ©Composant>,
    timestamp: Horodatage,
    canal: CanalPropagation
}
```

**Ce que la propagation ne fait pas :**
- Ne dÃ©clenche pas d'action
- Ne modifie pas l'Ã©tat transmis
- Ne filtre pas l'information (sauf le pÃ©rimÃ¨tre des destinataires)
- N'attend pas de rÃ©ponse

---

### 4.5 Condition

**DÃ©finition :** Fait observable qui peut influencer l'Ã©tat. Une condition est un Ã©lÃ©ment d'information brut, avant interprÃ©tation en termes d'Ã©tat.

**CaractÃ©ristiques :**
- Factuelle : ReprÃ©sente un fait, pas une interprÃ©tation
- Observable : Peut Ãªtre dÃ©tectÃ©e par Caring Nanny
- Temporelle : Valide Ã  un moment donnÃ©
- Contextuelle : A un contexte d'observation

**Forme canonique :**
```
Condition {
    type: TypeCondition,
    valeur: Valeur,
    timestamp: Horodatage,
    source: IdentitÃ©Composant
}
```

**Exemples de conditions :**
- La connexion rÃ©seau est disponible
- Le temps de rÃ©ponse dÃ©passe un seuil
- Un composant ne rÃ©pond pas
- Une synchronisation a Ã©chouÃ©
- L'espace disque est insuffisant

**DiffÃ©rence avec l'Ã©tat :**
- Une condition est un fait brut
- Un Ã©tat est une classification d'un ensemble de conditions

---

### 4.6 Anomalie

**DÃ©finition :** Condition qui s'Ã©carte du comportement attendu. Une anomalie signale un Ã©cart par rapport Ã  la norme, mais n'est pas nÃ©cessairement une erreur.

**CaractÃ©ristiques :**
- DÃ©tectÃ©e par Caring Nanny
- RapportÃ©e aux composants concernÃ©s
- Jamais corrigÃ©e par Caring Nanny
- Peut prÃ©cÃ©der une transition vers un Ã©tat dÃ©gradÃ© ou d'erreur

**Forme canonique :**
```
Anomalie {
    type: TypeAnomalie,
    condition_anormale: Condition,
    seuil_attendu: Valeur,
    valeur_observÃ©e: Valeur,
    timestamp: Horodatage
}
```

**Types d'anomalies :**

| Type | Description | Exemple |
|------|-------------|---------|
| Seuil dÃ©passÃ© | Une valeur dÃ©passe une limite | Temps de rÃ©ponse > 500ms |
| Pattern anormal | Un comportement inhabituel | Pic de requÃªtes anormal |
| Absence de signal | Un composant ne rÃ©pond plus | Timeout de healthcheck |
| IncohÃ©rence | DonnÃ©es contradictoires | Ã‰tats conflictuels |

**Ce que Caring Nanny fait avec une anomalie :**
- La dÃ©tecte
- L'enregistre
- La propage aux composants concernÃ©s

**Ce que Caring Nanny ne fait JAMAIS avec une anomalie :**
- La corriger
- Prendre une dÃ©cision corrective
- Bloquer des opÃ©rations

---

### 4.7 SantÃ©

**DÃ©finition :** CatÃ©gorie d'Ã©tat qui indique le niveau de fonctionnement d'un composant ou du systÃ¨me. La santÃ© est l'interprÃ©tation synthÃ©tique de l'Ã©tat.

**CatÃ©gories de santÃ© :**

| CatÃ©gorie | Signification | CaractÃ©ristiques |
|-----------|---------------|------------------|
| healthy | Fonctionnement normal | Aucune anomalie, toutes conditions nominales |
| degraded | Mode dÃ©gradÃ© | Certaines anomalies, fonctionnement partiel |
| offline | Mode dÃ©connectÃ© | Sans accÃ¨s aux autoritÃ©s centrales |
| syncing | Synchronisation en cours | OpÃ©rations potentiellement diffÃ©rÃ©es |
| error | Erreur critique | Certaines opÃ©rations impossibles |

**RÃ¨gles de catÃ©gorisation :**
- Les catÃ©gories sont mutuellement exclusives (un composant n'a qu'une seule catÃ©gorie Ã  un instant donnÃ©)
- La catÃ©gorie est dÃ©terminÃ©e par des rÃ¨gles de classification explicites
- La catÃ©gorie d'un Ã©tat systÃ¨me est agrÃ©gÃ©e depuis les catÃ©gories des Ã©tats applicatifs

---

### 4.8 Diagnostic

**DÃ©finition :** Analyse de l'historique d'observations pour identifier la cause d'un problÃ¨me. Le diagnostic utilise les donnÃ©es collectÃ©es par Caring Nanny mais n'est pas rÃ©alisÃ© par Caring Nanny.

**CaractÃ©ristiques :**
- Utilise l'historique des observations
- Utilise l'historique des transitions
- Recherche les causes racines
- Identifie les patterns

**Ce que Caring Nanny fournit pour le diagnostic :**
- L'historique complet des observations
- Les transitions avec leurs causes
- Les conditions observÃ©es
- Les anomalies dÃ©tectÃ©es

**Ce que Caring Nanny ne fait pas :**
- RÃ©aliser le diagnostic elle-mÃªme
- InterprÃ©ter les donnÃ©es
- Proposer des solutions
- Prendre des dÃ©cisions correctives

---

### 4.9 AgrÃ©gation

**DÃ©finition :** OpÃ©ration par laquelle Caring Nanny synthÃ©tise les Ã©tats partiels des composants en Ã©tat systÃ¨me global.

**CaractÃ©ristiques :**
- DÃ©terministe : MÃªmes entrÃ©es = mÃªme rÃ©sultat
- CohÃ©rente : Pas de contradiction dans le rÃ©sultat
- Reproductible : Peut Ãªtre recalculÃ©e Ã  l'identique
- DocumentÃ©e : Les rÃ¨gles d'agrÃ©gation sont explicites

**Forme canonique :**
```
AgrÃ©gation {
    Ã©tats_partiels: Liste<Ã‰tatApplicatif>,
    rÃ¨gles: RÃ¨glesAgrÃ©gation,
    rÃ©sultat: Ã‰tatSystÃ¨me,
    timestamp: Horodatage
}
```

**RÃ¨gles d'agrÃ©gation par dÃ©faut :**
- Si un Ã©tat partiel est "error", l'Ã©tat global est au minimum "degraded"
- Si tous les Ã©tats partiels sont "healthy", l'Ã©tat global est "healthy"
- Si un Ã©tat partiel est "offline", l'Ã©tat global reflÃ¨te le mode offline
- Si un Ã©tat partiel est "syncing", l'Ã©tat global peut Ãªtre "syncing"

---

### 4.10 Historique

**DÃ©finition :** Ensemble des observations enregistrÃ©es par Caring Nanny. L'historique est la mÃ©moire de l'Ã©volution du systÃ¨me dans le temps.

**CaractÃ©ristiques :**
- Complet : Toutes les observations sont enregistrÃ©es
- OrdonnÃ© : L'ordre chronologique est prÃ©servÃ©
- Immuable : L'historique n'est jamais modifiÃ© aprÃ¨s enregistrement
- Accessible : L'historique est consultable pour audit et diagnostic

**Ã‰lÃ©ments de l'historique :**
- Toutes les observations
- Toutes les transitions
- Toutes les propagations
- Toutes les anomalies dÃ©tectÃ©es
- Tous les Ã©tats calculÃ©s

**PropriÃ©tÃ©s de l'historique :**
- **IntÃ©gritÃ©** : Aucune perte d'information
- **AuthenticitÃ©** : Aucune modification possible
- **TraÃ§abilitÃ©** : Chaque entrÃ©e est horodatÃ©e et contextualisÃ©e

---

## 5. Termes des catÃ©gories d'Ã©tat

### 5.1 Healthy

**DÃ©finition :** CatÃ©gorie d'Ã©tat indiquant un fonctionnement normal. Aucune anomalie n'a Ã©tÃ© dÃ©tectÃ©e, toutes les conditions sont nominales.

**CaractÃ©ristiques :**
- Tous les composants observÃ©s fonctionnent normalement
- Aucun seuil n'est dÃ©passÃ©
- Aucune anomalie n'est active
- Toutes les dÃ©pendances sont disponibles

**Transition depuis healthy :**
- Vers degraded : si une anomalie non critique est dÃ©tectÃ©e
- Vers offline : si la connexion aux autoritÃ©s est perdue
- Vers syncing : si une synchronisation est dÃ©clenchÃ©e
- Vers error : si une erreur critique survient

---

### 5.2 Degraded

**DÃ©finition :** CatÃ©gorie d'Ã©tat indiquant un fonctionnement partiel ou dÃ©gradÃ©. Le systÃ¨me reste opÃ©rationnel mais certaines anomalies ont Ã©tÃ© dÃ©tectÃ©es.

**CaractÃ©ristiques :**
- Le systÃ¨me est toujours fonctionnel
- Certaines fonctionnalitÃ©s peuvent Ãªtre limitÃ©es
- Des anomalies sont actives
- Une intervention peut Ãªtre nÃ©cessaire

**Transition depuis degraded :**
- Vers healthy : si les anomalies sont rÃ©solues
- Vers offline : si la connexion aux autoritÃ©s est perdue
- Vers syncing : si une synchronisation est dÃ©clenchÃ©e
- Vers error : si une erreur critique survient

---

### 5.3 Offline

**DÃ©finition :** CatÃ©gorie d'Ã©tat indiquant un fonctionnement en mode dÃ©connectÃ©. Le systÃ¨me fonctionne sans accÃ¨s aux autoritÃ©s centrales.

**CaractÃ©ristiques :**
- Le systÃ¨me fonctionne localement
- Les autoritÃ©s centrales ne sont pas accessibles
- Certaines opÃ©rations sont diffÃ©rÃ©es
- Les donnÃ©es locales sont utilisÃ©es

**ConformitÃ© LOI-2 :** L'Ã©tat `offline` est reconnu comme un **Ã©tat normal**, pas comme une erreur. Cette distinction respecte **LOI-2** (le systÃ¨me accepte l'isolement comme Ã©tat normal) dÃ©finie dans [Miyukini Framework - Lois Autonomie Systeme.md](..//..//..//miyukini-webway-system//reference//_index.md).

**Transition depuis offline :**
- Vers syncing : si la connexion est rÃ©tablie
- Vers degraded : si des problÃ¨mes locaux surviennent
- Vers error : si une erreur critique survient

---

### 5.4 Syncing

**DÃ©finition :** CatÃ©gorie d'Ã©tat indiquant une synchronisation en cours. Le systÃ¨me est en train de rÃ©concilier son Ã©tat avec les autoritÃ©s centrales.

**CaractÃ©ristiques :**
- Une synchronisation est active
- Certaines opÃ©rations peuvent Ãªtre diffÃ©rÃ©es
- L'Ã©tat peut Ãªtre temporairement incohÃ©rent
- La durÃ©e est normalement limitÃ©e

**Transition depuis syncing :**
- Vers healthy : si la synchronisation rÃ©ussit sans anomalie
- Vers degraded : si la synchronisation rÃ©vÃ¨le des problÃ¨mes
- Vers offline : si la connexion est perdue pendant la synchronisation
- Vers error : si la synchronisation Ã©choue de maniÃ¨re critique

---

### 5.5 Error

**DÃ©finition :** CatÃ©gorie d'Ã©tat indiquant une erreur critique. Certaines opÃ©rations ne sont pas possibles et une intervention est requise.

**CaractÃ©ristiques :**
- Une erreur critique a Ã©tÃ© dÃ©tectÃ©e
- Certaines opÃ©rations sont impossibles
- Une intervention est requise
- Le systÃ¨me peut Ãªtre partiellement non fonctionnel

**Transition depuis error :**
- Vers degraded : si l'erreur est partiellement rÃ©solue
- Vers healthy : si l'erreur est complÃ¨tement rÃ©solue
- Vers offline : si la connexion est perdue

---

## 6. Termes architecturaux

### 6.1 Observateur

**DÃ©finition :** EntitÃ© responsable de la collecte des conditions et de la dÃ©tection des changements d'Ã©tat. Caring Nanny est l'observateur d'Ã©tat privilÃ©giÃ© du systÃ¨me.

**CaractÃ©ristiques :**
- Passif : N'influence pas ce qu'il observe
- PrivilÃ©giÃ© : A accÃ¨s Ã  l'information d'Ã©tat de tous les composants
- Unique : Il n'y a qu'un seul observateur d'Ã©tat systÃ¨me
- Non autoritaire : N'a aucun pouvoir de dÃ©cision

**PropriÃ©tÃ©s d'un observateur :**
- **PuretÃ©** : Aucun effet de bord
- **FidÃ©litÃ©** : Observation exacte de la rÃ©alitÃ©
- **ExhaustivitÃ©** : Observation de tous les aspects pertinents

---

### 6.2 Source d'Ã©tat

**DÃ©finition :** Composant qui produit des informations d'Ã©tat observables par Caring Nanny.

**Sources d'Ã©tat dans l'Ã©cosystÃ¨me :**

| Source | Type d'information | Exemple |
|--------|-------------------|---------|
| KindMother | Ã‰tat de persistance et synchronisation | DB disponible, en sync |
| StrongFather | Ã‰tat des politiques et permissions | Politique active, suspendue |
| BondingBrother | Ã‰tat de la mÃ©diation | Canal actif, congestionnÃ© |
| Modules SPM | Ã‰tat applicatif | Module chargÃ©, erreur de schÃ©ma |

**Ce qu'une source d'Ã©tat fournit :**
- Des conditions observables
- Des mÃ©triques de santÃ©
- Des signaux de changement

---

### 6.3 Canal d'observation

**DÃ©finition :** Voie par laquelle les conditions transitent depuis les sources d'Ã©tat vers Caring Nanny.

**Types de canaux :**

| Type | Mode | Description |
|------|------|-------------|
| Push | Actif | La source envoie les conditions |
| Pull | Passif | Caring Nanny interroge la source |
| Event | RÃ©actif | La source Ã©met des Ã©vÃ©nements |

**PropriÃ©tÃ©s d'un canal :**
- Fiable : Pas de perte d'information
- OrdonnÃ© : L'ordre des conditions est prÃ©servÃ©
- Non intrusif : N'impacte pas les performances

---

### 6.4 Destinataire

**DÃ©finition :** Composant qui reÃ§oit les notifications de changement d'Ã©tat propagÃ©es par Caring Nanny.

**Types de destinataires :**
- StrongFather (pour enrichir le contexte des dÃ©cisions)
- BondingBrother (pour propager aux produits)
- Modules SPM (pour rÃ©agir aux changements)
- Produits (via BondingBrother)

**Ce qu'un destinataire reÃ§oit :**
- La transition d'Ã©tat
- L'Ã©tat prÃ©cÃ©dent et l'Ã©tat actuel
- La cause de la transition
- L'horodatage

**Ce qu'un destinataire ne reÃ§oit pas :**
- D'instructions d'action
- De dÃ©cisions
- D'informations hors de son pÃ©rimÃ¨tre

---

## 7. Termes opÃ©rationnels

### 7.1 Flux d'observation

**DÃ©finition :** SÃ©quence d'Ã©tapes par laquelle Caring Nanny collecte et traite l'information d'Ã©tat.

**Ã‰tapes du flux :**
1. DÃ©tection de condition
2. Ã‰valuation de l'Ã©tat
3. AgrÃ©gation
4. DÃ©tection de transition

**PropriÃ©tÃ©s du flux :**
- Ordre strict des Ã©tapes
- Pas de saut d'Ã©tape
- TraÃ§abilitÃ© Ã  chaque Ã©tape
- Non bloquant

---

### 7.2 Flux de propagation

**DÃ©finition :** SÃ©quence d'Ã©tapes par laquelle Caring Nanny communique les changements d'Ã©tat.

**Ã‰tapes du flux :**
1. Identification des destinataires
2. Formulation du message
3. DÃ©lÃ©gation Ã  BondingBrother
4. Enregistrement dans l'historique

**PropriÃ©tÃ©s du flux :**
- SÃ©lectif (seuls les destinataires concernÃ©s)
- FidÃ¨le (pas d'altÃ©ration du message)
- TraÃ§able (enregistrement complet)

---

### 7.3 Flux de consultation

**DÃ©finition :** SÃ©quence d'Ã©tapes par laquelle un composant interroge Caring Nanny sur l'Ã©tat actuel.

**Ã‰tapes du flux :**
1. RÃ©ception de la demande d'Ã©tat
2. RÃ©cupÃ©ration de l'Ã©tat demandÃ©
3. Retour de l'Ã©tat avec contexte

**PropriÃ©tÃ©s du flux :**
- Sans effet de bord (la consultation ne modifie rien)
- Synchrone (rÃ©ponse immÃ©diate)
- ContextualisÃ© (horodatage inclus)

---

### 7.4 Classification

**DÃ©finition :** Processus par lequel une condition ou un ensemble de conditions est traduit en catÃ©gorie d'Ã©tat.

**CaractÃ©ristiques :**
- BasÃ©e sur des rÃ¨gles explicites
- DÃ©terministe (mÃªmes conditions = mÃªme catÃ©gorie)
- DocumentÃ©e (rÃ¨gles consultables)

**Ã‰lÃ©ments de classification :**
- Seuils (valeurs limites)
- Patterns (combinaisons de conditions)
- PrioritÃ©s (en cas de conflit)

---

### 7.5 Notification

**DÃ©finition :** Message envoyÃ© par Caring Nanny pour informer d'un changement d'Ã©tat.

**Forme canonique :**
```
Notification {
    type: TypeNotification,
    transition: Transition,
    destinataires: Liste<IdentitÃ©Composant>,
    timestamp: Horodatage
}
```

**Types de notifications :**
- Transition d'Ã©tat systÃ¨me
- Transition d'Ã©tat applicatif
- Anomalie dÃ©tectÃ©e
- Retour Ã  la normale

**PropriÃ©tÃ©s d'une notification :**
- Informative (pas directive)
- ComplÃ¨te (toutes les informations nÃ©cessaires)
- TraÃ§able (enregistrÃ©e dans l'historique)

---

### 7.6 Seuil

**DÃ©finition :** Valeur limite qui dÃ©termine si une condition est normale ou anormale.

**Types de seuils :**

| Type | Description | Exemple |
|------|-------------|---------|
| Seuil d'alerte | Valeur de vigilance | Temps de rÃ©ponse > 200ms |
| Seuil critique | Valeur de dÃ©gradation | Temps de rÃ©ponse > 500ms |
| Seuil d'erreur | Valeur d'erreur | Temps de rÃ©ponse > 2000ms |

**PropriÃ©tÃ©s d'un seuil :**
- Configurable (dÃ©fini par le produit ou l'Ã©cosystÃ¨me)
- DocumentÃ© (valeur et unitÃ© explicites)
- VersionnÃ© (historique des changements)

---

## 8. Termes contractuels

### 8.1 Contrat

**DÃ©finition :** Document normatif qui dÃ©finit les rÃ¨gles, interfaces, ou comportements que Caring Nanny s'engage Ã  respecter.

**Types de contrats :**

| Type | PortÃ©e | Exemple |
|------|--------|---------|
| Contrat de modÃ¨le | Structure des donnÃ©es | State Model Contract |
| Contrat de flux | Comportement des flux | Observation Flow Contract |
| Contrat d'intÃ©gration | Interactions avec autres composants | KindMother Integration Contract |
| Contrat opÃ©rationnel | PropriÃ©tÃ©s de fonctionnement | Performance Contract |

---

### 8.2 Invariant

**DÃ©finition :** PropriÃ©tÃ© qui doit toujours Ãªtre vraie, quelles que soient les circonstances, et qui ne peut jamais Ãªtre violÃ©e.

**Invariants de Caring Nanny (INV-CN) :**

| ID | Invariant | Description |
|----|-----------|-------------|
| INV-CN-1 | Observateur pur | Caring Nanny observe mais ne modifie jamais |
| INV-CN-2 | Aucune capacitÃ© d'exÃ©cution | Caring Nanny ne peut dÃ©clencher aucune action |
| INV-CN-3 | Non-autoritaire | Caring Nanny ne dÃ©tient aucune autoritÃ© |
| INV-CN-4 | Ã‰tat cohÃ©rent | L'Ã©tat rapportÃ© est toujours sans contradiction |
| INV-CN-5 | TraÃ§abilitÃ© complÃ¨te | Tout est enregistrÃ© et auditable |
| INV-CN-6 | Non-bloquant | Caring Nanny ne bloque jamais les opÃ©rations |
| INV-CN-7 | Propagation fidÃ¨le | L'information transmise n'est jamais altÃ©rÃ©e |

**PropriÃ©tÃ©s d'un invariant :**
- Non nÃ©gociable
- Non configurable
- Non dÃ©sactivable
- VÃ©rifiÃ© structurellement

---

### 8.3 Garantie

**DÃ©finition :** Engagement de Caring Nanny envers ses consommateurs (composants ou produits) sur un comportement ou une propriÃ©tÃ©.

**Exemples de garanties :**
- Vision cohÃ©rente de l'Ã©tat (pas de contradiction)
- Observation sans effet de bord (pas de modification)
- Transitions traÃ§ables et auditables (historique complet)
- Propagation fidÃ¨le et non altÃ©rÃ©e (information exacte)

**DiffÃ©rence avec l'invariant :**
- L'invariant est interne (Caring Nanny s'impose Ã  elle-mÃªme)
- La garantie est externe (Caring Nanny promet aux autres)

---

### 8.4 Violation

**DÃ©finition :** Situation oÃ¹ une rÃ¨gle, un invariant, ou un contrat n'est pas respectÃ©.

**Traitement des violations :**
- Violations d'invariant : Impossible par construction (erreur de conception si dÃ©tectÃ©e)
- Violations de contrat : Journalisation, notification, signalement

**Note :** Caring Nanny ne prend aucune action corrective en cas de violation. Elle se limite Ã  observer et rapporter.

---

## 9. Termes interdits

Les termes suivants sont **interdits** dans la documentation de Caring Nanny car ils sont ambigus ou porteurs de mauvaises connotations :

| Terme interdit | Raison | Terme Ã  utiliser |
|----------------|--------|------------------|
| DÃ©cision | Caring Nanny ne dÃ©cide pas | Classification ou CatÃ©gorisation |
| Action | Caring Nanny n'agit pas | Observation ou Propagation |
| Correction | Caring Nanny ne corrige pas | DÃ©tection ou Signalement |
| Commande | Caring Nanny ne commande pas | Notification |
| ContrÃ´le | Implique une autoritÃ© | Observation |
| Validation | Caring Nanny ne valide pas | Classification |
| Blocage | Caring Nanny ne bloque pas | (aucun Ã©quivalent â€” concept interdit) |
| ExÃ©cution | Caring Nanny n'exÃ©cute pas | (aucun Ã©quivalent â€” concept interdit) |
| Modification | Caring Nanny ne modifie pas | (aucun Ã©quivalent â€” concept interdit) |
| Cache | Implique un stockage actif | Historique |

---

## 10. Index alphabÃ©tique

| Terme | Section | CatÃ©gorie |
|-------|---------|-----------|
| AgrÃ©gation | 4.9 | Fondamental |
| Anomalie | 4.6 | Fondamental |
| Canal d'observation | 6.3 | Architectural |
| Classification | 7.4 | OpÃ©rationnel |
| Condition | 4.5 | Fondamental |
| Contrat | 8.1 | Contractuel |
| Degraded | 5.2 | CatÃ©gorie d'Ã©tat |
| Destinataire | 6.4 | Architectural |
| Diagnostic | 4.8 | Fondamental |
| Error | 5.5 | CatÃ©gorie d'Ã©tat |
| Ã‰tat | 4.1 | Fondamental |
| Flux d'observation | 7.1 | OpÃ©rationnel |
| Flux de consultation | 7.3 | OpÃ©rationnel |
| Flux de propagation | 7.2 | OpÃ©rationnel |
| Garantie | 8.3 | Contractuel |
| Healthy | 5.1 | CatÃ©gorie d'Ã©tat |
| Historique | 4.10 | Fondamental |
| Invariant | 8.2 | Contractuel |
| Notification | 7.5 | OpÃ©rationnel |
| Observateur | 6.1 | Architectural |
| Observation | 4.2 | Fondamental |
| Offline | 5.3 | CatÃ©gorie d'Ã©tat |
| Propagation | 4.4 | Fondamental |
| SantÃ© | 4.7 | Fondamental |
| Seuil | 7.6 | OpÃ©rationnel |
| Source d'Ã©tat | 6.2 | Architectural |
| Syncing | 5.4 | CatÃ©gorie d'Ã©tat |
| Transition | 4.3 | Fondamental |
| Violation | 8.4 | Contractuel |

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut RÃ‰FÃ‰RENCE**. Il Ã©tablit le vocabulaire officiel de Caring Nanny qui doit Ãªtre utilisÃ© dans toute documentation, code, et communication.

Tout terme utilisÃ© dans un document contractuel de Caring Nanny doit Ãªtre dÃ©fini dans ce glossaire. Toute modification terminologique nÃ©cessite une nouvelle version de ce document.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** RÃ‰FÃ‰RENCE â€” Normatif  
**DÃ©pendance :** Documentation Fondatrice v1.0 (Section 9)


