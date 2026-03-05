# MiyuValidate â€” Documentation Fondatrice

## 1. Contexte

**MiyuValidate** est le **kit d'outils (Toolkit)** de validation et sanitization de donnÃ©es de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de validation selon schÃ©ma (ex. JSON Schema), de sanitization de champs (string, nombre, liste), et de cohÃ©rence de types, sans logique mÃ©tier â€” le schÃ©ma et les donnÃ©es sont fournis dans le flux gouvernÃ© ; les rÃ¨gles mÃ©tier de validation (qui valide quoi, quand) relÃ¨vent de **StrongFather** et des OpÃ©rateurs.

L'autoritÃ© sur les schÃ©mas et rÃ¨gles mÃ©tier appartient Ã  **KindMother** et **StrongFather**. MiyuValidate expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (valider schÃ©ma, sanitiser) ; les dÃ©cisions sur les rÃ¨gles mÃ©tier Ã  appliquer relÃ¨vent de **StrongFather** et des OpÃ©rateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuValidate, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (moteur JSON Schema, sanitizers) ; les rÃ¨gles mÃ©tier de validation (StrongFather / OpÃ©rateurs). MiyuWeb couvre la validation de **formulaires** (structure, champs) ; MiyuValidate couvre la validation **gÃ©nÃ©rique** (schÃ©ma, sanitization) rÃ©utilisable par plusieurs OpÃ©rateurs.

---

## 3. DÃ©finition canonique

> **MiyuValidate est une composition officielle d'outils de validation et sanitization de donnÃ©es (schÃ©ma, sanitize), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuValidate **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuValidate **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (valider selon schÃ©ma fourni, sanitiser selon politique fournie) ; schÃ©ma et donnÃ©es fournis dans le flux ; pas de dÃ©cision sur les rÃ¨gles mÃ©tier.

**RÃ¨gle fondamentale :** Un Tool MiyuValidate exÃ©cute sur des **donnÃ©es et schÃ©ma/politique fournis** ; il ne dÃ©cide pas des rÃ¨gles mÃ©tier Ã  appliquer â€” cela relÃ¨ve de StrongFather et des OpÃ©rateurs.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.validate.miyuvalidate` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `validate` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuValidate - Reference Outils](./MiyuValidate%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.validate.schema.check` | Valide des donnÃ©es selon un schÃ©ma fourni (ex. JSON Schema) ; retourne succÃ¨s/erreurs |
| `tool.validate.sanitize` | Sanitise une valeur selon type et politique fournis (string, nombre, liste, Ã©chappement) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuValidate en contient deux.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : schÃ©ma et politique fournis dans le flux ; MiyuValidate ne lit pas la base directement ; pas de dÃ©cision mÃ©tier sur les rÃ¨gles de validation.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (sanitization peut Ãªtre sensible selon donnÃ©es) |
| **Ã‰tats autorisÃ©s** | Tous sauf restriction explicite |
| **Ã‰tats interdits** | Aucun par dÃ©faut |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les schÃ©mas et rÃ¨gles mÃ©tier. MiyuValidate **ne lit pas** la base directement : schÃ©ma et politique sont **fournis dans le flux** (aprÃ¨s lecture via MiyuSQL sous autoritÃ© KindMother si besoin). MiyuValidate n'Ã©crit pas la base ; il retourne un rÃ©sultat de validation ou une valeur sanitisÃ©e.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuValidate - Tool Governance Compliance Contract](./contracts/governance/MiyuValidate%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

Ã€ l'implÃ©mentation : chaque Tool MiyuValidate est une unitÃ© logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


