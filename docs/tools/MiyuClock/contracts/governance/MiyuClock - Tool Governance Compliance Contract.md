# MiyuClock — Tool Governance Compliance Contract

## 1. Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

Ce document définit la **conformité de MiyuClock** aux contrats de gouvernance des Outils et Kits d'Outils de Master Butler. MiyuClock est un **Kit d'Outils (Toolkit)** déclaré et gouverné par l'environnement ; ce contrat établit la déclaration formelle du ToolkitId, des ToolIds composants, et des capabilities associées.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- La conformité au [Master Butler - Tool Governance Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)
- La conformité au [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)
- La déclaration formelle du ToolkitId et des ToolIds MiyuClock
- Le catalogue des capabilities exposées

Ce document **ne couvre pas** :
- L'implémentation technique des Tools
- Les contrats MiyuClock hors gouvernance (sécurité, bornage, intégration KindMother)

---

## 3. Conformité au Tool Governance Contract

### 3.1 Principes respectés

| Principe Master Butler | Application MiyuClock |
|------------------------|---------------------|
| Tout Tool possède un ToolId unique et immuable | Chaque outil MiyuClock a un ToolId au format `tool.<domain>.<action>` |
| Tout Tool est lié à exactement une Capability | Chaque ToolId est associé à un capability_id (voir section 5) |
| Un Tool ne prend jamais de décision métier | Les Tools MiyuClock exécutent uniquement ; pas de décision, pas de persistance |
| Un Tool ne connaît jamais l'Opérateur appelant | MiyuClock reçoit un contexte gouverné ; pas d'identité Opérateur dans la logique Tool |

### 3.2 Format ToolId

Format canonique : `tool.<domain>.<action>`

- **Préfixe :** `tool.`
- **Domaine MiyuClock :** `time`
- **Segments :** minuscules, sans accents, séparés par des points

---

## 4. Conformité au Toolkit Composition Contract

### 4.1 Principes respectés

| Principe Toolkit | Application MiyuClock |
|------------------|---------------------|
| Un Toolkit agrège des Tools existants | MiyuClock regroupe deux Tools déclarés individuellement dans le catalogue |
| Un Toolkit n'ajoute aucune capacité nouvelle | MiyuClock n'expose que les capacités de ses Tools composants |
| Un Toolkit est déclaré et validé par l'environnement | MiyuClock est déclaré dans Master Butler avec ToolkitId `toolkit.time.miyuclock` |
| Tout Toolkit contient au moins deux Tools | MiyuClock contient deux Tools |

### 4.2 Structure formelle du Toolkit MiyuClock

| Champ | Valeur |
|-------|--------|
| **ToolkitId** | `toolkit.time.miyuclock` |
| **Format ToolkitId** | `toolkit.<domain>.<name>` |
| **Domaine** | `time` |
| **Name** | `miyuclock` |
| **Tools** | Ensemble des deux ToolIds (voir section 5) |
| **security_level** | 0 ou 1 (selon politique ; voir [MiyuClock - Security and States Contract](../security/MiyuClock%20-%20Security%20and%20States%20Contract.md)) |
| **allowed_states** | HEALTHY, DEGRADED |
| **disallowed_states** | SECURITY_LOCKDOWN, MAINTENANCE |
| **status** | Active |

---

## 5. Déclaration ToolkitId et ToolIds

### 5.1 ToolkitId

| Élément | Valeur |
|---------|--------|
| **ToolkitId** | `toolkit.time.miyuclock` |
| **Nom lisible** | MiyuClock |
| **Description** | Kit d'outils de mesure du temps (instant présent, delta entre instants) ; horloge locale, pas de temps global (LOI-4) |

### 5.2 Liste des ToolIds composants

| ToolId | capability_id (ex.) | Description courte |
|--------|---------------------|---------------------|
| `tool.time.now` | `time.now` | Retourne l'instant présent (référence locale ; pas de timezone imposée) |
| `tool.time.delta` | `time.delta` | Retourne la durée écoulée entre deux instants fournis dans le flux (t_prev, t_now) |

### 5.3 Invariants de déclaration

| Code | Invariant |
|------|-----------|
| **INV-DECL-1** | Le ToolkitId `toolkit.time.miyuclock` est unique dans le catalogue Master Butler |
| **INV-DECL-2** | Chaque ToolId listé est déclaré individuellement dans le catalogue avant d'être associé au Toolkit |
| **INV-DECL-3** | Le niveau de sécurité du Toolkit est au moins égal au maximum des niveaux de ses Tools (0 ou 1 selon politique) |
| **INV-DECL-4** | Aucun Tool hors domaine time n'est ajouté au Toolkit MiyuClock sans révision contractuelle |

---

## 6. Catalogue et utilisation

### 6.1 Enregistrement

- Master Butler déclare le Toolkit MiyuClock et la liste des ToolIds composants.
- Toute utilisation du Toolkit ou d'un de ses Tools passe par le catalogue et la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather).

### 6.2 Résolution

- Un Opérateur (ou adaptateur) demande l'utilisation d'un Tool ou du Toolkit via BondingBrother.
- Master Butler vérifie l'existence du Tool/Toolkit, les permissions requises, et le niveau de sécurité.
- En cas d'autorisation (StrongFather ALLOW), l'exécution est déléguée ; la mesure du temps s'appuie sur le Kernel (Clock) ; MiyuClock ne persiste pas.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Reference Outils | [MiyuClock - Reference Outils](../../MiyuClock%20-%20Reference%20Outils.md) |
| Master Butler - Tool Governance Contract | [Master Butler - Tool Governance Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
