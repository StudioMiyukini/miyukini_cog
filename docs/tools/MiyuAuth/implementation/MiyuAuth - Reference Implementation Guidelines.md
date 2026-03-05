# MiyuAuth â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuAuth conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuAuth en logique d'implÃ©mentation (Tools, gouvernance, KindMother, bornage).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuAuth (Tools resolve, attest, verify, role) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, KindMother Integration, Tool Governance Compliance, Security and States, Runtime Boundary, Dependencies, Unit Tests, Cycle Tests.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuAuth - Documentation Fondatrice** : IdentitÃ©, ToolkitId, liste des Tools, gouvernance
- **MiyuAuth - KindMother Integration Contract** : Confiance validÃ©e par KindMother uniquement, exÃ©cution sans dÃ©cision de confiance
- **MiyuAuth - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities
- **MiyuAuth - Security and States Contract** : Niveau 2 ou 3, Ã©tats autorisÃ©s/interdits
- **MiyuAuth - Runtime Boundary Contract** : Bornage, interdictions (BOUND-*), invariants INV-BOUND-*
- **MiyuAuth - Dependencies Contract** : Liste fermÃ©e des dÃ©pendances, ordre d'invocation
- **Master Butler - Tool Governance Contract** et **Toolkit Composition Contract** : Format ToolId, structure Toolkit

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

**Principe contractuel :** MiyuAuth ne dÃ©cide pas si une action doit Ãªtre autorisÃ©e ; StrongFather dÃ©cide ALLOW/DENY.

**Traduction en implÃ©mentation :**

- MiyuAuth est invoquÃ© uniquement aprÃ¨s dÃ©cision ALLOW de la gouvernance. L'implÃ©mentation ne doit pas rÃ©-Ã©valuer les permissions.
- En cas d'appel hors gouvernance (anomalie), MiyuAuth doit refuser l'exÃ©cution et signaler (pas de dÃ©cision de contournement).

### 2.2 Pas de confiance sans validation KindMother (BOUND-3)

**Principe contractuel :** Toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother.

**Traduction en implÃ©mentation :**

- L'implÃ©mentation des Tools (resolve, attest, verify, role) ne doit jamais utiliser une confiance non validÃ©e par KindMother.
- L'interface entre KindMother et MiyuAuth doit garantir que l'appel Ã  MiyuAuth n'a lieu qu'avec un contexte ou des artefacts dÃ©jÃ  validÃ©s (ou aprÃ¨s validation KindMother dans le flux).

### 2.3 RÃ©solution explicite du rÃ´le (tool.identity.resolve, tool.identity.role)

**Principe contractuel :** MiyuAuth rÃ©sout le contexte d'identitÃ© et dÃ©termine le rÃ´le (citoyen, visiteur, externe) sans dÃ©cider de l'autorisation.

**Traduction en implÃ©mentation :**

- Les Tools resolve et role retournent un rÃ©sultat structurÃ© (contexte, rÃ´le) ; ils ne retournent pas ALLOW/DENY.
- Alignement Connexion Inter-COG : Passeport Utilisateur, Visa de Connexion, COG Origine, COG HÃ©bergeur, Utilisateur Visiteur, Utilisateur Externe â€” terminologie et concepts respectÃ©s dans les structures de donnÃ©es.

### 2.4 Liste fermÃ©e des dÃ©pendances (INV-DEP-*)

**Principe contractuel :** MiyuAuth ne dÃ©pend que des Cores et du Kernel dÃ©finis dans le Dependencies Contract.

**Traduction en implÃ©mentation :**

- Aucune dÃ©pendance vers un OpÃ©rateur, un produit, ou une rÃ¨gle mÃ©tier.
- Les appels entrants passent par BondingBrother et la chaÃ®ne de gouvernance ; MiyuAuth n'expose pas d'API publique directe aux OpÃ©rateurs.
- Usage du Kernel (Id, Logger, Clock, Config, Lifecycle) pour identifiants, logs, horodatage, configuration locale, cycle de vie â€” sans logique mÃ©tier.

### 2.5 Alignement MIP/MSCM

**Principe contractuel :** Domaine `identity`, layer Strate 6 ; chaque Tool = bloc logique (id, do, role, layer).

**Traduction en implÃ©mentation :**

- Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le Protocole MIP v1.

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | Pas de code qui Ã©value ALLOW/DENY ; exÃ©cution uniquement sur mandat |
| **BOUND-2** | Pas d'autorisation mÃ©tier | Pas de code qui crÃ©e mandat, rÃ©vocation, modification des permissions |
| **BOUND-3** | Pas de confiance sans validation KindMother | Pas de code qui utilise une confiance non validÃ©e par KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte ; pas de rÃ©vocation, pas de crÃ©ation de Visa/Passeport |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Pas d'identitÃ© OpÃ©rateur dans la logique Tool ; contexte anonymisÃ© (permissions, niveau) |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Chaque Tool correspond exactement Ã  un ToolId dÃ©clarÃ© ; pas d'extension non dÃ©clarÃ©e |

---

## 4. Patterns recommandÃ©s

### 4.1 Structure des Tools

- Chaque ToolId est implÃ©mentÃ© comme une unitÃ© d'exÃ©cution atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres), sortie (rÃ©sultat ou erreur contractuelle).
- Pas d'Ã©tat mÃ©tier partagÃ© entre appels ; Ã©tat technique sous contrÃ´le du flux gouvernÃ©.

### 4.2 Interface avec KindMother

- L'implÃ©mentation MiyuAuth consomme un contexte ou des artefacts dÃ©jÃ  validÃ©s par KindMother (ou reÃ§oit un mandat d'exÃ©cution aprÃ¨s validation KindMother). Les paramÃ¨tres incluent : type d'opÃ©ration (resolve, attest, verify, role), paramÃ¨tres (contexte, Passeport/Visa mock ou validÃ©), contexte gouvernÃ© (sans identitÃ© OpÃ©rateur mÃ©tier).
- La rÃ©ponse inclut : succÃ¨s/Ã©chec, rÃ©sultat (contexte rÃ©solu, attestation, rÃ©sultat vÃ©rification, rÃ´le), ou erreur explicite (sans fuite d'information sensible).

### 4.3 Gestion des erreurs

- Les erreurs techniques (format invalide, signature, expiration) sont remontÃ©es de maniÃ¨re explicite sans exposer de donnÃ©es mÃ©tier.
- En cas de violation de bornage (ex. appel sans mandat, confiance non validÃ©e), refus d'exÃ©cution et signal (observability) ; pas d'exÃ©cution partielle.

### 4.4 TraÃ§abilitÃ©

- Utiliser le Logger du Kernel pour tracer les exÃ©cutions (sans contenu mÃ©tier sensible). ConformitÃ© aux contrats KindMother Observability et MiyuAuth Runtime Boundary.

---

## 5. Alignement MIP / MSCM

### 5.1 MIP v1

Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuAuth doit Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : `identity` (cohÃ©rent avec domains.json).
- **Layer** : Strate 6 (outil / toolkit) dans layers.json.
- **Blocs** : Chaque Tool MiyuAuth est une unitÃ© logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json.

### 5.2 MSCM

Les blocs de code correspondant aux Tools doivent Ãªtre balisÃ©s selon le standard MSCM (Miyukini Semantic Code Markup) pour permettre l'indexation et la gouvernance structurelle.

### 5.3 NumÃ©rotation des invariants

Les contrats MiyuAuth utilisent des **prÃ©fixes catÃ©goriels** pour les invariants : BOUND (bornage), DEP (dÃ©pendances), SEC (sÃ©curitÃ©), INV-KM-* (intÃ©gration KindMother), INV-UT-MAUTH-* (tests unitaires), INV-CT-MAUTH-* (tests de cycle). Le format canonique des invariants des Cores (INV-&lt;PREFIX&gt;-&lt;NUMERO&gt;) est dÃ©fini dans [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//miyukini-webway-system//reference//_index.md) ; les prÃ©fixes MiyuAuth restent cohÃ©rents en interne et distincts des codes Cores (KM, SF, etc.).

---

## 6. Tests (rappel)

- **Tests unitaires** : ConformÃ©ment au [MiyuAuth - Unit Tests Contract](../contracts/testing/MiyuAuth%20-%20Unit%20Tests%20Contract.md) â€” pas de modification de donnÃ©es mÃ©tier ; mocks ou sandbox pour attest/verify.
- **Test de cycle MiyuAuth** : ConformÃ©ment au [MiyuAuth - Cycle Tests Contract](../contracts/testing/MiyuAuth%20-%20Cycle%20Tests%20Contract.md) â€” scÃ©nario E2E (rÃ©solution â†’ rÃ´le â†’ vÃ©rification Passeport/Visa dans un scÃ©nario gouvernÃ©). ExÃ©cutable par MiyukiniAdmin.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../contracts/integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| MiyuAuth - Runtime Boundary Contract | [MiyuAuth - Runtime Boundary Contract](../contracts/boundaries/MiyuAuth%20-%20Runtime%20Boundary%20Contract.md) |
| MiyuAuth - Security and States Contract | [MiyuAuth - Security and States Contract](../contracts/security/MiyuAuth%20-%20Security%20and%20States%20Contract.md) |
| MiyuAuth - Dependencies Contract | [MiyuAuth - Dependencies Contract](../dependencies/MiyuAuth%20-%20Dependencies%20Contract.md) |
| MiyuAuth - Unit Tests Contract | [MiyuAuth - Unit Tests Contract](../contracts/testing/MiyuAuth%20-%20Unit%20Tests%20Contract.md) |
| MiyuAuth - Cycle Tests Contract | [MiyuAuth - Cycle Tests Contract](../contracts/testing/MiyuAuth%20-%20Cycle%20Tests%20Contract.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |
| Standardisation NumÃ©ration Invariants | [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif

