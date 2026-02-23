# MiyukiniTerminal — Spécification Build et Signature

## Contexte

Ce document décrit le **build** Android (dx bundle), la **configuration** (build.gradle ou équivalent Dioxus), la **signature** (debug/release), les versions (versionCode, versionName), ProGuard/R8, et le format APK vs AAB.

**Références :**

- [Stack Dioxus Mobile Spec](./MiyukiniTerminal%20-%20Stack%20Dioxus%20Mobile%20Spec.md)
- [Environnement Dev Android](./MiyukiniTerminal%20-%20Environnement%20Dev%20Android.md)
- [Spec CI CD](./MiyukiniTerminal%20-%20Spec%20CI%20CD.md)

---

## Portée / Scope

- dx bundle Android
- Configuration build
- Signature debug/release
- Versions
- ProGuard/R8
- APK vs AAB

---

## 1. dx bundle Android

| Commande | Usage |
|----------|-------|
| `dx serve` | Dev ; lance sur émulateur/device |
| `dx bundle` | Build release (APK ou AAB selon config) |
| `cargo build --target aarch64-linux-android` | Build Rust seul |

---

## 2. Configuration build

### 2.1 Dioxus / Cargo

- Cargo.toml : dépendances, features `mobile`
- Structure générée par `dx new` ou intégrée manuellement

### 2.2 build.gradle (Android)

- `minSdkVersion` : 24
- `targetSdkVersion` : 34
- `versionCode` : entier incrémental
- `versionName` : "1.0.0"

### 2.3 AndroidManifest.xml

- `package`, `applicationId`
- Permissions : INTERNET, POST_NOTIFICATIONS, CAMERA (si scan QR)
- `android:allowBackup="false"` pour données sensibles (optionnel)

---

## 3. Signature

### 3.1 Debug

- Keystore debug (généré par SDK)
- Utilisé pour tests, émulateur

### 3.2 Release

- Keystore propre ; protéger la clé
- `keyAlias`, `keyPassword`, `storePassword` (variables secrètes en CI)
- Validity : 25+ ans pour Google Play

### 3.3 Configuration Gradle

```groovy
signingConfigs {
    release {
        storeFile file("miyukini-release.keystore")
        storePassword System.getenv("KEYSTORE_PASSWORD")
        keyAlias "miyukini"
        keyPassword System.getenv("KEY_PASSWORD")
    }
}
```

---

## 4. Versions

| Champ | Format | Exemple |
|-------|--------|---------|
| versionCode | Integer | 1, 2, 3... |
| versionName | String | "1.0.0" |
| Cargo | [package] version | "0.1.0" |

Synchroniser versionCode/versionName avec le release tag.

---

## 5. ProGuard / R8

| Règle | Description |
|-------|-------------|
| Minification | Activer pour release (réduire taille) |
| Obfuscation | Optionnel ; peut casser Rust JNI/FFI |
| Keep rules | Garder classes Rust/FFI si nécessaire |
| Dioxus | Vérifier compatibilité avec R8 |

---

## 6. APK vs AAB

| Format | Usage |
|--------|-------|
| APK | Distribution directe (hors Play Store) |
| AAB | Google Play Store (requis) |

Dioxus/dx : vérifier support AAB ; générer les deux si besoin.

---

## 7. Références

- [Spec CI CD](./MiyukiniTerminal%20-%20Spec%20CI%20CD.md)
- [Dioxus bundle](https://dioxus.dev/learn/0.6/guide/bundle/)
- [Android Signing](https://developer.android.com/studio/publish/app-signing)
