# MiyuWeb â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuWeb conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuWeb en logique d'implÃ©mentation (Tools, gouvernance, KindMother, bornage, sanitization/CSP).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuWeb (Tools rendu HTML, layout, thÃ¨me, script, asset, formulaire, Ã©vÃ©nements) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, KindMother Integration, Tool Governance Compliance, Security and States, Runtime Boundary, Dependencies, Unit Tests, Cycle Tests.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuWeb - Documentation Fondatrice** : ToolkitId, liste des Tools, gouvernance, relation KindMother/MiyuSQL
- **MiyuWeb - KindMother Integration Contract** : Aucune lecture directe de la base ; donnÃ©es fournies dans le flux
- **MiyuWeb - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities
- **MiyuWeb - Security and States Contract** : Niveau 0, 1 ou 2, Ã©tats autorisÃ©s/interdits, XSS/CSP
- **MiyuWeb - Runtime Boundary Contract** : Bornage, interdictions (BOUND-*), invariants INV-BOUND-*
- **MiyuWeb - Dependencies Contract** : Liste fermÃ©e des dÃ©pendances, ordre d'invocation
- **Master Butler - Tool Governance Contract** et **Toolkit Composition Contract** : Format ToolId, structure Toolkit

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

**Principe contractuel :** MiyuWeb ne dÃ©cide pas si une action doit Ãªtre faite ; StrongFather dÃ©cide ALLOW/DENY.

**Traduction en implÃ©mentation :**

- MiyuWeb est invoquÃ© uniquement aprÃ¨s dÃ©cision ALLOW de la gouvernance. L'implÃ©mentation ne doit pas rÃ©-Ã©valuer les permissions.
- En cas d'appel hors gouvernance (anomalie), MiyuWeb doit refuser l'exÃ©cution et signaler (pas de dÃ©cision de contournement).

### 2.2 Pas de choix de contenu (BOUND-2)

**Principe contractuel :** MiyuWeb ne dÃ©cide pas quel contenu afficher ; il rend, rÃ©sout ou sert ce qui lui est fourni dans le flux.

**Traduction en implÃ©mentation :**

- Les Tools (html.render, layout.render, theme.resolve, asset.serve, etc.) ne choisissent jamais le contenu ; ils exÃ©cutent uniquement sur les donnÃ©es fournies en entrÃ©e.
- Aucune interprÃ©tation mÃ©tier du contenu ; MiyuWeb ne dÃ©cide pas des templates, thÃ¨mes ou rÃ¨gles applicatives.

### 2.3 Pas d'accÃ¨s direct Ã  la base (BOUND-3)

**Principe contractuel :** MiyuWeb ne lit jamais la base (templates, assets). Toutes les donnÃ©es sont fournies dans le flux gouvernÃ©.

**Traduction en implÃ©mentation :**

- L'implÃ©mentation ne doit contenir aucun appel direct Ã  une base de donnÃ©es, Ã  MiyuSQL, ni Ã  un stockage persistant pour lire templates ou assets.
- Les entrÃ©es (template, contenu, mÃ©tadonnÃ©es d'assets) sont toujours passÃ©es en paramÃ¨tres du flux gouvernÃ© (Ã©ventuellement aprÃ¨s lecture par MiyuSQL sous KindMother en amont).

### 2.4 Sanitization et CSP (Security Contract)

**Principe contractuel :** Tout contenu destinÃ© au rendu HTML ou Ã  l'exÃ©cution de script doit Ãªtre traitÃ© selon la politique de sanitization ; l'implÃ©mentation doit Ãªtre compatible avec les directives CSP dÃ©finies par WorrySentinel / environnement.

**Traduction en implÃ©mentation :**

- **XSS :** Ne jamais injecter de contenu non gouvernÃ© dans le HTML ou les scripts ; appliquer la sanitization dÃ©finie par l'environnement sur les donnÃ©es fournies dans le flux avant rendu ou exÃ©cution.
- **CSP :** Les Tools `tool.web.html.render` et `tool.web.script.execute` ne doivent pas contourner la Content Security Policy ; respecter les directives (sources autorisÃ©es, pas d'inline non autorisÃ©, pas d'eval non gouvernÃ©).

### 2.5 Liste fermÃ©e des dÃ©pendances (INV-DEP-*)

**Principe contractuel :** MiyuWeb ne dÃ©pend que des Cores et du Kernel dÃ©finis dans le Dependencies Contract.

**Traduction en implÃ©mentation :**

- Aucune dÃ©pendance vers un OpÃ©rateur, un produit, ou une rÃ¨gle mÃ©tier.
- Les appels entrants passent par BondingBrother et la chaÃ®ne de gouvernance ; MiyuWeb n'expose pas d'API publique directe aux OpÃ©rateurs.
- Usage du Kernel (Id, Logger, Clock, Config, Lifecycle) pour identifiants, logs, horodatage, configuration locale, cycle de vie â€” sans logique mÃ©tier.

### 2.6 Alignement MIP/MSCM

**Principe contractuel :** Domaine `web`, layer Strate 6 ; chaque Tool = bloc logique (id, do, role, layer).

**Traduction en implÃ©mentation :**

- Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le Protocole MIP v1.

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | Pas de code qui Ã©value ALLOW/DENY ; exÃ©cution uniquement sur mandat |
| **BOUND-2** | Pas de choix de contenu | Pas de code qui choisit templates, thÃ¨mes ou contenu ; exÃ©cution sur donnÃ©es fournies |
| **BOUND-3** | Pas d'accÃ¨s direct Ã  la base | Pas de lecture DB, MiyuSQL, ni stockage pour templates/assets |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte ; pas de mandat, pas de rÃ©vocation |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Pas d'identitÃ© OpÃ©rateur dans la logique Tool ; contexte anonymisÃ© (permissions, niveau) |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Chaque Tool correspond exactement Ã  un ToolId dÃ©clarÃ© ; pas d'extension non dÃ©clarÃ©e |

---

## 4. Patterns recommandÃ©s

### 4.1 Structure des Tools

- Chaque ToolId est implÃ©mentÃ© comme une unitÃ© d'exÃ©cution atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres : template, donnÃ©es, asset, etc.), sortie (rÃ©sultat ou erreur contractuelle).
- Pas d'Ã©tat mÃ©tier partagÃ© entre appels ; Ã©tat technique sous contrÃ´le du flux gouvernÃ©.
- Format ToolId : `tool.web.<sous-domaine>.<action>` ou `tool.web.<action>` (ex. `tool.web.html.render`, `tool.web.script.execute`).

### 4.2 Interface avec le flux (KindMother / donnÃ©es en flux)

- L'implÃ©mentation MiyuWeb consomme des donnÃ©es **dÃ©jÃ  prÃ©sentes dans le flux** : template, contenu, mÃ©tadonnÃ©es d'assets, contexte thÃ¨me, schÃ©ma formulaire, etc. Les paramÃ¨tres incluent : type d'opÃ©ration (render, resolve, execute, serve, validate, dispatch, capture), donnÃ©es fournies, contexte gouvernÃ© (sans identitÃ© OpÃ©rateur mÃ©tier).
- La rÃ©ponse inclut : succÃ¨s/Ã©chec, rÃ©sultat (HTML, structure layout, thÃ¨me rÃ©solu, rÃ©sultat script, asset servi, validation formulaire, Ã©vÃ©nement dispatchÃ©/capturÃ©), ou erreur explicite (sans fuite d'information sensible).
- Aucun appel Ã  KindMother ou MiyuSQL depuis MiyuWeb ; les donnÃ©es ont Ã©tÃ© lues en amont et sont passÃ©es en entrÃ©e.

### 4.3 Gestion des erreurs

- Les erreurs techniques (template invalide, script mal formÃ©, asset manquant dans le flux) sont remontÃ©es de maniÃ¨re explicite sans exposer de donnÃ©es mÃ©tier.
- En cas de violation de bornage (ex. appel sans mandat, tentative d'accÃ¨s direct Ã  une source de donnÃ©es), refus d'exÃ©cution et signal (observability) ; pas d'exÃ©cution partielle.

### 4.4 TraÃ§abilitÃ©

- Utiliser le Logger du Kernel pour tracer les exÃ©cutions (sans contenu mÃ©tier sensible). ConformitÃ© aux contrats KindMother Observability et MiyuWeb Runtime Boundary.

### 4.5 Sandbox et scripts

- Pour `tool.web.script.execute` : exÃ©cution dans un contexte sandboxÃ© ; aucun accÃ¨s direct Ã  la base ni dÃ©cision mÃ©tier depuis le script exÃ©cutÃ©. Les entrÃ©es (donnÃ©es, contexte) sont celles fournies dans le flux.

---

## 5. Alignement MIP / MSCM

### 5.1 MIP v1

Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuWeb doit Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : `web` (cohÃ©rent avec domains.json).
- **Layer** : Strate 6 (outil / toolkit) dans layers.json.
- **Blocs** : Chaque Tool MiyuWeb est une unitÃ© logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json.

### 5.2 MSCM

Les blocs de code correspondant aux Tools doivent Ãªtre balisÃ©s selon le standard MSCM (Miyukini Semantic Code Markup) pour permettre l'indexation et la gouvernance structurelle.

### 5.3 NumÃ©rotation des invariants

Les contrats MiyuWeb utilisent des **prÃ©fixes catÃ©goriels** pour les invariants : BOUND (bornage), DEP (dÃ©pendances), SEC (sÃ©curitÃ©), INV-KM-* (intÃ©gration KindMother), INV-UT-MWEB-* (tests unitaires), INV-CT-MWEB-* (tests de cycle). Le format canonique des invariants des Cores (INV-&lt;PREFIX&gt;-&lt;NUMERO&gt;) est dÃ©fini dans [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//miyukini-webway-system//reference//_index.md) ; les prÃ©fixes MiyuWeb restent cohÃ©rents en interne et distincts des codes Cores (KM, SF, etc.).

---

## 6. Tests (rappel)

- **Tests unitaires** : ConformÃ©ment au [MiyuWeb - Unit Tests Contract](../contracts/testing/MiyuWeb%20-%20Unit%20Tests%20Contract.md) â€” pas de modification de donnÃ©es mÃ©tier ; sandbox ou mocks pour templates/assets ; critÃ¨res par Tool (render, script.execute, script.compile, asset.serve, theme.resolve, form.validate, event.dispatch, input.capture).
- **Tests de cycle MiyuWeb** : ConformÃ©ment au [MiyuWeb - Cycle Tests Contract](../contracts/testing/MiyuWeb%20-%20Cycle%20Tests%20Contract.md) â€” scÃ©nario E2E (rÃ©solution thÃ¨me â†’ donnÃ©es template en flux â†’ rendu HTML/layout â†’ formulaire ou Ã©vÃ©nement) dans un flux gouvernÃ©. ExÃ©cutable par MiyukiniAdmin.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Reference Outils | [MiyuWeb - Reference Outils](../MiyuWeb%20-%20Reference%20Outils.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../contracts/integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Tool Governance Compliance Contract | [MiyuWeb - Tool Governance Compliance Contract](../contracts/governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| MiyuWeb - Runtime Boundary Contract | [MiyuWeb - Runtime Boundary Contract](../contracts/boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) |
| MiyuWeb - Security and States Contract | [MiyuWeb - Security and States Contract](../contracts/security/MiyuWeb%20-%20Security%20and%20States%20Contract.md) |
| MiyuWeb - Dependencies Contract | [MiyuWeb - Dependencies Contract](../dependencies/MiyuWeb%20-%20Dependencies%20Contract.md) |
| MiyuWeb - Unit Tests Contract | [MiyuWeb - Unit Tests Contract](../contracts/testing/MiyuWeb%20-%20Unit%20Tests%20Contract.md) |
| MiyuWeb - Cycle Tests Contract | [MiyuWeb - Cycle Tests Contract](../contracts/testing/MiyuWeb%20-%20Cycle%20Tests%20Contract.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |
| Standardisation NumÃ©ration Invariants | [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif

