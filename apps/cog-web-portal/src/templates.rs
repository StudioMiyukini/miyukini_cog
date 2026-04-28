//! Templates HTML du COG Web Portal.
//!
//! @id: cog_web_portal_templates @do: render_html_portal_pages
//! @role: ui @layer: app
//! @human: Templates Portal — page d'accueil services, page de service, page d'erreur, formulaire contact.

#![allow(clippy::format_collect)]

use cog_portal_contract::PublicPage;

/// Infos d'un service pour la page d'accueil.
pub struct ServiceInfo {
    pub slug: String,
    pub name: String,
    pub page_count: usize,
}

/// Génère la page d'accueil listant les services enregistrés.
pub fn render_home(services: &[ServiceInfo], nonce: &str) -> String {
    let remote_link = if std::env::var("PORTAL_CENTRAL_REMOTE_ADDR").is_ok() {
        r#"<article class="service-card" style="border-color:#a78bfa">
             <h2><a href="/central-remote">CentralRemote</a></h2>
             <p class="page-count">Contrôle à distance de Central</p>
           </article>"#
    } else {
        ""
    };
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

    let content = if services.is_empty() && remote_link.is_empty() {
        "<p class=\"empty\">Aucun service disponible pour le moment.</p>".to_string()
    } else {
        format!("<section class=\"services-grid\">{remote_link}{service_cards}</section>")
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
        .map_or("<p>Aucun contenu disponible.</p>", |p| {
            p.html_content.as_str()
        });

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

// ═══════════════════════════════════════════════════════════════════════════
// CentralRemote — Templates de contrôle à distance
// ═══════════════════════════════════════════════════════════════════════════

/// Page de connexion CentralRemote.
pub fn render_central_remote_login(remote_addr: &str, nonce: &str) -> String {
    let remote_addr_escaped = escape_html(remote_addr);

    let content = format!(
        r##"<section class="remote-login">
          <h1>CentralRemote</h1>
          <p class="remote-subtitle">Contrôle à distance de votre Central</p>
          <div id="remote-error" class="contact-error" style="display:none"></div>
          <form id="remote-login-form" class="contact-form" onsubmit="return false">
            <div class="form-field">
              <label for="email">Email</label>
              <input type="email" id="email" name="email" required maxlength="254" autocomplete="email">
            </div>
            <div class="form-field">
              <label for="password">Mot de passe</label>
              <input type="password" id="password" name="password" required maxlength="200" autocomplete="current-password">
            </div>
            <button type="submit" class="btn-primary" id="btn-login">Se connecter</button>
          </form>
        </section>
        <script nonce="{nonce}">
        (function() {{
          const REMOTE = "{remote_addr_escaped}";
          const form = document.getElementById('remote-login-form');
          const errDiv = document.getElementById('remote-error');
          const btn = document.getElementById('btn-login');

          form.addEventListener('submit', async function() {{
            btn.disabled = true;
            btn.textContent = 'Connexion…';
            errDiv.style.display = 'none';

            try {{
              const resp = await fetch('http://' + REMOTE + '/auth', {{
                method: 'POST',
                headers: {{'Content-Type': 'application/json'}},
                body: JSON.stringify({{
                  email: document.getElementById('email').value,
                  password: document.getElementById('password').value
                }})
              }});
              const data = await resp.json();
              if (data.success && data.token) {{
                sessionStorage.setItem('central_remote_token', data.token);
                sessionStorage.setItem('central_remote_user', data.user_display_name || '');
                sessionStorage.setItem('central_remote_addr', REMOTE);
                window.location.href = '/central-remote/app';
              }} else {{
                errDiv.textContent = data.error || 'Identifiants incorrects';
                errDiv.style.display = 'block';
              }}
            }} catch(e) {{
              errDiv.textContent = 'Impossible de contacter Central. Vérifiez que le remote est actif.';
              errDiv.style.display = 'block';
            }}

            btn.disabled = false;
            btn.textContent = 'Se connecter';
          }});
        }})();
        </script>"##
    );

    base_layout("CentralRemote — Connexion", &content, nonce, None)
}

/// Interface de contrôle à distance CentralRemote.
pub fn render_central_remote_ui(remote_addr: &str, nonce: &str) -> String {
    let remote_addr_escaped = escape_html(remote_addr);

    let content = format!(
        r##"<div id="remote-app">
          <div class="remote-header">
            <h1>CentralRemote</h1>
            <span id="remote-user" class="remote-user"></span>
            <span id="remote-status" class="remote-status remote-status--connecting">Connexion…</span>
            <button id="btn-disconnect" class="btn-secondary" onclick="disconnect()">Déconnexion</button>
          </div>

          <nav id="remote-main-tabs" class="remote-tabs"></nav>

          <div id="remote-open-tabs" class="remote-open-tabs"></div>

          <div id="remote-services" class="services-grid"></div>

          <div id="remote-empty" class="empty" style="display:none">
            <p>Aucun service disponible.</p>
          </div>
        </div>

        <style nonce="{nonce}">
          .remote-header {{ display: flex; align-items: center; gap: 16px; margin-bottom: 24px; flex-wrap: wrap; }}
          .remote-header h1 {{ margin: 0; font-size: 22px; color: #a78bfa; }}
          .remote-user {{ color: #ccc; font-size: 14px; }}
          .remote-status {{ font-size: 12px; padding: 4px 10px; border-radius: 12px; font-weight: 600; }}
          .remote-status--connecting {{ background: #854d0e; color: #fbbf24; }}
          .remote-status--connected {{ background: #14532d; color: #4ade80; }}
          .remote-status--disconnected {{ background: #450a0a; color: #f87171; }}
          .remote-tabs {{ display: flex; gap: 8px; margin-bottom: 16px; }}
          .remote-tabs button {{ background: #1a1a24; border: 1px solid #2a2a3a; color: #e0e0e0; padding: 8px 16px; border-radius: 6px; cursor: pointer; font-size: 14px; }}
          .remote-tabs button.active {{ background: #a78bfa; color: #0f0f14; border-color: #a78bfa; font-weight: 600; }}
          .remote-open-tabs {{ display: flex; gap: 6px; margin-bottom: 16px; flex-wrap: wrap; }}
          .remote-open-tab {{ display: flex; align-items: center; gap: 6px; background: #1a1a24; border: 1px solid #2a2a3a; padding: 6px 12px; border-radius: 4px; font-size: 13px; cursor: pointer; color: #ccc; }}
          .remote-open-tab.active {{ border-color: #a78bfa; color: #a78bfa; }}
          .remote-open-tab .close {{ color: #888; cursor: pointer; margin-left: 4px; }}
          .remote-open-tab .close:hover {{ color: #f87171; }}
          .btn-secondary {{ background: transparent; border: 1px solid #2a2a3a; color: #888; padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 13px; margin-left: auto; }}
          .btn-secondary:hover {{ border-color: #f87171; color: #f87171; }}
          .remote-svc-card {{ background: #1a1a24; border: 1px solid #2a2a3a; border-radius: 8px; padding: 16px; cursor: pointer; transition: border-color 0.2s; }}
          .remote-svc-card:hover {{ border-color: #a78bfa; }}
          .remote-svc-card h3 {{ margin: 0 0 6px; font-size: 16px; color: #e0e0e0; }}
          .remote-svc-card .svc-icon {{ font-size: 24px; margin-bottom: 8px; }}
          .remote-svc-card p {{ margin: 0; font-size: 13px; color: #888; }}
          .remote-svc-card .svc-badges {{ display: flex; gap: 6px; margin-top: 8px; }}
          .remote-svc-card .badge {{ font-size: 11px; padding: 2px 8px; border-radius: 10px; }}
          .badge-installed {{ background: #14532d; color: #4ade80; }}
          .badge-favorite {{ background: #854d0e; color: #fbbf24; }}
        </style>

        <script nonce="{nonce}">
        (function() {{
          const REMOTE = "{remote_addr_escaped}";
          const token = sessionStorage.getItem('central_remote_token');
          const userName = sessionStorage.getItem('central_remote_user');

          if (!token) {{
            window.location.href = '/central-remote';
            return;
          }}

          document.getElementById('remote-user').textContent = userName || '';

          let ws = null;
          let snapshot = null;

          function connect() {{
            const url = 'ws://' + REMOTE + '/ws?token=' + encodeURIComponent(token);
            ws = new WebSocket(url);

            ws.onopen = function() {{
              setStatus('connected', 'Connecté');
            }};

            ws.onmessage = function(evt) {{
              try {{
                const event = JSON.parse(evt.data);
                handleEvent(event);
              }} catch(e) {{}}
            }};

            ws.onclose = function() {{
              setStatus('disconnected', 'Déconnecté');
              setTimeout(connect, 3000);
            }};

            ws.onerror = function() {{
              setStatus('disconnected', 'Erreur');
            }};
          }}

          function setStatus(cls, text) {{
            const el = document.getElementById('remote-status');
            el.className = 'remote-status remote-status--' + cls;
            el.textContent = text;
          }}

          function handleEvent(event) {{
            switch(event.type) {{
              case 'snapshot':
                snapshot = event;
                renderAll();
                break;
              case 'tab_opened':
                if (snapshot) {{ snapshot.open_tabs.push(event.tab); renderOpenTabs(); }}
                break;
              case 'tab_closed':
                if (snapshot) {{
                  snapshot.open_tabs.splice(event.tab_index, 1);
                  if (snapshot.active_tab_index >= snapshot.open_tabs.length)
                    snapshot.active_tab_index = Math.max(0, snapshot.open_tabs.length - 1);
                  renderOpenTabs();
                }}
                break;
              case 'tab_activated':
                if (snapshot) {{ snapshot.active_tab_index = event.tab_index; renderOpenTabs(); }}
                break;
              case 'main_tab_changed':
                if (snapshot) {{ snapshot.main_tab = event.tab; renderMainTabs(); }}
                break;
              case 'error':
                console.error('CentralRemote error:', event.message);
                break;
              case 'pong':
                break;
            }}
          }}

          function renderAll() {{
            if (!snapshot) return;
            renderMainTabs();
            renderOpenTabs();
            renderServices();
          }}

          function renderMainTabs() {{
            const tabs = ['SALON', 'SERVICES', 'WEBWAY', 'MES AMIS'];
            const tabMap = {{'SALON': 'Salon', 'SERVICES': 'Bibliotheque', 'WEBWAY': 'Communaute', 'MES AMIS': 'MesAmis'}};
            const reverseMap = {{'Salon': 'SALON', 'Bibliotheque': 'SERVICES', 'Communaute': 'WEBWAY', 'MesAmis': 'MES AMIS'}};
            const container = document.getElementById('remote-main-tabs');
            container.innerHTML = tabs.map(function(t) {{
              const active = (reverseMap[snapshot.main_tab] === t) ? ' active' : '';
              return '<button class="' + active + '" onclick="sendCmd(\'navigate_tab\', \'' + tabMap[t] + '\')">' + t + '</button>';
            }}).join('');
          }}

          function renderOpenTabs() {{
            const container = document.getElementById('remote-open-tabs');
            container.innerHTML = snapshot.open_tabs.map(function(tab, i) {{
              const active = (i === snapshot.active_tab_index) ? ' active' : '';
              const closeBtn = tab.closable
                ? '<span class="close" onclick="event.stopPropagation(); sendCmd(\'close_tab_idx\', ' + i + ')">×</span>'
                : '';
              return '<div class="remote-open-tab' + active + '" onclick="sendCmd(\'activate_tab_idx\', ' + i + ')">'
                + tab.title + closeBtn + '</div>';
            }}).join('');
          }}

          function renderServices() {{
            const container = document.getElementById('remote-services');
            const empty = document.getElementById('remote-empty');
            if (!snapshot.services || snapshot.services.length === 0) {{
              container.innerHTML = '';
              empty.style.display = 'block';
              return;
            }}
            empty.style.display = 'none';
            container.innerHTML = snapshot.services.map(function(svc) {{
              const badges = [];
              if (svc.is_installed) badges.push('<span class="badge badge-installed">Installé</span>');
              if (svc.is_favorite) badges.push('<span class="badge badge-favorite">Favori</span>');
              return '<article class="remote-svc-card" onclick="sendCmd(\'open_svc\', \'' + svc.id + '\')">'
                + '<div class="svc-icon">' + svc.icon + '</div>'
                + '<h3>' + svc.name + '</h3>'
                + '<p>' + svc.description + '</p>'
                + '<div class="svc-badges">' + badges.join('') + '</div>'
                + '</article>';
            }}).join('');
          }}

          window.sendCmd = function(action, value) {{
            if (!ws || ws.readyState !== 1) return;
            let cmd;
            switch(action) {{
              case 'navigate_tab':
                cmd = {{ type: 'navigate_tab', tab: value }};
                break;
              case 'open_svc':
                cmd = {{ type: 'open_service', service_id: value }};
                break;
              case 'close_tab_idx':
                cmd = {{ type: 'close_tab', tab_index: value }};
                break;
              case 'activate_tab_idx':
                cmd = {{ type: 'activate_tab', tab_index: value }};
                break;
              default:
                return;
            }}
            ws.send(JSON.stringify(cmd));
          }};

          window.disconnect = function() {{
            sessionStorage.removeItem('central_remote_token');
            sessionStorage.removeItem('central_remote_user');
            sessionStorage.removeItem('central_remote_addr');
            if (ws) ws.close();
            window.location.href = '/central-remote';
          }};

          // Ping keepalive toutes les 30s.
          setInterval(function() {{
            if (ws && ws.readyState === 1) {{
              ws.send(JSON.stringify({{ type: 'ping' }}));
            }}
          }}, 30000);

          connect();
        }})();
        </script>"##
    );

    base_layout("CentralRemote", &content, nonce, None)
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
