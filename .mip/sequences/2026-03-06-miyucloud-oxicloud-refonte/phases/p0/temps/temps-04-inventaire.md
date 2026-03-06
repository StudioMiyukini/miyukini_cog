# P0 Temps 4 - Inventaire prerequis

## Statut

- Etat : Termine
- Phase : P0 Temps 4
- Responsable principal : Denis + Hugo + Jean
- Debut : 2026-03-06T13:59:19Z
- Fin : 2026-03-06T14:05:06Z

## TL;DR

ALERTE : OxiCloud est DEJA sur axum 0.8.8 (pas actix-web). Migration framework = non-probleme. 23 crates a evaluer/ajouter. 10 etapes macro (E0-E10), 20-30 sessions estimees. Budget ~$60-90. Binaire unique inchange, deploiement SQLite embedded.

## Denis -- Inventaire crates

### Crates a ajouter
quick-xml 0.39, mime_guess 2.0, moka 0.12, image 0.25, infer 0.19, percent-encoding 2.3, http-range-header 0.4, fs2 0.4, async-stream 0.3, flate2 1.1, mockall 0.14 (dev)

### Crates a NE PAS ajouter
sqlx (rester rusqlite), jsonwebtoken (miyukini-connect), mimalloc

### Crates COG reutilises
miyucloud (crate lib), miyucloud-server (app), miyukini-kernel, kindmother, miyukini-connect, miyauth, miyuweb

### 10 etapes macro
E0 Fork & Analyse | E1 Schema & Types | E2 Dedup & Compression | E3 WebDAV core | E4 WebDAV avance | E5 CalDAV | E6 CardDAV | E7 Thumbnails | E8 WOPI | E9 Integration & Tests | E10 Hardening

### Modules a creer
- `crates/miyucloud/` : webdav/, caldav/, carddav/, dedup/, compression/, wopi/, thumbnails/
- `apps/miyucloud/` : webdav/, caldav/, carddav/, wopi/, api/calendars.rs, api/contacts.rs

## Lise -- Tests et UI frontend Dioxus

- Tests unitaires et integration pour chaque module
- Composants UI Dioxus pour Central (explorer, calendrier, contacts)
- 26 composants miyuki-ui-dioxus reutilisables, 13 a creer

## Hugo -- Infrastructure

- Pas de port supplementaire : WebDAV/CalDAV/CardDAV sur 11442 path-based
- Binaire unique inchange, SQLite embarque
- CI: ajouter cache crates, tests integration WebDAV, +30-60s build (image crate)
- Health checks: etendre /admin/stats avec metriques CalDAV/CardDAV/dedup

## Jean -- Budget

- Modele : Sonnet 4 (tous agents), Opus pour decisions archi Denis
- Budget estime : ~$60-90 USD (6.6M tokens)

## Artefact source

Voir [P0-T4-inventaire.md](../../../briefs/2026-03-06-miyucloud-oxicloud-refonte-P0-T4-inventaire.md)
