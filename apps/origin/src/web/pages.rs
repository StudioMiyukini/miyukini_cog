//! Générateur de pages HTML pour le site web Origin.

use super::content::{AnnouncementType, ContentManager, DownloadCategory, Platform};
use super::server::simple_md_to_html;
use crate::tracker::pool::PoolManager;
use jayxpose::JayXposeDb;

/// Génère le layout HTML de base.
fn layout(title: &str, content: &str, active_nav: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Miyukini COG - Environnement de gouvernance orchestré par des Cores">
    <title>{title} — Miyukini</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono&display=swap" rel="stylesheet">
    <style>
        :root {{
            --primary: #8b5cf6;
            --primary-dark: #7c3aed;
            --secondary: #06b6d4;
            --accent: #f59e0b;
            --bg: #0a0a0f;
            --bg-surface: #12121a;
            --bg-elevated: #1a1a25;
            --text: #f0f0f5;
            --text-muted: #9ca3af;
            --border: rgba(139, 92, 246, 0.2);
            --success: #10b981;
            --warning: #f59e0b;
            --error: #ef4444;
            --gradient-1: linear-gradient(135deg, var(--primary), var(--secondary));
            --gradient-2: linear-gradient(135deg, #667eea, #764ba2);
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        html {{ scroll-behavior: smooth; }}
        body {{
            font-family: 'Inter', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            min-height: 100vh;
        }}
        a {{ color: var(--primary); text-decoration: none; transition: color 0.2s; }}
        a:hover {{ color: var(--secondary); }}
        code, pre {{ font-family: 'JetBrains Mono', monospace; }}
        pre {{
            background: var(--bg-elevated);
            border: 1px solid var(--border);
            border-radius: 0.5rem;
            padding: 1rem;
            overflow-x: auto;
        }}

        /* Header */
        .header {{
            background: var(--bg-surface);
            border-bottom: 1px solid var(--border);
            position: sticky;
            top: 0;
            z-index: 100;
            backdrop-filter: blur(10px);
        }}
        .header-inner {{
            max-width: 1400px;
            margin: 0 auto;
            padding: 0 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            height: 70px;
        }}
        .logo {{
            display: flex;
            align-items: center;
            gap: 0.75rem;
            font-size: 1.5rem;
            font-weight: 700;
            background: var(--gradient-1);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .logo-icon {{
            width: 40px;
            height: 40px;
            background: var(--gradient-1);
            border-radius: 10px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.25rem;
            -webkit-text-fill-color: white;
        }}
        nav {{ display: flex; gap: 0.5rem; }}
        nav a {{
            padding: 0.5rem 1rem;
            border-radius: 0.5rem;
            color: var(--text-muted);
            font-weight: 500;
            transition: all 0.2s;
        }}
        nav a:hover {{ background: var(--bg-elevated); color: var(--text); }}
        nav a.active {{
            background: rgba(139, 92, 246, 0.15);
            color: var(--primary);
        }}

        /* Main */
        main {{ max-width: 1400px; margin: 0 auto; padding: 2rem; }}

        /* Hero */
        .hero {{
            text-align: center;
            padding: 4rem 0;
            background: radial-gradient(ellipse at center, rgba(139, 92, 246, 0.1) 0%, transparent 70%);
        }}
        .hero h1 {{
            font-size: 3.5rem;
            font-weight: 700;
            margin-bottom: 1rem;
            background: var(--gradient-1);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .hero p {{ font-size: 1.25rem; color: var(--text-muted); max-width: 600px; margin: 0 auto 2rem; }}
        .hero-buttons {{ display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap; }}

        /* Buttons */
        .btn {{
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.75rem 1.5rem;
            border-radius: 0.5rem;
            font-weight: 600;
            transition: all 0.2s;
            border: none;
            cursor: pointer;
        }}
        .btn-primary {{
            background: var(--gradient-1);
            color: white;
        }}
        .btn-primary:hover {{ transform: translateY(-2px); box-shadow: 0 4px 20px rgba(139, 92, 246, 0.4); }}
        .btn-secondary {{
            background: var(--bg-elevated);
            color: var(--text);
            border: 1px solid var(--border);
        }}
        .btn-secondary:hover {{ background: var(--bg-surface); border-color: var(--primary); }}

        /* Cards */
        .card {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 1rem;
            padding: 1.5rem;
            transition: all 0.2s;
        }}
        .card:hover {{ border-color: var(--primary); transform: translateY(-2px); }}
        .card h3 {{ font-size: 1.25rem; margin-bottom: 0.5rem; }}
        .card p {{ color: var(--text-muted); }}

        /* Grid */
        .grid {{ display: grid; gap: 1.5rem; }}
        .grid-2 {{ grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); }}
        .grid-3 {{ grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); }}
        .grid-4 {{ grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); }}

        /* Section */
        .section {{ padding: 3rem 0; }}
        .section-title {{
            font-size: 2rem;
            font-weight: 700;
            margin-bottom: 0.5rem;
        }}
        .section-subtitle {{ color: var(--text-muted); margin-bottom: 2rem; }}

        /* Features */
        .feature-icon {{
            width: 48px;
            height: 48px;
            background: rgba(139, 92, 246, 0.15);
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.5rem;
            margin-bottom: 1rem;
        }}

        /* Announcement banner */
        .announcement {{
            padding: 0.75rem 1rem;
            border-radius: 0.5rem;
            margin-bottom: 1rem;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }}
        .announcement.release {{ background: rgba(16, 185, 129, 0.15); border: 1px solid rgba(16, 185, 129, 0.3); }}
        .announcement.security {{ background: rgba(239, 68, 68, 0.15); border: 1px solid rgba(239, 68, 68, 0.3); }}
        .announcement.maintenance {{ background: rgba(245, 158, 11, 0.15); border: 1px solid rgba(245, 158, 11, 0.3); }}
        .announcement.info {{ background: rgba(6, 182, 212, 0.15); border: 1px solid rgba(6, 182, 212, 0.3); }}

        /* Blog */
        .blog-card {{ display: block; }}
        .blog-card .date {{ color: var(--text-muted); font-size: 0.875rem; margin-bottom: 0.5rem; }}
        .blog-card .tags {{ display: flex; gap: 0.5rem; margin-top: 1rem; flex-wrap: wrap; }}
        .tag {{
            background: var(--bg-elevated);
            padding: 0.25rem 0.75rem;
            border-radius: 1rem;
            font-size: 0.75rem;
            color: var(--text-muted);
        }}

        /* Download */
        .download-card {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            flex-wrap: wrap;
            gap: 1rem;
        }}
        .download-info h3 {{ margin-bottom: 0.25rem; }}
        .download-meta {{
            display: flex;
            gap: 1rem;
            color: var(--text-muted);
            font-size: 0.875rem;
            margin-top: 0.5rem;
        }}
        .platform-badge {{
            background: var(--bg-elevated);
            padding: 0.25rem 0.5rem;
            border-radius: 0.25rem;
            font-size: 0.75rem;
        }}

        /* Docs */
        .docs-sidebar {{
            position: sticky;
            top: 90px;
        }}
        .docs-nav a {{
            display: block;
            padding: 0.5rem 1rem;
            border-radius: 0.5rem;
            color: var(--text-muted);
            margin-bottom: 0.25rem;
        }}
        .docs-nav a:hover {{ background: var(--bg-elevated); color: var(--text); }}
        .docs-nav a.active {{ background: rgba(139, 92, 246, 0.15); color: var(--primary); }}

        /* Stats */
        .stat {{ text-align: center; }}
        .stat-value {{ font-size: 2.5rem; font-weight: 700; color: var(--primary); }}
        .stat-label {{ color: var(--text-muted); }}

        /* Footer */
        .footer {{
            background: var(--bg-surface);
            border-top: 1px solid var(--border);
            padding: 3rem 2rem;
            margin-top: 4rem;
        }}
        .footer-inner {{
            max-width: 1400px;
            margin: 0 auto;
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 2rem;
        }}
        .footer h4 {{ margin-bottom: 1rem; }}
        .footer ul {{ list-style: none; }}
        .footer li {{ margin-bottom: 0.5rem; }}
        .footer a {{ color: var(--text-muted); }}
        .footer a:hover {{ color: var(--primary); }}
        .footer__bottom {{
            max-width: 1400px;
            margin: 2rem auto 0;
            padding-top: 2rem;
            border-top: 1px solid var(--border);
            text-align: center;
            color: var(--text-muted);
        }}

        /* Responsive */
        @media (max-width: 768px) {{
            .header-inner {{ padding: 0 1rem; }}
            nav {{ display: none; }}
            .hero h1 {{ font-size: 2rem; }}
            main {{ padding: 1rem; }}
        }}
    </style>
</head>
<body>
    <header class="header">
        <div class="header-inner">
            <a href="/" class="logo">
                <span class="logo-icon">宮</span>
                <span>Miyukini</span>
            </a>
            <nav>
                <a href="/" class="{home_active}">Miyukini COG</a>
                <a href="/catalog" class="{catalog_active}">Catalogue</a>
                <a href="/services" class="{services_active}">Services</a>
                <a href="/downloads" class="{downloads_active}">Télécharger</a>
                <a href="/docs" class="{docs_active}">Documentation</a>
                <a href="/about" class="{about_active}">À propos</a>
                <a href="/blog" class="{blog_active}">Blog</a>
                <a href="/announcements" class="{announcements_active}">Annonces</a>
            </nav>
        </div>
    </header>
    <main>
        {content}
    </main>
    <footer class="footer">
        <div class="footer-inner">
            <div>
                <h4>Miyukini COG</h4>
                <ul>
                    <li><a href="/docs">Documentation</a></li>
                    <li><a href="/services">Les services</a></li>
                    <li><a href="/downloads">Téléchargements</a></li>
                    <li><a href="/catalog">Catalogue MWS</a></li>
                    <li><a href="/about">À propos</a></li>
                    <li><a href="/blog">Dev Blog</a></li>
                </ul>
            </div>
            <div>
                <h4>Communauté</h4>
                <ul>
                    <li><a href="/community/discord">Discord</a></li>
                    <li><a href="/community/github">GitHub</a></li>
                    <li><a href="/community/forum">Forum</a></li>
                </ul>
            </div>
            <div>
                <h4>Ressources</h4>
                <ul>
                    <li><a href="/docs/overview/introduction">Vue d'ensemble</a></li>
                    <li><a href="/docs/architecture">Architecture</a></li>
                    <li><a href="/docs/cores/overview">Les Cores</a></li>
                    <li><a href="/docs/mws">Miyukini Webway</a></li>
                </ul>
            </div>
            <div>
                <h4>Legal</h4>
                <ul>
                    <li><a href="/legal/terms">Conditions d'utilisation</a></li>
                    <li><a href="/legal/privacy">Confidentialité</a></li>
                    <li><a href="/legal/licenses">Licences</a></li>
                </ul>
            </div>
        </div>
        <div class="footer__bottom">
            <p>© 2026 Miyukini. Tous droits réservés. Construit avec ❤️ pour la souveraineté numérique.</p>
        </div>
    </footer>
</body>
</html>"#,
        title = title,
        content = content,
        home_active = if active_nav == "home" { "active" } else { "" },
        docs_active = if active_nav == "docs" { "active" } else { "" },
        downloads_active = if active_nav == "downloads" { "active" } else { "" },
        blog_active = if active_nav == "blog" { "active" } else { "" },
        announcements_active = if active_nav == "announcements" { "active" } else { "" },
        catalog_active = if active_nav == "catalog" { "active" } else { "" },
        services_active = if active_nav == "services" { "active" } else { "" },
        about_active = if active_nav == "about" { "active" } else { "" },
    )
}

/// Page d'accueil — Portail MWS.
pub async fn home_page(content_mgr: &ContentManager, pool_mgr: &PoolManager) -> String {
    let announcements = content_mgr.get_announcements().await;
    let blog_posts = content_mgr.get_blog_posts().await;
    let total_cogs = pool_mgr.total_cog_count().await;
    let versions = pool_mgr.list_versions().await;
    let lobbys = pool_mgr.list_all_public_lobbys().await;

    // Dernières annonces
    let announcements_html: String = announcements
        .iter()
        .take(2)
        .map(|a| {
            let type_class = match a.announcement_type {
                AnnouncementType::Release => "release",
                AnnouncementType::Security => "security",
                AnnouncementType::Maintenance => "maintenance",
                AnnouncementType::Info => "info",
            };
            format!(
                r#"<div class="announcement {}"><span>{}</span> <strong>{}</strong></div>"#,
                type_class,
                match a.announcement_type {
                    AnnouncementType::Release => "🚀",
                    AnnouncementType::Security => "🔒",
                    AnnouncementType::Maintenance => "🔧",
                    AnnouncementType::Info => "ℹ️",
                },
                html_escape(&a.title)
            )
        })
        .collect();

    // Derniers articles
    let recent_posts_html: String = blog_posts
        .iter()
        .take(3)
        .map(|p| {
            format!(
                r#"<a href="/blog/{}" class="card blog-card">
                    <div class="date">{}</div>
                    <h3>{}</h3>
                    <p>{}</p>
                    <div class="tags">{}</div>
                </a>"#,
                html_escape(&p.id),
                p.published_at.format("%d %B %Y"),
                html_escape(&p.title),
                html_escape(&p.summary),
                p.tags.iter().map(|t| format!(r#"<span class="tag">{}</span>"#, html_escape(t))).collect::<String>()
            )
        })
        .collect();

    // Lobbys publics (max 5)
    let lobbys_html: String = if lobbys.is_empty() {
        r#"<p class="empty-state">Aucun lobby public actif actuellement</p>"#.to_string()
    } else {
        lobbys
            .iter()
            .take(5)
            .map(|(version, cog_id, lobby)| {
                format!(
                    r#"<div class="lobby-card">
                        <div class="lobby-info">
                            <span class="lobby-name">{}</span>
                            <span class="lobby-host">Hébergé par {}</span>
                        </div>
                        <div class="lobby-meta">
                            <span class="lobby-players">👥 {}/{}</span>
                            <span class="lobby-version">v{}</span>
                            {}
                        </div>
                    </div>"#,
                    html_escape(&lobby.name),
                    html_escape(&cog_id[..cog_id.len().min(16)]),
                    lobby.current_players,
                    lobby.max_players,
                    html_escape(version),
                    if lobby.password_required { r#"<span class="lobby-lock">🔒</span>"# } else { "" }
                )
            })
            .collect()
    };

    let content = format!(
        r##"
        {announcements}

        <!-- Hero Section : Miyukini COG -->
        <section class="hero portal-hero">
            <div class="portal-badge">🌐 Miyukini Webway System — Origin</div>
            <h1>Miyukini COG</h1>
            <p>Page d'accueil : dernières nouvelles, métriques des téléchargements et du réseau, 
               tout ce qu'offre Miyukini COG. Découvrez les services, connectez-vous aux COGs 
               et rejoignez l'écosystème de souveraineté numérique.</p>
            <div class="hero-buttons">
                <a href="/catalog" class="btn btn-primary">📡 Explorer le Catalogue</a>
                <a href="/downloads" class="btn btn-secondary">⬇️ Télécharger COG</a>
            </div>
        </section>

        <!-- Statistiques du réseau en temps réel -->
        <section class="section network-stats">
            <div class="stats-header">
                <span class="live-indicator"></span>
                <span>Réseau en temps réel</span>
            </div>
            <div class="grid grid-4">
                <div class="stat stat-highlight">
                    <div class="stat-value">{total_cogs}</div>
                    <div class="stat-label">COGs connectés</div>
                </div>
                <div class="stat">
                    <div class="stat-value">{total_lobbys}</div>
                    <div class="stat-label">Lobbys actifs</div>
                </div>
                <div class="stat">
                    <div class="stat-value">{total_versions}</div>
                    <div class="stat-label">Versions</div>
                </div>
                <div class="stat">
                    <div class="stat-value online-status">●</div>
                    <div class="stat-label">Origin Status</div>
                </div>
            </div>
        </section>

        <!-- Section Portail : Accès rapide -->
        <section class="section portal-access">
            <h2 class="section-title">🚀 Accès Rapide</h2>
            <p class="section-subtitle">Toutes les ressources Miyukini en un seul endroit</p>
            <div class="grid grid-3">
                <a href="/docs" class="portal-card">
                    <div class="portal-icon">📖</div>
                    <h3>Documentation</h3>
                    <p>Guide complet de l'architecture, des Cores et du développement sur COG.</p>
                    <span class="portal-link">Explorer la documentation →</span>
                </a>
                <a href="/downloads" class="portal-card">
                    <div class="portal-icon">⬇️</div>
                    <h3>Téléchargements</h3>
                    <p>COGs, Cores, Services officiels et packages pour toutes les plateformes.</p>
                    <span class="portal-link">Voir les téléchargements →</span>
                </a>
                <a href="/catalog" class="portal-card featured">
                    <div class="portal-icon">📡</div>
                    <h3>Catalogue MWS</h3>
                    <p>Services et lobbys publics disponibles sur le réseau Webway.</p>
                    <span class="portal-link">Accéder au catalogue →</span>
                </a>
                <a href="/blog" class="portal-card">
                    <div class="portal-icon">📝</div>
                    <h3>Dev Blog</h3>
                    <p>Actualités, tutoriels et avancées du développement Miyukini.</p>
                    <span class="portal-link">Lire le blog →</span>
                </a>
                <a href="/announcements" class="portal-card">
                    <div class="portal-icon">📢</div>
                    <h3>Annonces</h3>
                    <p>Communications officielles, mises à jour et alertes de sécurité.</p>
                    <span class="portal-link">Voir les annonces →</span>
                </a>
                <a href="/docs/getting_started" class="portal-card">
                    <div class="portal-icon">🎯</div>
                    <h3>Démarrage Rapide</h3>
                    <p>Commencez votre aventure Miyukini en quelques minutes.</p>
                    <span class="portal-link">Commencer →</span>
                </a>
            </div>
        </section>

        <!-- Lobbys publics actifs -->
        <section class="section lobbys-section">
            <div class="section-header-flex">
                <div>
                    <h2 class="section-title">🎮 Lobbys Publics</h2>
                    <p class="section-subtitle">Rejoignez des sessions en cours sur le réseau</p>
                </div>
                <a href="/catalog" class="btn btn-secondary">Voir tout →</a>
            </div>
            <div class="lobbys-list">
                {lobbys_html}
            </div>
        </section>

        <!-- Présentation Miyukini COG -->
        <section class="section about-section">
            <h2 class="section-title">Qu'est-ce que Miyukini COG ?</h2>
            <p class="section-subtitle">Un environnement de gouvernance orchestré par des Cores</p>
            <div class="grid grid-2">
                <div class="about-text">
                    <p>
                        <strong>Miyukini COG</strong> (Cores Orchestrated Governance) est un environnement 
                        applicatif révolutionnaire conçu pour garantir votre <em>souveraineté numérique</em>.
                    </p>
                    <p>
                        Contrairement aux solutions cloud traditionnelles, Miyukini fonctionne de manière 
                        autonome sur votre matériel. Vos données restent les vôtres, sans dépendance 
                        à des services externes.
                    </p>
                    <p>
                        Le <strong>Miyukini Webway System (MWS)</strong> permet aux COGs de se découvrir 
                        et de communiquer de manière sécurisée, créant un réseau fédéré de souveraineté.
                    </p>
                    <div class="about-actions">
                        <a href="/docs/architecture" class="btn btn-secondary">En savoir plus →</a>
                    </div>
                </div>
                <div class="about-features">
                    <div class="mini-feature">
                        <span class="mini-icon">🔐</span>
                        <div>
                            <strong>Souveraineté</strong>
                            <p>Vos données sur votre matériel</p>
                        </div>
                    </div>
                    <div class="mini-feature">
                        <span class="mini-icon">🏛️</span>
                        <div>
                            <strong>8 Cores immuables</strong>
                            <p>Gouvernance transparente</p>
                        </div>
                    </div>
                    <div class="mini-feature">
                        <span class="mini-icon">🌐</span>
                        <div>
                            <strong>Réseau fédéré</strong>
                            <p>Connexion P2P sécurisée</p>
                        </div>
                    </div>
                    <div class="mini-feature">
                        <span class="mini-icon">⚡</span>
                        <div>
                            <strong>Performance Rust</strong>
                            <p>Rapide et économe</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <!-- Derniers articles -->
        <section class="section">
            <div class="section-header-flex">
                <div>
                    <h2 class="section-title">📝 Derniers Articles</h2>
                    <p class="section-subtitle">Actualités et développement</p>
                </div>
                <a href="/blog" class="btn btn-secondary">Tous les articles →</a>
            </div>
            <div class="grid grid-3">
                {recent_posts}
            </div>
        </section>

        <!-- API Section -->
        <section class="section api-section">
            <h2 class="section-title">🔌 API Origin</h2>
            <p class="section-subtitle">Accédez aux données du réseau programmatiquement</p>
            <div class="api-grid">
                <div class="api-endpoint">
                    <code>GET /api/catalog</code>
                    <span>Catalogue complet (services, lobbys)</span>
                </div>
                <div class="api-endpoint">
                    <code>GET /api/status</code>
                    <span>État du serveur Origin</span>
                </div>
                <div class="api-endpoint">
                    <code>GET /api/blog</code>
                    <span>Articles du blog</span>
                </div>
                <div class="api-endpoint">
                    <code>GET /api/announcements</code>
                    <span>Annonces officielles</span>
                </div>
                <div class="api-endpoint">
                    <code>GET /api/downloads</code>
                    <span>Liste des téléchargements</span>
                </div>
                <div class="api-endpoint">
                    <code>GET /api/docs</code>
                    <span>Documentation structurée</span>
                </div>
            </div>
        </section>

        <style>
            .portal-hero {{ background: radial-gradient(ellipse at center, rgba(6, 182, 212, 0.15) 0%, transparent 60%); }}
            .portal-badge {{
                display: inline-block;
                background: linear-gradient(135deg, rgba(139, 92, 246, 0.2), rgba(6, 182, 212, 0.2));
                border: 1px solid rgba(6, 182, 212, 0.3);
                padding: 0.5rem 1rem;
                border-radius: 2rem;
                font-size: 0.875rem;
                margin-bottom: 1rem;
            }}
            .network-stats {{
                background: var(--bg-surface);
                border-radius: 1rem;
                padding: 2rem;
                border: 1px solid var(--border);
            }}
            .stats-header {{
                display: flex;
                align-items: center;
                gap: 0.5rem;
                margin-bottom: 1.5rem;
                color: var(--text-muted);
                font-size: 0.875rem;
            }}
            .live-indicator {{
                width: 8px;
                height: 8px;
                background: var(--success);
                border-radius: 50%;
                animation: pulse 2s infinite;
            }}
            @keyframes pulse {{
                0%, 100% {{ opacity: 1; }}
                50% {{ opacity: 0.5; }}
            }}
            .stat-highlight .stat-value {{ color: var(--secondary); }}
            .online-status {{ color: var(--success) !important; font-size: 1.5rem !important; }}
            
            .portal-card {{
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 1rem;
                padding: 1.5rem;
                display: block;
                transition: all 0.3s;
            }}
            .portal-card:hover {{
                border-color: var(--primary);
                transform: translateY(-4px);
                box-shadow: 0 10px 40px rgba(139, 92, 246, 0.15);
            }}
            .portal-card.featured {{
                background: linear-gradient(135deg, rgba(139, 92, 246, 0.1), rgba(6, 182, 212, 0.1));
                border-color: rgba(6, 182, 212, 0.3);
            }}
            .portal-icon {{
                font-size: 2rem;
                margin-bottom: 1rem;
            }}
            .portal-card h3 {{ margin-bottom: 0.5rem; }}
            .portal-card p {{ color: var(--text-muted); font-size: 0.9rem; margin-bottom: 1rem; }}
            .portal-link {{ color: var(--primary); font-size: 0.875rem; font-weight: 500; }}

            .section-header-flex {{
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                margin-bottom: 2rem;
                flex-wrap: wrap;
                gap: 1rem;
            }}
            
            .lobbys-list {{
                display: flex;
                flex-direction: column;
                gap: 0.75rem;
            }}
            .lobby-card {{
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 0.75rem;
                padding: 1rem 1.5rem;
                display: flex;
                justify-content: space-between;
                align-items: center;
                transition: border-color 0.2s;
            }}
            .lobby-card:hover {{ border-color: var(--primary); }}
            .lobby-info {{ display: flex; flex-direction: column; gap: 0.25rem; }}
            .lobby-name {{ font-weight: 600; }}
            .lobby-host {{ font-size: 0.8rem; color: var(--text-muted); }}
            .lobby-meta {{ display: flex; gap: 1rem; align-items: center; font-size: 0.875rem; color: var(--text-muted); }}
            .lobby-version {{ background: var(--bg-elevated); padding: 0.2rem 0.5rem; border-radius: 0.25rem; }}
            .lobby-lock {{ color: var(--warning); }}
            .empty-state {{ color: var(--text-muted); text-align: center; padding: 2rem; }}

            .about-section .about-text p {{ margin-bottom: 1rem; color: var(--text-muted); }}
            .about-section .about-text p strong {{ color: var(--text); }}
            .about-actions {{ margin-top: 1.5rem; }}
            .about-features {{ display: flex; flex-direction: column; gap: 1rem; }}
            .mini-feature {{
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 0.75rem;
                padding: 1rem;
                display: flex;
                gap: 1rem;
                align-items: center;
            }}
            .mini-icon {{ font-size: 1.5rem; }}
            .mini-feature strong {{ display: block; margin-bottom: 0.25rem; }}
            .mini-feature p {{ margin: 0; font-size: 0.8rem; color: var(--text-muted); }}

            .api-section {{ background: var(--bg-surface); border-radius: 1rem; padding: 2rem; border: 1px solid var(--border); }}
            .api-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1rem; margin-top: 1.5rem; }}
            .api-endpoint {{
                background: var(--bg-elevated);
                border-radius: 0.5rem;
                padding: 1rem;
                display: flex;
                flex-direction: column;
                gap: 0.5rem;
            }}
            .api-endpoint code {{
                background: var(--bg);
                padding: 0.5rem 0.75rem;
                border-radius: 0.25rem;
                font-size: 0.875rem;
                color: var(--secondary);
            }}
            .api-endpoint span {{ font-size: 0.8rem; color: var(--text-muted); }}
        </style>
        "##,
        announcements = announcements_html,
        total_cogs = total_cogs,
        total_lobbys = lobbys.len(),
        total_versions = versions.len(),
        lobbys_html = lobbys_html,
        recent_posts = recent_posts_html,
    );

    layout("Miyukini COG", &content, "home")
}

/// Page de documentation.
pub async fn docs_page(content_mgr: &ContentManager) -> String {
    let sections = content_mgr.get_doc_sections().await;

    let sections_html: String = sections
        .iter()
        .map(|s| {
            let articles_html: String = s.articles
                .iter()
                .map(|a| {
                    format!(
                        r#"<li><a href="/docs/{}/{}">{}</a></li>"#,
                        html_escape(&s.id),
                        html_escape(&a.id),
                        html_escape(&a.title)
                    )
                })
                .collect();

            format!(
                r#"<div class="card">
                    <div class="feature-icon">{}</div>
                    <h3>{}</h3>
                    <p>{}</p>
                    <ul style="margin-top: 1rem; padding-left: 1.25rem;">{}</ul>
                </div>"#,
                html_escape(&s.icon),
                html_escape(&s.title),
                html_escape(&s.description),
                articles_html
            )
        })
        .collect();

    let content = format!(
        r#"
        <section class="hero" style="padding: 2rem 0;">
            <h1>Documentation</h1>
            <p>Tout ce que vous devez savoir sur Miyukini COG</p>
        </section>

        <section class="section">
            <div class="grid grid-2">
                {}
            </div>
        </section>
        "#,
        sections_html
    );

    layout("Documentation", &content, "docs")
}

/// Page de téléchargements.
pub async fn downloads_page(content_mgr: &ContentManager) -> String {
    let downloads = content_mgr.get_downloads().await;

    let downloads_by_cat = |cat: DownloadCategory| -> String {
        downloads
            .iter()
            .filter(|d| d.category == cat)
            .map(|d| {
                let platforms: String = d.platforms
                    .iter()
                    .map(|p| {
                        let name = match p {
                            Platform::Windows => "Windows",
                            Platform::Linux => "Linux",
                            Platform::MacOs => "macOS",
                            Platform::Android => "Android",
                            Platform::Ios => "iOS",
                            Platform::Universal => "Universel",
                        };
                        format!(r#"<span class="platform-badge">{}</span>"#, name)
                    })
                    .collect();

                format!(
                    r#"<div class="card download-card">
                        <div class="download-info">
                            <h3>{}</h3>
                            <p>{}</p>
                            <div class="download-meta">
                                <span>v{}</span>
                                {}
                            </div>
                        </div>
                        <a href="{}" class="btn btn-primary">⬇️ Télécharger</a>
                    </div>"#,
                    html_escape(&d.name),
                    html_escape(&d.description),
                    html_escape(&d.version),
                    platforms,
                    html_escape(&d.download_url)
                )
            })
            .collect()
    };

    let content = format!(
        r#"
        <section class="hero" style="padding: 2rem 0;">
            <h1>Téléchargements</h1>
            <p>Téléchargez Miyukini COG et ses composants</p>
        </section>

        <section class="section">
            <h2 class="section-title">🖥️ COG Complet</h2>
            <p class="section-subtitle">L'environnement Miyukini COG prêt à l'emploi</p>
            <div class="grid" style="gap: 1rem;">
                {}
            </div>
        </section>

        <section class="section">
            <h2 class="section-title">⚙️ Cores</h2>
            <p class="section-subtitle">Les 8 Cores système</p>
            <div class="grid" style="gap: 1rem;">
                {}
            </div>
        </section>

        <section class="section">
            <h2 class="section-title">📦 Services & Opérateurs</h2>
            <p class="section-subtitle">Services officiels pour votre COG</p>
            <div class="grid" style="gap: 1rem;">
                {}
            </div>
        </section>
        "#,
        downloads_by_cat(DownloadCategory::Cog),
        downloads_by_cat(DownloadCategory::Cores),
        downloads_by_cat(DownloadCategory::Service),
    );

    layout("Téléchargements", &content, "downloads")
}

/// Page des services disponibles — catalogue avec carousel horizontal (conforme maquette).
pub async fn services_page(content_mgr: &ContentManager) -> String {
    use super::content::ServiceCategory;

    let services = content_mgr.get_services().await;

    // Générer la liste des catégories pour la sidebar
    let categories_list: String = ServiceCategory::all()
        .iter()
        .map(|cat| {
            format!(
                r#"<li><a href="/services?category={}">{}</a></li>"#,
                html_escape(&format!("{:?}", cat).to_lowercase()),
                html_escape(cat.label())
            )
        })
        .collect();

    // Générer les cartes de services pour le carousel
    let services_cards: String = services
        .iter()
        .map(|s| {
            format!(
                r#"<div class="service-card">
                    <div class="service-banner">{}</div>
                    <div class="service-content">
                        <h3 class="service-title">{}</h3>
                        <div class="service-category">{}</div>
                        <p class="service-description">{}</p>
                        <div class="service-footer">
                            <div class="service-meta">
                                <span class="service-editor">{}</span>
                                <span class="service-version">COG v{}</span>
                            </div>
                            <div class="service-actions">
                                <a href="/services/{}" class="btn btn-secondary btn-sm">En savoir plus</a>
                                <a href="/downloads" class="btn btn-primary btn-sm">Pré-installé</a>
                            </div>
                        </div>
                    </div>
                </div>"#,
                if s.banner_url.is_some() {
                    format!(r#"<img src="{}" alt="Bannière">"#, html_escape(s.banner_url.as_deref().unwrap_or("")))
                } else {
                    "Petite bannière".to_string()
                },
                html_escape(&s.name),
                html_escape(s.category.label()),
                html_escape(&s.short_description),
                html_escape(&s.editor),
                html_escape(&s.cog_version),
                html_escape(&s.id),
            )
        })
        .collect();

    let content = format!(
        r##"
        <div class="services-layout">
            <!-- Sidebar filtres -->
            <aside class="services-sidebar">
                <h2>LES SERVICES</h2>
                
                <div class="filter-group">
                    <label class="toggle-label">
                        <span>Gratuit</span>
                        <input type="checkbox" id="filter-free" checked>
                        <span class="toggle-slider"></span>
                    </label>
                </div>

                <div class="filter-group">
                    <label>Filtrer par catégorie :</label>
                    <select class="filter-select" id="filter-category">
                        <option value="">Toutes les catégories</option>
                        <option value="outils">Outils</option>
                        <option value="developpement">Développement</option>
                        <option value="commerce">Commerce</option>
                        <option value="jeux">Jeux</option>
                        <option value="styledevie">Style de vie</option>
                        <option value="social">Social</option>
                        <option value="autre">Autre</option>
                    </select>
                </div>

                <div class="filter-group">
                    <label>Filtrer par Éditeur :</label>
                    <select class="filter-select" id="filter-editor">
                        <option value="">Tous les éditeurs</option>
                        <option value="miyukini">Miyukini</option>
                    </select>
                </div>

                <div class="filter-group">
                    <label>Filtrer par prix :</label>
                    <div class="price-inputs">
                        <span>De</span>
                        <input type="number" class="price-input" placeholder="min" min="0">
                        <span>à</span>
                        <input type="number" class="price-input" placeholder="max" min="0">
                    </div>
                </div>

                <div class="filter-group categories-list">
                    <h4>Liste des Catégories</h4>
                    <ul>
                        {categories_list}
                    </ul>
                </div>
            </aside>

            <!-- Carousel horizontal des services -->
            <main class="services-main">
                <div class="carousel-container">
                    <div class="carousel-track" id="servicesCarousel">
                        {services_cards}
                    </div>
                </div>

                <div class="services-loadmore">
                    <div class="loadmore-progress">
                        <div class="loadmore-bar" id="carouselProgress"></div>
                    </div>
                    <button class="btn btn-secondary btn-loadmore" onclick="scrollCarousel()">Charger Plus de services &gt;&gt;&gt;</button>
                </div>
            </main>
        </div>

        <script>
            // Carousel horizontal avec défilement infini
            const carousel = document.getElementById('servicesCarousel');
            const progressBar = document.getElementById('carouselProgress');
            let scrollAmount = 0;
            const cardWidth = 300;
            
            function scrollCarousel() {{
                const maxScroll = carousel.scrollWidth - carousel.clientWidth;
                scrollAmount += cardWidth;
                
                if (scrollAmount >= maxScroll) {{
                    // Retour au début (boucle)
                    scrollAmount = 0;
                }}
                
                carousel.scrollTo({{
                    left: scrollAmount,
                    behavior: 'smooth'
                }});
                
                updateProgress();
            }}
            
            function updateProgress() {{
                const maxScroll = carousel.scrollWidth - carousel.clientWidth;
                const progress = maxScroll > 0 ? (scrollAmount / maxScroll) * 100 : 100;
                progressBar.style.width = progress + '%';
            }}
            
            // Défilement automatique toutes les 5 secondes
            let autoScroll = setInterval(scrollCarousel, 5000);
            
            // Pause au survol
            carousel.addEventListener('mouseenter', () => clearInterval(autoScroll));
            carousel.addEventListener('mouseleave', () => {{
                autoScroll = setInterval(scrollCarousel, 5000);
            }});
            
            // Mise à jour de la barre au scroll manuel
            carousel.addEventListener('scroll', () => {{
                scrollAmount = carousel.scrollLeft;
                updateProgress();
            }});
            
            // Init
            updateProgress();
        </script>

        <style>
            .services-layout {{
                display: grid;
                grid-template-columns: 250px 1fr;
                gap: 2rem;
                min-height: 80vh;
            }}

            /* Sidebar */
            .services-sidebar {{
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 1rem;
                padding: 1.5rem;
                height: fit-content;
                position: sticky;
                top: 90px;
            }}
            .services-sidebar h2 {{
                font-size: 1.25rem;
                margin-bottom: 1.5rem;
                padding-bottom: 0.75rem;
                border-bottom: 1px solid var(--border);
            }}
            .filter-group {{
                margin-bottom: 1.25rem;
            }}
            .filter-group label {{
                display: block;
                font-size: 0.875rem;
                color: var(--text-muted);
                margin-bottom: 0.5rem;
            }}
            .filter-select {{
                width: 100%;
                padding: 0.5rem;
                background: var(--bg-elevated);
                border: 1px solid var(--border);
                border-radius: 0.5rem;
                color: var(--text);
                font-size: 0.875rem;
            }}
            .price-inputs {{
                display: flex;
                align-items: center;
                gap: 0.5rem;
            }}
            .price-input {{
                width: 60px;
                padding: 0.375rem;
                background: var(--bg-elevated);
                border: 1px solid var(--border);
                border-radius: 0.25rem;
                color: var(--text);
                font-size: 0.8rem;
            }}
            .toggle-label {{
                display: flex;
                align-items: center;
                justify-content: space-between;
                cursor: pointer;
            }}
            .toggle-label span:first-child {{
                font-weight: 500;
                color: var(--text);
            }}
            .toggle-slider {{
                width: 40px;
                height: 22px;
                background: var(--bg-elevated);
                border-radius: 11px;
                position: relative;
                transition: background 0.2s;
            }}
            .toggle-slider::before {{
                content: '';
                position: absolute;
                width: 18px;
                height: 18px;
                border-radius: 50%;
                background: var(--text-muted);
                top: 2px;
                left: 2px;
                transition: transform 0.2s, background 0.2s;
            }}
            .toggle-label input {{
                display: none;
            }}
            .toggle-label input:checked + .toggle-slider {{
                background: var(--primary);
            }}
            .toggle-label input:checked + .toggle-slider::before {{
                transform: translateX(18px);
                background: white;
            }}
            .categories-list h4 {{
                font-size: 0.875rem;
                margin-bottom: 0.75rem;
            }}
            .categories-list ul {{
                list-style: none;
                padding: 0;
            }}
            .categories-list li {{
                margin-bottom: 0.375rem;
            }}
            .categories-list a {{
                color: var(--text-muted);
                font-size: 0.875rem;
                transition: color 0.2s;
            }}
            .categories-list a:hover {{
                color: var(--primary);
            }}

            /* Carousel horizontal */
            .services-main {{
                display: flex;
                flex-direction: column;
                overflow: hidden;
            }}
            .carousel-container {{
                overflow: hidden;
                position: relative;
            }}
            .carousel-track {{
                display: flex;
                gap: 1.5rem;
                overflow-x: auto;
                scroll-behavior: smooth;
                scrollbar-width: none;
                -ms-overflow-style: none;
                padding: 0.5rem 0;
            }}
            .carousel-track::-webkit-scrollbar {{
                display: none;
            }}

            /* Carte de service */
            .service-card {{
                flex: 0 0 280px;
                min-width: 280px;
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 1rem;
                overflow: hidden;
                transition: all 0.2s;
            }}
            .service-card:hover {{
                border-color: var(--primary);
                transform: translateY(-4px);
                box-shadow: 0 10px 30px rgba(139, 92, 246, 0.15);
            }}
            .service-banner {{
                height: 100px;
                background: linear-gradient(135deg, var(--bg-elevated), var(--bg));
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--text-muted);
                font-size: 0.875rem;
                border-bottom: 1px solid var(--border);
            }}
            .service-banner img {{
                width: 100%;
                height: 100%;
                object-fit: cover;
            }}
            .service-content {{
                padding: 1rem;
            }}
            .service-title {{
                font-size: 1.1rem;
                margin-bottom: 0.25rem;
            }}
            .service-category {{
                font-size: 0.8rem;
                color: var(--primary);
                font-weight: 500;
                margin-bottom: 0.5rem;
            }}
            .service-description {{
                font-size: 0.85rem;
                color: var(--text-muted);
                margin-bottom: 1rem;
                line-height: 1.4;
            }}
            .service-footer {{
                border-top: 1px solid var(--border);
                padding-top: 1rem;
            }}
            .service-meta {{
                display: flex;
                justify-content: space-between;
                font-size: 0.75rem;
                color: var(--text-muted);
                margin-bottom: 0.75rem;
            }}
            .service-actions {{
                display: flex;
                gap: 0.5rem;
            }}
            .btn-sm {{
                padding: 0.375rem 0.75rem;
                font-size: 0.8rem;
            }}

            /* Charger plus / Progress */
            .services-loadmore {{
                margin-top: 2rem;
                display: flex;
                align-items: center;
                gap: 1rem;
            }}
            .loadmore-progress {{
                flex: 1;
                height: 8px;
                background: var(--bg-elevated);
                border-radius: 4px;
                overflow: hidden;
            }}
            .loadmore-bar {{
                height: 100%;
                background: var(--primary);
                border-radius: 4px;
                transition: width 0.3s ease;
            }}
            .btn-loadmore {{
                white-space: nowrap;
            }}

            /* Responsive */
            @media (max-width: 900px) {{
                .services-layout {{
                    grid-template-columns: 1fr;
                }}
                .services-sidebar {{
                    position: static;
                }}
                .service-card {{
                    flex: 0 0 260px;
                    min-width: 260px;
                }}
            }}
        </style>
        "##,
        categories_list = categories_list,
        services_cards = services_cards,
    );

    layout("Les services", &content, "services")
}

/// Page de fiche détaillée d'un service.
pub async fn service_detail_page(content_mgr: &ContentManager, service_id: &str) -> Option<String> {
    let service = content_mgr.get_service(service_id).await?;

    // Générer les dots du carousel (simulé avec 4 points)
    let screenshots_dots = r#"<span class="dot active"></span><span class="dot"></span><span class="dot"></span><span class="dot"></span>"#;

    // Tags
    let tags_html = format!(
        r#"<span class="service-tag">COG v{}</span>
           <span class="service-tag">{}</span>
           <span class="service-tag">{}</span>
           <span class="service-tag">{}</span>"#,
        html_escape(&service.cog_version),
        html_escape(service.category.label()),
        service.release_date.format("%d/%m/%Y"),
        html_escape(service.service_type.label())
    );

    // Convertir la description complète en HTML
    let description_html = super::server::simple_md_to_html(&service.full_description);

    let content = format!(
        r##"
        <div class="service-detail">
            <header class="service-header">
                <h1>{}</h1>
                <a href="/services" class="btn btn-secondary">Retour aux Services</a>
            </header>

            <div class="service-detail-layout">
                <!-- Section principale : screenshot + infos -->
                <div class="service-detail-left">
                    <div class="service-screenshot">
                        <div class="screenshot-placeholder">Screenshot</div>
                        <div class="screenshot-dots">
                            {}
                        </div>
                    </div>
                </div>

                <div class="service-detail-right">
                    <div class="service-detail-banner">
                        {}
                    </div>

                    <div class="service-detail-info">
                        <p class="info-description">{}</p>
                        <p class="info-editor"><strong>Éditeur :</strong> {}</p>
                        <p class="info-license"><strong>Licence :</strong> {}</p>
                    </div>

                    <div class="service-detail-tags">
                        {}
                    </div>

                    <div class="service-detail-actions">
                        {}
                        <a href="/downloads" class="btn btn-primary btn-lg">Pré-installé</a>
                    </div>
                </div>
            </div>

            <!-- Description complète -->
            <section class="service-full-description">
                <h2>Description complète</h2>
                <div class="description-content">
                    {}
                </div>
            </section>
        </div>

        <style>
            .service-detail {{
                max-width: 1200px;
                margin: 0 auto;
            }}
            .service-header {{
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 2rem;
                padding-bottom: 1rem;
                border-bottom: 1px solid var(--border);
            }}
            .service-header h1 {{
                font-size: 2rem;
                margin: 0;
            }}

            .service-detail-layout {{
                display: grid;
                grid-template-columns: 1.5fr 1fr;
                gap: 2rem;
                margin-bottom: 2rem;
            }}

            /* Screenshot */
            .service-screenshot {{
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 1rem;
                overflow: hidden;
            }}
            .screenshot-placeholder {{
                height: 350px;
                display: flex;
                align-items: center;
                justify-content: center;
                background: linear-gradient(135deg, var(--bg-elevated), var(--bg));
                color: var(--text-muted);
                font-size: 1.5rem;
            }}
            .screenshot-dots {{
                display: flex;
                justify-content: center;
                gap: 0.5rem;
                padding: 1rem;
            }}
            .dot {{
                width: 10px;
                height: 10px;
                border-radius: 50%;
                background: var(--bg-elevated);
                border: 1px solid var(--border);
                cursor: pointer;
            }}
            .dot.active {{
                background: var(--primary);
                border-color: var(--primary);
            }}

            /* Right side */
            .service-detail-right {{
                display: flex;
                flex-direction: column;
                gap: 1rem;
            }}
            .service-detail-banner {{
                background: linear-gradient(135deg, rgba(139, 92, 246, 0.1), rgba(6, 182, 212, 0.1));
                border: 1px solid var(--border);
                border-radius: 1rem;
                padding: 2rem;
                text-align: center;
                color: var(--text-muted);
                min-height: 120px;
                display: flex;
                align-items: center;
                justify-content: center;
            }}
            .service-detail-info {{
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 1rem;
                padding: 1.25rem;
            }}
            .service-detail-info p {{
                margin: 0 0 0.5rem;
                font-size: 0.9rem;
            }}
            .service-detail-info .info-description {{
                color: var(--text-muted);
                margin-bottom: 1rem;
            }}
            .service-detail-tags {{
                display: flex;
                flex-wrap: wrap;
                gap: 0.5rem;
            }}
            .service-tag {{
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 2rem;
                padding: 0.375rem 0.875rem;
                font-size: 0.8rem;
                color: var(--text-muted);
            }}
            .service-detail-actions {{
                display: flex;
                gap: 1rem;
                margin-top: auto;
            }}
            .service-detail-actions .btn {{
                flex: 1;
                justify-content: center;
                text-align: center;
            }}
            .btn-lg {{
                padding: 1rem 1.5rem;
                font-size: 1rem;
            }}

            /* Description complète */
            .service-full-description {{
                background: var(--bg-surface);
                border: 1px solid var(--border);
                border-radius: 1rem;
                padding: 2rem;
            }}
            .service-full-description h2 {{
                font-size: 1.25rem;
                margin-bottom: 1.5rem;
                padding-bottom: 0.75rem;
                border-bottom: 1px solid var(--border);
            }}
            .description-content {{
                line-height: 1.8;
            }}
            .description-content h1 {{
                font-size: 1.5rem;
                margin-bottom: 1rem;
                color: var(--primary);
            }}
            .description-content h2 {{
                font-size: 1.25rem;
                margin: 1.5rem 0 0.75rem;
            }}
            .description-content h3 {{
                font-size: 1.1rem;
                margin: 1.25rem 0 0.5rem;
            }}
            .description-content p {{
                margin-bottom: 1rem;
                color: var(--text-muted);
            }}
            .description-content ul, .description-content ol {{
                margin: 0.5rem 0 1rem 1.5rem;
                color: var(--text-muted);
            }}
            .description-content li {{
                margin-bottom: 0.375rem;
            }}

            /* Responsive */
            @media (max-width: 900px) {{
                .service-detail-layout {{
                    grid-template-columns: 1fr;
                }}
            }}
        </style>
        "##,
        html_escape(&service.name),
        screenshots_dots,
        if service.banner_url.is_some() {
            format!(r#"<img src="{}" alt="Bannière" style="max-width: 100%; border-radius: 0.5rem;">"#, html_escape(service.banner_url.as_deref().unwrap_or("")))
        } else {
            "[petite bannière]".to_string()
        },
        html_escape(&service.short_description),
        html_escape(&service.editor),
        html_escape(&service.license),
        tags_html,
        if service.website_url.is_some() {
            format!(r#"<a href="{}" class="btn btn-secondary btn-lg" target="_blank">Site Officiel</a>"#, html_escape(service.website_url.as_deref().unwrap_or("#")))
        } else {
            String::new()
        },
        description_html,
    );

    Some(layout(&service.name, &content, "services"))
}

/// Page À propos du projet.
pub fn about_page() -> String {
    let content = r#"
        <section class="hero" style="padding: 3rem 0;">
            <h1>À propos du projet Miyukini COG</h1>
            <p style="max-width: 700px; margin: 0 auto;">Présentation du projet, de son but, du comment, par qui, pourquoi — la philosophie, l'expérimentation et les ambitions.</p>
        </section>

        <section class="section">
            <div class="grid grid-2">
                <div class="card">
                    <div class="feature-icon">🎯</div>
                    <h3>Le but</h3>
                    <p>Miyukini COG vise à redonner aux utilisateurs la maîtrise de leurs données et de leur environnement numérique. Chaque COG fonctionne sur votre matériel, sans dépendre de services cloud tiers. Vos données restent les vôtres.</p>
                </div>
                <div class="card">
                    <div class="feature-icon">🏗️</div>
                    <h3>Comment</h3>
                    <p>L'architecture repose sur 8 Cores immuables qui gouvernent toutes les décisions. Les Toolkits exécutent, les Opérateurs orchestrent, les Services sont les applications que vous utilisez. Le Miyukini Webway System permet la connexion optionnelle entre COGs.</p>
                </div>
                <div class="card">
                    <div class="feature-icon">👥</div>
                    <h3>Par qui</h3>
                    <p>Miyukini est développé par une équipe convaincue que la souveraineté numérique n'est pas un luxe mais une nécessité. Le projet est ouvert aux contributeurs partageant cette vision.</p>
                </div>
                <div class="card">
                    <div class="feature-icon">❓</div>
                    <h3>Pourquoi</h3>
                    <p>Face à la centralisation des données, aux abonnements obligatoires et à la perte de contrôle, Miyukini propose une alternative : un environnement complet qui respecte vos 8 Lois d'Autonomie et fonctionne hors ligne.</p>
                </div>
            </div>
        </section>

        <section class="section">
            <h2 class="section-title">🧘 La philosophie</h2>
            <p class="section-subtitle">Souveraineté, autonomie et fédération</p>
            <div class="card" style="padding: 2rem;">
                <p><strong>Souveraineté</strong> — Vos données sur votre matériel. Personne d'autre n'y a accès.</p>
                <p style="margin-top: 1rem;"><strong>Autonomie</strong> — Le système accepte l'isolement comme état normal. Il fonctionne sans Internet.</p>
                <p style="margin-top: 1rem;"><strong>Fédération</strong> — Lorsque vous le souhaitez, connectez-vous au réseau MWS pour découvrir d'autres COGs et services, tout en gardant le contrôle.</p>
            </div>
        </section>

        <section class="section">
            <h2 class="section-title">🔬 Expérimentation</h2>
            <p class="section-subtitle">Un laboratoire pour une informatique différente</p>
            <p>Miyukini est aussi un terrain d'expérimentation : architectures distribuées, gouvernance par des Cores, protocoles de confiance, mode Lone vs Connecté. Chaque choix technique reflète une vision du numérique.</p>
        </section>

        <section class="section">
            <h2 class="section-title">🚀 Ambitions</h2>
            <p class="section-subtitle">Où va Miyukini ?</p>
            <ul style="margin-left: 1.5rem; line-height: 2;">
                <li>Un écosystème de Services variés (productivité, divertissement, gestion)</li>
                <li>Un réseau fédéré de COGs interconnectés</li>
                <li>Une alternative viable aux écosystèmes propriétaires</li>
                <li>Une référence en matière de souveraineté numérique</li>
            </ul>
        </section>
        "#;

    layout("À propos", &content, "about")
}

/// Page du blog.
pub async fn blog_page(content_mgr: &ContentManager) -> String {
    let posts = content_mgr.get_blog_posts().await;

    let posts_html: String = posts
        .iter()
        .map(|p| {
            format!(
                r#"<a href="/blog/{}" class="card blog-card">
                    <div class="date">{}</div>
                    <h3>{}</h3>
                    <p>{}</p>
                    <div class="tags">{}</div>
                </a>"#,
                html_escape(&p.id),
                p.published_at.format("%d %B %Y"),
                html_escape(&p.title),
                html_escape(&p.summary),
                p.tags.iter().map(|t| format!(r#"<span class="tag">{}</span>"#, html_escape(t))).collect::<String>()
            )
        })
        .collect();

    let content = format!(
        r#"
        <section class="hero" style="padding: 2rem 0;">
            <h1>Dev Blog</h1>
            <p>Actualités, tutoriels et développement de Miyukini</p>
        </section>

        <section class="section">
            <div class="grid grid-2">
                {}
            </div>
        </section>
        "#,
        posts_html
    );

    layout("Dev Blog", &content, "blog")
}

/// Page d'un article de blog.
pub async fn blog_post_page(content_mgr: &ContentManager, post_id: &str) -> Option<String> {
    let post = content_mgr.get_blog_post(post_id).await?;

    // Simple Markdown to HTML (basique)
    let content_html = simple_markdown_to_html(&post.content);

    let content = format!(
        r#"
        <article style="max-width: 800px; margin: 0 auto;">
            <div style="margin-bottom: 2rem;">
                <a href="/blog" style="color: var(--text-muted);">← Retour au blog</a>
            </div>
            <header style="margin-bottom: 2rem;">
                <div style="color: var(--text-muted); margin-bottom: 0.5rem;">
                    {} • {}
                </div>
                <h1 style="font-size: 2.5rem; margin-bottom: 1rem;">{}</h1>
                <div class="tags">{}</div>
            </header>
            <div class="card" style="padding: 2rem;">
                {}
            </div>
        </article>
        "#,
        post.published_at.format("%d %B %Y"),
        html_escape(&post.author),
        html_escape(&post.title),
        post.tags.iter().map(|t| format!(r#"<span class="tag">{}</span>"#, html_escape(t))).collect::<String>(),
        content_html
    );

    Some(layout(&post.title, &content, "blog"))
}

/// Page des annonces.
pub async fn announcements_page(content_mgr: &ContentManager) -> String {
    let announcements = content_mgr.get_announcements().await;

    let announcements_html: String = announcements
        .iter()
        .map(|a| {
            let (type_class, type_icon, type_label) = match a.announcement_type {
                AnnouncementType::Release => ("release", "🚀", "Nouvelle version"),
                AnnouncementType::Security => ("security", "🔒", "Sécurité"),
                AnnouncementType::Maintenance => ("maintenance", "🔧", "Maintenance"),
                AnnouncementType::Info => ("info", "ℹ️", "Information"),
            };

            format!(
                r#"<div class="card">
                    <div class="announcement {}" style="margin-bottom: 1rem;">
                        <span>{}</span>
                        <strong>{}</strong>
                    </div>
                    <h3>{}</h3>
                    <p style="margin-top: 0.5rem;">{}</p>
                    <div style="margin-top: 1rem; color: var(--text-muted); font-size: 0.875rem;">
                        Publié le {}
                    </div>
                </div>"#,
                type_class,
                type_icon,
                type_label,
                html_escape(&a.title),
                html_escape(&a.content),
                a.published_at.format("%d %B %Y à %H:%M")
            )
        })
        .collect();

    let content = format!(
        r#"
        <section class="hero" style="padding: 2rem 0;">
            <h1>Annonces</h1>
            <p>Communications officielles de l'équipe Miyukini</p>
        </section>

        <section class="section">
            <div class="grid grid-2">
                {}
            </div>
        </section>
        "#,
        if announcements_html.is_empty() {
            r#"<div class="card"><p style="text-align: center; color: var(--text-muted);">Aucune annonce pour le moment</p></div>"#.to_string()
        } else {
            announcements_html
        }
    );

    layout("Annonces", &content, "announcements")
}

/// Conversion Markdown basique en HTML.
fn simple_markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;

    for line in markdown.lines() {
        if line.starts_with("```") {
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
            html.push_str(&html_escape(line));
            html.push('\n');
            continue;
        }

        if line.is_empty() {
            html.push_str("<br>\n");
        } else if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", html_escape(&line[2..])));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", html_escape(&line[3..])));
        } else if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", html_escape(&line[4..])));
        } else if line.starts_with("- ") {
            html.push_str(&format!("<li>{}</li>\n", html_escape(&line[2..])));
        } else if line.starts_with("> ") {
            html.push_str(&format!("<blockquote>{}</blockquote>\n", html_escape(&line[2..])));
        } else {
            // Inline formatting
            let formatted = line
                .replace("**", "<strong>")
                .replace("__", "<strong>")
                .replace("*", "<em>")
                .replace("`", "<code>");
            html.push_str(&format!("<p>{}</p>\n", formatted));
        }
    }

    if in_code_block {
        html.push_str("</code></pre>\n");
    }

    html
}

/// Échappe les caractères HTML.
pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// ═══════════════════════════════════════════════════════════════════════════
// Pages vitrine JayXpose (layout commun, données par KM, jumelles des natives)
// Distinctes de la Home du portail. Voir docs/services/JayXpose/JayXpose - Pages Web Publiques Vitrine.md
// ═══════════════════════════════════════════════════════════════════════════

/// Layout commun des pages vitrine (tous COG). En-tête avec logo/nom, nav, contenu, pied.
fn vitrine_layout(
    title: &str,
    content: &str,
    slug: &str,
    company_name: &str,
    active: &str,
    show_catalogue: bool,
    show_presentation: bool,
    show_contact: bool,
) -> String {
    let base = format!("/vitrine/{}", html_escape(slug));
    let nav_home = if active == "home" { "active" } else { "" };
    let nav_catalogue = if active == "catalogue" { "active" } else { "" };
    let nav_presentation = if active == "presentation" { "active" } else { "" };
    let nav_contact = if active == "contact" { "active" } else { "" };
    let mut nav_extra = Vec::new();
    if show_catalogue {
        nav_extra.push(format!(r#"<a href="{}/catalogue" class="{}">Catalogue</a>"#, base, nav_catalogue));
    }
    if show_presentation {
        nav_extra.push(format!(r#"<a href="{}/presentation" class="{}">Présentation</a>"#, base, nav_presentation));
    }
    if show_contact {
        nav_extra.push(format!(r#"<a href="{}/contact" class="{}">Contact</a>"#, base, nav_contact));
    }
    let nav_extra_s = nav_extra.join(" ");
    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Vitrine</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        .v-root {{ --v-primary: #2563eb; --v-bg: #0a0a0f; --v-surface: #12121a; --v-text: #f0f0f5; --v-muted: #9ca3af; --v-border: rgba(37,99,235,0.2); }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ font-family: 'Inter', sans-serif; background: var(--v-bg); color: var(--v-text); line-height: 1.6; min-height: 100vh; }}
        .v-header {{ background: var(--v-surface); border-bottom: 1px solid var(--v-border); padding: 0.75rem 2rem; display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 1rem; }}
        .v-logo {{ font-weight: 700; font-size: 1.25rem; color: var(--v-primary); }}
        .v-nav {{ display: flex; gap: 0.5rem; }}
        .v-nav a {{ padding: 0.5rem 1rem; border-radius: 0.5rem; color: var(--v-muted); text-decoration: none; }}
        .v-nav a:hover {{ color: var(--v-text); background: rgba(37,99,235,0.1); }}
        .v-nav a.active {{ color: var(--v-primary); }}
        main {{ max-width: 1000px; margin: 0 auto; padding: 2rem; }}
        .v-hero {{ padding: 2rem 0; }}
        .v-grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem; margin-top: 1rem; }}
        .v-card {{ background: var(--v-surface); border: 1px solid var(--v-border); border-radius: 0.5rem; padding: 1rem; }}
        .v-price {{ font-weight: 600; color: var(--v-primary); margin: 0.25rem 0; }}
        .v-avail {{ display: inline-block; font-size: 0.8rem; color: var(--v-muted); margin-top: 0.25rem; }}
        .v-footer {{ margin-top: 3rem; padding: 1.5rem 2rem; border-top: 1px solid var(--v-border); color: var(--v-muted); font-size: 0.875rem; }}
        .v-footer a {{ color: var(--v-primary); }}
    </style>
</head>
<body class="v-root">
    <header class="v-header">
        <a href="{}" class="v-logo">{}</a>
        <nav class="v-nav">
            <a href="{}" class="{}">Accueil</a>
            {}
        </nav>
    </header>
    <main>{}</main>
    <footer class="v-footer">
        <a href="/">← Retour au portail Miyukini</a>
    </footer>
</body>
</html>"#,
        html_escape(title),
        base,
        html_escape(company_name),
        base,
        nav_home,
        nav_extra_s,
        content
    )
}

/// Page index /vitrine — liste des vitrines ou message.
pub fn vitrine_index_page(db: &JayXposeDb) -> Option<String> {
    let exposants = db.exposants_list_annuaire().ok()?;
    let with_slug: Vec<_> = exposants
        .into_iter()
        .filter(|e| e.vitrine_slug.as_deref().map_or(false, |s: &str| !s.is_empty()))
        .filter(|e| e.vitrine_status.as_deref() == Some("publiee"))
        .collect();
    let list: String = if with_slug.is_empty() {
        r#"<p>Aucune vitrine publiée pour le moment.</p>"#.to_string()
    } else {
        with_slug
            .iter()
            .map(|e| {
                let name = e.company_name.as_deref().unwrap_or("Sans nom");
                let slug = e.vitrine_slug.as_deref().unwrap_or("");
                format!(r#"<li><a href="/vitrine/{}">{}</a></li>"#, html_escape(slug), html_escape(name))
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let body = format!(
        r#"<!DOCTYPE html><html lang="fr"><head><meta charset="UTF-8"><title>Vitrines — Miyukini</title></head><body>
        <h1>Vitrines exposants</h1><ul>{}</ul><p><a href="/">Retour à l'accueil</a></p></body></html>"#,
        list
    );
    Some(body)
}

/// Page accueil vitrine /vitrine/{slug} (PUB-E03).
pub fn vitrine_home_page(db: &JayXposeDb, slug: &str) -> Option<String> {
    let exposant = db.exposant_by_vitrine_slug(slug).ok()??;
    let company_name = exposant.company_name.as_deref().unwrap_or("Vitrine");
    let slogan = exposant.slogan.as_deref().unwrap_or("");
    let desc = exposant.description_short.as_deref().unwrap_or("");
    let banner = exposant.banner_url.as_deref().unwrap_or("");
    let produits = db.produits_by_exposant(exposant.id.as_deref().unwrap_or("")).ok().unwrap_or_default();
    let vedettes: Vec<_> = produits.into_iter().filter(|p| p.is_featured == Some(true)).take(6).collect();
    let vedettes_html: String = vedettes
        .iter()
        .map(|p| {
            let name = p.name.as_deref().unwrap_or("Produit");
            let price = p.price.map(|x| format!("{:.2} €", x)).unwrap_or_else(|| "Sur demande".to_string());
            format!(
                r#"<div class="v-card"><h4>{}</h4><p>{}</p></div>"#,
                html_escape(name),
                html_escape(&price)
            )
        })
        .collect();
    let hero_style = if banner.is_empty() {
        String::new()
    } else {
        format!(r#" style="background-image: url('{}'); background-size: cover; min-height: 200px;""#, html_escape(banner))
    };
    let content = format!(
        r#"<section class="v-hero"{}><h1>{}</h1><p>{}</p></section><p>{}</p><h2>Produits vedettes</h2><div class="v-grid">{}</div>"#,
        hero_style,
        html_escape(company_name),
        html_escape(slogan),
        html_escape(desc),
        vedettes_html
    );
    Some(vitrine_layout(
        company_name,
        &content,
        slug,
        company_name,
        "home",
        true,
        true,
        true,
    ))
}

/// Page catalogue /vitrine/{slug}/catalogue (PUB-E04).
pub fn vitrine_catalogue_page(db: &JayXposeDb, slug: &str, _category_id: Option<&str>) -> Option<String> {
    let exposant = db.exposant_by_vitrine_slug(slug).ok()??;
    let company_name = exposant.company_name.as_deref().unwrap_or("Vitrine");
    let exposant_id = exposant.id.as_deref().unwrap_or("");
    let produits = db.produits_by_exposant(exposant_id).ok().unwrap_or_default();
    let base = format!("/vitrine/{}", html_escape(slug));
    let availability_label = |a: Option<&String>| -> String {
        let s: &str = a.map(|x| x.as_str()).unwrap_or("disponible");
        match s {
            "rupture" => "Rupture",
            "sur_commande" => "Sur commande",
            _ => "Disponible",
        }
        .to_string()
    };
    let cards: String = produits
        .iter()
        .map(|p| {
            let id = p.id.as_deref().unwrap_or("");
            let name = p.name.as_deref().unwrap_or("Produit");
            let price = p.price.map(|x| format!("{:.2} €", x)).unwrap_or_else(|| "Sur demande".to_string());
            let avail = availability_label(p.availability.as_ref());
            format!(
                r#"<a href="{}/catalogue/{}" class="v-card"><h4>{}</h4><p class="v-price">{}</p><span class="v-avail">{}</span></a>"#,
                base,
                html_escape(id),
                html_escape(name),
                html_escape(&price),
                html_escape(&avail)
            )
        })
        .collect();
    let content = format!(r#"<h1>Catalogue</h1><div class="v-grid">{}</div>"#, cards);
    Some(vitrine_layout("Catalogue", &content, slug, company_name, "catalogue", true, true, true))
}

/// Page fiche produit /vitrine/{slug}/catalogue/{id}.
pub fn vitrine_produit_page(db: &JayXposeDb, slug: &str, produit_id: &str) -> Option<String> {
    let exposant = db.exposant_by_vitrine_slug(slug).ok()??;
    let company_name = exposant.company_name.as_deref().unwrap_or("Vitrine");
    let produit = db.produit_by_id(produit_id).ok()??;
    if produit.exposant_id.as_deref() != exposant.id.as_deref() {
        return None;
    }
    let name = produit.name.as_deref().unwrap_or("Produit");
    let desc = produit.description.as_deref().unwrap_or("");
    let price = produit.price.map(|x| format!("{:.2} €", x)).unwrap_or_else(|| "Sur demande".to_string());
    let availability = match produit.availability.as_deref().unwrap_or("disponible") {
        "rupture" => "Rupture",
        "sur_commande" => "Sur commande",
        _ => "Disponible",
    };
    let base = format!("/vitrine/{}", html_escape(slug));
    let content = format!(
        r#"<p><a href="{}/catalogue">← Catalogue</a></p><h1>{}</h1><p class="v-price">{}</p><p class="v-avail">Disponibilité : {}</p><div>{}</div>"#,
        base,
        html_escape(name),
        html_escape(&price),
        html_escape(availability),
        html_escape(desc)
    );
    Some(vitrine_layout(name, &content, slug, company_name, "catalogue", true, true, true))
}

/// Page présentation /vitrine/{slug}/presentation (PUB-E05).
pub fn vitrine_presentation_page(db: &JayXposeDb, slug: &str) -> Option<String> {
    let exposant = db.exposant_by_vitrine_slug(slug).ok()??;
    let company_name = exposant.company_name.as_deref().unwrap_or("Vitrine");
    let pages = db.vitrine_pages_by_exposant(exposant.id.as_deref().unwrap_or("")).ok().unwrap_or_default();
    let presentation = pages.into_iter().find(|p| p.page_type.as_deref() == Some("presentation") && p.is_visible == Some(true));
    let content_html = presentation
        .and_then(|p| p.content)
        .map(|c| simple_md_to_html(&c))
        .unwrap_or_else(|| r#"<p>Aucun contenu pour l'instant.</p>"#.to_string());
    let content = format!(r#"<h1>Présentation</h1><div class="v-content">{}</div>"#, content_html);
    Some(vitrine_layout("Présentation", &content, slug, company_name, "presentation", true, true, true))
}

/// Page contact /vitrine/{slug}/contact (PUB-E06).
pub fn vitrine_contact_page(db: &JayXposeDb, slug: &str) -> Option<String> {
    let exposant = db.exposant_by_vitrine_slug(slug).ok()??;
    let company_name = exposant.company_name.as_deref().unwrap_or("Vitrine");
    let email = exposant.contact_email.as_deref().unwrap_or("");
    let phone = exposant.contact_phone.as_deref().unwrap_or("");
    let adresse = exposant.adresse_siege.as_deref().unwrap_or("");
    let site_web = exposant.site_web.as_deref().unwrap_or("");
    let site_display = if site_web.is_empty() { "—".to_string() } else { html_escape(site_web) };
    let coord = format!(
        r#"<p><strong>Contact</strong><br>Email: {}<br>Tél: {}<br>Adresse: {}<br>Site: {}"#,
        html_escape(email),
        html_escape(phone),
        html_escape(adresse),
        site_display
    );
    let content = format!(r#"<h1>Contact</h1><div>{}</div><p>Formulaire de contact (à venir).</p>"#, coord);
    Some(vitrine_layout("Contact", &content, slug, company_name, "contact", true, true, true))
}
