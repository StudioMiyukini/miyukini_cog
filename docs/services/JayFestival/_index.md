# JayFestival

## Présentation

**JayFestival** est le service Miyukini dédié à la gestion d’événements et de festivals en B2B2C. Il reprend les spécificités de Catakana Orga et les développe pour l’organisation de la macro (catalogue, annuaires) et la distribution du service auprès des **organisateurs**, des **exposants** et des **visiteurs**. Tous les comptes sont **cross-événements**.

## Documentation

| Document | Description |
|----------|-------------|
| [JayFestival — Document fondateur](./JayFestival%20-%20Document%20Fondateur.md) | Document fondateur du service : raison d'être, vision, macro, distribution (organisateurs, exposants, visiteurs), positionnement. |
| [**Analyse approfondie Catakana Orga**](./JayFestival%20-%20Analyse%20Approfondie%20Catakana%20Orga.md) | **[NOUVEAU]** Analyse exhaustive du proto-service Catakana Orga (72 pages) : stack technique complète, modèles de données, UI/UX Atomic Design, fonctionnalités par module, parcours utilisateur, recommandations d'adaptation Rust/Dioxus, priorisation implémentation. |
| [Structure par public cible](./publics/_index.md) | Documentation organisée par type de public : **Organisateurs**, **Exposants**, **Visiteurs**, **Utilisateur non connecté**. |
| [Référence — Interpolarité](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages JayFestival avec JayXpose, JayFaim, JayKoa, JayKonta ; rôle de JayFestival dans chaque couplage. |
| [Référence — État documentation services interfacés](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) | Audit doc de chaque service interfacé (Jay, Miyu*, Cores) pour implémentation complète UI ; manques ; ambiguïtés et choix humains à trancher. |
| [Audit documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) | Audit qualité doc vs projet Catakana ; métriques, manques, recommandations. |
| [Bornage implémentation](./JayFestival%20-%20Bornage%20Implementation.md) | Périmètre MVP / phase 2, hors scope, dépendances, critères de livraison. |
| [Plan d'implémentation exhaustif](./JayFestival%20-%20Plan%20Implementation.md) | Plan d'implémentation JayFestival et services dépendants : phases, nomenclature [xx]-[fichier], MSCM, todo list (protocoles Implémentation générale et MIP v1). |
| [Référence UI — Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) | UI complète Catakana retranscrite dans la stack actuelle : Atomic, thème, ui-kit, écrans (Dioxus). |
| [Spécification UI conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | Spec normative : protocoles d'implémentation, atoms/molecules/organisms détaillés, parcours par écran (composants ordonnés), checklist conformité. |
| [Référence — Base de données et migration Supabase → SQLite](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | Interactions DB Supabase (Catakana), mapping tables/services → Kits ; migration vers SQLite + outils maison ; version alpha (Supabase en exception pré-COG). |
| [**Système Plan Interactif et Réservation Stands**](./JayFestival%20-%20Systeme%20Plan%20Interactif%20et%20Reservation%20Stands.md) | **[NOUVEAU]** Spécification complète : constructeur de plan (grille, positionnement relatif), réservation stands (workflow 3 états, temps réel, admin manuel), schémas SQL KindMother, types Rust, composants UI Dioxus, parcours utilisateur. |
| [**Plan du Service**](./JayFestival%20-%20Plan%20du%20Service.md) | **[NOUVEAU]** Plan du service (sitemap) complet : 71 écrans cartographiés, hiérarchie par public (UNC/ORG/EXP/VIS), mocks ASCII de navigation, wireframes des écrans clés, layouts, menus sidebar par rôle, passerelles inter-espaces, interconnexions services Jay. |

## Structure par public cible

| Public | Lien |
|--------|------|
| **Organisateurs** | [publics/Organisateurs](./publics/Organisateurs/_index.md) |
| **Exposants** | [publics/Exposants](./publics/Exposants/_index.md) |
| **Visiteurs** | [publics/Visiteurs](./publics/Visiteurs/_index.md) |
| **Utilisateur non connecté** | [publics/UtilisateurNonConnecte](./publics/UtilisateurNonConnecte/_index.md) |

## Interpolarité (services Jay)

JayFestival s’intègre avec les services Jay suivants :

| Service | Rôle du couplage |
|---------|-------------------|
| **JayXpose** | Fiche exposant et répertoire s’appuient sur le profil/vitrine JayXpose. |
| **JayFaim** | Restauration sur événement (stands, food trucks, commandes, créneaux). |
| **JayKoa** | Agenda agrégé ; éditions, participations, conflits de dates. |
| **JayKonta** | Budget édition, devis et factures exposants. |

**Référence** : [JayFestival - Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) ; [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md).

## Voir aussi

- [Audit Catakana → Miyukini COG B2B2C](../../modules/Catakana%20-%20Audit%20Conversion%20Miyukini%20COG%20B2B2C.md)
- [Miyukini Conceptual References — Vision stratégique](../../reference/Miyukini%20Conceptual%20References%20-%20Vision%20Strategique.md)
- [Miyukini Conceptual References — Interpolarité des services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md)
- [Miyukini — Stack UI Dioxus](../../ux_ui/Miyukini%20-%20Stack%20UI%20Dioxus.md)