# MWS — Trackers

## Découverte et Contrôle des COGs

Les **Trackers** sont les acteurs qui **indexent** les COGs participants, gèrent des **pools** (éventuellement par version), et permettent la **découverte** des autres COGs. Ils jouent un rôle de « douanier » : contrôle et enregistrement, pas de stockage des données utilisateur.

## Rôle

| Fonction | Description |
|----------|-------------|
| **Pool** | Liste des COGs enregistrés, statut (actif, inactif, blacklisté) |
| **Découverte** | Répondre aux requêtes « qui propose ce service ? », « où est ce COG ? » |
| **Heartbeat** | Recevoir les signaux de présence des participants |
| **Permis** | Valider ou contribuer à l’émission des Permis (selon architecture) |

## Pools par Version

Les Trackers peuvent organiser les COGs par version de Cores ou de protocole, pour assurer la compatibilité lors de la découverte.

## Toolkit : MiyuWebwayTracker

Un COG peut devenir Tracker en utilisant le toolkit **MiyuWebwayTracker** : gestion du pool, émission/validation de Permis, réponses aux requêtes de découverte.

## Flux Typique

1. Un COG participant s’enregistre auprès d’un Tracker (via MiyuWebwayParticipant).
2. Le Tracker l’ajoute au pool et peut délivrer ou valider un Permis.
3. Un autre COG interroge le Tracker pour découvrir des COGs (par type, service, version).
4. Le Tracker renvoie la liste des COGs correspondants (sans exposer de données sensibles).

## Sécurité

- Vérification d’identité avant enregistrement
- Blacklist et quarantaine (voir Origin/Relays)
- Pas d’accès aux données des COGs, uniquement métadonnées de présence
