# MiyuWebwayParticipant

## Toolkit de Participation au Réseau Webway

**MiyuWebwayParticipant** est le toolkit qui permet à un COG de participer au réseau Miyukini Webway System (MWS).

## Fonction

> MiyuWebwayParticipant **connecte** le COG au réseau de fédération.

Ce toolkit gère la participation du COG au réseau Webway, incluant l'enregistrement, la communication inter-COG et la gestion des connexions.

## Capacités

### Participation Réseau

| Capacité | Description |
|----------|-------------|
| Enregistrement | Inscription auprès d'un Tracker |
| Présence | Signalement d'activité |
| Découverte | Recherche d'autres COGs |
| Déconnexion | Sortie propre du réseau |

### Communication Inter-COG

| Capacité | Description |
|----------|-------------|
| Connexion | Établissement de lien |
| Envoi | Transmission de données |
| Réception | Réception de données |
| Synchronisation | Échange d'état |

## Architecture

```
┌─────────────────────────────────────────────────┐
│            MIYUWEBWAYPARTICIPANT                │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │           Participation Engine            │   │
│  └──────────────────────────────────────────┘   │
│       │           │           │           │     │
│       ▼           ▼           ▼           ▼     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ │
│  │Register│ │ Presence │ │Connect │ │  Sync  │ │
│  │Manager │ │  Beacon  │ │ Pool   │ │ Engine │ │
│  └────────┘ └──────────┘ └────────┘ └────────┘ │
│                     │                           │
│                     ▼                           │
│  ┌──────────────────────────────────────────┐   │
│  │              MiyuWeb (transport)         │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

## Flux de Participation

### Rejoindre le Réseau

```
COG veut rejoindre le Webway
        │
        ▼
┌─────────────────┐
│ Découverte      │──► Trouver un Tracker
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Authentification│──► Présenter l'identité COG
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Enregistrement  │──► S'inscrire au Tracker
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Permis obtenu   │──► Permis de circulation
└────────┬────────┘
         │
         ▼
    COG connecté au Webway
```

### Communication avec un autre COG

```
Demande de communication
        │
        ▼
┌─────────────────┐
│ Découverte      │──► Où est le COG cible ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Vérification    │──► Permis valide ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Établissement   │──► Connexion sécurisée
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Échange         │──► Envoi/réception données
└────────┬────────┘
         │
         ▼
    Communication terminée
```

## Identité dans le Webway

### Permis de Circulation

Le Permis de circulation identifie un COG dans le réseau :

```json
{
  "cog_id": "abc123...",
  "cog_type": "STABLE",
  "version": "1.2.0",
  "tracker": "tracker.example.com",
  "issued": "2024-01-15T10:00:00Z",
  "expires": "2024-01-16T10:00:00Z",
  "signature": "..."
}
```

### Niveaux d'Identité

| Niveau | Description | Confiance |
|--------|-------------|-----------|
| LSI | Auto-déclaré | Basse |
| VID | Vérifié par tiers | Moyenne |
| WID | Témoigné par COGs | Haute |

## API (via BondingBrother)

### Intentions Supportées

| Intention | Paramètres | Résultat |
|-----------|------------|----------|
| `WEBWAY_JOIN` | tracker_url | {permit} |
| `WEBWAY_LEAVE` | - | {left} |
| `WEBWAY_DISCOVER` | query | {cogs[]} |
| `WEBWAY_CONNECT` | cog_id | {connection} |
| `WEBWAY_SEND` | connection, data | {sent} |
| `WEBWAY_RECEIVE` | connection | {data} |

## Relation avec les Autres Acteurs

```
┌─────────────────────────────────────────────┐
│                  ORIGIN                      │
│           (Source de vérité)                │
└───────────────────┬─────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────┐
│                  RELAYS                      │
│          (Distribution, vérification)        │
└───────────────────┬─────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────┐
│                 TRACKERS                     │
│          (Découverte, contrôle)             │
└───────────────────┬─────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────┐
│        MIYUWEBWAYPARTICIPANT                │
│            (Ce toolkit)                      │
│                                             │
│              MON COG                         │
└─────────────────────────────────────────────┘
```

## Contrats

### Contrat de Frontière

MiyuWebwayParticipant **peut** :
- Rejoindre/quitter le réseau
- Découvrir des COGs
- Communiquer avec des COGs

MiyuWebwayParticipant **ne peut pas** :
- Agir en tant que Tracker
- Contourner les vérifications
- Forcer des connexions

### Contrat de Gouvernance

MiyuWebwayParticipant respecte :
- LOI-2 : Participation optionnelle
- LOI-6 : Fédération non obligatoire
- Consentement explicite requis

## Sécurité

### Authentification

- Identité COG vérifiée
- Permis validés
- Signatures cryptographiques

### Communication

- TLS 1.3 obligatoire
- Chiffrement bout-à-bout
- Intégrité des messages

### Contrôle

- BorderGuard impliqué
- Destinations vérifiées
- Volume limité

## Cas d'Usage

### Rejoindre le Webway

```
Utilisateur active le Webway
        │
        ▼
MiyuWebwayParticipant :
  1. Contacte le Tracker configuré
  2. Présente l'identité du COG
  3. Obtient un Permis de circulation
  4. Démarre le beacon de présence
        │
        ▼
COG visible dans le réseau Webway
```

### Partager un Document

```
Alice (COG A) veut envoyer à Bob (COG B)
        │
        ▼
MiyuWebwayParticipant (COG A) :
  1. Découvre COG B via Tracker
  2. Établit connexion sécurisée
  3. Vérifie Permis mutuels
  4. Transmet les données
        │
        ▼
MiyuWebwayParticipant (COG B) :
  - Reçoit les données
  - Valide via BorderGuard
  - Notifie Bob
```

## États de Connexion

| État | Description |
|------|-------------|
| DISCONNECTED | Hors réseau |
| CONNECTING | Connexion en cours |
| REGISTERED | Inscrit au Tracker |
| ACTIVE | Participation active |
| SUSPENDED | Temporairement suspendu |

## Monitoring

Métriques disponibles :
- État de connexion
- Temps depuis dernier heartbeat
- COGs découverts
- Messages envoyés/reçus
