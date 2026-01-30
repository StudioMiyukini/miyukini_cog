# MiyuAuth — Runtime Boundary Contract

## 1. Contexte

Ce document définit le **bornage (frontières d'exécution)** du kit MiyuAuth. Il établit ce que MiyuAuth ne fait jamais, les frontières avec les Cores, et les invariants de limite. MiyuAuth est un Kit d'Outils qui orchestre des capacités atomiques d'identité sans décision de confiance ni d'autorisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Ce que MiyuAuth ne fait jamais (pas de décision ALLOW/DENY, pas d'autorisation métier, pas de confiance sans validation KindMother, pas de capacité hors Tools composants)
- Les frontières avec les Cores (KindMother, StrongFather, Master Butler, WorrySentinel, Caring Nanny, BondingBrother)
- Les invariants de limite (bornage)

Ce document **ne couvre pas** :
- Les frontières internes de KindMother (voir KindMother - Identity & Cross-Domain Trust Contract)
- L'implémentation technique des Tools

---

## 3. Principe fondamental

### 3.1 Bornage

> **MiyuAuth exécute des capacités gouvernées d'identité (résolution, attestation, vérification, rôle). Il ne décide jamais de la confiance, ne prend jamais de décision ALLOW/DENY, et n'utilise jamais de confiance non validée par KindMother.**

### 3.2 Ce que MiyuAuth ne fait jamais

| Code | Interdiction |
|------|--------------|
| **BOUND-1** | **Pas de décision ALLOW/DENY** — MiyuAuth ne décide pas si une action doit être faite (ALLOW/DENY = StrongFather). Il exécute uniquement ce qui a été autorisé. |
| **BOUND-2** | **Pas d'autorisation métier** — MiyuAuth ne confère aucune autorisation ; l'autorisation reste à StrongFather et au COG Hébergeur. Il ne traite pas la reconnaissance d'identité comme une autorisation. |
| **BOUND-3** | **Pas de confiance sans validation KindMother** — MiyuAuth n'utilise aucune confiance inter-domaines non validée par KindMother. Toute confiance utilisée pour l'identité est validée par KindMother (Identity & Cross-Domain Trust). |
| **BOUND-4** | **Pas de modification du contexte d'autorisation** — MiyuAuth ne modifie pas les permissions, ne crée pas de mandat, ne révoque rien. Il utilise le contexte fourni. |
| **BOUND-5** | **Pas de connaissance de l'Opérateur appelant** — MiyuAuth ne connaît pas l'identité métier de l'Opérateur ; il reçoit un contexte gouverné (permissions, niveau, instance). |
| **BOUND-6** | **Pas de capacité nouvelle** — MiyuAuth n'ajoute aucune capacité qui n'existe pas dans ses Tools composants. Il orchestre, n'invente pas. |

---

## 4. Frontières avec les Cores

### 4.1 KindMother

| Frontière | Description |
|-----------|-------------|
| **Validation de confiance** | KindMother est l'unique validateur de la confiance inter-domaines. MiyuAuth exécute des capacités (resolve, attest, verify, role) sans décider de la confiance ; toute confiance utilisée pour l'identité est validée par KindMother. |
| **Limite** | MiyuAuth ne valide pas la confiance, ne délègue pas la validation. Il exécute les capacités mandatees après validation KindMother. |

### 4.2 StrongFather

| Frontière | Description |
|-----------|-------------|
| **Décision** | StrongFather décide ALLOW ou DENY. MiyuAuth n'est invoqué qu'en cas d'ALLOW. |
| **Limite** | MiyuAuth ne prend aucune décision stratégique. Il n'émet pas de mandat, ne révoque rien, ne confère aucune autorisation. |

### 4.3 Master Butler

| Frontière | Description |
|-----------|-------------|
| **Catalogue** | Master Butler déclare le Toolkit et les Tools. MiyuAuth n'enregistre pas lui-même les Tools ; il est déclaré par l'environnement. |
| **Limite** | MiyuAuth ne gère pas les permissions ni le catalogue. Il est invoqué après vérification Master Butler. |

### 4.4 WorrySentinel et Caring Nanny

| Frontière | Description |
|-----------|-------------|
| **Sécurité et état** | WorrySentinel (niveau de sécurité) et Caring Nanny (état système) sont vérifiés avant l'appel à MiyuAuth. |
| **Limite** | MiyuAuth ne modifie pas le niveau de sécurité ni l'état système. Il n'est invoqué que si les pré-conditions sont remplies. |

### 4.5 BondingBrother

| Frontière | Description |
|-----------|-------------|
| **Médiation** | BondingBrother traduit l'intention et prépare le contexte. MiyuAuth reçoit une demande déjà médiée. |
| **Limite** | MiyuAuth ne médie pas les intentions ; il exécute la capacité (resolve, attest, verify, role) fournie dans le contexte gouverné. |

---

## 5. Invariants de limite

Les invariants MiyuAuth utilisent des préfixes catégoriels (BOUND = bornage). Pour le format canonique des invariants des Cores, voir [Miyukini Conceptual References - Standardisation Numération Invariants](../../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md).

| Code | Invariant |
|------|-----------|
| **INV-BOUND-1** | Aucune utilisation de confiance inter-domaines sans validation KindMother |
| **INV-BOUND-2** | Aucune exécution sans passage par la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-BOUND-3** | Aucune décision ALLOW/DENY ou autorisation métier dans MiyuAuth ; exécution uniquement |
| **INV-BOUND-4** | Aucune reconnaissance d'identité traitée comme autorisation ; identité ≠ autorisation |
| **INV-BOUND-5** | Le Toolkit n'expose que les capacités de ses Tools composants ; pas de capacité nouvelle |

---

## 6. Réponses aux violations

### 6.1 Comportement attendu

Si une condition de bornage est violée (ex. appel sans gouvernance, utilisation de confiance non validée par KindMother), MiyuAuth ne doit pas exécuter. La réponse (rejet, erreur explicite) est gérée par la couche gouvernance (BondingBrother / StrongFather / KindMother), pas par MiyuAuth lui-même.

### 6.2 Traçabilité

Toute tentative d'appel hors bornage doit être tracée (observability, audit) selon les contrats KindMother et Caring Nanny ; MiyuAuth ne décide pas du contenu du trace, il peut fournir un signal d'échec au flux gouverné.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| MiyuAuth - Tool Governance Compliance Contract | [MiyuAuth - Tool Governance Compliance Contract](../governance/MiyuAuth%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| KindMother - Identity & Cross-Domain Trust Contract | [KindMother - Identity & Cross-Domain Trust Contract](../../../core/KindMother/contracts/authority/KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Standardisation Numération Invariants | [Miyukini Conceptual References - Standardisation Numération Invariants](../../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
