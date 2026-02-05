# JayFestival — Bornage pour l’implémentation

## Contexte

Ce document définit le **bornage** (périmètre, limites, priorités) pour l’**implémentation** du service JayFestival : ce qui est **in scope** et **hors scope** par phase, les **dépendances** techniques et fonctionnelles, et les **critères de livraison** pour une **version alpha** fonctionnelle puis les phases suivantes. Il s’appuie sur l’[Audit documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) et sur les documents publics (Organisateurs, Exposants, Visiteurs, Utilisateur non connecté).

**Version alpha** : Catakana fonctionne déjà ; l’alpha JayFestival vise une **reprise fonctionnelle** (egui/eframe + même périmètre métier) avec **Supabase en backend** (exception pré-COG). Ce n’est pas un MVP « minimal » mais une version **fonctionnelle**. La migration vers **SQLite + outils maison** (KindMother) est documentée pour la version COG-native ultérieure (voir [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md)).

## Portée / Scope

- **Périmètre** : Périmètre d’implémentation (Alpha, phase 2, etc.) ; backend alpha = Supabase (exception pré-COG) ; migration Supabase → SQLite + outils maison ; dépendances COG (Miyauth, KindMother, etc.) cible post-alpha ; stack UI (egui/eframe — voir [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md)).
- **Hors périmètre** : Spécifications techniques détaillées des API et schémas de données — référencées dans les contrats d’Opérateurs et Kits.

---

## 1. Périmètre fonctionnel par phase

### 1.1 Alpha (Phase 1) — Version fonctionnelle — In scope

| Capacité | Description | Priorité |
|----------|-------------|----------|
| **Catalogue (Façade publique)** | Annuaire des événements, répertoire des organisateurs, répertoire des exposants ; liste, filtres, fiche événement/organisateur/exposant (lecture seule). | Must |
| **Onboarding organisateur** | Création de compte organisateur (Miyauth, Miyuprofile), validation, attribution rôles (Admin, Manager), Mandat ; tableau de bord organisateur, liste des éditions. | Must |
| **Éditions** | Création, paramétrage, liste des éditions ; dashboard par édition (indicateurs : exposants, candidatures, budget synthèse, programme, plan). | Must |
| **Exposants (côté organisateur)** | Annuaire local par édition, réception candidatures, validation/refus, fiche exposant, devis et facture (Miyuinvoice / JayKonta), import CSV. | Must |
| **Plan de salle** | Zones, stands, attribution aux exposants (formulaire ou drag & drop), visualisation, export visuel. | Must |
| **Programme** | Animations, salles/scènes, horaires, blocage chevauchements, vues chronologique et par salle, publication programme public. | Must |
| **Budget** | Saisie revenus/dépenses, ventilation par catégorie, balance et statistiques par édition ; lien facturation → revenus. | Must |
| **Documents & Légal** | Contrats types, CGV, règlements ; envoi aux exposants, historique. | Must |
| **Notifications** | Annonces globales, notifications ciblées (Miyunotify), paramétrage par édition. | Must |
| **Dashboard exposant** | Compte cross-événements, liste candidatures/participations, agenda (conflits de dates via JayKoa), documents, factures. | Must |
| **Espace visiteur (base)** | Compte cross-événements, agenda personnel, billets/réservations (Miyubooking), pass VIP ; onboarding par festival ou groupe. | Must |
| **Gouvernance** | StrongFather (Mandats), Master Butler (permissions), KindMother (persistance), WorrySentinel (niveau de sécurité). | Must |
| **UI stack** | Implémentation UI selon [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) et **conformité obligatoire** à la [Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) : protocoles, thème (tokens), atoms/molecules/organisms egui, parcours par écran (composants ordonnés), checklist conformité. | Must |
| **Backend alpha** | **Supabase** (Auth + PostgreSQL + Storage si besoin) en **exception pré-COG** ; interactions DB documentées dans [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md). | Must |

### 1.2 Alpha (Phase 1) — Hors scope

| Élément | Raison |
|---------|--------|
| **Services visiteur avancés** | Jeux, concours, ateliers (configuration complète) : phase 2 ; base réservations/billets/pass en alpha. |
| **Journal des modifications (programme)** | Reporté phase 2 ; voir audit. |
| **Commentaires internes / notes privées exposants** | Reporté phase 2 ; voir audit. |
| **Import Google Sheet** | CSV/tableur en alpha ; Google Sheet explicite en phase 2 si besoin. |
| **Gestion matériel, reporting avancé, interventions techniques/urgences** | Hors scope alpha ; backlog ou autre service. |
| **Module Actualités (News) éditorial public** | Annonces (organisateur → exposants/équipe) en alpha ; flux Actualités type Catakana en phase 2 (Miyufeeds/Miyucms). |
| **RPG / gamification / galeries par édition** | Hors scope JayFestival v1 ou autre Opérateur. |
| **JayXpose (fiche exposant / répertoire)** | **JayXpose est dans l’alpha** : le parcours de **demande de stands** et l’**annuaire exposants** ne peuvent pas fonctionner sans JayXpose. Fiche exposant et répertoire s’appuient sur le profil JayXpose (données Supabase `exposants`/JayXpose). Voir [État Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md). |
| **JayFaim (restauration sur événement)** | **Phase 2** ; pas de doc Opérateurs/UI requise pour alpha. Voir [État Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md). |
| **UI web React (Catakana)** | Stack cible = egui/eframe (Miyukini) ; transcription des écrans et du design, pas reprise du code React. |
| **KindMother / SQLite en alpha** | Alpha = Supabase ; migration vers SQLite + outils maison en post-alpha (voir Reference Base de Donnees et Migration). |

### 1.3 Phase 2 — Extension prévue

| Capacité | Description |
|----------|-------------|
| **Services visiteur complets** | Jeux, concours, ateliers (créneaux, capacité), pass VIP (tarifs, avantages) ; configuration organisateur et consommation visiteur. |
| **Journal des modifications (programme)** | Historique des changements d’animations/créneaux. |
| **Commentaires internes / notes privées (exposants)** | Notes organisateur sur fiche exposant, non visibles par l’exposant. |
| **Actualités (News) public** | Flux éditorial par édition (Miyucms/Miyufeeds), affichage public. |
| **Export PDF programme / plan** | En plus des exports déjà prévus (plan, liste). |
| **Composants UI réutilisables** | Bibliothèque egui (cartes, listes, formulaires) alignée sur [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md). |

### 1.4 Phase 3 et au-delà — Optionnel

| Capacité | Description |
|----------|-------------|
| **Gestion matériel prêté/placé** | Catalogue matériel, inventaire, listes (voir audit). |
| **Reporting avancé** | Participation, paiements, retards, tableaux de bord analytiques. |
| **Interventions techniques / urgences** | Suivi des interventions, alertes. |
| **Synchronisation calendriers externes** | Via JayKoa ou services consommateurs (Google, Outlook). |

---

## 2. Backend et base de données (alpha et migration)

### 2.0 Principe

- **Genèse** : JayFestival a pour genèse **Catakana**, qui est **pré-COG** et repose sur **Supabase** (PostgreSQL, Auth, Storage). C’est la **seule exception** documentée : pour la **version alpha fonctionnelle**, le backend est **Supabase**.
- **Décision alpha** : **En alpha, vu que Catakana est déjà en production, on garde Supabase pour JayFestival** : même infrastructure (Auth, PostgreSQL, Storage), données mère et référence centrale (tracker) en alpha = Supabase Catakana. Détail : [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md).
- **Alpha** : Backend = Supabase (client REST/PostgREST, Auth). Tables et services Catakana (éditions, exposants, editions_exposants, budget_entries, invoices, stands, schedule_slots, etc.) ; RLS et `profiles.user_type` pour rôles.
- **Migration** : Pour la version **COG-native** (post-alpha), migration **Supabase → SQLite + outils maison** (KindMother, persistance locale). Stratégie en 6 étapes (schéma SQLite, contrats KindMother, export Supabase, import SQLite, couche d’abstraction, bascule) dans le même document. **Option** : Supabase peut être conservé comme **serveur zéro** (backup et restauration), sans dépendance critique à l’exécution (LOI-1).

---

## 3. Dépendances techniques et fonctionnelles

### 3.1 Dépendances alpha (exception Supabase)

| Dépendance | Rôle en alpha |
|------------|----------------|
| **Supabase** | **Backend alpha** : Auth (email/mot de passe, lien magique), PostgreSQL (tables éditions, exposants, editions_exposants, budget_entries, invoices, stands, programme, documents, etc.), Storage si besoin. RLS = proxy permissions (admin, manager, exhibitor, volunteer, visitor). **JayFestival dispose d’une Auth à lui**, dérivée de l’Auth Catakana qui utilise Supabase Auth ; en alpha, l’Auth JayFestival s’appuie sur Supabase Auth. |
| **Stack UI** | egui, eframe (voir [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md)). |

### 3.2 Dépendances cible post-alpha (COG-native)

| Dépendance | Rôle post-alpha |
|------------|------------------|
| **Miyauth** | Authentification (organisateur, exposant, visiteur), lien magique, session. |
| **Miyuprofile** | Profil utilisateur, fiche organisateur, fiche exposant. |
| **Miyunotify** | Annonces, notifications ciblées, rappels. |
| **Miyuinvoice / JayKonta** | Devis, factures, suivi paiements ; budget édition. |
| **JayKoa** | Agenda agrégé, conflits de dates. |
| **JayXpose** | Fiche exposant, répertoire exposants (optionnel alpha ; local JayFestival possible). |
| **JayFaim** | Restauration sur événement : phase 2 ou optionnel. |
| **Miyubooking** | Réservations (ateliers, créneaux, billets, pass). |
| **KindMother** | Persistance (SQLite + outils maison) : éditions, candidatures, plan de salle, programme, budget, documents. |
| **StrongFather** | Mandats de Permission. |
| **Master Butler** | Permissions par rôle. |
| **WorrySentinel** | Niveau de sécurité, états de confiance. |

### 3.3 Dépendances optionnelles (phases ultérieures)

| Dépendance | Rôle |
|------------|------|
| **Miyucms / Miyumedia** | Documents, médias, actualités. |
| **Miyufeeds** | Flux actualités public. |
| **Miyucptaledger / Miyuexpense / Miyucomptareports** | Comptabilité avancée, rapports. |

---

## 4. Critères de fin de phase (Alpha)

| Critère | Description |
|---------|-------------|
| **CF-ALPHA-1** | Catalogue (annuaire événements, répertoires organisateurs/exposants) accessible en lecture ; données fournies par Supabase. |
| **CF-ALPHA-2** | Organisateur : connexion (Supabase Auth), liste des éditions, dashboard par édition, exposants (candidatures, validation, fiches, devis/facture), plan de salle, programme, budget, documents, notifications. |
| **CF-ALPHA-3** | Exposant : dashboard (candidatures, participations, documents, factures) ; données depuis Supabase. |
| **CF-ALPHA-4** | Visiteur : espace dédié (agenda, billets, réservations, pass VIP) selon tables Supabase existantes. |
| **CF-ALPHA-5** | Rôles et accès : cohérents avec `profiles.user_type` et RLS Supabase (admin, manager, exhibitor, volunteer, visitor). |
| **CF-ALPHA-6** | UI : thème (tokens) et écrans principaux selon [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) ; stack egui/eframe. |
| **CF-ALPHA-7** | Documentation : Document fondateur, publics, Bornage (alpha + migration), [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md), Reference UI Transcription Catakana, Interpolarité à jour. |

---

## 5. Hors scope explicite (toutes phases sauf mention)

| Élément | Commentaire |
|---------|-------------|
| **Logique métier hors JayFestival** | Décision de validation candidature, émission facture : gouvernée par StrongFather / JayKonta ; JayFestival orchestre et affiche. |
| **Authentification** | Déléguée à Miyauth ; JayFestival consomme le contexte utilisateur (rôle, Mandat). |
| **Envoi d’emails / SMS** | Délégué à Miyunotify ; JayFestival déclenche, ne gère pas le transport. |
| **Copie canonique des données exposant** | Politique de résidence : COG de l’organisateur ou du Service Festival ; voir [Politique Residence Donnees Sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md). |

---

## 6. Références

| Document | Rôle |
|----------|------|
| [JayFestival - Document Fondateur](./JayFestival%20-%20Document%20Fondateur.md) | Contexte, vision, macro, distribution. |
| [JayFestival - Audit Documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) | Métriques, manques, recommandations. |
| [JayFestival - Reference Base de Donnees et Migration Supabase vers SQLite](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | État actuel Supabase, mapping tables/services, stratégie migration SQLite + outils maison, critères alpha. |
| [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) | UI complète Catakana → stack actuelle (Atomic, thème, ui-kit, écrans). |
| [JayFestival - Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages JayXpose, JayFaim, JayKoa, JayKonta. |
| [Miyukini - Stack UI egui eframe](../../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md) | Stack UI officielle Miyukini. |
| Organisateurs / Exposants / Visiteurs / UNC — Analyse des besoins, Écrans et cycle, Opérateurs et Toolkits | Besoins et écrans par public. |

---

**Document** : JayFestival — Bornage pour l’implémentation  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de référence (bornage implémentation)
