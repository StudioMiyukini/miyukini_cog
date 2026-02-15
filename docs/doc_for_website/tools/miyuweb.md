# MiyuWeb

## Toolkit de Communication Web

**MiyuWeb** est le toolkit responsable de toutes les communications HTTP/HTTPS dans Miyukini. Il gère les appels réseau de manière sécurisée et contrôlée.

## Fonction

> MiyuWeb **communique** avec l'extérieur de manière contrôlée.

MiyuWeb est l'outil utilisé pour toute communication web, toujours sous le contrôle de BorderGuard.

## Capacités

### Client HTTP

| Capacité | Description |
|----------|-------------|
| GET | Récupération de ressources |
| POST | Envoi de données |
| PUT/PATCH | Modification |
| DELETE | Suppression |
| Headers | Gestion complète |

### Serveur HTTP

| Capacité | Description |
|----------|-------------|
| Écoute | Réception de requêtes |
| Routage | Distribution des requêtes |
| Réponses | Envoi de données |
| TLS | Chiffrement HTTPS |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                    MIYUWEB                       │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │           Communication Engine            │   │
│  └──────────────────────────────────────────┘   │
│       │                       │                 │
│       ▼                       ▼                 │
│  ┌────────────────┐    ┌────────────────┐      │
│  │  HTTP Client   │    │  HTTP Server   │      │
│  └────────────────┘    └────────────────┘      │
│       │                       │                 │
│       └───────────┬───────────┘                 │
│                   ▼                             │
│  ┌──────────────────────────────────────────┐   │
│  │              TLS Engine                   │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

## Flux de Communication

### Requête Sortante

```
Instruction (via BondingBrother)
        │
        ▼
┌─────────────────┐
│ BorderGuard     │──► Destination autorisée ?
└────────┬────────┘
         │ OK
         ▼
┌─────────────────┐
│ Préparation     │──► Headers, body, TLS
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Envoi           │──► Réseau
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Réception       │──► Réponse
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Validation      │──► BorderGuard vérifie
└────────┬────────┘
         │
         ▼
    Résultat
```

### Requête Entrante

```
Requête externe
        │
        ▼
┌─────────────────┐
│ BorderGuard     │──► Source autorisée ?
└────────┬────────┘
         │ OK
         ▼
┌─────────────────┐
│ TLS décrypt     │──► Si HTTPS
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Routage         │──► Quel handler ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Traitement      │──► Logique métier (via Cores)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Réponse         │──► Envoi au client
└─────────────────┘
```

## API (via BondingBrother)

### Intentions Supportées

| Intention | Paramètres | Résultat |
|-----------|------------|----------|
| `HTTP_GET` | url, headers | {status, body, headers} |
| `HTTP_POST` | url, body, headers | {status, body, headers} |
| `HTTP_PUT` | url, body, headers | {status, body, headers} |
| `HTTP_DELETE` | url, headers | {status, body} |
| `SERVE` | port, routes | {server_id} |
| `STOP_SERVER` | server_id | {stopped} |

## Sécurité

### TLS/HTTPS

| Aspect | Configuration |
|--------|---------------|
| Version | TLS 1.3 minimum |
| Certificats | Vérification stricte |
| Ciphers | Suite sécurisée uniquement |
| HSTS | Activé par défaut |

### Contrôles BorderGuard

Avant chaque communication :
- ✓ Destination dans whitelist ?
- ✓ Protocole autorisé ?
- ✓ Volume acceptable ?
- ✓ Contenu valide ?

### Headers de Sécurité

```http
Content-Security-Policy: default-src 'self'
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
Strict-Transport-Security: max-age=31536000
```

## Timeouts et Limites

| Paramètre | Valeur |
|-----------|--------|
| Connexion timeout | 10 secondes |
| Read timeout | 30 secondes |
| Max body size | 10 Mo |
| Max connections | 100 |

## Contrats

### Contrat de Frontière

MiyuWeb **peut** :
- Envoyer des requêtes HTTP/HTTPS
- Recevoir des requêtes
- Gérer TLS

MiyuWeb **ne peut pas** :
- Contourner BorderGuard
- Accéder aux données locales
- Communiquer sans autorisation

### Contrat de Gouvernance

MiyuWeb respecte :
- LOI-1 : Optionnel (offline-first)
- LOI-2 : Fonctionne en isolation
- Contrôle BorderGuard obligatoire

## Cas d'Usage

### Appel API Externe

```
JayKonta : "Récupérer taux de change"
        │
        ▼
BondingBrother ──► MiyuWeb
        │
        ▼
MiyuWeb :
  1. Vérifie avec BorderGuard
  2. Prépare requête GET
  3. Établit connexion TLS
  4. Envoie requête
  5. Reçoit réponse
  6. Valide avec BorderGuard
        │
        ▼
{status: 200, body: {EUR: 1.0, USD: 1.08}}
```

### Serveur Local

```
Miyukini Central : "Démarrer serveur web local"
        │
        ▼
MiyuWeb :
  1. Configure port (ex: 8080)
  2. Charge certificat TLS local
  3. Démarre écoute
  4. Route les requêtes
        │
        ▼
Serveur actif sur https://localhost:8080
```

## Intégration Webway

MiyuWeb est utilisé comme transport bas niveau pour le Webway :
- Communication entre COGs
- Protocole Relay
- Découverte via Trackers

Mais la logique Webway est dans les toolkits dédiés (MiyuWebwayParticipant, MiyuWebwayTracker).

## Monitoring

Métriques disponibles :
- Requêtes envoyées/reçues
- Temps de réponse
- Erreurs réseau
- Bande passante utilisée
