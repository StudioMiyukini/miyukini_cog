# MWS â€” Guide de DÃ©ploiement

## Contexte

Ce document est un **guide condensÃ©** pour le dÃ©ploiement des composants MWS : **relay**, **tracker** et **services web**. Il synthÃ©tise les Ã©tapes essentielles et renvoie aux guides dÃ©taillÃ©s pour les configurations avancÃ©es.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- PrÃ©requis et architecture de dÃ©ploiement
- DÃ©ploiement d'un relay
- DÃ©ploiement d'un tracker
- Configuration TLS et certificats
- Systemd et supervision
- Monitoring et journalisation
- SÃ©curitÃ© et pare-feu

Pour les guides complets, voir :
- [Miyukini - Webway Relay Deployment Guide](..//setup//Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md)
- [Miyukini - Hostinger VPS Origin Webway](..//setup//Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md)
- [MWS - ImplÃ©mentation Origin Hostinger](./MWS%20-%20Implementation%20Origin%20Hostinger.md)

---

## 1. Architecture de dÃ©ploiement

### 1.1 Composants

```
+------------------+     +------------------+     +------------------+
|      Origin      |     |      Relay       |     |     Tracker      |
|------------------|     |------------------|     |------------------|
| Port 7000 (relay)|     | Port 7000 (relay)|     | Port 21000 (MWS) |
| Port 21000 (MWS) |     | Port 80/443 (web)|     | Port 80 (catalog)|
| Port 80/443 (web)|     +------------------+     +------------------+
+------------------+
```

### 1.2 Topologie recommandÃ©e

| Environnement | Configuration |
|---------------|---------------|
| **DÃ©veloppement** | Un seul serveur (Origin tout-en-un) |
| **Production** | Origin + plusieurs relays + plusieurs trackers distribuÃ©s |
| **Haute disponibilitÃ©** | Load balancer + multiples instances par rÃ´le |

### 1.3 PrÃ©requis

| Composant | Exigence |
|-----------|----------|
| **OS** | Linux (Debian 13, Ubuntu 22.04+, Oracle Linux 9+) |
| **Rust** | 1.70+ (pour compiler les binaires) |
| **RAM** | Minimum 1 Go (4 Go recommandÃ©) |
| **Stockage** | 10 Go minimum |
| **RÃ©seau** | IP publique ou accÃ¨s via NAT/reverse proxy |

---

## 2. DÃ©ploiement d'un Relay

### 2.1 Installation

```bash
# Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Cloner le dÃ©pÃ´t
git clone https://github.com/studiomiyukini/miyukini-webway-relay.git
cd miyukini-webway-relay

# Compiler
cargo build --release

# Installer le binaire
sudo cp target/release/miyukini-relay /usr/local/bin/
```

### 2.2 Configuration

CrÃ©er `/etc/miyukini/relay.toml` :

```toml
[server]
host = "0.0.0.0"
port = 7000

[tls]
cert_path = "/etc/letsencrypt/live/webway.example.com/fullchain.pem"
key_path = "/etc/letsencrypt/live/webway.example.com/privkey.pem"

[auth]
token_file = "/etc/miyukini/tokens.json"

[origin]
url = "https://origin.miyukini.com:7000"
sync_interval_seconds = 60

[logging]
level = "info"
file = "/var/log/miyukini/relay.log"

[limits]
max_connections = 10000
rate_limit_per_source = 100
heartbeat_interval_seconds = 30
tunnel_timeout_seconds = 300

# Contremesure R-002 â€” seuils dÃ©taillÃ©s (voir MWS - Protection DDoS)
[rate_limits]
register_per_minute_per_ip = 10
connections_per_token = 100
requests_per_hour_per_cog = 1000
tcp_connections_per_ip = 5000
```

### 2.3 Certificat TLS

```bash
# Installer Certbot
sudo apt install certbot

# Obtenir un certificat Let's Encrypt
sudo certbot certonly --standalone -d webway.example.com

# Renouvellement automatique
sudo systemctl enable certbot.timer
```

### 2.4 Service systemd

CrÃ©er `/etc/systemd/system/miyukini-relay.service` :

```ini
[Unit]
Description=Miyukini Webway Relay
After=network.target

[Service]
Type=simple
User=miyukini
Group=miyukini
ExecStart=/usr/local/bin/miyukini-relay --config /etc/miyukini/relay.toml
Restart=always
RestartSec=5
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

```bash
# Activer et dÃ©marrer
sudo systemctl daemon-reload
sudo systemctl enable miyukini-relay
sudo systemctl start miyukini-relay
```

---

## 3. DÃ©ploiement d'un Tracker

### 3.1 Installation

```bash
# Compiler le tracker
cd miyukini-webway-tracker
cargo build --release

# Installer
sudo cp target/release/miyukini-tracker /usr/local/bin/
```

### 3.2 Configuration

CrÃ©er `/etc/miyukini/tracker.toml` :

```toml
[server]
mws_port = 21000
web_port = 80

[tls]
cert_path = "/etc/letsencrypt/live/tracker.example.com/fullchain.pem"
key_path = "/etc/letsencrypt/live/tracker.example.com/privkey.pem"

[origin]
url = "https://origin.miyukini.com:7000"
sync_interval_seconds = 30

[pools]
# Pools sÃ©parÃ©s par version majeure des Cores
enable_version_isolation = true

[lobbys]
max_lobbys_per_cog = 10
password_max_attempts = 5

[logging]
level = "info"
file = "/var/log/miyukini/tracker.log"
```

### 3.3 Service systemd

CrÃ©er `/etc/systemd/system/miyukini-tracker.service` :

```ini
[Unit]
Description=Miyukini Webway Tracker
After=network.target

[Service]
Type=simple
User=miyukini
Group=miyukini
ExecStart=/usr/local/bin/miyukini-tracker --config /etc/miyukini/tracker.toml
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

---

## 4. Configuration du pare-feu

### 4.1 Ports Ã  ouvrir

| Port | Protocole | Usage |
|------|-----------|-------|
| 7000 | TCP | Protocole relay |
| 21000 | TCP | Protocole MWS (tracker) |
| 80 | TCP | HTTP (catalogue web) |
| 443 | TCP | HTTPS (services web) |

### 4.2 UFW (Ubuntu)

```bash
sudo ufw allow 7000/tcp comment "Miyukini Relay"
sudo ufw allow 21000/tcp comment "Miyukini Tracker MWS"
sudo ufw allow 80/tcp comment "HTTP"
sudo ufw allow 443/tcp comment "HTTPS"
sudo ufw enable
```

### 4.3 Hostinger VPS (Debian 13)

```bash
# Sur le VPS (ufw)
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 7000/tcp
sudo ufw allow 21000/tcp
sudo ufw --force enable
```

Dans le panneau Hostinger, s'assurer que les ports 22, 80, 443, 7000, 21000 sont autorisÃ©s pour le VPS.

---

## 5. Monitoring et journalisation

### 5.1 Logs

| Fichier | Contenu |
|---------|---------|
| `/var/log/miyukini/relay.log` | Logs du relay |
| `/var/log/miyukini/tracker.log` | Logs du tracker |
| `/var/log/miyukini/audit.log` | Ã‰vÃ©nements de sÃ©curitÃ© |

### 5.2 Rotation des logs

CrÃ©er `/etc/logrotate.d/miyukini` :

```
/var/log/miyukini/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0640 miyukini miyukini
    postrotate
        systemctl reload miyukini-relay 2>/dev/null || true
        systemctl reload miyukini-tracker 2>/dev/null || true
    endscript
}
```

### 5.3 MÃ©triques

| MÃ©trique | Description |
|----------|-------------|
| `connections_active` | Connexions actives |
| `tunnels_registered` | Tunnels enregistrÃ©s |
| `verifications_total` | VÃ©rifications effectuÃ©es |
| `verifications_failed` | VÃ©rifications Ã©chouÃ©es |
| `quarantines_active` | COGs en quarantaine |

### 5.4 Alertes recommandÃ©es

| Condition | Seuil | Action |
|-----------|-------|--------|
| CPU > 80% | 5 minutes | Alerte |
| RAM > 90% | 5 minutes | Alerte |
| Connexions > 8000 | ImmÃ©diat | Alerte + scaling |
| Taux d'Ã©chec > 10% | 1 minute | Alerte critique |

---

## 6. SÃ©curitÃ©

### 6.1 Checklist de sÃ©curitÃ©

| Ã‰lÃ©ment | VÃ©rifiÃ© |
|---------|---------|
| TLS 1.2+ activÃ© | â˜ |
| Certificat valide | â˜ |
| Pare-feu configurÃ© | â˜ |
| Tokens sÃ©curisÃ©s | â˜ |
| Logs en append-only | â˜ |
| Utilisateur dÃ©diÃ© (non-root) | â˜ |
| Mises Ã  jour automatiques | â˜ |
| NTP actif (drift < 5 s) | â˜ |
| Rate limiting configurÃ© | â˜ |
| Protection DDoS (Origin) | â˜ |

### 6.2 Utilisateur dÃ©diÃ©

```bash
# CrÃ©er l'utilisateur systÃ¨me
sudo useradd -r -s /bin/false miyukini

# Droits sur les fichiers
sudo chown -R miyukini:miyukini /etc/miyukini
sudo chmod 600 /etc/miyukini/tokens.json
sudo chmod 600 /etc/miyukini/*.toml
```

### 6.3 Hardening

```bash
# Limiter les connexions par IP
sudo sysctl -w net.ipv4.tcp_max_syn_backlog=2048
sudo sysctl -w net.core.somaxconn=2048

# Protection contre SYN flood
sudo sysctl -w net.ipv4.tcp_syncookies=1
```

### 6.4 Synchronisation NTP (contremesure R-006)

Tous les serveurs (Origin, relays, trackers) doivent Ãªtre synchronisÃ©s avec un serveur NTP pour que la **fenÃªtre d'acceptation des timestamps** (Â±10 secondes) soit respectÃ©e :

```bash
# systemd-timesyncd (Ubuntu/Debian)
sudo systemctl enable systemd-timesyncd
sudo systemctl start systemd-timesyncd
timedatectl set-ntp true

# VÃ©rification
timedatectl status
# Doit afficher "NTP service: active"
```

### 6.5 Protection DDoS (contremesure R-002)

Pour **Origin** et les relays exposÃ©s publiquement :

| Mesure | Description |
|--------|-------------|
| **Service anti-DDoS** | DÃ©ployer Origin derriÃ¨re Cloudflare, AWS Shield ou Ã©quivalent |
| **Challenge-response (PoW)** | ImplÃ©menter le PoW avant REGISTER (voir [MWS - Protection DDoS](../securite/MWS%20-%20Protection%20DDoS.md)) |
| **Rate limiting** | Activer les seuils dÃ©finis dans `[rate_limits]` (voir Â§ 2.2) |
| **Whitelist relays** | Configurer la liste des IP des relays connus pour assouplir le PoW |

---

## 7. Tests de validation

### 7.1 Test de connectivitÃ©

```bash
# Test TLS relay
openssl s_client -connect webway.example.com:7000 -tls1_2

# Test HTTP tracker
curl -I http://tracker.example.com/

# Test HTTPS
curl -I https://webway.example.com/
```

### 7.2 Test fonctionnel

```bash
# Envoi d'un message REGISTER de test
./miyukini-client test-register \
    --relay webway.example.com:7000 \
    --token test-token-123 \
    --cog-id test-cog-001
```

### 7.3 VÃ©rification des services

```bash
# Statut des services
sudo systemctl status miyukini-relay
sudo systemctl status miyukini-tracker

# Logs rÃ©cents
sudo journalctl -u miyukini-relay -f
sudo journalctl -u miyukini-tracker -f
```

---

## 8. Maintenance

### 8.1 Mise Ã  jour des binaires

```bash
# ArrÃªter les services
sudo systemctl stop miyukini-relay miyukini-tracker

# Mettre Ã  jour
cd miyukini-webway-relay
git pull
cargo build --release
sudo cp target/release/miyukini-relay /usr/local/bin/

# RedÃ©marrer
sudo systemctl start miyukini-relay miyukini-tracker
```

### 8.2 Sauvegarde

| Ã‰lÃ©ment | FrÃ©quence |
|---------|-----------|
| Configuration | Quotidienne |
| Tokens | Quotidienne |
| Logs | Mensuelle |
| Certificats | Avant expiration |

### 8.3 Renouvellement certificat

```bash
# Renouveler manuellement
sudo certbot renew

# RedÃ©marrer aprÃ¨s renouvellement
sudo systemctl restart miyukini-relay
```

---

## 9. DÃ©pannage

### 9.1 ProblÃ¨mes courants

| ProblÃ¨me | Cause possible | Solution |
|----------|----------------|----------|
| Connexion refusÃ©e | Pare-feu | VÃ©rifier UFW/iptables |
| TLS handshake failed | Certificat | VÃ©rifier validitÃ© et chemin |
| Token invalid | Configuration | VÃ©rifier tokens.json |
| Origin unreachable | RÃ©seau | VÃ©rifier connectivitÃ© |

### 9.2 Commandes de diagnostic

```bash
# VÃ©rifier les ports ouverts
sudo netstat -tlnp | grep -E '7000|21000|80|443'

# VÃ©rifier le certificat
openssl x509 -in /etc/letsencrypt/live/webway.example.com/cert.pem -text -noout

# Tester la connectivitÃ© Origin
curl -v https://origin.miyukini.com:7000/health

# VÃ©rifier les logs d'erreur
sudo grep -i error /var/log/miyukini/relay.log | tail -20
```

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [Miyukini - Webway Relay Deployment Guide](..//setup//Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) â€” Guide complet
- [Miyukini - Hostinger VPS Origin Webway](..//setup//Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) â€” Instance Origin (Hostinger, Debian 13)
- [MWS - ImplÃ©mentation Origin Hostinger](./MWS%20-%20Implementation%20Origin%20Hostinger.md) â€” Guide complet Origin sur Hostinger
- [MWS - Relays](../acteurs/MWS%20-%20Relays.md)
- [MWS - Trackers](../acteurs/MWS%20-%20Trackers.md)
- [MWS - Protection DDoS](../securite/MWS%20-%20Protection%20DDoS.md)
- [MWS - Contre-Mesures de SÃ©curitÃ©](../securite/MWS%20-%20Contre-Mesures%20de%20Securite.md)

---

**Version :** 2.0  
**Mise Ã  jour :** Rate limiting, NTP, Protection DDoS (R-002, R-006)  
**Classification :** Documentation MWS â€” DÃ©ploiement

