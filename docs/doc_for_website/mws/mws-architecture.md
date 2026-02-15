# MWS — Architecture et Subordination aux Cores

## Subordination aux Cores

Le MWS est **uniquement subordonné aux Cores**. Aucune décision MWS (connexion, envoi, réception, Permis) n’est exécutée sans validation des Cores concernés (StrongFather, BorderGuard, TAMR).

## Consommation par les Strates

| Strate | Usage du MWS |
|--------|----------------|
| **Cores** | Décision de participer, validation des flux, politique de fédération |
| **Outils** | MiyuWebwayParticipant, MiyuWebwayTracker exécutent sous instruction |
| **Opérateurs** | Services qui utilisent le réseau (partage, découverte) |
| **Services** | Fonctionnalités utilisateur (partage de document, sync optionnelle) |

## Schéma Simplifié

```
                    ORIGIN (source de vérité)
                              │
                              ▼
                         RELAYS
                    (vérification, Permis)
                              │
                              ▼
                        TRACKERS
                    (pools, découverte)
                              │
         ┌────────────────────┼────────────────────┐
         ▼                    ▼                    ▼
    COG A                 COG B                 COG C
    (Participant)         (Participant)         (Tracker)
```

## Flux de Vérification

1. **Phase A (Origin)** : Identité et manifeste.
2. **Phase B (Relay)** : Vérification (MIP, conformité), délivrance Permis.
3. **Phase C (Tracker)** : Enregistrement, heartbeat, découverte.
4. **Communication** : Tunnel étendu entre COGs, sous contrôle BorderGuard.

## Invariants

- Aucune donnée utilisateur stockée dans le MWS (sauf métadonnées de présence/config).
- Tout flux entrant/sortant passe par BorderGuard.
- Participation révocable à tout moment (LOI-6).

## Principe fondateur

> **Le Webway normalise la présence et facilite l'échange entre environnements ; il ne transporte pas la gouvernance ni les données métier — il permet de savoir où et comment initier une visite gouvernée.**

## Voir aussi

- [MWS - Document fondateur](mws-fondateur.md)
- [MWS - Origin](mws-origin.md), [Relays](mws-relays.md), [Trackers](mws-trackers.md)
- [MiyuWebwayParticipant](../tools/miyuwebwayparticipant.md), [MiyuWebwayTracker](../tools/miyuwebwaytracker.md)
