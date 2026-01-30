# MiyuBooking — Documentation Fondatrice

## 1. Contexte

**MiyuBooking** est le **kit d'outils (Toolkit)** de réservation en ligne (créneaux, disponibilités, réservations, ressources, tarification, participants) de l'écosystème Miyukini. Il intègre les outils de liste et résolution de créneaux, de création, mise à jour et annulation de réservations, de résolution et disponibilité des ressources, de calcul de prix et de participants, alignés sur KindMother pour la persistance des données.

L'autorité sur les données (règles de créneaux, ressources, réservations, tarifs) appartient à **KindMother** (Core de données, Strate 4). MiyuBooking expose des capacités d'exécution gouvernée (slots, create, update, cancel, resource, price, participants) sans remplacer KindMother ni StrongFather ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuBooking
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother (persistance créneaux, ressources, réservations)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (moteur de créneaux, fuseaux horaires)
- Toute décision d'autorisation, de confirmation ou d'annulation — celle-ci reste du ressort de StrongFather et des Cores
- Les rappels et notifications (Opérateur Automatisation ou phase ultérieure)

---

## 3. Définition canonique

> **MiyuBooking est une composition officielle d'outils de réservation en ligne (créneaux, réservations, ressources, tarification, participants), déclarée et gouvernée par l'environnement.**

- MiyuBooking **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuBooking **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (lister/résoudre créneaux, créer/mettre à jour/annuler réservation, résoudre/disponibilité ressource, calculer prix/participants) sans décider de l'autorisation ni de la politique de réservation.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Toute persistance (réservations, règles de créneaux, ressources) et toute décision (autoriser, annuler) sont sous autorité KindMother (WriteIntent) et StrongFather (ALLOW/DENY).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.booking.reservations` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `booking` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuBooking est composé des Tools suivants (format canonique `tool.booking.<sous-domaine>.<action>` ou `tool.booking.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) sera décrit dans MiyuBooking - Reference Outils (phase ultérieure).

| ToolId | Description courte |
|--------|---------------------|
| `tool.booking.slots.list` | Liste les créneaux disponibles pour un contexte (ressource, date, durée) fourni |
| `tool.booking.slots.resolve` | Résout un créneau par identifiant |
| `tool.booking.create` | Crée une réservation à partir de données fournies ; WriteIntent KindMother |
| `tool.booking.update` | Met à jour une réservation (déplacement, prolongation) |
| `tool.booking.cancel` | Annule une réservation ; décision politique = StrongFather |
| `tool.booking.resource.resolve` | Résout une ressource (salle, équipement) et ses contraintes |
| `tool.booking.resource.availability` | Retourne la disponibilité d'une ressource sur une plage donnée |
| `tool.booking.price.compute` | Calcule le prix d'une réservation (règles fournies) |
| `tool.booking.participants.compute` | Calcule places restantes / participants pour un créneau |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuBooking en contient neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : décision (création réservation, annulation) = StrongFather ; toute écriture (réservation) = WriteIntent KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** selon opération (lecture créneaux 0–1, création/annulation réservation 1–2) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother

- **KindMother** est l'autorité sur les règles de créneaux, les ressources (salles, équipements), les réservations et les tarifs. MiyuBooking exécute des capacités (slots.list/resolve, create, update, cancel, resource.resolve/availability, price.compute, participants.compute) **sans décider** de l'autorisation (StrongFather) ni de la politique de réservation ; les règles sont fournies par KindMother ou dans le flux.
- Les créneaux peuvent être affichés dans le fuseau horaire de l'utilisateur ; les règles de disponibilité et les ressources sont des données KindMother.

**Référence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuBooking sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `booking` — cohérent avec la projection domains.json (blocs du domaine « booking »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuBooking est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Équivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
