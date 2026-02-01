# Utilisateur non connecté — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du public cible **Utilisateur non connecté** pour le service JayRDV. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels liés à l’**accès sans compte** à la prise de rendez-vous (Façade publique gouvernée) : parcours guest, limites, passerelles vers inscription ou connexion. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md), [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md), [Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md).

## Portée / Scope

- **Public** : Toute personne accédant à la **page de réservation** du professionnel (lien ou widget) **sans compte** ni authentification (parcours guest).
- **Périmètre** : Tous les besoins liés à l’accès sans compte : affichage des services et créneaux, formulaire de réservation (nom, email, téléphone), confirmation, rappels, lien annulation/modification, passerelles vers création de compte ou connexion.
- **Hors périmètre** : Spécifications techniques d’implémentation (API, schémas) ; spécifications des espaces dédiés (Professionnels, Clients avec compte) — traitées dans leurs propres documents.

---

## 1. Profil du public et personas

### 1.1 Définition du public

L’**utilisateur non connecté** est toute personne qui accède à la **page de réservation** du professionnel (lien partagé ou widget sur le site du pro) **sans compte** ni authentification. Il bénéficie d’un **parcours guest** : il peut réserver un RDV en saisissant uniquement nom, email et téléphone ; il reçoit une confirmation par email (et/ou SMS) avec un lien « Annuler ou modifier le RDV ». Il n’a pas accès à l’espace « Mes RDV » ni au profil client tant qu’il ne crée pas de compte ou ne se connecte pas. L’objectif est de **réduire la friction** (pas d’obligation de créer un compte pour réserver) tout en offrant une **passerelle** vers le compte client pour ceux qui souhaitent centraliser leurs RDV.

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Client occasionnel** | Prend 1 à 2 RDV par an chez un pro ; ne souhaite pas créer de compte. | Réserver rapidement, recevoir une confirmation et un rappel, ne pas oublier le RDV. | Obligation de créer un compte, trop de champs obligatoires, processus long. |
| **Curieux / première visite** | Découvre le professionnel via un lien (email, site, réseaux sociaux) ; première réservation. | Voir les services et les créneaux disponibles, réserver en quelques clics. | Pas d’info claire sur les étapes, pas de rappel, risque d’oubli. |
| **Client pressé (mobile)** | Réserve depuis son smartphone ; préfère la rapidité. | Réserver en moins d’une minute, recevoir un rappel SMS, ajouter le RDV à son agenda. | Site non responsive, pas de lien « Ajouter à l’agenda », pas de SMS. |
| **Client méfiant** | Ne souhaite pas donner trop d’infos ni créer de compte. | Saisie minimale (nom, email, téléphone), pas de mot de passe. | Trop de champs obligatoires, demande de création de compte. |
| **Futur client avec compte** | Va créer un compte plus tard pour centraliser ses RDV. | Réserver maintenant sans compte ; option « Créer un compte » visible mais non obligatoire. | Pas de passerelle claire vers la création de compte après réservation. |

### 1.3 Contexte d’usage

- **Fréquence** : Visite ponctuelle (une réservation) ou première visite avant de créer un compte.
- **Appareils** : Mobile prioritaire pour la réservation (lien reçu par email ou trouvé sur le site du pro) ; desktop pour la planification.
- **Concurrence** : Appel téléphonique, email au pro, autres solutions imposant la création de compte ; attente d’un **parcours court** et **sans friction**.

### 1.4 Intentions types (sans compte)

| Intention | Comportement attendu | Besoins principaux |
|-----------|----------------------|---------------------|
| **Réserver** | Accéder au lien du pro, choisir un service et un créneau, saisir nom/email/téléphone, confirmer. | UNC-01 à UNC-10. |
| **S’informer** | Voir les services proposés et les créneaux disponibles sans s’engager. | UNC-01, UNC-02, UNC-03, UNC-04. |
| **Annuler ou modifier** | Cliquer sur le lien dans l’email de confirmation pour annuler ou modifier le RDV sans se connecter. | UNC-11, UNC-12. |
| **Créer un compte (optionnel)** | Voir une option « Créer un compte » ou « Se connecter » pour accéder à l’espace client après réservation ou avant. | UNC-13, UNC-14. |

---

## 2. Besoins fonctionnels

### 2.1 Accès à la page de réservation (sans compte)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-01 | Accès au lien de réservation sans compte | Pouvoir accéder à la page de réservation du professionnel via un lien partagé (email, site, réseaux sociaux) sans créer de compte ni se connecter. | Page s’affiche sans authentification ; pas de redirection forcée vers connexion ou inscription. |
| UNC-02 | Accès au widget sans compte | Pouvoir accéder à la prise de rendez-vous via un widget intégré sur le site du professionnel (iframe, overlay) sans compte. | Widget s’affiche ; services et créneaux accessibles ; réservation possible sans compte. |
| UNC-03 | Visibilité des services | Voir la liste des services proposés par le professionnel (nom, durée, tarif optionnel) sans compte. | Liste des services actifs affichée ; pas de données privées du pro (agenda détaillé, autres clients). |
| UNC-04 | Visibilité des créneaux disponibles | Voir les créneaux disponibles (date, heure) pour le service choisi, en temps réel, sans compte. | Créneaux affichés ; mise à jour en temps réel (pas de créneau déjà pris) ; pas d’accès aux RDV des autres clients. |
| UNC-05 | Pas d’obligation de créer un compte | Réserver un RDV sans créer de compte ; saisie minimale (nom, email, téléphone). | Formulaire de réservation avec champs obligatoires (nom, email, téléphone) uniquement ; pas de mot de passe ; confirmation envoyée à l’email saisi. |
| UNC-06 | Choix du praticien (si multi-praticiens) | Si le professionnel a plusieurs praticiens, pouvoir choisir un praticien ou « Premier disponible » sans compte. | Liste des praticiens ou option « Premier disponible » ; créneaux filtrés selon le choix. |

### 2.2 Formulaire de réservation (guest)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-07 | Formulaire minimal | Saisir les informations requises : nom, email, téléphone ; remarque optionnelle. | Champs obligatoires : nom, email, téléphone ; validation du format email ; remarque optionnelle. |
| UNC-08 | Confirmation à l’écran | Après validation, afficher une page de confirmation (récapitulatif du RDV). | Message « Votre RDV est confirmé » ; récapitulatif (date, heure, service, lieu) ; pas d’erreur si double clic. |
| UNC-09 | Confirmation par email (et/ou SMS) | Recevoir une confirmation par email (et/ou SMS) avec récapitulatif, lien « Ajouter à mon agenda » et lien « Annuler ou modifier le RDV ». | Email/SMS envoyé automatiquement ; contenu : date, heure, service, lieu, professionnel ; lien « Ajouter à mon agenda » ; lien « Annuler ou modifier le RDV ». |
| UNC-10 | Ajout du RDV à l’agenda client | Pouvoir ajouter le RDV à son agenda (Google, Outlook, Apple, iCal) depuis l’email de confirmation. | Lien « Ajouter à mon agenda » dans l’email ; génération de fichier iCal ou lien vers Google/Outlook/Apple. |

### 2.3 Annulation et modification (sans compte)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-11 | Lien d’annulation dans l’email | Cliquer sur un lien dans l’email de confirmation pour annuler le RDV sans se connecter. | Lien unique et temporaire (token sécurisé) ; page d’annulation avec récap RDV et bouton « Annuler » ; confirmation avant annulation ; email de confirmation d’annulation. |
| UNC-12 | Lien de modification dans l’email | Cliquer sur un lien dans l’email de confirmation pour modifier le créneau (reprendre un autre créneau) sans se connecter. | Lien unique et temporaire ; page avec créneaux disponibles ; sélection d’un nouveau créneau ; confirmation ; email de confirmation de modification. |
| UNC-12b | Expiration du lien | Le lien d’annulation/modification expire après une durée configurable (ex. J-1 ou 7 jours) ou après utilisation. | Message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. » si lien invalide. |

### 2.4 Rappels et notifications

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-13 | Rappel automatique | Recevoir un rappel (email et/ou SMS) la veille ou quelques heures avant le RDV. | Envoi au délai configuré par le professionnel ; contenu : date, heure, lieu, service ; lien annulation/modification optionnel. |
| UNC-14 | Notification si le pro annule ou modifie | Être notifié si le professionnel annule ou modifie le RDV. | Email/SMS avec motif optionnel ; proposition de reprendre un créneau si modification par le pro. |

### 2.5 Passerelles vers compte client

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-15 | CTA « Créer un compte » visible | Voir une option « Créer un compte » sur la page de réservation ou après confirmation (non obligatoire). | Lien ou bouton « Créer un compte » visible ; redirection vers formulaire d’inscription ; pas d’obligation de cliquer pour réserver. |
| UNC-16 | CTA « Se connecter » visible | Voir une option « Se connecter » sur la page de réservation (non obligatoire). | Lien ou bouton « Se connecter » visible ; redirection vers page de connexion ; après connexion, retour possible à la page de réservation (contexte conservé). |
| UNC-17 | Retour au contexte après connexion | Après connexion ou inscription, pouvoir revenir au contexte (page de réservation du pro) pour réserver avec pré-remplissage. | Redirection post-connexion/inscription vers la page de réservation ou vers « Mes RDV » selon contexte. |
| UNC-18 | Proposition de créer un compte après réservation | Après une réservation guest, proposer (optionnel) de créer un compte pour retrouver ce RDV dans « Mes RDV » et réserver plus vite la prochaine fois. | Message ou lien « Créer un compte pour retrouver vos RDV » après confirmation ; non bloquant. |

### 2.6 Limites et gouvernance (Façade publique)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-19 | Lecture seule des créneaux | L’utilisateur non connecté ne voit que les **créneaux disponibles** ; pas d’accès à l’agenda détaillé du professionnel ni aux RDV des autres clients. | Affichage des seules disponibilités ; pas de fuite de données (noms des autres clients, blocages personnels du pro). |
| UNC-20 | Pas d’accès à « Mes RDV » sans compte | Sans compte, pas d’accès à la liste des RDV du client (pas d’historique centralisé). | L’utilisateur non connecté ne peut pas accéder à « Mes RDV » ; message « Connectez-vous pour voir vos RDV » si tentative d’accès. |
| UNC-21 | Données personnelles minimales | Les données saisies (nom, email, téléphone) sont utilisées uniquement pour la réservation, la confirmation et les rappels ; pas de revente ni d’usage non autorisé (RGPD). | Consentement et informations sur les données ; droits d’accès, rectification, effacement ; durée de conservation configurable. |
| UNC-22 | Sécurité du lien d’annulation/modification | Le lien dans l’email est unique, temporaire et non devinable ; invalidé après utilisation ou expiration. | Token sécurisé ; vérification côté serveur ; message clair si lien expiré ou déjà utilisé. |

### 2.7 Paiement en ligne (si activé par le pro)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-23 | Paiement sans compte | Si le professionnel a activé le paiement en ligne, pouvoir payer (intégral ou acompte) à la réservation sans créer de compte. | Page de paiement sécurisée après saisie des infos (nom, email, téléphone) ; confirmation du paiement et du RDV ; reçu par email. |

### 2.8 Parcours court et expérience

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-24 | Parcours en quelques clics | Réduire le nombre d’étapes pour réserver (objectif marché : ~30 secondes à 1 minute). | Maximum 4 à 5 étapes : Choix service → Choix créneau → Saisie infos → Confirmation (et paiement si activé). |
| UNC-25 | Responsive (mobile, tablette, desktop) | La page de réservation est utilisable sur tous les appareils. | Mise en page adaptée ; boutons et champs accessibles ; pas de perte de fonctionnalité sur mobile. |
| UNC-26 | Message d’erreur clair | En cas de créneau déjà pris ou d’erreur, afficher un message clair et proposer une action. | Message « Ce créneau n’est plus disponible. Veuillez choisir un autre créneau. » ; proposition de rafraîchir les créneaux. |
| UNC-27 | Accessibilité (WCAG) | La page de réservation respecte les critères d’accessibilité (navigation clavier, lecteurs d’écran, contrastes). | Conformité WCAG 2.1 niveau AA. |

---

## 3. Besoins non fonctionnels

### 3.1 Performance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-01 | Temps de chargement de la page de réservation | La page (services et créneaux) se charge en moins de 3 secondes (réseau standard). |
| NFR-UNC-02 | Disponibilité des créneaux en temps réel | Les créneaux affichés sont à jour ; après une réservation par un autre client, le créneau disparaît en moins de 2 secondes. |
| NFR-UNC-03 | Temps de confirmation | Après clic « Confirmer », la confirmation s’affiche et l’email/SMS est envoyé en moins de 5 secondes. |

### 3.2 Disponibilité et fiabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-04 | Disponibilité | La page de réservation est disponible 99,5 % du temps (hors maintenance annoncée). |
| NFR-UNC-05 | Sauvegarde des réservations | Une réservation confirmée est enregistrée de façon durable ; pas de perte après confirmation. |
| NFR-UNC-06 | Idempotence | Si l’utilisateur clique deux fois sur « Confirmer », une seule réservation est créée. |

### 3.3 Sécurité et confidentialité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-07 | Données personnelles (RGPD) | Consentement, droit d’accès, de rectification, d’effacement ; durée de conservation configurable ; données utilisées uniquement pour la réservation et les rappels. |
| NFR-UNC-08 | Lien d’annulation/modification sécurisé | Lien unique, temporaire, non devinable ; invalidé après utilisation ou expiration (ex. 7 jours ou J-1). |
| NFR-UNC-09 | Paiement (si activé) | Paiement sécurisé (PCI, 3D Secure) ; pas de stockage des données de carte côté plateforme (ou conformité PCI-DSS). |

### 3.4 Utilisabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-10 | Parcours intuitif | Un utilisateur non familiarisé peut réserver un RDV en moins de 5 étapes sans aide. |
| NFR-UNC-11 | Messages en français | Interface et messages (erreur, confirmation) en français par défaut. |

---

## 4. Parcours détaillés et scénarios

### 4.1 Scénario : Réservation guest (premier RDV)

1. L’utilisateur reçoit un lien de réservation du professionnel (email, site, réseaux sociaux) ou clique sur le widget sur le site du pro.
2. La page s’affiche sans demande de connexion. Il voit la liste des services (ex. « Consultation 30 min », « Séance 1 h »).
3. Il sélectionne « Consultation 30 min ». Les créneaux disponibles s’affichent (ex. Jeudi 6 février : 9h, 10h, 14h, 15h).
4. Il sélectionne « Jeudi 6 février, 10h00 ». Le formulaire s’affiche : nom, email, téléphone, remarque (optionnel).
5. Il remplit et clique sur « Confirmer ». Une page de confirmation s’affiche : « Votre RDV est confirmé. Vous allez recevoir un email de confirmation. »
6. Il reçoit un email avec le récapitulatif, un lien « Ajouter à mon agenda » et un lien « Annuler ou modifier le RDV ».
7. La veille du RDV, il reçoit un rappel par email (ou SMS si configuré par le pro).
8. Il n’a pas créé de compte ; il ne peut pas accéder à « Mes RDV ». S’il souhaite centraliser ses RDV, il peut cliquer sur « Créer un compte » (optionnel).

**Besoins couverts** : UNC-01, UNC-03, UNC-04, UNC-05, UNC-07, UNC-08, UNC-09, UNC-10, UNC-13, UNC-15, UNC-18, UNC-24, UNC-25.

### 4.2 Scénario : Annulation depuis l’email (sans compte)

1. L’utilisateur ouvre l’email de confirmation et clique sur « Annuler ou modifier le RDV ».
2. Une page s’ouvre (sans connexion) avec le récapitulatif du RDV et deux boutons : « Annuler le RDV » et « Modifier le créneau ».
3. Il clique sur « Annuler le RDV ». Un message de confirmation s’affiche : « Êtes-vous sûr de vouloir annuler ? » (motif optionnel). Il valide.
4. Le RDV est annulé ; une page de confirmation s’affiche : « Votre RDV a bien été annulé. » Il reçoit un email de confirmation d’annulation.
5. Le créneau est libéré côté professionnel.

**Besoins couverts** : UNC-11, UNC-12b, UNC-22, NFR-UNC-08.

### 4.3 Scénario : Passage vers compte client (optionnel)

1. Après avoir réservé en guest, l’utilisateur voit un message : « Créer un compte pour retrouver vos RDV et réserver plus vite la prochaine fois. »
2. Il clique sur « Créer un compte ». Il est redirigé vers le formulaire d’inscription (email, mot de passe, nom, téléphone).
3. Après inscription, il est redirigé vers « Mes RDV » ou vers la page de réservation du pro (contexte conservé). Son RDV guest peut être rattaché à son compte si l’email correspond (optionnel selon implémentation).
4. Lors de sa prochaine réservation, il peut se connecter ; le formulaire sera pré-rempli.

**Besoins couverts** : UNC-15, UNC-17, UNC-18.

---

## 5. Pain points et opportunités

### 5.1 Pain points

| Pain point | Impact | Besoin associé |
|------------|--------|-----------------|
| Obligation de créer un compte | Friction ; abandon du parcours. | UNC-05 (réservation sans compte). |
| Parcours trop long | Trop d’étapes ; abandon. | UNC-24 (parcours court). |
| Pas de rappel | Oubli du RDV ; no-show. | UNC-13 (rappels). |
| Créneau déjà pris | L’utilisateur choisit un créneau qui vient d’être pris ; erreur ou frustration. | UNC-04, NFR-UNC-02 (temps réel), UNC-26 (message d’erreur). |
| Pas d’ajout à l’agenda | L’utilisateur doit ressaisir le RDV dans son agenda. | UNC-10 (lien « Ajouter à mon agenda »). |
| Annulation compliquée | Pas de lien dans l’email ; obligation d’appeler. | UNC-11 (lien d’annulation dans l’email). |
| Lien expiré sans explication | Message technique ou absent. | UNC-12b, NFR-UNC-08 (message clair). |

### 5.2 Opportunités

| Opportunité | Description | Besoin associé |
|-------------|-------------|-----------------|
| Parcours guest systématique | Réserver sans compte ; taux de conversion plus élevé (référence marché). | UNC-05, UNC-24. |
| Rappels systématiques | Réduction des no-show ; satisfaction client et pro. | UNC-13. |
| Lien annulation/modification | Autonomie de l’utilisateur ; moins d’appels pour le pro. | UNC-11, UNC-12. |
| Passerelle vers compte client | Option « Créer un compte » visible mais non obligatoire ; fidélisation. | UNC-15, UNC-16, UNC-18. |
| Ajout à l’agenda | Moins d’oubli ; meilleure expérience. | UNC-10. |

---

## 6. Priorisation des besoins (MoSCoW)

### 6.1 Must have (indispensable)

- UNC-01, UNC-02, UNC-03, UNC-04, UNC-05, UNC-07, UNC-08, UNC-09 (accès, services, créneaux, formulaire guest, confirmation).
- UNC-10 (ajout à l’agenda).
- UNC-11, UNC-12, UNC-12b (lien annulation/modification, expiration).
- UNC-13 (rappels).
- UNC-19, UNC-21, UNC-22 (limites Façade publique, RGPD, sécurité lien).
- UNC-24, UNC-25, UNC-26 (parcours court, responsive, message d’erreur).
- NFR-UNC-01 à NFR-UNC-08, NFR-UNC-10, NFR-UNC-11 (performance, dispo, RGPD, lien sécurisé, utilisabilité).

### 6.2 Should have (important)

- UNC-06 (choix du praticien).
- UNC-14 (notification si le pro annule ou modifie).
- UNC-15, UNC-16, UNC-17, UNC-18 (passerelles vers compte client).
- UNC-23 (paiement sans compte).
- UNC-27 (accessibilité WCAG).
- NFR-UNC-09 (paiement sécurisé).

### 6.3 Could have (souhaitable)

- Proposition de créer un compte après réservation (message personnalisé).
- Rappel SMS en plus de l’email (selon config pro).

### 6.4 Won’t have (hors périmètre ou report)

- Accès à « Mes RDV » sans compte (impossible par définition).
- Besoins spécifiques aux Professionnels ou aux Clients avec compte.

---

## 7. Dépendances et interfaces

### 7.1 Dépendances

| Dépendance | Description |
|------------|-------------|
| **Professionnels** | L’utilisateur non connecté dépend des créneaux et des services exposés par les professionnels (lien, widget). Les professionnels exposent la page de réservation publique. |
| **Clients** | Le parcours guest est **partagé** avec le public Clients (même page de réservation) ; la distinction est « avec compte » (pré-remplissage, Mes RDV) vs « sans compte » (formulaire guest uniquement). |
| **Plateforme** | Miyunotify (confirmations, rappels), Miyubooking (créneaux, RDV), KindMother (persistance), WorrySentinel (sécurité, RGPD). |

### 7.2 Interfaces

| Interface | Flux | Besoin UNC |
|-----------|------|------------|
| Utilisateur non connecté → Professionnel | Réservation guest ; données (nom, email, téléphone). | UNC-01 à UNC-10, UNC-23. |
| Professionnel → Utilisateur non connecté | Confirmation, rappels, notification annulation/modification. | UNC-09, UNC-13, UNC-14. |
| Utilisateur non connecté → Plateforme | Lien annulation/modification (token) ; passerelle vers compte client. | UNC-11, UNC-12, UNC-15 à UNC-18. |

---

## 8. User stories (format standard)

### 8.1 Réservation guest

- **US-UNC-01** — En tant qu’**utilisateur sans compte**, je veux **accéder à la page de réservation** du professionnel via un lien **afin de** réserver un RDV sans créer de compte. *Critères* : Page s’affiche avec services et créneaux ; pas de redirection vers connexion.
- **US-UNC-02** — En tant qu’**utilisateur sans compte**, je veux **choisir un service et un créneau** **afin de** réserver le RDV qui me convient. *Critères* : Liste des services ; créneaux en temps réel ; pas de double réservation.
- **US-UNC-03** — En tant qu’**utilisateur sans compte**, je veux **saisir uniquement mon nom, email et téléphone** **afin de** réserver sans créer de compte. *Critères* : Formulaire minimal ; pas de mot de passe ; confirmation par email.
- **US-UNC-04** — En tant qu’**utilisateur sans compte**, je veux **recevoir une confirmation** et **un rappel** avant le RDV **afin de** ne pas oublier. *Critères* : Confirmation immédiate (email/SMS) ; rappel 24h et/ou 2h avant ; lien « Ajouter à mon agenda » ; lien « Annuler ou modifier ».
- **US-UNC-05** — En tant qu’**utilisateur sans compte**, je veux **annuler ou modifier mon RDV** via un lien dans l’email **afin de** ne pas avoir à appeler. *Critères* : Lien unique et sécurisé ; annulation/modification sans connexion ; message clair si lien expiré.

### 8.2 Passerelles

- **US-UNC-06** — En tant qu’**utilisateur sans compte**, je veux **voir l’option « Créer un compte »** (sans être obligé de cliquer) **afin de** pouvoir centraliser mes RDV plus tard. *Critères* : Lien visible ; redirection vers inscription ; pas d’obligation pour réserver.
- **US-UNC-07** — En tant qu’**utilisateur sans compte**, je veux **voir l’option « Se connecter »** **afin de** réserver avec pré-remplissage si j’ai déjà un compte. *Critères* : Lien visible ; redirection vers connexion ; retour possible à la page de réservation.

---

## 9. Cas limites et règles métier

### 9.1 Règles métier

| Règle | Description |
|-------|-------------|
| **Pas de compte obligatoire** | L’utilisateur non connecté peut réserver sans créer de compte ; saisie minimale (nom, email, téléphone). |
| **Créneau unique** | Un créneau ne peut être réservé qu’une seule fois ; vérification en temps réel à la confirmation. |
| **Lien annulation/modification** | Valide jusqu’à une date limite (ex. J-1) ou jusqu’à utilisation ; après, message « Connectez-vous ou contactez le professionnel ». |
| **Données minimales** | Les données (nom, email, téléphone) sont utilisées uniquement pour la réservation, les confirmations et les rappels ; RGPD. |
| **Façade publique** | L’utilisateur non connecté ne voit que les créneaux disponibles ; pas d’accès à l’agenda détaillé du pro ni aux RDV des autres clients. |

### 9.2 Cas limites

| Cas | Comportement attendu |
|-----|----------------------|
| **Deux utilisateurs réservent le même créneau** | Un seul obtient la réservation ; l’autre reçoit « Ce créneau n’est plus disponible » et peut en choisir un autre. |
| **Utilisateur clique deux fois sur « Confirmer »** | Une seule réservation est créée ; message de succès une seule fois ; pas de double email. |
| **Lien d’annulation expiré** | Message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. » |
| **Email invalide** | Validation du format email à la saisie ; message d’erreur si invalide. |
| **Tentative d’accès à « Mes RDV » sans compte** | Redirection vers page de connexion ou message « Connectez-vous pour voir vos RDV ». |

### 9.3 Métriques de succès

| Métrique | Description | Cible (exemple) |
|----------|-------------|------------------|
| **Taux de complétion (guest)** | % de parcours de réservation guest complétés (de l’arrivée sur la page à la confirmation). | > 70 % |
| **Temps moyen de réservation (guest)** | Délai entre l’arrivée sur la page et la confirmation. | < 60 secondes |
| **Taux de passage vers compte client** | % d’utilisateurs guest qui créent un compte après réservation (optionnel). | Suivi |
| **Taux de no-show (guest)** | % de RDV confirmés non honorés (sans annulation) ; objectif réduction avec rappels. | Réduction avec rappels (référence : division par 5) |

---

## 10. Glossaire et références

### 10.1 Glossaire (extrait)

| Terme | Définition |
|-------|------------|
| **Utilisateur non connecté** | Personne qui accède à la page de réservation (lien ou widget) sans compte ni authentification ; parcours guest. |
| **Parcours guest** | Prise de RDV sans création de compte ; saisie minimale (nom, email, téléphone) ; confirmation par email. |
| **Façade publique** | Page de réservation exposée par le professionnel (lien ou widget) ; accès sans authentification ; affichage des seules disponibilités. |
| **Lien d’annulation** | Lien unique et temporaire dans l’email de confirmation permettant d’annuler (ou modifier) le RDV sans se connecter. |

### 10.2 Références documentaires

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md)
- [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md)
- [Utilisateur non connecté — Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md)
- [Public Clients](../Clients/_index.md) (parcours guest partagé) | [Public Professionnels](../Professionnels/_index.md)

### 10.3 Matrice besoins UNC / fonctionnalités marché

| Domaine benchmark | Besoins JayRDV (Utilisateur non connecté) | Priorité |
|-------------------|--------------------------------------------|----------|
| Prise de RDV client (sans compte) | UNC-01 à UNC-10 (accès, services, créneaux, formulaire guest, confirmation, ajout agenda) | Must |
| Notifications et rappels | UNC-09, UNC-13, UNC-14 (confirmation, rappels, notification annulation pro) | Must |
| Annulation/modification (lien email) | UNC-11, UNC-12, UNC-12b, UNC-22 (lien sécurisé, expiration) | Must |
| Passerelles vers compte | UNC-15 à UNC-18 (créer un compte, se connecter, retour contexte, proposition après réservation) | Should |
| Expérience | UNC-24 à UNC-27 (parcours court, responsive, message d’erreur, accessibilité) | Must / Should |
| Paiement sans compte | UNC-23, NFR-UNC-09 | Should |
| Limites Façade publique | UNC-19 à UNC-22 (créneaux uniquement, pas Mes RDV, RGPD, lien sécurisé) | Must |

### 10.4 Critères d’acceptation synthétiques (MVP Utilisateur non connecté)

Pour la première version livrable (MVP) du public Utilisateur non connecté, les critères d’acceptation globaux suivants sont retenus :

- **Accès** : L’utilisateur peut accéder au lien de réservation du professionnel (ou au widget) sans compte ; la page affiche les services et les créneaux disponibles en temps réel.
- **Réservation guest** : Formulaire minimal (nom, email, téléphone) ; pas de création de compte obligatoire ; confirmation à l’écran et par email (et/ou SMS) ; lien « Ajouter à mon agenda » et lien « Annuler ou modifier le RDV » dans l’email.
- **Rappels** : Rappel automatique (24h et/ou 2h avant) selon configuration du professionnel.
- **Annulation/modification** : Lien dans l’email fonctionne sans connexion ; token sécurisé et temporaire ; message clair si lien expiré.
- **Limites** : Affichage des seules disponibilités ; pas d’accès à « Mes RDV » sans compte ; données personnelles (RGPD) ; pas de fuite de données.
- **Passerelles** : Option « Créer un compte » et « Se connecter » visibles mais non obligatoires.
- **Parcours court** : Maximum 4 à 5 étapes ; responsive ; message d’erreur clair si créneau non disponible.

Ces critères couvrent les besoins UNC-01 à UNC-13, UNC-19 à UNC-22, UNC-24 à UNC-26 et les NFR-UNC-01 à NFR-UNC-08, NFR-UNC-10, NFR-UNC-11.

### 10.5 Index des sections (Analyse Utilisateur non connecté)

1. **Profil du public et personas** — Définition, personas (occasionnel, curieux, pressé, méfiant, futur client avec compte), contexte d’usage, intentions types.  
2. **Besoins fonctionnels** — Accès (lien, widget, services, créneaux, pas d’obligation compte), formulaire guest, confirmation, ajout agenda, annulation/modification (lien email), rappels, passerelles vers compte client, limites Façade publique, paiement, parcours court et expérience.  
3. **Besoins non fonctionnels** — Performance, disponibilité, sécurité et confidentialité, utilisabilité.  
4. **Parcours détaillés** — Réservation guest, annulation depuis l’email, passage vers compte client.  
5. **Pain points et opportunités** — Problèmes actuels ou anticipés, opportunités (guest, rappels, lien annulation, passerelle compte).  
6. **Priorisation MoSCoW** — Must, Should, Could, Won’t have.  
7. **Dépendances et interfaces** — Professionnels, Clients, Plateforme.  
8. **User stories** — Réservation guest, passerelles.  
9. **Cas limites et règles métier** — Règles (pas de compte obligatoire, créneau unique, lien annulation, RGPD, Façade publique), cas limites, métriques de succès.  
10. **Glossaire et références** — Glossaire, références documentaires, matrice besoins/fonctionnalités marché, critères MVP, index des sections.

### 10.6 Correspondance avec le public Clients

Le **parcours guest** de l’utilisateur non connecté est **identique** au parcours « Réservation guest » du public Clients (voir [Clients — Analyse des besoins](../Clients/Clients%20-%20Analyse%20des%20besoins.md) et [Clients — Parcours, capacités et livrables](../Clients/Clients%20-%20Parcours%20Capacites%20Livrables.md)). La différence est **conceptuelle** : le document « Utilisateur non connecté » se concentre sur les besoins et les limites de l’**accès sans compte** (Façade publique, pas de Mes RDV, passerelles vers compte) ; le document « Clients » couvre à la fois le parcours guest **et** le parcours avec compte (Mes RDV, profil, préférences). Les **besoins fonctionnels** du parcours guest (accès, services, créneaux, formulaire, confirmation, rappels, lien annulation/modification) sont donc **partagés** entre UNC et CLI (CLI-01, CLI-02, CLI-04, CLI-05, CLI-07, CLI-08, CLI-09, CLI-18, CLI-21, CLI-22, CLI-23). Les besoins **spécifiques** à l’utilisateur non connecté sont : UNC-15 à UNC-18 (passerelles), UNC-19 à UNC-22 (limites Façade publique, RGPD, sécurité lien), UNC-20 (pas d’accès à Mes RDV).

### 10.7 Dépendances techniques (référence)

| Besoin UNC | Composant / Kit | Rôle |
|------------|-----------------|------|
| Accès page réservation, créneaux | Miyubooking, lien pro, widget | Page publique, calcul des créneaux disponibles. |
| Formulaire guest, réservation | Miyubooking, Miyucontacts (léger) | Enregistrement RDV, données client (nom, email, téléphone). |
| Confirmation, rappels | Miyunotify | Email et SMS. |
| Lien annulation/modification | Miyubooking, token sécurisé | Lien unique temporaire, annulation sans connexion. |
| Passerelles vers compte | Miyauth, Miyuprofile | Inscription, connexion ; redirection. |
| RGPD, consentement | WorrySentinel, traçabilité | Données personnelles, droits. |
| Paiement (si activé) | Miyuinvoice ou partenaire | Paiement en ligne sans compte. |

### 10.8 Synthèse exécutive (Utilisateur non connecté)

Le public **Utilisateur non connecté** du service JayRDV regroupe les personnes qui accèdent à la **page de réservation** du professionnel (lien ou widget) **sans compte** ni authentification. Les besoins se structurent en **cinq blocs** : (1) **Accès** — lien ou widget, services et créneaux visibles en temps réel, pas d’obligation de créer un compte ; (2) **Formulaire guest** — saisie minimale (nom, email, téléphone), confirmation à l’écran et par email/SMS, lien « Ajouter à mon agenda » et lien « Annuler ou modifier le RDV » ; (3) **Annulation et modification** — lien dans l’email (token sécurisé, expiration), annulation ou modification sans connexion ; (4) **Rappels et notifications** — rappel automatique (24h et/ou 2h avant), notification si le pro annule ou modifie ; (5) **Passerelles et limites** — option « Créer un compte » et « Se connecter » (non obligatoires), pas d’accès à « Mes RDV » sans compte, Façade publique (créneaux uniquement), RGPD, sécurité du lien. La **priorisation MVP** retient en Must l’accès, la réservation guest, la confirmation et les rappels, l’annulation via lien email, l’ajout à l’agenda et les limites Façade publique ; les passerelles vers compte client et le paiement sans compte sont en Should. Le parcours guest est **partagé** avec le public Clients (même page, même flux) ; la documentation UNC met l’accent sur les besoins et limites de l’**accès sans compte**.

---

**Document** : Utilisateur non connecté — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit — référence pour le public Utilisateur non connecté

### 10.9 Checklist détaillée (parcours guest — QA)

- [ ] **Accès** : Lien de réservation du professionnel accessible sans compte ; page affiche les services et les créneaux disponibles.  
- [ ] **Choix service** : Liste des services (nom, durée, tarif si affiché) ; sélection d’un service ; passage à l’étape créneaux.  
- [ ] **Choix créneau** : Créneaux affichés en temps réel (pas de créneau déjà pris) ; sélection date/heure ; si multi-praticiens, choix du praticien ou « Premier disponible ».  
- [ ] **Formulaire** : Champs nom, email, téléphone (obligatoires) ; remarque (optionnel) ; pas de mot de passe ; pas d’obligation de créer un compte.  
- [ ] **Confirmation** : Message de succès à l’écran ; email et/ou SMS envoyé avec récapitulatif ; lien « Ajouter à mon agenda » ; lien « Annuler ou modifier le RDV ».  
- [ ] **Rappel** : Rappel envoyé au délai configuré par le pro (24h et/ou 2h avant) ; contenu : date, heure, lieu, service.  
- [ ] **Annulation** : Lien dans l’email fonctionne sans connexion ; page d’annulation avec confirmation ; motif optionnel ; email de confirmation d’annulation ; créneau libéré côté pro.  
- [ ] **Modification** : Lien dans l’email fonctionne sans connexion ; page avec créneaux disponibles ; sélection nouveau créneau ; confirmation ; email de confirmation de modification.  
- [ ] **Lien expiré** : Message clair « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. »  
- [ ] **Passerelles** : Option « Créer un compte » et « Se connecter » visibles ; pas d’obligation pour réserver.  
- [ ] **Limites** : Pas d’accès à « Mes RDV » sans compte ; affichage des seules disponibilités ; RGPD.  
- [ ] **Responsive** : Page de réservation utilisable sur mobile, tablette et desktop.

### 10.10 Récapitulatif des parcours (Utilisateur non connecté)

| Parcours | Déclencheur | Étapes clés | Besoins couverts |
|----------|-------------|-------------|------------------|
| **Réservation guest** | Clic sur lien du pro ou widget | Choix service → Choix créneau → Saisie nom/email/téléphone → Confirmation | UNC-01 à UNC-10, UNC-13, UNC-24, UNC-25 |
| **Annulation (lien email)** | Clic sur « Annuler ou modifier » dans email | Ouverture page (sans connexion) → Confirmation annulation → Validation | UNC-11, UNC-12b, UNC-22 |
| **Modification (lien email)** | Clic sur « Modifier le créneau » dans email | Affichage créneaux disponibles → Sélection nouveau créneau → Confirmation | UNC-12, UNC-12b |
| **Passage vers compte client** | Clic sur « Créer un compte » (optionnel) | Redirection inscription → Inscription → Retour contexte ou Mes RDV | UNC-15, UNC-17, UNC-18 |

### 10.11 Points de vigilance (Utilisateur non connecté)

- **Pas d’obligation de créer un compte** : Le parcours guest doit être complet et fonctionnel sans aucune étape de création de compte.
- **Créneaux en temps réel** : Vérification côté serveur à la confirmation ; pas de double réservation ; message clair si créneau plus disponible.
- **Lien annulation/modification** : Token unique, temporaire, non devinable ; expiration configurable (ex. J-1 ou 7 jours) ; invalidé après utilisation ; message clair si expiré.
- **Parcours court** : Objectif < 60 s (référence marché) ; maximum 4 à 5 étapes.
- **Rappels** : Objectif réduction no-show (référence : division par 5) ; modèles personnalisables par le pro.
- **RGPD** : Consentement et informations sur les données ; droits d’accès, rectification, effacement ; durée de conservation.
- **Façade publique** : Affichage des seules disponibilités ; pas de fuite de données (noms des autres clients, agenda détaillé du pro).

### 10.12 Correspondance besoins / user stories (Utilisateur non connecté)

| Besoin (Id) | User story (référence) |
|-------------|------------------------|
| UNC-01, UNC-02, UNC-03, UNC-04, UNC-05, UNC-07, UNC-08, UNC-09, UNC-10 | US-UNC-01, US-UNC-02, US-UNC-03, US-UNC-04 |
| UNC-11, UNC-12, UNC-12b | US-UNC-05 |
| UNC-15, UNC-16, UNC-17, UNC-18 | US-UNC-06, US-UNC-07 |

### 10.13 Priorisation détaillée (référence backlog)

| Priorité | Id besoins | Description courte |
|----------|------------|---------------------|
| **P0** | UNC-01 à UNC-05, UNC-07 à UNC-11, UNC-12, UNC-12b, UNC-13, UNC-19, UNC-21, UNC-22, UNC-24, UNC-25, UNC-26 | Accès, réservation guest, confirmation, rappels, annulation (lien email), limites Façade, parcours court, responsive, messages. |
| **P0** | NFR-UNC-01 à NFR-UNC-08, NFR-UNC-10, NFR-UNC-11 | Performance, dispo, RGPD, lien sécurisé, utilisabilité. |
| **P1** | UNC-06, UNC-14, UNC-15 à UNC-18, UNC-23, UNC-27 | Choix praticien, notification annulation pro, passerelles compte client, paiement sans compte, accessibilité. |
| **P1** | NFR-UNC-09 | Paiement sécurisé. |

### 10.14 Références croisées

- [Clients — Analyse des besoins](../Clients/Clients%20-%20Analyse%20des%20besoins.md) — parcours guest partagé (CLI-01, CLI-02, CLI-04, CLI-05, CLI-07, CLI-08, CLI-09, CLI-18, CLI-21, CLI-22, CLI-23).
- [Clients — Parcours, capacités et livrables](../Clients/Clients%20-%20Parcours%20Capacites%20Livrables.md) — livrables page réservation, confirmation, rappels, lien annulation/modification.
- [Professionnels — Analyse des besoins](../Professionnels/Professionnels%20-%20Analyse%20des%20besoins.md) — exposition des créneaux (lien, widget).
- [Utilisateur non connecté — Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md) — parcours détaillé, règles d’accès.
- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md) — vision, principes.
- [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md) — benchmark marché.

### 10.15 Historique des versions et validation

**Historique** : v1.0 (2026-01-31) — Création du document ; analyse des besoins du public Utilisateur non connecté ; priorisation MoSCoW ; user stories ; cas limites ; annexes (matrice, critères MVP, index, correspondance Clients, dépendances techniques, synthèse exécutive, checklist QA, récap parcours, points de vigilance, correspondance US, priorisation backlog, références croisées).

**Validation** : Ce document a été rédigé dans le cadre de la construction de la structure par public du service JayRDV. Il constitue la référence produit pour le public Utilisateur non connecté (parcours guest) et doit être maintenu à jour en cas d’évolution des besoins ou des priorités. Le parcours guest est **partagé** avec le public Clients ; les besoins communs sont documentés dans les deux analyses (UNC et CLI) pour traçabilité. Les spécifications techniques (Opérateurs, Kits, API) sont à documenter dans les livrables associés.

**Mots-clés** : JayRDV, Utilisateur non connecté, parcours guest, Façade publique, réservation sans compte, confirmation, rappels, lien annulation, lien modification, passerelles compte client, RGPD, priorisation MoSCoW, user stories, MVP.

**Résumé des sections** : § 1 Profil et personas — § 2 Besoins fonctionnels (accès, formulaire guest, confirmation, annulation/modification lien email, rappels, passerelles, limites Façade, paiement, parcours court) — § 3 Besoins non fonctionnels — § 4 Parcours détaillés (réservation guest, annulation depuis email, passage vers compte client) — § 5 Pain points et opportunités — § 6 Priorisation MoSCoW — § 7 Dépendances et interfaces — § 8 User stories — § 9 Cas limites et règles métier — § 10 Glossaire, références, annexes (matrice, critères MVP, index, correspondance Clients, dépendances techniques, synthèse exécutive, checklist QA, récap parcours, points de vigilance, correspondance US, priorisation backlog, références croisées, historique, validation, mots-clés).

**Document** : Utilisateur non connecté — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit — référence pour le public Utilisateur non connecté

**Audience** : Équipes produit, conception (UX/UI), développement, QA ; parties prenantes du service JayRDV. Document de référence pour le backlog, les user stories et les critères d’acceptation du parcours guest (Utilisateur non connecté).

**Note** : La présente analyse des besoins Utilisateur non connecté est **cohérente** avec l’analyse des besoins Clients (parcours guest partagé) et avec l’analyse des besoins Professionnels (exposition des créneaux via lien ou widget). Les évolutions fonctionnelles (passerelles vers compte client, paiement sans compte) sont priorisées dans le § 6 et le § 10.4 (critères MVP).

### 10.16 Annexe — Notes d’implémentation (parcours guest)

Les points suivants sont à prendre en compte lors de l’implémentation du parcours Utilisateur non connecté :

- **Identifiant de réservation** : Chaque réservation guest doit disposer d’un identifiant unique (ex. UUID) et d’un token d’annulation/modification non devinable, stocké côté serveur et transmis uniquement dans les liens email. Aucune donnée sensible ne doit figurer dans l’URL.
- **Session minimale** : Pour le parcours guest, une session technique (panier de réservation) peut être utilisée le temps de la réservation (choix service → créneau → formulaire → confirmation) ; elle ne doit pas imposer de cookie persistant ni de compte.
- **Rate limiting** : Les endpoints exposés sur la Façade publique (affichage créneaux, soumission formulaire, lien annulation) doivent être protégés par rate limiting (par IP et par lien pro) pour limiter les abus et les scrapings.
- **Accessibilité** : Les écrans du parcours guest (choix service, créneau, formulaire, confirmation, annulation) doivent respecter les critères WCAG 2.1 niveau AA (contraste, focus, labels, messages d’erreur annoncés).

**Correspondance avec le document Parcours et accès** : Les règles d’accès à la Façade publique (token, rate limiting, expiration, pas d’agenda détaillé exposé) et les critères d’acceptation par parcours sont détaillés dans [Utilisateur non connecté — Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md). Les besoins UNC-01 à UNC-26 et NFR-UNC-01 à NFR-UNC-11 sont couverts par les livrables décrits dans ce document Parcours et accès. Toute évolution des besoins (ajout, modification, priorisation) doit être répercutée dans le document Parcours et accès pour maintenir la cohérence entre analyse des besoins et livrables. Inversement, toute évolution des parcours ou des règles d’accès dans le document Parcours et accès doit être vérifiée au regard des besoins de la présente analyse.

**Traçabilité** : La matrice besoins / user stories (§ 10.12) et la priorisation backlog (§ 10.13) permettent de tracer chaque besoin jusqu’aux critères d’acceptation des livrables décrits dans le document Parcours et accès.

---

*Fin du document — Utilisateur non connecté — Analyse des besoins (JayRDV).*
