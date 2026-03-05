# MiyuBilling â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.billing.saas`

---

## Obligations spÃ©cifiques MiyuBilling

- **DÃ©cision** (crÃ©ation souscription, enregistrement paiement, rÃ©siliation) = StrongFather ; aucun Tool n'exÃ©cute de dÃ©cision mÃ©tier.
- **Toute Ã©criture** (souscription, facture, paiement) = **WriteIntent** vers KindMother ; aucun accÃ¨s direct Ã  la persistance depuis le kit.
- **Multi-tenant :** `tool.billing.tenant.resolve` fournit la rÃ©solution du contexte tenant (identifiant, pÃ©rimÃ¨tre) ; l'isolation des donnÃ©es par tenant relÃ¨ve de KindMother et Border Guard.
- **Niveau de sÃ©curitÃ© :** WorrySentinel applique le niveau facturation ; liste factures 1â€“2, crÃ©ation souscription / enregistrement paiement 2â€“3.
- **SchÃ©ma et pÃ©rimÃ¨tre** (offres, souscriptions, factures, paiements) = KindMother ; le kit ne modifie pas le schÃ©ma.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

