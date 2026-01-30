# MiyuValidate — Documentation Fondatrice

## 1. Contexte

**MiyuValidate** est le **kit d'outils (Toolkit)** de validation et sanitization de données de l'écosystème Miyukini. Il intègre les outils de validation selon schéma (ex. JSON Schema), de sanitization de champs (string, nombre, liste), et de cohérence de types, sans logique métier — le schéma et les données sont fournis dans le flux gouverné ; les règles métier de validation (qui valide quoi, quand) relèvent de **StrongFather** et des Opérateurs.

L'autorité sur les schémas et règles métier appartient à **KindMother** et **StrongFather**. MiyuValidate expose des capacités d'exécution gouvernée (valider schéma, sanitiser) ; les décisions sur les règles métier à appliquer relèvent de **StrongFather** et des Opérateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuValidate, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (moteur JSON Schema, sanitizers) ; les règles métier de validation (StrongFather / Opérateurs). MiyuWeb couvre la validation de **formulaires** (structure, champs) ; MiyuValidate couvre la validation **générique** (schéma, sanitization) réutilisable par plusieurs Opérateurs.

---

## 3. Définition canonique

> **MiyuValidate est une composition officielle d'outils de validation et sanitization de données (schéma, sanitize), déclarée et gouvernée par l'environnement.**

- MiyuValidate **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuValidate **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (valider selon schéma fourni, sanitiser selon politique fournie) ; schéma et données fournis dans le flux ; pas de décision sur les règles métier.

**Règle fondamentale :** Un Tool MiyuValidate exécute sur des **données et schéma/politique fournis** ; il ne décide pas des règles métier à appliquer — cela relève de StrongFather et des Opérateurs.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.validate.miyuvalidate` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `validate` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuValidate - Reference Outils](./MiyuValidate%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.validate.schema.check` | Valide des données selon un schéma fourni (ex. JSON Schema) ; retourne succès/erreurs |
| `tool.validate.sanitize` | Sanitise une valeur selon type et politique fournis (string, nombre, liste, échappement) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuValidate en contient deux.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : schéma et politique fournis dans le flux ; MiyuValidate ne lit pas la base directement ; pas de décision métier sur les règles de validation.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (sanitization peut être sensible selon données) |
| **États autorisés** | Tous sauf restriction explicite |
| **États interdits** | Aucun par défaut |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les schémas et règles métier. MiyuValidate **ne lit pas** la base directement : schéma et politique sont **fournis dans le flux** (après lecture via MiyuSQL sous autorité KindMother si besoin). MiyuValidate n'écrit pas la base ; il retourne un résultat de validation ou une valeur sanitisée.

---

## 9. Alignement MIP

À l'implémentation : chaque Tool MiyuValidate est une unité logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
