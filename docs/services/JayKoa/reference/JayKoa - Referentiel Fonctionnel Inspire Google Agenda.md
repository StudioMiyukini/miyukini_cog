# JayKoa â€” RÃ©fÃ©rentiel fonctionnel inspirÃ© de Google Agenda

## Contexte

Ce document constitue un **rÃ©fÃ©rentiel fonctionnel** pour JayKoa, inspirÃ© des capacitÃ©s des agendas grand public (notamment **Google Agenda / Google Calendar**). Il ne dÃ©crit pas Google Agenda en tant que tel : il **traduit** les concepts et usages repÃ©rÃ©s (vues, rappels, partage, libre/occupÃ©, calendriers multiples, etc.) en **capacitÃ©s et contraintes JayKoa**, dans le cadre COG (Mandats, WorrySentinel, services consommateurs).

**Objectif** : Enrichir la documentation conceptuelle et contractuelle de JayKoa en sâ€™appuyant sur des usages Ã©prouvÃ©s, tout en respectant la gouvernance, la sÃ©curitÃ© et le bornage du service.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : RÃ©fÃ©rentiel des capacitÃ©s inspirÃ©es de Google Agenda ; correspondance avec les documents fondateurs et les Ã©crans/parcours JayKoa.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques (API, schÃ©mas) ; description dÃ©taillÃ©e de Google Agenda.
- **RÃ©fÃ©rences** : [Document Fondateur](../JayKoa%20-%20Document%20Fondateur.md), [Ã‰crans et UI](../JayKoa%20-%20Ecrans%20et%20UI.md), [Parcours Utilisateurs](../JayKoa%20-%20Parcours%20Utilisateurs.md), [Protocole dâ€™Ã©criture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 1. Vues calendrier (inspirÃ©es Google Agenda)

Google Agenda propose plusieurs **vues** : Jour, Semaine, Mois, AnnÃ©e, **Agenda** (liste chronologique des Ã©vÃ©nements Ã  venir), et des vues personnalisÃ©es (ex. 4 jours). JayKoa positionne ces capacitÃ©s comme suit :

| Concept Google | CapacitÃ© JayKoa | RÃ©fÃ©rence |
|----------------|--------------------------|-----------|
| **Vue Jour** | Vue calendrier **jour** : grille horaire, entrÃ©es (RDV, Ã©vÃ©nements, ateliers) sur une journÃ©e. | AGD-UI-01 (vue calendrier jour/semaine/mois). |
| **Vue Semaine** | Vue calendrier **semaine** : grille 7 jours (ou personnalisable), entrÃ©es par jour. | AGD-UI-01. |
| **Vue Mois** | Vue calendrier **mois** : aperÃ§u du mois, indicateur de charge ou entrÃ©es par jour. | AGD-UI-01. |
| **Vue Agenda / Liste** | Vue **liste chronologique** des entrÃ©es Ã  venir (ordre temporel, sans grille) : Â« Prochains Ã©vÃ©nements Â». | Ã€ intÃ©grer comme **option** de la vue calendrier (AGD-UI-01) ou composant dÃ©diÃ© **AGD-UI-07** (vue liste/agenda). |
| **Vue AnnÃ©e** | AperÃ§u **annÃ©e** : vue synthÃ©tique (optionnel, phase 2 ou 3). | Hors scope MVP ; Ã  traiter en phase ultÃ©rieure. |
| **DÃ©but de semaine personnalisable** | ParamÃ¨tre utilisateur ou service (lundi vs dimanche) ; MiyuClock / fuseau. | DonnÃ©es fournies par JayKoa (fuseau, prÃ©fÃ©rence dÃ©but de semaine si stockÃ©e par le service consommateur). |

**RÃ¨gle** : Les vues sont **fournies en donnÃ©es** par JayKoa ; lâ€™affichage (grille, liste) est assurÃ© par les UIs des services consommateurs (JayRDV, JayFestival). Les composants recommandÃ©s (AGD-UI-01, et option liste/agenda) dÃ©crivent le **contrat** (pÃ©riode, filtres, entrÃ©es).

---

## 2. Rappels et notifications (inspirÃ©s Google Agenda)

Google Agenda permet des **rappels** par Ã©vÃ©nement (notification X minutes/heures/jours avant). JayKoa ne gÃ¨re pas lâ€™envoi direct des notifications (dÃ©lÃ©guÃ© Ã  Miyunotify ou aux services consommateurs) ; il peut **fournir les donnÃ©es** nÃ©cessaires pour dÃ©clencher les rappels :

| Concept Google | CapacitÃ© JayKoa | RÃ©fÃ©rence |
|----------------|--------------------------|-----------|
| **Rappel avant Ã©vÃ©nement** | JayKoa fournit les **entrÃ©es Ã  venir** (plage, type, libellÃ©, id opaque) ; le **service consommateur** ou **Miyunotify** dÃ©clenche le rappel (email, push, in-app) selon la rÃ¨gle mÃ©tier et les prÃ©fÃ©rences utilisateur. | Parcours utilisateurs ; Bornage (phase 3 : rappels). |
| **Rappel Â« prochaine entrÃ©e Â»** | DonnÃ©es pour lâ€™indicateur **prochaine entrÃ©e** (AGD-UI-05) et, en phase ultÃ©rieure, pour un rappel automatique (Miyunotify). | AGD-UI-05. |

**RÃ¨gle** : JayKoa **ne envoie pas** les notifications ; il **expose les entrÃ©es** (dates, types) pour que les services consommateurs ou Miyunotify dÃ©clenchent les rappels dans le respect des Mandats et du niveau de sÃ©curitÃ©.

---

## 3. Partage et niveaux dâ€™accÃ¨s (inspirÃ©s Google Agenda)

Google Agenda propose un **partage au niveau calendrier** avec des niveaux : voir uniquement **libre/occupÃ©** (sans dÃ©tail), **voir tous les Ã©vÃ©nements**, **modifier les Ã©vÃ©nements**, **gÃ©rer le partage**. JayKoa transpose ces notions dans le cadre **Mandats et permissions** :

| Concept Google | CapacitÃ© JayKoa | RÃ©fÃ©rence |
|----------------|--------------------------|-----------|
| **Libre / OccupÃ©** | Exposition dâ€™un **indicateur de disponibilitÃ©** (plages libres vs occupÃ©es) **sans dÃ©tail des entrÃ©es** : pour quâ€™un tiers (ex. professionnel JayRDV, organisateur JayFestival) puisse proposer des crÃ©neaux sans voir le dÃ©tail des RDV ou Ã©vÃ©nements. | Niveau de visibilitÃ© **0â€“1** ; donnÃ©es agrÃ©gÃ©es (occupÃ© oui/non par plage) ; Mandat avec pÃ©rimÃ¨tre Â« libre/occupÃ© Â» uniquement. Ã€ documenter comme **option** (phase 2) : AGD-UI-08 (vue libre/occupÃ©) ou API dÃ©diÃ©e. |
| **Voir les Ã©vÃ©nements** | AccÃ¨s **lecture** aux entrÃ©es agenda (plage, type, libellÃ©) selon Mandat et Master Butler. | Vue calendrier (AGD-UI-01), liste ; Niveaux SÃ©curitÃ©. |
| **Modifier les Ã©vÃ©nements** | La **modification** des entrÃ©es reste du ressort des **services consommateurs** (JayRDV, JayFestival) ; JayKoa enregistre les **rÃ©fÃ©rences** mises Ã  jour (plage, type) aprÃ¨s validation mÃ©tier. | Integration Services Consommateurs. |
| **GÃ©rer le partage** | Ã‰quivalent COG : **StrongFather** (Mandats), **Master Butler** (permissions) ; pas de Â« propriÃ©taire calendrier Â» unique hors contexte utilisateur et service. | Document Fondateur, Niveaux SÃ©curitÃ©. |

**RÃ¨gle** : Le partage et les niveaux dâ€™accÃ¨s sont gouvernÃ©s par **Mandats de Permission** et **niveaux de sÃ©curitÃ©** (WorrySentinel) ; JayKoa nâ€™expose que les donnÃ©es autorisÃ©es pour le contexte (libre/occupÃ©, dÃ©tail, export).

---

## 4. Calendriers multiples et agrÃ©gation (inspirÃ©s Google Agenda)

Google Agenda permet **plusieurs calendriers** (personnel, travail, etc.) affichÃ©s ensemble ou sÃ©parÃ©ment, avec couleurs par calendrier. JayKoa gÃ¨re lâ€™**agrÃ©gation multi-sources** (plusieurs services : JayRDV, JayFestival) pour un mÃªme utilisateur :

| Concept Google | CapacitÃ© JayKoa | RÃ©fÃ©rence |
|----------------|--------------------------|-----------|
| **Plusieurs calendriers** | **Plusieurs sources** dâ€™entrÃ©es (JayRDV, JayFestival, futurs services) pour un mÃªme utilisateur ; affichage agrÃ©gÃ© ou filtrÃ© par source/type. | Document Fondateur (Â§ 3 IntÃ©gration), Integration Services Consommateurs, AGD-UI-04 (filtre par source/type). |
| **Couleur / distinction par source** | Les entrÃ©es sont **typÃ©es et sourcÃ©es** (type, source) ; les UIs des services consommateurs peuvent afficher des couleurs ou styles par source/type. | DonnÃ©es fournies par JayKoa (type, source) ; implÃ©mentation UI cÃ´tÃ© consommateur. |
| **Masquer / afficher une source** | Filtre par **source** ou **type** (AGD-UI-04) : lâ€™utilisateur choisit dâ€™afficher uniquement RDV, ou uniquement festivals, ou tout. | AGD-UI-04. |

**RÃ¨gle** : Lâ€™agrÃ©gation multi-sources est soumise au **Mandat** et au **niveau de sÃ©curitÃ©** (AGD-SEC-2) ; JayKoa fournit les entrÃ©es agrÃ©gÃ©es ou filtrÃ©es selon le contexte.

---

## 5. Invitations Ã  un Ã©vÃ©nement (inspirÃ©es Google Agenda)

Google Agenda permet dâ€™**inviter des personnes** Ã  un Ã©vÃ©nement (acceptation, refus, peut-Ãªtre) et de voir les rÃ©ponses. JayKoa ne gÃ¨re pas directement les invitations ni les rÃ©ponses : celles-ci relÃ¨vent des **services consommateurs** (JayFestival pour un atelier, JayRDV pour un RDV). JayKoa peut :

| Concept Google | CapacitÃ© JayKoa | RÃ©fÃ©rence |
|----------------|--------------------------|-----------|
| **Inviter / accepter / refuser** | GÃ©rÃ© par le **service consommateur** (crÃ©ation Ã©vÃ©nement, envoi invitation, mise Ã  jour statut). JayKoa reÃ§oit les **entrÃ©es agenda** une fois lâ€™inscription ou la rÃ©servation validÃ©e (plage, type, rÃ©fÃ©rence utilisateur). | Integration Services Consommateurs. |
| **Voir qui est invitÃ©** | DonnÃ©es mÃ©tier (liste des invitÃ©s, rÃ©ponses) dÃ©tenues par le service consommateur ; JayKoa ne stocke pas la liste des invitÃ©s, uniquement les **rÃ©fÃ©rences** des entrÃ©es (qui a quelle plage). | AGD-SEC-1 (pas de copie canonique des donnÃ©es mÃ©tier). |

**RÃ¨gle** : Les invitations et rÃ©ponses sont du **domaine mÃ©tier** des services consommateurs ; JayKoa travaille sur les **entrÃ©es agenda** (plages, types, conflits, vues, export).

---

## 6. TÃ¢ches vs Ã©vÃ©nements (inspirÃ© Google Agenda)

Google Agenda distingue **Ã©vÃ©nements** (avec date/heure) et **tÃ¢ches** (to-do, optionnellement datÃ©es). JayKoa se concentre sur les **entrÃ©es temporelles** (plages dÃ©but/fin) :

| Concept Google | CapacitÃ© JayKoa | RÃ©fÃ©rence |
|----------------|--------------------------|-----------|
| **Ã‰vÃ©nement** | **EntrÃ©e agenda** avec plage (dÃ©but, fin), type, source. Pleinement couvert. | Document Fondateur, tous les docs. |
| **TÃ¢che (to-do)** | Une **tÃ¢che** peut Ãªtre modÃ©lisÃ©e comme entrÃ©e avec **plage optionnelle** ou **date dâ€™Ã©chÃ©ance** si un service consommateur la publie vers JayKoa (ex. Â« Ã€ faire avant le X Â»). Sinon, les tÃ¢ches restent hors pÃ©rimÃ¨tre JayKoa (gÃ©rÃ©es par un autre module ou service). | Bornage : tÃ¢ches avec date/plage peuvent Ãªtre des entrÃ©es ; tÃ¢ches sans date hors pÃ©rimÃ¨tre MVP. |

**RÃ¨gle** : JayKoa couvre les **entrÃ©es avec plage temporelle** (ou date dâ€™Ã©chÃ©ance) ; les tÃ¢ches sans dimension temporelle explicite ne sont pas dans le pÃ©rimÃ¨tre cÅ“ur.

---

## 7. Export et synchronisation (inspirÃ©s Google Agenda)

Google Agenda permet lâ€™**export** (iCal, etc.) et la **synchronisation** avec dâ€™autres calendriers (Apple, Outlook). JayKoa :

| Concept Google | CapacitÃ© JayKoa | RÃ©fÃ©rence |
|----------------|--------------------------|-----------|
| **Export iCal / PDF** | **Export iCal et PDF** (MVP : iCal ; phase 2 : PDF) ; pas de donnÃ©es au-delÃ  du niveau autorisÃ© (AGD-SEC-3). | AGD-UI-03, Document Fondateur, Bornage. |
| **Synchronisation externe** | **Hors scope MVP** ; en phase ultÃ©rieure, les services consommateurs ou une extension peuvent consommer les donnÃ©es JayKoa pour synchroniser avec des calendriers externes (Google, Outlook, Apple). | Bornage (phase 3). |
| **AccÃ¨s hors ligne** | Lâ€™Ã©cosystÃ¨me Miyukini accepte lâ€™**isolement comme Ã©tat normal** (LOI-2) ; lâ€™accÃ¨s hors ligne relÃ¨ve de lâ€™architecture des services consommateurs et du COG (cache, synchronisation diffÃ©rÃ©e). | Document Fondateur (LOI-2), hors pÃ©rimÃ¨tre direct JayKoa. |

**RÃ¨gle** : Export contrÃ´lÃ© (Mandat, niveau de sÃ©curitÃ©) ; synchronisation externe et hors ligne en phase ultÃ©rieure ou dÃ©lÃ©guÃ©es aux consommateurs.

---

## 8. SynthÃ¨se : correspondance Google Agenda â†’ JayKoa

| Domaine | Google Agenda | JayKoa (rÃ©fÃ©rentiel) |
|---------|---------------|-------------------------------|
| **Vues** | Jour, Semaine, Mois, AnnÃ©e, Agenda (liste) | AGD-UI-01 (jour/semaine/mois) ; option vue liste/agenda (AGD-UI-07) ; annÃ©e en phase 2+. |
| **Rappels** | Rappel avant Ã©vÃ©nement | DonnÃ©es pour Miyunotify / services ; AGD-UI-05 (prochaine entrÃ©e) ; phase 3 pour rappels automatiques. |
| **Partage** | Libre/occupÃ©, lecture, Ã©criture, gestion | Mandats, niveaux (WorrySentinel) ; option libre/occupÃ© (phase 2, AGD-UI-08 ou API). |
| **Calendriers multiples** | Plusieurs calendriers, couleurs, masquer/afficher | AgrÃ©gation multi-sources (JayRDV, JayFestival) ; filtre par source/type (AGD-UI-04). |
| **Invitations** | Inviter, accepter, refuser | Service consommateur ; JayKoa reÃ§oit les entrÃ©es validÃ©es. |
| **TÃ¢ches** | TÃ¢ches (to-do) | EntrÃ©es avec plage ou Ã©chÃ©ance si publiÃ©es ; tÃ¢ches sans date hors pÃ©rimÃ¨tre cÅ“ur. |
| **Export / sync** | iCal, sync externe, hors ligne | Export iCal/PDF (AGD-SEC-3) ; sync externe / hors ligne en phase 2+ ou par consommateurs. |

---

## 9. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayKoa - Document Fondateur](../JayKoa%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement. |
| [JayKoa - Ecrans et UI](../JayKoa%20-%20Ecrans%20et%20UI.md) | Composants UI (AGD-UI-01 Ã  06 ; options 07, 08). |
| [JayKoa - Parcours Utilisateurs](../JayKoa%20-%20Parcours%20Utilisateurs.md) | Parcours utilisateurs et cÃ´tÃ© service. |
| [JayKoa - Bornage Implementation](../JayKoa%20-%20Bornage%20Implementation.md) | MVP, phases, hors scope. |
| [Miyukini Prompt Protocol - Ecriture Documentation Conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole dâ€™Ã©criture de la documentation conceptuelle. |

---

**Document** : JayKoa â€” RÃ©fÃ©rentiel fonctionnel inspirÃ© de Google Agenda  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de rÃ©fÃ©rence (enrichissement conceptuel)

