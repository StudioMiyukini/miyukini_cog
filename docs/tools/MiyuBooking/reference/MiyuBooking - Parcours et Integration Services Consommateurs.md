# MiyuBooking — Parcours et intégration services consommateurs

## Contexte

Ce document décrit de façon **exhaustive** les **parcours** types (créneaux, réservation, annulation, modification), les **points d'entrée** et le **contrat d'intégration** pour les **services consommateurs** (JayFestival, JayRDV, etc.) qui utilisent MiyuBooking. Il permet une exploitation directe du kit dans ses services : flux d'appel, données échangées, cas d'usage, exemples.

**Références** : [MiyuBooking - Documentation Fondatrice](../MiyuBooking%20-%20Documentation%20Fondatrice.md), [MiyuBooking - Reference Outils](../MiyuBooking%20-%20Reference%20Outils.md), [MiyuBooking - Tool Governance Compliance Contract](../contracts/governance/MiyuBooking%20-%20Tool%20Governance%20Compliance%20Contract.md).

## Portée / Scope

- **Périmètre** : Parcours utilisateur (réserver, annuler, modifier), flux Opérateur → MiyuBooking → KindMother, données (créneaux, réservations, ressources, tarifs), points d'entrée pour JayFestival (billets, réservations ateliers/visiteur), JayRDV (créneaux RDV), et tout service consommateur.
- **Hors périmètre** : Implémentation détaillée du moteur de créneaux (fuseaux, récurrence) — référencée dans les Implementation Guidelines.

---

## 1. Rappel : outils MiyuBooking

| ToolId | Action | Niveau sécurité | Rôle pour le consommateur |
|--------|--------|------------------|----------------------------|
| `tool.booking.slots.list` | Lister les créneaux disponibles | 0–1 | Afficher les créneaux proposés (ressource, date, durée). |
| `tool.booking.slots.resolve` | Résoudre un créneau par identifiant | 0–1 | Récupérer le détail d'un créneau (début, fin, ressource). |
| `tool.booking.create` | Créer une réservation | 1–2 | Enregistrer la réservation (WriteIntent KindMother ; décision StrongFather). |
| `tool.booking.update` | Mettre à jour une réservation | 1–2 | Déplacer ou prolonger une réservation. |
| `tool.booking.cancel` | Annuler une réservation | 1–2 | Annuler (décision StrongFather ; WriteIntent KindMother). |
| `tool.booking.resource.resolve` | Résoudre une ressource (salle, équipement) | 0–1 | Obtenir contraintes et métadonnées de la ressource. |
| `tool.booking.resource.availability` | Disponibilité d'une ressource sur une plage | 0–1 | Vérifier dispo avant proposition de créneaux. |
| `tool.booking.price.compute` | Calculer le prix d'une réservation | 0–1 | Afficher prix ou total (règles fournies dans le flux). |
| `tool.booking.participants.compute` | Places restantes / participants pour un créneau | 0–1 | Afficher capacité restante, éviter surréservation. |

**Invariants** : Toute écriture = **WriteIntent** KindMother. Toute décision (créer, annuler) = **StrongFather**. Le kit n'accède jamais directement à la persistance.

---

## 2. Flux d'appel (gouvernance)

```
[Opérateur consommateur] (ex. JayFestival Visiteur, JayRDV Client)
        ↓ intention (réserver, annuler, lister créneaux)
[BondingBrother] — traduction intention → requête gouvernée
        ↓
[StrongFather] — décision ALLOW/DENY (création, annulation)
        ↓ si ALLOW
[Master Butler] — vérification permission, résolution ToolIds
        ↓
[MiyuBooking] — exécution Tools (slots.list, create, cancel, …)
        ↓ si écriture
[KindMother] — WriteIntent (réservation créée/mise à jour/annulée)
        ↓
[Réponse] — succès ou erreur (sans exposer données sensibles)
```

Le **service consommateur** (JayFestival, JayRDV) ne appelle **pas** MiyuBooking directement : il passe par **BondingBrother** avec une intention (ex. « réserver le créneau X pour l'utilisateur Y »). StrongFather décide ; MiyuBooking exécute.

---

## 3. Données et contrat KindMother

### 3.1 Données sous autorité KindMother

| Entité | Contenu | Rôle |
|--------|---------|------|
| **Règles de créneaux** | Grille horaire, durée min/max, récurrence (si applicable), exclusions | Définissent les créneaux proposés par `slots.list`. |
| **Ressources** | Salles, équipements, « slots » logiques (ex. atelier, stand) ; contraintes, capacité | `resource.resolve`, `resource.availability` ; utilisées par `slots.list` et `create`. |
| **Réservations** | Identifiant, ressource/créneau, participant(s), statut (confirmée, annulée), horodatage | Créées/mises à jour/annulées via WriteIntent (Tools `create`, `update`, `cancel`). |
| **Tarifs** | Règles de prix (par créneau, par ressource, par participant) | Fournies dans le flux ou par KindMother pour `price.compute`. |

Le **service consommateur** ne détient pas la source de vérité : il envoie des **intentions** (réserver, annuler) et reçoit des **résultats** (succès, identifiant réservation, ou erreur). Les données résident dans KindMother (ou en alpha dans une table « réservations » exposée via le même contrat).

### 3.2 Contrat d'intégration (résumé)

| Besoin consommateur | Tool(s) MiyuBooking | Données en entrée (flux) | Données en sortie |
|---------------------|---------------------|---------------------------|---------------------|
| Afficher créneaux disponibles | `slots.list` | Contexte : ressource_id (ou équivalent), date (ou plage), durée souhaitée | Liste de créneaux (id, début, fin, ressource) |
| Détail d'un créneau | `slots.resolve` | slot_id | Créneau (début, fin, ressource, métadonnées) |
| Réserver | `create` | Données réservation : créneau/slot, participant(s), métadonnées optionnelles | Succès + reservation_id ou erreur |
| Modifier réservation | `update` | reservation_id, nouvelles données (créneau, durée, etc.) | Succès ou erreur |
| Annuler réservation | `cancel` | reservation_id | Succès ou erreur |
| Infos ressource | `resource.resolve` | resource_id | Contraintes, capacité, métadonnées |
| Disponibilité ressource | `resource.availability` | resource_id, plage (début, fin) | Indicateur dispo ou créneaux déjà pris |
| Prix | `price.compute` | Réservation (créneau, ressource, participants) + règles (fournies) | Montant (ou détail lignes) |
| Places restantes | `participants.compute` | slot_id (ou créneau) | Nombre places restantes / participants inscrits |

---

## 4. Parcours types (exploitables par les services)

### 4.1 Parcours « Choisir un créneau et réserver »

| Étape | Acteur | Action | MiyuBooking / gouvernance |
|-------|--------|--------|----------------------------|
| 1 | Utilisateur | Choisit ressource (ex. atelier, créneau RDV) et plage (date, durée) | — |
| 2 | Service consommateur | Demande créneaux disponibles | Intention → BondingBrother → `slots.list` (contexte ressource, date, durée) |
| 3 | MiyuBooking | Retourne liste créneaux | Sortie `slots.list` |
| 4 | Utilisateur | Sélectionne un créneau | — |
| 5 | Service consommateur | (Optionnel) Demande prix | `price.compute` (créneau, règles) |
| 6 | Utilisateur | Confirme réservation | — |
| 7 | Service consommateur | Envoie intention « créer réservation » | BondingBrother → StrongFather (ALLOW) → `create` → WriteIntent KindMother |
| 8 | Système | Confirmation | reservation_id, succès |

### 4.2 Parcours « Annuler une réservation »

| Étape | Acteur | Action | MiyuBooking / gouvernance |
|-------|--------|--------|----------------------------|
| 1 | Utilisateur | Demande annulation | — |
| 2 | Service consommateur | Envoie intention « annuler réservation » | BondingBrother → StrongFather (ALLOW) → `cancel` → WriteIntent KindMother |
| 3 | Système | Confirmation annulation | Succès ou erreur |

### 4.3 Parcours « Modifier une réservation » (déplacer ou prolonger)

| Étape | Acteur | Action | MiyuBooking / gouvernance |
|-------|--------|--------|----------------------------|
| 1 | Utilisateur | Demande modification (nouveau créneau ou durée) | — |
| 2 | Service consommateur | Vérifie créneaux disponibles (si déplacement) | `slots.list` |
| 3 | Utilisateur | Choisit nouveau créneau | — |
| 4 | Service consommateur | Envoie intention « mettre à jour réservation » | BondingBrother → StrongFather → `update` → WriteIntent KindMother |
| 5 | Système | Confirmation | Succès ou erreur |

---

## 5. Points d'entrée par service consommateur

### 5.1 JayFestival (visiteur : billets, réservations ateliers)

| Cas d'usage JayFestival | Tools MiyuBooking utilisés | Données côté JayFestival |
|-------------------------|----------------------------|---------------------------|
| **Billets / pass événement** | `slots.list` (créneaux d'accès si applicable), `create` (réservation billet/pass), `cancel` | edition_id, visiteur_id, type billet/pass ; liaison édition ↔ ressources (KindMother ou tables alpha). |
| **Réservation atelier** | `slots.list`, `slots.resolve`, `resource.availability`, `participants.compute`, `create`, `cancel`, `update` | edition_id, atelier_id (ressource), visiteur_id ; créneaux ateliers = ressources avec règles de créneaux. |
| **Réservation créneau visiteur** (si applicable) | Idem | Même logique ; ressource = « créneau visiteur » pour une édition. |

**Flux typique** : L'Opérateur JayFestival Visiteur envoie les intentions (lister créneaux atelier X pour la date Y, réserver le créneau Z) via BondingBrother ; StrongFather valide ; MiyuBooking exécute ; KindMother persiste. En **alpha**, les réservations peuvent être stockées dans des tables Supabase (atelier_reservations, billets, etc.) en respectant le même contrat logique (WriteIntent équivalent).

### 5.2 JayRDV (créneaux RDV, rendez-vous)

| Cas d'usage JayRDV | Tools MiyuBooking utilisés | Données côté JayRDV |
|--------------------|----------------------------|----------------------|
| **Créneaux disponibles** | `slots.list`, `slots.resolve` | professionnel_id, lieu/prestation (ressource), date, durée ; règles de créneaux = grille du pro. |
| **Réserver un RDV** | `create` | client_id, professionnel_id, créneau, prestation, métadonnées. |
| **Annuler / modifier RDV** | `cancel`, `update` | reservation_id. |
| **Prix RDV** (si applicable) | `price.compute` | Règles tarifaires fournies dans le flux ou par KindMother. |

**Flux typique** : Le client (ou le pro) consulte les créneaux via l'Opérateur JayRDV ; l'intention « réserver » est envoyée ; MiyuBooking exécute `create` ; la réservation est persistée (KindMother ou table alpha `appointments` / `reservations`).

### 5.3 Autres services consommateurs

Tout **service** qui a besoin de **créneaux**, **réservations**, **ressources** ou **tarification** peut s'appuyer sur le même contrat : intention → BondingBrother → StrongFather → MiyuBooking → KindMother. Les **points d'entrée** sont les ToolIds listés en § 1 ; les **données** sont définies par le contexte métier du service (ex. édition, atelier, professionnel, prestation) et mappées vers ressource_id, slot_id, reservation_id.

---

## 6. Alpha / pré-COG : persistance équivalente

En **alpha** (Supabase ou autre backend pré-COG), la persistance des réservations peut être réalisée par des **tables locales** (ex. `atelier_reservations`, `billets`, `appointments`) et des **services** qui reproduisent le comportement attendu (création, annulation, liste créneaux) sans appeler encore KindMother. Le **contrat logique** reste le même : le service consommateur envoie des intentions ; une couche « réservation » exécute l’équivalent des Tools (liste créneaux, créer, annuler) et écrit en base. À la migration COG-native, cette couche est remplacée par l’appel réel à MiyuBooking + WriteIntent KindMother.

---

## 7. Références

| Document | Rôle |
|----------|------|
| [MiyuBooking - Documentation Fondatrice](../MiyuBooking%20-%20Documentation%20Fondatrice.md) | Identité, ToolkitId, liste Tools, gouvernance. |
| [MiyuBooking - Reference Outils](../MiyuBooking%20-%20Reference%20Outils.md) | Détail chaque ToolId, niveau sécurité. |
| [MiyuBooking - Tool Governance Compliance Contract](../contracts/governance/MiyuBooking%20-%20Tool%20Governance%20Compliance%20Contract.md) | Obligations (StrongFather, WriteIntent KindMother). |
| [MiyuBooking - Reference Implementation Guidelines](../implementation/MiyuBooking%20-%20Reference%20Implementation%20Guidelines.md) | Lignes directrices implémentation. |
| [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) | Schéma flux gouvernance. |

---

**Document** : MiyuBooking — Parcours et intégration services consommateurs  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de référence — exploitation directe dans les services
