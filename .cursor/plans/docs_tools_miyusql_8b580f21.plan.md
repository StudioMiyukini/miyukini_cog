---
name: Docs tools MiyuSQL
overview: Créer le dossier docs/tools/ et la documentation sur MiyuSQL en respectant le protocole d'écriture documentation conceptuelle (structure, Contexte, Portée, nomenclature des tâches) et l'alignement MIP (domaines, couches, structure indexable).
todos: []
isProject: false
---

# Plan : dossier docs/tools/ et documentation MiyuSQL

## Objectif

- Créer le dossier **docs/tools/** (absent actuellement de [docs/](docs/)).
- Rédiger la **documentation sur MiyuSQL** (kit d'outils de gestion de données DB) en appliquant :
  - **[Protocole Écriture Documentation Conceptuelle](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)** : cycle planification → distribution → vérification → gel ; document fondateur ; nomenclature des tâches [xx] - [document] ; 1 agent = 1 document.
  - **[Protocole MIP v1](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)** : structure et terminologie alignées avec le modèle d'index (domaines, layers, blocs) pour que la doc et le futur code MiyuSQL soient indexables (blocks.json, domains.json, layers.json).

---

## 1. Conformité aux protocoles

### 1.1 Protocole Écriture Documentation Conceptuelle

- **Phase 1 — Planification** : ce plan constitue la planification ; titre d'étape = "Planification documentation MiyuSQL".
- **Phase 2 — Distribution** : chaque fichier = une tâche avec nomenclature **[xx] - [nom du document]** ; 1 agent = 1 document ; pas de batch multi-documents. Préfixes proposés : [01], [02] (2 documents, donc ≤ 4 par groupe).
- **Phase 3 — Vérification** : après rédaction, vérification des liens, cohérence avec [Miyukini Conceptual References - Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) et contrats Master Butler.
- **Phase 4 — Gel** : optionnel pour ce plan ; version 1.0 et statut document de référence sur le document fondateur MiyuSQL.

### 1.2 Protocole MIP v1

- La documentation **ne génère pas** les fichiers `mscm_index/*` (MIP indexe le **code** balisé MSCM).
- La doc MiyuSQL doit **définir les concepts** de façon compatible MIP pour une future implémentation :
  - **Domaine** : `data` (cohérent avec domains.json : blocs "data").
  - **Layer** : outil / toolkit (Strate 6), à refléter dans layers.json quand le code existera.
  - **Blocs** : chaque Tool MiyuSQL = unité logique pouvant devenir un bloc MSCM (id, do, role, layer) dans le futur code.
- Inclure dans le document fondateur MiyuSQL une courte section **"Alignement MIP"** (ou sous-section dans Références) : préciser que les Tools et le Toolkit MiyuSQL sont conçus pour être balisés MSCM à l'implémentation (role, layer, domain) et alimenter blocks.json, domains.json, layers.json selon [MIP v1](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 2. Création du dossier et liste des fichiers


| Action | Chemin                                                                                                     |
| ------ | ---------------------------------------------------------------------------------------------------------- |
| Créer  | `docs/tools/` (dossier)                                                                                    |
| Créer  | [docs/tools/_index.md](docs/tools/_index.md)                                                               |
| Créer  | [docs/tools/MiyuSQL - Documentation Fondatrice.md](docs/tools/MiyuSQL%20-%20Documentation%20Fondatrice.md) |


Aucun sous-dossier `docs/tools/MiyuSQL/` pour rester minimal ; l'index permettra d'ajouter d'autres kits plus tard.

---

## 3. Nomenclature des tâches (protocole Écriture)


| Préfixe | Tâche                                   | Document à produire                                |
| ------- | --------------------------------------- | -------------------------------------------------- |
| [01]    | [01] - Index de navigation tools        | `docs/tools/_index.md`                             |
| [02]    | [02] - Documentation Fondatrice MiyuSQL | `docs/tools/MiyuSQL - Documentation Fondatrice.md` |


Règle : 1 agent = 1 document. Les tâches [01] et [02] peuvent être distribuées en parallèle (2 ≤ 4).

---

## 4. Contenu de `docs/tools/_index.md`

- **Titre H1** : Tools — Index de navigation (ou équivalent).
- **Contexte** : rôle du dossier `docs/tools/` — documentation des Kits d'Outils (Strate 6) et des outils individuels ; lien vers [Miyukini Conceptual References - Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) et Master Butler.
- **Portée** : liste des kits documentés (au minimum MiyuSQL).
- **Structure** : tableau des documents avec description et lien vers MiyuSQL - Documentation Fondatrice.
- **Références** : liens vers Tools et Toolkits, [Master Butler - Tool Governance Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md), [Master Butler - Toolkit Composition Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md).

Format aligné sur [docs/kernel/_index.md](docs/kernel/_index.md) : Contexte, structure en tableaux, liens relatifs. Nomenclature fichier : pas d'accents (Fondatrice OK).

---

## 5. Contenu de `docs/tools/MiyuSQL - Documentation Fondatrice.md`

Document **fondateur** unique pour MiyuSQL. Nomenclature : préfixe **MiyuSQL**, sujet **Documentation Fondatrice**, sans accent.

### 5.1 Sections obligatoires (règle projet + protocole)


| Section                            | Contenu                                                                                                                                                                                                                                                                                                                                                                           |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Titre H1**                       | MiyuSQL — Documentation Fondatrice                                                                                                                                                                                                                                                                                                                                                |
| **1. Contexte**                    | MiyuSQL = kit d'outils (Toolkit) de gestion de données DB ; primordial, très utilisé ; intègre les outils de manipulation de données en base ; autorité données = KindMother.                                                                                                                                                                                                     |
| **2. Portée / Scope**              | Ce qui est défini (identité, ToolkitId, Tools composants, gouvernance) ; hors scope (implémentation détaillée, logique métier).                                                                                                                                                                                                                                                   |
| **3. Définition canonique**        | MiyuSQL = composition officielle d'outils de manipulation de données DB, déclarée et gouvernée ; pas un nouveau Tool, pas de logique métier.                                                                                                                                                                                                                                      |
| **4. Identifiant et catalogue**    | **ToolkitId** : `toolkit.data.miyusql` (format Master Butler `toolkit.<domain>.<name>`). Référence au Tool Governance Contract.                                                                                                                                                                                                                                                   |
| **5. Outils composants**           | Liste des Tools (format `tool.<domain>.<action>`), ex. `tool.query.execute`, `tool.query.prepare`, `tool.transaction.begin`, `tool.transaction.commit`, `tool.transaction.rollback`, `tool.cache.get`, `tool.cache.set`, `tool.cache.invalidate`. Au moins 2 Tools (invariant Toolkit Composition). Pour chaque outil : id, action en une phrase, niveau de sécurité typique (2). |
| **6. Gouvernance**                 | Flux : Opérateur → BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather → exécution. KindMother = autorité sur les données. Référence au schéma de flux dans [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md).                                                                                       |
| **7. Niveau de sécurité et états** | Niveau **2** (données utilisateur). États autorisés / interdits (ex. HEALTHY, DEGRADED autorisés ; SECURITY_LOCKDOWN, MAINTENANCE interdits) — aligné Toolkit Composition Contract.                                                                                                                                                                                               |
| **8. Relation avec KindMother**    | Opérations DB sous autorité KindMother (persistance, WriteIntent côté métier). MiyuSQL expose capacités d'exécution gouvernée sans remplacer KindMother. Lien vers [Accès DB et Droits Agents IA](docs/reference/Miyukini%20Conceptual%20References%20-%20Acces%20DB%20et%20Droits%20Agents%20IA.md) si pertinent.                                                                |
| **9. Alignement MIP**              | Les Tools et le Toolkit MiyuSQL sont conçus pour être balisés MSCM à l'implémentation : domain `data`, layer outil/toolkit (Strate 6), chaque Tool = unité logique pour blocks.json (id, do, role, layer). Référence [MIP v1](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).                                                           |
| **10. Références croisées**        | Glossaire, Tools et Toolkits, Tool Governance Contract, Toolkit Composition Contract, KindMother.                                                                                                                                                                                                                                                                                 |
| **Pied de document**               | Date de création, version (ex. 1.0), statut (Document de référence / Fondateur).                                                                                                                                                                                                                                                                                                  |


### 5.2 Cohérence contractuelle

- **ToolId** : `tool.<domain>.<action>[.<qualifier>]` (ex. `tool.query.execute`).
- **ToolkitId** : `toolkit.data.miyusql`.
- Invariant : un Toolkit n'ajoute aucune capacité nouvelle.
- Security level du kit : 2 (max des Tools composants).

---

## 6. Fichiers impactés (résumé)


| Action | Fichier                                            |
| ------ | -------------------------------------------------- |
| Créer  | `docs/tools/`                                      |
| Créer  | `docs/tools/_index.md`                             |
| Créer  | `docs/tools/MiyuSQL - Documentation Fondatrice.md` |


Aucune modification de fichiers existants requise. Mise à jour optionnelle ultérieure : lien vers `docs/tools/` depuis README ou référence Tools et Toolkits (hors périmètre de ce plan).

---

## 7. Ordre d'exécution recommandé

1. Créer le dossier `docs/tools/`.
2. Rédiger `_index.md` (tâche [01]) puis `MiyuSQL - Documentation Fondatrice.md` (tâche [02]) — ou en parallèle si deux agents.
3. Vérifier liens et cohérence avec Tools et Toolkits + contrats Master Butler.
4. (Optionnel) Gel / versionnement du document fondateur (v1.0).

