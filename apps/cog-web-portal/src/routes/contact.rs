//! Route de contact — GET et POST /:service/contact.
//!
//! @id: cog_web_portal_route_contact @do: handle_contact_form_with_csrf
//! @role: api @layer: app
//! @human: Contact Portal — formulaire GET (avec token CSRF) et POST (validation CSRF + champs).

use axum::Extension;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use serde::Deserialize;
use tracing::warn;

use crate::AppState;
use crate::csrf::{generate_csrf_token, validate_csrf_token};
use crate::security_headers::CspNonce;
use crate::templates::render_contact_form;

/// Handler GET /:service/contact — affiche le formulaire avec token CSRF.
pub async fn contact_get(
    State(state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
    Path(service_slug): Path<String>,
) -> Response {
    let svc = state.services.iter().find(|s| s.service_slug() == service_slug);
    let Some(svc) = svc else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let token = generate_csrf_token();
    Html(render_contact_form(
        svc.service_name(),
        svc.service_slug(),
        &token,
        &nonce,
        None,
        false,
    ))
    .into_response()
}

/// Données du formulaire de contact.
#[derive(Deserialize)]
pub struct ContactForm {
    pub csrf_token: String,
    pub name: String,
    pub email: String,
    pub message: String,
}

/// Handler POST /:service/contact — valide CSRF et traite le formulaire.
pub async fn contact_post(
    State(state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
    Path(service_slug): Path<String>,
    Form(form): Form<ContactForm>,
) -> Response {
    let svc = state.services.iter().find(|s| s.service_slug() == service_slug);
    let Some(svc) = svc else {
        return StatusCode::NOT_FOUND.into_response();
    };

    // Validation CSRF.
    if !validate_csrf_token(&form.csrf_token) {
        warn!("CSRF validation failed for service {service_slug}");
        return Html(render_contact_form(
            svc.service_name(),
            svc.service_slug(),
            &generate_csrf_token(),
            &nonce,
            Some("Token de sécurité invalide. Veuillez réessayer."),
            false,
        ))
        .into_response();
    }

    // Validation des champs.
    let name = form.name.trim();
    let email = form.email.trim();
    let message = form.message.trim();

    if name.is_empty() || email.is_empty() || message.is_empty() {
        return Html(render_contact_form(
            svc.service_name(),
            svc.service_slug(),
            &generate_csrf_token(),
            &nonce,
            Some("Tous les champs sont requis."),
            false,
        ))
        .into_response();
    }

    if name.len() > 200 || email.len() > 254 || message.len() > 2000 {
        return Html(render_contact_form(
            svc.service_name(),
            svc.service_slug(),
            &generate_csrf_token(),
            &nonce,
            Some("Un ou plusieurs champs dépassent la taille maximale autorisée."),
            false,
        ))
        .into_response();
    }

    // Validation email basique.
    if !email.contains('@') || !email.contains('.') {
        return Html(render_contact_form(
            svc.service_name(),
            svc.service_slug(),
            &generate_csrf_token(),
            &nonce,
            Some("Adresse email invalide."),
            false,
        ))
        .into_response();
    }

    // TODO: envoyer le message (email, webhook, DB). Pour l'instant, log uniquement.
    tracing::info!(
        service = service_slug,
        name = name,
        email = email,
        "Contact form submitted"
    );

    // Redirect POST/GET pour éviter la resoumission du formulaire.
    Redirect::to(&format!("/{service_slug}/contact?success=1")).into_response()
}
