# MiyuCMS â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.content.cms`

---

## Obligations spÃ©cifiques MiyuCMS

- **DÃ©cision** (publication, modÃ©ration, restauration rÃ©vision) = StrongFather ; aucun Tool n'exÃ©cute de dÃ©cision mÃ©tier.
- **Toute Ã©criture** (contenu, mÃ©dia, rÃ©vision, commentaire) = **WriteIntent** vers KindMother ; aucun accÃ¨s direct Ã  la persistance depuis le kit.
- **SchÃ©ma et pÃ©rimÃ¨tre** (contenus, mÃ©dias, rÃ©visions, commentaires) = KindMother ; le kit ne modifie pas le schÃ©ma.
- **Niveau de sÃ©curitÃ© :** 0 Ã  2 selon politique d'exposition (contenu public Ã  Ã©ditorial sensible) ; cohÃ©rent WorrySentinel.
- L'affichage des contenus est du ressort de MiyuWeb (donnÃ©es fournies dans le flux) ; MiyuCMS gÃ¨re uniquement la gestion Ã©ditoriale et la persistance gouvernÃ©e.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

