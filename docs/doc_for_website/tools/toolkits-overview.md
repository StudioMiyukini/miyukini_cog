# Vue d'Ensemble des Toolkits

## Strate 6 : Outils & Kits d'Outils

Les **Toolkits** sont les capacités exécutables du système Miyukini. Situés à la Strate 6, ils reçoivent des instructions de BondingBrother et les exécutent.

## Principe Fondamental

> Les Outils **font**, mais ne **décident** jamais.

Un Outil exécute fidèlement ce qu'on lui demande. Il n'a aucune autonomie décisionnelle — toute décision vient des Cores via BondingBrother.

## Catalogue des Toolkits

### Authentification & Sécurité

| Toolkit | Fonction |
|---------|----------|
| **MiyuAuth** | Authentification et sessions |
| **MiyuValidate** | Validation des données |

### Persistance & Données

| Toolkit | Fonction |
|---------|----------|
| **MiyuSQL** | Accès base de données SQLite |
| **MiyuStore** | Stockage de fichiers |

### Communication

| Toolkit | Fonction |
|---------|----------|
| **MiyuWeb** | Communication HTTP/HTTPS |
| **MiyuWebwayParticipant** | Participation au réseau Webway |
| **MiyuWebwayTracker** | Tracking et découverte Webway |

### Interface Utilisateur

| Toolkit | Fonction |
|---------|----------|
| **MiyuWidgets** | Composants UI |
| **MiyuText** | Manipulation de texte |

### Métier

| Toolkit | Fonction |
|---------|----------|
| **MiyuTreasury** | Gestion financière |
| **MiyuStory** | Gestion de contenu narratif |
| **MiyuClock** | Gestion du temps |

### Social

| Toolkit | Fonction |
|---------|----------|
| **MiyuSocialProfile** | Profils utilisateurs |
| **MiyuSocialModeration** | Modération de contenu |

## Architecture Standard d'un Toolkit

Chaque Toolkit suit une structure normalisée :

```
miyu-<nom>/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Point d'entrée
│   ├── admin_cell.rs   # Interface admin
│   ├── context.rs      # Contexte d'exécution
│   └── errors.rs       # Gestion des erreurs
├── contracts/
│   ├── boundaries/     # Limites d'utilisation
│   ├── governance/     # Règles de gouvernance
│   ├── integration/    # Contrats d'intégration
│   └── testing/        # Spécifications de test
└── tests/
```

## Flux d'Exécution

```
BondingBrother
      │
      │ Instruction
      ▼
┌─────────────────────┐
│      TOOLKIT        │
│                     │
│  ┌───────────────┐  │
│  │ Réception     │  │
│  └───────┬───────┘  │
│          │          │
│          ▼          │
│  ┌───────────────┐  │
│  │ Validation    │  │
│  └───────┬───────┘  │
│          │          │
│          ▼          │
│  ┌───────────────┐  │
│  │ Exécution     │  │
│  └───────┬───────┘  │
│          │          │
│          ▼          │
│  ┌───────────────┐  │
│  │ Résultat      │  │
│  └───────────────┘  │
│                     │
└─────────────────────┘
      │
      │ Réponse
      ▼
BondingBrother
```

## Contrats des Toolkits

### Contrat de Frontière (Boundaries)

Définit les limites d'utilisation :
- Ce que le Toolkit peut faire
- Ce qu'il ne peut pas faire
- Conditions d'utilisation

### Contrat de Gouvernance

Définit les règles :
- Conformité aux Lois d'Autonomie
- Respect des invariants
- Comportement attendu

### Contrat d'Intégration

Définit les interfaces :
- API exposée
- Format des données
- Protocoles supportés

### Contrat de Test

Définit la validation :
- Tests unitaires requis
- Tests d'intégration
- Critères de couverture

## Relations entre Toolkits

Les Toolkits peuvent collaborer via BondingBrother :

```
                 BondingBrother
                       │
        ┌──────────────┼──────────────┐
        │              │              │
        ▼              ▼              ▼
   ┌─────────┐   ┌─────────┐   ┌─────────┐
   │MiyuAuth │   │MiyuSQL  │   │MiyuWeb  │
   └─────────┘   └─────────┘   └─────────┘
        │              │              │
        └──────────────┼──────────────┘
                       │
                  Coordination
```

**Important** : Les Toolkits ne communiquent **jamais** directement entre eux. Toute coordination passe par BondingBrother.

## Principes de Développement

### Principe de Responsabilité Unique

Chaque Toolkit a **une seule** responsabilité :
- MiyuAuth = authentification
- MiyuSQL = persistance
- etc.

### Principe de Non-Autonomie

Un Toolkit :
- ✗ Ne prend pas de décision
- ✗ Ne stocke pas d'état global
- ✗ N'appelle pas d'autres Toolkits
- ✓ Exécute ce qu'on lui demande

### Principe de Déterminisme

Même entrée = même sortie :
- Comportement prédictible
- Pas d'aléatoire caché
- Résultat reproductible

## Sécurité des Toolkits

### Code Sûr

```toml
[lints.rust]
unsafe_code = "forbid"
```

Aucun code `unsafe` n'est autorisé.

### Isolation

Chaque Toolkit :
- Fonctionne dans son contexte
- N'accède pas aux ressources des autres
- Est limité par ses contrats

### Audit

Toutes les actions des Toolkits sont :
- Loguées
- Traçables
- Auditables

## Liste Complète des 49 Toolkits

| # | Nom | Domaine |
|---|-----|---------|
| 1 | MiyuAuth | Authentification |
| 2 | MiyuSQL | Base de données |
| 3 | MiyuStore | Stockage fichiers |
| 4 | MiyuWeb | Communication web |
| 5 | MiyuWebwayParticipant | Réseau Webway |
| 6 | MiyuWebwayTracker | Tracking Webway |
| 7 | MiyuWidgets | Interface UI |
| 8 | MiyuText | Texte |
| 9 | MiyuValidate | Validation |
| 10 | MiyuTreasury | Finance |
| 11 | MiyuStory | Contenu |
| 12 | MiyuClock | Temps |
| 13 | MiyuSocialProfile | Profils |
| 14 | MiyuSocialModeration | Modération |
| ... | ... | ... |

*La liste complète des 49 toolkits est disponible dans la documentation de référence.*

## Utilisation

Un Toolkit n'est jamais appelé directement par :
- Un utilisateur
- Un opérateur
- Un autre Toolkit

Seul **BondingBrother** peut invoquer un Toolkit, sur instruction d'un **Core**.

```
Utilisateur ──► Service ──► Opérateur ──► Core ──► BondingBrother ──► Toolkit
```
