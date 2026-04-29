//! Générateur de pages HTML pour le site web Origin.

use super::content::{AnnouncementType, ContentManager, DownloadCategory};
use crate::tracker::pool::PoolManager;

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
            height: 56px;
        }}
        .logo {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            font-size: 1.35rem;
            font-weight: 700;
            background: var(--gradient-1);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .logo-icon {{
            width: 34px;
            height: 34px;
            background: var(--gradient-1);
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.1rem;
            -webkit-text-fill-color: white;
        }}
        nav {{ display: flex; gap: 0.25rem; align-items: center; }}
        nav a {{
            padding: 0.4rem 0.75rem;
            border-radius: 0.5rem;
            color: var(--text-muted);
            font-size: 0.9rem;
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
                <a href="/" class="{home_active}">Accueil</a>
                <a href="/mws" class="{catalog_active}">MWS</a>
                <a href="/services" class="{services_active}">Services</a>
                <a href="/downloads" class="{downloads_active}">Télécharger</a>
                <a href="/docs" class="{docs_active}">Docs</a>
                <a href="/mip" class="{mip_active}">MIP</a>
                <a href="/about" class="{about_active}">À propos</a>
                <a href="/?onboarding=1" class="nav-miou-replay" title="Redécouvrir avec Miou" style="font-size:1.1rem;-webkit-text-fill-color:initial;">🌸</a>
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
                    <li><a href="/mws">MWS</a></li>
                    <li><a href="/about">À propos</a></li>
                    <li><a href="/blog">Dev Blog</a></li>
                </ul>
            </div>
            <div>
                <h4>Communauté</h4>
                <ul>
                    <li><a href="/community/discord">Discord</a></li>
                    <li><a href="/community/github">GitHub</a></li>
                    <li><a href="https://forum.miyukini.com" target="_blank">Forum</a></li>
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
        downloads_active = if active_nav == "downloads" {
            "active"
        } else {
            ""
        },
        catalog_active = if active_nav == "catalog" || active_nav == "mws" {
            "active"
        } else {
            ""
        },
        services_active = if active_nav == "services" {
            "active"
        } else {
            ""
        },
        about_active = if active_nav == "about" { "active" } else { "" },
        mip_active = if active_nav == "mip" { "active" } else { "" },
    )
}

/// Page d'accueil — Onboarding VN interactif avec Miou.
pub async fn home_page(content_mgr: &ContentManager, pool_mgr: &PoolManager) -> String {
    let blog_posts = content_mgr.get_blog_posts().await;
    let total_cogs = pool_mgr.total_cog_count().await;
    let versions = pool_mgr.list_versions().await;
    let lobbys = pool_mgr.list_all_public_lobbys().await;
    let latest_version = versions.first().map(|v| v.as_str()).unwrap_or("0.1.0");

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
                </a>"#,
                html_escape(&p.id),
                p.published_at.format("%d %B %Y"),
                html_escape(&p.title),
                html_escape(&p.summary),
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
                    if lobby.password_required {
                        r#"<span class="lobby-lock">🔒</span>"#
                    } else {
                        ""
                    }
                )
            })
            .collect()
    };

    let content = format!(
        r##"
<!-- ══════════════════════════════════════ -->
<!--  VN OVERLAY : ONBOARDING FULLSCREEN  -->
<!-- ══════════════════════════════════════ -->
<div id="vn-overlay">

  <!-- === SCREEN 1 : TITLE === -->
  <div class="vn-screen active" id="vn-screen-1">
    <canvas id="starfield"></canvas>
    <div class="vn-ornament top-left"></div>
    <div class="vn-ornament top-right"></div>
    <div class="vn-ornament bottom-left"></div>
    <div class="vn-ornament bottom-right"></div>
    <div class="vn-title-center">
      <div class="vn-title-logo">Miyukini</div>
      <div class="vn-title-sub">COG</div>
      <div class="vn-title-tagline">Cores Orchestrated Governance</div>
      <div class="vn-click-prompt" id="vn-click-prompt">Cliquez pour d&eacute;couvrir</div>
    </div>
    <div class="vn-status-left">
      <div class="vn-status-item"><span class="vn-dot green"></span> Origin en ligne</div>
      <div class="vn-status-item">{total_cogs} COG connect&eacute;s</div>
      <div class="vn-status-item">v{latest_version}</div>
    </div>
    <div class="vn-status-right">
      <button class="vn-skip-btn" onclick="vnSkip()">Acc&egrave;s Portail Origin &rarr;</button>
    </div>
  </div>

  <!-- === SCREEN 2 : MIOU + PROFILE SELECT === -->
  <div class="vn-screen" id="vn-screen-2">
    <canvas id="starfield2"></canvas>
    <div class="vn-s2-center">
      <div class="miou-large" id="miou-large">
        <span class="miou-char">&#x1f338;</span>
        <div class="miou-glow-ring"></div>
      </div>
      <div class="miou-dialogue-box" id="miou-s2-dialogue">
        <span id="miou-s2-text"></span><span class="miou-caret">|</span>
      </div>
      <div class="vn-profiles" id="vn-profiles">
        <button class="vn-profile-btn" data-profile="curious" onclick="vnSelectProfile('curious')">
          <span class="vn-prof-icon">&#x1f50d;</span>
          <span class="vn-prof-label">Curieux</span>
          <span class="vn-prof-desc">Je d&eacute;couvre Miyukini</span>
        </button>
        <button class="vn-profile-btn" data-profile="user" onclick="vnSelectProfile('user')">
          <span class="vn-prof-icon">&#x1f3e0;</span>
          <span class="vn-prof-label">Utilisateur</span>
          <span class="vn-prof-desc">Je veux mon COG</span>
        </button>
        <button class="vn-profile-btn" data-profile="dev" onclick="vnSelectProfile('dev')">
          <span class="vn-prof-icon">&#x2699;&#xfe0f;</span>
          <span class="vn-prof-label">D&eacute;veloppeur</span>
          <span class="vn-prof-desc">Je cr&eacute;e sur COG</span>
        </button>
        <button class="vn-profile-btn" data-profile="pro" onclick="vnSelectProfile('pro')">
          <span class="vn-prof-icon">&#x1f4bc;</span>
          <span class="vn-prof-label">Professionnel</span>
          <span class="vn-prof-desc">Solution souveraine</span>
        </button>
      </div>
    </div>
  </div>

  <!-- === SCREEN 3 : PROFILE DETAIL (3 columns) === -->
  <div class="vn-screen" id="vn-screen-3">
    <canvas id="starfield3"></canvas>
    <div class="vn-s3-layout">
      <!-- Left: accordion menu -->
      <div class="vn-s3-menu" id="vn-s3-menu"></div>
      <!-- Center: Miou -->
      <div class="vn-s3-miou">
        <div class="miou-medium" id="miou-medium">
          <span class="miou-char">&#x1f338;</span>
          <div class="miou-glow-ring"></div>
        </div>
      </div>
      <!-- Right: dialogue box -->
      <div class="vn-s3-dialogue">
        <div class="vn-dialogue-header" id="vn-topic-title"></div>
        <div class="vn-dialogue-body" id="vn-topic-body"></div>
        <div class="vn-dialogue-miou">
          <span class="miou-mini">&#x1f338;</span>
          <span id="vn-miou-comment"></span>
        </div>
        <div class="vn-dialogue-choices" id="vn-choices"></div>
        <button class="vn-next-topic" id="vn-next-btn" onclick="vnNextTopic()">Suivant &rarr;</button>
        <button class="vn-finish-btn" id="vn-finish-btn" onclick="vnFinish()" style="display:none">
          Explorer le portail &#x1f338;
        </button>
      </div>
    </div>
  </div>
</div>

<!-- ══════════════════════════════════════ -->
<!--  HOME CONTENT (hidden during VN)      -->
<!-- ══════════════════════════════════════ -->
<div id="home-content" style="display:none">
  <div class="home-nav-grid">
    <a href="/downloads" class="home-nav-card">
      <span class="home-nav-icon">&#x1f4e5;</span>
      <span class="home-nav-title">T&eacute;l&eacute;charger</span>
      <span class="home-nav-desc">Obtenir Miyukini Central</span>
    </a>
    <a href="/services" class="home-nav-card">
      <span class="home-nav-icon">&#x2728;</span>
      <span class="home-nav-title">Services</span>
      <span class="home-nav-desc">D&eacute;couvrir les services</span>
    </a>
    <a href="/docs" class="home-nav-card">
      <span class="home-nav-icon">&#x1f4da;</span>
      <span class="home-nav-title">Documentation</span>
      <span class="home-nav-desc">Guides et r&eacute;f&eacute;rences</span>
    </a>
    <a href="/about" class="home-nav-card">
      <span class="home-nav-icon">&#x1f30d;</span>
      <span class="home-nav-title">&Agrave; propos</span>
      <span class="home-nav-desc">Vision et philosophie</span>
    </a>
  </div>

  <div class="home-stats-bar">
    <div class="home-stat"><strong>{total_cogs}</strong> COG connect&eacute;s</div>
    <div class="home-stat"><strong>{total_lobbys}</strong> lobbys actifs</div>
    <div class="home-stat"><strong>{total_versions}</strong> versions track&eacute;es</div>
  </div>

  <div class="home-sections">
    <section class="home-section" id="home-lobbys">
      <h2>Lobbys publics</h2>
      <div class="lobbys-list">{lobbys_html}</div>
    </section>
    <section class="home-section" id="home-blog">
      <h2>Derniers articles</h2>
      <div class="blog-list">{recent_posts}</div>
    </section>
  </div>

  <div class="home-miou-float" id="home-miou-float">
    <span class="miou-char">&#x1f338;</span>
    <div class="home-miou-tip" id="home-miou-tip"></div>
  </div>
</div>

<!-- ══════════════════════════════════════ -->
<!--  STYLES VN                            -->
<!-- ══════════════════════════════════════ -->
<style>
/* === VN OVERLAY === */
#vn-overlay {{
  position: fixed; inset: 0; z-index: 9999;
  background: #0a0a14;
}}
#vn-overlay.hidden {{ display: none; }}
.vn-screen {{
  position: absolute; inset: 0;
  display: none; flex-direction: column;
  align-items: center; justify-content: center;
  overflow: hidden;
}}
.vn-screen.active {{ display: flex; }}
.vn-screen canvas {{
  position: absolute; inset: 0; z-index: 0;
}}

/* Ornaments (corner decorations) */
.vn-ornament {{
  position: absolute; width: 80px; height: 80px; z-index: 1;
  border-color: rgba(255,183,197,0.3); border-style: solid; border-width: 0;
}}
.vn-ornament.top-left {{ top:20px; left:20px; border-top-width:2px; border-left-width:2px; border-top-left-radius: 12px; }}
.vn-ornament.top-right {{ top:20px; right:20px; border-top-width:2px; border-right-width:2px; border-top-right-radius: 12px; }}
.vn-ornament.bottom-left {{ bottom:20px; left:20px; border-bottom-width:2px; border-left-width:2px; border-bottom-left-radius: 12px; }}
.vn-ornament.bottom-right {{ bottom:20px; right:20px; border-bottom-width:2px; border-right-width:2px; border-bottom-right-radius: 12px; }}

/* Title screen */
.vn-title-center {{
  z-index: 2; text-align: center;
}}
.vn-title-logo {{
  font-size: 4.5rem; font-weight: 800; letter-spacing: 0.15em;
  background: linear-gradient(135deg, #ffb7c5, #fff, #ffb7c5);
  -webkit-background-clip: text; -webkit-text-fill-color: transparent;
  text-shadow: 0 0 40px rgba(255,183,197,0.4);
  animation: titlePulse 3s ease-in-out infinite;
}}
.vn-title-sub {{
  font-size: 2rem; font-weight: 300; letter-spacing: 0.5em;
  color: rgba(255,255,255,0.7); margin-top: -8px;
}}
.vn-title-tagline {{
  font-size: 0.9rem; color: rgba(255,183,197,0.6); margin-top: 16px;
  letter-spacing: 0.2em; text-transform: uppercase;
}}
.vn-click-prompt {{
  margin-top: 48px; font-size: 1rem; color: rgba(255,255,255,0.5);
  animation: promptFade 2s ease-in-out infinite;
  cursor: pointer;
}}
@keyframes titlePulse {{
  0%,100% {{ filter: brightness(1); }}
  50% {{ filter: brightness(1.2); }}
}}
@keyframes promptFade {{
  0%,100% {{ opacity: 0.4; }}
  50% {{ opacity: 1; }}
}}

/* Status bars */
.vn-status-left {{
  position: absolute; bottom: 24px; left: 28px; z-index: 2;
  display: flex; flex-direction: column; gap: 6px;
}}
.vn-status-item {{
  font-size: 0.8rem; color: rgba(255,255,255,0.5);
}}
.vn-dot {{
  display: inline-block; width: 8px; height: 8px; border-radius: 50%;
  margin-right: 6px; vertical-align: middle;
}}
.vn-dot.green {{ background: #4ade80; box-shadow: 0 0 8px #4ade80; }}
.vn-status-right {{
  position: absolute; bottom: 24px; right: 28px; z-index: 2;
}}
.vn-skip-btn {{
  background: rgba(255,183,197,0.15); border: 1px solid rgba(255,183,197,0.3);
  color: #ffb7c5; padding: 10px 24px; border-radius: 8px; cursor: pointer;
  font-size: 0.9rem; transition: all 0.3s;
}}
.vn-skip-btn:hover {{ background: rgba(255,183,197,0.25); }}

/* Screen 2 */
.vn-s2-center {{
  z-index: 2; display: flex; flex-direction: column;
  align-items: center; gap: 24px;
}}
.miou-large {{
  position: relative; font-size: 5rem;
  animation: miouFloat 3s ease-in-out infinite;
}}
.miou-medium {{
  position: relative; font-size: 3.5rem;
  animation: miouFloat 3s ease-in-out infinite;
}}
.miou-char {{ position: relative; z-index: 1; }}
.miou-glow-ring {{
  position: absolute; inset: -20px; border-radius: 50%;
  background: radial-gradient(circle, rgba(255,183,197,0.3), transparent 70%);
  animation: glowPulse 2s ease-in-out infinite;
}}
@keyframes miouFloat {{
  0%,100% {{ transform: translateY(0); }}
  50% {{ transform: translateY(-10px); }}
}}
@keyframes glowPulse {{
  0%,100% {{ opacity: 0.5; transform: scale(1); }}
  50% {{ opacity: 1; transform: scale(1.15); }}
}}
.miou-large.talking {{ animation: miouTalk 0.15s ease-in-out infinite; }}
@keyframes miouTalk {{
  0%,100% {{ transform: translateY(0) scale(1); }}
  50% {{ transform: translateY(-3px) scale(1.03); }}
}}

/* Dialogue box */
.miou-dialogue-box {{
  background: rgba(20,20,40,0.85); border: 1px solid rgba(255,183,197,0.3);
  border-radius: 16px; padding: 20px 28px; max-width: 520px; min-height: 60px;
  color: #fff; font-size: 1rem; line-height: 1.6;
  box-shadow: 0 0 30px rgba(255,183,197,0.1);
}}
.miou-caret {{
  color: #ffb7c5; animation: caretBlink 0.8s step-end infinite;
}}
@keyframes caretBlink {{
  0%,100% {{ opacity: 1; }}
  50% {{ opacity: 0; }}
}}

/* Profile buttons */
.vn-profiles {{
  display: flex; gap: 16px; margin-top: 12px;
  opacity: 0; transform: translateY(20px);
  transition: all 0.6s ease;
}}
.vn-profiles.visible {{
  opacity: 1; transform: translateY(0);
}}
.vn-profile-btn {{
  background: rgba(255,255,255,0.05); border: 1px solid rgba(255,183,197,0.2);
  border-radius: 12px; padding: 16px 20px; cursor: pointer;
  display: flex; flex-direction: column; align-items: center; gap: 6px;
  transition: all 0.3s; color: #fff; min-width: 130px;
}}
.vn-profile-btn:hover {{
  background: rgba(255,183,197,0.15); border-color: #ffb7c5;
  transform: translateY(-4px); box-shadow: 0 8px 24px rgba(255,183,197,0.2);
}}
.vn-prof-icon {{ font-size: 1.8rem; }}
.vn-prof-label {{ font-size: 0.95rem; font-weight: 600; }}
.vn-prof-desc {{ font-size: 0.75rem; color: rgba(255,255,255,0.5); }}

/* Screen 3 layout */
.vn-s3-layout {{
  position: relative; z-index: 2;
  display: grid; grid-template-columns: 260px 1fr 400px;
  width: 100%; height: 100%; padding: 32px;
  gap: 24px; align-items: start;
}}
.vn-s3-menu {{
  display: flex; flex-direction: column; gap: 4px;
  margin-top: 40px;
}}
.vn-menu-item {{
  background: rgba(255,255,255,0.04); border: 1px solid rgba(255,183,197,0.1);
  border-radius: 8px; padding: 12px 16px; cursor: pointer;
  color: rgba(255,255,255,0.6); font-size: 0.85rem;
  transition: all 0.3s; display: flex; align-items: center; gap: 8px;
}}
.vn-menu-item:hover {{ background: rgba(255,183,197,0.1); color: #fff; }}
.vn-menu-item.active {{
  background: rgba(255,183,197,0.15); border-color: #ffb7c5;
  color: #ffb7c5; font-weight: 600;
}}
.vn-menu-icon {{ font-size: 1.1rem; }}

/* S3 center miou */
.vn-s3-miou {{
  display: flex; align-items: center; justify-content: center;
  height: 100%;
}}

/* S3 dialogue panel */
.vn-s3-dialogue {{
  background: rgba(20,20,40,0.9); border: 1px solid rgba(255,183,197,0.25);
  border-radius: 16px; padding: 28px; margin-top: 32px;
  display: flex; flex-direction: column; gap: 16px;
  max-height: calc(100vh - 120px); overflow-y: auto;
}}
.vn-dialogue-header {{
  font-size: 1.15rem; font-weight: 700; color: #ffb7c5;
  padding-bottom: 12px; border-bottom: 1px solid rgba(255,183,197,0.15);
}}
.vn-dialogue-body {{
  font-size: 0.9rem; color: rgba(255,255,255,0.85); line-height: 1.7;
}}
.vn-dialogue-miou {{
  display: flex; align-items: flex-start; gap: 10px;
  background: rgba(255,183,197,0.08); border-radius: 10px; padding: 12px 16px;
  font-size: 0.85rem; color: rgba(255,255,255,0.7); font-style: italic;
}}
.miou-mini {{ font-size: 1.2rem; flex-shrink: 0; }}

/* Choices */
.vn-dialogue-choices {{
  display: flex; flex-direction: column; gap: 8px;
}}
.vn-choice-btn {{
  background: rgba(255,255,255,0.05); border: 1px solid rgba(255,183,197,0.2);
  border-radius: 8px; padding: 10px 16px; cursor: pointer;
  color: #fff; font-size: 0.85rem; text-align: left; transition: all 0.3s;
}}
.vn-choice-btn:hover {{
  background: rgba(255,183,197,0.15); border-color: #ffb7c5;
}}

/* Next/finish buttons */
.vn-next-topic, .vn-finish-btn {{
  background: linear-gradient(135deg, rgba(255,183,197,0.2), rgba(255,183,197,0.1));
  border: 1px solid rgba(255,183,197,0.3); border-radius: 10px;
  padding: 12px 24px; color: #ffb7c5; cursor: pointer;
  font-size: 0.9rem; font-weight: 600; transition: all 0.3s;
  align-self: flex-end;
}}
.vn-next-topic:hover, .vn-finish-btn:hover {{
  background: rgba(255,183,197,0.25); transform: translateY(-2px);
}}

/* === HOME CONTENT === */
#home-content {{ padding: 0; }}
.home-nav-grid {{
  display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;
  padding: 24px 32px;
}}
.home-nav-card {{
  background: rgba(255,255,255,0.03); border: 1px solid rgba(255,183,197,0.12);
  border-radius: 14px; padding: 28px 20px; text-align: center;
  display: flex; flex-direction: column; align-items: center; gap: 10px;
  transition: all 0.3s; text-decoration: none; color: inherit;
}}
.home-nav-card:hover {{
  background: rgba(255,183,197,0.08); border-color: #ffb7c5;
  transform: translateY(-4px); box-shadow: 0 8px 24px rgba(255,183,197,0.1);
}}
.home-nav-icon {{ font-size: 2rem; }}
.home-nav-title {{ font-size: 1rem; font-weight: 600; color: #ffb7c5; }}
.home-nav-desc {{ font-size: 0.8rem; color: rgba(255,255,255,0.5); }}

.home-stats-bar {{
  display: flex; justify-content: center; gap: 40px;
  padding: 16px; background: rgba(255,183,197,0.05);
  border-top: 1px solid rgba(255,183,197,0.1);
  border-bottom: 1px solid rgba(255,183,197,0.1);
}}
.home-stat {{ font-size: 0.9rem; color: rgba(255,255,255,0.6); }}
.home-stat strong {{ color: #ffb7c5; }}

.home-sections {{
  display: grid; grid-template-columns: 1fr 1fr; gap: 24px;
  padding: 24px 32px;
}}
.home-section h2 {{
  font-size: 1.1rem; color: #ffb7c5; margin-bottom: 16px;
  padding-bottom: 8px; border-bottom: 1px solid rgba(255,183,197,0.15);
}}

/* Floating Miou */
.home-miou-float {{
  position: fixed; bottom: 24px; right: 24px; z-index: 100;
  cursor: pointer; font-size: 2.5rem;
  animation: miouFloat 3s ease-in-out infinite;
}}
.home-miou-tip {{
  position: absolute; bottom: 100%; right: 0; margin-bottom: 8px;
  background: rgba(20,20,40,0.95); border: 1px solid rgba(255,183,197,0.3);
  border-radius: 10px; padding: 10px 14px; font-size: 0.8rem;
  color: rgba(255,255,255,0.8); white-space: nowrap;
  opacity: 0; transform: translateY(8px); transition: all 0.3s;
  pointer-events: none;
}}
.home-miou-float:hover .home-miou-tip {{
  opacity: 1; transform: translateY(0);
}}

/* Responsive */
@media (max-width: 1200px) {{
  .vn-s3-layout {{ grid-template-columns: 200px 1fr 340px; padding: 20px; }}
}}
@media (max-width: 900px) {{
  .vn-profiles {{ flex-wrap: wrap; justify-content: center; }}
  .vn-s3-layout {{ grid-template-columns: 1fr; padding: 16px; }}
  .vn-s3-miou {{ display: none; }}
  .home-nav-grid {{ grid-template-columns: repeat(2, 1fr); }}
  .home-sections {{ grid-template-columns: 1fr; }}
}}
</style>

<!-- ══════════════════════════════════════ -->
<!--  JAVASCRIPT VN ENGINE                 -->
<!-- ══════════════════════════════════════ -->
<script>
(function() {{
  'use strict';

  // === DATA : dialogue trees per profile ===
  const DIALOGUES = {{
    curious: [
      {{
        id: 'vision', icon: '\u{{1f31f}}', title: 'La vision Miyukini',
        body: 'Miyukini COG est un environnement num\u00e9rique souverain. Pas un logiciel, pas un cloud \u2014 un v\u00e9ritable espace de vie num\u00e9rique qui t\u0027appartient. Tes donn\u00e9es restent chez toi, toujours.',
        miou: 'Imagine un monde o\u00f9 ton t\u00e9l\u00e9phone, ton PC et tes fichiers ne d\u00e9pendent plus de personne. C\u0027est \u00e7a, un COG\u00a0!',
        choices: [
          {{label: 'Et le cloud\u00a0?', reply: 'Le cloud, c\u0027est pratique mais tu d\u00e9pends d\u0027une entreprise. Si elle ferme, change ses prix ou ses conditions\u2026 tu perds tout. Avec un COG, m\u00eame si Origin dispara\u00eet, ton environnement continue de fonctionner\u00a0!'}},
          {{label: 'Comment \u00e7a marche\u00a0?', reply: 'Tu installes Miyukini Central sur ton appareil, il cr\u00e9e ton COG. Les 8 Cores d\u00e9marrent automatiquement et prot\u00e8gent tout. Ensuite tu actives les services que tu veux\u00a0: m\u00e9dias, messagerie, partage\u2026'}}
        ]
      }},
      {{
        id: 'cores', icon: '\u{{1f3db}}\u{{fe0f}}', title: 'Les 8 Cores',
        body: 'Chaque COG est gouvern\u00e9 par 8 Cores immuables : StrongFather (identit\u00e9), KindMother (stockage), Caring Nanny (services), Master Butler (interface), Border Guard (s\u00e9curit\u00e9), Ever Buddy (communaut\u00e9), WorrySentinel (surveillance), TAMR (transactions).',
        miou: 'Ce sont les gardiens de ton COG. Personne ne peut les modifier, m\u00eame pas toi\u00a0! Ils prot\u00e8gent tout.',
        choices: null
      }},
      {{
        id: 'offline', icon: '\u{{1f4f5}}', title: 'Hors-ligne natif',
        body: 'Un COG fonctionne sans Internet. Tous tes services, fichiers et applications restent accessibles m\u00eame sans connexion. Quand tu te reconnectes, la synchronisation est automatique via le MWS.',
        miou: 'Plus de panique quand le WiFi coupe\u00a0! Ton COG continue de tourner comme si de rien n\u0027\u00e9tait.',
        choices: null
      }},
      {{
        id: 'mws', icon: '\u{{1f310}}', title: 'Le r\u00e9seau MWS',
        body: 'Le Miyukini Webway System (MWS) connecte les COGs entre eux de fa\u00e7on f\u00e9d\u00e9r\u00e9e. Pas de serveur central qui contr\u00f4le tout : chaque COG est autonome et choisit ses connexions.',
        miou: 'C\u0027est comme un r\u00e9seau d\u0027amis o\u00f9 chacun garde sa maison. Origin, c\u0027est juste le point de rendez-vous\u00a0!',
        choices: null
      }},
      {{
        id: 'compare', icon: '\u{{2696}}\u{{fe0f}}', title: 'vs. Le cloud traditionnel',
        body: 'Avec le cloud, tes donn\u00e9es sont sur les serveurs d\u0027une entreprise. Avec un COG, elles sont physiquement chez toi. Pas de frais mensuels, pas de conditions d\u0027utilisation qui changent, pas de censure possible.',
        miou: 'Cloud = louer un appartement. COG = \u00eatre propri\u00e9taire. La diff\u00e9rence est fondamentale\u00a0!',
        choices: null
      }}
    ],
    user: [
      {{
        id: 'start', icon: '\u{{1f680}}', title: 'D\u00e9marrer avec un COG',
        body: 'T\u00e9l\u00e9charge Miyukini Central, installe-le sur ton PC ou serveur, et c\u0027est parti\u00a0! L\u0027assistant de configuration te guide pas \u00e0 pas pour cr\u00e9er ton identit\u00e9 souveraine.',
        miou: 'C\u0027est super simple, je te guide \u00e0 chaque \u00e9tape\u00a0! En 5 minutes ton COG est pr\u00eat.',
        choices: [
          {{label: 'T\u00e9l\u00e9charger maintenant', reply: 'Super\u00a0! Rendez-vous sur la page T\u00e9l\u00e9chargements apr\u00e8s l\u0027onboarding. Tu y trouveras Central pour Windows, Linux et macOS. L\u0027installation prend moins de 2 minutes\u00a0!'}},
          {{label: 'En savoir plus d\u0027abord', reply: 'Bonne id\u00e9e\u00a0! Continue l\u0027exploration, je vais te montrer les services et la s\u00e9curit\u00e9. Tu pourras t\u00e9l\u00e9charger quand tu te sentiras pr\u00eat\u00a0!'}}
        ]
      }},
      {{
        id: 'central', icon: '\u{{1f3ae}}', title: 'Miyukini Central',
        body: 'Central est ton tableau de bord. Il te donne acc\u00e8s \u00e0 tous tes services, tes fichiers, tes contacts et tes param\u00e8tres. Une interface unique pour tout g\u00e9rer.',
        miou: 'Central, c\u0027est ta maison num\u00e9rique. Et moi je suis l\u00e0 dedans pour t\u0027aider\u00a0!',
        choices: null
      }},
      {{
        id: 'services', icon: '\u{{2728}}', title: 'Les services disponibles',
        body: 'MiyukiniWatch (m\u00e9dias), Jay1Tribu (famille et amis), JayKoa (gestion quotidienne), JayKonta (compta perso). Chaque service est int\u00e9gr\u00e9 dans ton COG et fonctionne hors-ligne.',
        miou: 'C\u0027est comme avoir Netflix, WhatsApp et Google Drive... mais tout est \u00e0 toi\u00a0!',
        choices: null
      }},
      {{
        id: 'security', icon: '\u{{1f512}}', title: 'S\u00e9curit\u00e9 et vie priv\u00e9e',
        body: 'Chiffrement de bout en bout, identit\u00e9 souveraine, pas de tracking, pas de publicit\u00e9. Border Guard prot\u00e8ge chaque connexion. Tes donn\u00e9es ne quittent jamais ton COG sans ta permission explicite.',
        miou: 'Ici, c\u0027est toi le patron. Aucune donn\u00e9e ne sort sans que tu le d\u00e9cides.',
        choices: null
      }},
      {{
        id: 'family', icon: '\u{{1f46a}}', title: 'COG familial',
        body: 'Un seul COG peut servir toute la famille. Chaque membre a son espace priv\u00e9 et ses permissions. Les enfants ont un acc\u00e8s s\u00e9curis\u00e9 adapt\u00e9 \u00e0 leur \u00e2ge.',
        miou: 'Toute la famille sous le m\u00eame toit num\u00e9rique, avec chacun sa chambre\u00a0!',
        choices: null
      }}
    ],
    dev: [
      {{
        id: 'archi', icon: '\u{{1f3d7}}\u{{fe0f}}', title: 'Architecture COG',
        body: 'Le COG utilise une architecture pyramidale \u00e0 10 strates (0\u20139). Les 8 Cores (Strate 4) forment le noyau immuable. BondingBrother (Strate 5) g\u00e8re la m\u00e9diation inter-crates. Tout est en Rust.',
        miou: 'C\u0027est du Rust pur, avec des contrats de gouvernance. Solide comme un roc\u00a0!',
        choices: [
          {{label: 'Voir la doc technique', reply: 'La doc technique couvre l\u0027architecture compl\u00e8te\u00a0: les 10 strates, les contrats de gouvernance, les protocoles MWS. Tout est dans la section Documentation du portail. C\u0027est bien comment\u00e9 et structur\u00e9\u00a0!'}},
          {{label: 'Parle-moi du MWS', reply: 'Le MWS c\u0027est le r\u00e9seau f\u00e9d\u00e9r\u00e9 qui connecte les COGs. Il y a 3 composants\u00a0: Origin (ici\u00a0!), le Relay pour les tunnels TLS chiffr\u00e9s, et le Tracker pour la d\u00e9couverte de pairs. Tout est v\u00e9rifi\u00e9 en 3 phases\u00a0!'}}
        ]
      }},
      {{
        id: 'mws-tech', icon: '\u{{1f5a7}}', title: 'MWS en d\u00e9tail',
        body: 'Le MWS se compose de 3 pi\u00e8ces : Origin (point central de v\u00e9rit\u00e9, c\u0027est ici\u00a0!), le Relay (tunnels TLS, sessions, v\u00e9rification 3 phases) et le Tracker (pools TCP, catalogues, lobbys).',
        miou: 'Origin = la mairie, Relay = les routes s\u00e9curis\u00e9es, Tracker = l\u0027annuaire du r\u00e9seau.',
        choices: null
      }},
      {{
        id: 'api', icon: '\u{{1f4e1}}', title: 'API Origin',
        body: 'Origin expose une API REST pour la gestion des pools, le catalogue de versions, les lobbys et la d\u00e9couverte de COGs. Documentation compl\u00e8te disponible dans /docs.',
        miou: 'L\u0027API est clean et bien document\u00e9e. Tu peux tout automatiser\u00a0!',
        choices: null
      }},
      {{
        id: 'contribute', icon: '\u{{1f91d}}', title: 'Contribuer',
        body: 'Le projet est open-source. Tu peux cr\u00e9er des services, des toolkits MWS, des adaptateurs inter-services. La documentation MSCM/MIP aide \u00e0 naviguer le codebase.',
        miou: 'On adore les contributions\u00a0! M\u00eame des petits fix, \u00e7a compte \u00e9norm\u00e9ment.',
        choices: null
      }},
      {{
        id: 'toolkits', icon: '\u{{1f9f0}}', title: 'Cr\u00e9er un service',
        body: 'Chaque service suit un pattern standard : modules data/, auth/, services/, export/. Les feature flags permettent d\u0027activer/d\u00e9sactiver des fonctionnalit\u00e9s. Les adaptateurs connectent les services entre eux.',
        miou: 'Le pattern est r\u00e9p\u00e9titif et pr\u00e9visible. Une fois que t\u0027as compris un service, tu les comprends tous\u00a0!',
        choices: null
      }}
    ],
    pro: [
      {{
        id: 'sovereignty', icon: '\u{{1f3e2}}', title: 'Souverainet\u00e9 des donn\u00e9es',
        body: 'Avec un COG professionnel, vos donn\u00e9es d\u0027entreprise restent dans vos locaux. Z\u00e9ro d\u00e9pendance cloud, conformit\u00e9 RGPD native, audit trail int\u00e9gr\u00e9 via les Cores.',
        miou: 'Fini les sueurs froides aux audits RGPD. Tout est chez vous, v\u00e9rifiable, souverain.',
        choices: [
          {{label: 'D\u00e9tails RGPD', reply: 'Un COG est RGPD-natif\u00a0: les donn\u00e9es ne quittent jamais vos locaux, le chiffrement est de bout en bout, et chaque acc\u00e8s est trac\u00e9 par les Cores. Pas besoin de DPO externe pour g\u00e9rer la conformit\u00e9\u00a0!'}},
          {{label: 'Cas d\u0027usage', reply: 'Cabinets m\u00e9dicaux pour les dossiers patients, PME pour la gestion interne, \u00e9coles pour les donn\u00e9es \u00e9l\u00e8ves, artisans pour leur compta locale\u2026 Partout o\u00f9 la souverainet\u00e9 des donn\u00e9es compte\u00a0!'}}
        ]
      }},
      {{
        id: 'jaykonta', icon: '\u{{1f9ee}}', title: 'JayKonta : compta unifi\u00e9e',
        body: 'JayKonta unifie votre comptabilit\u00e9 personnelle et professionnelle dans votre COG : devis, factures, paiements, rapports. Tout int\u00e9gr\u00e9, tout souverain.',
        miou: 'Votre boutique en ligne, mais sans plateforme tierce qui prend une commission\u00a0!',
        choices: null
      }},
      {{
        id: 'team', icon: '\u{{1f465}}', title: 'Travail d\u0027\u00e9quipe',
        body: 'Jay1Tribu permet la collaboration d\u0027\u00e9quipe : messagerie, partage de fichiers, espaces projet. Chaque membre a son identit\u00e9 souveraine et ses permissions granulaires.',
        miou: 'Slack + Drive + gestion de projet, mais tout reste dans votre COG.',
        choices: null
      }},
      {{
        id: 'costs', icon: '\u{{1f4b0}}', title: 'Co\u00fbts et mod\u00e8le',
        body: 'Pas d\u0027abonnement mensuel. Un COG tourne sur votre mat\u00e9riel existant (PC, serveur, Raspberry Pi). Les services sont inclus. Les mises \u00e0 jour sont gratuites.',
        miou: 'Z\u00e9ro frais r\u00e9currents. Votre investissement, c\u0027est juste le mat\u00e9riel que vous avez d\u00e9j\u00e0.',
        choices: null
      }},
      {{
        id: 'migration', icon: '\u{{1f504}}', title: 'Migration',
        body: 'Des outils de migration sont disponibles pour importer vos donn\u00e9es depuis les services cloud classiques. L\u0027import est progressif et non-destructif.',
        miou: 'On ne vous demande pas de tout couper d\u0027un coup. Migration en douceur, \u00e0 votre rythme.',
        choices: null
      }}
    ]
  }};

  const MIOU_REACTIONS = [
    'D\u0027accord\u00a0!', 'Tr\u00e8s bien\u00a0!', 'Allons-y\u00a0!',
    'Excellent choix\u00a0!', 'Parfait\u00a0!', 'Super\u00a0!'
  ];

  const HOME_TIPS = [
    'Besoin d\u0027aide\u00a0? Je suis l\u00e0\u00a0!',
    'Explore les services, tu vas adorer\u00a0!',
    'Pense \u00e0 t\u00e9l\u00e9charger Central\u00a0!',
    'Le r\u00e9seau MWS grandit chaque jour\u00a0!'
  ];

  let currentProfile = null;
  let currentTopicIdx = 0;
  let typingTimer = null;

  // === STARFIELD ===
  function initStarfield(canvasId) {{
    const canvas = document.getElementById(canvasId);
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    const stars = [];
    for (let i = 0; i < 200; i++) {{
      stars.push({{
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        r: Math.random() * 1.5 + 0.3,
        speed: Math.random() * 0.3 + 0.05,
        opacity: Math.random() * 0.8 + 0.2
      }});
    }}
    function draw() {{
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      stars.forEach(s => {{
        s.y -= s.speed;
        if (s.y < 0) {{ s.y = canvas.height; s.x = Math.random() * canvas.width; }}
        ctx.beginPath();
        ctx.arc(s.x, s.y, s.r, 0, Math.PI * 2);
        ctx.fillStyle = 'rgba(255,183,197,' + s.opacity + ')';
        ctx.fill();
      }});
      requestAnimationFrame(draw);
    }}
    draw();
  }}

  // === TYPEWRITER ===
  function typeText(el, text, speed, cb) {{
    let i = 0;
    const miou = document.getElementById('miou-large') || document.getElementById('miou-medium');
    if (miou) miou.classList.add('talking');
    clearInterval(typingTimer);
    el.textContent = '';
    typingTimer = setInterval(function() {{
      if (i < text.length) {{
        el.textContent += text[i];
        i++;
      }} else {{
        clearInterval(typingTimer);
        if (miou) miou.classList.remove('talking');
        if (cb) cb();
      }}
    }}, speed || 35);
  }}

  // === SCREEN TRANSITIONS ===
  function showScreen(n) {{
    document.querySelectorAll('.vn-screen').forEach(s => s.classList.remove('active'));
    const target = document.getElementById('vn-screen-' + n);
    if (target) target.classList.add('active');
    if (n === 2) initStarfield('starfield2');
    if (n === 3) initStarfield('starfield3');
  }}

  // === SCREEN 1 : click to continue ===
  document.getElementById('vn-screen-1').addEventListener('click', function(e) {{
    if (e.target.closest('.vn-skip-btn')) return;
    showScreen(2);
    setTimeout(function() {{
      typeText(
        document.getElementById('miou-s2-text'),
        'Bienvenue dans l\u0027univers Miyukini\u00a0! Je suis Miou, ton guide. Dis-moi qui tu es pour que je te montre ce qui t\u0027int\u00e9resse\u00a0!',
        30,
        function() {{ document.getElementById('vn-profiles').classList.add('visible'); }}
      );
    }}, 400);
  }});

  // === SCREEN 2 : profile selection ===
  window.vnSelectProfile = function(profile) {{
    currentProfile = profile;
    currentTopicIdx = 0;
    localStorage.setItem('miou_profile', profile);

    // Miou reaction
    const reaction = MIOU_REACTIONS[Math.floor(Math.random() * MIOU_REACTIONS.length)];
    document.getElementById('vn-profiles').style.pointerEvents = 'none';
    typeText(document.getElementById('miou-s2-text'), reaction, 25, function() {{
      setTimeout(function() {{
        showScreen(3);
        buildMenu(profile);
        showTopic(0);
      }}, 600);
    }});
  }};

  // === SCREEN 3 : build menu ===
  function buildMenu(profile) {{
    const menu = document.getElementById('vn-s3-menu');
    const topics = DIALOGUES[profile] || [];
    menu.innerHTML = '';
    topics.forEach(function(t, idx) {{
      const item = document.createElement('div');
      item.className = 'vn-menu-item' + (idx === 0 ? ' active' : '');
      item.innerHTML = '<span class="vn-menu-icon">' + t.icon + '</span> ' + t.title;
      item.onclick = function() {{ showTopic(idx); }};
      menu.appendChild(item);
    }});
  }}

  // === SCREEN 3 : show topic ===
  function showTopic(idx) {{
    const topics = DIALOGUES[currentProfile] || [];
    if (idx >= topics.length) return;
    currentTopicIdx = idx;
    const t = topics[idx];

    // Update menu active
    document.querySelectorAll('.vn-menu-item').forEach(function(el, i) {{
      el.classList.toggle('active', i === idx);
    }});

    document.getElementById('vn-topic-title').textContent = t.icon + ' ' + t.title;
    document.getElementById('vn-topic-body').textContent = t.body;
    document.getElementById('vn-miou-comment').textContent = '';
    typeText(document.getElementById('vn-miou-comment'), t.miou, 25);

    // Choices
    const choicesEl = document.getElementById('vn-choices');
    choicesEl.innerHTML = '';
    if (t.choices) {{
      t.choices.forEach(function(c) {{
        const btn = document.createElement('button');
        btn.className = 'vn-choice-btn';
        btn.textContent = c.label;
        btn.onclick = function() {{
          // Highlight selected, disable all
          choicesEl.querySelectorAll('.vn-choice-btn').forEach(function(b) {{
            b.disabled = true;
            b.style.opacity = '0.4';
            b.style.cursor = 'default';
          }});
          btn.style.opacity = '1';
          btn.style.borderColor = '#ffb7c5';
          btn.style.background = 'rgba(255,183,197,0.2)';
          // Miou responds
          document.getElementById('vn-miou-comment').textContent = '';
          typeText(document.getElementById('vn-miou-comment'), c.reply, 20);
        }};
        choicesEl.appendChild(btn);
      }});
    }}

    // Next or finish
    const isLast = idx >= topics.length - 1;
    document.getElementById('vn-next-btn').style.display = isLast ? 'none' : 'block';
    document.getElementById('vn-finish-btn').style.display = isLast ? 'block' : 'none';
  }}

  window.vnNextTopic = function() {{
    showTopic(currentTopicIdx + 1);
  }};

  // === FINISH : exit VN ===
  window.vnFinish = function() {{
    localStorage.setItem('miou_onboarding_done', '1');
    document.getElementById('vn-overlay').classList.add('hidden');
    document.getElementById('home-content').style.display = '';
    // Show header/footer
    const hdr = document.querySelector('.site-header');
    const ftr = document.querySelector('.site-footer');
    if (hdr) hdr.style.display = '';
    if (ftr) ftr.style.display = '';
    // Miou home tip
    const tip = HOME_TIPS[Math.floor(Math.random() * HOME_TIPS.length)];
    const tipEl = document.getElementById('home-miou-tip');
    if (tipEl) tipEl.textContent = tip;
  }};

  // === SKIP ===
  window.vnSkip = function() {{
    vnFinish();
  }};

  // === INIT ===
  function init() {{
    // Check localStorage
    const params = new URLSearchParams(window.location.search);
    const forceOnboarding = params.get('onboarding') === '1';
    const done = localStorage.getItem('miou_onboarding_done');

    if (done && !forceOnboarding) {{
      // Skip VN, show home directly
      document.getElementById('vn-overlay').classList.add('hidden');
      document.getElementById('home-content').style.display = '';
      const tip = HOME_TIPS[Math.floor(Math.random() * HOME_TIPS.length)];
      const tipEl = document.getElementById('home-miou-tip');
      if (tipEl) tipEl.textContent = tip;
    }} else {{
      // Hide header/footer during VN
      const hdr = document.querySelector('.site-header');
      const ftr = document.querySelector('.site-footer');
      if (hdr) hdr.style.display = 'none';
      if (ftr) ftr.style.display = 'none';
      // Start starfield on screen 1
      initStarfield('starfield');
    }}
  }}

  if (document.readyState === 'loading') {{
    document.addEventListener('DOMContentLoaded', init);
  }} else {{
    init();
  }}
}})();
</script>
        "##,
        total_cogs = total_cogs,
        total_lobbys = lobbys.len(),
        total_versions = versions.len(),
        latest_version = latest_version,
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
            let articles_html: String = s
                .articles
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

/// Page de téléchargements — style Genshin frost UI.
/// Seul Central est téléchargeable ici ; les services s'installent depuis l'app.
pub async fn downloads_page(content_mgr: &ContentManager) -> String {
    let downloads = content_mgr.get_downloads().await;
    let central = downloads
        .iter()
        .find(|d| d.category == DownloadCategory::Cog);

    let (dl_url, dl_version, dl_size_mb, dl_notes) = match central {
        Some(d) => (
            html_escape(&d.download_url),
            html_escape(&d.version),
            format!("{:.1}", d.size_bytes as f64 / 1_048_576.0),
            html_escape(&d.release_notes),
        ),
        None => (String::new(), "—".into(), "—".into(), String::new()),
    };

    let content = format!(
        r##"
<style>
/* ── Genshin-style frost page ── */
.gi-page {{
    position: relative;
    min-height: calc(100vh - 56px);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 1.5rem;
    overflow: hidden;
}}
.gi-page::before {{
    content: '';
    position: absolute; inset: 0;
    background:
        radial-gradient(ellipse 60% 50% at 30% 20%, rgba(100,160,255,0.12) 0%, transparent 70%),
        radial-gradient(ellipse 50% 40% at 70% 70%, rgba(139,92,246,0.10) 0%, transparent 70%),
        radial-gradient(ellipse 80% 60% at 50% 50%, rgba(6,182,212,0.06) 0%, transparent 80%);
    pointer-events: none;
    z-index: 0;
}}
.gi-page > * {{ position: relative; z-index: 1; }}

/* ── Titre ── */
.gi-title {{
    text-align: center;
    margin-bottom: 2.5rem;
}}
.gi-title h1 {{
    font-size: 2.2rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: #e8e4f0;
    text-shadow: 0 0 30px rgba(139,92,246,0.25);
    margin-bottom: 0.4rem;
}}
.gi-title p {{
    color: #9ca3af;
    font-size: 0.95rem;
    letter-spacing: 0.02em;
}}

/* ── Carte centrale frost ── */
.gi-card {{
    width: 100%;
    max-width: 520px;
    background: rgba(18,18,30,0.65);
    backdrop-filter: blur(24px) saturate(1.4);
    -webkit-backdrop-filter: blur(24px) saturate(1.4);
    border: 1px solid rgba(139,92,246,0.18);
    border-radius: 1.25rem;
    padding: 2.5rem 2rem 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    box-shadow:
        0 8px 40px rgba(0,0,0,0.35),
        inset 0 1px 0 rgba(255,255,255,0.04);
}}

/* icône app */
.gi-icon {{
    width: 80px; height: 80px;
    border-radius: 1rem;
    background: linear-gradient(135deg, #8b5cf6, #06b6d4);
    display: flex; align-items: center; justify-content: center;
    font-size: 2.2rem;
    box-shadow: 0 4px 24px rgba(139,92,246,0.35);
    margin-bottom: 1.25rem;
}}
.gi-app-name {{
    font-size: 1.35rem;
    font-weight: 600;
    color: #f0f0f5;
    letter-spacing: 0.02em;
}}
.gi-version {{
    font-size: 0.8rem;
    color: #9ca3af;
    margin-top: 0.2rem;
    letter-spacing: 0.03em;
}}
.gi-desc {{
    text-align: center;
    color: #b0aec0;
    font-size: 0.88rem;
    line-height: 1.55;
    margin: 1.25rem 0 1.5rem;
    max-width: 420px;
}}

/* badges */
.gi-badges {{
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
    justify-content: center;
}}
.gi-badge {{
    font-size: 0.72rem;
    padding: 0.3rem 0.7rem;
    border-radius: 999px;
    background: rgba(139,92,246,0.12);
    border: 1px solid rgba(139,92,246,0.2);
    color: #c4b5fd;
    letter-spacing: 0.02em;
}}

/* bouton télécharger */
.gi-dl-btn {{
    display: inline-flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.85rem 2.2rem;
    border-radius: 999px;
    background: linear-gradient(135deg, #8b5cf6, #7c3aed);
    color: #fff;
    font-size: 1rem;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-decoration: none;
    border: none;
    cursor: pointer;
    transition: all 0.25s ease;
    box-shadow: 0 4px 20px rgba(139,92,246,0.35);
}}
.gi-dl-btn:hover {{
    transform: translateY(-2px);
    box-shadow: 0 6px 28px rgba(139,92,246,0.5);
    color: #fff;
}}
.gi-dl-btn svg {{
    width: 18px; height: 18px;
    fill: currentColor;
}}
.gi-size {{
    font-size: 0.75rem;
    color: #9ca3af;
    margin-top: 0.75rem;
}}

/* séparateur */
.gi-sep {{
    width: 60%;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(139,92,246,0.25), transparent);
    margin: 2.5rem 0 1.5rem;
}}

/* note services */
.gi-note {{
    text-align: center;
    max-width: 460px;
}}
.gi-note h3 {{
    font-size: 0.95rem;
    font-weight: 500;
    color: #c4b5fd;
    margin-bottom: 0.5rem;
}}
.gi-note p {{
    font-size: 0.82rem;
    color: #7a7890;
    line-height: 1.5;
}}
.gi-services-row {{
    display: flex;
    gap: 0.6rem;
    flex-wrap: wrap;
    justify-content: center;
    margin-top: 0.8rem;
}}
.gi-svc {{
    display: flex;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.75rem;
    padding: 0.25rem 0.6rem;
    border-radius: 0.4rem;
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.06);
    color: #9ca3af;
}}
.gi-svc-icon {{ font-size: 0.9rem; }}

/* release notes */
.gi-release {{
    text-align: center;
    margin-top: 1rem;
    font-size: 0.78rem;
    color: #6b6880;
    font-style: italic;
}}

@media (max-width: 560px) {{
    .gi-card {{ padding: 2rem 1.25rem 1.5rem; }}
    .gi-title h1 {{ font-size: 1.6rem; }}
    .gi-icon {{ width: 64px; height: 64px; font-size: 1.8rem; }}
}}
</style>

<div class="gi-page">
    <div class="gi-title">
        <h1>Miyukini Central</h1>
        <p>Votre environnement COG, pr&ecirc;t &agrave; l&rsquo;emploi</p>
    </div>

    <div class="gi-card">
        <div class="gi-icon">&#x1f3ee;</div>
        <div class="gi-app-name">Miyukini Central</div>
        <div class="gi-version">v{dl_version} &mdash; Windows x64</div>

        <p class="gi-desc">
            Hub de gestion de votre COG. Inclut les 8 Cores syst&egrave;me,
            KindMother, le client MWS Webway et les voix de Miou.<br>
            Les services s&rsquo;installent directement depuis le Market int&eacute;gr&eacute;.
        </p>

        <div class="gi-badges">
            <span class="gi-badge">8 Cores</span>
            <span class="gi-badge">KindMother</span>
            <span class="gi-badge">Webway MWS</span>
            <span class="gi-badge">Voix Miou</span>
            <span class="gi-badge">Service Market</span>
        </div>

        <a href="{dl_url}" class="gi-dl-btn">
            <svg viewBox="0 0 24 24"><path d="M12 16l-5-5h3V4h4v7h3l-5 5zm-7 2h14v2H5v-2z"/></svg>
            T&eacute;l&eacute;charger
        </a>
        <div class="gi-size">{dl_size_mb} Mo &mdash; Installateur Windows (Inno Setup)</div>
    </div>

    <div class="gi-sep"></div>

    <div class="gi-note">
        <h3>7 services officiels disponibles depuis Central</h3>
        <p>Ouvrez le Market dans Central pour installer, mettre &agrave; jour<br>
        et g&eacute;rer vos services en un clic.</p>
        <div class="gi-services-row">
            <span class="gi-svc"><span class="gi-svc-icon">&#x1F4C6;</span> JayKoa</span>
            <span class="gi-svc"><span class="gi-svc-icon">&#x1F9EE;</span> JayKonta</span>
            <span class="gi-svc"><span class="gi-svc-icon">&#x1F4AC;</span> Jay1Tribu</span>
            <span class="gi-svc"><span class="gi-svc-icon">&#x1F4DA;</span> JayManga</span>
            <span class="gi-svc"><span class="gi-svc-icon">&#x1F441;</span> MiyukiniWatch</span>
            <span class="gi-svc"><span class="gi-svc-icon">&#x1F3AE;</span> Lord of the Click</span>
            <span class="gi-svc"><span class="gi-svc-icon">&#x1F3F0;</span> Miyukini Survivor</span>
        </div>
    </div>

    <div class="gi-release">{dl_notes}</div>
</div>
"##,
        dl_url = dl_url,
        dl_version = dl_version,
        dl_size_mb = dl_size_mb,
        dl_notes = dl_notes,
    );

    layout("Télécharger", &content, "downloads")
}

/// Page des services — inventaire Genshin Impact avec Miou mascotte.
pub async fn services_page(content_mgr: &ContentManager) -> String {
    let services = content_mgr.get_services().await;

    // Mapping service_id → emoji icon (from manifests)
    let icon_for = |id: &str| -> &str {
        match id {
            "jaykoa" => "&#x1F4C6;",
            "jaykonta" => "&#x1F9EE;",
            "miyukiniwatch" => "&#x1F441;",
            "jay1tribu" => "&#x1F4AC;",
            "jaymanga" => "&#x1F4DA;",
            _ => "&#x1F4E6;",
        }
    };

    // Couleur de rareté par catégorie (style Genshin)
    let rarity_for = |id: &str| -> &str {
        match id {
            "jaykoa" | "jaykonta" => "rarity-4",        // violet — productivité
            "jay1tribu" => "rarity-4",                  // violet — social
            "jaymanga" | "miyukiniwatch" => "rarity-3", // bleu — style de vie
            _ => "rarity-3",
        }
    };

    // Générer les slots d'inventaire
    let inventory_slots: String = services
        .iter()
        .map(|s| {
            format!(
                r#"<a href="/services/{id}" class="inv-slot {rarity}"
                    data-name="{name}" data-desc="{desc}" data-cat="{cat}">
                    <span class="inv-icon">{icon}</span>
                </a>"#,
                id = html_escape(&s.id),
                rarity = rarity_for(&s.id),
                name = html_escape(&s.name),
                desc = html_escape(&s.short_description),
                cat = html_escape(s.category.label()),
                icon = icon_for(&s.id),
            )
        })
        .collect();

    // Slots vides pour remplir la grille (style inventaire)
    let empty_count = if services.len() < 12 {
        12 - services.len()
    } else {
        0
    };
    let empty_slots: String = (0..empty_count)
        .map(|_| r#"<div class="inv-slot inv-empty"></div>"#.to_string())
        .collect();

    let content = format!(
        r##"
<style>
/* ═══ Genshin Inventory Page ═══ */
.inv-page {{
    position: relative;
    min-height: calc(100vh - 56px);
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem 1.5rem 3rem;
    overflow: hidden;
}}
.inv-page::before {{
    content: '';
    position: absolute; inset: 0;
    background:
        radial-gradient(ellipse 60% 50% at 25% 30%, rgba(100,160,255,0.10) 0%, transparent 70%),
        radial-gradient(ellipse 50% 40% at 75% 65%, rgba(139,92,246,0.08) 0%, transparent 70%),
        radial-gradient(ellipse 80% 60% at 50% 50%, rgba(6,182,212,0.05) 0%, transparent 80%);
    pointer-events: none;
    z-index: 0;
}}
.inv-page > * {{ position: relative; z-index: 1; }}

/* ── Titre ── */
.inv-header {{
    text-align: center;
    margin-bottom: 2rem;
}}
.inv-header h1 {{
    font-size: 1.8rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: #e8e4f0;
    text-shadow: 0 0 30px rgba(139,92,246,0.25);
    margin-bottom: 0.3rem;
}}
.inv-header p {{
    color: #9ca3af;
    font-size: 0.88rem;
    letter-spacing: 0.02em;
}}

/* ── Layout 3 panneaux ── */
.inv-layout {{
    display: grid;
    grid-template-columns: auto 1fr auto;
    gap: 2rem;
    align-items: start;
    max-width: 1100px;
    width: 100%;
}}

/* ── Panneau inventaire (gauche) ── */
.inv-panel {{
    background: rgba(18,18,30,0.55);
    backdrop-filter: blur(20px) saturate(1.3);
    -webkit-backdrop-filter: blur(20px) saturate(1.3);
    border: 1px solid rgba(139,92,246,0.15);
    border-radius: 1rem;
    padding: 1.25rem;
    box-shadow: 0 8px 32px rgba(0,0,0,0.3), inset 0 1px 0 rgba(255,255,255,0.03);
}}
.inv-panel-title {{
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: #8b8a9e;
    margin-bottom: 0.75rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid rgba(139,92,246,0.12);
}}
.inv-grid {{
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 6px;
}}

/* ── Slot d'inventaire ── */
.inv-slot {{
    width: 72px; height: 72px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    cursor: pointer;
    transition: all 0.2s ease;
    text-decoration: none;
    overflow: hidden;
}}
.inv-slot::before {{
    content: '';
    position: absolute; inset: 0;
    border-radius: 0.5rem;
    z-index: 0;
}}
.inv-slot:hover {{
    transform: scale(1.1);
    z-index: 5;
}}
.inv-icon {{
    font-size: 1.8rem;
    position: relative;
    z-index: 1;
    filter: drop-shadow(0 2px 6px rgba(0,0,0,0.4));
}}

/* Raretés (couleurs Genshin) */
.rarity-3 {{
    background: linear-gradient(180deg, #3b5998 0%, #465f8f 50%, #5a7ab5 100%);
    border: 1px solid rgba(90,122,181,0.5);
}}
.rarity-3::before {{
    background: linear-gradient(180deg, transparent 60%, rgba(90,122,181,0.25) 100%);
}}
.rarity-4 {{
    background: linear-gradient(180deg, #594580 0%, #6b4f96 50%, #8b6cb5 100%);
    border: 1px solid rgba(139,108,181,0.5);
}}
.rarity-4::before {{
    background: linear-gradient(180deg, transparent 60%, rgba(139,108,181,0.25) 100%);
}}
.rarity-5 {{
    background: linear-gradient(180deg, #8b6914 0%, #a07d24 50%, #c9a535 100%);
    border: 1px solid rgba(201,165,53,0.5);
}}
.rarity-5::before {{
    background: linear-gradient(180deg, transparent 60%, rgba(201,165,53,0.25) 100%);
}}
.inv-slot:hover.rarity-3 {{ box-shadow: 0 0 18px rgba(90,122,181,0.5); }}
.inv-slot:hover.rarity-4 {{ box-shadow: 0 0 18px rgba(139,108,181,0.5); }}
.inv-slot:hover.rarity-5 {{ box-shadow: 0 0 18px rgba(201,165,53,0.5); }}

/* Slot vide */
.inv-empty {{
    background: rgba(30,30,45,0.4);
    border: 1px dashed rgba(255,255,255,0.06);
    cursor: default;
}}

/* ── Mascotte Miou (centre) ── */
.inv-mascot {{
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 380px;
    padding: 1rem;
}}
.miou-figure {{
    width: 200px;
    height: 280px;
    background: radial-gradient(ellipse at center, rgba(139,92,246,0.08) 0%, transparent 70%);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
}}
.miou-figure::after {{
    content: '';
    position: absolute;
    bottom: 0;
    width: 120px;
    height: 20px;
    background: radial-gradient(ellipse, rgba(139,92,246,0.15) 0%, transparent 70%);
    border-radius: 50%;
}}
.miou-emoji {{
    font-size: 7rem;
    filter: drop-shadow(0 8px 24px rgba(139,92,246,0.3));
    animation: miou-float 3s ease-in-out infinite;
}}
@keyframes miou-float {{
    0%, 100% {{ transform: translateY(0); }}
    50% {{ transform: translateY(-8px); }}
}}
.miou-name {{
    font-size: 0.85rem;
    color: #c4b5fd;
    letter-spacing: 0.05em;
    margin-top: 0.5rem;
    font-weight: 500;
}}

/* ── Dialogue Miou (droite) ── */
.inv-dialogue {{
    background: rgba(18,18,30,0.55);
    backdrop-filter: blur(20px) saturate(1.3);
    -webkit-backdrop-filter: blur(20px) saturate(1.3);
    border: 1px solid rgba(139,92,246,0.15);
    border-radius: 1rem;
    padding: 1.5rem;
    max-width: 280px;
    min-width: 240px;
    box-shadow: 0 8px 32px rgba(0,0,0,0.3), inset 0 1px 0 rgba(255,255,255,0.03);
    position: relative;
}}
.inv-dialogue::before {{
    content: '';
    position: absolute;
    left: -8px; top: 40px;
    width: 0; height: 0;
    border-top: 8px solid transparent;
    border-bottom: 8px solid transparent;
    border-right: 8px solid rgba(18,18,30,0.55);
}}
.dlg-speaker {{
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: #c4b5fd;
    margin-bottom: 0.75rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid rgba(139,92,246,0.12);
}}
.dlg-text {{
    font-size: 0.88rem;
    color: #d0cde0;
    line-height: 1.6;
}}
.dlg-text em {{
    color: #c4b5fd;
    font-style: normal;
    font-weight: 500;
}}

/* ── Tooltip au curseur ── */
.inv-tooltip {{
    position: fixed;
    pointer-events: none;
    z-index: 999;
    background: rgba(12,12,22,0.92);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1px solid rgba(139,92,246,0.25);
    border-radius: 0.65rem;
    padding: 0.85rem 1rem;
    max-width: 260px;
    opacity: 0;
    transform: translateY(4px);
    transition: opacity 0.15s, transform 0.15s;
    box-shadow: 0 8px 28px rgba(0,0,0,0.5);
}}
.inv-tooltip.visible {{
    opacity: 1;
    transform: translateY(0);
}}
.tt-name {{
    font-size: 0.9rem;
    font-weight: 600;
    color: #f0f0f5;
    margin-bottom: 0.15rem;
}}
.tt-cat {{
    font-size: 0.7rem;
    color: #c4b5fd;
    letter-spacing: 0.03em;
    margin-bottom: 0.4rem;
}}
.tt-desc {{
    font-size: 0.78rem;
    color: #9ca3af;
    line-height: 1.45;
    margin-bottom: 0.5rem;
}}
.tt-cta {{
    font-size: 0.72rem;
    color: #8b5cf6;
    font-weight: 500;
    letter-spacing: 0.02em;
}}

/* ── Responsive ── */
@media (max-width: 900px) {{
    .inv-layout {{
        grid-template-columns: 1fr;
        gap: 1.5rem;
        justify-items: center;
    }}
    .inv-mascot {{ min-height: 200px; }}
    .miou-figure {{ width: 140px; height: 180px; }}
    .miou-emoji {{ font-size: 5rem; }}
    .inv-dialogue {{ max-width: 100%; }}
    .inv-dialogue::before {{ display: none; }}
}}
@media (max-width: 560px) {{
    .inv-grid {{ grid-template-columns: repeat(3, 1fr); }}
    .inv-slot {{ width: 64px; height: 64px; }}
    .inv-icon {{ font-size: 1.5rem; }}
}}
</style>

<div class="inv-page">
    <div class="inv-header">
        <h1>Services Miyukini</h1>
        <p>Survolez un service pour en savoir plus, cliquez pour ouvrir sa fiche</p>
    </div>

    <div class="inv-layout">
        <!-- Inventaire (gauche) -->
        <div class="inv-panel">
            <div class="inv-panel-title">Inventaire des services</div>
            <div class="inv-grid">
                {inventory_slots}
                {empty_slots}
            </div>
        </div>

        <!-- Miou (centre) -->
        <div class="inv-mascot">
            <div class="miou-figure">
                <span class="miou-emoji">&#x1F338;</span>
            </div>
            <span class="miou-name">Miou</span>
        </div>

        <!-- Dialogue (droite) -->
        <div class="inv-dialogue">
            <div class="dlg-speaker">Miou</div>
            <div class="dlg-text">
                Bienvenue dans l&rsquo;inventaire des services&nbsp;!<br><br>
                Survole les ic&ocirc;nes &agrave; gauche pour d&eacute;couvrir chaque service.
                Tous sont <em>gratuits</em> et disponibles depuis le Market de Central.<br><br>
                Clique sur un service pour voir sa pr&eacute;sentation compl&egrave;te&nbsp;!
            </div>
        </div>
    </div>
</div>

<!-- Tooltip flottant -->
<div class="inv-tooltip" id="inv-tooltip">
    <div class="tt-name" id="tt-name"></div>
    <div class="tt-cat" id="tt-cat"></div>
    <div class="tt-desc" id="tt-desc"></div>
    <div class="tt-cta">Cliquez pour en savoir plus &rarr;</div>
</div>

<script>
(function() {{
    var tip = document.getElementById('inv-tooltip');
    var slots = document.querySelectorAll('.inv-slot[data-name]');

    slots.forEach(function(slot) {{
        slot.addEventListener('mouseenter', function() {{
            document.getElementById('tt-name').textContent = slot.dataset.name;
            document.getElementById('tt-cat').textContent = slot.dataset.cat;
            document.getElementById('tt-desc').textContent = slot.dataset.desc;
            tip.classList.add('visible');
        }});

        slot.addEventListener('mousemove', function(e) {{
            var x = e.clientX + 16;
            var y = e.clientY + 16;
            // Prevent tooltip from going off-screen
            var rect = tip.getBoundingClientRect();
            if (x + rect.width > window.innerWidth) x = e.clientX - rect.width - 8;
            if (y + rect.height > window.innerHeight) y = e.clientY - rect.height - 8;
            tip.style.left = x + 'px';
            tip.style.top = y + 'px';
        }});

        slot.addEventListener('mouseleave', function() {{
            tip.classList.remove('visible');
        }});
    }});
}})();
</script>
"##,
        inventory_slots = inventory_slots,
        empty_slots = empty_slots,
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
            format!(
                r#"<img src="{}" alt="Bannière" style="max-width: 100%; border-radius: 0.5rem;">"#,
                html_escape(service.banner_url.as_deref().unwrap_or(""))
            )
        } else {
            "[petite bannière]".to_string()
        },
        html_escape(&service.short_description),
        html_escape(&service.editor),
        html_escape(&service.license),
        tags_html,
        if service.website_url.is_some() {
            format!(
                r#"<a href="{}" class="btn btn-secondary btn-lg" target="_blank">Site Officiel</a>"#,
                html_escape(service.website_url.as_deref().unwrap_or("#"))
            )
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

/// Page MIP v2 & MSCM — Présentation et distribution du protocole.
pub fn mip_page() -> String {
    let content = r##"
<!-- ══════════════════════════════════════════════════ -->
<!--  VN OVERLAY : ONBOARDING MIP IMMERSIF             -->
<!-- ══════════════════════════════════════════════════ -->
<div id="mip-vn-overlay">

  <!-- === SCREEN 1 : TITLE === -->
  <div class="mip-vn-screen active" id="mip-vn-s1">
    <div class="mip-vn-ornament top-left"></div>
    <div class="mip-vn-ornament top-right"></div>
    <div class="mip-vn-ornament bottom-left"></div>
    <div class="mip-vn-ornament bottom-right"></div>
    <div class="mip-vn-title-center">
      <div class="mip-vn-title-logo">MIP v2</div>
      <div class="mip-vn-title-sub">&amp; MSCM</div>
      <div class="mip-vn-title-tagline">Miyukini Implementation Protocol &mdash; AI-Governed Development</div>
      <div class="mip-vn-click" id="mip-vn-click" onclick="mipVnNext(1)">Cliquez pour d&eacute;couvrir</div>
    </div>
    <div class="mip-vn-skip-area">
      <button class="mip-vn-skip-btn" onclick="mipVnSkip()">Acc&egrave;s documentation directe &rarr;</button>
    </div>
  </div>

  <!-- === SCREEN 2 : MIOU + PATH CHOICE === -->
  <div class="mip-vn-screen" id="mip-vn-s2">
    <div class="mip-vn-s2-center">
      <div class="mip-miou-large">
        <span class="mip-miou-char">&#x1f338;</span>
        <div class="mip-miou-glow"></div>
      </div>
      <div class="mip-miou-dialogue" id="mip-s2-dialogue">
        <span id="mip-s2-text"></span><span class="mip-miou-caret">|</span>
      </div>
      <div class="mip-vn-paths" id="mip-vn-paths">
        <button class="mip-path-btn" onclick="mipVnStartImmersif()">
          <span class="mip-path-icon">&#x1f338;</span>
          <span class="mip-path-label">Immersif</span>
          <span class="mip-path-desc">Miou m'explique tout</span>
        </button>
        <button class="mip-path-btn" onclick="mipVnSkip()">
          <span class="mip-path-icon">&#x2699;&#xfe0f;</span>
          <span class="mip-path-label">Dev</span>
          <span class="mip-path-desc">Documentation directe</span>
        </button>
      </div>
    </div>
  </div>

  <!-- === SCREEN 3 : IMMERSIVE EXPLORATION === -->
  <div class="mip-vn-screen" id="mip-vn-s3">
    <div class="mip-vn-s3-layout">
      <div class="mip-vn-s3-menu" id="mip-vn-menu"></div>
      <div class="mip-vn-s3-miou">
        <div class="mip-miou-medium">
          <span class="mip-miou-char">&#x1f338;</span>
          <div class="mip-miou-glow"></div>
        </div>
      </div>
      <div class="mip-vn-s3-dialogue">
        <div class="mip-vn-topic-title" id="mip-topic-title"></div>
        <div class="mip-vn-topic-body" id="mip-topic-body"></div>
        <div class="mip-vn-topic-miou">
          <span class="mip-miou-mini">&#x1f338;</span>
          <span id="mip-miou-comment"></span>
        </div>
        <div class="mip-vn-choices" id="mip-vn-choices"></div>
        <button class="mip-vn-next" id="mip-next-btn" onclick="mipVnNextTopic()">Suivant &rarr;</button>
        <button class="mip-vn-finish" id="mip-finish-btn" onclick="mipVnSkip()" style="display:none">
          Voir la documentation compl&egrave;te &#x1f338;
        </button>
      </div>
    </div>
  </div>
</div>

<!-- ══════════════════════════════════════════════════ -->
<!--  DEV CONTENT (direct path)                        -->
<!-- ══════════════════════════════════════════════════ -->
<div id="mip-dev-content" style="display:none">

  <!-- Hero -->
  <section class="hero" style="padding: 3rem 0;">
    <h1>MIP v2 &amp; MSCM</h1>
    <p style="max-width:700px; margin:0 auto 1.5rem;">Protocole universel de gouvernance du d&eacute;veloppement assist&eacute; par IA &mdash; 10 agents sp&eacute;cialis&eacute;s, lifecycle complet, m&eacute;triques z&eacute;ro-estimation.</p>
    <div class="hero-buttons">
      <a href="https://github.com/StudioMiyukini/mip" class="btn btn-primary" target="_blank">GitHub &mdash; Installer MIP</a>
      <button class="btn btn-secondary" onclick="mipShowVn()">&#x1f338; D&eacute;couvrir avec Miou</button>
    </div>
  </section>

  <!-- Sticky TOC bar -->
  <nav class="mip-toc-bar" id="mip-toc-bar">
    <div class="mip-toc-track">
      <a href="#mip-what" class="mip-toc-link active" data-section="mip-what">MIP v2</a>
      <a href="#mscm-what" class="mip-toc-link" data-section="mscm-what">MSCM</a>
      <a href="#mip-architecture" class="mip-toc-link" data-section="mip-architecture">Architecture</a>
      <a href="#mip-agents" class="mip-toc-link" data-section="mip-agents">Agents</a>
      <a href="#mip-compare" class="mip-toc-link" data-section="mip-compare">Comparaison</a>
      <a href="#mip-pros-cons" class="mip-toc-link" data-section="mip-pros-cons">Forces &amp; limites</a>
      <a href="#mip-quickstart" class="mip-toc-link" data-section="mip-quickstart">Quick Start</a>
      <a href="#mip-conclusion" class="mip-toc-link" data-section="mip-conclusion">Conclusion</a>
    </div>
  </nav>

  <!-- Content -->
  <div class="mip-content">

      <!-- Section: What is MIP -->
      <section class="section" id="mip-what">
        <h2 class="section-title">Qu'est-ce que MIP v2&nbsp;?</h2>
        <p class="section-subtitle">Miyukini Implementation Protocol &mdash; version 2</p>
        <div class="card" style="padding:2rem;">
          <p><strong>MIP v2</strong> est un protocole universel de gouvernance du d&eacute;veloppement logiciel assist&eacute; par IA. Il structure le travail d'un agent IA (ou d'un essaim d'agents) en <strong>phases bien d&eacute;finies</strong>, avec des <strong>quality gates</strong> &agrave; chaque &eacute;tape, des <strong>agents sp&eacute;cialis&eacute;s par domaine</strong> et des <strong>m&eacute;triques mesur&eacute;es</strong> (jamais estim&eacute;es).</p>
          <p style="margin-top:1rem;">N&eacute; dans le projet Miyukini COG (Rust/Dioxus), MIP est <strong>stack-agnostique</strong> : son noyau (classification, phases, gates, agents) est invariant. Seule la configuration projet s'adapte &agrave; votre stack.</p>
        </div>
        <div class="grid grid-3" style="margin-top:1.5rem;">
          <div class="card">
            <div class="feature-icon">&#x1f3af;</div>
            <h3>5 classes de t&acirc;ches</h3>
            <p>T1 (micro-fix, 1 fichier) &rarr; T5 (chantier strat&eacute;gique, 10+ fichiers). Routing automatique vers les bonnes phases.</p>
          </div>
          <div class="card">
            <div class="feature-icon">&#x1f504;</div>
            <h3>7 phases</h3>
            <p>SETUP &rarr; P0 (cadrage 10 temps) &rarr; P3 (TDD parall&egrave;le) &rarr; P4 (audit) &rarr; P5 (livraison) &rarr; P6 (rapport final).</p>
          </div>
          <div class="card">
            <div class="feature-icon">&#x1f916;</div>
            <h3>3 modes d'autonomie</h3>
            <p><strong>FULL</strong> (autopilot), <strong>BIG_STEPS</strong> (gates inter-phases), <strong>GUIDED</strong> (validation &agrave; chaque &eacute;tape).</p>
          </div>
        </div>
      </section>

      <!-- Section: What is MSCM -->
      <section class="section" id="mscm-what">
        <h2 class="section-title">Qu'est-ce que MSCM&nbsp;?</h2>
        <p class="section-subtitle">Miyukini Semantic Code Markup</p>
        <div class="card" style="padding:2rem;">
          <p><strong>MSCM</strong> est un syst&egrave;me de balisage s&eacute;mantique du code source. Il permet &agrave; l'IA de <strong>raisonner sur la structure</strong> du code, valider son int&eacute;grit&eacute;, et effectuer du refactoring guid&eacute; par des m&eacute;tadonn&eacute;es.</p>
        </div>
        <div class="grid grid-2" style="margin-top:1.5rem;">
          <div class="card">
            <h3>5 annotations</h3>
            <p><code>@id</code> identifiant unique &bull; <code>@do</code> description fonctionnelle &bull; <code>@role</code> r&ocirc;le s&eacute;mantique &bull; <code>@layer</code> couche architecturale &bull; <code>@human</code> description lisible</p>
          </div>
          <div class="card">
            <h3>MSCM Index</h3>
            <p>10 fichiers JSON g&eacute;n&eacute;r&eacute;s automatiquement : blocks, hierarchy, graph, flows, domains, layers, dependencies, files, stats, registry.</p>
          </div>
        </div>
        <div class="card" style="margin-top:1rem; padding:1.5rem;">
          <h3>Exemple d'annotation</h3>
          <pre><code>// @id MGE-RENDER-001
// @do Rend le monde de jeu a l'ecran via le pipeline de rendu
// @role renderer
// @layer presentation
// @human Fonction principale de rendu du moteur de jeu
pub fn render_world(state: &amp;GameState) -&gt; Result&lt;()&gt; { ... }</code></pre>
        </div>
      </section>

      <!-- Section: Architecture -->
      <section class="section" id="mip-architecture">
        <h2 class="section-title">Architecture MIP</h2>
        <p class="section-subtitle">Lifecycle complet du cadrage au rapport final</p>
        <div class="card" style="padding:2rem;">
          <h3>Flux de travail standard</h3>
          <pre><code>Utilisateur
  &darr;
SETUP (une seule fois par environnement)
  &darr;
P0 &mdash; Cadrage (10 temps : exploration, ideation, concurrence,
      inventaire, securite, specs, plan, audit, CI/CD, synthese)
  &darr; [GATE : brief approuve + mode autonomie choisi]
P3 &mdash; Implementation TDD parallele (back + front)
  &darr; [GATE : tests + clippy + push]
P4 &mdash; Integration + Audit securite + Audit efficience
  &darr; [GATE : 0 defaut bloquant]
P5 &mdash; Livraison + Test humain
  &darr; [GATE : verdict utilisateur ACCEPTE/REFUSE]
P6 &mdash; Rapport final + Archivage + Capitalisation</code></pre>
        </div>
        <div class="grid grid-2" style="margin-top:1.5rem;">
          <div class="card">
            <h3>Classification T1-T5</h3>
            <table style="width:100%; font-size:0.85rem; margin-top:0.5rem;">
              <tr style="border-bottom:1px solid var(--border);"><td><strong>T1</strong></td><td>Micro-fix, 1 fichier, &lt;20 lignes</td><td>P3&rarr;P5</td></tr>
              <tr style="border-bottom:1px solid var(--border);"><td><strong>T2</strong></td><td>Fix cibl&eacute;, 1-3 fichiers</td><td>P2&rarr;P3&rarr;P5</td></tr>
              <tr style="border-bottom:1px solid var(--border);"><td><strong>T3</strong></td><td>Feature mod&eacute;r&eacute;e, 3-10 fichiers</td><td>P0&rarr;P6</td></tr>
              <tr style="border-bottom:1px solid var(--border);"><td><strong>T4</strong></td><td>Feature majeure, 10+ fichiers</td><td>P0&rarr;P6</td></tr>
              <tr><td><strong>T5</strong></td><td>Chantier strat&eacute;gique</td><td>P0&rarr;P6</td></tr>
            </table>
          </div>
          <div class="card">
            <h3>MASS &mdash; Agent Swarm System</h3>
            <p>Parall&eacute;lisation par DAG de d&eacute;pendances. 3 modes :</p>
            <ul style="margin:0.5rem 0 0 1rem; line-height:1.8;">
              <li><strong>Subagent burst</strong> (T2-T3) : agents l&eacute;gers en parall&egrave;le</li>
              <li><strong>Worktree swarm</strong> (T4) : chaque agent dans son worktree Git</li>
              <li><strong>Team swarm</strong> (T5) : &eacute;quipe compl&egrave;te coordonn&eacute;e</li>
            </ul>
            <p style="margin-top:0.5rem;">Loi 9 : si &gt;3 t&acirc;ches ind&eacute;pendantes &rarr; parall&eacute;lisation obligatoire.</p>
          </div>
        </div>
      </section>

      <!-- Section: Agents -->
      <section class="section" id="mip-agents">
        <h2 class="section-title">Les 10 agents sp&eacute;cialis&eacute;s</h2>
        <p class="section-subtitle">Chaque agent a un domaine d'expertise et des certifications</p>
        <div class="grid grid-2">
          <div class="card"><h3>&#x1f4cb; Maria &mdash; Chef de Projet</h3><p>Classifie les t&acirc;ches, dirige P0, brainstorming, suivi d'avancement, m&eacute;triques. Certifications : PMP, PRINCE2, PSM, ITIL 4.</p></div>
          <div class="card"><h3>&#x1f527; Denis &mdash; Lead Dev</h3><p>Architecture technique, plans d'impl&eacute;mentation, coordination back+front, tests finaux, merge. Certifications : TOGAF, ISO 25010.</p></div>
          <div class="card"><h3>&#x2699;&#xfe0f; Fran&ccedil;ois &mdash; Dev Back-End</h3><p>Impl&eacute;mentation back-end, API, DB, tests unitaires et int&eacute;gration. Certifications : ISTQB, OpenAPI 3.1.</p></div>
          <div class="card"><h3>&#x1f3a8; Lise &mdash; Dev Front-End</h3><p>UI/UX, composants, th&egrave;mes, onboarding, SEO, direction artistique. Certifications : WCAG 2.2, ISO 9241.</p></div>
          <div class="card"><h3>&#x1f4ca; Arianne &mdash; Team Manager QA</h3><p>Contr&ocirc;le qualit&eacute;, m&eacute;moire agents, anti-hallucination, archivage. Certifications : ISO 9001, Six Sigma.</p></div>
          <div class="card"><h3>&#x1f50d; George &mdash; Audit Expert</h3><p>Conformit&eacute; code vs doc, tests UX, tests globaux, benchmarks. Certifications : ISO 19011, CISA, RGPD.</p></div>
          <div class="card"><h3>&#x1f6e1;&#xfe0f; Victor &mdash; Cybers&eacute;curit&eacute;</h3><p>Threat modeling, audit surfaces d'attaque, OWASP, score s&eacute;curit&eacute; /100. Certifications : ISO 27001, CISSP, CEH.</p></div>
          <div class="card"><h3>&#x1f680; Hugo &mdash; DevOps</h3><p>CI/CD, conteneurisation, d&eacute;ploiement, monitoring, infra as code. Certifications : DevOps Foundation, AWS, CKA.</p></div>
          <div class="card"><h3>&#x26a1; Jean &mdash; Efficience IA</h3><p>Optimisation prompts, comptage tokens, audit consommation, benchmarks efficience. Certifications : FinOps, MLOps.</p></div>
          <div class="card"><h3>&#x1f4c8; Fabrice &mdash; Analyste PR</h3><p>Audit concurrentiel, cibles utilisateurs, points de friction, recommandations. Certifications : PSPO, Lean Startup.</p></div>
        </div>
      </section>

      <!-- Section: Comparison -->
      <section class="section" id="mip-compare">
        <h2 class="section-title">Comparaison avec les alternatives</h2>
        <p class="section-subtitle">7 frameworks analys&eacute;s &mdash; Mars 2026</p>
        <div style="overflow-x:auto;">
          <table class="mip-compare-table">
            <thead>
              <tr>
                <th>Dimension</th><th>MIP&nbsp;v2</th><th>BMAD</th><th>Spec&nbsp;Kit</th><th>OpenSpec</th><th>Kiro</th><th>GSD</th><th>Task&nbsp;Master</th>
              </tr>
            </thead>
            <tbody>
              <tr><td>Multi-agent</td><td class="mip-highlight">10 agents</td><td>8+ personas</td><td>Non</td><td>Non</td><td>Non</td><td>4 types</td><td>Non</td></tr>
              <tr><td>Phases lifecycle</td><td class="mip-highlight">7 phases</td><td>4 + Quick</td><td>4</td><td>3</td><td>3</td><td>6</td><td>3</td></tr>
              <tr><td>Quality gates</td><td class="mip-highlight">Par phase</td><td>Phase 3</td><td>3 gates</td><td>Minimal</td><td>Hooks</td><td>Verify</td><td>Deps</td></tr>
              <tr><td>S&eacute;curit&eacute; d&eacute;di&eacute;e</td><td class="mip-highlight">Victor /100</td><td>Non</td><td>Non</td><td>Non</td><td>Hooks</td><td>Minimal</td><td>Script</td></tr>
              <tr><td>M&eacute;triques</td><td class="mip-highlight">Zero-estimation</td><td>Non</td><td>Non</td><td>Non</td><td>Non</td><td>Alertes</td><td>Datetime</td></tr>
              <tr><td>Parall&eacute;lisation</td><td class="mip-highlight">MASS DAG</td><td>Party Mode</td><td>[P]</td><td>Non</td><td>Non</td><td>Vagues</td><td>Non</td></tr>
              <tr><td>Portabilit&eacute;</td><td>Claude Code</td><td>Multi-IDE</td><td class="mip-highlight">18+ outils</td><td class="mip-highlight">20+ outils</td><td>Kiro only</td><td>4 CLIs</td><td class="mip-highlight">7+ / 10+ IA</td></tr>
              <tr><td>Co&ucirc;t</td><td>Gratuit</td><td>Gratuit</td><td>Gratuit</td><td>Gratuit</td><td>$39-200/mo</td><td>Gratuit</td><td>BYOK</td></tr>
              <tr><td>Classification</td><td class="mip-highlight">T1-T5</td><td>L0-L4</td><td>Non</td><td>Non</td><td>Non</td><td>Quick/Full</td><td>Non</td></tr>
              <tr><td>Gouvernance code</td><td class="mip-highlight">MSCM</td><td>Non</td><td>Constitution</td><td>Non</td><td>Steering</td><td>Non</td><td>Non</td></tr>
            </tbody>
          </table>
        </div>
        <div class="grid grid-2" style="margin-top:1.5rem;">
          <div class="card">
            <h3>&#x1f3c6; BMAD Method <span style="color:var(--text-muted); font-weight:400;">~2k stars</span></h3>
            <p>Framework Agile le plus proche de MIP. 8+ personas, 34+ workflows. Manque : s&eacute;curit&eacute; d&eacute;di&eacute;e, m&eacute;triques, parall&eacute;lisation DAG.</p>
          </div>
          <div class="card">
            <h3>&#x1f4d0; GitHub Spec Kit <span style="color:var(--text-muted); font-weight:400;">~73k stars</span></h3>
            <p>Le plus populaire. Constitution system &eacute;l&eacute;gant. G&eacute;n&egrave;re ~2500 lignes de markdown par feature. Pas de multi-agent r&eacute;el.</p>
          </div>
          <div class="card">
            <h3>&#x26a1; OpenSpec <span style="color:var(--text-muted); font-weight:400;">~27k stars</span></h3>
            <p>L&eacute;ger et token-efficient. Delta specs + Gherkin. Aucune gouvernance ni s&eacute;curit&eacute;.</p>
          </div>
          <div class="card">
            <h3>&#x2601;&#xfe0f; AWS Kiro <span style="color:var(--text-muted); font-weight:400;">IDE propri&eacute;taire</span></h3>
            <p>Agent Hooks innovants. Lock-in IDE total. $39-200/mois. Pas de parall&eacute;lisation.</p>
          </div>
          <div class="card">
            <h3>&#x1f4aa; GSD <span style="color:var(--text-muted); font-weight:400;">~23k stars</span></h3>
            <p>R&eacute;sout le &laquo;context rot&raquo; avec des subagents frais. Vagues parall&egrave;les. Manque : agents sp&eacute;cialis&eacute;s par domaine.</p>
          </div>
          <div class="card">
            <h3>&#x1f4cb; Claude Task Master <span style="color:var(--text-muted); font-weight:400;">~15k stars</span></h3>
            <p>Le plus portable (7+ &eacute;diteurs, 10+ IA). Gestionnaire de t&acirc;ches, pas un protocole complet.</p>
          </div>
        </div>
      </section>

      <!-- Section: Pros/Cons -->
      <section class="section" id="mip-pros-cons">
        <h2 class="section-title">Forces &amp; limites</h2>
        <p class="section-subtitle">Une analyse honn&ecirc;te</p>
        <div class="grid grid-2">
          <div class="card" style="border-color: rgba(16,185,129,0.3);">
            <h3 style="color: var(--success);">&#x2705; Forces</h3>
            <ul style="margin:0.5rem 0 0 1rem; line-height:2;">
              <li><strong>10 agents sp&eacute;cialis&eacute;s</strong> dont 4 non-dev (s&eacute;curit&eacute;, infra, efficience, qualit&eacute;)</li>
              <li><strong>Lifecycle complet P0-P6</strong> avec quality gates explicites</li>
              <li><strong>MASS swarm</strong> &mdash; parall&eacute;lisation DAG, 3 modes</li>
              <li><strong>M&eacute;triques zero-estimation</strong> &mdash; valeurs mesur&eacute;es uniquement</li>
              <li><strong>MSCM</strong> &mdash; balisage s&eacute;mantique du code (unique)</li>
              <li><strong>Cadrage structur&eacute;</strong> &mdash; P0 en 10 temps pr&eacute;vient les d&eacute;rives</li>
              <li><strong>Audit s&eacute;curit&eacute; /100</strong> &mdash; agent Victor d&eacute;di&eacute;</li>
              <li><strong>Brainstorming standardis&eacute;</strong> &mdash; 5 sections, AskUserQuestion</li>
            </ul>
          </div>
          <div class="card" style="border-color: rgba(239,68,68,0.3);">
            <h3 style="color: var(--error);">&#x26a0;&#xfe0f; Limites</h3>
            <ul style="margin:0.5rem 0 0 1rem; line-height:2;">
              <li><strong>Portabilit&eacute; limit&eacute;e</strong> &mdash; coupl&eacute; &agrave; Claude Code (vs 20+ pour OpenSpec)</li>
              <li><strong>Courbe d'apprentissage</strong> &mdash; 10 agents, 7 phases, T1-T5</li>
              <li><strong>Overhead T1/T2</strong> &mdash; un simple fix n'a pas besoin de 10 temps de P0</li>
              <li><strong>CLAUDE.md dense</strong> &mdash; 200+ lignes (Anthropic recommande &lt;60)</li>
              <li><strong>Pas de communaut&eacute; publique</strong> &mdash; premier release</li>
              <li><strong>Token-hungry sur P0</strong> &mdash; 10 temps de cadrage consomment du contexte</li>
            </ul>
          </div>
        </div>
      </section>

      <!-- Section: Quick Start -->
      <section class="section" id="mip-quickstart">
        <h2 class="section-title">Quick Start</h2>
        <p class="section-subtitle">Installer MIP en 3 &eacute;tapes</p>
        <div class="card" style="padding:2rem;">
          <h3>1. Cloner le repo</h3>
          <pre><code>git clone https://github.com/StudioMiyukini/mip.git</code></pre>
          <h3 style="margin-top:1.5rem;">2. Copier dans votre projet</h3>
          <pre><code># Copier le skill MIP dans votre projet
cp -r mip/SKILL.md .cursor/skills/mip/SKILL.md
cp -r mip/modules/ .cursor/skills/mip/modules/

# OU pour Claude Code : ajouter la section MIP a votre CLAUDE.md
cat mip/templates/CLAUDE.md.template >> CLAUDE.md</code></pre>
          <h3 style="margin-top:1.5rem;">3. Lancer MIP</h3>
          <pre><code># Dans votre outil IA, lancez :
"Classe cette tache et lance le protocole MIP"

# MIP classifie automatiquement (T1-T5)
# et route vers les bonnes phases.</code></pre>
        </div>
        <div class="grid grid-3" style="margin-top:1.5rem;">
          <div class="card">
            <h3>Claude Code</h3>
            <p>Ajoutez la section MIP &agrave; votre <code>CLAUDE.md</code>. Les 10 agents sont automatiquement disponibles.</p>
          </div>
          <div class="card">
            <h3>Cursor</h3>
            <p>Placez <code>SKILL.md</code> + <code>modules/</code> dans <code>.cursor/skills/mip/</code>. R&eacute;f&eacute;rencez le skill dans votre workflow.</p>
          </div>
          <div class="card">
            <h3>Autres outils</h3>
            <p>Ajoutez le contenu de <code>SKILL.md</code> &agrave; votre fichier de contexte principal (<code>.cursorrules</code>, system prompt, etc.).</p>
          </div>
        </div>
      </section>

      <!-- Section: Conclusion -->
      <section class="section" id="mip-conclusion">
        <h2 class="section-title">Conclusion</h2>
        <div class="card" style="padding:2rem;">
          <p>MIP v2 est le premier protocole qui traite le d&eacute;veloppement assist&eacute; par IA comme un <strong>vrai processus industriel</strong> : avec des phases, des gates, des agents sp&eacute;cialis&eacute;s (y compris s&eacute;curit&eacute; et efficience), et des m&eacute;triques mesur&eacute;es &mdash; pas estim&eacute;es.</p>
          <p style="margin-top:1rem;">Combin&eacute; &agrave; <strong>MSCM</strong> pour la gouvernance s&eacute;mantique du code, il offre un cadre complet pour les projets ambitieux o&ugrave; la qualit&eacute;, la s&eacute;curit&eacute; et la tra&ccedil;abilit&eacute; comptent.</p>
          <div style="margin-top:2rem; text-align:center;">
            <a href="https://github.com/StudioMiyukini/mip" class="btn btn-primary" target="_blank" style="font-size:1.1rem; padding:1rem 2rem;">Installer MIP depuis GitHub</a>
          </div>
        </div>
      </section>

  </div><!-- .mip-content -->
</div><!-- #mip-dev-content -->

<!-- ══════════════════════════════════════════════════ -->
<!--  STYLES MIP PAGE                                  -->
<!-- ══════════════════════════════════════════════════ -->
<style>
/* === VN OVERLAY === */
#mip-vn-overlay {
  position: fixed; inset: 0; z-index: 9999;
  background: #0a0a14;
}
#mip-vn-overlay.hidden { display: none; }
.mip-vn-screen {
  position: absolute; inset: 0;
  display: none; flex-direction: column;
  align-items: center; justify-content: center;
  overflow: hidden;
  background: radial-gradient(ellipse at center, rgba(139,92,246,0.08) 0%, #0a0a14 70%);
}
.mip-vn-screen.active { display: flex; }
.mip-vn-ornament {
  position: absolute; width: 80px; height: 80px; z-index: 1;
  border-color: rgba(139,92,246,0.3); border-style: solid; border-width: 0;
}
.mip-vn-ornament.top-left { top:20px; left:20px; border-top-width:2px; border-left-width:2px; border-top-left-radius:12px; }
.mip-vn-ornament.top-right { top:20px; right:20px; border-top-width:2px; border-right-width:2px; border-top-right-radius:12px; }
.mip-vn-ornament.bottom-left { bottom:20px; left:20px; border-bottom-width:2px; border-left-width:2px; border-bottom-left-radius:12px; }
.mip-vn-ornament.bottom-right { bottom:20px; right:20px; border-bottom-width:2px; border-right-width:2px; border-bottom-right-radius:12px; }

.mip-vn-title-center { z-index:2; text-align:center; }
.mip-vn-title-logo {
  font-size:4.5rem; font-weight:800; letter-spacing:0.15em;
  background: linear-gradient(135deg, #8b5cf6, #06b6d4, #8b5cf6);
  -webkit-background-clip:text; -webkit-text-fill-color:transparent;
  animation: mipTitlePulse 3s ease-in-out infinite;
}
.mip-vn-title-sub {
  font-size:2rem; font-weight:300; letter-spacing:0.5em;
  color:rgba(255,255,255,0.7); margin-top:-8px;
}
.mip-vn-title-tagline {
  font-size:0.9rem; color:rgba(139,92,246,0.6); margin-top:16px;
  letter-spacing:0.15em; text-transform:uppercase;
}
.mip-vn-click {
  margin-top:48px; font-size:1rem; color:rgba(255,255,255,0.5);
  animation: mipPromptFade 2s ease-in-out infinite; cursor:pointer;
}
@keyframes mipTitlePulse { 0%,100%{filter:brightness(1);} 50%{filter:brightness(1.2);} }
@keyframes mipPromptFade { 0%,100%{opacity:0.4;} 50%{opacity:1;} }

.mip-vn-skip-area { position:absolute; bottom:24px; right:28px; z-index:2; }
.mip-vn-skip-btn {
  background:rgba(139,92,246,0.15); border:1px solid rgba(139,92,246,0.3);
  color:#8b5cf6; padding:10px 24px; border-radius:8px; cursor:pointer;
  font-size:0.9rem; transition:all 0.3s;
}
.mip-vn-skip-btn:hover { background:rgba(139,92,246,0.25); }

/* Screen 2 */
.mip-vn-s2-center {
  z-index:2; display:flex; flex-direction:column;
  align-items:center; gap:24px;
}
.mip-miou-large { position:relative; font-size:5rem; animation:mipMiouFloat 3s ease-in-out infinite; }
.mip-miou-medium { position:relative; font-size:3.5rem; animation:mipMiouFloat 3s ease-in-out infinite; }
.mip-miou-char { position:relative; z-index:1; }
.mip-miou-glow {
  position:absolute; inset:-20px; border-radius:50%;
  background:radial-gradient(circle, rgba(139,92,246,0.3), transparent 70%);
  animation:mipGlowPulse 2s ease-in-out infinite;
}
@keyframes mipMiouFloat { 0%,100%{transform:translateY(0);} 50%{transform:translateY(-10px);} }
@keyframes mipGlowPulse { 0%,100%{opacity:0.5;transform:scale(1);} 50%{opacity:1;transform:scale(1.15);} }
.mip-miou-large.talking { animation:mipMiouTalk 0.15s ease-in-out infinite; }
@keyframes mipMiouTalk { 0%,100%{transform:translateY(0) scale(1);} 50%{transform:translateY(-3px) scale(1.03);} }

.mip-miou-dialogue {
  background:rgba(20,20,40,0.85); border:1px solid rgba(139,92,246,0.3);
  border-radius:16px; padding:20px 28px; max-width:540px; min-height:60px;
  color:#fff; font-size:1rem; line-height:1.6;
  box-shadow:0 0 30px rgba(139,92,246,0.1);
}
.mip-miou-caret { color:#8b5cf6; animation:mipCaretBlink 0.8s step-end infinite; }
@keyframes mipCaretBlink { 0%,100%{opacity:1;} 50%{opacity:0;} }

.mip-vn-paths {
  display:flex; gap:16px; margin-top:12px;
  opacity:0; transform:translateY(20px); transition:all 0.6s ease;
}
.mip-vn-paths.visible { opacity:1; transform:translateY(0); }
.mip-path-btn {
  background:rgba(255,255,255,0.05); border:1px solid rgba(139,92,246,0.2);
  border-radius:12px; padding:20px 28px; cursor:pointer;
  display:flex; flex-direction:column; align-items:center; gap:8px;
  transition:all 0.3s; color:#fff; min-width:160px;
}
.mip-path-btn:hover {
  background:rgba(139,92,246,0.15); border-color:#8b5cf6;
  transform:translateY(-4px); box-shadow:0 8px 24px rgba(139,92,246,0.2);
}
.mip-path-icon { font-size:2rem; }
.mip-path-label { font-size:1.1rem; font-weight:600; }
.mip-path-desc { font-size:0.8rem; color:rgba(255,255,255,0.5); }

/* Screen 3 */
.mip-vn-s3-layout {
  position:relative; z-index:2;
  display:grid; grid-template-columns:260px 1fr 420px;
  width:100%; height:100%; padding:32px; gap:24px; align-items:start;
}
.mip-vn-s3-menu { display:flex; flex-direction:column; gap:4px; margin-top:40px; }
.mip-vn-menu-item {
  background:rgba(255,255,255,0.04); border:1px solid rgba(139,92,246,0.1);
  border-radius:8px; padding:12px 16px; cursor:pointer;
  color:rgba(255,255,255,0.6); font-size:0.85rem;
  transition:all 0.3s; display:flex; align-items:center; gap:8px;
}
.mip-vn-menu-item:hover { background:rgba(139,92,246,0.1); color:#fff; }
.mip-vn-menu-item.active {
  background:rgba(139,92,246,0.15); border-color:#8b5cf6;
  color:#8b5cf6; font-weight:600;
}
.mip-vn-menu-icon { font-size:1.1rem; }
.mip-vn-s3-miou { display:flex; align-items:center; justify-content:center; height:100%; }

.mip-vn-s3-dialogue {
  background:rgba(20,20,40,0.9); border:1px solid rgba(139,92,246,0.25);
  border-radius:16px; padding:28px; margin-top:32px;
  display:flex; flex-direction:column; gap:16px;
  max-height:calc(100vh - 120px); overflow-y:auto;
}
.mip-vn-topic-title {
  font-size:1.15rem; font-weight:700; color:#8b5cf6;
  padding-bottom:12px; border-bottom:1px solid rgba(139,92,246,0.15);
}
.mip-vn-topic-body { font-size:0.9rem; color:rgba(255,255,255,0.85); line-height:1.7; }
.mip-vn-topic-miou {
  display:flex; align-items:flex-start; gap:10px;
  background:rgba(139,92,246,0.08); border-radius:10px; padding:12px 16px;
  font-size:0.85rem; color:rgba(255,255,255,0.7); font-style:italic;
}
.mip-miou-mini { font-size:1.2rem; flex-shrink:0; }
.mip-vn-choices { display:flex; flex-direction:column; gap:8px; }
.mip-vn-choice-btn {
  background:rgba(255,255,255,0.05); border:1px solid rgba(139,92,246,0.2);
  border-radius:8px; padding:10px 16px; cursor:pointer;
  color:#fff; font-size:0.85rem; text-align:left; transition:all 0.3s;
}
.mip-vn-choice-btn:hover { background:rgba(139,92,246,0.15); border-color:#8b5cf6; }
.mip-vn-next, .mip-vn-finish {
  background:linear-gradient(135deg, rgba(139,92,246,0.2), rgba(6,182,212,0.1));
  border:1px solid rgba(139,92,246,0.3); border-radius:10px;
  padding:12px 24px; color:#8b5cf6; cursor:pointer;
  font-size:0.9rem; font-weight:600; transition:all 0.3s; align-self:flex-end;
}
.mip-vn-next:hover, .mip-vn-finish:hover { background:rgba(139,92,246,0.25); transform:translateY(-2px); }

/* === DEV CONTENT === */
.mip-toc-bar {
  position:sticky; top:64px; z-index:100;
  background:rgba(10,10,15,0.85); backdrop-filter:blur(12px); -webkit-backdrop-filter:blur(12px);
  border-bottom:1px solid rgba(139,92,246,0.15);
  padding:0 1rem;
  margin: 0 -2rem;
}
.mip-toc-track {
  display:flex; gap:0.25rem; overflow-x:auto; scrollbar-width:none;
  max-width:1200px; margin:0 auto; padding:0.5rem 0;
}
.mip-toc-track::-webkit-scrollbar { display:none; }
.mip-toc-link {
  flex-shrink:0; padding:0.45rem 1rem; border-radius:2rem;
  font-size:0.8rem; font-weight:500; white-space:nowrap;
  color:var(--text-muted); background:transparent;
  transition:all 0.2s ease; border:1px solid transparent;
}
.mip-toc-link:hover { color:var(--text); background:var(--bg-elevated); }
.mip-toc-link.active {
  color:#fff; background:rgba(139,92,246,0.2);
  border-color:rgba(139,92,246,0.4);
}
.mip-content { max-width:960px; margin:0 auto; }

/* Compare table */
.mip-compare-table {
  width:100%; border-collapse:collapse; font-size:0.8rem;
  background:var(--bg-surface); border:1px solid var(--border); border-radius:0.5rem;
}
.mip-compare-table th {
  background:var(--bg-elevated); padding:0.6rem 0.5rem; text-align:left;
  border-bottom:1px solid var(--border); font-weight:600; white-space:nowrap;
}
.mip-compare-table td {
  padding:0.5rem; border-bottom:1px solid rgba(255,255,255,0.05);
}
.mip-compare-table tr:hover { background:rgba(139,92,246,0.05); }
.mip-highlight { color:var(--primary); font-weight:600; }

/* Responsive */
@media (max-width:1200px) {
  .mip-vn-s3-layout { grid-template-columns:200px 1fr 340px; padding:20px; }
}
@media (max-width:900px) {
  .mip-vn-paths { flex-wrap:wrap; justify-content:center; }
  .mip-vn-s3-layout { grid-template-columns:1fr; padding:16px; }
  .mip-vn-s3-miou { display:none; }
  .mip-toc-link { font-size:0.75rem; padding:0.4rem 0.75rem; }
}
</style>

<!-- ══════════════════════════════════════════════════ -->
<!--  JAVASCRIPT MIP VN ENGINE                         -->
<!-- ══════════════════════════════════════════════════ -->
<script>
(function() {
  'use strict';

  const MIP_TOPICS = [
    {
      id:'what-mip', icon:'\u{1f3af}', title:'Qu\u0027est-ce que MIP\u00a0?',
      body:'<strong>MIP v2</strong> (Miyukini Implementation Protocol) est un protocole universel de gouvernance du d\u00e9veloppement assist\u00e9 par IA. Il structure le travail en <strong>phases bien d\u00e9finies</strong> avec des <strong>quality gates</strong>, des <strong>agents sp\u00e9cialis\u00e9s</strong> et des <strong>m\u00e9triques mesur\u00e9es</strong>.<br><br>5 classes de t\u00e2ches (T1 micro-fix \u2192 T5 chantier strat\u00e9gique), 7 phases (SETUP \u2192 P6), 3 modes d\u0027autonomie (FULL, BIG_STEPS, GUIDED).',
      miou:'Imagine un chef d\u0027orchestre pour ton \u00e9quipe IA. MIP dit qui fait quoi, quand, et v\u00e9rifie que c\u0027est bien fait\u00a0!'
    },
    {
      id:'what-mscm', icon:'\u{1f3f7}\ufe0f', title:'Qu\u0027est-ce que MSCM\u00a0?',
      body:'<strong>MSCM</strong> (Miyukini Semantic Code Markup) est un syst\u00e8me de balisage s\u00e9mantique du code source. 5 annotations (<code>@id</code>, <code>@do</code>, <code>@role</code>, <code>@layer</code>, <code>@human</code>) permettent \u00e0 l\u0027IA de <strong>raisonner sur la structure</strong> du code.<br><br>Le MSCM Index g\u00e9n\u00e8re 10 fichiers JSON : blocks, hierarchy, graph, flows, domains, layers, dependencies, files, stats, registry.',
      miou:'C\u0027est comme des \u00e9tiquettes intelligentes sur ton code. L\u0027IA peut les lire et comprendre la structure sans tout analyser\u00a0!'
    },
    {
      id:'agents', icon:'\u{1f465}', title:'Les 10 agents',
      body:'MIP d\u00e9finit 10 agents sp\u00e9cialis\u00e9s :<br><br>\u2022 <strong>Maria</strong> \u2014 Chef de Projet (PMP, PRINCE2)<br>\u2022 <strong>Denis</strong> \u2014 Lead Dev (TOGAF, ISO 25010)<br>\u2022 <strong>Fran\u00e7ois</strong> \u2014 Back-End (ISTQB, OpenAPI)<br>\u2022 <strong>Lise</strong> \u2014 Front-End (WCAG 2.2, ISO 9241)<br>\u2022 <strong>Arianne</strong> \u2014 QA Manager (ISO 9001, Six Sigma)<br>\u2022 <strong>George</strong> \u2014 Audit (ISO 19011, CISA)<br>\u2022 <strong>Victor</strong> \u2014 Cybers\u00e9curit\u00e9 (ISO 27001, CISSP)<br>\u2022 <strong>Hugo</strong> \u2014 DevOps (AWS, CKA)<br>\u2022 <strong>Jean</strong> \u2014 Efficience IA (FinOps, MLOps)<br>\u2022 <strong>Fabrice</strong> \u2014 Analyste PR (PSPO, Lean Startup)',
      miou:'4 agents ne sont m\u00eame pas des d\u00e9veloppeurs\u00a0! S\u00e9curit\u00e9, infra, efficience, qualit\u00e9\u2026 C\u0027est ce qui rend MIP unique.'
    },
    {
      id:'phases', icon:'\u{1f504}', title:'Les 7 phases',
      body:'<strong>SETUP</strong> \u2014 Onboarding environnement (une seule fois)<br><strong>P0</strong> \u2014 Cadrage complet en 10 temps (exploration, id\u00e9ation, concurrence, inventaire, s\u00e9curit\u00e9, specs, plan, audit, CI/CD, synth\u00e8se)<br><strong>P3</strong> \u2014 Impl\u00e9mentation TDD parall\u00e8le (back + front)<br><strong>P4</strong> \u2014 Int\u00e9gration + Audit s\u00e9curit\u00e9 + Audit efficience<br><strong>P5</strong> \u2014 Livraison + Test humain<br><strong>P6</strong> \u2014 Rapport final + Archivage + Capitalisation<br><br>Chaque transition a un <strong>quality gate</strong> explicite.',
      miou:'P0 c\u0027est le plus important\u00a0: 10 temps de r\u00e9flexion avant de coder. \u00c7a \u00e9vite de foncer dans le mur\u00a0!'
    },
    {
      id:'compare', icon:'\u{2696}\ufe0f', title:'Comparaison',
      body:'MIP v2 se compare \u00e0 7 alternatives :<br><br>\u2022 <strong>BMAD</strong> \u2014 8+ personas, pas de m\u00e9triques ni DAG<br>\u2022 <strong>GitHub Spec Kit</strong> \u2014 73k stars, 2500+ lignes/feature, pas de multi-agent<br>\u2022 <strong>OpenSpec</strong> \u2014 27k stars, l\u00e9ger mais aucune gouvernance<br>\u2022 <strong>AWS Kiro</strong> \u2014 IDE propri\u00e9taire, $39-200/mo<br>\u2022 <strong>GSD</strong> \u2014 23k stars, r\u00e9sout le context rot, pas d\u0027agents domaine<br>\u2022 <strong>Claude Task Master</strong> \u2014 15k stars, le plus portable, pas un protocole complet<br>\u2022 <strong>Better Agents</strong> \u2014 Standards + testing, pas un lifecycle',
      miou:'Aucun concurrent ne combine les 5 piliers de MIP\u00a0: multi-agent, lifecycle, MASS swarm, m\u00e9triques zero-estimation et MSCM\u00a0!'
    },
    {
      id:'quickstart', icon:'\u{1f680}', title:'Installation',
      body:'3 \u00e9tapes :<br><br><strong>1.</strong> <code>git clone https://github.com/StudioMiyukini/mip.git</code><br><br><strong>2.</strong> Copiez <code>SKILL.md</code> + <code>modules/</code> dans votre projet :<br>\u2022 Claude Code \u2192 ajoutez \u00e0 <code>CLAUDE.md</code><br>\u2022 Cursor \u2192 <code>.cursor/skills/mip/</code><br>\u2022 Autres \u2192 contexte/system prompt<br><br><strong>3.</strong> Dites \u00e0 votre IA : <em>\u00ab Classe cette t\u00e2che et lance MIP \u00bb</em>',
      miou:'C\u0027est litt\u00e9ralement 3 commandes. Clone, copie, lance. Ton IA fait le reste\u00a0!'
    },
    {
      id:'conclusion', icon:'\u{2728}', title:'Conclusion',
      body:'MIP v2 traite le d\u00e9veloppement assist\u00e9 par IA comme un <strong>vrai processus industriel</strong>\u00a0: phases, gates, agents sp\u00e9cialis\u00e9s, m\u00e9triques mesur\u00e9es. Combin\u00e9 \u00e0 MSCM pour la gouvernance s\u00e9mantique du code, il offre un cadre complet pour les projets ambitieux.<br><br><a href="https://github.com/StudioMiyukini/mip" target="_blank" style="color:#8b5cf6;font-weight:600;">Installer MIP depuis GitHub \u2192</a>',
      miou:'Merci d\u0027avoir explor\u00e9 MIP avec moi\u00a0! Si tu as des questions, le repo GitHub est l\u00e0 pour \u00e7a. \u00c0 bient\u00f4t\u00a0!'
    }
  ];

  let currentTopic = 0;

  function mipTypeText(el, text, cb) {
    el.textContent = '';
    let i = 0;
    const miouEl = document.querySelector('.mip-miou-large') || document.querySelector('.mip-miou-medium');
    if (miouEl) miouEl.classList.add('talking');
    const iv = setInterval(function() {
      if (i < text.length) { el.textContent += text[i]; i++; }
      else { clearInterval(iv); if (miouEl) miouEl.classList.remove('talking'); if (cb) cb(); }
    }, 28);
  }

  window.mipVnNext = function(from) {
    document.getElementById('mip-vn-s' + from).classList.remove('active');
    var next = from + 1;
    document.getElementById('mip-vn-s' + next).classList.add('active');
    if (next === 2) {
      setTimeout(function() {
        mipTypeText(document.getElementById('mip-s2-text'),
          'Salut\u00a0! Je suis Miou. Je vais te pr\u00e9senter MIP et MSCM, les protocoles qui gouvernent le d\u00e9veloppement IA chez Miyukini. Comment veux-tu d\u00e9couvrir\u00a0?',
          function() { document.getElementById('mip-vn-paths').classList.add('visible'); }
        );
      }, 400);
    }
  };

  window.mipVnStartImmersif = function() {
    document.getElementById('mip-vn-s2').classList.remove('active');
    document.getElementById('mip-vn-s3').classList.add('active');
    buildMenu();
    showTopic(0);
  };

  window.mipVnSkip = function() {
    document.getElementById('mip-vn-overlay').classList.add('hidden');
    document.getElementById('mip-dev-content').style.display = '';
  };

  window.mipShowVn = function() {
    document.getElementById('mip-dev-content').style.display = 'none';
    var overlay = document.getElementById('mip-vn-overlay');
    overlay.classList.remove('hidden');
    overlay.querySelector('.mip-vn-screen.active') ||
      document.getElementById('mip-vn-s1').classList.add('active');
  };

  function buildMenu() {
    var menu = document.getElementById('mip-vn-menu');
    menu.innerHTML = '';
    MIP_TOPICS.forEach(function(t, i) {
      var el = document.createElement('div');
      el.className = 'mip-vn-menu-item' + (i === 0 ? ' active' : '');
      el.innerHTML = '<span class="mip-vn-menu-icon">' + t.icon + '</span>' + t.title;
      el.onclick = function() { showTopic(i); };
      menu.appendChild(el);
    });
  }

  function showTopic(idx) {
    currentTopic = idx;
    var t = MIP_TOPICS[idx];
    document.getElementById('mip-topic-title').textContent = t.icon + ' ' + t.title;
    document.getElementById('mip-topic-body').innerHTML = t.body;
    document.getElementById('mip-miou-comment').textContent = t.miou;
    document.getElementById('mip-vn-choices').innerHTML = '';
    var items = document.querySelectorAll('.mip-vn-menu-item');
    items.forEach(function(el, i) { el.classList.toggle('active', i === idx); });
    var nextBtn = document.getElementById('mip-next-btn');
    var finishBtn = document.getElementById('mip-finish-btn');
    if (idx >= MIP_TOPICS.length - 1) {
      nextBtn.style.display = 'none';
      finishBtn.style.display = '';
    } else {
      nextBtn.style.display = '';
      finishBtn.style.display = 'none';
    }
  }

  window.mipVnNextTopic = function() {
    if (currentTopic < MIP_TOPICS.length - 1) showTopic(currentTopic + 1);
  };

  // Auto-show VN on first visit, dev content on return
  if (!sessionStorage.getItem('mip-visited')) {
    sessionStorage.setItem('mip-visited', '1');
  } else {
    mipVnSkip();
  }

  // Scroll-spy for TOC bar
  var tocLinks = document.querySelectorAll('.mip-toc-link');
  var sections = [];
  tocLinks.forEach(function(link) {
    var s = document.getElementById(link.getAttribute('data-section'));
    if (s) sections.push({el:s, link:link});
  });
  function updateToc() {
    var scrollY = window.scrollY + 140;
    var current = null;
    sections.forEach(function(item) {
      if (item.el.offsetTop <= scrollY) current = item;
    });
    tocLinks.forEach(function(l) { l.classList.remove('active'); });
    if (current) {
      current.link.classList.add('active');
      current.link.scrollIntoView({block:'nearest',inline:'center',behavior:'smooth'});
    }
  }
  window.addEventListener('scroll', updateToc, {passive:true});
})();
</script>
"##;

    layout("MIP v2 & MSCM", content, "mip")
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
                p.tags
                    .iter()
                    .map(|t| format!(r#"<span class="tag">{}</span>"#, html_escape(t)))
                    .collect::<String>()
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
        post.tags
            .iter()
            .map(|t| format!(r#"<span class="tag">{}</span>"#, html_escape(t)))
            .collect::<String>(),
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
            html.push_str(&format!(
                "<blockquote>{}</blockquote>\n",
                html_escape(&line[2..])
            ));
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

