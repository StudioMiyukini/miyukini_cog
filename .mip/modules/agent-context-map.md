# MIP — Carte de contexte agents

> **Source unique** pour les règles de chargement par phase et par agent.
> **1 Read = 1 besoin.** Ne jamais charger "par précaution".
> Certifications : `token-loading.md` section "Certifications" (max 2 simultanées, via load-map.json).
> Agents : `agents/INDEX.md` pour les règles d'escalade phase → FULL.

---

## Règles de base (non négociables)

| # | Règle |
|---|-------|
| 1 | Lire `p0-details-index.md` AVANT `p0-details.md` — drill-down par temps (offset/limit) |
| 2 | Max 2 certifications simultanées par agent — choisir via load-map.json |
| 3 | MASS : contexte < 80 lignes par worker (1 tâche isolée + 1 cert max) |
| 4 | Escalader FULL_agent.md uniquement si version phase insuffisante (règle agents/INDEX.md) |
| 5 | `patterns-and-lessons.md` : charger 1× au démarrage P3, drill-down section si besoin |
| 6 | `metrics/` : lire résumé totaux seulement si fichier > 200 lignes |

---

## Matrice de chargement par phase

| Phase | Agent | Fichiers à charger | Taille cible |
|-------|-------|-------------------|--------------|
| **P0 T1** | Maria | `p0-details-index.md` | 80 l |
| **P0 T2** | Maria + Lise | `p0-details.md` offset T2 | ~30 l |
| **P0 T3** | Fabrice | `p0-details.md` offset T3 | ~40 l |
| **P0 T4** | Denis + Hugo + Jean | `p0-details.md` offset T4, `project-file-map.md` | ~50 + 80 l |
| **P0 T5** | Victor | `p0-details.md` offset T5, `security-patterns.md` | ~60 + 100 l |
| **P0 T6** | François | `p0-details.md` offset T6, `stack-patterns.md`, Context7 | ~50 + 100 l |
| **P0 T7** | Maria | `agents/INDEX.md`, `agents/TEMPLATE_PHASE_AGENT.md` | 30 + 50 l |
| **P0 T8** | Denis | `p0-details.md` offset T8, `project-file-map.md` | ~55 + 80 l |
| **P0 T9** | Arianne + Jean | `p0-details.md` offset T9, `mip-decisions.md` | ~25 + 80 l |
| **P0 T10** | Hugo | `p0-details.md` offset T10, `environment.md` (Infra) | ~15 + 60 l |
| **P0 T11** | Maria | `p0-details.md` offset T11 | ~70 l |
| **P3 init** | Denis | `p3-execution.md`, `project-file-map.md`, `patterns-and-lessons.md` | 200 + 80 + 35 l |
| **P3** | François | `stack-patterns.md`, `api-contracts.md`, `test-templates.md` | 100 + 50 + 50 l |
| **P3** | Lise | `stack-cheatsheet.md`, `api-contracts.md`, `project-file-map.md` | 80 + 50 + 80 l |
| **P3 MASS** | Bob | `agents/bob/MASS_bob.md`, 1 tâche isolée | < 80 l total |
| **P3 MASS** | François/Lise | `agents/light/{agent}.md`, 1 tâche isolée | < 80 l total |
| **P4** | Denis | `p4-p5-p6.md` (section Denis), `project-file-map.md` | 50 + 80 l |
| **P4** | George | `p4-p5-p6.md` (section George), `code-annotations-templates.md` | 30 + 50 l |
| **P4** | Victor | `p4-p5-p6.md` (section Victor), `security-patterns.md` | 30 + 100 l |
| **P4** | Jean | `p4-p5-p6.md` (section Jean), `<seq>/metrics/` résumé | 20 + 50 l |
| **P4** | Hugo | `p4-p5-p6.md` (section Hugo), `environment.md` (Infra) | 20 + 60 l |
| **P5** | Denis + George | `p4-p5-p6.md` (section P5) | 60 l |
| **P6** | Arianne | `p4-p5-p6.md` (section P6), `mip-decisions.md`, `patterns-and-lessons.md` | 40 + 80 + 35 l |
| **P6** | Jean | `mip-performance-history.md`, `<seq>/metrics/` résumé totaux | 80 + 50 l |

---

## Modèles d'escalade

| Situation | Action |
|-----------|--------|
| Phase file insuffisant | Escalader vers `FULL_<agent>.md` — annoncer la raison |
| > 2 certs requises | Choisir les 2 plus pertinentes (priorité : tâche spécifique > générique) |
| `p4-p5-p6.md` section inconnue | Lire offset 1 limit 20 pour obtenir le plan de navigation |
| Fichier > 400 lignes | Lire uniquement la section pertinente (offset/limit) |
| MASS tâche simple (<30 min, 1-2 fichiers) | Assigner à Bob — contexte minimal MASS_bob.md |
