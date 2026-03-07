//! Templates HTML du COG Web Portal.
//!
//! @id: cog_web_portal_templates @do: render_html_portal_pages
//! @role: ui @layer: app
//! @human: Templates Portal — page d'accueil services, page de service, page d'erreur, formulaire contact.

use cog_portal_contract::PublicPage;

/// Infos d'un service pour la page d'accueil.
pub struct ServiceInfo {
    pub slug: String,
    pub name: String,
    pub page_count: usize,
}

/// Génère la page d'accueil listant les services enregistrés.
pub fn render_home(services: &[ServiceInfo], nonce: &str) -> String {
    let service_cards: String = services
        .iter()
        .map(|s| {
            format!(
                r#"<article class="service-card">
                  <h2><a href="/{slug}">{name}</a></h2>
                  <p class="page-count">{count} page(s) publique(s)</p>
                </article>"#,
                slug = escape_html(&s.slug),
                name = escape_html(&s.name),
                count = s.page_count,
            )
        })
        .collect();

    let content = if services.is_empty() {
        "<p class=\"empty\">Aucun service disponible pour le moment.</p>".to_string()
    } else {
        format!("<section class=\"services-grid\">{service_cards}</section>")
    };

    base_layout("COG Web Portal", &content, nonce, None)
}

/// Génère la page d'un service (liste des pages publiques ou affiche la page par défaut).
pub fn render_service_home(
    service_name: &str,
    service_slug: &str,
    pages: &[PublicPage],
    nonce: &str,
) -> String {
    let nav_links: String = pages
        .iter()
        .map(|p| {
            format!(
                r#"<li><a href="/{service_slug}/{slug}">{title}</a></li>"#,
                slug = escape_html(&p.slug),
                title = escape_html(&p.title),
            )
        })
        .collect();

    let first_page_html = pages
        .first()
        .map(|p| p.html_content.as_str())
        .unwrap_or("<p>Aucun contenu disponible.</p>");

    let content = format!(
        r#"<nav class="service-nav"><ul>{nav_links}</ul></nav>
           <main class="service-content">{first_page_html}</main>"#
    );

    base_layout(
        &format!("{service_name} — COG Portal"),
        &content,
        nonce,
        Some(service_slug),
    )
}

/// Génère une page de service par slug.
pub fn render_service_page(
    service_name: &str,
    service_slug: &str,
    page: &PublicPage,
    nonce: &str,
) -> String {
    let content = format!(
        r#"<article class="portal-page">
             <h1 class="page-title">{title}</h1>
             {desc}
             <div class="page-body">{html}</div>
           </article>"#,
        title = escape_html(&page.title),
        desc = page
            .description
            .as_deref()
            .map(|d| format!("<p class=\"page-desc\">{}</p>", escape_html(d)))
            .unwrap_or_default(),
        html = page.html_content,
    );

    base_layout(
        &format!("{} — {service_name}", page.title),
        &content,
        nonce,
        Some(service_slug),
    )
}

/// Génère le formulaire de contact pour un service.
pub fn render_contact_form(
    service_name: &str,
    service_slug: &str,
    csrf_token: &str,
    nonce: &str,
    error: Option<&str>,
    success: bool,
) -> String {
    let form_content = if success {
        "<p class=\"contact-success\">Votre message a été envoyé. Merci !</p>".to_string()
    } else {
        let error_html = error
            .map(|e| format!("<p class=\"contact-error\">{}</p>", escape_html(e)))
            .unwrap_or_default();
        format!(
            r#"{error_html}
            <form method="POST" action="/{service_slug}/contact" class="contact-form">
              <input type="hidden" name="csrf_token" value="{csrf_token}">
              <div class="form-field">
                <label for="name">Nom</label>
                <input type="text" id="name" name="name" required maxlength="200">
              </div>
              <div class="form-field">
                <label for="email">Email</label>
                <input type="email" id="email" name="email" required maxlength="254">
              </div>
              <div class="form-field">
                <label for="message">Message</label>
                <textarea id="message" name="message" required maxlength="2000" rows="6"></textarea>
              </div>
              <button type="submit" class="btn-primary">Envoyer</button>
            </form>"#
        )
    };

    let content = format!(
        "<section class=\"contact-section\"><h1>Contacter {service_name}</h1>{form_content}</section>",
        service_name = escape_html(service_name),
    );

    base_layout(
        &format!("Contact — {service_name}"),
        &content,
        nonce,
        Some(service_slug),
    )
}

/// Génère la page d'erreur (404 ou 500).
pub fn render_error(status: u16, message: &str, nonce: &str) -> String {
    let content = format!(
        r#"<section class="error-page">
             <h1 class="error-code">{status}</h1>
             <p class="error-message">{message}</p>
             <a href="/" class="btn-back">Retour à l'accueil</a>
           </section>"#,
        message = escape_html(message),
    );
    base_layout(&format!("Erreur {status} — COG Portal"), &content, nonce, None)
}

/// Layout HTML de base avec CSP nonce sur les balises style/script.
fn base_layout(title: &str, content: &str, nonce: &str, active_service: Option<&str>) -> String {
    let breadcrumb = active_service
        .map(|s| {
            format!(
                r#"<nav class="breadcrumb" aria-label="breadcrumb">
                     <a href="/">Accueil</a> &rsaquo; <span>{s}</span>
                   </nav>"#
            )
        })
        .unwrap_or_default();

    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title}</title>
  <style nonce="{nonce}">
    *, *::before, *::after {{ box-sizing: border-box; }}
    body {{ font-family: system-ui, sans-serif; margin: 0; background: #0f0f14; color: #e0e0e0; line-height: 1.6; }}
    header {{ background: #1a1a24; border-bottom: 1px solid #2a2a3a; padding: 12px 24px; display: flex; align-items: center; gap: 16px; }}
    header a {{ color: #a78bfa; text-decoration: none; font-weight: 600; font-size: 18px; }}
    .container {{ max-width: 900px; margin: 0 auto; padding: 24px; }}
    .breadcrumb {{ font-size: 14px; color: #888; margin-bottom: 16px; }}
    .breadcrumb a {{ color: #a78bfa; text-decoration: none; }}
    .services-grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 16px; margin-top: 24px; }}
    .service-card {{ background: #1a1a24; border: 1px solid #2a2a3a; border-radius: 8px; padding: 20px; }}
    .service-card h2 {{ margin: 0 0 8px; font-size: 18px; }}
    .service-card h2 a {{ color: #a78bfa; text-decoration: none; }}
    .service-nav ul {{ list-style: none; padding: 0; display: flex; gap: 12px; flex-wrap: wrap; }}
    .service-nav a {{ color: #a78bfa; text-decoration: none; border: 1px solid #a78bfa; padding: 6px 12px; border-radius: 4px; font-size: 14px; }}
    .contact-form {{ max-width: 540px; }}
    .form-field {{ margin-bottom: 16px; display: flex; flex-direction: column; gap: 4px; }}
    .form-field label {{ font-size: 14px; color: #aaa; }}
    .form-field input, .form-field textarea {{ background: #1a1a24; border: 1px solid #2a2a3a; border-radius: 4px; padding: 8px 12px; color: #e0e0e0; font-size: 14px; }}
    .btn-primary {{ background: #a78bfa; color: #0f0f14; border: none; border-radius: 4px; padding: 10px 20px; cursor: pointer; font-weight: 600; }}
    .error-page {{ text-align: center; padding: 80px 24px; }}
    .error-code {{ font-size: 72px; color: #a78bfa; margin: 0; }}
    .contact-error {{ color: #f87171; background: #3f1515; border-radius: 4px; padding: 10px 16px; margin-bottom: 16px; }}
    .contact-success {{ color: #4ade80; background: #0f2a1a; border-radius: 4px; padding: 16px; }}
  </style>
</head>
<body>
  <header>
    <a href="/">COG Web Portal</a>
  </header>
  <div class="container">
    {breadcrumb}
    {content}
  </div>
</body>
</html>"#
    )
}

/// Échappe les caractères HTML dangereux pour prévenir XSS.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_xss() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("a&b"), "a&amp;b");
    }

    #[test]
    fn render_error_contains_status() {
        let html = render_error(404, "Page introuvable", "nonce123");
        assert!(html.contains("404"));
        assert!(html.contains("nonce123"));
    }

    #[test]
    fn render_home_empty_services() {
        let html = render_home(&[], "test-nonce");
        assert!(html.contains("Aucun service"));
        assert!(html.contains("test-nonce"));
    }
}
