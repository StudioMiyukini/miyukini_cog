# Travail P0 - Miyukini Connect

## Synthese d'analyse

Miyukini Connect doit etre traite comme une couche produit d'orchestration d'identite:

1. Verification d'authentification et gestion session de bout en bout.
2. Publication de claims de niveau d'assurance de session.
3. Support natif des parcours step-up pour services sensibles.
4. Degradation offline explicite sans casser l'acces au COG local.

## Scenarios prioritaires

### Scenario S1 - Entree Central standard

- Central lance Connect au boot.
- Connect charge politique locale et facteurs disponibles.
- Si Origin disponible: enrichit capacites (`email`, `qr`, `authenticator`).
- Utilisateur se connecte (AAL1/AAL2).
- Central recoit session + claims.

### Scenario S2 - Demande sensible

- Un service demande `required_aal=3` ou `required_permission_tier=high`.
- Connect detecte insuffisance de session courante.
- Connect ouvre step-up UI (passkey/hardware key/TOTP fort).
- Session est elevee (ou jeton de step-up scope).

### Scenario S3 - Isolation totale

- Origin indisponible.
- Connect passe en mode `offline_local_only`.
- Login local continue avec methodes locales.
- Methodes online-only (email link) desactivees visiblement.

## Matrice permission-tier proposee

| permission_tier | Exigence minimum |
|-----------------|------------------|
| basic | AAL1 |
| standard_write | AAL2 |
| sensitive_read | AAL3 |
| sensitive_write | AAL3 + step-up recent |
| critical_admin | AAL4 + step-up immediat |

## Decisions UX

1. UI composable en 3 formats: frame embed, modal, full-screen.
2. Etat de connectivite affiche en permanence: online / degraded / isolated.
3. Message clair pour methodes indisponibles (ex: email OTP indisponible offline).
4. Option "Changer de methode" pour limiter abandon utilisateur.

## Questions de conception deja tranchees

- Oui au couplage de lancement Central + Connect.
- Oui au probe Origin au demarrage.
- Oui aux niveaux d'auth standardises AAL1..AAL4.
- Oui a la separation auth (Connect) / autorisation (gouvernance).
