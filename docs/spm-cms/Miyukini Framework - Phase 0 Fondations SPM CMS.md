# Phase 0 : Fondations SPM CMS

> MVP minimal du SPM CMS. Modules fondation permettant de créer, organiser et classifier des contenus.

---

## Contexte

La Phase 0 établit les **fondations fonctionnelles** du SPM CMS. Elle fournit les capacités minimales nécessaires pour valider l'architecture SPM et tester avec un produit pilote CMS.

**Principe :** Minimum viable pour démontrer que le SPM peut être consommé par un produit réel sans logique métier spécifique.

---

## Modules de la Phase 0

### Module Contenu

**Responsabilité :** Gestion des entités de contenu (pages, articles, blocs) : CRUD, statuts (brouillon/publié/archivé), relations, versioning, métadonnées.

**Dépendances :** Kernel (Id, Clock, Logger)

**Contrat :** Voir [Module Contenu — Contrat fonctionnel](modules/content/contrat.md)

**Livrables :**
- Contrat fonctionnel
- Documentation
- Démo fonctionnelle

---

### Module Hiérarchie

**Responsabilité :** Organisation hiérarchique des contenus (arborescence, navigation, breadcrumbs).

**Dépendances :** Module Contenu

**Contrat fonctionnel (à formaliser) :**
- Créer une hiérarchie (parent-enfant)
- Déplacer un contenu dans la hiérarchie
- Lister les enfants d'un contenu
- Obtenir le chemin (breadcrumbs) d'un contenu
- Valider l'acyclicité

**Invariants :**
- Pas de cycles dans la hiérarchie
- Un contenu a au plus un parent direct
- La hiérarchie est cohérente avec les contenus existants

**Hors-scope :**
- Rendu de navigation (UI)
- Permissions d'accès par niveau
- Ordre de tri des enfants (géré par le produit)

---

### Module Taxonomies

**Responsabilité :** Classification des contenus (catégories, tags, taxonomies personnalisées).

**Dépendances :** Module Contenu

**Contrat fonctionnel (à formaliser) :**
- Créer une taxonomie (catégorie, tag, etc.)
- Associer un contenu à une taxonomie
- Dissocier un contenu d'une taxonomie
- Lister les taxonomies d'un contenu
- Lister les contenus d'une taxonomie
- Gérer les taxonomies hiérarchiques (si supportées)

**Invariants :**
- Une taxonomie existe indépendamment des contenus
- Un contenu peut avoir plusieurs taxonomies
- Les relations contenu ↔ taxonomie sont bidirectionnelles

**Hors-scope :**
- Rendu des taxonomies (nuages de tags, etc.)
- Recherche par taxonomie (Module Recherche, Phase 2)
- Permissions sur les taxonomies

---

## Critères de validation Phase 0

### Critères fonctionnels

1. **Création de contenus :** Un produit CMS peut créer des pages/articles avec titre, statut initial et métadonnées.
2. **Organisation hiérarchique :** Un produit CMS peut organiser des contenus en arborescence (parent-enfant) et obtenir le chemin (breadcrumbs).
3. **Classification :** Un produit CMS peut classifier des contenus avec des catégories et tags, et lister les contenus par taxonomie.
4. **Modification et suppression :** Un produit CMS peut modifier et supprimer des contenus, avec gestion des relations.
5. **Liste et filtres :** Un produit CMS peut lister des contenus avec filtres basiques (type, statut) et tri.

### Critères techniques

1. **Intégration kernel :** Les modules utilisent correctement le kernel (Id, Clock, Logger).
2. **Contrats respectés :** Les contrats fonctionnels sont respectés sans logique métier spécifique.
3. **Dépendances claires :** Les dépendances entre modules sont explicites et unidirectionnelles.
4. **Tests :** Chaque module dispose de tests validant les opérations de base.

### Critères produit

1. **Produit pilote :** Un produit CMS minimal peut consommer les 3 modules et créer/organiser/classifier des contenus.
2. **Démo fonctionnelle :** Une démo montre les capacités Phase 0 en action.
3. **Documentation :** Chaque module dispose d'une documentation minimale (contrat, README, exemples).

---

## Ordre de développement recommandé

### Étape 1 : Module Contenu

**Priorité :** Critique — fondation de tout le reste.

**Séquence :**
1. Formaliser le contrat fonctionnel (entités, opérations, invariants)
2. Implémenter les opérations de base (CRUD)
3. Ajouter gestion des statuts
4. Ajouter gestion des relations (optionnel pour MVP)
5. Ajouter versioning (optionnel pour MVP)
6. Tests et démo

**Validation :** Un produit peut créer, lire, modifier, supprimer des contenus avec statuts.

---

### Étape 2 : Module Hiérarchie

**Priorité :** Haute — nécessaire pour organiser les contenus.

**Séquence :**
1. Formaliser le contrat fonctionnel
2. Implémenter création de hiérarchie (parent-enfant)
3. Implémenter déplacement dans la hiérarchie
4. Implémenter breadcrumbs
5. Valider acyclicité
6. Tests et démo

**Validation :** Un produit peut organiser des contenus en arborescence et obtenir le chemin.

---

### Étape 3 : Module Taxonomies

**Priorité :** Haute — nécessaire pour classifier les contenus.

**Séquence :**
1. Formaliser le contrat fonctionnel
2. Implémenter création de taxonomies
3. Implémenter association contenu ↔ taxonomie
4. Implémenter listes bidirectionnelles
5. Tests et démo

**Validation :** Un produit peut classifier des contenus et lister par taxonomie.

---

## Livrables Phase 0

### Documentation

- [x] Contrat fonctionnel Module Contenu
- [ ] Contrat fonctionnel Module Hiérarchie
- [ ] Contrat fonctionnel Module Taxonomies
- [ ] README Module Contenu
- [ ] README Module Hiérarchie
- [ ] README Module Taxonomies

### Code

- [ ] Implémentation Module Contenu
- [ ] Implémentation Module Hiérarchie
- [ ] Implémentation Module Taxonomies
- [ ] Tests unitaires (chaque module)
- [ ] Tests d'intégration (modules + kernel)

### Démo / Produit pilote

- [ ] Démo Module Contenu
- [ ] Démo Module Hiérarchie
- [ ] Démo Module Taxonomies
- [ ] Produit pilote CMS consommant les 3 modules
- [ ] Documentation d'exécution

---

## Critères de passage Phase 0 → Phase 1

**Fonctionnel :**
- Les 3 modules sont fonctionnels et testés
- Un produit CMS minimal peut créer, organiser et classifier des contenus
- Les contrats sont respectés sans logique métier spécifique

**Technique :**
- Intégration kernel validée
- Dépendances entre modules claires
- Tests passent

**Produit :**
- Produit pilote démontre les capacités Phase 0
- Documentation complète pour chaque module
- Démo fonctionnelle disponible

**Décision :** Phase 0 validée si tous les critères ci-dessus sont remplis. Passage à Phase 1 (Modules cœur : Références Média, Publication, Blocs).

---

## Risques et garde-fous

### Risque : Dérive métier

**Garde-fou :** Toute logique métier conditionnelle (ex. validation par rôles, règles de publication spécifiques) doit être refusée et laissée au produit.

### Risque : Couplage technique

**Garde-fou :** Aucune dépendance technique lourde (DB, framework, etc.) ne doit être imposée. Le module expose des contrats fonctionnels.

### Risque : Anticipation

**Garde-fou :** Ne pas ajouter de features "au cas où". Se limiter aux opérations de base validées par le contrat fonctionnel.

### Risque : Bloat

**Garde-fou :** Si une opération n'est pas utilisée par ≥2 produits distincts, elle reste optionnelle ou est laissée au produit.

---

## Notes

**Phase 0 = MVP fonctionnel.** L'objectif n'est pas la complétude, mais la validation de l'architecture SPM et la démonstration que des produits peuvent consommer les modules sans logique métier spécifique.

**Extensibilité future :** Les modules Phase 0 sont conçus pour être extensibles (métadonnées, relations, etc.) sans modification du contrat de base.

**Validation produit :** Un produit CMS minimal doit pouvoir fonctionner avec uniquement les modules Phase 0 pour valider l'approche.
