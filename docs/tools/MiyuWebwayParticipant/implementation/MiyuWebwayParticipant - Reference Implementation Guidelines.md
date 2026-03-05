# MiyuWebwayParticipant â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter le Kit Participant Webway (MiyuWebwayParticipant) conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MWS Participant en logique d'implÃ©mentation (Outils MWS : build, sign, validate, verify, transport.send, discovery.request.*, cog_list.*, port.check, address.tracker_default).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

## PortÃ©e / Scope

- Lignes directrices d'implÃ©mentation pour le Kit MiyuWebwayParticipant (dÃ©claration, transport, dÃ©couverte, listes de statuts).
- Document informatif et non contractuel ; les contrats fondateurs priment.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuWebwayParticipant (dÃ©claration MWS, transport, dÃ©couverte, liste locale COGs, port, adresse Tracker) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuWebwayParticipant - Documentation Fondatrice** : ToolkitId `toolkit.webway.participant`, liste des Outils MWS (declaration.build|sign|validate|verify, transport.send, discovery.request.*, cog_list.*, port.check, address.tracker_default), gouvernance (dÃ©cisions = Cores via BondingBrother).
- **MiyuWebwayParticipant - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuWebwayParticipant - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spÃ©cifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.
- **Miyukini Webway System - Outils et OpÃ©rateurs** : Normes MWS (format dÃ©claration, dÃ©couverte, ports).

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision (BOUND-1)

**DÃ©cisions = Cores (StrongFather, Border Guard, WorrySentinel) via BondingBrother.** Le Kit Participant **ne dÃ©cide pas** quand annoncer ni Ã  quels Trackers envoyer ; il **exÃ©cute** les intentions fournies par les Cores. L'implÃ©mentation ne doit pas rÃ©-Ã©valuer les politiques (annoncer, dÃ©couvrir). En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Outils MWS exÃ©cutent sur les donnÃ©es fournies (dÃ©claration, adresses, liste COGs, critÃ¨res de fusion). Aucune dÃ©cision sur politique de dÃ©couverte, filtrage ou acceptation â€” ressort des Cores (Border Guard, WorrySentinel).

### 2.3 Pas de persistance mÃ©tier KindMother (BOUND-3)

Les Outils MWS du Kit Participant **ne persistent pas** de donnÃ©es mÃ©tier dans KindMother au sens classique. La liste locale de COGs avec statuts peut Ãªtre **en mÃ©moire ou persistÃ©e selon l'implÃ©mentation** ; les rÃ¨gles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel). Aucune Ã©criture mÃ©tier directe vers KindMother.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds MWS dÃ©clarÃ©s (mws.declaration.*, mws.transport.send, mws.discovery.request.*, mws.cog_list.*, mws.port.check, mws.address.tracker_default).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **2 Ã  3** (rÃ©seau, dÃ©clarations, dÃ©couverte). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED` (selon WorrySentinel). Ã‰tats interdits : `SECURITY_LOCKDOWN`, `MAINTENANCE` ; WorrySentinel peut bloquer ou dÃ©grader la participation MWS. VÃ©rifier l'Ã©tat avant exÃ©cution.

### 2.8 Alignement MIP/MSCM

Domaine `webway`, layer Strate 6. Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision | DÃ©cisions (annoncer, dÃ©couvrir, politique) = Cores ; exÃ©cution sur intentions fournies uniquement |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision politique dÃ©couverte, filtrage, acceptation |
| **BOUND-3** | Pas de persistance mÃ©tier KindMother | Liste COGs en mÃ©moire ou persistÃ©e selon implÃ©mentation ; rÃ¨gles fusion/filtrage = Cores |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds MWS dÃ©clarÃ©s |

---

## 4. Patterns recommandÃ©s

### 4.1 Structure des Tools MWS

Chaque ToolId = unitÃ© atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres : dÃ©claration, adresse, liste COGs, critÃ¨res), sortie (rÃ©sultat ou erreur). Pas d'Ã©tat mÃ©tier partagÃ©. Format : `mws.declaration.build|sign|validate|verify`, `mws.transport.send`, `mws.discovery.request.build|send`, `mws.cog_list.get|update|merge`, `mws.port.check`, `mws.address.tracker_default`.

### 4.2 Interface avec le flux (pas KindMother mÃ©tier)

- **declaration.build|sign|validate|verify** : Construire, signer, valider format, vÃ©rifier signature selon normes MWS ; donnÃ©es fournies dans le flux.
- **transport.send** : Envoyer un message vers une adresse (Tracker ou COG) ; adresse et payload fournis dans le flux.
- **discovery.request.build|send** : Construire et envoyer une requÃªte de dÃ©couverte vers un/des Tracker(s) ; paramÃ¨tres fournis par les Cores.
- **cog_list.get|update|merge** : Lire, mettre Ã  jour, fusionner la liste locale de COGs avec statuts ; rÃ¨gles de fusion fournies par les Cores (Border Guard, WorrySentinel).
- **port.check** : VÃ©rifier si un port est dans la liste des ports exclus MWS (normes MWS).
- **address.tracker_default** : RÃ©soudre l'adresse complÃ¨te d'un Tracker (port 21000).

### 4.3 Liste locale COGs

La liste locale de COGs avec statuts peut Ãªtre en mÃ©moire ou persistÃ©e selon l'implÃ©mentation ; les rÃ¨gles de fusion et de filtrage viennent des Cores. Le Kit n'applique pas de politique mÃ©tier sur la liste.

### 4.4 Gestion des erreurs et traÃ§abilitÃ©

Erreurs techniques (format invalide, transport Ã©chouÃ©) remontÃ©es sans exposer de donnÃ©es sensibles (adresses, dÃ©clarations). Logger du Kernel pour traÃ§abilitÃ© (sans contenu des dÃ©clarations si sensible).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `webway` (toolkit.webway.participant).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Outil MWS MiyuWebwayParticipant = unitÃ© logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. Tests

Les tests relÃ¨vent des bonnes pratiques projet et du Tool Governance Compliance Contract. ScÃ©narios recommandÃ©s : declaration.build|sign|validate|verify avec donnÃ©es en flux, transport.send et discovery.request avec adresses fournies, cog_list.get|update|merge avec rÃ¨gles Cores, port.check et address.tracker_default ; vÃ©rifier qu'aucune dÃ©cision mÃ©tier n'est prise dans le kit.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWebwayParticipant - Documentation Fondatrice | [MiyuWebwayParticipant - Documentation Fondatrice](../MiyuWebwayParticipant%20-%20Documentation%20Fondatrice.md) |
| MiyuWebwayParticipant - Reference Outils | [MiyuWebwayParticipant - Reference Outils](../MiyuWebwayParticipant%20-%20Reference%20Outils.md) |
| MiyuWebwayParticipant - Tool Governance Compliance Contract | [MiyuWebwayParticipant - Tool Governance Compliance Contract](../contracts/governance/MiyuWebwayParticipant%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Miyukini Webway System - Outils et OpÃ©rateurs | [Miyukini Webway System - Outils et OpÃ©rateurs](..//..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif

