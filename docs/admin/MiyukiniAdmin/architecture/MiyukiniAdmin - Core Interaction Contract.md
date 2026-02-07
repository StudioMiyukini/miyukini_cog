# MiyukiniAdmin — Core Interaction Contract

## 1. Contexte

Ce document definit le contrat d'interaction entre MiyukiniAdmin et les cores du systeme Miyukini. Il specifie les protocoles de communication, les capacites accessibles, et les contraintes d'interaction.

MiyukiniAdmin interagit avec les cores **exclusivement via BondingBrother**, sans exception.

## 2. Portee / Scope

Ce document definit :
- Le modele d'interaction avec chaque core
- Les capacites reservees accessibles
- Les protocoles de requete/reponse
- Les contraintes et invariants d'interaction

Ce document **ne couvre pas** :
- L'implementation technique des interactions
- Les details de serialisation/transport
- La securite des communications (voir Security contracts)

---

## 3. Principe Fondamental

### 3.1 Mediation Obligatoire

> **Toute interaction entre MiyukiniAdmin et les cores passe par BondingBrother.**

```
MiyukiniAdmin ──▶ BondingBrother ──▶ Core
     │                  │
     │◀─────────────────│◀─── Reponse
```

**Aucun acces direct aux cores n'est autorise.**

### 3.2 Invariant d'Interaction

| Code | Invariant |
|------|-----------|
| **INV-CORE-1** | Toute interaction passe par BondingBrother |
| **INV-CORE-2** | Toute interaction est tracee |
| **INV-CORE-3** | Toute interaction est authentifiee |
| **INV-CORE-4** | Les capacites reservees sont explicitement declarees |

---

## 4. Interaction avec BondingBrother

### 4.1 Role de BondingBrother

**BondingBrother est le point d'acces exclusif pour MiyukiniAdmin.**

| Responsabilite | Description |
|----------------|-------------|
| **Mediation** | Traduit les requetes admin en requetes cores |
| **Validation** | Verifie les permissions et capacites |
| **Tracabilite** | Journalise toutes les interactions |
| **Routage** | Dirige vers le core approprie |

### 4.2 Protocole de Requete

```
┌─────────────────────────────────────────────────────────────┐
│ AdminRequest                                                 │
├─────────────────────────────────────────────────────────────┤
│ request_id: UUID           # Identifiant unique              │
│ timestamp: DateTime        # Horodatage                      │
│ operator_id: UUID          # Identite operateur              │
│ capability: String         # Capacite invoquee               │
│ parameters: Map            # Parametres de la requete        │
│ justification: String?     # Justification (si requise)      │
│ session_context: Context   # Contexte de session             │
└─────────────────────────────────────────────────────────────┘
```

### 4.3 Protocole de Reponse

```
┌─────────────────────────────────────────────────────────────┐
│ AdminResponse                                                │
├─────────────────────────────────────────────────────────────┤
│ request_id: UUID           # Reference a la requete          │
│ timestamp: DateTime        # Horodatage reponse              │
│ status: ResponseStatus     # SUCCESS | ERROR | DENIED        │
│ data: Any?                 # Donnees de reponse              │
│ error: Error?              # Erreur si applicable            │
│ audit_ref: UUID            # Reference audit log             │
└─────────────────────────────────────────────────────────────┘
```

---

## 5. Interaction avec StrongFather

### 5.1 Role dans les Interactions

**StrongFather valide les actions administratives.**

| Question | Description |
|----------|-------------|
| "Cette action admin est-elle autorisee ?" | Validation des operations critiques |
| "Le changement de securite est-il justifie ?" | Approbation changements niveau |
| "L'operation de maintenance est-elle valide ?" | Validation maintenance |

### 5.2 Capacites Liees a StrongFather

| Capacite | Description | Validation SF |
|----------|-------------|---------------|
| **admin.security.level.write** | Changement niveau securite | Obligatoire |
| **admin.db.maintenance** | Operations maintenance | Obligatoire |
| **admin.db.recovery** | Acces direct DB | Obligatoire + Conditions |
| **admin.operators.isolate** | Isolation d'un Operateur | Obligatoire |

### 5.3 Flux de Validation

```
MiyukiniAdmin                BondingBrother              StrongFather
     │                             │                          │
     │──AdminRequest───────────────▶│                          │
     │  (capability: security.write)│                          │
     │                             │──DecisionRequest─────────▶│
     │                             │                          │
     │                             │◀─DecisionResponse────────│
     │                             │  (APPROVED/DENIED)        │
     │◀─AdminResponse──────────────│                          │
     │                             │                          │
```

### 5.4 Donnees de Validation

```
┌─────────────────────────────────────────────────────────────┐
│ StrongFather Validation Context                              │
├─────────────────────────────────────────────────────────────┤
│ action_type: String        # Type d'action admin             │
│ current_security_level: 0-4 # Niveau actuel                  │
│ requested_change: Any      # Changement demande              │
│ justification: String      # Justification obligatoire       │
│ operator_role: Role        # Role de l'operateur             │
│ system_state: State        # Etat actuel du systeme          │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. Interaction avec KindMother

### 6.1 Role dans les Interactions

**KindMother gere l'acces aux donnees.**

| Question | Description |
|----------|-------------|
| "Quelles donnees sont accessibles ?" | Exploration tables |
| "Quel est l'etat de coherence ?" | Verification coherence |
| "Comment effectuer la maintenance ?" | Operations maintenance |

### 6.2 Capacites Liees a KindMother

| Capacite | Description | Mode |
|----------|-------------|------|
| **admin.db.read** | Lecture donnees | Lecture seule |
| **admin.db.stats** | Statistiques DB | Lecture seule |
| **admin.db.validate** | Validation coherence | Lecture seule |
| **admin.db.migrate** | Migration controlee | Ecriture controlee |
| **admin.db.repair** | Reparation controlee | Ecriture controlee |

### 6.3 Flux Lecture Donnees

```
MiyukiniAdmin                BondingBrother              KindMother
     │                             │                          │
     │──AdminRequest───────────────▶│                          │
     │  (capability: db.read)       │                          │
     │                             │──DataRequest─────────────▶│
     │                             │                          │
     │                             │◀─DataResponse────────────│
     │                             │  (data)                   │
     │◀─AdminResponse──────────────│                          │
     │                             │                          │
```

### 6.4 Flux Mode Recovery (Cas Extreme)

**Conditions cumulatives verifiees avant acces direct :**

```
MiyukiniAdmin                BondingBrother         StrongFather    KindMother
     │                             │                     │              │
     │──RecoveryRequest────────────▶│                     │              │
     │  (conditions cumulatives)    │                     │              │
     │                             │                     │              │
     │                             │──VerifyConditions───▶│              │
     │                             │                     │              │
     │                             │◀─ConditionsValid────│              │
     │                             │                     │              │
     │                             │──NotifyRecovery─────────────────────▶│
     │                             │  (blocage Operateurs)                │
     │                             │                     │              │
     │◀─RecoveryGranted────────────│                     │              │
     │                             │                     │              │
     │════════ ACCES DIRECT DB (temporaire) ════════════════════════════│
     │                             │                     │              │
     │──EndRecovery────────────────▶│                     │              │
     │                             │──Revalidation────────────────────────▶│
     │                             │                     │              │
```

---

## 7. Interaction avec CaringNanny

### 7.1 Role dans les Interactions

**CaringNanny expose l'etat du systeme.**

| Question | Description |
|----------|-------------|
| "Quel est l'etat global du systeme ?" | Sante generale |
| "Quelles sont les metriques actuelles ?" | CPU, RAM, disque, reseau |
| "Quels Operateurs sont actifs ?" | Etats des Operateurs |

### 7.2 Capacites Liees a CaringNanny

| Capacite | Description |
|----------|-------------|
| **admin.metrics.system** | Metriques systeme (CPU, RAM, etc.) |
| **admin.metrics.db** | Metriques base de donnees |
| **admin.metrics.operators** | Etats des Operateurs |
| **admin.metrics.latency** | Latence decisionnelle |
| **admin.metrics.health** | Sante globale |

### 7.3 Flux Monitoring

```
MiyukiniAdmin                BondingBrother              CaringNanny
     │                             │                          │
     │──AdminRequest───────────────▶│                          │
     │  (capability: metrics.system)│                          │
     │                             │──MetricsRequest──────────▶│
     │                             │                          │
     │                             │◀─MetricsResponse─────────│
     │                             │  (cpu, ram, disk, net)    │
     │◀─AdminResponse──────────────│                          │
     │                             │                          │
```

### 7.4 Structure des Metriques

```
┌─────────────────────────────────────────────────────────────┐
│ SystemMetrics                                                │
├─────────────────────────────────────────────────────────────┤
│ timestamp: DateTime                                          │
│ cpu_usage: Percentage                                        │
│ ram_usage: Percentage                                        │
│ disk_usage: Percentage                                       │
│ network_io: NetworkStats                                     │
│ active_connections: Integer                                  │
│ trust_level: T0-T4                                           │
│ security_level: 0-4                                          │
└─────────────────────────────────────────────────────────────┘
```

---

## 8. Interaction avec WorrySentinel

### 8.1 Role dans les Interactions

**WorrySentinel gere la securite.**

| Question | Description |
|----------|-------------|
| "Quel est le niveau de securite actuel ?" | Lecture niveau |
| "Comment changer le niveau ?" | Modification niveau |
| "Quels modes de degradation sont actifs ?" | Etat degradation |

### 8.2 Capacites Liees a WorrySentinel

| Capacite | Description | Contrainte |
|----------|-------------|------------|
| **admin.security.level.read** | Lecture niveau securite | Aucune |
| **admin.security.level.write** | Changement niveau | Validation SF + Justification |
| **admin.security.degradation.read** | Lecture modes degradation | Aucune |
| **admin.security.degradation.activate** | Activation mode | Validation SF |

### 8.3 Flux Changement Niveau Securite

```
MiyukiniAdmin            BondingBrother        StrongFather      WorrySentinel
     │                         │                    │                  │
     │──ChangeSecurityLevel────▶│                    │                  │
     │  (new_level, justif)     │                    │                  │
     │                         │                    │                  │
     │                         │──ValidateChange────▶│                  │
     │                         │                    │                  │
     │                         │◀─Approved──────────│                  │
     │                         │                    │                  │
     │                         │──ApplyChange───────────────────────────▶│
     │                         │                    │                  │
     │                         │◀─ChangeApplied────────────────────────│
     │                         │                    │                  │
     │◀─Success────────────────│                    │                  │
     │                         │                    │                  │
```

---

## 9. Catalogue des Capacites Reservees

### 9.1 Capacites Monitoring

| Capacite | Core cible | Description |
|----------|------------|-------------|
| `admin.metrics.system` | CaringNanny | Metriques systeme |
| `admin.metrics.db` | CaringNanny + KindMother | Metriques DB |
| `admin.metrics.operators` | CaringNanny | Etats Operateurs |
| `admin.metrics.latency` | CaringNanny | Latence decisionnelle |
| `admin.metrics.health` | CaringNanny | Sante globale |

### 9.2 Capacites Database

| Capacite | Core cible | Description |
|----------|------------|-------------|
| `admin.db.read` | KindMother | Lecture donnees |
| `admin.db.stats` | KindMother | Statistiques DB |
| `admin.db.validate` | KindMother | Validation coherence |
| `admin.db.migrate` | KindMother + StrongFather | Migration controlee |
| `admin.db.repair` | KindMother + StrongFather | Reparation controlee |
| `admin.db.recovery` | KindMother + StrongFather | Acces direct (recovery) |

### 9.3 Capacites Security

| Capacite | Core cible | Description |
|----------|------------|-------------|
| `admin.security.level.read` | WorrySentinel | Lecture niveau |
| `admin.security.level.write` | WorrySentinel + StrongFather | Changement niveau |
| `admin.security.degradation.read` | WorrySentinel | Lecture degradation |
| `admin.security.degradation.activate` | WorrySentinel + StrongFather | Activation mode |

### 9.4 Capacites Testing

| Capacite | Core cible | Description |
|----------|------------|-------------|
| `admin.tests.performance` | Multiple | Tests performance |
| `admin.tests.latency` | StrongFather | Tests latence decision |
| `admin.tests.coherence` | KindMother | Tests coherence DB |
| `admin.tests.compliance` | Multiple | Tests conformite |

### 9.5 Capacites Operators

| Capacite | Core cible | Description |
|----------|------------|-------------|
| `admin.operators.list` | CaringNanny | Liste Operateurs |
| `admin.operators.status` | CaringNanny | Statut Operateur |
| `admin.operators.isolate` | StrongFather | Isolation Operateur |
| `admin.operators.restore` | StrongFather | Restauration Operateur |

---

## 10. Contraintes d'Interaction

### 10.1 Contraintes Temporelles

| Contrainte | Description |
|------------|-------------|
| **Timeout requete** | 30 secondes max par defaut |
| **Timeout recovery** | Fenetre temporelle limitee (configurable) |
| **Frequence monitoring** | Max 1 requete / seconde par metrique |

### 10.2 Contraintes de Volume

| Contrainte | Description |
|------------|-------------|
| **Taille requete** | Max 1 MB |
| **Taille reponse** | Max 10 MB |
| **Resultats pagination** | Max 1000 items par page |

### 10.3 Contraintes de Securite

| Contrainte | Description |
|------------|-------------|
| **Authentification** | Obligatoire pour toute requete |
| **Justification** | Obligatoire pour actions critiques |
| **Validation SF** | Obligatoire pour modifications |

---

## 11. Gestion des Erreurs

### 11.1 Types d'Erreurs

| Code | Type | Description |
|------|------|-------------|
| `AUTH_001` | Authentication | Echec authentification |
| `PERM_001` | Permission | Permission insuffisante |
| `VALID_001` | Validation | Donnees invalides |
| `CORE_001` | Core Error | Erreur core cible |
| `TIMEOUT_001` | Timeout | Timeout requete |

### 11.2 Traitement des Erreurs

```
MiyukiniAdmin                BondingBrother              Core
     │                             │                      │
     │──AdminRequest───────────────▶│                      │
     │                             │──Request─────────────▶│
     │                             │                      │
     │                             │◀─Error───────────────│
     │                             │                      │
     │                             │  (Log erreur)         │
     │                             │                      │
     │◀─AdminResponse──────────────│                      │
     │   (status: ERROR)           │                      │
     │   (error: details)          │                      │
```

---

## 12. Documents Associes

- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Architecture & Flows](./MiyukiniAdmin%20-%20Architecture%20&%20Flows.md)
- [BondingBrother - Documentation Fondatrice](../../BondingBrother/foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [StrongFather - Documentation Fondatrice](../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)
- [KindMother - Documentation Fondatrice](../../KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [CaringNanny - Documentation Fondatrice](../../CaringNanny/foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [WorrySentinel - Documentation Fondatrice](../../WorrySentinel/WorrySentinel%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference
