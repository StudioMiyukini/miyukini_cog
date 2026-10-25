//! @id     miyuvigie_signal
//! @do     typer_signaux_vigies
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « signal » : signaux typés émis par les Vigies.

//! Signaux typés produits par une Vigie et transmis à CaringNanny, qui les
//! propage sans altération (INV-CN-7) à WorrySentinel.
//!
//! - [`Signal`] : une détection (règle, sévérité, voie, preuves).
//! - [`Pouls`] : preuve de vie d'un capteur — émis **uniquement** si la source
//!   a été relevée avec succès ; une ligne mal formée n'est jamais un pouls.
//! - [`LotVigie`] : ce que rend une analyse (signaux, pouls éventuel, compteurs).

use std::fmt;

use crate::zone::Zone;

/// Longueur maximale d'un identifiant (capteur, voie, règle, événement).
pub const LONGUEUR_MAX_IDENTIFIANT: usize = 128;

fn identifiant_valide(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= LONGUEUR_MAX_IDENTIFIANT
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}

macro_rules! identifiant {
    ($(#[$doc:meta])* $nom:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $nom(String);

        impl $nom {
            /// Construit l'identifiant ; refusé s'il est vide, trop long
            /// (> [`LONGUEUR_MAX_IDENTIFIANT`]) ou hors `[A-Za-z0-9._-]`.
            pub fn new(s: impl Into<String>) -> Option<Self> {
                let s = s.into();
                identifiant_valide(&s).then_some(Self(s))
            }

            /// Identifiant constant interne à la crate (règles intégrées). Le format
            /// est vérifié par les tests unitaires de chaque constante ; aucun chemin
            /// d'échec à l'exécution.
            #[allow(dead_code)]
            pub(crate) fn constante(s: &'static str) -> Self {
                debug_assert!(identifiant_valide(s), "identifiant constant invalide");
                Self(s.to_owned())
            }

            /// Valeur textuelle.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $nom {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

identifiant!(
    /// Identifiant d'un capteur (une Vigie implantée dans une zone).
    CapteurId
);
identifiant!(
    /// Identifiant d'une voie du Portail (une voie par service, EX-44).
    VoieId
);
identifiant!(
    /// Identifiant d'une règle de détection.
    RegleId
);
identifiant!(
    /// Identifiant d'un événement source (unique par capteur) ; sert à la
    /// déduplication et de preuve citée par les décisions.
    EvenementId
);

/// Sévérité d'un signal (ordonnée).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Severite {
    /// Information (aucune réaction).
    Info,
    /// Faible.
    Faible,
    /// Moyenne : voie surveillée.
    Moyenne,
    /// Élevée : voie surveillée, journal renforcé.
    Elevee,
    /// Critique : incident (EX-12) → coupure de la voie concernée (EX-49).
    Critique,
}

impl Severite {
    /// Nom utilisé dans le format des règles.
    pub fn nom(self) -> &'static str {
        match self {
            Severite::Info => "info",
            Severite::Faible => "faible",
            Severite::Moyenne => "moyenne",
            Severite::Elevee => "elevee",
            Severite::Critique => "critique",
        }
    }

    /// Analyse stricte du nom.
    pub fn depuis_nom(s: &str) -> Option<Self> {
        match s {
            "info" => Some(Severite::Info),
            "faible" => Some(Severite::Faible),
            "moyenne" => Some(Severite::Moyenne),
            "elevee" => Some(Severite::Elevee),
            "critique" => Some(Severite::Critique),
            _ => None,
        }
    }
}

/// Catégorie de détection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Categorie {
    /// Tentative de sortie de la zone COG (EX-12) — toujours critique.
    SortieZoneCog,
    /// Mouvement latéral (ex. voie A vers l'extrémité de la voie B).
    MouvementLateral,
    /// Flux hors contrat (§10) signalé par une règle.
    FluxHorsContrat,
    /// Donnée de capteur incohérente ou indéterminable (zone, verdict inconnus).
    DonneeIndeterminee,
    /// Capteur dégradé : lignes rejetées, source en échec.
    CapteurDegrade,
}

impl Categorie {
    /// Nom utilisé dans le format des règles.
    pub fn nom(self) -> &'static str {
        match self {
            Categorie::SortieZoneCog => "sortie_zone_cog",
            Categorie::MouvementLateral => "mouvement_lateral",
            Categorie::FluxHorsContrat => "flux_hors_contrat",
            Categorie::DonneeIndeterminee => "donnee_indeterminee",
            Categorie::CapteurDegrade => "capteur_degrade",
        }
    }

    /// Analyse stricte du nom.
    pub fn depuis_nom(s: &str) -> Option<Self> {
        match s {
            "sortie_zone_cog" => Some(Categorie::SortieZoneCog),
            "mouvement_lateral" => Some(Categorie::MouvementLateral),
            "flux_hors_contrat" => Some(Categorie::FluxHorsContrat),
            "donnee_indeterminee" => Some(Categorie::DonneeIndeterminee),
            "capteur_degrade" => Some(Categorie::CapteurDegrade),
            _ => None,
        }
    }
}

/// Signal émis par une Vigie.
///
/// `id` est déterministe (`capteur:evenement:regle`) : le même événement rejoué
/// produit le même identifiant, ce qui permet la déduplication en aval.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signal {
    /// Identifiant déterministe du signal.
    pub id: String,
    /// Capteur émetteur.
    pub capteur: CapteurId,
    /// Horodatage de l'événement source (ms depuis l'époque Unix, fourni par la source).
    pub horodatage_ms: u64,
    /// Voie concernée, si l'événement est attribuable à une voie.
    pub voie: Option<VoieId>,
    /// Zone source de l'événement.
    pub zone_src: Zone,
    /// Zone destination de l'événement.
    pub zone_dst: Zone,
    /// Sévérité.
    pub severite: Severite,
    /// Catégorie.
    pub categorie: Categorie,
    /// Règle déclenchée.
    pub regle: RegleId,
    /// Preuves : identifiants des événements sources.
    pub preuves: Vec<EvenementId>,
    /// Description lisible (explicabilité).
    pub detail: String,
}

/// Preuve de vie d'un capteur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pouls {
    /// Capteur.
    pub capteur: CapteurId,
    /// Instant de la relève réussie (ms, fourni par l'appelant).
    pub horodatage_ms: u64,
}

/// État de fonctionnement d'un capteur lors d'une analyse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtatCapteur {
    /// Toutes les entrées ont été comprises.
    Nominal,
    /// Des entrées ont été rejetées (mal formées, incohérentes, rejouées).
    Degrade,
    /// La source n'a pas pu être relevée.
    EnEchec,
}

/// Compteurs d'une analyse (jamais remis à zéro silencieusement : cumulés par la Vigie).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Compteurs {
    /// Entrées acceptées et analysées.
    pub acceptees: u64,
    /// Lignes de journal mal formées (ignorées, jamais bénignes).
    pub mal_formees: u64,
    /// Lignes hors convention de préfixe (n'appartiennent pas aux Vigies).
    pub hors_convention: u64,
    /// Entrées incohérentes (ex. zone du préfixe ≠ zone de l'adresse).
    pub incoherentes: u64,
    /// Doublons écartés (même identifiant d'événement).
    pub doublons: u64,
    /// Rejeus écartés (horodatage trop ancien par rapport au plus récent vu).
    pub rejeus: u64,
}

impl Compteurs {
    /// Ajoute `autre` à `self`.
    pub fn cumuler(&mut self, autre: &Compteurs) {
        self.acceptees += autre.acceptees;
        self.mal_formees += autre.mal_formees;
        self.hors_convention += autre.hors_convention;
        self.incoherentes += autre.incoherentes;
        self.doublons += autre.doublons;
        self.rejeus += autre.rejeus;
    }

    /// Nombre d'entrées rejetées pour défaut (hors doublons légitimes).
    pub fn rejets(&self) -> u64 {
        self.mal_formees + self.incoherentes + self.rejeus
    }
}

/// Résultat d'une analyse par une Vigie, transmis à CaringNanny.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LotVigie {
    /// Capteur.
    pub capteur: CapteurId,
    /// Voies surveillées par ce capteur (pour la santé par voie).
    pub voies_couvertes: Vec<VoieId>,
    /// Signaux produits.
    pub signaux: Vec<Signal>,
    /// Pouls : présent seulement si la source a été relevée avec succès.
    pub pouls: Option<Pouls>,
    /// État du capteur pour ce lot.
    pub etat: EtatCapteur,
    /// Compteurs de ce lot.
    pub compteurs: Compteurs,
}
