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
  - parallel_agents: "MASS suspendu. TÃ¢ches lancÃ©es rÃ©sonnes. Loi 9 appliquÃ©e : dÃ©coupage en P3 sÃ©quentiels"
  - terminal: "NO NATIVE ACCESS â†’ utilisateur exÃ©cute commande, copie rÃ©sultat en chat"
  - mcp: "Non disponible â†’ recherche web manuelle ou web.search() via instructions"
  - todo_write: "Annonces texte Ã  chaque Ã©tape (pas de TodoList outil)"
  - doc_verification: "Pattern memory ou skip"
  - multi_file_edit: "Max 2-3 fichiers par itÃ©ration â†’ dÃ©coupage en tÃ¢ches"
  - background_tasks: "Toutes en pseudo-parallelisme texte (annonces de progression)"
constraints:
  - "Context window 8k â†’ rÃ©sumÃ©s agressifs des fichiers lus"
  - "Rate limit : ~5 req/min vs Claude Code (illimitÃ©) â†’ patience requise"
  - "Pas d'accÃ¨s terminal â†’ Manuel (CRUCIAL pour SETUP, build, tests)"
  - "Pas de TodoWrite â†’ Annonces texte seules, utilisateur pilote la progression"
industrial_scope:
  - "âœ… Code review & debugging (< 3 fichiers)"
  - "âœ… Petit design (T1-T2)"
  - "âœ… Pair programming console"
  - "âŒ Refactor massif (T4-T5)"
  - "âŒ Multi-service orchestration"
---

# GitHub Copilot (Plan Gratuit)

> **Cas rÃ©el** : SalariÃ© Total avec accÃ¨s Copilot uniquement (VS Code standard).  
> Limitations sÃ©vÃ¨res. MIP â‰ˆ 40 % capacitÃ©s. DÃ©coupage en micro-tÃ¢ches CRUCIAL.

## Profil

- **Outil** : VS Code, extension Copilot (gratuite)
- **ModÃ¨le** : Claude 3.5 Sonnet (Anthropic)
- **Context** : ~8k tokens (trÃ¨s limitÃ©)
- **Rate limit** : ~5 req/min (lent)

## Limitations industrielles critiques

| Limitation | Impact | Adaptation |
|-----------|--------|-----------|
| **0 Terminal** | Impossible build, test, git automatique | â†’ Plan manuel : utilisateur exÃ©cute cmd, copie rÃ©sultat |
| **8k context** | Impossible lire 70+ crates | â†’ Lire 1-2 fichiers max, use MIP Index JSON |
| **No TodoWrite** | Pas d'orchestration interne | â†’ Annonces texte, utilisateur track progression |
| **Rate limit 5/min** | ~3h pour T3 (vs 15min Claude Code) | â†’ Batch requests, prÃ©fetch context |
| **No parallel agents** | Impossible MASS | â†’ SÃ©quentiel strict P3 â†’ P4 â†’ P5 â†’ P6 |

## Adaptations MIP appliquÃ©es

### A : Terminal â†’ Manuel

```
MIP: "ExÃ©cutons `cargo build -p miyukini-central`"
PrÃ©sentÃ© Ã  utilisateur :
  â–¶ MANUEL : ExÃ©cutez en PowerShell :
    cargo build -p miyukini-central
  â†’ Collez le rÃ©sultat ci-dessous (entrez quand prÃªt)
  [Utilisateur tape commande, copie output]
```

### B : Context 8k â†’ RÃ©sumÃ©s + MIP Index

```yaml
Instead of: "Lisons Cargo.toml entiÃ¨rement"
Use: "Je consulte mscm_index/blocks.json pour trouver les crates"
Then: "Lisons seulement ces 2 sections de Cargo.toml"
Result: context ~2k/8k utilisÃ©
```

### C : TodoWrite â†’ Annonces texte  

```
Instead of: (/manage_todo_list trigger)
Use: Textual announcements
  "ðŸ“Œ T2-fix-parser | P3 âœ… | P4 en cours..."
  "PrÃªt pour P5 (Human Review) ?"
```

### D : Multi-file â†’ 1-2 per iteration

T2 max 2 fichiers/iteration. T3+ â†’ dÃ©couper en 3-4 sous-tÃ¢ches.

## Workflow Copilot Plan Gratuit (T2)

```
ðŸ”µ P0 Framing
   - Utilisateur dÃ©crit le problÃ¨me (< 100 tokens)
   - Copilot dÃ©termine : C'est quoi cette tÃ¢che ? (T1 ? T2 ?)

ðŸŸ¢ P3 Implementation (sÃ©quentiel)
   Iter 1) Lire fichier_A (rÃ©sumÃ© 500 tokens)
         â†’ Code change file_A
         â†’ Utilisateur valide ("OK" ou "nope")
   Iter 2) Lire fichier_B
         â†’ Code change file_B
         â†’ Utilisateur valide

ðŸŸ£ P4 Integration (Manuel Ã  90 %)
   - "ExÃ©cutez : cargo clippy -p {crate}"
   - Utilisateur lance, copie output
   - Copilot lit errors, suggÃ¨re fix

ðŸŸ¡ P5 Delivery
   - "PrÃªt Ã  merger ?"
   - Utilisateur commit + push (Manuel)

âš« P6 Archive
   - Rapport texte court (~500 tokens)
   - MÃ©moire mise Ã  jour
```

## Quand utiliser ce profil ?

âœ… **BON** :
- Petits bugs (1-2 fichiers)
- Code review pair
- Architecture sketch (texte/dia)
- Apprendre pattern (T1-T2)

âŒ **MAUVAIS** :
- Refactor 10+ fichiers
- CI/CD setup automatisÃ©
- Multi-service orchestration
- Live debugging (besoin terminal)

## Token budget (T2 example)

```
P0 Framing      : ~500 tokens
P3 x2-3 iter    : ~3000 tokens (600/iter)
P4 Integration  : ~2000 tokens (manual)
P5 Delivery     : ~500 tokens
P6 Archive      : ~300 tokens
â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
Total ~6500 tokens (sÃ»r pour 8k limit)
```

## Recommandations

1. **DÃ©couper T3-T5 en 3-4 T2 parallÃ¨les** (travail sur une seule branche)
2. **PrÃ©parer contexte PRÃ‰-SESSION** : MIP Index JSON, fichier cible listÃ©
3. **Utilisateur = Copilot du Copilot** : Vous lancez les cmds, donnez feedback
4. **Pas de MASS** : Agents serait impossible
5. **Quotas** : ~1 T2/jour sans fatigue cognitive (3h)

## Lien documentation

- [MIP Workflow](..//..//README.md#workflow-phases) â€” P0-P6
- [MSCM/MIP Navigation](..//..//README.md) â€” Lire Index JSON
- [Capacities Matrix](..//..//README.md) â€” Comparaison outils

