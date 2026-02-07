//! Identifiants d'écrans JayXpose — EXP (Espace Exposant), PUB (Accès Public).
//!
//! Conforme à la spécification écrans JayXpose (XP-E01..XP-E12, XP-P01..XP-P06).

/// @id: jayxpose_screen_ids
/// @do: define_screen_identifiers_for_routing_jayxpose
/// @layer: app
/// Énumération des écrans JayXpose : EXP (espace exposant), PUB (accès public).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenId {
    // --- Espace Exposant (EXP) ---
    /// XP-E01 — Dashboard exposant
    ExpDashboard,
    /// XP-E02 — Fiche entreprise
    ExpFicheEntreprise,
    /// XP-E03 — Catalogue produits (liste)
    ExpCatalogueListe,
    /// XP-E04 — Fiche produit (création/modification)
    ExpFicheProduit,
    /// XP-E05 — Gestion des catégories
    ExpCategories,
    /// XP-E06 — Vitrine paramètres
    ExpVitrineParams,
    /// XP-E07 — Vitrine page présentation
    ExpVitrinePresentation,
    /// XP-E08 — Vitrine prévisualisation
    ExpVitrinePreview,
    /// XP-E09 — Coffre-fort documents
    ExpDocuments,
    /// XP-E10 — Upload document
    ExpUploadDocument,
    /// XP-E11 — Fiche publique / confidentialité
    ExpFichePublique,
    /// XP-E12 — Partage documents
    ExpPartageDocuments,

    // --- Accès Public (PUB) ---
    /// XP-P01 — Annuaire exposants
    PubAnnuaire,
    /// XP-P02 — Fiche exposant publique
    PubFicheExposant,
    /// XP-P03 — Catalogue produits publique
    PubCataloguePublic,
    /// XP-P04 — Site vitrine publique
    PubVitrine,
    /// XP-P05 — Recherche
    PubRecherche,
    /// XP-P06 — Page contact vitrine
    PubVitrineContact,
}

impl ScreenId {
    /// Retourne true si l'écran est du parcours EXP (espace exposant).
    pub fn is_exp(&self) -> bool {
        matches!(
            self,
            ScreenId::ExpDashboard
                | ScreenId::ExpFicheEntreprise
                | ScreenId::ExpCatalogueListe
                | ScreenId::ExpFicheProduit
                | ScreenId::ExpCategories
                | ScreenId::ExpVitrineParams
                | ScreenId::ExpVitrinePresentation
                | ScreenId::ExpVitrinePreview
                | ScreenId::ExpDocuments
                | ScreenId::ExpUploadDocument
                | ScreenId::ExpFichePublique
                | ScreenId::ExpPartageDocuments
        )
    }

    /// Retourne true si l'écran est du parcours PUB (accès public).
    pub fn is_pub(&self) -> bool {
        matches!(
            self,
            ScreenId::PubAnnuaire
                | ScreenId::PubFicheExposant
                | ScreenId::PubCataloguePublic
                | ScreenId::PubVitrine
                | ScreenId::PubRecherche
                | ScreenId::PubVitrineContact
        )
    }

    /// Liste de tous les écrans EXP (pour tests parcours).
    pub fn all_exp() -> &'static [ScreenId] {
        &[
            ScreenId::ExpDashboard,
            ScreenId::ExpFicheEntreprise,
            ScreenId::ExpCatalogueListe,
            ScreenId::ExpFicheProduit,
            ScreenId::ExpCategories,
            ScreenId::ExpVitrineParams,
            ScreenId::ExpVitrinePresentation,
            ScreenId::ExpVitrinePreview,
            ScreenId::ExpDocuments,
            ScreenId::ExpUploadDocument,
            ScreenId::ExpFichePublique,
            ScreenId::ExpPartageDocuments,
        ]
    }

    /// Liste de tous les écrans PUB (pour tests parcours).
    pub fn all_pub() -> &'static [ScreenId] {
        &[
            ScreenId::PubAnnuaire,
            ScreenId::PubFicheExposant,
            ScreenId::PubCataloguePublic,
            ScreenId::PubVitrine,
            ScreenId::PubRecherche,
            ScreenId::PubVitrineContact,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::ScreenId;

    #[test]
    fn screen_id_exp_all_variants() {
        for s in ScreenId::all_exp() {
            assert!(s.is_exp(), "{s:?} should be EXP");
            assert!(!s.is_pub(), "{s:?} should not be PUB");
        }
    }

    #[test]
    fn screen_id_pub_all_variants() {
        for s in ScreenId::all_pub() {
            assert!(s.is_pub(), "{s:?} should be PUB");
            assert!(!s.is_exp(), "{s:?} should not be EXP");
        }
    }

    #[test]
    fn screen_id_no_overlap() {
        for s in ScreenId::all_exp() {
            assert!(!s.is_pub(), "{s:?} should not overlap with PUB");
        }
        for s in ScreenId::all_pub() {
            assert!(!s.is_exp(), "{s:?} should not overlap with EXP");
        }
    }

    #[test]
    fn screen_id_exp_count() {
        assert_eq!(
            ScreenId::all_exp().len(),
            12,
            "EXP should have 12 screens (XP-E01..XP-E12)"
        );
    }

    #[test]
    fn screen_id_pub_count() {
        assert_eq!(
            ScreenId::all_pub().len(),
            6,
            "PUB should have 6 screens (XP-P01..XP-P06)"
        );
    }

    #[test]
    fn screen_id_default_is_exp() {
        let default = ScreenId::ExpDashboard;
        assert!(default.is_exp());
    }

    #[test]
    fn screen_id_clone_copy() {
        let a = ScreenId::PubAnnuaire;
        let b = a;
        assert_eq!(a, b);
    }
}

pub mod exp;
pub mod pub_;
