# Pyramide Miyukini — Detail des strates

## Strate 0 — Hardware & OS
Realite physique. Hors controle de Miyukini.

## Strate K — Kernel
Substrat technique neutre. Fondation reutilisable et agnostique.

| Composant | Crate | Role |
|-----------|-------|------|
| Id | `miyukini-kernel` | Generation d'identifiants uniques |
| Logger | `miyukini-kernel` | Logging structure |
| Clock | `miyukini-kernel` | Horloge locale (trace only) |
| Config | `miyukini-kernel` | Configuration locale |
| Lifecycle | `miyukini-kernel` | Gestion du cycle de vie |

**Invariants :** Aucune logique metier. Aucune dependance externe critique. Pas de protocole applicatif.

## Strate 3 — Invariants & Contrats
Principes architecturaux. Definis dans le code des Cores.

## Strate 4 — Cores Systeme
8 Cores de gouvernance avec autorite exclusive :

### StrongFather (`crates/strongfather/`)
Decision strategique. Emetteur des Mandats de Permission.
- Decide si une action devrait etre faite
- Valide les Contrats d'Equipe
- Ne possede jamais d'autorite d'execution

### KindMother (`crates/kindmother/`)
Donnees et persistance. Autorite absolue des donnees.
- Persistance, synchronisation, coherence
- Recoit les WriteIntents

### Caring Nanny (`crates/caringnanny/`)
Observation d'etat. Detecte, classe, propage.
- Bloque les Outils si environnement degrade
- N'observe jamais directement, ne modifie jamais

### Master Butler (`crates/masterbutler/`)
Capacites et permissions. Registre central.
- Declare quels Outils existent
- Lie Capability → Tool
- Ne les implemente ni execute

### Border Guard (`crates/borderguard/`)
Frontieres et confiance. Definition conceptuelle.
- Regles de franchissement
- Niveaux de confiance

### Ever Buddy (`crates/everbuddy/`)
Cycle de vie. Gouverne l'evolution.
- Versions, depreciation, compatibilite
- Migration (observation, pas execution)

### WorrySentinel (`crates/worrysentinel/`)
Gouvernance de securite.
- Niveau de confiance global (T0-T4)
- Niveau de securite actif (0-4)
- Mode de fonctionnement autorise

### TAMR (`crates/tamr/`)
Trust & Authority Mediation Resolver.
- Definit quand l'humain intervient

### Autres Cores
- **LogisticsSteward** (`crates/logisticssteward/`) — Orchestration des ressources
- **BondingBrother** (`crates/bondingbrother/`) — Mediation (Strate 5)

## Strate 5 — Interfaces & Adaptation
BondingBrother comme mediateur entre Operateurs et Cores.

## Strate 6 — Outils & Kits d'Outils
49 Toolkits implementes. Tous les crates `miyu*` :
- Capacites executables, sans autorite
- Gouvernes par les Cores
- Structures standardisees (voir SKILL miyukini-rust-patterns)

## Strate 7 — Operateurs
Entites fonctionnelles gouvernees :
- Services Jay : `jayfestival`, `jayxpose`
- Services Miyukini : `miyukini-central`
- Jeux : `lord_of_the_castle`, `miyuclicker`

## Strate 9 — MiyukiniAdmin
Operateur Souverain. Console d'administration (`crates/miyukini-admin/`).
Exception a la logique Operateur standard.
