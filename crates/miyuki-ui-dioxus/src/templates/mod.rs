// @id: MUID-Templates @do: templates-module @role: exports @layer: 6 @human: miyuk

//! Template-level page layouts.

pub mod auth;
pub mod dashboard;
pub mod detail;
pub mod fullscreen;
pub mod service;
pub mod settings;
pub mod split;

pub use auth::AuthLayout;
pub use dashboard::DashboardLayout;
pub use detail::DetailLayout;
pub use fullscreen::FullscreenLayout;
pub use service::ServiceLayout;
pub use settings::SettingsLayout;
pub use split::SplitLayout;
