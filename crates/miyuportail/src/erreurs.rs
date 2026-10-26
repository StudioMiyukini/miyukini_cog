//! @id     miyuportail_erreurs
//! @do     typer_refus_portail
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — erreurs typées : une variante par cause de refus (C-11, C-12).

use thiserror::Error;

/// Cause de refus d'un décodage MSCE (trame, charge, traité).
///
/// Chaque variante a un code stable sur un octet, transmis par le sas de
/// décodage à la zone COG (aucun texte libre ne traverse).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[repr(u8)]
pub enum RefusDecodage {
    /// Taille hors bornes, troncature ou octets en trop.
    #[error("taille invalide")]
    Taille = 0x01,
    /// Magic « MSCE » absent.
    #[error("magic MSCE absent")]
    Magic = 0x02,
    /// Version différente de Sceau v1.1.
    #[error("version non supportée")]
    Version = 0x03,
    /// Algorithme inconnu ou non implémenté.
    #[error("algorithme refusé")]
    Algorithme = 0x04,
    /// Type de trame différent de « déclaration de voie ».
    #[error("type de trame refusé")]
    TypeTrame = 0x05,
    /// Nombre de signatures, ordre des `kid` ou signataires non conformes.
    #[error("bloc de signatures non conforme")]
    Signataires = 0x06,
    /// Signature invalide (`verify_strict`).
    #[error("signature invalide")]
    Signature = 0x07,
    /// TLV non canonique (ordre, doublon, tag inconnu, champ manquant).
    #[error("TLV non canonique")]
    Tlv = 0x08,
    /// Valeur mal formée (UTF-8, caractère de contrôle, booléen, entier).
    #[error("valeur mal formée")]
    Valeur = 0x09,
    /// Service différent de celui de la voie.
    #[error("service inattendu")]
    Service = 0x0A,
    /// Sens différent de celui de la voie.
    #[error("sens inattendu")]
    Sens = 0x0B,
    /// Version de traité incompatible.
    #[error("version de traité incompatible")]
    VersionTraite = 0x0C,
    /// Type de déclaration absent du traité (INV-H3).
    #[error("type de déclaration non autorisé")]
    TypeDeclaration = 0x0D,
    /// Champ absent du traité, obligatoire manquant ou hors borne.
    #[error("champ non conforme au traité")]
    Champ = 0x0E,
    /// Requête du sas mal formée (protocole parent → sas).
    #[error("requête de sas mal formée")]
    Requete = 0x0F,
}

impl RefusDecodage {
    /// Code sur un octet (protocole du sas).
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// Reconstruit depuis un code ; code inconnu → `None` (refus en amont).
    pub const fn depuis_code(code: u8) -> Option<Self> {
        Some(match code {
            0x01 => Self::Taille,
            0x02 => Self::Magic,
            0x03 => Self::Version,
            0x04 => Self::Algorithme,
            0x05 => Self::TypeTrame,
            0x06 => Self::Signataires,
            0x07 => Self::Signature,
            0x08 => Self::Tlv,
            0x09 => Self::Valeur,
            0x0A => Self::Service,
            0x0B => Self::Sens,
            0x0C => Self::VersionTraite,
            0x0D => Self::TypeDeclaration,
            0x0E => Self::Champ,
            0x0F => Self::Requete,
            _ => return None,
        })
    }
}

/// Défaillance du sas de décodage (processus isolé).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ErreurSas {
    /// Plateforme sans seccomp : le sas refuse de fonctionner (fail-closed).
    #[error("sas de décodage indisponible sur cette plateforme")]
    PlateformeNonSupportee,
    /// Lancement du processus impossible.
    #[error("lancement du sas impossible")]
    Lancement,
    /// Délai dépassé : le processus a été tué.
    #[error("délai du sas dépassé, processus tué")]
    Delai,
    /// Le processus s'est terminé anormalement (plantage, signal, code ≠ 0).
    #[error("fin anormale du sas")]
    FinAnormale,
    /// Réponse mal formée, trop grande ou incohérente avec la requête.
    #[error("réponse du sas mal formée")]
    Reponse,
    /// Installation du bac à sable (rlimits, seccomp) impossible.
    #[error("installation du bac à sable impossible")]
    BacASable,
    /// Noyau sans Landlock (ou domaine non appliqué) : le sas refuse de
    /// démarrer, sans réglage pour lever ce refus (fail-closed).
    #[error("Landlock indisponible : sas refusé")]
    LandlockIndisponible,
    /// Descripteur au-delà de 0, 1, 2 transmissible au sas (côté parent :
    /// sans `FD_CLOEXEC`) ou hérité par le sas (code de sortie
    /// [`crate::sas::SORTIE_DESCRIPTEURS`]) : lancement refusé (fail-closed).
    #[error("descripteur hérité par le sas : refus")]
    DescripteurHerite,
    /// Chemin du binaire du sas non absolu (aucune résolution par `PATH`).
    #[error("chemin du sas non absolu : refus")]
    CheminRelatif,
}

/// Erreur du Portail interne. Toute erreur est un refus (C-12).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ErreurPortail {
    /// Identifiant de service invalide.
    #[error("identifiant de service invalide")]
    IdentifiantInvalide,
    /// Contrat de voies invalide (traité, bornes, classe).
    #[error("contrat de voies invalide : {0}")]
    ContratInvalide(&'static str),
    /// Classe H0 : jamais de façade, jamais de voie (EX-40).
    #[error("service H0 : aucune voie possible")]
    ClasseSansFacade,
    /// Voie déjà déclarée pour ce service.
    #[error("voie déjà déclarée")]
    VoieDejaDeclaree,
    /// Voie non déclarée : ce qui n'est pas déclaré n'existe pas.
    #[error("voie non déclarée")]
    VoieNonDeclaree,
    /// Sens non ouvert pour la classe du service (ex. entrée sur un H1).
    #[error("sens refusé pour cette voie")]
    SensRefuse,
    /// Voie désactivée (exposition désactivée par défaut, EX-48).
    #[error("voie désactivée")]
    VoieDesactivee,
    /// Voie coupée (WorrySentinel, EX-49).
    #[error("voie coupée")]
    VoieCoupee,
    /// Aucune session ouverte : pas de clé de façade épinglée.
    #[error("aucune session ouverte sur la voie")]
    SessionAbsente,
    /// Clé publique invalide ou faible.
    #[error("clé publique invalide")]
    CleInvalide,
    /// Trame trop grande pour le contrat de la voie.
    #[error("trame trop grande")]
    TropGrand,
    /// Trame sans magic « MSCE » (liste blanche de protocole).
    #[error("protocole hors liste blanche MSCE")]
    ProtocoleRefuse,
    /// File pleine (quota de la voie) : contre-pression.
    #[error("file de la voie pleine")]
    FilePleine,
    /// Débit du contrat dépassé.
    #[error("débit du contrat dépassé")]
    DebitDepasse,
    /// Décodage refusé (par le sas ou par la deuxième validation).
    #[error("décodage refusé : {0}")]
    Decodage(RefusDecodage),
    /// Défaillance du sas : le message est refusé.
    #[error("sas : {0}")]
    Sas(ErreurSas),
    /// Deuxième validation en désaccord avec le sas.
    #[error("deuxième validation divergente")]
    ValidationDivergente,
    /// Rejeu : numéro de séquence non strictement croissant pour cet auteur (A-007).
    #[error("séquence rejouée ou désordonnée")]
    Rejeu,
    /// Auteur non épinglé dans la session (jamais épinglé ou retiré).
    #[error("auteur non épinglé dans la session")]
    AuteurNonEpingle,
    /// Auteur déjà épinglé dans la session.
    #[error("auteur déjà épinglé dans la session")]
    AuteurDejaEpingle,
    /// Trop d'auteurs épinglés ou connus dans la session.
    #[error("trop d'auteurs dans la session")]
    TropDAuteurs,
    /// Désarmement (CDR) refusé.
    #[error("désarmement refusé")]
    Desarmement,
    /// Étiquette `nationale` : ne sort jamais (EX-47).
    #[error("donnée nationale : sortie interdite")]
    SortieNationale,
    /// Publication sans action explicite de l'utilisateur, ou par MAIA.
    #[error("publication sans action explicite de l'utilisateur")]
    ActionUtilisateurRequise,
    /// Inscription au registre impossible.
    #[error("inscription au registre impossible")]
    Inscription,
    /// Verrou interne empoisonné : refus.
    #[error("état interne indisponible")]
    EtatIndisponible,
    /// Transition d'état refusée.
    #[error("transition d'état refusée")]
    TransitionRefusee,
    /// Jeton de réponse invalide : signature, session ou service (A-022 §3).
    #[error("jeton de réponse invalide")]
    JetonInvalide,
    /// Jeton de réponse déjà consommé (usage unique).
    #[error("jeton de réponse déjà utilisé")]
    JetonDejaUtilise,
    /// Publication sans jeton sur une voie H2/H3 : seule une réponse sort.
    #[error("publication hors réponse refusée : jeton requis")]
    JetonRequis,
    /// Trop de jetons consommés dans la session : la rouvrir.
    #[error("jetons de la session épuisés")]
    JetonsEpuises,
    /// Réponse vide ou trop longue.
    #[error("réponse hors bornes (1 à 32 messages)")]
    ReponseHorsBornes,
}

impl From<RefusDecodage> for ErreurPortail {
    fn from(r: RefusDecodage) -> Self {
        Self::Decodage(r)
    }
}

impl From<ErreurSas> for ErreurPortail {
    fn from(e: ErreurSas) -> Self {
        Self::Sas(e)
    }
}
