# JayFestival â€” Bornage pour lâ€™implÃ©mentation

## Contexte

Ce document dÃ©finit le **bornage** (pÃ©rimÃ¨tre, limites, prioritÃ©s) pour lâ€™**implÃ©mentation** du service JayFestival : ce qui est **in scope** et **hors scope** par phase, les **dÃ©pendances** techniques et fonctionnelles, et les **critÃ¨res de livraison** pour une **version alpha** fonctionnelle puis les phases suivantes. Il sâ€™appuie sur lâ€™[Audit documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) et sur les documents publics (Organisateurs, Exposants, Visiteurs, Utilisateur non connectÃ©).

**Version alpha** : Catakana fonctionne dÃ©jÃ  ; lâ€™alpha JayFestival vise une **reprise fonctionnelle** (Dioxus + mÃªme pÃ©rimÃ¨tre mÃ©tier) avec **Supabase en backend** (exception prÃ©-COG). Ce nâ€™est pas un MVP Â« minimal Â» mais une version **fonctionnelle**. La migration vers **SQLite + outils maison** (KindMother) est documentÃ©e pour la version COG-native ultÃ©rieure (voir [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md)).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : PÃ©rimÃ¨tre dâ€™implÃ©mentation (Alpha, phase 2, etc.) ; backend alpha = Supabase (exception prÃ©-COG) ; migration Supabase â†’ SQLite + outils maison ; dÃ©pendances COG (Miyauth, KindMother, etc.) cible post-alpha ; stack UI (Dioxus â€” voir [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md)).
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques dÃ©taillÃ©es des API et schÃ©mas de donnÃ©es â€” rÃ©fÃ©rencÃ©es dans les contrats dâ€™OpÃ©rateurs et Kits.

---

## 1. PÃ©rimÃ¨tre fonctionnel par phase

### 1.1 Alpha (Phase 1) â€” Version fonctionnelle â€” In scope

| CapacitÃ© | Description | PrioritÃ© |
|----------|-------------|----------|
| **Catalogue (FaÃ§ade publique)** | Annuaire des Ã©vÃ©nements, rÃ©pertoire des organisateurs, rÃ©pertoire des exposants ; liste, filtres, fiche Ã©vÃ©nement/organisateur/exposant (lecture seule). | Must |
| **Onboarding organisateur** | CrÃ©ation de compte organisateur (Miyauth, Miyuprofile), validation, attribution rÃ´les (Admin, Manager), Mandat ; tableau de bord organisateur, liste des Ã©ditions. | Must |
| **Ã‰ditions** | CrÃ©ation, paramÃ©trage, liste des Ã©ditions ; dashboard par Ã©dition (indicateurs : exposants, candidatures, budget synthÃ¨se, programme, plan). | Must |
| **Exposants (cÃ´tÃ© organisateur)** | Annuaire local par Ã©dition, rÃ©ception candidatures, validation/refus, fiche exposant, devis et facture (Miyuinvoice / JayKonta), import CSV. | Must |
| **Plan de salle** | Zones, stands, attribution aux exposants (formulaire ou drag & drop), visualisation, export visuel. | Must |
| **Programme** | Animations, salles/scÃ¨nes, horaires, blocage chevauchements, vues chronologique et par salle, publication programme public. | Must |
| **Budget** | Saisie revenus/dÃ©penses, ventilation par catÃ©gorie, balance et statistiques par Ã©dition ; lien facturation â†’ revenus. | Must |
| **Documents & LÃ©gal** | Contrats types, CGV, rÃ¨glements ; envoi aux exposants, historique. | Must |
| **Notifications** | Annonces globales, notifications ciblÃ©es (Miyunotify), paramÃ©trage par Ã©dition. | Must |
| **Dashboard exposant** | Compte cross-Ã©vÃ©nements, liste candidatures/participations, agenda (conflits de dates via JayKoa), documents, factures. | Must |
| **Espace visiteur (base)** | Compte cross-Ã©vÃ©nements, agenda personnel, billets/rÃ©servations (Miyubooking), pass VIP ; onboarding par festival ou groupe. | Must |
| **Gouvernance** | StrongFather (Mandats), Master Butler (permissions), KindMother (persistance), WorrySentinel (niveau de sÃ©curitÃ©). | Must |
| **UI stack** | ImplÃ©mentation UI selon [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) et **conformitÃ© obligatoire** Ã  la [Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) : protocoles, thÃ¨me (tokens), atoms/molecules/organisms Dioxus, parcours par Ã©cran (composants ordonnÃ©s), checklist conformitÃ©. | Must |
| **Backend alpha** | **Supabase** (Auth + PostgreSQL + Storage si besoin) en **exception prÃ©-COG** ; interactions DB documentÃ©es dans [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md). | Must |

### 1.2 Alpha (Phase 1) â€” Hors scope

| Ã‰lÃ©ment | Raison |
|---------|--------|
| **Services visiteur avancÃ©s** | Jeux, concours, ateliers (configuration complÃ¨te) : phase 2 ; base rÃ©servations/billets/pass en alpha. |
| **Journal des modifications (programme)** | ReportÃ© phase 2 ; voir audit. |
| **Commentaires internes / notes privÃ©es exposants** | ReportÃ© phase 2 ; voir audit. |
| **Import Google Sheet** | CSV/tableur en alpha ; Google Sheet explicite en phase 2 si besoin. |
| **Gestion matÃ©riel, reporting avancÃ©, interventions techniques/urgences** | Hors scope alpha ; backlog ou autre service. |
| **Module ActualitÃ©s (News) Ã©ditorial public** | Annonces (organisateur â†’ exposants/Ã©quipe) en alpha ; flux ActualitÃ©s type Catakana en phase 2 (Miyufeeds/Miyucms). |
| **RPG / gamification / galeries par Ã©dition** | Hors scope JayFestival v1 ou autre OpÃ©rateur. |
| **JayXpose (fiche exposant / rÃ©pertoire)** | **JayXpose est dans lâ€™alpha** : le parcours de **demande de stands** et lâ€™**annuaire exposants** ne peuvent pas fonctionner sans JayXpose. Fiche exposant et rÃ©pertoire sâ€™appuient sur le profil JayXpose (donnÃ©es Supabase `exposants`/JayXpose). Voir [Ã‰tat Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md). |
| **JayFaim (restauration sur Ã©vÃ©nement)** | **Phase 2** ; pas de doc OpÃ©rateurs/UI requise pour alpha. Voir [Ã‰tat Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md). |
| **UI web React (Catakana)** | Stack cible = Dioxus (Miyukini) ; transcription des Ã©crans et du design, pas reprise du code React. |
| **KindMother / SQLite en alpha** | Alpha = Supabase ; migration vers SQLite + outils maison en post-alpha (voir Reference Base de Donnees et Migration). |

### 1.3 Phase 2 â€” Extension prÃ©vue

| CapacitÃ© | Description |
|----------|-------------|
| **Services visiteur complets** | Jeux, concours, ateliers (crÃ©neaux, capacitÃ©), pass VIP (tarifs, avantages) ; configuration organisateur et consommation visiteur. |
| **Journal des modifications (programme)** | Historique des changements dâ€™animations/crÃ©neaux. |
| **Commentaires internes / notes privÃ©es (exposants)** | Notes organisateur sur fiche exposant, non visibles par lâ€™exposant. |
| **ActualitÃ©s (News) public** | Flux Ã©ditorial par Ã©dition (Miyucms/Miyufeeds), affichage public. |
| **Export PDF programme / plan** | En plus des exports dÃ©jÃ  prÃ©vus (plan, liste). |
| **Composants UI rÃ©utilisables** | BibliothÃ¨que Dioxus (cartes, listes, formulaires) alignÃ©e sur [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md). |

### 1.4 Phase 3 et au-delÃ  â€” Optionnel

| CapacitÃ© | Description |
|----------|-------------|
| **Gestion matÃ©riel prÃªtÃ©/placÃ©** | Catalogue matÃ©riel, inventaire, listes (voir audit). |
| **Reporting avancÃ©** | Participation, paiements, retards, tableaux de bord analytiques. |
| **Interventions techniques / urgences** | Suivi des interventions, alertes. |
| **Synchronisation calendriers externes** | Via JayKoa ou services consommateurs (Google, Outlook). |

---

## 2. Backend et base de donnÃ©es (alpha et migration)

### 2.0 Principe

- **GenÃ¨se** : JayFestival a pour genÃ¨se **Catakana**, qui est **prÃ©-COG** et repose sur **Supabase** (PostgreSQL, Auth, Storage). Câ€™est la **seule exception** documentÃ©e : pour la **version alpha fonctionnelle**, le backend est **Supabase**.
- **DÃ©cision alpha** : **En alpha, vu que Catakana est dÃ©jÃ  en production, on garde Supabase pour JayFestival** : mÃªme infrastructure (Auth, PostgreSQL, Storage), donnÃ©es mÃ¨re et rÃ©fÃ©rence centrale (tracker) en alpha = Supabase Catakana. DÃ©tail : [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md).
- **Alpha** : Backend = Supabase (client REST/PostgREST, Auth). Tables et services Catakana (Ã©ditions, exposants, editions_exposants, budget_entries, invoices, stands, schedule_slots, etc.) ; RLS et `profiles.user_type` pour rÃ´les.
- **Migration** : Pour la version **COG-native** (post-alpha), migration **Supabase â†’ SQLite + outils maison** (KindMother, persistance locale). StratÃ©gie en 6 Ã©tapes (schÃ©ma SQLite, contrats KindMother, export Supabase, import SQLite, couche dâ€™abstraction, bascule) dans le mÃªme document. **Option** : Supabase peut Ãªtre conservÃ© comme **serveur zÃ©ro** (backup et restauration), sans dÃ©pendance critique Ã  lâ€™exÃ©cution (LOI-1).

---

## 3. DÃ©pendances techniques et fonctionnelles

### 3.1 DÃ©pendances alpha (exception Supabase)

| DÃ©pendance | RÃ´le en alpha |
|------------|----------------|
| **Supabase** | **Backend alpha** : Auth (email/mot de passe, lien magique), PostgreSQL (tables Ã©ditions, exposants, editions_exposants, budget_entries, invoices, stands, programme, documents, etc.), Storage si besoin. RLS = proxy permissions (admin, manager, exhibitor, volunteer, visitor). **JayFestival dispose dâ€™une Auth Ã  lui**, dÃ©rivÃ©e de lâ€™Auth Catakana qui utilise Supabase Auth ; en alpha, lâ€™Auth JayFestival sâ€™appuie sur Supabase Auth. |
| **Stack UI** | Dioxus (voir [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md)). |

### 3.2 DÃ©pendances cible post-alpha (COG-native)

| DÃ©pendance | RÃ´le post-alpha |
|------------|------------------|
| **Miyauth** | Authentification (organisateur, exposant, visiteur), lien magique, session. |
| **Miyuprofile** | Profil utilisateur, fiche organisateur, fiche exposant. |
| **Miyunotify** | Annonces, notifications ciblÃ©es, rappels. |
| **Miyuinvoice / JayKonta** | Devis, factures, suivi paiements ; budget Ã©dition. |
| **JayKoa** | Agenda agrÃ©gÃ©, conflits de dates. |
| **JayXpose** | Fiche exposant, rÃ©pertoire exposants (optionnel alpha ; local JayFestival possible). |
| **JayFaim** | Restauration sur Ã©vÃ©nement : phase 2 ou optionnel. |
| **Miyubooking** | RÃ©servations (ateliers, crÃ©neaux, billets, pass). |
| **KindMother** | Persistance (SQLite + outils maison) : Ã©ditions, candidatures, plan de salle, programme, budget, documents. |
| **StrongFather** | Mandats de Permission. |
| **Master Butler** | Permissions par rÃ´le. |
| **WorrySentinel** | Niveau de sÃ©curitÃ©, Ã©tats de confiance. |

### 3.3 DÃ©pendances optionnelles (phases ultÃ©rieures)

| DÃ©pendance | RÃ´le |
|------------|------|
| **Miyucms / Miyumedia** | Documents, mÃ©dias, actualitÃ©s. |
| **Miyufeeds** | Flux actualitÃ©s public. |
| **Miyucptaledger / Miyuexpense / Miyucomptareports** | ComptabilitÃ© avancÃ©e, rapports. |

---

## 4. CritÃ¨res de fin de phase (Alpha)

| CritÃ¨re | Description |
|---------|-------------|
| **CF-ALPHA-1** | Catalogue (annuaire Ã©vÃ©nements, rÃ©pertoires organisateurs/exposants) accessible en lecture ; donnÃ©es fournies par Supabase. |
| **CF-ALPHA-2** | Organisateur : connexion (Supabase Auth), liste des Ã©ditions, dashboard par Ã©dition, exposants (candidatures, validation, fiches, devis/facture), plan de salle, programme, budget, documents, notifications. |
| **CF-ALPHA-3** | Exposant : dashboard (candidatures, participations, documents, factures) ; donnÃ©es depuis Supabase. |
| **CF-ALPHA-4** | Visiteur : espace dÃ©diÃ© (agenda, billets, rÃ©servations, pass VIP) selon tables Supabase existantes. |
| **CF-ALPHA-5** | RÃ´les et accÃ¨s : cohÃ©rents avec `profiles.user_type` et RLS Supabase (admin, manager, exhibitor, volunteer, visitor). |
| **CF-ALPHA-6** | UI : thÃ¨me (tokens) et Ã©crans principaux selon [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) ; stack Dioxus. |
| **CF-ALPHA-7** | Documentation : Document fondateur, publics, Bornage (alpha + migration), [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md), Reference UI Transcription Catakana, InterpolaritÃ© Ã  jour. |

---

## 5. Hors scope explicite (toutes phases sauf mention)

| Ã‰lÃ©ment | Commentaire |
|---------|-------------|
| **Logique mÃ©tier hors JayFestival** | DÃ©cision de validation candidature, Ã©mission facture : gouvernÃ©e par StrongFather / JayKonta ; JayFestival orchestre et affiche. |
| **Authentification** | DÃ©lÃ©guÃ©e Ã  Miyauth ; JayFestival consomme le contexte utilisateur (rÃ´le, Mandat). |
| **Envoi dâ€™emails / SMS** | DÃ©lÃ©guÃ© Ã  Miyunotify ; JayFestival dÃ©clenche, ne gÃ¨re pas le transport. |
| **Copie canonique des donnÃ©es exposant** | Politique de rÃ©sidence : COG de lâ€™organisateur ou du Service Festival ; voir [Politique Residence Donnees Sensibles](..//..//miyukini-webway-system//reference//_index.md). |

---

## 6. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayFestival - Document Fondateur](./JayFestival%20-%20Document%20Fondateur.md) | Contexte, vision, macro, distribution. |
| [JayFestival - Audit Documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) | MÃ©triques, manques, recommandations. |
| [JayFestival - Reference Base de Donnees et Migration Supabase vers SQLite](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | Ã‰tat actuel Supabase, mapping tables/services, stratÃ©gie migration SQLite + outils maison, critÃ¨res alpha. |
| [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) | UI complÃ¨te Catakana â†’ stack actuelle (Atomic, thÃ¨me, ui-kit, Ã©crans). |
| [JayFestival - Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages JayXpose, JayFaim, JayKoa, JayKonta. |
| [Miyukini - Stack UI Dioxus](..//..//_index.md) | Stack UI officielle Miyukini. |
| Organisateurs / Exposants / Visiteurs / UNC â€” Analyse des besoins, Ã‰crans et cycle, OpÃ©rateurs et Toolkits | Besoins et Ã©crans par public. |

---

**Document** : JayFestival â€” Bornage pour lâ€™implÃ©mentation  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de rÃ©fÃ©rence (bornage implÃ©mentation)


