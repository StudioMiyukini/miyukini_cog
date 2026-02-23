# MiyukiniTerminal — Spécification Stack Dioxus Mobile

## Contexte

Ce document détaille la **stack Dioxus Mobile** pour MiyukiniTerminal : versions, features, rendu, APIs, limitations et chemins de build. Référence : [Dioxus Mobile Guide](https://dioxus.dev/learn/0.6/guides/mobile/).

**Références :**

- [Document Fondateur](./MiyukiniTerminal%20-%20Document%20Fondateur.md)
- [Architecture Technique](./MiyukiniTerminal%20-%20Architecture%20Technique.md)

---

## Portée / Scope

- Version Dioxus et features
- Rendu WebView vs WGPU
- APIs dioxus-mobile
- Limitations connues
- Cibles de build (aarch64, armv7, x86, x86_64)
- Commandes `dx serve` / `dx bundle`

---

## 1. Version et dépendances

### 1.1 Versions cibles

| Package | Version | Notes |
|---------|---------|-------|
| **dioxus** | `>= 0.6` | Features `mobile` |
| **dioxus-mobile** | `0.6.x` | Création projet mobile, build Android |
| **rust** | `>= 1.70` | Édition 2021 |

### 1.2 Cargo.toml (extrait)

```toml
[dependencies]
dioxus = { version = "0.6", features = ["mobile"] }
# ou selon structure Dioxus 0.6 :
# dioxus = { version = "0.6", features = ["mobile"] }
```

### 1.3 Features Dioxus

| Feature | Usage |
|---------|-------|
| **mobile** | Active le renderer mobile (WebView ou WGPU) |
| **desktop** | Non utilisé (Central l'utilise ; Terminal = mobile) |
| **web** | Non utilisé |

---

## 2. Rendu : WebView vs WGPU

### 2.1 Options de rendu

| Option | Description | Statut | Usage Terminal |
|--------|-------------|--------|----------------|
| **WebView** | Rendu via WebView Android natif | Stable | **Recommandé** |
| **WGPU** | Rendu GPU natif | Expérimental | Optionnel si performances |

### 2.2 WebView

- Utilise le WebView du système Android
- Supporte **CSS** (animations, transitions)
- Transparence possible
- Performances correctes pour UI type Central

### 2.3 WGPU (expérimental)

- Rendu direct GPU
- Pas de dépendance WebView
- Peut être requis pour certains effets avancés
- À valider stabilité sur Android

**Décision :** WebView par défaut ; WGPU si blocage ou besoin spécifique.

---

## 3. Cibles Rust Android

### 3.1 Targets à installer

```bash
rustup target add aarch64-linux-android   # ARM64 (physique actuel)
rustup target add armv7-linux-androideabi # ARM32 (anciens devices)
rustup target add i686-linux-android      # x86 (émulateur 32 bits)
rustup target add x86_64-linux-android    # x86_64 (émulateur 64 bits)
```

### 3.2 Usage

| Target | Usage |
|--------|-------|
| **aarch64-linux-android** | Appareils physiques récents (recommandé) |
| **x86_64-linux-android** | Émulateur Android (développement) |
| **armv7-linux-androideabi** | Anciens appareils ARM 32 bits |
| **i686-linux-android** | Émulateur 32 bits (legacy) |

### 3.3 Build

```bash
# Émulateur (développement)
cargo build --target x86_64-linux-android

# Appareil physique
cargo build --target aarch64-linux-android
```

---

## 4. APIs dioxus-mobile

### 4.1 use_eval (JavaScript / WebView)

Permet d'exécuter du JavaScript dans le WebView :

```rust
// Exemple : communication Rust ↔ JS
let mut eval = document::eval(r#"dioxus.send("Hello");"#);
eval.send("From Rust").unwrap();
```

**Usage Terminal :** Scan QR (camera API via JS) ; accès APIs Web non exposées.

### 4.2 asset! (ressources locales)

```rust
rsx! {
    img { src: asset!("/assets/static/scanner.png") }
}
```

**Usage Terminal :** Icônes, images statiques, ressources non-URL.

### 4.3 Configuration fenêtre (Wry)

Si besoin de contrôle bas niveau : `use_window`, `DesktopContext`. À valider disponibilité sur mobile.

---

## 5. Chemins de build et outils

### 5.1 dx (Dioxus CLI)

| Commande | Usage |
|----------|-------|
| `dx new my-app` | Créer projet (inclut mobile si config) |
| `dx serve` | Lancer en dev ; détecte émulateur/dispositif |
| `dx bundle` | Build release (APK/AAB) |

### 5.2 Émulateur Android

Avant `dx serve` :

```bash
# Lancer émulateur (ex. Pixel 6 API 34)
emulator -avd Pixel_6_API_34 -netdelay none -netspeed full
```

### 5.3 Structure build Dioxus Android

Dioxus génère/intègre :
- Fichiers Android (build.gradle, AndroidManifest.xml)
- NDK config
- Chemins vers binaires Rust compilés

---

## 6. Limitations connues

| Limitation | Description | Mitigation |
|------------|-------------|------------|
| **Animations natives** | Pas de widgets/animation Android natifs | CSS ; Dioxus abstrait |
| **WebView** | Dépend du WebView système | Tester sur plusieurs API levels |
| **Config lourde** | SDK, NDK, CMAKE requis | Doc [Environnement Dev Android](./MiyukiniTerminal%20-%20Environnement%20Dev%20Android.md) |
| **Hot-reload** | Support variable sur mobile | `dx serve` ; rebuild si nécessaire |
| **Expérimental** | Android support en maturation | POC Phase 3 ; suivi releases Dioxus |

---

## 7. Compatibilité API Android

| API Level | Android | Support Terminal |
|-----------|---------|------------------|
| 24 | 7.0 | **Minimum cible** |
| 30 | 11 | Recommandé |
| 34 | 14 | Émulateur dev courant |

---

## 8. Checklist configuration

- [ ] `rustup target add` (4 targets Android)
- [ ] Android Studio installé
- [ ] SDK, NDK, CMAKE (via SDK Manager)
- [ ] Variables : JAVA_HOME, ANDROID_HOME, NDK_HOME, PATH
- [ ] Émulateur créé et fonctionnel
- [ ] `dx serve` lance l'app sur émulateur
- [ ] `dx bundle` produit APK

---

## 9. Références

- [Dioxus Mobile](https://dioxus.dev/learn/0.6/guides/mobile/)
- [Dioxus Mobile APIs](https://dioxus.dev/learn/0.6/guides/mobile/apis)
- [Environnement Dev Android](./MiyukiniTerminal%20-%20Environnement%20Dev%20Android.md)
- [Alignement Central Dioxus](./MiyukiniTerminal%20-%20Alignement%20Central%20Dioxus.md)
