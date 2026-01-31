# MiyuPosPayment — Documentation Fondatrice

## 1. Contexte

**MiyuPosPayment** est le **kit d'outils (Toolkit)** paiements PoS de l'écosystème Miyukini. Il intègre les outils d'enregistrement paiement espèces, chèque, partage d'addition et les adaptateurs terminaux CB (autorisation, capture), alignés sur le document [Équivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md).

L'autorité sur les données (paiements, sessions) appartient à **KindMother**. Les décisions (partage d'addition, autorisation CB) relèvent de **StrongFather** et **WorrySentinel** (niveau de sécurité). MiyuPosPayment expose des capacités d'exécution gouvernée ; les Opérateurs (ex. Opérateur Caisse, Paiement) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuPosPayment, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implémentation détaillée (terminaux SumUp, Zettle, etc.) ; toute décision d'autorisation — ressort de StrongFather / WorrySentinel.

---

## 3. Définition canonique

> **MiyuPosPayment est une composition officielle d'outils paiements PoS (espèces, chèque, partage d'addition, terminaux CB), déclarée et gouvernée par l'environnement.**

- MiyuPosPayment **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuPosPayment **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; l'autorisation (partage, CB) appartient à StrongFather.

**Règle fondamentale :** Toute écriture (paiement enregistré) = WriteIntent vers KindMother. Niveau de sécurité élevé pour paiements (WorrySentinel).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyupospayment` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` / `payment` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuPosPayment - Reference Outils](./MiyuPosPayment%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.pos.payment.cash.record` | Enregistre un paiement espèces |
| `tool.pos.payment.check.record` | Enregistre un paiement chèque |
| `tool.pos.payment.split` | Répartit le paiement entre plusieurs moyens (données fournies) ; autorisation = StrongFather |
| `tool.payment.terminal.authorize` | Demande une autorisation à un terminal CB (données fournies) |
| `tool.payment.terminal.capture` | Confirme (capture) un paiement CB précédemment autorisé |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosPayment en contient cinq.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : WorrySentinel applique le niveau de sécurité paiement ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **2 à 3** (paiements sensibles ; détail dans Reference Outils) ; cohérent avec WorrySentinel. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données de paiement (espèces, chèque, partage, références CB). Toute écriture (enregistrement paiement) passe par **WriteIntent** sous autorité KindMother. MiyuPosPayment exécute des capacités atomiques ; l'autorisation (partage, autorisation CB) reste à StrongFather.

Les obligations de conformité détaillées sont dans [MiyuPosPayment - Tool Governance Compliance Contract](./contracts/governance/MiyuPosPayment%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuPosPayment sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuPosPayment devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
