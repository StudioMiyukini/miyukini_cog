---
name: miyukini-deployment
description: Deploiement et infrastructure de Miyukini COG. Couvre le deploiement VPS Linux (Debian/Oracle) pour Origin, le setup TLS (rustls, certificats auto-signes), la configuration firewall (UFW), le fichier origin.toml, l'installeur Windows (Inno Setup), les scripts de build, les ports reseau (7000 relay, 21000 tracker, 8080 web). Utiliser quand on deploie Origin sur un VPS, quand on configure TLS/certificats, quand on prepare une distribution Windows, quand on modifie les ports ou la configuration reseau, ou quand on travaille sur les scripts de deploiement.
---

# Deploiement & Infrastructure

## Cibles de deploiement

| Cible | Plateforme | Composant | Mode |
|-------|-----------|-----------|------|
| VPS Linux | Debian 13 / Oracle 9 | Origin (Relay + Tracker + Web) | Serveur |
| Desktop Windows | Windows 10+ | Miyukini Central | App native (Inno Setup) |
| Dev local | Windows/Linux/macOS | Tous | Cargo build |

---

## 1. Origin — Deploiement VPS Linux

### Ports reseau

| Port | Service | Protocole | Description |
|------|---------|-----------|-------------|
| 22 | SSH | TCP | Administration |
| 80 | HTTP | TCP | Redirect HTTPS ou site |
| 443 | HTTPS | TCP | Site web + API |
| 7000 | Relay | TCP+TLS | Sessions COG, tunnels |
| 21000 | Tracker | TCP | Pools, catalogues, lobbys |

### Configuration origin.toml

```toml
[relay]
port = 7000

[tracker]
port = 21000
web_port = 8080
max_lobbies_per_cog = 10
max_password_attempts = 3

[security]
token_rotation_days = 7

[rate_limiting]
max_registrations_per_minute_per_ip = 10
max_requests_per_hour_per_cog = 1000
```

### Script de setup — Debian (Hostinger)

**Fichier** : `scripts/setup-origin-hostinger.sh`
**Cible** : IP 46.202.129.65

12 phases :
1. Update systeme + outils de base
2. Creation utilisateur `miyukini` (systeme, sans shell interactif)
3. Arborescence `/opt/miyukini-origin/{bin,config,data,logs,certs}`
4. Generation certificat TLS auto-signe (ed25519)
5. Configuration permissions (0o700/0o600)
6. Installation Rust toolchain (rustup)
7. Compilation Origin en release
8. Copie du binaire dans `/opt/miyukini-origin/bin/`
9. Creation du fichier origin.toml
10. Creation du service systemd `miyukini-origin`
11. Configuration firewall (UFW)
12. Demarrage du service

### Script de setup — Oracle Linux

**Fichier** : `setup-origin.sh` (racine)
**Cible** : IP 84.235.227.152

15 phases (plus robuste) :
- Gestion des locks DNF
- Repos EPEL
- Certificat avec SubjectAltName (DNS + IP)

### Certificats TLS

```bash
# Generation certificat auto-signe (ed25519)
openssl req -x509 -newkey ed25519 -keyout key.pem -out cert.pem \
    -days 365 -nodes \
    -subj "/CN=miyukini-origin/O=StudioMiyukini" \
    -addext "subjectAltName=DNS:miyukini-origin,IP:$SERVER_IP"

# Permissions
chmod 600 key.pem cert.pem
chown miyukini:miyukini key.pem cert.pem
```

**Stack TLS** : rustls (pas OpenSSL) + tokio-rustls

### Service systemd

```ini
[Unit]
Description=Miyukini Origin Server
After=network.target

[Service]
Type=simple
User=miyukini
Group=miyukini
WorkingDirectory=/opt/miyukini-origin
ExecStart=/opt/miyukini-origin/bin/miyukini-origin
Restart=on-failure
RestartSec=5
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

### Firewall UFW

```bash
# scripts/vps-ufw-open-ports.sh
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp    # SSH
ufw allow 80/tcp    # HTTP
ufw allow 443/tcp   # HTTPS
ufw allow 7000/tcp  # Relay
ufw allow 21000/tcp # Tracker
ufw --force enable
```

**Deploiement distant** :
```bash
cat scripts/vps-ufw-open-ports.sh | ssh root@$IP "bash -s"
```

---

## 2. Desktop Windows — Inno Setup

**Fichier** : `installer/miyukini-central.iss`

### Configuration

```iss
[Setup]
AppId={{MIYUKINI-CENTRAL-GUID}}
AppName=Miyukini Central
AppVersion=0.1.0
DefaultDirName={localappdata}\Miyukini-COG  ; AppData (pas Program Files)
Compression=lzma2/ultra64
SolidCompression=yes
SetupMutexes=MiyukiniCentralSetup           ; Empeche installations concurrentes
```

### Structure installee

```
{localappdata}\Miyukini-COG\
├── miyukini-central.exe
├── WebView2Loader.dll
├── voices/
│   └── fr/
│       ├── miou_happy.wav
│       ├── miou_neutral.wav
│       └── ...
└── legal/
    ├── LICENCE_COG.txt
    └── LICENCE_COMPOSANTS_TIERS.txt
```

### Pourquoi AppData ?

`{localappdata}` au lieu de `{pf}` car :
- Pas besoin de droits admin pour installer
- Ecriture directe dans le dossier d'installation
- Conforme aux pratiques modernes (VS Code, Discord, etc.)

---

## 3. Build et Distribution

### Build release

```bash
# Origin (Linux)
cargo build -p miyukini-origin --release

# Central (Windows, via Tauri)
cd apps/central && pnpm tauri:build

# Clippy avant release
cargo clippy --workspace 2> clippy_output.txt
```

### MGE build separee

```powershell
# build-allumina.ps1
cd mge
cargo build --release
```

### Distribution publique

```
public-dist/
├── miyukini-cog-public-v0.1.0.zip
└── (archives distribuables)
```

---

## 4. Arborescence serveur

```
/opt/miyukini-origin/
├── bin/
│   └── miyukini-origin     # Binaire compile
├── config/
│   └── origin.toml         # Configuration
├── data/
│   └── (bases SQLite/SQLCipher)
├── logs/
│   └── (journaux)
├── certs/
│   ├── cert.pem            # Certificat TLS
│   └── key.pem             # Cle privee
└── backups/
    └── (sauvegardes auto)
```

---

## 5. Checklist de deploiement Origin

1. [ ] Serveur VPS accessible via SSH
2. [ ] Script de setup execute (setup-origin-hostinger.sh ou setup-origin.sh)
3. [ ] Certificat TLS genere et permissions 600
4. [ ] origin.toml configure (ports, limites)
5. [ ] Binaire compile en release et copie
6. [ ] Service systemd cree et enable
7. [ ] Firewall UFW configure (ports 22, 80, 443, 7000, 21000)
8. [ ] Service demarre : `systemctl start miyukini-origin`
9. [ ] Verification : `systemctl status miyukini-origin`
10. [ ] Test connexion relay : `telnet $IP 7000`
11. [ ] Test connexion tracker : `telnet $IP 21000`
