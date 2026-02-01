# Utilisateur non connecté — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du public cible **Utilisateur non connecté** pour le service Miyukini Festival Service. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels liés à la **Façade publique gouvernée** (catalogue en lecture seule), les parcours de découverte, les user stories, les pain points et opportunités, ainsi que la priorisation et les dépendances. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md), [Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md).

## Portée / Scope

- **Public** : Toute personne accédant au catalogue **sans compte** ni authentification (curieux, futurs organisateurs, exposants ou visiteurs, presse, partenaires).
- **Périmètre** : tous les besoins liés à la Façade publique (annuaire des événements, répertoire des organisateurs, répertoire des exposants), parcours de découverte, recherche, filtres, passerelles vers inscription/connexion.
- **Hors périmètre** : spécifications techniques d’implémentation (API, schémas de données détaillés), spécifications des espaces dédiés (organisateur, exposant, visiteur) — traitées dans leurs propres documents d’analyse.

---

## 1. Profil du public et personas

### 1.1 Définition du public

L’**utilisateur non connecté** est toute personne qui accède au **catalogue** (annuaire des événements, répertoire des organisateurs, répertoire des exposants) **sans compte** ni authentification. Il bénéficie d’une **Façade publique gouvernée** : consultation en **lecture seule**, sans accès aux espaces dédiés (organisateur, exposant, visiteur). L’objectif est la **découverte** et l’**information** ; la création de compte ou la connexion permet ensuite d’accéder aux espaces selon le type de public (organisateur, exposant, visiteur).

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Curieux / découverte** | Navigue sans objectif précis ; cherche des idées de sorties, festivals, exposants. | Parcourir les événements et les exposants ; comparer les offres ; s’informer. | Infos dispersées sur plusieurs sites ; pas de vue d’ensemble ; difficulté à comparer. |
| **Futur organisateur** | Structure (asso, collectivité) qui envisage d’organiser un événement. | Découvrir la plateforme, les organisateurs existants, les types d’événements ; évaluer la crédibilité avant de s’inscrire. | Manque de transparence sur les organisateurs ; pas de preuve sociale ; méfiance avant inscription. |
| **Futur exposant** | Entreprise ou artisan qui souhaite participer à des festivals. | Trouver les événements pertinents, les organisateurs, les conditions ; décider de déposer une candidature. | Pas de vue centralisée ; difficulté à identifier les événements adaptés ; processus d’inscription opaque. |
| **Futur visiteur** | Personne qui envisage d’assister à un ou plusieurs événements. | Consulter le programme, les exposants, les services proposés ; décider de réserver ou d’acheter un billet. | Infos incomplètes sur la fiche événement ; pas de visibilité sur les ateliers ou pass ; hésitation à créer un compte. |
| **Presse / partenaire** | Journaliste, partenaire média ou institutionnel. | Accéder aux informations publiques (dates, lieu, organisateur, programme) pour relayer ou collaborer. | Besoin d’informations fiables et à jour ; pas d’accès aux données privées sans compte. |

### 1.3 Contexte d’usage

- **Fréquence** : visite ponctuelle (recherche ciblée) ou exploration régulière (veille, comparaison).
- **Appareils** : mobile et desktop ; le catalogue doit être accessible et lisible sur tous les écrans.
- **Concurrence** : sites par festival, annuaires externes, réseaux sociaux ; attente d’un **guichet unique** (Store) pour découvrir événements, organisateurs et exposants.

### 1.4 Intentions types (sans compte)

| Intention | Comportement attendu | Besoins principaux |
|-----------|----------------------|---------------------|
| **Découvrir** | Parcourir les événements et exposants sans engagement. | UNC-01, UNC-04, UNC-07, UNC-13, UNC-15. |
| **Comparer** | Consulter plusieurs fiches (événements, organisateurs) pour comparer. | UNC-05, UNC-07, UNC-11, UNC-12, UNC-18. |
| **Trouver** | Recherche ciblée (mot-clé, filtre) pour un événement ou un exposant précis. | UNC-16, UNC-17, UNC-05, UNC-14. |
| **S’informer** | Lire le programme, les conditions, les coordonnées avant de décider. | UNC-07, UNC-08, UNC-12, UNC-15, UNC-25. |
| **Passer à l’action** | Décider de s’inscrire ou se connecter pour réserver, candidater ou gérer. | UNC-19 à UNC-22. |

---

## 2. Besoins fonctionnels

### 2.1 Accès à la Façade publique

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-01 | Accès au site ou à l’application sans compte | Pouvoir accéder à la landing ou à l’accueil du service sans créer de compte ni se connecter. | Page d’accueil ou landing affichée ; accès au catalogue (annuaire événements, répertoire organisateurs, répertoire exposants) ; pas de blocage ni de redirection forcée vers connexion. |
| UNC-02 | Visibilité des appels à l’action (inscription, connexion) | Voir clairement les options « S’inscrire » et « Se connecter » pour accéder aux espaces dédiés. | Liens ou boutons visibles (header, footer ou zone dédiée) ; distinction entre inscription (organisateur, exposant, visiteur) et connexion ; pas d’obligation de cliquer pour consulter le catalogue. |
| UNC-03 | Lecture seule stricte | Consulter le catalogue sans pouvoir créer, modifier ou supprimer de données. | Aucune action d’écriture (création d’événement, candidature, réservation, achat) sans compte ; messages explicites si l’utilisateur tente une action réservée aux comptes (ex. « Connectez-vous pour réserver »). |

### 2.2 Annuaire des événements

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-04 | Liste des événements publiés | Consulter la liste des événements (éditions) publiés par les organisateurs. | Liste affichée avec les événements dont le statut est « Publié » ; pas d’affichage des événements brouillon ou non publiés ; pagination ou scroll infini si volume important. |
| UNC-05 | Filtres sur l’annuaire événements | Filtrer les événements par date, lieu, organisateur, thème ou mot-clé. | Filtres disponibles (date de début/fin, lieu ou région, organisateur, thème/catégorie, recherche texte) ; résultats mis à jour selon les filtres ; réinitialisation des filtres possible. |
| UNC-06 | Vue carte ou liste | Consulter les événements en vue liste ou vue carte (géolocalisation) selon préférence. | Bascule vue liste / vue carte si applicable ; carte avec marqueurs ou zones ; liste avec vignette, titre, dates, lieu, organisateur. |
| UNC-07 | Fiche événement (détail public) | Accéder à la fiche détail d’un événement : présentation, dates, lieu, organisateur, exposants, programme public. | Fiche contenant : nom, description, dates, lieu, organisateur (lien vers fiche organisateur), liste des exposants (lien vers fiches), programme public (animations, horaires), services proposés (ateliers, concours, pass si activés) ; pas de données privées (budget, candidatures, documents internes). |
| UNC-08 | Programme public par événement | Consulter le programme public de l’événement (animations, créneaux, salles) sans compte. | Affichage du programme publié par l’organisateur (vue chronologique ou par salle) ; filtres jour, type d’animation ; pas d’accès aux créneaux réservables sans compte (lien vers inscription/connexion pour réserver). |
| UNC-09 | Exposants de l’événement | Voir la liste des exposants de l’événement avec accès aux fiches exposant. | Liste des exposants validés et publiés pour l’événement ; lien vers la fiche exposant (répertoire) ; pas d’affichage des exposants refusés ou non validés. |

### 2.3 Répertoire des organisateurs

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-10 | Liste des organisateurs | Consulter la liste des structures organisatrices présentes sur la plateforme. | Liste des organisateurs dont au moins un événement est publié (ou selon politique plateforme) ; pas d’affichage des organisateurs non validés ou sans événement publié. |
| UNC-11 | Filtres sur le répertoire organisateurs | Filtrer les organisateurs par nom, région, type d’événement ou nombre d’événements. | Filtres disponibles (recherche texte, région, type d’événement, année) ; résultats cohérents ; réinitialisation possible. |
| UNC-12 | Fiche organisateur (détail public) | Accéder à la fiche détail d’un organisateur : nom, événements, contact, charte. | Fiche contenant : nom de la structure, description, liste des événements publiés (liens vers fiches événement), coordonnées de contact (email, site web, selon paramétrage organisateur), charte ou valeurs si publiées ; pas d’accès aux données internes (équipe, budget, candidatures). |

### 2.4 Répertoire des exposants

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-13 | Liste des exposants | Consulter la liste des exposants (globale ou par événement). | Liste des exposants ayant au moins une participation validée et publiée ; vue globale (tous événements) ou vue par événement (filtrer par événement) ; pagination ou scroll si besoin. |
| UNC-14 | Filtres sur le répertoire exposants | Filtrer les exposants par nom, catégorie, événement, région. | Filtres disponibles (recherche texte, catégorie/secteur, événement, région ou lieu) ; résultats cohérents. |
| UNC-15 | Fiche exposant (détail public) | Accéder à la fiche détail d’un exposant : entreprise, stands, éditions participées, contact. | Fiche contenant : nom de l’entreprise ou de l’exposant, description, secteur/catégorie, liste des éditions participées (événements, stands attribués si publiés), coordonnées de contact (selon paramétrage) ; pas d’accès aux documents privés, factures, candidatures en cours. |

### 2.5 Recherche et découverte

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-16 | Recherche globale | Effectuer une recherche textuelle sur les trois piliers (événements, organisateurs, exposants). | Champ de recherche unique ou par pilier ; recherche (Miyusearch) sur les champs publics (titres, descriptions, noms) ; résultats regroupés par type (événements, organisateurs, exposants) ou liste mixte ; pas de résultat sur les données non publiées. |
| UNC-17 | Suggestions et affinage | Obtenir des suggestions (autocomplétion) et affiner les résultats. | Autocomplétion sur la recherche si configurée ; affinage par filtres après recherche ; tri des résultats (pertinence, date, nom). |
| UNC-18 | Liens croisés entre piliers | Naviguer facilement entre événement ↔ organisateur ↔ exposants. | Depuis une fiche événement : lien vers l’organisateur et vers chaque exposant ; depuis une fiche organisateur : liens vers ses événements ; depuis une fiche exposant : liens vers les événements auxquels il participe ; cohérence des données. |

### 2.6 Passerelles vers inscription et connexion

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-19 | CTA « S’inscrire » selon type de public | Pouvoir choisir de s’inscrire en tant qu’organisateur, exposant ou visiteur depuis la Façade publique. | Liens ou boutons « S’inscrire en tant qu’organisateur », « S’inscrire en tant qu’exposant », « S’inscrire en tant que visiteur » ; redirection vers le formulaire d’inscription correspondant ; après inscription, accès à l’espace dédié. |
| UNC-20 | CTA « Se connecter » | Pouvoir se connecter si l’utilisateur a déjà un compte. | Lien ou bouton « Se connecter » ; redirection vers la page de connexion ; après authentification, redirection vers l’espace correspondant au type de compte (organisateur, exposant, visiteur). |
| UNC-21 | CTA contextuels depuis le catalogue | Voir des appels à l’action contextuels (ex. « Réserver cet atelier », « Déposer une candidature pour cet événement ») avec redirection vers connexion/inscription. | Sur la fiche événement : « Réserver un atelier » / « Acheter un pass » → redirection vers inscription visiteur ou connexion ; sur la fiche événement : « Déposer une candidature exposant » → redirection vers inscription exposant ou connexion ; message clair « Connectez-vous ou créez un compte pour continuer ». |
| UNC-22 | Retour au contexte après connexion | Après connexion ou inscription, pouvoir revenir au contexte de consultation (fiche événement, page recherche). | Redirection post-connexion/inscription vers la page d’origine ou vers l’action demandée (ex. fiche événement pour réserver) ; pas de perte de contexte. |

### 2.7 Contenu et gouvernance de la Façade

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| UNC-23 | Données publiées uniquement | Seules les données **publiées** et **autorisées pour le public** sont visibles. | Événements en statut « Publié » ; organisateurs validés et avec au moins un événement publié (ou selon règle plateforme) ; exposants validés et dont la participation est publiée ; pas d’affichage des brouillons, refusés ou données internes. |
| UNC-24 | Cohérence catalogue / espaces dédiés | Les données affichées sur la Façade sont cohérentes avec celles des espaces dédiés (organisateur, exposant). | Même source de vérité (KindMother) ; mise à jour de la Façade lorsque l’organisateur publie ou met à jour un événement ; pas de décalage durable. |
| UNC-25 | Informations légales et accessibilité | Accéder aux informations légales (mentions légales, CGU, politique de confidentialité) et aux informations d’accessibilité. | Liens footer ou page dédiée : mentions légales, CGU, politique de confidentialité, accessibilité ; contenu à jour. |

---

## 3. Besoins non fonctionnels

### 3.1 Performance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-01 | Temps de chargement de la page d’accueil | La page d’accueil (landing ou catalogue) se charge en moins de 3 secondes (réseau standard). |
| NFR-UNC-02 | Temps de chargement des listes (événements, organisateurs, exposants) | Les listes (premier écran) se chargent en moins de 2 secondes. |
| NFR-UNC-03 | Temps de réponse de la recherche | Les résultats de recherche s’affichent en moins de 2 secondes après validation de la requête. |
| NFR-UNC-04 | Temps de chargement des fiches détail | Une fiche événement, organisateur ou exposant se charge en moins de 2 secondes. |

### 3.2 Disponibilité et fiabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-05 | Disponibilité de la Façade publique | La Façade publique (catalogue) est disponible 99,5 % du temps (hors fenêtres de maintenance annoncées). |
| NFR-UNC-06 | Robustesse face au trafic | La Façade supporte les pics de trafic (lancement d’un événement, campagne) sans dégradation majeure (délai < 5 s pour les pages critiques). |

### 3.3 Sécurité et gouvernance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-07 | Façade publique gouvernée | L’accès public est soumis à la gouvernance plateforme (WorrySentinel, Master Butler) ; seules les données autorisées pour le public sont exposées. |
| NFR-UNC-08 | Pas d’accès aux espaces protégés | Sans authentification, aucun accès aux URLs ou données des espaces organisateur, exposant, visiteur. |
| NFR-UNC-09 | Protection contre les abus | Limitation des requêtes (rate limiting) pour éviter le scraping ou les abus ; pas de blocage des usages légitimes. |

### 3.4 Utilisabilité et accessibilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-10 | Utilisabilité du catalogue | Les parcours principaux (consulter la liste des événements, ouvrir une fiche, utiliser les filtres, lancer une recherche) sont réalisables en moins de 5 clics depuis l’accueil. |
| NFR-UNC-11 | Accessibilité | Conformité WCAG 2.1 niveau AA pour la Façade publique (navigation clavier, lecteurs d’écran, contrastes, textes alternatifs). |
| NFR-UNC-12 | Responsive et mobile-first | Le catalogue est utilisable sur mobile, tablette et desktop ; listes et fiches lisibles et interactives sur petit écran. |
| NFR-UNC-13 | Clarté des CTAs | Les boutons et liens « S’inscrire » et « Se connecter » sont visibles et compréhensibles sans ambiguïté. |

### 3.5 Données et confidentialité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-UNC-14 | Données utilisateur non connecté | Aucune donnée personnelle de l’utilisateur non connecté n’est stockée pour le catalogue (hors cookies techniques ou analytics selon politique plateforme). |
| NFR-UNC-15 | RGPD et consentement | Si des cookies ou traceurs sont utilisés sur la Façade, consentement et information conformes au RGPD (bandeau, politique de confidentialité). |

---

## 4. Parcours détaillés et scénarios

### 4.1 Scénario : Découverte libre — exploration du catalogue

1. L’utilisateur arrive sur le site (landing ou accueil) sans compte.
2. Il voit la Façade publique : accès à l’annuaire des événements, au répertoire des organisateurs, au répertoire des exposants, et les CTAs « S’inscrire » / « Se connecter ».
3. Il clique sur « Événements » et consulte la liste des événements publiés.
4. Il applique des filtres (date : prochains 3 mois, lieu : « Paris ») ; la liste se met à jour.
5. Il ouvre la fiche d’un événement : présentation, dates, lieu, organisateur, liste des exposants, programme public.
6. Il clique sur le nom de l’organisateur et accède à la fiche organisateur (événements, contact).
7. Il revient à la fiche événement et clique sur un exposant ; il consulte la fiche exposant (entreprise, éditions participées).
8. Il n’a pas créé de compte ; il quitte le site après exploration.

**Besoins couverts** : UNC-01, UNC-02, UNC-04, UNC-05, UNC-07, UNC-09, UNC-10, UNC-12, UNC-13, UNC-15, UNC-18, UNC-23, NFR-UNC-10.

### 4.2 Scénario : Recherche ciblée — trouver un événement par mot-clé

1. L’utilisateur non connecté arrive sur la Façade publique.
2. Il saisit « festival artisanat » dans le champ de recherche globale.
3. Les résultats s’affichent : événements, organisateurs, exposants contenant le terme (ou pertinents).
4. Il filtre par « Événements » uniquement ; il obtient la liste des événements correspondants.
5. Il ouvre deux fiches événement pour comparer (dates, lieu, programme).
6. Il décide de s’inscrire en tant que visiteur pour réserver un atelier sur le premier événement.
7. Il clique sur « Réserver un atelier » sur la fiche événement ; un message s’affiche : « Connectez-vous ou créez un compte visiteur pour réserver. »
8. Il clique sur « S’inscrire en tant que visiteur » et est redirigé vers le formulaire d’inscription ; après inscription, il est redirigé vers la fiche événement (ou l’espace visiteur) pour poursuivre la réservation.

**Besoins couverts** : UNC-16, UNC-17, UNC-19, UNC-21, UNC-22, NFR-UNC-03.

### 4.3 Scénario : Futur exposant — évaluer la plateforme avant candidature

1. Un artisan (futur exposant) découvre la plateforme via une recherche web.
2. Il accède à l’annuaire des événements et filtre par « artisanat » et « région Île-de-France ».
3. Il consulte les fiches de 3 événements : programme, liste des exposants, coordonnées des organisateurs.
4. Il ouvre le répertoire des exposants et filtre par « céramique » pour voir quels exposants sont déjà présents.
5. Il consulte la fiche d’un organisateur pour voir sa charte et ses événements passés.
6. Il décide de déposer une candidature pour l’un des événements ; il clique sur « Déposer une candidature » sur la fiche événement.
7. Il est invité à se connecter ou à s’inscrire en tant qu’exposant ; il crée un compte exposant et est redirigé vers son dashboard pour déposer la candidature.

**Besoins couverts** : UNC-04, UNC-05, UNC-07, UNC-10, UNC-12, UNC-13, UNC-14, UNC-19, UNC-21, UNC-22.

### 4.4 Scénario : Tentative d’action réservée aux comptes

1. L’utilisateur non connecté consulte la fiche d’un événement qui propose des ateliers et des pass.
2. Il clique sur « Réserver un créneau » (atelier).
3. La plateforme affiche un message : « La réservation est réservée aux utilisateurs connectés. Connectez-vous ou créez un compte visiteur pour réserver. » avec les boutons « Se connecter » et « S’inscrire ».
4. Aucune donnée n’a été saisie ni enregistrée ; l’utilisateur peut continuer à consulter la fiche en lecture seule ou suivre le CTA pour s’inscrire/se connecter.

**Besoins couverts** : UNC-03, UNC-21, NFR-UNC-08.

### 4.5 Scénario : Presse — accès aux informations publiques

1. Un journaliste accède au site pour préparer un article sur un festival.
2. Il consulte la fiche événement : dates, lieu, programme public, liste des exposants, nom de l’organisateur.
3. Il consulte la fiche organisateur pour les coordonnées de contact (email, site web) et la charte.
4. Il n’a pas besoin de créer de compte ; il a toutes les informations publiques nécessaires.
5. S’il souhaite des informations complémentaires (dossier de presse, accès privilégié), un CTA « Contact presse » ou « Demande d’accréditation » peut le rediriger vers une page dédiée ou un formulaire (avec ou sans compte selon politique).

**Besoins couverts** : UNC-07, UNC-12, UNC-23, UNC-25.

### 4.6 Scénario : Multi-piliers — de l’événement à l’exposant puis à l’organisateur

1. L’utilisateur non connecté consulte la fiche d’un événement « Salon du livre 2026 ».
2. Il clique sur un exposant (librairie) dans la liste des exposants de l’événement ; il accède à la fiche exposant (entreprise, autres éditions participées, contact).
3. Depuis la fiche exposant, il clique sur un autre événement auquel l’exposant participe ; il accède à la fiche de cet événement.
4. Depuis cette fiche événement, il clique sur l’organisateur ; il accède à la fiche organisateur (tous les événements de cet organisateur, charte, contact).
5. Il décide de s’inscrire en tant qu’exposant pour le prochain événement de cet organisateur ; il clique sur « S’inscrire en tant qu’exposant » et est redirigé vers l’inscription.

**Besoins couverts** : UNC-18, UNC-19, UNC-22.

### 4.7 Scénario : Utilisation mobile — consultation rapide

1. L’utilisateur consulte le catalogue depuis son smartphone (non connecté).
2. Il ouvre la liste des événements ; la liste s’affiche de façon lisible (responsive), avec vignettes et titres.
3. Il applique un filtre « Ce mois-ci » ; la liste se met à jour.
4. Il ouvre une fiche événement ; le contenu est structuré (titres, blocs) et l’ensemble est scrollable ; les liens vers organisateur et exposants sont cliquables.
5. Il souhaite réserver un atelier ; le bouton « Réserver » affiche le message « Connectez-vous ou créez un compte » ; les boutons « Se connecter » et « S’inscrire » sont visibles et utilisables au doigt (taille tactile).

**Besoins couverts** : UNC-01, UNC-04, UNC-05, UNC-07, UNC-21, NFR-UNC-12, NFR-UNC-13.

---

## 5. Pain points et opportunités

### 5.1 Pain points

| Pain point | Impact | Besoin associé |
|------------|--------|-----------------|
| **Infos dispersées** | Les événements et organisateurs sont sur plusieurs sites ; pas de vue d’ensemble. | Catalogue unique (annuaire événements + répertoire organisateurs + répertoire exposants) (UNC-04, UNC-10, UNC-13). |
| **Méfiance avant inscription** | L’utilisateur hésite à créer un compte sans voir ce que propose la plateforme. | Façade publique en lecture seule sans obligation de compte (UNC-01, UNC-03, UNC-07, UNC-12, UNC-15). |
| **Recherche inefficace** | Impossible de trouver rapidement un événement ou un exposant par mot-clé. | Recherche globale et filtres (UNC-16, UNC-17, UNC-05, UNC-11, UNC-14). |
| **CTAs peu visibles** | L’utilisateur ne sait pas comment s’inscrire ou se connecter. | CTAs clairs et contextuels (UNC-02, UNC-19, UNC-20, UNC-21, NFR-UNC-13). |
| **Perte de contexte après connexion** | Après inscription, l’utilisateur ne retrouve pas la page qu’il consultait. | Retour au contexte après connexion/inscription (UNC-22). |
| **Données obsolètes** | Les fiches événement ou exposants ne sont pas à jour. | Cohérence catalogue / espaces dédiés (UNC-24). |

### 5.2 Opportunités

| Opportunité | Description | Besoin associé |
|-------------|-------------|-----------------|
| **Store unique** | Un seul lieu pour découvrir événements, organisateurs et exposants ; confiance et visibilité. | UNC-04 à UNC-18, UNC-23. |
| **Preuve sociale** | Afficher les organisateurs et exposants existants rassure les futurs inscrits. | UNC-10, UNC-12, UNC-13, UNC-15. |
| **Passerelles explicites** | CTAs contextuels (« Réserver », « Déposer une candidature ») guident vers l’inscription sans friction. | UNC-19 à UNC-22. |
| **Accessibilité et performance** | Façade rapide et accessible pour tous (mobile, handicap, faible débit). | NFR-UNC-01 à NFR-UNC-04, NFR-UNC-10 à NFR-UNC-12. |

### 5.3 Synthèse des opportunités produit

- **Réduction du taux de rebond** : Un catalogue clair, rapide et bien structuré (événements, organisateurs, exposants) incite à rester sur le site et à explorer plusieurs pages avant de quitter ou de s’inscrire.
- **Augmentation du taux de conversion visiteur → compte** : Des CTAs visibles et contextuels (« Réserver », « Déposer une candidature ») avec message explicite (« Connectez-vous ou créez un compte ») et retour au contexte après inscription réduisent l’abandon.
- **Différenciation concurrence** : Un Store unique (trois piliers + recherche globale) se distingue des sites par festival ou annuaires fragmentés ; positionnement « guichet unique » pour l’événementiel.
- **Conformité et confiance** : Informations légales (mentions, CGU, confidentialité) et accessibilité (WCAG) renforcent la crédibilité et réduisent les risques juridiques.

---

## 6. Parcours type — description wireframe (texte)

### 6.1 Écran d’accueil / landing (utilisateur non connecté)

- **En-tête** : Logo plateforme ; lien « Événements » ; lien « Organisateurs » ; lien « Exposants » ; bouton « Se connecter » ; bouton « S’inscrire » (ou menu déroulant : Organisateur / Exposant / Visiteur).
- **Zone principale** : Titre d’accroche (ex. « Découvrez les événements et festivals ») ; champ de recherche globale (placeholder « Rechercher un événement, un organisateur, un exposant ») ; ou bloc « Événements à la une » / « Prochains événements » (liste ou cartes).
- **Pied de page** : Liens « Mentions légales », « CGU », « Politique de confidentialité », « Accessibilité » ; contact ou formulaire selon politique.

### 6.2 Liste des événements (annuaire)

- **Filtres** (barre latérale ou bandeau) : Date (début/fin, « à venir ») ; Lieu (texte ou région) ; Organisateur (liste ou recherche) ; Thème (liste) ; bouton « Réinitialiser ».
- **Vue** : Bascule « Liste » / « Carte » (si carte : marqueurs géographiques).
- **Liste** : Cartes ou lignes : vignette (si applicable), titre événement, dates, lieu, organisateur (lien), bouton « Voir la fiche ».
- **Pagination** ou chargement progressif au scroll.

### 6.3 Fiche événement (détail public)

- **Bloc 1** : Titre ; dates ; lieu ; organisateur (nom, lien vers fiche organisateur).
- **Bloc 2** : Description / présentation (texte).
- **Bloc 3** : Programme public (animations, horaires, salles) ; vue chronologique ou par salle.
- **Bloc 4** : Exposants (liste avec liens vers fiches exposant).
- **Bloc 5** : Services proposés (ateliers, concours, pass si activés) ; bouton « Réserver un atelier » / « Acheter un pass » → message « Connectez-vous ou créez un compte visiteur » + boutons Connexion / Inscription ; bouton « Déposer une candidature exposant » → idem pour compte exposant.
- **Pied de fiche** : Retour liste ; partage (lien) si applicable.

Ces descriptions sont indicatives ; les maquettes et wireframes graphiques relèvent du design.

---

## 7. Priorisation des besoins (MoSCoW)

### 7.1 Must have (indispensable)

- UNC-01 à UNC-03 (accès Façade publique, CTAs, lecture seule).
- UNC-04 à UNC-09 (annuaire événements : liste, filtres, fiche, programme public, exposants de l’événement).
- UNC-10 à UNC-12 (répertoire organisateurs : liste, filtres, fiche).
- UNC-13 à UNC-15 (répertoire exposants : liste, filtres, fiche).
- UNC-16 (recherche globale).
- UNC-18 (liens croisés entre piliers).
- UNC-19, UNC-20 (inscription et connexion).
- UNC-23 (données publiées uniquement).
- UNC-25 (informations légales et accessibilité).
- NFR-UNC-01 à NFR-UNC-06 (performance, disponibilité).
- NFR-UNC-07, NFR-UNC-08 (gouvernance, pas d’accès espaces protégés).
- NFR-UNC-10, NFR-UNC-11, NFR-UNC-12 (utilisabilité, accessibilité, responsive).
- NFR-UNC-14 (pas de stockage de données personnelles pour le catalogue).

### 7.2 Should have (important)

- UNC-06 (vue carte ou liste pour événements).
- UNC-17 (suggestions et affinage recherche).
- UNC-21, UNC-22 (CTA contextuels, retour au contexte après connexion).
- UNC-24 (cohérence catalogue / espaces dédiés).
- NFR-UNC-09 (protection contre les abus).
- NFR-UNC-13 (clarté des CTAs).
- NFR-UNC-15 (RGPD et consentement cookies).

### 7.3 Could have (souhaitable)

- Vue carte géolocalisée pour les événements.
- Fil d’Ariane (breadcrumb) pour la navigation.
- Partage de fiche (lien vers fiche événement, organisateur, exposant).
- Bloc « Événements à venir » ou « À ne pas manquer » sur l’accueil.

### 7.4 Won’t have (hors périmètre ou report)

- Personnalisation du catalogue pour l’utilisateur non connecté (recommandations, historique) — nécessiterait des cookies ou un compte.
- Données ou fonctionnalités des espaces dédiés — traitées dans les documents Organisateurs, Exposants, Visiteurs.

---

## 8. Dépendances et interfaces avec les autres publics

### 8.1 Dépendances

| Dépendance | Description |
|------------|-------------|
| **Organisateurs** | Les événements et les données publiées (programme, exposants) proviennent des organisateurs ; la Façade n’affiche que ce qui est publié par eux. |
| **Exposants** | Les fiches exposant et les participations affichées dépendent des données validées et publiées par les organisateurs. |
| **Plateforme** | Catalogue et macro (KindMother, Miyusearch, Master Butler, WorrySentinel) ; Façade publique gouvernée. |

### 8.2 Interfaces

| Interface | Flux | Besoin UNC |
|-----------|------|------------|
| Utilisateur non connecté → Catalogue | Consultation annuaire événements, répertoire organisateurs, répertoire exposants ; recherche ; lecture seule. | UNC-04 à UNC-18, UNC-23, UNC-24. |
| Utilisateur non connecté → Passerelle inscription | Clic sur « S’inscrire » (organisateur, exposant, visiteur) ; redirection vers formulaire ; après inscription, accès à l’espace dédié. | UNC-19, UNC-22. |
| Utilisateur non connecté → Passerelle connexion | Clic sur « Se connecter » ; redirection vers connexion ; après authentification, accès à l’espace selon type de compte. | UNC-20, UNC-22. |
| Catalogue → Organisateur / Exposant / Visiteur | Les données affichées sont celles publiées par les organisateurs ; les CTAs contextuels pointent vers les actions des espaces dédiés. | UNC-21, UNC-23. |

---

## 9. User stories (format standard)

### 9.1 Accès et découverte

- **US-UNC-01** — En tant qu’**utilisateur non connecté**, je veux **accéder au catalogue** (événements, organisateurs, exposants) **sans créer de compte** **afin de** découvrir l’offre avant de m’engager.  
  *Critères* : Page d’accueil et catalogue accessibles ; pas de blocage ; lecture seule.*

- **US-UNC-02** — En tant qu’**utilisateur non connecté**, je veux **filtrer les événements** (date, lieu, thème) **afin de** trouver rapidement les événements qui m’intéressent.  
  *Critères* : Filtres disponibles ; résultats mis à jour ; réinitialisation possible.*

- **US-UNC-03** — En tant qu’**utilisateur non connecté**, je veux **consulter la fiche détail d’un événement** (présentation, dates, lieu, organisateur, exposants, programme) **afin de** décider si je souhaite participer.  
  *Critères* : Fiche complète en lecture seule ; liens vers organisateur et exposants ; pas de données privées.*

### 9.2 Organisateurs et exposants

- **US-UNC-04** — En tant qu’**utilisateur non connecté**, je veux **consulter le répertoire des organisateurs** et **leurs fiches** (événements, contact, charte) **afin de** identifier et faire confiance aux structures.  
  *Critères* : Liste et fiches organisateurs ; données publiques uniquement.*

- **US-UNC-05** — En tant qu’**utilisateur non connecté**, je veux **consulter le répertoire des exposants** et **leurs fiches** (entreprise, éditions participées, contact) **afin de** découvrir les exposants et leurs participations.  
  *Critères* : Liste et fiches exposants ; vue globale ou par événement ; données publiques.*

### 9.3 Recherche et navigation

- **US-UNC-06** — En tant qu’**utilisateur non connecté**, je veux **effectuer une recherche** sur les événements, organisateurs et exposants **afin de** trouver une information précise.  
  *Critères* : Recherche globale ; résultats par type ; filtres et tri.*

- **US-UNC-07** — En tant qu’**utilisateur non connecté**, je veux **naviguer entre une fiche événement, l’organisateur et les exposants** via des liens **afin de** ne pas perdre le contexte.  
  *Critères* : Liens croisés ; cohérence des données.*

### 9.4 Passerelles vers compte

- **US-UNC-08** — En tant qu’**utilisateur non connecté**, je veux **voir clairement comment m’inscrire** (organisateur, exposant, visiteur) ou **me connecter** **afin de** accéder aux espaces dédiés quand je suis prêt.  
  *Critères* : CTAs visibles ; distinction des types d’inscription ; redirection correcte.*

- **US-UNC-09** — En tant qu’**utilisateur non connecté**, je veux **cliquer sur « Réserver » ou « Déposer une candidature »** sur une fiche **afin d’être redirigé** vers l’inscription ou la connexion, puis de poursuivre mon action.  
  *Critères* : Message explicite si non connecté ; CTA vers inscription/connexion ; retour au contexte après authentification.*

### 9.5 Informations et confiance

- **US-UNC-10** — En tant qu’**utilisateur non connecté**, je veux **accéder aux informations légales** (mentions légales, CGU, confidentialité) **afin de** comprendre les conditions d’utilisation avant de m’inscrire.  
  *Critères* : Liens visibles (footer ou page dédiée) ; contenu à jour.*

- **US-UNC-11** — En tant qu’**utilisateur non connecté**, je veux **ne pas avoir à donner de données personnelles** pour consulter le catalogue **afin de** préserver ma vie privée.  
  *Critères* : Aucune saisie obligatoire pour la consultation ; pas de stockage de données personnelles pour le catalogue (NFR-UNC-14).*

---

## 10. Cas limites et règles métier

### 10.1 Règles métier

| Règle | Description |
|-------|-------------|
| **Lecture seule** | L’utilisateur non connecté ne peut effectuer aucune action d’écriture : pas de création d’événement, pas de candidature exposant, pas de réservation, pas d’achat de billet. |
| **Données publiées** | Seules les entités en statut « Publié » (ou équivalent) et autorisées pour le public sont visibles sur la Façade. |
| **Façade gouvernée** | Le contenu exposé respecte la gouvernance plateforme (Master Butler, WorrySentinel) ; pas d’exposition de données sensibles. |
| **Passerelle** | Toute action réservée aux comptes (réservation, candidature, achat) doit rediriger vers inscription ou connexion avec un message clair. |

### 10.2 Cas limites

| Cas | Comportement attendu |
|-----|----------------------|
| **Accès direct à une URL d’espace dédié** (ex. /organisateur/dashboard) | Redirection vers page de connexion ou message « Accès réservé aux utilisateurs connectés » ; pas d’affichage de données. |
| **Recherche avec aucun résultat** | Message « Aucun résultat » avec suggestion de modifier les critères ou les filtres. |
| **Événement dépublié ou supprimé** | La fiche n’apparaît plus dans la liste ; accès direct à l’URL ancienne → 404 ou message « Événement non disponible ». |
| **Organisateur sans événement publié** | Selon politique : non affiché dans le répertoire organisateurs ou affiché avec mention « Aucun événement publié ». |
| **Exposant avec participation non publiée** | La participation à l’événement n’apparaît pas sur la fiche événement ni sur la fiche exposant pour cet événement. |
| **Maintenance plateforme** | Page de maintenance ou message explicite ; pas d’affichage de données partielles ou incohérentes. |

### 10.3 Métriques de succès

| Métrique | Description | Cible (exemple) |
|----------|-------------|-----------------|
| **Taux de conversion visiteur → compte** | % d’utilisateurs non connectés qui créent un compte ou se connectent après avoir consulté le catalogue. | Suivi (objectif : augmentation). |
| **Taux de rebond** | % d’utilisateurs quittant le site sans interaction (ou après une seule page). | Réduction. |
| **Pages vues par session (non connecté)** | Nombre moyen de pages consultées par session sans compte. | Suivi (indicateur d’engagement). |
| **Utilisation de la recherche** | % de sessions avec au moins une recherche. | Suivi. |
| **Performance** | Temps de chargement moyen des pages catalogue (accueil, liste, fiche). | < 3 s (accueil), < 2 s (listes et fiches). |
| **Accessibilité** | Conformité WCAG 2.1 AA (audit). | Conformité validée. |

---

## 11. Mapping besoins / modules plateforme

| Besoin (sélection) | Module / Opérateur / Kit Miyukini | Rôle |
|--------------------|-----------------------------------|------|
| UNC-04 à UNC-09 (annuaire événements) | Catalogue plateforme, Édition (données publiées), Programme (public) | KindMother (persistance), publication organisateur |
| UNC-10 à UNC-12 (répertoire organisateurs) | Catalogue plateforme, Profil organisateur (public) | KindMother, Master Butler (visibilité) |
| UNC-13 à UNC-15 (répertoire exposants) | Catalogue plateforme, Exposant (participations publiées) | KindMother, publication organisateur |
| UNC-16, UNC-17 (recherche) | Miyusearch | Indexation et recherche sur champs publics |
| UNC-19 à UNC-22 (passerelles) | Miyauth (inscription, connexion), redirections applicatives | Authentification et routage selon type de compte |
| UNC-23, UNC-24 (données publiées, cohérence) | StrongFather, Master Butler, WorrySentinel | Gouvernance, permissions, états de confiance |
| NFR-UNC-07, NFR-UNC-08 (Façade gouvernée, pas d’accès espaces) | Master Butler, WorrySentinel | Contrôle d’accès, Mandat Public d’Accès (Façade publique gouvernée) |

Ce mapping est indicatif ; les spécifications techniques détaillées (API, crates) relèvent d’autres documents.

---

## 12. Critères d’acceptation détaillés (sélection)

### 12.1 Annuaire des événements (UNC-04 à UNC-09)

- **Liste** : Affichage des événements dont le statut est « Publié » ; colonnes ou cartes : titre, dates, lieu, organisateur (lien), vignette si applicable ; pagination ou chargement progressif au-delà de N résultats.
- **Filtres** : Au moins : date (début/fin ou « à venir »), lieu (texte ou région), organisateur (liste ou recherche), thème/catégorie ; bouton « Réinitialiser ».
- **Fiche événement** : Blocs : Présentation (nom, description, dates, lieu) ; Organisateur (nom, lien vers fiche organisateur) ; Exposants (liste avec liens vers fiches) ; Programme public (animations, horaires, salles) ; Services proposés (ateliers, concours, pass si activés — avec CTA « Connectez-vous pour réserver »). Aucun bloc budget, candidatures, documents internes.
- **Programme public** : Données issues du module Programme publié par l’organisateur ; vue chronologique ou par salle ; pas de formulaire de réservation sans compte.

### 12.2 Répertoire des organisateurs (UNC-10 à UNC-12)

- **Liste** : Organisateurs avec au moins un événement publié (ou selon règle plateforme) ; nom, région ou nombre d’événements ; lien vers fiche.
- **Fiche organisateur** : Nom, description, liste des événements publiés (liens), coordonnées de contact (email, site, selon paramétrage), charte ou valeurs si publiées. Pas d’accès équipe, budget, candidatures.

### 12.3 Répertoire des exposants (UNC-13 à UNC-15)

- **Liste** : Exposants avec au moins une participation validée et publiée ; vue « Tous » ou « Par événement » (filtre par événement) ; nom, catégorie, événements ; lien vers fiche.
- **Fiche exposant** : Nom entreprise/exposant, description, secteur/catégorie, liste des éditions participées (événement, stand si publié), contact (selon paramétrage). Pas de documents, factures, candidatures en cours.

### 12.4 Recherche (UNC-16, UNC-17)

- **Champ** : Recherche globale ou par pilier (événements, organisateurs, exposants) ; déclenchement sur validation (bouton ou Entrée).
- **Résultats** : Regroupement par type (onglets ou sections) ; titre, extrait, lien vers fiche ; tri (pertinence, date, nom) ; message « Aucun résultat » si vide.
- **Autocomplétion** (should have) : Suggestions pendant la saisie ; clic sur une suggestion → résultat ou fiche.

### 12.5 Passerelles (UNC-19 à UNC-22)

- **Inscription** : Trois options clairement libellées : « S’inscrire en tant qu’organisateur », « S’inscrire en tant qu’exposant », « S’inscrire en tant que visiteur » ; redirection vers le formulaire correspondant.
- **Connexion** : Un lien « Se connecter » ; redirection vers page de connexion ; après succès, redirection vers l’espace du type de compte (organisateur, exposant, visiteur).
- **CTA contextuels** : Sur fiche événement : « Réserver un atelier » / « Acheter un pass » → message « Connectez-vous ou créez un compte visiteur » + boutons Connexion / Inscription visiteur. « Déposer une candidature » → idem pour compte exposant.
- **Retour contexte** : Paramètre d’URL ou session pour mémoriser la page d’origine ; après connexion/inscription, redirection vers cette page ou vers l’action demandée.

---

## 13. Glossaire et références

### 13.1 Glossaire (extrait)

| Terme | Définition |
|-------|------------|
| **Utilisateur non connecté** | Personne accédant au catalogue sans compte ni authentification. |
| **Façade publique gouvernée** | Surface d’exposition du catalogue en lecture seule, soumise à la gouvernance plateforme (Master Butler, WorrySentinel) ; seules les données publiées et autorisées pour le public sont visibles. |
| **Catalogue (Store)** | Ensemble des trois piliers : annuaire des événements, répertoire des organisateurs, répertoire des exposants. |
| **Données publiées** | Données dont le statut et la visibilité ont été définis par l’organisateur ou la plateforme pour être exposées au public (ex. événement « Publié », exposant validé et publié). |
| **Passerelle** | Lien ou flux permettant de passer de l’état « non connecté » à « connecté » ou « inscrit » (inscription, connexion) pour accéder aux espaces dédiés. |
| **Mandat Public d’Accès** | Autorisation attachée à la Façade publique pour encadrer l’accès des utilisateurs externes non certifiés (glossaire Miyukini). |
| **Utilisateur externe** | Consommateur non certifié de services exposés par le COG ; ici, l’utilisateur non connecté est un utilisateur externe du catalogue. |

### 13.2 Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
- [Utilisateur non connecté — Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md)
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Exposants](../Exposants/_index.md) | [Public Visiteurs](../Visiteurs/_index.md)

---

**Document** : Utilisateur non connecté — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit — référence pour le public Utilisateur non connecté
