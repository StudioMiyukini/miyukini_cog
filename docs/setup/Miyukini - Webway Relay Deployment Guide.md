# Miyukini — Guide de déploiement du relay Webway

## Contexte

Ce guide décrit le déploiement pas à pas du **relay Miyukini Webway** : de la création de la VM à un relay fonctionnel, avec TLS, systemd, monitoring et tests de connectivité. Il s'appuie sur le guide [Miyukini - Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) pour la partie instance Oracle Cloud et complète par la compilation, la configuration TLS, le service systemd, les logs et le dépannage.

## Portée / Scope

- **Public** : administrateurs déployant le relay sur une VM (Oracle Cloud ou autre Linux).
- **Contenu** : ordre des opérations, commandes, emplacements des fichiers, configuration TLS, tests depuis Windows et Android, troubleshooting.
- **Hors scope** : développement du code du relay, spécification du protocole (voir [Miyukini Webway Relay](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) et [Miyukini Webway Relay Protocol](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md)).

---

## 1. Vue d'ensemble des étapes

| Étape | Description | Référence |
|-------|-------------|-----------|
| 1 | Créer et configurer la VM (ports, SSH) | [Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) sections 1–4 |
| 2 | Installer Rust sur la VM | [Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) section 5 |
| 3 | Compiler et déployer le binaire relay | [Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) section 6 |
| 4 | Configurer TLS (certificats, config) | Ce guide, section 4 |
| 5 | Configurer systemd et firewall | [Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) sections 7–8 |
| 6 | Démarrer le service, vérifier logs | [Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) section 9 |
| 7 | Tester la connectivité (Windows, Android) | Ce guide, section 7 |
| 8 | Dépannage | Ce guide, section 8 |

---

## 2. Prérequis

- **VM** : instance Linux (Ubuntu 22.04 / 24.04 ou Oracle Linux) avec accès SSH, IP publique si le relay doit être joignable depuis Internet.
- **Ports** : 22 (SSH), 7000 (relay) — et optionnellement 21000 (Tracker MWS) — ouverts dans la Security List OCI et dans le firewall de l'OS.
- **Code source** : workspace Miyukini COG (ou dépôt contenant le crate du relay, ex. `miyuwebway_relay`) accessible depuis la VM (clone git ou transfert).

---

## 3. Création de la VM et accès SSH

Suivre le guide [Miyukini - Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) :

- Sections 1–2 : créer une instance Always Free (Compute → Instances, forme VM.Standard.E2.1.Micro, image Ubuntu ou Oracle Linux).
- Section 3 : ajouter les règles d'ingress pour les ports 22, 7000 (et 21000 si Tracker).
- Section 4 : se connecter en SSH (`ssh -i cle_privee ubuntu@<IP_PUBLIQUE>` ou `opc@<IP_PUBLIQUE>`).

---

## 4. Installation de Rust et compilation

### 4.1 Rust et dépendances

Sur la VM, exécuter (voir [Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) section 5) :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
# Dépendances (Ubuntu)
sudo apt update && sudo apt install -y pkg-config libssl-dev build-essential
# ou Oracle Linux
sudo dnf install -y pkg-config openssl-devel gcc
```

### 4.2 Récupération du code et compilation

```bash
cd ~
git clone <URL_DU_WORKSPACE_MIYUKINI_COG> Miyukini_COG
cd Miyukini_COG
cargo build --release -p miyuwebway_relay
```

*(Remplacer `<URL_DU_WORKSPACE_MIYUKINI_COG>` et le nom du crate `miyuwebway_relay` selon le projet.)*

### 4.3 Déploiement du binaire

```bash
sudo mkdir -p /opt/miyukini-webway-relay
sudo cp target/release/miyuwebway_relay /opt/miyukini-webway-relay/relay
sudo chmod +x /opt/miyukini-webway-relay/relay
```

---

## 5. Configuration TLS

Le relay doit écouter en TLS sur le port 7000. Prévoir certificat(s) et clé(s) sur le serveur.

### 5.1 Obtenir ou générer certificats

**Option A — Certificat signé (production)**  
Utiliser un certificat délivré par une CA (ex. Let's Encrypt). Copier sur la VM :

- Certificat serveur (ex. `fullchain.pem`)
- Clé privée (ex. `privkey.pem`)

**Option B — Certificat auto-signé (test / dev)**  

```bash
sudo mkdir -p /opt/miyukini-webway-relay/certs
cd /opt/miyukini-webway-relay/certs
sudo openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj "/CN=webway-relay"
```

Les clients devront accepter ce certificat (ou l'ajouter aux trusted roots) pour les tests.

### 5.2 Fichier de configuration du relay

Créer une configuration (ex. `relay.toml`) dans `/opt/miyukini-webway-relay/`. Format indicatif (à adapter au crate) :

```toml
# Exemple de structure (selon implémentation réelle)
listen_addr = "0.0.0.0:7000"
tls_cert_path = "/opt/miyukini-webway-relay/certs/cert.pem"
tls_key_path = "/opt/miyukini-webway-relay/certs/key.pem"
# token_secret_path, rate_limit, etc. selon spec relay
```

Ajuster les chemins et options selon la documentation du binaire relay. S'assurer que les droits sur les fichiers sensibles (clé privée) sont restreints (ex. `chmod 600`).

---

## 6. Service systemd et firewall

### 6.1 Unité systemd

Créer `/etc/systemd/system/miyukini-webway-relay.service` comme décrit dans [Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) section 7 (WorkingDirectory `/opt/miyukini-webway-relay`, ExecStart `/opt/miyukini-webway-relay/relay`, Restart=on-failure, logs vers journald).

### 6.2 Firewall OS

- **Oracle Linux** : `sudo firewall-cmd --permanent --add-port=7000/tcp` puis `--reload`.
- **Ubuntu** : `sudo ufw allow 7000/tcp` puis `sudo ufw enable`.

### 6.3 Démarrage et vérification

```bash
sudo systemctl daemon-reload
sudo systemctl enable miyukini-webway-relay
sudo systemctl start miyukini-webway-relay
sudo systemctl status miyukini-webway-relay
sudo journalctl -u miyukini-webway-relay -f
```

Le relay doit afficher qu'il écoute sur `0.0.0.0:7000` (ou équivalent). Arrêter le suivi des logs avec Ctrl+C.

---

## 7. Tests de connectivité

### 7.1 Depuis la VM (localhost)

```bash
# Test TCP brut (sans TLS)
nc -zv 127.0.0.1 7000
# Si le relay exige TLS, la connexion peut se fermer après handshake ; un refus ou timeout indique un problème de port/firewall.
```

### 7.2 Depuis Windows

- **PowerShell** : test TCP (le relay attend du TLS, la connexion peut se fermer après handshake) :

```powershell
Test-NetConnection -ComputerName <IP_PUBLIQUE_RELAY> -Port 7000
```

- **Client TLS** : utiliser un outil (ex. OpenSSL en ligne de commande, ou un client de test fourni par le projet) pointant vers `<IP_PUBLIQUE_RELAY>:7000` avec le certificat si auto-signé.

### 7.3 Depuis Android

- Vérifier que l'appareil est sur un réseau pouvant joindre l'IP publique du relay (Wi‑Fi ou données).
- Utiliser une application de test réseau (ex. « Network Utilities », « TCP Client ») ou un Opérateur d'Interface COG Miyukini configuré avec l'adresse du relay : `relay_host:<IP_PUBLIQUE>` port 7000.
- En cas de certificat auto-signé, l'application devra accepter une exception de sécurité ou importer le certificat si le client le gère.

### 7.4 Checklist rapide

| Test | Attendu |
|------|---------|
| `systemctl status miyukini-webway-relay` | `active (running)` |
| `nc -zv 127.0.0.1 7000` (VM) | connexion établie (puis possible fermeture si TLS requis) |
| Test-NetConnection depuis Windows vers IP:7000 | TcpTestSucceeded = True |
| Client COG enregistré avec token | tunnel enregistré, heartbeat OK |

---

## 8. Dépannage (Troubleshooting)

### 8.1 Le service ne démarre pas

- **Vérifier les logs** : `sudo journalctl -u miyukini-webway-relay -n 50`
- **Erreur de binaire** : vérifier que `ExecStart` pointe vers le bon chemin et que le binaire est exécutable (`ls -la /opt/miyukini-webway-relay/relay`).
- **Erreur de config / certificats** : vérifier les chemins dans `relay.toml`, existence et droits de `cert.pem` / `key.pem` (lecture par l'utilisateur du service).
- **Port déjà utilisé** : `sudo ss -tlnp | grep 7000` ; arrêter le processus qui occupe le port ou changer le port dans la config.

### 8.2 Le relay démarre mais n'est pas joignable de l'extérieur

- **Security List OCI** : confirmer qu'une règle d'ingress autorise TCP 7000 depuis `0.0.0.0/0` (ou la plage source attendue).
- **Firewall OS** : `sudo firewall-cmd --list-ports` (Oracle Linux) ou `sudo ufw status` (Ubuntu) ; 7000/tcp doit être autorisé.
- **Écoute** : le relay doit écouter sur `0.0.0.0:7000` et non uniquement sur `127.0.0.1`.

### 8.3 Connexions refusées ou timeouts

- **TLS** : si le client n'utilise pas TLS alors que le relay l'exige (ou inversement), la connexion peut être fermée ou refusée. Vérifier la configuration TLS du client et du serveur.
- **Token / authentification** : échec d'auth après connexion TCP/TLS → vérifier token/secret et format du handshake (voir [Miyukini Webway Relay Protocol](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md)).

### 8.4 Le service redémarre en boucle (Restart=on-failure)

- Consulter `journalctl -u miyukini-webway-relay -n 100` pour la cause du crash (panic, erreur de config, bind failed).
- Corriger la config ou les certificats, puis `sudo systemctl restart miyukini-webway-relay`.

### 8.5 Logs et rotation

- **Voir les logs en continu** : `sudo journalctl -u miyukini-webway-relay -f`
- **Persistance / taille** : configurer journald si besoin (voir [Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md) section 9.2).

---

## 9. Résumé et références

- **VM et base** : [Miyukini - Oracle Cloud Instance Webway Relay](Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md)
- **Architecture relay** : [Miyukini Conceptual References - Miyukini Webway Relay](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md)
- **Protocole relay** : [Miyukini Conceptual References - Miyukini Webway Relay Protocol](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md)
- **MWS** : [Miyukini Conceptual References - Miyukini Webway System](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)

---

*Guide de déploiement du relay Miyukini Webway — de la VM au relay opérationnel.*
