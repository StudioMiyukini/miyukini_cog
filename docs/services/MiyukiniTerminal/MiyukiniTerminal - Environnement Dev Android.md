# MiyukiniTerminal — Environnement de Développement Android

## Contexte

Ce document est un **guide pas-à-pas** pour configurer l'environnement de développement Android nécessaire à MiyukiniTerminal : Rust targets, Android Studio, SDK, NDK, variables d'environnement, émulateurs et dépannage.

**Références :**

- [Stack Dioxus Mobile Spec](./MiyukiniTerminal%20-%20Stack%20Dioxus%20Mobile%20Spec.md)
- [Dioxus Mobile](https://dioxus.dev/learn/0.6/guides/mobile/)

---

## Portée / Scope

- Installation toolchain Rust Android
- Installation Android Studio, SDK, NDK, CMAKE
- Variables d'environnement (Windows, Linux)
- Configuration émulateur et device physique
- Dépannage des erreurs fréquentes

---

## 1. Prérequis

| Outil | Version minimale |
|-------|------------------|
| **Rust** | 1.70+ |
| **Android Studio** | Arctic Fox ou plus récent |
| **Java (JBR)** | Inclus avec Android Studio |
| **OS** | Windows 10/11, Linux, macOS |

---

## 2. Rust : cibles Android

### 2.1 Installation des targets

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

### 2.2 Vérification

```bash
rustup target list --installed | grep android
```

Résultat attendu : les 4 targets listés.

---

## 3. Android Studio

### 3.1 Installation

1. Télécharger depuis [developer.android.com/studio](https://developer.android.com/studio)
2. Installer (options par défaut recommandées)
3. Premier lancement : suivre l'assistant (SDK par défaut)

### 3.2 SDK Manager

Ouvrir **Tools > SDK Manager** (ou **SDK Manager** dans la barre d'outils).

#### SDK Tools (onglet)

Cocher et installer :

| Composant | Obligatoire |
|-----------|-------------|
| **Android SDK Command-line Tools** | ✅ |
| **Android SDK Build-Tools** | ✅ |
| **Android SDK Platform-Tools** | ✅ |
| **CMake** | ✅ |
| **NDK (Side by side)** | ✅ |
| **Android Emulator** | ✅ (pour dev) |
| **Android SDK Platform** (API 34 ou 33) | ✅ |

#### Version NDK

- Installer **NDK 25.x** ou **26.x** (ex. `25.2.9519653`)
- Noter le chemin : `$ANDROID_HOME/ndk/<version>`

---

## 4. Variables d'environnement

### 4.1 Windows (PowerShell / utilisateur)

```powershell
# Java (fourni par Android Studio)
[System.Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Android\Android Studio\jbr", "User")

# Android SDK (ajuster selon installation)
[System.Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LocalAppData\Android\Sdk", "User")

# NDK (version = celle installée)
[System.Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LocalAppData\Android\Sdk\ndk\25.2.9519653", "User")

# PATH : ajouter platform-tools et emulator
$path = [System.Environment]::GetEnvironmentVariable("PATH", "User")
$androidPath = "$env:LocalAppData\Android\Sdk\platform-tools;$env:LocalAppData\Android\Sdk\emulator"
[System.Environment]::SetEnvironmentVariable("PATH", "$path;$androidPath", "User")
```

**Note :** Redémarrer le terminal après modification.

### 4.2 Windows (Emplacement alternatif SDK)

Si Android Studio a installé le SDK ailleurs :

- `C:\Users\<user>\AppData\Local\Android\Sdk`
- Ou chemin personnalisé défini à l'installation

### 4.3 Linux / macOS

```bash
# Java (Android Studio JBR)
export JAVA_HOME="/opt/android-studio/jbr"  # ou chemin équivalent
# macOS : "/Applications/Android Studio.app/Contents/jbr/Contents/Home"

# Android SDK
export ANDROID_HOME="$HOME/Android/Sdk"
# macOS : "$HOME/Library/Android/sdk"

# NDK
export NDK_HOME="$ANDROID_HOME/ndk/25.2.9519653"

# PATH
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"
```

Ajouter ces lignes à `~/.bashrc` ou `~/.zshrc`.

### 4.4 Vérification

```bash
echo $JAVA_HOME
echo $ANDROID_HOME
echo $NDK_HOME
# Sous Windows : $env:JAVA_HOME, etc.
```

---

## 5. Émulateur Android

### 5.1 Création AVD

1. Ouvrir **Tools > Device Manager**
2. **Create Device** > choisir modèle (ex. Pixel 6)
3. **System Image** : API 34 (ou 33), x86_64
4. **Finish**

### 5.2 Lancer l'émulateur

```bash
emulator -avd Pixel_6_API_34 -netdelay none -netspeed full
```

Remplacer `Pixel_6_API_34` par le nom de l'AVD créé.

### 5.3 Liste des AVD

```bash
emulator -list-avds
```

---

## 6. Device physique

### 6.1 Activer le mode développeur

1. **Paramètres > À propos du téléphone**
2. Taper 7 fois sur **Numéro de build**
3. **Paramètres > Options pour les développeurs** : activer **Débogage USB**

### 6.2 Connexion

1. Brancher en USB
2. Accepter la demande d'autorisation sur le téléphone
3. Vérifier : `adb devices`

### 6.3 Build pour device

```bash
cargo build --target aarch64-linux-android
# Puis déployer via dx ou Android Studio
```

---

## 7. Commandes de développement

### 7.1 Lancer l'app (Dioxus)

```bash
cd apps/terminal  # ou chemin du projet
dx serve
```

`dx serve` détecte automatiquement l'émulateur ou le device connecté.

### 7.2 Build release

```bash
dx bundle
```

Génère l'APK (ou AAB selon config).

---

## 8. Dépannage fréquent

### 8.1 "NDK not found"

| Cause | Solution |
|-------|----------|
| NDK non installé | SDK Manager > NDK (Side by side) |
| NDK_HOME incorrect | Vérifier chemin ; utiliser version exacte |
| Version NDK | Dioxus peut exiger une version précise ; consulter doc Dioxus |

### 8.2 "SDK not found"

| Cause | Solution |
|-------|----------|
| ANDROID_HOME non défini | Définir variable ; redémarrer terminal |
| Chemin incorrect | Vérifier `$ANDROID_HOME/build-tools`, `platforms` |

### 8.3 "CMAKE not found"

| Cause | Solution |
|-------|----------|
| CMAKE non installé | SDK Manager > SDK Tools > CMAKE |
| Version | Installer CMAKE 3.18+ |

### 8.4 "No devices found"

| Cause | Solution |
|-------|----------|
| Émulateur non lancé | `emulator -avd <nom>` avant `dx serve` |
| Device non reconnu | Vérifier `adb devices` ; câble USB ; autorisation |

### 8.5 Erreur de compilation Rust (linker)

| Erreur | Solution |
|--------|----------|
| `linker 'cc' not found` | Installer build-essential (Linux) ; ou NDK inclut un linker |
| `aarch64-linux-android-clang` | Vérifier NDK ; `$NDK_HOME/toolchains/llvm/prebuilt/.../bin` dans PATH |

### 8.6 Emulator in wrong location

Certaines installations Android Studio placent l'émulateur ailleurs. Ajouter manuellement au PATH :

```bash
# Exemple
export PATH="$PATH:$ANDROID_HOME/emulator"
# Chemin alternatif possible : $ANDROID_HOME/../emulator
```

---

## 9. Checklist validation

- [ ] `rustup target add` (4 targets Android)
- [ ] `rustc --target aarch64-linux-android --print sysroot` fonctionne
- [ ] Android Studio installé
- [ ] SDK, NDK, CMAKE visibles dans SDK Manager
- [ ] JAVA_HOME, ANDROID_HOME, NDK_HOME définis
- [ ] `adb devices` liste un appareil (émulateur ou physique)
- [ ] Émulateur lance une image Android
- [ ] `dx serve` lance l'app sur l'émulateur (projet Dioxus minimal)

---

## 10. Références

- [Documentation officielle Android](https://developer.android.com/studio/intro/update#sdk-manager)
- [Dioxus Mobile Setup](https://dioxus.dev/learn/0.6/guides/mobile/)
- [Stack Dioxus Mobile Spec](./MiyukiniTerminal%20-%20Stack%20Dioxus%20Mobile%20Spec.md)
