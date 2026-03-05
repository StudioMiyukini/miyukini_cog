# Module MIP — Métriques et horodatages

> Ce module est chargé à l'initialisation des métriques (début de séquence MIP).

## Budget tokens par phase (optionnel)

Si `.mip/config/subscriptions.md` renseigné, budget indicatif par phase : P0 ~50k, P3 ~100k, MASS vague ~80k/worker. Alerte Jean si dépassement >150 % de l'estimation.

---

## Initialisation (Maria, début de séquence)

Maria crée `<sequence>/metrics/YYYY-MM-DD-<slug>.json` :

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
    "packages_touched": [],
    "packages_created": [],
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
| **P0** | Maria | `p0_start`, `p0_end`, `agents_engaged`, questions posées |
| **Git** | Denis | `autopilot_start` |
| **P3** | François/Lise | `p3_start/end`, `lines_written/deleted`, `commits`, `unit_tests_*`, `auto_corrections`, `packages_touched` |
| **P4** | Denis/George/Victor/Hugo | `p4_start/end`, `audits`, `audit_defects[]`, `security_score`, `global_tests_*`, `integration_tests_*` |
| **P5** | Denis | `p5_start/end`, `p5_test_start/end`, `satisfaction`, `human_interventions[]` |
| **P6** | Arianne | `p6_start/end`, `total_end`, compilation du rapport |

---

## Interventions humaines

Chaque intervention enregistrée :

```json
{
  "timestamp": "ISO8601",
  "type": "clarification|stop|pause|direction_change|error_finding|delta|other",
  "phase": "P0|P3|P4|P5",
  "description": "<description>",
  "impact": "none|minor|major|critical"
}
```

---

## Questions agent -> humain

Chaque question enregistrée :

```json
{
  "timestamp": "ISO8601",
  "agent": "Maria|Denis|Francois|Lise|George|Arianne|Fabrice|Victor|Hugo|Jean",
  "phase": "P0|P3|P4|P5",
  "nature": "clarification|validation|technical_choice|design_choice|blocker|other",
  "question": "<texte>",
  "response_summary": "<résumé>"
}
```

---

## Collecte tokens et horodatages (DONNÉES RÉELLES — ZÉRO estimation)

**Règle absolue** : Le rapport P6 ne contient que des valeurs **mesurées**. Les mots `~`, `environ`, `estimation`, `approximatif` sont **INTERDITS** dans les sections métriques du rapport final.

### Méthode 1 — Tokens des notifications de tâches (source principale)

Chaque subagent (outil Agent) génère une notification de tâche à la complétion. Cette notification contient :
- `total_tokens` : tokens totaux (entrée + sortie)
- `tool_uses` : nombre d'appels d'outils
- `duration` : durée réelle de l'invocation

L'orchestrateur **DOIT** collecter ces valeurs pour chaque invocation d'agent et les stocker dans le fichier métriques `<sequence>/metrics/`.

### Méthode 2 — Horodatages système de fichiers (source principale)

Les fichiers créés/modifiés portent un horodatage système de fichiers. Pour obtenir les temps réels :
```bash
stat --format='%n | %y' <fichier>           # Linux/Git Bash
(Get-Item <fichier>).LastWriteTime           # PowerShell
```

L'orchestrateur collecte les horodatages des fichiers clés à chaque changement de phase (création du fichier sujet, complétion audit, etc.).

### Méthode 3 — Horodatages inline dans le plan (source secondaire)

Chaque agent annote sa tâche dans le plan à la complétion :
```
> Démarré à HH:MM:SS. Terminé à HH:MM:SS avec [model] pour N tokens (mesurés).
```

**IMPORTANT** : Le format utilise les secondes (HH:MM:SS) et N tokens **sans tilde**. Si les tokens ne sont pas disponibles au moment de l'annotation, l'agent écrit `tokens: à compléter` et l'orchestrateur remplit après réception de la notification de tâche.

### Collecte obligatoire par l'orchestrateur

À chaque fin de phase, l'orchestrateur :
1. Lit les notifications de tâches de tous les agents lancés dans cette phase
2. Extrait `total_tokens`, `tool_uses`, `duration` de chaque notification
3. Collecte les horodatages système de fichiers des fichiers produits
4. Met à jour `<sequence>/metrics/YYYY-MM-DD-<slug>.json` avec les valeurs réelles
5. **Ne fabrique jamais une valeur** — si donnée manquante, écrire `null` avec commentaire

---

## Quotas et estimation vs consommation

Si `.mip/config/subscriptions.md` est renseigné, Jean et Arianne comparent la consommation mesurée au quota du fournisseur du profil actif. Rapport P6 : `tokens_consumed / tokens_quota_period` (ex. « 127k / 5M = 2,5 % »). Alerte si >80 % du quota.

---

## Indicateurs d'efficience (calculés en P6 par Arianne — valeurs mesurées)

| Indicateur | Formule | Source |
|------------|---------|--------|
| Tokens par ligne produite | tokens_total / lines_written | notifications de tâches |
| Lignes par heure effective | lines_written / effective_duration_h | horodatages système de fichiers |
| Tâches par heure effective | tasks_completed / effective_duration_h | horodatages système de fichiers |
| Gain de parallélisme | sequential_duration / wall_duration | notifications de tâches (somme vs max) |
| ROI (conversations à amortir) | tokens_total / tokens_saved_per_conv | calcul |

> **Vérification croisée** : Arianne vérifie que wall_duration (horodatages système de fichiers) est cohérente avec la somme des durées agents (notifications de tâches). Écart > 20 % = anomalie à documenter.
