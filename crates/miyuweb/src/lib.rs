#![allow(missing_docs)]
//! # MiyuWeb — toolkit.web.miyuweb
//!
//! Kit d'outils d'affichage web (rendu HTML, layout, thème, scripts, assets, formulaires, événements).
//! Données fournies dans le flux ; pas de décision de contenu ni accès direct base.
//! Alignement MIP : domaine `web`, layer tool/toolkit.

pub mod admin_cell;
pub mod asset;
pub mod context;
pub mod errors;
pub mod event;
pub mod form;
pub mod html;
pub mod input;
pub mod layout;
pub mod script;
pub mod theme;

pub use admin_cell::{
    miyuweb_admin_cell, MiyuwebAdminCell, MiyuwebIdentification, MiyuwebIntegrity,
    MiyuwebTestManifest, TOOLKIT_ID,
};
pub use asset::serve as asset_serve;
pub use context::GovernedContext;
pub use errors::MiyuwebError;
pub use event::dispatch as event_dispatch;
pub use form::validate as form_validate;
pub use html::render as html_render;
pub use input::capture as input_capture;
pub use layout::render as layout_render;
pub use script::{compile as script_compile, execute as script_execute};
pub use theme::resolve as theme_resolve;

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("test-mandate".to_string(), 1)
    }

    #[test]
    fn html_render_returns_template() {
        let c = ctx();
        let out = html_render(&c, "<p>Hello</p>", "{}").unwrap();
        assert_eq!(out, "<p>Hello</p>");
    }

    #[test]
    fn theme_resolve_returns_default() {
        let c = ctx();
        assert_eq!(theme_resolve(&c, "any").unwrap(), "default");
    }

    #[test]
    fn layout_render_returns_data() {
        let c = ctx();
        assert_eq!(layout_render(&c, "{\"title\":\"x\"}").unwrap(), "{\"title\":\"x\"}");
    }

    #[test]
    fn input_capture_returns_value() {
        let c = ctx();
        assert_eq!(input_capture(&c, "click", "btn-1").unwrap(), "btn-1");
    }

    #[test]
    fn asset_serve_event_form_script() {
        let c = ctx();
        assert!(asset_serve(&c, "img/logo.png", None).unwrap().is_empty());
        event_dispatch(&c, "click", "{}").unwrap();
        assert!(form_validate(&c, "{}", "{}").unwrap());
        assert!(script_compile(&c, "x").unwrap());
        assert!(script_execute(&c, "x", "{}").unwrap().is_empty());
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(html_render(&c, "", "").is_err());
        assert!(theme_resolve(&c, "").is_err());
    }
}
