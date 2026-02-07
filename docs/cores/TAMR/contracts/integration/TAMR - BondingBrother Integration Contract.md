# TAMR — BondingBrother Integration Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **TAMR — BondingBrother Integration Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles d'intégration entre TAMR (Human Interaction Core) et BondingBrother (Strate de Liaison Gouvernée) pour la **médiation des intentions d'intervention humaine**.

Ce contrat précise les points d'interaction, les flux de médiation, les responsabilités respectives, les invariants d'intégration, et les garanties offertes par cette relation architecturale.

### Portée

Ce contrat s'applique à **toute intention d'intervention humaine** transitant dans le système Miyukini et définit de manière absolue :
- la nature de la relation entre TAMR (cadre conceptuel) et BondingBrother (médiateur),
- les points d'interaction formels pour les intentions d'approbation, override, escalade et supervision,
- les flux de médiation autorisés,
- les responsabilités de chaque composant dans l'intégration,
- ce que l'intégration PEUT et NE PEUT JAMAIS faire,
- les invariants systémiques associés.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **[TAMR — Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md)** : Définition fondamentale du rôle de TAMR et relation avec BondingBrother
- **[TAMR — Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)** : Types d'intervention (Approval, Override, Escalation, Supervision)
- **[TAMR — Intervention Points Contract](../intervention/TAMR%20-%20Intervention%20Points%20Contract.md)** : Points et conditions d'intervention
- **[TAMR — Invariants & Guarantees](../governance/TAMR%20-%20Invariants%20%26%20Guarantees.md)** : Invariants INV-TAMR-1 à INV-TAMR-8
- **BondingBrother — Documentation Fondatrice** : Définition fondamentale du rôle de BondingBrother
- **[Miyukini Conceptual References — Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)** : Protocoles de liaison inter-COG
- **[Miyukini Conceptual References — Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Terminologie TAMR
- **[Miyukini Conceptual References — Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)** : Principes de sécurité
- **[Miyukini Conceptual References — Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité LOI-1 à LOI-6
- **[Miyukini Conceptual References — Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)** : Niveaux T0-T4
- **[Miyukini Conceptual References — Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)** : Niveaux 0-4

Il n'introduit aucune contradiction avec le corpus documentaire existant.

---

## 2. Nature de la relation

### 2.1 Positionnement architectural

TAMR et BondingBrother occupent des positions distinctes mais complémentaires dans l'architecture Miyukini :

| Composant | Position | Rôle fondamental |
|-----------|----------|------------------|
| **TAMR** | Core (cadre conceptuel) | Définition des types, limites et règles de l'intervention humaine |
| **BondingBrother** | Strate de Liaison | Médiation, traduction et transmission des intentions d'intervention |

**Relation architecturale :**

```
┌─────────────────────────────────────────────────────────────────┐
│                    FLUX D'INTENTION D'INTERVENTION               │
│                                                                   │
│  [Processus / Produit]                                            │
│        │ Intention d'intervention (approval, override, etc.)     │
│        ▼                                                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                  BONDING BROTHER                             ││
│  │                                                              ││
│  │   • Reçoit l'intention d'intervention (forme TAMR)          ││
│  │   • Valide la conformité au cadre TAMR (type, structure)   ││
│  │   • Traduit et transmet vers StrongFather                    ││
│  │   • Trace la médiation                                      ││
│  └─────────────────────────────────────────────────────────────┘│
│        │                                                          │
│        │ Intention traduite / contexte enrichi                    │
│        ▼                                                          │
│  ┌──────────────┐                                                │
│  │ StrongFather │  (décision : autoriser ou refuser l'intervention)│
│  └──────────────┘                                                │
│                                                                   │
│  TAMR : définit le cadre conceptuel (types, traces, limites)     │
│  TAMR ne transmet rien ; le producteur et BondingBrother le font  │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Caractérisation de la relation

**Relation de service :** TAMR définit le cadre normatif des interventions humaines. BondingBrother est le **canal obligatoire** de médiation pour toute intention d'intervention vers les autorités (StrongFather). L'intervention humaine est une intention comme une autre ; cette intention transite par BondingBrother.

**Relation sans autorité mutuelle :** TAMR ne commande pas BondingBrother. BondingBrother ne modifie pas les règles de TAMR. BondingBrother applique le cadre défini par TAMR pour valider la forme des intentions et les transmettre.

**Relation conceptuelle / exécution :** TAMR reste purement conceptuel (INV-TAMR-4). BondingBrother exécute la médiation technique. Les intentions doivent respecter les types et exigences de trace définis par TAMR.

### 2.3 Principe fondamental

> **Toute intention d'intervention humaine (approbation, override, escalade, supervision) transite par BondingBrother vers StrongFather. BondingBrother valide la conformité au cadre TAMR et transmet sans jamais décider de l'autorisation.**

Ce principe est non négociable. L'intégration garantit un canal unique et traçable pour les interventions humaines.

---

## 3. Points d'interaction formels

### 3.1 Médiation d'une intention d'approbation (Approval)

**Contexte d'utilisation :**

Un processus atteint un point d'approbation. Le système produit une intention d'approbation (demande de validation humaine avant exécution). Cette intention doit transiter par BondingBrother vers StrongFather.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           MÉDIATION D'INTENTION D'APPROBATION                      │
│                                                                   │
│  [Producteur : processus / produit]                              │
│      │                                                            │
│      │ 1. Crée une intention d'approbation conforme à TAMR       │
│      │    type: APPROVAL, contexte, point d'intervention, etc.    │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER REÇOIT                                    │ │
│  │                                                            │ │
│  │  • Vérifie la présence des champs requis par TAMR         │ │
│  │    (type, identité intervenant, point, contexte)           │ │
│  │  • Rejette si forme invalide (rejet de forme)             │ │
│  │  • Traduit pour StrongFather                              │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Intention traduite                                        │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER TRANSMET                                  │ │
│  │                                                            │ │
│  │  BondingBrother → StrongFather                            │ │
│  │  (StrongFather décide si l'approbation est autorisée)      │ │
│  │                                                            │ │
│  │  NOTE : BondingBrother ne décide PAS ; il médiatise        │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-APP-1 :** Toute intention d'approbation DOIT transiter par BondingBrother
- **INT-APP-2 :** BondingBrother PEUT rejeter une intention dont la forme ne respecte pas le cadre TAMR (type, champs de trace minimaux)
- **INT-APP-3 :** BondingBrother NE DOIT JAMAIS décider si l'approbation est accordée ou refusée
- **INT-APP-4 :** La décision d'autorisation appartient à StrongFather

### 3.2 Médiation d'une intention d'override (Override)

**Contexte d'utilisation :**

Un humain autorisé demande un override (dérogation à une décision automatique). L'intention d'override doit inclure une justification et transiter par BondingBrother vers StrongFather.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           MÉDIATION D'INTENTION D'OVERRIDE                         │
│                                                                   │
│  [Producteur]                                                    │
│      │                                                            │
│      │ 1. Crée une intention d'override conforme à TAMR          │
│      │    type: OVERRIDE, justification obligatoire (INV-TAMR-7) │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER REÇOIT                                    │ │
│  │                                                            │ │
│  │  • Vérifie type OVERRIDE et présence de justification     │ │
│  │  • Rejette si justification absente (rejet de forme)      │ │
│  │  • Transmet à StrongFather (vérification limites TAMR      │ │
│  │    et décision par StrongFather)                           │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  StrongFather : vérifie limites infranchissables (INV-TAMR-3),   │
│                 décide d'autoriser ou refuser l'override         │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-OVR-1 :** Toute intention d'override DOIT transiter par BondingBrother
- **INT-OVR-2 :** BondingBrother DOIT rejeter (forme) toute intention OVERRIDE sans justification
- **INT-OVR-3 :** BondingBrother NE PEUT PAS évaluer si l'override respecte les limites infranchissables ; StrongFather le fait
- **INT-OVR-4 :** La décision d'autoriser l'override appartient à StrongFather

### 3.3 Médiation d'une intention d'escalade (Escalation)

**Contexte d'utilisation :**

Une situation nécessite une escalade vers un niveau d'autorité supérieur. L'intention d'escalade transite par BondingBrother vers StrongFather (identification du niveau, destinataires).

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           MÉDIATION D'INTENTION D'ESCALADE                         │
│                                                                   │
│  [Producteur]                                                    │
│      │                                                            │
│      │ 1. Crée une intention d'escalade (type: ESCALATION)      │
│      │    contexte, point d'intervention, niveau cible si connu  │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER REÇOIT ET TRANSMET                         │ │
│  │                                                            │ │
│  │  • Valide la forme (type ESCALATION, champs TAMR)          │ │
│  │  • Transmet à StrongFather pour décision et routage         │ │
│  │  • BondingBrother ne décide pas du niveau ni des acteurs    │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-ESC-1 :** Toute intention d'escalade DOIT transiter par BondingBrother
- **INT-ESC-2 :** BondingBrother transmet la demande à StrongFather sans filtrer ni décider du niveau d'escalade
- **INT-ESC-3 :** La responsabilité du timeout / non-blocage (INV-TAMR-8) est du ressort du produit et de StrongFather, pas de BondingBrother

### 3.4 Médiation d'une intention de supervision (Supervision)

**Contexte d'utilisation :**

Un processus est placé sous supervision humaine. Les événements de supervision (début, fin, observations, interventions déclenchées) peuvent transiter par BondingBrother pour traçabilité et cohérence.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           MÉDIATION D'INTENTION / ÉVÉNEMENT DE SUPERVISION        │
│                                                                   │
│  [Producteur]                                                    │
│      │                                                            │
│      │ 1. Événements de supervision (début, fin, intervention    │
│      │    déclenchée dans le cadre de la supervision)            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER                                            │ │
│  │                                                            │ │
│  │  • Peut recevoir les événements pour traçabilité            │ │
│  │  • Transmet vers StrongFather si une intervention          │ │
│  │    (approval/override) est déclenchée depuis la supervision│ │
│  │  • Ne décide pas du périmètre de supervision                │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-SUP-1 :** Les interventions déclenchées dans le cadre d'une supervision suivent les mêmes règles (transit par BondingBrother) que les autres types
- **INT-SUP-2 :** BondingBrother peut être utilisé pour propager les événements de supervision aux composants concernés (observabilité, traçabilité)

### 3.5 Conformité de forme (cadre TAMR)

BondingBrother, lors de la réception d'une intention d'intervention, valide la **conformité de forme** au cadre TAMR :

| Exigence TAMR | Vérification BondingBrother |
|---------------|-----------------------------|
| Type d'intervention (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) | Présence et valeur reconnue |
| Champs de trace minimaux (identité intervenant, moment, contexte) | Présence des champs requis |
| Justification pour OVERRIDE | Présence obligatoire si type = OVERRIDE |
| Pas de champs interdits ou contradictoires | Rejet de forme si incohérence |

BondingBrother ne valide pas le contenu métier (ex. : l'intervenant a-t-il le droit ?), uniquement la forme et la cohérence avec les types TAMR.

---

## 4. Responsabilités dans l'intégration

### 4.1 Responsabilités de TAMR (cadre normatif)

TAMR étant un cadre conceptuel, il n'a pas de responsabilité d'exécution. Les responsabilités suivantes sont **définies par** TAMR et **respectées par** les producteurs et BondingBrother :

| Responsabilité | Description |
|----------------|-------------|
| **RESP-TAMR-1** | Définir les types d'intervention (Approval, Override, Escalation, Supervision) |
| **RESP-TAMR-2** | Définir les champs minimaux de trace pour toute intention d'intervention |
| **RESP-TAMR-3** | Définir l'obligation de justification pour les overrides |
| **RESP-TAMR-4** | Définir les limites infranchissables (évaluées par StrongFather, pas par BondingBrother) |

### 4.2 Responsabilités de BondingBrother

Dans le cadre de cette intégration, BondingBrother est responsable de :

| Responsabilité | Description |
|----------------|-------------|
| **RESP-BB-1** | Recevoir toute intention d'intervention humaine destinée à StrongFather |
| **RESP-BB-2** | Valider la conformité de forme au cadre TAMR (type, champs requis, justification si override) |
| **RESP-BB-3** | Rejeter (rejet de forme) les intentions non conformes sans les transmettre à StrongFather |
| **RESP-BB-4** | Traduire et transmettre les intentions conformes à StrongFather |
| **RESP-BB-5** | Ne jamais décider de l'autorisation ou du refus d'une intervention |
| **RESP-BB-6** | Tracer toute réception, rejet de forme, et transmission d'intention d'intervention |

### 4.3 Responsabilités partagées

| Responsabilité | TAMR (norme) | BondingBrother |
|----------------|--------------|----------------|
| **Traçabilité** | Définit ce qui doit être tracé | Trace la médiation et la transmission |
| **Conformité de forme** | Définit les critères | Vérifie et rejette si non conforme |
| **Non-décision** | INV-TAMR-5 (TAMR ne décide jamais) | Ne prend pas de décision d'autorisation |

---

## 5. Ce que l'intégration PEUT faire

### 5.1 Opérations autorisées

**PEUT-INT-1 : Transit obligatoire des intentions d'intervention**

Toute intention d'intervention humaine (approbation, override, escalade, supervision) PEUT et DOIT transiter par BondingBrother pour atteindre StrongFather. Aucun canal direct produit → StrongFather pour les interventions humaines n'est autorisé.

**PEUT-INT-2 : Validation de forme**

BondingBrother PEUT valider la conformité de forme des intentions au cadre TAMR (types, champs de trace, justification pour override) et rejeter les intentions non conformes (rejet de forme).

**PEUT-INT-3 : Traduction et enrichissement**

BondingBrother PEUT traduire et enrichir le contexte des intentions (sans modifier le sens) pour StrongFather, conformément à ses règles de médiation.

**PEUT-INT-4 : Traçabilité de la médiation**

BondingBrother PEUT et DOIT tracer toute réception, rejet de forme, et transmission d'intention d'intervention.

**PEUT-INT-5 : Propagation d'événements de supervision**

BondingBrother PEUT propager les événements de supervision (début, fin, interventions déclenchées) pour traçabilité et observabilité.

### 5.2 Garanties associées

- Toute intention d'intervention conforme à TAMR est transmise à StrongFather via BondingBrother.
- Les rejets de forme sont explicites et tracés.
- Aucune décision d'autorisation n'est prise par BondingBrother ; la souveraineté de StrongFather est préservée.

---

## 6. Ce que l'intégration NE PEUT JAMAIS faire

### 6.1 Interdictions absolues

**INTERDIT-INT-1 : Canal direct produit → StrongFather pour interventions**

Aucune intention d'intervention humaine NE PEUT être transmise directement du producteur à StrongFather en contournant BondingBrother. BondingBrother est le canal obligatoire.

**INTERDIT-INT-2 : Décision d'autorisation par BondingBrother**

BondingBrother NE PEUT JAMAIS décider si une intervention est autorisée ou refusée. Cette décision appartient exclusivement à StrongFather.

**INTERDIT-INT-3 : Évaluation des limites infranchissables par BondingBrother**

BondingBrother NE PEUT JAMAIS évaluer si un override respecte les limites infranchissables (INV-TAMR-3). Cette évaluation est du ressort de StrongFather.

**INTERDIT-INT-4 : Modification du cadre TAMR par BondingBrother**

BondingBrother NE PEUT JAMAIS étendre, restreindre ou modifier les types d'intervention ou les règles de trace définis par TAMR. Il applique le cadre, il ne le définit pas.

**INTERDIT-INT-5 : Transmission d'intentions non conformes**

BondingBrother NE PEUT JAMAIS transmettre à StrongFather une intention d'intervention qui ne respecte pas la forme TAMR (type reconnu, champs requis, justification si override). Il doit rejeter (forme) avant transmission.

**INTERDIT-INT-6 : Inférence ou enrichissement sémantique**

BondingBrother NE PEUT JAMAIS inférer ou ajouter des éléments de décision (ex. : « cet intervenant est autorisé »). Il transmet fidèlement l'intention et le contexte, sans verdict.

### 6.2 Justifications

Ces interdictions sont justifiées par :
- le respect de l'invariant TAMR INV-TAMR-5 (TAMR ne prend jamais de décision),
- le respect du rôle de BondingBrother (médiation, non-décision),
- la souveraineté de StrongFather sur les décisions d'autorisation,
- la traçabilité et l'auditabilité des interventions (INV-TAMR-1).

---

## 7. Invariants d'intégration

### 7.1 Invariants globaux

**INV-INT-1 : Canal unique**

Toute intention d'intervention humaine à destination de StrongFather transite par BondingBrother. Il n'existe pas de canal parallèle pour les interventions humaines.

**INV-INT-2 : Conformité de forme**

BondingBrother n'accepte pour transmission que les intentions conformes au cadre TAMR (types, champs de trace, justification pour override). Les autres sont rejetées (forme).

**INV-INT-3 : Non-décision**

BondingBrother ne prend aucune décision d'autorisation ou de refus d'intervention. Il médiatise uniquement.

**INV-INT-4 : Traçabilité de la médiation**

Toute réception, rejet de forme, et transmission d'intention d'intervention est tracée côté BondingBrother.

**INV-INT-5 : Préservation des invariants TAMR**

L'intégration préserve les invariants TAMR (INV-TAMR-1 à INV-TAMR-8). Notamment : traçabilité absolue, justification obligatoire pour override, limites infranchissables évaluées par StrongFather.

### 7.2 Invariants de flux

**INV-FLUX-1 : Sens unique intention → StrongFather**

Le flux des intentions d'intervention est : producteur → BondingBrother → StrongFather. StrongFather ne renvoie pas d'intention d'intervention à BondingBrother pour médiation (les réponses décisionnelles sont hors scope de ce contrat).

**INV-FLUX-2 : Rejet de forme sans transmission**

Toute intention rejetée pour non-conformité de forme n'est jamais transmise à StrongFather. Le rejet est explicite et tracé.

---

## 8. Cas d'utilisation concrets

### 8.1 Demande d'approbation avant publication

**Scénario :** Un rédacteur soumet un contenu pour publication. Le processus atteint un point d'approbation. L'intention d'approbation doit transiter par BondingBrother.

```
1. [Produit] Crée intention : type=APPROVAL, intervenant=id_rédacteur, point=pre_publication, contexte=content_id
2. [BondingBrother] Reçoit, valide forme (type, champs TAMR), traduit
3. [BondingBrother] Transmet à StrongFather
4. [StrongFather] Décide : approbation autorisée ou refusée pour ce rédacteur / ce contenu
5. Traces : BondingBrother trace réception et transmission ; le résultat est tracé selon TAMR (KindMother, etc.)
```

### 8.2 Override avec justification

**Scénario :** Un superviseur demande un override pour valider une action refusée automatiquement.

```
1. [Produit] Crée intention : type=OVERRIDE, intervenant=id_superviseur, justification="Validation exceptionnelle client X", contexte=action_id
2. [BondingBrother] Reçoit, vérifie présence de justification → conforme
3. [BondingBrother] Transmet à StrongFather
4. [StrongFather] Vérifie limites infranchissables, puis autorise ou refuse l'override
5. Si autorisé : l'override est appliqué et tracé (identité, justification, moment)
```

### 8.3 Rejet de forme (override sans justification)

**Scénario :** Une intention d'override est envoyée sans champ justification.

```
1. [Produit] Envoie intention : type=OVERRIDE, intervenant=id, contexte=action_id (sans justification)
2. [BondingBrother] Valide forme → justification absente → rejet de forme
3. [BondingBrother] Ne transmet pas à StrongFather, trace le rejet
4. [Produit] Reçoit erreur explicite (rejet de forme), peut corriger et renvoyer
```

---

## 9. Règles de traçabilité

### 9.1 Éléments à tracer côté BondingBrother

| Élément | Description |
|--------|-------------|
| `mediation_id` | Identifiant unique de la médiation |
| `timestamp` | Horodatage réception / transmission |
| `intention_type` | APPROVAL, OVERRIDE, ESCALATION, SUPERVISION |
| `outcome` | transmitted / rejected_form |
| `rejection_reason` | Si rejet de forme : raison (ex. missing_justification, unknown_type) |
| `correlation_id` | Lien avec la trace d'intervention côté produit / KindMother |

### 9.2 Corrélation avec les traces TAMR

Les traces d'intervention définies par TAMR (identité intervenant, type, moment, contexte, justification si override, résultat) sont produites par le système (produit, StrongFather, KindMother). Les traces BondingBrother permettent d'auditer que toute intention a bien transité par le canal obligatoire et avec quel résultat (transmise ou rejetée).

---

## 10. Gestion des erreurs

### 10.1 Rejets de forme (BondingBrother)

| Code / Raison | Signification | Action producteur |
|---------------|----------------|-------------------|
| `UNKNOWN_INTERVENTION_TYPE` | Type non reconnu (pas parmi APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) | Corriger le type selon TAMR |
| `MISSING_REQUIRED_FIELDS` | Champs de trace requis absents | Ajouter identité, moment, contexte |
| `MISSING_JUSTIFICATION` | Type OVERRIDE sans justification | Ajouter justification (INV-TAMR-7) |
| `MALFORMED_INTENTION` | Structure incohérente | Corriger la forme selon le cadre TAMR |

### 10.2 Principe

> **En cas de rejet de forme, BondingBrother DOIT retourner une erreur explicite et tracée. Aucune intention non conforme ne DOIT être transmise à StrongFather.**

---

## 11. Compatibilité avec les invariants existants

### 11.1 Respect des invariants TAMR

| Invariant TAMR | Respect dans l'intégration |
|----------------|----------------------------|
| **INV-TAMR-1** (Traçabilité absolue) | BondingBrother trace toute médiation ; les traces d'intervention complètes restent du ressort produit / KindMother |
| **INV-TAMR-5** (Non-décision) | BondingBrother ne prend aucune décision ; StrongFather décide |
| **INV-TAMR-7** (Justification override) | BondingBrother rejette toute intention OVERRIDE sans justification |
| **INV-TAMR-4** (Séparation conceptuel/technique) | TAMR reste conceptuel ; BondingBrother exécute la médiation technique |

### 11.2 Respect des invariants BondingBrother

| Invariant BB | Respect dans l'intégration |
|--------------|----------------------------|
| **BB-INV-1** (Non-décision) | BondingBrother ne décide jamais de l'autorisation d'une intervention |
| **BB-INV-4** (Traçabilité) | Toute médiation d'intention d'intervention est tracée |
| **BB-INV-3** (Non-déduction) | BondingBrother ne déduit pas de verdict à partir des intentions |
| **BB-INV-7** (Contrat) | Les échanges respectent ce contrat et le cadre TAMR |

---

## 12. Conformité aux Lois d'Autonomie Système

L'intégration respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) :

- **LOI-1 (Aucune dépendance externe critique)** : La médiation des intentions d'intervention peut s'effectuer localement ; BondingBrother et StrongFather fonctionnent en local.
- **LOI-2 (Isolement comme état normal)** : Les intentions d'intervention peuvent être produites et médiatisées en mode isolé ; la décision StrongFather et la traçabilité KindMother sont compatibles offline-first.
- **LOI-3 à LOI-6** : Aucune violation introduite par ce contrat ; le canal BondingBrother est un composant de la strate de liaison, pas une dépendance bloquante.

---

## 13. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable les règles d'intégration entre TAMR et BondingBrother pour la **médiation des intentions d'intervention humaine**.

Il garantit que :
- toute intention d'intervention (approbation, override, escalade, supervision) transite par BondingBrother vers StrongFather ;
- BondingBrother valide la conformité de forme au cadre TAMR et rejette les intentions non conformes sans les transmettre ;
- aucune décision d'autorisation n'est prise par BondingBrother ;
- la traçabilité de la médiation est assurée ;
- les invariants TAMR et BondingBrother sont respectés.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, [TAMR Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md), [TAMR Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md), [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md), [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)  
**Type :** Contrat d'intégration non négociable

---

## 14. Références croisées (plan)

| Référence | Usage |
|-----------|--------|
| [Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie TAMR (intervention, approbation, override, escalade, supervision, trace) |
| [Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) | Principes de sécurité |
| [Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) | Conformité LOI-1 à LOI-6 |
| [Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) | Niveaux T0-T4 |
| [Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Niveaux 0-4 |

---

## 15. Mini log — ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : TAMR n'a pas d'exécution

**Ambiguïté rencontrée :** TAMR est un cadre conceptuel (INV-TAMR-4) ; il ne « envoie » rien. Qui émet les intentions ?

**Décision prise :** Les intentions sont produites par le **système** (processus, produit). TAMR définit la **forme** et les **règles** que ces intentions doivent respecter. BondingBrother est le **canal obligatoire** pour les transmettre à StrongFather. Le contrat décrit donc l'obligation de transit et la validation de forme par BondingBrother, pas un échange TAMR ↔ BondingBrother au sens technique.

**Correction effectuée :** Sections 2 et 3 rédigées en conséquence (BondingBrother reçoit du producteur, valide selon cadre TAMR, transmet à StrongFather).

### Ambiguïté A2 : Rejet de forme vs rejet d'autorisation

**Ambiguïté rencontrée :** Ne pas confondre le rejet par BondingBrother (forme non conforme) avec le refus d'autorisation par StrongFather.

**Décision prise :** Clarification explicite : BondingBrother rejette en **rejet de forme** (intention non conforme au cadre TAMR) ; StrongFather décide **autorisation / refus**. Les deux sont tracés séparément.

**Correction effectuée :** Sections 3, 6 et 8 (cas 8.3) précisent cette distinction.

### Vérification de compatibilité

**Vérification effectuée :** Cohérence avec la Documentation Fondatrice TAMR (relation BondingBrother), avec les contrats Intervention Types / Points, et avec les invariants TAMR et BondingBrother. Aucune contradiction détectée.

**Conclusion :** Le contrat est compatible avec le corpus TAMR et BondingBrother. Il formalise le canal unique de médiation des intentions d'intervention humaine.

---

*Aucune autre erreur, warning ou ambiguïté rencontrée lors de la rédaction de ce document.*
