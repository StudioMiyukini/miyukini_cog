# MiyuWebwayTracker

## Toolkit de Tracking et Découverte Webway

**MiyuWebwayTracker** est le toolkit qui permet à un COG de fonctionner en tant que Tracker dans le réseau Miyukini Webway System (MWS).

## Fonction

> MiyuWebwayTracker **indexe** et **facilite** la découverte des COGs.

Ce toolkit transforme un COG en Tracker, lui permettant de gérer un pool de COGs participants et de faciliter leur découverte mutuelle.

## Capacités

### Gestion du Pool

| Capacité | Description |
|----------|-------------|
| Enregistrement | Accepter de nouveaux COGs |
| Désenregistrement | Retirer des COGs |
| Heartbeat | Vérifier la présence |
| Nettoyage | Retirer les COGs inactifs |

### Découverte

| Capacité | Description |
|----------|-------------|
| Index | Maintenir la liste des COGs |
| Recherche | Répondre aux requêtes |
| Filtrage | Par type, version, statut |
| Routing | Orienter vers le bon COG |

### Vérification

| Capacité | Description |
|----------|-------------|
| Permis | Émettre/valider des Permis |
| Identité | Vérifier les identités COG |
| Conformité | Contrôler les versions |

## Architecture

```
┌─────────────────────────────────────────────────┐
│              MIYUWEBWAYTRACKER                  │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │           Tracker Engine                  │   │
│  └──────────────────────────────────────────┘   │
│       │           │           │           │     │
│       ▼           ▼           ▼           ▼     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ │
│  │  Pool  │ │ Discovery│ │ Permit │ │ Health │ │
│  │Manager │ │  Index   │ │ Issuer │ │ Check  │ │
│  └────────┘ └──────────┘ └────────┘ └────────┘ │
└─────────────────────────────────────────────────┘
```

## Rôle dans le Webway

```
                ORIGIN
                   │
                   ▼
              ┌─────────┐
              │ RELAYS  │
              └────┬────┘
                   │
        ┌──────────┼──────────┐
        │          │          │
        ▼          ▼          ▼
   ┌─────────┐ ┌─────────┐ ┌─────────┐
   │TRACKER 1│ │TRACKER 2│ │TRACKER 3│  ◄── Ce toolkit
   └────┬────┘ └────┬────┘ └────┬────┘
        │          │          │
    ┌───┴───┐  ┌───┴───┐  ┌───┴───┐
    │COG A  │  │COG D  │  │COG G  │
    │COG B  │  │COG E  │  │COG H  │
    │COG C  │  │COG F  │  │COG I  │
    └───────┘  └───────┘  └───────┘
```

## Flux de Gestion

### Enregistrement d'un COG

```
COG veut s'enregistrer
        │
        ▼
┌─────────────────┐
│ Réception       │──► Demande d'enregistrement
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Vérification    │──► Identité valide ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Conformité      │──► Version compatible ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Émission Permis │──► Permis de circulation
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Ajout au Pool   │──► Index mis à jour
└────────┬────────┘
         │
         ▼
    COG enregistré
```

### Recherche de COG

```
Requête de découverte
        │
        ▼
┌─────────────────┐
│ Validation      │──► Requête valide ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Recherche       │──► Dans l'index local
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Filtrage        │──► Selon critères
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Résultats       │──► Liste des COGs correspondants
└─────────────────┘
```

## Pool de COGs

### Structure

```json
{
  "pool": [
    {
      "cog_id": "abc123...",
      "type": "STABLE",
      "version": "1.2.0",
      "last_seen": "2024-01-15T10:30:00Z",
      "status": "ACTIVE",
      "endpoints": ["https://cog-abc.local:8443"]
    },
    {
      "cog_id": "def456...",
      "type": "TERMINAL",
      "version": "1.2.0",
      "last_seen": "2024-01-15T10:29:00Z",
      "status": "ACTIVE",
      "endpoints": []
    }
  ]
}
```

### États des COGs

| État | Description |
|------|-------------|
| PENDING | En attente de validation |
| ACTIVE | Participant actif |
| INACTIVE | Pas de heartbeat récent |
| SUSPENDED | Suspendu temporairement |
| BLACKLISTED | Banni du pool |

## API (via BondingBrother)

### Intentions Supportées

| Intention | Paramètres | Résultat |
|-----------|------------|----------|
| `TRACKER_REGISTER` | cog_info | {permit, pool_id} |
| `TRACKER_UNREGISTER` | cog_id | {removed} |
| `TRACKER_SEARCH` | query | {cogs[]} |
| `TRACKER_HEARTBEAT` | cog_id | {acknowledged} |
| `TRACKER_STATUS` | - | {pool_size, active} |

## Permis de Circulation

### Émission

Le Tracker émet des Permis de circulation :

```json
{
  "permit_id": "permit-789...",
  "cog_id": "abc123...",
  "tracker_id": "tracker-001",
  "issued": "2024-01-15T10:00:00Z",
  "expires": "2024-01-16T10:00:00Z",
  "scope": "STANDARD",
  "signature": "tracker-signature..."
}
```

### Validation

Le Tracker valide les Permis :
- Signature correcte
- Non expiré
- COG toujours actif
- Pas blacklisté

## Contrats

### Contrat de Frontière

MiyuWebwayTracker **peut** :
- Gérer un pool de COGs
- Émettre des Permis
- Répondre aux découvertes

MiyuWebwayTracker **ne peut pas** :
- Forcer des connexions
- Accéder aux données des COGs
- Agir comme Relay ou Origin

### Contrat de Gouvernance

MiyuWebwayTracker respecte :
- LOI-6 : Fédération facilitée mais non forcée
- Neutralité : Pas de favoritisme
- Transparence : Règles publiques

## Sécurité

### Authentification

- Vérification identité COG
- Signatures cryptographiques
- Rotation des Permis

### Protection du Pool

- Rate limiting des requêtes
- Détection d'abus
- Blacklist automatique

### Communication

- TLS obligatoire
- Chiffrement des échanges
- Audit des opérations

## Cas d'Usage

### Démarrer un Tracker

```
COG veut devenir Tracker
        │
        ▼
MiyuWebwayTracker :
  1. Initialise le pool vide
  2. Configure les règles d'admission
  3. Démarre l'écoute
  4. S'annonce aux Relays
        │
        ▼
Tracker opérationnel
```

### Découverte de Services

```
COG A cherche un COG avec JayKonta
        │
        ▼
Requête au Tracker :
  {type: "STABLE", services: ["JayKonta"]}
        │
        ▼
MiyuWebwayTracker :
  1. Recherche dans le pool
  2. Filtre par services déclarés
  3. Vérifie disponibilité
        │
        ▼
Résultat : [COG B, COG E]
```

## Monitoring

Métriques disponibles :
- Taille du pool
- COGs actifs/inactifs
- Requêtes de découverte
- Permis émis
- Blacklists actifs

## Haute Disponibilité

Un Tracker peut :
- Se synchroniser avec d'autres Trackers
- Partager son pool
- Assurer la continuité de service
