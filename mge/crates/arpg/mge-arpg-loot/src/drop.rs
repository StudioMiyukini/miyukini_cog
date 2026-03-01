//! Loot generation: rolling drops from treasure classes.
//!
//! [`LootGenerator`] resolves [`TreasureClass`] entries (including nested TCs)
//! and produces [`DropRoll`] results.

use rand::Rng;

use crate::quality::{DropQuality, QualityRoller};
use crate::registry::TreasureClassRegistry;

/// Maximum recursion depth when resolving nested treasure classes.
const MAX_TC_DEPTH: u32 = 8;

/// A single resolved drop result.
#[derive(Debug, Clone)]
pub struct DropRoll {
    /// The item identifier that was dropped.
    pub item_id: String,
    /// The quality tier rolled via Magic Find.
    pub quality: DropQuality,
    /// How many of this item dropped (relevant for gold and stackables).
    pub quantity: u32,
}

/// Stateless loot generator.
///
/// All state (RNG, registry, parameters) is passed in per call so that the
/// generator itself carries no mutable state.
pub struct LootGenerator;

impl LootGenerator {
    /// Generates drops from the treasure class identified by `tc_id`.
    ///
    /// The generator makes `picks` independent rolls on the TC table, resolves
    /// nested TCs recursively (up to [`MAX_TC_DEPTH`] levels), and returns
    /// the final list of [`DropRoll`]s.
    ///
    /// If `tc_id` is not found in the registry the function returns an empty
    /// vector (no panic).
    pub fn generate(
        tc_id: &str,
        registry: &TreasureClassRegistry,
        ilvl: u32,
        magic_find: u32,
        rng: &mut impl Rng,
    ) -> Vec<DropRoll> {
        let mut results = Vec::new();
        Self::resolve(tc_id, registry, ilvl, magic_find, rng, 0, &mut results);
        results
    }

    /// Generates a direct gold drop without going through a treasure class.
    pub fn gold_drop(min: u32, max: u32, rng: &mut impl Rng) -> DropRoll {
        let qty = if min >= max {
            min
        } else {
            rng.gen_range(min..=max)
        };
        DropRoll {
            item_id: "gold".to_string(),
            quality: DropQuality::Normal,
            quantity: qty,
        }
    }

    /// Internal recursive resolver.
    fn resolve(
        tc_id: &str,
        registry: &TreasureClassRegistry,
        ilvl: u32,
        magic_find: u32,
        rng: &mut impl Rng,
        depth: u32,
        results: &mut Vec<DropRoll>,
    ) {
        if depth >= MAX_TC_DEPTH {
            // Silently stop -- prevents stack overflow on cyclic references.
            return;
        }

        let Some(tc) = registry.get(tc_id) else {
            return; // Unknown TC -- return empty, no panic.
        };

        // Clone the data we need so we don't hold a borrow on the registry
        // across the mutable rng calls.
        let picks = tc.picks;
        let entries = tc.entries.clone();
        let total = tc.total_weight();

        if total == 0 || entries.is_empty() {
            return;
        }

        for _ in 0..picks {
            let roll = rng.gen_range(0..total);

            // Walk through entries accumulating weight. If roll falls past all
            // entries, it landed on NoDrop.
            let mut cumulative: u32 = 0;
            for entry in &entries {
                cumulative = cumulative.saturating_add(entry.weight);
                if roll < cumulative {
                    if entry.is_treasure_class {
                        // Recurse into nested TC.
                        Self::resolve(
                            &entry.item_id,
                            registry,
                            ilvl,
                            magic_find,
                            rng,
                            depth + 1,
                            results,
                        );
                    } else {
                        let qty = if entry.min_qty >= entry.max_qty {
                            entry.min_qty
                        } else {
                            rng.gen_range(entry.min_qty..=entry.max_qty)
                        };
                        let quality = QualityRoller::roll(ilvl, magic_find, rng);
                        results.push(DropRoll {
                            item_id: entry.item_id.clone(),
                            quality,
                            quantity: qty,
                        });
                    }
                    break;
                }
            }
            // If we exhausted all entries without breaking, the roll landed on
            // NoDrop -- nothing added.
        }
    }
}
