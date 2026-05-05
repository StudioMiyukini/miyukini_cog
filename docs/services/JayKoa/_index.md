# JayKoa â€” Index du service

## Contexte

**JayKoa** est le **calendrier universel du COG** â€” un Service transversal spÃ©cialisÃ© dans le domaine temporel. Il reflÃ¨te, agrÃ¨ge et orchestre le temps issu des autres Services de l'Ã©cosystÃ¨me Miyukini (JayRDV, futurs services).

> **Note 2026-04-29 :** L'intÃ©gration JayFestival a Ã©tÃ© retirÃ©e (service supprimÃ©). Voir [DEPRECATED](../DEPRECATED.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Documentation fondatrice, design system, rÃ©fÃ©rence sÃ©curitÃ©, rÃ©fÃ©rence intÃ©gration.
- **Audience** : Ã‰quipes produit, technique, UX/UI, sÃ©curitÃ©, parties prenantes.

## Documents

| Document | Description |
|----------|-------------|
| [JayKoa - Document Fondateur](./JayKoa%20-%20Document%20Fondateur.md) | Positionnement architectural, modÃ¨le conceptuel, gouvernance, contraintes et invariants. |
| [JayKoa - Ecrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) | Ã‰crans, zones et composants UI calquÃ©s sur Google Agenda (header, sidebar, grilles, formulaires, popovers). |
| [JayKoa - Parcours Utilisateurs](./JayKoa%20-%20Parcours%20Utilisateurs.md) | 20 parcours utilisateurs calquÃ©s sur Google Agenda (consultation, crÃ©ation, modification, synchronisation, export, partage). |
| [JayKoa - Design System Atomic](./JayKoa%20-%20Design%20System%20Atomic.md) | Inventaire complet des composants UI : 27 Atomes, 20 MolÃ©cules, 17 Organismes (Atomic Design). |
| [JayKoa - Bornage Implementation](./JayKoa%20-%20Bornage%20Implementation.md) | Bornage pour l'implÃ©mentation : pÃ©rimÃ¨tre MVP / phases, hors scope, dÃ©pendances, critÃ¨res de livraison. |
| [JayKoa - Operateurs et Toolkits](./JayKoa%20-%20Operateurs%20et%20Toolkits.md) | OpÃ©rateurs, Kits (EntrÃ©es, Conflits, Vue & Export, Ã‰vÃ©nements publics), Ã‰quipe, filtres supportÃ©s. |
| [JayKoa - Audit Documentation et Manques](./JayKoa%20-%20Audit%20Documentation%20et%20Manques.md) | Audit de la documentation et manques pour un service complet. |
| [JayKoa - Axes Amelioration et Plan Implementation](./JayKoa%20-%20Axes%20Amelioration%20et%20Plan%20Implementation.md) | Audit implementation complet : 10 axes, 34 taches, priorisation MVP/Post-MVP/Phase 2/Phase 3. |
| [RÃ©fÃ©rence â€” Niveaux SÃ©curitÃ© et Protection](./reference/JayKoa%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | DÃ©tail des niveaux WorrySentinel (0â€“4) et des mesures de protection pour les donnÃ©es agenda. |
| [RÃ©fÃ©rence â€” Integration Services Consommateurs](./reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) | SchÃ©mas d'intÃ©gration avec JayRDV, futurs services (intÃ©gration JayFestival historique retirÃ©e). |
| [RÃ©fÃ©rence â€” Referentiel Inspire Google Agenda](./reference/JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | RÃ©fÃ©rentiel fonctionnel inspirÃ© de Google Agenda (vues, rappels, partage, libre/occupÃ©, calendriers multiples). |

## Arborescence

```
docs/services/JayKoa/
â”œâ”€â”€ _index.md
â”œâ”€â”€ JayKoa - Document Fondateur.md
â”œâ”€â”€ JayKoa - Ecrans et UI.md
â”œâ”€â”€ JayKoa - Parcours Utilisateurs.md
â”œâ”€â”€ JayKoa - Design System Atomic.md
â”œâ”€â”€ JayKoa - Bornage Implementation.md
â”œâ”€â”€ JayKoa - Operateurs et Toolkits.md
â”œâ”€â”€ JayKoa - Audit Documentation et Manques.md
â”œâ”€â”€ JayKoa - Axes Amelioration et Plan Implementation.md
â””â”€â”€ reference/
    â”œâ”€â”€ _index.md
    â”œâ”€â”€ JayKoa - Niveaux Securite et Protection Donnees.md
    â”œâ”€â”€ JayKoa - Integration Services Consommateurs.md
    â”œâ”€â”€ JayKoa - Maquettes UI Type Google Agenda.md
    â””â”€â”€ JayKoa - Referentiel Fonctionnel Inspire Google Agenda.md
```

## Services synchronisÃ©s

| Service | Domaine temporel |
|--------|------------------|
| **JayRDV** | Rendez-vous confirmÃ©s, crÃ©neaux bloquÃ©s, modifications, annulations. |
| **Futurs services** | Tout Service du COG exposant des capacitÃ©s temporelles. |

## Voir aussi

- [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md)
- [Services retirÃ©s](../DEPRECATED.md)
- [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md)
- [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//miyukini-webway-system//reference//_index.md)

---

*DerniÃ¨re mise Ã  jour : 2026-02-06*

