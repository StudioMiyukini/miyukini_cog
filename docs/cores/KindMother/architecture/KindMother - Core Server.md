# KindMother â€” Core Server

## 1. Introduction

### Objet du document

Ce document dÃ©crit l'architecture et le fonctionnement du **KindMother Core Server** â€” le processus isolÃ© qui dÃ©tient l'autoritÃ© exclusive sur la persistance des donnÃ©es dans l'Ã©cosystÃ¨me Miyukini COG.

### Contexte

Le Core Server est la matÃ©rialisation technique du principe fondamental :
> **"Les Cores gouvernent, jamais n'exÃ©cutent directement."**

En isolant KindMother dans un processus sÃ©parÃ©, nous garantissons que la gouvernance des donnÃ©es n'est pas une simple convention de code mais une **rÃ©alitÃ© technique incontournable**.

### PortÃ©e

Ce document couvre :
- Architecture interne du serveur
- MÃ©caniques d'arbitrage et de gouvernance
- Gestion des bases de donnÃ©es multiples
- Cycle de vie des requÃªtes
- ObservabilitÃ© et audit

### PrÃ©requis

- [KindMother - Documentation Fondatrice](../foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [Security - Gouvernance Cores Protection Donnees](..//..//WorrySentinel//_index.md)

### Ce document ne couvre PAS

- DÃ©tails d'implÃ©mentation Rust (voir [Systeme Persistance libSQL Migration](../implementation/KindMother%20-%20Systeme%20Persistance%20libSQL%20Migration.md))
- Configuration et dÃ©ploiement (voir document Migration)
- API client (voir [KindMother - Client](./KindMother%20-%20Client.md))

---

## 2. Architecture du Core Server

### 2.1 Vue d'ensemble

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        KINDMOTHER CORE SERVER                               â”‚
â”‚                     (Processus isolÃ©, autoritÃ© exclusive)                   â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                             â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                      COUCHE TRANSPORT (IPC)                           â”‚  â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                   â”‚  â”‚
â”‚  â”‚  â”‚ gRPC Server â”‚  â”‚ Unix Socket â”‚  â”‚ Named Pipe  â”‚                   â”‚  â”‚
â”‚  â”‚  â”‚   (tonic)   â”‚  â”‚   (Linux)   â”‚  â”‚  (Windows)  â”‚                   â”‚  â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”˜                   â”‚  â”‚
â”‚  â”‚         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                          â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                             â–¼                                               â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                      COUCHE ARBITRAGE                                 â”‚  â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚  â”‚
â”‚  â”‚  â”‚    Auth     â”‚  â”‚ Permission  â”‚  â”‚  Validation â”‚  â”‚   Quota     â”‚  â”‚  â”‚
â”‚  â”‚  â”‚  Validator  â”‚â†’ â”‚   Engine    â”‚â†’ â”‚   Engine    â”‚â†’ â”‚  Manager    â”‚  â”‚  â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                             â–¼                                               â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                      COUCHE ORCHESTRATION                             â”‚  â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                   â”‚  â”‚
â”‚  â”‚  â”‚  Database   â”‚  â”‚  WriteIntentâ”‚  â”‚    Sync     â”‚                   â”‚  â”‚
â”‚  â”‚  â”‚   Router    â”‚  â”‚   Handler   â”‚  â”‚ Coordinator â”‚                   â”‚  â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                   â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                             â–¼                                               â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                      COUCHE PERSISTANCE                               â”‚  â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚  â”‚
â”‚  â”‚  â”‚                    libSQL Engine                                â”‚  â”‚  â”‚
â”‚  â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚  â”‚  â”‚
â”‚  â”‚  â”‚  â”‚ jayxpose  â”‚  â”‚ jaykonta  â”‚  â”‚jayfestivalâ”‚  â”‚   ...     â”‚    â”‚  â”‚  â”‚
â”‚  â”‚  â”‚  â”‚    .db    â”‚  â”‚    .db    â”‚  â”‚    .db    â”‚  â”‚           â”‚    â”‚  â”‚  â”‚
â”‚  â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚  â”‚  â”‚
â”‚  â”‚  â”‚                     [Chiffrement AES-256-GCM]                   â”‚  â”‚  â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                                                             â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                      COUCHE OBSERVABILITÃ‰                             â”‚  â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                   â”‚  â”‚
â”‚  â”‚  â”‚   Metrics   â”‚  â”‚   Audit     â”‚  â”‚    Health   â”‚                   â”‚  â”‚
â”‚  â”‚  â”‚  Collector  â”‚  â”‚    Log      â”‚  â”‚   Monitor   â”‚                   â”‚  â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                   â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 2.2 Principes architecturaux

| Principe | Description | Garantie |
|----------|-------------|----------|
| **Isolation totale** | Processus sÃ©parÃ©, mÃ©moire isolÃ©e | Aucun accÃ¨s direct aux donnÃ©es |
| **AutoritÃ© exclusive** | Seul propriÃ©taire des fichiers DB | Permissions fichier `600` |
| **Chiffrement souverain** | ClÃ© dÃ©rivÃ©e localement | Fichiers illisibles sans clÃ© |
| **Arbitrage systÃ©matique** | Chaque requÃªte validÃ©e | Aucun contournement possible |

---

## 3. MÃ©caniques d'Arbitrage

L'arbitrage est le processus par lequel le Core Server **valide, autorise et gouverne** chaque opÃ©ration de donnÃ©es. C'est le coeur de la gouvernance KindMother.

### 3.1 Pipeline d'arbitrage

Chaque requÃªte traverse un pipeline de validation en 4 Ã©tapes :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                       PIPELINE D'ARBITRAGE                               â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                          â”‚
â”‚  RequÃªte    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚  entrante â†’ â”‚  AUTH   â”‚ â†’ â”‚  PERMS  â”‚ â†’ â”‚ VALIDA- â”‚ â†’ â”‚  QUOTA  â”‚ â†’   â”‚
â”‚             â”‚         â”‚    â”‚         â”‚    â”‚  TION   â”‚    â”‚         â”‚    â”‚
â”‚             â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜    â”‚
â”‚                  â”‚              â”‚              â”‚              â”‚         â”‚
â”‚             â”Œâ”€â”€â”€â”€â–¼â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â–¼â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â–¼â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â–¼â”€â”€â”€â”€â”    â”‚
â”‚             â”‚ Token   â”‚    â”‚ Matrice â”‚    â”‚ Schema  â”‚    â”‚ Rate    â”‚    â”‚
â”‚             â”‚ Verify  â”‚    â”‚ AccÃ¨s   â”‚    â”‚ Check   â”‚    â”‚ Limit   â”‚    â”‚
â”‚             â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚
â”‚                                                                          â”‚
â”‚  âŒ Rejet Ã  n'importe quelle Ã©tape = RequÃªte refusÃ©e                    â”‚
â”‚  âœ… Toutes les Ã©tapes passÃ©es = ExÃ©cution autorisÃ©e                     â”‚
â”‚                                                                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 3.2 Ã‰tape 1 : Authentification (Auth Validator)

**RÃ´le** : VÃ©rifier l'identitÃ© de l'appelant et la validitÃ© du token.

| VÃ©rification | Description | Ã‰chec = |
|--------------|-------------|---------|
| Token prÃ©sent | Le token d'auth est fourni | `UNAUTHENTICATED` |
| Signature valide | HMAC-SHA256 correct | `INVALID_TOKEN` |
| Non expirÃ© | Timestamp dans la fenÃªtre | `TOKEN_EXPIRED` |
| Non rejouÃ© | Request ID unique | `REPLAY_DETECTED` |

**Structure du token** :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     AUTH TOKEN                         â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  operator_id   â”‚ "jayxpose"                            â”‚
â”‚  request_id    â”‚ "uuid-unique-par-requÃªte"             â”‚
â”‚  timestamp     â”‚ 1707350400 (Unix epoch)               â”‚
â”‚  signature     â”‚ HMAC-SHA256(payload, shared_secret)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**FenÃªtre temporelle** : Â±5 minutes (configurable) pour compenser les dÃ©rives d'horloge locales.

### 3.3 Ã‰tape 2 : Permissions (Permission Engine)

**RÃ´le** : VÃ©rifier que l'opÃ©rateur a le droit d'effectuer l'opÃ©ration demandÃ©e.

#### Matrice de permissions

La matrice dÃ©finit les accÃ¨s par **OpÃ©rateur** Ã— **Base** Ã— **Table** Ã— **OpÃ©ration** :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    MATRICE DE PERMISSIONS                                 â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚    OpÃ©rateur   â”‚     Base      â”‚           Permissions                    â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚   jayxpose     â”‚   jayxpose    â”‚ exposants: CRUD                         â”‚
â”‚                â”‚               â”‚ produits: CRUD                          â”‚
â”‚                â”‚               â”‚ vitrines: CRUD                          â”‚
â”‚                â”‚               â”‚ documents: CRUD                         â”‚
â”‚                â”‚               â”‚ cms_articles: CRUD                      â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚   jaykonta     â”‚   jaykonta    â”‚ comptes: CRUD                           â”‚
â”‚                â”‚               â”‚ transactions: CR                        â”‚
â”‚                â”‚               â”‚ rapports: R                             â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚   jayfestival  â”‚  jayfestival  â”‚ evenements: CRUD                        â”‚
â”‚                â”‚               â”‚ participants: CRUD                      â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ miyukiniadmin  â”‚     TOUTES    â”‚ TOUTES: CRUD + ADMIN                    â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚   jayxpose     â”‚   jaykonta    â”‚ âŒ AUCUN ACCÃˆS                          â”‚
â”‚   jaykonta     â”‚   jayxpose    â”‚ âŒ AUCUN ACCÃˆS                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

#### OpÃ©rations

| Code | Signification | Description |
|------|---------------|-------------|
| **C** | Create | Insertion de nouvelles entitÃ©s |
| **R** | Read | Lecture d'entitÃ©s |
| **U** | Update | Modification d'entitÃ©s existantes |
| **D** | Delete | Suppression d'entitÃ©s |
| **ADMIN** | Administration | OpÃ©rations de maintenance (vacuum, reindex, etc.) |

#### RÃ¨gles d'isolation

| RÃ¨gle | Description |
|-------|-------------|
| **ISO-1** | Un OpÃ©rateur ne peut accÃ©der qu'Ã  ses propres bases |
| **ISO-2** | L'accÃ¨s inter-bases est interdit sauf pour MiyukiniAdmin |
| **ISO-3** | Les permissions sont dÃ©finies au dÃ©marrage, non modifiables Ã  runtime |

### 3.4 Ã‰tape 3 : Validation (Validation Engine)

**RÃ´le** : VÃ©rifier la cohÃ©rence et l'intÃ©gritÃ© de la requÃªte.

| Validation | Description | Exemple de rejet |
|------------|-------------|------------------|
| **Schema** | La requÃªte respecte le schÃ©ma attendu | Champ requis manquant |
| **Types** | Les types de donnÃ©es sont corrects | String au lieu de Integer |
| **RÃ©fÃ©rences** | Les clÃ©s Ã©trangÃ¨res existent | `exposant_id` inexistant |
| **Contraintes** | Les contraintes mÃ©tier sont respectÃ©es | Prix nÃ©gatif |
| **Taille** | Les donnÃ©es ne dÃ©passent pas les limites | Texte > 65535 chars |

#### Validation SQL

Pour les requÃªtes SQL directes (si autorisÃ©es), le Validation Engine applique :

| ContrÃ´le | Description |
|----------|-------------|
| **Whitelist tables** | Seules les tables autorisÃ©es sont accessibles |
| **Blacklist keywords** | `DROP`, `TRUNCATE`, `ALTER` interdits |
| **ParamÃ¨tres liÃ©s** | Pas de concatÃ©nation SQL (anti-injection) |
| **Limite rÃ©sultats** | Maximum 10 000 lignes par requÃªte |

### 3.5 Ã‰tape 4 : Quotas (Quota Manager)

**RÃ´le** : ProtÃ©ger le systÃ¨me contre les abus et surcharges.

| Quota | Limite par dÃ©faut | Description |
|-------|-------------------|-------------|
| **RequÃªtes/minute** | 1000 | Par opÃ©rateur |
| **Ã‰critures/minute** | 100 | Par opÃ©rateur |
| **Taille payload** | 10 MB | Par requÃªte |
| **Connexions simultanÃ©es** | 50 | Par opÃ©rateur |

#### RÃ©ponses en cas de dÃ©passement

| Situation | RÃ©ponse |
|-----------|---------|
| Limite atteinte | `RESOURCE_EXHAUSTED` + dÃ©lai retry |
| Abus dÃ©tectÃ© | Blocage temporaire (1-60 min) |
| Attaque suspectÃ©e | Alerte WorrySentinel |

---

## 4. Gestion Multi-Bases

Le Core Server gÃ¨re plusieurs bases de donnÃ©es simultanÃ©ment, une par OpÃ©rateur.

### 4.1 Architecture multi-bases

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    DATABASE ROUTER                                 â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                    â”‚
â”‚   RequÃªte                   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                   â”‚
â”‚   (database: "jayxpose") â†’ â”‚  Route Resolver  â”‚                   â”‚
â”‚                             â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                   â”‚
â”‚                                      â”‚                             â”‚
â”‚        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚        â–¼                             â–¼                         â–¼   â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”               â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚   â”‚jayxpose â”‚               â”‚  jaykonta   â”‚            â”‚festival â”‚ â”‚
â”‚   â”‚Connection               â”‚ Connection  â”‚            â”‚Connectionâ”‚ â”‚
â”‚   â”‚   Pool  â”‚               â”‚    Pool     â”‚            â”‚   Pool  â”‚ â”‚
â”‚   â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜               â””â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”˜            â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜ â”‚
â”‚        â”‚                           â”‚                        â”‚      â”‚
â”‚   â”Œâ”€â”€â”€â”€â–¼â”€â”€â”€â”€â”               â”Œâ”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”            â”Œâ”€â”€â”€â”€â–¼â”€â”€â”€â”€â” â”‚
â”‚   â”‚jayxpose â”‚               â”‚  jaykonta   â”‚            â”‚festival â”‚ â”‚
â”‚   â”‚   .db   â”‚               â”‚     .db     â”‚            â”‚   .db   â”‚ â”‚
â”‚   â”‚(chiffrÃ©)â”‚               â”‚  (chiffrÃ©)  â”‚            â”‚(chiffrÃ©)â”‚ â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜               â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                                                    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 4.2 Pool de connexions

Chaque base dispose de son propre pool de connexions :

| ParamÃ¨tre | Valeur | Description |
|-----------|--------|-------------|
| **min_connections** | 2 | Connexions maintenues Ã  froid |
| **max_connections** | 10 | Maximum simultanÃ© |
| **connection_timeout** | 30s | Attente max pour obtenir une connexion |
| **idle_timeout** | 300s | Fermeture aprÃ¨s inactivitÃ© |

### 4.3 Isolation des bases

| Garantie | MÃ©canisme |
|----------|-----------|
| **Isolation fichier** | Chaque base = fichier sÃ©parÃ© |
| **Isolation connexion** | Pool dÃ©diÃ© par base |
| **Isolation transaction** | Pas de transaction cross-base |
| **Isolation erreur** | Erreur sur une base n'affecte pas les autres |

---

## 5. Cycle de Vie des RequÃªtes

### 5.1 RequÃªte de lecture (Read)

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     CYCLE DE VIE : LECTURE                                 â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                            â”‚
â”‚  1. RÃ‰CEPTION                                                              â”‚
â”‚     Client â†’ IPC â†’ Core Server                                             â”‚
â”‚     Parse request, extract auth token                                      â”‚
â”‚                                                                            â”‚
â”‚  2. ARBITRAGE                                                              â”‚
â”‚     Auth â†’ Permissions â†’ Validation â†’ Quota                                â”‚
â”‚     Tout OK ? Continue : Reject                                            â”‚
â”‚                                                                            â”‚
â”‚  3. ROUTAGE                                                                â”‚
â”‚     Database Router â†’ Select correct DB                                    â”‚
â”‚     Connection Pool â†’ Acquire connection                                   â”‚
â”‚                                                                            â”‚
â”‚  4. EXÃ‰CUTION                                                              â”‚
â”‚     libSQL â†’ Execute query                                                 â”‚
â”‚     Decrypt data (transparent)                                             â”‚
â”‚                                                                            â”‚
â”‚  5. RÃ‰PONSE                                                                â”‚
â”‚     Serialize result â†’ IPC â†’ Client                                        â”‚
â”‚     Audit log (async)                                                      â”‚
â”‚                                                                            â”‚
â”‚  Latence typique : 1-5 ms (local)                                          â”‚
â”‚                                                                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2 RequÃªte d'Ã©criture (Write)

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     CYCLE DE VIE : Ã‰CRITURE                                â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                            â”‚
â”‚  1. RÃ‰CEPTION                                                              â”‚
â”‚     Client â†’ IPC â†’ Core Server                                             â”‚
â”‚     Parse WriteIntent                                                      â”‚
â”‚                                                                            â”‚
â”‚  2. ARBITRAGE                                                              â”‚
â”‚     Auth â†’ Permissions (write) â†’ Validation â†’ Quota                        â”‚
â”‚     VÃ©rifications supplÃ©mentaires pour Ã©critures                           â”‚
â”‚                                                                            â”‚
â”‚  3. VALIDATION AVANCÃ‰E                                                     â”‚
â”‚     Schema validation                                                      â”‚
â”‚     Foreign key check                                                      â”‚
â”‚     Business constraints                                                   â”‚
â”‚                                                                            â”‚
â”‚  4. TRANSACTION                                                            â”‚
â”‚     BEGIN TRANSACTION                                                      â”‚
â”‚     Execute write(s)                                                       â”‚
â”‚     COMMIT (ou ROLLBACK si erreur)                                         â”‚
â”‚                                                                            â”‚
â”‚  5. CONFIRMATION                                                           â”‚
â”‚     Result (id, affected rows) â†’ Client                                    â”‚
â”‚     Audit log (sync for writes)                                            â”‚
â”‚                                                                            â”‚
â”‚  Latence typique : 5-20 ms (local)                                         â”‚
â”‚                                                                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.3 Ã‰tats d'une requÃªte

```
                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                    â”‚ PENDING â”‚
                    â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜
                         â”‚
              â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
              â”‚     VALIDATING      â”‚
              â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                         â”‚
         â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
         â–¼               â–¼               â–¼
    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
    â”‚REJECTED â”‚    â”‚ EXECUTINGâ”‚    â”‚ QUEUED   â”‚
    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”˜
                        â”‚               â”‚
                        â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜
                                â–¼
                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                    â”‚    COMPLETED      â”‚
                    â”‚  (success/error)  â”‚
                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 6. ObservabilitÃ©

### 6.1 MÃ©triques collectÃ©es

| CatÃ©gorie | MÃ©trique | Type | Description |
|-----------|----------|------|-------------|
| **RequÃªtes** | `km_requests_total` | Counter | Total requÃªtes par opÃ©rateur/type |
| | `km_request_duration_ms` | Histogram | Latence des requÃªtes |
| | `km_request_errors` | Counter | Erreurs par type |
| **Arbitrage** | `km_auth_failures` | Counter | Ã‰checs d'authentification |
| | `km_permission_denials` | Counter | Refus de permission |
| | `km_validation_errors` | Counter | Erreurs de validation |
| **Base** | `km_db_connections` | Gauge | Connexions actives par base |
| | `km_db_queries_total` | Counter | RequÃªtes SQL exÃ©cutÃ©es |
| | `km_db_size_bytes` | Gauge | Taille des fichiers DB |
| **SystÃ¨me** | `km_memory_bytes` | Gauge | MÃ©moire utilisÃ©e |
| | `km_uptime_seconds` | Counter | Temps depuis dÃ©marrage |

### 6.2 Audit Log

Chaque opÃ©ration est journalisÃ©e dans un log d'audit structurÃ© :

```json
{
  "timestamp": "2026-02-08T14:30:00Z",
  "request_id": "uuid-xxx",
  "operator": "jayxpose",
  "operation": "write",
  "database": "jayxpose",
  "table": "exposants",
  "entity_id": "exp-123",
  "result": "success",
  "duration_ms": 12,
  "arbitrage": {
    "auth": "pass",
    "permission": "pass",
    "validation": "pass",
    "quota": "pass"
  }
}
```

### 6.3 Health Check

Le Core Server expose un endpoint de santÃ© :

| Check | Description | Sain si |
|-------|-------------|---------|
| **alive** | Processus en cours | Toujours vrai si rÃ©pond |
| **ready** | PrÃªt Ã  servir | Toutes les bases ouvertes |
| **db_health** | Chaque base accessible | RequÃªte `SELECT 1` rÃ©ussit |

---

## 7. Gestion des Erreurs

### 7.1 Codes d'erreur

| Code | Signification | Action client |
|------|---------------|---------------|
| `OK` | SuccÃ¨s | - |
| `UNAUTHENTICATED` | Token manquant/invalide | RÃ©-authentifier |
| `PERMISSION_DENIED` | Pas le droit | VÃ©rifier permissions |
| `INVALID_ARGUMENT` | DonnÃ©es invalides | Corriger payload |
| `NOT_FOUND` | EntitÃ© inexistante | - |
| `ALREADY_EXISTS` | Duplication | - |
| `RESOURCE_EXHAUSTED` | Quota dÃ©passÃ© | Retry aprÃ¨s dÃ©lai |
| `INTERNAL` | Erreur serveur | Reporter le bug |
| `UNAVAILABLE` | Service temporairement indisponible | Retry |

### 7.2 Retry Policy

| Erreur | Retry ? | StratÃ©gie |
|--------|---------|-----------|
| `UNAUTHENTICATED` | Non | RÃ©-authentification nÃ©cessaire |
| `PERMISSION_DENIED` | Non | Configuration nÃ©cessaire |
| `INVALID_ARGUMENT` | Non | Correction payload nÃ©cessaire |
| `NOT_FOUND` | Non | - |
| `RESOURCE_EXHAUSTED` | Oui | Backoff exponentiel |
| `INTERNAL` | Oui | Max 3 tentatives |
| `UNAVAILABLE` | Oui | Backoff exponentiel |

---

## 8. IntÃ©gration avec les Autres Cores

### 8.1 StrongFather

| Interaction | Direction | Description |
|-------------|-----------|-------------|
| Validation intention | SF â†’ KM | StrongFather valide l'intention avant Ã©criture |
| RÃ©sultat persistance | KM â†’ SF | KindMother confirme la persistance |

### 8.2 WorrySentinel

| Interaction | Direction | Description |
|-------------|-----------|-------------|
| Alerte sÃ©curitÃ© | KM â†’ WS | DÃ©tection de patterns suspects |
| RÃ©vocation mandat | WS â†’ KM | Blocage d'un opÃ©rateur compromis |

### 8.3 Caring Nanny

| Interaction | Direction | Description |
|-------------|-----------|-------------|
| MÃ©triques santÃ© | KM â†’ CN | Ã‰tat du Core Server |
| DÃ©tection anomalie | CN â†’ KM | Alerte sur patterns anormaux |

---

## 9. Invariants du Core Server

Ces invariants sont **non nÃ©gociables** et garantis par l'architecture :

| ID | Invariant | Violation = |
|----|-----------|-------------|
| **INV-SRV-1** | Toute requÃªte passe par le pipeline d'arbitrage | Faille de gouvernance |
| **INV-SRV-2** | Aucun accÃ¨s direct aux fichiers DB | Contournement Core |
| **INV-SRV-3** | La clÃ© de chiffrement reste en mÃ©moire | Fuite de secret |
| **INV-SRV-4** | Chaque opÃ©ration est auditÃ©e | Perte de traÃ§abilitÃ© |
| **INV-SRV-5** | Les erreurs ne rÃ©vÃ¨lent pas les donnÃ©es | Fuite d'information |
| **INV-SRV-6** | Un opÃ©rateur n'accÃ¨de qu'Ã  ses bases | Violation isolation |

---

## 10. RÃ©fÃ©rences

- [KindMother - Documentation Fondatrice](../foundation/KindMother%20-%20Documentation%20Fondatrice.md)
- [KindMother - Client](./KindMother%20-%20Client.md)
- [Systeme Persistance libSQL Migration](../implementation/KindMother%20-%20Systeme%20Persistance%20libSQL%20Migration.md)
- [Security - Gouvernance Cores Protection Donnees](..//..//WorrySentinel//_index.md)

---

**Date de crÃ©ation :** 2026-02-08  
**Version :** 1.0  
**Statut :** ARCHITECTURE â€” Document de rÃ©fÃ©rence  
**Auteur :** Architecture Miyukini

