//! @id     miyusceau_cles
//! @do     gerer_cles_signature
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « cles ».

//! Clés de signature Ed25519 et identifiants de clé (`kid`).

use ed25519_dalek::{Signer, SigningKey};
use rand_core::OsRng;
use sha2::{Digest, Sha256};

/// Identifiant de clé : SHA-256 de la clé publique Ed25519 (32 octets).
pub type Kid = [u8; 32];

/// Calcule le `kid` d'une clé publique.
pub fn empreinte_cle(cle_publique: &[u8; 32]) -> Kid {
    Sha256::digest(cle_publique).into()
}

/// Clé privée de signature. Effacée de la mémoire à la destruction
/// (`SigningKey` implémente `ZeroizeOnDrop`). Jamais affichée.
pub struct CleSignature {
    interne: SigningKey,
}

impl CleSignature {
    /// Génère une nouvelle clé à partir de l'aléa du système (OsRng).
    pub fn generer() -> Self {
        Self { interne: SigningKey::generate(&mut OsRng) }
    }

    /// Reconstruit une clé depuis sa graine de 32 octets.
    pub fn depuis_graine(graine: &[u8; 32]) -> Self {
        Self { interne: SigningKey::from_bytes(graine) }
    }

    /// Graine secrète (à chiffrer avant tout stockage).
    pub fn graine(&self) -> [u8; 32] {
        self.interne.to_bytes()
    }

    /// Clé publique (32 octets).
    pub fn cle_publique(&self) -> [u8; 32] {
        self.interne.verifying_key().to_bytes()
    }

    /// Identifiant de la clé.
    pub fn kid(&self) -> Kid {
        empreinte_cle(&self.cle_publique())
    }

    /// Signe des octets (usage interne au format Sceau).
    pub(crate) fn signer(&self, octets: &[u8]) -> [u8; 64] {
        self.interne.sign(octets).to_bytes()
    }
}

impl std::fmt::Debug for CleSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CleSignature")
            .field("kid", &hex(&self.kid()[..8]))
            .field("secret", &"<masqué>")
            .finish()
    }
}

pub(crate) fn hex(octets: &[u8]) -> String {
    use std::fmt::Write;
    octets.iter().fold(String::with_capacity(octets.len() * 2), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}
