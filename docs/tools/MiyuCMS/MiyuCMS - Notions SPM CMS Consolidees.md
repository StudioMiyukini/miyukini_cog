# MiyuCMS - Notions SPM CMS Consolidees

## Contexte

Ce document consolide les notions structurantes historiquement decrites dans `docs/spm-cms/`, afin de les conserver dans la structure active `docs/tools/MiyuCMS/`.

Objectif : garder les invariants de conception SPM CMS sans maintenir un dossier preliminaire separe.

## Portee

Ce document couvre :

- Le decoupage modulaire SPM CMS par phases.
- Les contrats fonctionnels minimaux par module.
- Le cadre d'implementation des adaptateurs produits vers KindMother.
- Les regles d'evolution anti-bloat.
- Les livrables minimaux attendus par module.

Ce document ne remplace pas les contrats fondateurs MiyuCMS ni les contrats KindMother.

## Decoupage modulaire par phase

| Phase | Modules | Role principal | Dependances minimales |
|---|---|---|---|
| 0 | Content, Hierarchy, Taxonomies | Fondations CMS mutualisables | Kernel + Content |
| 1 | Media references, Publication, Blocks | Capacites coeur CMS | Content + Kernel |
| 2 | Search, History | Recherche et tracabilite | Content, Taxonomies, Kernel |
| 3 | Themes (structure), Import/Export | Capacites optionnelles | Content + Blocks |

Regle de passage de phase : valider les usages en produit pilote avant extension.

## Contrats fonctionnels minimaux par module

### Content

Responsabilite : CRUD de contenu, statuts generiques, relations, versioning, metadonnees.

Operations minimales :

- `create_content`
- `get_content`
- `update_content`
- `delete_content`
- `list_contents`
- `add_relation`
- `remove_relation`
- `list_relations`
- `create_version`
- `get_version`
- `list_versions`
- `restore_version`

Invariants : identite unique, dates coherentes, statuts valides, relations coherentes, versioning coherent si active.

Hors-scope : SEO, rendu, UI, permissions, persistance technique, workflow metier specifique.

### Hierarchy

Responsabilite : arborescence de contenu, navigation structurelle, deplacement de noeuds, prevention des cycles.

Operations minimales :

- `create_root`
- `create_child`
- `parent`
- `children`
- `ancestors`
- `path_to_root`
- `move_node`
- `remove_node`

Invariants : pas de cycle, un parent direct maximum, coherence parent/enfants.

Hors-scope : rendu de navigation, tri visuel, permissions.

### Taxonomies

Responsabilite : classification de contenu (categories, tags, taxonomies personnalisees).

Operations minimales :

- `create_taxonomy`
- `add_term`
- `assign_term`
- `unassign_term`
- `terms_for_entity`
- `entities_for_term`

Invariants : taxonomies independantes du contenu, liaisons bidirectionnelles coherentes.

Hors-scope : rendu de taxonomie, permissions, indexation avancee.

### Media references

Responsabilite : reference fonctionnelle de medias et associations media <-> entites.

Operations minimales :

- `create_media`
- `get_media`
- `delete_media`
- `attach_media_to_entity`
- `detach_media_from_entity`
- `list_media_for_entity`

Invariants : references media coherentes et detachables.

Hors-scope : moteur de stockage physique, CDN, optimisation de delivery.

### Publication

Responsabilite : etats et transitions de publication generiques.

Operations minimales :

- `create_publication`
- `status`
- `schedule`
- `publish_now`
- `archive`
- `effective_status`

Invariants : transitions d'etat explicites, horodatage coherent.

Hors-scope : decisions d'autorisation et workflows metier conditionnels.

### Blocks

Responsabilite : composition logique de blocs (arbre de blocs), references content/media.

Operations minimales : creation, suppression, deplacement, reordonnancement, attache/detache media, lecture bloc, lecture structure complete.

Invariants : arbre acyclique, ordre logique stable, donnees de bloc opaques.

Hors-scope : rendu HTML, layout responsive, theming, editeur WYSIWYG, regles metier, A/B testing.

### Search

Responsabilite : indexation fonctionnelle et recherche par criteres sur champs indexes.

Operations minimales :

- `index_entity`
- `unindex_entity`
- `search`
- `list_indexed_entities`
- `is_indexed`
- `get_indexed_fields`
- `clear_index`

Invariants : index explicite, resultats deterministes a index donne.

Hors-scope : scoring, ranking, full-text linguistique, semantique, permissions, sync auto, cache, recherche distribuee.

### History

Responsabilite : tracabilite des operations et audit fonctionnel.

Statut : notion de module validee conceptuellement, details a contractualiser dans la doc active si implementation engagee.

### Themes (structure) et Import/Export

Responsabilite : capacites optionnelles, ajoutees uniquement si besoin produit confirme.

Regle : ne pas integrer au tronc tant que la reusabilite multi-produits n'est pas validee.

## Cadre adaptateurs produits (SPM -> KindMother)

Principe de reference : `Authoritative Core with Intent-Based Adapters`.

Regles non negociables :

- L'adaptateur implemente un contrat SPM et traduit vers CoreDataAPI.
- Toute ecriture passe par `WriteIntent` KindMother.
- Aucune persistance directe, aucun SQL/ORM/repository dans l'adaptateur.
- Contexte complet requis a chaque appel : utilisateur, autorisation, instance.
- Isolation stricte : aucune fuite de types/erreurs KindMother vers SPM.
- Traduction explicite des erreurs KindMother vers erreurs SPM.
- Concurrence safe si execution multi-thread.

## Regles d'evolution anti-bloat

Ajouter une capacite SPM seulement si :

- Besoin confirme par au moins deux produits distincts.
- Formulation possible sans logique metier specifique.
- Dependances simples et unidirectionnelles.
- Stabilite attendue moyen terme.

Refuser la generalisation si :

- Capacite metier specifique a un seul produit.
- Couplage fort a une stack technique.
- Besoin hypothetique uniquement.
- Complexite superieure au gain systemique.

## Livrables minimaux attendus par module

- Contrat fonctionnel.
- Documentation d'integration.
- Tests unitaires sur operations coeur.
- Test d'integration sur produit pilote.
- Trace de validation avant passage a la phase suivante.

## References canoniques actives

- [MiyuCMS - Documentation Fondatrice](./MiyuCMS%20-%20Documentation%20Fondatrice.md)
- [MiyuCMS - Reference Outils](./MiyuCMS%20-%20Reference%20Outils.md)
- [MiyuCMS - Reference Implementation Guidelines](./implementation/MiyuCMS%20-%20Reference%20Implementation%20Guidelines.md)
- [MiyuCMS - Tool Governance Compliance Contract](./contracts/governance/MiyuCMS%20-%20Tool%20Governance%20Compliance%20Contract.md)
- [KindMother - Interface & Contrat d'Integration](../../cores/KindMother/contracts/api/KindMother%20-%20Interface%20&%20Contrat%20d%27Int%C3%A9gration.md)
- [KindMother - Adapter Compliance Contract](../../cores/KindMother/contracts/compliance/KindMother%20-%20Adapter%20Compliance%20Contract.md)

---

Date de consolidation : 2026-03-05
Statut : Consolidation documentaire apres retrait du dossier preliminaire `docs/spm-cms/`.
