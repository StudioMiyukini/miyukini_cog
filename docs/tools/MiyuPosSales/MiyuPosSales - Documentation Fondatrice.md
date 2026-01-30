# MiyuPosSales — Documentation Fondatrice

## 1. Contexte

**MiyuPosSales** est le **kit d'outils (Toolkit)** de caisse et ventes au point de vente (PoS) de l'écosystème Miyukini. Il intègre les outils d'enregistrement des ventes (tickets, lignes, reçus), remises, remboursements, gestion de caisse, variantes et modificateurs d'articles, codes-barres, contexte magasin et affichage client, alignés sur le document [Équivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md).

L'autorité sur les données (ventes, tickets, reçus, mouvements caisse) appartient à **KindMother** (Core de données, Strate 4). MiyuPosSales expose des capacités d'exécution gouvernée sans remplacer KindMother ni StrongFather ; les Opérateurs (ex. Opérateur Caisse) passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuPosSales
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother (WriteIntent pour toute écriture)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (persistance, matériel imprimante/tiroir)
- Toute décision ALLOW/DENY (remboursement, remise) — ressort de StrongFather

---

## 3. Définition canonique

> **MiyuPosSales est une composition officielle d'outils de caisse et ventes PoS (ventes, tickets, reçus, remises, remboursements, caisse, articles, codes-barres, contexte magasin, affichage client), déclarée et gouvernée par l'environnement.**

- MiyuPosSales **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuPosSales **n'ajoute aucune logique métier** : il orchestre des capacités atomiques sans décider des autorisations (remboursement, remise = StrongFather).

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Toute écriture passe par WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyupossales` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `pos` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuPosSales est composé des Tools suivants (format canonique `tool.pos.<sous-domaine>.<action>`). Le détail de chaque outil est décrit dans [MiyuPosSales - Reference Outils](./MiyuPosSales%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.pos.sale.create` | Crée une vente (ticket) |
| `tool.pos.sale.add_item` | Ajoute une ligne à une vente |
| `tool.pos.sale.remove_item` | Retire une ligne |
| `tool.pos.ticket.open` | Ouvre un ticket |
| `tool.pos.ticket.save` | Sauvegarde un ticket |
| `tool.pos.ticket.close` | Clôture un ticket |
| `tool.pos.ticket.list` | Liste les tickets |
| `tool.pos.discount.apply` | Applique une remise |
| `tool.pos.refund.record` | Enregistre un remboursement ; autorisation = StrongFather |
| `tool.pos.receipt.render` | Produit le contenu du reçu |
| `tool.pos.receipt.print` | Imprime le reçu |
| `tool.pos.receipt.send` | Envoie le reçu par email |
| `tool.pos.receipt.list` | Liste les reçus |
| `tool.pos.item.variant.resolve` | Résout une variante article |
| `tool.pos.item.modifier.apply` | Applique des modificateurs |
| `tool.pos.cash.register.open` | Ouvre une session caisse |
| `tool.pos.cash.register.close` | Clôture une session caisse |
| `tool.pos.cash.movement.record` | Enregistre un mouvement espèces |
| `tool.pos.barcode.parse` | Parse un code-barres |
| `tool.pos.context.store.resolve` | Résout le magasin courant |
| `tool.pos.display.push` | Envoie les données à l'écran client |
| `tool.pos.order.service_type.set` | Définit le type de service (sur place / à emporter / livraison) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosSales en contient vingt-deux.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : décision métier (remboursement, remise) = StrongFather ; toute écriture = WriteIntent KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (détail par outil dans Reference Outils) ; cohérent avec WorrySentinel. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother

- **KindMother** est l'autorité sur toutes les données : ventes, tickets, reçus, mouvements caisse, catalogue. Toute écriture (vente, ticket, reçu, ouverture/fermeture caisse) passe par **WriteIntent** sous autorité KindMother.
- **MiyuPosSales** n'exécute que des capacités atomiques ; il ne décide pas (remboursement autorisé ou non = StrongFather). Les données sont fournies dans le flux ou persistées via KindMother/MiyuSQL en amont.

**Référence :** [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md).

---

## 9. Alignement MIP

- **Domaine** : `pos` — cohérent avec la projection domains.json.
- **Layer** : Strate 6 (Tools & Toolkits).
- **Blocs** : chaque Tool MiyuPosSales est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation.

À l'implémentation, le code devra être balisé MSCM afin que l'index MIP soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Équivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
