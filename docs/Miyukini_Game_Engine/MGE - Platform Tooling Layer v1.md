# MGE — Platform Tooling Layer v1

## Contexte

Le MGE évolue vers une **architecture plateforme** : le Kernel reste pur (simulation pure), tandis qu'une couche d'outils gravite autour sans polluer le runtime. Objectif : Kernel propre, runtime léger, outils puissants, SQL réservé au développement, IA intégrée intelligemment, briques rangées dans des bacs bien séparés.

## Portée / Scope

- **Applicable à :** Conception plateforme MGE, développement outils, workflow Allumina.
- **Audience :** Architectes, designers, développeurs moteur.
- **Statut :** Spécification normative v1.

---

## 1. Positionnement architectural

### Chaîne runtime (inchangée)

```
Kernel → Plugins → GCL → Runtime
```

Le Kernel reste pur. Aucun outil ne le modifie.

### Ajout : Tooling Layer

```
MGE Platform
├── Kernel (inchangé)
├── Plugins (inchangés)
├── GCL (Game Composition Layer)
│
└── Tooling Layer
    └── tools/ (8 outils)
```

Chaque outil :

| Principe | Description |
|----------|-------------|
| **Ne modifie jamais le kernel** | Lecture seule du Kernel/Plugins |
| **Produit des données** | Export vers runtime |
| **Peut utiliser SQL** | Uniquement en développement |
| **Peut intégrer IA** | Assistant connecté aux outils, pas au runtime |

---

## 2. Vision globale — Structure `mge/`

```
mge/
├── crates/           # Kernel + Plugins + Packs (existant)
│   ├── mge-core/
│   ├── mge-plugin-*/
│   └── {rpg,rts,social,...}/
├── gcl/              # Game Composition Layer (assemblage)
│
├── tools/            # Outils édition (Tooling Layer)
│   ├── data-authoring/
│   ├── prefab-editor/
│   ├── balance-lab/
│   ├── battle-sandbox/
│   ├── sprite-tool/
│   ├── rule-editor/
│   ├── export-pipeline/
│   └── ai-assist/
│
└── export/           # Sortie pipeline
    └── runtime_data/
```

---

## 3. Séparation des bacs

| Bac | Contenu |
|-----|---------|
| **Kernel** | Simulation pure (World, Time, RNG, EventQueue, etc.) |
| **Plugins** | Mécaniques (spatial, combat, physique, …) |
| **GCL** | Assemblage jeu (plugins actifs, config) |
| **Tools** | Édition (SQL, prefabs, balance, règles) |
| **Export** | Transformation données dynamiques → statiques |
| **Runtime** | Exécution (Engine, tick, rendu) |

**Aucun mélange.** Un bac ne déborde jamais dans un autre.

---

## 4. Flux complet

```
Design (game design)
        │
        ▼
┌─────────────────┐
│  Data Authoring │  SQL Tool — éditer stats, unités, factions, loot
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Balance Lab    │  Simuler sans lancer le jeu
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Prefab Editor  │  Créer entités visuellement
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Export Pipeline │  Validation → Optimisation → Export
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Runtime Data    │  /export/runtime_data/
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  GCL            │  Assemblage jeu
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Engine         │  tick(), simulation, rendu
└─────────────────┘
```

---

## 5. Les 8 outils — Synopsis

| Outil | Rôle | Sortie |
|-------|------|--------|
| **Data Authoring** | Édition game design (SQL) | → Export Pipeline |
| **Prefab Editor** | Création visuelle entités | Prefabs |
| **Balance Lab** | Simulation paramètres hors jeu | Graphiques, CSV, recommandations IA |
| **Battle Sandbox** | Mini runtime isolé (formations, LOD) | Tests visuels |
| **Sprite Tool** | Assets 2D (pivot, hitbox, atlas) | Metadata |
| **Rule Editor** | Règles gameplay déclaratives | Config rule-engine |
| **Export Pipeline** | SQL/JSON → runtime statique | `/export/runtime_data/` |
| **AI Assist** | Assistant IA connecté aux outils | Suggestions, génération |

Détail de chaque outil : [Index des outils](./platform-tools/_index.md).

---

## 6. Avantages

| Avantage | Effet |
|----------|-------|
| **Développement rapide** | Édition SQL, balance sans build complet |
| **Runtime pur** | Aucune dépendance outil en exécution |
| **SQL utile mais non intrusif** | Dev uniquement, jamais embarqué |
| **IA intégrée intelligemment** | Outils assistés, pas le kernel |
| **Expérience Construct-like** | Workflow visuel, itération rapide |
| **Architecture propre** | Bacs séparés, flux unidirectionnel |
| **Simulation-first respectée** | Kernel inchangé |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [MGE - Architecture Générale](./MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Pack Architecture](./MGE%20-%20Pack%20Architecture.md) | Packs, composition |
| [Index des outils](./platform-tools/_index.md) | Détail des 8 outils |

---

**Document** : MGE — Platform Tooling Layer v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Statut** : Spécification normative
