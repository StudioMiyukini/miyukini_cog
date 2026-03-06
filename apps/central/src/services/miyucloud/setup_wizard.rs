use dioxus::prelude::*;

use super::client::MiyuCloudClient;
use super::state::{MiyuCloudState, PermissionLevel};
use crate::services::miyucloud_settings::{
    load_miyucloud_config, save_miyucloud_config, MiyuCloudServiceConfig, SharePermissionPreset,
};
use crate::state::use_app_state;

#[component]
pub fn FirstLaunchSetupWizard(
    state: Signal<MiyuCloudState>,
    client: Signal<Option<MiyuCloudClient>>,
) -> Element {
    if !state.read().setup_wizard_open {
        return rsx! {};
    }

    let c = use_app_state().read().current_theme.palette();
    let initial = load_miyucloud_config();

    let mut step = use_signal(|| 0_usize);
    let mut saving = use_signal(|| false);
    let mut local_error: Signal<Option<String>> = use_signal(|| None);

    let mut storage_path = use_signal(|| initial.storage_path.clone());
    let mut quota_gb = use_signal(|| quota_bytes_to_gb_string(initial.quota_bytes));

    let mut public_links_enabled = use_signal(|| initial.share_policy.public_links_enabled);
    let mut internal_sharing_enabled =
        use_signal(|| initial.share_policy.internal_sharing_enabled);
    let mut default_permission = use_signal(|| initial.share_policy.default_permission);

    let mut ssh_enabled = use_signal(|| initial.ssh.enabled);
    let mut ssh_host = use_signal(|| initial.ssh.host.clone());
    let mut ssh_port = use_signal(|| initial.ssh.port.to_string());
    let mut ssh_username = use_signal(|| initial.ssh.username.clone());
    let mut ssh_root_path = use_signal(|| initial.ssh.root_path.clone());
    let mut ssh_private_key_path = use_signal(|| initial.ssh.private_key_path.clone());
    let mut ssh_keepalive = use_signal(|| initial.ssh.keepalive);

    let current_step = *step.read();
    let is_last_step = current_step >= 3;
    let is_saving = *saving.read();
    let save_button_opacity = if is_saving { "0.7" } else { "1" };

    let title = match current_step {
        0 => "1. Stockage",
        1 => "2. Partage et permissions",
        2 => "3. Connexion SSH",
        _ => "4. Validation",
    };

    let mut save_setup = move || {
        let parsed_quota_gb = quota_gb.read().trim().parse::<u64>().unwrap_or(0);
        if parsed_quota_gb == 0 {
            local_error.set(Some(
                "Indique une limite de stockage valide en Go.".to_string(),
            ));
            return;
        }

        let trimmed_storage = storage_path.read().trim().to_string();
        if trimmed_storage.is_empty() {
            local_error.set(Some(
                "Le dossier racine MiyuCloud ne peut pas etre vide.".to_string(),
            ));
            return;
        }

        let ssh_is_enabled = *ssh_enabled.read();
        let ssh_host_value = ssh_host.read().trim().to_string();
        let ssh_username_value = ssh_username.read().trim().to_string();
        let ssh_port_value = ssh_port.read().trim().parse::<u16>().unwrap_or(0);
        if ssh_is_enabled && (ssh_host_value.is_empty() || ssh_username_value.is_empty()) {
            local_error.set(Some(
                "Renseigne au minimum l'hote et l'utilisateur SSH.".to_string(),
            ));
            return;
        }
        if ssh_is_enabled && ssh_port_value == 0 {
            local_error.set(Some("Le port SSH doit etre valide.".to_string()));
            return;
        }

        local_error.set(None);
        saving.set(true);

        let mut config: MiyuCloudServiceConfig = load_miyucloud_config();
        let quota_bytes = parsed_quota_gb.saturating_mul(1024 * 1024 * 1024);
        config.storage_path = trimmed_storage.clone();
        config.quota_bytes = quota_bytes;
        config.first_launch_completed = true;
        config.share_policy.public_links_enabled = *public_links_enabled.read();
        config.share_policy.internal_sharing_enabled = *internal_sharing_enabled.read();
        config.share_policy.default_permission = *default_permission.read();
        config.ssh.enabled = ssh_is_enabled;
        config.ssh.host = ssh_host_value.clone();
        config.ssh.port = if ssh_is_enabled { ssh_port_value } else { 22 };
        config.ssh.username = ssh_username_value.clone();
        config.ssh.root_path = ssh_root_path.read().trim().to_string();
        config.ssh.private_key_path = ssh_private_key_path.read().trim().to_string();
        config.ssh.keepalive = *ssh_keepalive.read();

        match save_miyucloud_config(&config) {
            Ok(()) => {
                {
                    let mut writer = state.write();
                    writer.configured_quota_bytes = quota_bytes;
                    writer.first_launch_completed = true;
                    writer.setup_wizard_open = false;
                    writer.storage_path = config.storage_path.clone();
                    writer.public_share_links_enabled = config.share_policy.public_links_enabled;
                    writer.internal_sharing_enabled =
                        config.share_policy.internal_sharing_enabled;
                    writer.default_share_permission =
                        preset_to_permission_level(config.share_policy.default_permission);
                    writer.ssh_enabled = config.ssh.enabled;
                    writer.ssh_host = config.ssh.host.clone();
                    writer.ssh_port = config.ssh.port;
                    writer.ssh_username = config.ssh.username.clone();
                    writer.ssh_root_path = config.ssh.root_path.clone();
                    writer.ssh_private_key_path = config.ssh.private_key_path.clone();
                    writer.ssh_keepalive = config.ssh.keepalive;
                    writer.error_message = Some(
                        "Setup MiyuCloud enregistre. Relance le service pour reappliquer stockage, partage et profil SSH."
                            .to_string(),
                    );
                }

                spawn(async move {
                    let http = {
                        let c = client.read();
                        c.clone()
                    };
                    let Some(http) = http else { return };
                    let _ = http.set_quota(quota_bytes).await;
                    if let Ok(status) = http.get_onboarding_status().await {
                        state.write().onboarding_status = Some(status);
                    }
                    if let Ok(health) = http.get_health_status().await {
                        state.write().health_status = Some(health);
                    }
                });
            }
            Err(err) => local_error.set(Some(err)),
        }

        saving.set(false);
    };

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 1400; display: flex; align-items: center; justify-content: center; background: rgba(3, 8, 20, 0.78);",
            div {
                style: "width: min(760px, 94vw); max-height: 92vh; overflow: auto; background: {c.bg_card}; border: 1px solid {c.border}; border-radius: 14px; box-shadow: 0 24px 72px rgba(0,0,0,0.45);",
                div {
                    style: "padding: 22px 24px 16px; border-bottom: 1px solid {c.border};",
                    p { style: "margin: 0 0 6px; color: {c.accent_blue}; font-size: 12px; text-transform: uppercase; letter-spacing: 0.08em;", "Premier lancement MiyuCloud" }
                    h2 { style: "margin: 0; color: {c.text_white}; font-size: 24px;", "Assistant de configuration initiale" }
                    p { style: "margin: 10px 0 0; color: {c.text_secondary}; font-size: 13px; line-height: 1.6;", "Configure la racine de stockage, la limite d'espace, les regles de partage et le profil SSH persistant avant d'utiliser MiyuCloud." }
                }

                div {
                    style: "padding: 16px 24px 0; display: grid; grid-template-columns: repeat(4, 1fr); gap: 8px;",
                    for (index, label) in ["Stockage", "Partage", "SSH", "Validation"].iter().enumerate() {
                        {
                            let active = index == current_step;
                            let done = index < current_step;
                            let bg = if active {
                                c.accent_blue
                            } else if done {
                                c.accent_green
                            } else {
                                c.bg_hover
                            };
                            let fg = if active || done {
                                c.text_white
                            } else {
                                c.text_secondary
                            };
                            rsx! {
                                div {
                                    key: "{index}",
                                    style: "padding: 10px 12px; border-radius: 8px; background: {bg}; color: {fg}; font-size: 12px; font-weight: 600; text-align: center;",
                                    "{index + 1}. {label}"
                                }
                            }
                        }
                    }
                }

                div {
                    style: "padding: 20px 24px; display: flex; flex-direction: column; gap: 16px;",
                    h3 { style: "margin: 0; color: {c.text_white}; font-size: 18px;", "{title}" }

                    if let Some(err) = local_error.read().clone() {
                        div {
                            style: "padding: 10px 12px; background: {c.accent_red}20; border-left: 3px solid {c.accent_red}; border-radius: 6px; color: {c.accent_red}; font-size: 13px;",
                            "{err}"
                        }
                    }

                    if current_step == 0 {
                        div {
                            style: "display: grid; grid-template-columns: 1fr 180px; gap: 14px;",
                            WizardField {
                                label: "Dossier racine de stockage",
                                hint: "Ce dossier sera utilise comme racine MiyuCloud sur le COG.",
                                input: rsx! {
                                    input {
                                        value: "{storage_path}",
                                        oninput: move |evt| storage_path.set(evt.value()),
                                        style: "padding: 11px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 8px; font-size: 13px;",
                                        placeholder: "C:\\Data\\MiyuCloud",
                                    }
                                }
                            }
                            WizardField {
                                label: "Limite de stockage",
                                hint: "Par defaut: 10 Go. La valeur reste modifiable apres coup.",
                                input: rsx! {
                                    input {
                                        value: "{quota_gb}",
                                        oninput: move |evt| quota_gb.set(evt.value()),
                                        style: "padding: 11px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 8px; font-size: 13px;",
                                        placeholder: "10",
                                    }
                                }
                            }
                        }
                    }

                    if current_step == 1 {
                        div {
                            style: "display: flex; flex-direction: column; gap: 14px;",
                            ToggleCard {
                                label: "Liens publics",
                                hint: "Autorise la creation de liens web externes avec expiration, mot de passe et telechargement persistant.",
                                enabled: *public_links_enabled.read(),
                                ontoggle: move |_| {
                                    let next = {
                                        let current = public_links_enabled.read();
                                        !*current
                                    };
                                    public_links_enabled.set(next);
                                },
                            }
                            ToggleCard {
                                label: "Partage interne",
                                hint: "Autorise le partage profil/tribu a l'interieur du COG avec permissions persistantes.",
                                enabled: *internal_sharing_enabled.read(),
                                ontoggle: move |_| {
                                    let next = {
                                        let current = internal_sharing_enabled.read();
                                        !*current
                                    };
                                    internal_sharing_enabled.set(next);
                                },
                            }
                            div {
                                style: "display: flex; flex-direction: column; gap: 8px;",
                                span { style: "font-size: 13px; color: {c.text_primary}; font-weight: 600;", "Permission par defaut" }
                                div {
                                    style: "display: flex; gap: 10px;",
                                    ChoiceButton {
                                        active: *default_permission.read() == SharePermissionPreset::Read,
                                        label: "Lecture seule",
                                        onclick: move |_| default_permission.set(SharePermissionPreset::Read),
                                    }
                                    ChoiceButton {
                                        active: *default_permission.read() == SharePermissionPreset::ReadWrite,
                                        label: "Lecture et ecriture",
                                        onclick: move |_| default_permission.set(SharePermissionPreset::ReadWrite),
                                    }
                                }
                            }
                        }
                    }

                    if current_step == 2 {
                        div {
                            style: "display: flex; flex-direction: column; gap: 14px;",
                            ToggleCard {
                                label: "Profil SSH persistant",
                                hint: "Stocke un profil SSH reutilisable pour connexion rapide et partage vers une cible distante. Aucun mot de passe n'est stocke.",
                                enabled: *ssh_enabled.read(),
                                ontoggle: move |_| {
                                    let next = {
                                        let current = ssh_enabled.read();
                                        !*current
                                    };
                                    ssh_enabled.set(next);
                                },
                            }
                            if *ssh_enabled.read() {
                                div {
                                    style: "display: grid; grid-template-columns: 1.2fr 160px; gap: 14px;",
                                    WizardField {
                                        label: "Hote SSH",
                                        hint: "Nom DNS ou IP de la cible distante.",
                                        input: rsx! {
                                            input {
                                                value: "{ssh_host}",
                                                oninput: move |evt| ssh_host.set(evt.value()),
                                                style: "padding: 11px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 8px; font-size: 13px;",
                                                placeholder: "cloud.mondomaine.local",
                                            }
                                        }
                                    }
                                    WizardField {
                                        label: "Port",
                                        hint: "Port SSH persistant.",
                                        input: rsx! {
                                            input {
                                                value: "{ssh_port}",
                                                oninput: move |evt| ssh_port.set(evt.value()),
                                                style: "padding: 11px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 8px; font-size: 13px;",
                                                placeholder: "22",
                                            }
                                        }
                                    }
                                }
                                div {
                                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 14px;",
                                    WizardField {
                                        label: "Utilisateur",
                                        hint: "Compte SSH a reutiliser.",
                                        input: rsx! {
                                            input {
                                                value: "{ssh_username}",
                                                oninput: move |evt| ssh_username.set(evt.value()),
                                                style: "padding: 11px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 8px; font-size: 13px;",
                                                placeholder: "miyukini",
                                            }
                                        }
                                    }
                                    WizardField {
                                        label: "Racine distante",
                                        hint: "Dossier cible pour les echanges rapides.",
                                        input: rsx! {
                                            input {
                                                value: "{ssh_root_path}",
                                                oninput: move |evt| ssh_root_path.set(evt.value()),
                                                style: "padding: 11px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 8px; font-size: 13px;",
                                                placeholder: "/srv/miyucloud",
                                            }
                                        }
                                    }
                                }
                                WizardField {
                                    label: "Cle privee",
                                    hint: "Chemin local de cle privee. Laisse vide si l'agent SSH ou une autre methode gere l'authentification.",
                                    input: rsx! {
                                        input {
                                            value: "{ssh_private_key_path}",
                                            oninput: move |evt| ssh_private_key_path.set(evt.value()),
                                            style: "padding: 11px 12px; background: {c.bg_main}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 8px; font-size: 13px;",
                                            placeholder: "C:\\Users\\miyuk\\.ssh\\id_ed25519",
                                        }
                                    }
                                }
                                ToggleCard {
                                    label: "Keepalive SSH",
                                    hint: "Garde la session plus stable pour des echanges repetes ou longs.",
                                    enabled: *ssh_keepalive.read(),
                                    ontoggle: move |_| {
                                        let next = {
                                            let current = ssh_keepalive.read();
                                            !*current
                                        };
                                        ssh_keepalive.set(next);
                                    },
                                }
                            }
                        }
                    }

                    if current_step >= 3 {
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 14px;",
                            SummaryCard { label: "Stockage".to_string(), value: storage_path.read().clone() }
                            SummaryCard { label: "Quota".to_string(), value: format!("{} Go", quota_gb.read()) }
                            SummaryCard {
                                label: "Liens publics".to_string(),
                                value: if *public_links_enabled.read() { "Actifs".to_string() } else { "Desactives".to_string() }
                            }
                            SummaryCard {
                                label: "Partage interne".to_string(),
                                value: if *internal_sharing_enabled.read() { "Actif".to_string() } else { "Desactive".to_string() }
                            }
                            SummaryCard {
                                label: "Permission par defaut".to_string(),
                                value: default_permission.read().label().to_string()
                            }
                            SummaryCard {
                                label: "SSH".to_string(),
                                value: if *ssh_enabled.read() {
                                    format!(
                                        "{}@{}:{}",
                                        ssh_username.read(),
                                        ssh_host.read(),
                                        ssh_port.read()
                                    )
                                } else {
                                    "Desactive".to_string()
                                }
                            }
                        }
                    }
                }

                div {
                    style: "padding: 0 24px 24px; display: flex; align-items: center; justify-content: space-between; gap: 12px;",
                    p { style: "margin: 0; color: {c.text_muted}; font-size: 12px;", "Tu pourras rouvrir cet assistant plus tard depuis les reglages MiyuCloud." }
                    div {
                        style: "display: flex; gap: 10px;",
                        if current_step > 0 {
                            button {
                                style: "padding: 10px 14px; background: {c.bg_hover}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 8px; cursor: pointer;",
                                onclick: move |_| step.set(current_step.saturating_sub(1)),
                                "Retour"
                            }
                        }
                        button {
                            style: "padding: 10px 14px; background: {c.accent_blue}; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; opacity: {save_button_opacity};",
                            disabled: is_saving,
                            onclick: move |_| {
                                if is_last_step {
                                    save_setup();
                                } else {
                                    step.set(current_step + 1);
                                }
                            },
                            if is_last_step { "Enregistrer le setup" } else { "Suivant" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn WizardField(label: &'static str, hint: &'static str, input: Element) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        label {
            style: "display: flex; flex-direction: column; gap: 6px;",
            span { style: "font-size: 13px; color: {c.text_primary}; font-weight: 600;", "{label}" }
            span { style: "font-size: 12px; color: {c.text_muted};", "{hint}" }
            {input}
        }
    }
}

#[component]
fn ToggleCard(
    label: &'static str,
    hint: &'static str,
    enabled: bool,
    ontoggle: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if enabled { c.accent_green } else { c.bg_hover };
    let text = if enabled { "Active" } else { "Desactive" };

    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: space-between; gap: 12px; padding: 12px 14px; border: 1px solid {c.border}; border-radius: 10px; background: {c.bg_main};",
            div {
                style: "display: flex; flex-direction: column; gap: 4px;",
                span { style: "font-size: 13px; color: {c.text_primary}; font-weight: 600;", "{label}" }
                span { style: "font-size: 12px; color: {c.text_muted}; line-height: 1.5;", "{hint}" }
            }
            button {
                style: "padding: 8px 12px; background: {bg}; color: white; border: none; border-radius: 999px; cursor: pointer; font-size: 12px; font-weight: 700;",
                onclick: move |evt| ontoggle.call(evt),
                "{text}"
            }
        }
    }
}

#[component]
fn ChoiceButton(active: bool, label: &'static str, onclick: EventHandler<MouseEvent>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if active { c.accent_blue } else { c.bg_hover };
    let fg = if active { c.text_white } else { c.text_primary };

    rsx! {
        button {
            style: "padding: 10px 14px; background: {bg}; color: {fg}; border: 1px solid {c.border}; border-radius: 8px; cursor: pointer; font-size: 13px; font-weight: 600;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

#[component]
fn SummaryCard(label: String, value: String) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "padding: 12px 14px; border: 1px solid {c.border}; border-radius: 10px; background: {c.bg_main}; display: flex; flex-direction: column; gap: 4px;",
            span { style: "font-size: 11px; color: {c.text_muted}; text-transform: uppercase;", "{label}" }
            span { style: "font-size: 13px; color: {c.text_primary}; font-weight: 600; word-break: break-word;", "{value}" }
        }
    }
}

fn quota_bytes_to_gb_string(bytes: u64) -> String {
    let gib = bytes / (1024 * 1024 * 1024);
    gib.max(1).to_string()
}

fn preset_to_permission_level(preset: SharePermissionPreset) -> PermissionLevel {
    match preset {
        SharePermissionPreset::Read => PermissionLevel::Read,
        SharePermissionPreset::ReadWrite => PermissionLevel::ReadWrite,
    }
}
