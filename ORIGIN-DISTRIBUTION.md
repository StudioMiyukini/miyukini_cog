# Miyukini Origin — Distribution Prête à l'Emploi

## État de la Distribution

| Composant | État | Description |
|-----------|------|-------------|
| Binaire Release | ✅ | `target/release/miyukini-origin.exe` (3.8 MB) |
| Script déploiement | ✅ | `setup-origin.sh` pour Oracle Linux 9.7 |
| Documentation audit | ✅ | `docs/miyukini-webway-system/MWS - Audit Documentation.md` |

---

## Fonctionnalités Implémentées

### 1. Relay Server (Port 7000)
- [x] TLS avec rustls
- [x] Vérification Phase A (Cores)
- [x] Sessions et tunnels
- [x] Rate limiting
- [x] Métriques

### 2. Tracker Server (Port 21000)
- [x] Pools par version des Cores
- [x] Gestion des COGs
- [x] Lobbys publics et privés
- [x] Découverte et catalogue
- [x] Protocole binaire complet

### 3. Web Server (Port 8080)
- [x] Page d'accueil (Portail MWS)
- [x] Documentation structurée
- [x] Téléchargements
- [x] Dev Blog avec articles
- [x] Annonces officielles
- [x] Catalogue services/lobbys

### 4. Admin Server (Port 8081)
- [x] Dashboard avec statistiques temps réel
- [x] API status, sessions, pools
- [x] Gestion des COGs
- [x] Authentification Argon2id + JWT

### 5. API REST
| Endpoint | Description |
|----------|-------------|
| `GET /api/health` | Health check |
| `GET /api/status` | État du serveur |
| `GET /api/catalog` | Catalogue complet |
| `GET /api/blog` | Articles du blog |
| `GET /api/announcements` | Annonces |
| `GET /api/downloads` | Téléchargements |
| `GET /api/docs` | Documentation |

---

## Déploiement sur Oracle Cloud

### Étape 1 : Compiler le binaire (Windows)

```powershell
cd C:\Users\miyuk\Documents\Cursor\Miyukini_COG
cargo build --release -p miyukini-origin
```

Le binaire est généré dans `target/release/miyukini-origin.exe`.

### Étape 2 : Cross-compiler pour Linux (optionnel)

Pour compiler directement pour Linux :

```powershell
# Installer la toolchain Linux
rustup target add x86_64-unknown-linux-gnu

# Cross-compiler
cargo build --release -p miyukini-origin --target x86_64-unknown-linux-gnu
```

Sinon, copier les sources sur la VM et compiler sur place.

### Étape 3 : Copier sur la VM

```powershell
# Option A : Copier le binaire (si cross-compilé)
scp target/x86_64-unknown-linux-gnu/release/miyukini-origin opc@84.235.227.152:/tmp/

# Option B : Copier les sources
scp -r . opc@84.235.227.152:/tmp/miyukini-cog/
```

### Étape 4 : Exécuter le script de déploiement

```bash
# Se connecter à la VM
ssh opc@84.235.227.152

# Copier et exécuter le script
sudo bash /tmp/setup-origin.sh

# Si les sources sont présentes, compiler
cd /tmp/miyukini-cog
source ~/.cargo/env
cargo build --release -p miyukini-origin

# Installer le binaire
sudo cp target/release/miyukini-origin /usr/local/bin/
sudo systemctl restart miyukini-origin
```

### Étape 5 : Vérification

```bash
# Vérifier les services
sudo systemctl status miyukini-origin
sudo systemctl status nginx

# Tester les endpoints
curl -k https://84.235.227.152/api/health
curl -k https://84.235.227.152/api/status

# Vérifier les ports
sudo ss -tlnp | grep -E '7000|8080|8081|21000'
```

---

## Configuration

### Fichiers de configuration

| Fichier | Description |
|---------|-------------|
| `/etc/miyukini/origin.toml` | Configuration principale |
| `/etc/miyukini/admin.toml` | Configuration admin (confidentiel) |
| `/etc/miyukini/tokens.json` | Tokens d'authentification |
| `/etc/nginx/conf.d/origin-miyukini.conf` | Configuration Nginx |

### Ports réseau

| Port | Service | Protocol |
|------|---------|----------|
| 80 | HTTP (redirect) | TCP |
| 443 | HTTPS (Nginx) | TCP |
| 7000 | Relay MWS | TCP/TLS |
| 8080 | Web Server | TCP |
| 8081 | Admin Server | TCP |
| 21000 | Tracker MWS | TCP |

---

## Logs et Diagnostics

```bash
# Logs Origin
sudo journalctl -u miyukini-origin -f

# Logs Nginx
sudo tail -f /var/log/nginx/error.log

# Fichiers de log dédiés
sudo tail -f /var/log/miyukini/origin.log
sudo tail -f /var/log/miyukini/origin-error.log
```

---

## Sécurité

- TLS 1.2+ obligatoire
- Rate limiting sur tous les endpoints
- Argon2id pour les mots de passe
- JWT pour les sessions admin
- SELinux configuré
- Firewall configuré

---

## URLs de Production

| URL | Description |
|-----|-------------|
| https://84.235.227.152/ | Portail MWS public |
| https://84.235.227.152/admin | MiyukiniAdmin Origin |
| https://84.235.227.152/api/health | Health check |
| https://84.235.227.152/catalog | Catalogue services |

---

## Prochaines Étapes

1. **Certificat Let's Encrypt** : `sudo certbot --nginx -d origin.miyukini.com`
2. **Domaine DNS** : Configurer `origin.miyukini.com` → `84.235.227.152`
3. **Backup** : Configurer les sauvegardes de `/var/lib/miyukini/`
4. **Monitoring** : Intégrer Prometheus/Grafana

---

**Version** : 0.1.0  
**Date** : 2026-02-13  
**Statut** : Production-ready
