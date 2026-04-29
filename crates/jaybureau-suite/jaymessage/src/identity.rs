//! Cles d'identite long-terme (ed25519).

use crate::{JayMessageError, Result, UserId};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

/// Cle d'identite ed25519 d'un utilisateur (long-terme).
#[derive(Debug)]
pub struct IdentityKey {
    pub user_id: UserId,
    signing_key: SigningKey,
}

/// Cle publique partageable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIdentity {
    pub user_id: UserId,
    /// Cle publique ed25519 (32 bytes, hex).
    pub public_key: String,
}

impl IdentityKey {
    /// Genere une nouvelle identite aleatoire.
    pub fn generate(user_id: UserId) -> Self {
        let mut csprng = rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Self {
            user_id,
            signing_key,
        }
    }

    /// Charge depuis une cle privee hex.
    pub fn from_hex(user_id: UserId, hex_key: &str) -> Result<Self> {
        let bytes = hex::decode(hex_key)
            .map_err(|e| JayMessageError::Crypto(format!("hex decode: {e}")))?;
        if bytes.len() != 32 {
            return Err(JayMessageError::Crypto(
                "ed25519 secret must be 32 bytes".into(),
            ));
        }
        let arr: [u8; 32] = bytes.try_into().unwrap();
        let signing_key = SigningKey::from_bytes(&arr);
        Ok(Self {
            user_id,
            signing_key,
        })
    }

    /// Exporte la cle privee (pour persistence — a stocker chiffre !).
    pub fn to_hex(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }

    /// Cle publique partageable.
    pub fn public(&self) -> PublicIdentity {
        let vk: VerifyingKey = self.signing_key.verifying_key();
        PublicIdentity {
            user_id: self.user_id.clone(),
            public_key: hex::encode(vk.to_bytes()),
        }
    }

    /// Signe un message.
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let sig: Signature = self.signing_key.sign(message);
        sig.to_bytes().to_vec()
    }
}

impl PublicIdentity {
    /// Verifie une signature.
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()> {
        let vk_bytes = hex::decode(&self.public_key)
            .map_err(|e| JayMessageError::Crypto(format!("pubkey decode: {e}")))?;
        if vk_bytes.len() != 32 {
            return Err(JayMessageError::Crypto("pubkey must be 32 bytes".into()));
        }
        let vk_arr: [u8; 32] = vk_bytes.try_into().unwrap();
        let vk = VerifyingKey::from_bytes(&vk_arr)
            .map_err(|e| JayMessageError::Crypto(format!("invalid pubkey: {e}")))?;

        if signature.len() != 64 {
            return Err(JayMessageError::Crypto("sig must be 64 bytes".into()));
        }
        let sig_arr: [u8; 64] = signature.try_into().unwrap();
        let sig = Signature::from_bytes(&sig_arr);
        vk.verify(message, &sig)
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
}
