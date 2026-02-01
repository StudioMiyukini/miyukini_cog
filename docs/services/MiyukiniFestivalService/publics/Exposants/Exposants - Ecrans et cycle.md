# Exposants — Écrans et cycle

## Contexte

Ce document précise **tous les écrans** du cycle utilisateur du public **Exposants** pour le service Miyukini Festival Service, avec l’**organisation** (structure, blocs, zones) et les **besoins** associés à chaque écran. Il s’appuie sur le [parcours et le dashboard](./Exposants%20-%20Parcours%20Capacites%20Dashboard.md) et sur l’[analyse des besoins](./Exposants%20-%20Analyse%20des%20besoins.md).

## Portée / Scope

- **Public** : Exposants (professionnels ou structures participant à des événements en tant qu’exposants).
- **Périmètre** : tous les écrans du cycle (de l’arrivée à la clôture), organisation et besoins par écran.
- **Hors périmètre** : maquettes graphiques, spécifications API.

---

## 1. Vue d’ensemble du cycle

Le cycle exposant se décompose en **phases** :

| Phase | Description | Écrans concernés |
|-------|-------------|-------------------|
| **Accès** | Arrivée depuis le catalogue ou URL directe ; connexion ou inscription. | Landing (catalogue), Connexion, Inscription exposant |
| **Dashboard** | Vue unifiée sur candidatures, participations, agenda, documents, factures. | Dashboard exposant (accueil), Liste candidatures, Liste participations |
| **Candidatures** | Découverte des événements, dépôt, suivi, modification/annulation. | Annuaire événements (candidatures ouvertes), Dépôt candidature, Fiche candidature |
| **Agenda** | Calendrier cross-événements, conflits de dates, export. | Agenda exposant |
| **Participations** | Fiche par édition validée, plan de salle, programme, documents. | Fiche participation (édition), Plan de salle (emplacement), Programme public |
| **Documents et facturation** | Documents reçus, envoi signés, devis/factures, acceptation devis. | Documents par édition, Envoi document signé, Devis et factures, Acceptation devis |
| **Compte et répertoire** | Profil, fiche entreprise, visibilité répertoire. | Mon compte, Fiche publique (répertoire) |
| **Notifications** | Réception, préférences, historique. | Notifications, Préférences, Historique communications |

---

## 2. Écrans du cycle — détail

### 2.1 Accès

#### EXP-E01 — Landing / Accueil catalogue (passerelle)

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Point d’entrée depuis le catalogue (utilisateur non connecté) ; afficher les CTAs « S’inscrire » et « Se connecter ». |
| **Organisation** | En-tête : logo, lien Événements / Organisateurs / Exposants, bouton **Se connecter**, bouton **S’inscrire** (ou menu : Organisateur / Exposant / Visiteur). Zone principale : accroche + lien vers **S’inscrire en tant qu’exposant**. Pied : liens légaux. |
| **Besoins** | UNC-02, UNC-19 (passerelle depuis [Utilisateur non connecté](../UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md)). |
| **Navigation** | Entrée : URL catalogue. Sortie : Connexion (EXP-E02), Inscription exposant (EXP-E03). |

#### EXP-E02 — Connexion

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Permettre à un exposant de se connecter (email + mot de passe ou lien magique). |
| **Organisation** | Titre « Se connecter ». Champs : email, mot de passe. Liens : « Mot de passe oublié », « S’inscrire ». Bouton **Se connecter**. Message d’erreur si échec. |
| **Besoins** | EXP-01, Miyauth. |
| **Navigation** | Entrée : Landing, lien direct. Sortie : Dashboard exposant (EXP-E04). |

#### EXP-E03 — Inscription exposant

| Attribut | Description |
|----------|-------------|
| **Phase** | Accès |
| **Objectif** | Création du compte exposant (fiche entreprise/contact, validation selon politique). |
| **Organisation** | Titre « Créer un compte exposant ». Bloc 1 : email, mot de passe, confirmation. Bloc 2 : nom entreprise, activité/secteur, contact (téléphone, adresse), site web (optionnel). CGU + case à cocher. Bouton **S’inscrire**. Lien « Déjà un compte ? Se connecter ». |
| **Besoins** | EXP-01, EXP-02, EXP-04. |
| **Navigation** | Entrée : Landing. Sortie : Confirmation / attente validation, ou Connexion si validation auto ; puis Dashboard (EXP-E04). |

---

### 2.2 Dashboard exposant

#### EXP-E04 — Dashboard exposant (accueil)

| Attribut | Description |
|----------|-------------|
| **Phase** | Dashboard |
| **Objectif** | Vue d’ensemble unifiée : candidatures, participations, agenda, documents, factures (tous festivals). |
| **Organisation** | En-tête : nom entreprise ou utilisateur, menu (Candidatures, Participations, Agenda, Documents, Factures, Mon compte, Déconnexion). Zone principale : **Bloc synthèse** (candidatures en attente, prochain événement, alertes : conflit dates, document à signer, facture à payer). **Blocs ou onglets** : Candidatures (raccourci liste), Participations (raccourci liste), Agenda (aperçu calendrier), Documents (derniers reçus), Factures (en attente / à payer). |
| **Besoins** | EXP-03, EXP-05, EXP-08. |
| **Navigation** | Entrée : après Connexion. Sortie : Liste candidatures (EXP-E05), Liste participations (EXP-E06), Agenda (EXP-E09), Documents (EXP-E12), Factures (EXP-E13), Mon compte (EXP-E17). |

#### EXP-E05 — Liste des candidatures

| Attribut | Description |
|----------|-------------|
| **Phase** | Candidatures |
| **Objectif** | Afficher toutes les candidatures (en attente, validées, refusées) par édition. |
| **Organisation** | Titre « Mes candidatures ». Filtres : statut (en attente, validée, refusée), édition, date. Tableau ou cartes : édition, dates, statut, date de dépôt. Actions : **Voir** (fiche candidature), **Modifier** / **Annuler** (si en attente et autorisé). Bouton **Déposer une candidature** (vers annuaire événements). |
| **Besoins** | EXP-06, EXP-12, EXP-13. |
| **Navigation** | Entrée : Dashboard (EXP-E04). Sortie : Fiche candidature (EXP-E07), Annuaire événements (EXP-E08), Dépôt candidature (EXP-E10). |

#### EXP-E06 — Liste des participations

| Attribut | Description |
|----------|-------------|
| **Phase** | Participations |
| **Objectif** | Afficher les éditions auxquelles l’exposant participe (validé) avec accès documents et facturation. |
| **Organisation** | Titre « Mes participations ». Liste ou cartes : édition (nom, dates, lieu), statut, emplacement (stand), liens **Documents**, **Factures**, **Plan de salle**, **Programme**. Clic → Fiche participation (EXP-E11). |
| **Besoins** | EXP-07, EXP-08. |
| **Navigation** | Entrée : Dashboard (EXP-E04). Sortie : Fiche participation (EXP-E11). |

---

### 2.3 Candidatures

#### EXP-E08 — Annuaire des événements (candidatures ouvertes)

| Attribut | Description |
|----------|-------------|
| **Phase** | Candidatures |
| **Objectif** | Consulter les événements ouverts aux candidatures pour choisir où candidater. |
| **Organisation** | Titre « Événements ouverts aux candidatures ». Filtres : date, lieu, thème, organisateur. Liste/carte : événement (nom, dates, lieu, thème, délai candidature). Lien **Candidater** par événement. Alerte si conflit de dates avec un événement déjà inscrit/candidat (EXP-15). |
| **Besoins** | EXP-09. |
| **Navigation** | Entrée : Liste candidatures (EXP-E05), Dashboard (EXP-E04). Sortie : Dépôt candidature (EXP-E10). |

#### EXP-E10 — Dépôt d’une candidature

| Attribut | Description |
|----------|-------------|
| **Phase** | Candidatures |
| **Objectif** | Remplir et envoyer le formulaire de candidature pour une édition (champs définis par l’organisateur, pièces jointes). |
| **Organisation** | Titre « Candidater — [Nom édition] ». Fil d’Ariane : Mes candidatures > [Édition]. Formulaire dynamique (champs selon organisateur) : coordonnées, activité, description, etc. Zone **Pièces jointes** : upload (fiche entreprise, logo, etc.). **Vérification agenda** : alerte si conflit de dates (EXP-11). Boutons **Prévisualiser**, **Envoyer**, **Annuler**. Accusé de réception après envoi. |
| **Besoins** | EXP-10, EXP-11, NFR-EXP-12. |
| **Navigation** | Entrée : Annuaire événements (EXP-E08), Liste candidatures (EXP-E05). Sortie : Liste candidatures, Fiche candidature (EXP-E07). |

#### EXP-E07 — Fiche candidature (détail et suivi)

| Attribut | Description |
|----------|-------------|
| **Phase** | Candidatures |
| **Objectif** | Consulter le détail d’une candidature (données envoyées, pièces, statut) ; modifier ou annuler si en attente. |
| **Organisation** | Titre « Candidature — [Nom édition] ». Blocs : **Résumé** (édition, dates, statut, date dépôt), **Données envoyées** (formulaire + pièces jointes, téléchargement), **Statut** (en attente / validée / refusée, motif si refus), **Notifications** (historique). Actions : **Modifier** / **Annuler** (si en attente et autorisé). |
| **Besoins** | EXP-12, EXP-13. |
| **Navigation** | Entrée : Liste candidatures (EXP-E05). Sortie : Liste candidatures, Dépôt candidature (modification), Fiche participation (si validée). |

---

### 2.4 Agenda

#### EXP-E09 — Agenda exposant (calendrier cross-événements)

| Attribut | Description |
|----------|-------------|
| **Phase** | Agenda |
| **Objectif** | Visualiser les dates des événements (candidat ou inscrit) ; éviter les conflits ; exporter ou partager. |
| **Organisation** | Titre « Mon agenda ». Vue calendrier (mois, semaine) : événements (candidat ou inscrit) avec couleur/libellé par statut. Lien vers fiche édition. **Export** : iCal, PDF. **Partage** : lien optionnel. Compte à rebours « Prochain événement » si pertinent. |
| **Besoins** | EXP-14, EXP-16. |
| **Navigation** | Entrée : Dashboard (EXP-E04). Sortie : Fiche participation (EXP-E11), Annuaire événements (EXP-E08). |

---

### 2.5 Participations (éditions validées)

#### EXP-E11 — Fiche participation (édition validée)

| Attribut | Description |
|----------|-------------|
| **Phase** | Participations |
| **Objectif** | Consulter la fiche d’une édition à laquelle l’exposant participe : résumé, documents, emplacement, programme, facturation. |
| **Organisation** | Titre « Participation — [Nom édition] ». Blocs : **Résumé** (dates, lieu, statut, emplacement attribué), **Documents** (liste, téléchargement), **Plan de salle** (lien EXP-E11b), **Programme public** (lien EXP-E11c), **Facturation** (lien EXP-E13). Actions : Télécharger document, Voir plan, Voir programme, Consulter factures. |
| **Besoins** | EXP-17. |
| **Navigation** | Entrée : Liste participations (EXP-E06), Agenda (EXP-E09). Sortie : Plan de salle (EXP-E11b), Programme (EXP-E11c), Documents (EXP-E12), Factures (EXP-E13). |

#### EXP-E11b — Plan de salle (emplacement)

| Attribut | Description |
|----------|-------------|
| **Phase** | Participations |
| **Objectif** | Consulter le plan de salle et son emplacement attribué (lecture seule). |
| **Organisation** | Titre « Plan de salle — [Nom édition] ». Vue plan (lecture seule) avec emplacement exposant mis en évidence. Légende. Bouton **Export** / **Imprimer** si autorisé. |
| **Besoins** | EXP-18. |
| **Navigation** | Entrée : Fiche participation (EXP-E11). Sortie : Fiche participation. |

#### EXP-E11c — Programme public (édition)

| Attribut | Description |
|----------|-------------|
| **Phase** | Participations |
| **Objectif** | Consulter le programme public de l’édition (animations, créneaux, salles). |
| **Organisation** | Titre « Programme — [Nom édition] ». Vue chronologique ou par salle (lecture seule). Filtres jour, type. |
| **Besoins** | EXP-19. |
| **Navigation** | Entrée : Fiche participation (EXP-E11). Sortie : Fiche participation. |

---

### 2.6 Documents et facturation

#### EXP-E12 — Documents par édition

| Attribut | Description |
|----------|-------------|
| **Phase** | Documents |
| **Objectif** | Consulter et télécharger les documents reçus (contrats, règlements, conventions) ; envoyer documents signés ou complétés. |
| **Organisation** | Titre « Documents — [Nom édition] » ou « Mes documents ». Liste des documents par édition : nom, type, date réception, statut (reçu, à signer, signé). Action **Télécharger**. Pour documents à signer/compléter : **Envoyer document signé** (upload ou formulaire) → écran EXP-E12b. |
| **Besoins** | EXP-20, EXP-21. |
| **Navigation** | Entrée : Dashboard (EXP-E04), Fiche participation (EXP-E11). Sortie : Envoi document signé (EXP-E12b). |

#### EXP-E12b — Envoi document signé ou complété

| Attribut | Description |
|----------|-------------|
| **Phase** | Documents |
| **Objectif** | Uploader un document signé ou compléter un formulaire et l’envoyer à l’organisateur. |
| **Organisation** | Titre « Envoyer document — [Nom document] ». Upload fichier ou formulaire à compléter. Boutons **Envoyer**, **Annuler**. Accusé de réception. |
| **Besoins** | EXP-21. |
| **Navigation** | Entrée : Documents (EXP-E12). Sortie : Documents (EXP-E12). |

#### EXP-E13 — Devis et factures

| Attribut | Description |
|----------|-------------|
| **Phase** | Facturation |
| **Objectif** | Consulter les devis et factures par édition ; télécharger PDF ; accepter/refuser un devis ; voir le statut de paiement. |
| **Organisation** | Titre « Devis et factures » (global ou par édition). Liste : numéro, date, montant, statut (devis : envoyé, accepté, refusé ; facture : payé, en attente). Détail : lignes, conditions, **Télécharger PDF**. Pour devis : boutons **Accepter** / **Refuser** (EXP-23). Date d’échéance et rappel si configuré (EXP-24). |
| **Besoins** | EXP-22, EXP-23, EXP-24. |
| **Navigation** | Entrée : Dashboard (EXP-E04), Fiche participation (EXP-E11). Sortie : Fiche participation. |

---

### 2.7 Compte et répertoire

#### EXP-E17 — Mon compte (profil et fiche entreprise)

| Attribut | Description |
|----------|-------------|
| **Phase** | Compte |
| **Objectif** | Consulter et mettre à jour le profil exposant et la fiche entreprise (nom, contact, activité, logo, site web). |
| **Organisation** | Titre « Mon compte ». Onglets : **Profil** (email, mot de passe), **Fiche entreprise** (nom, activité, contact, logo, site web, réseaux). Formulaire par onglet. Boutons **Enregistrer**, **Changer mot de passe**. |
| **Besoins** | EXP-04, EXP-26. |
| **Navigation** | Entrée : Dashboard (EXP-E04). Sortie : Dashboard. |

#### EXP-E18 — Fiche publique (répertoire des exposants)

| Attribut | Description |
|----------|-------------|
| **Phase** | Compte |
| **Objectif** | Prévisualiser ou gérer les informations affichées dans le répertoire des exposants (si politique plateforme le permet). |
| **Organisation** | Titre « Ma fiche publique ». Aperçu de la fiche telle qu’elle apparaît dans le répertoire. Édition des champs autorisés (nom, description, logo, site web). Option désactivation visibilité si proposée. Bouton **Enregistrer**. |
| **Besoins** | EXP-25, EXP-26. |
| **Navigation** | Entrée : Mon compte (EXP-E17) ou lien Dashboard. Sortie : Mon compte. |

---

### 2.8 Notifications

#### EXP-E19 — Notifications et préférences

| Attribut | Description |
|----------|-------------|
| **Phase** | Notifications |
| **Objectif** | Consulter les notifications reçues et paramétrer les préférences (types, canaux, par édition). |
| **Organisation** | **Liste des notifications** : date, objet, lien vers l’action (candidature, document, facture). **Préférences** : types (candidature validée/refusée, nouveau document, devis/facture, rappel paiement), canaux (email, in-app), par édition si pertinent. Bouton **Enregistrer**. **Historique des communications** : liste des messages/annonces par édition (EXP-28). |
| **Besoins** | EXP-27, EXP-28. |
| **Navigation** | Entrée : Dashboard (EXP-E04), lien en-tête (icône notifications). Sortie : Fiche candidature, Documents, Factures, etc. selon notification. |

---

## 3. Récapitulatif des écrans et besoins

| Écran | Id | Phase | Besoins principaux |
|-------|-----|-------|--------------------|
| Landing (passerelle) | EXP-E01 | Accès | UNC-02, UNC-19 |
| Connexion | EXP-E02 | Accès | EXP-01 |
| Inscription exposant | EXP-E03 | Accès | EXP-01, EXP-02, EXP-04 |
| Dashboard exposant | EXP-E04 | Dashboard | EXP-03, EXP-05, EXP-08 |
| Liste candidatures | EXP-E05 | Candidatures | EXP-06, EXP-12, EXP-13 |
| Liste participations | EXP-E06 | Participations | EXP-07, EXP-08 |
| Fiche candidature | EXP-E07 | Candidatures | EXP-12, EXP-13 |
| Annuaire événements (candidatures ouvertes) | EXP-E08 | Candidatures | EXP-09 |
| Agenda exposant | EXP-E09 | Agenda | EXP-14, EXP-16 |
| Dépôt candidature | EXP-E10 | Candidatures | EXP-10, EXP-11 |
| Fiche participation | EXP-E11 | Participations | EXP-17 |
| Plan de salle (emplacement) | EXP-E11b | Participations | EXP-18 |
| Programme public | EXP-E11c | Participations | EXP-19 |
| Documents par édition | EXP-E12 | Documents | EXP-20, EXP-21 |
| Envoi document signé | EXP-E12b | Documents | EXP-21 |
| Devis et factures | EXP-E13 | Facturation | EXP-22, EXP-23, EXP-24 |
| Mon compte | EXP-E17 | Compte | EXP-04, EXP-26 |
| Fiche publique (répertoire) | EXP-E18 | Compte | EXP-25, EXP-26 |
| Notifications et préférences | EXP-E19 | Notifications | EXP-27, EXP-28 |

---

## 4. Navigation type (flux principal)

```
Landing (UNC) → Connexion / Inscription → Dashboard exposant
       → Liste candidatures → Annuaire événements → Dépôt candidature → Fiche candidature
       → Liste participations → Fiche participation → Plan de salle / Programme / Documents / Factures
       → Agenda (calendrier cross-événements)
       → Documents (par édition) → Envoi document signé
       → Devis et factures (accepter devis, suivi paiement)
       → Mon compte / Fiche publique
       → Notifications / Préférences
```

---

## 5. Références

- [Exposants — Parcours, capacités et dashboard](./Exposants%20-%20Parcours%20Capacites%20Dashboard.md)
- [Exposants — Analyse des besoins](./Exposants%20-%20Analyse%20des%20besoins.md)
- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
