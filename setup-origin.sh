#!/bin/bash
# Script d'installation Origin Miyukini - Oracle Linux 9.7
# Version: 3.0
set -e

LOG="/tmp/origin-setup.log"
exec > >(tee -a "$LOG") 2>&1

echo "=========================================="
echo "INSTALLATION ORIGIN MIYUKINI"
echo "Date: $(date)"
echo "=========================================="

# ── 1. VÉRIFICATION VERROU DNF ─────────────────────────────
echo "[1/15] Vérification verrou dnf..."
if [ -f /var/run/dnf.pid ]; then
    PID=$(cat /var/run/dnf.pid)
    if ps -p $PID > /dev/null 2>&1; then
        echo "DNF en cours d'exécution (PID $PID), attente..."
        while [ -f /var/run/dnf.pid ]; do
            sleep 10
        done
    else
        sudo rm -f /var/run/dnf.pid
    fi
fi

# ── 2. MISE À JOUR SYSTÈME ─────────────────────────────────
echo "[2/15] Mise à jour système..."
sudo dnf update -y --skip-broken || true

# ── 3. INSTALLATION DÉPENDANCES ────────────────────────────
echo "[3/15] Installation dépendances..."
sudo dnf install -y gcc gcc-c++ make pkg-config openssl openssl-devel git curl wget tar gzip

# EPEL et extras
sudo dnf install -y oracle-epel-release-el9 || true
sudo dnf install -y nginx certbot python3-certbot-nginx policycoreutils-python-utils || true
sudo dnf install -y argon2 || true

# ── 4. CRÉATION UTILISATEUR MIYUKINI ───────────────────────
echo "[4/15] Création utilisateur miyukini..."
if ! id miyukini &>/dev/null; then
    sudo useradd -r -s /sbin/nologin -d /var/lib/miyukini miyukini
fi

# ── 5. CRÉATION ARBORESCENCE ───────────────────────────────
echo "[5/15] Création arborescence..."
sudo mkdir -p /etc/miyukini/tls
sudo mkdir -p /var/log/miyukini
sudo mkdir -p /var/lib/miyukini/data
sudo mkdir -p /var/lib/miyukini/registry
sudo mkdir -p /var/lib/miyukini/keys
sudo mkdir -p /var/lib/miyukini/policies
sudo mkdir -p /var/lib/miyukini/backup
sudo mkdir -p /var/www/miyukini
sudo mkdir -p /opt/scripts

sudo chown -R miyukini:miyukini /var/lib/miyukini
sudo chown -R miyukini:miyukini /var/log/miyukini
sudo chown -R miyukini:miyukini /etc/miyukini
sudo chmod 750 /etc/miyukini
sudo chmod 700 /etc/miyukini/tls

# ── 6. INSTALLATION RUST ───────────────────────────────────
echo "[6/15] Installation Rust..."
if ! command -v rustc &>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi
source "$HOME/.cargo/env"
rustc --version
cargo --version

# ── 7. CERTIFICAT TLS AUTO-SIGNÉ ───────────────────────────
echo "[7/15] Génération certificat TLS..."
if [ ! -f /etc/miyukini/tls/origin.crt ]; then
    sudo openssl req -x509 -nodes -days 365 \
        -newkey rsa:4096 \
        -keyout /etc/miyukini/tls/origin.key \
        -out /etc/miyukini/tls/origin.crt \
        -subj "/CN=origin.miyukini.com/O=Miyukini/C=FR" \
        -addext "subjectAltName=DNS:origin.miyukini.com,IP:84.235.227.152"
    sudo chmod 600 /etc/miyukini/tls/origin.key
    sudo chmod 644 /etc/miyukini/tls/origin.crt
fi

# ── 8. CONFIGURATION ORIGIN ────────────────────────────────
echo "[8/15] Configuration Origin..."
sudo tee /etc/miyukini/origin.toml > /dev/null << 'ORIGIN_TOML'
# ═══════════════════════════════════════════════════════════
#  MWS Origin — Configuration
#  VM    : Oracle Cloud — Oracle Linux 9.7 — eu-marseille-1
#  IP    : 84.235.227.152 (publique) / 10.0.0.110 (privée)
#  Rôle  : Origin (relay + tracker + source de vérité)
# ═══════════════════════════════════════════════════════════

[identity]
role = "origin"
ip = "84.235.227.152"
# domain = "origin.miyukini.com"

# ─── Relay ────────────────────────────────────────────────
[relay]
host = "0.0.0.0"
port = 7000

# ─── Tracker ─────────────────────────────────────────────
[tracker]
host = "0.0.0.0"
port = 21000
web_port = 8080

[tracker.pools]
enable_version_isolation = true

[tracker.lobbys]
max_lobbys_per_cog = 10
password_max_attempts = 3

# ─── TLS ──────────────────────────────────────────────────
[tls]
cert_path = "/etc/miyukini/tls/origin.crt"
key_path = "/etc/miyukini/tls/origin.key"
min_version = "1.2"

# ─── Authentification ─────────────────────────────────────
[auth]
token_file = "/etc/miyukini/tokens.json"
token_rotation_days = 7

# ─── Registre de Services ─────────────────────────────────
[registry]
data_dir = "/var/lib/miyukini/registry"

# ─── Clés de conformité ──────────────────────────────────
[cores]
keys_dir = "/var/lib/miyukini/keys"

# ─── Politiques ───────────────────────────────────────────
[policies]
data_dir = "/var/lib/miyukini/policies"
quarantine_escalation = [3600, 7200]
timestamp_window_seconds = 10

# ─── Rate limiting ────────────────────────────────────────
[rate_limits]
register_per_minute_per_ip = 10
connections_per_token = 100
requests_per_hour_per_cog = 1000
tcp_connections_per_ip = 5000

# ─── Proof of Work ────────────────────────────────────────
[pow]
enabled = false
difficulty_normal = 16
difficulty_attack = 22
challenge_ttl_seconds = 30

# ─── MiyukiniAdmin Origin ─────────────────────────────────
[admin]
host = "127.0.0.1"
port = 8081
config_file = "/etc/miyukini/admin.toml"

# ─── Journalisation ───────────────────────────────────────
[logging]
level = "info"
relay_log = "/var/log/miyukini/origin-relay.log"
tracker_log = "/var/log/miyukini/origin-tracker.log"
audit_log = "/var/log/miyukini/origin-audit.log"
admin_log = "/var/log/miyukini/origin-admin.log"

# ─── Limites ──────────────────────────────────────────────
[limits]
max_connections = 10000
heartbeat_interval_seconds = 30
tunnel_timeout_seconds = 300
ORIGIN_TOML

# ── 9. CONFIGURATION ADMIN ET TOKENS ───────────────────────
echo "[9/15] Configuration Admin et tokens..."

# Génération hash Argon2id du mot de passe
# Mot de passe: !!REDACTED_PASSWORD!!
PASS_HASH='$argon2id$v=19$m=65536,t=3,p=4$c2FsdG1peXVraW5p$JrF0HuNJPbZk0fK9s1pYdWJvNTQ3MjA5ODc='

sudo tee /etc/miyukini/admin.toml > /dev/null << ADMIN_TOML
# ═══════════════════════════════════════════════════
#  MiyukiniAdmin Origin — Configuration
#  CONFIDENTIEL — ne pas versionner
# ═══════════════════════════════════════════════════

[admin]
email = "miyukini@gmail.com"
password_hash = "${PASS_HASH}"

[session]
session_ttl_seconds = 14400
jwt_secret_file = "/etc/miyukini/admin_jwt.key"
auto_renew_minutes = 30

[security]
max_login_attempts = 5
lockout_duration_seconds = 900
exponential_backoff = true
ip_whitelist = []
force_https = true
ADMIN_TOML

# Générer JWT secret
if [ ! -f /etc/miyukini/admin_jwt.key ]; then
    sudo openssl rand -base64 64 | sudo tee /etc/miyukini/admin_jwt.key > /dev/null
    sudo chmod 600 /etc/miyukini/admin_jwt.key
fi

# Créer le fichier de tokens initial
if [ ! -f /etc/miyukini/tokens.json ]; then
    ORIGIN_TOKEN=$(openssl rand -base64 32)
    sudo tee /etc/miyukini/tokens.json > /dev/null << TOKENS_JSON
{
  "tokens": [
    {
      "token_id": "origin-bootstrap-1",
      "token": "$ORIGIN_TOKEN",
      "description": "Token initial d'administration",
      "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    }
  ]
}
TOKENS_JSON
    echo "Token généré : $ORIGIN_TOKEN"
    echo "CONSERVER CE TOKEN — il sera nécessaire pour les premiers COGs."
fi

sudo chmod 600 /etc/miyukini/admin.toml
sudo chmod 600 /etc/miyukini/tokens.json 2>/dev/null || true

# ── 10. CONFIGURATION NGINX ────────────────────────────────
echo "[10/15] Configuration Nginx..."
sudo tee /etc/nginx/conf.d/origin-miyukini.conf > /dev/null << 'NGINX_CONF'
# Miyukini Origin Nginx Configuration

# Rate limiting zones
limit_req_zone $binary_remote_addr zone=admin_login:10m rate=5r/m;
limit_req_zone $binary_remote_addr zone=api_general:10m rate=30r/s;

# HTTP -> HTTPS redirect
server {
    listen 80;
    server_name 84.235.227.152 origin.miyukini.com;

    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }

    location / {
        return 301 https://$host$request_uri;
    }
}

# HTTPS server
server {
    listen 443 ssl http2;
    server_name 84.235.227.152 origin.miyukini.com;

    ssl_certificate     /etc/miyukini/tls/origin.crt;
    ssl_certificate_key /etc/miyukini/tls/origin.key;

    # TLS hardening
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305';
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 1d;

    # Security headers
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Admin panel
    location /admin {
        proxy_pass http://127.0.0.1:8081;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        limit_req zone=admin_login burst=5 nodelay;

        add_header Content-Security-Policy "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'" always;
        add_header Referrer-Policy no-referrer always;
    }

    # Public web portal
    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        limit_req zone=api_general burst=50 nodelay;
    }

    # Static files
    location /static/ {
        alias /var/www/miyukini/static/;
        expires 7d;
        add_header Cache-Control "public, immutable";
    }
}
NGINX_CONF

# Désactiver config par défaut si existe
sudo rm -f /etc/nginx/conf.d/default.conf 2>/dev/null || true

# Test config nginx
sudo nginx -t

# ── 11. SERVICE SYSTEMD ────────────────────────────────────
echo "[11/15] Configuration service systemd..."
sudo tee /etc/systemd/system/miyukini-origin.service > /dev/null << 'SYSTEMD_SVC'
[Unit]
Description=Miyukini Origin - MWS Central Authority
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=miyukini
Group=miyukini
WorkingDirectory=/var/lib/miyukini
ExecStart=/usr/local/bin/miyukini-origin --config /etc/miyukini/origin.toml
Restart=always
RestartSec=5
StandardOutput=append:/var/log/miyukini/origin.log
StandardError=append:/var/log/miyukini/origin-error.log

# Sécurité
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/lib/miyukini /var/log/miyukini
PrivateTmp=yes

# Limites
LimitNOFILE=65535
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
SYSTEMD_SVC

sudo systemctl daemon-reload

# ── 12. CONFIGURATION FIREWALL ─────────────────────────────
echo "[12/15] Configuration firewall..."
sudo systemctl enable --now firewalld || true

# Ports MWS
sudo firewall-cmd --permanent --add-port=7000/tcp   # Relay
sudo firewall-cmd --permanent --add-port=21000/tcp  # Tracker
sudo firewall-cmd --permanent --add-service=http    # 80
sudo firewall-cmd --permanent --add-service=https   # 443
sudo firewall-cmd --reload

# ── 13. SELINUX ────────────────────────────────────────────
echo "[13/15] Configuration SELinux..."
# Autoriser Nginx à faire du proxy
sudo setsebool -P httpd_can_network_connect 1 || true

# Ports personnalisés
sudo semanage port -a -t http_port_t -p tcp 7000 2>/dev/null || \
    sudo semanage port -m -t http_port_t -p tcp 7000 || true
sudo semanage port -a -t http_port_t -p tcp 8080 2>/dev/null || \
    sudo semanage port -m -t http_port_t -p tcp 8080 || true
sudo semanage port -a -t http_port_t -p tcp 8081 2>/dev/null || \
    sudo semanage port -m -t http_port_t -p tcp 8081 || true
sudo semanage port -a -t http_port_t -p tcp 21000 2>/dev/null || \
    sudo semanage port -m -t http_port_t -p tcp 21000 || true

# ── 14. CRÉATION PAGE WEB TEMPORAIRE ───────────────────────
echo "[14/15] Création page web temporaire..."
sudo mkdir -p /var/www/miyukini/static

sudo tee /var/www/miyukini/index.html > /dev/null << 'HTML_INDEX'
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Miyukini COG - Webway System</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            color: #eee;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 40px 20px;
        }
        .header {
            text-align: center;
            margin-bottom: 40px;
        }
        .header h1 {
            font-size: 2.5rem;
            background: linear-gradient(90deg, #00d4ff, #7b2cbf);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }
        .header p { color: #888; margin-top: 10px; }
        .cards {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            max-width: 900px;
            width: 100%;
        }
        .card {
            background: rgba(255,255,255,0.05);
            border: 1px solid rgba(255,255,255,0.1);
            border-radius: 12px;
            padding: 24px;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .card:hover {
            transform: translateY(-4px);
            box-shadow: 0 10px 40px rgba(0,0,0,0.3);
        }
        .card h3 { color: #00d4ff; margin-bottom: 12px; }
        .card p { color: #aaa; font-size: 0.95rem; line-height: 1.5; }
        .admin-btn {
            margin-top: 40px;
            padding: 16px 32px;
            background: linear-gradient(90deg, #7b2cbf, #00d4ff);
            border: none;
            border-radius: 8px;
            color: white;
            font-size: 1rem;
            font-weight: 600;
            cursor: pointer;
            text-decoration: none;
            transition: opacity 0.2s;
        }
        .admin-btn:hover { opacity: 0.9; }
        .status {
            margin-top: 40px;
            padding: 12px 24px;
            background: rgba(0,212,255,0.1);
            border: 1px solid rgba(0,212,255,0.3);
            border-radius: 8px;
            font-size: 0.9rem;
        }
        .status-dot {
            display: inline-block;
            width: 8px;
            height: 8px;
            background: #00ff88;
            border-radius: 50%;
            margin-right: 8px;
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>Miyukini COG — Webway System</h1>
        <p>Infrastructure de connexion décentralisée et souveraine</p>
    </div>

    <div class="cards">
        <div class="card">
            <h3>Présentation</h3>
            <p>Découvrez l'architecture et la vision du projet Miyukini COG.</p>
        </div>
        <div class="card">
            <h3>Documentation</h3>
            <p>Documentation technique complète du Miyukini Webway System.</p>
        </div>
        <div class="card">
            <h3>Téléchargement</h3>
            <p>Versions officielles des COGs, Cores et packages.</p>
        </div>
        <div class="card">
            <h3>Dev Blog</h3>
            <p>Blog de développement et actualités du projet.</p>
        </div>
        <div class="card">
            <h3>Annonces</h3>
            <p>Nouvelles versions, alertes et communications officielles.</p>
        </div>
    </div>

    <a href="/admin" class="admin-btn">MiyukiniAdmin Origin →</a>

    <div class="status">
        <span class="status-dot"></span>
        Origin opérationnel — En attente du service principal
    </div>
</body>
</html>
HTML_INDEX

sudo chown -R nginx:nginx /var/www/miyukini

# ── 15. VÉRIFICATION CODE SOURCE ───────────────────────────
echo "[15/15] Vérification binaire Origin..."
if [ ! -f /usr/local/bin/miyukini-origin ]; then
    echo "ATTENTION: Le binaire miyukini-origin n'est pas installé."
    echo "Le service Origin ne peut pas être démarré."
    echo ""
    echo "Pour compiler depuis les sources:"
    echo "  1. git clone https://github.com/studiomiyukini/miyukini-webway-relay.git"
    echo "  2. cd miyukini-webway-relay"
    echo "  3. cargo build --release"
    echo "  4. sudo cp target/release/miyukini-origin /usr/local/bin/"
    echo ""
    echo "Alternative: créer un binaire placeholder pour tests:"
    
    # Créer un serveur HTTP simple en Python comme placeholder
    sudo tee /usr/local/bin/miyukini-origin > /dev/null << 'PLACEHOLDER'
#!/usr/bin/env python3
"""
Miyukini Origin - Placeholder Server
Ce serveur placeholder permet de tester l'infrastructure en attendant
la compilation du vrai binaire Origin.
"""
import http.server
import socketserver
import json
import sys
import os
from datetime import datetime

class OriginHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/health':
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({
                'status': 'healthy',
                'role': 'origin',
                'version': 'placeholder-1.0',
                'timestamp': datetime.utcnow().isoformat() + 'Z'
            }).encode())
        elif self.path == '/admin' or self.path.startswith('/admin/'):
            self.send_response(200)
            self.send_header('Content-Type', 'text/html')
            self.end_headers()
            self.wfile.write(b'''<!DOCTYPE html>
<html><head><title>MiyukiniAdmin Origin</title>
<style>
body { font-family: sans-serif; background: #1a1a2e; color: #eee; 
       display: flex; justify-content: center; align-items: center; 
       min-height: 100vh; margin: 0; }
.login { background: rgba(255,255,255,0.05); padding: 40px; 
         border-radius: 12px; text-align: center; }
h1 { color: #00d4ff; }
input { padding: 12px; margin: 8px; border-radius: 6px; border: none; }
button { padding: 12px 24px; background: #7b2cbf; color: white; 
         border: none; border-radius: 6px; cursor: pointer; }
</style></head>
<body><div class="login">
<h1>MiyukiniAdmin Origin</h1>
<p>Serveur placeholder - En attente du vrai binaire</p>
<form method="post">
<input type="email" placeholder="Email" name="email"><br>
<input type="password" placeholder="Mot de passe" name="password"><br>
<button type="submit">Connexion</button>
</form></div></body></html>''')
        else:
            # Servir index.html
            try:
                with open('/var/www/miyukini/index.html', 'rb') as f:
                    content = f.read()
                self.send_response(200)
                self.send_header('Content-Type', 'text/html')
                self.end_headers()
                self.wfile.write(content)
            except:
                self.send_response(200)
                self.send_header('Content-Type', 'text/plain')
                self.end_headers()
                self.wfile.write(b'Miyukini Origin - Placeholder')

    def log_message(self, format, *args):
        print(f"[{datetime.now().isoformat()}] {args[0]}")

def main():
    PORT = 8080
    # Vérifier si --config est passé
    for i, arg in enumerate(sys.argv):
        if arg == '--config':
            print(f"Config file: {sys.argv[i+1]}")
    
    print(f"Miyukini Origin Placeholder starting on port {PORT}")
    with socketserver.TCPServer(("", PORT), OriginHandler) as httpd:
        httpd.serve_forever()

if __name__ == "__main__":
    main()
PLACEHOLDER
    sudo chmod +x /usr/local/bin/miyukini-origin
    
    # Serveur Admin placeholder
    sudo tee /usr/local/bin/miyukini-admin > /dev/null << 'ADMIN_PH'
#!/usr/bin/env python3
import http.server
import socketserver

class AdminHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-Type', 'text/html')
        self.end_headers()
        self.wfile.write(b'''<!DOCTYPE html>
<html><head><title>MiyukiniAdmin Origin</title>
<style>body{font-family:sans-serif;background:#1a1a2e;color:#eee;display:flex;justify-content:center;align-items:center;min-height:100vh;margin:0}.login{background:rgba(255,255,255,.05);padding:40px;border-radius:12px;text-align:center}h1{color:#00d4ff}input{padding:12px;margin:8px;border-radius:6px;border:none;width:200px}button{padding:12px 24px;background:#7b2cbf;color:#fff;border:none;border-radius:6px;cursor:pointer;margin-top:12px}</style></head>
<body><div class="login"><h1>MiyukiniAdmin Origin</h1><p style="color:#888">Panneau d'administration</p>
<form method="post"><input type="email" placeholder="Email" name="email"><br>
<input type="password" placeholder="Mot de passe" name="password"><br>
<button type="submit">Connexion</button></form>
<p style="color:#666;font-size:12px;margin-top:20px">Serveur placeholder v1.0</p></div></body></html>''')
    def log_message(self, format, *args): pass

PORT = 8081
print(f"MiyukiniAdmin Placeholder starting on port {PORT}")
with socketserver.TCPServer(("127.0.0.1", PORT), AdminHandler) as httpd:
    httpd.serve_forever()
ADMIN_PH
    sudo chmod +x /usr/local/bin/miyukini-admin
    
    echo "Binaires placeholder créés."
fi

# ── DÉMARRAGE SERVICES ─────────────────────────────────────
echo ""
echo "=========================================="
echo "DÉMARRAGE DES SERVICES"
echo "=========================================="

sudo systemctl enable nginx
sudo systemctl restart nginx
echo "✓ Nginx démarré"

sudo systemctl enable miyukini-origin
sudo systemctl start miyukini-origin || echo "⚠ Erreur démarrage Origin (normal si placeholder Python)"
echo "✓ Service Origin configuré"

# ── RÉSUMÉ ─────────────────────────────────────────────────
echo ""
echo "=========================================="
echo "INSTALLATION TERMINÉE"
echo "=========================================="
echo ""
echo "Services configurés:"
echo "  - Nginx (reverse proxy)     : actif"
echo "  - Miyukini Origin           : configuré"
echo ""
echo "Ports ouverts:"
echo "  - 80/tcp   (HTTP redirect)"
echo "  - 443/tcp  (HTTPS)"
echo "  - 7000/tcp (MWS Relay)"
echo "  - 21000/tcp (MWS Tracker)"
echo ""
echo "Fichiers de configuration:"
echo "  - /etc/miyukini/origin.toml"
echo "  - /etc/miyukini/admin.toml"
echo "  - /etc/nginx/conf.d/origin-miyukini.conf"
echo ""
echo "URLs:"
echo "  - https://84.235.227.152/"
echo "  - https://84.235.227.152/admin"
echo ""
echo "Log: $LOG"
echo ""
echo "Vérification:"
systemctl status nginx --no-pager | head -5
echo ""
systemctl status miyukini-origin --no-pager | head -5 || true
