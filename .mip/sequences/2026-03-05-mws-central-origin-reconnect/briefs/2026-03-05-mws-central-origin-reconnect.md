# Brief sequence mws-central-origin-reconnect

## Statut

- Etat : Lancee et executee localement
- Phase : P0 a P5
- Responsable principal : Codex

## TL;DR

Rebrancher Miyukini Webway System dans Central, activer l'autoconnexion par defaut avec un controle utilisateur persiste, cibler Relay et Tracker heberges sur le VPS d'Origin, et durcir le participant pour n'utiliser que les trackers officiels remis par Origin.

## Objectifs

- Remplacer le placeholder Communaute par la vraie vue MWS de Central.
- Persister la configuration MWS de Central et rendre `auto_connect` configurable.
- Lancer automatiquement la connexion MWS apres login quand la configuration l'autorise.
- Durcir la securite participant contre la derive vers un tracker non officiel.
- Appliquer effectivement les timeouts reseau declares dans Relay et Tracker.

## Decisions

- La configuration MWS de Central est stockee dans un fichier local dedie `mws-config.json`.
- Les adresses Origin par defaut sont derivees de `MIYUKINI_ORIGIN_URL` quand elle est disponible.
- La vue MWS expose un panneau de configuration persiste avec activation MWS, auto-connect, auto-reconnect et edition Relay/Tracker.
- Le participant parse `REGISTER_OK`, recupere `official_trackers` et force le Tracker client a utiliser exclusivement cette liste.
- L'integration JayXpose dans la connexion MWS Central n'est pas activee dans cette sequence car `ServiceConnections` ne transporte pas la base JayXpose dans ce binaire.

## Livrables

- Wiring UI MWS dans `apps/central`.
- Persistance de configuration et autoconnexion apres login.
- Enforcement des trackers officiels et timeouts reseau dans `crates/miyuwebway_participant`.
- Verification par `cargo test -p miyuwebway_participant` et `cargo check -p miyukini-central-native`.
