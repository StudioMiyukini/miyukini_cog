# JayKoa — Écrans et UI

## Contexte

Ce document décrit l'ensemble des **écrans, zones et composants UI** du Service JayKoa, calqués sur le modèle de **Google Agenda**. L'objectif est de fournir un référentiel visuel et fonctionnel complet, destiné à être adapté aux spécificités de l'écosystème Miyukini dans une phase ultérieure.

JayKoa est le calendrier universel du COG. Son interface utilisateur doit offrir une expérience comparable à Google Agenda : fluide, claire, multi-vue, multi-agenda, avec une navigation temporelle intuitive.

## Portée / Scope

- **Applicable à :** Définition des écrans, zones, composants et comportements UI de JayKoa
- **Audience :** Équipes produit, UX/UI, développement
- **Statut :** Document de référence — calque Google Agenda à adapter

### Hors périmètre

- Maquettes pixel-perfect ou design system final
- Choix de framework ou de librairie UI
- Spécifications techniques d'implémentation

---

## 1. Structure générale de l'écran

L'écran principal de JayKoa reprend la disposition de Google Agenda :

```
+------------------------------------------------------------------+
|                        BARRE D'EN-TÊTE                           |
+----------+-------------------------------------------------------+
|          |                                                       |
| PANNEAU  |                                                       |
| LATÉRAL  |              ZONE PRINCIPALE                          |
| GAUCHE   |              (GRILLE / VUE)                           |
|          |                                                       |
|          |                                                       |
|          |                                                       |
+----------+-------------------------------------------------------+
```

| Zone | Rôle | Référence Google Agenda |
|------|------|------------------------|
| **Barre d'en-tête** | Navigation temporelle, sélecteur de vue, recherche, paramètres, profil utilisateur | Barre supérieure |
| **Panneau latéral gauche** | Bouton de création, mini-calendrier, liste des agendas avec filtrage par couleur | Sidebar gauche |
| **Zone principale** | Affichage de la vue sélectionnée (Jour, Semaine, Mois, Planning) avec les événements positionnés | Zone centrale |

---

## 2. Barre d'en-tête (Header)

La barre d'en-tête est permanente et visible sur tous les écrans.

### 2.1 Composition

| Élément | Position | Comportement | Référence Google |
|---------|----------|-------------|------------------|
| **Menu hamburger** | Extrême gauche | Ouvre / réduit le panneau latéral gauche | Menu hamburger |
| **Logo et nom** | Gauche, après le menu | Affiche le logo JayKoa et le nom "Agenda" | Logo Google Agenda |
| **Bouton "Aujourd'hui"** | Centre-gauche | Ramène la vue à la date du jour | Bouton "Aujourd'hui" |
| **Flèches de navigation** | Après "Aujourd'hui" | Recule / avance d'une période (jour, semaine, mois selon la vue active) | Flèches < > |
| **Libellé de période** | Après les flèches | Affiche le mois, l'année et le numéro de semaine (ex. "Février 2026 — Semaine 6") | Libellé "Février 2026 Semaine 6" |
| **Recherche** | Centre-droit | Ouvre un champ de recherche pour trouver des événements par titre, description ou lieu | Icône loupe |
| **Paramètres** | Droite | Ouvre les paramètres de l'agenda (fuseau horaire, format d'heure, début de semaine, notifications) | Icône engrenage |
| **Sélecteur de vue** | Droite | Menu déroulant : Jour, Semaine, Mois, Année, Planning (Agenda) | Dropdown "Semaine" |
| **Bouton Actualiser** | Droite | Force le rafraîchissement des données (synchronisation manuelle) | Bouton "Actualiser" |
| **Avatar utilisateur** | Extrême droite | Profil utilisateur, bascule entre comptes si multi-utilisateur | Avatar / photo |

### 2.2 Comportements spécifiques

- Le **libellé de période** s'adapte à la vue active :
  - Vue Jour : "Vendredi 6 février 2026"
  - Vue Semaine : "Février 2026 — Semaine 6"
  - Vue Mois : "Février 2026"
  - Vue Année : "2026"
  - Vue Planning : "Février 2026"
- Les **flèches de navigation** déplacent la vue d'une unité de la période active
- Le bouton **"Aujourd'hui"** est estompé si la vue inclut déjà la date du jour

---

## 3. Panneau latéral gauche (Sidebar)

Le panneau latéral gauche est rétractable via le menu hamburger.

### 3.1 Bouton de création

| Élément | Comportement |
|---------|-------------|
| **Bouton "+" / "Créer"** | Gros bouton circulaire ou rectangulaire en haut du panneau. Ouvre le formulaire de création rapide d'un événement. Équivalent du bouton "+" de Google Agenda |

### 3.2 Mini-calendrier (Date Picker)

| Élément | Comportement |
|---------|-------------|
| **Mini-calendrier mensuel** | Grille compacte du mois en cours. Navigation mois par mois (flèches). Clic sur un jour = navigation vers cette date dans la vue principale. Le jour courant est mis en surbrillance (cercle bleu). Les jours contenant des événements peuvent être marqués d'un point |

### 3.3 Liste des agendas

La liste des agendas affiche tous les agendas de l'utilisateur avec un système de filtrage par activation/désactivation :

| Élément | Comportement |
|---------|-------------|
| **Section "Mes agendas"** | Liste des agendas personnels de l'utilisateur. Chaque agenda possède un nom, une couleur et une case à cocher |
| **Case à cocher par agenda** | Active ou désactive l'affichage des événements de cet agenda dans la vue principale |
| **Pastille de couleur** | Identifie visuellement l'agenda. Les événements de cet agenda utilisent cette couleur dans la grille |
| **Section "Autres agendas"** | Agendas partagés, abonnements, jours fériés. Même logique de case à cocher et couleur |
| **Section "Services synchronisés"** | Propre à JayKoa : affiche les sources de synchronisation inter-Services (JayFestival, JayRDV, etc.) avec case à cocher et couleur dédiée |

### 3.4 Exemple de liste d'agendas JayKoa

```
MES AGENDAS
  [x] Personnel                    (bleu)
  [x] Professionnel                (vert)
  [ ] Sport                        (orange)

SERVICES SYNCHRONISÉS
  [x] JayRDV — Mes rendez-vous     (cyan)
  [x] JayFestival — Mes festivals   (violet)
  [ ] JayFestival — Événements publics (gris)

AUTRES AGENDAS
  [x] Jours fériés — France         (vert clair)
  [ ] Anniversaires                 (jaune)
```

---

## 4. Zone principale — Vues calendrier

La zone principale affiche la vue sélectionnée. JayKoa propose les mêmes vues que Google Agenda.

### 4.1 Vue Semaine (vue par défaut)

C'est la vue visible dans la capture de référence.

| Élément | Description |
|---------|-------------|
| **En-tête de colonnes** | Jours de la semaine en cours : abréviation du jour + numéro (ex. "LUN. 2", "MAR. 3", etc.). Le jour courant est mis en surbrillance (cercle coloré sur le numéro) |
| **Fuseau horaire** | Affiché en haut à gauche de la grille (ex. "GMT+01"). Clic pour changer de fuseau |
| **Zone journée entière** | Bande horizontale en haut de la grille, sous les en-têtes de colonnes. Affiche les événements "journée entière" sous forme de bandeaux colorés horizontaux. Si plus de 2-3 événements, un lien "X autres" permet de déplier |
| **Grille horaire** | Colonnes par jour, lignes par heure (00:00 à 23:00). Les demi-heures sont marquées par une ligne fine |
| **Ligne de l'heure actuelle** | Ligne horizontale rouge avec un point rouge, indiquant l'heure en temps réel. Se déplace au fil du temps |
| **Événements temporels** | Blocs rectangulaires colorés positionnés dans la grille selon leur plage horaire. La hauteur du bloc est proportionnelle à la durée. Le titre et l'horaire sont inscrits dans le bloc |
| **Événements simultanés** | Si plusieurs événements se chevauchent sur le même créneau, ils se partagent la largeur de la colonne (côte à côte ou superposés avec décalage) |

### 4.2 Vue Jour

| Élément | Description |
|---------|-------------|
| **En-tête** | Jour unique affiché (ex. "VEN. 6"). Jour courant en surbrillance |
| **Zone journée entière** | Identique à la vue Semaine mais pour un seul jour |
| **Grille horaire** | Une seule colonne, plus large. Lignes par heure et demi-heure. Même principe de positionnement des événements |
| **Niveau de détail** | Plus d'espace pour chaque événement : titre complet, lieu, description courte visible directement dans le bloc |

### 4.3 Vue Mois

| Élément | Description |
|---------|-------------|
| **Grille mensuelle** | 5 ou 6 lignes de 7 colonnes (Lun à Dim). Chaque cellule représente un jour |
| **Contenu des cellules** | Numéro du jour + liste des événements (titre court, pastille de couleur). Si trop d'événements dans une cellule, un lien "+X autres" ouvre une vue détaillée de la journée |
| **Jours hors mois** | Les jours des mois précédent et suivant sont estompés |
| **Jour courant** | Numéro en surbrillance (cercle coloré) |

### 4.4 Vue Année

| Élément | Description |
|---------|-------------|
| **12 mini-calendriers** | Grille de 12 mois (3 colonnes x 4 lignes ou 4 x 3). Chaque mois est un mini-calendrier compact. Les jours contenant des événements sont marqués |
| **Navigation** | Clic sur un mois = bascule vers la Vue Mois pour ce mois. Clic sur un jour = bascule vers la Vue Jour |

### 4.5 Vue Planning (Liste / Agenda)

| Élément | Description |
|---------|-------------|
| **Liste chronologique** | Événements listés dans l'ordre chronologique, regroupés par jour. Chaque jour est un séparateur avec la date complète |
| **Carte d'événement** | Pour chaque événement : pastille de couleur, titre, horaire (début — fin), lieu si renseigné, agenda source |
| **Défilement infini** | Scroll vers le bas pour voir les jours suivants, vers le haut pour les précédents |
| **Jours vides** | Les jours sans événement ne sont pas affichés (ou affichés avec la mention "Aucun événement") |

---

## 5. Événements — Affichage et représentation

### 5.1 Types d'affichage des événements

| Type | Affichage | Exemple |
|------|-----------|---------|
| **Événement temporel** | Bloc rectangulaire coloré positionné dans la grille horaire. Hauteur proportionnelle à la durée. Titre + horaire inscrits | "Psychologue — De 14:45 à 15:45" (bloc bleu) |
| **Événement journée entière** | Bandeau horizontal coloré dans la zone "journée entière", en haut de la grille | "Anniversaire Raymond" (bandeau saumon), "Anniversaire Marine" (bandeau vert) |
| **Événement multi-jours** | Bandeau horizontal étendu sur plusieurs colonnes/jours dans la zone "journée entière" | Un festival du 5 au 8 février |
| **Événement court (< 30 min)** | Bloc fin avec titre tronqué. Le détail est accessible au clic (popover) | Rappel de 15 minutes |

### 5.2 Codage couleur

| Source / Agenda | Couleur type | Signification |
|----------------|-------------|---------------|
| **Agenda personnel** | Bleu | Événements créés par l'utilisateur |
| **Agenda professionnel** | Vert | Événements professionnels |
| **JayRDV — Rendez-vous** | Cyan | Rendez-vous synchronisés depuis JayRDV |
| **JayFestival — Festivals** | Violet | Événements synchronisés depuis JayFestival |
| **Anniversaires** | Jaune/ambre | Dates d'anniversaire |
| **Jours fériés** | Vert clair | Jours fériés officiels |
| **Personnalisé** | Au choix de l'utilisateur | Chaque agenda peut recevoir une couleur personnalisée |

### 5.3 Informations affichées dans le bloc événement

| Vue | Informations visibles dans le bloc |
|-----|-------------------------------------|
| **Vue Semaine** | Titre (tronqué si nécessaire), horaire (ex. "De 14:00 à 15:00") |
| **Vue Jour** | Titre complet, horaire, lieu (si renseigné) |
| **Vue Mois** | Pastille de couleur + titre court (tronqué) |
| **Vue Planning** | Pastille de couleur, titre complet, horaire, lieu, agenda source |

### 5.4 Événements synchronisés (spécifique JayKoa)

Les événements issus de Services synchronisés (JayRDV, JayFestival) possèdent des attributs visuels supplémentaires :

| Attribut | Affichage |
|----------|-----------|
| **Icône de source** | Petite icône du Service source dans le coin du bloc événement (icône JayRDV, icône JayFestival) |
| **Badge "Synchronisé"** | Indicateur visuel discret signalant que l'événement est en lecture réfléchie (non modifiable depuis JayKoa) |
| **Statut temporel** | Indicateur de statut : informatif (neutre), bloquant (marquage fort), annulé (barré/estompé), modifié (indicateur de changement) |

---

## 6. Popover de détail événement

Au clic sur un événement dans la grille, un **popover** (bulle flottante) s'affiche avec les informations détaillées. Calqué sur le comportement de Google Agenda.

### 6.1 Contenu du popover

| Élément | Description |
|---------|-------------|
| **Titre de l'événement** | En gras, en haut du popover |
| **Date et horaire** | Date complète, heure de début — heure de fin (ex. "Vendredi 6 février 2026, 14:00 — 15:00") |
| **Récurrence** | Si récurrent : "Tous les vendredis", "Chaque semaine", etc. |
| **Lieu** | Adresse ou nom du lieu, si renseigné |
| **Description** | Texte de description libre |
| **Agenda** | Nom de l'agenda contenant l'événement, avec pastille de couleur |
| **Source** | Pour les événements synchronisés : nom du Service source (JayRDV, JayFestival) avec icône |
| **Statut temporel** | Informatif, Bloquant, Annulé, Modifié |
| **Rappels** | Liste des rappels configurés (ex. "30 minutes avant", "1 jour avant") |
| **Participants** | Liste des participants si renseignés (nom ou référence) |

### 6.2 Actions du popover

| Action | Comportement | Disponibilité |
|--------|-------------|---------------|
| **Modifier** | Ouvre le formulaire d'édition complet | Événements internes uniquement |
| **Supprimer** | Supprime l'événement (avec confirmation) | Événements internes uniquement |
| **Ouvrir dans le Service source** | Redirige vers le Service source pour voir le détail métier complet | Événements synchronisés uniquement |
| **Fermer** | Ferme le popover | Toujours |
| **Voir dans l'agenda** | Ouvre la vue Jour centrée sur cet événement | Toujours |

Pour les événements **synchronisés** (JayRDV, JayFestival), les actions Modifier et Supprimer sont remplacées par un lien "Ouvrir dans [Service source]" car JayKoa ne modifie jamais les données métier d'un autre Service.

---

## 7. Formulaire de création d'événement — Détail complet

Le formulaire de création est le point d'entrée principal pour ajouter un événement dans JayKoa. Il reprend le modèle de Google Agenda avec un **formulaire rapide** (popover) et un **formulaire complet** (pleine page).

### 7.1 Formulaire rapide (Quick Add) — Popover de création

Accessible via :
- Le bouton "+" dans le panneau latéral gauche
- Un clic sur un créneau vide dans la grille
- Un clic-glisser sur un créneau vide (sélection de plage)

Le formulaire rapide s'affiche sous forme de **popover flottant** ancré au créneau cliqué ou centré sur l'écran.

#### Structure complète du popover (calqué sur Google Agenda)

```
+--------------------------------------------------+
| [≡]                                         [X]  |
|                                                  |
| Ajouter un titre                                 |
| ________________________________________________ |
|                                                  |
| [Événement]  Tâche  Planning des RDV [Nouveauté] |
|                                                  |
| ⏰ Samedi, 7 février  22:30 — 23:30              |
|    Fuseau horaire · Une seule fois               |
|                                                  |
| 👥 Ajouter des invités                           |
|                                                  |
| ─────────────────────────────────────────────── |
|                                                  |
| 📍 Ajouter un lieu                               |
|                                                  |
| ≡  Ajouter une description                      |
|                                                  |
| 📎 Ajouter une pièce jointe                     |
|                                                  |
| ─────────────────────────────────────────────── |
|                                                  |
| 📅 [Miyukini ▾]        [● ▾]                    |
|                                                  |
| 💼 [Occupé ▾]                                    |
|                                                  |
|           Autres options     [Enregistrer]       |
+--------------------------------------------------+
```

#### Description de chaque zone du popover

| # | Zone | Composant Atomic | Comportement |
|---|------|-----------------|-------------|
| 1 | **Barre supérieure** | Bouton icône menu (A03) + Bouton icône fermer (A03) | Le menu ouvre des options supplémentaires. Fermer annule la création |
| 2 | **Champ titre** | Input texte (A05) — placeholder "Ajouter un titre" | Focus automatique à l'ouverture. Texte libre. Champ principal |
| 3 | **Barre d'onglets** | Onglets (A11) : "Événement" (actif, fond bleu arrondi), "Tâche" (inactif), "Planning des rendez-vous" (inactif + Badge A12 "Nouveauté") | Clic sur un onglet change le contexte du formulaire. "Événement" est sélectionné par défaut |
| 4 | **Sélecteur date/heure** | Icône horloge (A16) + Label date "Samedi, 7 février" (A17) + Heures "22:30 — 23:30" (A17) + Sous-texte "Fuseau horaire · Une seule fois" (A17 Caption) | Clic sur la date ouvre le mini-calendrier. Clic sur les heures ouvre un dropdown par pas de 15 min. "Une seule fois" est cliquable pour ouvrir le sélecteur de récurrence (M20) |
| 5 | **Participants** | Icône personnes (A16) + Placeholder "Ajouter des invités" (A05) | Champ de saisie avec auto-complétion sur les contacts/utilisateurs |
| 6 | **Séparateur** | Divider (A19) | Séparation visuelle entre la section temporelle et la section détails |
| 7 | **Lieu** | Icône lieu/pin (A16) + Placeholder "Ajouter un lieu" (A05) | Texte libre avec suggestion optionnelle |
| 8 | **Description** | Icône description/lignes (A16) + Placeholder "Ajouter une description" (A06) | Zone de texte multiligne |
| 9 | **Pièce jointe** | Icône pièce jointe (A16) + Lien "Ajouter une pièce jointe" (A18) | Clic ouvre le sélecteur de fichier |
| 10 | **Séparateur** | Divider (A19) | Séparation entre détails et paramètres |
| 11 | **Sélecteur d'agenda** | Icône calendrier (A16) + Dropdown nom agenda (A10) + Pastille couleur (A13) + Dropdown couleur (A14) | Le dropdown liste les agendas internes uniquement. La pastille reflète la couleur. Le dropdown couleur permet de personnaliser |
| 12 | **Sélecteur de statut** | Icône valise (A16) + Dropdown "Occupé" / "Libre" (A10) | Définit si la plage est bloquante ou informative |
| 13 | **Barre d'actions** | Lien "Autres options" (A18) + Bouton "Enregistrer" (A01 bleu) | "Autres options" ouvre le formulaire complet. "Enregistrer" crée l'événement et ferme le popover |

#### Onglets du formulaire de création

| Onglet | Contenu | Statut |
|--------|---------|--------|
| **Événement** | Formulaire de création d'événement standard (tous les champs décrits ci-dessus) | Actif par défaut |
| **Tâche** | Formulaire de création de tâche : titre, date d'échéance (optionnelle), description, liste de tâches. Les tâches avec date apparaissent dans l'agenda | Disponible |
| **Planning des rendez-vous** | Création de créneaux de disponibilité que d'autres utilisateurs peuvent réserver. Titre du planning, durée des créneaux, plages de disponibilité, règles de récurrence | Disponible (badge "Nouveauté") |

### 7.2 Formulaire complet (Full Edit)

Accessible via "Autres options" depuis le formulaire rapide, ou en cliquant "Modifier" dans le popover de détail.

Le formulaire complet s'affiche en **pleine page** ou en **panneau large**.

#### Champs du formulaire complet

| # | Champ | Type | Description | Pré-remplissage |
|---|-------|------|-------------|-----------------|
| 1 | **Titre** | Input texte (A05) | Nom de l'événement | Vide ou titre du formulaire rapide |
| 2 | **Date de début** | Sélecteur de date | Date de début. Clic ouvre un calendrier | Date du créneau cliqué |
| 3 | **Heure de début** | Dropdown heure (A10) | Heure de début, par pas de 15 min | Heure du créneau cliqué |
| 4 | **Date de fin** | Sélecteur de date | Date de fin | Même que date de début |
| 5 | **Heure de fin** | Dropdown heure (A10) | Heure de fin | Début + 1h par défaut |
| 6 | **Journée entière** | Toggle (A08) | Active/désactive le mode journée entière. Si activé, les champs d'heure disparaissent | Désactivé |
| 7 | **Fuseau horaire** | Dropdown (A10) | Fuseau horaire de l'événement | Fuseau local (ex. Europe/Paris) |
| 8 | **Récurrence** | Dropdown + formulaire (M20 / O09) | "Une seule fois", "Tous les jours", "Toutes les semaines le [jour]", "Tous les mois", "Tous les ans", "Personnalisé..." | "Une seule fois" |
| 9 | **Lieu** | Input texte (A05) | Lieu ou adresse | Vide |
| 10 | **Description** | Textarea riche (A06) | Description avec mise en forme basique (gras, italique, listes) | Vide |
| 11 | **Pièce jointe** | Sélecteur fichier (M19) | Ajout de documents | Vide |
| 12 | **Participants** | Input multi-saisie (A05) + Lignes participant (M05) | Ajout de participants avec auto-complétion. Chaque participant ajouté apparaît en ligne avec avatar et nom | Vide |
| 13 | **Rappels** | Lignes de rappel (M06) multiples + Lien "Ajouter une notification" (A18) | Un ou plusieurs rappels configurables (délai + méthode). Par défaut : 1 rappel "30 minutes avant" | 1 rappel par défaut |
| 14 | **Agenda** | Dropdown (A10) + Pastille (A13) | Sélection de l'agenda cible (internes uniquement) | Agenda par défaut de l'utilisateur |
| 15 | **Couleur** | Sélecteur de couleur (A14) | Couleur personnalisée (grille de pastilles) | Couleur de l'agenda |
| 16 | **Statut** | Dropdown (A10) | "Occupé" / "Libre" | "Occupé" |
| 17 | **Visibilité** | Dropdown (A10) | "Par défaut" / "Public" / "Privé" | "Par défaut" |

#### Barre d'actions du formulaire complet

| Action | Composant | Comportement |
|--------|-----------|-------------|
| **Enregistrer** | Bouton primaire (A01) | Crée ou met à jour l'événement. Ferme le formulaire et retourne à la vue principale |
| **Autres options / Annuler** | Bouton secondaire (A02) | Annule les modifications et ferme le formulaire |

### 7.3 Popover de détail — Contenu détaillé (calqué sur Google Agenda)

Au clic sur un événement existant, le popover de détail s'affiche :

```
+--------------------------------------------------+
| [✏️] [🗑️]                                  [X]  |
|                                                  |
| CMP avec Thérésa dr Boudou                      |
|                                                  |
| (●) Sonia Mateo                                  |
|                                                  |
| ⏰ Mardi, 3 février                             |
|    De 14:00 à 15:00                              |
|                                                  |
| 👤 Créé par : Sonia Mateo                        |
|                                                  |
| ─────────────────────────────────────────────── |
|                                                  |
| 🔔 10 minutes avant, par e-mail          ▾      |
|                                                  |
| 🔔 10 minutes avant                      ▾      |
|                                                  |
| Ajouter une notification                         |
|                                                  |
|           Autres options     [Enregistrer]       |
+--------------------------------------------------+
```

#### Description de chaque zone du popover de détail

| # | Zone | Composant Atomic | Comportement |
|---|------|-----------------|-------------|
| 1 | **Barre d'actions en-tête** | Bouton icône modifier/crayon (A03) + Bouton icône supprimer/corbeille (A03) + Bouton icône fermer (A03) | Modifier ouvre le formulaire complet. Supprimer demande confirmation. Fermer ferme le popover |
| 2 | **Titre** | Label H2 (A17) | Titre de l'événement en gras |
| 3 | **Participant principal** | Ligne participant (M05) — avatar (A15) + nom (A17) | Affiché si des participants sont renseignés |
| 4 | **Date et heure** | Icône horloge (A16) + Label date complète (A17) + Label horaire (A17) | "Mardi, 3 février" + "De 14:00 à 15:00" |
| 5 | **Créateur** | Ligne créateur (M07) — icône personne (A16) + "Créé par : [Nom]" (A17) | Affiché si l'événement a un créateur distinct de l'utilisateur courant |
| 6 | **Séparateur** | Divider (A19) | Séparation entre les informations et les rappels |
| 7 | **Rappels** | Lignes de rappel (M06) — icône cloche (A16) + dropdown délai (A10) + dropdown méthode (A10) | Chaque rappel est modifiable directement dans le popover. Ex. "10 minutes avant, par e-mail", "10 minutes avant" |
| 8 | **Ajout rappel** | Lien "Ajouter une notification" (A18) | Clic ajoute une nouvelle ligne de rappel |
| 9 | **Barre d'actions** | Lien "Autres options" (A18) + Bouton "Enregistrer" (A01) | "Autres options" ouvre le formulaire complet. "Enregistrer" sauvegarde les modifications de rappels |

#### Variante : popover d'événement synchronisé

| Modification | Détail |
|-------------|--------|
| **Barre d'actions en-tête** | Les boutons Modifier et Supprimer sont **absents**. Remplacés par un bouton "Ouvrir dans [Service source]" (A03 avec icône lien externe) |
| **Badge source** | Badge (A12) sous le titre : "Synchronisé depuis JayRDV" ou "Synchronisé depuis JayFestival" |
| **Rappels** | Les rappels sont configurables localement dans JayKoa (l'utilisateur peut ajouter ses propres rappels sur un événement synchronisé) |
| **Barre d'actions bas** | Pas de bouton "Enregistrer" pour le contenu métier. Uniquement "Ouvrir dans [Service source]" |

### 7.4 Règles du formulaire de création

- Le formulaire de création n'est disponible que pour les **agendas internes** de JayKoa
- Il est **impossible** de créer un événement dans un agenda synchronisé (JayRDV, JayFestival) depuis JayKoa
- Si un **conflit temporel** est détecté avec un événement existant (chevauchement), un avertissement est affiché sans bloquer la création
- La récurrence personnalisée permet de définir : fréquence, intervalle, jours spécifiques, date de fin ou nombre d'occurrences, exceptions
- Le champ "Ajouter des invités" n'est pas une invitation Google : il ajoute des **références de participants** dans le modèle JayKoa
- Le champ "Pièce jointe" est optionnel et limité aux fichiers locaux ou documents du COG (pas de Google Drive)
- L'onglet "Planning des rendez-vous" est une fonctionnalité de création de créneaux de disponibilité, à adapter pour JayKoa (potentiellement lié à JayRDV via synchronisation)

### 7.5 Référence Design System Atomic

Tous les composants de cette section sont décrits en détail dans le document [JayKoa - Design System Atomic](./JayKoa%20-%20Design%20System%20Atomic.md), notamment :

| Organisme | ID | Description |
|-----------|-----|-------------|
| Formulaire de création rapide | O08 | Popover complet avec onglets, champs et actions |
| Formulaire d'édition complet | O10 | Formulaire pleine page avec tous les paramètres |
| Popover de détail événement | O11 | Popover de consultation avec actions et rappels |
| Formulaire de récurrence | O09 | Éditeur de récurrence personnalisée |

---

## 8. Formulaire de récurrence (détail)

Accessible via l'option "Personnalisé..." dans le champ Récurrence du formulaire complet.

| Champ | Description |
|-------|-------------|
| **Fréquence** | Quotidienne, Hebdomadaire, Mensuelle, Annuelle |
| **Intervalle** | Tous les X jours/semaines/mois/ans (ex. "Toutes les 2 semaines") |
| **Jours de la semaine** | Pour fréquence hebdomadaire : cases à cocher Lun, Mar, Mer, Jeu, Ven, Sam, Dim |
| **Jour du mois** | Pour fréquence mensuelle : "Le 15 de chaque mois" ou "Le 2e mardi de chaque mois" |
| **Fin de récurrence** | Jamais / Après X occurrences / À la date du... |

---

## 9. Écran des paramètres de l'agenda

Accessible via l'icône engrenage dans la barre d'en-tête.

### 9.1 Paramètres généraux

| Paramètre | Description | Valeur par défaut |
|-----------|-------------|-------------------|
| **Langue** | Langue de l'interface | Français |
| **Fuseau horaire** | Fuseau horaire principal | Fuseau local (ex. Europe/Paris, GMT+01) |
| **Afficher un fuseau secondaire** | Affiche un second fuseau en parallèle dans la grille | Désactivé |
| **Format d'heure** | 24h ou AM/PM | 24h |
| **Début de la semaine** | Lundi, Samedi ou Dimanche | Lundi |

### 9.2 Paramètres de vue

| Paramètre | Description | Valeur par défaut |
|-----------|-------------|-------------------|
| **Vue par défaut** | Vue affichée à l'ouverture de l'agenda | Semaine |
| **Jours visibles en vue Semaine** | 5 jours (Lun-Ven) ou 7 jours (Lun-Dim) | 7 jours |
| **Afficher les week-ends** | Afficher ou masquer Sam-Dim | Activé |
| **Afficher les événements refusés** | Afficher les événements refusés en estompé | Désactivé |
| **Densité de l'affichage** | Compacte / Normale / Confortable | Normale |

### 9.3 Paramètres de notifications

| Paramètre | Description | Valeur par défaut |
|-----------|-------------|-------------------|
| **Rappel par défaut** | Rappel automatique appliqué aux nouveaux événements | 30 minutes avant |
| **Rappel journée entière** | Rappel pour les événements journée entière | 1 jour avant |
| **Mode de notification** | Notification système / Email / Les deux | Notification système |

### 9.4 Gestion des agendas

| Action | Description |
|--------|-------------|
| **Créer un agenda** | Créer un nouvel agenda personnel (nom, couleur, description) |
| **Modifier un agenda** | Renommer, changer la couleur, modifier la description |
| **Supprimer un agenda** | Supprimer un agenda personnel et tous ses événements (avec confirmation) |
| **Gérer les abonnements** | Activer ou désactiver la synchronisation avec les Services COG (JayRDV, JayFestival) |
| **Partager un agenda** | Définir les règles de partage (qui peut voir, niveau de détail) |

---

## 10. Recherche

### 10.1 Barre de recherche

| Élément | Comportement |
|---------|-------------|
| **Champ de recherche** | Texte libre. Recherche dans : titres, descriptions, lieux, noms de participants |
| **Résultats** | Liste d'événements correspondants, triés par pertinence ou par date. Pour chaque résultat : titre, date, horaire, agenda, pastille de couleur |
| **Clic sur un résultat** | Navigation vers la date de l'événement dans la vue principale + ouverture du popover de détail |
| **Filtres de recherche** | Par période (de... à...), par agenda, par source synchronisée |

---

## 11. Glisser-déposer et interactions directes

### 11.1 Interactions dans la grille

| Interaction | Comportement |
|------------|-------------|
| **Clic sur un créneau vide** | Ouvre le formulaire de création rapide, pré-rempli avec la date/heure du créneau |
| **Clic-glisser sur un créneau vide** | Sélectionne une plage horaire et ouvre le formulaire de création rapide avec début/fin pré-remplis |
| **Clic sur un événement** | Ouvre le popover de détail |
| **Glisser un événement** | Déplace l'événement vers un autre créneau horaire ou un autre jour (réaffectation de date/heure). Uniquement pour les événements internes |
| **Redimensionner un événement** | Glisser le bord inférieur d'un bloc événement pour modifier la durée. Uniquement pour les événements internes |
| **Double-clic sur un événement** | Ouvre directement le formulaire d'édition complet. Uniquement pour les événements internes |

### 11.2 Restrictions pour les événements synchronisés

Les événements synchronisés depuis les Services COG (JayRDV, JayFestival) sont en **lecture réfléchie** :

- Pas de glisser-déposer
- Pas de redimensionnement
- Pas de modification directe
- Le double-clic ouvre le popover de détail avec l'option "Ouvrir dans le Service source"

---

## 12. Indicateurs visuels spécifiques

### 12.1 Ligne de l'heure actuelle

- Ligne horizontale rouge traversant la grille à l'heure exacte
- Point rouge au début de la ligne (côté gauche)
- Se déplace en temps réel
- Visible uniquement dans les vues Jour et Semaine

### 12.2 Indicateur de conflit temporel

Lorsque deux événements se chevauchent dans l'agenda d'un utilisateur :

| Situation | Affichage |
|-----------|-----------|
| **Chevauchement simple** | Les blocs événements se partagent la largeur de la colonne, côte à côte |
| **Conflit de présence physique** | En plus du partage de largeur : bordure rouge ou orange sur les événements en conflit, et un indicateur visuel "Conflit" |
| **Notification de conflit** | Bandeau ou badge en haut de la vue : "X conflits de présence physique à résoudre" |

### 12.3 Indicateurs de statut temporel (événements synchronisés)

| Statut | Affichage |
|--------|-----------|
| **Informatif** | Bloc avec couleur normale, sans marquage spécial |
| **Bloquant** | Bloc avec couleur pleine et bord marqué |
| **Annulé** | Bloc estompé, titre barré |
| **Modifié** | Petit badge "Modifié" ou icône de changement dans le coin du bloc |

---

## 13. Responsive et adaptations

### 13.1 Écran large (Desktop)

Disposition complète : barre d'en-tête + panneau latéral gauche + zone principale. Toutes les vues sont disponibles.

### 13.2 Écran moyen (Tablette)

- Le panneau latéral gauche est rétracté par défaut (accessible via le menu hamburger)
- La zone principale occupe toute la largeur
- Les vues Jour et Planning sont privilégiées

### 13.3 Écran petit (Mobile)

- Pas de panneau latéral visible par défaut
- La vue par défaut est Planning (liste) ou Jour
- Le mini-calendrier est accessible via un panneau coulissant depuis le haut
- Le formulaire de création est accessible via un bouton flottant (FAB) en bas à droite
- Les popovers de détail sont remplacés par des écrans plein-écran

---

## 14. Export et partage

### 14.1 Zone d'export

Accessible depuis les paramètres ou depuis un menu contextuel de l'agenda.

| Action | Comportement |
|--------|-------------|
| **Exporter en iCal** | Génère un fichier .ics contenant les événements de l'agenda sélectionné pour la période choisie |
| **Exporter en PDF** | Génère un document PDF avec la vue calendrier imprimable (Jour, Semaine ou Mois) |
| **Choix de la période** | Sélecteur : Semaine en cours, Mois en cours, Période personnalisée (du... au...) |
| **Choix des agendas** | Cases à cocher pour sélectionner quels agendas inclure dans l'export |

### 14.2 Partage d'agenda

| Action | Comportement |
|--------|-------------|
| **Partager un agenda** | Générer un lien de partage ou inviter des utilisateurs spécifiques |
| **Niveaux de partage** | Voir libre/occupé uniquement — Voir les détails — Modifier les événements |
| **Révoquer le partage** | Supprimer l'accès partagé à tout moment |

---

## 15. Synthèse des écrans

| Écran / Zone | Description | Référence Google |
|-------------|-------------|------------------|
| **Barre d'en-tête** | Navigation temporelle, vue, recherche, paramètres | Header bar |
| **Panneau latéral gauche** | Création, mini-calendrier, liste des agendas | Left sidebar |
| **Vue Jour** | Grille horaire sur 1 jour | Day view |
| **Vue Semaine** | Grille horaire sur 7 jours | Week view |
| **Vue Mois** | Grille mensuelle compacte | Month view |
| **Vue Année** | 12 mini-calendriers | Year view |
| **Vue Planning** | Liste chronologique des événements | Schedule/Agenda view |
| **Popover de détail** | Bulle flottante au clic sur un événement | Event detail popover |
| **Formulaire rapide** | Création rapide d'événement | Quick add popup |
| **Formulaire complet** | Édition complète d'un événement | Full event editor |
| **Paramètres** | Configuration de l'agenda | Settings page |
| **Recherche** | Recherche d'événements | Search overlay |
| **Export** | Export iCal / PDF | Export settings |

---

## 16. Références

| Document | Rôle |
|----------|------|
| [JayKoa - Document Fondateur](./JayKoa%20-%20Document%20Fondateur.md) | Positionnement et architecture du Service |
| [JayKoa - Parcours Utilisateurs](./JayKoa%20-%20Parcours%20Utilisateurs.md) | Parcours utilisateurs dans lesquels ces écrans s'insèrent |
| [JayKoa - Design System Atomic](./JayKoa%20-%20Design%20System%20Atomic.md) | Inventaire Atomic Design complet (Atomes, Molécules, Organismes) |
| [JayKoa - Referentiel Fonctionnel Inspire Google Agenda](./reference/JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | Correspondance fonctionnelle Google Agenda vers JayKoa |

---

**Document** : JayKoa — Écrans et UI
**Version** : 2.1
**Date** : 2026-02-06
**Statut** : Document de référence — calque Google Agenda à adapter
