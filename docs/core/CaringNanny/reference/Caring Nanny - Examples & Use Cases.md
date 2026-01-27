# Caring Nanny - Examples & Use Cases

## 1. Contexte

Ce document présente des exemples concrets et des cas d'usage de Caring Nanny, le core d'observation d'état (Strate 4) du Miyukini Core System. Ces exemples illustrent comment Caring Nanny observe, classe, et propage les états dans différents scénarios réels.

## 2. Portée / Scope

Ce document couvre :
- Des scénarios d'observation d'état courants
- Des exemples de transitions d'état
- Des cas d'usage d'intégration avec les autres cores
- Des exemples de propagation de changements d'état
- Des cas limites et comportements attendus

Ce document **ne couvre pas** :
- Les spécifications techniques détaillées (voir [Architecture et Composants](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md))
- Les contrats formels (voir les documents dans `/contracts/`)
- Les définitions normatives (voir [Documentation Fondatrice](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md))

**Note :** Ces exemples sont **conceptuels et illustratifs**. Ils ne représentent pas une implémentation technique spécifique.

---

## 3. Exemples d'observation d'état

### Exemple 1 : Observation de l'état de KindMother

**Scénario :** Un produit souhaite connaître l'état actuel de la persistance gérée par KindMother.

**Contexte :**
- KindMother gère les données du produit
- Le produit a besoin de savoir si les données sont synchronisées avant une opération critique

**Observation de Caring Nanny :**

```
┌─────────────────────────────────────────────────────────────┐
│ OBSERVATION - KindMother                                     │
├─────────────────────────────────────────────────────────────┤
│ Composant observé : KindMother                               │
│ Timestamp local   : 2026-01-27T14:32:15.123Z                 │
│                                                              │
│ Conditions observées :                                       │
│ ├─ Santé persistance    : disponible                         │
│ ├─ État synchronisation : synchronisé                        │
│ ├─ DB Mère              : accessible                         │
│ ├─ DB Filles connectées : 2/2                                │
│ └─ Écritures en attente : 0                                  │
│                                                              │
│ État évalué : healthy                                        │
└─────────────────────────────────────────────────────────────┘
```

**Résultat :** Caring Nanny rapporte que KindMother est en état `healthy`. Le produit peut procéder à son opération critique.

**Ce que Caring Nanny n'a PAS fait :**
- Elle n'a pas accédé aux données de KindMother
- Elle n'a pas déclenché de synchronisation
- Elle n'a pas validé l'opération du produit

**Référence :** [KindMother Integration Contract](../contracts/integration/Caring%20Nanny%20-%20KindMother%20Integration%20Contract.md)

---

### Exemple 2 : Observation en mode offline

**Scénario :** L'application fonctionne sans connexion réseau.

**Contexte :**
- L'utilisateur est dans un avion sans connexion
- L'application doit fonctionner normalement en mode local

**Observation de Caring Nanny :**

```
┌─────────────────────────────────────────────────────────────┐
│ OBSERVATION - État système                                   │
├─────────────────────────────────────────────────────────────┤
│ Timestamp local : 2026-01-27T14:45:22.456Z                   │
│                                                              │
│ Conditions observées :                                       │
│ ├─ Connexion réseau     : indisponible                       │
│ ├─ DB locale            : opérationnelle                     │
│ ├─ StrongFather local   : actif                              │
│ └─ Sync pending         : 12 opérations                      │
│                                                              │
│ État évalué : offline                                        │
│                                                              │
│ Note : État NORMAL (LOI-2), pas une erreur                   │
└─────────────────────────────────────────────────────────────┘
```

**Résultat :** Caring Nanny rapporte que le système est en état `offline`. Cet état est **normal** et non une erreur, conformément à LOI-2.

**Comportement attendu :**
- Le système continue de fonctionner localement
- Les opérations locales sont autorisées
- Les synchronisations sont mises en attente
- Aucune alerte d'erreur n'est générée pour l'état offline

**Référence :** [Documentation Fondatrice, Section 10 - LOI-2](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Exemple 3 : Détection d'anomalie

**Scénario :** Un composant ne répond plus dans les délais attendus.

**Contexte :**
- Un module SPM (Content) ne répond plus
- Caring Nanny détecte l'anomalie

**Observation de Caring Nanny :**

```
┌─────────────────────────────────────────────────────────────┐
│ OBSERVATION - Anomalie détectée                              │
├─────────────────────────────────────────────────────────────┤
│ Composant observé : Module SPM Content                       │
│ Timestamp local   : 2026-01-27T15:12:33.789Z                 │
│                                                              │
│ Conditions observées :                                       │
│ ├─ Temps de réponse     : >5000ms (seuil: 1000ms)            │
│ ├─ Dernière réponse     : il y a 12 secondes                 │
│ ├─ Tentatives           : 3/3 échouées                       │
│ └─ Pattern              : dégradation progressive            │
│                                                              │
│ Anomalie classifiée : Module non répondant                   │
│ État évalué (module)  : error                                │
│ Impact sur état global: degraded                             │
└─────────────────────────────────────────────────────────────┘
```

**Résultat :** Caring Nanny détecte l'anomalie, la classifie, et met à jour l'état global à `degraded`.

**Ce que Caring Nanny a fait :**
- Détecté les conditions anormales
- Classifié l'anomalie
- Mis à jour l'état du module et l'état global
- Préparé une notification pour propagation

**Ce que Caring Nanny n'a PAS fait :**
- Elle n'a pas tenté de réparer le module
- Elle n'a pas bloqué les opérations
- Elle n'a pas décidé d'une action corrective

**Référence :** [Documentation Fondatrice, Section 5.2](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

## 4. Exemples de transitions d'état

### Exemple 4 : Transition healthy → syncing → healthy

**Scénario :** Une synchronisation démarre et se termine avec succès.

**Contexte :**
- Le système était en état `healthy`
- Une synchronisation avec le serveur central commence
- La synchronisation se termine sans erreur

**Séquence de transitions :**

```
┌─────────────────────────────────────────────────────────────┐
│ TRANSITION 1                                                 │
├─────────────────────────────────────────────────────────────┤
│ Timestamp : 2026-01-27T16:00:00.000Z                         │
│ État précédent : healthy                                     │
│ État actuel    : syncing                                     │
│ Cause          : Synchronisation démarrée par KindMother     │
│ Contexte       : 45 deltas à propager                        │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRANSITION 2                                                 │
├─────────────────────────────────────────────────────────────┤
│ Timestamp : 2026-01-27T16:00:12.345Z                         │
│ État précédent : syncing                                     │
│ État actuel    : healthy                                     │
│ Cause          : Synchronisation terminée avec succès        │
│ Contexte       : 45 deltas propagés, 0 conflits              │
└─────────────────────────────────────────────────────────────┘
```

**Caractéristiques de ces transitions :**
- **Déterministes** : `healthy` peut passer à `syncing`, `syncing` peut revenir à `healthy`
- **Traçables** : Chaque transition a un timestamp, une cause, et un contexte
- **Causales** : La cause est identifiable (démarrage/fin de synchronisation)

**Référence :** [State Model Contract](../contracts/observability/Caring%20Nanny%20-%20State%20Model%20Contract.md)

---

### Exemple 5 : Transition healthy → degraded → error

**Scénario :** Dégradation progressive vers une erreur critique.

**Contexte :**
- Un problème de connexion à la base de données se développe
- La dégradation s'aggrave jusqu'à une erreur critique

**Séquence de transitions :**

```
┌─────────────────────────────────────────────────────────────┐
│ TRANSITION 1 - Dégradation initiale                          │
├─────────────────────────────────────────────────────────────┤
│ Timestamp : 2026-01-27T17:30:00.000Z                         │
│ État précédent : healthy                                     │
│ État actuel    : degraded                                    │
│ Cause          : Latence DB élevée (800ms > seuil 500ms)     │
│ Contexte       : Pool connexions à 80% capacité              │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRANSITION 2 - Erreur critique                               │
├─────────────────────────────────────────────────────────────┤
│ Timestamp : 2026-01-27T17:32:15.678Z                         │
│ État précédent : degraded                                    │
│ État actuel    : error                                       │
│ Cause          : DB inaccessible (timeout après 3 tentatives)│
│ Contexte       : Pool connexions épuisé                      │
└─────────────────────────────────────────────────────────────┘
```

**Résultat :** Caring Nanny a enregistré la dégradation progressive, permettant un diagnostic post-mortem détaillé.

**Référence :** [Observation Flow Contract](../contracts/observability/Caring%20Nanny%20-%20Observation%20Flow%20Contract.md)

---

### Exemple 6 : Transition offline → syncing → healthy (reconnexion)

**Scénario :** L'application retrouve la connexion après une période offline.

**Contexte :**
- L'utilisateur était hors ligne (avion)
- L'appareil se reconnecte au réseau
- La synchronisation des données locales commence

**Séquence de transitions :**

```
┌─────────────────────────────────────────────────────────────┐
│ ÉTAT INITIAL                                                 │
├─────────────────────────────────────────────────────────────┤
│ Timestamp : 2026-01-27T18:00:00.000Z                         │
│ État      : offline                                          │
│ Contexte  : 3 heures hors ligne, 28 opérations en attente    │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRANSITION 1 - Reconnexion détectée                          │
├─────────────────────────────────────────────────────────────┤
│ Timestamp : 2026-01-27T21:15:00.000Z                         │
│ État précédent : offline                                     │
│ État actuel    : syncing                                     │
│ Cause          : Connexion réseau rétablie                   │
│ Contexte       : Synchronisation automatique démarrée        │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRANSITION 2 - Synchronisation terminée                      │
├─────────────────────────────────────────────────────────────┤
│ Timestamp : 2026-01-27T21:15:45.123Z                         │
│ État précédent : syncing                                     │
│ État actuel    : healthy                                     │
│ Cause          : Synchronisation terminée avec succès        │
│ Contexte       : 28 opérations synchronisées, 0 conflits     │
└─────────────────────────────────────────────────────────────┘
```

**Résultat :** Caring Nanny a documenté la transition complète de offline à healthy.

**Référence :** [Documentation Fondatrice, Section 10](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

## 5. Exemples de propagation d'état

### Exemple 7 : Propagation d'un changement d'état aux composants concernés

**Scénario :** L'état passe de `healthy` à `degraded`, et les composants concernés doivent être informés.

**Contexte :**
- KindMother signale une latence élevée
- Caring Nanny détecte la transition vers `degraded`
- Les composants intéressés doivent être notifiés

**Flux de propagation :**

```
┌─────────────────────────────────────────────────────────────┐
│ PROPAGATION - Changement d'état détecté                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ 1. IDENTIFICATION DES DESTINATAIRES                          │
│    ├─ StrongFather    : oui (contexte pour décisions)        │
│    ├─ Produit App1    : oui (abonné aux états KindMother)    │
│    ├─ Produit App2    : non (pas abonné)                     │
│    └─ Module Content  : oui (dépend de KindMother)           │
│                                                              │
│ 2. MESSAGE CONSTRUIT                                         │
│    {                                                         │
│      type: "state_change",                                   │
│      source: "CaringNanny",                                  │
│      timestamp: "2026-01-27T19:00:00.000Z",                  │
│      previous_state: "healthy",                              │
│      current_state: "degraded",                              │
│      cause: "KindMother latency elevated",                   │
│      affected_component: "KindMother",                       │
│      context: { latency_ms: 850, threshold_ms: 500 }         │
│    }                                                         │
│                                                              │
│ 3. TRANSMISSION À BONDINGBROTHER                             │
│    → BondingBrother reçoit le message                        │
│    → BondingBrother distribue aux destinataires              │
│                                                              │
│ 4. ENREGISTREMENT                                            │
│    Propagation enregistrée dans HistoryStore                 │
│    ID: prop-2026-01-27-190000-001                            │
└─────────────────────────────────────────────────────────────┘
```

**Ce que Caring Nanny a fait :**
- Identifié les destinataires selon les règles d'abonnement
- Construit un message fidèle à l'observation
- Transmis le message à BondingBrother
- Enregistré la propagation pour traçabilité

**Ce que Caring Nanny n'a PAS fait :**
- Elle n'a pas distribué elle-même le message
- Elle n'a pas attendu de confirmation des destinataires
- Elle n'a pas interprété ou filtré le message

**Référence :** [Propagation Flow Contract](../contracts/observability/Caring%20Nanny%20-%20Propagation%20Flow%20Contract.md)

---

### Exemple 8 : Propagation vers StrongFather pour enrichir le contexte

**Scénario :** StrongFather doit évaluer une intention, et il consulte Caring Nanny pour le contexte d'état.

**Contexte :**
- Un produit soumet une intention de modification de données
- StrongFather doit décider si l'intention est autorisée
- L'état du système peut influencer la décision

**Flux de consultation :**

```
┌─────────────────────────────────────────────────────────────┐
│ CONSULTATION - StrongFather interroge Caring Nanny           │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ DEMANDE DE STRONGFATHER :                                    │
│ "Quel est l'état actuel de KindMother ?"                     │
│                                                              │
│ RÉPONSE DE CARING NANNY :                                    │
│ {                                                            │
│   component: "KindMother",                                   │
│   state: "syncing",                                          │
│   timestamp: "2026-01-27T19:30:00.000Z",                     │
│   context: {                                                 │
│     sync_progress: "45%",                                    │
│     pending_writes: 12,                                      │
│     estimated_completion: "30 seconds"                       │
│   }                                                          │
│ }                                                            │
│                                                              │
│ DÉCISION DE STRONGFATHER :                                   │
│ → StrongFather décide de différer l'intention                │
│ → Cette décision est prise par StrongFather, pas Caring Nanny│
└─────────────────────────────────────────────────────────────┘
```

**Ce que Caring Nanny a fait :**
- Fourni l'état actuel demandé
- Inclus le contexte pertinent
- Retourné une réponse sans effet de bord

**Ce que Caring Nanny n'a PAS fait :**
- Elle n'a pas recommandé de différer l'intention
- Elle n'a pas pris la décision de différer
- Elle n'a pas influencé le résultat de l'évaluation

**Référence :** [StrongFather Integration Contract](../contracts/integration/Caring%20Nanny%20-%20StrongFather%20Integration%20Contract.md)

---

## 6. Cas d'usage spécifiques

### Cas d'usage 1 : Surveillance de l'état pour un Tool

**Scénario :** Un Tool doit vérifier si l'environnement permet son exécution.

**Contexte :**
- Un UI Toolkit doit s'exécuter
- L'environnement pourrait être en état `SECURITY_LOCKDOWN`
- Le Tool consulte Caring Nanny avant exécution

**Flux :**

```
┌─────────────────────────────────────────────────────────────┐
│ VÉRIFICATION D'ÉTAT POUR TOOL                                │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ QUESTION : "L'état permet-il l'appel de UI Toolkit ?"        │
│                                                              │
│ OBSERVATION DE CARING NANNY :                                │
│ ├─ État environnement : healthy                              │
│ ├─ États bloquants    : aucun                                │
│ └─ Résultat           : environnement disponible             │
│                                                              │
│ RÉPONSE : État compatible avec l'exécution du Tool           │
│                                                              │
│ NOTE : La décision d'exécuter le Tool est prise par          │
│        StrongFather, pas par Caring Nanny                    │
└─────────────────────────────────────────────────────────────┘
```

**Autre scénario (état bloquant) :**

```
┌─────────────────────────────────────────────────────────────┐
│ VÉRIFICATION D'ÉTAT POUR TOOL - ÉTAT BLOQUANT                │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ OBSERVATION DE CARING NANNY :                                │
│ ├─ État environnement : SECURITY_LOCKDOWN                    │
│ ├─ États bloquants    : UI Toolkit bloqué en SECURITY_LOCKDOWN│
│ └─ Résultat           : environnement non disponible         │
│                                                              │
│ RÉPONSE : État incompatible avec l'exécution du Tool         │
│                                                              │
│ NOTE : Caring Nanny rapporte l'état, StrongFather décide     │
│        du blocage effectif                                   │
└─────────────────────────────────────────────────────────────┘
```

**Référence :** [Documentation Fondatrice, Section 3.4](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Cas d'usage 2 : Diagnostic post-mortem

**Scénario :** Un incident s'est produit, et l'équipe technique doit comprendre ce qui s'est passé.

**Contexte :**
- Une erreur critique s'est produite à 14h32
- L'équipe veut reconstituer la séquence d'événements
- Caring Nanny fournit l'historique des observations

**Interrogation de l'historique :**

```
┌─────────────────────────────────────────────────────────────┐
│ DIAGNOSTIC - Historique des observations                     │
├─────────────────────────────────────────────────────────────┤
│ Période demandée : 14:00 - 14:45                             │
│                                                              │
│ CHRONOLOGIE RECONSTITUÉE :                                   │
│                                                              │
│ 14:00:00 │ État : healthy                                    │
│          │ Tous les composants fonctionnent normalement      │
│                                                              │
│ 14:15:23 │ Condition : Latence KindMother élevée (600ms)     │
│          │ État reste : healthy (seuil: 800ms)               │
│                                                              │
│ 14:22:45 │ Condition : Latence KindMother critique (1200ms)  │
│          │ Transition : healthy → degraded                   │
│          │ Propagation envoyée à StrongFather, Produit       │
│                                                              │
│ 14:28:12 │ Condition : Pool connexions à 95%                 │
│          │ État reste : degraded                             │
│                                                              │
│ 14:32:00 │ Condition : DB timeout après 3 tentatives         │
│          │ Transition : degraded → error                     │
│          │ Propagation envoyée (alerte critique)             │
│                                                              │
│ 14:35:00 │ Condition : Connexion DB rétablie                 │
│          │ Transition : error → healthy                      │
│                                                              │
│ CONCLUSION : La dégradation progressive a commencé à 14:15   │
│ avec une latence élevée, avant l'incident critique à 14:32   │
└─────────────────────────────────────────────────────────────┘
```

**Valeur du diagnostic :**
- Séquence chronologique complète
- Causes identifiables pour chaque transition
- Contexte complet pour chaque observation
- Traçabilité des propagations effectuées

**Référence :** [Documentation Fondatrice, Section 5.5](../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)

---

### Cas d'usage 3 : Observation d'un conflit de synchronisation

**Scénario :** Deux appareils ont modifié les mêmes données en mode offline.

**Contexte :**
- Appareil A et Appareil B étaient offline
- Les deux ont modifié la même entité
- À la reconnexion, un conflit est détecté

**Observation de Caring Nanny :**

```
┌─────────────────────────────────────────────────────────────┐
│ OBSERVATION - Conflit de synchronisation                     │
├─────────────────────────────────────────────────────────────┤
│ Timestamp : 2026-01-27T20:00:00.000Z                         │
│                                                              │
│ CONDITIONS OBSERVÉES :                                       │
│ ├─ État KindMother    : syncing                              │
│ ├─ Conflits détectés  : 1                                    │
│ ├─ Entité concernée   : User#12345                           │
│ ├─ Sources conflit    : Appareil A (v3), Appareil B (v4)     │
│ └─ Résolution         : en attente                           │
│                                                              │
│ ÉTAT SYSTÈME : syncing (avec conflits en attente)            │
│                                                              │
│ NOTE : Caring Nanny observe le conflit, mais ne le résout pas│
│        La résolution est du ressort de KindMother            │
└─────────────────────────────────────────────────────────────┘
```

**Ce que Caring Nanny a fait :**
- Observé les conditions de conflit
- Rapporté l'état avec le contexte des conflits
- Enregistré l'observation pour traçabilité

**Ce que Caring Nanny n'a PAS fait :**
- Elle n'a pas résolu le conflit
- Elle n'a pas choisi quelle version conserver
- Elle n'a pas déclenché de processus de résolution

**Référence :** [KindMother Integration Contract](../contracts/integration/Caring%20Nanny%20-%20KindMother%20Integration%20Contract.md)

---

## 7. Cas limites et comportements attendus

### Cas limite 1 : États contradictoires de composants

**Scénario :** Deux composants rapportent des états apparemment contradictoires.

**Contexte :**
- Module A rapporte que la connexion est disponible
- Module B rapporte que la connexion est indisponible

**Comportement de Caring Nanny :**

```
┌─────────────────────────────────────────────────────────────┐
│ RÉSOLUTION DE CONTRADICTION                                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ OBSERVATION :                                                │
│ ├─ Module A : connexion disponible                           │
│ └─ Module B : connexion indisponible                         │
│                                                              │
│ ANALYSE (selon règles de priorité) :                         │
│ ├─ Module B a observé plus récemment (delta: 500ms)          │
│ ├─ Module B a une sonde plus critique                        │
│ └─ Règle appliquée : état le plus pessimiste prévaut         │
│                                                              │
│ ÉTAT AGRÉGÉ : degraded                                       │
│ Contexte : Connexion instable (états contradictoires)        │
│                                                              │
│ GARANTIE INV-CN-4 : L'état global est cohérent               │
│ (pas de contradiction dans la réponse finale)                │
└─────────────────────────────────────────────────────────────┘
```

**Référence :** [Invariants et Garanties](../contracts/governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)

---

### Cas limite 2 : Auto-observation de Caring Nanny

**Scénario :** Comment Caring Nanny observe-t-elle son propre état sans récursion infinie ?

**Comportement de Caring Nanny :**

```
┌─────────────────────────────────────────────────────────────┐
│ AUTO-OBSERVATION (SelfHealthReporter)                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ VÉRIFICATIONS :                                              │
│ ├─ Sondes actives      : 5/5 opérationnelles                 │
│ ├─ Capacité historique : 72% utilisé                         │
│ ├─ Connexion BB        : active                              │
│ └─ Latence interne     : 12ms (seuil: 100ms)                 │
│                                                              │
│ ÉTAT CARING NANNY : healthy                                  │
│                                                              │
│ MÉCANISME ANTI-RÉCURSION :                                   │
│ ├─ SelfHealthReporter ne s'observe PAS lui-même              │
│ ├─ Pas de récursion dans la boucle d'observation             │
│ └─ L'état de Caring Nanny est une mesure ponctuelle          │
└─────────────────────────────────────────────────────────────┘
```

**Référence :** [Architecture et Composants, Section 4.4](../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

### Cas limite 3 : Observation sans propagation

**Scénario :** Un changement d'état n'a aucun destinataire intéressé.

**Comportement de Caring Nanny :**

```
┌─────────────────────────────────────────────────────────────┐
│ PROPAGATION SANS DESTINATAIRES                               │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│ TRANSITION DÉTECTÉE :                                        │
│ Module secondaire : healthy → degraded                       │
│                                                              │
│ IDENTIFICATION DES DESTINATAIRES :                           │
│ ├─ StrongFather    : non abonné à ce module                  │
│ ├─ Produits        : aucun abonné                            │
│ └─ Autres modules  : aucun dépendant                         │
│                                                              │
│ RÉSULTAT : 0 destinataires identifiés                        │
│                                                              │
│ COMPORTEMENT :                                               │
│ ├─ Transition enregistrée dans l'historique ✓                │
│ ├─ Aucune propagation effectuée                              │
│ └─ Pas d'erreur (comportement normal)                        │
│                                                              │
│ NOTE : L'historique reste complet pour audit futur           │
└─────────────────────────────────────────────────────────────┘
```

**Référence :** [Propagation Flow Contract](../contracts/observability/Caring%20Nanny%20-%20Propagation%20Flow%20Contract.md)

---

## 8. Anti-patterns et erreurs à éviter

### Anti-pattern 1 : Demander à Caring Nanny de prendre une décision

**❌ Incorrect :**
```
"Caring Nanny, dois-je différer cette opération vu l'état du système ?"
```

**✅ Correct :**
```
"Caring Nanny, quel est l'état actuel du système ?"
→ [Réponse : degraded]
→ [Le composant décide lui-même de différer ou non]
```

---

### Anti-pattern 2 : Attendre une action corrective de Caring Nanny

**❌ Incorrect :**
```
"Caring Nanny a détecté une erreur, elle va la corriger"
```

**✅ Correct :**
```
"Caring Nanny a détecté une erreur et l'a rapportée"
"Le composant responsable (ou le produit) prend l'action corrective"
```

---

### Anti-pattern 3 : Utiliser Caring Nanny pour bloquer des opérations

**❌ Incorrect :**
```
"Caring Nanny bloque les écritures car le système est degraded"
```

**✅ Correct :**
```
"Caring Nanny rapporte que le système est degraded"
"StrongFather décide de refuser les écritures basé sur cet état"
```

---

## 9. Résumé des comportements

| Scénario | Caring Nanny fait | Caring Nanny ne fait PAS |
|----------|-------------------|--------------------------|
| État demandé | Retourne l'état avec contexte | Recommande une action |
| Anomalie détectée | Enregistre et propage | Corrige l'anomalie |
| Transition observée | Enregistre avec cause | Décide de la réponse |
| Conflit détecté | Rapporte le conflit | Résout le conflit |
| Tool vérifie état | Retourne disponibilité | Autorise/refuse le Tool |
| Mode offline | Rapporte état `offline` | Considère comme erreur |
| Propagation | Transmet à BondingBrother | Distribue directement |

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
