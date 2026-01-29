# MiyuSQL — Tool Governance Compliance Contract

## 1. Contexte

Ce document definit la **conformite de MiyuSQL** aux contrats de gouvernance des Outils et Kits d'Outils de Master Butler. MiyuSQL est un **Kit d'Outils (Toolkit)** declare et gouverne par l'environnement ; ce contrat etablit la declaration formelle du ToolkitId, des ToolIds composants, et des capabilities associees.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portee / Scope

Ce document definit :
- La conformite au [Master Butler - Tool Governance Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)
- La conformite au [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)
- La declaration formelle du ToolkitId et des ToolIds MiyuSQL
- Le catalogue des capabilities exposees

Ce document **ne couvre pas** :
- L'implementation technique des Tools
- Les contrats MiyuSQL hors gouvernance (integration KindMother, securite, bornage)

---

## 3. Conformite au Tool Governance Contract

### 3.1 Principes Respectes

| Principe Master Butler | Application MiyuSQL |
|------------------------|---------------------|
| Tout Tool possede un ToolId unique et immuable | Chaque outil MiyuSQL a un ToolId au format `tool.<domain>.<action>` |
| Tout Tool est lie a exactement une Capability | Chaque ToolId est associe a un capability_id (voir section 5) |
| Un Tool ne prend jamais de decision metier | Les Tools MiyuSQL executent uniquement ; pas de logique metier |
| Un Tool ne connait jamais l'Operateur appelant | MiyuSQL reçoit un contexte gouverné ; pas d'identite Operateur dans la logique Tool |

### 3.2 Format ToolId

Format canonique : `tool.<domain>.<action>[.<qualifier>]`

- **Prefixe :** `tool.`
- **Domaine MiyuSQL :** `query`, `transaction`, `cache`, `schema`
- **Segments :** minuscules, sans accents, separes par des points

---

## 4. Conformite au Toolkit Composition Contract

### 4.1 Principes Respectes

| Principe Toolkit | Application MiyuSQL |
|------------------|---------------------|
| Un Toolkit agrège des Tools existants | MiyuSQL regroupe neuf Tools declares individuellement dans le catalogue |
| Un Toolkit n'ajoute aucune capacite nouvelle | MiyuSQL n'expose que les capacites de ses Tools composants |
| Un Toolkit est declare et valide par l'environnement | MiyuSQL est declare dans Master Butler avec ToolkitId `toolkit.data.miyusql` |
| Tout Toolkit contient au moins deux Tools | MiyuSQL contient neuf Tools |

### 4.2 Structure Formelle du Toolkit MiyuSQL

| Champ | Valeur |
|-------|--------|
| **ToolkitId** | `toolkit.data.miyusql` |
| **Format ToolkitId** | `toolkit.<domain>.<name>` |
| **Domaine** | `data` |
| **Name** | `miyusql` |
| **Tools** | Ensemble des neuf ToolIds (voir section 5) |
| **security_level** | 2 |
| **allowed_states** | HEALTHY, DEGRADED |
| **disallowed_states** | SECURITY_LOCKDOWN, MAINTENANCE |
| **status** | Active |

---

## 5. Declaration ToolkitId et ToolIds

### 5.1 ToolkitId

| Element | Valeur |
|---------|--------|
| **ToolkitId** | `toolkit.data.miyusql` |
| **Nom lisible** | MiyuSQL |
| **Description** | Kit d'outils de manipulation de donnees en base (requetes, transactions, cache, schema) |

### 5.2 Liste des ToolIds Composants

| ToolId | capability_id (ex.) | Description courte |
|--------|---------------------|---------------------|
| `tool.query.execute` | `data.query.execute` | Execute une requete (lecture ou ecriture selon intention gouvernee) |
| `tool.query.prepare` | `data.query.prepare` | Prepare ou valide une requete sans l'executer |
| `tool.transaction.begin` | `data.transaction.begin` | Demarre une transaction |
| `tool.transaction.commit` | `data.transaction.commit` | Valide une transaction |
| `tool.transaction.rollback` | `data.transaction.rollback` | Annule une transaction |
| `tool.cache.get` | `cache.read` | Recupere une entree depuis le cache |
| `tool.cache.set` | `cache.write` | Enregistre une entree dans le cache |
| `tool.cache.invalidate` | `cache.invalidate` | Invalide une ou plusieurs entrees du cache |
| `tool.schema.read` | `data.schema.read` | Lit les metadonnees du schema (tables, colonnes) sans modifier |

### 5.3 Invariants de Declaration

| Code | Invariant |
|------|-----------|
| **INV-DECL-1** | Le ToolkitId `toolkit.data.miyusql` est unique dans le catalogue Master Butler |
| **INV-DECL-2** | Chaque ToolId listé est declare individuellement dans le catalogue avant d'etre associe au Toolkit |
| **INV-DECL-3** | Le niveau de securite du Toolkit est au moins egal au maximum des niveaux de ses Tools (ici : 2) |
| **INV-DECL-4** | Aucun Tool hors domaine data/cache n'est ajoute au Toolkit MiyuSQL sans revision contractuelle |

---

## 6. Catalogue et Utilisation

### 6.1 Enregistrement

- Master Butler declare le Toolkit MiyuSQL et la liste des ToolIds composants.
- Toute utilisation du Toolkit ou d'un de ses Tools passe par le catalogue et la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather).

### 6.2 Resolution

- Un Operateur (ou adaptateur) demande l'utilisation d'un Tool ou du Toolkit via BondingBrother.
- Master Butler verifie l'existence du Tool/Toolkit, les permissions requises, et le niveau de securite.
- En cas d'autorisation (StrongFather ALLOW), l'execution est delegatee ; les operations de donnees sont realisees sous autorite KindMother.

---

## 7. References Croisees

| Document | Lien |
|----------|------|
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../../MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - Reference Outils | [MiyuSQL - Reference Outils](../../MiyuSQL%20-%20Reference%20Outils.md) |
| Master Butler - Tool Governance Contract | [Master Butler - Tool Governance Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de creation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Contrat de reference
