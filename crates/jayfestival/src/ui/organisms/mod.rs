//! Organisms JayFestival — composants de niveau page (Header, Layout, grilles, bannières).
//!
//! Conforme [Specification UI Conforme Catakana] § 2. Dépendent des atoms et molecules.
//! @layer: ui — réexport uniquement.

mod cta_section;
mod directory_banner;
mod features_grid;
mod gestion_layout;
mod header;
mod header_landing;
mod header_with_edition;
mod hero_section;
mod layout;
mod roles_grid;

pub use cta_section::{cta_section_render, CtaCardItem};
pub use directory_banner::directory_banner_render;
pub use features_grid::{features_grid_render, FeatureCardItem};
pub use gestion_layout::gestion_layout_show;
pub use header::header_render;
pub use header_landing::{header_landing_render, LandingHeaderResponses};
pub use header_with_edition::header_with_edition_render;
pub use hero_section::hero_section_render;
pub use layout::layout_show;
pub use roles_grid::{roles_grid_render, RoleCardItem};
