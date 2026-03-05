# MiyuWidgets â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.web.widgets`

---

## Obligations spÃ©cifiques MiyuWidgets

- **DonnÃ©es dans le flux :** les Tools opÃ¨rent uniquement sur des donnÃ©es fournies dans le flux ; MiyuWidgets ne lit pas la base directement.
- **Persistance** (templates, structures de layout) = KindMother ; toute Ã©criture de templates/layouts passe par WriteIntent KindMother (exÃ©cutÃ©e par d'autres flux).
- **Pas de dÃ©cision mÃ©tier** : le kit ne dÃ©cide pas du contenu ni de la logique mÃ©tier ; dÃ©cision = OpÃ©rateurs et Cores.
- **Niveau de sÃ©curitÃ© :** 0 Ã  2 selon politique d'exposition (page builder Ã©ditorial) ; cohÃ©rent WorrySentinel.
- MiyuWeb fournit les capacitÃ©s de base (html.render, layout.render, theme.resolve, etc.) ; MiyuWidgets complÃ¨te MiyuWeb pour l'Ã©dition visuelle de pages et de thÃ¨mes (widgets, layout.apply, template.resolve).

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

