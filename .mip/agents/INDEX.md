# MIP Agents Index (Phase-Bounded)

## Structure
Chaque agent a son dossier:
.mip/agents/<agent>/

Avec:
- FULL_<agent>.md : reference complete
- <PHASE>_<agent>.md : version bornee par phase (SETUP, P0, P3, P4, P5, P6, MASS selon agent)
- TEMPLATE_PHASE_AGENT.md : template normatif pour creer/mettre a jour les versions phase

## Regle de chargement (obligatoire)
1. Charger d abord la version de phase.
2. Charger FULL_<agent>.md seulement en cas d escalation justifiee.
3. Ne jamais charger plusieurs FULL_*.md sans justification explicite.
4. Les fichiers phase doivent etre construits depuis `TEMPLATE_PHASE_AGENT.md` et rester courts/injectables.

## Agents
- maria
- fabrice
- denis
- lise
- francois
- victor
- george
- hugo
- jean
- arianne
- bob
