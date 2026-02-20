# MGE — MSCM & MIP Governance

Obligations de balisage MSCM, politique d'ID, règles d'intégrité, structure des blocs et intégration au pipeline MIP pour le MGE.

## Contexte

Le MGE est gouverné par le protocole MSCM (Miyukini Semantic Code Markup) et MIP (MSCM Index Protocol). Tout module public du MGE doit être balisé pour permettre l'indexation, la gouvernance et l'attestation d'intégrité lors de la vérification MWS (Phase B).

## Portée / Scope

- **Applicable à :** Code MGE (mge-core, mge-plugins, mge-render, mge-cog-bridge), développeurs moteur.
- **Audience :** Développeurs moteur, mainteneurs, pipelines CI.
- **Statut :** Spécification normative.

---

## 1. Obligations MSCM

### 1.1 Modules publics

- Tout **module public** (crate, lib, module `pub mod`) doit avoir un bloc MSCM en en-tête.
- Les blocs sont placés dans les commentaires doc (`//!`) au début du fichier ou du module.

### 1.2 Champs obligatoires

| Champ | Obligatoire | Description |
|-------|-------------|-------------|
| `@id` | Oui | Identifiant unique global du bloc |
| `@do` | Oui | Description fonctionnelle courte |
| `@role` | Non | Rôle sémantique (simulation, rendering, data, security...) |
| `@layer` | Non | Couche architecturale (core, plugin, bridge) |
| `@human` | Non | Description humaine lisible |

### 1.3 Exemple minimal

```rust
//! @id mge.core.engine
//! @role simulation
//! @layer core
//! @human Moteur principal : cycle de vie, tick, seed RNG
//! @do orchestrate_game_loop_and_plugin_lifecycle
```

---

## 2. Politique d'ID

### 2.1 Préfixes par couche

| Préfixe | Couche | Exemple |
|---------|--------|---------|
| `mge.core.` | mge-core | `mge.core.engine`, `mge.core.world`, `mge.core.scheduler` |
| `mge.plugin.` | mge-plugins | `mge.plugin.physics`, `mge.plugin.input` |
| `mge.render.` | mge-render | `mge.render.backend`, `mge.render.camera` |
| `mge.cog.` | mge-cog-bridge | `mge.cog.runtime`, `mge.cog.service` |

### 2.2 Format

- **Structure** : `mge.{layer}.{module}.{sous_module}` (optionnel)
- **Unicité** : Un ID ne doit apparaître qu'une fois dans tout le codebase.
- **Stabilité** : Les IDs ne changent pas sans versioning majeur (breaking).

### 2.3 Exemples par composant

```
mge.core.engine
mge.core.world
mge.core.scheduler
mge.core.event_bus
mge.core.rng
mge.core.time
mge.plugin.physics.collision
mge.plugin.physics.movement
mge.plugin.input.keyboard
mge.plugin.network.snapshot
mge.render.backend_minifb
mge.render.backend_wgpu
mge.cog.runtime
mge.cog.service
```

---

## 3. Règles d'intégrité

### 3.1 ID unique global

- Aucun doublon d'ID dans le projet.
- Vérification par le générateur MIP ; erreur si doublon détecté.

### 3.2 Aucun bloc orphelin

- Tout bloc doit être référençable (fichier, module parent, dépendance).
- Un bloc sans parent ni référence dans la hiérarchie est considéré orphelin ; warning ou erreur selon configuration.

### 3.3 Pas de cycle invalide

- Les dépendances logiques entre blocs ne doivent pas former de cycle incohérent.
- Ex. : mge.core ne doit pas dépendre de mge.plugin.

### 3.4 Hiérarchie cohérente

- La hiérarchie parent-enfant reflète la structure des modules.
- `mge.core.engine` est enfant de `mge.core` (si défini).

### 3.5 Pas de conflit layer

- Un bloc ne doit pas déclarer un `@layer` incompatible avec son préfixe ID.
- Ex. : un bloc `mge.plugin.*` doit avoir `@layer` plugin ou compatible.

---

## 4. Structure attendue des blocs

### 4.1 Emplacement

- En tête de fichier : `//!` pour les modules Rust.
- Un bloc par fichier de module public significatif.

### 4.2 Exemples MSCM dans mge-core

**engine.rs**
```rust
//! @id mge.core.engine
//! @role simulation
//! @layer core
//! @human Point d'entrée du moteur : init, run, shutdown
//! @do orchestrate_game_loop_and_plugin_lifecycle
```

**world.rs**
```rust
//! @id mge.core.world
//! @role simulation
//! @layer core
//! @human Stockage entités et composants (SoA)
//! @do store_entities_and_components
```

**scheduler.rs**
```rust
//! @id mge.core.scheduler
//! @role simulation
//! @layer core
//! @human Ordonnancement déterministe des systèmes
//! @do execute_systems_in_deterministic_order
```

**event_bus.rs**
```rust
//! @id mge.core.event_bus
//! @role simulation
//! @layer core
//! @human Communication inter-plugins par événements typés
//! @do route_typed_events_between_plugins
```

---

## 5. Intégration au pipeline MIP

### 5.1 Scan

- Le générateur MIP (`tools/mip-generator/`) scanne le codebase.
- Les crates MGE sont inclus dans le scan (`crates/mge-*`).

### 5.2 Parse MSCM

- Extraction des blocs MSCM depuis les commentaires.
- Validation des champs obligatoires (@id, @do).

### 5.3 Génération index

- `mscm_index/blocks.json` : liste des blocs avec file, start_line, end_line, role, layer, do, human.
- `mscm_index/files.json` : cartographie fichier → blocs.
- `mscm_index/layers.json` : projection par couche (core, plugin, render, cog).

### 5.4 Intégration CI

- Le pipeline MIP peut être exécuté en CI.
- Erreur si règles d'intégrité violées (IDs dupliqués, blocs orphelins, etc.).

---

## 6. Vérification MWS (Phase B)

- Lors de la **vérification de conformité** par un relay MWS, la **Phase B** demande des blocs de code au COG.
- Le relay envoie un ID de bloc (ex. `mge.core.engine`).
- Le COG renvoie le contenu du bloc (chiffré) ; le relay vérifie avec les références Origin.
- Les blocs MSCM/MIP du MGE servent ainsi à l'**attestation d'intégrité** des jeux qui utilisent le MGE.

---

## 7. Références

| Document | Rôle |
|----------|------|
| [Protocole MIP](../contrats/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) | Spécification complète MIP. |
| [Skill MSCM/MIP](../../.cursor/skills/miyukini-mscm-mip/SKILL.md) | Règles MSCM, format blocks.json. |
| [MWS - Flux de Vérification](../miyukini-webway-system/verification/MWS%20-%20Flux%20de%20Verification.md) | Phase B, attestation. |
| [MGE - Architecture Générale](./MGE%20-%20Architecture%20Generale.md) | Couches MGE (core, plugin, render, cog). |

---

**Document** : MGE — MSCM & MIP Governance  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Spécification normative
