#![allow(missing_docs)]
//! # MiyuText — toolkit.text.miyutext
//!
//! Kit d'outils de traitement de texte (markdown, replace, template, sanitize). Contenu fourni dans le flux ; pas de décision métier.
//! Alignement MIP : domaine `text`, layer tool/toolkit.

// @id: toolkit.text.miyutext
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuText ; expose les modules tools.
/// @do: expose_miyutext_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod markdown;
pub mod replace;
pub mod sanitize;
pub mod template;

pub use admin_cell::{
    miyutext_admin_cell, MiyuTextAdminCell, MiyuTextIdentification, MiyuTextIntegrity,
    MiyuTextTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuTextError;
pub use markdown::{render as markdown_render, MarkdownOptions};
pub use replace::{replace as text_replace, ReplaceMode};
pub use sanitize::{sanitize as text_sanitize, SanitizePolicy};
pub use template::apply as template_apply;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_literal_ok() {
        let ctx = crate::context::GovernedContext::new("m".into(), 0);
        let out = text_replace(&ctx, "hello world", "world", "Rust", ReplaceMode::Literal).unwrap();
        assert_eq!(out, "hello Rust");
    }

    #[test]
    fn replace_regex_returns_explicit_error_v01() {
        let ctx = crate::context::GovernedContext::new("m".into(), 0);
        let err = text_replace(&ctx, "a", ".", "x", ReplaceMode::Regex).unwrap_err();
        assert!(matches!(err, MiyuTextError::RegexNotSupportedV01));
    }
}
