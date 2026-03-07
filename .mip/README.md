# MIP v2 - Miyukini Implementation Protocol

Protocole de pilotage de sequences de dev assistees IA, structure par phases, gates et artefacts tracables.

## TL;DR

- MIP organise tout travail en sequence `P0 -> Git -> P3 -> P4 -> P5 -> P6` (T3+).
- Chaque demande est classee `T1` a `T5` avant execution.
- Chaque sequence a son dossier dedie sous `.mip/sequences/YYYY-MM-DD-<slug>/`.
- Les decisions, preuves, metriques et rapports sont conserves dans les artefacts de sequence.
- MIP est profile-aware (outil/LLM) et autonomie-aware (FULL/BIG_STEPS/GUIDED).

---

## 1) A quoi sert MIP

MIP sert a industrialiser l'execution IA sans perdre la gouvernance:

- cadrage explicite avant code (P0),
- execution disciplinee (TDD, checkpoints, audits),
- validation humaine obligatoire (P5),
- capitalisation systematique (P6).

Resultat: moins d'improvisation, moins de rework, meilleure auditabilite.

---

## 2) Avantages concrets

| Axe | Apport MIP |
| --- | --- |
| Predictibilite | Phases invariantes + gates explicites |
| Qualite | TDD en P3 + audits P4 + validation P5 |
| Tracabilite | Artefacts versionnes par sequence |
| Securite | RPS en P0 T5, PASS-0/PASS-XX/RAS en P4 |
| Collaboration | Roles agents clairs + handoffs formels |
| Portabilite | Protocole independant de la stack et du LLM |
| Pilotage cout IA | Metriques tokens, efficience et quotas |

---

## 3) Concepts de base (nomenclature canonique)

```text
Sequence MIP
  -> Phase (P0, Git, P3, P4, P5, P6)
     -> Temps (P0 uniquement)
     -> Etape (P3 uniquement)
     -> Volet (P4/P5/P6)
        -> Tache (unite atomique)
```

Regles:

- "Temps" est reserve a P0.
- "Etape" est reserve a P3.
- "Volet" est reserve a P4/P5/P6.

---

## 4) Flux global du protocole

```text
SETUP (une fois)
   |
   v
Classification T1-T5 (Maria)
   |
   +--> T1/T2: P3 -> P5
   |
   +--> T3/T4/T5:
         P0 -> Git -> P3 -> P4 -> P5 -> P6
                         |      |      |
                         |      |      +-> verdict humain
                         |      +-> audit conformite + securite + efficience
                         +-> implementation TDD + checkpoints
```

---

## 5) Classes de taches

| Classe | Critere | Phases |
| --- | --- | --- |
| T1 | Micro-fix, 1 fichier, <20 lignes | P3 -> P5 |
| T2 | Fix cible, 1-3 fichiers | P3 -> P5 |
| T3 | Feature moderee, 3-10 fichiers | P0 -> Git -> P3 -> P4 -> P5 -> P6 |
| T4 | Feature majeure, 10+ fichiers | P0 -> Git -> P3 -> P4 -> P5 -> P6 |
| T5 | Chantier strategique | P0 -> Git -> P3 -> P4 -> P5 -> P6 |

En cas de doute, classer un cran au-dessus.

---

## 6) Phases et gates

### SETUP (une seule fois)

- Cree/maj la config de reference (`.mip/environment.md`).
- Detecte environnement, stack, outil IA, preferences utilisateur.
- Initialise profils, agents et memoire de projet.

### P0 - Cadrage (11 Temps, T3+)

- Phase humaine obligatoire.
- Produit brief, spec, plan, risques, ressources, RPS securite.
- Gate: brief approuve + mode autonomie choisi.
- Artefacts initialises par script 1 (avant P0) et script 2 (fin T2). Aucune action supplementaire post-gate.

### Git

- Creation branche `feat/<slug>`.

### P3 - Implementation

- Execution back/front selon plan.
- TDD obligatoire.
- Checkpoints periodiques.
- Gate: tests + lint verts sur scope.

### P4 - Integration et audit

- Build/tests/lint globaux.
- Audit conformite (George).
- Audit securite /100 (Victor) + PASS-0/PASS-XX/RAS.
- Audit efficience tokens (Jean).
- Gate: 0 defaut bloquant + score securite conforme.

### P5 - Livraison et validation utilisateur

- Presentation livrable.
- Test humain guide.
- Verdict: `ACCEPTE`, `ACCEPTE AVEC RESERVES`, `REFUSE`.

### P6 - Rapport et capitalisation (T3+)

- Rapport final.
- Mise a jour memoire projet.
- Historisation des metriques et lecons apprises.

---

## 7) Modes d'autonomie (execution de sequence)

| Mode | Fonctionnement |
| --- | --- |
| FULL | Execution continue apres P0, intervention humaine minimale jusqu'a P5 |
| BIG_STEPS | Validation humaine entre grandes phases |
| GUIDED | Validation reguliere, supervision rapprochee |

Ce mode n'est pas le meme concept que le "mode profil LLM" (Mode 1-5 dans `ADAPTIVE-MODES`).

---

## 8) Structure standard d'une sequence

```text
.mip/sequences/YYYY-MM-DD-<slug>/
  briefs/
  specs/
  gpi/
  phases/            (traces + dag.json)
  plans_p3/
  audits/
  metrics/
  rapports_finaux/
  ressources/
  agents/            (prompts agents fine-tuned, C4+)
  ui/                (index.html + manifest.json)
```

Dossiers partages (racine `.mip/`):

- `memory/`
- `skills/`
- `modules/`
- `config/`

---

## 9) Interfaces graphiques MIP (sans build)

### Portail racine

- Fichier: `.mip/index.html`
- Affiche dynamiquement les sequences (via `sequences/index.json`)
- Tri/filtre par metadonnees MSCM, dates, type T1-T5, nom.

### Mini-site sequence

- Fichier: `.mip/sequences/<sequence>/ui/index.html`
- Onglets standardises: `P0`, `P3`, `P4`, `P5`, `Rapport final`.
- Lecture des artefacts via `ui/manifest.json`.

### Setup/config UI

- Fichier: `.mip/tools/mip-ui/index.html`

### Note execution navigateur

Pour eviter les erreurs `file://` + CORS:

```powershell
cd .mip
.\start-portal.cmd
```

Puis ouvrir `http://127.0.0.1:8765/index.html`.

---

## 10) Scripts utiles

| Script | Usage | Quand |
| --- | --- | --- |
| `.mip/scripts/init-sequence-base.ps1` | Creer les artefacts de base (brief, metriques, T1, T2) | Avant P0 (obligatoire) |
| `.mip/scripts/init-sequence-by-complexity.ps1` | Creer les artefacts selon la complexite C1-C5 | Fin de T2 (obligatoire) |
| `.mip/scripts/rebuild-sequences-index.ps1` | Regenerer `sequences/index.json` | Apres creation/archivage sequence |
| `.mip/scripts/lint-mip-coherence.ps1` | Verifier la coherence protocole | Sur demande |
| `.mip/start-portal.ps1` / `.mip/start-portal.cmd` | Servir le portail MIP localement | Pour consulter l'UI |

Exemples:

```powershell
# Script 1 - avant P0
powershell -ExecutionPolicy Bypass -File .mip/scripts/init-sequence-base.ps1 -SequencePath .mip/sequences/2026-03-05-mon-slug

# Script 2 - fin T2, apres estimation de la complexite
powershell -ExecutionPolicy Bypass -File .mip/scripts/init-sequence-by-complexity.ps1 -SequencePath .mip/sequences/2026-03-05-mon-slug -Complexity C3

powershell -ExecutionPolicy Bypass -File .mip/scripts/rebuild-sequences-index.ps1
powershell -ExecutionPolicy Bypass -File .mip/scripts/lint-mip-coherence.ps1
```

---

## 11) Profils (outil/LLM) et adaptivite

- Profil actif: `.mip/profiles/active`
- Index profils: `.mip/profiles/core/INDEX.md`
- Capacites et degradation: `.mip/profiles/ADAPTIVE-MODES.md`
- Negotiation de capacites: `.mip/profiles/CAPABILITY-NEGOTIATION.md`

Le protocole MIP reste identique; seules les capacites d'execution s'adaptent au profil.

---

## 12) Invariants de gouvernance (resume)

- Classification avant toute action.
- Pas de code avant gate P0 (T3+).
- TDD obligatoire en P3.
- Gate stricte entre phases.
- Test humain obligatoire en P5.
- Metriques mesurees, pas estimees.
- Roles agents fixes.
- Artefacts ranges par sequence.

---

## 13) MSCM et generateur d'index

Terminologie a jour:

- L'ancien nom "MIP generator" est obsolete.
- Le nom courant est "MSCM generator".
- Outil: `tools/mscm-generator/`.

---

## 14) Parcours rapide (nouvelle sequence)

```text
1. Classer la demande (T1-T5)
2. Creer sequence YYYY-MM-DD-<slug>
3. Script 1 : init-sequence-base.ps1          (cree brief, metriques, T1, T2)
4. Executer P0 T1 + T2 (si T3+ : T1-T11)
5. Script 2 : init-sequence-by-complexity.ps1  (fin T2, complexite estimee C1-C5)
6. Valider brief + choisir FULL/BIG_STEPS/GUIDED
7. Executer P3 -> P4 -> P5
8. Si accepte, finaliser P6
```

---

## 15) References canoniques

- Workflow principal: `.mip/modules/workflow.md`
- Conventions partagees: `.mip/protocol/conventions.md`
- Setup detaille: `.mip/modules/setup.md`
- P3 detaille: `.mip/modules/p3-execution.md`
- P4/P5/P6 detaille: `.mip/modules/p4-p5-p6.md`
- Profils: `.mip/profiles/core/INDEX.md`
- Skill workflow: `.mip/skills/miyukini-mip-workflow/SKILL.md`
