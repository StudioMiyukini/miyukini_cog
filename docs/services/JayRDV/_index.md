# JayRDV — Index du service

## Contexte

**JayRDV** est le service dédié à la **prise de rendez-vous et à la réservation en ligne** au sein de l'écosystème Miyukini. Ce dossier regroupe la documentation produit, les analyses de marché et les spécifications liées aux solutions de réservation en ligne.

## Portée / Scope

- **Périmètre** : Analyse produit, benchmark des fonctionnalités, spécifications du service JayRDV.
- **Audience** : Équipes produit, technique, parties prenantes.

## Documents

| Document | Description |
|----------|-------------|
| [JayRDV - Document Fondateur](./JayRDV%20-%20Document%20Fondateur.md) | Contexte, raison d'être et portée du service. |
| [JayRDV - Specification Complete du Service](./JayRDV%20-%20Specification%20Complete%20du%20Service.md) | **Spécification exhaustive** : vision, modèle de données, flux, écrans, gouvernance, contraintes, roadmap. Exclusion médicale (→ JayBobo). |
| [JayRDV - Accessibilite Client et Parcours de Reservation](./JayRDV%20-%20Accessibilite%20Client%20et%20Parcours%20de%20Reservation.md) | Stratégies d'accessibilité (web, PWA, mini COG, fédération) et parcours de réservation. |
| [JayRDV - Fonctionnalites Solutions Reservation en Ligne](./reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md) | Analyse produit : fonctionnalités identifiées des solutions de réservation en ligne (benchmark). |

## Publics

La documentation produit est structurée par **public** : Professionnels, Clients, Utilisateur non connecté.

| Public | Description | Documents |
|--------|-------------|-----------|
| [Professionnels](./publics/Professionnels/_index.md) | Praticiens, entreprises, équipes qui exposent des créneaux et gèrent les réservations. | Analyse des besoins, Parcours capacités livrables. |
| [Clients](./publics/Clients/_index.md) | Particuliers qui réservent (guest ou avec compte), espace « Mes RDV ». | Analyse des besoins, Parcours capacités livrables. |
| [Utilisateur non connecté](./publics/UtilisateurNonConnecte/_index.md) | Visiteurs qui réservent sans compte (parcours guest), Façade publique. | Analyse des besoins, Parcours et accès. |

## Arborescence

```
docs/services/JayRDV/
├── _index.md
├── JayRDV - Document Fondateur.md
├── JayRDV - Specification Complete du Service.md
├── JayRDV - Accessibilite Client et Parcours de Reservation.md
├── reference/
│   └── JayRDV - Fonctionnalites Solutions Reservation en Ligne.md
└── publics/
    ├── Professionnels/
    │   ├── _index.md
    │   ├── Professionnels - Analyse des besoins.md
    │   └── Professionnels - Parcours Capacites Livrables.md
    ├── Clients/
    │   ├── _index.md
    │   ├── Clients - Analyse des besoins.md
    │   └── Clients - Parcours Capacites Livrables.md
    └── UtilisateurNonConnecte/
        ├── _index.md
        ├── UtilisateurNonConnecte - Analyse des besoins.md
        └── UtilisateurNonConnecte - Parcours et acces.md
```

---
*Dernière mise à jour : 2026-01-31*
