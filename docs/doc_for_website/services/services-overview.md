# Vue d'Ensemble des Services

## Strate 7 : Services et Opérateurs

Les **Services** (ou Opérateurs) sont les entités fonctionnelles visibles par l'utilisateur. Situés à la Strate 7, ils sont gouvernés par les Cores et utilisent les Outils pour fonctionner.

## Principe Fondamental

> Les Services sont **gouvernés**, jamais **autonomes**.

Un Service ne prend aucune décision de gouvernance. Il propose des fonctionnalités à l'utilisateur, mais chaque action est validée et contrôlée par les Cores.

## Catalogue des Services

### Services Centraux

| Service | Description | Statut |
|---------|-------------|--------|
| **Miyukini Central** | Hub principal et point d'accès | Fonctionnel |
| **Miou** | Assistant IA intégré à Central | Fonctionnel |
| **MiyukiniAdmin** | Administration (Strate 9) | Fonctionnel |

### Services Professionnels (Jay)

| Service | Description | Statut |
|---------|-------------|--------|
| **JayKoa** | Calendrier universel du COG | Fonctionnel |
| **JayKonta** | Comptabilité unifiée Purse + Account | Fonctionnel (Bourse) |
| **JayRDV** | Prise de rendez-vous et créneaux | Beta |
| **JayShop** | Commerce, point de vente, billetterie | En préparation |
| **JayFaim** | Réservation de tables et restauration | En préparation |
| **MiyukiniSales** | Cycle devis → facture → paiement | En préparation |

### Suite Bureautique (Jay Bureau)

| Service | Description | Statut |
|---------|-------------|--------|
| **Jay Bureau Hub** | Lanceur de la suite collaborative | Fonctionnel |
| **Jay Docs** | Éditeur de documents collaboratif (CRDT Yrs) | Fonctionnel |
| **Jay Sheets** | Tableur | Fonctionnel |
| **Jay Slides** | Présentations avec thumbnails | Fonctionnel |
| **Jay Formulaire** | Builder de formulaires | Fonctionnel |
| **Jay Reunion** | Visioconférence (lobby + stage) | Fonctionnel |
| **Jay Club** | Réseau social type Meta | Fonctionnel |
| **Jay Mail** | Client email SMTP + IMAP | Fonctionnel |
| **Jay Message** | Messagerie chiffrée bout-en-bout (Signal-like) | Fonctionnel |

### Services Sociaux & Médias

| Service | Description | Statut |
|---------|-------------|--------|
| **Jay1Tribu** | Messagerie P2P, tribus, salons et amis | En préparation |
| **JayManga** | Lecture et boutique manga/BD | En préparation |

### Services Cloud & IA

| Service | Description | Statut |
|---------|-------------|--------|
| **Miyukini Cloud** | Cloud privé (WebDAV/CalDAV/CardDAV) | Fonctionnel |
| **MAIA / Miou LLM** | IA locale (LLM, STT) sans internet | Fonctionnel |
| **Miyukini Whisper** | Dictée locale STT/TTS | Fonctionnel |
| **Alicia** | Assistante domotique (vocal, MQTT) | MVP |

### Services de Monitoring

| Service | Description | Statut |
|---------|-------------|--------|
| **MiyukiniWatch** | Métriques et surveillance utilisateur | Fonctionnel |

### Services Ludiques

| Service | Description | Statut |
|---------|-------------|--------|
| **MiyukiniClicker** | Jeu idle/clicker avec gestion de cité | Jouable |
| **MiyukiniSurvivor** | Jeu Survivor + Tower Defense | Jouable |
| **MiyukiniLifeGame** | Simulation de vie | En préparation |
| **MGE** | Moteur de jeu multijoueur | En cours |

## Flux Standard

```
Utilisateur ──► Service ──► Cores ──► BondingBrother ──► Outils ──► Exécution
```

## Caractéristiques Communes

- **Gouvernés** par les Cores
- **Liés** à un seul COG
- **Interface** graphique intégrée à Central
- **Permissions** validées par TAMR

## Services retirés

Les services suivants ont été retirés du périmètre du projet (voir [DEPRECATED.md](../../services/DEPRECATED.md) pour les détails) :

- **JayXpose** — Profil exposant, vitrine produit, catalogue (retiré le 2026-04-29)
- **JayFestival** — Gestion de festivals, éditions, exposants, visiteurs (retiré le 2026-04-29)
