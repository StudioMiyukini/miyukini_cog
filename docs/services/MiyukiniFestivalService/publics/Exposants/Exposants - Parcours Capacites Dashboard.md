# Exposants — Parcours, capacités et dashboard

## Contexte

Ce document détaille le **parcours**, les **capacités** et le **dashboard** du public cible **Exposants** dans le cadre du service Miyukini Festival Service. Il complète le [document fondateur](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md).

## Portée / Scope

- **Public** : Exposants (professionnels ou structures participant à des événements en tant qu’exposants).
- **Périmètre** : onboarding, dashboard dédié, candidatures, participations, agenda, conflits de dates, limites.
- **Hors périmètre** : spécifications techniques d’implémentation (Opérateurs, Kits, API).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Professionnels, entreprises, associations participant à des festivals/événements avec un stand ou une présence exposant. |
| **Compte** | Cross-événements : un même exposant peut **participer à plusieurs festivals**. |
| **Accès** | Authentification (Miyauth), permissions (Master Butler), rôle exposant. |
| **Espace** | **Dashboard exposant dédié** : vue unifiée sur toutes ses candidatures, participations, documents, factures et **agenda**. |

---

## 2. Parcours utilisateur

### 2.1 Onboarding

1. **Création de compte** : inscription en tant qu’exposant (Miyauth, Miyuprofile, fiche entreprise/contact).
2. **Validation** : selon politique plateforme ou selon validation par l’organisateur pour une édition donnée.
3. **Attribution des permissions** : rôle exposant (Master Butler).
4. **Première candidature** : dépôt d’une candidature pour une édition (festival) ; l’exposant peut ensuite en déposer d’autres pour d’autres éditions.

Le compte est **cross-événements** dès l’origine : l’exposant peut candidater et participer à autant de festivals que souhaité, sous réserve des règles d’agenda (conflits de dates).

### 2.2 Parcours type (cycle de vie)

| Étape | Action | Résultat |
|-------|--------|----------|
| **Connexion** | Connexion avec identifiants exposant. | Accès au **dashboard exposant**. |
| **Vue d’ensemble** | Consultation du dashboard : candidatures, participations, agenda, documents, factures. | Vue unifiée sur **tous les festivals** concernés. |
| **Découverte** | Consultation de l’annuaire des événements (catalogue). | Liste des festivals ouverts aux candidatures. |
| **Candidature** | Dépôt d’une candidature pour une édition ; saisie des informations demandées par l’organisateur. | Candidature en attente ; **vérification agenda** (conflit de dates ?). |
| **Suivi** | Consultation du statut (en attente, validée, refusée) ; réception des documents, devis, factures. | Suivi par édition. |
| **Participation** | Une fois validé : accès aux documents de l’édition, emplacement, programme, facturation (Miyuinvoice). | Participation active à l’édition. |
| **Clôture** | Fin de l’édition : archivage des documents et factures dans le dashboard. | Historique conservé ; possibilité de candidater à d’autres éditions. |

### 2.3 Gestion d’agenda et conflits de dates

- **Problématique** : un exposant ne doit pas s’inscrire à **deux événements à la même date** (besoin déjà rencontré en pratique — « déjà vu »).
- **Solution** : **gestion d’agenda** (calendrier cross-événements) :
  - Visualisation des dates des événements auxquels l’exposant est inscrit ou candidat.
  - **Alerte ou blocage** en cas de chevauchement de dates avant validation d’une nouvelle candidature.
  - L’exposant peut organiser son planning sur plusieurs festivals sans double engagement.

Cette capacité relève de l’Opérateur ou Kit **Agenda cross-événements** (MiyuClock, Miyubooking, données d’édition).

### 2.4 Points de sortie / passerelles

- **Vers organisateurs** : les candidatures et participations sont gérées par les organisateurs de chaque édition ; l’exposant ne modifie pas les paramètres des éditions.
- **Vers catalogue** : la fiche exposant peut apparaître dans le **répertoire des exposants** (selon politique plateforme), visible par [utilisateur non connecté](../UtilisateurNonConnecte/_index.md) et tous les publics.
- **Vers visiteurs** : un exposant peut aussi être visiteur sur d’autres événements (compte distinct ou même personne avec deux rôles selon modèle plateforme).

---

## 3. Dashboard exposant : capacités et livrables

### 3.1 Vue d’ensemble

| Bloc | Contenu |
|------|---------|
| **Candidatures** | Liste des candidatures (en attente, validées, refusées) par édition ; accès au détail et aux pièces jointes. |
| **Participations** | Liste des éditions auxquelles l’exposant participe (validé) ; accès aux documents, emplacement, programme de l’édition. |
| **Agenda** | Calendrier cross-événements : dates des événements (candidat ou inscrit) ; alerte conflits de dates. |
| **Documents** | Documents reçus ou à renvoyer par édition (contrats, règlements, conventions). |
| **Factures** | Devis et factures (Miyuinvoice) par édition ; statut de paiement, téléchargement. |

### 3.2 Candidatures

- **Dépôt** : formulaire de candidature par édition (champs définis par l’organisateur).
- **Pièces jointes** : upload de documents (fiche entreprise, logo, etc.).
- **Statuts** : en attente, validée, refusée ; notification (Miyunotify) selon paramétrage organisateur.
- **Vérification agenda** : avant validation côté organisateur ou à la soumission, la plateforme peut signaler un conflit de dates avec une autre édition à laquelle l’exposant est déjà inscrit ou candidat.

### 3.3 Participations (éditions validées)

- **Fiche par édition** : résumé (dates, lieu, statut), lien vers les documents et la facturation.
- **Emplacement** : stand ou zone attribué (lien vers plan de salle si exposé par l’organisateur).
- **Programme** : accès au programme public de l’édition si mis à disposition.

### 3.4 Documents et facturation

- **Documents** : consultation et téléchargement des contrats, règlements, conventions ; envoi des documents signés ou complétés selon workflow organisateur.
- **Devis et factures** (Miyuinvoice) : consultation, téléchargement PDF, suivi du statut de paiement (payé / en attente).

### 3.5 Répertoire des exposants

- **Visibilité** : la fiche exposant (entreprise, contact, éditions participées, etc.) peut être publiée dans le **répertoire des exposants** du catalogue (annuaire), selon la politique plateforme et les choix de l’organisateur.
- **Bénéfice** : mise en visibilité pour les visiteurs et les autres organisateurs.

---

## 4. Limites et gouvernance

| Aspect | Règle |
|--------|--------|
| **Candidatures** | L’exposant ne peut pas modifier les paramètres des éditions ; il dépose une candidature et attend la décision de l’organisateur (StrongFather, validation). |
| **Agenda** | La plateforme signale ou bloque les conflits de dates ; l’exposant reste responsable de la cohérence de son planning. |
| **Données** | Les données exposant (fiche, candidatures, factures) sont souveraines et protégées ; accès restreint selon Mandat et rôle (Master Butler). |
| **Facturation** | Émise par l’organisateur via Miyuinvoice ; l’exposant consulte et paie selon les modalités de l’édition. |

---

## 5. Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md) — § 5 Distribution exposants
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Visiteurs](../Visiteurs/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
