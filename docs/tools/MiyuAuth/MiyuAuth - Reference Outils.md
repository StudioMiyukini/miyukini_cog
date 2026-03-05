# MiyuAuth â€” RÃ©fÃ©rence des outils

## 1. Contexte

Ce document dÃ©crit **chaque outil (Tool)** composant le kit MiyuAuth. Il constitue la rÃ©fÃ©rence technique des capacitÃ©s atomiques d'identitÃ© utilisateur (rÃ©solution de rÃ´le, attestation, vÃ©rification Passeport Utilisateur et Visa de Connexion) sans dÃ©cision de confiance ni d'autorisation. Les Tools sont gouvernÃ©s par les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather) ; la validation de la confiance appartient Ã  KindMother.

**RÃ©fÃ©rence du kit :** [MiyuAuth - Documentation Fondatrice](./MiyuAuth%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

**Ce document fournit :**

- La liste exhaustive des Tools du kit MiyuAuth
- Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte Â« fait quoi Â»), **niveau de sÃ©curitÃ©** typique, **capability_id** si applicable

**Hors scope :**

- L'implÃ©mentation (stockage Passeport/Visa, Ã©mission)
- La dÃ©cision ALLOW/DENY ou l'autorisation mÃ©tier (StrongFather, Cores)

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sÃ©curitÃ© | capability_id (ex.) |
|--------|-------------|--------|------------------|----------------------|
| `tool.identity.resolve` | RÃ©solution contexte identitÃ© | RÃ©sout le contexte d'identitÃ© (citoyen, visiteur, externe) Ã  partir des donnÃ©es fournies ; ne dÃ©cide pas de la confiance. | 2 ou 3 | `identity.resolve` |
| `tool.identity.attest` | Attestation identitÃ© | Produit une attestation d'identitÃ© Ã  partir du contexte validÃ© ; ne dÃ©cide pas de la confiance. | 2 ou 3 | `identity.attest` |
| `tool.identity.verify` | VÃ©rification Passeport/Visa | VÃ©rifie l'intÃ©gritÃ© et la validitÃ© d'un Passeport Utilisateur ou d'un Visa de Connexion ; ne dÃ©cide pas de l'autorisation. | 2 ou 3 | `identity.verify` |
| `tool.identity.role` | RÃ´le identitÃ© | DÃ©termine le rÃ´le (citoyen, visiteur, externe) Ã  partir du contexte validÃ© par KindMother ; ne dÃ©cide pas de l'autorisation. | 2 ou 3 | `identity.role` |

**Format ToolId :** `tool.<domain>.<action>` â€” conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md).

---

## 4. DÃ©tail par outil (rÃ©sumÃ©)

### 4.1 RÃ©solution

- **tool.identity.resolve** â€” Construit ou enrichit le contexte d'identitÃ© Ã  partir des artefacts fournis (Passeport, Visa, session, etc.). Retourne un contexte structurÃ© (citoyen / visiteur / externe) sans dÃ©cider de la confiance. La confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother.

### 4.2 Attestation

- **tool.identity.attest** â€” Produit une attestation d'identitÃ© Ã  partir du contexte dÃ©jÃ  validÃ©. Ne crÃ©e pas la confiance ; exÃ©cute une capacitÃ© d'attestation gouvernÃ©e.

### 4.3 VÃ©rification

- **tool.identity.verify** â€” VÃ©rifie l'intÃ©gritÃ© et la validitÃ© (signature, dates, champs) d'un Passeport Utilisateur ou d'un Visa de Connexion. Peut porter sur les champs dÃ©finis dans Connexion Inter-COG (ex. Passeport Â§ 3.1, Visa Â§ 3.3, niveaux S1â€“S5 du Visa). Retourne un rÃ©sultat de vÃ©rification (valide / invalide / expirÃ©, etc.) sans dÃ©cider de l'autorisation (ALLOW/DENY = StrongFather).

### 4.4 RÃ´le

- **tool.identity.role** â€” DÃ©termine le rÃ´le identitÃ© (citoyen, visiteur, externe) Ã  partir du contexte validÃ©. AlignÃ© sur la Connexion Inter-COG (COG Origine, COG HÃ©bergeur, Utilisateur Visiteur, Utilisateur Externe). Le rÃ´le Â« externe Â» correspond Ã  l'Utilisateur Externe (accÃ¨s via FaÃ§ade Publique GouvernÃ©e / Mandat Public d'AccÃ¨s). Ne dÃ©cide pas de l'autorisation.

---

## 5. Alignement MIP

Chaque outil listÃ© ci-dessus est conÃ§u pour Ãªtre une **unitÃ© logique** pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation :

- **id** : identifiant du bloc (ex. dÃ©rivÃ© du ToolId)
- **do** : description fonctionnelle courte (ex. Â« rÃ©sout le contexte d'identitÃ© Â»)
- **role** : rÃ´le sÃ©mantique (ex. `identity`)
- **layer** : couche (Strate 6 â€” outil / toolkit)

Ã€ l'implÃ©mentation, le code fournissant ces Tools devra Ãªtre balisÃ© MSCM afin d'alimenter **blocks.json**, **domains.json**, **layers.json** selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuAuth | [MiyuAuth - Documentation Fondatrice](./MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence


