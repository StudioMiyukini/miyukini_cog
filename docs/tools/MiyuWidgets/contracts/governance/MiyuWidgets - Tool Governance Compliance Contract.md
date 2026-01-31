# MiyuWidgets — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.web.widgets`

---

## Obligations spécifiques MiyuWidgets

- **Données dans le flux :** les Tools opèrent uniquement sur des données fournies dans le flux ; MiyuWidgets ne lit pas la base directement.
- **Persistance** (templates, structures de layout) = KindMother ; toute écriture de templates/layouts passe par WriteIntent KindMother (exécutée par d'autres flux).
- **Pas de décision métier** : le kit ne décide pas du contenu ni de la logique métier ; décision = Opérateurs et Cores.
- **Niveau de sécurité :** 0 à 2 selon politique d'exposition (page builder éditorial) ; cohérent WorrySentinel.
- MiyuWeb fournit les capacités de base (html.render, layout.render, theme.resolve, etc.) ; MiyuWidgets complète MiyuWeb pour l'édition visuelle de pages et de thèmes (widgets, layout.apply, template.resolve).

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
