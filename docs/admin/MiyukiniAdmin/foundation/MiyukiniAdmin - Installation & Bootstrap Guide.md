# MiyukiniAdmin - Installation & Bootstrap Guide

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif â€” Guide d'installation et de bootstrap  
**PortÃ©e :** Installation initiale, bootstrap, rÃ©solution des dÃ©pendances circulaires

---

## 1. Contexte

Ce document dÃ©finit le **processus d'installation et de bootstrap** de MiyukiniAdmin, l'OpÃ©rateur Souverain (Strate 9) de l'Ã©cosystÃ¨me Miyukini Core System.

**ProblÃ©matique identifiÃ©e :** MiyukiniAdmin nÃ©cessite BondingBrother pour fonctionner, mais BondingBrother nÃ©cessite MiyukiniAdmin pour son installation initiale. Ce document rÃ©sout cette **dÃ©pendance circulaire** en dÃ©finissant un processus de bootstrap explicite.

**RÃ©fÃ©rences :**
- [MiyukiniAdmin - Documentation Fondatrice](./MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) â€” dÃ©tection environnement vierge, verrou StrongFather, Futur Admin, compte admin
- [MiyukiniAdmin - Environment Identity Protocol EIP](..//..//..//contrats//MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md) â€” identitÃ© environnement chiffrÃ©e (EIP)
- [BondingBrother - Documentation Fondatrice](..//..//..//cores//BondingBrother//foundation//BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Kernel - Definition](..//..//..//kernel//Miyukini%20Core%20System%20-%20Definition%20Kernel.md)

---

## 2. PortÃ©e / Scope

Ce document couvre :
- PrÃ©requis matÃ©riels et logiciels
- **Premier dÃ©marrage** : dÃ©tection environnement vierge vs initialisÃ©, verrou StrongFather, parcours Futur Admin et crÃ©ation du compte admin (dÃ©tail dans [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md))
- **IdentitÃ© environnement** : production des donnÃ©es d'identitÃ© du COG de faÃ§on chiffrÃ©e par les Cores (protocole [EIP](..//..//..//contrats//MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md))
- Processus d'installation Ã©tape par Ã©tape
- Bootstrap initial (rÃ©solution dÃ©pendance circulaire)
- VÃ©rification post-installation
- Cas d'erreur et rÃ©cupÃ©ration

Ce document **ne couvre pas** :
- Mise Ã  jour de MiyukiniAdmin (voir Versioning & Evolution Contract)
- Migration entre environnements (voir Migration & Compatibility Contract)
- Configuration avancÃ©e (voir Configuration Contract)

---

## 3. PrÃ©requis

### 3.1 MatÃ©riel

| Composant | Minimum | RecommandÃ© |
|-----------|---------|------------|
| **CPU** | 2 cores | 4+ cores |
| **RAM** | 4 Go | 8+ Go |
| **Disque** | 20 Go libres | 50+ Go libres (SSD recommandÃ©) |
| **RÃ©seau** | Optionnel (offline-first) | Connexion pour synchronisation |

### 3.2 Logiciel

| Composant | Version | Notes |
|-----------|---------|-------|
| **OS** | Linux (Ubuntu 22.04+, Debian 11+), macOS 12+, Windows 11+ | â€” |
| **Rust** | 1.75+ | InstallÃ© via rustup |
| **SQLite** | 3.35+ | Inclus dans KindMother |
| **Permissions** | AccÃ¨s lecture/Ã©criture au rÃ©pertoire d'installation | â€” |

### 3.3 Environnement

- Variables d'environnement : aucune requise pour le bootstrap minimal
- Ports rÃ©seau : aucun port requis pour le bootstrap (offline-first)
- Services externes : aucun service externe requis (conforme LOI-1)

---

## 4. Processus d'installation

### 4.1 Phase 0 : PrÃ©paration

#### Ã‰tape 0.1 : VÃ©rification du systÃ¨me

```bash
# VÃ©rifier Rust
rustc --version  # Doit Ãªtre >= 1.75

# VÃ©rifier l'espace disque
df -h  # VÃ©rifier au moins 20 Go libres

# VÃ©rifier les permissions
mkdir -p /tmp/miyukini_test && rmdir /tmp/miyukini_test
```

#### Ã‰tape 0.2 : TÃ©lÃ©chargement

```bash
# Cloner le repository (ou tÃ©lÃ©charger l'archive)
git clone https://github.com/miyukini/miyukini-kernel.git
cd miyukini-kernel

# Ou tÃ©lÃ©charger l'archive release
# wget https://github.com/miyukini/miyukini-kernel/releases/v1.0.0/miyukini-admin.tar.gz
# tar -xzf miyukini-admin.tar.gz
```

---

### 4.2 Phase 1 : Installation du Kernel

**Objectif :** Installer le Kernel (Strate 0) qui est la fondation technique minimale.

#### Ã‰tape 1.1 : Compilation du Kernel

```bash
cd crates/miyukini-kernel
cargo build --release
```

#### Ã‰tape 1.2 : VÃ©rification du Kernel

```bash
# ExÃ©cuter les tests unitaires
cargo test

# VÃ©rifier les invariants
cargo test --test invariants_tests
```

**CritÃ¨res de succÃ¨s :**
- âœ… Compilation sans erreur
- âœ… Tous les tests passent
- âœ… Aucune dÃ©pendance externe requise (vÃ©rification offline)

---

### 4.3 Phase 2 : Bootstrap initial (RÃ©solution dÃ©pendance circulaire)

**ProblÃ¨me :** MiyukiniAdmin nÃ©cessite BondingBrother, mais BondingBrother nÃ©cessite MiyukiniAdmin pour son installation.

**Solution :** Bootstrap en **mode minimal** sans BondingBrother, puis activation progressive.

**Ã‰tat de l'environnement au dÃ©marrage :** Au lancement, MiyukiniAdmin dÃ©termine si l'environnement est **vierge** (donnÃ©es critiques absentes ou invalides) ou **dÃ©jÃ  initialisÃ©**. En environnement vierge, StrongFather applique un **verrou bootstrap** : seuls MiyukiniAdmin et les Cores peuvent agir ; l'utilisateur est traitÃ© comme **Futur Admin** et dirigÃ© vers le **parcours d'installation** (crÃ©ation compte admin, gÃ©nÃ©ration identitÃ© environnement EIP, config minimale). DÃ©tail complet : [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md). IdentitÃ© environnement chiffrÃ©e : [Environment Identity Protocol EIP](..//..//..//contrats//MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md).

#### Ã‰tape 2.1 : Installation de MiyukiniAdmin en mode bootstrap

```bash
cd crates/miyukini-admin

# Compilation en mode bootstrap (sans dÃ©pendance BondingBrother)
cargo build --release --features bootstrap

# Ou via script d'installation
./scripts/bootstrap.sh
```

**Mode bootstrap :**
- âœ… Kernel initialisÃ©
- âœ… Base de donnÃ©es locale crÃ©Ã©e (SQLite)
- âœ… IdentitÃ©s systÃ¨me gÃ©nÃ©rÃ©es
- âœ… Configuration minimale chargÃ©e
- âš ï¸ BondingBrother **non activÃ©** (mode dÃ©gradÃ©)

#### Ã‰tape 2.2 : Initialisation de la base de donnÃ©es

```bash
# ExÃ©cuter les migrations SQL initiales
./miyukini-admin migrate init

# VÃ©rifier la crÃ©ation des tables
sqlite3 data/miyukini.db ".tables"
```

**Tables crÃ©Ã©es :**
- `kernel_config` : Configuration systÃ¨me
- `system_identities` : IdentitÃ©s systÃ¨me gÃ©nÃ©rÃ©es
- `bootstrap_log` : Journal du bootstrap
- `core_registry` : Registre des cores (vide initialement)

#### Ã‰tape 2.3 : GÃ©nÃ©ration des identitÃ©s systÃ¨me

```bash
# GÃ©nÃ©rer les identitÃ©s systÃ¨me (via Kernel)
./miyukini-admin generate-identities

# VÃ©rifier les identitÃ©s
./miyukini-admin list-identities
```

**IdentitÃ©s gÃ©nÃ©rÃ©es :**
- `system_admin_id` : IdentitÃ© de l'administrateur systÃ¨me
- `environment_id` : IdentitÃ© de l'environnement
- `bootstrap_token` : Token temporaire pour activation BondingBrother

---

### 4.4 Phase 3 : Installation des Cores (sans BondingBrother)

**Objectif :** Installer les cores fondamentaux nÃ©cessaires avant BondingBrother.

#### Ã‰tape 3.1 : Installation de StrongFather

```bash
cd crates/miyukini-strongfather
cargo build --release
cargo test

# Enregistrement dans MiyukiniAdmin
../miyukini-admin/bin/miyukini-admin register-core StrongFather
```

#### Ã‰tape 3.2 : Installation de KindMother

```bash
cd crates/kindmother
cargo build --release
cargo test

# Enregistrement dans MiyukiniAdmin
../miyukini-admin/bin/miyukini-admin register-core KindMother
```

#### Ã‰tape 3.3 : Installation des autres cores nÃ©cessaires

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
4. WorrySentinel (sÃ©curitÃ©)
5. BorderGuard (frontiÃ¨res)

---

### 4.5 Phase 4 : Activation de BondingBrother

**Objectif :** Activer BondingBrother maintenant que les dÃ©pendances sont en place.

#### Ã‰tape 4.1 : Installation de BondingBrother

```bash
cd crates/bonding-brother
cargo build --release
cargo test

# Enregistrement dans MiyukiniAdmin
../miyukini-admin/bin/miyukini-admin register-core BondingBrother
```

#### Ã‰tape 4.2 : Activation du mode normal

```bash
# Passer du mode bootstrap au mode normal
./miyukini-admin activate-normal-mode

# VÃ©rifier l'Ã©tat
./miyukini-admin status
```

**Transition bootstrap â†’ normal :**
- âœ… BondingBrother activÃ©
- âœ… Synchronisation disponible (si rÃ©seau)
- âœ… Tous les cores opÃ©rationnels
- âœ… Mode bootstrap dÃ©sactivÃ©

---

### 4.6 Phase 5 : VÃ©rification post-installation

#### Ã‰tape 5.1 : VÃ©rification de conformitÃ©

```bash
# VÃ©rifier l'Ã©tat de tous les cores
./miyukini-admin verify-installation

# VÃ©rifier les invariants
./miyukini-admin verify-invariants
```

**Checklist de vÃ©rification :**

| VÃ©rification | Commande | RÃ©sultat attendu |
|-------------|----------|------------------|
| Kernel opÃ©rationnel | `./miyukini-admin kernel-status` | âœ… OpÃ©rationnel |
| Cores enregistrÃ©s | `./miyukini-admin list-cores` | 6+ cores listÃ©s |
| Base de donnÃ©es | `sqlite3 data/miyukini.db "SELECT COUNT(*) FROM core_registry"` | > 0 |
| IdentitÃ©s gÃ©nÃ©rÃ©es | `./miyukini-admin list-identities` | 3+ identitÃ©s |
| Mode bootstrap | `./miyukini-admin status` | Mode normal |
| Invariants | `./miyukini-admin verify-invariants` | Tous respectÃ©s |

#### Ã‰tape 5.2 : Tests de fonctionnement

```bash
# Test de dÃ©cision StrongFather
./miyukini-admin test-strongfather

# Test de persistance KindMother
./miyukini-admin test-kindmother

# Test d'observation CaringNanny
./miyukini-admin test-caringnanny

# Test de sÃ©curitÃ© WorrySentinel
./miyukini-admin test-worrysentinel
```

---

## 5. RÃ©solution de la dÃ©pendance circulaire

### 5.1 ProblÃ¨me

**DÃ©pendance circulaire identifiÃ©e :**
- MiyukiniAdmin nÃ©cessite BondingBrother pour la synchronisation
- BondingBrother nÃ©cessite MiyukiniAdmin pour son installation initiale

### 5.2 Solution : Mode Bootstrap

**Principe :** Installation en **deux phases** avec un **mode bootstrap temporaire**.

#### Phase Bootstrap (sans BondingBrother)

| Composant | Ã‰tat | Raison |
|-----------|------|--------|
| Kernel | âœ… OpÃ©rationnel | Fondation technique |
| StrongFather | âœ… OpÃ©rationnel | NÃ©cessaire pour gouvernance |
| KindMother | âœ… OpÃ©rationnel | NÃ©cessaire pour persistance |
| CaringNanny | âœ… OpÃ©rationnel | NÃ©cessaire pour observation |
| WorrySentinel | âœ… OpÃ©rationnel | NÃ©cessaire pour sÃ©curitÃ© |
| BorderGuard | âœ… OpÃ©rationnel | NÃ©cessaire pour frontiÃ¨res |
| BondingBrother | âš ï¸ Non activÃ© | DÃ©pendance circulaire rÃ©solue |

#### Phase Normale (avec BondingBrother)

| Composant | Ã‰tat | Raison |
|-----------|------|--------|
| Tous les cores | âœ… OpÃ©rationnels | Installation complÃ¨te |
| BondingBrother | âœ… ActivÃ© | DÃ©pendance rÃ©solue |

### 5.3 Garanties du mode Bootstrap

**FonctionnalitÃ©s disponibles :**
- âœ… Installation et configuration des cores
- âœ… GÃ©nÃ©ration des identitÃ©s systÃ¨me
- âœ… CrÃ©ation de la base de donnÃ©es
- âœ… VÃ©rification de conformitÃ©
- âœ… Tests techniques

**FonctionnalitÃ©s limitÃ©es :**
- âš ï¸ Pas de synchronisation inter-instances
- âš ï¸ Pas de fÃ©dÃ©ration
- âš ï¸ Mode dÃ©gradÃ© (offline uniquement)

**Transition vers mode normal :**
- Une fois BondingBrother installÃ©, activation automatique
- VÃ©rification de toutes les dÃ©pendances
- Passage en mode normal sans perte de donnÃ©es

---

## 6. Cas d'erreur et rÃ©cupÃ©ration

### 6.1 Erreur de compilation

**SymptÃ´me :** `cargo build` Ã©choue

**Solutions :**
```bash
# VÃ©rifier la version de Rust
rustc --version

# Nettoyer et reconstruire
cargo clean
cargo build --release

# VÃ©rifier les dÃ©pendances
cargo tree
```

### 6.2 Erreur de base de donnÃ©es

**SymptÃ´me :** Erreur SQLite lors de l'initialisation

**Solutions :**
```bash
# Supprimer la base corrompue
rm -f data/miyukini.db

# RÃ©initialiser
./miyukini-admin migrate init
```

### 6.3 Erreur de dÃ©pendance circulaire

**SymptÃ´me :** Impossible d'activer BondingBrother

**Solutions :**
```bash
# VÃ©rifier l'ordre d'installation
./miyukini-admin list-cores

# RÃ©installer dans le bon ordre
./miyukini-admin reinstall-cores --order StrongFather,KindMother,CaringNanny,WorrySentinel,BorderGuard,BondingBrother
```

### 6.4 Mode bootstrap bloquÃ©

**SymptÃ´me :** Impossible de passer en mode normal

**Solutions :**
```bash
# VÃ©rifier les logs
./miyukini-admin logs bootstrap

# Forcer l'activation (si sÃ»r)
./miyukini-admin activate-normal-mode --force

# RÃ©initialiser complÃ¨tement (dernier recours)
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

# Phase 5 : VÃ©rification
echo "Phase 5 : VÃ©rification"
./miyukini-admin verify-installation

echo "=== Bootstrap terminÃ© avec succÃ¨s ==="
```

### 7.2 Utilisation

```bash
chmod +x scripts/bootstrap.sh
./scripts/bootstrap.sh
```

---

## 8. SÃ©curitÃ© du bootstrap

### 8.1 Token de bootstrap

**GÃ©nÃ©ration :** Token temporaire gÃ©nÃ©rÃ© lors du bootstrap

**Utilisation :** Authentification pour l'activation de BondingBrother

**Expiration :** Token valide 24 heures aprÃ¨s gÃ©nÃ©ration

**SÃ©curitÃ© :**
- Token stockÃ© de maniÃ¨re sÃ©curisÃ©e (chiffrÃ©)
- Token invalidÃ© aprÃ¨s activation du mode normal
- Token non rÃ©utilisable

### 8.2 VÃ©rifications de sÃ©curitÃ©

**Avant activation du mode normal :**
- âœ… VÃ©rification de l'intÃ©gritÃ© des cores
- âœ… VÃ©rification des identitÃ©s systÃ¨me
- âœ… VÃ©rification des permissions
- âœ… VÃ©rification de la base de donnÃ©es

---

## 9. RÃ©fÃ©rences

- [MiyukiniAdmin - Documentation Fondatrice](./MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [BondingBrother - Documentation Fondatrice](..//..//..//cores//BondingBrother//foundation//BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Kernel - Definition](..//..//..//kernel//Miyukini%20Core%20System%20-%20Definition%20Kernel.md)
- [Lois d'Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif â€” Guide d'installation et de bootstrap  
**Action requise :** ImplÃ©menter le processus de bootstrap selon ce guide lors de l'implÃ©mentation de MiyukiniAdmin



