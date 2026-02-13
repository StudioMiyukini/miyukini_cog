# Miyukini — VPS Hostinger pour Origin Webway

## Contexte

Ce guide documente le **VPS Hostinger** qui héberge **Origin** (relay + tracker + source de vérité MWS). Migration depuis Oracle Cloud vers Hostinger (février 2026).

**Hébergeur :** Hostinger  
**OS :** Debian 13  
**Rôle :** Origin MWS (relay, tracker, catalogue, MiyukiniAdmin)

Pour le guide complet d'implémentation logicielle (compilation, configuration, démarrage), voir :  
**[MWS - Implémentation Origin Hostinger](../miyukini-webway-system/deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md)**

---

## 1. VPS Origin (Hostinger)

### 1.1 Informations générales

| Paramètre | Valeur |
|-----------|--------|
| **Provider** | Hostinger |
| **Type** | VPS |
| **Système d'exploitation** | **Debian 13** |
| **Région** | Selon offre Hostinger (à documenter) |
| **Accès** | SSH (clé publique) |

### 1.2 Réseau

| Paramètre | Valeur |
|-----------|--------|
| **Adresse IPv4 publique** | `46.202.129.65` |
| **Nom d'hôte** | À configurer (ex. `origin-miyukini`) |

### 1.3 Pare-feu (côté Hostinger)

Ouvrir les ports suivants dans le panneau Hostinger (ou via `ufw` sur le VPS) :

| Source | Protocole | Port | Description |
|--------|-----------|------|-------------|
| `0.0.0.0/0` | TCP | 22 | SSH |
| `0.0.0.0/0` | TCP | 80 | HTTP (catalogue web) |
| `0.0.0.0/0` | TCP | 443 | HTTPS (web + MiyukiniAdmin) |
| `0.0.0.0/0` | TCP | 7000 | Origin Relay |
| `0.0.0.0/0` | TCP | 21000 | Origin Tracker |

---

## 2. Accès SSH

### 2.1 Clé SSH

| Fichier | Chemin dans le workspace | Usage |
|---------|--------------------------|-------|
| **Clé privée** | `ssh-key-2026-02-12.key` | Connexion SSH |
| **Clé publique** | `ssh-key-2026-02-12.key.pub` | Enregistrée sur le VPS |

**Clé publique de référence (miyukini@gmail.com) — à conserver pour tout hébergeur :**

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIVTDqC5yyNd6Ir9/NLjzUTT1IhugyRyRCo6+O5LZC4Z miyukini@gmail.com
```

À ajouter dans `~/.ssh/authorized_keys` sur le VPS (lors de la création du VPS Hostinger, fournir cette clé publique).

**Utilisateur SSH :** `root`

### 2.2 Connexion depuis Windows (PowerShell)

```powershell
# Depuis la racine du workspace Miyukini_COG
ssh -i ssh-key-2026-02-12.key root@46.202.129.65
```

Ou sans fichier de clé (si la clé est dans l'agent SSH) :

```powershell
ssh root@46.202.129.65
```

> Si erreur "permissions too open" :
> ```powershell
> icacls ssh-key-2026-02-12.key /inheritance:r /grant:r "%USERNAME%:R"
> ```

### 2.3 Connexion depuis Linux / macOS

```bash
chmod 600 ssh-key-2026-02-12.key
ssh -i ssh-key-2026-02-12.key root@46.202.129.65
```

Ou si la clé est dans l'agent : `ssh root@46.202.129.65`

### 2.4 Vérification après connexion

```bash
cat /etc/os-release     # → Debian 13 (Trixie)
hostname                 # → nom du VPS
ip -4 addr show         # → IP publique
```

---

## 3. Spécificités Debian 13

| Aspect | Commande / outil |
|--------|------------------|
| **Package manager** | `apt` |
| **Installer un paquet** | `sudo apt update && sudo apt install -y <paquet>` |
| **Mettre à jour** | `sudo apt update && sudo apt upgrade -y` |
| **Firewall** | `ufw` (pas firewalld) |
| **Ouvrir un port** | `sudo ufw allow <port>/tcp && sudo ufw reload` |
| **NTP** | `systemd-timesyncd` (actif par défaut) ou `chrony` |
| **SELinux** | Non utilisé par défaut sur Debian |
| **Nginx** | Config dans `/etc/nginx/sites-available` / `sites-enabled` |
| **Services** | `systemctl enable/start/stop/status <service>` |
| **Dépendances build** | `build-essential pkg-config libssl-dev` |

---

## 4. Domaine DNS (à configurer)

| Entrée | Type | Valeur | Rôle |
|--------|------|--------|------|
| `origin.miyukini.com` | A | `46.202.129.65` | Adresse canonique d'Origin |
| `webway.miyukini.com` | CNAME | `origin.miyukini.com` | Alias |

---

## 5. Résumé des ports

| Port | Rôle | Référence MWS |
|------|------|---------------|
| 22 | SSH (administration) | — |
| 80 | HTTP (redirect → HTTPS) | Catalogue web MWS |
| 443 | HTTPS (catalogue + MiyukiniAdmin) | Portail Origin |
| 7000 | Origin Relay (transport) | Protocole relay MWS |
| 21000 | Origin Tracker (découverte) | Port officiel MWS |

---

## Références

- [MWS - Implémentation Origin Hostinger](../miyukini-webway-system/deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md) — **guide complet d'implémentation**
- [MWS - Origin](../miyukini-webway-system/acteurs/MWS%20-%20Origin.md)
- [Miyukini Webway System](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)
- [MWS - Guide de Déploiement](../miyukini-webway-system/deploiement/MWS%20-%20Guide%20de%20Deploiement.md)

---

**Version :** 1.0  
**Mise à jour :** Migration Oracle Cloud → Hostinger VPS (Debian 13)  
**Classification :** Documentation MWS — Setup
