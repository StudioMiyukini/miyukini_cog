# JayManga — Publication et Catalogue

## Contexte

Ce document detaille les fonctionnalites de **publication et de gestion du catalogue** de JayManga. Il couvre l'import d'oeuvres, la gestion des formats de lecture, l'outil d'optimisation et de compression integre, l'organisation en series, les metadonnees et les statuts de publication. Ces fonctionnalites sont accessibles au vendeur (Admin COG) depuis **Miyukini Central**.

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md).

---

## 1. Formats de lecture supportes

JayManga ne se limite pas au manga classique. Le vendeur declare le format de lecture de chaque oeuvre ; la liseuse s'adapte automatiquement.

### 1.1 Formats natifs

| Format | Identifiant | Ratio typique | Sens de lecture | Comportement liseuse |
|--------|-------------|---------------|-----------------|----------------------|
| **Manga** | `manga` | Portrait (~2:3) | Droite a gauche (RTL) | Navigation page par page ou double-page. Pages affichees en vis-a-vis pour les double-pages (mode desktop). |
| **Webtoon** | `webtoon` | Bande verticale (~1:3+) | Haut en bas | Defilement vertical continu. Les pages/panneaux sont concatenes en une bande unique. Chargement progressif (lazy loading). |
| **Paysage / 16:9** | `landscape` | Paysage (16:9, 16:10) | Gauche a droite (LTR) | Navigation page par page. Ideal pour les illustrations panoramiques, double-pages, planches cinematiques. |
| **Comics** | `comics` | Portrait (~2:3) | Gauche a droite (LTR) | Navigation page par page ou double-page. Meme comportement que `manga` mais en sens inverse. |
| **Format libre** | `free` | Ratio personnalise | Configurable (LTR ou RTL) | La liseuse s'adapte au ratio de chaque page individuellement. Permet de mixer des pages portrait et paysage dans une meme oeuvre. |

### 1.2 Format mixte

Une oeuvre en format `free` peut contenir des pages de ratios differents. La liseuse detecte le ratio de chaque page et ajuste l'affichage :

- Pages portrait → affichage centre, redimensionnement en hauteur.
- Pages paysage → affichage pleine largeur, redimensionnement en largeur.
- Le vendeur peut forcer un comportement par page via les metadonnees de page (optionnel, Phase 2).

### 1.3 Configuration par defaut

Le vendeur configure un **format de lecture par defaut** dans la configuration du service (`SellerConfig.reading_direction`). Ce format s'applique automatiquement aux nouvelles oeuvres mais peut etre ecrase individuellement.

---

## 2. Import d'oeuvres

### 2.1 Methodes d'import

| Methode | Description |
|---------|-------------|
| **Import par fichiers** | L'admin selectionne des fichiers images depuis son systeme de fichiers. Formats acceptes : JPEG, PNG, WebP, AVIF. |
| **Import par dossier** | L'admin selectionne un dossier. JayManga detecte automatiquement la structure : un sous-dossier par chapitre, les fichiers images tries par nom de fichier (tri alphabetique/numerique). |
| **Import par archive** | L'admin importe un fichier ZIP ou CBZ. JayManga extrait les images et detecte la structure (sous-dossiers = chapitres). |
| **Import incrementiel** | L'admin peut ajouter des chapitres a une oeuvre existante sans reimporter l'ensemble. |

### 2.2 Detection automatique de structure

Lors de l'import par dossier ou archive, JayManga applique les regles de detection suivantes :

1. **Racine avec sous-dossiers** : chaque sous-dossier est interprete comme un chapitre. L'ordre des chapitres suit le tri naturel des noms de dossiers.
2. **Racine avec images uniquement** : un seul chapitre est cree contenant toutes les images.
3. **Tri des pages** : les images dans chaque chapitre sont triees par nom de fichier en ordre naturel (`page_1.jpg` < `page_2.jpg` < `page_10.jpg`).
4. **Filtrage** : les fichiers non-image et les fichiers systeme (`.DS_Store`, `Thumbs.db`, etc.) sont ignores.

### 2.3 Formats d'image acceptes

| Format | Extension | Support |
|--------|-----------|---------|
| JPEG | `.jpg`, `.jpeg` | Natif |
| PNG | `.png` | Natif |
| WebP | `.webp` | Natif |
| AVIF | `.avif` | Natif |
| GIF | `.gif` | Natif (premiere frame pour les animes) |
| TIFF | `.tiff`, `.tif` | Converti en JPEG a l'import |
| BMP | `.bmp` | Converti en PNG a l'import |

### 2.4 Aucune limite de stockage

JayManga n'impose **aucun quota de stockage**. La seule limite est la capacite physique du disque du COG vendeur, conformement a LOI-5 (cout proportionnel au hardware). Cela signifie :

- Pas de limite sur le nombre d'oeuvres.
- Pas de limite sur le nombre de pages par oeuvre.
- Pas de limite sur la taille des fichiers images.
- Pas d'abonnement premium pour debloquer du stockage.

Le tableau de bord vendeur affiche l'espace disque utilise par JayManga et l'espace restant sur le COG pour informer le vendeur.

---

## 3. Outil d'optimisation et de compression

### 3.1 Objectif

L'outil d'optimisation integre a JayManga permet de **reduire la taille des fichiers images** servis aux lecteurs distants, afin d'**accelerer le temps de chargement des pages** sans degrader significativement la qualite visuelle. Les fichiers originaux importes par le vendeur sont **toujours preserves**.

### 3.2 Fonctionnement

Lors de l'import d'une oeuvre (ou a la demande), l'outil genere des **variantes optimisees** de chaque page :

```
Page originale (4000x6000, JPEG, 8 Mo)
  ├─ Variante HD   (2000x3000, WebP, qualite 85, ~800 Ko)
  ├─ Variante SD   (1200x1800, WebP, qualite 80, ~350 Ko)
  ├─ Variante mobile (800x1200, WebP, qualite 75, ~150 Ko)
  └─ Miniature     (300x450, WebP, qualite 70, ~30 Ko)
```

### 3.3 Profils de resolution

| Profil | Largeur max | Usage principal | Suffixe fichier |
|--------|-------------|-----------------|-----------------|
| **original** | — | Telechargement hors-ligne, archive | `_orig` |
| **hd** | 2000 px | Lecture desktop plein ecran | `_hd` |
| **sd** | 1200 px | Lecture tablette, desktop fenetre | `_sd` |
| **mobile** | 800 px | Lecture mobile | `_mob` |
| **thumb** | 300 px | Miniature catalogue, couverture liste | `_thumb` |

Les dimensions sont calculees en preservant le ratio d'aspect original. La largeur maximale est le bord le plus long pour les pages portrait ; pour les pages paysage (16:9, landscape), c'est la largeur.

### 3.4 Formats de compression

| Format | Extension | Avantage | Utilisation |
|--------|-----------|----------|-------------|
| **WebP** | `.webp` | Compression superieure a JPEG (~30%), support universel navigateurs modernes | Defaut pour toutes les variantes |
| **AVIF** | `.avif` | Compression encore meilleure (~50% vs JPEG), support croissant | Optionnel, genere si active dans la configuration |
| **JPEG** | `.jpg` | Compatibilite maximale | Fallback pour navigateurs anciens (si configure) |

### 3.5 Modes d'optimisation

| Mode | Description |
|------|-------------|
| **Automatique a l'import** | Active par defaut. Chaque page importee est optimisee en arriere-plan. L'oeuvre est publiable immediatement avec les originaux ; les variantes optimisees deviennent disponibles au fur et a mesure. |
| **Manuel** | L'admin peut declencher l'optimisation d'une oeuvre ou d'un chapitre specifique depuis l'interface de gestion. |
| **Re-optimisation** | L'admin peut re-generer les variantes avec de nouveaux parametres (qualite, profils actifs, format). |
| **Desactivation** | L'admin peut desactiver l'optimisation pour une oeuvre specifique (les originaux sont servis directement). |

### 3.6 Parametres configurables

| Parametre | Type | Defaut | Description |
|-----------|------|--------|-------------|
| `quality_hd` | INTEGER (1-100) | 85 | Qualite de compression pour le profil HD. |
| `quality_sd` | INTEGER (1-100) | 80 | Qualite pour le profil SD. |
| `quality_mobile` | INTEGER (1-100) | 75 | Qualite pour le profil mobile. |
| `quality_thumb` | INTEGER (1-100) | 70 | Qualite pour les miniatures. |
| `output_format` | TEXT | `webp` | Format de sortie (`webp`, `avif`, `jpeg`). |
| `generate_avif` | BOOLEAN | false | Generer aussi des variantes AVIF (en plus du format principal). |
| `active_profiles` | JSON | `["hd","sd","mobile","thumb"]` | Profils de resolution a generer. |
| `max_concurrent_jobs` | INTEGER | 2 | Nombre de pages optimisees en parallele (preserve les performances du COG). |
| `jpeg_fallback` | BOOLEAN | false | Generer un fallback JPEG pour les navigateurs anciens. |

### 3.7 Selection de variante a la lecture

Lorsque le lecteur ouvre une page, la liseuse selectionne automatiquement la variante la plus appropriee :

1. Detection de la **taille de l'ecran** du lecteur (viewport).
2. Detection du **pixel ratio** (retina, HiDPI).
3. Detection du **format supporte** par le navigateur (WebP, AVIF).
4. Selection de la variante la plus proche en resolution sans sous-echantillonnage.
5. Si aucune variante optimisee n'est disponible, l'original est servi.

Le mecanisme utilise les attributs `srcset` et `<picture>` en HTML pour les pages web, ou une logique equivalente dans la liseuse native (Central).

### 3.8 Stockage des variantes

Les variantes optimisees sont stockees via KindMother dans un sous-repertoire dedie de l'oeuvre :

```
/jaymanga/works/{work_id}/chapters/{chapter_id}/
  ├── originals/
  │   ├── page_001.jpg
  │   ├── page_002.png
  │   └── ...
  └── optimized/
      ├── page_001_hd.webp
      ├── page_001_sd.webp
      ├── page_001_mob.webp
      ├── page_001_thumb.webp
      ├── page_002_hd.webp
      └── ...
```

### 3.9 Indicateurs de performance

Le tableau de bord vendeur affiche pour chaque oeuvre :

| Indicateur | Description |
|------------|-------------|
| Taille originale totale | Poids total des fichiers originaux. |
| Taille optimisee totale | Poids total des variantes generees. |
| Ratio de compression | Pourcentage d'espace economise (ex. « 72% plus leger »). |
| Statut d'optimisation | Nombre de pages optimisees / total. Progression en cours. |
| Temps de chargement estime | Estimation du temps de chargement d'une page par profil (sur une connexion 4G typique). |

---

## 4. Metadonnees des oeuvres

### 4.1 Metadonnees obligatoires

| Champ | Description |
|-------|-------------|
| **Titre** | Titre de l'oeuvre. Minimum 1 caractere. |
| **Format de lecture** | `manga` / `webtoon` / `landscape` / `comics` / `free`. |
| **Statut** | `draft` / `published` / `unlisted` / `archived`. |

### 4.2 Metadonnees recommandees

| Champ | Description |
|-------|-------------|
| Auteur(s) | Liste des auteurs avec role optionnel (scenariste, dessinateur, encreur, coloriste). |
| Genre(s) | Un ou plusieurs genres parmi une liste predefinies extensible (action, romance, fantasy, horreur, tranche de vie, sci-fi, comedy, drama, mystery, sports, mecha, isekai, shonen, shojo, seinen, josei, etc.). |
| Synopsis | Resume de l'oeuvre (texte libre, recommande < 500 caracteres). |
| Couverture | Image de couverture (generee automatiquement a partir de la premiere page si non fournie). |
| Langue | Code ISO 639-1 (ex. `fr`, `ja`, `en`, `ko`). |
| Date de publication | Date de la publication originale de l'oeuvre. |
| Tags | Tags libres pour ameliorer la recherche (ex. « ninja », « ecole », « post-apocalyptique »). |

### 4.3 Metadonnees de serie

| Champ | Description |
|-------|-------------|
| Titre de la serie | Nom de la serie regroupant plusieurs volumes. |
| Numero de volume | Position du volume dans la serie. |
| Statut de la serie | `ongoing` / `completed` / `hiatus`. |

---

## 5. Organisation du catalogue

### 5.1 Hierarchie

```
Catalogue vendeur
  └── Serie (optionnel)
        └── Oeuvre (Volume)
              └── Chapitre
                    └── Page
```

Une oeuvre peut exister independamment d'une serie (one-shot, oeuvre unique).

### 5.2 Operations de gestion

| Operation | Description |
|-----------|-------------|
| Creer une serie | Titre, synopsis, couverture, statut. Les oeuvres sont ajoutees manuellement. |
| Reordonner les volumes | Drag-and-drop ou saisie du numero de volume. |
| Reordonner les chapitres | Drag-and-drop ou saisie du numero de chapitre. |
| Reordonner les pages | Drag-and-drop dans l'interface de gestion des pages. |
| Ajouter un chapitre | Import de nouvelles pages dans un chapitre existant ou creation d'un nouveau chapitre. |
| Remplacer une page | Remplacement d'une page specifique (correction d'erreur, mise a jour). L'optimisation est relancee pour la page remplacee. |
| Supprimer une page / chapitre / oeuvre | Suppression avec confirmation. Les licences existantes restent valides (les acheteurs peuvent relire localement si telecharge). |
| Deplacer une oeuvre dans une serie | Association ou dissociation d'une oeuvre et d'une serie. |

### 5.3 Statuts de publication

| Statut | Visible catalogue | Accessible par lien | Achetable |
|--------|-------------------|---------------------|-----------|
| `draft` | Non | Non | Non |
| `published` | Oui | Oui | Oui (si payant) |
| `unlisted` | Non | Oui | Oui (si payant) |
| `archived` | Non | Oui (lecture seule pour les acheteurs existants) | Non |

---

## 6. Pages de demonstration

### 6.1 Configuration

| Parametre | Description |
|-----------|-------------|
| **Nombre de pages demo par defaut** | Configure dans `SellerConfig.default_demo_pages`. Applique automatiquement aux nouvelles oeuvres. |
| **Nombre de pages demo par oeuvre** | Ecrasable individuellement dans `Work.demo_pages_count`. |
| **Minimum** | 1 page (au moins la premiere page est toujours accessible). |
| **Maximum** | 50% du nombre total de pages de l'oeuvre (RM-07). |

### 6.2 Comportement

Les pages de demonstration correspondent aux **N premieres pages** de l'oeuvre (dans l'ordre de lecture). A la fin de la demonstration, un ecran d'incitation a l'achat s'affiche avec le prix, un bouton d'achat et un apercu des informations de l'oeuvre.

Pour les oeuvres gratuites (`pricing_model = free`), toutes les pages sont accessibles sans restriction.

---

## 7. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service. |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Detail du composant liseuse et des modes de lecture. |

---

**Document** : JayManga — Publication et Catalogue
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Specification fonctionnelle detaillee.
