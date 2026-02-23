# MiyukiniTerminal — Spécification Token Liaison Sécurité

## Contexte

Ce document détaille le **format du token de liaison**, le payload, la signature, le stockage sécurisé Android (EncryptedSharedPreferences / Keystore) et la protection contre le rejeu.

**Références :**

- [Spec Flux Liaison Parent](./MiyukiniTerminal%20-%20Spec%20Flux%20Liaison%20Parent.md)
- [Spec Securite](./MiyukiniTerminal%20-%20Spec%20Securite.md)

---

## Portée / Scope

- Format token (JWT ou custom)
- Payload : parent_cog_id, user_id, expiration, nonce
- Signature
- Stockage sécurisé Android
- Protection rejeu

---

## 1. Format token

### 1.1 Option A : JWT

```
header.payload.signature
```

**Header :**
```json
{"alg": "HS256", "typ": "JWT"}
```

**Payload :**
```json
{
  "iss": "miyukini-central",
  "sub": "terminal-link",
  "cog_id": "uuid-terminal",
  "parent_cog_id": "uuid-stable",
  "user_id": "user-uuid",
  "exp": 1234567890,
  "iat": 1234567800,
  "nonce": "random-32-bytes-hex"
}
```

**Signature :** HMAC-SHA256(secret, base64(header).base64(payload))

**Secret :** Clé partagée (config Central/Origin) ou clé asymétrique (Ed25519 : signer avec clé privée Central, vérifier avec clé publique connue du Terminal).

### 1.2 Option B : Custom (JSON signé)

```
base64(payload_json).signature_ed25519
```

**Payload :**
```json
{
  "v": 1,
  "cog_id": "uuid",
  "parent_cog_id": "uuid",
  "user_id": "uuid",
  "exp": 1234567890,
  "nonce": "hex"
}
```

**Signature :** Ed25519(payload_bytes, private_key) → 64 bytes.

Le Terminal possède la clé publique Origin/Central ; vérifie la signature avant d'accepter.

### 1.3 Recommandation

**JWT** si écosystème existant ; **custom Ed25519** si contrôle total et pas de dépendance JWT. Les deux sont acceptables.

---

## 2. Champs payload

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| cog_id | string | Oui | UUID du Terminal à créer |
| parent_cog_id | string | Oui | UUID du STABLE parent |
| user_id | string | Oui | Identifiant utilisateur (vérification même utilisateur) |
| exp | int | Oui | Expiration (epoch seconds) |
| iat | int | Optionnel | Émis à (epoch) |
| nonce | string | Oui | Valeur aléatoire (32 bytes hex) ; anti-rejeu |

---

## 3. Signature

### 3.1 HMAC-SHA256 (JWT)

- Secret partagé stocké côté Central et délivré au Terminal (via config, premier déploiement, ou protocole d'échange de clés).
- Risque : si le Terminal est compromis, le secret l'est aussi. Utiliser une clé par environnement.

### 3.2 Ed25519

- Central signe avec clé privée.
- Terminal vérifie avec clé publique (incluse dans l'app ou récupérée depuis Origin).
- Avantage : le Terminal ne peut pas forger de token.

---

## 4. Stockage sécurisé Android

### 4.1 Données à stocker après validation

| Donnée | Sensibilité | Stockage |
|--------|-------------|----------|
| cog_id | Haute | Keystore ou EncryptedSharedPreferences |
| parent_cog_id | Haute | Idem |
| user_id | Moyenne | Idem |
| permis_id | Haute | Idem |
| Token brut | Critique | Ne jamais stocker après utilisation |

### 4.2 EncryptedSharedPreferences (Android Jetpack)

```kotlin
// Exemple conceptuel (Rust/FFI ou équivalent)
EncryptedSharedPreferences.create(
    "miyukini_terminal_identity",
    masterKey,
    context,
    EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
    EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
)
```

Stockage : `cog_id`, `parent_cog_id`, `user_id`.

### 4.3 Android Keystore

Pour les données les plus sensibles :
- Clés stockées dans le Keystore matériel (si disponible)
- Chiffrement AES-256 des valeurs
- Déchiffrement uniquement lorsque l'app a accès (après auth device si configuré)

### 4.4 Chemin Rust/Dioxus

Sous Android, Dioxus/Rust utilise le NDK. Options :
- **rust-android** : appels JNI vers `EncryptedSharedPreferences` (via crate `jni`)
- **Crate `android-storage`** ou équivalent pour accès sécurisé
- **SQLCipher** : base SQLite chiffrée (si KindMother avec db-encryption)

---

## 5. Protection contre le rejeu

### 5.1 Nonce

| Règle | Description |
|-------|-------------|
| Génération | 32 bytes aléatoires (cryptographically secure) |
| Unicité | Un nonce = un token |
| Côté Central | Stocker nonce utilisé ; refuser token rejoué avec même nonce |
| Côté Terminal | Vérifier que le token n'a pas déjà été utilisé (si applicable) |

### 5.2 Expiration courte

| Paramètre | Valeur |
|-----------|--------|
| Validité | 15 minutes |
| Vérification | `exp` > now (avec marge 1 min pour décalage horaire) |

### 5.3 Usage unique (optionnel)

Côté Central : après première validation du token par un Terminal, marquer le token comme utilisé. Toute tentative ultérieure avec le même token = refus.

---

## 6. Flux de validation (Terminal)

```
1. Recevoir token (QR, lien, code)
2. Décoder (base64 si applicable)
3. Vérifier signature
4. Vérifier exp > now
5. Vérifier nonce (si rejeu check côté client)
6. Extraire cog_id, parent_cog_id
7. Stocker en secure storage
8. Invalider token de la mémoire (ne pas garder)
9. Procéder à REGISTER Relay
```

---

## 7. Références

- [Spec Flux Liaison Parent](./MiyukiniTerminal%20-%20Spec%20Flux%20Liaison%20Parent.md)
- [Spec Securite](./MiyukiniTerminal%20-%20Spec%20Securite.md)
- [Android Keystore](https://developer.android.com/training/articles/keystore)
- [EncryptedSharedPreferences](https://developer.android.com/reference/androidx/security/crypto/EncryptedSharedPreferences)
