# MiyuShipping â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.commerce.shipping`

---

## Obligations spÃ©cifiques MiyuShipping

- **DÃ©cision** (crÃ©ation Ã©tiquette, expÃ©dition) = StrongFather ; aucun Tool n'exÃ©cute de dÃ©cision mÃ©tier.
- **Toute Ã©criture** (expÃ©dition, Ã©tat commande) = **WriteIntent** vers KindMother ; aucun accÃ¨s direct Ã  la persistance depuis le kit.
- **SchÃ©ma et pÃ©rimÃ¨tre** (zones, rÃ¨gles de livraison, Ã©tat des commandes et expÃ©ditions) = KindMother ; les rÃ¨gles sont fournies par KindMother ou dans le flux.
- **Niveau de sÃ©curitÃ© :** tarifs/suivi 0â€“1, Ã©tiquettes/expÃ©ditions 2 ; cohÃ©rent WorrySentinel.
- MiyuStore inclut les Tools de base (rate, zones.resolve) pour le checkout ; MiyuShipping agrÃ¨ge l'ensemble des Tools livraison pour le Service complet de livraison et d'expÃ©dition.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

