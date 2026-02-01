# Odoo Blog — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Blog** d'Odoo (version 19.0). Il identifie les composants d'interface, les vues backend et frontend, les options de personnalisation et les patterns de navigation pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module website_blog

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues backend (Configuration ‣ Blogs : Blogs, Tags, Tag Categories)
- Frontend : homepage du blog, page article, mode Edit et Customize
- Options de personnalisation (Top Banner, Layout, Sidebar, Posts List, etc.)
- Patterns de navigation (menu Blog, breadcrumb, filtres tag/archive, article suivant)
- Intégration avec le website builder (blocs, éditeur)

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Backend (Configuration)

### 1.1 Blogs : Blogs

**Chemin :** Website ‣ Configuration ‣ Blogs : Blogs

**UI :**
- Liste des blogs (vue liste ou kanban selon configuration)
- Actions : New, Edit, Delete
- Champs principaux : Name, Subtitle (et champs techniques selon version)

**Navigation :**
- Accès depuis le menu Website, sous-section Configuration

### 1.2 Blogs : Tags

**Chemin :** Website ‣ Configuration ‣ Blogs : Tags

**UI :**
- Liste des tags
- Champs : Name, Category (lien vers blog.tag.category), Used in (lignes vers blog.post)
- Création / édition en formulaire

### 1.3 Blogs : Tag Categories

**Chemin :** Website ‣ Configuration ‣ Blogs : Tag Categories

**UI :**
- Gestion des catégories de tags pour regrouper les tags dans la sidebar

---

## 2. Frontend — Page d'accueil du blog (Blog Homepage)

### 2.1 Structure type

- **En-tête / Bannière** : Selon Customize ‣ Top Banner
  - « Name/Latest Post » : titre du dernier article
  - « Drop Zone for Building Blocks » : blocs libres
- **Zone principale** : Liste des articles (Grille ou Liste)
- **Barre latérale (Sidebar)** : Si activée — About us, Archives, Follow Us, Tags List

### 2.2 Liste des articles (Posts List)

- **Cover** : Afficher ou masquer les images de couverture (Cover / No Cover)
- **Author** : Afficher l’auteur
- **Comments/Views Stats** : Nombre de commentaires et de vues
- **Teaser & Tags** : Premières phrases et tags

### 2.3 Options Customize (homepage)

**Accès :** Ouvrir une homepage de blog ‣ Edit ‣ Customize

**Options documentées :**
- **Top Banner** : Name/Latest Post | Drop Zone for Building Blocks
- **Layout** : Grid | List
- **Cards** : Effet carte
- **Increase Readability** : Lisibilité du texte
- **Sidebar** : About us
- **Archives** : Filtre par mois (oui/non)
- **Follow Us** : Liens réseaux sociaux (configurés ailleurs)
- **Tags List** : Liste des tags (oui/non)
- **Posts List** : Cover, Author, Comments/Views Stats, Teaser & Tags

---

## 3. Frontend — Page article (Blog Post)

### 3.1 Structure type

- **Couverture** : Image de couverture (si définie)
- **Titre** : Inside Cover ou Above Cover (Customize)
- **Contenu** : Corps de l’article (blocs website builder)
- **Barre latérale** : Archive, Author, Blog List, Share Links, Tags
- **Fil d’Ariane** : Breadcrumb (optionnel)
- **Bas de page** : Next Article, Comments, Select To Tweet

### 3.2 Options Customize (article)

**Accès :** Ouvrir un article ‣ Edit ‣ Customize

**Options documentées :**
- **Layout** : Title Inside Cover | Title above Cover
- **Increase Readability**
- **Sidebar** : Archive, Author (auteur + date), Blog List, Share Links, Tags
- **Breadcrumb** : Affichage du chemin
- **Bottom** : Next Article, Comments
- **Select To Tweet** : Proposer de tweeter le texte sélectionné

---

## 4. Création / Édition d'article (Frontend)

### 4.1 Création

- **Déclencheur** : + New (en haut à droite du site) ‣ Blog Post
- **Popup** : Sélection du blog, saisie du titre ‣ Save
- **Suite** : Rédaction du contenu et personnalisation de la page (website builder)
- **Publication** : Toggle « Unpublished » en haut à droite ‣ basculer pour publier

### 4.2 Édition

- Mode Edit sur la page de l’article
- Barre d’outils / panneau Customize comme pour le Website Builder
- Modification du contenu (blocs), de la couverture, des tags (via Customize ‣ couverture ‣ Tags)

---

## 5. Patterns de Navigation

| Élément | Rôle |
|--------|------|
| Menu **Blog** | Point d’entrée unique (regroupe tous les blogs) ; ajouté à la première création d’un blog |
| **Homepage blog** | Liste des articles ; filtres sidebar (Archives, Tags) |
| **Page article** | Lecture ; breadcrumb, article suivant, commentaires, partage |
| **Archives** | Sélection d’un mois → liste des articles de ce mois |
| **Tags List** | Clic sur un tag → liste des articles avec ce tag |
| **Next Article** | Lien vers l’article suivant en bas de page |

---

## 6. Intégration Website Builder

- **Edit** : Même mode Edit que le site (bouton Edit sur la page)
- **Customize** : Edit ‣ Customize pour les options spécifiques blog (bannière, layout, sidebar, etc.)
- **Blocs** : Contenu de l’article éditable par blocs (drag-and-drop)
- **Raccourci** : `/` dans l’éditeur de texte pour formater et ajouter des éléments
- **Images** : Intégration Unsplash (images libres de droit) pour illustrer les articles

---

## 7. Responsive et Accessibilité

- Les pages blog suivent le layout du site (responsive)
- Options « Increase Readability » pour améliorer la lisibilité
- Breadcrumb et structure de titres pour la navigation et le SEO
- Partage et commentaires accessibles (selon configuration et droits)

---

## 8. Recommandations pour Miyukini

- **UI Backend** : Écrans de configuration équivalents (Blogs, Tags, Tag Categories) avec nomenclature claire.
- **UI Frontend** : Homepage blog (liste + filtres) et page article (contenu + sidebar + breadcrumb + next/comments) avec options de personnalisation configurables.
- **Customize** : Grouper les options par thème (Layout, Sidebar, Posts List, Bottom) et distinguer « global pour tous les blogs/articles » vs « surcharge par blog/article » si le produit le prévoit.
- **Création d’article** : Workflow en 2 temps (choix blog + titre puis contenu) avec rappel de publication.
- **Navigation** : Menu Blog, tags, archives, article suivant ; respect des principes Façade Publique Gouvernée pour les visiteurs.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
