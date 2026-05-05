# Pyramide des Strates

## Architecture Hiérarchique de Miyukini

Le système Miyukini est organisé en **strates hiérarchiques**, chacune avec un rôle précis et des responsabilités définies. Cette architecture garantit la séparation des préoccupations et la gouvernance par les Cores.

## Vue d'Ensemble

```
┌─────────────────────────────────────────┐
│         STRATE 9 - MiyukiniAdmin        │  Opérateur Souverain
├─────────────────────────────────────────┤
│         STRATE 7 - Opérateurs           │  JayKoa, JayKonta...
├─────────────────────────────────────────┤
│         STRATE 6 - Outils               │  MiyuAuth, MiyuSQL...
├─────────────────────────────────────────┤
│         STRATE 5 - Interface            │  BondingBrother
├─────────────────────────────────────────┤
│         STRATE 4 - Cores                │  8 Cores de Gouvernance
├─────────────────────────────────────────┤
│         STRATE 3 - Invariants           │  Principes & Contrats
├─────────────────────────────────────────┤
│         STRATE K - Kernel               │  Substrat Technique
├─────────────────────────────────────────┤
│         STRATE 0 - Hardware             │  OS & Matériel
└─────────────────────────────────────────┘
```

---

## Strate 0 : Hardware & OS

**La réalité physique**

Cette strate représente le matériel et le système d'exploitation sous-jacent. Miyukini est agnostique vis-à-vis de cette couche.

| Aspect | Description |
|--------|-------------|
| Contenu | CPU, RAM, stockage, OS (Windows, Linux, macOS) |
| Responsabilité | Exécution physique des instructions |
| Contrôle | Hors du système Miyukini |

---

## Strate K : Kernel

**Le substrat technique neutre**

Le Kernel est la fondation technique de Miyukini. Il fournit les primitives de base sans aucune logique métier.

| Aspect | Description |
|--------|-------------|
| Contenu | Primitives système, génération d'ID, bootstrap |
| Responsabilité | Démarrage et infrastructure technique |
| Principe | Aucune logique métier, neutralité absolue |
| Crate | `miyukini-kernel` |

---

## Strate 3 : Invariants & Contrats

**Les principes architecturaux**

Cette strate contient les règles fondamentales qui ne peuvent jamais être violées. Elle n'a pas de code propre mais définit les contrats que toutes les autres strates doivent respecter.

| Aspect | Description |
|--------|-------------|
| Contenu | Les 8 Lois d'Autonomie, contrats de sécurité |
| Responsabilité | Définition des invariants système |
| Manifestation | Intégré dans le code des Cores |

---

## Strate 4 : Cores Système

**Les 8 piliers de gouvernance**

La strate la plus critique. Les 8 Cores gouvernent l'ensemble du système. Ils **décident** mais n'**exécutent** jamais directement.

| Core | Rôle |
|------|------|
| **StrongFather** | Orchestration et pilotage |
| **KindMother** | Persistance et gestion des données |
| **TAMR** | Accès et permissions |
| **BorderGuard** | Protection des frontières |
| **WorrySentinel** | Surveillance et alertes |
| **LogisticsSteward** | Gestion des ressources |
| (+ 2 autres) | Fonctions complémentaires |

**Principe fondamental** : Les Cores sont **immuables** (LOI-7). Une fois déployés, ils ne changent jamais.

---

## Strate 5 : Interface & Adaptation

**BondingBrother**

BondingBrother est l'interface unique entre les Cores (Strate 4) et les Outils (Strate 6). Il traduit les décisions des Cores en instructions pour les Outils.

| Aspect | Description |
|--------|-------------|
| Contenu | BondingBrother |
| Responsabilité | Adaptation et traduction |
| Flux | Cores → BondingBrother → Outils |
| Crate | `bondingbrother` |

---

## Strate 6 : Outils & Kits d'Outils

**Les capacités exécutables**

49 toolkits fournissent les capacités techniques du système. Les Outils **font** mais ne **décident** jamais.

| Catégorie | Exemples |
|-----------|----------|
| Authentification | MiyuAuth |
| Base de données | MiyuSQL |
| Communication | MiyuWeb, MiyuWebwayParticipant |
| Interface | MiyuWidgets |
| Stockage | MiyuStore |

**Principe** : Un Outil exécute ce qu'on lui demande, sans autonomie décisionnelle.

---

## Strate 7 : Opérateurs

**Les services fonctionnels**

Les Opérateurs sont les services visibles par l'utilisateur. Ils sont **gouvernés** par les Cores et utilisent les Outils pour fonctionner.

| Opérateur | Domaine |
|-----------|---------|
| JayKoa | Calendrier universel |
| JayKonta | Comptabilité |
| JayRDV | Rendez-vous |
| JayShop | Commerce |
| JayFaim | Restauration et tables |
| Jay Bureau | Suite collaborative |
| Miyukini Cloud | Cloud privé |
| Miyukini Central | Hub principal |
| Miou | Assistant IA |

**Principe** : Un Opérateur est lié à un seul environnement (COG).

---

## Strate 9 : MiyukiniAdmin

**L'opérateur souverain**

MiyukiniAdmin est une exception architecturale : un opérateur avec des privilèges spéciaux pour l'administration du système.

| Aspect | Description |
|--------|-------------|
| Contenu | Panneau d'administration |
| Responsabilité | Configuration, monitoring, maintenance |
| Privilèges | Accès direct aux Cores (exception) |
| Crate | `miyukini-admin` |

---

## Flux Standard

Le flux typique d'une action utilisateur :

```
Utilisateur
    │
    ▼
Service (Interface utilisateur)
    │
    ▼
Opérateur (Strate 7)
    │
    ▼
BondingBrother (Strate 5)
    │
    ▼
Cores (Strate 4) ──► Décision
    │
    ▼
BondingBrother (Strate 5)
    │
    ▼
Outils (Strate 6) ──► Exécution
    │
    ▼
Résultat
```

## Règles Inter-Strates

| Règle | Description |
|-------|-------------|
| Descente uniquement | Une strate haute ne peut appeler qu'une strate inférieure |
| Pas de saut | Impossible de sauter une strate intermédiaire |
| Gouvernance centralisée | Toute décision passe par les Cores |
| Exécution déléguée | Seuls les Outils exécutent les actions |
