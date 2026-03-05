# MIP v2 - Miyukini Implementation Protocol

Protocole de developpement orchestre pour l'equipe Miyukini AI Studio.

## Principe

Chaque demande est classee (T1-T5) et traverse les phases necessaires avec des quality gates obligatoires. Tous les artefacts sont structures par sequence.

## Classification

| Classe | Description | Phases |
| ------ | ----------- | ------ |
| T1 | Micro-fix (<20 lignes, 1 fichier) | P3 -> P5 |
| T2 | Fix cible (1-3 fichiers) | P3 -> P5 |
| T3 | Feature moderee (3-10 fichiers) | P0 -> P3 -> P4 -> P5 -> P6 |
| T4 | Feature majeure (10+ fichiers) | P0 -> P3 -> P4 -> P5 -> P6 |
| T5 | Chantier strategique | P0 -> P3 -> P4 -> P5 -> P6 |

Note T2 : mini-plan de Denis en entree de P3 (equivalent ancien P2), puis execution.

## Phases

| Phase | Nom | Agents | Gate |
| ----- | --- | ------ | ---- |
| P0 | Cadrage (10 temps) | Maria (lead) + Lise, Fabrice, Denis, Francois, Victor, Hugo, Jean, Arianne | Brief approuve + autonomie |
| Git | Branche feature | Denis | feat/slug creee |
| P3 | Implementation | Francois + Lise (Denis coordonne) | Tests + clippy |
| P4 | Integration & Audit | Denis + George + Victor + Hugo + Jean | 0 defaut bloquant |
| P5 | Livraison | Denis + George | Verdict utilisateur (ACCEPTE/REFUSE) |
| P6 | Archivage | Arianne + Jean | Memoire mise a jour |

## Structure par sequence

Chaque sequence MIP a son dossier : `.mip/sequences/YYYY-MM-DD-<slug>/`

| Dossier | Contenu |
| ------- | ------- |
| `briefs/` | Briefs de cadrage (P0) |
| `specs/` | Specifications techniques (P0 T6) |
| `gpi/` | Gouvernance, pilotage, initiatives |
| `phases/` | Traces, livrables, DAG (`dag.json`) |
| `plans_p3/` | Plans d'execution atomiques |
| `audits/` | Rapports d'audit (P4) |
| `metrics/` | Metriques de la sequence |
| `rapports_finaux/` | Rapport P6 |
| `ressources/` | Index des ressources (voir `ressources/index.md`) |

**Partages** (racine `.mip/`) : `memory/`, `skills/`, `modules/`, `config/`

## Abonnements et quotas tokens

L'utilisateur peut renseigner ses abonnements (Anthropic, OpenAI, Gemini, Moonshot/Kimi, Z, etc.) et ses tokens disponibles dans `.mip/config/subscriptions.md` pour :
- Estimer la consommation avant une sequence (P0)
- Comparer consommation vs quota (rapport P6)
- Recevoir une alerte si >80 % du quota consomme

Creer le fichier a partir de `.mip/config/subscriptions.example.md`.

## Profils MIP (bascule outil/LLM)

Basculer entre Claude Code, Cursor, Codex, Ollama, LM Studio, solution interne. MIP s'adapte aux capacites du profil.

- **Profil actif** : `.mip/profiles/active` (defaut : `anthropic-opus`)
- **Commandes** : `/mip_profile`, `/mip_profile <slug>`, `/mip_profile list`
- **Index** : `.mip/profiles/INDEX.md`

## References

- **Workflow complet** : `.mip/modules/workflow.md`
- **Profils** : `.mip/profiles/INDEX.md`
- **Conventions** : `.mip/protocol/conventions.md`
- **Skill MIP** : `.mip/skills/miyukini-mip-workflow/SKILL.md`

## Lint de coherence

Commande de verification rapide du protocole :

`powershell -ExecutionPolicy Bypass -File .mip/scripts/lint-mip-coherence.ps1`
