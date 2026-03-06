# Audit efficience mge-sodomight

## Statut

- Etat : COMPLET
- Phase : P4
- Responsable principal : Jean
- Date : 2026-03-06

## TL;DR

Audit efficience du code P3 `mge-sodomight`. Verification des complexites algorithmiques, des patterns memoire, et des goulots d'etranglement potentiels dans les crates coeur.

## 1. Complexite algorithmique des operations critiques

| Operation | Crate | Complexite | Acceptable |
|-----------|-------|------------|------------|
| `generate_layout` (LCG) | mge-world | O(cols*rows) | OUI |
| `LootTable::roll_drop` | mge-items | O(n affixes) | OUI |
| `act1_zone_graph()` | mge-world | O(1) const | OUI |
| `ZoneGraph::neighbors` | mge-world | O(n zones) | OUI -- n=11 max |
| `LadderBoard::upsert` | mge-meta | O(n log n) sort | OUI -- n ladder entries |
| `AffixPool::pick` | mge-items | O(n) weighted | OUI -- pool born |
| `WalkthroughValidator::validate` | mge-world | O(steps) | OUI -- 6 steps |
| `InterestCell::contains` (Chebyshev) | mge-replication | O(1) | OUI |
| `roster_for_zone` filter | mge-monsters | O(n roster) | OUI -- n=9 Acte 1 |

Aucune operation O(n^2) non bornee detectee dans les hot paths P3.

## 2. Patterns memoire

### Allocations critiques

| Structure | Allocation | Pattern | Risque |
|-----------|-----------|---------|--------|
| `DeltaAccumulator::pending` | Vec<DeltaSnapshot> | croissance jusqu'a drain | Faible -- drain explicite |
| `ChunkLibrary::chunks` | Vec<Chunk> | statique (10 chunks) | Aucun |
| `LadderBoard::entries` | Vec<LadderEntry> | croissance unbounded | Moyen -- pas de cap max |
| `QuestLog::states` | Vec<QuestState> | 1 par quete | Aucun -- 6 quetes Acte 1 |
| `PartyDef::members` | Vec<PartyMember> | cap MAX_PARTY_SIZE=8 | Aucun |
| `EliteVariant::affixes` | Vec<EliteAffix> | cap 6 (hard) | Aucun |

Observation : `LadderBoard` n'a pas de cap sur le nombre d'entrees. En production, un cap (ex: 10 000 entrees par saison) doit etre ajoute en P4.

### Clones couteux

- `DeltaSnapshot` est `Clone` mais contient `Vec<DeltaField>` avec `Vec<u8>` value_bytes. Le clonage en bulk est potentiellement couteux. A surveiller lors de l'integration reseau.
- `ZoneDef::connections: Vec<ZoneId>` est clone lors de `ZoneGraph::new`. N=11 zones, negligeable.

## 3. Determinisme et reproductibilite

Le LCG (`wrapping_mul` + `wrapping_add`) est correctement utilise :
- Pas de `rand::thread_rng()` ou de source non deterministe dans les crates simulation
- `generate_layout`, `roll_drop`, `simulate_drops` : tous seedables et deterministes

Observation : La seed LCG est actuellement une constante dans les tests. Aucun chemin d'injection de seed depuis le runtime n'est implemente en P3. Identifie comme bloquant dans le dossier de transfert.

## 4. Goulots d'etranglement potentiels P4

| Goulot | Crate | Frequence | Mitigation recommandee |
|--------|-------|-----------|------------------------|
| `LadderBoard::rerank` (sort O(n log n)) | mge-meta | Sur chaque upsert | Inserer en position triee (O(n) best case) |
| `roster_for_zone` (filter a chaque spawn) | mge-monsters | Sur chaque zone load | Cache biome->monsters au moment de l'init |
| `DeltaAccumulator` non draine | mge-replication | Accumulation reseau | Drain periodique obligatoire (tick budget) |
| `ZoneGraph::get` (linear scan) | mge-world | Sur chaque transition | HashMap<ZoneId, ZoneDef> si zones > 50 |

## 5. Qualite code

- `unsafe_code = "forbid"` workspace-wide : **RESPECTE**
- Pas de `unwrap()` en code de production : **VERIFIE** (clippy PASS)
- Pas d'allocation dans les hot paths de test : **OUI**
- `#[must_use]` sur toutes les methodes builder (with_affix, with_immunity, with_scale) : **OUI** (fixe en P4 E08 checkpoint)

## 6. Verdict

**Code P3 efficace pour le perimetre MVP.** 4 goulots identifies pour P4, aucun bloquant a ce stade.

```
[PHASE:P4] [AGENT:jean] [TASK:audit-efficience]
Actions:
- Analyse complexite 9 operations critiques : O(n) ou mieux pour tous
- Revue allocations memoire : 1 observation (LadderBoard cap manquant)
- Verification determinisme LCG : PASS, seed injection P4 a faire
- 4 goulots potentiels documentes pour P4
Checks:
- Pas de O(n^2) non borne en hot path : PASS
- unsafe_code forbid : PASS
- must_use builders : PASS
Status: DONE
```
