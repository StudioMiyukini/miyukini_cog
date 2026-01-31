# MiyuStore — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuStore** (`toolkit.commerce.store`). Chaque outil est une capacité atomique gouvernée ; décision (checkout, paiement, création commande) = StrongFather ; persistance (produits, paniers, commandes) = WriteIntent KindMother.

**Référence :** [MiyuStore - Documentation Fondatrice](./MiyuStore%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|------------------|------|
| `tool.commerce.product.list` | Lister les produits | 0–1 | Filtres fournis |
| `tool.commerce.product.resolve` | Résoudre un produit par identifiant | 0–1 | Lecture |
| `tool.commerce.product.variations` | Lister les variations d'un produit | 0–1 | Lecture |
| `tool.commerce.product.create` | Créer un produit | 2 | Décision StrongFather ; WriteIntent KindMother |
| `tool.commerce.product.update` | Mettre à jour un produit | 2 | WriteIntent KindMother |
| `tool.commerce.cart.add` | Ajouter une ligne au panier | 1 | Données fournies ; WriteIntent KindMother |
| `tool.commerce.cart.update` | Mettre à jour une ligne du panier | 1 | WriteIntent KindMother |
| `tool.commerce.cart.remove` | Supprimer une ligne du panier | 1 | WriteIntent KindMother |
| `tool.commerce.cart.compute` | Calculer totaux, taxes, livraison du panier | 0–1 | Règles fournies dans le flux |
| `tool.commerce.checkout.validate` | Valider les données de checkout | 1 | Structure, champs ; pas d'écriture |
| `tool.commerce.checkout.submit` | Soumettre le checkout et créer la commande | 2–3 | WriteIntent KindMother ; décision StrongFather |
| `tool.commerce.payment.capture` | Capturer un paiement | 3 | Exécution ; autorisation StrongFather |
| `tool.commerce.payment.refund` | Rembourser un paiement | 3 | Décision StrongFather ; WriteIntent si état géré |
| `tool.commerce.payment.status` | Retourner le statut d'un paiement | 1–2 | Lecture gouvernée |
| `tool.commerce.shipping.rate` | Calculer le tarif de livraison | 0–1 | Panier/zone fourni |
| `tool.commerce.shipping.zones.resolve` | Résoudre les zones de livraison | 0–1 | Lecture gouvernée |
| `tool.commerce.order.create` | Créer une commande | 2 | Souvent appelé par checkout.submit ; WriteIntent KindMother |
| `tool.commerce.order.update` | Mettre à jour une commande | 2 | Statut, champs ; WriteIntent KindMother |
| `tool.commerce.order.status` | Retourner le statut d'une commande | 0–1 | Lecture |
| `tool.commerce.order.list` | Lister les commandes | 0–2 | Filtres fournis |

---

**Invariant :** Toute écriture (produit, panier, commande) = **WriteIntent** vers KindMother. Décision (checkout, paiement, création commande) = StrongFather. WorrySentinel applique le niveau sécurité paiement.
