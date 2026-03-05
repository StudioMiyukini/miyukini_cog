# docs/tools â€” Template Reference Implementation Guidelines

## Contexte

Ce document est le **template commun** pour les guides d'implÃ©mentation des Kits d'Outils (Toolkits) dans **docs/tools/**. Chaque toolkit prioritaire dispose d'un guide dÃ©rivÃ© de ce template, adaptÃ© Ã  son identitÃ©, ses Tools et ses contrats.

**Usage :** Copier ce template dans `<MiyuXXX>/implementation/MiyuXXX - Reference Implementation Guidelines.md` et remplacer les placeholders par les valeurs du kit (MiyuXXX, ToolkitId, domaine, liste des contrats sources, principes spÃ©cifiques, patterns).

**RÃ©fÃ©rence :** [docs_tools - Audit Qualite Conformite Securite Implementation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md) â€” section 5 Guides d'implÃ©mentation.

---

## Structure type du guide (Ã  adapter par kit)

### Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter le kit conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats du kit en logique d'implÃ©mentation (Tools, gouvernance, KindMother le cas Ã©chÃ©ant, bornage, sÃ©curitÃ©).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

### 1. Introduction

#### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit **MiyuXXX** (rÃ©sumer en une phrase : domaine et familles de Tools) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, [Ã©numÃ©rer les contrats spÃ©cifiques du kit : Tool Governance Compliance, KindMother Integration si prÃ©sent, Security and States si prÃ©sent, Runtime Boundary si prÃ©sent, Dependencies si prÃ©sent, Tests si prÃ©sent].

#### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

#### 1.3 Sources contractuelles

- **MiyuXXX - Documentation Fondatrice** : ToolkitId, liste des Tools, gouvernance, relation KindMother le cas Ã©chÃ©ant.
- **MiyuXXX - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuXXX - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spÃ©cifiques.
- *[Si applicable]* **MiyuXXX - KindMother Integration Contract** : RÃ¨gles d'accÃ¨s donnÃ©es, WriteIntent.
- *[Si applicable]* **MiyuXXX - Security and States Contract** : Niveau de sÃ©curitÃ©, Ã©tats autorisÃ©s/interdits.
- *[Si applicable]* **MiyuXXX - Runtime Boundary Contract** : Bornage, interdictions (BOUND-*).
- *[Si applicable]* **MiyuXXX - Dependencies Contract** : Liste fermÃ©e des dÃ©pendances.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

### 2. Principes Ã  respecter absolument

#### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

**Principe contractuel :** Le kit ne dÃ©cide pas si une action doit Ãªtre faite ; StrongFather (ou les Cores) dÃ©cide(nt) ALLOW/DENY.

**Traduction en implÃ©mentation :**

- Le kit est invoquÃ© uniquement aprÃ¨s dÃ©cision ALLOW de la gouvernance. L'implÃ©mentation ne doit pas rÃ©-Ã©valuer les permissions.
- En cas d'appel hors gouvernance (anomalie), refuser l'exÃ©cution et signaler (pas de dÃ©cision de contournement).

#### 2.2 Pas de choix mÃ©tier (BOUND-2)

**Principe contractuel :** Le kit ne dÃ©cide pas du contenu, du pÃ©rimÃ¨tre ou des rÃ¨gles mÃ©tier ; il exÃ©cute sur ce qui lui est fourni dans le flux.

**Traduction en implÃ©mentation :**

- Les Tools exÃ©cutent uniquement sur les donnÃ©es et paramÃ¨tres fournis en entrÃ©e. Aucune interprÃ©tation mÃ©tier (choix de contenu, pertinence, politique) dans le code du kit.

#### 2.3 Pas d'accÃ¨s direct non gouvernÃ© (BOUND-3)

**Principe contractuel :** Aucune lecture/Ã©criture directe de donnÃ©es mÃ©tier hors flux gouvernÃ©. Toute Ã©criture mÃ©tier = **WriteIntent** vers KindMother (si le kit produit des Ã©critures).

**Traduction en implÃ©mentation :**

- *[Si le kit Ã©crit des donnÃ©es mÃ©tier]* : Aucun accÃ¨s direct Ã  la base ; toute Ã©criture passe par WriteIntent vers KindMother (ou mÃ©canisme documentÃ© par le contrat KindMother).
- *[Si le kit ne persiste pas de donnÃ©es mÃ©tier]* : Ne pas crÃ©er de persistance mÃ©tier ; Ã©tat technique sous contrÃ´le du flux gouvernÃ©.

#### 2.4 Pas de modification du contexte d'autorisation (BOUND-4)

**Principe contractuel :** Le kit ne modifie pas les mandats, permissions ni le contexte d'autorisation.

**Traduction en implÃ©mentation :** Lecture seule du contexte ; pas d'Ã©mission de mandat, pas de rÃ©vocation.

#### 2.5 Pas de connaissance de l'OpÃ©rateur appelant (BOUND-5)

**Principe contractuel :** Le kit ne raisonne pas sur l'identitÃ© mÃ©tier de l'OpÃ©rateur ; contexte anonymisÃ© (permissions, niveau de sÃ©curitÃ©).

**Traduction en implÃ©mentation :** Pas d'identitÃ© OpÃ©rateur dans la logique Tool ; uniquement contexte gouvernÃ© (niveau, permissions).

#### 2.6 Pas de capacitÃ© nouvelle (BOUND-6)

**Principe contractuel :** Chaque Tool correspond exactement Ã  un ToolId dÃ©clarÃ© dans le catalogue ; pas d'extension non dÃ©clarÃ©e.

**Traduction en implÃ©mentation :** ImplÃ©menter uniquement les ToolIds listÃ©s dans la Documentation Fondatrice et Reference Outils ; pas de Tool additionnel non dÃ©clarÃ©.

#### 2.7 SÃ©curitÃ© et Ã©tats (si applicable)

*[Adapter selon le kit : niveau 0â€“4, Ã©tats HEALTHY/DEGRADED/SECURITY_LOCKDOWN/MAINTENANCE. Si sanitization (XSS, CSP) ou envoi externe : rappeler les rÃ¨gles du contrat Security ou de la Doc Fondatrice.]*

#### 2.8 DÃ©pendances (si contrat Dependencies)

**Principe contractuel :** Le kit ne dÃ©pend que des Cores et du Kernel (ou liste fermÃ©e dÃ©finie dans le contrat).

**Traduction en implÃ©mentation :** Aucune dÃ©pendance vers un OpÃ©rateur ou une rÃ¨gle mÃ©tier ; appels entrants passent par BondingBrother ; usage du Kernel (Id, Logger, Clock, Config, Lifecycle) pour technique uniquement.

#### 2.9 Alignement MIP/MSCM

**Principe contractuel :** Domaine et layer cohÃ©rents avec le ToolkitId ; chaque Tool = bloc logique (id, do, role, layer).

**Traduction en implÃ©mentation :** Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

### 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat |
| **BOUND-2** | Pas de choix mÃ©tier | ExÃ©cution sur donnÃ©es/paramÃ¨tres fournis |
| **BOUND-3** | Pas d'accÃ¨s direct non gouvernÃ© | WriteIntent KindMother ou pas de persistance mÃ©tier |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© (niveau, permissions) |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds dÃ©clarÃ©s |

---

### 4. Patterns recommandÃ©s

#### 4.1 Structure des Tools

- Chaque ToolId est implÃ©mentÃ© comme une unitÃ© d'exÃ©cution atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres), sortie (rÃ©sultat ou erreur contractuelle).
- Pas d'Ã©tat mÃ©tier partagÃ© entre appels ; Ã©tat technique sous contrÃ´le du flux gouvernÃ©.
- Format ToolId : cohÃ©rent avec [MiyuXXX - Reference Outils](..//_index.md) (ex. `tool.<domaine>.<action>`, `mws.*` pour Webway).

#### 4.2 Interface avec le flux (KindMother / donnÃ©es en flux)

- *[Si le kit lit/Ã©crit des donnÃ©es mÃ©tier]* : Consommer des donnÃ©es **dÃ©jÃ  prÃ©sentes dans le flux** ou produire des **WriteIntent** vers KindMother ; aucun accÃ¨s direct Ã  MiyuSQL ou Ã  la base depuis le kit.
- *[Si le kit ne persiste pas de donnÃ©es mÃ©tier]* : EntrÃ©es/sorties uniquement via le flux gouvernÃ© ; pas d'Ã©criture mÃ©tier.

#### 4.3 Gestion des erreurs

- Erreurs techniques remontÃ©es de maniÃ¨re explicite sans exposer de donnÃ©es mÃ©tier sensibles.
- En cas de violation de bornage (appel sans mandat, tentative d'accÃ¨s direct), refus d'exÃ©cution et signal (observability).

#### 4.4 TraÃ§abilitÃ©

- Utiliser le Logger du Kernel pour tracer les exÃ©cutions (sans contenu mÃ©tier sensible).

#### 4.5 SpÃ©cificitÃ©s du kit

*[Ã€ complÃ©ter par kit : ex. sanitization pour rendu HTML, ports MWS, format dÃ©clarations, etc.]*

---

### 5. Alignement MIP / MSCM

#### 5.1 MIP v1

Ã€ l'implÃ©mentation, le code fournissant les Tools du kit doit Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : cohÃ©rent avec le ToolkitId (ex. `community`, `notify`, `search`, `webway`).
- **Layer** : Strate 6 (outil / toolkit) dans layers.json.
- **Blocs** : Chaque Tool est une unitÃ© logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json.

#### 5.2 MSCM

Les blocs de code correspondant aux Tools doivent Ãªtre balisÃ©s selon le standard MSCM (Miyukini Semantic Code Markup) pour permettre l'indexation et la gouvernance structurelle.

---

### 6. Tests (rappel)

*[Si le kit dispose de contrats de tests (Unit Tests, Cycle Tests), les lister et rappeler les critÃ¨res. Sinon : Â« Les tests relÃ¨vent des bonnes pratiques projet et du Tool Governance Compliance Contract (obligations de non-rÃ©gression, Ã©tats). Â»]*

---

### 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuXXX - Documentation Fondatrice | [MiyuXXX - Documentation Fondatrice](..//_index.md) |
| MiyuXXX - Reference Outils | [MiyuXXX - Reference Outils](..//_index.md) |
| MiyuXXX - Tool Governance Compliance Contract | [MiyuXXX - Tool Governance Compliance Contract](..//_index.md) |
| *[Contrats additionnels si prÃ©sents]* | *[Liens]* |
| Tools et Toolkits (rÃ©fÃ©rence conceptuelle) | [Miyukini Conceptual References - Tools et Toolkits](..//miyukini-webway-system//reference//_index.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date du template :** 2026-01-30  
**Version :** 1.0  
**Statut :** Template â€” Ã  adapter par kit dans `<MiyuXXX>/implementation/MiyuXXX - Reference Implementation Guidelines.md`


