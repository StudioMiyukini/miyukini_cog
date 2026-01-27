# Miyukini Conceptual References — Outils et Kits d'Outils (Tools & Toolkits)

## Contexte

Ce document définit les **concepts canoniques d'Outil (Tool) et de Kit d'Outils (Toolkit)** dans l'écosystème Miyukini. Il établit les règles de gouvernance, les responsabilités des Cores impliqués, et les contraintes architecturales associées.

**Phrase de doctrine :**

> **Tools are governed executable capabilities. Toolkits are official compositions of tools, optimized for efficiency but never for authority.**

> **Les Outils sont des capacités exécutables gouvernées. Les Kits d'Outils sont des compositions officielles d'outils, optimisées pour l'efficience mais jamais pour l'autorité.**

## Portée / Scope

- **Applicable à :** Architecture système, développement d'Opérateurs, gouvernance des capacités
- **Audience :** Architectes, développeurs, opérateurs d'environnement
- **Statut :** Document de référence normatif

---

## 1. Définition Canonique : Outil (Tool)

### Énoncé canonique

> Un **Outil (Tool)** est une **capacité exécutable**, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.

### Caractéristiques d'un Outil

| Propriété | Description |
|-----------|-------------|
| **Capacité exécutable** | Fait quelque chose de concret et atomique |
| **Sans autorité** | Ne décide jamais si l'action doit être faite |
| **Sans décision métier** | Ne contient aucune logique business |
| **Sans connaissance du contexte** | Ignore quel Opérateur l'appelle et pourquoi |
| **Gouverné par les Cores** | Soumis à l'autorisation et au contrôle des Cores |

### Règle fondamentale

> **👉 Un Outil fait, mais ne décide jamais.**

### Ce qu'un Outil N'EST PAS

| ❌ N'est pas | Pourquoi |
|--------------|----------|
| Un service métier | Pas de logique business |
| Un décideur | Pas d'autorité |
| Un core | Pas de gouvernance propre |
| Un Opérateur | Pas de connaissance du contexte |

### Exemples d'Outils

| Domaine | Outil | Action |
|---------|------|--------|
| UI | `layout.render` | Rend un layout |
| UI | `input.capture` | Capture une saisie utilisateur |
| UI | `form.validate` | Valide un formulaire |
| UI | `theme.resolve` | Résout un thème |
| UI | `event.dispatch` | Dispatch un événement |
| Data | `query.execute` | Exécute une requête |
| Data | `cache.get` | Récupère depuis le cache |
| IO | `file.read` | Lit un fichier |
| IO | `file.write` | Écrit un fichier |

---

## 2. Définition Canonique : Kit d'Outils (Toolkit)

### Énoncé canonique

> Un **Kit d'Outils (Toolkit)** est une **composition officielle d'Outils**, validée et déclarée par l'environnement, optimisée pour efficience, cohérence et performance.

### Caractéristiques d'un Kit d'Outils

| Propriété | Description |
|-----------|-------------|
| **Composition officielle** | Agrégation formelle d'Outils existants |
| **Validé par l'environnement** | Déclaré et gouverné |
| **Optimisé** | Pour efficience, cohérence, performance |
| **Sans capacité nouvelle** | N'ajoute aucune fonctionnalité que les Outils n'ont pas |
| **Sans logique métier** | Orchestration pure, pas de décision |

### Règle fondamentale

> **👉 Un Kit d'Outils n'ajoute aucune capacité nouvelle, il orchestre proprement des Outils existants.**

### Ce qu'un Kit d'Outils N'EST PAS

| ❌ N'est pas | Pourquoi |
|--------------|----------|
| Un nouvel Outil | Il ne crée pas de capacité nouvelle |
| Un service | Il n'a pas de logique propre |
| Un décideur | Il n'a pas d'autorité |
| Une librairie libre | Il est gouverné |

### Exemple : Kit d'Outils UI

```
Kit d'Outils UI
 ├─ layout.render
 ├─ input.capture
 ├─ form.validate
 ├─ theme.resolve
 └─ event.dispatch
```

**Ce que fait le Kit d'Outils UI :**
- Regroupe ces Outils
- Optimise les appels
- Normalise les flux

**Ce que le Kit d'Outils UI ne fait PAS :**
- Décider quand afficher une UI
- Choisir quel thème appliquer
- Autoriser ou refuser une action

---

## 3. Structure Mentale : Outil vs Kit d'Outils

```
Outil (Tool)
 └─ expose des capabilities atomiques

Kit d'Outils (Toolkit)
 └─ agrège des Outils
 └─ sans logique métier
 └─ sans décision
```

### Flux d'appel

Un Opérateur peut :
- Appeler un **Outil isolé** pour une capacité atomique
- Appeler un **Kit d'Outils** pour plus d'efficience

**Mais dans les deux cas :**
- Il passe par la **même gouvernance**
- Il est soumis aux **mêmes règles**

**Note terminologique :** Le terme "produit" est incorrect. La terminologie correcte est **Opérateur**. Voir [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md).

### Schéma de flux

```
Opérateur (Strate 7)
    │
    ▼
┌───────────────────────────────────────┐
│  BondingBrother (médiation)           │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Master Butler : "Cet Outil existe-   │
│  t-il et cet Opérateur a-t-il le      │
│  droit ?"                             │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  WorrySentinel : "Le niveau de        │
│  sécurité permet-il cet appel ?"      │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Caring Nanny : "L'état système       │
│  permet-il cet appel ?"               │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Outil / Kit d'Outils (exécution)     │
└───────────────────────────────────────┘
```

---

## 4. Gouvernance des Outils et Kits d'Outils

### Règle ABSOLUE

> **Un environnement Miyukini possède une bibliothèque d'outils finie, déclarée, gouvernée.**

| Règle | Description |
|-------|-------------|
| **Pas d'injection sauvage** | Aucun Outil ne peut être ajouté dynamiquement sans gouvernance |
| **Pas d'Outil "local"** | Tout Outil doit être déclaré dans l'environnement |
| **Pas de dépendance externe cachée** | Aucune librairie externe non déclarée |

**👉 C'est une souveraineté applicative.**

### Ce que les Cores NE FONT PAS

| Core | Ce qu'il ne fait PAS |
|------|----------------------|
| **Master Butler** | N'implémente pas les Outils |
| **Master Butler** | Ne décrit pas leur logique |
| **Master Butler** | Ne gère pas leur cycle de vie technique |

---

## 5. Responsabilités des Cores

### 5.1 Master Butler — Capability & Permission Core

**Rôle central :** Catalogue des capacités et permissions.

| Responsabilité | Description |
|----------------|-------------|
| **Déclarer** | Quelles capabilities existent dans l'environnement |
| **Lier** | Capability → Outil |
| **Autoriser** | Qui peut appeler quoi |
| **Définir** | Les permissions d'accès |

**Question à laquelle Master Butler répond :**

> *"Qu'est-ce qui est possible dans cet environnement ?"*

**Ce que Master Butler connaît :**
- Liste des Outils disponibles
- Liste des Kits d'Outils déclarés
- Mapping Capability → Outil
- Permissions par Opérateur/rôle

**Ce que Master Butler ne fait PAS :**
- Implémenter les Outils
- Exécuter les Outils
- Décider si un Outil doit être appelé

---

### 5.2 Ever Buddy — Lifecycle & Evolution Core

**Rôle :** Cycle de vie et évolution des Outils.

| Responsabilité | Description |
|----------------|-------------|
| **Versions** | Gère les versions des Outils |
| **Dépréciation** | Marque les Outils obsolètes |
| **Compatibilité** | Vérifie Outil ↔ Environnement |
| **Migration** | Gère la transition Outil → nouvelle version |

**Question à laquelle Ever Buddy répond :**

> *"Est-ce que cet outil existe encore, est compatible, ou doit être migré ?"*

**Ce que Ever Buddy connaît :**
- Version actuelle de chaque Outil
- Versions dépréciées
- Chemins de migration
- Compatibilité avec l'environnement COG

---

### 5.3 Caring Nanny — Product State Core

**Rôle :** Cohérence globale de l'environnement.

| Responsabilité | Description |
|----------------|-------------|
| **États autorisés** | Définit quand un Outil peut être utilisé |
| **Blocage conditionnel** | Bloque si l'environnement est dégradé |
| **Observation** | Surveille l'état du système |

**Question à laquelle Caring Nanny répond :**

> *"L'état actuel du système permet-il cet appel ?"*

**Exemple de blocage :**

```
Kit d'Outils UI indisponible car environnement en état SECURITY_LOCKDOWN
```

**Ce que Caring Nanny connaît :**
- État actuel de l'environnement
- États qui bloquent certains Outils
- Règles de dégradation

---

### 5.4 WorrySentinel — Security Governance Core

**Rôle :** Gouvernance de sécurité des Outils.

| Responsabilité | Description |
|----------------|-------------|
| **Niveau de sécurité** | Définit le niveau requis pour chaque Outil |
| **Dégradation** | Gère la dégradation sécuritaire |
| **Blocage** | Bloque les Outils en cas de menace |
| **Audit** | Trace les appels pour audit |
| **Autorisation conditionnelle** | Autorise sous conditions |

**Question à laquelle WorrySentinel répond :**

> *"Le niveau de sécurité actuel permet-il cet appel ?"*

**Ce que WorrySentinel connaît :**
- Niveau de sécurité requis par Outil
- Niveau de sécurité actuel de l'environnement
- Règles d'autorisation conditionnelle

---

## 6. Définition des Kits d'Outils

### Qui définit les Kits d'Outils ?

**👉 Pas un seul Core.**

Les Kits d'Outils sont :

| Étape | Core responsable |
|-------|------------------|
| **Déclarés** | Master Butler |
| **Composés** | Documentation + Manifeste |
| **Validés** | WorrySentinel |
| **Compatibilisés** | Ever Buddy |

### Structure d'un Kit d'Outils

Un Kit d'Outils est défini par :

| Élément | Description |
|---------|-------------|
| **Identifiant** | Nom unique du Kit d'Outils |
| **Liste des Outils** | Outils composant le Kit |
| **Version** | Version du Kit d'Outils |
| **Niveau de sécurité** | Niveau requis pour utiliser le Kit |
| **États autorisés** | États système dans lesquels le Kit fonctionne |

### Exemple de manifeste Kit d'Outils

```yaml
toolkit:
  id: "ui.standard"
  version: "1.0.0"
  description: "Kit d'Outils UI Standard"
  tools:
    - layout.render
    - input.capture
    - form.validate
    - theme.resolve
    - event.dispatch
  security_level: 2
  allowed_states:
    - HEALTHY
    - DEGRADED
  disallowed_states:
    - SECURITY_LOCKDOWN
    - MAINTENANCE
```

---

## 7. Règles d'Usage pour les Opérateurs

### Ce qu'un Opérateur peut faire

| Action | Autorisé |
|--------|----------|
| Appeler un Outil isolé | ✅ Oui (si autorisé) |
| Appeler un Kit d'Outils | ✅ Oui (si autorisé) |
| Créer un Outil local | ❌ Non |
| Modifier un Outil | ❌ Non |
| Bypasser la gouvernance | ❌ Non |

### Flux d'appel depuis un Opérateur

```
Opérateur : "Je veux une UI"
    │
    ▼
Environnement : "Voici les Outils autorisés, dans ce cadre"
    │
    ▼
Outil / Kit d'Outils : Exécution
```

### Ce qui est interdit

| ❌ Interdit | Pourquoi |
|-------------|----------|
| Injection d'Outil | Pas d'Outil non déclaré |
| Outil local | Tout doit être dans l'environnement |
| Dépendance cachée | Pas de librairie externe non gouvernée |
| Appel direct | Toujours via BondingBrother |

---

## 8. Résumé : Tableau des Responsabilités

| Élément | Rôle |
|---------|------|
| **Outil (Tool)** | Capacité atomique exécutable |
| **Kit d'Outils (Toolkit)** | Composition officielle d'Outils |
| **Master Butler** | Catalogue des capacités et permissions |
| **Ever Buddy** | Cycle de vie et versions |
| **Caring Nanny** | Cohérence d'état système |
| **WorrySentinel** | Sécurité et audit |
| **Opérateur** | Utilisateur d'Outils (via gouvernance) |

---

## 9. Règles Fondamentales (à graver)

| # | Règle | Statut |
|---|-------|--------|
| 1 | Un Outil fait, mais ne décide jamais | **NON NÉGOCIABLE** |
| 2 | Un Kit d'Outils orchestre, mais n'ajoute pas de capacité | **NON NÉGOCIABLE** |
| 3 | La bibliothèque d'outils est finie et gouvernée | **NON NÉGOCIABLE** |
| 4 | Pas d'injection sauvage d'Outils | **NON NÉGOCIABLE** |
| 5 | Tout appel passe par la gouvernance | **NON NÉGOCIABLE** |

### Formulation officielle

> **Tools are governed executable capabilities.**  
> **Toolkits are official compositions of tools, optimized for efficiency but never for authority.**

> **Les Outils sont des capacités exécutables gouvernées.**  
> **Les Kits d'Outils sont des compositions officielles d'outils, optimisées pour l'efficience mais jamais pour l'autorité.**

---

**Date de création :** 2026-01-27  
**Version :** 1.3 (terminologie française Outil, Kit d'Outils, Opérateur)  
**Statut :** Document de référence normatif

**Références croisées :**
- [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) : Terminologie officielle Opérateurs
- [Master Butler - Documentation Fondatrice](../core/MasterButler/Master%20Butler%20-%20Documentation%20Fondatrice.md) : Catalogue des capacités
- [Ever Buddy - Documentation Fondatrice](../core/EverBuddy/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) : Cycle de vie
- [Caring Nanny - Documentation Fondatrice](../core/CaringNanny/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) : Cohérence d'état
- [WorrySentinel - Documentation Fondatrice](../core/WorrySentinel/WorrySentinel%20-%20Documentation%20Fondatrice.md) : Sécurité
- [BondingBrother - Documentation Fondatrice](../core/BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md) : Médiation
- [Miyukini Conceptual References - Souveraineté Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) : Souveraineté applicative
- [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) : Architecture en strates
