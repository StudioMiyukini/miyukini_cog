//! @id     miyusql_context
//! @do     representer_contexte_gouverne
//! @role   data
//! @layer  toolkit
//! @human  miyusql — contexte gouverné (mandat signé, niveau de sécurité).
//!
//! Contexte gouverné pour l'exécution MiyuSQL.
//!
//! Mandat d'exécution, niveau de sécurité, pas d'identité Opérateur (BOUND-5).
//!
//! ## Vérification du mandat (K-19, C-12)
//!
//! Un identifiant de mandat non vide ne suffit plus : le mandat doit porter une
//! [`SignatureMandat`] émise par StrongFather. Tant que la signature StrongFather
//! n'est pas câblée (lot C du plan), aucune signature ne peut être construite hors
//! tests : la vérification échoue donc toujours (fail-closed) et MiyuSQL refuse
//! toute exécution avec [`MiyuSQLError::MandatNonSigne`].

use crate::errors::MiyuSQLError;

// @id miyusql_governed_context
// @role data
// @layer toolkit
// @do represent_governed_context
// @human Contexte gouverné pour l'exécution MiyuSQL (mandat, niveau sécurité, pas d'identité Opérateur).
// Contexte fourni par la gouvernance ; MiyuSQL ne connaît pas l'Opérateur appelant.

/// @id miyusql_signature_mandat
/// @role data
/// @layer toolkit
/// @do represent_mandate_signature
//@human / @human Preuve de signature StrongFather d'un mandat (crochet typé).
//@human Aucun constructeur public hors tests : en production, aucun mandat n'est
//@human vérifiable tant que la signature StrongFather n'est pas implémentée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureMandat {
    /// Champ privé : empêche toute construction hors de ce module.
    _scelle: (),
}

impl SignatureMandat {
    /// @id miyusql_signature_mandat_pour_tests
    /// @role mutator
    /// @layer toolkit
    /// @do build_test_signature
    //@human / @human Signature factice réservée aux tests (`cfg(test)` ou feature
    //@human `test-support`, désactivée par défaut).
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn pour_tests() -> Self {
        Self { _scelle: () }
    }
}

/// @id miyusql_governed_context_struct
/// @role data
/// @layer toolkit
/// @do hold_governed_context
//@human / @human Contexte d'exécution gouverné.
/// @depends miyusql_governed_context
#[derive(Debug, Clone)]
pub struct GovernedContext {
    /// @id miyusql_context_mandate_id
    /// @role data
    /// @layer toolkit
    /// @do store_mandate_id
    //@human / @human Identifiant du mandat d'exécution.
    /// @depends miyusql_governed_context_struct
    pub mandate_id: String,

    /// @id miyusql_context_security_level
    /// @role data
    /// @layer toolkit
    /// @do store_security_level
    //@human / @human Niveau de sécurité (0-4, WorrySentinel).
    /// @depends miyusql_governed_context_struct
    pub security_level: u8,

    /// Signature StrongFather du mandat (absente par défaut → refus).
    signature: Option<SignatureMandat>,
}

impl GovernedContext {
    /// @id miyusql_context_new
    /// @role mutator
    /// @layer toolkit
    /// @do build_governed_context
    //@human / @human Construit un contexte gouverné, sans signature : il sera refusé
    //@human tant qu'une [`SignatureMandat`] n'y est pas attachée.
    /// @depends miyusql_governed_context_struct
    #[must_use]
    pub fn new(mandate_id: String, security_level: u8) -> Self {
        Self {
            mandate_id,
            security_level,
            signature: None,
        }
    }

    /// @id miyusql_context_avec_signature
    /// @role mutator
    /// @layer toolkit
    /// @do attach_mandate_signature
    //@human / @human Attache la signature StrongFather du mandat.
    /// @depends miyusql_signature_mandat
    #[must_use]
    pub fn avec_signature(mut self, signature: SignatureMandat) -> Self {
        self.signature = Some(signature);
        self
    }

    /// @id miyusql_context_verifier_mandat
    /// @role accessor
    /// @layer toolkit
    /// @do verify_mandate_fail_closed
    //@human / @human Vérifie le mandat (fail-closed) : identifiant non vide ET signature
    //@human StrongFather présente. Sinon refus explicite.
    /// @depends miyusql_governed_context_struct
    pub fn verifier_mandat(&self) -> Result<(), MiyuSQLError> {
        if self.mandate_id.trim().is_empty() {
            return Err(MiyuSQLError::NoMandate);
        }
        if self.signature.is_none() {
            return Err(MiyuSQLError::MandatNonSigne);
        }
        Ok(())
    }

    /// @id miyusql_context_has_mandate
    /// @role accessor
    /// @layer toolkit
    /// @do check_mandate_valid
    //@human / @human Indique si le contexte a un mandat vérifié (voir `verifier_mandat`).
    /// @depends miyusql_governed_context_struct
    #[must_use]
    pub fn has_mandate(&self) -> bool {
        self.verifier_mandat().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id miyusql_context_test_mandat_non_signe
    /// @role test
    /// @layer toolkit
    /// @do verify_unsigned_mandate_refused
    //@human / @human Un mandat non vide mais non signé est refusé (K-19).
    #[test]
    fn mandat_non_signe_refuse() {
        let ctx = GovernedContext::new("mandat-quelconque".into(), 2);
        assert!(!ctx.has_mandate());
        assert!(matches!(
            ctx.verifier_mandat(),
            Err(MiyuSQLError::MandatNonSigne)
        ));
    }

    /// @id miyusql_context_test_mandat_vide
    /// @role test
    /// @layer toolkit
    /// @do verify_empty_mandate_refused
    //@human / @human Un mandat vide est refusé, même signé.
    #[test]
    fn mandat_vide_refuse_meme_signe() {
        let ctx =
            GovernedContext::new("  ".into(), 2).avec_signature(SignatureMandat::pour_tests());
        assert!(matches!(
            ctx.verifier_mandat(),
            Err(MiyuSQLError::NoMandate)
        ));
    }

    /// @id miyusql_context_test_mandat_signe
    /// @role test
    /// @layer toolkit
    /// @do verify_signed_mandate_accepted
    //@human / @human Un mandat signé (signature de test) est accepté.
    #[test]
    fn mandat_signe_accepte() {
        let ctx = GovernedContext::new("m".into(), 2).avec_signature(SignatureMandat::pour_tests());
        assert!(ctx.has_mandate());
    }
}
