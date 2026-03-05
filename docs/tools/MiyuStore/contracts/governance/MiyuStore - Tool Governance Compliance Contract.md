# MiyuStore â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.commerce.store`

---

## Obligations spÃ©cifiques MiyuStore

- **DÃ©cision** (checkout, paiement, crÃ©ation commande, autorisation promo) = StrongFather ; aucun Tool n'exÃ©cute de dÃ©cision mÃ©tier.
- **Toute Ã©criture** (produit, panier, commande) = **WriteIntent** vers KindMother ; aucun accÃ¨s direct Ã  la persistance depuis le kit.
- **SchÃ©ma et pÃ©rimÃ¨tre** (produits, paniers, commandes, rÃ¨gles livraison et paiement) = KindMother ; le kit ne modifie pas le schÃ©ma.
- **Niveau de sÃ©curitÃ© :** WorrySentinel applique le niveau paiement ; catalogue 0â€“1, panier/checkout 1â€“2, paiement 3.
- MiyuShipping (toolkit.commerce.shipping) complÃ¨te MiyuStore pour Ã©tiquettes, comparaison transporteurs, suivi colis et expÃ©ditions ; MiyuStore inclut les Tools de base shipping.rate et shipping.zones.resolve pour le checkout.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

