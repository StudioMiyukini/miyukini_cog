# MiyuAuth — KindMother Integration Contract

## 1. Contexte

Ce document définit le contrat d'intégration entre **MiyuAuth** (kit d'outils d'identité utilisateur) et **KindMother** (Core de données, Strate 4). KindMother est l'unique validateur de la confiance inter-domaines ([KindMother - Identity & Cross-Domain Trust Contract](../../../core/KindMother/contracts/authority/KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md)). MiyuAuth exécute des capacités (resolve, attest, verify, role) sans décider de la confiance ; toute confiance utilisée pour l'identité est validée par KindMother.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Le rôle unique de KindMother comme validateur de la confiance inter-domaines
- L'exécution des capacités MiyuAuth (resolve, attest, verify, role) sans décision de confiance
- L'invariant : aucune confiance sans validation KindMother, pas de délégation de validation

Ce document **ne couvre pas** :
- L'implémentation interne de KindMother
- Les contrats MiyuAuth hors intégration (gouvernance, sécurité, bornage)
- Le détail du modèle Identity & Cross-Domain Trust (voir KindMother - Identity & Cross-Domain Trust Contract)

---

## 3. Principe fondamental

### 3.1 KindMother = validateur unique de la confiance

> **KindMother est l'unique validateur de toute confiance inter-domaines. MiyuAuth exécute des capacités (resolve, attest, verify, role) sans décider de la confiance ; toute confiance utilisée pour l'identité est validée par KindMother.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-KM-1** | Aucune confiance inter-domaines n'est utilisée pour l'identité sans validation explicite par KindMother |
| **INV-KM-2** | MiyuAuth ne valide pas la confiance ; il exécute les capacités mandatees après validation KindMother |
| **INV-KM-3** | La validation de la confiance n'est pas déléguable ; KindMother ne délègue pas à MiyuAuth ni à un adaptateur |
| **INV-KM-4** | MiyuAuth n'exécute que ce qui a été autorisé par la gouvernance (StrongFather, KindMother) |
| **INV-KM-5** | MiyuAuth n'ajoute aucune logique métier ; il orchestre des capacités atomiques d'identité |

---

## 4. Rôle des Tools MiyuAuth

### 4.1 Capacités exécutées, pas de décision de confiance

| ToolId | Rôle | Autorité / Validation |
|--------|------|------------------------|
| `tool.identity.resolve` | Résout un contexte d'identité (citoyen, visiteur, externe) à partir des données fournies | Toute confiance utilisée pour la résolution est validée par KindMother ; MiyuAuth exécute, ne décide pas |
| `tool.identity.attest` | Produit une attestation d'identité pour un contexte validé | Le contexte doit avoir été validé par KindMother ; MiyuAuth exécute l'attestation |
| `tool.identity.verify` | Vérifie un Passeport Utilisateur ou un Visa de Connexion (structure, signature) | Vérification technique ; la validation de la confiance reste à KindMother |
| `tool.identity.role` | Retourne le rôle résolu (citoyen, visiteur, externe) | Contexte gouverné ; MiyuAuth exécute, ne confère pas d'autorisation |

### 4.2 Ce que MiyuAuth ne fait jamais

| Interdiction | Description |
|-------------|-------------|
| **INTERDIT-1** | Décider de la confiance inter-domaines (validation = KindMother uniquement) |
| **INTERDIT-2** | Utiliser une confiance non validée par KindMother pour l'identité |
| **INTERDIT-3** | Déléguer ou recevoir une délégation de validation de confiance |
| **INTERDIT-4** | Traiter la reconnaissance d'identité comme une autorisation (identité ≠ autorisation) |

---

## 5. Flux de confiance

```
Opérateur / Adaptateur
        │
        │ 1. Demande d'utilisation d'un Tool MiyuAuth (resolve, attest, verify, role)
        ▼
BondingBrother ──► Master Butler ──► WorrySentinel ──► Caring Nanny ──► StrongFather
        │                                                                      │
        │ 2. ALLOW                                                             │
        ▼                                                                      │
KindMother : validation de la confiance (si nécessaire pour l'identité)       │
        │                                                                      │
        │ 3. Mandat d'exécution (tool.identity.*)                             │
        ▼                                                                      │
MiyuAuth Tools : exécution gouvernée (sans décision de confiance)
```

---

## 6. Absence de contournement

Aucun chemin ne peut contourner :
1. La médiation BondingBrother (intention, contexte)
2. Le catalogue Master Butler (Tool/Toolkit, permissions)
3. Les Cores WorrySentinel et Caring Nanny (sécurité, état système)
4. La décision StrongFather (ALLOW/DENY)
5. La validation KindMother (confiance inter-domaines pour l'identité)

MiyuAuth n'exécute que dans le cadre de ce flux ; il ne valide jamais la confiance lui-même.

---

## 6bis. Relation avec MiyuSQL — Données d'identification, Passeport, Visa

### 6bis.1 Persistance des données d'identification

La **persistance** (lecture / écriture en base) des données d'identification, des Passeports Utilisateurs et des Visas de Connexion relève de **KindMother** et est **exécutée via MiyuSQL** lorsque KindMother mandate les opérations (WriteIntent pour les écritures, mandat d'exécution pour les lectures). MiyuAuth **ne persiste pas** et **ne lit pas** en base ; il ne dépend pas de MiyuSQL et n'accède pas à la persistance.

| Opération | Autorité | Exécution technique | MiyuAuth |
|-----------|----------|----------------------|----------|
| Stockage (création, mise à jour) Passeport / Visa | KindMother | MiyuSQL (sous WriteIntent) | N'intervient pas |
| Lecture Passeport / Visa depuis la base | KindMother | MiyuSQL (mandat d'exécution) | N'intervient pas |
| Vérification (structure, signature) d'un artefact fourni | — | MiyuAuth (`tool.identity.verify`) | Exécute sur l'artefact reçu |
| Résolution rôle / contexte à partir de données fournies | — | MiyuAuth (`tool.identity.resolve`, `tool.identity.role`) | Exécute sur le contexte reçu |

### 6bis.2 Flux gouverné typique

Les données (Passeport, Visa, enregistrements d'identité) sont d'abord **lues ou produites** sous autorité KindMother (avec MiyuSQL pour la persistance). Elles sont ensuite **fournies au flux** (contexte, session, paramètres). MiyuAuth est invoqué sur ces données **déjà présentes dans le flux** pour vérification, résolution ou attestation — sans accéder lui-même à la base.

**Référence :** [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) (section 8bis Relation avec MiyuSQL), [MiyuSQL - KindMother Integration Contract](../../MiyuSQL/contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md).

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| KindMother - Identity & Cross-Domain Trust Contract | [KindMother - Identity & Cross-Domain Trust Contract](../../../core/KindMother/contracts/authority/KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md) |
| KindMother - Index | [KindMother - Index](../../../core/KindMother/_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](../../MiyuSQL/contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |
| Security Levels (référence conceptuelle) | [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
