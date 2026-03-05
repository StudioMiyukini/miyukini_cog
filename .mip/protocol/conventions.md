# Conventions MIP - Tables de reference partagees

> Ce fichier remplace les references a CLAUDE.md pour les sections utilisees par le protocole MIP. Toutes les references sont internes a `.mip/`.

---

## Classification des taches (T1-T5)

| Classe | Critere | Phases |
|--------|---------|--------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 -> P5 |
| **T2** | Fix cible, 1-3 fichiers | P3 -> P5 |
| **T3** | Feature moderee, 3-10 fichiers | P0 -> P3 -> P4 -> P5 -> P6 |
| **T4** | Feature majeure, 10+ fichiers | P0 -> P3 -> P4 -> P5 -> P6 |
| **T5** | Chantier strategique | P0 -> P3 -> P4 -> P5 -> P6 |

**Qui classifie** : Maria. En cas de doute, classer **UN CRAN AU-DESSUS**.

Note T2 : mini-plan de Denis au demarrage de P3 (pas de phase P2 distincte).

---

## Equipe MIP (10 agents coeur + Bob optionnel MASS)

| Agent | Role | Phases principales |
|-------|------|-------------------|
| Maria | Chef de projet, orchestration | P0 (lead), P5 (boucle si refus) |
| Fabrice | Analyse concurrentielle | P0 Temps 3 (T4-T5) |
| Denis | Chef dev, architecture | P0 T4&T8, P3, P4, P5 |
| Lise | Dev front-end | P0 T2, P3 |
| Victor | Expert cybersecurite | P0 T5 (T3+), P4 |
| Francois | Dev back-end | P0 T6 (spec), P3 |
| George | Audit conformite | P4 |
| Hugo | DevOps, infra | P0 T4&T10 (T4-T5), P4 |
| Jean | Efficience tokens | P0 T9, P4, P6 |
| Arianne | Team manager, QA, memoire | P0 T9, P6 |
| Bob (optionnel) | Codeur leger (MASS, T1-T2) | P3 workers |

References agents :
- index canonique : `.mip/agents/INDEX.md`
- version complete : `.mip/agents/<agent>/FULL_<agent>.md`
- version phase : `.mip/agents/<agent>/<PHASE>_<agent>.md`

Bornage protocolaire (obligatoire) :
1. Charger d'abord la version phase de l'agent.
2. Charger `FULL_<agent>.md` uniquement en escalade justifiee.
3. Ne pas charger plusieurs `FULL_*.md` simultanement sans justification explicite.
4. Bob est actif uniquement si MASS est actif (T4-T5) ou tache simple explicite.

---

## Artefacts MIP

| Artefact | Chemin | Phase |
|----------|--------|-------|
| Briefs | `<sequence>/briefs/` | P0 |
| RPS (preliminaire securite) | `<sequence>/briefs/` (integre au brief P0) | P0 T5 |
| Specifications | `<sequence>/specs/` | P0 T6 |
| Agents fine-tuned de sequence | `<sequence>/agents/` | P0 T7 |
| GPI securite (planification) | `<sequence>/gpi/` | P0 T5 |
| Plans | `<sequence>/plans_p3/` | P0 T8, P3 |
| Audits | `<sequence>/audits/` | P4 |
| PASS-0 (plan audit securite) | `<sequence>/audits/YYYY-MM-DD-<slug>-pass-0.md` | P4 |
| PASS-XX (rapports taches audit) | `<sequence>/audits/YYYY-MM-DD-<slug>-pass-xx.md` | P4 |
| RAS (rapport audit securite) | `<sequence>/audits/YYYY-MM-DD-<slug>-ras.md` | P4 |
| Metriques | `<sequence>/metrics/` | Toutes |
| Rapports | `<sequence>/rapports_finaux/` | P6 |
| DAG (MASS) | `<sequence>/phases/dag.json` | P0 T11, P3 |

> `<sequence>` = `.mip/sequences/YYYY-MM-DD-<slug>/`

---

## Workflow standard

1. **SETUP** (une fois) -> `.mip/environment.md`
2. **Classification** (Maria) -> T1-T5
3. **P0** -> Brief approuve (T3+)
4. **P3** -> Implementation + tests (T2 : mini-plan Denis au demarrage)
5. **P4** -> Integration + audit (PASS-0 -> PASS-XX -> RAS, score securite /100)
6. **P5** -> Livraison + test humain
7. **P6** -> Rapport + archivage (T3+)

Priorite de chargement des prompts agents en execution :
1. `<sequence>/agents/<PHASE>_<agent>.md`
2. `.mip/agents/<agent>/<PHASE>_<agent>.md`
3. `.mip/agents/<agent>/FULL_<agent>.md` (escalade justifiee uniquement)

---

## References internes .mip

| Contenu | Chemin |
|---------|--------|
| Skill MIP workflow | `.mip/skills/miyukini-mip-workflow/SKILL.md` |
| Modules (P0, P3, P4-P6, etc.) | `.mip/modules/` |
| Certifications | `.mip/certifications/` (voir `INDEX.md`) |
| Memoire projet | `.mip/memory/` |
| Secrets (auth GitHub, VPS, API) | `.mip/secrets/` - **dans .gitignore** |
| Config (abonnements tokens) | `.mip/config/` - `subscriptions.md` (quota par fournisseur) |
| Profils MIP (outil/LLM) | `.mip/profiles/` (voir `INDEX.md`) |
| Protocole | `.mip/protocol/` |
