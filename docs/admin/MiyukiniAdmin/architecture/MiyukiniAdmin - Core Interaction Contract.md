# MiyukiniAdmin â€” Core Interaction Contract

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
MiyukiniAdmin â”€â”€â–¶ BondingBrother â”€â”€â–¶ Core
     â”‚                  â”‚
     â”‚â—€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚â—€â”€â”€â”€ Reponse
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
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ AdminRequest                                                 â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ request_id: UUID           # Identifiant unique              â”‚
â”‚ timestamp: DateTime        # Horodatage                      â”‚
â”‚ operator_id: UUID          # Identite operateur              â”‚
â”‚ capability: String         # Capacite invoquee               â”‚
â”‚ parameters: Map            # Parametres de la requete        â”‚
â”‚ justification: String?     # Justification (si requise)      â”‚
â”‚ session_context: Context   # Contexte de session             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 4.3 Protocole de Reponse

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ AdminResponse                                                â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ request_id: UUID           # Reference a la requete          â”‚
â”‚ timestamp: DateTime        # Horodatage reponse              â”‚
â”‚ status: ResponseStatus     # SUCCESS | ERROR | DENIED        â”‚
â”‚ data: Any?                 # Donnees de reponse              â”‚
â”‚ error: Error?              # Erreur si applicable            â”‚
â”‚ audit_ref: UUID            # Reference audit log             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
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
     â”‚                             â”‚                          â”‚
     â”‚â”€â”€AdminRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚  (capability: security.write)â”‚                          â”‚
     â”‚                             â”‚â”€â”€DecisionRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                             â”‚                          â”‚
     â”‚                             â”‚â—€â”€DecisionResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                             â”‚  (APPROVED/DENIED)        â”‚
     â”‚â—€â”€AdminResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚                             â”‚                          â”‚
```

### 5.4 Donnees de Validation

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ StrongFather Validation Context                              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ action_type: String        # Type d'action admin             â”‚
â”‚ current_security_level: 0-4 # Niveau actuel                  â”‚
â”‚ requested_change: Any      # Changement demande              â”‚
â”‚ justification: String      # Justification obligatoire       â”‚
â”‚ operator_role: Role        # Role de l'operateur             â”‚
â”‚ system_state: State        # Etat actuel du systeme          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
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
     â”‚                             â”‚                          â”‚
     â”‚â”€â”€AdminRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚  (capability: db.read)       â”‚                          â”‚
     â”‚                             â”‚â”€â”€DataRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                             â”‚                          â”‚
     â”‚                             â”‚â—€â”€DataResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                             â”‚  (data)                   â”‚
     â”‚â—€â”€AdminResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚                             â”‚                          â”‚
```

### 6.4 Flux Mode Recovery (Cas Extreme)

**Conditions cumulatives verifiees avant acces direct :**

```
MiyukiniAdmin                BondingBrother         StrongFather    KindMother
     â”‚                             â”‚                     â”‚              â”‚
     â”‚â”€â”€RecoveryRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                     â”‚              â”‚
     â”‚  (conditions cumulatives)    â”‚                     â”‚              â”‚
     â”‚                             â”‚                     â”‚              â”‚
     â”‚                             â”‚â”€â”€VerifyConditionsâ”€â”€â”€â–¶â”‚              â”‚
     â”‚                             â”‚                     â”‚              â”‚
     â”‚                             â”‚â—€â”€ConditionsValidâ”€â”€â”€â”€â”‚              â”‚
     â”‚                             â”‚                     â”‚              â”‚
     â”‚                             â”‚â”€â”€NotifyRecoveryâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                             â”‚  (blocage Operateurs)                â”‚
     â”‚                             â”‚                     â”‚              â”‚
     â”‚â—€â”€RecoveryGrantedâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                     â”‚              â”‚
     â”‚                             â”‚                     â”‚              â”‚
     â”‚â•â•â•â•â•â•â•â• ACCES DIRECT DB (temporaire) â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â”‚
     â”‚                             â”‚                     â”‚              â”‚
     â”‚â”€â”€EndRecoveryâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                     â”‚              â”‚
     â”‚                             â”‚â”€â”€Revalidationâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                             â”‚                     â”‚              â”‚
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
     â”‚                             â”‚                          â”‚
     â”‚â”€â”€AdminRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                          â”‚
     â”‚  (capability: metrics.system)â”‚                          â”‚
     â”‚                             â”‚â”€â”€MetricsRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                             â”‚                          â”‚
     â”‚                             â”‚â—€â”€MetricsResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                             â”‚  (cpu, ram, disk, net)    â”‚
     â”‚â—€â”€AdminResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                          â”‚
     â”‚                             â”‚                          â”‚
```

### 7.4 Structure des Metriques

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ SystemMetrics                                                â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ timestamp: DateTime                                          â”‚
â”‚ cpu_usage: Percentage                                        â”‚
â”‚ ram_usage: Percentage                                        â”‚
â”‚ disk_usage: Percentage                                       â”‚
â”‚ network_io: NetworkStats                                     â”‚
â”‚ active_connections: Integer                                  â”‚
â”‚ trust_level: T0-T4                                           â”‚
â”‚ security_level: 0-4                                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
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
     â”‚                         â”‚                    â”‚                  â”‚
     â”‚â”€â”€ChangeSecurityLevelâ”€â”€â”€â”€â–¶â”‚                    â”‚                  â”‚
     â”‚  (new_level, justif)     â”‚                    â”‚                  â”‚
     â”‚                         â”‚                    â”‚                  â”‚
     â”‚                         â”‚â”€â”€ValidateChangeâ”€â”€â”€â”€â–¶â”‚                  â”‚
     â”‚                         â”‚                    â”‚                  â”‚
     â”‚                         â”‚â—€â”€Approvedâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                  â”‚
     â”‚                         â”‚                    â”‚                  â”‚
     â”‚                         â”‚â”€â”€ApplyChangeâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                         â”‚                    â”‚                  â”‚
     â”‚                         â”‚â—€â”€ChangeAppliedâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                         â”‚                    â”‚                  â”‚
     â”‚â—€â”€Successâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                    â”‚                  â”‚
     â”‚                         â”‚                    â”‚                  â”‚
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
     â”‚                             â”‚                      â”‚
     â”‚â”€â”€AdminRequestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚                      â”‚
     â”‚                             â”‚â”€â”€Requestâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚
     â”‚                             â”‚                      â”‚
     â”‚                             â”‚â—€â”€Errorâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
     â”‚                             â”‚                      â”‚
     â”‚                             â”‚  (Log erreur)         â”‚
     â”‚                             â”‚                      â”‚
     â”‚â—€â”€AdminResponseâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚                      â”‚
     â”‚   (status: ERROR)           â”‚                      â”‚
     â”‚   (error: details)          â”‚                      â”‚
```

---

## 12. Documents Associes

- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Architecture & Flows](./MiyukiniAdmin%20-%20Architecture%20&%20Flows.md)
- [BondingBrother - Documentation Fondatrice](..//..//..//cores//BondingBrother//foundation//BondingBrother%20-%20Documentation%20Fondatrice.md)
- [StrongFather - Documentation Fondatrice](..//..//..//cores//StrongFather//foundation//StrongFather%20-%20Documentation%20Fondatrice.md)
- [KindMother - Documentation Fondatrice](..//..//..//cores//KindMother//foundation//KindMother%20-%20Documentation%20Fondatrice.md)
- [CaringNanny - Documentation Fondatrice](..//..//..//cores//CaringNanny//foundation//Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [WorrySentinel - Documentation Fondatrice](..//..//..//cores//WorrySentinel//foundation//WorrySentinel%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference

