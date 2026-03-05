# MIP — Chargement tokens par agent

> **Source unique** pour tables « Fichiers à charger par agent ». Autres docs (workflow, SKILL, conventions) renvoient ici.
> Charger UNIQUEMENT les fichiers listés. **1 Read = 1 besoin**. Si réponse ligne 10, ne pas charger 200 lignes.

---

## Compétences techniques (par agent)

| Agent | Fichiers |
|-------|----------|
| François | stack-patterns, api-contracts, test-templates, annotations (Miyukini: rust-patterns, mscm-templates) |
| Lise | stack-cheatsheet, api-contracts, project-file-map, annotations (Miyukini: dioxus-cheatsheet) |
| Denis | project-file-map, stack-patterns, mip-decisions, patterns-and-lessons |
| George | project-file-map, annotations, patterns-and-lessons |
| Victor | security-patterns, patterns-and-lessons, stack-patterns, project-file-map |
| Hugo | project-file-map, .mip/environment.md (Infra), mip-decisions |
| Jean | mip-performance-history, MEMORY.md (synthèse 1× en début P4/P6), \<sequence\>/metrics/ (résumé totaux si >200 lignes) |
| Arianne | mip-decisions, patterns-and-lessons, mip-performance-history, team-skills-audit |

Paths : `.mip/memory/`, `.mip/` (certifications, environment, agents). **MASS tâches simples** : `.mip/agents/bob.md` (~40 lignes, haiku). **MASS tâches complexes** : `.mip/agents/light/{francois,lise,victor}.md` (50-80 lignes).

---

## patterns-and-lessons — Sections ciblées

Charger 1 fois au démarrage P3 (Denis ou premier agent). Drill-down par section si besoin.

| Section | Lignes approx | Quand |
|---------|---------------|-------|
| Patterns confirmés | 11-26 | P3 code général |
| Erreurs à ne pas répéter | 27-35 | Revue sécurité, anti-patterns |
| Anti-patterns | 36-80 | MASS (AP-08, AP-09), merge, worktree |

---

## Certifications (chargement par tâche)

**Règle stricte** : Jamais charger une cert sans clé load-map correspondante. Pas de chargement « au cas où ».
**Limite** : Max 2 certs simultanées par agent. Si load-map en liste 3, choisir les 2 les plus pertinentes.

Index : `.mip/certifications/INDEX.md`. Load-map : `.mip/certifications/load-map.json`.

| Tâche/Contexte | Clé load-map | Certs (exemples) |
|----------------|--------------|------------------|
| François écrit tests | `francois.unit_test` | istqb/REFERENCE.md |
| François conçoit API | `francois.api_design` | openapi/REFERENCE.md |
| Lise crée composant UI | `lise.ui_component` | wcag, iso_9241 |
| Victor threat modeling | `victor.threat_model` | cissp, iso_27001 |
| Victor audit sécurité P4 | `victor.audit_securite` | iso_27001, ceh |
| George audit conformité | `george.audit_conformite` | iso_19011, cisa |
| Hugo Docker/K8s | `hugo.docker` / `hugo.kubernetes` | docker, cka |
| Jean efficience tokens | `jean.efficience_tokens` | prompt_eng, finops |
| Phase fallback | `p4.victor` etc. | 1 cert principal + 1 fallback si hybride |

**Workflow** : 1) Identifier agent + type tâche → 2) Lire load-map.json → 3) Charger max 2 REFERENCE.md listés.
