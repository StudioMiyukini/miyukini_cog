# JayKoa — Intégration avec les services consommateurs

## Contexte

Ce document décrit les **schémas d’intégration** entre **JayKoa** et les **services consommateurs** (JayRDV et futurs services ; JayXpose/JayFestival retirés le 2026-04-29). **JayKoa intègre tout ce qui manipule des dates** (JayRDV, futurs services). Il précise qui publie quelles entrées agenda, qui interroge quoi, et comment la gouvernance (Mandats, niveaux de sécurité) s’applique.

## Portée / Scope

- **Périmètre** : Modèle d’intégration JayKoa ↔ JayRDV, futurs services ; types d’entrées ; flux et responsabilités.
- **Hors périmètre** : Spécifications API détaillées (référencées dans les contrats d’Opérateurs et Kits).

---

## 1. Principe d’intégration

- **JayKoa** expose des **Opérateurs** et **Kits d’outils** (entrées, conflits, vue, export).
- Chaque **service consommateur** :
  - **détient** les données métier (qui a quel RDV, quelle candidature, etc.) ;
  - **publie** vers JayKoa les **entrées agenda** nécessaires (plage, type, **nature** — ex. présence physique —, identifiant opaque, niveau de sécurité) ;
  - **interroge** JayKoa pour conflits, vues agrégées, export, selon Mandat et permissions.
- **Types d’événements** : JayKoa distingue notamment les événements de type **présence physique** (ne doivent pas se chevaucher ; si l’utilisateur force, pas de blocage mais notification et indicateurs UI — alertes, rouge clignotant — jusqu’à résolution). Les services consommateurs **déclarent la nature** des entrées (présence physique ou autre) lors de la publication pour que JayKoa applique la règle de conflit adéquate.
- **KindMother** : la résidence des données sensibles reste définie par le **contrat de chaque service** (JayRDV) et la [Politique de résidence des données sensibles](..//..//..//miyukini-webway-system//reference//_index.md). JayKoa peut détenir des **références** ou des **synthèses** sans être la seule copie des données personnelles ou métier.

---

## 2. JayRDV

### 2.1 Usage de JayKoa

| Capacité | Description |
|----------|-------------|
| **Créneaux et RDV** | Publication des plages (RDV, créneaux réservés, exceptions) ; type d’entrée « RDV » ou « créneau ». |
| **Conflits** | Vérification de conflit (double réservation, chevauchement) avant validation d’un RDV. |
| **Vue calendrier** | Vue pro (RDV du jour, semaine, mois) ; vue client (« Mes RDV ») ; agrégation possible avec d’autres sources si Mandat. |
| **Export** | Export iCal/PDF pour le professionnel ou le client ; pas d’exposition des données d’autres clients. |

### 2.2 Types d’entrées agenda (JayRDV)

| Type d’entrée | Données publiées vers JayKoa | Niveau WorrySentinel typique |
|---------------|----------------------------------------|------------------------------|
| **RDV** | Plage (début, fin), fuseau, id opaque, référence utilisateur (pro, client), **nature** (ex. présence physique), pas de nom ni détail en clair dans JayKoa | 1–2 selon contexte |
| **Créneau réservé** | Plage, id opaque, référence ressource (pro, praticien) | 0–1 |
| **Exception** | Plage (congés, absence), id opaque, référence pro | 1 |

### 2.3 Responsabilités

| Acteur | Responsabilité |
|--------|----------------|
| **JayRDV** | Détient les données métier (qui, quoi, où) ; publie les entrées agenda (plage, type, id) ; interroge JayKoa pour conflits et vues. |
| **JayKoa** | Stocke les références et synthèses ; calcule les conflits ; fournit vues agrégées et export selon Mandat et niveau de sécurité. |

---

## 3. Agrégation multi-services

Lorsqu’un **même utilisateur** a des entrées agenda issues de **plusieurs services** :

- **JayKoa** peut fournir une **vue agrégée** (calendrier unifié) et une **détection de conflits cross-service** **si et seulement si** :
  - les deux services ont publié les entrées concernées vers JayKoa ;
  - le **Mandat de Permission** ou le **Mandat public d’accès** autorise l’agrégation pour cet utilisateur ;
  - le **niveau de sécurité** du contexte (WorrySentinel) est respecté pour chaque entrée.

Les **services consommateurs** restent responsables du **niveau** des données qu’ils publient ; JayKoa applique les **règles de visibilité** (pas d’affichage ni d’export au-delà du niveau autorisé).

---

## 4. Futurs services

Tout **nouveau service** qui gère des plages temporelles, des réservations ou des événements (formations, interventions, maintenance, etc.) peut **s’intégrer** à JayKoa en :

- **Déclarant** les types d’entrées qu’il publie (plage, type, niveau WorrySentinel).
- **Publient** les entrées agenda selon le contrat JayKoa.
- **Interrogeant** JayKoa pour conflits, vues agrégées, export, selon Mandat.

La liste des types d’entrées et des niveaux est **extensible** ; les règles de conflit et de visibilité restent gérées par JayKoa de manière uniforme.

---

## 5. Séquence d’intégration (qui appelle quoi, dans quel ordre)

Les flux ci-dessous décrivent **étape par étape** comment les services consommateurs interagissent avec JayKoa.

### 5.1 Flux publication d’entrées

| Étape | Acteur | Action |
|-------|--------|--------|
| 1 | Utilisateur | Valide une action métier (création RDV, inscription atelier) dans l’UI du service consommateur (JayRDV). |
| 2 | Service consommateur | Enregistre les données métier (qui, quoi, où) ; **appelle** JayKoa (Opérateur Entrées / Kit Entrées) : **publier** l’entrée agenda (plage, type, nature, id opaque, référence utilisateur, source). |
| 3 | JayKoa | Enregistre la **référence** ; met à jour les index (conflits, vues) ; rend l’entrée disponible pour les requêtes selon Mandat. |
| 4 | Service consommateur | Affiche la confirmation à l’utilisateur ; l’entrée apparaîtra dans « Mon agenda » lors des prochaines interrogations vue. |

### 5.2 Flux vérification conflit (avant ou lors de la validation)

| Étape | Acteur | Action |
|-------|--------|--------|
| 1 | Utilisateur | Remplit un formulaire (candidature, réservation, inscription) avec une date ou une plage. |
| 2 | Service consommateur | **Appelle** JayKoa (Opérateur Conflits / Kit Conflits) : **vérifier conflit** pour utilisateur U, plage P, type T. |
| 3 | JayKoa | Compare la plage avec les entrées existantes de l’utilisateur ; retourne conflit oui/non + liste des entrées en conflit (plage, type, libellé court). |
| 4 | Service consommateur | Affiche l’alerte (AGD-UI-02) si conflit ; l’utilisateur modifie, annule ou confirme. Si confirmation : enregistrement métier puis **flux publication** (6.1). |
| 5 | JayKoa | Si publication : enregistre l’entrée ; pour présence physique en conflit, maintient le statut « conflit non résolu » et fournit les données pour AGD-UI-06. |

### 5.3 Flux vue calendrier / liste / export

| Étape | Acteur | Action |
|-------|--------|--------|
| 1 | Utilisateur | Ouvre la page « Mon agenda », calendrier, liste ou export. |
| 2 | Service consommateur | **Appelle** JayKoa (Opérateur Vue & Export / Kit Vue & Export) : **entrées** pour utilisateur U, **période** P, **filtres** F (source, type, statut, visibilité — voir § 6). |
| 3 | JayKoa | Retourne les entrées éligibles (Mandat, niveau de sécurité, période, filtres). Pour export : génère le fichier iCal/PDF (AGD-SEC-3). |
| 4 | Service consommateur | Affiche la vue calendrier (AGD-UI-01), liste (AGD-UI-07) ou propose le téléchargement du fichier. |

### 5.4 Flux événements publics et ajout à mon agenda

| Étape | Acteur | Action |
|-------|--------|--------|
| 1 | Utilisateur | Ouvre la page « Découvrir », « Événements publics » ou une zone « Ajouter à mon agenda » dans l’UI du service consommateur (JayRDV). |
| 2 | Service consommateur | **Appelle** JayKoa (Opérateur Événements publics / Kit Événements publics) : **liste des événements publics** pour période P, filtres F (source, type). |
| 3 | JayKoa | Retourne la liste des événements publics éligibles (plage, type, source, libellé court, id opaque) — sans données personnelles. |
| 4 | Service consommateur | Affiche la vue événements publics (AGD-UI-09). |
| 5 | Utilisateur | **Sélectionne** un ou plusieurs événements et clique « Ajouter à mon agenda » (AGD-UI-10). |
| 6 | Service consommateur | Déclenche le **flux métier** (inscription, réservation) ; **appelle** JayKoa pour **vérification conflit** (6.2) ; affiche alerte si conflit. |
| 7 | Utilisateur | Confirme ou modifie ; le service consommateur enregistre l’inscription/réservation puis **publie** l’entrée vers JayKoa (flux 6.1). |
| 8 | JayKoa | Enregistre l’entrée ; l’événement apparaît dans « Mon agenda ». |

---

## 6. Contrat conceptuel : paramètres et filtres

Les **paramètres** passés par les services consommateurs à JayKoa et les **filtres** supportés sont formalisés ci-dessous (contrat conceptuel, pas spécification API).

### 6.1 Paramètres d’interrogation (vue, export, événements publics)

| Paramètre | Description | Obligatoire / optionnel |
|-----------|-------------|-------------------------|
| **Utilisateur** | Référence utilisateur (id opaque ou contexte Mandat) pour « mes entrées ». | Obligatoire pour vue/export « Mon agenda ». |
| **Période** | Date début, date fin (ou plage) pour restreindre les entrées. | Obligatoire pour vue/export. |
| **Filtres** | Voir § 6.2. | Optionnel ; si absent, toutes les entrées éligibles (Mandat, niveau de sécurité) sont retournées. |

### 6.2 Filtres supportés par JayKoa

| Filtre | Description | Utilisé dans |
|--------|-------------|--------------|
| **Source** | Service d’origine (JayRDV, etc.). | Vue agrégée, filtre AGD-UI-04. |
| **Type** | Type d’entrée (RDV, édition, atelier, participation, etc.). | Vue calendrier, liste, AGD-UI-04. |
| **Statut** | Statut métier si exposé par le consommateur (ex. candidat, inscrit, confirmé). | Filtres détaillés, vue liste. |
| **Visibilité** | Public vs privé : n’afficher que les entrées « public » (pour catalogue événements publics) ou « mes entrées » (privé). | Vue événements publics (AGD-UI-09), vue « Mon agenda ». |
| **Nature** | Nature de l’événement (ex. présence physique) pour règles de conflit ou affichage. | Conflits, vues. |

Les services consommateurs **passent** ces filtres lors des appels à JayKoa ; JayKoa **applique** les règles de visibilité (Mandat, WorrySentinel) et retourne uniquement les entrées éligibles.

### 6.3 Format de sortie (conceptuel)

- **Liste d’entrées** : pour chaque entrée : plage (début, fin), type, libellé court, source, id opaque, statut/nature si exposé.
- **Conflit** : conflit oui/non ; liste des entrées en conflit (plage, type, libellé court).
- **Export** : fichier iCal ou PDF (entrées éligibles, pas de données au-delà du niveau autorisé).

---

## 7. Références

| Document | Rôle |
|----------|------|
| [JayKoa - Document Fondateur](../JayKoa%20-%20Document%20Fondateur.md) | Contexte, positionnement, intégration synthétique. |
| [JayKoa - Operateurs et Toolkits](../JayKoa%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs, Kits, filtres supportés. |
| [JayRDV - Document Fondateur](../../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (RDV, créneaux). |

---

**Document** : JayKoa — Intégration avec les services consommateurs  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de référence (intégration, séquence, contrat, filtres)

