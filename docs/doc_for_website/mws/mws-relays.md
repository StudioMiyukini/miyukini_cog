# MWS — Relays

## Nœuds de Relais et Vérification

Les **Relays** sont les nœuds du Webway qui **dupliquent**, **vérifient** et participent à la délivrance des **Permis de circulation**. Ils ne stockent pas les données utilisateur mais assurent la cohérence et la confiance du réseau.

## Rôle

| Fonction | Description |
|----------|-------------|
| **Relais** | Transmettre les messages entre COGs et vers l’Origin/Trackers |
| **Vérification** | Contrôler la conformité (MIP, protocole) avant délivrance de Permis |
| **Permis** | Émettre ou valider les Permis de circulation |

## Protocole Relay

Le protocole Relay est décrit dans `MWS - Protocole Relay.md` : formats de messages, étapes de vérification (Phase B), et interaction avec les Trackers.

## Relation avec les Autres Acteurs

```
Origin ──► définit les règles
    │
    ▼
Relays ──► appliquent la vérification, délivrent Permis
    │
    ▼
Trackers ──► gèrent les pools, découverte
    │
    ▼
Participants ──► utilisent les Permis pour communiquer
```

## Sécurité

- Chiffrement TLS
- Vérification des signatures
- Pas de stockage de données métier
- Registre de services et isolation documentés
