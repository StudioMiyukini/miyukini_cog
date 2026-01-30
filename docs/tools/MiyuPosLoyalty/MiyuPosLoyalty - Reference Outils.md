# MiyuPosLoyalty — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuPosLoyalty. Référence technique des capacités atomiques CRM et fidélité ; octroi/rédemption points = autorisation StrongFather. Persistance = KindMother (WriteIntent).

**Référence du kit :** [MiyuPosLoyalty - Documentation Fondatrice](./MiyuPosLoyalty%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.crm.customer.get` | Lecture client | Retourne un client par identifiant | 0–1 |
| `tool.crm.customer.list` | Liste clients | Liste les clients (filtres, recherche) | 0–1 |
| `tool.crm.customer.create` | Création client | Crée un client à partir de données fournies | 1–2 |
| `tool.crm.customer.update` | Mise à jour client | Met à jour un client | 1–2 |
| `tool.crm.customer.address.get` | Adresse livraison | Retourne l'adresse (livraison) du client | 0–1 |
| `tool.crm.customer.note.add` | Ajout note client | Ajoute une note à un client | 1–2 |
| `tool.crm.customer.note.list` | Liste notes client | Liste les notes d'un client | 0–1 |
| `tool.loyalty.points.grant` | Octroi points | Accorde des points (règles fournies ou gouvernées) | 1–2 |
| `tool.loyalty.points.redeem` | Rédemption points | Déduit des points (échange) ; autorisation = StrongFather | 1–2 |
| `tool.loyalty.balance.get` | Solde points | Retourne le solde points d'un client | 0–1 |
| `tool.loyalty.card.resolve` | Résolution carte fidélité | Résout une carte fidélité (code/QR) → client + solde | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
