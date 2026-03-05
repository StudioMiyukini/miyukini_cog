# MiyuWebwayTracker â€” Documentation Fondatrice

## 1. Contexte

**MiyuWebwayTracker** est le **Kit d'Outils Tracker Webway (MWS)** de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils MWS nÃ©cessaires pour **tenir le rÃ´le Tracker** (recevoir, valider, rÃ©pondre aux requÃªtes de dÃ©couverte, maintenir et Ã©changer les listes de statuts), alignÃ©s sur [Miyukini Webway System - Outils et OpÃ©rateurs](..//..//miyukini-webway-system//reference//_index.md).

Les dÃ©cisions (accepter ou rejeter une dÃ©claration, politique de filtrage, statuts) relÃ¨vent des **Cores** (Border Guard, WorrySentinel, StrongFather). Le Kit Tracker **ne dÃ©cide pas** ; il fournit les capacitÃ©s (validate, verify, transport.receive/send, discovery.response.*, cog_list.*, port.check).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique du Kit Tracker Webway, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©.

**Hors scope :** le Kit Participant Webway (MiyuWebwayParticipant) ; les normes et standards MWS (format, protocole, ports) ; l'implÃ©mentation dÃ©taillÃ©e (systÃ¨mes passifs/actifs).

---

## 3. DÃ©finition canonique

> **MiyuWebwayTracker est une composition officielle d'outils MWS pour tenir le rÃ´le Tracker (recevoir, valider, rÃ©pondre Ã  la dÃ©couverte, maintenir listes de statuts), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuWebwayTracker **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Outils MWS existants.
- MiyuWebwayTracker **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; **dÃ©cisions (accepter, rejeter, filtrer) = Cores** (Border Guard, WorrySentinel) via BondingBrother.

**RÃ¨gle fondamentale :** Le Kit **ne dÃ©cide pas** d'accepter ou rejeter une dÃ©claration ; il **exÃ©cute** les validations, vÃ©rifications et filtrages ; les politiques viennent des Cores.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.webway.tracker` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `webway` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuWebwayTracker - Reference Outils](./MiyuWebwayTracker%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `mws.declaration.validate` | Valider le format d'une dÃ©claration reÃ§ue |
| `mws.declaration.verify` | VÃ©rifier la signature d'une dÃ©claration reÃ§ue |
| `mws.transport.receive` | Recevoir un message sur un endpoint (ex. port 21000) |
| `mws.transport.send` | Envoyer un message (rÃ©ponse, liste de statuts) |
| `mws.discovery.response.build` | Construire une rÃ©ponse de dÃ©couverte (liste COGs filtrÃ©e) |
| `mws.discovery.response.send` | Envoyer la rÃ©ponse au demandeur |
| `mws.cog_list.get` | Lire la liste locale de COGs avec statuts |
| `mws.cog_list.update` | Mettre Ã  jour une entrÃ©e dans la liste locale |
| `mws.cog_list.merge` | Fusionner une liste reÃ§ue avec la liste locale |
| `mws.cog_list.filter` | Filtrer la liste selon critÃ¨re (ex. exclure Rejected) ; critÃ¨re fourni par Cores |
| `mws.port.check` | VÃ©rifier si un port est dans la liste des ports exclus MWS |
| `mws.address.tracker_default` | RÃ©soudre l'adresse complÃ¨te d'un Tracker (port 21000) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuWebwayTracker en contient douze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cisions (accepter, rejeter, filtrer) = Border Guard, WorrySentinel** ; traduction en intentions = BondingBrother ; le Kit exÃ©cute uniquement.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **2 Ã  3** (rÃ©seau, dÃ©clarations, dÃ©couverte, protection maillage) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon WorrySentinel) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` ; WorrySentinel peut bloquer le rÃ´le Tracker |

---

## 8. Relation avec KindMother

Les Outils MWS du Kit Tracker **ne persistent pas** de donnÃ©es mÃ©tier dans KindMother au sens classique ; la liste locale de COGs avec statuts peut Ãªtre en mÃ©moire ou persistÃ©e selon l'implÃ©mentation. Les rÃ¨gles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuWebwayTracker - Tool Governance Compliance Contract](./contracts/governance/MiyuWebwayTracker%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuWebwayTracker sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Outils MWS MiyuWebwayTracker devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Webway System - Outils et OpÃ©rateurs | [Miyukini Webway System - Outils et Operateurs](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../cores/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur

