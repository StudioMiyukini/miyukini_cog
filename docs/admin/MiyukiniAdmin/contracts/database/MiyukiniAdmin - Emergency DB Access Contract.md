# MiyukiniAdmin — Emergency DB Access Contract

## 1. Contexte

Ce document definit le contrat pour l'**acces direct a la base de donnees en mode recovery**. Ce mode est une **exception ultra-controlee** qui permet a MiyukiniAdmin d'ecrire directement en DB, contournant temporairement KindMother.

Ce mode est comparable a un **mode recovery** de systeme d'exploitation : exceptionnel, dangereux, necessaire.

## 2. Portee / Scope

Ce document definit :
- Les conditions d'activation du mode recovery
- Le protocole d'intervention
- Les operations autorisees
- Les contraintes de securite
- Le retour au mode normal

Ce document **ne couvre pas** :
- Les operations DB normales (voir DB Operations Contract)
- Les metriques DB (voir DB Metrics Contract)
- L'interface utilisateur detaillee (voir UI documentation)

---

## 3. Principe Fondamental

### 3.1 Exception Ultra-Controlee

> **L'acces direct DB est une operation exceptionnelle, temporaire, strictement encadree, qui ne doit etre utilisee qu'en dernier recours.**

**Ce mode :**
- Est comparable a un mode recovery
- N'est PAS un fallback normal
- Requiert des conditions cumulatives strictes
- Est entierement trace et auditable

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-EDB-1** | Conditions cumulatives obligatoires |
| **INV-EDB-2** | Intervention humaine authentifiee requise |
| **INV-EDB-3** | Fenetre temporelle limitee |
| **INV-EDB-4** | Journalisation complete de chaque action |
| **INV-EDB-5** | Blocage des Operateurs pendant l'operation |
| **INV-EDB-6** | Revalidation obligatoire apres intervention |

---

## 4. Conditions d'Activation

### 4.1 Conditions Cumulatives Obligatoires

**TOUTES les conditions suivantes doivent etre reunies :**

| # | Condition | Verification |
|---|-----------|--------------|
| 1 | **Etat systeme >= Critique** | Niveau de confiance T3 ou T4 |
| 2 | **Protocole securite renforce active** | Flag WorrySentinel = REINFORCED |
| 3 | **Intervention humaine authentifiee** | MFA + Role Admin/Recovery |
| 4 | **Justification documentee** | Texte obligatoire |
| 5 | **Approbation StrongFather** | Decision explicite APPROVED |
| 6 | **Fenetre temporelle definie** | Duree max configuree |

### 4.2 Verification des Conditions

```
┌─────────────────────────────────────────────────────────────┐
│ Verification Condition 1 : Etat Systeme                      │
├─────────────────────────────────────────────────────────────┤
│ CaringNanny.trust_level >= T3                               │
│ OU CaringNanny.system_state == CRITICAL                     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Verification Condition 2 : Protocole Securite               │
├─────────────────────────────────────────────────────────────┤
│ WorrySentinel.protocol_state == REINFORCED                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Verification Condition 3 : Authentification Humaine         │
├─────────────────────────────────────────────────────────────┤
│ operator.authenticated == true                               │
│ AND operator.mfa_verified == true                           │
│ AND operator.role IN (Admin, Recovery)                      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Verification Condition 4 : Justification                    │
├─────────────────────────────────────────────────────────────┤
│ request.justification.length >= 50 caracteres               │
│ AND request.incident_reference IS NOT NULL                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Verification Condition 5 : Approbation StrongFather         │
├─────────────────────────────────────────────────────────────┤
│ StrongFather.emergency_approval == APPROVED                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Verification Condition 6 : Fenetre Temporelle               │
├─────────────────────────────────────────────────────────────┤
│ request.max_duration <= config.max_recovery_duration        │
│ (default: 30 minutes, max: 2 heures)                        │
└─────────────────────────────────────────────────────────────┘
```

---

## 5. Protocole d'Intervention

### 5.1 Flux Complet

```
Operateur                MiyukiniAdmin         BondingBrother        Cores
    │                          │                     │                │
    │──RecoveryRequest────────▶│                     │                │
    │  (justification,         │                     │                │
    │   incident_ref,          │                     │                │
    │   duration)              │                     │                │
    │                          │                     │                │
    │◀─MFAChallenge───────────│                     │                │
    │                          │                     │                │
    │──MFAResponse────────────▶│                     │                │
    │                          │                     │                │
    │                          │──VerifyConditions──▶│                │
    │                          │                     │                │
    │                          │                     │──CheckTrustLevel──▶│ CaringNanny
    │                          │                     │◀─T3/T4─────────────│
    │                          │                     │                │
    │                          │                     │──CheckSecurityProto─▶│ WorrySentinel
    │                          │                     │◀─REINFORCED────────│
    │                          │                     │                │
    │                          │                     │──RequestApproval───▶│ StrongFather
    │                          │                     │◀─APPROVED──────────│
    │                          │                     │                │
    │                          │◀─ConditionsValid───│                │
    │                          │                     │                │
    │                          │──NotifyRecoveryStart────────────────────▶│ CaringNanny
    │                          │                     │     (blocage Ops)  │
    │                          │                     │                │
    │◀─RecoveryGranted────────│                     │                │
    │                          │                     │                │
    │  ═══════════════════════════════════════════════════════════════│
    │           MODE RECOVERY ACTIF (acces direct DB)                  │
    │  ═══════════════════════════════════════════════════════════════│
    │                          │                     │                │
    │──DBCommand──────────────▶│                     │                │
    │  (INSERT/UPDATE/DELETE)  │────────────────────────────────────────▶│ DB Direct
    │                          │                     │                │
    │◀─CommandResult──────────│                     │                │
    │                          │                     │                │
    │  [... autres commandes ...]                    │                │
    │                          │                     │                │
    │──EndRecovery────────────▶│                     │                │
    │                          │                     │                │
    │                          │──TriggerRevalidation─────────────────▶│ KindMother
    │                          │◀─RevalidationComplete────────────────│
    │                          │                     │                │
    │                          │──NotifyRecoveryEnd──────────────────▶│ CaringNanny
    │                          │                     │ (deblocage Ops)  │
    │                          │                     │                │
    │◀─RecoveryReport─────────│                     │                │
    │                          │                     │                │
```

### 5.2 Phases de l'Intervention

| Phase | Description | Duree typique |
|-------|-------------|---------------|
| **1. Authentification** | MFA + verification role | 1-2 min |
| **2. Validation conditions** | Verification automatique | < 30s |
| **3. Preparation** | Blocage Operateurs, backup | 1-5 min |
| **4. Intervention** | Operations DB directes | Variable (max config) |
| **5. Revalidation** | Verification coherence via KindMother | 1-5 min |
| **6. Retour normal** | Deblocage, rapport | < 1 min |

---

## 6. Operations Autorisees

### 6.1 Categories d'Operations

| Categorie | Operations | Niveau risque |
|-----------|------------|---------------|
| **Correction** | UPDATE, DELETE cibles | Moyen |
| **Restauration** | INSERT depuis backup | Moyen |
| **Schema** | ALTER TABLE (correctif) | Eleve |
| **Truncate** | TRUNCATE table | Eleve |

### 6.2 Operations Interdites

| Operation | Raison |
|-----------|--------|
| DROP TABLE | Destruction irreversible |
| DROP DATABASE | Destruction totale |
| ALTER USER/ROLE | Modification securite |
| CREATE/DROP INDEX massif | Impact performance |

### 6.3 Format Commande

```json
{
  "recovery_session_id": "uuid-recovery-001",
  "command_sequence": 1,
  "operation": {
    "type": "UPDATE",
    "target": "users",
    "set": {"status": "active"},
    "where": {"id": "uuid-user-corrupted"},
    "limit": 1
  },
  "justification": "Correction statut utilisateur corrompu - Incident #INC-2026-0128"
}
```

### 6.4 Validation Pre-Execution

Avant chaque commande :
1. Verification que la session est toujours valide
2. Verification que la fenetre temporelle n'est pas expiree
3. Estimation du nombre de lignes affectees
4. Confirmation si > 100 lignes affectees

---

## 7. Securite

### 7.1 Authentification Renforcee

| Etape | Requirement |
|-------|-------------|
| Login | Credentials valides |
| MFA | Code TOTP ou hardware key |
| Role | Admin ou Recovery |
| Session | Nouvelle session (pas de reutilisation) |

### 7.2 Isolation

| Mesure | Description |
|--------|-------------|
| **Blocage Operateurs** | Tous les Operateurs sont bloques |
| **Session exclusive** | Une seule session recovery a la fois |
| **Timeout strict** | Expiration automatique |
| **Pas de parallelisme** | Commandes sequentielles uniquement |

### 7.3 Annulation d'Urgence

**Kill Switch :**
- Bouton d'annulation immediat dans l'UI
- Commande d'annulation via CLI
- Timeout automatique si inactivite > 5 min

---

## 8. Journalisation

### 8.1 Donnees Tracees

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `recovery_session_id` | ID unique de session | Oui |
| `timestamp_start` | Debut de session | Oui |
| `timestamp_end` | Fin de session | Oui |
| `operator_id` | Operateur humain | Oui |
| `operator_ip` | Adresse IP | Oui |
| `justification` | Texte justificatif | Oui |
| `incident_reference` | Reference incident | Oui |
| `conditions_verified` | Liste conditions verifiees | Oui |
| `commands_executed` | Liste de toutes les commandes | Oui |
| `rows_affected_total` | Total lignes modifiees | Oui |
| `revalidation_result` | Resultat revalidation | Oui |

### 8.2 Format Log Commande

```json
{
  "session_id": "uuid-recovery-001",
  "command_id": "uuid-cmd-001",
  "sequence": 1,
  "timestamp": "2026-01-28T12:05:30Z",
  "operation": {
    "type": "UPDATE",
    "sql_executed": "UPDATE users SET status = 'active' WHERE id = 'uuid-user-corrupted'",
    "rows_affected": 1,
    "execution_time_ms": 15
  },
  "before_state": {
    "id": "uuid-user-corrupted",
    "status": "corrupted"
  },
  "after_state": {
    "id": "uuid-user-corrupted", 
    "status": "active"
  }
}
```

### 8.3 Retention

| Type | Retention |
|------|-----------|
| Session recovery | Permanent |
| Commandes executees | Permanent |
| Etats before/after | 5 ans |

---

## 9. Revalidation Post-Intervention

### 9.1 Processus

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Fin des commandes recovery                                │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Trigger revalidation KindMother                          │
├─────────────────────────────────────────────────────────────┤
│ - Verification integrite referentielle                      │
│ - Verification contraintes                                  │
│ - Verification coherence                                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Execution suite tests Post-Recovery                      │
├─────────────────────────────────────────────────────────────┤
│ - COH-001 (Integrite referentielle)                         │
│ - COH-002 (NOT NULL)                                        │
│ - COH-003 (Unicite)                                         │
│ - CONF-001 (Schema)                                         │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              ▼                               ▼
┌─────────────────────────┐    ┌─────────────────────────┐
│ Tests OK                 │    │ Tests FAIL              │
│ → Deblocage Operateurs   │    │ → Alerte + Investigation│
│ → Retour mode normal     │    │ → Operateurs bloques    │
└─────────────────────────┘    └─────────────────────────┘
```

### 9.2 Actions si Revalidation Echoue

| Severite | Action |
|----------|--------|
| WARNING | Log + Alerte + Deblocage |
| FAIL | Log + Alerte + Deblocage avec surveillance |
| CRITICAL | Log + Alerte + Operateurs bloques + Escalade |

---

## 10. Interface UI

### 10.1 Ecran Mode Recovery

| Zone | Contenu |
|------|---------|
| **Banner** | ALERTE ROUGE - MODE RECOVERY ACTIF |
| **Timer** | Temps restant avant expiration |
| **Console** | Zone de saisie SQL |
| **Log** | Historique commandes en temps reel |
| **Stats** | Lignes affectees, duree |
| **Actions** | Executer, Annuler, Terminer |

### 10.2 Indicateurs Visuels

| Etat | Couleur | Icone |
|------|---------|-------|
| Pre-recovery | Jaune | Warning |
| Recovery actif | Rouge | Danger |
| Revalidation | Orange | Loading |
| Complete | Vert | Check |
| Echec | Rouge | X |

---

## 11. Limites et Garde-Fous

### 11.1 Limites Configurables

| Parametre | Default | Max |
|-----------|---------|-----|
| `max_recovery_duration` | 30 min | 2 heures |
| `max_rows_per_command` | 1000 | 10000 |
| `max_commands_per_session` | 100 | 500 |
| `inactivity_timeout` | 5 min | 15 min |

### 11.2 Garde-Fous Automatiques

| Garde-fou | Comportement |
|-----------|--------------|
| Expiration fenetre | Fin automatique + revalidation |
| Inactivite | Fin automatique apres timeout |
| Erreur SQL | Log + Continue ou Stop (configurable) |
| > 1000 lignes | Confirmation explicite requise |

---

## 12. Documents Associes

- [MiyukiniAdmin - DB Operations Contract](./MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [MiyukiniAdmin - Security Level Management Contract](../security/MiyukiniAdmin%20-%20Security%20Level%20Management%20Contract.md)
- [Miyukini Conceptual References - MiyukiniAdmin Status](../../../../reference/Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md)
- [KindMother - Documentation Fondatrice](../../../KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference CRITIQUE
