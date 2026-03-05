# Miyukini â€” Instance Oracle Cloud pour Origin Webway

> **DÃ©prÃ©ciÃ© â€” Migration fÃ©vrier 2026 :** L'hÃ©bergement Origin a migrÃ© vers **Hostinger VPS (Debian 13)**. Utiliser [Miyukini - Hostinger VPS Origin Webway](Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) et [MWS - ImplÃ©mentation Origin Hostinger](..//deploiement//MWS%20-%20Implementation%20Origin%20Hostinger.md). Ce document est conservÃ© pour archive (ancienne instance OCI).

## Contexte

Ce guide documentait l'**instance Oracle Cloud Always Free** qui hÃ©bergeait **Origin** (relay + tracker + source de vÃ©ritÃ© MWS). L'instance est **crÃ©Ã©e et opÃ©rationnelle** depuis le 12 fÃ©vrier 2026.

**RÃ©gion :** France South (Marseille) â€” `eu-marseille-1`

Pour le guide complet d'implÃ©mentation logicielle (compilation, configuration, dÃ©marrage), voir :  
**[MWS - ImplÃ©mentation Origin Oracle Cloud](..//deploiement//MWS%20-%20Implementation%20Origin%20Oracle%20Cloud.md)**

---

## 1. Instance Origin (crÃ©Ã©e)

### 1.1 Informations gÃ©nÃ©rales

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **Nom** | (instance Origin MWS) |
| **Compartiment** | `studiomiyukini` (racine) |
| **Domaine de disponibilitÃ©** | AD-1 |
| **Domaine de pannes** | FD-2 |
| **RÃ©gion** | `eu-marseille-1` (France South, Marseille) |
| **LancÃ©e** | 12 fÃ©vrier 2026, 21:06:20 UTC |
| **Type de capacitÃ©** | Ã€ la demande |

### 1.2 Image et forme

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **SystÃ¨me d'exploitation** | **Oracle Linux** |
| **Version** | **9** |
| **Image** | `Oracle-Linux-9.7-2026.01.29-0` |
| **Forme** | `VM.Standard.E2.1.Micro` (Always Free) |
| **OCPU** | 1 |
| **Bande passante** | 0.5 Gbits/s |
| **MÃ©moire** | 1 Go |
| **Disque local** | Stockage de blocs uniquement |
| **Microprogramme** | UEFI_64 |

### 1.3 Options de lancement

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **Type d'attachement rÃ©seau** | PARAVIRTUALIZED |
| **Volume de donnÃ©es distantes** | PARAVIRTUALIZED |
| **Cryptage en transit** | ActivÃ© |
| **Initialisation sÃ©curisÃ©e** | DÃ©sactivÃ© |
| **Initialisation mesurÃ©e** | DÃ©sactivÃ© |
| **Module de plate-forme sÃ©curisÃ©e** | DÃ©sactivÃ© |
| **Type de volume d'initialisation** | PARAVIRTUALIZED |
| **Mode de lancement** | PARAVIRTUALIZED |
| **Service de mÃ©tadonnÃ©es** | Versions 1 et 2 |
| **Migration en direct** | Valeur par dÃ©faut recommandÃ©e |
| **Full Stack Disaster Recovery** | Non activÃ© |

---

## 2. RÃ©seau

### 2.1 Carte d'interface rÃ©seau (VNIC) principale

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **Adresse IPv4 publique** | `84.235.227.152` |
| **Adresse IPv4 privÃ©e** | `10.0.0.110` |
| **RÃ©seau cloud virtuel (VCN)** | `origin-miyukini-webway` |
| **Sous-rÃ©seau** | `webway-0.1` |
| **Table de routage** | `Default Route Table for origin-miyukini-webway` |
| **Groupes de sÃ©curitÃ© rÃ©seau** | `ig-quick-action-NSG` |
| **Enregistrement DNS privÃ©** | Activer |
| **Nom d'hÃ´te** | `origin-miyukini-webway-interface` |
| **FQDN interne** | `origin-miyukini-webway-interface.subnet02122206.vcn02122206.oraclevcn.com` |

### 2.2 RÃ¨gles de sÃ©curitÃ© (ports ouverts)

Les ports MWS sont ouverts dans le groupe de sÃ©curitÃ© `ig-quick-action-NSG` :

| Source | Protocole | Port | Description |
|--------|-----------|------|-------------|
| `0.0.0.0/0` | TCP | 22 | SSH |
| `0.0.0.0/0` | TCP | 80 | HTTP (catalogue web) |
| `0.0.0.0/0` | TCP | 443 | HTTPS (web + MiyukiniAdmin) |
| `0.0.0.0/0` | TCP | 7000 | Origin Relay |
| `0.0.0.0/0` | TCP | 21000 | Origin Tracker |

Pour ajouter ou modifier des rÃ¨gles :  
**Console OCI â†’ RÃ©seau â†’ RÃ©seaux de cloud virtuels â†’ `origin-miyukini-webway` â†’ Groupes de sÃ©curitÃ© rÃ©seau â†’ `ig-quick-action-NSG`**

---

## 3. AccÃ¨s SSH

### 3.1 ClÃ© SSH

| Fichier | Chemin dans le workspace | Usage |
|---------|--------------------------|-------|
| **ClÃ© privÃ©e** | `ssh-key-2026-02-12.key` | Connexion SSH |
| **ClÃ© publique** | `ssh-key-2026-02-12.key.pub` | EnregistrÃ©e sur l'instance OCI |

**ClÃ© publique de rÃ©fÃ©rence (miyukini@gmail.com) â€” Ã  conserver pour tout hÃ©bergeur :**

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIVTDqC5yyNd6Ir9/NLjzUTT1IhugyRyRCo6+O5LZC4Z miyukini@gmail.com
```

Ã€ ajouter dans `~/.ssh/authorized_keys` sur chaque serveur (Oracle Cloud, nouvel hÃ©bergeur Debian/Ubuntu, etc.).

**Utilisateur par dÃ©faut :** `opc` (Oracle Linux) ; sur Debian/Ubuntu : selon compte crÃ©Ã© (ex. `miyukini` ou `opc`).

### 3.2 Connexion depuis Windows (PowerShell)

```powershell
# Depuis la racine du workspace Miyukini_COG
ssh -i ssh-key-2026-02-12.key opc@84.235.227.152
```

> Si erreur "permissions too open" :
> ```powershell
> icacls ssh-key-2026-02-12.key /inheritance:r /grant:r "%USERNAME%:R"
> ```

### 3.3 Connexion depuis Linux / macOS

```bash
chmod 600 ssh-key-2026-02-12.key
ssh -i ssh-key-2026-02-12.key opc@84.235.227.152
```

### 3.4 VÃ©rification aprÃ¨s connexion

```bash
cat /etc/oracle-release     # â†’ Oracle Linux Server release 9.7
hostname                     # â†’ origin-miyukini-webway-interface
ip addr show | grep 84.235  # â†’ IP publique
```

---

## 4. SpÃ©cificitÃ©s Oracle Linux 9.7

| Aspect | Commande / outil |
|--------|-----------------|
| **Package manager** | `dnf` (pas `apt`) |
| **Installer un paquet** | `sudo dnf install -y <paquet>` |
| **Mettre Ã  jour** | `sudo dnf update -y` |
| **Firewall** | `firewalld` (pas `ufw`, pas `iptables` directement) |
| **Ouvrir un port** | `sudo firewall-cmd --permanent --add-port=<port>/tcp && sudo firewall-cmd --reload` |
| **NTP** | `chronyd` (pas `systemd-timesyncd`) |
| **SELinux** | ActivÃ© (Enforcing) â€” `getenforce` |
| **Utilisateur SSH** | `opc` (pas `ubuntu`) |
| **Services** | `systemctl enable/start/stop/status <service>` |
| **DÃ©pendances build** | `gcc gcc-c++ make pkg-config openssl-devel` |
| **EPEL** | `sudo dnf install -y oracle-epel-release-el9` |

---

## 5. Domaine DNS (Ã  configurer)

| EntrÃ©e | Type | Valeur | RÃ´le |
|--------|------|--------|------|
| `origin.miyukini.com` | A | `84.235.227.152` | Adresse canonique d'Origin |
| `webway.miyukini.com` | CNAME | `origin.miyukini.com` | Alias |

> Tant que le DNS n'est pas en place, utiliser l'IP `84.235.227.152` directement.

---

## 6. RÃ©sumÃ© des ports

| Port | RÃ´le | RÃ©fÃ©rence MWS |
|------|------|---------------|
| 22 | SSH (administration) | â€” |
| 80 | HTTP (redirect â†’ HTTPS) | Catalogue web MWS |
| 443 | HTTPS (catalogue + MiyukiniAdmin) | Portail Origin |
| 7000 | Origin Relay (transport) | Protocole relay MWS |
| 21000 | Origin Tracker (dÃ©couverte) | Port officiel MWS |

---

## RÃ©fÃ©rences

- [MWS - ImplÃ©mentation Origin Oracle Cloud](..//deploiement//MWS%20-%20Implementation%20Origin%20Oracle%20Cloud.md) â€” **guide complet d'implÃ©mentation**
- [MWS - Origin](..//README.md)
- [Miyukini Webway System](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)
- [MWS Normes et Standards](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)
- [Miyukini - Webway Relay Deployment Guide](Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md)
- Oracle Cloud : [Always Free Resources](https://docs.oracle.com/en-us/iaas/Content/FreeTier/freetier_topic-Always_Free_Resources.htm)

---

**Version :** 2.0  
**Mise Ã  jour :** Oracle Linux 9.7 (infos rÃ©elles vÃ©rifiÃ©es), clÃ© SSH, rÃ©seau dÃ©taillÃ©, consolidation avec le guide d'implÃ©mentation  
**Classification :** Documentation MWS â€” Setup


