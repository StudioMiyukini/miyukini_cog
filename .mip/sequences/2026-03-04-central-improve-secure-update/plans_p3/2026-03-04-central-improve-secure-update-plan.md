# Plan P3 — Audit COG central-improve-secure-update

## TL;DR

6 phases : inventaire → scan monolithiques → plan découpage → indexation MSCM → conformité → réorganisation. Livrables : project-file-map, liste monolithiques, plan priorisé, index MSCM, checklist conformité, structure réorganisée.

---

## Étape 1 — Inventaire (Terminée)

- [x] project-file-map complété
- [x] Zones COG hors MGE identifiées
- [x] Fichiers critiques listés
- [x] Scan monolithiques : 569 fichiers (>400 lignes)
- Artefact : `.mip/memory/project-file-map.md`, `audits/monolithiques-scan.txt`

---

## Étape 2 — Plan découpage priorisé (Terminée)

- [x] Top 10 identifié et documenté

**Top 10 COG hors MGE** (ordre priorité) :

| # | Fichier | Lignes | Axe |
|---|---------|--------|-----|
| 1 | apps/origin/src/web/pages.rs | 3575 | index + pages-*.rs par section |
| 2 | apps/origin/src/web/content.rs | 2440 | index + content-*.rs |
| 3 | apps/central/src/services/ui_builder.rs | 2204 | sous-modules par responsabilité |
| 4 | crates/jayxpose/.../e07_vitrine_presentation.rs | 2251 | extraction composants |
| 5 | crates/lord_of_the_castle/loot.rs | 2155 | granulation |
| 6 | docs/.../Glossaire.md | 2158 | index + glossaire-*.md |
| 7 | apps/central/src/services/mws_view.rs | 1095 | découpage vues |
| 8 | crates/miyucloud/.../kindmother_db.rs | 1803 | modules CRUD |
| 9 | crates/miyukini-central/.../mws/mod.rs | 1682 | sous-modules |
| 10 | crates/jayxpose/.../kindmother_db.rs | 1636 | modules CRUD |

---

## Étape 3 — Indexation MSCM (Complète)

- [x] mip-generator présent dans tools/
- [x] mscm_index/ existant (1578 blocs, 696 fichiers)
- Vérifier mscm_index/ à jour
- Couverture cible : apps/central (18/156 → objectif 50 %+)
- Artefact : `mscm_index/blocks.json`, `hierarchy.json`

---

## Étape 4 — Conformité (Terminée)

- [x] Audit conformité produit : audits/2026-03-04-conformite.md
- Checklist George : build, tests, clippy (en cours / à valider)
- Règle I-14 : prioriser découpage top 10
- Anti-patterns : patterns-and-lessons.md
- Artefact : `<sequence>/audits/YYYY-MM-DD-conformite.md`

---

## Étape 5 — Réorganisation environnement (Terminée)

- [x] .mip/ : memory/, skills/, modules/ — structure validée
- docs/ : index par domaine
- Ressources : `<sequence>/ressources/index.md`

---

## Étape 6 — Livraison P5

- Commit final
- Rapport P6
- Mémoire mise à jour
