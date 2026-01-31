# MiyuWebwayParticipant — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter le Kit Participant Webway (MiyuWebwayParticipant) conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MWS Participant en logique d'implémentation (Outils MWS : build, sign, validate, verify, transport.send, discovery.request.*, cog_list.*, port.check, address.tracker_default).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuWebwayParticipant (déclaration MWS, transport, découverte, liste locale COGs, port, adresse Tracker) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuWebwayParticipant - Documentation Fondatrice** : ToolkitId `toolkit.webway.participant`, liste des Outils MWS (declaration.build|sign|validate|verify, transport.send, discovery.request.*, cog_list.*, port.check, address.tracker_default), gouvernance (décisions = Cores via BondingBrother).
- **MiyuWebwayParticipant - Reference Outils** : Détail de chaque ToolId.
- **MiyuWebwayParticipant - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spécifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.
- **Miyukini Webway System - Outils et Opérateurs** : Normes MWS (format déclaration, découverte, ports).

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision (BOUND-1)

**Décisions = Cores (StrongFather, Border Guard, WorrySentinel) via BondingBrother.** Le Kit Participant **ne décide pas** quand annoncer ni à quels Trackers envoyer ; il **exécute** les intentions fournies par les Cores. L'implémentation ne doit pas ré-évaluer les politiques (annoncer, découvrir). En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Outils MWS exécutent sur les données fournies (déclaration, adresses, liste COGs, critères de fusion). Aucune décision sur politique de découverte, filtrage ou acceptation — ressort des Cores (Border Guard, WorrySentinel).

### 2.3 Pas de persistance métier KindMother (BOUND-3)

Les Outils MWS du Kit Participant **ne persistent pas** de données métier dans KindMother au sens classique. La liste locale de COGs avec statuts peut être **en mémoire ou persistée selon l'implémentation** ; les règles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel). Aucune écriture métier directe vers KindMother.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds MWS déclarés (mws.declaration.*, mws.transport.send, mws.discovery.request.*, mws.cog_list.*, mws.port.check, mws.address.tracker_default).

### 2.7 Niveau de sécurité et états

Niveau **2 à 3** (réseau, déclarations, découverte). États autorisés : `HEALTHY`, `DEGRADED` (selon WorrySentinel). États interdits : `SECURITY_LOCKDOWN`, `MAINTENANCE` ; WorrySentinel peut bloquer ou dégrader la participation MWS. Vérifier l'état avant exécution.

### 2.8 Alignement MIP/MSCM

Domaine `webway`, layer Strate 6. À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision | Décisions (annoncer, découvrir, politique) = Cores ; exécution sur intentions fournies uniquement |
| **BOUND-2** | Pas de choix métier | Pas de décision politique découverte, filtrage, acceptation |
| **BOUND-3** | Pas de persistance métier KindMother | Liste COGs en mémoire ou persistée selon implémentation ; règles fusion/filtrage = Cores |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds MWS déclarés |

---

## 4. Patterns recommandés

### 4.1 Structure des Tools MWS

Chaque ToolId = unité atomique : entrée (contexte gouverné, paramètres : déclaration, adresse, liste COGs, critères), sortie (résultat ou erreur). Pas d'état métier partagé. Format : `mws.declaration.build|sign|validate|verify`, `mws.transport.send`, `mws.discovery.request.build|send`, `mws.cog_list.get|update|merge`, `mws.port.check`, `mws.address.tracker_default`.

### 4.2 Interface avec le flux (pas KindMother métier)

- **declaration.build|sign|validate|verify** : Construire, signer, valider format, vérifier signature selon normes MWS ; données fournies dans le flux.
- **transport.send** : Envoyer un message vers une adresse (Tracker ou COG) ; adresse et payload fournis dans le flux.
- **discovery.request.build|send** : Construire et envoyer une requête de découverte vers un/des Tracker(s) ; paramètres fournis par les Cores.
- **cog_list.get|update|merge** : Lire, mettre à jour, fusionner la liste locale de COGs avec statuts ; règles de fusion fournies par les Cores (Border Guard, WorrySentinel).
- **port.check** : Vérifier si un port est dans la liste des ports exclus MWS (normes MWS).
- **address.tracker_default** : Résoudre l'adresse complète d'un Tracker (port 21000).

### 4.3 Liste locale COGs

La liste locale de COGs avec statuts peut être en mémoire ou persistée selon l'implémentation ; les règles de fusion et de filtrage viennent des Cores. Le Kit n'applique pas de politique métier sur la liste.

### 4.4 Gestion des erreurs et traçabilité

Erreurs techniques (format invalide, transport échoué) remontées sans exposer de données sensibles (adresses, déclarations). Logger du Kernel pour traçabilité (sans contenu des déclarations si sensible).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `webway` (toolkit.webway.participant).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Outil MWS MiyuWebwayParticipant = unité logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Tests

Les tests relèvent des bonnes pratiques projet et du Tool Governance Compliance Contract. Scénarios recommandés : declaration.build|sign|validate|verify avec données en flux, transport.send et discovery.request avec adresses fournies, cog_list.get|update|merge avec règles Cores, port.check et address.tracker_default ; vérifier qu'aucune décision métier n'est prise dans le kit.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuWebwayParticipant - Documentation Fondatrice | [MiyuWebwayParticipant - Documentation Fondatrice](../MiyuWebwayParticipant%20-%20Documentation%20Fondatrice.md) |
| MiyuWebwayParticipant - Reference Outils | [MiyuWebwayParticipant - Reference Outils](../MiyuWebwayParticipant%20-%20Reference%20Outils.md) |
| MiyuWebwayParticipant - Tool Governance Compliance Contract | [MiyuWebwayParticipant - Tool Governance Compliance Contract](../contracts/governance/MiyuWebwayParticipant%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Miyukini Webway System - Outils et Opérateurs | [Miyukini Webway System - Outils et Opérateurs](../../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
