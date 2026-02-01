# Odoo Blog — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Blog** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0, module website_blog

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo (website, mail, portal, etc.)
- Flux de données inter-apps (pages dynamiques, commentaires, partage)
- Mécanismes d'intégration (controllers, vues QWeb, modèles)
- APIs et hooks typiques
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (typiques du module `website_blog`)

**Dépendances explicites (structure classique Odoo) :**
- **website** : Pages dynamiques, éditeur, layout, menus, building blocks
- **base** : Modèles de base, res.company, res.users, res.partner
- **web** : Framework web, assets, frontend
- **mail** (souvent) : Chatter, commentaires sur les articles (blog.post)
- **portal** (optionnel) : Utilisateurs portail (visiteurs connectés pour commenter)

**Module cœur :**
- **website_blog** : Modèles blog.blog, blog.post, blog.tag, blog.tag.category ; controllers ; vues QWeb ; intégration website (pages /blog, blocs, Edit ‣ Customize)

### 1.2 Modules optionnels (intégrations si installés)

**Apps qui étendent ou consomment le Blog :**
- **website** : Obligatoire — pages dynamiques, éditeur, menus, blocs
- **website_sale** (eCommerce) : Blocs produits possibles sur des pages ; pas de lien direct blog obligatoire
- **website_event** : Événements ; pas de lien direct blog obligatoire
- **website_slides** (eLearning) : Slides ; pas de lien direct blog obligatoire
- **mass_mailing** (Email Marketing) : Newsletter ; possibilité de liens vers articles de blog
- **social_*** : Partage réseaux sociaux (Follow Us, Share Links, Select To Tweet) — configuration via bloc Social Media sur le site
- **Plausible / Analytics** : Suivi du trafic du blog (documentation Odoo : Plausible pour analytics)
- **Unsplash** : Intégration images libres de droit pour illustrer les articles (documentation Odoo : intégrations Unsplash)

---

## 2. Flux de Données Inter-Apps

### 2.1 Website → Blog

**Flux :**
- **Menu** : website.menu peut pointer vers /blog ; l’item « Blog » est ajouté au menu du site à la première création d’un blog (website_blog)
- **Pages dynamiques** : Routes /blog, /blog/[blog_slug], /blog/[blog_slug]/post/[post_slug] gérées par website_blog ; rendu QWeb avec données blog.blog, blog.post
- **Éditeur** : Création et édition d’articles en frontend via le website builder (blocs, drag-and-drop) ; le contenu est stocké dans blog.post (champ HTML / structure)
- **Customize** : Options de personnalisation (Top Banner, Layout, Sidebar, etc.) partagées avec le website builder (Edit ‣ Customize)

**Données échangées :**
- blog.blog, blog.post, blog.tag : lus par les controllers website_blog pour le rendu
- website_id : association blog ↔ site (multi-website)
- Menus : website.menu lié aux URLs blog

### 2.2 Blog → Mail (commentaires)

**Flux :**
- **Commentaires** : Si le module mail est utilisé, les articles (blog.post) peuvent avoir un chatter / fil de commentaires
- **Affichage** : Option « Comments » en bas de l’article (Customize ‣ Bottom)
- **Données** : mail.message, mail.followers ; modération selon paramètres Odoo

**Règles :**
- Commentaires activés/désactivés par option Customize
- Visiteurs : selon configuration (anonyme, connecté, modération)

### 2.3 Blog → Portal / Auth

**Flux :**
- **Visiteurs connectés** : Si portal ou auth_signup est utilisé, les visiteurs peuvent se connecter ; accès éventuel à des contenus réservés (si visibilité « Signed In » sur des pages)
- **Commentaires** : Connexion possible pour commenter (selon configuration)
- **Partage** : Share Links, Select To Tweet — pas d’obligation de compte

### 2.4 Intégrations externes (documentation Odoo)

- **Unsplash** : Illustrer les articles avec des images libres de droit
- **Plausible** : Analytics sur le trafic du blog (Website ‣ Reporting ‣ Analytics)
- **Réseaux sociaux** : Follow Us, Share Links — configurés via le bloc Social Media ailleurs sur le site

---

## 3. Mécanismes d'Intégration

### 3.1 Controllers

- **Routing** : Routes HTTP avec `@http.route(..., website=True)` pour /blog, /blog/<blog>, /blog/<blog>/post/<post>
- **Rendu** : `request.render('website_blog.template_name', values)` ; `values` contient blog, post, tags, archives, etc.
- **Soumission** : Commentaires (si mail) ; pas de formulaire métier type « Create Opportunity » sur le blog lui-même (sauf si formulaire générique website sur une page blog)

### 3.2 Vues QWeb

- **Héritage** : `inherit_id="website.layout"` pour les pages blog (header, footer communs)
- **Templates** : website_blog fournit les templates pour homepage blog, page article, blocs (liste articles, dernier article, etc.)
- **Snippets / blocs** : Blocs spécifiques blog enregistrés dans website_blog (utilisables sur d’autres pages du site si exposés)

### 3.3 Modèles

- **blog.blog** : website_id (lien vers website) ; name, subtitle
- **blog.post** : blog_id, name, content (HTML), cover, author_id, tag_ids, website_published ; héritage possible de mail.thread pour chatter
- **blog.tag** : name, category_id (blog.tag.category) ; relation many2many avec blog.post
- **blog.tag.category** : Regroupement des tags pour la sidebar

### 3.4 Configuration

- **Website ‣ Configuration ‣ Blogs** : Blogs, Tags, Tag Categories
- Pas de res.config.settings dédié « Blog » documenté dans la doc utilisateur 19.0 ; paramètres de personnalisation via Customize (appliqués à tous les blogs / tous les articles)

---

## 4. APIs et Hooks Typiques

- **Controllers** : Routes /blog, /blog/<int:blog_id>, /blog/<int:blog_id>/post/<int:post_id> (ou slugs selon version)
- **Modèles** : Méthodes de recherche (posts publiés, par tag, par mois) ; compteurs (commentaires, vues) si exposés
- **Website builder** : Blocs blog (liste d’articles, dernier article) — enregistrement dans website_blog ; insertion dans les pages via Edit
- **Menu** : Création automatique de l’item « Blog » à la première création d’un blog (hook ou override website.menu)

---

## 5. Synthèse des Dépendances

| App | Relation avec Blog |
|-----|---------------------|
| **website** | Obligatoire — pages dynamiques, éditeur, menus, Customize |
| **mail** | Optionnel — commentaires sur les articles |
| **portal** | Optionnel — visiteurs connectés, commentaires |
| **auth_signup** | Optionnel — inscription depuis le site |
| **website_sale** | Optionnel — pas de lien direct ; blocs produits sur d’autres pages |
| **mass_mailing** | Optionnel — liens vers articles dans newsletters |
| **Unsplash** | Optionnel — images pour articles |
| **Plausible / Analytics** | Optionnel — trafic du blog |

---

## 6. Recommandations pour Miyukini

- **Blog** : Dépendre de l’Opérateur / Équipe Website (pages dynamiques, menus, éditeur) ; ne pas dupliquer la logique de pages et de menus.
- **Commentaires** : Traiter comme un flux métier distinct (MiyuForum ou module dédié) ; gouvernance par StrongFather/TAMR pour modération ; Mandat Public d’Accès pour visiteurs.
- **Partage et analytics** : Façade Publique Gouvernée ; pas d’exposition de données sensibles ; intégrations externes (réseaux sociaux, analytics) sous contrôle WorrySentinel.
- **Tags et catégories** : Modèle Tag + TagCategory réutilisable ; filtrage et archives côté lecture ; création depuis config et depuis l’article.
- **Multi-site** : Si Miyukini gère plusieurs sites (website_id équivalent), associer chaque blog à un site et filtrer les menus et URLs en conséquence.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
