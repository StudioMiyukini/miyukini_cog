# Audit Catakana → Miyukini COG — Conversion B2B2C

## Contexte

Ce document audite le projet **Catakana** (dossier `.Catakana`) en vue d’une conversion vers l’écosystème **Miyukini COG**. L’objectif est d’offrir un **service de Gestion d’événements en B2B2C** : la plateforme propose le service à des **organisateurs**, qui l’adaptent à leurs besoins. Le modèle cible s’apparente à un **Store** avec :

- **Annuaire de tous les événements**
- **Répertoire d’organisateurs**
- **Répertoire d’exposants**

## Portée / Scope

- **Périmètre** : codebase Catakana (React/Vite/TypeScript, Supabase), comparé aux crates et toolkits Miyukini existants.
- **Livrables** : identification des Kits d’outils manquants, correspondance logiques métier → Opérateurs, besoins UI pour la conversion.
- **Références** : Glossaire Miyukini, MIP v1, nomenclature documentation.

---

## 1. Kits d’outils : ce que Catakana utilise vs ce que Miyukini possède

### 1.1 Inventaire des capacités Catakana (par domaine)

| Domaine Catakana | Capacités / « outils » | Kit Miyukini existant ? | Commentaire |
|------------------|------------------------|--------------------------|-------------|
| **Authentification** | Comptes organisateurs, rôles (bénévole, exposant, manager, admin), email/mot de passe, lien magique, protection routes | **Miyauth** (toolkit.identity.miyauth) | Couverture partielle ; rôles métier (organisateur/exposant) à brancher sur Master Butler. |
| **Exposants** | CRUD exposants, filtres, import CSV/Google Sheet, statuts, commentaires, documents | — | **Pas de kit dédié « annuaire exposants »**. MiyuContacts + MiyuProfile + MiyuStore (fiche produit) peuvent composer. |
| **Devis & factures** | Devis, conversion devis→facture, PDF, historique, envoi email, marquage payé | **Miyuinvoice** (toolkit.invoice.standalone) | Alignement fort ; à coupler avec MiyuContacts / exposant. |
| **Plan & emplacement** | Plan interactif (Fabric.js), attribution stands, zones, tailles, export visuel | — | **Pas de kit « plan / floor plan »** dans Miyukini. Logique métier spécifique événement. |
| **Programme / agenda** | Animations, scènes/salles, horaires, chevauchements bloqués, vues chrono/salle, filtres | **Miyubooking** (réservations/créneaux) partiel ; **MiyuClock** (temps) | Pas de kit « programme / schedule » dédié. Miyubooking pour créneaux, à étendre. |
| **Documents & légal** | Contrats types, envoi à signer, historique par exposant | **MiyuCMS** (contenu) + **MiyuDocuments** ? | Miyukini n’a pas de toolkit « documents contractuels / signatures » explicite. |
| **Notifications** | Annonces globales, notifications ciblées (rôle, équipe), journal, planification | **Miyunotify** (toolkit.notify.miyunotify) | Bon candidat pour annonces et notifications. |
| **Budget** | Revenus/dépenses, ventilation, stats, balance par édition | **Miyucptaledger**, **Miyuexpense**, **Miyucomptareports**, **Miyutreasury** | Couverture possible par combinaison compta/trésorerie. |
| **Éditions** | Tableau de bord par édition, infos générales, équipe, débriefing | — | **Pas de kit « édition / événement »**. Notion d’« édition » = agrégat métier à modéliser. |
| **Candidatures** | Candidatures exposants, validation, workflow | **Miyubooking** (demandes) ou logique custom | Pas de kit « candidature / workflow validation » dédié. |
| **Recherche / annuaire** | Liste exposants, filtres, recherche | **Miyusearch** (toolkit.search.miyusearch) | Réutilisable pour annuaires et Store. |
| **Média / fichiers** | Logos, photos, documents exposants | **Miyumedia** (toolkit.content.media) | Alignement direct. |
| **Export** | Export données, CSV, PDF | **Miyuexport** (toolkit.export.miyuexport) | Réutilisable. |

### 1.2 Kits d’outils à créer ou à étendre (manquants Miyukini)

| Kit à créer / étendre | Id proposé | Rôle | Dépendances Miyukini probables |
|------------------------|------------|------|--------------------------------|
| **Événement / Édition** | `toolkit.event.edition` (ex. miyuedition) | Gestion des éditions (événements) : métadonnées, dates, lieu, statut, lien organisateur. | MiyuSQL, MiyuClock, MiyuProfile |
| **Plan de salle / Emplacement** | `toolkit.event.floorplan` (ex. miyufloorplan) | Plan interactif, zones, stands, attribution exposant↔emplacement. | MiyuSQL, Miyumedia (visuels), évent. Miyuweb (canvas) |
| **Programme / Schedule** | `toolkit.event.schedule` (ex. miyuschedule) | Programme d’animations, créneaux, salles/scènes, conflits horaires. | MiyuClock, Miyubooking (créneaux), MiyuSQL |
| **Exposant / Annuaire** | `toolkit.event.exhibitor` (ex. miyuexhibitor) | Annuaire exposants, fiche, statuts, lien édition, documents. | MiyuContacts, MiyuProfile, Miyumedia, Miyusearch |
| **Candidature** | Extension **Miyubooking** ou module léger | Workflow candidature (exposant → édition), validation, statuts. | Miyubooking, StrongFather (décision), Miyunotify |

Résumé : **Miyukini ne possède pas** de toolkits dédiés **événement**, **plan de salle**, **programme/schedule**, ni **annuaire exposants**. Ces capacités sont soit à créer (nouveaux crates), soit à modéliser comme **Équipe d’Opérateurs** composée des toolkits existants (MiyuContacts, MiyuInvoice, Miyubooking, etc.).

---

## 2. Logiques métier → correspondance Opérateurs Miyukini

Dans le glossaire Miyukini, un **Opérateur** est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur. Ci-dessous la correspondance **logiques métier Catakana → Opérateurs (existants ou à créer)**.

### 2.1 Opérateurs réutilisables (déjà dans l’écosystème)

| Logique métier Catakana | Opérateur Miyukini | Crate / toolkit | Notes |
|-------------------------|--------------------|------------------|--------|
| Authentification, rôles | Opérateur identité / auth | Miyauth | Rôles (admin, manager, exposant, bénévole) = permissions Master Butler. |
| Profil utilisateur | Opérateur profil | Miyuprofile | Avatar, préférences, thème. |
| Facturation, devis | Opérateur facturation | Miyuinvoice | Devis → facture, PDF, envoi. |
| Contacts / exposants (fiche) | Opérateur contacts | Miyucontacts | Contact, entreprise, coordonnées. |
| Contenu, CMS | Opérateur contenu | Miyucms | Pages, règlements, CGV. |
| Média | Opérateur média | Miyumedia | Logos, photos, pièces jointes. |
| Recherche | Opérateur recherche | Miyusearch | Annuaires, filtres, Store. |
| Notifications | Opérateur notification | Miyunotify | Alertes, annonces, emails. |
| Réservations / créneaux | Opérateur réservation | Miyubooking | Créneaux, programme partiel. |
| Compta / trésorerie | Opérateurs compta | Miyucptaledger, Miyuexpense, Miyutreasury, Miyucomptareports | Budget par édition. |
| Export | Opérateur export | Miyuexport | CSV, rapports. |

### 2.2 Opérateurs à créer (métier événement B2B2C)

| Logique métier Catakana | Opérateur proposé | Rôle | Contrat d’équipe / Mandat |
|-------------------------|--------------------|------|----------------------------|
| **Gestion d’éditions** | Opérateur Édition (ou Événement) | Création/édition d’événements, paramètres, lien organisateur. | Équipe avec MiyuProfile (organisateur), MiyuClock, MiyuSQL. |
| **Organisateur** | Opérateur Organisateur | Représente une structure qui organise des événements ; annuaire organisateurs. | Équipe avec MiyuProfile, Miyucontacts, Miyunotify. |
| **Annuaire exposants** | Opérateur Exposant | Fiche exposant, statuts, participations aux éditions, documents. | Équipe avec Miyucontacts, Miyuprofile, Miyumedia, Miyuinvoice. |
| **Plan de salle** | Opérateur Plan de salle | Plan interactif, zones, stands, attribution. | Équipe avec MiyuSQL, évent. Miyuweb (rendu). |
| **Programme** | Opérateur Programme | Animations, créneaux, salles, conflits. | Équipe avec Miyubooking, MiyuClock. |
| **Candidatures** | Opérateur Candidature | Workflow candidature exposant → édition. | Équipe avec Miyubooking, StrongFather (validation), Miyunotify. |

Pour un **Store B2B2C** :

- **Registre d’événements** = catalogue géré par l’Opérateur Édition + Miyusearch.
- **Répertoire d’organisateurs** = annuaire géré par l’Opérateur Organisateur + Miyuprofile / Miyucontacts.
- **Répertoire d’exposants** = annuaire géré par l’Opérateur Exposant + Miyusearch / Miyucontacts.

Les **Mandats de Permission** et **Contrats d’équipe** définiront qui (organisateur, exposant, admin plateforme) peut créer/modifier quelles ressources (éditions, exposants, plan, programme).

---

## 3. Besoins UI pour la conversion Miyukini

### 3.1 Écrans Catakana à mapper (sources : `src/pages`, `src/components`)

| Écran / zone Catakana | Rôle | Conversion Miyukini (page / Opérateur d’interface) |
|-----------------------|------|----------------------------------------------------|
| **Home / Landing** | Accueil public | Façade publique gouvernée : annuaire événements + répertoire organisateurs + répertoire exposants (Store-like). |
| **Login / Signup / Reset** | Auth | Opérateur identité (Miyauth) + Master Butler (rôles). |
| **Dashboard** | Tableau de bord utilisateur | Hub post-login : accès selon rôle (organisateur / exposant / admin). |
| **Editions** (liste) | Liste des éditions | **Annuaire des événements** : liste/carte, filtres, recherche (Miyusearch). |
| **EditionDashboardPage** | Dashboard par édition | Vue organisateur : résumé édition, accès Plan, Programme, Exposants, Budget, etc. |
| **Exposants** (liste + détail + formulaire) | CRUD exposants | **Répertoire exposants** : liste, fiche, édition (Opérateur Exposant + Miyucontacts, Miyumedia). |
| **Exhibitors** (angl.) / **ExposantForm** | Idem | Unifier avec répertoire exposants + formulaire candidature. |
| **FloorPlan** | Plan de salle | Opérateur Plan de salle : canvas (Fabric.js ou équivalent), zones, drag & drop stands. |
| **Budget / BudgetReport** | Budget édition | Opérateurs compta (Miyucptaledger, Miyuexpense, Miyucomptareports). |
| **Documents** | Documents édition | Miyucms + stockage (KindMother / Miyumedia). |
| **Conventions** | Conventions / lieux | Contenu + carte (Miyucms, évent. Miyuweb). |
| **Contact** | Contact public | Façade publique + Miyucontacts ou formulaire notifié (Miyunotify). |
| **Settings / Account** | Paramètres, compte | Miyuprofile + Miyauth. |
| **Candidatures** (feature) | Candidatures exposants | Opérateur Candidature : liste, détail, validation. |
| **Programme** (sections) | Programme / agenda | Opérateur Programme : grille, créneaux, salles. |
| **News** (admin + public) | Actualités | Miyucms ou Miyufeeds. |
| **Gamification / Rewards** | Récompenses, QR | Miyuposloyalty ou module léger. |

### 3.2 Principes UI pour le modèle Store B2B2C

1. **Annuaire d’événements (catalogue)**  
   - Liste/carte des événements (éditions) avec filtres (date, lieu, organisateur, thème).  
   - Fiche événement : présentation, dates, lieu, lien organisateur, liste exposants, programme public.  
   - Côté Miyukini : **Façade publique gouvernée** + Opérateur Édition + Miyusearch.

2. **Répertoire d’organisateurs**  
   - Liste des structures organisatrices.  
   - Fiche organisateur : nom, événements, contact, charte.  
   - Côté Miyukini : Opérateur Organisateur + Miyuprofile / Miyucontacts, exposé en lecture via Façade publique.

3. **Répertoire d’exposants**  
   - Liste des exposants (toutes éditions ou par événement).  
   - Fiche exposant : entreprise, stands, éditions participées, contact.  
   - Côté Miyukini : Opérateur Exposant + Miyusearch, Façade publique pour partie publique.

4. **Espace organisateur (post-login)**  
   - Tableau de bord par édition : Plan, Programme, Exposants, Candidatures, Budget, Documents.  
   - Gestion des rôles (admin, manager, bénévole) via Master Butler.

5. **Espace exposant (post-login)**  
   - Mes candidatures, mes éditions, mes documents, factures (Miyuinvoice).

6. **Design system**  
   - Catakana : Atomic Design (atoms, molecules, organisms), shadcn/ui, Tailwind.  
   - Conversion : réutiliser composants (boutons, cartes, formulaires) comme **Outil** côté Miyuweb/Miyuwidgets ; garder cohérence avec Miyukini (thèmes, accessibilité).

### 3.3 Stack technique Catakana vs cible Miyukini

| Couche | Catakana | Cible Miyukini (recommandation) |
|--------|----------|----------------------------------|
| Frontend | React 18, Vite, TypeScript | Conserver React/TS pour UI ; appels vers BondingBrother / API COG. |
| UI | Tailwind, Radix, shadcn, Fabric.js (plan) | Conserver ou aligner sur stack Miyukini (Miyuweb, Miyuwidgets). |
| Données | Supabase (PostgreSQL, Auth, Storage) | KindMother + MiyuSQL côté COG ; Supabase peut rester comme persistance déléguée selon contrat. |
| State / API | React Query, services Supabase directs | Remplacer par appels Mandatés vers Opérateurs (BondingBrother). |
| Auth | Supabase Auth + rôles custom | Miyauth + Master Butler (permissions). |

---

## 4. Synthèse et prochaines étapes

### 4.1 Kits d’outils

- **Réutilisables tels quels** : Miyauth, Miyuprofile, Miyuinvoice, Miyucontacts, Miyucms, Miyumedia, Miyusearch, Miyunotify, Miyubooking, Miyuexport, Miyucptaledger, Miyuexpense, Miyutreasury, Miyucomptareports.
- **À créer ou étendre** : Édition/événement, Plan de salle (floor plan), Programme/schedule, Annuaire exposants (exhibitor), Candidature (ou extension Miyubooking).

### 4.2 Opérateurs

- **Existants** : Identité, Profil, Facturation, Contacts, Contenu, Média, Recherche, Notification, Réservation, Compta, Export.
- **À concevoir** : Édition (événement), Organisateur, Exposant, Plan de salle, Programme, Candidature ; avec Contrats d’équipe et Mandats de Permission pour le B2B2C.

### 4.3 UI

- **Store-like** : Annuaire événements + Répertoire organisateurs + Répertoire exposants (Façade publique).
- **Espace organisateur** : Dashboard par édition, Plan, Programme, Exposants, Candidatures, Budget, Documents.
- **Espace exposant** : Candidatures, éditions, documents, factures.
- **Design** : Partir d’Atomic Design / shadcn existant et aligner sur Miyuweb/Miyuwidgets et gouvernance COG.

### 4.4 Ordre de travail suggéré

1. **Modéliser** les entités COG : Édition, Organisateur, Exposant (schémas KindMother / MiyuSQL).  
2. **Déclarer** les Opérateurs (Édition, Organisateur, Exposant) et leurs Contrats d’équipe.  
3. **Implémenter** les toolkits manquants (au moins Édition, Exposant, puis Plan, Programme).  
4. **Exposer** les annuaires (événements, organisateurs, exposants) via Façade publique + Miyusearch.  
5. **Migrer** les écrans Catakana un par un (auth → dashboard → éditions → exposants → plan → programme → budget).  
6. **Connecter** authentification et rôles à Miyauth + Master Butler.

---

**Document** : Audit Catakana → Miyukini COG (conversion B2B2C)  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document d’audit — base pour plan de conversion
