//! Moteur de décision Miou — détermine quelle bulle afficher.

use super::context::BotContext;
use super::state::MiouState;

/// Catégories de bulles disponibles.
pub const CAT_ACCUEIL_MATIN: &str = "accueil_matin";
pub const CAT_ACCUEIL_APRES_MIDI: &str = "accueil_apres_midi";
pub const CAT_ACCUEIL_SOIR: &str = "accueil_soir";
pub const CAT_ACCUEIL_NUIT: &str = "accueil_nuit";
pub const CAT_PAUSE_SANTE: &str = "pause_sante";
pub const CAT_RAPPEL_EVENEMENT: &str = "rappel_evenement";
pub const CAT_RETOUR_ABSENCE: &str = "retour_absence";
pub const CAT_RAPPEL_AMI: &str = "rappel_ami";
pub const CAT_SUGGESTION_SERVICE: &str = "suggestion_service";
pub const CAT_SPECS_RAM: &str = "specs_ram_demande";
pub const CAT_SPECS_STOCKAGE: &str = "specs_stockage_demande";
pub const CAT_SPECS_UPGRADE: &str = "specs_upgrade_commentaire";

/// Résultat de la décision du moteur.
#[derive(Debug, Clone)]
pub struct DecisionResult {
    /// Catégorie sélectionnée (None = silence).
    pub categorie: Option<&'static str>,
    /// Raison de la décision (debug).
    #[allow(dead_code)]
    pub reason: String,
    /// Priorité de la catégorie (0 = plus haute).
    #[allow(dead_code)]
    pub priority: u8,
}

impl DecisionResult {
    fn silence(reason: &str) -> Self {
        Self {
            categorie: None,
            reason: reason.to_string(),
            priority: 255,
        }
    }

    fn category(cat: &'static str, reason: &str, priority: u8) -> Self {
        Self {
            categorie: Some(cat),
            reason: reason.to_string(),
            priority,
        }
    }
}

/// Décide quelle catégorie de bulle afficher.
///
/// Ordre de priorité (0.1.x) :
/// - P0 : Silence si bulles désactivées ou DND
/// - P1 : pause_sante (si seuil atteint)
/// - P2 : rappel_evenement (si < 1h)
/// - P3 : accueil_* (première connexion)
/// - P4 : retour_absence (si >= 3 jours)
/// - P5 : rappel_ami (si >= 7 jours)
/// - P6 : suggestion_service (si >= 14 jours)
/// - P7 : specs (RAM < 512, disk < 1 Go, upgrade)
/// - Silence sinon
pub fn decide(context: &BotContext, state: &MiouState) -> DecisionResult {
    // P0 : Silence si désactivé
    if !context.bulles_actives {
        return DecisionResult::silence("bulles_actives=false");
    }
    if context.dnd_actif {
        return DecisionResult::silence("dnd_actif=true");
    }

    // Vérifier quota de bulles
    if context.bulles_deja_affichees >= context.max_bulles_par_session {
        return DecisionResult::silence("quota bulles atteint");
    }

    // P1 : Pause santé
    if context.rappels_pause_actives
        && context.seuil_pause_minutes > 0
        && context.session_duration_minutes >= context.seuil_pause_minutes
        && !state.is_pause_recently_dismissed()
        && !state.categories_shown.contains(CAT_PAUSE_SANTE)
    {
        return DecisionResult::category(CAT_PAUSE_SANTE, "session >= seuil_pause", 1);
    }

    // P2 : Rappel événement
    if context.evenement_dans_moins_d_une_heure {
        if let Some((event_name, _)) = &context.evenement_prochain {
            // Vérifier si pas déjà rappelé (utiliser le nom comme ID simple)
            let event_id = event_name.replace(' ', "_").to_lowercase();
            if !state.is_event_reminded(&event_id) {
                return DecisionResult::category(CAT_RAPPEL_EVENEMENT, "evenement < 1h", 2);
            }
        }
    }

    // P3 : Accueil (première connexion de session, jamais affiché)
    if context.is_first_connection_of_session && context.bulles_deja_affichees == 0 {
        let cat = match context.periode_journee() {
            "matin" => CAT_ACCUEIL_MATIN,
            "apres_midi" => CAT_ACCUEIL_APRES_MIDI,
            "soir" => CAT_ACCUEIL_SOIR,
            _ => CAT_ACCUEIL_NUIT,
        };
        return DecisionResult::category(cat, "premiere connexion", 3);
    }

    // P4 : Retour absence (>= 3 jours)
    if context.is_first_connection_of_session {
        if let Some(jours) = context.jours_depuis_derniere_visite {
            if jours >= 3 && !state.categories_shown.contains(CAT_RETOUR_ABSENCE) {
                return DecisionResult::category(
                    CAT_RETOUR_ABSENCE,
                    &format!("absence {} jours", jours),
                    4,
                );
            }
        }
    }

    // P5 : Rappel ami (>= 7 jours)
    if let Some((_, jours)) = &context.ami_plus_delaisse {
        if *jours >= 7 && !state.categories_shown.contains(CAT_RAPPEL_AMI) {
            return DecisionResult::category(
                CAT_RAPPEL_AMI,
                &format!("ami delaisse {} jours", jours),
                5,
            );
        }
    }

    // P6 : Suggestion service (>= 14 jours)
    if let Some((_, jours)) = &context.service_delaisse {
        if *jours >= 14 && !state.categories_shown.contains(CAT_SUGGESTION_SERVICE) {
            return DecisionResult::category(
                CAT_SUGGESTION_SERVICE,
                &format!("service delaisse {} jours", jours),
                6,
            );
        }
    }

    // P7 : Specs machine
    if context.specs_upgraded_since_last && !state.categories_shown.contains(CAT_SPECS_UPGRADE) {
        return DecisionResult::category(CAT_SPECS_UPGRADE, "specs ameliorees", 7);
    }
    if context.ram_available_mb > 0
        && context.ram_available_mb < 512
        && !state.categories_shown.contains(CAT_SPECS_RAM)
    {
        return DecisionResult::category(CAT_SPECS_RAM, "RAM < 512 MB", 7);
    }
    if context.disk_free_gb > 0.0
        && context.disk_free_gb < 1.0
        && !state.categories_shown.contains(CAT_SPECS_STOCKAGE)
    {
        return DecisionResult::category(CAT_SPECS_STOCKAGE, "disk < 1 GB", 7);
    }

    // Silence par défaut
    DecisionResult::silence("aucune condition remplie")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decide_bulles_off() {
        let mut ctx = BotContext::default();
        ctx.bulles_actives = false;
        let state = MiouState::default();
        let result = decide(&ctx, &state);
        assert!(result.categorie.is_none());
    }

    #[test]
    fn test_decide_accueil() {
        let ctx = BotContext::default();
        let state = MiouState::default();
        let result = decide(&ctx, &state);
        assert!(result.categorie.is_some());
        // Devrait être un accueil car première connexion
        let cat = result.categorie.unwrap();
        assert!(cat.starts_with("accueil_"));
    }
}
