//! @id     bondingbrother_test_porte_bout_en_bout
//! @do     verifier_porte_obligatoire
//! @role   test
//! @layer  core
//! @human  Critère de sortie du lot C : une intention refusée par StrongFather n'exécute pas
//! @human  l'action ; une intention autorisée l'exécute exactement une fois.

use std::cell::Cell;
use std::collections::HashMap;

use bondingbrother::strongfather::{
    ActionType, CallContext, Intent, IntentData, PolicySet, Verdict, DEFAULT_DENY_RULE,
};
use bondingbrother::{executer_si_autorise, Gate, PorteStrongFather};

const POLITIQUES: &str = r#"
version = 1

# MAIA peut lire les comptes JayKonta.
[[policy]]
id = "maia-lit-jaykonta"
type = "permission"
effect = "authorize"
rule = "caller == 'maia'"
[policy.target]
actions = ["read"]
subjects = ["jaykonta"]

# MAIA peut créer une écriture si le montant est raisonnable.
[[policy]]
id = "maia-ecrit-jaykonta"
type = "permission"
effect = "authorize"
rule = "caller == 'maia'"
[policy.target]
actions = ["creation"]
subjects = ["jaykonta"]

[[policy]]
id = "plafond-ecriture"
type = "constraint"
effect = "constrain"
priority = "critical"
rule = "has(data.montant) && data.montant <= 1000"
[policy.target]
actions = ["creation"]

# Aucune suppression hors zone H0.
[[policy]]
id = "pas-de-suppression-hors-h0"
type = "permission"
effect = "deny"
rule = "!(has(meta.zone) && meta.zone == 'H0')"
[policy.target]
actions = ["deletion"]

[[policy]]
id = "maia-supprime-jaykonta"
type = "permission"
effect = "authorize"
rule = "caller == 'maia'"
[policy.target]
actions = ["deletion"]
subjects = ["jaykonta"]
"#;

fn porte() -> PorteStrongFather {
    PorteStrongFather::depuis_politiques(PolicySet::from_toml_str(POLITIQUES).unwrap()).unwrap()
}

fn intent(action: ActionType, subject: &str, caller: &str, data: &[(&str, &str)]) -> Intent {
    Intent::new(
        format!("req-{subject}-{caller}"),
        action,
        subject.to_string(),
        CallContext {
            caller_identity: caller.to_string(),
            origin: "maia".to_string(),
            instance: "cog-local".to_string(),
        },
        IntentData {
            content: data
                .iter()
                .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
                .collect(),
        },
    )
}

/// Exécute l'intention via la porte et rend (nombre d'exécutions de l'action, résultat).
fn executer(porte: &dyn Gate, i: &Intent) -> (u32, Result<String, bondingbrother::Refus>) {
    let compteur = Cell::new(0u32);
    let r = executer_si_autorise(porte, i, |jeton| {
        compteur.set(compteur.get() + 1);
        assert!(
            jeton.couvre(i),
            "le jeton doit couvrir l'intention exécutée"
        );
        jeton.regle().to_string()
    });
    (compteur.get(), r)
}

/// @id bondingbrother_test_refus_n_execute_pas
/// @role test
/// @layer core
/// @do verify_denied_action_not_run
//@human / @human Intention refusée ⇒ l'action n'est jamais exécutée (compteur à 0).
#[test]
fn intention_refusee_n_execute_pas_l_action() {
    let p = porte();
    let cas: Vec<(Intent, &str)> = vec![
        // Aucun Authorize : refus par défaut.
        (
            intent(ActionType::Modification, "jaykonta", "maia", &[]),
            DEFAULT_DENY_RULE,
        ),
        // Pas de confusion par sous-chaîne.
        (
            intent(ActionType::Read, "jaykonta-evil", "maia", &[]),
            DEFAULT_DENY_RULE,
        ),
        // Mauvais appelant.
        (
            intent(ActionType::Read, "jaykonta", "alicia", &[]),
            DEFAULT_DENY_RULE,
        ),
        // Contrainte violée malgré l'Authorize.
        (
            intent(
                ActionType::Creation,
                "jaykonta",
                "maia",
                &[("montant", "5000")],
            ),
            "plafond-ecriture",
        ),
        // Contrainte : montant absent ⇒ règle fausse ⇒ refus.
        (
            intent(ActionType::Creation, "jaykonta", "maia", &[]),
            "plafond-ecriture",
        ),
        // Contrainte : montant non numérique ⇒ erreur de type ⇒ refus (fail-closed).
        (
            intent(
                ActionType::Creation,
                "jaykonta",
                "maia",
                &[("montant", "beaucoup")],
            ),
            "plafond-ecriture",
        ),
        // Deny explicite > Authorize (suppression sans zone).
        (
            intent(ActionType::Deletion, "jaykonta", "maia", &[]),
            "pas-de-suppression-hors-h0",
        ),
        // Intention structurellement invalide.
        (
            intent(ActionType::Read, "jaykonta", "", &[]),
            "validation-structurelle",
        ),
    ];
    for (i, regle) in cas {
        let (executions, r) = executer(&p, &i);
        assert_eq!(executions, 0, "action exécutée malgré le refus : {i:?}");
        let refus = r.unwrap_err();
        assert_eq!(refus.regle(), regle, "règle décisive pour {i:?}");
        assert_eq!(refus.decision.verdict(), Verdict::Deny);
        assert!(refus.decision.matches_intent(&i));
    }
}

/// @id bondingbrother_test_autorise_execute_une_fois
/// @role test
/// @layer core
/// @do verify_allowed_action_runs_once
//@human / @human Intention autorisée ⇒ l'action est exécutée exactement une fois, avec le jeton.
#[test]
fn intention_autorisee_execute_l_action_une_fois() {
    let p = porte();
    let mut zone = intent(ActionType::Deletion, "jaykonta", "maia", &[]);
    zone.metadata = HashMap::from([("zone".to_string(), "H0".to_string())]);
    let cas = vec![
        (
            intent(ActionType::Read, "jaykonta", "maia", &[]),
            "maia-lit-jaykonta",
        ),
        (
            intent(
                ActionType::Creation,
                "jaykonta",
                "maia",
                &[("montant", "999")],
            ),
            "maia-ecrit-jaykonta",
        ),
        (zone, "maia-supprime-jaykonta"),
    ];
    for (i, regle) in cas {
        let (executions, r) = executer(&p, &i);
        assert_eq!(executions, 1, "l'action doit s'exécuter une fois : {i:?}");
        assert_eq!(r.unwrap(), regle);
    }
}

/// @id bondingbrother_test_fichier_invalide
/// @role test
/// @layer core
/// @do verify_invalid_policy_file_blocks
//@human / @human Fichier de politiques absent ou invalide ⇒ aucune porte (donc aucune action possible).
#[test]
fn fichier_de_politiques_invalide_bloque() {
    let dir = std::env::temp_dir().join(format!("bb-porte-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    assert!(PorteStrongFather::depuis_fichier(dir.join("absent.toml")).is_err());
    let vide = dir.join("vide.toml");
    std::fs::write(&vide, "version = 1\n").unwrap();
    assert!(PorteStrongFather::depuis_fichier(&vide).is_err());
    let casse = dir.join("casse.json");
    std::fs::write(&casse, "{ \"version\": 1, \"policy\": [ ").unwrap();
    assert!(PorteStrongFather::depuis_fichier(&casse).is_err());
    let ok = dir.join("ok.toml");
    std::fs::write(&ok, POLITIQUES).unwrap();
    let p = PorteStrongFather::depuis_fichier(&ok).unwrap();
    let (executions, _) = executer(&p, &intent(ActionType::Read, "jaykonta", "maia", &[]));
    assert_eq!(executions, 1);
    let _ = std::fs::remove_dir_all(&dir);
}

/// @id bondingbrother_test_porte_partagee
/// @role test
/// @layer core
/// @do verify_gate_shareable
//@human / @human La porte se partage (Arc, référence, clone) sans changer les décisions.
#[test]
fn porte_partagee_et_clonee() {
    let partagee = std::sync::Arc::new(porte());
    let clone = (*partagee).clone();
    let lecture = intent(ActionType::Read, "jaykonta", "maia", &[]);
    assert!(partagee.autoriser(&lecture).is_ok());
    assert!(clone.autoriser(&lecture).is_ok());
    // Implémentation générique `Gate for &G` (référence vers un objet-trait).
    let objet: &dyn Gate = &clone;
    assert!(executer_si_autorise(&objet, &lecture, |_| ()).is_ok());
    let jeton_arc = partagee.autoriser(&lecture).unwrap();
    let jeton_clone = clone.autoriser(&lecture).unwrap();
    assert_eq!(jeton_arc.decision(), jeton_clone.decision());
}
