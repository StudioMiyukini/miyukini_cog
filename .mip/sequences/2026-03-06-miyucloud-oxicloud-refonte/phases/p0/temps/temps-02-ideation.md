# P0 Temps 2 - Ideation

## Statut

- Etat : Termine
- Phase : P0 Temps 2
- Responsable principal : Maria + Lise
- Debut : 2026-03-06T13:55:20Z
- Fin : 2026-03-06T13:59:19Z

## TL;DR

3 approches proposees (fork complet recommandee, extraction selective, wrapper/proxy). 9 risques identifies. Lise : 26 composants miyuki-ui-dioxus reutilisables, 13 a creer. Direction visuelle : theme COG sombre conserve, elimination emojis Unicode.

## Approches proposees

### Approche A -- Fork complet + migration progressive (RECOMMANDEE)
- Importer OxiCloud dans nouveau crate `miyucloud-dav`
- Etendre crate existant pour dedup/compression/thumbnails
- Migration module par module
- **Pour** : progression mesurable, tests reutilisables, architecture Clean
- **Contre** : volume ~2680 LOC, coexistence temporaire

### Approche B -- Extraction selective
- Extraire uniquement les modules domaine/application
- **Non recommandee** : perte architecture Clean/DDD

### Approche C -- Wrapper/Proxy
- Executer OxiCloud tel quel, proxier depuis MiyuCloud
- **Non recommandee** : deux processus, dependance PostgreSQL

## Risques identifies (9)
R1-R9 : SQLite adapter, conformite protocoles, volume code, CVE rusqlite, XXE XML, perte crypto E2E, perte sync P2P, maintenance fork, concurrence SQLite single-writer

## Direction visuelle (Lise)
- 26 composants miyuki-ui-dioxus reutilisables
- 13 composants a creer (CalDAV/CardDAV UI differe Phase 3 front)
- Elimination emojis Unicode -> icones vectorielles
- Migration tokens typographiques et espacement

## Artefact source

Voir [P0-T2-ideation.md](../../../briefs/2026-03-06-miyucloud-oxicloud-refonte-P0-T2-ideation.md)
