//! @id     jaybureau_suite_jaymessage_identity
//! @do     definir_identity_key
//! @role   donnee
//! @layer  core
//! @human  jaybureau-suite — module « jaymessage identity ».

//! Cles d'identite long-terme (ed25519).

use crate::{JayMessageError, Result, UserId};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

/// Cle d'identite ed25519 d'un utilisateur (long-terme).
///
/// La cle privee est effacee a la destruction (`ed25519-dalek` / `zeroize`) et
/// n'apparait jamais dans `Debug` (C-25).
pub struct IdentityKey {
    pub user_id: UserId,
    signing_key: SigningKey,
}

impl std::fmt::Debug for IdentityKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdentityKey")
            .field("user_id", &self.user_id)
            .field("public_key", &hex::encode(self.verifying_key_bytes()))
            .field("signing_key", &"<redacted>")
            .finish()
    }
}

/// Cle publique partageable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicIdentity {
    pub user_id: UserId,
    /// Cle publique ed25519 (32 bytes, hex).
    pub public_key: String,
}

impl IdentityKey {
    /// Genere une nouvelle identite aleatoire.
    pub fn generate(user_id: UserId) -> Self {
        use rand::RngCore;
        let mut seed = Zeroizing::new([0u8; 32]);
        rand::rngs::OsRng.fill_bytes(seed.as_mut());
        let signing_key = SigningKey::from_bytes(&seed);
        Self {
            user_id,
            signing_key,
        }
    }

    /// Charge depuis une cle privee hex.
    pub fn from_hex(user_id: UserId, hex_key: &str) -> Result<Self> {
        let bytes = Zeroizing::new(
            hex::decode(hex_key)
                .map_err(|e| JayMessageError::Crypto(format!("hex decode: {e}")))?,
        );
        let mut arr = Zeroizing::new([0u8; 32]);
        if bytes.len() != arr.len() {
            return Err(JayMessageError::Crypto(
                "ed25519 secret must be 32 bytes".into(),
            ));
        }
        arr.copy_from_slice(&bytes);
        let signing_key = SigningKey::from_bytes(&arr);
        Ok(Self {
            user_id,
            signing_key,
        })
    }

    /// Exporte la cle privee (pour persistence — a stocker chiffre !).
    /// La chaine retournee est effacee de la memoire a sa destruction.
    pub fn to_hex(&self) -> Zeroizing<String> {
        Zeroizing::new(hex::encode(self.signing_key.to_bytes()))
    }

    /// Cle publique partageable.
    pub fn public(&self) -> PublicIdentity {
        PublicIdentity {
            user_id: self.user_id.clone(),
            public_key: hex::encode(self.verifying_key_bytes()),
        }
    }

    /// Cle publique ed25519 brute.
    pub fn verifying_key_bytes(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }

    /// Signe un message.
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let sig: Signature = self.signing_key.sign(message);
        sig.to_bytes().to_vec()
    }
}

impl PublicIdentity {
    /// Construit a partir d'une cle publique brute.
    pub fn from_bytes(user_id: UserId, public_key: &[u8; 32]) -> Self {
        Self {
            user_id,
            public_key: hex::encode(public_key),
        }
    }

    /// Decode la cle publique ed25519 (32 octets, point valide).
    pub fn verifying_key(&self) -> Result<VerifyingKey> {
        let vk_bytes = hex::decode(&self.public_key)
            .map_err(|e| JayMessageError::Crypto(format!("pubkey decode: {e}")))?;
        let vk_arr: [u8; 32] = vk_bytes
            .as_slice()
            .try_into()
            .map_err(|_| JayMessageError::Crypto("pubkey must be 32 bytes".into()))?;
        VerifyingKey::from_bytes(&vk_arr)
            .map_err(|e| JayMessageError::Crypto(format!("invalid pubkey: {e}")))
    }

    /// Verifie une signature (`verify_strict` : rejette les cles faibles et
    /// les signatures malleables).
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        let vk = self.verifying_key()?;
        let sig_arr: [u8; 64] = signature
            .try_into()
            .map_err(|_| JayMessageError::Crypto("sig must be 64 bytes".into()))?;
        let sig = Signature::from_bytes(&sig_arr);
        vk.verify_strict(message, &sig)
            .map_err(|e| JayMessageError::Crypto(format!("verify: {e}")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_roundtrip() {
        let id = IdentityKey::generate("alice".into());
        let pub_id = id.public();

        let message = b"hello jay message";
        let sig = id.sign(message);

        pub_id.verify(message, &sig).unwrap();
    }

    #[test]
    fn tampered_message_fails_verify() {
        let id = IdentityKey::generate("alice".into());
        let pub_id = id.public();

        let sig = id.sign(b"original");
        assert!(pub_id.verify(b"tampered", &sig).is_err());
    }

    #[test]
    fn export_import_roundtrip() {
        let id1 = IdentityKey::generate("alice".into());
        let hex = id1.to_hex();
        let id2 = IdentityKey::from_hex("alice".into(), &hex).unwrap();
        assert_eq!(id1.public().public_key, id2.public().public_key);
    }

    #[test]
    fn debug_does_not_leak_secret() {
        let id = IdentityKey::generate("alice".into());
        let secret = id.to_hex();
        let dbg = format!("{id:?}");
        assert!(!dbg.contains(secret.as_str()));
        assert!(dbg.contains("<redacted>"));
    }
}
