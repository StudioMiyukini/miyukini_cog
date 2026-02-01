# Miyukini Agenda — Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du **service Miyukini Agenda**. Miyukini Agenda est un service de **plateforme** : il n’a pas de « public » final direct comme JayRDV ou MFS — ses **consommateurs** sont les **services métier** (JayRDV, Miyukini Festival Service, futurs services) et, via eux, les utilisateurs finaux. Ce document précise quels Opérateurs exposent les capacités agenda, quels Kits d’outils agrègent les outils sous-jacents, et comment les services consommateurs s’y couplent.

## Portée / Scope

- **Périmètre** : Identification des Opérateurs et Toolkits Miyukini Agenda ; consommation par JayRDV, MFS ; Contrat d’équipe si applicable.
- **Hors périmètre** : Spécifications API détaillées (référencées dans les contrats d’implémentation) ; définition détaillée des Cores (Glossaire Miyukini).

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |
| **Équipe d’Opérateurs** | Collectif gouverné d’Opérateurs qui collaborent sous règles explicites pour délivrer un Service. |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |

Les **services consommateurs** (JayRDV, MFS) **appellent** les Opérateurs et Kits Miyukini Agenda ; ils ne les « utilisent » pas comme un utilisateur final — ils **intègrent** Miyukini Agenda dans leurs propres Opérateurs (ex. JayRDV Pro, MFS Exposant) qui, eux, exposent les écrans aux utilisateurs.

---

## 2. Besoins en Opérateurs (Miyukini Agenda)

Miyukini Agenda peut être exposé par **un ou plusieurs Opérateurs** selon l’architecture retenue. Deux options courantes :

- **Option A — Un seul Opérateur « Miyukini Agenda »** : agrège toutes les capacités (entrées, conflits, vue, export, événements publics) ; les services consommateurs appellent cet Opérateur pour toutes les opérations.
- **Option B — Équipe d’Opérateurs** : plusieurs Opérateurs spécialisés (ex. « Miyukini Agenda Entrées », « Miyukini Agenda Conflits », « Miyukini Agenda Vue & Export », « Miyukini Agenda Événements publics ») collaborant sous un **Contrat d’équipe** ; les services consommateurs appellent l’Opérateur ou l’Équipe selon le flux.

Le document ci-dessous décrit les **capacités** nécessaires ; l’architecture (un Opérateur vs Équipe) est à trancher en conception.

### 2.1 Capacités exposées par Miyukini Agenda (regroupement logique)

| Regroupement | Rôle | Consommé par |
|--------------|------|--------------|
| **Entrées** | Publication, mise à jour, suppression d’entrées agenda (plage, type, nature, id opaque, référence utilisateur) ; enregistrement des références côté Miyukini Agenda. | JayRDV, MFS (après validation métier : création RDV, candidature, inscription atelier). |
| **Conflits** | Vérification de conflit pour un utilisateur et une plage ; retour conflit oui/non + liste des entrées en conflit (plage, type, libellé court) ; règles par type (ex. présence physique : pas de blocage, notification). | JayRDV, MFS (avant ou lors de la validation d’une réservation, candidature, inscription). |
| **Vue & Export** | Fourniture des entrées agenda pour un utilisateur sur une période, avec **filtres** (source, type, période, statut, visibilité) ; export iCal/PDF (entrées éligibles, pas de données au-delà du niveau autorisé). | JayRDV, MFS (affichage « Mon agenda », calendrier, liste ; téléchargement iCal/PDF). |
| **Événements publics** | Fourniture d’une **liste d’événements publics** éligibles (plage, type, source, libellé court, id opaque) — sans données personnelles — pour affichage « catalogue » ou « à ajouter à mon agenda » ; après **sélection et validation** par l’utilisateur, le service consommateur enregistre l’inscription/réservation puis **publie** l’entrée vers Miyukini Agenda (flux Entrées). | MFS (catalogue festivals/ateliers ouverts), JayRDV (créneaux publics), futurs services. |

### 2.2 Synthèse Opérateurs

| Opérateur (ou capacité) | Usage par le service consommateur | Livrables couverts |
|-------------------------|-----------------------------------|---------------------|
| **Miyukini Agenda Entrées** (ou capacité Entrées) | Publier / mettre à jour / supprimer les entrées agenda après validation métier. | Références agenda enregistrées ; index pour conflits et vues. |
| **Miyukini Agenda Conflits** (ou capacité Conflits) | Interroger avant ou lors d’une action : conflit pour utilisateur U et plage P ? | Alerte conflit (AGD-UI-02) ; indicateur conflit non résolu (AGD-UI-06). |
| **Miyukini Agenda Vue & Export** (ou capacité Vue & Export) | Interroger entrées pour utilisateur U, période, filtres ; générer export iCal/PDF. | Vue calendrier (AGD-UI-01), liste (AGD-UI-07), export (AGD-UI-03), filtres (AGD-UI-04 et filtres détaillés). |
| **Miyukini Agenda Événements publics** (ou capacité Événements publics) | Interroger la liste des événements publics éligibles (sans données personnelles) ; après sélection utilisateur, le consommateur gère l’inscription/réservation puis publie l’entrée (flux Entrées). | Vue événements publics (AGD-UI-09), Sélection / Ajouter à mon agenda (AGD-UI-10). |

*Note :* Si l’architecture retient une **Équipe d’Opérateurs**, un **Contrat d’équipe** définit les flux autorisés entre ces Opérateurs et les règles validées par StrongFather. Les services consommateurs obtiennent un **Mandat de Permission** pour appeler l’Équipe (ou l’Opérateur unique) Miyukini Agenda.

---

## 3. Besoins en Toolkits (Miyukini Agenda)

Les **Toolkits** sont des compositions d’**Outils** (capacités exécutables sans autorité). Ils sont **consommés** par les Opérateurs Miyukini Agenda (ou par l’Équipe) et **exposés** aux services consommateurs via ces Opérateurs.

### 3.1 Kit « Entrées Agenda »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **publication**, la **mise à jour** et la **suppression** des entrées agenda (références : plage, type, nature, id opaque, référence utilisateur, source). |
| **Outils agrégés (exemples)** | `agenda.entry.publish` (publier une entrée), `agenda.entry.update` (mettre à jour une plage ou un type), `agenda.entry.remove` (supprimer une référence), `agenda.entry.get` (détail d’une entrée par id opaque). |
| **Consommé par** | Opérateur(s) Miyukini Agenda Entrées ; appelé par JayRDV, MFS après validation métier (création RDV, candidature validée, inscription atelier). |
| **Composants sous-jacents** | KindMother (persistance des références), MiyuClock (fuseaux, pas de logique métier). |

### 3.2 Kit « Conflits »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **vérification de conflit** (chevauchement) pour un utilisateur et une plage ; retourner conflit oui/non + liste des entrées en conflit ; appliquer les règles par type (ex. présence physique : pas de blocage). |
| **Outils agrégés (exemples)** | `agenda.conflict.check` (vérifier conflit pour utilisateur U, plage P, type T), `agenda.conflict.list` (liste des entrées en conflit pour un utilisateur, pour affichage indicateur persistant AGD-UI-06). |
| **Consommé par** | Opérateur(s) Miyukini Agenda Conflits ; appelé par JayRDV, MFS (formulaires candidature, réservation, inscription). |
| **Composants sous-jacents** | KindMother (lecture des références), MiyuClock (comparaison de plages). |

### 3.3 Kit « Vue & Export »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **fourniture des entrées** pour un utilisateur sur une période, avec **filtres** (source, type, période, statut, visibilité) ; génération **export iCal/PDF** (entrées éligibles, respect AGD-SEC-3). |
| **Outils agrégés (exemples)** | `agenda.entries.list` (liste des entrées pour utilisateur U, période P, filtres F), `agenda.export.ical` (générer iCal pour U, période P, filtres F), `agenda.export.pdf` (phase 2). |
| **Consommé par** | Opérateur(s) Miyukini Agenda Vue & Export ; appelé par JayRDV, MFS (pages « Mon agenda », calendrier, export). |
| **Composants sous-jacents** | KindMother (lecture des références), MiyuClock (fuseaux, période), WorrySentinel (niveau de sécurité pour export). |

### 3.4 Kit « Événements publics »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Fournir une **liste d’événements publics** éligibles (plage, type, source, libellé court, id opaque) — sans données personnelles — pour affichage catalogue « à ajouter à mon agenda ». Les données peuvent être **publiées** par les services consommateurs (MFS, JayRDV) comme « événement public » (ex. édition festival ouverte aux candidatures, atelier ouvert aux inscriptions) ; Miyukini Agenda les agrège et les expose selon Mandat et visibilité. |
| **Outils agrégés (exemples)** | `agenda.public.list` (liste des événements publics pour période P, filtres F — source, type), `agenda.public.get` (détail public d’un événement par id opaque). |
| **Consommé par** | Opérateur(s) Miyukini Agenda Événements publics ; appelé par MFS (catalogue festivals/ateliers), JayRDV (créneaux publics), pour afficher la vue AGD-UI-09. La **sélection** et l’**ajout à mon agenda** (AGD-UI-10) déclenchent le flux métier du consommateur (inscription, réservation) puis la **publication** vers Miyukini Agenda (Kit Entrées). |
| **Composants sous-jacents** | KindMother (références des événements publics), MiyuClock (période). |

---

## 4. Filtres supportés par Miyukini Agenda

Les **filtres** sont des paramètres d’interrogation passés par les services consommateurs à Miyukini Agenda (Kit Vue & Export, Kit Événements publics). Liste des filtres à supporter :

| Filtre | Description | Utilisé par |
|--------|-------------|-------------|
| **Période** | Date début, date fin (ou plage) pour restreindre les entrées retournées. | Vue calendrier, liste, export. |
| **Source** | Filtrer par service d’origine (JayRDV, MFS, etc.). | AGD-UI-04, vue agrégée. |
| **Type** | Filtrer par type d’entrée (RDV, édition, atelier, participation, etc.). | AGD-UI-04. |
| **Statut** | Filtrer par statut métier si exposé (ex. candidat, inscrit, confirmé) — selon ce que le consommateur publie. | Vue liste, filtres détaillés. |
| **Visibilité** | Public vs privé : n’afficher que les entrées marquées « public » (ex. pour vue événements publics) ou « mes entrées » (privé). | Vue événements publics (AGD-UI-09), vue « Mon agenda ». |
| **Nature** | Filtrer par nature (ex. présence physique) pour règles de conflit ou affichage. | Conflits, vues. |

Les services consommateurs **passent** ces filtres lors des appels à Miyukini Agenda ; Miyukini Agenda **applique** les règles de visibilité (Mandat, WorrySentinel) et retourne uniquement les entrées éligibles.

---

## 5. Contrat d’équipe (si Équipe d’Opérateurs)

Si Miyukini Agenda est exposé par une **Équipe d’Opérateurs** (plusieurs Opérateurs spécialisés) :

- **Contrat d’équipe** : définit les Opérateurs membres (Entrées, Conflits, Vue & Export, Événements publics), les **flux autorisés** (qui peut appeler qui), la **direction des flux** (services consommateurs → Opérateurs Miyukini Agenda ; pas d’appel Miyukini Agenda → JayRDV/MFS sauf callback si prévu), les **types d’échanges** et les **conditions préalables** (Mandat valide).
- **Validation** : StrongFather valide le contrat ; les services consommateurs obtiennent un **Mandat de Permission** pour appeler l’Équipe Miyukini Agenda (ou un point d’entrée unique qui délègue aux Opérateurs internes).

Si Miyukini Agenda est exposé par **un seul Opérateur**, pas de Contrat d’équipe interne ; le Mandat autorise l’appel à cet Opérateur.

---

## 6. Références

| Document | Rôle |
|----------|------|
| [Miyukini Agenda - Document Fondateur](./Miyukini%20Agenda%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement. |
| [Miyukini Agenda - Integration Services Consommateurs](./reference/Miyukini%20Agenda%20-%20Integration%20Services%20Consommateurs.md) | Séquence d’intégration, contrat conceptuel, filtres. |
| [Miyukini Agenda - Ecrans et UI](./Miyukini%20Agenda%20-%20Ecrans%20et%20UI.md) | Composants UI (AGD-UI-01 à 10), filtres détaillés. |
| [Miyukini Agenda - Audit Documentation et Manques](./Miyukini%20Agenda%20-%20Audit%20Documentation%20et%20Manques.md) | Audit et manques pour service complet. |

---

**Document** : Miyukini Agenda — Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (Opérateurs, Toolkits, Équipe)
