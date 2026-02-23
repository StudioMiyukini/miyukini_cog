# MiyukiniTerminal — Spécification CI CD

## Contexte

Ce document décrit le **pipeline CI/CD** pour MiyukiniTerminal : build Android (GitHub Actions ou équivalent), tests, lint, artifact APK, variables secrètes et déclencheurs.

**Références :**

- [Spec Build et Signature](./MiyukiniTerminal%20-%20Spec%20Build%20et%20Signature.md)
- [Spec Strategy Tests](./MiyukiniTerminal%20-%20Spec%20Strategy%20Tests.md)
- [Environnement Dev Android](./MiyukiniTerminal%20-%20Environnement%20Dev%20Android.md)

---

## Portée / Scope

- Pipeline (étapes)
- Build Android
- Tests, lint
- Artifact APK
- Variables secrètes
- Déclencheurs

---

## 1. Étapes du pipeline

| Étape | Ordre | Description |
|-------|-------|-------------|
| Checkout | 1 | git checkout |
| Setup Rust | 2 | rustup, targets Android |
| Setup Android | 3 | ANDROID_HOME, NDK, etc. |
| Lint | 4 | cargo clippy |
| Test | 5 | cargo test |
| Build | 6 | cargo build --target aarch64 ou dx bundle |
| Sign (release) | 7 | jarsigner / apksigner |
| Upload artifact | 8 | APK vers GitHub Actions artifacts |

---

## 2. GitHub Actions (exemple)

```yaml
jobs:
  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-action@stable
        with:
          targets: aarch64-linux-android
      - name: Install Android SDK
        uses: nttld/setup-ndk@v1
        with:
          ndk-version: r25c
      - name: Build
        run: cargo build --target aarch64-linux-android --release
      - name: Upload APK
        uses: actions/upload-artifact@v4
        with:
          name: miyukini-terminal
          path: target/aarch64-linux-android/release/*.so  # ou chemin APK selon Dioxus
```

Adapter selon structure Dioxus (dx bundle produit l'APK).

---

## 3. Variables secrètes

| Variable | Usage |
|----------|-------|
| KEYSTORE_PASSWORD | Signature release |
| KEY_PASSWORD | Signature release |
| ANDROID_SIGNING_KEY | Keystore base64 (si stocké en secret) |

Ne jamais exposer en log.

---

## 4. Déclencheurs

| Événement | Action |
|-----------|--------|
| Push main | Build + test |
| Push tag v* | Build release + sign + artifact |
| Pull request | Build + test + lint |

---

## 5. Références

- [Spec Build et Signature](./MiyukiniTerminal%20-%20Spec%20Build%20et%20Signature.md)
- [GitHub Actions](https://docs.github.com/en/actions)
