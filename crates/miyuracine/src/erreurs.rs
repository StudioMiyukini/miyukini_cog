//! @id     miyuracine_erreurs
//! @do     definir_erreurs_racine
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « erreurs ».

//! Motifs de refus. Toute erreur est un refus (fail-closed) : aucune
//! opération partielle n'est rendue à l'appelant.
//!
//! Les messages ne contiennent jamais de matériel secret.

use std::fmt;

/// Une variante par cause de refus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurRacine {
    /// Paramètres de partage hors spécification (seuils, nombre de parts…).
    ParametresInvalides(&'static str),
    /// Longueur du secret maître invalide (≥ 128 bits, multiple de 16 bits).
    LongueurSecretInvalide,
    /// Phrase secrète hors ASCII imprimable (codes 32 à 126).
    PhraseSecreteInvalide,
    /// Mot absent de la liste officielle SLIP-39.
    MotInconnu,
    /// Nombre de mots d'une part invalide (trop court ou remplissage > 8 bits).
    LongueurPartInvalide,
    /// Somme de contrôle RS1024 invalide.
    SommeControleInvalide,
    /// Bits de remplissage non nuls.
    RemplissageInvalide,
    /// Seuil de groupes supérieur au nombre de groupes.
    SeuilGroupesSuperieurAuNombre,
    /// Aucune part fournie.
    AucunePart,
    /// Parts d'origines différentes (identifiant, exposant, drapeau, seuil ou nombre de groupes).
    PartsIncompatibles,
    /// Longueurs de valeurs différentes entre parts.
    LongueursDifferentes,
    /// Seuils de membres différents dans un même groupe.
    SeuilsMembresDifferents,
    /// Indices de parts dupliqués.
    IndicesDupliques,
    /// Nombre de groupes différent du seuil de groupes.
    NombreGroupesInvalide,
    /// Nombre de parts d'un groupe différent de son seuil.
    NombrePartsInvalide,
    /// Empreinte de contrôle du secret partagé invalide (part falsifiée ou mélangée).
    EmpreinteInvalide,
    /// Parts valides SLIP-39 mais pas au schéma de la racine COG (1 groupe, 2 sur N, 256 bits).
    SchemaRacineInattendu,
    /// Indice de dérivation hors plage (≥ 2^31 avant durcissement).
    IndiceHorsPlage,
    /// Graine SLIP-10 hors plage (128 à 512 bits).
    LongueurGraineInvalide,
    /// Génération invalide pour cet usage (l'identité n'a qu'une génération, 0).
    GenerationInvalide,
    /// Usage inconnu.
    UsageInconnu,
    /// Générateur aléatoire du système indisponible.
    AleaIndisponible,
    /// Passation : les deux racines sont identiques.
    PassationRacinesIdentiques,
    /// Passation : la racine annoncée ne correspond pas à l'ancienne racine attendue.
    PassationAncienneRacineInattendue,
    /// Erreur du format Sceau (scellement ou vérification).
    Sceau(miyusceau::ErreurSceau),
}

impl fmt::Display for ErreurRacine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParametresInvalides(m) => write!(f, "paramètres de partage invalides : {m}"),
            Self::LongueurSecretInvalide => {
                f.write_str("longueur du secret maître invalide (≥ 128 bits, multiple de 16 bits)")
            }
            Self::PhraseSecreteInvalide => f.write_str("phrase secrète hors ASCII imprimable"),
            Self::MotInconnu => f.write_str("mot absent de la liste SLIP-39"),
            Self::LongueurPartInvalide => f.write_str("longueur de part invalide"),
            Self::SommeControleInvalide => f.write_str("somme de contrôle de la part invalide"),
            Self::RemplissageInvalide => f.write_str("remplissage de la part invalide"),
            Self::SeuilGroupesSuperieurAuNombre => {
                f.write_str("seuil de groupes supérieur au nombre de groupes")
            }
            Self::AucunePart => f.write_str("aucune part fournie"),
            Self::PartsIncompatibles => f.write_str(
                "parts incompatibles : identifiant, exposant, seuil ou nombre de groupes différents",
            ),
            Self::LongueursDifferentes => f.write_str("parts de longueurs différentes"),
            Self::SeuilsMembresDifferents => f.write_str("seuils de membres différents dans un groupe"),
            Self::IndicesDupliques => f.write_str("indices de parts dupliqués"),
            Self::NombreGroupesInvalide => f.write_str("nombre de groupes différent du seuil"),
            Self::NombrePartsInvalide => f.write_str("nombre de parts différent du seuil"),
            Self::EmpreinteInvalide => f.write_str("empreinte du secret invalide : part falsifiée ou mélangée"),
            Self::SchemaRacineInattendu => {
                f.write_str("parts hors schéma de la racine COG (1 groupe, 2 sur N, 256 bits)")
            }
            Self::IndiceHorsPlage => f.write_str("indice de dérivation hors plage (≥ 2^31)"),
            Self::LongueurGraineInvalide => f.write_str("graine SLIP-10 hors plage (128 à 512 bits)"),
            Self::GenerationInvalide => f.write_str("génération invalide pour cet usage"),
            Self::UsageInconnu => f.write_str("usage inconnu"),
            Self::AleaIndisponible => f.write_str("générateur aléatoire du système indisponible"),
            Self::PassationRacinesIdentiques => f.write_str("passation : ancienne et nouvelle racines identiques"),
            Self::PassationAncienneRacineInattendue => {
                f.write_str("passation : ancienne racine différente de la racine attendue")
            }
            Self::Sceau(e) => write!(f, "sceau : {e}"),
        }
    }
}

impl std::error::Error for ErreurRacine {}

impl From<miyusceau::ErreurSceau> for ErreurRacine {
    fn from(e: miyusceau::ErreurSceau) -> Self {
        Self::Sceau(e)
    }
}
