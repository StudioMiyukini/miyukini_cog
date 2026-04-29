//! Barre d'outils de Jay Docs.

use dioxus::prelude::*;

#[component]
pub fn Toolbar() -> Element {
    rsx! {
        div {
            class: "doc-toolbar",
            ToolGroup {
                ToolBtn { icon: "↶", title: "Annuler" }
                ToolBtn { icon: "↷", title: "Rétablir" }
            }
            div { class: "tool-sep" }
            ToolGroup {
                StyleSelect {}
            }
            div { class: "tool-sep" }
            ToolGroup {
                ToolBtn { icon: "B", title: "Gras" }
                ToolBtn { icon: "I", title: "Italique" }
                ToolBtn { icon: "U", title: "Souligné" }
                ToolBtn { icon: "S", title: "Barré" }
            }
            div { class: "tool-sep" }
            ToolGroup {
                ToolBtn { icon: "↤", title: "Aligner à gauche" }
                ToolBtn { icon: "↔", title: "Centrer" }
                ToolBtn { icon: "↦", title: "Aligner à droite" }
            }
            div { class: "tool-sep" }
            ToolGroup {
                ToolBtn { icon: "•", title: "Liste à puces" }
                ToolBtn { icon: "1.", title: "Liste numérotée" }
            }
            div { class: "tool-sep" }
            ToolGroup {
                ToolBtn { icon: "🔗", title: "Insérer lien" }
                ToolBtn { icon: "🖼", title: "Insérer image" }
                ToolBtn { icon: "💬", title: "Commenter" }
            }
        }
    }
}

#[component]
fn ToolGroup(children: Element) -> Element {
    rsx! {
        div { class: "tool-group", {children} }
    }
}

#[component]
fn ToolBtn(icon: &'static str, title: &'static str) -> Element {
    rsx! {
        button {
            class: "tool-btn",
            title: "{title}",
            "{icon}"
        }
    }
}

#[component]
fn StyleSelect() -> Element {
    rsx! {
        select {
            class: "tool-select",
            option { value: "paragraph", "Texte normal" }
            option { value: "h1", "Titre 1" }
            option { value: "h2", "Titre 2" }
            option { value: "h3", "Titre 3" }
            option { value: "code", "Code" }
            option { value: "quote", "Citation" }
        }
    }
}
