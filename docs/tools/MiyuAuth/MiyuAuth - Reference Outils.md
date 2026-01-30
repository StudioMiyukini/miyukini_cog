# MiyuAuth — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuAuth. Il constitue la référence technique des capacités atomiques d'identité utilisateur (résolution de rôle, attestation, vérification Passeport Utilisateur et Visa de Connexion) sans décision de confiance ni d'autorisation. Les Tools sont gouvernés par les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather) ; la validation de la confiance appartient à KindMother.

**Référence du kit :** [MiyuAuth - Documentation Fondatrice](./MiyuAuth%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

**Ce document fournit :**

- La liste exhaustive des Tools du kit MiyuAuth
- Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte « fait quoi »), **niveau de sécurité** typique, **capability_id** si applicable

**Hors scope :**

- L'implémentation (stockage Passeport/Visa, émission)
- La décision ALLOW/DENY ou l'autorisation métier (StrongFather, Cores)

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité | capability_id (ex.) |
|--------|-------------|--------|------------------|----------------------|
| `tool.identity.resolve` | Résolution contexte identité | Résout le contexte d'identité (citoyen, visiteur, externe) à partir des données fournies ; ne décide pas de la confiance. | 2 ou 3 | `identity.resolve` |
| `tool.identity.attest` | Attestation identité | Produit une attestation d'identité à partir du contexte validé ; ne décide pas de la confiance. | 2 ou 3 | `identity.attest` |
| `tool.identity.verify` | Vérification Passeport/Visa | Vérifie l'intégrité et la validité d'un Passeport Utilisateur ou d'un Visa de Connexion ; ne décide pas de l'autorisation. | 2 ou 3 | `identity.verify` |
| `tool.identity.role` | Rôle identité | Détermine le rôle (citoyen, visiteur, externe) à partir du contexte validé par KindMother ; ne décide pas de l'autorisation. | 2 ou 3 | `identity.role` |

**Format ToolId :** `tool.<domain>.<action>` — conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md).

---

## 4. Détail par outil (résumé)

### 4.1 Résolution

- **tool.identity.resolve** — Construit ou enrichit le contexte d'identité à partir des artefacts fournis (Passeport, Visa, session, etc.). Retourne un contexte structuré (citoyen / visiteur / externe) sans décider de la confiance. La confiance utilisée pour l'identité est validée par KindMother.

### 4.2 Attestation

- **tool.identity.attest** — Produit une attestation d'identité à partir du contexte déjà validé. Ne crée pas la confiance ; exécute une capacité d'attestation gouvernée.

### 4.3 Vérification

- **tool.identity.verify** — Vérifie l'intégrité et la validité (signature, dates, champs) d'un Passeport Utilisateur ou d'un Visa de Connexion. Peut porter sur les champs définis dans Connexion Inter-COG (ex. Passeport § 3.1, Visa § 3.3, niveaux S1–S5 du Visa). Retourne un résultat de vérification (valide / invalide / expiré, etc.) sans décider de l'autorisation (ALLOW/DENY = StrongFather).

### 4.4 Rôle

- **tool.identity.role** — Détermine le rôle identité (citoyen, visiteur, externe) à partir du contexte validé. Aligné sur la Connexion Inter-COG (COG Origine, COG Hébergeur, Utilisateur Visiteur, Utilisateur Externe). Le rôle « externe » correspond à l'Utilisateur Externe (accès via Façade Publique Gouvernée / Mandat Public d'Accès). Ne décide pas de l'autorisation.

---

## 5. Alignement MIP

Chaque outil listé ci-dessus est conçu pour être une **unité logique** pouvant devenir un **bloc MSCM** à l'implémentation :

- **id** : identifiant du bloc (ex. dérivé du ToolId)
- **do** : description fonctionnelle courte (ex. « résout le contexte d'identité »)
- **role** : rôle sémantique (ex. `identity`)
- **layer** : couche (Strate 6 — outil / toolkit)

À l'implémentation, le code fournissant ces Tools devra être balisé MSCM afin d'alimenter **blocks.json**, **domains.json**, **layers.json** selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Références croisées

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuAuth | [MiyuAuth - Documentation Fondatrice](./MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
