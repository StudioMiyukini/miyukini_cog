# JayXpose — Catalogue Produits

## Contexte

Ce document spécifie le **module catalogue de produits** du service **JayXpose**. Le catalogue permet à chaque exposant de gérer ses produits, créations ou services avec fiches détaillées, visuels, catégories et mise en avant. Le catalogue est consultable depuis le **site vitrine** de l'exposant, depuis l'**annuaire des exposants** et depuis le **répertoire JayFestival**.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) (JXP-20 à JXP-29), [Operateurs et Toolkits](./JayXpose%20-%20Operateurs%20et%20Toolkits.md).

## Portée / Scope

- **Périmètre** : Spécification complète du catalogue (fiches produits, catégories, visuels, vedettes, consultation publique, intégration vitrine et JayFestival).
- **Hors périmètre** : Vente en ligne / paiement (Miyustore) ; gestion de stock avancée ; place de marché.

---

## 1. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Propriété exposant** | Chaque produit appartient à un seul exposant. L'exposant est le seul à pouvoir créer, modifier, supprimer ses produits. |
| **Gouvernance COG** | CRUD gouverné par StrongFather (décision), KindMother (persistance), Master Butler (capacités), WorrySentinel (sécurité). |
| **Pas de vente directe** | Le catalogue est un outil de présentation, pas de commerce en ligne. Si l'exposant souhaite vendre, renvoi vers Miyustore ou site externe. |
| **Multi-canal** | Le catalogue est consommé par la vitrine JayXpose, l'annuaire, et JayFestival. Une seule source de vérité. |
| **Sécurité** | Catalogue public = Standard (1). Données prix/stratégie = Sensitive (2) si l'exposant les considère confidentielles (option). |

---

## 2. Modèle de données

### 2.1 Fiche produit

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | Oui (auto) | Identifiant unique du produit. |
| exposant_id | UUID (FK) | Oui | Exposant propriétaire. |
| name | TEXT | Oui | Nom du produit (max 200 caractères). |
| description | TEXT | Non | Description détaillée (texte formaté, max 2000 caractères). |
| price | NUMERIC | Non | Prix en centimes (nullable = « Sur demande »). |
| currency | TEXT | Non | Devise (défaut : EUR). |
| category_id | UUID (FK) | Non | Catégorie du produit. |
| availability | TEXT | Oui (défaut) | `disponible` / `rupture` / `sur_commande`. Défaut : `disponible`. |
| is_featured | BOOLEAN | Non | Produit vedette (mis en avant). Défaut : false. |
| sort_order | INTEGER | Non | Ordre d'affichage dans la liste. |
| created_at | TIMESTAMPTZ | Oui (auto) | Date de création. |
| updated_at | TIMESTAMPTZ | Oui (auto) | Date de dernière modification. |

### 2.2 Catégorie

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | Oui (auto) | Identifiant unique. |
| exposant_id | UUID (FK) | Oui | Exposant propriétaire. |
| name | TEXT | Oui | Nom de la catégorie (max 100 caractères). |
| description | TEXT | Non | Description optionnelle de la catégorie. |
| sort_order | INTEGER | Non | Ordre d'affichage. |
| created_at | TIMESTAMPTZ | Oui (auto) | Date de création. |

### 2.3 Visuel produit

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | Oui (auto) | Identifiant unique. |
| produit_id | UUID (FK) | Oui | Produit associé. |
| url | TEXT | Oui | URL du fichier image (Supabase Storage alpha). |
| alt_text | TEXT | Non | Texte alternatif (accessibilité, SEO). |
| is_primary | BOOLEAN | Non | Image principale (défaut : false ; la première uploadée est primaire). |
| sort_order | INTEGER | Non | Ordre dans la galerie. |
| created_at | TIMESTAMPTZ | Oui (auto) | Date de création. |

---

## 3. Règles métier

### 3.1 Limites

| Règle | Valeur | Justification |
|-------|--------|---------------|
| Produits par exposant (max) | 500 | Performance et scalabilité (NFR-JXP-05). |
| Visuels par produit (max) | 5 | Stockage et UX. |
| Catégories par exposant (max) | 50 | UX et navigation. |
| Produits vedettes (max) | 6 | Affichage vitrine et annuaire. |
| Taille visuel (max) | 5 Mo | Stockage et performance. |
| Formats visuels | PNG, JPG, WEBP | Standards web. |
| Dimensions recommandées | 800x800 px minimum | Qualité d'affichage. |

### 3.2 Règles de suppression

| Action | Comportement |
|--------|-------------|
| Supprimer un produit | Soft delete (archivage). Le produit disparaît du catalogue public mais reste en base (historique). |
| Supprimer une catégorie | Possible si aucun produit n'y est lié. Sinon : déplacer les produits vers « Sans catégorie » ou refuser. |
| Supprimer un visuel | Suppression réelle (le fichier est retiré du Storage). |

### 3.3 Règles de visibilité

| Contexte | Visibilité catalogue |
|----------|----------------------|
| Vitrine publiée | Tous les produits (non archivés) sont visibles sur la page catalogue vitrine. |
| Annuaire (fiche détaillée) | Produits vedettes affichés en aperçu (max 6). Lien vers la vitrine pour le catalogue complet. |
| JayFestival (répertoire) | Produits vedettes affichés dans la fiche exposant. Lien vers la vitrine. |
| Vitrine non publiée | Catalogue non accessible publiquement (sauf en prévisualisation pour l'exposant). |

---

## 4. Parcours utilisateur

### 4.1 Créer un produit

```
[Mon catalogue] → Button « Ajouter un produit »
    → Formulaire (nom, description, prix, catégorie, disponibilité)
    → Upload visuels (1 à 5)
    → Désigner image principale
    → Optionnel : cocher « Produit vedette »
    → Enregistrer
    → Retour liste catalogue
```

### 4.2 Modifier un produit

```
[Mon catalogue] → Clic sur un produit
    → Formulaire pré-rempli
    → Modification des champs
    → Ajouter / supprimer / réordonner visuels
    → Enregistrer
```

### 4.3 Gérer les catégories

```
[Mon catalogue] → Button « Gérer les catégories »
    → Liste des catégories (nom, nb produits)
    → Ajouter : Input nom → Enregistrer
    → Renommer : Clic édition → modifier → Enregistrer
    → Supprimer : Confirmation si pas de produits liés
    → Réordonner : Drag & drop
```

### 4.4 Consulter le catalogue (visiteur)

```
[Vitrine exposant] → Page Catalogue
    → Filtre par catégorie / recherche
    → Grille de produits (image, nom, prix, disponibilité)
    → Clic produit → Fiche détaillée (galerie, description, prix, catégorie)
```

---

## 5. Intégration

### 5.1 Vitrine JayXpose

- La **page Catalogue** de la vitrine affiche tous les produits (non archivés) de l'exposant.
- La **page Accueil** affiche les produits vedettes (jusqu'à 6).
- Le catalogue hérite de la personnalisation vitrine (couleurs, mise en page).

### 5.2 Annuaire JayXpose

- La **fiche détaillée** d'un exposant dans l'annuaire affiche un aperçu des produits vedettes.
- Lien « Voir le catalogue complet » → vitrine de l'exposant.

### 5.3 JayFestival

- La **fiche exposant** dans le répertoire JayFestival affiche les produits vedettes.
- L'**organisateur** peut voir le catalogue de l'exposant dans le contexte d'une candidature.
- Le catalogue est en **lecture seule** côté JayFestival.

---

## 6. Requêtes SQL (alpha Supabase)

### 6.1 Liste produits d'un exposant

```sql
SELECT p.*, pv.url AS primary_image_url, c.name AS category_name
FROM produits_catalogue p
LEFT JOIN produits_visuels pv ON pv.produit_id = p.id AND pv.is_primary = true
LEFT JOIN categories_produits c ON c.id = p.category_id
WHERE p.exposant_id = :exposant_id
ORDER BY p.sort_order, p.created_at DESC;
```

### 6.2 Produits vedettes d'un exposant

```sql
SELECT p.*, pv.url AS primary_image_url
FROM produits_catalogue p
LEFT JOIN produits_visuels pv ON pv.produit_id = p.id AND pv.is_primary = true
WHERE p.exposant_id = :exposant_id AND p.is_featured = true
ORDER BY p.sort_order
LIMIT 6;
```

### 6.3 Catalogue public (vitrine)

```sql
SELECT p.*, pv.url AS primary_image_url, c.name AS category_name
FROM produits_catalogue p
LEFT JOIN produits_visuels pv ON pv.produit_id = p.id AND pv.is_primary = true
LEFT JOIN categories_produits c ON c.id = p.category_id
WHERE p.exposant_id = :exposant_id
ORDER BY p.sort_order, p.name
LIMIT :limit OFFSET :offset;
```

### 6.4 Filtrer par catégorie

```sql
SELECT p.*, pv.url AS primary_image_url
FROM produits_catalogue p
LEFT JOIN produits_visuels pv ON pv.produit_id = p.id AND pv.is_primary = true
WHERE p.exposant_id = :exposant_id AND p.category_id = :category_id
ORDER BY p.sort_order, p.name;
```

---

## 7. RLS (alpha Supabase)

```sql
-- Lecture publique des produits (exposant visible en annuaire)
CREATE POLICY "produits_select_public"
  ON produits_catalogue FOR SELECT
  USING (
    EXISTS (SELECT 1 FROM exposants e WHERE e.id = produits_catalogue.exposant_id AND e.visible_annuaire = true)
  );

-- Écriture : propriétaire uniquement
CREATE POLICY "produits_insert_own"
  ON produits_catalogue FOR INSERT
  WITH CHECK (exposant_id = auth.uid());

CREATE POLICY "produits_update_own"
  ON produits_catalogue FOR UPDATE
  USING (exposant_id = auth.uid())
  WITH CHECK (exposant_id = auth.uid());

CREATE POLICY "produits_delete_own"
  ON produits_catalogue FOR DELETE
  USING (exposant_id = auth.uid());
```

---

## 8. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Operateurs et Toolkits](./JayXpose%20-%20Operateurs%20et%20Toolkits.md)
- [JayXpose - Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)

---

**Document** : JayXpose — Catalogue Produits
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : Référence produit
