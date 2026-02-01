# Miyukini Agenda — Bornage pour l’implémentation

## Contexte

Ce document définit le **bornage** (périmètre, limites, priorités) pour l’**implémentation** du service Miyukini Agenda : ce qui est **in scope** et **hors scope** par phase, les **dépendances** techniques et fonctionnelles, et les **critères de livraison** pour une première version (MVP) et les phases suivantes.

## Portée / Scope

- **Périmètre** : Périmètre d’implémentation (MVP, phase 2, etc.) ; dépendances (MiyuClock, Miyubooking, KindMother, WorrySentinel) ; hors scope explicite ; critères de fin de phase.
- **Hors périmètre** : Spécifications techniques détaillées (API, schémas de données) — référencées dans les contrats d’Opérateurs et Kits.

---

## 1. Périmètre fonctionnel par phase

### 1.1 MVP (Phase 1) — In scope

| Capacité | Description | Priorité |
|----------|-------------|----------|
| **Entrées agenda** | Modélisation des entrées (plage début/fin, type, id opaque, référence utilisateur, source) ; enregistrement des références publiées par les services consommateurs. | Must |
| **Détection de conflits** | Vérification de chevauchement pour un utilisateur et une plage donnée ; retour conflit oui/non + liste des entrées en conflit (plage, type, libellé court). | Must |
| **Vue calendrier (données)** | Fourniture des entrées pour un utilisateur sur une période (jour, semaine, mois) ; filtres par type et par source. Pas d’UI propre : **données** consommées par les UIs des services (JayRDV, MFS). | Must |
| **Export iCal** | Génération d’un fichier iCal contenant les entrées éligibles (Mandat, niveau de sécurité) ; pas de données sensibles ni de noms de tiers au-delà du niveau autorisé. | Must |
| **Gouvernance** | Intégration StrongFather (Mandats), Master Butler (permissions), KindMother (persistance des références), WorrySentinel (niveau de sécurité). | Must |
| **Intégration avec au moins un consommateur** | Au moins un service consommateur (JayRDV ou MFS) publie des entrées et interroge conflits + vue + export. | Must |

### 1.2 MVP (Phase 1) — Hors scope

| Élément | Raison |
|---------|--------|
| **UI propre Miyukini Agenda** | Les écrans sont intégrés dans les UIs des services consommateurs ; pas de portail « Miyukini Agenda » standalone pour l’utilisateur final. |
| **Export PDF** | Reporté en phase 2 ; priorité à iCal pour l’MVP. |
| **Agrégation multi-sources** | Si un seul service consommateur en phase 1, l’agrégation peut être limitée ; l’agrégation multi-sources (JayRDV + MFS) est cible dès que deux consommateurs sont connectés. |
| **Partage de calendrier (lien public)** | Hors scope MVP ; à traiter en phase 2 ou 3 selon besoin. |
| **Synchronisation avec calendriers externes (Google, Outlook)** | Hors scope MVP ; intégration externe à traiter par les services consommateurs ou en phase ultérieure. |

### 1.3 Phase 2 — Extension prévue

| Capacité | Description |
|----------|-------------|
| **Export PDF** | Génération d’un export PDF des entrées agenda (même règles de visibilité que iCal). |
| **Agrégation multi-sources complète** | Vue et export agrégés (JayRDV + MFS + autres) avec filtres par source ; deux consommateurs ou plus connectés. |
| **Composants UI réutilisables** | Livrable de composants (vue calendrier, alerte conflit, bloc export) réutilisables par JayRDV et MFS (design system, contrat clair). |
| **Partage (lien optionnel)** | Lien de partage contrôlé pour un agenda (lecture seule, période limitée) si besoin métier. |

### 1.4 Phase 3 et au-delà — Optionnel

| Capacité | Description |
|----------|-------------|
| **Rappels / notifications** | Alertes « Prochain événement dans X heures » (délégation à Miyunotify ou aux services consommateurs). |
| **Synchronisation calendriers externes** | Lecture/écriture avec Google Calendar, Outlook, Apple Calendar (via services consommateurs ou extension Miyukini Agenda). |
| **Règles de conflit configurables** | Règles métier par service (ex. « bloquer si même jour » vs « bloquer si chevauchement > 1 h »). |

---

## 2. Dépendances techniques et fonctionnelles

### 2.1 Dépendances obligatoires (MVP)

| Dépendance | Rôle |
|------------|------|
| **MiyuClock** | Référence temporelle (trace only) ; fuseaux ; pas de temps global requis (LOI-4). |
| **Miyubooking** | Réservation de créneaux, plages ; peut être utilisé pour les plages RDV ou en complément des entrées « édition / atelier ». Selon architecture : Miyukini Agenda s’appuie sur Miyubooking ou coexiste avec lui pour les types d’entrées non couverts par Miyubooking. |
| **KindMother** | Persistance des **références** agenda (entrées, index pour conflits) ; pas la copie canonique des données métier des services consommateurs. |
| **StrongFather** | Émission des Mandats de Permission pour les services consommateurs et les utilisateurs. |
| **Master Butler** | Permissions (qui peut voir quelles entrées, qui peut exporter). |
| **WorrySentinel** | Niveau de sécurité des données et des flux ; états de confiance (T0–T4) pour restreindre capacités si dégradation. |

### 2.2 Dépendances optionnelles (phases ultérieures)

| Dépendance | Rôle |
|------------|------|
| **Miyunotify** | Notifications « prochain événement », rappels (si intégrés au niveau agenda). |
| **Design system / composants UI** | Pour livrer des composants réutilisables (vue calendrier, alerte, export) en phase 2. |

---

## 3. Interfaces et responsabilités

| Interface | Responsable | Contrat |
|-----------|-------------|---------|
| **Publication d’entrées** | Service consommateur (JayRDV, MFS) | Envoi plage, type, id opaque, référence utilisateur, niveau ; Miyukini Agenda enregistre et indexe. |
| **Interrogation conflit** | Miyukini Agenda | Entrée : utilisateur, plage. Sortie : conflit oui/non + liste entrées en conflit. |
| **Interrogation vue calendrier** | Miyukini Agenda | Entrée : utilisateur, période, filtres (type, source). Sortie : liste d’entrées (plage, type, libellé, source, id opaque). |
| **Export iCal** | Miyukini Agenda (ou service consommateur avec données Miyukini Agenda) | Entrée : utilisateur, période, format. Sortie : fichier iCal (pas de données au-delà du niveau autorisé). |
| **UI (écrans)** | Service consommateur | Les écrans sont hébergés par JayRDV, MFS ; ils appellent Miyukini Agenda pour les données et la logique. |

---

## 4. Critères de fin de phase (MVP)

| Critère | Description |
|---------|-------------|
| **CF-MVP-1** | Les entrées agenda peuvent être publiées par au moins un service consommateur (JayRDV ou MFS) et enregistrées par Miyukini Agenda. |
| **CF-MVP-2** | La détection de conflit fonctionne pour un utilisateur et une plage donnée ; le service consommateur peut afficher une alerte (AGD-UI-02) et bloquer ou laisser confirmer selon règle métier. |
| **CF-MVP-3** | La vue calendrier (données) est disponible : un service consommateur peut récupérer les entrées pour un utilisateur sur une période et les afficher dans son UI. |
| **CF-MVP-4** | L’export iCal est disponible ; le fichier ne contient pas de données au-delà du niveau autorisé (AGD-SEC-3). |
| **CF-MVP-5** | Gouvernance en place : Mandats, permissions, WorrySentinel (niveau de sécurité) appliqués aux flux. |
| **CF-MVP-6** | Documentation : Document fondateur, Écrans et UI, Parcours, Bornage, Niveaux sécurité, Integration consommateurs à jour. |

---

## 5. Hors scope explicite (toutes phases sauf mention)

| Élément | Commentaire |
|---------|-------------|
| **Copie canonique des données métier** | Miyukini Agenda ne détient pas la copie canonique des données personnelles ou métier (RDV détail, candidature détail) ; il travaille sur références et synthèses (AGD-SEC-1). |
| **Décision métier « accepter ou refuser malgré conflit »** | La règle (bloquer vs laisser confirmer) est du ressort du service consommateur, pas de Miyukini Agenda. |
| **Authentification utilisateur** | Gérée par Miyauth et les services consommateurs ; Miyukini Agenda reçoit une référence utilisateur (id opaque ou contexte Mandat). |
| **Envoi d’emails / SMS** | Géré par Miyunotify ou les services consommateurs ; Miyukini Agenda ne envoie pas de notifications directes en MVP. |

---

## 6. Références

| Document | Rôle |
|----------|------|
| [Miyukini Agenda - Document Fondateur](./Miyukini%20Agenda%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement. |
| [Miyukini Agenda - Ecrans et UI](./Miyukini%20Agenda%20-%20Ecrans%20et%20UI.md) | Composants UI à livrer (phase 2 pour composants réutilisables). |
| [Miyukini Agenda - Parcours Utilisateurs](./Miyukini%20Agenda%20-%20Parcours%20Utilisateurs.md) | Parcours couverts par l’implémentation. |
| [Miyukini Agenda - Integration Services Consommateurs](./reference/Miyukini%20Agenda%20-%20Integration%20Services%20Consommateurs.md) | Contrats d’intégration avec JayRDV, MFS. |

---

**Document** : Miyukini Agenda — Bornage pour l’implémentation  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (bornage implémentation)
