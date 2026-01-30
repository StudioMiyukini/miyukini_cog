# MiyuPosLoyalty — Documentation Fondatrice

## 1. Contexte

**MiyuPosLoyalty** est le **kit d'outils (Toolkit)** CRM et fidélité PoS de l'écosystème Miyukini. Il intègre les outils de gestion clients (CRUD, adresse, notes) et de programme fidélité (points, solde, carte, octroi/rédemption), alignés sur le document [Équivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md).

L'autorité sur les données (clients, adresses, notes, points fidélité) appartient à **KindMother** (Core de données, Strate 4). MiyuPosLoyalty expose des capacités d'exécution gouvernée sans remplacer KindMother ni StrongFather ; les Opérateurs (ex. Opérateur Fidélité/CRM) passent par la gouvernance pour utiliser ces outils. L'octroi et la rédemption de points sont soumis à StrongFather (autorisation).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuPosLoyalty, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implémentation détaillée ; toute décision (octroi/rédemption points) — ressort de StrongFather.

---

## 3. Définition canonique

> **MiyuPosLoyalty est une composition officielle d'outils CRM et fidélité (clients, adresses, notes, points, cartes fidélité, octroi/rédemption), déclarée et gouvernée par l'environnement.**

- MiyuPosLoyalty **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuPosLoyalty **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; l'octroi et la rédemption de points sont autorisés par StrongFather.

**Règle fondamentale :** Toute écriture (client, points) passe par WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyuposloyalty` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` / `crm` / `loyalty` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuPosLoyalty - Reference Outils](./MiyuPosLoyalty%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.crm.customer.get` | Retourne un client par identifiant |
| `tool.crm.customer.list` | Liste les clients (filtres, recherche) |
| `tool.crm.customer.create` | Crée un client à partir de données fournies |
| `tool.crm.customer.update` | Met à jour un client |
| `tool.crm.customer.address.get` | Retourne l'adresse (livraison) du client |
| `tool.crm.customer.note.add` | Ajoute une note à un client |
| `tool.crm.customer.note.list` | Liste les notes d'un client |
| `tool.loyalty.points.grant` | Accorde des points (règles fournies ou gouvernées) |
| `tool.loyalty.points.redeem` | Déduit des points (échange) ; autorisation = StrongFather |
| `tool.loyalty.balance.get` | Retourne le solde points d'un client |
| `tool.loyalty.card.resolve` | Résout une carte fidélité (code/QR) → client + solde |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosLoyalty en contient onze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : points grant/redeem = autorisation StrongFather ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (détail par outil dans Reference Outils) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur toutes les données : clients, adresses, notes, points fidélité. Toute écriture (création/mise à jour client, octroi/rédemption points) passe par **WriteIntent** sous autorité KindMother. MiyuPosLoyalty exécute des capacités atomiques ; l'autorisation d'octroi/rédemption reste à StrongFather.

---

## 9. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
