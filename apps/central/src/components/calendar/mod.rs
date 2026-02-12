//! Composants calendrier generiques pour Miyukini Central.
//!
//! @id: central.components.calendar
//! @do: provide_reusable_calendar_views
//! @role: ui
//! @layer: ui
//! @human: Composants calendrier reutilisables (jour, semaine, mois)
//!
//! Ce module fournit des composants calendrier generiques utilisables
//! par tous les services (JayKoa, JayFestival, etc.).

pub mod types;
pub mod day_view;
pub mod week_view;
pub mod month_view;
pub mod agenda_list;

pub use types::{ViewMode, SimpleEvent, extract_date};
pub use day_view::CalendarDayView;
pub use week_view::CalendarWeekView;
pub use month_view::CalendarMonthView;
pub use agenda_list::AgendaListItem;
