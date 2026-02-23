# MiyukiniTerminal — Référence Technique Complète

## Contexte

Ce document est une **référence condensée** : tables, champs, constantes, formats et index rapide vers les documents détaillés. À utiliser comme aide-mémoire pendant le développement.

**Références :** Voir [Index Documentation](./MiyukiniTerminal%20-%20Index%20Documentation.md).

---

## Portée / Scope

- Champs, formats, codes
- Constantes
- Tables de référence
- Liens vers specs

---

## 1. Types COG

| cog_type | Octet | Nom |
|----------|-------|-----|
| TERMINAL | 0x05 | COG mobile |

| os_type | Octet | Nom |
|---------|-------|-----|
| ANDROID | 0x03 | Google Android |

---

## 2. Passeport TERMINAL

| Champ | Obligatoire | Format |
|-------|-------------|--------|
| cog_id | ✅ | UUID |
| cog_type | ✅ | 0x05 |
| os_type | ✅ | 0x03 |
| core_version | ✅ | MAJOR.MINOR |
| service_list | ✅ | JSON |
| environment_health | ✅ | JSON |
| parent_cog_id | ✅ | UUID parent |
| passport_type | ✅ | STANDARD (0) |
| nonce | ✅ | 16 bytes |
| timestamp | ✅ | epoch |

---

## 3. Tables stockage

### identity

| Colonne | Type |
|---------|------|
| cog_id | TEXT |
| parent_cog_id | TEXT |
| permis_id | TEXT |
| permis_expires_at | INTEGER |
| created_at | INTEGER |
| updated_at | INTEGER |

### queue_actions

| Colonne | Type |
|---------|------|
| action_type | TEXT |
| payload | TEXT |
| status | pending/sent/failed |
| created_at | INTEGER |
| retry_count | INTEGER |

---

## 4. Messages Relay

| Code | Nom |
|------|-----|
| 0x01 | REGISTER |
| 0x02 | REGISTER_OK |
| 0x03 | REGISTER_ERR |
| 0x08 | HEARTBEAT |
| 0x09 | HEARTBEAT_ACK |

---

## 5. Constantes

| Nom | Valeur |
|-----|--------|
| Relay port | 7000 |
| Token expiration | 15 min |
| Heartbeat interval | 30–60 s |
| Max terminaux/STABLE | 5 |
| Queue max pending | 100 |
| Retry max | 5 |
| API Android min | 24 |

---

## 6. Référence MIP (MSCM)

### 6.1 Préfixes @id

| Préfixe | Module |
|---------|--------|
| terminal.liaison.v1 | Liaison parent |
| terminal.mws.v1 | Client MWS / Relay |
| terminal.sync.v1 | Synchronisation |
| terminal.storage.v1 | Stockage local |
| terminal.ui.v1 | Composants UI |

### 6.2 Rôles et couches

| @role | Usage |
|-------|-------|
| security | Validation, signature, chiffrement |
| data | Persistance, cache, queue |
| logic | Règles métier, détection conflits |
| api | Appels réseau |
| infra | Transport, stockage bas niveau |
| ui | Composants Dioxus |

| @layer | Usage |
|--------|-------|
| domain | Logique métier |
| infra | Technique |
| service | Orchestration |
| ui | Interface |

### 6.3 Phase B : mapping service_list → blocs

Pour le `svc_manifest` Terminal, utiliser les IDs de modules MIP comme `service_id` :

```json
["terminal.liaison.v1", "terminal.mws.v1", "terminal.sync.v1"]
```

---

## 7. Liens rapides

| Document | Sujet |
|----------|-------|
| [Spec MWS Passeport](./MiyukiniTerminal%20-%20Spec%20MWS%20Passeport%20Permis.md) | Passeport détaillé |
| [Spec Protocole Relay](./MiyukiniTerminal%20-%20Spec%20Protocole%20Relay%20Terminal.md) | Messages, trames |
| [Spec MSCM MIP](./MiyukiniTerminal%20-%20Spec%20MSCM%20MIP%20Conformite.md) | Conformité MSCM/MIP |
| [Spec Stockage](./MiyukiniTerminal%20-%20Spec%20Stockage%20Local.md) | Schéma complet |
| [Spec Flux Liaison](./MiyukiniTerminal%20-%20Spec%20Flux%20Liaison%20Parent.md) | Liaison |
| [Index](./MiyukiniTerminal%20-%20Index%20Documentation.md) | Tous les documents |
