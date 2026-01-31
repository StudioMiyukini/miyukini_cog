# MiyuMedia — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.content.media`

---

## Obligations spécifiques MiyuMedia

- **Toute écriture** (upload, métadonnées) = **WriteIntent** vers KindMother ; aucun accès direct à la persistance depuis le kit.
- **Schéma et périmètre** (médias) = KindMother ; le kit ne modifie pas le schéma.
- **Niveau de sécurité :** 0 à 2 selon politique d'exposition (médias publics à sensibles) ; cohérent WorrySentinel.
- Toute décision de politique de stockage ou de quota = StrongFather / Cores ; MiyuMedia exécute uniquement les capacités upload, serve, transform.
- MiyuCMS peut agréger MiyuMedia (tool.media.*) pour le Service CMS complet ; MiyuMedia peut être utilisé seul lorsque seule la gestion des médias est requise.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
