//! @id     miyuregistre_erreurs
//! @do     definir_erreurs_registre
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « erreurs » (refus du stockage, du journal, du temps de confiance).

//! Erreurs du registre. Toutes signifient « refus » : aucune ne se récupère en
//! acceptant la donnée fautive.

use miyusceau::{ErreurRegistre, ErreurSceau};

/// Refus ou panne du stockage append-only.
#[derive(Debug, thiserror::Error)]
pub enum ErreurStockage {
    /// Entrée/sortie du système de fichiers.
    #[error("entrée/sortie du stockage : {0}")]
    Io(#[from] std::io::Error),
    /// Entête de fichier absente ou d'une autre version.
    #[error("entête de stockage invalide")]
    EnteteInvalide,
    /// Enregistrement altéré au milieu du journal (pas une fin tronquée) :
    /// aucune réparation automatique, refus.
    #[error("enregistrement {index} altéré au milieu du journal")]
    Corrompu {
        /// Index de l'enregistrement fautif.
        index: u64,
    },
    /// Enregistrement vide ou au-delà de la taille maximale.
    #[error("enregistrement de taille invalide")]
    TailleInvalide,
    /// Index hors du journal.
    #[error("index {0} hors du journal")]
    HorsBornes(u64),
    /// Une écriture précédente a échoué sans pouvoir être annulée : le
    /// stockage doit être rouvert (récupération de la fin tronquée).
    #[error("stockage empoisonné par une écriture interrompue : rouvrir")]
    Empoisonne,
}

/// Refus du temps de confiance (EX-22).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErreurTemps {
    /// Horodatage antérieur au Median-Time-Past des dernières têtes vues.
    #[error("horodatage {horodatage} antérieur au Median-Time-Past {mtp}")]
    AnterieurMedian {
        /// Horodatage refusé.
        horodatage: u64,
        /// Médiane en vigueur.
        mtp: u64,
    },
    /// État persistant du temps de confiance mal formé.
    #[error("état du temps de confiance invalide")]
    EtatInvalide,
}

/// Refus d'une opération sur un registre (journal émetteur, suivi, miroir, fédération).
#[derive(Debug, thiserror::Error)]
pub enum ErreurJournal {
    /// Refus du stockage.
    #[error(transparent)]
    Stockage(#[from] ErreurStockage),
    /// Refus d'un sceau (format, signature, seuil, validité).
    #[error("sceau refusé : {0}")]
    Sceau(#[from] ErreurSceau),
    /// Refus d'une tête (fraîcheur, monotonie, équivoque, preuve).
    #[error("tête refusée : {0}")]
    Registre(#[from] ErreurRegistre),
    /// Refus du temps de confiance (Median-Time-Past).
    #[error(transparent)]
    Temps(#[from] ErreurTemps),
    /// Horodatage de tête en recul côté émetteur.
    #[error("horodatage d'émission en recul")]
    HorodatageEnRecul,
    /// Taille de tête demandée hors du journal.
    #[error("taille hors du journal")]
    TailleHorsJournal,
    /// Le nombre d'entrées fournies ne correspond pas à la tête.
    #[error("entrées incohérentes avec la tête : {attendues} attendues, {recues} reçues")]
    EntreesIncoherentes {
        /// Entrées attendues (`taille de la tête − taille du miroir`).
        attendues: u64,
        /// Entrées reçues.
        recues: u64,
    },
    /// Les entrées reçues ne reconstruisent pas la racine signée.
    #[error("les entrées ne reconstruisent pas la racine signée")]
    RacineDifferente,
    /// Le stockage ne contient pas l'histoire de l'état persistant.
    #[error("stockage incohérent avec l'état persistant")]
    StockageIncoherent,
    /// Registre non inscrit (fédération).
    #[error("registre inconnu")]
    RegistreInconnu,
    /// Aucune tête acceptée pour ce registre.
    #[error("aucune tête acceptée")]
    AucuneTete,
    /// Témoins (contresignatures de membres distincts, Origin = un au plus)
    /// insuffisants pour une tête fédérale (A-007, EX-34).
    #[error("témoins insuffisants : {obtenu} sur {requis}")]
    TemoinsInsuffisants {
        /// Témoins exigés.
        requis: usize,
        /// Témoins valides comptés.
        obtenu: usize,
    },
}
