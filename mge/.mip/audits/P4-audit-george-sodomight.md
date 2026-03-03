# Rapport d'Audit P4 -- Sodomight MVP (Sprints 1-3)

**Auditeur** : George (Audit Expert Analyste)
**Date** : 2026-03-03
**Branch** : `feat/sodomight-mvp`
**Scope** : 34 crates workspace MGE, focus Sprints 1-3

---

## Resume executif

**Score global : 91/100**

Le workspace MGE Sodomight MVP est dans un etat solide. 1194 tests passent, 0 echecs, 0 clippy warnings. L'architecture 4 couches (Kernel / Engine / ARPG Pack / Game) est respectee. Les patterns Rust sont bien appliques (types d'erreur explicites, pas d'unsafe, saturating arithmetic). Cinq fichiers manquent leurs annotations MSCM, et un `unwrap()` subsiste dans du code de production (sodomight-client). La securite est bien geree (SEC-16 a SEC-22 identifies et implementes). Le code est livrable avec des reserves mineures.

---

## 1. Conformite fonctionnelle (24/25)

### Build et Tests

| Verification | Resultat |
|---|---|
| `cargo build --workspace` | OK (0.54s) |
| `cargo test --workspace` | **1194 tests, 0 echecs, 0 ignores** |
| `cargo clippy --workspace -- -D warnings` | **0 warnings** |
| Test suites | 74 suites executees, 38 avec tests actifs |

### Annotations MSCM

- [x] `games/sodomight/src/boss.rs` -- `@id: Sodomight-Boss @do: boss-andariel @role: back-end @layer: 4`
- [x] `games/sodomight/src/content.rs` -- `@id: Sodomight-Content @do: act1-content-data @role: back-end @layer: 4`
- [x] `games/sodomight/src/world.rs` -- `@id: Sodomight-World @do: gameplay-world @role: back-end @layer: 4`
- [x] `games/sodomight/src/lib.rs` -- `@id: Sodomight-Game @do: game-core @role: back-end @layer: 4`
- [x] `games/sodomight/src/state.rs` -- `@id: Sodomight-Game-State @do: game-state @role: back-end @layer: 4`
- [x] `games/sodomight/src/error.rs` -- `@id: Sodomight-Game-Error @do: game-error @role: back-end @layer: 4`
- [x] `games/sodomight/src/config.rs` -- `@id: Sodomight-Game-Config @do: game-config @role: back-end @layer: 4`
- [x] `games/sodomight/src/data_loader.rs` -- `@id: Sodomight-DataLoader @do: data-loading @role: back-end @layer: 4`
- [x] `crates/arpg/mge-arpg-quest/src/act1_quests.rs` -- present
- [x] `crates/arpg/mge-arpg-quest/src/npc.rs` -- present
- [x] `crates/arpg/mge-arpg-quest/src/journal.rs` -- present
- [x] `crates/arpg/mge-arpg-quest/src/def.rs` -- present
- [x] `crates/arpg/mge-arpg-items/src/runewords.rs` -- present
- [x] `crates/arpg/mge-arpg-trade/src/vendor.rs` -- present
- [x] `crates/arpg/mge-arpg-trade/src/wallet.rs` -- present
- [x] `crates/engine/mge-collision-rich/src/capsule.rs` -- present
- [x] `crates/engine/mge-collision-rich/src/circle.rs` -- present
- [x] `crates/engine/mge-collision-rich/src/intersect.rs` -- present
- [x] `crates/engine/mge-net/src/codec.rs` -- present
- [x] `crates/engine/mge-script/src/engine.rs` -- present
- [x] `crates/engine/mge-save/src/characters.rs` -- present
- [x] `crates/engine/mge-save/src/db.rs` -- present
- [x] `crates/engine/mge-ui/src/inventory.rs` -- present
- [ ] `crates/engine/mge-ui/src/menus/main_menu.rs` -- **MANQUANTE**
- [ ] `crates/engine/mge-ui/src/menus/char_select.rs` -- **MANQUANTE**
- [ ] `crates/engine/mge-audio/src/bgm.rs` -- **MANQUANTE**
- [ ] `crates/engine/mge-audio/src/sfx.rs` -- **MANQUANTE**
- [ ] `crates/engine/mge-audio/src/manager.rs` -- **MANQUANTE**

**Couverture MSCM** : 23/28 fichiers audites (82%). 5 fichiers manquent les annotations `@id/@do/@role/@layer`.

**Total annotations MSCM workspace** : 265 occurrences dans 248 fichiers (bon ratio, 1:1).

### Ecarts fonctionnels

- `draw_char_select()` contient un placeholder `"(A implementer -- liste des personnages)"` -- le char select est un stub UI sans listing reel des personnages. **Accepte pour un MVP.**

---

## 2. Qualite structurelle (19/20)

### Architecture 4 couches

| Couche | Crates | Conformite |
|---|---|---|
| Kernel (L0-L1) | mge-core, mge-ecs, mge-math, mge-asset, mge-platform | OK |
| Engine (L2) | mge-render, mge-audio, mge-ui, mge-pathfinding, mge-collision, mge-collision-rich, mge-script, mge-net, mge-save | OK |
| ARPG Pack (L3) | mge-arpg-world, mge-arpg-entity, mge-arpg-combat, mge-arpg-items, mge-arpg-stats, mge-arpg-skills, mge-arpg-loot, mge-arpg-ai, mge-arpg-quest, mge-arpg-trade | OK |
| Game (L4) | sodomight, sodomight-server, sodomight-client | OK |

- [x] Pas de dependances circulaires (verifiable par compilation reussie)
- [x] `unsafe_code = "forbid"` dans le workspace `Cargo.toml`
- [x] `#![deny(unsafe_code)]` ajoute explicitement dans 12 crates lib.rs
- [x] Clippy pedantic active (`all = "warn"`, `pedantic = "warn"`)
- [x] Types d'erreur explicites par module (thiserror utilise partout : `GameError`, `WalletError`, `TradeError`, `QuestError`, `InventoryError`, `DataLoadError`, `PersistenceError`, `ScriptError`, `NetError`)
- [x] Pas d'URL hardcodees dans le code source (0 occurrences `https?://` dans crates/)

### Point de deduction

Le workspace contient 34 crates + 7 outils, ce qui est un volume important pour un MVP. Cependant, chaque crate a un perimetre clair et une responsabilite unique, ce qui est conforme au principe de separation des preoccupations.

---

## 3. Tests (18/20)

### Metriques

- **Total** : 1194 tests
- **Echecs** : 0
- **Ignores** : 0
- **Temps d'execution** : < 1 seconde pour l'ensemble (hors doc-tests)

### Qualite des tests par fichier audite

| Fichier | Tests | Cas limites | Assertions | Qualite |
|---|---|---|---|---|
| `boss.rs` | 9 | Rage phase, death, resistance | Valeurs exactes D2 | Excellente |
| `act1_quests.rs` | 8 | Objectives, rewards, act number | Patterns matching | Excellente |
| `npc.rs` | 7 | Out-of-bounds index, invalid node | None handling | Excellente |
| `runewords.rs` | 9 | Wrong order, wrong base, partial, round-trip | Exhaustive | Excellente |
| `vendor.rs` | 10 | Insufficient gold, out of stock, SEC-21, fallback | Error variants | Excellente |
| `wallet.rs` | 11 | Overflow, underflow, SEC-19 (negative amounts) | Boundary values | Excellente |
| `capsule.rs` | 8 | Degenerate segment, AABB overlap/miss, radius reach | Epsilon comparisons | Excellente |
| `circle.rs` | 8 | Zero radius, touching boundary, bounding AABB | Edge cases | Excellente |
| `intersect.rs` | 9 | Circle-circle, circle-capsule, capsule-AABB, OBB fallback | Dispatch matrix | Excellente |
| `characters.rs` | 9 | Name validation SEC-17, roundtrip, level bounds | Injection prevention | Excellente |
| `codec.rs` | Tests dans `tests.rs` | CRC32, message too large | Protocol integrity | Bonne |
| `engine.rs` (script) | Tests dans `tests.rs` | Security limits, API registration | Rhai sandbox | Bonne |
| `inventory.rs` | 6 | Overlap, out-of-bounds, swap, stash | Grid logic | Bonne |

### Points forts

- Les tests couvrent systematiquement les cas d'erreur, pas uniquement le happy path
- Les boundary values sont testees (GOLD_MAX, resist clamping, zero-length segments)
- Les tests de securite (SEC-17 a SEC-22) sont integres directement aux modules concernes

### Point de deduction

- Le journal (`journal.rs`) n'a pas de tests inline (les tests sont dans le fichier `tests.rs` separe) -- conforme mais la tracabilite est moins evidente
- Pas de tests de performance (benchmarks) -- acceptable pour un MVP

---

## 4. Securite (18/20)

### Mesures de securite identifiees et verifiees

| Reference | Description | Fichier | Statut |
|---|---|---|---|
| SEC-16 | Taille max message reseau 64 KiB | `codec.rs:9` | OK |
| SEC-17 | Validation nom personnage (HTML injection, longueur, caracteres) | `characters.rs:230-264` | OK |
| SEC-18 | Cap item reward qty (100) et XP reward (100,000) dans scripts | `engine.rs:256-268` | OK |
| SEC-19 | Reject negative amounts dans Wallet | `wallet.rs:58-88` | OK |
| SEC-20 | CRC32 checksum sur frames reseau | `codec.rs:89-175` | OK |
| SEC-21 | Prix vente depuis definition serveur, pas client | `vendor.rs:117-145` | OK |
| SEC-22 | Backup base de donnees avant session | `db.rs:71-78` | OK |
| SEC-09 | Hash verification scripts Rhai (FNV-1a) | `engine.rs:304-333` | OK |
| -- | `eval` desactive dans Rhai | `engine.rs:36` | OK |
| -- | Limites Rhai : 50K ops, 32 call levels, 4K string | `engine.rs:28-33` | OK |
| -- | `unsafe_code = "forbid"` global | `Cargo.toml:60` | OK |

### Points de vigilance

- **decode_verify_crc** reutilise `NetError::MessageTooLarge` pour les erreurs CRC32 (semantique inexacte). Non bloquant mais pourrait induire en erreur lors du debugging. **Severite : Mineure.**
- **FNV-1a vs SHA-256** : Le commentaire doc de `verify_script_hash` mentionne "SHA-256" mais l'implementation utilise FNV-1a 64-bit. FNV-1a n'est PAS un hash cryptographique -- un attaquant pourrait forger un script avec le meme hash. Pour un MVP listen-server (fichiers locaux), c'est acceptable. Pour du multiplayer avec scripts distants, il faudra migrer vers SHA-256 ou BLAKE3. **Severite : Mineure (MVP), Majeure (production multiplayer).**

### Donnees sensibles

- [x] Pas de mots de passe hardcodes
- [x] Pas de cles API en clair
- [x] Argon2 en dependance pour le hashing de mots de passe (workspace)
- [x] Pas d'URL externe en dur (LOI-1 respectee)

---

## 5. Patterns Rust / Maintenabilite (12/15)

### unwrap() en production

| Fichier | Ligne | Contexte | Severite |
|---|---|---|---|
| `sodomight-client/src/game.rs` | 194 | `msg[start.unwrap()..i]` dans un parser de nombres | Mineure |

**Analyse** : Ce `unwrap()` est protege par la condition `if start.is_some()` sur la ligne precedente (191), donc il ne peut pas paniquer a l'execution. Le `#[allow(clippy::unwrap_used)]` est documente avec un commentaire justificatif. **Non bloquant mais recommande de remplacer par un `if let` pour la clarte.**

Tous les autres `unwrap()` detectes (environ 100+ occurrences) sont exclusivement dans des blocs `#[cfg(test)]`. **Conforme.**

### Types d'erreur

- [x] `thiserror` utilise systematiquement
- [x] Variants d'erreur descriptifs avec contexte (item_id, have/need, etc.)
- [x] `Result` type alias definis (`PersistResult`, `ScriptResult`)

### serde(default) sur nouveaux champs

- [x] `NpcVendor::refresh_timer` utilise `#[serde(default)]` -- CONFORME

### Patterns positifs observes

- **Saturating arithmetic** systematique (`saturating_sub`, `saturating_add`) dans wallet, boss, cooldowns
- **`#[must_use]`** sur toutes les fonctions factory pures (runewords, quests, NPCs, boss)
- **`f32` epsilon comparisons** dans les tests (pas de `==` direct sur floats)
- **Builder pattern** sur `VendorItem::with_stock()`
- **Clone-on-read** avec `clone_from()` pour eviter les allocations (npc.rs:109)

### Points de deduction

- **5 fichiers sans annotations MSCM** (main_menu.rs, char_select.rs, bgm.rs, sfx.rs, manager.rs) -- Ecart MSCM
- **char_select.rs** : stub UI avec placeholder text -- acceptable MVP mais devrait etre documente dans un FIXME/TODO
- **content.rs** : `#![allow(clippy::too_many_lines)]` sur un fichier de 800+ lignes de donnees hardcodees. Justifie pour le MVP, mais la migration TOML devrait etre prioritaire

---

## Anomalies

| # | Severite | Description | Fichier | Recommandation |
|---|---|---|---|---|
| 1 | Mineure | 5 fichiers sans annotations MSCM `@id/@do/@role/@layer` | `main_menu.rs`, `char_select.rs`, `bgm.rs`, `sfx.rs`, `manager.rs` | Ajouter les annotations en en-tete |
| 2 | Mineure | `decode_verify_crc` reutilise `NetError::MessageTooLarge` pour les erreurs CRC -- semantique incorrecte | `codec.rs:168` | Ajouter un variant `ChecksumMismatch` a `NetError` |
| 3 | Mineure | Doc de `verify_script_hash` mentionne SHA-256 mais l'implementation utilise FNV-1a 64-bit (non-cryptographique) | `engine.rs:303` | Corriger la doc ou migrer vers un hash cryptographique |
| 4 | Info | `unwrap()` protege par guard condition dans le parser de nombres | `game.rs:194` | Remplacer par `if let Some(s) = start` pour la clarte |
| 5 | Info | Character select est un stub UI | `char_select.rs:141` | Documenter dans un TODO avec le ticket de tracking |

---

## Optimisations recommandees

| # | Impact | Description | Effort |
|---|---|---|---|
| 1 | Eleve | Migrer les donnees hardcodees de `content.rs` (800+ lignes) vers les fichiers TOML existants dans `data/` | Moyen |
| 2 | Moyen | Implementer le listing reel des personnages dans char_select.rs | Moyen |
| 3 | Moyen | Ajouter un variant `ChecksumMismatch` a `NetError` pour la verification CRC | Faible |
| 4 | Faible | Remplacer `SfxPool::active` Vec par VecDeque pour O(1) eviction FIFO | Faible |
| 5 | Faible | Ajouter des benchmarks `#[bench]` ou `criterion` pour collision et combat | Moyen |

---

## Checklist d'audit standardisee MIP v2

- [x] `cargo build --workspace` OK
- [x] `cargo test --workspace` OK (1194 tests, 0 echecs)
- [x] `cargo clippy --workspace -- -D warnings` propre
- [x] Pas d'`unwrap()` en production hors test (1 occurrence protegee par guard, `#[allow]` documente)
- [x] Pas d'URL hardcodees
- [x] Pas de donnees sensibles en clair
- [x] Annotations MSCM presentes sur 82% des fichiers audites (5 manquantes)
- [x] Lois d'Autonomie respectees (LOI-1 a LOI-9)
- [x] `unsafe_code = "forbid"` dans le workspace Cargo.toml
- [ ] Parcours utilisateur UI : char_select est un stub (acceptable MVP)

---

## Score detaille

| Critere | Score | Max | Justification |
|---|---|---|---|
| Conformite fonctionnelle | 24 | 25 | 5 fichiers sans MSCM |
| Qualite structurelle | 19 | 20 | Volume de crates eleve mais bien structure |
| Tests | 18 | 20 | 1194 tests solides, pas de benchmarks |
| Securite | 18 | 20 | FNV-1a non-crypto, semantique CRC error |
| Maintenabilite | 12 | 15 | 5 MSCM manquantes, stub UI, content.rs monolithique |
| **TOTAL** | **91** | **100** | |

---

## Conclusion

**Verdict : CONFORME AVEC RESERVES**

Le workspace MGE Sodomight MVP est livrable en l'etat pour validation P5. Les 5 anomalies identifiees sont toutes de severite Mineure ou Info -- aucun defaut BLOQUANT n'a ete detecte.

**Reserves** :
1. Les 5 fichiers sans annotations MSCM doivent etre completes avant le merge final
2. La documentation de `verify_script_hash` (SHA-256 vs FNV-1a) doit etre corrigee
3. Le variant `NetError::ChecksumMismatch` devrait etre ajoute pour la clarte semantique

**Points forts remarquables** :
- Architecture 4 couches propre avec 34 crates bien delimites
- 1194 tests avec 0 echecs -- couverture TDD exemplaire
- Securite integree des la conception (SEC-16 a SEC-22, Rhai sandbox)
- Patterns Rust idiomatiques (thiserror, saturating arithmetic, `#[must_use]`)
- Donnees de jeu fideles au canon Diablo 2 (resistances, quetes, bosses, runewords)

**Gate P4** : PASSE -- 0 defaut BLOQUANT.
Le livrable peut passer en P5 (test humain).

---

*George -- Audit Expert Analyste, Miyukini AI Studio*
*ISO 19011:2018 / CISA / RGPD*
