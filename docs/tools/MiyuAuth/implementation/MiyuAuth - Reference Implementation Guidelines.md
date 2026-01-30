# MiyuAuth — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuAuth conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuAuth en logique d'implémentation (Tools, gouvernance, KindMother, bornage).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuAuth (Tools resolve, attest, verify, role) de manière conforme aux contrats : Documentation Fondatrice, KindMother Integration, Tool Governance Compliance, Security and States, Runtime Boundary, Dependencies, Unit Tests, Cycle Tests.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuAuth - Documentation Fondatrice** : Identité, ToolkitId, liste des Tools, gouvernance
- **MiyuAuth - KindMother Integration Contract** : Confiance validée par KindMother uniquement, exécution sans décision de confiance
- **MiyuAuth - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities
- **MiyuAuth - Security and States Contract** : Niveau 2 ou 3, états autorisés/interdits
- **MiyuAuth - Runtime Boundary Contract** : Bornage, interdictions (BOUND-*), invariants INV-BOUND-*
- **MiyuAuth - Dependencies Contract** : Liste fermée des dépendances, ordre d'invocation
- **Master Butler - Tool Governance Contract** et **Toolkit Composition Contract** : Format ToolId, structure Toolkit

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

**Principe contractuel :** MiyuAuth ne décide pas si une action doit être autorisée ; StrongFather décide ALLOW/DENY.

**Traduction en implémentation :**

- MiyuAuth est invoqué uniquement après décision ALLOW de la gouvernance. L'implémentation ne doit pas ré-évaluer les permissions.
- En cas d'appel hors gouvernance (anomalie), MiyuAuth doit refuser l'exécution et signaler (pas de décision de contournement).

### 2.2 Pas de confiance sans validation KindMother (BOUND-3)

**Principe contractuel :** Toute confiance utilisée pour l'identité est validée par KindMother.

**Traduction en implémentation :**

- L'implémentation des Tools (resolve, attest, verify, role) ne doit jamais utiliser une confiance non validée par KindMother.
- L'interface entre KindMother et MiyuAuth doit garantir que l'appel à MiyuAuth n'a lieu qu'avec un contexte ou des artefacts déjà validés (ou après validation KindMother dans le flux).

### 2.3 Résolution explicite du rôle (tool.identity.resolve, tool.identity.role)

**Principe contractuel :** MiyuAuth résout le contexte d'identité et détermine le rôle (citoyen, visiteur, externe) sans décider de l'autorisation.

**Traduction en implémentation :**

- Les Tools resolve et role retournent un résultat structuré (contexte, rôle) ; ils ne retournent pas ALLOW/DENY.
- Alignement Connexion Inter-COG : Passeport Utilisateur, Visa de Connexion, COG Origine, COG Hébergeur, Utilisateur Visiteur, Utilisateur Externe — terminologie et concepts respectés dans les structures de données.

### 2.4 Liste fermée des dépendances (INV-DEP-*)

**Principe contractuel :** MiyuAuth ne dépend que des Cores et du Kernel définis dans le Dependencies Contract.

**Traduction en implémentation :**

- Aucune dépendance vers un Opérateur, un produit, ou une règle métier.
- Les appels entrants passent par BondingBrother et la chaîne de gouvernance ; MiyuAuth n'expose pas d'API publique directe aux Opérateurs.
- Usage du Kernel (Id, Logger, Clock, Config, Lifecycle) pour identifiants, logs, horodatage, configuration locale, cycle de vie — sans logique métier.

### 2.5 Alignement MIP/MSCM

**Principe contractuel :** Domaine `identity`, layer Strate 6 ; chaque Tool = bloc logique (id, do, role, layer).

**Traduction en implémentation :**

- À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le Protocole MIP v1.

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Pas de code qui évalue ALLOW/DENY ; exécution uniquement sur mandat |
| **BOUND-2** | Pas d'autorisation métier | Pas de code qui crée mandat, révocation, modification des permissions |
| **BOUND-3** | Pas de confiance sans validation KindMother | Pas de code qui utilise une confiance non validée par KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte ; pas de révocation, pas de création de Visa/Passeport |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Pas d'identité Opérateur dans la logique Tool ; contexte anonymisé (permissions, niveau) |
| **BOUND-6** | Pas de capacité nouvelle | Chaque Tool correspond exactement à un ToolId déclaré ; pas d'extension non déclarée |

---

## 4. Patterns recommandés

### 4.1 Structure des Tools

- Chaque ToolId est implémenté comme une unité d'exécution atomique : entrée (contexte gouverné, paramètres), sortie (résultat ou erreur contractuelle).
- Pas d'état métier partagé entre appels ; état technique sous contrôle du flux gouverné.

### 4.2 Interface avec KindMother

- L'implémentation MiyuAuth consomme un contexte ou des artefacts déjà validés par KindMother (ou reçoit un mandat d'exécution après validation KindMother). Les paramètres incluent : type d'opération (resolve, attest, verify, role), paramètres (contexte, Passeport/Visa mock ou validé), contexte gouverné (sans identité Opérateur métier).
- La réponse inclut : succès/échec, résultat (contexte résolu, attestation, résultat vérification, rôle), ou erreur explicite (sans fuite d'information sensible).

### 4.3 Gestion des erreurs

- Les erreurs techniques (format invalide, signature, expiration) sont remontées de manière explicite sans exposer de données métier.
- En cas de violation de bornage (ex. appel sans mandat, confiance non validée), refus d'exécution et signal (observability) ; pas d'exécution partielle.

### 4.4 Traçabilité

- Utiliser le Logger du Kernel pour tracer les exécutions (sans contenu métier sensible). Conformité aux contrats KindMother Observability et MiyuAuth Runtime Boundary.

---

## 5. Alignement MIP / MSCM

### 5.1 MIP v1

À l'implémentation, le code fournissant les Tools MiyuAuth doit être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

- **Domaine** : `identity` (cohérent avec domains.json).
- **Layer** : Strate 6 (outil / toolkit) dans layers.json.
- **Blocs** : Chaque Tool MiyuAuth est une unité logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json.

### 5.2 MSCM

Les blocs de code correspondant aux Tools doivent être balisés selon le standard MSCM (Miyukini Semantic Code Markup) pour permettre l'indexation et la gouvernance structurelle.

### 5.3 Numérotation des invariants

Les contrats MiyuAuth utilisent des **préfixes catégoriels** pour les invariants : BOUND (bornage), DEP (dépendances), SEC (sécurité), INV-KM-* (intégration KindMother), INV-UT-MAUTH-* (tests unitaires), INV-CT-MAUTH-* (tests de cycle). Le format canonique des invariants des Cores (INV-&lt;PREFIX&gt;-&lt;NUMERO&gt;) est défini dans [Miyukini Conceptual References - Standardisation Numération Invariants](../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) ; les préfixes MiyuAuth restent cohérents en interne et distincts des codes Cores (KM, SF, etc.).

---

## 6. Tests (rappel)

- **Tests unitaires** : Conformément au [MiyuAuth - Unit Tests Contract](../contracts/testing/MiyuAuth%20-%20Unit%20Tests%20Contract.md) — pas de modification de données métier ; mocks ou sandbox pour attest/verify.
- **Test de cycle MiyuAuth** : Conformément au [MiyuAuth - Cycle Tests Contract](../contracts/testing/MiyuAuth%20-%20Cycle%20Tests%20Contract.md) — scénario E2E (résolution → rôle → vérification Passeport/Visa dans un scénario gouverné). Exécutable par MiyukiniAdmin.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../contracts/integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| MiyuAuth - Runtime Boundary Contract | [MiyuAuth - Runtime Boundary Contract](../contracts/boundaries/MiyuAuth%20-%20Runtime%20Boundary%20Contract.md) |
| MiyuAuth - Security and States Contract | [MiyuAuth - Security and States Contract](../contracts/security/MiyuAuth%20-%20Security%20and%20States%20Contract.md) |
| MiyuAuth - Dependencies Contract | [MiyuAuth - Dependencies Contract](../dependencies/MiyuAuth%20-%20Dependencies%20Contract.md) |
| MiyuAuth - Unit Tests Contract | [MiyuAuth - Unit Tests Contract](../contracts/testing/MiyuAuth%20-%20Unit%20Tests%20Contract.md) |
| MiyuAuth - Cycle Tests Contract | [MiyuAuth - Cycle Tests Contract](../contracts/testing/MiyuAuth%20-%20Cycle%20Tests%20Contract.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |
| Standardisation Numération Invariants | [Miyukini Conceptual References - Standardisation Numération Invariants](../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
