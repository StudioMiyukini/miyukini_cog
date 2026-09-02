//! @id     jaykoa_views_calendar_view
//! @do     definir_calendar_main_view_props
//! @role   orchestration
//! @layer  app
//! @human  jaykoa — module « views calendar view ».

//! Vue calendrier wrapper — Composant conteneur pour les differentes vues.

use dioxus::prelude::*;

/// Props pour la vue calendrier principale.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarMainViewProps {
    pub children: Element,
}

/// Conteneur de la vue calendrier.
#[component]
pub fn CalendarMainView(props: CalendarMainViewProps) -> Element {
    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",
            {props.children}
        }
    }
}
