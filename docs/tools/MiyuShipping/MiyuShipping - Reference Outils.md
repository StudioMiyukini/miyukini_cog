# MiyuShipping — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuShipping** (`toolkit.commerce.shipping`). Chaque outil est une capacité atomique gouvernée ; décision (création étiquette, expédition) = StrongFather ; persistance (expéditions, état commande) = WriteIntent KindMother.

**Référence :** [MiyuShipping - Documentation Fondatrice](./MiyuShipping%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|------------------|------|
| `tool.commerce.shipping.rate` | Calculer le tarif de livraison | 0–1 | Panier/zone fourni ; règles KindMother ou flux |
| `tool.commerce.shipping.zones.resolve` | Résoudre les zones de livraison applicables | 0–1 | Lecture gouvernée |
| `tool.commerce.shipping.label.create` | Créer une étiquette d'expédition | 2 | Commande/colis fourni ; décision StrongFather ; WriteIntent si état géré |
| `tool.commerce.shipping.label.print` | Produire les données d'impression d'une étiquette | 1–2 | Exécution seule |
| `tool.commerce.shipping.rates.compare` | Comparer les tarifs de plusieurs transporteurs | 0–1 | Colis donné ; lecture |
| `tool.commerce.shipping.tracking.get` | Retourner le statut de suivi d'un envoi | 0–1 | Identifiant fourni |
| `tool.commerce.shipping.shipment.create` | Créer une expédition (colis) | 2 | WriteIntent si état commande géré ; décision StrongFather |
| `tool.commerce.shipping.shipment.list` | Lister les expéditions d'une commande | 0–1 | Lecture gouvernée |

---

**Invariant :** Toute écriture (expédition, état commande) = **WriteIntent** vers KindMother. Décision (création étiquette, expédition) = StrongFather.
