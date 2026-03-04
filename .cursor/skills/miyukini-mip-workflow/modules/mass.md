# Module MIP — MASS (Miyukini Agent Swarm System)

> Ce module est charge pour les taches T4-T5 necessitant la parallelisation par DAG.

<!-- @id: mas -->
<!-- @do: Pattern de parallelisation des taches MIP via DAG de dependances et vagues d'execution -->
<!-- @role: Maria (orchestrateur), Denis (merge coordinator), Workers (Francois, Lise, Victor) -->
<!-- @layer: Protocole MIP v2 -->

---

## Architecture en 3 couches

<!-- @id: mass.architecture -->

**Couche 1 — Orchestrateur (Maria)** : Decompose le plan en DAG de dependances. Identifie les vagues paralleles. Applique la Loi 9. Choisit le mode de dispatch. DAG stocke dans `.mip/dags/YYYY-MM-DD-<slug>.json`.

**Couche 2 — Pool Workers (agents)** : Francois, Lise, Victor (spot-check) executent les taches en parallele. Chaque agent ne touche JAMAIS un fichier assigne a un autre agent dans la meme vague.

**Couche 3 — Synchronisation (Denis)** : Merge les resultats de chaque vague, verifie coherence (build + test + clippy), lance la vague suivante.

---

## Format DAG JSON

<!-- @id: mass.dag-format -->

Stocke dans `.mip/dags/YYYY-MM-DD-<slug>.json`. Genere par Maria en P0 Temps 10 ou debut P3.

### Schema

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | "1.0" |
| `slug` | string | Identifiant du chantier |
| `dispatch_mode` | enum | "subagent_burst" / "worktree_swarm" / "team_swarm" |
| `total_tasks` | number | Nombre total de taches |
| `total_waves` | number | Nombre de vagues |
| `nodes[]` | array | Taches : id, label, agent, type, wave, deps[], estimated_minutes, files[], status |
| `edges[]` | array | Aretes : from, to |
| `waves[]` | array | Vagues : number, tasks[], parallelism, estimated_minutes, status |

**Types de noeuds** : `code`, `test_unit`, `test_integration`, `test_global`, `audit`, `correct`
**Statuts** : `pending`, `running`, `done`, `failed`, `skipped`

### Validation

- Aucun cycle (graphe acyclique)
- Aucune dependance a soi-meme
- Chaque tache dans exactement une vague
- Pas de dependances intra-vague
- Dependances dans des vagues anterieures

---

## Modes de dispatch

<!-- @id: mass.dispatch-modes -->

| Mode | Declencheur | Mecanisme | Git strategy | Max // |
|------|-------------|-----------|-------------|--------|
| **Subagent burst** | T2-T3 ou vague <=3 taches | Maria lance N subagents (Task tool) | Branche unique, commits sequentiels | ~3 |
| **Worktree swarm** | T4 ou vague >3 taches, fichiers disjoints | Denis cree N git worktrees | 1 worktree/agent, merge dans branche principale | ~5 |
| **Team swarm** | T5, vagues complexes | Flag Agent Teams (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`) | Gere par Agent Teams | Limite plateforme |

### Regles de selection (Maria)

1. Classe <= T3 ET vague <=3 taches → **subagent burst**
2. Classe T4 OU vague >3 taches ET fichiers disjoints → **worktree swarm**
3. Classe T5 ET Agent Teams active → **team swarm**
4. Fallback si Agent Teams non dispo → **worktree swarm**
5. **Loi 9** : >3 taches independantes → parallelisation OBLIGATOIRE

### Isolation des fichiers (ABSOLUE)

Dans une vague parallele, deux agents ne PEUVENT PAS toucher le meme fichier. Si chevauchement, Denis DOIT reordonnancer les taches dans des vagues differentes.

### Worktree swarm — detail

```bash
git worktree add ../wt-francois feat/slug
git worktree add ../wt-lise feat/slug
# Agents travaillent, puis Denis merge
git worktree remove ../wt-francois
git worktree remove ../wt-lise
```

---

## Protocole de merge coordination (Denis)

<!-- @id: mass.merge-coordination -->

### Avant chaque vague
1. Lire taches et fichiers assignes
2. Verifier pas de chevauchement fichier (si conflit → reordonnancer)
3. Preparer contexte pour chaque agent
4. Lancer agents selon mode dispatch

### Pendant la vague
- Chaque agent travaille sur ses fichiers UNIQUEMENT
- Chaque agent commit dans sa branche/worktree

### Apres la vague (merge sequence)
1. Collecter les commits
2. Merge sequentiellement (`git merge --no-ff` si worktree, commit direct si subagent burst)
3. Si conflit : resoudre (priorite = derniere tache dans l'ordre DAG), logger `merge_conflicts++`
4. Checkpoint : build + lint + test workspace
5. Si echec → corriger AVANT vague suivante
6. MAJ metriques swarm (wave.status = "done")
7. Lancer vague suivante

---

## Loi 9 — Anti-Serial-Collapse (NON NEGOCIABLE)

<!-- @id: mass.law-9 -->

> Si une vague contient >3 taches independantes, ces taches DOIVENT etre executees en parallele.

**Seuil** : >3 taches independantes dans une meme vague
**Detection** : Maria, lors de la generation du DAG
**Consequence** : Maria selectionne le mode dispatch adapte
**Exception** : Outil IA sans support agents paralleles → Loi 9 suspendue avec warning
**Metriques** : Compteur `serial_collapses_prevented`

---

## Metriques swarm

<!-- @id: mass.metrics -->

Section `swarm` dans `.mip/metrics/YYYY-MM-DD-<slug>.json`. Maria initialise, Denis alimente.

| Champ | Description |
|-------|-------------|
| `swarm.enabled` | true si MASS actif |
| `swarm.dispatch_mode` | Mode choisi |
| `swarm.dag_file` | Chemin DAG JSON |
| `swarm.total_waves` | Nombre de vagues |
| `swarm.total_parallel_tasks` | Taches // (cumul) |
| `swarm.total_serial_tasks` | Taches series (cumul) |
| `swarm.max_parallelism_achieved` | Max // dans une vague |
| `swarm.merge_conflicts` | Total conflits |
| `swarm.serial_collapses_prevented` | Loi 9 appliquee |
| `swarm.wave_details[]` | Par vague : tasks_planned, tasks_completed, parallelism_effective, duration_seconds, merge_conflicts, started_at, ended_at |

### Indicateurs derives (P6, Arianne)

- **Parallelisme effectif** = parallel / (parallel + serial)
- **Ratio serial/parallel** = vagues a 1 tache / total
- **Throughput** = total_tasks / duree P3
- **Merge conflict rate** = conflicts / waves

### Efficience swarm (Jean)

Jean analyse l'efficience du swarm en complement des metriques Arianne :
- Budget tokens multiplie par le nombre d'agents paralleles = **faux positif connu** (ne pas alerter sur la consommation totale)
- Analyser tokens/tache **par agent individuellement** pour detecter les outliers
- Verifier que chaque agent du swarm charge uniquement les fichiers necessaires a sa tache
- Recommander le modele par agent dans le swarm (workers en sonnet/haiku, lead en opus)

---

## Integration MASS dans MIP

<!-- @id: mass.mip-integration -->

| Phase | Action MASS |
|-------|-------------|
| **P0 T7** | Denis inclut `deps[]` dans le plan |
| **P0 T10** | Maria genere le DAG, classifie vagues, selectionne dispatch |
| **P3 debut** | Denis valide DAG (pas de cycle, fichiers disjoints) |
| **P3 vagues** | Denis lance agents, merge, checkpoint, MAJ metriques |
| **P3 checkpoints** | Inchanges, cumul toutes vagues. Victor spot-check |
| **P4** | George verifie coherence post-merge (pas de regression inter-vagues) |
| **P5** | Denis inclut stats swarm dans le resume |
| **P6** | Arianne calcule indicateurs derives, capitalise patterns swarm |
