# MiyuAuth — Dependencies Contract

## 1. Contexte

Ce document définit le contrat des **dépendances** du kit MiyuAuth. Il établit la liste fermée des dépendances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Kernel), l'absence de dépendance métier, et l'ordre ou les contraintes d'utilisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- La liste fermée des dépendances de MiyuAuth (Cores et Kernel)
- L'absence de dépendance métier (Opérateurs, produits, règles métier)
- L'ordre et les contraintes (flux d'appel, pré-conditions)
- Les invariants de dépendance

Ce document **ne couvre pas** :
- Les dépendances d'implémentation (stockage identité, signatures, librairies techniques) — hors scope documentaire fondateur
- Les dépendances des Cores eux-mêmes

---

## 3. Principe fondamental

### 3.1 Liste fermée

> **MiyuAuth ne dépend que des Cores et du Kernel définis dans ce contrat. Aucune dépendance métier (Opérateur, produit, règle métier) n'est autorisée.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-DEP-1** | MiyuAuth ne connaît pas les Opérateurs ; il reçoit un contexte gouverné (BondingBrother) |
| **INV-DEP-2** | MiyuAuth ne dépend d'aucun produit ni règle métier applicative |
| **INV-DEP-3** | Toute invocation de MiyuAuth passe par la médiation BondingBrother et la gouvernance (Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-DEP-4** | Toute utilisation de confiance inter-domaines pour l'identité est validée par KindMother uniquement |
| **INV-DEP-5** | MiyuAuth n'est invoqué qu'après décision ALLOW de StrongFather |

---

## 4. Liste fermée des dépendances

### 4.1 Cores (Strate 4)

| Dépendance | Rôle pour MiyuAuth | Contrainte |
|------------|---------------------|------------|
| **KindMother** | Validateur unique de la confiance inter-domaines ; toute confiance utilisée pour l'identité est validée par KindMother | MiyuAuth n'utilise aucune confiance non validée par KindMother ; pas de délégation de validation |
| **Master Butler** | Catalogue des Tools et Toolkits ; permissions ; déclaration de MiyuAuth et des ToolIds | MiyuAuth est invoqué après vérification Master Butler |
| **StrongFather** | Décision ALLOW/DENY pour l'utilisation des Tools | MiyuAuth n'est invoqué qu'en cas d'ALLOW |
| **WorrySentinel** | Niveau de sécurité ; vérification que le niveau actuel permet l'appel | Pré-condition à l'invocation |
| **Caring Nanny** | État système ; vérification que l'état (HEALTHY, DEGRADED, etc.) permet l'appel | Pré-condition à l'invocation |

### 4.2 Interface & Médiation (Strate 5)

| Dépendance | Rôle pour MiyuAuth | Contrainte |
|------------|---------------------|------------|
| **BondingBrother** | Médiation ; traduction de l'intention ; préparation du contexte ; passage des demandes vers les Cores | MiyuAuth reçoit les demandes via BondingBrother (ou via le flux gouverné initié par BondingBrother) |

### 4.3 Kernel (Strate K)

| Dépendance | Rôle pour MiyuAuth | Contrainte |
|------------|---------------------|------------|
| **Kernel** | Id (identifiants), Logger (traçabilité), Clock (horodatage), Config (configuration locale), Lifecycle | Usage minimal et neutre ; pas de logique métier ; conformité aux invariants Kernel |

---

## 5. Ordre et contraintes

### 5.1 Flux d'invocation (ordre)

L'ordre d'implication des dépendances lors d'un appel à un Tool MiyuAuth est :

1. **Opérateur** (hors dépendance MiyuAuth) émet une intention.
2. **BondingBrother** — médiation, traduction, contexte.
3. **Master Butler** — vérification Tool/Toolkit, permissions.
4. **WorrySentinel** — niveau de sécurité.
5. **Caring Nanny** — état système.
6. **StrongFather** — décision ALLOW/DENY.
7. Si ALLOW et si confiance inter-domaines nécessaire : **KindMother** — validation de la confiance.
8. **MiyuAuth** — exécution du Tool mandaté (resolve, attest, verify, role).

### 5.2 Contraintes

| Contrainte | Description |
|------------|-------------|
| **Pas d'invocation directe** | MiyuAuth n'est jamais invoqué directement par un Opérateur ; toujours via BondingBrother et la chaîne de gouvernance |
| **Pas de bypass** | Aucune dépendance ne peut être contournée (pas d'utilisation de confiance sans KindMother, pas d'exécution sans StrongFather ALLOW) |
| **Pas de dépendance inverse métier** | Aucun Core ni Kernel ne dépend de MiyuAuth pour sa logique métier ; MiyuAuth est un outil consommé par le flux |

---

## 6. Absence de dépendance métier

### 6.1 Ce dont MiyuAuth ne dépend pas

| Type | Exemples | Raison |
|------|----------|--------|
| **Opérateurs** | MiyukiniAdmin, tout Opérateur de domaine | MiyuAuth est un Toolkit ; les Opérateurs utilisent MiyuAuth via la gouvernance |
| **Produits / Règles métier** | Schémas applicatifs, règles métier | MiyuAuth n'interprète pas le métier ; il exécute des capacités mandatees |
| **Autres Toolkits** | MiyuSQL, kits d'outils métier | MiyuAuth est indépendant des autres Toolkits ; pas de couplage fonctionnel direct. La **persistance** des données d'identification (Passeport, Visa) est exécutée via **MiyuSQL** sous autorité **KindMother** ; MiyuAuth ne lit ni n'écrit en base — il opère sur des données fournies dans le flux (voir [MiyuAuth - Documentation Fondatrice](../MiyuAuth%20-%20Documentation%20Fondatrice.md) section 8bis). |
| **Services externes** | APIs externes, réseau métier | Conformité LOI-1 ; pas de dépendance externe critique |

### 6.2 Dépendances techniques (hors scope contractuel)

Les dépendances techniques (stockage identité, signatures, librairies) sont hors scope de ce contrat fondateur. Elles seront définies dans le guide d'implémentation (MiyuAuth - Reference Implementation Guidelines) et doivent rester neutres (pas de logique métier).

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../contracts/integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| MiyuAuth - Tool Governance Compliance Contract | [MiyuAuth - Tool Governance Compliance Contract](../contracts/governance/MiyuAuth%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| MiyuAuth - Runtime Boundary Contract | [MiyuAuth - Runtime Boundary Contract](../contracts/boundaries/MiyuAuth%20-%20Runtime%20Boundary%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
