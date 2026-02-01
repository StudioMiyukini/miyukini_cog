# Odoo Blog — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Blog** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module website_blog

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux (création blog, article, tags, personnalisation)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Administrateur Site / Éditeur (Website Administrator)

**Profil :**
- Rôle : Gestion des blogs et de la configuration
- Responsabilités :
  - Créer et configurer les blogs (nom, sous-titre)
  - Gérer les tags et catégories de tags (Website ‣ Configuration ‣ Blogs : Tags / Tag Categories)
  - Personnaliser les homepages de blog et les pages d’article (Edit ‣ Customize)
  - Définir les options globales (sidebar, layout, archives, partage)

**Besoins :**
- Accès backend (Configuration ‣ Blogs)
- Accès frontend en mode Edit pour personnaliser homepages et articles
- Compréhension des options « appliquées à tous les blogs / tous les articles »

**Permissions :**
- Droits d’édition sur le module Website / Blog

### 1.2 Rédacteur / Créateur de contenu (Content Writer)

**Profil :**
- Rôle : Rédaction et publication d’articles
- Responsabilités :
  - Créer des articles (+ New ‣ Blog Post)
  - Rédiger titre et contenu
  - Ajouter couverture, médias (ex. Unsplash)
  - Associer des tags
  - Publier (toggle Unpublished → Published)

**Besoins :**
- Workflow simple : choix du blog, titre, puis contenu et personnalisation
- Rappel explicite de publier (éviter l’oubli du toggle)
- Accès aux images libres de droit et à la mise en forme (/ dans l’éditeur)

**Permissions :**
- Droits de création/édition sur les articles de blog

### 1.3 Visiteur (Public / Utilisateur externe)

**Profil :**
- Rôle : Lecture des articles, navigation, filtrage
- Responsabilités : Aucune ; consulter les blogs, filtrer par tag ou par mois (archives), commenter si activé

**Besoins :**
- Liste d’articles claire (grille ou liste)
- Filtrage par tag et par archive (mois)
- Lecture confortable (lisibilité, breadcrumb, article suivant, partage)
- Commentaires et partage social si proposés

**Permissions :**
- Aucun droit d’édition ; accès aux articles publiés uniquement

### 1.4 Lecteur engagé (Commentaires, partage)

**Profil :**
- Rôle : Commenter, partager, sélectionner du texte pour tweeter
- Responsabilités : Participer (commentaires), diffuser (Share Links, Select To Tweet)

**Besoins :**
- Commentaires activés et visibles (option Bottom)
- Liens de partage (Sidebar ‣ Share Links)
- Select To Tweet fonctionnel

**Permissions :**
- Souvent soumis à connexion ou modération selon paramètres du site

---

## 2. Parcours d'Onboarding

### 2.1 Premier blog

1. **Administrateur** : Website ‣ Configuration ‣ Blogs : Blogs ‣ New
2. Saisir **Blog Name** et **Blog Subtitle** ‣ Save
3. Le menu « Blog » apparaît sur le site (première fois)
4. (Optionnel) Personnaliser la homepage du blog : aller sur la page du blog ‣ Edit ‣ Customize

### 2.2 Premier article

1. **Rédacteur** : Aller sur le site ‣ + New ‣ Blog Post
2. Popup : **Sélectionner le blog** ‣ Saisir **Title** ‣ Save
3. Rédiger le contenu (blocs, texte, images)
4. **Publier** : basculer Unpublished → Published
5. (Optionnel) Ajouter des tags (Edit ‣ Customize ‣ couverture ‣ Tags)

### 2.3 Tags et catégories

1. **Administrateur** : Website ‣ Configuration ‣ Blogs : Tags ‣ New (Name, Category, Used in)
2. Ou depuis un article : Edit ‣ Customize ‣ Tags ‣ Choose a record… ‣ créer ou sélectionner
3. Catégories : Website ‣ Configuration ‣ Blogs : Tag Categories

---

## 3. Scénarios d'Usage Principaux

### 3.1 Créer et publier un article

- **Acteur** : Rédacteur
- **Objectif** : Publier un nouvel article sur un blog existant
- **Étapes** : + New ‣ Blog Post ‣ choix blog, titre ‣ Save ‣ rédaction ‣ personnalisation ‣ Publier
- **Critère de succès** : Article visible sur le site, listé sur la homepage du blog

### 3.2 Filtrer les articles par tag

- **Acteur** : Visiteur
- **Objectif** : Voir tous les articles d’un tag
- **Étapes** : Aller sur le blog ‣ Sidebar ‣ Tags List ‣ clic sur un tag
- **Critère de succès** : Liste filtrée affichée

### 3.3 Filtrer par mois (Archives)

- **Acteur** : Visiteur
- **Objectif** : Voir les articles d’un mois donné
- **Étapes** : Blog ‣ Sidebar ‣ Archives ‣ sélectionner un mois
- **Critère de succès** : Liste filtrée par mois

### 3.4 Personnaliser la homepage du blog

- **Acteur** : Administrateur / Éditeur
- **Objectif** : Changer bannière, layout, sidebar, affichage des articles
- **Étapes** : Ouvrir une homepage de blog ‣ Edit ‣ Customize ‣ modifier Top Banner, Layout, Sidebar, Posts List
- **Critère de succès** : Paramètres appliqués à toutes les homepages de blog

### 3.5 Ajouter un tag à un article

- **Acteur** : Rédacteur
- **Objectif** : Associer ou créer un tag
- **Étapes** : Ouvrir l’article ‣ Edit ‣ Customize ‣ sélectionner la couverture ‣ Tags ‣ Choose a record… ‣ sélectionner ou créer
- **Critère de succès** : Tag affiché sur l’article et utilisable pour le filtrage

---

## 4. Points de Friction Identifiés

| Friction | Description | Recommandation Miyukini |
|---------|-------------|---------------------------|
| Oubli de publication | L’article reste invisible si le toggle « Unpublished » n’est pas basculé | Rappel explicite (message, checklist), ou publication par défaut avec option « Brouillon » |
| Paramètres globaux | Customize s’applique à *tous* les blogs / *tous* les articles | Permettre des surcharges par blog ou par article si besoin |
| Tags créés à deux endroits | Configuration centrale vs depuis l’article | Garder les deux chemins ; assurer cohérence et découverte (suggestions, autocomplete) |
| Contenu HTML vs blocs | Le corps est HTML ; l’édition frontend passe par le website builder | Clarifier le modèle de contenu (blocs uniquement vs HTML + blocs) pour Miyukini |
| Premier blog = menu Blog | Comportement automatique ; pas de choix d’emplacement du menu | Exposer la gestion du menu (Website Builder) pour positionnement et libellé |

---

## 5. Recommandations pour Miyukini

- **Service Blog** : Opérateurs dédiés (BlogContainerOperator, BlogPostOperator, BlogTagOperator) en équipe avec Website (pages dynamiques, menus).
- **Publication** : Gouvernance par StrongFather/Ever Buddy (cycle de vie brouillon → publié) ; rappel ou workflow explicite pour éviter les oublis.
- **Tags** : Modèle Tag + TagCategory ; filtrage et archives côté lecture ; création depuis config et depuis l’article avec synchronisation.
- **Personnalisation** : Options par défaut globales + surcharge possible par blog/article si le produit le prévoit.
- **Visiteur** : Façade Publique Gouvernée ; commentaires et partage sous Mandat Public d’Accès et modération (TAMR / WorrySentinel si nécessaire).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
