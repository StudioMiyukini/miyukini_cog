use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum DavError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Locked: {0}")]
    Locked(String),

    #[error("Precondition failed: {0}")]
    PreconditionFailed(String),

    #[error("Unsupported media type")]
    UnsupportedMediaType,

    #[error("Method not allowed")]
    MethodNotAllowed,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("XML parse error: {0}")]
    XmlParse(String),

    #[error("Path validation error: {0}")]
    PathValidation(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for DavError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Locked(_) => StatusCode::LOCKED,
            Self::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            Self::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Self::BadRequest(_) | Self::XmlParse(_) | Self::PathValidation(_) => {
                StatusCode::BAD_REQUEST
            }
            Self::Storage(_) | Self::Database(_) | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<d:error xmlns:d=\"DAV:\">\n  <d:message>{self}</d:message>\n</d:error>");

        (status, [("content-type", "application/xml; charset=utf-8")], body).into_response()
    }
}

impl From<rusqlite::Error> for DavError {
    fn from(e: rusqlite::Error) -> Self {
        tracing::error!(error = %e, "Database error");
        Self::Database(e.to_string())
    }
}

impl From<quick_xml::Error> for DavError {
    fn from(e: quick_xml::Error) -> Self {
        Self::XmlParse(e.to_string())
    }
}
