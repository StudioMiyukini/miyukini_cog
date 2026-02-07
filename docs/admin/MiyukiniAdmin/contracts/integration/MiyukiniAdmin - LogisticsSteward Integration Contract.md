# MiyukiniAdmin — LogisticsSteward Integration Contract

## 1. Contexte

Ce document definit le contrat d'integration entre MiyukiniAdmin et **LogisticsSteward**. LogisticsSteward est le core responsable de la gouvernance de l'allocation, de la priorisation et de la limitation des ressources.

## 2. Portee / Scope

Ce document definit :
- Le role de LogisticsSteward pour MiyukiniAdmin
- Les regles de priorite applicables a MiyukiniAdmin
- Les protocoles d'exception
- Les quotas et limitations
- La tracabilite des demandes de ressources

Ce document **ne couvre pas** :
- L'implementation interne de LogisticsSteward
- Les autres integrations cores
- Les mecanismes techniques d'allocation (responsabilite du Kernel)

---

## 3. Role de LogisticsSteward

### 3.1 Gouverneur des Ressources pour MiyukiniAdmin

> **LogisticsSteward arbitre l'usage des ressources par MiyukiniAdmin selon des regles explicites.**

**Question fondamentale :**
> "MiyukiniAdmin a-t-il droit a ces ressources, a cette priorite, dans ce contexte ?"

### 3.2 Responsabilites

| Responsabilite | Description |
|----------------|-------------|
| **Arbitrage des priorites** | Decide du niveau de priorite de MiyukiniAdmin |
| **Gestion des quotas** | Applique les quotas definis pour l'administration |
| **Protocole d'exception** | Gere les demandes de privilege exceptionnel |
| **Protection du systeme** | Garantit que MiyukiniAdmin ne monopolise pas les ressources |

---

## 4. Regles de Priorite pour MiyukiniAdmin

### 4.1 Principe Fondamental

> **MiyukiniAdmin peut demander des priorites maximales, mais reste soumis a la gouvernance globale.**

MiyukiniAdmin n'est pas au-dessus de LogisticsSteward. Il peut demander des exceptions, pas les imposer.

### 4.2 Niveaux de Priorite Disponibles

| Niveau | Description | Conditions d'acces |
|--------|-------------|-------------------|
| **P0 - Critique** | Priorite maximale, ressources garanties | Protocole d'exception valide |
| **P1 - Haute** | Priorite elevee, preemption possible | Operations critiques (recovery, securite) |
| **P2 - Normale** | Priorite standard | Operations de monitoring et maintenance |
| **P3 - Basse** | Priorite reduite | Operations non urgentes (tests de charge) |

### 4.3 Priorite par Type d'Operation

| Operation | Priorite par defaut | Priorite maximale demandable |
|-----------|---------------------|------------------------------|
| Lecture metriques | P2 | P2 |
| Tests unitaires | P3 | P2 |
| Tests de charge | P3 | P2 |
| Changement niveau securite | P1 | P0 |
| Acces DB recovery | P1 | P0 |
| Isolation Operateur | P1 | P0 |
| Migration DB | P1 | P0 |

---

## 5. Protocole d'Exception

### 5.1 Conditions d'Activation

Le protocole d'exception permet a MiyukiniAdmin d'obtenir des privileges au-dela de la gouvernance standard.

**Conditions cumulatives requises :**

| Condition | Description |
|-----------|-------------|
| **Justification explicite** | Raison detaillee et documentee |
| **Validation StrongFather** | Decision approuvee par StrongFather |
| **Duree limitee** | Exception valide pour un temps defini |
| **Tracabilite complete** | Chaque action sous exception est journalisee |

### 5.2 Format de Demande d'Exception

```json
{
  "exception_request_id": "uuid-exception-001",
  "timestamp": "2026-01-28T12:00:00Z",
  "source": "miyukini_admin",
  "request": {
    "type": "PRIORITY_EXCEPTION",
    "requested_priority": "P0",
    "operation": "DB_RECOVERY",
    "capability": "admin.db.recovery"
  },
  "justification": {
    "reason": "Corruption DB detectee - Incident #INC-2026-0128",
    "urgency": "CRITICAL",
    "expected_duration_minutes": 30,
    "impact_assessment": "Service indisponible sans intervention"
  },
  "context": {
    "current_priority": "P1",
    "system_state": {
      "trust_level": "T3",
      "degradation_level": "D2",
      "active_operators": 5
    }
  }
}
```

### 5.3 Format de Reponse

```json
{
  "exception_request_id": "uuid-exception-001",
  "exception_id": "uuid-exception-response-001",
  "timestamp": "2026-01-28T12:00:01Z",
  "decision": "GRANTED",
  "granted_priority": "P0",
  "conditions": {
    "max_duration_minutes": 30,
    "resources_allocated": ["cpu_high", "memory_reserved"],
    "monitoring": "intensive"
  },
  "validity": {
    "expires_at": "2026-01-28T12:30:01Z",
    "renewable": false
  },
  "strongfather_decision_id": "uuid-sf-decision-001"
}
```

### 5.4 Decisions Possibles

| Decision | Description |
|----------|-------------|
| `GRANTED` | Exception accordee avec conditions |
| `DENIED` | Exception refusee |
| `PARTIAL` | Exception partiellement accordee |
| `DEFERRED` | Exception mise en attente |

---

## 6. Quotas et Limitations

### 6.1 Quotas par Defaut pour MiyukiniAdmin

| Ressource | Quota par defaut | Justification |
|-----------|------------------|---------------|
| **Operations simultanees** | 5 max | Eviter monopolisation |
| **Requetes DB/minute** | 100 | Proteger la base |
| **Tests de charge actifs** | 1 | Impact performance |
| **Exceptions actives** | 3 max | Maintenir gouvernance |

### 6.2 Quotas en Mode Exception

| Ressource | Quota exception | Conditions |
|-----------|-----------------|------------|
| **Operations simultanees** | 10 max | P0 ou P1 |
| **Requetes DB/minute** | 500 | Recovery uniquement |
| **Preemption Operateurs** | Oui | Validation SF requise |

---

## 7. Flux d'Interaction

### 7.1 Flux Standard (Sans Exception)

```
MiyukiniAdmin           BondingBrother          LogisticsSteward
     │                        │                        │
     │──ResourceRequest───────▶│                        │
     │  (priority: P2)         │                        │
     │                        │                        │
     │                        │──ArbitrationRequest────▶│
     │                        │                        │
     │                        │                        │  [Evaluation]
     │                        │                        │  - Quotas
     │                        │                        │  - Regles
     │                        │                        │  - Etat systeme
     │                        │                        │
     │                        │◀─ArbitrationDecision───│
     │                        │  (ALLOWED, priority: P2)│
     │                        │                        │
     │◀─ResourceResponse──────│                        │
     │  (granted)              │                        │
```

### 7.2 Flux avec Demande d'Exception

```
MiyukiniAdmin           BondingBrother          LogisticsSteward        StrongFather
     │                        │                        │                      │
     │──ExceptionRequest──────▶│                        │                      │
     │  (priority: P0)         │                        │                      │
     │                        │                        │                      │
     │                        │──ExceptionArbitration──▶│                      │
     │                        │                        │                      │
     │                        │                        │──ValidationRequest───▶│
     │                        │                        │                      │
     │                        │                        │◀─Decision─────────────│
     │                        │                        │  (APPROVED)           │
     │                        │                        │                      │
     │                        │◀─ExceptionGranted──────│                      │
     │                        │  (P0, conditions)       │                      │
     │                        │                        │                      │
     │◀─ExceptionResponse─────│                        │                      │
     │  (granted, validity)    │                        │                      │
```

### 7.3 Flux avec Refus

```
MiyukiniAdmin           BondingBrother          LogisticsSteward
     │                        │                        │
     │──ResourceRequest───────▶│                        │
     │  (exceeds quota)        │                        │
     │                        │                        │
     │                        │──ArbitrationRequest────▶│
     │                        │                        │
     │                        │◀─ArbitrationDecision───│
     │                        │  (DENIED)               │
     │                        │  (reason: quota_exceeded)│
     │                        │                        │
     │◀─ResourceResponse──────│                        │
     │  (denied, reason)       │                        │
```

---

## 8. Degradation et MiyukiniAdmin

### 8.1 Comportement par Niveau de Degradation

| Niveau | Impact sur MiyukiniAdmin |
|--------|-------------------------|
| **D0 - Normal** | Toutes operations disponibles |
| **D1 - Prudent** | Tests de charge desactives |
| **D2 - Restreint** | Operations non critiques limitees |
| **D3 - Critique** | Monitoring + recovery uniquement |
| **D4 - Survie** | Recovery uniquement |

### 8.2 Priorite en Degradation

En mode degrade, MiyukiniAdmin conserve :
- La capacite de monitoring (lecture seule)
- La capacite de recovery (avec exception)
- La priorite P1 pour les operations critiques

---

## 9. Tracabilite et Audit

### 9.1 Donnees Tracees

| Champ | Description |
|-------|-------------|
| `request_id` | ID de la demande |
| `timestamp` | Horodatage |
| `operation` | Type d'operation |
| `requested_priority` | Priorite demandee |
| `granted_priority` | Priorite accordee |
| `decision` | ALLOWED/DENIED/EXCEPTION |
| `quota_status` | Etat des quotas |
| `exception_id` | ID exception si applicable |

### 9.2 Retention

| Type | Retention |
|------|-----------|
| Demandes standards | 1 an |
| Exceptions accordees | 2 ans |
| Exceptions refusees | 2 ans |
| Operations recovery | Permanent |

---

## 10. Gestion des Erreurs

### 10.1 Erreurs Possibles

| Code | Description | Action |
|------|-------------|--------|
| `LS_001` | LogisticsSteward indisponible | Retry avec backoff |
| `LS_002` | Quota depasse | Attendre ou demander exception |
| `LS_003` | Priorite refusee | Justifier ou reduire priorite |
| `LS_004` | Exception expiree | Renouveler si possible |

### 10.2 Fallback

**En cas d'indisponibilite LogisticsSteward :**
- Operations critiques : Mode degrade automatique (P2 max)
- Exceptions : REFUSEES par defaut
- Monitoring lecture seule : AUTORISE

---

## 11. Integration UI

### 11.1 Indicateur de Quota

```
┌─────────────────────────────────────────────────────────────┐
│ Ressources MiyukiniAdmin                                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Operations simultanees:  [████████░░] 4/5                  │
│ Requetes DB/minute:      [███░░░░░░░] 32/100               │
│ Exceptions actives:      [██░░░░░░░░] 1/3                  │
│                                                             │
│ Priorite actuelle: P2 (Normale)                            │
│ Niveau degradation: D0 (Normal)                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 11.2 Dialogue de Demande d'Exception

```
┌─────────────────────────────────────────────────────────────┐
│ Demande de Priorite Exceptionnelle                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Operation: Acces DB Recovery                                │
│ Priorite actuelle: P1                                       │
│ Priorite demandee: P0 (Critique)                           │
│                                                             │
│ Justification: (obligatoire)                                │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ Corruption DB detectee suite a coupure electrique.     │ │
│ │ Incident #INC-2026-0128. Intervention urgente requise. │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                                             │
│ Duree estimee: [30] minutes                                │
│                                                             │
│ ⚠ Cette demande sera validee par StrongFather              │
│                                                             │
│ [Soumettre]                    [Annuler]                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 11.3 Resultat de Demande

```
┌─────────────────────────────────────────────────────────────┐
│ Decision LogisticsSteward                                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ [✓] EXCEPTION ACCORDEE                                      │
│                                                             │
│ Priorite accordee: P0 (Critique)                           │
│ Validite: 30 minutes (expire a 12:30)                      │
│                                                             │
│ Conditions:                                                 │
│  • Monitoring intensif active                               │
│  • Ressources CPU/Memoire reservees                         │
│  • Non renouvelable automatiquement                         │
│                                                             │
│ ID Exception: EXC-2026-0128-001                            │
│ Validation SF: DEC-2026-0128-045                           │
│                                                             │
│ [Commencer l'operation]        [Annuler]                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 12. Invariants d'Integration

| Code | Invariant |
|------|-----------|
| **INV-MA-LS-1** | MiyukiniAdmin ne peut pas bypasser LogisticsSteward |
| **INV-MA-LS-2** | Toute exception requiert validation StrongFather |
| **INV-MA-LS-3** | Les quotas s'appliquent meme a MiyukiniAdmin |
| **INV-MA-LS-4** | La tracabilite est complete et auditable |
| **INV-MA-LS-5** | En degradation, priorite P1 maximum sans exception |

---

## 13. Documents Associes

- [MiyukiniAdmin - Core Interaction Contract](../../architecture/MiyukiniAdmin%20-%20Core%20Interaction%20Contract.md)
- [MiyukiniAdmin - BondingBrother Integration Contract](./MiyukiniAdmin%20-%20BondingBrother%20Integration%20Contract.md)
- [MiyukiniAdmin - StrongFather Integration Contract](./MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md)
- [LogisticsSteward - Documentation Fondatrice](../../../LogisticsSteward/foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md)
- [LogisticsSteward - MiyukiniAdmin Integration Contract](../../../LogisticsSteward/contracts/integration/LogisticsSteward%20-%20MiyukiniAdmin%20Integration%20Contract.md)
- [LogisticsSteward - Priority Management Contract](../../../LogisticsSteward/contracts/resources/LogisticsSteward%20-%20Priority%20Management%20Contract.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference
