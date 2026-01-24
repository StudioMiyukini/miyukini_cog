# Module Contenu — Contrat fonctionnel

> Fondation du SPM CMS. Gestion des entités de contenu : CRUD, statuts, relations, versioning, métadonnées.

---

## Responsabilité

Le Module Contenu gère les entités de contenu (pages, articles, blocs) : création, lecture, modification, suppression, gestion des statuts (brouillon/publié/archivé), relations entre contenus, versioning et métadonnées.

---

## Entités

### Contenu

**Identité :**
- Identifiant unique (fourni par le kernel : `Id`)
- Type de contenu (page, article, bloc, etc.) — défini par le produit
- Métadonnées de base (titre, description, etc.) — structure définie par le produit

**Cycle de vie :**
- Date de création (fournie par le kernel : `Clock`)
- Date de modification (fournie par le kernel : `Clock`)
- Statut fonctionnel (brouillon, publié, archivé) — états génériques

**Relations :**
- Références vers d'autres contenus (many-to-many, hiérarchiques)
- Relations définies par le produit (ex. "article parent", "contenu lié")

**Versioning :**
- Historique des versions (identifiants de versions)
- Version courante
- Métadonnées de version (auteur, date, commentaire) — si fourni par le produit

**Métadonnées :**
- Données structurées additionnelles — structure définie par le produit
- Pas de format imposé (JSON, clé-valeur, etc.)

---

## Opérations

### Création

**Opération :** Créer un contenu

**Entrées :**
- Type de contenu
- Données de base (titre, etc.)
- Statut initial (par défaut : brouillon)
- Métadonnées optionnelles

**Sorties :**
- Identifiant du contenu créé
- Contenu créé avec dates de création/modification

**Contraintes :**
- L'identifiant est généré par le kernel
- Les dates sont fournies par le kernel
- Le type de contenu est validé par le produit

---

### Lecture

**Opération :** Lire un contenu

**Entrées :**
- Identifiant du contenu
- Option : version spécifique

**Sorties :**
- Contenu complet (données, statut, métadonnées, relations, version)
- Erreur si contenu inexistant

**Contraintes :**
- Lecture possible quel que soit le statut
- Le produit gère les permissions d'accès (hors scope du module)

---

### Modification

**Opération :** Modifier un contenu

**Entrées :**
- Identifiant du contenu
- Données à modifier (partielles)
- Option : créer une nouvelle version

**Sorties :**
- Contenu modifié
- Nouvelle version créée si demandée
- Erreur si contenu inexistant ou état invalide

**Contraintes :**
- La date de modification est mise à jour automatiquement
- Le versioning est optionnel (défini par le produit)
- Les relations peuvent être modifiées

---

### Suppression

**Opération :** Supprimer un contenu

**Entrées :**
- Identifiant du contenu
- Option : suppression douce (archivage) vs suppression définitive

**Sorties :**
- Confirmation de suppression
- Erreur si contenu inexistant ou contraintes (ex. contenus enfants)

**Contraintes :**
- La suppression peut être conditionnée par des relations (ex. contenus enfants) — règles définies par le produit
- L'archivage change le statut sans supprimer les données

---

### Liste / Recherche

**Opération :** Lister des contenus

**Entrées :**
- Filtres (type, statut, date, etc.)
- Tri (date, titre, etc.)
- Pagination (offset, limit)

**Sorties :**
- Liste de contenus correspondant aux critères
- Métadonnées de pagination (total, page, etc.)

**Contraintes :**
- Les filtres sont définis par le produit
- Le tri est défini par le produit
- La recherche full-text est hors scope (Module Recherche)

---

### Relations

**Opération :** Gérer les relations entre contenus

**Entrées :**
- Identifiant du contenu source
- Type de relation
- Identifiant du contenu cible
- Action (créer, supprimer, lister)

**Sorties :**
- Relations créées/supprimées
- Liste des relations

**Contraintes :**
- Les types de relations sont définis par le produit
- Les relations peuvent être bidirectionnelles ou unidirectionnelles
- Les relations peuvent être typées (ex. "parent", "enfant", "lié")

---

### Versioning

**Opération :** Gérer les versions d'un contenu

**Entrées :**
- Identifiant du contenu
- Action (créer version, restaurer version, lister versions)

**Sorties :**
- Version créée avec identifiant
- Version restaurée
- Liste des versions avec métadonnées

**Contraintes :**
- Le versioning est optionnel (activé par le produit)
- Chaque version conserve l'état complet du contenu à un instant donné
- La restauration crée une nouvelle version (pas de modification directe de l'historique)

---

## Invariants

1. **Identité unique :** Un contenu a un identifiant unique et immuable.
2. **Dates cohérentes :** La date de modification est toujours ≥ date de création.
3. **Statuts valides :** Seuls les statuts définis (brouillon, publié, archivé) sont autorisés.
4. **Relations acycliques :** Les relations hiérarchiques ne peuvent pas créer de cycles (si le produit impose cette contrainte).
5. **Versioning cohérent :** Si le versioning est activé, chaque modification crée une version, et la version courante existe toujours.
6. **Intégrité référentielle :** Les relations pointent vers des contenus existants (validation par le produit).

---

## Hors-scope

### Stockage et persistance

- Format de stockage (DB, fichiers, etc.)
- Schéma de base de données
- Indexation et optimisation
- Réplication et sauvegarde

### Permissions et accès

- Qui peut créer/modifier/supprimer
- Règles d'accès par rôle
- Validation des permissions
- Audit des accès

### Rendu et affichage

- Templates de rendu
- Format de sortie (HTML, JSON, etc.)
- Transformation des données
- Prévisualisation

### Recherche full-text

- Indexation full-text
- Recherche sémantique
- Ranking et scoring
- Facettes et filtres avancés

### Workflow métier

- Circuits d'approbation
- Règles de publication conditionnelles
- Validation par rôles
- Notifications de workflow

### SEO et référencement

- Meta tags
- Sitemaps
- Optimisations SEO
- Analytics

### Intégrations externes

- APIs tierces
- Webhooks
- Synchronisation externe
- Connecteurs

---

## Dépendances

**Kernel :**
- `Id` : Génération d'identifiants uniques
- `Clock` : Horodatage (création, modification)
- `Logger` : Logging des opérations (optionnel)

**Autres modules SPM :**
- Aucune (module fondation)

**Produit :**
- Définition des types de contenu
- Structure des métadonnées
- Types de relations
- Règles de validation métier
- Gestion des permissions

---

## Exemples d'usage (pseudo-code)

### Créer un contenu

```
content = ContentManager.create({
    type: "article",
    title: "Mon article",
    status: "draft",
    metadata: { author: "user123" }
})
// Retourne : { id: "abc-123", created_at: "2026-01-23T10:00:00Z", ... }
```

### Modifier un contenu

```
ContentManager.update(
    id: "abc-123",
    updates: { title: "Nouveau titre" },
    create_version: true
)
// Met à jour le contenu et crée une nouvelle version
```

### Lister des contenus

```
contents = ContentManager.list({
    filters: { type: "article", status: "published" },
    sort: { field: "created_at", order: "desc" },
    pagination: { offset: 0, limit: 20 }
})
// Retourne : { items: [...], total: 42, page: 1 }
```

### Gérer des relations

```
ContentManager.add_relation(
    source_id: "abc-123",
    relation_type: "parent",
    target_id: "def-456"
)
// Crée une relation parent-enfant
```

---

## Notes d'implémentation

**Contrat fonctionnel :** Ce document définit le contrat fonctionnel, indépendant de toute implémentation technique. Une implémentation Rust pourrait exposer un trait `ContentManager`, mais le contrat reste fonctionnel.

**Extensibilité :** Le module est conçu pour être extensible par le produit (types de contenu, métadonnées, relations) sans modification du contrat de base.

**Performance :** Les considérations de performance (indexation, cache, etc.) sont du ressort du produit ou d'un module infra futur.
