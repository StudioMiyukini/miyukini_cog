# Kernel

## Strate K : Le Substrat Technique

Le **Kernel** est la fondation technique de Miyukini. Situé à la Strate K (entre la Strate 0/Hardware et la Strate 3/Invariants), il fournit les primitives de base nécessaires au fonctionnement du système.

## Principe Fondamental

> Le Kernel ne contient **aucune logique métier**.

Le Kernel est **neutre**. Il fournit des outils techniques sans jamais prendre de décision fonctionnelle. Toute intelligence est dans les Cores (Strate 4).

## Responsabilités

### Ce que le Kernel FAIT

| Fonction | Description |
|----------|-------------|
| Bootstrap | Démarrage initial du système |
| Génération d'ID | Création d'identifiants uniques |
| Primitives système | Accès fichiers, réseau bas niveau |
| Gestion mémoire | Allocation et libération |
| Traits API | Interfaces pour les Cores |

### Ce que le Kernel NE FAIT PAS

| Interdit | Raison |
|----------|--------|
| Décisions métier | Réservé aux Cores |
| Validation fonctionnelle | Réservé aux Outils |
| Logique applicative | Réservé aux Opérateurs |
| Stockage structuré | Réservé à KindMother |

## Architecture

```
┌─────────────────────────────────────────┐
│              STRATE 4                   │
│              (Cores)                    │
└───────────────────┬─────────────────────┘
                    │ utilise
                    ▼
┌─────────────────────────────────────────┐
│              STRATE K                   │
│              (Kernel)                   │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │Bootstrap│ │ ID Gen  │ │ Traits  │   │
│  └─────────┘ └─────────┘ └─────────┘   │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │ Memory  │ │  I/O    │ │ Network │   │
│  └─────────┘ └─────────┘ └─────────┘   │
└───────────────────┬─────────────────────┘
                    │ s'appuie sur
                    ▼
┌─────────────────────────────────────────┐
│              STRATE 0                   │
│           (Hardware & OS)               │
└─────────────────────────────────────────┘
```

## Composants du Kernel

### Bootstrap

Le processus de démarrage :

1. **Initialisation** — Vérification de l'intégrité
2. **Chargement** — Mise en place des primitives
3. **Éveil des Cores** — Activation de la gouvernance
4. **Prêt** — Système opérationnel

### Génération d'Identifiants

Le Kernel génère des identifiants uniques :
- **COG ID** — Identité de l'environnement
- **Session ID** — Identité de session
- **Transaction ID** — Traçabilité des opérations

Propriétés garanties :
- Unicité globale (sans coordination centrale)
- Non-prédictibilité
- Compacité

### Traits API

Le Kernel définit les interfaces (traits Rust) que les Cores implémentent :

```rust
// Exemple simplifié de trait Kernel
pub trait KernelPrimitive {
    fn initialize(&self) -> Result<(), KernelError>;
    fn shutdown(&self) -> Result<(), KernelError>;
}
```

Ces traits sont **neutres** — ils définissent des capacités, pas des comportements.

## Invariants du Kernel

| Invariant | Description |
|-----------|-------------|
| Neutralité | Aucune logique métier |
| Minimalisme | Seul le strict nécessaire |
| Stabilité | API figée entre versions |
| Sécurité | `unsafe_code = "forbid"` |

## Relation avec les Cores

Le Kernel **sert** les Cores sans les **influencer** :

```
Cores (gouvernance) ──────► Décisions
         │
         │ demandent
         ▼
Kernel (technique) ────────► Primitives
         │
         │ utilise
         ▼
OS (hardware) ─────────────► Ressources
```

Les Cores utilisent le Kernel pour :
- Obtenir des identifiants
- Accéder aux ressources système
- Interagir avec le hardware

Le Kernel ne dit **jamais** aux Cores quoi faire.

## Versionnement

Le Kernel suit un versionnement strict :

| Version | Signification |
|---------|---------------|
| Majeure | Changement d'API incompatible |
| Mineure | Nouvelles primitives rétro-compatibles |
| Patch | Corrections sans changement d'API |

Un COG est lié à une version **spécifique** du Kernel, qui ne change jamais (LOI-7).

## Sécurité

### Code Sûr

```toml
# Dans Cargo.toml
[lints.rust]
unsafe_code = "forbid"
```

Le Kernel interdit tout code `unsafe` Rust.

### Surface Minimale

Principe de moindre privilège :
- Seules les primitives nécessaires sont exposées
- Pas de fonctionnalités "au cas où"
- Chaque primitive justifiée et documentée

## Tests

Le Kernel fait l'objet de tests exhaustifs :

- **Tests unitaires** — Chaque primitive isolément
- **Tests d'intégration** — Interactions entre primitives
- **Tests de stabilité** — Comportement sur la durée
- **Tests de charge** — Performance sous stress

## Crate

```
crates/
└── miyukini-kernel/
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── bootstrap.rs
    │   ├── id.rs
    │   └── traits.rs
    └── tests/
```
