//! Compréhension des commandes par le **moteur local** `maia-llm` (llama.cpp).
//!
//! Alicia n'appelle aucun service d'IA extérieur : elle partage le moteur
//! `maia-llm` de MAIA (un seul chargement du modèle), par requêtes signées de
//! sa clé (usage `maia-llm/inference`, clé à épingler dans `appelants_inference`).
//!
//! # Sortie contrainte
//!
//! - Le modèle reçoit une **grammaire GBNF** ([`grammaire_gbnf`]) générée depuis
//!   le vocabulaire fermé (pièces configurées, types d'appareils, actions
//!   compatibles, routines) : il ne peut produire qu'un objet JSON d'intention.
//! - La sortie est **revalidée** par [`valider_sortie`] (clés exactes, valeurs
//!   dans les listes fermées, bornes numériques) : un moteur qui ignorerait la
//!   grammaire ne fait pas passer une intention hors vocabulaire.
//!
//! # L'IA propose, les Cores disposent
//!
//! Le résultat est une [`PropositionIa`] : rien n'est exécuté ici. L'exécution
//! passe par [`crate::AliciaService::executer_proposition`], derrière la porte
//! des Cores (`CoreGate` : StrongFather, alimenté par BorderGuard).
//!
//! # Transparence (AI Act art. 50)
//!
//! [`PRESENTATION_ALICIA`] et [`MENTION_IA_ALICIA`] : Alicia se déclare comme IA ;
//! chaque proposition porte `genere_par_ia = true`.

// @id service.alicia.nlu-local
// @role nlu_client
// @layer 7
// @do propose_intent_with_local_llm
// @human Compréhension des commandes par le moteur local maia-llm (GBNF + validation), proposition seulement.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use miyusceau::CleSignature;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use miyualicia_skills::{ConfirmationPolicy, ServiceCapability};

use crate::intent::Intent;

/// Usage signé des requêtes vers maia-llm (égal à celui de MAIA).
pub const USAGE_MAIA_LLM_INFERENCE: &str = "maia-llm/inference";

/// Présentation d'Alicia (AI Act art. 50 §1), dite à la première interaction.
pub const PRESENTATION_ALICIA: &str = "Bonjour, je suis Alicia, une assistante vocale à \
    intelligence artificielle (IA) qui fonctionne localement chez vous. Je comprends vos \
    commandes grâce à un modèle d'IA local ; les actions sur la maison restent soumises aux \
    règles de votre COG.";

/// Mention jointe à toute réponse ou proposition produite par l'IA.
pub const MENTION_IA_ALICIA: &str = "Interprétation produite par une IA (Alicia, modèle local).";

/// Types d'appareils pilotables (vocabulaire fermé).
pub const TYPES_APPAREILS: &[&str] = &["light", "shutter", "thermostat", "outlet", "lock"];
/// Propriétés interrogeables.
pub const PROPRIETES: &[&str] = &[
    "temperature",
    "humidity",
    "power",
    "on",
    "state",
    "position",
    "brightness",
];
/// Horizons météo.
pub const HORIZONS: &[&str] = &["now", "today", "tomorrow"];

/// Actions autorisées par type d'appareil ; `true` = valeur numérique requise.
const ACTIONS: &[(&str, &[(&str, bool)])] = &[
    (
        "light",
        &[
            ("on", false),
            ("off", false),
            ("toggle", false),
            ("set_brightness", true),
        ],
    ),
    (
        "shutter",
        &[("open", false), ("close", false), ("set_position", true)],
    ),
    (
        "thermostat",
        &[("on", false), ("off", false), ("set_temperature", true)],
    ),
    (
        "outlet",
        &[("on", false), ("off", false), ("toggle", false)],
    ),
    ("lock", &[("lock", false), ("unlock", false)]),
];

/// Bornes de la valeur d'une action numérique.
fn bornes(action: &str) -> Option<(f64, f64)> {
    match action {
        "set_brightness" | "set_position" => Some((0.0, 100.0)),
        "set_temperature" => Some((5.0, 30.0)),
        _ => None,
    }
}

/// Erreurs du NLU local.
#[derive(Debug, thiserror::Error)]
pub enum ErreurNluLocal {
    /// Moteur injoignable, délai dépassé ou réponse HTTP en erreur.
    #[error("moteur local indisponible : {0}")]
    Moteur(String),
    /// Sortie du modèle hors schéma (refusée).
    #[error("sortie du modèle refusée : {0}")]
    SortieRefusee(String),
}

/// Configuration du moteur local.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoteurLocalConfig {
    /// URL de base de maia-llm (ex. `http://127.0.0.1:11435`).
    pub url: String,
    /// Délai maximal d'une interprétation (ms) ; CPU lent ⇒ prévoir large.
    #[serde(default = "MoteurLocalConfig::delai_defaut")]
    pub timeout_ms: u64,
}

impl MoteurLocalConfig {
    fn delai_defaut() -> u64 {
        20_000
    }
}

/// Pièce connue : identifiant et nom parlé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceConnue {
    /// Identifiant (`[a-z0-9_-]`).
    pub id: String,
    /// Nom lisible (ex. « Chambre parentale »).
    pub nom: String,
}

/// Vocabulaire fermé de la maison.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VocabulaireNlu {
    /// Pièces (identifiants sûrs seulement, voir [`VocabulaireNlu::new`]).
    pub pieces: Vec<PieceConnue>,
    /// Routines nommées.
    pub routines: Vec<String>,
}

fn identifiant_sur(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-' || c == b'_')
}

fn nom_sur(nom: &str) -> bool {
    !nom.trim().is_empty()
        && nom.chars().count() <= 64
        && nom
            .chars()
            .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '\'' || c == '_')
}

impl VocabulaireNlu {
    /// Vocabulaire filtré : pièces à identifiant sûr, routines au nom sûr
    /// (les entrées non sûres sont ignorées, jamais injectées dans la grammaire).
    pub fn new(pieces: Vec<PieceConnue>, routines: Vec<String>) -> Self {
        Self {
            pieces: pieces
                .into_iter()
                .filter(|p| identifiant_sur(&p.id) && nom_sur(&p.nom))
                .collect(),
            routines: routines.into_iter().filter(|r| nom_sur(r)).collect(),
        }
    }

    fn piece(&self, id: &str) -> bool {
        self.pieces.iter().any(|p| p.id == id)
    }
}

/// Littéral GBNF d'une chaîne JSON `"texte"` (texte déjà filtré : sans `"` ni `\`).
fn lit_json(texte: &str) -> String {
    format!("\"\\\"{texte}\\\"\"")
}

fn alternatives(valeurs: impl IntoIterator<Item = String>) -> String {
    valeurs.into_iter().collect::<Vec<_>>().join(" | ")
}

/// Grammaire GBNF (racine `root`) des intentions, depuis le vocabulaire.
pub fn grammaire_gbnf(v: &VocabulaireNlu) -> String {
    let mut lignes: Vec<String> = Vec::new();
    let mut racines = vec!["ctrl", "query", "weather", "help", "unknown"];
    if !v.routines.is_empty() {
        racines.insert(2, "routine");
    }
    lignes.push(format!("root ::= {}\n", racines.join(" | ")));
    lignes.push(String::from(
        "ctrl ::= \"{\\\"intent\\\":\\\"control_device\\\",\" dev \",\\\"room_id\\\":\" room \"}\"\n",
    ));
    let noms_regles: Vec<String> = ACTIONS.iter().map(|(t, _)| format!("dev-{t}")).collect();
    lignes.push(format!("dev ::= {}\n", noms_regles.join(" | ")));
    for (t, actions) in ACTIONS {
        let alts = alternatives(actions.iter().map(|(a, valeur)| {
            if *valeur {
                let regle = if *a == "set_temperature" {
                    "temp"
                } else {
                    "pct"
                };
                format!("\"\\\"{a}\\\",\\\"value\\\":\" {regle}")
            } else {
                format!("\"\\\"{a}\\\",\\\"value\\\":null\"")
            }
        }));
        lignes.push(format!(
            "dev-{t} ::= \"\\\"device_type\\\":\\\"{t}\\\",\\\"action\\\":\" ( {alts} )\n"
        ));
    }
    lignes.push(String::from("pct ::= \"100\" | [1-9] [0-9] | [0-9]\n"));
    lignes.push(String::from(
        "temp ::= [1-3] [0-9] ( \".5\" )? | [5-9] ( \".5\" )?\n",
    ));
    let pieces = v.pieces.iter().map(|p| lit_json(&p.id));
    lignes.push(format!(
        "room ::= {}\n",
        alternatives(std::iter::once("\"null\"".to_string()).chain(pieces.clone()))
    ));
    lignes.push(String::from(
        "query ::= \"{\\\"intent\\\":\\\"query_state\\\",\\\"target\\\":\" target \",\\\"property\\\":\" prop \"}\"\n",
    ));
    let cibles = std::iter::once(lit_json("home"))
        .chain(pieces)
        .chain(TYPES_APPAREILS.iter().map(|t| lit_json(t)))
        .chain(std::iter::once(lit_json("sensor")));
    lignes.push(format!("target ::= {}\n", alternatives(cibles)));
    lignes.push(format!(
        "prop ::= {}\n",
        alternatives(
            std::iter::once("\"null\"".to_string()).chain(PROPRIETES.iter().map(|p| lit_json(p)))
        )
    ));
    if !v.routines.is_empty() {
        lignes.push(String::from(
            "routine ::= \"{\\\"intent\\\":\\\"activate_routine\\\",\\\"routine_name\\\":\" rname \"}\"\n",
        ));
        lignes.push(format!(
            "rname ::= {}\n",
            alternatives(v.routines.iter().map(|r| lit_json(r)))
        ));
    }
    lignes.push(String::from(
        "weather ::= \"{\\\"intent\\\":\\\"query_weather\\\",\\\"location\\\":null,\\\"horizon\\\":\" hz \"}\"\n",
    ));
    lignes.push(format!(
        "hz ::= {}\n",
        alternatives(
            std::iter::once("\"null\"".to_string()).chain(HORIZONS.iter().map(|h| lit_json(h)))
        )
    ));
    lignes.push(String::from(
        "help ::= \"{\\\"intent\\\":\\\"help\\\",\\\"topic\\\":null}\"\n",
    ));
    lignes.push(String::from(
        "unknown ::= \"{\\\"intent\\\":\\\"unknown\\\"}\"\n",
    ));
    lignes.concat()
}

/// Consigne système (vocabulaire et exemples) donnée au modèle.
pub fn prompt_systeme(v: &VocabulaireNlu) -> String {
    let pieces = v
        .pieces
        .iter()
        .map(|p| format!("{} (« {} »)", p.id, p.nom))
        .collect::<Vec<_>>()
        .join(", ");
    let routines = if v.routines.is_empty() {
        "aucune".to_string()
    } else {
        v.routines.join(", ")
    };
    format!(
        "Tu es Alicia, une IA domotique locale. Tu traduis une commande française en UNE \
         intention JSON, sans rien exécuter.\n\
         Appareils : light (lumière, lampe), shutter (volet, store), thermostat (chauffage, \
         radiateur), outlet (prise), lock (serrure, porte d'entrée).\n\
         Actions : light → on, off, toggle, set_brightness (0-100 %) ; shutter → open, close, \
         set_position (0-100 %) ; thermostat → on, off, set_temperature (5-30 °C) ; outlet → on, \
         off, toggle ; lock → lock, unlock.\n\
         Pièces : {pieces}. room_id = null si aucune de ces pièces n'est dite (la porte \
         d'entrée n'est pas une pièce).\n\
         Routines : {routines}. Une demande qui nomme une routine → activate_routine.\n\
         Question sur l'état (température, allumé…) → query_state ; météo → query_weather ; \
         demande d'aide → help ; toute autre demande → unknown.\n\
         Exemples :\n\
         « allume la lumière du salon » → {{\"intent\":\"control_device\",\"device_type\":\"light\",\
         \"action\":\"on\",\"value\":null,\"room_id\":\"salon\"}}\n\
         « mets le chauffage à 19 degrés » → {{\"intent\":\"control_device\",\"device_type\":\
         \"thermostat\",\"action\":\"set_temperature\",\"value\":19,\"room_id\":null}}\n\
         « quelle température fait-il dans le salon ? » → {{\"intent\":\"query_state\",\"target\":\
         \"salon\",\"property\":\"temperature\"}}\n\
         « active la routine matin » (si la routine « matin » existe) → {{\"intent\":\
         \"activate_routine\",\"routine_name\":\"matin\"}}\n\
         « commande une pizza » → {{\"intent\":\"unknown\"}}"
    )
}

fn texte(o: &Map<String, Value>, cle: &str) -> Result<Option<String>, ErreurNluLocal> {
    match o.get(cle) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(_) => Err(ErreurNluLocal::SortieRefusee(format!(
            "« {cle} » n'est pas un texte"
        ))),
    }
}

fn exiger_cles(o: &Map<String, Value>, permises: &[&str]) -> Result<(), ErreurNluLocal> {
    match o.keys().find(|k| !permises.contains(&k.as_str())) {
        Some(k) => Err(ErreurNluLocal::SortieRefusee(format!(
            "clé inattendue « {k} »"
        ))),
        None => Ok(()),
    }
}

fn dans(valeur: &str, liste: &[&str], quoi: &str) -> Result<(), ErreurNluLocal> {
    if liste.contains(&valeur) {
        Ok(())
    } else {
        Err(ErreurNluLocal::SortieRefusee(format!(
            "{quoi} « {valeur} » hors vocabulaire"
        )))
    }
}

/// Valide la sortie du modèle (JSON, schéma fermé) et la convertit en [`Intent`].
pub fn valider_sortie(
    sortie: &str,
    v: &VocabulaireNlu,
    transcript: &str,
) -> Result<Intent, ErreurNluLocal> {
    let refus = |m: String| ErreurNluLocal::SortieRefusee(m);
    let valeur: Value =
        serde_json::from_str(sortie.trim()).map_err(|e| refus(format!("JSON invalide : {e}")))?;
    let o = valeur
        .as_object()
        .ok_or_else(|| refus("objet JSON attendu".into()))?;
    let intent = texte(o, "intent")?.ok_or_else(|| refus("« intent » absent".into()))?;
    match intent.as_str() {
        "control_device" => {
            exiger_cles(o, &["intent", "device_type", "action", "value", "room_id"])?;
            let device_type =
                texte(o, "device_type")?.ok_or_else(|| refus("device_type absent".into()))?;
            let action = texte(o, "action")?.ok_or_else(|| refus("action absente".into()))?;
            let actions = ACTIONS
                .iter()
                .find(|(t, _)| *t == device_type)
                .map(|(_, a)| *a)
                .ok_or_else(|| refus(format!("appareil « {device_type} » hors vocabulaire")))?;
            let (_, avec_valeur) = actions.iter().find(|(a, _)| *a == action).ok_or_else(|| {
                refus(format!("action « {action} » impossible sur {device_type}"))
            })?;
            let value = match (o.get("value"), avec_valeur, bornes(&action)) {
                (None | Some(Value::Null), false, _) => None,
                (Some(Value::Number(n)), true, Some((min, max))) => {
                    let x = n
                        .as_f64()
                        .ok_or_else(|| refus("valeur non numérique".into()))?;
                    if !(min..=max).contains(&x) {
                        return Err(refus(format!("valeur {x} hors [{min}, {max}]")));
                    }
                    Some(Value::Number(n.clone()))
                }
                _ => return Err(refus(format!("valeur incohérente pour « {action} »"))),
            };
            let room_id = texte(o, "room_id")?;
            if let Some(r) = &room_id {
                if !v.piece(r) {
                    return Err(refus(format!("pièce « {r} » inconnue")));
                }
            }
            Ok(Intent::ControlDevice {
                device_type,
                room_id,
                action,
                value,
            })
        }
        "query_state" => {
            exiger_cles(o, &["intent", "target", "property"])?;
            let target = texte(o, "target")?.ok_or_else(|| refus("target absente".into()))?;
            if target != "home"
                && target != "sensor"
                && !v.piece(&target)
                && !TYPES_APPAREILS.contains(&target.as_str())
            {
                return Err(refus(format!("cible « {target} » hors vocabulaire")));
            }
            let property = texte(o, "property")?;
            if let Some(p) = &property {
                dans(p, PROPRIETES, "propriété")?;
            }
            Ok(Intent::QueryState { target, property })
        }
        "activate_routine" => {
            exiger_cles(o, &["intent", "routine_name"])?;
            let r = texte(o, "routine_name")?.ok_or_else(|| refus("routine absente".into()))?;
            if !v.routines.contains(&r) {
                return Err(refus(format!("routine « {r} » inconnue")));
            }
            Ok(Intent::ActivateRoutine { routine_name: r })
        }
        "query_weather" => {
            exiger_cles(o, &["intent", "location", "horizon"])?;
            if texte(o, "location")?.is_some() {
                return Err(refus("lieu libre refusé (lieu configuré seulement)".into()));
            }
            let horizon = texte(o, "horizon")?;
            if let Some(h) = &horizon {
                dans(h, HORIZONS, "horizon")?;
            }
            Ok(Intent::QueryWeather {
                location: None,
                horizon,
            })
        }
        "help" => {
            exiger_cles(o, &["intent", "topic"])?;
            Ok(Intent::Help { topic: None })
        }
        "unknown" => {
            exiger_cles(o, &["intent"])?;
            Ok(Intent::Unknown {
                transcript: transcript.to_string(),
            })
        }
        autre => Err(refus(format!("intention « {autre} » hors vocabulaire"))),
    }
}

/// Intention **proposée** par l'IA (jamais exécutée sans la porte des Cores).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PropositionIa {
    /// Intention typée et validée.
    pub intent: Intent,
    /// Toujours `true` : contenu produit par une IA (AI Act art. 50 §2).
    pub genere_par_ia: bool,
    /// Mention lisible.
    pub mention_ia: &'static str,
    /// Modèle ayant produit la proposition (tel que déclaré par le moteur).
    pub modele: String,
}

/// Capacité « domotique » soumise aux Cores pour une intention actionnable
/// (`None` : intention de lecture ou inconnue, rien à exécuter).
///
/// Une serrure (`lock`) devient la capacité **`domotique.serrure`** à
/// confirmation obligatoire : la politique de base ne l'autorise pas, l'IA ne
/// peut donc jamais (dé)verrouiller une porte sans règle dédiée (refus par défaut).
pub fn capacite_domotique(intent: &Intent) -> Option<(ServiceCapability, Value)> {
    let mut confirmation = ConfirmationPolicy::Never;
    let (nom, params) = match intent {
        Intent::ControlDevice {
            device_type,
            room_id,
            action,
            value,
        } => {
            let nom = if device_type == "lock" {
                confirmation = ConfirmationPolicy::Always;
                "serrure"
            } else {
                "control_device"
            };
            (
                nom,
                json!({"device_type": device_type, "room_id": room_id, "action": action, "value": value}),
            )
        }
        Intent::ActivateRoutine { routine_name } => {
            ("activate_routine", json!({"routine_name": routine_name}))
        }
        _ => return None,
    };
    Some((
        ServiceCapability {
            service_id: "domotique".into(),
            intent: nom.into(),
            phrases_examples: Vec::new(),
            params: BTreeMap::new(),
            writes: true,
            confirmation_default: confirmation,
            description: Some("Commande domotique proposée par l'IA d'Alicia".into()),
            category: Some("domotique".into()),
        },
        params,
    ))
}

/// Client du moteur local (maia-llm).
#[derive(Debug, Clone)]
pub struct NluLocal {
    config: MoteurLocalConfig,
    client: reqwest::Client,
    cle: Option<Arc<CleSignature>>,
    vocabulaire: VocabulaireNlu,
    grammaire: String,
    systeme: String,
}

impl NluLocal {
    /// Client pour `config`, vocabulaire de la maison.
    pub fn new(
        config: MoteurLocalConfig,
        vocabulaire: VocabulaireNlu,
        client: reqwest::Client,
    ) -> Self {
        let grammaire = grammaire_gbnf(&vocabulaire);
        let systeme = prompt_systeme(&vocabulaire);
        Self {
            config,
            client,
            cle: None,
            vocabulaire,
            grammaire,
            systeme,
        }
    }

    /// Signe les requêtes avec la clé d'Alicia (à épingler dans maia-llm).
    pub fn avec_cle(mut self, cle: Arc<CleSignature>) -> Self {
        self.cle = Some(cle);
        self
    }

    /// Vocabulaire en vigueur.
    pub fn vocabulaire(&self) -> &VocabulaireNlu {
        &self.vocabulaire
    }

    /// Demande au moteur local une intention pour `transcript` (proposition seulement).
    pub async fn proposer(&self, transcript: &str) -> Result<PropositionIa, ErreurNluLocal> {
        let transcript = transcript.trim();
        if transcript.is_empty() || transcript.chars().count() > 500 {
            return Err(ErreurNluLocal::SortieRefusee(
                "commande vide ou trop longue".into(),
            ));
        }
        let corps = json!({
            "model": "default",
            "messages": [
                {"role": "system", "content": self.systeme},
                {"role": "user", "content": transcript},
            ],
            "temperature": 0.0,
            "max_tokens": 96,
            "grammar": self.grammaire,
            "stream": false,
        });
        let url = format!(
            "{}/v1/chat/completions",
            self.config.url.trim_end_matches('/')
        );
        let requete = crate::nlu_bridge::post_signe(
            &self.client,
            &url,
            USAGE_MAIA_LLM_INFERENCE,
            self.cle.as_deref(),
            &corps,
        )
        .map_err(|e| ErreurNluLocal::Moteur(e.to_string()))?;
        let reponse = requete
            .timeout(Duration::from_millis(self.config.timeout_ms))
            .send()
            .await
            .map_err(|e| ErreurNluLocal::Moteur(e.to_string()))?;
        if !reponse.status().is_success() {
            return Err(ErreurNluLocal::Moteur(format!("HTTP {}", reponse.status())));
        }
        let v: Value = reponse
            .json()
            .await
            .map_err(|e| ErreurNluLocal::Moteur(format!("réponse illisible : {e}")))?;
        let contenu = v
            .pointer("/choices/0/message/content")
            .and_then(Value::as_str)
            .ok_or_else(|| ErreurNluLocal::Moteur("réponse sans contenu".into()))?;
        let intent = valider_sortie(contenu, &self.vocabulaire, transcript)?;
        Ok(PropositionIa {
            intent,
            genere_par_ia: true,
            mention_ia: MENTION_IA_ALICIA,
            modele: v
                .get("model")
                .and_then(Value::as_str)
                .unwrap_or("inconnu")
                .to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vocab() -> VocabulaireNlu {
        VocabulaireNlu::new(
            vec![
                PieceConnue {
                    id: "salon".into(),
                    nom: "Salon".into(),
                },
                PieceConnue {
                    id: "chambre-parentale".into(),
                    nom: "Chambre parentale".into(),
                },
                PieceConnue {
                    id: "cuisine\"; root ::= x".into(),
                    nom: "piège".into(),
                },
            ],
            vec!["bonne nuit".into(), "je pars".into(), "x\" | evil".into()],
        )
    }

    #[test]
    fn vocabulaire_filtre_les_entrees_non_sures() {
        let v = vocab();
        assert_eq!(v.pieces.len(), 2);
        assert_eq!(
            v.routines,
            vec!["bonne nuit".to_string(), "je pars".to_string()]
        );
        let g = grammaire_gbnf(&v);
        assert!(!g.contains("evil") && !g.contains("cuisine"));
        assert!(g.starts_with("root ::= ctrl | query | routine | weather | help | unknown\n"));
        assert!(g.contains("\\\"chambre-parentale\\\""));
        assert!(g.contains("dev-lock ::="));
    }

    #[test]
    fn grammaire_sans_routine() {
        let g = grammaire_gbnf(&VocabulaireNlu::new(vec![], vec![]));
        assert!(!g.contains("routine"));
        assert!(g.contains("room ::= \"null\"\n"));
    }

    #[test]
    fn sorties_valides_converties() {
        let v = vocab();
        let i = valider_sortie(
            r#"{"intent":"control_device","device_type":"light","action":"on","value":null,"room_id":"salon"}"#,
            &v,
            "allume le salon",
        )
        .unwrap();
        assert_eq!(
            i,
            Intent::ControlDevice {
                device_type: "light".into(),
                room_id: Some("salon".into()),
                action: "on".into(),
                value: None
            }
        );
        let t = valider_sortie(
            r#"{"intent":"control_device","device_type":"thermostat","action":"set_temperature","value":19.5,"room_id":null}"#,
            &v,
            "",
        )
        .unwrap();
        assert!(matches!(
            t,
            Intent::ControlDevice {
                value: Some(_),
                room_id: None,
                ..
            }
        ));
        assert!(matches!(
            valider_sortie(r#"{"intent":"unknown"}"#, &v, "pizza").unwrap(),
            Intent::Unknown { transcript } if transcript == "pizza"
        ));
        assert!(matches!(
            valider_sortie(
                r#"{"intent":"activate_routine","routine_name":"bonne nuit"}"#,
                &v,
                ""
            )
            .unwrap(),
            Intent::ActivateRoutine { .. }
        ));
        assert!(valider_sortie(
            r#"{"intent":"query_state","target":"salon","property":"temperature"}"#,
            &v,
            ""
        )
        .is_ok());
    }

    #[test]
    fn sorties_hors_schema_refusees() {
        let v = vocab();
        for s in [
            "pas du json",
            "[]",
            r#"{"intent":"rm -rf"}"#,
            r#"{"intent":"control_device","device_type":"oven","action":"on","value":null,"room_id":null}"#,
            r#"{"intent":"control_device","device_type":"lock","action":"on","value":null,"room_id":null}"#,
            r#"{"intent":"control_device","device_type":"light","action":"on","value":null,"room_id":"garage"}"#,
            r#"{"intent":"control_device","device_type":"light","action":"set_brightness","value":250,"room_id":null}"#,
            r#"{"intent":"control_device","device_type":"light","action":"set_brightness","value":null,"room_id":null}"#,
            r#"{"intent":"control_device","device_type":"light","action":"on","value":3,"room_id":null}"#,
            r#"{"intent":"control_device","device_type":"light","action":"on","value":null,"room_id":null,"x":1}"#,
            r#"{"intent":"activate_routine","routine_name":"ouvre tout"}"#,
            r#"{"intent":"query_weather","location":"Paris","horizon":"now"}"#,
            r#"{"intent":"query_state","target":"garage","property":null}"#,
        ] {
            assert!(valider_sortie(s, &v, "").is_err(), "accepté à tort : {s}");
        }
    }

    #[test]
    fn seules_les_intentions_actionnables_vont_aux_cores() {
        let i = Intent::ControlDevice {
            device_type: "light".into(),
            room_id: None,
            action: "off".into(),
            value: None,
        };
        let (cap, params) = capacite_domotique(&i).unwrap();
        assert_eq!(cap.qualified_id(), "domotique.control_device");
        assert!(cap.writes);
        assert_eq!(params["action"], "off");
        assert!(capacite_domotique(&Intent::Help { topic: None }).is_none());
        let serrure = Intent::ControlDevice {
            device_type: "lock".into(),
            room_id: None,
            action: "unlock".into(),
            value: None,
        };
        let (cap, _) = capacite_domotique(&serrure).unwrap();
        assert_eq!(cap.qualified_id(), "domotique.serrure");
        assert_eq!(cap.confirmation_default, ConfirmationPolicy::Always);
    }

    #[test]
    fn alicia_se_declare_comme_ia() {
        assert!(PRESENTATION_ALICIA.contains("intelligence artificielle"));
        assert!(MENTION_IA_ALICIA.contains("IA"));
        assert!(prompt_systeme(&vocab()).starts_with("Tu es Alicia, une IA"));
    }

    #[tokio::test]
    async fn moteur_injoignable_erreur_sans_appel_exterieur() {
        let n = NluLocal::new(
            MoteurLocalConfig {
                url: "http://127.0.0.1:9".into(),
                timeout_ms: 500,
            },
            vocab(),
            reqwest::Client::new(),
        );
        assert!(matches!(
            n.proposer("allume").await,
            Err(ErreurNluLocal::Moteur(_))
        ));
        assert!(matches!(
            n.proposer("  ").await,
            Err(ErreurNluLocal::SortieRefusee(_))
        ));
    }
}
