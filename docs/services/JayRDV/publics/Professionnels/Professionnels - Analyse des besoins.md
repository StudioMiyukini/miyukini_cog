# Professionnels — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du public cible **Professionnels** pour le service JayRDV. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels, les parcours détaillés, les user stories, les pain points et opportunités, ainsi que la priorisation et les dépendances. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md), [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md), [Parcours, capacités et livrables](./Professionnels%20-%20Parcours%20Capacites%20Livrables.md).

## Portée / Scope

- **Public** : Professionnels (praticiens, entreprises, équipes) qui proposent des créneaux de réservation et gèrent leurs plannings.
- **Périmètre** : Tous les besoins identifiés pour ce public (fonctionnels, non fonctionnels, parcours, scénarios, priorisation).
- **Hors périmètre** : Spécifications techniques d’implémentation (API, schémas de données détaillés), spécifications des autres publics (clients, utilisateur non connecté) — traitées dans leurs propres documents.

---

## 1. Profil du public et personas

### 1.1 Définition du public

Les **professionnels** sont des personnes physiques ou morales (praticiens, entreprises, équipes) qui **proposent des créneaux de réservation** à leurs clients. Ils disposent d’un **compte professionnel** (mono ou multi-établissement selon offre) et opèrent dans le cadre de la gouvernance COG (StrongFather, Master Butler, KindMother, WorrySentinel). Ils ne peuvent pas accéder aux données des autres professionnels ni modifier la gouvernance plateforme.

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Praticien solo** | Médecin, thérapeute, coiffeur ; un seul agenda ; peu de temps pour l’admin. | Recevoir des RDV 24h/24 sans gérer les appels, réduire les no-show, synchroniser avec son agenda perso. | Double saisie agenda, oublis de rappel, créneaux pris alors qu’il est absent. |
| **Cabinet / petit groupe** | 2 à 5 praticiens ; un secrétariat ou réception partagée. | Un calendrier partagé, répartition des RDV (round-robin ou par praticien), rappels automatiques. | Conflits de créneaux, manque de visibilité sur les plannings des autres. |
| **Responsable multi-établissements** | Réseau de cabinets, franchises ; plusieurs sites. | Piloter plusieurs établissements, stats consolidées, paramétrage centralisé avec délégation locale. | Outils différents par site, pas de vue globale, reporting manuel. |
| **Prestataire avec ressources** | Salles, équipements, machines à réserver en plus des personnes. | Associer des ressources (salle, matériel) aux RDV, éviter les doubles réservations de ressources. | Ressources réservées deux fois, pas de vue ressource vs personne. |
| **Pro avec cours / ateliers** | Formateur, coach ; créneaux collectifs avec places limitées. | Proposer des cours ou ateliers avec inscription en ligne, gestion des places, liste d’attente. | Gestion manuelle des inscriptions, pas d’alerte désistement. |

### 1.3 Contexte d’usage

- **Fréquence** : Connexion quotidienne ou plusieurs fois par jour pour consulter le calendrier, gérer les RDV, paramétrer les plannings.
- **Appareils** : Desktop pour la configuration et la gestion avancée ; mobile et tablette pour la consultation et les modifications rapides (déplacement, annulation).
- **Concurrence** : Agendas papier, tableurs, outils métier (logiciels de cabinet), solutions de réservation tierces (Calendly, Doctolib, etc.) ; attente d’un **guichet unique** pour la réservation et la synchronisation.

---

## 2. Besoins fonctionnels

### 2.1 Onboarding et compte professionnel

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-01 | Création de compte professionnel | Pouvoir s’inscrire en tant que professionnel (email, mot de passe ou lien magique, informations structure, type d’activité). | Formulaire d’inscription dédié ; validation email si configurée ; création du profil pro (Miyauth, Miyuprofile). |
| PRO-02 | Validation du compte | Le compte peut être validé manuellement (plateforme) ou automatiquement selon politique. | Workflow de validation configurable ; notification au professionnel (validé / en attente / refusé). |
| PRO-03 | Attribution des permissions | Attribution du rôle professionnel (Admin, Gestionnaire, Praticien) et émission du Mandat de Permission. | Rôle et Mandat attribués après validation ; accès à l’espace pro selon le rôle. |
| PRO-04 | Compte mono ou multi-établissement | Un même professionnel peut gérer un ou plusieurs établissements (lieux, sites) selon offre. | Liste des établissements ; création d’un nouvel établissement depuis le même compte si autorisé par l’offre. |
| PRO-05 | Rattachement à une structure existante | Pouvoir rejoindre une structure existante (cabinet, réseau) sur invitation. | Flux d’invitation (email) ; acceptation et rattachement ; attribution d’un rôle au sein de la structure. |
| PRO-06 | Paramétrage du profil public | Configurer les informations visibles par les clients (nom, spécialité, photo, description, lien site web). | Formulaire de paramétrage ; prévisualisation côté client ; publication sur la page de réservation. |

### 2.2 Gestion des services et des créneaux types

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-07 | Création de services (types de RDV) | Définir les types de rendez-vous proposés (consultation, coupe, séance, etc.) avec durée, description, tarif optionnel. | Formulaire de création ; nom, durée, description, tarif ; association à un ou plusieurs praticiens ou ressources. |
| PRO-08 | Liste et édition des services | Consulter et modifier la liste des services ; activer/désactiver un service. | Liste des services ; édition, suppression (avec vérification des RDV existants) ; statut actif/inactif. |
| PRO-09 | Durées variables par service | Définir une durée par service (ex. 30 min, 1 h) ; durée par défaut et éventuellement durée personnalisable. | Champ durée obligatoire ; option « durée variable » pour permettre au client de choisir (ex. 30 ou 45 min). |
| PRO-10 | Tarification et acompte | Associer un tarif à un service ; exiger un acompte ou un paiement à la réservation. | Champs tarif, acompte (% ou montant fixe) ; intégration avec module paiement (Miyuinvoice ou partenaire). |
| PRO-11 | Services avec ressources | Associer un ou plusieurs types de ressources (salle, équipement) à un service. | Sélection des ressources requises par service ; vérification de disponibilité ressource lors de la prise de RDV. |
| PRO-12 | Buffer time (temps tampon) | Définir un temps minimum entre deux RDV (ex. 10 min) pour un service ou un praticien. | Paramètre par service ou par praticien ; prise en compte dans le calcul des disponibilités. |
| PRO-13 | Préavis minimum de réservation | Définir le délai minimum entre la réservation et le créneau (ex. 24 h, 2 h). | Paramètre global ou par service ; les créneaux non éligibles ne sont pas proposés au client. |
| PRO-14 | Préavis maximum (réservation à l’avance) | Définir jusqu’à quand un client peut réserver (ex. 3 mois à l’avance). | Paramètre global ou par service ; masquage des créneaux au-delà de la limite. |

### 2.3 Gestion des plannings et disponibilités

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-15 | Définition des créneaux récurrents | Définir les horaires de travail par jour de la semaine (ex. lun–ven 9h–18h, sam 9h–12h). | Formulaire par jour ; plages horaires ; application par défaut à tous les créneaux ou par praticien/ressource. |
| PRO-16 | Exceptions (congés, absences) | Bloquer des plages ou des jours (congés, formation, absence) pour un ou plusieurs praticiens ou ressources. | Création d’une exception (date, plage, motif optionnel) ; les créneaux concernés ne sont pas proposés. |
| PRO-17 | Jours fériés et fermetures | Définir les jours fériés et les fermetures exceptionnelles (global ou par établissement). | Liste des jours bloqués ; possibilité d’importer un calendrier jours fériés (pays/région). |
| PRO-18 | Synchronisation avec agendas externes | Synchroniser le calendrier JayRDV avec Google Calendar, Outlook, Apple iCal (lecture et écriture). | Connexion OAuth ou équivalent ; synchronisation bidirectionnelle ; les événements externes bloquent les créneaux. |
| PRO-19 | Éviter les doubles réservations | Un même créneau ne peut être réservé qu’une seule fois (par praticien ou par ressource). | Vérification en temps réel à la prise de RDV ; blocage si créneau déjà pris (par client ou par pro). |
| PRO-20 | Affichage des seules disponibilités côté client | Le client ne voit que les créneaux disponibles ; l’agenda détaillé du professionnel reste masqué. | Côté client : liste de créneaux disponibles uniquement ; pas d’accès aux RDV des autres clients ni aux blocages détaillés. |
| PRO-21 | Gestion multi-praticiens | Plusieurs praticiens avec plannings distincts ; le client choisit le praticien ou le système propose (round-robin). | Liste des praticiens ; plannings séparés ; option « premier disponible » ou choix explicite du praticien. |
| PRO-22 | Round-robin | Répartir les réservations entre plusieurs praticiens selon une règle (équilibrage, ordre). | Paramétrage de la règle ; attribution automatique du créneau à un praticien selon la règle. |
| PRO-23 | Gestion des ressources (salles, équipements) | Définir des ressources réservables et leurs plannings. | Création de ressources (nom, type) ; plannings par ressource ; association ressource ↔ service. |
| PRO-24 | Vue calendrier centralisée | Consulter tous les RDV (et optionnellement les blocages) dans une vue calendrier (jour, semaine, mois). | Vue jour/semaine/mois ; filtres par praticien, ressource, établissement ; couleurs par type de RDV ou statut. |
| PRO-25 | Création manuelle de RDV | Ajouter un RDV manuellement (client, créneau, service) depuis le tableau de bord. | Formulaire de création ; choix du créneau (avec vérification disponibilité) ; notification client optionnelle. |
| PRO-26 | Modification et annulation de RDV | Modifier un RDV (créneau, service, client) ou l’annuler ; motif d’annulation optionnel. | Actions Modifier / Annuler ; vérification des conflits à la modification ; notification client et politique d’annulation (délai, pénalité). |

### 2.4 Notifications et rappels

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-27 | Confirmation automatique au client | Envoyer une confirmation (email et/ou SMS) au client à la prise de RDV. | Déclenchement automatique ; contenu personnalisable (logo, texte, lien ajout à l’agenda) ; canal configurable. |
| PRO-28 | Rappels automatiques | Envoyer un rappel au client la veille ou quelques heures avant le RDV (email, SMS). | Paramétrage du délai (24 h, 2 h, etc.) et du canal ; réduction des no-show (objectif mesuré). |
| PRO-29 | Notification au professionnel | Notifier le professionnel à chaque nouvelle réservation (email, push, in-app). | Contenu : client, créneau, service ; paramétrage activable/désactivable. |
| PRO-30 | Rappel désistement / alerte libération | Prévenir le client si un créneau plus tôt se libère (option liste d’attente). | Enregistrement des souhaits « créneau plus tôt » ; envoi d’une notification si libération ; lien vers reprise de RDV. |
| PRO-31 | Personnalisation des messages | Personnaliser les modèles de messages (confirmation, rappel, annulation) avec logo, texte, variables. | Éditeur de modèles ; variables (nom client, date, heure, service, lien) ; prévisualisation. |
| PRO-32 | Suivi post-RDV (relance, questionnaire) | Déclencher une relance ou un questionnaire après le RDV (optionnel). | Paramétrage du délai et du canal ; modèle de message ; lien vers questionnaire externe ou intégré. |

### 2.5 Paiements et gestion commerciale

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-33 | Paiement en ligne à la réservation | Proposer le paiement (intégral ou acompte) lors de la prise de RDV. | Intégration avec passerelle de paiement (Miyuinvoice ou partenaire) ; sécurisation (PCI, 3D Secure). |
| PRO-34 | Acompte obligatoire ou optionnel | Exiger un acompte pour confirmer le RDV (par service ou global). | Paramètre par service ; montant ou pourcentage ; blocage de la confirmation si acompte non réglé. |
| PRO-35 | Politique d’annulation et pénalités | Définir les règles d’annulation (délai gratuit, pénalité après X h). | Paramètres : délai (ex. 24 h), pénalité (montant ou %) ; application automatique ou manuelle. |
| PRO-36 | Facturation et historique des paiements | Consulter l’historique des paiements liés aux RDV ; export pour comptabilité. | Liste des transactions par période, par client ; export CSV/PDF ; lien avec Miyuinvoice si activé. |
| PRO-37 | Gestion client (fiche, historique) | Consulter et éditer la fiche client ; voir l’historique des RDV par client. | Fiche client (nom, contact, préférences) ; liste des RDV passés et à venir ; notes optionnelles. |
| PRO-38 | CRM léger | Enregistrer des notes, des tags ou des champs personnalisés sur le client. | Champs personnalisables par professionnel ; recherche et filtres par tag ou note. |

### 2.6 Statistiques et analytics

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-39 | Tableau de bord synthétique | Afficher les indicateurs clés : RDV du jour, de la semaine, taux de remplissage, no-show. | Vue dashboard : nombre de RDV (jour, semaine, mois), taux de remplissage, taux de no-show. |
| PRO-40 | Analyses d’activité | Consulter les statistiques par période, par service, par praticien. | Graphiques et tableaux : volume de RDV, répartition par service, par praticien ; évolution dans le temps. |
| PRO-41 | Taux de no-show | Mesurer et afficher le taux de non-présentation (absents sans annulation). | Calcul automatique (RDV marqués no-show / RDV confirmés) ; tendance ; comparaison avant/après rappels. |
| PRO-42 | Export des données | Exporter les RDV et les statistiques (CSV, PDF) pour reporting externe. | Export par période, par praticien, par établissement ; formats CSV, PDF (rapport). |
| PRO-43 | Créneaux les plus demandés | Identifier les créneaux (jour, heure) les plus réservés. | Vue ou rapport : heatmap ou tableau des créneaux les plus pris ; aide à l’optimisation des plannings. |

### 2.7 Intégrations et exposition

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-44 | Lien de réservation personnalisé | Obtenir un lien unique à partager (site web, email, réseaux sociaux) pour la prise de RDV. | Génération du lien ; paramètres optionnels (service pré-sélectionné, praticien) ; page de réservation dédiée. |
| PRO-45 | Widget à intégrer sur site web | Intégrer un bouton ou un formulaire de réservation sur le site du professionnel (iframe, overlay). | Code d’intégration (iframe, script) ; personnalisation (couleurs, service par défaut) ; responsive. |
| PRO-46 | API REST | Accéder aux créneaux, aux réservations et aux annulations via API pour intégration avec un logiciel métier. | Documentation API ; authentification (token, OAuth) ; endpoints créneaux, RDV, annulation. |
| PRO-47 | Webhooks | Recevoir des notifications en temps réel (nouveau RDV, annulation, modification). | Configuration des événements et de l’URL de callback ; payload JSON ; retry et signature. |
| PRO-48 | Synchronisation calendriers (export client) | Proposer au client d’ajouter le RDV à son agenda (Google, Outlook, Apple, iCal). | Lien « Ajouter à mon agenda » dans l’email de confirmation ; génération de fichier iCal. |
| PRO-49 | Plugin CMS (ex. WordPress) | Proposer un plugin pour les sites WordPress (ou autre CMS) pour afficher le widget. | Plugin téléchargeable ou lien vers dépôt ; configuration (lien pro, service, couleurs). |
| PRO-50 | Intégrations tierces (Zapier, Make) | Connecter JayRDV à des outils tiers (CRM, emailing, compta) via Zapier ou équivalent. | Connexion Zapier/Make ; triggers et actions (nouveau RDV, annulation) ; documentation. |

### 2.8 Équipe et rôles

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-51 | Gestion des rôles | Attribuer les rôles (Admin, Gestionnaire, Praticien) aux membres de l’équipe. | Liste des membres ; attribution du rôle ; périmètre (tous les établissements ou un seul) ; invitation par email. |
| PRO-52 | Admin | Accès complet : paramétrage, plannings, RDV, stats, facturation, équipe. | Toutes les capacités ; gestion des autres membres. |
| PRO-53 | Gestionnaire | Gestion des RDV et des plannings sans modifier les paramètres globaux (services, tarifs, notifications). | Calendrier, création/modification/annulation RDV, vue clients ; pas d’accès aux paramètres de compte. |
| PRO-54 | Praticien | Accès à son propre planning et à ses RDV ; modification de ses indisponibilités. | Vue limitée à son planning ; édition de ses exceptions (congés, pauses) ; pas d’accès aux autres praticiens ni aux paramètres. |
| PRO-55 | Délégation par établissement | Un Gestionnaire peut être assigné à un ou plusieurs établissements uniquement. | Assignation Gestionnaire → établissements ; le Gestionnaire ne voit que ses établissements. |

### 2.9 Cours, ateliers et créneaux collectifs

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-56 | Création de cours / ateliers | Définir des créneaux collectifs avec nombre de places limité (ex. cours du mardi 18h, 10 places). | Formulaire : nom, date/heure, durée, nombre de places, prix optionnel ; récurrence possible. |
| PRO-57 | Gestion des inscriptions | Consulter la liste des inscrits ; ouvrir/fermer les inscriptions. | Liste des participants ; statut (inscrit, waitlist, annulé) ; fermeture des inscriptions à J-1 ou paramétrable. |
| PRO-58 | Liste d’attente | Enregistrer les clients en liste d’attente si le créneau est complet ; notification en cas de désistement. | Inscription en liste d’attente ; alerte au professionnel en cas de place libérée ; proposition au premier de la liste. |
| PRO-59 | Récurrence des cours | Définir des séries de cours (ex. tous les mardis pendant 10 semaines). | Paramètre de récurrence (hebdo, bi-hebdo, mensuel) ; génération des occurrences ; édition groupée ou individuelle. |

### 2.10 Multi-établissements

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PRO-60 | Liste des établissements | Consulter et gérer la liste des établissements (sites, adresses). | Liste des établissements ; création, édition, désactivation ; adresse, contact, horaires par établissement. |
| PRO-61 | Paramétrage par établissement | Définir des plannings, des services et des praticiens par établissement. | Chaque établissement peut avoir ses propres créneaux, services, praticiens ; vue consolidée ou par établissement. |
| PRO-62 | Lien de réservation par établissement | Obtenir un lien de réservation par établissement (ou par praticien). | Génération de liens distincts ; pré-sélection de l’établissement ou du praticien. |
| PRO-63 | Statistiques consolidées ou par établissement | Consulter les stats globales ou par établissement. | Filtre par établissement dans le dashboard ; comparaison entre établissements. |

---

## 3. Besoins non fonctionnels

### 3.1 Performance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-PRO-01 | Temps de chargement du tableau de bord | Le tableau de bord pro se charge en moins de 3 secondes (réseau standard). |
| NFR-PRO-02 | Temps de chargement du calendrier | La vue calendrier (jour, semaine) se charge en moins de 2 secondes. |
| NFR-PRO-03 | Disponibilité des créneaux en temps réel | Les créneaux proposés au client sont calculés et mis à jour en temps réel (latence < 2 s après une réservation). |
| NFR-PRO-04 | Export (CSV, PDF) | Les exports (liste RDV, rapport stats) sont générés en moins de 15 secondes pour des volumes raisonnables (< 5000 RDV). |

### 3.2 Disponibilité et fiabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-PRO-05 | Disponibilité | Le service est disponible 99,5 % du temps (hors fenêtres de maintenance annoncées). |
| NFR-PRO-06 | Sauvegarde et récupération | Les données (RDV, plannings, clients) sont sauvegardées et récupérables en cas d’incident ; pas de perte de données validées. |
| NFR-PRO-07 | Prise de RDV hors ligne (résilience) | En cas d’indisponibilité temporaire, les créneaux déjà envoyés au client ne sont pas réservables ailleurs (cohérence après reconnexion). |

### 3.3 Sécurité et gouvernance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-PRO-08 | Authentification | Authentification sécurisée (Miyauth) ; mot de passe ou lien magique ; session avec expiration. |
| NFR-PRO-09 | Permissions | Les actions sont soumises aux permissions (Master Butler) ; un utilisateur ne peut accéder qu’aux établissements et données autorisés par son rôle. |
| NFR-PRO-10 | Isolation des données | Les données d’un professionnel ne sont pas accessibles aux autres professionnels ; isolation stricte par Mandat. |
| NFR-PRO-11 | Audit | Les actions sensibles (création/modification/annulation RDV, modification plannings, paramètres) sont tracées (qui, quand, quoi) pour audit. |
| NFR-PRO-12 | Données personnelles (RGPD) | Conformité RGPD : consentement, droit d’accès, de rectification, d’effacement ; durée de conservation configurable. |

### 3.4 Utilisabilité et accessibilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-PRO-13 | Utilisabilité | Les parcours principaux (création de RDV manuel, modification planning, consultation calendrier) sont réalisables en moins de 5 clics depuis le tableau de bord. |
| NFR-PRO-14 | Accessibilité | Conformité WCAG 2.1 niveau AA pour les écrans de l’espace pro (navigation clavier, lecteurs d’écran, contrastes). |
| NFR-PRO-15 | Responsive | Le tableau de bord et le calendrier sont utilisables sur tablette et mobile ; les actions de configuration complexes restent optimisées desktop. |
| NFR-PRO-16 | Multi-langue (optionnel) | Interface et emails disponibles en français par défaut ; extension possible à d’autres langues (paramètre ou fichier de traduction). |

### 3.5 Maintenabilité et évolutivité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-PRO-17 | Volume | Le système supporte au moins 50 000 RDV par professionnel et 100 établissements par compte sans dégradation majeure. |
| NFR-PRO-18 | Évolutivité | L’ajout de nouveaux types de services, de ressources ou de champs personnalisés ne nécessite pas de refonte de l’architecture. |

---

## 4. Parcours détaillés et scénarios

### 4.1 Scénario : Premier usage — inscription et première configuration

1. Le professionnel arrive sur la plateforme (depuis un lien ou une recherche).
2. Il clique sur « Créer un compte professionnel ».
3. Il remplit le formulaire (email, mot de passe, nom, type d’activité, établissement).
4. Il reçoit un email de validation (si configuré) et valide son compte.
5. Après validation (automatique ou manuelle), il accède au tableau de bord pro (vide).
6. Il définit ses services (ex. Consultation 30 min, Séance 1 h).
7. Il définit ses plannings récurrents (lun–ven 9h–18h).
8. Il génère son lien de réservation et le partage sur son site ou par email.
9. Les premiers RDV arrivent ; il reçoit les notifications et consulte le calendrier.

**Besoins couverts** : PRO-01 à PRO-04, PRO-07, PRO-08, PRO-15, PRO-44, PRO-27, PRO-29, PRO-24.

### 4.2 Scénario : Gestion des no-show et rappels

1. Le professionnel active les rappels automatiques (24 h et 2 h avant).
2. Il personnalise le message de rappel (logo, texte).
3. Les clients reçoivent les rappels ; le taux de no-show baisse (mesuré dans les stats).
4. Il consulte le tableau de bord : taux de no-show avant/après rappels.
5. Il décide d’exiger un acompte pour les nouveaux clients afin de réduire encore les absences.

**Besoins couverts** : PRO-27, PRO-28, PRO-31, PRO-39, PRO-41, PRO-34.

### 4.3 Scénario : Multi-praticiens et round-robin

1. Le cabinet a 3 praticiens ; chacun a son planning.
2. Le professionnel configure l’option « Premier disponible » (round-robin) pour le service « Consultation ».
3. Le client qui réserve « Consultation » ne choisit pas le praticien ; le système attribue le premier créneau disponible parmi les 3.
4. Le gestionnaire consulte la vue calendrier : tous les RDV des 3 praticiens sont visibles, avec couleurs par praticien.
5. Un praticien annule un RDV ; le créneau redevient disponible pour les 3.

**Besoins couverts** : PRO-21, PRO-22, PRO-24, PRO-26.

### 4.4 Scénario : Intégration widget et API

1. Le professionnel souhaite intégrer la réservation sur son site WordPress.
2. Il récupère le code du widget (iframe ou script) dans son espace JayRDV.
3. Il colle le code sur sa page « Prendre rendez-vous » ; le widget s’affiche avec ses services et ses créneaux.
4. Son logiciel de gestion de cabinet appelle l’API JayRDV pour récupérer les RDV du jour et les afficher dans son interface.
5. Il configure un webhook « Nouveau RDV » vers son CRM ; à chaque réservation, le CRM reçoit les données du client et du créneau.

**Besoins couverts** : PRO-44, PRO-45, PRO-46, PRO-47, PRO-49, PRO-50.

### 4.5 Scénario : Cours et liste d’attente

1. Le professionnel crée un cours « Yoga mardi 18h » (10 places, récurrence hebdo).
2. Les clients s’inscrivent en ligne ; après 10 inscriptions, le créneau affiche « Complet ».
3. Un 11e client s’inscrit en liste d’attente.
4. Un inscrit annule ; le premier de la liste d’attente reçoit une notification et peut prendre la place.
5. Le professionnel consulte la liste des participants et la liste d’attente depuis son tableau de bord.

**Besoins couverts** : PRO-56, PRO-57, PRO-58, PRO-59.

---

## 5. Pain points et opportunités

### 5.1 Pain points

| Pain point | Impact | Besoin associé |
|------------|--------|-----------------|
| Double saisie agenda | Le pro saisit dans son agenda perso et dans l’outil de réservation ; risque d’oubli et de conflit. | PRO-18 (synchronisation agendas). |
| No-show élevé | Absences sans annulation ; perte de temps et de chiffre d’affaires. | PRO-27, PRO-28, PRO-41 (rappels, mesure). |
| Créneaux pris alors qu’absent | Pas de blocage des congés ; des clients réservent des créneaux où le pro est absent. | PRO-16, PRO-17 (exceptions, jours fériés). |
| Manque de visibilité multi-praticiens | Pas de vue consolidée ; conflits de créneaux entre collègues. | PRO-24, PRO-21 (calendrier centralisé, multi-praticiens). |
| Outils dispersés | Réservation sur un outil, facturation sur un autre, pas d’API. | PRO-46, PRO-47, PRO-33 à PRO-36 (API, webhooks, paiement). |
| Widget peu personnalisable | Le widget ne s’intègre pas au design du site. | PRO-45 (widget, personnalisation). |

### 5.2 Opportunités

| Opportunité | Description | Besoin associé |
|-------------|-------------|-----------------|
| Réduction mesurée des no-show | Rappels systématiques ; objectif division par 5 (référence marché). | PRO-28, PRO-41. |
| Réservation 24h/24 sans intervention | Les clients réservent sans appeler ; gain de temps pour le pro et la réception. | PRO-44, PRO-45, PRO-20. |
| Vue consolidée multi-établissements | Un seul tableau de bord pour tous les sites ; comparaison des performances. | PRO-60 à PRO-63. |
| Automatisation (webhooks, Zapier) | Connexion avec CRM, emailing, compta ; moins de saisie manuelle. | PRO-47, PRO-50. |
| Différenciation (cours, ressources, round-robin) | Proposer des créneaux collectifs et des ressources ; positionnement pro. | PRO-56 à PRO-59, PRO-23. |

---

## 6. Priorisation des besoins (MoSCoW)

### 6.1 Must have (indispensable)

- PRO-01 à PRO-06 (onboarding, compte, profil).
- PRO-07 à PRO-14 (services, créneaux types, buffer, préavis).
- PRO-15 à PRO-21 (plannings, exceptions, synchro agendas, pas de double réservation, multi-praticiens).
- PRO-24 à PRO-26 (vue calendrier, création/modification/annulation RDV).
- PRO-27, PRO-28 (confirmation et rappels automatiques).
- PRO-44 (lien de réservation).
- PRO-51 à PRO-54 (rôles Admin, Gestionnaire, Praticien).
- NFR-PRO-08 à NFR-PRO-11 (sécurité, permissions, isolation, audit).

### 6.2 Should have (important)

- PRO-22 (round-robin), PRO-23 (ressources).
- PRO-29 à PRO-32 (notifications pro, personnalisation messages, alerte désistement, suivi post-RDV).
- PRO-33 à PRO-38 (paiement, acompte, politique annulation, fiche client, CRM léger).
- PRO-39 à PRO-43 (stats, taux no-show, export).
- PRO-45, PRO-46, PRO-47 (widget, API, webhooks).
- PRO-55 (délégation par établissement).
- PRO-60 à PRO-63 (multi-établissements).
- NFR-PRO-01 à NFR-PRO-07, NFR-PRO-12 à NFR-PRO-16 (performance, dispo, RGPD, utilisabilité, accessibilité).

### 6.3 Could have (souhaitable)

- PRO-48, PRO-49, PRO-50 (ajout à l’agenda client, plugin CMS, Zapier).
- PRO-56 à PRO-59 (cours, ateliers, liste d’attente).
- NFR-PRO-17, NFR-PRO-18 (volume, évolutivité).

### 6.4 Won’t have (hors périmètre ou report)

- Besoins spécifiques aux autres publics (clients, utilisateur non connecté).
- Fonctionnalités avancées (IA prédictive, billetterie événementiel) — hors périmètre v1.

---

## 7. Dépendances et interfaces avec les autres publics

### 7.1 Dépendances

| Dépendance | Description |
|------------|-------------|
| **Clients** | Les professionnels dépendent des réservations effectuées par les clients ; les clients dépendent des créneaux et des liens exposés par les professionnels. |
| **Utilisateur non connecté** | Les professionnels exposent des liens et des widgets ; les utilisateurs non connectés réservent via la Façade publique (parcours guest). |
| **Plateforme** | Mandat de Permission (StrongFather), permissions (Master Butler), persistance (KindMother), sécurité (WorrySentinel), notifications (Miyunotify), paiement (Miyuinvoice si activé). |

### 7.2 Interfaces

| Interface | Flux | Besoin pro |
|-----------|------|------------|
| Pro → Client | Lien de réservation, confirmation, rappel, annulation. | PRO-44, PRO-27, PRO-28, PRO-26. |
| Pro → Utilisateur non connecté | Page de réservation publique, widget ; confirmation par email/SMS. | PRO-44, PRO-45, PRO-20. |
| Client → Pro | Prise de RDV, annulation ; données client (nom, contact). | Côté client ; pro reçoit et consulte. |

---

## 8. User stories (format standard)

### 8.1 Onboarding et compte

- **US-PRO-01** — En tant que **professionnel**, je veux **créer un compte** (email, mot de passe, informations structure) **afin de** accéder à l’espace pro et configurer mes créneaux. *Critères* : Formulaire dédié ; validation email si configurée ; création profil (Miyauth, Miyuprofile).
- **US-PRO-02** — En tant que **professionnel**, je veux **définir mes services** (nom, durée, tarif optionnel) **afin que** les clients puissent choisir le type de RDV. *Critères* : Formulaire de création ; liste des services ; association à un ou plusieurs praticiens.
- **US-PRO-03** — En tant que **professionnel**, je veux **synchroniser mon agenda** (Google, Outlook, Apple) **afin de** ne pas avoir de double saisie et éviter les créneaux pris alors que je suis absent. *Critères* : Connexion OAuth ; synchro bidirectionnelle ; événements externes bloquent les créneaux.

### 8.2 Plannings et RDV

- **US-PRO-04** — En tant que **professionnel**, je veux **définir mes horaires récurrents** et **bloquer mes congés** **afin que** seuls les créneaux disponibles soient proposés. *Critères* : Plages par jour de la semaine ; exceptions (date, plage) ; jours fériés.
- **US-PRO-05** — En tant que **professionnel**, je veux **voir tous mes RDV** dans un calendrier (jour, semaine, mois) **afin de** gérer mon activité. *Critères* : Vue calendrier ; filtres praticien, ressource, établissement ; création/modification/annulation depuis la vue.
- **US-PRO-06** — En tant que **gestionnaire**, je veux **créer un RDV manuellement** pour un client **afin de** enregistrer une prise de RDV par téléphone ou en présentiel. *Critères* : Formulaire client, créneau, service ; vérification disponibilité ; notification client optionnelle.

### 8.3 Notifications et rappels

- **US-PRO-07** — En tant que **professionnel**, je veux **que mes clients reçoivent une confirmation** et **un rappel** avant le RDV **afin de** réduire les no-show. *Critères* : Confirmation à la réservation ; rappel configurable (délai, canal) ; personnalisation des messages.
- **US-PRO-08** — En tant que **professionnel**, je veux **être notifié à chaque nouvelle réservation** **afin de** être informé en temps réel. *Critères* : Notification email, push ou in-app ; contenu : client, créneau, service.

### 8.4 Paiement et clients

- **US-PRO-09** — En tant que **professionnel**, je veux **proposer le paiement en ligne** (intégral ou acompte) à la réservation **afin de** sécuriser mes revenus et réduire les no-show. *Critères* : Intégration passerelle ; paramètre par service ; sécurisation PCI.
- **US-PRO-10** — En tant que **professionnel**, je veux **consulter la fiche et l’historique des RDV** d’un client **afin de** personnaliser l’accueil et le suivi. *Critères* : Fiche client ; liste des RDV passés et à venir ; notes optionnelles.

### 8.5 Intégrations

- **US-PRO-11** — En tant que **professionnel**, je veux **obtenir un lien de réservation** et **un widget** à mettre sur mon site **afin que** mes clients réservent en ligne 24h/24. *Critères* : Lien unique ; code widget (iframe/script) ; personnalisation couleurs, service.
- **US-PRO-12** — En tant que **professionnel**, je veux **recevoir des webhooks** (nouveau RDV, annulation) **afin de** connecter JayRDV à mon CRM ou mon logiciel métier. *Critères* : Configuration URL et événements ; payload JSON ; retry et signature.

### 8.6 Équipe et multi-établissements

- **US-PRO-13** — En tant qu’**Admin**, je veux **inviter des membres** (Gestionnaire, Praticien) et **attribuer des rôles** **afin de** déléguer la gestion des RDV. *Critères* : Invitation par email ; rôle et périmètre (établissement) ; accès selon rôle.
- **US-PRO-14** — En tant que **professionnel multi-établissements**, je veux **gérer plusieurs sites** avec des plannings distincts **afin de** piloter toute mon activité depuis un seul compte. *Critères* : Liste des établissements ; paramétrage par établissement ; stats consolidées ou par site.

---

## 9. Cas limites et règles métier

### 9.1 Règles métier

| Règle | Description |
|-------|-------------|
| **Mandat** | Un professionnel ne peut configurer des services, des plannings ou des RDV que dans le cadre de son Mandat de Permission (StrongFather, Master Butler). |
| **Isolation** | Les données d’un professionnel (RDV, clients, plannings) ne sont accessibles qu’à lui et à son équipe (rôles assignés) ; pas d’accès aux données des autres professionnels. |
| **Rôle Praticien** | Un Praticien ne voit que son propre planning et ses RDV ; il peut modifier ses indisponibilités mais pas les paramètres globaux. |
| **Créneau unique** | Un même créneau (praticien ou ressource) ne peut être réservé qu’une seule fois ; vérification en temps réel à la prise de RDV. |
| **Préavis** | Les créneaux proposés respectent le préavis minimum (réservation à l’avance) et le préavis maximum (pas au-delà de X mois). |

### 9.2 Cas limites

| Cas | Comportement attendu |
|-----|----------------------|
| **Réservation simultanée** | Deux clients choisissent le même créneau au même moment ; un seul obtient la réservation ; l’autre reçoit un message « Créneau non disponible » et peut en choisir un autre. |
| **Modification de planning avec RDV existants** | Si le pro supprime une plage ou une exception qui chevauche un RDV existant, le système alerte et propose de déplacer ou annuler le RDV. |
| **Annulation par le client après délai gratuit** | Application de la politique d’annulation (pénalité) ; enregistrement du motif ; option de remboursement partiel ou total selon paramètre. |
| **Synchronisation agenda externe en retard** | Les créneaux déjà réservés par un client restent bloqués même si la synchro externe n’a pas encore mis à jour ; priorité aux RDV confirmés. |
| **Praticien supprimé avec RDV à venir** | Blocage ou confirmation forte ; proposition de réaffectation des RDV à un autre praticien ou d’annulation avec notification client. |

### 9.3 Métriques de succès

| Métrique | Description | Cible (exemple) |
|----------|-------------|------------------|
| **Taux d’activation** | % de professionnels ayant configuré au moins un service et un planning après inscription. | > 85 % |
| **Temps jusqu’à première réservation** | Délai entre création de compte et premier RDV pris par un client. | < 7 jours |
| **Taux de no-show (avant/après rappels)** | Réduction du taux de no-show après activation des rappels. | Division par 5 (référence marché) |
| **Utilisation du widget / lien** | % de RDV pris via le lien ou le widget (vs création manuelle). | > 70 % |
| **Satisfaction professionnel** | Score NPS ou enquête satisfaction (facilité, gain de temps). | Suivi annuel |

---

## 10. Glossaire et références

### 10.1 Glossaire (extrait)

| Terme | Définition |
|-------|------------|
| **Créneau** | Plage horaire disponible pour un RDV (début, fin), associée à un praticien et/ou une ressource. |
| **Buffer time** | Temps minimum entre deux RDV consécutifs pour un même praticien ou ressource. |
| **No-show** | Absence du client au RDV sans annulation préalable. |
| **Round-robin** | Répartition automatique des réservations entre plusieurs praticiens (premier disponible ou règle d’équilibrage). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée (StrongFather) permettant au professionnel d’agir dans le périmètre autorisé. |

### 10.2 Annexes — Matrice besoins / fonctionnalités marché

Le tableau ci-dessous positionne les besoins Professionnels par rapport aux fonctionnalités identifiées dans le benchmark « Fonctionnalités des solutions de réservation en ligne » (document de référence JayRDV).

| Domaine benchmark | Besoins JayRDV (Professionnels) | Priorité |
|-------------------|-----------------------------------|----------|
| Gestion calendaire et planification | PRO-15 à PRO-25 (plannings, exceptions, synchro, calendrier centralisé, création/modification RDV) | Must / Should |
| Prise de RDV client (côté exposition) | PRO-44, PRO-45, PRO-20 (lien, widget, affichage des seules disponibilités) | Must |
| Notifications et rappels | PRO-27 à PRO-32 (confirmation, rappels, notifications pro, personnalisation, alerte désistement) | Must / Should |
| Paiements et gestion commerciale | PRO-33 à PRO-38 (paiement en ligne, acompte, politique annulation, fiche client, CRM léger) | Should |
| Intégrations et synchronisation | PRO-18, PRO-46 à PRO-50 (synchro agendas, API, webhooks, widget, plugin, Zapier) | Should / Could |
| Statistiques et analytics | PRO-39 à PRO-43 (dashboard, analyses, taux no-show, export) | Should |
| Gestion administrative | PRO-24 à PRO-26, PRO-51 à PRO-55 (calendrier centralisé, annulations, rôles, délégation) | Must |
| Cours / ateliers / liste d’attente | PRO-56 à PRO-59 | Could |
| Multi-établissements | PRO-60 à PRO-63 | Should |

### 10.3 Critères d’acceptation synthétiques (MVP)

Pour la première version livrable (MVP) du public Professionnels, les critères d’acceptation globaux suivants sont retenus :

- **Inscription et paramétrage** : Le professionnel peut créer un compte, valider son email (si activé), définir au moins un service (nom, durée) et un planning récurrent (horaires par jour de la semaine). Il peut bloquer des exceptions (congés, absences) et des jours fériés.
- **Exposition** : Le professionnel obtient un lien de réservation unique qu’il peut partager ; les clients voient uniquement les créneaux disponibles (pas l’agenda détaillé). Option : widget intégrable (iframe ou script) sur son site.
- **Calendrier** : Le professionnel consulte une vue calendrier (jour, semaine, mois) avec tous ses RDV ; il peut créer, modifier et annuler un RDV manuellement. Aucun double réservation possible (vérification en temps réel).
- **Notifications** : À chaque réservation, le client reçoit une confirmation (email et/ou SMS) ; un rappel est envoyé automatiquement (paramétrable : 24 h et/ou 2 h avant). Le professionnel peut être notifié à chaque nouveau RDV.
- **Sécurité et gouvernance** : Authentification (Miyauth), permissions (Master Butler), isolation des données par professionnel, traçabilité des actions sensibles (audit).
- **Rôles** : Au moins deux rôles — Admin (accès complet) et Praticien (accès à son propre planning et ses RDV). Option : Gestionnaire (gestion des RDV sans paramétrage global).

Ces critères couvrent les besoins PRO-01 à PRO-06, PRO-07 à PRO-08, PRO-15 à PRO-21, PRO-24 à PRO-29, PRO-44, PRO-51 à PRO-54 et les NFR-PRO-08 à NFR-PRO-11.

### 10.4 Références documentaires

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md)
- [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md)
- [Professionnels — Parcours, capacités et livrables](./Professionnels%20-%20Parcours%20Capacites%20Livrables.md)
- [Public Clients](../Clients/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)

---

**Document** : Professionnels — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit — référence pour le public Professionnels
