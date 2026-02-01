# Odoo Blog — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Blog** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Blog
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores et avec l'Équipe Website

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **BlogContainerOperator** | Gestion des blogs (création, édition, configuration : nom, sous-titre) | Opérateur de Service |
| **BlogPostOperator** | Gestion des articles (création, édition, publication, contenu, couverture, tags) | Opérateur de Service |
| **BlogTagOperator** | Gestion des tags et catégories de tags (création, édition, association aux articles) | Opérateur de Service |
| **BlogUI** | Interface utilisateur (homepage blog, page article, Customize, création/édition frontend) | Opérateur d'Interface |

**Équipe Website (existante) :** Blog s’appuie sur WebsitePageOperator, WebsiteMenuOperator, WebsiteBlockOperator pour les pages dynamiques, menus et blocs. BlogUI et les Opérateurs Blog collaborent avec l’Équipe Website via BondingBrother.

### 1.2 Équipe d'Opérateurs : BlogService

**Définition :**
> **BlogService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de blog (blogs, articles, tags, publication, personnalisation).**

**Composition :**
- BlogContainerOperator (niveau sécurité 2)
- BlogPostOperator (niveau sécurité 2)
- BlogTagOperator (niveau sécurité 1)
- BlogUI (niveau sécurité 1)

**Contrat d'équipe :**
- BlogContainerOperator, BlogPostOperator, BlogTagOperator collaborent sous Mandat de Permission émis par StrongFather.
- BlogUI consomme BlogPostOperator, BlogContainerOperator, BlogTagOperator et WebsiteBlockOperator (pour le rendu des blocs de contenu).
- Flux autorisés : blog.create/update, post.create/update/publish, tag.create/update, lecture publique (Façade Publique Gouvernée).

---

## 2. Opérateurs Détaillés

### 2.1 BlogContainerOperator

**Rôle :** Gestion des blogs (conteneurs d’articles) — création, édition, nom, sous-titre.

**Capacités :**
- Création / modification de blogs (nom, sous-titre)
- Association au site (website_id équivalent)
- Pas de suppression documentée obligatoire ; règles de suppression à définir (articles orphelins, etc.)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création / modification blog
- **KindMother** : Persistance des blogs (WriteIntent)
- **Master Butler** : Permissions création / édition
- **WorrySentinel** : Niveau sécurité

**Contrat d'équipe :**
- Consomme : WebsiteMenuOperator (ajout de l’item « Blog » au menu à la première création)
- Expose : `blog.create`, `blog.update`, `blog.read`

**Mandat de Permission requis :**
- Création / modification blog : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.2 BlogPostOperator

**Rôle :** Gestion des articles (création, édition, publication, contenu, couverture, tags).

**Capacités :**
- Création / modification d’articles (titre, contenu, couverture, blog, tags)
- Publication / dépublication (toggle Published)
- Gestion du contenu (HTML / blocs via WebsiteBlockOperator pour le rendu)
- Association auteur, dates

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision publication, création, modification
- **KindMother** : Persistance des articles (WriteIntent)
- **Master Butler** : Permissions création / édition / publication
- **WorrySentinel** : Niveau sécurité, visibilité (public par défaut)
- **Ever Buddy** : Cycle de vie (brouillon → publié)

**Contrat d'équipe :**
- Consomme : BlogContainerOperator (blog_id), BlogTagOperator (tags), WebsiteBlockOperator (contenu blocs)
- Expose : `post.create`, `post.update`, `post.publish`, `post.unpublish`, `post.read`

**Mandat de Permission requis :**
- Création / modification article : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Publication : Mandat avec StrongFather (décision) + KindMother (WriteIntent)

### 2.3 BlogTagOperator

**Rôle :** Gestion des tags et catégories de tags (création, édition, association aux articles).

**Capacités :**
- Création / modification de tags (nom, catégorie)
- Création / modification de catégories de tags
- Association tags ↔ articles (many2many)
- Filtrage côté lecture (liste par tag, par archive/mois)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **StrongFather** : Décision création / modification tag ou catégorie
- **KindMother** : Persistance des tags et catégories (WriteIntent)
- **Master Butler** : Permissions création / édition

**Contrat d'équipe :**
- Consomme : Aucun autre Opérateur métier
- Expose : `tag.create`, `tag.update`, `tag_category.create`, `tag_category.update`, `tag.read`, `tag.associate_posts`

**Mandat de Permission requis :**
- Création / modification tag ou catégorie : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.4 BlogUI

**Rôle :** Interface utilisateur (homepage blog, page article, Customize, création/édition frontend).

**Capacités :**
- Affichage homepage blog (liste articles, filtres Archives/Tags, sidebar)
- Affichage page article (contenu, sidebar, breadcrumb, next article, commentaires, partage)
- Mode Edit et Customize (options Top Banner, Layout, Sidebar, Posts List, Bottom, etc.)
- Création / édition d’article en frontend (+ New ‣ Blog Post ; choix blog, titre, contenu)
- Publication (toggle) et rappel si non publié

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions affichage, édition (selon rôle)
- **WorrySentinel** : Visibilité (Façade Publique Gouvernée pour visiteurs)
- **BondingBrother** : Traduction des intentions vers BlogPostOperator, BlogContainerOperator, BlogTagOperator, WebsiteBlockOperator

**Contrat d'équipe :**
- Consomme : BlogPostOperator, BlogContainerOperator, BlogTagOperator, WebsitePageOperator (pages dynamiques), WebsiteMenuOperator (menu Blog), WebsiteBlockOperator (blocs contenu)
- Expose : Écrans et actions UI (lecture, création, édition, publication, personnalisation)

**Mandat de Permission requis :**
- Édition / création / publication : Mandat couvrant BlogPostOperator, BlogContainerOperator, BlogTagOperator selon action
- Lecture publique : Mandat Public d’Accès (utilisateurs externes) ou Visa (utilisateurs visiteurs)

---

## 3. Contrat d'Équipe BlogService

**Opérateurs membres :** BlogContainerOperator, BlogPostOperator, BlogTagOperator, BlogUI

**Flux autorisés :**
- BlogUI → BlogPostOperator : create, update, publish, unpublish, read
- BlogUI → BlogContainerOperator : create, update, read
- BlogUI → BlogTagOperator : create, update, read, associate_posts
- BlogPostOperator → BlogContainerOperator : read (blog_id)
- BlogPostOperator → BlogTagOperator : read, associate (tags)
- BlogContainerOperator → WebsiteMenuOperator : ajout item Blog (première création)
- BlogUI → WebsitePageOperator / WebsiteBlockOperator : pages dynamiques, rendu blocs

**Types d'échanges :** Données blog, post, tag (identifiants, métadonnées, contenu) ; pas de données sensibles hors métadonnées de publication.

**Conditions préalables :** Mandat de Permission valide émis par StrongFather pour les flux d’écriture ; Mandat Public d’Accès ou Visa pour la lecture publique.

**Niveau de validation requis :** StrongFather pour toute décision de création, modification, publication ; KindMother pour toute persistance (WriteIntent).

---

## 4. Intégration avec les Cores

| Core | Rôle pour Blog |
|------|----------------|
| **StrongFather** | Décision création/modification blog, article, tag ; décision publication |
| **KindMother** | Persistance blog, post, tag, tag_category (WriteIntent) |
| **Master Butler** | Permissions création, édition, publication ; déclaration des capacités Blog |
| **WorrySentinel** | Niveau sécurité ; visibilité (public) ; modération commentaires si applicable |
| **Ever Buddy** | Cycle de vie article (brouillon → publié) ; versions/dépréciation si prévu |
| **BondingBrother** | Traduction des intentions UI vers BlogService et WebsiteService |
| **TAMR** | Intervention humaine (modération commentaires, validation publication si politique stricte) |

---

## 5. Correspondance Odoo → Miyukini

| Odoo | Miyukini |
|------|----------|
| blog.blog | BlogContainerOperator + modèle Blog |
| blog.post | BlogPostOperator + modèle Post |
| blog.tag / blog.tag.category | BlogTagOperator + modèles Tag, TagCategory |
| website_blog (pages, controllers) | BlogUI + WebsitePageOperator (pages dynamiques) |
| Edit ‣ Customize | BlogUI (options) + WebsiteBlockOperator (blocs) |
| Menu Blog | WebsiteMenuOperator (item ajouté par BlogContainerOperator à la première création) |
| Publication (toggle) | BlogPostOperator (publish/unpublish) sous StrongFather + KindMother |

---

## 6. Service Miyukini Proposé

**Nom :** `MiyukiniBlog` ou `MiyuBlog`

**Opérateurs :**
- **BlogContainerOperator** : Gestion des blogs (conteneurs)
- **BlogPostOperator** : Gestion des articles (création, publication, contenu)
- **BlogTagOperator** : Gestion des tags et catégories
- **BlogUI** : Interface (homepage, article, Customize, création/édition frontend)

**Équipe d'Opérateurs :** BlogService

**Dépendance :** Équipe Website (WebsitePageOperator, WebsiteMenuOperator, WebsiteBlockOperator) pour pages dynamiques, menu et blocs de contenu.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
