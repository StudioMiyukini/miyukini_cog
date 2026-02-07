# Ever Buddy — Evolution Scenarios

## Contexte

Ce document présente les **scénarios d'évolution types** qui illustrent comment Ever Buddy gouverne les transitions de cycle de vie dans le Miyukini Core System. Ces scénarios représentent les cas d'usage les plus courants et servent de référence pour comprendre les mécanismes d'évolution.

**Document source :** [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) — Section 10  
**Terminologie :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## Portée / Scope

- **Ce document couvre :** 5 scénarios d'évolution types avec étapes détaillées, exemples, et conditions
- **Ce document ne couvre pas :** L'implémentation technique des transitions, les APIs spécifiques
- **Audience :** Architectes, développeurs, intégrateurs
- **Statut :** Document de référence non négociable

---

## Scénarios d'évolution

Ever Buddy gouverne 5 scénarios fondamentaux d'évolution qui couvrent la majorité des cas d'usage dans l'écosystème Miyukini.

| Scénario | Description | Fréquence |
|----------|-------------|-----------|
| **Scénario 1** | Évolution mineure rétrocompatible | Très fréquent |
| **Scénario 2** | Évolution majeure avec rupture | Occasionnel |
| **Scénario 3** | Abandon sans successeur | Rare |
| **Scénario 4** | Réactivation d'un élément déprécié | Exceptionnel |
| **Scénario 5** | Dette structurelle excessive | Variable |

---

## Scénario 1 : Évolution mineure rétrocompatible

### Description

Un élément ACTIVE reçoit une amélioration qui **ne rompt pas la compatibilité** avec les consommateurs existants. C'est le scénario d'évolution le plus courant et le plus souhaitable.

### Conditions préalables

| Condition | Description |
|-----------|-------------|
| État initial | Élément en état **ACTIVE** |
| Type de changement | Ajout de fonctionnalité ou amélioration |
| Impact compatibilité | **Rétrocompatible** — aucun consommateur existant n'est affecté |
| Version | Incrémentation **mineure** (ex: v2.1 → v2.2) |

### Étapes du scénario

```
┌─────────────────────────────────────────────────────────────────┐
│                     SCÉNARIO 1 : ÉVOLUTION MINEURE              │
├─────────────────────────────────────────────────────────────────┤
│  1. Développement et test de l'amélioration                     │
│                         │                                       │
│                         ▼                                       │
│  2. Ever Buddy valide : changement rétrocompatible              │
│     → Version mineure autorisée                                 │
│                         │                                       │
│                         ▼                                       │
│  3. Publication de la nouvelle version                          │
│     → Ancienne version toujours disponible                      │
│                         │                                       │
│                         ▼                                       │
│  4. Consommateurs migrent à leur rythme                         │
│     → Aucune contrainte temporelle                              │
│                         │                                       │
│                         ▼                                       │
│  5. Coexistence naturelle des versions                          │
│     → Aucune dépréciation nécessaire                            │
└─────────────────────────────────────────────────────────────────┘
```

**Étape 1 — Développement et test**

L'amélioration est développée en respectant les règles de rétrocompatibilité :
- Ajout de nouvelles fonctions sans modification des signatures existantes
- Extension de schémas sans suppression de champs
- Ajout de comportements optionnels sans impact sur le comportement par défaut

**Étape 2 — Validation par Ever Buddy**

Ever Buddy analyse le changement et confirme sa rétrocompatibilité :
- Vérification des signatures de contrat préservées
- Vérification de l'absence de breaking changes
- Attribution de la version mineure appropriée

**Étape 3 — Publication**

La nouvelle version est publiée avec garantie de coexistence :
- L'ancienne version reste disponible et supportée
- Les deux versions sont fonctionnellement équivalentes pour les consommateurs existants
- La documentation des nouvelles fonctionnalités est disponible

**Étape 4 — Migration libre**

Les consommateurs adoptent la nouvelle version selon leurs besoins :
- Aucune pression temporelle
- Migration possible sans modification du code consommateur
- Accès aux nouvelles fonctionnalités par opt-in

**Étape 5 — Coexistence**

Les deux versions coexistent naturellement :
- Pas de période de dépréciation obligatoire
- Ever Buddy maintient le registre des versions actives
- Le debt ratio n'est pas impacté (pas d'élément DEPRECATED)

### Exemple concret

> **Contexte :** Le contrat `KindMother - WriteIntent Contract` v2.1 doit supporter un nouveau type de métadonnées optionnelles.
>
> **Évolution :**
> - v2.1 : WriteIntent avec métadonnées standard
> - v2.2 : WriteIntent avec métadonnées standard + métadonnées étendues optionnelles
>
> **Résultat :** Les consommateurs utilisant v2.1 continuent de fonctionner sans modification. Les consommateurs qui souhaitent les métadonnées étendues peuvent migrer vers v2.2 à leur convenance.

### Invariants respectés

| Invariant | Respect |
|-----------|---------|
| **INV-EB-5** | ✅ Rétrocompatibilité par défaut |
| **INV-EB-7** | ✅ Documentation de la nouvelle version |
| **INV-EB-9** | ✅ Règles de transition prévisibles |

---

## Scénario 2 : Évolution majeure avec rupture

### Description

Un élément ACTIVE nécessite un **changement incompatible** (breaking change). Ce scénario requiert une période de transition contrôlée avec dépréciation de l'ancienne version.

### Conditions préalables

| Condition | Description |
|-----------|-------------|
| État initial | Élément en état **ACTIVE** |
| Type de changement | Modification structurelle incompatible |
| Impact compatibilité | **Breaking change** — consommateurs existants impactés |
| Version | Incrémentation **majeure** (ex: v2.x → v3.0) |
| Justification | Obligatoire et documentée |

### Étapes du scénario

```
┌─────────────────────────────────────────────────────────────────┐
│                SCÉNARIO 2 : ÉVOLUTION MAJEURE                   │
├─────────────────────────────────────────────────────────────────┤
│  1. Développement du successeur (état DRAFT)                    │
│                         │                                       │
│                         ▼                                       │
│  2. Ever Buddy enregistre le plan de transition                 │
│     → Justification documentée                                  │
│                         │                                       │
│                         ▼                                       │
│  3. Ancienne version → DEPRECATED avec annonce                  │
│     → Successeur identifié                                      │
│                         │                                       │
│                         ▼                                       │
│  4. Période de dépréciation (coexistence)                       │
│     → Surveillance du taux d'adoption                           │
│                         │                                       │
│                         ▼                                       │
│  5. Fin de période → Ancienne version → RETIRED                 │
│     → Consommateurs migrés notifiés                             │
│                         │                                       │
│                         ▼                                       │
│  6. Période de grâce                                            │
│     → Corrections critiques uniquement                          │
│                         │                                       │
│                         ▼                                       │
│  7. Ancienne version → ARCHIVED                                 │
│     → Conservée pour référence historique                       │
└─────────────────────────────────────────────────────────────────┘
```

**Étape 1 — Développement du successeur**

Le successeur est développé en état DRAFT :
- Conception de la nouvelle structure
- Documentation des différences avec l'ancienne version
- Préparation du guide de migration

**Étape 2 — Enregistrement du plan de transition**

Ever Buddy enregistre formellement le plan de transition :
- Justification du breaking change documentée
- Période de dépréciation définie
- Critères de complétion de la transition établis
- Chemin de migration documenté

**Étape 3 — Passage à DEPRECATED**

L'ancienne version passe à l'état DEPRECATED :
- Annonce officielle à tous les consommateurs
- Successeur clairement identifié (INV-EB-10)
- Date de fin de dépréciation communiquée
- Guide de migration disponible

**Étape 4 — Période de dépréciation**

Coexistence contrôlée des deux versions :
- Ever Buddy surveille le taux d'adoption du successeur
- Alertes aux consommateurs non migrés
- Support maintenu pour les deux versions
- Debt ratio en augmentation (élément DEPRECATED)

**Étape 5 — Passage à RETIRED**

À la fin de la période de dépréciation :
- Vérification que le taux d'adoption est suffisant
- Passage de l'ancienne version à RETIRED
- Notification aux derniers consommateurs
- Fin du support actif

**Étape 6 — Période de grâce**

Temps supplémentaire pour les retardataires :
- Corrections critiques de sécurité uniquement
- Aucune nouvelle fonctionnalité
- Avertissements renforcés

**Étape 7 — Archivage**

Fin de vie définitive :
- Passage à ARCHIVED
- Conservation pour référence historique uniquement
- Aucune garantie de fonctionnement

### Exemple concret

> **Contexte :** Le contrat `StrongFather - Intent Model Contract` v1.x doit être restructuré pour supporter les Mandats de Permission.
>
> **Évolution :**
> - v1.x : Intent Model sans notion de mandat
> - v2.0 : Intent Model avec support des mandats (breaking change sur la structure)
>
> **Plan de transition :**
> 1. v2.0 développée et publiée (DRAFT → ACTIVE)
> 2. v1.x passe à DEPRECATED avec période de 2 cycles de release
> 3. Guide de migration publié
> 4. Surveillance du taux d'adoption
> 5. v1.x → RETIRED après période de dépréciation
> 6. v1.x → ARCHIVED après période de grâce

### Invariants respectés

| Invariant | Respect |
|-----------|---------|
| **INV-EB-4** | ✅ Passage obligatoire par DEPRECATED |
| **INV-EB-6** | ✅ Vision long terme — impact sur 2 générations |
| **INV-EB-7** | ✅ Documentation complète de la transition |
| **INV-EB-10** | ✅ Unicité du successeur déclaré |
| **INV-EB-12** | ✅ Responsabilité de l'annonce |

### Matrice des transitions

| Étape | État avant | État après | Condition |
|-------|------------|------------|-----------|
| 1 | — | DRAFT | Nouveau développement |
| 2 | DRAFT | ACTIVE | Validation et publication |
| 3 | ACTIVE | DEPRECATED | Annonce et successeur |
| 5 | DEPRECATED | RETIRED | Fin période dépréciation |
| 7 | RETIRED | ARCHIVED | Fin période de grâce |

---

## Scénario 3 : Abandon sans successeur

### Description

Un élément ACTIVE n'est plus utile et **n'a pas de successeur**. Ce scénario représente la fin de vie naturelle d'un élément devenu obsolète ou redondant.

### Conditions préalables

| Condition | Description |
|-----------|-------------|
| État initial | Élément en état **ACTIVE** |
| Type de changement | Retrait définitif |
| Successeur | **Aucun** — abandon explicite |
| Justification | Obligatoire et documentée |

### Étapes du scénario

```
┌─────────────────────────────────────────────────────────────────┐
│                SCÉNARIO 3 : ABANDON SANS SUCCESSEUR             │
├─────────────────────────────────────────────────────────────────┤
│  1. Ever Buddy enregistre la décision d'abandon                 │
│     → Justification documentée                                  │
│                         │                                       │
│                         ▼                                       │
│  2. Élément → DEPRECATED avec annonce explicite                 │
│     → "Pas de successeur"                                       │
│                         │                                       │
│                         ▼                                       │
│  3. Consommateurs avertis de retirer leur dépendance            │
│     → Recommandations de remplacement (si applicable)           │
│                         │                                       │
│                         ▼                                       │
│  4. Période de dépréciation s'écoule                            │
│     → Surveillance des dépendances restantes                    │
│                         │                                       │
│                         ▼                                       │
│  5. Élément → RETIRED                                           │
│     → Fin du support                                            │
│                         │                                       │
│                         ▼                                       │
│  6. Élément → ARCHIVED                                          │
│     → Conservation pour référence historique                    │
└─────────────────────────────────────────────────────────────────┘
```

**Étape 1 — Enregistrement de la décision**

Ever Buddy enregistre formellement la décision d'abandon :
- Justification de l'obsolescence
- Analyse d'impact sur les consommateurs
- Recommandations de remplacement (si applicable)

**Étape 2 — Passage à DEPRECATED**

L'élément passe à DEPRECATED avec annonce explicite :
- Message clair : "Cet élément sera retiré, aucun successeur prévu"
- Date de fin de dépréciation communiquée
- Recommandations alternatives (si d'autres éléments peuvent couvrir les besoins)

**Étape 3 — Notification aux consommateurs**

Les consommateurs sont avertis de retirer leur dépendance :
- Liste des consommateurs identifiés
- Conseils de migration vers des alternatives
- Support pendant la période de transition

**Étape 4 — Période de dépréciation**

La période de dépréciation s'écoule :
- Surveillance des dépendances restantes
- Alertes aux consommateurs non adaptés
- Aucune nouvelle fonctionnalité

**Étape 5 — Passage à RETIRED**

À la fin de la période de dépréciation :
- Élément retiré du système actif
- Corrections critiques de sécurité uniquement
- Période de grâce pour les retardataires

**Étape 6 — Archivage**

Fin de vie définitive :
- Conservation pour référence historique
- Aucune garantie de fonctionnement

### Exemple concret

> **Contexte :** Un adaptateur spécifique `CMS-LegacyImporter` n'est plus utilisé car le format de données legacy n'est plus supporté par aucun système.
>
> **Processus d'abandon :**
> 1. Décision d'abandon enregistrée : "Format legacy abandonné par tous les systèmes sources"
> 2. `CMS-LegacyImporter` → DEPRECATED avec message : "Aucun successeur, format legacy obsolète"
> 3. Recommandation aux 2 consommateurs restants de migrer vers d'autres sources de données
> 4. Période de dépréciation de 1 cycle de release
> 5. `CMS-LegacyImporter` → RETIRED
> 6. `CMS-LegacyImporter` → ARCHIVED

### Invariants respectés

| Invariant | Respect |
|-----------|---------|
| **INV-EB-4** | ✅ Passage obligatoire par DEPRECATED |
| **INV-EB-7** | ✅ Documentation de l'abandon |
| **INV-EB-10** | ✅ Successeur explicitement "aucun" |
| **INV-EB-12** | ✅ Annonce claire aux consommateurs |

---

## Scénario 4 : Réactivation d'un élément déprécié

### Description

Le successeur prévu est annulé ou échoue, l'élément déprécié doit être **réactivé**. Ce scénario est exceptionnel et requiert une justification forte.

### Conditions préalables

| Condition | Description |
|-----------|-------------|
| État initial | Élément en état **DEPRECATED** |
| Cause | Annulation ou échec du successeur |
| Condition | Élément déprécié encore **fonctionnel** |
| Justification | **Obligatoire** et documentée |

### Étapes du scénario

```
┌─────────────────────────────────────────────────────────────────┐
│             SCÉNARIO 4 : RÉACTIVATION D'UN ÉLÉMENT              │
├─────────────────────────────────────────────────────────────────┤
│  1. Constat : successeur annulé ou échoué                       │
│                         │                                       │
│                         ▼                                       │
│  2. Ever Buddy vérifie que l'élément est encore fonctionnel     │
│     → Analyse d'intégrité                                       │
│                         │                                       │
│                         ▼                                       │
│  3. Décision de réactivation documentée                         │
│     → Justification complète                                    │
│                         │                                       │
│                         ▼                                       │
│  4. Élément → ACTIVE (depuis DEPRECATED)                        │
│     → Transition exceptionnelle                                 │
│                         │                                       │
│                         ▼                                       │
│  5. Consommateurs informés de la réactivation                   │
│     → Annulation de la migration vers le successeur             │
│                         │                                       │
│                         ▼                                       │
│  6. Historique conserve la trace de la dépréciation temporaire  │
│     → Traçabilité complète                                      │
└─────────────────────────────────────────────────────────────────┘
```

**Étape 1 — Constat d'échec du successeur**

Le successeur prévu ne peut pas être déployé :
- Problèmes techniques insurmontables
- Changement de stratégie
- Ressources insuffisantes
- Incompatibilité découverte tardivement

**Étape 2 — Vérification de fonctionnalité**

Ever Buddy vérifie que l'élément déprécié est encore fonctionnel :
- Tests d'intégrité
- Vérification des dépendances
- Analyse de la dette technique accumulée

**Étape 3 — Documentation de la réactivation**

La décision de réactivation est documentée :
- Raison de l'échec du successeur
- Analyse de l'impact de la réactivation
- Plan pour le futur (nouveau successeur prévu ?)

**Étape 4 — Transition DEPRECATED → ACTIVE**

L'élément est réactivé :
- Transition exceptionnelle autorisée par la matrice des transitions
- État passe de DEPRECATED à ACTIVE
- Compteur de version préservé

**Étape 5 — Communication**

Les consommateurs sont informés :
- Annonce de réactivation
- Annulation des plans de migration
- Retour au support normal

**Étape 6 — Traçabilité**

L'historique conserve la trace complète :
- Période de dépréciation temporaire enregistrée
- Raisons de la réactivation documentées
- INV-EB-2 respecté (traçabilité immuable)

### Exemple concret

> **Contexte :** `KindMother - StorageAdapter v3.2` était DEPRECATED en faveur de v4.0. Cependant, v4.0 présente des problèmes de performance critiques en production et doit être abandonnée.
>
> **Processus de réactivation :**
> 1. Constat : v4.0 ne peut pas être déployée (performance inacceptable)
> 2. Vérification : v3.2 toujours fonctionnelle et stable
> 3. Décision documentée : "v4.0 abandonnée pour problèmes de performance, réactivation de v3.2"
> 4. v3.2 : DEPRECATED → ACTIVE
> 5. Consommateurs informés : "Migration vers v4.0 annulée, v3.2 reste la version recommandée"
> 6. Historique conserve : période de dépréciation du 2026-01-15 au 2026-01-27

### Invariants respectés

| Invariant | Respect |
|-----------|---------|
| **INV-EB-2** | ✅ Traçabilité complète de la période de dépréciation |
| **INV-EB-3** | ✅ État non ambigu — retour clair à ACTIVE |
| **INV-EB-7** | ✅ Documentation complète de la réactivation |

### Conditions de validité

La transition DEPRECATED → ACTIVE est autorisée **uniquement si** :

| Condition | Obligatoire |
|-----------|-------------|
| Le successeur est annulé | ✅ Oui |
| L'élément déprécié est encore fonctionnel | ✅ Oui |
| La justification est documentée | ✅ Oui |
| L'élément n'a pas atteint RETIRED | ✅ Oui |

---

## Scénario 5 : Dette structurelle excessive

### Description

Le **debt ratio** (rapport entre éléments DEPRECATED/RETIRED et éléments ACTIVE) dépasse le seuil acceptable. Ce scénario déclenche un plan de nettoyage gouverné.

### Conditions préalables

| Condition | Description |
|-----------|-------------|
| Déclencheur | Debt ratio > seuil défini |
| Mesure | (DEPRECATED + RETIRED) / ACTIVE |
| Seuil recommandé | Variable selon la catégorie d'éléments |
| Action | Plan de nettoyage obligatoire |

### Étapes du scénario

```
┌─────────────────────────────────────────────────────────────────┐
│             SCÉNARIO 5 : DETTE STRUCTURELLE EXCESSIVE           │
├─────────────────────────────────────────────────────────────────┤
│  1. Ever Buddy détecte le dépassement de seuil                  │
│     → Calcul du debt ratio                                      │
│                         │                                       │
│                         ▼                                       │
│  2. Alerte émise vers les consommateurs concernés               │
│     → Gravité et urgence évaluées                               │
│                         │                                       │
│                         ▼                                       │
│  3. Ever Buddy recommande un plan de nettoyage                  │
│     → Priorisation des éléments à archiver                      │
│                         │                                       │
│                         ▼                                       │
│  4. Éléments RETIRED les plus anciens → candidats ARCHIVED      │
│     → Vérification des dépendances résiduelles                  │
│                         │                                       │
│                         ▼                                       │
│  5. Plan exécuté progressivement                                │
│     → Transitions RETIRED → ARCHIVED                            │
│                         │                                       │
│                         ▼                                       │
│  6. Debt ratio revient sous le seuil                            │
│     → Clôture de l'alerte                                       │
└─────────────────────────────────────────────────────────────────┘
```

**Étape 1 — Détection du dépassement**

Ever Buddy surveille en permanence le debt ratio :
- Calcul périodique du ratio
- Comparaison avec les seuils définis
- Détection du dépassement

**Étape 2 — Émission d'alerte**

Une alerte est émise vers les consommateurs concernés :
- Identification des consommateurs impactés
- Évaluation de la gravité (léger, modéré, critique)
- Communication de l'urgence

**Étape 3 — Plan de nettoyage**

Ever Buddy recommande un plan de nettoyage :
- Inventaire des éléments DEPRECATED et RETIRED
- Priorisation par âge et impact
- Identification des candidats à l'archivage
- Vérification des dépendances résiduelles

**Étape 4 — Sélection des candidats**

Les éléments RETIRED les plus anciens sont candidats à l'archivage :
- Vérification qu'aucun consommateur actif ne dépend de l'élément
- Confirmation que la période de grâce est écoulée
- Préparation de la transition

**Étape 5 — Exécution progressive**

Le plan est exécuté progressivement :
- Transitions RETIRED → ARCHIVED par lots
- Vérification après chaque lot
- Documentation des archivages

**Étape 6 — Retour sous le seuil**

Le debt ratio revient sous le seuil acceptable :
- Calcul du nouveau ratio
- Confirmation du retour à la normale
- Clôture de l'alerte

### Exemple concret

> **Contexte :** Le registre des contrats de BondingBrother présente un debt ratio de 0.35 (35% d'éléments DEPRECATED ou RETIRED) alors que le seuil est de 0.25.
>
> **Plan de nettoyage :**
> 1. Détection : debt ratio = 0.35 > 0.25
> 2. Alerte : "Dette structurelle excessive dans BondingBrother contracts"
> 3. Plan recommandé :
>    - 5 contrats RETIRED depuis > 6 mois → candidats ARCHIVED
>    - 2 contrats DEPRECATED avec 100% adoption du successeur → candidats RETIRED
> 4. Vérification : aucun consommateur actif pour les 5 contrats RETIRED
> 5. Exécution :
>    - Lot 1 : 3 contrats RETIRED → ARCHIVED
>    - Lot 2 : 2 contrats RETIRED → ARCHIVED
>    - Lot 3 : 2 contrats DEPRECATED → RETIRED
> 6. Résultat : debt ratio = 0.20 < 0.25, alerte clôturée

### Seuils recommandés

| Catégorie d'éléments | Seuil d'alerte | Seuil critique |
|---------------------|----------------|----------------|
| Contrats fondateurs (FONDATION) | 0.10 | 0.20 |
| Contrats opérationnels | 0.25 | 0.40 |
| Interfaces techniques | 0.35 | 0.50 |
| Éléments internes | 0.50 | 0.70 |

### Invariants respectés

| Invariant | Respect |
|-----------|---------|
| **INV-EB-2** | ✅ Traçabilité de toutes les transitions d'archivage |
| **INV-EB-4** | ✅ Passage obligatoire par DEPRECATED avant RETIRED |
| **INV-EB-7** | ✅ Documentation du plan de nettoyage |

---

## Comparaison des scénarios

### Tableau récapitulatif

| Aspect | Scénario 1 | Scénario 2 | Scénario 3 | Scénario 4 | Scénario 5 |
|--------|------------|------------|------------|------------|------------|
| **Déclencheur** | Amélioration | Breaking change | Obsolescence | Échec successeur | Debt ratio |
| **Successeur** | Nouvelle version | Nouvelle version | Aucun | Réactivation | N/A |
| **DEPRECATED** | Non | Oui | Oui | Annulé | Oui/Non |
| **Impact consommateurs** | Minimal | Migration requise | Retrait dépendance | Annulation migration | Variable |
| **Fréquence** | Très fréquent | Occasionnel | Rare | Exceptionnel | Variable |
| **Complexité** | Faible | Élevée | Moyenne | Élevée | Moyenne |

### Diagramme des transitions par scénario

```
                            ┌──────────────────────────────────────────────────┐
                            │              ÉTATS DE CYCLE DE VIE                │
                            ├──────────────────────────────────────────────────┤
                            │                                                  │
    Scénario 1:             │  DRAFT ──────► ACTIVE ◄────┐                     │
    (mineure)               │           │                │                     │
                            │           │                │                     │
    Scénario 2:             │           │                │  Scénario 4         │
    (majeure)               │           ▼                │  (réactivation)     │
                            │      DEPRECATED ───────────┘                     │
    Scénario 3:             │           │                                      │
    (abandon)               │           │                                      │
                            │           ▼                                      │
    Scénario 5:             │       RETIRED                                    │
    (dette)                 │           │                                      │
                            │           │                                      │
                            │           ▼                                      │
                            │      ARCHIVED                                    │
                            │                                                  │
                            └──────────────────────────────────────────────────┘
```

---

## Bonnes pratiques

### Pour les producteurs d'éléments

| Pratique | Recommandation |
|----------|----------------|
| **Conception** | Favoriser les évolutions mineures rétrocompatibles |
| **Documentation** | Documenter chaque changement, même mineur |
| **Anticipation** | Planifier les évolutions majeures à l'avance |
| **Communication** | Annoncer les dépréciations le plus tôt possible |
| **Migration** | Fournir des guides de migration clairs |

### Pour les consommateurs d'éléments

| Pratique | Recommandation |
|----------|----------------|
| **Surveillance** | Surveiller les annonces de dépréciation |
| **Réactivité** | Migrer pendant la période de dépréciation |
| **Tests** | Tester la compatibilité avec les nouvelles versions |
| **Feedback** | Signaler les problèmes de migration |
| **Planification** | Intégrer les migrations dans les cycles de release |

---

## Références croisées

### Documents liés

| Document | Relation |
|----------|----------|
| [Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) | Source des scénarios (Section 10) |
| [Lifecycle States Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) | Définition des états DRAFT, ACTIVE, etc. |
| [Transition Rules Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) | Matrice des transitions valides |
| [Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md) | Surveillance de la dette structurelle |
| [Invariants & Guarantees](../contracts/governance/Ever%20Buddy%20-%20Invariants%20&%20Guarantees.md) | Invariants INV-EB-1 à INV-EB-12 |

### Glossaire

| Terme | Définition | Référence |
|-------|------------|-----------|
| **ACTIVE** | État d'un élément en usage normal | [Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#actif-active--état-de-vie) |
| **DEPRECATED** | État d'un élément dont l'usage est découragé | [Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#déprécié-deprecated--état-de-vie) |
| **DRAFT** | État d'un élément en cours de définition | [Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#brouillon-draft--état-de-vie) |
| **RETIRED** | État d'un élément retiré du système | [Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#retiré-retired--état-de-vie) |
| **Ever Buddy** | Core de cycle de vie et d'évolution | [Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#ever-buddy) |
| **Debt ratio** | Rapport (DEPRECATED + RETIRED) / ACTIVE | [Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) — Section 9 |
| **Breaking change** | Changement qui rompt la compatibilité | [Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) — Section 9 |
| **Successeur** | Élément qui remplace un élément déprécié | [Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) — Section 9 |

---

**Date de création :** 2026-01-27  
**Version :** 1.0  
**Statut :** Document de référence  
**Source :** [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) — Section 10
