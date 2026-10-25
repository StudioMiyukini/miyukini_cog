//! @id     jayrdv_facade_api
//! @do     decrire_api_reservation_federee
//! @role   contrat
//! @layer  service
//! @human  jayrdv-facade — API externe de la façade : requête / réponse HTTP abstraites, corps JSON stricts (champs inconnus refusés).

//! # API externe (HTTP, sans framework)
//!
//! | Méthode | Chemin | Authentification | Corps | Réponse |
//! |---|---|---|---|---|
//! | `POST` | `/v1/defi` | aucune | vide | 200 [`DefiJson`] |
//! | `POST` | `/v1/creneaux` | visa + PoP | [`DemandeCreneauxJson`] | 202 [`AccepteJson`] |
//! | `POST` | `/v1/reservations` | visa + PoP | [`ReservationJson`] | 202 [`AccepteJson`] |
//! | `GET` | `/v1/reponses/<demande>` | visa + PoP | vide | 200 [`ReponseJson`] ou 202 en attente |
//! | `GET` | `/v1/creneaux` | visa + PoP | vide | 200 cache des créneaux publiés (non autoritaire) |
//! | `DELETE` | `/v1/session` | visa + PoP | vide | 204 : session cliente de l'appareil close (quota rendu) |
//!
//! Refus de quota (contrat de service) : 429 `quota_sessions`, 429
//! `quota_taches` ; 503 `db_secondaire_pleine` (disque).
//!
//! Les corps entrants portent les champs du message du traité, la séquence,
//! la date d'émission et la **signature de l'auteur** (clé d'appareil du
//! visa) sur la déclaration MSCE correspondante : la façade reconstruit la
//! déclaration depuis les champs typés, ne recopie aucun octet reçu, et ne
//! peut rien déposer que l'auteur n'ait signé.

use serde::{Deserialize, Serialize};

/// Taille maximale d'un corps de requête.
pub const MAX_CORPS: usize = 8192;

/// Requête HTTP terminée par la façade (vue indépendante du serveur).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequeteHttp {
    /// Méthode.
    pub methode: String,
    /// Chemin (sans chaîne de requête).
    pub chemin: String,
    /// En-têtes (noms en minuscules).
    pub entetes: Vec<(String, String)>,
    /// Corps.
    pub corps: Vec<u8>,
}

impl RequeteHttp {
    /// Valeur d'un en-tête (nom insensible à la casse ; doublon = absent, fail-closed).
    pub fn entete(&self, nom: &str) -> Option<&str> {
        let mut valeurs = self.entetes.iter().filter(|(n, _)| n.eq_ignore_ascii_case(nom));
        let premiere = valeurs.next()?;
        if valeurs.next().is_some() {
            return None;
        }
        Some(premiere.1.as_str())
    }
}

/// Réponse HTTP de la façade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReponseHttp {
    /// Statut.
    pub statut: u16,
    /// Corps JSON.
    pub corps: Vec<u8>,
}

impl ReponseHttp {
    pub(crate) fn json<T: Serialize>(statut: u16, valeur: &T) -> Self {
        Self { statut, corps: serde_json::to_vec(valeur).unwrap_or_default() }
    }

    pub(crate) fn refus(statut: u16, motif: &str) -> Self {
        Self::json(statut, &RefusJson { refus: motif.to_string() })
    }
}

/// Défi de possession émis par la façade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefiJson {
    /// Identifiant (base64 URL, 16 o).
    pub id: String,
    /// Défi (base64 URL, 32 o).
    pub defi: String,
    /// Expiration (secondes Unix).
    pub expire_a: u64,
}

/// Corps de `POST /v1/creneaux`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DemandeCreneauxJson {
    /// Séquence de l'auteur (strictement croissante dans la session).
    pub sequence: u64,
    /// Date d'émission déclarée.
    pub emise_le: u64,
    /// Début de la fenêtre (ISO 8601).
    pub debut: String,
    /// Fin de la fenêtre.
    pub fin: String,
    /// Ressource (facultative).
    pub ressource: Option<String>,
    /// Signature de l'auteur (base64 URL, 64 o).
    pub signature: String,
}

/// Corps de `POST /v1/reservations`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReservationJson {
    /// Séquence de l'auteur.
    pub sequence: u64,
    /// Date d'émission déclarée.
    pub emise_le: u64,
    /// Créneau visé.
    pub creneau: String,
    /// Nom du client.
    pub nom: String,
    /// Contact (facultatif).
    pub contact: Option<String>,
    /// Note (facultative).
    pub note: Option<String>,
    /// Signature de l'auteur (base64 URL, 64 o).
    pub signature: String,
}

/// Demande acceptée en quarantaine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccepteJson {
    /// Empreinte de la trame déposée (base64 URL) : clé de la réponse.
    pub demande: String,
}

/// Créneau (réponse ou cache).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreneauJson {
    /// Identifiant.
    pub creneau: String,
    /// Début.
    pub debut: String,
    /// Fin.
    pub fin: String,
}

/// Réponse du moteur, telle que publiée.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReponseJson {
    /// Liste des créneaux libres.
    Creneaux {
        /// Créneaux.
        creneaux: Vec<CreneauJson>,
    },
    /// Rendez-vous créé.
    Confirmee {
        /// Identifiant du rendez-vous.
        rendez_vous: String,
        /// Créneau.
        creneau: CreneauJson,
    },
    /// Réservation refusée.
    Refusee {
        /// Motif (`indisponible`, `inconnu`, `invalide`, `interne`).
        motif: String,
    },
}

/// Réponse pas encore publiée.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnAttenteJson {
    /// Toujours `en_attente`.
    pub etat: String,
}

/// Refus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefusJson {
    /// Motif court.
    pub refus: String,
}
