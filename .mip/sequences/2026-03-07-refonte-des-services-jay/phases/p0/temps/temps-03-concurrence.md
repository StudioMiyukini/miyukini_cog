# P0 Temps 3 - Analyse concurrentielle

## Statut

- Etat : Terminé
- Phase : P0 Temps 3
- Agent : Fabrice
- Date : 2026-03-07

## TL;DR

3 axes concurrentiels : festivals (Eventbrite, Weezevent), vitrine B2B (Shopify), portail web (Ghost, Webflow). Différenciation Miyukini : souveraineté données, 0% commission, intégration native inter-services Jay, stack Rust.

## Solutions existantes

### Axe JayFestival — Gestion festivals

| Solution | Points forts | Points faibles | Pertinence |
|----------|-------------|---------------|-----------|
| **Eventbrite** | UX reconnue, billetterie intégrée, audience massive | SaaS (données cloud), commission élevée, pas d'intégration COG | Référence UX à étudier |
| **Weezevent** | Fort en France, billetterie + exposants | SaaS, coût élevé, pas white-label | Concurrent direct |
| **Hello Asso** | Associatif, gratuit | Limité fonctionnellement | Référence partielle |

### Axe JayXpose — Vitrine exposant B2B

| Solution | Points forts | Points faibles | Pertinence |
|----------|-------------|---------------|-----------|
| **Shopify** | UX excellence, catalogue intuitif | SaaS, données cloud, coût mensuel | Référence UX catalogue |
| **WooCommerce** | Self-hosted, flexible | Complexe, maintenance lourde | Non pertinent (PHP) |

### Axe COG Web Portal — Portail multi-services

| Solution | Points forts | Points faibles | Pertinence |
|----------|-------------|---------------|-----------|
| **Ghost** | Clean, portal web, typographie forte | Pas d'intégration services Jay | Inspiration layout |
| **Webflow** | Design flexible | SaaS, pas d'intégration données live | Inspiration design |
| **apps/miyucloud** | Même stack Rust+axum, CSP nonce, sécurité production | UI basique | **Référence technique directe** |

## Positionnement

### Différenciation Miyukini

| Avantage | Explication |
|----------|-------------|
| **Souveraineté données** | Local-first dans le COG, pas de cloud tiers, RGPD maîtrisé |
| **0% commission** | Pas de marketplace intermédiaire |
| **Intégration native** | JayFestival ↔ JayXpose ↔ COG Web Portal : écosystème cohérent |
| **White-label par COG** | Chaque COG son portail et son identité |
| **Stack Rust** | Performance, sécurité, fiabilité native |

### Inspirations UX à retenir

| Source | Ce qu'on retient |
|--------|-----------------|
| Eventbrite | Flow réservation clair, cartes événements, filtres |
| Shopify dashboard | Dashboard exposant propre, catalogue intuitif, stats |
| Linear.app | UI dense et fonctionnelle, sidebar navigation efficace |
| Ghost portal | Layout portail web clean, typographie forte, pages statiques rapides |

