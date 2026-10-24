//! @id     miyauth_pairwise
//! @do     deriver_identifiant_par_cog
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — identifiant par paire (pairwise) du titulaire, différent par COG visité (EX-36).

//! Identifiant par paire (EX-36).
//!
//! La doc gelée (v1.2) exige un identifiant différent par COG visité sans fixer la
//! dérivation. Choix retenu (signalé au coordinateur) : **HKDF-SHA256** (RFC 5869)
//!
//! ```text
//! sel   = "miyukini/miyauth/pairwise/v1"
//! ikm   = secret du titulaire (32 o, détenu par le COG d'origine, jamais exporté)
//! info  = "cog:" || cog_id visité (26 o, longueur fixe → encodage non ambigu)
//! sortie = 32 o
//! ```
//!
//! - Stable : même secret + même COG → même identifiant.
//! - Non reliable : sans le secret, deux identifiants pour deux COG sont
//!   indistinguables de valeurs aléatoires (HKDF = PRF).
//! - Séparation de domaine : sel propre à cet usage et versionné.

use hkdf::Hkdf;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use zeroize::Zeroize;

use crate::titres::CogId;

/// Sel de domaine de la dérivation (versionné).
pub const SEL_PAIRWISE: &[u8] = b"miyukini/miyauth/pairwise/v1";

/// Secret de pseudonymisation d'un titulaire (32 octets), détenu par son COG
/// d'origine. Effacé de la mémoire à la destruction ; jamais affiché.
#[derive(Clone)]
pub struct SecretTitulaire([u8; 32]);

impl Drop for SecretTitulaire {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl SecretTitulaire {
    /// Nouveau secret aléatoire (aléa du système).
    pub fn generer() -> Self {
        let mut octets = [0u8; 32];
        OsRng.fill_bytes(&mut octets);
        Self(octets)
    }

    /// Reconstruit un secret depuis son stockage (déchiffré par l'appelant).
    pub fn depuis_octets(octets: [u8; 32]) -> Self {
        Self(octets)
    }

    /// Octets secrets (à chiffrer avant tout stockage).
    pub fn octets(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for SecretTitulaire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SecretTitulaire(<masqué>)")
    }
}

/// Identifiant par paire du titulaire pour le COG `cog_visite`.
pub fn identifiant_pairwise(secret: &SecretTitulaire, cog_visite: &CogId) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(Some(SEL_PAIRWISE), &secret.0);
    let mut info = Vec::with_capacity(4 + cog_visite.as_str().len());
    info.extend_from_slice(b"cog:");
    info.extend_from_slice(cog_visite.as_str().as_bytes());
    let mut sortie = [0u8; 32];
    // 32 o ≤ 255 × 32 o : l'expansion HKDF ne peut pas échouer.
    hk.expand(&info, &mut sortie).expect("longueur HKDF valide");
    sortie
}
