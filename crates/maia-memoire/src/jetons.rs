//! @id     maia_memoire_jetons
//! @do     mesurer_jetons_et_budgets
//! @role   securite
//! @layer  8
//! @human  maia-memoire — comptage des jetons et budgets par profil (§2, §7, EX-50).

//! Le budget est **mesuré**, jamais estimé à la baisse (EX-50) : le comptage
//! passe par un [`Tokeniseur`]. Le tokenizer réel du modèle actif s'y branche ;
//! à défaut, [`BorneOctets`] donne une **borne supérieure garantie** pour les
//! tokenizers BPE ou SentencePiece à repli sur octets (Llama, Qwen, Gemma…) :
//! chaque jeton couvre au moins un octet, plus un jeton de préfixe. Le budget
//! n'est donc jamais dépassé, au prix d'une mémoire plus courte.

/// Compte les jetons d'un texte pour le modèle actif.
pub trait Tokeniseur: Send + Sync {
    /// Nombre de jetons de `texte`.
    fn compter(&self, texte: &str) -> usize;
    /// Nom (journal, diagnostic).
    fn nom(&self) -> &str;
}

/// Borne supérieure garantie : octets UTF-8 + 1 (préfixe).
#[derive(Debug, Clone, Copy, Default)]
pub struct BorneOctets;

impl Tokeniseur for BorneOctets {
    fn compter(&self, texte: &str) -> usize {
        texte.len() + 1
    }
    fn nom(&self) -> &'static str {
        "borne-octets"
    }
}

/// Plafond de N0 (§2).
pub const MAX_JETONS_N0: usize = 200;
/// Plafond d'un résumé de domaine N1 (§2).
pub const MAX_JETONS_N1_DOMAINE: usize = 40;
/// Plafond d'un résumé N2 (§2).
pub const MAX_JETONS_N2: usize = 150;
/// Plafond d'une note N3 (§2) — appliqué aussi aux unités de N0.
pub const MAX_JETONS_N3: usize = 60;

/// Profil d'inférence et budget mémoire associé (§7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profil {
    /// Petit modèle (contexte 4 k) : ≤ 600 jetons.
    Petit,
    /// Moyen (8 k) : ≤ 1 500 jetons.
    Moyen,
    /// Grand (32 k et +) : ≤ 4 000 jetons.
    Grand,
    /// Strata : ≤ 4 000 jetons, préfixe N0 + N1 stable.
    Strata,
}

impl Profil {
    /// Budget mémoire du profil, en jetons.
    pub fn budget(self) -> usize {
        match self {
            Profil::Petit => 600,
            Profil::Moyen => 1_500,
            Profil::Grand | Profil::Strata => 4_000,
        }
    }

    /// Lecture d'un nom de profil ; inconnu ⇒ `None` (l'appelant retient le plus petit).
    pub fn depuis_str(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "petit" | "s1" => Some(Profil::Petit),
            "moyen" => Some(Profil::Moyen),
            "grand" => Some(Profil::Grand),
            "strata" | "s2" => Some(Profil::Strata),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borne_octets_majorant() {
        assert_eq!(BorneOctets.compter(""), 1);
        assert_eq!(BorneOctets.compter("abc"), 4);
        assert_eq!(BorneOctets.compter("é"), 3);
        assert_eq!(Profil::Petit.budget(), 600);
        assert_eq!(Profil::depuis_str("STRATA"), Some(Profil::Strata));
        assert_eq!(Profil::depuis_str("inconnu"), None);
    }
}
