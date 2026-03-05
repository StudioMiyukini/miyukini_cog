# JayFestival

## PrÃ©sentation

**JayFestival** est le service Miyukini dÃ©diÃ© Ã  la gestion dâ€™Ã©vÃ©nements et de festivals en B2B2C. Il reprend les spÃ©cificitÃ©s de Catakana Orga et les dÃ©veloppe pour lâ€™organisation de la macro (catalogue, annuaires) et la distribution du service auprÃ¨s des **organisateurs**, des **exposants** et des **visiteurs**. Tous les comptes sont **cross-Ã©vÃ©nements**.

## Documentation

| Document | Description |
|----------|-------------|
| [JayFestival â€” Document fondateur](./JayFestival%20-%20Document%20Fondateur.md) | Document fondateur du service : raison d'Ãªtre, vision, macro, distribution (organisateurs, exposants, visiteurs), positionnement. |
| [**Analyse approfondie Catakana Orga**](./JayFestival%20-%20Analyse%20Approfondie%20Catakana%20Orga.md) | **[NOUVEAU]** Analyse exhaustive du proto-service Catakana Orga (72 pages) : stack technique complÃ¨te, modÃ¨les de donnÃ©es, UI/UX Atomic Design, fonctionnalitÃ©s par module, parcours utilisateur, recommandations d'adaptation Rust/Dioxus, priorisation implÃ©mentation. |
| [Structure par public cible](./publics/_index.md) | Documentation organisÃ©e par type de public : **Organisateurs**, **Exposants**, **Visiteurs**, **Utilisateur non connectÃ©**. |
| [RÃ©fÃ©rence â€” InterpolaritÃ©](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages JayFestival avec JayXpose, JayFaim, JayKoa, JayKonta ; rÃ´le de JayFestival dans chaque couplage. |
| [**RÃ©fÃ©rence â€” Connexions et synchronisation**](./reference/JayFestival%20-%20Connexions%20Synchronisation%20Services%20Jay.md) | **DÃ©pendances Cargo, liaisons mÃ©tier, bornes, implÃ©mentation sync JayKoa, sync JayXpose et annuaire exposants** ; chemins de code. |
| [RÃ©fÃ©rence â€” Ã‰tat documentation services interfacÃ©s](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) | Audit doc de chaque service interfacÃ© (Jay, Miyu*, Cores) pour implÃ©mentation complÃ¨te UI ; manques ; ambiguÃ¯tÃ©s et choix humains Ã  trancher. |
| [Audit documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) | Audit qualitÃ© doc vs projet Catakana ; mÃ©triques, manques, recommandations. |
| [Bornage implÃ©mentation](./JayFestival%20-%20Bornage%20Implementation.md) | PÃ©rimÃ¨tre MVP / phase 2, hors scope, dÃ©pendances, critÃ¨res de livraison. |
| [**Documentation de l'implÃ©mentation**](./JayFestival%20-%20Implementation.md) | **Architecture actuelle, structure du code (crate + UI Central), flux de donnÃ©es, points d'entrÃ©e, Ã©tat des tests.** |
| [Plan d'implÃ©mentation exhaustif](./JayFestival%20-%20Plan%20Implementation.md) | Plan d'implÃ©mentation JayFestival et services dÃ©pendants : phases, nomenclature [xx]-[fichier], MSCM, todo list (protocoles ImplÃ©mentation gÃ©nÃ©rale et MIP v1). |
| [RÃ©fÃ©rence UI â€” Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) | UI complÃ¨te Catakana retranscrite dans la stack actuelle : Atomic, thÃ¨me, ui-kit, Ã©crans (Dioxus). |
| [SpÃ©cification UI conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | Spec normative : protocoles d'implÃ©mentation, atoms/molecules/organisms dÃ©taillÃ©s, parcours par Ã©cran (composants ordonnÃ©s), checklist conformitÃ©. |
| [RÃ©fÃ©rence â€” Base de donnÃ©es et migration Supabase â†’ SQLite](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | Interactions DB Supabase (Catakana), mapping tables/services â†’ Kits ; migration vers SQLite + outils maison ; version alpha (Supabase en exception prÃ©-COG). |
| [**SystÃ¨me Plan Interactif et RÃ©servation Stands**](./JayFestival%20-%20Systeme%20Plan%20Interactif%20et%20Reservation%20Stands.md) | **[NOUVEAU]** SpÃ©cification complÃ¨te : constructeur de plan (grille, positionnement relatif), rÃ©servation stands (workflow 3 Ã©tats, temps rÃ©el, admin manuel), schÃ©mas SQL KindMother, types Rust, composants UI Dioxus, parcours utilisateur. |
| [**Plan du Service**](./JayFestival%20-%20Plan%20du%20Service.md) | **[NOUVEAU]** Plan du service (sitemap) complet : 71 Ã©crans cartographiÃ©s, hiÃ©rarchie par public (UNC/ORG/EXP/VIS), mocks ASCII de navigation, wireframes des Ã©crans clÃ©s, layouts, menus sidebar par rÃ´le, passerelles inter-espaces, interconnexions services Jay. |

## Structure par public cible

| Public | Lien |
|--------|------|
| **Organisateurs** | [publics/Organisateurs](./publics/Organisateurs/_index.md) |
| **Exposants** | [publics/Exposants](./publics/Exposants/_index.md) |
| **Visiteurs** | [publics/Visiteurs](./publics/Visiteurs/_index.md) |
| **Utilisateur non connectÃ©** | [publics/UtilisateurNonConnecte](./publics/UtilisateurNonConnecte/_index.md) |

## InterpolaritÃ© (services Jay)

JayFestival sâ€™intÃ¨gre avec les services Jay suivants :

| Service | RÃ´le du couplage |
|---------|-------------------|
| **JayXpose** | Fiche exposant et rÃ©pertoire sâ€™appuient sur le profil/vitrine JayXpose. |
| **JayFaim** | Restauration sur Ã©vÃ©nement (stands, food trucks, commandes, crÃ©neaux). |
| **JayKoa** | Agenda agrÃ©gÃ© ; Ã©ditions, participations, conflits de dates. |
| **JayKonta** | Budget Ã©dition, devis et factures exposants. |

**RÃ©fÃ©rence** : [JayFestival - Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) ; [JayFestival - Connexions Synchronisation Services Jay](./reference/JayFestival%20-%20Connexions%20Synchronisation%20Services%20Jay.md) ; [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md).

## Voir aussi

- [Audit Catakana â†’ Miyukini COG B2B2C](..//..//_index.md)
- [Miyukini Conceptual References â€” Vision stratÃ©gique](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References â€” InterpolaritÃ© des services Jay](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini â€” Stack UI Dioxus](..//..//_index.md)

