# Miyukini — Instance Oracle Cloud pour le relay Webway

## Contexte

Ce guide permet de créer une **instance Always Free** sur Oracle Cloud pour héberger le **relay Miyukini Webway** (transport multi-COG) et, optionnellement, un **COG Tracker MWS** (découverte sur le port 21000). L'Always Free utilise des **instances de calcul** (VMs), pas les « Hôtes de machine virtuelle dédiés ».

**Région utilisée ici :** France South (Marseille) — `eu-marseille-1`.

## Portée / Scope

- Création et configuration d'une instance Oracle Cloud Always Free pour héberger le relay Webway et/ou le Tracker MWS.
- Règles de sécurité OCI (ports), connexion SSH, installation Rust, compilation, systemd, firewall OS, monitoring.
- Hors scope : développement du relay, spécification du protocole, gouvernance MWS.

---

## 1. Où créer l'instance (Important)

- **Ne pas utiliser** : **Compute → Hôtes de machine virtuelle dédiés**. (Hôtes dédiés = offre payante.)
- **Utiliser** : **Compute → Instances** (ou **Présentation** puis **Créer une instance**).

Dans le menu de gauche OCI : **Menu hamburger → Compute → Instances**.

---

## 2. Créer une instance Always Free

### 2.1 Nom et compartiment

- **Nom :** ex. `webway-relay` ou `miyukini-webway-1`
- **Compartiment :** `studiomiyukini` (racine) ou celui de votre choix

### 2.2 Placement

- **Domaine de disponibilité :** laisser par défaut (ex. AD-1)
- **Capacité :** laisser par défaut

### 2.3 Image et forme (Always Free)

- **Image :** **Ubuntu 22.04** (ou 24.04 LTS) — Canonical, image par défaut
- **Forme :** **VM.Standard.E2.1.Micro** (AMD)  
  - C'est la forme **Always Free** (1/8 OCPU, 1 Go RAM).  
  - Vérifier que la forme affiche le badge **Always Free** / « Toujours gratuit ».

*(En Arm : Ampere A1 avec 4 OCPU et 24 Go RAM est aussi Always Free, selon la région et la disponibilité.)*

### 2.4 Réseau

- **Réseau de cloud virtuel (VCN) :** 
  - Si aucun VCN : **Créer un nouveau réseau de cloud virtuel** (création automatique d'un VCN, d'un sous-réseau public et des règles par défaut).
  - Si VCN existant : sélectionner un VCN avec sous-réseau **public**.
- **Sous-réseau :** sous-réseau public du VCN choisi.
- **Adresse IP publique :** **Attribuer une adresse IPv4 publique** (pour accéder à la VM et exposer le relay).

### 2.5 Clé SSH

- **Ajouter une clé SSH** : soit générer une paire (Oracle enregistre la clé publique), soit coller votre clé publique existante.
- Conserver la **clé privée** en lieu sûr ; elle sert pour `ssh opc@<IP_PUBLIQUE>` (utilisateur par défaut sur Oracle : `opc` pour les images Oracle Linux, ou `ubuntu` pour Ubuntu).

### 2.6 Lancer l'instance

- Cliquer sur **Créer**. L'instance démarre ; l'**IP publique** apparaît dans la liste des instances (quelques secondes à une minute).

---

## 3. Règles de sécurité (ports ouverts)

Pour que le relay et le Tracker MWS soient joignables, ouvrir les ports dans le **Security List** du sous-réseau de l'instance.

### 3.1 Aller à la Security List

- **Réseau (Networking) → Réseaux de cloud virtuels** → cliquer sur le VCN utilisé.
- **Ressources** (à gauche) → **Listes de sécurité** → sélectionner la liste de sécurité du **sous-réseau public** (celle attachée à l'instance).
- **Règles d'entrée (Ingress)** → **Ajouter des règles d'entrée**.

### 3.2 Règles à ajouter

| Source        | Protocol | Destination port | Description (optionnel)     |
|---------------|----------|------------------|-----------------------------|
| `0.0.0.0/0`   | TCP      | 22               | SSH (administration)         |
| `0.0.0.0/0`   | TCP      | 21000            | MWS Tracker (découverte)    |
| `0.0.0.0/0`   | TCP      | 7000             | Relay Webway (transport)    |

- **7000** : port du relay de transport (bore-like) ; modifiable selon l'implémentation.
- **21000** : port officiel MWS pour le COG Tracker (découverte).

Sauvegarder les règles.

---

## 4. Connexion SSH à l'instance

Une fois l'IP publique assignée :

```bash
ssh -i chemin/vers/cle_privee ubuntu@<IP_PUBLIQUE>
```

*(Remplacer `ubuntu` par `opc` si l'image est Oracle Linux.)*

---

## 5. Installation de Rust sur la VM

Le binaire du relay Miyukini Webway est compilé en **Rust**. Sur la VM (Ubuntu ou Oracle Linux), installer l'outillage Rust via `rustup`.

### 5.1 Prérequis

```bash
# Mise à jour des paquets (Ubuntu / Oracle Linux)
sudo apt update && sudo apt upgrade -y   # Ubuntu / Debian
# ou
sudo dnf update -y                        # Oracle Linux
```

### 5.2 Installation de rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choisir l'installation par défaut (1). Puis charger l'environnement dans la session courante :

```bash
source "$HOME/.cargo/env"
```

Vérifier :

```bash
rustc --version
cargo --version
```

### 5.3 Dépendances optionnelles pour la compilation

Pour certains crates (ex. TLS, compression), des librairies système peuvent être nécessaires :

```bash
# Ubuntu / Debian
sudo apt install -y pkg-config libssl-dev build-essential

# Oracle Linux
sudo dnf install -y pkg-config openssl-devel gcc
```

---

## 6. Compilation et déploiement du binaire relay

### 6.1 Récupération du code source

Depuis la machine de développement (ou un dépôt accessible), transférer le workspace Miyukini COG sur la VM, ou cloner le dépôt :

```bash
# Exemple : cloner (remplacer par l'URL réelle du dépôt)
git clone https://github.com/votre-org/Miyukini_COG.git
cd Miyukini_COG
```

Si le relay est dans un crate dédié (ex. `miyuwebway_relay`), se placer à la racine du workspace pour compiler.

### 6.2 Compilation en release

```bash
# À la racine du workspace
cargo build --release -p miyuwebway_relay
```

*(Remplacer `miyuwebway_relay` par le nom exact du crate relay si différent.)*

Le binaire se trouve typiquement dans `target/release/miyuwebway_relay` (ou le nom du crate).

### 6.3 Déploiement du binaire

Créer un répertoire dédié et y copier le binaire ainsi que les fichiers de configuration (certificats TLS, config) :

```bash
sudo mkdir -p /opt/miyukini-webway-relay
sudo cp target/release/miyuwebway_relay /opt/miyukini-webway-relay/relay
sudo chmod +x /opt/miyukini-webway-relay/relay
```

Placer les certificats et la configuration (ex. `relay.toml`, `certs/`) dans `/opt/miyukini-webway-relay/` selon la documentation du relay. Ajuster les droits si nécessaire (lecture seule pour l'utilisateur du service).

---

## 7. Configuration systemd (service relay, démarrage automatique)

Créer un fichier de service systemd pour que le relay démarre au boot et soit géré par systemd (restart automatique, logs via journald).

### 7.1 Fichier de service

```bash
sudo nano /etc/systemd/system/miyukini-webway-relay.service
```

Contenu type :

```ini
[Unit]
Description=Miyukini Webway Relay (transport multi-COG)
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=root
Group=root
WorkingDirectory=/opt/miyukini-webway-relay
ExecStart=/opt/miyukini-webway-relay/relay
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal
SyslogIdentifier=miyukini-webway-relay

# Sécurité optionnelle
NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

*(Pour plus de sécurité en production, créer un utilisateur dédié et remplacer `User=root` / `Group=root` par cet utilisateur.)*

### 7.2 Activer et démarrer le service

```bash
sudo systemctl daemon-reload
sudo systemctl enable miyukini-webway-relay
sudo systemctl start miyukini-webway-relay
sudo systemctl status miyukini-webway-relay
```

Commandes utiles :

- `sudo systemctl stop miyukini-webway-relay` — arrêt
- `sudo systemctl restart miyukini-webway-relay` — redémarrage

---

## 8. Configuration du firewall OS (firewalld / ufw)

Les **Security List** OCI ouvrent les ports au niveau cloud ; sur l'OS, un pare-feu peut encore filtrer. Configurer le firewall de l'hôte pour autoriser les ports du relay et du Tracker.

### 8.1 Oracle Linux (firewalld)

```bash
# Vérifier que firewalld est actif
sudo systemctl status firewalld

# Ouvrir les ports 22 (SSH), 7000 (relay), 21000 (Tracker)
sudo firewall-cmd --permanent --add-port=22/tcp
sudo firewall-cmd --permanent --add-port=7000/tcp
sudo firewall-cmd --permanent --add-port=21000/tcp
sudo firewall-cmd --reload

# Vérifier
sudo firewall-cmd --list-ports
```

### 8.2 Ubuntu (ufw)

```bash
sudo ufw allow 22/tcp
sudo ufw allow 7000/tcp
sudo ufw allow 21000/tcp
sudo ufw enable
sudo ufw status
```

---

## 9. Monitoring et logs

### 9.1 Logs du service (journald)

Les sorties du relay sont envoyées au journal systemd :

```bash
# Dernières lignes en temps réel
sudo journalctl -u miyukini-webway-relay -f

# Dernières 200 lignes
sudo journalctl -u miyukini-webway-relay -n 200

# Depuis une date/heure
sudo journalctl -u miyukini-webway-relay --since "2025-02-12 10:00:00"
```

### 9.2 Rotation et persistance

Par défaut, journald limite la taille des logs. Pour conserver plus longtemps les logs du relay, on peut créer une drop-in :

```bash
sudo mkdir -p /etc/systemd/journald.conf.d
sudo nano /etc/systemd/journald.conf.d/relay.conf
```

Exemple (augmenter la taille max et la durée) :

```ini
[Journal]
SystemMaxUse=500M
MaxRetentionSec=1month
```

Puis :

```bash
sudo systemctl restart systemd-journald
```

### 9.3 Surveillance basique (optionnel)

- **État du service** : `systemctl status miyukini-webway-relay`
- **Connectivité** : depuis une autre machine, `telnet <IP_PUBLIQUE> 7000` ou `nc -zv <IP_PUBLIQUE> 7000`
- **Métriques** : si le relay expose un endpoint de métriques (ex. Prometheus), configurer une sonde ou un scraper selon votre stack de monitoring.

---

## 10. Prochaines étapes (connectivité Miyukini Webway)

1. **Relay de transport** : le binaire relay écoute sur le port **7000**, enregistre les tunnels par `cog_id` et route le trafic (TLS et authentification par token/secret selon la configuration).
2. **Tracker MWS (optionnel)** : sur la même VM ou une autre, exposer le service Tracker sur le port **21000** pour la découverte (annonces, requêtes MWS). Les COGs participants pourront s'enregistrer et découvrir les adresses de connexion (ex. `relay_ip:7000` + token COG).
3. **DNS (optionnel)** : réserver un nom de domaine pointant vers l'IP publique (ex. `webway.studiomiyukini.com`) pour une adresse stable du relay et du Tracker.

Pour un guide pas à pas complet (de la VM vierge au relay fonctionnel, tests, dépannage), voir **[Miyukini - Webway Relay Deployment Guide](Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md)**.

---

## 11. Résumé des ports

| Port  | Rôle                          | Référence MWS              |
|-------|-------------------------------|----------------------------|
| 22    | SSH                           | —                          |
| 7000  | Relay de transport (multi-COG)| À définir (implémentation)|
| 21000 | COG Tracker (découverte)      | Port officiel MWS          |

---

## Références

- [Miyukini Webway System](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)
- [MWS Normes et Standards](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (section 2.7.4 — port 21000)
- [Miyukini - Webway Relay Deployment Guide](Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md)
- Oracle Cloud : [Always Free Resources](https://docs.oracle.com/en-us/iaas/Content/FreeTier/freetier_topic-Always_Free_Resources.htm)

---

*Document créé pour la mise en place du relay Miyukini Webway sur Oracle Cloud Always Free.*
