# MiyuAuth — Tool Governance Compliance Contract

## 1. Contexte

Ce document définit la **conformité de MiyuAuth** aux contrats de gouvernance des Outils et Kits d'Outils de Master Butler. MiyuAuth est un **Kit d'Outils (Toolkit)** déclaré et gouverné par l'environnement ; ce contrat établit la déclaration formelle du ToolkitId, des ToolIds composants, et des capabilities associées.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- La conformité au [Master Butler - Tool Governance Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)
- La conformité au [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)
- La déclaration formelle du ToolkitId et des ToolIds MiyuAuth
- Le catalogue des capabilities exposées

Ce document **ne couvre pas** :
- L'implémentation technique des Tools
- Les contrats MiyuAuth hors gouvernance (intégration KindMother, sécurité, bornage)

---

## 3. Conformité au Tool Governance Contract

### 3.1 Principes respectés

| Principe Master Butler | Application MiyuAuth |
|------------------------|---------------------|
| Tout Tool possède un ToolId unique et immuable | Chaque outil MiyuAuth a un ToolId au format `tool.<domain>.<action>` |
| Tout Tool est lié à exactement une Capability | Chaque ToolId est associé à un capability_id (voir section 5) |
| Un Tool ne prend jamais de décision métier | Les Tools MiyuAuth exécutent uniquement ; pas de décision de confiance ni d'autorisation |
| Un Tool ne connaît jamais l'Opérateur appelant | MiyuAuth reçoit un contexte gouverné ; pas d'identité Opérateur dans la logique Tool |

### 3.2 Format ToolId

Format canonique : `tool.<domain>.<action>`

- **Préfixe :** `tool.`
- **Domaine MiyuAuth :** `identity`
- **Segments :** minuscules, sans accents, séparés par des points

---

## 4. Conformité au Toolkit Composition Contract

### 4.1 Principes respectés

| Principe Toolkit | Application MiyuAuth |
|------------------|---------------------|
| Un Toolkit agrège des Tools existants | MiyuAuth regroupe quatre Tools déclarés individuellement dans le catalogue |
| Un Toolkit n'ajoute aucune capacité nouvelle | MiyuAuth n'expose que les capacités de ses Tools composants |
| Un Toolkit est déclaré et validé par l'environnement | MiyuAuth est déclaré dans Master Butler avec ToolkitId `toolkit.identity.miyauth` |
| Tout Toolkit contient au moins deux Tools | MiyuAuth contient quatre Tools |

### 4.2 Structure formelle du Toolkit MiyuAuth

| Champ | Valeur |
|-------|--------|
| **ToolkitId** | `toolkit.identity.miyauth` |
| **Format ToolkitId** | `toolkit.<domain>.<name>` |
| **Domaine** | `identity` |
| **Name** | `miyauth` |
| **Tools** | Ensemble des quatre ToolIds (voir section 5) |
| **security_level** | 2 ou 3 (selon politique identité ; voir [MiyuAuth - Security and States Contract](../security/MiyuAuth%20-%20Security%20and%20States%20Contract.md)) |
| **allowed_states** | HEALTHY, DEGRADED |
| **disallowed_states** | SECURITY_LOCKDOWN, MAINTENANCE |
| **status** | Active |

---

## 5. Déclaration ToolkitId et ToolIds

### 5.1 ToolkitId

| Élément | Valeur |
|---------|--------|
| **ToolkitId** | `toolkit.identity.miyauth` |
| **Nom lisible** | MiyuAuth |
| **Description** | Kit d'outils d'identité utilisateur (résolution rôle, attestation, vérification Passeport/Visa) |

### 5.2 Liste des ToolIds composants

| ToolId | capability_id (ex.) | Description courte |
|--------|---------------------|---------------------|
| `tool.identity.resolve` | `identity.resolve` | Résout un contexte d'identité (citoyen, visiteur, externe) ; ne décide pas de la confiance |
| `tool.identity.attest` | `identity.attest` | Produit une attestation d'identité pour un contexte validé par KindMother |
| `tool.identity.verify` | `identity.verify` | Vérifie un Passeport Utilisateur ou un Visa de Connexion ; ne valide pas la confiance |
| `tool.identity.role` | `identity.role` | Retourne le rôle résolu (citoyen, visiteur, externe) pour un contexte gouverné |

### 5.3 Invariants de déclaration

| Code | Invariant |
|------|-----------|
| **INV-DECL-1** | Le ToolkitId `toolkit.identity.miyauth` est unique dans le catalogue Master Butler |
| **INV-DECL-2** | Chaque ToolId listé est déclaré individuellement dans le catalogue avant d'être associé au Toolkit |
| **INV-DECL-3** | Le niveau de sécurité du Toolkit est au moins égal au maximum des niveaux de ses Tools (2 ou 3 selon politique) |
| **INV-DECL-4** | Aucun Tool hors domaine identity n'est ajouté au Toolkit MiyuAuth sans révision contractuelle |

---

## 6. Catalogue et utilisation

### 6.1 Enregistrement

- Master Butler déclare le Toolkit MiyuAuth et la liste des ToolIds composants.
- Toute utilisation du Toolkit ou d'un de ses Tools passe par le catalogue et la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather).

### 6.2 Résolution

- Un Opérateur (ou adaptateur) demande l'utilisation d'un Tool ou du Toolkit via BondingBrother.
- Master Butler vérifie l'existence du Tool/Toolkit, les permissions requises, et le niveau de sécurité.
- En cas d'autorisation (StrongFather ALLOW), l'exécution est déléguée ; toute confiance utilisée pour l'identité est validée par KindMother.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - Reference Outils | [MiyuAuth - Reference Outils](../../MiyuAuth%20-%20Reference%20Outils.md) |
| Master Butler - Tool Governance Contract | [Master Butler - Tool Governance Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
