# JayFaim — Document fondateur

## Contexte

**JayFaim** est le service Miyukini dédié à la **réservation de tables**, à la **commande en ligne** de nourriture et à la gestion des créneaux et menus (restaurants, traiteurs, food trucks). (JayXpose/JayFestival retirés le 2026-04-29 : le couplage « restauration sur événement festival » n’a plus de service appelant.)

Ce document est le **document fondateur** du service : il en fixe la raison d’être, la portée, les principes directeurs et l’intégration avec JayKonta (encaissement si applicable). Il s’adresse aux équipes produit, technique et aux parties prenantes.

## Portée / Scope

- **Périmètre** : Définition du service JayFaim, positionnement, intégration avec JayKonta, usage autonome ou couplé.
- **Hors périmètre** : Spécifications techniques détaillées, contrats d’API, implémentation (référencés dans d’autres documents).
- **Références** : Glossaire Miyukini, document fondateur JayKonta, [Interpolarité des services Jay](..//..//miyukini-webway-system//reference//_index.md).

---

## 1. Raison d’être

### 1.1 Proposition de valeur

**JayFaim** permet à des **restaurateurs, traiteurs et food trucks** de :

- **Gérer la réservation de tables** : créneaux, capacité, confirmation, rappels.
- **Proposer la commande en ligne** : menus, créneaux de service, panier, validation, préparation.
- **Gérer les créneaux et les menus** : plages horaires, disponibilités, cartes, formules, options (allergies, préférences).

### 1.2 Positionnement

| Mode | Description |
|------|-------------|
| **Autonome** | JayFaim est utilisé en autonomie : restaurant, traiteur ou food truck en site propre (réservation, commande en ligne, créneaux). |

---

## 2. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather, KindMother, Master Butler, WorrySentinel. |
| **Réutilisabilité** | S’appuyer sur les Kits d’outils Miyukini existants (Miyauth, Miyubooking, Miyustore, Miyunotify, Miyubilling / JayKonta, etc.) et définir les Opérateurs et Kits spécifiques « restauration » (créneaux, commandes, menus). |
| **Interpolarité** | Conçu pour se coupler avec JayKonta (paiement, encaissement) ; les couplages sont explicites et gouvernés (Mandats de Permission, niveaux de sécurité). |

---

## 3. Intégration et interpolarité

### 3.1 JayFaim avec JayKonta

- **Encaissement** : les paiements (commandes, réservations) peuvent transiter par les Opérateurs JayKonta (mouvements, facturation) selon Mandat et contexte.
- **Responsabilités** : JayFaim détient les données commande/créneaux ; JayKonta détient les données comptables et moyens de paiement.

### 3.2 Référence interpolarité

Voir [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md) pour le principe global et les couplages entre services Jay.

---

## 4. Niveaux de sécurité (orientation)

Les données **commandes, réservations et moyens de paiement** sont au moins niveau **Sensitive (2)**. Les données liées aux **paiements et encaissements** relèvent du niveau défini pour JayKonta lorsqu’il y a couplage. La résidence (COG de référence) et les règles d’accès sont à préciser dans un document dédié (niveaux de sécurité, politique de résidence), aligné avec le Glossaire et la Politique de résidence des données sensibles.

---

## 5. Prochaines étapes (orientation)

1. **Fonder** : Valider ce document fondateur et le diffuser.
2. **Spécifier** : Documenter les Opérateurs et Kits JayFaim (créneaux, commandes, menus, liaison JayKonta).
3. **Intégration** : Formaliser les contrats d’intégration avec JayKonta (encaissement).
4. **Implémentation** : Développer les Opérateurs et Kits en s’appuyant sur les Cores.

---

## 6. Références

| Document | Rôle |
|----------|------|
| [Miyukini Conceptual References — Glossaire](..//..//miyukini-webway-system//reference//_index.md) | Terminologie (Opérateur, Mandat, COG, Niveaux de sécurité). |
| [JayKonta - Document Fondateur](../JayKonta/JayKonta%20-%20Document%20Fondateur.md) | Service consommateur pour encaissement et facturation. |
| [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md) | Principe d’interpolarité et couplages entre services Jay. |

---

**Document** : JayFaim — Document fondateur  
**Version** : 1.0  
**Date** : 2026-02-02  
**Statut** : Document de référence — non contractuel pour l’implémentation.

