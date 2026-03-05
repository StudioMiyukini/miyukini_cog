# Miyukini â€” VPS Hostinger pour Origin Webway

## Contexte

Ce guide documente le **VPS Hostinger** qui hÃ©berge **Origin** (relay + tracker + source de vÃ©ritÃ© MWS). Migration depuis Oracle Cloud vers Hostinger (fÃ©vrier 2026).

**HÃ©bergeur :** Hostinger  
**OS :** Debian 13  
**RÃ´le :** Origin MWS (relay, tracker, catalogue, MiyukiniAdmin)

Pour le guide complet d'implÃ©mentation logicielle (compilation, configuration, dÃ©marrage), voir :  
**[MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md)**

---

## 1. VPS Origin (Hostinger)

### 1.1 Informations gÃ©nÃ©rales

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **Provider** | Hostinger |
| **Type** | VPS |
| **SystÃ¨me d'exploitation** | **Debian 13** |
| **RÃ©gion** | Selon offre Hostinger (Ã  documenter) |
| **AccÃ¨s** | SSH (clÃ© publique) |

### 1.2 RÃ©seau

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **Adresse IPv4 publique** | `46.202.129.65` |
| **Nom d'hÃ´te** | Ã€ configurer (ex. `origin-miyukini`) |

### 1.3 Pare-feu (cÃ´tÃ© Hostinger)

Ouvrir les ports suivants dans le panneau Hostinger (ou via `ufw` sur le VPS) :

| Source | Protocole | Port | Description |
|--------|-----------|------|-------------|
| `0.0.0.0/0` | TCP | 22 | SSH |
| `0.0.0.0/0` | TCP | 80 | HTTP (catalogue web) |
| `0.0.0.0/0` | TCP | 443 | HTTPS (web + MiyukiniAdmin) |
| `0.0.0.0/0` | TCP | 7000 | Origin Relay |
| `0.0.0.0/0` | TCP | 21000 | Origin Tracker |

---

## 2. AccÃ¨s SSH

### 2.1 ClÃ© SSH

| Fichier | Chemin dans le workspace | Usage |
|---------|--------------------------|-------|
| **ClÃ© privÃ©e** | `ssh-key-2026-02-12.key` | Connexion SSH |
| **ClÃ© publique** | `ssh-key-2026-02-12.key.pub` | EnregistrÃ©e sur le VPS |

**ClÃ© publique de rÃ©fÃ©rence (miyukini@gmail.com) â€” Ã  conserver pour tout hÃ©bergeur :**

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIVTDqC5yyNd6Ir9/NLjzUTT1IhugyRyRCo6+O5LZC4Z miyukini@gmail.com
```

Ã€ ajouter dans `~/.ssh/authorized_keys` sur le VPS (lors de la crÃ©ation du VPS Hostinger, fournir cette clÃ© publique).

**Utilisateur SSH :** `root`

### 2.2 Connexion depuis Windows (PowerShell)

```powershell
# Depuis la racine du workspace Miyukini_COG
ssh -i ssh-key-2026-02-12.key root@46.202.129.65
```

Ou sans fichier de clÃ© (si la clÃ© est dans l'agent SSH) :

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

Ou si la clÃ© est dans l'agent : `ssh root@46.202.129.65`

### 2.4 VÃ©rification aprÃ¨s connexion

```bash
cat /etc/os-release     # â†’ Debian 13 (Trixie)
hostname                 # â†’ nom du VPS
ip -4 addr show         # â†’ IP publique
```

---

## 3. SpÃ©cificitÃ©s Debian 13

| Aspect | Commande / outil |
|--------|------------------|
| **Package manager** | `apt` |
| **Installer un paquet** | `sudo apt update && sudo apt install -y <paquet>` |
| **Mettre Ã  jour** | `sudo apt update && sudo apt upgrade -y` |
| **Firewall** | `ufw` (pas firewalld) |
| **Ouvrir un port** | `sudo ufw allow <port>/tcp && sudo ufw reload` |
| **NTP** | `systemd-timesyncd` (actif par dÃ©faut) ou `chrony` |
| **SELinux** | Non utilisÃ© par dÃ©faut sur Debian |
| **Nginx** | Config dans `/etc/nginx/sites-available` / `sites-enabled` |
| **Services** | `systemctl enable/start/stop/status <service>` |
| **DÃ©pendances build** | `build-essential pkg-config libssl-dev` |

---

## 4. Domaine DNS (Ã  configurer)

| EntrÃ©e | Type | Valeur | RÃ´le |
|--------|------|--------|------|
| `origin.miyukini.com` | A | `46.202.129.65` | Adresse canonique d'Origin |
| `webway.miyukini.com` | CNAME | `origin.miyukini.com` | Alias |

---

## 5. RÃ©sumÃ© des ports

| Port | RÃ´le | RÃ©fÃ©rence MWS |
|------|------|---------------|
| 22 | SSH (administration) | â€” |
| 80 | HTTP (redirect â†’ HTTPS) | Catalogue web MWS |
| 443 | HTTPS (catalogue + MiyukiniAdmin) | Portail Origin |
| 7000 | Origin Relay (transport) | Protocole relay MWS |
| 21000 | Origin Tracker (dÃ©couverte) | Port officiel MWS |

---

## RÃ©fÃ©rences

- [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md) â€” **guide complet d'implÃ©mentation**
- [MWS - Origin](..//README.md)
- [Miyukini Webway System](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)
- [MWS - Guide de DÃ©ploiement](..//deploiement//MWS%20-%20Guide%20de%20Deploiement.md)

---

**Version :** 1.0  
**Mise Ã  jour :** Migration Oracle Cloud â†’ Hostinger VPS (Debian 13)  
**Classification :** Documentation MWS â€” Setup


