# P0 Temps 8 - Plan execution

## Statut

- Etat : Termine
- Phase : P0 Temps 8
- Responsable principal : Denis
- Debut : 2026-03-06T14:10:35Z
- Fin : 2026-03-06T14:16:14Z

## TL;DR

88 taches en 10 etapes + buffer corrections (20%). 5 agents : Francois (~40 taches), Lise (~25), Hugo (~10), Victor (~8), Denis (~5). DAG : E0 -> E1 -> {E2, E3, E6} -> {E4, E5, E7} -> E8 -> E9 -> E10. Parallelisme intra-etape marque [P].

## DAG de dependances

```
E0 --> E1 --> E2 (dedup)     \
              E3 (WebDAV) --> E4 (CalDAV) --> E8 --> E9 --> E10
              E6 (thumbs)    E5 (CardDAV) /
                             E7 (WOPI)   /
```

## Etapes

| Etape | Titre | Taches | Agents principaux |
|-------|-------|--------|-------------------|
| E0 | Fork OxiCloud & Fondations | 8 | Francois, Denis, Victor |
| E1 | Schema SQL & Types domaine | 10 | Francois, Lise |
| E2 | Dedup & Compression | 8 | Francois, Lise |
| E3 | WebDAV Core | 12 | Francois, Hugo, Lise, Victor |
| E4 | CalDAV | 8 | Francois, Lise |
| E5 | CardDAV | 7 | Francois, Lise |
| E6 | Thumbnails | 5 | Francois, Lise |
| E7 | WOPI (Office Online) | 6 | Hugo, Lise |
| E8 | Integration Central | 6 | Hugo, Lise |
| E9 | Integration & Tests E2E | 6 | Lise, Denis |
| E10 | Hardening & Audit Final | 8 | Victor, Denis, Lise |
| BUF | Buffer corrections (20%) | 4 | Francois, Lise |
| **TOTAL** | | **88** | **5 agents** |

## Categories de taches

- CODE : implementation
- TEST-U : tests unitaires
- TEST-I : tests integration
- TEST-S : tests securite
- AUDIT : conformite
- INFRA : setup
- DOC : documentation

## Artefact source

Voir [plan.md](../../../plans_p3/2026-03-06-miyucloud-oxicloud-refonte-plan.md)
