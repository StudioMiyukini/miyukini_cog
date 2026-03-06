# Plan exhaustif -- miyucloud-oxicloud-refonte (INDEX)

**Sequence** : 2026-03-06-miyucloud-oxicloud-refonte
**Auteur** : Denis (Chef Dev) -- P0 T8
**Debut** : 2026-03-06T14:10:35Z | **Fin** : 2026-03-06T14:16:14Z
**Classification** : T5

## TL;DR
88 taches en 10 etapes + buffer corrections (20%). Agents P3 : Francois (backend), Lise (tests/front), Hugo (UI/API/WOPI), Denis (archi/review), Victor (securite). George intervient en P4 uniquement (audit qualite/sodomight). DAG : E0 -> E1 -> {E2, E3, E6} -> {E4, E5, E7} -> E8 -> E9 -> E10. Parallelisme intra-etape marque [P].

## INDEX DES ETAPES

| Etape | Titre | Taches | Agents | Depend |
|-------|-------|--------|--------|--------|
| E0 | Fork OxiCloud & Fondations | 8 | Francois, Denis, Victor | -- |
| E1 | Schema SQL & Types domaine | 10 | Francois, Lise | E0 |
| E2 | Dedup & Compression | 8 | Francois, Lise | E1 |
| E3 | WebDAV Core | 12 | Francois, Hugo, Lise, Victor | E1 |
| E4 | CalDAV | 8 | Francois, Lise | E3 |
| E5 | CardDAV | 7 | Francois, Lise | E3 |
| E6 | Thumbnails | 5 | Francois, Lise | E1 |
| E7 | WOPI (Office Online) | 6 | Hugo, Lise | E3 |
| E8 | Integration Central | 6 | Hugo, Lise | E3, E4, E5 |
| E9 | Integration & Tests E2E | 6 | Lise, Denis | E0-E8 |
| E10 | Hardening & Audit Final | 8 | Victor, Denis, Lise | E9 |
| BUF | Buffer corrections (20%) | 4 | Francois, Lise | E9 |
| **TOTAL** | | **88** | **5 agents** | |

## DAG de dependances

```
E0 --> E1 --> E2 (dedup)     \
              E3 (WebDAV) --> E4 (CalDAV) --> E8 --> E9 --> E10
              E6 (thumbs)    E5 (CardDAV) /
                             E7 (WOPI)   /
```

## Distribution agents

| Agent | Taches | Perimetre |
|-------|--------|-----------|
| Francois | ~40 | Backend: crate miyucloud-dav, storage, domain, schema |
| Lise | ~25 | Tests unitaires + integration + front |
| Hugo | ~10 | WOPI + integration Central + API routes |
| Victor | ~8 | Securite: XML defense, path validation, audit final |
| Denis | ~5 | Review archi, integration, coordination |
| George | P4 only | Audit qualite sodomight (hors plan P3) |

## Detail des taches
Le plan detaille (88 taches avec code, tests, commits) est conserve dans le contexte de la sequence. Chaque tache suit le format :
- [CATEGORIE-NN] Titre
- Agent, Fichier(s), depends, Description, Commande test, Message commit

Categories : CODE (implementation), TEST-U (tests unitaires), TEST-I (integration), TEST-S (securite), AUDIT (conformite), INFRA (setup), DOC (documentation)
