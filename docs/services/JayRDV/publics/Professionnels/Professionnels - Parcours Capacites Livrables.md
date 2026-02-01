# Professionnels — Parcours, capacités et livrables

## Contexte

Ce document détaille le **parcours**, les **capacités** et les **livrables** du public cible **Professionnels** dans le cadre du service JayRDV. Il complète le [document fondateur](../../JayRDV%20-%20Document%20Fondateur.md) et s’appuie sur l’[analyse des besoins](./Professionnels%20-%20Analyse%20des%20besoins.md).

## Portée / Scope

- **Public** : Professionnels (praticiens, entreprises, équipes) qui proposent des créneaux de réservation.
- **Périmètre** : Onboarding, espace dédié, rôles, capacités métier, livrables (dashboard, calendrier, notifications, stats, intégrations), limites.
- **Hors périmètre** : Spécifications techniques d’implémentation (Opérateurs, Kits, API).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Médecins, thérapeutes, coiffeurs, consultants, formateurs, prestataires de services ; cabinets, équipes, réseaux multi-établissements. |
| **Compte** | Compte professionnel (mono ou multi-établissement selon offre). |
| **Accès** | Authentification (Miyauth), permissions (Master Butler), Mandat de Permission pour gérer plannings, services et RDV. |
| **Espace** | Tableau de bord pro ; calendrier centralisé ; gestion des services, des plannings, des ressources et des RDV. |

---

## 2. Parcours utilisateur

### 2.1 Onboarding

1. **Création de compte** : inscription en tant que professionnel (Miyauth, Miyuprofile) : email, mot de passe ou lien magique, nom, type d’activité, établissement(s).
2. **Validation** : selon politique plateforme (validation manuelle ou automatique) ; notification (validé / en attente / refusé).
3. **Attribution des permissions** : rôle professionnel (Admin, Gestionnaire, Praticien), émission du Mandat de Permission (StrongFather, Master Butler).
4. **Paramétrage initial** : définition d’au moins un service (nom, durée), d’un planning récurrent (horaires par jour de la semaine), et génération du lien de réservation.

Le compte est **opérationnel** dès que le professionnel a défini au moins un service et un planning ; il peut ensuite inviter des membres (Gestionnaire, Praticien) et ajouter des établissements si l’offre le permet.

### 2.2 Parcours type (cycle de vie)

| Étape | Action | Résultat |
|-------|--------|----------|
| **Connexion** | Connexion avec identifiants professionnel. | Accès au tableau de bord pro. |
| **Vue d’ensemble** | Consultation du dashboard : RDV du jour, de la semaine, indicateurs (taux de remplissage, no-show). | Vue synthétique de l’activité. |
| **Calendrier** | Consultation du calendrier (jour, semaine, mois) ; filtres par praticien, ressource, établissement. | Vue détaillée des RDV et des blocages. |
| **Gestion des RDV** | Création manuelle d’un RDV, modification (créneau, service, client), annulation. | RDV à jour ; notifications client si configurées. |
| **Paramétrage** | Modification des services, des plannings, des exceptions (congés, absences), des notifications. | Créneaux et messages à jour. |
| **Exposition** | Partage du lien de réservation, intégration du widget sur le site, configuration API/webhooks. | Les clients peuvent réserver en ligne 24h/24. |
| **Statistiques** | Consultation des analyses (volume de RDV, taux de no-show, créneaux les plus demandés). | Pilotage de l’activité. |
| **Équipe** | Invitation de membres, attribution des rôles, délégation par établissement. | Délégation opérationnelle. |

### 2.3 Points de sortie / passerelles

- **Vers clients** : le professionnel expose un lien et un widget ; les clients (avec ou sans compte) réservent depuis la page publique ; le professionnel reçoit les RDV et les notifications.
- **Vers utilisateur non connecté** : les utilisateurs non connectés accèdent à la même page de réservation (parcours guest) ; ils reçoivent une confirmation par email/SMS sans créer de compte.
- **Vers outils externes** : synchronisation avec Google Calendar, Outlook, Apple iCal ; API et webhooks pour CRM, logiciel métier, Zapier/Make.

---

## 3. Rôles côté professionnel

| Rôle | Périmètre | Capacités principales |
|------|-----------|------------------------|
| **Admin** | Gestion complète du compte. | Paramétrage (services, plannings, notifications, paiement, équipe), calendrier (tous les RDV), stats, intégrations (lien, widget, API), gestion des membres. |
| **Gestionnaire** | Gestion des RDV et des plannings sans modifier les paramètres globaux. | Calendrier (selon établissements assignés), création/modification/annulation RDV, vue clients ; pas d’accès aux paramètres de compte (services, tarifs, notifications). |
| **Praticien** | Accès à son propre planning. | Vue limitée à son planning et ses RDV ; modification de ses indisponibilités (congés, pauses) ; pas d’accès aux autres praticiens ni aux paramètres. |

*Note :* un **Client** correspond à un compte ou un parcours distinct (voir [Public Clients](../Clients/_index.md)) ; un utilisateur non connecté réserve sans compte (voir [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)).

### 3.1 Délégation par établissement

- Un **Gestionnaire** peut être assigné à un ou plusieurs établissements uniquement ; il ne voit que les RDV et plannings de ces établissements.
- Un **Admin** voit tous les établissements et peut gérer l’équipe (invitation, attribution des rôles et des établissements).
- Un **Praticien** est associé à un ou plusieurs établissements ; son planning est visible uniquement dans le périmètre de ces établissements.

---

## 4. Capacités et livrables

### 4.1 Tableau de bord pro (dashboard)

| Bloc | Contenu | Rôle |
|------|---------|------|
| **RDV du jour** | Liste des RDV du jour (heure, client, service, praticien) ; liens vers détail et modification. | Vue rapide pour démarrer la journée. |
| **RDV de la semaine** | Aperçu des RDV à venir (7 jours) ; filtres par praticien, établissement. | Anticipation et planification. |
| **Indicateurs clés** | Nombre de RDV (jour, semaine, mois), taux de remplissage, taux de no-show (optionnel). | Pilotage de l’activité. |
| **Actions rapides** | Boutons : Créer un RDV, Voir le calendrier, Paramètres, Lien de réservation. | Accès direct aux capacités principales. |
| **Notifications** | Dernières réservations, annulations, rappels envoyés (optionnel). | Suivi en temps réel. |

Le tableau de bord est la **page d’accueil** après connexion ; il doit se charger en moins de 3 secondes (NFR-PRO-01) et être utilisable sur desktop et tablette (NFR-PRO-15).

### 4.2 Calendrier centralisé

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Vue jour** | Affichage des créneaux et RDV du jour, par praticien ou par ressource. | Grille horaire avec RDV (couleurs par type ou statut) ; créneaux libres visibles ou masqués selon paramètre. |
| **Vue semaine** | Affichage de la semaine (lun–dim ou mar–lun) avec RDV et blocages. | Grille 7 jours ; colonnes par praticien ou par ressource ; drag & drop pour déplacer un RDV (optionnel). |
| **Vue mois** | Affichage du mois avec indicateur de charge (nombre de RDV par jour). | Calendrier mensuel ; clic sur un jour pour ouvrir la vue jour. |
| **Filtres** | Filtre par praticien, ressource, établissement, type de service. | Liste déroulante ou onglets ; mise à jour immédiate de la vue. |
| **Création manuelle** | Création d’un RDV depuis le calendrier (clic sur un créneau libre ou bouton « Créer un RDV »). | Formulaire : client (recherche ou saisie), service, créneau (pré-rempli si clic sur créneau), remarque optionnelle ; vérification disponibilité en temps réel. |
| **Modification** | Clic sur un RDV pour modifier (créneau, service, client) ou annuler. | Formulaire d’édition ; motif d’annulation optionnel ; notification client si configurée. |
| **Annulation** | Annulation d’un RDV avec motif optionnel ; application de la politique d’annulation (pénalité si délai dépassé). | Confirmation avant annulation ; enregistrement du motif ; notification client ; libération du créneau. |

Le calendrier est le **livrable central** pour la gestion opérationnelle ; il doit refléter en temps réel les réservations (pas de double réservation) et se charger en moins de 2 secondes (NFR-PRO-02).

### 4.3 Gestion des services (types de RDV)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Liste des services** | Liste de tous les types de RDV proposés (consultation, séance, cours, etc.). | Tableau ou cartes : nom, durée, tarif, statut (actif/inactif), nombre de RDV à venir (optionnel). |
| **Création** | Ajout d’un nouveau service : nom, durée, description, tarif optionnel, acompte optionnel, association à un ou plusieurs praticiens ou ressources. | Formulaire avec champs obligatoires (nom, durée) et optionnels (description, tarif, acompte, ressources). |
| **Édition** | Modification d’un service existant ; désactivation (les créneaux passés restent visibles, les nouveaux RDV ne peuvent plus être pris pour ce service). | Formulaire d’édition ; alerte si des RDV à venir existent pour ce service. |
| **Suppression** | Suppression d’un service (bloquée si des RDV passés ou à venir existent ; proposer la désactivation). | Message d’erreur ou proposition de désactivation. |
| **Buffer time** | Temps minimum entre deux RDV consécutifs pour ce service (ex. 10 min). | Champ numérique (minutes) dans le formulaire service ou au niveau praticien. |
| **Préavis** | Préavis minimum (réservation à l’avance) et préavis maximum (réservation jusqu’à X mois) par service ou global. | Champs dans le formulaire ou dans les paramètres globaux. |

### 4.4 Gestion des plannings et disponibilités

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Horaires récurrents** | Définition des plages horaires par jour de la semaine (ex. lun–ven 9h–18h, sam 9h–12h). | Formulaire par jour (ou réplication « Dupliquer sur tous les jours ») ; plages début–fin ; application par défaut à tous les praticiens ou par praticien. |
| **Exceptions** | Blocage de plages ou de jours (congés, formation, absence). | Création d’une exception : date ou plage de dates, plage horaire (ou toute la journée), motif optionnel ; les créneaux concernés ne sont pas proposés au client. |
| **Jours fériés** | Liste des jours fériés (pays/région) ou saisie manuelle. | Import d’un calendrier jours fériés ou saisie manuelle ; blocage automatique des créneaux. |
| **Synchronisation agendas externes** | Connexion à Google Calendar, Outlook, Apple iCal (OAuth ou équivalent) ; synchronisation bidirectionnelle. | Page « Connexions » : bouton « Connecter Google » (etc.) ; les événements externes bloquent les créneaux dans JayRDV ; les RDV JayRDV sont écrits dans l’agenda externe. |
| **Multi-praticiens** | Plusieurs praticiens avec plannings distincts ; le client choisit le praticien ou le système propose (round-robin). | Liste des praticiens ; plannings séparés dans le calendrier ; paramètre « Premier disponible » ou « Choix du client » par service. |
| **Ressources** | Salles, équipements réservables ; association ressource ↔ service. | Création de ressources (nom, type) ; plannings par ressource ; association à un ou plusieurs services ; vérification disponibilité ressource à la prise de RDV. |

### 4.5 Notifications et rappels

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Confirmation client** | Envoi automatique d’un email et/ou SMS au client à la prise de RDV. | Modèle de message personnalisable (logo, texte, variables : nom client, date, heure, service, lien ajout à l’agenda) ; canal(s) configurable(s). |
| **Rappel client** | Envoi automatique d’un rappel la veille et/ou quelques heures avant le RDV. | Paramétrage du délai (24 h, 2 h, etc.) et du canal ; modèle personnalisable ; objectif : réduction des no-show. |
| **Notification professionnel** | Notification au professionnel à chaque nouvelle réservation (email, push, in-app). | Contenu : client, créneau, service ; paramètre activable/désactivable. |
| **Alerte désistement** | Si un créneau plus tôt se libère, notification au client inscrit en liste d’attente (optionnel). | Enregistrement des souhaits « créneau plus tôt » ; envoi d’une notification si libération ; lien vers reprise de RDV. |
| **Personnalisation** | Éditeur de modèles pour confirmation, rappel, annulation ; variables dynamiques. | Éditeur de texte avec variables (nom, date, heure, service, lien) ; prévisualisation. |
| **Suivi post-RDV** | Relance ou questionnaire après le RDV (optionnel). | Paramétrage du délai et du canal ; modèle de message ; lien vers questionnaire externe ou intégré. |

Les notifications s’appuient sur Miyunotify (ou équivalent) ; les modèles et les canaux sont configurables par le professionnel dans son espace.

### 4.6 Paiements et gestion commerciale

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Paiement en ligne** | Paiement (intégral ou acompte) à la réservation ; intégration avec passerelle (Miyuinvoice ou partenaire). | Paramètre par service (tarif, acompte) ; page de paiement sécurisée (PCI, 3D Secure) ; confirmation du paiement et du RDV. |
| **Politique d’annulation** | Délai gratuit (ex. 24 h), pénalité après ce délai (montant ou %). | Paramètres dans l’espace pro ; application automatique ou manuelle à l’annulation ; enregistrement du motif. |
| **Historique des paiements** | Liste des transactions par période, par client ; export pour comptabilité. | Tableau avec filtres ; export CSV/PDF ; lien avec Miyuinvoice si activé. |
| **Fiche client** | Consulter et éditer la fiche client (nom, contact, préférences) ; historique des RDV par client. | Fiche détail ; liste des RDV passés et à venir ; notes optionnelles. |
| **CRM léger** | Champs personnalisés, notes, tags sur le client. | Configuration des champs par le professionnel ; recherche et filtres par tag ou note. |

### 4.7 Statistiques et analytics

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Dashboard synthétique** | Indicateurs : nombre de RDV (jour, semaine, mois), taux de remplissage, taux de no-show. | Bloc dashboard : chiffres clés, graphiques (évolution dans le temps). |
| **Analyses d’activité** | Volume de RDV par période, par service, par praticien, par établissement. | Graphiques et tableaux ; filtres par période, praticien, établissement. |
| **Taux de no-show** | Calcul automatique (RDV marqués no-show / RDV confirmés) ; tendance ; comparaison avant/après rappels. | Indicateur dans le dashboard ; rapport dédié. |
| **Créneaux les plus demandés** | Identification des créneaux (jour, heure) les plus réservés. | Heatmap ou tableau ; aide à l’optimisation des plannings. |
| **Export** | Export des RDV et des statistiques (CSV, PDF) par période. | Bouton « Exporter » ; choix du format et de la période. |

### 4.8 Intégrations et exposition

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Lien de réservation** | Lien unique à partager (site web, email, réseaux sociaux) ; paramètres optionnels (service pré-sélectionné, praticien). | Génération du lien dans l’espace pro ; copier-coller ; prévisualisation de la page de réservation. |
| **Widget** | Code d’intégration (iframe ou script) pour le site du professionnel ; personnalisation (couleurs, service par défaut). | Page « Intégrations » : code à copier ; options de personnalisation (couleur, service, praticien). |
| **API REST** | Accès programmatique aux créneaux, aux réservations et aux annulations. | Documentation API ; authentification (token, OAuth) ; endpoints : liste créneaux, créer RDV, annuler RDV. |
| **Webhooks** | Notifications en temps réel (nouveau RDV, annulation, modification) vers une URL de callback. | Configuration des événements et de l’URL ; payload JSON ; retry et signature pour sécurisation. |
| **Ajout à l’agenda client** | Lien « Ajouter à mon agenda » dans l’email de confirmation ; génération de fichier iCal. | Lien dans le modèle de confirmation ; téléchargement iCal (Google, Outlook, Apple). |
| **Plugin CMS** | Plugin WordPress (ou autre) pour afficher le widget. | Lien vers dépôt ou téléchargement ; configuration (lien pro, service, couleurs). |
| **Zapier / Make** | Connexion à des outils tiers (CRM, emailing, compta). | Connexion Zapier/Make ; triggers (nouveau RDV, annulation) ; actions (créer RDV) ; documentation. |

### 4.9 Cours, ateliers et créneaux collectifs

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Création de cours** | Définition d’un créneau collectif : nom, date/heure, durée, nombre de places, prix optionnel ; récurrence possible. | Formulaire : nom, date/heure, durée, places, prix ; option récurrence (hebdo, bi-hebdo, mensuel). |
| **Gestion des inscriptions** | Liste des inscrits ; statut (inscrit, waitlist, annulé) ; ouverture/fermeture des inscriptions. | Tableau des participants ; bouton « Fermer les inscriptions » (paramétrable à J-1 ou manuel). |
| **Liste d’attente** | Inscription en liste d’attente si le créneau est complet ; notification en cas de désistement. | Bouton « Rejoindre la liste d’attente » côté client ; alerte au pro si place libérée ; proposition au premier de la liste. |
| **Récurrence** | Génération des occurrences (ex. tous les mardis pendant 10 semaines) ; édition groupée ou individuelle. | Paramètre de récurrence ; génération des occurrences ; édition d’une occurrence sans impacter les autres (optionnel). |

### 4.10 Multi-établissements

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Liste des établissements** | Création, édition, désactivation d’établissements (sites, adresses). | Liste des établissements ; formulaire par établissement (nom, adresse, contact, horaires). |
| **Paramétrage par établissement** | Plannings, services, praticiens par établissement ; vue consolidée ou par établissement. | Onglets ou filtre par établissement dans le calendrier et les paramètres. |
| **Lien par établissement** | Lien de réservation par établissement (ou par praticien) pour pré-sélectionner le lieu. | Génération de liens distincts ; paramètre « établissement » ou « praticien » dans l’URL. |
| **Statistiques par établissement** | Filtre par établissement dans le dashboard et les rapports ; comparaison entre établissements. | Filtre « Établissement » dans les vues stats ; tableau comparatif (optionnel). |

---

## 5. Limites et gouvernance

| Aspect | Règle |
|--------|--------|
| **Données** | Les données d’un professionnel (RDV, clients, plannings) sont souveraines dans le cadre de son Mandat ; pas d’accès aux données des autres professionnels. |
| **Mandat** | Les actions (configurer des services, des plannings, gérer des RDV, exposer un lien) sont encadrées par le Mandat de Permission (StrongFather, Master Butler). |
| **Révocation** | Le Mandat peut être révoqué (conditions définies par la gouvernance) ; le professionnel perd alors l’accès aux actions concernées. |
| **Créneau unique** | Un même créneau (praticien ou ressource) ne peut être réservé qu’une seule fois ; vérification en temps réel à la prise de RDV. |
| **Exposition** | Le professionnel expose uniquement les créneaux disponibles ; l’agenda détaillé (noms des clients, blocages personnels) reste masqué côté client. |
| **Rôles** | Un Praticien ne voit que son planning ; un Gestionnaire ne voit que les établissements assignés ; un Admin a accès complet. |

---

## 6. Synthèse des livrables par bloc

| Bloc | Livrable principal | Objectif |
|------|--------------------|----------|
| **Dashboard** | Page d’accueil avec RDV du jour/semaine, indicateurs, actions rapides. | Vue synthétique et accès direct aux capacités. |
| **Calendrier** | Vue jour/semaine/mois, création/modification/annulation RDV, filtres. | Gestion opérationnelle des RDV. |
| **Services** | Liste, création, édition des types de RDV (nom, durée, tarif, buffer, préavis). | Définition de l’offre proposée aux clients. |
| **Plannings** | Horaires récurrents, exceptions, jours fériés, synchro agendas, ressources. | Définition des disponibilités. |
| **Notifications** | Confirmation, rappels, notification pro, personnalisation des messages. | Réduction des no-show et suivi en temps réel. |
| **Paiement** | Paiement en ligne, acompte, politique d’annulation, historique, fiche client. | Sécurisation des revenus et relation client. |
| **Stats** | Dashboard synthétique, analyses, taux no-show, export. | Pilotage de l’activité. |
| **Intégrations** | Lien, widget, API, webhooks, ajout agenda client, plugin, Zapier. | Exposition 24h/24 et connexion aux outils métier. |
| **Équipe** | Invitation, rôles (Admin, Gestionnaire, Praticien), délégation par établissement. | Délégation opérationnelle. |
| **Cours / ateliers** | Créneaux collectifs, places limitées, liste d’attente, récurrence. | Offre différenciante (cours, ateliers). |
| **Multi-établissements** | Liste établissements, paramétrage par site, lien par site, stats par site. | Pilotage multi-sites. |

---

## 7. Parcours détaillés (flows)

### 7.1 Flow : Premier paramétrage après inscription

1. Le professionnel se connecte pour la première fois après validation de son compte.
2. Il est redirigé vers une page « Configuration initiale » ou un assistant en plusieurs étapes.
3. **Étape 1 — Services** : Il crée au moins un service (ex. « Consultation 30 min », durée 30 min, tarif optionnel). Il peut en ajouter plusieurs (ex. « Séance 1 h », « Bilan »).
4. **Étape 2 — Planning** : Il définit ses horaires récurrents (ex. lun–ven 9h–18h, sam 9h–12h). Il peut définir des plages différentes par jour. Option : bloquer des exceptions (congés, absence) dès cette étape.
5. **Étape 3 — Lien de réservation** : Le système génère son lien unique. Il peut le copier et le prévisualiser. Option : personnaliser le texte de la page (message d’accueil, instructions).
6. **Étape 4 — Notifications** : Il active ou désactive la confirmation client (email/SMS) et les rappels (24 h, 2 h avant). Il peut personnaliser les modèles de message.
7. À la fin, il accède au tableau de bord avec un message de succès : « Votre lien de réservation est prêt. Partagez-le pour recevoir vos premiers RDV. »

**Livrables sollicités** : Dashboard, Services, Plannings, Lien de réservation, Notifications.

### 7.2 Flow : Création manuelle d’un RDV

1. Le professionnel (ou le Gestionnaire) est sur le tableau de bord ou le calendrier.
2. Il clique sur « Créer un RDV » ou sur un créneau libre dans le calendrier.
3. Un formulaire s’ouvre : **Client** (recherche dans la base ou saisie nom/email/téléphone pour un nouveau client), **Service** (liste déroulante), **Date et heure** (pré-remplies si clic sur créneau), **Praticien** (si multi-praticiens), **Remarque** (optionnel).
4. Le système vérifie en temps réel que le créneau est disponible (praticien et ressource si applicable). Si le créneau est déjà pris, un message d’erreur s’affiche et propose de choisir un autre créneau.
5. Il valide ; le RDV est créé. Option : envoyer une confirmation au client (email/SMS). Le RDV apparaît dans le calendrier et dans la liste « RDV du jour » ou « RDV de la semaine ».

**Livrables sollicités** : Calendrier, Fiche client (recherche ou création), Notifications (confirmation client).

### 7.3 Flow : Modification du planning (exception, congés)

1. Le professionnel (ou l’Admin) accède à « Plannings » ou « Disponibilités ».
2. Il sélectionne le praticien (si multi-praticiens) et la période concernée.
3. Il clique sur « Ajouter une exception » (ou « Bloquer un créneau »).
4. Il saisit : **Date** ou **Plage de dates** (ex. 15/02/2026 – 20/02/2026), **Plage horaire** (ex. toute la journée ou 9h–12h), **Motif** (optionnel, ex. « Congés »).
5. Il valide ; les créneaux concernés sont bloqués et ne sont plus proposés aux clients. Ils apparaissent en grisé ou avec une légende « Indisponible » dans le calendrier pro.
6. Si des RDV existaient déjà sur ces créneaux, le système alerte : « X RDV sont concernés. Souhaitez-vous les déplacer ou les annuler ? » Le professionnel choisit et les clients sont notifiés si configuré.

**Livrables sollicités** : Plannings (exceptions), Calendrier, Notifications (annulation/déplacement).

### 7.4 Flow : Intégration du widget sur le site

1. Le professionnel accède à « Intégrations » ou « Lien et widget » dans son espace.
2. Il voit son **lien de réservation** (URL) avec bouton « Copier ». Il peut prévisualiser la page de réservation dans un nouvel onglet.
3. Il clique sur l’onglet **Widget** : choix entre « Bouton » (lien simple), « Iframe » (formulaire intégré dans la page), « Overlay » (popup au clic).
4. Il sélectionne les options : **Couleur** du bouton ou du thème, **Service pré-sélectionné** (optionnel), **Praticien pré-sélectionné** (optionnel).
5. Il copie le code (HTML ou script) et le colle dans le code source de son site (page « Prendre rendez-vous » ou équivalent).
6. Il enregistre et teste sur son site : le widget s’affiche et les créneaux disponibles s’affichent après sélection du service (et éventuellement du praticien). La réservation se fait sans quitter le site (iframe) ou en ouvrant la page JayRDV (bouton).

**Livrables sollicités** : Lien de réservation, Widget, Paramétrage (couleur, service, praticien).

### 7.5 Flow : Consultation des statistiques et export

1. Le professionnel accède au **tableau de bord** ou à la section **Statistiques**.
2. Il voit les indicateurs par défaut : RDV du jour, de la semaine, du mois ; taux de remplissage (optionnel) ; taux de no-show (optionnel).
3. Il peut filtrer par **période** (date de début – date de fin), par **praticien**, par **établissement** (si multi-établissements).
4. Il consulte les graphiques : évolution du nombre de RDV dans le temps, répartition par service, par praticien.
5. Il clique sur **Export** : choix du format (CSV, PDF), de la période et du contenu (liste des RDV, résumé stats). Le fichier est généré et téléchargé.

**Livrables sollicités** : Dashboard, Statistiques, Export.

---

## 8. Critères d’acceptation par livrable

### 8.1 Dashboard

- Le dashboard s’affiche en moins de 3 secondes après connexion (NFR-PRO-01).
- Les RDV du jour sont affichés avec heure, client, service, praticien ; clic pour ouvrir le détail ou modifier.
- Les indicateurs (nombre de RDV jour/semaine/mois) sont corrects et mis à jour après chaque création/modification/annulation.
- Les actions rapides (Créer un RDV, Voir le calendrier, Paramètres, Lien) sont accessibles en un clic.
- Le dashboard est responsive (desktop et tablette).

### 8.2 Calendrier

- La vue jour/semaine/mois se charge en moins de 2 secondes (NFR-PRO-02).
- Les RDV sont affichés avec une couleur ou un libellé par type de service ou par statut ; les créneaux libres sont identifiables (ou masqués selon paramètre).
- La création d’un RDV depuis le calendrier (clic sur créneau libre) pré-remplit la date et l’heure ; la vérification de disponibilité est faite en temps réel.
- La modification ou l’annulation d’un RDV depuis le calendrier met à jour la vue immédiatement et envoie la notification client si configurée.
- Les filtres (praticien, ressource, établissement) mettent à jour la vue sans rechargement complet de la page.

### 8.3 Services

- La liste des services affiche au minimum : nom, durée, statut (actif/inactif). Option : tarif, nombre de RDV à venir.
- La création d’un service exige au minimum : nom, durée. Les champs optionnels (description, tarif, acompte, buffer, préavis, ressources) sont sauvegardés si renseignés.
- L’édition d’un service existant ne supprime pas les RDV passés ni à venir ; une désactivation masque le service pour les nouvelles réservations uniquement.
- La suppression d’un service est bloquée si des RDV (passés ou à venir) existent pour ce service ; un message propose la désactivation.

### 8.4 Plannings

- Les horaires récurrents sont définis par jour de la semaine avec au moins une plage début–fin. Les créneaux proposés au client respectent ces plages.
- Les exceptions (date ou plage, plage horaire, motif) bloquent les créneaux concernés ; ils ne sont plus proposés au client.
- Les jours fériés (saisie manuelle ou import) bloquent les créneaux du jour.
- La synchronisation avec un agenda externe (Google, Outlook, Apple) : après connexion OAuth, les événements externes bloquent les créneaux dans JayRDV ; les RDV JayRDV sont écrits dans l’agenda externe (option configurable).

### 8.5 Notifications

- La confirmation client est envoyée automatiquement à la prise de RDV (par un client ou par le pro en création manuelle) si l’option est activée ; le canal (email, SMS) et le modèle sont configurables.
- Le rappel client est envoyé au délai configuré (ex. 24 h et 2 h avant) ; le modèle est personnalisable ; l’envoi est tracé (log).
- La notification au professionnel (nouveau RDV) est envoyée si l’option est activée ; le canal (email, push, in-app) est configurable.

### 8.6 Lien et widget

- Le lien de réservation généré est unique et stable ; il affiche la page de réservation avec les services et les créneaux disponibles du professionnel. Les créneaux sont calculés en temps réel (latence < 2 s après une réservation).
- Le widget (iframe ou script) s’intègre sur un site tiers ; les options de personnalisation (couleur, service pré-sélectionné) sont appliquées. La réservation aboutit à la création du RDV dans JayRDV et à l’envoi de la confirmation client.

### 8.7 Équipe et rôles

- Un Admin peut inviter un membre par email et lui attribuer le rôle Gestionnaire ou Praticien. Le Gestionnaire peut être assigné à un ou plusieurs établissements ; le Praticien est associé à un ou plusieurs établissements.
- Un Praticien ne voit que son propre planning et ses RDV ; il peut modifier ses indisponibilités (exceptions) mais pas les paramètres globaux (services, tarifs, notifications).
- Un Gestionnaire voit le calendrier des établissements assignés et peut créer/modifier/annuler des RDV ; il ne peut pas modifier les services, les plannings récurrents ni les paramètres de compte.

---

## 9. Cas limites et comportements attendus

| Cas | Comportement attendu |
|-----|----------------------|
| **Réservation simultanée** | Deux clients (ou un client et le pro) tentent de réserver le même créneau au même moment. Un seul obtient la réservation ; l’autre reçoit « Créneau non disponible » et peut en choisir un autre. Verrouillage optimiste ou pessimiste selon implémentation. |
| **Modification de planning avec RDV existants** | Le pro ajoute une exception qui chevauche un ou plusieurs RDV existants. Le système alerte et propose de déplacer ou annuler les RDV concernés ; notification client si configurée. |
| **Suppression d’un praticien** | Si le praticien a des RDV à venir, le système bloque la suppression ou propose de réaffecter les RDV à un autre praticien ou d’annuler avec notification client. |
| **Désactivation d’un service** | Les RDV à venir pour ce service restent valides ; les nouveaux clients ne peuvent plus choisir ce service. Le pro peut réactiver le service à tout moment. |
| **Synchro agenda externe en retard** | Les RDV déjà confirmés dans JayRDV restent bloqués même si la synchro avec l’agenda externe n’a pas encore mis à jour ; priorité aux données JayRDV. |
| **Client sans compte (guest)** | Le client réserve via le lien ou le widget sans créer de compte ; il saisit nom, email, téléphone. La confirmation est envoyée à l’email saisi. Le RDV apparaît dans le calendrier pro avec les infos client saisies. |

---

## 10. Écrans et zones fonctionnelles (description)

### 10.1 Écran Tableau de bord (accueil)

- **En-tête** : Logo / nom du service, menu (Calendrier, Services, Plannings, Notifications, Intégrations, Statistiques, Paramètres), profil utilisateur (nom, rôle, déconnexion).
- **Zone principale** : Bloc « RDV du jour » (liste horaire, client, service, praticien ; lien vers détail/modification). Bloc « RDV de la semaine » (aperçu 7 jours, clic pour ouvrir le calendrier). Bloc « Indicateurs » (nombre de RDV jour/semaine/mois, taux de remplissage, taux de no-show). Bloc « Actions rapides » (boutons : Créer un RDV, Voir le calendrier, Paramètres, Copier le lien de réservation).
- **Zone secondaire** (optionnel) : Dernières notifications (nouveau RDV, annulation), rappels envoyés.
- **Pied de page** : Liens légaux, aide, contact.

### 10.2 Écran Calendrier

- **En-tête** : Même que tableau de bord ; filtre par praticien, ressource, établissement (listes déroulantes ou onglets).
- **Zone principale** : Vue jour (grille horaire, colonnes par praticien ou par ressource ; RDV en blocs colorés ; créneaux libres en grisé ou masqués). Navigation (flèches jour précédent/suivant, date du jour). Bouton « Créer un RDV » (ouvre formulaire). Clic sur un RDV : menu contextuel (Modifier, Annuler). Clic sur un créneau libre : ouverture du formulaire de création avec date/heure pré-remplies.
- **Vue semaine** : Même principe ; grille 7 jours ; colonnes par praticien ou par ressource.
- **Vue mois** : Calendrier mensuel ; indicateur de charge (nombre de RDV par jour) ; clic sur un jour pour ouvrir la vue jour.

### 10.3 Écran Services

- **Liste** : Tableau ou cartes (nom, durée, tarif, statut actif/inactif, nombre de RDV à venir). Bouton « Ajouter un service ».
- **Formulaire création/édition** : Champs nom (obligatoire), durée (obligatoire, en minutes), description (optionnel), tarif (optionnel), acompte (optionnel), buffer time (optionnel), préavis min/max (optionnel), association à des praticiens ou ressources (optionnel). Boutons Enregistrer, Annuler. Alerte si édition d’un service avec RDV à venir (pas de blocage, simple information).

### 10.4 Écran Plannings / Disponibilités

- **Onglets ou sections** : Horaires récurrents, Exceptions, Jours fériés, Synchronisation agendas.
- **Horaires récurrents** : Formulaire par jour de la semaine (lun, mar, mer, jeu, ven, sam, dim) ; pour chaque jour : une ou plusieurs plages début–fin. Option « Dupliquer sur tous les jours ». Sélection du praticien ou « Tous » si un seul.
- **Exceptions** : Liste des exceptions (date ou plage, plage horaire, motif) ; bouton « Ajouter une exception ». Formulaire : date ou plage de dates, plage horaire (ou « Toute la journée »), motif (optionnel). Alerte si chevauchement avec des RDV existants.
- **Jours fériés** : Liste des jours bloqués ; bouton « Importer un calendrier » (pays/région) ou « Ajouter un jour » (saisie manuelle).
- **Synchronisation** : Boutons « Connecter Google Calendar », « Connecter Outlook », « Connecter Apple iCal ». Après connexion OAuth : statut « Connecté » ; option « Déconnecter ». Information : « Les événements de votre agenda externe bloquent les créneaux dans JayRDV. Les RDV JayRDV sont ajoutés à votre agenda externe. »

### 10.5 Écran Notifications

- **Confirmation client** : Activer/désactiver ; canal(s) (email, SMS) ; éditeur de modèle (texte, variables : nom client, date, heure, service, lien ajout à l’agenda). Prévisualisation.
- **Rappel client** : Activer/désactiver ; délai(s) (ex. 24 h et 2 h avant) ; canal(s) ; éditeur de modèle. Prévisualisation.
- **Notification professionnel** : Activer/désactiver ; canal(s) (email, push, in-app). Contenu : client, créneau, service.
- **Annonce désistement** : Activer/désactiver (si liste d’attente activée) ; modèle de message.

### 10.6 Écran Intégrations

- **Lien de réservation** : URL affichée avec bouton « Copier ». Options : service pré-sélectionné, praticien pré-sélectionné (listes déroulantes). Bouton « Prévisualiser » (ouvre la page de réservation dans un nouvel onglet).
- **Widget** : Choix du type (bouton, iframe, overlay). Options : couleur, service par défaut, praticien par défaut. Code HTML ou script affiché avec bouton « Copier ». Instructions d’intégration (où coller le code sur son site).
- **API** : Lien vers la documentation API ; section Authentification (token, OAuth) ; section Endpoints (liste créneaux, créer RDV, annuler RDV).
- **Webhooks** : Formulaire : URL de callback, événements sélectionnés (nouveau RDV, annulation, modification). Bouton « Tester » (envoi d’un payload de test). Affichage des derniers appels (log) avec statut (succès, erreur).

### 10.7 Écran Statistiques

- **Période** : Sélecteur date de début – date de fin (ou prédéfinis : aujourd’hui, cette semaine, ce mois, ce trimestre).
- **Filtres** : Praticien, établissement (si multi-établissements).
- **Bloc indicateurs** : Nombre total de RDV, taux de remplissage (%), taux de no-show (%). Comparaison avec période précédente (optionnel).
- **Graphiques** : Évolution du nombre de RDV dans le temps (courbe ou barres). Répartition par service (camembert ou barres). Répartition par praticien (barres). Créneaux les plus demandés (heatmap ou tableau).
- **Export** : Bouton « Exporter » ; choix du format (CSV, PDF), de la période et du contenu (liste des RDV, résumé stats). Téléchargement du fichier.

### 10.8 Écran Équipe (Admin uniquement)

- **Liste des membres** : Tableau (nom, email, rôle, établissements assignés, date d’invitation, statut). Bouton « Inviter un membre ».
- **Formulaire d’invitation** : Email, rôle (Admin, Gestionnaire, Praticien), établissements assignés (si Gestionnaire ou Praticien). Envoi de l’invitation par email ; le membre reçoit un lien pour accepter et créer son mot de passe (ou lien magique).
- **Édition d’un membre** : Modification du rôle ou des établissements assignés ; désactivation du compte (révoquer l’accès).

### 10.9 Synthèse des écrans par rôle

| Écran | Admin | Gestionnaire | Praticien |
|-------|-------|--------------|-----------|
| **Tableau de bord** | Oui (tous les RDV, tous les établissements) | Oui (RDV des établissements assignés) | Oui (ses RDV uniquement) |
| **Calendrier** | Oui (tous) | Oui (établissements assignés) | Oui (son planning) |
| **Services** | Oui (création, édition) | Non | Non |
| **Plannings** | Oui (tous) | Non (ou lecture seule selon config) | Oui (ses exceptions uniquement) |
| **Notifications** | Oui (paramétrage) | Non | Non |
| **Paiement** | Oui | Lecture (historique) | Non |
| **Statistiques** | Oui (tous) | Oui (établissements assignés) | Non ou limité (ses stats) |
| **Intégrations** | Oui (lien, widget, API, webhooks) | Non | Non |
| **Équipe** | Oui (invitation, rôles) | Non | Non |
| **Paramètres compte** | Oui | Non | Non |
| **Fiche client** | Oui (tous) | Oui (établissements assignés) | Oui (clients de ses RDV) |

Cette matrice permet de valider les droits d’accès par rôle et d’aligner l’implémentation (Master Butler, permissions) avec les livrables décrits dans ce document.

### 10.10 Checklist livrables MVP (Professionnels)

Pour la première version livrable (MVP) du public Professionnels, les livrables suivants sont attendus :

- [ ] **Inscription et validation** : Formulaire d’inscription pro, validation email (optionnel), attribution du rôle Admin ou Praticien.
- [ ] **Services** : Création d’au moins un service (nom, durée) ; liste et édition.
- [ ] **Plannings** : Horaires récurrents par jour de la semaine ; exceptions (congés, absences) ; jours fériés (saisie ou import).
- [ ] **Calendrier** : Vue jour et semaine ; création manuelle de RDV (client, service, créneau) ; modification et annulation ; pas de double réservation.
- [ ] **Lien de réservation** : Génération d’un lien unique ; page de réservation publique avec services et créneaux disponibles ; copier-coller et prévisualisation.
- [ ] **Notifications** : Confirmation client (email et/ou SMS) à la réservation ; rappel client (24 h et/ou 2 h avant) ; notification pro (nouveau RDV) ; modèles personnalisables.
- [ ] **Dashboard** : RDV du jour, RDV de la semaine, indicateurs (nombre de RDV jour/semaine/mois) ; actions rapides (Créer un RDV, Calendrier, Paramètres, Lien).
- [ ] **Rôles** : Admin (accès complet) ; Praticien (son planning et ses RDV, ses exceptions). Option : Gestionnaire (calendrier et RDV des établissements assignés).
- [ ] **Sécurité** : Authentification (Miyauth), permissions (Master Butler), isolation des données par professionnel, traçabilité des actions sensibles.

Les livrables **widget**, **API**, **webhooks**, **paiement en ligne**, **statistiques avancées**, **multi-établissements**, **cours/ateliers** et **liste d’attente** sont en scope **Should** ou **Could** pour les versions suivantes (voir [Analyse des besoins](./Professionnels%20-%20Analyse%20des%20besoins.md) § 6 Priorisation).

### 10.11 Dépendances techniques (référence)

Les livrables Professionnels s’appuient sur les composants et Kits Miyukini suivants (référence pour l’implémentation ; hors périmètre détaillé de ce document) :

| Livrable | Composant / Kit | Rôle |
|----------|-----------------|------|
| Authentification, rôles, permissions | Miyauth, Master Butler | Compte pro, Mandat, isolation des données. |
| Profil professionnel | Miyuprofile | Données structure, établissement(s). |
| Notifications (email, SMS) | Miyunotify | Confirmation, rappels, notification pro. |
| Paiement (si activé) | Miyuinvoice ou partenaire | Paiement en ligne, acompte, historique. |
| Calendrier, créneaux, RDV | Miyubooking, MiyuClock | Gestion des plannings, des créneaux et des RDV. |
| Fiche client, CRM léger | Miyucontacts (ou équivalent) | Fiche client, historique RDV, notes. |
| Persistance, cohérence | KindMother | Données RDV, plannings, clients. |
| Sécurité, audit | WorrySentinel, traçabilité | Niveaux de sécurité, audit des actions. |

La répartition exacte entre Opérateurs, Kits d’outils et Cores est définie dans les spécifications techniques (hors scope de ce document).

### 10.12 Récapitulatif des parcours principaux

| Parcours | Déclencheur | Étapes clés | Livrables sollicités |
|----------|-------------|-------------|----------------------|
| **Premier paramétrage** | Connexion première fois après validation compte | Services → Plannings → Lien → Notifications | Services, Plannings, Lien, Notifications, Dashboard |
| **Création manuelle RDV** | Clic « Créer un RDV » ou clic sur créneau libre | Formulaire client/service/créneau → Vérification dispo → Validation | Calendrier, Fiche client, Notifications |
| **Modification planning** | Ajout exception (congés, absence) | Exceptions → Saisie dates/plage → Alerte RDV existants | Plannings, Calendrier, Notifications |
| **Intégration widget** | Pro souhaite mettre le widget sur son site | Intégrations → Choix type/options → Copier code → Coller sur site | Lien, Widget |
| **Consultation stats** | Pro consulte son activité | Dashboard ou Statistiques → Filtres période/praticien → Export | Dashboard, Statistiques, Export |
| **Invitation équipe** | Admin invite un Gestionnaire ou Praticien | Équipe → Inviter → Email/rôle/établissements → Envoi | Équipe, Permissions (Master Butler) |

Ce récapitulatif sert de base pour les tests d’acceptation et la validation des parcours par les équipes produit et QA.

---

## 11. Références documentaires

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md) — vision, raison d’être, principes.
- [Professionnels — Analyse des besoins](./Professionnels%20-%20Analyse%20des%20besoins.md) — besoins fonctionnels et non fonctionnels, user stories, priorisation MoSCoW.
- [Public Clients](../Clients/_index.md) — parcours et livrables côté client (réservation, compte client).
- [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md) — parcours guest (réservation sans compte).
- [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md) — benchmark marché (référence pour le positionnement des livrables).

---

**Document** : Professionnels — Parcours, capacités et livrables  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence pour le public Professionnels

---

*Ce document est aligné avec l’[analyse des besoins](./Professionnels%20-%20Analyse%20des%20besoins.md) du public Professionnels et avec le [benchmark des fonctionnalités des solutions de réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md). Toute évolution des livrables (ajout de capacités, modification des parcours ou des critères d’acceptation) doit être répercutée dans l’analyse des besoins et dans les spécifications techniques.*

### Index des sections (Parcours Professionnels)

1. **Profil du public** — Qui, compte, accès, espace.  
2. **Parcours utilisateur** — Onboarding, cycle de vie (connexion, dashboard, calendrier, paramétrage, exposition, stats, équipe), passerelles (clients, utilisateur non connecté, outils externes).  
3. **Rôles** — Admin, Gestionnaire, Praticien ; délégation par établissement.  
4. **Capacités et livrables** — Dashboard, calendrier centralisé, services, plannings, notifications, paiement, stats, intégrations (lien, widget, API, webhooks), cours/ateliers, multi-établissements.  
5. **Limites et gouvernance** — Données, Mandat, révocation, créneau unique, exposition, rôles.  
6. **Synthèse des livrables par bloc** — Tableau récapitulatif.  
7. **Parcours détaillés (flows)** — Premier paramétrage, création manuelle RDV, modification planning, intégration widget, consultation stats.  
8. **Critères d’acceptation par livrable** — Dashboard, calendrier, services, plannings, notifications, lien/widget, équipe.  
9. **Cas limites** — Réservation simultanée, modification planning avec RDV, suppression praticien, désactivation service, synchro agenda, client guest.  
10. **Écrans et zones fonctionnelles** — Tableau de bord, calendrier, services, plannings, notifications, intégrations, stats, équipe ; synthèse par rôle ; checklist MVP ; dépendances techniques ; récap parcours.  
11. **Références documentaires** — Liens vers document fondateur, analyse des besoins, publics Clients et Utilisateur non connecté, benchmark marché.

*Fin du document — Professionnels — Parcours, capacités et livrables (JayRDV).*
