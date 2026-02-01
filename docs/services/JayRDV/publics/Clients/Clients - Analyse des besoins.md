# Clients — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du public cible **Clients** pour le service JayRDV. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels, les parcours détaillés, les user stories, les pain points et opportunités, ainsi que la priorisation et les dépendances. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md), [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md), [Parcours, capacités et livrables](./Clients%20-%20Parcours%20Capacites%20Livrables.md).

## Portée / Scope

- **Public** : Clients (personnes qui prennent rendez-vous auprès des professionnels — B2C).
- **Périmètre** : Tous les besoins identifiés pour ce public (réservation, compte client, historique, rappels, annulations, préférences).
- **Hors périmètre** : Spécifications techniques d’implémentation (API, schémas), spécifications des autres publics (Professionnels, Utilisateur non connecté).

---

## 1. Profil du public et personas

### 1.1 Définition du public

Les **clients** sont les personnes qui **prennent rendez-vous** auprès des professionnels (médecins, thérapeutes, coiffeurs, consultants, etc.). Ils peuvent disposer d’un **compte client** (optionnel selon parcours) pour consulter leur historique, gérer leurs RDV et leurs préférences, ou réserver **sans compte** (parcours guest). Le compte client permet une expérience unifiée sur plusieurs professionnels et établissements (si la plateforme le permet).

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Client occasionnel** | Prend 1 à 2 RDV par an chez un même pro ; préfère la simplicité. | Réserver rapidement, recevoir une confirmation et un rappel, ne pas oublier le RDV. | Processus long, création de compte obligatoire, pas de rappel. |
| **Client régulier** | Consulte le même professionnel plusieurs fois par an (suivi, soins récurrents). | Réserver en quelques clics, voir son historique, être reconnu (fiche client côté pro). | Saisie répétée des mêmes infos, pas d’historique centralisé. |
| **Client multi-pros** | Consulte plusieurs professionnels (médecin, kiné, coiffeur) ; besoin de cohérence. | Un seul compte pour tous ses RDV, agenda unifié, rappels centralisés. | Multiples comptes ou liens, pas de vue consolidée, risque d’oubli. |
| **Client mobile-first** | Réserve et consulte surtout depuis son smartphone. | Réserver depuis le mobile, recevoir un rappel SMS, ajouter le RDV à son agenda. | Site non responsive, pas de lien « Ajouter à l’agenda », pas de SMS. |
| **Client sans compte (guest)** | Ne souhaite pas créer de compte ; une réservation ponctuelle. | Réserver sans inscription, recevoir la confirmation par email. | Obligation de créer un compte, trop de champs obligatoires. |

### 1.3 Contexte d’usage

- **Fréquence** : Ponctuelle (réservation) ou régulière (consultation agenda, annulation, reprise de RDV).
- **Appareils** : Mobile prioritaire pour la réservation et les rappels ; desktop pour la planification et l’historique.
- **Concurrence** : Appel téléphonique, email, autres solutions de réservation (Doctolib, Calendly, etc.) ; attente d’un **parcours court** et **fiable** (confirmation, rappel).

---

## 2. Besoins fonctionnels

### 2.1 Accès à la réservation (avec ou sans compte)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| CLI-01 | Accès au lien de réservation | Pouvoir accéder à la page de réservation du professionnel via un lien partagé (email, site, réseaux sociaux). | Page s’affiche sans compte ; affichage des services et des créneaux disponibles du professionnel. |
| CLI-02 | Réservation sans compte (guest) | Pouvoir réserver un RDV sans créer de compte ; saisie minimale (nom, email, téléphone). | Formulaire de réservation avec champs obligatoires (nom, email, téléphone) ; pas de mot de passe ; confirmation envoyée à l’email saisi. |
| CLI-03 | Réservation avec compte client | Pouvoir se connecter et réserver en étant reconnu (données pré-remplies, historique). | Connexion (Miyauth) ; formulaire pré-rempli avec les données du compte ; réservation enregistrée dans l’historique client. |
| CLI-04 | Choix du service | Voir la liste des services proposés par le professionnel (nom, durée, tarif optionnel) et en sélectionner un. | Liste des services actifs ; sélection d’un service ; passage à l’étape « Choix du créneau ». |
| CLI-05 | Choix du créneau | Voir les créneaux disponibles (date, heure) pour le service choisi et en sélectionner un. | Affichage des créneaux disponibles (temps réel) ; sélection date/heure ; pas de créneau déjà pris (vérification en temps réel). |
| CLI-06 | Choix du praticien (si multi-praticiens) | Si le professionnel a plusieurs praticiens, pouvoir choisir un praticien ou « Premier disponible ». | Liste des praticiens ou option « Premier disponible » ; créneaux filtrés selon le choix. |
| CLI-07 | Formulaire de réservation | Saisir les informations requises (nom, email, téléphone, remarque optionnelle) ; pré-rempli si compte client. | Champs obligatoires selon paramétrage pro ; remarque optionnelle ; validation avant confirmation. |
| CLI-08 | Confirmation de réservation | Après validation, recevoir une confirmation (écran + email et/ou SMS). | Message de succès à l’écran ; email et/ou SMS envoyé avec récapitulatif (date, heure, service, lieu, lien ajout à l’agenda). |
| CLI-09 | Ajout du RDV à l’agenda client | Pouvoir ajouter le RDV à son agenda (Google, Outlook, Apple, iCal) depuis l’email de confirmation ou l’écran. | Lien « Ajouter à mon agenda » dans l’email ; génération de fichier iCal ou lien vers Google/Outlook/Apple. |
| CLI-10 | Paiement en ligne (si activé par le pro) | Payer en ligne (intégral ou acompte) à la réservation si le professionnel l’a configuré. | Page de paiement sécurisée après saisie des infos ; confirmation du paiement et du RDV ; reçu par email. |

### 2.2 Compte client (optionnel)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| CLI-11 | Création de compte client | Pouvoir s’inscrire en tant que client (email, mot de passe ou lien magique, nom, téléphone). | Formulaire d’inscription ; validation email si configurée ; création du profil client (Miyauth, Miyuprofile). |
| CLI-12 | Connexion | Se connecter avec email/mot de passe ou lien magique. | Page de connexion ; authentification sécurisée ; redirection vers l’espace client ou vers la page de réservation (contexte). |
| CLI-13 | Récupération de mot de passe | Réinitialiser son mot de passe en cas d’oubli. | Lien « Mot de passe oublié » ; saisie email ; envoi d’un lien de réinitialisation ; formulaire nouveau mot de passe. |
| CLI-14 | Profil client | Consulter et modifier son profil (nom, email, téléphone, préférences). | Page « Mon profil » ; édition des champs ; sauvegarde ; cohérence avec les données utilisées pour les réservations. |
| CLI-15 | Préférences de notification | Choisir les canaux (email, SMS) et les types de notifications (confirmation, rappel, annulation). | Page « Préférences » ; activation/désactivation par type et par canal ; prise en compte pour les prochains RDV. |

### 2.3 Mes RDV (espace client)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| CLI-16 | Liste des RDV à venir | Consulter la liste de ses prochains RDV (tous professionnels confondus si compte multi-pros). | Liste avec date, heure, professionnel, service, lieu ; tri par date ; lien vers détail. |
| CLI-17 | Détail d’un RDV | Voir le détail d’un RDV (professionnel, service, date, heure, lieu, remarque, statut). | Fiche détail ; informations complètes ; boutons Modifier (créneau), Annuler selon règles du professionnel. |
| CLI-18 | Annulation par le client | Annuler un RDV depuis l’espace client ou depuis le lien dans l’email de confirmation. | Bouton « Annuler » ; confirmation (motif optionnel) ; application de la politique d’annulation du pro (délai gratuit, pénalité) ; notification au professionnel ; libération du créneau. |
| CLI-19 | Modification du créneau par le client | Changer la date ou l’heure d’un RDV (reprendre un autre créneau) si le professionnel l’autorise. | Bouton « Modifier » ou « Reprendre un créneau » ; affichage des créneaux disponibles ; sélection d’un nouveau créneau ; confirmation ; notification au pro. |
| CLI-20 | Historique des RDV passés | Consulter l’historique de ses RDV passés (date, professionnel, service). | Liste des RDV passés ; filtre par période, par professionnel ; pas de modification possible sur les RDV passés. |
| CLI-21 | Lien d’annulation ou de modification dans l’email | Cliquer sur un lien dans l’email de confirmation pour annuler ou modifier le RDV sans se connecter (token sécurisé). | Lien unique et temporaire dans l’email ; accès à la page d’annulation ou de modification ; pas de connexion obligatoire ; lien invalidé après utilisation ou expiration. |

### 2.4 Rappels et notifications

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| CLI-22 | Rappel automatique | Recevoir un rappel (email et/ou SMS) la veille ou quelques heures avant le RDV. | Envoi automatique au délai configuré par le professionnel ; contenu : date, heure, lieu, service ; lien vers annulation ou modification si proposé. |
| CLI-23 | Confirmation à la réservation | Recevoir une confirmation immédiate (email et/ou SMS) après la prise de RDV. | Envoi automatique ; récapitulatif (date, heure, service, lieu) ; lien « Ajouter à mon agenda » ; lien d’annulation/modification. |
| CLI-24 | Notification d’annulation ou de modification | Être notifié si le professionnel annule ou modifie le RDV. | Email et/ou SMS avec motif optionnel ; proposition de reprendre un créneau si modification par le pro. |
| CLI-25 | Alerte désistement (si activée) | Être notifié si un créneau plus tôt se libère (liste d’attente). | Inscription à la liste d’attente (option) ; notification si libération ; lien vers reprise de RDV. |

### 2.5 Multi-professionnels et agenda

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| CLI-26 | Vue agrégée des RDV | Si le compte client permet plusieurs professionnels, voir tous ses RDV (tous pros) dans une même liste ou un agenda. | Liste ou calendrier avec tous les RDV ; filtre par professionnel ; tri par date. |
| CLI-27 | Conflits de dates | Être alerté si le client tente de réserver un créneau qui chevauche un autre RDV déjà pris (même jour, même heure). | Détection des chevauchements ; alerte « Vous avez déjà un RDV à cette date/heure » ; suggestion de choisir un autre créneau. |
| CLI-28 | Accès aux différents professionnels | Depuis le compte client, accéder aux pages de réservation de différents professionnels (liens sauvegardés ou annuaire). | Liste « Mes professionnels » ou favoris ; lien vers la page de réservation de chaque pro ; historique par pro. |

### 2.6 Accessibilité et expérience

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| CLI-29 | Parcours court (réservation en quelques clics) | Réduire le nombre d’étapes pour réserver (objectif marché : ~30 secondes). | Maximum 4 à 5 étapes : Choix service → Choix créneau → Saisie infos (ou connexion) → Confirmation. |
| CLI-30 | Responsive (mobile, tablette, desktop) | La page de réservation et l’espace client sont utilisables sur tous les appareils. | Mise en page adaptée ; boutons et champs accessibles ; pas de perte de fonctionnalité sur mobile. |
| CLI-31 | Accessibilité (WCAG) | La page de réservation et l’espace client respectent les critères d’accessibilité (navigation clavier, lecteurs d’écran, contrastes). | Conformité WCAG 2.1 niveau AA ; navigation au clavier ; labels et contrastes. |
| CLI-32 | Message d’erreur clair | En cas de créneau déjà pris ou d’erreur, afficher un message clair et proposer une action (choisir un autre créneau). | Message explicite (« Ce créneau n’est plus disponible ») ; proposition de rafraîchir les créneaux ou de choisir une autre date. |

---

## 3. Besoins non fonctionnels

### 3.1 Performance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-CLI-01 | Temps de chargement de la page de réservation | La page de réservation (services et créneaux) se charge en moins de 3 secondes (réseau standard). |
| NFR-CLI-02 | Disponibilité des créneaux en temps réel | Les créneaux affichés sont à jour ; après une réservation (par un autre client), le créneau disparaît en moins de 2 secondes. |
| NFR-CLI-03 | Temps de confirmation | Après clic « Confirmer », la confirmation s’affiche et l’email/SMS est envoyé en moins de 5 secondes. |
| NFR-CLI-04 | Temps de chargement de l’espace client | La page « Mes RDV » se charge en moins de 2 secondes. |

### 3.2 Disponibilité et fiabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-CLI-05 | Disponibilité | La page de réservation et l’espace client sont disponibles 99,5 % du temps (hors maintenance annoncée). |
| NFR-CLI-06 | Sauvegarde des réservations | Une réservation confirmée est enregistrée de façon durable ; pas de perte après confirmation. |
| NFR-CLI-07 | Idempotence (éviter double réservation) | Si le client clique deux fois sur « Confirmer », une seule réservation est créée ; message clair en cas de conflit. |

### 3.3 Sécurité et confidentialité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-CLI-08 | Authentification (compte client) | Connexion sécurisée (Miyauth) ; mot de passe ou lien magique ; session avec expiration. |
| NFR-CLI-09 | Données personnelles (RGPD) | Consentement, droit d’accès, de rectification, d’effacement ; durée de conservation configurable ; données utilisées uniquement pour la réservation et les rappels. |
| NFR-CLI-10 | Lien d’annulation/modification sécurisé | Le lien dans l’email est unique, temporaire et non devinable ; invalidé après utilisation ou expiration (ex. 7 jours). |
| NFR-CLI-11 | Paiement (si activé) | Paiement sécurisé (PCI, 3D Secure) ; pas de stockage des données de carte côté plateforme (ou conformité PCI-DSS). |

### 3.4 Utilisabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-CLI-12 | Parcours intuitif | Un utilisateur non familiarisé peut réserver un RDV en moins de 5 étapes sans aide. |
| NFR-CLI-13 | Messages en français | Interface et messages (erreur, confirmation) en français par défaut. |
| NFR-CLI-14 | Pré-remplissage (compte client) | Lorsqu’un client connecté réserve, ses coordonnées sont pré-remplies ; pas de ressaisie. |

---

## 4. Parcours détaillés et scénarios

### 4.1 Scénario : Réservation guest (sans compte)

1. Le client clique sur le lien de réservation du professionnel (reçu par email ou trouvé sur le site).
2. La page s’affiche : liste des services (ex. « Consultation 30 min », « Séance 1 h »). Il sélectionne « Consultation 30 min ».
3. Les créneaux disponibles s’affichent (calendrier ou liste des prochains jours). Il sélectionne « Jeudi 6 février, 10h00 ».
4. Le formulaire s’affiche : nom, email, téléphone, remarque (optionnel). Il remplit et clique sur « Confirmer ».
5. Une page de confirmation s’affiche : « Votre RDV est confirmé. Vous allez recevoir un email de confirmation. »
6. Il reçoit un email avec le récapitulatif et un lien « Ajouter à mon agenda » et un lien « Annuler ou modifier le RDV ».
7. La veille du RDV, il reçoit un rappel par email (ou SMS si configuré par le pro).

**Besoins couverts** : CLI-01, CLI-02, CLI-04, CLI-05, CLI-07, CLI-08, CLI-09, CLI-22, CLI-23.

### 4.2 Scénario : Réservation avec compte client

1. Le client se connecte à son espace client (ou clique sur « Se connecter » depuis la page de réservation).
2. Il est redirigé vers la page de réservation du professionnel (contexte conservé). Les services s’affichent ; il sélectionne un service.
3. Il choisit un créneau. Le formulaire est pré-rempli avec son nom, email, téléphone. Il peut ajouter une remarque et confirmer.
4. La réservation est enregistrée ; il est redirigé vers « Mes RDV » ou voit un message de succès. Il reçoit la confirmation par email.
5. Depuis « Mes RDV », il voit son prochain RDV ; il peut annuler ou modifier (reprendre un créneau) selon les règles du pro.

**Besoins couverts** : CLI-03, CLI-07, CLI-11, CLI-12, CLI-16, CLI-18, CLI-19, CLI-26, NFR-CLI-14.

### 4.3 Scénario : Annulation depuis l’email

1. Le client reçoit l’email de confirmation avec un lien « Annuler ou modifier le RDV ».
2. Il clique sur le lien ; une page s’ouvre (sans connexion) avec le récapitulatif du RDV et un bouton « Annuler le RDV ».
3. Il clique sur « Annuler » ; une demande de confirmation s’affiche (motif optionnel). Il valide.
4. Le RDV est annulé ; il reçoit un email de confirmation d’annulation. Le créneau est libéré côté professionnel.

**Besoins couverts** : CLI-21, CLI-18, NFR-CLI-10.

### 4.4 Scénario : Conflit de dates (multi-pros)

1. Le client a déjà un RDV chez le médecin le jeudi 6 février à 10h. Il tente de réserver un RDV chez le kiné le même jour à 10h.
2. Après avoir choisi le créneau « Jeudi 6 février, 10h00 » chez le kiné, le système détecte le chevauchement avec le RDV chez le médecin.
3. Un message s’affiche : « Vous avez déjà un RDV à cette date/heure (Médecin Dupont). Choisissez un autre créneau. »
4. Le client choisit 14h00 ; la réservation est confirmée.

**Besoins couverts** : CLI-27, CLI-26.

---

## 5. Pain points et opportunités

### 5.1 Pain points

| Pain point | Impact | Besoin associé |
|------------|--------|-----------------|
| Création de compte obligatoire | Friction ; abandon du parcours. | CLI-02 (réservation guest). |
| Parcours trop long | Trop d’étapes ; abandon. | CLI-29 (parcours court). |
| Pas de rappel | Oubli du RDV ; no-show. | CLI-22, CLI-23 (rappels, confirmation). |
| Créneau déjà pris | Le client choisit un créneau qui vient d’être pris ; erreur ou frustration. | CLI-05, NFR-CLI-02 (temps réel), CLI-32 (message d’erreur). |
| Pas d’ajout à l’agenda | Le client doit ressaisir le RDV dans son agenda. | CLI-09 (lien « Ajouter à mon agenda »). |
| Annulation compliquée | Pas de lien dans l’email ; obligation d’appeler. | CLI-21 (lien d’annulation dans l’email). |

### 5.2 Opportunités

| Opportunité | Description | Besoin associé |
|-------------|-------------|-----------------|
| Parcours guest | Réserver sans compte ; taux de conversion plus élevé. | CLI-02, CLI-29. |
| Rappels systématiques | Réduction des no-show ; satisfaction client et pro. | CLI-22, CLI-23. |
| Compte client optionnel | Historique, pré-remplissage, vue agrégée ; fidélisation. | CLI-11 à CLI-15, CLI-26, CLI-27. |
| Lien annulation/modification | Autonomie du client ; moins d’appels pour le pro. | CLI-21, CLI-18, CLI-19. |
| Ajout à l’agenda | Moins d’oubli ; meilleure expérience. | CLI-09. |

---

## 6. Priorisation des besoins (MoSCoW)

### 6.1 Must have (indispensable)

- CLI-01, CLI-02, CLI-04, CLI-05, CLI-07, CLI-08 (accès, réservation guest, choix service/créneau, formulaire, confirmation).
- CLI-09 (ajout à l’agenda).
- CLI-22, CLI-23 (rappels, confirmation).
- CLI-18, CLI-21 (annulation par le client, lien dans l’email).
- CLI-29, CLI-30, CLI-32 (parcours court, responsive, message d’erreur).
- NFR-CLI-01 à NFR-CLI-07, NFR-CLI-09 à NFR-CLI-11 (performance, dispo, RGPD, lien sécurisé, paiement).

### 6.2 Should have (important)

- CLI-03, CLI-11 à CLI-15 (compte client, connexion, profil, préférences).
- CLI-16 à CLI-20, CLI-26 (mes RDV, liste, détail, annulation/modification depuis l’espace, historique, vue agrégée).
- CLI-19 (modification du créneau par le client).
- CLI-24, CLI-25 (notification annulation/modification pro, alerte désistement).
- CLI-27, CLI-28 (conflits de dates, accès multi-pros).
- CLI-10 (paiement en ligne).
- NFR-CLI-08, NFR-CLI-12 à NFR-CLI-14 (authentification, utilisabilité, pré-remplissage).
- NFR-CLI-31 (accessibilité WCAG).

### 6.3 Could have (souhaitable)

- CLI-06 (choix du praticien) — peut être géré côté pro par défaut.
- Amélioration des préférences de notification (granularité par pro).
- Export de l’historique des RDV (PDF, CSV).

### 6.4 Won’t have (hors périmètre ou report)

- Besoins spécifiques aux Professionnels ou à l’Utilisateur non connecté.
- Fonctionnalités avancées (notation, avis, chat) — hors périmètre v1.

---

## 7. Dépendances et interfaces

### 7.1 Dépendances

| Dépendance | Description |
|------------|-------------|
| **Professionnels** | Les clients dépendent des créneaux et des services exposés par les professionnels (lien, widget) ; les professionnels dépendent des réservations effectuées par les clients. |
| **Utilisateur non connecté** | Le parcours guest est partagé avec l’utilisateur non connecté (même page de réservation) ; la distinction est « avec compte » vs « sans compte » après réservation. |
| **Plateforme** | Miyauth (compte client), Miyunotify (confirmations, rappels), Miyubooking (créneaux, RDV), KindMother (persistance), WorrySentinel (sécurité). |

### 7.2 Interfaces

| Interface | Flux | Besoin client |
|-----------|------|----------------|
| Client → Professionnel | Réservation, annulation, modification ; données client (nom, email, téléphone). | CLI-01 à CLI-10, CLI-18 à CLI-21. |
| Professionnel → Client | Confirmation, rappels, notification annulation/modification. | CLI-22 à CLI-25. |
| Client → Plateforme | Compte client, mes RDV, préférences. | CLI-11 à CLI-20, CLI-26 à CLI-28. |

---

## 8. User stories (format standard)

### 8.1 Réservation

- **US-CLI-01** — En tant que **client**, je veux **accéder à la page de réservation** du professionnel via un lien **afin de** réserver un RDV sans appeler. *Critères* : Page s’affiche avec services et créneaux ; pas de compte obligatoire.
- **US-CLI-02** — En tant que **client**, je veux **réserver un RDV sans créer de compte** (nom, email, téléphone) **afin de** gagner du temps. *Critères* : Formulaire minimal ; confirmation par email ; pas de mot de passe.
- **US-CLI-03** — En tant que **client**, je veux **choisir un service et un créneau** **afin de** réserver le RDV qui me convient. *Critères* : Liste des services ; créneaux en temps réel ; pas de double réservation.
- **US-CLI-04** — En tant que **client**, je veux **recevoir une confirmation** et **un rappel** avant le RDV **afin de** ne pas oublier. *Critères* : Confirmation immédiate (email/SMS) ; rappel 24h et/ou 2h avant ; lien « Ajouter à mon agenda ».
- **US-CLI-05** — En tant que **client**, je veux **annuler ou modifier mon RDV** via un lien dans l’email **afin de** ne pas avoir à appeler. *Critères* : Lien unique et sécurisé ; annulation/modification sans connexion ; confirmation par email.

### 8.2 Compte client

- **US-CLI-06** — En tant que **client**, je veux **créer un compte** (email, mot de passe) **afin de** retrouver mes RDV et réserver plus vite la prochaine fois. *Critères* : Inscription ; validation email ; accès à « Mes RDV ».
- **US-CLI-07** — En tant que **client connecté**, je veux **voir la liste de mes prochains RDV** **afin de** planifier ma semaine. *Critères* : Liste triée par date ; détail par RDV ; boutons Annuler/Modifier selon règles du pro.
- **US-CLI-08** — En tant que **client connecté**, je veux **que mes coordonnées soient pré-remplies** lors d’une réservation **afin de** ne pas les ressaisir. *Critères* : Formulaire pré-rempli ; sauvegarde après réservation.

### 8.3 Notifications et conflits

- **US-CLI-09** — En tant que **client**, je veux **être notifié** si le professionnel annule ou modifie mon RDV **afin de** m’organiser. *Critères* : Email et/ou SMS ; motif optionnel ; proposition de reprendre un créneau.
- **US-CLI-10** — En tant que **client avec plusieurs RDV**, je veux **être alerté** si je réserve un créneau qui chevauche un autre RDV **afin de** éviter les conflits. *Critères* : Détection des chevauchements ; message clair ; suggestion d’un autre créneau.

---

## 9. Cas limites et règles métier

### 9.1 Règles métier

| Règle | Description |
|-------|-------------|
| **Créneau unique** | Un créneau ne peut être réservé qu’une seule fois ; vérification en temps réel à la confirmation. |
| **Politique d’annulation** | Les règles d’annulation (délai gratuit, pénalité) sont définies par le professionnel ; affichées au client avant confirmation. |
| **Lien d’annulation** | Le lien dans l’email est valide jusqu’à une date limite (ex. J-1) ou jusqu’à utilisation ; après, le client doit contacter le pro ou se connecter. |
| **Données client** | Les données (nom, email, téléphone) sont utilisées uniquement pour la réservation, les confirmations et les rappels ; pas de revente ni d’usage non autorisé (RGPD). |

### 9.2 Cas limites

| Cas | Comportement attendu |
|-----|----------------------|
| **Deux clients réservent le même créneau** | Un seul obtient la réservation ; l’autre reçoit « Ce créneau n’est plus disponible » et peut en choisir un autre. Verrouillage optimiste ou pessimiste côté serveur. |
| **Client clique deux fois sur « Confirmer »** | Une seule réservation est créée ; message de succès une seule fois ; pas de double email. |
| **Lien d’annulation expiré** | Message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. » |
| **Annulation après délai gratuit** | Application de la pénalité (si configurée par le pro) ; information au client avant validation de l’annulation. |
| **Client sans email valide** | Validation du format email à la saisie ; en cas d’email invalide, la confirmation ne part pas — affichage d’un message d’erreur côté client. |

### 9.3 Métriques de succès

| Métrique | Description | Cible (exemple) |
|----------|-------------|------------------|
| **Taux de complétion** | % de parcours de réservation complétés (de l’arrivée sur la page à la confirmation). | > 70 % |
| **Taux de réservation guest** | % de réservations faites sans compte (si option proposée). | Suivi ; objectif selon positionnement. |
| **Taux de no-show** | % de RDV confirmés non honorés (sans annulation). | Réduction avec rappels (référence : division par 5). |
| **Satisfaction client** | Score NPS ou enquête (facilité de réservation, clarté des rappels). | Suivi annuel. |
| **Temps moyen de réservation** | Délai entre l’arrivée sur la page et la confirmation. | < 60 secondes (objectif marché). |

---

## 10. Glossaire et références

### 10.1 Glossaire (extrait)

| Terme | Définition |
|-------|------------|
| **Réservation guest** | Prise de RDV sans création de compte ; saisie minimale (nom, email, téléphone) ; confirmation par email. |
| **Compte client** | Compte utilisateur (Miyauth) permettant de consulter « Mes RDV », de modifier son profil et de réserver avec pré-remplissage. |
| **Lien d’annulation** | Lien unique et temporaire dans l’email de confirmation permettant d’annuler (ou modifier) le RDV sans se connecter. |
| **Créneau** | Plage horaire disponible pour un RDV (date, heure de début, heure de fin), associée à un professionnel et/ou une ressource. |

### 10.2 Références documentaires

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md)
- [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md)
- [Clients — Parcours, capacités et livrables](./Clients%20-%20Parcours%20Capacites%20Livrables.md)
- [Public Professionnels](../Professionnels/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)

### 10.3 Matrice besoins Clients / fonctionnalités marché

| Domaine benchmark | Besoins JayRDV (Clients) | Priorité |
|-------------------|---------------------------|----------|
| Prise de RDV client | CLI-01 à CLI-10 (accès, guest, choix service/créneau, formulaire, confirmation, ajout agenda, paiement) | Must / Should |
| Notifications et rappels | CLI-22 à CLI-25 (rappels, confirmation, notification annulation, alerte désistement) | Must |
| Compte client / historique | CLI-11 à CLI-20, CLI-26 à CLI-28 (compte, mes RDV, annulation, modification, historique, vue agrégée) | Should |
| Expérience utilisateur | CLI-29 à CLI-32, NFR-CLI-12 à NFR-CLI-14 (parcours court, responsive, accessibilité, messages) | Must / Should |

### 10.4 Critères d’acceptation synthétiques (MVP Clients)

Pour la première version livrable (MVP) du public Clients, les critères d’acceptation globaux suivants sont retenus :

- **Réservation guest** : Le client peut accéder au lien de réservation du professionnel, choisir un service et un créneau (affichés en temps réel), saisir nom/email/téléphone, confirmer et recevoir une confirmation par email. Aucun compte n’est obligatoire.
- **Confirmation et rappels** : À la réservation, le client reçoit une confirmation (email et/ou SMS) avec récapitulatif et lien « Ajouter à mon agenda ». Un rappel est envoyé automatiquement (24h et/ou 2h avant) selon la configuration du professionnel.
- **Annulation** : Le client peut annuler son RDV via un lien sécurisé dans l’email de confirmation (sans connexion) ou depuis l’espace « Mes RDV » s’il a un compte. La politique d’annulation du professionnel (délai gratuit, pénalité) est appliquée et affichée.
- **Compte client (optionnel)** : Le client peut créer un compte (email, mot de passe), se connecter, consulter « Mes RDV » (liste à venir et historique), modifier son profil et réserver avec pré-remplissage de ses coordonnées.
- **Parcours court** : Maximum 4 à 5 étapes pour réserver (service → créneau → infos → confirmation). Page responsive (mobile, tablette, desktop). Message d’erreur clair si créneau non disponible.
- **Sécurité et RGPD** : Données personnelles utilisées uniquement pour la réservation et les notifications ; consentement et droits (accès, rectification, effacement) respectés. Lien d’annulation unique, temporaire et sécurisé.

Ces critères couvrent les besoins CLI-01 à CLI-09, CLI-18, CLI-21 à CLI-23, CLI-29, CLI-30, CLI-32 et les NFR-CLI-01 à NFR-CLI-07, NFR-CLI-09 à NFR-CLI-11. Le compte client (CLI-11 à CLI-20) et la modification de créneau (CLI-19) sont en scope Should pour le MVP.

### 10.5 Index des sections (Analyse Clients)

1. **Profil du public et personas** — Définition, personas (occasionnel, régulier, multi-pros, mobile-first, guest), contexte d’usage.  
2. **Besoins fonctionnels** — Accès à la réservation (guest, compte), choix service/créneau, formulaire, confirmation, compte client, mes RDV, annulation/modification, rappels et notifications, multi-pros et agenda, accessibilité et expérience.  
3. **Besoins non fonctionnels** — Performance, disponibilité, sécurité et confidentialité, utilisabilité.  
4. **Parcours détaillés** — Réservation guest, réservation avec compte, annulation depuis l’email, conflit de dates.  
5. **Pain points et opportunités** — Problèmes actuels ou anticipés, opportunités (guest, rappels, compte optionnel, lien annulation, ajout agenda).  
6. **Priorisation MoSCoW** — Must, Should, Could, Won’t have.  
7. **Dépendances et interfaces** — Professionnels, Utilisateur non connecté, Plateforme.  
8. **User stories** — Réservation, compte client, notifications et conflits.  
9. **Cas limites et règles métier** — Règles (créneau unique, politique annulation, lien annulation, RGPD), cas limites (double réservation, double clic, lien expiré, annulation après délai), métriques de succès.  
10. **Glossaire et références** — Glossaire, références documentaires, matrice besoins/fonctionnalités marché, critères MVP, index des sections.

### 10.6 Annexes — Checklist détaillée (parcours client)

- [ ] **Accès** : Lien de réservation du professionnel accessible sans compte ; page affiche les services et les créneaux disponibles.  
- [ ] **Choix service** : Liste des services (nom, durée, tarif si affiché) ; sélection d’un service ; passage à l’étape créneaux.  
- [ ] **Choix créneau** : Créneaux affichés en temps réel (pas de créneau déjà pris) ; sélection date/heure ; si multi-praticiens, choix du praticien ou « Premier disponible ».  
- [ ] **Formulaire** : Champs nom, email, téléphone (obligatoires) ; remarque (optionnel) ; si compte client connecté, pré-remplissage.  
- [ ] **Confirmation** : Message de succès à l’écran ; email et/ou SMS envoyé avec récapitulatif ; lien « Ajouter à mon agenda » ; lien « Annuler ou modifier le RDV ».  
- [ ] **Rappel** : Rappel envoyé au délai configuré par le pro (24h et/ou 2h avant) ; contenu : date, heure, lieu, service.  
- [ ] **Annulation** : Lien dans l’email fonctionne sans connexion ; page d’annulation avec confirmation ; motif optionnel ; email de confirmation d’annulation ; créneau libéré côté pro.  
- [ ] **Compte client** : Inscription (email, mot de passe) ; connexion ; page « Mes RDV » (liste à venir, détail, annulation/modification) ; page « Mon profil » ; pré-remplissage à la réservation.  
- [ ] **Erreurs** : Si créneau plus disponible, message clair « Ce créneau n’est plus disponible » ; proposition de choisir un autre créneau.  
- [ ] **Responsive** : Page de réservation et espace client utilisables sur mobile, tablette et desktop.  
- [ ] **RGPD** : Consentement et informations sur les données ; droits d’accès, rectification, effacement ; durée de conservation.

### 10.7 Dépendances techniques (référence)

| Besoin client | Composant / Kit | Rôle |
|---------------|-----------------|------|
| Accès page réservation, créneaux | Miyubooking, lien pro | Page publique, calcul des créneaux disponibles. |
| Réservation guest, formulaire | Miyubooking, Miyucontacts (léger) | Enregistrement RDV, données client (nom, email, téléphone). |
| Compte client, connexion | Miyauth, Miyuprofile | Authentification, profil client. |
| Confirmation, rappels | Miyunotify | Email et SMS (confirmation, rappel, annulation). |
| Mes RDV, historique | Miyubooking, KindMother | Liste des RDV par client, persistance. |
| Lien annulation/modification | Miyubooking, token sécurisé | Lien unique temporaire, annulation sans connexion. |
| Paiement (si activé) | Miyuinvoice ou partenaire | Paiement en ligne à la réservation. |
| RGPD, consentement | WorrySentinel, traçabilité | Données personnelles, droits, audit. |

La répartition exacte entre Opérateurs, Kits et Cores est définie dans les spécifications techniques (hors scope de ce document).

### 10.8 Récapitulatif des parcours principaux (Clients)

| Parcours | Déclencheur | Étapes clés | Besoins couverts |
|----------|-------------|-------------|------------------|
| **Réservation guest** | Clic sur le lien de réservation du pro | Choix service → Choix créneau → Saisie nom/email/téléphone → Confirmation | CLI-01, CLI-02, CLI-04, CLI-05, CLI-07, CLI-08, CLI-09, CLI-22, CLI-23 |
| **Réservation avec compte** | Connexion puis accès au lien du pro | Connexion → Choix service → Choix créneau → Formulaire pré-rempli → Confirmation | CLI-03, CLI-07, CLI-11, CLI-12, CLI-16, NFR-CLI-14 |
| **Annulation depuis l’email** | Clic sur le lien « Annuler ou modifier » dans l’email | Ouverture page (sans connexion) → Confirmation annulation → Validation | CLI-21, CLI-18, NFR-CLI-10 |
| **Modification de créneau** | Clic sur « Modifier » (espace client ou lien email) | Affichage créneaux disponibles → Sélection nouveau créneau → Confirmation | CLI-19, CLI-21 |
| **Consultation « Mes RDV »** | Connexion à l’espace client | Accès « Mes RDV » → Liste à venir et historique → Détail, Annuler, Modifier | CLI-16, CLI-17, CLI-18, CLI-19, CLI-20 |
| **Conflit de dates** | Réservation d’un créneau qui chevauche un autre RDV | Détection chevauchement → Message d’alerte → Choix d’un autre créneau | CLI-27, CLI-26 |

Ce récapitulatif sert de base pour les tests d’acceptation et la validation des parcours par les équipes produit et QA.

### 10.9 Exemples de messages (référence)

| Contexte | Message type (référence) |
|----------|---------------------------|
| **Confirmation de réservation** | « Votre rendez-vous est confirmé. [Date], [Heure] — [Service] — [Professionnel], [Lieu]. Vous recevrez un rappel la veille. [Lien Ajouter à mon agenda] [Lien Annuler ou modifier] » |
| **Rappel** | « Rappel : votre rendez-vous demain [Date] à [Heure] — [Service] — [Professionnel], [Lieu]. [Lien Annuler ou modifier] » |
| **Annulation par le client** | « Votre rendez-vous du [Date] à [Heure] a bien été annulé. Vous pouvez reprendre un créneau via [lien]. » |
| **Créneau non disponible** | « Ce créneau n’est plus disponible. Veuillez choisir un autre créneau. » |
| **Conflit de dates** | « Vous avez déjà un rendez-vous à cette date/heure ([Professionnel], [Service]). Choisissez un autre créneau. » |
| **Lien expiré** | « Ce lien a expiré. Connectez-vous pour annuler ou modifier votre RDV, ou contactez le professionnel. » |

Les libellés exacts et la personnalisation (logo, texte) sont définis par le professionnel dans son espace (modèles de messages). La plateforme fournit les variables (date, heure, service, professionnel, lieu, lien).

### 10.10 Priorisation détaillée (référence backlog)

| Priorité | Id besoins | Description courte |
|----------|------------|--------------------|
| **P0** | CLI-01, CLI-02, CLI-04, CLI-05, CLI-07, CLI-08, CLI-09, CLI-22, CLI-23, CLI-18, CLI-21 | Accès, réservation guest, confirmation, rappels, annulation (lien email), ajout agenda. |
| **P0** | CLI-29, CLI-30, CLI-32, NFR-CLI-01 à NFR-CLI-07, NFR-CLI-09 à NFR-CLI-11 | Parcours court, responsive, messages d’erreur, performance, dispo, RGPD, lien sécurisé, paiement. |
| **P1** | CLI-03, CLI-11 à CLI-15, CLI-16 à CLI-20, CLI-26, CLI-19, CLI-24, CLI-25, CLI-27, CLI-28, CLI-10 | Compte client, mes RDV, annulation/modification depuis l’espace, modification créneau, notifications pro, conflits, multi-pros, paiement. |
| **P1** | NFR-CLI-08, NFR-CLI-12 à NFR-CLI-14, NFR-CLI-31 | Authentification, utilisabilité, pré-remplissage, accessibilité. |
| **P2** | CLI-06, préférences notification granulaires, export historique | Choix praticien, préférences avancées, export. |

Cette priorisation est alignée avec le § 6 Priorisation MoSCoW et peut être utilisée pour le découpage des sprints et le backlog produit.

### 10.11 Correspondance besoins / user stories

| Besoin (Id) | User story (référence) |
|-------------|-------------------------|
| CLI-01, CLI-02, CLI-04, CLI-05, CLI-07, CLI-08 | US-CLI-01, US-CLI-02, US-CLI-03 |
| CLI-09, CLI-22, CLI-23 | US-CLI-04 |
| CLI-21, CLI-18, CLI-19 | US-CLI-05 |
| CLI-11, CLI-12, CLI-16, CLI-17, CLI-18, CLI-19 | US-CLI-06, US-CLI-07, US-CLI-08 |
| CLI-24, CLI-27 | US-CLI-09, US-CLI-10 |

Cette correspondance permet de tracer les besoins jusqu’aux user stories et aux critères d’acceptation des sprints.

### 10.12 Synthèse exécutive (Clients)

Le public **Clients** du service JayRDV regroupe les personnes qui prennent rendez-vous auprès des professionnels (B2C). Les besoins se structurent en **quatre blocs** : (1) **Accès à la réservation** — avec ou sans compte (guest), choix du service et du créneau, formulaire, confirmation, ajout à l’agenda, paiement optionnel ; (2) **Compte client** — inscription, connexion, profil, préférences, « Mes RDV », annulation et modification depuis l’espace ; (3) **Rappels et notifications** — confirmation à la réservation, rappels automatiques, notification en cas d’annulation ou de modification par le professionnel, alerte désistement ; (4) **Expérience et accessibilité** — parcours court (objectif ~30 s), responsive, messages d’erreur clairs, conformité WCAG. La **priorisation MVP** retient en Must have la réservation guest, la confirmation et les rappels, l’annulation via lien dans l’email, l’ajout à l’agenda et les critères de performance et de RGPD ; le compte client et « Mes RDV » sont en Should have. Les **dépendances** principales sont les Professionnels (exposition des créneaux et des services), l’Utilisateur non connecté (parcours guest partagé) et la plateforme (Miyauth, Miyunotify, Miyubooking, KindMother, WorrySentinel).

| Bloc | Must have | Should have | Could have |
|------|-----------|-------------|------------|
| **Réservation** | Guest, choix service/créneau, formulaire, confirmation, ajout agenda, rappels, annulation (lien email) | Compte client, pré-remplissage, modification créneau, paiement en ligne | Choix praticien, export historique |
| **Mes RDV** | — | Liste à venir, détail, annulation/modification depuis l’espace, historique, vue agrégée | Préférences notification granulaires |
| **Notifications** | Confirmation, rappels | Notification annulation/modification pro, alerte désistement | — |
| **Expérience** | Parcours court, responsive, message d’erreur | Accessibilité WCAG, pré-remplissage | — |
| **NFR** | Performance, dispo, RGPD, lien sécurisé, paiement sécurisé | Authentification, utilisabilité | — |

*Ce document est aligné avec le [benchmark des fonctionnalités des solutions de réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md) et avec le [document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md). Toute évolution des besoins (ajout de capacités, modification des parcours ou de la priorisation) doit être répercutée dans le document [Parcours, capacités et livrables](./Clients%20-%20Parcours%20Capacites%20Livrables.md) et dans les spécifications techniques.*

**Résumé des sections** : § 1 Profil et personas — § 2 Besoins fonctionnels (accès réservation, compte client, mes RDV, rappels, multi-pros, expérience) — § 3 Besoins non fonctionnels — § 4 Parcours détaillés (guest, compte, annulation, conflit) — § 5 Pain points et opportunités — § 6 Priorisation MoSCoW — § 7 Dépendances et interfaces — § 8 User stories — § 9 Cas limites et règles métier — § 10 Glossaire, références, annexes (matrice, critères MVP, index, checklist, dépendances techniques, récap parcours, exemples messages, priorisation backlog, correspondance US, synthèse exécutive).

**Points de vigilance** : (1) Réservation guest obligatoire pour le MVP — ne pas imposer la création de compte. (2) Lien d’annulation dans l’email — sécurisé et temporaire. (3) Créneaux en temps réel — pas de double réservation. (4) Parcours court — objectif < 60 s. (5) Rappels systématiques — objectif réduction no-show (référence marché : division par 5).

**Références croisées** : [Professionnels — Analyse des besoins](../Professionnels/Professionnels%20-%20Analyse%20des%20besoins.md) (exposition des créneaux, confirmation, rappels) ; [Utilisateur non connecté — Analyse des besoins](../UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md) (parcours guest partagé) ; [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md) (benchmark) ; [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md) (vision, principes).

**Historique des versions** : v1.0 (2026-01-31) — Création du document ; analyse des besoins du public Clients ; priorisation MoSCoW ; user stories ; cas limites ; annexes (matrice, critères MVP, checklist, dépendances techniques, récap parcours, exemples messages, priorisation backlog, correspondance US, synthèse exécutive, points de vigilance, références croisées).

**Validation** : Ce document a été rédigé dans le cadre de la construction de la structure par public du service JayRDV. Il constitue la référence produit pour le public Clients et doit être maintenu à jour en cas d’évolution des besoins ou des priorités (backlog, roadmap). Les critères d’acceptation détaillés et les spécifications techniques sont à documenter dans les livrables associés (Parcours, capacités et livrables ; spécifications techniques).

**Métriques cibles (rappel)** : Taux de complétion réservation > 70 % ; temps moyen de réservation < 60 s ; réduction du taux de no-show avec rappels (référence : division par 5) ; satisfaction client (NPS) suivie annuellement.

**Audience** : Équipes produit, conception (UX/UI), développement, QA ; parties prenantes du service JayRDV. Document de référence pour le backlog, les user stories et les critères d’acceptation du public Clients.

**Mots-clés** : JayRDV, Clients, réservation, guest, compte client, Mes RDV, confirmation, rappels, annulation, parcours court, RGPD, priorisation MoSCoW, user stories, MVP.

**Document lié** : [Clients — Parcours, capacités et livrables](./Clients%20-%20Parcours%20Capacites%20Livrables.md) — parcours détaillés, livrables (page réservation, espace client, Mes RDV), critères d’acceptation par écran, cas limites.

**Note** : La présente analyse des besoins Clients est cohérente avec l’analyse des besoins Professionnels (exposition des créneaux, confirmation, rappels) et avec l’analyse des besoins Utilisateur non connecté (parcours guest). Les évolutions fonctionnelles (paiement en ligne, modification de créneau par le client, conflits de dates multi-pros) sont priorisées dans le § 6 et le § 10.4 (critères MVP).

---

**Document** : Clients — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit — référence pour le public Clients

*Fin du document — Clients — Analyse des besoins (JayRDV).*

---
