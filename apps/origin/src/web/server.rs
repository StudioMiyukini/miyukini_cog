//! Serveur Web Origin — Implémentation HTTP.

use super::{api, content::ContentManager, pages};
use crate::config::OriginConfig;
use crate::tracker::{catalog::Catalog, pool::PoolManager};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tracing::{debug, error, info};

/// Serveur web public.
pub struct WebServer {
    config: Arc<OriginConfig>,
    pool_manager: Arc<PoolManager>,
    content_manager: Arc<ContentManager>,
}

impl WebServer {
    /// Crée un nouveau serveur web.
    #[must_use]
    pub fn new(config: Arc<OriginConfig>, pool_manager: Arc<PoolManager>) -> Self {
        Self {
            config,
            pool_manager,
            content_manager: Arc::new(ContentManager::new()),
        }
    }

    /// Démarre le serveur web.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bind_addr = format!("0.0.0.0:{}", self.config.tracker.web_port);
        let listener = TcpListener::bind(&bind_addr).await?;
        info!("🌐 Web server listening on http://{}", bind_addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let config = Arc::clone(&self.config);
                    let pool_mgr = Arc::clone(&self.pool_manager);
                    let content_mgr = Arc::clone(&self.content_manager);

                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, config, pool_mgr, content_mgr).await {
                            debug!("Connection error from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    }
}

/// Gère une connexion HTTP.
async fn handle_connection(
    mut stream: tokio::net::TcpStream,
    _config: Arc<OriginConfig>,
    pool_mgr: Arc<PoolManager>,
    content_mgr: Arc<ContentManager>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).await?;

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    // Lire les headers (pour ignorer le corps)
    let mut _headers = Vec::new();
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).await?;
        if line.trim().is_empty() {
            break;
        }
        _headers.push(line);
    }

    debug!("{} {}", method, path);

    // Router
    let (status, content_type, body) = route_request(path, &pool_mgr, &content_mgr).await;

    // Réponse HTTP
    let response = format!(
        "HTTP/1.1 {}\r\n\
         Content-Type: {}; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Connection: close\r\n\
         \r\n",
        status,
        content_type,
        body.len()
    );

    writer.write_all(response.as_bytes()).await?;
    writer.write_all(body.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}

/// Route les requêtes HTTP.
async fn route_request(
    path: &str,
    pool_mgr: &PoolManager,
    content_mgr: &ContentManager,
) -> (String, String, String) {
    // Séparer path et query string
    let (path_only, _query) = path.split_once('?').unwrap_or((path, ""));

    // Nettoyer le path
    let path_clean = path_only.trim_end_matches('/');
    let path_clean = if path_clean.is_empty() { "/" } else { path_clean };

    match path_clean {
        // ═══════════════════════════════════════════════════════════════════
        // Pages HTML
        // ═══════════════════════════════════════════════════════════════════
        "/" => {
            let body = pages::home_page(content_mgr, pool_mgr).await;
            ("200 OK".to_string(), "text/html".to_string(), body)
        }

        "/docs" => {
            let body = pages::docs_page(content_mgr).await;
            ("200 OK".to_string(), "text/html".to_string(), body)
        }

        "/downloads" => {
            let body = pages::downloads_page(content_mgr).await;
            ("200 OK".to_string(), "text/html".to_string(), body)
        }

        "/blog" => {
            let body = pages::blog_page(content_mgr).await;
            ("200 OK".to_string(), "text/html".to_string(), body)
        }

        "/announcements" => {
            let body = pages::announcements_page(content_mgr).await;
            ("200 OK".to_string(), "text/html".to_string(), body)
        }

        "/catalog" => {
            let catalog = Catalog::from_pools(pool_mgr).await;
            let body = catalog.to_html();
            ("200 OK".to_string(), "text/html".to_string(), body)
        }

        "/services" => {
            let body = pages::services_page(content_mgr).await;
            ("200 OK".to_string(), "text/html".to_string(), body)
        }

        "/about" => {
            let body = pages::about_page();
            ("200 OK".to_string(), "text/html".to_string(), body)
        }

        // ═══════════════════════════════════════════════════════════════════
        // API JSON
        // ═══════════════════════════════════════════════════════════════════
        "/api/health" => {
            let body = api::api_health();
            ("200 OK".to_string(), "application/json".to_string(), body)
        }

        "/api/status" => {
            let body = api::api_status(pool_mgr).await;
            ("200 OK".to_string(), "application/json".to_string(), body)
        }

        "/api/blog" => {
            let body = api::api_blog(content_mgr).await;
            ("200 OK".to_string(), "application/json".to_string(), body)
        }

        "/api/announcements" => {
            let body = api::api_announcements(content_mgr).await;
            ("200 OK".to_string(), "application/json".to_string(), body)
        }

        "/api/downloads" => {
            let body = api::api_downloads(content_mgr).await;
            ("200 OK".to_string(), "application/json".to_string(), body)
        }

        "/api/docs" => {
            let body = api::api_docs(content_mgr).await;
            ("200 OK".to_string(), "application/json".to_string(), body)
        }

        "/api/catalog" => {
            let body = api::api_catalog(pool_mgr).await;
            ("200 OK".to_string(), "application/json".to_string(), body)
        }

        // ═══════════════════════════════════════════════════════════════════
        // Routes dynamiques
        // ═══════════════════════════════════════════════════════════════════
        _ if path_clean.starts_with("/blog/") => {
            let post_id = &path_clean[6..]; // Après "/blog/"
            if let Some(body) = pages::blog_post_page(content_mgr, post_id).await {
                ("200 OK".to_string(), "text/html".to_string(), body)
            } else {
                not_found_page()
            }
        }

        _ if path_clean.starts_with("/api/blog/") => {
            let post_id = &path_clean[10..]; // Après "/api/blog/"
            if let Some(body) = api::api_blog_post(content_mgr, post_id).await {
                ("200 OK".to_string(), "application/json".to_string(), body)
            } else {
                ("404 Not Found".to_string(), "application/json".to_string(), r#"{"error": "Post not found"}"#.to_string())
            }
        }

        _ if path_clean.starts_with("/api/downloads/") => {
            let category = &path_clean[15..]; // Après "/api/downloads/"
            if let Some(body) = api::api_downloads_by_category(content_mgr, category).await {
                ("200 OK".to_string(), "application/json".to_string(), body)
            } else {
                ("404 Not Found".to_string(), "application/json".to_string(), r#"{"error": "Category not found"}"#.to_string())
            }
        }

        _ if path_clean.starts_with("/docs/") => {
            // /docs/{section} ou /docs/{section}/{article}
            let remainder = &path_clean[6..];
            let parts: Vec<&str> = remainder.split('/').collect();

            match parts.as_slice() {
                [section_id] => {
                    if let Some(section) = content_mgr.get_doc_section(section_id).await {
                        let body = doc_section_page(&section);
                        ("200 OK".to_string(), "text/html".to_string(), body)
                    } else {
                        not_found_page()
                    }
                }
                [section_id, article_id] => {
                    if let Some(article) = content_mgr.get_doc_article(section_id, article_id).await {
                        let body = doc_article_page(section_id, &article);
                        ("200 OK".to_string(), "text/html".to_string(), body)
                    } else {
                        not_found_page()
                    }
                }
                _ => not_found_page(),
            }
        }

        _ if path_clean.starts_with("/api/docs/") => {
            let remainder = &path_clean[10..];
            let parts: Vec<&str> = remainder.split('/').collect();

            match parts.as_slice() {
                [section_id] => {
                    if let Some(body) = api::api_doc_section(content_mgr, section_id).await {
                        ("200 OK".to_string(), "application/json".to_string(), body)
                    } else {
                        ("404 Not Found".to_string(), "application/json".to_string(), r#"{"error": "Section not found"}"#.to_string())
                    }
                }
                [section_id, article_id] => {
                    if let Some(body) = api::api_doc_article(content_mgr, section_id, article_id).await {
                        ("200 OK".to_string(), "application/json".to_string(), body)
                    } else {
                        ("404 Not Found".to_string(), "application/json".to_string(), r#"{"error": "Article not found"}"#.to_string())
                    }
                }
                _ => ("404 Not Found".to_string(), "application/json".to_string(), r#"{"error": "Invalid path"}"#.to_string()),
            }
        }

        // ═══════════════════════════════════════════════════════════════════
        // 404
        // ═══════════════════════════════════════════════════════════════════
        _ => not_found_page(),
    }
}

/// Page 404.
fn not_found_page() -> (String, String, String) {
    let body = r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 — Miyukini</title>
    <style>
        :root {
            --primary: #8b5cf6;
            --bg: #0a0a0f;
            --text: #f0f0f5;
            --text-muted: #9ca3af;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: 'Inter', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            text-align: center;
        }
        .container { padding: 2rem; }
        h1 { font-size: 8rem; color: var(--primary); opacity: 0.5; }
        h2 { font-size: 2rem; margin: 1rem 0; }
        p { color: var(--text-muted); margin-bottom: 2rem; }
        a {
            display: inline-block;
            background: var(--primary);
            color: white;
            padding: 0.75rem 1.5rem;
            border-radius: 0.5rem;
            text-decoration: none;
            font-weight: 600;
        }
        a:hover { transform: translateY(-2px); }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <h2>Page non trouvée</h2>
        <p>Le chemin que vous cherchez n'existe pas ou a été déplacé.</p>
        <a href="/">Retour à l'accueil</a>
    </div>
</body>
</html>"#.to_string();

    ("404 Not Found".to_string(), "text/html".to_string(), body)
}

/// Page d'une section de documentation.
fn doc_section_page(section: &super::content::DocSection) -> String {
    let articles_html: String = section.articles
        .iter()
        .map(|a| {
            format!(
                r#"<a href="/docs/{}/{}" class="card" style="display: block;">
                    <h3>{}</h3>
                    <p style="color: var(--text-muted); font-size: 0.875rem;">
                        Mis à jour le {}
                    </p>
                </a>"#,
                pages::html_escape(&section.id),
                pages::html_escape(&a.id),
                pages::html_escape(&a.title),
                a.updated_at.format("%d %B %Y")
            )
        })
        .collect();

    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Documentation — Miyukini</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {{
            --primary: #8b5cf6;
            --bg: #0a0a0f;
            --bg-surface: #12121a;
            --bg-elevated: #1a1a25;
            --text: #f0f0f5;
            --text-muted: #9ca3af;
            --border: rgba(139, 92, 246, 0.2);
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: 'Inter', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            padding: 2rem;
        }}
        a {{ color: var(--primary); text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .breadcrumb {{ margin-bottom: 2rem; color: var(--text-muted); }}
        h1 {{ font-size: 2rem; margin-bottom: 0.5rem; }}
        .description {{ color: var(--text-muted); margin-bottom: 2rem; }}
        .card {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 0.5rem;
            padding: 1rem;
            margin-bottom: 0.75rem;
            transition: border-color 0.2s;
        }}
        .card:hover {{ border-color: var(--primary); }}
        h3 {{ margin-bottom: 0.25rem; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="breadcrumb">
            <a href="/">Accueil</a> / <a href="/docs">Documentation</a> / {}
        </div>
        <h1>{} {}</h1>
        <p class="description">{}</p>
        <div class="articles">
            {}
        </div>
    </div>
</body>
</html>"#,
        pages::html_escape(&section.title),
        pages::html_escape(&section.title),
        pages::html_escape(&section.icon),
        pages::html_escape(&section.title),
        pages::html_escape(&section.description),
        articles_html
    )
}

/// Page d'un article de documentation.
fn doc_article_page(section_id: &str, article: &super::content::DocArticle) -> String {
    // Simple markdown to HTML
    let content_html = simple_md_to_html(&article.content);

    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Documentation — Miyukini</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono&display=swap" rel="stylesheet">
    <style>
        :root {{
            --primary: #8b5cf6;
            --bg: #0a0a0f;
            --bg-surface: #12121a;
            --bg-elevated: #1a1a25;
            --text: #f0f0f5;
            --text-muted: #9ca3af;
            --border: rgba(139, 92, 246, 0.2);
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: 'Inter', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            padding: 2rem;
        }}
        a {{ color: var(--primary); text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .breadcrumb {{ margin-bottom: 2rem; color: var(--text-muted); }}
        h1 {{ font-size: 2rem; margin-bottom: 0.5rem; }}
        h2 {{ font-size: 1.5rem; margin: 2rem 0 1rem; }}
        h3 {{ font-size: 1.25rem; margin: 1.5rem 0 0.75rem; }}
        p {{ margin-bottom: 1rem; }}
        code {{ font-family: 'JetBrains Mono', monospace; background: var(--bg-elevated); padding: 0.2rem 0.4rem; border-radius: 0.25rem; }}
        pre {{
            background: var(--bg-elevated);
            border: 1px solid var(--border);
            border-radius: 0.5rem;
            padding: 1rem;
            overflow-x: auto;
            margin: 1rem 0;
        }}
        pre code {{ background: none; padding: 0; }}
        ul, ol {{ margin: 1rem 0; padding-left: 1.5rem; }}
        li {{ margin-bottom: 0.5rem; }}
        table.doc-table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1rem 0;
        }}
        table.doc-table th, table.doc-table td {{
            border: 1px solid var(--border);
            padding: 0.5rem 0.75rem;
            text-align: left;
        }}
        table.doc-table th {{ background: var(--bg-elevated); font-weight: 600; }}
        table.doc-table tr:hover td {{ background: rgba(139, 92, 246, 0.05); }}
        blockquote {{ border-left: 4px solid var(--primary); padding-left: 1rem; margin: 1rem 0; color: var(--text-muted); font-style: italic; }}
        .meta {{ color: var(--text-muted); font-size: 0.875rem; margin-bottom: 2rem; }}
        .content {{ background: var(--bg-surface); border-radius: 0.5rem; padding: 2rem; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="breadcrumb">
            <a href="/">Accueil</a> / <a href="/docs">Documentation</a> / <a href="/docs/{}">{}</a>
        </div>
        <h1>{}</h1>
        <p class="meta">Mis à jour le {}</p>
        <div class="content">
            {}
        </div>
    </div>
</body>
</html>"#,
        pages::html_escape(&article.title),
        pages::html_escape(section_id),
        pages::html_escape(section_id),
        pages::html_escape(&article.title),
        article.updated_at.format("%d %B %Y"),
        content_html
    )
}

/// Convertit les liens Markdown [text](url) en HTML.
fn md_links_to_html(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find('[') {
        if let Some(mid) = rest[start..].find(']') {
            let mid = start + mid;
            if rest.get(mid + 1..mid + 2) == Some("(") {
                if let Some(end) = rest[mid + 2..].find(')') {
                    let end = mid + 2 + end;
                    let text = &rest[start + 1..mid];
                    let url = &rest[mid + 2..end];
                    result.push_str(&pages::html_escape(&rest[..start]));
                    result.push_str("<a href=\"");
                    result.push_str(&pages::html_escape(url));
                    result.push_str("\">");
                    result.push_str(&pages::html_escape(text));
                    result.push_str("</a>");
                    rest = &rest[end + 1..];
                    continue;
                }
            }
        }
        result.push_str(&pages::html_escape(&rest[..start + 1]));
        rest = &rest[start + 1..];
    }
    result.push_str(&pages::html_escape(rest));
    result
}

/// Conversion Markdown basique en HTML.
fn simple_md_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_table = false;

    for line in markdown.lines() {
        if line.starts_with("```") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                html.push_str("<pre><code>");
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            html.push_str(&pages::html_escape(line));
            html.push('\n');
            continue;
        }

        if line.is_empty() {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            continue;
        } else if line.starts_with("# ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<h1>{}</h1>\n", md_links_to_html(&line[2..])));
        } else if line.starts_with("## ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<h2>{}</h2>\n", md_links_to_html(&line[3..])));
        } else if line.starts_with("### ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<h3>{}</h3>\n", md_links_to_html(&line[4..])));
        } else if line.starts_with("|") && line.contains('|') {
            let cells: Vec<&str> = line.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            if !in_table {
                html.push_str(r#"<table class="doc-table"><thead><tr>"#);
                for cell in &cells {
                    html.push_str("<th>");
                    html.push_str(&md_links_to_html(cell));
                    html.push_str("</th>");
                }
                html.push_str("</tr></thead><tbody>");
                in_table = true;
            } else if cells.iter().all(|c| c.chars().all(|ch| ch == '-' || ch == ':')) {
                continue;
            } else {
                html.push_str("<tr>");
                for cell in &cells {
                    html.push_str("<td>");
                    html.push_str(&md_links_to_html(cell));
                    html.push_str("</td>");
                }
                html.push_str("</tr>");
            }
        } else if line.starts_with("- ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<li>{}</li>\n", md_links_to_html(&line[2..])));
        } else if line.starts_with("> ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<blockquote>{}</blockquote>\n", md_links_to_html(&line[2..])));
        } else {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<p>{}</p>\n", md_links_to_html(line)));
        }
    }

    if in_table {
        html.push_str("</tbody></table>\n");
    }
    if in_code_block {
        html.push_str("</code></pre>\n");
    }

    html
}
