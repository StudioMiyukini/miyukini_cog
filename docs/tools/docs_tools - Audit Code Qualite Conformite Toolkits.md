# docs_tools — Audit code, qualité et conformité des Toolkits (crates)

**Version :** 1.0  
**Statut :** Rapport d'audit  
**Date :** 2026-01-31  
**Références :** [docs_tools - Verification Pret Implementation Bornes](./docs_tools%20-%20Verification%20Pret%20Implementation%20Bornes.md), [docs_tools - Audit Qualite Conformite Securite Implementation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md), [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md), [Miyukini Protocol - Ecriture Enrichie Toolkits](../protocols/Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md)

---

## 1. Contexte

Ce document audite le **code source des crates Toolkits** (crates `miyu*` du workspace) : qualité du code, respect des protocoles (MIP, BOUND-*), bon fonctionnement (compilation, invariants) et déviances éventuelles.

**Périmètre :** crates sous `crates/` de type toolkit (miyauth, miyucalc, miyutext, …), hors Cores et miyukini-admin / miyukini-central / miyukini-kernel. Les outils `tools/mip-generator`, `tools/toolkit-registry-export`, `tools/toolkit-skeleton` sont inclus pour cohérence du pipeline.

---

## 2. Synthèse exécutive

| Critère | État | Commentaire |
|--------|------|-------------|
| **Compilation** | OK | `cargo check --workspace` réussit. |
| **Unsafe** | Conforme | Aucun bloc `unsafe` dans les crates. |
| **unsafe_code = "forbid"** | Conforme | Appliqué dans l’ensemble des crates toolkits et Cores (51+ Cargo.toml). |
| **Gouvernance (mandat)** | Bon | 1205+ usages `has_mandate` / `GovernedContext` dans 293 fichiers. |
| **Balisage MSCM / MIP** | Bon | 2588+ marqueurs @id / @do (ou équivalent) dans 304 fichiers ; index MIP à jour. |
| **BOUND-1 (pas de décision ALLOW/DENY)** | Conforme | Aucun toolkit ne décide ALLOW/DENY ; mentions uniquement en commentaires ou dans Cores (StrongFather, BorderGuard). |
| **unwrap / expect** | À surveiller | 1 occurrence en code toolkit production (miyufeeds : `expect` sur `duration_since(UNIX_EPOCH)` — justifié et documenté). Autres occurrences limitées aux tests, kernel, admin, Cores. |
| **Stubs Unimplemented** | En cours | 432 occurrences dans 206 fichiers ; nombreux toolkits avec logique partielle ou stub. |

---

## 3. Métriques

### 3.1 Workspace et index

| Métrique | Valeur |
|----------|--------|
| Crates toolkits (miyu*) | 49 |
| Fichiers Rust (crates) | 487 (index MIP) |
| Blocs MSCM (blocks) | 1302 |
| Domaines (domains) | 65 |
| Layers | 5 |
| Rôles | 17 |
| Intégrité MIP | ok (registry.json) |

### 3.2 Qualité et conformité code

| Métrique | Valeur |
|----------|--------|
| Occurrences `unsafe` | 0 |
| Crates avec `unsafe_code = "forbid"` | 51+ (dont tous les toolkits) |
| Références `has_mandate` / `GovernedContext` | 1205 (293 fichiers) |
| Références MSCM (@id / @do ou équivalent) | 2588 (304 fichiers) |
| Occurrences `Unimplemented` / `unimplemented!` | 432 (206 fichiers) |
| `unwrap` / `expect` en production toolkits | 1 (miyufeeds — `expect` sur temps système, documenté) |

### 3.3 Conformité BOUND-*

| Borne | Vérification | Résultat |
|-------|--------------|----------|
| BOUND-1 | Pas de décision ALLOW/DENY dans les toolkits | Conforme (décision = StrongFather) |
| BOUND-2 | Pas de choix métier dans les Tools | Conforme (exécution sur données/paramètres fournis) |
| BOUND-3 | Pas d’accès direct non gouverné | Conforme (WriteIntent / pas de persistance métier directe dans les Tools) |
| BOUND-4 | Pas de modification du contexte d’autorisation | Conforme (lecture seule du contexte) |
| BOUND-5 | Pas de connaissance de l’Opérateur appelant | Conforme (contexte anonymisé) |
| BOUND-6 | Pas de capacité nouvelle | Conforme (uniquement ToolIds déclarés) |

---

## 4. Déviances identifiées et corrections

### 4.1 Corrigé lors de l’audit

- **miyufeeds/feed.rs** : `unwrap()` sur `SystemTime::now().duration_since(UNIX_EPOCH)` remplacé par `expect("system time before UNIX_EPOCH")` pour traçabilité et cohérence avec la politique d’évitement de `unwrap` en production.

### 4.2 À surveiller (hors toolkits)

- **miyukini-admin** : `unwrap()` sur `rows.get_mut(idx - 1)` (UI) ; `expect("bind")` / `expect("serve")` au démarrage — acceptables pour l’admin et le démarrage réseau.
- **kindmother/storage.rs** : `unwrap()` dans des tests ou scénarios de persistance — à confiner aux tests si possible.
- **strongfather/policy_engine.rs** : `unwrap()` dans des scénarios de politique — vérifier qu’ils sont en contexte test ou initialisation.

Les **tests** (miyusql, miyukini-kernel, etc.) et la **doc** (exemples dans id.rs, time.rs) peuvent conserver `expect`/`unwrap` pour la clarté.

---

## 5. Bon fonctionnement

- **Compilation** : `cargo check --workspace` exécuté avec succès (sortie 0).
- **Index MIP** : `mscm_index/registry.json` et `stats.json` cohérents ; `toolkit_registry.json` alimenté par `toolkit-registry-export`.
- **Structure des toolkits** : pattern répété `admin_cell`, `context` (GovernedContext), `errors`, modules par domaine (ex. calc : expression, number, round, unit) — cohérent avec le protocole et le plan d’implémentation.

---

## 6. Axes d’amélioration

### 6.1 Priorité haute

1. **Réduire les stubs Unimplemented**  
   Poursuivre l’implémentation de la logique réelle pour les toolkits encore en stub (432 occurrences), en priorisant les lots 7–9 du plan d’implémentation et les kits à fort impact (commerce, compta, contenu, identité).

2. **Réduire unwrap/expect en production**  
   Pour tout nouveau code toolkit : privilégier `map_err`, `?` et types `Result` ; réserver `expect` aux cas documentés (ex. invariant temps système dans miyufeeds).

### 6.2 Priorité moyenne

3. **Reference Implementation Guidelines pour les kits « Avec précautions »**  
   28 kits sans bornes explicites (cf. [docs_tools - Verification Pret Implementation Bornes](./docs_tools%20-%20Verification%20Pret%20Implementation%20Bornes.md)) : ajouter un guide minimal (BOUND-* + gestion d’erreurs + traçabilité) pour limiter les déviances lors des prochaines implémentations.

4. **Documentation des expect restants**  
   Pour chaque `expect` en code production (hors tests/doc), ajouter un commentaire ou message explicite (comme pour miyufeeds) pour justifier l’invariant.

5. **Uniformiser unsafe_code = "forbid"**  
   Vérifier que toute nouvelle crate toolkit et tout outil sous `tools/` déclare `unsafe_code = "forbid"` dans son `Cargo.toml`.

### 6.3 Priorité basse

6. **Couverture de tests**  
   Étendre les tests unitaires et de cycle (ex. miyusql) aux toolkits ayant une logique métier réelle (miyucalc, miyutext, miyufeeds, miyujobs, etc.).

7. **Revue des unwrap dans Cores**  
   Dans kindmother, strongfather, etc., confiner les `unwrap` aux tests ou les remplacer par une remontée d’erreur si le code est sur un chemin de production.

---

## 7. Références

| Document | Lien |
|----------|------|
| Vérification prêt implémentation (bornes) | [docs_tools - Verification Pret Implementation Bornes](./docs_tools%20-%20Verification%20Pret%20Implementation%20Bornes.md) |
| Audit qualité conformité sécurité | [docs_tools - Audit Qualite Conformite Securite Implementation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md) |
| Template Reference Implementation Guidelines | [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| Protocole Ecriture Enrichie Toolkits | [Miyukini Protocol - Ecriture Enrichie Toolkits](../protocols/Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md) |
| Index docs/tools | [_index](./_index.md) |

---

**Date de création :** 2026-01-31  
**Statut :** Rapport d’audit — à mettre à jour après corrections ou évolution des métriques.
