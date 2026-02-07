# JayKoa — Index du service

## Contexte

**JayKoa** est le **calendrier universel du COG** — un Service transversal spécialisé dans le domaine temporel. Il reflète, agrège et orchestre le temps issu des autres Services de l'écosystème Miyukini (JayRDV, JayFestival, futurs services).

## Portée / Scope

- **Périmètre** : Documentation fondatrice, design system, référence sécurité, référence intégration.
- **Audience** : Équipes produit, technique, UX/UI, sécurité, parties prenantes.

## Documents

| Document | Description |
|----------|-------------|
| [JayKoa - Document Fondateur](./JayKoa%20-%20Document%20Fondateur.md) | Positionnement architectural, modèle conceptuel, gouvernance, contraintes et invariants. |
| [JayKoa - Ecrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) | Écrans, zones et composants UI calqués sur Google Agenda (header, sidebar, grilles, formulaires, popovers). |
| [JayKoa - Parcours Utilisateurs](./JayKoa%20-%20Parcours%20Utilisateurs.md) | 20 parcours utilisateurs calqués sur Google Agenda (consultation, création, modification, synchronisation, export, partage). |
| [JayKoa - Design System Atomic](./JayKoa%20-%20Design%20System%20Atomic.md) | Inventaire complet des composants UI : 27 Atomes, 20 Molécules, 17 Organismes (Atomic Design). |
| [JayKoa - Bornage Implementation](./JayKoa%20-%20Bornage%20Implementation.md) | Bornage pour l'implémentation : périmètre MVP / phases, hors scope, dépendances, critères de livraison. |
| [JayKoa - Operateurs et Toolkits](./JayKoa%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs, Kits (Entrées, Conflits, Vue & Export, Événements publics), Équipe, filtres supportés. |
| [JayKoa - Audit Documentation et Manques](./JayKoa%20-%20Audit%20Documentation%20et%20Manques.md) | Audit de la documentation et manques pour un service complet. |
| [JayKoa - Axes Amelioration et Plan Implementation](./JayKoa%20-%20Axes%20Amelioration%20et%20Plan%20Implementation.md) | Audit implementation complet : 10 axes, 34 taches, priorisation MVP/Post-MVP/Phase 2/Phase 3. |
| [Référence — Niveaux Sécurité et Protection](./reference/JayKoa%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Détail des niveaux WorrySentinel (0–4) et des mesures de protection pour les données agenda. |
| [Référence — Integration Services Consommateurs](./reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) | Schémas d'intégration avec JayRDV, JayFestival, futurs services. |
| [Référence — Referentiel Inspire Google Agenda](./reference/JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | Référentiel fonctionnel inspiré de Google Agenda (vues, rappels, partage, libre/occupé, calendriers multiples). |

## Arborescence

```
docs/services/JayKoa/
├── _index.md
├── JayKoa - Document Fondateur.md
├── JayKoa - Ecrans et UI.md
├── JayKoa - Parcours Utilisateurs.md
├── JayKoa - Design System Atomic.md
├── JayKoa - Bornage Implementation.md
├── JayKoa - Operateurs et Toolkits.md
├── JayKoa - Audit Documentation et Manques.md
├── JayKoa - Axes Amelioration et Plan Implementation.md
└── reference/
    ├── _index.md
    ├── JayKoa - Niveaux Securite et Protection Donnees.md
    ├── JayKoa - Integration Services Consommateurs.md
    ├── JayKoa - Maquettes UI Type Google Agenda.md
    └── JayKoa - Referentiel Fonctionnel Inspire Google Agenda.md
```

## Services synchronisés

| Service | Domaine temporel |
|--------|------------------|
| **JayRDV** | Rendez-vous confirmés, créneaux bloqués, modifications, annulations. |
| **JayFestival** | Dates de festivals, inscriptions, deadlines, événements favoris, participations. |
| **Futurs services** | Tout Service du COG exposant des capacités temporelles. |

## Voir aussi

- [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md)
- [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md)
- [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)

---

*Dernière mise à jour : 2026-02-06*
