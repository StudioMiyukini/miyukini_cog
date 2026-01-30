# MiyuAuth — Audit de la documentation

## Contexte

Ce document constitue l'**audit de la documentation MiyuAuth** au regard des références [docs/reference](../../reference) et du modèle [docs/tools/MiyuSQL](../../MiyuSQL). Il identifie les points conformes et les améliorations possibles.

**Date d'audit :** 2026-01-30  
**Références utilisées :** Glossaire, Tools et Toolkits, Connexion Inter-COG, Security Levels, Standardisation Numération Invariants, MiyuSQL (structure et contrats).

---

## 1. Conformité déjà acquise

| Domaine | État | Détail |
|--------|------|--------|
| **Structure** | ✅ | Arborescence alignée sur MiyuSQL : Fondatrice, Reference Outils, _index, contracts (governance, boundaries, security, integration, testing), dependencies, implementation. |
| **Terminologie** | ✅ | Glossaire respecté : Passeport Utilisateur, Visa de Connexion, COG Hébergeur, COG Origine, Utilisateur Visiteur, Utilisateur Externe, citoyen. |
| **Flux de gouvernance** | ✅ | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather → Exécution ; cohérent avec Tools et Toolkits (MiyuAuth explicite StrongFather, la référence schématise sans le détailler). |
| **Relation KindMother** | ✅ | KindMother = validateur unique de la confiance ; MiyuAuth exécute sans décider ; invariants INV-KM-* clairs. |
| **Relation MiyuSQL** | ✅ | Section 8bis Documentation Fondatrice + 6bis KindMother Integration : persistance = KindMother + MiyuSQL ; MiyuAuth opère sur données fournies. |
| **ToolkitId / ToolIds** | ✅ | Format `toolkit.identity.miyauth`, `tool.identity.<action>` conforme Master Butler. |
| **Sécurité et états** | ✅ | Niveau 2 ou 3, HEALTHY/DEGRADED autorisés, SECURITY_LOCKDOWN/MAINTENANCE interdits ; alignement WorrySentinel et Caring Nanny. |
| **MIP** | ✅ | Domaine `identity`, layer Strate 6, blocs futurs (id, do, role, layer). |

---

## 2. Améliorations possibles

### 2.1 Références documentaires manquantes ou à renforcer

| Référence | Où l’ajouter | Raison |
|-----------|--------------|--------|
| **Security Levels** ([Miyukini Conceptual References - Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)) | Security and States Contract, section Références croisées | Référence conceptuelle des niveaux 0–4 ; justification explicite du niveau 2 ou 3 (Niveau 2 = Sensitive Data, Niveau 3 = Critical System / Auth). |
| **Ever Buddy** (Tools et Toolkits § 5.2, § 6) | Documentation Fondatrice § 6 ou Dependencies Contract | Ever Buddy gère cycle de vie, versions et compatibilité des Outils ; MiyuAuth est déclaré et compatibilisé par l’environnement (Toolkit Composition). |
| **Standardisation Numération Invariants** | _index ou Implementation Guidelines | Documenter les préfixes d’invariants MiyuAuth (INV-BOUND-*, INV-DEP-*, INV-KM-*, INV-SEC-*, etc.) et leur lien éventuel au standard (format INV-&lt;PREFIX&gt;-&lt;NUMERO&gt;). Les contrats MiyuAuth utilisent des préfixes catégoriels (BOUND, DEP, SEC) ; le standard couvre les Cores ; option : ajouter une note dans _index ou Dependencies. |
| **Souveraineté Environnement / Pyramide** | Références croisées Documentation Fondatrice ou _index | Contexte Strate 6 et souveraineté applicative (bibliothèque d’outils gouvernée). |
| **Connexion Inter-COG — champs Passeport / Visa** | Reference Outils (§ 4.3, 4.4) | Rappeler que les champs des artefacts (ex. Passeport § 3.1, Visa § 3.3 et niveaux S1–S5) sont ceux sur lesquels `verify` / `role` opèrent, sans décider de l’autorisation. |

### 2.2 Contenu à préciser

| Sujet | Suggestion |
|-------|------------|
| **Niveaux Visa (S1–S5)** | Dans Reference Outils (tool.identity.verify) ou Security Contract : préciser que la vérification peut porter sur le `security_level` (S1–S5) du Visa (Connexion Inter-COG § 4) sans que MiyuAuth décide de l’autorisation. |
| **Rôle « externe »** | Dans Reference Outils § 4.4 (tool.identity.role) : ajouter une phrase reliant « externe » à Utilisateur Externe et Mandat Public d’Accès (Connexion Inter-COG § 11), pour cohérence avec la référence. |
| **_index — description Fondation** | Dans _index, ligne « Documentation Fondatrice » : ajouter « relation MiyuSQL (données identité, Passeport, Visa) » pour refléter la section 8bis. |
| **Flux schématique** | Documentation Fondatrice § 6 : le flux liste bien StrongFather ; la référence Tools et Toolkits ne le montre pas dans le schéma ASCII. Conserver la formulation MiyuAuth (complète) ; pas de changement requis. |

### 2.3 Invariants et numérotation

| Élément | État | Recommandation |
|--------|------|----------------|
| **INV-KM-*** | Utilisé dans KindMother Integration | KM = KindMother dans le standard ; ici ce sont des invariants du *contrat* d’intégration. Pas d’ambiguïté si le contrat est lu en entier ; option : préfixe INV-MAUTH-KM-* pour distinguer « contrat MiyuAuth–KindMother » du core KindMother. |
| **INV-BOUND-*, INV-DEP-*, INV-SEC-*** | Préfixes catégoriels | Cohérents en interne ; le standard cible les Cores. Ajouter une courte note dans Runtime Boundary ou dans _index : « Invariants de bornage (BOUND), dépendances (DEP), sécurité (SEC) ; voir Standardisation Numération Invariants pour le format canonique des Cores. » |
| **INV-UT-MAUTH-*, INV-CT-MAUTH-*** | Tests | Clairs ; pas de changement requis. |

### 2.4 Alignement MiyuSQL (modèle)

| Aspect | MiyuSQL | MiyuAuth | Action |
|--------|---------|----------|--------|
| Référence « Acces DB et Droits Agents IA » | Oui (Fondatrice) | N/A (MiyuAuth ne touche pas à la DB) | — |
| Référence MiyukiniAdmin Cycle Tests | Oui (_index) | Mention « exécutable par MiyukiniAdmin » dans contrats | Option : ajouter dans _index un lien vers MiyukiniAdmin - Cycle Tests Contract si un test MiyuAuth y est défini. |
| Référence Security Levels | Non explicite dans MiyuSQL | Idem | Ajouter dans MiyuAuth Security and States Contract. |

---

## 3. Plan d’actions recommandé

### Priorité haute (références et cohérence)

1. **Security and States Contract** : ajouter en références croisées [Miyukini Conceptual References - Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md).
2. **_index** : compléter la description de la Documentation Fondatrice par « relation MiyuSQL (données identité, Passeport, Visa) ».
3. **Reference Outils** : dans § 4.4 (role), ajouter une phrase sur le rôle « externe » (Utilisateur Externe / Mandat Public d’Accès).

### Priorité moyenne (enrichissement)

4. **Documentation Fondatrice ou Dependencies Contract** : mentionner Ever Buddy (cycle de vie / compatibilité des Outils, conformément à Tools et Toolkits).
5. **Reference Outils** § 4.3 (verify) : préciser que la vérification peut porter sur les champs Passeport/Visa (dont `security_level` S1–S5) sans décider de l’autorisation.
6. **KindMother Integration Contract** : ajouter en référence [Miyukini Conceptual References - Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) si le contrat mentionne le niveau de sécurité.

### Priorité basse (optionnel)

7. **Standardisation Numération Invariants** : courte note dans _index ou Implementation Guidelines sur les préfixes d’invariants MiyuAuth.
8. **Références Souveraineté / Pyramide** : lien depuis Documentation Fondatrice ou _index pour ancrage architecture.
9. **MiyukiniAdmin - Cycle Tests Contract** : lien depuis _index MiyuAuth si un test MiyuAuth y est décrit.

---

## 3bis. État d'application des recommandations

| # | Recommandation | Statut |
|---|----------------|--------|
| 1 | Security and States Contract — référence Security Levels | ✅ Appliqué |
| 2 | _index — description Documentation Fondatrice (relation MiyuSQL) | ✅ Appliqué |
| 3 | Reference Outils § 4.4 — rôle « externe » (Utilisateur Externe / Mandat Public) | ✅ Appliqué |
| 4 | Documentation Fondatrice / Dependencies — Ever Buddy | ✅ Appliqué (Documentation Fondatrice § 6) |
| 5 | Reference Outils § 4.3 — verify, champs Passeport/Visa S1–S5 | ✅ Appliqué |
| 6 | KindMother Integration Contract — référence Security Levels | ✅ Appliqué |
| 7 | Standardisation Numération Invariants — note _index + Implementation Guidelines | ✅ Appliqué (_index Contexte ; Implementation Guidelines § 5.3 ; Runtime Boundary § 5) |
| 8 | Souveraineté / Pyramide — références Documentation Fondatrice et _index | ✅ Appliqué |
| 9 | MiyukiniAdmin - Cycle Tests Contract — lien _index | ✅ Appliqué |

---

## 4. Synthèse

La documentation MiyuAuth est **déjà alignée** avec les références (Glossaire, Tools et Toolkits, Connexion Inter-COG) et avec le modèle MiyuSQL (structure, contrats, relation KindMother, relation MiyuSQL). Les améliorations proposées portent sur :

- **Références explicites** : Security Levels, Ever Buddy, éventuellement Souveraineté / Pyramide.
- **Précisions de contenu** : rôle « externe », champs Passeport/Visa et niveaux S1–S5, description _index.
- **Numérotation des invariants** : note optionnelle pour lien avec le standard.

Les modifications de priorité haute peuvent être appliquées immédiatement ; les autres au fil des mises à jour.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document d’audit
