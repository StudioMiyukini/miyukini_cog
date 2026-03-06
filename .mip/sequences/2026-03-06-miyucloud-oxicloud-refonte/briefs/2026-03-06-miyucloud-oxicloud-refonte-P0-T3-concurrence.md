# P0 Temps 3 -- Analyse concurrentielle (Fabrice)

**Sequence** : 2026-03-06-miyucloud-oxicloud-refonte
**Debut** : 2026-03-06T13:55:20Z | **Fin** : 2026-03-06T13:59:19Z

## TL;DR
6 concurrents analyses (Nextcloud, Seafile, oCIS, Syncthing, FileRun, OxiCloud). Miyukini Cloud se positionne comme cloud prive ultra-leger (<128 Mo), securise (Rust), complet (WebDAV/CalDAV/CardDAV), integre nativement a l'ecosysteme COG. Cible : ceux qui trouvent Nextcloud trop lourd et Syncthing trop limite.

## Positionnement strategique
*"Le cloud prive qui tient dans 128 Mo -- fichiers, calendriers, contacts, zero compromis."*

## Avantages differentieurs
1. Rust + axum + SQLite : binaire unique, empreinte < 128 Mo
2. Integration ecosysteme Miyukini (Central, MWS, Market) -- unique
3. UI native Dioxus (pas de stack JS separee)
4. Securite certif-ready (memory-safe, chiffrement par defaut)
5. Protocoles complets (WebDAV + CalDAV + CardDAV)

## Faiblesses exploitables des concurrents
- Nextcloud : lourd (512Mo-2Go RAM), install complexe, bloat
- Seafile : pas de CalDAV/CardDAV
- oCIS : pas de CalDAV/CardDAV (issue #4130)
- Syncthing : pas de web UI, pas de partage
- FileRun : proprietaire, pas de CalDAV

## Risques concurrentiels
- Communaute inexistante (vs Nextcloud 400k+)
- Pas de client mobile natif (clients WebDAV tiers en attendant)
- OxiCloud upstream instable (fork avec cherry-pick selectif)
