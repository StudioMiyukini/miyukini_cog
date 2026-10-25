//! @id     miyucloud_dav_lib
//! @do     declarer_module_common
//! @role   orchestration
//! @layer  core
//! @human  miyucloud-dav — module « lib ».

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

pub mod common;
pub mod webdav;
pub mod caldav;
pub mod carddav;
pub mod thumbnails;
pub mod wopi;
