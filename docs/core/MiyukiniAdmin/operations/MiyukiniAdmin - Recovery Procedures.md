# MiyukiniAdmin - Recovery Procedures

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Procédures de récupération en cas de compromission  
**Portée :** Mode recovery, écriture DB directe, procédures d'urgence

---

## 1. Contexte

Ce document définit les **procédures de récupération** pour MiyukiniAdmin en cas de compromission système, d'incident critique, ou de nécessité d'intervention d'urgence.

**Références :**
- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md) : Section 6.6 Recovery Exceptionnel
- [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [WorrySentinel - Security Levels Governance Contract](../../WorrySentinel/contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)

---

## 2. Portée / Scope

Ce document définit :
- Les conditions d'activation du mode recovery
- Les procédures d'écriture DB directe
- Les étapes de récupération post-incident
- Les vérifications de sécurité post-recovery
- Les procédures de retour à la normale

Ce document **ne couvre pas** :
- Les procédures de backup/restore (voir Backup Contract)
- Les procédures de migration (voir Migration Contract)
- Les procédures de mise à jour (voir Versioning Contract)

---

## 3. Mode Recovery : Conditions d'activation

### 3.1 Conditions cumulatives strictes

Le mode recovery ne peut être activé que si **toutes** les conditions suivantes sont remplies :

| Condition | Description | Vérification |
|-----------|-------------|--------------|
| **État système ≥ Critique** | Niveau de confiance T3 ou T4 (WorrySentinel) | `worrysentinel get-trust-level` |
| **Protocole sécurité renforcée** | Mode sécurité renforcée activé explicitement | `miyukini-admin enable-enhanced-security` |
| **Intervention humaine authentifiée** | Validation manuelle obligatoire avec MFA | Authentification admin + token recovery |
| **Fenêtre temporelle limitée** | Durée maximale définie (ex. 1 heure) | Timer automatique de désactivation |
| **Journalisation complète** | Toute opération tracée | Audit log activé |
| **Revalidation obligatoire** | Vérification post-intervention | Checklist de validation |

### 3.2 Activation du mode recovery

**Commande :**
```bash
# Vérifier les conditions préalables
miyukini-admin check-recovery-conditions

# Activer le mode recovery (nécessite authentification MFA)
miyukini-admin activate-recovery-mode \
  --duration 3600 \
  --reason "Corruption DB critique détectée" \
  --admin-token <MFA_TOKEN>
```

**Vérifications automatiques :**
- ✅ Niveau de confiance T3 ou T4
- ✅ Protocole sécurité renforcée activé
- ✅ Authentification admin valide
- ✅ Token recovery valide
- ✅ Audit log opérationnel

**Résultat :**
- Mode recovery activé pour la durée spécifiée
- Blocage des Opérateurs pendant l'opération
- Journalisation complète activée
- Timer de désactivation automatique démarré

---

## 4. Écriture DB directe en mode recovery

### 4.1 Limitations

**Règle :** L'écriture DB directe n'est autorisée qu'en mode recovery et uniquement pour :
- Réparation de corruption de données
- Restauration d'intégrité critique
- Correction d'erreurs système bloquantes

**Interdit :**
- ❌ Modification de données métier normales
- ❌ Bypass des règles de gouvernance
- ❌ Modification des politiques StrongFather
- ❌ Modification des configurations WorrySentinel

### 4.2 Procédure d'écriture DB directe

#### Étape 1 : Préparation

```bash
# Sauvegarder l'état actuel
miyukini-admin backup-create --label "pre-recovery-$(date +%Y%m%d-%H%M%S)"

# Vérifier l'intégrité de la backup
miyukini-admin backup-verify --latest

# Activer le mode recovery
miyukini-admin activate-recovery-mode --duration 3600
```

#### Étape 2 : Analyse

```bash
# Analyser la corruption
miyukini-admin db-analyze --table <table_name>

# Identifier les enregistrements corrompus
miyukini-admin db-verify-integrity

# Générer un rapport d'analyse
miyukini-admin db-report --output recovery-analysis.json
```

#### Étape 3 : Écriture DB directe

```bash
# Ouvrir une session DB en mode recovery
miyukini-admin db-recovery-session start

# Exécuter les corrections (exemple)
miyukini-admin db-recovery-session execute \
  --sql "UPDATE core_registry SET status = 'operational' WHERE id = '...'"

# Vérifier chaque modification
miyukini-admin db-recovery-session verify --last-operation
```

**Contraintes :**
- Chaque opération SQL est journalisée
- Chaque opération nécessite une confirmation explicite
- Les opérations sont validées avant exécution
- Les rollback sont possibles pour chaque opération

#### Étape 4 : Validation

```bash
# Vérifier l'intégrité post-modification
miyukini-admin db-verify-integrity

# Vérifier la cohérence avec les cores
miyukini-admin verify-core-consistency

# Générer un rapport de validation
miyukini-admin db-report --output recovery-validation.json
```

#### Étape 5 : Fermeture de la session

```bash
# Fermer la session recovery
miyukini-admin db-recovery-session close

# Désactiver le mode recovery
miyukini-admin deactivate-recovery-mode
```

---

## 5. Procédures de récupération par type d'incident

### 5.1 Corruption de base de données

**Symptômes :**
- Erreurs SQLite lors des opérations
- Incohérences dans les données
- Échec de vérification d'intégrité

**Procédure :**
1. Activer le mode recovery
2. Créer une backup complète
3. Analyser la corruption (`db-analyze`)
4. Réparer les enregistrements corrompus (`db-recovery-session execute`)
5. Vérifier l'intégrité (`db-verify-integrity`)
6. Désactiver le mode recovery
7. Vérifier le fonctionnement normal

### 5.2 Perte de connectivité avec les cores

**Symptômes :**
- Impossible de communiquer avec un core
- Timeout sur les opérations
- Erreurs de médiation BondingBrother

**Procédure :**
1. Vérifier l'état des cores (`list-cores --status`)
2. Vérifier les logs (`logs core <core_name>`)
3. Redémarrer le core si nécessaire (`restart-core <core_name>`)
4. Si échec, réinitialiser le core (`reset-core <core_name>`)
5. Vérifier la récupération (`verify-core-consistency`)

**Note :** Pas besoin de mode recovery pour cette procédure.

### 5.3 Compromission de sécurité

**Symptômes :**
- Activité suspecte détectée
- Violation d'invariants
- Niveau de confiance dégradé (T3/T4)

**Procédure :**
1. **Isolation immédiate :**
   ```bash
   # Passer en mode isolation
   miyukini-admin security-isolate --reason "Compromission détectée"
   ```

2. **Analyse de la compromission :**
   ```bash
   # Analyser les logs de sécurité
   miyukini-admin security-audit --since <timestamp>
   
   # Identifier les actions suspectes
   miyukini-admin security-analyze --output compromise-report.json
   ```

3. **Activation du mode recovery si nécessaire :**
   ```bash
   # Si modification DB nécessaire
   miyukini-admin activate-recovery-mode --duration 1800
   ```

4. **Nettoyage :**
   ```bash
   # Révoquer les accès compromis
   miyukini-admin security-revoke-access --user <user_id>
   
   # Réinitialiser les tokens
   miyukini-admin security-reset-tokens
   ```

5. **Vérification post-récupération :**
   ```bash
   # Vérifier l'intégrité système
   miyukini-admin verify-system-integrity
   
   # Vérifier les invariants
   miyukini-admin verify-invariants
   ```

### 5.4 Perte de données critiques

**Symptômes :**
- Données manquantes dans la DB
- Échec de restauration depuis backup
- Incohérences détectées

**Procédure :**
1. Activer le mode recovery
2. Analyser l'étendue de la perte (`db-analyze --missing-data`)
3. Tenter restauration depuis backup (`backup-restore --backup-id <id>`)
4. Si échec, reconstruction manuelle en mode recovery
5. Vérification complète post-récupération
6. Désactivation du mode recovery

---

## 6. Vérifications post-recovery

### 6.1 Checklist de validation

**Avant désactivation du mode recovery :**

- [ ] Intégrité DB vérifiée (`db-verify-integrity`)
- [ ] Cohérence avec les cores vérifiée (`verify-core-consistency`)
- [ ] Invariants respectés (`verify-invariants`)
- [ ] Logs de recovery complets (`logs recovery --verify`)
- [ ] Backup post-recovery créée (`backup-create`)
- [ ] Tests de fonctionnement effectués (`test-system`)

### 6.2 Tests de fonctionnement

```bash
# Tests de base
miyukini-admin test-system

# Tests spécifiques
miyukini-admin test-strongfather
miyukini-admin test-kindmother
miyukini-admin test-caringnanny
miyukini-admin test-worrysentinel
```

### 6.3 Rapport de récupération

**Génération :**
```bash
miyukini-admin recovery-report \
  --session-id <session_id> \
  --output recovery-report-$(date +%Y%m%d).json
```

**Contenu du rapport :**
- Résumé des opérations effectuées
- Liste des modifications DB
- Vérifications effectuées
- Résultats des tests
- Recommandations post-recovery

---

## 7. Retour à la normale

### 7.1 Désactivation du mode recovery

**Commande :**
```bash
miyukini-admin deactivate-recovery-mode --verify
```

**Vérifications automatiques :**
- ✅ Intégrité DB vérifiée
- ✅ Cohérence avec les cores vérifiée
- ✅ Invariants respectés
- ✅ Tests de fonctionnement passés

**Résultat :**
- Mode recovery désactivé
- Opérateurs réactivés
- Retour au mode normal
- Journalisation normale

### 7.2 Monitoring post-recovery

**Durée :** 24-48 heures après récupération

**Actions :**
```bash
# Monitoring intensif
miyukini-admin monitor --intensive --duration 48h

# Vérifications périodiques
miyukini-admin verify-system-integrity --periodic --interval 1h
```

### 7.3 Documentation de l'incident

**Obligatoire :** Documenter l'incident et la récupération

**Contenu :**
- Description de l'incident
- Cause racine identifiée
- Procédures de récupération appliquées
- Modifications effectuées
- Leçons apprises
- Actions préventives recommandées

---

## 8. Sécurité du mode recovery

### 8.1 Authentification renforcée

**Exigences :**
- Authentification admin avec MFA
- Token recovery unique et temporaire
- Validation manuelle obligatoire
- Traçabilité complète

### 8.2 Limitations temporelles

**Règle :** Le mode recovery est limité dans le temps (par défaut 1 heure, maximum 4 heures).

**Extension :**
```bash
# Demander une extension (nécessite nouvelle authentification)
miyukini-admin extend-recovery-mode --duration 3600 --admin-token <NEW_TOKEN>
```

### 8.3 Audit et traçabilité

**Journalisation :**
- Toutes les opérations DB sont journalisées
- Toutes les commandes sont tracées
- Tous les accès sont enregistrés
- Rapports d'audit générés automatiquement

**Consultation :**
```bash
# Consulter les logs de recovery
miyukini-admin logs recovery --session-id <session_id>

# Générer un rapport d'audit
miyukini-admin audit-report --type recovery --session-id <session_id>
```

---

## 9. Références

- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [WorrySentinel - Security Levels Governance Contract](../../WorrySentinel/contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)
- [Kernel - Invariants & Guarantees](../../kernel/contracts/Kernel%20-%20Invariants%20%26%20Guarantees.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Procédures de récupération en cas de compromission  
**Action requise :** Implémenter ces procédures lors du développement de MiyukiniAdmin
