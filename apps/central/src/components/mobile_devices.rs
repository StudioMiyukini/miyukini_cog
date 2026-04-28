//! Composant UI pour gérer les devices mobiles appairés au COG Host.
//!
//! Affiche la liste des devices en attente d'approbation et ceux déjà appairés.
//! Permet d'approuver, refuser ou révoquer un device.

use crate::remote;
use crate::state::AppContext;
use dioxus::prelude::*;

/// Panneau de gestion des devices mobiles.
#[component]
pub fn MobileDevicesPanel() -> Element {
    let ctx = use_context::<AppContext>();
    let remote_enabled = ctx.remote_state.read().enabled;

    // Rafraîchir la liste toutes les 2 secondes pour voir les nouveaux devices
    let mut pending = use_signal(Vec::new);
    let mut paired = use_signal(Vec::new);

    use_effect(move || {
        let mut pending_sig = pending;
        let mut paired_sig = paired;
        spawn(async move {
            loop {
                if let Some(mgr) = remote::pairing_manager() {
                    pending_sig.set(mgr.pending_devices());
                    paired_sig.set(mgr.paired_devices());
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });
    });

    if !remote_enabled {
        return rsx! {
            div {
                style: "padding: 20px; color: #888; text-align: center;",
                "Activez le Remote pour gérer les devices mobiles"
            }
        };
    }

    rsx! {
        div {
            style: "padding: 20px; display: flex; flex-direction: column; gap: 20px;",

            h2 {
                style: "font-size: 22px; font-weight: bold; margin-bottom: 8px;",
                "Devices Mobiles"
            }

            // Devices en attente d'approbation
            if !pending.read().is_empty() {
                div {
                    style: "padding: 16px; background: #2a1a1a; border: 1px solid #ff6b6b; border-radius: 12px;",

                    h3 {
                        style: "font-size: 16px; color: #ff6b6b; margin-bottom: 12px;",
                        "En attente d'approbation ({pending.read().len()})"
                    }

                    for device in pending.read().iter() {
                        PendingDeviceRow {
                            key: "{device.device_id}",
                            device_id: device.device_id.clone(),
                            name: device.name.clone(),
                            requested_at: device.requested_at,
                        }
                    }
                }
            }

            // Devices appairés
            div {
                style: "padding: 16px; background: #1a2a1a; border: 1px solid #22c55e; border-radius: 12px;",

                h3 {
                    style: "font-size: 16px; color: #22c55e; margin-bottom: 12px;",
                    "Appairés ({paired.read().len()})"
                }

                if paired.read().is_empty() {
                    p {
                        style: "color: #666; font-size: 14px;",
                        "Aucun device appairé. Lancez Central Mobile sur votre téléphone."
                    }
                } else {
                    for device in paired.read().iter() {
                        PairedDeviceRow {
                            key: "{device.device_id}",
                            device_id: device.device_id.clone(),
                            name: device.name.clone(),
                            mode: device.mode.clone(),
                            last_seen: device.last_seen,
                        }
                    }
                }
            }

            // Aide
            div {
                style: "padding: 12px; background: #1a1a2e; border-radius: 8px; color: #888; font-size: 13px;",

                p { "Pour appairer un téléphone :" }
                ol { style: "margin-left: 20px; margin-top: 8px;",
                    li { "Lancer Central Mobile sur votre téléphone (même WiFi)" }
                    li { "Choisir 'Rechercher sur le WiFi' dans l'app mobile" }
                    li { "Approuver la demande ici" }
                }
            }
        }
    }
}

#[component]
fn PendingDeviceRow(device_id: String, name: String, requested_at: u64) -> Element {
    let id_for_approve = device_id.clone();
    let id_for_reject = device_id.clone();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px; background: #1a0a0a; border-radius: 8px; margin-bottom: 8px;",

            div {
                style: "flex: 1;",
                div { style: "font-weight: bold; font-size: 14px;", "{name}" }
                div { style: "font-size: 11px; color: #666; font-family: monospace;", "{device_id}" }
            }

            button {
                style: "padding: 8px 16px; background: #22c55e; border: none; border-radius: 6px; color: white; font-weight: bold; cursor: pointer;",
                onclick: move |_| {
                    if let Some(mgr) = remote::pairing_manager() {
                        let ok = mgr.approve_device(&id_for_approve);
                        tracing::info!("Device {} approuvé: {}", id_for_approve, ok);
                    }
                },
                "Approuver"
            }

            button {
                style: "padding: 8px 16px; background: transparent; border: 1px solid #666; border-radius: 6px; color: #ccc; cursor: pointer;",
                onclick: move |_| {
                    if let Some(mgr) = remote::pairing_manager() {
                        let ok = mgr.reject_device(&id_for_reject);
                        tracing::info!("Device {} refusé: {}", id_for_reject, ok);
                    }
                },
                "Refuser"
            }
        }
    }
}

#[component]
fn PairedDeviceRow(device_id: String, name: String, mode: String, last_seen: u64) -> Element {
    let id_for_revoke = device_id.clone();

    // Affichage du temps relatif depuis last_seen
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let elapsed = now.saturating_sub(last_seen);
    let last_seen_str = if elapsed < 60 {
        "il y a quelques secondes".to_string()
    } else if elapsed < 3600 {
        format!("il y a {} min", elapsed / 60)
    } else if elapsed < 86400 {
        format!("il y a {} h", elapsed / 3600)
    } else {
        format!("il y a {} j", elapsed / 86400)
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px; background: #0a1a0a; border-radius: 8px; margin-bottom: 8px;",

            div {
                style: "flex: 1;",
                div { style: "font-weight: bold; font-size: 14px;", "{name}" }
                div {
                    style: "font-size: 12px; color: #888;",
                    "{mode} • {last_seen_str}"
                }
            }

            button {
                style: "padding: 6px 12px; background: transparent; border: 1px solid #ef4444; border-radius: 6px; color: #ef4444; font-size: 12px; cursor: pointer;",
                onclick: move |_| {
                    if let Some(mgr) = remote::pairing_manager() {
                        let ok = mgr.revoke_device(&id_for_revoke);
                        tracing::info!("Device {} révoqué: {}", id_for_revoke, ok);
                    }
                },
                "Révoquer"
            }
        }
    }
}
