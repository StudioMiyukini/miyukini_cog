# MiyuClock — Security and States Contract

## 1. Contexte

Ce document définit le contrat de **sécurité et d'états système** pour le kit MiyuClock. Il établit le niveau de sécurité du Toolkit, les états autorisés et interdits pour son utilisation, et l'alignement avec WorrySentinel et Caring Nanny.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Le niveau de sécurité du kit MiyuClock (niveau 0 ou 1)
- Les états système autorisés (HEALTHY, DEGRADED)
- Les états système interdits (SECURITY_LOCKDOWN, MAINTENANCE, etc.)
- L'alignement avec WorrySentinel (niveaux de sécurité) et Caring Nanny (états de confiance)

Ce document **ne couvre pas** :
- Le modèle de menace détaillé (voir contrats KindMother / MiyukiniAdmin si pertinent)
- Les permissions par Opérateur (voir Master Butler - Permission Registry)
- Les tests de sécurité (voir MiyuClock - Unit Tests Contract / Cycle Tests Contract)

---

## 3. Niveau de sécurité

### 3.1 Niveau du Toolkit

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit MiyuClock** | **0 ou 1** (données publiques / standard) |
| **Justification** | Les Tools MiyuClock fournissent uniquement la mesure du temps (instant présent, delta) ; aucune donnée sensible ni identité. Le niveau est cohérent avec WorrySentinel (time measurement = 0 ou 1 selon politique). Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |

### 3.2 Alignement WorrySentinel

| Niveau | Nom | Description | MiyuClock |
|--------|-----|-------------|----------|
| **0** | **Public** | **Données publiques** | **Niveau nominal possible** |
| **1** | **Standard** | **Données standard** | **Niveau nominal possible selon politique** |
| 2 | Sensitive | Données sensibles | Non applicable au kit par défaut |
| 3 | Critical | Données critiques | Non applicable au kit par défaut |
| 4 | Highest | Sécurité maximale | Non applicable au kit par défaut |

### 3.3 Invariants sécurité

| Code | Invariant |
|------|-----------|
| **INV-SEC-1** | Aucun appel à un Tool MiyuClock n'est autorisé si le niveau de sécurité actuel (WorrySentinel) est inférieur au niveau requis (0 ou 1) pour les opérations de mesure du temps |
| **INV-SEC-2** | Le niveau du Toolkit ne peut pas être abaissé sans révision contractuelle |
| **INV-SEC-3** | Les opérations de mesure du temps (now, delta) sont assujetties au niveau de sécurité défini pour le Toolkit |

---

## 4. États système autorisés

### 4.1 États autorisés

| État | Description | Usage MiyuClock |
|------|-------------|----------------|
| **HEALTHY** | Système sain, toutes capacités disponibles | Tous les Tools MiyuClock sont utilisables |
| **DEGRADED** | Incohérence persistante, capacités réduites | Utilisation selon politique Caring Nanny (ex. now prioritaire, delta restreint ou autorisé) |

### 4.2 Alignement Caring Nanny (États de confiance)

| État confiance | Nom | Description | MiyuClock |
|----------------|-----|-------------|----------|
| **T0** | Normal | Système sain | Autorisé |
| **T1** | Instable | Anomalie détectée | Autorisé avec surveillance |
| **T2** | Dégradé | Capacités réduites | Autorisé selon politique |
| T3 | Restreint | Suspicion forte | Interdit ou très restreint |
| T4 | Bloqué | Intégrité rompue | Interdit |

---

## 5. États système interdits

### 5.1 États interdits

| État | Description | Effet |
|------|-------------|-------|
| **SECURITY_LOCKDOWN** | Verrouillage sécurité | Aucun Tool MiyuClock utilisable (ou now seul selon politique) |
| **MAINTENANCE** | Maintenance en cours | Aucun Tool MiyuClock utilisable |
| **Autres** | Selon [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) | Les états disallowed_states du Toolkit s'appliquent |

### 5.2 Règle d'exécution

> **Aucun Tool MiyuClock ne doit être exécuté si l'état système (Caring Nanny) est dans la liste des états interdits pour le Toolkit.**

---

## 6. Contraintes d'utilisation

### 6.1 Pré-conditions à l'appel

Avant toute exécution d'un Tool MiyuClock :
1. WorrySentinel : le niveau de sécurité actuel est au moins égal au niveau requis (0 ou 1 selon politique).
2. Caring Nanny : l'état système est dans la liste des états autorisés (HEALTHY ou DEGRADED selon politique).
3. StrongFather : la décision est ALLOW pour l'opération demandée.

### 6.2 Refus d'exécution

Si l'une des pré-conditions n'est pas remplie, l'exécution est refusée ; le Tool MiyuClock ne doit pas être invoqué. La réponse est gérée par la gouvernance (BondingBrother / StrongFather), pas par MiyuClock lui-même.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Tool Governance Compliance Contract | [MiyuClock - Tool Governance Compliance Contract](../governance/MiyuClock%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire — États de confiance, Niveaux de sécurité | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
