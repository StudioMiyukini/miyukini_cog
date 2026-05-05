# Miyukini Web Portal â€” Point d'entrÃ©e Web du COG

## Contexte

**Miyukini Web Portal** (le **Portail**) est le **Service Fondamental** qui expose les surfaces web des Services du COG aux utilisateurs externes. Il constitue l'Ã©quivalent de Miyukini Central pour le web : lÃ  oÃ¹ Central est le point d'entrÃ©e pour l'utilisateur du COG, le Portail est le point d'entrÃ©e pour les utilisateurs externes accÃ©dant via un navigateur.

**RÃ¨gle canonique :**

> **Central = COG, Portail = Web.**

Le Portail fait partie intÃ©grante de l'environnement versionnÃ© du COG.

## Documentation principale

| Document | RÃ´le |
|----------|------|
| [Miyukini Web Portal - Document Fondateur](./Miyukini%20Web%20Portal%20-%20Document%20Fondateur.md) | Vision, scope, positionnement, architecture, relation avec Central. |
| [Miyukini Web Portal - Surface Web Implementation et Gouvernance](./Miyukini%20Web%20Portal%20-%20Surface%20Web%20Implementation%20et%20Gouvernance.md) | Comment implÃ©menter, guider, borner et normer une surface web. |

## En bref

| Aspect | Valeur |
|--------|--------|
| **Type** | Service Fondamental (OpÃ©rateur d'Interface, Strate 7) |
| **RÃ´le** | Point d'entrÃ©e web pour les utilisateurs externes |
| **Cible** | Utilisateurs externes (clients, visiteurs, prospects) sans COG |
| **Relation** | Ã‰quivalent de Central, mais pour le web |
| **Gouvernance** | BorderGuard + Visa + Mandat Public d'AccÃ¨s |

## Services exposÃ©s via le Portail (Type 2)

Les Services de **Type 2** (Ã  surface web externe) exposent leurs faÃ§ades publiques via le Portail :

| Service | Surface exposÃ©e |
|---------|-----------------|
| **JayRDV** | Page de rÃ©servation, parcours guest, annulation/modification |
| **JayKonta** | Portail client (consultation factures, paiement) |
| **JayShop** | Boutique en ligne et catalogue produit |
| **JayManga** | Liseuse web, vitrine vendeur et Portail AgrÃ©gÃ© |

## Architecture simplifiÃ©e

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Utilisateur externe (navigateur web)                        â”‚
â”‚  https://mon-commerce.miyukini.cog / https://kine-rdv.cog   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚ HTTPS
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              Miyukini Web Portal (Portail)                   â”‚
â”‚  Â· Routage vers les surfaces des Services (Type 2)          â”‚
â”‚  Â· Identification et fichage des connexions entrantes        â”‚
â”‚  Â· Mandat Public d'AccÃ¨s                                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚ BondingBrother
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  BorderGuard Â· StrongFather Â· KindMother Â· WorrySentinel     â”‚
â”‚  (Gouvernance, sÃ©curitÃ©, persistance)                        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Services (JayRDV, JayKonta, JayShop, JayManga, ...)         â”‚
â”‚  (Exposent leurs capacitÃ©s au Portail)                       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

## Ce que le Portail N'EST PAS

- **Pas un serveur central unique** qui affiche tous les COGs â€” chaque COG expose **son** Portail
- **Pas un remplacement de Central** â€” Central reste le point d'entrÃ©e pour l'utilisateur COG
- **Pas une porte ouverte** â€” tout accÃ¨s passe par BorderGuard, identification et Mandat Public

## Voir aussi

- [Miyukini Central - Hub de gestion des Services](..//..//miyukini-webway-system//reference//_index.md)
- [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md)
- [Glossaire â€” FaÃ§ade Publique GouvernÃ©e](..//..//miyukini-webway-system//reference//_index.md)
- [Glossaire â€” Utilisateur Externe](..//..//miyukini-webway-system//reference//_index.md)

