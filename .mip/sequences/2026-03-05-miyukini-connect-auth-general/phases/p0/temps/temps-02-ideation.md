# P0 Temps 2 - Ideation

## Statut

- Etat : Termine
- Phase : P0 Temps 2
- Responsable principal : Maria/Lise
- Date : 2026-03-05

## TL;DR

Ideation terminee: solution retenue = Miyukini Connect comme facade d'authentification multi-facteurs avec moteur de politiques AAL (1 a 4), step-up dynamique, UI composable et mode offline natif.

## Idees candidates evaluees

### Option A - Facade Connect + policy engine (retenue)

- Connect orchestre methodes d'auth + sessions + claims.
- Niveaux AAL standardises.
- Step-up demande a la vollee selon sensibilite de l'action.
- Integration simple avec Central et les services.

### Option B - Moteur auth complet nouveau et autonome

- Controle maximal.
- Cout implementation + risque duplication eleves.

### Option C - Login minimal puis evolution tardive

- Time-to-first-delivery rapide.
- Risque securite et dette de migration plus forts.

## Direction UX retenue

1. 3 modes d'integration: frame, modal, full-screen.
2. Etat runtime visible: `ONLINE_FULL`, `ONLINE_DEGRADED`, `ISOLATED`.
3. Changement de methode simple en cas d'echec.
4. Parcours step-up explicite pour actions sensibles.

## Direction technique retenue

1. Claims session standards: `aal`, `methods`, `permission_tier`, `step_up_until`.
2. API v1: bootstrap, auth flow, introspection, step-up, origin capabilities.
3. Probe Origin au demarrage avec timeout court et cache local.
4. Fallback local by-default en cas d'absence Origin.

## Axes d'amelioration ajoutes (securite isolement)

1. Auth forte offline-native prioritaire pour donnees sensibles: passkey/FIDO2/hardware key.
2. MFA locale stricte: secret local + facteur local; email OTP reserve au recovery.
3. Sessions durcies: timeout inactif court, expiration absolue, rotation session id, re-auth sur action sensible.
4. Step-up obligatoire par criticite: AAL3/AAL4 pour operations sensibles et administratives.
5. Break-glass controle: double validation (2 facteurs forts distincts), tracabilite obligatoire.

## Axes d'amelioration ajoutes (attaque en 2 temps)

1. En mode isolation, bloquer l'enrolement de nouveaux facteurs et les recuperations faibles.
2. Lier la session a l'etat d'integrite du poste (attestation locale/empreinte d'etat).
3. Journal local append-only chaine par hash pour detection de tampering.
4. Mode suspicion automatique: exigence d'AAL plus elevee, voire lecture seule.
5. A la reconnexion, verification d'integrite et rotation/revocation preventive des sessions.

## Sorties T2

- Architecture candidate choisie (Option A).
- UX cible definie pour l'authentification integrable.
- Pre-requis de specification T3+ identifies.
