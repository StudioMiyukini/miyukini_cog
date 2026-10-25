# Définition d'un COG

## Core-Orchestrated Governance Environment

Un **COG** est l'unité fondamentale du système Miyukini : un environnement **souverain, gouverné, isolé et identifié**, qui tourne sur votre propre matériel.

### Décryptage de l'acronyme

| Lettre | Signification | Sens |
|--------|---------------|------|
| **C** | **C**ore | Les Cores sont les unités fondamentales de gouvernance |
| **O** | **O**rchestrated | Coordonné activement — pas « Operating » (ce n'est pas un OS) |
| **G** | **G**overnance Environment | Environnement de gouvernance — actif, institutionnel |

### Formulations courtes

- **Français** : *Miyukini est un COG — un environnement de gouvernance orchestré par des Cores. Il coordonne, sécurise et fait fonctionner des services complets, du noyau jusqu'à l'utilisateur.*
- **English** : *Miyukini is a COG — a Core-Orchestrated Governance Environment. It governs, coordinates and operates software systems from the core to the end user.*

### Analogie de l'engrenage

Le mot « cog » (engrenage) évoque des pièces qui fonctionnent ensemble, avec précision, et transmettent la force de manière prévisible. Phrase signature : *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

## Qu'est-ce qu'un COG ?

Un COG n'est **pas** :
- un simple conteneur ou une VM ;
- une instance cloud ;
- un processus isolé.

Un COG **est** :
- un environnement complet et autonome ;
- gouverné par les **10 Cores** ;
- identifié par une **racine cryptographique** ;
- capable de fonctionner en **isolation totale**.

## Caractéristiques fondamentales

### Souveraineté

L'état local est souverain (LOI-3). Aucune entité externe ne peut :
- modifier son état sans autorisation ;
- **pousser** une mise à jour : le COG relève lui-même les mises à jour du canal officiel, signées (2 sur 2), et c'est son propriétaire qui décide ;
- accéder à ses données sans consentement ;
- **détruire** quoi que ce soit à distance.

### Gouvernance

Rien n'est implicite : toute action est une **intention** décidée par StrongFather (refus par défaut), persistée par KindMother (seul écrivain), surveillée par WorrySentinel, et les décisions critiques passent par une validation humaine (TAMR).

### Isolation

Les frontières d'un COG sont strictes et organisées en **zones** :

```
  Extérieur ──► Douane ──► Zone Franche ──► Portail ◄── Zone COG (aucune sortie)
                           (façades,                    (Cores, services,
                            quarantaine)                 bases principales)
```

- la **zone COG** ne sort jamais : elle tire ce dont elle a besoin par le Portail ;
- chaque service n'a droit qu'aux flux qu'il déclare ;
- toute communication avec l'extérieur est explicite et contrôlée.

### Identification

Un COG est identifié par l'**empreinte de sa racine**, une clé conservée hors ligne. De cette racine dérivent les clés du COG, les **passeports** des personnes et des appareils, et les **visas** d'accès aux services.

## L'analogie du « pays »

| Analogie | Dans un COG |
|----------|-------------|
| Territoire | Zones et frontières définies par BorderGuard |
| Douane | La Douane (pare-feu) et la quarantaine |
| Constitution | Doc gelée, invariants et contrats |
| Gouvernement | Les 10 Cores |
| Citoyens | Services et opérateurs, assujettis aux Cores |
| Papiers d'identité | Passeports et visas signés |
| Relations diplomatiques | Fédération : reconnaissance mutuelle, quorum, preuves signées |

*C'est une instance de gouvernance, pas un simple runtime.*

## Confiance et fédération

Un COG peut choisir de :

1. **Rester seul** : fonctionnement totalement autonome, aucune communication externe (LOI-2).
2. **Rejoindre la Fédération** : il est reconnu progressivement par les autres COG.

| Statut fédéral | Comment on l'obtient |
|----------------|----------------------|
| **Candidat** | Candidature signée, avec une preuve de calcul légère |
| **Invité** | Confirmé par au moins 2 membres |
| **Membre** | Confirmé par au moins 4 membres pendant 7 jours |
| **Suspendu** | Accusé au quorum de 6, preuve signée à l'appui |
| **Révoqué** | 72 h de suspension sans recours accepté |

Côté frontières, BorderGuard classe chaque interlocuteur : **TRUSTED**, **VERIFIED**, **UNKNOWN** ou **HOSTILE**.

## Cycle de vie

### Création

Un COG naît avec :
1. une **racine** (clé hors ligne) et son identifiant ;
2. ses **zones** et ses frontières ;
3. un ensemble de services initiaux, chacun avec son compte, son profil AppArmor et ses flux déclarés.

### Évolution

- Les mises à jour arrivent par le **canal officiel** signé ; le COG les **relève**, les vérifie, et peut revenir en arrière automatiquement en cas d'échec.
- **EverBuddy** garde l'historique immuable des évolutions.
- La doc de référence est **gelée** : toute évolution de conception passe par un amendement écrit avant le code.

### Données

- Les données restent sur le COG, chiffrées, une clé par base.
- L'**oubli** est réel : une donnée oubliée est effacée physiquement, journal compris.

## Exemple concret

```
Mon COG « Cabinet »
├── Racine : clé hors ligne (empreinte = identité du COG)
├── Zone COG : Cores, KindMother, MAIA, JayKoa, JayKonta, JayRDV
├── Zone Franche : page de prise de rendez-vous publique
├── Passeports : moi, mon téléphone, mon assistante
└── Fédération : membre (ou seul, au choix)
```

Ce COG :
- fonctionne même sans Internet ;
- garde vos données chez vous, chiffrées ;
- laisse MAIA vous aider sans jamais sortir vos données ;
- peut coopérer avec d'autres COG si vous le souhaitez ;
- reste sous votre contrôle total.

## Voir aussi

- [Présentation générale](presentation-generale.md)
- [Lois d'autonomie](../architecture/lois-autonomie.md)
- [Souveraineté environnement](../architecture/souverainete-environnement.md)
- [Pyramide des strates](../architecture/pyramide-strates.md)
- [Glossaire](../reference/glossaire.md)
