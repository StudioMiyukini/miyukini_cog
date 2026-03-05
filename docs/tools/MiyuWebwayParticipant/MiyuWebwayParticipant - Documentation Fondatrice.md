# MiyuWebwayParticipant â€” Documentation Fondatrice

## 1. Contexte

**MiyuWebwayParticipant** est le **Kit d'Outils Participant Webway (MWS)** de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils MWS nÃ©cessaires pour **participer** au maillage (annoncer, dÃ©couvrir, maintenir la liste de statuts), alignÃ©s sur [Miyukini Webway System - Outils et OpÃ©rateurs](..//..//miyukini-webway-system//reference//_index.md).

Les dÃ©cisions (quand annoncer, Ã  quels Trackers envoyer, politique de dÃ©couverte) relÃ¨vent des **Cores** (StrongFather, Border Guard, WorrySentinel) et sont traduites en intentions par **BondingBrother**. Le Kit Participant **ne dÃ©cide pas** ; il fournit les capacitÃ©s (build, sign, validate, verify, transport.send, discovery.request.*, cog_list.*, port.check, address.tracker_default).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique du Kit Participant Webway, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©.

**Hors scope :** le Kit Tracker Webway (MiyuWebwayTracker) ; les normes et standards MWS (format, protocole, ports) ; l'implÃ©mentation dÃ©taillÃ©e (binding transport).

---

## 3. DÃ©finition canonique

> **MiyuWebwayParticipant est une composition officielle d'outils MWS pour participer au maillage (annoncer, dÃ©couvrir, maintenir liste de statuts), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuWebwayParticipant **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Outils MWS existants.
- MiyuWebwayParticipant **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; **dÃ©cisions = Cores** (StrongFather, Border Guard, WorrySentinel) via BondingBrother.

**RÃ¨gle fondamentale :** Le Kit **ne dÃ©cide pas** quand annoncer ni Ã  quels Trackers envoyer ; il **exÃ©cute** les intentions fournies par les Cores.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.webway.participant` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `webway` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuWebwayParticipant - Reference Outils](./MiyuWebwayParticipant%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `mws.declaration.build` | Construire un message de dÃ©claration conforme MWS |
| `mws.declaration.sign` | Signer une dÃ©claration |
| `mws.declaration.validate` | Valider le format d'une dÃ©claration |
| `mws.declaration.verify` | VÃ©rifier la signature d'une dÃ©claration |
| `mws.transport.send` | Envoyer un message vers une adresse (Tracker ou COG) |
| `mws.discovery.request.build` | Construire une requÃªte de dÃ©couverte |
| `mws.discovery.request.send` | Envoyer une requÃªte de dÃ©couverte vers un/des Tracker(s) |
| `mws.cog_list.get` | Lire la liste locale de COGs avec statuts |
| `mws.cog_list.update` | Mettre Ã  jour une entrÃ©e dans la liste locale |
| `mws.cog_list.merge` | Fusionner une liste reÃ§ue avec la liste locale |
| `mws.port.check` | VÃ©rifier si un port est dans la liste des ports exclus MWS |
| `mws.address.tracker_default` | RÃ©soudre l'adresse complÃ¨te d'un Tracker (port 21000) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuWebwayParticipant en contient douze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cisions (annoncer, dÃ©couvrir, politique) = StrongFather, Border Guard, WorrySentinel** ; traduction en intentions = BondingBrother ; le Kit exÃ©cute uniquement.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **2 Ã  3** (rÃ©seau, dÃ©clarations, dÃ©couverte) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon WorrySentinel) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` ; WorrySentinel peut bloquer ou dÃ©grader la participation MWS |

---

## 8. Relation avec KindMother

Les Outils MWS du Kit Participant **ne persistent pas** de donnÃ©es mÃ©tier dans KindMother au sens classique ; la liste locale de COGs avec statuts peut Ãªtre en mÃ©moire ou persistÃ©e selon l'implÃ©mentation. Les rÃ¨gles de fusion et de filtrage viennent des Cores (Border Guard, WorrySentinel).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuWebwayParticipant - Tool Governance Compliance Contract](./contracts/governance/MiyuWebwayParticipant%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuWebwayParticipant sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Outils MWS MiyuWebwayParticipant devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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

