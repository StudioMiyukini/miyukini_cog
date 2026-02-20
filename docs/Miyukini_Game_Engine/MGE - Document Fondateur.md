# MGE — Document Fondateur

Document fondateur du Miyukini Game Engine (MGE) : vision, philosophie, positionnement et rôle dans l'écosystème Miyukini COG.

## Contexte

Le MGE est le moteur de jeu maison de l'écosystème Miyukini COG. Il vise à fournir une base technique professionnelle, légère et extensible pour des jeux 2D simulation-first : ARPG, MMO-lite, simulation sociale, batailles à grande échelle, jeux solo offline ou host authoritative via le MWS (Miyukini Webway System).

## Portée / Scope

- **Applicable à :** Architecture MGE, philosophie moteur, décisions fondatrices, alignement COG.
- **Audience :** Développeurs moteur, développeurs tiers, LLM, architectes.
- **Statut :** Document fondateur normatif.

---

## 1. Vision

> **Un moteur généraliste 2D simulation-first**, microkernel, déterministe, CPU-aware, headless-capable et IA-friendly, conçu pour durer 5+ ans et supporter des ambitions massives sans compromettre la simplicité de base.

| Ambition | Description |
|----------|-------------|
| **Simulation-first** | La logique de simulation est le cœur. Le rendu est une vue optionnelle sur l'état. |
| **Généraliste** | Pas destiné à un seul genre ; aucune hypothèse implicite (Position, grille, combat, tick rate). Le moteur ne suppose aucun gameplay. Voir [§8 Non-objectifs](#8-non-objectifs-explicites). |
| **2D prioritaire** | 2D natif ; 3D optionnel. **Le core est spatial-agnostic** : il ignore la dimension ; un plugin spatial (2D ou 3D) l'introduit. |
| **Long terme** | Blueprint stable pour 5+ ans ; pas de pivot architectural prévu. |
| **Open-engine** | Documentation et API pensées pour des développeurs tiers et des LLM. |

---

## 2. Positionnement face au marché

### 2.1 Comparaison avec les moteurs existants

| Moteur | Caractéristiques | Différence MGE |
|--------|------------------|----------------|
| **Godot** | Tout-en-un, scène graphique, GDScript, éditeur intégré. | MGE : microkernel, pas d'éditeur intégré, Rust natif. Simulation découplée du rendu. |
| **Bevy** | ECS pur, rendering-first, data-driven, tout en une boucle. | MGE : simulation-first ; le rendu est un plugin. Headless natif. Déterministe explicite. |
| **Unity** | Propriétaire, lourd, C#/Burst, écosystème fermé. | MGE : Rust, open, binaire statique, pas de runtime externe. |
| **Piston / ggez** | 2D léger, immediate-mode, pas d'ECS. | MGE : ECS + plugins, scheduling ordonné, RNG déterministe. |
| **Amethyst** | ECS, abandonné. | MGE : suite l'esprit ECS mais sans dépendance à un crate particulier. |

### 2.2 Ce que le MGE n'est pas

- **Pas un moteur monolithique** — le cœur est minimal ; tout le reste est plugin.
- **Pas un moteur rendering-first** — la simulation tourne sans fenêtre ni GPU.
- **Pas une couche au-dessus d'un moteur tiers** — le MGE est autonome ; le rendu est assuré par des plugins (minifb, wgpu, etc.).
- **Pas une dépendance critique** — conformité LOI-1 : binaires statiques, pas de runtime externe.

---

## 3. Philosophie fondatrice — Six piliers

### 3.1 Simulation-first

La simulation est la source de vérité. Le rendu est une projection de l'état.

- La boucle principale met à jour l'état du monde (entités, composants, événements).
- Le rendu lit l'état et l'affiche ; il ne modifie pas la simulation.
- En mode headless, la simulation tourne sans aucun rendu.
- Cas d'usage : replay, serveur dédié, IA, tests, déterministe.

### 3.2 Deterministic-first

Reproductibilité garantie par un seed et un ordre fixe.

- RNG déterministe : même seed → même séquence.
- Ordre des systèmes fixe et documenté.
- Pas de dépendance au temps système pour la logique (option fixed timestep).
- Cas d'usage : lockstep optionnel, replay, debug, synchronisation réseau.

### 3.3 Microkernel

Le cœur minimal : Engine, World, Scheduler, EventQueue, RNG, Time.

- Aucune physique, rendu, audio ou réseau dans le core.
- Tout est plugin : physics, render, input, audio, ai, network.
- Le moteur orchestre ; les plugins exécutent.
- Cas d'usage : jeux minimalistes sans physique, serveurs headless, configurations légères.

### 3.4 Plugin-driven

Extensibilité par composition, pas par héritage. **Isolation stricte** : les plugins ne se connaissent pas.

- Trait `Plugin` : `fn build(&self, engine: &mut Engine)`.
- Les plugins enregistrent composants, systèmes, et abonnements événements.
- **Communication uniquement via World et EventQueue** — aucun appel direct entre plugins. Sinon on recrée un couplage vertical (PluginCombat → PluginPhysics → PluginAI).
- Les dépendances déclaratives servent uniquement à l'ordre de build, jamais à des imports croisés.
- Cas d'usage : jeux custom, outils tiers, backends rendu interchangeables.

### 3.5 CPU-aware

Maîtrise explicite du coût et du budget. **Distinction cruciale** : Tick ≠ Frame.

- **Tick** = unité de simulation (Scheduler) ; **Frame** = unité de rendu. Le core ne dépend que du concept **tick**.
- Budget CPU par tick configurable (pas par frame — le frame est du ressort du plugin rendu).
- LOD comportemental : Full, Reduced, Sleep.
- Pas d'allocation cachée dans le hot path.
- Profiling intégré : métriques par système.
- Cas d'usage : 10k+ entités simulées, mobiles, serveurs.

### 3.6 Headless-capable

Exécution sans fenêtre ni GPU.

- Mode headless natif : pas de création de fenêtre.
- Simulation identique en headless et en mode rendu.
- Cas d'usage : serveurs dédiés, bots, tests, CI, reprise de partie.

---

## 4. Rôle dans l'écosystème Miyukini

### 4.1 Hiérarchie des concepts

| Élément | Rôle |
|--------|------|
| **MGE** | Bibliothèque moteur (lib) ; fournit Engine, World, plugins. |
| **Jeu (ex. Allumina)** | Service Inter-COG (Type 3) ; exécutable séparé ; consomme MGE. |
| **Miyukini Central** | Launcher ; affiche le catalogue, lance les jeux (exe séparés). |
| **Cores** | KindMother (sauvegardes), StrongFather (autorisation), MWS (réseau). |

### 4.2 Flux standard

```
Utilisateur → Central → Lance jeu (.exe) → GameRuntime (MGE + jeu)
                              ↓
                    Jeu = processus séparé
                              ↓
                    BondingBrother → Cores (KindMother, MWS, etc.)
```

Le MGE ne parle jamais directement aux Cores. Toute persistance ou réseau passe par BondingBrother (Strate 5) qui médie avec les Cores (Strate 4).

### 4.3 Conformité LOI (Lois d'Autonomie)

| Loi | Application MGE |
|-----|-----------------|
| **LOI-1** | Binaires statiques, pas de runtime critique. Moteur maison = crate Rust. |
| **LOI-2** | Jeu jouable hors-ligne en solo ; MWS optionnel pour le multijoueur. |
| **LOI-3** | État local souverain ; sauvegardes via KindMother, données maîtrisées. |
| **LOI-4** | Pas de temps global requis ; simulation locale, sync par accord. |
| **LOI-5** | Coût proportionnel au hardware ; budget CPU, LOD, pas de surcoût inutile. |
| **LOI-6** | Autonomie n'empêche pas la fédération ; multijoueur via MWS et Lobbys. |
| **LOI-7** | Évolution du moteur par versions ; pas de rupture dans les Cores. |
| **LOI-8** | Migration = processus formel ; jamais copie brute entre environnements. |

---

## 5. Cibles long terme

Le MGE est conçu pour supporter à terme :

- **ARPG massifs** — centaines d'entités à l'écran, pathfinding, loot, instances.
- **MMO-lite** — Lobbys via MWS, host authoritative, réplication snapshot/delta.
- **Simulation sociale** — NPCs, réputation, économie, groupes.
- **Batailles à grande échelle** — milliers d'unités, LOD comportemental.
- **Jeux solo offline** — sans aucune dépendance réseau.
- **Jeux host authoritative via MWS** — découverte Lobbys, accord d'hôte, transport.

**Implications techniques** (détaillées dans la [Core Specification](./MGE%20-%20Core%20Specification%20Technique.md) et [Performance Philosophy](./MGE%20-%20Performance%20Philosophy.md)) : stockage SoA, parallélisme contrôlé, isolation du hot path, distinction systèmes read-only vs write, pipeline de réplication.

---

## 6. IA-friendly et audit-able

- **Documentation structurée** : Contexte, Portée, sections claires. Utilisable par des LLM.
- **MSCM & MIP** : Balisage sémantique du code ; index pour gouvernance et attestation.
- **Pas de fluff marketing** : Orienté ingénierie, décisions justifiées.
- **Blueprint solide** : Base pour Composer, pour 5+ ans.

---

## 8. Non-objectifs explicites

Pour protéger le moteur contre les dérives futures, le MGE **ne fournit pas** :

| Non-objectif | Description |
|--------------|-------------|
| **Éditeur intégré** | Pas d'éditeur visuel type Godot/Unity. Les jeux utilisent des outils externes ou des formats texte/binaire. |
| **Système de scène hiérarchique** | Pas de graphe de scène parent-enfant imposé par le core. Un plugin peut en fournir un. |
| **Pipeline asset imposé** | Pas de format ou workflow asset prédéfini. Chaque jeu gère ses assets. |
| **Gameplay préconçu** | Pas de combat, inventaire, quêtes ou genre intégré. Le core est agnostique du gameplay. |

Ces choix évitent les hypothèses implicites et gardent le moteur généraliste.

---

## 9. Références

| Document | Rôle |
|----------|------|
| [MGE - Architecture Générale](./MGE%20-%20Architecture%20Generale.md) | Couches, diagrammes, responsabilités. |
| [MGE - Core Specification Technique](./MGE%20-%20Core%20Specification%20Technique.md) | Engine, World, Scheduler, EventQueue, RNG, Time ; garanties généraliste et spatial-agnostic. |
| [MGE - Plugin Contract](./MGE%20-%20Plugin%20Contract.md) | Trait Plugin, isolation via World/EventQueue. |
| [MGE - Performance Philosophy](./MGE%20-%20Performance%20Philosophy.md) | SoA, hot path, réplication. |
| [Miyukini - Moteur Jeux et Central Launcher](./reference/Miyukini%20-%20Moteur%20Jeux%20et%20Central%20Launcher.md) | Architecture jeux + Central. |
| [MWS - Document Fondateur](../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | MWS : présence, découverte, transport. |
| [Allumina - Document Fondateur](../services/Allumina/Allumina%20-%20Document%20Fondateur.md) | Cas d'usage : jeu Action RPG sur MGE. |

---

**Document** : MGE — Document Fondateur  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Document fondateur normatif
