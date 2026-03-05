# JayManga â€” Portail Agrege et Decouverte

## Contexte

Ce document detaille le **Portail Agrege** de JayManga : une interface inter-COG (Type 3) qui collecte les catalogues des COGs proposant JayManga et les presente dans une **vue unifiee**, emulant l'experience d'un catalogue en ligne centralise (Mangadraft, Manga.io) tout en restant **entierement decentralise**.

Le Portail Agrege permet au lecteur de parcourir un catalogue global de manga sans avoir a visiter chaque COG individuellement. Les oeuvres provenant de COGs actuellement hors-ligne sont **visibles mais grisees** â€” leurs metadonnees restent en cache, mais la lecture n'est pas possible tant que le COG vendeur n'est pas en ligne. Lorsque le lecteur clique sur une oeuvre, il est **redirige vers le Portail du COG vendeur d'origine** pour la lecture, l'achat et le telechargement.

**Principe fondamental** : les fichiers manga ne transitent **jamais** par le COG aggregateur. Celui-ci ne stocke que des metadonnees en cache (titre, couverture, auteurs, genres, prix, statut de presence). La souverainete de chaque COG vendeur est preservee (LOI-3).

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md).

---

## 1. Vision et positionnement

### 1.1 Le probleme resolu

Sans le Portail Agrege, un lecteur doit :
1. Connaitre l'existence d'un COG proposant JayManga (decouverte via MWS ou bouche-a-oreille).
2. Visiter le Portail de chaque COG individuellement.
3. Parcourir chaque catalogue separement.
4. Gerer mentalement la disponibilite de chaque COG.

Ce modele est fonctionnel mais freine la decouverte et l'engagement. Les plateformes centralisees (Mangadraft, Manga.io, Webtoon, Tapas) offrent une experience de navigation fluide dans un catalogue unique. Le Portail Agrege reproduit cette experience sans centraliser les donnees.

### 1.2 Ce que le Portail Agrege est

| Aspect | Description |
|--------|-------------|
| **Vitrine unifiee** | Un point d'entree unique pour parcourir les manga de tous les COGs JayManga connus. |
| **Cache de metadonnees** | Le COG aggregateur stocke un cache des catalogues (titres, couvertures, auteurs, genres, prix) pour un affichage rapide. |
| **Indicateur de presence** | Chaque oeuvre et chaque vendeur affiche son statut : en ligne (actif) ou hors-ligne (grise). |
| **Redirecteur** | Lorsque le lecteur clique sur une oeuvre, il est redirige vers le Portail du COG vendeur pour la lecture, la demonstration et l'achat. |
| **Interface Type 3** | Composante inter-COG du service JayManga, hebergee sur la surface web du COG aggregateur. |

### 1.3 Ce que le Portail Agrege n'est pas

| Aspect | Description |
|--------|-------------|
| **Pas un hebergeur** | Les fichiers manga ne sont jamais copies sur le COG aggregateur. |
| **Pas un intermediaire de vente** | Le paiement se fait directement sur le COG vendeur. Le COG aggregateur ne prend aucune commission. |
| **Pas un service centralise** | N'importe quel COG peut heberger son propre Portail Agrege. Il peut en exister plusieurs en parallele, chacun avec sa propre couverture des COGs connus. |
| **Pas obligatoire** | Un vendeur peut refuser l'indexation par les Portails Agreges (`allow_aggregation = false`). |

---

## 2. Architecture

### 2.1 Vue d'ensemble

```
                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                    â”‚   Trackers MWS           â”‚
                    â”‚   (decouverte des COGs   â”‚
                    â”‚    proposant JayManga)    â”‚
                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                 â”‚ liste des COGs JayManga
                                 â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                COG Aggregateur                          â”‚
â”‚                                                         â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  Collecteur   â”‚â”€â”€â”€â†’â”‚  Cache de catalogues         â”‚  â”‚
â”‚  â”‚  (cron MWS)   â”‚    â”‚  (metadonnees par COG)       â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                      â”‚                  â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚         Portail Agrege (surface web)              â”‚  â”‚
â”‚  â”‚  Catalogue unifie, recherche, filtres, presence   â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚                          â”‚                â”‚
         â–¼                          â–¼                â–¼
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚ COG A    â”‚            â”‚ COG B    â”‚       â”‚ COG C    â”‚
   â”‚ ðŸŸ¢ onlineâ”‚            â”‚ âš« offlineâ”‚       â”‚ ðŸŸ¢ onlineâ”‚
   â”‚ 42 manga â”‚            â”‚ 15 manga â”‚       â”‚ 8 manga  â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 2.2 Roles

| Role | COG | Description |
|------|-----|-------------|
| **Aggregateur** | Le COG hebergeant le Portail Agrege | Collecte les catalogues via le MWS, maintient le cache, sert l'interface web unifiee. |
| **Vendeur** | Tout COG proposant JayManga | Expose une API de federation catalogue. Controle l'opt-in d'aggregation. |
| **Lecteur** | Visiteur du Portail Agrege | Parcourt le catalogue unifie. Est redirige vers le COG vendeur pour la lecture et l'achat. |

### 2.3 Qui peut etre aggregateur ?

N'importe quel COG peut activer la fonction Portail Agrege. Cas d'usage typiques :

| Scenario | Description |
|----------|-------------|
| **COG vendeur qui est aussi aggregateur** | Un vendeur propose son propre catalogue ET affiche les catalogues des autres COGs. Son propre catalogue est mis en avant (section Â« Notre collection Â») tandis que les autres apparaissent dans la section globale. |
| **COG dedie a l'aggregation** | Un COG ne publie pas de manga lui-meme mais heberge uniquement le Portail Agrege comme service communautaire. |
| **Plusieurs aggregateurs concurrents** | Plusieurs COGs hebergent chacun leur Portail Agrege. Chacun voit les COGs que ses trackers MWS connaissent, ce qui peut varier. Pas de monopole. |

---

## 3. Collecte des catalogues

### 3.1 Decouverte des COGs JayManga

L'aggregateur utilise le MWS pour decouvrir les COGs proposant JayManga. Depuis l'introduction des **manifestes de services** dans le Tracker (voir [MWS - Manifestes de Services](../../miyukini-webway-system/protocole/MWS%20-%20Manifestes%20de%20Services.md)), la decouverte est **enrichie** :

```
COG Aggregateur
  â†’ Requete QUERY_MANIFESTS_BY_SERVICE("jaymanga") au Tracker
  â†’ Obtient : liste de manifestes JayManga avec resume du catalogue
    (shop_name, work_count, genres, formats, allow_aggregation, presence)
  â†’ Filtre : COGs avec allow_aggregation = true
  â†’ Compare last_catalog_update avec le cache local
  â†’ Ne contacte directement que les COGs ayant du contenu modifie
```

Ce mecanisme remplace l'ancien flux de decouverte simple (filtrage Passeport uniquement) par une **decouverte enrichie** ou l'aggregateur obtient un apercu du catalogue de chaque vendeur directement depuis le Tracker, sans multiplier les connexions directes.

### 3.2 API de federation catalogue

Chaque COG vendeur expose une API de federation permettant aux aggregateurs de recuperer un resume de son catalogue. Cette API est **opt-in** : le vendeur doit l'activer.

| Endpoint (sur le COG vendeur) | Methode | Description |
|-------------------------------|---------|-------------|
| `GET /api/jaymanga/federation/catalog` | GET | Retourne un resume du catalogue public : liste des oeuvres publiees avec metadonnees legeres. |
| `GET /api/jaymanga/federation/catalog/since/{timestamp}` | GET | Retourne uniquement les oeuvres ajoutees ou modifiees depuis le timestamp (synchronisation incrementielle). |
| `GET /api/jaymanga/federation/info` | GET | Retourne les informations du vendeur : shop_name, description, work_count, avatar, derniere mise a jour. |

### 3.3 Donnees exposees par la federation

Pour chaque oeuvre publiee, l'API de federation retourne :

| Champ | Description |
|-------|-------------|
| `work_id` | Identifiant unique de l'oeuvre sur le COG vendeur. |
| `title` | Titre. |
| `authors` | Auteurs (nom, role). |
| `genres` | Genres. |
| `synopsis` | Resume (tronque a 300 caracteres pour la federation). |
| `cover_thumb_url` | URL de la miniature de couverture (profil `thumb`). |
| `reading_format` | Format de lecture (`manga`, `webtoon`, `landscape`, `comics`, `free`). |
| `pricing_model` | `free` / `paid`. |
| `price` | Prix en centimes (si payant). |
| `currency` | Devise. |
| `chapter_count` | Nombre de chapitres. |
| `total_pages` | Nombre total de pages. |
| `demo_pages_count` | Nombre de pages de demonstration. |
| `series_title` | Titre de la serie (si applicable). |
| `tags` | Tags. |
| `language` | Langue. |
| `published_at` | Date de publication. |
| `updated_at` | Date de derniere modification. |
| `portal_url` | URL directe vers la fiche oeuvre sur le Portail du vendeur. |

### 3.4 Processus de collecte

```
[Periodique â€” configurable, defaut : toutes les 30 minutes]

Phase 1 â€” Decouverte via manifestes Tracker :
1. QUERY_MANIFESTS_BY_SERVICE("jaymanga") au Tracker
   â†’ Obtenir les manifestes avec resume catalogue et statut presence
2. Identifier les COGs avec allow_aggregation = true
3. Comparer last_catalog_update avec le cache local
   â†’ Determiner quels COGs necessitent une synchronisation

Phase 2 â€” Synchronisation directe (COGs modifies uniquement) :
4. Pour chaque COG en ligne, opt-in, et modifie depuis la derniere sync :
   a. GET /api/jaymanga/federation/info â†’ maj infos vendeur
   b. GET /api/jaymanga/federation/catalog/since/{last_sync} â†’ delta catalogue
   c. Telecharger les nouvelles miniatures de couverture
   d. Mettre a jour le cache local
5. Pour les COGs hors-ligne : conserver le cache existant, marquer comme offline
   (le manifeste du Tracker fournit le statut de presence)
6. Pour les COGs nouvellement decouverts : collecte complete (sans /since)
```

Ce flux en deux phases optimise la bande passante : l'aggregateur ne contacte directement que les COGs dont le catalogue a effectivement change, grace au champ `last_catalog_update` du manifeste Tracker.

### 3.5 Controle du vendeur (opt-in)

| Parametre | Description |
|-----------|-------------|
| `allow_aggregation` | BOOLEAN. Defaut : `true`. Si `false`, l'API de federation retourne 403 et l'aggregateur ne peut pas indexer ce COG. |
| `federation_synopsis_length` | INTEGER. Defaut : 300. Longueur max du synopsis expose. |
| `federation_include_prices` | BOOLEAN. Defaut : `true`. Si `false`, les prix ne sont pas exposes (le lecteur decouvre le prix sur le Portail du vendeur). |

Le vendeur peut modifier ces parametres a tout moment depuis Central. Les changements prennent effet au prochain cycle de collecte.

---

## 4. Interface du Portail Agrege

### 4.1 Page d'accueil

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  ðŸ” Rechercher un manga, un auteur, un genre...              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                               â”‚
â”‚  â–¶ Tendances             â–¶ Nouveautes           â–¶ Gratuit    â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”        â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”       â”Œâ”€â”€â”€â”€â”€â”      â”‚
â”‚  â”‚coverâ”‚ â”‚coverâ”‚        â”‚coverâ”‚ â”‚coverâ”‚       â”‚coverâ”‚      â”‚
â”‚  â”‚  ðŸŸ¢ â”‚ â”‚  ðŸŸ¢ â”‚        â”‚  âš« â”‚ â”‚  ðŸŸ¢ â”‚       â”‚  ðŸŸ¢ â”‚      â”‚
â”‚  â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜        â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜       â””â”€â”€â”€â”€â”€â”˜      â”‚
â”‚                                                               â”‚
â”‚  â–¶ Vendeurs en ligne (12)         â–¶ Vendeurs hors-ligne (3)  â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”             â”‚
â”‚  â”‚ COG Alpha  â”‚ â”‚ COG Beta   â”‚    â”‚ COG Gamma  â”‚ (grise)     â”‚
â”‚  â”‚ ðŸŸ¢ 42 mangaâ”‚ â”‚ ðŸŸ¢ 8 manga â”‚    â”‚ âš« 15 mangaâ”‚             â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜             â”‚
â”‚                                                               â”‚
â”‚  â–¶ Catalogue complet                                          â”‚
â”‚  [Filtres] Genre â–¼  Format â–¼  Prix â–¼  Langue â–¼  Statut â–¼    â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”          â”‚
â”‚  â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚          â”‚
â”‚  â”‚  ðŸŸ¢ â”‚ â”‚  ðŸŸ¢ â”‚ â”‚  âš« â”‚ â”‚  ðŸŸ¢ â”‚ â”‚  ðŸŸ¢ â”‚ â”‚  âš« â”‚          â”‚
â”‚  â”‚Titreâ”‚ â”‚Titreâ”‚ â”‚Titreâ”‚ â”‚Titreâ”‚ â”‚Titreâ”‚ â”‚Titreâ”‚          â”‚
â”‚  â”‚Auteurâ”‚ â”‚Auteurâ”‚ â”‚Auteurâ”‚ â”‚Auteurâ”‚ â”‚Auteurâ”‚ â”‚Auteurâ”‚     â”‚
â”‚  â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 4.2 Sections principales

| Section | Description |
|---------|-------------|
| **Barre de recherche** | Recherche textuelle sur titre, auteur, genre, tags. Resultats instantanes avec auto-completion. |
| **Tendances** | Oeuvres les plus consultees ou ajoutees en favoris recemment (calcule par l'aggregateur a partir de ses propres statistiques de clics). |
| **Nouveautes** | Oeuvres ajoutees ou mises a jour recemment, triees par `updated_at`. |
| **Gratuit** | Oeuvres gratuites (`pricing_model = free`). |
| **Vendeurs** | Liste des COGs vendeurs avec nombre d'oeuvres, avatar, statut en ligne/hors-ligne. Vendeurs hors-ligne grises et positionnes apres les vendeurs en ligne. |
| **Catalogue complet** | Vue paginee de toutes les oeuvres avec filtres et tri. |

### 4.3 Filtres et tri

| Filtre | Options |
|--------|---------|
| **Genre** | Multi-selection parmi les genres indexes (action, romance, fantasy, etc.). |
| **Format** | `manga`, `webtoon`, `landscape`, `comics`, `free`. |
| **Prix** | Gratuit / Payant / Fourchette de prix. |
| **Langue** | Filtre par code ISO 639-1. |
| **Disponibilite** | En ligne uniquement / Tous (y compris hors-ligne). |
| **Vendeur** | Filtre par COG vendeur specifique. |

| Tri | Description |
|-----|-------------|
| Pertinence | Defaut pour la recherche textuelle. |
| Nouveaute | `updated_at` decroissant. |
| Popularite | Nombre de clics/favoris sur l'aggregateur. |
| Prix croissant / decroissant | Par prix. |
| Titre A-Z / Z-A | Alphabetique. |

### 4.4 Fiche oeuvre sur le Portail Agrege

La fiche oeuvre sur le Portail Agrege est une **fiche intermediaire** qui affiche les metadonnees en cache et redirige vers le COG vendeur :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                                                           â”‚
â”‚  [Couverture]    Titre de l'Oeuvre                        â”‚
â”‚                  Auteur(s) â€” Genre(s)                     â”‚
â”‚                  Format : Webtoon                         â”‚
â”‚                  12 chapitres Â· 248 pages                 â”‚
â”‚                  Prix : 3,99 â‚¬ â€” 10 pages de demo         â”‚
â”‚                  Langue : FR                              â”‚
â”‚                                                           â”‚
â”‚                  Synopsis :                               â”‚
â”‚                  Lorem ipsum dolor sit amet...            â”‚
â”‚                                                           â”‚
â”‚                  Heberge par : COG Alpha ðŸŸ¢               â”‚
â”‚                                                           â”‚
â”‚          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                â”‚
â”‚          â”‚  Lire / Acheter sur COG Alpha  â”‚  â†’ redirectionâ”‚
â”‚          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                â”‚
â”‚                                                           â”‚
â”‚          OU (si COG hors-ligne) :                         â”‚
â”‚                                                           â”‚
â”‚          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                â”‚
â”‚          â”‚  COG hors-ligne âš«             â”‚  (grise)      â”‚
â”‚          â”‚  Ajouter aux favoris pour      â”‚                â”‚
â”‚          â”‚  etre notifie quand disponible  â”‚                â”‚
â”‚          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                â”‚
â”‚                                                           â”‚
â”‚  Oeuvres similaires :                                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”                        â”‚
â”‚  â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚                        â”‚
â”‚  â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜                        â”‚
â”‚                                                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 4.5 Page vendeur sur le Portail Agrege

Chaque COG vendeur dispose d'une **page vitrine** sur le Portail Agrege :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  [Avatar]  Nom de la librairie                            â”‚
â”‚            Description du vendeur                         â”‚
â”‚            ðŸŸ¢ En ligne â€” 42 oeuvres â€” Depuis 2025         â”‚
â”‚                                                           â”‚
â”‚            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                   â”‚
â”‚            â”‚  Visiter le Portail â†—    â”‚  â†’ redirection    â”‚
â”‚            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                   â”‚
â”‚                                                           â”‚
â”‚  Catalogue de ce vendeur :                                â”‚
â”‚  [Filtres] Genre â–¼  Format â–¼  Prix â–¼                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”               â”‚
â”‚  â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚ â”‚coverâ”‚               â”‚
â”‚  â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”˜               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 5. Gestion de la disponibilite (COGs hors-ligne)

### 5.1 Affichage des COGs hors-ligne

| Element | Comportement en ligne | Comportement hors-ligne |
|---------|----------------------|------------------------|
| **Couverture** | Affichee normalement | Affichee en **niveaux de gris** avec opacite reduite (50%). |
| **Badge de statut** | ðŸŸ¢ vert | âš« gris fonce |
| **Bouton d'action** | Â« Lire / Acheter sur [COG] Â» (cliquable) | Â« COG hors-ligne Â» (desactive, grise) |
| **Position dans les listes** | Melange avec les resultats normaux | Positionne **apres** les oeuvres en ligne, sauf si le tri explicite (prix, titre) est applique. |
| **Fiche oeuvre** | Lien de redirection actif | Lien desactive + suggestion Â« Ajouter aux favoris pour etre notifie Â». |
| **Page vendeur** | Lien Â« Visiter le Portail Â» actif | Lien desactive + message Â« Ce vendeur est actuellement hors-ligne Â». |

### 5.2 Filtre de disponibilite

Le lecteur peut filtrer l'affichage :

| Option | Comportement |
|--------|-------------|
| **Tous** | Affiche les oeuvres en ligne et hors-ligne (defaut). |
| **En ligne uniquement** | Masque les oeuvres dont le COG est hors-ligne. |

### 5.3 Notification de retour en ligne

Si le lecteur a un COG propre et a mis en favoris une oeuvre dont le COG vendeur est hors-ligne, il peut etre notifie lorsque le COG revient en ligne. Ce mecanisme passe par la bibliotheque lecteur (Central) et non par le Portail Agrege :

```
Bibliotheque (Central)
  â†’ Verification periodique de la presence des COGs favoris
  â†’ Si un COG passe de offline a online â†’ notification locale
```

---

## 6. Cache et synchronisation

### 6.1 Structure du cache

Le COG aggregateur maintient un cache local (KindMother) des metadonnees collectees :

| Donnee | Stockage | Duree de retention |
|--------|----------|-------------------|
| Infos vendeur (shop_name, description, avatar) | JSON par COG | Indefinie (mis a jour a chaque cycle) |
| Catalogue (metadonnees des oeuvres) | JSON par COG | Indefinie (mis a jour incrementiellement) |
| Miniatures de couverture | Fichiers images (thumb) | Indefinie (re-telechargees si mises a jour) |
| Statut de presence | En memoire + cache disque | 5 minutes (rafraichi a chaque cycle MWS) |
| Statistiques d'engagement (clics, favoris) | Base locale | Indefinie (calcul interne a l'aggregateur) |

### 6.2 Cycle de synchronisation

| Parametre | Defaut | Description |
|-----------|--------|-------------|
| `sync_interval` | 30 minutes | Intervalle entre les cycles de collecte complete. |
| `presence_refresh` | 5 minutes | Intervalle de rafraichissement de la presence MWS. |
| `max_concurrent_syncs` | 3 | Nombre de COGs synchronises en parallele. |
| `sync_timeout` | 30 secondes | Timeout par requete de federation. |
| `full_resync_interval` | 24 heures | Intervalle entre les resynchronisations completes (sans `/since`). |

### 6.3 Gestion de la fraicheur

Le Portail Agrege affiche pour chaque vendeur l'indicateur de fraicheur du cache :

| Anciennete du cache | Affichage |
|--------------------|-----------|
| < 1 heure | Aucun indicateur (considere frais). |
| 1 heure - 24 heures | Â« Mis a jour il y a X heures Â». |
| > 24 heures | Â« Donnees potentiellement obsoletes â€” derniere synchronisation il y a X jours Â». |

### 6.4 Volume de donnees

Estimation pour un Portail Agrege indexant 100 COGs avec en moyenne 50 oeuvres chacun :

| Donnee | Taille estimee |
|--------|---------------|
| Metadonnees (5000 oeuvres) | ~5 Mo (JSON) |
| Miniatures de couverture (5000 Ã— 30 Ko) | ~150 Mo |
| **Total** | **~155 Mo** |

Le stockage necessaire est minimal et ne represente pas une contrainte pour le COG aggregateur.

---

## 7. Oeuvres similaires et recommandations

### 7.1 Recommandations basiques (V1)

Le Portail Agrege propose des oeuvres similaires sur chaque fiche oeuvre, basees sur :

| Critere | Poids | Description |
|---------|-------|-------------|
| Genres communs | Fort | Oeuvres partageant au moins 2 genres avec l'oeuvre consultee. |
| Tags communs | Moyen | Oeuvres partageant des tags. |
| Meme auteur | Fort | Autres oeuvres du meme auteur (eventuellement sur un autre COG). |
| Meme format | Faible | Oeuvres du meme format de lecture. |
| Meme langue | Moyen | Oeuvres dans la meme langue. |

### 7.2 Engagement et tendances

L'aggregateur collecte des statistiques anonymes d'engagement :

| Statistique | Description |
|-------------|-------------|
| Clics sur fiche oeuvre | Nombre de fois que la fiche intermediaire est consultee. |
| Redirections vers le COG vendeur | Nombre de fois que le lecteur clique Â« Lire / Acheter Â». |
| Ajouts aux favoris (si authentifie) | Si le lecteur a un COG et ajoute un favori depuis le Portail Agrege. |

Ces statistiques alimentent la section Â« Tendances Â» et le tri par popularite. Elles sont **locales a l'aggregateur** et ne sont pas partagees avec les COGs vendeurs (respect de la vie privee du lecteur).

---

## 8. Configuration du Portail Agrege (Central)

L'admin du COG aggregateur configure le Portail Agrege depuis Central :

### 8.1 Parametres generaux

| Parametre | Type | Defaut | Description |
|-----------|------|--------|-------------|
| `aggregator_enabled` | BOOLEAN | false | Active/desactive le Portail Agrege. |
| `aggregator_name` | TEXT | â€” | Nom du Portail Agrege affiche aux lecteurs. |
| `aggregator_description` | TEXT | â€” | Description / sous-titre. |
| `sync_interval_minutes` | INTEGER | 30 | Intervalle de synchronisation des catalogues. |
| `show_offline_works` | BOOLEAN | true | Afficher les oeuvres de COGs hors-ligne (grisees). |
| `highlight_own_catalog` | BOOLEAN | true | Mettre en avant les propres oeuvres du COG aggregateur (si il est aussi vendeur). |
| `max_indexed_cogs` | INTEGER | 500 | Nombre maximum de COGs indexes (protection contre la surcharge). |

### 8.2 Personnalisation visuelle

| Parametre | Description |
|-----------|-------------|
| `theme` | Couleurs, banniere, logo du Portail Agrege. |
| `featured_works` | Liste d'oeuvres mises en avant manuellement par l'admin (section Â« Selection de la redaction Â»). |
| `blocked_cogs` | Liste de COGs explicitement exclus de l'indexation (moderation). |

### 8.3 Moderation

L'admin de l'aggregateur peut :

| Action | Description |
|--------|-------------|
| **Bloquer un COG** | Exclure un COG vendeur de l'indexation. Ses oeuvres disparaissent du Portail Agrege. |
| **Signaler une oeuvre** | Masquer une oeuvre specifique du Portail Agrege (contenu inapproprie, droits d'auteur). L'oeuvre reste accessible sur le Portail du COG vendeur. |
| **Definir des categories mises en avant** | Organiser la page d'accueil avec des selections thematiques. |

---

## 9. Flux utilisateur

### 9.1 Lecteur â€” decouverte et lecture via le Portail Agrege

```
Lecteur â†’ Portail Agrege (surface web du COG aggregateur)
  â†’ Parcourir le catalogue unifie (recherche, filtres, tendances)
  â†’ Ouvrir une fiche oeuvre (fiche intermediaire sur l'aggregateur)
  â†’ [Si COG en ligne] â†’ Clic "Lire / Acheter" â†’ Redirection vers le Portail du COG vendeur
    â†’ Lecture demo / Achat / Telechargement (sur le COG vendeur)
  â†’ [Si COG hors-ligne] â†’ Fiche grisee â†’ Option "Ajouter aux favoris"
```

### 9.2 Vendeur â€” opt-in a l'aggregation

```
Vendeur â†’ Central â†’ JayManga (configuration)
  â†’ Parametres de federation
  â†’ Activer/Desactiver allow_aggregation
  â†’ Configurer les donnees exposees (synopsis, prix)
  â†’ Les Portails Agreges collectent automatiquement au prochain cycle
```

### 9.3 Admin aggregateur â€” gestion du Portail Agrege

```
Admin â†’ Central â†’ JayManga (configuration Portail Agrege)
  â†’ Activer le Portail Agrege
  â†’ Configurer le nom, la description, le theme
  â†’ Gerer les COGs bloques
  â†’ Voir les statistiques d'engagement
  â†’ Definir les oeuvres mises en avant
```

---

## 10. Modele de donnees supplementaire

### 10.1 Cache catalogue agrege (AggregatedCatalogEntry)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de l'entree cache. |
| seller_cog_id | TEXT | Identifiant du COG vendeur. |
| work_id | TEXT | Identifiant de l'oeuvre sur le COG vendeur. |
| title | TEXT | Titre en cache. |
| authors | JSON | Auteurs en cache. |
| genres | JSON | Genres en cache. |
| synopsis | TEXT | Synopsis tronque. |
| cover_thumb_path | TEXT | Chemin local de la miniature en cache. |
| reading_format | TEXT | Format de lecture. |
| pricing_model | TEXT | `free` / `paid`. |
| price | NUMERIC | Prix en centimes. |
| currency | TEXT | Devise. |
| chapter_count | INTEGER | Nombre de chapitres. |
| total_pages | INTEGER | Nombre total de pages. |
| demo_pages_count | INTEGER | Nombre de pages de demonstration. |
| series_title | TEXT (optionnel) | Titre de la serie. |
| tags | JSON | Tags. |
| language | TEXT | Langue. |
| portal_url | TEXT | URL directe vers la fiche sur le Portail du vendeur. |
| published_at | TEXT | ISO 8601. |
| updated_at | TEXT | ISO 8601. |
| cached_at | TEXT | ISO 8601 â€” date de mise en cache. |

### 10.2 Vendeur indexe (IndexedSeller)

| Champ | Type | Description |
|-------|------|-------------|
| cog_id | TEXT (PK) | Identifiant du COG vendeur. |
| shop_name | TEXT | Nom de la librairie. |
| shop_description | TEXT | Description. |
| avatar_path | TEXT | Chemin local de l'avatar en cache. |
| work_count | INTEGER | Nombre d'oeuvres indexees. |
| online_status | TEXT | `online` / `offline` / `unknown`. |
| last_synced_at | TEXT | ISO 8601. |
| last_seen_online_at | TEXT | ISO 8601. |
| blocked | BOOLEAN | Bloque par l'admin de l'aggregateur. |

### 10.3 Statistiques d'engagement (AggregatorStats)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique. |
| seller_cog_id | TEXT | COG vendeur. |
| work_id | TEXT | Oeuvre. |
| event_type | TEXT | `view` / `redirect` / `favorite`. |
| event_date | TEXT | ISO 8601. |
| count | INTEGER | Nombre d'evenements (aggrege par jour). |

---

## 11. Securite et vie privee

| Mesure | Description |
|--------|-------------|
| **Pas de donnees lecteur partagees** | L'aggregateur ne transmet aucune donnee sur les lecteurs aux COGs vendeurs. Les statistiques d'engagement sont locales. |
| **API de federation en lecture seule** | L'API de federation expose uniquement des metadonnees publiques. Aucune ecriture, aucune modification. |
| **Opt-in vendeur** | Un vendeur peut refuser l'indexation a tout moment. |
| **Moderation aggregateur** | L'admin de l'aggregateur peut bloquer des COGs ou masquer des oeuvres. |
| **Pas de proxy de contenu** | Les fichiers manga ne transitent jamais par l'aggregateur. La lecture passe toujours par le COG vendeur d'origine. |
| **Cache de metadonnees uniquement** | Niveau de securite Public (0) a Standard (1). Pas de donnees sensibles sur l'aggregateur. |

---

## 12. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service (DS-12, RM-09, RM-10). |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Favoris cross-COG et notification de retour en ligne. |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Liseuse (la lecture passe toujours par le COG vendeur). |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Presence et decouverte des COGs. |
| [MWS - Trackers](../../miyukini-webway-system/acteurs/MWS%20-%20Trackers.md) | Tracker, manifestes de services (section 5.5). |
| [MWS - Manifestes de Services](../../miyukini-webway-system/protocole/MWS%20-%20Manifestes%20de%20Services.md) | Protocole de manifestes â€” schema JayManga. |
| [Miyukini Conceptual References - Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md) | Classification Type 3 (inter-COG). |

---

**Document** : JayManga â€” Portail Agrege et Decouverte
**Version** : 1.1
**Date** : 2026-02-24
**Statut** : Specification fonctionnelle detaillee.

