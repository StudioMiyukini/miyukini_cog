# MiyuWeb — Runtime Boundary Contract

## 1. Contexte

Ce document définit le **bornage (frontières d'exécution)** du kit MiyuWeb. Il établit ce que MiyuWeb ne fait jamais, les frontières avec les Cores, et les invariants de limite. MiyuWeb est un Kit d'Outils qui orchestre des capacités atomiques d'affichage web (rendu HTML, scripts, assets, thème, layout, formulaires, événements) sans décision de contenu ni accès direct à la base.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Ce que MiyuWeb ne fait jamais (pas de décision ALLOW/DENY, pas de choix de contenu, pas d'accès direct à la base, pas de capacité hors Tools composants)
- Les frontières avec les Cores (KindMother, StrongFather, Master Butler, WorrySentinel, Caring Nanny, BondingBrother)
- Les invariants de limite (bornage)

Ce document **ne couvre pas** :
- Les frontières internes de KindMother (voir KindMother - Runtime Boundary & Enforcement Contract)
- L'implémentation technique des Tools

---

## 3. Principe fondamental

### 3.1 Bornage

> **MiyuWeb exécute des capacités gouvernées d'affichage web (rendu, résolution thème/layout, script, asset, formulaire, événement). Il ne décide jamais du contenu, ne prend jamais de décision ALLOW/DENY, et n'accède jamais à la base directement — les templates et assets sont fournis dans le flux.**

### 3.2 Ce que MiyuWeb ne fait jamais

| Code | Interdiction |
|------|--------------|
| **BOUND-1** | **Pas de décision ALLOW/DENY** — MiyuWeb ne décide pas si une action doit être faite (ALLOW/DENY = StrongFather). Il exécute uniquement ce qui a été autorisé. |
| **BOUND-2** | **Pas de choix de contenu** — MiyuWeb ne décide pas quel contenu afficher ; il rend, résout ou sert ce qui lui est fourni dans le flux (templates, données, assets). Il n'interprète pas le métier. |
| **BOUND-3** | **Pas d'accès direct à la base** — MiyuWeb ne lit pas la base (templates, assets). Il reçoit les données (contenu de template, contenu ou métadonnées d'assets) dans le flux gouverné, après lecture via MiyuSQL sous autorité KindMother ou transmission par un Opérateur. |
| **BOUND-4** | **Pas de modification du contexte d'autorisation** — MiyuWeb ne modifie pas les permissions, ne crée pas de mandat, ne révoque rien. Il utilise le contexte fourni. |
| **BOUND-5** | **Pas de connaissance de l'Opérateur appelant** — MiyuWeb ne connaît pas l'identité métier de l'Opérateur ; il reçoit un contexte gouverné (permissions, niveau, instance). |
| **BOUND-6** | **Pas de capacité nouvelle** — MiyuWeb n'ajoute aucune capacité qui n'existe pas dans ses Tools composants. Il orchestre, n'invente pas. |

---

## 4. Frontières avec les Cores

### 4.1 KindMother

| Frontière | Description |
|-----------|-------------|
| **Autorité sur les données** | KindMother est l'autorité sur toutes les données, dont templates et assets. MiyuWeb exécute des capacités (rendu, résolution thème/layout, script, asset, formulaire, événement) sur des données fournies dans le flux ; il ne lit pas la base. |
| **Limite** | MiyuWeb ne persiste pas, ne lit pas les templates ni les assets en base. Il n'est alimenté que par des données déjà lues via MiyuSQL sous autorité KindMother ou transmises dans le flux gouverné. |

### 4.2 StrongFather

| Frontière | Description |
|-----------|-------------|
| **Décision** | StrongFather décide ALLOW ou DENY. MiyuWeb n'est invoqué qu'en cas d'ALLOW. |
| **Limite** | MiyuWeb ne prend aucune décision stratégique. Il n'émet pas de mandat, ne révoque rien, ne confère aucune autorisation. |

### 4.3 Master Butler

| Frontière | Description |
|-----------|-------------|
| **Catalogue** | Master Butler déclare le Toolkit et les Tools. MiyuWeb n'enregistre pas lui-même les Tools ; il est déclaré par l'environnement. |
| **Limite** | MiyuWeb ne gère pas les permissions ni le catalogue. Il est invoqué après vérification Master Butler. |

### 4.4 WorrySentinel et Caring Nanny

| Frontière | Description |
|-----------|-------------|
| **Sécurité et état** | WorrySentinel (niveau de sécurité) et Caring Nanny (état système) sont vérifiés avant l'appel à MiyuWeb. |
| **Limite** | MiyuWeb ne modifie pas le niveau de sécurité ni l'état système. Il n'est invoqué que si les pré-conditions sont remplies. |

### 4.5 BondingBrother

| Frontière | Description |
|-----------|-------------|
| **Médiation** | BondingBrother traduit l'intention et prépare le contexte. MiyuWeb reçoit une demande déjà médiée (données, template, asset à rendre ou à servir). |
| **Limite** | MiyuWeb ne médie pas les intentions ; il exécute la capacité (render, resolve, execute, serve, validate, dispatch, capture) fournie dans le contexte gouverné. |

---

## 5. Invariants de limite

Les invariants MiyuWeb utilisent des préfixes catégoriels (BOUND = bornage). Pour le format canonique des invariants des Cores, voir [Miyukini Conceptual References - Standardisation Numération Invariants](../../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md).

| Code | Invariant |
|------|-----------|
| **INV-BOUND-1** | Aucun accès direct à la base (templates, assets) ; toutes les données sont fournies dans le flux gouverné |
| **INV-BOUND-2** | Aucune exécution sans passage par la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-BOUND-3** | Aucune décision ALLOW/DENY ou choix de contenu dans MiyuWeb ; exécution uniquement |
| **INV-BOUND-4** | Aucune interprétation métier du contenu ; MiyuWeb rend, résout ou sert ce qui est fourni |
| **INV-BOUND-5** | Le Toolkit n'expose que les capacités de ses Tools composants ; pas de capacité nouvelle |

---

## 6. Réponses aux violations

### 6.1 Comportement attendu

Si une condition de bornage est violée (ex. appel sans gouvernance, tentative d'accès direct à la base, décision de contenu), MiyuWeb ne doit pas exécuter. La réponse (rejet, erreur explicite) est gérée par la couche gouvernance (BondingBrother / StrongFather / KindMother), pas par MiyuWeb lui-même.

### 6.2 Traçabilité

Toute tentative d'appel hors bornage doit être tracée (observability, audit) selon les contrats KindMother et Caring Nanny ; MiyuWeb ne décide pas du contenu du trace, il peut fournir un signal d'échec au flux gouverné.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Tool Governance Compliance Contract | [MiyuWeb - Tool Governance Compliance Contract](../governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| KindMother - Runtime Boundary & Enforcement Contract | [KindMother - Runtime Boundary & Enforcement Contract](../../../core/KindMother/contracts/boundaries/KindMother%20-%20Runtime%20Boundary%20%26%20Enforcement%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Standardisation Numération Invariants | [Miyukini Conceptual References - Standardisation Numération Invariants](../../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
