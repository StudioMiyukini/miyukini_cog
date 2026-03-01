//! Ecran Dispositifs Alicia Home Assistante.
//!
//! Liste complete de tous les dispositifs domotiques de la maison.
//! Placeholder : les donnees reelles seront fournies par miyualicia-devices.

use dioxus::prelude::*;

use crate::state::use_app_state;
use super::state::AliciaSnapshot;

// ─────────────────────────────────────────────────────────────────────────────
// Types locaux (placeholders en attendant miyualicia-devices)
// ─────────────────────────────────────────────────────────────────────────────

/// Types de dispositifs domotiques supportes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeviceType {
    Light,
    Shutter,
    Sensor,
    Switch,
    Thermostat,
}

impl DeviceType {
    fn label(self) -> &'static str {
        match self {
            Self::Light => "Lumiere",
            Self::Shutter => "Volet",
            Self::Sensor => "Capteur",
            Self::Switch => "Interrupteur",
            Self::Thermostat => "Thermostat",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::Light => "[L]",
            Self::Shutter => "[V]",
            Self::Sensor => "[S]",
            Self::Switch => "[I]",
            Self::Thermostat => "[T]",
        }
    }

    fn all() -> &'static [DeviceType] {
        &[
            Self::Light,
            Self::Shutter,
            Self::Sensor,
            Self::Switch,
            Self::Thermostat,
        ]
    }
}

/// Protocoles de communication supportes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Protocol {
    Mqtt,
    Http,
    Zigbee,
    Zwave,
}

impl Protocol {
    fn label(self) -> &'static str {
        match self {
            Self::Mqtt => "MQTT",
            Self::Http => "HTTP",
            Self::Zigbee => "Zigbee",
            Self::Zwave => "Z-Wave",
        }
    }

    fn all() -> &'static [Protocol] {
        &[Self::Mqtt, Self::Http, Self::Zigbee, Self::Zwave]
    }
}

/// Pieces disponibles pour l'affectation des dispositifs.
const ROOM_OPTIONS: &[(&str, &str)] = &[
    ("chambre-theresa", "Chambre Theresa"),
    ("chambre-parentale", "Chambre parentale"),
    ("chambre-eleanore", "Chambre Eleanore"),
    ("salon", "Salon"),
];

// ─────────────────────────────────────────────────────────────────────────────
// Composant principal
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn DevicesScreen() -> Element {
    let app_state = use_app_state();
    let c = app_state.read().current_theme.palette();
    let _snapshot = use_context::<Signal<Option<AliciaSnapshot>>>();

    // Signal local pour afficher/masquer le formulaire d'ajout
    let mut show_add_form = use_signal(|| false);

    // Signaux locaux pour le formulaire d'ajout
    let mut form_name = use_signal(String::new);
    let mut form_type_idx = use_signal(|| 0_usize);
    let mut form_room_idx = use_signal(|| 0_usize);
    let mut form_protocol_idx = use_signal(|| 0_usize);
    let mut form_address = use_signal(String::new);

    let is_form_visible = *show_add_form.read();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 1000px;",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    h2 {
                        style: "font-size: 16px; font-weight: 600; color: {c.text_white};",
                        "Dispositifs"
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_secondary};",
                        "Tous les dispositifs domotiques de ta maison, centralises."
                    }
                }
                button {
                    style: "padding: 8px 16px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 500;",
                    onclick: move |_| {
                        let current = *show_add_form.read();
                        show_add_form.set(!current);
                    },
                    if is_form_visible { "Annuler" } else { "Ajouter un dispositif" }
                }
            }

            // Formulaire d'ajout (visible si show_add_form)
            if is_form_visible {
                div {
                    style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.accent_blue}40; display: flex; flex-direction: column; gap: 16px;",
                    h3 {
                        style: "font-size: 14px; font-weight: 600; color: {c.text_white};",
                        "Nouveau dispositif"
                    }

                    // Nom
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label {
                            style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                            "Nom"
                        }
                        input {
                            r#type: "text",
                            placeholder: "Ex: Lampe bureau",
                            style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                            value: "{form_name}",
                            oninput: move |evt: Event<FormData>| {
                                form_name.set(evt.value());
                            },
                        }
                    }

                    // Ligne Type + Piece
                    div {
                        style: "display: flex; gap: 16px;",

                        // Type
                        div {
                            style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                            label {
                                style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                                "Type"
                            }
                            select {
                                style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                                onchange: move |evt: Event<FormData>| {
                                    if let Ok(idx) = evt.value().parse::<usize>() {
                                        form_type_idx.set(idx);
                                    }
                                },
                                for (i, dt) in DeviceType::all().iter().enumerate() {
                                    {
                                        let idx_str = format!("{i}");
                                        let lbl = dt.label();
                                        let selected = *form_type_idx.read() == i;
                                        rsx! {
                                            option {
                                                value: "{idx_str}",
                                                selected: selected,
                                                "{lbl}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Piece
                        div {
                            style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                            label {
                                style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                                "Piece"
                            }
                            select {
                                style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                                onchange: move |evt: Event<FormData>| {
                                    if let Ok(idx) = evt.value().parse::<usize>() {
                                        form_room_idx.set(idx);
                                    }
                                },
                                for (i, (_id, name)) in ROOM_OPTIONS.iter().enumerate() {
                                    {
                                        let idx_str = format!("{i}");
                                        let selected = *form_room_idx.read() == i;
                                        rsx! {
                                            option {
                                                value: "{idx_str}",
                                                selected: selected,
                                                "{name}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Ligne Protocole + Adresse
                    div {
                        style: "display: flex; gap: 16px;",

                        // Protocole
                        div {
                            style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                            label {
                                style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                                "Protocole"
                            }
                            select {
                                style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                                onchange: move |evt: Event<FormData>| {
                                    if let Ok(idx) = evt.value().parse::<usize>() {
                                        form_protocol_idx.set(idx);
                                    }
                                },
                                for (i, proto) in Protocol::all().iter().enumerate() {
                                    {
                                        let idx_str = format!("{i}");
                                        let lbl = proto.label();
                                        let selected = *form_protocol_idx.read() == i;
                                        rsx! {
                                            option {
                                                value: "{idx_str}",
                                                selected: selected,
                                                "{lbl}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Adresse
                        div {
                            style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                            label {
                                style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                                "Adresse (topic MQTT ou URL)"
                            }
                            input {
                                r#type: "text",
                                placeholder: "Ex: home/salon/lampe1",
                                style: "padding: 8px 12px; background: {c.bg_secondary}; border-radius: 4px; border: 1px solid {c.border}; color: {c.text_primary}; font-size: 13px;",
                                value: "{form_address}",
                                oninput: move |evt: Event<FormData>| {
                                    form_address.set(evt.value());
                                },
                            }
                        }
                    }

                    // Bouton enregistrer (placeholder — pas de backend)
                    div {
                        style: "display: flex; gap: 12px; margin-top: 4px;",
                        button {
                            style: "padding: 8px 20px; background: {c.accent_green}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 500; opacity: 0.5;",
                            disabled: true,
                            "Enregistrer (backend non integre)"
                        }
                    }
                }
            }

            // Tableau des dispositifs (placeholder vide)
            div {
                style: "background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border}; overflow: hidden;",

                // En-tete du tableau
                div {
                    style: "display: grid; grid-template-columns: 40px 1fr 100px 120px 100px 80px 140px 80px; gap: 8px; padding: 12px 16px; background: {c.bg_secondary}; border-bottom: 1px solid {c.border};",
                    span {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        ""
                    }
                    span {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Nom"
                    }
                    span {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Type"
                    }
                    span {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Piece"
                    }
                    span {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Protocole"
                    }
                    span {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Etat"
                    }
                    span {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Mise a jour"
                    }
                    span {
                        style: "font-size: 11px; color: {c.text_secondary}; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Actions"
                    }
                }

                // Corps vide
                div {
                    style: "padding: 48px 16px; text-align: center; color: {c.text_muted};",
                    p {
                        style: "font-size: 14px; margin-bottom: 8px;",
                        "Aucun dispositif configure"
                    }
                    p {
                        style: "font-size: 12px;",
                        "Les dispositifs seront disponibles une fois le crate miyualicia-devices integre au backend. "
                        "Le formulaire ci-dessus te permet de preparer la configuration."
                    }
                }
            }

            // Note informative
            div {
                style: "padding: 16px; background: {c.bg_secondary}; border-radius: 8px; display: flex; gap: 12px; align-items: flex-start;",
                span {
                    style: "font-size: 18px; color: {c.accent_blue};",
                    "[i]"
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    p {
                        style: "font-size: 13px; color: {c.text_primary}; font-weight: 500;",
                        "Integration en cours"
                    }
                    p {
                        style: "font-size: 12px; color: {c.text_secondary};",
                        "L'ecran Dispositifs sera pleinement fonctionnel avec l'integration de miyualicia-devices "
                        "et du bridge MQTT. Les dispositifs Zigbee/Z-Wave necesseront un dongle compatible."
                    }
                }
            }
        }
    }
}
