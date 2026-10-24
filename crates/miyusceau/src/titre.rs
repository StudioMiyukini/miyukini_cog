//! @id     miyusceau_titre
//! @do     definir_trait_titre
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « titre ».

//! Trait des titres scellables et type `Verifie<T>`.

use crate::ancres::NiveauConfiance;
use crate::cles::Kid;
use crate::erreurs::ErreurSceau;
use crate::format::TypeSceau;
use crate::tlv::{EcrivainTlv, LecteurTlv};

/// Contenu signable par un Sceau (passeport, visa, manifeste…).
pub trait Titre: Sized {
    /// Type de sceau (séparation de domaine).
    const TYPE: TypeSceau;

    /// Encode les champs, tags dans l'ordre croissant.
    fn encoder(&self, ecrivain: &mut EcrivainTlv) -> Result<(), ErreurSceau>;

    /// Décode les champs, tags dans l'ordre croissant. Le lecteur est ensuite
    /// contrôlé par `terminer()` : tout champ non lu est un refus.
    fn decoder(lecteur: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau>;

    /// Fenêtre de validité `(pas_avant, pas_apres)` en secondes Unix, si le titre
    /// en a une. Vérifiée avec la tolérance d'horloge du format.
    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        None
    }
}

/// Titre dont la signature, le signataire et la validité ont été vérifiés.
///
/// Seules [`crate::verifier`] et [`crate::verifier_seuil`] peuvent en construire un : détenir un `Verifie<T>`
/// prouve que le contrôle a eu lieu.
#[derive(Debug, Clone)]
pub struct Verifie<T> {
    titre: T,
    signataires: Vec<Kid>,
    niveau: NiveauConfiance,
}

impl<T> Verifie<T> {
    /// `signataires` : non vide, `kid` croissants (garanti par le vérificateur).
    pub(crate) fn nouveau(titre: T, signataires: Vec<Kid>, niveau: NiveauConfiance) -> Self {
        Self { titre, signataires, niveau }
    }

    /// Titre vérifié.
    pub fn titre(&self) -> &T {
        &self.titre
    }

    /// `kid` du premier signataire compté (le plus petit `kid`). Pour un sceau
    /// à signature simple, c'est le signataire.
    pub fn signataire(&self) -> &Kid {
        &self.signataires[0]
    }

    /// Tous les signataires épinglés dont la signature a été vérifiée
    /// (`kid` croissants, sans doublon).
    pub fn signataires(&self) -> &[Kid] {
        &self.signataires
    }

    /// Niveau de confiance : le plus faible des signataires comptés.
    pub fn niveau(&self) -> NiveauConfiance {
        self.niveau
    }

    /// Récupère le titre.
    pub fn into_titre(self) -> T {
        self.titre
    }
}
