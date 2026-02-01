# Exposants — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du public cible **Exposants** pour le service Miyukini Festival Service. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels, les parcours détaillés, les user stories, les pain points et opportunités, ainsi que la priorisation et les dépendances. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md), [Parcours, capacités et dashboard](./Exposants%20-%20Parcours%20Capacites%20Dashboard.md), [Politique de résidence des données sensibles](../../../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) (COG de référence, données exposant).

## Portée / Scope

- **Public** : Exposants (professionnels ou structures participant à des événements en tant qu’exposants).
- **Périmètre** : tous les besoins identifiés pour ce public (fonctionnels, non fonctionnels, parcours, scénarios, priorisation).
- **Hors périmètre** : spécifications techniques d’implémentation (API, schémas de données détaillés), spécifications des autres publics (organisateurs, visiteurs, utilisateur non connecté) — traitées dans leurs propres documents d’analyse.

---

## 1. Profil du public et personas

### 1.1 Définition du public

Les **exposants** sont des professionnels, entreprises ou associations qui **participent à des événements/festivals** en tant qu’exposants (stand, présence commerciale ou associative). Ils disposent d’un **compte cross-événements** : un même exposant peut **participer à plusieurs festivals** depuis un seul espace. Ils ont accès à un **dashboard exposant dédié** (candidatures, participations, agenda, documents, factures). La plateforme propose une **gestion d’agenda** pour éviter qu’un exposant s’inscrive à deux événements à la même date (conflits de dates — besoin déjà vu en pratique).

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Artisan / créateur** | Petit exposant ; participe à 2 à 5 festivals par an ; peu de temps pour l’administratif. | Candidater facilement, suivre les statuts et les factures, éviter les doublons de dates. | Multiples plateformes par festival, emails éparpillés, risque de s’inscrire à deux festivals le même week-end. |
| **Entreprise / marque** | Exposant régulier ; participe à 10 à 30 salons/festivals par an ; équipe dédiée ou prestataire. | Un seul point d’accès pour toutes les participations, agenda consolidé, visibilité dans un répertoire. | Pas de vue consolidée, reporting manuel, difficulté à planifier sur plusieurs événements. |
| **Association / collectif** | Exposant occasionnel ; 1 à 3 événements par an ; bénévoles. | Candidater, récupérer les documents et règlements, payer en temps utile. | Manque de clarté sur les étapes, documents perdus, délais de paiement oubliés. |
| **Exposant multi-festivals** | Exposant actif sur plusieurs événements (saison, thématiques) ; besoin de cohérence. | Dashboard unifié, agenda cross-événements, alerte conflits de dates, historique des participations. | Risque de double inscription à la même date ; pas de calendrier global. |

### 1.3 Contexte d’usage

- **Fréquence** : connexion ponctuelle (candidature, suivi statut, documents, factures) ; plus régulière en phase de préparation d’un événement.
- **Appareils** : desktop et mobile (consultation dashboard, dépôt candidature, téléchargement documents).
- **Concurrence** : emails et formulaires par festival ; attente d’un **guichet unique** pour toutes les participations.

---

## 2. Besoins fonctionnels

### 2.1 Onboarding et compte

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| EXP-01 | Création de compte exposant | Pouvoir s’inscrire en tant qu’exposant (email, mot de passe ou lien magique, fiche entreprise/contact). | Formulaire d’inscription dédié ; validation email si configurée ; création du profil exposant (Miyauth, Miyuprofile, fiche entreprise). |
| EXP-02 | Validation du compte | Le compte peut être validé automatiquement ou selon politique plateforme/organisateur. | Workflow de validation configurable ; notification à l’exposant (validé / en attente / refusé). |
| EXP-03 | Compte cross-événements | Un même exposant peut **participer à plusieurs festivals** sans recréer de compte. | Dashboard agrège candidatures, participations, documents et factures pour **tous** les festivals concernés. |
| EXP-04 | Fiche entreprise / contact | Pouvoir compléter et mettre à jour la fiche entreprise (nom, contact, activité, logo, site web). | Formulaire fiche exposant ; champs configurables ; mise à jour à tout moment ; fiche utilisée pour les candidatures et le répertoire. |

### 2.2 Dashboard exposant

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| EXP-05 | Vue d’ensemble du dashboard | Avoir une vue unifiée sur candidatures, participations, agenda, documents, factures. | Page d’accueil dashboard avec blocs ou onglets : Candidatures, Participations, Agenda, Documents, Factures ; indicateurs synthétiques (ex. candidatures en attente, prochain événement). |
| EXP-06 | Liste des candidatures | Voir la liste de toutes les candidatures (en attente, validées, refusées) par édition. | Liste filtrable (statut : en attente, validée, refusée ; édition ; date) ; accès au détail et aux pièces jointes ; tri et pagination. |
| EXP-07 | Liste des participations | Voir la liste des éditions auxquelles l’exposant participe (validé) avec accès aux documents et facturation. | Liste des éditions validées ; fiche par édition (dates, lieu, statut, emplacement, lien documents/factures) ; accès au programme public si mis à disposition. |
| EXP-08 | Accès rapide aux documents et factures | Accéder aux documents et factures depuis le dashboard (par édition ou global). | Liens directs vers documents (contrats, règlements) et factures (devis, factures) ; téléchargement PDF ; statut de paiement visible. |

### 2.3 Candidatures

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| EXP-09 | Découverte des événements ouverts aux candidatures | Consulter l’annuaire des événements pour identifier les festivals ouverts aux candidatures. | Liste/carte des événements avec filtre « Candidatures ouvertes » ; fiche événement (dates, lieu, thème, délai candidature) ; lien « Candidater ». |
| EXP-10 | Dépôt d’une candidature | Déposer une candidature pour une édition en remplissant le formulaire et en joignant les pièces demandées. | Formulaire de candidature par édition (champs définis par l’organisateur) ; upload de pièces jointes (fiche entreprise, logo, etc.) ; prévisualisation avant envoi ; envoi et accusé de réception. |
| EXP-11 | Vérification agenda avant candidature | Être alerté ou bloqué si la candidature concerne une date en conflit avec une autre édition (déjà inscrit ou candidat). | Vérification des dates à la soumission ; alerte « Conflit de dates avec l’événement X » ou blocage ; suggestion de consulter l’agenda. |
| EXP-12 | Suivi du statut de la candidature | Consulter le statut de chaque candidature (en attente, validée, refusée) et recevoir une notification en cas de changement. | Statut visible dans la liste et la fiche candidature ; notification (Miyunotify) à la validation ou au refus ; motif de refus affiché si communiqué par l’organisateur. |
| EXP-13 | Modification ou annulation de candidature | Modifier ou annuler une candidature en attente (selon règles de l’édition). | Actions « Modifier » / « Annuler » si autorisées par l’organisateur et si statut « En attente » ; confirmation ; notification à l’organisateur si configuré. |

### 2.4 Gestion d’agenda et conflits de dates

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| EXP-14 | Calendrier cross-événements | Visualiser les dates des événements auxquels l’exposant est inscrit ou candidat. | Vue calendrier (mois, semaine) avec les événements (candidat ou inscrit) ; couleur ou libellé par statut ; lien vers la fiche édition. |
| EXP-15 | Alerte conflits de dates | Être alerté ou bloqué en cas de chevauchement de dates avant validation d’une nouvelle candidature. | Détection des chevauchements (même week-end, même jour, selon règle) ; alerte à la soumission de candidature ; message explicite (« Conflit avec Festival X »). |
| EXP-16 | Export ou partage d’agenda | Exporter ou partager son agenda (calendrier) pour planification externe. | Export calendrier (iCal, PDF) ou lien de partage ; mise à jour automatique si nouvelles participations. |

### 2.5 Participations (éditions validées)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| EXP-17 | Fiche par édition participée | Consulter la fiche de chaque édition à laquelle l’exposant participe (résumé, documents, emplacement, programme). | Fiche édition : dates, lieu, statut, emplacement attribué (stand/zone), lien plan de salle si exposé ; accès aux documents et à la facturation. |
| EXP-18 | Accès au plan de salle (emplacement) | Consulter le plan de salle et son emplacement attribué si mis à disposition par l’organisateur. | Vue plan de salle (lecture seule) avec emplacement mis en évidence ; légende ; export ou impression si autorisé. |
| EXP-19 | Accès au programme public | Consulter le programme public de l’édition si mis à disposition. | Lien vers le programme public (animations, créneaux, salles) ; vue lecture seule. |

### 2.6 Documents et facturation

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| EXP-20 | Consultation et téléchargement des documents | Consulter et télécharger les documents reçus (contrats, règlements, conventions) par édition. | Liste des documents par édition ; téléchargement PDF ou fichier ; statut (reçu, à signer, signé) si applicable. |
| EXP-21 | Envoi de documents signés ou complétés | Envoyer des documents signés ou complétés selon le workflow organisateur. | Upload de document signé ou formulaire à compléter ; envoi à l’organisateur ; accusé de réception ; suivi (envoyé, reçu). |
| EXP-22 | Consultation des devis et factures | Consulter les devis et factures (Miyuinvoice) par édition ; télécharger le PDF. | Liste des devis et factures par édition ; détail (lignes, montants, conditions) ; téléchargement PDF ; statut (devis : envoyé, accepté, refusé ; facture : payé, en attente). |
| EXP-23 | Acceptation d’un devis | Accepter ou refuser un devis reçu (si workflow organisateur le prévoit). | Action « Accepter » / « Refuser » sur le devis ; notification à l’organisateur ; mise à jour du statut ; conversion en facture côté organisateur si accepté. |
| EXP-24 | Suivi du statut de paiement | Voir le statut de paiement des factures (payé / en attente) et les échéances. | Statut visible sur chaque facture ; date d’échéance ; rappel ou alerte si configuré (Miyunotify). |

### 2.7 Répertoire des exposants

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| EXP-25 | Visibilité dans le répertoire | La fiche exposant peut apparaître dans le répertoire des exposants du catalogue (selon politique plateforme et choix organisateur). | Fiche exposant publiée dans le répertoire (entreprise, contact, éditions participées, etc.) ; visible par [utilisateur non connecté](../UtilisateurNonConnecte/_index.md) et tous les publics ; option de désactivation si proposée. |
| EXP-26 | Mise à jour de la fiche publique | Pouvoir mettre à jour les informations de la fiche exposant affichée dans le répertoire. | Édition des champs autorisés (nom, description, logo, site web, réseaux) ; mise à jour reflétée dans le répertoire ; modération selon politique plateforme si applicable. |

### 2.8 Notifications et communication

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| EXP-27 | Réception des notifications | Recevoir des notifications (candidature validée/refusée, nouveau document, devis/facture envoyé, rappel paiement). | Notifications (Miyunotify) par email et/ou in-app ; préférences de notification configurables (par type, par édition). |
| EXP-28 | Historique des communications | Consulter l’historique des communications reçues (annonces, documents envoyés) par édition. | Liste des notifications/messages par édition ; date, objet, lien vers le document ou l’action. |

---

## 3. Besoins non fonctionnels

### 3.1 Performance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-EXP-01 | Temps de chargement du dashboard | Le dashboard exposant se charge en moins de 3 secondes (réseau standard). |
| NFR-EXP-02 | Temps de soumission d’une candidature | La soumission d’une candidature (formulaire + pièces jointes) s’effectue en moins de 5 secondes après clic « Envoyer ». |
| NFR-EXP-03 | Téléchargement de documents | Le téléchargement d’un document (PDF) s’effectue en moins de 5 secondes pour des fichiers < 5 Mo. |

### 3.2 Disponibilité et fiabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-EXP-04 | Disponibilité | Le service est disponible 99,5 % du temps (hors fenêtres de maintenance annoncées). |
| NFR-EXP-05 | Sauvegarde des données | Les données (fiche, candidatures, documents reçus) sont sauvegardées et récupérables ; pas de perte à la soumission d’une candidature. |

### 3.3 Sécurité et gouvernance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-EXP-06 | Authentification | Authentification sécurisée (Miyauth) ; mot de passe ou lien magique ; session avec expiration. |
| NFR-EXP-07 | Isolation des données | Les données exposant (fiche, candidatures, factures) ne sont accessibles qu’à l’exposant et aux organisateurs des éditions concernées ; pas d’accès aux données des autres exposants. |
| NFR-EXP-08 | Confidentialité des documents | Les documents et factures ne sont accessibles qu'à l'exposant et à l'organisateur de l'édition concernée. |
| NFR-EXP-09 | Résidence centralisée des données sensibles | Les données exposant (fiche, candidatures, documents, facturation) sont à **résidence centralisée** : la copie canonique réside sur le **COG de référence** (COG organisateur ou COG du Service). En cas de coupure du terminal exposant, les données restent disponibles pour les organisateurs. Le terminal exposant n'en détient pas la seule copie (voir [Politique Residence Donnees Sensibles](../../../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)). | et factures ne sont accessibles qu’à l’exposant et à l’organisateur de l’édition concernée. |

### 3.4 Utilisabilité et accessibilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-EXP-09 | Utilisabilité | Les parcours principaux (dépôt candidature, consultation statut, téléchargement facture) sont réalisables en moins de 5 clics depuis le dashboard. |
| NFR-EXP-10 | Accessibilité | Conformité WCAG 2.1 niveau AA pour le dashboard exposant (navigation clavier, lecteurs d’écran, contrastes). |
| NFR-EXP-11 | Responsive | Le dashboard et le formulaire de candidature sont utilisables sur mobile (consultation, dépôt candidature, téléchargement). |

### 3.5 Gestion d’agenda

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-EXP-12 | Détection des conflits de dates | La plateforme détecte un conflit de dates (même jour ou chevauchement selon règle) avant validation d’une nouvelle candidature et alerte ou bloque. |
| NFR-EXP-13 | Précision des dates | Les dates des événements (début, fin) sont affichées avec précision (jour, heure si pertinent) pour permettre la planification. |

---

## 4. Parcours détaillés et scénarios

### 4.1 Scénario : Premier usage — création de compte et première candidature

1. L’utilisateur découvre un événement depuis le catalogue (annuaire des événements) en [utilisateur non connecté](../UtilisateurNonConnecte/_index.md).
2. Il clique sur « Candidater » ; il est redirigé vers l’inscription ou la connexion.
3. Il crée un compte exposant (email, mot de passe, fiche entreprise : nom, contact, activité, logo).
4. Après validation (automatique ou manuelle), il accède au dashboard exposant (vide).
5. Il retourne sur la fiche de l’événement et dépose sa candidature (formulaire défini par l’organisateur, pièces jointes).
6. À la soumission, la plateforme vérifie l’agenda : pas de conflit de dates ; la candidature est enregistrée en « En attente ».
7. L’organisateur reçoit la candidature ; l’exposant reçoit un accusé de réception.
8. L’organisateur valide la candidature ; l’exposant reçoit une notification et voit le statut « Validée » dans son dashboard.
9. L’exposant accède à la fiche de l’édition (documents, emplacement quand attribué, facturation).

**Besoins couverts** : EXP-01 à EXP-05, EXP-09 à EXP-12, EXP-17, EXP-20, EXP-27.

### 4.2 Scénario : Conflit de dates — alerte à la candidature

1. L’exposant est déjà inscrit à « Festival A » (dates 15-16 juin).
2. Il consulte l’annuaire et souhaite candidater à « Festival B » (dates 15-17 juin).
3. Il remplit le formulaire de candidature pour Festival B et clique sur « Envoyer ».
4. La plateforme détecte un chevauchement de dates avec Festival A.
5. Un message s’affiche : « Conflit de dates : vous êtes déjà inscrit à Festival A (15-16 juin). Festival B a lieu le 15-17 juin. Souhaitez-vous tout de même soumettre votre candidature ? » (ou blocage selon règle).
6. L’exposant peut annuler ou confirmer (avec avertissement) ; s’il confirme, la candidature est envoyée mais l’organisateur peut être informé du conflit.

**Besoins couverts** : EXP-11, EXP-14, EXP-15, NFR-EXP-12.

### 4.3 Scénario : Multi-festivals — vue consolidée et facturation

1. L’exposant participe à 3 festivals (validé) et a 2 candidatures en attente.
2. Il se connecte et accède au dashboard ; il voit les blocs Candidatures (2 en attente), Participations (3 éditions), Agenda (calendrier des 5 événements), Documents, Factures.
3. Il clique sur « Participations » et voit la liste des 3 éditions avec pour chacune : dates, lieu, emplacement, lien documents, lien factures.
4. Il ouvre la facture d’une édition : statut « En attente », échéance 30/04 ; il télécharge le PDF et procède au paiement hors plateforme.
5. Il consulte l’agenda : vue calendrier avec les 5 événements (2 en attente, 3 validés) ; pas de chevauchement.
6. Il dépose une nouvelle candidature pour un 4e festival ; la plateforme vérifie l’agenda : pas de conflit ; candidature enregistrée.

**Besoins couverts** : EXP-05 à EXP-08, EXP-14, EXP-17, EXP-20, EXP-22, EXP-24.

### 4.4 Scénario : Documents et facturation

1. L’organisateur envoie un contrat type et un règlement à l’exposant (par édition).
2. L’exposant reçoit une notification ; il accède au dashboard, onglet Documents, et voit les nouveaux documents pour l’édition X.
3. Il télécharge le contrat et le règlement, les signe, et les renvoie via l’interface (upload ou formulaire).
4. L’organisateur reçoit les documents signés ; il génère un devis et l’envoie à l’exposant.
5. L’exposant reçoit une notification ; il consulte le devis dans l’onglet Factures, l’accepte.
6. L’organisateur convertit le devis en facture ; l’exposant voit la facture (statut « En attente »), télécharge le PDF, paie.
7. L’organisateur marque la facture comme payée (ou synchronisation paiement si intégré) ; l’exposant voit le statut « Payé ».

**Besoins couverts** : EXP-20 à EXP-24, EXP-27.

---

## 5. Pain points et opportunités

### 5.1 Pain points

| Pain point | Impact | Besoin associé |
|------------|--------|-----------------|
| **Multiples plateformes par festival** | Un outil différent par événement ; identifiants et processus multiples. | Un seul compte cross-événements et un seul dashboard (EXP-03, EXP-05). |
| **Emails éparpillés** | Documents et factures reçus par email ; risque de perte, pas de vue consolidée. | Dashboard avec documents et factures par édition (EXP-08, EXP-20 à EXP-24). |
| **Risque de double inscription à la même date** | S’inscrire à deux festivals le même week-end ; conflit opérationnel. | Gestion d’agenda et alerte conflits de dates (EXP-14, EXP-15, NFR-EXP-12). |
| **Manque de clarté sur les étapes** | Ne pas savoir où en est la candidature, le devis, la facture. | Suivi du statut (EXP-12, EXP-22, EXP-24) et notifications (EXP-27). |
| **Documents perdus** | Contrats et règlements éparpillés. | Centralisation des documents par édition dans le dashboard (EXP-20, EXP-21). |
| **Délais de paiement oubliés** | Oubli d’échéance de facture. | Suivi du statut de paiement et rappels (EXP-24, EXP-27). |

### 5.2 Opportunités

| Opportunité | Description | Besoin associé |
|-------------|-------------|-----------------|
| **Vue consolidée multi-festivals** | Un seul écran pour toutes les participations et candidatures. | Dashboard unifié (EXP-05 à EXP-08). |
| **Agenda cross-événements** | Calendrier global pour planifier sans conflit. | Calendrier et alerte conflits (EXP-14, EXP-15). |
| **Visibilité répertoire** | Être visible dans le répertoire des exposants pour les visiteurs et les organisateurs. | EXP-25, EXP-26. |
| **Réduction de la charge administrative** | Moins d’emails, tout au même endroit. | Dashboard, documents, factures centralisés (EXP-05 à EXP-08, EXP-20 à EXP-24). |

---

## 6. Priorisation des besoins (MoSCoW)

### 6.1 Must have (indispensable)

- EXP-01 à EXP-05 (onboarding, compte cross-événements, fiche entreprise, vue dashboard).
- EXP-09 à EXP-13 (candidatures : découverte, dépôt, vérification agenda, suivi statut, modification/annulation).
- EXP-14, EXP-15 (calendrier cross-événements, alerte conflits de dates).
- EXP-17 (fiche par édition participée).
- EXP-20 à EXP-24 (documents, devis, factures, acceptation devis, suivi paiement).
- EXP-27 (notifications).
- NFR-EXP-06 à NFR-EXP-08 (authentification, isolation, confidentialité).
- NFR-EXP-12 (détection conflits de dates).

### 6.2 Should have (important)

- EXP-06 à EXP-08 (liste candidatures, liste participations, accès rapide documents/factures).
- EXP-16 (export ou partage agenda).
- EXP-18, EXP-19 (plan de salle, programme public).
- EXP-25, EXP-26 (répertoire des exposants, mise à jour fiche publique).
- EXP-28 (historique des communications).
- NFR-EXP-01 à NFR-EXP-05, NFR-EXP-09 à NFR-EXP-11, NFR-EXP-13 (performance, dispo, utilisabilité, accessibilité, responsive, précision dates).

### 6.3 Could have (souhaitable)

- Amélioration des préférences de notification (granularité par type, par édition).
- Export des données (candidatures, participations, factures) pour comptabilité.

### 6.4 Won’t have (hors périmètre ou report)

- Paiement en ligne intégré (si hors périmètre v1) ; suivi du statut de paiement reste manuel côté organisateur.
- Besoins spécifiques aux autres publics — traités dans leurs documents.

---

## 7. Dépendances et interfaces avec les autres publics

### 7.1 Dépendances

| Dépendance | Description |
|------------|-------------|
| **Organisateurs** | Les candidatures sont traitées par les organisateurs (validation, refus) ; les devis et factures sont émis par les organisateurs (Miyuinvoice) ; les documents sont envoyés par les organisateurs. |
| **Catalogue** | L’annuaire des événements (catalogue) permet à l’exposant de découvrir les événements ouverts aux candidatures ; le répertoire des exposants peut afficher la fiche de l’exposant (EXP-25). |
| **Plateforme** | Authentification (Miyauth), permissions (Master Butler), persistance (KindMother), agenda cross-événements (MiyuClock, Miyubooking, données d’édition). |

### 7.2 Interfaces

| Interface | Flux | Besoin exposant |
|-----------|------|------------------|
| Exposant → Organisateur | Dépôt candidature, acceptation devis, envoi documents signés. | EXP-10, EXP-21, EXP-23. |
| Organisateur → Exposant | Validation/refus candidature, envoi documents, envoi devis/facture. | EXP-12, EXP-20, EXP-22, EXP-27. |
| Exposant → Catalogue | Consultation annuaire des événements ; visibilité dans le répertoire des exposants. | EXP-09, EXP-25, EXP-26. |

---

## 8. User stories (format standard)

### 8.1 Onboarding et dashboard

- **US-EXP-01** — En tant qu’**exposant**, je veux **créer un compte** (email, mot de passe, fiche entreprise) **afin de** candidater à des événements et gérer mes participations.  
  *Critères* : Formulaire dédié ; fiche entreprise (nom, contact, activité, logo) ; création profil (Miyauth, Miyuprofile).*

- **US-EXP-02** — En tant qu’**exposant**, je veux **accéder à un dashboard unique** avec toutes mes candidatures, participations, documents et factures **afin de** tout retrouver au même endroit.  
  *Critères* : Vue d’ensemble ; blocs Candidatures, Participations, Agenda, Documents, Factures ; accès en moins de 3 clics aux éléments principaux.*

### 8.2 Candidatures et agenda

- **US-EXP-03** — En tant qu’**exposant**, je veux **déposer une candidature** pour un événement (formulaire, pièces jointes) **afin de** participer au festival.  
  *Critères* : Formulaire par édition ; upload pièces ; vérification agenda (conflit de dates) ; accusé de réception.*

- **US-EXP-04** — En tant qu’**exposant**, je veux **être alerté en cas de conflit de dates** avant de valider une candidature **afin de** ne pas m’inscrire à deux événements à la même date.  
  *Critères* : Détection chevauchement ; message explicite ; alerte ou blocage selon règle.*

- **US-EXP-05** — En tant qu’**exposant**, je veux **voir le statut de mes candidatures** (en attente, validée, refusée) et **recevoir une notification** en cas de changement **afin de** suivre l’avancement.  
  *Critères* : Statut visible dans la liste et la fiche ; notification à la validation/refus ; motif de refus si communiqué.*

### 8.3 Documents et facturation

- **US-EXP-06** — En tant qu’**exposant**, je veux **consulter et télécharger** les documents (contrats, règlements) et les factures **afin de** les archiver et payer en temps utile.  
  *Critères* : Liste par édition ; téléchargement PDF ; statut de paiement visible.*

- **US-EXP-07** — En tant qu’**exposant**, je veux **accepter un devis** reçu **afin de** confirmer ma participation et déclencher l’émission de la facture.  
  *Critères* : Action Accepter/Refuser ; notification à l’organisateur ; mise à jour statut.*

### 8.4 Visibilité

- **US-EXP-08** — En tant qu’**exposant**, je veux **apparaître dans le répertoire des exposants** (fiche entreprise, éditions participées) **afin de** être visible par les visiteurs et les organisateurs.  
  *Critères* : Fiche publiée selon politique plateforme ; mise à jour par l’exposant ; option désactivation si proposée.*

---

## 9. Cas limites et règles métier

### 9.1 Règles métier

| Règle | Description |
|-------|-------------|
| **Candidature** | L’exposant ne peut pas modifier les paramètres des éditions ; il dépose une candidature et attend la décision de l’organisateur (StrongFather, validation). |
| **Agenda** | La plateforme signale ou bloque les conflits de dates ; l’exposant peut toutefois confirmer sa candidature malgré un conflit (selon règle) ; l’organisateur peut être informé. |
| **Documents et factures** | Les documents et factures sont émis par l’organisateur ; l’exposant consulte, télécharge et renvoie les documents signés ; le paiement peut être hors plateforme (suivi du statut par l’organisateur). |
| **Répertoire** | La fiche exposant peut être publiée dans le répertoire selon la politique plateforme et les choix de l’organisateur ; l’exposant peut mettre à jour les champs autorisés. |

### 9.2 Cas limites

| Cas | Comportement attendu |
|-----|----------------------|
| **Candidature sur une édition clôturée** | Impossible : les candidatures sont fermées pour les éditions clôturées. |
| **Candidature sur une édition dont les dates chevauchent une autre édition (déjà inscrit)** | Alerte ou blocage à la soumission ; message explicite avec le nom de l’événement en conflit. |
| **Modification de candidature après validation** | Impossible : une candidature validée ne peut pas être modifiée par l’exposant ; contacter l’organisateur. |
| **Suppression de compte exposant avec participations en cours** | Blocage ou processus spécifique : les données de participations et facturation doivent être conservées pour l’organisateur ; proposer la désactivation du compte et l’archivage des données. |
| **Devis expiré** | Si l’organisateur a défini une date d’expiration au devis, l’exposant ne peut plus accepter après cette date ; message « Devis expiré ». |

### 9.3 Métriques de succès

| Métrique | Description | Cible (exemple) |
|----------|-------------|------------------|
| **Taux d’activation** | % d’exposants ayant déposé au moins une candidature après inscription. | > 70 % |
| **Taux de conflits de dates évités** | % de candidatures où une alerte conflit a été affichée et l’exposant a annulé ou modifié. | Suivi |
| **Satisfaction exposant** | Score NPS ou enquête (facilité, clarté, gain de temps). | Suivi annuel |
| **Nombre de participations par exposant** | Moyenne et médiane du nombre d’éditions (candidatures + participations) par exposant. | Suivi ; objectif croissance |

---

## 10. Critères d’acceptation détaillés (sélection)

### 10.1 Candidature (EXP-10, EXP-11)

- **Formulaire** : Champs définis par l’organisateur (obligatoires et optionnels) ; validation côté client (format email, champs requis) ; sauvegarde brouillon si proposé.
- **Pièces jointes** : Types de fichiers autorisés (PDF, images) ; taille max par fichier et globale ; prévisualisation avant envoi.
- **Vérification agenda** : À la soumission, comparaison des dates de l’édition avec les dates des événements auxquels l’exposant est déjà inscrit ou candidat ; règle de chevauchement (même jour, même week-end, chevauchement partiel) configurable ; message d’alerte avec nom de l’événement en conflit.
- **Accusé de réception** : Email et/ou notification in-app confirmant l’enregistrement de la candidature ; numéro ou identifiant de candidature pour suivi.

### 10.2 Dashboard (EXP-05 à EXP-08)

- **Vue d’ensemble** : Blocs ou onglets Candidatures, Participations, Agenda, Documents, Factures ; indicateurs synthétiques (ex. « 2 candidatures en attente », « Prochain événement : Festival X, 15-16 juin »).
- **Liste des candidatures** : Colonnes : Édition, Date dépôt, Statut, Date mise à jour ; tri par date, statut ; filtre par statut (en attente, validée, refusée) ; lien vers fiche détail et pièces jointes.
- **Liste des participations** : Colonnes : Édition, Dates, Lieu, Statut, Emplacement ; lien vers fiche édition (documents, plan, programme, factures).
- **Agenda** : Vue calendrier (mois, semaine) ; événements (candidat ou inscrit) affichés avec libellé et statut ; lien vers fiche édition.

### 10.3 Documents et facturation (EXP-20 à EXP-24)

- **Documents** : Liste par édition ; colonnes : Document, Date envoi, Statut (reçu, à signer, signé) ; bouton Télécharger (PDF).
- **Factures** : Liste par édition ; colonnes : Numéro, Date, Montant, Statut (devis envoyé, accepté, refusé ; facture en attente, payée), Échéance ; bouton Télécharger PDF ; action Accepter/Refuser sur devis si workflow activé.
- **Suivi paiement** : Statut « Payé » ou « En attente » ; date de paiement si renseignée par l’organisateur ; rappel ou alerte si échéance proche (configurable).

---

## 11. Glossaire et références

### 11.1 Glossaire (extrait)

| Terme | Définition |
|-------|------------|
| **Candidature** | Demande de participation d’un exposant à une édition ; statuts : en attente, validée, refusée. |
| **Dashboard exposant** | Espace dédié à l’exposant : vue unifiée sur candidatures, participations, agenda, documents, factures. |
| **Conflit de dates** | Chevauchement des dates d’une édition avec une autre édition à laquelle l’exposant est déjà inscrit ou candidat. |
| **Répertoire des exposants** | Annuaire du catalogue listant les exposants (fiche entreprise, éditions participées) ; visible par le public. |

### 11.2 Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
- [Exposants — Parcours, capacités et dashboard](./Exposants%20-%20Parcours%20Capacites%20Dashboard.md)
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Visiteurs](../Visiteurs/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)

---

**Document** : Exposants — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit — référence pour le public Exposants
