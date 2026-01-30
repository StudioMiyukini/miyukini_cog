# MiyuPosKitchen — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuPosKitchen. Référence technique des capacités atomiques restaurant / bar (cuisine, type de service, tickets prédéfinis).

**Référence du kit :** [MiyuPosKitchen - Documentation Fondatrice](./MiyuPosKitchen%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.pos.kitchen.print` | Impression cuisine | Envoie la commande à l'imprimante cuisine (données fournies) | 1 |
| `tool.pos.kitchen.order.push` | Push ordre cuisine | Envoie un ordre à l'affichage cuisine | 1 |
| `tool.pos.kitchen.order.update_status` | Mise à jour statut ordre | Met à jour le statut d'un ordre cuisine (en cours, prêt) | 1 |
| `tool.pos.order.service_type.set` | Type de service | Définit le type de service (sur place / à emporter / livraison) | 1 |
| `tool.pos.ticket.preset.assign` | Libellé prédéfini ticket | Assigne un libellé prédéfini (ex. Table 1) à un ticket | 1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
