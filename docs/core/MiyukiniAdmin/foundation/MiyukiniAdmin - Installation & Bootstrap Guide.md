# MiyukiniAdmin - Installation & Bootstrap Guide

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Guide d'installation et de bootstrap  
**Portée :** Installation initiale, bootstrap, résolution des dépendances circulaires

---

## 1. Contexte

Ce document définit le **processus d'installation et de bootstrap** de MiyukiniAdmin, l'Opérateur Souverain (Strate 9) de l'écosystème Miyukini Core System.

**Problématique identifiée :** MiyukiniAdmin nécessite BondingBrother pour fonctionner, mais BondingBrother nécessite MiyukiniAdmin pour son installation initiale. Ce document résout cette **dépendance circulaire** en définissant un processus de bootstrap explicite.

**Références :**
- [MiyukiniAdmin - Documentation Fondatrice](./MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) — détection environnement vierge, verrou StrongFather, Futur Admin, compte admin
- [MiyukiniAdmin - Environment Identity Protocol EIP](../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md) — identité environnement chiffrée (EIP)
- [BondingBrother - Documentation Fondatrice](../BondingBrother/foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Kernel - Definition](../Kernel/Miyukini%20Core%20System%20-%20Definition%20Kernel.md)

---

## 2. Portée / Scope

Ce document couvre :
- Prérequis matériels et logiciels
- **Premier démarrage** : détection environnement vierge vs initialisé, verrou StrongFather, parcours Futur Admin et création du compte admin (détail dans [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md))
- **Identité environnement** : production des données d'identité du COG de façon chiffrée par les Cores (protocole [EIP](../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md))
- Processus d'installation étape par étape
- Bootstrap initial (résolution dépendance circulaire)
- Vérification post-installation
- Cas d'erreur et récupération

Ce document **ne couvre pas** :
- Mise à jour de MiyukiniAdmin (voir Versioning & Evolution Contract)
- Migration entre environnements (voir Migration & Compatibility Contract)
- Configuration avancée (voir Configuration Contract)

---

## 3. Prérequis

### 3.1 Matériel

| Composant | Minimum | Recommandé |
|-----------|---------|------------|
| **CPU** | 2 cores | 4+ cores |
| **RAM** | 4 Go | 8+ Go |
| **Disque** | 20 Go libres | 50+ Go libres (SSD recommandé) |
| **Réseau** | Optionnel (offline-first) | Connexion pour synchronisation |

### 3.2 Logiciel

| Composant | Version | Notes |
|-----------|---------|-------|
| **OS** | Linux (Ubuntu 22.04+, Debian 11+), macOS 12+, Windows 11+ | — |
| **Rust** | 1.75+ | Installé via rustup |
| **SQLite** | 3.35+ | Inclus dans KindMother |
| **Permissions** | Accès lecture/écriture au répertoire d'installation | — |

### 3.3 Environnement

- Variables d'environnement : aucune requise pour le bootstrap minimal
- Ports réseau : aucun port requis pour le bootstrap (offline-first)
- Services externes : aucun service externe requis (conforme LOI-1)

---

## 4. Processus d'installation

### 4.1 Phase 0 : Préparation

#### Étape 0.1 : Vérification du système

```bash
# Vérifier Rust
rustc --version  # Doit être >= 1.75

# Vérifier l'espace disque
df -h  # Vérifier au moins 20 Go libres

# Vérifier les permissions
mkdir -p /tmp/miyukini_test && rmdir /tmp/miyukini_test
```

#### Étape 0.2 : Téléchargement

```bash
# Cloner le repository (ou télécharger l'archive)
git clone https://github.com/miyukini/miyukini-kernel.git
cd miyukini-kernel

# Ou télécharger l'archive release
# wget https://github.com/miyukini/miyukini-kernel/releases/v1.0.0/miyukini-admin.tar.gz
# tar -xzf miyukini-admin.tar.gz
```

---

### 4.2 Phase 1 : Installation du Kernel

**Objectif :** Installer le Kernel (Strate 0) qui est la fondation technique minimale.

#### Étape 1.1 : Compilation du Kernel

```bash
cd crates/miyukini-kernel
cargo build --release
```

#### Étape 1.2 : Vérification du Kernel

```bash
# Exécuter les tests unitaires
cargo test

# Vérifier les invariants
cargo test --test invariants_tests
```

**Critères de succès :**
- ✅ Compilation sans erreur
- ✅ Tous les tests passent
- ✅ Aucune dépendance externe requise (vérification offline)

---

### 4.3 Phase 2 : Bootstrap initial (Résolution dépendance circulaire)

**Problème :** MiyukiniAdmin nécessite BondingBrother, mais BondingBrother nécessite MiyukiniAdmin pour son installation.

**Solution :** Bootstrap en **mode minimal** sans BondingBrother, puis activation progressive.

**État de l'environnement au démarrage :** Au lancement, MiyukiniAdmin détermine si l'environnement est **vierge** (données critiques absentes ou invalides) ou **déjà initialisé**. En environnement vierge, StrongFather applique un **verrou bootstrap** : seuls MiyukiniAdmin et les Cores peuvent agir ; l'utilisateur est traité comme **Futur Admin** et dirigé vers le **parcours d'installation** (création compte admin, génération identité environnement EIP, config minimale). Détail complet : [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md). Identité environnement chiffrée : [Environment Identity Protocol EIP](../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md).

#### Étape 2.1 : Installation de MiyukiniAdmin en mode bootstrap

```bash
cd crates/miyukini-admin

# Compilation en mode bootstrap (sans dépendance BondingBrother)
cargo build --release --features bootstrap

# Ou via script d'installation
./scripts/bootstrap.sh
```

**Mode bootstrap :**
- ✅ Kernel initialisé
- ✅ Base de données locale créée (SQLite)
- ✅ Identités système générées
- ✅ Configuration minimale chargée
- ⚠️ BondingBrother **non activé** (mode dégradé)

#### Étape 2.2 : Initialisation de la base de données

```bash
# Exécuter les migrations SQL initiales
./miyukini-admin migrate init

# Vérifier la création des tables
sqlite3 data/miyukini.db ".tables"
```

**Tables créées :**
- `kernel_config` : Configuration système
- `system_identities` : Identités système générées
- `bootstrap_log` : Journal du bootstrap
- `core_registry` : Registre des cores (vide initialement)

#### Étape 2.3 : Génération des identités système

```bash
# Générer les identités système (via Kernel)
./miyukini-admin generate-identities

# Vérifier les identités
./miyukini-admin list-identities
```

**Identités générées :**
- `system_admin_id` : Identité de l'administrateur système
- `environment_id` : Identité de l'environnement
- `bootstrap_token` : Token temporaire pour activation BondingBrother

---

### 4.4 Phase 3 : Installation des Cores (sans BondingBrother)

**Objectif :** Installer les cores fondamentaux nécessaires avant BondingBrother.

#### Étape 3.1 : Installation de StrongFather

```bash
cd crates/miyukini-strongfather
cargo build --release
cargo test

# Enregistrement dans MiyukiniAdmin
../miyukini-admin/bin/miyukini-admin register-core StrongFather
```

#### Étape 3.2 : Installation de KindMother

```bash
cd crates/kindmother
cargo build --release
cargo test

# Enregistrement dans MiyukiniAdmin
../miyukini-admin/bin/miyukini-admin register-core KindMother
```

#### Étape 3.3 : Installation des autres cores nécessaires

```bash
# CaringNanny
cd crates/caring-nanny && cargo build --release
../miyukini-admin/bin/miyukini-admin register-core CaringNanny

# WorrySentinel
cd crates/worry-sentinel && cargo build --release
../miyukini-admin/bin/miyukini-admin register-core WorrySentinel

# BorderGuard
cd crates/border-guard && cargo build --release
../miyukini-admin/bin/miyukini-admin register-core BorderGuard
```

**Ordre d'installation :**
1. StrongFather (gouvernance)
2. KindMother (persistance)
3. CaringNanny (observation)
4. WorrySentinel (sécurité)
5. BorderGuard (frontières)

---

### 4.5 Phase 4 : Activation de BondingBrother

**Objectif :** Activer BondingBrother maintenant que les dépendances sont en place.

#### Étape 4.1 : Installation de BondingBrother

```bash
cd crates/bonding-brother
cargo build --release
cargo test

# Enregistrement dans MiyukiniAdmin
../miyukini-admin/bin/miyukini-admin register-core BondingBrother
```

#### Étape 4.2 : Activation du mode normal

```bash
# Passer du mode bootstrap au mode normal
./miyukini-admin activate-normal-mode

# Vérifier l'état
./miyukini-admin status
```

**Transition bootstrap → normal :**
- ✅ BondingBrother activé
- ✅ Synchronisation disponible (si réseau)
- ✅ Tous les cores opérationnels
- ✅ Mode bootstrap désactivé

---

### 4.6 Phase 5 : Vérification post-installation

#### Étape 5.1 : Vérification de conformité

```bash
# Vérifier l'état de tous les cores
./miyukini-admin verify-installation

# Vérifier les invariants
./miyukini-admin verify-invariants
```

**Checklist de vérification :**

| Vérification | Commande | Résultat attendu |
|-------------|----------|------------------|
| Kernel opérationnel | `./miyukini-admin kernel-status` | ✅ Opérationnel |
| Cores enregistrés | `./miyukini-admin list-cores` | 6+ cores listés |
| Base de données | `sqlite3 data/miyukini.db "SELECT COUNT(*) FROM core_registry"` | > 0 |
| Identités générées | `./miyukini-admin list-identities` | 3+ identités |
| Mode bootstrap | `./miyukini-admin status` | Mode normal |
| Invariants | `./miyukini-admin verify-invariants` | Tous respectés |

#### Étape 5.2 : Tests de fonctionnement

```bash
# Test de décision StrongFather
./miyukini-admin test-strongfather

# Test de persistance KindMother
./miyukini-admin test-kindmother

# Test d'observation CaringNanny
./miyukini-admin test-caringnanny

# Test de sécurité WorrySentinel
./miyukini-admin test-worrysentinel
```

---

## 5. Résolution de la dépendance circulaire

### 5.1 Problème

**Dépendance circulaire identifiée :**
- MiyukiniAdmin nécessite BondingBrother pour la synchronisation
- BondingBrother nécessite MiyukiniAdmin pour son installation initiale

### 5.2 Solution : Mode Bootstrap

**Principe :** Installation en **deux phases** avec un **mode bootstrap temporaire**.

#### Phase Bootstrap (sans BondingBrother)

| Composant | État | Raison |
|-----------|------|--------|
| Kernel | ✅ Opérationnel | Fondation technique |
| StrongFather | ✅ Opérationnel | Nécessaire pour gouvernance |
| KindMother | ✅ Opérationnel | Nécessaire pour persistance |
| CaringNanny | ✅ Opérationnel | Nécessaire pour observation |
| WorrySentinel | ✅ Opérationnel | Nécessaire pour sécurité |
| BorderGuard | ✅ Opérationnel | Nécessaire pour frontières |
| BondingBrother | ⚠️ Non activé | Dépendance circulaire résolue |

#### Phase Normale (avec BondingBrother)

| Composant | État | Raison |
|-----------|------|--------|
| Tous les cores | ✅ Opérationnels | Installation complète |
| BondingBrother | ✅ Activé | Dépendance résolue |

### 5.3 Garanties du mode Bootstrap

**Fonctionnalités disponibles :**
- ✅ Installation et configuration des cores
- ✅ Génération des identités système
- ✅ Création de la base de données
- ✅ Vérification de conformité
- ✅ Tests techniques

**Fonctionnalités limitées :**
- ⚠️ Pas de synchronisation inter-instances
- ⚠️ Pas de fédération
- ⚠️ Mode dégradé (offline uniquement)

**Transition vers mode normal :**
- Une fois BondingBrother installé, activation automatique
- Vérification de toutes les dépendances
- Passage en mode normal sans perte de données

---

## 6. Cas d'erreur et récupération

### 6.1 Erreur de compilation

**Symptôme :** `cargo build` échoue

**Solutions :**
```bash
# Vérifier la version de Rust
rustc --version

# Nettoyer et reconstruire
cargo clean
cargo build --release

# Vérifier les dépendances
cargo tree
```

### 6.2 Erreur de base de données

**Symptôme :** Erreur SQLite lors de l'initialisation

**Solutions :**
```bash
# Supprimer la base corrompue
rm -f data/miyukini.db

# Réinitialiser
./miyukini-admin migrate init
```

### 6.3 Erreur de dépendance circulaire

**Symptôme :** Impossible d'activer BondingBrother

**Solutions :**
```bash
# Vérifier l'ordre d'installation
./miyukini-admin list-cores

# Réinstaller dans le bon ordre
./miyukini-admin reinstall-cores --order StrongFather,KindMother,CaringNanny,WorrySentinel,BorderGuard,BondingBrother
```

### 6.4 Mode bootstrap bloqué

**Symptôme :** Impossible de passer en mode normal

**Solutions :**
```bash
# Vérifier les logs
./miyukini-admin logs bootstrap

# Forcer l'activation (si sûr)
./miyukini-admin activate-normal-mode --force

# Réinitialiser complètement (dernier recours)
./miyukini-admin reset --confirm
```

---

## 7. Scripts d'automatisation

### 7.1 Script de bootstrap complet

**Fichier :** `scripts/bootstrap.sh`

```bash
#!/bin/bash
set -e

echo "=== Bootstrap MiyukiniAdmin ==="

# Phase 1 : Kernel
echo "Phase 1 : Installation Kernel"
cd crates/miyukini-kernel
cargo build --release
cargo test

# Phase 2 : Bootstrap MiyukiniAdmin
echo "Phase 2 : Bootstrap MiyukiniAdmin"
cd ../miyukini-admin
cargo build --release --features bootstrap
./miyukini-admin migrate init
./miyukini-admin generate-identities

# Phase 3 : Installation Cores
echo "Phase 3 : Installation Cores"
./miyukini-admin install-cores --bootstrap-mode

# Phase 4 : Activation BondingBrother
echo "Phase 4 : Activation BondingBrother"
cd ../bonding-brother
cargo build --release
cd ../miyukini-admin
./miyukini-admin register-core BondingBrother
./miyukini-admin activate-normal-mode

# Phase 5 : Vérification
echo "Phase 5 : Vérification"
./miyukini-admin verify-installation

echo "=== Bootstrap terminé avec succès ==="
```

### 7.2 Utilisation

```bash
chmod +x scripts/bootstrap.sh
./scripts/bootstrap.sh
```

---

## 8. Sécurité du bootstrap

### 8.1 Token de bootstrap

**Génération :** Token temporaire généré lors du bootstrap

**Utilisation :** Authentification pour l'activation de BondingBrother

**Expiration :** Token valide 24 heures après génération

**Sécurité :**
- Token stocké de manière sécurisée (chiffré)
- Token invalidé après activation du mode normal
- Token non réutilisable

### 8.2 Vérifications de sécurité

**Avant activation du mode normal :**
- ✅ Vérification de l'intégrité des cores
- ✅ Vérification des identités système
- ✅ Vérification des permissions
- ✅ Vérification de la base de données

---

## 9. Références

- [MiyukiniAdmin - Documentation Fondatrice](./MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [BondingBrother - Documentation Fondatrice](../BondingBrother/foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Kernel - Definition](../Kernel/Miyukini%20Core%20System%20-%20Definition%20Kernel.md)
- [Lois d'Autonomie Système](../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Guide d'installation et de bootstrap  
**Action requise :** Implémenter le processus de bootstrap selon ce guide lors de l'implémentation de MiyukiniAdmin
