#!/bin/bash
# Installation Origin Miyukini - Hostinger VPS Debian 13
# IP: 46.202.129.65
set -e

ORIGIN_IP="46.202.129.65"
LOG="/tmp/origin-setup-hostinger.log"
exec > >(tee -a "$LOG") 2>&1

echo "=========================================="
echo "INSTALLATION ORIGIN MIYUKINI (Hostinger)"
echo "Date: $(date)"
echo "=========================================="

# ── 1. Mise à jour et dépendances (apt) ─────────────────────
echo "[1/12] Mise à jour et dépendances..."
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq build-essential pkg-config libssl-dev git curl wget tar gzip
apt-get install -y -qq nginx certbot python3-certbot-nginx || true
apt-get install -y -qq argon2 || true

# ── 2. Utilisateur miyukini ────────────────────────────────
echo "[2/12] Utilisateur miyukini..."
if ! id miyukini &>/dev/null; then
    useradd -r -s /sbin/nologin -d /var/lib/miyukini miyukini
fi

# ── 3. Arborescence ────────────────────────────────────────
echo "[3/12] Arborescence..."
mkdir -p /etc/miyukini/tls /var/log/miyukini /var/lib/miyukini/{data,registry,keys,policies,backup} /var/www/miyukini/static /opt/scripts
chown -R miyukini:miyukini /var/lib/miyukini /var/log/miyukini /etc/miyukini
chmod 750 /etc/miyukini
chmod 700 /etc/miyukini/tls

# ── 4. Rust ────────────────────────────────────────────────
echo "[4/12] Rust..."
if ! command -v rustc &>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.cargo/bin:$PATH"
rustc --version
cargo --version

# ── 5. Certificat TLS ──────────────────────────────────────
echo "[5/12] Certificat TLS..."
if [ ! -f /etc/miyukini/tls/origin.crt ]; then
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout /etc/miyukini/tls/origin.key \
        -out /etc/miyukini/tls/origin.crt \
        -subj "/CN=${ORIGIN_IP}/O=Miyukini/C=FR"
    chmod 600 /etc/miyukini/tls/origin.key
    chown miyukini:miyukini /etc/miyukini/tls/*
fi

# ── 6. Configuration origin.toml ───────────────────────────
echo "[6/12] Configuration origin.toml..."
tee /etc/miyukini/origin.toml > /dev/null << ORIGIN_TOML
[identity]
role = "origin"
ip = "${ORIGIN_IP}"

[relay]
host = "0.0.0.0"
port = 7000

[tracker]
host = "0.0.0.0"
port = 21000
web_port = 8080

[tracker.pools]
enable_version_isolation = true

[tracker.lobbys]
max_lobbys_per_cog = 10
password_max_attempts = 3

[tls]
cert_path = "/etc/miyukini/tls/origin.crt"
key_path = "/etc/miyukini/tls/origin.key"
min_version = "1.2"

[auth]
token_file = "/etc/miyukini/tokens.json"
token_rotation_days = 7

[registry]
data_dir = "/var/lib/miyukini/registry"

[cores]
keys_dir = "/var/lib/miyukini/keys"

[policies]
data_dir = "/var/lib/miyukini/policies"
quarantine_escalation = [3600, 7200]
timestamp_window_seconds = 10

[rate_limits]
register_per_minute_per_ip = 10
connections_per_token = 100
requests_per_hour_per_cog = 1000
tcp_connections_per_ip = 5000

[pow]
enabled = false

[admin]
host = "127.0.0.1"
port = 8081
config_file = "/etc/miyukini/admin.toml"

[logging]
level = "info"
relay_log = "/var/log/miyukini/origin-relay.log"
tracker_log = "/var/log/miyukini/origin-tracker.log"
audit_log = "/var/log/miyukini/origin-audit.log"
admin_log = "/var/log/miyukini/origin-admin.log"

[limits]
max_connections = 10000
heartbeat_interval_seconds = 30
tunnel_timeout_seconds = 300
ORIGIN_TOML

# ── 7. Admin et tokens ─────────────────────────────────────
echo "[7/12] Admin et tokens..."
PASS_HASH='$argon2id$v=19$m=65536,t=3,p=4$c2FsdG1peXVraW5p$JrF0HuNJPbZk0fK9s1pYdWJvNTQ3MjA5ODc='
tee /etc/miyukini/admin.toml > /dev/null << ADMIN_TOML
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

[ ! -f /etc/miyukini/admin_jwt.key ] && openssl rand -base64 64 > /etc/miyukini/admin_jwt.key && chmod 600 /etc/miyukini/admin_jwt.key

if [ ! -f /etc/miyukini/tokens.json ]; then
    ORIGIN_TOKEN=$(openssl rand -base64 32)
    tee /etc/miyukini/tokens.json > /dev/null << TOKENS_JSON
{
  "tokens": [
    {
      "token_id": "origin-bootstrap-1",
      "token": "$ORIGIN_TOKEN",
      "description": "Token initial",
      "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    }
  ]
}
TOKENS_JSON
    echo "Token: $ORIGIN_TOKEN"
fi

chmod 600 /etc/miyukini/admin.toml /etc/miyukini/tokens.json 2>/dev/null || true
chown -R miyukini:miyukini /etc/miyukini

# ── 8. Nginx (sites-available) ─────────────────────────────
echo "[8/12] Nginx..."
tee /etc/nginx/sites-available/origin-miyukini.conf > /dev/null << 'NGINX_CONF'
limit_req_zone $binary_remote_addr zone=admin_login:10m rate=5r/m;

server {
    listen 80;
    server_name 46.202.129.65 origin.miyukini.com;
    location /.well-known/acme-challenge/ { root /var/www/html; }
    location / { return 301 https://$host$request_uri; }
}

server {
    listen 443 ssl http2;
    server_name 46.202.129.65 origin.miyukini.com;
    ssl_certificate     /etc/miyukini/tls/origin.crt;
    ssl_certificate_key /etc/miyukini/tls/origin.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;

    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;
    add_header Strict-Transport-Security "max-age=31536000" always;

    location /admin {
        proxy_pass http://127.0.0.1:8081;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        limit_req zone=admin_login burst=5 nodelay;
    }

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
NGINX_CONF

ln -sf /etc/nginx/sites-available/origin-miyukini.conf /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default
nginx -t

# ── 9. Service systemd ─────────────────────────────────────
echo "[9/12] Service systemd..."
tee /etc/systemd/system/miyukini-origin.service > /dev/null << 'SYSTEMD_SVC'
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
NoNewPrivileges=yes
ProtectSystem=strict
ReadWritePaths=/var/lib/miyukini /var/log/miyukini
PrivateTmp=yes
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
SYSTEMD_SVC

systemctl daemon-reload

# ── 10. Binaire : build si source présente ───────────────────
echo "[10/12] Binaire Origin..."
if [ -f /usr/local/bin/miyukini-origin ]; then
    echo "Binaire déjà installé."
elif [ -d /root/miyukini-cog ]; then
    echo "Compilation depuis /root/miyukini-cog..."
    export PATH="$HOME/.cargo/bin:$PATH"
    cd /root/miyukini-cog
    cargo build --release -p miyukini-origin
    cp target/release/miyukini-origin /usr/local/bin/
    chmod +x /usr/local/bin/miyukini-origin
    echo "Binaire installé."
else
    echo "ATTENTION: Pas de binaire ni de source. Copiez le binaire vers /usr/local/bin/miyukini-origin ou les sources vers /root/miyukini-cog puis relancez."
fi

# ── 11. Démarrer services ───────────────────────────────────
echo "[11/12] Démarrage services..."
systemctl enable nginx
systemctl restart nginx

if [ -f /usr/local/bin/miyukini-origin ]; then
    systemctl enable miyukini-origin
    systemctl start miyukini-origin
    echo "Origin démarré."
else
    echo "Origin non démarré (pas de binaire)."
fi

# ── 12. Résumé ─────────────────────────────────────────────
echo "[12/12] Résumé"
echo "=========================================="
echo "URLs: https://${ORIGIN_IP}/  https://${ORIGIN_IP}/admin"
echo "Log: $LOG"
echo "=========================================="
systemctl status nginx --no-pager | head -3
[ -f /usr/local/bin/miyukini-origin ] && systemctl status miyukini-origin --no-pager | head -5 || true
