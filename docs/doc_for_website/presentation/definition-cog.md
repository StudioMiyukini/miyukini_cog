# Définition d'un COG

## Core-Orchestrated Governance Environment

Un **COG** (Core-Orchestrated Governance Environment) est l'unité fondamentale du système Miyukini. C'est une entité **souveraine, versionnée, isolée et identifiée**.

### Décryptage de l'acronyme

| Lettre | Signification | Sens |
|--------|---------------|------|
| **C** | **C**ore | Les Cores sont les unités fondamentales de gouvernance |
| **O** | **O**rchestrated | Coordonné activement — pas « Operating » (ce n'est pas un OS) |
| **G** | **G**overnance Environment | Environnement de gouvernance — actif, institutionnel |

### Formulations courtes

- **Français** : *Miyukini est un COG — un environnement de gouvernance orchestré par des Cores. Il coordonne, sécurise et fait fonctionner des systèmes logiciels complets, du noyau jusqu'à l'utilisateur final.*
- **English** : *Miyukini is a COG — a Core-Orchestrated Governance Environment. It governs, coordinates and operates software systems from the core to the end user.*

### Analogie de l'engrenage

Le mot « cog » (engrenage) en anglais évoque : **système** (pièces qui fonctionnent ensemble), **interconnexion**, **précision**, **transmission** de la force de manière prévisible. Phrase signature : *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

## Qu'est-ce qu'un COG ?

Un COG n'est **pas** :
- Un simple conteneur ou une VM
- Une instance cloud
- Un processus isolé

Un COG **est** :
- Un environnement complet et autonome
- Gouverné par une version figée des Cores
- Identifié de manière unique
- Capable de fonctionner en isolation totale

## Caractéristiques Fondamentales

### Souveraineté

Chaque COG est maître de son domaine. Aucune entité externe ne peut :
- Modifier son état sans autorisation
- Forcer une mise à jour
- Accéder à ses données sans consentement

### Versionnement

Un COG est lié à une **version spécifique et immuable** des Cores. Cette version ne change jamais — l'évolution passe par la création d'un nouveau COG et la migration diplomatique des données.

### Isolation

Les frontières d'un COG sont strictes :
- Chaque opérateur est lié à un environnement unique
- Aucune interférence entre COGs
- Communication explicite et contrôlée uniquement

### Identification

Chaque COG possède un **identifiant unique** généré par le Kernel, garantissant son unicité dans le réseau Webway.

## L'analogie du « pays »

| Analogie | Environnement COG |
|----------|-------------------|
| Territoire | Frontières définies par BorderGuard |
| Constitution | Invariants et contrats (Strate 3) |
| Gouvernement | Cores système (Strate 4) |
| Citoyens | Opérateurs assujettis (Strate 7) |
| Identité nationale | ID d'environnement unique |
| Relations diplomatiques | Migration via protocoles formels (LOI-8) |

*C'est une instance de gouvernance, pas un simple runtime.*

## Niveaux d'Identité

Un COG peut avoir différents niveaux de confiance dans le réseau :

| Niveau | Nom | Description |
|--------|-----|-------------|
| **LSI** | Local Sovereign ID | Identité auto-déclarée, confiance minimale |
| **VID** | Verified ID | Identité attestée par un tiers de confiance |
| **WID** | Witnessed ID | Identité témoignée par d'autres COGs |

## Types de COG

Selon leur rôle dans le réseau Webway :

| Type | Description |
|------|-------------|
| **ORIGIN** | Source de vérité, héberge les services d'authentification |
| **RELAY** | Nœud de distribution et de vérification |
| **TRACKER** | Découverte et contrôle des autres COGs |
| **STABLE** | COG standard avec présence permanente |
| **TERMINAL** | COG client léger, dépendant d'un Origin |
| **LONE** | COG isolé, sans participation réseau |

## Cycle de Vie

### Création

Un COG naît avec :
1. Une version des Cores (figée définitivement)
2. Un identifiant unique
3. Un ensemble d'opérateurs initiaux
4. Des frontières définies

### Évolution

Un COG **n'évolue pas** au sens traditionnel :
- Les Cores restent figés (LOI-7)
- Les nouvelles fonctionnalités = nouveau COG + migration
- Les données migrent via un processus diplomatique (LOI-8)

### Migration

La migration entre COGs est un acte formel :
- Jamais une simple copie de données
- Négociation entre environnements
- Validation des invariants
- Préservation de la souveraineté

## COG et Fédération

Un COG peut choisir de :

1. **Rester isolé** (mode LONE)
   - Fonctionnement totalement autonome
   - Aucune communication externe
   - Conformité LOI-2

2. **Rejoindre le réseau Webway**
   - Découverte par les Trackers
   - Communication via les Relays
   - Fédération optionnelle (LOI-6)

## Exemple Concret

Imaginez votre COG personnel :

```
Mon COG "Maison"
├── Cores v1.2.0 (figés)
├── Identité : LSI-abc123...
├── Services actifs :
│   ├── Miyukini Central
│   ├── JayKonta (comptabilité)
│   └── Miou (assistant)
└── Réseau : STABLE (connecté au Webway)
```

Ce COG :
- Fonctionne même sans Internet
- Garde vos données localement
- Peut communiquer avec d'autres COGs si vous le souhaitez
- Reste sous votre contrôle total

## Voir aussi

- [Présentation générale](presentation-generale.md)
- [Lois d'autonomie](../architecture/lois-autonomie.md) — LOI-7 (immutabilité Cores), LOI-8 (migration)
- [Souveraineté environnement](../architecture/souverainete-environnement.md)
- [Pyramide des strates](../architecture/pyramide-strates.md)
- [Glossaire](../reference/glossaire.md)
