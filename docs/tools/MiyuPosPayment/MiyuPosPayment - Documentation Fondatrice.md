# MiyuPosPayment â€” Documentation Fondatrice

## 1. Contexte

**MiyuPosPayment** est le **kit d'outils (Toolkit)** paiements PoS de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'enregistrement paiement espÃ¨ces, chÃ¨que, partage d'addition et les adaptateurs terminaux CB (autorisation, capture), alignÃ©s sur le document [Ã‰quivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (paiements, sessions) appartient Ã  **KindMother**. Les dÃ©cisions (partage d'addition, autorisation CB) relÃ¨vent de **StrongFather** et **WorrySentinel** (niveau de sÃ©curitÃ©). MiyuPosPayment expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les OpÃ©rateurs (ex. OpÃ©rateur Caisse, Paiement) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuPosPayment, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (terminaux SumUp, Zettle, etc.) ; toute dÃ©cision d'autorisation â€” ressort de StrongFather / WorrySentinel.

---

## 3. DÃ©finition canonique

> **MiyuPosPayment est une composition officielle d'outils paiements PoS (espÃ¨ces, chÃ¨que, partage d'addition, terminaux CB), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuPosPayment **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuPosPayment **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; l'autorisation (partage, CB) appartient Ã  StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (paiement enregistrÃ©) = WriteIntent vers KindMother. Niveau de sÃ©curitÃ© Ã©levÃ© pour paiements (WorrySentinel).

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyupospayment` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` / `payment` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuPosPayment - Reference Outils](./MiyuPosPayment%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.pos.payment.cash.record` | Enregistre un paiement espÃ¨ces |
| `tool.pos.payment.check.record` | Enregistre un paiement chÃ¨que |
| `tool.pos.payment.split` | RÃ©partit le paiement entre plusieurs moyens (donnÃ©es fournies) ; autorisation = StrongFather |
| `tool.payment.terminal.authorize` | Demande une autorisation Ã  un terminal CB (donnÃ©es fournies) |
| `tool.payment.terminal.capture` | Confirme (capture) un paiement CB prÃ©cÃ©demment autorisÃ© |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosPayment en contient cinq.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : WorrySentinel applique le niveau de sÃ©curitÃ© paiement ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **2 Ã  3** (paiements sensibles ; dÃ©tail dans Reference Outils) ; cohÃ©rent avec WorrySentinel. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es de paiement (espÃ¨ces, chÃ¨que, partage, rÃ©fÃ©rences CB). Toute Ã©criture (enregistrement paiement) passe par **WriteIntent** sous autoritÃ© KindMother. MiyuPosPayment exÃ©cute des capacitÃ©s atomiques ; l'autorisation (partage, autorisation CB) reste Ã  StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuPosPayment - Tool Governance Compliance Contract](./contracts/governance/MiyuPosPayment%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuPosPayment sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuPosPayment devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


