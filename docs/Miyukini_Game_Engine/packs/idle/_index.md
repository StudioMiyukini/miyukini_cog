# MGE — Pack Idle

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/idle/`  
**Nombre de crates** : 5  

---

## 1. Contexte

Le Pack Idle fournit les mecaniques generiques des jeux idle/incremental : producteurs automatiques, multiplicateurs, upgrades achetables, prestige et progression hors-ligne. Il est leger et compose bien avec les packs Tycoon et RPG.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Clicker, idle games, incrementals, auto-battlers (couche economique).
- **Hors portee** : Rendu, UI (prestige screen), monetisation, logique de jeu specifique.
- **Audience** : Developpeurs moteur, designers, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack recommande.

---

## 3. Vision

Le Pack Idle est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/idle/
├── mge-idle-producer/      # Producteurs automatiques, generation ressources
├── mge-idle-upgrade/       # Upgrades achetables, effets
├── mge-idle-multiplier/    # Multiplicateurs, bonus empilables
├── mge-idle-prestige/      # Reset prestige, bonus permanents
└── mge-idle-offline/       # Progression hors-ligne, rattrapage
```

### Graphe de dependances intra-pack

```
mge-idle-offline ──────► mge-idle-producer ──────► mge-idle-multiplier
                              │
mge-idle-prestige ────────────┘
                              │
mge-idle-upgrade ─────────────┘
```

Crates feuilles (sans dependance intra-pack) : `mge-idle-multiplier`.

---

## 5. Sous-packs

Aucun. Les 5 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-idle-producer` | `mge.idle.producer.v1` | [mge-idle-producer.md](mge-idle-producer.md) | Producteurs automatiques, taux de production, generation de ressources |
| 2 | `mge-idle-upgrade` | `mge.idle.upgrade.v1` | [mge-idle-upgrade.md](mge-idle-upgrade.md) | Upgrades achetables, conditions, effets sur producteurs/multiplicateurs |
| 3 | `mge-idle-multiplier` | `mge.idle.multiplier.v1` | [mge-idle-multiplier.md](mge-idle-multiplier.md) | Multiplicateurs empilables, sources de bonus, calcul global |
| 4 | `mge-idle-prestige` | `mge.idle.prestige.v1` | [mge-idle-prestige.md](mge-idle-prestige.md) | Reset prestige, monnaie prestige, bonus permanents |
| 5 | `mge-idle-offline` | `mge.idle.offline.v1` | [mge-idle-offline.md](mge-idle-offline.md) | Progression hors-ligne, calcul gains au retour |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|--------------------|------------------------------|
| producer | Producer, ProductionRate, ResourceOutput, ProducerCount | aucun |
| upgrade | UpgradeState, UpgradeCost, UpgradeEffect | UpgradeDef |
| multiplier | Multiplier, MultiplierStack, GlobalMultiplier | aucun |
| prestige | PrestigeState, PrestigeCurrency, PermanentBonus | PrestigeTier |
| offline | OfflineState, OfflineEarnings, LastPlayedTimestamp | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 2100-2102 | producer | tick_producers, accumulate_resources, apply_production_rate |
| 2110-2112 | upgrade | validate_upgrade_purchase, apply_upgrade_effect, scale_upgrade_cost |
| 2120-2122 | multiplier | compute_global_multiplier, apply_multiplier_to_production, expire_temporary_multipliers |
| 2130-2132 | prestige | evaluate_prestige_readiness, execute_prestige_reset, apply_permanent_bonus |
| 2140-2142 | offline | compute_offline_duration, calculate_offline_earnings, apply_offline_earnings |

**Ordre d'execution** : producer (2100) → upgrade (2110) → multiplier (2120) → prestige (2130) → offline (2140).

**Justification** : les producteurs generent les ressources de base. Les upgrades modifient les producteurs. Les multiplicateurs s'appliquent au resultat. Le prestige evalue sur le total courant. L'offline est calcule au lancement uniquement.

**Total** : 15 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| producer | (aucun, tick automatique) | ResourceProduced |
| upgrade | PurchaseUpgradeRequest | UpgradePurchased, UpgradeFailed |
| multiplier | (aucun, lecture) | MultiplierChanged, MultiplierExpired |
| prestige | PrestigeRequest | PrestigeExecuted, PrestigeBonusGranted |
| offline | (aucun, au login) | OfflineEarningsCalculated |

**Total** : 2 requests + 8 events = 10 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 5 crates | `mge-ecs`, `mge-event` |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-idle-producer` | `mge-idle-multiplier` |
| `mge-idle-upgrade` | `mge-idle-producer` |
| `mge-idle-prestige` | `mge-idle-producer` |
| `mge-idle-offline` | `mge-idle-producer` |

### Dependances externes (aucune)

Le Pack Idle n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL (Game Composition Layer) configure les plugins Idle sans recompilation.

**Parametres exposables :**

- Taux de production de base par type de producteur
- Courbe de cout des upgrades (base, facteur de scaling)
- Plafond multiplicateur global
- Seuils de prestige, formule de monnaie prestige
- Duree maximale offline, taux de rendement offline

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates Idle utilises | Usage |
|----------------|----------------------|-------|
| **Tycoon** | producer, multiplier | Revenus facilities comme producteurs |

Le Pack Idle ne depend d'aucun autre pack genre.

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Seed RNG** | Aucun aleatoire dans le pack Idle |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | producer (tick), multiplier (compute) |
| **Budget cible** | < 1ms pour 10 000 producteurs a 60 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de producteurs composes (output d'un = input d'un autre) | Simplification v1 |
| Pas de synergies inter-producteurs | Extension v2 |
| Pas de prestige multi-couche | Un seul niveau de prestige |
| Pas d'offline partiel (accelerateur) | Tout ou rien |
| Pas d'automation (auto-buy) | Hors scope, UI layer |

---

## 16. Exemple d'assemblage

### Minimal (headless, producteur uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeIdleProducerPlugin);
engine.add_plugin(MgeIdleMultiplierPlugin);
engine.build();
```

### Complet (Idle game jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginSaveLoad::default());
// Pack Idle
engine.add_plugin(MgeIdleProducerPlugin);
engine.add_plugin(MgeIdleUpgradePlugin);
engine.add_plugin(MgeIdleMultiplierPlugin);
engine.add_plugin(MgeIdlePrestigePlugin);
engine.add_plugin(MgeIdleOfflinePlugin);
engine.build();
```

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
