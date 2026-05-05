# Présentation Générale de Miyukini COG

## Qu'est-ce que Miyukini ?

**Miyukini COG** (Core-Orchestrated Governance Environment) est un écosystème logiciel souverain et autonome, conçu pour offrir une alternative complète aux solutions cloud centralisées.

Miyukini n'est **pas** un système d'exploitation. C'est un **environnement de gouvernance** orchestré par des composants appelés **Cores**, qui garantissent l'intégrité, la sécurité et l'autonomie de chaque installation.

### Vision en une phrase

> **Miyukini est un écosystème logiciel autonome et gouverné, capable de remplacer CMS et SaaS tout en offrant souveraineté technique, fonctionnement offline et contrôle total de la chaîne, du noyau jusqu'à l'utilisateur.**

### L'ampleur du projet

- **Architecture complète** : Kernel + 8 Cores + 49 Toolkits en Rust
- **Souveraineté** : versionning immuable (LOI-7, LOI-8), évolution par environnements
- **Offline-first** : SQLite local, aucune dépendance externe critique
- **Gouvernance structurelle** : StrongFather, KindMother, WorrySentinel, TAMR, BorderGuard
- **Recomposition** : briques (Outils, Toolkits) recomposables en Opérateurs et Services

## Philosophie Fondatrice

### Autonomie Totale

Miyukini est construit sur le principe fondamental que **votre environnement vous appartient**. Aucune dépendance externe critique n'est requise pour le fonctionnement du système. Vous pouvez opérer en isolation complète ou en fédération avec d'autres COGs.

### Gouvernance par les Cores

Huit **Cores** immuables gouvernent le système selon des règles strictes et non négociables. Ces Cores ne sont jamais modifiés — ils évoluent uniquement par création de nouveaux environnements.

### Souveraineté des Données

Vos données restent sur votre matériel. Le système est conçu pour fonctionner offline-first, avec la fédération comme option, jamais comme obligation.

## Architecture en Strates

Miyukini est structuré en **strates hiérarchiques**, de la réalité physique (Strate 0) jusqu'à l'administration souveraine (Strate 9) :

| Strate | Nom | Description |
|--------|-----|-------------|
| **9** | MiyukiniAdmin | Opérateur souverain d'administration |
| **7** | Opérateurs | Services fonctionnels (JayKoa, JayKonta...) |
| **6** | Outils | Capacités techniques (MiyuAuth, MiyuSQL...) |
| **5** | Interface | BondingBrother (adaptation Cores ↔ Outils) |
| **4** | Cores | 8 Cores de gouvernance immuables |
| **3** | Invariants | Principes architecturaux fondamentaux |
| **K** | Kernel | Substrat technique neutre |
| **0** | Hardware | Réalité physique (OS, matériel) |

## Les 8 Lois d'Autonomie

Le système repose sur **8 lois non négociables** qui garantissent l'indépendance de chaque COG :

1. **LOI-1** : Aucune dépendance externe critique à l'exécution
2. **LOI-2** : Le système accepte l'isolement comme état normal
3. **LOI-3** : L'état local est souverain
4. **LOI-4** : Pas de temps global requis
5. **LOI-5** : Le coût doit être proportionnel au hardware
6. **LOI-6** : L'autonomie n'empêche pas la fédération
7. **LOI-7** : La strate Cores est immuable
8. **LOI-8** : Migration = diplomatie entre environnements

## Services Intégrés

Miyukini propose une suite complète de services professionnels et personnels :

- **Miyukini Central** : Hub principal et point d'accès
- **Miou** : Assistant intelligent intégré
- **JayKoa** : Calendrier universel du COG
- **JayKonta** : Comptabilité et gestion financière
- **JayRDV** : Gestion de rendez-vous
- **JayShop** : Commerce en ligne et point de vente
- **Jay Bureau** : Suite collaborative (Docs, Sheets, Slides, Mail, Message)
- **Miyukini Cloud** : Cloud privé (WebDAV, CalDAV, CardDAV)
- **MAIA** : IA locale (LLM, STT) sans internet
- Et plus encore...

## Réseau Webway (MWS)

Le **Miyukini Webway System** permet aux COGs de se découvrir et de communiquer entre eux de manière sécurisée et optionnelle. Ce système comprend :

- **Origin** : Source de vérité du réseau
- **Relays** : Nœuds de distribution et vérification
- **Trackers** : Découverte et contrôle des COGs

## Pour Qui ?

Miyukini s'adresse à :

- **Professionnels indépendants** (kiné, artisan, restaurateur) : JayRDV + JayKonta + JayKoa interconnectés, un seul COG, données unifiées
- **Collectivités et associations** : déploiement sur mini PC ou NAS, budget maîtrisé, données citoyens en local
- **Équipes collaboratives** : suite Jay Bureau souveraine (documents, tableurs, présentations, messagerie chiffrée) sans cloud tiers
- **Décideurs techniques** : éviter le lock-in vendor, traçabilité structurelle, évolution sans refonte
- **Développeurs** : 49 Toolkits composables sous gouvernance, contrats clairs, pas de dépendance externe critique

## Ce que Miyukini n'est pas

| Ce n'est pas | C'est |
|--------------|--------|
| Un OS (Linux, Windows) | Un **environnement de gouvernance** qui tourne *sur* un OS |
| Un framework « ouvert » où chacun fait comme il veut | Un **environnement gouverné** : les Cores sont le socle non substituable |
| Une application ou un CMS amélioré (WordPress++, Shopify++) | Un **écosystème** qui permet de déployer des Services gouvernés |
| Un outil no-code magique sans contraintes | Un socle **exigeant** (autonomie, contrats, traçabilité) en échange de garanties (offline, souveraineté, pérennité) |

## Commencer

Explorez la documentation par catégorie :

- 📐 **Architecture** : Comprendre la structure du système
- ⚙️ **Cores** : Les 8 piliers de gouvernance
- 🔧 **Outils** : Les capacités techniques disponibles
- 📱 **Services** : Les applications intégrées
- 🌐 **MWS** : Le réseau de fédération
- 🔒 **Sécurité** : Protection et confiance
