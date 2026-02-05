# Visiteurs — Écrans et cycle

## Contexte

Ce document précise **tous les écrans** du cycle utilisateur du public **Visiteurs** pour le service JayFestival, avec l’**organisation** (structure, blocs, zones) et les **besoins** associés à chaque écran. Il s’appuie sur le [parcours et les services](./Visiteurs%20-%20Parcours%20Capacites%20Services.md) et sur l’[analyse des besoins](./Visiteurs%20-%20Analyse%20des%20besoins.md).

## Portée / Scope

- **Public** : Visiteurs (personnes qui fréquentent les événements/festivals : public, participants ateliers, jeux, concours, réservations, pass VIP).
- **Périmètre** : tous les écrans du cycle (de l’arrivée à la clôture), organisation et besoins par écran.
- **Hors périmètre** : maquettes graphiques, spécifications API.

---

## 1. Vue d’ensemble du cycle

Le cycle visiteur se décompose en **phases** :

| Phase | Description | Écrans concernés |
|-------|-------------|-------------------|
| **Accès** | Arrivée depuis le catalogue ; connexion ou inscription (par festival ou groupe de festivals). | Landing (catalogue), Connexion, Inscription visiteur (festival / groupe) |
| **Espace visiteur** | Vue d’ensemble : agenda, billets, réservations, pass VIP, activités. | Page d’accueil espace visiteur, Navigation par événement |
| **Agenda** | Calendrier personnel, compte à rebours, conflits, export. | Agenda personnel, Export agenda |
| **Billets et réservations** | Billets/tickets, réservation ateliers/créneaux, pass VIP. | Billets et tickets, Réservations, Pass VIP, Réservation depuis fiche événement |
| **Jeux et concours** | Participation, suivi scores, résultats, historique. | Jeux (liste et participation), Concours (liste et participation), Historique participations |
| **Catalogue (connecté)** | Découverte d’événements, services activés, achat/réservation. | Fiche événement (connecté), Services activés |
| **Compte et notifications** | Profil, préférences notifications. | Mon compte, Préférences notifications |

---

## 2. Écrans du cycle — détail

### 2.1 Accès

#### VIS-E01 — Landing / Accueil catalogue (passerelle)

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Point d’entrée depuis le catalogue (utilisateur non connecté) ; afficher les CTAs « S’inscrire » et « Se connecter ». |
| **Organisation** | En-tête : logo, lien Événements / Organisateurs / Exposants, bouton **Se connecter**, bouton **S’inscrire** (ou menu : Organisateur / Exposant / Visiteur). Zone principale : accroche + lien vers **S’inscrire en tant que visiteur**. Pied : liens légaux. |
| **Besoins** | UNC-02, UNC-19 (passerelle depuis [Utilisateur non connecté](../UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md)). |
| **Navigation** | Entrée : URL catalogue. Sortie : Connexion (VIS-E02), Inscription visiteur (VIS-E03). |

#### VIS-E02 — Connexion

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Permettre à un visiteur de se connecter (email + mot de passe ou lien magique). |
| **Organisation** | Titre « Se connecter ». Champs : email, mot de passe. Liens : « Mot de passe oublié », « S’inscrire ». Bouton **Se connecter**. Message d’erreur si échec. Retour au contexte (fiche événement, etc.) après connexion si applicable. |
| **Besoins** | VIS-01, Miyauth. |
| **Navigation** | Entrée : Landing, fiche événement (CTA Réserver). Sortie : Page d’accueil espace visiteur (VIS-E04) ou fiche événement (contexte). |

#### VIS-E03 — Inscription visiteur (par festival ou groupe)

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Création du compte visiteur ; onboarding par festival (contexte d’un événement) ou par groupe de festivals. |
| **Organisation** | **Contexte** : depuis fiche événement (inscription pour cet événement) ou depuis page « Groupe de festivals » (inscription pour le groupe). Titre « Créer un compte visiteur » ou « S’inscrire — [Nom événement/groupe] ». Bloc 1 : email, mot de passe, confirmation. Bloc 2 : nom, prénom, préférences optionnelles. CGU + case à cocher. Bouton **S’inscrire**. Lien « Déjà un compte ? Se connecter ». Après inscription : accès espace visiteur (événement ou groupe pré-associé). |
| **Besoins** | VIS-01, VIS-02, VIS-03, VIS-04. |
| **Navigation** | Entrée : Landing, fiche événement (CTA Réserver / S’inscrire), page groupe festivals. Sortie : Page d’accueil espace visiteur (VIS-E04). |

---

### 2.2 Espace visiteur (accueil et navigation)

#### VIS-E04 — Page d’accueil espace visiteur

| Attribut | Description |
|----------|-------------|
| **Phase** | Espace visiteur |
| **Objectif** | Vue d’ensemble : agenda, billets, réservations, pass VIP, suivi d’activités ; indicateurs (prochain événement, compte à rebours). |
| **Organisation** | En-tête : nom utilisateur ou « Mon espace », menu (Agenda, Billets, Réservations, Pass VIP, Activités, Catalogue, Mon compte, Déconnexion). Zone principale : **Bloc synthèse** (« Prochain événement : [Nom], dans X jours »). **Blocs ou onglets** : **Agenda** (aperçu calendrier ou prochains créneaux), **Billets** (liste raccourcie), **Réservations** (en cours), **Pass VIP** (liste raccourcie), **Activités** (dernières participations jeux/concours). Filtre ou onglet **Par événement** (VIS-06). |
| **Besoins** | VIS-04, VIS-05, VIS-06, VIS-08. |
| **Navigation** | Entrée : après Connexion ou Inscription. Sortie : Agenda (VIS-E05), Billets (VIS-E06), Réservations (VIS-E07), Pass VIP (VIS-E08), Activités (VIS-E11), Catalogue (fiche événement), Mon compte (VIS-E14). |

---

### 2.3 Agenda

#### VIS-E05 — Agenda personnel

| Attribut | Description |
|----------|-------------|
| **Phase** | Agenda |
| **Objectif** | Consulter l’agenda personnel : ateliers réservés, créneaux, animations, concours ; synchronisé entre événements ; compte à rebours ; alerte conflits. |
| **Organisation** | Titre « Mon agenda ». Filtres : événement, type (atelier, animation, concours), jour. Vue **Calendrier** (mois, semaine) ou **Liste** : créneaux réservés, événements, ateliers, concours ; libellé et couleur par événement. **Compte à rebours** : « Dans X jours » / « Dans X heures » pour le prochain. **Alerte conflit** : message si chevauchement (VIS-09). Bouton **Export** (iCal, PDF) ou **Partager** (VIS-10). |
| **Besoins** | VIS-07, VIS-08, VIS-09, VIS-10. |
| **Navigation** | Entrée : Page d’accueil (VIS-E04). Sortie : Réservation (VIS-E09), Fiche événement (contexte). |

---

### 2.4 Billets, réservations et pass VIP

#### VIS-E06 — Billets et tickets

| Attribut | Description |
|----------|-------------|
| **Phase** | Billets |
| **Objectif** | Consulter et télécharger les billets et tickets acquis (par événement). |
| **Organisation** | Titre « Mes billets ». Filtre par événement. Liste : billet/ticket (type, date, lieu, événement). Actions : **Télécharger PDF**, **Afficher QR code** (mobile-friendly). Détail : type, date, lieu, conditions. |
| **Besoins** | VIS-11. |
| **Navigation** | Entrée : Page d’accueil (VIS-E04). Sortie : Page d’accueil, Fiche événement (acheter un autre billet). |

#### VIS-E07 — Réservations (ateliers, créneaux, places)

| Attribut | Description |
|----------|-------------|
| **Phase** | Réservations |
| **Objectif** | Consulter les réservations en cours ; annuler ou modifier selon règles de l’édition. |
| **Organisation** | Titre « Mes réservations ». Filtre par événement. Liste : réservation (atelier/créneau, date, lieu, événement, statut). Actions : **Voir** (détail), **Annuler** / **Modifier** (si autorisé, délai). Règles et conditions (VIS-20) affichées sur la fiche ou en lien. Intégration à l’agenda (lien vers VIS-E05). |
| **Besoins** | VIS-12, VIS-20. |
| **Navigation** | Entrée : Page d’accueil (VIS-E04). Sortie : Réservation (VIS-E09) pour modifier, Agenda (VIS-E05). |

#### VIS-E08 — Pass VIP et avantages

| Attribut | Description |
|----------|-------------|
| **Phase** | Pass VIP |
| **Objectif** | Consulter les pass VIP ou pass journée acquis ; avantages associés ; QR code ou justificatif. |
| **Organisation** | Titre « Mes pass ». Liste par événement : pass (type, événement(s), avantages). Fiche détail : type, événement(s), avantages, conditions ; **QR code** ou justificatif téléchargeable. |
| **Besoins** | VIS-13. |
| **Navigation** | Entrée : Page d’accueil (VIS-E04). Sortie : Page d’accueil, Fiche événement (acheter un pass). |

#### VIS-E09 — Réservation (flux : atelier, créneau, place ou pass)

| Attribut | Description |
|----------|-------------|
| **Phase** | Réservations |
| **Objectif** | Réserver un atelier, un créneau ou une place ; ou acheter un pass ; vérification agenda (conflit) avant validation. |
| **Organisation** | Titre « Réserver — [Nom événement] » ou « Acheter un pass ». Contexte : depuis fiche événement (VIS-E10) ou depuis espace visiteur. **Étape 1** : choix type (atelier, créneau, pass). **Étape 2** : sélection créneau ou type de pass ; affichage places restantes (VIS-19) ; file d’attente si complet. **Étape 3** : vérification agenda (alerte conflit VIS-09) ; confirmation. **Étape 4** : accusé de réception ; réservation ajoutée à l’agenda et à la liste Réservations/Billets. Règles et conditions (VIS-20) affichées ou en lien. |
| **Besoins** | VIS-12, VIS-14, VIS-09, VIS-19, VIS-20. |
| **Navigation** | Entrée : Fiche événement (VIS-E10), Page d’accueil (VIS-E04), Billets (VIS-E06). Sortie : Page d’accueil, Agenda (VIS-E05), Réservations (VIS-E07). |

---

### 2.5 Fiche événement (connecté) et services activés

#### VIS-E10 — Fiche événement (visiteur connecté)

| Attribut | Description |
|----------|-------------|
| **Phase** | Catalogue (connecté) |
| **Objectif** | Consulter la fiche événement avec les services activés ; accéder aux CTAs Réserver, Acheter pass, Participer (jeux, concours). |
| **Organisation** | Même structure que fiche événement [utilisateur non connecté](../UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md) : présentation, dates, lieu, organisateur, exposants, programme public. **Bloc Services activés** (VIS-18) : liste des services proposés (ateliers, réservations, pass, jeux, concours). CTAs : **Réserver un atelier**, **Acheter un pass**, **Participer au jeu [X]**, **S’inscrire au concours [Y]**. Clic → Réservation (VIS-E09), Jeu (VIS-E12), Concours (VIS-E13). |
| **Besoins** | VIS-14, VIS-18. |
| **Navigation** | Entrée : Catalogue (recherche, liste événements), Page d’accueil (lien Catalogue). Sortie : Réservation (VIS-E09), Jeux (VIS-E12), Concours (VIS-E13). |

---

### 2.6 Jeux et concours

#### VIS-E11 — Suivi d’activités (historique participations)

| Attribut | Description |
|----------|-------------|
| **Phase** | Activités |
| **Objectif** | Consulter l’historique des participations : jeux joués, concours, ateliers suivis, récompenses. |
| **Organisation** | Titre « Mes activités ». Filtres : événement, type (jeu, concours, atelier). Liste ou timeline : participation (type, événement, date, score ou résultat, récompense si applicable). Lien vers détail jeu/concours. |
| **Besoins** | VIS-17. |
| **Navigation** | Entrée : Page d’accueil (VIS-E04). Sortie : Jeu (VIS-E12), Concours (VIS-E13). |

#### VIS-E12 — Jeux (liste et participation)

| Attribut | Description |
|----------|-------------|
| **Phase** | Jeux |
| **Objectif** | Participer aux jeux proposés par le festival (quizz, chasses au trésor, défis) ; suivi des scores. |
| **Organisation** | Titre « Jeux — [Nom événement] » ou « Jeux ». Liste des jeux ouverts : nom, type, événement. Clic **Participer** → écran de jeu (questions quizz, étapes chasse au trésor, etc.). Enregistrement des réponses ; affichage du **score** ou du **classement** à la fin. Retour à la liste ou à l’historique (VIS-E11). |
| **Besoins** | VIS-15. |
| **Navigation** | Entrée : Fiche événement (VIS-E10), Page d’accueil (VIS-E04), Activités (VIS-E11). Sortie : Activités (VIS-E11), Fiche événement. |

#### VIS-E13 — Concours (liste et participation)

| Attribut | Description |
|----------|-------------|
| **Phase** | Concours |
| **Objectif** | S’inscrire et participer aux concours ; consulter les résultats et les récompenses. |
| **Organisation** | Titre « Concours — [Nom événement] » ou « Concours ». Liste des concours ouverts : nom, événement, date limite. Clic **S’inscrire** ou **Participer** → formulaire ou envoi (ex. photo, texte). Accusé de réception. **Résultats** : affichage selon règles organisateur (date de publication) ; récompenses listées. Notification si configurée (VIS-21). |
| **Besoins** | VIS-16. |
| **Navigation** | Entrée : Fiche événement (VIS-E10), Page d’accueil (VIS-E04), Activités (VIS-E11). Sortie : Activités (VIS-E11), Fiche événement. |

---

### 2.7 Compte et notifications

#### VIS-E14 — Mon compte

| Attribut | Description |
|----------|-------------|
| **Phase** | Compte |
| **Objectif** | Consulter et modifier le profil visiteur (nom, prénom, email, préférences). |
| **Organisation** | Titre « Mon compte ». Formulaire : nom, prénom, email, préférences optionnelles. Boutons **Enregistrer**, **Changer mot de passe** (si applicable). Lien **Préférences de notification** (VIS-E15). |
| **Besoins** | VIS-01 (profil). |
| **Navigation** | Entrée : Page d’accueil (VIS-E04). Sortie : Page d’accueil, Préférences notifications (VIS-E15). |

#### VIS-E15 — Préférences de notification

| Attribut | Description |
|----------|-------------|
| **Phase** | Notifications |
| **Objectif** | Configurer les préférences (types de notifications, canaux, fréquence). |
| **Organisation** | Titre « Préférences de notification ». Choix par type : rappels (réservation, événement), changements de programme, alertes, récompenses (jeux, concours). Choix canal : email, in-app. Option par événement si pertinent. Bouton **Enregistrer**. |
| **Besoins** | VIS-21, VIS-22. |
| **Navigation** | Entrée : Mon compte (VIS-E14), lien en-tête (icône paramètres). Sortie : Mon compte. |

---

## 3. Récapitulatif des écrans et besoins

| Écran | Id | Phase | Besoins principaux |
|-------|-----|-------|--------------------|
| Landing (passerelle) | VIS-E01 | Accès | UNC-02, UNC-19 |
| Connexion | VIS-E02 | Accès | VIS-01 |
| Inscription visiteur | VIS-E03 | Accès | VIS-01, VIS-02, VIS-03, VIS-04 |
| Page d’accueil espace visiteur | VIS-E04 | Espace visiteur | VIS-04, VIS-05, VIS-06, VIS-08 |
| Agenda personnel | VIS-E05 | Agenda | VIS-07, VIS-08, VIS-09, VIS-10 |
| Billets et tickets | VIS-E06 | Billets | VIS-11 |
| Réservations (liste) | VIS-E07 | Réservations | VIS-12, VIS-20 |
| Pass VIP | VIS-E08 | Pass VIP | VIS-13 |
| Réservation (flux) | VIS-E09 | Réservations | VIS-12, VIS-14, VIS-09, VIS-19, VIS-20 |
| Fiche événement (connecté) | VIS-E10 | Catalogue | VIS-14, VIS-18 |
| Suivi d’activités (historique) | VIS-E11 | Activités | VIS-17 |
| Jeux (liste et participation) | VIS-E12 | Jeux | VIS-15 |
| Concours (liste et participation) | VIS-E13 | Concours | VIS-16 |
| Mon compte | VIS-E14 | Compte | VIS-01 |
| Préférences de notification | VIS-E15 | Notifications | VIS-21, VIS-22 |

---

## 4. Navigation type (flux principal)

```
Landing (UNC) → Connexion / Inscription → Page d’accueil espace visiteur
       → Agenda (calendrier, compte à rebours, export)
       → Billets / Réservations / Pass VIP
       → Réservation (depuis fiche événement ou espace) [vérification conflit]
       → Fiche événement (catalogue connecté) → Réserver / Jeu / Concours
       → Jeux (participation, score)
       → Concours (inscription, participation, résultats)
       → Suivi d’activités (historique)
       → Mon compte / Préférences notifications
```

---

## 5. Références

- [Visiteurs — Parcours, capacités et services](./Visiteurs%20-%20Parcours%20Capacites%20Services.md)
- [Visiteurs — Analyse des besoins](./Visiteurs%20-%20Analyse%20des%20besoins.md)
- [Document fondateur JayFestival](../../JayFestival%20-%20Document%20Fondateur.md)
