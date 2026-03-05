# Plan P3 -- miyucloud-v2-reprise

## TL;DR

Execution P3 demarree en mode FULL.
V0 setup termine localement (branche + deps + smoke tests).
V1 securite critique terminee (timing, XSS, headers, IP, trust_proxy).
V2/V3 back-end deja present et valide via tests modules domaine.
V4 routes API sessions + TOTP ajoutees et compilees.
V5 hardening securite termine.
V6 composants front 2FA/onboarding/health implementes (validation compile front bloquee par dependance assets externe).
V7 infrastructure de deploiement ajoutee (workflow, systemd, Caddy, scripts).

## V0 -- Setup (TERMINEE)

- [x] Creer branche `feat/miyucloud-v2-reprise`
- [x] Verifier dependances ciblees (`totp-rs`, `subtle`, `zeroize`, `base64`)
- [x] Lancer smoke test compile `cargo test -p miyucloud --no-run`
- [x] Lancer smoke test exec `cargo test -p miyucloud --test v2_smoke_test`
- [ ] Push distant `git push -u origin feat/miyucloud-v2-reprise` (bloque: token invalide)

## V1 -- Securite critique (TERMINEE)

- [x] Fix F-11 timing attack comparaison token COG
- [x] Fix F-04 XSS (durcissement token URL/cookie)
- [x] Fix F-12 header injection (sanitization Content-Type)
- [x] Fix F-14 IP non loggee (extraction IP/UA + trust_proxy)
- [x] Fix F-02 timing (comparaison constant-time token middleware)
- [x] Lancer tests miyucloud apres corrections

## V2 -- Auth TOTP + Sessions (TERMINEE)

- [x] Validation module `auth::totp` (15 tests)
- [x] Validation module `auth::sessions` (14 tests)
- [x] Verification structures DB (sessions, totp_secrets, recovery_codes) deja presentes

## V3 -- Onboarding + Monitoring (TERMINEE)

- [x] Validation module `domain::onboarding` (13 tests)
- [x] Validation module `monitoring` (6 tests)

## V4 -- API Handlers + Routes (TERMINEE, lot back principal)

- [x] Ajout module API `apps/miyucloud/src/api/auth_2fa.rs`
- [x] Ajout routes:
  - `POST /api/auth/session`
  - `GET /api/auth/sessions`
  - `DELETE /api/auth/sessions`
  - `DELETE /api/auth/sessions/{id}`
  - `GET /api/auth/totp/status`
  - `POST /api/auth/totp/setup`
  - `POST /api/auth/totp/verify`
  - `POST /api/auth/totp/recovery/verify`
  - `POST /api/auth/totp/recovery/regenerate`
  - `DELETE /api/auth/totp`
- [x] Derivation `Serialize` pour reponses domaine (`SessionInfo`, `TotpSetupResult`)
- [x] Compilation/test serveur OK

## V5 -- Hardening securite (TERMINEE)

- [x] Middleware global `SecurityHeadersLayer` (API + web surface)
- [x] Validation UUID sur IDs de routes sensibles
- [x] Sanitization MIME/segments path et logs IP/UA securises
- [x] Tests serveur OK (`cargo test -p miyucloud-server`)

## V6 -- UX front-end (EN COURS, lot principal implemente)

- [x] Ajout des 6 composants front:
  - `TotpSetupWizard`
  - `TotpVerifyForm`
  - `SessionList`
  - `OnboardingWizard`
  - `HealthDashboard`
  - `RecoveryCodesModal`
- [x] Integration dans `CloudSettings`
- [x] Extension client Central pour endpoints auth/onboarding/health
- [x] Extension API serveur onboarding + health
- [ ] Validation compile `miyukini-central-native` (bloque par assets manquants dans `crates/lord_of_the_castle`)

## V7 -- Infrastructure (TERMINEE, lot de base)

- [x] Workflow GitHub Actions `miyucloud-ci.yml` (lint -> test -> security -> build-release)
- [x] Unite systemd hardenee `deploy/miyucloud/miyucloud.service`
- [x] Config reverse proxy `deploy/miyucloud/Caddyfile`
- [x] Script `apps/miyucloud/scripts/deploy.sh` (rollback auto)
- [x] Script `apps/miyucloud/scripts/backup.sh`
- [x] Script `apps/miyucloud/scripts/healthcheck.sh`
- [ ] Validation shell `bash -n` (non executable dans environnement Windows actuel)

## Commandes executees

```powershell
git checkout -b feat/miyucloud-v2-reprise
cargo test -p miyucloud --no-run
cargo test -p miyucloud --test v2_smoke_test
git push -u origin feat/miyucloud-v2-reprise
cargo test -p miyucloud sanitize
cargo test -p miyucloud auth::totp
cargo test -p miyucloud auth::sessions
cargo test -p miyucloud domain::onboarding
cargo test -p miyucloud monitoring::
cargo test -p miyucloud-server
cargo test -p miyukini-central-native --no-run
bash -n apps/miyucloud/scripts/deploy.sh
bash -n apps/miyucloud/scripts/backup.sh
bash -n apps/miyucloud/scripts/healthcheck.sh
```

