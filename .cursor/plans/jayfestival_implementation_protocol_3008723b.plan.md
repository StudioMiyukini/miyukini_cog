---
name: JayFestival Implementation Protocol
overview: Planifier l'implémentation de JayFestival en alignant le plan existant (12 phases, tâches [01] à [213]) sur le protocole d'implémentation générale (cycle Planification → Distribution → Vérification → Gel) et le protocole MIP v1 (MSCM, index, intégrité), sans modifier le code.
todos:
  - id: phase1
    content: Phase 1 — Fondations [01] Crate et config, [02] Thème, [03] Main et boucle app, [04] Constantes écrans
    status: completed
  - id: phase2-atoms
    content: Phase 2 — Atoms [11] IconWrapper à [24] mod atoms
    status: completed
  - id: phase2-molecules
    content: Phase 2 — Molecules [31] FeatureCard à [42] mod molecules
    status: completed
  - id: phase3
    content: Phase 3 — Organisms et Layout [51] Header à [72] mod organisms
    status: completed
  - id: phase4
    content: Phase 4 — Supabase et Auth [81]–[84]
    status: completed
  - id: phase5
    content: Phase 5 — Écrans UNC [91]–[115]
    status: completed
  - id: phase6
    content: Phase 6 — JayXpose [121]–[122]
    status: completed
  - id: phase7
    content: Phase 7 — Écrans ORG [131]–[162]
    status: completed
  - id: phase8
    content: Phase 8 — Écrans EXP [171]–[175]
    status: completed
  - id: phase9
    content: Phase 9 — Écrans VIS [181]–[184]
    status: completed
  - id: phase10
    content: Phase 10 — Intégrations [191]–[196]
    status: completed
  - id: phase11
    content: Phase 11 — Vérification [201]–[204]
    status: completed
  - id: phase12
    content: Phase 12 — Gel [211]–[213]
    status: pending
isProject: false
---

# Plan d'implémentation JayFestival — Conformité protocoles

## 1. Alignement avec les protocoles

Le [JayFestival - Plan Implementation](docs/services/JayFestival/JayFestival%20-%20Plan%20Implementation.md) est déjà structuré en 12 phases. Ce plan formalise son **exécution** selon :

- **[Miyukini Prompt Protocol - Implémentation générale](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20Implémentation%20générale.md)** : cycle en 4 phases (Planification, Distribution, Vérification, Gel), 1 étape = 1 fichier, 1 agent = 1 tâche, max 4 agents simultanés, balisage MSCM obligatoire.
- **[Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)** : index dans `mscm_index/` (reconstruit uniquement par pipeline, jamais modifié à la main), pipeline Scan → Parse MSCM → Extraction blocs → Génération index.

**Correspondance cycle global ↔ phases JayFestival :**


| Cycle protocole             | Phases JayFestival                     | Rôle                                                                                                    |
| --------------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **Phase 1 — Planification** | Phase 0 (0.1–0.7) + détail Phases 1–12 | Objectif, périmètre, nomenclature [xx]-[fichier], blocs MSCM par tâche, ordre et parallélisme           |
| **Phase 2 — Distribution**  | Phases 1 à 10 (tâches [01]–[196])      | Délégation tâche par tâche, contexte vierge, 1 fichier par tâche, max 4 tâches en parallèle par préfixe |
| **Phase 3 — Vérification**  | Phase 11 ([201]–[204])                 | Vérification globale, tests, conformité MSCM, **régénération MIP**                                      |
| **Phase 4 — Gel**           | Phase 12 ([211]–[213])                 | Document de gel, **index MIP final**, version explicite                                                 |


---

## 2. Règles d'exécution par protocole

### 2.1 Protocole Implémentation générale

- **Nomenclature des tâches** : `[xx] - [nom du fichier à produire]` (ex. `[01] - Crate et config`). Aucun groupe de préfixe ne doit contenir plus de **4 tâches** pour exécution parallèle.
- **Distribution** : Chaque tâche est déléguée avec **contexte vierge** ; pas de batch (1 agent = 1 fichier).
- **Entête de prompt obligatoire** (à inclure dans tout prompt de délégation) :
  ```
  COMPLEXITÉ : Simple | Complexe | Extreme
  CHARGE CONTEXTUELLE : Faible | Moyenne | Élevée
  MODÈLE AUTORISÉ : (selon phase, cf. plan § 0.3)
  MODE IA ACTIF : AI Mode 1 | AI Mode 2
  ```
- **Cadre de travail** : Documentation autorisée = liste fermée (JayFestival, JayXpose, publics, protocoles) ; **outils interdits** : modification manuelle de `mscm_index/`.
- **Arrêt strict** : Arrêt immédiat si ambiguïté bloquante, dépendance manquante ou contexte insuffisant ; pas de fichier partiel.
- **Corrections** : Toute correction = nouvelle tâche (nomenclature [xx]-[fichier]), Phase 2 du protocole s’applique.

### 2.2 Protocole MIP v1

- **Balisage MSCM minimal par bloc** : `@id` (unique global), `@do` (description fonctionnelle). Optionnel : `@role`, `@layer`, `@human`.
- **Couches utilisées dans le plan** : `infra` | `ui` | `app` | `domain` (alignées avec les blocs attendus dans le plan JayFestival).
- **Index** : Répertoire [mscm_index/](mscm_index/) — **généré uniquement** par le pipeline (outil [tools/mip-generator](tools/mip-generator/)). Ne jamais éditer manuellement les JSON.
- **Régénération MIP** : À exécuter en **Phase 11** (tâche [204]) et en **Phase 12** (tâche [212]). Commande typique depuis la racine : `cargo run --manifest-path tools/mip-generator/Cargo.toml` (ou script équivalent).
- **Avant gel** : Vérifier `mscm_index/registry.json` → `integrity: "ok"`, pas de bloc orphelin, pas de cycle invalide (règles d’intégrité MIP § 8).

---

## 3. Checkpoints de conformité

### Avant de lancer une tâche (Phase 2 — Distribution)

- La tâche est identifiée par son id [xx] et le fichier à produire.
- Les dépendances listées dans le plan sont satisfaites (fichiers déjà livrés).
- Le prompt de délégation contient : entête complexité/modèle, cadre de travail, liste fermée de docs, **blocs MSCM attendus** (id, do, layer) pour ce fichier.
- Aucun autre agent ne travaille sur une tâche du même groupe de 4 si la limite est atteinte.

### À la livraison de chaque fichier

- Fichier unique livré (pas de fusion avec une autre étape).
- Chaque bloc fonctionnel a au moins `@id` et `@do` ; `@layer` conforme au plan.
- Pas de correction hors périmètre de la tâche.

### Phase 11 — Vérification (avant gel)

- **[201]** Vérification globale : incohérences, non-conformité docs, violations PROTO-1 à PROTO-8 (Specification UI).
- **[202]** Tests : `cargo test` pour domain/auth/supabase/services ; justification si pas de tests (ex. UI pure egui).
- **[203]** Conformité MSCM : tous les blocs avec @id, @do, @layer ; pas de bloc orphelin (checklist § 5.4 protocole Implémentation).
- **[204]** **Régénération MIP** : lancer le pipeline MIP ; vérifier que `mscm_index/` est à jour et `registry.json` cohérent.

### Phase 12 — Gel

- **[211]** Rédiger le document de gel : `docs/services/JayFestival/JayFestival - Gel Implementation Alpha vX.Y.Z.md` (liste exhaustive des éléments gelés).
- **[212]** Index MIP final : régénération + archivage ; vérification `integrity: "ok"`.
- **[213]** Version : attribution (ex. v0.1.0-alpha), règles d’évolution, conditions de dégel ; mention dans le document de gel + tag git si applicable.

---

## 4. Ordre d’exécution et parallélisme

Les **phases** doivent être exécutées dans l’ordre (Phase 1 → … → Phase 12). Au sein d’une phase :

- **Phase 1** : [01] d’abord ; puis [02], [03], [04] en parallèle (max 3 agents).
- **Phase 2** : [11] puis [12],[13],[14] en parallèle ; puis [21]–[23], [24] ; puis [31]–[34], [41],[42].
- **Phases 3–10** : Respecter les dépendances du plan (ex. organisms après molecules, écrans après layout).
- **Phase 11** : [201] → [202] → [203] → [204] (ordre recommandé : vérification puis tests puis MSCM puis MIP).
- **Phase 12** : [211] puis [212] puis [213].

Aucun groupe de préfixe ne dépasse 4 tâches en parallèle (conformité protocole).

---

## 5. Références techniques

- **Génération MIP** : [tools/mip-generator](tools/mip-generator/) — scan des `crates/*/src` et `tools/*/src`, parsing des blocs MSCM (regex), génération de `mscm_index/*.json`. La future crate `crates/jayfestival/` sera prise en compte dès qu’elle existera.
- **Checklist conformité** : [Miyukini COG vers. 0.1.0 - MSCM MIP Compliance Checklist](docs/implementation/Miyukini%20COG%200.1%20-%20MSCM%20MIP%20Compliance%20Checklist.md) (critères MSCM/MIP réutilisables pour JayFestival).
- **Todo list détaillée** : Voir section 5.1 ci-dessous.

---

## 5.1 Todo list d’implémentation

Les tâches sont à exécuter **dans l’ordre des phases** ; au sein d’une phase, parallélisation selon les dépendances (max 4 agents simultanés par groupe de préfixe).

### Phase 1 — Fondations

- [01] Crate et config
- [02] Thème
- [03] Main et boucle app
- [04] Constantes écrans

### Phase 2 — Atoms

- [11] IconWrapper
- [12] Button
- [13] Input
- [14] Label
- [21] Badge
- [22] Checkbox
- [23] Select
- [24] mod atoms

### Phase 2 — Molecules

- [31] FeatureCard
- [32] DirectoryCard
- [33] RoleCard
- [34] CTACard
- [41] Card
- [42] mod molecules

### Phase 3 — Organisms et Layout

- [51] Header
- [52] HeaderWithEdition
- [53] HeroSection
- [54] FeaturesGrid
- [61] DirectoryBanner
- [62] RolesGrid
- [63] CTASection
- [64] Layout
- [71] GestionLayout
- [72] mod organisms

### Phase 4 — Supabase et Auth

- [81] Client Supabase
- [82] Types Supabase
- [83] Auth service
- [84] RLS et permissions

### Phase 5 — Écrans UNC

- [91] UNC-E01 Landing
- [92] UNC-E02 Liste événements
- [93] UNC-E03 Fiche événement
- [94] UNC-E06 Liste organisateurs
- [101] UNC-E07 Fiche organisateur
- [102] UNC-E08 Liste exposants
- [103] UNC-E09 Fiche exposant
- [104] UNC-E10 Recherche
- [111] UNC-E11 CTA contextuels
- [112] UNC-E12 Connexion
- [113] UNC-E13 Inscription
- [114] UNC-E14 Mentions
- [115] Router UNC

### Phase 6 — JayXpose

- [121] JayXpose client
- [122] Contrat JayXpose

### Phase 7 — Écrans ORG

- [131] ORG-E04 Dashboard
- [132] ORG-E05 Liste éditions
- [133] ORG-E06 Création édition
- [134] ORG-E07 Dashboard édition
- [141] ORG-E08 Liste exposants
- [142] ORG-E09 Candidatures
- [143] ORG-E10 Fiche exposant (org)
- [144] ORG-E11 Plan de salle
- [151] ORG-E12 Programme
- [152] ORG-E13 Budget
- [153] ORG-E14 Devis / Factures
- [154] ORG-E15 Documents
- [161] ORG-E16–E25 (reste)
- [162] Router ORG

### Phase 8 — Écrans EXP

- [171] EXP-E04 Dashboard exposant
- [172] EXP-E05 Candidatures
- [173] EXP-E06 Participations
- [174] EXP-E07–E19 (reste)
- [175] Router EXP

### Phase 9 — Écrans VIS

- [181] VIS-E04 Dashboard visiteur
- [182] VIS-E05 Agenda
- [183] VIS-E06–E15 (billets, réservations, pass)
- [184] Router VIS

### Phase 10 — Intégrations

- [191] Adapter JayKoa
- [192] Adapter JayKonta / Miyuinvoice
- [193] Adapter Miyunotify
- [194] Adapter Miyubooking
- [195] Adapter MiyuClock
- [196] Router global et état app

### Phase 11 — Vérification

- [201] Vérification globale
- [202] Tests unitaires
- [203] Conformité MSCM
- [204] Régénération MIP

### Phase 12 — Gel

- [211] Document de gel
- [212] Index MIP final
- [213] Version

---

## 6. Synthèse

- **Planification** : Déjà en place (Phase 0 + détail des 12 phases avec blocs MSCM attendus).
- **Distribution** : Exécuter les tâches une par une avec entête de prompt, cadre de travail et blocs MSCM attendus ; max 4 agents par groupe de préfixe.
- **Vérification** : Phase 11 avec vérification globale, tests, conformité MSCM, puis **régénération MIP** (tâche [204]).
- **Gel** : Phase 12 avec document de gel, **index MIP final** (tâche [212]), version et intégrité.

En suivant ce plan, l’implémentation JayFestival respecte à la fois le protocole d’implémentation générale et le protocole MIP v1 MSCM Index Protocol.