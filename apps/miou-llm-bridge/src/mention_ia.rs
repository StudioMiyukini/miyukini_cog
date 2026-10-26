//! @id     miou_llm_bridge_mention_ia
//! @do     declarer_maia_comme_ia
//! @role   conformite
//! @layer  app
//! @human  MAIA — transparence AI Act art. 50 : MAIA se présente comme une IA, la consigne système l'impose, chaque réponse générée est marquée de façon lisible par machine.

//! Transparence (règlement (UE) 2024/1689, « AI Act », art. 50)
//!
//! - §1 : la personne est informée qu'elle interagit avec une IA →
//!   [`PRESENTATION`] (route publique `GET /maia/presentation`) et
//!   [`consigne_systeme`] (le modèle ne se fait jamais passer pour un humain) ;
//! - §2 : les contenus générés sont marqués de façon lisible par machine →
//!   [`marquer`] ajoute `genere_par_ia` et `mention_ia` à chaque réponse.

use serde_json::Value;

/// Texte de présentation de MAIA (affiché à la première interaction).
pub const PRESENTATION: &str = "Je suis MAIA, une intelligence artificielle (IA) qui fonctionne \
    localement sur votre COG, sans service d'IA extérieur. Mes réponses sont générées \
    automatiquement par un modèle de langue : elles peuvent contenir des erreurs, vérifiez \
    les informations importantes.";

/// Mention courte jointe à chaque réponse générée.
pub const MENTION_IA: &str = "Réponse générée par une IA (MAIA, modèle local).";

/// Consigne système placée **avant** celle de l'agent : identité d'IA non
/// négociable (le prompt de l'agent ne peut pas la lever).
pub fn consigne_systeme(nom_agent: &str) -> String {
    format!(
        "Tu es {nom_agent}, un agent de MAIA, une intelligence artificielle (IA) qui fonctionne \
         localement sur le COG de l'utilisateur. Tu ne prétends jamais être un humain : si on te \
         demande qui ou ce que tu es, tu réponds que tu es une IA. Tu réponds en français sauf \
         demande contraire.\n\n"
    )
}

/// Marque une réponse JSON comme contenu généré par une IA (objet seulement).
pub fn marquer(mut reponse: Value, genere_par_ia: bool) -> Value {
    if let Some(o) = reponse.as_object_mut() {
        o.insert("genere_par_ia".into(), Value::Bool(genere_par_ia));
        o.insert(
            "mention_ia".into(),
            Value::String(if genere_par_ia {
                MENTION_IA.to_string()
            } else {
                "Réponse automatique de secours de MAIA (IA indisponible).".to_string()
            }),
        );
    }
    reponse
}

/// `GET /maia/presentation` — présentation publique de MAIA comme IA.
pub async fn presentation_handler() -> axum::Json<Value> {
    axum::Json(serde_json::json!({
        "nom": "MAIA",
        "nature": "intelligence artificielle",
        "presentation": PRESENTATION,
        "moteur": "maia-llm (llama.cpp, local)",
        "service_ia_exterieur": false,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maia_se_declare_comme_ia() {
        assert!(PRESENTATION.contains("intelligence artificielle"));
        let c = consigne_systeme("Miou");
        assert!(c.contains("Miou") && c.contains("une IA") && c.contains("jamais être un humain"));
    }

    #[test]
    fn reponses_marquees() {
        let r = marquer(serde_json::json!({"content": "x"}), true);
        assert_eq!(r["genere_par_ia"], true);
        assert_eq!(r["mention_ia"], MENTION_IA);
        let secours = marquer(serde_json::json!({}), false);
        assert_eq!(secours["genere_par_ia"], false);
        // Une valeur non objet n'est pas altérée.
        assert_eq!(marquer(Value::Null, true), Value::Null);
    }

    #[tokio::test]
    async fn presentation_publique() {
        let axum::Json(v) = presentation_handler().await;
        assert_eq!(v["service_ia_exterieur"], false);
        assert!(v["presentation"].as_str().unwrap().contains("IA"));
    }
}
