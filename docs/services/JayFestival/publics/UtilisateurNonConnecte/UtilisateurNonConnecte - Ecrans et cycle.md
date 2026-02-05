# Utilisateur non connecté — Écrans et cycle

## Contexte

Ce document précise **tous les écrans** du cycle utilisateur du public **Utilisateur non connecté** pour le service JayFestival, avec l’**organisation** (structure, blocs, zones) et les **besoins** associés à chaque écran. Il s’appuie sur le [parcours et l’accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md) et sur l’[analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md).

## Portée / Scope

- **Public** : Toute personne accédant au catalogue **sans compte** ni authentification.
- **Périmètre** : tous les écrans du cycle (Façade publique gouvernée), organisation et besoins par écran.
- **Hors périmètre** : maquettes graphiques, spécifications API.

---

## 1. Vue d’ensemble du cycle

Le cycle utilisateur non connecté se décompose en **phases** :

| Phase | Description | Écrans concernés |
|-------|-------------|-------------------|
| **Accès** | Arrivée sur le site ; accès au catalogue sans compte. | Landing / Accueil catalogue |
| **Annuaire des événements** | Liste, filtres, fiche événement, programme public, exposants de l’événement. | Liste événements, Fiche événement, Programme public, Exposants de l’événement |
| **Répertoire des organisateurs** | Liste, filtres, fiche organisateur. | Liste organisateurs, Fiche organisateur |
| **Répertoire des exposants** | Liste, filtres, fiche exposant. | Liste exposants, Fiche exposant |
| **Recherche** | Recherche globale sur les trois piliers ; résultats, affinage. | Recherche (résultats), Affinage |
| **Passerelles** | Connexion ou inscription (organisateur, exposant, visiteur). | Connexion, Inscription (choix type), CTA contextuels |
| **Informations légales** | Mentions légales, CGU, confidentialité, accessibilité. | Mentions légales, CGU, Confidentialité, Accessibilité |

---

## 2. Écrans du cycle — détail

### 2.1 Accès

#### UNC-E01 — Landing / Accueil catalogue

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Point d’entrée du site ; afficher la Façade publique (accès au catalogue) et les CTAs inscription/connexion. |
| **Organisation** | **En-tête** : logo plateforme ; lien **Événements** ; lien **Organisateurs** ; lien **Exposants** ; bouton **Se connecter** ; bouton **S’inscrire** (ou menu déroulant : Organisateur / Exposant / Visiteur). **Zone principale** : titre d’accroche (ex. « Découvrez les événements et festivals ») ; **champ de recherche globale** (placeholder « Rechercher un événement, un organisateur, un exposant ») ; ou bloc « Événements à la une » / « Prochains événements » (liste ou cartes). **Pied de page** : liens **Mentions légales**, **CGU**, **Politique de confidentialité**, **Accessibilité** ; contact ou formulaire selon politique. |
| **Besoins** | UNC-01, UNC-02, UNC-16, UNC-25. |
| **Navigation** | Entrée : URL site. Sortie : Liste événements (UNC-E02), Liste organisateurs (UNC-E06), Liste exposants (UNC-E08), Recherche (UNC-E10), Connexion (UNC-E12), Inscription (UNC-E13). |

---

### 2.2 Annuaire des événements

#### UNC-E02 — Liste des événements

| Attribut | Description |
|----------|-------------|
| **Phase** | Annuaire événements |
| **Objectif** | Afficher la liste des événements (éditions) publiés ; filtrer et trier ; accéder aux fiches. |
| **Organisation** | Titre « Événements ». **Filtres** (barre latérale ou bandeau) : Date (début/fin, « à venir ») ; Lieu (texte ou région) ; Organisateur (liste ou recherche) ; Thème (liste) ; bouton **Réinitialiser**. **Vue** : Bascule **Liste** / **Carte** (si carte : marqueurs géographiques). **Liste** : cartes ou lignes — vignette (si applicable), titre événement, dates, lieu, organisateur (lien), bouton **Voir la fiche**. **Pagination** ou chargement progressif au scroll. |
| **Besoins** | UNC-04, UNC-05, UNC-06. |
| **Navigation** | Entrée : Landing (UNC-E01), menu Événements. Sortie : Fiche événement (UNC-E03). |

#### UNC-E03 — Fiche événement (détail public)

| Attribut | Description |
|----------|-------------|
| **Phase** | Annuaire événements |
| **Objectif** | Afficher le détail public d’un événement : présentation, dates, lieu, organisateur, exposants, programme public ; CTAs contextuels (Réserver, Candidater) → redirection vers connexion/inscription. |
| **Organisation** | **Bloc 1** : Titre ; dates ; lieu ; organisateur (nom, lien vers fiche organisateur UNC-E07). **Bloc 2** : Description / présentation (texte). **Bloc 3** : **Programme public** (animations, horaires, salles) — vue chronologique ou par salle ; filtres jour, type. **Bloc 4** : **Exposants de l’événement** (liste avec liens vers fiches exposant UNC-E09). **Bloc 5** : **Services proposés** (ateliers, concours, pass si activés) ; bouton **Réserver un atelier** / **Acheter un pass** → message « Connectez-vous ou créez un compte visiteur » + boutons Connexion / Inscription visiteur (UNC-21) ; bouton **Déposer une candidature exposant** → idem pour compte exposant. **Pied de fiche** : Retour liste ; partage (lien) si applicable. |
| **Besoins** | UNC-07, UNC-08, UNC-09, UNC-21, UNC-23. |
| **Navigation** | Entrée : Liste événements (UNC-E02), Recherche (UNC-E10). Sortie : Fiche organisateur (UNC-E07), Fiche exposant (UNC-E09), Connexion (UNC-E12), Inscription (UNC-E13). |

---

### 2.3 Répertoire des organisateurs

#### UNC-E06 — Liste des organisateurs

| Attribut | Description |
|----------|-------------|
| **Phase** | Répertoire organisateurs |
| **Objectif** | Afficher la liste des structures organisatrices ; filtrer ; accéder aux fiches. |
| **Organisation** | Titre « Organisateurs ». **Filtres** : Nom (recherche) ; Région ; Type d’événement ; Année. **Liste** : cartes ou lignes — nom structure, région, nombre d’événements (ou liste courte), lien **Voir la fiche**. Pagination si besoin. |
| **Besoins** | UNC-10, UNC-11. |
| **Navigation** | Entrée : Landing (UNC-E01), menu Organisateurs. Sortie : Fiche organisateur (UNC-E07). |

#### UNC-E07 — Fiche organisateur (détail public)

| Attribut | Description |
|----------|-------------|
| **Phase** | Répertoire organisateurs |
| **Objectif** | Afficher le détail public d’un organisateur : nom, description, événements publiés, contact, charte. |
| **Organisation** | **Bloc 1** : Nom de la structure ; description. **Bloc 2** : **Liste des événements publiés** (liens vers fiches événement UNC-E03). **Bloc 3** : Coordonnées de contact (email, site web, selon paramétrage organisateur). **Bloc 4** : Charte ou valeurs (si publiées). Pas d’accès équipe, budget, candidatures. **Pied de fiche** : Retour liste. |
| **Besoins** | UNC-12, UNC-18. |
| **Navigation** | Entrée : Liste organisateurs (UNC-E06), Fiche événement (UNC-E03), Recherche (UNC-E10). Sortie : Fiche événement (UNC-E03). |

---

### 2.4 Répertoire des exposants

#### UNC-E08 — Liste des exposants

| Attribut | Description |
|----------|-------------|
| **Phase** | Répertoire exposants |
| **Objectif** | Afficher la liste des exposants (globale ou par événement) ; filtrer ; accéder aux fiches. |
| **Organisation** | Titre « Exposants ». **Filtres** : Recherche (nom, catégorie) ; Catégorie/secteur ; Événement (liste) ; Région. **Vue** : globale (tous événements) ou **Par événement** (sélection d’un événement). **Liste** : cartes ou lignes — nom entreprise/exposant, catégorie, événements participés, lien **Voir la fiche**. Pagination si besoin. |
| **Besoins** | UNC-13, UNC-14. |
| **Navigation** | Entrée : Landing (UNC-E01), menu Exposants. Sortie : Fiche exposant (UNC-E09). |

#### UNC-E09 — Fiche exposant (détail public)

| Attribut | Description |
|----------|-------------|
| **Phase** | Répertoire exposants |
| **Objectif** | Afficher le détail public d’un exposant : entreprise, secteur, éditions participées, contact. |
| **Organisation** | **Bloc 1** : Nom entreprise ou exposant ; description ; secteur/catégorie. **Bloc 2** : **Liste des éditions participées** (événements, stands attribués si publiés) — liens vers fiches événement (UNC-E03). **Bloc 3** : Coordonnées de contact (selon paramétrage). Pas d’accès documents privés, factures, candidatures en cours. **Pied de fiche** : Retour liste. |
| **Besoins** | UNC-15, UNC-18. |
| **Navigation** | Entrée : Liste exposants (UNC-E08), Fiche événement (UNC-E03), Recherche (UNC-E10). Sortie : Fiche événement (UNC-E03). |

---

### 2.5 Recherche

#### UNC-E10 — Recherche (résultats et affinage)

| Attribut | Description |
|----------|-------------|
| **Phase** | Recherche |
| **Objectif** | Effectuer une recherche textuelle sur événements, organisateurs et exposants ; affiner et trier les résultats. |
| **Organisation** | **Champ de recherche** : placeholder « Rechercher un événement, un organisateur, un exposant » ; bouton **Rechercher** ou déclenchement à la validation (Entrée). **Résultats** : regroupement par type (onglets ou sections) — **Événements**, **Organisateurs**, **Exposants**. Pour chaque type : liste (titre/nom, extrait, lien vers fiche). **Affinage** : filtres après recherche (date, lieu, catégorie, etc.). **Tri** : pertinence, date, nom. Message **« Aucun résultat »** si vide, avec suggestion de modifier les critères. **Autocomplétion** (optionnel) : suggestions pendant la saisie. |
| **Besoins** | UNC-16, UNC-17, UNC-18. |
| **Navigation** | Entrée : Landing (UNC-E01), champ recherche en-tête. Sortie : Fiche événement (UNC-E03), Fiche organisateur (UNC-E07), Fiche exposant (UNC-E09). |

---

### 2.6 Passerelles vers connexion et inscription

#### UNC-E12 — Connexion

| Attribut | Description |
|----------|-------------|
| **Phase** | Passerelles |
| **Objectif** | Permettre à un utilisateur ayant déjà un compte de se connecter ; après authentification, redirection vers l’espace correspondant (organisateur, exposant, visiteur). |
| **Organisation** | Titre « Se connecter ». Champs : email, mot de passe. Liens : « Mot de passe oublié », « S’inscrire ». Bouton **Se connecter**. Message d’erreur si échec. **Retour au contexte** : après connexion, redirection vers la page d’origine ou l’action demandée (ex. fiche événement pour réserver) si mémorisé (UNC-22). |
| **Besoins** | UNC-20, UNC-22. |
| **Navigation** | Entrée : Landing (UNC-E01), CTA « Se connecter » sur fiche événement (UNC-E03). Sortie : Espace organisateur / exposant / visiteur selon type de compte. |

#### UNC-E13 — Inscription (choix du type de compte)

| Attribut | Description |
|----------|-------------|
| **Phase** | Passerelles |
| **Objectif** | Permettre de choisir le type d’inscription (organisateur, exposant, visiteur) et rediriger vers le formulaire correspondant. |
| **Organisation** | Titre « S’inscrire ». **Trois options** clairement libellées : **S’inscrire en tant qu’organisateur** ; **S’inscrire en tant qu’exposant** ; **S’inscrire en tant que visiteur**. Clic → redirection vers le formulaire d’inscription du public concerné (documents Organisateurs, Exposants, Visiteurs). Lien « Déjà un compte ? Se connecter » (UNC-E12). **Retour au contexte** après inscription (UNC-22). |
| **Besoins** | UNC-19, UNC-22. |
| **Navigation** | Entrée : Landing (UNC-E01), CTA « S’inscrire » sur fiche événement (UNC-E03). Sortie : Formulaire inscription organisateur / exposant / visiteur (hors périmètre UNC). |

#### UNC-E11 — CTA contextuels (message non connecté)

| Attribut | Description |
|----------|-------------|
| **Phase** | Passerelles |
| **Objectif** | Afficher un message explicite lorsque l’utilisateur non connecté clique sur une action réservée aux comptes (Réserver, Candidater) ; proposer Connexion ou Inscription. |
| **Organisation** | **Modal ou bandeau** : message « La réservation est réservée aux utilisateurs connectés. Connectez-vous ou créez un compte visiteur pour réserver. » (ou équivalent pour « Déposer une candidature » → compte exposant). Boutons **Se connecter** (→ UNC-E12), **S’inscrire** (→ UNC-E13 avec pré-sélection type : visiteur ou exposant). Option **Retour** (fermer et rester sur la fiche). Aucune donnée saisie ni enregistrée. |
| **Besoins** | UNC-03, UNC-21. |
| **Navigation** | Entrée : Fiche événement (UNC-E03) — clic sur « Réserver », « Acheter un pass », « Déposer une candidature ». Sortie : Connexion (UNC-E12), Inscription (UNC-E13), ou fermeture (rester sur fiche). |

---

### 2.7 Informations légales

#### UNC-E14 — Mentions légales, CGU, Confidentialité, Accessibilité

| Attribut | Description |
|----------|-------------|
| **Phase** | Informations légales |
| **Objectif** | Donner accès aux informations légales et à la politique d’accessibilité. |
| **Organisation** | **Pages ou sections** accessibles depuis le pied de page (Landing et écrans catalogue) : **Mentions légales** (éditeur, hébergeur, contact) ; **CGU** (conditions générales d’utilisation) ; **Politique de confidentialité** (données, cookies, RGPD) ; **Accessibilité** (engagement, contact). Contenu à jour. Pas de formulaire ni de saisie ; lecture seule. |
| **Besoins** | UNC-25. |
| **Navigation** | Entrée : Footer (liens) sur Landing (UNC-E01) et écrans catalogue. Sortie : Retour (précédent ou accueil). |

---

## 3. Récapitulatif des écrans et besoins

| Écran | Id | Phase | Besoins principaux |
|-------|-----|-------|--------------------|
| Landing / Accueil catalogue | UNC-E01 | Accès | UNC-01, UNC-02, UNC-16, UNC-25 |
| Liste des événements | UNC-E02 | Annuaire événements | UNC-04, UNC-05, UNC-06 |
| Fiche événement | UNC-E03 | Annuaire événements | UNC-07, UNC-08, UNC-09, UNC-21, UNC-23 |
| Liste des organisateurs | UNC-E06 | Répertoire organisateurs | UNC-10, UNC-11 |
| Fiche organisateur | UNC-E07 | Répertoire organisateurs | UNC-12, UNC-18 |
| Liste des exposants | UNC-E08 | Répertoire exposants | UNC-13, UNC-14 |
| Fiche exposant | UNC-E09 | Répertoire exposants | UNC-15, UNC-18 |
| Recherche (résultats et affinage) | UNC-E10 | Recherche | UNC-16, UNC-17, UNC-18 |
| CTA contextuels (message non connecté) | UNC-E11 | Passerelles | UNC-03, UNC-21 |
| Connexion | UNC-E12 | Passerelles | UNC-20, UNC-22 |
| Inscription (choix type) | UNC-E13 | Passerelles | UNC-19, UNC-22 |
| Mentions légales / CGU / Confidentialité / Accessibilité | UNC-E14 | Informations légales | UNC-25 |

---

## 4. Navigation type (flux principal)

```
Landing (accueil catalogue)
  ├── Liste événements → Fiche événement → [CTA Réserver/Candidater] → CTA contextuel → Connexion / Inscription
  ├── Liste organisateurs → Fiche organisateur → (liens vers événements) → Fiche événement
  ├── Liste exposants → Fiche exposant → (liens vers événements) → Fiche événement
  ├── Recherche → Résultats (événements, organisateurs, exposants) → Fiches
  ├── Connexion → Espace (organisateur / exposant / visiteur)
  ├── Inscription → Formulaire (organisateur / exposant / visiteur)
  └── Footer → Mentions légales, CGU, Confidentialité, Accessibilité
```

**Liens croisés** (UNC-18) : depuis Fiche événement → Fiche organisateur, Fiche exposant ; depuis Fiche organisateur → Fiche événement ; depuis Fiche exposant → Fiche événement.

---

## 5. Références

- [Utilisateur non connecté — Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md)
- [Utilisateur non connecté — Analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md)
- [Document fondateur JayFestival](../../JayFestival%20-%20Document%20Fondateur.md)
