//! @id     miyufederation_statut
//! @do     definir_statuts_federaux
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « statut » (candidat, invité, membre, suspendu, révoqué ; seuils de la doc gelée).

//! Statuts fédéraux (zones §16) et seuils de la doc gelée.

/// Confirmations distinctes de membres pour le statut invité (zones §16).
pub const CONFIRMATIONS_INVITE: usize = 2;
/// Confirmations distinctes de membres pour le statut membre (zones §16).
pub const CONFIRMATIONS_MEMBRE: usize = 4;
/// Durée continue à ≥ 4 confirmations pour devenir membre : 7 jours (zones §16).
pub const DUREE_MEMBRE_SECS: u64 = 7 * 86_400;
/// Quorum de membres indépendants d'une accusation (EX-32, zones §16).
pub const QUORUM_BLACKLIST: usize = 6;
/// Quorum de membres distincts (hors accusé) d'un « recours accepté » (EX-32, A-007).
pub const QUORUM_RECOURS: usize = 6;
/// Délai de contestation : suspension immédiate, révocation définitive après
/// 72 h sans recours accepté (EX-32, décision du propriétaire A-007).
pub const DELAI_CONTESTATION_SECS: u64 = 72 * 3_600;
/// Durée de validité maximale d'une confirmation : renouvelée toutes les 24 h (zones §16).
pub const DUREE_CONFIRMATION_MAX_SECS: u64 = 86_400;

/// Statut d'un COG dans la vue fédérale locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Statut {
    /// 0–1 confirmation : aucun accès fédéré.
    Candidat,
    /// ≥ 2 confirmations : sessions sur invitation d'un membre, ne confirme pas.
    Invite,
    /// ≥ 4 confirmations continues pendant 7 jours (ou fondateur) : accès complet, confirme et signale.
    Membre,
    /// Accusation au quorum : **suspendu** (titres refusés partout) en
    /// attente de la révocation définitive (72 h) ou d'un recours accepté
    /// (EX-32, A-007).
    Suspendu {
        /// Heure locale de la suspension (réception de l'accusation au quorum).
        depuis: u64,
        /// Identité de l'accusation (SHA-256 de ses octets signés).
        accusation: [u8; 32],
    },
    /// Identité révoquée définitivement (blacklist) : tous les pairs refusent.
    Revoque,
}

impl Statut {
    /// Vrai si le statut ouvre un accès fédéré (invité ou membre).
    pub const fn acces_federe(self) -> bool {
        matches!(self, Self::Invite | Self::Membre)
    }
}

/// Seule sanction fédérale existante (EX-33, INV-Z11) : la révocation de
/// l'identité. **Aucune destruction à distance** : les données restent au
/// propriétaire (souvent victime d'un piratage) ; la réinitialisation est une
/// décision locale proposée par WorrySentinel au propriétaire (TAMR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sanction {
    /// Révocation définitive du `cog_id` (entrée `Revocation { cible: Cle(kid) }`).
    RevocationIdentite,
}

impl Sanction {
    /// Toutes les sanctions possibles (une seule).
    pub const TOUTES: [Self; 1] = [Self::RevocationIdentite];
}
