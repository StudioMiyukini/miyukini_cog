# LogisticsSteward

## Core de Gestion des Ressources

**LogisticsSteward** est le Core responsable de la gestion des ressources système. Il optimise l'allocation et garantit un fonctionnement efficace sur tout type de hardware.

## Rôle Principal

> LogisticsSteward **alloue** et **optimise** les ressources sans gaspillage.

LogisticsSteward est l'intendant du COG. Il veille à ce que chaque composant dispose des ressources nécessaires, tout en respectant les contraintes du matériel (LOI-5).

## Responsabilités

### Gestion des Ressources

| Fonction | Description |
|----------|-------------|
| Allocation | Distribution des ressources |
| Monitoring | Suivi de la consommation |
| Optimisation | Répartition efficace |
| Libération | Récupération des ressources inutilisées |

### Adaptation Hardware

| Fonction | Description |
|----------|-------------|
| Détection | Identification des capacités |
| Scaling | Adaptation aux ressources disponibles |
| Dégradation gracieuse | Fonctionnement réduit si nécessaire |
| Recovery | Récupération après surcharge |

## Architecture

```
┌─────────────────────────────────────────────────┐
│             LOGISTICSSTEWARD                     │
│  ┌───────────────────────────────────────────┐  │
│  │           Resource Manager                 │  │
│  └───────────────────────────────────────────┘  │
│       │           │           │           │     │
│       ▼           ▼           ▼           ▼     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ │
│  │Memory  │ │  CPU     │ │Storage │ │Network │ │
│  │Manager │ │ Scheduler│ │ Quota  │ │ Quota  │ │
│  └────────┘ └──────────┘ └────────┘ └────────┘ │
└─────────────────────────────────────────────────┘
```

## Types de Ressources Gérées

| Ressource | Description |
|-----------|-------------|
| **Mémoire** | RAM disponible et utilisée |
| **CPU** | Temps processeur |
| **Stockage** | Espace disque |
| **Réseau** | Bande passante |
| **Threads** | Exécution parallèle |
| **Handles** | Fichiers, connexions |

## Flux d'Allocation

```
Demande de ressource
        │
        ▼
┌─────────────────┐
│ Ressource       │
│ disponible ?    │──► Non ──► File d'attente / Refus
└────────┬────────┘
         │ Oui
         ▼
┌─────────────────┐
│ Priorité        │──► Calcul selon demandeur
│ évaluée         │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Allocation      │──► Attribution + Monitoring
└────────┬────────┘
         │
         ▼
    Ressource fournie
```

## Interactions avec les Autres Cores

```
StrongFather ──► Coordination
        │
        ▼
┌──────────────────┐
│LogisticsSteward  │
└──────┬───────────┘
       │
       ├──► KindMother : "Quota stockage ?"
       │
       ├──► WorrySentinel : "Signaler surcharge"
       │
       └──► Tous : "Ressources allouées"
```

## Adaptation au Hardware (LOI-5)

### Détection Automatique

LogisticsSteward détecte :
- Quantité de RAM
- Nombre de cœurs CPU
- Espace disque disponible
- Vitesse réseau

### Profils d'Adaptation

| Profil | RAM | CPU | Caractéristiques |
|--------|-----|-----|------------------|
| **Minimal** | < 2 Go | 1-2 cœurs | Fonctions essentielles uniquement |
| **Standard** | 2-8 Go | 2-4 cœurs | Fonctionnement normal |
| **Confort** | 8-16 Go | 4-8 cœurs | Toutes fonctionnalités |
| **Performance** | > 16 Go | > 8 cœurs | Optimisations activées |

### Dégradation Gracieuse

```
Ressources suffisantes
        │
        │ Baisse
        ▼
┌─────────────────┐
│ Désactivation   │──► Fonctions non-essentielles
│ progressive     │
└────────┬────────┘
         │ Baisse
         ▼
┌─────────────────┐
│ Mode économique │──► Cache réduit, UI simplifiée
└────────┬────────┘
         │ Baisse
         ▼
┌─────────────────┐
│ Mode minimal    │──► Fonctions critiques uniquement
└─────────────────┘
```

## Quotas et Limites

### Quotas par Composant

| Composant | Quota Mémoire | Quota CPU |
|-----------|---------------|-----------|
| Cores | 20% | 10% |
| Outils | 40% | 50% |
| Opérateurs | 30% | 30% |
| Système | 10% | 10% |

### Limites de Sécurité

| Limite | Valeur | Action si dépassée |
|--------|--------|-------------------|
| Mémoire max | 80% RAM | Libération forcée |
| CPU max | 90% | Throttling |
| Stockage max | 90% disque | Alerte + blocage |

## Principes de Gouvernance

### Principe de Proportionnalité (LOI-5)

LogisticsSteward garantit :
- Coût proportionnel au hardware
- Fonctionnement sur machines modestes
- Pas de gaspillage de ressources

### Principe d'Équité

LogisticsSteward assure :
- Répartition équitable entre composants
- Pas de monopolisation
- Priorité aux fonctions critiques

## États de Fonctionnement

| État | Description |
|------|-------------|
| **OPTIMAL** | Ressources abondantes |
| **NORMAL** | Fonctionnement standard |
| **TENDU** | Ressources limitées |
| **CRITIQUE** | Pénurie de ressources |
| **RECOVERY** | Récupération en cours |

## Invariants

| Invariant | Description |
|-----------|-------------|
| Non-gaspillage | Ressources libérées si inutilisées |
| Équité | Pas de famine de ressources |
| Résilience | Fonctionnement même limité |
| Traçabilité | Consommation suivie |

## Contrats

### Contrat d'Allocation

LogisticsSteward garantit :
- ✓ Réponse en temps borné
- ✓ Allocation ou refus explicite
- ✓ Libération automatique
- ✓ Pas de fuite de ressources

### Contrat d'Adaptation

LogisticsSteward garantit :
- ✓ Détection du hardware
- ✓ Adaptation automatique
- ✓ Dégradation gracieuse
- ✓ Récupération après surcharge

## Cas d'Usage

### Exemple : Allocation pour Opérateur

```
JayKonta : "J'ai besoin de 200 Mo pour un rapport"
                │
                ▼
LogisticsSteward évalue :
- RAM disponible : 4 Go
- Déjà alloué : 2 Go
- Reste : 2 Go
- Demande : 200 Mo ✓
                │
                ▼
Allocation accordée :
- Quota temporaire : 200 Mo
- Durée : 5 minutes max
- Monitoring activé
```

### Exemple : Surcharge Système

```
Situation détectée :
- RAM utilisée : 95%
- CPU : 100%
- Alerte WorrySentinel reçue
                │
                ▼
LogisticsSteward réagit :
1. Identification des gros consommateurs
2. Libération caches non-critiques
3. Throttling des processus secondaires
4. Passage en mode économique
                │
                ▼
Résultat :
- RAM : 70%
- CPU : 60%
- Fonctionnement rétabli
```

## Monitoring

LogisticsSteward fournit des métriques :

| Métrique | Description |
|----------|-------------|
| `mem_used` | Mémoire utilisée |
| `mem_available` | Mémoire disponible |
| `cpu_load` | Charge CPU |
| `disk_used` | Stockage utilisé |
| `allocation_queue` | File d'attente |

## Sécurité

- Isolation des ressources par composant
- Pas d'accès direct aux ressources système
- Quotas stricts respectés
- Audit de la consommation
