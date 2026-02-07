# JayKoa — Parcours Utilisateurs

## Contexte

Ce document décrit les **parcours utilisateurs** de JayKoa, calqués sur l'expérience de **Google Agenda**. Chaque parcours est décrit tel qu'un utilisateur le vit dans l'interface, étape par étape, avec les écrans traversés et les comportements attendus.

L'objectif est de couvrir l'ensemble des parcours fondamentaux d'un agenda, en intégrant les spécificités JayKoa (synchronisation inter-Services, événements en lecture réfléchie, conflits temporels).

Ce document est destiné à être adapté aux spécificités Miyukini dans une phase ultérieure.

## Portée / Scope

- **Applicable à :** Parcours utilisateurs de JayKoa (consultation, création, modification, navigation, synchronisation, export, partage)
- **Audience :** Équipes produit, UX/UI, développement
- **Statut :** Document de référence — calque Google Agenda à adapter

### Hors périmètre

- Parcours d'administration système (MiyukiniAdmin)
- Parcours d'authentification (géré par le COG)
- Parcours techniques côté serveur

---

## 1. Parcours — Première ouverture de l'agenda

L'utilisateur ouvre JayKoa pour la première fois.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Ouvre JayKoa depuis son espace utilisateur | Affichage de la Vue Semaine centrée sur la date du jour. Le panneau latéral gauche est ouvert avec la liste des agendas | Vue Semaine (écran principal) |
| 2 | Découvre la grille vide (aucun événement) | Le mini-calendrier indique le jour courant (cercle bleu). La ligne rouge de l'heure actuelle est visible | Barre d'en-tête + Panneau latéral + Grille |
| 3 | Voit la section "Services synchronisés" dans le panneau latéral | Les Services disponibles (JayRDV, JayFestival) sont listés avec des cases à cocher. Par défaut, aucun n'est activé | Panneau latéral gauche |
| 4 | (Optionnel) Active la synchronisation avec JayRDV ou JayFestival | Les événements synchronisés depuis ces Services apparaissent dans la grille avec leur couleur respective | Panneau latéral + Grille |

**Résultat** : L'utilisateur voit son agenda vide, prêt à recevoir des événements internes ou des synchronisations.

---

## 2. Parcours — Consultation de l'agenda (navigation quotidienne)

L'utilisateur consulte son agenda au quotidien.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Ouvre JayKoa | Affichage de la Vue Semaine (ou dernière vue utilisée) centrée sur la date du jour. Les événements de la semaine sont affichés dans la grille | Vue Semaine |
| 2 | Regarde ses événements du jour | Les événements du jour sont visibles dans la colonne correspondante. Le jour courant est surligné (cercle bleu sur le numéro). La ligne rouge indique l'heure actuelle | Grille horaire |
| 3 | Clique sur les flèches < > pour naviguer à la semaine suivante | La grille se décale d'une semaine. Le libellé de période se met à jour (ex. "Février 2026 — Semaine 7") | Barre d'en-tête + Grille |
| 4 | Clique sur "Aujourd'hui" | Retour à la semaine contenant la date du jour | Barre d'en-tête + Grille |
| 5 | Clique sur un jour dans le mini-calendrier | La vue principale se centre sur le jour cliqué | Panneau latéral + Grille |
| 6 | Change de vue via le sélecteur (ex. "Mois") | La grille bascule en Vue Mois. Les événements sont affichés sous forme compacte dans les cellules | Sélecteur de vue + Zone principale |

**Résultat** : L'utilisateur navigue librement dans son agenda, change de période et de vue, et voit tous ses événements (internes et synchronisés).

---

## 3. Parcours — Création d'un événement (Quick Add)

L'utilisateur crée un événement rapidement.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique sur un créneau vide dans la grille (ex. mercredi 14:00) | Le formulaire de création rapide s'ouvre, pré-rempli avec la date et l'heure du créneau cliqué | Formulaire rapide (popup) |
| 2 | Saisit le titre de l'événement (ex. "Réunion équipe") | Le titre est affiché dans le champ | Formulaire rapide |
| 3 | (Optionnel) Ajuste l'heure de fin | L'heure de fin se met à jour | Formulaire rapide |
| 4 | (Optionnel) Sélectionne l'agenda cible dans le dropdown | L'agenda sélectionné détermine la couleur de l'événement | Formulaire rapide |
| 5 | Clique sur "Enregistrer" | L'événement est créé et apparaît immédiatement dans la grille sous forme de bloc coloré | Grille horaire |

**Alternative** : L'utilisateur peut aussi cliquer sur le bouton "+" dans le panneau latéral pour ouvrir le formulaire rapide sans pré-remplissage de créneau.

**Alternative** : L'utilisateur peut cliquer-glisser sur un créneau vide pour définir une plage horaire (début — fin) et ouvrir le formulaire rapide pré-rempli avec cette plage.

---

## 4. Parcours — Création d'un événement (formulaire complet)

L'utilisateur crée un événement avec tous les détails.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Ouvre le formulaire rapide (clic sur créneau ou bouton "+") | Formulaire rapide affiché | Formulaire rapide |
| 2 | Clique sur "Plus d'options" | Le formulaire complet s'ouvre en pleine page ou en panneau large | Formulaire complet |
| 3 | Saisit le titre | — | Formulaire complet |
| 4 | Ajuste la date et l'heure de début/fin | Les champs se mettent à jour | Formulaire complet |
| 5 | (Optionnel) Active "Journée entière" | Les champs d'heure disparaissent. L'événement sera affiché dans la zone journée entière | Formulaire complet |
| 6 | (Optionnel) Sélectionne une récurrence (ex. "Toutes les semaines") | Le détail de récurrence s'affiche si "Personnalisé" est choisi | Formulaire complet |
| 7 | (Optionnel) Saisit le lieu | — | Formulaire complet |
| 8 | (Optionnel) Saisit la description | — | Formulaire complet |
| 9 | (Optionnel) Ajoute des participants | Les participants sont ajoutés à la liste | Formulaire complet |
| 10 | (Optionnel) Configure les rappels (ex. "30 minutes avant") | Le rappel est ajouté | Formulaire complet |
| 11 | Sélectionne l'agenda cible | — | Formulaire complet |
| 12 | (Optionnel) Choisit une couleur personnalisée | La couleur remplace celle de l'agenda par défaut | Formulaire complet |
| 13 | (Optionnel) Définit le statut : Libre / Occupé | — | Formulaire complet |
| 14 | (Optionnel) Définit la visibilité : Par défaut / Public / Privé | — | Formulaire complet |
| 15 | Clique sur "Enregistrer" | L'événement est créé avec tous les paramètres. Il apparaît dans la grille. Si récurrent, toutes les occurrences sont générées | Retour à la vue principale |

**Résultat** : L'événement est créé dans l'agenda sélectionné avec tous les détails, la récurrence, les rappels et les participants configurés.

---

## 5. Parcours — Création d'un événement journée entière

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique dans la zone "journée entière" en haut de la grille (au-dessus des lignes horaires) | Le formulaire rapide s'ouvre avec "Journée entière" activé et la date pré-remplie | Formulaire rapide |
| 2 | Saisit le titre (ex. "Anniversaire Marine") | — | Formulaire rapide |
| 3 | Clique sur "Enregistrer" | L'événement apparaît dans la zone journée entière sous forme de bandeau coloré | Zone journée entière |

---

## 6. Parcours — Consultation du détail d'un événement

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique sur un événement dans la grille (ex. "Psychologue — De 14:45 à 15:45") | Le popover de détail s'affiche à côté du bloc événement | Popover de détail |
| 2 | Lit les informations : titre, date/heure, lieu, description, agenda, rappels | Toutes les informations sont affichées | Popover de détail |
| 3a | (Événement interne) Clique sur "Modifier" | Le formulaire complet s'ouvre avec les données pré-remplies | Formulaire complet |
| 3b | (Événement synchronisé) Clique sur "Ouvrir dans JayRDV" ou "Ouvrir dans JayFestival" | Redirection vers le Service source pour voir le détail métier | Service source |
| 4 | Clique en dehors du popover ou sur "Fermer" | Le popover se ferme | Retour à la grille |

---

## 7. Parcours — Modification d'un événement interne

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique sur un événement interne dans la grille | Le popover de détail s'affiche | Popover de détail |
| 2 | Clique sur "Modifier" | Le formulaire complet s'ouvre avec les données de l'événement pré-remplies | Formulaire complet |
| 3 | Modifie les champs souhaités (titre, date, heure, lieu, description, rappels, etc.) | Les champs se mettent à jour | Formulaire complet |
| 4 | Clique sur "Enregistrer" | L'événement est mis à jour. Les modifications sont visibles immédiatement dans la grille | Retour à la vue principale |

**Variante récurrence** : Si l'événement est récurrent, le système demande : "Modifier cet événement uniquement / Cet événement et les suivants / Tous les événements de la série".

**Variante glisser-déposer** : L'utilisateur peut glisser un événement interne vers un autre créneau pour le déplacer directement, ou redimensionner le bloc pour modifier la durée.

---

## 8. Parcours — Suppression d'un événement interne

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique sur un événement interne | Le popover de détail s'affiche | Popover de détail |
| 2 | Clique sur "Supprimer" | Une confirmation s'affiche : "Supprimer cet événement ?" | Popover / Modal de confirmation |
| 3 | Confirme la suppression | L'événement disparaît de la grille | Retour à la vue principale |

**Variante récurrence** : Si l'événement est récurrent, le système demande : "Supprimer cet événement uniquement / Cet événement et les suivants / Tous les événements de la série".

---

## 9. Parcours — Filtrage des agendas

L'utilisateur choisit quels agendas afficher.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Regarde la liste des agendas dans le panneau latéral gauche | Tous les agendas sont listés avec leur case à cocher et leur pastille de couleur | Panneau latéral |
| 2 | Décoche un agenda (ex. "JayRDV — Mes rendez-vous") | Les événements de cet agenda disparaissent immédiatement de la grille | Panneau latéral + Grille |
| 3 | Recoche l'agenda | Les événements réapparaissent | Panneau latéral + Grille |
| 4 | Décoche "JayFestival — Événements publics" | Les événements publics JayFestival disparaissent. Seuls les agendas cochés restent visibles | Panneau latéral + Grille |

**Résultat** : L'utilisateur peut afficher ou masquer n'importe quel agenda (personnel, professionnel, synchronisé) pour personnaliser sa vue.

---

## 10. Parcours — Changement de vue

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique sur le sélecteur de vue dans la barre d'en-tête (ex. "Semaine") | Le menu déroulant affiche les options : Jour, Semaine, Mois, Année, Planning | Barre d'en-tête |
| 2 | Sélectionne "Mois" | La zone principale bascule en Vue Mois. Les événements sont affichés sous forme compacte dans les cellules du mois. Le libellé de période change (ex. "Février 2026") | Zone principale |
| 3 | Sélectionne "Planning" | La zone principale bascule en Vue Planning (liste chronologique). Les événements sont listés par jour dans l'ordre temporel | Zone principale |
| 4 | Sélectionne "Jour" | La zone principale bascule en Vue Jour. Un seul jour est affiché avec tous les détails | Zone principale |

---

## 11. Parcours — Recherche d'un événement

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique sur l'icône de recherche dans la barre d'en-tête | Le champ de recherche s'ouvre (overlay ou barre étendue) | Barre de recherche |
| 2 | Saisit un terme de recherche (ex. "Psychologue") | Les résultats s'affichent en temps réel : liste d'événements correspondants avec titre, date, horaire, agenda | Résultats de recherche |
| 3 | Clique sur un résultat (ex. "Psychologue — Lun 2 fév, 14:45") | La vue principale navigue vers la date de l'événement et le popover de détail s'ouvre | Vue principale + Popover |
| 4 | Ferme la recherche | Retour à la vue précédente | Vue principale |

---

## 12. Parcours — Activation de la synchronisation avec un Service COG

L'utilisateur active la synchronisation avec JayRDV ou JayFestival.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Ouvre les paramètres (icône engrenage) ou regarde la section "Services synchronisés" dans le panneau latéral | Les Services disponibles sont listés (JayRDV, JayFestival) avec leur statut (activé/désactivé) | Paramètres ou Panneau latéral |
| 2 | Coche "JayRDV — Mes rendez-vous" | La synchronisation s'active. Les rendez-vous confirmés, créneaux bloqués et modifications/annulations de JayRDV apparaissent dans la grille avec la couleur dédiée (cyan) | Panneau latéral + Grille |
| 3 | Coche "JayFestival — Mes festivals" | Les dates de festivals, inscriptions, deadlines et événements favoris de JayFestival apparaissent dans la grille avec la couleur dédiée (violet) | Panneau latéral + Grille |
| 4 | Consulte son agenda | La vue consolidée affiche tous les événements : internes + JayRDV + JayFestival, chacun avec sa couleur et son icône de source | Grille |

**Résultat** : L'utilisateur voit une timeline consolidée de tous ses engagements temporels, issus de ses agendas personnels et des Services synchronisés.

---

## 13. Parcours — Consultation d'un événement synchronisé (lecture réfléchie)

L'utilisateur consulte un rendez-vous issu de JayRDV.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Voit un bloc cyan dans la grille : "CMP avec Thérésa — De 14:00 à 15:00" avec une icône JayRDV | L'événement est affiché en lecture réfléchie. Pas de poignée de redimensionnement, pas de curseur de déplacement | Grille |
| 2 | Clique sur l'événement | Le popover de détail s'ouvre. Les informations temporelles sont affichées. Un badge "Synchronisé depuis JayRDV" est visible. Les boutons "Modifier" et "Supprimer" sont absents | Popover de détail |
| 3 | Voit le bouton "Ouvrir dans JayRDV" | — | Popover de détail |
| 4 | Clique sur "Ouvrir dans JayRDV" | Redirection vers JayRDV pour consulter le détail métier complet du rendez-vous | Service JayRDV |

**Résultat** : L'utilisateur consulte les informations temporelles d'un événement synchronisé et peut accéder au détail métier dans le Service source.

---

## 14. Parcours — Détection et gestion d'un conflit temporel

L'utilisateur a deux événements qui se chevauchent.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Consulte la Vue Semaine | Deux événements se chevauchent sur le même créneau (ex. un RDV JayRDV et un atelier JayFestival, tous les deux le vendredi de 15:00 à 16:00). Les blocs se partagent la largeur de la colonne | Grille |
| 2 | Voit un indicateur de conflit | Les deux blocs en chevauchement ont une bordure rouge. Un bandeau en haut de la vue indique : "1 conflit de présence physique à résoudre" | Grille + Bandeau d'alerte |
| 3 | Clique sur le bandeau d'alerte | Un panneau ou une modal détaille le conflit : les deux événements en chevauchement, leurs horaires, et les options de résolution | Panneau de conflit |
| 4 | Décide de modifier l'un des événements | Pour un événement interne : ouvre le formulaire de modification. Pour un événement synchronisé : redirige vers le Service source | Formulaire ou Service source |
| 5 | Résout le conflit (modification de l'horaire ou annulation) | Le bandeau d'alerte disparaît. Les bordures rouges sont retirées. Les événements ne se chevauchent plus | Grille |

**Résultat** : L'utilisateur est informé des conflits temporels et guidé vers la résolution, sans que JayKoa ne bloque aucune action.

---

## 15. Parcours — Création d'un événement avec conflit

L'utilisateur crée un événement qui chevauche un événement existant.

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique sur un créneau déjà occupé par un autre événement | Le formulaire de création rapide s'ouvre, pré-rempli avec le créneau | Formulaire rapide |
| 2 | Saisit le titre et clique sur "Enregistrer" | Un avertissement s'affiche : "Conflit avec [Événement existant] — De 14:00 à 15:00. Créer quand même ?" | Alerte de conflit |
| 3 | Confirme la création | L'événement est créé. Les deux événements se partagent la colonne avec un indicateur de conflit (bordure rouge) | Grille |

**Résultat** : JayKoa avertit l'utilisateur du conflit mais ne bloque jamais la création.

---

## 16. Parcours — Export de l'agenda

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Ouvre les paramètres ou le menu d'un agenda | L'option "Exporter" est disponible | Paramètres / Menu contextuel |
| 2 | Clique sur "Exporter" | Un dialogue s'ouvre avec les options d'export | Dialogue d'export |
| 3 | Sélectionne le format : iCal (.ics) ou PDF | — | Dialogue d'export |
| 4 | Sélectionne la période : Semaine en cours, Mois en cours, Personnalisée | — | Dialogue d'export |
| 5 | Sélectionne les agendas à inclure (cases à cocher) | — | Dialogue d'export |
| 6 | Clique sur "Télécharger" | Le fichier est généré et téléchargé | Navigateur (téléchargement) |

**Résultat** : L'utilisateur obtient un fichier exportable contenant ses événements pour la période et les agendas choisis.

---

## 17. Parcours — Partage d'un agenda

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Ouvre les paramètres d'un agenda (via le menu contextuel dans le panneau latéral) | Les options de l'agenda s'affichent | Paramètres de l'agenda |
| 2 | Clique sur "Partager cet agenda" | Le panneau de partage s'ouvre | Panneau de partage |
| 3 | Saisit l'identifiant ou le nom de l'utilisateur avec qui partager | L'utilisateur est trouvé et proposé | Panneau de partage |
| 4 | Sélectionne le niveau de partage : "Voir libre/occupé uniquement" ou "Voir les détails" ou "Modifier les événements" | — | Panneau de partage |
| 5 | Clique sur "Partager" | L'agenda est partagé. L'utilisateur destinataire le verra dans sa section "Autres agendas" | Panneau de partage |
| 6 | (Optionnel) Révoque le partage ultérieurement | L'accès est supprimé immédiatement | Paramètres de l'agenda |

---

## 18. Parcours — Configuration des paramètres

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Clique sur l'icône engrenage dans la barre d'en-tête | La page des paramètres s'ouvre | Paramètres |
| 2 | Modifie le fuseau horaire (ex. Europe/Paris) | Le fuseau est mis à jour. Les horaires de la grille reflètent le nouveau fuseau | Paramètres généraux |
| 3 | Modifie le début de la semaine (Lundi → Dimanche) | La grille se réorganise avec Dimanche en première colonne | Paramètres généraux |
| 4 | Modifie la vue par défaut (Semaine → Mois) | La prochaine ouverture de l'agenda affichera la Vue Mois | Paramètres de vue |
| 5 | Modifie le rappel par défaut (30 min → 1h avant) | Les prochains événements créés auront un rappel de 1h par défaut | Paramètres de notifications |
| 6 | Crée un nouvel agenda (nom : "Sport", couleur : orange) | L'agenda apparaît dans la liste "Mes agendas" du panneau latéral | Gestion des agendas |
| 7 | Ferme les paramètres | Retour à la vue principale avec les paramètres appliqués | Vue principale |

---

## 19. Parcours — Navigation au mini-calendrier

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | Regarde le mini-calendrier dans le panneau latéral gauche | Le mois en cours est affiché. Le jour courant a un cercle bleu. Les jours contenant des événements ont un point | Panneau latéral |
| 2 | Clique sur les flèches < > du mini-calendrier | Le mini-calendrier passe au mois précédent ou suivant | Panneau latéral |
| 3 | Clique sur un jour (ex. le 15) | La vue principale se centre sur le 15 du mois affiché. En Vue Semaine, la semaine contenant le 15 est affichée. En Vue Jour, le 15 est affiché | Panneau latéral + Zone principale |

---

## 20. Parcours — Réception d'un rappel

| Étape | Action utilisateur | Réponse du système | Écran |
|-------|-------------------|-------------------|-------|
| 1 | L'utilisateur est dans l'agenda ou ailleurs dans le COG | 30 minutes avant un événement (ou selon le rappel configuré), une notification apparaît | Notification système |
| 2 | Voit la notification : "Réunion équipe dans 30 minutes" | La notification affiche le titre, l'horaire et l'agenda source | Notification |
| 3 | Clique sur la notification | JayKoa s'ouvre (ou se met au premier plan) avec la vue centrée sur l'événement concerné. Le popover de détail s'ouvre | Vue principale + Popover |
| 4 | (Optionnel) Ignore ou repousse la notification | La notification disparaît. L'événement reste dans l'agenda | — |

---

## 21. Synthèse des parcours

| # | Parcours | Type | Écrans principaux |
|---|----------|------|-------------------|
| 1 | Première ouverture | Découverte | Vue Semaine, Panneau latéral |
| 2 | Consultation quotidienne | Navigation | Vue Semaine/Jour/Mois, Barre d'en-tête |
| 3 | Création rapide | Création | Formulaire rapide, Grille |
| 4 | Création complète | Création | Formulaire complet |
| 5 | Événement journée entière | Création | Formulaire rapide, Zone journée entière |
| 6 | Consultation détail | Consultation | Popover de détail |
| 7 | Modification | Édition | Popover + Formulaire complet |
| 8 | Suppression | Édition | Popover + Confirmation |
| 9 | Filtrage des agendas | Navigation | Panneau latéral + Grille |
| 10 | Changement de vue | Navigation | Sélecteur de vue + Zone principale |
| 11 | Recherche | Navigation | Barre de recherche + Résultats |
| 12 | Activation synchronisation | Configuration | Paramètres / Panneau latéral |
| 13 | Consultation événement synchronisé | Consultation | Popover (lecture réfléchie) |
| 14 | Détection conflit | Information | Grille + Bandeau d'alerte |
| 15 | Création avec conflit | Création | Formulaire + Alerte |
| 16 | Export | Export | Dialogue d'export |
| 17 | Partage | Partage | Paramètres + Panneau de partage |
| 18 | Configuration paramètres | Configuration | Page paramètres |
| 19 | Navigation mini-calendrier | Navigation | Panneau latéral |
| 20 | Réception rappel | Notification | Notification + Popover |

---

## 22. Références

| Document | Rôle |
|----------|------|
| [JayKoa - Document Fondateur](./JayKoa%20-%20Document%20Fondateur.md) | Positionnement et architecture du Service |
| [JayKoa - Ecrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) | Écrans, zones et composants UI calqués sur Google Agenda |
| [JayKoa - Referentiel Fonctionnel Inspire Google Agenda](./reference/JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | Correspondance fonctionnelle Google Agenda vers JayKoa |

---

**Document** : JayKoa — Parcours Utilisateurs
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Document de référence — calque Google Agenda à adapter
