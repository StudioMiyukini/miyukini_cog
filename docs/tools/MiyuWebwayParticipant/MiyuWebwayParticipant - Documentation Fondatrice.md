# MiyuWebwayParticipant — Documentation Fondatrice

## 1. Contexte

**MiyuWebwayParticipant** est le **Kit d'Outils Participant Webway (MWS)** de l'écosystème Miyukini. Il intègre les outils MWS nécessaires pour **participer** au maillage (annoncer, découvrir, maintenir la liste de statuts), alignés sur [Miyukini Webway System - Outils et Opérateurs](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md).

Les décisions (quand annoncer, à quels Trackers envoyer, politique de découverte) relèvent des **Cores** (StrongFather, Border Guard, WorrySentinel) et sont traduites en intentions par **BondingBrother**. Le Kit Participant **ne décide pas** ; il fournit les capacités (build, sign, validate, verify, transport.send, discovery.request.*, cog_list.*, port.check, address.tracker_default).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique du Kit Participant Webway, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité.

**Hors scope :** le Kit Tracker Webway (MiyuWebwayTracker) ; les normes et standards MWS (format, protocole, ports) ; l'implémentation détaillée (binding transport).

---

## 3. Définition canonique

> **MiyuWebwayParticipant est une composition officielle d'outils MWS pour participer au maillage (annoncer, découvrir, maintenir liste de statuts), déclarée et gouvernée par l'environnement.**

- MiyuWebwayParticipant **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Outils MWS existants.
- MiyuWebwayParticipant **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; **décisions = Cores** (StrongFather, Border Guard, WorrySentinel) via BondingBrother.

**Règle fondamentale :** Le Kit **ne décide pas** quand annoncer ni à quels Trackers envoyer ; il **exécute** les intentions fournies par les Cores.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.webway.participant` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `webway` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuWebwayParticipant - Reference Outils](./MiyuWebwayParticipant%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `mws.declaration.build` | Construire un message de déclaration conforme MWS |
| `mws.declaration.sign` | Signer une déclaration |
| `mws.declaration.validate` | Valider le format d'une déclaration |
| `mws.declaration.verify` | Vérifier la signature d'une déclaration |
| `mws.transport.send` | Envoyer un message vers une adresse (Tracker ou COG) |
| `mws.discovery.request.build` | Construire une requête de découverte |
| `mws.discovery.request.send` | Envoyer une requête de découverte vers un/des Tracker(s) |
| `mws.cog_list.get` | Lire la liste locale de COGs avec statuts |
| `mws.cog_list.update` | Mettre à jour une entrée dans la liste locale |
| `mws.cog_list.merge` | Fusionner une liste reçue avec la liste locale |
| `mws.port.check` | Vérifier si un port est dans la liste des ports exclus MWS |
| `mws.address.tracker_default` | Résoudre l'adresse complète d'un Tracker (port 21000) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuWebwayParticipant en contient douze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décisions (annoncer, découvrir, politique) = StrongFather, Border Guard, WorrySentinel** ; traduction en intentions = BondingBrother ; le Kit exécute uniquement.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **2 à 3** (réseau, déclarations, découverte) |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon WorrySentinel) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` ; WorrySentinel peut bloquer ou dégrader la participation MWS |

---

## 8. Relation avec KindMother

Les Outils MWS du Kit Participant **ne persistent pas** de données métier dans KindMother au sens classique ; la liste locale de COGs avec statuts peut être en mémoire ou persistée selon l'implémentation. Les règles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel).

Les obligations de conformité détaillées sont dans [MiyuWebwayParticipant - Tool Governance Compliance Contract](./contracts/governance/MiyuWebwayParticipant%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuWebwayParticipant sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Outils MWS MiyuWebwayParticipant devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Webway System - Outils et Opérateurs | [Miyukini Webway System - Outils et Operateurs](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
