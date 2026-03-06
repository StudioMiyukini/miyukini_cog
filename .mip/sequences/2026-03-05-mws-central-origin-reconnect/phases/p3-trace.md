# Trace P3

## Statut

- Etat : Termine
- Phase : P3
- Responsable principal : Codex

## TL;DR

Implementation locale de la persistance MWS Central, du panneau de configuration, de l'autoconnexion apres login et du verrouillage du Tracker client sur la liste officielle d'Origin.

## Modifications executees

- `apps/central/src/services/mws_settings.rs`
  - chargement/sauvegarde de `mws-config.json`
  - normalisation `home_http_bind` et `public_address`
  - derivation Relay/Tracker depuis `MIYUKINI_ORIGIN_URL`
- `apps/central/src/app.rs`
  - provider `MwsViewState`
  - tentative d'autoconnexion apres login selon la config persistante
- `apps/central/src/services/mws_view.rs`
  - insertion d'une carte de configuration persistante
  - remplacement des anciennes branches `real_mws_connect` / `real_mws_disconnect`
  - affichage des endpoints reels dans la carte d'etat
  - persistance du mode Lone et des toggles d'automatisation
- `crates/miyuwebway_participant/src/protocol.rs`
  - extraction de `official_trackers` et `tracker_signature` depuis `REGISTER_OK`
- `crates/miyuwebway_participant/src/relay_client.rs`
  - application effective des timeouts de connexion Relay/TLS
  - stockage des trackers officiels dans la session
- `crates/miyuwebway_participant/src/tracker_client.rs`
  - memorisation de la liste officielle
  - bascule automatique vers un tracker officiel
  - refusal implicite d'un tracker hors liste
  - application effective du timeout de connexion Tracker
- `crates/miyuwebway_participant/src/mws_service.rs`
  - propagation de la liste officielle avant ANNOUNCE
