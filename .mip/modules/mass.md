# Module MIP — MASS (Miyukini Agent Swarm System)

> Ce module est chargé pour les tâches T4-T5 nécessitant une parallélisation basée sur un DAG.
> **Pré-condition** : Lire `.mip/profiles/active`. Si profil sans MASS (ollama, lm-studio, aider, etc.), ne pas charger ce module.

---

## Architecture en 3 couches

**Couche 1 — Orchestrateur (Maria)** : Décompose le plan en un DAG de dépendances. Identifie les vagues parallèles. Applique la Loi 9. Choisit le mode de dispatch. DAG stocké dans `<sequence>/phases/dag.json`.

**Couche 2 — Pool de workers (agents)** : Bob (tâches simples), François, Lise, Victor (spot-check) exécutent les tâches en parallèle. Chaque agent ne touche JAMAIS un fichier assigné à un autre agent dans la même vague.

**Couche 3 — Synchronisation (Denis)** : Fusionne les résultats de chaque vague, vérifie la cohérence (build + test + lint), lance la vague suivante.

---

## Format JSON du DAG

Stocké dans `<sequence>/phases/dag.json`. Généré par Maria en P0 Temps 10 ou début de P3.

### Schéma

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | "1.0" |
| `slug` | string | Identifiant de l'initiative |
| `dispatch_mode` | enum | "subagent_burst" / "worktree_swarm" / "team_swarm" |
| `total_tasks` | number | Nombre total de tâches |
| `total_waves` | number | Nombre de vagues |
| `nodes[]` | array | Tâches : id, label, agent (bob|francois|lise|victor), type, wave, deps[], estimated_minutes, files[], status |
| `edges[]` | array | Arêtes : from, to |
| `waves[]` | array | Vagues : number, tasks[], parallelism, estimated_minutes, status |

**Types de nœuds** : `code`, `test_unit`, `test_integration`, `test_global`, `audit`, `correct`
**Statuts** : `pending`, `running`, `done`, `failed`, `skipped`

**Assignation Bob** : Tâche simple (1-2 fichiers, <30 min estimé, pas de spec complexe) → agent `bob` pour optimiser tokens (haiku).

### Validation

- Pas de cycles (graphe acyclique)
- Pas de dépendances à soi-même
- Chaque tâche dans exactement une vague
- Pas de dépendances intra-vague
- Dépendances dans des vagues antérieures

---

## Modes de dispatch

| Mode | Déclencheur | Mécanisme | Stratégie Git | Parallélisme max |
|------|-------------|-----------|---------------|-------------------|
| **Subagent burst** | T2-T3 ou vague <=3 tâches | Maria lance N subagents (outil Task) | Branche unique, commits séquentiels | ~3 |
| **Worktree swarm** | T4 ou vague >3 tâches, fichiers disjoints | Denis crée N git worktrees | 1 worktree/agent, merge dans branche principale | ~5 |
| **Team swarm** | T5, vagues complexes | Flag Agent Teams (expérimental) | Géré par Agent Teams | Limite plateforme |

### Règles de sélection (Maria)

1. Classe <= T3 ET vague <=3 tâches -> **subagent burst**
2. Classe T4 OU vague >3 tâches ET fichiers disjoints -> **worktree swarm**
3. Classe T5 ET Agent Teams actif -> **team swarm**
4. Fallback si Agent Teams indisponible -> **worktree swarm**
5. **Loi 9** : >3 tâches indépendantes -> parallélisation OBLIGATOIRE

### Isolation des fichiers (ABSOLUE)

Dans une vague parallèle, deux agents ne PEUVENT PAS toucher le même fichier. Si chevauchement, Denis DOIT réordonnancer les tâches dans des vagues différentes.

### Worktree Swarm — Détail

```bash
git worktree add ../wt-francois feat/slug
git worktree add ../wt-lise feat/slug
# Les agents travaillent, puis Denis fusionne
git worktree remove ../wt-francois
git worktree remove ../wt-lise
```

---

## Protocole de coordination des merges (Denis)

### Avant chaque vague
1. Lire les tâches et fichiers assignés
2. Vérifier l'absence de chevauchement de fichiers (si conflit -> réordonnancer)
3. Préparer le contexte minimal pour chaque agent (voir § Contexte minimal par worker)
4. Lancer les agents selon le mode de dispatch

### Contexte minimal par worker

**Chaque agent d'une vague MASS reçoit UNIQUEMENT** :
1. **Tâche isolée** : extrait du plan (1 tâche : id, label, files[], code attendu, test) — pas le plan complet
2. **Fichiers** : Read uniquement des fichiers assignés (<500 lignes ou extrait pertinent)
3. **Cert** : 0 ou 1 REFERENCE.md selon load-map pour le type de tâche
4. **Instructions** : Bob (`.mip/agents/bob.md`, ~40 lignes) pour tâches simples ; sinon agent light (`.mip/agents/light/{nom}.md`)

**INTERDIT** : plan complet, brief, spec, MEMORY.md, patterns-and-lessons complet, agent complet >80 lignes.

**Note** : Chaque worker est une invocation isolée (subagent/Task) — pas de partage de contexte entre workers. Loi 9 conservée ; l'optimisation passe par le contexte minimal, pas par réduire le parallélisme.

### Pendant la vague
- Chaque agent travaille UNIQUEMENT sur ses fichiers
- Chaque agent committe dans sa branche/worktree

### Après la vague (séquence merge)
1. Collecter les commits
2. Fusionner séquentiellement (`git merge --no-ff` si worktree, commit direct si subagent burst)
3. Si conflit : résoudre (priorité = dernière tâche dans l'ordre du DAG), logger `merge_conflicts++`
4. Checkpoint : build + lint + test workspace
5. Si échec -> corriger AVANT la vague suivante
6. Mettre à jour les métriques swarm (wave.status = "done")
7. Lancer la vague suivante

---

## Loi 9 — Anti-Serial-Collapse (NON NÉGOCIABLE)

> Si une vague contient >3 tâches indépendantes, ces tâches DOIVENT être exécutées en parallèle.

**Seuil** : >3 tâches indépendantes dans une seule vague
**Détection** : Maria, pendant la génération du DAG
**Conséquence** : Maria sélectionne le mode de dispatch approprié
**Exception** : Outil IA sans support d'agents parallèles -> Loi 9 suspendue avec avertissement
**Métriques** : Compteur `serial_collapses_prevented`

---

## Métriques Swarm

Section `swarm` dans `<sequence>/metrics/YYYY-MM-DD-<slug>.json`. Maria initialise, Denis alimente.

| Champ | Description |
|-------|-------------|
| `swarm.enabled` | true si MASS actif |
| `swarm.dispatch_mode` | Mode choisi |
| `swarm.dag_file` | Chemin du DAG JSON |
| `swarm.total_waves` | Nombre de vagues |
| `swarm.total_parallel_tasks` | Tâches parallèles (cumul) |
| `swarm.total_serial_tasks` | Tâches séquentielles (cumul) |
| `swarm.max_parallelism_achieved` | Parallélisme max dans une vague |
| `swarm.merge_conflicts` | Conflits totaux |
| `swarm.serial_collapses_prevented` | Loi 9 appliquée |
| `swarm.wave_details[]` | Par vague : tasks_planned, tasks_completed, parallelism_effective, duration_seconds, merge_conflicts, started_at, ended_at |

### Indicateurs dérivés (P6, Arianne)

- **Parallélisme effectif** = parallèle / (parallèle + série)
- **Ratio série/parallèle** = vagues mono-tâche / total
- **Débit** = total_tasks / durée P3
- **Taux de conflits de merge** = conflits / vagues

### Efficience Swarm (Jean)

Jean analyse l'efficience swarm en complément des métriques d'Arianne :
- Budget tokens multiplié par nombre d'agents parallèles = **faux positif connu** (ne pas alerter sur consommation totale)
- Analyser tokens/tâche **par agent individuel** pour détecter les valeurs aberrantes
- Vérifier que chaque agent swarm charge uniquement les fichiers nécessaires à sa tâche
- Recommander le modèle par agent swarm : Bob → haiku/sonnet, workers François/Lise → sonnet, lead Denis → opus

---

## Intégration MASS dans MIP

| Phase | Action MASS |
|-------|-------------|
| **P0 T7** | Denis inclut `deps[]` dans le plan |
| **P0 T10** | Maria génère le DAG, classe les vagues, sélectionne le dispatch |
| **Début P3** | Denis valide le DAG (pas de cycles, fichiers disjoints) |
| **Vagues P3** | Denis lance les agents, fusionne, checkpoints, met à jour les métriques |
| **Checkpoints P3** | Inchangés, cumulatifs sur toutes les vagues. Spot-check Victor |
| **P4** | George vérifie la cohérence post-merge (pas de régression inter-vagues) |
| **P5** | Denis inclut les stats swarm dans le résumé |
| **P6** | Arianne calcule les indicateurs dérivés, capitalise les patterns swarm |
