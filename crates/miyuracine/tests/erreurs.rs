//! @id     miyuracine_tests_erreurs
//! @do     verifier_messages_refus
//! @role   test
//! @layer  toolkit
//! @human  miyuracine — chaque motif de refus a un message propre, sans matériel secret.

use std::collections::HashSet;

use miyuracine::ErreurRacine;

fn toutes() -> Vec<ErreurRacine> {
    use ErreurRacine::*;
    vec![
        ParametresInvalides("seuil"),
        LongueurSecretInvalide,
        PhraseSecreteInvalide,
        MotInconnu,
        LongueurPartInvalide,
        SommeControleInvalide,
        RemplissageInvalide,
        SeuilGroupesSuperieurAuNombre,
        AucunePart,
        PartsIncompatibles,
        LongueursDifferentes,
        SeuilsMembresDifferents,
        IndicesDupliques,
        NombreGroupesInvalide,
        NombrePartsInvalide,
        EmpreinteInvalide,
        SchemaRacineInattendu,
        IndiceHorsPlage,
        LongueurGraineInvalide,
        GenerationInvalide,
        UsageInconnu,
        AleaIndisponible,
        PassationRacinesIdentiques,
        PassationAncienneRacineInattendue,
        Sceau(miyusceau::ErreurSceau::SignatureInvalide),
        FichierCleInvalide,
        FichierCleRefuse("lien symbolique"),
        FichierSecretRefuse("lien symbolique"),
        ParametresArgon2HorsBornes,
        MotDePasseTropCourt,
        EntreeSortie(std::io::ErrorKind::NotFound),
        Durcissement("core dumps"),
        VerrouillageMemoire,
    ]
}

#[test]
fn messages_distincts_et_non_vides() {
    let erreurs = toutes();
    let messages: HashSet<String> = erreurs.iter().map(ToString::to_string).collect();
    assert_eq!(
        messages.len(),
        erreurs.len(),
        "deux motifs partagent un message"
    );
    assert!(messages.iter().all(|m| !m.trim().is_empty()));
    // Le détail statique est repris tel quel, le contexte Sceau est préfixé.
    assert!(ErreurRacine::FichierCleRefuse("lien symbolique")
        .to_string()
        .contains("lien symbolique"));
    let sceau: ErreurRacine = miyusceau::ErreurSceau::SignatureInvalide.into();
    assert_eq!(
        sceau,
        ErreurRacine::Sceau(miyusceau::ErreurSceau::SignatureInvalide)
    );
    assert!(sceau.to_string().starts_with("sceau : "));
    let _: &dyn std::error::Error = &sceau;
}

#[test]
fn fichier_cle_invalide_ne_distingue_pas_les_causes() {
    // Mot de passe faux, fichier altéré, format inconnu : un seul message (fail-closed).
    let m = ErreurRacine::FichierCleInvalide.to_string();
    for cause in ["mot de passe", "altéré", "format"] {
        assert!(m.contains(cause), "{m}");
    }
}
