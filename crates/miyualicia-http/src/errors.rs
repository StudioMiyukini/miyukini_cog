//! Types d'erreur du client HTTP Alicia.

// @id: toolkit.alicia.http.errors
// @role: error_types
// @layer: 6
// @human: Erreurs HTTP Alicia : timeout, connexion, parsing, authentification.
// @do: define_http_error_types

/// Erreurs du client HTTP Alicia.
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    /// Le dispositif n'a pas repondu dans le delai imparti.
    #[error("timeout sur {url} apres {timeout_ms}ms")]
    Timeout {
        /// URL cible.
        url: String,
        /// Delai en millisecondes.
        timeout_ms: u64,
    },

    /// Erreur de connexion au dispositif (reseau inaccessible, hote inconnu).
    #[error("connexion echouee vers {url} : {message}")]
    ConnectionFailed {
        /// URL cible.
        url: String,
        /// Description de l'erreur.
        message: String,
    },

    /// Le dispositif a repondu avec un code HTTP d'erreur.
    #[error("reponse HTTP {status} depuis {url} : {body}")]
    HttpStatus {
        /// URL cible.
        url: String,
        /// Code de statut HTTP.
        status: u16,
        /// Corps de la reponse (tronque a 512 octets).
        body: String,
    },

    /// Erreur de parsing de la reponse JSON du dispositif.
    #[error("parsing JSON echoue depuis {url} : {message}")]
    ParseError {
        /// URL source.
        url: String,
        /// Description du probleme.
        message: String,
    },

    /// Erreur d'authentification (credentials invalides ou manquants).
    #[error("authentification echouee vers {url} : {message}")]
    AuthenticationFailed {
        /// URL cible.
        url: String,
        /// Description.
        message: String,
    },

    /// Erreur de serialisation d'un payload.
    #[error("erreur serialisation : {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Erreur reqwest sous-jacente.
    #[error("erreur HTTP : {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// L'adaptateur n'est pas supporte pour ce type de dispositif.
    #[error("adaptateur inconnu pour le dispositif a l'adresse {address}")]
    UnsupportedAdapter {
        /// Adresse du dispositif.
        address: String,
    },
}
