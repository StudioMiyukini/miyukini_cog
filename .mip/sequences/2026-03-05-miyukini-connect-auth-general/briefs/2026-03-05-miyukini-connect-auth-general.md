# Brief P0 - Miyukini Connect (service d'authentification general)

<!-- @id: mip.brief.miyukini-connect.auth-general -->
<!-- @do: lancer_sequence_mip_connect_auth_online_offline -->
<!-- @role: Maria (Chef de Projet) -->
<!-- @layer: S7-Operator -->

## Classification

- Classe MIP : T4 (feature strategique securite + UX transverse)
- Date : 2026-03-05
- Sequence : `.mip/sequences/2026-03-05-miyukini-connect-auth-general/`
- Statut : APPROUVE CONDITIONNEL (fin P0)
- Mode autonomie recommande : BIG_STEPS

## TL;DR

Miyukini Connect devient la porte d'entree unique des sessions utilisateur du COG, lance en meme temps que Central.
Le service respecte les LOI COG: operation complete avec internet, operation complete en isolation, degradation explicite et tracable.
Les permissions de session sont graduees par niveau d'assurance d'authentification (AAL1 a AAL4).
Au boot, Connect interroge Origin pour savoir si les capacites renforcees (email verification, QR, futur Miyukini Authenticator) sont disponibles.
Les services sensibles imposent du step-up fort (passkey/hardware key/TOTP robuste), meme si cela est plus contraignant.

## Contexte

Tu demandes un service unifie d'authentification pour tout Miyukini COG:

1. Plusieurs moyens d'authentification standardises.
2. Niveau de permission de session derive du moyen utilise.
3. Exigence de robustesse accrue pour donnees sensibles.
4. API appelees par les autres services.
5. UI/UX integrable en frame ou en ecran natif.
6. Couplage de demarrage avec Central.
7. Ping Origin au lancement pour les capacites renforcees.

## Contraintes LOI COG retenues

| Loi | Application dans Miyukini Connect |
|-----|-----------------------------------|
| LOI-1 | Aucune dependance externe critique: auth locale possible sans Origin |
| LOI-2 | L'isolement est normal: mode offline natif, sans crash ni blocage global |
| LOI-3 | Etat local souverain: sessions, secrets, politiques stockes localement |
| LOI-6 | Federation sans perte autonomie: Origin ajoute des capacites, ne bloque pas le local |

## Objectifs

### Objectifs principaux

1. Etablir un moteur d'auth multi-facteurs avec niveaux d'assurance (AAL).
2. Standardiser les claims de session: `aal`, `methods`, `permission_tier`, `requires_step_up`.
3. Fournir des API d'auth/session/introspection pour tous les services.
4. Definir une UX d'auth composable (frame, modal, ecran complet).
5. Garantir un comportement coherent online/offline avec politique de degradation explicite.

### Objectifs secondaires

1. Preparer l'arrivee du service futur "Miyukini Authenticator".
2. Ajouter des metriques securite et UX pour piloter la qualite.
3. Mettre en place une trajectoire de migration depuis les auth locales existantes.

## Niveaux d'assurance proposes (AAL)

| Niveau | Methodes minimales | Usage |
|--------|--------------------|-------|
| AAL1 | Secret local (mot de passe/PIN) | Services standard a faible risque |
| AAL2 | Secret + 2e facteur local (TOTP) | Services standard avec ecriture |
| AAL3 | Passkey/WebAuthn locale ou QR challenge signe | Donnees sensibles |
| AAL4 | Hardware key ou double preuve forte + revalidation courte | Actions critiques/regaliennes |

Note: email OTP est accepte pour recovery/convenience online, mais ne doit pas etre la base des actions critiques.

## Decisions d'architecture retenues

1. **Connect comme facade produit**, s'appuyant sur les briques auth deja presentes (ex: MiyuAuth) pour eviter duplication.
2. **Autorisation finale conservee par la gouvernance** (StrongFather/MasterButler), Connect fournit des preuves de session normalisees.
3. **Step-up dynamique**: un service peut exiger un AAL superieur sans casser la session courante.
4. **Boot couple Central + Connect**: Central ouvre l'ecran session via Connect, puis recupere les claims de permission.
5. **Probe Origin au demarrage**: activation conditionnelle des canaux `email`, `qr`, `authenticator`.

## Decision finale P0

### Verdict

- **GO conditionnel vers P3**.

### Preconditions obligatoires

1. C1: migration hash legacy SHA256 -> Argon2id.
2. C2: policy AAL normative gelee/versionnee.
3. C3: integrite session + audit chain v1.
4. Pipeline CI dedie Connect avec checks C1/C2/C3 bloquants avant G3.
5. Tests offline/isolated obligatoires avant G5.

### Clauses d'arret

1. Echec PASS-0 a G1/G2 => arret et correction immediate.
2. Echec test attaque 2-temps a G4 => blocage G5.
3. Regression offline critique => retour etape precedente.

## References P0

1. GPI: `gpi/2026-03-05-miyukini-connect-auth-general-gpi.md`
2. Spec: `specs/2026-03-05-miyukini-connect-auth-general-spec.md`
3. Plan P3: `plans_p3/2026-03-05-miyukini-connect-auth-general-plan.md`
4. Audit faisabilite: `phases/p0/temps/temps-09-faisabilite.md`
5. CI/CD readiness: `phases/p0/temps/temps-10-cicd.md`
