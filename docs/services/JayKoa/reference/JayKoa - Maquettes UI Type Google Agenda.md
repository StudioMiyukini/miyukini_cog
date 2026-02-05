# JayKoa — Maquettes UI type Google Agenda

## Contexte

Ce document décrit une **copie de l’UI de Google Agenda** (Google Calendar) adaptée au contexte JayKoa : structure générale, **fonctionnalités de tri**, **filtre** et **affichage par jour / 3 jours / semaine / mois**. Les **maquettes ASCII** servent de référence pour les composants à intégrer dans les UIs des services consommateurs (JayRDV, Miyukini Festival Service).

**Objectif** : Fournir un modèle de mise en page et de comportement (barre d’outils, vues, filtres, tri) sans imposer un design graphique — les services consommateurs peuvent s’en inspirer pour leurs écrans « Mon agenda » ou « Calendrier ».

## Portée / Scope

- **Périmètre** : Structure de l’écran type Google Agenda ; vues Jour, 3 jours, Semaine, Mois ; barre de filtres et tri ; maquettes ASCII (wireframes).
- **Hors périmètre** : Design system Miyukini (couleurs, typo), spécifications API (référencées dans Operateurs et Toolkits, Integration).

---

## 1. Fonctionnalités documentées

### 1.1 Affichage (vues)

| Vue | Description | Granularité |
|-----|-------------|-------------|
| **Jour** | Un seul jour affiché ; grille horaire (ex. 6h–22h) avec créneaux et événements en barres. | Heure |
| **3 jours** | Trois colonnes (3 jours consécutifs) ; même grille horaire par colonne. | Heure |
| **Semaine** | Sept colonnes (lun–dim ou dim–sam selon préférence) ; grille horaire commune. | Heure |
| **Mois** | Grille calendrier du mois ; chaque cellule = un jour ; événements en libellé court ou indicateur de charge. | Jour |

### 1.2 Tri

| Tri | Description | Appliqué sur |
|-----|-------------|----------------|
| **Chronologique** | Ordre par date/heure de début (défaut pour les vues jour/3j/semaine). | Liste d’entrées dans une plage |
| **Par type** | Grouper ou ordonner par type d’entrée (RDV, édition, atelier). | Liste (vue Agenda/liste) |
| **Par source** | Grouper ou ordonner par source (JayRDV, JayFestival). | Liste, vue agrégée |
| **Par priorité / statut** | Si le consommateur expose un champ priorité ou statut (ex. candidat, inscrit). | Liste |

Le **tri** s’applique aux données fournies par JayKoa (paramètre de requête ou tri côté client après réception).

### 1.3 Filtres

| Filtre | Description | Zone UI type |
|--------|-------------|--------------|
| **Source** | Afficher uniquement RDV (JayRDV), ou uniquement Festivals (JayFestival), ou Tous. | Barre d’outils ou panneau latéral |
| **Type** | RDV, Édition, Atelier, Participation, etc. | Barre d’outils ou panneau latéral |
| **Calendriers / couches** | Afficher ou masquer des « calendriers » (équivalent sources ou types) ; case à cocher par source/type. | Panneau latéral (liste de calendriers avec case à cocher) |
| **Période** | Date début, date fin (souvent géré par la vue : jour = une date, semaine = 7 jours, etc.). | Sélecteur de date + vue |
| **Statut** | Candidat, Inscrit, Confirmé (si exposé par le consommateur). | Barre d’outils ou filtre avancé |
| **Visibilité** | Public (événements publics) vs Mon agenda (entrées personnelles). | Onglets ou panneau |

### 1.4 Navigation temporelle

| Élément | Description |
|---------|-------------|
| **Précédent / Suivant** | Flèches ou boutons pour avancer/reculer d’un jour, d’une semaine ou d’un mois selon la vue. |
| **Aujourd’hui** | Bouton pour revenir à la date du jour. |
| **Sélecteur de date** | Mini-calendrier ou champ date pour aller à une date précise. |
| **Période affichée** | Libellé de la période courante (ex. « 27 janv. – 2 févr. 2026 »). |

---

## 2. Structure générale de l’écran (maquette ASCII)

```
+--------------------------------------------------------------------------------------------------+
|  [Logo / Titre]    Mon agenda                    [Recherche...]     [Export v] [Paramètres] [User]|
+--------------------------------------------------------------------------------------------------+
|  [<] [Aujourd'hui] [>]    |  Jour  | 3 jours | Semaine | Mois  |  Liste  |   27 janv. – 2 févr. 2026  |
+--------------------------------------------------------------------------------------------------+
| Filtres / Calendriers     |                                                                      |
| [x] Tous                  |                                                                      |
| [x] JayRDV (RDV)          |                    ZONE PRINCIPALE (VUE)                             |
| [x] JayFestival (Festivals)       |                    Jour / 3j / Semaine / Mois / Liste                |
| [ ] Ateliers              |                                                                      |
| [x] Participations        |                    Grille ou liste selon vue sélectionnée            |
|                           |                                                                      |
| Tri: [Chronologique v]     |                                                                      |
|                           |                                                                      |
+---------------------------+----------------------------------------------------------------------+
|  Prochaine entrée : RDV Dr Martin — lun. 28 janv. 14h00                    [Voir agenda]         |
+--------------------------------------------------------------------------------------------------+
```

---

## 3. Maquette ASCII — Vue Jour

```
+--------------------------------------------------------------------------------------------------+
|  [<] [Aujourd'hui] [>]    | *Jour* | 3 jours | Semaine | Mois  |  Liste  |   Lundi 27 janvier 2026   |
+--------------------------------------------------------------------------------------------------+
| Filtres                    | Heure    | Lundi 27 janvier 2026                                    |
| [x] Tous                   +----------+----------------------------------------------------------+
| [x] RDV                    | 06:00    |                                                          |
| [x] Festivals              | 07:00    |                                                          |
|                             | 08:00    |  [==== RDV Dr Martin 09h00 =====]                         |
|                             | 09:00    |  [==== 1h ======================]                         |
|                             | 10:00    |                                                          |
|                             | 11:00    |  [=== Festival X - Stand ===]                             |
|                             | 12:00    |  [=== 12h-18h ================]                            |
|                             | 13:00    |                                                          |
|                             | ...      |                                                          |
|                             | 18:00    |                                                          |
|                             | 19:00    |  [= Atelier Y 19h30 =]                                     |
|                             | 20:00    |                                                          |
+-----------------------------+----------+----------------------------------------------------------+
```

---

## 4. Maquette ASCII — Vue 3 jours

```
+--------------------------------------------------------------------------------------------------+
|  [<] [Aujourd'hui] [>]    |  Jour  | *3 jours* | Semaine | Mois  |  Liste  |  27-29 janv. 2026       |
+--------------------------------------------------------------------------------------------------+
| Filtres                    | Heure    | Lun 27      | Mar 28      | Mer 29                          |
| [x] Tous                   +----------+-------------+-------------+--------------------------------+
| [x] RDV                    | 08:00    |             | [RDV 09h]   |                                 |
| [x] Festivals              | 09:00    | [RDV Dr M.] | [======]    | [Festival X]                    |
|                             | 10:00    | [======]    |             | [Stand 10h-18h]                 |
|                             | 11:00    |             |             | [=============]                 |
|                             | 12:00    | [Festival X]|             |                                 |
|                             | ...      | [Stand]     |             |                                 |
|                             | 18:00    |             |             |                                 |
+-----------------------------+----------+-------------+-------------+--------------------------------+
```

---

## 5. Maquette ASCII — Vue Semaine

```
+--------------------------------------------------------------------------------------------------+
|  [<] [Aujourd'hui] [>]    |  Jour  | 3 jours | *Semaine* | Mois  |  Liste  |  27 janv. – 2 févr. 2026 |
+--------------------------------------------------------------------------------------------------+
| Filtres                    | Heure    | Lun 27 | Mar 28 | Mer 29 | Jeu 30 | Ven 31 | Sam 1 | Dim 2 |
| [x] Tous                   +----------+--------+--------+--------+--------+--------+-------+-------+
| [x] RDV                    | 08:00    |        | [RDV]  |        |        |        |       |       |
| [x] Festivals              | 09:00    | [RDV]  | [===]  | [Fest] |        |        |       |       |
|                             | 10:00    | [===]  |        | [Stand]|        |        |       |       |
|                             | 11:00    |        |        | [=====]|        |        |       |       |
|                             | 12:00    | [Fest] |        |        |        |        |       |       |
|                             | ...      | [Stand]|        |        |        |        |       |       |
|                             | 18:00    |        |        |        |        |        |       |       |
+-----------------------------+----------+--------+--------+--------+--------+--------+-------+-------+
```

---

## 6. Maquette ASCII — Vue Mois

```
+--------------------------------------------------------------------------------------------------+
|  [<] [Aujourd'hui] [>]    |  Jour  | 3 jours | Semaine | *Mois* |  Liste  |     Janvier 2026         |
+--------------------------------------------------------------------------------------------------+
| Filtres                    |  Lun   |  Mar   |  Mer   |  Jeu   |  Ven   |  Sam   |  Dim              |
| [x] Tous                   +-------+-------+-------+-------+-------+-------+-------+
| [x] RDV                    |   1   |   2   |   3   |   4   |   5   |   6   |   7   |
| [x] Festivals               |       |       | RDV   |       |       |       |       |
|                             +-------+-------+-------+-------+-------+-------+-------+
|                             |   8   |   9   |  10   |  11   |  12   |  13   |  14   |
|                             |       | Fest. |       |       |       |       |       |
|                             +-------+-------+-------+-------+-------+-------+-------+
|                             |  15   |  16   |  17   |  18   |  19   |  20   |  21   |
|                             +-------+-------+-------+-------+-------+-------+-------+
|                             |  22   |  23   |  24   |  25   |  26   | *27*  |  28   |
|                             |       |       |       |       |       | Auj.  |       |
|                             +-------+-------+-------+-------+-------+-------+-------+
|                             |  29   |  30   |  31   |       |       |       |       |
|                             |       |       |       |       |       |       |       |
+-----------------------------+-------+-------+-------+-------+-------+-------+-------+
     Légende: *27* = jour courant ; libellés courts ou indicateur (• 3 événements) par cellule.
```

---

## 7. Maquette ASCII — Barre de filtres et tri (détail)

```
+------------------------------------------+
| Filtres / Calendriers                    |
+------------------------------------------+
| [x] Tous les calendriers                 |
| ---------------------------------------- |
| [x] JayRDV — Mes RDV         [couleur]   |
| [x] JayFestival — Participations     [couleur]   |
| [x] JayFestival — Ateliers           [couleur]   |
| [ ] JayFestival — Candidatures       [couleur]   |
| ---------------------------------------- |
| Tri : [ Chronologique ▼ ]                |
|       • Chronologique                    |
|       • Par type                         |
|       • Par source                       |
|       • Par statut                       |
| ---------------------------------------- |
| Période : [ Semaine courante ▼ ]         |
| Début semaine : [ Lundi ▼ ]              |
+------------------------------------------+
```

---

## 8. Maquette ASCII — Vue Liste (Agenda)

```
+--------------------------------------------------------------------------------------------------+
|  [<] [Aujourd'hui] [>]    |  Jour  | 3 jours | Semaine | Mois  | *Liste* |   Prochains événements   |
+--------------------------------------------------------------------------------------------------+
| Filtres                    | Date       | Heure  | Événement              | Source   | Type     |
| [x] Tous                   +------------+--------+-------------------------+----------+----------+
| [x] RDV                    | Lun 27 jan | 09:00  | RDV Dr Martin           | JayRDV   | RDV      |
| [x] Festivals              | Lun 27 jan | 12:00  | Festival X — Stand      | JayFestival      | Particip.|
|                             | Mar 28 jan | 14:00  | Atelier Y                | JayFestival      | Atelier  |
| Tri: [Chronologique v]      | Mer 29 jan | 10:00  | Festival X — Stand       | JayFestival      | Particip.|
|                             | ...        |        |                          |          |          |
+-----------------------------+------------+--------+-------------------------+----------+----------+
```

---

## 9. Correspondance avec les composants JayKoa

| Zone / fonctionnalité | Composant JayKoa | Données fournies par |
|-----------------------|----------------------------|----------------------|
| **Zone principale (grille)** | AGD-UI-01 (Vue calendrier jour/semaine/mois) | JayKoa — entrées pour utilisateur, période, filtres. |
| **Vue 3 jours** | AGD-UI-01 (option période = 3 jours) | JayKoa — même contrat, période = 3 jours. |
| **Vue Liste** | AGD-UI-07 (Vue liste / agenda) | JayKoa — liste chronologique. |
| **Barre filtres (source, type)** | AGD-UI-04 (Filtre par source / type) | JayKoa — liste des sources/types ; paramètres de requête. |
| **Tri** | Paramètre de requête ou tri côté client | JayKoa — entrées ; tri chronologique par défaut (date début). |
| **Navigation (date, Aujourd’hui)** | Données période ; MiyuClock (date courante) | Service consommateur + JayKoa (entrées pour nouvelle période). |
| **Export** | AGD-UI-03 (Bloc export iCal, PDF) | JayKoa — export selon Mandat. |
| **Prochaine entrée** | AGD-UI-05 (Indicateur prochaine entrée) | JayKoa — prochaine entrée. |

---

## 10. Références

| Document | Rôle |
|----------|------|
| [JayKoa - Ecrans et UI](../JayKoa%20-%20Ecrans%20et%20UI.md) | Composants AGD-UI-01 à 10, filtres détaillés. |
| [JayKoa - Referentiel Fonctionnel Inspire Google Agenda](./JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | Référentiel Google Agenda (vues, partage, etc.). |
| [JayKoa - Operateurs et Toolkits](../JayKoa%20-%20Operateurs%20et%20Toolkits.md) | Contrat données (période, filtres, tri). |

---

**Document** : JayKoa — Maquettes UI type Google Agenda  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (maquettes ASCII, vues, tri, filtres)
