# Abonnements et quotas tokens — Configuration utilisateur

<!-- @id mip.config.subscriptions
     @do provide_token_budget_for_estimation
     @role config
     @layer config
     @human Quotas tokens par fournisseur — estimation consommation -->

> L'utilisateur renseigne ses abonnements pour permettre une **estimation consommation vs tokens disponibles**. Optionnel mais recommandé pour maîtriser les coûts.

---

## Modèle par fournisseur

Pour chaque fournisseur utilisé, renseigner :

| Champ | Type | Description |
|-------|------|-------------|
| `provider` | string | anthropic, openai, google, mistral, moonshot, deepseek, groq, autres |
| `plan` | string | free, pro, team, enterprise, payg (pay-as-you-go), custom |
| `tokens_period` | number | Tokens inclus par période (ex. 5000000) |
| `period` | string | monthly, daily, annual |
| `period_start` | string | Date début période (YYYY-MM-DD) pour calcul restant |
| `input_output_separate` | boolean | true si quota input/output séparés (ex. OpenAI) |
| `tokens_input_period` | number | (optionnel) Si séparé |
| `tokens_output_period` | number | (optionnel) Si séparé |

**Unité** : tokens (équivalent comptage standard, ~4 caractères/token en moyenne). Les APIs renvoient souvent input_tokens et output_tokens.

---

## Référence fournisseurs (ordres de grandeur)

| Fournisseur | Plan | Ordre de grandeur | Période |
|-------------|------|-------------------|---------|
| **Anthropic** | Pro | 5M tokens | Mensuel |
| **Anthropic** | Team | 50M+ | Mensuel |
| **OpenAI** | Plus | ~500$ credit | Mensuel (conversion variable) |
| **OpenAI** | Team | Selon contrat | Mensuel |
| **Google (Gemini)** | Free | 1.5M tokens | Mensuel |
| **Google (Gemini)** | Pro | 1M/jour | Quotidien |
| **Mistral** | Divers | Pay-as-you-go ou forfait | — |
| **Moonshot (Kimi)** | Pro | ~30M tokens | Mensuel |
| **DeepSeek** | Pro | Forfait | Mensuel |
| **Groq** | Free | 30 req/min | Rate limit |
| **Z** (Z AI, Alibaba, etc.) | Variable | À renseigner | — |
| **Local (Ollama, LM Studio)** | — | Illimité (ressources machine) | — |

> Les quotas évoluent. Vérifier sur le site du fournisseur. Ce tableau est indicatif.

---

## Format de configuration

L'utilisateur crée `.mip/config/subscriptions.md` à partir du fichier exemple `.mip/config/subscriptions.example.md`, ou renseigne pendant SETUP-4 (S4.5-S4.8).

Le profil actif (`.mip/profiles/active`) définit le `provider` (ex. anthropic). MIP associe automatiquement le quota configuré pour ce fournisseur.

---

## Utilisation par MIP

1. **Jean (efficience)** : Compare consommation mesurée (`<sequence>/metrics/`) vs quotas configurés. Alerte si >80 % du quota.
2. **P0 Temps 8 (Arianne+Jean)** : Estime le budget tokens de la séquence selon la classe (T3 ~50k, T4 ~200k, T5 ~500k+) et le profil actif. Vérifie que le quota restant couvre l'estimation.
3. **Rapport P6** : Affiche `tokens_consumed` vs `quota_period` (si renseigné). Ex. « 127k tokens consommés / 5M quota Anthropic = 2,5 % ».

---

## SETUP-4 — Questions ajoutées

Lors de la détection outil IA, Maria pose (optionnel) :

- S4.5 : Fournisseur(s) IA utilisés ? (Anthropic, OpenAI, Google, Mistral, Moonshot, DeepSeek, autre)
- S4.6 : Plan/abonnement pour chaque fournisseur ? (free, pro, team, etc.)
- S4.7 : Quota tokens par période si connu ? (ex. 5M/mois Anthropic Pro)
- S4.8 : Souhaitez-vous suivre la consommation vs quota ? (oui/non)

Si oui, le fichier `subscriptions.md` est créé ou complété.
