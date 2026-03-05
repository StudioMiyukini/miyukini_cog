# MiyuPosSales â€” RÃ©fÃ©rence des outils

## 1. Contexte

Ce document dÃ©crit **chaque outil (Tool)** composant le kit MiyuPosSales. Il constitue la rÃ©fÃ©rence technique des capacitÃ©s atomiques de caisse et ventes PoS sans dÃ©cision mÃ©tier (remboursement, remise = StrongFather). Les Tools sont gouvernÃ©s par les Cores ; la persistance relÃ¨ve de KindMother (WriteIntent).

**RÃ©fÃ©rence du kit :** [MiyuPosSales - Documentation Fondatrice](./MiyuPosSales%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

**Ce document fournit :** la liste exhaustive des Tools du kit MiyuPosSales avec ToolId, nom lisible, action, niveau de sÃ©curitÃ© typique.

**Hors scope :** l'implÃ©mentation ; la dÃ©cision ALLOW/DENY (StrongFather).

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sÃ©curitÃ© |
|--------|-------------|--------|------------------|
| `tool.pos.sale.create` | CrÃ©ation vente | CrÃ©e une vente (ticket) Ã  partir du contexte fourni | 1â€“2 |
| `tool.pos.sale.add_item` | Ajout ligne | Ajoute une ligne (article, variante, modificateurs, qtÃ©) Ã  une vente | 1â€“2 |
| `tool.pos.sale.remove_item` | Retrait ligne | Retire une ligne d'une vente | 1â€“2 |
| `tool.pos.ticket.open` | Ouverture ticket | Ouvre un ticket (ordre) pour paiement diffÃ©rÃ© | 1â€“2 |
| `tool.pos.ticket.save` | Sauvegarde ticket | Sauvegarde un ticket sans le clÃ´turer | 1â€“2 |
| `tool.pos.ticket.close` | ClÃ´ture ticket | ClÃ´ture un ticket (aprÃ¨s paiement ou annulation) | 1â€“2 |
| `tool.pos.ticket.list` | Liste tickets | Liste les tickets ouverts (filtres fournis) | 0â€“1 |
| `tool.pos.discount.apply` | Application remise | Applique une remise (article ou reÃ§u) ; rÃ¨gles = StrongFather | 1â€“2 |
| `tool.pos.refund.record` | Enregistrement remboursement | Enregistre un remboursement (item ou reÃ§u) ; autorisation = StrongFather | 2 |
| `tool.pos.receipt.render` | Rendu reÃ§u | Produit le contenu du reÃ§u Ã  partir des donnÃ©es de vente | 0â€“1 |
| `tool.pos.receipt.print` | Impression reÃ§u | Envoie le reÃ§u Ã  l'imprimante (donnÃ©es fournies) | 1 |
| `tool.pos.receipt.send` | Envoi reÃ§u email | Envoie le reÃ§u par email (donnÃ©es fournies) | 1â€“2 |
| `tool.pos.receipt.list` | Liste reÃ§us | Liste les reÃ§us (filtres fournis) | 0â€“1 |
| `tool.pos.item.variant.resolve` | RÃ©solution variante | RÃ©sout une variante (taille, couleur, etc.) pour un article | 0â€“1 |
| `tool.pos.item.modifier.apply` | Application modificateurs | Applique des modificateurs (add-ons) Ã  une ligne | 0â€“1 |
| `tool.pos.cash.register.open` | Ouverture session caisse | Ouvre une session caisse (ouverture de tiroir) | 2 |
| `tool.pos.cash.register.close` | ClÃ´ture session caisse | ClÃ´ture une session caisse (comptage, Ã©cart) | 2 |
| `tool.pos.cash.movement.record` | Mouvement espÃ¨ces | Enregistre un mouvement espÃ¨ces (entrÃ©e/sortie) | 2 |
| `tool.pos.barcode.parse` | Parse code-barres | Parse un code-barres (optionnel : poids) ; retourne item + quantitÃ© | 0â€“1 |
| `tool.pos.context.store.resolve` | RÃ©solution magasin | RÃ©sout le magasin/point de vente courant pour le contexte | 0â€“1 |
| `tool.pos.display.push` | Affichage client | Envoie les donnÃ©es Ã  afficher sur l'Ã©cran client | 0â€“1 |
| `tool.pos.order.service_type.set` | Type de service | DÃ©finit le type de service (sur place / Ã  emporter / livraison) | 1 |

**Format ToolId :** `tool.pos.<sous-domaine>.<action>` â€” conforme au Master Butler - Tool Governance Contract.

---

## 4. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuPosSales | [MiyuPosSales - Documentation Fondatrice](./MiyuPosSales%20-%20Documentation%20Fondatrice.md) |
| Ã‰quivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence

