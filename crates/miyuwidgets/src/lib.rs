#![allow(missing_docs)]
//! # MiyuWidgets — toolkit.web.widgets
//!
//! Kit d'outils widgets web (layout, blocs texte/image/bouton/grille/conteneur, template).
//! Données fournies dans le flux ; persistance templates/layouts = KindMother.
//! Alignement MIP : domaine `web` / widgets, layer tool/toolkit.

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod layout;
mod store;
pub mod template;
pub mod widget;

pub use admin_cell::{
    miyuwidgets_admin_cell, MiyuwidgetsAdminCell, MiyuwidgetsIdentification, MiyuwidgetsIntegrity,
    MiyuwidgetsTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuwidgetsError;
pub use layout::apply as layout_apply;
pub use template::{register as template_register, resolve as template_resolve};
pub use widget::{
    button_render as widget_button_render,
    container_render as widget_container_render,
    grid_render as widget_grid_render,
    image_render as widget_image_render,
    text_render as widget_text_render,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn widget_renders_return_markup() {
        let c = ctx();
        assert!(widget_text_render(&c, "hi").unwrap().contains("<span>"));
        assert!(widget_image_render(&c, "url").unwrap().contains("<img"));
        assert!(widget_button_render(&c, "OK").unwrap().contains("<button>"));
        assert!(widget_grid_render(&c, "{}").unwrap().contains("grid"));
        assert!(widget_container_render(&c, "{}").unwrap().contains("container"));
    }

    #[test]
    fn template_register_resolve() {
        let c = ctx();
        template_register(&c, "t1", "content").unwrap();
        assert_eq!(template_resolve(&c, "t1").unwrap(), "content");
        assert_eq!(template_resolve(&c, "missing").unwrap(), "");
    }

    #[test]
    fn layout_apply_returns_id() {
        let c = ctx();
        let id = layout_apply(&c, "{}").unwrap();
        assert!(id.starts_with("layout:"));
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(widget_text_render(&c, "x").is_err());
        assert!(template_resolve(&c, "x").is_err());
    }
}
