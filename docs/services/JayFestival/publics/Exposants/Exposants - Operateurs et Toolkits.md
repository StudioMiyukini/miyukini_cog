# Exposants — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du public **Exposants** du service JayFestival. Il s’appuie sur le [Parcours, capacités et dashboard](./Exposants%20-%20Parcours%20Capacites%20Dashboard.md) et les documents associés. Il vise à fournir une référence produit pour l’architecture gouvernée : quels Opérateurs exposent les capacités aux exposants, et quels Kits d’outils agrègent les outils sous-jacents.

## Portée / Scope

- **Public** : Exposants (professionnels ou structures participant à des événements/festivals en tant qu’exposants).
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir les livrables du public (dashboard exposant, candidatures, participations, agenda cross-événements, documents, factures, répertoire exposants).
- **Hors périmètre** : Spécifications d’implémentation (API, schémas, code) ; définition détaillée des Cores — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |

Les exposants **interagissent avec** des Opérateurs gouvernés (dashboard exposant, candidatures, participations, agenda, documents, factures) ; ces Opérateurs s’appuient sur des Toolkits et des composants Miyukini (Miyauth, Miyuprofile, Miyunotify, Miyuinvoice, MiyuClock, Miyubooking, etc.).

---

## 2. Besoins en Opérateurs (public Exposants)

### 2.1 Opérateur « MFS Exposant » (dashboard exposant)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **dashboard exposant** : vue unifiée sur toutes les candidatures, participations, agenda cross-événements, documents et factures. |
| **Public servi** | Exposants authentifiés (rôle exposant, Master Butler). |
| **Gouvernance** | Mandat de Permission (StrongFather) pour accéder aux données de l’exposant ; permissions (Master Butler) ; persistance (KindMother) ; sécurité (WorrySentinel). |
| **Capacités exposées** | Liste des candidatures (en attente, validées, refusées) par édition ; liste des participations (éditions validées) ; agenda cross-événements (dates des événements candidat ou inscrit, alerte conflits de dates) ; documents (consultation, téléchargement, envoi) ; factures (Miyuinvoice : consultation, téléchargement PDF, statut de paiement). |
| **Ne fait pas** | Décision de validation des candidatures (organisateur) ; émission des factures (organisateur via Miyuinvoice). |

Cet Opérateur est le **point d’entrée** principal de l’exposant : il agrège les capacités métier (candidatures, participations, agenda, documents, factures) et s’appuie sur les Toolkits listés en § 3.

### 2.2 Opérateur « JayFestival Candidatures » (dépôt et suivi candidatures)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **dépôt et le suivi des candidatures** : formulaire de candidature par édition (champs définis par l’organisateur), pièces jointes, statuts (en attente, validée, refusée), vérification agenda (conflit de dates). |
| **Public servi** | Exposants authentifiés. |
| **Gouvernance** | Mandat de Permission ; validation côté organisateur (StrongFather) ; notification (Miyunotify) selon paramétrage organisateur. |
| **Capacités exposées** | Création d’une candidature pour une édition ; saisie des champs demandés par l’organisateur ; upload de pièces jointes ; consultation du statut ; alerte si conflit de dates avec une autre édition à laquelle l’exposant est déjà inscrit ou candidat. |
| **Lien avec MFS Exposant** | MFS Exposant **consomme** les données candidatures (liste, détail) ; JayFestival Candidatures peut être un **sous-capacité** de MFS Exposant ou un Opérateur distinct selon l’architecture. |

### 2.3 Synthèse des Opérateurs (public Exposants)

| Opérateur | Usage par l’exposant | Livrables couverts |
|-----------|----------------------|---------------------|
| **MFS Exposant** | Connexion, dashboard (candidatures, participations, agenda, documents, factures). | Dashboard exposant, vue unifiée, agenda cross-événements, conflits de dates. |
| **JayFestival Candidatures** | Dépôt candidature, suivi statut, pièces jointes, vérification agenda. | Candidatures, participations (accès après validation), documents, factures. |

*Note :* Selon l’architecture, JayFestival Candidatures peut être **intégré** dans MFS Exposant (un seul Opérateur « MFS Exposant » avec capacités candidatures, participations, agenda, documents, factures) ou **séparé** pour une délégation de responsabilité plus fine.

---

## 3. Besoins en Toolkits (public Exposants)

### 3.1 Kit « Candidatures Exposant » (JayFestival)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer le dépôt et le suivi des candidatures : formulaire par édition, champs dynamiques (définis par l’organisateur), pièces jointes, statuts, notification. |
| **Outils agrégés (exemples)** | `application.form.get` (champs du formulaire par édition), `application.create` (dépôt candidature), `application.upload` (pièces jointes), `application.list.byExposant` (liste des candidatures de l’exposant), `application.get` (détail, statut), `application.status` (en attente, validée, refusée). |
| **Consommé par** | MFS Exposant, JayFestival Candidatures. |
| **Composants sous-jacents** | KindMother (persistance candidatures), Miyunotify (notifications). |

### 3.2 Kit « Participations & Éditions » (JayFestival)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’accès aux **éditions validées** (participations) : documents de l’édition, emplacement (stand), programme, facturation. |
| **Outils agrégés (exemples)** | `edition.list.byExposant` (éditions auxquelles l’exposant participe), `edition.get` (détail édition), `edition.documents.list`, `edition.stand.get` (emplacement attribué), `edition.programme.get` (programme public). |
| **Consommé par** | MFS Exposant. |
| **Composants sous-jacents** | KindMother (données édition, participations). |

### 3.3 Kit « Agenda cross-événements » (JayFestival)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **gestion d’agenda** cross-événements : visualisation des dates des événements (candidat ou inscrit), détection des conflits de dates (chevauchement), alerte ou blocage avant validation d’une nouvelle candidature. |
| **Outils agrégés (exemples)** | `agenda.dates.get` (dates des événements auxquels l’exposant est candidat ou inscrit), `agenda.conflict.check` (chevauchement avec une autre édition), `agenda.visualize` (calendrier cross-événements). |
| **Consommé par** | MFS Exposant, JayFestival Candidatures (vérification avant dépôt). |
| **Composants sous-jacents** | MiyuClock, Miyubooking (ou équivalent pour plages de dates), données d’édition. |

### 3.4 Kit « Documents Exposant » (JayFestival)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **consultation et l’envoi des documents** par édition : contrats, règlements, conventions ; téléchargement ; envoi des documents signés ou complétés selon workflow organisateur. |
| **Outils agrégés (exemples)** | `document.list.byEdition` (documents reçus ou à renvoyer), `document.download`, `document.upload` (document signé ou complété), `document.status` (reçu, à renvoyer, validé). |
| **Consommé par** | MFS Exposant. |
| **Composants sous-jacents** | KindMother (stockage documents), Miyunotify (notifications). |

### 3.5 Kit « Facturation Exposant » (MFS / Miyuinvoice)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **consultation des devis et factures** par édition : téléchargement PDF, suivi du statut de paiement (payé / en attente). |
| **Outils agrégés (exemples)** | `invoice.list.byEdition` (devis et factures par édition), `invoice.download` (PDF), `invoice.status` (payé, en attente). |
| **Consommé par** | MFS Exposant. |
| **Composants sous-jacents** | Miyuinvoice. |

### 3.6 Kit « Répertoire Exposants » (JayFestival)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **publication de la fiche exposant** dans le répertoire des exposants du catalogue (annuaire) : visibilité selon politique plateforme et choix de l’organisateur. |
| **Outils agrégés (exemples)** | `exposant.profile.get.public` (fiche exposant pour le catalogue), `exposant.visibility.set` (paramètre de visibilité), `exposant.list.public` (liste des exposants pour le catalogue). |
| **Consommé par** | MFS Exposant (paramètre visibilité) ; Opérateur Catalogue (Façade publique) pour l’affichage. |
| **Composants sous-jacents** | Miyuprofile (fiche entreprise/contact), KindMother. |

### 3.7 Synthèse des Toolkits (public Exposants)

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|---------------------|
| **Candidatures Exposant** | MFS Exposant, JayFestival Candidatures | Candidatures (dépôt, suivi, pièces jointes, statuts). |
| **Participations & Éditions** | MFS Exposant | Participations (éditions validées), documents, emplacement, programme. |
| **Agenda cross-événements** | MFS Exposant, JayFestival Candidatures | Agenda, conflits de dates. |
| **Documents Exposant** | MFS Exposant | Documents (contrats, règlements, conventions). |
| **Facturation Exposant** | MFS Exposant | Devis et factures (Miyuinvoice). |
| **Répertoire Exposants** | MFS Exposant, Catalogue | Répertoire des exposants (fiche publique). |

---

## 4. Matrice Parcours / Livrables / Opérateurs / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) |
|----------------------|-----------|------------|
| Onboarding (inscription exposant) | Miyauth, Miyuprofile ; MFS Exposant (premier accès) | Équipe & Permissions (Miyauth), Répertoire Exposants (fiche). |
| Dashboard (vue d’ensemble) | MFS Exposant | Candidatures, Participations & Éditions, Agenda cross-événements, Documents, Facturation. |
| Dépôt candidature | JayFestival Candidatures / MFS Exposant | Candidatures Exposant, Agenda cross-événements (conflit check). |
| Suivi candidature, participation | MFS Exposant | Candidatures Exposant, Participations & Éditions. |
| Agenda, conflits de dates | MFS Exposant | Agenda cross-événements. |
| Documents (consultation, envoi) | MFS Exposant | Documents Exposant. |
| Factures (consultation, téléchargement) | MFS Exposant | Facturation Exposant. |
| Répertoire exposants (visibilité) | MFS Exposant | Répertoire Exposants. |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Authentification, rôles | Miyauth, Master Butler | Compte exposant, Mandat, permissions. |
| Profil exposant (entreprise, contact) | Miyuprofile | Fiche entreprise, contact, éditions participées. |
| Notifications | Miyunotify | Statut candidature, documents, factures. |
| Facturation | Miyuinvoice | Devis, factures, statut de paiement. |
| Agenda, dates | MiyuClock, Miyubooking (ou équivalent) | Plages de dates, conflits. |
| Persistance | KindMother | Candidatures, participations, documents. |
| Sécurité, audit | WorrySentinel, traçabilité | Niveaux de sécurité, audit. |

---

## 6. Références

- [Document fondateur JayFestival](../../JayFestival%20-%20Document%20Fondateur.md)
- [Exposants — Parcours, capacités et dashboard](./Exposants%20-%20Parcours%20Capacites%20Dashboard.md)
- [Exposants — Écrans et cycle](./Exposants%20-%20Ecrans%20et%20cycle.md)
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Visiteurs](../Visiteurs/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
- Glossaire Miyukini (Opérateur, Outil, Kit d’Outils, Mandat de Permission)

---

**Document** : Exposants — Besoins en Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence produit pour l’architecture gouvernée (JayFestival)
