# P0 Temps 5 - Analyse securite

## Statut

- Etat : Termine
- Phase : P0 Temps 5
- Responsable principal : Victor
- Date : 2026-03-05

## TL;DR

Analyse securite detaillee terminee pour Miyukini Connect (option D). Le risque majeur est l'attaque en 2 temps: compromission online preparatoire puis abus en mode isole. La strategie retenue combine MFA forte locale, step-up systematique selon criticite, session liee a l'integrite locale, journal inviolable et verification agressive a la reconnexion.

## 1) Actifs critiques a proteger

1. Secrets d'authentification (hash mots de passe, seeds TOTP, credentials passkey).
2. Sessions actives et claims (`aal`, `permission_tier`, `step_up_until`).
3. Politiques locales de securite (mapping AAL, seuils, mode suspicion).
4. Traces d'audit (preuves d'evenements auth et elevation).
5. Integrite runtime du service Connect (binaire + config + etat).

## 2) Adversaires et capacites

### A1 - Attaquant reseau opportuniste

- Interception/rejeu, brute force, credential stuffing.

### A2 - Attaquant cible avec persistence locale

- Prepare une porte d'entree online, attend le mode offline pour contourner les controles distants.

### A3 - Attaquant interne / poste compromis

- Modifie config/politiques, tente enrolment malveillant de facteurs, abuse une session existante.

## 3) Scenario critique: attaque en 2 temps

### Temps 1 (online)

1. Intrusion reseau initiale (credential vol, session theft, malware drop).
2. Implant cherche a persister et a attendre perte de connectivite.

### Temps 2 (isolation)

1. Tentative d'elevation locale (enrolment nouveau facteur faible / bypass step-up).
2. Utilisation de session stale pour actions sensibles.
3. Alteration des journaux pour effacer traces.

## 4) Analyse de risque (synthese)

| Risque | Probabilite | Impact | Niveau |
|--------|-------------|--------|--------|
| Vol session + reuse offline | Moyenne | Tres eleve | Critique |
| Enrolment malveillant en isolement | Moyenne | Tres eleve | Critique |
| Bypass step-up action sensible | Faible/Moyenne | Tres eleve | Critique |
| Tampering des traces auth | Moyenne | Eleve | Haut |
| Brute force credentials locaux | Moyenne | Eleve | Haut |

## 5) Controles obligatoires (MUST)

### 5.1 Prevention

1. Password hashing Argon2id (migration legacy SHA256 obligatoire).
2. MFA locale forte pour sensible: passkey/FIDO2/hardware key prioritaire.
3. Step-up obligatoire par `permission_tier`.
4. Rotation session id apres login et apres step-up.
5. Idle timeout court + absolute timeout strict.
6. Rate limit par compte, IP et facteur; lockout progressif.
7. En mode `ISOLATED`: blocage enrolment facteurs et blocage recovery faible (email OTP indisponible).

### 5.2 Detection

1. Detection anomalies auth: echecs repetes, geographie/device anormaux, step-up refuses.
2. Mode `SUSPICIOUS` automatique sur signaux forts.
3. Journal auth append-only chaine par hash (event -> prev_hash -> event_hash).
4. Compteur monotone session pour detecter rejeu.

### 5.3 Response

1. Revocation immediate des sessions suspectes.
2. Re-auth forte forcee pour toute operation sensible apres alerte.
3. Degradation securitaire: lecture seule partielle si suspicion severe.
4. Freeze operations critiques tant que step-up fort non valide.

### 5.4 Recovery

1. Reconciliation a la reconnexion: verification integrite journaux/politiques.
2. Rotation preventive des tokens/session keys apres retour online.
3. Purge/invalidations des sessions ouvertes avant incident.
4. Rapport d'incident local + export vers Origin quand disponible.

## 6) Exigences de conception securite pour T6/P3

1. `connect-policy-engine` doit exposer une table normative `permission_tier -> required_aal`.
2. `connect-session-engine` doit lier session a une empreinte d'integrite locale (v1 logicielle).
3. `connect-api` doit forcer `authorize-hint` avant action sensible.
4. `connect-origin-adapter` doit etre fail-safe: indisponibilite Origin ne reduit jamais les exigences locales.
5. `connect-ui-sdk` doit afficher explicitement l'etat `ISOLATED` et ses restrictions.

## 7) Criteres PASS securite (acceptance)

### PASS-0 (minimum go)

1. Argon2id active + migration legacy testee.
2. MFA locale operationnelle (TOTP minimum) + step-up actif.
3. Timeouts session + rotation session id verifies.
4. Rate limits et lockout verifies.

### PASS-01 (durcissement)

1. Passkey/FIDO2 ou hardware key pour AAL3/AAL4.
2. Blocage enrolment/recovery faible en isolation.
3. Chaine de hash audit active + verification integrite locale.
4. Test scenario attaque 2 temps passe.

### PASS-RAS (objectif stabilisation)

1. Zero bypass step-up sur test d'intrusion interne.
2. Zero elevation critique sans preuve AAL requise.
3. Reconciliation online post-incident stable et reproductible.

## 8) Residual risks

1. Compromission materielle profonde du poste (hors scope v1).
2. UX degradee pour users sensibles (cout acceptable vu criticite).
3. Besoin d'automatisation outillage forensic pour runbook P4/P5.

## Decision T5

- Analyse securite detaillee terminee.
- Exigences MUST et PASS definies.
- Go recommande pour T6 (spec technique detaillee avec exigences securite integrees).
