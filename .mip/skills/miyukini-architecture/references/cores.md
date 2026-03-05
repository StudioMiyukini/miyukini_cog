# Cores Miyukini — Reference detaillee

## Regles fondamentales

1. Chaque Core a une **autorite exclusive** dans son domaine
2. Les Cores **decident ou gouvernent**, n'executent jamais
3. La strate Cores est **immuable** (LOI-7)
4. Les Cores ne communiquent pas directement entre eux — passage par BondingBrother

## Matrice des responsabilites

| Core | Decide | Gouverne | Execute | Observe |
|------|--------|----------|---------|---------|
| StrongFather | Oui | Oui | Non | Non |
| KindMother | Non | Oui | Non | Non |
| Caring Nanny | Non | Non | Non | Oui |
| Master Butler | Non | Oui | Non | Non |
| Border Guard | Non | Oui | Non | Non |
| Ever Buddy | Non | Oui | Non | Oui |
| WorrySentinel | Oui | Oui | Non | Oui |
| TAMR | Oui | Non | Non | Non |

## Interactions typiques

### Flux d'execution standard
```
Operateur → BondingBrother → StrongFather (decide)
                           → Master Butler (permissions)
                           → WorrySentinel (securite)
                           → KindMother (donnees)
                           → Outil (execute)
```

### Flux de securite
```
Anomalie detectee → Caring Nanny (observe)
                  → WorrySentinel (evalue)
                  → StrongFather (decide action)
                  → TAMR (si intervention humaine necessaire)
```

### Flux de migration
```
Demande migration → Border Guard (regles frontieres)
                  → Ever Buddy (compatibilite)
                  → StrongFather (decision)
                  → BondingBrother (traduction)
                  → KindMother (persistance)
```

## Etats de confiance (WorrySentinel)

| Etat | Nom | Impact |
|------|-----|--------|
| T0 | Normal | Toutes capacites disponibles |
| T1 | Instable | Surveillance accrue |
| T2 | Degrade | Capacites reduites |
| T3 | Restreint | Gel des non-essentiels |
| T4 | Bloque | Uniquement diagnostics |

## Niveaux de securite (WorrySentinel)

| Niveau | Nom | Usage |
|--------|-----|-------|
| 0 | Public | Donnees publiques |
| 1 | Standard | Contraintes de base |
| 2 | Sensitive | Contraintes renforcees |
| 3 | Critical | Contraintes strictes |
| 4 | Highest | Securite maximale |

## Securite heterogene

Un Operateur a un SEUL niveau de securite. Une Equipe peut combiner plusieurs niveaux. Les ponts entre niveaux sont explicites, rares, audites par WorrySentinel.
