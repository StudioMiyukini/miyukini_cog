# Brief : Audit complet Miyukini COG — Inventaire, modularisation, MSCM, conformité

## TL;DR (5 lignes max)

Audit T5 de l'écosystème Miyukini COG **hors MGE**. Inventaire au standard MIP, identification des fichiers monolithiques (>400 lignes) avec axes de refactorisation/modularisation/granulation, indexation MSCM, remise en conformité et réorganisation de l'environnement. Livrables : inventaire, cartographie monolithiques, plan de découpage, index MSCM à jour, environnement réorganisé.

---

## Métadonnées

- **Classe** : T5
- **Date** : 2026-03-04
- **Séquence** : `.mip/sequences/2026-03-04-central-improve-secure-update/`
- **Début séquence** : 2026-03-04T21:30:00Z
- **Profil** : cursor-composer-1.5
- **Périmètre** : Miyukini COG hors MGE (crates/, apps/, docs/, .mip/, tools/ — exclut mge/)

---

## Contexte

- **Objectif** : Audit complet, inventaire standard MIP, identification monolithiques, indexation MSCM, conformité, réorganisation environnement
- **Exclusions** : Workspace MGE (`mge/`, crates mge-*)
- **Références** : Règle I-14 (400 lignes max), MSCM Index (mscm_index/), project-file-map, conventions MIP
- **État actuel** : apps/central 18/156 MSCM (11,5 %), miyuki-ui 100 %, miyucloud 100 %, certifications 100 %

---

## Objectifs

| Priorité | Objectif | Critère de succès |
|----------|----------|-------------------|
| Principal | Inventaire au standard MIP | project-file-map à jour, zones principales, fichiers critiques |
| Principal | Identification fichiers monolithiques | Liste exhaustive >400 lignes avec axes refactorisation/modularisation/granulation |
| Principal | Indexation MSCM | mscm_index/ à jour, couverture cible, blocs cohérents |
| Secondaire | Remise en conformité | Règle I-14, anti-patterns, Lois d'Autonomie |
| Secondaire | Réorganisation environnement | Structure .mip/, docs/, tools/ alignée conventions |

---

## Périmètre

### Inclus

- **Inventaire** : crates/ (hors mge), apps/ (central, origin, miou-llm-bridge, etc.), docs/, .mip/, tools/
- **Monolithiques** : tout fichier .rs, .md, .ts >400 lignes (règle I-14)
- **Axes** : refactorisation (découpage), modularisation (index + annexes), granulation (sous-modules)
- **MSCM** : annotations @id, @do, @role, @layer, @human ; génération index
- **Conformité** : conventions MIP, patterns-and-lessons, security-patterns
- **Réorganisation** : .mip/ (memory, skills, modules, config), docs/, ressources

### Exclus

- Workspace MGE (mge/, sodomight, mge-*)
- Demos MGE (demos/mge-pathfinding-labyrinthe)

---

## Approches proposées

### Approche A — Audit séquentiel par zone (recommandée)

**Description** : Phase 1 inventaire (Denis, project-file-map). Phase 2 scan monolithiques (script + manuel). Phase 3 plan découpage par fichier. Phase 4 indexation MSCM (mip-generator + corrections). Phase 5 conformité (George checklist). Phase 6 réorganisation (Hugo, structure).

**Pour** : Aligné workflow MIP, livrables incrémentaux.  
**Contre** : Séquentiel, pas de parallélisme massif.  
**Effort** : ~25–30 j.

### Approche B — Audit par type (monolithiques d'abord)

**Description** : Prioriser l'identification et le découpage des monolithiques, puis inventaire et MSCM.

**Pour** : Gain rapide sur la dette I-14.  
**Contre** : Inventaire incomplet au démarrage.  
**Effort** : ~22–28 j.

### Approche C — Inventaire + MSCM prioritaire

**Description** : Compléter project-file-map et mscm_index en premier, puis monolithiques et conformité.

**Pour** : Base de données solide pour la suite.  
**Contre** : Monolithiques restent en l'état plus longtemps.  
**Effort** : ~28–32 j.

**Recommandation** : Approche A (séquentiel structuré).

---

## Direction visuelle (Lise)

Audit documentaire et structurel — pas d'UI utilisateur. Livrables : tableaux, index, rapports. Convention : index + annexes si >400 lignes, nomenclature cohérente.

---

## Analyse concurrentielle (Fabrice)

Benchmarks méthodologie audit : SonarQube, CodeClimate, Semgrep. Différenciateur COG : standard MIP (I-14, MSCM), inventaire par zone, intégration mip-generator. Cible : maintenabilité, gouvernance sémantique.

---

## Analyse de sécurité (Victor)

| Surface | Scénario | Impact | Mitigation |
|---------|----------|--------|------------|
| Découpage monolithiques | Régression, fuite contexte | Moyen | Tests avant/après, review |
| Index MSCM | Intégrité blocs | Faible | Vérification relay Phase B |
| Réorganisation | Fichiers sensibles déplacés | Faible | Inventaire, .gitignore |

Niveau : standard. Checklist : pas de secrets dans inventaire, audit dépendances optionnel.

---

## Pipeline CI/CD (Hugo)

T9 sauté — pas de CI/CD en place. Réorganisation n'impacte pas de pipeline existant. Recommandation : documenter structure cible pour future CI.

---

## Inventaire des prérequis (Denis)

**Compétences** : scan fichiers, parsing MSCM, Rust/Cargo. **Outils** : mip-generator, rg, scripts PowerShell. **Étapes** : inventaire → scan → plan → MSCM → conformité → réorg. **Matrice** : project-file-map à créer, mscm_index existant à valider.

---

## Spécification technique (François)

Voir annexe `<sequence>/specs/` (T6 produit en P0). Méthodologie scan : `Get-ChildItem -Recurse | Where-Object { $_.Extension -match '\.(rs|md|ts)$' } | ForEach-Object { (Get-Content $_.FullName | Measure-Object -Line).Lines }` ou équivalent. Seuil 400. Format livrable : tableau chemin, lignes, zone, axes.

---

## Audit de faisabilité (Arianne)

**Verdict** : CONFORME. Agents disponibles, prérequis identifiés, dépendances OK. Manques mineurs : project-file-map à compléter en Phase 1. Mémoire : patterns-and-lessons chargé, anti-patterns AP-08/AP-09 connus.

---

## Identification monolithiques — Méthodologie

1. **Scan** : `rg` ou script sur .rs, .md, .ts — compter lignes par fichier
2. **Seuil** : >400 lignes = monolithique (I-14)
3. **Classification** : type (code, doc, spec), zone, complexité
4. **Axes** :
   - **Refactorisation** : extraire fonctions/structs, réduire couplage
   - **Modularisation** : index + annexes (ex. plan.md + plan-etape-X.md)
   - **Granulation** : sous-modules, découpage par responsabilité

---

## Plan de développement (Denis)

- **Phase 1** : Inventaire (project-file-map, zones, fichiers critiques) — ~4 j
- **Phase 2** : Scan monolithiques, liste, axes par fichier — ~3 j
- **Phase 3** : Plan découpage priorisé — ~2 j
- **Phase 4** : Indexation MSCM (générateur + corrections) — ~8 j
- **Phase 5** : Conformité (checklist George, anti-patterns) — ~4 j
- **Phase 6** : Réorganisation environnement — ~3 j
- **Buffer** : 20 % — ~5 j
- **Total estimé** : ~29 j

---

## Risques

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Nombre élevé de monolithiques | Élevée | Moyen | Priorisation, plan par vague |
| Régression découpage | Moyenne | Moyen | Tests, validation build |
| Index MSCM obsolète | Moyenne | Faible | Automatisation mip-generator |

---

## Décision

**APPROUVÉ** — 2026-03-04

---

## Mode d'autonomie

**FULL** — Autopilot complet. Prochaine interaction : test P5 (validation livrable).
