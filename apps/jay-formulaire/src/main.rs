//! Jay Formulaire — créateur de formulaires et sondages.

use dioxus::prelude::*;
use jaybureau_core::{FormField, FormFieldKind, FormSchema};

fn main() {
    tracing_subscriber::fmt().init();
    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("Jay Formulaire")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1000.0, 800.0)),
    );
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mut form = use_signal(|| FormSchema::new("Nouveau formulaire"));

    rsx! {
        style { "
            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
            body, html {{ height: 100%; font-family: 'Segoe UI', sans-serif; background: #f5f5fa; }}
            .form-app {{ display: flex; flex-direction: column; height: 100vh; }}
            .form-header {{ padding: 10px 16px; background: #a855f7; color: white; font-weight: 600; }}
            .form-canvas {{ flex: 1; overflow-y: auto; padding: 30px; }}
            .form-card {{ max-width: 720px; margin: 0 auto 16px auto; background: white; border-radius: 8px; padding: 24px; box-shadow: 0 1px 3px rgba(0,0,0,0.08); }}
            .form-title-input {{ width: 100%; font-size: 28px; font-weight: 500; border: none; outline: none; color: #202124; margin-bottom: 8px; }}
            .form-desc-input {{ width: 100%; font-size: 14px; color: #5f6368; border: none; outline: none; }}
            .field-card {{ border-left: 4px solid transparent; }}
            .field-card.selected {{ border-left-color: #a855f7; }}
            .field-label-input {{ width: 100%; font-size: 16px; font-weight: 500; border: none; outline: none; margin-bottom: 8px; padding: 8px 0; border-bottom: 2px solid #e0e0e8; }}
            .field-kind-select {{ padding: 8px 12px; border: 1px solid #e0e0e8; border-radius: 6px; font-size: 14px; background: white; }}
            .btn {{ padding: 8px 16px; background: #a855f7; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; font-weight: 500; }}
            .btn-ghost {{ background: transparent; border: 1px solid #e0e0e8; color: #555; }}
            .btn + .btn {{ margin-left: 8px; }}
        " }
        div {
            class: "form-app",
            div { class: "form-header", "📝 Jay Formulaire" }
            div {
                class: "form-canvas",
                div {
                    class: "form-card",
                    input {
                        class: "form-title-input",
                        value: "{form.read().title}",
                        oninput: move |evt| {
                            form.write().title = evt.value();
                        },
                    }
                    input {
                        class: "form-desc-input",
                        placeholder: "Description du formulaire",
                        value: "{form.read().description}",
                        oninput: move |evt| {
                            form.write().description = evt.value();
                        },
                    }
                }

                for (idx, field) in form.read().fields.clone().iter().enumerate() {
                    FieldCard {
                        key: "{field.id}",
                        index: idx,
                        label: field.label.clone(),
                        on_remove: move |i: usize| {
                            form.write().fields.remove(i);
                        },
                    }
                }

                div {
                    class: "form-card",
                    style: "text-align: center; cursor: pointer; color: #a855f7;",
                    onclick: move |_| {
                        form.write().fields.push(FormField::new("Question sans titre", FormFieldKind::ShortText));
                    },
                    "+ Ajouter une question"
                }

                div {
                    style: "max-width: 720px; margin: 0 auto; display: flex; gap: 8px; padding: 16px 0;",
                    button { class: "btn", "Publier" }
                    button { class: "btn btn-ghost", "Aperçu" }
                    span { style: "margin-left: auto; color: #888; font-size: 13px; align-self: center;",
                        "{form.read().fields.len()} question(s)"
                    }
                }
            }
        }
    }
}

#[component]
fn FieldCard(index: usize, label: String, on_remove: EventHandler<usize>) -> Element {
    rsx! {
        div {
            class: "form-card field-card selected",
            div {
                style: "display: flex; gap: 16px; align-items: flex-start;",
                div {
                    style: "flex: 1;",
                    input {
                        class: "field-label-input",
                        value: "{label}",
                        placeholder: "Question sans titre",
                    }
                    select {
                        class: "field-kind-select",
                        option { value: "short_text", "Réponse courte" }
                        option { value: "long_text", "Paragraphe" }
                        option { value: "radio", "Choix unique" }
                        option { value: "checkbox", "Cases à cocher" }
                        option { value: "dropdown", "Liste déroulante" }
                        option { value: "scale", "Échelle" }
                        option { value: "date", "Date" }
                        option { value: "file", "Fichier" }
                    }
                }
                button {
                    style: "background: transparent; border: none; color: #888; cursor: pointer; font-size: 18px; padding: 4px 8px;",
                    onclick: move |_| on_remove.call(index),
                    "✕"
                }
            }
        }
    }
}
