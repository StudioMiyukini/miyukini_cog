# JayKoa â€” Bornage pour lâ€™implÃ©mentation

## Contexte

Ce document dÃ©finit le **bornage** (pÃ©rimÃ¨tre, limites, prioritÃ©s) pour lâ€™**implÃ©mentation** du service JayKoa : ce qui est **in scope** et **hors scope** par phase, les **dÃ©pendances** techniques et fonctionnelles, et les **critÃ¨res de livraison** pour une premiÃ¨re version (MVP) et les phases suivantes.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : PÃ©rimÃ¨tre dâ€™implÃ©mentation (MVP, phase 2, etc.) ; dÃ©pendances (MiyuClock, Miyubooking, KindMother, WorrySentinel) ; hors scope explicite ; critÃ¨res de fin de phase.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques dÃ©taillÃ©es (API, schÃ©mas de donnÃ©es) â€” rÃ©fÃ©rencÃ©es dans les contrats dâ€™OpÃ©rateurs et Kits.

---

## 1. PÃ©rimÃ¨tre fonctionnel par phase

### 1.1 MVP (Phase 1) â€” In scope

| CapacitÃ© | Description | PrioritÃ© |
|----------|-------------|----------|
| **EntrÃ©es agenda** | ModÃ©lisation des entrÃ©es (plage dÃ©but/fin, type, id opaque, rÃ©fÃ©rence utilisateur, source) ; enregistrement des rÃ©fÃ©rences publiÃ©es par les services consommateurs. | Must |
| **DÃ©tection de conflits** | VÃ©rification de chevauchement pour un utilisateur et une plage donnÃ©e ; retour conflit oui/non + liste des entrÃ©es en conflit (plage, type, libellÃ© court). | Must |
| **Vue calendrier (donnÃ©es)** | Fourniture des entrÃ©es pour un utilisateur sur une pÃ©riode (jour, semaine, mois) ; filtres par type et par source. Pas dâ€™UI propre : **donnÃ©es** consommÃ©es par les UIs des services (JayRDV, JayFestival). | Must |
| **Export iCal** | GÃ©nÃ©ration dâ€™un fichier iCal contenant les entrÃ©es Ã©ligibles (Mandat, niveau de sÃ©curitÃ©) ; pas de donnÃ©es sensibles ni de noms de tiers au-delÃ  du niveau autorisÃ©. | Must |
| **Gouvernance** | IntÃ©gration StrongFather (Mandats), Master Butler (permissions), KindMother (persistance des rÃ©fÃ©rences), WorrySentinel (niveau de sÃ©curitÃ©). | Must |
| **IntÃ©gration avec au moins un consommateur** | Au moins un service consommateur (JayRDV ou JayFestival) publie des entrÃ©es et interroge conflits + vue + export. | Must |

### 1.2 MVP (Phase 1) â€” Hors scope

| Ã‰lÃ©ment | Raison |
|---------|--------|
| **UI propre JayKoa** | Les Ã©crans sont intÃ©grÃ©s dans les UIs des services consommateurs ; pas de portail Â« JayKoa Â» standalone pour lâ€™utilisateur final. |
| **Export PDF** | ReportÃ© en phase 2 ; prioritÃ© Ã  iCal pour lâ€™MVP. |
| **AgrÃ©gation multi-sources** | Si un seul service consommateur en phase 1, lâ€™agrÃ©gation peut Ãªtre limitÃ©e ; lâ€™agrÃ©gation multi-sources (JayRDV + JayFestival) est cible dÃ¨s que deux consommateurs sont connectÃ©s. |
| **Partage de calendrier (lien public)** | Hors scope MVP ; Ã  traiter en phase 2 ou 3 selon besoin. |
| **Synchronisation avec calendriers externes (Google, Outlook)** | Hors scope MVP ; intÃ©gration externe Ã  traiter par les services consommateurs ou en phase ultÃ©rieure. |

### 1.3 Phase 2 â€” Extension prÃ©vue

| CapacitÃ© | Description |
|----------|-------------|
| **Export PDF** | GÃ©nÃ©ration dâ€™un export PDF des entrÃ©es agenda (mÃªme rÃ¨gles de visibilitÃ© que iCal). |
| **AgrÃ©gation multi-sources complÃ¨te** | Vue et export agrÃ©gÃ©s (JayRDV + JayFestival + autres) avec filtres par source ; deux consommateurs ou plus connectÃ©s. |
| **Composants UI rÃ©utilisables** | Livrable de composants (vue calendrier, alerte conflit, bloc export) rÃ©utilisables par JayRDV et JayFestival (design system, contrat clair). |
| **Partage (lien optionnel)** | Lien de partage contrÃ´lÃ© pour un agenda (lecture seule, pÃ©riode limitÃ©e) si besoin mÃ©tier. |

### 1.4 Phase 3 et au-delÃ  â€” Optionnel

| CapacitÃ© | Description |
|----------|-------------|
| **Rappels / notifications** | Alertes Â« Prochain Ã©vÃ©nement dans X heures Â» (dÃ©lÃ©gation Ã  Miyunotify ou aux services consommateurs). |
| **Synchronisation calendriers externes** | Lecture/Ã©criture avec Google Calendar, Outlook, Apple Calendar (via services consommateurs ou extension JayKoa). |
| **RÃ¨gles de conflit configurables** | RÃ¨gles mÃ©tier par service (ex. Â« bloquer si mÃªme jour Â» vs Â« bloquer si chevauchement > 1 h Â»). |

---

## 2. DÃ©pendances techniques et fonctionnelles

### 2.1 DÃ©pendances obligatoires (MVP)

| DÃ©pendance | RÃ´le |
|------------|------|
| **MiyuClock** | RÃ©fÃ©rence temporelle (trace only) ; fuseaux ; pas de temps global requis (LOI-4). |
| **Miyubooking** | RÃ©servation de crÃ©neaux, plages ; peut Ãªtre utilisÃ© pour les plages RDV ou en complÃ©ment des entrÃ©es Â« Ã©dition / atelier Â». Selon architecture : JayKoa sâ€™appuie sur Miyubooking ou coexiste avec lui pour les types dâ€™entrÃ©es non couverts par Miyubooking. |
| **KindMother** | Persistance des **rÃ©fÃ©rences** agenda (entrÃ©es, index pour conflits) ; pas la copie canonique des donnÃ©es mÃ©tier des services consommateurs. |
| **StrongFather** | Ã‰mission des Mandats de Permission pour les services consommateurs et les utilisateurs. |
| **Master Butler** | Permissions (qui peut voir quelles entrÃ©es, qui peut exporter). |
| **WorrySentinel** | Niveau de sÃ©curitÃ© des donnÃ©es et des flux ; Ã©tats de confiance (T0â€“T4) pour restreindre capacitÃ©s si dÃ©gradation. |

### 2.2 DÃ©pendances optionnelles (phases ultÃ©rieures)

| DÃ©pendance | RÃ´le |
|------------|------|
| **Miyunotify** | Notifications Â« prochain Ã©vÃ©nement Â», rappels (si intÃ©grÃ©s au niveau agenda). |
| **Design system / composants UI** | Pour livrer des composants rÃ©utilisables (vue calendrier, alerte, export) en phase 2. |

---

## 3. Interfaces et responsabilitÃ©s

| Interface | Responsable | Contrat |
|-----------|-------------|---------|
| **Publication dâ€™entrÃ©es** | Service consommateur (JayRDV, JayFestival) | Envoi plage, type, id opaque, rÃ©fÃ©rence utilisateur, niveau ; JayKoa enregistre et indexe. |
| **Interrogation conflit** | JayKoa | EntrÃ©e : utilisateur, plage. Sortie : conflit oui/non + liste entrÃ©es en conflit. |
| **Interrogation vue calendrier** | JayKoa | EntrÃ©e : utilisateur, pÃ©riode, filtres (type, source). Sortie : liste dâ€™entrÃ©es (plage, type, libellÃ©, source, id opaque). |
| **Export iCal** | JayKoa (ou service consommateur avec donnÃ©es JayKoa) | EntrÃ©e : utilisateur, pÃ©riode, format. Sortie : fichier iCal (pas de donnÃ©es au-delÃ  du niveau autorisÃ©). |
| **UI (Ã©crans)** | Service consommateur | Les Ã©crans sont hÃ©bergÃ©s par JayRDV, JayFestival ; ils appellent JayKoa pour les donnÃ©es et la logique. |

---

## 4. CritÃ¨res de fin de phase (MVP)

| CritÃ¨re | Description |
|---------|-------------|
| **CF-MVP-1** | Les entrÃ©es agenda peuvent Ãªtre publiÃ©es par au moins un service consommateur (JayRDV ou JayFestival) et enregistrÃ©es par JayKoa. |
| **CF-MVP-2** | La dÃ©tection de conflit fonctionne pour un utilisateur et une plage donnÃ©e ; le service consommateur peut afficher une alerte (AGD-UI-02) et bloquer ou laisser confirmer selon rÃ¨gle mÃ©tier. |
| **CF-MVP-3** | La vue calendrier (donnÃ©es) est disponible : un service consommateur peut rÃ©cupÃ©rer les entrÃ©es pour un utilisateur sur une pÃ©riode et les afficher dans son UI. |
| **CF-MVP-4** | Lâ€™export iCal est disponible ; le fichier ne contient pas de donnÃ©es au-delÃ  du niveau autorisÃ© (AGD-SEC-3). |
| **CF-MVP-5** | Gouvernance en place : Mandats, permissions, WorrySentinel (niveau de sÃ©curitÃ©) appliquÃ©s aux flux. |
| **CF-MVP-6** | Documentation : Document fondateur, Ã‰crans et UI, Parcours, Bornage, Niveaux sÃ©curitÃ©, Integration consommateurs Ã  jour. |

---

## 5. Hors scope explicite (toutes phases sauf mention)

| Ã‰lÃ©ment | Commentaire |
|---------|-------------|
| **Copie canonique des donnÃ©es mÃ©tier** | JayKoa ne dÃ©tient pas la copie canonique des donnÃ©es personnelles ou mÃ©tier (RDV dÃ©tail, candidature dÃ©tail) ; il travaille sur rÃ©fÃ©rences et synthÃ¨ses (AGD-SEC-1). |
| **DÃ©cision mÃ©tier Â« accepter ou refuser malgrÃ© conflit Â»** | La rÃ¨gle (bloquer vs laisser confirmer) est du ressort du service consommateur, pas de JayKoa. |
| **Authentification utilisateur** | GÃ©rÃ©e par Miyauth et les services consommateurs ; JayKoa reÃ§oit une rÃ©fÃ©rence utilisateur (id opaque ou contexte Mandat). |
| **Envoi dâ€™emails / SMS** | GÃ©rÃ© par Miyunotify ou les services consommateurs ; JayKoa ne envoie pas de notifications directes en MVP. |

---

## 6. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayKoa - Document Fondateur](_index.md) | Contexte, besoins, positionnement. |
| [JayKoa - Ecrans et UI](_index.md) | Composants UI Ã  livrer (phase 2 pour composants rÃ©utilisables). |
| [JayKoa - Parcours Utilisateurs](_index.md) | Parcours couverts par lâ€™implÃ©mentation. |
| [JayKoa - Integration Services Consommateurs](reference//_index.md) | Contrats dâ€™intÃ©gration avec JayRDV, JayFestival. |

---

**Document** : JayKoa â€” Bornage pour lâ€™implÃ©mentation  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de rÃ©fÃ©rence (bornage implÃ©mentation)

