# MWS — Implémentation Origin sur Hostinger VPS (Debian 13)

## Contexte

Ce document est le **guide d'implémentation complet et exécutable** d'Origin sur le VPS Hostinger (Debian 13). Un agent IA ou un opérateur humain doit pouvoir suivre ce guide **de A à Z** et obtenir un Origin fonctionnel.

Origin est le point central de vérité du MWS : il cumule les fonctions **relay** (vérification de conformité), **tracker** (pools, catalogue, connexions) et **source de vérité** (Registre de Services, versions, politiques).

**Références :**
- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](../acteurs/MWS%20-%20Origin.md)
- [MWS - MiyukiniAdmin Origin](../administration/MWS%20-%20MiyukiniAdmin.md)
- [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md)

---

## 0. Fiche d'identité de l'instance

| Paramètre | Valeur |
|-----------|--------|
| **Provider** | Hostinger |
| **Type** | VPS |
| **OS** | **Debian 13** (Trixie) |
| **IP publique** | `46.202.129.65` |
| **Utilisateur SSH** | `root` |

### Clé SSH

| Fichier | Chemin dans le workspace |
|---------|--------------------------|
| **Clé privée** | `ssh-key-2026-02-12.key` |
| **Clé publique** | `ssh-key-2026-02-12.key.pub` |

**Clé publique de référence (à conserver sur tout hébergeur) :**

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIVTDqC5yyNd6Ir9/NLjzUTT1IhugyRyRCo6+O5LZC4Z miyukini@gmail.com
```

À placer dans `~/.ssh/authorized_keys` du compte admin sur le VPS (lors de la création du VPS Hostinger, fournir cette clé).

> **CONFIDENTIEL :** La clé privée SSH ne doit **jamais** être commitée dans un dépôt public.

### Ports MWS (à ouvrir dans le pare-feu Hostinger et/ou ufw)

| Port | Protocole | Usage |
|------|-----------|-------|
| 22 | TCP | SSH (administration) |
| 80 | TCP | HTTP (catalogue web) |
| 443 | TCP | HTTPS (web + MiyukiniAdmin) |
| 7000 | TCP | Origin Relay |
| 21000 | TCP | Origin Tracker |

---

## 1. Architecture cible

```
┌──────────────────────────────────────────────────────────┐
│  Hostinger VPS — Debian 13                               │
│  IP publique : 46.202.129.65                             │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │               ORIGIN (MWS uniquement)              │  │
│  │────────────────────────────────────────────────────│  │
│  │  :7000   ── miyukini-origin (relay)                │  │
│  │  :21000  ── miyukini-origin (tracker)              │  │
│  │  :443    ── nginx (HTTPS → catalogue + admin)      │  │
│  │  :80     ── nginx (HTTP → redirect HTTPS)          │  │
│  │  :8080   ── catalogue web MWS (interne)            │  │
│  │  :8081   ── MiyukiniAdmin Origin (interne)         │  │
│  │                                                    │  │
│  │  Config  : /etc/miyukini/                          │  │
│  │  Données : /var/lib/miyukini/                      │  │
│  │  Logs    : /var/log/miyukini/                      │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

| Service | Port | Protocole | Processus |
|---------|------|-----------|-----------|
| **Origin Relay** | 7000 | TCP + TLS | `miyukini-origin` |
| **Origin Tracker** | 21000 | TCP + TLS | `miyukini-origin` |
| **Catalogue web** | 8080 | HTTP local | `miyukini-origin` |
| **MiyukiniAdmin Origin** | 8081 | HTTP local | `miyukini-origin` |
| **HTTPS** | 443 | HTTPS | `nginx` (TLS termination) |
| **HTTP** | 80 | HTTP | `nginx` (redirect) |
| **SSH** | 22 | TCP | `sshd` |

> **Règle :** Origin n'exécute **aucun** service hors périmètre MWS.

---

## 2. Connexion SSH

### Depuis Windows (PowerShell)

```powershell
ssh -i ssh-key-2026-02-12.key root@46.202.129.65
```

Ou : `ssh root@46.202.129.65` (si la clé est dans l'agent).

> Si SSH refuse la clé avec "permissions too open" :
> ```powershell
> icacls ssh-key-2026-02-12.key /inheritance:r /grant:r "%USERNAME%:R"
> ```

### Depuis Linux / macOS

```bash
chmod 600 ssh-key-2026-02-12.key
ssh -i ssh-key-2026-02-12.key root@46.202.129.65
```

### Vérification

```bash
# Une fois connecté :
cat /etc/os-release   # → Debian 13 (Trixie)
uname -a
```

---

## 3. Préparation du système (Debian 13)

> **Important :** Debian 13 utilise `apt` (pas `dnf`) et `ufw` (pas `firewalld`). Pas de SELinux par défaut.

### 3.1 Mise à jour du système

```bash
sudo apt update && sudo apt upgrade -y
```

### 3.2 Installation des dépendances

```bash
# Outils de compilation (Rust a besoin de gcc, libssl-dev, etc.)
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    git curl wget \
    tar gzip

# Nginx et Certbot
sudo apt install -y nginx certbot python3-certbot-nginx

# Argon2 (pour le hash du mot de passe MiyukiniAdmin)
sudo apt install -y argon2 || {
    echo "argon2 non disponible, compilation manuelle..."
    cd /tmp
    git clone https://github.com/P-H-C/phc-winner-argon2.git
    cd phc-winner-argon2
    make
    sudo make install
    cd ~
}
```

### 3.3 Création de l'utilisateur dédié

```bash
sudo useradd -r -s /sbin/nologin -m -d /var/lib/miyukini miyukini
```

### 3.4 Création de l'arborescence

```bash
sudo mkdir -p /etc/miyukini
sudo mkdir -p /var/log/miyukini
sudo mkdir -p /var/lib/miyukini/{registry,keys,policies}
sudo mkdir -p /opt/scripts

sudo chown -R miyukini:miyukini /var/lib/miyukini /var/log/miyukini
sudo chown -R miyukini:miyukini /etc/miyukini
```

---

## 4. Installation de Rust

```bash
# Installer rustup (en tant que ADMIN_USER, ex. root)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Charger l'environnement
source "$HOME/.cargo/env"

# Vérifier
rustc --version    # >= 1.70
cargo --version
```

---

## 5. Compilation et installation du binaire Origin

### 5.1 Cloner le dépôt

```bash
cd ~
git clone https://github.com/studiomiyukini/miyukini-webway-relay.git
cd miyukini-webway-relay
```

### 5.2 Compiler

```bash
cargo build --release
```

> Si la RAM est insuffisante, activer un swap temporaire :
> ```bash
> sudo dd if=/dev/zero of=/swapfile bs=1M count=2048
> sudo chmod 600 /swapfile && sudo mkswap /swapfile && sudo swapon /swapfile
> # Après compilation : sudo swapoff /swapfile && sudo rm /swapfile
> ```

### 5.3 Installer le binaire

```bash
sudo cp target/release/miyukini-origin /usr/local/bin/
sudo chmod +x /usr/local/bin/miyukini-origin

# Vérifier
miyukini-origin --version 2>/dev/null || echo "Binaire installé"
ls -la /usr/local/bin/miyukini-origin
```

---

## 6. Configuration d'Origin

### 6.1 Fichier de configuration principal

```bash
sudo tee /etc/miyukini/origin.toml > /dev/null << ORIGIN_TOML
# ═══════════════════════════════════════════════════════════
#  MWS Origin — Configuration
#  VPS  : Hostinger — Debian 13
#  IP   : 46.202.129.65
#  Rôle : Origin (relay + tracker + source de vérité)
# ═══════════════════════════════════════════════════════════

[identity]
role = "origin"
ip = "46.202.129.65"
# domain = "origin.miyukini.com"

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
key_path  = "/etc/miyukini/tls/origin.key"
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
enabled = true
difficulty_normal = 16
difficulty_attack = 22
challenge_ttl_seconds = 30

[admin]
host = "127.0.0.1"
port = 8081
config_file = "/etc/miyukini/admin.toml"

[logging]
level = "info"
relay_log   = "/var/log/miyukini/origin-relay.log"
tracker_log = "/var/log/miyukini/origin-tracker.log"
audit_log   = "/var/log/miyukini/origin-audit.log"
admin_log   = "/var/log/miyukini/origin-admin.log"

[limits]
max_connections = 10000
heartbeat_interval_seconds = 30
tunnel_timeout_seconds = 300
ORIGIN_TOML
```

### 6.2 Fichier de tokens

```bash
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
echo "CONSERVER CE TOKEN — nécessaire pour les premiers COGs."
```

### 6.3 Certificat TLS auto-signé (mode initial)

```bash
sudo mkdir -p /etc/miyukini/tls

sudo openssl req -x509 -nodes -days 365 \
    -newkey rsa:2048 \
    -keyout /etc/miyukini/tls/origin.key \
    -out /etc/miyukini/tls/origin.crt \
    -subj "/CN=46.202.129.65/O=Miyukini MWS/C=FR"

sudo chown miyukini:miyukini /etc/miyukini/tls/*
sudo chmod 600 /etc/miyukini/tls/origin.key
```

> Migrer vers Let's Encrypt dès que le domaine `origin.miyukini.com` est actif (certbot).

### 6.4 Certificat SSL wildcard Let's Encrypt (sous-domaines COG)

Pour activer les sous-domaines COG (`xxx.miyukini.com`), un certificat wildcard est requis. Let's Encrypt exige le **challenge DNS-01** (HTTP-01 ne fonctionne pas pour les wildcards).

**Prérequis :** Le DNS wildcard `*.miyukini.com` doit pointer vers l'IP du VPS (46.202.129.65).

**Étape 1 — Lancer certbot (sur le VPS ou en local) :**

```bash
sudo certbot certonly --manual --preferred-challenges dns \
  -d "miyukini.com" -d "*.miyukini.com"
```

**Étape 2 — Enregistrement TXT temporaire :**

Certbot affiche un enregistrement TXT à ajouter dans le DNS Hostinger :

- Type : `TXT`
- Nom : `_acme-challenge` ou `_acme-challenge.miyukini.com` (selon l'instruction certbot)
- Valeur : (chaîne fournie par certbot)
- TTL : 300 ou 3600

**Étape 3 — Propagation DNS :**

Attendre 2 à 10 minutes, vérifier la propagation :

```bash
dig TXT _acme-challenge.miyukini.com +short
```

**Étape 4 — Validation :**

Appuyer sur Entrée dans le terminal certbot pour lancer la vérification.

**Étape 5 — Chemins des certificats :**

Les certificats sont installés dans :

- `/etc/letsencrypt/live/miyukini.com/fullchain.pem`
- `/etc/letsencrypt/live/miyukini.com/privkey.pem`

**Étape 6 — Nginx :**

Mettre à jour la configuration nginx (voir section 8) pour utiliser ces chemins. La config de référence `origin-miyukini.conf` utilise déjà `/etc/letsencrypt/live/miyukini.com/`.

**Renouvellement (tous les 90 jours) :**

Sans plugin DNS Hostinger, le renouvellement est manuel :

```bash
sudo certbot renew --manual --preferred-challenges dns
```

Répéter l'ajout de l'enregistrement TXT dans Hostinger à chaque renouvellement. Pour automatiser, envisager un plugin DNS (ex. `certbot-dns-cloudflare`) ou un hook personnalisé.

---

## 7. Configuration MiyukiniAdmin Origin

### 7.1 Générer le hash du mot de passe

```bash
ADMIN_HASH=$(echo -n '!!REDACTED_PASSWORD!!' | argon2 $(openssl rand -base64 16) -id -m 16 -t 3 -p 4 -l 32 -e)
echo "Hash Argon2id : $ADMIN_HASH"
```

### 7.2 Créer admin.toml

```bash
sudo tee /etc/miyukini/admin.toml > /dev/null << ADMIN_TOML
# MiyukiniAdmin Origin — Configuration (CONFIDENTIEL)

[admin]
email = "miyukini@gmail.com"
password_hash = "$ADMIN_HASH"

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
```

### 7.3 Générer la clé JWT

```bash
openssl rand -base64 64 | sudo tee /etc/miyukini/admin_jwt.key > /dev/null
```

### 7.4 Sécuriser les fichiers

```bash
sudo chown -R miyukini:miyukini /etc/miyukini/
sudo chmod 600 /etc/miyukini/origin.toml
sudo chmod 600 /etc/miyukini/tokens.json
sudo chmod 600 /etc/miyukini/admin.toml
sudo chmod 600 /etc/miyukini/admin_jwt.key
```

---

## 8. Configuration Nginx (reverse proxy)

**Debian utilise `sites-available` / `sites-enabled`.**

Pour la config complète incluant les sous-domaines COG (`*.miyukini.com`), voir `docs/doc_for_website/origin-miyukini.conf` à la racine du dépôt.

> **Après le certificat wildcard (section 6.4)** : remplacer les chemins `ssl_certificate` par `/etc/letsencrypt/live/miyukini.com/fullchain.pem` et `privkey.pem`, puis `sudo nginx -t && sudo systemctl reload nginx`.

```bash
sudo tee /etc/nginx/sites-available/origin-miyukini.conf > /dev/null << NGINX_CONF
# Nginx — Origin Miyukini MWS (Hostinger VPS — 46.202.129.65)

limit_req_zone \$binary_remote_addr zone=admin_login:10m rate=5r/m;

server {
    listen 80;
    server_name 46.202.129.65 origin.miyukini.com *.miyukini.com;

    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }

    location / {
        return 301 https://\$host\$request_uri;
    }
}

server {
    listen 443 ssl http2;
    server_name 46.202.129.65 origin.miyukini.com;

    ssl_certificate     /etc/miyukini/tls/origin.crt;
    ssl_certificate_key /etc/miyukini/tls/origin.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5:!RC4:!3DES;
    ssl_prefer_server_ciphers on;

    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains" always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-Frame-Options DENY always;

    location /admin {
        proxy_pass http://127.0.0.1:8081;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        limit_req zone=admin_login burst=5 nodelay;
        add_header Content-Security-Policy "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'" always;
        add_header Referrer-Policy no-referrer always;
    }

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
NGINX_CONF

sudo ln -sf /etc/nginx/sites-available/origin-miyukini.conf /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default

sudo mkdir -p /var/www/html
sudo nginx -t
sudo systemctl enable nginx
sudo systemctl start nginx
```

---

## 9. Service systemd Origin

```bash
sudo tee /etc/systemd/system/miyukini-origin.service > /dev/null << 'SYSTEMD_UNIT'
[Unit]
Description=Miyukini Webway Origin (relay + tracker + source de vérité + admin)
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=miyukini
Group=miyukini
WorkingDirectory=/var/lib/miyukini
ExecStart=/usr/local/bin/miyukini-origin --config /etc/miyukini/origin.toml
Restart=always
RestartSec=5
LimitNOFILE=65535

NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ReadWritePaths=/var/lib/miyukini /var/log/miyukini

StandardOutput=journal
StandardError=journal
SyslogIdentifier=miyukini-origin

[Install]
WantedBy=multi-user.target
SYSTEMD_UNIT

sudo systemctl daemon-reload
sudo systemctl enable miyukini-origin
sudo systemctl start miyukini-origin
sudo systemctl status miyukini-origin
```

---

## 10. Pare-feu (ufw)

Debian 13 utilise **ufw** (pas firewalld). Si ufw n'est pas installé, l'installer d'abord : `apt-get update && apt-get install -y ufw`.

```bash
# Autoriser SSH d'abord pour ne pas se déconnecter
ufw allow 22/tcp
ufw allow 80/tcp
ufw allow 443/tcp
ufw allow 7000/tcp
ufw allow 21000/tcp

ufw --force enable
ufw status numbered
```

**Depuis ta machine (Windows)** : tu peux aussi lancer le script fourni (installe ufw si besoin, puis ouvre les ports) :

```powershell
# À la racine du repo (avec clé dans l'agent ou -i si besoin)
Get-Content .\scripts\vps-ufw-open-ports.sh -Raw | ssh root@46.202.129.65 "bash -s"
```

---

## 11. Hardening système

### 11.1 SYN cookies et limites TCP

```bash
sudo tee /etc/sysctl.d/99-miyukini.conf > /dev/null << 'SYSCTL'
net.ipv4.tcp_syncookies = 1
net.ipv4.tcp_max_syn_backlog = 2048
net.core.somaxconn = 2048
SYSCTL

sudo sysctl --system
```

### 11.2 NTP (contremesure R-006)

Debian 13 utilise **systemd-timesyncd** par défaut.

```bash
sudo systemctl enable systemd-timesyncd
sudo systemctl start systemd-timesyncd
timedatectl status   # → "System clock synchronized: yes"
```

---

## 12. Rotation des logs

```bash
sudo tee /etc/logrotate.d/miyukini > /dev/null << 'LOGROTATE'
/var/log/miyukini/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0640 miyukini miyukini
    postrotate
        systemctl reload miyukini-origin 2>/dev/null || true
    endscript
}
LOGROTATE
```

---

## 13. Script de sauvegarde

```bash
sudo tee /opt/scripts/backup-origin.sh > /dev/null << 'BACKUP_SCRIPT'
#!/bin/bash
BACKUP_DIR="/root/backups/$(date +%Y-%m-%d)"
mkdir -p "$BACKUP_DIR"

sudo tar czf "$BACKUP_DIR/miyukini-data.tar.gz" \
    /var/lib/miyukini/ \
    /etc/miyukini/ \
    --exclude='*.log'

echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] Sauvegarde terminée : $BACKUP_DIR"
BACKUP_SCRIPT

sudo chmod +x /opt/scripts/backup-origin.sh

# Cron quotidien à 3h
(sudo crontab -l 2>/dev/null; echo "0 3 * * * /opt/scripts/backup-origin.sh >> /var/log/miyukini/backup.log 2>&1") | sudo crontab -
```

---

## 14. Manifeste Origin signé

Voir [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md). Générer les clés de l'autorité MWS sur une machine sécurisée, calculer le pin TLS depuis le certificat du VPS, puis signer le manifeste avec l'IP/URL canonique du VPS Hostinger.

---

## 15. Tests de validation

### 15.1 Checklist sur le VPS

```bash
echo "=== Checklist Origin (Debian 13) ==="
echo -n "1. OS : " && cat /etc/os-release | grep PRETTY_NAME
echo -n "2. Binaire : " && ls /usr/local/bin/miyukini-origin && echo "OK" || echo "FAIL"
echo -n "3. Service Origin : " && systemctl is-active miyukini-origin
echo -n "4. Nginx : " && systemctl is-active nginx
echo -n "5. UFW : " && sudo ufw status | head -5
echo "6. Ports : " && sudo ss -tlnp | grep -E ':(7000|21000|8080|8081|80|443) '
echo "=== Fin checklist ==="
```

### 15.2 Tests depuis une machine distante

```bash
ssh -i ssh-key-2026-02-12.key root@46.202.129.65 "echo OK"
nc -zv 46.202.129.65 7000
nc -zv 46.202.129.65 21000
curl -I http://46.202.129.65/
curl -kI https://46.202.129.65/
curl -kI https://46.202.129.65/admin
openssl s_client -connect 46.202.129.65:7000 -tls1_2
```

---

## 16. Récapitulatif

| Élément | Valeur |
|---------|--------|
| **OS** | Debian 13 (Trixie) |
| **IP publique** | `46.202.129.65` |
| **Utilisateur SSH** | `root` ou compte dédié |
| **Clé SSH** | `ssh-key-2026-02-12.key` |
| **Port relay** | 7000 |
| **Port tracker** | 21000 |
| **Port web** | 80 → redirect / 443 HTTPS |
| **MiyukiniAdmin** | `https://46.202.129.65/admin` (port interne 8081) |
| **Binaire** | `/usr/local/bin/miyukini-origin` |
| **Config** | `/etc/miyukini/origin.toml`, `admin.toml`, `tokens.json` |
| **TLS** | `/etc/miyukini/tls/` (auto-signé → Let's Encrypt si DNS) |
| **Données** | `/var/lib/miyukini/` |
| **Logs** | `/var/log/miyukini/` |
| **Utilisateur système** | `miyukini` (nologin) |
| **Package manager** | `apt` |
| **Firewall** | `ufw` |
| **NTP** | `systemd-timesyncd` |
| **Scope** | **MWS uniquement** |

---

## Références

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](../acteurs/MWS%20-%20Origin.md)
- [MWS - MiyukiniAdmin Origin](../administration/MWS%20-%20MiyukiniAdmin.md)
- [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md)
- [MWS - Guide de Déploiement](./MWS%20-%20Guide%20de%20Deploiement.md)
- [Miyukini - Hostinger VPS Origin Webway](../../setup/Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md)

---

**Version :** 1.0  
**Mise à jour :** Implémentation Origin sur Hostinger VPS (Debian 13), migration depuis Oracle Cloud  
**Classification :** Documentation MWS — Déploiement
