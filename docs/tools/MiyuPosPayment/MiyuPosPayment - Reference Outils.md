# MiyuPosPayment — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuPosPayment. Référence technique des capacités atomiques paiements PoS ; partage et autorisation CB = StrongFather. Persistance = KindMother (WriteIntent).

**Référence du kit :** [MiyuPosPayment - Documentation Fondatrice](./MiyuPosPayment%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.pos.payment.cash.record` | Enregistrement paiement espèces | Enregistre un paiement espèces ; montant + session ; KindMother | 2 |
| `tool.pos.payment.check.record` | Enregistrement paiement chèque | Enregistre un paiement chèque | 2 |
| `tool.pos.payment.split` | Partage paiement | Répartit le paiement entre plusieurs moyens (données fournies) ; autorisation = StrongFather | 2 |
| `tool.payment.terminal.authorize` | Autorisation terminal CB | Demande une autorisation à un terminal CB (données fournies) ; intégrations externes sous gouvernance | 2–3 |
| `tool.payment.terminal.capture` | Capture paiement CB | Confirme (capture) un paiement CB précédemment autorisé | 2–3 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
