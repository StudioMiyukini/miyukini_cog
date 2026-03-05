# Miyukini â€” Guide de dÃ©ploiement du relay Webway

## Contexte

Ce guide dÃ©crit le dÃ©ploiement pas Ã  pas du **relay Miyukini Webway** : de la crÃ©ation de la VM Ã  un relay fonctionnel, avec TLS, systemd, monitoring et tests de connectivitÃ©. Pour l'instance Origin actuelle (Hostinger VPS, Debian 13), voir [Miyukini - Hostinger VPS Origin Webway](Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) et [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md). Ce guide complÃ¨te par la compilation, la configuration TLS, le service systemd, les logs et le dÃ©pannage.

## PortÃ©e / Scope

- **Public** : administrateurs dÃ©ployant le relay sur une VM (Oracle Cloud ou autre Linux).
- **Contenu** : ordre des opÃ©rations, commandes, emplacements des fichiers, configuration TLS, tests depuis Windows et Android, troubleshooting.
- **Hors scope** : dÃ©veloppement du code du relay, spÃ©cification du protocole (voir [Miyukini Webway Relay](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) et [Miyukini Webway Relay Protocol](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md)).

---

## 1. Vue d'ensemble des Ã©tapes

| Ã‰tape | Description | RÃ©fÃ©rence |
|-------|-------------|-----------|
| 1 | CrÃ©er et configurer le VPS (ports, SSH) | [Hostinger VPS Origin Webway](Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) |
| 2 | Installer Rust sur le VPS | [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) section 4 |
| 3 | Compiler et dÃ©ployer le binaire relay | [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) section 5 |
| 4 | Configurer TLS (certificats, config) | Ce guide, section 4 |
| 5 | Configurer systemd et firewall | [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) sections 9â€“10 |
| 6 | DÃ©marrer le service, vÃ©rifier logs | [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) section 15 |
| 7 | Tester la connectivitÃ© (Windows, Android) | Ce guide, section 7 |
| 8 | DÃ©pannage | Ce guide, section 8 |

---

## 2. PrÃ©requis

- **VM / VPS** : instance Linux (Debian 13, Ubuntu 22.04+) avec accÃ¨s SSH, IP publique si le relay doit Ãªtre joignable depuis Internet. Origin actuel : Hostinger VPS (Debian 13).
- **Ports** : 22 (SSH), 7000 (relay) â€” et optionnellement 21000 (Tracker MWS) â€” ouverts dans la Security List OCI et dans le firewall de l'OS.
- **Code source** : workspace Miyukini COG (ou dÃ©pÃ´t contenant le crate du relay, ex. `miyuwebway_relay`) accessible depuis la VM (clone git ou transfert).

---

## 3. CrÃ©ation de la VM et accÃ¨s SSH

Suivre le guide [Miyukini - Hostinger VPS Origin Webway](Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) ou [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) :

- CrÃ©er un VPS Hostinger avec Debian 13, ouvrir les ports 22, 80, 443, 7000, 21000, ajouter la clÃ© SSH.
- Section 3 : ajouter les rÃ¨gles d'ingress pour les ports 22, 7000 (et 21000 si Tracker).
- Section 4 : se connecter en SSH (`ssh -i cle_privee ubuntu@<IP_PUBLIQUE>` ou `opc@<IP_PUBLIQUE>`).

---

## 4. Installation de Rust et compilation

### 4.1 Rust et dÃ©pendances

Sur le VPS, exÃ©cuter (voir [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) section 4) :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
# DÃ©pendances (Ubuntu)
sudo apt update && sudo apt install -y pkg-config libssl-dev build-essential
# ou Debian : sudo apt install -y build-essential pkg-config libssl-dev
sudo dnf install -y pkg-config openssl-devel gcc
```

### 4.2 RÃ©cupÃ©ration du code et compilation

```bash
cd ~
git clone <URL_DU_WORKSPACE_MIYUKINI_COG> Miyukini_COG
cd Miyukini_COG
cargo build --release -p miyuwebway_relay
```

*(Remplacer `<URL_DU_WORKSPACE_MIYUKINI_COG>` et le nom du crate `miyuwebway_relay` selon le projet.)*

### 4.3 DÃ©ploiement du binaire

```bash
sudo mkdir -p /opt/miyukini-webway-relay
sudo cp target/release/miyuwebway_relay /opt/miyukini-webway-relay/relay
sudo chmod +x /opt/miyukini-webway-relay/relay
```

---

## 5. Configuration TLS

Le relay doit Ã©couter en TLS sur le port 7000. PrÃ©voir certificat(s) et clÃ©(s) sur le serveur.

### 5.1 Obtenir ou gÃ©nÃ©rer certificats

**Option A â€” Certificat signÃ© (production)**  
Utiliser un certificat dÃ©livrÃ© par une CA (ex. Let's Encrypt). Copier sur la VM :

- Certificat serveur (ex. `fullchain.pem`)
- ClÃ© privÃ©e (ex. `privkey.pem`)

**Option B â€” Certificat auto-signÃ© (test / dev)**  

```bash
sudo mkdir -p /opt/miyukini-webway-relay/certs
cd /opt/miyukini-webway-relay/certs
sudo openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj "/CN=webway-relay"
```

Les clients devront accepter ce certificat (ou l'ajouter aux trusted roots) pour les tests.

### 5.2 Fichier de configuration du relay

CrÃ©er une configuration (ex. `relay.toml`) dans `/opt/miyukini-webway-relay/`. Format indicatif (Ã  adapter au crate) :

```toml
# Exemple de structure (selon implÃ©mentation rÃ©elle)
listen_addr = "0.0.0.0:7000"
tls_cert_path = "/opt/miyukini-webway-relay/certs/cert.pem"
tls_key_path = "/opt/miyukini-webway-relay/certs/key.pem"
# token_secret_path, rate_limit, etc. selon spec relay
```

Ajuster les chemins et options selon la documentation du binaire relay. S'assurer que les droits sur les fichiers sensibles (clÃ© privÃ©e) sont restreints (ex. `chmod 600`).

---

## 6. Service systemd et firewall

### 6.1 UnitÃ© systemd

CrÃ©er le service systemd comme dÃ©crit dans [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) section 9 (WorkingDirectory `/var/lib/miyukini`, ExecStart `/usr/local/bin/miyukini-origin`, Restart=always, logs vers journald).

### 6.2 Firewall OS

- **Debian / Ubuntu (ufw)** : `sudo ufw allow 7000/tcp` puis `sudo ufw reload`.
- **Ubuntu** : `sudo ufw allow 7000/tcp` puis `sudo ufw enable`.

### 6.3 DÃ©marrage et vÃ©rification

```bash
sudo systemctl daemon-reload
sudo systemctl enable miyukini-webway-relay
sudo systemctl start miyukini-webway-relay
sudo systemctl status miyukini-webway-relay
sudo journalctl -u miyukini-webway-relay -f
```

Le relay doit afficher qu'il Ã©coute sur `0.0.0.0:7000` (ou Ã©quivalent). ArrÃªter le suivi des logs avec Ctrl+C.

---

## 7. Tests de connectivitÃ©

### 7.1 Depuis la VM (localhost)

```bash
# Test TCP brut (sans TLS)
nc -zv 127.0.0.1 7000
# Si le relay exige TLS, la connexion peut se fermer aprÃ¨s handshake ; un refus ou timeout indique un problÃ¨me de port/firewall.
```

### 7.2 Depuis Windows

- **PowerShell** : test TCP (le relay attend du TLS, la connexion peut se fermer aprÃ¨s handshake) :

```powershell
Test-NetConnection -ComputerName <IP_PUBLIQUE_RELAY> -Port 7000
```

- **Client TLS** : utiliser un outil (ex. OpenSSL en ligne de commande, ou un client de test fourni par le projet) pointant vers `<IP_PUBLIQUE_RELAY>:7000` avec le certificat si auto-signÃ©.

### 7.3 Depuis Android

- VÃ©rifier que l'appareil est sur un rÃ©seau pouvant joindre l'IP publique du relay (Wiâ€‘Fi ou donnÃ©es).
- Utiliser une application de test rÃ©seau (ex. Â« Network Utilities Â», Â« TCP Client Â») ou un OpÃ©rateur d'Interface COG Miyukini configurÃ© avec l'adresse du relay : `relay_host:<IP_PUBLIQUE>` port 7000.
- En cas de certificat auto-signÃ©, l'application devra accepter une exception de sÃ©curitÃ© ou importer le certificat si le client le gÃ¨re.

### 7.4 Checklist rapide

| Test | Attendu |
|------|---------|
| `systemctl status miyukini-webway-relay` | `active (running)` |
| `nc -zv 127.0.0.1 7000` (VM) | connexion Ã©tablie (puis possible fermeture si TLS requis) |
| Test-NetConnection depuis Windows vers IP:7000 | TcpTestSucceeded = True |
| Client COG enregistrÃ© avec token | tunnel enregistrÃ©, heartbeat OK |

---

## 8. DÃ©pannage (Troubleshooting)

### 8.1 Le service ne dÃ©marre pas

- **VÃ©rifier les logs** : `sudo journalctl -u miyukini-webway-relay -n 50`
- **Erreur de binaire** : vÃ©rifier que `ExecStart` pointe vers le bon chemin et que le binaire est exÃ©cutable (`ls -la /opt/miyukini-webway-relay/relay`).
- **Erreur de config / certificats** : vÃ©rifier les chemins dans `relay.toml`, existence et droits de `cert.pem` / `key.pem` (lecture par l'utilisateur du service).
- **Port dÃ©jÃ  utilisÃ©** : `sudo ss -tlnp | grep 7000` ; arrÃªter le processus qui occupe le port ou changer le port dans la config.

### 8.2 Le relay dÃ©marre mais n'est pas joignable de l'extÃ©rieur

- **Security List OCI** : confirmer qu'une rÃ¨gle d'ingress autorise TCP 7000 depuis `0.0.0.0/0` (ou la plage source attendue).
- **Firewall OS** : `sudo ufw status` (Debian/Ubuntu) ; 7000/tcp doit Ãªtre autorisÃ©.
- **Ã‰coute** : le relay doit Ã©couter sur `0.0.0.0:7000` et non uniquement sur `127.0.0.1`.

### 8.3 Connexions refusÃ©es ou timeouts

- **TLS** : si le client n'utilise pas TLS alors que le relay l'exige (ou inversement), la connexion peut Ãªtre fermÃ©e ou refusÃ©e. VÃ©rifier la configuration TLS du client et du serveur.
- **Token / authentification** : Ã©chec d'auth aprÃ¨s connexion TCP/TLS â†’ vÃ©rifier token/secret et format du handshake (voir [Miyukini Webway Relay Protocol](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md)).

### 8.4 Le service redÃ©marre en boucle (Restart=on-failure)

- Consulter `journalctl -u miyukini-webway-relay -n 100` pour la cause du crash (panic, erreur de config, bind failed).
- Corriger la config ou les certificats, puis `sudo systemctl restart miyukini-webway-relay`.

### 8.5 Logs et rotation

- **Voir les logs en continu** : `sudo journalctl -u miyukini-webway-relay -f`
- **Persistance / taille** : configurer journald si besoin (voir [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) section 12).

---

## 9. RÃ©sumÃ© et rÃ©fÃ©rences

- **VPS et base** : [Miyukini - Hostinger VPS Origin Webway](Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md), [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md)
- **Architecture relay** : [Miyukini Conceptual References - Miyukini Webway Relay](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md)
- **Protocole relay** : [Miyukini Conceptual References - Miyukini Webway Relay Protocol](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md)
- **MWS** : [Miyukini Conceptual References - Miyukini Webway System](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)

---

*Guide de dÃ©ploiement du relay Miyukini Webway â€” de la VM au relay opÃ©rationnel.*

