---
name: Documentation MiyuSQL tools
overview: Créer le dossier docs/tools/MiyuSQL/, planifier la structure et l'écriture de la documentation sur MiyuSQL (kit d'outils) et sur tous les outils de manipulation DB qu'il contient, en appliquant le Protocole Écriture Documentation Conceptuelle et le MIP v1.
todos: []
isProject: false
---

# Plan : dossier docs/tools/MiyuSQL/ et documentation MiyuSQL

## Objectif

- **Créer** le dossier [docs/tools/MiyuSQL/](docs/tools/MiyuSQL/) (et le parent [docs/tools/](docs/tools/) si absent).
- **Planifier** la structure et l'écriture de la documentation sur :
  - MiyuSQL **en tant que kit d'outils** (identité, composition, gouvernance) ;
  - **Tous les outils** qu'il possède pour manipuler la DB (référence détaillée par outil).
- **Appliquer** :
  - [Protocole Écriture Documentation Conceptuelle](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) : cycle planification → distribution → vérification → gel ; document fondateur en premier ; nomenclature [xx] - [document] ; 1 agent = 1 document ; max 4 agents simultanés par groupe de préfixe.
  - [Protocole MIP v1](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) : terminologie et structure compatibles index (domaines, layers, blocs) pour doc et futur code indexable.

---

## 1. Structure du dossier

```
docs/
├── tools/
│   ├── _index.md
│   └── MiyuSQL/
│       ├── _index.md
│       ├── MiyuSQL - Documentation Fondatrice.md
│       └── MiyuSQL - Reference Outils.md
```

- **docs/tools/** : index global des Kits d'Outils (Strate 6) ; lien vers [Miyukini Conceptual References - Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) et contrats Master Butler.
- **docs/tools/MiyuSQL/** : documentation dédiée au kit MiyuSQL et à ses outils.
- Pas de sous-dossiers `foundation/` ou `reference/` dans MiyuSQL pour rester minimal ; les deux documents principaux sont à la racine de `MiyuSQL/`. Une évolution ultérieure pourra ajouter `contracts/` (ex. intégration KindMother) si besoin.

---

## 2. Conformité aux protocoles

### 2.1 Protocole Écriture Documentation Conceptuelle

- **Phase 1 — Planification** : ce plan = planification ; titre d'étape = « Planification documentation MiyuSQL ».
- **Phase 2 — Distribution** : chaque fichier = une tâche **[xx] - [nom du document]** ; 1 agent = 1 document ; pas de batch multi-documents. Préfixes [01] à [04] (4 documents, ≤ 4 par groupe).
- **Phase 3 — Vérification** : après rédaction, vérification des liens, cohérence avec [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Tool Governance Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md), [Toolkit Composition Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md).
- **Phase 4 — Gel** : optionnel ; version 1.0 et statut sur le document fondateur.

### 2.2 Protocole MIP v1

- La documentation **ne génère pas** les fichiers `mscm_index/*` (MIP indexe le **code** balisé MSCM).
- La doc MiyuSQL **définit les concepts** de façon compatible MIP pour une future implémentation :
  - **Domaine** : `data` (cohérent avec domains.json).
  - **Layer** : outil / toolkit (Strate 6).
  - **Blocs** : chaque Tool = unité logique pouvant devenir un bloc MSCM (`id`, `do`, `role`, `layer`) dans le futur code.
- Inclure une section **Alignement MIP** dans le document fondateur (et/ou en rappel dans la Référence Outils) : préciser que les Tools et le Toolkit sont conçus pour être balisés MSCM à l'implémentation et alimenter blocks.json, domains.json, layers.json selon [MIP v1](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Fichiers à créer et nomenclature des tâches


| Action        | Chemin                                                                                                                     | Tâche (nomenclature)                    |
| ------------- | -------------------------------------------------------------------------------------------------------------------------- | --------------------------------------- |
| Créer dossier | `docs/tools/`                                                                                                              | —                                       |
| Créer dossier | `docs/tools/MiyuSQL/`                                                                                                      | —                                       |
| Créer         | [docs/tools/_index.md](docs/tools/_index.md)                                                                               | [04] - Index de navigation tools        |
| Créer         | [docs/tools/MiyuSQL/_index.md](docs/tools/MiyuSQL/_index.md)                                                               | [03] - Index MiyuSQL                    |
| Créer         | [docs/tools/MiyuSQL/MiyuSQL - Documentation Fondatrice.md](docs/tools/MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) | [01] - Documentation Fondatrice MiyuSQL |
| Créer         | [docs/tools/MiyuSQL/MiyuSQL - Reference Outils.md](docs/tools/MiyuSQL/MiyuSQL%20-%20Reference%20Outils.md)                 | [02] - Reference Outils MiyuSQL         |


- **Ordre de rédaction** : [01] en premier (document fondateur). Puis [02] (dépend de la liste des outils fixée dans [01]). Puis [03] et [04] peuvent être rédigés en parallèle (ils ne font que lier les docs existantes).
- **Règle** : 1 agent = 1 document. [03] et [04] peuvent être distribués en parallèle (2 ≤ 4).

---

## 4. Contenu prévu par document

### 4.1 [01] MiyuSQL - Documentation Fondatrice.md

Document **fondateur** (1er rédigé). Nomenclature : préfixe **MiyuSQL**, sujet **Documentation Fondatrice**, sans accent.


| Section                            | Contenu                                                                                                                                                                                                                                                                                                                |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Titre H1**                       | MiyuSQL — Documentation Fondatrice                                                                                                                                                                                                                                                                                     |
| **1. Contexte**                    | MiyuSQL = kit d'outils (Toolkit) de gestion de données DB ; primordial, très utilisé ; intègre tous les outils de manipulation de données en base ; autorité données = KindMother.                                                                                                                                     |
| **2. Portée / Scope**              | Ce qui est défini (identité, ToolkitId, Tools composants, gouvernance) ; hors scope (implémentation détaillée, logique métier).                                                                                                                                                                                        |
| **3. Définition canonique**        | MiyuSQL = composition officielle d'outils de manipulation de données DB, déclarée et gouvernée ; pas un nouveau Tool, pas de logique métier.                                                                                                                                                                           |
| **4. Identifiant et catalogue**    | **ToolkitId** : `toolkit.data.miyusql` (format Master Butler `toolkit.<domain>.<name>`). Référence au [Tool Governance Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md).                                                                                        |
| **5. Liste des outils composants** | Énumération des ToolIds (ex. `tool.query.execute`, `tool.query.prepare`, `tool.transaction.begin`, `tool.transaction.commit`, `tool.transaction.rollback`, `tool.cache.get`, `tool.cache.set`, `tool.cache.invalidate`) ; au moins 2 (invariant Toolkit Composition). Renvoi vers [02] pour le détail de chaque outil. |
| **6. Gouvernance**                 | Flux : Opérateur → BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather → exécution. KindMother = autorité sur les données. Schéma ou lien vers [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md).                                         |
| **7. Niveau de sécurité et états** | Niveau **2** (données utilisateur). États autorisés / interdits (ex. HEALTHY, DEGRADED ; SECURITY_LOCKDOWN, MAINTENANCE interdits) — aligné [Toolkit Composition Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md).                                          |
| **8. Relation avec KindMother**    | Opérations DB sous autorité KindMother (persistance, WriteIntent côté métier). MiyuSQL expose capacités d'exécution gouvernée sans remplacer KindMother. Lien [Accès DB et Droits Agents IA](docs/reference/Miyukini%20Conceptual%20References%20-%20Acces%20DB%20et%20Droits%20Agents%20IA.md) si pertinent.          |
| **9. Alignement MIP**              | Domain `data`, layer outil/toolkit (Strate 6) ; chaque Tool = unité logique pour blocks.json (id, do, role, layer) à l'implémentation. Référence [MIP v1](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).                                                                    |
| **10. Références croisées**        | Glossaire, Tools et Toolkits, Tool Governance, Toolkit Composition, KindMother.                                                                                                                                                                                                                                        |
| **Pied**                           | Date de création, version (ex. 1.0), statut.                                                                                                                                                                                                                                                                           |


### 4.2 [02] MiyuSQL - Reference Outils.md

Référence de **tous les outils** du kit : pour chaque outil, décrire id, action (une phrase), niveau de sécurité typique, et éventuellement capability_id, input/output schéma si défini. Format aligné sur les exemples du [Tool Governance Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) (ToolId `tool.<domain>.<action>`).


| Section                                     | Contenu                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Titre H1**                                | MiyuSQL — Référence des outils                                                                                                                                                                                                                                                                                                                                                                                                                               |
| **1. Contexte**                             | Ce document décrit chaque outil (Tool) composant le kit MiyuSQL ; référence technique, pas de logique métier.                                                                                                                                                                                                                                                                                                                                                |
| **2. Portée / Scope**                       | Liste exhaustive des Tools du kit ; hors scope : implémentation, choix de driver SQL.                                                                                                                                                                                                                                                                                                                                                                        |
| **3. Tableau (ou sous-sections) par outil** | Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte « fait quoi »), **niveau de sécurité** (2 pour données), **capability_id** si applicable. Outils à couvrir (liste indicative) : `tool.query.execute`, `tool.query.prepare`, `tool.transaction.begin`, `tool.transaction.commit`, `tool.transaction.rollback`, `tool.cache.get`, `tool.cache.set`, `tool.cache.invalidate` ; ajouter si besoin `tool.schema.read` (lecture schéma). |
| **4. Alignement MIP**                       | Rappel : chaque outil = bloc logique (id, do, role, layer) pour future indexation MIP.                                                                                                                                                                                                                                                                                                                                                                       |
| **5. Références croisées**                  | Documentation Fondatrice MiyuSQL, Tool Governance Contract, Glossaire.                                                                                                                                                                                                                                                                                                                                                                                       |
| **Pied**                                    | Date, version, statut.                                                                                                                                                                                                                                                                                                                                                                                                                                       |


### 4.3 [03] docs/tools/MiyuSQL/_index.md

Index de navigation du sous-dossier MiyuSQL.

- **Titre H1** : MiyuSQL — Index de navigation (ou équivalent).
- **Contexte** : rôle du kit MiyuSQL (gestion de données DB), lien vers doc fondatrice et référence outils.
- **Structure** : tableau des documents avec description et lien vers [Documentation Fondatrice](docs/tools/MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md), [Référence Outils](docs/tools/MiyuSQL/MiyuSQL%20-%20Reference%20Outils.md).
- **Références** : Tools et Toolkits, Master Butler (Tool Governance, Toolkit Composition), KindMother.

Format aligné sur [docs/kernel/_index.md](docs/kernel/_index.md) (Contexte, tableaux, liens relatifs).

### 4.4 [04] docs/tools/_index.md

Index global du dossier tools.

- **Titre H1** : Tools — Index de navigation.
- **Contexte** : rôle de `docs/tools/` — documentation des Kits d'Outils (Strate 6) et des outils ; lien vers [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) et Master Butler.
- **Portée** : kits documentés (au minimum MiyuSQL).
- **Structure** : tableau des kits / dossiers (lien vers [MiyuSQL/_index.md](docs/tools/MiyuSQL/_index.md)).
- **Références** : Tools et Toolkits, [Tool Governance Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md), [Toolkit Composition Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md).

---

## 5. Points de cohérence contractuelle

- **ToolId** : format `tool.<domain>.<action>[.<qualifier>]` (ex. `tool.query.execute`).
- **ToolkitId** : `toolkit.data.miyusql`.
- **Invariant** : un Toolkit n'ajoute aucune capacité nouvelle ; MiyuSQL orchestre des Tools existants.
- **Security level** du kit : 2 (max des Tools composants).
- **Nomenclature fichiers** : pas d'accents (Reference, Fondatrice).

---

## 6. Ordre d'exécution recommandé

1. Créer les dossiers `docs/tools/` et `docs/tools/MiyuSQL/`.
2. Rédiger **[01] MiyuSQL - Documentation Fondatrice.md** (document fondateur).
3. Rédiger **[02] MiyuSQL - Reference Outils.md** (tous les outils détaillés).
4. Rédiger **[03] MiyuSQL/_index.md** et **[04] docs/tools/_index.md** (en parallèle si deux agents).
5. Vérifier liens et cohérence avec Tools et Toolkits + contrats Master Butler.
6. (Optionnel) Gel / versionnement (v1.0) du document fondateur.

---

## 7. Résumé des livrables


| Livrable         | Description                                                                                                         |
| ---------------- | ------------------------------------------------------------------------------------------------------------------- |
| Dossiers         | `docs/tools/`, `docs/tools/MiyuSQL/`                                                                                |
| Doc fondatrice   | MiyuSQL comme kit `toolkit.data.miyusql`, composition, gouvernance, alignement MIP                                  |
| Référence outils | Tous les Tools (query, transaction, cache) avec id, action, niveau sécurité                                         |
| Index            | `_index.md` au niveau tools et au niveau MiyuSQL                                                                    |
| Protocoles       | Écriture conceptuelle (1 doc = 1 tâche, ordre fondateur → référence → index) ; MIP (domain data, layer tool, blocs) |


Aucune modification de fichiers existants requise. Mise à jour optionnelle ultérieure : lien vers `docs/tools/` depuis README ou référence Tools et Toolkits (hors périmètre de ce plan).