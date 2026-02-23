# Miyukini — Instance Oracle Cloud pour Origin Webway

> **Déprécié — Migration février 2026 :** L'hébergement Origin a migré vers **Hostinger VPS (Debian 13)**. Utiliser [Miyukini - Hostinger VPS Origin Webway](Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) et [MWS - Implémentation Origin Hostinger](../miyukini-webway-system/deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md). Ce document est conservé pour archive (ancienne instance OCI).

## Contexte

Ce guide documentait l'**instance Oracle Cloud Always Free** qui hébergeait **Origin** (relay + tracker + source de vérité MWS). L'instance est **créée et opérationnelle** depuis le 12 février 2026.

**Région :** France South (Marseille) — `eu-marseille-1`

Pour le guide complet d'implémentation logicielle (compilation, configuration, démarrage), voir :  
**[MWS - Implémentation Origin Oracle Cloud](../miyukini-webway-system/deploiement/MWS%20-%20Implementation%20Origin%20Oracle%20Cloud.md)**

---

## 1. Instance Origin (créée)

### 1.1 Informations générales

| Paramètre | Valeur |
|-----------|--------|
| **Nom** | (instance Origin MWS) |
| **Compartiment** | `studiomiyukini` (racine) |
| **Domaine de disponibilité** | AD-1 |
| **Domaine de pannes** | FD-2 |
| **Région** | `eu-marseille-1` (France South, Marseille) |
| **Lancée** | 12 février 2026, 21:06:20 UTC |
| **Type de capacité** | À la demande |

### 1.2 Image et forme

| Paramètre | Valeur |
|-----------|--------|
| **Système d'exploitation** | **Oracle Linux** |
| **Version** | **9** |
| **Image** | `Oracle-Linux-9.7-2026.01.29-0` |
| **Forme** | `VM.Standard.E2.1.Micro` (Always Free) |
| **OCPU** | 1 |
| **Bande passante** | 0.5 Gbits/s |
| **Mémoire** | 1 Go |
| **Disque local** | Stockage de blocs uniquement |
| **Microprogramme** | UEFI_64 |

### 1.3 Options de lancement

| Paramètre | Valeur |
|-----------|--------|
| **Type d'attachement réseau** | PARAVIRTUALIZED |
| **Volume de données distantes** | PARAVIRTUALIZED |
| **Cryptage en transit** | Activé |
| **Initialisation sécurisée** | Désactivé |
| **Initialisation mesurée** | Désactivé |
| **Module de plate-forme sécurisée** | Désactivé |
| **Type de volume d'initialisation** | PARAVIRTUALIZED |
| **Mode de lancement** | PARAVIRTUALIZED |
| **Service de métadonnées** | Versions 1 et 2 |
| **Migration en direct** | Valeur par défaut recommandée |
| **Full Stack Disaster Recovery** | Non activé |

---

## 2. Réseau

### 2.1 Carte d'interface réseau (VNIC) principale

| Paramètre | Valeur |
|-----------|--------|
| **Adresse IPv4 publique** | `84.235.227.152` |
| **Adresse IPv4 privée** | `10.0.0.110` |
| **Réseau cloud virtuel (VCN)** | `origin-miyukini-webway` |
| **Sous-réseau** | `webway-0.1` |
| **Table de routage** | `Default Route Table for origin-miyukini-webway` |
| **Groupes de sécurité réseau** | `ig-quick-action-NSG` |
| **Enregistrement DNS privé** | Activer |
| **Nom d'hôte** | `origin-miyukini-webway-interface` |
| **FQDN interne** | `origin-miyukini-webway-interface.subnet02122206.vcn02122206.oraclevcn.com` |

### 2.2 Règles de sécurité (ports ouverts)

Les ports MWS sont ouverts dans le groupe de sécurité `ig-quick-action-NSG` :

| Source | Protocole | Port | Description |
|--------|-----------|------|-------------|
| `0.0.0.0/0` | TCP | 22 | SSH |
| `0.0.0.0/0` | TCP | 80 | HTTP (catalogue web) |
| `0.0.0.0/0` | TCP | 443 | HTTPS (web + MiyukiniAdmin) |
| `0.0.0.0/0` | TCP | 7000 | Origin Relay |
| `0.0.0.0/0` | TCP | 21000 | Origin Tracker |

Pour ajouter ou modifier des règles :  
**Console OCI → Réseau → Réseaux de cloud virtuels → `origin-miyukini-webway` → Groupes de sécurité réseau → `ig-quick-action-NSG`**

---

## 3. Accès SSH

### 3.1 Clé SSH

| Fichier | Chemin dans le workspace | Usage |
|---------|--------------------------|-------|
| **Clé privée** | `ssh-key-2026-02-12.key` | Connexion SSH |
| **Clé publique** | `ssh-key-2026-02-12.key.pub` | Enregistrée sur l'instance OCI |

**Clé publique de référence (miyukini@gmail.com) — à conserver pour tout hébergeur :**

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIVTDqC5yyNd6Ir9/NLjzUTT1IhugyRyRCo6+O5LZC4Z miyukini@gmail.com
```

À ajouter dans `~/.ssh/authorized_keys` sur chaque serveur (Oracle Cloud, nouvel hébergeur Debian/Ubuntu, etc.).

**Utilisateur par défaut :** `opc` (Oracle Linux) ; sur Debian/Ubuntu : selon compte créé (ex. `miyukini` ou `opc`).

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

### 3.4 Vérification après connexion

```bash
cat /etc/oracle-release     # → Oracle Linux Server release 9.7
hostname                     # → origin-miyukini-webway-interface
ip addr show | grep 84.235  # → IP publique
```

---

## 4. Spécificités Oracle Linux 9.7

| Aspect | Commande / outil |
|--------|-----------------|
| **Package manager** | `dnf` (pas `apt`) |
| **Installer un paquet** | `sudo dnf install -y <paquet>` |
| **Mettre à jour** | `sudo dnf update -y` |
| **Firewall** | `firewalld` (pas `ufw`, pas `iptables` directement) |
| **Ouvrir un port** | `sudo firewall-cmd --permanent --add-port=<port>/tcp && sudo firewall-cmd --reload` |
| **NTP** | `chronyd` (pas `systemd-timesyncd`) |
| **SELinux** | Activé (Enforcing) — `getenforce` |
| **Utilisateur SSH** | `opc` (pas `ubuntu`) |
| **Services** | `systemctl enable/start/stop/status <service>` |
| **Dépendances build** | `gcc gcc-c++ make pkg-config openssl-devel` |
| **EPEL** | `sudo dnf install -y oracle-epel-release-el9` |

---

## 5. Domaine DNS (à configurer)

| Entrée | Type | Valeur | Rôle |
|--------|------|--------|------|
| `origin.miyukini.com` | A | `84.235.227.152` | Adresse canonique d'Origin |
| `webway.miyukini.com` | CNAME | `origin.miyukini.com` | Alias |

> Tant que le DNS n'est pas en place, utiliser l'IP `84.235.227.152` directement.

---

## 6. Résumé des ports

| Port | Rôle | Référence MWS |
|------|------|---------------|
| 22 | SSH (administration) | — |
| 80 | HTTP (redirect → HTTPS) | Catalogue web MWS |
| 443 | HTTPS (catalogue + MiyukiniAdmin) | Portail Origin |
| 7000 | Origin Relay (transport) | Protocole relay MWS |
| 21000 | Origin Tracker (découverte) | Port officiel MWS |

---

## Références

- [MWS - Implémentation Origin Oracle Cloud](../miyukini-webway-system/deploiement/MWS%20-%20Implementation%20Origin%20Oracle%20Cloud.md) — **guide complet d'implémentation**
- [MWS - Origin](../miyukini-webway-system/acteurs/MWS%20-%20Origin.md)
- [Miyukini Webway System](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)
- [MWS Normes et Standards](../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)
- [Miyukini - Webway Relay Deployment Guide](Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md)
- Oracle Cloud : [Always Free Resources](https://docs.oracle.com/en-us/iaas/Content/FreeTier/freetier_topic-Always_Free_Resources.htm)

---

**Version :** 2.0  
**Mise à jour :** Oracle Linux 9.7 (infos réelles vérifiées), clé SSH, réseau détaillé, consolidation avec le guide d'implémentation  
**Classification :** Documentation MWS — Setup
