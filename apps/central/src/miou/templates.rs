//! Banque de templates Miou et injection de variables.

use super::bubble::{ActionType, BulleAction, BulleOutput};
use super::context::BotContext;
use super::engine::*;
use std::collections::HashMap;

/// Template de bulle.
#[derive(Debug, Clone)]
pub struct Template {
    pub id: &'static str,
    pub categorie: &'static str,
    pub texte: &'static str,
    pub actions: Vec<(&'static str, ActionType, Option<&'static str>)>,
}

/// Banque de templates 0.1.x.
pub fn get_templates() -> HashMap<&'static str, Vec<Template>> {
    let mut bank: HashMap<&'static str, Vec<Template>> = HashMap::new();

    // Accueil Matin
    bank.insert(
        CAT_ACCUEIL_MATIN,
        vec![
            Template {
                id: "matin_1",
                categorie: CAT_ACCUEIL_MATIN,
                texte: "Bonjour {pseudo} ! 🌅 Prêt·e pour une nouvelle journée ?",
                actions: vec![("Merci Miou", ActionType::Dismiss, None)],
            },
            Template {
                id: "matin_2",
                categorie: CAT_ACCUEIL_MATIN,
                texte: "Salut {pseudo}, une nouvelle journée commence. Je suis là si tu as besoin.",
                actions: vec![("OK", ActionType::Dismiss, None)],
            },
            Template {
                id: "matin_3",
                categorie: CAT_ACCUEIL_MATIN,
                texte: "Hey {pseudo} ! Le soleil se lève, ton COG aussi. Bonne journée !",
                actions: vec![("Merci", ActionType::Dismiss, None)],
            },
        ],
    );

    // Accueil Après-midi
    bank.insert(
        CAT_ACCUEIL_APRES_MIDI,
        vec![
            Template {
                id: "aprem_1",
                categorie: CAT_ACCUEIL_APRES_MIDI,
                texte: "Hey {pseudo}, contente de te voir cet après-midi.",
                actions: vec![("OK", ActionType::Dismiss, None)],
            },
            Template {
                id: "aprem_2",
                categorie: CAT_ACCUEIL_APRES_MIDI,
                texte: "Bonne après-midi {pseudo} ! Qu'est-ce qu'on fait de beau ?",
                actions: vec![("Merci Miou", ActionType::Dismiss, None)],
            },
        ],
    );

    // Accueil Soir
    bank.insert(
        CAT_ACCUEIL_SOIR,
        vec![
            Template {
                id: "soir_1",
                categorie: CAT_ACCUEIL_SOIR,
                texte: "Bonsoir {pseudo}. 🌙 Une soirée tranquille en perspective ?",
                actions: vec![("Oui !", ActionType::Dismiss, None)],
            },
            Template {
                id: "soir_2",
                categorie: CAT_ACCUEIL_SOIR,
                texte: "Hey {pseudo}, bonne soirée sur ton COG. N'oublie pas de te reposer.",
                actions: vec![("Merci", ActionType::Dismiss, None)],
            },
        ],
    );

    // Accueil Nuit
    bank.insert(
        CAT_ACCUEIL_NUIT,
        vec![Template {
            id: "nuit_1",
            categorie: CAT_ACCUEIL_NUIT,
            texte: "Encore debout, {pseudo} ? 🦉 Je veille avec toi.",
            actions: vec![("Merci", ActionType::Dismiss, None)],
        }],
    );

    // Pause Santé
    bank.insert(
        CAT_PAUSE_SANTE,
        vec![
            Template {
                id: "pause_1",
                categorie: CAT_PAUSE_SANTE,
                texte: "Ça fait {duree} que tu es là — accorde-toi une petite pause ! 🍵",
                actions: vec![
                    ("Je prends une pause", ActionType::Pause, None),
                    ("Plus tard", ActionType::Dismiss, None),
                ],
            },
            Template {
                id: "pause_2",
                categorie: CAT_PAUSE_SANTE,
                texte: "{pseudo}, {duree} déjà ! Tes yeux méritent un peu de repos.",
                actions: vec![
                    ("OK, je fais une pause", ActionType::Pause, None),
                    ("Pas maintenant", ActionType::Dismiss, None),
                ],
            },
            Template {
                id: "pause_3",
                categorie: CAT_PAUSE_SANTE,
                texte: "Une pause ? Tu en as bien besoin après {duree}. 🧘",
                actions: vec![
                    ("Je fais une pause", ActionType::Pause, None),
                    ("Je continue", ActionType::Dismiss, None),
                ],
            },
        ],
    );

    // Rappel Événement
    bank.insert(
        CAT_RAPPEL_EVENEMENT,
        vec![
            Template {
                id: "event_1",
                categorie: CAT_RAPPEL_EVENEMENT,
                texte: "📅 Rappel : « {evenement} » dans moins d'une heure !",
                actions: vec![
                    (
                        "Voir dans JayKoa",
                        ActionType::OuvrirService,
                        Some("jaykoa"),
                    ),
                    ("OK", ActionType::Dismiss, None),
                ],
            },
            Template {
                id: "event_2",
                categorie: CAT_RAPPEL_EVENEMENT,
                texte: "{pseudo}, n'oublie pas : {evenement} approche !",
                actions: vec![
                    ("Ouvrir JayKoa", ActionType::OuvrirService, Some("jaykoa")),
                    ("Merci", ActionType::Dismiss, None),
                ],
            },
        ],
    );

    // Retour Absence
    bank.insert(
        CAT_RETOUR_ABSENCE,
        vec![
            Template {
                id: "retour_1",
                categorie: CAT_RETOUR_ABSENCE,
                texte: "Te revoilà {pseudo} ! Ça fait {jours} jours. Tu m'as manqué. 💕",
                actions: vec![("Merci Miou", ActionType::Dismiss, None)],
            },
            Template {
                id: "retour_2",
                categorie: CAT_RETOUR_ABSENCE,
                texte: "Contente de te revoir après {jours} jours, {pseudo}. Tout va bien ?",
                actions: vec![("Oui, merci !", ActionType::Dismiss, None)],
            },
        ],
    );

    // Rappel Ami
    bank.insert(
        CAT_RAPPEL_AMI,
        vec![Template {
            id: "ami_1",
            categorie: CAT_RAPPEL_AMI,
            texte:
                "Tu n'as pas donné de nouvelles à {ami} depuis {jours} jours. Un petit message ?",
            actions: vec![
                (
                    "Voir Jay1Tribu",
                    ActionType::OuvrirService,
                    Some("jay1tribu"),
                ),
                ("Plus tard", ActionType::Dismiss, None),
            ],
        }],
    );

    // Suggestion Service
    bank.insert(
        CAT_SUGGESTION_SERVICE,
        vec![Template {
            id: "service_1",
            categorie: CAT_SUGGESTION_SERVICE,
            texte: "Tu n'as pas ouvert {service} depuis un moment. Envie d'y faire un tour ?",
            actions: vec![
                (
                    "Ouvrir {service}",
                    ActionType::OuvrirService,
                    Some("{service_id}"),
                ),
                ("Pas maintenant", ActionType::Dismiss, None),
            ],
        }],
    );

    // Specs RAM
    bank.insert(
        CAT_SPECS_RAM,
        vec![Template {
            id: "ram_1",
            categorie: CAT_SPECS_RAM,
            texte: "J'aimerais un peu plus de RAM pour mieux te servir. 🧠",
            actions: vec![("Compris", ActionType::Dismiss, None)],
        }],
    );

    // Specs Stockage
    bank.insert(
        CAT_SPECS_STOCKAGE,
        vec![Template {
            id: "disk_1",
            categorie: CAT_SPECS_STOCKAGE,
            texte: "Mon disque s'essouffle — un peu de ménage ? 🧹",
            actions: vec![("Je vais voir", ActionType::Dismiss, None)],
        }],
    );

    // Specs Upgrade
    bank.insert(
        CAT_SPECS_UPGRADE,
        vec![Template {
            id: "upgrade_1",
            categorie: CAT_SPECS_UPGRADE,
            texte: "Tu as amélioré la machine — merci {pseudo} ! ⚡",
            actions: vec![("De rien", ActionType::Dismiss, None)],
        }],
    );

    bank
}

/// Sélectionne une variante non utilisée récemment pour une catégorie.
pub fn select_variante(
    categorie: &str,
    used_variantes: &std::collections::HashSet<String>,
) -> Option<Template> {
    let bank = get_templates();
    let templates = bank.get(categorie)?;

    // Chercher une variante non utilisée
    for t in templates {
        let key = format!("{}:{}", categorie, t.id);
        if !used_variantes.contains(&key) {
            return Some(t.clone());
        }
    }

    // Fallback : reprendre la première
    templates.first().cloned()
}

/// Injecte les variables dans le texte du template.
pub fn inject_variables(text: &str, context: &BotContext) -> String {
    let mut result = text.to_string();

    // {pseudo}
    result = result.replace("{pseudo}", &context.pseudo);

    // {duree}
    result = result.replace("{duree}", &context.format_duree());

    // {jours} - depuis dernière visite ou ami/service
    if let Some(jours) = context.jours_depuis_derniere_visite {
        result = result.replace("{jours}", &jours.to_string());
    }
    if let Some((_, jours)) = &context.ami_plus_delaisse {
        result = result.replace("{jours}", &jours.to_string());
    }
    if let Some((_, jours)) = &context.service_delaisse {
        result = result.replace("{jours}", &jours.to_string());
    }

    // {service}
    if let Some((service, _)) = &context.service_delaisse {
        result = result.replace("{service}", service);
        // Convertir nom service en ID (simplification)
        let service_id = service.to_lowercase().replace(' ', "_");
        result = result.replace("{service_id}", &service_id);
    }

    // {ami}
    if let Some((ami, _)) = &context.ami_plus_delaisse {
        result = result.replace("{ami}", ami);
    }

    // {evenement}
    if let Some((event, _)) = &context.evenement_prochain {
        result = result.replace("{evenement}", event);
    }

    result
}

/// Génère une BulleOutput à partir d'un template et du contexte.
pub fn generate_bulle(template: &Template, context: &BotContext) -> BulleOutput {
    let texte = inject_variables(template.texte, context);

    let actions: Vec<BulleAction> = template
        .actions
        .iter()
        .map(|(label, action_type, payload)| {
            let label_injected = inject_variables(label, context);
            let payload_injected = payload.map(|p| inject_variables(p, context));
            BulleAction {
                label: label_injected,
                action_type: action_type.clone(),
                payload: payload_injected,
            }
        })
        .collect();

    BulleOutput::new(texte, template.categorie, template.id, actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_pseudo() {
        let mut ctx = BotContext::default();
        ctx.pseudo = "Luna".to_string();
        let result = inject_variables("Bonjour {pseudo} !", &ctx);
        assert_eq!(result, "Bonjour Luna !");
    }

    #[test]
    fn test_inject_duree() {
        let mut ctx = BotContext::default();
        ctx.session_duration_minutes = 135;
        let result = inject_variables("Tu es là depuis {duree}.", &ctx);
        assert_eq!(result, "Tu es là depuis 2h15.");
    }

    #[test]
    fn test_select_variante() {
        let used = std::collections::HashSet::new();
        let template = select_variante(CAT_ACCUEIL_MATIN, &used);
        assert!(template.is_some());
    }
}
