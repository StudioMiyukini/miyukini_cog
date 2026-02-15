# BorderGuard

## Core de Protection des Frontières

**BorderGuard** est le Core responsable de la protection des frontières du COG. Il contrôle tout ce qui entre et sort de l'environnement.

## Rôle Principal

> BorderGuard **protège** les frontières, mais ne **décide** pas des politiques.

BorderGuard est le douanier du COG. Il vérifie, valide et contrôle les flux aux frontières selon les règles établies par les autres Cores.

## Responsabilités

### Contrôle d'Entrée

| Fonction | Description |
|----------|-------------|
| Validation des données | Vérification de conformité |
| Sanitization | Nettoyage des entrées |
| Quarantaine | Isolation des éléments suspects |
| Filtrage | Blocage des contenus interdits |

### Contrôle de Sortie

| Fonction | Description |
|----------|-------------|
| Validation export | Vérification avant envoi |
| Marquage | Étiquetage des données sortantes |
| Limitation | Contrôle des volumes |
| Audit | Traçabilité des sorties |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                 BORDERGUARD                      │
│  ┌───────────────────────────────────────────┐  │
│  │           Frontier Control Engine          │  │
│  └───────────────────────────────────────────┘  │
│       │           │           │           │     │
│       ▼           ▼           ▼           ▼     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ │
│  │ Input  │ │  Output  │ │Quarant.│ │ Audit  │ │
│  │Validator│ │Validator │ │ Zone   │ │ Logger │ │
│  └────────┘ └──────────┘ └────────┘ └────────┘ │
└─────────────────────────────────────────────────┘
```

## Frontières du COG

```
                    EXTÉRIEUR
                        │
                        ▼
┌───────────────────────────────────────────┐
│               BORDERGUARD                  │
│  ┌─────────────────────────────────────┐  │
│  │         Point de Contrôle           │  │
│  └─────────────────────────────────────┘  │
└───────────────────────────────────────────┘
                        │
                        ▼
                    INTÉRIEUR
                      (COG)
```

### Types de Frontières

| Frontière | Description |
|-----------|-------------|
| **Réseau** | Communications entrantes/sortantes |
| **Fichiers** | Import/export de fichiers |
| **API** | Appels externes |
| **Webway** | Communication inter-COG |

## Flux de Contrôle

### Entrée de Données

```
Données externes
        │
        ▼
┌─────────────────┐
│ Format valide ? │──► Non ──► REJET
└────────┬────────┘
         │ Oui
         ▼
┌─────────────────┐
│ Contenu sûr ?   │──► Non ──► QUARANTAINE
└────────┬────────┘
         │ Oui
         ▼
┌─────────────────┐
│ Autorisé ?      │──► TAMR
└────────┬────────┘
         │ Oui
         ▼
    ACCEPTÉ
```

### Sortie de Données

```
Demande d'export
        │
        ▼
┌─────────────────┐
│ Données         │
│ exportables ?   │──► Non ──► BLOCAGE
└────────┬────────┘
         │ Oui
         ▼
┌─────────────────┐
│ Niveau sécurité │
│ compatible ?    │──► Non ──► BLOCAGE
└────────┬────────┘
         │ Oui
         ▼
┌─────────────────┐
│ Marquage &      │
│ Audit           │
└────────┬────────┘
         │
         ▼
    EXPORT AUTORISÉ
```

## Interactions avec les Autres Cores

```
StrongFather ──► Coordination générale
        │
        ▼
┌──────────────┐
│ BorderGuard  │
└──────┬───────┘
       │
       ├──► TAMR : "Permission d'entrée/sortie ?"
       │
       ├──► KindMother : "Stocker en quarantaine"
       │
       └──► WorrySentinel : "Signaler tentative suspecte"
```

## Zone de Quarantaine

Quand BorderGuard détecte un élément suspect :

1. **Isolation** — L'élément est mis en quarantaine
2. **Analyse** — Examen approfondi
3. **Décision** — Accepter, rejeter ou demander intervention
4. **Notification** — Alerte à WorrySentinel si nécessaire

```
┌─────────────────────────────────────────┐
│           ZONE DE QUARANTAINE           │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │Suspect 1│  │Suspect 2│  │Suspect 3│ │
│  └─────────┘  └─────────┘  └─────────┘ │
│                                         │
│  Status : En analyse / En attente       │
└─────────────────────────────────────────┘
```

## Règles de Filtrage

### Règles d'Entrée

| Règle | Action |
|-------|--------|
| Format invalide | Rejet |
| Taille excessive | Rejet ou troncature |
| Contenu malveillant | Quarantaine |
| Source inconnue | Vérification renforcée |

### Règles de Sortie

| Règle | Action |
|-------|--------|
| Données critiques | Blocage par défaut |
| Volume excessif | Limitation |
| Destination non autorisée | Blocage |
| Export non marqué | Marquage obligatoire |

## Principes de Gouvernance

### Principe de Défense en Profondeur

BorderGuard applique plusieurs couches de vérification :
- Validation de format
- Validation de contenu
- Validation de contexte
- Validation de permission

### Principe de Méfiance Par Défaut

Toute donnée externe est considérée comme :
- Potentiellement malveillante
- Non fiable jusqu'à preuve du contraire
- Suspecte si inhabituelle

## États de Fonctionnement

| État | Description |
|------|-------------|
| **NORMAL** | Contrôle standard |
| **VIGILANT** | Contrôles renforcés |
| **LOCKDOWN** | Frontières fermées |
| **EMERGENCY** | Mode d'urgence |

## Invariants

| Invariant | Description |
|-----------|-------------|
| Exhaustivité | Tout flux est contrôlé |
| Non-bypass | Aucun chemin de contournement |
| Traçabilité | Tout passage est logué |
| Réversibilité | Quarantaine annulable |

## Contrats

### Contrat de Contrôle

BorderGuard garantit :
- ✓ Vérification de tous les flux
- ✓ Temps de contrôle borné
- ✓ Décision claire (accepté/refusé/quarantaine)
- ✓ Audit complet

### Contrat de Protection

BorderGuard garantit :
- ✓ Blocage des menaces connues
- ✓ Quarantaine des suspects
- ✓ Alerte en cas d'anomalie
- ✓ Isolation des zones compromises

## Cas d'Usage

### Exemple : Import de Fichier

```
Utilisateur : "Importer document.pdf"
                │
                ▼
BorderGuard reçoit le fichier
                │
    ┌───────────┴───────────┐
    ▼                       ▼
Format PDF ?          Taille OK ?
(vérifié)             (< limite)
    │                       │
    └───────────┬───────────┘
                ▼
    Scan contenu :
    - Structure PDF valide
    - Pas de JavaScript malveillant
    - Pas de macros dangereuses
                │
                ▼
    TAMR : Permission d'import ?
                │
                ▼
    ACCEPTÉ ──► Transfert à KindMother
```

### Exemple : Tentative d'Exfiltration

```
Processus : "Envoyer base_clients.db vers IP externe"
                │
                ▼
BorderGuard intercepte
                │
    ┌───────────┴───────────┐
    ▼                       ▼
Données critiques ?   Destination autorisée ?
(niveau 4)            (IP inconnue)
    │                       │
    BLOCAGE                 BLOCAGE
    │                       │
    └───────────┬───────────┘
                ▼
    Alerte WorrySentinel
    Audit détaillé logué
```

## Sécurité

- Validation multi-couches
- Signatures de menaces mises à jour
- Isolation stricte de la quarantaine
- Aucune exécution de contenu non validé
