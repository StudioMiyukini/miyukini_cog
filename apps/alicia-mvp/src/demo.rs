//! Service demo, devices factices, generation de reponses templates,
//! lancement d'applications, et reconnaissance faciale.

use miyualicia::config::AliciaConfig;
use miyualicia::intent::Intent;
use miyualicia::AliciaService;
use miyualicia_devices::{
    Device, DeviceCapabilities, DeviceConfig, DeviceProtocol, DeviceState, DeviceType,
};

// ---------------------------------------------------------------------------
// Demo service + devices
// ---------------------------------------------------------------------------

pub async fn build_demo_service() -> AliciaService {
    let config = AliciaConfig {
        mqtt: None,
        ..Default::default()
    };
    let service = AliciaService::new(config);
    register_demo_devices(&service).await;
    service
}

pub async fn register_demo_devices(service: &AliciaService) {
    let demo_devices = [
        ("salon", DeviceType::Light, "Lampe salon"),
        ("salon", DeviceType::Shutter, "Volet salon"),
        (
            "chambre-parentale",
            DeviceType::Light,
            "Lampe chambre parentale",
        ),
        (
            "chambre-theresa",
            DeviceType::Light,
            "Lampe chambre Theresa",
        ),
        (
            "chambre-eleanore",
            DeviceType::Light,
            "Lampe chambre Eleanore",
        ),
        ("salon", DeviceType::Thermostat, "Thermostat salon"),
        ("salon", DeviceType::Outlet, "Prise salon"),
    ];

    for (room, dtype, name) in demo_devices {
        let id = uuid::Uuid::new_v4();
        let device = Device {
            id,
            room_id: room.to_string(),
            device_type: dtype,
            name: name.to_string(),
            protocol: DeviceProtocol::HttpLocal,
            address: format!(
                "http://127.0.0.1:9999/{room}/{}",
                format!("{dtype:?}").to_lowercase()
            ),
            capabilities: DeviceCapabilities::default(),
            config: DeviceConfig::default(),
            active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let state = DeviceState::unknown(id);
        let _ = service.register_device(device, state).await;
    }
    println!("  [Demo] {} devices enregistres", demo_devices.len());
}

// ---------------------------------------------------------------------------
// Response generation (template statique)
// ---------------------------------------------------------------------------

pub fn generate_response_text(intent: &Intent, transcript: &str) -> String {
    match intent {
        Intent::ControlDevice {
            device_type,
            room_id,
            action,
            value,
        } => {
            let room_str = room_id
                .as_deref()
                .map(|r| format!(" dans {r}"))
                .unwrap_or_default();
            let value_str = value
                .as_ref()
                .map(|v| format!(" a {v}"))
                .unwrap_or_default();
            let action_fr = match action.as_str() {
                "on" => "allume",
                "off" => "eteint",
                "open" => "ouvre",
                "close" => "ferme",
                "lock" => "verrouille",
                "unlock" => "deverrouille",
                "set_brightness" => "regle la luminosite de",
                "set_temperature" => "regle la temperature du",
                _ => action.as_str(),
            };
            format!("OK, je {action_fr} {device_type}{room_str}{value_str}.")
        }
        Intent::ActivateRoutine { routine_name } => {
            format!("OK, j'active la routine \"{routine_name}\".")
        }
        Intent::QueryState { target, property } => {
            let prop = property.as_deref().unwrap_or("etat");
            format!("Voici le {prop} de {target}.")
        }
        Intent::QueryWeather { location, horizon } => {
            let loc = location.as_deref().unwrap_or("ici");
            let hor = horizon.as_deref().unwrap_or("maintenant");
            format!("La meteo a {loc} {hor} : information non disponible en mode demo.")
        }
        Intent::Help { topic } => {
            let t = topic.as_deref().unwrap_or("general");
            format!("Aide sur {t} : je peux allumer/eteindre les lumieres, regler le thermostat, ouvrir/fermer les volets.")
        }
        Intent::Unknown { transcript: ref t } if t.starts_with("__launch_app:") => {
            let app = t.strip_prefix("__launch_app:").unwrap_or("l'application");
            format!("OK, je lance {app}.")
        }
        Intent::Unknown { transcript: ref t } if t == "__identify_face" => {
            "Je regarde qui est devant la camera.".to_string()
        }
        Intent::Unknown { transcript: ref t } if t.starts_with("__register_face:") => {
            let name = t.strip_prefix("__register_face:").unwrap_or("toi");
            format!("OK, j'enregistre ton visage sous le nom {name}.")
        }
        Intent::Unknown { .. } => {
            format!("Je n'ai pas compris \"{transcript}\". Essaie : allume la lumiere, ferme les volets, bonne nuit...")
        }
    }
}

// ---------------------------------------------------------------------------
// Lancement d'applications
// ---------------------------------------------------------------------------

pub fn launch_app(app_name: &str) -> anyhow::Result<()> {
    use std::process::Command;

    let lower = app_name.to_lowercase();
    let cmd = match lower.as_str() {
        "chrome" | "google chrome" | "google" => "chrome",
        "firefox" => "firefox",
        "notepad" | "bloc-notes" | "bloc notes" => "notepad",
        "explorer" | "explorateur" => "explorer",
        "spotify" => "spotify",
        "discord" => "discord",
        "steam" => "steam",
        "vscode" | "code" | "visual studio code" => "code",
        // S-03: whitelist-only, pas de fallback arbitraire
        _ => {
            anyhow::bail!(
                "Application '{app_name}' non reconnue. Apps disponibles: \
                 chrome, firefox, notepad, explorer, spotify, discord, steam, vscode"
            );
        }
    };

    println!("  [Launch] Commande : start \"\" \"{cmd}\"");
    Command::new("cmd")
        .args(["/C", "start", "", cmd])
        .spawn()?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Reconnaissance faciale
// ---------------------------------------------------------------------------

pub async fn call_face_identify(
    client: &reqwest::Client,
    base_url: &str,
) -> anyhow::Result<String> {
    let url = format!("{base_url}/api/face/identify");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({}))
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Face HTTP error: {body}");
    }

    let json: serde_json::Value = resp.json().await?;
    let count = json["count"].as_u64().unwrap_or(0);
    let total_ms = json["total_ms"].as_u64().unwrap_or(0);

    if count == 0 {
        return Ok(format!("Aucun visage detecte ({total_ms}ms)"));
    }

    let faces = json["faces"].as_array();
    let mut names = Vec::new();
    if let Some(faces) = faces {
        for face in faces {
            let name = face["name"].as_str().unwrap_or("inconnu");
            let conf = face["confidence"].as_f64().unwrap_or(0.0);
            names.push(format!("{name} ({:.0}%)", conf * 100.0));
        }
    }

    Ok(format!(
        "{count} visage(s) : {} ({total_ms}ms)",
        names.join(", ")
    ))
}

pub async fn call_face_register(
    client: &reqwest::Client,
    base_url: &str,
    name: &str,
) -> anyhow::Result<String> {
    let url = format!("{base_url}/api/face/register");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({ "name": name }))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Face register error: {body}");
    }

    let json: serde_json::Value = resp.json().await?;
    let photos = json["photos"].as_u64().unwrap_or(0);
    Ok(format!("Visage de '{name}' enregistre ({photos} photo(s))"))
}
