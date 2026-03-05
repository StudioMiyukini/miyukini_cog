# MiyuPosLoyalty â€” Documentation Fondatrice

## 1. Contexte

**MiyuPosLoyalty** est le **kit d'outils (Toolkit)** CRM et fidÃ©litÃ© PoS de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de gestion clients (CRUD, adresse, notes) et de programme fidÃ©litÃ© (points, solde, carte, octroi/rÃ©demption), alignÃ©s sur le document [Ã‰quivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (clients, adresses, notes, points fidÃ©litÃ©) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuPosLoyalty expose des capacitÃ©s d'exÃ©cution gouvernÃ©e sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs (ex. OpÃ©rateur FidÃ©litÃ©/CRM) passent par la gouvernance pour utiliser ces outils. L'octroi et la rÃ©demption de points sont soumis Ã  StrongFather (autorisation).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuPosLoyalty, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e ; toute dÃ©cision (octroi/rÃ©demption points) â€” ressort de StrongFather.

---

## 3. DÃ©finition canonique

> **MiyuPosLoyalty est une composition officielle d'outils CRM et fidÃ©litÃ© (clients, adresses, notes, points, cartes fidÃ©litÃ©, octroi/rÃ©demption), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuPosLoyalty **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuPosLoyalty **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; l'octroi et la rÃ©demption de points sont autorisÃ©s par StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (client, points) passe par WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyuposloyalty` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` / `crm` / `loyalty` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuPosLoyalty - Reference Outils](./MiyuPosLoyalty%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.crm.customer.get` | Retourne un client par identifiant |
| `tool.crm.customer.list` | Liste les clients (filtres, recherche) |
| `tool.crm.customer.create` | CrÃ©e un client Ã  partir de donnÃ©es fournies |
| `tool.crm.customer.update` | Met Ã  jour un client |
| `tool.crm.customer.address.get` | Retourne l'adresse (livraison) du client |
| `tool.crm.customer.note.add` | Ajoute une note Ã  un client |
| `tool.crm.customer.note.list` | Liste les notes d'un client |
| `tool.loyalty.points.grant` | Accorde des points (rÃ¨gles fournies ou gouvernÃ©es) |
| `tool.loyalty.points.redeem` | DÃ©duit des points (Ã©change) ; autorisation = StrongFather |
| `tool.loyalty.balance.get` | Retourne le solde points d'un client |
| `tool.loyalty.card.resolve` | RÃ©sout une carte fidÃ©litÃ© (code/QR) â†’ client + solde |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosLoyalty en contient onze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : points grant/redeem = autorisation StrongFather ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (dÃ©tail par outil dans Reference Outils) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur toutes les donnÃ©es : clients, adresses, notes, points fidÃ©litÃ©. Toute Ã©criture (crÃ©ation/mise Ã  jour client, octroi/rÃ©demption points) passe par **WriteIntent** sous autoritÃ© KindMother. MiyuPosLoyalty exÃ©cute des capacitÃ©s atomiques ; l'autorisation d'octroi/rÃ©demption reste Ã  StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuPosLoyalty - Tool Governance Compliance Contract](./contracts/governance/MiyuPosLoyalty%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuPosLoyalty sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuPosLoyalty devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


