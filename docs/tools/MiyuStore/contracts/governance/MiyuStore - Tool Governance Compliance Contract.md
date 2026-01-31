# MiyuStore — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.commerce.store`

---

## Obligations spécifiques MiyuStore

- **Décision** (checkout, paiement, création commande, autorisation promo) = StrongFather ; aucun Tool n'exécute de décision métier.
- **Toute écriture** (produit, panier, commande) = **WriteIntent** vers KindMother ; aucun accès direct à la persistance depuis le kit.
- **Schéma et périmètre** (produits, paniers, commandes, règles livraison et paiement) = KindMother ; le kit ne modifie pas le schéma.
- **Niveau de sécurité :** WorrySentinel applique le niveau paiement ; catalogue 0–1, panier/checkout 1–2, paiement 3.
- MiyuShipping (toolkit.commerce.shipping) complète MiyuStore pour étiquettes, comparaison transporteurs, suivi colis et expéditions ; MiyuStore inclut les Tools de base shipping.rate et shipping.zones.resolve pour le checkout.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
