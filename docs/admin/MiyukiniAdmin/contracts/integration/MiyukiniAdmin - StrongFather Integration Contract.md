# MiyukiniAdmin — StrongFather Integration Contract

## 1. Contexte

Ce document definit le contrat d'integration entre MiyukiniAdmin et **StrongFather**. StrongFather est l'autorite de decision qui valide les actions administratives critiques.

## 2. Portee / Scope

Ce document definit :
- Le role de StrongFather pour MiyukiniAdmin
- Les decisions requises
- Les protocoles de validation
- Les criteres d'approbation

Ce document **ne couvre pas** :
- L'implementation interne de StrongFather
- Les autres integrations cores

---

## 3. Role de StrongFather

### 3.1 Autorite sur les Decisions Administratives

> **StrongFather valide les actions administratives critiques avant leur execution.**

**Question fondamentale :**
> "Cette action administrative devrait-elle etre effectuee ?"

### 3.2 Responsabilites

| Responsabilite | Description |
|----------------|-------------|
| **Validation actions** | Approuve/refuse les operations critiques |
| **Controle securite** | Valide les changements de niveau |
| **Arbitrage** | Decide en cas de conflit |
| **Gouvernance** | Applique les politiques systeme |

---

## 4. Actions Necessitant Validation

### 4.1 Liste des Actions

| Action | Capacite | Criticite |
|--------|----------|-----------|
| Changement niveau securite | `admin.security.level.write` | CRITIQUE |
| Activation mode degradation | `admin.security.degradation.activate` | HAUTE |
| Isolation Operateur | `admin.operators.isolate` | HAUTE |
| Migration DB | `admin.db.migrate` | CRITIQUE |
| Reparation DB | `admin.db.repair` | HAUTE |
| Acces DB recovery | `admin.db.recovery` | CRITIQUE |
| Tests de charge | `admin.tests.load` | MOYENNE |

### 4.2 Actions Sans Validation SF

| Action | Capacite | Raison |
|--------|----------|--------|
| Lecture metriques | `admin.metrics.*` | Lecture seule |
| Lecture niveau securite | `admin.security.level.read` | Lecture seule |
| Tests unitaires | `admin.tests.coherence` | Non impactant |
| Liste Operateurs | `admin.operators.list` | Lecture seule |

---

## 5. Protocole de Validation

### 5.1 Format de Demande

```json
{
  "decision_request_id": "uuid-decision-001",
  "timestamp": "2026-01-28T12:00:00Z",
  "source": "miyukini_admin",
  "action": {
    "type": "SECURITY_LEVEL_CHANGE",
    "capability": "admin.security.level.write",
    "parameters": {
      "current_level": 2,
      "new_level": 3
    }
  },
  "context": {
    "operator_id": "uuid-operator",
    "operator_role": "Admin",
    "justification": "Detection tentatives intrusion - Incident #INC-2026-0128",
    "system_state": {
      "trust_level": "T0",
      "security_level": 2,
      "active_operators": 15
    }
  }
}
```

### 5.2 Format de Reponse

```json
{
  "decision_request_id": "uuid-decision-001",
  "decision_id": "uuid-decision-response-001",
  "timestamp": "2026-01-28T12:00:01Z",
  "decision": "APPROVED",
  "reasoning": "Justification valide, contexte coherent, role suffisant",
  "conditions": [],
  "validity": {
    "expires_at": "2026-01-28T12:05:01Z",
    "max_uses": 1
  }
}
```

### 5.3 Decisions Possibles

| Decision | Description |
|----------|-------------|
| `APPROVED` | Action autorisee |
| `DENIED` | Action refusee |
| `PENDING` | En attente d'information |
| `CONDITIONAL` | Approuve sous conditions |

---

## 6. Flux de Validation

### 6.1 Flux Standard

```
MiyukiniAdmin           BondingBrother              StrongFather
     │                        │                          │
     │──ActionRequest─────────▶│                          │
     │  (capability: critical) │                          │
     │                        │                          │
     │                        │──DecisionRequest─────────▶│
     │                        │                          │
     │                        │                          │  [Evaluation]
     │                        │                          │  - Politique
     │                        │                          │  - Contexte
     │                        │                          │  - Historique
     │                        │                          │
     │                        │◀─DecisionResponse────────│
     │                        │  (APPROVED)               │
     │                        │                          │
     │                        │  [Execute action]         │
     │                        │                          │
     │◀─ActionResponse────────│                          │
```

### 6.2 Flux avec Rejet

```
MiyukiniAdmin           BondingBrother              StrongFather
     │                        │                          │
     │──ActionRequest─────────▶│                          │
     │                        │                          │
     │                        │──DecisionRequest─────────▶│
     │                        │                          │
     │                        │◀─DecisionResponse────────│
     │                        │  (DENIED)                 │
     │                        │  (reason: "...")          │
     │                        │                          │
     │◀─ActionResponse────────│                          │
     │  (status: DENIED)       │                          │
     │  (reason: "...")        │                          │
```

---

## 7. Criteres d'Approbation

### 7.1 Criteres Generaux

| Critere | Description |
|---------|-------------|
| **Role suffisant** | Operateur a le role requis |
| **Justification valide** | Justification coherente et detaillee |
| **Contexte coherent** | Action coherente avec l'etat systeme |
| **Politique respectee** | Conforme aux politiques definies |

### 7.2 Criteres Specifiques par Action

#### Changement Niveau Securite

| Critere | Verification |
|---------|--------------|
| Role | Admin minimum |
| Justification | >= 50 caracteres |
| Coherence | Niveau demande logique vs situation |
| Historique | Pas de changement recurrent suspect |

#### Acces DB Recovery

| Critere | Verification |
|---------|--------------|
| Role | Recovery |
| Etat systeme | T3 ou T4 |
| Protocole | REINFORCED actif |
| Justification | Detaillee avec incident reference |
| MFA | Verifie |

#### Isolation Operateur

| Critere | Verification |
|---------|--------------|
| Role | Admin |
| Raison | Comportement anormal documente |
| Impact | Evaluation impact utilisateurs |

---

## 8. Politiques de Decision

### 8.1 Politique par Defaut

```yaml
policy:
  admin_actions:
    security_level_change:
      require_role: Admin
      require_justification: true
      min_justification_length: 50
      cooldown_minutes: 5
      
    db_recovery:
      require_role: Recovery
      require_conditions:
        - trust_level: [T3, T4]
        - protocol: REINFORCED
        - mfa: true
      max_duration_minutes: 30
      
    operator_isolation:
      require_role: Admin
      require_justification: true
      require_impact_assessment: true
```

### 8.2 Overrides d'Urgence

En situation d'urgence absolue, StrongFather peut :
- Raccourcir le cooldown
- Accepter une justification plus courte
- MAIS jamais bypasser le role minimum

---

## 9. Audit et Tracabilite

### 9.1 Donnees Tracees

| Champ | Description |
|-------|-------------|
| `decision_request_id` | ID demande |
| `decision_id` | ID decision |
| `timestamp` | Horodatage |
| `action_type` | Type d'action |
| `operator_id` | Demandeur |
| `decision` | APPROVED/DENIED |
| `reasoning` | Raisonnement |
| `policy_applied` | Politique utilisee |

### 9.2 Retention

| Type | Retention |
|------|-----------|
| Decisions APPROVED | 2 ans |
| Decisions DENIED | 2 ans |
| Decisions critiques | Permanent |

---

## 10. Gestion des Erreurs

### 10.1 Erreurs Possibles

| Code | Description | Action |
|------|-------------|--------|
| `SF_001` | StrongFather indisponible | Retry avec backoff |
| `SF_002` | Timeout decision | Retry ou abandon |
| `SF_003` | Decision expiree | Nouvelle demande |
| `SF_004` | Decision invalide | Log + alerte |

### 10.2 Fallback

**En cas d'indisponibilite StrongFather :**
- Actions critiques : REFUSEES par defaut
- Actions moyennes : REFUSEES par defaut
- Pas de fallback "auto-approve"

---

## 11. Integration UI

### 11.1 Dialogue de Validation

```
┌─────────────────────────────────────────────────────────────┐
│ Validation StrongFather Requise                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Action: Changement niveau securite                          │
│ De: Niveau 2 (SENSITIVE)                                   │
│ Vers: Niveau 3 (CRITICAL)                                  │
│                                                             │
│ Justification fournie:                                      │
│ "Detection tentatives intrusion - Incident #INC-2026-0128" │
│                                                             │
│ Statut: [Spinner] En attente de decision...                │
│                                                             │
│ [Annuler]                                                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 11.2 Resultat Validation

```
┌─────────────────────────────────────────────────────────────┐
│ Decision StrongFather                                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ [✓] APPROUVE                                                │
│                                                             │
│ Raisonnement:                                               │
│ "Justification valide, contexte coherent, role suffisant"  │
│                                                             │
│ Validite: 5 minutes                                        │
│                                                             │
│ [Executer]              [Annuler]                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 12. Documents Associes

- [MiyukiniAdmin - Core Interaction Contract](../../architecture/MiyukiniAdmin%20-%20Core%20Interaction%20Contract.md)
- [MiyukiniAdmin - BondingBrother Integration Contract](./MiyukiniAdmin%20-%20BondingBrother%20Integration%20Contract.md)
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference
