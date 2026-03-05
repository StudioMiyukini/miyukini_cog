# MiyuMedia â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.content.media`

---

## Obligations spÃ©cifiques MiyuMedia

- **Toute Ã©criture** (upload, mÃ©tadonnÃ©es) = **WriteIntent** vers KindMother ; aucun accÃ¨s direct Ã  la persistance depuis le kit.
- **SchÃ©ma et pÃ©rimÃ¨tre** (mÃ©dias) = KindMother ; le kit ne modifie pas le schÃ©ma.
- **Niveau de sÃ©curitÃ© :** 0 Ã  2 selon politique d'exposition (mÃ©dias publics Ã  sensibles) ; cohÃ©rent WorrySentinel.
- Toute dÃ©cision de politique de stockage ou de quota = StrongFather / Cores ; MiyuMedia exÃ©cute uniquement les capacitÃ©s upload, serve, transform.
- MiyuCMS peut agrÃ©ger MiyuMedia (tool.media.*) pour le Service CMS complet ; MiyuMedia peut Ãªtre utilisÃ© seul lorsque seule la gestion des mÃ©dias est requise.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

