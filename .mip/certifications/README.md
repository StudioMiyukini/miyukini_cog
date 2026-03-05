# Certifications MIP

> Referentiels par agent. Chargement selectif par tache via `load-map.json`.

## Usage IA

1. Identifier agent + type de tache.
2. Resoudre `load-map.json` vers les `cert_id` pertinents.
3. Charger uniquement les `REFERENCE.md` via `registry.json`.
4. Utiliser `KNOWLEDGE.md` quand il faut preparer une obtention/certification.
5. Synchroniser les passeports via `powershell -File .mip/scripts/sync-cert-diplomas.ps1`.

## Fichiers

| Fichier | Role |
|---------|------|
| `INDEX.md` | Index lisible par agent |
| `registry.json` | Source de verite (cert_id -> folder, agent, tldr) |
| `load-map.json` | Tache -> cert_ids |
| `SCHEMA.md` | Structure, conventions MSCM |
| `agent-certification-protocol/INDEX.md` | Version decoupee du protocole agent (canonique) |
| `monolithiques_decoupes/INDEX.md` | Decoupage des gros fichiers monolithiques |
| `sources/{agent}.md` | Sources officielles compactes par agent |
| `diplomas/INDEX.md` | Index des passeports agents (diplome simulation) |
| `diplomas/{agent}.md` | Passeport agent avec parcours certifs |
| `legal/OBLIGATIONS.md` | HDS, NF525, NF203, RGPD |
| `{folder}/REFERENCE.md` | Referentiel court pour chargement IA |
| `{folder}/KNOWLEDGE.md` | Connaissances requises pour obtention (modules + preuves) |
| `{folder}/academy/*` | Modules de formation locale + assessment + capstone |

## Statistiques

37 certifications, 10 agents. 2 obligatoires sous conditions (HDS, NF525).


## Dossier critique cyber

- `.mip/certifications/critical_cyber/INDEX.md` : base atomique pour verification des controles cyber certifiants.


