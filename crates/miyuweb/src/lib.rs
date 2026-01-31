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
