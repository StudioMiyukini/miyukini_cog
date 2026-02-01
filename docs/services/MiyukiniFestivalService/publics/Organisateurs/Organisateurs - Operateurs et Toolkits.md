# Organisateurs — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du public **Organisateurs** du service Miyukini Festival Service. Il s’appuie sur le [Parcours, capacités et livrables](./Organisateurs%20-%20Parcours%20Capacites%20Livrables.md) et les documents associés. Il vise à fournir une référence produit pour l’architecture gouvernée : quels Opérateurs exposent les capacités aux organisateurs, et quels Kits d’outils agrègent les outils sous-jacents.

## Portée / Scope

- **Public** : Organisateurs (structures qui créent et gèrent des événements/festivals).
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir les livrables du public (éditions multi-festivals, exposants, plan de salle, programme, budget, documents, services visiteur, publication catalogue).
- **Hors périmètre** : Spécifications d’implémentation (API, schémas, code) ; définition détaillée des Cores — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |

Les organisateurs **interagissent avec** des Opérateurs gouvernés (tableau de bord, éditions, exposants, plan de salle, programme, budget, documents, services visiteur, publication) ; ces Opérateurs s’appuient sur des Toolkits et des composants Miyukini.

---

## 2. Besoins en Opérateurs (public Organisateurs)

### 2.1 Opérateur « MFS Organisateur » (tableau de bord organisateur)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **tableau de bord organisateur** : liste de toutes les éditions (multi-festivals), accès au dashboard par édition, équipe (rôles, bénévoles), paramètres. |
| **Public servi** | Organisateurs authentifiés (rôle Admin organisateur ou Manager). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; permissions (Master Butler) ; persistance (KindMother) ; sécurité (WorrySentinel). |
| **Capacités exposées** | Liste des éditions (passées, en cours, à venir) ; accès au dashboard par édition (exposants, candidatures, budget, programme, plan, documents) ; gestion de l’équipe (Admin, Manager, Bénévole) ; paramètres de la structure ; publication au catalogue. |
| **Ne fait pas** | Décision de validation des candidatures exposants (StrongFather) ; émission des factures (Miyuinvoice). |

Cet Opérateur est le **point d’entrée** principal de l’organisateur : il agrège la vue multi-éditions et délègue le détail par édition à MFS Édition.

### 2.2 Opérateur « MFS Édition » (dashboard par édition)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **dashboard par édition** : exposants (candidatures, validation, refus, fiches), plan de salle (zones, stands, attribution), programme (animations, salles, horaires), budget (revenus, dépenses), documents (contrats, règlements), services visiteur (activation, paramétrage), notifications. |
| **Public servi** | Organisateurs authentifiés (Admin ou Manager selon édition assignée). |
| **Gouvernance** | Mandat de Permission ; données souveraines à l’organisateur pour son édition ; pas d’accès aux données des autres organisateurs. |
| **Capacités exposées** | Candidatures exposants (réception, consultation, validation, refus) ; fiches exposants ; plan de salle (zones, stands, attribution) ; programme (animations, salles, créneaux) ; budget (revenus, dépenses, ventilation) ; documents (contrats types, CGV, conventions) ; services visiteur (jeux, concours, ateliers, réservations, pass VIP — activation et règles) ; annonces et notifications (Miyunotify). |
| **Lien avec MFS Organisateur** | MFS Organisateur **orchestre** l’accès aux éditions ; MFS Édition est **instancié par édition** (ou un seul Opérateur avec contexte édition). |

### 2.3 Synthèse des Opérateurs (public Organisateurs)

| Opérateur | Usage par l’organisateur | Livrables couverts |
|-----------|--------------------------|---------------------|
| **MFS Organisateur** | Connexion, liste des éditions, équipe, paramètres, publication catalogue. | Tableau de bord organisateur, multi-festivals. |
| **MFS Édition** | Dashboard par édition : exposants, plan de salle, programme, budget, documents, services visiteur, notifications. | Éditions, exposants, plan de salle, programme, budget, documents, services visiteur, publication. |

---

## 3. Besoins en Toolkits (public Organisateurs)

### 3.1 Kit « Éditions » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **création et le paramétrage des éditions** (événements) : nom, dates, lieu, thème, règles, objectifs. |
| **Outils agrégés (exemples)** | `edition.create`, `edition.update`, `edition.list.byOrganiser`, `edition.get`, `edition.publish` (publication au catalogue), `edition.close`. |
| **Consommé par** | MFS Organisateur, MFS Édition. |
| **Composants sous-jacents** | KindMother. |

### 3.2 Kit « Exposants (côté organisateur) » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **gestion des exposants par édition** : réception des candidatures, consultation, validation, refus, fiches exposants, attribution emplacement, devis et factures (Miyuinvoice). |
| **Outils agrégés (exemples)** | `application.list.byEdition` (candidatures reçues), `application.get`, `application.validate`, `application.reject`, `exposant.list.byEdition` (exposants validés), `exposant.stand.assign`, `invoice.generate`, `invoice.send`. |
| **Consommé par** | MFS Édition. |
| **Composants sous-jacents** | KindMother (candidatures), Miyuinvoice (devis, factures), Miyunotify (notifications). |

### 3.3 Kit « Plan de salle » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **définition du plan de salle** : zones, tailles de stand, légende ; attribution des emplacements aux exposants (formulaire ou drag & drop) ; export visuel. |
| **Outils agrégés (exemples)** | `floorplan.create`, `floorplan.update`, `floorplan.zones.list`, `floorplan.stand.assign`, `floorplan.export` (PDF, image). |
| **Consommé par** | MFS Édition. |
| **Composants sous-jacents** | KindMother. |

### 3.4 Kit « Programme » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **gestion du programme** : animations, salles/scènes, horaires, blocage des chevauchements ; vues chronologique ou par salle. |
| **Outils agrégés (exemples)** | `programme.animation.create`, `programme.animation.update`, `programme.animation.delete`, `programme.room.list`, `programme.slot.check` (chevauchement), `programme.view.chrono`, `programme.view.byRoom`. |
| **Consommé par** | MFS Édition. |
| **Composants sous-jacents** | KindMother, MiyuClock (créneaux). |

### 3.5 Kit « Budget » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **saisie et la ventilation du budget** : revenus, dépenses, ventilation par catégorie ; statistiques et balance par édition ou période. |
| **Outils agrégés (exemples)** | `budget.entry.create` (revenu ou dépense), `budget.entry.list`, `budget.aggregate` (statistiques, balance), `budget.export`. |
| **Consommé par** | MFS Édition. |
| **Composants sous-jacents** | KindMother ; Miyuinvoice (revenus liés aux factures). |

### 3.6 Kit « Documents & Légal » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer les **contrats types, CGV, conventions, règlements** : création, mise à jour, partage avec les exposants ; historique des documents validés. |
| **Outils agrégés (exemples)** | `document.template.list`, `document.template.create`, `document.template.update`, `document.send.toExposant`, `document.history.get`. |
| **Consommé par** | MFS Édition. |
| **Composants sous-jacents** | KindMother, Miyunotify. |

### 3.7 Kit « Services visiteur » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’**activation et le paramétrage des services visiteur** par édition : jeux, concours, ateliers, réservations, pass VIP ; places limitées, dates, publics éligibles. |
| **Outils agrégés (exemples)** | `visitorService.list` (services disponibles), `visitorService.activate` (activer pour l’édition), `visitorService.config.set` (règles, places, dates), `visitorService.eligibility.set`. |
| **Consommé par** | MFS Édition. |
| **Composants sous-jacents** | KindMother ; Opérateur MFS Visiteur (consommation côté visiteur). |

### 3.8 Kit « Publication catalogue » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **publication au catalogue** : annuaire des événements, répertoire des organisateurs, répertoire des exposants (selon politique plateforme). |
| **Outils agrégés (exemples)** | `catalogue.edition.publish`, `catalogue.organiser.profile.get.public`, `catalogue.exposant.list.public` (par édition). |
| **Consommé par** | MFS Organisateur, MFS Édition ; Opérateur Catalogue (Façade publique). |
| **Composants sous-jacents** | KindMother, Border Guard (règles d’exposition). |

### 3.9 Kit « Équipe & Permissions » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’**équipe organisateur** : invitation, rôles (Admin, Manager, Bénévole), attribution par édition. |
| **Outils agrégés (exemples)** | `team.invite`, `team.list`, `team.role.set`, `team.edition.assign`, `permission.check`. |
| **Consommé par** | MFS Organisateur. |
| **Composants sous-jacents** | Miyauth, Master Butler, Miyuprofile. |

### 3.10 Synthèse des Toolkits (public Organisateurs)

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|---------------------|
| **Éditions** | MFS Organisateur, MFS Édition | Création, paramétrage, liste éditions, publication. |
| **Exposants (côté organisateur)** | MFS Édition | Candidatures, validation, refus, fiches, facturation. |
| **Plan de salle** | MFS Édition | Zones, stands, attribution, export. |
| **Programme** | MFS Édition | Animations, salles, horaires, vues. |
| **Budget** | MFS Édition | Revenus, dépenses, ventilation, balance. |
| **Documents & Légal** | MFS Édition | Contrats, CGV, conventions, partage. |
| **Services visiteur** | MFS Édition | Activation, paramétrage (jeux, concours, ateliers, pass). |
| **Publication catalogue** | MFS Organisateur, MFS Édition | Annuaire événements, répertoires. |
| **Équipe & Permissions** | MFS Organisateur | Équipe, rôles, éditions assignées. |

---

## 4. Matrice Parcours / Livrables / Opérateurs / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) |
|----------------------|-----------|------------|
| Onboarding (inscription organisateur) | Miyauth, Miyuprofile ; MFS Organisateur | Équipe & Permissions, Éditions. |
| Liste des éditions (multi-festivals) | MFS Organisateur | Éditions. |
| Dashboard par édition | MFS Édition | Éditions, Exposants, Plan de salle, Programme, Budget, Documents, Services visiteur. |
| Candidatures exposants (validation, refus) | MFS Édition | Exposants (côté organisateur). |
| Plan de salle (zones, attribution) | MFS Édition | Plan de salle. |
| Programme (animations, horaires) | MFS Édition | Programme. |
| Budget (revenus, dépenses) | MFS Édition | Budget. |
| Documents (contrats, CGV) | MFS Édition | Documents & Légal. |
| Services visiteur (activation) | MFS Édition | Services visiteur. |
| Publication catalogue | MFS Organisateur, MFS Édition | Publication catalogue. |
| Équipe (rôles, bénévoles) | MFS Organisateur | Équipe & Permissions. |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Authentification, rôles | Miyauth, Master Butler | Compte organisateur, Mandat, permissions (Admin, Manager, Bénévole). |
| Profil organisateur | Miyuprofile | Structure, contact, charte. |
| Notifications | Miyunotify | Annonces, notifications ciblées (exposants, équipe). |
| Facturation (exposants) | Miyuinvoice | Devis, factures, suivi paiements. |
| Persistance | KindMother | Éditions, candidatures, plan de salle, programme, budget, documents. |
| Sécurité, audit | WorrySentinel, traçabilité | Niveaux de sécurité, audit. |
| Catalogue (macro) | Border Guard, Mandat public | Exposition annuaire, répertoires. |

---

## 6. Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
- [Organisateurs — Parcours, capacités et livrables](./Organisateurs%20-%20Parcours%20Capacites%20Livrables.md)
- [Public Exposants](../Exposants/_index.md) | [Public Visiteurs](../Visiteurs/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
- Glossaire Miyukini (Opérateur, Outil, Kit d’Outils, Mandat de Permission)

---

**Document** : Organisateurs — Besoins en Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence produit pour l’architecture gouvernée (Miyukini Festival Service)
