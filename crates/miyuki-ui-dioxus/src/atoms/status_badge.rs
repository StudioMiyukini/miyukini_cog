// @id: MUID-StatusBadge @do: status-badge-atom @role: component @layer: 6 @human: miyuk

//! StatusBadge atom — domaine-specific status indicator built on Badge.
//!
//! Maps domain status strings (active, inactive, pending, etc.) to semantic BadgeVariant.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     StatusBadge { status: StatusKind::Active }
//!     StatusBadge { status: StatusKind::Pending, size: BadgeSize::Sm }
//! }
//! ```
//!
//! @id: muid_status_badge @do: render_status_badge
//! @role: ui @layer: service
//! @human: StatusBadge — badge sémantique domaine (Active/Inactive/Pending/Archived/Draft).

use dioxus::prelude::*;

use super::badge::{Badge, BadgeSize, BadgeVariant};

/// Domain-level status kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusKind {
    /// Active / published / confirmed.
    Active,
    /// Inactive / closed.
    Inactive,
    /// Pending review or approval.
    #[default]
    Pending,
    /// Archived (soft-deleted).
    Archived,
    /// Draft (not yet submitted).
    Draft,
}

impl StatusKind {
    /// Returns the display label for this status.
    pub fn label(self) -> &'static str {
        match self {
            Self::Active => "Actif",
            Self::Inactive => "Inactif",
            Self::Pending => "En attente",
            Self::Archived => "Archivé",
            Self::Draft => "Brouillon",
        }
    }

    /// Maps to BadgeVariant for visual styling.
    pub fn variant(self) -> BadgeVariant {
        match self {
            Self::Active => BadgeVariant::Success,
            Self::Inactive => BadgeVariant::Error,
            Self::Pending => BadgeVariant::Warning,
            Self::Archived => BadgeVariant::Default,
            Self::Draft => BadgeVariant::Info,
        }
    }
}

/// Domain-specific status badge using semantic coloring.
#[component]
pub fn StatusBadge(
    /// Status kind to display.
    status: StatusKind,
    /// Badge size.
    #[props(default)]
    size: BadgeSize,
) -> Element {
    rsx! {
        Badge {
            label: status.label().to_string(),
            variant: status.variant(),
            size,
        }
    }
}
