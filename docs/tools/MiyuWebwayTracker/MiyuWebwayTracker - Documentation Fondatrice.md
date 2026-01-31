# MiyuWebwayTracker — Documentation Fondatrice

## 1. Contexte

**MiyuWebwayTracker** est le **Kit d'Outils Tracker Webway (MWS)** de l'écosystème Miyukini. Il intègre les outils MWS nécessaires pour **tenir le rôle Tracker** (recevoir, valider, répondre aux requêtes de découverte, maintenir et échanger les listes de statuts), alignés sur [Miyukini Webway System - Outils et Opérateurs](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md).

Les décisions (accepter ou rejeter une déclaration, politique de filtrage, statuts) relèvent des **Cores** (Border Guard, WorrySentinel, StrongFather). Le Kit Tracker **ne décide pas** ; il fournit les capacités (validate, verify, transport.receive/send, discovery.response.*, cog_list.*, port.check).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique du Kit Tracker Webway, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité.

**Hors scope :** le Kit Participant Webway (MiyuWebwayParticipant) ; les normes et standards MWS (format, protocole, ports) ; l'implémentation détaillée (systèmes passifs/actifs).

---

## 3. Définition canonique

> **MiyuWebwayTracker est une composition officielle d'outils MWS pour tenir le rôle Tracker (recevoir, valider, répondre à la découverte, maintenir listes de statuts), déclarée et gouvernée par l'environnement.**

- MiyuWebwayTracker **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Outils MWS existants.
- MiyuWebwayTracker **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; **décisions (accepter, rejeter, filtrer) = Cores** (Border Guard, WorrySentinel) via BondingBrother.

**Règle fondamentale :** Le Kit **ne décide pas** d'accepter ou rejeter une déclaration ; il **exécute** les validations, vérifications et filtrages ; les politiques viennent des Cores.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.webway.tracker` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `webway` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuWebwayTracker - Reference Outils](./MiyuWebwayTracker%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `mws.declaration.validate` | Valider le format d'une déclaration reçue |
| `mws.declaration.verify` | Vérifier la signature d'une déclaration reçue |
| `mws.transport.receive` | Recevoir un message sur un endpoint (ex. port 21000) |
| `mws.transport.send` | Envoyer un message (réponse, liste de statuts) |
| `mws.discovery.response.build` | Construire une réponse de découverte (liste COGs filtrée) |
| `mws.discovery.response.send` | Envoyer la réponse au demandeur |
| `mws.cog_list.get` | Lire la liste locale de COGs avec statuts |
| `mws.cog_list.update` | Mettre à jour une entrée dans la liste locale |
| `mws.cog_list.merge` | Fusionner une liste reçue avec la liste locale |
| `mws.cog_list.filter` | Filtrer la liste selon critère (ex. exclure Rejected) ; critère fourni par Cores |
| `mws.port.check` | Vérifier si un port est dans la liste des ports exclus MWS |
| `mws.address.tracker_default` | Résoudre l'adresse complète d'un Tracker (port 21000) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuWebwayTracker en contient douze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décisions (accepter, rejeter, filtrer) = Border Guard, WorrySentinel** ; traduction en intentions = BondingBrother ; le Kit exécute uniquement.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **2 à 3** (réseau, déclarations, découverte, protection maillage) |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon WorrySentinel) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` ; WorrySentinel peut bloquer le rôle Tracker |

---

## 8. Relation avec KindMother

Les Outils MWS du Kit Tracker **ne persistent pas** de données métier dans KindMother au sens classique ; la liste locale de COGs avec statuts peut être en mémoire ou persistée selon l'implémentation. Les règles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel).

Les obligations de conformité détaillées sont dans [MiyuWebwayTracker - Tool Governance Compliance Contract](./contracts/governance/MiyuWebwayTracker%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuWebwayTracker sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Outils MWS MiyuWebwayTracker devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
