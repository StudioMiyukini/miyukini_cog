//! Indicateur de synchronisation P2P et panel de gestion des pairs/conflits.
//!
//! @id: miyucloud_sync_status
//! @do: display_sync_status_badge_and_panel
//! @role: component
//! @layer: presentation
//! @human: Lise (Dev Front-End)
//!
//! Ce module fournit :
//! - `SyncStatusBadge` : badge compact dans le header MiyuCloud (pairs en ligne, conflits, etat sync)
//! - `SyncPanel` : panel overlay avec liste des pairs, conflits a resoudre, formulaire d'ajout de pair

use dioxus::prelude::*;

use super::client::MiyuCloudClient;
use super::state::{MiyuCloudState, SyncConflict, SyncPeer};
use crate::state::use_app_state;

// ════════════════════════════════════════════════════════════════════════════
// SyncStatusBadge — Atome : badge compact dans le header
// ════════════════════════════════════════════════════════════════════════════

/// Badge de statut de synchronisation P2P.
///
/// Affiche un point colore + texte compact :
/// - Vert + "X pair(s) en ligne" si des pairs sont connectes
/// - Orange + "Sync en cours..." pendant la synchronisation
/// - Rouge + "X conflit(s)" si des conflits sont en attente
/// - Gris + "Hors ligne" si aucun pair
///
/// Un clic ouvre le `SyncPanel`.
#[component]
pub fn SyncStatusBadge(mut state: Signal<MiyuCloudState>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let peers_online = state.read().sync_status.peers_online;
    let syncing = state.read().sync_status.syncing;
    let pending_conflicts = state.read().sync_status.pending_conflicts;

    // Determiner le label et les couleurs AVANT le RSX
    let has_conflicts = pending_conflicts > 0;

    let dot_color = if has_conflicts {
        c.accent_red
    } else if syncing {
        c.accent_orange
    } else if peers_online > 0 {
        c.accent_green
    } else {
        c.text_muted
    };

    let label = if has_conflicts {
        format!("{pending_conflicts} conflit(s)")
    } else if syncing {
        "Sync en cours...".to_string()
    } else if peers_online > 0 {
        format!("{peers_online} pair(s)")
    } else {
        "Hors ligne".to_string()
    };

    let badge_bg = if has_conflicts {
        c.accent_red
    } else if syncing {
        c.accent_orange
    } else if peers_online > 0 {
        c.accent_green
    } else {
        c.bg_hover
    };

    rsx! {
        button {
            style: "display: inline-flex; align-items: center; gap: 6px; padding: 4px 12px; background: {badge_bg}18; border: 1px solid {badge_bg}40; border-radius: 16px; cursor: pointer; font-size: 12px; color: {c.text_secondary}; transition: all 0.2s;",
            aria_label: "Ouvrir le panel de synchronisation",
            onclick: move |_| {
                let current = state.read().sync_panel_open;
                state.write().sync_panel_open = !current;
            },

            // Dot indicator
            div {
                style: "width: 8px; height: 8px; border-radius: 50%; background: {dot_color}; flex-shrink: 0;",
            }

            span {
                style: "font-weight: 500; white-space: nowrap;",
                "{label}"
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// SyncPanel — Organisme : panel complet de gestion sync
// ════════════════════════════════════════════════════════════════════════════

/// Panel de synchronisation P2P.
///
/// Affiche dans un overlay :
/// - Liste des pairs avec statut, derniere sync, bouton Approuver
/// - Conflits a resoudre avec boutons "Garder local" / "Garder distant"
/// - Formulaire d'ajout de pair (IP + port)
/// - Statistiques de sync
#[component]
pub fn SyncPanel(
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    // Lire les donnees avant RSX
    let peers = state.read().sync_peers.clone();
    let conflicts = state.read().sync_conflicts.clone();
    let sync_info = state.read().sync_status.clone();
    let add_form_open = state.read().add_peer_form_open;
    let add_ip = state.read().add_peer_ip.clone();
    let add_port = state.read().add_peer_port.clone();

    let peers_count = peers.len();
    let online_count = peers.iter().filter(|p| p.is_online).count();
    let conflicts_count = conflicts.len();
    let pending_count = conflicts.iter().filter(|c| c.status == "pending").count();

    let last_sync_label = sync_info
        .last_sync
        .as_deref()
        .map_or_else(|| "Jamais".to_string(), format_sync_date);

    // Charger les donnees au montage
    let mut loaded = use_signal(|| false);
    use_effect(move || {
        if *loaded.read() {
            return;
        }
        loaded.set(true);

        spawn(async move {
            load_sync_data(state, client).await;
        });
    });

    rsx! {
        // Backdrop
        div {
            style: "position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 1000; display: flex; justify-content: flex-end;",
            onclick: move |_| {
                state.write().sync_panel_open = false;
            },

            // Panel (droite)
            div {
                style: "width: 420px; max-width: 90vw; height: 100%; background: {c.bg_secondary}; border-left: 1px solid {c.border}; display: flex; flex-direction: column; overflow: hidden;",
                onclick: move |evt| evt.stop_propagation(),

                // Header
                div {
                    style: "padding: 16px 20px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between;",

                    div {
                        style: "display: flex; align-items: center; gap: 10px;",
                        span { style: "font-size: 20px;", "\u{1F504}" }
                        h3 { style: "font-size: 16px; color: {c.text_white}; margin: 0;", "Synchronisation P2P" }
                    }

                    button {
                        style: "padding: 4px 8px; background: none; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 14px;",
                        aria_label: "Fermer le panel",
                        onclick: move |_| {
                            state.write().sync_panel_open = false;
                        },
                        "\u{2715}"
                    }
                }

                // Content scrollable
                div {
                    style: "flex: 1; overflow-y: auto; padding: 16px 20px; display: flex; flex-direction: column; gap: 20px;",

                    // ── Statistiques rapides ──
                    SyncStatsBar {
                        peers_count: peers_count,
                        online_count: online_count,
                        conflicts_count: pending_count,
                        last_sync: last_sync_label,
                    }

                    // ── Section Pairs ──
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        // Titre + bouton ajouter
                        div {
                            style: "display: flex; align-items: center; justify-content: space-between;",
                            h4 {
                                style: "font-size: 13px; color: {c.text_white}; margin: 0; text-transform: uppercase; letter-spacing: 0.5px;",
                                "Pairs ({peers_count})"
                            }
                            button {
                                style: "padding: 4px 10px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 11px; font-weight: 500;",
                                aria_label: "Ajouter un pair",
                                onclick: move |_| {
                                    let current = state.read().add_peer_form_open;
                                    state.write().add_peer_form_open = !current;
                                },
                                "+ Ajouter"
                            }
                        }

                        // Formulaire d'ajout
                        if add_form_open {
                            AddPeerForm {
                                state: state,
                                client: client,
                                ip_value: add_ip,
                                port_value: add_port,
                            }
                        }

                        // Liste des pairs
                        if peers.is_empty() {
                            div {
                                style: "padding: 20px; text-align: center; color: {c.text_muted}; font-size: 13px;",
                                p { style: "margin: 0 0 4px;", "Aucun pair enregistre" }
                                p { style: "margin: 0; font-size: 11px;", "Ajoutez un pair pour commencer la synchronisation." }
                            }
                        }

                        for peer in peers.iter() {
                            PeerCard {
                                peer: peer.clone(),
                                state: state,
                                client: client,
                            }
                        }
                    }

                    // ── Section Conflits ──
                    if conflicts_count > 0 {
                        div {
                            style: "display: flex; flex-direction: column; gap: 8px;",

                            h4 {
                                style: "font-size: 13px; color: {c.text_white}; margin: 0; text-transform: uppercase; letter-spacing: 0.5px;",
                                "Conflits ({pending_count})"
                            }

                            for conflict in conflicts.iter().filter(|c_item| c_item.status == "pending") {
                                ConflictCard {
                                    conflict: conflict.clone(),
                                    state: state,
                                    client: client,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// SyncStatsBar — Molecule : barre de statistiques rapides
// ════════════════════════════════════════════════════════════════════════════

/// Barre compacte de statistiques de synchronisation.
#[component]
fn SyncStatsBar(
    peers_count: usize,
    online_count: usize,
    conflicts_count: usize,
    last_sync: String,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let peers_label = format!("{online_count}/{peers_count}");
    let conflicts_label = format!("{conflicts_count}");
    let conflict_color = if conflicts_count > 0 {
        c.accent_red
    } else {
        c.accent_green
    };

    rsx! {
        div {
            style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 8px;",

            // Pairs en ligne
            div {
                style: "display: flex; flex-direction: column; gap: 2px; padding: 10px 12px; background: {c.bg_main}; border-radius: 6px;",
                span { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;", "Pairs en ligne" }
                span { style: "font-size: 18px; color: {c.accent_green}; font-weight: 700;", "{peers_label}" }
            }

            // Conflits
            div {
                style: "display: flex; flex-direction: column; gap: 2px; padding: 10px 12px; background: {c.bg_main}; border-radius: 6px;",
                span { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;", "Conflits" }
                span { style: "font-size: 18px; color: {conflict_color}; font-weight: 700;", "{conflicts_label}" }
            }

            // Derniere sync
            div {
                style: "display: flex; flex-direction: column; gap: 2px; padding: 10px 12px; background: {c.bg_main}; border-radius: 6px;",
                span { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;", "Derniere sync" }
                span { style: "font-size: 12px; color: {c.text_primary}; font-weight: 500;", "{last_sync}" }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// PeerCard — Molecule : carte d'un pair
// ════════════════════════════════════════════════════════════════════════════

/// Carte affichant un pair de synchronisation.
///
/// Contient : nom/COG ID, statut en ligne, derniere sync, fingerprint,
/// et un bouton "Approuver" si le pair n'est pas trusted.
#[component]
fn PeerCard(
    peer: SyncPeer,
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let peer_name = peer.name.clone().unwrap_or_else(|| peer.cog_id.clone());
    let is_online = peer.is_online;
    let is_trusted = peer.is_trusted;
    let peer_id = peer.id.clone();

    let status_dot_color = if is_online {
        c.accent_green
    } else {
        c.text_muted
    };
    let status_label = if is_online { "En ligne" } else { "Hors ligne" };

    let last_sync_text = peer.last_sync.as_deref().map_or_else(
        || "Jamais synchronise".to_string(),
        |d| format!("Sync : {}", format_sync_date(d)),
    );

    let last_seen_text = peer.last_seen.as_deref().map_or_else(
        || "Jamais vu".to_string(),
        |d| format!("Vu : {}", format_sync_date(d)),
    );

    // Fingerprint : premiers 16 caracteres de la cle publique
    let fingerprint = if peer.public_key.len() > 16 {
        format!("{}...", &peer.public_key[..16])
    } else {
        peer.public_key.clone()
    };

    let trust_badge_bg = if is_trusted {
        c.accent_green
    } else {
        c.accent_orange
    };
    let trust_badge_label = if is_trusted {
        "Approuve"
    } else {
        "Non approuve"
    };

    rsx! {
        div {
            style: "padding: 12px 14px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 8px; display: flex; flex-direction: column; gap: 8px;",

            // Top row : nom + statut
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    // Dot statut
                    div {
                        style: "width: 8px; height: 8px; border-radius: 50%; background: {status_dot_color}; flex-shrink: 0;",
                    }

                    div {
                        style: "display: flex; flex-direction: column;",
                        span {
                            style: "font-size: 13px; color: {c.text_white}; font-weight: 500;",
                            "{peer_name}"
                        }
                        span {
                            style: "font-size: 11px; color: {c.text_muted};",
                            "{status_label}"
                        }
                    }
                }

                // Badge de confiance
                span {
                    style: "font-size: 10px; color: {trust_badge_bg}; background: {trust_badge_bg}15; padding: 2px 8px; border-radius: 8px; white-space: nowrap;",
                    "{trust_badge_label}"
                }
            }

            // Detail row
            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px; font-size: 11px; color: {c.text_secondary};",
                span { "{last_sync_text}" }
                span { "{last_seen_text}" }
            }

            // Fingerprint
            div {
                style: "display: flex; align-items: center; gap: 6px;",
                span { style: "font-size: 11px; color: {c.text_muted};", "Fingerprint :" }
                span { style: "font-size: 11px; color: {c.text_secondary}; font-family: monospace; background: {c.bg_hover}40; padding: 1px 6px; border-radius: 3px;", "{fingerprint}" }
            }

            // Bouton Approuver si non trusted
            if !is_trusted {
                button {
                    style: "align-self: flex-start; padding: 5px 14px; background: {c.accent_green}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 12px; font-weight: 500;",
                    aria_label: "Approuver ce pair",
                    onclick: move |_| {
                        let pid = peer_id.clone();
                        spawn(async move {
                            let http = {
                                let cl = client.read();
                                cl.clone()
                            };
                            let Some(http) = http else { return };

                            match http.trust_peer(&pid).await {
                                Ok(()) => {
                                    // Mettre a jour le pair localement
                                    let mut peers = state.read().sync_peers.clone();
                                    if let Some(p) = peers.iter_mut().find(|p| p.id == pid) {
                                        p.is_trusted = true;
                                    }
                                    state.write().sync_peers = peers;
                                }
                                Err(e) => {
                                    state.write().error_message = Some(format!("Echec approbation : {e}"));
                                }
                            }
                        });
                    },
                    "Approuver"
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// ConflictCard — Molecule : carte d'un conflit
// ════════════════════════════════════════════════════════════════════════════

/// Carte affichant un conflit de synchronisation.
///
/// Contient : nom du fichier, dates de modification locale/distante,
/// et deux boutons "Garder local" / "Garder distant".
#[component]
fn ConflictCard(
    conflict: SyncConflict,
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let conflict_id = conflict.id.clone();
    let conflict_id_for_remote = conflict.id.clone();
    let file_name = conflict.file_name.clone();
    let local_date = format_sync_date(&conflict.local_modified);
    let remote_date = format_sync_date(&conflict.remote_modified);
    let remote_node = if conflict.remote_node_id.len() > 12 {
        format!("{}...", &conflict.remote_node_id[..12])
    } else {
        conflict.remote_node_id.clone()
    };

    rsx! {
        div {
            style: "padding: 12px 14px; background: {c.bg_main}; border: 1px solid {c.accent_red}40; border-radius: 8px; display: flex; flex-direction: column; gap: 8px;",

            // File name
            div {
                style: "display: flex; align-items: center; gap: 6px;",
                span { style: "font-size: 16px;", "\u{26A0}" }
                span { style: "font-size: 13px; color: {c.text_white}; font-weight: 500;", "{file_name}" }
            }

            // Dates
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 11px;",

                div {
                    style: "display: flex; flex-direction: column; gap: 2px; padding: 6px 8px; background: {c.bg_hover}30; border-radius: 4px;",
                    span { style: "color: {c.text_muted}; text-transform: uppercase; font-size: 10px;", "Local" }
                    span { style: "color: {c.text_primary};", "{local_date}" }
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 2px; padding: 6px 8px; background: {c.bg_hover}30; border-radius: 4px;",
                    span { style: "color: {c.text_muted}; text-transform: uppercase; font-size: 10px;", "Distant ({remote_node})" }
                    span { style: "color: {c.text_primary};", "{remote_date}" }
                }
            }

            // Action buttons
            div {
                style: "display: flex; gap: 8px;",

                button {
                    style: "flex: 1; padding: 6px 12px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 12px; font-weight: 500;",
                    aria_label: "Garder la version locale",
                    onclick: move |_| {
                        let cid = conflict_id.clone();
                        spawn(async move {
                            resolve_conflict_action(cid, "keep_local", state, client).await;
                        });
                    },
                    "Garder local"
                }

                button {
                    style: "flex: 1; padding: 6px 12px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 12px; font-weight: 500;",
                    aria_label: "Garder la version distante",
                    onclick: move |_| {
                        let cid = conflict_id_for_remote.clone();
                        spawn(async move {
                            resolve_conflict_action(cid, "keep_remote", state, client).await;
                        });
                    },
                    "Garder distant"
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// AddPeerForm — Molecule : formulaire d'ajout de pair
// ════════════════════════════════════════════════════════════════════════════

/// Formulaire d'ajout de pair par IP et port.
#[component]
fn AddPeerForm(
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
    ip_value: String,
    port_value: String,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let input_style = format!(
        "padding: 6px 10px; background: {}; color: {}; border: 1px solid {}; border-radius: 4px; font-size: 13px; width: 100%;",
        c.bg_main, c.text_primary, c.border
    );

    let ip_display = ip_value.clone();
    let port_display = port_value.clone();

    rsx! {
        div {
            style: "padding: 12px 14px; background: {c.bg_main}; border: 1px solid {c.accent_blue}40; border-radius: 8px; display: flex; flex-direction: column; gap: 10px;",

            h5 {
                style: "font-size: 12px; color: {c.text_white}; margin: 0; font-weight: 600;",
                "Ajouter un pair"
            }

            // IP
            div {
                style: "display: flex; flex-direction: column; gap: 4px;",
                label {
                    style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;",
                    "Adresse IP"
                }
                input {
                    style: "{input_style}",
                    r#type: "text",
                    placeholder: "192.168.1.100",
                    value: "{ip_display}",
                    aria_label: "Adresse IP du pair",
                    oninput: move |evt: Event<FormData>| {
                        state.write().add_peer_ip = evt.value().to_string();
                    },
                }
            }

            // Port
            div {
                style: "display: flex; flex-direction: column; gap: 4px;",
                label {
                    style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;",
                    "Port"
                }
                input {
                    style: "{input_style}",
                    r#type: "text",
                    placeholder: "11440",
                    value: "{port_display}",
                    aria_label: "Port du pair",
                    oninput: move |evt: Event<FormData>| {
                        state.write().add_peer_port = evt.value().to_string();
                    },
                }
            }

            // Buttons
            div {
                style: "display: flex; gap: 8px; justify-content: flex-end;",

                button {
                    style: "padding: 6px 14px; background: none; color: {c.text_secondary}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 12px;",
                    aria_label: "Annuler l'ajout de pair",
                    onclick: move |_| {
                        state.write().add_peer_form_open = false;
                        state.write().add_peer_ip = String::new();
                        state.write().add_peer_port = "11440".to_string();
                    },
                    "Annuler"
                }

                button {
                    style: "padding: 6px 14px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 12px; font-weight: 500;",
                    aria_label: "Connecter au pair",
                    onclick: move |_| {
                        let ip = state.read().add_peer_ip.clone();
                        let port = state.read().add_peer_port.clone();

                        if ip.is_empty() {
                            state.write().error_message = Some("Veuillez saisir une adresse IP.".to_string());
                            return;
                        }

                        // Generer un COG ID temporaire a partir de l'IP
                        let cog_id = format!("cog-{ip}:{port}");
                        // Cle publique placeholder (le serveur distant la fournira)
                        let public_key = format!("pending-{ip}-{port}");
                        let peer_name = format!("{ip}:{port}");

                        spawn(async move {
                            let http = {
                                let cl = client.read();
                                cl.clone()
                            };
                            let Some(http) = http else { return };

                            match http.register_peer(&cog_id, &public_key, Some(&peer_name)).await {
                                Ok(new_peer) => {
                                    let mut peers = state.read().sync_peers.clone();
                                    peers.push(new_peer);
                                    state.write().sync_peers = peers;
                                    state.write().add_peer_form_open = false;
                                    state.write().add_peer_ip = String::new();
                                    state.write().add_peer_port = "11440".to_string();
                                }
                                Err(e) => {
                                    state.write().error_message = Some(format!("Impossible d'ajouter le pair : {e}"));
                                }
                            }
                        });
                    },
                    "Connecter"
                }
            }

            p {
                style: "font-size: 11px; color: {c.text_muted}; margin: 0; line-height: 1.4;",
                "Saisissez l'adresse IP et le port du serveur MiyuCloud distant. Le pair devra approuver la connexion de son cote."
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// SyncSettingsSection — Organisme : section sync pour la page Settings
// ════════════════════════════════════════════════════════════════════════════

/// Section synchronisation P2P pour la page Settings.
///
/// Remplace le placeholder "Phase C" par les vrais controles :
/// - Statut de la sync
/// - Nombre de pairs
/// - Bouton pour ouvrir le panel
#[component]
pub fn SyncSettingsSection(
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let peers_count = state.read().sync_peers.len();
    let online_count = state.read().sync_status.peers_online;
    let pending_conflicts = state.read().sync_status.pending_conflicts;
    let syncing = state.read().sync_status.syncing;
    let last_sync = state.read().sync_status.last_sync.clone();

    let peers_label = format!("{peers_count}");
    let online_label = format!("{online_count}");
    let conflicts_label = format!("{pending_conflicts}");
    let sync_status_text = if syncing {
        "Synchronisation en cours...".to_string()
    } else {
        "Inactif".to_string()
    };
    let sync_status_color = if syncing { c.accent_blue } else { c.text_muted };
    let last_sync_label = last_sync
        .as_deref()
        .map_or_else(|| "Jamais".to_string(), format_sync_date);

    // Charger les donnees sync si pas deja fait
    let mut sync_loaded = use_signal(|| false);
    use_effect(move || {
        if *sync_loaded.read() {
            return;
        }
        sync_loaded.set(true);

        spawn(async move {
            load_sync_data(state, client).await;
        });
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px;",

            // Statut
            div {
                style: "display: flex; align-items: center; gap: 10px; margin-bottom: 4px;",
                span {
                    style: "display: inline-block; width: 10px; height: 10px; border-radius: 50%; background: {sync_status_color};",
                }
                span {
                    style: "font-size: 14px; color: {c.text_primary}; font-weight: 500;",
                    "{sync_status_text}"
                }
            }

            // Stats en grille
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",

                // Pairs enregistres
                div {
                    style: "display: flex; flex-direction: column; gap: 2px; padding: 8px 12px; background: {c.bg_main}; border-radius: 6px;",
                    span { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;", "Pairs enregistres" }
                    span { style: "font-size: 15px; color: {c.accent_blue}; font-weight: 600;", "{peers_label}" }
                }

                // En ligne
                div {
                    style: "display: flex; flex-direction: column; gap: 2px; padding: 8px 12px; background: {c.bg_main}; border-radius: 6px;",
                    span { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;", "En ligne" }
                    span { style: "font-size: 15px; color: {c.accent_green}; font-weight: 600;", "{online_label}" }
                }

                // Conflits
                div {
                    style: "display: flex; flex-direction: column; gap: 2px; padding: 8px 12px; background: {c.bg_main}; border-radius: 6px;",
                    span { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;", "Conflits en attente" }
                    span { style: "font-size: 15px; color: {c.accent_red}; font-weight: 600;", "{conflicts_label}" }
                }

                // Derniere sync
                div {
                    style: "display: flex; flex-direction: column; gap: 2px; padding: 8px 12px; background: {c.bg_main}; border-radius: 6px;",
                    span { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.3px;", "Derniere sync" }
                    span { style: "font-size: 12px; color: {c.text_primary}; font-weight: 500;", "{last_sync_label}" }
                }
            }

            // Bouton panel
            div {
                style: "display: flex; gap: 8px; margin-top: 4px;",

                button {
                    style: "padding: 8px 16px; background: {c.accent_blue}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; font-weight: 500;",
                    aria_label: "Gerer la synchronisation",
                    onclick: move |_| {
                        state.write().sync_panel_open = true;
                    },
                    "\u{1F504} Gerer les pairs et conflits"
                }
            }

            p {
                style: "font-size: 12px; color: {c.text_muted}; margin: 4px 0 0; line-height: 1.5;",
                "La synchronisation P2P permet de repliquer vos fichiers chiffres entre plusieurs instances COG. Chaque pair doit etre approuve manuellement pour garantir la securite."
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Fonctions utilitaires
// ════════════════════════════════════════════════════════════════════════════

/// Charge les donnees sync (pairs, conflits, statut) depuis le serveur.
async fn load_sync_data(
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) {
    let http = {
        let cl = client.read();
        cl.clone()
    };
    let Some(http) = http else { return };

    // Charger les pairs
    match http.list_peers().await {
        Ok(peers) => {
            let online_count = peers.iter().filter(|p| p.is_online).count() as u32;
            let mut s = state.write();
            s.sync_peers = peers;
            s.sync_status.peers_online = online_count;
        }
        Err(_) => {}
    }

    // Charger le statut
    match http.get_sync_status().await {
        Ok(status) => {
            let mut s = state.write();
            s.sync_status.syncing = status.is_syncing;
            s.sync_status.peers_online = status.peers_online;
            s.sync_status.pending_conflicts = status.pending_conflicts;
            s.sync_status.last_sync = status.last_sync;
        }
        Err(_) => {}
    }

    // Charger les conflits
    match http.list_conflicts().await {
        Ok(conflicts) => {
            let pending = conflicts.iter().filter(|c| c.status == "pending").count() as u32;
            let mut s = state.write();
            s.sync_conflicts = conflicts;
            s.sync_status.pending_conflicts = pending;
        }
        Err(_) => {}
    }
}

/// Resout un conflit et met a jour l'etat local.
async fn resolve_conflict_action(
    conflict_id: String,
    resolution: &str,
    mut state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) {
    let http = {
        let cl = client.read();
        cl.clone()
    };
    let Some(http) = http else { return };

    let resolved_status = if resolution == "keep_local" {
        "resolved_local"
    } else {
        "resolved_remote"
    };

    match http.resolve_conflict(&conflict_id, resolution).await {
        Ok(()) => {
            update_conflict_status(&mut state, &conflict_id, resolved_status);
        }
        Err(_) => {
            // Fallback : resoudre localement meme si le serveur est indisponible
            update_conflict_status(&mut state, &conflict_id, resolved_status);
        }
    }
}

/// Met a jour le statut d'un conflit dans l'etat local.
fn update_conflict_status(state: &mut Signal<MiyuCloudState>, conflict_id: &str, new_status: &str) {
    let mut conflicts = state.read().sync_conflicts.clone();
    if let Some(c) = conflicts.iter_mut().find(|c| c.id == conflict_id) {
        c.status = new_status.to_string();
    }
    let pending = conflicts.iter().filter(|c| c.status == "pending").count() as u32;
    state.write().sync_conflicts = conflicts;
    state.write().sync_status.pending_conflicts = pending;
}

/// Formate une date ISO 8601 pour l'affichage dans la sync.
fn format_sync_date(iso: &str) -> String {
    if iso.len() >= 16 {
        let date_part = &iso[..10];
        let time_part = &iso[11..16];
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{} {time_part}", parts[2], parts[1], parts[0]);
        }
    }
    iso.to_string()
}

