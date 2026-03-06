# P0 Temps 3 - Analyse concurrentielle

## Statut

- Etat : Termine
- Phase : P0 Temps 3
- Responsable principal : Fabrice
- Date : 2026-03-05

## TL;DR

T3 confirme que la meilleure direction pour Miyukini Connect est une architecture **local-first souveraine** avec MFA forte offline, step-up dynamique et mecanismes anti-tampering en isolement. Les approches cloud-centriques restent utiles en federation, mais insuffisantes comme socle de securite pour LOI-1/LOI-2 et pour le risque d'attaque en 2 temps.

## Hypothese d'analyse

Le besoin Miyukini Connect depasse un login classique: il faut maintenir une authentification robuste et une session fiable en modes `ONLINE_FULL`, `ONLINE_DEGRADED`, `ISOLATED`, y compris face a une compromission partielle avant isolement.

## Cadre de benchmark

Criteres compares (score 1 a 5):

1. Robustesse offline sur donnees sensibles.
2. Resistance a l'attaque en 2 temps (intrusion reseau puis abus en isolation).
3. Experience utilisateur (friction controlee).
4. Complexite d'integration avec Central + services.
5. Cout d'exploitation et gouvernance.

## Families de solutions comparees

### A. IAM cloud-centrique

- Principe: authentification majoritairement delegatee a un fournisseur distant.
- Avantages: federation, outillage mature, onboarding rapide.
- Limites: dependance reseau critique, recovery cloud, faible souverainete en isolement.

### B. IAM hybride avec cache local

- Principe: fournisseur distant + capacites offline partielles via cache.
- Avantages: meilleure continuite que A.
- Limites: coherence et gouvernance difficiles a garantir au retour reseau.

### C. Auth locale souveraine + MFA forte

- Principe: verification locale native (secret local + TOTP/passkey/FIDO2).
- Avantages: autonomie forte, tres bonne robustesse offline.
- Limites: UX potentiellement plus contraignante, gestion cycle de vie facteurs.

### D. Auth locale souveraine + step-up + integrity sealing (cible Miyukini Connect)

- Principe: C + politiques AAL dynamiques + session liee a l'integrite locale + logs inviolables.
- Avantages: meilleure posture face a l'attaque en 2 temps, bon compromis securite/UX.
- Limites: implementation plus exigeante au depart.

## Matrice de scoring

| Option | Offline sensible | Anti 2-temps | UX | Integration | Exploitation | Total |
|--------|------------------|--------------|----|-------------|--------------|-------|
| A Cloud-centrique | 1 | 2 | 4 | 4 | 3 | 14 |
| B Hybride cache local | 3 | 3 | 4 | 3 | 3 | 16 |
| C Locale souveraine + MFA | 5 | 4 | 3 | 3 | 3 | 18 |
| D Locale + step-up + integrity sealing | 5 | 5 | 4 | 3 | 3 | 20 |

## Enseignements competitifs

1. Les solutions cloud-centriques sont performantes pour federation et comfort, mais non alignes seules avec l'autonomie COG.
2. Le cache local hybride ameliore la continuite mais ne traite pas completement le risque de compromission avant isolement.
3. La combinaison **MFA forte locale + step-up + verification d'integrite** est la plus defendable pour les donnees sensibles.
4. Le cout UX peut rester acceptable si la contrainte forte est reservee aux actions critiques.

## Recommandation T3 (validee)

Retenir l'option **D** comme cible Miyukini Connect:

1. Base auth locale souveraine (AAL1..AAL4).
2. Step-up obligatoire pour `permission_tier` sensible/critique.
3. En isolement: gel d'enrolement, blocage recovery faible, mode suspicion possible.
4. Journalisation locale append-only avec verification au retour reseau.
5. Federation Origin uniquement comme extension de capacites, jamais comme prerequis vital.

## Impacts sur la suite P0

- T4 devra inventorier les prerequis techniques pour:
  - passkeys/FIDO2/hardware key,
  - scellage d'integrite session,
  - journaux inviolables et verification post-reconnexion,
  - politique AAL -> permission_tier inter-services.

## Decision T3

- Analyse concurrentielle terminee.
- Direction architecture confirmee: **local-first durci + step-up + anti-2-temps**.
- Passage recommande vers T4.
