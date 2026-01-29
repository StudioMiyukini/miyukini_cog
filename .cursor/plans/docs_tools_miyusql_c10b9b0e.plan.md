---
name: Docs tools MiyuSQL
overview: Créer le dossier docs/tools/ et y rédiger la documentation fondatrice du kit d'outils MiyuSQL (gestion de données DB), alignée sur les contrats Master Butler et la référence Tools et Toolkits.
todos:
  - id: todo-1769685871259-ldcrtr73f
    content: ""
    status: pending
isProject: false
---

# Plan : dossier docs/tools/ et documentation MiyuSQL

## Objectif

- Créer le dossier **docs/tools/** (actuellement absent de [docs/](docs/)).
- Rédiger la **documentation sur MiyuSQL** : kit d'outils primordial de gestion de données DB, conforme aux contrats existants (Tool Governance, Toolkit Composition) et à la référence [Miyukini Conceptual References - Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md).

---

## 1. Création du dossier et structure

- **Créer** le dossier `docs/tools/`.
- **Créer** deux fichiers :
  - `docs/tools/_index.md` — index de navigation du dossier tools.
  - `docs/tools/MiyuSQL - Documentation Fondatrice.md` — document principal sur MiyuSQL.

Aucun autre sous-dossier n’est prévu pour l’instant (pas de `docs/tools/MiyuSQL/` séparé) afin de rester simple ; l’index permettra d’ajouter d’autres kits plus tard (storage.media, ui.standard, etc.).

---

## 2. Contenu de `docs/tools/_index.md`

- **Titre H1** : Tools — Index de navigation (ou équivalent).
- **Contexte** : rôle du dossier `docs/tools/` — documentation des Kits d’Outils (Strate 6) et des outils individuels ; lien vers la référence conceptuelle Tools et Toolkits et vers Master Butler.
- **Portée** : liste des kits documentés dans ce dossier (au minimum MiyuSQL).
- **Structure** : tableau des documents avec description (lien vers MiyuSQL - Documentation Fondatrice).
- **Références** : lien vers [Miyukini Conceptual References - Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Master Butler - Tool Governance Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md), [Master Butler - Toolkit Composition Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md).

Format et style alignés sur [docs/kernel/_index.md](docs/kernel/_index.md) (Contexte, structure en tableaux, liens relatifs).

---

## 3. Contenu de `docs/tools/MiyuSQL - Documentation Fondatrice.md`

Document unique dédié à MiyuSQL. Nomenclature : préfixe **MiyuSQL**, sujet **Documentation Fondatrice**, sans accent (Fondatrice). Sections obligatoires (règle projet) : **Contexte**, **Portée / Scope**.

### 3.1 Structure proposée


| Section                            | Contenu                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Titre H1**                       | MiyuSQL — Documentation Fondatrice                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| **1. Contexte**                    | MiyuSQL = kit d’outils (Toolkit) de gestion de données DB ; primordial, très utilisé ; intègre tous les outils de manipulation de données en base ; autorité données = KindMother.                                                                                                                                                                                                                                                                                                                                                                         |
| **2. Portée / Scope**              | Ce qui est défini (identité, ToolkitId, Tools composants, gouvernance) ; hors scope (implémentation détaillée, logique métier).                                                                                                                                                                                                                                                                                                                                                                                                                            |
| **3. Définition canonique**        | MiyuSQL = composition officielle d’outils de manipulation de données DB, déclarée et gouvernée ; pas un nouveau Tool, pas de logique métier.                                                                                                                                                                                                                                                                                                                                                                                                               |
| **4. Identifiant et catalogue**    | **ToolkitId** : `toolkit.data.miyusql` (format Master Butler : `toolkit.<domain>.<name>`). Référence au Tool Governance Contract (format canonique).                                                                                                                                                                                                                                                                                                                                                                                                       |
| **5. Outils composants**           | Liste des Tools du kit (format `tool.<domain>.<action>`), cohérente avec [Master Butler - Tool Governance Contract](docs/core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) (ex. `tool.query.execute`, `tool.query.prepare`, `tool.transaction.begin`, `tool.transaction.commit`, `tool.transaction.rollback`, `tool.cache.get`, `tool.cache.set`, `tool.cache.invalidate`). Au moins 2 Tools (invariant Toolkit Composition). Chaque outil : id, action en une phrase, niveau de sécurité typique (2 pour données). |
| **6. Gouvernance**                 | Flux d’appel : Opérateur → BondingBrother → Master Butler (catalogue) → WorrySentinel (niveau sécurité) → Caring Nanny (état) → StrongFather (décision) → exécution. KindMother = autorité sur les données ; les Tools n’exécutent pas de décision métier. Référence à [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) (schéma de flux).                                                                                                                                                             |
| **7. Niveau de sécurité et états** | Niveau de sécurité du kit : **2** (données utilisateur), cohérent avec WorrySentinel (Data Tools = 2). États autorisés / interdits : à préciser (ex. HEALTHY, DEGRADED autorisés ; SECURITY_LOCKDOWN, MAINTENANCE interdits) — aligné sur Toolkit Composition Contract.                                                                                                                                                                                                                                                                                    |
| **8. Relation avec KindMother**    | Les opérations DB sont sous autorité KindMother (persistance, WriteIntent côté métier). MiyuSQL expose les capacités d’exécution gouvernée (requête, transaction, cache) sans remplacer KindMother. Lien vers [Accès DB et Droits Agents IA](docs/reference/Miyukini%20Conceptual%20References%20-%20Acces%20DB%20et%20Droits%20Agents%20IA.md) si pertinent (distinction outillage vs WriteIntent).                                                                                                                                                       |
| **9. Références croisées**         | Liens vers : Glossaire, Tools et Toolkits, Tool Governance Contract, Toolkit Composition Contract, KindMother (index ou doc fondatrice).                                                                                                                                                                                                                                                                                                                                                                                                                   |
| **Pied de document**               | Date de création, version (ex. 1.0), statut (Document de référence / Fondateur).                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |


### 3.2 Points de cohérence contractuelle

- **ToolId** : format `tool.<domain>.<action>[.<qualifier>]` (ex. `tool.query.execute`).
- **ToolkitId** : format `toolkit.<domain>.<name>` → `toolkit.data.miyusql`.
- **Invariant** : un Toolkit n’ajoute aucune capacité nouvelle ; MiyuSQL orchestre des Tools existants.
- **Security level** : 2 pour le kit (max des Tools composants).
- Pas d’accents dans les noms de fichiers (déjà respecté : « Fondatrice »).

---

## 4. Fichiers impactés


| Action | Fichier                                            |
| ------ | -------------------------------------------------- |
| Créer  | `docs/tools/_index.md`                             |
| Créer  | `docs/tools/MiyuSQL - Documentation Fondatrice.md` |


Aucune modification de fichiers existants n’est requise pour ce plan. Une mise à jour optionnelle ultérieure : ajouter un lien vers `docs/tools/` depuis [README.md](README.md) ou depuis la référence [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) (section « Où trouver la documentation des kits ») — non incluse dans ce plan pour rester minimal.

---

## 5. Résumé

- Création du dossier **docs/tools/**.
- **Index** : `_index.md` avec Contexte, Portée, structure des docs, liens vers les contrats et la référence Tools et Toolkits.
- **MiyuSQL** : un seul document fondateur qui définit MiyuSQL comme le kit `toolkit.data.miyusql`, liste les Tools composants (query, transaction, cache), décrit la gouvernance et le niveau de sécurité, et référence KindMother et les contrats Master Butler.

