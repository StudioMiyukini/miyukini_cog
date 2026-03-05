# P3 -- Trace d execution

Statut: EN COURS

## Vague V0 -- Setup (TERMINEE)

- Branche locale creee: `feat/miyucloud-v2-reprise`
- Verification deps ciblees: `totp-rs`, `subtle`, `zeroize`, `base64` deja presentes dans `crates/miyucloud/Cargo.toml`
- Smoke test compilation: `cargo test -p miyucloud --no-run` -> OK
- Smoke test execution: `cargo test -p miyucloud --test v2_smoke_test` -> OK (1 passed)
- Publication distante: `git push -u origin feat/miyucloud-v2-reprise` -> ECHEC (auth GitHub: username/token invalide)

## Vague V1 -- Securite critique (TERMINEE)

- F-11/F-02: comparaison token COG en constant-time (`apps/miyucloud/src/api/auth.rs`)
- F-12: sanitization stricte du `Content-Type` pour les reponses download
- F-14: extraction IP/UA effective pour access logs web + support `trust_proxy`
- F-04: durcissement token pour URL/cookie path via sanitization de segment
- Rate limiter: ajout `trust_proxy` + purge periodique HashMap toutes les 100 requetes

### Tests V1

- `cargo test -p miyucloud --no-run` -> OK
- `cargo test -p miyucloud sanitize` -> OK
- `cargo test -p miyucloud-server --no-run` -> OK
- `cargo test -p miyucloud-server` -> OK (36 passed)

## Vague V2 -- Auth TOTP + Sessions (TERMINEE)

- Les modules domaine `auth::totp` et `auth::sessions` sont deja presentes dans `crates/miyucloud`.
- Validation executee:
  - `cargo test -p miyucloud auth::totp` -> OK (15 passed)
  - `cargo test -p miyucloud auth::sessions` -> OK (14 passed)

## Vague V3 -- Onboarding + Monitoring (TERMINEE)

- Les modules domaine `domain::onboarding` et `monitoring` sont deja presentes.
- Validation executee:
  - `cargo test -p miyucloud domain::onboarding` -> OK (13 passed)
  - `cargo test -p miyucloud monitoring::` -> OK (6 passed)

## Vague V4 -- API Handlers + Routes (TERMINEE, lot principal)

- Ajout du module `apps/miyucloud/src/api/auth_2fa.rs`
- Ajout des routes API sessions + TOTP dans `apps/miyucloud/src/api/mod.rs`
- Reponses domaine serialisables: `SessionInfo`, `TotpSetupResult`
- Validation: `cargo test -p miyucloud-server` -> OK (36 passed)

## Vague V5 -- Securite hardening (TERMINEE)

- Middleware headers de securite ajoute (`apps/miyucloud/src/security_headers.rs`) et applique sur API + web surface
- Validation UUID stricte ajoutee sur IDs sensibles (`files`, `folders`, `shares`, `trash`, `sync`, `auth_2fa`)
- Sanitization MIME et segments de path consolidee (`crates/miyucloud/src/utils/sanitize.rs`)
- Hardening trust-proxy + access logs + rate limiter conserve et valide
- Validation: `cargo test -p miyucloud-server` -> OK (36 passed)

## Vague V6 -- UX front-end (EN COURS, lot principal implemente)

- Ajout des composants:
  - `TotpSetupWizard`
  - `TotpVerifyForm`
  - `SessionList`
  - `OnboardingWizard`
  - `HealthDashboard`
  - `RecoveryCodesModal`
  dans `apps/central/src/services/miyucloud/auth_security.rs`
- Integration des composants dans `CloudSettings`
- Extension du client Central (`client.rs`) pour endpoints:
  - sessions/2FA
  - onboarding status/complete/reset
  - health detaille
- Extension API serveur:
  - `GET /api/admin/health`
  - `GET /api/onboarding/status`
  - `POST /api/onboarding/complete`
  - `POST /api/onboarding/reset`
- Blocage de validation compile front:
  - `cargo test -p miyukini-central-native --no-run` echoue sur assets manquants de `crates/lord_of_the_castle` (hors perimetre MiyuCloud)

## Vague V7 -- Infrastructure (TERMINEE, lot de base)

- Ajout du workflow CI/CD: `.github/workflows/miyucloud-ci.yml`
  - jobs: `lint`, `test`, `security`, `build-release`
- Ajout de l'unite systemd hardenee: `deploy/miyucloud/miyucloud.service`
- Ajout de la config reverse proxy Caddy + TLS: `deploy/miyucloud/Caddyfile`
- Ajout scripts d'exploitation:
  - `apps/miyucloud/scripts/deploy.sh` (deploy + rollback auto)
  - `apps/miyucloud/scripts/backup.sh`
  - `apps/miyucloud/scripts/healthcheck.sh`
- Validation scripts shell non executee (environnement Windows sans `bash`)

## Prochaine vague

- V8 -- Documentation + MSCM
