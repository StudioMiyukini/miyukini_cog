# MiyuWebwayTracker — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter le Kit Tracker Webway (MiyuWebwayTracker) conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MWS Tracker en logique d'implémentation (Outils MWS : validate, verify, transport.receive|send, discovery.response.*, cog_list.*, filter, port.check, address.tracker_default).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

## Portée / Scope

- Lignes directrices d'implémentation pour le Kit MiyuWebwayTracker (réception, validation, vérification, découverte, listes de statuts).
- Document informatif et non contractuel ; les contrats fondateurs priment.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuWebwayTracker (réception déclarations, validation, vérification, réponse découverte, liste locale COGs, filtrage, port, adresse Tracker) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuWebwayTracker - Documentation Fondatrice** : ToolkitId `toolkit.webway.tracker`, liste des Outils MWS (declaration.validate|verify, transport.receive|send, discovery.response.*, cog_list.*|filter, port.check, address.tracker_default), gouvernance (décisions accepter/rejeter/filtrer = Cores via BondingBrother).
- **MiyuWebwayTracker - Reference Outils** : Détail de chaque ToolId.
- **MiyuWebwayTracker - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spécifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.
- **Miyukini Webway System - Outils et Opérateurs** : Normes MWS (format déclaration, découverte, ports).

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision (BOUND-1)

**Décisions (accepter, rejeter, filtrer) = Border Guard, WorrySentinel, StrongFather via BondingBrother.** Le Kit Tracker **ne décide pas** d'accepter ou rejeter une déclaration ; il **exécute** les validations, vérifications et filtrages ; les politiques viennent des Cores. L'implémentation ne doit pas ré-évaluer les politiques. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Outils MWS exécutent sur les données fournies (déclaration reçue, liste COGs, critère de filtrage). Aucune décision sur accepter/rejeter une déclaration ou sur le filtrage métier — ressort des Cores (Border Guard, WorrySentinel). Le critère de filtrage (ex. exclure Rejected) est **fourni par les Cores**.

### 2.3 Pas de persistance métier KindMother (BOUND-3)

Les Outils MWS du Kit Tracker **ne persistent pas** de données métier dans KindMother au sens classique. La liste locale de COGs avec statuts peut être **en mémoire ou persistée selon l'implémentation** ; les règles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel). Aucune écriture métier directe vers KindMother.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds MWS déclarés (mws.declaration.validate|verify, mws.transport.receive|send, mws.discovery.response.*, mws.cog_list.*|filter, mws.port.check, mws.address.tracker_default).

### 2.7 Niveau de sécurité et états

Niveau **2 à 3** (réseau, déclarations, découverte, protection maillage). États autorisés : `HEALTHY`, `DEGRADED` (selon WorrySentinel). États interdits : `SECURITY_LOCKDOWN`, `MAINTENANCE` ; WorrySentinel peut bloquer le rôle Tracker. Vérifier l'état avant exécution.

### 2.8 Alignement MIP/MSCM

Domaine `webway`, layer Strate 6. À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](../../../contrats/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision | Décisions (accepter, rejeter, filtrer) = Cores ; exécution sur intentions/critères fournis uniquement |
| **BOUND-2** | Pas de choix métier | Pas de décision accepter/rejeter déclaration ni politique de filtrage ; critère fourni par Cores |
| **BOUND-3** | Pas de persistance métier KindMother | Liste COGs en mémoire ou persistée selon implémentation ; règles fusion/filtrage = Cores |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds MWS déclarés |

---

## 4. Patterns recommandés

### 4.1 Structure des Tools MWS

Chaque ToolId = unité atomique : entrée (contexte gouverné, paramètres : déclaration, liste COGs, critère filtre), sortie (résultat ou erreur). Pas d'état métier partagé. Format : `mws.declaration.validate|verify`, `mws.transport.receive|send`, `mws.discovery.response.build|send`, `mws.cog_list.get|update|merge|filter`, `mws.port.check`, `mws.address.tracker_default`.

### 4.2 Interface avec le flux (pas KindMother métier)

- **declaration.validate|verify** : Valider le format et vérifier la signature d'une déclaration **reçue** ; données fournies dans le flux. Pas de décision accepter/rejeter — résultat (valide/invalide) utilisé en amont par les Cores.
- **transport.receive|send** : Recevoir un message sur un endpoint (ex. port 21000) ; envoyer une réponse ou une liste de statuts ; paramètres fournis par les Cores.
- **discovery.response.build|send** : Construire une réponse de découverte (liste COGs **filtrée**) ; critère de filtrage **fourni par les Cores** ; envoyer la réponse au demandeur.
- **cog_list.get|update|merge|filter** : Lire, mettre à jour, fusionner, filtrer la liste locale de COGs avec statuts ; critère de filtrage (ex. exclure Rejected) **fourni par les Cores**.
- **port.check** : Vérifier si un port est dans la liste des ports exclus MWS.
- **address.tracker_default** : Résoudre l'adresse complète d'un Tracker (port 21000).

### 4.3 Liste locale COGs et filtrage

La liste locale de COGs avec statuts peut être en mémoire ou persistée selon l'implémentation. Les règles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel). **mws.cog_list.filter** applique un critère **fourni par les Cores** ; le Kit n'invente pas de règle de filtrage.

### 4.4 Gestion des erreurs et traçabilité

Erreurs techniques (déclaration invalide, transport échoué) remontées sans exposer de données sensibles. Logger du Kernel pour traçabilité (sans contenu des déclarations si sensible).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `webway` (toolkit.webway.tracker).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Outil MWS MiyuWebwayTracker = unité logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](../../../contrats/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Tests

Les tests relèvent des bonnes pratiques projet et du Tool Governance Compliance Contract. Scénarios recommandés : declaration.validate|verify sur déclaration reçue, transport.receive|send sur endpoint, discovery.response.build|send avec critère filtre fourni, cog_list.get|update|merge|filter avec critères Cores, port.check et address.tracker_default ; vérifier qu'aucune décision accepter/rejeter/filtrer n'est prise dans le kit.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuWebwayTracker - Documentation Fondatrice | [MiyuWebwayTracker - Documentation Fondatrice](../MiyuWebwayTracker%20-%20Documentation%20Fondatrice.md) |
| MiyuWebwayTracker - Reference Outils | [MiyuWebwayTracker - Reference Outils](../MiyuWebwayTracker%20-%20Reference%20Outils.md) |
| MiyuWebwayTracker - Tool Governance Compliance Contract | [MiyuWebwayTracker - Tool Governance Compliance Contract](../contracts/governance/MiyuWebwayTracker%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Miyukini Webway System - Outils et Opérateurs | [Miyukini Webway System - Outils et Opérateurs](../../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](../../../contrats/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
