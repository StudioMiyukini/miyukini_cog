# Jay1Tribu â€” Index du service

## Contexte

**Jay1Tribu** est le **Service de messagerie pair-Ã -pair (P2P)** de l'Ã©cosystÃ¨me Miyukini COG. Il permet les discussions entre COGs, l'envoi de fichiers et d'images, avec archives maintenues **uniquement chez les participants** (base de donnÃ©es locale de chaque COG) et **transit cryptÃ©**. Philosophie : remplacer les systÃ¨mes de messagerie qui conservent les donnÃ©es Ã  l'insu des utilisateurs.

FonctionnalitÃ©s clÃ©s : salons de discussion, tribus (partage Ã  la reconnexion, rÃ´les, Chef de tribu), liste d'amis et prÃ©sence, envoi de messages, fichiers et images â€” le tout hÃ©bergÃ© chez les utilisateurs.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Documentation complÃ¨te du service â€” vision, architecture, spÃ©cification fonctionnelle, contraintes, sÃ©curitÃ©, intÃ©gration Central/Miou, guide d'implÃ©mentation, gouvernance.
- **Audience** : Ã‰quipes produit, architecture, sÃ©curitÃ©, dÃ©veloppeurs Central et Miou.

## Documents

| Document | Description |
|----------|-------------|
| [Jay1Tribu - Document Fondateur](./Jay1Tribu%20-%20Document%20Fondateur.md) | Vision, principes fondateurs, type de Service (Type 3), capacitÃ©s clÃ©s, dÃ©pendances MWS et Cores. |
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Vision dÃ©taillÃ©e, concepts (tribus, salons, amis, rÃ´les), persistance, sÃ©curitÃ©, contraintes et invariants conceptuels. |
| [Jay1Tribu - Architecture et Positionnement](./Jay1Tribu%20-%20Architecture%20et%20Positionnement.md) | Positionnement dans la Pyramide Miyukini, OpÃ©rateurs, dÃ©pendance MWS, Cores, flux d'exÃ©cution. |
| [Jay1Tribu - Specification Fonctionnelle](./Jay1Tribu%20-%20Specification%20Fonctionnelle.md) | Cas d'usage, parcours utilisateur, rÃ¨gles mÃ©tier (salons, tribus, amis, partage Ã  la reconnexion). |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | Contraintes non nÃ©gociables (C-1 Ã  C-8), invariants architecturaux, de donnÃ©es et d'intÃ©gration, conformitÃ© Lois d'Autonomie. |
| [Jay1Tribu - Securite et Conformite](./Jay1Tribu%20-%20Securite%20et%20Conformite.md) | Classification des donnÃ©es, chiffrement (transit, au repos), contrÃ´les d'accÃ¨s, modÃ¨le de menaces, conformitÃ©. |
| [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) | Contrat d'intÃ©gration avec Miyukini Central et Miou (get_online_friends, get_friends_list), dÃ©gradation gracieuse. |
| [Jay1Tribu - Guide Implementation](./Jay1Tribu%20-%20Guide%20Implementation.md) | Structure du crate, modules (data, transport, domain), phases de dÃ©veloppement, matrice de vÃ©rification. |
| [Jay1Tribu - Interface Utilisateur et Ecrans](./Jay1Tribu%20-%20Interface%20Utilisateur%20et%20Ecrans.md) | UI type Steam (liste d'amis + chat) et Discord (tribus, salons, CRUD tribu/membres/rÃ´les/salons). |
| [Jay1Tribu - Service Governance Compliance](./contracts/governance/Jay1Tribu%20-%20Service%20Governance%20Compliance.md) | ConformitÃ© gouvernance (Cores, contraintes). |

## Arborescence

```
docs/services/Jay1Tribu/
â”œâ”€â”€ _index.md
â”œâ”€â”€ Jay1Tribu - Document Fondateur.md
â”œâ”€â”€ Jay1Tribu - Document Conceptuel.md
â”œâ”€â”€ Jay1Tribu - Architecture et Positionnement.md
â”œâ”€â”€ Jay1Tribu - Specification Fonctionnelle.md
â”œâ”€â”€ Jay1Tribu - Contraintes et Invariants.md
â”œâ”€â”€ Jay1Tribu - Securite et Conformite.md
â”œâ”€â”€ Jay1Tribu - Integration Central et Miou.md
â”œâ”€â”€ Jay1Tribu - Guide Implementation.md
â”œâ”€â”€ Jay1Tribu - Interface Utilisateur et Ecrans.md
â””â”€â”€ contracts/
    â””â”€â”€ governance/
        â””â”€â”€ Jay1Tribu - Service Governance Compliance.md
```

## Concepts clÃ©s

| Concept | Description |
|---------|-------------|
| **Salon** | Espace de discussion (direct ou collectif) ; messages cryptÃ©s en transit ; archives locales chez chaque participant. |
| **Tribu** | Groupe partageant discussions, fichiers et images ; synchronisation Ã  la reconnexion (si l'Ã©metteur est connectÃ©) ; rÃ´les attribuÃ©s par le Chef de tribu. |
| **Amis** | Liste d'amis pour voir la prÃ©sence (MWS) et initier une discussion directe rapidement. |
| **Transit cryptÃ©** | Toutes les donnÃ©es Ã©changÃ©es entre COGs sont cryptÃ©es. |
| **HÃ©bergement utilisateur** | Discussions, fichiers et images restent dans les COGs des participants ; pas d'archives centrales. |

## Type de Service

**Service Inter-COG (Type 3)** â€” Espace Miyukini Central (gestion, UI) + Protocoles Inter-COG (MWS, cryptage, transport).

## Voir aussi

- [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md)
- [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md)
- [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Central â€” Miou](../MiyukiniCentral/Miou/_index.md)
- [MiyukiniWatch â€” IntÃ©gration Miou](../MiyukiniWatch/MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md)

---

*DerniÃ¨re mise Ã  jour : 2026-02-15*

