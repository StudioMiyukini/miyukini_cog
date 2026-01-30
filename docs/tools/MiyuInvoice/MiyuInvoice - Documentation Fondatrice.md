# MiyuInvoice — Documentation Fondatrice

## 1. Contexte

**MiyuInvoice** est le **kit d'outils (Toolkit)** de facturation métier indépendants (devis, factures ponctuelles, acomptes, relances, facturation électronique B2B 2026) de l'écosystème Miyukini. Il intègre les outils de création et mise à jour de devis, de conversion devis → facture, de création et envoi de factures, de relances, de génération de lien de paiement et de soumission à la facturation électronique, alignés sur [Équivalents Comptabilité Indépendants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md).

L'autorité sur les données (devis, factures, clients facturation) appartient à **KindMother** (Core de données, Strate 4). MiyuInvoice expose des capacités d'exécution gouvernée ; les décisions (envoi relance, conversion devis → facture) relèvent de **StrongFather**. Les Opérateurs (ex. Facturation indépendants) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuInvoice, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implémentation détaillée (génération PDF, plateformes agréées facturation électronique) ; la facturation SaaS récurrente (voir MiyuBilling).

---

## 3. Définition canonique

> **MiyuInvoice est une composition officielle d'outils de facturation métier indépendants (devis, factures, relances, facturation électronique B2B), déclarée et gouvernée par l'environnement.**

- MiyuInvoice **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuInvoice **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; les décisions (relance, conversion devis → facture) = StrongFather.

**Règle fondamentale :** Toute écriture (devis, facture) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.invoice.standalone` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `invoice` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuInvoice - Reference Outils](./MiyuInvoice%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.invoice.quote.create` | Crée un devis à partir de données fournies |
| `tool.invoice.quote.update` | Met à jour un devis existant |
| `tool.invoice.quote.to_invoice` | Convertit un devis en facture (exécution ; décision = StrongFather) |
| `tool.invoice.create` | Crée une facture (métier indépendant) à partir de données fournies |
| `tool.invoice.send` | Envoie une facture par canal fourni (email, etc.) |
| `tool.invoice.electronic.submit` | Soumet à la facturation électronique (plateforme agréée 2026) |
| `tool.invoice.reminder.send` | Envoie une relance (exécution ; règles = StrongFather) |
| `tool.invoice.payment.link.generate` | Génère un lien de paiement pour une facture |
| `tool.invoice.customer.resolve` | Résout un client (facturation) par identifiant |
| `tool.invoice.customer.list` | Liste les clients (filtres fournis) pour facturation |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuInvoice en contient dix.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : permissions (qui peut émettre/modifier factures) = Master Butler + StrongFather ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (détail par outil dans Reference Outils) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : devis, factures, clients facturation. Toute écriture (quote.create, quote.update, invoice.create) passe par **WriteIntent** sous autorité KindMother. MiyuInvoice exécute des capacités atomiques ; les décisions (relance, conversion devis → facture) = StrongFather.

Les obligations de conformité détaillées sont dans [MiyuInvoice - Tool Governance Compliance Contract](./contracts/governance/MiyuInvoice%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Comptabilité Indépendants | [Miyukini Conceptual References - Equivalents Comptabilite Independants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
