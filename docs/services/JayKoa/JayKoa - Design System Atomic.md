# JayKoa — Design System Atomic

## Contexte

Ce document constitue l'**inventaire complet des composants UI** du Service JayKoa, organisé selon la méthodologie **Atomic Design** (Brad Frost). Chaque composant est classé en trois niveaux :

- **Atome** — Élément UI irréductible, indivisible (bouton, icône, champ texte, pastille, label)
- **Molécule** — Assemblage de quelques atomes formant un groupe fonctionnel (champ de formulaire, sélecteur de date, ligne de participant)
- **Organisme** — Assemblage de molécules et d'atomes formant une section complète d'interface (formulaire de création, barre d'en-tête, grille calendrier)

L'inventaire est calqué sur **Google Agenda** et sera adapté aux spécificités Miyukini ultérieurement.

## Portée / Scope

- **Applicable à :** Inventaire et description de chaque composant UI de JayKoa
- **Audience :** Équipes UX/UI, développement front-end, intégrateurs
- **Statut :** Document de référence — inventaire Atomic Design à adapter

---

# NIVEAU 1 — ATOMES

Les atomes sont les éléments UI les plus petits. Ils ne peuvent pas être décomposés davantage.

---

## A01 — Bouton primaire

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Action principale d'un écran ou d'un formulaire |
| **Apparence** | Fond coloré (bleu), texte blanc, coins arrondis |
| **États** | Normal, Hover, Actif, Désactivé |
| **Tailles** | Standard, Compact |
| **Exemples** | "Enregistrer", "Créer", "Télécharger" |

---

## A02 — Bouton secondaire

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Action secondaire, alternative ou annulation |
| **Apparence** | Fond transparent, texte bleu, sans bordure (style lien) |
| **États** | Normal, Hover, Actif, Désactivé |
| **Exemples** | "Autres options", "Annuler", "Ajouter une notification" |

---

## A03 — Bouton icône

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Action compacte représentée par une icône seule |
| **Apparence** | Cercle ou carré transparent, icône centrée. Fond coloré au hover |
| **États** | Normal, Hover, Actif, Désactivé |
| **Tailles** | Petit (24px), Standard (36px), Grand (48px) |
| **Exemples** | Fermer (X), Modifier (crayon), Supprimer (corbeille), Navigation (<, >) |

---

## A04 — Bouton flottant (FAB — Floating Action Button)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Action principale de l'écran, toujours visible |
| **Apparence** | Cercle coloré avec icône "+" centrée, ombre portée |
| **États** | Normal, Hover, Actif |
| **Usage** | Bouton de création d'événement dans le panneau latéral (calqué sur le "+" de Google Agenda) |

---

## A05 — Champ de texte (Input)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Saisie de texte libre par l'utilisateur |
| **Apparence** | Zone rectangulaire avec bordure fine, texte placeholder en gris |
| **États** | Vide (placeholder), Focus (bordure bleue), Rempli, Erreur (bordure rouge), Désactivé |
| **Variantes** | Ligne seule (titre), Underline (style Material) |
| **Exemples** | "Ajouter un titre", "Ajouter un lieu" |

---

## A06 — Zone de texte (Textarea)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Saisie de texte multiligne |
| **Apparence** | Zone rectangulaire extensible, placeholder en gris |
| **États** | Vide, Focus, Rempli, Désactivé |
| **Exemples** | "Ajouter une description" |

---

## A07 — Case à cocher (Checkbox)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Sélection binaire (activé/désactivé) |
| **Apparence** | Carré avec bordure. Cochée : fond coloré + coche blanche |
| **États** | Non cochée, Cochée, Indéterminée, Désactivée |
| **Exemples** | Activer/désactiver un agenda dans le panneau latéral, sélection d'agendas à exporter |

---

## A08 — Toggle / Interrupteur (Switch)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Bascule binaire (on/off) |
| **Apparence** | Rail horizontal avec curseur rond glissant. Activé : rail coloré. Désactivé : rail gris |
| **États** | Activé, Désactivé |
| **Exemples** | "Journée entière" (toggle dans le formulaire de création) |

---

## A09 — Radio bouton (Radio)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Choix exclusif parmi plusieurs options |
| **Apparence** | Cercle avec bordure. Sélectionné : cercle intérieur coloré |
| **États** | Non sélectionné, Sélectionné, Désactivé |
| **Exemples** | Choix de fin de récurrence ("Jamais" / "Après X occurrences" / "À la date du...") |

---

## A10 — Dropdown / Sélecteur (Select)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Choix parmi une liste d'options prédéfinies |
| **Apparence** | Champ avec texte de la valeur sélectionnée + flèche vers le bas. Menu déroulant au clic |
| **États** | Fermé, Ouvert (liste visible), Sélectionné, Désactivé |
| **Exemples** | Sélecteur de vue ("Semaine"), Sélecteur d'agenda ("Miyukini"), Statut ("Occupé"), Récurrence |

---

## A11 — Onglet (Tab)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Navigation entre sections d'un même contexte |
| **Apparence** | Texte horizontal. Actif : fond coloré arrondi ou soulignement. Inactif : texte gris |
| **États** | Actif, Inactif, Hover |
| **Exemples** | Onglets "Événement" / "Tâche" / "Planning des rendez-vous" dans le formulaire de création |

---

## A12 — Badge

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Indicateur visuel compact (compteur, statut, nouveauté) |
| **Apparence** | Petit rectangle arrondi avec texte court. Couleur de fond variable |
| **Variantes** | Compteur (chiffre), Statut ("Synchronisé"), Nouveauté ("Nouveauté"), Source ("JayRDV") |
| **Exemples** | Badge "Nouveauté" sur l'onglet "Planning des rendez-vous", Badge "Synchronisé depuis JayRDV" |

---

## A13 — Pastille de couleur (Color Dot)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Identifiant visuel d'un agenda ou d'une catégorie |
| **Apparence** | Petit cercle plein coloré (8-12px de diamètre) |
| **Variantes** | Toutes les couleurs de la palette (bleu, vert, cyan, violet, jaune, orange, rouge, gris, etc.) |
| **Exemples** | Pastille bleue à côté de "Miyukini" dans le sélecteur d'agenda, pastille dans la liste des agendas |

---

## A14 — Sélecteur de couleur (Color Picker)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Choix d'une couleur parmi la palette disponible |
| **Apparence** | Grille de pastilles de couleur cliquables. La couleur sélectionnée a un contour ou une coche |
| **États** | Fermé (pastille seule), Ouvert (grille visible) |
| **Exemples** | Sélecteur de couleur dans le formulaire de création (à côté du dropdown agenda) |

---

## A15 — Avatar

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Représentation visuelle d'un utilisateur |
| **Apparence** | Cercle avec photo de profil ou initiales sur fond coloré |
| **Tailles** | Petit (24px), Standard (32px), Grand (40px) |
| **Exemples** | Avatar de l'utilisateur dans le header, avatar du créateur dans le popover de détail ("Sonia Mateo") |

---

## A16 — Icône

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Représentation graphique compacte d'une action ou d'un concept |
| **Apparence** | Pictogramme monochrome (style Material Icons / Outlined) |
| **Tailles** | Petit (16px), Standard (20px), Grand (24px) |
| **Catalogue d'icônes JayKoa** | Voir section dédiée ci-dessous |

### Catalogue d'icônes

| Icône | Nom | Usage |
|-------|-----|-------|
| Horloge | `icon-clock` | Date et heure, fuseau horaire |
| Personnes | `icon-people` | Participants, invités |
| Lieu / Pin | `icon-location` | Lieu de l'événement |
| Description / Lignes | `icon-description` | Description textuelle |
| Pièce jointe | `icon-attachment` | Fichiers joints |
| Calendrier | `icon-calendar` | Sélecteur d'agenda |
| Valise / Occupé | `icon-briefcase` | Statut libre/occupé |
| Crayon | `icon-edit` | Modifier |
| Corbeille | `icon-delete` | Supprimer |
| Croix | `icon-close` | Fermer |
| Loupe | `icon-search` | Rechercher |
| Engrenage | `icon-settings` | Paramètres |
| Flèche gauche | `icon-chevron-left` | Navigation précédente |
| Flèche droite | `icon-chevron-right` | Navigation suivante |
| Menu hamburger | `icon-menu` | Ouvrir/fermer la sidebar |
| Plus | `icon-add` | Créer, ajouter |
| Cloche | `icon-notification` | Rappels, notifications |
| Récurrence | `icon-repeat` | Événement récurrent |
| Lien externe | `icon-external` | Ouvrir dans le Service source |
| Télécharger | `icon-download` | Export, téléchargement |
| Partager | `icon-share` | Partage d'agenda |
| Œil | `icon-visibility` | Visibilité (public/privé) |
| Conflit | `icon-warning` | Conflit temporel |
| Source JayRDV | `icon-jayrdv` | Événement synchronisé JayRDV |
| Source JayFestival | `icon-jayfestival` | Événement synchronisé JayFestival |
| Synchronisé | `icon-sync` | Événement en lecture réfléchie |

---

## A17 — Label / Texte

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Affichage de texte statique |
| **Variantes** | H1 (titre principal), H2 (titre section), H3 (sous-titre), Body (corps), Caption (légende), Overline (sur-titre) |
| **Exemples** | "Février 2026 — Semaine 6" (H2), "De 14:00 à 15:00" (Body), "Fuseau horaire · Une seule fois" (Caption), "LUN." (Overline) |

---

## A18 — Lien (Link)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Navigation ou action sous forme de texte cliquable |
| **Apparence** | Texte bleu, souligné au hover |
| **États** | Normal, Hover, Visité, Désactivé |
| **Exemples** | "Autres options", "Ajouter une notification", "Ajouter une pièce jointe" |

---

## A19 — Séparateur (Divider)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Séparation visuelle entre deux sections |
| **Apparence** | Ligne horizontale fine gris clair |
| **Variantes** | Pleine largeur, Indentée (avec marge gauche) |
| **Exemples** | Séparateur entre les sections du formulaire de création, entre les zones du popover |

---

## A20 — Ligne de grille horaire (Grid Line)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Marqueur temporel dans la grille calendrier |
| **Variantes** | Ligne heure (trait normal), Ligne demi-heure (trait fin pointillé), Ligne heure actuelle (trait rouge avec point) |
| **Exemples** | Lignes "13:00", "14:00", "15:00" dans la grille semaine |

---

## A21 — Numéro de jour (Day Number)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Affichage du numéro de jour dans les en-têtes de colonnes ou le mini-calendrier |
| **Apparence** | Chiffre centré. Jour courant : cercle bleu avec texte blanc. Jour sélectionné : cercle gris |
| **États** | Normal, Aujourd'hui (cercle bleu), Sélectionné, Hors mois (estompé) |
| **Exemples** | "6" (avec cercle bleu = aujourd'hui), "2", "3", "7", "8" |

---

## A22 — Étiquette de jour (Day Label)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Abréviation du jour de la semaine dans les en-têtes |
| **Apparence** | Texte court en majuscules, gris |
| **Exemples** | "LUN.", "MAR.", "MER.", "JEU.", "VEN.", "SAM.", "DIM." |

---

## A23 — Étiquette d'heure (Hour Label)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Marqueur d'heure dans la marge gauche de la grille |
| **Apparence** | Texte petit, gris, aligné avec la ligne de grille correspondante |
| **Exemples** | "11:00", "12:00", "13:00", "14:00" |

---

## A24 — Étiquette de fuseau (Timezone Label)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Affichage du fuseau horaire actif |
| **Apparence** | Texte compact en haut de la colonne d'heures |
| **Exemples** | "GMT+01" |

---

## A25 — Tooltip

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Information contextuelle au survol d'un élément |
| **Apparence** | Bulle compacte sombre avec texte blanc, apparaissant au hover |
| **Exemples** | Tooltip sur un bouton icône : "Paramètres", "Rechercher", "Semaine suivante" |

---

## A26 — Point d'événement (Event Dot)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Indicateur compact d'événement dans le mini-calendrier ou la vue mois |
| **Apparence** | Petit point coloré (3-4px) sous le numéro de jour |
| **Variantes** | Un point (1 événement), plusieurs points (plusieurs événements/couleurs) |

---

## A27 — Indicateur "Maintenant" (Now Marker)

| Propriété | Valeur |
|-----------|--------|
| **Rôle** | Marqueur de l'heure actuelle en temps réel dans la grille |
| **Apparence** | Point rouge (8px) + ligne rouge horizontale traversant toute la grille |
| **Comportement** | Se déplace en temps réel. Visible uniquement en Vue Jour et Vue Semaine |

---

# NIVEAU 2 — MOLÉCULES

Les molécules sont des assemblages fonctionnels de quelques atomes.

---

## M01 — Champ de formulaire (Form Field)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône (A16) + Label (A17) / Placeholder + Input (A05) |
| **Rôle** | Saisie d'une donnée avec contexte visuel |
| **Variantes** | Avec icône à gauche (style Google Agenda), Sans icône, Avec label au-dessus |
| **Exemples** | Icône horloge + "Samedi, 7 février 22:30 — 23:30", Icône lieu + "Ajouter un lieu" |

---

## M02 — Sélecteur de date et heure (DateTime Picker)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône horloge (A16) + Label date (A17) + Sélecteur heure début (A10) + Tiret + Sélecteur heure fin (A10) |
| **Rôle** | Choix complet de la plage temporelle d'un événement |
| **Sous-texte** | "Fuseau horaire · Une seule fois" (Caption A17) — cliquable pour modifier le fuseau ou la récurrence |
| **Comportement** | Clic sur la date ouvre un calendrier (mini-calendar). Clic sur l'heure ouvre une liste déroulante de créneaux (pas de 15 ou 30 min). Le sous-texte "Une seule fois" est cliquable et ouvre le sélecteur de récurrence |

---

## M03 — Sélecteur d'agenda (Calendar Selector)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône calendrier (A16) + Dropdown agenda (A10) + Pastille de couleur (A13) + Sélecteur de couleur (A14) |
| **Rôle** | Choix de l'agenda cible et de la couleur de l'événement |
| **Comportement** | Le dropdown liste les agendas internes de l'utilisateur. La pastille reflète la couleur de l'agenda sélectionné. Le sélecteur de couleur permet de personnaliser la couleur de l'événement |
| **Exemple** | Icône calendrier + "Miyukini" (dropdown) + pastille bleue + dropdown couleur |

---

## M04 — Sélecteur de statut (Status Selector)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône valise (A16) + Dropdown statut (A10) |
| **Rôle** | Définir si l'événement marque la plage comme occupée ou libre |
| **Options** | "Occupé", "Libre" |
| **Exemple** | Icône valise + "Occupé" (dropdown) |

---

## M05 — Ligne de participant (Participant Row)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Avatar (A15) + Label nom (A17) |
| **Rôle** | Affichage d'un participant dans le popover de détail ou le formulaire |
| **Variantes** | Avec statut de réponse (accepté/refusé/en attente), Avec bouton supprimer (A03) |
| **Exemple** | Avatar de Sonia Mateo + "Sonia Mateo" |

---

## M06 — Ligne de rappel (Reminder Row)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône cloche (A16) + Dropdown délai (A10) + Dropdown méthode (A10) |
| **Rôle** | Configuration d'un rappel pour un événement |
| **Options délai** | "5 minutes avant", "10 minutes avant", "15 minutes avant", "30 minutes avant", "1 heure avant", "1 jour avant", Personnalisé |
| **Options méthode** | "Notification", "Par e-mail" |
| **Exemples** | "10 minutes avant, par e-mail", "10 minutes avant" (notification) |

---

## M07 — Ligne de créateur (Creator Row)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône personne (A16) + Label "Créé par :" (A17) + Nom du créateur (A17) |
| **Rôle** | Affichage du créateur d'un événement dans le popover de détail |
| **Exemple** | Icône + "Créé par : Sonia Mateo" |

---

## M08 — Barre d'onglets (Tab Bar)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Onglet actif (A11) + Onglet(s) inactif(s) (A11) + Badge optionnel (A12) |
| **Rôle** | Navigation entre les types de création dans le formulaire |
| **Onglets JayKoa** | "Événement" (par défaut, actif), "Tâche", "Planning des rendez-vous" (+ badge "Nouveauté") |
| **Comportement** | Clic sur un onglet change le contenu du formulaire |

---

## M09 — Bouton de navigation temporelle (Time Navigation)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Bouton "Aujourd'hui" (A01/A02) + Bouton icône gauche (A03) + Bouton icône droite (A03) |
| **Rôle** | Naviguer dans le temps (période précédente, suivante, retour à aujourd'hui) |
| **Comportement** | Le bouton "Aujourd'hui" ramène à la date courante. Les flèches décalent d'une unité de la période active |

---

## M10 — Sélecteur de vue (View Selector)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Dropdown (A10) avec options de vues |
| **Options** | "Jour", "Semaine", "Mois", "Année", "Planning" |
| **Comportement** | Change la vue de la zone principale |

---

## M11 — Champ de recherche (Search Field)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône loupe (A16) + Input (A05) + Bouton fermer (A03) |
| **Rôle** | Recherche d'événements par titre, description ou lieu |
| **Comportement** | Résultats en temps réel pendant la saisie |

---

## M12 — Ligne d'agenda (Agenda Row)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Case à cocher (A07) + Pastille de couleur (A13) + Label nom agenda (A17) |
| **Rôle** | Élément de la liste des agendas dans le panneau latéral |
| **Comportement** | Cocher/décocher active/désactive l'affichage des événements de cet agenda |
| **Exemple** | [x] (bleu) Personnel |

---

## M13 — En-tête de colonne jour (Day Column Header)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Étiquette de jour (A22) + Numéro de jour (A21) |
| **Rôle** | En-tête d'une colonne de jour dans la grille semaine |
| **Exemples** | "LUN. 2", "MAR. 3", "VEN. 6" (avec cercle bleu si aujourd'hui) |

---

## M14 — Bloc événement temporel (Event Block)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Fond coloré (couleur agenda A13) + Label titre (A17) + Label horaire (A17) + Icône source optionnelle (A16) + Badge statut optionnel (A12) |
| **Rôle** | Représentation d'un événement dans la grille horaire |
| **Dimensionnement** | Hauteur proportionnelle à la durée. Largeur = largeur de la colonne (ou partagée si chevauchement) |
| **Exemples** | Bloc bleu "Psychologue — De 14:45 à 15:45", Bloc cyan "CMP avec Thérésa dr Boudou — De 14:00 à 15:00" |

---

## M15 — Bandeau événement journée entière (All-Day Event Banner)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Fond coloré (couleur agenda A13) + Label titre (A17) |
| **Rôle** | Représentation d'un événement "journée entière" dans la zone en haut de la grille |
| **Dimensionnement** | Largeur = une colonne (un jour) ou multi-colonnes (multi-jours) |
| **Exemples** | Bandeau saumon "Anniversaire Raymond", Bandeau vert "Anniversaire Marine" |

---

## M16 — Ligne d'action (Action Bar)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Lien secondaire (A18) + Bouton primaire (A01) |
| **Rôle** | Barre d'actions en bas d'un formulaire ou popover |
| **Exemples** | "Autres options" (lien) + "Enregistrer" (bouton bleu) |

---

## M17 — Indicateur de conflit (Conflict Indicator)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône warning (A16) + Bordure rouge sur le bloc événement (M14) + Badge "Conflit" (A12) |
| **Rôle** | Signaler un chevauchement entre deux événements |
| **Comportement** | Visible tant que le conflit n'est pas résolu |

---

## M18 — Lien "X autres" (Overflow Link)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Label compact (A17) cliquable |
| **Rôle** | Indiquer qu'il y a plus d'événements que l'espace ne peut afficher |
| **Comportement** | Clic ouvre une vue détaillée de la journée ou un panneau avec tous les événements |
| **Exemples** | "2 autres", "1 autre" (visible dans la zone journée entière de Google Agenda) |

---

## M19 — Sélecteur de pièce jointe (Attachment Selector)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône pièce jointe (A16) + Lien "Ajouter une pièce jointe" (A18) |
| **Rôle** | Ajouter un fichier ou un document à l'événement |
| **Comportement** | Clic ouvre un sélecteur de fichier |

---

## M20 — Sélecteur de récurrence (Recurrence Selector)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône récurrence (A16) + Dropdown (A10) |
| **Options** | "Une seule fois", "Tous les jours", "Toutes les semaines le [jour]", "Tous les mois le [date]", "Tous les ans le [date]", "Personnalisé..." |
| **Comportement** | "Personnalisé..." ouvre le formulaire de récurrence complet (Organisme O09) |

---

# NIVEAU 3 — ORGANISMES

Les organismes sont des assemblages complexes de molécules et d'atomes formant des sections complètes d'interface.

---

## O01 — Barre d'en-tête (Header Bar)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Bouton icône hamburger (A03) + Logo + Label "Agenda" (A17) + Bouton navigation temporelle (M09) + Label période (A17) + Champ de recherche (M11) + Bouton icône paramètres (A03) + Sélecteur de vue (M10) + Bouton icône actualiser (A03) + Avatar (A15) |
| **Rôle** | Barre supérieure permanente de l'application |
| **Comportement** | Fixe en haut de l'écran. Tous les éléments sont accessibles en permanence |
| **Position** | Pleine largeur, hauteur fixe (~64px) |

---

## O02 — Panneau latéral gauche (Sidebar)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Bouton FAB création (A04) + Mini-calendrier (O03) + Section "Mes agendas" : titre (A17) + lignes d'agenda (M12) + Section "Services synchronisés" : titre (A17) + lignes d'agenda (M12) + Section "Autres agendas" : titre (A17) + lignes d'agenda (M12) |
| **Rôle** | Panneau de navigation et filtrage |
| **Comportement** | Rétractable via le menu hamburger. Largeur fixe (~256px). Scroll vertical si la liste est longue |
| **Position** | Gauche, sous la barre d'en-tête |

---

## O03 — Mini-calendrier (Mini Calendar)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Bouton icône gauche (A03) + Label mois/année (A17) + Bouton icône droite (A03) + Grille 7x6 de numéros de jour (A21) avec étiquettes de jour (A22) en en-tête + Points d'événement (A26) |
| **Rôle** | Navigation rapide par date |
| **Comportement** | Clic sur un jour = navigation dans la vue principale. Le jour courant a un cercle bleu. Les jours avec événements ont un point |

---

## O04 — Grille calendrier Semaine (Week Grid)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | 7 en-têtes de colonne jour (M13) + Étiquette fuseau (A24) + Zone journée entière : bandeaux événements (M15) + liens overflow (M18) + Grille horaire : étiquettes d'heure (A23) + lignes de grille (A20) + blocs événements (M14) + indicateur maintenant (A27) + indicateurs conflit (M17) |
| **Rôle** | Vue principale de l'agenda en mode Semaine |
| **Comportement** | Scroll vertical pour parcourir les heures. Les événements sont positionnés en absolu selon leur plage. Le chevauchement partage la largeur des colonnes |

---

## O05 — Grille calendrier Jour (Day Grid)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Identique à O04 mais avec une seule colonne jour |
| **Rôle** | Vue principale de l'agenda en mode Jour |
| **Comportement** | Plus d'espace horizontal pour chaque événement. Détails plus visibles dans les blocs |

---

## O06 — Grille calendrier Mois (Month Grid)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | 7 étiquettes de jour en en-tête (A22) + 5-6 lignes x 7 colonnes de cellules jour contenant : numéro de jour (A21) + liste courte d'événements (pastille A13 + titre A17) + lien overflow (M18) |
| **Rôle** | Vue principale de l'agenda en mode Mois |
| **Comportement** | Clic sur un jour = bascule en Vue Jour. Clic sur "+X autres" = détail de la journée |

---

## O07 — Vue Planning / Liste (Schedule View)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Liste chronologique : séparateurs de jour (label date A17) + cartes d'événement : pastille (A13) + titre (A17) + horaire (A17) + lieu (A17) + badge source (A12) |
| **Rôle** | Vue principale de l'agenda en mode Planning/Liste |
| **Comportement** | Défilement infini. Les jours sans événement sont masqués ou affichés vides |

---

## O08 — Formulaire de création rapide (Quick Add Popover)

C'est l'organisme visible dans la première capture Google Agenda.

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Bouton icône fermer (A03) + Bouton icône menu (A03) + **Champ titre** (A05 — "Ajouter un titre") + **Barre d'onglets** (M08 — "Événement" / "Tâche" / "Planning des rendez-vous") + **Sélecteur date/heure** (M02) + **Champ participants** (M01 — icône personnes + "Ajouter des invités") + **Séparateur** (A19) + **Champ lieu** (M01 — icône lieu + "Ajouter un lieu") + **Champ description** (M01 — icône description + "Ajouter une description") + **Champ pièce jointe** (M19) + **Séparateur** (A19) + **Sélecteur d'agenda** (M03) + **Sélecteur de statut** (M04) + **Barre d'actions** (M16 — "Autres options" + "Enregistrer") |
| **Rôle** | Création rapide d'un événement depuis la grille ou le bouton "+" |
| **Position** | Popover flottant ancré au créneau cliqué ou centré sur l'écran |
| **Comportement** | Ouverture avec les champs pré-remplis (date/heure du créneau). L'onglet "Événement" est actif par défaut. "Autres options" ouvre le formulaire complet (O10). "Enregistrer" crée l'événement et ferme le popover |

### Détail de la composition séquentielle (calqué sur Google Agenda)

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

---

## O09 — Formulaire de récurrence personnalisée (Recurrence Editor)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Label "Répéter tous les" (A17) + Input numérique (A05) + Dropdown fréquence (A10 — "jours"/"semaines"/"mois"/"ans") + Grille de jours de semaine (7x A07 — cases à cocher Lun à Dim, pour fréquence hebdomadaire) + Section fin : radios (A09) "Jamais" / "Après X occurrences" (A05) / "À la date du" (sélecteur date) + Barre d'actions (M16 — "Annuler" + "OK") |
| **Rôle** | Configuration fine de la récurrence d'un événement |
| **Position** | Dialogue modal ou section inline du formulaire complet |

---

## O10 — Formulaire d'édition complet (Full Event Editor)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Tous les champs de O08 (formulaire rapide) **plus** : Sélecteur de récurrence complet (M20) + Lignes de rappel multiples (M06) + Lien "Ajouter une notification" (A18) + Sélecteur de visibilité (M01 — dropdown "Par défaut"/"Public"/"Privé") + Champ participants étendu (M05 multiples) + Sélecteur de couleur complet (A14) |
| **Rôle** | Édition complète d'un événement avec tous les paramètres |
| **Position** | Pleine page ou panneau large |
| **Comportement** | Ouvert depuis "Autres options" du formulaire rapide ou depuis "Modifier" dans le popover de détail |

---

## O11 — Popover de détail événement (Event Detail Popover)

C'est l'organisme visible dans la deuxième capture Google Agenda.

| Propriété | Valeur |
|-----------|--------|
| **Composition** | **En-tête** : Bouton icône modifier (A03) + Bouton icône supprimer (A03) + Bouton icône fermer (A03) + **Titre** (A17 — H2) + **Participant / Créateur** (M05 — avatar + nom) + **Date et heure** (M01 — icône horloge + "Mardi, 3 février — De 14:00 à 15:00") + **Créateur** (M07 — "Créé par : Sonia Mateo") + **Séparateur** (A19) + **Rappels** (M06 — "10 minutes avant, par e-mail" + "10 minutes avant") + **Lien ajout** (A18 — "Ajouter une notification") + **Barre d'actions** (M16 — "Autres options" + "Enregistrer") |
| **Rôle** | Affichage détaillé d'un événement au clic |
| **Position** | Popover flottant ancré au bloc événement cliqué |

### Variante : événement synchronisé

Pour un événement issu de JayRDV ou JayFestival :

| Modification | Détail |
|-------------|--------|
| **Boutons modifier/supprimer** | Remplacés par un bouton "Ouvrir dans [Service source]" (A03 + A16 icon-external) |
| **Badge source** | Badge (A12) "Synchronisé depuis JayRDV" ou "Synchronisé depuis JayFestival" visible sous le titre |
| **Barre d'actions** | Pas de bouton "Enregistrer". Uniquement "Ouvrir dans [Service source]" |

### Schéma du popover (calqué sur Google Agenda)

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

---

## O12 — Bandeau d'alerte conflit (Conflict Alert Banner)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Icône warning (A16) + Label alerte (A17 — "X conflits de présence physique à résoudre") + Lien "Voir les conflits" (A18) + Bouton icône fermer (A03) |
| **Rôle** | Avertissement persistant en cas de conflits temporels non résolus |
| **Position** | Bandeau horizontal en haut de la zone principale, sous la barre d'en-tête |
| **Comportement** | Reste visible tant qu'au moins un conflit existe. Clic sur "Voir les conflits" ouvre un détail |

---

## O13 — Dialogue d'export (Export Dialog)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Titre "Exporter l'agenda" (A17) + Dropdown format (A10 — "iCal (.ics)" / "PDF") + Sélecteur période (M02 simplifié) + Liste d'agendas avec cases à cocher (M12 multiples) + Barre d'actions (M16 — "Annuler" + "Télécharger") |
| **Rôle** | Configuration et lancement de l'export |
| **Position** | Dialogue modal centré |

---

## O14 — Dialogue de partage (Share Dialog)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Titre "Partager l'agenda" (A17) + Champ recherche utilisateur (M11) + Liste de partages existants : lignes participant (M05) + dropdown niveau (A10 — "Libre/occupé" / "Voir les détails" / "Modifier") + bouton supprimer (A03) + Barre d'actions (M16 — "Annuler" + "Enregistrer") |
| **Rôle** | Gestion du partage d'un agenda avec d'autres utilisateurs |
| **Position** | Dialogue modal centré |

---

## O15 — Page des paramètres (Settings Page)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Sections de paramètres groupées : Section "Général" (dropdowns A10 pour langue, fuseau, format heure, début semaine) + Section "Vue" (dropdowns A10, toggles A08) + Section "Notifications" (dropdowns A10, lignes rappel M06) + Section "Gestion des agendas" (liste d'agendas avec boutons modifier A03, supprimer A03, créer A01) |
| **Rôle** | Configuration complète de l'agenda |
| **Position** | Pleine page ou panneau large |

---

## O16 — Résultats de recherche (Search Results)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Champ de recherche (M11) + Liste de résultats : pastille couleur (A13) + titre (A17) + date/heure (A17) + agenda source (A17/A12) |
| **Rôle** | Affichage des événements correspondant à la recherche |
| **Position** | Overlay sous la barre de recherche |
| **Comportement** | Résultats en temps réel. Clic sur un résultat = navigation vers l'événement |

---

## O17 — Zone journée entière (All-Day Zone)

| Propriété | Valeur |
|-----------|--------|
| **Composition** | Conteneur horizontal sous les en-têtes de colonnes (M13). Contient : bandeaux événements journée entière (M15) + liens overflow (M18 — "2 autres", "1 autre") |
| **Rôle** | Affichage des événements "journée entière" et multi-jours |
| **Comportement** | Extensible verticalement si beaucoup d'événements. Replié par défaut avec lien overflow |

---

# INVENTAIRE RÉCAPITULATIF

## Atomes (27 composants)

| ID | Nom | Catégorie |
|----|-----|-----------|
| A01 | Bouton primaire | Action |
| A02 | Bouton secondaire | Action |
| A03 | Bouton icône | Action |
| A04 | Bouton flottant (FAB) | Action |
| A05 | Champ de texte (Input) | Saisie |
| A06 | Zone de texte (Textarea) | Saisie |
| A07 | Case à cocher (Checkbox) | Sélection |
| A08 | Toggle / Interrupteur | Sélection |
| A09 | Radio bouton | Sélection |
| A10 | Dropdown / Sélecteur | Sélection |
| A11 | Onglet (Tab) | Navigation |
| A12 | Badge | Indicateur |
| A13 | Pastille de couleur | Indicateur |
| A14 | Sélecteur de couleur | Sélection |
| A15 | Avatar | Identité |
| A16 | Icône | Visuel |
| A17 | Label / Texte | Typographie |
| A18 | Lien | Navigation |
| A19 | Séparateur (Divider) | Structure |
| A20 | Ligne de grille horaire | Structure |
| A21 | Numéro de jour | Temporel |
| A22 | Étiquette de jour | Temporel |
| A23 | Étiquette d'heure | Temporel |
| A24 | Étiquette de fuseau | Temporel |
| A25 | Tooltip | Information |
| A26 | Point d'événement | Indicateur |
| A27 | Indicateur "Maintenant" | Temporel |

## Molécules (20 composants)

| ID | Nom | Catégorie |
|----|-----|-----------|
| M01 | Champ de formulaire | Formulaire |
| M02 | Sélecteur date/heure | Formulaire |
| M03 | Sélecteur d'agenda | Formulaire |
| M04 | Sélecteur de statut | Formulaire |
| M05 | Ligne de participant | Affichage |
| M06 | Ligne de rappel | Formulaire |
| M07 | Ligne de créateur | Affichage |
| M08 | Barre d'onglets | Navigation |
| M09 | Bouton navigation temporelle | Navigation |
| M10 | Sélecteur de vue | Navigation |
| M11 | Champ de recherche | Recherche |
| M12 | Ligne d'agenda | Filtrage |
| M13 | En-tête de colonne jour | Structure |
| M14 | Bloc événement temporel | Événement |
| M15 | Bandeau événement journée entière | Événement |
| M16 | Ligne d'action (Action Bar) | Action |
| M17 | Indicateur de conflit | Indicateur |
| M18 | Lien "X autres" (Overflow) | Navigation |
| M19 | Sélecteur de pièce jointe | Formulaire |
| M20 | Sélecteur de récurrence | Formulaire |

## Organismes (17 composants)

| ID | Nom | Catégorie |
|----|-----|-----------|
| O01 | Barre d'en-tête (Header Bar) | Structure |
| O02 | Panneau latéral gauche (Sidebar) | Structure |
| O03 | Mini-calendrier | Navigation |
| O04 | Grille calendrier Semaine | Vue |
| O05 | Grille calendrier Jour | Vue |
| O06 | Grille calendrier Mois | Vue |
| O07 | Vue Planning / Liste | Vue |
| O08 | Formulaire de création rapide | Création |
| O09 | Formulaire de récurrence | Création |
| O10 | Formulaire d'édition complet | Édition |
| O11 | Popover de détail événement | Consultation |
| O12 | Bandeau d'alerte conflit | Alerte |
| O13 | Dialogue d'export | Export |
| O14 | Dialogue de partage | Partage |
| O15 | Page des paramètres | Configuration |
| O16 | Résultats de recherche | Recherche |
| O17 | Zone journée entière | Structure |

---

## Références

| Document | Rôle |
|----------|------|
| [JayKoa - Document Fondateur](./JayKoa%20-%20Document%20Fondateur.md) | Positionnement et architecture du Service |
| [JayKoa - Ecrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) | Écrans et zones calqués sur Google Agenda |
| [JayKoa - Parcours Utilisateurs](./JayKoa%20-%20Parcours%20Utilisateurs.md) | Parcours dans lesquels ces composants s'insèrent |

---

**Document** : JayKoa — Design System Atomic
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : Document de référence — inventaire Atomic Design calqué sur Google Agenda
