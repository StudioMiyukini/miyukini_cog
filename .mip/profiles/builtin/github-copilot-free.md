---
id: github-copilot-free
name: GitHub Copilot (Claude 3.5 Sonnet) - Plan Gratuit
tool: vscode-copilot
llm:
  provider: anthropic
  model: claude-3-5-sonnet
  context_window: 8000
  fallback: null
capabilities:
  parallel_agents: false
  todo_write: false
  mcp: false
  terminal: false
  background_tasks: false
  doc_verification: false
  multi_file_edit: false
  ask_user_question: true
adaptations:
  - parallel_agents: "MASS suspendu. Tâches lancées résonnes. Loi 9 appliquée : découpage en P3 séquentiels"
  - terminal: "NO NATIVE ACCESS → utilisateur exécute commande, copie résultat en chat"
  - mcp: "Non disponible → recherche web manuelle ou web.search() via instructions"
  - todo_write: "Annonces texte à chaque étape (pas de TodoList outil)"
  - doc_verification: "Pattern memory ou skip"
  - multi_file_edit: "Max 2-3 fichiers par itération → découpage en tâches"
  - background_tasks: "Toutes en pseudo-parallelisme texte (annonces de progression)"
constraints:
  - "Context window 8k → résumés agressifs des fichiers lus"
  - "Rate limit : ~5 req/min vs Claude Code (illimité) → patience requise"
  - "Pas d'accès terminal → Manuel (CRUCIAL pour SETUP, build, tests)"
  - "Pas de TodoWrite → Annonces texte seules, utilisateur pilote la progression"
industrial_scope:
  - "✅ Code review & debugging (< 3 fichiers)"
  - "✅ Petit design (T1-T2)"
  - "✅ Pair programming console"
  - "❌ Refactor massif (T4-T5)"
  - "❌ Multi-service orchestration"
---

# GitHub Copilot (Plan Gratuit)

> **Cas réel** : Salarié Total avec accès Copilot uniquement (VS Code standard).  
> Limitations sévères. MIP ≈ 40 % capacités. Découpage en micro-tâches CRUCIAL.

## Profil

- **Outil** : VS Code, extension Copilot (gratuite)
- **Modèle** : Claude 3.5 Sonnet (Anthropic)
- **Context** : ~8k tokens (très limité)
- **Rate limit** : ~5 req/min (lent)

## Limitations industrielles critiques

| Limitation | Impact | Adaptation |
|-----------|--------|-----------|
| **0 Terminal** | Impossible build, test, git automatique | → Plan manuel : utilisateur exécute cmd, copie résultat |
| **8k context** | Impossible lire 70+ crates | → Lire 1-2 fichiers max, use MIP Index JSON |
| **No TodoWrite** | Pas d'orchestration interne | → Annonces texte, utilisateur track progression |
| **Rate limit 5/min** | ~3h pour T3 (vs 15min Claude Code) | → Batch requests, préfetch context |
| **No parallel agents** | Impossible MASS | → Séquentiel strict P3 → P4 → P5 → P6 |

## Adaptations MIP appliquées

### A : Terminal → Manuel

```
MIP: "Exécutons `cargo build -p miyukini-central`"
Présenté à utilisateur :
  ▶ MANUEL : Exécutez en PowerShell :
    cargo build -p miyukini-central
  → Collez le résultat ci-dessous (entrez quand prêt)
  [Utilisateur tape commande, copie output]
```

### B : Context 8k → Résumés + MIP Index

```yaml
Instead of: "Lisons Cargo.toml entièrement"
Use: "Je consulte mscm_index/blocks.json pour trouver les crates"
Then: "Lisons seulement ces 2 sections de Cargo.toml"
Result: context ~2k/8k utilisé
```

### C : TodoWrite → Annonces texte  

```
Instead of: (/manage_todo_list trigger)
Use: Textual announcements
  "📌 T2-fix-parser | P3 ✅ | P4 en cours..."
  "Prêt pour P5 (Human Review) ?"
```

### D : Multi-file → 1-2 per iteration

T2 max 2 fichiers/iteration. T3+ → découper en 3-4 sous-tâches.

## Workflow Copilot Plan Gratuit (T2)

```
🔵 P0 Framing
   - Utilisateur décrit le problème (< 100 tokens)
   - Copilot détermine : C'est quoi cette tâche ? (T1 ? T2 ?)

🟢 P3 Implementation (séquentiel)
   Iter 1) Lire fichier_A (résumé 500 tokens)
         → Code change file_A
         → Utilisateur valide ("OK" ou "nope")
   Iter 2) Lire fichier_B
         → Code change file_B
         → Utilisateur valide

🟣 P4 Integration (Manuel à 90 %)
   - "Exécutez : cargo clippy -p {crate}"
   - Utilisateur lance, copie output
   - Copilot lit errors, suggère fix

🟡 P5 Delivery
   - "Prêt à merger ?"
   - Utilisateur commit + push (Manuel)

⚫ P6 Archive
   - Rapport texte court (~500 tokens)
   - Mémoire mise à jour
```

## Quand utiliser ce profil ?

✅ **BON** :
- Petits bugs (1-2 fichiers)
- Code review pair
- Architecture sketch (texte/dia)
- Apprendre pattern (T1-T2)

❌ **MAUVAIS** :
- Refactor 10+ fichiers
- CI/CD setup automatisé
- Multi-service orchestration
- Live debugging (besoin terminal)

## Token budget (T2 example)

```
P0 Framing      : ~500 tokens
P3 x2-3 iter    : ~3000 tokens (600/iter)
P4 Integration  : ~2000 tokens (manual)
P5 Delivery     : ~500 tokens
P6 Archive      : ~300 tokens
─────────────────────────────────
Total ~6500 tokens (sûr pour 8k limit)
```

## Recommandations

1. **Découper T3-T5 en 3-4 T2 parallèles** (travail sur une seule branche)
2. **Préparer contexte PRÉ-SESSION** : MIP Index JSON, fichier cible listé
3. **Utilisateur = Copilot du Copilot** : Vous lancez les cmds, donnez feedback
4. **Pas de MASS** : Agents serait impossible
5. **Quotas** : ~1 T2/jour sans fatigue cognitive (3h)

## Lien documentation

- [MIP Workflow](../protocol/conventions.md#workflow-phases) — P0-P6
- [MSCM/MIP Navigation](../skills/miyukini-mscm-mip/SKILL.md) — Lire Index JSON
- [Capacities Matrix](./capabilities-matrix.md) — Comparaison outils
