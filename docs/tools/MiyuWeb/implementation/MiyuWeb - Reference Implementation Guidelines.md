# MiyuWeb — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuWeb conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuWeb en logique d'implémentation (Tools, gouvernance, KindMother, bornage, sanitization/CSP).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuWeb (Tools rendu HTML, layout, thème, script, asset, formulaire, événements) de manière conforme aux contrats : Documentation Fondatrice, KindMother Integration, Tool Governance Compliance, Security and States, Runtime Boundary, Dependencies, Unit Tests, Cycle Tests.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuWeb - Documentation Fondatrice** : ToolkitId, liste des Tools, gouvernance, relation KindMother/MiyuSQL
- **MiyuWeb - KindMother Integration Contract** : Aucune lecture directe de la base ; données fournies dans le flux
- **MiyuWeb - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities
- **MiyuWeb - Security and States Contract** : Niveau 0, 1 ou 2, états autorisés/interdits, XSS/CSP
- **MiyuWeb - Runtime Boundary Contract** : Bornage, interdictions (BOUND-*), invariants INV-BOUND-*
- **MiyuWeb - Dependencies Contract** : Liste fermée des dépendances, ordre d'invocation
- **Master Butler - Tool Governance Contract** et **Toolkit Composition Contract** : Format ToolId, structure Toolkit

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

**Principe contractuel :** MiyuWeb ne décide pas si une action doit être faite ; StrongFather décide ALLOW/DENY.

**Traduction en implémentation :**

- MiyuWeb est invoqué uniquement après décision ALLOW de la gouvernance. L'implémentation ne doit pas ré-évaluer les permissions.
- En cas d'appel hors gouvernance (anomalie), MiyuWeb doit refuser l'exécution et signaler (pas de décision de contournement).

### 2.2 Pas de choix de contenu (BOUND-2)

**Principe contractuel :** MiyuWeb ne décide pas quel contenu afficher ; il rend, résout ou sert ce qui lui est fourni dans le flux.

**Traduction en implémentation :**

- Les Tools (html.render, layout.render, theme.resolve, asset.serve, etc.) ne choisissent jamais le contenu ; ils exécutent uniquement sur les données fournies en entrée.
- Aucune interprétation métier du contenu ; MiyuWeb ne décide pas des templates, thèmes ou règles applicatives.

### 2.3 Pas d'accès direct à la base (BOUND-3)

**Principe contractuel :** MiyuWeb ne lit jamais la base (templates, assets). Toutes les données sont fournies dans le flux gouverné.

**Traduction en implémentation :**

- L'implémentation ne doit contenir aucun appel direct à une base de données, à MiyuSQL, ni à un stockage persistant pour lire templates ou assets.
- Les entrées (template, contenu, métadonnées d'assets) sont toujours passées en paramètres du flux gouverné (éventuellement après lecture par MiyuSQL sous KindMother en amont).

### 2.4 Sanitization et CSP (Security Contract)

**Principe contractuel :** Tout contenu destiné au rendu HTML ou à l'exécution de script doit être traité selon la politique de sanitization ; l'implémentation doit être compatible avec les directives CSP définies par WorrySentinel / environnement.

**Traduction en implémentation :**

- **XSS :** Ne jamais injecter de contenu non gouverné dans le HTML ou les scripts ; appliquer la sanitization définie par l'environnement sur les données fournies dans le flux avant rendu ou exécution.
- **CSP :** Les Tools `tool.web.html.render` et `tool.web.script.execute` ne doivent pas contourner la Content Security Policy ; respecter les directives (sources autorisées, pas d'inline non autorisé, pas d'eval non gouverné).

### 2.5 Liste fermée des dépendances (INV-DEP-*)

**Principe contractuel :** MiyuWeb ne dépend que des Cores et du Kernel définis dans le Dependencies Contract.

**Traduction en implémentation :**

- Aucune dépendance vers un Opérateur, un produit, ou une règle métier.
- Les appels entrants passent par BondingBrother et la chaîne de gouvernance ; MiyuWeb n'expose pas d'API publique directe aux Opérateurs.
- Usage du Kernel (Id, Logger, Clock, Config, Lifecycle) pour identifiants, logs, horodatage, configuration locale, cycle de vie — sans logique métier.

### 2.6 Alignement MIP/MSCM

**Principe contractuel :** Domaine `web`, layer Strate 6 ; chaque Tool = bloc logique (id, do, role, layer).

**Traduction en implémentation :**

- À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le Protocole MIP v1.

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Pas de code qui évalue ALLOW/DENY ; exécution uniquement sur mandat |
| **BOUND-2** | Pas de choix de contenu | Pas de code qui choisit templates, thèmes ou contenu ; exécution sur données fournies |
| **BOUND-3** | Pas d'accès direct à la base | Pas de lecture DB, MiyuSQL, ni stockage pour templates/assets |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte ; pas de mandat, pas de révocation |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Pas d'identité Opérateur dans la logique Tool ; contexte anonymisé (permissions, niveau) |
| **BOUND-6** | Pas de capacité nouvelle | Chaque Tool correspond exactement à un ToolId déclaré ; pas d'extension non déclarée |

---

## 4. Patterns recommandés

### 4.1 Structure des Tools

- Chaque ToolId est implémenté comme une unité d'exécution atomique : entrée (contexte gouverné, paramètres : template, données, asset, etc.), sortie (résultat ou erreur contractuelle).
- Pas d'état métier partagé entre appels ; état technique sous contrôle du flux gouverné.
- Format ToolId : `tool.web.<sous-domaine>.<action>` ou `tool.web.<action>` (ex. `tool.web.html.render`, `tool.web.script.execute`).

### 4.2 Interface avec le flux (KindMother / données en flux)

- L'implémentation MiyuWeb consomme des données **déjà présentes dans le flux** : template, contenu, métadonnées d'assets, contexte thème, schéma formulaire, etc. Les paramètres incluent : type d'opération (render, resolve, execute, serve, validate, dispatch, capture), données fournies, contexte gouverné (sans identité Opérateur métier).
- La réponse inclut : succès/échec, résultat (HTML, structure layout, thème résolu, résultat script, asset servi, validation formulaire, événement dispatché/capturé), ou erreur explicite (sans fuite d'information sensible).
- Aucun appel à KindMother ou MiyuSQL depuis MiyuWeb ; les données ont été lues en amont et sont passées en entrée.

### 4.3 Gestion des erreurs

- Les erreurs techniques (template invalide, script mal formé, asset manquant dans le flux) sont remontées de manière explicite sans exposer de données métier.
- En cas de violation de bornage (ex. appel sans mandat, tentative d'accès direct à une source de données), refus d'exécution et signal (observability) ; pas d'exécution partielle.

### 4.4 Traçabilité

- Utiliser le Logger du Kernel pour tracer les exécutions (sans contenu métier sensible). Conformité aux contrats KindMother Observability et MiyuWeb Runtime Boundary.

### 4.5 Sandbox et scripts

- Pour `tool.web.script.execute` : exécution dans un contexte sandboxé ; aucun accès direct à la base ni décision métier depuis le script exécuté. Les entrées (données, contexte) sont celles fournies dans le flux.

---

## 5. Alignement MIP / MSCM

### 5.1 MIP v1

À l'implémentation, le code fournissant les Tools MiyuWeb doit être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

- **Domaine** : `web` (cohérent avec domains.json).
- **Layer** : Strate 6 (outil / toolkit) dans layers.json.
- **Blocs** : Chaque Tool MiyuWeb est une unité logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json.

### 5.2 MSCM

Les blocs de code correspondant aux Tools doivent être balisés selon le standard MSCM (Miyukini Semantic Code Markup) pour permettre l'indexation et la gouvernance structurelle.

### 5.3 Numérotation des invariants

Les contrats MiyuWeb utilisent des **préfixes catégoriels** pour les invariants : BOUND (bornage), DEP (dépendances), SEC (sécurité), INV-KM-* (intégration KindMother), INV-UT-MWEB-* (tests unitaires), INV-CT-MWEB-* (tests de cycle). Le format canonique des invariants des Cores (INV-&lt;PREFIX&gt;-&lt;NUMERO&gt;) est défini dans [Miyukini Conceptual References - Standardisation Numération Invariants](../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) ; les préfixes MiyuWeb restent cohérents en interne et distincts des codes Cores (KM, SF, etc.).

---

## 6. Tests (rappel)

- **Tests unitaires** : Conformément au [MiyuWeb - Unit Tests Contract](../contracts/testing/MiyuWeb%20-%20Unit%20Tests%20Contract.md) — pas de modification de données métier ; sandbox ou mocks pour templates/assets ; critères par Tool (render, script.execute, script.compile, asset.serve, theme.resolve, form.validate, event.dispatch, input.capture).
- **Tests de cycle MiyuWeb** : Conformément au [MiyuWeb - Cycle Tests Contract](../contracts/testing/MiyuWeb%20-%20Cycle%20Tests%20Contract.md) — scénario E2E (résolution thème → données template en flux → rendu HTML/layout → formulaire ou événement) dans un flux gouverné. Exécutable par MiyukiniAdmin.

---

## 7. Références croisées

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
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |
| Standardisation Numération Invariants | [Miyukini Conceptual References - Standardisation Numération Invariants](../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
