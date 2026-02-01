# Visiteurs — Parcours, capacités et services

## Contexte

Ce document détaille le **parcours**, les **capacités** et les **services** du public cible **Visiteurs** dans le cadre du service Miyukini Festival Service. Il complète le [document fondateur](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md).

## Portée / Scope

- **Public** : Visiteurs (personnes qui fréquentent les événements/festivals : public, participants ateliers, jeux, concours, etc.).
- **Périmètre** : onboarding (festival / groupe de festivals), espace dédié, compte cross-événement, agenda, services activables par l’organisateur, limites.
- **Hors périmètre** : spécifications techniques d’implémentation (Opérateurs, Kits, API).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Public des festivals/événements : visiteurs occasionnels ou réguliers, participants aux ateliers, jeux, concours, réservations, pass VIP. |
| **Compte** | Cross-événements : un même visiteur peut suivre ses activités sur **plusieurs événements**. |
| **Accès** | Authentification (Miyauth), permissions (Master Butler), rôle visiteur. |
| **Espace** | **Espace dédié visiteur** : agenda, billets, réservations, pass VIP, suivi d’activités (jeux, concours, ateliers). |

---

## 2. Parcours utilisateur

### 2.1 Onboarding : par festival ou par groupe de festivals

| Mode | Description |
|------|-------------|
| **Onboarding par festival** | Le visiteur crée un compte ou se connecte dans le contexte d’un **seul événement** ; il peut ensuite étendre son compte à d’autres événements (compte cross-événement). |
| **Onboarding par groupe de festivals** | L’organisateur ou la plateforme propose une **famille d’événements** (ex. « Festivals partenaires 2026 ») ; le visiteur s’inscrit **une fois** et accède à **tous les événements du groupe** avec le même compte, agenda et billets unifiés. |

Cela permet des partenariats entre organisateurs (groupes de festivals) et une expérience visiteur cohérente sur plusieurs événements.

### 2.2 Parcours type (cycle de vie)

| Étape | Action | Résultat |
|-------|--------|----------|
| **Découverte** | Consultation du catalogue (annuaire des événements) en [utilisateur non connecté](../UtilisateurNonConnecte/_index.md) ou connecté. | Liste des événements, fiches événement. |
| **Inscription / Connexion** | Création de compte ou connexion (onboarding par festival ou groupe de festivals). | Accès à l’**espace dédié visiteur**. |
| **Exploration** | Consultation des événements auxquels le visiteur est inscrit ou éligible ; consultation des services activés (jeux, concours, ateliers, réservations, pass). | Choix des activités. |
| **Réservation / Inscription** | Réservation d’ateliers, inscription à des concours, achat de billets ou pass VIP (selon ce que l’organisateur propose). | **Vérification agenda** (conflit de dates ?). |
| **Organisation** | Consultation de l’**agenda** personnel (ateliers, animations, créneaux réservés) ; compte à rebours ; billets et pass. | Visite organisée sur un ou plusieurs événements. |
| **Participation** | Participation aux jeux, concours, ateliers ; suivi des récompenses et de l’historique. | **Suivi d’activités** dans l’espace visiteur. |
| **Clôture** | Fin de l’événement ; archivage des billets, réservations et participations dans le compte cross-événement. | Historique conservé ; possibilité de s’inscrire à d’autres événements. |

### 2.3 Gestion d’agenda et conflits de dates

- **Problématique** : un visiteur ne doit pas s’inscrire à **deux événements ou deux créneaux à la même date** (même besoin que pour les exposants — « déjà vu »).
- **Solution** : **gestion d’agenda** (calendrier cross-événements) :
  - Visualisation des dates des événements et créneaux auxquels le visiteur est inscrit ou a réservé.
  - **Alerte ou blocage** en cas de chevauchement avant validation d’une nouvelle réservation ou inscription.

La gouvernance (StrongFather, Master Butler, KindMother) garantit que les données du visiteur restent souveraines et que l’accès cross-événement respecte les Mandats et les choix de chaque organisateur.

### 2.4 Points de sortie / passerelles

- **Vers organisateurs** : les services visiteurs (jeux, concours, ateliers, pass) sont **activés et paramétrés** par les organisateurs ; le visiteur consomme ces services sans modifier les paramètres des éditions.
- **Vers catalogue** : le visiteur peut consulter l’annuaire des événements et le répertoire des organisateurs et exposants (comme un [utilisateur non connecté](../UtilisateurNonConnecte/_index.md)) ; une fois connecté, il accède en plus à son espace dédié.

---

## 3. Compte cross-événement : capacités et livrables

### 3.1 Vue unifiée (plusieurs événements)

| Capacité | Description |
|----------|-------------|
| **Agenda** | Programme personnel : ateliers, animations, concours auxquels le visiteur est inscrit ou qu’il souhaite suivre, **synchronisé entre événements**. |
| **Compte à rebours** | Jours/heures restants avant les événements ou créneaux réservés. |
| **Billets / tickets** | Accès centralisé aux billets et tickets acquis (par événement ou groupe d’événements). |
| **Réservations** | Ateliers, créneaux, places réservés ; annulation ou modification dans le cadre des règles de l’édition. |
| **Pass VIP** | Pass et avantages associés (par événement ou multi-événements), selon ce que l’organisateur met en place. |
| **Suivi d’activités** | Historique et suivi des participations : jeux joués, concours, ateliers suivis, récompenses, etc. |

### 3.2 Services proposés aux visiteurs (activables par l’organisateur)

Chaque **organisateur** peut **mettre à disposition** pour ses événements tout ou partie des services suivants. La plateforme les fournit ; l’organisateur **choisit lesquels activer** et avec quelles règles :

| Service | Description |
|---------|-------------|
| **Jeux** | Jeux liés au festival (quizz, chasses au trésor, défis) ; participation et suivi dans l’espace visiteur. |
| **Concours** | Inscription et participation à des concours ; résultats, récompenses, historique. |
| **Inscriptions ateliers** | Réservation de créneaux d’ateliers ; annulation, rappels, intégration à l’agenda visiteur. |
| **Réservations** | Réservation de places, créneaux ou activités (Miyubooking) ; billets et pass. |
| **Pass et avantages** | Pass VIP, pass journée, avantages fidélité ; liaison avec le compte cross-événement. |
| **Notifications** | Rappels, changements de programme, alertes (Miyunotify), selon préférences visiteur et règles édition. |

L’organisateur configure, par édition ou par groupe d’éditions, quels services sont ouverts et selon quelles conditions (places limitées, dates, publics). La **distribution** visiteur est ainsi **paramétrable** par organisateur sans modifier la gouvernance plateforme.

---

## 4. Limites et gouvernance

| Aspect | Règle |
|--------|--------|
| **Services** | Le visiteur ne peut accéder qu’aux services **activés** par l’organisateur pour chaque édition ; pas de création de jeux ou concours par le visiteur. |
| **Agenda** | La plateforme signale ou bloque les conflits de dates ; le visiteur reste responsable de la cohérence de son planning. |
| **Données** | Les données visiteur (profil, agenda, billets, participations) sont souveraines et protégées ; accès restreint selon Mandat et rôle (Master Butler). |
| **Billets et pass** | Émis ou gérés selon les règles de l’organisateur ; le visiteur consulte et utilise dans le cadre de l’édition. |

---

## 5. Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md) — § 6 Distribution visiteurs
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Exposants](../Exposants/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
