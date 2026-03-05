# Exposants â€” Analyse des besoins

## Contexte

Ce document constitue lâ€™**analyse des besoins** du public cible **Exposants** pour le service JayFestival. Il identifie lâ€™ensemble des besoins fonctionnels et non fonctionnels, les parcours dÃ©taillÃ©s, les user stories, les pain points et opportunitÃ©s, ainsi que la priorisation et les dÃ©pendances. Il sâ€™adresse aux Ã©quipes produit, conception et dÃ©veloppement.

**RÃ©fÃ©rences** : [Document fondateur](../../JayFestival%20-%20Document%20Fondateur.md), [Parcours, capacitÃ©s et dashboard](./Exposants%20-%20Parcours%20Capacites%20Dashboard.md), [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//..//..//miyukini-webway-system//reference//_index.md) (COG de rÃ©fÃ©rence, donnÃ©es exposant).

## PortÃ©e / Scope

- **Public** : Exposants (professionnels ou structures participant Ã  des Ã©vÃ©nements en tant quâ€™exposants).
- **PÃ©rimÃ¨tre** : tous les besoins identifiÃ©s pour ce public (fonctionnels, non fonctionnels, parcours, scÃ©narios, priorisation).
- **Hors pÃ©rimÃ¨tre** : spÃ©cifications techniques dâ€™implÃ©mentation (API, schÃ©mas de donnÃ©es dÃ©taillÃ©s), spÃ©cifications des autres publics (organisateurs, visiteurs, utilisateur non connectÃ©) â€” traitÃ©es dans leurs propres documents dâ€™analyse.

---

## 1. Profil du public et personas

### 1.1 DÃ©finition du public

Les **exposants** sont des professionnels, entreprises ou associations qui **participent Ã  des Ã©vÃ©nements/festivals** en tant quâ€™exposants (stand, prÃ©sence commerciale ou associative). Ils disposent dâ€™un **compte cross-Ã©vÃ©nements** : un mÃªme exposant peut **participer Ã  plusieurs festivals** depuis un seul espace. Ils ont accÃ¨s Ã  un **dashboard exposant dÃ©diÃ©** (candidatures, participations, agenda, documents, factures). La plateforme propose une **gestion dâ€™agenda** pour Ã©viter quâ€™un exposant sâ€™inscrive Ã  deux Ã©vÃ©nements Ã  la mÃªme date (conflits de dates â€” besoin dÃ©jÃ  vu en pratique).

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Artisan / crÃ©ateur** | Petit exposant ; participe Ã  2 Ã  5 festivals par an ; peu de temps pour lâ€™administratif. | Candidater facilement, suivre les statuts et les factures, Ã©viter les doublons de dates. | Multiples plateformes par festival, emails Ã©parpillÃ©s, risque de sâ€™inscrire Ã  deux festivals le mÃªme week-end. |
| **Entreprise / marque** | Exposant rÃ©gulier ; participe Ã  10 Ã  30 salons/festivals par an ; Ã©quipe dÃ©diÃ©e ou prestataire. | Un seul point dâ€™accÃ¨s pour toutes les participations, agenda consolidÃ©, visibilitÃ© dans un rÃ©pertoire. | Pas de vue consolidÃ©e, reporting manuel, difficultÃ© Ã  planifier sur plusieurs Ã©vÃ©nements. |
| **Association / collectif** | Exposant occasionnel ; 1 Ã  3 Ã©vÃ©nements par an ; bÃ©nÃ©voles. | Candidater, rÃ©cupÃ©rer les documents et rÃ¨glements, payer en temps utile. | Manque de clartÃ© sur les Ã©tapes, documents perdus, dÃ©lais de paiement oubliÃ©s. |
| **Exposant multi-festivals** | Exposant actif sur plusieurs Ã©vÃ©nements (saison, thÃ©matiques) ; besoin de cohÃ©rence. | Dashboard unifiÃ©, agenda cross-Ã©vÃ©nements, alerte conflits de dates, historique des participations. | Risque de double inscription Ã  la mÃªme date ; pas de calendrier global. |

### 1.3 Contexte dâ€™usage

- **FrÃ©quence** : connexion ponctuelle (candidature, suivi statut, documents, factures) ; plus rÃ©guliÃ¨re en phase de prÃ©paration dâ€™un Ã©vÃ©nement.
- **Appareils** : desktop et mobile (consultation dashboard, dÃ©pÃ´t candidature, tÃ©lÃ©chargement documents).
- **Concurrence** : emails et formulaires par festival ; attente dâ€™un **guichet unique** pour toutes les participations.

---

## 2. Besoins fonctionnels

### 2.1 Onboarding et compte

| Id | Besoin | Description | CritÃ¨res dâ€™acceptation |
|----|--------|-------------|-------------------------|
| EXP-01 | CrÃ©ation de compte exposant | Pouvoir sâ€™inscrire en tant quâ€™exposant (email, mot de passe ou lien magique, fiche entreprise/contact). | Formulaire dâ€™inscription dÃ©diÃ© ; validation email si configurÃ©e ; crÃ©ation du profil exposant (Miyauth, Miyuprofile, fiche entreprise). |
| EXP-02 | Validation du compte | Le compte peut Ãªtre validÃ© automatiquement ou selon politique plateforme/organisateur. | Workflow de validation configurable ; notification Ã  lâ€™exposant (validÃ© / en attente / refusÃ©). |
| EXP-03 | Compte cross-Ã©vÃ©nements | Un mÃªme exposant peut **participer Ã  plusieurs festivals** sans recrÃ©er de compte. | Dashboard agrÃ¨ge candidatures, participations, documents et factures pour **tous** les festivals concernÃ©s. |
| EXP-04 | Fiche entreprise / contact | Pouvoir complÃ©ter et mettre Ã  jour la fiche entreprise (nom, contact, activitÃ©, logo, site web). | Formulaire fiche exposant ; champs configurables ; mise Ã  jour Ã  tout moment ; fiche utilisÃ©e pour les candidatures et le rÃ©pertoire. |

### 2.2 Dashboard exposant

| Id | Besoin | Description | CritÃ¨res dâ€™acceptation |
|----|--------|-------------|-------------------------|
| EXP-05 | Vue dâ€™ensemble du dashboard | Avoir une vue unifiÃ©e sur candidatures, participations, agenda, documents, factures. | Page dâ€™accueil dashboard avec blocs ou onglets : Candidatures, Participations, Agenda, Documents, Factures ; indicateurs synthÃ©tiques (ex. candidatures en attente, prochain Ã©vÃ©nement). |
| EXP-06 | Liste des candidatures | Voir la liste de toutes les candidatures (en attente, validÃ©es, refusÃ©es) par Ã©dition. | Liste filtrable (statut : en attente, validÃ©e, refusÃ©e ; Ã©dition ; date) ; accÃ¨s au dÃ©tail et aux piÃ¨ces jointes ; tri et pagination. |
| EXP-07 | Liste des participations | Voir la liste des Ã©ditions auxquelles lâ€™exposant participe (validÃ©) avec accÃ¨s aux documents et facturation. | Liste des Ã©ditions validÃ©es ; fiche par Ã©dition (dates, lieu, statut, emplacement, lien documents/factures) ; accÃ¨s au programme public si mis Ã  disposition. |
| EXP-08 | AccÃ¨s rapide aux documents et factures | AccÃ©der aux documents et factures depuis le dashboard (par Ã©dition ou global). | Liens directs vers documents (contrats, rÃ¨glements) et factures (devis, factures) ; tÃ©lÃ©chargement PDF ; statut de paiement visible. |

### 2.3 Candidatures

| Id | Besoin | Description | CritÃ¨res dâ€™acceptation |
|----|--------|-------------|-------------------------|
| EXP-09 | DÃ©couverte des Ã©vÃ©nements ouverts aux candidatures | Consulter lâ€™annuaire des Ã©vÃ©nements pour identifier les festivals ouverts aux candidatures. | Liste/carte des Ã©vÃ©nements avec filtre Â« Candidatures ouvertes Â» ; fiche Ã©vÃ©nement (dates, lieu, thÃ¨me, dÃ©lai candidature) ; lien Â« Candidater Â». |
| EXP-10 | DÃ©pÃ´t dâ€™une candidature | DÃ©poser une candidature pour une Ã©dition en remplissant le formulaire et en joignant les piÃ¨ces demandÃ©es. | Formulaire de candidature par Ã©dition (champs dÃ©finis par lâ€™organisateur) ; upload de piÃ¨ces jointes (fiche entreprise, logo, etc.) ; prÃ©visualisation avant envoi ; envoi et accusÃ© de rÃ©ception. |
| EXP-11 | VÃ©rification agenda avant candidature | ÃŠtre alertÃ© ou bloquÃ© si la candidature concerne une date en conflit avec une autre Ã©dition (dÃ©jÃ  inscrit ou candidat). | VÃ©rification des dates Ã  la soumission ; alerte Â« Conflit de dates avec lâ€™Ã©vÃ©nement X Â» ou blocage ; suggestion de consulter lâ€™agenda. |
| EXP-12 | Suivi du statut de la candidature | Consulter le statut de chaque candidature (en attente, validÃ©e, refusÃ©e) et recevoir une notification en cas de changement. | Statut visible dans la liste et la fiche candidature ; notification (Miyunotify) Ã  la validation ou au refus ; motif de refus affichÃ© si communiquÃ© par lâ€™organisateur. |
| EXP-13 | Modification ou annulation de candidature | Modifier ou annuler une candidature en attente (selon rÃ¨gles de lâ€™Ã©dition). | Actions Â« Modifier Â» / Â« Annuler Â» si autorisÃ©es par lâ€™organisateur et si statut Â« En attente Â» ; confirmation ; notification Ã  lâ€™organisateur si configurÃ©. |

### 2.4 Gestion dâ€™agenda et conflits de dates

| Id | Besoin | Description | CritÃ¨res dâ€™acceptation |
|----|--------|-------------|-------------------------|
| EXP-14 | Calendrier cross-Ã©vÃ©nements | Visualiser les dates des Ã©vÃ©nements auxquels lâ€™exposant est inscrit ou candidat. | Vue calendrier (mois, semaine) avec les Ã©vÃ©nements (candidat ou inscrit) ; couleur ou libellÃ© par statut ; lien vers la fiche Ã©dition. |
| EXP-15 | Alerte conflits de dates | ÃŠtre alertÃ© ou bloquÃ© en cas de chevauchement de dates avant validation dâ€™une nouvelle candidature. | DÃ©tection des chevauchements (mÃªme week-end, mÃªme jour, selon rÃ¨gle) ; alerte Ã  la soumission de candidature ; message explicite (Â« Conflit avec Festival X Â»). |
| EXP-16 | Export ou partage dâ€™agenda | Exporter ou partager son agenda (calendrier) pour planification externe. | Export calendrier (iCal, PDF) ou lien de partage ; mise Ã  jour automatique si nouvelles participations. |

### 2.5 Participations (Ã©ditions validÃ©es)

| Id | Besoin | Description | CritÃ¨res dâ€™acceptation |
|----|--------|-------------|-------------------------|
| EXP-17 | Fiche par Ã©dition participÃ©e | Consulter la fiche de chaque Ã©dition Ã  laquelle lâ€™exposant participe (rÃ©sumÃ©, documents, emplacement, programme). | Fiche Ã©dition : dates, lieu, statut, emplacement attribuÃ© (stand/zone), lien plan de salle si exposÃ© ; accÃ¨s aux documents et Ã  la facturation. |
| EXP-18 | AccÃ¨s au plan de salle (emplacement) | Consulter le plan de salle et son emplacement attribuÃ© si mis Ã  disposition par lâ€™organisateur. | Vue plan de salle (lecture seule) avec emplacement mis en Ã©vidence ; lÃ©gende ; export ou impression si autorisÃ©. |
| EXP-19 | AccÃ¨s au programme public | Consulter le programme public de lâ€™Ã©dition si mis Ã  disposition. | Lien vers le programme public (animations, crÃ©neaux, salles) ; vue lecture seule. |

### 2.6 Documents et facturation

| Id | Besoin | Description | CritÃ¨res dâ€™acceptation |
|----|--------|-------------|-------------------------|
| EXP-20 | Consultation et tÃ©lÃ©chargement des documents | Consulter et tÃ©lÃ©charger les documents reÃ§us (contrats, rÃ¨glements, conventions) par Ã©dition. | Liste des documents par Ã©dition ; tÃ©lÃ©chargement PDF ou fichier ; statut (reÃ§u, Ã  signer, signÃ©) si applicable. |
| EXP-21 | Envoi de documents signÃ©s ou complÃ©tÃ©s | Envoyer des documents signÃ©s ou complÃ©tÃ©s selon le workflow organisateur. | Upload de document signÃ© ou formulaire Ã  complÃ©ter ; envoi Ã  lâ€™organisateur ; accusÃ© de rÃ©ception ; suivi (envoyÃ©, reÃ§u). |
| EXP-22 | Consultation des devis et factures | Consulter les devis et factures (Miyuinvoice) par Ã©dition ; tÃ©lÃ©charger le PDF. | Liste des devis et factures par Ã©dition ; dÃ©tail (lignes, montants, conditions) ; tÃ©lÃ©chargement PDF ; statut (devis : envoyÃ©, acceptÃ©, refusÃ© ; facture : payÃ©, en attente). |
| EXP-23 | Acceptation dâ€™un devis | Accepter ou refuser un devis reÃ§u (si workflow organisateur le prÃ©voit). | Action Â« Accepter Â» / Â« Refuser Â» sur le devis ; notification Ã  lâ€™organisateur ; mise Ã  jour du statut ; conversion en facture cÃ´tÃ© organisateur si acceptÃ©. |
| EXP-24 | Suivi du statut de paiement | Voir le statut de paiement des factures (payÃ© / en attente) et les Ã©chÃ©ances. | Statut visible sur chaque facture ; date dâ€™Ã©chÃ©ance ; rappel ou alerte si configurÃ© (Miyunotify). |

### 2.7 RÃ©pertoire des exposants

| Id | Besoin | Description | CritÃ¨res dâ€™acceptation |
|----|--------|-------------|-------------------------|
| EXP-25 | VisibilitÃ© dans le rÃ©pertoire | La fiche exposant peut apparaÃ®tre dans le rÃ©pertoire des exposants du catalogue (selon politique plateforme et choix organisateur). | Fiche exposant publiÃ©e dans le rÃ©pertoire (entreprise, contact, Ã©ditions participÃ©es, etc.) ; visible par [utilisateur non connectÃ©](../UtilisateurNonConnecte/_index.md) et tous les publics ; option de dÃ©sactivation si proposÃ©e. |
| EXP-26 | Mise Ã  jour de la fiche publique | Pouvoir mettre Ã  jour les informations de la fiche exposant affichÃ©e dans le rÃ©pertoire. | Ã‰dition des champs autorisÃ©s (nom, description, logo, site web, rÃ©seaux) ; mise Ã  jour reflÃ©tÃ©e dans le rÃ©pertoire ; modÃ©ration selon politique plateforme si applicable. |

### 2.8 Notifications et communication

| Id | Besoin | Description | CritÃ¨res dâ€™acceptation |
|----|--------|-------------|-------------------------|
| EXP-27 | RÃ©ception des notifications | Recevoir des notifications (candidature validÃ©e/refusÃ©e, nouveau document, devis/facture envoyÃ©, rappel paiement). | Notifications (Miyunotify) par email et/ou in-app ; prÃ©fÃ©rences de notification configurables (par type, par Ã©dition). |
| EXP-28 | Historique des communications | Consulter lâ€™historique des communications reÃ§ues (annonces, documents envoyÃ©s) par Ã©dition. | Liste des notifications/messages par Ã©dition ; date, objet, lien vers le document ou lâ€™action. |

---

## 3. Besoins non fonctionnels

### 3.1 Performance

| Id | Besoin | CritÃ¨res dâ€™acceptation |
|----|--------|-------------------------|
| NFR-EXP-01 | Temps de chargement du dashboard | Le dashboard exposant se charge en moins de 3 secondes (rÃ©seau standard). |
| NFR-EXP-02 | Temps de soumission dâ€™une candidature | La soumission dâ€™une candidature (formulaire + piÃ¨ces jointes) sâ€™effectue en moins de 5 secondes aprÃ¨s clic Â« Envoyer Â». |
| NFR-EXP-03 | TÃ©lÃ©chargement de documents | Le tÃ©lÃ©chargement dâ€™un document (PDF) sâ€™effectue en moins de 5 secondes pour des fichiers < 5 Mo. |

### 3.2 DisponibilitÃ© et fiabilitÃ©

| Id | Besoin | CritÃ¨res dâ€™acceptation |
|----|--------|-------------------------|
| NFR-EXP-04 | DisponibilitÃ© | Le service est disponible 99,5 % du temps (hors fenÃªtres de maintenance annoncÃ©es). |
| NFR-EXP-05 | Sauvegarde des donnÃ©es | Les donnÃ©es (fiche, candidatures, documents reÃ§us) sont sauvegardÃ©es et rÃ©cupÃ©rables ; pas de perte Ã  la soumission dâ€™une candidature. |

### 3.3 SÃ©curitÃ© et gouvernance

| Id | Besoin | CritÃ¨res dâ€™acceptation |
|----|--------|-------------------------|
| NFR-EXP-06 | Authentification | Authentification sÃ©curisÃ©e (Miyauth) ; mot de passe ou lien magique ; session avec expiration. |
| NFR-EXP-07 | Isolation des donnÃ©es | Les donnÃ©es exposant (fiche, candidatures, factures) ne sont accessibles quâ€™Ã  lâ€™exposant et aux organisateurs des Ã©ditions concernÃ©es ; pas dâ€™accÃ¨s aux donnÃ©es des autres exposants. |
| NFR-EXP-08 | ConfidentialitÃ© des documents | Les documents et factures ne sont accessibles qu'Ã  l'exposant et Ã  l'organisateur de l'Ã©dition concernÃ©e. |
| NFR-EXP-09 | RÃ©sidence centralisÃ©e des donnÃ©es sensibles | Les donnÃ©es exposant (fiche, candidatures, documents, facturation) sont Ã  **rÃ©sidence centralisÃ©e** : la copie canonique rÃ©side sur le **COG de rÃ©fÃ©rence** (COG organisateur ou COG du Service). En cas de coupure du terminal exposant, les donnÃ©es restent disponibles pour les organisateurs. Le terminal exposant n'en dÃ©tient pas la seule copie (voir [Politique Residence Donnees Sensibles](..//..//..//..//miyukini-webway-system//reference//_index.md)). | et factures ne sont accessibles quâ€™Ã  lâ€™exposant et Ã  lâ€™organisateur de lâ€™Ã©dition concernÃ©e. |

### 3.4 UtilisabilitÃ© et accessibilitÃ©

| Id | Besoin | CritÃ¨res dâ€™acceptation |
|----|--------|-------------------------|
| NFR-EXP-09 | UtilisabilitÃ© | Les parcours principaux (dÃ©pÃ´t candidature, consultation statut, tÃ©lÃ©chargement facture) sont rÃ©alisables en moins de 5 clics depuis le dashboard. |
| NFR-EXP-10 | AccessibilitÃ© | ConformitÃ© WCAG 2.1 niveau AA pour le dashboard exposant (navigation clavier, lecteurs dâ€™Ã©cran, contrastes). |
| NFR-EXP-11 | Responsive | Le dashboard et le formulaire de candidature sont utilisables sur mobile (consultation, dÃ©pÃ´t candidature, tÃ©lÃ©chargement). |

### 3.5 Gestion dâ€™agenda

| Id | Besoin | CritÃ¨res dâ€™acceptation |
|----|--------|-------------------------|
| NFR-EXP-12 | DÃ©tection des conflits de dates | La plateforme dÃ©tecte un conflit de dates (mÃªme jour ou chevauchement selon rÃ¨gle) avant validation dâ€™une nouvelle candidature et alerte ou bloque. |
| NFR-EXP-13 | PrÃ©cision des dates | Les dates des Ã©vÃ©nements (dÃ©but, fin) sont affichÃ©es avec prÃ©cision (jour, heure si pertinent) pour permettre la planification. |

---

## 4. Parcours dÃ©taillÃ©s et scÃ©narios

### 4.1 ScÃ©nario : Premier usage â€” crÃ©ation de compte et premiÃ¨re candidature

1. Lâ€™utilisateur dÃ©couvre un Ã©vÃ©nement depuis le catalogue (annuaire des Ã©vÃ©nements) en [utilisateur non connectÃ©](../UtilisateurNonConnecte/_index.md).
2. Il clique sur Â« Candidater Â» ; il est redirigÃ© vers lâ€™inscription ou la connexion.
3. Il crÃ©e un compte exposant (email, mot de passe, fiche entreprise : nom, contact, activitÃ©, logo).
4. AprÃ¨s validation (automatique ou manuelle), il accÃ¨de au dashboard exposant (vide).
5. Il retourne sur la fiche de lâ€™Ã©vÃ©nement et dÃ©pose sa candidature (formulaire dÃ©fini par lâ€™organisateur, piÃ¨ces jointes).
6. Ã€ la soumission, la plateforme vÃ©rifie lâ€™agenda : pas de conflit de dates ; la candidature est enregistrÃ©e en Â« En attente Â».
7. Lâ€™organisateur reÃ§oit la candidature ; lâ€™exposant reÃ§oit un accusÃ© de rÃ©ception.
8. Lâ€™organisateur valide la candidature ; lâ€™exposant reÃ§oit une notification et voit le statut Â« ValidÃ©e Â» dans son dashboard.
9. Lâ€™exposant accÃ¨de Ã  la fiche de lâ€™Ã©dition (documents, emplacement quand attribuÃ©, facturation).

**Besoins couverts** : EXP-01 Ã  EXP-05, EXP-09 Ã  EXP-12, EXP-17, EXP-20, EXP-27.

### 4.2 ScÃ©nario : Conflit de dates â€” alerte Ã  la candidature

1. Lâ€™exposant est dÃ©jÃ  inscrit Ã  Â« Festival A Â» (dates 15-16 juin).
2. Il consulte lâ€™annuaire et souhaite candidater Ã  Â« Festival B Â» (dates 15-17 juin).
3. Il remplit le formulaire de candidature pour Festival B et clique sur Â« Envoyer Â».
4. La plateforme dÃ©tecte un chevauchement de dates avec Festival A.
5. Un message sâ€™affiche : Â« Conflit de dates : vous Ãªtes dÃ©jÃ  inscrit Ã  Festival A (15-16 juin). Festival B a lieu le 15-17 juin. Souhaitez-vous tout de mÃªme soumettre votre candidature ? Â» (ou blocage selon rÃ¨gle).
6. Lâ€™exposant peut annuler ou confirmer (avec avertissement) ; sâ€™il confirme, la candidature est envoyÃ©e mais lâ€™organisateur peut Ãªtre informÃ© du conflit.

**Besoins couverts** : EXP-11, EXP-14, EXP-15, NFR-EXP-12.

### 4.3 ScÃ©nario : Multi-festivals â€” vue consolidÃ©e et facturation

1. Lâ€™exposant participe Ã  3 festivals (validÃ©) et a 2 candidatures en attente.
2. Il se connecte et accÃ¨de au dashboard ; il voit les blocs Candidatures (2 en attente), Participations (3 Ã©ditions), Agenda (calendrier des 5 Ã©vÃ©nements), Documents, Factures.
3. Il clique sur Â« Participations Â» et voit la liste des 3 Ã©ditions avec pour chacune : dates, lieu, emplacement, lien documents, lien factures.
4. Il ouvre la facture dâ€™une Ã©dition : statut Â« En attente Â», Ã©chÃ©ance 30/04 ; il tÃ©lÃ©charge le PDF et procÃ¨de au paiement hors plateforme.
5. Il consulte lâ€™agenda : vue calendrier avec les 5 Ã©vÃ©nements (2 en attente, 3 validÃ©s) ; pas de chevauchement.
6. Il dÃ©pose une nouvelle candidature pour un 4e festival ; la plateforme vÃ©rifie lâ€™agenda : pas de conflit ; candidature enregistrÃ©e.

**Besoins couverts** : EXP-05 Ã  EXP-08, EXP-14, EXP-17, EXP-20, EXP-22, EXP-24.

### 4.4 ScÃ©nario : Documents et facturation

1. Lâ€™organisateur envoie un contrat type et un rÃ¨glement Ã  lâ€™exposant (par Ã©dition).
2. Lâ€™exposant reÃ§oit une notification ; il accÃ¨de au dashboard, onglet Documents, et voit les nouveaux documents pour lâ€™Ã©dition X.
3. Il tÃ©lÃ©charge le contrat et le rÃ¨glement, les signe, et les renvoie via lâ€™interface (upload ou formulaire).
4. Lâ€™organisateur reÃ§oit les documents signÃ©s ; il gÃ©nÃ¨re un devis et lâ€™envoie Ã  lâ€™exposant.
5. Lâ€™exposant reÃ§oit une notification ; il consulte le devis dans lâ€™onglet Factures, lâ€™accepte.
6. Lâ€™organisateur convertit le devis en facture ; lâ€™exposant voit la facture (statut Â« En attente Â»), tÃ©lÃ©charge le PDF, paie.
7. Lâ€™organisateur marque la facture comme payÃ©e (ou synchronisation paiement si intÃ©grÃ©) ; lâ€™exposant voit le statut Â« PayÃ© Â».

**Besoins couverts** : EXP-20 Ã  EXP-24, EXP-27.

---

## 5. Pain points et opportunitÃ©s

### 5.1 Pain points

| Pain point | Impact | Besoin associÃ© |
|------------|--------|-----------------|
| **Multiples plateformes par festival** | Un outil diffÃ©rent par Ã©vÃ©nement ; identifiants et processus multiples. | Un seul compte cross-Ã©vÃ©nements et un seul dashboard (EXP-03, EXP-05). |
| **Emails Ã©parpillÃ©s** | Documents et factures reÃ§us par email ; risque de perte, pas de vue consolidÃ©e. | Dashboard avec documents et factures par Ã©dition (EXP-08, EXP-20 Ã  EXP-24). |
| **Risque de double inscription Ã  la mÃªme date** | Sâ€™inscrire Ã  deux festivals le mÃªme week-end ; conflit opÃ©rationnel. | Gestion dâ€™agenda et alerte conflits de dates (EXP-14, EXP-15, NFR-EXP-12). |
| **Manque de clartÃ© sur les Ã©tapes** | Ne pas savoir oÃ¹ en est la candidature, le devis, la facture. | Suivi du statut (EXP-12, EXP-22, EXP-24) et notifications (EXP-27). |
| **Documents perdus** | Contrats et rÃ¨glements Ã©parpillÃ©s. | Centralisation des documents par Ã©dition dans le dashboard (EXP-20, EXP-21). |
| **DÃ©lais de paiement oubliÃ©s** | Oubli dâ€™Ã©chÃ©ance de facture. | Suivi du statut de paiement et rappels (EXP-24, EXP-27). |

### 5.2 OpportunitÃ©s

| OpportunitÃ© | Description | Besoin associÃ© |
|-------------|-------------|-----------------|
| **Vue consolidÃ©e multi-festivals** | Un seul Ã©cran pour toutes les participations et candidatures. | Dashboard unifiÃ© (EXP-05 Ã  EXP-08). |
| **Agenda cross-Ã©vÃ©nements** | Calendrier global pour planifier sans conflit. | Calendrier et alerte conflits (EXP-14, EXP-15). |
| **VisibilitÃ© rÃ©pertoire** | ÃŠtre visible dans le rÃ©pertoire des exposants pour les visiteurs et les organisateurs. | EXP-25, EXP-26. |
| **RÃ©duction de la charge administrative** | Moins dâ€™emails, tout au mÃªme endroit. | Dashboard, documents, factures centralisÃ©s (EXP-05 Ã  EXP-08, EXP-20 Ã  EXP-24). |

---

## 6. Priorisation des besoins (MoSCoW)

### 6.1 Must have (indispensable)

- EXP-01 Ã  EXP-05 (onboarding, compte cross-Ã©vÃ©nements, fiche entreprise, vue dashboard).
- EXP-09 Ã  EXP-13 (candidatures : dÃ©couverte, dÃ©pÃ´t, vÃ©rification agenda, suivi statut, modification/annulation).
- EXP-14, EXP-15 (calendrier cross-Ã©vÃ©nements, alerte conflits de dates).
- EXP-17 (fiche par Ã©dition participÃ©e).
- EXP-20 Ã  EXP-24 (documents, devis, factures, acceptation devis, suivi paiement).
- EXP-27 (notifications).
- NFR-EXP-06 Ã  NFR-EXP-08 (authentification, isolation, confidentialitÃ©).
- NFR-EXP-12 (dÃ©tection conflits de dates).

### 6.2 Should have (important)

- EXP-06 Ã  EXP-08 (liste candidatures, liste participations, accÃ¨s rapide documents/factures).
- EXP-16 (export ou partage agenda).
- EXP-18, EXP-19 (plan de salle, programme public).
- EXP-25, EXP-26 (rÃ©pertoire des exposants, mise Ã  jour fiche publique).
- EXP-28 (historique des communications).
- NFR-EXP-01 Ã  NFR-EXP-05, NFR-EXP-09 Ã  NFR-EXP-11, NFR-EXP-13 (performance, dispo, utilisabilitÃ©, accessibilitÃ©, responsive, prÃ©cision dates).

### 6.3 Could have (souhaitable)

- AmÃ©lioration des prÃ©fÃ©rences de notification (granularitÃ© par type, par Ã©dition).
- Export des donnÃ©es (candidatures, participations, factures) pour comptabilitÃ©.

### 6.4 Wonâ€™t have (hors pÃ©rimÃ¨tre ou report)

- Paiement en ligne intÃ©grÃ© (si hors pÃ©rimÃ¨tre v1) ; suivi du statut de paiement reste manuel cÃ´tÃ© organisateur.
- Besoins spÃ©cifiques aux autres publics â€” traitÃ©s dans leurs documents.

---

## 7. DÃ©pendances et interfaces avec les autres publics

### 7.1 DÃ©pendances

| DÃ©pendance | Description |
|------------|-------------|
| **Organisateurs** | Les candidatures sont traitÃ©es par les organisateurs (validation, refus) ; les devis et factures sont Ã©mis par les organisateurs (Miyuinvoice) ; les documents sont envoyÃ©s par les organisateurs. |
| **Catalogue** | Lâ€™annuaire des Ã©vÃ©nements (catalogue) permet Ã  lâ€™exposant de dÃ©couvrir les Ã©vÃ©nements ouverts aux candidatures ; le rÃ©pertoire des exposants peut afficher la fiche de lâ€™exposant (EXP-25). |
| **Plateforme** | Authentification (Miyauth), permissions (Master Butler), persistance (KindMother), agenda cross-Ã©vÃ©nements (MiyuClock, Miyubooking, donnÃ©es dâ€™Ã©dition). |

### 7.2 Interfaces

| Interface | Flux | Besoin exposant |
|-----------|------|------------------|
| Exposant â†’ Organisateur | DÃ©pÃ´t candidature, acceptation devis, envoi documents signÃ©s. | EXP-10, EXP-21, EXP-23. |
| Organisateur â†’ Exposant | Validation/refus candidature, envoi documents, envoi devis/facture. | EXP-12, EXP-20, EXP-22, EXP-27. |
| Exposant â†’ Catalogue | Consultation annuaire des Ã©vÃ©nements ; visibilitÃ© dans le rÃ©pertoire des exposants. | EXP-09, EXP-25, EXP-26. |

---

## 8. User stories (format standard)

### 8.1 Onboarding et dashboard

- **US-EXP-01** â€” En tant quâ€™**exposant**, je veux **crÃ©er un compte** (email, mot de passe, fiche entreprise) **afin de** candidater Ã  des Ã©vÃ©nements et gÃ©rer mes participations.  
  *CritÃ¨res* : Formulaire dÃ©diÃ© ; fiche entreprise (nom, contact, activitÃ©, logo) ; crÃ©ation profil (Miyauth, Miyuprofile).*

- **US-EXP-02** â€” En tant quâ€™**exposant**, je veux **accÃ©der Ã  un dashboard unique** avec toutes mes candidatures, participations, documents et factures **afin de** tout retrouver au mÃªme endroit.  
  *CritÃ¨res* : Vue dâ€™ensemble ; blocs Candidatures, Participations, Agenda, Documents, Factures ; accÃ¨s en moins de 3 clics aux Ã©lÃ©ments principaux.*

### 8.2 Candidatures et agenda

- **US-EXP-03** â€” En tant quâ€™**exposant**, je veux **dÃ©poser une candidature** pour un Ã©vÃ©nement (formulaire, piÃ¨ces jointes) **afin de** participer au festival.  
  *CritÃ¨res* : Formulaire par Ã©dition ; upload piÃ¨ces ; vÃ©rification agenda (conflit de dates) ; accusÃ© de rÃ©ception.*

- **US-EXP-04** â€” En tant quâ€™**exposant**, je veux **Ãªtre alertÃ© en cas de conflit de dates** avant de valider une candidature **afin de** ne pas mâ€™inscrire Ã  deux Ã©vÃ©nements Ã  la mÃªme date.  
  *CritÃ¨res* : DÃ©tection chevauchement ; message explicite ; alerte ou blocage selon rÃ¨gle.*

- **US-EXP-05** â€” En tant quâ€™**exposant**, je veux **voir le statut de mes candidatures** (en attente, validÃ©e, refusÃ©e) et **recevoir une notification** en cas de changement **afin de** suivre lâ€™avancement.  
  *CritÃ¨res* : Statut visible dans la liste et la fiche ; notification Ã  la validation/refus ; motif de refus si communiquÃ©.*

### 8.3 Documents et facturation

- **US-EXP-06** â€” En tant quâ€™**exposant**, je veux **consulter et tÃ©lÃ©charger** les documents (contrats, rÃ¨glements) et les factures **afin de** les archiver et payer en temps utile.  
  *CritÃ¨res* : Liste par Ã©dition ; tÃ©lÃ©chargement PDF ; statut de paiement visible.*

- **US-EXP-07** â€” En tant quâ€™**exposant**, je veux **accepter un devis** reÃ§u **afin de** confirmer ma participation et dÃ©clencher lâ€™Ã©mission de la facture.  
  *CritÃ¨res* : Action Accepter/Refuser ; notification Ã  lâ€™organisateur ; mise Ã  jour statut.*

### 8.4 VisibilitÃ©

- **US-EXP-08** â€” En tant quâ€™**exposant**, je veux **apparaÃ®tre dans le rÃ©pertoire des exposants** (fiche entreprise, Ã©ditions participÃ©es) **afin de** Ãªtre visible par les visiteurs et les organisateurs.  
  *CritÃ¨res* : Fiche publiÃ©e selon politique plateforme ; mise Ã  jour par lâ€™exposant ; option dÃ©sactivation si proposÃ©e.*

---

## 9. Cas limites et rÃ¨gles mÃ©tier

### 9.1 RÃ¨gles mÃ©tier

| RÃ¨gle | Description |
|-------|-------------|
| **Candidature** | Lâ€™exposant ne peut pas modifier les paramÃ¨tres des Ã©ditions ; il dÃ©pose une candidature et attend la dÃ©cision de lâ€™organisateur (StrongFather, validation). |
| **Agenda** | La plateforme signale ou bloque les conflits de dates ; lâ€™exposant peut toutefois confirmer sa candidature malgrÃ© un conflit (selon rÃ¨gle) ; lâ€™organisateur peut Ãªtre informÃ©. |
| **Documents et factures** | Les documents et factures sont Ã©mis par lâ€™organisateur ; lâ€™exposant consulte, tÃ©lÃ©charge et renvoie les documents signÃ©s ; le paiement peut Ãªtre hors plateforme (suivi du statut par lâ€™organisateur). |
| **RÃ©pertoire** | La fiche exposant peut Ãªtre publiÃ©e dans le rÃ©pertoire selon la politique plateforme et les choix de lâ€™organisateur ; lâ€™exposant peut mettre Ã  jour les champs autorisÃ©s. |

### 9.2 Cas limites

| Cas | Comportement attendu |
|-----|----------------------|
| **Candidature sur une Ã©dition clÃ´turÃ©e** | Impossible : les candidatures sont fermÃ©es pour les Ã©ditions clÃ´turÃ©es. |
| **Candidature sur une Ã©dition dont les dates chevauchent une autre Ã©dition (dÃ©jÃ  inscrit)** | Alerte ou blocage Ã  la soumission ; message explicite avec le nom de lâ€™Ã©vÃ©nement en conflit. |
| **Modification de candidature aprÃ¨s validation** | Impossible : une candidature validÃ©e ne peut pas Ãªtre modifiÃ©e par lâ€™exposant ; contacter lâ€™organisateur. |
| **Suppression de compte exposant avec participations en cours** | Blocage ou processus spÃ©cifique : les donnÃ©es de participations et facturation doivent Ãªtre conservÃ©es pour lâ€™organisateur ; proposer la dÃ©sactivation du compte et lâ€™archivage des donnÃ©es. |
| **Devis expirÃ©** | Si lâ€™organisateur a dÃ©fini une date dâ€™expiration au devis, lâ€™exposant ne peut plus accepter aprÃ¨s cette date ; message Â« Devis expirÃ© Â». |

### 9.3 MÃ©triques de succÃ¨s

| MÃ©trique | Description | Cible (exemple) |
|----------|-------------|------------------|
| **Taux dâ€™activation** | % dâ€™exposants ayant dÃ©posÃ© au moins une candidature aprÃ¨s inscription. | > 70 % |
| **Taux de conflits de dates Ã©vitÃ©s** | % de candidatures oÃ¹ une alerte conflit a Ã©tÃ© affichÃ©e et lâ€™exposant a annulÃ© ou modifiÃ©. | Suivi |
| **Satisfaction exposant** | Score NPS ou enquÃªte (facilitÃ©, clartÃ©, gain de temps). | Suivi annuel |
| **Nombre de participations par exposant** | Moyenne et mÃ©diane du nombre dâ€™Ã©ditions (candidatures + participations) par exposant. | Suivi ; objectif croissance |

---

## 10. CritÃ¨res dâ€™acceptation dÃ©taillÃ©s (sÃ©lection)

### 10.1 Candidature (EXP-10, EXP-11)

- **Formulaire** : Champs dÃ©finis par lâ€™organisateur (obligatoires et optionnels) ; validation cÃ´tÃ© client (format email, champs requis) ; sauvegarde brouillon si proposÃ©.
- **PiÃ¨ces jointes** : Types de fichiers autorisÃ©s (PDF, images) ; taille max par fichier et globale ; prÃ©visualisation avant envoi.
- **VÃ©rification agenda** : Ã€ la soumission, comparaison des dates de lâ€™Ã©dition avec les dates des Ã©vÃ©nements auxquels lâ€™exposant est dÃ©jÃ  inscrit ou candidat ; rÃ¨gle de chevauchement (mÃªme jour, mÃªme week-end, chevauchement partiel) configurable ; message dâ€™alerte avec nom de lâ€™Ã©vÃ©nement en conflit.
- **AccusÃ© de rÃ©ception** : Email et/ou notification in-app confirmant lâ€™enregistrement de la candidature ; numÃ©ro ou identifiant de candidature pour suivi.

### 10.2 Dashboard (EXP-05 Ã  EXP-08)

- **Vue dâ€™ensemble** : Blocs ou onglets Candidatures, Participations, Agenda, Documents, Factures ; indicateurs synthÃ©tiques (ex. Â« 2 candidatures en attente Â», Â« Prochain Ã©vÃ©nement : Festival X, 15-16 juin Â»).
- **Liste des candidatures** : Colonnes : Ã‰dition, Date dÃ©pÃ´t, Statut, Date mise Ã  jour ; tri par date, statut ; filtre par statut (en attente, validÃ©e, refusÃ©e) ; lien vers fiche dÃ©tail et piÃ¨ces jointes.
- **Liste des participations** : Colonnes : Ã‰dition, Dates, Lieu, Statut, Emplacement ; lien vers fiche Ã©dition (documents, plan, programme, factures).
- **Agenda** : Vue calendrier (mois, semaine) ; Ã©vÃ©nements (candidat ou inscrit) affichÃ©s avec libellÃ© et statut ; lien vers fiche Ã©dition.

### 10.3 Documents et facturation (EXP-20 Ã  EXP-24)

- **Documents** : Liste par Ã©dition ; colonnes : Document, Date envoi, Statut (reÃ§u, Ã  signer, signÃ©) ; bouton TÃ©lÃ©charger (PDF).
- **Factures** : Liste par Ã©dition ; colonnes : NumÃ©ro, Date, Montant, Statut (devis envoyÃ©, acceptÃ©, refusÃ© ; facture en attente, payÃ©e), Ã‰chÃ©ance ; bouton TÃ©lÃ©charger PDF ; action Accepter/Refuser sur devis si workflow activÃ©.
- **Suivi paiement** : Statut Â« PayÃ© Â» ou Â« En attente Â» ; date de paiement si renseignÃ©e par lâ€™organisateur ; rappel ou alerte si Ã©chÃ©ance proche (configurable).

---

## 11. Glossaire et rÃ©fÃ©rences

### 11.1 Glossaire (extrait)

| Terme | DÃ©finition |
|-------|------------|
| **Candidature** | Demande de participation dâ€™un exposant Ã  une Ã©dition ; statuts : en attente, validÃ©e, refusÃ©e. |
| **Dashboard exposant** | Espace dÃ©diÃ© Ã  lâ€™exposant : vue unifiÃ©e sur candidatures, participations, agenda, documents, factures. |
| **Conflit de dates** | Chevauchement des dates dâ€™une Ã©dition avec une autre Ã©dition Ã  laquelle lâ€™exposant est dÃ©jÃ  inscrit ou candidat. |
| **RÃ©pertoire des exposants** | Annuaire du catalogue listant les exposants (fiche entreprise, Ã©ditions participÃ©es) ; visible par le public. |

### 11.2 RÃ©fÃ©rences

- [Document fondateur JayFestival](../../JayFestival%20-%20Document%20Fondateur.md)
- [Exposants â€” Parcours, capacitÃ©s et dashboard](./Exposants%20-%20Parcours%20Capacites%20Dashboard.md)
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Visiteurs](../Visiteurs/_index.md) | [Utilisateur non connectÃ©](../UtilisateurNonConnecte/_index.md)

---

**Document** : Exposants â€” Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit â€” rÃ©fÃ©rence pour le public Exposants

