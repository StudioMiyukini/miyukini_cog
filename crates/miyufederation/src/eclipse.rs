//! @id     miyufederation_eclipse
//! @do     choisir_pairs_diversifies
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « eclipse » (carnet de pairs : diversité des réseaux, pairs anciens conservés, EX-37).

//! Carnet de pairs anti-éclipse (EX-37, zones §16 « pairs choisis dans des
//! réseaux différents, pairs anciens conservés » ; repris de l'`addrman` de
//! Bitcoin).
//!
//! - Un pair garde son **premier** contact et son réseau d'origine (un pair ne
//!   rajeunit pas et ne change pas de réseau pour contourner la diversité)
//! - Choix : tour de rôle entre réseaux, du plus ancien au plus récent dans
//!   chaque réseau ; seuls les pairs éligibles (invité ou membre, ni révoqué ni
//!   banni) sont retenus
//! - « Réseau » = étiquette fournie par la couche transport (préfixe, AS…) :
//!   sa définition n'est pas fixée par la doc gelée (à amender)
//!
//! Le second volet anti-éclipse (têtes contresignées par ≥ 3 membres) est
//! dans [`crate::Federation::recevoir_tete`].

use std::collections::BTreeMap;

use miyauth::CogId;

use crate::federation::Federation;

/// Longueur maximale d'une étiquette de réseau.
pub const TAILLE_MAX_RESEAU: usize = 64;

#[derive(Debug, Clone)]
struct Pair {
    reseau: String,
    premier_contact: u64,
}

/// Carnet des pairs connus.
#[derive(Debug, Clone, Default)]
pub struct CarnetPairs {
    pairs: BTreeMap<CogId, Pair>,
}

impl CarnetPairs {
    /// Carnet vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Note un contact avec `cog` depuis `reseau`. Le premier contact et le
    /// premier réseau sont conservés. Étiquette vide ou trop longue : ignorée.
    pub fn noter_contact(&mut self, cog: &CogId, reseau: &str, maintenant: u64) -> bool {
        if reseau.is_empty() || reseau.len() > TAILLE_MAX_RESEAU || reseau.chars().any(char::is_control) {
            return false;
        }
        self.pairs
            .entry(cog.clone())
            .or_insert_with(|| Pair { reseau: reseau.to_string(), premier_contact: maintenant });
        true
    }

    /// Réseau d'un pair.
    pub fn reseau(&self, cog: &CogId) -> Option<&str> {
        self.pairs.get(cog).map(|p| p.reseau.as_str())
    }

    /// Choisit au plus `n` pairs éligibles, réseaux alternés, anciens d'abord.
    pub fn choisir(&self, n: usize, federation: &Federation, maintenant: u64) -> Vec<CogId> {
        let mut par_reseau: BTreeMap<&str, Vec<(u64, &CogId)>> = BTreeMap::new();
        for (cog, p) in &self.pairs {
            if federation.pair_eligible(cog, maintenant) {
                par_reseau.entry(p.reseau.as_str()).or_default().push((p.premier_contact, cog));
            }
        }
        let mut files: Vec<Vec<(u64, &CogId)>> = par_reseau.into_values().collect();
        for f in &mut files {
            f.sort_unstable();
            f.reverse(); // le plus ancien en fin de file (`pop`)
        }
        // Réseaux ordonnés par leur pair le plus ancien.
        files.sort_by_key(|f| f.last().map(|(t, _)| *t));
        let mut choix = Vec::with_capacity(n);
        while choix.len() < n && files.iter().any(|f| !f.is_empty()) {
            for f in &mut files {
                if choix.len() >= n {
                    break;
                }
                if let Some((_, cog)) = f.pop() {
                    choix.push(cog.clone());
                }
            }
        }
        choix
    }
}
