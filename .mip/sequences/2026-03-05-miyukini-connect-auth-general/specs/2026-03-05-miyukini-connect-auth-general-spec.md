# Specification technique - Miyukini Connect

## 1. Objectif et scope

Miyukini Connect est le service d'authentification general du COG.
Il fournit:

1. Authentification multi-facteurs standardisee.
2. Gestion de session avec niveau d'assurance (AAL1..AAL4).
3. Introspection de session pour les autres services.
4. Step-up pour operations sensibles.
5. Fonctionnement complet en online, degraded et isolated.

Hors scope v1:

1. Federation multi-COG complete.
2. Risk scoring ML avance.
3. Biometrics cloud.

## 2. Architecture cible

```text
Central
  -> Miyukini Connect (service local)
      -> connect-api
      -> connect-policy-engine
      -> connect-session-engine
      -> connect-factors
      -> connect-origin-adapter
      -> connect-audit-chain
  -> Services COG (introspection + authorize-hint)

Origin (optionnel)
  <- probe capabilities / reconciliation
```

## 3. Etats runtime

| Etat | Description | Regle |
|------|-------------|-------|
| ONLINE_FULL | Origin joignable, capacites enrichies actives | Toutes methodes autorisees selon policy |
| ONLINE_DEGRADED | Origin partiellement joignable | Methodes online-only restreintes |
| ISOLATED | Origin indisponible | Auth locale continue, aucune baisse exigence securite |
| SUSPICIOUS | Signaux d'anomalie eleves | Exigences AAL relevees + possible read-only sensible |

## 4. Niveaux AAL et permission tiers

### 4.1 Mapping methodes -> AAL max

| Methode | AAL max | Notes |
|---------|---------|-------|
| Password/PIN local | 1 | Base minimale |
| Password + TOTP local | 2 | Standard write |
| Passkey/WebAuthn locale | 3 | Sensible |
| QR challenge signe | 3 | Conditionne a agent local fiable |
| Hardware key FIDO2 | 4 | Critique |
| Email OTP | 2 | Recovery uniquement, jamais critique |

### 4.2 Mapping permission_tier -> AAL requis

| permission_tier | AAL requis | Step-up |
|-----------------|-----------|---------|
| basic | 1 | Non |
| standard_write | 2 | Non |
| sensitive_read | 3 | Selon contexte |
| sensitive_write | 3 | Oui, fenetre courte |
| critical_admin | 4 | Oui, immediat |

## 5. Contrats de donnees

## 5.1 Claims session normalises

```json
{
  "session_id": "uuid",
  "subject_id": "uuid",
  "aal": 3,
  "permission_tier": "sensitive_read",
  "methods": ["passkey"],
  "auth_time": "2026-03-05T21:00:00Z",
  "step_up_until": "2026-03-05T21:10:00Z",
  "runtime_state": "ISOLATED",
  "integrity_fingerprint": "sha256:...",
  "origin_capabilities_snapshot": ["qr_login"]
}
```

### 5.2 Tables minimales

1. `connect_identities`
2. `connect_credentials_password`
3. `connect_totp_secrets`
4. `connect_passkeys`
5. `connect_sessions`
6. `connect_step_up_events`
7. `connect_integrity_snapshots`
8. `connect_audit_chain`

Contraintes cles:

1. Hash password Argon2id uniquement.
2. Secrets sensibles chiffrables at-rest et zeroized en memoire.
3. `connect_audit_chain.prev_hash` obligatoire sauf premier evenement.

## 6. API v1

### 6.1 Bootstrap et etat

1. `POST /v1/connect/bootstrap`
2. `GET /v1/connect/health`
3. `GET /v1/connect/capabilities`

`/bootstrap` reponse type:

```json
{
  "runtime_state": "ONLINE_FULL",
  "available_factors": ["password", "totp", "passkey", "email_otp"],
  "policy_version": "2026.03.05",
  "origin_probe": { "status": "ok", "latency_ms": 143 }
}
```

### 6.2 Auth flow

1. `POST /v1/connect/auth/initiate`
2. `POST /v1/connect/auth/challenge`
3. `POST /v1/connect/auth/verify`
4. `POST /v1/connect/auth/step-up`
5. `POST /v1/connect/auth/logout`

Principes:

1. `initiate` annonce methodes admissibles pour le contexte.
2. `verify` ne cree session que si methodes satisfont `required_aal`.
3. `step-up` peut elevater session existante sans nouveau login complet.

### 6.3 Session inter-services

1. `GET /v1/connect/session/current`
2. `POST /v1/connect/session/introspect`
3. `POST /v1/connect/session/authorize-hint`
4. `POST /v1/connect/session/revoke`

`authorize-hint` exemple:

```json
{
  "requested_action": "jaykonta.transfer.confirm",
  "current_aal": 2,
  "required_aal": 3,
  "step_up_required": true,
  "allowed": false
}
```

### 6.4 Origin integration

1. `POST /v1/connect/origin/ping`
2. `GET /v1/connect/origin/capabilities-cache`

Regle: echec Origin ne degrade jamais les controles locaux.

## 7. Flux principaux

### 7.1 Demarrage Central + Connect

1. Central demarre Connect.
2. Connect charge policy locale.
3. Connect ping Origin (timeout court).
4. Connect publie `runtime_state`.
5. Central affiche UI login Connect.

### 7.2 Acces sensible avec step-up

1. Service appelle `authorize-hint`.
2. Connect detecte `required_aal` superieur.
3. Connect exige facteur fort (passkey/hardware key).
4. Session est elevee avec `step_up_until` court.

### 7.3 Scenario attaque en 2 temps

1. Signal d'anomalie -> passage `SUSPICIOUS`.
2. Sessions sensibles revokees ou degradees.
3. Actions critiques bloquees sans AAL max.
4. A la reconnexion Origin: verification chain audit + rotation sessions.

## 8. Exigences securite MUST (integrees T5)

1. Argon2id obligatoire.
2. Comparaisons constant-time.
3. Rotation session ID apres login et step-up.
4. Idle timeout + absolute timeout.
5. Rate limiting et lockout progressif.
6. Blocage enrolment/recovery faible en `ISOLATED`.
7. Chaine d'audit hash append-only.
8. Reconciliation et revocation preventive au retour online.

## 9. UX/Produit

1. SDK UI integrable: frame, modal, full-screen.
2. Indicateur d'etat runtime visible.
3. Messages explicites pour methodes indisponibles.
4. Parcours "Changer de methode" et recovery degrade.

## 10. Migration

1. Conserver auth Central legacy pendant transition.
2. Migrer hash SHA256 -> Argon2id sur login reussi.
3. Brancher Central vers Connect via feature flag `connect.auth.v1`.
4. Basculer enforcement `permission_tier` par vagues service par service.

## 11. Test strategy et PASS

### PASS-0

1. Login local + TOTP.
2. AAL/permission mapping valide.
3. Session timeout/rotation valide.
4. Rate limits valides.

### PASS-01

1. Passkey/FIDO2 AAL3/AAL4.
2. Isolation hardening actif.
3. Audit chain integrity validee.
4. Test attaque 2 temps passe.

### PASS-RAS

1. Zero bypass step-up.
2. Zero action critique sans AAL requis.
3. Reconciliation post-retour reseau stable.

## 12. Definition of Done P3 readiness

P3 peut demarrer si:

1. Contrats API geles v1.
2. Schema donnees valide.
3. Exigences MUST securite tracees.
4. Plan migration legacy approuve.
5. Jeux de tests PASS etablis.
