# MWS â€” ImplÃ©mentation Origin sur Oracle Cloud

> **DÃ©prÃ©ciÃ© â€” Migration fÃ©vrier 2026 :** L'hÃ©bergement Origin a migrÃ© vers **Hostinger VPS (Debian 13)**. Utiliser [MWS - ImplÃ©mentation Origin Hostinger](./MWS%20-%20Implementation%20Origin%20Hostinger.md) et [Miyukini - Hostinger VPS Origin Webway](..//setup//Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md). Ce document est conservÃ© pour archive.

## Contexte

Ce document Ã©tait le **guide d'implÃ©mentation complet et exÃ©cutable** d'Origin sur la VM Oracle Cloud. Un agent IA ou un opÃ©rateur humain doit pouvoir suivre ce guide **de A Ã  Z** et obtenir un Origin fonctionnel.

Origin est le point central de vÃ©ritÃ© du MWS : il cumule les fonctions **relay** (vÃ©rification de conformitÃ©), **tracker** (pools, catalogue, connexions) et **source de vÃ©ritÃ©** (Registre de Services, versions, politiques).

**RÃ©fÃ©rences :**
- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](../acteurs/MWS%20-%20Origin.md)
- [MWS - MiyukiniAdmin Origin](../administration/MWS%20-%20MiyukiniAdmin.md)
- [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md)

---

## 0. Fiche d'identitÃ© de l'instance (vÃ©rifiÃ©e)

Ces informations proviennent de la console Oracle Cloud (captures du 13/02/2026).

| ParamÃ¨tre | Valeur exacte |
|-----------|---------------|
| **Provider** | Oracle Cloud Infrastructure (OCI) |
| **RÃ©gion** | France South (Marseille) â€” `eu-marseille-1` |
| **Domaine de disponibilitÃ©** | AD-1 |
| **Domaine de pannes** | FD-2 |
| **Compartiment** | `studiomiyukini` (racine) |
| **OS / Image** | **Oracle Linux 9.7** â€” `Oracle-Linux-9.7-2026.01.29-0` |
| **Forme** | `VM.Standard.E2.1.Micro` (Always Free) |
| **OCPU** | 1 |
| **RAM** | 1 Go |
| **Bande passante** | 0.5 Gbits/s |
| **Stockage** | Stockage de blocs uniquement (boot volume) |
| **Microprogramme** | UEFI_64 |
| **LancÃ©e** | 12 fÃ©vrier 2026, 21:06:20 UTC |
| **IP publique** | `84.235.227.152` |
| **IP privÃ©e** | `10.0.0.110` |
| **Utilisateur SSH** | **`opc`** (utilisateur par dÃ©faut Oracle Linux) |
| **VCN** | `origin-miyukini-webway` |
| **Sous-rÃ©seau** | `webway-0.1` |
| **Groupe de sÃ©curitÃ©** | `ig-quick-action-NSG` |
| **Nom d'hÃ´te** | `origin-miyukini-webway-interface` |
| **FQDN interne** | `origin-miyukini-webway-interface.subnet02122206.vcn02122206.oraclevcn.com` |
| **Cryptage en transit** | ActivÃ© |
| **Disaster Recovery** | Non activÃ© |

### ClÃ© SSH

| Fichier | Chemin dans le workspace |
|---------|--------------------------|
| **ClÃ© privÃ©e** | `ssh-key-2026-02-12.key` |
| **ClÃ© publique** | `ssh-key-2026-02-12.key.pub` |

**ClÃ© publique de rÃ©fÃ©rence (Ã  conserver sur tout hÃ©bergeur) :**

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIVTDqC5yyNd6Ir9/NLjzUTT1IhugyRyRCo6+O5LZC4Z miyukini@gmail.com
```

Ã€ placer dans `~/.ssh/authorized_keys` du compte admin sur lâ€™instance (Oracle Linux, Debian, Ubuntu, etc.).

> **CONFIDENTIEL :** La clÃ© privÃ©e SSH ne doit **jamais** Ãªtre commitÃ©e dans un dÃ©pÃ´t public. Elle est prÃ©sente dans le workspace pour permettre aux agents IA d'opÃ©rer sur la VM.

### Ports MWS (dÃ©jÃ  ouverts dans le Security Group OCI)

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
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Oracle Cloud â€” eu-marseille-1 (AD-1 / FD-2)            â”‚
â”‚  Oracle Linux 9.7 â€” VM.Standard.E2.1.Micro              â”‚
â”‚  IP publique : 84.235.227.152 / privÃ©e : 10.0.0.110     â”‚
â”‚                                                          â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚               ORIGIN (MWS uniquement)              â”‚  â”‚
â”‚  â”‚â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚  â”‚
â”‚  â”‚  :7000   â”€â”€ miyukini-origin (relay)                â”‚  â”‚
â”‚  â”‚  :21000  â”€â”€ miyukini-origin (tracker)              â”‚  â”‚
â”‚  â”‚  :443    â”€â”€ nginx (HTTPS â†’ catalogue + admin)      â”‚  â”‚
â”‚  â”‚  :80     â”€â”€ nginx (HTTP â†’ redirect HTTPS)          â”‚  â”‚
â”‚  â”‚  :8080   â”€â”€ catalogue web MWS (interne)            â”‚  â”‚
â”‚  â”‚  :8081   â”€â”€ MiyukiniAdmin Origin (interne)         â”‚  â”‚
â”‚  â”‚                                                    â”‚  â”‚
â”‚  â”‚  Config  : /etc/miyukini/                          â”‚  â”‚
â”‚  â”‚  DonnÃ©es : /var/lib/miyukini/                      â”‚  â”‚
â”‚  â”‚  Logs    : /var/log/miyukini/                      â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
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

> **RÃ¨gle :** Origin n'exÃ©cute **aucun** service hors pÃ©rimÃ¨tre MWS.

---

## 2. Connexion SSH

### Depuis Windows (machine de dev)

```powershell
# PowerShell â€” depuis la racine du workspace Miyukini_COG
ssh -i ssh-key-2026-02-12.key opc@84.235.227.152
```

> Si SSH refuse la clÃ© avec "permissions too open", exÃ©cuter d'abord :
>
> ```powershell
> icacls ssh-key-2026-02-12.key /inheritance:r /grant:r "%USERNAME%:R"
> ```

### Depuis Linux / macOS

```bash
chmod 600 ssh-key-2026-02-12.key
ssh -i ssh-key-2026-02-12.key opc@84.235.227.152
```

### VÃ©rification

```bash
# Une fois connectÃ© en tant que opc :
cat /etc/oracle-release
# â†’ Oracle Linux Server release 9.7
uname -a
# â†’ Linux origin-miyukini-webway-interface ...
```

---

## 3. PrÃ©paration du systÃ¨me (Oracle Linux 9.7)

> **Important :** Oracle Linux 9.7 utilise `dnf` (pas `apt`) et `firewalld` (pas `ufw`).

### 3.1 Mise Ã  jour du systÃ¨me

```bash
sudo dnf update -y
```

### 3.2 Installation des dÃ©pendances

```bash
# Outils de compilation (Rust a besoin de gcc, openssl-devel, etc.)
sudo dnf install -y \
    gcc gcc-c++ make \
    pkg-config \
    openssl openssl-devel \
    git curl wget \
    tar gzip

# EPEL (nÃ©cessaire pour nginx et certains outils)
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

### 3.3 CrÃ©ation de l'utilisateur dÃ©diÃ©

```bash
sudo useradd -r -s /sbin/nologin -m -d /var/lib/miyukini miyukini
```

### 3.4 CrÃ©ation de l'arborescence

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

# VÃ©rifier
rustc --version    # >= 1.70
cargo --version
```

---

## 5. Compilation et installation du binaire Origin

### 5.1 Cloner le dÃ©pÃ´t

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
> # AprÃ¨s la compilation : sudo swapoff /swapfile && sudo rm /swapfile
> ```

### 5.3 Installer le binaire

```bash
sudo cp target/release/miyukini-origin /usr/local/bin/
sudo chmod +x /usr/local/bin/miyukini-origin

# VÃ©rifier
miyukini-origin --version 2>/dev/null || echo "Binaire installÃ© (pas de --version si non implÃ©mentÃ©)"
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
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  MWS Origin â€” Configuration
#  VM    : Oracle Cloud â€” Oracle Linux 9.7 â€” eu-marseille-1
#  IP    : 84.235.227.152 (publique) / 10.0.0.110 (privÃ©e)
#  RÃ´le  : Origin (relay + tracker + source de vÃ©ritÃ©)
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

[identity]
role = "origin"
ip = "84.235.227.152"
# Domaine (activer quand le DNS est en place)
# domain = "origin.miyukini.com"

# â”€â”€â”€ Relay â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[relay]
host = "0.0.0.0"
port = 7000

# â”€â”€â”€ Tracker â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[tracker]
host = "0.0.0.0"
port = 21000
web_port = 8080   # catalogue interne (nginx proxy 80/443 â†’ 8080)

[tracker.pools]
enable_version_isolation = true

[tracker.lobbys]
max_lobbys_per_cog = 10
password_max_attempts = 3     # Contremesure R-011

# â”€â”€â”€ TLS â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[tls]
# Mode initial : certificat auto-signÃ© (Let's Encrypt quand DNS en place)
cert_path = "/etc/miyukini/tls/origin.crt"
key_path  = "/etc/miyukini/tls/origin.key"
min_version = "1.2"

# â”€â”€â”€ Authentification â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[auth]
token_file = "/etc/miyukini/tokens.json"
token_rotation_days = 7       # Contremesure R-007

# â”€â”€â”€ Registre de Services â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[registry]
data_dir = "/var/lib/miyukini/registry"

# â”€â”€â”€ ClÃ©s de conformitÃ© â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[cores]
keys_dir = "/var/lib/miyukini/keys"

# â”€â”€â”€ Politiques â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[policies]
data_dir = "/var/lib/miyukini/policies"
quarantine_escalation = [3600, 7200]   # 1h, 2h puis blacklist
timestamp_window_seconds = 10          # Contremesure R-006

# â”€â”€â”€ Rate limiting (Contremesure R-002) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[rate_limits]
register_per_minute_per_ip = 10
connections_per_token = 100
requests_per_hour_per_cog = 1000
tcp_connections_per_ip = 5000

# â”€â”€â”€ Proof of Work (Contremesure R-002) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[pow]
enabled = true
difficulty_normal = 16
difficulty_attack = 22
challenge_ttl_seconds = 30

# â”€â”€â”€ MiyukiniAdmin Origin â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[admin]
host = "127.0.0.1"
port = 8081
config_file = "/etc/miyukini/admin.toml"

# â”€â”€â”€ Journalisation â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[logging]
level = "info"
relay_log   = "/var/log/miyukini/origin-relay.log"
tracker_log = "/var/log/miyukini/origin-tracker.log"
audit_log   = "/var/log/miyukini/origin-audit.log"
admin_log   = "/var/log/miyukini/origin-admin.log"

# â”€â”€â”€ Limites â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
[limits]
max_connections = 10000
heartbeat_interval_seconds = 30
tunnel_timeout_seconds = 300
ORIGIN_TOML
```

### 6.2 Fichier de tokens

```bash
# GÃ©nÃ©rer un token de 256 bits
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

echo "Token gÃ©nÃ©rÃ© : $ORIGIN_TOKEN"
echo "CONSERVER CE TOKEN â€” il sera nÃ©cessaire pour les premiers COGs."
```

### 6.3 Certificat TLS auto-signÃ© (mode initial)

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

> Migrer vers Let's Encrypt dÃ¨s que le domaine `origin.miyukini.com` est actif :
> ```bash
> sudo systemctl stop nginx
> sudo certbot certonly --standalone -d origin.miyukini.com
> # Puis modifier origin.toml : cert_path/key_path â†’ /etc/letsencrypt/live/...
> sudo systemctl start nginx
> sudo systemctl restart miyukini-origin
> ```

---

## 7. Configuration MiyukiniAdmin Origin

### 7.1 GÃ©nÃ©rer le hash du mot de passe

```bash
# GÃ©nÃ©rer le hash Argon2id
ADMIN_HASH=$(echo -n '!!REDACTED_PASSWORD!!' | argon2 $(openssl rand -base64 16) -id -m 16 -t 3 -p 4 -l 32 -e)
echo "Hash Argon2id : $ADMIN_HASH"
```

### 7.2 CrÃ©er le fichier admin.toml

```bash
sudo tee /etc/miyukini/admin.toml > /dev/null << ADMIN_TOML
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  MiyukiniAdmin Origin â€” Configuration
#  CONFIDENTIEL â€” ne pas versionner
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

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

### 7.3 GÃ©nÃ©rer la clÃ© JWT

```bash
openssl rand -base64 64 | sudo tee /etc/miyukini/admin_jwt.key > /dev/null
```

### 7.4 SÃ©curiser les fichiers de configuration

```bash
sudo chown -R miyukini:miyukini /etc/miyukini/
sudo chmod 600 /etc/miyukini/origin.toml
sudo chmod 600 /etc/miyukini/tokens.json
sudo chmod 600 /etc/miyukini/admin.toml
sudo chmod 600 /etc/miyukini/admin_jwt.key
```

---

## 8. Configuration Nginx (reverse proxy)

### 8.1 CrÃ©er la configuration

```bash
sudo tee /etc/nginx/conf.d/origin-miyukini.conf > /dev/null << 'NGINX_CONF'
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  Nginx â€” Origin Miyukini MWS
#  IP : 84.235.227.152
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

# Rate limiting pour MiyukiniAdmin login
limit_req_zone $binary_remote_addr zone=admin_login:10m rate=5r/m;

# HTTP â†’ HTTPS redirect
server {
    listen 80;
    server_name 84.235.227.152 origin.miyukini.com;

    # Certbot challenge
    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }

    # Tout le reste â†’ HTTPS
    location / {
        return 301 https://$host$request_uri;
    }
}

# HTTPS
server {
    listen 443 ssl http2;
    server_name 84.235.227.152 origin.miyukini.com;

    # Certificat (auto-signÃ© initialement, Let's Encrypt ensuite)
    ssl_certificate     /etc/miyukini/tls/origin.crt;
    ssl_certificate_key /etc/miyukini/tls/origin.key;

    # TLS hardening
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5:!RC4:!3DES;
    ssl_prefer_server_ciphers on;

    # Headers de sÃ©curitÃ©
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains" always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-Frame-Options DENY always;

    # â”€â”€â”€ MiyukiniAdmin Origin (/admin) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
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

    # â”€â”€â”€ Catalogue web MWS (/) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
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

### 8.2 Supprimer la config par dÃ©faut et activer

```bash
# Supprimer ou renommer le bloc server par dÃ©faut
sudo mv /etc/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf.bak 2>/dev/null || true

# CrÃ©er le rÃ©pertoire pour Certbot
sudo mkdir -p /var/www/html

# Tester la configuration
sudo nginx -t

# Activer et dÃ©marrer
sudo systemctl enable nginx
sudo systemctl start nginx
```

> **Note :** Sur Oracle Linux 9, nginx utilise `/etc/nginx/conf.d/*.conf` (pas `sites-available/sites-enabled`).

---

## 9. Service systemd Origin

### 9.1 CrÃ©er le fichier de service

```bash
sudo tee /etc/systemd/system/miyukini-origin.service > /dev/null << 'SYSTEMD_UNIT'
[Unit]
Description=Miyukini Webway Origin (relay + tracker + source de vÃ©ritÃ© + admin)
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

# SÃ©curitÃ©
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

### 9.2 Activer et dÃ©marrer

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
# VÃ©rifier que firewalld est actif
sudo systemctl status firewalld

# Ouvrir les ports MWS
sudo firewall-cmd --permanent --add-port=80/tcp
sudo firewall-cmd --permanent --add-port=443/tcp
sudo firewall-cmd --permanent --add-port=7000/tcp
sudo firewall-cmd --permanent --add-port=21000/tcp

# Appliquer
sudo firewall-cmd --reload

# VÃ©rifier
sudo firewall-cmd --list-ports
# â†’ 80/tcp 443/tcp 7000/tcp 21000/tcp
```

---

## 11. Hardening systÃ¨me

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
# Oracle Linux 9 utilise chrony par dÃ©faut
sudo systemctl enable chronyd
sudo systemctl start chronyd

# VÃ©rifier la synchronisation
chronyc tracking
# â†’ "Leap status : Normal" = OK
timedatectl
# â†’ "NTP service: active" ou "System clock synchronized: yes"
```

### 11.3 SELinux

Oracle Linux 9 a SELinux activÃ©. Si nginx ou miyukini-origin rencontre des refus :

```bash
# VÃ©rifier le statut
getenforce
# â†’ Enforcing

# Autoriser nginx Ã  faire du proxy rÃ©seau
sudo setsebool -P httpd_can_network_connect 1

# Si miyukini-origin Ã©coute sur des ports non standard, autoriser :
sudo semanage port -a -t http_port_t -p tcp 7000
sudo semanage port -a -t http_port_t -p tcp 8080
sudo semanage port -a -t http_port_t -p tcp 8081
sudo semanage port -a -t http_port_t -p tcp 21000
```

> Si `semanage` n'est pas installÃ© : `sudo dnf install -y policycoreutils-python-utils`

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
# Sauvegarde quotidienne des donnÃ©es Origin
BACKUP_DIR="/home/opc/backups/$(date +%Y-%m-%d)"
mkdir -p "$BACKUP_DIR"

sudo tar czf "$BACKUP_DIR/miyukini-data.tar.gz" \
    /var/lib/miyukini/ \
    /etc/miyukini/ \
    --exclude='*.log'

echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] Sauvegarde terminÃ©e : $BACKUP_DIR"
BACKUP_SCRIPT

sudo chmod +x /opt/scripts/backup-origin.sh

# Cron quotidien Ã  3h du matin
(crontab -l 2>/dev/null; echo "0 3 * * * /opt/scripts/backup-origin.sh >> /var/log/miyukini/backup.log 2>&1") | crontab -
```

---

## 14. Manifeste Origin signÃ©

### 14.1 GÃ©nÃ©rer les clÃ©s de l'autoritÃ© MWS

```bash
# Sur la machine locale (PAS sur la VM en production)
openssl genpkey -algorithm Ed25519 -out mws_authority.key
openssl pkey -in mws_authority.key -pubout -out mws_authority.pub

# GARDER mws_authority.key en lieu trÃ¨s sÃ»r (hors ligne, chiffrÃ©)
```

### 14.2 Calculer le pin TLS

```bash
# Sur la VM (avec le certificat actuel)
openssl x509 -in /etc/miyukini/tls/origin.crt \
    -pubkey -noout | \
    openssl pkey -pubin -outform DER | \
    openssl dgst -sha256 -binary | base64
```

### 14.3 CrÃ©er et signer le manifeste

Voir [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md) pour le format complet et la procÃ©dure de signature.

---

## 15. Tests de validation

### 15.1 Checklist de dÃ©ploiement

ExÃ©cuter depuis la VM (`opc@84.235.227.152`) :

```bash
echo "=== Checklist Origin ==="

echo -n "1. OS Oracle Linux 9 : "
cat /etc/oracle-release

echo -n "2. Binaire installÃ© : "
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

echo "10. DonnÃ©es :"
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

# Test HTTP â†’ HTTPS redirect
curl -I http://84.235.227.152/

# Test HTTPS (accepte le certificat auto-signÃ©)
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

## 16. RÃ©capitulatif

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **OS** | Oracle Linux 9.7 |
| **IP publique** | `84.235.227.152` |
| **IP privÃ©e** | `10.0.0.110` |
| **Utilisateur SSH** | `opc` |
| **ClÃ© SSH** | `ssh-key-2026-02-12.key` |
| **Forme** | VM.Standard.E2.1.Micro (1 OCPU, 1 Go, 0.5 Gbits/s) |
| **VCN** | `origin-miyukini-webway` |
| **Sous-rÃ©seau** | `webway-0.1` |
| **Port relay** | 7000 |
| **Port tracker** | 21000 |
| **Port web** | 80 â†’ redirect / 443 HTTPS |
| **MiyukiniAdmin** | `https://84.235.227.152/admin` (port interne 8081) |
| **Binaire** | `/usr/local/bin/miyukini-origin` |
| **Config Origin** | `/etc/miyukini/origin.toml` |
| **Config Admin** | `/etc/miyukini/admin.toml` |
| **Tokens** | `/etc/miyukini/tokens.json` |
| **TLS** | `/etc/miyukini/tls/` (auto-signÃ©) â†’ Let's Encrypt quand DNS prÃªt |
| **DonnÃ©es** | `/var/lib/miyukini/` |
| **Logs** | `/var/log/miyukini/` |
| **Utilisateur systÃ¨me** | `miyukini` (non-root, nologin) |
| **Package manager** | `dnf` |
| **Firewall** | `firewalld` |
| **NTP** | `chronyd` |
| **SELinux** | Enforcing (avec `httpd_can_network_connect=on`) |
| **Scope** | **MWS uniquement** |

---

## 17. ProcÃ©dure rÃ©sumÃ©e (commandes sÃ©quentielles)

Pour un agent IA, voici la sÃ©quence complÃ¨te Ã  exÃ©cuter **dans l'ordre** aprÃ¨s connexion SSH :

```bash
# â”€â”€ 1. MISE Ã€ JOUR â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo dnf update -y

# â”€â”€ 2. DÃ‰PENDANCES â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo dnf install -y gcc gcc-c++ make pkg-config openssl openssl-devel git curl wget tar gzip
sudo dnf install -y oracle-epel-release-el9
sudo dnf install -y nginx certbot python3-certbot-nginx policycoreutils-python-utils
sudo dnf install -y argon2 || true

# â”€â”€ 3. UTILISATEUR â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo useradd -r -s /sbin/nologin -m -d /var/lib/miyukini miyukini 2>/dev/null || true

# â”€â”€ 4. ARBORESCENCE â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo mkdir -p /etc/miyukini /var/log/miyukini /var/lib/miyukini/{registry,keys,policies} /opt/scripts
sudo chown -R miyukini:miyukini /var/lib/miyukini /var/log/miyukini /etc/miyukini

# â”€â”€ 5. RUST â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# â”€â”€ 6. SWAP (si nÃ©cessaire pour compiler) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo dd if=/dev/zero of=/swapfile bs=1M count=2048
sudo chmod 600 /swapfile && sudo mkswap /swapfile && sudo swapon /swapfile

# â”€â”€ 7. CLONER ET COMPILER â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
cd ~ && git clone https://github.com/studiomiyukini/miyukini-webway-relay.git
cd miyukini-webway-relay && cargo build --release

# â”€â”€ 8. INSTALLER LE BINAIRE â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo cp target/release/miyukini-origin /usr/local/bin/
sudo chmod +x /usr/local/bin/miyukini-origin

# â”€â”€ 9. SUPPRIMER LE SWAP â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo swapoff /swapfile && sudo rm /swapfile

# â”€â”€ 10. CRÃ‰ER LES CONFIGS â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# (ExÃ©cuter les blocs des sections 6.1, 6.2, 6.3, 7.1-7.4)

# â”€â”€ 11. NGINX â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# (ExÃ©cuter les blocs de la section 8)

# â”€â”€ 12. SYSTEMD â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# (ExÃ©cuter les blocs de la section 9)

# â”€â”€ 13. FIREWALL â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo firewall-cmd --permanent --add-port=80/tcp
sudo firewall-cmd --permanent --add-port=443/tcp
sudo firewall-cmd --permanent --add-port=7000/tcp
sudo firewall-cmd --permanent --add-port=21000/tcp
sudo firewall-cmd --reload

# â”€â”€ 14. SELINUX â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo setsebool -P httpd_can_network_connect 1
sudo semanage port -a -t http_port_t -p tcp 7000 2>/dev/null || true
sudo semanage port -a -t http_port_t -p tcp 8080 2>/dev/null || true
sudo semanage port -a -t http_port_t -p tcp 8081 2>/dev/null || true
sudo semanage port -a -t http_port_t -p tcp 21000 2>/dev/null || true

# â”€â”€ 15. HARDENING â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
# (ExÃ©cuter les blocs de la section 11)

# â”€â”€ 16. DÃ‰MARRER â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
sudo systemctl daemon-reload
sudo systemctl enable --now miyukini-origin
sudo systemctl enable --now nginx
sudo systemctl enable chronyd
```

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](../acteurs/MWS%20-%20Origin.md)
- [MWS - MiyukiniAdmin Origin](../administration/MWS%20-%20MiyukiniAdmin.md)
- [MWS - Manifeste Origin et Adresse Canonique](../securite/MWS%20-%20Manifeste%20Origin%20et%20Adresse%20Canonique.md)
- [MWS - Chiffrement et TLS](../securite/MWS%20-%20Chiffrement%20et%20TLS.md)
- [MWS - Protection DDoS](../securite/MWS%20-%20Protection%20DDoS.md)
- [MWS - Guide de DÃ©ploiement](./MWS%20-%20Guide%20de%20Deploiement.md)

---

**Version :** 3.0  
**Mise Ã  jour :** Oracle Linux 9.7 (dnf, firewalld, chronyd, SELinux), clÃ© SSH rÃ©elle, fiche d'identitÃ© vÃ©rifiÃ©e, procÃ©dure exÃ©cutable  
**Classification :** Documentation MWS â€” DÃ©ploiement / ImplÃ©mentation

