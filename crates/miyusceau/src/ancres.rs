//! @id     miyusceau_ancres
//! @do     gerer_ancres_confiance
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « ancres ».

//! Ancres de confiance : clés publiques épinglées, indexées par `kid`.
//!
//! Incrément 1 : ancres directes (le signataire doit être lui-même une ancre).
//! La validation de chaîne (racine → clé d'émission via certificat) arrive à
//! l'incrément 2 et s'appuiera sur ce même registre.

use std::collections::HashMap;

use ed25519_dalek::VerifyingKey;

use crate::cles::{empreinte_cle, Kid};
use crate::erreurs::ErreurSceau;

/// Niveau de confiance d'une ancre (correspond aux niveaux BorderGuard).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NiveauConfiance {
    /// Clé de son propre COG (BorderGuard `Trusted`).
    Souverain,
    /// Clé d'un COG reconnu ou d'Origin (BorderGuard `Verified`).
    Reconnu,
}

/// Registre des clés publiques de confiance.
#[derive(Debug, Default, Clone)]
pub struct AncresDeConfiance {
    cles: HashMap<Kid, (VerifyingKey, NiveauConfiance)>,
}

impl AncresDeConfiance {
    /// Registre vide (refuse tout).
    pub fn new() -> Self {
        Self::default()
    }

    /// Épingle une clé publique. Refuse les clés invalides ou faibles
    /// (point d'ordre petit), qui permettraient de forger des signatures.
    pub fn ajouter(&mut self, cle_publique: &[u8; 32], niveau: NiveauConfiance) -> Result<Kid, ErreurSceau> {
        let cle = VerifyingKey::from_bytes(cle_publique).map_err(|_| ErreurSceau::CleInvalide)?;
        if cle.is_weak() {
            return Err(ErreurSceau::CleInvalide);
        }
        let kid = empreinte_cle(cle_publique);
        self.cles.insert(kid, (cle, niveau));
        Ok(kid)
    }

    /// Retire une ancre (révocation locale).
    pub fn retirer(&mut self, kid: &Kid) -> bool {
        self.cles.remove(kid).is_some()
    }

    /// Nombre d'ancres.
    pub fn len(&self) -> usize {
        self.cles.len()
    }

    /// Vrai si aucune ancre.
    pub fn is_empty(&self) -> bool {
        self.cles.is_empty()
    }

    pub(crate) fn trouver(&self, kid: &Kid) -> Option<&(VerifyingKey, NiveauConfiance)> {
        self.cles.get(kid)
    }
}
