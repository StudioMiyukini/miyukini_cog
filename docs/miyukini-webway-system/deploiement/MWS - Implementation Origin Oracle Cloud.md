# MWS — Implémentation Origin sur Oracle Cloud

> **Déprécié — Migration février 2026 :** L'hébergement Origin a migré vers **Hostinger VPS (Debian 13)**. Utiliser [MWS - Implémentation Origin Hostinger](./MWS%20-%20Implementation%20Origin%20Hostinger.md) et [Miyukini - Hostinger VPS Origin Webway](../../setup/Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md). Ce document est conservé pour archive.

## Contexte

Ce document était le **guide d'implémentation complet et exécutable** d'Origin sur la VM Oracle Cloud. Un agent IA ou un opérateur humain doit pouvoir suivre ce guide **de A à Z** et obtenir un Origin fonctionnel.

Origin est le point central de vérité du MWS : il cumule les fonctions **relay** (vérification de conformité), **tracker** (pools, catalogue, connexions) et **source de vérité** (Registre de Services, versions, politiques).

**Références :**
- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](../acteurs/MWS%20-%20Origin.md)
- [MWS - MiyukiniAdmin Origin](../administration/MWS%20-%20MiyukiniAdmin.md)
- [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md)

---

## 0. Fiche d'identité de l'instance (vérifiée)

Ces informations proviennent de la console Oracle Cloud (captures du 13/02/2026).

| Paramètre | Valeur exacte |
|-----------|---------------|
| **Provider** | Oracle Cloud Infrastructure (OCI) |
| **Région** | France South (Marseille) — `eu-marseille-1` |
| **Domaine de disponibilité** | AD-1 |
| **Domaine de pannes** | FD-2 |
| **Compartiment** | `studiomiyukini` (racine) |
| **OS / Image** | **Oracle Linux 9.7** — `Oracle-Linux-9.7-2026.01.29-0` |
| **Forme** | `VM.Standard.E2.1.Micro` (Always Free) |
| **OCPU** | 1 |
| **RAM** | 1 Go |
| **Bande passante** | 0.5 Gbits/s |
| **Stockage** | Stockage de blocs uniquement (boot volume) |
| **Microprogramme** | UEFI_64 |
| **Lancée** | 12 février 2026, 21:06:20 UTC |
| **IP publique** | `84.235.227.152` |
| **IP privée** | `10.0.0.110` |
| **Utilisateur SSH** | **`opc`** (utilisateur par défaut Oracle Linux) |
| **VCN** | `origin-miyukini-webway` |
| **Sous-réseau** | `webway-0.1` |
| **Groupe de sécurité** | `ig-quick-action-NSG` |
| **Nom d'hôte** | `origin-miyukini-webway-interface` |
| **FQDN interne** | `origin-miyukini-webway-interface.subnet02122206.vcn02122206.oraclevcn.com` |
| **Cryptage en transit** | Activé |
| **Disaster Recovery** | Non activé |

### Clé SSH

| Fichier | Chemin dans le workspace |
|---------|--------------------------|
| **Clé privée** | `ssh-key-2026-02-12.key` |
| **Clé publique** | `ssh-key-2026-02-12.key.pub` |

**Clé publique de référence (à conserver sur tout hébergeur) :**

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIVTDqC5yyNd6Ir9/NLjzUTT1IhugyRyRCo6+O5LZC4Z miyukini@gmail.com
```

À placer dans `~/.ssh/authorized_keys` du compte admin sur l’instance (Oracle Linux, Debian, Ubuntu, etc.).

> **CONFIDENTIEL :** La clé privée SSH ne doit **jamais** être commitée dans un dépôt public. Elle est présente dans le workspace pour permettre aux agents IA d'opérer sur la VM.

### Ports MWS (déjà ouverts dans le Security Group OCI)

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
│  Oracle Cloud — eu-marseille-1 (AD-1 / FD-2)            │
│  Oracle Linux 9.7 — VM.Standard.E2.1.Micro              │
│  IP publique : 84.235.227.152 / privée : 10.0.0.110     │
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

### Depuis Windows (machine de dev)

```powershell
# PowerShell — depuis la racine du workspace Miyukini_COG
ssh -i ssh-key-2026-02-12.key opc@84.235.227.152
```

> Si SSH refuse la clé avec "permissions too open", exécuter d'abord :
>
> ```powershell
> icacls ssh-key-2026-02-12.key /inheritance:r /grant:r "%USERNAME%:R"
> ```

### Depuis Linux / macOS

```bash
chmod 600 ssh-key-2026-02-12.key
ssh -i ssh-key-2026-02-12.key opc@84.235.227.152
```

### Vérification

```bash
# Une fois connecté en tant que opc :
cat /etc/oracle-release
# → Oracle Linux Server release 9.7
uname -a
# → Linux origin-miyukini-webway-interface ...
```

---

## 3. Préparation du système (Oracle Linux 9.7)

> **Important :** Oracle Linux 9.7 utilise `dnf` (pas `apt`) et `firewalld` (pas `ufw`).

### 3.1 Mise à jour du système

```bash
sudo dnf update -y
```

### 3.2 Installation des dépendances

```bash
# Outils de compilation (Rust a besoin de gcc, openssl-devel, etc.)
sudo dnf install -y \
    gcc gcc-c++ make \
    pkg-config \
    openssl openssl-devel \
    git curl wget \
    tar gzip

# EPEL (nécessaire pour nginx et certains outils)
sudo dnf install -y oracle-epel-release-el9
sudo dnf install -y nginx certbot python3-certbot-nginx

# Argon2 (pour le hash du mot de passe MiyukiniAdmin)
# Si argon2 n'est pas disponible dans les repos, compiler :
sudo dnf install -y argon2 || {
    echo "argon2 non disponible dans les repos, compilation manuelle..."
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
# Installer rustup (en tant que opc)
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

> **Note :** Sur la VM.Standard.E2.1.Micro (1 OCPU, 1 Go RAM), la compilation peut prendre **15-30 minutes**. Si la RAM est insuffisante, activer un swap temporaire :
>
> ```bash
> sudo dd if=/dev/zero of=/swapfile bs=1M count=2048
> sudo chmod 600 /swapfile
> sudo mkswap /swapfile
> sudo swapon /swapfile
> # Après la compilation : sudo swapoff /swapfile && sudo rm /swapfile
> ```

### 5.3 Installer le binaire

```bash
sudo cp target/release/miyukini-origin /usr/local/bin/
sudo chmod +x /usr/local/bin/miyukini-origin

# Vérifier
miyukini-origin --version 2>/dev/null || echo "Binaire installé (pas de --version si non implémenté)"
ls -la /usr/local/bin/miyukini-origin
```

> Si le binaire s'appelle autrement (ex. `miyuwebway_relay`), adapter :
> ```bash
> sudo cp target/release/miyuwebway_relay /usr/local/bin/miyukini-origin
> ```

---

## 6. Configuration d'Origin

### 6.1 Fichier de configuration principal

```bash
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
# Domaine (activer quand le DNS est en place)
# domain = "origin.miyukini.com"

# ─── Relay ────────────────────────────────────────────────
[relay]
host = "0.0.0.0"
port = 7000

# ─── Tracker ─────────────────────────────────────────────
[tracker]
host = "0.0.0.0"
port = 21000
web_port = 8080   # catalogue interne (nginx proxy 80/443 → 8080)

[tracker.pools]
enable_version_isolation = true

[tracker.lobbys]
max_lobbys_per_cog = 10
password_max_attempts = 3     # Contremesure R-011

# ─── TLS ──────────────────────────────────────────────────
[tls]
# Mode initial : certificat auto-signé (Let's Encrypt quand DNS en place)
cert_path = "/etc/miyukini/tls/origin.crt"
key_path  = "/etc/miyukini/tls/origin.key"
min_version = "1.2"

# ─── Authentification ─────────────────────────────────────
[auth]
token_file = "/etc/miyukini/tokens.json"
token_rotation_days = 7       # Contremesure R-007

# ─── Registre de Services ─────────────────────────────────
[registry]
data_dir = "/var/lib/miyukini/registry"

# ─── Clés de conformité ──────────────────────────────────
[cores]
keys_dir = "/var/lib/miyukini/keys"

# ─── Politiques ───────────────────────────────────────────
[policies]
data_dir = "/var/lib/miyukini/policies"
quarantine_escalation = [3600, 7200]   # 1h, 2h puis blacklist
timestamp_window_seconds = 10          # Contremesure R-006

# ─── Rate limiting (Contremesure R-002) ───────────────────
[rate_limits]
register_per_minute_per_ip = 10
connections_per_token = 100
requests_per_hour_per_cog = 1000
tcp_connections_per_ip = 5000

# ─── Proof of Work (Contremesure R-002) ───────────────────
[pow]
enabled = true
difficulty_normal = 16
difficulty_attack = 22
challenge_ttl_seconds = 30

# ─── MiyukiniAdmin Origin ────────────────────────────────
[admin]
host = "127.0.0.1"
port = 8081
config_file = "/etc/miyukini/admin.toml"

# ─── Journalisation ──────────────────────────────────────
[logging]
level = "info"
relay_log   = "/var/log/miyukini/origin-relay.log"
tracker_log = "/var/log/miyukini/origin-tracker.log"
audit_log   = "/var/log/miyukini/origin-audit.log"
admin_log   = "/var/log/miyukini/origin-admin.log"

# ─── Limites ─────────────────────────────────────────────
[limits]
max_connections = 10000
heartbeat_interval_seconds = 30
tunnel_timeout_seconds = 300
ORIGIN_TOML
```

### 6.2 Fichier de tokens

```bash
# Générer un token de 256 bits
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
```

### 6.3 Certificat TLS auto-signé (mode initial)

```bash
sudo mkdir -p /etc/miyukini/tls

sudo openssl req -x509 -nodes -days 365 \
    -newkey rsa:2048 \
    -keyout /etc/miyukini/tls/origin.key \
    -out /etc/miyukini/tls/origin.crt \
    -subj "/CN=84.235.227.152/O=Miyukini MWS/C=FR"

sudo chown miyukini:miyukini /etc/miyukini/tls/*
sudo chmod 600 /etc/miyukini/tls/origin.key
```

> Migrer vers Let's Encrypt dès que le domaine `origin.miyukini.com` est actif :
> ```bash
> sudo systemctl stop nginx
> sudo certbot certonly --standalone -d origin.miyukini.com
> # Puis modifier origin.toml : cert_path/key_path → /etc/letsencrypt/live/...
> sudo systemctl start nginx
> sudo systemctl restart miyukini-origin
> ```

---

## 7. Configuration MiyukiniAdmin Origin

### 7.1 Générer le hash du mot de passe

```bash
# Générer le hash Argon2id
ADMIN_HASH=$(echo -n '!!REDACTED_PASSWORD!!' | argon2 $(openssl rand -base64 16) -id -m 16 -t 3 -p 4 -l 32 -e)
echo "Hash Argon2id : $ADMIN_HASH"
```

### 7.2 Créer le fichier admin.toml

```bash
sudo tee /etc/miyukini/admin.toml > /dev/null << ADMIN_TOML
# ═══════════════════════════════════════════════════
#  MiyukiniAdmin Origin — Configuration
#  CONFIDENTIEL — ne pas versionner
# ═══════════════════════════════════════════════════

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

### 7.4 Sécuriser les fichiers de configuration

```bash
sudo chown -R miyukini:miyukini /etc/miyukini/
sudo chmod 600 /etc/miyukini/origin.toml
sudo chmod 600 /etc/miyukini/tokens.json
sudo chmod 600 /etc/miyukini/admin.toml
sudo chmod 600 /etc/miyukini/admin_jwt.key
```

---

## 8. Configuration Nginx (reverse proxy)

### 8.1 Créer la configuration

```bash
sudo tee /etc/nginx/conf.d/origin-miyukini.conf > /dev/null << 'NGINX_CONF'
# ═══════════════════════════════════════════════════
#  Nginx — Origin Miyukini MWS
#  IP : 84.235.227.152
# ═══════════════════════════════════════════════════

# Rate limiting pour MiyukiniAdmin login
limit_req_zone $binary_remote_addr zone=admin_login:10m rate=5r/m;

# HTTP → HTTPS redirect
server {
    listen 80;
    server_name 84.235.227.152 origin.miyukini.com;

    # Certbot challenge
    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }

    # Tout le reste → HTTPS
    location / {
        return 301 https://$host$request_uri;
    }
}

# HTTPS
server {
    listen 443 ssl http2;
    server_name 84.235.227.152 origin.miyukini.com;

    # Certificat (auto-signé initialement, Let's Encrypt ensuite)
    ssl_certificate     /etc/miyukini/tls/origin.crt;
    ssl_certificate_key /etc/miyukini/tls/origin.key;

    # TLS hardening
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5:!RC4:!3DES;
    ssl_prefer_server_ciphers on;

    # Headers de sécurité
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains" always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-Frame-Options DENY always;

    # ─── MiyukiniAdmin Origin (/admin) ────────────
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

    # ─── Catalogue web MWS (/) ────────────────────
    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
NGINX_CONF
```

### 8.2 Supprimer la config par défaut et activer

```bash
# Supprimer ou renommer le bloc server par défaut
sudo mv /etc/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf.bak 2>/dev/null || true

# Créer le répertoire pour Certbot
sudo mkdir -p /var/www/html

# Tester la configuration
sudo nginx -t

# Activer et démarrer
sudo systemctl enable nginx
sudo systemctl start nginx
```

> **Note :** Sur Oracle Linux 9, nginx utilise `/etc/nginx/conf.d/*.conf` (pas `sites-available/sites-enabled`).

---

## 9. Service systemd Origin

### 9.1 Créer le fichier de service

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

# Sécurité
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
```

### 9.2 Activer et démarrer

```bash
sudo systemctl daemon-reload
sudo systemctl enable miyukini-origin
sudo systemctl start miyukini-origin
sudo systemctl status miyukini-origin
```

---

## 10. Pare-feu OS (firewalld)

Oracle Linux 9 utilise **firewalld** (pas iptables directement, pas ufw).

```bash
# Vérifier que firewalld est actif
sudo systemctl status firewalld

# Ouvrir les ports MWS
sudo firewall-cmd --permanent --add-port=80/tcp
sudo firewall-cmd --permanent --add-port=443/tcp
sudo firewall-cmd --permanent --add-port=7000/tcp
sudo firewall-cmd --permanent --add-port=21000/tcp

# Appliquer
sudo firewall-cmd --reload

# Vérifier
sudo firewall-cmd --list-ports
# → 80/tcp 443/tcp 7000/tcp 21000/tcp
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

```bash
# Oracle Linux 9 utilise chrony par défaut
sudo systemctl enable chronyd
sudo systemctl start chronyd

# Vérifier la synchronisation
chronyc tracking
# → "Leap status : Normal" = OK
timedatectl
# → "NTP service: active" ou "System clock synchronized: yes"
```

### 11.3 SELinux

Oracle Linux 9 a SELinux activé. Si nginx ou miyukini-origin rencontre des refus :

```bash
# Vérifier le statut
getenforce
# → Enforcing

# Autoriser nginx à faire du proxy réseau
sudo setsebool -P httpd_can_network_connect 1

# Si miyukini-origin écoute sur des ports non standard, autoriser :
sudo semanage port -a -t http_port_t -p tcp 7000
sudo semanage port -a -t http_port_t -p tcp 8080
sudo semanage port -a -t http_port_t -p tcp 8081
sudo semanage port -a -t http_port_t -p tcp 21000
```

> Si `semanage` n'est pas installé : `sudo dnf install -y policycoreutils-python-utils`

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
# Sauvegarde quotidienne des données Origin
BACKUP_DIR="/home/opc/backups/$(date +%Y-%m-%d)"
mkdir -p "$BACKUP_DIR"

sudo tar czf "$BACKUP_DIR/miyukini-data.tar.gz" \
    /var/lib/miyukini/ \
    /etc/miyukini/ \
    --exclude='*.log'

echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] Sauvegarde terminée : $BACKUP_DIR"
BACKUP_SCRIPT

sudo chmod +x /opt/scripts/backup-origin.sh

# Cron quotidien à 3h du matin
(crontab -l 2>/dev/null; echo "0 3 * * * /opt/scripts/backup-origin.sh >> /var/log/miyukini/backup.log 2>&1") | crontab -
```

---

## 14. Manifeste Origin signé

### 14.1 Générer les clés de l'autorité MWS

```bash
# Sur la machine locale (PAS sur la VM en production)
openssl genpkey -algorithm Ed25519 -out mws_authority.key
openssl pkey -in mws_authority.key -pubout -out mws_authority.pub

# GARDER mws_authority.key en lieu très sûr (hors ligne, chiffré)
```

### 14.2 Calculer le pin TLS

```bash
# Sur la VM (avec le certificat actuel)
openssl x509 -in /etc/miyukini/tls/origin.crt \
    -pubkey -noout | \
    openssl pkey -pubin -outform DER | \
    openssl dgst -sha256 -binary | base64
```

### 14.3 Créer et signer le manifeste

Voir [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md) pour le format complet et la procédure de signature.

---

## 15. Tests de validation

### 15.1 Checklist de déploiement

Exécuter depuis la VM (`opc@84.235.227.152`) :

```bash
echo "=== Checklist Origin ==="

echo -n "1. OS Oracle Linux 9 : "
cat /etc/oracle-release

echo -n "2. Binaire installé : "
ls -la /usr/local/bin/miyukini-origin && echo "OK" || echo "FAIL"

echo -n "3. Service Origin : "
systemctl is-active miyukini-origin

echo -n "4. Nginx : "
systemctl is-active nginx

echo -n "5. Firewalld : "
sudo firewall-cmd --list-ports

echo -n "6. NTP sync : "
chronyc tracking | grep "Leap status"

echo -n "7. SELinux httpd_can_network_connect : "
getsebool httpd_can_network_connect

echo "8. Ports locaux :"
sudo ss -tlnp | grep -E ':(7000|21000|8080|8081|80|443) '

echo "9. Config files :"
ls -la /etc/miyukini/origin.toml /etc/miyukini/admin.toml /etc/miyukini/tokens.json

echo "10. Données :"
ls -la /var/lib/miyukini/

echo "=== Fin checklist ==="
```

### 15.2 Tests depuis une machine distante

```bash
# Test SSH
ssh -i ssh-key-2026-02-12.key opc@84.235.227.152 "echo OK"

# Test port 7000 (relay)
nc -zv 84.235.227.152 7000

# Test port 21000 (tracker)
nc -zv 84.235.227.152 21000

# Test HTTP → HTTPS redirect
curl -I http://84.235.227.152/

# Test HTTPS (accepte le certificat auto-signé)
curl -kI https://84.235.227.152/

# Test MiyukiniAdmin
curl -kI https://84.235.227.152/admin

# Test TLS relay
openssl s_client -connect 84.235.227.152:7000 -tls1_2
```

### 15.3 Test fonctionnel REGISTER

```bash
./miyukini-client test-register \
    --origin 84.235.227.152:7000 \
    --token <TOKEN_GENERE_EN_6.2> \
    --cog-id test-cog-001 \
    --cog-type STABLE \
    --os-type LINUX
```

---

## 16. Récapitulatif

| Élément | Valeur |
|---------|--------|
| **OS** | Oracle Linux 9.7 |
| **IP publique** | `84.235.227.152` |
| **IP privée** | `10.0.0.110` |
| **Utilisateur SSH** | `opc` |
| **Clé SSH** | `ssh-key-2026-02-12.key` |
| **Forme** | VM.Standard.E2.1.Micro (1 OCPU, 1 Go, 0.5 Gbits/s) |
| **VCN** | `origin-miyukini-webway` |
| **Sous-réseau** | `webway-0.1` |
| **Port relay** | 7000 |
| **Port tracker** | 21000 |
| **Port web** | 80 → redirect / 443 HTTPS |
| **MiyukiniAdmin** | `https://84.235.227.152/admin` (port interne 8081) |
| **Binaire** | `/usr/local/bin/miyukini-origin` |
| **Config Origin** | `/etc/miyukini/origin.toml` |
| **Config Admin** | `/etc/miyukini/admin.toml` |
| **Tokens** | `/etc/miyukini/tokens.json` |
| **TLS** | `/etc/miyukini/tls/` (auto-signé) → Let's Encrypt quand DNS prêt |
| **Données** | `/var/lib/miyukini/` |
| **Logs** | `/var/log/miyukini/` |
| **Utilisateur système** | `miyukini` (non-root, nologin) |
| **Package manager** | `dnf` |
| **Firewall** | `firewalld` |
| **NTP** | `chronyd` |
| **SELinux** | Enforcing (avec `httpd_can_network_connect=on`) |
| **Scope** | **MWS uniquement** |

---

## 17. Procédure résumée (commandes séquentielles)

Pour un agent IA, voici la séquence complète à exécuter **dans l'ordre** après connexion SSH :

```bash
# ── 1. MISE À JOUR ──────────────────────────────────────
sudo dnf update -y

# ── 2. DÉPENDANCES ──────────────────────────────────────
sudo dnf install -y gcc gcc-c++ make pkg-config openssl openssl-devel git curl wget tar gzip
sudo dnf install -y oracle-epel-release-el9
sudo dnf install -y nginx certbot python3-certbot-nginx policycoreutils-python-utils
sudo dnf install -y argon2 || true

# ── 3. UTILISATEUR ──────────────────────────────────────
sudo useradd -r -s /sbin/nologin -m -d /var/lib/miyukini miyukini 2>/dev/null || true

# ── 4. ARBORESCENCE ─────────────────────────────────────
sudo mkdir -p /etc/miyukini /var/log/miyukini /var/lib/miyukini/{registry,keys,policies} /opt/scripts
sudo chown -R miyukini:miyukini /var/lib/miyukini /var/log/miyukini /etc/miyukini

# ── 5. RUST ─────────────────────────────────────────────
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# ── 6. SWAP (si nécessaire pour compiler) ───────────────
sudo dd if=/dev/zero of=/swapfile bs=1M count=2048
sudo chmod 600 /swapfile && sudo mkswap /swapfile && sudo swapon /swapfile

# ── 7. CLONER ET COMPILER ──────────────────────────────
cd ~ && git clone https://github.com/studiomiyukini/miyukini-webway-relay.git
cd miyukini-webway-relay && cargo build --release

# ── 8. INSTALLER LE BINAIRE ────────────────────────────
sudo cp target/release/miyukini-origin /usr/local/bin/
sudo chmod +x /usr/local/bin/miyukini-origin

# ── 9. SUPPRIMER LE SWAP ───────────────────────────────
sudo swapoff /swapfile && sudo rm /swapfile

# ── 10. CRÉER LES CONFIGS ──────────────────────────────
# (Exécuter les blocs des sections 6.1, 6.2, 6.3, 7.1-7.4)

# ── 11. NGINX ───────────────────────────────────────────
# (Exécuter les blocs de la section 8)

# ── 12. SYSTEMD ─────────────────────────────────────────
# (Exécuter les blocs de la section 9)

# ── 13. FIREWALL ────────────────────────────────────────
sudo firewall-cmd --permanent --add-port=80/tcp
sudo firewall-cmd --permanent --add-port=443/tcp
sudo firewall-cmd --permanent --add-port=7000/tcp
sudo firewall-cmd --permanent --add-port=21000/tcp
sudo firewall-cmd --reload

# ── 14. SELINUX ─────────────────────────────────────────
sudo setsebool -P httpd_can_network_connect 1
sudo semanage port -a -t http_port_t -p tcp 7000 2>/dev/null || true
sudo semanage port -a -t http_port_t -p tcp 8080 2>/dev/null || true
sudo semanage port -a -t http_port_t -p tcp 8081 2>/dev/null || true
sudo semanage port -a -t http_port_t -p tcp 21000 2>/dev/null || true

# ── 15. HARDENING ───────────────────────────────────────
# (Exécuter les blocs de la section 11)

# ── 16. DÉMARRER ────────────────────────────────────────
sudo systemctl daemon-reload
sudo systemctl enable --now miyukini-origin
sudo systemctl enable --now nginx
sudo systemctl enable chronyd
```

---

## Références

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](../acteurs/MWS%20-%20Origin.md)
- [MWS - MiyukiniAdmin Origin](../administration/MWS%20-%20MiyukiniAdmin.md)
- [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md)
- [MWS - Chiffrement et TLS](../securite/MWS%20-%20Chiffrement%20et%20TLS.md)
- [MWS - Protection DDoS](../securite/MWS%20-%20Protection%20DDoS.md)
- [MWS - Guide de Déploiement](./MWS%20-%20Guide%20de%20Deploiement.md)

---

**Version :** 3.0  
**Mise à jour :** Oracle Linux 9.7 (dnf, firewalld, chronyd, SELinux), clé SSH réelle, fiche d'identité vérifiée, procédure exécutable  
**Classification :** Documentation MWS — Déploiement / Implémentation
