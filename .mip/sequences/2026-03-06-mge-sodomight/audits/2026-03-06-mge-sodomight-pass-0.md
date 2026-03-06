# PASS-0 securite mge-sodomight

## Statut

- Etat : COMPLET
- Phase : P4
- Responsable principal : Victor
- Date : 2026-03-06

## TL;DR

PASS-0 securite : inventaire des surfaces d'attaque et threat modeling initial pour `mge-sodomight`. Perimetre P3 (standalone + packaging Central).

## 1. Surfaces d'attaque recensees

Selon `phases/p0/temps/temps-05-securite.md` :

| Surface | Presente en P3 | Risque actuel |
|---------|----------------|---------------|
| Packages `.msp` / binaires installes | OUI (dist/sodomight) | Moyen -- checksums non valides automatiquement |
| Manifests falsifies ou checksum absent | OUI (service.manifest.json) | Moyen -- verification cote Central a confirmer |
| Mods non signes | NON (pas de systeme de mods P3) | Aucun |
| Corruption de sauvegarde / injection stash | OUI (mge-save) | Faible -- format interne, pas de validation schema cote load |
| RPC locale Central <-> jeu | Partielle (IPC protocole a definir) | Faible P3, a surveiller P4 |
| Lobbies multijoueur / synchro d'etat | NON (reseau non integre P3) | Hors perimetre P3 |

## 2. Threat model STRIDE (perimetre P3)

| Menace STRIDE | Vecteur | Mitigation presente | Statut |
|---------------|---------|---------------------|--------|
| **S**poofing identite joueur | Save file modifie | Format interne non signe | A traiter P4 |
| **T**ampering donnees | Stash inject via save | Pas de signature save | A traiter P4 |
| **R**epudiation action | Pas de log d'action P3 | -- | Post-P4 |
| **I**nformation disclosure | Binaire client leak config | Config non sensible P3 | RAS |
| **D**enial of service | LCG unbounded seed | Seed serveur non injectee | A traiter P4 |
| **E**levation privilege | Central exec binaire | Trust implicite post-install | Faible -- sandbox a prevoir P4 |

## 3. Points positifs securite P3

- `unsafe_code = "forbid"` workspace-wide : **PASS** -- elimine toute la surface UB/memory corruption
- Pas de `unwrap()` en production : **PASS** -- clippy -D warnings enforce
- Pas de secrets en source (pas de cles, tokens, mots de passe hardcodes) : **VERIFIE**
- Pas d'URL hardcodees hors tests : **VERIFIE**
- LCG deterministe sans source externe non controlee : **OUI** (mais seed fixe en P3, a corriger P4)
- Pas de SQL, pas d'injection SQL possible : **OUI** (pas de base relationnelle)
- Pas de deserialisation JSON/binaire d'entrees non controlees en P3 : **OUI** (serde local uniquement)

## 4. Risques residuels P3 -> P4

| Risque | Severite | Traitement recommande |
|--------|----------|-----------------------|
| Save non signee | Moyen | HMAC ou signature ed25519 sur le blob save en P4 |
| Checksum package non verifie cote Central | Moyen | Valider signature manifeste avant exec en P4 |
| Seed LCG fixe en production | Eleve (equite) | Injection seed par serveur des P4 |
| Pas de sandbox binaire jeu | Faible | Prevoir AppContainer/seccomp en P5 |

```
[PHASE:P4] [AGENT:victor] [TASK:pass-0-securite]
Actions:
- Inventaire 6 surfaces d'attaque selon temps-05-securite.md
- Threat model STRIDE sur perimetre P3
- Verification invariants securite code (unsafe, unwrap, secrets, URLs)
- 4 risques residuels documentes avec severite et mitigation
Checks:
- unsafe_code forbid : PASS
- Pas de secrets en source : PASS
- Pas de unwrap() production : PASS
Status: DONE
```
