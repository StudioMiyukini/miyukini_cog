# JayKoa — Parcours utilisateurs

## Contexte

JayKoa est un **service de plateforme** : les parcours utilisateurs qui font intervenir l’agenda sont **vécus par l’utilisateur final via les services consommateurs** (JayRDV, JayFestival). Ce document décrit les **parcours types** dans lesquels JayKoa intervient (consultation agenda, vérification conflit, export, agrégation), ainsi que les **parcours côté service** (publication d’entrées, interrogation, agrégation) pour clarifier les responsabilités et les points de contact.

## Portée / Scope

- **Périmètre** : Parcours utilisateurs impliquant l’agenda (vécus via JayRDV, JayFestival) ; parcours « côté service » (flux JayKoa ↔ services consommateurs).
- **Hors périmètre** : Parcours entièrement gérés par les services consommateurs sans appel à l’agenda (ex. connexion, paramétrage notifications).

---

## 1. Parcours utilisateurs (côté utilisateur final)

Les parcours ci-dessous sont **vécus par l’utilisateur** dans l’interface de **JayRDV** ou **JayFestival** ; JayKoa fournit les **données et la logique** (entrées, conflits, export) via ses Opérateurs et Kits.

### 1.1 Consultation de l’agenda (vue calendrier)

| Étape | Action utilisateur | Rôle JayKoa | Écran / contexte |
|-------|--------------------|----------------------|-------------------|
| 1 | Ouverture de la page « Mon agenda » ou du bloc calendrier (dashboard). | Fourniture des entrées agenda (plage, type, libellé, source) pour l’utilisateur courant, selon Mandat et période demandée. | Dashboard exposant (JayFestival), Dashboard pro (JayRDV), « Mes RDV » (JayRDV), Espace visiteur (JayFestival). |
| 2 | Changement de période (jour / semaine / mois) ou filtre (source, type). | Mise à jour des données (même API ou requête avec nouvelle période/filtre). | Vue calendrier (AGD-UI-01), filtre (AGD-UI-04). |
| 3 | Clic sur une entrée pour voir le détail. | Fourniture de l’id opaque ; le **détail métier** (nom client, objet RDV, fiche édition) est géré par le service consommateur. | Redirection vers écran du service (fiche RDV, fiche participation, etc.). |

**Résultat** : L’utilisateur voit ses entrées agenda (RDV, éditions, ateliers) dans une vue unifiée ou filtrée, sans quitter l’espace du service (JayRDV ou JayFestival).

### 1.2 Vérification conflit avant une action (candidature, réservation, inscription)

| Étape | Action utilisateur | Rôle JayKoa | Écran / contexte |
|-------|--------------------|----------------------|-------------------|
| 1 | Remplissage d’un formulaire (candidature à un festival, réservation d’un RDV, inscription à un atelier) avec une date ou une plage. | — | Dépôt candidature (JayFestival), Création RDV (JayRDV), Inscription atelier (JayFestival). |
| 2 | Soumission du formulaire (ou sélection de la date selon flux). | Vérification conflit : comparaison de la plage avec les entrées existantes de l’utilisateur ; retour conflit oui/non + type d’événement (ex. présence physique) + liste des entrées en conflit. | Appel JayKoa par le service consommateur. |
| 3 | Affichage d’une alerte si conflit (AGD-UI-02). | Données de conflit (entrées en chevauchement, type). | Alerte conflit (AGD-UI-02). |
| 4 | Utilisateur modifie la date, annule ou **confirme malgré le conflit** (selon règle métier du service). Pour les événements **présence physique** : la confirmation est **autorisée** — la réservation ou l’entrée dans l’agenda est enregistrée ; l’utilisateur est **notifié** et JayKoa **poussera à la résolution** par alertes et indicateurs UI (rouge clignotant) jusqu’à résolution (AGD-SEC-6, AGD-UI-06). | Si confirmation : enregistrement de la nouvelle entrée (publication vers JayKoa par le service consommateur après validation métier). Pour présence physique en conflit : l’entrée est enregistrée ; JayKoa maintient le statut « conflit non résolu » et fournit les données pour alertes et indicateurs UI. | Formulaire, écran de confirmation. |
| 5 | (Présence physique uniquement) Tant que le conflit n’est pas résolu : l’utilisateur voit des **alertes** et des **indicateurs UI en rouge clignotant** sur les entrées en conflit (AGD-UI-06). Il peut annuler, reporter ou modifier l’un des événements pour supprimer le chevauchement. | Fourniture des entrées en conflit pour affichage des indicateurs ; disparition du conflit lorsque l’utilisateur a résolu le chevauchement. | Vue calendrier, liste, dashboard (AGD-UI-06). |

**Résultat** : L’utilisateur est informé d’un éventuel conflit de dates avant validation ; il peut ajuster ou confirmer. Pour les événements **présence physique** : la confirmation ne bloque pas l’enregistrement ; l’utilisateur est notifié et JayKoa pousse à la résolution par alertes et indicateurs UI (rouge clignotant) jusqu’à ce que le conflit soit résolu.

### 1.3 Export de l’agenda (iCal, PDF)

| Étape | Action utilisateur | Rôle JayKoa | Écran / contexte |
|-------|--------------------|----------------------|-------------------|
| 1 | Ouverture de la page agenda ou du bloc export. | — | Page « Mon agenda », bloc export (AGD-UI-03). |
| 2 | Clic sur « Télécharger iCal » ou « Télécharger PDF » ; choix de la période si proposé. | Fourniture des entrées éligibles à l’export (selon Mandat et niveau de sécurité) ; génération du fichier (ou données pour génération côté service) ; pas de données sensibles ni de noms de tiers au-delà du niveau autorisé. | Bloc export (AGD-UI-03). |
| 3 | Téléchargement du fichier. | — | Navigateur. |

**Résultat** : L’utilisateur obtient un fichier iCal ou PDF contenant ses entrées agenda (RDV, événements, ateliers) dans le respect des règles de visibilité (AGD-SEC-3).

### 1.4 Agenda agrégé multi-sources (RDV + festivals + ateliers)

| Étape | Action utilisateur | Rôle JayKoa | Écran / contexte |
|-------|--------------------|----------------------|-------------------|
| 1 | Ouverture d’une page « Mon agenda » ou « Calendrier » qui agrège plusieurs sources (JayRDV + JayFestival). | Fourniture des entrées **agrégées** (toutes sources autorisées par le Mandat) ; une seule liste ou flux pour la vue calendrier. | Page agenda unifiée (si le service consommateur propose une telle page). |
| 2 | Filtre par source (RDV uniquement, festivals uniquement, etc.). | Fourniture des entrées filtrées par source ou type. | Vue calendrier + filtre (AGD-UI-04). |
| 3 | Consultation, export. | Même logique que 1.1 et 1.3 sur le périmètre filtré ou agrégé. | Vue calendrier, export. |

**Résultat** : L’utilisateur peut voir et exporter un agenda unifié (RDV + participations festivals + ateliers) lorsque les Mandats et les services consommateurs le permettent.

**Condition** : L’agrégation multi-sources n’est disponible que si (1) plusieurs services ont publié des entrées pour cet utilisateur et (2) le Mandat autorise l’agrégation (AGD-SEC-2).

### 1.5 Vue des événements publics sur son agenda et sélection pour valider sur son propre agenda

| Étape | Action utilisateur | Rôle JayKoa | Écran / contexte |
|-------|--------------------|----------------------|-------------------|
| 1 | Ouverture de la page « Découvrir », « Événements publics » ou zone « Ajouter à mon agenda » dans l’UI du service consommateur (JayFestival, JayRDV). | Fourniture de la **liste des événements publics** éligibles (plage, type, source, libellé court, id opaque) — sans données personnelles — pour période et filtres demandés (Kit Événements publics). | Vue événements publics (AGD-UI-09). |
| 2 | Consultation de la liste ou du catalogue d’événements publics (festivals ouverts, ateliers ouverts, créneaux publics). | Données déjà fournies à l’étape 1 ; mise à jour si changement de période ou de filtre. | Vue événements publics (AGD-UI-09). |
| 3 | **Sélection** d’un ou plusieurs événements et clic sur « Ajouter à mon agenda » (AGD-UI-10). | — | Sélection / Ajouter à mon agenda (AGD-UI-10). |
| 4 | Le service consommateur déclenche le flux métier (inscription, réservation) et **interroge** JayKoa pour **vérification conflit** (utilisateur U, plage de l’événement sélectionné). | Vérification conflit : retour conflit oui/non + liste des entrées en conflit (AGD-UI-02). | Appel JayKoa par le service consommateur. |
| 5 | Affichage d’une alerte si conflit (AGD-UI-02) ; l’utilisateur modifie, annule ou confirme. | Données de conflit si conflit. | Alerte conflit (AGD-UI-02). |
| 6 | Si confirmation : le service consommateur **enregistre** l’inscription ou la réservation (données métier) puis **publie** l’entrée vers JayKoa. | Enregistrement de l’entrée (Kit Entrées) ; l’événement apparaît dans « Mon agenda ». | Formulaire de confirmation, redirection vers « Mon agenda ». |
| 7 | L’utilisateur consulte « Mon agenda » : l’événement ajouté est visible dans la vue calendrier ou la liste. | Fourniture des entrées pour l’utilisateur (vue calendrier, liste) selon Mandat et filtres. | Vue calendrier (AGD-UI-01), liste (AGD-UI-07). |

**Résultat** : L’utilisateur **voit** les événements publics sur (ou à côté de) son agenda, **sélectionne** ceux qu’il souhaite ajouter, **valide** (avec vérification conflit) et les événements sont **enregistrés sur son propre agenda** ; JayKoa fournit la liste des événements publics et enregistre les entrées après validation par le service consommateur.

---

## 2. Parcours côté service (flux JayKoa ↔ services consommateurs)

Ces parcours décrivent les **flux techniques** entre les services consommateurs et JayKoa ; ils ne sont pas « vus » directement par l’utilisateur final mais sous-tendent les parcours ci-dessus.

### 2.1 Publication d’entrées agenda

| Étape | Acteur | Action |
|-------|--------|--------|
| 1 | Service consommateur (JayRDV, JayFestival) | Après validation métier (création RDV, validation candidature, inscription atelier), **publie** vers JayKoa les entrées agenda : plage (début, fin), type (RDV, édition, atelier), id opaque, référence utilisateur, niveau WorrySentinel. |
| 2 | JayKoa | Enregistre la **référence** (pas la copie canonique des données métier) ; met à jour les index pour conflits et vues agrégées. |
| 3 | JayKoa | Rend l’entrée disponible pour les requêtes (vue calendrier, conflit, export) selon Mandat et niveau de sécurité. |

### 2.2 Interrogation pour conflit

| Étape | Acteur | Action |
|-------|--------|--------|
| 1 | Service consommateur | Lors d’une action utilisateur (soumission candidature, création RDV), **interroge** JayKoa : « Y a-t-il un conflit pour l’utilisateur U et la plage [début, fin] ? » |
| 2 | JayKoa | Compare la plage avec les entrées existantes de l’utilisateur ; retourne conflit oui/non + liste des entrées en conflit (plage, type, libellé court). |
| 3 | Service consommateur | Affiche l’alerte (AGD-UI-02) ou bloque selon règle métier ; si l’utilisateur confirme, enregistre côté métier puis publie la nouvelle entrée (parcours 2.1). |

### 2.3 Interrogation pour vue calendrier ou export

| Étape | Acteur | Action |
|-------|--------|--------|
| 1 | Service consommateur | Lors de l’affichage de la page agenda ou de l’export, **interroge** JayKoa : « Entrées pour l’utilisateur U, période P, filtres F (source, type). » |
| 2 | JayKoa | Retourne les entrées éligibles (Mandat, niveau de sécurité, période, filtres). |
| 3 | Service consommateur | Affiche la vue calendrier (AGD-UI-01) ou génère le fichier iCal/PDF (AGD-UI-03) ; ne pas exposer de données au-delà du niveau autorisé. |

---

## 3. Synthèse des parcours

| Parcours | Côté utilisateur final | Côté service |
|----------|-------------------------|--------------|
| **Consultation agenda** | Ouverture page/calendrier → vue période/filtre → détail entrée (service). | Interrogation JayKoa (entrées, période, filtre). |
| **Vérification conflit** | Formulaire → soumission → alerte conflit → modification / annulation / confirmation. | Interrogation conflit → affichage alerte ; si confirmation → publication entrée. |
| **Export** | Clic export → choix période → téléchargement. | Interrogation entrées éligibles → génération fichier (JayKoa ou service). |
| **Agenda agrégé** | Ouverture page agrégée → filtre optionnel → consultation / export. | Interrogation JayKoa (agrégation multi-sources selon Mandat). |

---

## 4. Références

| Document | Rôle |
|----------|------|
| [JayKoa - Document Fondateur](./JayKoa%20-%20Document%20Fondateur.md) | Contexte, positionnement. |
| [JayKoa - Ecrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) | Composants UI (vue calendrier, alerte, export) intégrés dans ces parcours. |
| [JayKoa - Integration Services Consommateurs](./reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) | Responsabilités JayRDV, JayFestival, JayKoa ; séquence événements publics. |
| [JayKoa - Operateurs et Toolkits](./JayKoa%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs, Kits (Entrées, Conflits, Vue & Export, Événements publics). |

---

**Document** : JayKoa — Parcours utilisateurs  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (parcours)
