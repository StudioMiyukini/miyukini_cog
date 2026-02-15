# Miyukini COG - Guide de Portage vers Linux

**Version :** 0.1  
**Statut :** Reference d'implementation  
**Audience :** Developpeurs, contributeurs, mainteneurs

---

## Contexte

Miyukini COG est actuellement developpe sur Windows. Ce document decrit les etapes et adaptations necessaires pour compiler et executer le projet sur une distribution Linux (Debian/Ubuntu, Fedora, Arch Linux, etc.).

Le code Rust du projet contient deja des adaptations conditionnelles (`#[cfg(unix)]`, `#[cfg(windows)]`) pour les specificites de chaque systeme d'exploitation.

---

## Portee / Scope

Ce document couvre :

- Prerequisites systeme et dependances Linux
- Compilation du workspace complet
- Specificites par application (Central, Origin)
- Differences de comportement Windows/Linux
- Tests et validation

**Exclusions :**

- Deploiement en production (voir documentation Origin)
- Distributions exotiques ou embarquees
- Cross-compilation Windows vers Linux

---

## 1. Prerequisites Systeme

### 1.1 Distribution cible

**Distributions supportees :**

| Distribution | Version minimale | Statut |
|--------------|------------------|--------|
| Ubuntu / Debian | 22.04 LTS / Bookworm | Recommande |
| Fedora | 38+ | Supporte |
| Arch Linux | Rolling | Supporte |
| Alpine Linux | 3.18+ | Non teste |

### 1.2 Toolchain Rust

**Version Rust minimale :** 1.75 (definie dans `Cargo.toml`)

```bash
# Installation via rustup (recommande)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verification
rustc --version   # >= 1.75.0
cargo --version
```

---

## 2. Dependances Systeme

### 2.1 Dependances de compilation

**Ubuntu / Debian :**

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libasound2-dev
```

**Fedora :**

```bash
sudo dnf install -y \
    gcc gcc-c++ \
    pkg-config \
    openssl-devel \
    sqlite-devel \
    gtk3-devel \
    webkit2gtk4.1-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    alsa-lib-devel
```

**Arch Linux :**

```bash
sudo pacman -S --needed \
    base-devel \
    pkg-config \
    openssl \
    sqlite \
    gtk3 \
    webkit2gtk-4.1 \
    libappindicator-gtk3 \
    librsvg \
    alsa-lib
```

### 2.2 Dependances specifiques par application

| Application | Dependance | Usage | Paquet (Ubuntu) |
|-------------|------------|-------|-----------------|
| **Central** | GTK3 / WebKit2GTK | Dioxus Desktop (webview) | `libgtk-3-dev`, `libwebkit2gtk-4.1-dev` |
| **Central** | ALSA | Audio (rodio, voix Miou) | `libasound2-dev` |
| **Origin** | OpenSSL | TLS pour Relay | `libssl-dev` |
| **Tous** | SQLite | Base de donnees | `libsqlite3-dev` |

### 2.3 Dependances optionnelles

```bash
# eSpeak-ng pour la synthese vocale Miou (TTS)
sudo apt install espeak-ng

# Verification
espeak-ng --version
```

---

## 3. Compilation

### 3.1 Clonage et build complet

```bash
# Cloner le depot
git clone https://github.com/StudioMiyukini/miyukini-cog.git
cd miyukini-cog

# Build complet du workspace
cargo build --release

# Build d'une application specifique
cargo build --release -p miyukini-central-native
cargo build --release -p miyukini-origin
```

### 3.2 Verification de la compilation

```bash
# Verifier que les binaires sont crees
ls -la target/release/miyukini-central
ls -la target/release/miyukini-origin

# Tester l'execution
./target/release/miyukini-central --help
./target/release/miyukini-origin --help
```

### 3.3 Build avec features specifiques

```bash
# KindMother avec chiffrement SQLCipher
cargo build --release -p kindmother-service --features db-encryption

# Service avec migration depuis SQLite legacy
cargo build --release -p jayxpose --features legacy-sqlite
```

---

## 4. Adaptations Code par OS

Le code contient des adaptations conditionnelles pour Linux. Voici les points cles :

### 4.1 Identification machine (`kindmother-db-key`)

**Fichier :** `crates/kindmother-db-key/src/key_derivation.rs`

| OS | Source | Chemin |
|----|--------|--------|
| Windows | Registre | `HKLM\SOFTWARE\Microsoft\Cryptography\MachineGuid` |
| Linux/macOS | Fichier | `/etc/machine-id` ou `/var/lib/dbus/machine-id` |

```rust
#[cfg(not(windows))]
fn get_machine_id() -> Result<Vec<u8>, DbKeyError> {
    let machine_id = std::fs::read_to_string("/etc/machine-id")
        .or_else(|_| std::fs::read_to_string("/var/lib/dbus/machine-id"))
        .map_err(|_| DbKeyError("Cannot read machine ID".to_string()))?;
    Ok(machine_id.trim().as_bytes().to_vec())
}
```

**Verification sur Linux :**

```bash
cat /etc/machine-id
# Exemple: 8a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d
```

### 4.2 Permissions fichiers (`kindmother-service`)

**Fichier :** `crates/kindmother-service/src/database.rs`

Sur Linux, les permissions restrictives sont appliquees aux repertoires et fichiers de donnees :

```rust
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    // Repertoire: 700 (rwx------)
    let perms = std::fs::Permissions::from_mode(0o700);
    std::fs::set_permissions(&data_dir, perms)?;
    
    // Fichiers DB: 600 (rw-------)
    let perms = std::fs::Permissions::from_mode(0o600);
    std::fs::set_permissions(&db_path, perms)?;
}
```

### 4.3 WebView2 Data Directory (`apps/central`)

**Fichier :** `apps/central/src/main.rs`

Sur Windows, le dossier WebView2 est redirige vers `LOCALAPPDATA`. Sur Linux, Dioxus utilise le chemin par defaut (generalement `~/.local/share/`).

```rust
#[cfg(windows)]
{
    // Redirection vers LOCALAPPDATA\Miyukini-COG\webview2
    config = config.with_data_directory(webview2_dir);
}
// Sur Linux : pas de configuration speciale necessaire
```

### 4.4 Audio fallback (`apps/central`)

**Fichier :** `apps/central/src/audio.rs`

Sur Windows, un fallback via `cmd /C start` existe si rodio echoue. Sur Linux, seul rodio est utilise (ALSA backend).

```rust
fn play_mp3_sync(path: &PathBuf) -> Result<(), String> {
    match rodio_play(file, path) {
        Ok(()) => Ok(()),
        Err(e) => {
            #[cfg(windows)]
            {
                // Fallback Windows shell
                play_via_shell_windows(path)
            }
            #[cfg(not(windows))]
            Err(e)  // Pas de fallback sur Linux
        }
    }
}
```

### 4.5 Detection OS pour MWS (`miyuwebway_participant`)

**Fichier :** `crates/miyuwebway_participant/src/relay_client.rs`

Le type d'OS est transmis au Relay pour identification :

| OS | Code |
|----|------|
| Windows | 1 |
| Linux | 2 |
| macOS | 3 |
| Autre | 0 |

```rust
fn get_os_type() -> u8 {
    #[cfg(target_os = "windows")]
    return 1;
    #[cfg(target_os = "linux")]
    return 2;
    #[cfg(target_os = "macos")]
    return 3;
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    return 0;
}
```

---

## 5. Chemins et Repertoires

### 5.1 Repertoires de donnees

| Type | Windows | Linux |
|------|---------|-------|
| Donnees application | `%LOCALAPPDATA%\Miyukini-COG\` | `~/.local/share/miyukini-cog/` |
| Configuration | `%APPDATA%\Miyukini\` | `~/.config/miyukini/` |
| Cache | `%LOCALAPPDATA%\Miyukini-COG\cache\` | `~/.cache/miyukini-cog/` |
| Logs | `%LOCALAPPDATA%\Miyukini-COG\logs\` | `~/.local/share/miyukini-cog/logs/` |
| Bases KindMother | `%LOCALAPPDATA%\KindMother\` | `~/.local/share/kindmother/` |

### 5.2 Fichiers critiques

| Fichier | Description | Chemin Linux |
|---------|-------------|--------------|
| `.kindmother_secret` | Secret d'installation (chiffrement DB) | `~/.local/share/kindmother/.kindmother_secret` |
| `machine-id` | Identifiant machine (system) | `/etc/machine-id` |
| `*.db` | Bases SQLite | `~/.local/share/kindmother/*.db` |

---

## 6. Execution et Tests

### 6.1 Lancer Miyukini Central

```bash
# Depuis le repertoire du projet
./target/release/miyukini-central

# Ou avec logs debug
RUST_LOG=debug ./target/release/miyukini-central
```

**Note :** Central lance automatiquement `kindmother-server` s'il n'est pas deja en cours d'execution. Assurez-vous que le binaire `kindmother-server` est dans le meme repertoire ou dans le PATH.

### 6.2 Lancer Miyukini Origin

```bash
# Avec configuration par defaut
./target/release/miyukini-origin

# Avec fichier de configuration
./target/release/miyukini-origin --config /etc/miyukini/origin.toml
```

### 6.3 Executer les tests

```bash
# Tests unitaires du workspace
cargo test --release

# Tests d'un crate specifique
cargo test --release -p kindmother-db-key
cargo test --release -p miyuwebway_participant

# Tests avec logs
RUST_LOG=debug cargo test --release -- --nocapture
```

---

## 7. Problemes Connus et Solutions

### 7.1 Erreur : WebKit2GTK introuvable

**Symptome :**

```
error: could not find system library 'webkit2gtk-4.1'
```

**Solution :**

```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev

# Si la version 4.1 n'existe pas, essayer 4.0
sudo apt install libwebkit2gtk-4.0-dev
```

### 7.2 Erreur : ALSA non trouve

**Symptome :**

```
error: could not find system library 'alsa'
```

**Solution :**

```bash
sudo apt install libasound2-dev
```

### 7.3 Erreur : machine-id introuvable

**Symptome :**

```
DB key error: Cannot read machine ID
```

**Solution :**

```bash
# Verifier que le fichier existe
cat /etc/machine-id

# Si absent, le generer (systemd)
sudo systemd-machine-id-setup
```

### 7.4 Erreur : Permissions sur .kindmother_secret

**Symptome :**

```
Cannot read install secret: Permission denied
```

**Solution :**

```bash
# Verifier les permissions
ls -la ~/.local/share/kindmother/

# Si necessaire, corriger
chmod 400 ~/.local/share/kindmother/.kindmother_secret
chmod 700 ~/.local/share/kindmother/
```

### 7.5 Pas de son (rodio / ALSA)

**Symptome :** La voix Miou ne joue pas, aucune erreur visible.

**Solution :**

```bash
# Verifier que ALSA fonctionne
aplay -l

# Installer alsa-utils si necessaire
sudo apt install alsa-utils

# Tester avec un fichier audio
aplay /usr/share/sounds/alsa/Front_Center.wav
```

---

## 8. Configuration Systemd (Origin en service)

Pour deployer Origin comme service systemd sur un serveur Linux :

**Fichier :** `/etc/systemd/system/miyukini-origin.service`

```ini
[Unit]
Description=Miyukini Origin - MWS Central Hub
After=network.target

[Service]
Type=simple
User=miyukini
Group=miyukini
WorkingDirectory=/opt/miyukini
ExecStart=/opt/miyukini/miyukini-origin --config /etc/miyukini/origin.toml
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

**Commandes :**

```bash
# Recharger systemd
sudo systemctl daemon-reload

# Demarrer le service
sudo systemctl start miyukini-origin

# Activer au demarrage
sudo systemctl enable miyukini-origin

# Voir les logs
journalctl -u miyukini-origin -f
```

---

## 9. Differences Windows / Linux - Resume

| Aspect | Windows | Linux |
|--------|---------|-------|
| Machine ID | Registre Windows | `/etc/machine-id` |
| Permissions fichiers | ACL Windows | chmod Unix |
| WebView runtime | WebView2 (Edge) | WebKitGTK |
| Audio backend | WASAPI | ALSA |
| Audio fallback | Shell `start` | Non (rodio uniquement) |
| Repertoire donnees | `%LOCALAPPDATA%` | `~/.local/share/` |
| Service daemon | Windows Service | systemd |
| Extension binaire | `.exe` | (aucune) |

---

## 10. Check-list Portage

### 10.1 Avant compilation

- [ ] Rust >= 1.75 installe
- [ ] Dependances systeme installees (GTK3, WebKitGTK, ALSA, OpenSSL)
- [ ] `/etc/machine-id` present

### 10.2 Apres compilation

- [ ] Binaires crees dans `target/release/`
- [ ] Tests unitaires passants (`cargo test --release`)
- [ ] Central demarre sans erreur
- [ ] Audio fonctionne (voix Miou)

### 10.3 Deploiement Origin

- [ ] Configuration `/etc/miyukini/origin.toml` en place
- [ ] Service systemd configure
- [ ] Ports ouverts (7000 relay, 7001 tracker, 8080 web)
- [ ] Certificats TLS configures

---

## References

- **Architecture Miyukini :** `docs/reference/Miyukini Conceptual References - Pyramide Architecture Complete.md`
- **Deploiement Origin :** `docs/miyukini-webway-system/deploiement/`
- **KindMother Service :** `docs/core/KindMother/`
- **Dioxus Desktop :** https://dioxuslabs.com/learn/0.6/getting_started

---

**Document cree le :** 2026-02-15  
**Version :** 0.1  
**Statut :** Reference d'implementation
