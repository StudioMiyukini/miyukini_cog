# MiyuBooking â€” Parcours et intÃ©gration services consommateurs

## Contexte

Ce document dÃ©crit de faÃ§on **exhaustive** les **parcours** types (crÃ©neaux, rÃ©servation, annulation, modification), les **points d'entrÃ©e** et le **contrat d'intÃ©gration** pour les **services consommateurs** (JayFestival, JayRDV, etc.) qui utilisent MiyuBooking. Il permet une exploitation directe du kit dans ses services : flux d'appel, donnÃ©es Ã©changÃ©es, cas d'usage, exemples.

**RÃ©fÃ©rences** : [MiyuBooking - Documentation Fondatrice](../MiyuBooking%20-%20Documentation%20Fondatrice.md), [MiyuBooking - Reference Outils](../MiyuBooking%20-%20Reference%20Outils.md), [MiyuBooking - Tool Governance Compliance Contract](../contracts/governance/MiyuBooking%20-%20Tool%20Governance%20Compliance%20Contract.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Parcours utilisateur (rÃ©server, annuler, modifier), flux OpÃ©rateur â†’ MiyuBooking â†’ KindMother, donnÃ©es (crÃ©neaux, rÃ©servations, ressources, tarifs), points d'entrÃ©e pour JayFestival (billets, rÃ©servations ateliers/visiteur), JayRDV (crÃ©neaux RDV), et tout service consommateur.
- **Hors pÃ©rimÃ¨tre** : ImplÃ©mentation dÃ©taillÃ©e du moteur de crÃ©neaux (fuseaux, rÃ©currence) â€” rÃ©fÃ©rencÃ©e dans les Implementation Guidelines.

---

## 1. Rappel : outils MiyuBooking

| ToolId | Action | Niveau sÃ©curitÃ© | RÃ´le pour le consommateur |
|--------|--------|------------------|----------------------------|
| `tool.booking.slots.list` | Lister les crÃ©neaux disponibles | 0â€“1 | Afficher les crÃ©neaux proposÃ©s (ressource, date, durÃ©e). |
| `tool.booking.slots.resolve` | RÃ©soudre un crÃ©neau par identifiant | 0â€“1 | RÃ©cupÃ©rer le dÃ©tail d'un crÃ©neau (dÃ©but, fin, ressource). |
| `tool.booking.create` | CrÃ©er une rÃ©servation | 1â€“2 | Enregistrer la rÃ©servation (WriteIntent KindMother ; dÃ©cision StrongFather). |
| `tool.booking.update` | Mettre Ã  jour une rÃ©servation | 1â€“2 | DÃ©placer ou prolonger une rÃ©servation. |
| `tool.booking.cancel` | Annuler une rÃ©servation | 1â€“2 | Annuler (dÃ©cision StrongFather ; WriteIntent KindMother). |
| `tool.booking.resource.resolve` | RÃ©soudre une ressource (salle, Ã©quipement) | 0â€“1 | Obtenir contraintes et mÃ©tadonnÃ©es de la ressource. |
| `tool.booking.resource.availability` | DisponibilitÃ© d'une ressource sur une plage | 0â€“1 | VÃ©rifier dispo avant proposition de crÃ©neaux. |
| `tool.booking.price.compute` | Calculer le prix d'une rÃ©servation | 0â€“1 | Afficher prix ou total (rÃ¨gles fournies dans le flux). |
| `tool.booking.participants.compute` | Places restantes / participants pour un crÃ©neau | 0â€“1 | Afficher capacitÃ© restante, Ã©viter surrÃ©servation. |

**Invariants** : Toute Ã©criture = **WriteIntent** KindMother. Toute dÃ©cision (crÃ©er, annuler) = **StrongFather**. Le kit n'accÃ¨de jamais directement Ã  la persistance.

---

## 2. Flux d'appel (gouvernance)

```
[OpÃ©rateur consommateur] (ex. JayFestival Visiteur, JayRDV Client)
        â†“ intention (rÃ©server, annuler, lister crÃ©neaux)
[BondingBrother] â€” traduction intention â†’ requÃªte gouvernÃ©e
        â†“
[StrongFather] â€” dÃ©cision ALLOW/DENY (crÃ©ation, annulation)
        â†“ si ALLOW
[Master Butler] â€” vÃ©rification permission, rÃ©solution ToolIds
        â†“
[MiyuBooking] â€” exÃ©cution Tools (slots.list, create, cancel, â€¦)
        â†“ si Ã©criture
[KindMother] â€” WriteIntent (rÃ©servation crÃ©Ã©e/mise Ã  jour/annulÃ©e)
        â†“
[RÃ©ponse] â€” succÃ¨s ou erreur (sans exposer donnÃ©es sensibles)
```

Le **service consommateur** (JayFestival, JayRDV) ne appelle **pas** MiyuBooking directement : il passe par **BondingBrother** avec une intention (ex. Â« rÃ©server le crÃ©neau X pour l'utilisateur Y Â»). StrongFather dÃ©cide ; MiyuBooking exÃ©cute.

---

## 3. DonnÃ©es et contrat KindMother

### 3.1 DonnÃ©es sous autoritÃ© KindMother

| EntitÃ© | Contenu | RÃ´le |
|--------|---------|------|
| **RÃ¨gles de crÃ©neaux** | Grille horaire, durÃ©e min/max, rÃ©currence (si applicable), exclusions | DÃ©finissent les crÃ©neaux proposÃ©s par `slots.list`. |
| **Ressources** | Salles, Ã©quipements, Â« slots Â» logiques (ex. atelier, stand) ; contraintes, capacitÃ© | `resource.resolve`, `resource.availability` ; utilisÃ©es par `slots.list` et `create`. |
| **RÃ©servations** | Identifiant, ressource/crÃ©neau, participant(s), statut (confirmÃ©e, annulÃ©e), horodatage | CrÃ©Ã©es/mises Ã  jour/annulÃ©es via WriteIntent (Tools `create`, `update`, `cancel`). |
| **Tarifs** | RÃ¨gles de prix (par crÃ©neau, par ressource, par participant) | Fournies dans le flux ou par KindMother pour `price.compute`. |

Le **service consommateur** ne dÃ©tient pas la source de vÃ©ritÃ© : il envoie des **intentions** (rÃ©server, annuler) et reÃ§oit des **rÃ©sultats** (succÃ¨s, identifiant rÃ©servation, ou erreur). Les donnÃ©es rÃ©sident dans KindMother (ou en alpha dans une table Â« rÃ©servations Â» exposÃ©e via le mÃªme contrat).

### 3.2 Contrat d'intÃ©gration (rÃ©sumÃ©)

| Besoin consommateur | Tool(s) MiyuBooking | DonnÃ©es en entrÃ©e (flux) | DonnÃ©es en sortie |
|---------------------|---------------------|---------------------------|---------------------|
| Afficher crÃ©neaux disponibles | `slots.list` | Contexte : ressource_id (ou Ã©quivalent), date (ou plage), durÃ©e souhaitÃ©e | Liste de crÃ©neaux (id, dÃ©but, fin, ressource) |
| DÃ©tail d'un crÃ©neau | `slots.resolve` | slot_id | CrÃ©neau (dÃ©but, fin, ressource, mÃ©tadonnÃ©es) |
| RÃ©server | `create` | DonnÃ©es rÃ©servation : crÃ©neau/slot, participant(s), mÃ©tadonnÃ©es optionnelles | SuccÃ¨s + reservation_id ou erreur |
| Modifier rÃ©servation | `update` | reservation_id, nouvelles donnÃ©es (crÃ©neau, durÃ©e, etc.) | SuccÃ¨s ou erreur |
| Annuler rÃ©servation | `cancel` | reservation_id | SuccÃ¨s ou erreur |
| Infos ressource | `resource.resolve` | resource_id | Contraintes, capacitÃ©, mÃ©tadonnÃ©es |
| DisponibilitÃ© ressource | `resource.availability` | resource_id, plage (dÃ©but, fin) | Indicateur dispo ou crÃ©neaux dÃ©jÃ  pris |
| Prix | `price.compute` | RÃ©servation (crÃ©neau, ressource, participants) + rÃ¨gles (fournies) | Montant (ou dÃ©tail lignes) |
| Places restantes | `participants.compute` | slot_id (ou crÃ©neau) | Nombre places restantes / participants inscrits |

---

## 4. Parcours types (exploitables par les services)

### 4.1 Parcours Â« Choisir un crÃ©neau et rÃ©server Â»

| Ã‰tape | Acteur | Action | MiyuBooking / gouvernance |
|-------|--------|--------|----------------------------|
| 1 | Utilisateur | Choisit ressource (ex. atelier, crÃ©neau RDV) et plage (date, durÃ©e) | â€” |
| 2 | Service consommateur | Demande crÃ©neaux disponibles | Intention â†’ BondingBrother â†’ `slots.list` (contexte ressource, date, durÃ©e) |
| 3 | MiyuBooking | Retourne liste crÃ©neaux | Sortie `slots.list` |
| 4 | Utilisateur | SÃ©lectionne un crÃ©neau | â€” |
| 5 | Service consommateur | (Optionnel) Demande prix | `price.compute` (crÃ©neau, rÃ¨gles) |
| 6 | Utilisateur | Confirme rÃ©servation | â€” |
| 7 | Service consommateur | Envoie intention Â« crÃ©er rÃ©servation Â» | BondingBrother â†’ StrongFather (ALLOW) â†’ `create` â†’ WriteIntent KindMother |
| 8 | SystÃ¨me | Confirmation | reservation_id, succÃ¨s |

### 4.2 Parcours Â« Annuler une rÃ©servation Â»

| Ã‰tape | Acteur | Action | MiyuBooking / gouvernance |
|-------|--------|--------|----------------------------|
| 1 | Utilisateur | Demande annulation | â€” |
| 2 | Service consommateur | Envoie intention Â« annuler rÃ©servation Â» | BondingBrother â†’ StrongFather (ALLOW) â†’ `cancel` â†’ WriteIntent KindMother |
| 3 | SystÃ¨me | Confirmation annulation | SuccÃ¨s ou erreur |

### 4.3 Parcours Â« Modifier une rÃ©servation Â» (dÃ©placer ou prolonger)

| Ã‰tape | Acteur | Action | MiyuBooking / gouvernance |
|-------|--------|--------|----------------------------|
| 1 | Utilisateur | Demande modification (nouveau crÃ©neau ou durÃ©e) | â€” |
| 2 | Service consommateur | VÃ©rifie crÃ©neaux disponibles (si dÃ©placement) | `slots.list` |
| 3 | Utilisateur | Choisit nouveau crÃ©neau | â€” |
| 4 | Service consommateur | Envoie intention Â« mettre Ã  jour rÃ©servation Â» | BondingBrother â†’ StrongFather â†’ `update` â†’ WriteIntent KindMother |
| 5 | SystÃ¨me | Confirmation | SuccÃ¨s ou erreur |

---

## 5. Points d'entrÃ©e par service consommateur

### 5.1 JayFestival (visiteur : billets, rÃ©servations ateliers)

| Cas d'usage JayFestival | Tools MiyuBooking utilisÃ©s | DonnÃ©es cÃ´tÃ© JayFestival |
|-------------------------|----------------------------|---------------------------|
| **Billets / pass Ã©vÃ©nement** | `slots.list` (crÃ©neaux d'accÃ¨s si applicable), `create` (rÃ©servation billet/pass), `cancel` | edition_id, visiteur_id, type billet/pass ; liaison Ã©dition â†” ressources (KindMother ou tables alpha). |
| **RÃ©servation atelier** | `slots.list`, `slots.resolve`, `resource.availability`, `participants.compute`, `create`, `cancel`, `update` | edition_id, atelier_id (ressource), visiteur_id ; crÃ©neaux ateliers = ressources avec rÃ¨gles de crÃ©neaux. |
| **RÃ©servation crÃ©neau visiteur** (si applicable) | Idem | MÃªme logique ; ressource = Â« crÃ©neau visiteur Â» pour une Ã©dition. |

**Flux typique** : L'OpÃ©rateur JayFestival Visiteur envoie les intentions (lister crÃ©neaux atelier X pour la date Y, rÃ©server le crÃ©neau Z) via BondingBrother ; StrongFather valide ; MiyuBooking exÃ©cute ; KindMother persiste. En **alpha**, les rÃ©servations peuvent Ãªtre stockÃ©es dans des tables Supabase (atelier_reservations, billets, etc.) en respectant le mÃªme contrat logique (WriteIntent Ã©quivalent).

### 5.2 JayRDV (crÃ©neaux RDV, rendez-vous)

| Cas d'usage JayRDV | Tools MiyuBooking utilisÃ©s | DonnÃ©es cÃ´tÃ© JayRDV |
|--------------------|----------------------------|----------------------|
| **CrÃ©neaux disponibles** | `slots.list`, `slots.resolve` | professionnel_id, lieu/prestation (ressource), date, durÃ©e ; rÃ¨gles de crÃ©neaux = grille du pro. |
| **RÃ©server un RDV** | `create` | client_id, professionnel_id, crÃ©neau, prestation, mÃ©tadonnÃ©es. |
| **Annuler / modifier RDV** | `cancel`, `update` | reservation_id. |
| **Prix RDV** (si applicable) | `price.compute` | RÃ¨gles tarifaires fournies dans le flux ou par KindMother. |

**Flux typique** : Le client (ou le pro) consulte les crÃ©neaux via l'OpÃ©rateur JayRDV ; l'intention Â« rÃ©server Â» est envoyÃ©e ; MiyuBooking exÃ©cute `create` ; la rÃ©servation est persistÃ©e (KindMother ou table alpha `appointments` / `reservations`).

### 5.3 Autres services consommateurs

Tout **service** qui a besoin de **crÃ©neaux**, **rÃ©servations**, **ressources** ou **tarification** peut s'appuyer sur le mÃªme contrat : intention â†’ BondingBrother â†’ StrongFather â†’ MiyuBooking â†’ KindMother. Les **points d'entrÃ©e** sont les ToolIds listÃ©s en Â§ 1 ; les **donnÃ©es** sont dÃ©finies par le contexte mÃ©tier du service (ex. Ã©dition, atelier, professionnel, prestation) et mappÃ©es vers ressource_id, slot_id, reservation_id.

---

## 6. Alpha / prÃ©-COG : persistance Ã©quivalente

En **alpha** (Supabase ou autre backend prÃ©-COG), la persistance des rÃ©servations peut Ãªtre rÃ©alisÃ©e par des **tables locales** (ex. `atelier_reservations`, `billets`, `appointments`) et des **services** qui reproduisent le comportement attendu (crÃ©ation, annulation, liste crÃ©neaux) sans appeler encore KindMother. Le **contrat logique** reste le mÃªme : le service consommateur envoie des intentions ; une couche Â« rÃ©servation Â» exÃ©cute lâ€™Ã©quivalent des Tools (liste crÃ©neaux, crÃ©er, annuler) et Ã©crit en base. Ã€ la migration COG-native, cette couche est remplacÃ©e par lâ€™appel rÃ©el Ã  MiyuBooking + WriteIntent KindMother.

---

## 7. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [MiyuBooking - Documentation Fondatrice](../MiyuBooking%20-%20Documentation%20Fondatrice.md) | IdentitÃ©, ToolkitId, liste Tools, gouvernance. |
| [MiyuBooking - Reference Outils](../MiyuBooking%20-%20Reference%20Outils.md) | DÃ©tail chaque ToolId, niveau sÃ©curitÃ©. |
| [MiyuBooking - Tool Governance Compliance Contract](../contracts/governance/MiyuBooking%20-%20Tool%20Governance%20Compliance%20Contract.md) | Obligations (StrongFather, WriteIntent KindMother). |
| [MiyuBooking - Reference Implementation Guidelines](../implementation/MiyuBooking%20-%20Reference%20Implementation%20Guidelines.md) | Lignes directrices implÃ©mentation. |
| [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md) | SchÃ©ma flux gouvernance. |

---

**Document** : MiyuBooking â€” Parcours et intÃ©gration services consommateurs  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de rÃ©fÃ©rence â€” exploitation directe dans les services

