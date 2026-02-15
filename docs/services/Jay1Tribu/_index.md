# Jay1Tribu — Index du service

## Contexte

**Jay1Tribu** est le **Service de messagerie pair-à-pair (P2P)** de l'écosystème Miyukini COG. Il permet les discussions entre COGs, l'envoi de fichiers et d'images, avec archives maintenues **uniquement chez les participants** (base de données locale de chaque COG) et **transit crypté**. Philosophie : remplacer les systèmes de messagerie qui conservent les données à l'insu des utilisateurs.

Fonctionnalités clés : salons de discussion, tribus (partage à la reconnexion, rôles, Chef de tribu), liste d'amis et présence, envoi de messages, fichiers et images — le tout hébergé chez les utilisateurs.

## Portée / Scope

- **Périmètre** : Documentation complète du service — vision, architecture, spécification fonctionnelle, contraintes, sécurité, intégration Central/Miou, guide d'implémentation, gouvernance.
- **Audience** : Équipes produit, architecture, sécurité, développeurs Central et Miou.

## Documents

| Document | Description |
|----------|-------------|
| [Jay1Tribu - Document Fondateur](./Jay1Tribu%20-%20Document%20Fondateur.md) | Vision, principes fondateurs, type de Service (Type 3), capacités clés, dépendances MWS et Cores. |
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Vision détaillée, concepts (tribus, salons, amis, rôles), persistance, sécurité, contraintes et invariants conceptuels. |
| [Jay1Tribu - Architecture et Positionnement](./Jay1Tribu%20-%20Architecture%20et%20Positionnement.md) | Positionnement dans la Pyramide Miyukini, Opérateurs, dépendance MWS, Cores, flux d'exécution. |
| [Jay1Tribu - Specification Fonctionnelle](./Jay1Tribu%20-%20Specification%20Fonctionnelle.md) | Cas d'usage, parcours utilisateur, règles métier (salons, tribus, amis, partage à la reconnexion). |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | Contraintes non négociables (C-1 à C-8), invariants architecturaux, de données et d'intégration, conformité Lois d'Autonomie. |
| [Jay1Tribu - Securite et Conformite](./Jay1Tribu%20-%20Securite%20et%20Conformite.md) | Classification des données, chiffrement (transit, au repos), contrôles d'accès, modèle de menaces, conformité. |
| [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) | Contrat d'intégration avec Miyukini Central et Miou (get_online_friends, get_friends_list), dégradation gracieuse. |
| [Jay1Tribu - Guide Implementation](./Jay1Tribu%20-%20Guide%20Implementation.md) | Structure du crate, modules (data, transport, domain), phases de développement, matrice de vérification. |
| [Jay1Tribu - Interface Utilisateur et Ecrans](./Jay1Tribu%20-%20Interface%20Utilisateur%20et%20Ecrans.md) | UI type Steam (liste d'amis + chat) et Discord (tribus, salons, CRUD tribu/membres/rôles/salons). |
| [Jay1Tribu - Service Governance Compliance](./contracts/governance/Jay1Tribu%20-%20Service%20Governance%20Compliance.md) | Conformité gouvernance (Cores, contraintes). |

## Arborescence

```
docs/services/Jay1Tribu/
├── _index.md
├── Jay1Tribu - Document Fondateur.md
├── Jay1Tribu - Document Conceptuel.md
├── Jay1Tribu - Architecture et Positionnement.md
├── Jay1Tribu - Specification Fonctionnelle.md
├── Jay1Tribu - Contraintes et Invariants.md
├── Jay1Tribu - Securite et Conformite.md
├── Jay1Tribu - Integration Central et Miou.md
├── Jay1Tribu - Guide Implementation.md
├── Jay1Tribu - Interface Utilisateur et Ecrans.md
└── contracts/
    └── governance/
        └── Jay1Tribu - Service Governance Compliance.md
```

## Concepts clés

| Concept | Description |
|---------|-------------|
| **Salon** | Espace de discussion (direct ou collectif) ; messages cryptés en transit ; archives locales chez chaque participant. |
| **Tribu** | Groupe partageant discussions, fichiers et images ; synchronisation à la reconnexion (si l'émetteur est connecté) ; rôles attribués par le Chef de tribu. |
| **Amis** | Liste d'amis pour voir la présence (MWS) et initier une discussion directe rapidement. |
| **Transit crypté** | Toutes les données échangées entre COGs sont cryptées. |
| **Hébergement utilisateur** | Discussions, fichiers et images restent dans les COGs des participants ; pas d'archives centrales. |

## Type de Service

**Service Inter-COG (Type 3)** — Espace Miyukini Central (gestion, UI) + Protocoles Inter-COG (MWS, cryptage, transport).

## Voir aussi

- [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md)
- [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md)
- [Miyukini Central — Miou](../MiyukiniCentral/Miou/_index.md)
- [MiyukiniWatch — Intégration Miou](../MiyukiniWatch/MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md)

---

*Dernière mise à jour : 2026-02-15*
