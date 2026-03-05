# WorrySentinel - StrongFather Integration Contract

## 1. Contexte

Ce document definit le **contrat d'integration entre WorrySentinel et StrongFather**. Il specifie l'interface, le protocole, les regles de communication, et les garanties associees a l'integration entre le core de gouvernance de securite (WorrySentinel) et le moteur de decision strategique et politique (StrongFather).

Ce document complete la section "Relations avec les autres Cores" de l'[Index de Navigation](../../_index.md) et s'appuie sur :
- [WorrySentinel - Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) pour la nature de WorrySentinel
- [WorrySentinel - Core Interaction Contract](../../architecture/WorrySentinel%20-%20Core%20Interaction%20Contract.md) pour le modele d'interaction
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md) pour la nature de StrongFather
- [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) pour les niveaux 0-4
- [Miyukini Conceptual References - Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md) pour les etats T0-T4

L'integration respecte les [Lois d'Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) : toutes les interactions sont locales et ne requierent aucune dependance externe (**LOI-1**).

---

## 2. Portee / Scope

Ce document couvre :
- L'interface contractuelle entre WorrySentinel et StrongFather
- Le role de WorrySentinel dans la gouvernance de la severite des decisions
- Les donnees echangees entre les deux cores
- L'impact des etats de confiance sur les decisions de StrongFather
- Le flux montant d'observation des decisions refusees
- Les garanties de l'integration

Ce document **ne couvre pas** :
- Les details internes de StrongFather (voir documentation StrongFather)
- Les details internes de WorrySentinel (voir documentation WorrySentinel)
- La logique de degradation progressive detaillee (voir Progressive Degradation Contract)
- L'integration avec les autres cores (voir contrats d'integration specifiques)

---

## 3. Principe fondamental

**WorrySentinel gouverne la severite selon laquelle StrongFather doit decider. StrongFather decide si une action est autorisee. WorrySentinel ne prend jamais de decision a la place de StrongFather.**

La relation est de **gouvernance-adaptation** :
- WorrySentinel fournit le contexte de securite (niveaux de securite, etats de confiance)
- StrongFather adapte la severite de ses politiques selon ce contexte
- WorrySentinel observe les decisions refusees pour alimenter son evaluation de l'etat global

Cette relation garantit que :
- La gouvernance de securite reste centralisee dans WorrySentinel
- L'autorite decisionnelle reste dans StrongFather
- Les deux cores sont complementaires et independants
- Aucune confusion d'autorite n'existe

---

## 4. Nature de la relation WorrySentinel â€” StrongFather

### 4.1 Relation de gouvernance-adaptation

**WorrySentinel fournit a StrongFather :**
- Le niveau de securite applicable (0-4)
- L'etat de confiance du systeme (T0-T4)
- Les contraintes de severite requises

**StrongFather fournit a WorrySentinel :**
- Les signaux de decisions refusees (pour observation et correlation)

**Regle WS-SF-01 : Gouvernance sans substitution**

WorrySentinel gouverne la severite des decisions de StrongFather mais ne prend jamais de decision a sa place. L'autorite decisionnelle reste exclusivement dans StrongFather.

**Regle WS-SF-02 : Adaptation obligatoire**

StrongFather DOIT adapter la severite de ses politiques selon les contraintes de WorrySentinel. Cette adaptation est non negociable en etat T2+.

**Regle WS-SF-03 : Observation sans ingererence**

WorrySentinel observe les decisions refusees par StrongFather pour alimenter son evaluation de l'etat global. Cette observation n'interfere jamais avec le processus decisionnel de StrongFather.

### 4.2 Separation des responsabilites

| Responsabilite | WorrySentinel | StrongFather |
|----------------|---------------|--------------|
| **Definir les niveaux de securite** | âœ… Autorite | âŒ Consommateur |
| **Gouverner les etats de confiance** | âœ… Autorite | âŒ Consommateur |
| **Evaluer les intentions** | âŒ Jamais | âœ… Autorite |
| **Produire des decisions** | âŒ Jamais | âœ… Autorite |
| **Appliquer des politiques** | âŒ Jamais | âœ… Autorite |
| **Definir la severite requise** | âœ… Autorite | âŒ Consommateur |
| **Adapter la severite des politiques** | âŒ Jamais | âœ… Execution |
| **Observer les decisions refusees** | âœ… Observateur | âœ… Source |

**Regle WS-SF-04 : Aucun chevauchement decisif**

WorrySentinel ne decide jamais si une intention est acceptee ou refusee. StrongFather ne decide jamais de l'etat de confiance du systeme. Aucun chevauchement d'autorite.

### 4.3 Hierarchie des strates

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 5 â€” Cores fonctionnels            â”‚
â”‚                                          â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  StrongFather                      â”‚ â”‚
â”‚  â”‚  (moteur de decision)              â”‚ â”‚
â”‚  â”‚  Decide si une intention est       â”‚ â”‚
â”‚  â”‚  acceptee/refusee/ambigue          â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                    â–²
                    â”‚ adapte severite selon
                    â”‚ contraintes gouvernance
                    â”‚
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 4 â€” Gouvernance securite          â”‚
â”‚                                          â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  WorrySentinel                     â”‚ â”‚
â”‚  â”‚  (pression verticale)             â”‚ â”‚
â”‚  â”‚  Gouverne niveaux securite        â”‚ â”‚
â”‚  â”‚  et etats de confiance            â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Principe :** WorrySentinel exerce une pression verticale sur StrongFather depuis la Strate 4. StrongFather reste l'autorite decisionnelle dans la Strate 5.

---

## 5. Flux d'interaction

### 5.1 Flux descendant : Gouvernance

WorrySentinel transmet a StrongFather le contexte de securite qui contraint ses decisions :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  WorrySentinel  â”‚
â”‚  (gouverne)     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
         â”‚  SecurityLevel (0-4)                      â”‚
         â”‚  TrustState (T0-T4)                       â”‚
         â”‚  SeverityConstraint                       â”‚
         â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  StrongFather   â”‚
â”‚  (adapte)       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Donnees transmises (WS â†’ SF) :**

| Element | Type | Description | Obligatoire |
|---------|------|-------------|-------------|
| `security_level` | `SecurityLevel` (0-4) | Niveau de securite applicable | âœ… Oui |
| `trust_state` | `TrustState` (T0-T4) | Etat de confiance du systeme | âœ… Oui |
| `severity_constraint` | `SeverityConstraint` | Severite requise pour les decisions | âœ… Oui |
| `active_restrictions` | `RestrictionSet` | Restrictions actives (capacites bloquees) | âŒ Optionnel |
| `timestamp` | `LogicalClock` | Horodatage logique | âœ… Oui |

**Structure SeverityConstraint :**

```typescript
interface SeverityConstraint {
  // Niveau de severite globale
  level: SeverityLevel;  // STANDARD | ELEVATED | STRICT | MAXIMUM | LOCKDOWN
  
  // Comportements specifiques
  require_explicit_justification: boolean;
  reject_ambiguous_intents: boolean;
  defer_critical_decisions: boolean;
  
  // Capacites a restreindre
  restricted_capabilities: CapabilityId[];
  
  // Actions specifiques
  force_tamr_on_critical: boolean;
}
```

### 5.2 Flux montant : Observation

StrongFather transmet a WorrySentinel les signaux de decisions refusees pour observation :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  WorrySentinel  â”‚
â”‚  (observe)      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â–²â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
         â”‚  DecisionRejectionSignal                  â”‚
         â”‚  (decisions refusees)                     â”‚
         â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  StrongFather   â”‚
â”‚  (signale)      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Donnees transmises (SF â†’ WS) :**

| Element | Type | Description | Obligatoire |
|---------|------|-------------|-------------|
| `signal_id` | `UUID` | Identifiant unique du signal | âœ… Oui |
| `intent_id` | `UUID` | Identifiant de l'intention refusee | âœ… Oui |
| `rejection_type` | `RejectionType` | Type de refus | âœ… Oui |
| `rejection_reason` | `string` | Raison du refus | âœ… Oui |
| `policies_applied` | `PolicyId[]` | Politiques appliquees | âœ… Oui |
| `severity_at_decision` | `SeverityLevel` | Severite active au moment de la decision | âœ… Oui |
| `timestamp` | `LogicalClock` | Horodatage logique | âœ… Oui |

**Types de refus transmis :**

| RejectionType | Description | Pertinence pour WS |
|---------------|-------------|-------------------|
| `POLICY_VIOLATION` | Violation de politique | Moyenne |
| `UNAUTHORIZED` | Autorite insuffisante | Haute |
| `AMBIGUOUS` | Intention ambigue | Moyenne |
| `SECURITY_LEVEL_EXCEEDED` | Niveau de securite depasse | Haute |
| `TRUST_STATE_RESTRICTED` | Restreint par etat de confiance | Haute |
| `CAPABILITY_BLOCKED` | Capacite bloquee | Moyenne |

**Regle WS-SF-05 : Signalement selectif**

StrongFather ne transmet pas toutes les decisions refusees a WorrySentinel. Seules les decisions significatives pour l'evaluation de l'etat de confiance sont transmises (refus pour raisons de securite, patterns suspects).

---

## 6. Impact de la gouvernance sur StrongFather

### 6.1 Matrice severite par etat de confiance

| Etat de confiance | Severite | Impact sur les decisions StrongFather |
|-------------------|----------|---------------------------------------|
| **T0 (Normal)** | STANDARD | Decisions normales, evaluation standard des politiques |
| **T1 (Instable)** | ELEVATED | Logging renforce, seuils de refus abaisses, justifications plus detaillees |
| **T2 (Degrade)** | STRICT | Decisions plus strictes, capacites non essentielles refusees automatiquement |
| **T3 (Restreint)** | MAXIMUM | Decisions critiques â†’ AMBIGUE / DIFFEREE, TAMR requis pour override |
| **T4 (Bloque)** | LOCKDOWN | Plus aucune decision operationnelle autorisee, mode diagnostic uniquement |

### 6.2 Comportements par etat de confiance

**T0 â€” Normal (Severite STANDARD)**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Comportement StrongFather en T0                                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ â€¢ Evaluation normale des intentions                                  â”‚
â”‚ â€¢ Politiques appliquees avec seuils standards                       â”‚
â”‚ â€¢ Toutes les capacites disponibles                                  â”‚
â”‚ â€¢ Decisions acceptees/refusees/ambigues selon politiques            â”‚
â”‚ â€¢ Logging standard                                                   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**T1 â€” Instable (Severite ELEVATED)**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Comportement StrongFather en T1                                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ â€¢ Logging renforce sur toutes les decisions                         â”‚
â”‚ â€¢ Seuils de refus legerement abaisses (plus de vigilance)           â”‚
â”‚ â€¢ Justifications plus detaillees requises                           â”‚
â”‚ â€¢ Toutes les capacites encore disponibles                           â”‚
â”‚ â€¢ Alertes sur patterns anormaux de refus                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**T2 â€” Degrade (Severite STRICT)**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Comportement StrongFather en T2                                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ â€¢ Decisions plus strictes sur toutes les intentions                 â”‚
â”‚ â€¢ Capacites non essentielles refusees automatiquement               â”‚
â”‚ â€¢ Intentions ambigues systematiquement refusees                     â”‚
â”‚ â€¢ Alertes transmises a WorrySentinel                                â”‚
â”‚ â€¢ Seuils de refus significativement abaisses                        â”‚
â”‚ â€¢ Historique detaille de toutes les decisions                       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**T3 â€” Restreint (Severite MAXIMUM)**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Comportement StrongFather en T3                                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ â€¢ Decisions critiques â†’ AMBIGUE ou DIFFEREE                         â”‚
â”‚ â€¢ TAMR requis pour tout override de decision                        â”‚
â”‚ â€¢ Seules les capacites essentielles disponibles                     â”‚
â”‚ â€¢ Intentions non essentielles refusees systematiquement             â”‚
â”‚ â€¢ Chaque decision tracee pour audit complet                         â”‚
â”‚ â€¢ Mode "fail-closed" : en cas de doute, refuser                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**T4 â€” Bloque (Severite LOCKDOWN)**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Comportement StrongFather en T4                                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ â€¢ Plus aucune decision operationnelle autorisee                     â”‚
â”‚ â€¢ Toutes les intentions â†’ REFUSEE (sauf diagnostic)                 â”‚
â”‚ â€¢ Mode diagnostic uniquement                                         â”‚
â”‚ â€¢ TAMR obligatoire pour toute action                                 â”‚
â”‚ â€¢ Etat lisible, sortie propre possible                              â”‚
â”‚ â€¢ Aucune capacite disponible                                         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 6.3 Impact du niveau de securite (0-4)

Le niveau de securite (0-4) defini par WorrySentinel s'ajoute a l'etat de confiance (T0-T4) :

| Niveau securite | Impact sur les decisions |
|-----------------|--------------------------|
| **Niveau 0 â€” Public** | Contraintes minimales, politiques souples |
| **Niveau 1 â€” Standard** | Contraintes de base, verifications standards |
| **Niveau 2 â€” Sensitive** | Contraintes renforcees, verifications supplementaires |
| **Niveau 3 â€” Critical** | Contraintes strictes, justification obligatoire |
| **Niveau 4 â€” Highest** | Contraintes maximales, validation multi-niveaux |

**Combinaison niveau + etat :**

La severite finale est le maximum entre la severite de l'etat de confiance et celle du niveau de securite. Une intention de niveau 4 en etat T0 sera quand meme traitee avec une severite elevee.

---

## 7. Regles de collaboration

### 7.1 Regles du cote WorrySentinel

| ID | Regle |
|----|-------|
| **COL-SF-1** | WorrySentinel ne prend jamais de decision a la place de StrongFather |
| **COL-SF-2** | WorrySentinel fournit le contexte de securite sans imposer de decision specifique |
| **COL-SF-3** | WorrySentinel observe les decisions refusees sans interferer avec le processus |
| **COL-SF-4** | WorrySentinel ne peut pas forcer StrongFather a accepter une intention |
| **COL-SF-5** | WorrySentinel gouverne la severite, pas les decisions individuelles |

### 7.2 Regles du cote StrongFather

| ID | Regle |
|----|-------|
| **COL-WS-1** | StrongFather adapte sa severite selon les contraintes de WorrySentinel |
| **COL-WS-2** | StrongFather ne peut pas ignorer un etat de confiance T3+ |
| **COL-WS-3** | StrongFather transmet les signaux de decisions refusees pertinents a WorrySentinel |
| **COL-WS-4** | StrongFather ne modifie jamais l'etat de confiance (responsabilite de WorrySentinel) |
| **COL-WS-5** | StrongFather applique les restrictions de capacites imposees par WorrySentinel |

---

## 8. Protocole d'echange

### 8.1 Reception du contexte de securite

StrongFather recoit le contexte de securite de WorrySentinel selon le format standardise :

**Structure de requete contexte (WS â†’ SF) :**

```typescript
interface SecurityContextUpdate {
  // Identification
  context_id: UUID;
  
  // Contexte de securite
  security_level: SecurityLevel;     // 0-4
  trust_state: TrustState;           // T0-T4
  severity_constraint: SeverityConstraint;
  
  // Restrictions actives
  restricted_capabilities: CapabilityId[];
  blocked_capabilities: CapabilityId[];
  
  // Metadata
  effective_from: LogicalClock;
  valid_until?: LogicalClock;  // Optionnel, si absent = jusqu'a mise a jour
  
  // Tracabilite
  source: "WorrySentinel";
  timestamp: LogicalClock;
}
```

**Regle WS-SF-PROT-01 : Mise a jour atomique**

La mise a jour du contexte de securite est atomique. StrongFather applique le nouveau contexte immediatement apres reception.

**Regle WS-SF-PROT-02 : Contexte toujours disponible**

StrongFather doit toujours avoir un contexte de securite valide. En cas d'absence (demarrage, erreur), le contexte par defaut est T0/Niveau1/STANDARD.

### 8.2 Transmission des signaux de refus

**Structure de signal de refus (SF â†’ WS) :**

```typescript
interface DecisionRejectionSignal {
  // Identification
  signal_id: UUID;
  intent_id: UUID;
  
  // Details du refus
  rejection_type: RejectionType;
  rejection_reason: string;
  
  // Contexte de decision
  policies_applied: PolicyId[];
  severity_at_decision: SeverityLevel;
  security_context_id: UUID;  // Reference au contexte actif
  
  // Classification
  security_relevant: boolean;     // Pertinent pour evaluation securite
  pattern_contribution: boolean;  // Contribue a detection de patterns
  
  // Metadata
  timestamp: LogicalClock;
}
```

**Regle WS-SF-PROT-03 : Transmission asynchrone**

Les signaux de refus sont transmis de maniere asynchrone. Ils n'impactent pas le temps de decision de StrongFather.

**Regle WS-SF-PROT-04 : Filtrage pre-transmission**

StrongFather filtre les signaux avant transmission. Seuls les signaux marques `security_relevant: true` ou `pattern_contribution: true` sont transmis a WorrySentinel.

---

## 9. Invariants de l'integration

### 9.1 Invariants de separation

**INV-WS-SF-1 : Separation de l'autorite decisionnelle**

WorrySentinel ne possede jamais d'autorite sur les decisions individuelles de StrongFather. L'autorite de decision reste exclusivement dans StrongFather.

**INV-WS-SF-2 : Separation de l'autorite de gouvernance**

StrongFather ne possede jamais d'autorite sur les niveaux de securite ou les etats de confiance. La gouvernance de securite reste exclusivement dans WorrySentinel.

### 9.2 Invariants de comportement

**INV-WS-SF-3 : Adaptation obligatoire en T2+**

StrongFather DOIT adapter sa severite selon les contraintes de WorrySentinel en etat T2 ou superieur. Cette adaptation est non negociable.

**INV-WS-SF-4 : Observation non interferente**

L'observation des decisions refusees par WorrySentinel n'interfere jamais avec le processus decisionnel de StrongFather.

**INV-WS-SF-5 : Blocage total en T4**

En etat T4 (Bloque), StrongFather doit refuser toutes les decisions operationnelles. Seul le mode diagnostic est autorise.

### 9.3 Invariants de coherence

**INV-WS-SF-6 : Coherence du contexte**

Le contexte de securite applique par StrongFather est toujours coherent avec le contexte gouverne par WorrySentinel. Aucune desynchronisation n'est autorisee.

**INV-WS-SF-7 : Tracabilite complete**

Toute interaction entre WorrySentinel et StrongFather est tracable avec contexte complet. Le journal contient toutes les informations necessaires pour reconstruire la sequence d'evenements.

---

## 10. Garanties de l'integration

### 10.1 Garantie de gouvernance

**Engagement :** WorrySentinel fournit toujours un contexte de securite valide a StrongFather. Le contexte est mis a jour en temps reel lors des changements d'etat.

### 10.2 Garantie d'adaptation

**Engagement :** StrongFather adapte toujours sa severite selon les contraintes de WorrySentinel. L'adaptation est immediate et atomique.

### 10.3 Garantie de non-interference

**Engagement :** WorrySentinel n'interfere jamais avec les decisions individuelles de StrongFather. L'observation est passive et sans effet de bord.

### 10.4 Garantie de tracabilite

**Engagement :** Toute interaction entre WorrySentinel et StrongFather est tracable de bout en bout. Le journal contient le contexte complet de chaque echange.

### 10.5 Garantie de disponibilite locale

**Engagement :** L'integration fonctionne sans dependance externe (conformite LOI-1). WorrySentinel et StrongFather operent localement.

---

## 11. Gestion des erreurs

### 11.1 Erreur de contexte invalide

**Scenario :** WorrySentinel transmet un contexte de securite invalide ou mal forme.

**Traitement :**
1. StrongFather rejette le contexte invalide
2. StrongFather conserve le dernier contexte valide
3. StrongFather signale l'erreur (pour audit)
4. L'operation de decision continue avec le contexte precedent

### 11.2 Absence de contexte

**Scenario :** StrongFather n'a pas recu de contexte de securite (demarrage initial, perte de communication).

**Traitement :**
1. StrongFather applique le contexte par defaut : T0/Niveau1/STANDARD
2. StrongFather signale l'absence de contexte (pour audit)
3. L'operation de decision continue avec le contexte par defaut
4. Des que WorrySentinel transmet un contexte, il est applique immediatement

### 11.3 Signal de refus non transmissible

**Scenario :** Un signal de refus ne peut pas etre transmis a WorrySentinel.

**Traitement :**
1. StrongFather conserve le signal en file d'attente locale
2. StrongFather tente une retransmission ulterieure
3. Apres N echecs, le signal est archive localement avec flag "non transmis"
4. L'operation de decision n'est jamais bloquee par un echec de transmission

---

## 12. Exemples d'interaction

### 12.1 Transition vers etat T2

**Scenario :** Le systeme detecte des anomalies persistantes. WorrySentinel passe de T1 a T2.

**Flux :**

```
1. WorrySentinel detecte la necessite de transition T1 â†’ T2
2. WorrySentinel met a jour l'etat de confiance : T2
3. WorrySentinel transmet le nouveau contexte a StrongFather :
   {
     security_level: 2,
     trust_state: "T2",
     severity_constraint: {
       level: "STRICT",
       require_explicit_justification: true,
       reject_ambiguous_intents: true,
       defer_critical_decisions: false,
       restricted_capabilities: ["CAP-SENSITIVE-001", "CAP-SENSITIVE-002"],
       force_tamr_on_critical: false
     }
   }
4. StrongFather recoit le contexte et l'applique immediatement
5. StrongFather adapte sa severite : politiques plus strictes
6. Les intentions suivantes sont evaluees avec severite STRICT
```

### 12.2 Decision refusee en T2

**Scenario :** En etat T2, StrongFather refuse une intention pour raison de securite.

**Flux :**

```
1. StrongFather recoit une intention :
   {
     intent_id: "int-001",
     action: "CREATE",
     capability_required: "CAP-SENSITIVE-001"
   }
2. StrongFather evalue l'intention avec severite STRICT :
   - Capacite CAP-SENSITIVE-001 dans restricted_capabilities
   - Decision : REFUSEE
3. StrongFather produit la decision :
   {
     intent_id: "int-001",
     result: "REJECTED",
     reason: "Capacite restreinte en etat T2"
   }
4. StrongFather transmet le signal a WorrySentinel :
   {
     signal_id: "sig-001",
     intent_id: "int-001",
     rejection_type: "CAPABILITY_BLOCKED",
     rejection_reason: "Capacite CAP-SENSITIVE-001 restreinte en etat T2",
     severity_at_decision: "STRICT",
     security_relevant: true,
     pattern_contribution: false
   }
5. WorrySentinel observe le signal pour correlation avec autres signaux
```

### 12.3 Decision en T4 (Lockdown)

**Scenario :** En etat T4, toutes les decisions operationnelles sont refusees.

**Flux :**

```
1. Contexte actif : T4/LOCKDOWN
2. StrongFather recoit n'importe quelle intention operationnelle
3. StrongFather applique la regle T4 : REFUS systematique
4. StrongFather produit la decision :
   {
     intent_id: "int-002",
     result: "REJECTED",
     reason: "Systeme en etat T4 (Bloque) - Aucune decision operationnelle autorisee"
   }
5. Le signal n'est pas transmis (volume trop important en T4)
6. Seul TAMR peut debloquer la situation
```

---

## 13. Conformite aux invariants FONDATION

### 13.1 Respect des invariants WorrySentinel

| Invariant | Statut | Justification |
|-----------|--------|---------------|
| **INV-WS-1** | âœ… Conforme | WorrySentinel n'implemente aucun controle de securite |
| **INV-WS-2** | âœ… Conforme | WorrySentinel n'execute aucune action |
| **INV-WS-3** | âœ… Conforme | WorrySentinel n'accede a aucune donnee persistee |
| **INV-WS-4** | âœ… Conforme | WorrySentinel ne modifie pas l'etat de StrongFather |
| **INV-WS-5** | âœ… Conforme | Aucune logique temporelle technique |
| **INV-WS-6** | âœ… Conforme | Zero-trust respecte |
| **INV-WS-7** | âœ… Conforme | Gouvernance explicite (contexte structure) |
| **INV-WS-8** | âœ… Conforme | Tracabilite complete de tous les echanges |

### 13.2 Respect des invariants StrongFather

| Invariant | Statut | Justification |
|-----------|--------|---------------|
| **INV-SF-1** | âœ… Conforme | StrongFather n'execute aucune action |
| **INV-SF-2** | âœ… Conforme | StrongFather n'accede a aucune donnee persistee |
| **INV-SF-3** | âœ… Conforme | StrongFather ne modifie pas l'etat du systeme |
| **INV-SF-4** | âœ… Conforme | Aucune logique temporelle technique |
| **INV-SF-5** | âœ… Conforme | Zero-trust respecte |
| **INV-SF-6** | âœ… Conforme | Decisions non ambigues |
| **INV-SF-7** | âœ… Conforme | Politiques explicites |
| **INV-SF-8** | âœ… Conforme | Tracabilite complete |

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il etablit l'interface et le protocole entre WorrySentinel et StrongFather.

Toute implementation de l'integration entre WorrySentinel et StrongFather doit respecter ce contrat. Toute violation entraine un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**Dependances :**
- WorrySentinel - Documentation Fondatrice v1.2 (Section 9.2)
- WorrySentinel - Core Interaction Contract v1.0 (Section 5.2)
- StrongFather - Documentation Fondatrice v1.5
- Miyukini Conceptual References - Security Levels
- Miyukini Conceptual References - Integrity Degradation System
- Miyukini Conceptual References - Lois Autonomie Systeme (LOI-1)

---

## 15. Mini log de generation

### Decision editoriale E1 : Perspective du document

**Decision prise :** Ce document adopte la perspective de WorrySentinel (gouverneur) et non celle de StrongFather (decisionnaire). Il documente comment WorrySentinel contraint StrongFather.

**Application :** Les sections sont orientees gouvernance, contexte de securite, et observation des decisions.

### Decision editoriale E2 : Symetrie avec Core Interaction Contract

**Decision prise :** Ce document est coherent et symetrique avec le [WorrySentinel - Core Interaction Contract](../../architecture/WorrySentinel%20-%20Core%20Interaction%20Contract.md) Section 5.2. Les formats et protocoles sont identiques.

**Application :** Les contrats d'interface et les regles de collaboration reprennent exactement les definitions du Core Interaction Contract.

### Decision editoriale E3 : Structure similaire au modele LogisticsSteward

**Decision prise :** Ce document suit la structure du contrat d'integration StrongFather-LogisticsSteward pour assurer une coherence documentaire.

**Application :** Sections organisees selon le meme pattern : Contexte, Portee, Principe fondamental, Nature de la relation, Flux, Impact, Regles, Protocole, Invariants, Garanties, Erreurs, Exemples, Conformite, Statut.

### Verification de coherence

**Verification effectuee :**
- âœ… Coherence avec WorrySentinel - Documentation Fondatrice : Section 9.2 respectee
- âœ… Coherence avec WorrySentinel - Core Interaction Contract : Section 5.2 respectee
- âœ… Coherence avec StrongFather - Documentation Fondatrice : Relation complementaire confirmee
- âœ… Respect INV-WS-1 : Aucune autorite sur l'implementation
- âœ… Respect INV-WS-2 : Aucune autorite sur l'execution
- âœ… Respect INV-WS-3 : Aucune autorite sur la persistance
- âœ… Respect INV-SF-1 : StrongFather n'execute pas
- âœ… Respect INV-SF-2 : StrongFather ne persiste pas
- âœ… Conformite LOI-1 : Aucune dependance externe
- âœ… Separation des responsabilites preservee

**Conclusion :** Aucune contradiction detectee. Le document est coherent avec la documentation fondatrice des deux cores.

---

*Aucune autre erreur, warning, ou ambiguite rencontree lors de la redaction de ce document.*

