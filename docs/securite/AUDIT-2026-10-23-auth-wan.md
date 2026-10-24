# Audit sécurité — Authentification, WAN, fédération, secrets (2026-10-23)

Audit en lecture seule de `miyukini-cog` (branche `mscm/reformat-rust`, commit 5f84ffdc), sur 4 axes :
identité/auth, surface réseau, fédération/crypto, secrets/déploiement. Aucune valeur secrète n'est
reproduite. **[V]** = vérifié dans le code, **[S]** = supposé (dépend du déploiement).

## État d'exposition au 2026-10-23

- Serveur domestique (192.168.1.10) : **aucun service COG en écoute** (vérifié `ss -ltnp`). Les risques
  ci-dessous sont donc latents ici, mais réels dès qu'un binaire est lancé avec sa config par défaut.
- VPS Origin (miyukini.com via nginx → :8080/:8081) : état non vérifiable depuis ce poste **[S]**.
  Si Origin y tourne, C2 et C4 sont exploitables depuis Internet **maintenant**.

## Synthèse

Les primitives crypto sont bien choisies (ChaCha20-Poly1305, X25519, Ed25519, Argon2id, HKDF, OsRng).
Le problème est systémique : **défauts fail-open** (bind `0.0.0.0`, CORS `Any`, auth optionnelle),
**aucune identité prouvée cryptographiquement** (passeports, appareils, COG, packages), et des
**protocoles maison** autour de bonnes primitives.

## CRITIQUE

| # | Constat | Où | Exposition |
|---|---|---|---|
| C1 | **Exécution de commandes à distance sans auth** : MAIA bridge écoute `0.0.0.0:11435`, `require_auth_all=false`, IP lue dans `X-Forwarded-For` (défaut 127.0.0.1), skill `shell_exec` avec `shell=true` → `sh -c`. CORS `Any` : exploitable aussi depuis une page web. | `apps/miou-llm-bridge/src/config.rs:112,127` ; `proxy.rs:89,466,758-768` ; `skills/shell_exec.rs:143-209` ; `skills/file_ops.rs:74-85` (sandbox contournée si `canonicalize` échoue) | Dès lancement [V] |
| C2 | **Prise de contrôle de n'importe quel compte** : `POST /api/auth/forum/sync` public écrase `password_hash` d'un `central_id` (`ON CONFLICT DO UPDATE`), hash SHA-256 accepté. `/api/auth/security/info` divulgue email/état sans auth. | `apps/origin/src/web/forum_auth.rs:328-357,477-541` ; routage `web/server.rs:219-231` | WAN si Origin déployé avec `forum_profiles_db_path` [V/S] |
| C3 | **Packages du Market installés et exécutés sans signature** ; checksum calculé par le serveur et non vérifié par le client ; URL Origin surchargeable (y compris `http://`). | `apps/central/src/market_client.rs:89-165` ; `services/market/service_detail.rs:224,323` ; `services/market/mod.rs:159` ; `launcher.rs:75` | Tous les postes Central [V] |
| C4 | **Publication/suppression Market sans auth** : `verify_developer_token` accepte tout jeton non vide ; `Content-Length` non plafonné ; `version` non validée (traversée possible). | `apps/origin/src/web/market.rs:288,359,397-411,178-183` ; `web/server.rs:311,316,335` | WAN si Origin déployé [V] |
| C5 | **Zip Slip** dans l'installateur Central (`install_dir.join(file.name())`, pas d'`enclosed_name`), `binary_name` non filtré. | `apps/central/src/service_manager/installer.rs:69-104` | Combiné à C3/C4 [V] |
| C6 | **CentralRemote : appairage sans auth + CORS `Any`** : `/api/bridge/approve`, `/generate-token` (sans expiration), `/devices` (fuite des `device_id` au porteur). Défaut struct `0.0.0.0:8091`, UDP discovery toujours `0.0.0.0`. Le bouton « désactiver » ne coupe pas le serveur (`mem::forget`). | `crates/central-remote/src/server.rs:45,96-113,404-504` ; `pairing.rs:160` ; `discovery.rs:32` ; `apps/central/src/components/header.rs:138,157` | LAN / page web locale [V] |
| C7 | **Réutilisation de nonce ChaCha20-Poly1305** : même clé et compteur depuis 0 dans les deux sens (pont, JayMessage 1:1) et pour tous les membres d'un groupe → fuite XOR des clairs + forge. KDF maison (`DefaultHasher`, pas du SHA-256), pas de contrôle contributif X25519. | `crates/miyukini-cog-bridge/src/e2e.rs:85-142,189-209` ; `jaymessage/src/session.rs:42` ; `groups.rs:46-50` | [V] |
| C8 | **Passeport/Visa/Déclarations falsifiables** : `miyauth::verify` sans signature (`[0x01][0×8]` = valide à vie) ; `resolve` donne un rôle sur simple non-vacuité ; `declaration::verify` renvoie toujours `true`. Code mort aujourd'hui. | `crates/miyauth/src/{verify.rs:29-54,resolve.rs:35-51,attest.rs:17}` ; `crates/miyuwebway_participant/src/declaration.rs:24-62` | Latent [V] |
| C9 | **Alicia API : JWT Admin délivré sans vérifier le secret** (`client_id="admin"`), scopes non contrôlés sur dispatch domotique, rate-limit/CORS non implémentés, bind `0.0.0.0`, host invalide → `0.0.0.0`. Non branchée. | `crates/miyualicia-api/src/router.rs:159-193` ; `server.rs:63-67` ; `handlers/capabilities.rs` | Latent [V] |

## HAUTE

- **H1 MAIA-LLM ouvert** : `0.0.0.0:11435`, `auth_key=None` → aucune auth, CORS `Any` (inférence, load/unload de modèles, DoS). `apps/maia-llm/src/{config.rs:52-59,auth.rs:27-30,main.rs:90}` [V]
- **H2 `/auth` CentralRemote** : repli `auth_db.sign_in` contourne verrouillage et TOTP → brute-force illimité ; énumération d'emails par timing ; token en query string ; expiration non revérifiée en WebSocket. `server.rs:211-302` [V]
- **H3 Mobile en clair** : X25519 anonyme sur HTTP, endpoint handshake inexistant, repli silencieux en clair puis email+mot de passe en HTTP ; découverte UDP accepte la première réponse. `bridge.rs:170-209` ; `apps/central-mobile/src/screens/connexion.rs:167-181` [V]
- **H4 Hash SHA-256 non salé** encore créé (feature `kindmother-only`) et accepté en legacy (Central, Origin), comparaisons `==`. `miyukini-central/src/auth/{db_client.rs:22-24,db.rs:47-58}` ; `origin/src/web/forum_auth.rs:151-165` [V]
- **H5 Reset COG par non-admin** puis auto-promotion admin (liste de profils vide). `apps/central/src/screens/profile_window.rs:31-41,110-112` ; `db.rs:554-587` [V]
- **H6 MiyukiniAdmin fail-open** : erreur de `detect()` → mode setup public, `create-admin` rejouable, aucun compteur d'échecs, `session_id` renvoyé en JSON, CORS `Any`. `crates/miyukini-admin/src/{main.rs:158-161,222-225,379,466},services/auth.rs:129-136` [V]
- **H7 Fédération Origin/Relay/Tracker** : `cog_id` au premier arrivé, nonce REGISTER non vérifié, permis au porteur non signé, `tracker_signature` à zéro jamais vérifiée, Tracker TCP sans TLS, `core_conformity_key` partagé dans chaque binaire. `apps/origin/src/relay/server.rs:167,685-795` ; `tracker_client.rs:192` [V]
- **H8 Synchro MiyuCloud** : DH anonyme ; une annonce UDP écrase la clé d'un pair « de confiance ». `crates/miyucloud/src/sync/{transport.rs:151-216,peer_discovery.rs:199-207}` (sync encore stub) [V]
- **H9 Keystore Android + mots de passe suivis par git** : `apps/central-mobile/debug.keystore`, `apps/central-mobile/Dioxus.toml:17-20` [V]
- **H10 Déploiement en root** : `curl | bash` root, `git reset --hard origin/main && cargo build` root sans `--locked` ; `Cargo.lock` dans `.gitignore`. `scripts/deploy-origin.sh:6,30-38` [V]
- **H11 Secrets** : hash Argon2id admin Origin avec sel statique dans `setup-origin.sh:185` et `scripts/setup-origin-hostinger.sh:129` ; historique déjà réécrit (filter-repo, marqueurs `!!REDACTED_PASSWORD!!`) → mot de passe probablement exposé avant réécriture [S] ; PAT GitHub en clair sur disque `.git_token.md` (ignoré, non suivi) [V].

## MOYENNE

- MFA inopérante en pratique (TOTP jamais enrôlé, `totp_code: None` partout ; pas d'anti-rejeu TOTP). `miyukini-connect/src/service.rs:202-209` ; `apps/central/src/data.rs:80,118,170`
- Chaîne d'audit Connect en mémoire, SHA-256 sans clé, champs concaténés ambigus, échecs non journalisés. `miyukini-connect/src/audit.rs:98-117`
- Origin admin derrière nginx : rate-limit par `peer_addr` (=127.0.0.1) → verrouillage global ; email non vérifié ; cookie sans `Secure` ; offset de parsing cookie faux. `apps/origin/src/admin/server.rs:195,214-216,266,300,433-456`
- KindMother : TCP+JSON sans auth (`operator_id` déclaré par le client), loopback. `crates/kindmother-service/src/server.rs:51`
- TEAM hub `0.0.0.0:7800` sans auth, CORS `Any`. `apps/team/src/{main.rs:123,config.rs:51}`
- Trames Relay : `payload_len` tronqué u16 (désynchronisation/injection), requêtes relayées vues comme loopback. `miyuwebway_participant/src/{protocol.rs:228,relay_client.rs:405,446}`
- MiyuCloud au repos sans AAD (réordonnancement/troncature de chunks) ; clé TLS écrite sans 0600. `crypto/at_rest.rs:34-50` ; `apps/miyucloud/src/main.rs:519-530`
- JayMessage : `verify` au lieu de `verify_strict`, signature ne lie ni émetteur ni destinataire ; `GroupKey: Debug`. `identity.rs:94` ; `protocol.rs:63` ; `groups.rs:23`
- Secrets au porteur sur l'appareil en clair ; `ConnectionMode` dérive `Debug/Serialize` avec token et clé privée. `platform.rs:59,198` ; `bridge.rs:20-42`
- CSRF secret de repli codé en dur. `apps/cog-web-portal/src/csrf.rs:17-18`
- PostgreSQL `postgres/postgres` publié `0.0.0.0:5432` (Docker contourne ufw). `apps/miyucloud/docker-compose.yml:6-10`
- CI : pas de `permissions:`, actions non épinglées par SHA. `.github/workflows/miyucloud-ci.yml`
- StrongFather : `_intent` ignoré, politiques Constraint/Validation/Composite toujours `Satisfied`. `strongfather/src/policy_engine.rs:151,167-224`

## BASSE

`Origin_VPS_SSH_and_IP` suivi (IP publique, clé publique, email) ; `curl -k`/certs auto-signés dans la doc ;
comparaison de versions lexicographique (`service_manager/mod.rs:116`) ; course sur le premier admin
(`db.rs:582`) ; `Argon2::default()` (19 MiB) à aligner sur 64 MiB ; dépendances à surveiller (`rsa 0.9`
RUSTSEC-2023-0071, `glib 0.18` RUSTSEC-2024-0429, crates non maintenues) — lancer `cargo audit`.

## Vérifié sain / réutilisable

- MiyuCloud (fork OxiCloud) : JWT HS256 imposé + `exp`, secret 32 o OsRng 0600, Argon2id 64 MiB, lockout, CSRF/CSP/HSTS.
- `miyucloud/src/crypto/keys.rs` (Argon2id + HKDF + zeroize), `jaycloud/src/kits/crypto_kit.rs` (AEAD nonce OsRng),
  `subtle::ct_eq` (`constant_time.rs`, `token_kit.rs`), génération Ed25519 OsRng.
- Aucun `danger_accept_invalid_certs` / `NoVerifier` ; relay en TLS 1.3 ; unités systemd MiyuCloud durcies.
- Historique git : les occurrences `ghp_` et `PRIVATE KEY` sont du code (préfixe / chaîne PEM), pas des secrets.

## Plan de remédiation

**Lot 0 — immédiat (hors code)**
1. Vérifier sur le VPS si Origin tourne ; si oui, bloquer `/api/auth/forum/*`, `/api/auth/security/*`,
   `/api/market/publish`, `DELETE /api/market/package` dans nginx.
2. Rotation : mot de passe admin Origin (+ comptes réutilisant ce mot de passe), clés SSH VPS, PAT GitHub de
   `.git_token.md`, bootstrap token Origin. Demander à GitHub Support la purge des objets d'avant réécriture.

**Lot 1 — fail-closed (petits correctifs, gros gain)**
Bind `127.0.0.1` par défaut partout + refus de démarrer hors loopback sans auth ; suppression des CORS `Any` ;
`miyauth::verify`/`declaration::verify` → `Err(NotImplemented)` ; désactiver `shell=true` ; auth sur routes
Market/forum Origin ; `enclosed_name()` dans l'installateur ; supprimer le repli `sign_in` legacy ;
MiyukiniAdmin `detect()` → `Compromis` ; reset COG réservé admin ; retirer keystore/Dioxus.toml/
`Origin_VPS_SSH_and_IP` du suivi.

**Lot 2 — identité cryptographique (le « passeport type NFT »)**
Passeport v1 signé Ed25519 (encodage canonique, étiquette de domaine `miyukini/passport/v1`, `issuer_cog_pk`,
`subject_device_pk`, `scope`, `nbf/exp`, `serial`, `verify_strict`) ; preuve de possession par challenge ;
registre de révocation append-only à têtes signées ; `cog_id` = empreinte de clé ; packages `.msp` signés par
clé hors ligne, clé publique embarquée dans Central.

**Lot 3 — canaux**
Noise (crate `snow`) pour pont mobile, CentralRemote, inter-COG et synchro MiyuCloud ; clés directionnelles,
anti-rejeu ; épinglage par QR à l'appairage ; TLS Tracker.

**Lot 4 — architecture Intérieur / Frontière + COG OS**
Services Intérieur sans réseau (`PrivateNetwork=yes`, sockets Unix), BorderGuard seul processus exposé,
nftables default-deny, puis image immuable (mkosi, dm-verity, TPM, mises à jour A/B signées).
