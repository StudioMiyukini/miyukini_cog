# Specification mws-central-origin-reconnect

## Statut

- Etat : Validee pour implementation locale
- Phase : P0 Temps 6
- Responsable principal : Codex

## TL;DR

Central doit pouvoir se reconnecter au Webway Origin automatiquement apres login avec un comportement configurable et persiste. Le participant MWS ne doit plus annoncer ou interroger un tracker arbitraire: il doit utiliser la liste officielle recue dans `REGISTER_OK`.

## Exigences fonctionnelles

- Central charge une configuration MWS persistante au demarrage.
- `auto_connect` reste actif par defaut mais doit etre modifiable depuis la vue MWS.
- L'onglet Communaute doit rendre la vraie vue MWS et non un placeholder.
- Si l'utilisateur est connecte et si `enabled && !lone_mode && auto_connect`, Central tente la connexion MWS apres login.
- Relay et Tracker affiches/utilises par Central doivent refleter la configuration persistante.

## Exigences de securite

- Le participant parse la liste des trackers officiels depuis `REGISTER_OK`.
- Le Tracker client memorise cette liste et bascule automatiquement vers le premier tracker officiel si l'adresse configuree n'est pas officielle.
- Si Origin ne fournit aucun tracker officiel, la sequence de mise en ligne doit echouer.
- Les timeouts declares dans `RelayClientConfig` et `TrackerClientConfig` doivent etre appliques aux connexions TCP/TLS.

## Contraintes

- Le Relay Origin reste en TLS sur le port `7000`.
- Le Tracker Origin reste sur le port `21000`.
- Le binaire Central actuel n'expose pas la base JayXpose dans `ServiceConnections`; l'exposition vitrine n'est donc pas forcee dans cette sequence.

## Criteres d'acceptation

- `cargo test -p miyuwebway_participant` passe.
- `cargo check -p miyukini-central-native` passe.
- La vue MWS permet de sauvegarder Relay, Tracker, bind Home, adresse publique et toggles d'automatisation.
- Le code du participant refuse d'utiliser un tracker hors liste officielle.
