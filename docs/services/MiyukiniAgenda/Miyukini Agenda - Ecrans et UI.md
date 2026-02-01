# Miyukini Agenda — Écrans et UI

## Contexte

Miyukini Agenda est un **service de plateforme** consommé par JayRDV, Miyukini Festival Service et futurs services. Il **n’expose pas d’écrans directs** à l’utilisateur final : les écrans agenda sont **intégrés dans les UIs des services consommateurs** (dashboard pro, dashboard exposant, espace visiteur, etc.). Ce document précise les **besoins en écrans et en UI** que Miyukini Agenda **fournit ou recommande** pour être intégrés dans ces UIs : composants réutilisables, patterns d’écrans, zones et comportements.

## Portée / Scope

- **Périmètre** : Composants UI et patterns d’écrans agenda (vue calendrier, alerte conflit, export, filtres) ; intégration dans les UIs des services consommateurs (JayRDV, MFS).
- **Hors périmètre** : Maquettes graphiques détaillées, design system Miyukini (référencé ailleurs) ; écrans entièrement gérés par les services consommateurs (connexion, formulaire candidature, etc.).

---

## 1. Principe : écrans fournis vs écrans hébergés

| Type | Description | Exemple |
|------|-------------|---------|
| **Composants / patterns fournis ou recommandés par Miyukini Agenda** | Blocs ou zones que les services consommateurs **intègrent** dans leurs écrans (dashboard, page dédiée). Miyukini Agenda définit le **contrat** (données en entrée/sortie, comportement) ; l’implémentation UI peut être partagée (composant réutilisable) ou propre à chaque service. | Vue calendrier (jour/semaine/mois), alerte conflit, bloc export (iCal, PDF). |
| **Écrans hébergés par les services consommateurs** | Les écrans sont **détenus** par JayRDV, MFS, etc. ; ils **appellent** Miyukini Agenda (Opérateurs, Kits) pour les données et la logique (conflits, agrégation, export). | Dashboard exposant (MFS), tableau de bord pro (JayRDV), page « Mes RDV » (JayRDV). |

Les **besoins en écrans et UI** ci-dessous concernent les **composants et patterns** que Miyukini Agenda doit permettre de livrer (contrat, données, comportement), pas la maquette pixel-perfect.

---

## 2. Besoins en composants UI (écrans partiels)

### 2.1 Vue calendrier (jour / semaine / mois)

| Attribut | Description |
|----------|-------------|
| **Identifiant** | AGD-UI-01 |
| **Objectif** | Afficher les entrées agenda (RDV, éditions, participations, ateliers) sur une grille temporelle (jour, semaine, mois) avec possibilité d’agrégation multi-sources (JayRDV + MFS) selon Mandat. |
| **Données** | Liste d’entrées (plage début/fin, type, libellé court, source, id opaque) ; fuseau ; période affichée. Fournies par Miyukini Agenda (Kit/Opérateur) selon contexte utilisateur et Mandat. |
| **Organisation recommandée** | Grille ou calendrier : axe temps (heure ou jour) ; entrées sous forme de blocs ou barres (couleur par type ou par source) ; clic sur une entrée → détail (géré par le service consommateur, avec id opaque). |
| **Comportements** | Changement de période (jour/semaine/mois) ; filtre par type d’entrée ou par source (si agrégation) ; pas d’édition directe des entrées dans ce composant (l’édition reste dans le service d’origine). |
| **Intégration** | Bloc ou zone dans un écran « Dashboard », « Mon agenda », « Calendrier » du service consommateur (JayRDV Pro, MFS Exposant, MFS Visiteur). |

### 2.2 Alerte conflit de dates

| Attribut | Description |
|----------|-------------|
| **Identifiant** | AGD-UI-02 |
| **Objectif** | Signaler à l’utilisateur un **conflit de dates** (chevauchement) avant ou après validation d’une action (candidature, réservation, inscription). Pour les événements de type **présence physique**, la réservation ou l’entrée dans l’agenda **n’est pas bloquée** : l’utilisateur est notifié et peut confirmer ; Miyukini Agenda poussera ensuite à la résolution par alertes et indicateurs UI (voir AGD-UI-06). |
| **Données** | Résultat de vérification conflit (Miyukini Agenda) : conflit oui/non ; type d’événement (ex. présence physique) ; liste des entrées en conflit (plage, type, libellé court, source) si conflit. |
| **Organisation recommandée** | Bandeau ou modal : message clair (« Conflit de dates avec [événement X] / [RDV Y] ») ; liste des entrées en conflit ; actions « Modifier la date », « Annuler », « Confirmer malgré le conflit » (pour **présence physique** : confirmation possible, l’utilisateur sera notifié et les indicateurs conflit resteront actifs jusqu’à résolution). |
| **Comportements** | Affichage au moment de la soumission (dépôt candidature, création RDV) ou en amont (sélection de date) selon le flux du service consommateur. Pour **présence physique** : si l’utilisateur confirme, l’entrée est enregistrée ; les alertes et indicateurs UI (rouge clignotant) restent affichés tant que le conflit n’est pas résolu. |
| **Intégration** | Intégré dans les écrans de formulaire des services consommateurs : Dépôt candidature (MFS), Création RDV (JayRDV), Inscription atelier (MFS). |

### 2.3 Bloc export (iCal, PDF)

| Attribut | Description |
|----------|-------------|
| **Identifiant** | AGD-UI-03 |
| **Objectif** | Permettre à l’utilisateur d’**exporter** son agenda (ou une sélection) au format iCal ou PDF, sans exposer de données au-delà du niveau autorisé (WorrySentinel, AGD-SEC-3). |
| **Données** | Entrées agenda éligibles à l’export (plage, type, libellé) ; pas de noms de tiers ni de données sensibles en export public. Fournies par Miyukini Agenda selon Mandat et niveau de sécurité. |
| **Organisation recommandée** | Zone ou menu : boutons « Télécharger iCal », « Télécharger PDF » ; option « Période » (semaine, mois, tout) si pertinent ; message court sur le contenu exporté (ex. « Vos événements et RDV »). |
| **Comportements** | Génération du fichier côté Miyukini Agenda (ou service consommateur avec données Miyukini Agenda) ; téléchargement ; pas d’envoi automatique à un tiers sans consentement. |
| **Intégration** | Écran « Mon agenda », « Calendrier », ou page dédiée « Export » dans les espaces utilisateur (JayRDV Pro/Client, MFS Exposant/Visiteur). |

### 2.4 Filtre par source / type d’entrée

| Attribut | Description |
|----------|-------------|
| **Identifiant** | AGD-UI-04 |
| **Objectif** | Permettre à l’utilisateur de **filtrer** les entrées affichées (vue calendrier, liste) par **source** (JayRDV, MFS, etc.) ou par **type** (RDV, édition, atelier). |
| **Données** | Liste des sources et types disponibles pour l’utilisateur courant (selon Mandat et données publiées). Fournie par Miyukini Agenda. |
| **Organisation recommandée** | Liste déroulante, cases à cocher ou onglets : « Tous », « RDV uniquement », « Festivals uniquement », « Ateliers », etc. |
| **Comportements** | Mise à jour immédiate de la vue calendrier ou de la liste lors du changement de filtre. |
| **Intégration** | Associé à la vue calendrier (AGD-UI-01) ou à une liste d’entrées agenda. |

### 2.4bis Filtres de l’agenda (détaillés)

En plus du filtre **source / type** (AGD-UI-04), Miyukini Agenda supporte les **filtres détaillés** suivants, passés par le service consommateur lors des interrogations (voir [Integration Services Consommateurs](./reference/Miyukini%20Agenda%20-%20Integration%20Services%20Consommateurs.md) § 7) :

| Filtre | Description | Données fournies par Miyukini Agenda | Comportement UI recommandé |
|--------|-------------|--------------------------------------|----------------------------|
| **Période** | Date début, date fin (plage d’affichage). | Entrées dont la plage intersecte la période demandée. | Sélecteur de période (jour, semaine, mois) ; mise à jour de la vue. |
| **Source** | Service d’origine (JayRDV, MFS, etc.). | Entrées filtrées par source. | Liste déroulante ou onglets (AGD-UI-04). |
| **Type** | Type d’entrée (RDV, édition, atelier, participation, etc.). | Entrées filtrées par type. | Cases à cocher ou onglets (AGD-UI-04). |
| **Statut** | Statut métier si exposé (ex. candidat, inscrit, confirmé). | Entrées filtrées par statut (si le consommateur publie ce champ). | Liste déroulante ou onglets ; optionnel selon service. |
| **Visibilité** | Public vs privé : n’afficher que les entrées « public » (catalogue) ou « mes entrées » (privé). | Entrées publiques (pour vue événements publics) ou entrées de l’utilisateur (Mon agenda). | Deux vues ou onglets : « Mon agenda » (privé) vs « Événements publics » (public). |
| **Nature** | Nature de l’événement (ex. présence physique). | Entrées filtrées par nature ; utilisé pour règles de conflit et affichage. | Optionnel ; filtre avancé ou tri. |

Les **données** pour alimenter ces filtres (liste des sources, types, statuts disponibles) sont fournies par Miyukini Agenda selon Mandat et niveau de sécurité ; les UIs des services consommateurs affichent les options et passent les filtres sélectionnés lors des appels.

### 2.5 Indicateur « Prochaine entrée » / compte à rebours

| Attribut | Description |
|----------|-------------|
| **Identifiant** | AGD-UI-05 |
| **Objectif** | Afficher la **prochaine entrée** agenda (RDV, événement, atelier) et un compte à rebours (optionnel) pour renforcer la prise de conscience. |
| **Données** | Prochaine entrée (plage, type, libellé, source) ; date/heure de début. Fournie par Miyukini Agenda. |
| **Organisation recommandée** | Bloc compact : « Prochain : [libellé] — [date/heure] » ou « Dans X jours / X heures ». Lien vers détail ou vers la vue calendrier. |
| **Comportements** | Mise à jour selon l’heure (MiyuClock) ; masqué si aucune entrée à venir. |
| **Intégration** | Dashboard ou page d’accueil des espaces utilisateur (JayRDV Pro, MFS Exposant, MFS Visiteur). |

### 2.6 Indicateur conflit non résolu (présence physique)

| Attribut | Description |
|----------|-------------|
| **Identifiant** | AGD-UI-06 |
| **Objectif** | Pour les événements de type **présence physique** en conflit (chevauchement non résolu), **pousser l’utilisateur à résoudre** le conflit horaire par des **alertes** et des **indices UI** visibles tant que le conflit persiste. La réservation ou l’entrée dans l’agenda n’est pas bloquée (AGD-SEC-6) ; Miyukini Agenda signale le conflit et incite à la résolution. |
| **Données** | Liste des entrées en conflit (présence physique) pour l’utilisateur courant ; plage, type, libellé, source. Fournie par Miyukini Agenda. |
| **Organisation recommandée** | **Vue calendrier / liste** : entrées en conflit affichées avec un **indicateur visuel rouge clignotant** (ou équivalent accessible) ; **bandeau ou alerte** récurrente (ex. « Vous avez X conflits de présence physique à résoudre ») avec lien vers l’agenda ou la liste des entrées en conflit. L’indicateur reste actif jusqu’à ce que l’utilisateur annule, reporte ou modifie l’un des événements pour supprimer le chevauchement. |
| **Comportements** | Affichage persistant tant qu’il existe au moins une paire d’entrées « présence physique » en chevauchement ; disparition de l’alerte et du clignotant lorsque le conflit est résolu. Accessibilité : prévoir une alternative au clignotant (icône rouge fixe, bordure, texte « Conflit ») pour respecter les recommandations WCAG (éviter le clignotement excessif ou proposer une option pour le désactiver). |
| **Intégration** | Vue calendrier (AGD-UI-01), liste d’entrées agenda, dashboard (bandeau alerte). |

---

## 3. Synthèse des besoins écrans / UI

| Identifiant | Composant | Intégration principale |
|-------------|-----------|-------------------------|
| **AGD-UI-01** | Vue calendrier (jour/semaine/mois) | Dashboard, page « Mon agenda » (JayRDV, MFS). |
| **AGD-UI-02** | Alerte conflit de dates | Formulaires candidature, réservation, inscription (MFS, JayRDV). |
| **AGD-UI-03** | Bloc export (iCal, PDF) | Page agenda, page dédiée export. |
| **AGD-UI-04** | Filtre par source / type | Associé à la vue calendrier ou liste. |
| **AGD-UI-05** | Indicateur prochaine entrée | Dashboard, accueil. |
| **AGD-UI-06** | Indicateur conflit non résolu (présence physique) | Vue calendrier, liste, dashboard ; rouge clignotant + alertes jusqu’à résolution. |
| **AGD-UI-09** | Vue catalogue / liste événements publics | Page ou zone « Découvrir », « Événements publics », « Ajouter à mon agenda ». |
| **AGD-UI-10** | Sélection / Ajouter à mon agenda | Bouton(s) « Ajouter à mon agenda » ; vérification conflit ; confirmation ; entrée enregistrée. |

### 3.1 Composants optionnels (référentiel inspiré Google Agenda)

Les composants suivants sont inspirés des usages des agendas grand public (voir [Référentiel fonctionnel inspiré Google Agenda](./reference/Miyukini%20Agenda%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md)) ; leur intégration est prévue en **phase 2** ou selon besoin :

| Identifiant | Composant | Description |
|-------------|-----------|-------------|
| **AGD-UI-07** | Vue liste / agenda | Vue **liste chronologique** des entrées à venir (ordre temporel, sans grille), équivalent « Agenda » de Google Agenda. Données : liste d’entrées (plage, type, libellé, source) pour la période demandée. |
| **AGD-UI-08** | Indicateur libre / occupé | Exposition **sans détail** des plages libres vs occupées (pour qu’un tiers propose des créneaux sans voir le détail des entrées). Données : agrégat occupé oui/non par plage ; soumis à Mandat avec périmètre « libre/occupé » uniquement. |

---

## 4. Règles UI et sécurité

- **Visibilité** : Seules les entrées autorisées par le Mandat et le niveau de sécurité (WorrySentinel) sont affichées ; les composants ne doivent pas exposer de données au-delà du niveau du contexte.
- **Export** : Le bloc export (AGD-UI-03) ne doit pas inclure de noms de tiers ni de données sensibles en export partagé (voir [Niveaux Sécurité et Protection](./reference/Miyukini%20Agenda%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md)).
- **Présence physique en conflit** (AGD-SEC-6, AGD-UI-06) : pour les événements de type présence physique en conflit, la réservation ou l’entrée dans l’agenda n’est pas bloquée ; l’utilisateur est notifié et Miyukini Agenda pousse à la résolution par **alertes** et **indicateurs UI** (rouge clignotant ou équivalent accessible) jusqu’à résolution du chevauchement.
- **Accessibilité** : Les composants recommandés doivent être conçus pour être accessibles (WCAG, contraste, navigation clavier) selon les standards du design system Miyukini. Pour l’indicateur conflit (AGD-UI-06), prévoir une alternative au clignotant (icône rouge fixe, bordure, texte « Conflit ») ou une option pour désactiver le clignotement.

---

## 5. Références

| Document | Rôle |
|----------|------|
| [Miyukini Agenda - Document Fondateur](./Miyukini%20Agenda%20-%20Document%20Fondateur.md) | Contexte, positionnement. |
| [Miyukini Agenda - Parcours Utilisateurs](./Miyukini%20Agenda%20-%20Parcours%20Utilisateurs.md) | Parcours dans lesquels ces composants s’insèrent. |
| [Miyukini Agenda - Bornage Implementation](./Miyukini%20Agenda%20-%20Bornage%20Implementation.md) | Périmètre d’implémentation des composants (MVP, phases). |
| [Miyukini Agenda - Referentiel Fonctionnel Inspire Google Agenda](./reference/Miyukini%20Agenda%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | Référentiel inspiré de Google Agenda (vues liste/agenda, libre/occupé, rappels, partage). |

---

**Document** : Miyukini Agenda — Écrans et UI  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (écrans, UI)
