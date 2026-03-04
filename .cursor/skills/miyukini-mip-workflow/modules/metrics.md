# Module MIP — Metriques & Horodatage

> Ce module est charge a l'initialisation des metriques (debut de sequence MIP).

---

## Initialisation (Maria, debut de sequence)

Maria cree `.mip/metrics/YYYY-MM-DD-<slug>.json` :

```json
{
  "project": {
    "title": "<titre du brief>",
    "description": "<description courte>",
    "class": "T3|T4|T5",
    "slug": "<slug>",
    "mip_sequence_number": 1,
    "autonomy_mode": "FULL|BIG_STEPS|GUIDED"
  },
  "timestamps": {
    "p0_start": "ISO8601",
    "p0_end": null,
    "autopilot_start": null,
    "p3_start": null, "p3_end": null,
    "p4_start": null, "p4_end": null,
    "p5_start": null, "p5_end": null,
    "p5_test_start": null, "p5_test_end": null,
    "p6_start": null, "p6_end": null,
    "total_end": null
  },
  "counters": {
    "lines_written": 0,
    "lines_deleted": 0,
    "crates_touched": [],
    "crates_created": [],
    "files_created": 0,
    "files_modified": 0,
    "commits": 0,
    "agents_engaged": [],
    "mip_loops": 1,
    "unit_tests_total": 0,
    "unit_tests_failed": 0,
    "integration_tests_total": 0,
    "integration_tests_failed": 0,
    "global_tests_total": 0,
    "global_tests_failed": 0,
    "auto_corrections": 0,
    "audits": 0,
    "audit_defects": [],
    "security_score": null,
    "security_defects": [],
    "infra_checks_passed": null,
    "emergency_brakes": 0
  },
  "human_interventions": [],
  "agent_questions": [],
  "satisfaction": null,
  "notes": null
}
```

---

## Collecte par phase

| Phase | Qui collecte | Quoi |
|-------|-------------|------|
| **P0** | Maria | `p0_start`, `p0_end`, `agents_engaged`, questions posees |
| **Git** | Denis | `autopilot_start` |
| **P3** | Francois/Lise | `p3_start/end`, `lines_written/deleted`, `commits`, `unit_tests_*`, `auto_corrections`, `crates_touched` |
| **P4** | Denis/George/Victor/Hugo | `p4_start/end`, `audits`, `audit_defects[]`, `security_score`, `global_tests_*`, `integration_tests_*` |
| **P5** | Denis | `p5_start/end`, `p5_test_start/end`, `satisfaction`, `human_interventions[]` |
| **P6** | Arianne | `p6_start/end`, `total_end`, compilation rapport |

---

## Interventions humaines

Chaque intervention loggee :

```json
{
  "timestamp": "ISO8601",
  "type": "precision|arret|pause|changement_direction|constat_erreur|delta|autre",
  "phase": "P0|P3|P4|P5",
  "description": "<description>",
  "impact": "aucun|mineur|majeur|critique"
}
```

---

## Questions agents → humain

Chaque question loggee :

```json
{
  "timestamp": "ISO8601",
  "agent": "Maria|Denis|Francois|Lise|George|Arianne|Fabrice|Victor|Hugo",
  "phase": "P0|P3|P4|P5",
  "nature": "clarification|validation|choix_technique|choix_design|blocage|autre",
  "question": "<texte>",
  "response_summary": "<resume>"
}
```

---

## Collecte des tokens et timestamps (DONNEES REELLES — ZERO estimation)

**Regle absolue** : Le rapport P6 ne contient que des valeurs **mesurees**. Les mots `~`, `environ`, `estimation`, `approximation` sont **INTERDITS** dans les sections metriques du rapport final.

### Methode 1 — Tokens depuis les task-notifications (source primaire)

Chaque subagent (Agent tool) genere une task-notification a sa completion. Cette notification contient :
- `total_tokens` : tokens totaux (input + output)
- `tool_uses` : nombre d'appels outils
- `duration` : duree reelle de l'invocation

L'orchestrateur **DOIT** collecter ces valeurs pour chaque invocation d'agent et les stocker dans le fichier metriques `.mip/metrics/`.

### Methode 2 — Timestamps depuis le filesystem (source primaire)

Les fichiers crees/modifies portent un timestamp filesystem. Pour obtenir les heures reelles :
```bash
stat --format='%n | %y' <fichier>           # Linux/Git Bash
(Get-Item <fichier>).LastWriteTime           # PowerShell
```

L'orchestrateur collecte les timestamps des fichiers cles a chaque changement de phase (creation topic files, fin audit, etc.).

### Methode 3 — Horodatage inline dans le plan (source secondaire)

Chaque agent annote sa tache dans le plan a la completion :
```
> Demarre a HH:MM:SS. Termine a HH:MM:SS avec [model] pour N tokens (mesures).
```

**IMPORTANT** : Le format utilise des secondes (HH:MM:SS) et N tokens **sans tilde**. Si les tokens ne sont pas disponibles au moment de l'annotation, l'agent ecrit `tokens: a completer` et l'orchestrateur remplit apres reception de la task-notification.

### Collecte obligatoire par l'orchestrateur

A chaque fin de phase, l'orchestrateur :
1. Lit les task-notifications de tous les agents lances dans cette phase
2. Extrait `total_tokens`, `tool_uses`, `duration` de chaque notification
3. Collecte les timestamps filesystem des fichiers produits
4. Met a jour `.mip/metrics/YYYY-MM-DD-<slug>.json` avec les valeurs reelles
5. **Ne fabrique JAMAIS de valeur** — si une donnee manque, inscrire `null` avec commentaire

---

## Indicateurs d'efficacite (calcules en P6 par Arianne — valeurs mesurees)

| Indicateur | Formule | Source |
|------------|---------|--------|
| Tokens par ligne produite | tokens_total / lignes_ecrites | task-notifications |
| Lignes par heure effective | lignes_ecrites / duree_effective_h | filesystem timestamps |
| Taches par heure effective | taches_completees / duree_effective_h | filesystem timestamps |
| Gain parallelisme | duree_sequentielle / duree_wall | task-notifications (somme vs max) |
| ROI (conversations pour amortir) | tokens_total / tokens_economises_par_conv | calcul |

> **Verification croisee** : Arianne verifie que duree_wall (timestamps filesystem) est coherente avec la somme des durees agents (task-notifications). Ecart > 20% = anomalie a documenter.
