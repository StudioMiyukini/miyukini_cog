# JayFestival

**JayFestival** est le service Miyukini dÃ©diÃ© Ã  la **gestion dâ€™Ã©vÃ©nements et de festivals** en B2B2C. Il reprend les spÃ©cificitÃ©s de Catakana Orga et les dÃ©veloppe pour lâ€™organisation de la macro (catalogue, annuaires) et la distribution du service auprÃ¨s des **organisateurs**, des **exposants** et des **visiteurs**. **Tous les comptes sont cross-Ã©vÃ©nements.**

## Documentation

| Document | Description |
|----------|-------------|
| [Document fondateur](./JayFestival%20-%20Document%20Fondateur.md) | Raison dâ€™Ãªtre, vision, macro, distribution (organisateurs, exposants, visiteurs), positionnement. |
| [Index de la documentation](./_index.md) | Vue dâ€™ensemble, structure par public, interpolaritÃ©, audit, bornage, rÃ©fÃ©rence UI. |
| [Audit documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) | Audit qualitÃ© doc vs Catakana ; mÃ©triques, manques, recommandations. |
| [Bornage implÃ©mentation](./JayFestival%20-%20Bornage%20Implementation.md) | PÃ©rimÃ¨tre MVP / phase 2, hors scope, dÃ©pendances, critÃ¨res de livraison. |
| [**Documentation de l'implÃ©mentation**](./JayFestival%20-%20Implementation.md) | Architecture, structure du code, flux de donnÃ©es, Ã©tat des tests. |
| [RÃ©fÃ©rence UI â€” Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) | UI complÃ¨te Catakana retranscrite dans la stack actuelle (Atomic, thÃ¨me, ui-kit, Ã©crans Dioxus). |
| [SpÃ©cification UI conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | Spec normative : protocoles, atoms/molecules/organisms dÃ©taillÃ©s, parcours par Ã©cran, checklist conformitÃ©. |
| [RÃ©fÃ©rence â€” Base de donnÃ©es et migration Supabase â†’ SQLite](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | DB Supabase (Catakana), mapping tables/services, migration SQLite + outils maison, alpha (Supabase exception prÃ©-COG). |
| [InterpolaritÃ© avec les services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages avec JayXpose, JayFaim, JayKoa, JayKonta. |
| [**Connexions et synchronisation**](./reference/JayFestival%20-%20Connexions%20Synchronisation%20Services%20Jay.md) | DÃ©pendances, liaisons, bornes, implÃ©mentation sync JayKoa, sync JayXpose et annuaire exposants. |
| [Ã‰tat documentation services interfacÃ©s](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) | Audit doc de chaque service interfacÃ© pour implÃ©mentation complÃ¨te UI ; ambiguÃ¯tÃ©s et dÃ©cisions Ã  trancher. |

## InterpolaritÃ©

JayFestival sâ€™intÃ¨gre avec **JayXpose** (fiche/rÃ©pertoire exposants), **JayFaim** (restauration sur Ã©vÃ©nement), **JayKoa** (agenda agrÃ©gÃ©), **JayKonta** (budget, facturation). Voir le [document de rÃ©fÃ©rence InterpolaritÃ©](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) et le [document global InterpolaritÃ© des services Jay](..//..//miyukini-webway-system//reference//_index.md).

---

*Documentation Miyukini â€” JayFestival*

