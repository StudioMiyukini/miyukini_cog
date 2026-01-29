# LogisticsSteward - Architecture & Flows

## 1. Contexte

Ce document decrit l'architecture conceptuelle interne de LogisticsSteward, ses composants structurels, et les flux d'arbitrage qui gouvernent l'allocation des ressources. Il complete la [Documentation Fondatrice](../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) en detaillant **comment** LogisticsSteward est structure et **comment** les decisions d'arbitrage circulent, sans jamais remettre en question **pourquoi** il existe ou **ce qu'il fait**.

Cette architecture respecte les [Lois d'Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md), notamment **LOI-1** (aucune dependance externe critique), **LOI-2** (isolement comme etat normal), et **LOI-5** (cout proportionnel au hardware).

## 2. Portee / Scope

Ce document couvre :
- Le positionnement de LogisticsSteward dans la pyramide Miyukini
- La structure en couches conceptuelles
- Les composants internes et leurs responsabilites
- Les flux d'arbitrage detailles (entree, evaluation, sortie)
- Les interfaces entre composants
- Les invariants architecturaux
- Les points d'extension et de non-extension

Ce document **ne couvre pas** :
- Les regles metier specifiques de quotas (voir [Quota Definition Contract](../contracts/resources/LogisticsSteward%20-%20Quota%20Definition%20Contract.md))
- Les strategies de degradation detaillees (voir [Degradation Strategy Contract](../contracts/degradation/LogisticsSteward%20-%20Degradation%20Strategy%20Contract.md))
- Les contrats d'integration specifiques (voir dossier contracts/integration/)
- Les details d'implementation technique (voir [Reference Implementation Guidelines](../implementation/LogisticsSteward%20-%20Reference%20Implementation%20Guidelines.md))

---

## 3. Positionnement dans la pyramide Miyukini

LogisticsSteward est positionne en **Strate 3 — Gouvernance des Ressources**, entre le Kernel (infrastructure technique) et les Cores Systeme (autorites).

```
┌──────────────────────────────────────────┐
│ STRATE 9 — MiyukiniAdmin (EXCEPTION)     │
│ Operateur Souverain d'administration     │
└──────────────────────────────────────────┘
          ▲
          │ (hors pyramide)
          │
┌──────────────────────────────────────────┐
│ STRATE 5 — Liaison                       │
│ BondingBrother                           │
└──────────────────────────────────────────┘
          ▲
┌──────────────────────────────────────────┐
│ STRATE 4 — Cores Systeme                 │
│ StrongFather, KindMother, WorrySentinel  │
└──────────────────────────────────────────┘
          ▲
┌──────────────────────────────────────────┐
│ STRATE 3 — Gouvernance Ressources        │  ◄── LogisticsSteward
│ Arbitrage, Quotas, Priorites             │
└──────────────────────────────────────────┘
          ▲
┌──────────────────────────────────────────┐
│ STRATE 2 — Capacites                     │
│ MasterButler                             │
└──────────────────────────────────────────┘
          ▲
┌──────────────────────────────────────────┐
│ STRATE 1 — Kernel                        │
│ Infrastructure technique                 │
└──────────────────────────────────────────┘
```

**Regles fondamentales de positionnement :**
- LogisticsSteward **consomme** l'etat systeme abstrait du Kernel (lecture seule)
- LogisticsSteward **soumet** ses decisions a StrongFather (validation)
- LogisticsSteward **adapte** ses regles sur alerte de WorrySentinel
- LogisticsSteward **transmet** ses decisions via BondingBrother
- LogisticsSteward **limite l'usage** des capacites exposees par MasterButler

---

## 4. Architecture en couches

LogisticsSteward est organise en **quatre couches conceptuelles**, chacune avec une responsabilite unique et des interfaces claires.

```
┌─────────────────────────────────────────────────────────────┐
│                    COUCHE DECISION                          │
│    (Generation des decisions d'arbitrage, determinisme)     │
├─────────────────────────────────────────────────────────────┤
│                    COUCHE EVALUATION                        │
│    (Application des regles, calcul des priorites)           │
├─────────────────────────────────────────────────────────────┤
│                    COUCHE CONTEXTE                          │
│    (Lecture etat systeme, resolution contexte entite)       │
├─────────────────────────────────────────────────────────────┤
│                    COUCHE RECEPTION                         │
│    (Entree des demandes, validation structurelle)           │
└─────────────────────────────────────────────────────────────┘
```

### 4.1 Couche Reception

**Responsabilite :** Recevoir et valider structurellement les demandes de ressources entrant dans LogisticsSteward.

**Composants conceptuels :**
- **RequestReceiver** : Point d'entree unique pour toutes les demandes de ressources
- **StructuralValidator** : Validation de la structure des demandes (champs requis, types)
- **EntityIdentifier** : Identification de l'entite demandeuse (Operateur, Equipe, Service)
- **RequestLogger** : Journalisation de toute demande recue

**Interfaces exposees :**
- `IResourceRequest` : Soumission d'une demande de ressource
- `IQuotaQuery` : Interrogation du quota actuel d'une entite
- `IPriorityQuery` : Interrogation de la priorite actuelle d'une entite

**Regle architecturale :** Cette couche ne decide jamais. Elle valide la forme, identifie le demandeur, et transmet a la couche suivante.

### 4.2 Couche Contexte

**Responsabilite :** Construire le contexte complet necessaire a l'evaluation d'une demande en assemblant l'etat systeme et les informations de l'entite.

**Composants conceptuels :**
- **SystemStateReader** : Lecture de l'etat systeme abstrait (fourni par le Kernel)
- **EntityContextResolver** : Resolution du contexte de l'entite (quotas attribues, historique)
- **DegradationLevelReader** : Lecture du niveau de degradation actuel du systeme
- **ContextAssembler** : Assemblage du contexte complet pour l'evaluation

**Interfaces internes :**
- `ISystemState` : Contrat d'acces a l'etat systeme abstrait
- `IEntityContext` : Contrat de resolution du contexte entite
- `IDegradationLevel` : Contrat de lecture du niveau de degradation

**Regle architecturale :** Cette couche est en **lecture seule**. Elle ne modifie aucun etat, ne prend aucune decision, ne stocke aucune donnee operationnelle.

### 4.3 Couche Evaluation

**Responsabilite :** Appliquer les regles d'arbitrage au contexte assemble pour determiner si la demande est autorisee et sous quelles conditions.

**Composants conceptuels :**
- **RuleEngine** : Moteur d'evaluation des regles declaratives
- **QuotaEvaluator** : Verification du respect des quotas attribues
- **PriorityCalculator** : Calcul de la priorite effective de la demande
- **ConflictResolver** : Resolution des conflits entre regles contradictoires
- **DegradationApplier** : Application des restrictions liees au niveau de degradation

**Interfaces internes :**
- `IRule` : Contrat de definition d'une regle d'arbitrage
- `IRuleEvaluation` : Contrat d'evaluation d'une regle
- `IConflictResolution` : Contrat de resolution de conflit

**Regle architecturale :** L'evaluation est **deterministe**. Memes entrees = meme resultat d'evaluation. Aucune inference, aucune heuristique, aucun hasard.

### 4.4 Couche Decision

**Responsabilite :** Generer la decision d'arbitrage finale et la preparer pour validation par StrongFather.

**Composants conceptuels :**
- **DecisionGenerator** : Generation de la decision d'arbitrage
- **DecisionFormatter** : Formatage de la decision pour transmission
- **ValidationPreparer** : Preparation de la decision pour soumission a StrongFather
- **DecisionLogger** : Journalisation complete de la decision (tracabilite)

**Interfaces de sortie :**
- `IArbitrationDecision` : Contrat de decision d'arbitrage
- `IDecisionSubmission` : Contrat de soumission a StrongFather
- `IDecisionAudit` : Contrat d'audit des decisions

**Regle architecturale :** Cette couche **propose**, elle ne dispose pas. Toute decision doit etre validee par StrongFather avant application.

---

## 5. Composants transversaux

Ces composants servent plusieurs couches et assurent des fonctions critiques non specifiques a une couche.

### 5.1 RuleRepository

**Responsabilite :** Stocker et fournir l'ensemble des regles d'arbitrage declarees.

**Caracteristiques :**
- Regles explicites et declaratives
- Immutables apres declaration (modification = nouvelle version)
- Auditables (historique complet)
- Versionneees

**Ce qu'il ne fait pas :**
- Ne genere pas de regles implicites
- Ne prend aucune decision
- Ne modifie pas les regles dynamiquement en production

### 5.2 AuditJournal

**Responsabilite :** Journaliser toutes les demandes, evaluations et decisions pour audit et tracabilite.

**Elements journalises :**
- Demande recue (timestamp, entite, ressource demandee)
- Contexte assemble (etat systeme, contexte entite)
- Evaluation (regles appliquees, resultat)
- Decision (autorisee/refusee, conditions, justification)
- Validation StrongFather (acceptee/rejetee)

**Ce qu'il ne fait pas :**
- Ne prend aucune decision basee sur le journal
- Ne modifie pas le comportement de LogisticsSteward
- Ne stocke pas de donnees metier operationnelles

### 5.3 MetricsCollector

**Responsabilite :** Collecter les metriques de fonctionnement sans impacter le flux principal.

**Metriques collectees :**
- Nombre de demandes recues/evaluees/decidees
- Temps de traitement par etape
- Taux d'autorisation/refus par type de ressource
- Distribution des priorites utilisees
- Taux de degradation actif

**Ce qu'il ne fait pas :**
- Ne prend aucune decision basee sur les metriques
- Ne modifie pas les regles d'arbitrage
- Ne stocke pas de donnees metier

### 5.4 HealthMonitor

**Responsabilite :** Surveiller l'etat de sante de LogisticsSteward et de ses connexions.

**Verifications :**
- Disponibilite du Kernel (etat systeme)
- Connectivite vers StrongFather (validation)
- Integrite du RuleRepository
- Capacite du AuditJournal

**Ce qu'il ne fait pas :**
- Ne repare pas automatiquement
- Ne modifie pas le comportement d'arbitrage
- Ne masque pas les problemes

---

## 6. Flux d'arbitrage detailles

### 6.1 Flux principal : Demande de ressource

```
Entite (Operateur / Service / Equipe)
         │
         │ Demande de ressource
         ▼
┌─────────────────────┐
│   RequestReceiver   │ ◄── Reception
└─────────┬───────────┘
          │ Demande validee structurellement
          ▼
┌─────────────────────┐
│  StructuralValidator│ ◄── Validation forme
└─────────┬───────────┘
          │ Demande conforme
          ▼
┌─────────────────────┐
│  EntityIdentifier   │ ◄── Identification demandeur
└─────────┬───────────┘
          │ Entite identifiee
          ▼
┌─────────────────────┐
│   RequestLogger     │ ◄── Journalisation demande
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│  SystemStateReader  │ ◄── Lecture etat systeme (Kernel)
└─────────┬───────────┘
          │ Etat systeme abstrait
          ▼
┌─────────────────────┐
│EntityContextResolver│ ◄── Resolution contexte entite
└─────────┬───────────┘
          │ Contexte entite
          ▼
┌─────────────────────┐
│DegradationLevelReader│ ◄── Niveau degradation actuel
└─────────┬───────────┘
          │ Niveau degradation
          ▼
┌─────────────────────┐
│  ContextAssembler   │ ◄── Assemblage contexte complet
└─────────┬───────────┘
          │ Contexte complet
          ▼
┌─────────────────────┐
│     RuleEngine      │ ◄── Evaluation regles
└─────────┬───────────┘
          │ Regles applicables
          ▼
┌─────────────────────┐
│   QuotaEvaluator    │ ◄── Verification quotas
└─────────┬───────────┘
          │ Quota OK / Depasse
          ▼
┌─────────────────────┐
│ PriorityCalculator  │ ◄── Calcul priorite effective
└─────────┬───────────┘
          │ Priorite calculee
          ▼
┌─────────────────────┐
│  ConflictResolver   │ ◄── Resolution conflits
└─────────┬───────────┘
          │ Conflits resolus
          ▼
┌─────────────────────┐
│ DegradationApplier  │ ◄── Application restrictions degradation
└─────────┬───────────┘
          │ Restrictions appliquees
          ▼
┌─────────────────────┐
│ DecisionGenerator   │ ◄── Generation decision
└─────────┬───────────┘
          │ Decision brute
          ▼
┌─────────────────────┐
│  DecisionFormatter  │ ◄── Formatage decision
└─────────┬───────────┘
          │ Decision formatee
          ▼
┌─────────────────────┐
│  DecisionLogger     │ ◄── Journalisation decision
└─────────┬───────────┘
          │ Decision journalisee
          ▼
┌─────────────────────┐
│ ValidationPreparer  │ ◄── Preparation validation
└─────────┬───────────┘
          │
          ▼
    ┌─────────────┐
    │ StrongFather│ ◄── Validation externe
    └─────────────┘
          │
          │ Validation / Invalidation
          ▼
    ┌─────────────┐
    │   Kernel    │ ◄── Execution (si valide)
    └─────────────┘
```

### 6.2 Flux de consultation : Interrogation quota/priorite

```
Entite
   │
   │ Interrogation (quota ou priorite)
   ▼
┌─────────────────────┐
│   RequestReceiver   │ ◄── Reception
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│  EntityIdentifier   │ ◄── Identification
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│EntityContextResolver│ ◄── Resolution contexte
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│   QuotaEvaluator /  │ ◄── Lecture valeurs actuelles
│ PriorityCalculator  │
└─────────┬───────────┘
          │
          ▼
   Reponse (valeurs actuelles)
```

### 6.3 Flux de degradation : Declenchement par WorrySentinel

```
┌─────────────────────┐
│   WorrySentinel     │ ◄── Detection anomalie
└─────────┬───────────┘
          │ Alerte durcissement
          ▼
┌─────────────────────┐
│LogisticsSteward     │
│(DegradationApplier) │ ◄── Reception alerte
└─────────┬───────────┘
          │ Elevation niveau degradation
          ▼
┌─────────────────────┐
│   RuleEngine        │ ◄── Activation regles de degradation
└─────────┬───────────┘
          │ Nouvelles restrictions actives
          ▼
┌─────────────────────┐
│  DecisionGenerator  │ ◄── Decisions futures appliquent degradation
└─────────────────────┘
```

---

## 7. Types de decisions d'arbitrage

LogisticsSteward produit des decisions d'arbitrage normalisees. Chaque decision contient :

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `decision_id` | Identifiant unique de la decision | ✅ |
| `timestamp` | Horodatage de la decision | ✅ |
| `entity_id` | Identifiant de l'entite concernee | ✅ |
| `resource_type` | Type de ressource demandee | ✅ |
| `decision_type` | Type de decision (voir ci-dessous) | ✅ |
| `priority_applied` | Niveau de priorite applique | ✅ |
| `quota_remaining` | Quota restant apres decision | ✅ |
| `conditions` | Conditions associees a la decision | ❌ |
| `justification` | Raison de la decision | ✅ |
| `degradation_level` | Niveau de degradation au moment de la decision | ✅ |
| `rules_applied` | Liste des regles appliquees | ✅ |

### 7.1 Types de decisions

| Type | Code | Description |
|------|------|-------------|
| **Autorise** | `GRANTED` | Demande accordee sans restriction |
| **Autorise conditionnel** | `GRANTED_CONDITIONAL` | Accordee avec conditions |
| **Refuse quota** | `DENIED_QUOTA` | Refusee pour depassement de quota |
| **Refuse priorite** | `DENIED_PRIORITY` | Refusee car priorite insuffisante |
| **Refuse degradation** | `DENIED_DEGRADATION` | Refusee car systeme en degradation |
| **Differe** | `DEFERRED` | Reportee a un moment ulterieur |
| **Reduit** | `REDUCED` | Accordee partiellement (quantite reduite) |

---

## 8. Invariants architecturaux

Ces invariants sont **non negociables** et s'appliquent a toute implementation.

| Code | Invariant | Description |
|------|-----------|-------------|
| **ARCH-LS-1** | Lecture seule du systeme | LogisticsSteward ne modifie jamais l'etat systeme |
| **ARCH-LS-2** | Flux unidirectionnel | Les donnees circulent toujours de la Reception vers la Decision |
| **ARCH-LS-3** | Isolation des couches | Chaque couche n'accede qu'a ses dependances directes |
| **ARCH-LS-4** | Determinisme absolu | Memes entrees + meme etat = meme decision |
| **ARCH-LS-5** | Tracabilite complete | Toute decision est journalisee avec son contexte complet |
| **ARCH-LS-6** | Validation externe | Aucune decision n'est auto-appliquee |
| **ARCH-LS-7** | Regles explicites | Aucune regle n'est deduite ou implicite |
| **ARCH-LS-8** | Sans etat operationnel | LogisticsSteward ne stocke pas d'etat metier |

---

## 9. Isolation et encapsulation

### 9.1 Principe d'isolation

Chaque couche est **strictement isolee** des autres :
- Une couche ne peut acceder qu'a ses propres composants internes
- Une couche ne communique qu'avec la couche adjacente via interfaces
- Aucun partage d'etat entre couches

**Interdit :**
- Acces direct d'une couche a une couche non adjacente
- Partage de references mutables entre composants
- Dependances circulaires

### 9.2 Frontieres de responsabilite

| Composant | Responsable de | Non responsable de |
|-----------|----------------|-------------------|
| RequestReceiver | Recevoir les demandes | Decider de leur validite metier |
| SystemStateReader | Lire l'etat systeme | Modifier l'etat systeme |
| RuleEngine | Evaluer les regles | Creer des regles |
| QuotaEvaluator | Verifier les quotas | Attribuer les quotas |
| DecisionGenerator | Generer la decision | Appliquer la decision |

---

## 10. Extensibilite

### 10.1 Points d'extension

LogisticsSteward peut etre etendu **uniquement** aux points suivants :

| Point d'extension | Type | Contrainte |
|-------------------|------|------------|
| Nouveaux types de ressources | Addition | Doivent suivre le contrat IResourceRequest |
| Nouvelles regles d'arbitrage | Addition | Doivent etre explicites et declaratives |
| Nouveaux types de decisions | Addition | Doivent suivre le contrat IArbitrationDecision |
| Nouvelles metriques | Addition | Ne doivent pas impacter le flux principal |
| Nouveaux niveaux de degradation | Addition | Doivent etre ordonnes et documentees |

### 10.2 Points non extensibles

Ces elements sont **figes** et non extensibles :

- Structure en 4 couches (Reception → Contexte → Evaluation → Decision)
- Flux de donnees (direction et ordre des etapes)
- Principe de validation externe (StrongFather)
- Separation Kernel/LogisticsSteward
- Determinisme des decisions
- Tracabilite complete

---

## 11. Dependances

### 11.1 Dependances internes (entre composants)

```
RequestReceiver ──────► StructuralValidator
                              │
                              ▼
                       EntityIdentifier
                              │
                              ▼
                       SystemStateReader ◄──── Kernel (externe)
                              │
                              ▼
                    EntityContextResolver
                              │
                              ▼
                       ContextAssembler
                              │
                              ▼
                         RuleEngine ◄──── RuleRepository (config)
                              │
                              ▼
                       QuotaEvaluator
                              │
                              ▼
                      PriorityCalculator
                              │
                              ▼
                      DecisionGenerator
                              │
                              ▼
                     ValidationPreparer ────► StrongFather (externe)
```

### 11.2 Dependances externes (vers l'ecosysteme)

| Dependance | Type | Direction | Criticite |
|------------|------|-----------|-----------|
| Kernel | Lecture etat systeme | LS ← K | Haute |
| StrongFather | Validation decisions | LS → SF | Critique |
| WorrySentinel | Alertes durcissement | WS → LS | Haute |
| BondingBrother | Transport decisions | LS → BB | Haute |
| MasterButler | Catalogue capacites | MB → LS | Moyenne |

### 11.3 Absence de dependances

LogisticsSteward **ne depend pas** :
- D'aucune base de donnees metier
- D'aucun service externe autre que les cores Miyukini
- D'aucune logique metier specifique a un operateur
- D'aucune connexion reseau permanente (conforme a **LOI-1** et **LOI-2**)

---

## 12. Comportement en mode degrade

Conforme a **LOI-2** (l'isolement est un etat normal), LogisticsSteward fonctionne meme en environnement degrade.

### 12.1 Kernel indisponible

Si l'etat systeme n'est pas disponible :
- Utilisation du dernier etat systeme connu (cache local)
- Decisions marquees comme "basees sur etat cache"
- Journalisation de l'anomalie
- Alerte vers WorrySentinel

### 12.2 StrongFather indisponible

Si la validation n'est pas possible :
- Decisions mises en file d'attente
- Aucune decision auto-appliquee
- Buffer local (conforme a LOI-2)
- Reconciliation a la reconnexion

### 12.3 Mode autonome

En mode completement isole :
- Regles locales appliquees
- Decisions tracees localement
- Validation differee
- Pas de blocage du systeme

---

## 13. Relations avec les autres cores

| Core | Relation | Direction | Description |
|------|----------|-----------|-------------|
| **Kernel** | Fournisseur etat | K → LS | Fournit l'etat systeme abstrait certifie |
| **StrongFather** | Validateur | LS ↔ SF | Valide ou invalide les decisions |
| **WorrySentinel** | Surveillant | WS → LS | Declenche durcissement des regles |
| **MasterButler** | Catalogue | MB → LS | Expose les capacites a limiter |
| **BondingBrother** | Transporteur | LS → BB | Transporte les decisions validees |
| **KindMother** | Indirect | — | Aucune relation directe |
| **MiyukiniAdmin** | Demandeur privilegie | MA → LS | Peut demander priorites maximales |

---

## 14. Pourquoi LogisticsSteward est critique

### Sans lui

| Probleme | Consequence |
|----------|-------------|
| Pas de gouvernance des ressources | Chaos, monopolisation |
| Pas de quotas | Entites gourmandes sans limite |
| Pas de priorites | Pas d'arbitrage en cas de contention |
| Pas de degradation controlee | Effondrement brutal |
| Pas de tracabilite | Impossible d'auditer |

### Avec lui

| Benefice | Description |
|----------|-------------|
| Cohabitation stable | Chaque entite a sa part |
| Protection proactive | Degradation avant effondrement |
| Decisions auditables | Tracabilite complete |
| Arbitrage deterministe | Comportement previsible |
| Separation des pouvoirs | LogisticsSteward decide, Kernel execute |

---

## 15. Phrase fondatrice architecturale

> **LogisticsSteward est structure pour garantir que chaque decision d'arbitrage est explicite, deterministe, tracable, et validee externement — sans jamais executer ni controler techniquement.**

Cette phrase resume l'architecture : separation des couches (explicite), moteur de regles (deterministe), journalisation (tracable), validation StrongFather (externe), lecture seule (pas d'execution).

---

## 16. Statut contractuel

Ce document est **contractuel, normatif, et de statut ARCHITECTURE**. Il etablit la structure interne de LogisticsSteward et les flux d'arbitrage qui ne peuvent etre modifies sans processus formel de versionnement.

Toute implementation de LogisticsSteward doit respecter cette architecture. Toute extension doit utiliser les points d'extension definis. Toute modification structurelle necessite une nouvelle version de ce document.

---

## 17. Documents associes

- [LogisticsSteward - Index de Navigation](../_index.md)
- [LogisticsSteward - Documentation Fondatrice](../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md)
- [LogisticsSteward - Core Interaction Contract](./LogisticsSteward%20-%20Core%20Interaction%20Contract.md)
- [LogisticsSteward - Quota Definition Contract](../contracts/resources/LogisticsSteward%20-%20Quota%20Definition%20Contract.md)
- [LogisticsSteward - Priority Management Contract](../contracts/resources/LogisticsSteward%20-%20Priority%20Management%20Contract.md)
- [LogisticsSteward - Degradation Strategy Contract](../contracts/degradation/LogisticsSteward%20-%20Degradation%20Strategy%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** ARCHITECTURE — Normatif  
**Dependance :** [Documentation Fondatrice v1.0.0](../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md)  
**Reference :** Miyukini Core System v2.4
