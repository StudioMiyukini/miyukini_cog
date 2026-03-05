<!-- @id mem.tuning.agents
     @do tune_agent_configurations
     @role config
     @layer memory
     @human Reglages prompts/modeles/outils par agent -->

# Agent tuning

> A completer en P4/P6.

## Reglages

| Agent | Modele recommande | Prompt notes | Outils |
|-------|-------------------|--------------|--------|
| `A completer` | `A completer` | `A completer` | `A completer` |
| denis | GPT-5-codex | Prioriser gate scope sequence + documenter hors scope | shell, apply_patch, cargo |
| victor | GPT-5-codex | Sortie PASS->RAS avec score et defauts actionnables | analyse code + checklists |
| george | GPT-5-codex | Conformite d abord, puis recommandations | checklists + preuves commandes |
