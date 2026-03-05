# MiyuAuth â€” Tool Governance Compliance Contract

## 1. Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

Ce document dÃ©finit la **conformitÃ© de MiyuAuth** aux contrats de gouvernance des Outils et Kits d'Outils de Master Butler. MiyuAuth est un **Kit d'Outils (Toolkit)** dÃ©clarÃ© et gouvernÃ© par l'environnement ; ce contrat Ã©tablit la dÃ©claration formelle du ToolkitId, des ToolIds composants, et des capabilities associÃ©es.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- La conformitÃ© au [Master Butler - Tool Governance Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)
- La conformitÃ© au [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)
- La dÃ©claration formelle du ToolkitId et des ToolIds MiyuAuth
- Le catalogue des capabilities exposÃ©es

Ce document **ne couvre pas** :
- L'implÃ©mentation technique des Tools
- Les contrats MiyuAuth hors gouvernance (intÃ©gration KindMother, sÃ©curitÃ©, bornage)

---

## 3. ConformitÃ© au Tool Governance Contract

### 3.1 Principes respectÃ©s

| Principe Master Butler | Application MiyuAuth |
|------------------------|---------------------|
| Tout Tool possÃ¨de un ToolId unique et immuable | Chaque outil MiyuAuth a un ToolId au format `tool.<domain>.<action>` |
| Tout Tool est liÃ© Ã  exactement une Capability | Chaque ToolId est associÃ© Ã  un capability_id (voir section 5) |
| Un Tool ne prend jamais de dÃ©cision mÃ©tier | Les Tools MiyuAuth exÃ©cutent uniquement ; pas de dÃ©cision de confiance ni d'autorisation |
| Un Tool ne connaÃ®t jamais l'OpÃ©rateur appelant | MiyuAuth reÃ§oit un contexte gouvernÃ© ; pas d'identitÃ© OpÃ©rateur dans la logique Tool |

### 3.2 Format ToolId

Format canonique : `tool.<domain>.<action>`

- **PrÃ©fixe :** `tool.`
- **Domaine MiyuAuth :** `identity`
- **Segments :** minuscules, sans accents, sÃ©parÃ©s par des points

---

## 4. ConformitÃ© au Toolkit Composition Contract

### 4.1 Principes respectÃ©s

| Principe Toolkit | Application MiyuAuth |
|------------------|---------------------|
| Un Toolkit agrÃ¨ge des Tools existants | MiyuAuth regroupe quatre Tools dÃ©clarÃ©s individuellement dans le catalogue |
| Un Toolkit n'ajoute aucune capacitÃ© nouvelle | MiyuAuth n'expose que les capacitÃ©s de ses Tools composants |
| Un Toolkit est dÃ©clarÃ© et validÃ© par l'environnement | MiyuAuth est dÃ©clarÃ© dans Master Butler avec ToolkitId `toolkit.identity.miyauth` |
| Tout Toolkit contient au moins deux Tools | MiyuAuth contient quatre Tools |

### 4.2 Structure formelle du Toolkit MiyuAuth

| Champ | Valeur |
|-------|--------|
| **ToolkitId** | `toolkit.identity.miyauth` |
| **Format ToolkitId** | `toolkit.<domain>.<name>` |
| **Domaine** | `identity` |
| **Name** | `miyauth` |
| **Tools** | Ensemble des quatre ToolIds (voir section 5) |
| **security_level** | 2 ou 3 (selon politique identitÃ© ; voir [MiyuAuth - Security and States Contract](../security/MiyuAuth%20-%20Security%20and%20States%20Contract.md)) |
| **allowed_states** | HEALTHY, DEGRADED |
| **disallowed_states** | SECURITY_LOCKDOWN, MAINTENANCE |
| **status** | Active |

---

## 5. DÃ©claration ToolkitId et ToolIds

### 5.1 ToolkitId

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **ToolkitId** | `toolkit.identity.miyauth` |
| **Nom lisible** | MiyuAuth |
| **Description** | Kit d'outils d'identitÃ© utilisateur (rÃ©solution rÃ´le, attestation, vÃ©rification Passeport/Visa) |

### 5.2 Liste des ToolIds composants

| ToolId | capability_id (ex.) | Description courte |
|--------|---------------------|---------------------|
| `tool.identity.resolve` | `identity.resolve` | RÃ©sout un contexte d'identitÃ© (citoyen, visiteur, externe) ; ne dÃ©cide pas de la confiance |
| `tool.identity.attest` | `identity.attest` | Produit une attestation d'identitÃ© pour un contexte validÃ© par KindMother |
| `tool.identity.verify` | `identity.verify` | VÃ©rifie un Passeport Utilisateur ou un Visa de Connexion ; ne valide pas la confiance |
| `tool.identity.role` | `identity.role` | Retourne le rÃ´le rÃ©solu (citoyen, visiteur, externe) pour un contexte gouvernÃ© |

### 5.3 Invariants de dÃ©claration

| Code | Invariant |
|------|-----------|
| **INV-DECL-1** | Le ToolkitId `toolkit.identity.miyauth` est unique dans le catalogue Master Butler |
| **INV-DECL-2** | Chaque ToolId listÃ© est dÃ©clarÃ© individuellement dans le catalogue avant d'Ãªtre associÃ© au Toolkit |
| **INV-DECL-3** | Le niveau de sÃ©curitÃ© du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools (2 ou 3 selon politique) |
| **INV-DECL-4** | Aucun Tool hors domaine identity n'est ajoutÃ© au Toolkit MiyuAuth sans rÃ©vision contractuelle |

---

## 6. Catalogue et utilisation

### 6.1 Enregistrement

- Master Butler dÃ©clare le Toolkit MiyuAuth et la liste des ToolIds composants.
- Toute utilisation du Toolkit ou d'un de ses Tools passe par le catalogue et la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather).

### 6.2 RÃ©solution

- Un OpÃ©rateur (ou adaptateur) demande l'utilisation d'un Tool ou du Toolkit via BondingBrother.
- Master Butler vÃ©rifie l'existence du Tool/Toolkit, les permissions requises, et le niveau de sÃ©curitÃ©.
- En cas d'autorisation (StrongFather ALLOW), l'exÃ©cution est dÃ©lÃ©guÃ©e ; toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - Reference Outils | [MiyuAuth - Reference Outils](../../MiyuAuth%20-%20Reference%20Outils.md) |
| Master Butler - Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence



