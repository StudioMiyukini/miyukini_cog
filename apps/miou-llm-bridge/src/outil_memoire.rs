//! @id     maia_outil_memoire
//! @do     noter_en_memoire_depuis_le_planificateur
//! @role   security
//! @layer  app
//! @human  MAIA — outil `memoire.noter` du planificateur : note ordinaire écrite par MAIA (A-009,
//! @human  A-010) via `VueMaia::noter` ; tour avec contenu étranger ⇒ candidat seulement.

//! # Outil `memoire.noter` (nom OpenAI : `memoire_noter`)
//!
//! - Le planificateur (modèle avec outils) propose une note ; elle passe par
//!   [`maia_memoire::VueMaia::noter`] : écriture directe d'une note ordinaire
//!   (N2 / N3, déduction, hors domaine sensible, quota) ou candidat sinon ;
//! - **garde supplémentaire** (revue de sécurité du lot 7) : si un résultat
//!   d'outil du tour n'est pas `locale` (`etranger` ou `inconnue`, F3 ;
//!   [`crate::garde_etranger::EtatTour::tous_locaux`]), la note est marquée
//!   `etranger` et devient **toujours** un candidat
//!   ([`maia_memoire::VueMaia::proposer`]) : jamais d'écriture directe ;
//! - **compétence explicite** [`COMPETENCE_MEMOIRE`] (F6) : l'outil n'est
//!   proposé qu'aux agents qui la déclarent, et contrôlé avant toute
//!   exécution ;
//! - décision StrongFather obligatoire avant tout effet (K-11) :
//!   `data.operation = 'memoire.noter'` (voir [`intention_noter`]) ; sans
//!   politique applicable : refus par défaut ;
//! - aucun oubli, aucune Règle, aucune lecture d'une autre mémoire : l'outil ne
//!   détient qu'une [`maia_memoire::VueMaia`].

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use bondingbrother::strongfather::{ActionType, CallContext, IntentData};
use bondingbrother::Intent;
use maia_memoire::{
    Branche, Entrepot, ErreurMemoire, IdMemoire, Notee, Proposition, Provenance, TypeUnite, Uuid,
    VueMaia,
};

use crate::agents::AgentDef;
use crate::garde_etranger;
use crate::gouvernance::{GouvernanceMaia, APPELANT_LOCAL, ORIGINE_MAIA};

/// Nom de l'outil (format des fonctions OpenAI : pas de point).
pub const NOM_OUTIL_NOTER: &str = "memoire_noter";
/// Valeur de `data.operation` de l'intention StrongFather.
pub const OPERATION_NOTER: &str = "memoire.noter";
/// Compétence d'agent requise pour l'outil (à déclarer dans `skill_ids`, F6).
pub const COMPETENCE_MEMOIRE: &str = "memoire";

/// Vrai si l'agent déclare la compétence [`COMPETENCE_MEMOIRE`] (et en a le niveau).
pub fn competence_memoire(agent: &AgentDef) -> bool {
    matches!(
        crate::security::check_agent_skill_permission(agent, COMPETENCE_MEMOIRE),
        crate::security::PermissionCheck::Allowed
    )
}

type Futur<'a, T> = Pin<Box<dyn Future<Output = Result<T, ErreurMemoire>> + Send + 'a>>;

/// Ce que l'outil peut faire sur la mémoire : noter et proposer, rien d'autre.
pub trait CarnetMaia: Send + Sync {
    /// Voir [`VueMaia::noter`].
    fn noter(&self, p: Proposition) -> Futur<'_, Notee>;
    /// Voir [`VueMaia::proposer`].
    fn proposer(&self, p: Proposition) -> Futur<'_, Uuid>;
}

impl<E: Entrepot + 'static> CarnetMaia for VueMaia<E> {
    fn noter(&self, p: Proposition) -> Futur<'_, Notee> {
        Box::pin(VueMaia::noter(self, p))
    }

    fn proposer(&self, p: Proposition) -> Futur<'_, Uuid> {
        Box::pin(VueMaia::proposer(self, p))
    }
}

/// Définition de l'outil au format OpenAI.
pub fn definition_noter() -> crate::tools::ToolDefinition {
    crate::tools::ToolDefinition {
        tool_type: "function".into(),
        function: crate::tools::FunctionDefinition {
            name: NOM_OUTIL_NOTER.into(),
            description: "Note une déduction durable dans la mémoire de l'utilisateur (une ligne courte). \
                          Une note sensible, importante ou issue de contenu externe reste une proposition \
                          que l'utilisateur valide. Jamais de règle."
                .into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "branche": { "type": "string", "description": "branche hiérarchique, ex. travail.cog" },
                    "type": { "type": "string", "enum": ["fait", "preference", "episode", "procedure", "entite"] },
                    "texte": { "type": "string" },
                    "importance": { "type": "integer", "minimum": 0, "maximum": 9 },
                    "cible": { "type": "string", "description": "identifiant d'une note à corriger (optionnel)" }
                },
                "required": ["branche", "type", "texte", "importance"],
                "additionalProperties": false
            }),
        },
    }
}

/// Proposition construite à partir des arguments de l'outil. `contenu_etranger` :
/// provenance `etranger` (EX-53, EX-60 : une valeur dérivée reste étrangère).
pub fn proposition(
    params: &HashMap<String, serde_json::Value>,
    contenu_etranger: bool,
) -> Result<Proposition, String> {
    const CLES: [&str; 5] = ["branche", "type", "texte", "importance", "cible"];
    if let Some(k) = params.keys().find(|k| !CLES.contains(&k.as_str())) {
        return Err(format!("argument inconnu : {k}"));
    }
    let texte = |k: &str| {
        params
            .get(k)
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("argument « {k} » manquant"))
    };
    let type_unite = match texte("type")? {
        "fait" => TypeUnite::Fait,
        "preference" => TypeUnite::Preference,
        "episode" => TypeUnite::Episode,
        "procedure" => TypeUnite::Procedure,
        "entite" => TypeUnite::Entite,
        autre => return Err(format!("type refusé : {autre}")),
    };
    let importance = params
        .get("importance")
        .and_then(serde_json::Value::as_u64)
        .and_then(|v| u8::try_from(v).ok())
        .ok_or("argument « importance » invalide")?;
    let cible = match params.get("cible") {
        None | Some(serde_json::Value::Null) => None,
        Some(v) => Some(
            IdMemoire::parse(v.as_str().ok_or("argument « cible » invalide")?)
                .map_err(|e| e.to_string())?,
        ),
    };
    Ok(Proposition {
        cible,
        branche: Branche::parse(texte("branche")?).map_err(|e| e.to_string())?,
        type_unite,
        texte: texte("texte")?.to_string(),
        importance,
        // Plafonnée par la bibliothèque selon la provenance (≤ 6 déduit, ≤ 4 étranger).
        confiance: 9,
        provenance: if contenu_etranger {
            Provenance::Etranger
        } else {
            Provenance::Deduit
        },
        liens: Vec::new(),
        sources: Vec::new(),
    })
}

/// Note de MAIA : tour avec contenu étranger ⇒ **candidat** seulement ; sinon
/// [`VueMaia::noter`] (écriture directe d'une note ordinaire ou candidat).
pub async fn noter(
    carnet: &dyn CarnetMaia,
    mut p: Proposition,
    contenu_etranger: bool,
) -> Result<Notee, ErreurMemoire> {
    if contenu_etranger || p.provenance == Provenance::Etranger {
        p.provenance = Provenance::Etranger;
        return carnet.proposer(p).await.map(Notee::Candidat);
    }
    carnet.noter(p).await
}

/// Intention StrongFather de l'outil : `action = modification`,
/// `subject = maia.agent.<agent>`, `data.operation = 'memoire.noter'`,
/// `data.agent`, `data.competence_memoire` (`true` si l'agent déclare la
/// compétence `memoire`), `data.contenu_etranger` (un résultat d'outil du tour
/// n'est pas `locale`), `data.ecriture_directe_possible` (`false` dans un tour
/// étranger : candidat seulement).
pub fn intention_noter(agent: &AgentDef, contenu_etranger: bool) -> Intent {
    let mut data = HashMap::new();
    data.insert("operation".into(), OPERATION_NOTER.into());
    data.insert("outil".into(), NOM_OUTIL_NOTER.into());
    data.insert("agent".into(), agent.id.clone());
    data.insert(
        "competence_memoire".into(),
        competence_memoire(agent).to_string(),
    );
    data.insert("contenu_etranger".into(), contenu_etranger.to_string());
    data.insert(
        "ecriture_directe_possible".into(),
        (!contenu_etranger).to_string(),
    );
    Intent::new(
        format!(
            "maia-memoire-noter-{}",
            garde_etranger::identifiant_aleatoire()
        ),
        ActionType::Modification,
        format!("maia.agent.{}", agent.id.to_lowercase()),
        CallContext {
            caller_identity: APPELANT_LOCAL.to_string(),
            origin: ORIGINE_MAIA.to_string(),
            instance: "local".to_string(),
        },
        IntentData { content: data },
    )
}

/// Exécute l'outil (StrongFather puis note) et rend le résultat JSON pour le modèle.
pub async fn executer(
    gouvernance: &GouvernanceMaia,
    carnet: Option<&dyn CarnetMaia>,
    agent: &AgentDef,
    params: &HashMap<String, serde_json::Value>,
    contenu_etranger: bool,
) -> serde_json::Value {
    if !competence_memoire(agent) {
        return serde_json::json!({
            "success": false,
            "error": format!("Agent '{}' sans la compétence « {COMPETENCE_MEMOIRE} »", agent.display_name),
        });
    }
    let Some(carnet) = carnet else {
        return serde_json::json!({ "success": false, "error": "mémoire de l'utilisateur non disponible" });
    };
    let p = match proposition(params, contenu_etranger) {
        Ok(p) => p,
        Err(e) => return serde_json::json!({ "success": false, "error": e }),
    };
    let intent = intention_noter(agent, contenu_etranger);
    if let Err(motif) = gouvernance.executer(&intent, |_| ()) {
        return serde_json::json!({ "success": false, "error": motif });
    }
    match noter(carnet, p, contenu_etranger).await {
        Ok(Notee::Ecrite(id)) => serde_json::json!({ "success": true, "ecrite": id.as_str() }),
        Ok(Notee::Candidat(k)) => serde_json::json!({
            "success": true,
            "candidat": k.to_string(),
            "message": "proposition en attente de validation par l'utilisateur",
        }),
        Err(e) => serde_json::json!({ "success": false, "error": e.to_string() }),
    }
}

#[cfg(test)]
#[allow(clippy::assert_is_empty, clippy::many_single_char_names)]
mod tests {
    use super::*;
    use maia_memoire::{BorneOctets, EntrepotMemoire, MemoireMaia};
    use std::sync::Arc;

    async fn vue() -> (Arc<MemoireMaia<EntrepotMemoire>>, VueMaia<EntrepotMemoire>) {
        let m = Arc::new(
            MemoireMaia::ouvrir(EntrepotMemoire::new(), Arc::new(BorneOctets))
                .await
                .expect("ouvrir"),
        );
        (Arc::clone(&m), VueMaia::new(m))
    }

    fn params(v: serde_json::Value) -> HashMap<String, serde_json::Value> {
        serde_json::from_value(v).expect("params")
    }

    fn note() -> HashMap<String, serde_json::Value> {
        params(
            serde_json::json!({ "branche": "travail.cog", "type": "fait", "texte": "Prépare le lot 8", "importance": 4 }),
        )
    }

    /// Agent intégré qui déclare la compétence `memoire` (F6).
    fn alicia() -> AgentDef {
        crate::agents::AgentRegistry::new()
            .get_agent("alicia")
            .expect("alicia")
    }

    /// Domaine « travail » établi par l'utilisateur (F4).
    async fn domaine_travail(m: &MemoireMaia<EntrepotMemoire>) {
        m.utilisateur()
            .ecrire(maia_memoire::Ecrit::new(
                Branche::parse("travail.lot").expect("b"),
                TypeUnite::Fait,
                "Lot 7",
                5,
            ))
            .await
            .expect("écrire");
    }

    const POLITIQUE_NOTER: &str = r#"
version = 1

[[policy]]
id = "maia-memoire-noter"
type = "permission"
effect = "authorize"
condition = "data.operation == 'memoire.noter'"
rule = "data.agent != '' && data.competence_memoire == 'true'"
[policy.target]
actions = ["modification"]
origins = ["maia"]
"#;

    #[tokio::test]
    async fn tour_sain_note_ordinaire_ecrite() {
        let (m, v) = vue().await;
        domaine_travail(&m).await;
        let g = crate::tamr::tests::gouvernance(POLITIQUE_NOTER);
        let r = executer(&g, Some(&v), &alicia(), &note(), false).await;
        assert_eq!(r["success"], true, "{r}");
        assert_eq!(r["ecrite"], "travail.cog.f1");
        assert_eq!(m.utilisateur().consulter().await.expect("c").len(), 2);
    }

    #[tokio::test]
    async fn agent_sans_competence_memoire_refuse() {
        // F6 : sans la compétence `memoire`, rien n'est écrit ni proposé, même
        // si une politique StrongFather permissive l'autoriserait.
        let (m, v) = vue().await;
        domaine_travail(&m).await;
        let permissive = POLITIQUE_NOTER.replace(" && data.competence_memoire == 'true'", "");
        let g = crate::tamr::tests::gouvernance(&permissive);
        for id in ["maria", "victor", "denis"] {
            let agent = crate::agents::AgentRegistry::new()
                .get_agent(id)
                .expect("agent");
            assert!(!competence_memoire(&agent), "{id}");
            let r = executer(&g, Some(&v), &agent, &note(), false).await;
            assert_eq!(r["success"], false, "{id} : {r}");
        }
        assert_eq!(m.utilisateur().consulter().await.expect("c").len(), 1);
        assert!(m.utilisateur().candidats().await.expect("k").is_empty());
        let i = intention_noter(&alicia(), false);
        assert_eq!(i.data.content["competence_memoire"], "true");
    }

    #[tokio::test]
    async fn tour_etranger_toujours_candidat() {
        let (m, v) = vue().await;
        domaine_travail(&m).await;
        let g = crate::tamr::tests::gouvernance(POLITIQUE_NOTER);
        let r = executer(&g, Some(&v), &alicia(), &note(), true).await;
        assert_eq!(r["success"], true, "{r}");
        assert!(
            r.get("candidat").is_some() && r.get("ecrite").is_none(),
            "{r}"
        );
        let u = m.utilisateur();
        assert_eq!(
            u.consulter().await.expect("c").len(),
            1,
            "aucune écriture directe"
        );
        let k = u.candidats().await.expect("k");
        assert_eq!(k.len(), 1);
        assert_eq!(k[0].provenance, Provenance::Etranger);
        assert!(k[0].confiance <= 4);
        // Même via la fonction : une proposition déduite devient candidate dans un tour étranger.
        let p = proposition(&note(), false).expect("p");
        assert!(matches!(noter(&v, p, true).await, Ok(Notee::Candidat(_))));
        assert_eq!(u.consulter().await.expect("c").len(), 1);
    }

    #[tokio::test]
    async fn strongfather_absent_ou_sans_regle_rien_ecrit() {
        let (m, v) = vue().await;
        let agent = alicia();
        for g in [
            GouvernanceMaia::depuis_fichier(std::env::temp_dir().join("maia-noter-absent.toml")),
            crate::tamr::tests::gouvernance(crate::tamr::tests::POLITIQUE_SANS_TAMR),
        ] {
            let r = executer(&g, Some(&v), &agent, &note(), false).await;
            assert_eq!(r["success"], false, "{r}");
        }
        assert!(m.utilisateur().consulter().await.expect("c").is_empty());
        assert!(m.utilisateur().candidats().await.expect("k").is_empty());
        let g = crate::tamr::tests::gouvernance(POLITIQUE_NOTER);
        let r = executer(&g, None, &agent, &note(), false).await;
        assert_eq!(r["success"], false);
    }

    #[test]
    fn politiques_de_base_competence_et_refus_critique() {
        // F6 (config/strongfather/maia.toml) : compétence exigée ; écriture directe
        // annoncée dans un tour étranger ⇒ refus critique.
        let g = crate::gouvernance::tests::gouvernance_de_base();
        assert!(g
            .executer(&intention_noter(&alicia(), false), |_| ())
            .is_ok());
        assert!(
            g.executer(&intention_noter(&alicia(), true), |_| ())
                .is_ok(),
            "candidat seulement : autorisé"
        );
        let maria = crate::agents::AgentRegistry::new()
            .get_agent("maria")
            .expect("maria");
        assert!(g.executer(&intention_noter(&maria, false), |_| ()).is_err());
        let mut incoherente = intention_noter(&alicia(), true);
        incoherente
            .data
            .content
            .insert("ecriture_directe_possible".into(), "true".into());
        let e = g.executer(&incoherente, |_| ()).unwrap_err();
        assert!(e.contains("maia-memoire-noter-etranger-direct"), "{e}");
    }

    #[test]
    fn arguments_stricts_regle_refusee() {
        let mut p = note();
        p.insert("type".into(), serde_json::json!("regle"));
        assert!(proposition(&p, false).is_err());
        let mut p = note();
        p.insert("provenance".into(), serde_json::json!("utilisateur"));
        assert!(proposition(&p, false).is_err());
        assert_eq!(
            proposition(&note(), true).expect("p").provenance,
            Provenance::Etranger
        );
        assert_eq!(definition_noter().function.name, NOM_OUTIL_NOTER);
    }
}
