# MiyuCMS — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.content.cms`

---

## Obligations spécifiques MiyuCMS

- **Décision** (publication, modération, restauration révision) = StrongFather ; aucun Tool n'exécute de décision métier.
- **Toute écriture** (contenu, média, révision, commentaire) = **WriteIntent** vers KindMother ; aucun accès direct à la persistance depuis le kit.
- **Schéma et périmètre** (contenus, médias, révisions, commentaires) = KindMother ; le kit ne modifie pas le schéma.
- **Niveau de sécurité :** 0 à 2 selon politique d'exposition (contenu public à éditorial sensible) ; cohérent WorrySentinel.
- L'affichage des contenus est du ressort de MiyuWeb (données fournies dans le flux) ; MiyuCMS gère uniquement la gestion éditoriale et la persistance gouvernée.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
