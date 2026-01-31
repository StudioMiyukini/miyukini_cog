# MiyuText — Documentation Fondatrice

## 1. Contexte

**MiyuText** est le **kit d'outils (Toolkit)** de traitement de texte de l'écosystème Miyukini. Il intègre les outils de rendu markdown vers HTML, de recherche/remplacement dans des chaînes, de substitution de templates texte (placeholders), et de sanitization pour affichage sécurisé (XSS, échappement), sans logique métier — le contenu et les options sont fournis dans le flux gouverné.

L'autorité sur le contenu métier appartient à **KindMother** et aux Opérateurs. MiyuText expose des capacités d'exécution gouvernée (rendre, remplacer, substituer, sanitiser) ; les décisions sur le contenu à afficher ou modifier relèvent de **StrongFather** et des Opérateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuText, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (moteur markdown, politique CSP) ; l'édition de fichiers (MiyuText opère sur chaînes fournies).

---

## 3. Définition canonique

> **MiyuText est une composition officielle d'outils de traitement de texte (markdown, recherche/remplacement, templates, sanitization), déclarée et gouvernée par l'environnement.**

- MiyuText **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuText **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (rendre du markdown, remplacer, substituer des placeholders, sanitiser) ; le contenu et les options sont fournis dans le flux.

**Règle fondamentale :** Un Tool MiyuText exécute sur des **chaînes et options fournies** ; il ne lit pas la base ni ne décide du contenu à publier.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.text.miyutext` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `text` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuText - Reference Outils](./MiyuText%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.text.markdown.render` | Rend du markdown fourni en HTML (options fournies) ; ne décide pas du contenu |
| `tool.text.replace` | Recherche et remplacement dans une chaîne (littéral ou regex fournis) |
| `tool.text.template.apply` | Substitue des placeholders dans un template (données fournies) |
| `tool.text.sanitize` | Sanitise une chaîne pour affichage sécurisé (XSS, échappement) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuText en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : contenu et options fournis dans le flux ; aucune lecture base ; toute écriture métier = WriteIntent KindMother (MiyuText n'écrit pas la base).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (sanitization = sensible selon contenu) |
| **États autorisés** | Tous sauf restriction explicite |
| **États interdits** | Aucun par défaut |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur le contenu métier. MiyuText **n'écrit pas** et **ne lit pas** la base : il opère sur des **chaînes fournies dans le flux**. Les Opérateurs (ex. MiyuCMS, MiyuWeb) récupèrent le contenu via MiyuSQL sous autorité KindMother, puis appellent MiyuText pour rendre, remplacer ou sanitiser.

Les obligations de conformité détaillées sont dans [MiyuText - Tool Governance Compliance Contract](./contracts/governance/MiyuText%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

À l'implémentation : chaque Tool MiyuText est une unité logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

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
