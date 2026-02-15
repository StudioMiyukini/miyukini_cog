# TAMR

## Core de Gestion des Accès et Permissions

**TAMR** (Trust, Access, Management, Rights) est le Core responsable de toutes les questions d'autorisation. Il décide qui peut faire quoi, et dans quel contexte.

## Rôle Principal

> TAMR **autorise** ou **refuse**, jamais n'exécute.

TAMR est le gardien des droits. Chaque action dans le système passe par une validation TAMR avant d'être autorisée.

## Responsabilités

### Gestion des Accès

| Fonction | Description |
|----------|-------------|
| Authentification | Vérification de l'identité |
| Autorisation | Validation des permissions |
| Sessions | Gestion des contextes actifs |
| Délégation | Transmission contrôlée de droits |

### Gestion des Droits

| Fonction | Description |
|----------|-------------|
| Rôles | Définition des groupes de permissions |
| Permissions | Droits granulaires |
| Politiques | Règles d'accès |
| Audit | Traçabilité des décisions |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                     TAMR                         │
│  ┌───────────────────────────────────────────┐  │
│  │           Access Control Engine            │  │
│  └───────────────────────────────────────────┘  │
│       │           │           │           │     │
│       ▼           ▼           ▼           ▼     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ │
│  │Identity│ │Permission│ │ Policy │ │ Audit  │ │
│  │Manager │ │  Engine  │ │ Engine │ │ Logger │ │
│  └────────┘ └──────────┘ └────────┘ └────────┘ │
└─────────────────────────────────────────────────┘
```

## Modèle de Permissions

### Structure Hiérarchique

```
Utilisateur
    │
    ├── Rôle(s)
    │       │
    │       ├── Permission(s)
    │       │       │
    │       │       └── Action(s) sur Ressource(s)
    │       │
    │       └── Politique(s)
    │               │
    │               └── Contraintes contextuelles
    │
    └── Permissions directes (exceptions)
```

### Exemple de Hiérarchie

```
Alice (Utilisateur)
    │
    ├── Rôle : "Comptable"
    │       │
    │       ├── Permission : "lire_factures"
    │       ├── Permission : "créer_factures"
    │       └── Permission : "modifier_factures"
    │
    └── Rôle : "Utilisateur_Standard"
            │
            ├── Permission : "voir_dashboard"
            └── Permission : "modifier_profil"
```

## Flux de Validation

```
Demande d'action
        │
        ▼
┌─────────────────┐
│   Identification│──► Qui demande ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Intention     │──► Quelle action ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Ressource     │──► Sur quoi ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Contexte      │──► Dans quelles conditions ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Décision      │──► AUTORISÉ / REFUSÉ
└────────┬────────┘
         │
         ▼
    Audit logué
```

## Interactions avec les Autres Cores

```
StrongFather ──► "Alice peut-elle modifier X ?"
        │
        ▼
┌──────────────┐
│     TAMR     │
└──────┬───────┘
       │
       ├──► KindMother : "Charger les permissions d'Alice"
       │
       ├──► BorderGuard : "Contexte de sécurité ?"
       │
       └──► WorrySentinel : "Alerter si anomalie"
```

## Niveaux de Confiance

| Niveau | Nom | Description |
|--------|-----|-------------|
| **T0** | Normal | Confiance établie, accès standard |
| **T1** | Instable | Comportement inhabituel détecté |
| **T2** | Dégradé | Accès réduit par précaution |
| **T3** | Restreint | Accès minimal uniquement |
| **T4** | Bloqué | Aucun accès autorisé |

## Politiques d'Accès

### Types de Politiques

| Type | Description |
|------|-------------|
| **Temporelle** | Accès limité dans le temps |
| **Contextuelle** | Accès selon le contexte (lieu, appareil) |
| **Ressource** | Accès limité à certaines ressources |
| **Action** | Accès limité à certaines actions |
| **Cumulative** | Combinaison de plusieurs contraintes |

### Exemple de Politique

```yaml
politique: "acces_comptabilite_heures_bureau"
  conditions:
    - heure: 09:00 - 18:00
    - jours: lundi - vendredi
    - role: comptable
  permissions:
    - lire_factures
    - créer_factures
  restrictions:
    - pas_de_suppression
```

## Principes de Gouvernance

### Principe du Moindre Privilège

TAMR applique toujours :
- Permissions minimales par défaut
- Escalade explicite requise
- Durée limitée pour les privilèges élevés

### Principe de Défaut Sécurisé

En cas de doute, TAMR **refuse** :
- Pas de "fail open"
- Erreur = refus
- Ambiguïté = refus

## États de Fonctionnement

| État | Description |
|------|-------------|
| **READY** | Prêt à valider |
| **VALIDATING** | Validation en cours |
| **LOCKED** | Validation temporairement suspendue |
| **EMERGENCY** | Mode d'urgence (restrictions maximales) |

## Invariants

| Invariant | Description |
|-----------|-------------|
| Complétude | Toute action est validée |
| Traçabilité | Toute décision est auditée |
| Cohérence | Pas de contradiction dans les droits |
| Défaut sécurisé | Refus en cas de doute |

## Contrats

### Contrat d'Accès

TAMR garantit :
- ✓ Validation systématique
- ✓ Décision en temps borné
- ✓ Cohérence des permissions
- ✓ Audit complet

### Contrat d'Audit

TAMR garantit :
- ✓ Log de chaque décision
- ✓ Contexte complet enregistré
- ✓ Non-répudiation
- ✓ Accès contrôlé aux logs

## Cas d'Usage

### Exemple : Accès à un Document Sensible

```
Bob : "Voir le document confidentiel X"
                │
                ▼
TAMR reçoit la demande
                │
    ┌───────────┴───────────┐
    ▼                       ▼
Identification          Chargement droits
(Bob confirmé)          (rôle : manager)
    │                       │
    └───────────┬───────────┘
                ▼
    Vérification politique :
    - Document niveau 3 (critique)
    - Bob a permission niveau 2 max
                │
                ▼
    DÉCISION : REFUSÉ
                │
                ▼
    Audit logué + Alerte WorrySentinel
```

## Interventions

TAMR peut être sollicité pour des interventions :

| Type | Description |
|------|-------------|
| **Révocation** | Retrait immédiat de droits |
| **Escalade** | Attribution temporaire de privilèges |
| **Reset** | Réinitialisation des sessions |
| **Blocage** | Interdiction totale d'un acteur |

## Sécurité

- Aucun bypass possible
- Double validation pour actions critiques
- Rate limiting sur les tentatives
- Isolation des contextes d'évaluation
