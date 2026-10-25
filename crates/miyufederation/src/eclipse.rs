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
/// Pairs au plus dans un carnet (borne mémoire et persistance ; au-delà, un
/// nouveau pair n'entre qu'en évinçant un pair **inéligible** : les anciens
/// pairs éligibles ne sont jamais chassés, voir [`Federation::noter_contact`]).
pub const MAX_PAIRS_CARNET: usize = 4_096;

/// Étiquette de réseau acceptée : non vide, ≤ [`TAILLE_MAX_RESEAU`] octets, sans caractère de contrôle.
pub(crate) fn reseau_valide(reseau: &str) -> bool {
    !reseau.is_empty() && reseau.len() <= TAILLE_MAX_RESEAU && !reseau.chars().any(char::is_control)
}

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
    /// Carnet plein ([`MAX_PAIRS_CARNET`]) : nouveau pair ignoré (`false`) ;
    /// [`Federation::noter_contact`] évince d'abord un pair inéligible.
    pub fn noter_contact(&mut self, cog: &CogId, reseau: &str, maintenant: u64) -> bool {
        if !reseau_valide(reseau) {
            return false;
        }
        if !self.pairs.contains_key(cog) && self.pairs.len() >= MAX_PAIRS_CARNET {
            return false;
        }
        self.pairs
            .entry(cog.clone())
            .or_insert_with(|| Pair { reseau: reseau.to_string(), premier_contact: maintenant });
        true
    }

    /// Nombre de pairs connus.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Carnet vide.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Premier contact d'un pair.
    pub fn premier_contact(&self, cog: &CogId) -> Option<u64> {
        self.pairs.get(cog).map(|p| p.premier_contact)
    }

    /// Pairs dans l'ordre des `cog_id` : `(cog, réseau, premier contact)`.
    pub(crate) fn pairs(&self) -> impl Iterator<Item = (&CogId, &str, u64)> {
        self.pairs.iter().map(|(c, p)| (c, p.reseau.as_str(), p.premier_contact))
    }

    pub(crate) fn contient(&self, cog: &CogId) -> bool {
        self.pairs.contains_key(cog)
    }

    pub(crate) fn retirer(&mut self, cog: &CogId) {
        self.pairs.remove(cog);
    }

    /// Restauration d'un état persisté (contrôles faits par l'appelant).
    pub(crate) fn restaurer(&mut self, cog: CogId, reseau: String, premier_contact: u64) {
        self.pairs.insert(cog, Pair { reseau, premier_contact });
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
