# MiyuInvoice — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuInvoice. Référence technique des capacités atomiques de facturation métier indépendants. Persistance = KindMother (WriteIntent). Décisions (relance, conversion devis → facture) = StrongFather.

**Référence du kit :** [MiyuInvoice - Documentation Fondatrice](./MiyuInvoice%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.invoice.quote.create` | Créer devis | Crée un devis à partir de données fournies ; persistance = KindMother | 1–2 |
| `tool.invoice.quote.update` | Mettre à jour devis | Met à jour un devis existant | 1–2 |
| `tool.invoice.quote.to_invoice` | Devis → facture | Convertit un devis en facture ; décision = StrongFather | 2 |
| `tool.invoice.create` | Créer facture | Crée une facture (métier indépendant) à partir de données fournies | 1–2 |
| `tool.invoice.send` | Envoyer facture | Envoie une facture par canal fourni (email, etc.) | 1–2 |
| `tool.invoice.electronic.submit` | Facturation électronique | Soumet à la facturation électronique (plateforme agréée 2026) | 2 |
| `tool.invoice.reminder.send` | Envoyer relance | Envoie une relance ; règles = StrongFather | 1–2 |
| `tool.invoice.payment.link.generate` | Lien paiement | Génère un lien de paiement pour une facture | 1–2 |
| `tool.invoice.customer.resolve` | Résoudre client | Résout un client (facturation) par identifiant | 0–1 |
| `tool.invoice.customer.list` | Lister clients | Liste les clients (filtres fournis) pour facturation | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
