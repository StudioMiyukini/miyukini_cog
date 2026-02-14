# JayXpose — Pages web publiques vitrine

## Contexte

Ce document décrit l’**ajout des pages JayXpose publiques au format web** dans le service WEB (Origin) : présentation et catalogue de produits et services de l’exposant. Ces pages respectent un **layout commun à tous les COG**, récupèrent les données **via KindMother** (base JayXpose), et sont les **jumelles web** des écrans natifs Vitrine (Central). Elles sont **distinctes de la description Home** du portail.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md), [Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md), [Miyukini Web Portal - Surface Web Implementation](../../MiyukiniWebPortal/Miyukini%20Web%20Portal%20-%20Surface%20Web%20Implementation%20et%20Gouvernance.md).

---

## 1. Portée

| Élément | Description |
|--------|-------------|
| **Périmètre** | Pages web publiques vitrine (accueil, catalogue, présentation, contact) servies par le service WEB (Origin) ; layout commun ; données en lecture seule depuis la DB JayXpose (KindMother). |
| **Hors périmètre** | E‑shop (paiement), blog CMS dédié, annuaire global (PUB-E01/PUB-E02) — traités séparément. |

---

## 2. Principes directeurs

### 2.1 Layout commun à tous les COG

- Les pages vitrine publiques (accueil, catalogue, présentation, contact) partagent un **même layout HTML/CSS** pour toutes les vitrines exposant, sur tous les COG.
- Ce layout inclut : en-tête (logo exposant, navigation vitrine), zone de contenu principal, pied de page (coordonnées, réseaux selon confidentialité).
- Personnalisation limitée aux **données** (nom, logo, bannière, couleurs si configurées) et au **contenu** (texte présentation, produits), pas à la structure du layout.

### 2.2 Source des données : KindMother (KM)

- Les données affichées proviennent de la **base JayXpose** (KindMother Daughter). Le service WEB ne fait **aucune écriture** sur cette base pour les pages publiques.
- Accès en **lecture seule** : exposant (par slug vitrine, statut `publiee`), produits catalogue, catégories, pages vitrine (présentation), règles de confidentialité pour l’affichage contact.
- Pas d’accès direct aux Cores par l’utilisateur final ; le service WEB orchestre la lecture via la couche données JayXpose.

### 2.3 Jumelles des pages « natives »

- Chaque page web publique est la **jumelle** d’un écran natif Central :

| Page web publique | Écran natif (Central) | Données |
|-------------------|------------------------|--------|
| Accueil vitrine | Ma vitrine — Prévisualisation (XP-E08) / paramètres (XP-E06) | `exposants` (bannière, slogan, description courte) + produits vedettes |
| Catalogue | Ma vitrine — Prévisualisation | `produits_catalogue`, `produits_visuels`, `categories_produits` |
| Présentation | Ma vitrine — Page Présentation (XP-E07) | `vitrine_pages` (page_type = `presentation`) |
| Contact | Ma vitrine — Prévisualisation | `exposants` (coordonnées filtrées par confidentialité) |

- Même modèle de données ; seule la **présentation** change (natif vs HTML web).

### 2.4 Distinctes de la description Home

- La **Home** du portail (Origin) décrit le **COG / Miyukini / MWS** : présentation générale, catalogue des COGs, téléchargements, documentation, blog, annonces. C’est la page d’accueil du **système**, pas d’un exposant.
- Les **pages JayXpose publiques** sont dédiées à **un exposant** : sa vitrine, son catalogue, sa présentation, son contact. Elles utilisent le **layout vitrine commun** et ne réutilisent pas le layout « Home » du portail.

---

## 3. Architecture d’exposition

### 3.1 Service WEB (Origin)

- Le serveur web Origin peut exposer des **routes vitrine** lorsque la base JayXpose est disponible (chemin configuré ou convention).
- Routes proposées (alignées sur [Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)) :

| Route | Page | Écran doc |
|-------|------|-----------|
| `GET /vitrine` | Liste / redirection (ex. vers première vitrine ou annuaire) | Optionnel |
| `GET /vitrine/{slug}` | Accueil vitrine | PUB-E03 |
| `GET /vitrine/{slug}/catalogue` | Catalogue produits | PUB-E04 |
| `GET /vitrine/{slug}/catalogue/{produit_id}` | Fiche produit | PUB-E04 (détail) |
| `GET /vitrine/{slug}/presentation` | Présentation (contenu riche) | PUB-E05 |
| `GET /vitrine/{slug}/contact` | Contact (coordonnées + formulaire) | PUB-E06 |

- `{slug}` : `vitrine_slug` de l’exposant ; vitrine uniquement si `vitrine_status = 'publiee'`.

### 3.2 Données requises (lecture seule, depuis KM)

- **Exposant** : récupéré par `exposant_by_vitrine_slug(slug)` avec `vitrine_status = 'publiee'`.
- **Produits** : `produits_by_exposant(exposant_id)`, `produit_by_id(id)` ; visuels : `visuels_by_produit(produit_id)`.
- **Catégories** : `categories_by_exposant(exposant_id)`.
- **Page présentation** : `vitrine_pages_by_exposant(exposant_id)` puis page `page_type = 'presentation'`, `is_visible = true`.
- **Contact** : champs exposant filtrés selon `confidentialite_profil` (public / authentifié / etc.).

### 3.3 Layout commun vitrine

- **En-tête** : logo exposant, nom entreprise, navigation (Accueil, Catalogue, Présentation, Contact — selon pages activées).
- **Contenu** : zone principale (hero accueil, grille catalogue, contenu présentation, bloc contact).
- **Pied** : lien « Retour au portail », coordonnées et réseaux (selon confidentialité), mention Miyukini COG / JayXpose.
- Styles : palette éventuelle issue de `vitrine_colors` (exposant) ; défaut commun si non renseigné.

---

## 4. Contrat d’exposition (Surface Web)

Conforme au guide [Surface Web Implementation et Gouvernance](../../MiyukiniWebPortal/Miyukini%20Web%20Portal%20-%20Surface%20Web%20Implementation%20et%20Gouvernance.md) :

| Élément | JayXpose vitrine |
|--------|------------------|
| **Capacités exposées** | `vitrine.get.public`, `catalogue.list.public`, `produit.get.public`, `page.presentation.get.public`, `contact.form.submit` (optionnel) |
| **Données exposées** | Catalogue produits (public), pages vitrine (publiées), informations contact (selon confidentialité) |
| **Actions autorisées** | Consultation catalogue, lecture pages, soumission formulaire contact |
| **Niveau de sécurité** | Lecture : 0 (public) ; formulaire contact : 1 |
| **Pas exposé** | Données sensibles (documents, RIB, champs privés), actions d’édition (Central uniquement) |

---

## 5. Implémentation technique (orientation)

- **Origin** : dépendance optionnelle sur le crate `jayxpose` (feature `legacy-sqlite` pour lecture SQLite). Si le chemin vers la base JayXpose est configuré (ex. `registry.data_dir` ou `jayxpose.db_path`), ouvrir la base en **lecture seule** et exposer les routes `/vitrine/...`.
- **Crate jayxpose** : exposer une méthode `exposant_by_vitrine_slug(slug)` retournant l’exposant dont `vitrine_slug = slug` et `vitrine_status = 'publiee'`.
- **Rendu** : HTML généré côté serveur (même philosophie que les autres pages Origin), avec un **template layout vitrine** commun.

---

## 6. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [JayXpose - Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md)
- [JayXpose - Vérification Conformité](./JayXpose%20-%20Verification%20Conformite%20Implementation.md)
- [Miyukini Web Portal - Surface Web Implementation et Gouvernance](../../MiyukiniWebPortal/Miyukini%20Web%20Portal%20-%20Surface%20Web%20Implementation%20et%20Gouvernance.md)

---

**Document** : JayXpose — Pages web publiques vitrine  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Référence — spécification des pages web publiques et layout commun
