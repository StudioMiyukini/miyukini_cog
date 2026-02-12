# JayShop - Commerce, vente en ligne et point de vente

## Contexte

JayShop est le service Miyukini dedie au **commerce et a la vente** :
- onboarding vendeur
- vente en ligne (boutique web)
- point de vente (PoS) en caisse
- gestion des tickets et historique des ventes
- **gestion des evenements/festivals** (fiches, couts, stock temporaire, benefices)
- integration catalogue et stocks (JayXpose)
- suivi comptable (JayKonta)
- **integration JayFestival** (synchronisation automatique des participations)

## Documentation principale

| Document | Role |
|----------|------|
| [JayShop - Document Fondateur](./JayShop%20-%20Document%20Fondateur.md) | Vision, scope, objectifs, principes directeurs. |
| [JayShop - Analyse des besoins](./JayShop%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels et non-fonctionnels. |
| [JayShop - Parcours Utilisateur](./JayShop%20-%20Parcours%20Utilisateur.md) | Parcours Admin et Client. |
| [JayShop - Ecrans et UI](./JayShop%20-%20Ecrans%20et%20UI.md) | Cartographie ecrans, composants, flux PoS et paiement. |

## Architecture et implementation

| Document | Role |
|----------|------|
| [JayShop - Guide Implementation](./JayShop%20-%20Guide%20Implementation.md) | Architecture crate, schema libSQL (KindMother), Operateurs et Kits, plan par phases, integration Central. |
| [JayShop - Interfaces Inter-Services](./JayShop%20-%20Interfaces%20Inter-Services.md) | Contrats JayXpose, JayKonta, JayFestival, Central. |

## Reference

| Document | Role |
|----------|------|
| [reference/_index.md](./reference/_index.md) | Index reference technique. |
| [JayShop - Reference Loyverse Back Office](./reference/JayShop%20-%20Reference%20Loyverse%20Back%20Office.md) | Analyse concurrentielle et screenshots annotes du back office Loyverse POS. |

## Liaisons ecosysteme

- [JayXpose - Document Fondateur](../JayXpose/JayXpose%20-%20Document%20Fondateur.md) — Catalogue produits, stocks
- [JayKonta - Document Fondateur](../JayKonta/JayKonta%20-%20Document%20Fondateur.md) — Suivi comptable
- [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) — **Gestion evenements, synchronisation participations**
- [Miyukini Sales - Document Fondateur](../MiyukiniSales/Miyukini%20Sales%20-%20Document%20Fondateur.md) — Socle Operateurs partages
- [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md)
