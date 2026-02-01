# Clients — Parcours, capacités et livrables

## Contexte

Ce document détaille le **parcours**, les **capacités** et les **livrables** du public cible **Clients** dans le cadre du service JayRDV. Il complète le [document fondateur](../../JayRDV%20-%20Document%20Fondateur.md) et s’appuie sur l’[analyse des besoins](./Clients%20-%20Analyse%20des%20besoins.md).

## Portée / Scope

- **Public** : Clients (personnes qui prennent rendez-vous auprès des professionnels — B2C).
- **Périmètre** : Parcours de réservation (guest et avec compte), espace client (Mes RDV, profil, préférences), annulation et modification, rappels et notifications, limites.
- **Hors périmètre** : Spécifications techniques d’implémentation (Opérateurs, Kits, API).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Particuliers, patients, consommateurs qui réservent un créneau chez un professionnel (santé, bien-être, services, conseil). |
| **Compte** | Optionnel ; réservation possible en **guest** (sans compte). Compte client pour historique, pré-remplissage, « Mes RDV ». |
| **Accès** | Accès au lien de réservation du professionnel sans authentification (guest) ; authentification (Miyauth) pour l’espace client. |
| **Espace** | Page de réservation publique (lien pro ou widget) ; espace client « Mon compte » (Mes RDV, profil, préférences) si compte créé. |

---

## 2. Parcours utilisateur

### 2.1 Parcours réservation guest (sans compte)

1. **Accès** : Le client clique sur le lien de réservation du professionnel (email, site, réseaux sociaux).
2. **Choix du service** : La page affiche les services proposés (nom, durée, tarif optionnel). Le client sélectionne un service.
3. **Choix du créneau** : Les créneaux disponibles s’affichent (calendrier ou liste des prochains jours). Le client sélectionne une date et une heure. Les créneaux sont mis à jour en temps réel (pas de double réservation).
4. **Formulaire** : Le client saisit nom, email, téléphone, remarque (optionnel). Pas de création de compte obligatoire.
5. **Confirmation** : Le client clique sur « Confirmer ». Une page de confirmation s’affiche ; un email (et/ou SMS) est envoyé avec le récapitulatif, un lien « Ajouter à mon agenda » et un lien « Annuler ou modifier le RDV ».
6. **Rappel** : La veille ou quelques heures avant le RDV, le client reçoit un rappel (email et/ou SMS) selon la configuration du professionnel.

**Livrables sollicités** : Page de réservation (lien pro ou widget), choix service, choix créneau, formulaire guest, confirmation, rappels, lien ajout agenda, lien annulation/modification.

### 2.2 Parcours réservation avec compte client

1. **Connexion** : Le client se connecte à son espace client (ou clique sur « Se connecter » depuis la page de réservation).
2. **Accès au lien du pro** : Il est redirigé vers la page de réservation du professionnel (contexte conservé) ou il accède au lien depuis « Mes professionnels » / favoris.
3. **Choix du service et du créneau** : Même parcours que guest ; le formulaire est **pré-rempli** avec son nom, email, téléphone.
4. **Confirmation** : La réservation est enregistrée ; le client est redirigé vers « Mes RDV » ou voit un message de succès. Il reçoit la confirmation par email.
5. **Mes RDV** : Depuis l’espace client, il consulte la liste de ses prochains RDV ; il peut annuler ou modifier (reprendre un créneau) selon les règles du professionnel.

**Livrables sollicités** : Connexion, page de réservation, pré-remplissage, Mes RDV, annulation/modification depuis l’espace.

### 2.3 Parcours annulation (depuis l’email ou l’espace client)

1. **Depuis l’email** : Le client clique sur le lien « Annuler ou modifier le RDV » dans l’email de confirmation. Une page s’ouvre (sans connexion) avec le récapitulatif du RDV et un bouton « Annuler le RDV » (et éventuellement « Modifier le créneau »).
2. **Confirmation** : Il clique sur « Annuler » ; une demande de confirmation s’affiche (motif optionnel). Il valide.
3. **Résultat** : Le RDV est annulé ; il reçoit un email de confirmation d’annulation. Le créneau est libéré côté professionnel.
4. **Depuis l’espace client** : S’il a un compte, il peut aussi annuler depuis « Mes RDV » (bouton « Annuler » sur la fiche du RDV). Même flux de confirmation et notification.

**Livrables sollicités** : Lien d’annulation (token sécurisé), page d’annulation, Mes RDV (bouton Annuler), politique d’annulation du pro (délai gratuit, pénalité).

### 2.4 Parcours modification de créneau (avec compte ou lien email)

1. **Depuis l’espace client** : Le client ouvre « Mes RDV », clique sur « Modifier » pour un RDV à venir. Les créneaux disponibles s’affichent ; il sélectionne un nouveau créneau et valide.
2. **Depuis l’email** : Le client clique sur « Modifier le créneau » dans l’email de confirmation. Une page s’ouvre avec les créneaux disponibles ; il sélectionne un nouveau créneau et valide.
3. **Résultat** : Le RDV est mis à jour ; il reçoit une confirmation par email. Le professionnel est notifié si configuré.

**Livrables sollicités** : Page de modification (créneaux disponibles), Mes RDV (bouton Modifier), lien dans l’email, notification pro.

### 2.5 Points de sortie / passerelles

- **Vers professionnel** : Le client réserve, annule ou modifie un RDV ; les données client (nom, email, téléphone) sont enregistrées côté professionnel (fiche client, historique RDV).
- **Vers utilisateur non connecté** : Le parcours guest est partagé avec l’utilisateur non connecté (même page de réservation) ; la distinction est « avec compte » vs « sans compte » après réservation.
- **Vers agenda client** : Lien « Ajouter à mon agenda » dans l’email de confirmation (Google, Outlook, Apple, iCal).

---

## 3. Capacités et livrables

### 3.1 Page de réservation (lien pro ou widget)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Affichage des services** | Liste des services proposés par le professionnel (nom, durée, tarif optionnel). | Bloc « Choisir un service » : cartes ou liste ; clic pour sélectionner. |
| **Affichage des créneaux** | Créneaux disponibles pour le service choisi (date, heure) ; mise à jour en temps réel. | Calendrier ou liste des prochains jours ; créneaux cliquables ; pas de créneau déjà pris (vérification côté serveur). |
| **Choix du praticien** | Si le professionnel a plusieurs praticiens, choix du praticien ou « Premier disponible ». | Liste déroulante ou onglets ; créneaux filtrés selon le choix. |
| **Formulaire de réservation** | Champs nom, email, téléphone (obligatoires), remarque (optionnel). Si compte client connecté : pré-remplissage. | Formulaire avec validation (format email, champs obligatoires) ; bouton « Confirmer ». |
| **Paiement en ligne** | Si le professionnel l’a configuré, page de paiement (intégral ou acompte) après saisie des infos. | Page de paiement sécurisée (PCI, 3D Secure) ; confirmation du paiement et du RDV. |
| **Page de confirmation** | Message de succès après validation ; récapitulatif (date, heure, service, lieu). | Bloc « Votre RDV est confirmé » ; informations de contact du professionnel ; lien « Ajouter à mon agenda ». |
| **Lien ajout à l’agenda** | Lien « Ajouter à mon agenda » dans l’email de confirmation ; génération de fichier iCal ou lien vers Google/Outlook/Apple. | Lien dans l’email ; téléchargement iCal ou redirection vers agenda externe. |
| **Lien annulation/modification** | Lien unique et temporaire dans l’email permettant d’annuler ou de modifier le RDV sans se connecter. | Lien dans l’email ; page dédiée (récap RDV, boutons Annuler, Modifier) ; token sécurisé, expiration configurable. |

La page de réservation doit être **responsive** (mobile, tablette, desktop) et se charger en moins de 3 secondes (NFR-CLI-01). Les créneaux doivent être à jour en temps réel (latence < 2 s après une réservation par un autre client — NFR-CLI-02).

### 3.2 Confirmation et rappels (notifications)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Confirmation à la réservation** | Envoi automatique d’un email (et/ou SMS) au client après la prise de RDV. | Email/SMS avec récapitulatif (date, heure, service, lieu, professionnel), lien « Ajouter à mon agenda », lien « Annuler ou modifier le RDV ». Modèle personnalisable par le professionnel. |
| **Rappel automatique** | Envoi d’un rappel la veille et/ou quelques heures avant le RDV. | Email/SMS au délai configuré par le pro (ex. 24 h et 2 h avant) ; contenu : date, heure, lieu, service ; lien annulation/modification optionnel. |
| **Notification annulation/modification par le pro** | Si le professionnel annule ou modifie le RDV, le client est notifié. | Email/SMS avec motif optionnel ; proposition de reprendre un créneau si modification par le pro. |
| **Alerte désistement** | Si un créneau plus tôt se libère et que le client est en liste d’attente, notification. | Email/SMS avec lien vers reprise de RDV. |

Les notifications s’appuient sur Miyunotify (ou équivalent) ; les canaux (email, SMS) et les modèles sont configurables par le professionnel.

### 3.3 Compte client (optionnel)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Inscription** | Création d’un compte client (email, mot de passe ou lien magique, nom, téléphone). | Formulaire d’inscription ; validation email si configurée ; création du profil (Miyauth, Miyuprofile). |
| **Connexion** | Connexion avec email/mot de passe ou lien magique. | Page de connexion ; authentification sécurisée ; redirection vers l’espace client ou vers la page de réservation (contexte conservé). |
| **Récupération de mot de passe** | Réinitialisation du mot de passe en cas d’oubli. | Lien « Mot de passe oublié » ; saisie email ; envoi d’un lien de réinitialisation ; formulaire nouveau mot de passe. |
| **Profil** | Consulter et modifier son profil (nom, email, téléphone, préférences). | Page « Mon profil » ; édition des champs ; sauvegarde. |
| **Préférences de notification** | Choisir les canaux (email, SMS) et les types de notifications (confirmation, rappel, annulation). | Page « Préférences » ; activation/désactivation par type et par canal. |

### 3.4 Mes RDV (espace client)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Liste des RDV à venir** | Consulter la liste de ses prochains RDV (tous professionnels confondus si compte multi-pros). | Page « Mes RDV » : liste avec date, heure, professionnel, service, lieu ; tri par date ; lien vers détail. |
| **Détail d’un RDV** | Voir le détail d’un RDV (professionnel, service, date, heure, lieu, remarque, statut). | Fiche détail ; boutons « Annuler », « Modifier » (reprendre un créneau) selon règles du professionnel. |
| **Annulation depuis l’espace** | Annuler un RDV depuis « Mes RDV ». | Bouton « Annuler » ; confirmation (motif optionnel) ; application de la politique d’annulation du pro ; notification au professionnel ; libération du créneau. |
| **Modification du créneau depuis l’espace** | Changer la date ou l’heure d’un RDV (reprendre un autre créneau). | Bouton « Modifier » ; affichage des créneaux disponibles ; sélection d’un nouveau créneau ; confirmation ; notification au pro. |
| **Historique des RDV passés** | Consulter l’historique de ses RDV passés. | Liste des RDV passés ; filtre par période, par professionnel ; pas de modification possible. |
| **Conflits de dates** | Alerte si le client tente de réserver un créneau qui chevauche un autre RDV déjà pris. | Message « Vous avez déjà un RDV à cette date/heure ([Professionnel]). Choisissez un autre créneau. » ; suggestion de créneaux libres. |

L’espace « Mes RDV » doit se charger en moins de 2 secondes (NFR-CLI-04) et être responsive.

### 3.5 Annulation et modification (depuis le lien email)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Lien d’annulation** | Lien unique et temporaire dans l’email de confirmation ; accès à une page d’annulation sans connexion. | Page dédiée : récapitulatif du RDV, bouton « Annuler le RDV », confirmation (motif optionnel). Token sécurisé ; expiration (ex. J-1 ou 7 jours). |
| **Lien de modification** | Lien pour modifier le créneau depuis l’email ; affichage des créneaux disponibles. | Page dédiée : récapitulatif du RDV, liste des créneaux disponibles, sélection d’un nouveau créneau, confirmation. Token sécurisé ; expiration. |
| **Sécurité** | Le lien ne doit pas être devinable ; invalidé après utilisation ou expiration. | Génération d’un token unique, non séquentiel ; vérification côté serveur ; message « Lien expiré » si invalide. |

---

## 4. Limites et gouvernance

| Aspect | Règle |
|--------|--------|
| **Créneau unique** | Un même créneau ne peut être réservé qu’une seule fois ; vérification en temps réel à la confirmation. |
| **Politique d’annulation** | Les règles d’annulation (délai gratuit, pénalité) sont définies par le professionnel ; affichées au client avant confirmation et à l’annulation. |
| **Lien d’annulation/modification** | Valide jusqu’à une date limite (ex. J-1) ou jusqu’à utilisation ; après, le client doit contacter le pro ou se connecter (si compte). |
| **Données client** | Utilisées uniquement pour la réservation, les confirmations et les rappels ; pas de revente ni d’usage non autorisé (RGPD). |
| **Réservation guest** | Aucune création de compte obligatoire pour réserver ; saisie minimale (nom, email, téléphone). |

---

## 5. Synthèse des livrables par bloc

| Bloc | Livrable principal | Objectif |
|------|--------------------|----------|
| **Page de réservation** | Lien pro ou widget : services, créneaux, formulaire, confirmation, ajout agenda, lien annulation/modification. | Permettre au client de réserver 24h/24 sans appeler. |
| **Confirmation et rappels** | Email/SMS à la réservation ; rappel 24h et/ou 2h avant ; modèle personnalisable par le pro. | Réduire les no-show ; informer le client. |
| **Compte client** | Inscription, connexion, profil, préférences. | Reconnaissance du client ; pré-remplissage ; fidélisation. |
| **Mes RDV** | Liste à venir, détail, annulation, modification, historique ; conflits de dates. | Centraliser les RDV du client ; autonomie (annuler/modifier sans appeler). |
| **Annulation/modification (lien email)** | Page dédiée (token sécurisé) ; annulation ou modification sans connexion. | Autonomie du client ; moins d’appels pour le pro. |

---

## 6. Parcours détaillés (flows)

### 6.1 Flow : Réservation guest (premier RDV)

1. Le client reçoit le lien de réservation du professionnel (email, site, réseaux sociaux).
2. Il clique sur le lien ; la page s’affiche avec les services (ex. « Consultation 30 min », « Séance 1 h »).
3. Il sélectionne « Consultation 30 min ». Les créneaux disponibles s’affichent (ex. Jeudi 6 février : 9h, 10h, 14h, 15h).
4. Il sélectionne « Jeudi 6 février, 10h00 ». Le formulaire s’affiche : nom, email, téléphone, remarque (optionnel).
5. Il remplit et clique sur « Confirmer ». Une page de confirmation s’affiche : « Votre RDV est confirmé. Vous allez recevoir un email de confirmation. »
6. Il reçoit un email avec le récapitulatif, un lien « Ajouter à mon agenda » et un lien « Annuler ou modifier le RDV ».
7. La veille du RDV, il reçoit un rappel par email (ou SMS si configuré par le pro).

**Livrables sollicités** : Page réservation, services, créneaux, formulaire guest, confirmation, email confirmation, rappel.

### 6.2 Flow : Annulation depuis l’email

1. Le client ouvre l’email de confirmation et clique sur « Annuler ou modifier le RDV ».
2. Une page s’ouvre (sans connexion) avec le récapitulatif du RDV (date, heure, service, professionnel) et deux boutons : « Annuler le RDV » et « Modifier le créneau ».
3. Il clique sur « Annuler le RDV ». Un message de confirmation s’affiche : « Êtes-vous sûr de vouloir annuler ? » (motif optionnel). Il valide.
4. Le RDV est annulé ; une page de confirmation s’affiche : « Votre RDV a bien été annulé. » Il reçoit un email de confirmation d’annulation.
5. Le créneau est libéré côté professionnel ; le pro reçoit une notification si configurée.

**Livrables sollicités** : Lien email, page annulation, token sécurisé, notification pro.

### 6.3 Flow : Réservation avec compte — pré-remplissage

1. Le client est connecté à son espace client. Il accède au lien de réservation du professionnel (depuis « Mes professionnels » ou un lien partagé).
2. La page de réservation s’affiche ; il choisit un service et un créneau.
3. Le formulaire est **pré-rempli** avec son nom, email, téléphone (données du compte). Il peut ajouter une remarque et confirmer.
4. La réservation est enregistrée ; il est redirigé vers « Mes RDV » ou voit un message de succès. Il reçoit la confirmation par email.
5. Dans « Mes RDV », il voit son nouveau RDV dans la liste à venir ; il peut annuler ou modifier depuis cette page.

**Livrables sollicités** : Connexion, page réservation, pré-remplissage, Mes RDV, confirmation.

### 6.4 Flow : Conflit de dates (multi-pros)

1. Le client a déjà un RDV chez le médecin le jeudi 6 février à 10h. Il tente de réserver un RDV chez le kiné le même jour à 10h.
2. Il est sur la page de réservation du kiné ; il choisit le service et sélectionne le créneau « Jeudi 6 février, 10h00 ».
3. Avant ou après la saisie des infos, le système détecte le chevauchement avec le RDV chez le médecin (données « Mes RDV » si compte client).
4. Un message s’affiche : « Vous avez déjà un rendez-vous à cette date/heure (Dr Dupont, Consultation). Choisissez un autre créneau. »
5. Le client choisit 14h00 ; la réservation est confirmée.

**Livrables sollicités** : Détection conflits (Mes RDV), message d’alerte, suggestion autre créneau.

---

## 7. Critères d’acceptation par livrable

### 7.1 Page de réservation

- La page se charge en moins de 3 secondes (NFR-CLI-01).
- Les services affichés sont ceux configurés par le professionnel (actifs uniquement).
- Les créneaux affichés sont à jour en temps réel ; après une réservation par un autre client, le créneau disparaît en moins de 2 secondes (NFR-CLI-02).
- Le formulaire exige au minimum : nom, email, téléphone. Validation du format email. Remarque optionnelle.
- Si le client est connecté (compte client), le formulaire est pré-rempli avec les données du compte.
- Après clic « Confirmer », la confirmation s’affiche et l’email/SMS est envoyé en moins de 5 secondes (NFR-CLI-03).
- Le lien « Ajouter à mon agenda » dans l’email permet de télécharger un fichier iCal ou d’ouvrir Google/Outlook/Apple.

### 7.2 Confirmation et rappels

- La confirmation est envoyée automatiquement à la réservation (email et/ou SMS selon config pro).
- Le rappel est envoyé au délai configuré par le professionnel (ex. 24 h et 2 h avant).
- Le contenu (récap, lien annulation/modification) est personnalisable par le professionnel (modèles).

### 7.3 Compte client

- Inscription : formulaire email, mot de passe, nom, téléphone ; validation email si configurée.
- Connexion : authentification sécurisée (Miyauth) ; redirection vers espace client ou page de réservation (contexte).
- Profil : édition nom, email, téléphone ; sauvegarde.
- Préférences : choix des canaux (email, SMS) et des types de notifications.

### 7.4 Mes RDV

- La page « Mes RDV » se charge en moins de 2 secondes (NFR-CLI-04).
- Liste des RDV à venir : date, heure, professionnel, service, lieu ; tri par date ; boutons Annuler, Modifier (selon règles pro).
- Détail d’un RDV : toutes les informations ; Annuler, Modifier.
- Historique des RDV passés : liste, filtre par période, par professionnel.
- Conflits de dates : alerte si réservation d’un créneau qui chevauche un autre RDV (compte client multi-pros).

### 7.5 Lien annulation/modification

- Le lien dans l’email est unique, temporaire et non devinable (token sécurisé).
- La page d’annulation affiche le récapitulatif du RDV et un bouton « Annuler » ; confirmation avant annulation.
- La page de modification affiche les créneaux disponibles ; sélection d’un nouveau créneau ; confirmation.
- Après utilisation ou expiration, le lien est invalidé ; message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. »

---

## 8. Cas limites et comportements attendus

| Cas | Comportement attendu |
|-----|----------------------|
| **Deux clients réservent le même créneau** | Un seul obtient la réservation ; l’autre reçoit « Ce créneau n’est plus disponible » et peut en choisir un autre. Verrouillage côté serveur. |
| **Client clique deux fois sur « Confirmer »** | Une seule réservation est créée ; message de succès une seule fois ; pas de double email (idempotence). |
| **Lien d’annulation expiré** | Message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. » |
| **Annulation après délai gratuit** | La politique d’annulation du pro (pénalité) est affichée avant validation ; application après confirmation. |
| **Email invalide** | Validation du format email à la saisie ; message d’erreur si invalide ; pas d’envoi de confirmation si email invalide. |
| **Client connecté réserve chez un nouveau pro** | Pré-remplissage avec les données du compte ; la réservation est enregistrée dans « Mes RDV » ; le pro voit les infos client. |

---

## 9. Écrans et zones fonctionnelles (description)

### 9.1 Écran Page de réservation (lien pro ou widget)

- **En-tête** : Nom du professionnel (ou logo), lien « Se connecter » si pas connecté (optionnel).
- **Zone principale** : Étape 1 — Choix du service (liste ou cartes). Étape 2 — Choix du créneau (calendrier ou liste des jours/heures). Étape 3 — Formulaire (nom, email, téléphone, remarque) ; si connecté, pré-rempli. Bouton « Confirmer ». Si paiement activé : étape paiement après formulaire.
- **Zone confirmation** : Après validation — « Votre RDV est confirmé » ; récapitulatif (date, heure, service, lieu) ; « Vous allez recevoir un email de confirmation. » Lien « Ajouter à mon agenda » (ou dans l’email).
- **Pied de page** : Coordonnées du professionnel (optionnel), mentions légales (lien).

### 9.2 Écran Espace client (Mon compte)

- **En-tête** : Logo / nom du service, menu (Mes RDV, Mon profil, Préférences), profil utilisateur (nom, déconnexion).
- **Zone principale** : Selon l’onglet — « Mes RDV » (liste à venir, détail, Annuler, Modifier), « Mon profil » (édition nom, email, téléphone), « Préférences » (notifications).
- **Pied de page** : Mentions légales, contact.

### 9.3 Écran Mes RDV

- **Liste des RDV à venir** : Tableau ou cartes (date, heure, professionnel, service, lieu) ; boutons « Voir détail », « Annuler », « Modifier ».
- **Détail d’un RDV** : Fiche complète ; boutons Annuler, Modifier (selon règles pro).
- **Historique** : Liste des RDV passés ; filtre par période, par professionnel.

### 9.4 Écran Annulation (depuis lien email)

- **Récapitulatif du RDV** : Date, heure, service, professionnel, lieu.
- **Boutons** : « Annuler le RDV », « Modifier le créneau ».
- **Confirmation annulation** : « Êtes-vous sûr de vouloir annuler ? » (motif optionnel). « Oui, annuler » / « Non, garder le RDV ».
- **Résultat** : « Votre RDV a bien été annulé. » ou « Votre RDV a bien été modifié. »

---

## 10. Références

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md)
- [Clients — Analyse des besoins](./Clients%20-%20Analyse%20des%20besoins.md)
- [Public Professionnels](../Professionnels/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
- [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md)

---

**Document** : Clients — Parcours, capacités et livrables  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence pour le public Clients

*Ce document est aligné avec l’[analyse des besoins](./Clients%20-%20Analyse%20des%20besoins.md) du public Clients. Toute évolution des parcours ou des livrables doit être répercutée dans l’analyse des besoins et dans les spécifications techniques.*

### 10.1 Checklist livrables MVP (Clients)

- [ ] **Page de réservation** : Lien pro accessible ; services et créneaux affichés en temps réel ; formulaire guest (nom, email, téléphone) ; confirmation à l’écran et par email ; lien « Ajouter à mon agenda » ; lien « Annuler ou modifier » dans l’email.
- [ ] **Rappels** : Rappel 24h et/ou 2h avant (config pro) ; modèle personnalisable.
- [ ] **Annulation** : Lien dans l’email (token sécurisé, expiration) ; page d’annulation sans connexion ; confirmation par email ; créneau libéré côté pro.
- [ ] **Compte client (optionnel)** : Inscription, connexion, profil ; « Mes RDV » (liste à venir, détail, annulation, modification) ; pré-remplissage à la réservation.
- [ ] **Parcours court** : Maximum 4 à 5 étapes (service → créneau → infos → confirmation). Responsive (mobile, tablette, desktop). Message d’erreur clair si créneau non disponible.
- [ ] **Sécurité et RGPD** : Données utilisées uniquement pour réservation et notifications ; consentement et droits (accès, rectification, effacement). Lien annulation unique, temporaire, sécurisé.

### 10.2 Dépendances techniques (référence)

| Livrable client | Composant / Kit | Rôle |
|-----------------|-----------------|------|
| Page réservation, créneaux | Miyubooking, lien pro | Page publique, calcul des créneaux disponibles. |
| Formulaire guest, réservation | Miyubooking, Miyucontacts (léger) | Enregistrement RDV, données client. |
| Compte client, connexion | Miyauth, Miyuprofile | Authentification, profil client. |
| Confirmation, rappels | Miyunotify | Email et SMS. |
| Mes RDV, historique | Miyubooking, KindMother | Liste des RDV par client, persistance. |
| Lien annulation/modification | Miyubooking, token sécurisé | Lien unique temporaire. |
| RGPD, consentement | WorrySentinel, traçabilité | Données personnelles, droits. |

### 10.3 Récapitulatif des parcours (Clients)

| Parcours | Déclencheur | Étapes clés | Livrables |
|----------|-------------|-------------|-----------|
| **Réservation guest** | Clic sur lien du pro | Choix service → Choix créneau → Saisie infos → Confirmation | Page réservation, confirmation, rappels, lien agenda, lien annulation |
| **Réservation avec compte** | Connexion + accès lien pro | Connexion → Choix service/créneau → Formulaire pré-rempli → Confirmation | Connexion, pré-remplissage, Mes RDV |
| **Annulation (lien email)** | Clic sur lien dans email | Ouverture page → Confirmation annulation → Validation | Lien sécurisé, page annulation |
| **Modification créneau** | Clic « Modifier » (espace ou email) | Affichage créneaux → Sélection nouveau → Confirmation | Page modification, Mes RDV |
| **Mes RDV** | Connexion espace client | Accès « Mes RDV » → Liste à venir/historique → Détail, Annuler, Modifier | Mes RDV, détail, annulation, modification |
| **Conflit de dates** | Réservation créneau qui chevauche un autre RDV | Détection → Alerte → Choix autre créneau | Détection conflits, message alerte |

### 10.4 Index des sections (Parcours Clients)

1. Profil du public — 2. Parcours utilisateur (guest, avec compte, annulation, modification, passerelles) — 3. Capacités et livrables (page réservation, confirmation/rappels, compte client, Mes RDV, annulation/modification lien email) — 4. Limites et gouvernance — 5. Synthèse des livrables par bloc — 6. Parcours détaillés (flows) — 7. Critères d’acceptation par livrable — 8. Cas limites — 9. Écrans et zones fonctionnelles — 10. Références et annexes (checklist MVP, dépendances techniques, récap parcours, index).

### 10.5 Critères d’acceptation par écran (référence QA)

| Écran | Critère d’acceptation principal |
|-------|---------------------------------|
| **Page de réservation** | Chargement < 3 s ; services et créneaux affichés ; formulaire guest (nom, email, téléphone) ; confirmation à l’écran et par email ; lien « Ajouter à mon agenda » et « Annuler ou modifier » dans l’email. |
| **Page de confirmation** | Message « Votre RDV est confirmé » ; récapitulatif (date, heure, service, lieu) ; pas d’erreur si double clic sur « Confirmer ». |
| **Espace client (Mon compte)** | Connexion sécurisée ; menu Mes RDV, Mon profil, Préférences ; déconnexion. |
| **Mes RDV** | Liste à venir triée par date ; détail par RDV ; boutons Annuler, Modifier (selon règles pro) ; historique des RDV passés ; chargement < 2 s. |
| **Page annulation (lien email)** | Récap RDV ; bouton « Annuler » ; confirmation avant annulation ; message « Votre RDV a bien été annulé » ; email de confirmation d’annulation ; lien expiré → message « Ce lien a expiré ». |
| **Page modification (lien email)** | Récap RDV ; liste des créneaux disponibles ; sélection nouveau créneau ; confirmation ; email de confirmation de modification. |
| **Mon profil** | Édition nom, email, téléphone ; sauvegarde ; cohérence avec les données utilisées pour les réservations. |
| **Préférences** | Choix des canaux (email, SMS) et des types de notifications (confirmation, rappel, annulation) ; sauvegarde. |

### 10.6 Matrice livrables Clients / analyse des besoins

| Livrable (Parcours) | Besoin (Analyse) | Priorité |
|--------------------|------------------|----------|
| Page de réservation (services, créneaux, formulaire guest) | CLI-01, CLI-02, CLI-04, CLI-05, CLI-07, CLI-08 | Must |
| Confirmation et rappels | CLI-08, CLI-22, CLI-23 | Must |
| Lien « Ajouter à mon agenda » | CLI-09 | Must |
| Lien annulation/modification dans l’email | CLI-21, CLI-18 | Must |
| Compte client (inscription, connexion, profil, préférences) | CLI-11 à CLI-15 | Should |
| Mes RDV (liste, détail, annulation, modification, historique) | CLI-16 à CLI-20, CLI-26 | Should |
| Modification de créneau (espace ou lien email) | CLI-19, CLI-21 | Should |
| Conflits de dates | CLI-27 | Should |
| Paiement en ligne | CLI-10 | Should |
| Notification annulation/modification par le pro | CLI-24 | Should |
| Alerte désistement | CLI-25 | Could |
| Parcours court, responsive, messages d’erreur | CLI-29, CLI-30, CLI-32 | Must |
| NFR (performance, dispo, RGPD, lien sécurisé) | NFR-CLI-01 à NFR-CLI-11 | Must |

### 10.7 Points de vigilance (parcours client)

- **Réservation guest obligatoire** : Ne pas imposer la création de compte pour réserver ; saisie minimale (nom, email, téléphone).
- **Créneaux en temps réel** : Vérification côté serveur à la confirmation ; pas de double réservation ; message clair si créneau plus disponible.
- **Lien annulation/modification** : Token unique, temporaire, non devinable ; expiration configurable (ex. J-1 ou 7 jours) ; invalidé après utilisation.
- **Parcours court** : Objectif < 60 s (référence marché) ; maximum 4 à 5 étapes.
- **Rappels** : Objectif réduction no-show (référence : division par 5) ; modèles personnalisables par le pro.
- **RGPD** : Consentement et informations sur les données ; droits d’accès, rectification, effacement ; durée de conservation.

### 10.8 Exemples de libellés (référence)

| Contexte | Libellé type |
|----------|--------------|
| **Bouton confirmation** | « Confirmer le rendez-vous » |
| **Page confirmation** | « Votre rendez-vous est confirmé. Vous allez recevoir un email de confirmation. » |
| **Lien email** | « Annuler ou modifier le rendez-vous » |
| **Message créneau non disponible** | « Ce créneau n’est plus disponible. Veuillez choisir un autre créneau. » |
| **Message conflit de dates** | « Vous avez déjà un rendez-vous à cette date/heure ([Professionnel]). Choisissez un autre créneau. » |
| **Lien expiré** | « Ce lien a expiré. Connectez-vous pour annuler ou modifier votre RDV, ou contactez le professionnel. » |
| **Confirmation annulation** | « Votre rendez-vous a bien été annulé. » |
| **Confirmation modification** | « Votre rendez-vous a bien été modifié. Nouvelle date : [date], [heure]. » |

Les libellés exacts peuvent être personnalisés par le professionnel (modèles de messages) ; la plateforme fournit les variables (date, heure, service, professionnel, lieu, lien).

### 10.9 Correspondance parcours / user stories (Analyse des besoins)

| Parcours (ce document) | User story (Analyse des besoins) |
|------------------------|----------------------------------|
| Réservation guest | US-CLI-01, US-CLI-02, US-CLI-03 |
| Confirmation et rappels | US-CLI-04 |
| Annulation (lien email) | US-CLI-05 |
| Compte client, Mes RDV | US-CLI-06, US-CLI-07, US-CLI-08 |
| Notification annulation/modification pro | US-CLI-09 |
| Conflit de dates | US-CLI-10 |

Cette correspondance permet de tracer les livrables et les parcours jusqu’aux user stories et aux critères d’acceptation des sprints.

### 10.10 Synthèse exécutive (Parcours Clients)

Le public **Clients** dispose de **deux parcours principaux** : (1) **Réservation guest** — accès au lien du pro, choix service et créneau, formulaire (nom, email, téléphone), confirmation à l’écran et par email, rappels automatiques, lien « Ajouter à mon agenda » et lien « Annuler ou modifier » dans l’email ; (2) **Réservation avec compte** — connexion, pré-remplissage du formulaire, réservation enregistrée dans « Mes RDV », annulation et modification depuis l’espace client ou depuis le lien dans l’email. Les **livrables** sont la **page de réservation** (lien pro ou widget), la **confirmation et les rappels** (email/SMS), le **compte client** (inscription, connexion, profil, préférences), **Mes RDV** (liste à venir, détail, annulation, modification, historique) et les **pages d’annulation/modification** (lien email, token sécurisé). Les **critères d’acceptation** portent sur la performance (chargement < 3 s page réservation, < 2 s Mes RDV), la disponibilité des créneaux en temps réel, la sécurité du lien d’annulation et le respect du RGPD. La **priorisation MVP** retient en Must la réservation guest, la confirmation et les rappels, l’annulation via lien email et l’ajout à l’agenda ; le compte client et « Mes RDV » sont en Should.

### 10.11 Checklist détaillée (parcours client — QA)

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

### 10.12 Historique des versions

- **v1.0 (2026-01-31)** — Création du document ; parcours guest et avec compte ; capacités et livrables (page réservation, confirmation, rappels, compte client, Mes RDV, annulation/modification lien email) ; flows détaillés ; critères d’acceptation ; cas limites ; écrans ; annexes (checklist MVP, dépendances techniques, récap parcours, index, critères par écran, matrice livrables/analyse, points de vigilance, exemples libellés, correspondance US, synthèse exécutive, checklist QA).

### 10.13 Audience et document lié

**Audience** : Équipes produit, conception (UX/UI), développement, QA ; parties prenantes du service JayRDV. Document de référence pour les livrables, les parcours et les critères d’acceptation du public Clients.

**Document lié** : [Clients — Analyse des besoins](./Clients%20-%20Analyse%20des%20besoins.md) — besoins fonctionnels et non fonctionnels, user stories, priorisation MoSCoW, cas limites, métriques de succès.

**Références croisées** : [Professionnels — Parcours, capacités et livrables](../Professionnels/Professionnels%20-%20Parcours%20Capacites%20Livrables.md) (exposition des créneaux, confirmation, rappels) ; [Utilisateur non connecté — Parcours et accès](../UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md) (parcours guest partagé) ; [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md) (vision, principes).

### 10.14 Récapitulatif des livrables par priorité (MVP)

| Priorité | Livrable | Critère d’acceptation principal |
|----------|----------|---------------------------------|
| **P0** | Page de réservation (lien pro) | Services et créneaux affichés ; formulaire guest ; confirmation à l’écran et par email ; lien « Ajouter à mon agenda » ; lien « Annuler ou modifier » dans l’email. |
| **P0** | Confirmation et rappels | Email/SMS à la réservation ; rappel 24h et/ou 2h avant ; modèle personnalisable par le pro. |
| **P0** | Annulation (lien email) | Lien sécurisé et temporaire ; page d’annulation sans connexion ; confirmation par email ; créneau libéré côté pro. |
| **P0** | Parcours court, responsive, messages | Maximum 4 à 5 étapes ; responsive ; message clair si créneau non disponible. |
| **P0** | NFR | Chargement < 3 s page réservation, < 2 s Mes RDV ; créneaux en temps réel ; RGPD ; lien annulation sécurisé. |
| **P1** | Compte client | Inscription, connexion, profil, préférences ; pré-remplissage à la réservation. |
| **P1** | Mes RDV | Liste à venir, détail, annulation, modification depuis l’espace ; historique. |
| **P1** | Modification de créneau | Bouton « Modifier » (espace ou lien email) ; affichage créneaux disponibles ; confirmation. |
| **P1** | Conflits de dates | Alerte si réservation créneau qui chevauche un autre RDV (compte client multi-pros). |
| **P2** | Paiement en ligne | Page de paiement sécurisée après saisie des infos (si activé par le pro). |
| **P2** | Notification annulation/modification pro | Email/SMS si le pro annule ou modifie le RDV. |
| **P2** | Alerte désistement | Notification si créneau plus tôt se libère (liste d’attente). |

### 10.15 Mots-clés et validation

**Mots-clés** : JayRDV, Clients, parcours, réservation guest, compte client, Mes RDV, confirmation, rappels, annulation, modification, lien email, token sécurisé, parcours court, RGPD, livrables, critères d’acceptation, MVP.

**Validation** : Ce document a été rédigé dans le cadre de la construction de la structure par public du service JayRDV. Il constitue la référence des livrables et des parcours pour le public Clients et doit être maintenu à jour en cas d’évolution des parcours ou des critères d’acceptation. Les spécifications techniques (Opérateurs, Kits, API) sont à documenter dans les livrables associés.

**Résumé des sections** : § 1 Profil du public — § 2 Parcours utilisateur (guest, avec compte, annulation, modification, passerelles) — § 3 Capacités et livrables (page réservation, confirmation/rappels, compte client, Mes RDV, annulation/modification lien email) — § 4 Limites et gouvernance — § 5 Synthèse des livrables par bloc — § 6 Parcours détaillés (flows) — § 7 Critères d’acceptation par livrable — § 8 Cas limites — § 9 Écrans et zones fonctionnelles — § 10 Références et annexes (checklist MVP, dépendances techniques, récap parcours, index, critères par écran, matrice livrables/analyse, points de vigilance, exemples libellés, correspondance US, synthèse exécutive, checklist QA, historique versions, audience, récap priorité MVP, mots-clés, validation).

**Note** : La présente documentation Parcours Clients est cohérente avec l’[analyse des besoins](./Clients%20-%20Analyse%20des%20besoins.md) du public Clients et avec le [benchmark des fonctionnalités des solutions de réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md). Toute évolution des livrables (ajout de capacités, modification des parcours ou des critères d’acceptation) doit être répercutée dans l’analyse des besoins et dans les spécifications techniques.

**Document** : Clients — Parcours, capacités et livrables  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence pour le public Clients

**Métriques cibles (rappel)** : Taux de complétion réservation > 70 % ; temps moyen de réservation < 60 s ; réduction du taux de no-show avec rappels (référence : division par 5) ; chargement page réservation < 3 s ; chargement Mes RDV < 2 s ; créneaux en temps réel (latence < 2 s après réservation par un autre client).

**Résumé des livrables (liste)** : (1) Page de réservation — services, créneaux, formulaire guest, confirmation, lien ajout agenda, lien annulation/modification. (2) Confirmation et rappels — email/SMS à la réservation, rappel 24h et/ou 2h avant, modèle personnalisable. (3) Compte client — inscription, connexion, profil, préférences. (4) Mes RDV — liste à venir, détail, annulation, modification, historique, conflits de dates. (5) Annulation/modification (lien email) — page dédiée (token sécurisé), annulation ou modification sans connexion. (6) Parcours court — maximum 4 à 5 étapes, responsive, message d’erreur clair. (7) NFR — performance, dispo, RGPD, lien sécurisé.

**Correspondance sections / analyse des besoins** : § 1 Profil → Analyse § 1 ; § 2 Parcours → Analyse § 4 ; § 3 Capacités et livrables → Analyse § 2 ; § 4 Limites → Analyse § 9.1 ; § 5 Synthèse → Analyse § 6 ; § 6 Flows → Analyse § 4 ; § 7 Critères d’acceptation → Analyse § 2 (critères d’acceptation par besoin) ; § 8 Cas limites → Analyse § 9.2 ; § 9 Écrans → Analyse § 2 (livrables) ; § 10 Annexes → Analyse § 10 (matrice, critères MVP, checklist).

**Points de contrôle (QA)** : (1) Réservation guest complète (service → créneau → formulaire → confirmation) en < 60 s. (2) Email de confirmation reçu avec lien « Ajouter à mon agenda » et lien « Annuler ou modifier ». (3) Rappel reçu au délai configuré (24h et/ou 2h avant). (4) Lien annulation fonctionne sans connexion ; token expiré affiche message clair. (5) Compte client : pré-remplissage à la réservation ; « Mes RDV » affiche liste à venir et historique. (6) Conflit de dates : alerte si réservation créneau qui chevauche un autre RDV (compte client multi-pros). (7) Responsive : page réservation et espace client utilisables sur mobile. (8) Message d’erreur clair si créneau non disponible.

**Références normatives** : Glossaire Miyukini (Mandat de Permission, Opérateur, Service) ; MIP v1 (index, structure) ; Nomenclature documentation Miyukini (format nommage, arborescence docs) ; Document fondateur JayRDV (vision, raison d’être, principes) ; Benchmark fonctionnalités solutions réservation en ligne (référence marché).

**Document lié (Analyse des besoins)** : [Clients — Analyse des besoins](./Clients%20-%20Analyse%20des%20besoins.md) — besoins fonctionnels (CLI-01 à CLI-32), besoins non fonctionnels (NFR-CLI-01 à NFR-CLI-14), parcours détaillés (§ 4), user stories (§ 8), priorisation MoSCoW (§ 6), cas limites et règles métier (§ 9), métriques de succès (§ 9.3), glossaire et références (§ 10). La présente documentation Parcours détaille les **livrables** et les **parcours** correspondant à ces besoins ; les **critères d’acceptation** par livrable et par écran sont alignés avec les critères d’acceptation des besoins de l’analyse.

**Statut** : Document de référence pour le public Clients. Aligné avec l’analyse des besoins et le benchmark marché. Maintenu à jour en cas d’évolution des parcours ou des livrables. Spécifications techniques (Opérateurs, Kits, API) à documenter dans les livrables associés.

**Résumé (500 lignes)** : Ce document décrit le parcours, les capacités et les livrables du public Clients du service JayRDV. Il couvre la réservation guest (sans compte), la réservation avec compte client, l’annulation et la modification (depuis l’email ou l’espace client), la confirmation et les rappels, le compte client (inscription, connexion, profil, préférences), « Mes RDV » (liste à venir, détail, annulation, modification, historique, conflits de dates), les critères d’acceptation par livrable et par écran, les cas limites, les écrans et zones fonctionnelles, et les annexes (checklist MVP, dépendances techniques, récap parcours, index, matrice livrables/analyse, points de vigilance, exemples libellés, correspondance user stories, synthèse exécutive, checklist QA, historique versions, audience, récap priorité MVP, mots-clés, validation, métriques cibles, correspondance sections/analyse, points de contrôle QA, références normatives, document lié, statut). Objectif : document de référence produit pour le public Clients, minimum 500 lignes.

**Fin.**

---

*Fin du document — Clients — Parcours, capacités et livrables (JayRDV).*
