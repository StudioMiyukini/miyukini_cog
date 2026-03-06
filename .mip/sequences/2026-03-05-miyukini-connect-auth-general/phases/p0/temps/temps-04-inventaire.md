# P0 Temps 4 - Inventaire prerequis

## Statut

- Etat : Termine
- Phase : P0 Temps 4
- Responsable principal : Denis/Hugo/Jean
- Date : 2026-03-05

## TL;DR

Inventaire T4 complete pour lancer Miyukini Connect selon l'option D (local-first durci + step-up + anti-2-temps). Le socle existe partiellement (Central auth local, Origin session/crypto, docs MiyuAuth), mais des gaps critiques restent: hash legacy SHA256 cote Central, absence de moteur AAL unifie, absence passkey/FIDO2, absence de scellage d'integrite session et de journal inviolable auth.

## 1) Inventaire de l'existant exploitable

### 1.1 Central / profils / login

- `apps/central/src/screens/connexion.rs`
  - Ecran de connexion deja present (email + mot de passe).
  - Integration immediate possible pour redirection vers Miyukini Connect UI.
- `crates/miyukini-central/src/auth/db.rs`
  - `CentralAuthDb` en place (profils, session courante, metadata).
  - `sign_in`/`sign_up` existants.
  - **Point bloquant securite**: hash mot de passe en SHA256 (fonction `hash_password`), insuffisant pour cible sensible.

### 1.2 Origin / session reseau

- `apps/origin/src/relay/session.rs`
  - Gestionnaire de sessions reseau mature (state machine, timeouts, session key, nettoyage).
  - Reutilisable conceptuellement pour lifecycle session Connect.
- `apps/origin/src/web/forum_auth.rs`
  - Validation Argon2id + migration legacy SHA256 deja implementee dans un contexte web forum.
  - Source de pattern utile pour migration Central -> hash robuste.

### 1.3 Toolkit MiyuAuth

- `crates/miyauth/src/lib.rs`
  - Toolkit d'identite existant mais plutot orientee "capacites gouvernees" (resolve/attest/verify/role).
  - Pas encore un moteur de session produit multi-facteurs complet.

### 1.4 Gouvernance Cores

- StrongFather / MasterButler / WorrySentinel disponibles dans le workspace.
- Bornage deja clair dans la doc: Connect atteste identite/session; decision metier finale conservee par gouvernance.

## 2) Gaps critiques identifies

| Domaine | Etat actuel | Gap | Priorite |
|--------|-------------|-----|----------|
| Hash mot de passe Central | SHA256 en prod locale | Migrer vers Argon2id + migration progressive | P0 |
| MFA locale unifiee | Partielle selon services | Standardiser TOTP + passkey/hardware key | P0 |
| Niveaux AAL | Non unifies | Definir moteur AAL1..AAL4 + claims | P0 |
| Step-up dynamique | Non centralise | Ajouter API et UX step-up par criticite | P0 |
| Anti 2-temps en isolement | Non implemente | Gel enrolment, integrity binding, suspicious mode | P0 |
| Audit auth inviolable | Journalisation eparse | Chaine de hash locale + verif post-reconnexion | P1 |
| Couplage Central+Connect | Non implemente | Bootstrap/handshake runtime | P0 |
| Probe Origin capacites | Non implemente | Ping + cache capabilities + timeout | P0 |

## 3) Prerequis techniques detailles

### 3.1 Data model Connect (nouveau)

Tables minimales:

1. `connect_identities` (subject, email, status, created_at).
2. `connect_credentials_password` (argon2 hash, version, migrated_from).
3. `connect_totp_secrets` (secret chiffre, recovery codes hash).
4. `connect_passkeys` (credential_id, public_key, counter, device_meta).
5. `connect_sessions` (session_id, subject_id, aal, methods, issued_at, idle_exp, abs_exp, state).
6. `connect_step_up_events` (session_id, required_aal, result, timestamp).
7. `connect_integrity_snapshots` (session_id, host_fingerprint, policy_hash, monotonic_counter).
8. `connect_audit_chain` (event_id, prev_hash, event_hash, payload, ts).

### 3.2 API contract prerequis (v1)

1. Bootstrap/etat: `/v1/connect/bootstrap`, `/health`, `/capabilities`.
2. Auth: `/auth/initiate`, `/auth/verify`, `/auth/step-up`, `/auth/logout`.
3. Session: `/session/current`, `/session/introspect`, `/session/revoke`.
4. Origin: `/origin/ping`, `/origin/capabilities-cache`.

### 3.3 Security prerequis

1. Argon2id obligatoire pour mots de passe locaux.
2. Comparaison constant-time partout.
3. Rotation session id apres elevation de privilege.
4. Idle timeout court + absolute timeout.
5. Device binding/session integrity fingerprint (minimum software fingerprint v1).
6. En mode isolation: blocage enrolment facteurs faibles et recovery cloud.

### 3.4 UX/UI prerequis

1. Composants integrables: frame, modal, full-screen.
2. Indicator runtime: ONLINE_FULL / ONLINE_DEGRADED / ISOLATED / SUSPICIOUS.
3. Ecran step-up dedie pour actions sensibles.
4. Ecran recovery degrade (sans facteur cloud).

### 3.5 Infra & run prerequis

1. Demarrage couple Central + Connect (ordre deterministe).
2. Probe Origin timeout <= 1200 ms + retry backoff.
3. Mode fully-local si Origin indisponible.
4. Config locale signee/hashable (policy pack).

### 3.6 Tests prerequis

1. Tests unitaires policy AAL et mapping permission_tier.
2. Tests integration online/degraded/isolated.
3. Test attaque en 2 temps (scenario simulation):
   - intrusion reseau,
   - passage offline,
   - tentative d'elevation locale,
   - detection + blocage + audit.
4. Test re-synchronisation securisee au retour reseau.

## 4) Dependances candidates (a valider en T6)

1. `argon2` (si non deja mutualise dans module cible).
2. `totp-rs` pour TOTP RFC6238.
3. `webauthn-rs` (ou equivalent Rust) pour passkey/FIDO2.
4. `zeroize` pour secrets memoire.
5. `subtle` pour comparaisons constant-time.

Note: aucune dependance cloud obligatoire pour le chemin critique.

## 5) Plan de migration prerequis (legacy -> Connect)

1. Conserver login actuel Central comme filet de secours temporaire.
2. Ajouter migration transparente SHA256 -> Argon2id a la prochaine authentification valide.
3. Basculer UI Central vers Connect progressivement via feature flag.
4. Activer enforcement AAL par service par vagues.

## 6) Definition of Ready pour P3

T4 considere pret si:

1. Gaps P0 identifies et priorises.
2. Modele donnees cible decrit.
3. Contrat API v1 balise.
4. Exigences securite offline/2-temps explicites.
5. Strategie de migration legacy etablie.

## Decision T4

- Inventaire prerequis termine.
- Go pour T5 (analyse securite detaillee).
