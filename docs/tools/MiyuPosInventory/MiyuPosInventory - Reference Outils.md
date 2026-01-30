# MiyuPosInventory — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuPosInventory. Référence technique des capacités atomiques d'inventaire sans décision métier (ajustement, transfert, réconciliation = StrongFather). Persistance = KindMother (WriteIntent).

**Référence du kit :** [MiyuPosInventory - Documentation Fondatrice](./MiyuPosInventory%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.inventory.stock.get` | Lecture stock | Retourne le stock (et composants si applicable) pour un article/site | 0–1 |
| `tool.inventory.stock.adjust` | Ajustement stock | Ajuste le stock (réception, casse, perte) ; décision = StrongFather | 2 |
| `tool.inventory.import.items` | Import articles | Importe des articles à partir d'un flux structuré (ex. CSV) ; persistance = KindMother | 2 |
| `tool.inventory.alert.low.evaluate` | Alertes stock bas | Évalue les articles sous seuil bas (données seuils fournies) | 0–1 |
| `tool.inventory.purchase_order.create` | Création bon de commande | Crée un bon de commande fournisseur à partir de données fournies | 1–2 |
| `tool.inventory.purchase_order.update` | Mise à jour bon de commande | Met à jour un bon de commande (réception partielle, etc.) | 1–2 |
| `tool.inventory.purchase_order.track` | Suivi bon de commande | Retourne le statut / suivi d'un bon de commande | 0–1 |
| `tool.inventory.transfer.create` | Création transfert | Crée un transfert entre sites à partir de données fournies | 1–2 |
| `tool.inventory.transfer.execute` | Exécution transfert | Exécute (confirme) un transfert ; autorisation = StrongFather | 2 |
| `tool.inventory.transfer.list` | Liste transferts | Liste les transferts (filtres fournis) | 0–1 |
| `tool.inventory.count.start` | Démarrage inventaire | Démarre une session d'inventaire physique | 1–2 |
| `tool.inventory.count.record` | Enregistrement comptage | Enregistre un comptage (article, quantité) pour une session | 1–2 |
| `tool.inventory.count.reconcile` | Réconciliation comptage | Clôture un comptage et propose/applique les écarts ; décision = StrongFather | 2 |
| `tool.inventory.production.record` | Enregistrement production | Enregistre une production (débit composants, crédit produit) ; données recette fournies | 1–2 |
| `tool.pos.label.print` | Impression étiquette | Imprime une étiquette code-barres (données fournies) | 1 |
| `tool.inventory.history.list` | Historique mouvements | Liste l'historique des mouvements (filtres fournis) | 0–1 |
| `tool.inventory.valuation.report` | Rapport valorisation | Retourne un rapport de valorisation (coût / marge potentielle) ; lecture seule | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
