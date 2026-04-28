//! Store centralisé — accès gouverné au CentralProfile.
//!
//! Store en mémoire (persistance réelle déléguée à KindMother côté produit).
//! Fournit des opérations CRUD par section sur le CentralProfile.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use crate::sections::CentralProfile;
use std::collections::HashMap;
use std::sync::Mutex;

static CENTRAL_STORE: std::sync::OnceLock<Mutex<HashMap<String, CentralProfile>>> =
    std::sync::OnceLock::new();

fn central_store() -> &'static Mutex<HashMap<String, CentralProfile>> {
    CENTRAL_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// tool.profile.central.get — Récupère le profil Central complet.
pub fn get(ctx: &GovernedContext, user_id: &str) -> Result<CentralProfile, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let guard = central_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("central store lock".into()))?;
    Ok(guard.get(user_id).cloned().unwrap_or_else(|| CentralProfile {
        user_id: user_id.to_string(),
        ..Default::default()
    }))
}

/// tool.profile.central.save — Sauvegarde le profil Central complet.
pub fn save(
    ctx: &GovernedContext,
    profile: &CentralProfile,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = central_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("central store lock".into()))?;
    guard.insert(profile.user_id.clone(), profile.clone());
    Ok(())
}

/// tool.profile.central.update_section — Met à jour une section via un closure.
///
/// Charge le profil, applique la modification, et sauvegarde.
pub fn update_with<F>(
    ctx: &GovernedContext,
    user_id: &str,
    updater: F,
) -> Result<(), MiyuprofileError>
where
    F: FnOnce(&mut CentralProfile),
{
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = central_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("central store lock".into()))?;
    let profile = guard
        .entry(user_id.to_string())
        .or_insert_with(|| CentralProfile {
            user_id: user_id.to_string(),
            ..Default::default()
        });
    updater(profile);
    Ok(())
}

/// tool.profile.central.update_section_json — Met à jour une section via JSON.
///
/// Pratique pour les appels dynamiques où la section est connue à l'exécution.
pub fn update_section_json(
    ctx: &GovernedContext,
    user_id: &str,
    section_key: &str,
    json_value: serde_json::Value,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = central_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("central store lock".into()))?;
    let profile = guard
        .entry(user_id.to_string())
        .or_insert_with(|| CentralProfile {
            user_id: user_id.to_string(),
            ..Default::default()
        });

    match section_key {
        "identity" => {
            profile.identity = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("identity: {e}")))?,
            );
        }
        "contacts" => {
            profile.contacts = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("contacts: {e}")))?,
            );
        }
        "documents" => {
            profile.documents = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("documents: {e}")))?,
            );
        }
        "health" => {
            profile.health = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("health: {e}")))?,
            );
        }
        "professional" => {
            profile.professional = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("professional: {e}")))?,
            );
        }
        "enterprises" => {
            profile.enterprises = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("enterprises: {e}")))?,
            );
        }
        "contracts" => {
            profile.contracts = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("contracts: {e}")))?,
            );
        }
        "finance" => {
            profile.finance = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("finance: {e}")))?,
            );
        }
        "credentials" => {
            profile.credentials = Some(
                serde_json::from_value(json_value)
                    .map_err(|e| MiyuprofileError::InvalidInput(format!("credentials: {e}")))?,
            );
        }
        other => {
            return Err(MiyuprofileError::InvalidInput(format!(
                "unknown section: {other}"
            )));
        }
    }
    Ok(())
}

/// tool.profile.central.get_section_json — Lit une section en JSON.
pub fn get_section_json(
    ctx: &GovernedContext,
    user_id: &str,
    section_key: &str,
) -> Result<Option<serde_json::Value>, MiyuprofileError> {
    let profile = get(ctx, user_id)?;
    let value = match section_key {
        "identity" => profile.identity.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        "contacts" => profile.contacts.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        "documents" => profile.documents.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        "health" => profile.health.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        "professional" => profile.professional.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        "enterprises" => profile.enterprises.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        "contracts" => profile.contracts.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        "finance" => profile.finance.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        "credentials" => profile.credentials.as_ref().and_then(|v| serde_json::to_value(v).ok()),
        other => {
            return Err(MiyuprofileError::InvalidInput(format!(
                "unknown section: {other}"
            )));
        }
    };
    Ok(value)
}

/// tool.profile.central.delete — Supprime le profil Central complet.
pub fn delete(ctx: &GovernedContext, user_id: &str) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = central_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("central store lock".into()))?;
    guard.remove(user_id);
    Ok(())
}
