//! ORG-E21 — Gestion de l'equipe et des invitations.
//!
//! @id: jf_org_equipe @do: render_org_equipe
//! @role: ui @layer: service
//! @human: Ecran ORG-E21 JayFestival: gestion de l equipe organisatrice et invitations.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use super::components::{ActionButton, Badge};

/// Page gestion de l'equipe organisateur.
#[component]
pub fn OrgEquipe() -> Element {
    let p = use_palette();

    let mut show_invite_form = use_signal(|| false);

    // Données de démonstration
    let membres = vec![
        ("Alice Martin", "alice@organisation.fr", "Admin", "Actif"),
        ("Bob Dupont", "bob@organisation.fr", "Manager", "Actif"),
        ("Claire Petit", "claire@organisation.fr", "Contributeur", "Invite"),
    ];

    let invitations = vec![
        ("david@example.com", "Manager", "En attente", "01/02/2026"),
    ];

    rsx! {
        div {
            style: "max-width: 900px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",

                h2 {
                    style: "font-size: 24px; color: {p.text_high};",
                    "Mon equipe"
                }

                ActionButton {
                    label: "Inviter un membre".to_string(),
                    icon: "➕".to_string(),
                    accent: true,
                    onclick: move |_| show_invite_form.set(true),
                }
            }

            // Formulaire d'invitation
            if *show_invite_form.read() {
                section {
                    style: "background: {p.bg_secondary}; border-radius: 8px; padding: 24px; margin-bottom: 24px;",

                    h3 {
                        style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                        "Inviter un nouveau membre"
                    }

                    div {
                        style: "display: flex; gap: 16px; align-items: flex-end;",

                        div {
                            style: "flex: 1;",
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 6px;",
                                "Email"
                            }
                            input {
                                r#type: "email",
                                style: "width: 100%; padding: 10px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 14px;",
                                placeholder: "email@example.com",
                            }
                        }

                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 6px;",
                                "Role"
                            }
                            select {
                                style: "padding: 10px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 14px;",

                                option { value: "admin", "Admin" }
                                option { value: "manager", "Manager" }
                                option { value: "contributeur", "Contributeur" }
                                option { value: "lecteur", "Lecteur" }
                            }
                        }

                        ActionButton {
                            label: "Envoyer l'invitation".to_string(),
                            icon: "📧".to_string(),
                            accent: true,
                            onclick: move |_| show_invite_form.set(false),
                        }

                        button {
                            style: "padding: 10px 16px; background: transparent; border: none; color: {p.text_muted}; cursor: pointer; font-size: 13px;",
                            onclick: move |_| show_invite_form.set(false),
                            "Annuler"
                        }
                    }
                }
            }

            // Membres actuels
            section {
                style: "margin-bottom: 32px;",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Membres ({membres.len()})"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    for (name, email, role, status) in &membres {
                        div {
                            style: "display: flex; align-items: center; gap: 16px; background: {p.bg_secondary}; border-radius: 8px; padding: 16px;",

                            // Avatar
                            div {
                                style: "width: 48px; height: 48px; background: {p.bg_overlay}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 18px; color: {p.text_high};",
                                {name.chars().next().unwrap_or('?').to_string()}
                            }

                            // Infos
                            div {
                                style: "flex: 1;",
                                p {
                                    style: "font-size: 14px; color: {p.text_high};",
                                    "{name}"
                                }
                                p {
                                    style: "font-size: 12px; color: {p.text_muted};",
                                    "{email}"
                                }
                            }

                            // Role
                            Badge {
                                text: role.to_string(),
                                color: match *role {
                                    "Admin" => p.error.to_string(),
                                    "Manager" => p.accent_primary.to_string(),
                                    _ => p.success.to_string(),
                                },
                            }

                            // Statut
                            Badge {
                                text: status.to_string(),
                                color: if *status == "Actif" { p.success.to_string() } else { p.warning.to_string() },
                            }

                            // Actions
                            div {
                                style: "display: flex; gap: 8px;",

                                button {
                                    style: "padding: 6px 10px; background: {p.bg_overlay}; border: none; border-radius: 4px; color: {p.text_primary}; cursor: pointer; font-size: 11px;",
                                    "Modifier"
                                }
                                button {
                                    style: "padding: 6px 10px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_muted}; cursor: pointer; font-size: 11px;",
                                    "Retirer"
                                }
                            }
                        }
                    }
                }
            }

            // Invitations en attente
            section {
                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Invitations en attente ({invitations.len()})"
                }

                if invitations.is_empty() {
                    div {
                        style: "padding: 20px; text-align: center; background: {p.bg_secondary}; border-radius: 8px; color: {p.text_muted};",
                        "Aucune invitation en attente"
                    }
                } else {
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        for (email, role, status, date) in &invitations {
                            div {
                                style: "display: flex; align-items: center; gap: 16px; background: {p.bg_secondary}; border-radius: 8px; padding: 16px;",

                                // Icône
                                div {
                                    style: "width: 48px; height: 48px; background: {p.bg_overlay}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 18px;",
                                    "📧"
                                }

                                // Infos
                                div {
                                    style: "flex: 1;",
                                    p {
                                        style: "font-size: 14px; color: {p.text_high};",
                                        "{email}"
                                    }
                                    p {
                                        style: "font-size: 12px; color: {p.text_muted};",
                                        "Invite le {date}"
                                    }
                                }

                                // Role
                                Badge {
                                    text: role.to_string(),
                                    color: p.accent_primary.to_string(),
                                }

                                // Statut
                                Badge {
                                    text: status.to_string(),
                                    color: p.warning.to_string(),
                                }

                                // Actions
                                div {
                                    style: "display: flex; gap: 8px;",

                                    button {
                                        style: "padding: 6px 10px; background: {p.bg_overlay}; border: none; border-radius: 4px; color: {p.text_primary}; cursor: pointer; font-size: 11px;",
                                        "Renvoyer"
                                    }
                                    button {
                                        style: "padding: 6px 10px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_muted}; cursor: pointer; font-size: 11px;",
                                        "Annuler"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Rôles et permissions
            section {
                style: "margin-top: 32px; padding-top: 32px; border-top: 1px solid {p.border_default};",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Roles et permissions"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    for (role, desc, perms) in [
                        ("Admin", "Acces complet a l'organisation", vec!["Gerer les membres", "Gerer les evenements", "Gerer la facturation"]),
                        ("Manager", "Gestion des evenements", vec!["Creer des evenements", "Gerer les exposants", "Voir les statistiques"]),
                        ("Contributeur", "Participation aux evenements", vec!["Modifier les evenements", "Gerer le programme", "Communiquer"]),
                        ("Lecteur", "Consultation uniquement", vec!["Voir les evenements", "Voir les statistiques"]),
                    ] {
                        div {
                            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 16px;",

                            h4 {
                                style: "font-size: 14px; color: {p.text_high}; margin-bottom: 8px;",
                                "{role}"
                            }
                            p {
                                style: "font-size: 12px; color: {p.text_muted}; margin-bottom: 12px;",
                                "{desc}"
                            }
                            ul {
                                style: "margin: 0; padding-left: 16px;",
                                for perm in perms {
                                    li {
                                        style: "font-size: 11px; color: {p.text_secondary}; margin-bottom: 4px;",
                                        "{perm}"
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
