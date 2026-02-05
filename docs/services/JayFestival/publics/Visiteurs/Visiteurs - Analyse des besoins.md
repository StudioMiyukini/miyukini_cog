# Visiteurs — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du public cible **Visiteurs** pour le service JayFestival. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels, les parcours détaillés, les user stories, les pain points et opportunités, ainsi que la priorisation et les dépendances. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur](../../JayFestival%20-%20Document%20Fondateur.md), [Parcours, capacités et services](./Visiteurs%20-%20Parcours%20Capacites%20Services.md).

## Portée / Scope

- **Public** : Visiteurs (personnes qui fréquentent les événements/festivals : public, participants ateliers, jeux, concours, réservations, pass VIP).
- **Périmètre** : tous les besoins identifiés pour ce public (fonctionnels, non fonctionnels, parcours, scénarios, priorisation).
- **Hors périmètre** : spécifications techniques d’implémentation (API, schémas de données détaillés), spécifications des autres publics (organisateurs, exposants, utilisateur non connecté) — traitées dans leurs propres documents d’analyse.

---

## 1. Profil du public et personas

### 1.1 Définition du public

Les **visiteurs** sont les personnes qui **fréquentent les événements/festivals** en tant que public : visiteurs occasionnels ou réguliers, participants aux ateliers, jeux, concours, réservations, pass VIP. Ils disposent d’un **espace dédié** et d’un **compte cross-événements** : un même visiteur peut suivre ses activités et organiser sa visite sur **plusieurs événements**. L’onboarding peut se faire **par festival** ou **par groupe de festivals**. Les organisateurs **activent** les services proposés aux visiteurs (jeux, concours, inscriptions ateliers, réservations, pass VIP) par édition ; le visiteur consomme ces services sans les créer.

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Visiteur occasionnel** | Va à 1 à 2 festivals par an ; découvre via le catalogue ; peu d’outils numériques. | Trouver les infos (dates, lieu, programme), réserver un atelier ou acheter un pass si proposé. | Infos dispersées, pas de rappel, risque d’oubli de réservation. |
| **Participant ateliers** | S’inscrit à des ateliers et créneaux ; plusieurs festivals par an ; besoin de planifier. | Réserver des créneaux, avoir un agenda personnel, éviter les doublons de dates. | Pas d’agenda unifié, risque de réserver deux ateliers au même moment. |
| **Joueur / concours** | Participe aux jeux et concours proposés par les festivals ; cherche les récompenses. | S’inscrire aux jeux et concours, suivre ses participations et récompenses. | Pas de suivi centralisé, perte des résultats. |
| **Visiteur multi-festivals** | Fréquente plusieurs festivals (saison, partenariats) ; besoin de cohérence. | Un seul compte pour tous les événements, agenda unifié, billets et pass centralisés. | Multiples comptes, pas de vue consolidée, risque de conflit de dates. |

### 1.3 Contexte d’usage

- **Fréquence** : connexion ponctuelle (réservation, inscription) ou régulière en phase de préparation (consultation agenda, billets).
- **Appareils** : mobile prioritaire pour la consultation et les réservations ; desktop pour la planification et l’agenda.
- **Concurrence** : sites par festival, emails, billeterie externe ; attente d’un **guichet unique** pour organiser sa visite sur plusieurs événements.

---

## 2. Besoins fonctionnels

### 2.1 Onboarding et compte

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| VIS-01 | Création de compte visiteur | Pouvoir s’inscrire en tant que visiteur (email, mot de passe ou lien magique, informations profil). | Formulaire d’inscription dédié ; validation email si configurée ; création du profil visiteur (Miyauth, Miyuprofile). |
| VIS-02 | Onboarding par festival | Créer un compte ou se connecter dans le contexte d’un **seul événement** ; étendre ensuite à d’autres événements. | Inscription depuis la fiche événement ; compte créé ; accès à l’espace visiteur pour cet événement ; possibilité d’ajouter d’autres événements (compte cross-événement). |
| VIS-03 | Onboarding par groupe de festivals | S’inscrire **une fois** pour une **famille d’événements** (ex. « Festivals partenaires 2026 ») et accéder à tous avec le même compte. | Inscription depuis la page groupe de festivals ; compte créé ; accès à tous les événements du groupe ; agenda et billets unifiés. |
| VIS-04 | Compte cross-événements | Un même visiteur peut suivre ses activités sur **plusieurs événements** sans recréer de compte. | Agenda, billets, réservations, pass VIP, suivi d’activités agrégés pour tous les événements concernés. |

### 2.2 Espace dédié visiteur — Vue d’ensemble

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| VIS-05 | Page d’accueil espace visiteur | Avoir une vue d’ensemble : agenda, billets, réservations, pass VIP, suivi d’activités. | Page d’accueil avec blocs ou onglets : Agenda, Billets, Réservations, Pass VIP, Activités ; indicateurs (ex. « Prochain événement : Festival X, dans 5 jours »). |
| VIS-06 | Navigation par événement | Filtrer ou naviguer par événement (agenda, billets, réservations par édition). | Filtre ou onglet par événement ; vue agrégée ou vue par événement selon choix. |

### 2.3 Agenda et organisation de la visite

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| VIS-07 | Agenda personnel | Consulter un agenda personnel : ateliers, animations, concours auxquels le visiteur est inscrit ou qu’il souhaite suivre, **synchronisé entre événements**. | Vue calendrier ou liste ; ateliers réservés, créneaux, animations « favoris » ou « à ne pas manquer » ; filtres (événement, type, jour). |
| VIS-08 | Compte à rebours | Voir les jours/heures restants avant les événements ou créneaux réservés. | Affichage « Dans X jours » ou « Dans X heures » pour le prochain événement ou créneau ; mise à jour en temps réel ou à l’actualisation. |
| VIS-09 | Gestion d’agenda et conflits de dates | Être alerté ou bloqué en cas de chevauchement (deux réservations ou deux événements à la même date). | Détection des chevauchements à la réservation ; alerte « Conflit avec [créneau/événement X] » ou blocage ; suggestion de créneaux libres. |
| VIS-10 | Export ou partage d’agenda | Exporter ou partager son agenda (calendrier) pour planification externe. | Export calendrier (iCal, PDF) ou lien de partage ; mise à jour automatique si nouvelles réservations. |

### 2.4 Billets, réservations et pass VIP

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| VIS-11 | Accès centralisé aux billets et tickets | Consulter et télécharger les billets et tickets acquis (par événement ou groupe d’événements). | Liste des billets/tickets par événement ; détail (type, date, lieu) ; téléchargement PDF ou QR code ; affichage mobile-friendly. |
| VIS-12 | Réservations (ateliers, créneaux, places) | Réserver des ateliers, créneaux ou places ; annuler ou modifier dans le cadre des règles de l’édition. | Formulaire ou sélection de créneaux ; confirmation ; annulation ou modification selon règles (délai, quota) ; intégration à l’agenda. |
| VIS-13 | Pass VIP et avantages | Consulter les pass VIP ou pass journée acquis ; voir les avantages associés (par événement ou multi-événements). | Fiche pass (type, événement(s), avantages) ; lien vers conditions ; affichage QR code ou justificatif si applicable. |
| VIS-14 | Achat ou réservation depuis la fiche événement | Acheter un billet, réserver un atelier ou un pass depuis la fiche événement (catalogue). | Lien « Réserver » / « Acheter » sur la fiche événement ; redirection vers le flux de réservation ou billetterie ; vérification agenda (conflit) avant validation. |

### 2.5 Suivi d’activités (jeux, concours, ateliers)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| VIS-15 | Participation aux jeux | Participer aux jeux proposés par le festival (quizz, chasses au trésor, défis) ; suivi des participations et des scores. | Accès aux jeux activés par l’organisateur ; enregistrement des participations ; affichage du score ou du classement si applicable. |
| VIS-16 | Inscription et participation aux concours | S’inscrire et participer aux concours ; consulter les résultats et les récompenses. | Liste des concours ouverts ; inscription ; envoi des réponses ou participation ; résultats et récompenses affichés selon règles organisateur. |
| VIS-17 | Historique des participations | Consulter l’historique des participations : jeux joués, concours, ateliers suivis, récompenses. | Liste ou timeline des participations par événement ; filtre par type (jeu, concours, atelier) ; récompenses gagnées. |

### 2.6 Services activables par l’organisateur

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| VIS-18 | Visibilité des services par événement | Voir quels services sont proposés pour chaque événement (jeux, concours, ateliers, réservations, pass). | Sur la fiche événement : liste des services activés ; lien vers chaque service (réservation, inscription jeu/concours). |
| VIS-19 | Places limitées et file d’attente | Réserver ou s’inscrire dans la limite des places ; être mis en file d’attente si complet (si proposé). | Affichage des places restantes ; blocage si complet ; option file d’attente si configurée ; notification si place libérée. |
| VIS-20 | Règles et conditions par édition | Consulter les règles et conditions (annulation, report) pour les réservations et pass par édition. | Affichage des règles sur la fiche réservation ou pass ; lien vers CGV ou règlement si applicable. |

### 2.7 Notifications et communication

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| VIS-21 | Réception des notifications | Recevoir des notifications (rappel réservation, changement de programme, alerte, récompense). | Notifications (Miyunotify) par email et/ou in-app ; préférences configurables (par type, par événement). |
| VIS-22 | Préférences de notification | Configurer les préférences (types de notifications, canaux, fréquence). | Page ou modal préférences ; choix par type (rappels, changements programme, alertes, récompenses) ; choix canal (email, in-app). |

---

## 3. Besoins non fonctionnels

### 3.1 Performance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-VIS-01 | Temps de chargement de l’espace visiteur | La page d’accueil de l’espace visiteur se charge en moins de 3 secondes (réseau standard). |
| NFR-VIS-02 | Temps de réservation | La réservation d’un créneau (sélection + confirmation) s’effectue en moins de 5 secondes après clic « Confirmer ». |
| NFR-VIS-03 | Affichage de l’agenda | L’agenda (calendrier ou liste) se charge en moins de 2 secondes. |

### 3.2 Disponibilité et fiabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-VIS-04 | Disponibilité | Le service est disponible 99,5 % du temps (hors fenêtres de maintenance annoncées). |
| NFR-VIS-05 | Sauvegarde des réservations | Les réservations et inscriptions sont sauvegardées et récupérables ; pas de perte à la confirmation. |

### 3.3 Sécurité et gouvernance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-VIS-06 | Authentification | Authentification sécurisée (Miyauth) ; mot de passe ou lien magique ; session avec expiration. |
| NFR-VIS-07 | Isolation des données | Les données visiteur (profil, agenda, billets, participations) ne sont accessibles qu’au visiteur et aux organisateurs des éditions concernées. |
| NFR-VIS-08 | Confidentialité des données personnelles | Respect du RGPD ; données personnelles utilisées uniquement pour les finalités déclarées (réservation, participation, notifications). |

### 3.4 Utilisabilité et accessibilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-VIS-09 | Utilisabilité | Les parcours principaux (réservation atelier, consultation agenda, téléchargement billet) sont réalisables en moins de 5 clics depuis l’espace visiteur. |
| NFR-VIS-10 | Accessibilité | Conformité WCAG 2.1 niveau AA pour l’espace visiteur (navigation clavier, lecteurs d’écran, contrastes). |
| NFR-VIS-11 | Responsive et mobile-first | L’espace visiteur et les flux de réservation sont optimisés pour mobile (consultation agenda, billet, QR code). |

### 3.5 Gestion d’agenda

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-VIS-12 | Détection des conflits de dates | La plateforme détecte un conflit (deux réservations ou deux événements à la même date/heure) avant validation et alerte ou bloque. |
| NFR-VIS-13 | Synchronisation multi-événements | L’agenda affiche les créneaux et événements de tous les événements auxquels le visiteur est inscrit ; pas de doublon ni d’écrasement. |

---

## 4. Parcours détaillés et scénarios

### 4.1 Scénario : Premier usage — onboarding par festival et première réservation

1. Le visiteur découvre un événement depuis le [catalogue](../UtilisateurNonConnecte/_index.md) (annuaire des événements).
2. Il consulte la fiche événement (dates, lieu, programme public, services proposés : ateliers, concours, pass).
3. Il clique sur « Réserver un atelier » (ou « Acheter un pass ») ; il est redirigé vers l’inscription ou la connexion.
4. Il crée un compte visiteur (email, mot de passe, nom, prénom).
5. Après validation (automatique ou email), il accède à l’espace visiteur (vide).
6. Il retourne sur la fiche événement et sélectionne un créneau d’atelier ; il clique sur « Réserver ».
7. La plateforme vérifie l’agenda : pas de conflit ; la réservation est enregistrée.
8. Le visiteur reçoit un accusé de réception ; le créneau apparaît dans son agenda.
9. Il consulte son agenda : créneau affiché avec compte à rebours « Dans X jours ».

**Besoins couverts** : VIS-01, VIS-02, VIS-04, VIS-05, VIS-07, VIS-08, VIS-12, VIS-14, VIS-21.

### 4.2 Scénario : Onboarding par groupe de festivals

1. La plateforme ou un organisateur propose un groupe « Festivals partenaires 2026 » (3 événements).
2. Le visiteur clique sur « S’inscrire au groupe » ; il crée un compte (email, mot de passe, profil).
3. Après validation, il accède à l’espace visiteur avec les 3 événements du groupe déjà associés.
4. Il consulte l’agenda : vide pour l’instant ; il peut réserver ou s’inscrire à des activités pour chacun des 3 événements.
5. Il réserve un atelier pour le Festival A et un pass pour le Festival B ; les deux apparaissent dans son agenda avec les dates respectives.
6. Pas de conflit de dates ; il reçoit les billets/pass dans son espace (onglet Billets, Pass VIP).

**Besoins couverts** : VIS-03, VIS-04, VIS-05, VIS-07, VIS-11, VIS-12, VIS-13.

### 4.3 Scénario : Conflit de dates — alerte à la réservation

1. Le visiteur a déjà réservé un atelier « Festival A », créneau samedi 15 juin 14h-16h.
2. Il souhaite réserver un atelier « Festival B », créneau samedi 15 juin 14h-15h.
3. Il sélectionne le créneau et clique sur « Réserver ».
4. La plateforme détecte un chevauchement avec « Festival A — Atelier X, 14h-16h ».
5. Un message s’affiche : « Conflit de dates : vous avez déjà réservé [Festival A — Atelier X] le 15/06 à 14h. Souhaitez-vous annuler cette réservation ou choisir un autre créneau pour Festival B ? »
6. Le visiteur peut annuler, choisir un autre créneau pour Festival B, ou confirmer (avec avertissement) selon les règles de l’édition.

**Besoins couverts** : VIS-09, NFR-VIS-12.

### 4.4 Scénario : Jeux et concours

1. L’organisateur a activé « Jeu quizz » et « Concours photo » pour son édition.
2. Le visiteur (connecté) consulte la fiche événement ; il voit les blocs « Jeu quizz » et « Concours photo ».
3. Il clique sur « Participer au quizz » ; il répond aux questions ; son score est enregistré.
4. Il clique sur « S’inscrire au concours photo » ; il uploade sa photo et envoie ; il reçoit un accusé.
5. Dans son espace visiteur, onglet « Activités », il voit : « Quizz Festival X — Score : 8/10 » ; « Concours photo Festival X — En attente de résultats ».
6. Les résultats sont publiés ; il reçoit une notification ; il consulte « Concours photo — 2e place » et sa récompense.

**Besoins couverts** : VIS-15, VIS-16, VIS-17, VIS-18, VIS-21.

### 4.5 Scénario : Multi-événements — agenda et billets unifiés

1. Le visiteur participe à 2 festivals (réservations et pass) et a 1 réservation en attente de confirmation.
2. Il se connecte et accède à l’espace visiteur ; il voit les blocs Agenda, Billets, Réservations, Pass VIP, Activités.
3. Il ouvre l’agenda : vue calendrier avec les 2 festivals (créneaux réservés, événements) ; compte à rebours pour le prochain.
4. Il ouvre Billets : liste des billets et pass par événement ; téléchargement PDF ou affichage QR code.
5. Il ouvre Pass VIP : 1 pass pour Festival A ; avantages listés.
6. Il réserve un atelier pour un 3e événement ; la plateforme vérifie l’agenda : pas de conflit ; réservation enregistrée ; nouvel événement ajouté à l’agenda.

**Besoins couverts** : VIS-05, VIS-06, VIS-07, VIS-08, VIS-11, VIS-12, VIS-13, VIS-09, NFR-VIS-13.

---

## 5. Pain points et opportunités

### 5.1 Pain points

| Pain point | Impact | Besoin associé |
|------------|--------|-----------------|
| **Infos dispersées** | Dates, programme, réservation sur des supports différents. | Vue centralisée (agenda, billets, réservations) dans l’espace visiteur (VIS-05, VIS-07, VIS-11). |
| **Pas de rappel** | Oubli de réservation ou de date d’événement. | Compte à rebours et notifications (VIS-08, VIS-21). |
| **Risque de doublon de réservation** | Réserver deux ateliers au même moment. | Gestion d’agenda et alerte conflits (VIS-09, NFR-VIS-12). |
| **Multiples comptes** | Un compte par festival ; pas de vue consolidée. | Compte cross-événements et onboarding par groupe (VIS-03, VIS-04). |
| **Pas de suivi des participations** | Perte des résultats jeux/concours. | Suivi d’activités et historique (VIS-15, VIS-16, VIS-17). |

### 5.2 Opportunités

| Opportunité | Description | Besoin associé |
|-------------|-------------|-----------------|
| **Agenda unifié** | Un seul agenda pour tous les événements ; planification sans conflit. | VIS-07, VIS-09, VIS-10. |
| **Billets et pass centralisés** | Tous les billets et pass au même endroit ; QR codes et PDF. | VIS-11, VIS-13. |
| **Expérience enrichie** | Jeux, concours, ateliers pour fidéliser et animer. | VIS-15, VIS-16, VIS-17, VIS-18. |
| **Groupes de festivals** | Partenariats organisateurs ; inscription une fois, accès à plusieurs événements. | VIS-03. |

---

## 6. Priorisation des besoins (MoSCoW)

### 6.1 Must have (indispensable)

- VIS-01 à VIS-05 (onboarding, compte cross-événements, vue espace visiteur).
- VIS-07, VIS-08 (agenda personnel, compte à rebours).
- VIS-09 (gestion d’agenda et conflits de dates).
- VIS-11 à VIS-14 (billets, réservations, pass VIP, achat/réservation depuis fiche événement).
- VIS-18 (visibilité des services par événement).
- VIS-21 (notifications).
- NFR-VIS-06 à NFR-VIS-08 (authentification, isolation, confidentialité).
- NFR-VIS-12 (détection conflits de dates).

### 6.2 Should have (important)

- VIS-02, VIS-03 (onboarding par festival, par groupe de festivals).
- VIS-06 (navigation par événement).
- VIS-10 (export ou partage agenda).
- VIS-15 à VIS-17 (jeux, concours, historique participations).
- VIS-19, VIS-20 (places limitées, règles et conditions).
- VIS-22 (préférences de notification).
- NFR-VIS-01 à NFR-VIS-05, NFR-VIS-09 à NFR-VIS-11, NFR-VIS-13 (performance, dispo, utilisabilité, accessibilité, responsive, synchronisation).

### 6.3 Could have (souhaitable)

- File d’attente si complet (VIS-19).
- Gamification (badges, niveaux) pour les jeux et concours.
- Recommandations d’événements ou d’ateliers selon profil.

### 6.4 Won’t have (hors périmètre ou report)

- Paiement en ligne intégré (si hors périmètre v1) ; réservation gratuite ou redirection vers billetterie externe.
- Besoins spécifiques aux autres publics — traités dans leurs documents.

---

## 7. Dépendances et interfaces avec les autres publics

### 7.1 Dépendances

| Dépendance | Description |
|------------|-------------|
| **Organisateurs** | Les services visiteurs (jeux, concours, ateliers, réservations, pass) sont **activés et paramétrés** par les organisateurs ; le visiteur ne fait que consommer. |
| **Catalogue** | L’annuaire des événements (catalogue) permet au visiteur de découvrir les événements et les services proposés ; la fiche événement est le point d’entrée pour réservation et inscription. |
| **Plateforme** | Authentification (Miyauth), permissions (Master Butler), persistance (KindMother), agenda cross-événements (MiyuClock, Miyubooking). |

### 7.2 Interfaces

| Interface | Flux | Besoin visiteur |
|-----------|------|------------------|
| Visiteur → Organisateur | Réservation, inscription jeu/concours, achat pass ; données de participation. | VIS-12, VIS-14, VIS-15, VIS-16. |
| Organisateur → Visiteur | Activation des services ; envoi des rappels, changements de programme ; publication des résultats. | VIS-18, VIS-21. |
| Visiteur → Catalogue | Consultation annuaire ; accès à la fiche événement pour réserver ou s’inscrire. | VIS-14, lien depuis [utilisateur non connecté](../UtilisateurNonConnecte/_index.md). |

---

## 8. User stories (format standard)

### 8.1 Onboarding et espace

- **US-VIS-01** — En tant que **visiteur**, je veux **créer un compte** (email, mot de passe, profil) **afin de** réserver des ateliers, acheter des pass et suivre mes activités.  
  *Critères* : Formulaire dédié ; création profil (Miyauth, Miyuprofile) ; accès à l’espace visiteur.*

- **US-VIS-02** — En tant que **visiteur**, je veux **m’inscrire à un groupe de festivals** en une fois **afin de** accéder à tous les événements du groupe avec un seul compte.  
  *Critères* : Inscription depuis la page groupe ; compte créé ; accès à tous les événements du groupe ; agenda et billets unifiés.*

- **US-VIS-03** — En tant que **visiteur**, je veux **voir mon agenda** (ateliers, créneaux, événements) **synchronisé entre tous mes événements** **afin de** organiser ma visite sans conflit.  
  *Critères* : Vue calendrier ou liste ; tous les événements ; compte à rebours ; alerte conflits.*

### 8.2 Réservations et billets

- **US-VIS-04** — En tant que **visiteur**, je veux **réserver un atelier** (créneau, place) **afin de** garantir ma place.  
  *Critères* : Sélection du créneau ; vérification agenda (conflit) ; confirmation ; intégration à l’agenda ; accusé de réception.*

- **US-VIS-05** — En tant que **visiteur**, je veux **consulter et télécharger mes billets et pass** **afin de** les avoir sur moi (PDF, QR code).  
  *Critères* : Liste par événement ; téléchargement PDF ou affichage QR ; mobile-friendly.*

- **US-VIS-06** — En tant que **visiteur**, je veux **être alerté en cas de conflit de dates** avant de valider une réservation **afin de** ne pas réserver deux créneaux au même moment.  
  *Critères* : Détection chevauchement ; message explicite ; alerte ou blocage ; suggestion de créneaux libres.*

### 8.3 Jeux et concours

- **US-VIS-07** — En tant que **visiteur**, je veux **participer aux jeux et concours** proposés par le festival **afin de** m’amuser et gagner des récompenses.  
  *Critères* : Accès aux jeux/concours activés ; enregistrement des participations ; résultats et récompenses selon règles organisateur.*

- **US-VIS-08** — En tant que **visiteur**, je veux **voir l’historique de mes participations** (jeux, concours, ateliers) **afin de** suivre mes scores et récompenses.  
  *Critères* : Liste ou timeline par événement ; filtre par type ; récompenses gagnées.*

### 8.4 Notifications

- **US-VIS-09** — En tant que **visiteur**, je veux **recevoir des rappels** (réservation, événement) et des **alertes** (changement de programme) **afin de** ne rien oublier.  
  *Critères* : Notifications (Miyunotify) par email et/ou in-app ; préférences configurables.*

---

## 9. Cas limites et règles métier

### 9.1 Règles métier

| Règle | Description |
|-------|-------------|
| **Services activés par l’organisateur** | Le visiteur ne peut accéder qu’aux services (jeux, concours, ateliers, réservations, pass) **activés** par l’organisateur pour chaque édition. |
| **Places limitées** | Les réservations sont soumises aux quotas définis par l’organisateur ; pas de réservation au-delà des places disponibles (sauf file d’attente si proposée). |
| **Agenda** | La plateforme signale ou bloque les conflits de dates ; le visiteur peut toutefois confirmer malgré un conflit selon règle (avertissement). |
| **Annulation** | Les règles d’annulation (délai, conditions) sont définies par l’organisateur ; le visiteur peut annuler ou modifier dans ce cadre. |

### 9.2 Cas limites

| Cas | Comportement attendu |
|-----|----------------------|
| **Réservation sur un créneau déjà complet** | Blocage ou message « Plus de places disponibles » ; proposition de file d’attente si configurée. |
| **Réservation sur un créneau en conflit avec une autre réservation** | Alerte avec le détail du conflit ; blocage ou confirmation avec avertissement selon règle. |
| **Annulation hors délai** | Blocage ou message « Annulation impossible au-delà du délai » selon règles de l’édition. |
| **Événement annulé ou reporté** | Notification au visiteur ; remboursement ou report selon politique organisateur ; mise à jour de l’agenda et des billets. |
| **Suppression de compte visiteur avec réservations en cours** | Blocage ou processus spécifique : les réservations doivent être annulées ou transférées ; données archivées pour l’organisateur. |

### 9.3 Métriques de succès

| Métrique | Description | Cible (exemple) |
|----------|-------------|------------------|
| **Taux d’activation** | % de visiteurs ayant effectué au moins une réservation ou inscription après création de compte. | > 60 % |
| **Taux de conflits de dates évités** | % de réservations où une alerte conflit a été affichée et le visiteur a modifié ou annulé. | Suivi |
| **Satisfaction visiteur** | Score NPS ou enquête (facilité, utilité de l’agenda, gain de temps). | Suivi annuel |
| **Taux d’utilisation des services** | % de visiteurs ayant utilisé au moins un service (jeu, concours, atelier, pass) par événement. | Suivi |

---

## 10. Critères d’acceptation détaillés (sélection)

### 10.1 Agenda (VIS-07, VIS-09)

- **Vue** : Calendrier (mois, semaine) ou liste ; créneaux réservés, événements auxquels le visiteur est inscrit ; libellé et couleur par événement.
- **Synchronisation** : Tous les événements du visiteur (réservations, pass, inscriptions jeux/concours) sont affichés ; pas de doublon ; mise à jour à l’actualisation ou en temps réel selon config.
- **Conflits** : À la réservation, comparaison des dates/heures du créneau avec les créneaux et événements déjà enregistrés ; règle de chevauchement (même jour, même créneau, même heure) configurable ; message explicite avec le nom de l’événement/créneau en conflit.

### 10.2 Réservations (VIS-12, VIS-14)

- **Flux** : Sélection de l’événement et du type (atelier, place, pass) ; sélection du créneau ou du type de pass ; vérification agenda ; confirmation ; accusé de réception.
- **Places limitées** : Affichage des places restantes par créneau ; blocage si complet ; option file d’attente si configurée.
- **Annulation** : Action « Annuler » selon délai et règles de l’édition ; libération de la place ; mise à jour de l’agenda.

### 10.3 Billets et pass (VIS-11, VIS-13)

- **Liste** : Par événement ; colonnes : Type (billet, pass), Date/Événement, Statut ; bouton Télécharger PDF ou afficher QR code.
- **QR code** : Affichage mobile-friendly pour contrôle à l’entrée ; mise à jour si statut change (annulation, report).
- **Pass VIP** : Fiche détail (type, événement(s), avantages, conditions) ; justificatif téléchargeable ou QR code.

### 10.4 Jeux et concours (VIS-15, VIS-16, VIS-17)

- **Accès** : Liste des jeux et concours ouverts par événement ; lien « Participer » ; enregistrement des réponses ou des participations.
- **Résultats** : Affichage selon règles organisateur (date de publication, visibilité) ; récompenses listées ; notification si configurée.
- **Historique** : Liste des participations par événement et par type ; score ou résultat ; récompense gagnée si applicable.

---

## 11. Glossaire et références

### 11.1 Glossaire (extrait)

| Terme | Définition |
|-------|------------|
| **Espace visiteur** | Espace dédié au visiteur : agenda, billets, réservations, pass VIP, suivi d’activités (jeux, concours, ateliers). |
| **Compte cross-événements** | Un même compte visiteur permet de suivre ses activités sur **plusieurs événements** ; agenda et billets unifiés. |
| **Onboarding par groupe de festivals** | Inscription une fois pour une famille d’événements (groupe partenaires) ; accès à tous les événements du groupe avec le même compte. |
| **Services visiteur** | Jeux, concours, inscriptions ateliers, réservations, pass VIP ; **activés** par l’organisateur par édition. |

### 11.2 Références

- [Document fondateur JayFestival](../../JayFestival%20-%20Document%20Fondateur.md)
- [Visiteurs — Parcours, capacités et services](./Visiteurs%20-%20Parcours%20Capacites%20Services.md)
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Exposants](../Exposants/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)

---

**Document** : Visiteurs — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit — référence pour le public Visiteurs
