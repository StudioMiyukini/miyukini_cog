# Odoo Website Builder — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Website Builder** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module Website

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Administrateur Site / Éditeur (Website Administrator)

**Profil :**
- Rôle : Gestion globale du site et du contenu
- Responsabilités :
  - Créer et organiser les pages
  - Gérer les menus (header, footer)
  - Publier / dépublier des pages
  - Définir la homepage
  - Configurer les propriétés (URL, visibilité, SEO)
  - Utiliser l’éditeur visuel (building blocks)

**Besoins :**
- Accès backend (Website ‣ Site ‣ Pages / Menus / Properties)
- Accès frontend en mode « Edit » pour édition WYSIWYG
- Gestion des redirections (mode développeur)
- Duplication et suppression de pages avec gestion des liens

**Permissions :**
- Droits d’édition sur le module Website
- Accès aux paramètres du site (multi-website si applicable)

### 1.2 Créateur de Contenu / Rédacteur (Content Editor)

**Profil :**
- Rôle : Création et mise à jour du contenu des pages
- Responsabilités :
  - Ajouter et déplacer des blocs (drag & drop)
  - Rédiger textes, titres, descriptions
  - Insérer images, vidéos, liens
  - Configurer formulaires (champs, actions)
  - Enregistrer des blocs personnalisés
  - Créer des ancres et liens internes

**Besoins :**
- Éditeur visuel simple (Edit sur le frontend)
- Panneau Customize (onglet par bloc)
- Accès aux médias (bibliothèque ou upload)
- Pas obligatoirement l’accès aux propriétés avancées (URL, SEO, visibilité)

**Permissions :**
- Droits d’édition website (éventuellement restreints à certaines pages/sections)

### 1.3 Visiteur (Public / Utilisateur externe)

**Profil :**
- Rôle : Consultation du site
- Responsabilités : Aucune ; navigation, lecture, soumission de formulaires (contact, newsletter, etc.)

**Besoins :**
- Pages publiées et accessibles (selon visibilité)
- Navigation claire (menus)
- Formulaires fonctionnels et message de succès ou redirection
- Bonne performance et responsive

**Permissions :**
- Aucun droit d’édition
- Accès selon règles de visibilité (Public / Signed In / Restricted Group / With Password)

### 1.4 Utilisateur Connecté (Signed-in User)

**Profil :**
- Rôle : Visiteur authentifié
- Responsabilités : Accéder aux pages « Signed In » ou « Restricted Group » si autorisé

**Besoins :**
- Connexion (portail / auth)
- Accès aux contenus réservés aux connectés ou à certains groupes

**Permissions :**
- Droits définis par le groupe d’accès Odoo et par les paramètres de visibilité des pages

### 1.5 Développeur / Intégrateur (Developer)

**Profil :**
- Rôle : Thèmes, redirections, intégrations techniques
- Responsabilités :
  - Activer le mode développeur
  - Créer des redirections (Website ‣ Configuration ‣ Redirects)
  - Adapter thèmes ou vues QWeb
  - Intégrer formulaires en iframe sur site externe

**Besoins :**
- Accès technique (redirects, assets, héritage de vues)
- Documentation QWeb / controllers

**Permissions :**
- Droits techniques / administrateur

---

## 2. Parcours d'Onboarding

### 2.1 Premier déploiement du site

1. **Installation** du module Website (ou suite Websites).
2. **Accès** à l’app Website : choix entre frontend (voir le site) et backend (Pages, Menus, Properties).
3. **Homepage** : par défaut une page d’accueil type ; configuration dans Website ‣ Site ‣ Properties ‣ Publish ‣ Use as Homepage.
4. **Première page** : + New ‣ Page ‣ choix du template ‣ titre ‣ Create ‣ édition ‣ Save ‣ Publish.
5. **Menu** : la page peut être « In Menu » ou non ; l’ordre et la structure se gèrent dans l’éditeur de menu (header/footer).

### 2.2 Prise en main de l’éditeur

1. Ouvrir le site en frontend.
2. Cliquer sur **Edit** (si droits).
3. Découvrir les **building blocks** : onglet Blocks, glisser-déposer Catégories puis Inner Content.
4. **Customize** : cliquer sur un bloc ‣ onglet Customize ‣ modifier texte, fond, layout (Grid/Cols), etc.
5. **Sauvegarder** : Save.
6. **Propriétés** : Site ‣ Properties pour URL, menu, publication, visibilité.

### 2.3 Première publication planifiée

1. Créer ou éditer une page.
2. Site ‣ Properties ‣ **Publishing Date** : choisir date/heure.
3. **Published** : activer ou laisser pour la date planifiée (selon implémentation Odoo).
4. Valider.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Créer une page « À propos »

1. Website ‣ + New ‣ Page.
2. Choisir template **About** (ou Basic).
3. Saisir titre (ex. « À propos »), Create.
4. En mode Edit : adapter textes, images, blocs.
5. Site ‣ Properties : activer « In Menu », définir URL si besoin.
6. Publish.

### 3.2 Mettre en place un formulaire de contact avec création d’opportunité CRM

1. Éditer une page (ou en créer une).
2. Glisser-déposer **Contact & Forms** ‣ **Form**.
3. Customize ‣ Form ‣ **Action** : Create an Opportunity (CRM).
4. Ajouter/modifier les champs (Customize ‣ + Field).
5. On Success : redirection ou message.
6. Save ‣ Publish.

### 3.3 Changer la homepage

1. Créer ou choisir la page cible.
2. Website ‣ Site ‣ **Properties** ‣ onglet Publish.
3. **Use as Homepage** : activer pour cette page.
4. Sauvegarder.

### 3.4 Gérer une redirection après suppression de page

1. Avant suppression : noter l’ancienne URL.
2. Website ‣ Site ‣ Pages ‣ ouvrir la page ‣ Properties ‣ **Delete Page**.
3. Consulter les liens référents, les corriger ou définir une redirection.
4. Mode développeur ‣ Website ‣ Configuration ‣ **Redirects** ‣ New.
5. URL from : ancienne URL ; URL to : nouvelle page ou 404 ; Action : 301 ou 302.
6. Valider, cocher « I am sure » dans le popup de suppression si demandé.

### 3.5 Rendre une page réservée aux connectés

1. Ouvrir la page en édition (ou Properties).
2. Site ‣ Properties ‣ **Visibility** : **Signed In** (ou Restricted Group / With Password).
3. Save. La page n’est plus accessible en anonyme.

---

## 4. Points de Friction Identifiés

| Friction | Description | Impact |
|----------|-------------|--------|
| Multi-entrées | Édition possible depuis frontend (Edit) et backend (Pages) ; risque de confusion | Utilisateurs ne savent pas où modifier |
| Propriétés dispersées | URL, menu, publication, SEO, visibilité dans « Properties » ; pas toujours visible | Découverte difficile |
| Redirections en mode dev | Redirects accessibles uniquement avec mode développeur | Dépendance technique pour les redirections |
| Suppression de page | Vérification des liens manuelle, risque de liens cassés | Charge cognitive, erreurs possibles |
| Formulaires et apps | Actions (CRM, Helpdesk, etc.) dépendent des apps installées | Comportement variable selon la suite |
| Building blocks et dépendances | Certains blocs (ex. Products) nécessitent eCommerce | Attentes non satisfaites si module absent |
| Responsive | Options colonnes/mobile dans Customize ; pas toujours évident | Résultat mobile parfois sous-optimal |

---

## 5. Recommandations pour Miyukini

- **Unifier les parcours** : distinguer clairement « Écran de conception » (structure, propriétés) et « Écran de livraison » (page publiée), tout en gardant un accès unique cohérent (cf. règle « Une page sert à livrer, un écran sert à concevoir »).
- **Opérateur d’interface dédié** : MiyuWeb ou MiyukiniWeb comme Opérateur d’Interface pour l’édition ; séparation nette entre édition (Mandat, permissions) et consultation (Façade Publique Gouvernée pour visiteurs).
- **Formulaires** : modéliser les actions (email, CRM, Helpdesk, etc.) comme flux gouvernés (Mandats, Contrats d’équipe) entre MiyuWeb et MiyuContacts, MiyuCRM, MiyuForum, etc.
- **Visibilité et sécurité** : aligner Public / Signed In / Restricted / Password sur les niveaux de sécurité et Mandats Public d’Accès (utilisateurs externes) et Visa/Passeport (utilisateurs visiteurs).
- **SEO et redirections** : traiter comme données de configuration gouvernées (KindMother, StrongFather) avec traçabilité (Ever Buddy).
- **Onboarding** : parcours guidé (tours ou documentation ciblée) pour « Première page », « Premier formulaire », « Publication » et « Redirections » afin de réduire les frictions.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
