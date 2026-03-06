# Trace P4

## Statut

- Etat : Termine
- Phase : P4
- Responsable principal : Codex

## TL;DR

Verification technique locale et controle des regressions de compilation introduites par les changements Central et participant.

## Verifications

- `cargo test -p miyuwebway_participant`
  - resultat : OK
  - couverture immediate : parser `REGISTER_OK`, mise a jour trackers officiels, creation service
- `cargo check -p miyukini-central-native`
  - resultat : OK
  - corrections intermediaires : styles Dioxus, mutabilite `mws_state`, retrait des references `jayxpose` absentes de `ServiceConnections`

## Risques restants

- Les warnings `cfg(feature = "service-jay1tribu")` restent presents et preexistaient a la sequence.
- La signature Ed25519 de `tracker_addresses` est maintenant transportee cote participant mais pas encore verifiee faute d'ancre de confiance cote Central/participant.
