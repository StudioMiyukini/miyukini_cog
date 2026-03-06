# Ressources -- miyucloud-integration-fix

## Sequences liees
- `2026-03-05-miyucloud-v2-reprise` : Sequence precedente (bloquee V6/V8)

## Memoire projet
- `.mip/memory/project-miyucloud.md` : Architecture, crypto, defauts
- `.mip/memory/patterns-and-lessons.md` : Patterns et anti-patterns

## Code source
- `apps/miyucloud/` : Serveur MiyuCloud (API + surface web)
- `crates/miyucloud/` : Bibliotheque core (crypto, data, domain, sync)
- `apps/central/src/services/miyucloud/` : UI Dioxus dans Central (14 fichiers)
- `apps/central/src/service_manager/launcher.rs` : Lancement des services

## Audits precedents
- Score securite George : 87/100
- Score securite Victor (approfondi) : 72/100
- Defauts en attente : F-01 (download sans session), F-02 (timing attack), F-03 (passphrase defaut)
  - F-02 corrige en V1 de la sequence precedente
  - F-03 corrige (passphrase obligatoire, refus de demarrage)
