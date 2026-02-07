# MiyukiniAdmin — Backup & Restore Contract

## 1. Contexte

Ce document definit le contrat pour les **operations de sauvegarde et de restauration** de la base de donnees dans MiyukiniAdmin. Ces operations sont effectuees **via KindMother**, sous validation **StrongFather**, et respectent les **Niveaux de securite (0-4)** et **Etats de confiance (T0-T4)** gouvernes par WorrySentinel.

## 2. Portee / Scope

Ce document definit :
- Le declenchement des sauvegardes (manuel, planifie)
- Le stockage des sauvegardes
- Les conditions et le workflow de restauration
- La traçabilite obligatoire

Ce document **ne couvre pas** :
- Le mode recovery (voir Emergency DB Access Contract)
- Les metriques DB (voir DB Metrics Contract)

---

## 3. Principe fondamental

> **Toute sauvegarde et toute restauration passent par KindMother. La decision de declencher ou d'autoriser une restauration releve de StrongFather.**

---

## 4. Operations

### 4.1 Sauvegarde (BACKUP)

| Operation | Description | Validation StrongFather |
|-----------|-------------|--------------------------|
| **BACKUP-001** | Sauvegarde complete (dump logical) | Oui (si declenchement manuel depuis MiyukiniAdmin) |
| **BACKUP-002** | Sauvegarde incrementale (si implementee) | Oui |

**Declenchement :**
- **Manuel** : depuis MiyukiniAdmin (UI ou API interne) ; justification optionnelle selon niveau securite.
- **Planifie** : cron / job ; traçabilite automatique ; pas de validation humaine a chaque execution si politique le prevoit.

**Stockage :** hors DB (fichier ou objet) ; pas de stockage des backups dans la meme instance que la DB cible.

### 4.2 Restauration (RESTORE)

| Operation | Description | Validation StrongFather |
|-----------|-------------|--------------------------|
| **RESTORE-001** | Restauration depuis un backup | Oui (obligatoire) |

**Conditions :**
- Niveau de confiance et niveau de securite compatibles (WorrySentinel).
- Justification obligatoire pour toute restauration (risque de perte de donnees recentes).
- Backup cible identifie et accessible.

**Workflow :** Pre-validation → Blocage operations concurrentes (CaringNanny) → Restauration via KindMother → Post-validation → Deblocage.

---

## 5. Traçabilite

| Champ | Description |
|-------|-------------|
| `operation_id` | Identifiant unique |
| `timestamp` | Horodatage |
| `operator_id` | Operateur humain (MiyukiniAdmin) |
| `operation_type` | BACKUP-001 / BACKUP-002 / RESTORE-001 |
| `target` | Chemin / identifiant du backup |
| `result` | SUCCESS / FAIL |
| `duration_ms` | Duree d'execution |
| `justification` | Si requise |

**Retention :** Permanent pour RESTORE ; 1 an minimum pour BACKUP (selon politique).

---

## 6. Documents associes

- [MiyukiniAdmin - Gestion DB type Supabase](../reference/MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)
- [MiyukiniAdmin - DB Operations Contract](./MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [MiyukiniAdmin - Emergency DB Access Contract](./MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Contrat de reference
