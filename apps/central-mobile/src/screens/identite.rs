//! @id     central_mobile_screens_identite
//! @do     afficher_regeneration_identite
//! @role   ui
//! @layer  app
//! @human  central-mobile — écran de régénération de l'identité de l'appareil (I6).

//! Écran « Régénérer l'identité de l'appareil ».
//!
//! 1. Préparer : nouvelle clé générée et stockée à part (rien ne change encore).
//! 2. Lier dans MiyuCloud :
//!    - ancienne clé disponible : rotation signée par l'ancienne clé, envoyée
//!      directement (MiyuCloud révoque l'ancienne) ;
//!    - sinon : la demande de liaison est affichée, à coller depuis un appareil
//!      de confiance (profil MiyuCloud → « Lier un nouvel appareil »).
//! 3. Confirmer : la nouvelle clé remplace l'ancienne ; la connexion au COG
//!    est oubliée et un nouvel appairage (approuvé sur Central Desktop, où
//!    l'ancienne clé est à révoquer) est demandé.
//!
//! La logique (stockage, signatures, requêtes) est dans [`crate::identite`].

use dioxus::prelude::*;

use crate::identite::{self as id, ModeLiaison, StockageIdentite};
use crate::state::{AppScreen, AppState};

fn maintenant() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

const BOUTON: &str = "width: 100%; max-width: 320px; padding: 12px; background: #7c3aed; border: none; border-radius: 8px; color: white; font-size: 15px; font-weight: bold; cursor: pointer;";
const BOUTON_SECONDAIRE: &str = "width: 100%; max-width: 320px; padding: 10px; background: transparent; border: 1px solid #3d3d55; border-radius: 8px; color: #bbb; font-size: 14px; cursor: pointer;";

/// Écran de régénération de l'identité.
#[component]
pub fn IdentiteScreen() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let mut mode = use_signal(|| None::<ModeLiaison>);
    let mut message = use_signal(String::new);
    let mut demande = use_signal(String::new);
    let mut url = use_signal(String::new);
    let mut occupe = use_signal(|| false);

    let mode_courant = *mode.read();
    let texte_mode = match mode_courant {
        None => "Aucune nouvelle clé préparée.",
        Some(ModeLiaison::PreuveAncienne) => {
            "Nouvelle clé prête. L'ancienne clé peut signer sa remplaçante."
        }
        Some(ModeLiaison::NouvelleLiaison) => {
            "Nouvelle clé prête. Ancienne clé indisponible : liez la nouvelle depuis un appareil de confiance."
        }
    };

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; align-items: center; padding: 24px; gap: 12px; overflow-y: auto;",

            h1 { style: "font-size: 22px; color: #7c3aed; margin: 0;", "Identité de l'appareil" }
            p { style: "font-size: 13px; color: #aaa; max-width: 320px; text-align: center;", "{texte_mode}" }

            button {
                style: BOUTON,
                disabled: *occupe.read(),
                onclick: move |_| {
                    match id::preparer(&StockageIdentite::plateforme()) {
                        Ok(r) => {
                            mode.set(Some(r.mode()));
                            message.set(format!("Nouvelle clé : {}…", &hex::encode(r.cle_nouvelle())[..16]));
                        }
                        Err(e) => message.set(e),
                    }
                },
                "1. Préparer une nouvelle clé"
            }

            input {
                style: "width: 100%; max-width: 320px; padding: 10px; background: #1a1a2e; border: 1px solid #3d3d55; border-radius: 8px; color: #e0e0e0;",
                placeholder: "https://miyucloud.exemple",
                value: "{url}",
                oninput: move |e| url.set(e.value()),
            }

            button {
                style: BOUTON,
                disabled: *occupe.read() || mode_courant.is_none(),
                onclick: move |_| {
                    let base = url.read().clone();
                    occupe.set(true);
                    spawn(async move {
                        let resultat = lier_miyucloud(&base).await;
                        match resultat {
                            Ok(Some(texte)) => {
                                demande.set(texte);
                                message.set("Collez cette demande dans MiyuCloud (profil → Lier un nouvel appareil) depuis un appareil de confiance, puis retirez l'ancien.".into());
                            }
                            Ok(None) => message.set("Nouvelle clé liée à MiyuCloud ; l'ancienne y est révoquée.".into()),
                            Err(e) => message.set(e),
                        }
                        occupe.set(false);
                    });
                },
                "2. Lier dans MiyuCloud"
            }

            if !demande.read().is_empty() {
                textarea {
                    style: "width: 100%; max-width: 320px; height: 96px; font-family: monospace; font-size: 11px; background: #11111f; color: #e0e0e0; border: 1px solid #3d3d55; border-radius: 8px;",
                    readonly: true,
                    value: "{demande}",
                }
            }

            button {
                style: BOUTON,
                disabled: *occupe.read() || mode_courant.is_none(),
                onclick: move |_| {
                    match id::confirmer(&StockageIdentite::plateforme()) {
                        Ok(()) => {
                            // Le COG ne connaît pas encore la nouvelle clé : nouvel
                            // appairage (Central Desktop), l'ancienne y est à révoquer.
                            let _ = crate::platform::clear_connection();
                            state.write().saved_connection = None;
                            state.write().screen = AppScreen::Pairing;
                        }
                        Err(e) => message.set(e),
                    }
                },
                "3. Confirmer et ré-appairer"
            }

            button {
                style: BOUTON_SECONDAIRE,
                disabled: *occupe.read(),
                onclick: move |_| {
                    match id::abandonner(&StockageIdentite::plateforme()) {
                        Ok(()) => {
                            mode.set(None);
                            demande.set(String::new());
                            state.write().screen = if state.read().saved_connection.is_some() {
                                AppScreen::Connexion
                            } else {
                                AppScreen::Pairing
                            };
                        }
                        Err(e) => message.set(e),
                    }
                },
                "Annuler"
            }

            if !message.read().is_empty() {
                p { style: "font-size: 13px; color: #e0e0e0; max-width: 320px; text-align: center;", "{message}" }
            }
        }
    }
}

/// Liaison MiyuCloud : rotation directe si l'ancienne clé est lisible (rend
/// `None`), sinon la demande à coller depuis un appareil de confiance.
async fn lier_miyucloud(base: &str) -> Result<Option<String>, String> {
    let regeneration = id::preparer(&StockageIdentite::plateforme())?;
    let cog = id::contexte_miyucloud(base).await?;
    let t = maintenant();
    match regeneration.mode() {
        ModeLiaison::PreuveAncienne => {
            let requete = regeneration.requete_rotation(&cog, t)?;
            id::envoyer_rotation(base, &requete).await?;
            Ok(None)
        }
        ModeLiaison::NouvelleLiaison => Ok(Some(regeneration.demande_nouvelle_liaison(&cog, t))),
    }
}
