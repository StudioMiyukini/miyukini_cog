# MiyukiniAdmin - Recovery Procedures

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif â€” ProcÃ©dures de rÃ©cupÃ©ration en cas de compromission  
**PortÃ©e :** Mode recovery, Ã©criture DB directe, procÃ©dures d'urgence

---

## 1. Contexte

Ce document dÃ©finit les **procÃ©dures de rÃ©cupÃ©ration** pour MiyukiniAdmin en cas de compromission systÃ¨me, d'incident critique, ou de nÃ©cessitÃ© d'intervention d'urgence.

**RÃ©fÃ©rences :**
- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md) : Section 6.6 Recovery Exceptionnel
- [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [WorrySentinel - Security Levels Governance Contract](..//..//..//cores//WorrySentinel//contracts//levels//WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Les conditions d'activation du mode recovery
- Les procÃ©dures d'Ã©criture DB directe
- Les Ã©tapes de rÃ©cupÃ©ration post-incident
- Les vÃ©rifications de sÃ©curitÃ© post-recovery
- Les procÃ©dures de retour Ã  la normale

Ce document **ne couvre pas** :
- Les procÃ©dures de backup/restore (voir Backup Contract)
- Les procÃ©dures de migration (voir Migration Contract)
- Les procÃ©dures de mise Ã  jour (voir Versioning Contract)

---

## 3. Mode Recovery : Conditions d'activation

### 3.1 Conditions cumulatives strictes

Le mode recovery ne peut Ãªtre activÃ© que si **toutes** les conditions suivantes sont remplies :

| Condition | Description | VÃ©rification |
|-----------|-------------|--------------|
| **Ã‰tat systÃ¨me â‰¥ Critique** | Niveau de confiance T3 ou T4 (WorrySentinel) | `worrysentinel get-trust-level` |
| **Protocole sÃ©curitÃ© renforcÃ©e** | Mode sÃ©curitÃ© renforcÃ©e activÃ© explicitement | `miyukini-admin enable-enhanced-security` |
| **Intervention humaine authentifiÃ©e** | Validation manuelle obligatoire avec MFA | Authentification admin + token recovery |
| **FenÃªtre temporelle limitÃ©e** | DurÃ©e maximale dÃ©finie (ex. 1 heure) | Timer automatique de dÃ©sactivation |
| **Journalisation complÃ¨te** | Toute opÃ©ration tracÃ©e | Audit log activÃ© |
| **Revalidation obligatoire** | VÃ©rification post-intervention | Checklist de validation |

### 3.2 Activation du mode recovery

**Commande :**
```bash
# VÃ©rifier les conditions prÃ©alables
miyukini-admin check-recovery-conditions

# Activer le mode recovery (nÃ©cessite authentification MFA)
miyukini-admin activate-recovery-mode \
  --duration 3600 \
  --reason "Corruption DB critique dÃ©tectÃ©e" \
  --admin-token <MFA_TOKEN>
```

**VÃ©rifications automatiques :**
- âœ… Niveau de confiance T3 ou T4
- âœ… Protocole sÃ©curitÃ© renforcÃ©e activÃ©
- âœ… Authentification admin valide
- âœ… Token recovery valide
- âœ… Audit log opÃ©rationnel

**RÃ©sultat :**
- Mode recovery activÃ© pour la durÃ©e spÃ©cifiÃ©e
- Blocage des OpÃ©rateurs pendant l'opÃ©ration
- Journalisation complÃ¨te activÃ©e
- Timer de dÃ©sactivation automatique dÃ©marrÃ©

---

## 4. Ã‰criture DB directe en mode recovery

### 4.1 Limitations

**RÃ¨gle :** L'Ã©criture DB directe n'est autorisÃ©e qu'en mode recovery et uniquement pour :
- RÃ©paration de corruption de donnÃ©es
- Restauration d'intÃ©gritÃ© critique
- Correction d'erreurs systÃ¨me bloquantes

**Interdit :**
- âŒ Modification de donnÃ©es mÃ©tier normales
- âŒ Bypass des rÃ¨gles de gouvernance
- âŒ Modification des politiques StrongFather
- âŒ Modification des configurations WorrySentinel

### 4.2 ProcÃ©dure d'Ã©criture DB directe

#### Ã‰tape 1 : PrÃ©paration

```bash
# Sauvegarder l'Ã©tat actuel
miyukini-admin backup-create --label "pre-recovery-$(date +%Y%m%d-%H%M%S)"

# VÃ©rifier l'intÃ©gritÃ© de la backup
miyukini-admin backup-verify --latest

# Activer le mode recovery
miyukini-admin activate-recovery-mode --duration 3600
```

#### Ã‰tape 2 : Analyse

```bash
# Analyser la corruption
miyukini-admin db-analyze --table <table_name>

# Identifier les enregistrements corrompus
miyukini-admin db-verify-integrity

# GÃ©nÃ©rer un rapport d'analyse
miyukini-admin db-report --output recovery-analysis.json
```

#### Ã‰tape 3 : Ã‰criture DB directe

```bash
# Ouvrir une session DB en mode recovery
miyukini-admin db-recovery-session start

# ExÃ©cuter les corrections (exemple)
miyukini-admin db-recovery-session execute \
  --sql "UPDATE core_registry SET status = 'operational' WHERE id = '...'"

# VÃ©rifier chaque modification
miyukini-admin db-recovery-session verify --last-operation
```

**Contraintes :**
- Chaque opÃ©ration SQL est journalisÃ©e
- Chaque opÃ©ration nÃ©cessite une confirmation explicite
- Les opÃ©rations sont validÃ©es avant exÃ©cution
- Les rollback sont possibles pour chaque opÃ©ration

#### Ã‰tape 4 : Validation

```bash
# VÃ©rifier l'intÃ©gritÃ© post-modification
miyukini-admin db-verify-integrity

# VÃ©rifier la cohÃ©rence avec les cores
miyukini-admin verify-core-consistency

# GÃ©nÃ©rer un rapport de validation
miyukini-admin db-report --output recovery-validation.json
```

#### Ã‰tape 5 : Fermeture de la session

```bash
# Fermer la session recovery
miyukini-admin db-recovery-session close

# DÃ©sactiver le mode recovery
miyukini-admin deactivate-recovery-mode
```

---

## 5. ProcÃ©dures de rÃ©cupÃ©ration par type d'incident

### 5.1 Corruption de base de donnÃ©es

**SymptÃ´mes :**
- Erreurs SQLite lors des opÃ©rations
- IncohÃ©rences dans les donnÃ©es
- Ã‰chec de vÃ©rification d'intÃ©gritÃ©

**ProcÃ©dure :**
1. Activer le mode recovery
2. CrÃ©er une backup complÃ¨te
3. Analyser la corruption (`db-analyze`)
4. RÃ©parer les enregistrements corrompus (`db-recovery-session execute`)
5. VÃ©rifier l'intÃ©gritÃ© (`db-verify-integrity`)
6. DÃ©sactiver le mode recovery
7. VÃ©rifier le fonctionnement normal

### 5.2 Perte de connectivitÃ© avec les cores

**SymptÃ´mes :**
- Impossible de communiquer avec un core
- Timeout sur les opÃ©rations
- Erreurs de mÃ©diation BondingBrother

**ProcÃ©dure :**
1. VÃ©rifier l'Ã©tat des cores (`list-cores --status`)
2. VÃ©rifier les logs (`logs core <core_name>`)
3. RedÃ©marrer le core si nÃ©cessaire (`restart-core <core_name>`)
4. Si Ã©chec, rÃ©initialiser le core (`reset-core <core_name>`)
5. VÃ©rifier la rÃ©cupÃ©ration (`verify-core-consistency`)

**Note :** Pas besoin de mode recovery pour cette procÃ©dure.

### 5.3 Compromission de sÃ©curitÃ©

**SymptÃ´mes :**
- ActivitÃ© suspecte dÃ©tectÃ©e
- Violation d'invariants
- Niveau de confiance dÃ©gradÃ© (T3/T4)

**ProcÃ©dure :**
1. **Isolation immÃ©diate :**
   ```bash
   # Passer en mode isolation
   miyukini-admin security-isolate --reason "Compromission dÃ©tectÃ©e"
   ```

2. **Analyse de la compromission :**
   ```bash
   # Analyser les logs de sÃ©curitÃ©
   miyukini-admin security-audit --since <timestamp>
   
   # Identifier les actions suspectes
   miyukini-admin security-analyze --output compromise-report.json
   ```

3. **Activation du mode recovery si nÃ©cessaire :**
   ```bash
   # Si modification DB nÃ©cessaire
   miyukini-admin activate-recovery-mode --duration 1800
   ```

4. **Nettoyage :**
   ```bash
   # RÃ©voquer les accÃ¨s compromis
   miyukini-admin security-revoke-access --user <user_id>
   
   # RÃ©initialiser les tokens
   miyukini-admin security-reset-tokens
   ```

5. **VÃ©rification post-rÃ©cupÃ©ration :**
   ```bash
   # VÃ©rifier l'intÃ©gritÃ© systÃ¨me
   miyukini-admin verify-system-integrity
   
   # VÃ©rifier les invariants
   miyukini-admin verify-invariants
   ```

### 5.4 Perte de donnÃ©es critiques

**SymptÃ´mes :**
- DonnÃ©es manquantes dans la DB
- Ã‰chec de restauration depuis backup
- IncohÃ©rences dÃ©tectÃ©es

**ProcÃ©dure :**
1. Activer le mode recovery
2. Analyser l'Ã©tendue de la perte (`db-analyze --missing-data`)
3. Tenter restauration depuis backup (`backup-restore --backup-id <id>`)
4. Si Ã©chec, reconstruction manuelle en mode recovery
5. VÃ©rification complÃ¨te post-rÃ©cupÃ©ration
6. DÃ©sactivation du mode recovery

---

## 6. VÃ©rifications post-recovery

### 6.1 Checklist de validation

**Avant dÃ©sactivation du mode recovery :**

- [ ] IntÃ©gritÃ© DB vÃ©rifiÃ©e (`db-verify-integrity`)
- [ ] CohÃ©rence avec les cores vÃ©rifiÃ©e (`verify-core-consistency`)
- [ ] Invariants respectÃ©s (`verify-invariants`)
- [ ] Logs de recovery complets (`logs recovery --verify`)
- [ ] Backup post-recovery crÃ©Ã©e (`backup-create`)
- [ ] Tests de fonctionnement effectuÃ©s (`test-system`)

### 6.2 Tests de fonctionnement

```bash
# Tests de base
miyukini-admin test-system

# Tests spÃ©cifiques
miyukini-admin test-strongfather
miyukini-admin test-kindmother
miyukini-admin test-caringnanny
miyukini-admin test-worrysentinel
```

### 6.3 Rapport de rÃ©cupÃ©ration

**GÃ©nÃ©ration :**
```bash
miyukini-admin recovery-report \
  --session-id <session_id> \
  --output recovery-report-$(date +%Y%m%d).json
```

**Contenu du rapport :**
- RÃ©sumÃ© des opÃ©rations effectuÃ©es
- Liste des modifications DB
- VÃ©rifications effectuÃ©es
- RÃ©sultats des tests
- Recommandations post-recovery

---

## 7. Retour Ã  la normale

### 7.1 DÃ©sactivation du mode recovery

**Commande :**
```bash
miyukini-admin deactivate-recovery-mode --verify
```

**VÃ©rifications automatiques :**
- âœ… IntÃ©gritÃ© DB vÃ©rifiÃ©e
- âœ… CohÃ©rence avec les cores vÃ©rifiÃ©e
- âœ… Invariants respectÃ©s
- âœ… Tests de fonctionnement passÃ©s

**RÃ©sultat :**
- Mode recovery dÃ©sactivÃ©
- OpÃ©rateurs rÃ©activÃ©s
- Retour au mode normal
- Journalisation normale

### 7.2 Monitoring post-recovery

**DurÃ©e :** 24-48 heures aprÃ¨s rÃ©cupÃ©ration

**Actions :**
```bash
# Monitoring intensif
miyukini-admin monitor --intensive --duration 48h

# VÃ©rifications pÃ©riodiques
miyukini-admin verify-system-integrity --periodic --interval 1h
```

### 7.3 Documentation de l'incident

**Obligatoire :** Documenter l'incident et la rÃ©cupÃ©ration

**Contenu :**
- Description de l'incident
- Cause racine identifiÃ©e
- ProcÃ©dures de rÃ©cupÃ©ration appliquÃ©es
- Modifications effectuÃ©es
- LeÃ§ons apprises
- Actions prÃ©ventives recommandÃ©es

---

## 8. SÃ©curitÃ© du mode recovery

### 8.1 Authentification renforcÃ©e

**Exigences :**
- Authentification admin avec MFA
- Token recovery unique et temporaire
- Validation manuelle obligatoire
- TraÃ§abilitÃ© complÃ¨te

### 8.2 Limitations temporelles

**RÃ¨gle :** Le mode recovery est limitÃ© dans le temps (par dÃ©faut 1 heure, maximum 4 heures).

**Extension :**
```bash
# Demander une extension (nÃ©cessite nouvelle authentification)
miyukini-admin extend-recovery-mode --duration 3600 --admin-token <NEW_TOKEN>
```

### 8.3 Audit et traÃ§abilitÃ©

**Journalisation :**
- Toutes les opÃ©rations DB sont journalisÃ©es
- Toutes les commandes sont tracÃ©es
- Tous les accÃ¨s sont enregistrÃ©s
- Rapports d'audit gÃ©nÃ©rÃ©s automatiquement

**Consultation :**
```bash
# Consulter les logs de recovery
miyukini-admin logs recovery --session-id <session_id>

# GÃ©nÃ©rer un rapport d'audit
miyukini-admin audit-report --type recovery --session-id <session_id>
```

---

## 9. RÃ©fÃ©rences

- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [WorrySentinel - Security Levels Governance Contract](..//..//..//cores//WorrySentinel//contracts//levels//WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)
- [Kernel - Invariants & Guarantees](..//..//..//kernel//contracts//Kernel%20-%20Invariants%20%26%20Guarantees.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif â€” ProcÃ©dures de rÃ©cupÃ©ration en cas de compromission  
**Action requise :** ImplÃ©menter ces procÃ©dures lors du dÃ©veloppement de MiyukiniAdmin

