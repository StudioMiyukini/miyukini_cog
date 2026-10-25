//! @id     miyucloud_lib
//! @do     declarer_module_application
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « lib ».

#![allow(async_fn_in_trait)]
// C-10 : aucun unwrap/expect/panic hors tests dans le code exposé au réseau.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

// Export the main project modules
pub mod application;
pub mod common;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

// Common public re-exports
pub use application::services::folder_service::FolderService;
pub use application::services::i18n_application_service::I18nApplicationService;
pub use domain::services::path_service::StoragePath;
pub use infrastructure::services::path_service::PathService;
