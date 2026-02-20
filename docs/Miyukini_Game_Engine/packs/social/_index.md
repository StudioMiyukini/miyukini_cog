# MGE — Pack Social Simulation

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/social/`  
**Nombre de crates** : 8  

---

## 1. Contexte

Le Pack Social Simulation gere les interactions sociales entre entites : relations interpersonnelles, factions, reputation, besoins physiologiques et sociaux, plannings quotidiens, traits de personnalite, propagation d'informations (rumeurs) et foyers domestiques. Il est autonome mais sert de dependance aux packs RTS (factions) et Grand Strategy (factions, reputation).

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Simulation sociale (The Sims, Dwarf Fortress), city builders, RPG sociaux, visual novels.
- **Hors portee** : Logique specifique a un jeu, rendu, audio, reseau.
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event).

---

## 3. Vision

Le Pack Social Simulation est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/social/
├── mge-social-relationship/    # Relations, affinite, historique
├── mge-social-faction/         # Factions, appartenance, hierarchie
├── mge-social-reputation/      # Score reputation, seuils
├── mge-social-need/            # Besoins faim/repos/social
├── mge-social-schedule/        # Planning, routines, activites
├── mge-social-personality/     # Traits, preferences
├── mge-social-gossip/          # Propagation info, rumeurs, memoire
└── mge-social-household/       # Foyers, cohabitation, ressources
```

### Graphe de dependances intra-pack

```
mge-social-gossip ──► mge-social-relationship
     │
     └────────────► mge-social-reputation

mge-social-schedule ──► mge-social-need

mge-social-household ──► mge-social-relationship
     │
     └────────────────► mge-social-need

mge-social-faction ──► mge-social-reputation
```

Crates feuilles (sans dependance intra-pack) : `mge-social-relationship`, `mge-social-reputation`, `mge-social-need`, `mge-social-personality`.

---

## 5. Sous-packs

Aucun. Les 8 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-social-relationship` | `mge.social.relationship.v1` | [mge-social-relationship.md](mge-social-relationship.md) | Relations entre entites, affinite, historique interactions |
| 2 | `mge-social-faction` | `mge.social.faction.v1` | [mge-social-faction.md](mge-social-faction.md) | Factions, appartenance, hierarchie, rangs |
| 3 | `mge-social-reputation` | `mge.social.reputation.v1` | [mge-social-reputation.md](mge-social-reputation.md) | Score reputation par faction/region, seuils |
| 4 | `mge-social-need` | `mge.social.need.v1` | [mge-social-need.md](mge-social-need.md) | Besoins physiologiques et sociaux (faim, repos, social) |
| 5 | `mge-social-schedule` | `mge.social.schedule.v1` | [mge-social-schedule.md](mge-social-schedule.md) | Planning quotidien, routines, activites planifiees |
| 6 | `mge-social-personality` | `mge.social.personality.v1` | [mge-social-personality.md](mge-social-personality.md) | Traits de personnalite, preferences, compatibilite |
| 7 | `mge-social-gossip` | `mge.social.gossip.v1` | [mge-social-gossip.md](mge-social-gossip.md) | Propagation d'informations, rumeurs, memoire sociale |
| 8 | `mge-social-household` | `mge.social.household.v1` | [mge-social-household.md](mge-social-household.md) | Foyers, cohabitation, ressources partagees |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|-------------------|------------------------------|
| relationship | Relationship, RelationshipHistory, SocialLink | aucun |
| faction | Faction, FactionMember, FactionRank, FactionRelation | aucun |
| reputation | Reputation, ReputationThresholds | aucun |
| need | NeedSet, NeedState | aucun |
| schedule | Schedule, ScheduleSlot, CurrentActivity | aucun |
| personality | PersonalityTraits, Preferences | aucun |
| gossip | GossipMemory, GossipItem, GossipSpreadState | aucun |
| household | Household, HouseholdMember, SharedResources | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1000-1003 | relationship | update_affinity, decay_relationships, process_interaction, check_relationship_threshold |
| 1010-1013 | faction | process_join_faction, update_faction_rank, process_leave_faction, update_faction_relations |
| 1020-1022 | reputation | update_reputation_score, check_reputation_thresholds, decay_reputation |
| 1030-1033 | need | tick_needs, satisfy_need, check_critical_need, evaluate_need_priority |
| 1040-1042 | schedule | advance_schedule, assign_activity, check_schedule_conflict |
| 1050-1052 | personality | compute_compatibility, apply_personality_modifier, evaluate_preference |
| 1060-1063 | gossip | spread_gossip, decay_gossip, memorize_event, evaluate_gossip_impact |
| 1070-1072 | household | update_shared_resources, check_household_capacity, process_household_change |

**Ordre d'execution** : relationship (1000) → faction (1010) → reputation (1020) → need (1030) → schedule (1040) → personality (1050) → gossip (1060) → household (1070).

**Justification** : les relations sont la base. Les factions s'appuient sur les relations. La reputation depend des factions. Les besoins sont independants mais lus par schedule. Le gossip lit relations et reputation. Le household combine relations et besoins.

**Total** : 27 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| relationship | InteractionRequest | RelationshipFormed, RelationshipBroken, AffinityChanged |
| faction | JoinFactionRequest, LeaveFactionRequest | FactionJoined, FactionLeft, RankChanged, FactionDissolved |
| reputation | (aucun, ecriture directe) | ReputationChanged, ThresholdCrossed |
| need | SatisfyNeedRequest | NeedCritical, NeedSatisfied, NeedChanged |
| schedule | AssignActivityRequest | ActivityStarted, ActivityCompleted, ScheduleConflict |
| personality | (aucun, lecture seule) | CompatibilityEvaluated |
| gossip | (aucun, lit events) | GossipSpread, GossipForgotten, RumorConfirmed |
| household | HouseholdChangeRequest | MemberJoined, MemberLeft, ResourcesDepleted |

**Total** : 5 requests + 21 events = 26 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 8 crates | `mge-ecs`, `mge-event` |

### Dependances inter-pack

Aucune. Le Pack Social est autonome.

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-social-faction` | `mge-social-reputation` |
| `mge-social-schedule` | `mge-social-need` |
| `mge-social-gossip` | `mge-social-relationship`, `mge-social-reputation` |
| `mge-social-household` | `mge-social-relationship`, `mge-social-need` |

### Dependances externes (aucune)

Le Pack Social n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL (Game Composition Layer) configure les plugins Social sans recompilation.

**Parametres exposables :**

- Taux de declin des relations, seuils d'affinite
- Vitesse de degradation des besoins
- Duree des activites, taille des plannings
- Rayon de propagation des rumeurs
- Capacite max des foyers

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates Social utilises | Usage |
|----------------|------------------------|-------|
| **RTS** | faction | Factions pour camps adverses |
| **Grand Strategy** | faction, reputation | Diplomatie, relations inter-factions |

Le Pack Social ne depend d'aucun autre pack genre.

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Utiliser operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Seed RNG** | Gossip et personality utilisent le RNG kernel (mge-rng) |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | need (tick chaque frame), relationship (updates frequents) |
| **Budget cible** | < 3ms pour 500 entites sociales a 30 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path (pre-allouer) |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de memoire long terme infinie | Limite fixe de souvenirs par entite |
| Pas de reseaux sociaux complexes | Graphe plat (paires), pas de cliques |
| Pas de heritage familial | Hors scope v1 |
| Pas de communication verbale | Le gossip est abstrait, pas de dialogue |
| Pas d'emotions composees | Traits simples, pas de modele OCC |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| Reseaux sociaux | Graphe de cliques, communautes |
| Heritage | Lignees familiales, heredite traits |
| Emotions composees | Modele OCC, emotions secondaires |
| Economie domestique | Budget, achats, epargne par foyer |
| Evenements sociaux | Fetes, mariages, funerailles |

---

## 17. Exemple d'assemblage

### Minimal (headless, relations + besoins)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeSocialRelationshipPlugin);
engine.add_plugin(MgeSocialNeedPlugin);
engine.build();
```

### Complet (simulation sociale)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeSocialRelationshipPlugin);
engine.add_plugin(MgeSocialFactionPlugin);
engine.add_plugin(MgeSocialReputationPlugin);
engine.add_plugin(MgeSocialNeedPlugin);
engine.add_plugin(MgeSocialSchedulePlugin);
engine.add_plugin(MgeSocialPersonalityPlugin);
engine.add_plugin(MgeSocialGossipPlugin);
engine.add_plugin(MgeSocialHouseholdPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/social/
├── mge-social-relationship/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.social.relationship.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-social-faction/
│   └── (meme structure)
├── mge-social-reputation/
│   └── (meme structure)
├── mge-social-need/
│   └── (meme structure)
├── mge-social-schedule/
│   └── (meme structure)
├── mge-social-personality/
│   └── (meme structure)
├── mge-social-gossip/
│   └── (meme structure)
└── mge-social-household/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack Social Simulation est la brique fondamentale des interactions sociales dans MGE. Il :

- Fournit 8 plugins couvrant relations, factions, reputation, besoins, schedules, personnalite, rumeurs et foyers.
- Reste generique : aucune logique specifique a un jeu.
- S'execute en headless, en deterministe, sans rendu.
- Sert de dependance pour RTS (factions) et Grand Strategy (factions, reputation).
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 8 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
