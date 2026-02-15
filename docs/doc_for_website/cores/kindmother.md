# KindMother

## Core de Persistance et Gestion des Données

**KindMother** est le Core responsable de la persistance des données. Elle protège et organise tout le stockage du COG avec bienveillance et rigueur.

## Rôle Principal

> KindMother **persiste** et **protège** les données.

KindMother est la gardienne des données du COG. Elle assure que toute information est stockée de manière sûre, intègre et accessible uniquement aux entités autorisées.

## Responsabilités

### Persistance

| Fonction | Description |
|----------|-------------|
| Stockage | Écriture des données |
| Récupération | Lecture des données |
| Organisation | Structure et indexation |
| Migration | Évolution des schémas |

### Protection

| Fonction | Description |
|----------|-------------|
| Intégrité | Vérification des données |
| Chiffrement | Protection au repos |
| Sauvegarde | Copies de sécurité |
| Isolation | Séparation par contexte |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                 KINDMOTHER                       │
│  ┌───────────────────────────────────────────┐  │
│  │           Storage Manager                  │  │
│  └───────────────────────────────────────────┘  │
│       │           │           │           │     │
│       ▼           ▼           ▼           ▼     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ │
│  │ Write  │ │   Read   │ │ Index  │ │Encrypt │ │
│  │ Engine │ │  Engine  │ │ Engine │ │ Engine │ │
│  └────────┘ └──────────┘ └────────┘ └────────┘ │
│                     │                           │
│                     ▼                           │
│  ┌───────────────────────────────────────────┐  │
│  │              SQLite + SQLCipher           │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

## Stockage Local

### Base de Données

KindMother utilise **SQLite** avec **SQLCipher** pour :
- ✓ Fonctionnement offline (LOI-1, LOI-2)
- ✓ Données souveraines (LOI-3)
- ✓ Chiffrement au repos
- ✓ Performance sur hardware modeste (LOI-5)

### Structure Type

```
data/
├── kindmother.db          # Base principale (chiffrée)
├── kindmother.db-wal      # Write-Ahead Log
├── kindmother.db-shm      # Shared Memory
└── backups/
    └── kindmother_YYYYMMDD.db
```

## Interactions avec les Autres Cores

```
StrongFather ──► "Persister ces données"
        │
        ▼
┌──────────────┐
│  KindMother  │
└──────┬───────┘
       │
       ├──► TAMR : "Qui a le droit ?"
       │
       ├──► BorderGuard : "Données valides ?"
       │
       └──► WorrySentinel : "Anomalie détectée ?"
```

## Flux de Persistance

### Écriture

```
Demande d'écriture
        │
        ▼
┌─────────────────┐
│   Validation    │──► Schéma respecté ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Autorisation  │──► TAMR (permissions)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Chiffrement   │──► SQLCipher
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Écriture     │──► SQLite
└────────┬────────┘
         │
         ▼
    Confirmation
```

### Lecture

```
Demande de lecture
        │
        ▼
┌─────────────────┐
│   Autorisation  │──► TAMR (permissions)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Lecture      │──► SQLite
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Déchiffrement  │──► SQLCipher
└────────┬────────┘
         │
         ▼
    Données
```

## Niveaux de Sécurité des Données

| Niveau | Description | Protection |
|--------|-------------|------------|
| **0** | Public | Accessible librement |
| **1** | Standard | Authentification requise |
| **2** | Sensible | Chiffrement renforcé |
| **3** | Critique | Accès très restreint |
| **4** | Maximum | Protection maximale |

## Principes de Gouvernance

### Principe de Bienveillance

KindMother protège les données comme une mère protège ses enfants :
- Jamais de suppression sans confirmation
- Toujours une sauvegarde avant modification majeure
- Alerte en cas d'anomalie

### Principe de Non-Divulgation

KindMother ne donne **jamais** de données sans :
- Validation par TAMR
- Intention explicite
- Contexte approprié

## États de Fonctionnement

| État | Description |
|------|-------------|
| **READY** | Prête à persister |
| **WRITING** | Écriture en cours |
| **READING** | Lecture en cours |
| **MIGRATING** | Migration de schéma |
| **BACKUP** | Sauvegarde en cours |
| **LOCKED** | Accès temporairement bloqué |

## Invariants

| Invariant | Description |
|-----------|-------------|
| Intégrité | Données jamais corrompues |
| Durabilité | Données jamais perdues sans backup |
| Isolation | Contextes strictement séparés |
| Confidentialité | Chiffrement au repos |

## Contrats

### Contrat de Persistance

KindMother garantit :
- ✓ Écriture ACID (Atomicité, Cohérence, Isolation, Durabilité)
- ✓ Lecture cohérente
- ✓ Transactions supportées
- ✓ Rollback possible

### Contrat de Protection

KindMother garantit :
- ✓ Chiffrement SQLCipher
- ✓ Sauvegardes régulières
- ✓ Vérification d'intégrité
- ✓ Isolation des contextes

## Cas d'Usage

### Exemple : Enregistrement d'un Document

```
JayXpose : "Enregistrer ce portfolio"
                │
                ▼
StrongFather : Orchestration
                │
                ▼
KindMother reçoit la demande
                │
    ┌───────────┴───────────┐
    ▼                       ▼
Validation schéma      TAMR vérifie
(structure OK?)        (permission OK?)
    │                       │
    └───────────┬───────────┘
                ▼
    Chiffrement des données
                │
                ▼
    Écriture SQLite
                │
                ▼
    Confirmation envoyée
```

## Migration de Données

Lors d'une migration inter-COG (LOI-8) :

1. **Export** — KindMother extrait les données
2. **Validation** — Intégrité vérifiée
3. **Transformation** — Adaptation au nouveau schéma
4. **Import** — Intégration dans le COG destination
5. **Confirmation** — Validation de cohérence

## Sécurité

- Chiffrement SQLCipher (AES-256)
- Clé dérivée du contexte utilisateur
- Aucun accès sans TAMR
- Audit de toutes les opérations
