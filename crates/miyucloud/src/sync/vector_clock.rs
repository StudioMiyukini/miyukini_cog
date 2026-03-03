//! Horloges vectorielles pour la synchronisation P2P.
//!
//! @id: miyucloud_sync_vector_clock
//! @do: implement_vector_clock_compare_merge
//! @role: sync
//! @layer: infra
//!
//! Chaque modification d'un fichier incremente le compteur du noeud local.
//! La comparaison de deux horloges permet de determiner l'ordre causal
//! (Before, After, Equal, Concurrent). Les horloges concurrentes signalent
//! un conflit qui doit etre resolu par l'utilisateur.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Horloge vectorielle : `HashMap<NodeId, compteur_logique>`.
///
/// Chaque noeud maintient un compteur logique qui est incremente a chaque
/// modification locale. La comparaison de deux horloges permet de determiner
/// l'ordonnancement causal des evenements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorClock {
    /// Compteurs logiques par identifiant de noeud (COG ID).
    pub clocks: HashMap<String, u64>,
}

/// Resultat de la comparaison de deux horloges vectorielles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockOrder {
    /// `self` est strictement avant `other` (other est plus recent).
    Before,
    /// `self` est strictement apres `other` (self est plus recent).
    After,
    /// Les deux horloges sont identiques.
    Equal,
    /// Ni avant ni apres : les modifications sont concurrentes (CONFLIT).
    Concurrent,
}

impl VectorClock {
    /// Cree une horloge vectorielle vide.
    pub fn new() -> Self {
        Self {
            clocks: HashMap::new(),
        }
    }

    /// Cree une horloge vectorielle a partir d'entrees existantes.
    pub fn from_entries(entries: Vec<(String, u64)>) -> Self {
        Self {
            clocks: entries.into_iter().collect(),
        }
    }

    /// Incremente le compteur du noeud local.
    pub fn increment(&mut self, node_id: &str) {
        let counter = self.clocks.entry(node_id.to_string()).or_insert(0);
        *counter += 1;
    }

    /// Retourne le compteur pour un noeud donne.
    pub fn get(&self, node_id: &str) -> u64 {
        self.clocks.get(node_id).copied().unwrap_or(0)
    }

    /// Compare cette horloge avec une autre. Retourne l'ordre causal.
    ///
    /// - `Equal` : les deux horloges sont identiques.
    /// - `Before` : `self` < `other` (toutes les composantes de self <= other,
    ///   avec au moins une strictement inferieure).
    /// - `After` : `self` > `other` (toutes les composantes de self >= other,
    ///   avec au moins une strictement superieure).
    /// - `Concurrent` : ni avant ni apres (certaines composantes sont plus grandes
    ///   dans self, d'autres dans other).
    pub fn compare(&self, other: &VectorClock) -> ClockOrder {
        let mut has_less = false;
        let mut has_greater = false;

        // Collect all unique node IDs from both clocks
        let all_nodes: std::collections::HashSet<&str> = self
            .clocks
            .keys()
            .chain(other.clocks.keys())
            .map(String::as_str)
            .collect();

        for node_id in all_nodes {
            let self_val = self.get(node_id);
            let other_val = other.get(node_id);

            if self_val < other_val {
                has_less = true;
            }
            if self_val > other_val {
                has_greater = true;
            }

            // Short-circuit: if both flags are set, it is concurrent
            if has_less && has_greater {
                return ClockOrder::Concurrent;
            }
        }

        match (has_less, has_greater) {
            (false, false) => ClockOrder::Equal,
            (true, false) => ClockOrder::Before,
            (false, true) => ClockOrder::After,
            (true, true) => ClockOrder::Concurrent, // unreachable due to short-circuit
        }
    }

    /// Fusionne avec une horloge distante (max par composante).
    ///
    /// Apres la fusion, chaque composante de `self` est le maximum
    /// entre la valeur locale et la valeur distante.
    pub fn merge(&mut self, other: &VectorClock) {
        for (node_id, &other_val) in &other.clocks {
            let self_val = self.clocks.entry(node_id.clone()).or_insert(0);
            if other_val > *self_val {
                *self_val = other_val;
            }
        }
    }

    /// Retourne `true` si l'horloge est vide.
    pub fn is_empty(&self) -> bool {
        self.clocks.is_empty() || self.clocks.values().all(|&v| v == 0)
    }
}

impl Default for VectorClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clock_is_empty() {
        let clock = VectorClock::new();
        assert!(clock.is_empty());
        assert!(clock.clocks.is_empty());
    }

    #[test]
    fn test_increment_increases_counter() {
        let mut clock = VectorClock::new();
        assert_eq!(clock.get("node-a"), 0);
        clock.increment("node-a");
        assert_eq!(clock.get("node-a"), 1);
        clock.increment("node-a");
        assert_eq!(clock.get("node-a"), 2);
        clock.increment("node-b");
        assert_eq!(clock.get("node-b"), 1);
        assert_eq!(clock.get("node-a"), 2);
    }

    #[test]
    fn test_compare_equal() {
        let mut a = VectorClock::new();
        a.increment("node-a");
        a.increment("node-b");

        let mut b = VectorClock::new();
        b.increment("node-a");
        b.increment("node-b");

        assert_eq!(a.compare(&b), ClockOrder::Equal);
    }

    #[test]
    fn test_compare_equal_empty() {
        let a = VectorClock::new();
        let b = VectorClock::new();
        assert_eq!(a.compare(&b), ClockOrder::Equal);
    }

    #[test]
    fn test_compare_before() {
        // a = {node-a: 1}, b = {node-a: 2} -> a < b
        let a = VectorClock::from_entries(vec![("node-a".into(), 1)]);
        let b = VectorClock::from_entries(vec![("node-a".into(), 2)]);
        assert_eq!(a.compare(&b), ClockOrder::Before);
    }

    #[test]
    fn test_compare_before_subset() {
        // a = {node-a: 1}, b = {node-a: 1, node-b: 1} -> a < b
        let a = VectorClock::from_entries(vec![("node-a".into(), 1)]);
        let b = VectorClock::from_entries(vec![
            ("node-a".into(), 1),
            ("node-b".into(), 1),
        ]);
        assert_eq!(a.compare(&b), ClockOrder::Before);
    }

    #[test]
    fn test_compare_after() {
        // a = {node-a: 3}, b = {node-a: 1} -> a > b
        let a = VectorClock::from_entries(vec![("node-a".into(), 3)]);
        let b = VectorClock::from_entries(vec![("node-a".into(), 1)]);
        assert_eq!(a.compare(&b), ClockOrder::After);
    }

    #[test]
    fn test_compare_after_superset() {
        // a = {node-a: 1, node-b: 2}, b = {node-a: 1} -> a > b
        let a = VectorClock::from_entries(vec![
            ("node-a".into(), 1),
            ("node-b".into(), 2),
        ]);
        let b = VectorClock::from_entries(vec![("node-a".into(), 1)]);
        assert_eq!(a.compare(&b), ClockOrder::After);
    }

    #[test]
    fn test_compare_concurrent() {
        // a = {node-a: 2, node-b: 1}, b = {node-a: 1, node-b: 2} -> concurrent
        let a = VectorClock::from_entries(vec![
            ("node-a".into(), 2),
            ("node-b".into(), 1),
        ]);
        let b = VectorClock::from_entries(vec![
            ("node-a".into(), 1),
            ("node-b".into(), 2),
        ]);
        assert_eq!(a.compare(&b), ClockOrder::Concurrent);
    }

    #[test]
    fn test_compare_concurrent_disjoint() {
        // a = {node-a: 1}, b = {node-b: 1} -> concurrent
        let a = VectorClock::from_entries(vec![("node-a".into(), 1)]);
        let b = VectorClock::from_entries(vec![("node-b".into(), 1)]);
        assert_eq!(a.compare(&b), ClockOrder::Concurrent);
    }

    #[test]
    fn test_merge_takes_max() {
        let mut a = VectorClock::from_entries(vec![
            ("node-a".into(), 3),
            ("node-b".into(), 1),
        ]);
        let b = VectorClock::from_entries(vec![
            ("node-a".into(), 1),
            ("node-b".into(), 5),
            ("node-c".into(), 2),
        ]);
        a.merge(&b);
        assert_eq!(a.get("node-a"), 3);
        assert_eq!(a.get("node-b"), 5);
        assert_eq!(a.get("node-c"), 2);
    }

    #[test]
    fn test_merge_symmetric() {
        // merge(A, B) gives the same result as merge(B, A)
        let a = VectorClock::from_entries(vec![
            ("node-a".into(), 2),
            ("node-b".into(), 1),
        ]);
        let b = VectorClock::from_entries(vec![
            ("node-a".into(), 1),
            ("node-b".into(), 3),
        ]);

        let mut a_merged = a.clone();
        a_merged.merge(&b);

        let mut b_merged = b.clone();
        b_merged.merge(&a);

        assert_eq!(a_merged, b_merged);
    }

    #[test]
    fn test_merge_with_empty() {
        let mut a = VectorClock::from_entries(vec![("node-a".into(), 2)]);
        let b = VectorClock::new();
        a.merge(&b);
        assert_eq!(a.get("node-a"), 2);
    }

    #[test]
    fn test_from_entries() {
        let clock = VectorClock::from_entries(vec![
            ("node-a".into(), 5),
            ("node-b".into(), 3),
        ]);
        assert_eq!(clock.get("node-a"), 5);
        assert_eq!(clock.get("node-b"), 3);
        assert!(!clock.is_empty());
    }

    #[test]
    fn test_serialize_roundtrip() {
        let clock = VectorClock::from_entries(vec![
            ("node-a".into(), 5),
            ("node-b".into(), 3),
        ]);
        let json = serde_json::to_string(&clock).unwrap();
        let restored: VectorClock = serde_json::from_str(&json).unwrap();
        assert_eq!(clock, restored);
    }
}
