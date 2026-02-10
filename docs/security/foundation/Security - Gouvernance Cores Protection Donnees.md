# Miyukini Security — Gouvernance Cores et Protection des Données

## 1. Introduction

### Objet du document

Ce document définit l'**architecture de protection maximale des données** dans l'écosystème Miyukini COG. Il établit les principes, mécanismes et implémentations qui garantissent que les Cores (en particulier KindMother) sont les **seuls gardiens légitimes** des données, sans possibilité de contournement.

Ce document est **fondamental et contractuel**. Il traduit les Lois d'Autonomie (LOI-1 à LOI-8) en architecture technique de sécurité.

### Contexte

L'architecture Miyukini repose sur le principe que :
> **"Les Cores décident ou gouvernent, jamais n'exécutent"**

Pour que ce principe soit respecté, il ne suffit pas d'une convention de code. Il faut une **garantie technique** que les Opérateurs ne peuvent pas contourner les Cores pour accéder directement aux données.

### Portée

Ce document couvre :
- L'analyse des failles structurelles des bases de données fichier
- L'architecture d'isolation des Cores
- Le système de chiffrement souverain
- La communication inter-processus sécurisée
- La conformité aux Lois d'Autonomie

### Statut contractuel

**FONDATION** — Document fondateur non négociable. Toute implémentation de persistance DOIT respecter les principes énoncés ici.

---

## 2. Problématique : Faille Structurelle des Bases Fichier

### 2.1 Le Problème SQLite

SQLite (et toute base de données fichier) présente une **faille structurelle de sécurité** :

| Aspect | Comportement SQLite | Conséquence |
|--------|---------------------|-------------|
| Authentification | Aucune | N'importe qui peut ouvrir le fichier |
| Permissions | Niveau système de fichiers | Contournable si même utilisateur |
| Chiffrement | Non natif (extensions) | Doit être ajouté |
| Isolation processus | Aucune | Tout code du même processus peut accéder |

### 2.2 Violation de Gouvernance

Sans protection, un Opérateur peut contourner KindMother :

```
SITUATION NON SÉCURISÉE (INTERDITE)
┌──────────────────────────────────────────────────┐
│ Processus unique Miyukini                        │
│                                                  │
│   JayXpose ─────────┐                            │
│                     │ Accès direct               │
│   KindMother ───────┼──► jayxpose.db             │
│                     │ (fichier accessible)       │
│   Code malveillant ─┘                            │
│                                                  │
│   ⚠️ TOUT LE MONDE peut accéder au fichier       │
└──────────────────────────────────────────────────┘
```

**Ceci viole LOI-3 (État local souverain)** car la souveraineté de KindMother sur les données n'est que conceptuelle, pas technique.

---

## 3. Architecture de Protection Maximale

### 3.1 Principe Fondamental

> **"La gouvernance Core n'est valide que si elle est techniquement forcée."**

La seule façon de garantir que KindMother est le seul gardien des données est de :
1. **Isoler KindMother dans un processus séparé**
2. **Restreindre l'accès fichier à ce processus uniquement**
3. **Chiffrer les données avec une clé connue uniquement de ce processus**
4. **Forcer toute communication via une API authentifiée**

### 3.2 Architecture Isolée

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           SYSTÈME D'EXPLOITATION                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────┐    ┌─────────────────────────────┐ │
│  │  PROCESSUS PRINCIPAL                │    │  PROCESSUS KINDMOTHER       │ │
│  │  (Opérateurs + Services)            │    │  (Isolation complète)       │ │
│  │                                     │    │                             │ │
│  │  ┌───────────┐  ┌───────────┐       │    │  ┌─────────────────────┐    │ │
│  │  │ JayXpose  │  │ JayKonta  │       │    │  │  API IPC Server     │    │ │
│  │  └─────┬─────┘  └─────┬─────┘       │    │  │  (gRPC/Unix Socket) │    │ │
│  │        │              │             │    │  └──────────┬──────────┘    │ │
│  │  ┌─────┴──────────────┴─────┐       │    │             │               │ │
│  │  │   KindMother Client      │       │    │  ┌──────────▼──────────┐    │ │
│  │  │   (Stub IPC)             │───────┼────┼──│  KindMother Core    │    │ │
│  │  └──────────────────────────┘       │    │  │  - Validation       │    │ │
│  │                                     │    │  │  - Gouvernance      │    │ │
│  │  ❌ AUCUN accès fichier DB          │    │  │  - Autorisation     │    │ │
│  │  ❌ AUCUNE clé de chiffrement       │    │  └──────────┬──────────┘    │ │
│  │                                     │    │             │               │ │
│  └─────────────────────────────────────┘    │  ┌──────────▼──────────┐    │ │
│                                             │  │  libSQL + Encrypt   │    │ │
│                                             │  │  [Clé en mémoire]   │    │ │
│                                             │  └──────────┬──────────┘    │ │
│                                             │             │               │ │
│                                             │  ┌──────────▼──────────┐    │ │
│                                             │  │  *.db (chiffrés)    │    │ │
│                                             │  │  Permissions: 600   │    │ │
│                                             │  │  Owner: kindmother  │    │ │
│                                             │  └─────────────────────┘    │ │
│                                             │                             │ │
│                                             └─────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.3 Couches de Protection

| Couche | Mécanisme | Menace Bloquée |
|--------|-----------|----------------|
| **1. Isolation processus** | Séparation mémoire OS | Module malveillant interne |
| **2. Permissions fichier** | `chmod 600`, owner=kindmother | Accès par autre processus |
| **3. Chiffrement libSQL** | AES-256-GCM / AEGIS | Copie/vol du fichier |
| **4. Clé dérivée machine** | Argon2 + identifiant machine | Fichier copié ailleurs |
| **5. API authentifiée** | Token signé par requête | Requêtes IPC non autorisées |

---

## 4. Système de Chiffrement Souverain

### 4.1 Choix Technologique : libSQL

Après analyse des alternatives (voir Annexe A), **libSQL** est la solution recommandée :

| Critère | libSQL | Justification |
|---------|--------|---------------|
| Chiffrement natif | ✅ AES-256-GCM / AEGIS | Ajouté octobre 2025 |
| Performance | ✅ 6% lecture / 14% écriture overhead | Acceptable |
| Compatibilité SQLite | ✅ Drop-in replacement | Migration facile |
| Clé en mémoire | ✅ Jamais sur disque | Sécurité maximale |
| Conformité LOI-1 | ✅ Embarqué, pas de serveur | Pas de dépendance externe |
| Maintenance | ✅ 16k+ stars, actif | Pérennité assurée |

### 4.2 Dérivation de Clé Souveraine

La clé de chiffrement est dérivée de secrets **locaux et souverains** (LOI-3) :

```
┌─────────────────────────────────────────────────────────────────┐
│                  DÉRIVATION DE CLÉ SOUVERAINE                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────────────┐                                           │
│   │  Machine ID     │ ← Identifiant unique du hardware          │
│   │  (BIOS UUID)    │   (ne quitte jamais la machine)           │
│   └────────┬────────┘                                           │
│            │                                                    │
│   ┌────────▼────────┐                                           │
│   │  Install Secret │ ← Généré à l'installation du COG          │
│   │  (fichier local)│   (unique à cette instance)               │
│   └────────┬────────┘                                           │
│            │                                                    │
│   ┌────────▼────────┐                                           │
│   │  COG ID         │ ← Identifiant de l'environnement COG      │
│   │  (Kernel)       │   (version figée des Cores)               │
│   └────────┬────────┘                                           │
│            │                                                    │
│   ┌────────▼────────┐                                           │
│   │  Argon2id       │ ← Fonction de dérivation résistante       │
│   │  + Salt fixe    │   aux attaques GPU/ASIC                   │
│   └────────┬────────┘                                           │
│            │                                                    │
│   ┌────────▼────────┐                                           │
│   │  Master Key     │ → Clé AES-256 de 32 octets                │
│   │  (en RAM)       │   JAMAIS stockée sur disque               │
│   └─────────────────┘                                           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Propriétés de sécurité :**
- La clé **n'existe nulle part** sur le disque
- Elle est **recalculée** à chaque démarrage du processus KindMother
- Elle est **unique** à cette machine + cette installation + ce COG
- **Copier le fichier DB** sur une autre machine = fichier inutilisable

### 4.3 Algorithmes de Chiffrement

Deux algorithmes disponibles selon le contexte :

| Algorithme | Usage | Performance | Sécurité |
|------------|-------|-------------|----------|
| **AEGIS** | Par défaut (matériel récent) | Très rapide | AES-NI + auth |
| **AES-256-GCM** | Compatibilité maximale | Rapide | NIST standard |

**Règle** : Utiliser AEGIS si le matériel le supporte, sinon AES-256-GCM.

---

## 5. Communication Inter-Processus (IPC)

### 5.1 Protocole

La communication entre le processus principal et KindMother utilise **gRPC sur Unix Domain Socket** (Linux/macOS) ou **Named Pipe** (Windows).

```
┌─────────────────────────────────────────────────────────────────┐
│                    PROTOCOLE IPC KINDMOTHER                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Opérateur          KindMother Client           KindMother      │
│  (JayXpose)              (Stub)                 (Service)       │
│      │                     │                        │           │
│      │  db.read(...)       │                        │           │
│      │────────────────────►│                        │           │
│      │                     │  IPC Request           │           │
│      │                     │  + Auth Token          │           │
│      │                     │───────────────────────►│           │
│      │                     │                        │           │
│      │                     │          ┌─────────────┤           │
│      │                     │          │ 1. Verify   │           │
│      │                     │          │    Token    │           │
│      │                     │          │ 2. Check    │           │
│      │                     │          │    Perms    │           │
│      │                     │          │ 3. Execute  │           │
│      │                     │          │    Query    │           │
│      │                     │          │ 4. Decrypt  │           │
│      │                     │          │    Data     │           │
│      │                     │          └─────────────┤           │
│      │                     │                        │           │
│      │                     │  IPC Response          │           │
│      │                     │◄───────────────────────│           │
│      │  Result             │                        │           │
│      │◄────────────────────│                        │           │
│      │                     │                        │           │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Authentification des Requêtes

Chaque requête IPC contient un **token d'authentification** :

| Champ | Description |
|-------|-------------|
| `operator_id` | Identifiant de l'Opérateur demandeur |
| `request_id` | UUID unique de la requête (anti-replay) |
| `timestamp` | Horodatage de la requête |
| `signature` | Signature HMAC-SHA256 par StrongFather |

**Règle** : Une requête sans token valide est **rejetée silencieusement**.

### 5.3 Permissions par Opérateur

KindMother maintient une **matrice de permissions** :

| Opérateur | Tables Accessibles | Opérations |
|-----------|-------------------|------------|
| JayXpose | exposants, produits, vitrines | CRUD |
| JayKonta | comptes, transactions, rapports | CRUD |
| JayFestival | evenements, participants | CRUD |
| MiyukiniAdmin | TOUTES | CRUD + Admin |

**Règle** : Un Opérateur ne peut accéder qu'aux tables explicitement autorisées.

---

## 6. Conformité aux Lois d'Autonomie

### 6.1 Tableau de Conformité

| LOI | Énoncé | Conformité | Mécanisme |
|-----|--------|------------|-----------|
| **LOI-1** | Aucune dépendance externe critique | ✅ | libSQL embarqué, pas de serveur |
| **LOI-2** | Le système accepte l'isolement | ✅ | Fonctionne 100% offline |
| **LOI-3** | L'état local est souverain | ✅ | Clé dérivée de secrets locaux |
| **LOI-4** | Pas de temps global requis | ✅ | Horodatage local suffisant |
| **LOI-5** | Coût proportionnel au hardware | ✅ | Overhead IPC ~1-5ms/requête |
| **LOI-6** | L'autonomie n'empêche pas la fédération | ✅ | Sync possible via API KindMother |
| **LOI-7** | La strate Cores est immuable | ✅ | KindMother seul gardien, non contournable |
| **LOI-8** | Migration = diplomatie entre environnements | ✅ | Export/import via API Core uniquement |

### 6.2 Garanties Architecturales

Cette architecture garantit que :

1. **Aucun Opérateur** ne peut accéder directement aux fichiers DB
2. **Aucun code** dans le processus principal n'a la clé de chiffrement
3. **Aucune copie** du fichier DB n'est exploitable sur une autre machine
4. **Toute requête** passe par validation et gouvernance KindMother
5. **Toute opération** est tracée et auditable

---

## 7. Comparaison Avant/Après

| Aspect | Architecture Actuelle | Architecture Sécurisée |
|--------|----------------------|------------------------|
| Accès fichier DB | Tout le monde | KindMother uniquement |
| Chiffrement | Aucun | AES-256-GCM + clé dérivée |
| Contournement Core | Possible | Impossible |
| Gouvernance | Conceptuelle | Technique et forcée |
| Overhead | 0 | ~1-5ms par requête |
| Sécurité fichier volé | Aucune | Données illisibles |
| Conformité LOI | Partielle | Totale |

---

## 8. Invariants de Sécurité

Ces invariants sont **non négociables** :

| ID | Invariant | Violation = |
|----|-----------|-------------|
| **INV-1** | Le fichier DB n'est accessible qu'au processus KindMother | Faille critique |
| **INV-2** | La clé de chiffrement n'est jamais stockée sur disque | Faille critique |
| **INV-3** | Toute requête de données passe par l'API IPC | Contournement Core |
| **INV-4** | Toute requête est authentifiée par token signé | Accès non autorisé |
| **INV-5** | La clé est dérivée de secrets locaux souverains | Violation LOI-3 |
| **INV-6** | Le processus KindMother s'exécute avec un utilisateur dédié | Escalade privilèges |

---

## 9. Annexe A : Analyse des Alternatives

### Bases de données évaluées

| Solution | Chiffrement | SQL | Migration | Conformité LOI | Verdict |
|----------|-------------|-----|-----------|----------------|---------|
| **libSQL** | ✅ Natif AES/AEGIS | ✅ SQLite | ✅ Drop-in | ✅ Totale | **RECOMMANDÉ** |
| BonsaiDb | ✅ ChaCha20 | ❌ Document | ❌ Réécriture | ✅ Totale | Alternative permissions |
| Redbx | ✅ AES-256-GCM | ❌ Key-Value | ❌ Réécriture | ✅ Totale | Trop différent |
| SurrealDB | ⚠️ Optionnel local | ⚠️ SurrealQL | ❌ Réécriture | ⚠️ Lourd | Non recommandé |
| PostgreSQL | ✅ Natif | ✅ SQL | ⚠️ Adaptation | ❌ LOI-1 violation | Exclu |

### Justification du choix libSQL

1. **Migration minimale** : `libsql-rusqlite` est un drop-in replacement de `rusqlite`
2. **Chiffrement natif performant** : 6-14% overhead seulement
3. **Conformité totale** aux 8 Lois d'Autonomie
4. **Projet actif** : 16k+ stars, maintenu par Turso

---

## 10. Annexe B : Structure des Crates

```
crates/
├── kindmother/                    # Core existant (API publique)
│   ├── src/
│   │   ├── lib.rs                # Trait Storage, types publics
│   │   └── ...
│   └── Cargo.toml
│
├── kindmother-service/            # NOUVEAU: Processus isolé
│   ├── src/
│   │   ├── main.rs               # Point d'entrée daemon
│   │   ├── server.rs             # Serveur IPC (gRPC/socket)
│   │   ├── auth.rs               # Validation tokens
│   │   ├── encryption.rs         # Dérivation clé + libSQL
│   │   └── permissions.rs        # Matrice d'accès
│   └── Cargo.toml
│
├── kindmother-client/             # NOUVEAU: Client IPC
│   ├── src/
│   │   ├── lib.rs                # API client (impl trait Storage)
│   │   └── ipc.rs                # Communication avec service
│   └── Cargo.toml
│
├── jayxpose/
│   └── Cargo.toml                # Dépend de kindmother-client
├── jaykonta/
│   └── Cargo.toml                # Dépend de kindmother-client
```

---

## 11. Références

### Documents internes

- [Security - Documentation Fondatrice](./Security%20-%20Documentation%20Fondatrice.md)
- [KindMother - Documentation Fondatrice](../../cores/KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [KindMother - Persistence & Storage Contract](../../cores/KindMother/contracts/persistence/KindMother%20-%20Persistence%20&%20Storage%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- [Miyukini Conceptual References - Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

### Documents techniques externes

- [libSQL Documentation](https://docs.turso.tech/libsql)
- [Turso Encryption Announcement](https://turso.tech/blog/introducing-fast-native-encryption-in-turso-database)

---

**Date de création :** 2026-02-08  
**Version :** 1.0  
**Statut :** FONDATION — Document fondateur contractuel  
**Auteur :** Architecture Miyukini  

---

## 12. Mini Log de Génération

### Décisions structurantes

- Architecture en processus isolé choisie pour garantie technique maximale
- libSQL sélectionné après analyse comparative (voir Annexe A)
- Dérivation de clé basée sur secrets locaux pour conformité LOI-3
- IPC via gRPC/socket pour performance et sécurité

### Vérification de cohérence

- ✅ Cohérence avec les 8 Lois d'Autonomie
- ✅ Cohérence avec la Documentation Fondatrice Sécurité
- ✅ Cohérence avec la Documentation KindMother
- ✅ Respect du principe "Les Cores gouvernent, jamais n'exécutent"

**Aucune contradiction détectée.**
