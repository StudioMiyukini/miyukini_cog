# Organisateurs — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du public cible **Organisateurs** pour le service Miyukini Festival Service. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels, les parcours détaillés, les user stories, les pain points et opportunités, ainsi que la priorisation et les dépendances. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md), [Parcours, capacités et livrables](./Organisateurs%20-%20Parcours%20Capacites%20Livrables.md).

## Portée / Scope

- **Public** : Organisateurs (structures qui créent et gèrent des événements/festivals).
- **Périmètre** : tous les besoins identifiés pour ce public (fonctionnels, non fonctionnels, parcours, scénarios, priorisation).
- **Hors périmètre** : spécifications techniques d’implémentation (API, schémas de données détaillés), spécifications des autres publics (exposants, visiteurs, utilisateur non connecté) — traitées dans leurs propres documents d’analyse.

---

## 1. Profil du public et personas

### 1.1 Définition du public

Les **organisateurs** sont des structures (associations, collectivités, sociétés, collectifs) qui **créent et gèrent des événements ou festivals**. Ils disposent d’un **compte cross-événements** : un même organisateur peut gérer **plusieurs éditions** (festivals/événements) depuis un seul espace. Ils opèrent dans le cadre d’un **Mandat de Permission** (StrongFather, Master Butler) et ne peuvent pas accéder aux données des autres organisateurs ni modifier la gouvernance plateforme.

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Admin association** | Bénévole ou salarié d’une association ; gère 1 à 3 festivals par an ; peu de temps à consacrer à l’outil. | Centraliser les infos, publier l’événement, gérer les exposants et le budget sans multiplier les outils. | Outils dispersés, doublons de saisie, manque de visibilité sur l’avancement. |
| **Responsable collectivité** | Agent ou élu ; gère plusieurs événements (festivals, salons, foires) ; contraintes réglementaires et budgétaires fortes. | Piloter plusieurs éditions, respecter les règles, rendre des comptes (budget, rapports). | Traçabilité insuffisante, reporting manuel, difficulté à comparer les éditions. |
| **Manager festival** | Coordinateur opérationnel ; gère le programme, les exposants, le plan de salle ; travaille avec une équipe (bénévoles, prestataires). | Avoir une vue claire par édition, déléguer des tâches (rôles), communiquer avec exposants et équipe. | Manque de délégation fine, communications éparpillées, risque d’erreurs sur les créneaux ou emplacements. |
| **Organisateur multi-festivals** | Structure qui organise plusieurs festivals (saison, thématiques différentes) ; besoin de cohérence et de réutilisation. | Un seul compte pour tous les festivals, réutiliser des paramètres (règlements, contrats types), comparer les éditions. | Duplication des configs, pas de vue consolidée, risque d’incohérence entre éditions. |

### 1.3 Contexte d’usage

- **Fréquence** : connexion régulière en phase de préparation (quotidienne ou hebdomadaire), puis ponctuelle en phase d’exploitation et de clôture.
- **Appareils** : desktop prioritaire pour la configuration et la gestion ; mobile pour la consultation et les alertes (notifications).
- **Concurrence** : outils métier (tableurs, email, outils de planification) ; attente d’un **guichet unique** pour l’événementiel.

---

## 2. Besoins fonctionnels

### 2.1 Onboarding et compte

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-01 | Création de compte organisateur | Pouvoir s’inscrire en tant qu’organisateur (email, mot de passe ou lien magique, informations structure). | Formulaire d’inscription dédié ; validation email si configurée ; création du profil organisateur (Miyauth, Miyuprofile). |
| ORG-02 | Validation du compte | Le compte peut être validé manuellement (plateforme) ou automatiquement selon politique. | Workflow de validation configurable ; notification à l’organisateur (validé / en attente / refusé). |
| ORG-03 | Attribution des permissions | Attribution du rôle organisateur (Admin ou Manager) et émission du Mandat de Permission. | Rôle et Mandat attribués après validation ; accès à l’espace organisateur selon le rôle. |
| ORG-04 | Compte cross-événements | Un même organisateur peut gérer **plusieurs éditions** sans recréer de compte. | Liste de toutes les éditions de l’organisateur (passées, en cours, à venir) ; création d’une nouvelle édition depuis le même compte. |
| ORG-05 | Rattachement à une structure existante | Pouvoir rejoindre une structure organisatrice existante (invitation, lien). | Flux d’invitation (email) ; acceptation et rattachement ; attribution d’un rôle (Admin, Manager) au sein de la structure. |

### 2.2 Gestion des éditions (multi-festivals)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-06 | Création d’une édition | Pouvoir créer une nouvelle édition (événement) avec les métadonnées de base. | Formulaire de création (nom, dates, lieu, thème, statut brouillon/publié) ; édition créée et visible dans la liste des éditions. |
| ORG-07 | Liste globale des éditions | Avoir une vue de **toutes** les éditions de l’organisateur (multi-festivals). | Liste avec filtres (statut : brouillon, en cours, passée, à venir ; année ; recherche par nom) ; tri et pagination si besoin. |
| ORG-08 | Tableau de bord par édition | Accéder à un dashboard dédié par édition avec indicateurs et accès rapides. | Vue synthétique : nombre d’exposants, candidatures en attente, budget (revenus/dépenses), programme (nombre d’animations), plan de salle (stands attribués) ; liens vers les modules (exposants, plan, programme, budget, documents). |
| ORG-09 | Paramétrage de l’édition | Configurer les paramètres de l’édition (nom, dates, lieu, thème, objectifs, règles, conditions d’inscription). | Formulaire de paramétrage par édition ; sauvegarde et historique des modifications si requis ; pas d’impact sur les autres éditions. |
| ORG-10 | Duplication d’édition | Pouvoir dupliquer une édition existante pour en créer une nouvelle (réutilisation des paramètres, du plan, du programme type). | Option « Dupliquer l’édition » ; choix des éléments à dupliquer (paramètres, plan, programme, documents types) ; nouvelle édition créée en statut brouillon. |
| ORG-11 | Clôture et archivage | Clôturer une édition et archiver les données. | Action « Clôturer l’édition » ; édition passée en statut « Clôturée » ; données en lecture seule ; possibilité d’export (rapport, données) avant clôture. |

### 2.3 Exposants

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-12 | Annuaire local des exposants | Voir la liste des exposants par édition avec statut et informations clés. | Liste filtrable (statut : candidat, validé, refusé ; catégorie ; recherche) ; colonnes configurables ; export CSV/Excel. |
| ORG-13 | Réception des candidatures | Recevoir et consulter les candidatures exposants déposées pour l’édition. | Liste des candidatures en attente ; fiche détail (données exposant, pièces jointes, date de dépôt) ; notification à l’arrivée d’une nouvelle candidature (Miyunotify). |
| ORG-14 | Validation ou refus des candidatures | Valider ou refuser une candidature avec motif optionnel et notification à l’exposant. | Actions « Valider » / « Refuser » ; champ motif (obligatoire en cas de refus) ; notification à l’exposant (Miyunotify) ; mise à jour du statut dans l’annuaire local. |
| ORG-15 | Fiche exposant | Consulter et éditer la fiche exposant (coordonnées, statut, emplacement attribué, documents). | Fiche détail ; édition des champs autorisés par l’organisateur ; historique des statuts et des documents. |
| ORG-16 | Génération de devis | Générer un devis à partir des données exposant et des tarifs de l’édition (Miyuinvoice). | Création de devis (lignes, montants, conditions) ; envoi par email à l’exposant ; suivi (envoyé, accepté, refusé). |
| ORG-17 | Conversion devis → facture | Convertir un devis accepté en facture. | Action « Convertir en facture » ; facture créée (Miyuinvoice) ; envoi par email ; suivi du statut de paiement (payé / en attente). |
| ORG-18 | Import exposants (CSV / tableur) | Importer une liste d’exposants depuis un fichier CSV ou une feuille de calcul. | Upload de fichier ; mapping des colonnes ; prévisualisation et validation ; création ou mise à jour des fiches exposants. |

### 2.4 Plan de salle

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-19 | Définition des zones et stands | Définir les zones du plan et les stands (tailles, légende, zones techniques ou réservées). | Interface de création/édition du plan (canvas ou formulaire) ; zones, stands, légende ; sauvegarde. |
| ORG-20 | Attribution des emplacements | Attribuer un emplacement (stand/zone) à un exposant (formulaire ou drag & drop). | Liste des exposants et des stands disponibles ; attribution (sélection ou glisser-déposer) ; conflit si stand déjà attribué ; mise à jour du plan. |
| ORG-21 | Visualisation du plan | Visualiser le plan de salle avec les attributions et la légende. | Vue lecture du plan ; légende ; export visuel (PDF, image) pour impressions. |
| ORG-22 | Export du plan | Exporter le plan pour impression ou partage. | Export PDF ou image ; options (avec/sans légende, avec noms exposants). |

### 2.5 Programme

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-23 | Création d’animations | Ajouter des animations (nom, type, durée, salle/scène, horaire). | Formulaire de création ; association à une salle/scène/lieu ; choix de la date et du créneau horaire. |
| ORG-24 | Gestion des chevauchements | Bloquer ou alerter en cas de chevauchement d’animations sur une même salle ou créneau. | Vérification des conflits à la saisie ; alerte ou blocage selon configuration ; suggestion de créneaux libres. |
| ORG-25 | Vues du programme | Consulter le programme en vue chronologique ou par salle ; filtrer (jour, scène, type). | Vue chronologique ; vue par salle ; filtres (jour, scène, type d’activité) ; export (PDF, CSV). |
| ORG-26 | Modification et suppression | Modifier ou supprimer une animation. | Édition et suppression avec confirmation ; mise à jour des vues et des exports. |
| ORG-27 | Publication du programme public | Publier une version du programme visible par les visiteurs et le public (catalogue). | Action « Publier le programme » ; version publique figée ou dynamique selon config ; visible sur la fiche événement (catalogue). |

### 2.6 Budget

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-28 | Saisie des revenus et dépenses | Saisir les lignes de revenus et de dépenses par catégorie. | Formulaire de saisie (date, libellé, catégorie, montant, édition) ; catégories configurables par édition ou réutilisables. |
| ORG-29 | Ventilation par catégorie | Ventiler les revenus et dépenses par catégorie et consulter les totaux. | Vue par catégorie ; totaux et sous-totaux ; graphiques (répartition). |
| ORG-30 | Balance et statistiques | Consulter la balance (revenus - dépenses) et les statistiques par édition ou période. | Balance par édition ; comparaison entre éditions (si multi-festivals) ; export rapport (PDF, Excel). |
| ORG-31 | Lien avec la facturation | Les factures émises (Miyuinvoice) alimentent automatiquement les revenus si configuré. | Synchronisation factures → revenus (catégorie « Facturation exposants » ou équivalent) ; cohérence des montants. |

### 2.7 Documents et légal

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-32 | Contrats types et règlements | Stocker et gérer les contrats types, CGV, conventions, règlements (Miyucms, Miyumedia). | Upload et versioning des documents ; association à une édition ou globale (toutes éditions) ; accès restreint selon rôle (Master Butler). |
| ORG-33 | Envoi de documents aux exposants | Envoyer un document à signer ou à compléter à un ou plusieurs exposants. | Sélection du document et des exposants ; envoi par email (lien ou pièce jointe) ; suivi (envoyé, ouvert, complété). |
| ORG-34 | Historique des documents | Consulter l’historique des documents validés ou partagés avec les exposants. | Liste par édition et par exposant ; date d’envoi, statut, téléchargement. |

### 2.8 Notifications et communication

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-35 | Annonces globales | Diffuser une annonce à tous les exposants ou à l’équipe (ex. changement de programme). | Rédaction d’une annonce ; choix des destinataires (exposants, équipe, rôle) ; envoi (Miyunotify) ; historique des envois. |
| ORG-36 | Notifications ciblées | Envoyer des notifications ciblées (par rôle, par équipe, par exposant). | Sélection des destinataires (groupe, liste, rôle) ; message et pièce jointe optionnelle ; envoi et suivi. |
| ORG-37 | Paramétrage des notifications | Activer/désactiver et paramétrer les notifications par édition (rappels, alertes). | Configuration par type de notification (nouvelle candidature, paiement reçu, etc.) ; choix des canaux (email, in-app). |

### 2.9 Services visiteur (activables par l’organisateur)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-38 | Activation des services visiteur | Choisir quels services proposer aux visiteurs pour chaque édition (jeux, concours, ateliers, réservations, pass VIP). | Liste des services disponibles ; activation/désactivation par édition ; paramétrage (places limitées, dates, publics éligibles). |
| ORG-39 | Configuration des jeux | Configurer les jeux proposés (quizz, chasses au trésor, défis) si le service est activé. | Création/édition des jeux ; règles, questions, récompenses ; liaison avec l’édition. |
| ORG-40 | Configuration des concours | Configurer les concours (inscription, critères, jury, récompenses) si le service est activé. | Création/édition des concours ; dates, règles ; suivi des inscriptions et des résultats. |
| ORG-41 | Configuration des ateliers | Configurer les créneaux d’ateliers (capacité, durée, inscription) si le service est activé. | Création des créneaux ; capacité par créneau ; lien avec le programme ; réservations visiteurs (Miyubooking). |
| ORG-42 | Configuration des pass VIP | Configurer les pass VIP ou pass journée (tarifs, avantages) si le service est activé. | Création des types de pass ; tarifs ; avantages associés ; liaison avec la billetterie ou les réservations. |

### 2.10 Publication au catalogue

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-43 | Publication d’une édition | Demander la publication de l’édition dans l’annuaire des événements (catalogue). | Action « Publier au catalogue » ; workflow de validation selon politique plateforme (automatique ou manuel) ; édition visible dans l’annuaire une fois validée. |
| ORG-44 | Fiche organisateur dans le répertoire | La fiche organisateur (nom, événements, contact, charte) est visible dans le répertoire des organisateurs. | Fiche créée et mise à jour depuis l’espace organisateur ; visibilité dans le catalogue (répertoire des organisateurs) ; lien vers les éditions publiées. |
| ORG-45 | Visibilité des exposants | Selon politique plateforme, les exposants de l’édition peuvent apparaître dans le répertoire des exposants. | Option « Inclure les exposants dans le répertoire » par édition ; synchronisation avec le répertoire global ; respect de la politique plateforme. |

### 2.11 Équipe et rôles

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| ORG-46 | Gestion des rôles | Attribuer les rôles (Admin, Manager, Bénévole) aux membres de l’équipe organisateur. | Liste des membres ; attribution du rôle ; périmètre (toutes les éditions ou éditions assignées) ; invitation par email. |
| ORG-47 | Gestion des bénévoles | Gérer les bénévoles (zones, créneaux, informations de terrain). | Fiche bénévole ; attribution à des zones et créneaux ; accès limité (informations terrain uniquement) selon Master Butler. |
| ORG-48 | Délégation par édition | Un Manager peut être assigné à une ou plusieurs éditions uniquement. | Assignation Manager → éditions ; le Manager ne voit que ses éditions ; l’Admin voit toutes les éditions. |

---

## 3. Besoins non fonctionnels

### 3.1 Performance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-ORG-01 | Temps de chargement du tableau de bord | Le tableau de bord organisateur se charge en moins de 3 secondes (réseau standard). |
| NFR-ORG-02 | Temps de chargement du dashboard édition | Le dashboard d’une édition se charge en moins de 2 secondes. |
| NFR-ORG-03 | Export (CSV, PDF) | Les exports (liste exposants, rapport budget, plan) sont générés en moins de 10 secondes pour des volumes raisonnables (< 500 lignes / 100 stands). |

### 3.2 Disponibilité et fiabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-ORG-04 | Disponibilité | Le service est disponible 99,5 % du temps (hors fenêtres de maintenance annoncées). |
| NFR-ORG-05 | Sauvegarde et récupération | Les données sont sauvegardées et récupérables en cas d’incident ; pas de perte de données validées. |

### 3.3 Sécurité et gouvernance

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-ORG-06 | Authentification | Authentification sécurisée (Miyauth) ; mot de passe ou lien magique ; session avec expiration. |
| NFR-ORG-07 | Permissions | Les actions sont soumises aux permissions (Master Butler) ; un utilisateur ne peut accéder qu’aux éditions et données autorisées par son rôle. |
| NFR-ORG-08 | Isolation des données | Les données d’un organisateur ne sont pas accessibles aux autres organisateurs ; isolation stricte par Mandat. |
| NFR-ORG-09 | Audit | Les actions sensibles (validation candidature, publication, modification budget) sont tracées (qui, quand, quoi) pour audit. |

### 3.4 Utilisabilité et accessibilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-ORG-10 | Utilisabilité | Les parcours principaux (création édition, validation candidature, attribution stand) sont réalisables en moins de 5 clics depuis le tableau de bord. |
| NFR-ORG-11 | Accessibilité | Conformité WCAG 2.1 niveau AA pour les écrans de l’espace organisateur (navigation clavier, lecteurs d’écran, contrastes). |
| NFR-ORG-12 | Responsive | Le tableau de bord et les listes (éditions, exposants) sont utilisables sur tablette ; les actions de configuration complexes restent optimisées desktop. |

### 3.5 Maintenabilité et évolutivité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-ORG-13 | Multi-éditions | Le système supporte au moins 50 éditions par organisateur et 10 000 lignes (exposants, budget, programme) par édition sans dégradation majeure. |
| NFR-ORG-14 | Évolutivité | L’ajout de nouvelles éditions ou de nouveaux champs (exposant, édition) ne nécessite pas de refonte de l’architecture. |

---

## 4. Parcours détaillés et scénarios

### 4.1 Scénario : Premier usage — création de compte et première édition

1. L’utilisateur arrive sur la plateforme (depuis le catalogue ou un lien dédié).
2. Il clique sur « Créer un compte organisateur ».
3. Il remplit le formulaire (email, mot de passe, nom de la structure, type de structure).
4. Il reçoit un email de validation (si configuré) et valide son compte.
5. Après validation (automatique ou manuelle par la plateforme), il accède au tableau de bord organisateur (vide).
6. Il clique sur « Créer une édition ».
7. Il saisit les métadonnées (nom du festival, dates, lieu, thème) et enregistre.
8. L’édition est créée en statut « Brouillon » ; il accède au dashboard de cette édition.
9. Il configure les paramètres (règlement, contrats types), ouvre les candidatures exposants, configure le plan de salle et le programme selon ses besoins.
10. Il demande la publication au catalogue ; après validation, l’édition apparaît dans l’annuaire des événements.

**Besoins couverts** : ORG-01 à ORG-04, ORG-06, ORG-08, ORG-09, ORG-32, ORG-43.

### 4.2 Scénario : Gestion des candidatures et facturation

1. L’organisateur se connecte et sélectionne une édition en cours.
2. Il accède au module « Exposants » ; il voit la liste des candidatures en attente.
3. Il ouvre une candidature, consulte la fiche et les pièces jointes.
4. Il valide la candidature ; l’exposant reçoit une notification.
5. Il génère un devis (lignes : emplacement, options) et l’envoie à l’exposant.
6. L’exposant accepte le devis (depuis son dashboard).
7. L’organisateur convertit le devis en facture ; la facture est envoyée à l’exposant.
8. Il attribue un emplacement (plan de salle) à l’exposant.
9. Lorsque l’exposant paie, le statut de la facture est mis à jour et le revenu est enregistré dans le budget (si synchronisation activée).

**Besoins couverts** : ORG-12 à ORG-17, ORG-19 à ORG-20, ORG-28, ORG-31.

### 4.3 Scénario : Multi-festivals — comparaison et réutilisation

1. L’organisateur gère déjà 2 éditions (Festival A 2025, Festival B 2025).
2. Il crée une nouvelle édition « Festival A 2026 » en dupliquant « Festival A 2025 ».
3. Il choisit de dupliquer : paramètres, plan de salle, programme type, documents types.
4. La nouvelle édition est créée en brouillon ; il ajuste les dates et le lieu.
5. Depuis la liste globale des éditions, il filtre par « À venir » et voit Festival A 2026 et Festival B 2026.
6. Il consulte le rapport budget de Festival A 2025 (éditions passées) pour comparer avec la prévision 2026.

**Besoins couverts** : ORG-04, ORG-06, ORG-07, ORG-10, ORG-11, ORG-30.

### 4.4 Scénario : Activation des services visiteur

1. L’organisateur ouvre une édition et accède à « Services visiteur ».
2. Il active « Inscriptions ateliers » et « Concours ».
3. Il configure les créneaux d’ateliers (salle, capacité, horaires) et crée un concours (règles, dates, récompenses).
4. Il publie le programme public ; les visiteurs voient les ateliers et le concours sur la fiche événement et peuvent s’inscrire depuis leur espace.
5. Il consulte les inscriptions ateliers et les participants au concours depuis son dashboard édition.

**Besoins couverts** : ORG-38 à ORG-41, ORG-27.

---

## 5. Pain points et opportunités

### 5.1 Pain points (problèmes actuels ou anticipés)

| Pain point | Impact | Besoin associé |
|------------|--------|-----------------|
| **Outils dispersés** | Tableurs, email, outils de planification séparés ; doublons de saisie, risque d’erreur. | Centralisation (tableau de bord, tous modules dans un même espace). |
| **Manque de visibilité sur l’avancement** | Difficile de savoir où en est la préparation (candidatures, plan, budget). | Tableau de bord par édition avec indicateurs (ORG-08). |
| **Reporting manuel** | Export et consolidation manuels pour rendre des comptes. | Rapports et exports (ORG-30, ORG-31). |
| **Délégation insuffisante** | Un seul responsable accède à tout ; pas de délégation fine. | Rôles et assignation par édition (ORG-46 à ORG-48). |
| **Duplication des configs** | Réécrire les paramètres pour chaque nouvelle édition. | Duplication d’édition (ORG-10). |
| **Communications éparpillées** | Emails et messages non centralisés avec les exposants. | Notifications et annonces (ORG-35, ORG-36). |
| **Risque d’erreurs sur créneaux ou emplacements** | Chevauchements, doubles attributions. | Gestion des chevauchements programme (ORG-24), attribution plan (ORG-20). |

### 5.2 Opportunités

| Opportunité | Description | Besoin associé |
|-------------|-------------|-----------------|
| **Vue consolidée multi-festivals** | Un seul écran pour tous les festivals ; comparaison et tendances. | ORG-07, ORG-30. |
| **Réutilisation et templates** | Dupliquer une édition, réutiliser contrats types et plan type. | ORG-10, ORG-32. |
| **Automatisation facturation** | Devis → facture → suivi paiement → revenus budget. | ORG-16, ORG-17, ORG-31. |
| **Services visiteur différenciants** | Proposer jeux, concours, ateliers pour fidéliser et animer. | ORG-38 à ORG-42. |
| **Visibilité catalogue** | Publication dans l’annuaire pour augmenter la visibilité des événements. | ORG-43 à ORG-45. |

---

## 6. Priorisation des besoins (MoSCoW)

### 6.1 Must have (indispensable)

- ORG-01 à ORG-05 (onboarding, compte cross-événements).
- ORG-06 à ORG-09 (création édition, liste, dashboard, paramétrage).
- ORG-12 à ORG-15 (annuaire exposants, candidatures, validation, fiche).
- ORG-16, ORG-17 (devis, facture).
- ORG-19 à ORG-21 (plan de salle : zones, attribution, visualisation).
- ORG-23 à ORG-25 (programme : création, chevauchements, vues).
- ORG-28, ORG-29 (budget : saisie, ventilation).
- ORG-32, ORG-33 (documents : contrats types, envoi).
- ORG-43, ORG-44 (publication édition, fiche organisateur).
- ORG-46 (gestion des rôles).
- NFR-ORG-06 à NFR-ORG-09 (sécurité, permissions, isolation, audit).

### 6.2 Should have (important)

- ORG-10 (duplication édition).
- ORG-11 (clôture et archivage).
- ORG-18 (import exposants CSV).
- ORG-22 (export plan).
- ORG-26, ORG-27 (modification programme, publication programme public).
- ORG-30, ORG-31 (balance, lien facturation).
- ORG-34 (historique documents).
- ORG-35 à ORG-37 (annonces, notifications, paramétrage).
- ORG-38, ORG-41 (activation services visiteur, ateliers).
- ORG-45 (visibilité exposants répertoire).
- ORG-47, ORG-48 (bénévoles, délégation par édition).
- NFR-ORG-01 à NFR-ORG-05, NFR-ORG-10 à NFR-ORG-12 (performance, dispo, utilisabilité, accessibilité).

### 6.3 Could have (souhaitable)

- ORG-39, ORG-40, ORG-42 (jeux, concours, pass VIP).
- NFR-ORG-13, NFR-ORG-14 (multi-éditions, évolutivité).

### 6.4 Won’t have (hors périmètre ou report)

- Besoins spécifiques à d’autres publics (exposants, visiteurs) — traités dans leurs documents.
- Fonctionnalités avancées (BI, prédictif) — hors périmètre v1.

---

## 7. Dépendances et interfaces avec les autres publics

### 7.1 Dépendances

| Dépendance | Description |
|------------|-------------|
| **Exposants** | Les organisateurs dépendent des candidatures et des fiches exposants ; les exposants dépendent des décisions (validation, facturation) des organisateurs. |
| **Visiteurs** | Les organisateurs activent les services visiteur ; les visiteurs consomment ces services (jeux, concours, ateliers, réservations). |
| **Catalogue (utilisateur non connecté)** | Les organisateurs publient les éditions et la fiche organisateur ; le catalogue expose ces données en lecture seule. |
| **Plateforme** | Mandat de Permission (StrongFather), permissions (Master Butler), persistance (KindMother), sécurité (WorrySentinel). |

### 7.2 Interfaces

| Interface | Flux | Besoin organisateur |
|-----------|------|----------------------|
| Organisateur → Exposant | Validation candidature, envoi devis/facture, envoi documents, attribution emplacement. | ORG-14, ORG-16, ORG-17, ORG-33, ORG-20. |
| Organisateur → Visiteur | Activation services (jeux, concours, ateliers, pass) ; publication programme public. | ORG-38 à ORG-42, ORG-27. |
| Organisateur → Catalogue | Publication édition, fiche organisateur, visibilité exposants. | ORG-43 à ORG-45. |
| Exposant → Organisateur | Dépôt candidature, acceptation devis, paiement facture, envoi documents signés. | Côté exposant ; organisateur reçoit et traite. |

---

## 8. Glossaire et références

### 8.1 Glossaire (extrait)

| Terme | Définition |
|-------|------------|
| **Édition** | Occurrence d’un événement ou festival (dates, lieu, paramètres) ; une même manifestation peut avoir plusieurs éditions (ex. Festival X 2025, Festival X 2026). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée (StrongFather) permettant à l’organisateur d’agir dans le périmètre autorisé. |
| **Dashboard édition** | Vue synthétique par édition : indicateurs et accès aux modules (exposants, plan, programme, budget, documents). |
| **Catalogue** | Annuaire des événements + répertoire des organisateurs + répertoire des exposants (Store) ; Façade publique gouvernée. |

### 8.2 Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
- [Organisateurs — Parcours, capacités et livrables](./Organisateurs%20-%20Parcours%20Capacites%20Livrables.md)
- [Public Exposants](../Exposants/_index.md) | [Public Visiteurs](../Visiteurs/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)

---

## 9. User stories (format standard)

Les user stories ci-dessous reprennent les besoins fonctionnels en format « En tant que… je veux… afin de… » avec critères d’acceptation détaillés.

### 9.1 Onboarding et compte

- **US-ORG-01** — En tant qu’**organisateur**, je veux **créer un compte** (email, mot de passe, informations structure) **afin de** accéder à l’espace organisateur et gérer mes éditions.  
  *Critères* : Formulaire dédié ; validation email si configurée ; création profil (Miyauth, Miyuprofile).*

- **US-ORG-02** — En tant qu’**organisateur**, je veux **gérer plusieurs éditions depuis un seul compte** **afin de** ne pas multiplier les comptes et avoir une vue consolidée.  
  *Critères* : Liste de toutes mes éditions (passées, en cours, à venir) ; création d’une nouvelle édition depuis le même compte.*

- **US-ORG-03** — En tant qu’**Admin organisateur**, je veux **inviter des membres** (Manager, Bénévole) **afin de** déléguer la gestion de tout ou partie des éditions.  
  *Critères* : Flux d’invitation par email ; attribution du rôle ; assignation à une ou plusieurs éditions pour un Manager.*

### 9.2 Éditions

- **US-ORG-04** — En tant qu’**organisateur**, je veux **créer une nouvelle édition** (nom, dates, lieu, thème) **afin de** préparer un nouvel événement.  
  *Critères* : Formulaire de création ; édition en statut brouillon ; accès au dashboard édition.*

- **US-ORG-05** — En tant qu’**organisateur**, je veux **dupliquer une édition existante** **afin de** réutiliser les paramètres, le plan et le programme type.  
  *Critères* : Option « Dupliquer » ; choix des éléments à dupliquer ; nouvelle édition en brouillon.*

- **US-ORG-06** — En tant qu’**organisateur**, je veux **voir un tableau de bord par édition** (exposants, candidatures, budget, programme, plan) **afin de** avoir une vue d’ensemble de l’avancement.  
  *Critères* : Indicateurs synthétiques ; liens vers chaque module ; mise à jour en temps réel ou à l’actualisation.*

### 9.3 Exposants et facturation

- **US-ORG-07** — En tant qu’**organisateur**, je veux **recevoir et traiter les candidatures exposants** (validation, refus, motif) **afin de** constituer la liste des exposants de l’édition.  
  *Critères* : Liste des candidatures en attente ; fiche détail ; actions Valider / Refuser ; notification à l’exposant.*

- **US-ORG-08** — En tant qu’**organisateur**, je veux **générer un devis puis une facture** pour un exposant **afin de** encaisser les montants et suivre les paiements.  
  *Critères* : Création devis ; envoi à l’exposant ; conversion en facture ; suivi statut paiement ; alimentation budget si configuré.*

- **US-ORG-09** — En tant qu’**organisateur**, je veux **importer une liste d’exposants** (CSV) **afin de** ne pas tout saisir manuellement.  
  *Critères* : Upload fichier ; mapping colonnes ; prévisualisation ; création ou mise à jour des fiches.*

### 9.4 Plan de salle et programme

- **US-ORG-10** — En tant qu’**organisateur**, je veux **définir le plan de salle** (zones, stands) et **attribuer les emplacements aux exposants** **afin de** organiser l’implantation.  
  *Critères* : Création/édition du plan ; attribution (formulaire ou drag & drop) ; conflit si stand déjà attribué ; export visuel.*

- **US-ORG-11** — En tant qu’**organisateur**, je veux **créer le programme** (animations, salles, horaires) avec **alerte en cas de chevauchement** **afin de** éviter les conflits de créneaux.  
  *Critères* : Création animation ; association salle/créneau ; alerte ou blocage si conflit ; vues chronologique et par salle.*

### 9.5 Budget et documents

- **US-ORG-12** — En tant qu’**organisateur**, je veux **saisir les revenus et dépenses** et **consulter la balance** par édition **afin de** piloter le budget.  
  *Critères* : Saisie par catégorie ; ventilation ; balance ; lien optionnel avec facturation (Miyuinvoice).*

- **US-ORG-13** — En tant qu’**organisateur**, je veux **stocker et envoyer des documents** (contrats types, règlements) aux exposants **afin de** formaliser les engagements.  
  *Critères* : Upload et versioning ; envoi par email ; suivi (envoyé, ouvert, complété).*

### 9.6 Publication et services visiteur

- **US-ORG-14** — En tant qu’**organisateur**, je veux **publier mon édition dans l’annuaire** et **ma fiche dans le répertoire des organisateurs** **afin de** augmenter la visibilité.  
  *Critères* : Action « Publier au catalogue » ; workflow de validation ; visibilité dans l’annuaire et le répertoire.*

- **US-ORG-15** — En tant qu’**organisateur**, je veux **activer des services visiteurs** (jeux, concours, ateliers, pass VIP) par édition **afin de** enrichir l’expérience des visiteurs.  
  *Critères* : Liste des services ; activation/désactivation par édition ; paramétrage (places, dates, publics).*

---

## 10. Cas limites et règles métier

### 10.1 Règles métier

| Règle | Description |
|-------|-------------|
| **Mandat** | Un organisateur ne peut créer des éditions, gérer des exposants ou publier au catalogue que dans le cadre de son Mandat de Permission (StrongFather, Master Butler). |
| **Isolation** | Les données d’un organisateur (éditions, exposants, budget, documents) ne sont accessibles qu’à lui et à son équipe (rôles assignés) ; pas d’accès aux données des autres organisateurs. |
| **Rôle Manager** | Un Manager ne voit que les éditions auxquelles il est assigné ; un Admin voit toutes les éditions de la structure. |
| **Publication** | Une édition ne peut être publiée au catalogue que si elle respecte les critères plateforme (dates, champs obligatoires, etc.) ; la publication peut être soumise à validation manuelle. |
| **Clôture** | Une édition clôturée passe en lecture seule ; les données sont conservées pour historique et reporting. |

### 10.2 Cas limites

| Cas | Comportement attendu |
|-----|----------------------|
| **Candidature exposant sur une édition clôturée** | Impossible : les candidatures sont fermées pour les éditions clôturées. |
| **Attribution d’un stand déjà attribué** | Alerte ou blocage ; l’organisateur doit choisir un autre stand ou libérer le stand. |
| **Chevauchement d’animations (même salle, même créneau)** | Alerte ou blocage à la saisie ; suggestion de créneaux libres. |
| **Publication d’une édition sans dates ou sans lieu** | Blocage ou alerte : champs obligatoires pour la publication. |
| **Suppression d’une édition avec des exposants validés** | Blocage ou confirmation forte : les données exposants et facturation sont impactées ; proposer l’archivage plutôt que la suppression. |
| **Révoquer le Mandat d’un organisateur** | Les éditions et données restent accessibles en lecture seule pour audit ; l’organisateur ne peut plus modifier ni publier. |

### 10.3 Métriques de succès (indicateurs produit)

| Métrique | Description | Cible (exemple) |
|----------|-------------|------------------|
| **Taux d’activation** | % d’organisateurs ayant créé au moins une édition après inscription. | > 80 % |
| **Temps moyen jusqu’à première publication** | Délai entre création de compte et première publication d’une édition au catalogue. | < 30 jours |
| **Nombre d’éditions par organisateur** | Moyenne et médiane du nombre d’éditions gérées par organisateur (multi-festivals). | Suivi ; objectif croissance |
| **Taux d’utilisation des services visiteur** | % d’éditions avec au moins un service visiteur activé (jeux, concours, ateliers). | > 50 % |
| **Satisfaction organisateur** | Score NPS ou enquête satisfaction (facilité d’usage, complétude). | Suivi annuel |

---

**Document** : Organisateurs — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Analyse produit — référence pour le public Organisateurs
