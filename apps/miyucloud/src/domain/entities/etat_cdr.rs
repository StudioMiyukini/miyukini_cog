//! @id     miyucloud_domain_entities_etat_cdr
//! @do     definir_etat_cdr
//! @role   securite
//! @layer  app
//! @human  miyucloud — état de désarmement (CDR) d'un fichier stocké et règles d'usage associées.

//! État CDR d'un fichier (décision A-004, EX-45).
//!
//! Tout fichier entrant dans MiyuCloud passe par le CDR (`miyucdr`) :
//! - **désarmé** : les octets stockés sont ceux reconstruits par le CDR ;
//! - **quarantaine** : type non pris en charge par le CDR (PDF, vidéo,
//!   archive, bureautique…) ou trop volumineux pour lui — octets d'origine
//!   stockés à part, marqués « non désarmé », jamais ouverts par COG ;
//! - **inconnu** : fichier antérieur au CDR (valeur par défaut de la
//!   migration) ou écrit par un chemin qui ne déclare pas d'état — traité
//!   comme non désarmé pour tout traitement de contenu (fermé par défaut).
//!
//! Les règles d'usage sont centralisées ici pour que chaque point de lecture
//! (vignettes, aperçus, WOPI, liens publics, téléchargements, WebDAV) applique
//! la même politique.

use std::fmt;

/// État de désarmement d'un fichier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EtatCdr {
    /// Octets reconstruits par le CDR.
    Desarme,
    /// Non pris en charge ou trop volumineux : stocké tel quel, non désarmé.
    Quarantaine,
    /// Antérieur au CDR ou origine non déclarée : non désarmé.
    Inconnu,
}

impl EtatCdr {
    /// Valeur stockée dans `storage.files.etat_cdr`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Desarme => "desarme",
            Self::Quarantaine => "quarantaine",
            Self::Inconnu => "inconnu",
        }
    }

    /// Lecture depuis la base. Toute valeur inattendue est traitée comme
    /// `Quarantaine` (fermé par défaut).
    pub fn depuis_stockage(valeur: &str) -> Self {
        match valeur {
            "desarme" => Self::Desarme,
            "inconnu" => Self::Inconnu,
            _ => Self::Quarantaine,
        }
    }

    /// Vrai si les octets stockés sont issus du CDR.
    pub fn est_desarme(self) -> bool {
        self == Self::Desarme
    }

    /// Un composant COG peut-il **ouvrir** le contenu (vignette, aperçu,
    /// transcodage, édition WOPI, indexation, extraction de métadonnées) ?
    ///
    /// Seulement pour un fichier désarmé.
    pub fn autorise_traitement_contenu(self) -> bool {
        self.est_desarme()
    }

    /// Un lien de partage public peut-il viser ce fichier ?
    ///
    /// Seulement pour un fichier désarmé : un tiers ne doit jamais recevoir
    /// un contenu non désarmé sans l'action explicite de son propriétaire.
    pub fn autorise_lien_public(self) -> bool {
        self.est_desarme()
    }

    /// Le téléchargement par le propriétaire exige-t-il une action explicite
    /// (`?accepter_non_desarme=1`) ?
    ///
    /// Oui pour la quarantaine. Les fichiers `Inconnu` (antérieurs au CDR)
    /// restent téléchargeables par leur propriétaire sans drapeau, pour ne pas
    /// couper les clients de synchronisation sur les données existantes ; ils
    /// ne sont en revanche jamais ouverts par COG (voir
    /// [`Self::autorise_traitement_contenu`]).
    pub fn exige_accord_telechargement(self) -> bool {
        self == Self::Quarantaine
    }

    /// Téléchargement autorisé compte tenu de l'accord explicite éventuel.
    pub fn autorise_telechargement(self, accord_explicite: bool) -> bool {
        !self.exige_accord_telechargement() || accord_explicite
    }

    /// Lecture WebDAV (`GET`) : un client DAV ne peut pas exprimer l'accord
    /// explicite, la quarantaine y est donc refusée (403).
    pub fn autorise_lecture_dav(self) -> bool {
        self.autorise_telechargement(false)
    }
}

impl fmt::Display for EtatCdr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// État CDR et compte rendu à écrire avec la ligne `storage.files`.
#[derive(Debug, Clone)]
pub struct MarqueCdr {
    /// État à stocker.
    pub etat: EtatCdr,
    /// Compte rendu (JSONB) ; `None` = NULL.
    pub rapport: Option<serde_json::Value>,
}

impl MarqueCdr {
    /// Marque des écritures qui ne sont pas passées par l'entonnoir :
    /// `inconnu`, donc traitées comme non désarmées (fermé par défaut).
    pub fn inconnue() -> Self {
        Self {
            etat: EtatCdr::Inconnu,
            rapport: None,
        }
    }
}

/// Nom du paramètre de requête exprimant l'accord explicite de l'utilisateur
/// pour télécharger un fichier non désarmé.
pub const PARAM_ACCORD_NON_DESARME: &str = "accepter_non_desarme";

/// En-tête de réponse exposant l'état CDR du fichier servi.
pub const ENTETE_ETAT_CDR: &str = "x-miyucloud-etat-cdr";

/// Message renvoyé quand un fichier en quarantaine est demandé sans accord.
pub const MESSAGE_QUARANTAINE: &str = "Fichier non désarmé (quarantaine CDR) : type non pris en \
     charge par le désarmement ou trop volumineux. Téléchargement possible uniquement après \
     accord explicite (paramètre accepter_non_desarme=1) ; jamais ouvert ni prévisualisé par COG.";

/// Interprète la valeur du paramètre d'accord (`1` ou `true`).
pub fn accord_explicite(valeur: Option<&str>) -> bool {
    matches!(valeur, Some("1") | Some("true"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aller_retour_stockage() {
        for etat in [EtatCdr::Desarme, EtatCdr::Quarantaine, EtatCdr::Inconnu] {
            assert_eq!(EtatCdr::depuis_stockage(etat.as_str()), etat);
        }
        // Valeur inattendue : fermé par défaut.
        assert_eq!(EtatCdr::depuis_stockage("autre"), EtatCdr::Quarantaine);
    }

    #[test]
    fn seul_desarme_autorise_le_traitement_et_les_liens() {
        assert!(EtatCdr::Desarme.autorise_traitement_contenu());
        assert!(EtatCdr::Desarme.autorise_lien_public());
        for etat in [EtatCdr::Quarantaine, EtatCdr::Inconnu] {
            assert!(!etat.autorise_traitement_contenu());
            assert!(!etat.autorise_lien_public());
        }
    }

    #[test]
    fn quarantaine_exige_accord_explicite() {
        assert!(!EtatCdr::Quarantaine.autorise_telechargement(false));
        assert!(EtatCdr::Quarantaine.autorise_telechargement(true));
        assert!(!EtatCdr::Quarantaine.autorise_lecture_dav());
        assert!(EtatCdr::Desarme.autorise_telechargement(false));
        assert!(EtatCdr::Inconnu.autorise_telechargement(false));
        assert!(EtatCdr::Inconnu.autorise_lecture_dav());
    }

    #[test]
    fn interpretation_accord() {
        assert!(accord_explicite(Some("1")));
        assert!(accord_explicite(Some("true")));
        assert!(!accord_explicite(Some("0")));
        assert!(!accord_explicite(None));
    }
}
