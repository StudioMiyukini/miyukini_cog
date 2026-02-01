# Organisateurs — Écrans et cycle

## Contexte

Ce document précise **tous les écrans** du cycle utilisateur du public **Organisateurs** pour le service Miyukini Festival Service, avec l’**organisation** (structure, blocs, zones) et les **besoins** associés à chaque écran. Il s’appuie sur le [parcours et les capacités](./Organisateurs%20-%20Parcours%20Capacites%20Livrables.md) et sur l’[analyse des besoins](./Organisateurs%20-%20Analyse%20des%20besoins.md).

## Portée / Scope

- **Public** : Organisateurs (structures qui créent et gèrent des événements/festivals).
- **Périmètre** : tous les écrans du cycle (de l’arrivée à la clôture), organisation et besoins par écran.
- **Hors périmètre** : maquettes graphiques, spécifications API.

---

## 1. Vue d’ensemble du cycle

Le cycle organisateur se décompose en **phases** :

| Phase | Description | Écrans concernés |
|-------|-------------|-------------------|
| **Accès** | Arrivée depuis le catalogue ou URL directe ; connexion ou inscription. | Landing (catalogue), Connexion, Inscription organisateur |
| **Tableau de bord** | Vue globale sur toutes les éditions (multi-festivals). | Tableau de bord organisateur, Liste des éditions |
| **Édition** | Création, paramétrage, dashboard par édition. | Création édition, Paramétrage édition, Dashboard édition |
| **Exposants** | Annuaire, candidatures, fiches, devis, factures. | Liste exposants, Candidatures, Fiche exposant, Devis, Factures, Import exposants |
| **Plan de salle** | Zones, stands, attribution. | Plan de salle (définition), Attribution emplacements, Visualisation plan |
| **Programme** | Animations, créneaux, salles. | Programme (liste/vues), Création/édition animation |
| **Budget** | Revenus, dépenses, balance. | Saisie budget, Ventilation, Balance et statistiques |
| **Documents** | Contrats types, envoi, historique. | Documents et légal, Envoi documents |
| **Communication** | Annonces, notifications. | Annonces, Notifications ciblées, Paramétrage notifications |
| **Services visiteur** | Activation et paramétrage par édition. | Services visiteur (activation) |
| **Publication** | Publication au catalogue, clôture. | Publication édition, Clôture édition |
| **Compte et équipe** | Profil, équipe, invitations. | Mon compte, Équipe, Invitation membre |

---

## 2. Écrans du cycle — détail

### 2.1 Accès

#### ORG-E01 — Landing / Accueil catalogue (passerelle)

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Point d’entrée depuis le catalogue (utilisateur non connecté) ; afficher les CTAs « S’inscrire » et « Se connecter ». |
| **Organisation** | En-tête : logo, lien Événements / Organisateurs / Exposants, bouton **Se connecter**, bouton **S’inscrire** (ou menu : Organisateur / Exposant / Visiteur). Zone principale : accroche + lien vers **S’inscrire en tant qu’organisateur**. Pied : liens légaux. |
| **Besoins** | UNC-02, UNC-19 (passerelle depuis [Utilisateur non connecté](../UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md)). |
| **Navigation** | Entrée : URL catalogue. Sortie : Connexion (ORG-E02), Inscription organisateur (ORG-E03). |

#### ORG-E02 — Connexion

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Permettre à un organisateur de se connecter (email + mot de passe ou lien magique). |
| **Organisation** | Titre « Se connecter ». Champs : email, mot de passe. Liens : « Mot de passe oublié », « S’inscrire ». Bouton **Se connecter**. Message d’erreur si échec. |
| **Besoins** | ORG-01 (compte), Miyauth. |
| **Navigation** | Entrée : Landing, lien direct. Sortie : Tableau de bord organisateur (ORG-E04). |

#### ORG-E03 — Inscription organisateur

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Création du compte organisateur (structure, contact, validation selon politique). |
| **Organisation** | Titre « Créer un compte organisateur ». Bloc 1 : email, mot de passe, confirmation. Bloc 2 : nom de la structure, type (asso, collectivité, société), contact (téléphone, adresse). CGU + case à cocher. Bouton **S’inscrire**. Lien « Déjà un compte ? Se connecter ». |
| **Besoins** | ORG-01, ORG-02, ORG-03. |
| **Navigation** | Entrée : Landing. Sortie : Confirmation / attente validation, ou Connexion si validation auto ; puis Tableau de bord (ORG-E04). |

---

### 2.2 Tableau de bord et éditions

#### ORG-E04 — Tableau de bord organisateur

| Attribut | Description |
|----------|-------------|
| **Phase** | Tableau de bord |
| **Objectif** | Vue d’ensemble : toutes les éditions de l’organisateur, indicateurs globaux, accès rapides. |
| **Organisation** | En-tête : nom structure, menu (Éditions, Mon compte, Équipe, Déconnexion). Zone principale : **Bloc synthèse** (nombre d’éditions, prochain événement, alertes éventuelles). **Bloc « Mes éditions »** : raccourci vers liste ou cartes des éditions (brouillon, en cours, à venir, passées). Liens rapides : Créer une édition, Paramètres. |
| **Besoins** | ORG-04, ORG-07. |
| **Navigation** | Entrée : après Connexion. Sortie : Liste des éditions (ORG-E05), Création édition (ORG-E06), Mon compte (ORG-E20), Équipe (ORG-E21). |

#### ORG-E05 — Liste des éditions

| Attribut | Description |
|----------|-------------|
| **Phase** | Tableau de bord |
| **Objectif** | Afficher toutes les éditions (multi-festivals) avec filtres et tri. |
| **Organisation** | Titre « Mes éditions ». Filtres : statut (brouillon, en cours, à venir, passée), année, recherche par nom. Vue : liste ou cartes. Colonnes/cartes : nom, dates, lieu, statut, indicateurs (nb exposants, candidatures en attente). Action : **Voir** (dashboard édition), **Dupliquer**, **Créer une édition**. |
| **Besoins** | ORG-06, ORG-07, ORG-10. |
| **Navigation** | Entrée : Tableau de bord (ORG-E04). Sortie : Dashboard édition (ORG-E07), Création édition (ORG-E06), Duplication (vers ORG-E06 avec pré-remplissage). |

#### ORG-E06 — Création d’une édition

| Attribut | Description |
|----------|-------------|
| **Phase** | Édition |
| **Objectif** | Créer une nouvelle édition avec les métadonnées de base. |
| **Organisation** | Titre « Nouvelle édition ». Formulaire : nom, dates (début, fin), lieu, thème, statut (brouillon). Boutons **Enregistrer**, **Annuler**. Option « Dupliquer depuis une édition » (choix édition source, éléments à dupliquer). |
| **Besoins** | ORG-06, ORG-10. |
| **Navigation** | Entrée : Liste des éditions (ORG-E05), Tableau de bord (ORG-E04). Sortie : Dashboard édition (ORG-E07) ou Liste des éditions. |

#### ORG-E07 — Dashboard édition

| Attribut | Description |
|----------|-------------|
| **Phase** | Édition |
| **Objectif** | Vue synthétique par édition : indicateurs, accès aux modules (exposants, plan, programme, budget, documents). |
| **Organisation** | Fil d’Ariane : Mes éditions > [Nom édition]. Onglets ou menu latéral : **Vue d’ensemble**, Exposants, Plan de salle, Programme, Budget, Documents, Notifications, Services visiteur, Paramètres, Publication. Zone principale : **Indicateurs** (nombre d’exposants, candidatures en attente, budget synthèse, nombre d’animations, stands attribués). Liens rapides vers chaque module. |
| **Besoins** | ORG-08, ORG-09. |
| **Navigation** | Entrée : Liste des éditions (ORG-E05), après Création (ORG-E06). Sortie : tous les écrans module (exposants, plan, programme, budget, documents, etc.). |

#### ORG-E08 — Paramétrage de l’édition

| Attribut | Description |
|----------|-------------|
| **Phase** | Édition |
| **Objectif** | Configurer les paramètres de l’édition (nom, dates, lieu, thème, règles, conditions). |
| **Organisation** | Titre « Paramètres de l’édition ». Onglets ou sections : **Général** (nom, dates, lieu, thème), **Règles** (conditions d’inscription exposants, règles réservation), **Objectifs**. Formulaire par section. Boutons **Enregistrer**, **Annuler**. |
| **Besoins** | ORG-09. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Dashboard édition. |

---

### 2.3 Exposants

#### ORG-E09 — Liste des exposants (annuaire local)

| Attribut | Description |
|----------|-------------|
| **Phase** | Exposants |
| **Objectif** | Afficher la liste des exposants de l’édition avec statut et informations clés. |
| **Organisation** | Titre « Exposants — [Nom édition] ». Filtres : statut (candidat, validé, refusé), catégorie, recherche. Tableau : colonnes (nom, contact, statut, emplacement, actions). Actions : Voir, Modifier, Export CSV/Excel. Bouton **Importer des exposants**. |
| **Besoins** | ORG-12, ORG-18. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Fiche exposant (ORG-E11), Candidatures (ORG-E10), Import exposants (ORG-E18). |

#### ORG-E10 — Candidatures (réception et traitement)

| Attribut | Description |
|----------|-------------|
| **Phase** | Exposants |
| **Objectif** | Consulter et traiter les candidatures exposants (validation, refus). |
| **Organisation** | Titre « Candidatures en attente ». Liste des candidatures : exposant, date de dépôt, pièces jointes. Fiche détail (panneau ou modal) : données exposant, pièces, **Valider** / **Refuser** (avec motif si refus). Notification envoyée à l’exposant (Miyunotify). |
| **Besoins** | ORG-13, ORG-14. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07), Liste exposants (ORG-E09). Sortie : Liste exposants, Fiche exposant (ORG-E11). |

#### ORG-E11 — Fiche exposant

| Attribut | Description |
|----------|-------------|
| **Phase** | Exposants |
| **Objectif** | Consulter et éditer la fiche exposant (coordonnées, statut, emplacement, documents). |
| **Organisation** | Titre « Fiche exposant — [Nom] ». Blocs : **Identité** (nom, contact, catégorie), **Statut** (candidat / validé / refusé, motif si refus), **Emplacement** (stand attribué, lien plan), **Documents** (liste, téléchargement), **Historique** (statuts, dates). Actions : Modifier, Générer devis (ORG-E12), Convertir en facture (ORG-E13). |
| **Besoins** | ORG-15, ORG-16, ORG-17. |
| **Navigation** | Entrée : Liste exposants (ORG-E09), Candidatures (ORG-E10). Sortie : Devis (ORG-E12), Factures (ORG-E13), Liste exposants. |

#### ORG-E12 — Génération de devis

| Attribut | Description |
|----------|-------------|
| **Phase** | Exposants |
| **Objectif** | Créer un devis à partir des données exposant et des tarifs de l’édition. |
| **Organisation** | Titre « Nouveau devis — [Exposant] ». Formulaire : lignes (libellé, quantité, prix unitaire, total), conditions, date d’échéance. Aperçu PDF. Boutons **Enregistrer**, **Envoyer par email**, **Annuler**. |
| **Besoins** | ORG-16. |
| **Navigation** | Entrée : Fiche exposant (ORG-E11). Sortie : Fiche exposant, Factures (ORG-E13). |

#### ORG-E13 — Factures (suivi et conversion)

| Attribut | Description |
|----------|-------------|
| **Phase** | Exposants |
| **Objectif** | Consulter les factures, convertir un devis accepté en facture, suivre les paiements. |
| **Organisation** | Titre « Facturation — [Édition] » ou intégré à la fiche exposant. Liste des devis/factures par exposant : numéro, date, montant, statut (envoyé, accepté, refusé, payé). Action **Convertir en facture** (depuis devis accepté). Envoi par email (Miyuinvoice). |
| **Besoins** | ORG-17. |
| **Navigation** | Entrée : Fiche exposant (ORG-E11), Liste exposants (ORG-E09). Sortie : Fiche exposant. |

#### ORG-E18 — Import exposants (CSV / tableur)

| Attribut | Description |
|----------|-------------|
| **Phase** | Exposants |
| **Objectif** | Importer une liste d’exposants depuis un fichier CSV ou tableur. |
| **Organisation** | Titre « Importer des exposants ». Zone upload : glisser-déposer ou sélection fichier. Étape 2 : mapping des colonnes (aperçu). Étape 3 : prévisualisation, validation. Boutons **Importer**, **Annuler**. |
| **Besoins** | ORG-18. |
| **Navigation** | Entrée : Liste exposants (ORG-E09). Sortie : Liste exposants. |

---

### 2.4 Plan de salle

#### ORG-E14 — Plan de salle (définition des zones et stands)

| Attribut | Description |
|----------|-------------|
| **Phase** | Plan de salle |
| **Objectif** | Définir les zones et stands (tailles, légende, zones techniques ou réservées). |
| **Organisation** | Titre « Plan de salle — [Édition] ». Canvas ou grille : dessin des zones et stands. Panneau latéral : liste des stands (nom, taille, type), légende. Boutons **Enregistrer**, **Export PDF/image**. |
| **Besoins** | ORG-19, ORG-22. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Attribution emplacements (ORG-E15), Visualisation plan (ORG-E16). |

#### ORG-E15 — Attribution des emplacements

| Attribut | Description |
|----------|-------------|
| **Phase** | Plan de salle |
| **Objectif** | Attribuer un emplacement (stand/zone) à un exposant. |
| **Organisation** | Vue partagée : **Plan** (canvas avec stands) + **Liste des exposants** (validés sans emplacement ou avec emplacement). Drag & drop exposant → stand ou formulaire (sélection exposant, sélection stand). Alerte si stand déjà attribué. Bouton **Enregistrer**. |
| **Besoins** | ORG-20. |
| **Navigation** | Entrée : Plan de salle (ORG-E14), Liste exposants (ORG-E09). Sortie : Plan de salle, Visualisation plan (ORG-E16). |

#### ORG-E16 — Visualisation du plan

| Attribut | Description |
|----------|-------------|
| **Phase** | Plan de salle |
| **Objectif** | Visualiser le plan en lecture avec légende et export. |
| **Organisation** | Plan en lecture seule avec légende et noms exposants (si publiés). Boutons **Export PDF**, **Export image**. |
| **Besoins** | ORG-21, ORG-22. |
| **Navigation** | Entrée : Plan de salle (ORG-E14), Attribution (ORG-E15). Sortie : Plan de salle. |

---

### 2.5 Programme

#### ORG-E17a — Programme (vues chronologique / par salle)

| Attribut | Description |
|----------|-------------|
| **Phase** | Programme |
| **Objectif** | Consulter le programme en vue chronologique ou par salle ; filtrer (jour, scène, type). |
| **Organisation** | Titre « Programme — [Édition] ». Filtres : jour, scène/salle, type d’activité. Bascule **Vue chronologique** / **Vue par salle**. Liste ou grille : animations (nom, horaire, salle, durée). Actions : Ajouter animation, Modifier, Supprimer. Export PDF/CSV. |
| **Besoins** | ORG-23, ORG-25, ORG-26, ORG-27. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Création/édition animation (ORG-E17b). |

#### ORG-E17b — Création / édition d’une animation

| Attribut | Description |
|----------|-------------|
| **Phase** | Programme |
| **Objectif** | Ajouter ou modifier une animation (nom, type, durée, salle, horaire). |
| **Organisation** | Formulaire : nom, type, durée, salle/scène, date, créneau horaire. Alerte si chevauchement (même salle, même créneau). Boutons **Enregistrer**, **Annuler**. |
| **Besoins** | ORG-23, ORG-24, ORG-26. |
| **Navigation** | Entrée : Programme (ORG-E17a). Sortie : Programme. |

---

### 2.6 Budget

#### ORG-E19 — Saisie et ventilation budget

| Attribut | Description |
|----------|-------------|
| **Phase** | Budget |
| **Objectif** | Saisir les lignes de revenus et dépenses, ventiler par catégorie, consulter balance et statistiques. |
| **Organisation** | Titre « Budget — [Édition] ». Onglets : **Revenus**, **Dépenses**, **Ventilation**, **Balance**. Formulaire de saisie (date, libellé, catégorie, montant). Tableaux par catégorie, totaux. Graphiques répartition. Lien factures → revenus (ORG-31). Export PDF/Excel. |
| **Besoins** | ORG-28, ORG-29, ORG-30, ORG-31. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Dashboard édition. |

---

### 2.7 Documents et communication

#### ORG-E22 — Documents et légal (contrats types, CGV, règlements)

| Attribut | Description |
|----------|-------------|
| **Phase** | Documents |
| **Objectif** | Stocker et gérer les contrats types, CGV, conventions, règlements. |
| **Organisation** | Titre « Documents — [Édition] ». Liste des documents : nom, type, version, date. Actions : Upload, Télécharger, Associer à l’édition ou global. **Envoi aux exposants** : sélection document + exposants, envoi par email (lien ou pièce jointe). Historique des envois. |
| **Besoins** | ORG-32, ORG-33, ORG-34. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Dashboard édition. |

#### ORG-E23 — Annonces et notifications

| Attribut | Description |
|----------|-------------|
| **Phase** | Communication |
| **Objectif** | Diffuser des annonces (exposants, équipe) et paramétrer les notifications. |
| **Organisation** | Titre « Communication ». **Annonces** : rédaction, choix destinataires (exposants, équipe, rôle), envoi (Miyunotify). **Notifications ciblées** : par rôle, équipe, exposant. **Paramétrage** : activation par type (nouvelle candidature, paiement reçu, etc.), canaux (email, in-app). |
| **Besoins** | ORG-35, ORG-36, ORG-37. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Dashboard édition. |

---

### 2.8 Services visiteur et publication

#### ORG-E24 — Services visiteur (activation par édition)

| Attribut | Description |
|----------|-------------|
| **Phase** | Services visiteur |
| **Objectif** | Activer et paramétrer les services proposés aux visiteurs (jeux, concours, ateliers, réservations, pass). |
| **Organisation** | Titre « Services visiteurs — [Édition] ». Liste des services : Jeux, Concours, Inscriptions ateliers, Réservations, Pass VIP, Notifications. Pour chaque : case **Activé**, paramètres (places limitées, dates, règles). Bouton **Enregistrer**. |
| **Besoins** | ORG-38, ORG-39, ORG-40. |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Dashboard édition. |

#### ORG-E25 — Publication et clôture édition

| Attribut | Description |
|----------|-------------|
| **Phase** | Publication |
| **Objectif** | Publier l’édition au catalogue (annuaire événements) et clôturer l’édition. |
| **Organisation** | **Publication** : case « Publier au catalogue » ; choix visibilité (annuaire, répertoire organisateur). Bouton **Publier**. **Clôture** : bouton **Clôturer l’édition** (confirmation) ; export rapport/données avant clôture. Après clôture : données en lecture seule. |
| **Besoins** | ORG-11, ORG-27 (programme public). |
| **Navigation** | Entrée : Dashboard édition (ORG-E07). Sortie : Liste des éditions (ORG-E05). |

---

### 2.9 Compte et équipe

#### ORG-E20 — Mon compte

| Attribut | Description |
|----------|-------------|
| **Phase** | Compte |
| **Objectif** | Consulter et modifier le profil organisateur (structure, contact). |
| **Organisation** | Formulaire : nom structure, type, contact (email, téléphone, adresse), préférences. Boutons **Enregistrer**, **Changer mot de passe** (si applicable). |
| **Besoins** | ORG-01 (profil). |
| **Navigation** | Entrée : Tableau de bord (ORG-E04). Sortie : Tableau de bord. |

#### ORG-E21 — Équipe et invitations

| Attribut | Description |
|----------|-------------|
| **Phase** | Compte |
| **Objectif** | Gérer les membres de l’équipe (rôles Admin, Manager, Bénévole) et inviter de nouveaux membres. |
| **Organisation** | Liste des membres : nom, email, rôle, éditions assignées (pour Manager). Actions : Modifier rôle, Révoquer. Bouton **Inviter un membre** : email, rôle, éditions (si Manager). Envoi invitation (Miyunotify). |
| **Besoins** | ORG-05 (rattachement structure). |
| **Navigation** | Entrée : Tableau de bord (ORG-E04). Sortie : Tableau de bord. |

---

## 3. Récapitulatif des écrans et besoins

| Écran | Id | Phase | Besoins principaux |
|-------|-----|-------|--------------------|
| Landing (passerelle) | ORG-E01 | Accès | UNC-02, UNC-19 |
| Connexion | ORG-E02 | Accès | ORG-01 |
| Inscription organisateur | ORG-E03 | Accès | ORG-01, ORG-02, ORG-03 |
| Tableau de bord organisateur | ORG-E04 | Tableau de bord | ORG-04, ORG-07 |
| Liste des éditions | ORG-E05 | Tableau de bord | ORG-06, ORG-07, ORG-10 |
| Création édition | ORG-E06 | Édition | ORG-06, ORG-10 |
| Dashboard édition | ORG-E07 | Édition | ORG-08, ORG-09 |
| Paramétrage édition | ORG-E08 | Édition | ORG-09 |
| Liste exposants | ORG-E09 | Exposants | ORG-12, ORG-18 |
| Candidatures | ORG-E10 | Exposants | ORG-13, ORG-14 |
| Fiche exposant | ORG-E11 | Exposants | ORG-15, ORG-16, ORG-17 |
| Devis | ORG-E12 | Exposants | ORG-16 |
| Factures | ORG-E13 | Exposants | ORG-17 |
| Plan de salle (définition) | ORG-E14 | Plan de salle | ORG-19, ORG-22 |
| Attribution emplacements | ORG-E15 | Plan de salle | ORG-20 |
| Visualisation plan | ORG-E16 | Plan de salle | ORG-21, ORG-22 |
| Programme (vues) | ORG-E17a | Programme | ORG-23, ORG-25, ORG-26, ORG-27 |
| Création/édition animation | ORG-E17b | Programme | ORG-23, ORG-24, ORG-26 |
| Import exposants | ORG-E18 | Exposants | ORG-18 |
| Budget | ORG-E19 | Budget | ORG-28 à ORG-31 |
| Mon compte | ORG-E20 | Compte | ORG-01 |
| Équipe | ORG-E21 | Compte | ORG-05 |
| Documents et légal | ORG-E22 | Documents | ORG-32, ORG-33, ORG-34 |
| Annonces et notifications | ORG-E23 | Communication | ORG-35, ORG-36, ORG-37 |
| Services visiteur | ORG-E24 | Services visiteur | ORG-38, ORG-39, ORG-40 |
| Publication et clôture | ORG-E25 | Publication | ORG-11 |

---

## 4. Navigation type (flux principal)

```
Landing (UNC) → Connexion / Inscription → Tableau de bord organisateur
       → Liste des éditions → Dashboard édition
            → Exposants (liste, candidatures, fiche, devis, factures)
            → Plan de salle → Attribution
            → Programme → Création animation
            → Budget
            → Documents
            → Annonces / Notifications
            → Services visiteur
            → Publication / Clôture
       → Mon compte / Équipe
```

---

## 5. Références

- [Organisateurs — Parcours, capacités et livrables](./Organisateurs%20-%20Parcours%20Capacites%20Livrables.md)
- [Organisateurs — Analyse des besoins](./Organisateurs%20-%20Analyse%20des%20besoins.md)
- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
