# JayFestival

**JayFestival** est le service Miyukini dédié à la **gestion d’événements et de festivals** en B2B2C. Il reprend les spécificités de Catakana Orga et les développe pour l’organisation de la macro (catalogue, annuaires) et la distribution du service auprès des **organisateurs**, des **exposants** et des **visiteurs**. **Tous les comptes sont cross-événements.**

## Documentation

| Document | Description |
|----------|-------------|
| [Document fondateur](./JayFestival%20-%20Document%20Fondateur.md) | Raison d’être, vision, macro, distribution (organisateurs, exposants, visiteurs), positionnement. |
| [Index de la documentation](./_index.md) | Vue d’ensemble, structure par public, interpolarité, audit, bornage, référence UI. |
| [Audit documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) | Audit qualité doc vs Catakana ; métriques, manques, recommandations. |
| [Bornage implémentation](./JayFestival%20-%20Bornage%20Implementation.md) | Périmètre MVP / phase 2, hors scope, dépendances, critères de livraison. |
| [Référence UI — Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) | UI complète Catakana retranscrite dans la stack actuelle (Atomic, thème, ui-kit, écrans Dioxus). |
| [Spécification UI conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | Spec normative : protocoles, atoms/molecules/organisms détaillés, parcours par écran, checklist conformité. |
| [Référence — Base de données et migration Supabase → SQLite](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | DB Supabase (Catakana), mapping tables/services, migration SQLite + outils maison, alpha (Supabase exception pré-COG). |
| [Interpolarité avec les services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages avec JayXpose, JayFaim, JayKoa, JayKonta. |
| [État documentation services interfacés](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) | Audit doc de chaque service interfacé pour implémentation complète UI ; ambiguïtés et décisions à trancher. |

## Interpolarité

JayFestival s’intègre avec **JayXpose** (fiche/répertoire exposants), **JayFaim** (restauration sur événement), **JayKoa** (agenda agrégé), **JayKonta** (budget, facturation). Voir le [document de référence Interpolarité](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) et le [document global Interpolarité des services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md).

---

*Documentation Miyukini — JayFestival*
