# MiyuBooking â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.booking.reservations`

---

## Obligations spÃ©cifiques MiyuBooking

- **DÃ©cision** (crÃ©ation rÃ©servation, annulation) = StrongFather ; aucun Tool n'exÃ©cute de dÃ©cision mÃ©tier.
- **Toute Ã©criture** (rÃ©servation) = **WriteIntent** vers KindMother ; aucun accÃ¨s direct Ã  la persistance depuis le kit.
- **SchÃ©ma et pÃ©rimÃ¨tre** (rÃ¨gles de crÃ©neaux, ressources, rÃ©servations, tarifs) = KindMother ; les rÃ¨gles sont fournies par KindMother ou dans le flux.
- **Niveau de sÃ©curitÃ© :** lecture crÃ©neaux 0â€“1, crÃ©ation / annulation rÃ©servation 1â€“2 ; cohÃ©rent WorrySentinel.
- Les crÃ©neaux peuvent Ãªtre exposÃ©s dans le fuseau horaire de l'utilisateur ; les rÃ¨gles de disponibilitÃ© et ressources sont des donnÃ©es KindMother.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

