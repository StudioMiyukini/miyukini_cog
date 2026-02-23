# MiyukiniTerminal — Spécification Sécurité

## Contexte

Ce document décrit les mesures de **sécurité** du Terminal Android : stockage sensible (Keystore, EncryptedSharedPreferences), TLS obligatoire, validation certificats, absence de logs de tokens, verrouillage app (PIN/biométrie), permissions Android.

**Références :**

- [Spec Token Liaison Securite](./MiyukiniTerminal%20-%20Spec%20Token%20Liaison%20Securite.md)
- [Spec Stockage Local](./MiyukiniTerminal%20-%20Spec%20Stockage%20Local.md)
- [Spec Conformite Cores](./MiyukiniTerminal%20-%20Spec%20Conformite%20Cores.md)

---

## Portée / Scope

- Stockage sensible
- TLS, certificats
- Logs (interdictions)
- Verrouillage app
- Permissions Android

---

## 1. Stockage sensible

### 1.1 Données sensibles

| Donnée | Stockage |
|--------|----------|
| cog_id | Keystore / EncryptedSharedPreferences |
| parent_cog_id | Idem |
| permis_id | Idem |
| Token (temporaire) | Mémoire uniquement ; jamais persister après validation |
| Clés API (si applicable) | Keystore |

### 1.2 EncryptedSharedPreferences

- Chiffrement AES-256-GCM
- Clé maître dans Android Keystore
- Utiliser pour identity, preferences sensibles

### 1.3 Android Keystore

- Clés cryptographiques stockées matériellement (si disponible)
- Pas d'extraction des clés en clair
- Résistant au root (relativement)

---

## 2. TLS obligatoire

| Règle | Description |
|-------|-------------|
| Relay | Connexion toujours TLS (port 7000) |
| API parent | HTTPS obligatoire |
| Certificats | Valider chaîne ; pas de mode "trust all" en prod |
| Pinning | Optionnel : pinner certificat Origin pour éviter MITM |

---

## 3. Validation certificats

| Règle | Description |
|-------|-------------|
| Chaîne complète | Vérifier jusqu'à CA racine |
| Révoqués | Vérifier CRL/OCSP si configuré |
| Hostname | Vérifier que le CN/SAN correspond à l'hôte attendu |

---

## 4. Pas de log de tokens

| Règle | Description |
|-------|-------------|
| Token | Ne jamais logger (même hash en debug) |
| Mots de passe | Jamais |
| Données personnelles | Éviter ; anonymiser si nécessaire |
| Logs prod | Niveau info ; pas de traces sensibles |

---

## 5. Verrouillage app

### 5.1 Option

| Paramètre | Description |
|-----------|-------------|
| PIN | 4–6 chiffres |
| Biométrie | Empreinte, Face (si dispo) |
| Timeout | Verrouiller après N min d'inactivité |

### 5.2 Comportement

- Au retour sur l'app (foreground) : demander PIN/biométrie si timeout dépassé
- Données en mémoire : nettoyer ou protéger par chiffrement session

---

## 6. Permissions Android

| Permission | Usage |
|------------|-------|
| INTERNET | Connexion Relay, API |
| POST_NOTIFICATIONS | Notifications (API 33+) |
| CAMERA | Scan QR (si besoin) |
| VIBRATE | Retour haptique (optionnel) |

**Principe :** Demander uniquement les permissions nécessaires ; expliquer le besoin à l'utilisateur.

---

## 7. Références

- [Spec Token Liaison Securite](./MiyukiniTerminal%20-%20Spec%20Token%20Liaison%20Securite.md)
- [Android Security](https://developer.android.com/topic/security)
