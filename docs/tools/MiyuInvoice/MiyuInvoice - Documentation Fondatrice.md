# MiyuInvoice â€” Documentation Fondatrice

## 1. Contexte

**MiyuInvoice** est le **kit d'outils (Toolkit)** de facturation mÃ©tier indÃ©pendants (devis, factures ponctuelles, acomptes, relances, facturation Ã©lectronique B2B 2026) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de crÃ©ation et mise Ã  jour de devis, de conversion devis â†’ facture, de crÃ©ation et envoi de factures, de relances, de gÃ©nÃ©ration de lien de paiement et de soumission Ã  la facturation Ã©lectronique, alignÃ©s sur [Ã‰quivalents ComptabilitÃ© IndÃ©pendants](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (devis, factures, clients facturation) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuInvoice expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (envoi relance, conversion devis â†’ facture) relÃ¨vent de **StrongFather**. Les OpÃ©rateurs (ex. Facturation indÃ©pendants) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuInvoice, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (gÃ©nÃ©ration PDF, plateformes agrÃ©Ã©es facturation Ã©lectronique) ; la facturation SaaS rÃ©currente (voir MiyuBilling).

---

## 3. DÃ©finition canonique

> **MiyuInvoice est une composition officielle d'outils de facturation mÃ©tier indÃ©pendants (devis, factures, relances, facturation Ã©lectronique B2B), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuInvoice **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuInvoice **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; les dÃ©cisions (relance, conversion devis â†’ facture) = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (devis, facture) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.invoice.standalone` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `invoice` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuInvoice - Reference Outils](./MiyuInvoice%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.invoice.quote.create` | CrÃ©e un devis Ã  partir de donnÃ©es fournies |
| `tool.invoice.quote.update` | Met Ã  jour un devis existant |
| `tool.invoice.quote.to_invoice` | Convertit un devis en facture (exÃ©cution ; dÃ©cision = StrongFather) |
| `tool.invoice.create` | CrÃ©e une facture (mÃ©tier indÃ©pendant) Ã  partir de donnÃ©es fournies |
| `tool.invoice.send` | Envoie une facture par canal fourni (email, etc.) |
| `tool.invoice.electronic.submit` | Soumet Ã  la facturation Ã©lectronique (plateforme agrÃ©Ã©e 2026) |
| `tool.invoice.reminder.send` | Envoie une relance (exÃ©cution ; rÃ¨gles = StrongFather) |
| `tool.invoice.payment.link.generate` | GÃ©nÃ¨re un lien de paiement pour une facture |
| `tool.invoice.customer.resolve` | RÃ©sout un client (facturation) par identifiant |
| `tool.invoice.customer.list` | Liste les clients (filtres fournis) pour facturation |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuInvoice en contient dix.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : permissions (qui peut Ã©mettre/modifier factures) = Master Butler + StrongFather ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (dÃ©tail par outil dans Reference Outils) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : devis, factures, clients facturation. Toute Ã©criture (quote.create, quote.update, invoice.create) passe par **WriteIntent** sous autoritÃ© KindMother. MiyuInvoice exÃ©cute des capacitÃ©s atomiques ; les dÃ©cisions (relance, conversion devis â†’ facture) = StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuInvoice - Tool Governance Compliance Contract](./contracts/governance/MiyuInvoice%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuInvoice sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuInvoice devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents ComptabilitÃ© IndÃ©pendants | [Miyukini Conceptual References - Equivalents Comptabilite Independants](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


