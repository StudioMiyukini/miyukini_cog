# Trace P0

## Statut

- Etat : Termine
- Phase : P0
- Responsable principal : Codex

## TL;DR

Exploration locale du cablage MWS Central et du participant pour identifier la vraie source de configuration, les chemins de connexion actifs et les surfaces de durcissement.

## Constat principal

- `MainTab::Communaute` devait etre recable sur une vue MWS completement pilotee par une config persistante.
- `CentralMwsConfig` contenait deja `auto_connect`, `auto_reconnect`, `relay_address` et `tracker_address`, mais l'UI reconstruisait encore une config ad hoc.
- `REGISTER_OK` d'Origin contient `tracker_addresses` et `tracker_signature`, mais le participant ignorait encore ces champs.
- `RelayClientConfig.connect_timeout` et `TrackerClientConfig.connect_timeout` etaient declares sans etre appliques.
- Le Relay et le Tracker du VPS Origin sont references localement sur `7000` et `21000`.
- `ServiceConnections` dans Central n'embarque actuellement que `auth_db` et `connect`; l'exposition JayXpose ne peut pas etre forcee ici.
