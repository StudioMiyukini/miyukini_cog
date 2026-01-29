# MiyukiniAdmin — Security Level Management Contract

## 1. Contexte

Ce document definit le contrat pour la **gestion des niveaux de securite** dans MiyukiniAdmin. MiyukiniAdmin est la seule entite autorisee a changer manuellement le niveau de securite du systeme (0-4).

Le changement de niveau de securite est une operation **critique** qui requiert validation StrongFather et justification obligatoire.

## 2. Portee / Scope

Ce document definit :
- Les operations de lecture du niveau de securite
- Les operations de changement de niveau
- Les protocoles de validation
- Les modes de degradation
- L'integration avec WorrySentinel

Ce document **ne couvre pas** :
- La definition des niveaux de securite (voir Security Levels reference)
- Le modele de menaces (voir Threat Model Contract)
- Les protocoles de securite detailles (voir Security Protocols reference)

---

## 3. Principe Fondamental

### 3.1 Gouvernance de Securite

> **La securite est un parametre de gouvernance, pas un choix applicatif. MiyukiniAdmin est l'interface humaine pour modifier ce parametre.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-SLM-1** | Changement de niveau = validation StrongFather obligatoire |
| **INV-SLM-2** | Justification obligatoire pour tout changement |
| **INV-SLM-3** | Tracabilite complete (qui, quand, pourquoi, resultat) |
| **INV-SLM-4** | Notification automatique a CaringNanny |
| **INV-SLM-5** | Propagation immediate a tous les cores |

---

## 4. Les 5 Niveaux de Securite

### 4.1 Rappel des Niveaux

| Niveau | Nom | Description | Impact Performance |
|--------|-----|-------------|-------------------|
| **0** | PUBLIC | Site vitrine, donnees publiques | Quasi nul |
| **1** | STANDARD | CMS, backoffice simple | Faible |
| **2** | SENSITIVE | Donnees personnelles, profils | Modere |
| **3** | CRITICAL | Auth, paiement, decisions | Accepte |
| **4** | HARDENED | Environnement isole, hostile | Secondaire |

### 4.2 Reference Complete

Voir [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)

---

## 5. Operations de Lecture

### 5.1 Lecture Niveau Actuel

**Capacite :** `admin.security.level.read`

**Validation StrongFather :** Non requise

```json
{
  "operation": "READ_SECURITY_LEVEL"
}
```

**Reponse :**

```json
{
  "current_level": 2,
  "level_name": "SENSITIVE",
  "since": "2026-01-28T10:00:00Z",
  "set_by": "operator-uuid",
  "justification": "Passage en mode donnees sensibles",
  "trust_level": "T0",
  "degradation_active": false
}
```

### 5.2 Lecture Historique

**Capacite :** `admin.security.level.history`

```json
{
  "operation": "READ_SECURITY_HISTORY",
  "parameters": {
    "from": "2026-01-01T00:00:00Z",
    "to": "2026-01-28T23:59:59Z",
    "limit": 50
  }
}
```

---

## 6. Operations de Changement

### 6.1 Changement de Niveau

**Capacite :** `admin.security.level.write`

**Validation StrongFather :** Obligatoire

**Role minimum :** Admin

### 6.2 Protocole de Changement

```
Operateur                MiyukiniAdmin         BondingBrother        StrongFather    WorrySentinel
    │                          │                     │                    │               │
    │──ChangeSecurityLevel────▶│                     │                    │               │
    │  (new_level: 3,          │                     │                    │               │
    │   justification: "...")   │                     │                    │               │
    │                          │                     │                    │               │
    │                          │──ValidateChange─────▶│                    │               │
    │                          │                     │                    │               │
    │                          │                     │──RequestApproval───▶│               │
    │                          │                     │                    │               │
    │                          │                     │◀─APPROVED──────────│               │
    │                          │                     │                    │               │
    │                          │                     │──ApplyChange────────────────────────▶│
    │                          │                     │                    │               │
    │                          │                     │◀─ChangeApplied─────────────────────│
    │                          │                     │                    │               │
    │                          │◀─ChangeConfirmed───│                    │               │
    │                          │                     │                    │               │
    │◀─ChangeResult───────────│                     │                    │               │
    │  (success, new_level)     │                    │                    │               │
```

### 6.3 Format Requete

```json
{
  "operation": "CHANGE_SECURITY_LEVEL",
  "parameters": {
    "new_level": 3,
    "justification": "Passage en mode CRITICAL suite a detection tentatives d'intrusion - Incident #INC-2026-0128",
    "incident_reference": "INC-2026-0128",
    "notify_operators": true
  }
}
```

### 6.4 Validations

| Validation | Description |
|------------|-------------|
| **Role** | Operateur avec role Admin minimum |
| **Justification** | Minimum 50 caracteres |
| **Coherence** | Niveau demande valide (0-4) |
| **Etat systeme** | Pas de changement pendant recovery |

### 6.5 Restrictions de Transition

| De | Vers | Restriction |
|----|------|-------------|
| 0-2 | 3-4 | Justification detaillee requise |
| 3-4 | 0-2 | Verification etat systeme OK |
| Tout | 4 | Confirmation supplementaire requise |
| 4 | Tout | Periode de verification post-changement |

---

## 7. Modes de Degradation

### 7.1 Definition

Les modes de degradation sont des etats temporaires qui restreignent les fonctionnalites pour proteger le systeme.

| Mode | Description | Declencheur |
|------|-------------|-------------|
| **WATCHFUL** | Surveillance renforcee | Anomalies detectees |
| **RESTRICTED** | Fonctions sensibles desactivees | Menace potentielle |
| **LOCKDOWN** | Lecture seule | Menace confirmee |
| **ISOLATED** | Isolation complete | Compromission |

### 7.2 Activation Manuelle

**Capacite :** `admin.security.degradation.activate`

```json
{
  "operation": "ACTIVATE_DEGRADATION",
  "parameters": {
    "mode": "RESTRICTED",
    "reason": "Tentatives d'intrusion detectees",
    "duration_minutes": 60,
    "auto_review": true
  }
}
```

### 7.3 Desactivation

**Capacite :** `admin.security.degradation.deactivate`

```json
{
  "operation": "DEACTIVATE_DEGRADATION",
  "parameters": {
    "confirm_threat_resolved": true,
    "justification": "Menace ecartee, retour normal"
  }
}
```

---

## 8. Isolation de Modules

### 8.1 Isolation d'Operateur

MiyukiniAdmin peut isoler un Operateur specifique.

**Capacite :** `admin.operators.isolate`

```json
{
  "operation": "ISOLATE_OPERATOR",
  "parameters": {
    "operator_id": "uuid-operator-suspect",
    "reason": "Comportement anormal detecte",
    "isolation_level": "FULL",
    "duration_minutes": 30
  }
}
```

### 8.2 Niveaux d'Isolation

| Niveau | Effet |
|--------|-------|
| **MONITOR** | Surveillance renforcee, pas de restriction |
| **THROTTLE** | Limitation du debit |
| **RESTRICTED** | Fonctions sensibles bloquees |
| **FULL** | Acces completement bloque |

### 8.3 Restauration

```json
{
  "operation": "RESTORE_OPERATOR",
  "parameters": {
    "operator_id": "uuid-operator-suspect",
    "justification": "Verification terminee, comportement normal"
  }
}
```

---

## 9. Integration WorrySentinel

### 9.1 Flux d'Information

```
MiyukiniAdmin ──▶ BondingBrother ──▶ WorrySentinel
                                          │
                                          ├── Lecture niveau actuel
                                          ├── Lecture modes degradation
                                          ├── Changement niveau
                                          └── Activation degradation
```

### 9.2 Notifications Recues

MiyukiniAdmin recoit de WorrySentinel :
- Alertes de securite
- Changements automatiques de niveau
- Declenchements de degradation automatiques
- Recommandations d'action

---

## 10. Interface UI

### 10.1 Panneau Securite

| Zone | Contenu |
|------|---------|
| **Niveau actuel** | Indicateur visuel (0-4) avec couleur |
| **Historique** | Timeline des changements recents |
| **Degradation** | Etat des modes de degradation |
| **Operateurs** | Liste avec etat d'isolation |
| **Actions** | Boutons de changement |

### 10.2 Indicateurs Visuels Niveau

| Niveau | Couleur | Icone |
|--------|---------|-------|
| 0 - PUBLIC | Vert | Unlock |
| 1 - STANDARD | Bleu | Shield |
| 2 - SENSITIVE | Jaune | Shield+ |
| 3 - CRITICAL | Orange | Shield-Alert |
| 4 - HARDENED | Rouge | Fortress |

### 10.3 Dialogue Changement

```
┌─────────────────────────────────────────────────────────────┐
│ Changement de Niveau de Securite                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Niveau actuel: [2 - SENSITIVE]                             │
│                                                             │
│ Nouveau niveau: [Dropdown 0-4]                              │
│                                                             │
│ Justification (obligatoire):                                │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │                                                         │ │
│ │ (minimum 50 caracteres)                                 │ │
│ │                                                         │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                                             │
│ Reference incident (optionnel): [_______________]           │
│                                                             │
│ [ ] Notifier les Operateurs                                 │
│                                                             │
│ Impact:                                                     │
│ - Performance: [Graphique impact]                           │
│ - Fonctionnalites: [Liste restrictions]                     │
│                                                             │
│        [Annuler]                    [Confirmer]             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 11. Audit et Tracabilite

### 11.1 Donnees Tracees

| Champ | Description |
|-------|-------------|
| `change_id` | Identifiant unique |
| `timestamp` | Horodatage |
| `operator_id` | Operateur humain |
| `previous_level` | Niveau avant |
| `new_level` | Nouveau niveau |
| `justification` | Texte justificatif |
| `incident_reference` | Reference incident |
| `strongfather_approval` | ID decision SF |
| `propagation_time_ms` | Temps de propagation |

### 11.2 Retention

| Type | Retention |
|------|-----------|
| Changements niveau | Permanent |
| Activations degradation | 2 ans |
| Isolations operateurs | 1 an |

---

## 12. Recommandations Automatiques

### 12.1 Suggestions de Niveau

MiyukiniAdmin peut suggerer un changement de niveau base sur :
- Alertes WorrySentinel
- Metriques CaringNanny
- Historique d'incidents

```json
{
  "recommendation": {
    "suggested_level": 3,
    "reason": "Augmentation de 300% des tentatives d'auth echouees",
    "confidence": 0.85,
    "auto_apply": false
  }
}
```

### 12.2 Alertes Proactives

| Condition | Alerte |
|-----------|--------|
| Niveau 0-1 + donnees sensibles | "Niveau potentiellement insuffisant" |
| Niveau 3-4 prolonge | "Verifier si necessaire" |
| Incidents repetes | "Envisager niveau superieur" |

---

## 13. Documents Associes

- [MiyukiniAdmin - Threat Model Contract](./MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)
- [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)
- [WorrySentinel - Documentation Fondatrice](../../../WorrySentinel/WorrySentinel%20-%20Documentation%20Fondatrice.md)
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Contrat de reference
