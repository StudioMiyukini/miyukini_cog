//! Service Lord of the Castle — intégré dans le body de Miyukini Central.
//!
//! Point d'accès utilisateur unique : Miyukini Central (CANON-CENTRAL).
//! Le jeu s'exécute dans le body de Central via `show_into`.
//! Sauvegarde/chargement liés au profil Central (central_profile_saves + profile_service_refs).

use crate::auth::CentralAuthDb;
use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use lord_of_the_castle::LordOfTheCastleApp;
use std::sync::Arc;

const SERVICE_KEY: &str = "lord_of_the_castle";
const SLOT: i64 = 0;

/// Service Lord of the Castle (Miyukini Survivor) : wrapper autour de LordOfTheCastleApp pour exécution dans le body de Central.
pub struct LordOfTheCastleService {
    app: LordOfTheCastleApp,
}

impl LordOfTheCastleService {
    /// Crée une instance (utilise LordOfTheCastleApp::new_embedded pour intégration Central).
    pub fn new() -> Self {
        Self {
            app: LordOfTheCastleApp::new_embedded(),
        }
    }
}

impl Default for LordOfTheCastleService {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceUi for LordOfTheCastleService {
    fn id(&self) -> ServiceId {
        ServiceId::LordOfTheCastle
    }
    fn title(&self) -> &'static str {
        "Lord of the Castle"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        self.app.show_into(ui);
    }
    fn set_lotc_save_load(
        &mut self,
        auth_db: Option<Arc<CentralAuthDb>>,
        profile_id: Option<&str>,
    ) {
        match (auth_db, profile_id) {
            (Some(db), Some(pid)) => {
                let db_save = Arc::clone(&db);
                let pid_save = pid.to_string();
                let db_load = Arc::clone(&db);
                let pid_load = pid.to_string();
                self.app.set_save_load_callbacks(
                    move |bytes| {
                        if let Some(ref_id) = db_save
                            .get_profile_service_ref(&pid_save, SERVICE_KEY)
                            .map_err(|e| e.0)?
                        {
                            db_save.update_profile_save(&ref_id, bytes).map_err(|e| e.0)?;
                        } else {
                            let id = db_save
                                .insert_profile_save(&pid_save, SERVICE_KEY, SLOT, bytes)
                                .map_err(|e| e.0)?;
                            db_save
                                .set_profile_service_ref(&pid_save, SERVICE_KEY, Some(&id))
                                .map_err(|e| e.0)?;
                        }
                        Ok(())
                    },
                    move || {
                        let ref_id = db_load
                            .get_profile_service_ref(&pid_load, SERVICE_KEY)
                            .map_err(|e| e.0)?
                            .ok_or_else(|| "Aucune sauvegarde.".to_string())?;
                        let save = db_load
                            .get_profile_save(&ref_id)
                            .map_err(|e| e.0)?
                            .ok_or_else(|| "Sauvegarde introuvable.".to_string())?;
                        Ok(save.data)
                    },
                );
            }
            _ => {
                self.app.set_save_load_callbacks(
                    |_| Err("Connectez-vous pour sauvegarder.".into()),
                    || Err("Connectez-vous pour charger.".into()),
                );
            }
        }
    }
}
