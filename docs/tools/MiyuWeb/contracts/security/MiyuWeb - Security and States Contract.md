# MiyuWeb — Security and States Contract

## 1. Contexte

Ce document définit le contrat de **sécurité et d'états système** pour le kit MiyuWeb. Il établit le niveau de sécurité du Toolkit, les états autorisés et interdits pour son utilisation, et l'alignement avec WorrySentinel et Caring Nanny. Il mentionne les risques spécifiques au domaine web (XSS, CSP) dans le périmètre d'exposition du Toolkit.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Le niveau de sécurité du kit MiyuWeb (niveau 0, 1 ou 2 selon politique d'exposition)
- Les états système autorisés (HEALTHY, DEGRADED)
- Les états système interdits (SECURITY_LOCKDOWN, MAINTENANCE, etc.)
- L'alignement avec WorrySentinel (niveaux de sécurité) et Caring Nanny (états de confiance)
- Les contraintes de sécurité applicables au domaine web (sanitization, CSP)

Ce document **ne couvre pas** :
- Le modèle de menace détaillé (voir contrats KindMother / MiyukiniAdmin si pertinent)
- Les permissions par Opérateur (voir Master Butler - Permission Registry)
- Les tests de sécurité (voir MiyuWeb - Unit Tests Contract / Cycle Tests Contract)

---

## 3. Niveau de sécurité

### 3.1 Niveau du Toolkit

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit MiyuWeb** | **0, 1 ou 2** selon politique d'exposition |
| **Justification** | Les Tools MiyuWeb manipulent l'affichage de contenu web (rendu HTML, scripts, assets, formulaires, événements). Le niveau dépend de l'exposition : contenu public (0), standard (1), contenu sensible ou contrôlé (2). Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. Cohérent avec WorrySentinel. |

### 3.2 Alignement WorrySentinel

| Niveau | Nom | Description | MiyuWeb |
|--------|-----|-------------|----------|
| **0** | **Public** | **Données publiques, exposition faible** | **Niveau nominal possible (façade publique)** |
| **1** | **Standard** | **Données standard, contraintes de base** | **Niveau nominal possible** |
| **2** | **Sensitive** | **Données sensibles, contraintes renforcées** | **Niveau nominal possible selon politique** |
| 3 | Critical | Données critiques | Non applicable au kit par défaut |
| 4 | Highest | Sécurité maximale | Non applicable au kit par défaut |

### 3.3 Invariants sécurité

| Code | Invariant |
|------|-----------|
| **INV-SEC-1** | Aucun appel à un Tool MiyuWeb n'est autorisé si le niveau de sécurité actuel (WorrySentinel) est inférieur au niveau requis (0, 1 ou 2 selon politique d'exposition) pour les opérations web |
| **INV-SEC-2** | Le niveau du Toolkit ne peut pas être abaissé sans révision contractuelle |
| **INV-SEC-3** | Les opérations de rendu, script, asset, formulaire et événement sont assujetties au niveau de sécurité défini pour le Toolkit |

---

## 4. Risques web (XSS, CSP)

### 4.1 Périmètre contractuel

MiyuWeb opère sur des **données fournies dans le flux** (templates, contenu, assets). Les risques suivants relèvent du périmètre d'exposition du Toolkit et doivent être pris en compte par l'implémentation et la gouvernance :

| Risque | Description | Contrainte contractuelle |
|--------|-------------|---------------------------|
| **XSS (Cross-Site Scripting)** | Injection de script malveillant dans le contenu rendu | Tout contenu destiné au rendu HTML ou à l'exécution de script doit être traité selon la politique de sanitization définie par l'environnement ; MiyuWeb n'introduit pas de contenu non gouverné |
| **CSP (Content Security Policy)** | Politique de sécurité du contenu (sources autorisées, inline, eval) | L'implémentation des Tools MiyuWeb (notamment `tool.web.script.execute`, `tool.web.html.render`) doit être compatible avec les directives CSP définies par WorrySentinel / environnement ; MiyuWeb ne contourne pas la CSP |

### 4.2 Règle

> **MiyuWeb exécute le rendu et les scripts sur des données fournies dans le flux ; la responsabilité de la sanitization et du respect de la CSP incombe à l'implémentation et à la gouvernance (WorrySentinel, politique d'exposition), pas à la décision de contenu par MiyuWeb.**

---

## 5. États système autorisés

### 5.1 États autorisés

| État | Description | Usage MiyuWeb |
|------|-------------|----------------|
| **HEALTHY** | Système sain, toutes capacités disponibles | Tous les Tools MiyuWeb sont utilisables |
| **DEGRADED** | Incohérence persistante, capacités réduites | Utilisation selon politique Caring Nanny (ex. rendu prioritaire, scripts restreints ou désactivés) |

### 5.2 Alignement Caring Nanny (États de confiance)

| État confiance | Nom | Description | MiyuWeb |
|----------------|-----|-------------|----------|
| **T0** | Normal | Système sain | Autorisé |
| **T1** | Instable | Anomalie détectée | Autorisé avec surveillance |
| **T2** | Dégradé | Capacités réduites | Autorisé selon politique |
| T3 | Restreint | Suspicion forte | Interdit ou très restreint |
| T4 | Bloqué | Intégrité rompue | Interdit |

---

## 6. États système interdits

### 6.1 États interdits

| État | Description | Effet |
|------|-------------|-------|
| **SECURITY_LOCKDOWN** | Verrouillage sécurité | Aucun Tool MiyuWeb d'exécution de script ou de rendu non sécurisé ; rendu restreint selon politique |
| **MAINTENANCE** | Maintenance en cours | Aucun Tool MiyuWeb utilisable (ou rendu seul selon politique) |
| **Autres** | Selon [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) | Les états disallowed_states du Toolkit s'appliquent |

### 6.2 Règle d'exécution

> **Aucun Tool MiyuWeb ne doit être exécuté si l'état système (Caring Nanny) est dans la liste des états interdits pour le Toolkit.**

---

## 7. Contraintes d'utilisation

### 7.1 Pré-conditions à l'appel

Avant toute exécution d'un Tool MiyuWeb :
1. WorrySentinel : le niveau de sécurité actuel est au moins égal au niveau requis (0, 1 ou 2 selon politique).
2. Caring Nanny : l'état système est dans la liste des états autorisés (HEALTHY ou DEGRADED selon politique).
3. StrongFather : la décision est ALLOW pour l'opération demandée.
4. KindMother : les données (templates, assets) fournies dans le flux proviennent d'un chemin gouverné ; MiyuWeb ne lit pas la base directement (voir [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md)).

### 7.2 Refus d'exécution

Si l'une des pré-conditions n'est pas remplie, l'exécution est refusée ; le Tool MiyuWeb ne doit pas être invoqué. La réponse est gérée par la gouvernance (BondingBrother / StrongFather), pas par MiyuWeb lui-même.

---

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Tool Governance Compliance Contract | [MiyuWeb - Tool Governance Compliance Contract](../governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire — États de confiance, Niveaux de sécurité | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Security Levels (référence conceptuelle) | [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
