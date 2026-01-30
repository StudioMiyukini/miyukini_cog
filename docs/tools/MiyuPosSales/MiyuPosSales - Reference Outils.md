# MiyuPosSales — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuPosSales. Il constitue la référence technique des capacités atomiques de caisse et ventes PoS sans décision métier (remboursement, remise = StrongFather). Les Tools sont gouvernés par les Cores ; la persistance relève de KindMother (WriteIntent).

**Référence du kit :** [MiyuPosSales - Documentation Fondatrice](./MiyuPosSales%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

**Ce document fournit :** la liste exhaustive des Tools du kit MiyuPosSales avec ToolId, nom lisible, action, niveau de sécurité typique.

**Hors scope :** l'implémentation ; la décision ALLOW/DENY (StrongFather).

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.pos.sale.create` | Création vente | Crée une vente (ticket) à partir du contexte fourni | 1–2 |
| `tool.pos.sale.add_item` | Ajout ligne | Ajoute une ligne (article, variante, modificateurs, qté) à une vente | 1–2 |
| `tool.pos.sale.remove_item` | Retrait ligne | Retire une ligne d'une vente | 1–2 |
| `tool.pos.ticket.open` | Ouverture ticket | Ouvre un ticket (ordre) pour paiement différé | 1–2 |
| `tool.pos.ticket.save` | Sauvegarde ticket | Sauvegarde un ticket sans le clôturer | 1–2 |
| `tool.pos.ticket.close` | Clôture ticket | Clôture un ticket (après paiement ou annulation) | 1–2 |
| `tool.pos.ticket.list` | Liste tickets | Liste les tickets ouverts (filtres fournis) | 0–1 |
| `tool.pos.discount.apply` | Application remise | Applique une remise (article ou reçu) ; règles = StrongFather | 1–2 |
| `tool.pos.refund.record` | Enregistrement remboursement | Enregistre un remboursement (item ou reçu) ; autorisation = StrongFather | 2 |
| `tool.pos.receipt.render` | Rendu reçu | Produit le contenu du reçu à partir des données de vente | 0–1 |
| `tool.pos.receipt.print` | Impression reçu | Envoie le reçu à l'imprimante (données fournies) | 1 |
| `tool.pos.receipt.send` | Envoi reçu email | Envoie le reçu par email (données fournies) | 1–2 |
| `tool.pos.receipt.list` | Liste reçus | Liste les reçus (filtres fournis) | 0–1 |
| `tool.pos.item.variant.resolve` | Résolution variante | Résout une variante (taille, couleur, etc.) pour un article | 0–1 |
| `tool.pos.item.modifier.apply` | Application modificateurs | Applique des modificateurs (add-ons) à une ligne | 0–1 |
| `tool.pos.cash.register.open` | Ouverture session caisse | Ouvre une session caisse (ouverture de tiroir) | 2 |
| `tool.pos.cash.register.close` | Clôture session caisse | Clôture une session caisse (comptage, écart) | 2 |
| `tool.pos.cash.movement.record` | Mouvement espèces | Enregistre un mouvement espèces (entrée/sortie) | 2 |
| `tool.pos.barcode.parse` | Parse code-barres | Parse un code-barres (optionnel : poids) ; retourne item + quantité | 0–1 |
| `tool.pos.context.store.resolve` | Résolution magasin | Résout le magasin/point de vente courant pour le contexte | 0–1 |
| `tool.pos.display.push` | Affichage client | Envoie les données à afficher sur l'écran client | 0–1 |
| `tool.pos.order.service_type.set` | Type de service | Définit le type de service (sur place / à emporter / livraison) | 1 |

**Format ToolId :** `tool.pos.<sous-domaine>.<action>` — conforme au Master Butler - Tool Governance Contract.

---

## 4. Références croisées

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuPosSales | [MiyuPosSales - Documentation Fondatrice](./MiyuPosSales%20-%20Documentation%20Fondatrice.md) |
| Équivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
