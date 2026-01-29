# MiyuSQL — Security and States Contract

## 1. Contexte

Ce document definit le contrat de **securite et d'etats systeme** pour le kit MiyuSQL. Il etablit le niveau de securite du Toolkit, les etats autorises et interdits pour son utilisation, et l'alignement avec WorrySentinel et Caring Nanny.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portee / Scope

Ce document definit :
- Le niveau de securite du kit MiyuSQL (niveau 2)
- Les etats systeme autorises (HEALTHY, DEGRADED)
- Les etats systeme interdits (SECURITY_LOCKDOWN, MAINTENANCE, etc.)
- L'alignement avec WorrySentinel (niveaux de securite) et Caring Nanny (etats de confiance)

Ce document **ne couvre pas** :
- Le modele de menace detaille (voir contrats KindMother / MiyukiniAdmin si pertinent)
- Les permissions par Operateur (voir Master Butler - Permission Registry)
- Les tests de securite (voir MiyuSQL - Unit Tests Contract / Cycle Tests Contract)

---

## 3. Niveau de Securite

### 3.1 Niveau du Toolkit

| Element | Valeur |
|---------|--------|
| **Niveau de securite du kit MiyuSQL** | **2** (donnees utilisateur) |
| **Justification** | Les Tools MiyuSQL manipulent des donnees en base ; le niveau est coherent avec WorrySentinel (Data Tools = 2). Le niveau du Toolkit est au moins egal au maximum des niveaux de ses Tools composants. |

### 3.2 Alignement WorrySentinel

| Niveau | Nom | Description | MiyuSQL |
|--------|-----|-------------|---------|
| 0 | Public | Donnees publiques | Non applicable |
| 1 | Standard | Donnees standard | Tools lecture seule peuvent etre autorises en 1 selon politique |
| **2** | **Sensitive** | **Donnees utilisateur** | **Niveau nominal du kit** |
| 3 | Critical | Donnees critiques | Non applicable au kit par defaut |
| 4 | Highest | Securite maximale | Non applicable |

### 3.3 Invariants Securite

| Code | Invariant |
|------|-----------|
| **INV-SEC-1** | Aucun appel a un Tool MiyuSQL n'est autorise si le niveau de securite actuel (WorrySentinel) est inferieur au niveau requis (2) pour les operations d'ecriture |
| **INV-SEC-2** | Les operations de lecture seule (tool.schema.read, SELECT via tool.query.execute) peuvent etre assujetties a une politique moins stricte selon l'environnement ; le contrat par defaut reste niveau 2 |
| **INV-SEC-3** | Le niveau du Toolkit ne peut pas etre abaisse sans revision contractuelle |

---

## 4. Etats Systeme Autorisés

### 4.1 Etats Autorises

| Etat | Description | Usage MiyuSQL |
|------|-------------|---------------|
| **HEALTHY** | Systeme sain, toutes capacites disponibles | Tous les Tools MiyuSQL sont utilisables |
| **DEGRADED** | Incoherence persistante, capacites reduites | Utilisation selon politique Caring Nanny (ex. lecture prioritaire, ecriture restreinte ou interdite) |

### 4.2 Alignement Caring Nanny (Etats de Confiance)

| Etat confiance | Nom | Description | MiyuSQL |
|----------------|-----|-------------|---------|
| **T0** | Normal | Systeme sain | Autorisé |
| **T1** | Instable | Anomalie detectee | Autorisé avec surveillance |
| **T2** | Dégradé | Capacites reduites | Autorisé selon politique (lecture prioritaire) |
| T3 | Restreint | Suspicion forte | Interdit ou tres restreint |
| T4 | Bloqué | Integrite rompue | Interdit |

---

## 5. Etats Systeme Interdits

### 5.1 Etats Interdits

| Etat | Description | Effet |
|------|-------------|-------|
| **SECURITY_LOCKDOWN** | Verrouillage securite | Aucun Tool MiyuSQL d'ecriture ; lecture eventuellement restreinte |
| **MAINTENANCE** | Maintenance en cours | Aucun Tool MiyuSQL utilisable (ou lecture seule selon politique) |
| **Autres** | Selon [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) | Les etats disallowed_states du Toolkit s'appliquent |

### 5.2 Regle d'Execution

> **Aucun Tool MiyuSQL ne doit etre execute si l'etat systeme (Caring Nanny) est dans la liste des etats interdits pour le Toolkit.**

---

## 6. Contraintes d'Utilisation

### 6.1 Pre-conditions a l'Appel

Avant toute execution d'un Tool MiyuSQL :
1. WorrySentinel : le niveau de securite actuel est au moins egal au niveau requis (2 pour ecriture).
2. Caring Nanny : l'etat systeme est dans la liste des etats autorises (HEALTHY ou DEGRADED selon politique).
3. StrongFather : la decision est ALLOW pour l'operation demandee.
4. KindMother : pour toute ecriture, une WriteIntent a ete acceptee.

### 6.2 Refus d'Execution

Si l'une des pre-conditions n'est pas remplie, l'execution est refusee ; le Tool MiyuSQL ne doit pas etre invoque. La reponse est geree par la gouvernance (BondingBrother / StrongFather), pas par MiyuSQL lui-meme.

---

## 7. References Croisees

| Document | Lien |
|----------|------|
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../../MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - Tool Governance Compliance Contract | [MiyuSQL - Tool Governance Compliance Contract](../governance/MiyuSQL%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire — Etats de confiance, Niveaux de securite | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de creation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Contrat de reference
