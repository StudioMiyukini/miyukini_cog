# WorrySentinel - BorderGuard Integration Contract

## 1. Contexte

Ce document definit formellement le contrat d'integration entre **WorrySentinel** (core de gouvernance transversale de securite) et **BorderGuard** (core de definition des frontieres et classification de confiance). Il precise les flux bidirectionnels, les responsabilites respectives, les regles d'adaptation, et les invariants de cette integration strategique.

**Document fondateur WorrySentinel :** [Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

**Document fondateur BorderGuard :** [Border Guard - Documentation Fondatrice](../../../../core/BorderGuard/foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non negociable**. Il etablit les regles absolues de la relation entre WorrySentinel et BorderGuard.

---

## 2. Portee / Scope

### Ce document couvre

- La nature de la relation WorrySentinel-BorderGuard
- Le flux descendant de gouvernance (WorrySentinel → BorderGuard)
- Le flux montant de signaux (BorderGuard → WorrySentinel)
- L'adaptation des frontieres selon les niveaux de securite (0-4)
- L'adaptation des frontieres selon les etats de confiance (T0-T4)
- La matrice d'interaction entre gouvernance et definition de frontieres
- Les invariants de l'integration
- Les violations et anti-patterns

### Ce document ne couvre pas

- La definition detaillee des niveaux de securite (voir [Security Levels Governance Contract](../levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md))
- La definition detaillee des etats de confiance (voir [Trust States Governance Contract](../levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md))
- Les mecanismes de degradation progressive (voir [Progressive Degradation Contract](../degradation/WorrySentinel%20-%20Progressive%20Degradation%20Contract.md))
- Les details internes de BorderGuard (voir documentation BorderGuard)

---

## 3. Principe fondamental de la relation

### 3.1 Complementarite sans chevauchement

WorrySentinel et BorderGuard sont **complementaires et independants**. Chacun possede son domaine d'autorite exclusive :

| Core | Domaine d'autorite | Ce qu'il ne fait jamais |
|------|-------------------|------------------------|
| **WorrySentinel** | Gouvernance des niveaux de securite (0-4), gouvernance des etats de confiance (T0-T4), orchestration degradation | Ne definit pas de frontieres, ne classifie pas les sources |
| **BorderGuard** | Definition des frontieres, classification de confiance (trusted/verified/unknown/hostile), regles de franchissement | Ne gouverne pas les niveaux de securite, ne modifie pas l'etat global |

### 3.2 Nature de l'interaction

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     RELATION WORRYSENTINEL ↔ BORDERGUARD                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  WorrySentinel                                         BorderGuard           │
│  ─────────────                                         ───────────           │
│  • Gouverne les niveaux de securite (0-4)             • Definit les frontieres│
│  • Gouverne les etats de confiance (T0-T4)            • Classifie la confiance│
│  • Impose contraintes de durcissement                    (trusted→hostile)   │
│  • Observe les signaux d'anomalie                     • Etablit regles de    │
│  • Ne definit JAMAIS de frontiere                       franchissement       │
│  • Ne classifie JAMAIS de source                      • Ne gouverne JAMAIS   │
│                                                          l'etat global       │
│                              ▼                              ▲                │
│                              │                              │                │
│                    FLUX DESCENDANT                  FLUX MONTANT             │
│                    (durcissement)                   (signaux)                │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.3 Distinction entre niveaux de confiance

**IMPORTANT :** Deux concepts distincts de "confiance" coexistent :

| Concept | Defini par | Echelle | Nature |
|---------|------------|---------|--------|
| **Etat de confiance systeme** | WorrySentinel | T0 → T4 | Integrite globale du systeme |
| **Niveau de confiance source** | BorderGuard | trusted → hostile | Classification d'une source/destination |

Ces deux concepts sont **independants mais interconnectes**. L'etat de confiance systeme (T0-T4) influence la rigueur avec laquelle BorderGuard classifie les sources.

---

## 4. Flux descendant : Gouvernance → Frontieres

WorrySentinel impose des contraintes de gouvernance sur BorderGuard selon deux axes : le niveau de securite declare et l'etat de confiance du systeme.

### 4.1 Contraintes selon niveau de securite (0-4)

WorrySentinel gouverne l'adaptation des frontieres selon le niveau de securite declare par l'Operateur :

```
┌─────────────────────────────────────────────────────────────────────────────┐
│             CONTRAINTES WORRYSENTINEL → BORDERGUARD (NIVEAU SECURITE)        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Niveau 0 (PUBLIC)          ──→ Frontieres assouplies, classification simple │
│                                                                              │
│  Niveau 1 (STANDARD)        ──→ Frontieres standard, classification normale  │
│                                                                              │
│  Niveau 2 (SENSITIVE)       ──→ Frontieres renforcees, classification stricte│
│                                                                              │
│  Niveau 3 (CRITICAL)        ──→ Frontieres strictes, classification rigoureuse│
│                                                                              │
│  Niveau 4 (HARDENED)        ──→ Frontieres maximales, classification ultra-stricte│
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Tableau detaille des contraintes par niveau :**

| Niveau securite | Permeabilite frontiere externe | Classification sources | Seuil hostile | TTL VERIFIED |
|-----------------|-------------------------------|----------------------|---------------|--------------|
| **0 - PUBLIC** | Ouverte | Simplifiee | Haut (tolerant) | Long (heures) |
| **1 - STANDARD** | Controlee | Normale | Standard | Standard (minutes) |
| **2 - SENSITIVE** | Controlee + verif | Renforcee | Bas (sensible) | Court |
| **3 - CRITICAL** | Stricte | Stricte | Tres bas | Tres court |
| **4 - HARDENED** | Fermee | Ultra-stricte | Zero tolerance | Minimal |

### 4.2 Contraintes selon etat de confiance (T0-T4)

WorrySentinel impose un durcissement progressif des frontieres selon l'etat de confiance global :

```
┌─────────────────────────────────────────────────────────────────────────────┐
│             CONTRAINTES WORRYSENTINEL → BORDERGUARD (ETAT CONFIANCE)         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  T0 (NORMAL)      ──→ Fonctionnement normal des frontieres                   │
│                                                                              │
│  T1 (INSTABLE)    ──→ + Verifications supplementaires                        │
│                        + Surveillance renforcee des frontieres               │
│                        + Reevaluation plus frequente des classifications     │
│                                                                              │
│  T2 (DEGRADE)     ──→ Frontieres resserrees                                  │
│                        Certains types de franchissement suspendus            │
│                        Classification plus restrictive                       │
│                                                                              │
│  T3 (RESTREINT)   ──→ Frontieres minimales                                   │
│                        Uniquement franchissements essentiels                 │
│                        Classification ultra-stricte                          │
│                                                                              │
│  T4 (BLOQUE)      ──→ Frontieres fermees                                     │
│                        Aucun franchissement externe autorise                 │
│                        Mode isolement total                                  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Tableau des actions de durcissement :**

| Etat confiance | Durcissement frontieres | Action sur integrations | Impact classification |
|----------------|------------------------|------------------------|----------------------|
| **T0** | Aucun | Aucune | Normale |
| **T1** | + Verifications | Surveillance accrue | + Rigueur |
| **T2** | Resserrement | Restrictions moderees | Stricte |
| **T3** | Frontieres minimales | Suspension non essentielles | Ultra-stricte |
| **T4** | Fermees | Toutes suspendues | N/A (isole) |

### 4.3 Interface de gouvernance

WorrySentinel communique ses contraintes a BorderGuard via une interface conceptuelle :

| Interface | Description | Direction |
|-----------|-------------|-----------|
| `IBoundaryHardening` | Niveau de durcissement requis des frontieres | WS → BG |
| `IClassificationRigor` | Rigueur de classification requise | WS → BG |
| `IPermeabilityConstraint` | Contrainte de permeabilite des frontieres | WS → BG |
| `IIntegrationRestriction` | Restrictions sur les integrations externes | WS → BG |

---

## 5. Flux montant : Signaux → Gouvernance

BorderGuard signale a WorrySentinel les evenements qui peuvent influencer l'etat de confiance global.

### 5.1 Types de signaux

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     SIGNAUX BORDERGUARD → WORRYSENTINEL                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  BorderGuard                                              WorrySentinel      │
│  ───────────                                              ─────────────      │
│                                                                              │
│  • Anomalies I/O                           ──────────→   Correlation et      │
│    (patterns inhabituels, frequences                     analyse             │
│     anormales, volumes suspects)                                             │
│                                                                              │
│  • Violations de frontieres                ──────────→   Impact sur etat     │
│    (tentatives non autorisees,                           de confiance        │
│     contournements detectes)                                                 │
│                                                                              │
│  • Classifications HOSTILE declenchees     ──────────→   Signal d'alerte     │
│    (sources nouvellement hostiles)                                           │
│                                                                              │
│  • Defaillances d'integrations             ──────────→   Evaluation          │
│    (partenaires defaillants,                             degradation         │
│     connexions interrompues)                                                 │
│                                                                              │
│  • Patterns de franchissement anormaux     ──────────→   Detection           │
│    (tentatives repetees,                                 intrusion           │
│     escalade de privileges)                                                  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Catalogue des signaux

| Signal | Description | Gravite | Impact potentiel sur T |
|--------|-------------|---------|------------------------|
| `SIG-BG-ANOMALY-IO` | Pattern I/O inhabituel detecte | Moderee | T0 → T1 |
| `SIG-BG-VIOLATION` | Tentative de franchissement non autorise | Haute | T1 → T2 |
| `SIG-BG-HOSTILE-NEW` | Nouvelle source classifiee HOSTILE | Haute | T1 → T2 |
| `SIG-BG-INTEGRATION-FAIL` | Integration externe defaillante | Moderee | T0 → T1 |
| `SIG-BG-ESCALATION` | Tentative d'escalade de privileges | Critique | T2 → T3 |
| `SIG-BG-BREACH` | Violation confirmee de frontiere | Critique | T2 → T3+ |
| `SIG-BG-PATTERN-ATTACK` | Pattern d'attaque detecte | Critique | T2 → T3+ |

### 5.3 Interface de signalement

| Interface | Description | Direction |
|-----------|-------------|-----------|
| `IBoundarySignal` | Signalement de violation de frontiere | BG → WS |
| `IAnomalyIOSignal` | Signalement d'anomalie I/O | BG → WS |
| `IHostileClassificationSignal` | Signalement de nouvelle classification hostile | BG → WS |
| `IIntegrationFailureSignal` | Signalement de defaillance d'integration | BG → WS |

---

## 6. Matrice d'interaction Niveau securite × Etat confiance

L'interaction entre le niveau de securite et l'etat de confiance produit un comportement combine pour BorderGuard.

### 6.1 Matrice de comportement des frontieres

```
                        NIVEAUX DE SECURITE
                    0        1        2        3        4
                ┌────────┬────────┬────────┬────────┬────────┐
         T0     │ Ouvert │Standard│Renforcé│ Strict │ Fermé  │
                │        │        │        │        │sauf int│
                ├────────┼────────┼────────┼────────┼────────┤
E        T1     │Standard│Standard│Strict  │Strict+ │Fermé   │
T               │+verif  │+verif  │        │+attesta│        │
A               ├────────┼────────┼────────┼────────┼────────┤
T        T2     │Renforcé│Strict  │Minimal │Minimal │Isolé   │
S               │        │        │        │+gel    │        │
                ├────────┼────────┼────────┼────────┼────────┤
         T3     │Strict  │Minimal │Isolé   │Isolé   │Isolé   │
                │        │        │partiel │total   │total   │
                ├────────┼────────┼────────┼────────┼────────┤
         T4     │Isolé   │Isolé   │Isolé   │Isolé   │Isolé   │
                │        │        │        │        │        │
                └────────┴────────┴────────┴────────┴────────┘

Legende :
• Ouvert    : Franchissement libre sous conditions minimales
• Standard  : Franchissement controle
• Renforcé  : Franchissement soumis a verification stricte
• Strict    : Zero-trust strict, signatures obligatoires
• Minimal   : Uniquement franchissements essentiels
• Isolé     : Frontieres fermees, mode survie
```

### 6.2 Matrice de classification des sources

```
                        NIVEAUX DE SECURITE
                    0        1        2        3        4
                ┌────────┬────────┬────────┬────────┬────────┐
         T0     │Simple  │Normale │Renforcée│Stricte │Ultra   │
                │        │        │        │        │stricte │
                ├────────┼────────┼────────┼────────┼────────┤
E        T1     │Normale │Normale │Stricte │Stricte │Ultra   │
T               │        │+trace  │        │+verif  │stricte │
A               ├────────┼────────┼────────┼────────┼────────┤
T        T2     │Renforcée│Stricte│Ultra   │Ultra   │Tout    │
S               │        │        │stricte │stricte │suspect │
                ├────────┼────────┼────────┼────────┼────────┤
         T3     │Stricte │Ultra   │Tout    │Tout    │Tout    │
                │        │stricte │suspect │hostile │hostile │
                ├────────┼────────┼────────┼────────┼────────┤
         T4     │Tout    │Tout    │Tout    │Tout    │Tout    │
                │suspect │hostile │hostile │hostile │hostile │
                └────────┴────────┴────────┴────────┴────────┘
```

---

## 7. Regles d'integration

### 7.1 Regles de gouvernance descendante

| Regle | Description |
|-------|-------------|
| **REGLE-WS-BG-1** | WorrySentinel impose le durcissement des frontieres sans les definir |
| **REGLE-WS-BG-2** | BorderGuard adapte ses definitions selon les contraintes de WorrySentinel |
| **REGLE-WS-BG-3** | Tout changement d'etat de confiance declenche une reevaluation des frontieres |
| **REGLE-WS-BG-4** | BorderGuard ne peut pas assouplir une frontiere au-dela des contraintes imposees |
| **REGLE-WS-BG-5** | En etat T3+, BorderGuard passe en mode frontieres minimales automatiquement |
| **REGLE-WS-BG-6** | En etat T4, BorderGuard isole completement le systeme |

### 7.2 Regles de signalement montant

| Regle | Description |
|-------|-------------|
| **REGLE-BG-WS-1** | BorderGuard signale toute anomalie I/O significative |
| **REGLE-BG-WS-2** | BorderGuard signale toute violation de frontiere |
| **REGLE-BG-WS-3** | BorderGuard signale toute classification HOSTILE declenchee |
| **REGLE-BG-WS-4** | Les signaux sont emis en temps reel sans delai |
| **REGLE-BG-WS-5** | Les signaux incluent le contexte complet (source, frontiere, moment) |
| **REGLE-BG-WS-6** | BorderGuard ne decide pas de l'impact sur l'etat de confiance |

### 7.3 Regles de coherence

| Regle | Description |
|-------|-------------|
| **REGLE-COHERENCE-1** | La classification de confiance source est independante de l'etat de confiance systeme |
| **REGLE-COHERENCE-2** | Le niveau de securite influence la rigueur, pas le resultat de classification |
| **REGLE-COHERENCE-3** | Un etat de confiance degrade ne transforme pas automatiquement toutes les sources en HOSTILE |
| **REGLE-COHERENCE-4** | Les contraintes sont additives : niveau securite + etat confiance = restrictions combinees |

---

## 8. Invariants du contrat

Ces invariants sont **non negociables** et definissent les limites absolues de l'integration.

### 8.1 Invariants de WorrySentinel dans cette integration

| Code | Invariant | Description |
|------|-----------|-------------|
| **INV-WS-BG-1** | Aucune definition de frontiere | WorrySentinel ne definit jamais de frontiere |
| **INV-WS-BG-2** | Aucune classification de source | WorrySentinel ne classifie jamais trusted/verified/unknown/hostile |
| **INV-WS-BG-3** | Aucune execution de controle | WorrySentinel n'execute jamais de verification de frontiere |
| **INV-WS-BG-4** | Contraintes uniquement | WorrySentinel impose des contraintes, pas des definitions |

### 8.2 Invariants de BorderGuard dans cette integration

| Code | Invariant | Description |
|------|-----------|-------------|
| **INV-BG-WS-1** | Aucune gouvernance d'etat | BorderGuard ne gouverne jamais l'etat de confiance systeme |
| **INV-BG-WS-2** | Aucune modification d'etat | BorderGuard ne modifie jamais l'etat de confiance systeme |
| **INV-BG-WS-3** | Signalement uniquement | BorderGuard signale, WorrySentinel correle et decide |
| **INV-BG-WS-4** | Respect des contraintes | BorderGuard ne peut pas contourner les contraintes de WorrySentinel |

### 8.3 Invariants mutuels

| Code | Invariant | Description |
|------|-----------|-------------|
| **INV-MUTUAL-1** | Separation des domaines | Chaque core reste souverain dans son domaine |
| **INV-MUTUAL-2** | Pas de dependance cyclique | Les flux sont unidirectionnels par type (gouvernance vs signaux) |
| **INV-MUTUAL-3** | Tracabilite complete | Tout flux (contrainte ou signal) est trace |
| **INV-MUTUAL-4** | Zero-trust mutuel | Chaque core valide les informations recues |

---

## 9. Violations et anti-patterns

### 9.1 Violations

| Code | Violation | Invariant viole |
|------|-----------|-----------------|
| **VIOL-WS-BG-1** | WorrySentinel definit une frontiere | INV-WS-BG-1 |
| **VIOL-WS-BG-2** | WorrySentinel classifie une source | INV-WS-BG-2 |
| **VIOL-WS-BG-3** | WorrySentinel execute un controle de frontiere | INV-WS-BG-3 |
| **VIOL-BG-WS-1** | BorderGuard modifie l'etat de confiance | INV-BG-WS-2 |
| **VIOL-BG-WS-2** | BorderGuard ignore les contraintes de WorrySentinel | INV-BG-WS-4 |
| **VIOL-BG-WS-3** | BorderGuard decide de l'impact d'un signal | INV-BG-WS-3 |

### 9.2 Anti-patterns

| Anti-pattern | Description | Correction |
|--------------|-------------|------------|
| **Confusion confiance** | Confondre etat de confiance systeme (T0-T4) et classification source (trusted/hostile) | Maintenir la distinction : systeme ≠ source |
| **Bypass contraintes** | BorderGuard ignore les contraintes en invoquant la commodite | Les contraintes sont non negociables |
| **Correlation locale** | BorderGuard correle les signaux au lieu de les transmettre | WorrySentinel est seul responsable de la correlation |
| **Gouvernance de frontiere** | WorrySentinel specifie des frontieres au lieu de contraintes | WorrySentinel gouverne le durcissement, pas les definitions |
| **Decision operationnelle** | WorrySentinel prend des decisions de blocage de franchissement | StrongFather decide, BorderGuard definit, WorrySentinel gouverne |

---

## 10. Exemples concrets

### 10.1 Scenario : Degradation T0 → T2

```
CONTEXTE : Operateur niveau 2 (SENSITIVE), etat initial T0

1. BorderGuard detecte des patterns I/O anormaux
   → Signal SIG-BG-ANOMALY-IO emis vers WorrySentinel

2. WorrySentinel correle avec d'autres signaux
   → Transition T0 → T1 declaree

3. WorrySentinel impose contraintes T1 a BorderGuard :
   → + Verifications supplementaires
   → Reevaluation plus frequente des classifications

4. BorderGuard detecte une violation de frontiere
   → Signal SIG-BG-VIOLATION emis vers WorrySentinel

5. WorrySentinel correle
   → Transition T1 → T2 declaree

6. WorrySentinel impose contraintes T2 a BorderGuard :
   → Frontieres resserrees
   → Classification ultra-stricte
   → Certains types de franchissement suspendus

7. BorderGuard adapte ses definitions :
   → Frontieres externes passent en mode "Strict"
   → Integrations non essentielles suspendues
   → Classification devient ultra-stricte
```

### 10.2 Scenario : Niveau de securite eleve

```
CONTEXTE : Operateur niveau 4 (HARDENED), etat T0

1. WorrySentinel impose contraintes niveau 4 a BorderGuard :
   → Frontieres maximales (fermees sauf interne)
   → Classification ultra-stricte
   → Seuil hostile = zero tolerance
   → Reevaluation TRUSTED constante

2. BorderGuard applique ces contraintes :
   → Frontiere externe : Fermee
   → Frontiere integration : Minimale/Aucune
   → TTL VERIFIED : Minimal
   → Criteres VERIFIED : Ultra-stricts (verification continue)
   → Distribution TRUSTED : Quasi nulle (isolement)

3. Meme en T0, le systeme fonctionne en mode tres restrictif
   car le niveau de securite l'impose.
```

### 10.3 Scenario : Signal hostile

```
CONTEXTE : Operateur niveau 2, etat T1

1. BorderGuard classifie une source comme HOSTILE
   (pattern d'attaque detecte)

2. Signal SIG-BG-HOSTILE-NEW emis vers WorrySentinel
   Contenu : {source, raison, frontiere, moment}

3. WorrySentinel correle avec signaux existants :
   → Plusieurs sources hostiles = pattern
   → Correlation avec StrongFather (decisions refusees)

4. WorrySentinel decide de la transition :
   → Si pattern confirme : T1 → T2
   → Si isole : T1 maintenu, surveillance accrue

5. BorderGuard ne connait PAS le resultat de la correlation
   Il continue a signaler et a appliquer les contraintes recues
```

---

## 11. Protocoles de communication

### 11.1 Protocole de contrainte descendante

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     PROTOCOLE CONTRAINTE (WS → BG)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  1. WorrySentinel evalue le contexte                                         │
│     • Niveau de securite de l'Operateur                                     │
│     • Etat de confiance actuel                                              │
│                                                                              │
│  2. WorrySentinel calcule les contraintes                                    │
│     • Niveau de durcissement requis                                         │
│     • Rigueur de classification requise                                     │
│     • Restrictions sur integrations                                         │
│                                                                              │
│  3. WorrySentinel emet les contraintes                                       │
│     • Via interfaces IBoundaryHardening, IClassificationRigor, etc.          │
│     • Contraintes explicites et non ambigues                                │
│                                                                              │
│  4. BorderGuard recoit et valide                                             │
│     • Verification coherence des contraintes                                │
│     • Journalisation de la reception                                        │
│                                                                              │
│  5. BorderGuard adapte ses definitions                                       │
│     • Application immediate des contraintes                                 │
│     • Reevaluation des classifications en cours                             │
│     • Ajustement de la permeabilite des frontieres                          │
│                                                                              │
│  6. BorderGuard confirme l'application                                       │
│     • Retour de confirmation vers WorrySentinel (optionnel)                  │
│     • Journalisation des adaptations effectuees                             │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 11.2 Protocole de signal montant

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     PROTOCOLE SIGNAL (BG → WS)                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  1. BorderGuard detecte un evenement                                         │
│     • Anomalie I/O                                                          │
│     • Violation de frontiere                                                │
│     • Classification HOSTILE                                                │
│     • Defaillance d'integration                                             │
│                                                                              │
│  2. BorderGuard construit le signal                                          │
│     • Type de signal (SIG-BG-*)                                             │
│     • Contexte complet (source, frontiere, moment)                          │
│     • Gravite evaluee                                                       │
│     • Details techniques pertinents                                         │
│                                                                              │
│  3. BorderGuard emet le signal                                               │
│     • Via interface IBoundarySignal ou specifique                            │
│     • Emission immediate (pas de batching)                                  │
│     • Journalisation de l'emission                                          │
│                                                                              │
│  4. WorrySentinel recoit et journalise                                       │
│     • Reception confirmee                                                   │
│     • Signal ajoute a la correlation                                        │
│                                                                              │
│  5. WorrySentinel correle                                                    │
│     • Correlation avec autres signaux                                       │
│     • Evaluation de l'impact potentiel                                      │
│     • Decision de transition (ou non)                                       │
│                                                                              │
│  6. Si transition : nouvelles contraintes emises                             │
│     → Retour au protocole de contrainte                                     │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 12. Documents associes

### Documentation WorrySentinel

- [WorrySentinel - Index de Navigation](../../_index.md)
- [WorrySentinel - Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)
- [WorrySentinel - Architecture & Flows](../../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md)
- [WorrySentinel - Security Levels Governance Contract](../levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)
- [WorrySentinel - Trust States Governance Contract](../levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)
- [WorrySentinel - Progressive Degradation Contract](../degradation/WorrySentinel%20-%20Progressive%20Degradation%20Contract.md)

### Documentation BorderGuard

- [BorderGuard - Index de Navigation](../../../../core/BorderGuard/_index.md)
- [Border Guard - Documentation Fondatrice](../../../../core/BorderGuard/foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)
- [Border Guard - Security Levels Adaptation Contract](../../../../core/BorderGuard/contracts/security/Border%20Guard%20-%20Security%20Levels%20Adaptation%20Contract.md)
- [Border Guard - Boundary Definition Contract](../../../../core/BorderGuard/contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md)
- [Border Guard - Trust Level Classification Contract](../../../../core/BorderGuard/contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md)

### Documentation transversale

- [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- [Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)
- [Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

## 13. Synthese contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Separation des domaines** — WorrySentinel gouverne, BorderGuard definit, aucun chevauchement
2. **Flux bidirectionnels** — Gouvernance descendante, signaux montants
3. **Adaptation automatique** — BorderGuard adapte ses definitions selon les contraintes recues
4. **Correlation centralisee** — WorrySentinel seul correle et decide de l'impact sur l'etat
5. **Tracabilite complete** — Toute interaction est tracee
6. **Coherence garantie** — Matrice d'interaction explicite et non ambigue

### Phrase de synthese

> **WorrySentinel impose des contraintes de durcissement des frontieres selon les niveaux de securite et les etats de confiance, tandis que BorderGuard signale les anomalies et violations qui alimentent la correlation — chacun souverain dans son domaine, complementaires sans chevauchement, unis par des flux explicites et traces.**

---

## 14. Mini log de generation

### Ambiguite A1 : Confusion entre types de confiance

**Ambiguite rencontree :** Risque de confusion entre "etat de confiance systeme" (T0-T4, gouverne par WorrySentinel) et "niveau de confiance source" (trusted/verified/unknown/hostile, classifie par BorderGuard).

**Decision prise :** Section 3.3 ajoutee pour clarifier explicitement cette distinction. Les deux concepts sont documentes comme independants mais interconnectes.

**Correction effectuee :** Tableau de distinction ajoute, terminologie strictement separee dans tout le document.

### Ambiguite A2 : Direction des flux

**Ambiguite rencontree :** La relation etait decrite comme "contrainte" dans la documentation fondatrice, mais sans precision sur la bidirectionnalite.

**Decision prise :** Deux flux distincts documentes : flux descendant (gouvernance → durcissement) et flux montant (signaux → correlation).

**Correction effectuee :** Sections 4 et 5 structurees selon cette distinction, diagrammes de flux ajoutes.

### Verification de coherence

**Verification effectuee :**
- ✅ Compatible avec WorrySentinel - Documentation Fondatrice (section 9, relation avec BorderGuard)
- ✅ Compatible avec WorrySentinel - Architecture & Flows (section 9.3)
- ✅ Compatible avec Border Guard - Documentation Fondatrice (relation avec WorrySentinel)
- ✅ Compatible avec Border Guard - Security Levels Adaptation Contract
- ✅ Invariants WorrySentinel respectes (INV-WS-1 a INV-WS-8)
- ✅ Invariants BorderGuard respectes (INV-BG-1 a INV-BG-10)
- ✅ Separation gouvernance/definition preservee
- ✅ Aucune contradiction detectee

---

**Version :** 1.0.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Reference :** WorrySentinel v1.2, BorderGuard v1.5  
**Type :** Contrat d'integration entre cores
