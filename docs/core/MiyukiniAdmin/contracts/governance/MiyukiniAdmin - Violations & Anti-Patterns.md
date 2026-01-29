# MiyukiniAdmin — Violations & Anti-Patterns

## 1. Contexte

Ce document catalogue les **violations** des invariants MiyukiniAdmin et les **anti-patterns** a eviter. Il sert de guide pour identifier, prevenir et corriger les erreurs architecturales.

## 2. Portee / Scope

Ce document definit :
- Les violations cataloguees par type
- Les anti-patterns courants
- Les signaux d'alerte
- Les procedures de correction

Ce document **ne couvre pas** :
- La definition des invariants (voir Invariants & Guarantees)
- Les procedures de remediation detaillees
- Les sanctions organisationnelles

---

## 3. Catalogue des Violations

### 3.1 Violations d'Independance

#### VIOL-MA-001 : Dependance Operateur vers MiyukiniAdmin

**Invariant viole :** INV-MA-1

**Description :**
Un Operateur importe ou appelle directement MiyukiniAdmin.

**Exemple de violation :**
```rust
// Dans un Operateur
use miyukini_admin::monitoring;  // VIOLATION!

fn check_system() {
    monitoring::get_metrics();  // VIOLATION!
}
```

**Impact :**
- Couplage fort inapproprie
- Risque de cascade de defaillances
- Violation du principe de console root

**Detection :**
- Analyse statique des imports
- Revue de code

**Correction :**
- Supprimer la dependance
- Utiliser les mecanismes standards (BondingBrother + cores)

---

#### VIOL-MA-002 : Consommation d'Outil par MiyukiniAdmin

**Invariant viole :** INV-MA-2

**Description :**
MiyukiniAdmin utilise un Outil ou Kit d'Outils du catalogue.

**Exemple de violation :**
```rust
// Dans MiyukiniAdmin
use toolkit_ui::components;  // VIOLATION!
use tool_auth::verify;       // VIOLATION!
```

**Impact :**
- Perte d'auto-suffisance
- Dependance circulaire potentielle
- Risque de blocage

**Detection :**
- Analyse des dependances cargo/npm
- Verification contre catalogue MasterButler

**Correction :**
- Implementer la fonctionnalite en interne
- Pas de raccourci vers les Outils

---

#### VIOL-MA-003 : Exposition API Publique

**Invariant viole :** INV-MA-3

**Description :**
MiyukiniAdmin expose un endpoint accessible depuis l'exterieur.

**Exemple de violation :**
```yaml
# Configuration exposant un port public
server:
  bind: 0.0.0.0:8080  # VIOLATION - accessible publiquement
```

**Impact :**
- Surface d'attaque elargie
- Risque de compromission
- Violation du modele de securite

**Detection :**
- Scan de ports
- Audit configuration reseau

**Correction :**
- Bind sur localhost uniquement
- Acces via VPN/tunnel

---

### 3.2 Violations de Communication

#### VIOL-MA-004 : Bypass BondingBrother

**Invariant viole :** INV-MA-4

**Description :**
MiyukiniAdmin accede directement a un core sans passer par BondingBrother.

**Exemple de violation :**
```rust
// Acces direct au core
let decision = strong_father::decide(intent);  // VIOLATION!

// Correct
let decision = bonding_brother.request_decision(intent);
```

**Impact :**
- Perte de tracabilite
- Bypass des validations
- Violation de l'architecture

**Detection :**
- Analyse des appels
- Verification logs BondingBrother

**Correction :**
- Router tous les appels via BondingBrother
- Pas d'import direct des cores

---

#### VIOL-MA-005 : Action Non Tracee

**Invariant viole :** INV-MA-5

**Description :**
Une action est effectuee sans etre enregistree dans l'audit log.

**Exemple de violation :**
```rust
// Action sans audit
fn change_config(key: &str, value: &str) {
    config.set(key, value);  // VIOLATION - pas de trace
}

// Correct
fn change_config(key: &str, value: &str) {
    audit_log.record(Action::ConfigChange { key, value, operator });
    config.set(key, value);
}
```

**Impact :**
- Perte de responsabilite
- Impossible de reconstruire l'historique
- Non-conformite audit

**Detection :**
- Test de couverture audit
- Revue de code

**Correction :**
- Ajouter logging systematique
- Decorator/middleware obligatoire

---

#### VIOL-MA-006 : Action Silencieuse

**Invariant viole :** INV-MA-10

**Description :**
Une action est effectuee sans feedback visible ou sans confirmation explicite.

**Exemple de violation :**
```rust
// Action automatique sans feedback
fn auto_cleanup() {
    delete_old_logs();  // VIOLATION - silencieux
}
```

**Impact :**
- Actions non voulues
- Perte de controle utilisateur
- Difficulte de diagnostic

**Detection :**
- Tests UI
- Revue UX

**Correction :**
- Toujours notifier l'utilisateur
- Confirmer avant actions destructives

---

### 3.3 Violations de Donnees

#### VIOL-MA-007 : Acces DB Direct Sans Conditions

**Invariant viole :** INV-MA-6

**Description :**
Acces direct a la DB sans que TOUTES les conditions cumulatives soient reunies.

**Exemple de violation :**
```rust
// Acces direct sans verification complete
fn quick_fix(query: &str) {
    db.execute(query);  // VIOLATION - conditions non verifiees
}
```

**Conditions requises (TOUTES) :**
1. Etat systeme >= T3
2. Protocole securite renforce
3. MFA valide
4. Fenetre temporelle active
5. Approbation StrongFather
6. Journalisation active

**Impact :**
- Corruption potentielle des donnees
- Bypass des validations KindMother
- Perte d'integrite

**Detection :**
- Verification pre-execution
- Audit des acces DB

**Correction :**
- Implementer verification stricte des 6 conditions
- Rejeter si une condition manque

---

#### VIOL-MA-008 : Logique Metier Applicative

**Invariant viole :** INV-MA-8

**Description :**
MiyukiniAdmin contient de la logique metier qui devrait etre dans un Operateur.

**Exemple de violation :**
```rust
// Logique metier dans MiyukiniAdmin
fn calculate_subscription_price(plan: Plan, duration: Duration) -> Price {
    // VIOLATION - c'est de la logique metier applicative
}
```

**Impact :**
- Violation separation des preoccupations
- Couplage inapproprie
- Difficulte de maintenance

**Detection :**
- Revue de code
- Analyse semantique

**Correction :**
- Deplacer vers l'Operateur concerne
- MiyukiniAdmin = admin technique uniquement

---

### 3.4 Violations d'Architecture

#### VIOL-MA-009 : Composant UI Partage

**Invariant viole :** INV-MA-7

**Description :**
Un composant UI de MiyukiniAdmin est reutilise par un autre Operateur.

**Exemple de violation :**
```typescript
// Dans un Operateur
import { AdminButton } from 'miyukini-admin/ui';  // VIOLATION!
```

**Impact :**
- Couplage UI inapproprie
- Risque de propagation de changements
- Violation frontiere de securite

**Detection :**
- Analyse des imports
- Scan des exports

**Correction :**
- Marquer tous les exports comme privates
- Pas d'exposition de composants UI

---

#### VIOL-MA-010 : Dependance Externe Critique

**Invariant viole :** INV-MA-9

**Description :**
MiyukiniAdmin depend d'un service externe pour son fonctionnement de base.

**Exemple de violation :**
```rust
// Dependance a un service externe
async fn start() {
    let auth = external_auth_service.connect().await?;  // VIOLATION
    // Ne peut pas demarrer sans le service externe
}
```

**Impact :**
- Incapacite de fonctionnement autonome
- Dependance a des services tiers
- Risque de blocage

**Detection :**
- Test de demarrage isole
- Analyse des dependances

**Correction :**
- Implementer authentification en interne
- Fonctionnalites de base sans externe

---

## 4. Anti-Patterns

### 4.1 Anti-Pattern : "Admin Everywhere"

**Description :**
Utiliser MiyukiniAdmin pour des taches qui devraient etre dans des Operateurs.

**Symptomes :**
- MiyukiniAdmin devient le point central de tout
- Logique metier dans admin
- Operateurs appellent admin

**Correction :**
- MiyukiniAdmin = monitoring + maintenance + securite
- Logique metier = Operateurs

---

### 4.2 Anti-Pattern : "Direct Database"

**Description :**
Acceder a la DB directement au lieu de passer par KindMother.

**Symptomes :**
- Requetes SQL brutes frequentes
- Bypass du mode recovery
- Modifications non tracees

**Correction :**
- Toujours via KindMother (mode normal)
- Recovery uniquement pour urgences

---

### 4.3 Anti-Pattern : "Silent Admin"

**Description :**
Effectuer des actions sans feedback ni confirmation.

**Symptomes :**
- Actions automatiques sans notification
- Pas de confirmation pour operations critiques
- Utilisateur surpris par des changements

**Correction :**
- Notifier toute action
- Confirmer avant modification
- Log visible en temps reel

---

### 4.4 Anti-Pattern : "Security Theater"

**Description :**
Avoir des controles de securite apparents mais inefficaces.

**Symptomes :**
- MFA desactivable
- Logs non verifies
- Conditions recovery ignorables

**Correction :**
- Controles non bypassables
- Verification systematique
- Audit regulier

---

### 4.5 Anti-Pattern : "God Admin"

**Description :**
Un seul compte admin avec tous les pouvoirs.

**Symptomes :**
- Un seul utilisateur admin
- Pas de separation des roles
- Actions critiques sans second regard

**Correction :**
- Roles separes (Viewer, Operator, Admin, Recovery)
- Principe du moindre privilege
- Approbation multiple pour critiques

---

## 5. Signaux d'Alerte

### 5.1 Signaux dans le Code

| Signal | Violation probable |
|--------|-------------------|
| Import `miyukini_admin` dans Operateur | VIOL-MA-001 |
| Import de toolkit dans admin | VIOL-MA-002 |
| Appel direct a core sans BB | VIOL-MA-004 |
| Action sans `audit_log.record` | VIOL-MA-005 |
| `db.execute` sans verification conditions | VIOL-MA-007 |

### 5.2 Signaux dans les Logs

| Signal | Violation probable |
|--------|-------------------|
| Acces DB sans entry recovery | VIOL-MA-007 |
| Action sans operator_id | VIOL-MA-005 |
| Requete core sans trace BB | VIOL-MA-004 |

### 5.3 Signaux dans l'Architecture

| Signal | Violation probable |
|--------|-------------------|
| Port expose publiquement | VIOL-MA-003 |
| Composant UI exporte | VIOL-MA-009 |
| Service externe dans critical path | VIOL-MA-010 |

---

## 6. Procedures de Detection

### 6.1 Analyse Statique

```bash
# Verifier imports inappropries
grep -r "miyukini_admin" src/operators/

# Verifier exports UI
grep -r "export.*from.*miyukini-admin/ui" 

# Verifier appels directs aux cores
grep -r "strong_father::" src/miyukini_admin/
```

### 6.2 Tests Automatises

```rust
#[test]
fn test_no_operator_depends_on_admin() {
    for operator in list_operators() {
        assert!(!operator.dependencies.contains("miyukini_admin"));
    }
}

#[test]
fn test_all_actions_are_audited() {
    // Simuler actions et verifier presence dans audit log
}
```

### 6.3 Revue de Code

Checklist obligatoire :
- [ ] Pas d'import miyukini_admin dans Operateur
- [ ] Pas d'import toolkit dans admin
- [ ] Tous les appels via BondingBrother
- [ ] Toutes les actions auditees
- [ ] Pas de logique metier applicative

---

## 7. Documents Associes

- [MiyukiniAdmin - Invariants & Guarantees](./MiyukiniAdmin%20-%20Invariants%20&%20Guarantees.md)
- [MiyukiniAdmin - Documentation Fondatrice](../../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Threat Model Contract](../security/MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document de reference
