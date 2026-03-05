# MiyuBooking â€” Documentation Fondatrice

## 1. Contexte

**MiyuBooking** est le **kit d'outils (Toolkit)** de rÃ©servation en ligne (crÃ©neaux, disponibilitÃ©s, rÃ©servations, ressources, tarification, participants) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de liste et rÃ©solution de crÃ©neaux, de crÃ©ation, mise Ã  jour et annulation de rÃ©servations, de rÃ©solution et disponibilitÃ© des ressources, de calcul de prix et de participants, alignÃ©s sur KindMother pour la persistance des donnÃ©es.

L'autoritÃ© sur les donnÃ©es (rÃ¨gles de crÃ©neaux, ressources, rÃ©servations, tarifs) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuBooking expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (slots, create, update, cancel, resource, price, participants) sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuBooking
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother (persistance crÃ©neaux, ressources, rÃ©servations)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (moteur de crÃ©neaux, fuseaux horaires)
- Toute dÃ©cision d'autorisation, de confirmation ou d'annulation â€” celle-ci reste du ressort de StrongFather et des Cores
- Les rappels et notifications (OpÃ©rateur Automatisation ou phase ultÃ©rieure)

---

## 3. DÃ©finition canonique

> **MiyuBooking est une composition officielle d'outils de rÃ©servation en ligne (crÃ©neaux, rÃ©servations, ressources, tarification, participants), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuBooking **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuBooking **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (lister/rÃ©soudre crÃ©neaux, crÃ©er/mettre Ã  jour/annuler rÃ©servation, rÃ©soudre/disponibilitÃ© ressource, calculer prix/participants) sans dÃ©cider de l'autorisation ni de la politique de rÃ©servation.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. Toute persistance (rÃ©servations, rÃ¨gles de crÃ©neaux, ressources) et toute dÃ©cision (autoriser, annuler) sont sous autoritÃ© KindMother (WriteIntent) et StrongFather (ALLOW/DENY).

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.booking.reservations` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `booking` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuBooking - Reference Outils](./MiyuBooking%20-%20Reference%20Outils.md). MiyuBooking est composÃ© des Tools suivants (format canonique `tool.booking.<sous-domaine>.<action>` ou `tool.booking.<action>`).

| ToolId | Description courte |
|--------|---------------------|
| `tool.booking.slots.list` | Liste les crÃ©neaux disponibles pour un contexte (ressource, date, durÃ©e) fourni |
| `tool.booking.slots.resolve` | RÃ©sout un crÃ©neau par identifiant |
| `tool.booking.create` | CrÃ©e une rÃ©servation Ã  partir de donnÃ©es fournies ; WriteIntent KindMother |
| `tool.booking.update` | Met Ã  jour une rÃ©servation (dÃ©placement, prolongation) |
| `tool.booking.cancel` | Annule une rÃ©servation ; dÃ©cision politique = StrongFather |
| `tool.booking.resource.resolve` | RÃ©sout une ressource (salle, Ã©quipement) et ses contraintes |
| `tool.booking.resource.availability` | Retourne la disponibilitÃ© d'une ressource sur une plage donnÃ©e |
| `tool.booking.price.compute` | Calcule le prix d'une rÃ©servation (rÃ¨gles fournies) |
| `tool.booking.participants.compute` | Calcule places restantes / participants pour un crÃ©neau |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuBooking en contient neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : dÃ©cision (crÃ©ation rÃ©servation, annulation) = StrongFather ; toute Ã©criture (rÃ©servation) = WriteIntent KindMother. Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** selon opÃ©ration (lecture crÃ©neaux 0â€“1, crÃ©ation/annulation rÃ©servation 1â€“2) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuBooking - Tool Governance Compliance Contract](./contracts/governance/MiyuBooking%20-%20Tool%20Governance%20Compliance%20Contract.md).

- **KindMother** est l'autoritÃ© sur les rÃ¨gles de crÃ©neaux, les ressources (salles, Ã©quipements), les rÃ©servations et les tarifs. MiyuBooking exÃ©cute des capacitÃ©s (slots.list/resolve, create, update, cancel, resource.resolve/availability, price.compute, participants.compute) **sans dÃ©cider** de l'autorisation (StrongFather) ni de la politique de rÃ©servation ; les rÃ¨gles sont fournies par KindMother ou dans le flux.
- Les crÃ©neaux peuvent Ãªtre affichÃ©s dans le fuseau horaire de l'utilisateur ; les rÃ¨gles de disponibilitÃ© et les ressources sont des donnÃ©es KindMother.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuBooking sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). RÃ©fÃ©rence : [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : `booking` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« booking Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuBooking est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


