# MiyuWebwayTracker â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter le Kit Tracker Webway (MiyuWebwayTracker) conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MWS Tracker en logique d'implÃ©mentation (Outils MWS : validate, verify, transport.receive|send, discovery.response.*, cog_list.*, filter, port.check, address.tracker_default).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

## PortÃ©e / Scope

- Lignes directrices d'implÃ©mentation pour le Kit MiyuWebwayTracker (rÃ©ception, validation, vÃ©rification, dÃ©couverte, listes de statuts).
- Document informatif et non contractuel ; les contrats fondateurs priment.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuWebwayTracker (rÃ©ception dÃ©clarations, validation, vÃ©rification, rÃ©ponse dÃ©couverte, liste locale COGs, filtrage, port, adresse Tracker) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuWebwayTracker - Documentation Fondatrice** : ToolkitId `toolkit.webway.tracker`, liste des Outils MWS (declaration.validate|verify, transport.receive|send, discovery.response.*, cog_list.*|filter, port.check, address.tracker_default), gouvernance (dÃ©cisions accepter/rejeter/filtrer = Cores via BondingBrother).
- **MiyuWebwayTracker - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuWebwayTracker - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spÃ©cifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.
- **Miyukini Webway System - Outils et OpÃ©rateurs** : Normes MWS (format dÃ©claration, dÃ©couverte, ports).

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision (BOUND-1)

**DÃ©cisions (accepter, rejeter, filtrer) = Border Guard, WorrySentinel, StrongFather via BondingBrother.** Le Kit Tracker **ne dÃ©cide pas** d'accepter ou rejeter une dÃ©claration ; il **exÃ©cute** les validations, vÃ©rifications et filtrages ; les politiques viennent des Cores. L'implÃ©mentation ne doit pas rÃ©-Ã©valuer les politiques. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Outils MWS exÃ©cutent sur les donnÃ©es fournies (dÃ©claration reÃ§ue, liste COGs, critÃ¨re de filtrage). Aucune dÃ©cision sur accepter/rejeter une dÃ©claration ou sur le filtrage mÃ©tier â€” ressort des Cores (Border Guard, WorrySentinel). Le critÃ¨re de filtrage (ex. exclure Rejected) est **fourni par les Cores**.

### 2.3 Pas de persistance mÃ©tier KindMother (BOUND-3)

Les Outils MWS du Kit Tracker **ne persistent pas** de donnÃ©es mÃ©tier dans KindMother au sens classique. La liste locale de COGs avec statuts peut Ãªtre **en mÃ©moire ou persistÃ©e selon l'implÃ©mentation** ; les rÃ¨gles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel). Aucune Ã©criture mÃ©tier directe vers KindMother.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds MWS dÃ©clarÃ©s (mws.declaration.validate|verify, mws.transport.receive|send, mws.discovery.response.*, mws.cog_list.*|filter, mws.port.check, mws.address.tracker_default).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **2 Ã  3** (rÃ©seau, dÃ©clarations, dÃ©couverte, protection maillage). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED` (selon WorrySentinel). Ã‰tats interdits : `SECURITY_LOCKDOWN`, `MAINTENANCE` ; WorrySentinel peut bloquer le rÃ´le Tracker. VÃ©rifier l'Ã©tat avant exÃ©cution.

### 2.8 Alignement MIP/MSCM

Domaine `webway`, layer Strate 6. Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision | DÃ©cisions (accepter, rejeter, filtrer) = Cores ; exÃ©cution sur intentions/critÃ¨res fournis uniquement |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision accepter/rejeter dÃ©claration ni politique de filtrage ; critÃ¨re fourni par Cores |
| **BOUND-3** | Pas de persistance mÃ©tier KindMother | Liste COGs en mÃ©moire ou persistÃ©e selon implÃ©mentation ; rÃ¨gles fusion/filtrage = Cores |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds MWS dÃ©clarÃ©s |

---

## 4. Patterns recommandÃ©s

### 4.1 Structure des Tools MWS

Chaque ToolId = unitÃ© atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres : dÃ©claration, liste COGs, critÃ¨re filtre), sortie (rÃ©sultat ou erreur). Pas d'Ã©tat mÃ©tier partagÃ©. Format : `mws.declaration.validate|verify`, `mws.transport.receive|send`, `mws.discovery.response.build|send`, `mws.cog_list.get|update|merge|filter`, `mws.port.check`, `mws.address.tracker_default`.

### 4.2 Interface avec le flux (pas KindMother mÃ©tier)

- **declaration.validate|verify** : Valider le format et vÃ©rifier la signature d'une dÃ©claration **reÃ§ue** ; donnÃ©es fournies dans le flux. Pas de dÃ©cision accepter/rejeter â€” rÃ©sultat (valide/invalide) utilisÃ© en amont par les Cores.
- **transport.receive|send** : Recevoir un message sur un endpoint (ex. port 21000) ; envoyer une rÃ©ponse ou une liste de statuts ; paramÃ¨tres fournis par les Cores.
- **discovery.response.build|send** : Construire une rÃ©ponse de dÃ©couverte (liste COGs **filtrÃ©e**) ; critÃ¨re de filtrage **fourni par les Cores** ; envoyer la rÃ©ponse au demandeur.
- **cog_list.get|update|merge|filter** : Lire, mettre Ã  jour, fusionner, filtrer la liste locale de COGs avec statuts ; critÃ¨re de filtrage (ex. exclure Rejected) **fourni par les Cores**.
- **port.check** : VÃ©rifier si un port est dans la liste des ports exclus MWS.
- **address.tracker_default** : RÃ©soudre l'adresse complÃ¨te d'un Tracker (port 21000).

### 4.3 Liste locale COGs et filtrage

La liste locale de COGs avec statuts peut Ãªtre en mÃ©moire ou persistÃ©e selon l'implÃ©mentation. Les rÃ¨gles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel). **mws.cog_list.filter** applique un critÃ¨re **fourni par les Cores** ; le Kit n'invente pas de rÃ¨gle de filtrage.

### 4.4 Gestion des erreurs et traÃ§abilitÃ©

Erreurs techniques (dÃ©claration invalide, transport Ã©chouÃ©) remontÃ©es sans exposer de donnÃ©es sensibles. Logger du Kernel pour traÃ§abilitÃ© (sans contenu des dÃ©clarations si sensible).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `webway` (toolkit.webway.tracker).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Outil MWS MiyuWebwayTracker = unitÃ© logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. Tests

Les tests relÃ¨vent des bonnes pratiques projet et du Tool Governance Compliance Contract. ScÃ©narios recommandÃ©s : declaration.validate|verify sur dÃ©claration reÃ§ue, transport.receive|send sur endpoint, discovery.response.build|send avec critÃ¨re filtre fourni, cog_list.get|update|merge|filter avec critÃ¨res Cores, port.check et address.tracker_default ; vÃ©rifier qu'aucune dÃ©cision accepter/rejeter/filtrer n'est prise dans le kit.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWebwayTracker - Documentation Fondatrice | [MiyuWebwayTracker - Documentation Fondatrice](../MiyuWebwayTracker%20-%20Documentation%20Fondatrice.md) |
| MiyuWebwayTracker - Reference Outils | [MiyuWebwayTracker - Reference Outils](../MiyuWebwayTracker%20-%20Reference%20Outils.md) |
| MiyuWebwayTracker - Tool Governance Compliance Contract | [MiyuWebwayTracker - Tool Governance Compliance Contract](../contracts/governance/MiyuWebwayTracker%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Miyukini Webway System - Outils et OpÃ©rateurs | [Miyukini Webway System - Outils et OpÃ©rateurs](..//..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif

