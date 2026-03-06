# Plan P3 mws-central-origin-reconnect

## Statut

- Etat : Execute
- Phase : P3
- Responsable principal : Codex

## TL;DR

Plan d'execution realise en quatre blocs: persistance/autoconnexion Central, cablage UI Communaute, durcissement reseau du participant, puis verification.

## Etapes executees

1. Ajouter `mws_settings` et persister la configuration MWS dans Central.
2. Exposer `MwsNetworkView` et `MwsViewState` dans `apps/central/src/services/mod.rs`.
3. Brancher l'onglet Communaute sur la vraie vue MWS et tenter l'autoconnexion apres login.
4. Remplacer les anciennes branches `real_mws_connect` / `real_mws_disconnect` par les helpers unifies.
5. Ajouter une carte de configuration MWS persistante avec toggles `enabled`, `auto_connect`, `auto_reconnect` et edition d'endpoints.
6. Parser `REGISTER_OK` cote participant pour extraire les trackers officiels.
7. Forcer le Tracker client a utiliser uniquement la liste officielle et appliquer les timeouts de connexion.
8. Verifier avec les commandes Cargo cibles.
