# MWS — Protocole Relay

## Vue d'Ensemble

Le **protocole Relay** définit comment les COGs et les nœuds MWS (Origin, Relays, Trackers) échangent des messages : formats, étapes de vérification et délivrance des Permis de circulation.

## Phases de Vérification

| Phase | Acteur | Contenu |
|-------|--------|---------|
| **Phase A** | Origin | Identité, manifeste, adresse canonique |
| **Phase B** | Relay | Vérification (MIP, conformité), délivrance Permis |
| **Phase C** | Tracker | Enregistrement, heartbeat, découverte |

## Flux de Messages

1. **Demande de Permis** : Le COG participant envoie une requête (identité, version) au Relay.
2. **Vérification** : Le Relay vérifie conformité et signatures (MIP si applicable).
3. **Émission Permis** : Si OK, le Relay (ou l’Origin) émet un Permis de circulation signé.
4. **Enregistrement Tracker** : Le participant présente son Permis au Tracker et s’inscrit au pool.
5. **Communication** : Les COGs utilisent leur Permis pour établir des tunnels sécurisés entre eux.

## Passeport et Visa

- **Passeport** : Document d’identité du COG (équivalent conceptuel).
- **Visa / Permis de circulation** : Autorisation temporaire ou scopée à circuler sur le Webway, délivrée après vérification.

Documentation détaillée : `docs/miyukini-webway-system/verification/`, `protocole/`.
