//! Identifiants d'écrans JayFestival — UNC, ORG, EXP, VIS.
//!
//! Conforme docs « Écrans et cycle » (UtilisateurNonConnecte, Organisateurs, Exposants, Visiteurs)
//! et [Specification UI § 4].

/// @id: jayfestival_screen_ids
/// @do: define_screen_identifiers_for_routing
/// @layer: app
/// Énumération des écrans : UNC (non connecté), ORG (organisateur), EXP (exposant), VIS (visiteur).

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenId {
    // --- Utilisateur non connecté (UNC) ---
    /// UNC-E01 — Landing / Accueil catalogue
    UncLanding,
    /// UNC-E02 — Liste des événements
    UncListeEvenements,
    /// UNC-E03 — Fiche événement
    UncFicheEvenement,
    /// UNC-E06 — Liste organisateurs
    UncListeOrganisateurs,
    /// UNC-E07 — Fiche organisateur
    UncFicheOrganisateur,
    /// UNC-E08 — Liste exposants
    UncListeExposants,
    /// UNC-E09 — Fiche exposant
    UncFicheExposant,
    /// UNC-E10 — Recherche
    UncRecherche,
    /// UNC-E11 — CTA contextuels
    UncCtaContextuels,
    /// UNC-E12 — Connexion
    UncConnexion,
    /// UNC-E13 — Inscription
    UncInscription,
    /// UNC-E14 — Mentions / CGU
    UncMentions,

    // --- Organisateur (ORG) ---
    /// ORG-E04 — Dashboard organisateur
    OrgDashboard,
    /// ORG-E05 — Liste éditions
    OrgListeEditions,
    /// ORG-E06 — Création édition
    OrgCreationEdition,
    /// ORG-E07 — Dashboard édition
    OrgDashboardEdition,
    /// ORG-E08 — Liste exposants
    OrgListeExposants,
    /// ORG-E09 — Candidatures
    OrgCandidatures,
    /// ORG-E10 — Fiche exposant (org)
    OrgFicheExposant,
    /// ORG-E11 — Plan de salle
    OrgPlanSalle,
    /// ORG-E12 — Programme
    OrgProgramme,
    /// ORG-E13 — Budget
    OrgBudget,
    /// ORG-E14 — Devis / Factures
    OrgDevisFactures,
    /// ORG-E15 — Documents
    OrgDocuments,
    /// ORG-E16 à E25 — reste (réservés)
    OrgReserved(u8),

    // --- Exposant (EXP) ---
    /// EXP-E04 — Dashboard exposant
    ExpDashboard,
    /// EXP-E05 — Candidatures
    ExpCandidatures,
    /// EXP-E06 — Participations
    ExpParticipations,
    /// EXP-E07 à E19 — reste (réservés)
    ExpReserved(u8),

    // --- Visiteur (VIS) ---
    /// VIS-E04 — Dashboard visiteur
    VisDashboard,
    /// VIS-E05 — Agenda
    VisAgenda,
    /// VIS-E06 à E15 — Billets, réservations, pass (réservés)
    VisReserved(u8),
}

impl ScreenId {
    /// Retourne true si l'écran est du parcours UNC (utilisateur non connecté).
    pub fn is_unc(&self) -> bool {
        matches!(
            self,
            ScreenId::UncLanding
                | ScreenId::UncListeEvenements
                | ScreenId::UncFicheEvenement
                | ScreenId::UncListeOrganisateurs
                | ScreenId::UncFicheOrganisateur
                | ScreenId::UncListeExposants
                | ScreenId::UncFicheExposant
                | ScreenId::UncRecherche
                | ScreenId::UncCtaContextuels
                | ScreenId::UncConnexion
                | ScreenId::UncInscription
                | ScreenId::UncMentions
        )
    }

    /// Retourne true si l'écran est du parcours ORG (organisateur).
    pub fn is_org(&self) -> bool {
        matches!(
            self,
            ScreenId::OrgDashboard
                | ScreenId::OrgListeEditions
                | ScreenId::OrgCreationEdition
                | ScreenId::OrgDashboardEdition
                | ScreenId::OrgListeExposants
                | ScreenId::OrgCandidatures
                | ScreenId::OrgFicheExposant
                | ScreenId::OrgPlanSalle
                | ScreenId::OrgProgramme
                | ScreenId::OrgBudget
                | ScreenId::OrgDevisFactures
                | ScreenId::OrgDocuments
                | ScreenId::OrgReserved(_)
        )
    }

    /// Retourne true si l'écran est du parcours EXP (exposant).
    pub fn is_exp(&self) -> bool {
        matches!(
            self,
            ScreenId::ExpDashboard | ScreenId::ExpCandidatures | ScreenId::ExpParticipations | ScreenId::ExpReserved(_)
        )
    }

    /// Retourne true si l'écran est du parcours VIS (visiteur).
    pub fn is_vis(&self) -> bool {
        matches!(
            self,
            ScreenId::VisDashboard | ScreenId::VisAgenda | ScreenId::VisReserved(_)
        )
    }

    /// Liste de tous les écrans UNC (pour tests parcours).
    pub fn all_unc() -> &'static [ScreenId] {
        &[
            ScreenId::UncLanding,
            ScreenId::UncListeEvenements,
            ScreenId::UncFicheEvenement,
            ScreenId::UncListeOrganisateurs,
            ScreenId::UncFicheOrganisateur,
            ScreenId::UncListeExposants,
            ScreenId::UncFicheExposant,
            ScreenId::UncRecherche,
            ScreenId::UncCtaContextuels,
            ScreenId::UncConnexion,
            ScreenId::UncInscription,
            ScreenId::UncMentions,
        ]
    }

    /// Liste de tous les écrans ORG (pour tests parcours).
    pub fn all_org() -> &'static [ScreenId] {
        &[
            ScreenId::OrgDashboard,
            ScreenId::OrgListeEditions,
            ScreenId::OrgCreationEdition,
            ScreenId::OrgDashboardEdition,
            ScreenId::OrgListeExposants,
            ScreenId::OrgCandidatures,
            ScreenId::OrgFicheExposant,
            ScreenId::OrgPlanSalle,
            ScreenId::OrgProgramme,
            ScreenId::OrgBudget,
            ScreenId::OrgDevisFactures,
            ScreenId::OrgDocuments,
        ]
    }

    /// Liste de tous les écrans EXP (pour tests parcours).
    pub fn all_exp() -> &'static [ScreenId] {
        &[
            ScreenId::ExpDashboard,
            ScreenId::ExpCandidatures,
            ScreenId::ExpParticipations,
        ]
    }

    /// Liste de tous les écrans VIS (pour tests parcours).
    pub fn all_vis() -> &'static [ScreenId] {
        &[ScreenId::VisDashboard, ScreenId::VisAgenda]
    }
}

#[cfg(test)]
mod tests {
    use super::ScreenId;

    #[test]
    fn screen_id_unc_all_variants() {
        for s in ScreenId::all_unc() {
            assert!(s.is_unc(), "{:?} should be UNC", s);
            assert!(!s.is_org());
            assert!(!s.is_exp());
            assert!(!s.is_vis());
        }
    }

    #[test]
    fn screen_id_org_all_variants() {
        for s in ScreenId::all_org() {
            assert!(s.is_org(), "{:?} should be ORG", s);
            assert!(!s.is_unc());
            assert!(!s.is_exp());
            assert!(!s.is_vis());
        }
        assert!(ScreenId::OrgReserved(16).is_org());
        assert!(ScreenId::OrgReserved(25).is_org());
    }

    #[test]
    fn screen_id_exp_all_variants() {
        for s in ScreenId::all_exp() {
            assert!(s.is_exp(), "{:?} should be EXP", s);
            assert!(!s.is_unc());
            assert!(!s.is_org());
            assert!(!s.is_vis());
        }
        assert!(ScreenId::ExpReserved(7).is_exp());
    }

    #[test]
    fn screen_id_vis_all_variants() {
        for s in ScreenId::all_vis() {
            assert!(s.is_vis(), "{:?} should be VIS", s);
            assert!(!s.is_unc());
            assert!(!s.is_org());
            assert!(!s.is_exp());
        }
        assert!(ScreenId::VisReserved(6).is_vis());
    }

    #[test]
    fn screen_id_no_overlap() {
        for s in ScreenId::all_unc() {
            assert!(!s.is_org() && !s.is_exp() && !s.is_vis());
        }
        for s in ScreenId::all_org() {
            assert!(!s.is_unc() && !s.is_exp() && !s.is_vis());
        }
        for s in ScreenId::all_exp() {
            assert!(!s.is_unc() && !s.is_org() && !s.is_vis());
        }
        for s in ScreenId::all_vis() {
            assert!(!s.is_unc() && !s.is_org() && !s.is_exp());
        }
    }
}

pub mod unc;
pub mod org;
pub mod exp;
pub mod vis;
