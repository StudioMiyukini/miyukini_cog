# MGE — Pack Grand Strategy

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/grand-strategy/`  
**Nombre de crates** : 10  

---

## 1. Contexte

Le Pack Grand Strategy gere les mecaniques fondamentales des jeux de grande strategie : diplomatie, economie, commerce, armees, demographie, religion, culture, provinces, decisions strategiques et casus belli. Il s'appuie sur le Pack RPG (stats, combat) et le Pack Social (factions) pour les fondations.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Grande strategie (Crusader Kings, Europa Universalis, Victoria, Hearts of Iron), 4X, jeux de conquete.
- **Hors portee** : Logique specifique a un jeu, rendu, audio, reseau.
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Pack RPG (stats, combat). Pack Social (factions, reputation).

---

## 3. Vision

Le Pack Grand Strategy est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/grand-strategy/
├── mge-gs-diplomacy/       # Relations diplomatiques, traites, alliances
├── mge-gs-economy/         # Production, tresor, inflation
├── mge-gs-trade/           # Routes commerciales, echanges, embargo
├── mge-gs-military/        # Armees, recrutement, entretien
├── mge-gs-population/      # Demographie, croissance, migration
├── mge-gs-religion/        # Systemes religieux, conversion
├── mge-gs-culture/         # Groupes culturels, assimilation
├── mge-gs-province/        # Territoires, controle, developpement
├── mge-gs-decision/        # Decisions strategiques, conditions, effets
└── mge-gs-cb/              # Casus belli, justifications guerre, legitimite
```

### Graphe de dependances intra-pack

```
mge-gs-trade ──► mge-gs-economy
mge-gs-military ──► mge-gs-economy
     │
     └──────────► mge-gs-population
mge-gs-religion ──► mge-gs-population
mge-gs-culture ──► mge-gs-population
mge-gs-decision ──► mge-gs-economy
     │
     └──────────► mge-gs-diplomacy
mge-gs-cb ──► mge-gs-diplomacy
```

Crates feuilles (sans dependance intra-pack) : `mge-gs-diplomacy`, `mge-gs-economy`, `mge-gs-population`, `mge-gs-province`.

---

## 5. Sous-packs

Aucun. Les 10 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-gs-diplomacy` | `mge.gs.diplomacy.v1` | [mge-gs-diplomacy.md](mge-gs-diplomacy.md) | Relations diplomatiques, traites, alliances |
| 2 | `mge-gs-economy` | `mge.gs.economy.v1` | [mge-gs-economy.md](mge-gs-economy.md) | Production, tresor, inflation, budget |
| 3 | `mge-gs-trade` | `mge.gs.trade.v1` | [mge-gs-trade.md](mge-gs-trade.md) | Routes commerciales, echanges, embargo |
| 4 | `mge-gs-military` | `mge.gs.military.v1` | [mge-gs-military.md](mge-gs-military.md) | Armees, recrutement, entretien, attrition |
| 5 | `mge-gs-population` | `mge.gs.population.v1` | [mge-gs-population.md](mge-gs-population.md) | Demographie, croissance, migration |
| 6 | `mge-gs-religion` | `mge.gs.religion.v1` | [mge-gs-religion.md](mge-gs-religion.md) | Systemes religieux, conversion, autorite |
| 7 | `mge-gs-culture` | `mge.gs.culture.v1` | [mge-gs-culture.md](mge-gs-culture.md) | Groupes culturels, assimilation, traditions |
| 8 | `mge-gs-province` | `mge.gs.province.v1` | [mge-gs-province.md](mge-gs-province.md) | Territoires, controle, developpement |
| 9 | `mge-gs-decision` | `mge.gs.decision.v1` | [mge-gs-decision.md](mge-gs-decision.md) | Decisions strategiques, conditions, effets |
| 10 | `mge-gs-cb` | `mge.gs.cb.v1` | [mge-gs-cb.md](mge-gs-cb.md) | Casus belli, justifications guerre, legitimite |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|-------------------|------------------------------|
| diplomacy | DiplomaticStance, Treaty, DiplomaticAction | aucun |
| economy | Treasury, EconomicOutput, Inflation | aucun |
| trade | TradeRoute, TradeAgreement, Embargo | aucun |
| military | Army, MilitaryUnit, Recruitment, Maintenance | aucun |
| population | Population, Migration, PopGrowth | aucun |
| religion | Religion, ReligiousPopulation, ConversionProgress | aucun |
| culture | Culture, CulturalPopulation, Assimilation | aucun |
| province | Province, Development, ProvinceModifier | aucun |
| decision | Decision, DecisionCooldown, DecisionEffect | aucun |
| cb | CasusBelli, CbFabrication, WarGoal | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1200-1203 | diplomacy | tick_treaties, process_diplomatic_action, update_opinion, check_treaty_expiration |
| 1210-1213 | economy | collect_taxes, pay_expenses, update_inflation, compute_economic_output |
| 1220-1223 | trade | tick_trade_routes, process_trade_income, apply_embargo, check_trade_disruption |
| 1230-1233 | military | tick_recruitment, compute_maintenance, apply_attrition, update_army_state |
| 1240-1243 | population | tick_population_growth, process_migration, update_class_distribution, check_population_events |
| 1250-1253 | religion | tick_conversion, update_religious_authority, process_religious_event, check_heresy |
| 1260-1263 | culture | tick_assimilation, update_cultural_influence, process_cultural_event, check_unrest |
| 1270-1273 | province | tick_development, update_province_control, apply_province_modifiers, check_province_events |
| 1280-1283 | decision | evaluate_decision_conditions, execute_decision, tick_cooldowns, check_decision_effects |
| 1290-1293 | cb | tick_fabrication, validate_casus_belli, check_cb_expiration, evaluate_legitimacy |

**Ordre d'execution** : diplomacy (1200) → economy (1210) → trade (1220) → military (1230) → population (1240) → religion (1250) → culture (1260) → province (1270) → decision (1280) → cb (1290).

**Justification** : la diplomatie est traitee d'abord car elle conditionne les relations. L'economie vient ensuite car le tresor influence tout. Le commerce depend de l'economie. Le militaire consomme le tresor. La population alimente le recrutement. Religion et culture modifient la population. Les provinces aggregent les effets. Les decisions lisent tout. Les casus belli sont evalues en dernier.

**Total** : 40 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| diplomacy | DiplomaticActionRequest | WarDeclared, PeaceSigned, AllianceFormed, AllianceBroken, TreatyExpired |
| economy | (aucun, ecriture directe) | TreasuryBankrupt, InflationCrisis, EconomicBoom, TaxCollected |
| trade | EstablishTradeRequest | TradeRouteEstablished, TradeRouteBroken, EmbargoImposed, EmbargoLifted |
| military | RecruitmentOrder | ArmyRaised, UnitRecruited, ArmyDisbanded, AttritionApplied |
| population | (aucun, tick auto) | PopulationGrowth, PopulationDecline, MigrationWave, Famine |
| religion | ConversionOrder | ConversionComplete, HeresySpread, ReligiousUprising, AuthorityChanged |
| culture | (aucun, tick auto) | AssimilationComplete, CulturalUnrest, TraditionAdopted, CulturalShift |
| province | (aucun, tick auto) | ProvinceConquered, ProvinceLiberated, DevelopmentComplete, ProvinceUnrest |
| decision | DecisionRequest | DecisionTaken, DecisionAvailable, DecisionEffectExpired, DecisionLocked |
| cb | FabricateClaimRequest | CbFabricated, CbExpired, CbDiscovered, WarJustified |

**Total** : 5 requests + 39 events = 44 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 10 crates | `mge-ecs`, `mge-event` |

### Dependances inter-pack

| Crate | Depend de |
|-------|-----------|
| military | Pack RPG (`mge-rpg-stats`, `mge-rpg-combat`) |
| diplomacy, cb | Pack Social (`mge-social-faction`) |
| diplomacy, trade | Pack Social (`mge-social-reputation`) |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-gs-trade` | `mge-gs-economy` |
| `mge-gs-military` | `mge-gs-economy`, `mge-gs-population` |
| `mge-gs-religion` | `mge-gs-population` |
| `mge-gs-culture` | `mge-gs-population` |
| `mge-gs-decision` | `mge-gs-economy`, `mge-gs-diplomacy` |
| `mge-gs-cb` | `mge-gs-diplomacy` |

### Dependances externes (aucune)

Le Pack Grand Strategy n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL (Game Composition Layer) configure les plugins Grand Strategy sans recompilation.

**Parametres exposables :**

- Duree des traites, seuils d'opinion
- Taux d'imposition, inflation maximale
- Valeur des routes commerciales, duree embargo
- Cout recrutement, taux d'attrition
- Taux de croissance demographique, capacite migratoire
- Vitesse de conversion, seuil d'heresie
- Vitesse d'assimilation, seuil d'unrest
- Vitesse de developpement, cout de fabrication CB

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates GS utilises | Usage |
|----------------|---------------------|-------|
| (aucun a ce jour) | — | — |

Le Pack Grand Strategy depend de :
- **Pack RPG** : stats individuelles des commandants, resolution combats
- **Pack Social** : factions, reputation inter-etats

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Utiliser operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Seed RNG** | population, religion, culture, cb utilisent le RNG kernel (mge-rng) |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | economy (taxes chaque tick), population (croissance), province (modifiers) |
| **Budget cible** | < 8ms pour 500 provinces et 100 factions a 1 tick/jour |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path (pre-allouer) |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de diplomatie multi-laterale | Traites bilateraux uniquement |
| Pas de systeme financier avance | Pas de prets, dettes, banques |
| Pas de commerce maritime avance | Routes terrestres + cotieres basiques |
| Pas de batailles tactiques integrees | Voir Pack Massive Battle |
| Pas de systeme legal | Hors scope v1 |
| Pas de succession automatique | Hors scope v1 |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| Diplomatie multi-laterale | Congres, traites collectifs, votes |
| Systeme financier | Prets, dettes, banques, faillite |
| Commerce maritime | Routes maritimes, blocus, piraterie |
| Succession | Lignees, heritage, crises de succession |
| Espionnage | Agents, missions, contre-espionnage |
| Ideologies | Mouvements politiques, revolutions |

---

## 17. Exemple d'assemblage

### Minimal (headless, diplomacy + economy)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeSocialFactionPlugin);
engine.add_plugin(MgeGsDiplomacyPlugin);
engine.add_plugin(MgeGsEconomyPlugin);
engine.build();
```

### Complet (grande strategie)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Pack RPG
engine.add_plugin(MgeRpgStatsPlugin);
engine.add_plugin(MgeRpgCombatPlugin);
// Pack Social
engine.add_plugin(MgeSocialFactionPlugin);
engine.add_plugin(MgeSocialReputationPlugin);
// Pack Grand Strategy
engine.add_plugin(MgeGsDiplomacyPlugin);
engine.add_plugin(MgeGsEconomyPlugin);
engine.add_plugin(MgeGsTradePlugin);
engine.add_plugin(MgeGsMilitaryPlugin);
engine.add_plugin(MgeGsPopulationPlugin);
engine.add_plugin(MgeGsReligionPlugin);
engine.add_plugin(MgeGsCulturePlugin);
engine.add_plugin(MgeGsProvincePlugin);
engine.add_plugin(MgeGsDecisionPlugin);
engine.add_plugin(MgeGsCbPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/grand-strategy/
├── mge-gs-diplomacy/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.gs.diplomacy.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-gs-economy/
│   └── (meme structure)
├── mge-gs-trade/
│   └── (meme structure)
├── mge-gs-military/
│   └── (meme structure)
├── mge-gs-population/
│   └── (meme structure)
├── mge-gs-religion/
│   └── (meme structure)
├── mge-gs-culture/
│   └── (meme structure)
├── mge-gs-province/
│   └── (meme structure)
├── mge-gs-decision/
│   └── (meme structure)
└── mge-gs-cb/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack Grand Strategy est la brique fondamentale des jeux de grande strategie dans MGE. Il :

- Fournit 10 plugins couvrant diplomatie, economie, commerce, militaire, population, religion, culture, provinces, decisions et casus belli.
- Reste generique : aucune logique specifique a un jeu ou a une epoque.
- S'execute en headless, en deterministe, sans rendu.
- Depend du Pack RPG pour les stats/combat et du Pack Social pour les factions/reputation.
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 10 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
