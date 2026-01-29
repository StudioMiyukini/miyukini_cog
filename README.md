# Miyukini Core System

> *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

**Miyukini est un COG — Core-Orchestrated Governance Environment.** Un environnement de gouvernance orchestré par des cores, qui coordonne, sécurise et fait fonctionner des systèmes logiciels complets, du noyau jusqu'à l'utilisateur final.

---

## Table des matières

### Partie I — Vue d'ensemble
1. [Introduction](#1-introduction)
2. [À qui s'adresse le projet](#2-à-qui-sadresse-le-projet)
3. [Pourquoi Miyukini existe](#3-pourquoi-miyukini-existe)

### Partie II — Structure du dépôt
4. [Arborescence du projet](#4-arborescence-du-projet)
5. [Cartographie des crates](#5-cartographie-des-crates)
6. [Documentation](#6-documentation)

### Partie III — Concepts fondamentaux
7. [Vision et philosophie](#7-vision-et-philosophie)
8. [La pyramide Miyukini](#8-la-pyramide-miyukini)
9. [Lois d'autonomie](#9-lois-dautonomie)
10. [Cores système (résumé)](#10-cores-système-résumé)

### Partie IV — Gouvernance et sécurité
11. [Sécurité et gouvernance](#11-sécurité-et-gouvernance)
12. [MiyukiniAdmin](#12-miyukiniadmin)

### Partie V — Positionnement
13. [Cas d'usage](#13-cas-dusage)
14. [Comparatif avec l'existant](#14-comparatif-avec-lexistant)
15. [Apports inédits](#15-apports-inédits)
16. [À qui Miyukini n'est PAS destiné](#16-à-qui-miyukini-nest-pas-destiné)

### Partie VI — État et référence
17. [État du projet](#17-état-du-projet)
18. [Documentation de référence](#18-documentation-de-référence)
19. [Personal Vibe Coding Gouverné](#19-personal-vibe-coding-gouverné)
20. [Conclusion](#20-conclusion)
21. [Mini log de rédaction](#21-mini-log-de-rédaction)

---

## Partie I — Vue d'ensemble

### 1. Introduction

**Qu'est-ce que Miyukini**

Miyukini Core System (MCS) est un **écosystème logiciel gouverné** conçu pour produire des applications autonomes, sécurisées structurellement, et capables de fonctionner dans des conditions de contrainte extrême (offline, ressources limitées, environnements isolés).

Miyukini n'est pas un framework. Ce n'est pas une bibliothèque. C'est un **environnement gouverné dans lequel des Opérateurs opèrent**.

La distinction est fondamentale : un framework fournit des outils que le développeur utilise comme bon lui semble. Miyukini fournit un **cadre non négociable** dans lequel les Opérateurs opèrent selon des règles strictes, des invariants vérifiables, et une gouvernance centralisée.

---

### 2. À qui s'adresse le projet

| Acteur | Besoin |
|--------|--------|
| **Architectes système** | Autonomie structurelle, sécurité par conception, traçabilité auditable, fonctionnement déterministe en isolation |
| **Développeurs d'Opérateurs** | Collectivités, événements sans réseau fiable, IoT/edge, contextes réglementés |
| **Décideurs techniques** | Projets long terme (5–10 ans), systèmes critiques, contrôle total non négociable |

---

### 3. Pourquoi Miyukini existe

Les architectures logicielles modernes reposent sur des hypothèses implicites : connectivité permanente, ressources cloud élastiques, services tiers toujours accessibles. Ces hypothèses excluent une part significative des cas d'usage réels.

Miyukini adopte la posture inverse : **la déconnexion est un état normal du système, pas une erreur à corriger**.

Résultat : systèmes qui démarrent sans réseau, fonctionnent sans cloud, dégradent proprement en isolation, restent administrables localement, et évoluent quand le réseau revient (sans reconstruction).

---

## Partie II — Structure du dépôt

### 4. Arborescence du projet

```
miyukini_kernel/
├── Cargo.toml              # Workspace Rust (crates + tools)
├── README.md               # Ce document
├── crates/                 # Composants Miyukini (Kernel + Cores + MiyukiniAdmin)
│   ├── miyukini-kernel/    # Substrat technique (Strate K)
│   ├── strongfather/       # Core décision (Strate 4)
│   ├── kindmother/         # Core données (Strate 4)
│   ├── borderguard/        # Core frontières (Strate 4)
│   ├── caringnanny/        # Core observation d'état (Strate 4)
│   ├── masterbutler/       # Core capacités (Strate 4)
│   ├── bondingbrother/    # Médiation (Strate 5)
│   ├── everbuddy/          # Core cycle de vie (Strate 4)
│   ├── worrysentinel/      # Core gouvernance sécurité (Strate 4)
│   ├── tamr/               # Core intervention humaine (Strate 4)
│   ├── logisticssteward/  # Gestion ressources (Strate 4)
│   └── miyukini-admin/     # Opérateur Souverain (Strate 9)
├── tools/
│   └── mip-generator/      # Génération index MIP (MSCM)
├── docs/                   # Documentation conceptuelle et contractuelle
│   ├── core/               # Documentation par Core (StrongFather, KindMother, …)
│   ├── reference/          # Références conceptuelles (Glossaire, Pyramide, Lois)
│   ├── protocols/          # Protocoles (MIP, écriture doc, implémentation)
│   ├── kernel/             # Kernel (architecture, contrats, tests)
│   ├── implementation/    # Implémentation COG 0.1
│   ├── security/           # Sécurité
│   └── tools/              # Outils (MiyuSQL, etc.)
├── mscm_index/             # Index MIP (généré, ne pas modifier à la main)
└── .cursor/                # Plans et contexte Cursor
```

**Règles de lecture :**

- **Code** = dans `crates/` et `tools/`.
- **Documentation normative** = dans `docs/` (conceptuelle, contrats, protocoles).
- **Index structurel** = `mscm_index/` (reconstruit, jamais édité manuellement).

---

### 5. Cartographie des crates

Chaque crate du workspace correspond à une strate ou à un Core de la pyramide Miyukini.

| Crate | Rôle conceptuel | Strate |
|-------|-----------------|--------|
| `miyukini-kernel` | Substrat technique (Id, Logger, Clock, Config, Lifecycle) | K (Kernel) |
| `strongfather` | Moteur de décision ; émetteur des Mandats de Permission | 4 |
| `kindmother` | Autorité données et persistance ; WriteIntent, sync | 4 |
| `borderguard` | Définition des frontières et niveaux de confiance | 4 |
| `caringnanny` | Observation d'état ; santé, métriques | 4 |
| `masterbutler` | Registre des capacités et permissions | 4 |
| `everbuddy` | Cycle de vie, versions, compatibilité, migration | 4 |
| `worrysentinel` | Gouvernance sécurité ; niveaux de confiance (T0–T4) | 4 |
| `tamr` | Intervention humaine ; classification, métadonnées | 4 |
| `logisticssteward` | Gestion des ressources (allocation, optimisation) | 4 |
| `bondingbrother` | Médiation ; traduction intentions ↔ cores | 5 |
| `miyukini-admin` | Opérateur Souverain ; administration, diagnostic | 9 |

**Outillage :**

| Outil | Rôle |
|-------|------|
| `mip-generator` | Génère l'index MIP (registry, blocks, hierarchy, graph, etc.) à partir du code balisé MSCM |

---

### 6. Documentation

La documentation suit la [nomenclature Miyukini](docs/) et est organisée par **cadre** (core, reference, protocols) et **sujet**.

| Dossier | Contenu |
|---------|---------|
| `docs/core/` | Par Core : fondation, contrats, implémentation, référence (StrongFather, KindMother, MiyukiniAdmin, etc.) |
| `docs/reference/` | Références conceptuelles transverses : Glossaire, Pyramide, Lois d'autonomie, COG, Opérateurs, Tools, Souveraineté |
| `docs/protocols/` | Protocoles (MIP, écriture documentation conceptuelle, implémentation générale) |
| `docs/kernel/` | Kernel : architecture, contrats, tests |
| `docs/implementation/` | Implémentation COG 0.1 (gel, conformité, quick reference) |
| `docs/security/` | Politiques et contrats de sécurité |
| `docs/tools/` | Documentation des outils (ex. MiyuSQL) |

**Entrée recommandée :** [Glossaire officiel](docs/reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) (source de vérité terminologique).

---

## Partie III — Concepts fondamentaux

### 7. Vision et philosophie

**Systèmes autonomes**

Un système Miyukini est **autonome** au sens strict : démarrable sans réseau, fonctionnel sans cloud, dégradé proprement en isolation, prévisible sans synchronisation, administrable localement, évolutif à la reconnexion. Cette autonomie est structurelle et vérifiable.

**Séparation stricte des responsabilités**

- **Décision** (StrongFather) ≠ **Exécution** (Opérateurs, adaptateurs) ≠ **Persistance** (KindMother). Aucun core n’empiète sur les autres.

**IA gouvernée, non magique**

Si une IA intervient : elle propose, les cores valident ; contrats explicites ; décisions traçables et auditables ; pas de contournement des invariants.

**Environnement isolé**

Conçu pour réseau intermittent ou absent, hardware contraint, temps non synchronisé. Les **8 lois d'autonomie** (voir ci-dessous) codifient ces contraintes.

---

### 8. La pyramide Miyukini

L'écosystème est organisé en **strates hiérarchiques**, dépendance strictement unidirectionnelle (de haut en bas).

```
┌──────────────────────────────────────────────────┐
│ 🔧 STRATE 9 — MiyukiniAdmin (EXCEPTION)            │
│ Opérateur Souverain d'administration               │
└──────────────────────────────────────────────────┘
                      ▲
┌──────────────────────────────────────────────────┐
│ 🟩 STRATE 7 — OPÉRATEURS                           │
│ Entités fonctionnelles gouvernées                  │
└──────────────────────────────────────────────────┘
                      ▲
┌──────────────────────────────────────────────────┐
│ 🟦 STRATE 6 — TOOLS & TOOLKITS                     │
│ Capacités exécutables gouvernées                   │
└──────────────────────────────────────────────────┘
                      ▲
┌──────────────────────────────────────────────────┐
│ 🟨 STRATE 5 — INTERFACES & ADAPTATION              │
│ BondingBrother                                     │
└──────────────────────────────────────────────────┘
                      ▲
┌──────────────────────────────────────────────────┐
│ 🟥 STRATE 4 — CORES SYSTÈME                        │
│ StrongFather · KindMother · Caring Nanny · …       │
└──────────────────────────────────────────────────┘
                      ▲
┌──────────────────────────────────────────────────┐
│ 🟪 STRATE 3 — INVARIANTS & CONTRATS                │
└──────────────────────────────────────────────────┘
                      ▲
┌──────────────────────────────────────────────────┐
│ ⚙️  KERNEL — Id · Logger · Clock · Config · Lifecycle │
└──────────────────────────────────────────────────┘
                      ▲
┌──────────────────────────────────────────────────┐
│ 🟫 STRATE 0 — HARDWARE & OS                        │
└──────────────────────────────────────────────────┘
```

Le **Kernel** fournit : Id, Logger, Clock (trace only), Config, Lifecycle. Il ne contient ni logique métier ni protocole applicatif ni dépendance externe critique.

---

### 9. Lois d'autonomie

Les **8 lois d'autonomie** sont des invariants architecturaux non négociables.

| Loi | Énoncé |
|-----|--------|
| **LOI-1** | Aucune dépendance externe critique à l'exécution |
| **LOI-2** | Le système accepte l'isolement comme état normal |
| **LOI-3** | L'état local est souverain |
| **LOI-4** | Pas de temps global requis |
| **LOI-5** | Le coût doit être proportionnel au hardware |
| **LOI-6** | L'autonomie n'empêche pas la fédération |
| **LOI-7** | La strate Cores est immuable — évolution par environnement |
| **LOI-8** | Migration = diplomatie entre environnements |

Question de conception : *« Est-ce que ça fonctionne encore si le système est seul, lent, et isolé ? »*

---

### 10. Cores système (résumé)

| Core | Question fondamentale | Rôle en une phrase |
|------|------------------------|---------------------|
| **StrongFather** | Devrait-on faire cette action ? | Décision pure ; émetteur des Mandats de Permission |
| **KindMother** | Comment les données sont-elles persistées et synchronisées ? | Autorité données et persistance |
| **Caring Nanny** | Dans quel état se trouve le système ? | Observation d'état, pas de modification |
| **Master Butler** | Qu'est-ce qui peut être fait, et qui a le droit ? | Registre capacités et permissions |
| **Border Guard** | Où sont les frontières et les règles de franchissement ? | Définition des frontières |
| **Ever Buddy** | Comment le système évolue-t-il sans se rompre ? | Cycle de vie, versions, compatibilité |
| **TAMR** | Quand l'humain a-t-il le droit d'intervenir ? | Intervention humaine gouvernée |
| **WorrySentinel** | Quel niveau de sécurité et quel état de confiance ? | Gouvernance sécurité (T0–T4, niveaux 0–4) |
| **BondingBrother** | Comment traduire les intentions pour les autorités ? | Médiation uniquement, jamais d'autorité |

Détail de chaque Core : voir `docs/core/<NomDuCore>/`.

---

## Partie IV — Gouvernance et sécurité

### 11. Sécurité et gouvernance

- **Zero-trust** : invariant architectural ; aucun appelant présumé valide ; toute intention évaluée selon les politiques.
- **Niveaux de sécurité** (0–4) : profil de risque du produit ; gouvernés par WorrySentinel.
- **États de confiance** (T0–T4) : Normal, Instable, Dégradé, Restreint, Bloqué ; dégradation progressive, pas de blocage brutal.
- **Offline-first** : invariant ; WriteIntent acceptés localement, réconciliation explicite à la reconnexion.

---

### 12. MiyukiniAdmin

**MiyukiniAdmin** est un **Opérateur Souverain** (Strate 9), exception à la logique Opérateur standard. Rôle : installation, diagnostic, arbitrage, accès exceptionnel. Il n'est pas utilisé par les autres Opérateurs ; il n'expose pas d'API publique.

Pouvoirs : installer et configurer l'écosystème, consulter états et métriques, modifier niveaux de sécurité, intervenir en maintenance. Toute action est traçable, horodatée, justifiable, auditable. Mal utilisé, il peut compromettre l'intégrité ; il est strictement encadré (niveau de sécurité maximal, journalisation obligatoire).

Documentation : `docs/core/MiyukiniAdmin/`.

---

## Partie V — Positionnement

### 13. Cas d'usage

- **B2B** : livrable = Outils & Kits d'Outils (Strate 6) ; briques recomposables, contrats stables.
- **B2C** : livrable = Opérateurs (Strate 7) ; opérateur clé en main, offline, administration via MiyukiniAdmin.
- **B2B2C** : Opérateurs + Outils sous licence ; revendeurs personnalisent et revendent.
- **Environnements contraints** : collectivités, événements, IoT/edge, zones blanches, réglementation, long terme.
- **Temps réel critique** (<100 ms) : non adapté (latence des contrôles). **Asynchrone** : adapté.

---

### 14. Comparatif avec l'existant

| Critère | Frameworks classiques | Cloud-native | Miyukini |
|---------|------------------------|--------------|----------|
| Autonomie | Dépendante | Cloud requis | Offline-first |
| Sécurité | Configuration | Service | Architecture |
| Gouvernance | Développeur | Plateforme | Cores |
| Évolution | Breaking changes | Versions | Souveraineté environnement |
| Complexité | Accumulée | Distribuée | Collaboration gouvernée |
| Traçabilité | Optionnelle | Service externe | Structurelle |

Miyukini ne concurrence pas WordPress, Laravel ou Kubernetes ; il répond à un autre problème : systèmes autonomes, sécurisés par conception, capables de fonctionner en contrainte extrême.

---

### 15. Apports inédits

- **Décision pure sans exécution** (StrongFather) : élimination d’une classe de bugs et failles.
- **IA gouvernée par contrats** : pas de boîte noire ; tout comportement explicable.
- **Architecture modulaire réelle** : autorité exclusive par core, frontières explicites et vérifiables.
- **Dégradation progressive** : T0→T4 sans blocage brutal.
- **Sécurité structurelle** : zero-trust et niveaux de sécurité comme paramètres de gouvernance, pas choix applicatif.

---

### 16. À qui Miyukini n'est PAS destiné

- Projets nécessitant une mise en production immédiate sans comprendre l’architecture.
- Applications **temps réel critique** (latence <100 ms).
- Équipes refusant les contraintes de gouvernance et d’invariants.
- Projets sans exigence d’autonomie (connectivité permanente, dépendance cloud assumée).
- Startups en phase d’exploration pure où l’architecture structurée serait un frein.

---

## Partie VI — État et référence

### 17. État du projet

**Maturité conceptuelle** : Pyramide, Cores, Lois d'autonomie, contrats de sécurité et gouvernance d’écosystème sont stabilisés et documentés.

**Implémentation (workspace actuel)** :

- **Kernel** : `miyukini-kernel` (Id, Logger, Clock, Config, Lifecycle).
- **Cores Strate 4** : `strongfather`, `kindmother`, `borderguard`, `caringnanny`, `masterbutler`, `everbuddy`, `worrysentinel`, `tamr`, `logisticssteward`.
- **Médiation** : `bondingbrother`.
- **Opérateur Souverain** : `miyukini-admin`.
- **Outil** : `mip-generator` (index MIP).

**Roadmap implicite** : Kernel → Cores → MiyukiniAdmin → Outils & Kits d'Outils → Opérateurs. L’étape Outils & Kits d’Outils reste la clé stratégique qui différencie Miyukini d’un simple framework.

---

### 18. Documentation de référence

| Thème | Document principal |
|-------|--------------------|
| **Terminologie** | [Glossaire officiel](docs/reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **Architecture** | [Pyramide Architecture Complete](docs/reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| **Autonomie** | [Lois Autonomie Système](docs/reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |
| **COG** | [Definition COG](docs/reference/Miyukini%20Conceptual%20References%20-%20Definition%20COG.md) |
| **Opérateurs** | [Operators et Terminologie](docs/reference/Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| **Tools** | [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |

**Protocoles** : `docs/protocols/` (MIP, écriture documentation conceptuelle, implémentation générale).

**Cores** : `docs/core/<NomDuCore>/` (fondation, contrats, implémentation).

---

### 19. Personal Vibe Coding Gouverné

Miyukini est développé selon une approche expérimentale : **Personal Vibe Coding Gouverné**. Le vibe coding (développement assisté par IA, rapide, intuitif) est encadré par une architecture contractuelle stricte. L’IA génère ; les contrats gouvernent ; les invariants ne négocient pas.

**Thèse** : si on peut « vibe coder » un écosystème complet en respectant une architecture rigoureuse, le vibe coding devient une méthode de production viable, pas seulement un outil de prototypage. Miyukini est autant un produit qu’une expérimentation méthodologique.

---

### 20. Conclusion

Miyukini ne vise pas à être le plus rapide ni le plus flexible, mais **prévisible, traçable, autonome et structurellement sécurisé**. Il demande un investissement initial (architecture en strates, contraintes de gouvernance, invariants) en échange de garanties : fonctionnement déterministe en isolation, sécurité par conception, évolution sans rupture, traçabilité complète.

*« Miyukini n'est pas une bibliothèque. C'est un environnement gouverné dans lequel des Opérateurs opèrent. »*

---

### 21. Mini log de rédaction

**2026-01-29 — Réorganisation README racine**

- Structure en **6 parties** : Vue d’ensemble, Structure du dépôt, Concepts fondamentaux, Gouvernance et sécurité, Positionnement, État et référence.
- **Nouvelles sections** : Arborescence du projet (4), Cartographie des crates (5), Documentation (6) avec mapping dossiers / contenu.
- **État du projet** aligné sur le workspace actuel (Cargo.toml) : suppression des références aux crates SPM-CMS et démos retirées ; ajout de `logisticssteward`, `mip-generator`.
- Table des matières réorganisée avec ancres par partie.

**2026-01-28 — Alignement Glossaire v1.7**

- Terminologie : Opérateurs, Outils & Kits d’Outils, Lois LOI-7 et LOI-8, section Documentation de référence.

**2026-01-27 — Section Personal Vibe Coding Gouverné**

- Ajout de la section 11 (devenant 19) et clarification de l’expérimentation méthodologique.

---

**Document** : README racine officiel  
**Dernière mise à jour** : 2026-01-29  
**Références** : Glossaire officiel, Pyramide Architecture Complete, Lois d’autonomie
