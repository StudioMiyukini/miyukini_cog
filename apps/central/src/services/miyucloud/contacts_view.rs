//! Vue contacts CardDAV dans Central.
//!
//! @id: miyucloud_contacts_view
//! @do: display_carddav_addressbooks_contacts
//! @role: component
//! @layer: presentation
//!
//! Affiche la liste des carnets d'adresses (sidebar gauche) et la liste
//! des contacts du carnet selectionne avec recherche.

use dioxus::prelude::*;

use super::client::MiyuCloudClient;
use super::state::MiyuCloudState;
use crate::state::use_app_state;

/// Vue principale contacts CardDAV.
#[component]
pub fn ContactsView(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let addressbooks = state.read().addressbooks.clone();
    let selected_book = state.read().selected_addressbook.clone();
    let contacts = state.read().contacts.clone();
    let search = state.read().contacts_search.clone();
    let selected_contact = state.read().selected_contact_uid.clone();
    let error_msg = state.read().error_message.clone();

    // Charger les carnets au montage
    let mut loaded = use_signal(|| false);
    use_effect(move || {
        if *loaded.read() {
            return;
        }
        loaded.set(true);

        spawn(async move {
            let http = { client.read().clone() };
            let Some(http) = http else { return };

            match http.list_addressbooks().await {
                Ok(books) => {
                    let first_name = books.first().map(|b| b.name.clone());
                    state.write().addressbooks = books;
                    if state.read().selected_addressbook.is_none() {
                        if let Some(name) = first_name {
                            state.write().selected_addressbook = Some(name);
                        }
                    }
                }
                Err(e) => {
                    state.write().error_message =
                        Some(format!("Echec chargement carnets : {e}"));
                }
            }
        });
    });

    // Charger les contacts quand le carnet selectionne change
    let selected_book_memo = use_memo(move || state.read().selected_addressbook.clone());
    use_effect(move || {
        let book_name = selected_book_memo.read().clone();
        let Some(book_name) = book_name else { return };

        spawn(async move {
            let http = { client.read().clone() };
            let Some(http) = http else { return };

            match http.list_contacts(&book_name).await {
                Ok(cts) => {
                    state.write().contacts = cts;
                }
                Err(e) => {
                    state.write().error_message =
                        Some(format!("Echec chargement contacts : {e}"));
                }
            }
        });
    });

    // Filtrer les contacts par recherche
    let filtered: Vec<_> = if search.is_empty() {
        contacts.clone()
    } else {
        let q = search.to_lowercase();
        contacts
            .iter()
            .filter(|ct| {
                ct.full_name.to_lowercase().contains(&q)
                    || ct.email.as_deref().unwrap_or("").to_lowercase().contains(&q)
                    || ct.phone.as_deref().unwrap_or("").contains(&q)
            })
            .cloned()
            .collect()
    };

    rsx! {
        div {
            style: "flex: 1; display: flex; overflow: hidden;",

            // Sidebar carnets
            div {
                style: "width: 220px; min-width: 220px; border-right: 1px solid {c.border}; display: flex; flex-direction: column; overflow-y: auto; background: {c.bg_secondary};",

                // Header
                div {
                    style: "padding: 12px 16px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 14px; color: {c.text_white}; font-weight: 600;", "Carnets" }
                    button {
                        style: "padding: 2px 8px; background: {c.accent_blue}20; color: {c.accent_blue}; border: 1px solid {c.accent_blue}40; border-radius: 4px; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            spawn(async move {
                                let http = { client.read().clone() };
                                let Some(http) = http else { return };
                                let name = format!("book-{}", rand_id());
                                match http.create_addressbook(&name, "Nouveau carnet").await {
                                    Ok(book) => {
                                        let mut s = state.write();
                                        s.addressbooks.push(book);
                                        s.selected_addressbook = Some(name);
                                    }
                                    Err(e) => {
                                        state.write().error_message = Some(format!("Echec creation : {e}"));
                                    }
                                }
                            });
                        },
                        "+"
                    }
                }

                // Liste des carnets
                for book in addressbooks.iter() {
                    {
                        let book_name = book.name.clone();
                        let display = book.display_name.clone();
                        let is_selected = selected_book.as_deref() == Some(book_name.as_str());
                        let bg = if is_selected { c.bg_hover } else { "transparent" };
                        let text_col = if is_selected { c.text_white } else { c.text_secondary };

                        rsx! {
                            button {
                                style: "display: flex; align-items: center; gap: 8px; padding: 8px 16px; background: {bg}; color: {text_col}; border: none; cursor: pointer; font-size: 13px; text-align: left; width: 100%;",
                                onclick: move |_| {
                                    state.write().selected_addressbook = Some(book_name.clone());
                                    state.write().selected_contact_uid = None;
                                },
                                span { "\u{1F4D6}" }
                                span { style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{display}" }
                            }
                        }
                    }
                }
            }

            // Zone principale : liste contacts
            div {
                style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",

                // Erreur
                if let Some(ref err) = error_msg {
                    div {
                        style: "padding: 8px 16px; background: {c.accent_red}20; border-left: 3px solid {c.accent_red}; margin: 8px 16px 0; border-radius: 4px; color: {c.accent_red}; font-size: 13px;",
                        "{err}"
                    }
                }

                // Barre de recherche
                div {
                    style: "display: flex; align-items: center; gap: 8px; padding: 8px 16px; border-bottom: 1px solid {c.border};",
                    span { style: "font-size: 14px; color: {c.text_muted};", "\u{1F50D}" }
                    input {
                        style: "flex: 1; padding: 6px 10px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; font-size: 13px; outline: none;",
                        r#type: "text",
                        placeholder: "Rechercher un contact...",
                        value: "{search}",
                        oninput: move |evt| {
                            state.write().contacts_search = evt.value();
                        },
                    }
                    button {
                        style: "padding: 6px 12px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            let book = state.read().selected_addressbook.clone();
                            let Some(book_name) = book else { return };

                            spawn(async move {
                                let http = { client.read().clone() };
                                let Some(http) = http else { return };

                                let contact = serde_json::json!({
                                    "full_name": "Nouveau Contact",
                                    "email": "",
                                    "phone": "",
                                });
                                match http.upsert_contact(&book_name, &contact).await {
                                    Ok(ct) => {
                                        state.write().contacts.push(ct);
                                    }
                                    Err(e) => {
                                        state.write().error_message = Some(format!("Echec creation : {e}"));
                                    }
                                }
                            });
                        },
                        "+ Contact"
                    }
                }

                // Liste des contacts
                div {
                    style: "flex: 1; overflow-y: auto;",

                    if filtered.is_empty() {
                        div {
                            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 48px; color: {c.text_muted};",
                            span { style: "font-size: 48px; margin-bottom: 16px; opacity: 0.5;", "\u{1F464}" }
                            p { style: "font-size: 16px; margin: 0 0 8px;", "Aucun contact" }
                            p { style: "font-size: 13px;", "Selectionnez un carnet et ajoutez des contacts." }
                        }
                    }

                    for contact in filtered.iter() {
                        {
                            let uid = contact.uid.clone();
                            let name = contact.full_name.clone();
                            let email = contact.email.clone().unwrap_or_default();
                            let phone = contact.phone.clone().unwrap_or_default();
                            let is_selected = selected_contact.as_deref() == Some(uid.as_str());
                            let bg = if is_selected { c.bg_hover } else { "transparent" };
                            let border_left = if is_selected {
                                format!("3px solid {}", c.accent_blue)
                            } else {
                                "3px solid transparent".to_string()
                            };

                            // Initiale pour l'avatar
                            let initial = name.chars().next().unwrap_or('?').to_uppercase().to_string();

                            rsx! {
                                div {
                                    style: "display: flex; align-items: center; gap: 12px; padding: 10px 16px; background: {bg}; border-left: {border_left}; cursor: pointer; border-bottom: 1px solid {c.border}10;",
                                    onclick: move |_| {
                                        state.write().selected_contact_uid = Some(uid.clone());
                                    },

                                    // Avatar rond
                                    div {
                                        style: "width: 36px; height: 36px; border-radius: 50%; background: {c.accent_blue}30; color: {c.accent_blue}; display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 600; flex-shrink: 0;",
                                        "{initial}"
                                    }

                                    // Info
                                    div {
                                        style: "flex: 1; min-width: 0;",
                                        div {
                                            style: "font-size: 14px; color: {c.text_white}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                            "{name}"
                                        }
                                        if !email.is_empty() {
                                            div {
                                                style: "font-size: 12px; color: {c.text_muted}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                                "{email}"
                                            }
                                        }
                                        if !phone.is_empty() {
                                            div {
                                                style: "font-size: 12px; color: {c.text_muted};",
                                                "{phone}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Genere un petit ID aleatoire.
fn rand_id() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("{:x}", now & 0xFFFF_FFFF)
}
