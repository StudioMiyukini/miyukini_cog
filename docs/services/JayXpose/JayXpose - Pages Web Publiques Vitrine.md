# JayXpose â€” Pages web publiques vitrine

## Contexte

Ce document dÃ©crit lâ€™**ajout des pages JayXpose publiques au format web** dans le service WEB (Origin) : prÃ©sentation et catalogue de produits et services de lâ€™exposant. Ces pages respectent un **layout commun Ã  tous les COG**, rÃ©cupÃ¨rent les donnÃ©es **via KindMother** (base JayXpose), et sont les **jumelles web** des Ã©crans natifs Vitrine (Central). Elles sont **distinctes de la description Home** du portail.

**RÃ©fÃ©rences** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md), [Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md), [Miyukini Web Portal - Surface Web Implementation](..//MiyukiniWebPortal//Miyukini%20Web%20Portal%20-%20Surface%20Web%20Implementation%20et%20Gouvernance.md).

---

## 1. PortÃ©e

| Ã‰lÃ©ment | Description |
|--------|-------------|
| **PÃ©rimÃ¨tre** | Pages web publiques vitrine (accueil, catalogue, prÃ©sentation, contact) servies par le service WEB (Origin) ; layout commun ; donnÃ©es en lecture seule depuis la DB JayXpose (KindMother). |
| **Hors pÃ©rimÃ¨tre** | Eâ€‘shop (paiement), blog CMS dÃ©diÃ©, annuaire global (PUB-E01/PUB-E02) â€” traitÃ©s sÃ©parÃ©ment. |

---

## 2. Principes directeurs

### 2.1 Layout commun Ã  tous les COG

- Les pages vitrine publiques (accueil, catalogue, prÃ©sentation, contact) partagent un **mÃªme layout HTML/CSS** pour toutes les vitrines exposant, sur tous les COG.
- Ce layout inclut : en-tÃªte (logo exposant, navigation vitrine), zone de contenu principal, pied de page (coordonnÃ©es, rÃ©seaux selon confidentialitÃ©).
- Personnalisation limitÃ©e aux **donnÃ©es** (nom, logo, banniÃ¨re, couleurs si configurÃ©es) et au **contenu** (texte prÃ©sentation, produits), pas Ã  la structure du layout.

### 2.2 Source des donnÃ©es : KindMother (KM)

- Les donnÃ©es affichÃ©es proviennent de la **base JayXpose** (KindMother Daughter). Le service WEB ne fait **aucune Ã©criture** sur cette base pour les pages publiques.
- AccÃ¨s en **lecture seule** : exposant (par slug vitrine, statut `publiee`), produits catalogue, catÃ©gories, pages vitrine (prÃ©sentation), rÃ¨gles de confidentialitÃ© pour lâ€™affichage contact.
- Pas dâ€™accÃ¨s direct aux Cores par lâ€™utilisateur final ; le service WEB orchestre la lecture via la couche donnÃ©es JayXpose.

### 2.3 Jumelles des pages Â« natives Â»

- Chaque page web publique est la **jumelle** dâ€™un Ã©cran natif Central :

| Page web publique | Ã‰cran natif (Central) | DonnÃ©es |
|-------------------|------------------------|--------|
| Accueil vitrine | Ma vitrine â€” PrÃ©visualisation (XP-E08) / paramÃ¨tres (XP-E06) | `exposants` (banniÃ¨re, slogan, description courte) + produits vedettes |
| Catalogue | Ma vitrine â€” PrÃ©visualisation | `produits_catalogue`, `produits_visuels`, `categories_produits` |
| PrÃ©sentation | Ma vitrine â€” Page PrÃ©sentation (XP-E07) | `vitrine_pages` (page_type = `presentation`) |
| Contact | Ma vitrine â€” PrÃ©visualisation | `exposants` (coordonnÃ©es filtrÃ©es par confidentialitÃ©) |

- MÃªme modÃ¨le de donnÃ©es ; seule la **prÃ©sentation** change (natif vs HTML web).

### 2.4 Distinctes de la description Home

- La **Home** du portail (Origin) dÃ©crit le **COG / Miyukini / MWS** : prÃ©sentation gÃ©nÃ©rale, catalogue des COGs, tÃ©lÃ©chargements, documentation, blog, annonces. Câ€™est la page dâ€™accueil du **systÃ¨me**, pas dâ€™un exposant.
- Les **pages JayXpose publiques** sont dÃ©diÃ©es Ã  **un exposant** : sa vitrine, son catalogue, sa prÃ©sentation, son contact. Elles utilisent le **layout vitrine commun** et ne rÃ©utilisent pas le layout Â« Home Â» du portail.

---

## 3. Architecture dâ€™exposition

### 3.1 Service WEB (Origin)

- Le serveur web Origin peut exposer des **routes vitrine** lorsque la base JayXpose est disponible (chemin configurÃ© ou convention).
- Routes proposÃ©es (alignÃ©es sur [Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)) :

| Route | Page | Ã‰cran doc |
|-------|------|-----------|
| `GET /vitrine` | Liste / redirection (ex. vers premiÃ¨re vitrine ou annuaire) | Optionnel |
| `GET /vitrine/{slug}` | Accueil vitrine | PUB-E03 |
| `GET /vitrine/{slug}/catalogue` | Catalogue produits | PUB-E04 |
| `GET /vitrine/{slug}/catalogue/{produit_id}` | Fiche produit | PUB-E04 (dÃ©tail) |
| `GET /vitrine/{slug}/presentation` | PrÃ©sentation (contenu riche) | PUB-E05 |
| `GET /vitrine/{slug}/contact` | Contact (coordonnÃ©es + formulaire) | PUB-E06 |

- `{slug}` : `vitrine_slug` de lâ€™exposant ; vitrine uniquement si `vitrine_status = 'publiee'`.

### 3.2 DonnÃ©es requises (lecture seule, depuis KM)

- **Exposant** : rÃ©cupÃ©rÃ© par `exposant_by_vitrine_slug(slug)` avec `vitrine_status = 'publiee'`.
- **Produits** : `produits_by_exposant(exposant_id)`, `produit_by_id(id)` ; visuels : `visuels_by_produit(produit_id)`.
- **CatÃ©gories** : `categories_by_exposant(exposant_id)`.
- **Page prÃ©sentation** : `vitrine_pages_by_exposant(exposant_id)` puis page `page_type = 'presentation'`, `is_visible = true`.
- **Contact** : champs exposant filtrÃ©s selon `confidentialite_profil` (public / authentifiÃ© / etc.).

### 3.3 Layout commun vitrine

- **En-tÃªte** : logo exposant, nom entreprise, navigation (Accueil, Catalogue, PrÃ©sentation, Contact â€” selon pages activÃ©es).
- **Contenu** : zone principale (hero accueil, grille catalogue, contenu prÃ©sentation, bloc contact).
- **Pied** : lien Â« Retour au portail Â», coordonnÃ©es et rÃ©seaux (selon confidentialitÃ©), mention Miyukini COG / JayXpose.
- Styles : palette Ã©ventuelle issue de `vitrine_colors` (exposant) ; dÃ©faut commun si non renseignÃ©.

---

## 4. Contrat dâ€™exposition (Surface Web)

Conforme au guide [Surface Web Implementation et Gouvernance](..//MiyukiniWebPortal//Miyukini%20Web%20Portal%20-%20Surface%20Web%20Implementation%20et%20Gouvernance.md) :

| Ã‰lÃ©ment | JayXpose vitrine |
|--------|------------------|
| **CapacitÃ©s exposÃ©es** | `vitrine.get.public`, `catalogue.list.public`, `produit.get.public`, `page.presentation.get.public`, `contact.form.submit` (optionnel) |
| **DonnÃ©es exposÃ©es** | Catalogue produits (public), pages vitrine (publiÃ©es), informations contact (selon confidentialitÃ©) |
| **Actions autorisÃ©es** | Consultation catalogue, lecture pages, soumission formulaire contact |
| **Niveau de sÃ©curitÃ©** | Lecture : 0 (public) ; formulaire contact : 1 |
| **Pas exposÃ©** | DonnÃ©es sensibles (documents, RIB, champs privÃ©s), actions dâ€™Ã©dition (Central uniquement) |

---

## 5. ImplÃ©mentation technique (orientation)

- **Origin** : dÃ©pendance optionnelle sur le crate `jayxpose` (feature `legacy-sqlite` pour lecture SQLite). Si le chemin vers la base JayXpose est configurÃ© (ex. `registry.data_dir` ou `jayxpose.db_path`), ouvrir la base en **lecture seule** et exposer les routes `/vitrine/...`.
- **Crate jayxpose** : exposer une mÃ©thode `exposant_by_vitrine_slug(slug)` retournant lâ€™exposant dont `vitrine_slug = slug` et `vitrine_status = 'publiee'`.
- **Rendu** : HTML gÃ©nÃ©rÃ© cÃ´tÃ© serveur (mÃªme philosophie que les autres pages Origin), avec un **template layout vitrine** commun.

---

## 6. RÃ©fÃ©rences

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [JayXpose - Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md)
- [JayXpose - VÃ©rification ConformitÃ©](./JayXpose%20-%20Verification%20Conformite%20Implementation.md)
- [Miyukini Web Portal - Surface Web Implementation et Gouvernance](..//MiyukiniWebPortal//Miyukini%20Web%20Portal%20-%20Surface%20Web%20Implementation%20et%20Gouvernance.md)

---

**Document** : JayXpose â€” Pages web publiques vitrine  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : RÃ©fÃ©rence â€” spÃ©cification des pages web publiques et layout commun

