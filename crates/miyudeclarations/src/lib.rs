#![allow(missing_docs)]
//! # MiyuDeclarations — toolkit.compta.declarations
//!
//! Kit d'outils déclarations fiscales et sociales (URSSAF, TVA, deadline, list, estimate).
//! Alignement MIP : domaine `compta`, layer tool/toolkit.

// @id: toolkit.accounting.miyudeclarations
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuDeclarations ; expose les modules tools.
/// @do: expose_miyudeclarations_toolkit

pub mod admin_cell;
pub mod context;
pub mod deadline;
pub mod declaration;
pub mod errors;
pub mod estimate;
pub mod tva;
pub mod urssaf;

pub use admin_cell::{
    miyudeclarations_admin_cell, MiyudeclarationsAdminCell, MiyudeclarationsIdentification,
    MiyudeclarationsIntegrity, MiyudeclarationsTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use deadline::{list as deadline_list, DeadlineItem};
pub use declaration::{list as declaration_list, DeclarationFilters, DeclarationItem};
pub use errors::MiyudeclarationsError;
pub use estimate::{cotisations as estimate_cotisations, CotisationsEstimate};
pub use tva::{prepare as tva_prepare, submit as tva_submit, TvaPrepareResult};
pub use urssaf::{prepare as urssaf_prepare, submit as urssaf_submit, UrssafPrepareResult};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn declaration_deadline_estimate_tva_urssaf() {
        let c = ctx();
        let filters = DeclarationFilters::default();
        assert!(declaration_list(&c, &filters).unwrap().is_empty());
        assert!(deadline_list(&c, "2024-01-01", "2024-12-31").unwrap().is_empty());
        let est = estimate_cotisations(&c, 0.0, "micro").unwrap();
        assert_eq!(est.total, 0.0);
        let tva = tva_prepare(&c, "2024-Q1").unwrap();
        assert!(tva.declaration_id.is_empty());
        assert_eq!(tva_submit(&c, "tva1").unwrap(), "tva:submitted");
        let urssaf = urssaf_prepare(&c, "2024-01").unwrap();
        assert!(urssaf.declaration_id.is_empty());
        assert_eq!(urssaf_submit(&c, "u1").unwrap(), "urssaf:submitted");
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(tva_submit(&c, "x").is_err());
    }
}
