# Miyukini Conceptual References — Interpolarité des services Jay

## Contexte

Les **services Jay** (JayRDV, JayFestival, JayKoa, JayKonta/JayBudget, JayXpose, JayFaim, etc.) sont conçus pour **se coupler** au sein de l’écosystème COG. L’**interpolarité** désigne cette capacité des services à s’intégrer mutuellement : un service peut consommer les capacités d’un autre, exposer des données vers un intégrateur commun (ex. JayKoa pour les dates), ou s’insérer dans le parcours d’un autre (ex. JayXpose dans JayFestival).

Ce document fixe la **plus-value interpolarité** : il en définit le principe, les couplages majeurs et les références croisées. Il s’adresse aux équipes produit, technique et aux parties prenantes.

## Portée / Scope

- **Périmètre** : Principe d’interpolarité, couplages entre services Jay (JayXpose ↔ JayFestival, JayFaim ↔ JayFestival, JayKoa intégrateur des dates), références vers les documents fondateurs.
- **Hors périmètre** : Spécifications techniques détaillées des API et contrats d’Opérateurs (référencés dans les documents de chaque service).
- **Nomenclature** : Les services de la famille « Jay » suivent le préfixe **JayXxx** (voir [Glossaire — Nomenclature des composants](./Miyukini%20Conceptual%20References%20-%20Glossaire.md#nomenclature-des-composants-préfixes)).
- **Références** : Glossaire Miyukini, documents fondateurs JayFestival, JayKoa, JayKonta, JayRDV, JayXpose, JayFaim.

---

## 1. Principe

> **Les services Jay sont conçus pour se coupler ; l’interpolarité est une propriété de conception, pas un ajout a posteriori.**

- Chaque service expose des **Opérateurs** et **Kits d’outils** gouvernés (StrongFather, KindMother, Master Butler, WorrySentinel).
- Les **couplages** sont explicites : un service peut consommer un autre (ex. JayFestival consomme JayKonta pour la facturation), s’intégrer dans un autre (ex. JayXpose dans JayFestival), ou agréger des données d’autres services (ex. JayKoa agrège les entrées agenda de JayRDV et JayFestival).
- Les **Mandats de Permission** et les **niveaux de sécurité** (WorrySentinel) encadrent les flux entre services ; aucune donnée ni décision ne circule hors gouvernance.

---

## 2. JayXpose ↔ JayFestival

**JayXpose** (profil exposant / site vitrine) **s’intègre dans JayFestival**.

| Aspect | Description |
|--------|-------------|
| **Rôle de JayXpose** | Profil exposant et site vitrine pour artisans, artistes, petites marques : catalogue, contact, portfolio, lien vers réservation ou boutique. |
| **Intégration dans JayFestival** | La **fiche exposant** et le **répertoire des exposants** de JayFestival peuvent s’appuyer sur JayXpose ; un exposant peut avoir une **vitrine JayXpose** et participer à des **éditions JayFestival** avec le même profil. |
| **Vitrine autonome** | JayXpose peut également être utilisé en mode autonome (site vitrine sans événement festival). |
| **Données** | Les données exposant (profil, contenu vitrine) sont gouvernées ; la résidence et le niveau de sécurité sont définis par le contrat du service et le contexte (JayFestival vs. autonome). |

**Référence** : [JayXpose - Document Fondateur](../services/JayXpose/JayXpose%20-%20Document%20Fondateur.md), [JayFestival - Document Fondateur](../services/JayFestival/JayFestival%20-%20Document%20Fondateur.md).

---

## 3. JayFaim ↔ JayFestival

Les **commandes en ligne JayFaim** (restauration, food trucks, réservation de tables) **peuvent se coupler avec JayFestival**.

| Aspect | Description |
|--------|-------------|
| **Rôle de JayFaim** | Réservation de tables, commande en ligne de nourriture, gestion des créneaux et des menus (restaurants, traiteurs, food trucks). |
| **Couplage avec JayFestival** | Sur un **événement JayFestival**, la restauration (stands, food trucks, points de vente) peut être gérée via JayFaim : créneaux, commandes, paiement selon Mandats. Les flux **commande / créneaux / paiement** sont orchestrés entre JayFaim et JayFestival (et JayKonta pour l’encaissement si applicable). |
| **Données** | JayFaim détient les données métier (menus, commandes, créneaux) ; JayFestival détient les données événement (éditions, stands, exposants). La liaison est explicite et gouvernée. |

**Référence** : [JayFaim - Document Fondateur](../services/JayFaim/JayFaim%20-%20Document%20Fondateur.md), [JayFestival - Document Fondateur](../services/JayFestival/JayFestival%20-%20Document%20Fondateur.md).

---

## 4. JayKoa, intégrateur des dates

**JayKoa intègre tout ce qui manipule des dates.**

| Aspect | Description |
|--------|-------------|
| **Rôle de JayKoa** | Service unifié du domaine **agenda** : modélisation des entrées (RDV, éditions, ateliers, participations), détection de conflits, vue calendrier agrégée, fuseaux, export (iCal, PDF). |
| **Services sources** | JayKoa agrège les **entrées agenda** publiées par : **JayRDV** (RDV, créneaux, exceptions), **JayFestival** (éditions, participations, ateliers réservés), et **tout futur service** qui manipule des plages temporelles (formations, interventions, etc.). |
| **Vue agrégée** | Un même utilisateur (ex. exposant, professionnel, visiteur) peut disposer d’une **vue calendrier unifiée** et d’une **détection de conflits cross-service** (ex. RDV le même jour qu’une édition festival), sous réserve des Mandats et des niveaux de sécurité. |
| **Énoncé** | *« JayKoa intègre tout ce qui manipule des dates. »* |

**Référence** : [JayKoa - Document Fondateur](../services/JayKoa/JayKoa%20-%20Document%20Fondateur.md), [JayKoa - Integration Services Consommateurs](../services/JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md).

### 4.1 JayKoa vs MiyuClock (rôles)

| Composant | Rôle |
|-----------|------|
| **JayKoa** | **Organise les données** (entrées agenda, éditions, participations, créneaux) et **fait l’interface avec l’utilisateur** (vue calendrier, conflits, export, fuseaux d’affichage). |
| **MiyuClock** | **Atteste l’horaire et la date IRL** (référentiel temps réel) : outils de mesure du temps (instant présent, delta), fournis par le Kernel ; pas de persistance, pas de temps global (LOI-4). Les services consommateurs (JayKoa, JayFestival, etc.) utilisent MiyuClock pour l’attestation temporelle et JayKoa pour l’organisation et l’affichage des données agenda. |

**Référence** : [MiyuClock - Documentation Fondatrice](../tools/MiyuClock/MiyuClock%20-%20Documentation%20Fondatrice.md), [MiyuClock - Reference Outils](../tools/MiyuClock/MiyuClock%20-%20Reference%20Outils.md).

---

## 5. Références croisées (documents fondateurs)

| Service | Document fondateur |
|---------|---------------------|
| **JayRDV** | [JayRDV - Document Fondateur](../services/JayRDV/JayRDV%20-%20Document%20Fondateur.md) |
| **JayFestival** | [JayFestival - Document Fondateur](../services/JayFestival/JayFestival%20-%20Document%20Fondateur.md) |
| **JayKoa** | [JayKoa - Document Fondateur](../services/JayKoa/JayKoa%20-%20Document%20Fondateur.md) |
| **JayKonta** (service COG, marques JayBudget et JayKonta) | [JayKonta - Document Fondateur](../services/JayKonta/JayKonta%20-%20Document%20Fondateur.md) |
| **JayXpose** | [JayXpose - Document Fondateur](../services/JayXpose/JayXpose%20-%20Document%20Fondateur.md) |
| **JayFaim** | [JayFaim - Document Fondateur](../services/JayFaim/JayFaim%20-%20Document%20Fondateur.md) |

---

## 6. Voir aussi

- [Miyukini Conceptual References — Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Terminologie (Opérateur, Mandat, COG, Service).
- [Miyukini Conceptual References — Vision stratégique](./Miyukini%20Conceptual%20References%20-%20Vision%20Strategique.md) : Objectifs stratégiques, B2B2C.

---

**Document** : Miyukini Conceptual References — Interpolarité des services Jay  
**Version** : 1.0  
**Date** : 2026-02-02  
**Statut** : Document de référence — source de vérité pour l’interpolarité des services Jay.
