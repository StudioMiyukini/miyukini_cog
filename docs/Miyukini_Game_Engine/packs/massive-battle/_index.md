# MGE — Pack Massive Battle

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/massive-battle/`  
**Nombre de crates** : 6  

---

## 1. Contexte

Le Pack Massive Battle gere les combats a grande echelle : formations militaires, unites de groupe, moral, tactiques de champ de bataille, approvisionnement et sieges. Il s'appuie sur le Pack RPG pour les stats individuelles et le combat, et sur le Core Universal Pack pour le spatial et la physique.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Jeux de guerre tactique, batailles en masse (Total War, Mount & Blade), wargames.
- **Hors portee** : Logique specifique a un jeu, rendu, audio, reseau.
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack (spatial, basic-physics). Pack RPG (stats, combat).

---

## 3. Vision

Le Pack Massive Battle est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/massive-battle/
├── mge-mb-formation/       # Formations, rangs, colonnes, slots
├── mge-mb-unit/            # Squads, cohesion, ordres de groupe
├── mge-mb-morale/          # Moral, panique, brisure, routage
├── mge-mb-tactics/         # Flancs, charge, retraite, manoeuvres
├── mge-mb-supply/          # Logistique, munitions, ravitaillement
└── mge-mb-siege/           # Assiegeants, defenseurs, murs, engins
```

### Graphe de dependances intra-pack

```
mge-mb-tactics ──► mge-mb-formation
     │
     └──────────► mge-mb-morale ──► mge-mb-unit ──► mge-mb-formation

mge-mb-supply ──► mge-mb-unit

mge-mb-siege ──► mge-mb-unit
     │
     └──────────► mge-mb-supply
```

Crates feuilles (sans dependance intra-pack) : `mge-mb-formation`.

---

## 5. Sous-packs

Aucun. Les 6 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-mb-formation` | `mge.mb.formation.v1` | [mge-mb-formation.md](mge-mb-formation.md) | Formations geometriques, rangs, colonnes, slots |
| 2 | `mge-mb-unit` | `mge.mb.unit.v1` | [mge-mb-unit.md](mge-mb-unit.md) | Regroupement soldats, cohesion, ordres de groupe |
| 3 | `mge-mb-morale` | `mge.mb.morale.v1` | [mge-mb-morale.md](mge-mb-morale.md) | Moral, panique, brisure, routage |
| 4 | `mge-mb-tactics` | `mge.mb.tactics.v1` | [mge-mb-tactics.md](mge-mb-tactics.md) | Flancs, charge, retraite, manoeuvres tactiques |
| 5 | `mge-mb-supply` | `mge.mb.supply.v1` | [mge-mb-supply.md](mge-mb-supply.md) | Logistique, munitions, ravitaillement, depots |
| 6 | `mge-mb-siege` | `mge.mb.siege.v1` | [mge-mb-siege.md](mge-mb-siege.md) | Assiegeants, defenseurs, murs, engins de siege |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|-------------------|------------------------------|
| formation | Formation, FormationSlot, FormationOffset, FormationMembership | aucun |
| unit | Squad, SquadMember, Cohesion, GroupOrder, UnitBanner | aucun |
| morale | Morale, PanicState, RoutBehavior, MoraleConfig | aucun |
| tactics | TacticalStance, FlankBonus, ChargeState, ManeuverOrder | aucun |
| supply | SupplyStock, SupplyLine, Depot, SupplyConsumer | aucun |
| siege | SiegeParticipant, WallSection, SiegeEngine, GateState | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 900-903 | formation | assign_formation_slots, compute_formation_positions, rotate_formation, compact_formation |
| 910-913 | unit | process_group_orders, update_cohesion, check_squad_integrity, rally_scattered |
| 920-923 | morale | update_morale, check_panic_threshold, process_rout, spread_panic |
| 930-933 | tactics | detect_flank_opportunity, execute_charge, apply_tactical_bonuses, process_retreat |
| 940-943 | supply | consume_supplies, update_supply_lines, check_depot_status, apply_supply_penalty |
| 950-954 | siege | update_siege_engines, apply_wall_damage, check_breach, process_assault, update_gate |

**Ordre d'execution** : formation (900) → unit (910) → morale (920) → tactics (930) → supply (940) → siege (950).

**Justification** : les formations sont calculees en premier car les unites s'y referent. Les unites sont mises a jour avant le moral qui en depend. Les tactiques lisent moral et formations. Le supply affecte les unites. Le siege est en dernier car il combine tous les systemes precedents.

**Total** : 25 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| formation | (aucun, ecriture directe) | SlotAssigned, FormationRotated, FormationCompacted, FormationBroken |
| unit | OrderIssued | SquadBroken, SquadRallied, MemberLost |
| morale | (aucun, lit les events) | MoraleBroken, PanicTriggered, RoutStarted, MoraleRestored |
| tactics | ManeuverRequested | FlankDetected, ChargeImpact, RetreatOrdered, ManeuverComplete |
| supply | ResupplyRequest | SupplyDepleted, DepotDestroyed, SupplyLineCut, ResupplyReceived |
| siege | AssaultOrder | WallBreached, GateDestroyed, SiegeEngineDestroyed, AssaultLaunched, SiegeLifted |

**Total** : 3 requests + 21 events = 24 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 6 crates | `mge-ecs`, `mge-event` |

### Dependances inter-pack

| Crate | Depend de |
|-------|-----------|
| Tous les 6 crates | Pack RPG (`mge-rpg-stats`, `mge-rpg-combat`) |
| formation, unit, tactics | Core Universal (`mge-plugin-spatial`) |
| siege | Core Universal (`mge-plugin-basic-physics`) |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-mb-unit` | `mge-mb-formation` |
| `mge-mb-morale` | `mge-mb-unit` |
| `mge-mb-tactics` | `mge-mb-formation`, `mge-mb-morale` |
| `mge-mb-supply` | `mge-mb-unit` |
| `mge-mb-siege` | `mge-mb-unit`, `mge-mb-supply` |

### Dependances externes (aucune)

Le Pack Massive Battle n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL (Game Composition Layer) configure les plugins Massive Battle sans recompilation.

**Parametres exposables :**

- Taille max des formations, espacement
- Seuils de moral, taux de panique
- Bonus flancs et charge
- Taux consommation munitions
- Resistance murs, degats engins

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates MB utilises | Usage |
|----------------|---------------------|-------|
| **Grand Strategy** | formation, tactics | Batailles resolues avec formations |
| **RTS** | unit, tactics | Micro-management unites |

Le Pack Massive Battle depend de :
- **Pack RPG** : stats individuelles des soldats, resolution degats
- **Core Universal** : positions spatiales, physique de base

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Utiliser operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Seed RNG** | Moral et tactics utilisent le RNG kernel (mge-rng) |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | formation (positions), morale (updates), tactics (bonuses) |
| **Budget cible** | < 5ms pour 10000 soldats a 30 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path (pre-allouer) |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de formations 3D | Simplification v1, 2D seulement |
| Pas de meteo sur le combat | Hors scope v1 |
| Pas de moral individuel | Moral par squad uniquement |
| Pas de siege naval | Voir extension v2 |
| Pas de ravitaillement dynamique par route | Lignes fixes uniquement |
| Pas de fortifications modulaires | Murs predecoupes en sections |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| Formations 3D | Support terrain 3D et hauteurs |
| Meteo / terrain | Impact pluie, boue, neige sur moral et mouvement |
| Moral individuel | Moral par soldat en plus du squad |
| Siege naval | Blocus, debarquement, artillerie navale |
| Supply routes dynamiques | Pathfinding supply avec risque d'interception |
| Fortifications modulaires | Construction et destruction par blocs |
| Campements | Bases temporaires avec bonus defensifs |

---

## 17. Exemple d'assemblage

### Minimal (headless, formation + unit)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeRpgStatsPlugin);
engine.add_plugin(MgeRpgCombatPlugin);
engine.add_plugin(MgeMbFormationPlugin);
engine.add_plugin(MgeMbUnitPlugin);
engine.build();
```

### Complet (bataille massive)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginBasicPhysics::default());
// Pack RPG
engine.add_plugin(MgeRpgStatsPlugin);
engine.add_plugin(MgeRpgCombatPlugin);
// Pack Massive Battle
engine.add_plugin(MgeMbFormationPlugin);
engine.add_plugin(MgeMbUnitPlugin);
engine.add_plugin(MgeMbMoralePlugin);
engine.add_plugin(MgeMbTacticsPlugin);
engine.add_plugin(MgeMbSupplyPlugin);
engine.add_plugin(MgeMbSiegePlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/massive-battle/
├── mge-mb-formation/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.mb.formation.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-mb-unit/
│   └── (meme structure)
├── mge-mb-morale/
│   └── (meme structure)
├── mge-mb-tactics/
│   └── (meme structure)
├── mge-mb-supply/
│   └── (meme structure)
└── mge-mb-siege/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack Massive Battle est la brique fondamentale des combats a grande echelle dans MGE. Il :

- Fournit 6 plugins couvrant formations, unites, moral, tactiques, supply et sieges.
- Reste generique : aucune logique specifique a un jeu.
- S'execute en headless, en deterministe, sans rendu.
- Depend du Pack RPG pour les stats et le combat individuel.
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 6 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
