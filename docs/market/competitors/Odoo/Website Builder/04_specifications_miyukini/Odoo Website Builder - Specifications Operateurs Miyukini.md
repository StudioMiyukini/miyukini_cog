# Odoo Website Builder — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Website Builder** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Website Builder
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **WebsitePageOperator** | Gestion des pages (création, édition, publication, propriétés) | Opérateur de Service |
| **WebsiteMenuOperator** | Gestion des menus (header, footer, structure navigation) | Opérateur de Service |
| **WebsiteBlockOperator** | Catalogue et rendu des building blocks (blocs, templates) | Opérateur de Service |
| **WebsiteRedirectOperator** | Gestion des redirections URL (301, 302, 308, 404) | Opérateur de Service |
| **WebsiteFormOperator** | Traitement des formulaires (champs, actions, envoi vers Opérateurs métier) | Opérateur de Service |
| **WebsiteUI** | Interface utilisateur (éditeur visuel, propriétés, menus) | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : WebsiteService

**Définition :**
> **WebsiteService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de site web (pages, menus, blocs, formulaires, redirections).**

**Composition :**
- WebsitePageOperator (niveau sécurité 2)
- WebsiteMenuOperator (niveau sécurité 2)
- WebsiteBlockOperator (niveau sécurité 1)
- WebsiteRedirectOperator (niveau sécurité 2)
- WebsiteFormOperator (niveau sécurité 2)
- WebsiteUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 WebsitePageOperator

**Rôle :** Gestion des pages (création, édition, publication, propriétés, duplication, suppression).

**Capacités :**
- Création / modification de pages (titre, URL, contenu/structure)
- Gestion publication / dépublication et date de publication planifiée
- Gestion des propriétés (In Menu, Is Homepage, Indexed, Visibility)
- Duplication de page
- Suppression avec gestion des liens et redirections
- Gestion multi-site (website_id équivalent)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision publication, changement visibilité, suppression
- **KindMother** : Persistance des pages (WriteIntent)
- **Master Butler** : Permissions création / édition / publication
- **WorrySentinel** : Niveau sécurité, visibilité (Public / Signed In / Restricted / Password)
- **Ever Buddy** : Cycle de vie (brouillon → publié → archivé)

**Contrat d'équipe :**
- Consomme : WebsiteMenuOperator (menus), WebsiteBlockOperator (blocs), WebsiteRedirectOperator (redirections)
- Expose : `page.create`, `page.update`, `page.publish`, `page.unpublish`, `page.duplicate`, `page.delete`

**Mandat de Permission requis :**
- Création / modification page : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Publication : Mandat avec StrongFather (décision) + KindMother (WriteIntent)
- Suppression : Mandat avec StrongFather (décision) + WebsiteRedirectOperator (redirections si besoin)

### 2.2 WebsiteMenuOperator

**Rôle :** Gestion des menus (header, footer, hiérarchie, ordre).

**Capacités :**
- Création / modification d’items de menu
- Liaison page ou URL externe
- Hiérarchie (parent / enfant) et sequence
- Visibilité (header / footer / autre)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création / modification menu
- **KindMother** : Persistance des menus (WriteIntent)
- **Master Butler** : Permissions édition menus
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consomme : WebsitePageOperator (pages liées)
- Expose : `menu.create`, `menu.update`, `menu.reorder`, `menu.delete`

**Mandat de Permission requis :**
- Création / modification menu : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.3 WebsiteBlockOperator

**Rôle :** Catalogue et rendu des building blocks (catégories, inner content, blocs personnalisés).

**Capacités :**
- Déclaration des blocs disponibles (catégories, inner content)
- Rendu des blocs (structure, pas de logique métier)
- Enregistrement de blocs personnalisés (Custom)
- Pas d’exécution métier : formulaires délégués à WebsiteFormOperator

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Catalogue des blocs (Capabilities)
- **KindMother** : Persistance des définitions de blocs personnalisés (WriteIntent)
- **StrongFather** : Décision sauvegarde bloc personnalisé

**Contrat d'équipe :**
- Consommé par : WebsitePageOperator, WebsiteUI
- Expose : `block.list`, `block.render`, `block.save_custom`

**Mandat de Permission requis :**
- Sauvegarde bloc personnalisé : Mandat avec KindMother (WriteIntent)

### 2.4 WebsiteRedirectOperator

**Rôle :** Gestion des redirections URL (301, 302, 308, 404).

**Capacités :**
- Création / modification / désactivation de redirections
- URL from / URL to, type d’action, site (multi-website), sequence
- Pas d’exécution HTTP : fournit les règles ; le composant de routage les applique

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création / modification redirection
- **KindMother** : Persistance des redirections (WriteIntent)
- **Master Butler** : Permissions édition redirections
- **Ever Buddy** : Traçabilité (ancienne URL → nouvelle)

**Contrat d'équipe :**
- Consommé par : WebsitePageOperator (lors de suppression ou changement d’URL)
- Expose : `redirect.create`, `redirect.update`, `redirect.list`

**Mandat de Permission requis :**
- Création / modification redirection : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.5 WebsiteFormOperator

**Rôle :** Traitement des formulaires (validation, routage vers Opérateurs métier).

**Capacités :**
- Réception des soumissions de formulaires (champs, action choisie)
- Validation des champs (types, requis)
- Délégation à l’Opérateur métier concerné (MiyuContacts, MiyuCRM, MiyuForum/Helpdesk, MiyuProject, MiyuNotify, etc.) via BondingBrother
- Gestion « On Success » : redirection, message, rien
- Pas de persistance directe : délégation sous Mandat

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision d’autoriser la soumission (contexte, visiteur)
- **Master Butler** : Permissions formulaire (action autorisée)
- **WorrySentinel** : Niveau sécurité (données sensibles)
- **BondingBrother** : Traduction intention → appel Opérateur métier

**Contrat d'équipe :**
- Consomme : MiyuContacts (Create a Customer), MiyuCRM (Create an Opportunity), MiyuForum/Helpdesk (Create a Ticket), MiyuHR/Recruitment (Apply for a Job), MiyuProject (Create a Task), MiyuNotify (Newsletter, email)
- Expose : `form.submit`, `form.validate`

**Mandat de Permission requis :**
- Soumission formulaire : Mandat avec StrongFather (décision) + Mandat vers l’Opérateur métier cible (création enregistrement)

### 2.6 WebsiteUI

**Rôle :** Interface utilisateur (éditeur visuel, propriétés de page, menus, building blocks).

**Capacités :**
- Affichage du site en mode consultation (Façade Publique Gouvernée pour visiteurs)
- Mode édition : barre d’outils, panneau Blocks, panneau Customize
- Écran propriétés de page (URL, menu, publication, visibilité, SEO)
- Édition des menus (header, footer)
- Pas d’autorité : toute action passe par BondingBrother vers les Opérateurs du WebsiteService

**Niveau de sécurité :** 1 (Standard) pour l’édition ; visibilité des contenus selon règles de chaque page (Public / Signed In / Restricted / Password)

**Gouvernance :**
- **Master Butler** : Permissions d’édition (qui peut Edit)
- **WorrySentinel** : Contrôle accès aux contenus restreints (Signed In, Restricted Group, Password)
- **BondingBrother** : Traduction des intentions (créer page, publier, modifier menu, etc.)

**Contrat d'équipe :**
- Consommé par : Utilisateur (éditeur) ou Visiteur (consultation)
- Consomme : WebsitePageOperator, WebsiteMenuOperator, WebsiteBlockOperator, WebsiteFormOperator (affichage formulaires)

**Mandat de Permission requis :**
- Édition : Mandat avec WebsitePageOperator / WebsiteMenuOperator / WebsiteBlockOperator selon l’action
- Consultation : Mandat Public d’Accès (utilisateurs externes) ou Visa (utilisateurs visiteurs) selon visibilité de la page

---

## 3. Contrat d'Équipe : WebsiteService

**Flux autorisés (résumé) :**
- WebsiteUI → BondingBrother → WebsitePageOperator, WebsiteMenuOperator, WebsiteBlockOperator, WebsiteFormOperator, WebsiteRedirectOperator
- WebsitePageOperator → WebsiteMenuOperator (lien page-menu), WebsiteRedirectOperator (création redirection)
- WebsiteFormOperator → BondingBrother → MiyuContacts, MiyuCRM, MiyuForum, MiyuProject, MiyuNotify, etc.

**Types d’échanges :**
- Intentions (créer page, publier, soumettre formulaire) ; WriteIntent (KindMother) ; réponses (succès, erreur, redirection).

**Niveau de sécurité maximum de l’équipe :** 2 (Sensitive) ; WebsiteUI et WebsiteBlockOperator en 1 pour l’affichage et le catalogue.

---

## 4. Niveaux de Sécurité par Rôle

| Opérateur | Niveau | Justification |
|-----------|--------|----------------|
| WebsitePageOperator | 2 | Données de contenu et publication |
| WebsiteMenuOperator | 2 | Structure navigation |
| WebsiteBlockOperator | 1 | Catalogue et rendu non sensible |
| WebsiteRedirectOperator | 2 | Impact SEO et URLs |
| WebsiteFormOperator | 2 | Données formulaires (contact, CRM, etc.) |
| WebsiteUI | 1 | Interface ; accès contenu gouverné par visibilité page |

---

## 5. Correspondance Odoo → Miyukini

| Odoo | Miyukini |
|------|----------|
| website.page | WebsitePageOperator + KindMother (WriteIntent) |
| website.menu | WebsiteMenuOperator + KindMother (WriteIntent) |
| Building blocks / QWeb | WebsiteBlockOperator + Master Butler (catalogue) |
| website.redirect | WebsiteRedirectOperator + KindMother (WriteIntent) |
| Formulaires (actions CRM, Helpdesk, etc.) | WebsiteFormOperator + Contrats d’équipe avec MiyuCRM, MiyuForum, etc. |
| Éditeur visuel / Frontend | WebsiteUI |
| Visiteur (Public) | Utilisateur Externe (Façade Publique Gouvernée, Mandat Public d’Accès) |
| Signed In / Restricted | Utilisateur Visiteur ou citoyen (Visa / Mandat) |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
