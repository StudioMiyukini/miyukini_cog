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
pub mod template;
pub mod widget;

pub use admin_cell::{
    miyuwidgets_admin_cell, MiyuwidgetsAdminCell, MiyuwidgetsIdentification, MiyuwidgetsIntegrity,
    MiyuwidgetsTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuwidgetsError;
pub use layout::apply as layout_apply;
pub use template::resolve as template_resolve;
pub use widget::{
    button_render as widget_button_render,
    container_render as widget_container_render,
    grid_render as widget_grid_render,
    image_render as widget_image_render,
    text_render as widget_text_render,
};
