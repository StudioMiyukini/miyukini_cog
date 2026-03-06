# Runbook P3 - Miyukini Connect

## Objectif

Definir les procedures operationnelles minimales apres implementation P3:
incident auth, recovery mode isole, reconciliation post-retour Origin.

## 1) Incident auth critique

Declencheurs:
1. Echecs auth massifs.
2. Detection de sessions anormales.
3. Rupture integrite audit chain.

Actions:
1. Basculer runtime `SUSPICIOUS`.
2. Revoquer sessions non `basic`.
3. Forcer step-up AAL max pour operations critiques.
4. Exporter et figer la chain d'audit pour investigation.

Sortie:
1. Incident contient.
2. Journal d'actions complete.

## 2) Recovery en mode ISOLATED

Regles:
1. Ne jamais baisser les exigences AAL.
2. Interdire enrolment/recovery faible (`EmailOtp`, `QrSigned`).
3. Autoriser auth locale robuste (password+TOTP, passkey, hardware key).

Actions:
1. Confirmer runtime `ISOLATED`.
2. Continuer authentication locale selon policy.
3. Bloquer toute operation admin sans AAL requis.
4. Conserver tous les evenements dans audit chain locale.

## 3) Reconciliation au retour Origin

Prerequis:
1. `origin_ping` passe en `ONLINE_FULL` ou `ONLINE_DEGRADED`.
2. Integrite audit chain locale validee.

Actions:
1. Envoyer snapshot capacites Origin et comparer au cache local.
2. Revoquer sessions creees pendant isolation si signaux anormaux.
3. Recalculer claims runtime sur sessions actives.
4. Archiver rapport reconciliation.

Sortie:
1. Etat runtime coherent.
2. Sessions conformes policy versionnee.

## 4) Checklist pre-P4

1. `cargo test -p miyukini-connect` vert.
2. `cargo clippy -p miyukini-connect -- -D warnings` vert.
3. Gates G1..G5 marques PASS en trace P3.
4. Ecarts restants documentes pour integration UI Central en P4.
