# docs/tools — Template Reference Implementation Guidelines

## Contexte

Ce document est le **template commun** pour les guides d'implémentation des Kits d'Outils (Toolkits) dans **docs/tools/**. Chaque toolkit prioritaire dispose d'un guide dérivé de ce template, adapté à son identité, ses Tools et ses contrats.

**Usage :** Copier ce template dans `<MiyuXXX>/implementation/MiyuXXX - Reference Implementation Guidelines.md` et remplacer les placeholders par les valeurs du kit (MiyuXXX, ToolkitId, domaine, liste des contrats sources, principes spécifiques, patterns).

**Référence :** [docs_tools - Audit Qualite Conformite Securite Implementation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md) — section 5 Guides d'implémentation.

---

## Structure type du guide (à adapter par kit)

### Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter le kit conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats du kit en logique d'implémentation (Tools, gouvernance, KindMother le cas échéant, bornage, sécurité).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

### 1. Introduction

#### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit **MiyuXXX** (résumer en une phrase : domaine et familles de Tools) de manière conforme aux contrats : Documentation Fondatrice, [énumérer les contrats spécifiques du kit : Tool Governance Compliance, KindMother Integration si présent, Security and States si présent, Runtime Boundary si présent, Dependencies si présent, Tests si présent].

#### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

#### 1.3 Sources contractuelles

- **MiyuXXX - Documentation Fondatrice** : ToolkitId, liste des Tools, gouvernance, relation KindMother le cas échéant.
- **MiyuXXX - Reference Outils** : Détail de chaque ToolId.
- **MiyuXXX - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spécifiques.
- *[Si applicable]* **MiyuXXX - KindMother Integration Contract** : Règles d'accès données, WriteIntent.
- *[Si applicable]* **MiyuXXX - Security and States Contract** : Niveau de sécurité, états autorisés/interdits.
- *[Si applicable]* **MiyuXXX - Runtime Boundary Contract** : Bornage, interdictions (BOUND-*).
- *[Si applicable]* **MiyuXXX - Dependencies Contract** : Liste fermée des dépendances.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

### 2. Principes à respecter absolument

#### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

**Principe contractuel :** Le kit ne décide pas si une action doit être faite ; StrongFather (ou les Cores) décide(nt) ALLOW/DENY.

**Traduction en implémentation :**

- Le kit est invoqué uniquement après décision ALLOW de la gouvernance. L'implémentation ne doit pas ré-évaluer les permissions.
- En cas d'appel hors gouvernance (anomalie), refuser l'exécution et signaler (pas de décision de contournement).

#### 2.2 Pas de choix métier (BOUND-2)

**Principe contractuel :** Le kit ne décide pas du contenu, du périmètre ou des règles métier ; il exécute sur ce qui lui est fourni dans le flux.

**Traduction en implémentation :**

- Les Tools exécutent uniquement sur les données et paramètres fournis en entrée. Aucune interprétation métier (choix de contenu, pertinence, politique) dans le code du kit.

#### 2.3 Pas d'accès direct non gouverné (BOUND-3)

**Principe contractuel :** Aucune lecture/écriture directe de données métier hors flux gouverné. Toute écriture métier = **WriteIntent** vers KindMother (si le kit produit des écritures).

**Traduction en implémentation :**

- *[Si le kit écrit des données métier]* : Aucun accès direct à la base ; toute écriture passe par WriteIntent vers KindMother (ou mécanisme documenté par le contrat KindMother).
- *[Si le kit ne persiste pas de données métier]* : Ne pas créer de persistance métier ; état technique sous contrôle du flux gouverné.

#### 2.4 Pas de modification du contexte d'autorisation (BOUND-4)

**Principe contractuel :** Le kit ne modifie pas les mandats, permissions ni le contexte d'autorisation.

**Traduction en implémentation :** Lecture seule du contexte ; pas d'émission de mandat, pas de révocation.

#### 2.5 Pas de connaissance de l'Opérateur appelant (BOUND-5)

**Principe contractuel :** Le kit ne raisonne pas sur l'identité métier de l'Opérateur ; contexte anonymisé (permissions, niveau de sécurité).

**Traduction en implémentation :** Pas d'identité Opérateur dans la logique Tool ; uniquement contexte gouverné (niveau, permissions).

#### 2.6 Pas de capacité nouvelle (BOUND-6)

**Principe contractuel :** Chaque Tool correspond exactement à un ToolId déclaré dans le catalogue ; pas d'extension non déclarée.

**Traduction en implémentation :** Implémenter uniquement les ToolIds listés dans la Documentation Fondatrice et Reference Outils ; pas de Tool additionnel non déclaré.

#### 2.7 Sécurité et états (si applicable)

*[Adapter selon le kit : niveau 0–4, états HEALTHY/DEGRADED/SECURITY_LOCKDOWN/MAINTENANCE. Si sanitization (XSS, CSP) ou envoi externe : rappeler les règles du contrat Security ou de la Doc Fondatrice.]*

#### 2.8 Dépendances (si contrat Dependencies)

**Principe contractuel :** Le kit ne dépend que des Cores et du Kernel (ou liste fermée définie dans le contrat).

**Traduction en implémentation :** Aucune dépendance vers un Opérateur ou une règle métier ; appels entrants passent par BondingBrother ; usage du Kernel (Id, Logger, Clock, Config, Lifecycle) pour technique uniquement.

#### 2.9 Alignement MIP/MSCM

**Principe contractuel :** Domaine et layer cohérents avec le ToolkitId ; chaque Tool = bloc logique (id, do, role, layer).

**Traduction en implémentation :** À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

### 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat |
| **BOUND-2** | Pas de choix métier | Exécution sur données/paramètres fournis |
| **BOUND-3** | Pas d'accès direct non gouverné | WriteIntent KindMother ou pas de persistance métier |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé (niveau, permissions) |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés |

---

### 4. Patterns recommandés

#### 4.1 Structure des Tools

- Chaque ToolId est implémenté comme une unité d'exécution atomique : entrée (contexte gouverné, paramètres), sortie (résultat ou erreur contractuelle).
- Pas d'état métier partagé entre appels ; état technique sous contrôle du flux gouverné.
- Format ToolId : cohérent avec [MiyuXXX - Reference Outils](../MiyuXXX/MiyuXXX%20-%20Reference%20Outils.md) (ex. `tool.<domaine>.<action>`, `mws.*` pour Webway).

#### 4.2 Interface avec le flux (KindMother / données en flux)

- *[Si le kit lit/écrit des données métier]* : Consommer des données **déjà présentes dans le flux** ou produire des **WriteIntent** vers KindMother ; aucun accès direct à MiyuSQL ou à la base depuis le kit.
- *[Si le kit ne persiste pas de données métier]* : Entrées/sorties uniquement via le flux gouverné ; pas d'écriture métier.

#### 4.3 Gestion des erreurs

- Erreurs techniques remontées de manière explicite sans exposer de données métier sensibles.
- En cas de violation de bornage (appel sans mandat, tentative d'accès direct), refus d'exécution et signal (observability).

#### 4.4 Traçabilité

- Utiliser le Logger du Kernel pour tracer les exécutions (sans contenu métier sensible).

#### 4.5 Spécificités du kit

*[À compléter par kit : ex. sanitization pour rendu HTML, ports MWS, format déclarations, etc.]*

---

### 5. Alignement MIP / MSCM

#### 5.1 MIP v1

À l'implémentation, le code fournissant les Tools du kit doit être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

- **Domaine** : cohérent avec le ToolkitId (ex. `community`, `notify`, `search`, `webway`).
- **Layer** : Strate 6 (outil / toolkit) dans layers.json.
- **Blocs** : Chaque Tool est une unité logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json.

#### 5.2 MSCM

Les blocs de code correspondant aux Tools doivent être balisés selon le standard MSCM (Miyukini Semantic Code Markup) pour permettre l'indexation et la gouvernance structurelle.

---

### 6. Tests (rappel)

*[Si le kit dispose de contrats de tests (Unit Tests, Cycle Tests), les lister et rappeler les critères. Sinon : « Les tests relèvent des bonnes pratiques projet et du Tool Governance Compliance Contract (obligations de non-régression, états). »]*

---

### 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuXXX - Documentation Fondatrice | [MiyuXXX - Documentation Fondatrice](../MiyuXXX/MiyuXXX%20-%20Documentation%20Fondatrice.md) |
| MiyuXXX - Reference Outils | [MiyuXXX - Reference Outils](../MiyuXXX/MiyuXXX%20-%20Reference%20Outils.md) |
| MiyuXXX - Tool Governance Compliance Contract | [MiyuXXX - Tool Governance Compliance Contract](../MiyuXXX/contracts/governance/MiyuXXX%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| *[Contrats additionnels si présents]* | *[Liens]* |
| Tools et Toolkits (référence conceptuelle) | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date du template :** 2026-01-30  
**Version :** 1.0  
**Statut :** Template — à adapter par kit dans `<MiyuXXX>/implementation/MiyuXXX - Reference Implementation Guidelines.md`
