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
| Denis | Chef dev, architecture | P0 T4&T7, P3, P4, P5 |
| Lise | Dev front-end | P0 T2, P3 |
| Victor | Expert cybersecurite | P0 T5 (T3+), P4 |
| Francois | Dev back-end | P0 T6 (spec), P3 |
| George | Audit conformite | P4 |
| Hugo | DevOps, infra | P0 T4&T9 (T4-T5), P4 |
| Jean | Efficience tokens | P0 T8, P4, P6 |
| Arianne | Team manager, QA, memoire | P0 T8, P6 |
| Bob (optionnel) | Codeur leger (MASS, T1-T2) | P3 workers |

Reference complete : `.mip/agents/*.md`. Bob est active uniquement si MASS est active (T4-T5) ou tache simple explicite.

---

## Artefacts MIP

| Artefact | Chemin | Phase |
|----------|--------|-------|
| Briefs | `<sequence>/briefs/` | P0 |
| Specifications | `<sequence>/specs/` | P0 T6 |
| Plans | `<sequence>/plans_p3/` | P0 T7, P3 |
| Audits | `<sequence>/audits/` | P4 |
| Metriques | `<sequence>/metrics/` | Toutes |
| Rapports | `<sequence>/rapports_finaux/` | P6 |
| DAG (MASS) | `<sequence>/phases/dag.json` | P0 T10, P3 |

> `<sequence>` = `.mip/sequences/YYYY-MM-DD-<slug>/`

---

## Workflow standard

1. **SETUP** (une fois) -> `.mip/environment.md`
2. **Classification** (Maria) -> T1-T5
3. **P0** -> Brief approuve (T3+)
4. **P3** -> Implementation + tests (T2 : mini-plan Denis au demarrage)
5. **P4** -> Integration + audit
6. **P5** -> Livraison + test humain
7. **P6** -> Rapport + archivage (T3+)

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
