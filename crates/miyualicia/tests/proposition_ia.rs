//! Alicia : l'IA locale **propose**, les Cores **disposent** (COG-POSTE-100 A3, A4).
//!
//! Tests sans modèle : porte des Cores (politiques réelles `config/strongfather/alicia.toml`
//! ou indisponibles), repli regex, refus d'un moteur hors boucle locale, mention IA.
//!
//! Test avec un vrai modèle (ignoré par défaut) :
//!
//! ```text
//! ALICIA_MOTEUR_URL=http://127.0.0.1:11435 ALICIA_CLE=/chemin/graine-alicia.hex \
//!   cargo test -p miyualicia --test proposition_ia -- --ignored --nocapture
//! ```
//!
//! `ALICIA_MOTEUR_URL` désigne maia-llm (modèle signé chargé, clé publique de
//! `ALICIA_CLE` épinglée dans `appelants_inference`). Pour un **essai seulement**,
//! l'URL peut viser un autre moteur compatible OpenAI en boucle locale
//! (ex. koboldcpp) : ce test n'est jamais le chemin de production.

use std::path::PathBuf;
use std::sync::Arc;

use miyualicia::nlu_local::{MoteurLocalConfig, NluLocal, PieceConnue, VocabulaireNlu};
use miyualicia::{AliciaConfig, AliciaError, AliciaService, Intent, PropositionIa};
use miyualicia_devices::CommandSource;
use miyualicia_dispatcher::StrongFatherCoreGate;

fn politiques_alicia() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/strongfather/alicia.toml")
}

fn service() -> AliciaService {
    AliciaService::new(AliciaConfig {
        mqtt: None,
        ..Default::default()
    })
}

fn proposition(intent: Intent) -> PropositionIa {
    PropositionIa {
        intent,
        genere_par_ia: true,
        mention_ia: miyualicia::MENTION_IA_ALICIA,
        modele: "essai".into(),
    }
}

fn lumiere_salon() -> Intent {
    Intent::ControlDevice {
        device_type: "light".into(),
        room_id: Some("salon".into()),
        action: "on".into(),
        value: None,
    }
}

#[tokio::test]
async fn sans_politiques_la_proposition_est_refusee() {
    let s = service();
    let gate = StrongFatherCoreGate::indisponible("essai : aucune politique");
    let r = s
        .executer_proposition(
            &proposition(lumiere_salon()),
            &gate,
            "alicia",
            CommandSource::Voice,
        )
        .await;
    assert!(matches!(r, Err(AliciaError::RefusParLesCores(_))), "{r:?}");
    assert!(
        s.recent_activity(10).await.is_empty(),
        "rien n'a été exécuté"
    );
}

#[tokio::test]
async fn politique_reelle_autorise_le_pilotage_puis_l_execution_continue() {
    let s = service();
    let gate = StrongFatherCoreGate::depuis_fichier(politiques_alicia());
    let r = s
        .executer_proposition(
            &proposition(lumiere_salon()),
            &gate,
            "alicia",
            CommandSource::Voice,
        )
        .await;
    // Les Cores ont autorisé : l'échec vient de l'absence d'appareil, pas de la porte.
    assert!(
        matches!(r, Err(ref e) if !matches!(e, AliciaError::RefusParLesCores(_))),
        "{r:?}"
    );
}

#[tokio::test]
async fn l_ia_ne_deverrouille_jamais_une_porte_sous_la_politique_de_base() {
    let s = service();
    let gate = StrongFatherCoreGate::depuis_fichier(politiques_alicia());
    for action in ["unlock", "lock"] {
        let r = s
            .executer_proposition(
                &proposition(Intent::ControlDevice {
                    device_type: "lock".into(),
                    room_id: None,
                    action: action.into(),
                    value: None,
                }),
                &gate,
                "alicia",
                CommandSource::Voice,
            )
            .await;
        assert!(
            matches!(r, Err(AliciaError::RefusParLesCores(_))),
            "{action} : {r:?}"
        );
    }
}

#[tokio::test]
async fn intention_de_lecture_jamais_soumise_ni_executee() {
    let s = service();
    let gate = StrongFatherCoreGate::indisponible("non consultée");
    let r = s
        .executer_proposition(
            &proposition(Intent::QueryState {
                target: "salon".into(),
                property: Some("temperature".into()),
            }),
            &gate,
            "alicia",
            CommandSource::Voice,
        )
        .await;
    assert!(matches!(r, Ok(None)));
}

#[tokio::test]
async fn sans_moteur_repli_regex_marque_sans_ia() {
    let p = service()
        .proposer_intention("allume la lumière du salon")
        .await;
    assert!(!p.genere_par_ia);
    assert_eq!(p.intent, lumiere_salon());
}

#[tokio::test]
async fn moteur_hors_boucle_locale_ignore() {
    let s = AliciaService::new(AliciaConfig {
        mqtt: None,
        moteur_ia: Some(MoteurLocalConfig {
            url: "https://api.exemple.com".into(),
            timeout_ms: 200,
        }),
        ..Default::default()
    });
    // Aucun appel extérieur : repli regex immédiat.
    let p = s.proposer_intention("éteins la lumière du salon").await;
    assert!(!p.genere_par_ia);
}

// ── Avec un vrai modèle ────────────────────────────────────────────────

/// Jeu de phrases françaises et intention attendue.
fn jeu_de_phrases() -> Vec<(&'static str, Intent)> {
    let ctrl = |t: &str, r: Option<&str>, a: &str, v: Option<f64>| Intent::ControlDevice {
        device_type: t.into(),
        room_id: r.map(Into::into),
        action: a.into(),
        value: v.map(|x| serde_json::json!(x)),
    };
    vec![
        (
            "allume la lumière du salon",
            ctrl("light", Some("salon"), "on", None),
        ),
        (
            "éteins la lumière du salon",
            ctrl("light", Some("salon"), "off", None),
        ),
        (
            "allume la lampe de la chambre parentale",
            ctrl("light", Some("chambre-parentale"), "on", None),
        ),
        (
            "éteins la lumière dans la chambre de Theresa",
            ctrl("light", Some("chambre-theresa"), "off", None),
        ),
        (
            "mets la lumière du salon à 40 %",
            ctrl("light", Some("salon"), "set_brightness", Some(40.0)),
        ),
        (
            "ferme les volets du salon",
            ctrl("shutter", Some("salon"), "close", None),
        ),
        (
            "ouvre les volets de la chambre d'Eleanore",
            ctrl("shutter", Some("chambre-eleanore"), "open", None),
        ),
        (
            "mets le chauffage à 19 degrés",
            ctrl("thermostat", None, "set_temperature", Some(19.0)),
        ),
        (
            "règle le thermostat du salon sur 21",
            ctrl("thermostat", Some("salon"), "set_temperature", Some(21.0)),
        ),
        (
            "allume la prise du salon",
            ctrl("outlet", Some("salon"), "on", None),
        ),
        (
            "verrouille la porte d'entrée",
            ctrl("lock", None, "lock", None),
        ),
        ("déverrouille la porte", ctrl("lock", None, "unlock", None)),
        (
            "quelle température fait-il dans le salon ?",
            Intent::QueryState {
                target: "salon".into(),
                property: Some("temperature".into()),
            },
        ),
        (
            "quel temps fera-t-il demain ?",
            Intent::QueryWeather {
                location: None,
                horizon: Some("tomorrow".into()),
            },
        ),
        (
            "lance la routine bonne nuit",
            Intent::ActivateRoutine {
                routine_name: "bonne nuit".into(),
            },
        ),
        (
            "je pars, active la routine je pars",
            Intent::ActivateRoutine {
                routine_name: "je pars".into(),
            },
        ),
        (
            "commande-moi une pizza",
            Intent::Unknown {
                transcript: "commande-moi une pizza".into(),
            },
        ),
        (
            "raconte-moi une blague",
            Intent::Unknown {
                transcript: "raconte-moi une blague".into(),
            },
        ),
    ]
}

/// Égalité « métier » : les valeurs numériques comparées en flottant.
fn conforme(obtenu: &Intent, attendu: &Intent) -> bool {
    match (obtenu, attendu) {
        (
            Intent::ControlDevice {
                device_type: t1,
                room_id: r1,
                action: a1,
                value: v1,
            },
            Intent::ControlDevice {
                device_type: t2,
                room_id: r2,
                action: a2,
                value: v2,
            },
        ) => {
            t1 == t2
                && r1 == r2
                && a1 == a2
                && v1.as_ref().and_then(serde_json::Value::as_f64)
                    == v2.as_ref().and_then(serde_json::Value::as_f64)
        }
        (Intent::QueryWeather { horizon: h1, .. }, Intent::QueryWeather { horizon: h2, .. }) => {
            h1 == h2
        }
        _ => obtenu == attendu,
    }
}

#[tokio::test]
#[ignore = "exige un moteur réel : ALICIA_MOTEUR_URL (+ ALICIA_CLE pour maia-llm)"]
async fn intentions_francaises_avec_le_vrai_modele() {
    let url = std::env::var("ALICIA_MOTEUR_URL").expect("ALICIA_MOTEUR_URL");
    let defaut = AliciaConfig::default();
    let vocab = VocabulaireNlu::new(
        defaut
            .rooms
            .iter()
            .map(|r| PieceConnue {
                id: r.id.clone(),
                nom: r.name.clone(),
            })
            .collect(),
        vec!["bonne nuit".into(), "je pars".into()],
    );
    let mut n = NluLocal::new(
        MoteurLocalConfig {
            url,
            timeout_ms: 120_000,
        },
        vocab,
        reqwest::Client::new(),
    );
    if let Ok(chemin) = std::env::var("ALICIA_CLE") {
        let cle = miyauth::http::service::charger_ou_creer_cle(std::path::Path::new(&chemin))
            .expect("clé d'Alicia");
        println!(
            "clé publique d'Alicia (appelants_inference) : {}",
            miyauth::http::service::encoder_cle_publique(&cle.cle_publique())
        );
        n = n.avec_cle(Arc::new(cle));
    }
    let jeu = jeu_de_phrases();
    let mut reussites = 0usize;
    let mut refus_schema = 0usize;
    let debut = std::time::Instant::now();
    for (phrase, attendu) in &jeu {
        let t = std::time::Instant::now();
        match n.proposer(phrase).await {
            Ok(p) => {
                assert!(p.genere_par_ia);
                let ok = conforme(&p.intent, attendu);
                reussites += usize::from(ok);
                println!(
                    "{} {:>5} ms  « {phrase} » → {}",
                    if ok { "OK " } else { "KO " },
                    t.elapsed().as_millis(),
                    serde_json::to_string(&p.intent).unwrap_or_default()
                );
            }
            Err(e) => {
                refus_schema += 1;
                println!("ERR {:>5} ms  « {phrase} » → {e}", t.elapsed().as_millis());
            }
        }
    }
    let taux = reussites as f64 / jeu.len() as f64;
    println!(
        "TAUX {reussites}/{} = {:.0} % ; erreurs (moteur ou schéma) : {refus_schema} ; durée totale {} ms",
        jeu.len(),
        taux * 100.0,
        debut.elapsed().as_millis()
    );
    assert!(taux >= 0.75, "taux de réussite insuffisant : {taux:.2}");
}
