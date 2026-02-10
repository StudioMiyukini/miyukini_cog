# Miyukini Conceptual References — Souveraineté Environnement

## Contexte

Ce document définit les **règles fondamentales de souveraineté** des environnements Miyukini (COG). Il établit le principe architectural selon lequel un COG est une entité souveraine, versionnée, isolée et identifiable de manière unique.

Ce document formalise la distinction entre Miyukini et les modèles SaaS traditionnels : **Miyukini adopte le modèle kernel/distribution, pas le modèle SaaS.**

## Portée / Scope

- **Applicable à :** Architecture globale, stratégie de versioning, déploiement
- **Audience :** Architectes, développeurs, opérateurs d'environnement
- **Statut :** Document de référence normatif

---

## 1. Règle Fondatrice : Pas de Patch, Que des Environnements

### Énoncé canonique

> **Dans Miyukini, la strate Cores est immuable.**  
> **Toute évolution se fait par la création d'un nouvel environnement complet.**  
> **Les Opérateurs sont liés à un environnement unique et ne peuvent exister hors de celui-ci.**

### Ce que cette règle signifie

| Aspect | Règle |
|--------|-------|
| **Pas de micro-patch** | La strate Cores (Strate 4) n'est jamais patchée individuellement |
| **Versions complètes uniquement** | Toute évolution produit une version complète d'environnement |
| **Pas de hotfix sauvage** | Pas de "fix en prod à 3h du matin" |
| **Stabilité temporelle** | Délai imposé entre les versions |

### Pourquoi cette règle est fondamentale

| Bénéfice | Explication |
|----------|-------------|
| ✅ Évite l'enfer des micro-patchs | Pas de cascade de corrections incompatibles |
| ✅ Évite la fragmentation de compatibilité | Un environnement = une version cohérente |
| ✅ Évite la course permanente des devs tiers | Cible stable, temps de développement prévisible |
| ✅ Facilite l'audit et la certification | Version figée, auditable, certifiable |
| ✅ Permet des environnements LTS | Long-Term Support possible |

---

## 2. Définition : Un Environnement COG est une Entité Souveraine

### Définition canonique

> Un environnement Miyukini (COG) est une **entité souveraine, versionnée, isolée et identifiée de manière unique**.

### Caractéristiques d'un environnement COG

| Propriété | Description |
|-----------|-------------|
| **Version complète des cores** | Ensemble cohérent et figé de tous les cores (Strate 4) |
| **Itération unique** | Numéro de version distinct dans l'historique |
| **ID d'environnement unique** | Identifiant généré à la création par le kernel |
| **Ensemble d'Opérateurs assujettis** | Opérateurs (Strate 7) liés à cet environnement |
| **Frontières strictes** | Limites claires entre l'environnement et l'extérieur |

### Ce qu'un environnement COG représente

**👉 C'est une instance de gouvernance, pas un simple runtime.**

L'analogie du "pays" est techniquement pertinente :

| Analogie | Environnement COG |
|----------|-------------------|
| Territoire | Frontières définies par Border Guard |
| Constitution | Invariants et contrats (Strate 3) |
| Gouvernement | Cores système (Strate 4) |
| Citoyens | Opérateurs assujettis (Strate 7) |
| Identité nationale | ID d'environnement unique |
| Relations diplomatiques | Migration via BondingBrother |

---

## 3. Règle d'Or : Dépendance Verticale Stricte

### Énoncé

| Règle | Implication |
|-------|-------------|
| ❌ **Un Opérateur ne peut pas communiquer avec plusieurs versions de strates Cores** | Pas de "multi-version runtime" |
| ❌ **Un Opérateur n'est jamais portable dynamiquement entre environnements** | Pas de migration à chaud |

### Conséquence : Liaison Stricte Opérateur/Environnement

Un Opérateur dans Miyukini est :

| Propriété | Description |
|-----------|-------------|
| **Lié à une version de l'environnement** | Compilé/configuré pour une version spécifique |
| **Lié à une itération précise** | Pas de compatibilité implicite entre itérations |
| **Asservi aux règles de son environnement** | Soumis aux politiques StrongFather de cet environnement |

### Ce qui est interdit

| ❌ Interdit | Pourquoi |
|-------------|----------|
| Cross-core hacks | Un Opérateur ne peut pas contourner les cores pour communiquer |
| Compat layer sauvage | Pas de couche de compatibilité non officielle |
| Import dynamique de cores | Un Opérateur ne charge pas de cores à la volée |
| Multi-environment runtime | Un Opérateur ne tourne pas "entre" deux environnements |

---

## 4. Coexistence sur un Même Hardware : Validée

### Règle

**Plusieurs environnements COG peuvent coexister sur un même hardware physique.**

### Schéma d'architecture

```
Hardware Physique
 │
 ├─ Miyukini Env A (COG v1.2 LTS)
 │   ├─ Opérateurs A1, A2
 │   └─ [ID: env-a-uuid]
 │
 ├─ Miyukini Env B (COG v2.0)
 │   ├─ Opérateurs B1
 │   └─ [ID: env-b-uuid]
 │
 └─ Miyukini Env C (isolé / offline)
     ├─ Opérateurs C1
     └─ [ID: env-c-uuid]
```

### Pourquoi aucun conflit

| Raison | Explication |
|--------|-------------|
| **Pas de patch partagé** | Chaque environnement a ses propres cores complets |
| **Pas de core mutualisé** | Aucune ressource système partagée entre environnements |
| **Pas de dépendance transversale** | Chaque environnement est autonome |
| **Isolation complète** | Les frontières sont strictement définies |

---

## 5. Identité d'Environnement : Modèle à 3 Niveaux

### Principe de génération

| Propriété | Règle |
|-----------|-------|
| **Générée par le kernel** | Seul le kernel peut créer une ID d'environnement |
| **Unique** | Garantie d'unicité (locale ou globale selon le mode) |
| **Immuable** | L'ID ne change jamais après création |

### Les 3 niveaux d'identité

#### Niveau 1 : Local Sovereign ID (LSI)

| Aspect | Description |
|--------|-------------|
| **Génération** | Par le kernel local à la création |
| **Validité** | Toujours valide localement |
| **Unicité** | Garantie localement (UUID v4 ou équivalent) |
| **Cas d'usage** | Environnement isolé, offline permanent |
| **Confiance** | Souveraine — l'environnement s'auto-déclare |

#### Niveau 2 : Verified ID (VID)

| Aspect | Description |
|--------|-------------|
| **Génération** | LSI vérifiée par un registre global |
| **Validité** | Valide globalement si réseau disponible |
| **Unicité** | Vérifiée contre un registre central |
| **Cas d'usage** | Environnement connecté, fédéré |
| **Confiance** | Attestée — un tiers a vérifié l'identité |

#### Niveau 3 : Witnessed ID (WID)

| Aspect | Description |
|--------|-------------|
| **Génération** | LSI vérifiée par échange indirect |
| **Validité** | Valide dans un réseau de confiance distribué |
| **Unicité** | Vérifiée par témoins (autres environnements) |
| **Cas d'usage** | Environnement semi-connecté, clé USB, QR, signature |
| **Confiance** | Témoignée — d'autres environnements attestent |

### Gradation de confiance

```
Confiance minimale                                    Confiance maximale
      │                                                      │
      ▼                                                      ▼
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│  Local      │  →   │  Witnessed  │  →   │  Verified   │
│  Sovereign  │      │  ID         │      │  ID         │
│  ID (LSI)   │      │  (WID)      │      │  (VID)      │
└─────────────┘      └─────────────┘      └─────────────┘
  Auto-déclaré        Témoigné            Attesté
```

### Compatibilité avec l'autonomie

Cette gradation est **parfaitement compatible** avec le modèle Miyukini :

| Principe | Respect |
|----------|---------|
| **Offline-first** | ✅ LSI fonctionne sans réseau |
| **Souverain** | ✅ Chaque niveau est valide dans son contexte |
| **Interopérabilité contrôlée** | ✅ Les niveaux supérieurs permettent la fédération |

---

## 6. Migration = Diplomatie entre Environnements

### Principe fondamental

> **Migration ≠ Communication directe**

Les environnements COG peuvent échanger des données **si et seulement si** cela est **explicitement permis**.

### Règles de migration

| Règle | Description |
|-------|-------------|
| **Migration = processus** | Pas d'échange instantané, mais une procédure formelle |
| **Migration = contrat** | Les deux environnements acceptent explicitement l'échange |
| **Migration = frontière contrôlée** | Border Guard valide chaque transfert |
| **Migration = traduction** | BondingBrother traduit entre versions, pas de copie brute |

### Acteurs de la migration

| Core | Rôle dans la migration |
|------|------------------------|
| **Border Guard** | Définit les règles de franchissement |
| **BondingBrother** | Traduit et médie l'échange |
| **StrongFather** | Décide si la migration est autorisée |
| **KindMother** | Exécute la persistance des données migrées |
| **Ever Buddy** | Valide la compatibilité des versions |

### Ce qui est migrable vs ce qui ne l'est jamais

| Migrable | Non migrable |
|----------|--------------|
| Données métier (avec traduction) | Politiques actives |
| Journaux exportables | Sessions et tokens |
| Métadonnées de synchronisation | État système temps réel |
| Schémas (si compatibles) | Cache et données transitoires |

---

## 7. Sécurité & Temporalité : Ralentir Volontairement l'Évolution

### Principe

> **On veut garantir la sécurité, et imposer un délai entre les versions pour éviter la course aux patches.**

### Implications

| Aspect | Règle |
|--------|-------|
| **Pas de hotfix sauvage** | Toute correction passe par une nouvelle version complète |
| **Pas de "fix en prod à 3h du matin"** | Les urgences sont gérées par dégradation, pas par patch |
| **Délai minimal entre versions** | Temps de stabilisation imposé (LTS par exemple) |
| **Cycle prévisible** | Les développeurs tiers peuvent planifier |

### Bénéfices pour les développeurs tiers

| Bénéfice | Explication |
|----------|-------------|
| **Cible stable** | Une version = une cible de développement fixe |
| **Temps disponible** | Pas de course permanente à la compatibilité |
| **Qualité** | Possibilité de tests approfondis |
| **Prévisibilité** | Planning de développement fiable |

### Ce que Miyukini privilégie

| Priorité | Description |
|----------|-------------|
| ✅ **Fiabilité** | Un environnement fonctionne ou ne fonctionne pas — pas d'état intermédiaire douteux |
| ✅ **Prévisibilité** | Comportement déterministe et documenté |
| ✅ **Qualité système** | Architecture solide plutôt que features rapides |
| ❌ **Pas la hype** | Pas de course aux nouvelles fonctionnalités |

---

## 8. Modèle de Souveraineté Logicielle Versionnée

### Ce que Miyukini crée

> **Un modèle de souveraineté logicielle versionnée.**

### Ce que Miyukini n'est PAS

| ❌ N'est pas | Pourquoi |
|--------------|----------|
| Un OS | C'est un environnement de gouvernance, pas un système d'exploitation |
| Un framework | C'est un écosystème complet avec ses propres règles |
| Un SaaS | Pas de dépendance cloud, pas de patch continu |

### Ce que Miyukini EST

| ✅ Est | Description |
|--------|-------------|
| **Environnement gouverné** | Règles explicites et appliquées |
| **Versionné** | Chaque environnement a une version complète |
| **Isolable** | Peut fonctionner seul, offline, sans dépendance |
| **Migrable** | Les données peuvent être transférées entre environnements |
| **Auditable** | Toute action est traçable et vérifiable |

---

## 9. Résumé des Règles Fondamentales

### Tableau de synthèse

| # | Règle | Statut |
|---|-------|--------|
| 1 | La strate Cores n'est jamais patchée | **NON NÉGOCIABLE** |
| 2 | Un COG est une entité souveraine | **NON NÉGOCIABLE** |
| 3 | Un Opérateur est lié à un environnement unique | **NON NÉGOCIABLE** |
| 4 | Plusieurs COG peuvent coexister sur un hardware | **AUTORISÉ** |
| 5 | L'identité est à 3 niveaux (LSI, WID, VID) | **RECOMMANDÉ** |
| 6 | Migration = diplomatie explicite | **NON NÉGOCIABLE** |
| 7 | Évolution ralentie volontairement | **RECOMMANDÉ** |
| 8 | Données sensibles à résidence centralisée : copie canonique sur COG de référence | **NORMATIF** (voir Politique de résidence) |
| 9 | Les Services Fondamentaux (Central, Portail) font partie de l'environnement versionné | **NON NÉGOCIABLE** |

### Formulation officielle

> **Dans Miyukini, la strate Cores est immuable.**  
> **Toute évolution se fait par la création d'un nouvel environnement complet.**  
> **Les Opérateurs sont liés à un environnement unique et ne peuvent exister hors de celui-ci.**

---

## 10. Résidence des données sensibles

Certaines données sensibles (données personnelles, métier critique) ne doivent pas avoir pour seule copie un terminal ou un COG tiers. Leur **copie canonique** réside sur un **COG de référence** désigné (voir [Politique de résidence des données sensibles](./Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)).

**Effet :** En cas de coupure du terminal (ex. exposant), les données restent disponibles sur le COG de référence (ex. pour les organisateurs). La souveraineté du COG de référence inclut la détention canonique de ces données ; les terminaux accèdent via Visite gouvernée ou synchronisation, sans en être la seule copie.

---

## 11. Services Fondamentaux

### Définition

Les **Services Fondamentaux** sont les points d'entrée structurels de l'écosystème COG dont la présence fait partie de l'environnement versionné. Ils ne sont pas optionnels.

### Les deux Services Fondamentaux

| Service | Rôle | Cible |
|---------|------|-------|
| **Miyukini Central** | Hub de gestion des Services — point d'entrée utilisateur COG | Utilisateur du COG (gestion, admin, création) |
| **Miyukini Web Portal** | Hub des surfaces web — point d'entrée utilisateurs externes | Utilisateurs externes (web, sans COG) |

### Règle canonique

> **Central = COG, Portail = Web.**
>
> Un COG sans Central ne peut pas être administré.  
> Un COG sans Portail ne peut pas exposer de surfaces web.

### Position dans l'environnement versionné

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Environnement COG (versionné)                     │
│                                                                      │
│  ┌────────────────────────────┐  ┌────────────────────────────┐    │
│  │   Miyukini Central         │  │   Miyukini Web Portal      │    │
│  │   (Service Fondamental)    │  │   (Service Fondamental)    │    │
│  │   · Point d'entrée COG     │  │   · Point d'entrée Web     │    │
│  └────────────────────────────┘  └────────────────────────────┘    │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    Services (Type 1, 2, 3)                    │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    Strate Cores (immuable)                    │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### Implication pour le versioning

Lorsque l'environnement COG est versionné, les Services Fondamentaux font partie de cette version :

- Version des Cores (Strate 4) → immuable
- Version des Services Fondamentaux (Central, Portail) → versionnée avec l'environnement
- Version des Services métier → peut évoluer indépendamment (sous gouvernance)

**Voir aussi :** [Types de Services et Espaces](./Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md), [Miyukini Central Hub Services](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md), [Miyukini Web Portal](../services/MiyukiniWebPortal/_index.md)

---

**Date de création :** 2026-01-27  
**Version :** 1.4 (ajout Services Fondamentaux : Miyukini Central et Miyukini Web Portal)  
**Statut :** Document de référence normatif

**Références croisées :**
- [Miyukini Conceptual References - Comportement COG Environnements](./Miyukini%20Conceptual%20References%20-%20Comportement%20COG%20Environnements.md) : Schéma et comportement des environnements COG
- [Miyukini Conceptual References - Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md) : Définition officielle COG
- [Miyukini Conceptual References - Types de Services et Espaces](./Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) : Classification des Services (Type 1, 2, 3)
- [Miyukini Conceptual References - Miyukini Central Hub Services](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md) : Service Fondamental — Hub COG
- [Miyukini Web Portal - Document Fondateur](../services/MiyukiniWebPortal/Miyukini%20Web%20Portal%20-%20Document%20Fondateur.md) : Service Fondamental — Hub Web
- [Miyukini Conceptual References - Politique Residence Donnees Sensibles](./Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) : Centralisation et résidence des données sensibles
- [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) : Terminologie officielle
- [Miyukini Conceptual References - Lois Autonomie Systeme](./Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : Contraintes d'autonomie
- [Miyukini Conceptual References - Vision Strategique](./Miyukini%20Conceptual%20References%20-%20Vision%20Strategique.md) : Stratégie globale
- [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) : Architecture en strates
- [Miyukini Conceptual References - Kernel Maintenance Observability Contract](./Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md) : Capacités bas niveau de maintenance (compatible isolation)
- [BondingBrother - Migration & Compatibility Contract](../core/BondingBrother/BondingBrother%20-%20Migration%20%26%20Compatibility%20Contract.md) : Contrat de migration
- [Border Guard - Documentation Fondatrice](../core/BorderGuard/Border%20Guard%20-%20Documentation%20Fondatrice.md) : Frontières et confiance
- [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Services Fondamentaux, Types de Services