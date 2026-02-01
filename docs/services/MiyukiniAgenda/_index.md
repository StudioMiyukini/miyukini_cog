# Miyukini Agenda — Index du service

## Contexte

**Miyukini Agenda** est le **service Miyukini unifié du domaine agenda** au sein de l’écosystème COG. Il fournit une couche commune de modélisation, de conflits, de fuseaux et d’export pour tout ce qui relève du calendrier et des plages temporelles. Les services métier (JayRDV, Miyukini Festival Service, et futurs services) **s’appuient sur Miyukini Agenda** pour éviter la duplication et permettre l’agenda multi-sources.

## Portée / Scope

- **Périmètre** : Documentation fondatrice, référence sécurité, référence intégration.
- **Audience** : Équipes produit, technique, sécurité, parties prenantes.

## Documents

| Document | Description |
|----------|-------------|
| [Miyukini Agenda - Document Fondateur](./Miyukini%20Agenda%20-%20Document%20Fondateur.md) | Contexte, besoins stratégiques, positionnement, intégration multi-services, niveaux de sécurité et solutions de protection. |
| [Miyukini Agenda - Ecrans et UI](./Miyukini%20Agenda%20-%20Ecrans%20et%20UI.md) | Besoins en écrans et UI : composants (vue calendrier, alerte conflit, export, filtres) intégrés dans les UIs des services consommateurs. |
| [Miyukini Agenda - Parcours Utilisateurs](./Miyukini%20Agenda%20-%20Parcours%20Utilisateurs.md) | Parcours utilisateurs (consultation agenda, vérification conflit, export, agrégation) vécus via JayRDV et MFS ; parcours côté service. |
| [Miyukini Agenda - Bornage Implementation](./Miyukini%20Agenda%20-%20Bornage%20Implementation.md) | Bornage pour l’implémentation : périmètre MVP / phases, hors scope, dépendances, critères de livraison. |
| [Miyukini Agenda - Operateurs et Toolkits](./Miyukini%20Agenda%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs, Kits (Entrées, Conflits, Vue & Export, Événements publics), Équipe, filtres supportés. |
| [Miyukini Agenda - Audit Documentation et Manques](./Miyukini%20Agenda%20-%20Audit%20Documentation%20et%20Manques.md) | Audit de la documentation et manques pour un service complet. |
| [Référence — Niveaux Sécurité et Protection](./reference/Miyukini%20Agenda%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Détail des niveaux WorrySentinel (0–4) et des mesures de protection pour les données agenda. |
| [Référence — Integration Services Consommateurs](./reference/Miyukini%20Agenda%20-%20Integration%20Services%20Consommateurs.md) | Schémas d’intégration avec JayRDV, MFS, futurs services. |
| [Référence — Referentiel Inspire Google Agenda](./reference/Miyukini%20Agenda%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | Référentiel fonctionnel inspiré de Google Agenda (vues, rappels, partage, libre/occupé, calendriers multiples). |

## Arborescence

```
docs/services/MiyukiniAgenda/
├── _index.md
├── Miyukini Agenda - Document Fondateur.md
├── Miyukini Agenda - Ecrans et UI.md
├── Miyukini Agenda - Parcours Utilisateurs.md
├── Miyukini Agenda - Bornage Implementation.md
├── Miyukini Agenda - Operateurs et Toolkits.md
├── Miyukini Agenda - Audit Documentation et Manques.md
└── reference/
    ├── _index.md
    ├── Miyukini Agenda - Niveaux Securite et Protection Donnees.md
    ├── Miyukini Agenda - Integration Services Consommateurs.md
    └── Miyukini Agenda - Referentiel Fonctionnel Inspire Google Agenda.md
```

## Services consommateurs

| Service | Usage principal |
|--------|------------------|
| **JayRDV** | RDV, créneaux, plannings ; conflits ; vue calendrier ; export. |
| **Miyukini Festival Service** | Agenda cross-événements ; conflits de dates (exposants, visiteurs) ; vue calendrier ; export. |
| **Futurs services** | Tout service intervenant sur le domaine agenda (formations, interventions, etc.). |

## Voir aussi

- [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md)
- [Miyukini Festival Service - Document Fondateur](../MiyukiniFestivalService/Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
- [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)

---

*Dernière mise à jour : 2026-01-31*
