# JayFestival — Interpolarité avec les services Jay

## Contexte

**JayFestival** s’intègre avec plusieurs **services Jay** au sein de l’écosystème COG. L’**interpolarité** désigne cette capacité des services à se coupler : JayFestival consomme ou s’appuie sur JayKonta, JayKoa, JayXpose et JayFaim ; il expose des données vers des intégrateurs communs (ex. JayKoa pour les dates).

Ce document décrit les **couplages côté JayFestival** et pointe vers les documents fondateurs des services partenaires et vers le document de référence global. Il s’adresse aux équipes produit et technique.

## Portée / Scope

- **Périmètre** : Couplages JayFestival ↔ JayXpose, JayFaim, JayKoa, JayKonta ; rôle de JayFestival dans chaque couplage.
- **Hors périmètre** : Spécifications techniques détaillées des API et contrats d’Opérateurs (référencés dans les documents de chaque service).
- **Référence globale** : [Miyukini Conceptual References - Interpolarite Services Jay](../../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md).

---

## 1. JayFestival ↔ JayXpose

**JayXpose** (profil exposant / site vitrine) **s’intègre dans JayFestival**.

| Aspect | Côté JayFestival |
|--------|-------------------|
| **Fiche exposant** | La fiche exposant et le répertoire des exposants de JayFestival peuvent s’appuyer sur le profil JayXpose (données vitrine, catalogue, contact). |
| **Répertoire** | L’annuaire ou le répertoire des exposants (global ou par événement) peut afficher les vitrines JayXpose. |
| **Identité unique** | Un exposant peut avoir une vitrine JayXpose et participer à des éditions JayFestival avec le même profil ; pas de duplication. |

**Référence** : [JayXpose - Document Fondateur](../../JayXpose/JayXpose%20-%20Document%20Fondateur.md).

---

## 2. JayFestival ↔ JayFaim

**JayFaim** (restauration, food trucks, commande en ligne) **se couple avec JayFestival** sur les événements.

| Aspect | Côté JayFestival |
|--------|-------------------|
| **Restauration sur événement** | Sur une édition festival, la restauration (stands, food trucks, points de vente) peut être gérée via JayFaim : créneaux, commandes, paiement selon Mandats. |
| **Orchestration** | Les flux **commande / créneaux / paiement** sont orchestrés entre JayFaim, JayFestival et JayKonta (encaissement si applicable). |
| **Données** | JayFestival détient les données événement (éditions, stands, exposants) ; JayFaim détient les données métier restauration (menus, commandes, créneaux). |

**Référence** : [JayFaim - Document Fondateur](../../JayFaim/JayFaim%20-%20Document%20Fondateur.md).

---

## 3. JayFestival ↔ JayKoa

**JayKoa** intègre tout ce qui manipule des **dates** ; JayFestival publie des entrées agenda vers JayKoa.

| Aspect | Côté JayFestival |
|--------|-------------------|
| **Entrées agenda** | JayFestival publie les **éditions**, **participations** et **ateliers / créneaux** vers JayKoa pour agrégation calendrier et détection de conflits. |
| **Vue agrégée** | Un exposant ou un visiteur peut disposer d’une **vue calendrier unifiée** (JayKoa) incluant les éditions et participations JayFestival. |
| **Conflits de dates** | La gestion d’agenda cross-événements (exposant/visiteur) s’appuie sur les capacités JayKoa (conflits, fuseaux, export). |

**Référence** : [JayKoa - Document Fondateur](../../JayKoa/JayKoa%20-%20Document%20Fondateur.md), [JayKoa - Integration Services Consommateurs](../../JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md).

---

## 4. JayFestival ↔ JayKonta

**JayKonta** (budget, devis, facturation) est **consommé par JayFestival** pour la comptabilité par édition et la facturation des exposants.

| Aspect | Côté JayFestival |
|--------|-------------------|
| **Budget par édition** | JayFestival enregistre les revenus et dépenses par édition via les Opérateurs JayKonta (`budget.movements.record`). |
| **Devis et factures exposants** | Création de devis et émission de factures pour les exposants via JayKonta (`quote.create`, `invoice.emit`). |
| **Données** | JayFestival détient les données métier (exposant, édition) ; JayKonta détient les données comptables. |

**Référence** : [JayKonta - Document Fondateur](../../JayKonta/JayKonta%20-%20Document%20Fondateur.md), [JayKonta - Integration Services](../../JayKonta/reference/JayKonta%20-%20Integration%20Services.md).

---

## 5. Synthèse des couplages

| Service | Rôle du couplage avec JayFestival |
|---------|-----------------------------------|
| **JayXpose** | Profil exposant et vitrine ; fiche et répertoire exposants. |
| **JayFaim** | Restauration sur événement ; créneaux, commandes, paiement. |
| **JayKoa** | Agenda agrégé ; éditions, participations, conflits de dates. |
| **JayKonta** | Budget édition, devis et factures exposants, encaissements. |

---

## 6. État de la documentation et décisions à trancher

Pour une **implémentation complète incluant l’UI**, l’état de la documentation de chaque service interfacé (Jay, Miyu*, Cores), les **manques** et les **ambiguïtés ou choix humains** à trancher sont détaillés dans :

- [JayFestival - Etat Documentation Services Interfaces](./JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md).

**Décisions P0 (tranchées)** : **Miyuinvoice + JayKonta** — facturation exposants = Miyuinvoice en façade avec JayKonta en backend (devis, factures, encaissements). **JayXpose est dans l’alpha** — le parcours demande de stands et l’annuaire exposants en dépendent (fiche exposant, répertoire). JayFaim = hors scope alpha (phase 2). **P1 (tranchées)** : **Miyuprofile** = Supabase uniquement pour le moment (source de vérité profil = tables Supabase). **JayKoa** organise les données et fait l’interface avec l’utilisateur ; **MiyuClock** atteste l’horaire et la date IRL (référentiel temps réel). Voir [Bornage Implementation](../JayFestival%20-%20Bornage%20Implementation.md) et [État Documentation Services Interfaces](./JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md).

---

## 7. Voir aussi

- [Miyukini Conceptual References - Interpolarite Services Jay](../../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) — principe global et tableau des documents fondateurs.
- [JayFestival - Document Fondateur](../JayFestival%20-%20Document%20Fondateur.md) — raison d’être, vision, macro, distribution.

---

**Document** : JayFestival — Interpolarité avec les services Jay  
**Version** : 1.0  
**Date** : 2026-02-02  
**Statut** : Document de référence — interpolarité côté JayFestival.
