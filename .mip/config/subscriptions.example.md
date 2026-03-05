# Abonnements — Exemple à copier vers subscriptions.md

> Copier ce fichier vers `subscriptions.md` et renseigner vos quotas. Référence : `.mip/config/subscriptions.md`

```yaml
# Abonnements tokens par fournisseur
# Utilisé pour : estimation P0, comparaison P6, alerte si >80% quota

anthropic:
  plan: pro
  tokens_period: 5000000
  period: monthly
  period_start: "2026-03-01"
  active: true

openai:
  plan: plus
  tokens_period: 2000000
  period: monthly
  period_start: "2026-03-01"
  active: true

google:
  plan: free
  tokens_period: 1500000
  period: monthly
  active: false

moonshot:
  plan: pro
  tokens_period: 30000000
  period: monthly
  active: false

# Autres : mistral, deepseek, groq, z — ajouter selon besoin
# Local (Ollama, LM Studio) : tokens_period = null (illimité)
```
