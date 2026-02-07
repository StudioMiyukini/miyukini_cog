# WorrySentinel - Architecture & Flows

## 1. Contexte

Ce document decrit l'architecture conceptuelle de WorrySentinel, son positionnement unique en tant que core de gouvernance transversale, et les flux de gouvernance qu'il orchestre. Il complete la [Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) en detaillant **comment** WorrySentinel est structure architecturalement et **comment** les flux de gouvernance circulent, sans jamais remettre en question **pourquoi** il existe ou **ce qu'il fait**.

Cette architecture respecte les principes fondamentaux de Miyukini Core System, notamment la separation stricte entre gouvernance et implementation, et le caractere transversal de WorrySentinel.

## 2. Portee / Scope

Ce document couvre :
- Le positionnement de WorrySentinel dans la pyramide Miyukini (Strate 4)
- La nature unique de WorrySentinel en tant que pression verticale
- Les deux axes de gouvernance (niveaux de securite et etats de confiance)
- Les flux de gouvernance descendant et montant
- Les interfaces conceptuelles avec les autres cores
- Les invariants architecturaux
- Les points d'extension et de non-extension

Ce document **ne couvre pas** :
- Les regles specifiques de chaque niveau de securite (voir [Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md))
- Les transitions entre etats de confiance (voir [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md))
- Les strategies de degradation progressive (voir [Progressive Degradation Contract](../contracts/degradation/WorrySentinel%20-%20Progressive%20Degradation%20Contract.md))
- Les contrats d'integration specifiques (voir dossier contracts/integration/)

---

## 3. Positionnement dans la pyramide Miyukini

WorrySentinel est positionne en **Strate 4 — Gouvernance de Securite**, entre le Kernel (infrastructure technique) et les Cores fonctionnels. Cette position est unique : WorrySentinel n'est pas un core fonctionnel, mais un **core de gouvernance transversale**.

```
┌──────────────────────────────────────────────────┐
│ STRATE 9 — MiyukiniAdmin (EXCEPTION)              │
│ Operateur Souverain d'administration              │
└──────────────────────────────────────────────────┘
          ▲
          │ (hors pyramide)
          │
┌──────────────────────────────────────────────────┐
│ STRATE 5 — Cores fonctionnels                     │
│ StrongFather · KindMother · MasterButler          │
│ CaringNanny · EverBuddy · BorderGuard · TAMR      │
└──────────────────────────────────────────────────┘
          ▲
          │ gouvernes par
          │
┌──────────────────────────────────────────────────┐
│ STRATE 4 — 🛡️ WorrySentinel                        │ ◄── Gouvernance transversale
│ Gouvernance de securite                           │
│ Niveaux de securite (0-4), Etats de confiance (T0-T4) │
│ Degradation progressive                           │
└──────────────────────────────────────────────────┘
          ▲
          │ observe
          │
┌──────────────────────────────────────────────────┐
│ STRATE 3 — Gouvernance Ressources                 │
│ LogisticsSteward                                  │
└──────────────────────────────────────────────────┘
          ▲
┌──────────────────────────────────────────────────┐
│ STRATE 2 — Kernel Miyukini                        │
│ Identite, Horloge, Logger, Sondes                 │
└──────────────────────────────────────────────────┘
```

**Regle architecturale fondamentale :** WorrySentinel n'est pas une brique horizontale — c'est une **pression verticale**. Il ne remplace jamais un core, il contraint tous les cores selon les niveaux de securite et les etats de confiance.

---

## 4. Nature unique de WorrySentinel

### 4.1 Ce que WorrySentinel est

WorrySentinel est un **gouvernant conceptuel** qui :

| Caracteristique | Description |
|-----------------|-------------|
| **Transversal** | Traverse toutes les couches, n'appartient a aucune |
| **Non fonctionnel** | Ne possede aucune logique metier |
| **Pression verticale** | Contraint le comportement de tous les cores |
| **Observateur** | Observe et correle les signaux du systeme |
| **Declarant** | Declare l'etat global du systeme |

### 4.2 Ce que WorrySentinel n'est pas

| Anti-pattern | Explication |
|--------------|-------------|
| ❌ Un core fonctionnel | Il ne traite pas de requetes metier |
| ❌ Un executeur | Il ne realise aucune action |
| ❌ Un implementeur | Il ne code aucun controle de securite |
| ❌ Un persisteur | Il ne stocke aucune donnee |
| ❌ Un decideur specifique | Il ne prend pas de decisions operationnelles |

### 4.3 Distinction fondamentale

```
┌─────────────────────────────────────────────────────────┐
│               CORES FONCTIONNELS                         │
│  StrongFather, KindMother, MasterButler, BorderGuard... │
│  ─────────────────────────────────────────────────────  │
│  • Traitent des requetes                                 │
│  • Prennent des decisions                                │
│  • Executent des operations                              │
│  • Ont des responsabilites definies                      │
└─────────────────────────────────────────────────────────┘
                         │
                         ▼ gouvernes par
┌─────────────────────────────────────────────────────────┐
│               WORRYSENTINEL                              │
│  ─────────────────────────────────────────────────────  │
│  • Gouverne les niveaux                                  │
│  • Declare les etats                                     │
│  • Contraint les comportements                           │
│  • Observe les signaux                                   │
│  • N'execute JAMAIS                                      │
└─────────────────────────────────────────────────────────┘
```

---

## 5. Les deux axes de gouvernance

WorrySentinel gouverne selon deux axes independants mais interagissant :

### 5.1 Axe 1 : Niveaux de securite (0-4)

**Definition :** Profil de risque des Operateurs et produits.

```
┌─────────────────────────────────────────────────────────┐
│             NIVEAUX DE SECURITE (0-4)                    │
├─────────────────────────────────────────────────────────┤
│ Niveau 0 — PUBLIC                                        │
│ • Donnees publiques                                      │
│ • Aucune contrainte stricte                              │
│ • Performance maximale                                   │
├─────────────────────────────────────────────────────────┤
│ Niveau 1 — STANDARD                                      │
│ • Donnees standard                                       │
│ • Contraintes de base                                    │
│ • Auth simple                                            │
├─────────────────────────────────────────────────────────┤
│ Niveau 2 — SENSITIVE DATA                                │
│ • Donnees sensibles                                      │
│ • Contraintes renforcees                                 │
│ • Auth renforcee + signatures                            │
├─────────────────────────────────────────────────────────┤
│ Niveau 3 — CRITICAL SYSTEM                               │
│ • Donnees critiques                                      │
│ • Contraintes strictes                                   │
│ • Zero-trust + verifications croisees                    │
├─────────────────────────────────────────────────────────┤
│ Niveau 4 — HARDENED / ISOLATED                           │
│ • Securite maximale                                      │
│ • Contraintes maximales                                  │
│ • Controles continus + attestations                      │
└─────────────────────────────────────────────────────────┘
```

**Reference complete :** [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)

### 5.2 Axe 2 : Etats de confiance (T0-T4)

**Definition :** Niveau d'integrite du systeme global.

```
┌─────────────────────────────────────────────────────────┐
│             ETATS DE CONFIANCE (T0-T4)                   │
├─────────────────────────────────────────────────────────┤
│ T0 — NORMAL (🟢 Nominal)                                 │
│ • Systeme sain                                           │
│ • Toutes capacites disponibles                           │
│ • Monitoring standard                                    │
├─────────────────────────────────────────────────────────┤
│ T1 — INSTABLE (🟡 Doute)                                 │
│ • Anomalie detectee                                      │
│ • Log renforce, tracabilite etendue                      │
│ • Aucun blocage                                          │
├─────────────────────────────────────────────────────────┤
│ T2 — DEGRADE (🟠 Suspect)                                │
│ • Incoherence persistante                                │
│ • Certaines capacites desactivees                        │
│ • Monitoring visible                                     │
├─────────────────────────────────────────────────────────┤
│ T3 — RESTREINT (🔴 Critique)                             │
│ • Suspicion forte                                        │
│ • Gel des produits non essentiels                        │
│ • TAMR requis pour override                              │
├─────────────────────────────────────────────────────────┤
│ T4 — BLOQUE (⛔ Compromis)                               │
│ • Integrite rompue                                       │
│ • Plus aucune decision operationnelle                    │
│ • Uniquement diagnostics                                 │
└─────────────────────────────────────────────────────────┘
```

**Reference complete :** [Miyukini Conceptual References - Integrity Degradation System](../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)

### 5.3 Interaction entre les deux axes

Les deux axes sont **independants mais interagissent**. WorrySentinel gouverne cette interaction :

```
                        NIVEAUX DE SECURITE
                    0        1        2        3        4
                ┌────────┬────────┬────────┬────────┬────────┐
         T0     │ Normal │ Normal │ Normal │ Normal │ Normal │
                ├────────┼────────┼────────┼────────┼────────┤
E   T1     │ Doute  │ Doute  │ Doute+ │ Doute+ │ Doute++│
T               ├────────┼────────┼────────┼────────┼────────┤
A   T2     │ Modere │ Modere │ Strict │ Strict │ Strict+│
T               ├────────┼────────┼────────┼────────┼────────┤
S   T3     │ Limite │ Restreint│ Gel   │ Gel+   │ Gel++  │
                ├────────┼────────┼────────┼────────┼────────┤
         T4     │ Bloque │ Bloque │ Bloque │ Bloque │ Bloque │
                └────────┴────────┴────────┴────────┴────────┘

Legende :
• Normal    : Fonctionnement standard
• Doute(+)  : Surveillance renforcee (+ selon niveau)
• Modere    : Restrictions moderees
• Strict(+) : Restrictions strictes
• Limite    : Fonctionnalites limitees
• Restreint : Mode minimal
• Gel(+)    : Gel des operations non essentielles
• Bloque    : Arret operationnel
```

**Regle fondamentale :** Les restrictions sont **cumulatives**. Un produit de niveau de securite eleve (3-4) en etat de confiance degrade (T2+) subit les restrictions maximales.

---

## 6. Flux de gouvernance

WorrySentinel opere selon deux flux complementaires et opposes.

### 6.1 Flux descendant : Pression de gouvernance

WorrySentinel impose des contraintes verticales sur tous les cores fonctionnels. Ce flux est **unidirectionnel et non negociable**.

```
                    ┌─────────────────────────┐
                    │     WorrySentinel       │
                    │  Niveau securite : N    │
                    │  Etat confiance : Tx    │
                    └───────────┬─────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│  StrongFather │      │  MasterButler │      │  BorderGuard  │
│               │      │               │      │               │
│ Severite des  │      │ Permissions   │      │ Durcissement  │
│ decisions     │      │ actives       │      │ I/O           │
└───────────────┘      └───────────────┘      └───────────────┘
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│  CaringNanny  │      │   KindMother  │      │ LogisticsSteward│
│               │      │               │      │               │
│ Intensite     │      │ Restrictions  │      │ Durcissement  │
│ monitoring    │      │ acces donnees │      │ quotas        │
└───────────────┘      └───────────────┘      └───────────────┘
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│     TAMR      │      │    Kernel     │      │   EverBuddy   │
│               │      │               │      │               │
│ Droits        │      │ Frequence     │      │ Restrictions  │
│ intervention  │      │ sondes        │      │ contextuelles │
│ humaine       │      │               │      │               │
└───────────────┘      └───────────────┘      └───────────────┘
```

**Tableau des contraintes imposees :**

| Core | Contrainte imposee par WorrySentinel |
|------|--------------------------------------|
| **StrongFather** | Severite des decisions (plus stricte en T2+, niveau 3+) |
| **MasterButler** | Permissions actives (reduites en T2+) |
| **BorderGuard** | Durcissement des frontieres I/O |
| **CaringNanny** | Intensite du monitoring (plus frequent en T1+) |
| **KindMother** | Restrictions d'acces aux donnees sensibles |
| **LogisticsSteward** | Durcissement des quotas et priorites |
| **TAMR** | Droits d'intervention humaine (requis en T3+) |
| **Kernel** | Frequence des sondes d'integrite |
| **EverBuddy** | Restrictions sur l'apprentissage contextuel |

**Principe :** WorrySentinel ne remplace jamais un core. Il contraint le comportement de chaque core selon les niveaux de securite et les etats de confiance gouvernes.

### 6.2 Flux montant : Observation et correlation

WorrySentinel observe et correle les signaux remontant des cores pour determiner l'etat global du systeme. Ce flux est **passif et non intrusif**.

```
                    ┌─────────────────────────┐
                    │     WorrySentinel       │
                    │                         │
                    │  Observe, correle,      │
                    │  declare un etat        │
                    └───────────┬─────────────┘
                                ▲
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        │                       │                       │
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│    Kernel     │      │ StrongFather  │      │ BorderGuard   │
│               │      │               │      │               │
│ • signaux     │      │ • decisions   │      │ • anomalies   │
│   clock       │      │   refusees    │      │   I/O         │
│ • signaux id  │      │ • patterns    │      │ • violations  │
│ • traces      │      │   suspects    │      │   frontieres  │
└───────────────┘      └───────────────┘      └───────────────┘
        ▲                       ▲                       ▲
        │                       │                       │
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│ CaringNanny   │      │  KindMother   │      │LogisticsSteward│
│               │      │               │      │               │
│ • signaux     │      │ • incoherences│      │ • derives     │
│   consolides  │      │   detectees   │      │   allocation  │
│ • anomalies   │      │ • corruptions │      │ • patterns    │
│   monitoring  │      │   donnees     │      │   anormaux    │
└───────────────┘      └───────────────┘      └───────────────┘
        ▲                       ▲                       ▲
        │                       │                       │
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│BondingBrother │      │  MasterButler │      │   EverBuddy   │
│               │      │               │      │               │
│ • comportements│     │ • tentatives  │      │ • derives     │
│   produits    │      │   acces       │      │   contextuelles│
│ • anomalies   │      │   non autorises│     │ • anomalies   │
│   liaison     │      │               │      │   apprentissage│
└───────────────┘      └───────────────┘      └───────────────┘
```

**Types de signaux observes :**

| Source | Signaux observes |
|--------|------------------|
| **Kernel** | Signaux clock, signaux identite, traces d'execution |
| **StrongFather** | Decisions refusees, patterns de decisions suspects |
| **BorderGuard** | Anomalies I/O, violations de frontieres |
| **CaringNanny** | Signaux consolides, anomalies de monitoring |
| **KindMother** | Incoherences detectees, corruptions de donnees |
| **LogisticsSteward** | Derives d'allocation, patterns de consommation anormaux |
| **BondingBrother** | Comportements produits anormaux, anomalies de liaison |
| **MasterButler** | Tentatives d'acces non autorises |
| **EverBuddy** | Derives contextuelles, anomalies d'apprentissage |

**Principe :** WorrySentinel observe, correle les signaux, et declare un etat global. Il ne prend jamais de decision operationnelle basee sur ces signaux — cette responsabilite appartient aux cores fonctionnels.

### 6.3 Cycle de gouvernance complet

```
┌──────────────────────────────────────────────────────────────────────┐
│                     CYCLE DE GOUVERNANCE WORRYSENTINEL                │
└──────────────────────────────────────────────────────────────────────┘

1. OBSERVATION
   │
   │  Cores → WorrySentinel
   │  • Signaux d'integrite
   │  • Anomalies detectees
   │  • Decisions refusees
   ▼
┌─────────────────┐
│   CORRELATION   │ ◄─── WorrySentinel correle les signaux multiples
└────────┬────────┘
         │
         ▼
2. EVALUATION
   │
   │  WorrySentinel evalue :
   │  • Coherence des signaux
   │  • Persistance des anomalies
   │  • Correlation inter-cores
   ▼
┌─────────────────┐
│   DECLARATION   │ ◄─── WorrySentinel declare l'etat global (T0-T4)
└────────┬────────┘
         │
         ▼
3. GOUVERNANCE
   │
   │  WorrySentinel → Cores
   │  • Contraintes selon etat de confiance
   │  • Contraintes selon niveau de securite
   │  • Regles de degradation
   ▼
┌─────────────────┐
│   ADAPTATION    │ ◄─── Chaque core adapte son comportement
└────────┬────────┘
         │
         ▼
4. TRACABILITE
   │
   │  • Etat declare journalise
   │  • Contraintes imposees tracees
   │  • Signaux correles archives
   │
   └──────────► Retour a 1. OBSERVATION
```

---

## 7. Architecture interne conceptuelle

WorrySentinel n'est pas structure en couches comme un core fonctionnel. Il est structure en **domaines de gouvernance**.

```
┌──────────────────────────────────────────────────────────────────────┐
│                         WORRYSENTINEL                                 │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │             DOMAINE : GOUVERNANCE DES NIVEAUX                   │  │
│  │  • Definition des niveaux de securite (0-4)                     │  │
│  │  • Attribution des niveaux aux produits                         │  │
│  │  • Regles d'adaptation comportementale par niveau               │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                              │                                        │
│                              ▼                                        │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │             DOMAINE : GOUVERNANCE DES ETATS                     │  │
│  │  • Definition des etats de confiance (T0-T4)                    │  │
│  │  • Regles de transition entre etats                             │  │
│  │  • Declaration de l'etat global                                 │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                              │                                        │
│                              ▼                                        │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │             DOMAINE : DEGRADATION PROGRESSIVE                   │  │
│  │  • Regles de degradation par niveau de confiance                │  │
│  │  • Interaction niveaux securite × etats confiance               │  │
│  │  • Orchestration de la degradation                              │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                              │                                        │
│                              ▼                                        │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │             DOMAINE : OBSERVATION ET CORRELATION                │  │
│  │  • Reception des signaux des cores                              │  │
│  │  • Correlation des signaux multiples                            │  │
│  │  • Alimentation des domaines superieurs                         │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                              │                                        │
│                              ▼                                        │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │             DOMAINE : TRACABILITE                               │  │
│  │  • Journalisation des etats declares                            │  │
│  │  • Journalisation des contraintes imposees                      │  │
│  │  • Archivage des signaux correles                               │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

**Important :** Ces domaines sont **conceptuels**, pas des composants techniques. WorrySentinel ne possede pas de logique d'execution — il definit des regles de gouvernance que les autres cores appliquent.

---

## 8. Interfaces conceptuelles

WorrySentinel expose des **interfaces conceptuelles** (pas des APIs techniques) aux autres cores.

### 8.1 Interfaces de consultation

| Interface | Description | Consommateurs |
|-----------|-------------|---------------|
| `ISecurityLevelQuery` | Interrogation du niveau de securite d'une entite | Tous les cores |
| `ITrustStateQuery` | Interrogation de l'etat de confiance global | Tous les cores |
| `IConstraintQuery` | Interrogation des contraintes applicables | Tous les cores |
| `IDegradationQuery` | Interrogation du niveau de degradation | Tous les cores |

### 8.2 Interfaces de signalement

| Interface | Description | Producteurs |
|-----------|-------------|-------------|
| `IIntegritySignal` | Signalement de signal d'integrite | Kernel, CaringNanny |
| `IAnomalySignal` | Signalement d'anomalie | Tous les cores |
| `IDecisionSignal` | Signalement de decision refusee | StrongFather |
| `IBoundarySignal` | Signalement de violation de frontiere | BorderGuard |
| `IAllocationSignal` | Signalement de derive d'allocation | LogisticsSteward |

### 8.3 Interfaces de gouvernance

| Interface | Description | Direction |
|-----------|-------------|-----------|
| `IConstraintImposition` | Imposition de contraintes aux cores | WS → Cores |
| `IDegradationOrchestration` | Orchestration de la degradation | WS → Cores |
| `IAdaptationRequirement` | Exigence d'adaptation comportementale | WS → Cores |

---

## 9. Relations detaillees avec les autres cores

### 9.1 Relation avec StrongFather

```
WorrySentinel                          StrongFather
     │                                       │
     │ ─── niveau securite (N) ───────────→ │ (ajuste severite)
     │                                       │
     │ ─── etat confiance (Tx) ────────────→ │ (ajuste politique)
     │                                       │
     │ ←── decisions refusees ───────────── │ (signaux)
     │                                       │
     │ ←── patterns suspects ─────────────── │ (signaux)
```

**Nature :** WorrySentinel gouverne la severite de StrongFather sans jamais prendre de decision a sa place. StrongFather applique les politiques selon les contraintes de WorrySentinel.

### 9.2 Relation avec CaringNanny

```
WorrySentinel                          CaringNanny
     │                                       │
     │ ─── intensite monitoring ──────────→ │ (ajuste frequence)
     │                                       │
     │ ←── signaux consolides ───────────── │ (alimente correlation)
     │                                       │
     │ ←── anomalies monitoring ─────────── │ (signaux)
     │                                       │
     │ ←── propositions transition etat ─── │ (suggestions)
```

**Nature :** CaringNanny consolide les signaux et propose des transitions d'etat. WorrySentinel correle et declare l'etat final.

### 9.3 Relation avec BorderGuard

```
WorrySentinel                          BorderGuard
     │                                       │
     │ ─── durcissement frontieres ───────→ │ (ajuste I/O)
     │                                       │
     │ ←── anomalies I/O ────────────────── │ (signaux)
     │                                       │
     │ ←── violations frontieres ─────────── │ (signaux)
```

**Nature :** WorrySentinel impose le durcissement des frontieres selon l'etat de confiance. BorderGuard signale les anomalies qui alimentent la correlation.

### 9.4 Relation avec LogisticsSteward

```
WorrySentinel                          LogisticsSteward
     │                                       │
     │ ─── durcissement quotas ───────────→ │ (ajuste regles)
     │                                       │
     │ ─── contraintes securitaires ──────→ │ (impose limites)
     │                                       │
     │ ←── derives allocation ───────────── │ (signaux)
     │                                       │
     │ ←── patterns consommation ─────────── │ (signaux)
```

**Nature :** WorrySentinel supervise LogisticsSteward et peut imposer un durcissement des regles d'arbitrage en etat T1+. LogisticsSteward reste souverain sur l'arbitrage mais doit adapter ses decisions selon les contraintes securitaires.

### 9.5 Relation avec TAMR

```
WorrySentinel                          TAMR
     │                                       │
     │ ─── droits intervention ───────────→ │ (ajuste capacites)
     │                                       │
     │ ─── exigence override humain ──────→ │ (en T3+)
     │                                       │
     │ ←── interventions effectuees ─────── │ (tracabilite)
```

**Nature :** WorrySentinel gouverne les droits d'intervention humaine. En T3+, l'intervention TAMR est requise pour tout override.

### 9.6 Relation avec MiyukiniAdmin

```
WorrySentinel                          MiyukiniAdmin
     │                                       │
     │ ←── consultation etat ─────────────── │ (lecture)
     │                                       │
     │ ←── configuration gouvernance ─────── │ (via StrongFather)
     │                                       │
     │ ─── etat global visible ───────────→ │ (dashboard)
```

**Nature :** MiyukiniAdmin consulte WorrySentinel pour afficher l'etat de securite. Toute configuration passe par StrongFather pour validation.

---

## 10. Invariants architecturaux

Ces invariants sont **non negociables** et definissent les frontieres absolues de WorrySentinel.

| Code | Invariant | Description |
|------|-----------|-------------|
| **ARCH-WS-1** | Aucune execution | WorrySentinel ne realise jamais d'action technique |
| **ARCH-WS-2** | Aucune decision operationnelle | WorrySentinel ne prend jamais de decision metier |
| **ARCH-WS-3** | Aucune persistance | WorrySentinel ne stocke aucune donnee operationnelle |
| **ARCH-WS-4** | Aucune modification d'etat | WorrySentinel ne modifie jamais l'etat du systeme |
| **ARCH-WS-5** | Pression uniquement | WorrySentinel contraint, ne remplace jamais |
| **ARCH-WS-6** | Transversalite | WorrySentinel traverse toutes les couches |
| **ARCH-WS-7** | Tracabilite complete | Toute gouvernance est tracee |
| **ARCH-WS-8** | Zero-trust | WorrySentinel ne fait confiance a aucun appelant |
| **ARCH-WS-9** | Gouvernance explicite | Toutes les regles sont declaratives |
| **ARCH-WS-10** | Independance des axes | Niveaux de securite et etats de confiance sont independants |

---

## 11. Comportement en mode degrade

WorrySentinel lui-meme fonctionne meme en environnement degrade.

### 11.1 Signaux non disponibles

Si les signaux des cores ne sont pas disponibles :
- WorrySentinel **ne peut pas** ameliorer l'etat de confiance
- WorrySentinel **peut** maintenir ou degrader l'etat
- Absence de signaux = suspicion = T1 minimum

### 11.2 Cores indisponibles

Si un core ne repond pas aux contraintes :
- Le core est considere comme **non conforme**
- Le niveau de confiance global est **impacte**
- L'anomalie est **tracee**

### 11.3 Mode autonome

En mode completement isole :
- WorrySentinel fonctionne avec les regles locales
- L'etat de confiance est gere localement
- La reconciliation intervient a la reconnexion

---

## 12. Points d'extension et non-extension

### 12.1 Points d'extension

WorrySentinel peut etre etendu **uniquement** aux points suivants :

| Point d'extension | Type | Contrainte |
|-------------------|------|------------|
| Nouveaux signaux d'integrite | Addition | Doivent suivre les interfaces definies |
| Nouvelles regles de correlation | Addition | Doivent etre explicites et declaratives |
| Nouveaux types de contraintes | Addition | Doivent respecter la nature de gouvernance |
| Nouvelles metriques d'observation | Addition | Ne doivent pas impacter la gouvernance |

### 12.2 Points non extensibles

Ces elements sont **figes** et non extensibles :

| Element | Raison |
|---------|--------|
| Nombre de niveaux de securite (0-4) | Echelle fixee par design |
| Nombre d'etats de confiance (T0-T4) | Echelle fixee par design |
| Nature transversale de WorrySentinel | Positionnement architectural |
| Separation gouvernance/implementation | Invariant fondateur |
| Flux descendant (pression) | Principe architectural |
| Flux montant (observation) | Principe architectural |

---

## 13. Phrase fondatrice architecturale

> **WorrySentinel est une pression verticale, pas une brique horizontale. Il gouverne les niveaux de securite et les etats de confiance de l'ecosysteme entier, observe et correle les signaux de tous les cores, et impose des contraintes adaptatives — sans jamais executer, decider operationnellement, ou persister.**

Cette phrase resume l'architecture : pression verticale (transversal), gouvernance des niveaux et etats (les deux axes), observation et correlation (flux montant), contraintes adaptatives (flux descendant), et les interdits absolus (execution, decision, persistance).

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut ARCHITECTURE**. Il etablit la structure conceptuelle de WorrySentinel et les flux de gouvernance qui ne peuvent etre modifies sans processus formel de versionnement.

Toute implementation de WorrySentinel doit respecter cette architecture. Toute extension doit utiliser les points d'extension definis. Toute modification structurelle necessite une nouvelle version de ce document.

---

## 15. Documents associes

- [WorrySentinel - Index de Navigation](../_index.md)
- [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)
- [WorrySentinel - Core Interaction Contract](./WorrySentinel%20-%20Core%20Interaction%20Contract.md)
- [WorrySentinel - Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)
- [WorrySentinel - Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)
- [WorrySentinel - Progressive Degradation Contract](../contracts/degradation/WorrySentinel%20-%20Progressive%20Degradation%20Contract.md)
- [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- [Miyukini Conceptual References - Integrity Degradation System](../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** ARCHITECTURE — Normatif  
**Dependance :** [Documentation Fondatrice v1.2](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)  
**Reference :** Miyukini Core System v2.4
