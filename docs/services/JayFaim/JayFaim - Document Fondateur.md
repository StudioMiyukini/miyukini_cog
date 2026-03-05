# JayFaim â€” Document fondateur

## Contexte

**JayFaim** est le service Miyukini dÃ©diÃ© Ã  la **rÃ©servation de tables**, Ã  la **commande en ligne** de nourriture et Ã  la gestion des crÃ©neaux et menus (restaurants, traiteurs, food trucks). Il peut **se coupler avec JayFestival** : sur un Ã©vÃ©nement festival, la restauration (stands, food trucks, points de vente) peut Ãªtre gÃ©rÃ©e via JayFaim (crÃ©neaux, commandes, paiement selon Mandats).

Ce document est le **document fondateur** du service : il en fixe la raison dâ€™Ãªtre, la portÃ©e, les principes directeurs et lâ€™intÃ©gration avec JayFestival (et JayKonta pour lâ€™encaissement si applicable). Il sâ€™adresse aux Ã©quipes produit, technique et aux parties prenantes.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : DÃ©finition du service JayFaim, positionnement, intÃ©gration avec JayFestival et JayKonta, usage autonome ou couplÃ©.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques dÃ©taillÃ©es, contrats dâ€™API, implÃ©mentation (rÃ©fÃ©rencÃ©s dans dâ€™autres documents).
- **RÃ©fÃ©rences** : Glossaire Miyukini, document fondateur JayFestival, document fondateur JayKonta, [InterpolaritÃ© des services Jay](..//..//miyukini-webway-system//reference//_index.md).

---

## 1. Raison dâ€™Ãªtre

### 1.1 Proposition de valeur

**JayFaim** permet Ã  des **restaurateurs, traiteurs et food trucks** de :

- **GÃ©rer la rÃ©servation de tables** : crÃ©neaux, capacitÃ©, confirmation, rappels.
- **Proposer la commande en ligne** : menus, crÃ©neaux de service, panier, validation, prÃ©paration.
- **GÃ©rer les crÃ©neaux et les menus** : plages horaires, disponibilitÃ©s, cartes, formules, options (allergies, prÃ©fÃ©rences).

En **mode couplÃ© JayFestival**, sur un **Ã©vÃ©nement festival**, la restauration (stands, food trucks, points de vente) est gÃ©rÃ©e via JayFaim : crÃ©neaux, commandes, paiement selon Mandats. Les flux **commande / crÃ©neaux / paiement** sont orchestrÃ©s entre JayFaim, JayFestival et JayKonta (encaissement si applicable).

### 1.2 Positionnement

| Mode | Description |
|------|-------------|
| **CouplÃ© JayFestival** | Sur un Ã©vÃ©nement JayFestival, la restauration (stands, food trucks) est gÃ©rÃ©e via JayFaim ; crÃ©neaux, commandes, paiement ; orchestration avec JayFestival et JayKonta. |
| **Autonome** | JayFaim peut Ãªtre utilisÃ© sans JayFestival : restaurant, traiteur ou food truck en site propre (rÃ©servation, commande en ligne, crÃ©neaux). |

---

## 2. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather, KindMother, Master Butler, WorrySentinel. |
| **RÃ©utilisabilitÃ©** | Sâ€™appuyer sur les Kits dâ€™outils Miyukini existants (Miyauth, Miyubooking, Miyustore, Miyunotify, Miyubilling / JayKonta, etc.) et dÃ©finir les OpÃ©rateurs et Kits spÃ©cifiques Â« restauration Â» (crÃ©neaux, commandes, menus). |
| **InterpolaritÃ©** | ConÃ§u pour se coupler avec JayFestival (restauration sur Ã©vÃ©nement) et JayKonta (paiement, encaissement) ; les couplages sont explicites et gouvernÃ©s (Mandats de Permission, niveaux de sÃ©curitÃ©). |

---

## 3. IntÃ©gration et interpolaritÃ©

### 3.1 JayFaim avec JayFestival

- Sur un **Ã©vÃ©nement JayFestival**, la restauration (stands, food trucks, points de vente) peut Ãªtre gÃ©rÃ©e via JayFaim.
- **CrÃ©neaux** : plages de service, capacitÃ© par stand, conflits avec le programme festival (JayKoa si besoin).
- **Commandes** : prise de commande en ligne, prÃ©paration, retrait ou livraison sur site selon rÃ¨gles de lâ€™Ã©dition.
- **Paiement** : selon Mandats ; orchestration avec JayKonta pour lâ€™encaissement (facturation, rÃ¨glement).

JayFestival dÃ©tient les donnÃ©es **Ã©vÃ©nement** (Ã©ditions, stands, exposants) ; JayFaim dÃ©tient les donnÃ©es **mÃ©tier restauration** (menus, commandes, crÃ©neaux). La liaison est explicite et gouvernÃ©e.

### 3.2 JayFaim avec JayKonta

- **Encaissement** : les paiements (commandes, rÃ©servations) peuvent transiter par les OpÃ©rateurs JayKonta (mouvements, facturation) selon Mandat et contexte (festival vs. autonome).
- **ResponsabilitÃ©s** : JayFaim dÃ©tient les donnÃ©es commande/crÃ©neaux ; JayKonta dÃ©tient les donnÃ©es comptables et moyens de paiement.

### 3.3 RÃ©fÃ©rence interpolaritÃ©

Voir [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md) pour le principe global et les couplages entre services Jay.

---

## 4. Niveaux de sÃ©curitÃ© (orientation)

Les donnÃ©es **commandes, rÃ©servations et moyens de paiement** sont au moins niveau **Sensitive (2)**. Les donnÃ©es liÃ©es aux **paiements et encaissements** relÃ¨vent du niveau dÃ©fini pour JayKonta lorsquâ€™il y a couplage. La rÃ©sidence (COG de rÃ©fÃ©rence) et les rÃ¨gles dâ€™accÃ¨s sont Ã  prÃ©ciser dans un document dÃ©diÃ© (niveaux de sÃ©curitÃ©, politique de rÃ©sidence), alignÃ© avec le Glossaire et la Politique de rÃ©sidence des donnÃ©es sensibles.

---

## 5. Prochaines Ã©tapes (orientation)

1. **Fonder** : Valider ce document fondateur et le diffuser.
2. **SpÃ©cifier** : Documenter les OpÃ©rateurs et Kits JayFaim (crÃ©neaux, commandes, menus, liaison JayFestival et JayKonta).
3. **IntÃ©gration** : Formaliser les contrats dâ€™intÃ©gration avec JayFestival (restauration sur Ã©vÃ©nement) et JayKonta (encaissement).
4. **ImplÃ©mentation** : DÃ©velopper les OpÃ©rateurs et Kits en sâ€™appuyant sur les Cores.

---

## 6. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md) | Terminologie (OpÃ©rateur, Mandat, COG, Niveaux de sÃ©curitÃ©). |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service avec lequel JayFaim se couple (restauration sur Ã©vÃ©nement). |
| [JayKonta - Document Fondateur](../JayKonta/JayKonta%20-%20Document%20Fondateur.md) | Service consommateur pour encaissement et facturation. |
| [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md) | Principe dâ€™interpolaritÃ© et couplage JayFaim â†” JayFestival. |

---

**Document** : JayFaim â€” Document fondateur  
**Version** : 1.0  
**Date** : 2026-02-02  
**Statut** : Document de rÃ©fÃ©rence â€” non contractuel pour lâ€™implÃ©mentation.

