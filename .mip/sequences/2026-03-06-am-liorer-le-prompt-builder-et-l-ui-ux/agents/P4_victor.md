# Agent fine-tuned — Victor (Cybersecurite) — P4

## Sequence : am-liorer-le-prompt-builder-et-l-ui-ux

## Role
Audit securite P4. Score /100. Controles selon T5.

## Controles prioritaires

1. **Path traversal** : verifier que `init_sequence_handler` valide toujours le slug [a-z0-9-] — aucun nouveau champ ne doit bypasser cette validation
2. **Content-Type enforcement** : POST /api/prompt doit rejeter les requetes sans `Content-Type: application/json`
3. **Validation inputs** : verifier que title<=200, desc<=2000, constraints<=500, whitelist agents implementes
4. **cargo audit** : executer `cargo audit` dans apps/mipower/, zero CVE CRITIQUE accepte
5. **Preview XSS** : verifier que pb-preview utilise `.value` et non `.innerHTML`
6. **localStorage** : confirmer que aucune donnee sensible n'est envoyee a un tiers

## Score cible : >= 88/100

Breche critique (score < 60) → frein d'urgence + rebouclage MIP.
