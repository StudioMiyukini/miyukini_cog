# MiyukiniWatch — Index du service

## Contexte

**MiyukiniWatch** est un **Service silencieux** toujours actif qui mesure les habitudes et les interactions de l'utilisateur avec le COG. Il ne lit pas les contenus ; il enregistre uniquement des métriques d'usage (temps, fréquence, services, amis, clics). Silencieux mais **pas opaque** : l'utilisateur peut ouvrir le service comme tout autre, consulter les mesures prises et décider de les effacer ou de les conserver.

Les données alimentent **Miou** (avatar/mascotte des COGs) pour adapter ses bulles, rappels et suggestions au bien-être et aux habitudes de l'utilisateur.

## Portée / Scope

- **Périmètre** : Spécification complète du service — vision, architecture, métriques, gouvernance, interface, sécurité, intégration Miou, contraintes.
- **Audience** : Équipes produit, architecture, sécurité, développeurs Central.

## Documents

| Document | Description |
|----------|-------------|
| [MiyukiniWatch - Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision, principe silencieux/non opaque, métriques initiales, non-lecture des contenus, interface utilisateur, effacement, lien avec Miou. |
| [MiyukiniWatch - Architecture et Positionnement](./MiyukiniWatch%20-%20Architecture%20et%20Positionnement.md) | Positionnement dans la Pyramide Miyukini, Opérateurs, interactions avec les Cores, flux de gouvernance, dépendances, conformité aux Lois d'Autonomie. |
| [MiyukiniWatch - Spécification Fonctionnelle Métriques et Collecte](./MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Catalogue exhaustif des métriques (sessions, services, amis, clics, cycle de vie), événements déclencheurs, structures de données, agrégation, limites. |
| [MiyukiniWatch - Gouvernance Données et Rétention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | Souveraineté locale, politique de rétention (brut/quotidien/hebdomadaire), purge automatique, droits de l'utilisateur (effacement, désactivation), audit, migration. |
| [MiyukiniWatch - Interface Utilisateur et Écrans](./MiyukiniWatch%20-%20Interface%20Utilisateur%20et%20Ecrans.md) | Écrans (tableau de bord, détail, paramètres, audit), composants UI, flux de navigation, principes UX, accessibilité. |
| [MiyukiniWatch - Intégration Miou et Agrégats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Contrat d'intégration MiyukiniWatch → Miou, catalogue des agrégats exposés, règles de consommation, exemples de bulles, dégradation gracieuse. |
| [MiyukiniWatch - Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | Contraintes non négociables (non-lecture des contenus, localité, transparence, gouvernance Cores), invariants architecturaux, de données et d'intégration, matrice de vérification. |
| [MiyukiniWatch - Sécurité et Conformité](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md) | Classification des données, chiffrement au repos, contrôles d'accès, états de confiance (T0–T4), modèle de menaces, audit sécurité, conformité code. |
| [MiyukiniWatch - Guide d'Implémentation Complet](./MiyukiniWatch%20-%20Guide%20Implementation%20Complet.md) | Guide exhaustif d'implémentation : phases, schémas, opérateurs, intégration Central/Miou, matrices de vérification, checklist. |

## Arborescence

```
docs/services/MiyukiniWatch/
├── _index.md
├── MiyukiniWatch - Document Fondateur.md
├── MiyukiniWatch - Architecture et Positionnement.md
├── MiyukiniWatch - Specification Fonctionnelle Metriques et Collecte.md
├── MiyukiniWatch - Gouvernance Donnees et Retention.md
├── MiyukiniWatch - Interface Utilisateur et Ecrans.md
├── MiyukiniWatch - Integration Miou et Agregats.md
├── MiyukiniWatch - Contraintes et Invariants.md
├── MiyukiniWatch - Securite et Conformite.md
└── MiyukiniWatch - Guide Implementation Complet.md
```

## Concepts clés

| Concept | Description |
|---------|-------------|
| **Service silencieux** | Toujours actif en arrière-plan ; n'affiche rien tant que l'utilisateur n'ouvre pas le service. |
| **Pas opaque** | Données consultables et maîtrisables par l'utilisateur (voir, effacer, garder, désactiver). |
| **Pas de lecture de contenus** | MiyukiniWatch ne lit pas le contenu des messages, des champs saisis ou des fichiers ; uniquement des métriques d'interaction et de timing. |
| **Miou** | Avatar/mascotte des COGs ; utilise les agrégats de MiyukiniWatch (et le profil utilisateur) pour adapter ses bulles et son rôle bienveillant. |
| **Agrégats** | Données pré-calculées (résumés, tendances, classements) exposées à Miou ; jamais les données brutes. |
| **Rétention** | Politique configurable : brut (30 j.), quotidien (90 j.), hebdomadaire (365 j.). Purge automatique en cascade. |
| **Opérateurs** | Collector (collecte passive), Aggregator (agrégation périodique), Presenter (interface utilisateur). |
| **Gouvernance Cores** | Toute opération est gouvernée : StrongFather (décision), KindMother (persistance), Master Butler (permissions), WorrySentinel (sécurité), Border Guard (frontières). |

## Type de Service

**Service interne COG (Type 1)** — Espace Miyukini Central uniquement. Aucune surface web externe ; données strictement locales au COG.

## Voir aussi

- [Miyukini Central — Miou, avatar, bulles et rôle](../MiyukiniCentral/Miyukini%20Central%20-%20Miou%20avatar%20bulles%20et%20role.md)
- [Miou — Index du sous-service](../MiyukiniCentral/Miou/_index.md)
- [Miyukini Central — Salon propositions lieu de vie gamification Miou](../MiyukiniCentral/Miyukini%20Central%20-%20Salon%20propositions%20lieu%20de%20vie%20gamification%20Miou.md)
- [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md)
- [Security — Liste des Mesures de Sécurité](../../security/reference/Security%20-%20Liste%20des%20Mesures%20de%20Securite%20Miyukini%20COG%20et%20MWS.md)

---

*Dernière mise à jour : 2026-02-15*
