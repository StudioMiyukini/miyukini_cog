# TAMR - Examples Interventions

## Contexte

Ce document illustre TAMR par des **exemples d'interventions** (approbation, override, escalade, supervision). Il fait partie de la documentation de reference et est **informativ** (non contractuel).

**Documents connexes :**
- [TAMR - Intervention Types Contract](../contracts/intervention/TAMR%20-%20Intervention%20Types%20Contract.md)
- [TAMR - Trace Contract](../contracts/audit/TAMR%20-%20Trace%20Contract.md)
- [TAMR - Architecture & Flows](../architecture/TAMR%20-%20Architecture%20&%20Flows.md)

**Terminologie :** [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 1. Exemple APPROVAL (Approbation)

### Contexte

Publication d'un article : le systeme atteint un point d'approbation ; un editeur doit valider avant mise en ligne.

### Scenario

1. Le processus "publication" atteint le point d'intervention `IP-CONTENT-PUBLISH-001` (categorie DECISION_GATE).
2. Une demande d'approbation est creee ; l'intention transite par BondingBrother ; StrongFather autorise l'approbation pour l'utilisateur `editor-alice`.
3. Le produit notifie `editor-alice` ; elle ouvre l'interface et approuve.
4. Une trace est emise et persistee (KindMother).

### Trace (structure conceptuelle)

| Champ | Valeur |
|-------|--------|
| `intervention_id` | int-2026-01-28-approval-001 |
| `type` | APPROVAL |
| `approver_id` | editor-alice |
| `requested_at` | 2026-01-28T10:00:00Z (local) |
| `resolved_at` | 2026-01-28T10:05:00Z (local) |
| `result` | APPROUVÃ‰ |
| `subject` | Publication article article-12345 |
| `context` | process_id=CONTENT-LIFECYCLE, point_id=IP-CONTENT-PUBLISH-001 |

### Cas refus

Si `editor-alice` refuse : `result` = REFUSÃ‰ ; le processus n'execute pas la publication.

### Cas expiration

Si aucun retour avant timeout (ex. 24h) : `result` = EXPIRÃ‰ ; le comportement par defaut (ex. refus par defaut) est applique et trace.

---

## 2. Exemple OVERRIDE (Derogation)

### Contexte

Une action automatiquement refusee (ex. depot hors quota) ; un manager demande un override pour raison exceptionnelle.

### Scenario

1. StrongFather a emis une decision REFUSEE pour l'action "depot fichier" (quota depasse).
2. L'utilisateur `manager-bob` demande un override (type FORCE) avec justification.
3. L'intention transite par BondingBrother ; StrongFather verifie les limites infranchissables (ex. pas de suppression de donnees de securite) et autorise l'override.
4. La trace est emise avec justification obligatoire.

### Trace (structure conceptuelle)

| Champ | Valeur |
|-------|--------|
| `intervention_id` | int-2026-01-28-override-001 |
| `type` | OVERRIDE |
| `override_type` | FORCE |
| `overrider_id` | manager-bob |
| `justification` | "Campagne Q1 exceptionnelle, accord direction, quota etendu temporairement." |
| `original_decision` | REFUSEE (quota depasse) |
| `overridden_at` | 2026-01-28T11:00:00Z (local) |
| `subject` | Depot fichier campaign-Q1.xlsx |
| `context` | process_id=UPLOAD, decision_id=dec-456 |
| `limits_checked` | true |

### Cas limite infranchissable

Si l'override avait concerne une action protegee (ex. desactivation d'une regle de securite critique), StrongFather aurait refuse ; la trace de refus serait enregistree (sans execution de l'override).

---

## 3. Exemple ESCALATION (Escalade)

### Contexte

Un validateur doute de la decision a prendre (contenu sensible) ; il escalade vers son superviseur.

### Scenario

1. Le point d'intervention est atteint ; l'utilisateur `validator-carol` initie une escalade avec motif "contenu a la frontiere politique, demande arbitrage niveau 2".
2. L'intention transite par BondingBrother ; StrongFather autorise l'escalade vers le niveau 2 (superviseur).
3. Le produit notifie le superviseur `supervisor-dave` ; timeout 48h ; comportement par defaut : REFUSED_BY_DEFAULT.
4. `supervisor-dave` repond dans le delai : approbation avec reserve. La trace est completee.

### Trace (structure conceptuelle)

| Champ | Valeur |
|-------|--------|
| `intervention_id` | int-2026-01-28-escalation-001 |
| `type` | ESCALATION |
| `initiator_id` | validator-carol |
| `motif` | Contenu a la frontiere politique, demande arbitrage niveau 2 |
| `escalation_path` | [niveau_1, niveau_2] |
| `current_level` | 2 |
| `initiated_at` | 2026-01-28T12:00:00Z (local) |
| `resolved_at` | 2026-01-28T14:00:00Z (local) |
| `resolver_id` | supervisor-dave |
| `resolution` | APPROVED_WITH_RESERVE |
| `subject` | Validation article article-67890 |
| `context` | process_id=CONTENT-LIFECYCLE, point_id=IP-CONTENT-SENSITIVE |
| `timeout_behavior` | REFUSED_BY_DEFAULT |

### Cas timeout

Si `supervisor-dave` ne repond pas avant 48h : le comportement par defaut REFUSED_BY_DEFAULT est applique ; la trace contient `resolution` = REFUSED_BY_DEFAULT, `resolved_at` = moment du timeout. Le flux reprend sans blocage (INV-TAMR-8).

---

## 4. Exemple SUPERVISION (Supervision)

### Contexte

Un administrateur surveille les acces sensibles pendant une fenetre de maintenance.

### Scenario

1. L'utilisateur `admin-eve` active une supervision : perimetre = "acces donnees sensibles", duree prevue = 2h.
2. La trace de debut est emise. Pendant 2h, `admin-eve` observe ; a 1h, elle declenche une intervention OVERRIDE (blocage d'un acces suspect) avec justification.
3. A 2h, la supervision se termine explicitement.

### Trace supervision (structure conceptuelle)

| Champ | Valeur |
|-------|--------|
| `intervention_id` | int-2026-01-28-supervision-001 |
| `type` | SUPERVISION |
| `supervisor_id` | admin-eve |
| `scope` | Acces donnees sensibles (module AUDIT) |
| `started_at` | 2026-01-28T15:00:00Z (local) |
| `ended_at` | 2026-01-28T17:00:00Z (local) |
| `end_reason` | explicit |
| `duration_planned` | 2h |
| `context` | maintenance window M-2026-01-28 |
| `interventions_triggered` | [int-2026-01-28-override-002] |

L'intervention OVERRIDE declenchee pendant la supervision a sa propre trace (int-2026-01-28-override-002), liee par `interventions_triggered`.

---

## 5. Resume

| Type | Moment | Acteur typique | Resultat trace |
|------|--------|----------------|----------------|
| APPROVAL | Avant action | Approbateur designe | APPROUVÃ‰ / REFUSÃ‰ / EXPIRÃ‰ |
| OVERRIDE | Apres decision auto | Humain autorise | FORCE / BLOCK + justification |
| ESCALATION | En cours | Initiateur + niveau superieur | resolution + timeout_behavior |
| SUPERVISION | Continu | Superviseur | started_at, ended_at, interventions_triggered |

Tous les exemples supposent une implementation conforme aux contrats TAMR (Trace Contract, Intervention Types Contract, Inviolable Limits, INV-TAMR-8 pour escalade).

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** INFORMATIF  
**Reference :** TAMR Intervention Types Contract, Trace Contract

