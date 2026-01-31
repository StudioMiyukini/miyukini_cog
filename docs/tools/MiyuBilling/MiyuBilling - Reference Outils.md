# MiyuBilling — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuBilling** (`toolkit.billing.saas`). Chaque outil est une capacité atomique gouvernée ; décision = StrongFather ; persistance (souscriptions, factures, paiements) = WriteIntent KindMother.

**Référence :** [MiyuBilling - Documentation Fondatrice](./MiyuBilling%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|------------------|------|
| `tool.billing.subscription.create` | Créer une souscription | 2 | WriteIntent KindMother ; décision StrongFather |
| `tool.billing.subscription.update` | Mettre à jour une souscription | 2 | Renouvellement, changement offre ; WriteIntent KindMother |
| `tool.billing.subscription.cancel` | Annuler / résilier une souscription | 2 | Décision StrongFather ; WriteIntent KindMother |
| `tool.billing.subscription.status` | Retourner le statut d'une souscription | 1 | Lecture ; données fournies dans le flux ou KindMother |
| `tool.billing.invoice.generate` | Générer une facture | 2 | Règles fournies ; WriteIntent KindMother |
| `tool.billing.invoice.list` | Lister les factures | 1–2 | Filtres fournis ; lecture gouvernée |
| `tool.billing.payment.record` | Enregistrer un paiement reçu | 3 | Exécution ; décision StrongFather ; WriteIntent KindMother |
| `tool.billing.tenant.resolve` | Résoudre le contexte tenant | 1 | Isolation multi-tenant ; pas d'écriture |

---

**Invariant :** Toute écriture (souscription, facture, paiement) = **WriteIntent** vers KindMother. Toute décision (autoriser souscription, enregistrer paiement, résilier) = StrongFather.
