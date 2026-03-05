# MiyukiniWatch â€” Index du service

## Contexte

**MiyukiniWatch** est un **Service silencieux** toujours actif qui mesure les habitudes et les interactions de l'utilisateur avec le COG. Il ne lit pas les contenus ; il enregistre uniquement des mÃ©triques d'usage (temps, frÃ©quence, services, amis, clics). Silencieux mais **pas opaque** : l'utilisateur peut ouvrir le service comme tout autre, consulter les mesures prises et dÃ©cider de les effacer ou de les conserver.

Les donnÃ©es alimentent **Miou** (avatar/mascotte des COGs) pour adapter ses bulles, rappels et suggestions au bien-Ãªtre et aux habitudes de l'utilisateur.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : SpÃ©cification complÃ¨te du service â€” vision, architecture, mÃ©triques, gouvernance, interface, sÃ©curitÃ©, intÃ©gration Miou, contraintes.
- **Audience** : Ã‰quipes produit, architecture, sÃ©curitÃ©, dÃ©veloppeurs Central.

## Documents

| Document | Description |
|----------|-------------|
| [MiyukiniWatch - Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision, principe silencieux/non opaque, mÃ©triques initiales, non-lecture des contenus, interface utilisateur, effacement, lien avec Miou. |
| [MiyukiniWatch - Architecture et Positionnement](./MiyukiniWatch%20-%20Architecture%20et%20Positionnement.md) | Positionnement dans la Pyramide Miyukini, OpÃ©rateurs, interactions avec les Cores, flux de gouvernance, dÃ©pendances, conformitÃ© aux Lois d'Autonomie. |
| [MiyukiniWatch - SpÃ©cification Fonctionnelle MÃ©triques et Collecte](./MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Catalogue exhaustif des mÃ©triques (sessions, services, amis, clics, cycle de vie), Ã©vÃ©nements dÃ©clencheurs, structures de donnÃ©es, agrÃ©gation, limites. |
| [MiyukiniWatch - Gouvernance DonnÃ©es et RÃ©tention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | SouverainetÃ© locale, politique de rÃ©tention (brut/quotidien/hebdomadaire), purge automatique, droits de l'utilisateur (effacement, dÃ©sactivation), audit, migration. |
| [MiyukiniWatch - Interface Utilisateur et Ã‰crans](./MiyukiniWatch%20-%20Interface%20Utilisateur%20et%20Ecrans.md) | Ã‰crans (tableau de bord, dÃ©tail, paramÃ¨tres, audit), composants UI, flux de navigation, principes UX, accessibilitÃ©. |
| [MiyukiniWatch - IntÃ©gration Miou et AgrÃ©gats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Contrat d'intÃ©gration MiyukiniWatch â†’ Miou, catalogue des agrÃ©gats exposÃ©s, rÃ¨gles de consommation, exemples de bulles, dÃ©gradation gracieuse. |
| [MiyukiniWatch - Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | Contraintes non nÃ©gociables (non-lecture des contenus, localitÃ©, transparence, gouvernance Cores), invariants architecturaux, de donnÃ©es et d'intÃ©gration, matrice de vÃ©rification. |
| [MiyukiniWatch - SÃ©curitÃ© et ConformitÃ©](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md) | Classification des donnÃ©es, chiffrement au repos, contrÃ´les d'accÃ¨s, Ã©tats de confiance (T0â€“T4), modÃ¨le de menaces, audit sÃ©curitÃ©, conformitÃ© code. |
| [MiyukiniWatch - Guide d'ImplÃ©mentation Complet](./MiyukiniWatch%20-%20Guide%20Implementation%20Complet.md) | Guide exhaustif d'implÃ©mentation : phases, schÃ©mas, opÃ©rateurs, intÃ©gration Central/Miou, matrices de vÃ©rification, checklist. |

## Arborescence

```
docs/services/MiyukiniWatch/
â”œâ”€â”€ _index.md
â”œâ”€â”€ MiyukiniWatch - Document Fondateur.md
â”œâ”€â”€ MiyukiniWatch - Architecture et Positionnement.md
â”œâ”€â”€ MiyukiniWatch - Specification Fonctionnelle Metriques et Collecte.md
â”œâ”€â”€ MiyukiniWatch - Gouvernance Donnees et Retention.md
â”œâ”€â”€ MiyukiniWatch - Interface Utilisateur et Ecrans.md
â”œâ”€â”€ MiyukiniWatch - Integration Miou et Agregats.md
â”œâ”€â”€ MiyukiniWatch - Contraintes et Invariants.md
â”œâ”€â”€ MiyukiniWatch - Securite et Conformite.md
â””â”€â”€ MiyukiniWatch - Guide Implementation Complet.md
```

## Concepts clÃ©s

| Concept | Description |
|---------|-------------|
| **Service silencieux** | Toujours actif en arriÃ¨re-plan ; n'affiche rien tant que l'utilisateur n'ouvre pas le service. |
| **Pas opaque** | DonnÃ©es consultables et maÃ®trisables par l'utilisateur (voir, effacer, garder, dÃ©sactiver). |
| **Pas de lecture de contenus** | MiyukiniWatch ne lit pas le contenu des messages, des champs saisis ou des fichiers ; uniquement des mÃ©triques d'interaction et de timing. |
| **Miou** | Avatar/mascotte des COGs ; utilise les agrÃ©gats de MiyukiniWatch (et le profil utilisateur) pour adapter ses bulles et son rÃ´le bienveillant. |
| **AgrÃ©gats** | DonnÃ©es prÃ©-calculÃ©es (rÃ©sumÃ©s, tendances, classements) exposÃ©es Ã  Miou ; jamais les donnÃ©es brutes. |
| **RÃ©tention** | Politique configurable : brut (30 j.), quotidien (90 j.), hebdomadaire (365 j.). Purge automatique en cascade. |
| **OpÃ©rateurs** | Collector (collecte passive), Aggregator (agrÃ©gation pÃ©riodique), Presenter (interface utilisateur). |
| **Gouvernance Cores** | Toute opÃ©ration est gouvernÃ©e : StrongFather (dÃ©cision), KindMother (persistance), Master Butler (permissions), WorrySentinel (sÃ©curitÃ©), Border Guard (frontiÃ¨res). |

## Type de Service

**Service interne COG (Type 1)** â€” Espace Miyukini Central uniquement. Aucune surface web externe ; donnÃ©es strictement locales au COG.

## Voir aussi

- [Miyukini Central â€” Miou, avatar, bulles et rÃ´le](..//..//_index.md)
- [Miou â€” Index du sous-service](../MiyukiniCentral/Miou/_index.md)
- [Miyukini Central â€” Salon propositions lieu de vie gamification Miou](../MiyukiniCentral/Miyukini%20Central%20-%20Salon%20propositions%20lieu%20de%20vie%20gamification%20Miou.md)
- [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md)
- [Security â€” Liste des Mesures de SÃ©curitÃ©](..//..//cores//WorrySentinel//_index.md)

---

*DerniÃ¨re mise Ã  jour : 2026-02-15*


