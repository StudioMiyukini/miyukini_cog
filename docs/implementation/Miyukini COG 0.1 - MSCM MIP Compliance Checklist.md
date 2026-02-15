# Miyukini COG vers. 0.1.0 — MSCM MIP Compliance Checklist

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contractuel — Obligatoire pour toute implémentation  
**Référence :** [MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md), [Protocole d'implémentation générale](../../protocols/Miyukini%20Prompt%20Protocol%20-%20Implémentation%20générale.md)

---

## Contexte

Cette check-list définit les **critères de conformité obligatoires** pour le balisage MSCM (Miyukini Semantic Code Markup) et l'indexation MIP (MSCM Index Protocol) dans le cadre de l'implémentation de Miyukini COG vers. 0.1.0.

**Principe fondamental :**
> La sémantique est dans le code.  
> La structure est dans l'index.  
> La gouvernance est dans le graphe.

**Règle absolue :**
👉 Tout code produit DOIT être conforme MSCM.  
👉 Tout projet DOIT maintenir un index MIP valide.  
👉 Un fichier sans balisage MSCM conforme est considéré comme **non livrable**.

---

## Portée / Scope

Cette check-list s'applique à :

- **Phase 1 :** Kernel (fondation)
- **Phase 2 :** Cores système (StrongFather, KindMother, BondingBrother, CaringNanny, MasterButler, BorderGuard, EverBuddy, WorrySentinel, TAMR, LogisticsSteward)
- **Phase 3 :** MiyukiniAdmin (opérateur souverain)

**Vérifications couvertes :**
- Conformité du balisage MSCM dans le code
- Intégrité de l'index MIP
- Cohérence structurelle et hiérarchique
- Validation des dépendances
- Critères de gel et versionnement

---

## 1. Vérifications MSCM — Balisage du Code

### 1.1 Obligations Minimales (OBLIGATOIRES)

Chaque bloc fonctionnel DOIT respecter les obligations suivantes :

- [ ] **@id** : Identifiant unique global présent
  - Format : `snake_case` ou `kebab-case`
  - Unicité vérifiée dans tout le codebase
  - Pas de duplication d'identifiants

- [ ] **@role** : Rôle sémantique explicite déclaré
  - Valeurs possibles : `security`, `data`, `logic`, `api`, `infra`, `domain`, `service`, etc.
  - Cohérence avec la documentation de référence
  - Rôle adapté à la fonction du bloc

- [ ] **@layer** : Couche architecturale déclarée
  - Valeurs possibles : `domain`, `infra`, `api`, `service`, `core`, `kernel`, etc.
  - Cohérence avec l'architecture définie
  - Pas de conflit de couche (un bloc ne peut être dans plusieurs couches)

- [ ] **@human** : Description humaine présente
  - Description claire et compréhensible
  - Explique le "quoi" et le "pourquoi" du bloc
  - Pas de description vide ou générique

### 1.2 Dépendances Inter-Blocs

- [ ] **Dépendances déclarées** : Toutes les dépendances inter-blocs sont explicitement déclarées
  - Format MSCM conforme
  - Dépendances réelles (pas de dépendances fantômes)
  - Pas de dépendances circulaires invalides

### 1.3 Couverture du Balisage

- [ ] **Blocs critiques balisés** : Tous les blocs critiques sont balisés MSCM
  - Fonctions publiques
  - Structures de données principales
  - Points d'entrée API
  - Gestionnaires d'événements
  - Logique métier complexe

- [ ] **Pas de bloc orphelin** : Aucun bloc fonctionnel sans `@id` ou `@role`
  - Tous les blocs identifiables sont balisés
  - Cohérence avec la documentation

### 1.4 Qualité du Balisage

- [ ] **Identifiants descriptifs** : Les identifiants `@id` sont descriptifs et cohérents
  - Nommage explicite (pas de `block1`, `func2`, etc.)
  - Conventions de nommage respectées
  - Lisibilité pour les humains

- [ ] **Rôles cohérents** : Les rôles `@role` sont cohérents avec la fonction réelle
  - Pas de rôle générique inapproprié
  - Alignement avec la documentation de référence

- [ ] **Couches cohérentes** : Les couches `@layer` respectent l'architecture définie
  - Pas de mélange de couches
  - Hiérarchie respectée (kernel < core < domain < infra)

---

## 2. Vérifications MIP — Index Structurel

### 2.1 Génération de l'Index

- [ ] **Index régénéré** : L'index MIP a été régénéré après chaque modification
  - Dernière génération récente
  - Pas de modification manuelle de l'index
  - Code source comme seule source de vérité

- [ ] **Génération sans erreur** : La génération de l'index MIP réussit sans erreur
  - Aucune erreur de parsing MSCM
  - Aucune erreur de construction du graphe
  - Aucune erreur de validation

### 2.2 Fichiers d'Index Requis

Tous les fichiers suivants DOIVENT être présents dans `mscm_index/` :

- [ ] **registry.json** : Métadonnées et intégrité
  - Version MIP déclarée (`mip_v1`)
  - Version MSCM déclarée (`v1`)
  - Timestamp de génération présent
  - Intégrité validée (`integrity: "ok"`)

- [ ] **blocks.json** : Identité sémantique des blocs
  - Tous les blocs MSCM présents
  - Métadonnées complètes (id, file, start_line, end_line, role, layer, do, human)
  - Pas de bloc manquant

- [ ] **hierarchy.json** : Structure hiérarchique
  - Hiérarchie cohérente
  - Pas de cycles invalides
  - Relations parent-enfant valides

- [ ] **graph.json** : Relations transverses
  - Graphe de dépendances complet
  - Relations bidirectionnelles cohérentes
  - Pas de nœuds isolés non justifiés

- [ ] **flows.json** : Processus métier
  - Flux identifiés et documentés
  - Ordre des étapes cohérent
  - Pas de flux incomplets

- [ ] **domains.json** : Vision métier
  - Domaines identifiés
  - Attribution des blocs aux domaines cohérente
  - Pas de domaine vide

- [ ] **layers.json** : Architecture technique
  - Couches identifiées
  - Attribution des blocs aux couches cohérente
  - Pas de conflit de couche

- [ ] **dependencies.json** : Dépendances logiques
  - Toutes les dépendances déclarées
  - Pas de dépendance circulaire invalide
  - Dépendances cohérentes avec le code

- [ ] **files.json** : Cartographie code
  - Tous les fichiers avec blocs MSCM présents
  - Mapping fichier → blocs complet
  - Pas de fichier orphelin

- [ ] **stats.json** : Métriques
  - Statistiques présentes (blocks, files, depth_max, domains, layers)
  - Métriques cohérentes avec le contenu réel
  - Pas d'incohérence statistique

### 2.3 Intégrité Structurelle

- [ ] **ID unique global** : Aucun identifiant dupliqué dans l'index
  - Vérification dans `blocks.json`
  - Vérification dans tous les fichiers d'index
  - Pas de conflit d'identifiants

- [ ] **Aucun bloc orphelin** : Tous les blocs référencés existent
  - Vérification dans `hierarchy.json`
  - Vérification dans `graph.json`
  - Vérification dans `dependencies.json`
  - Toutes les références valides

- [ ] **Aucun cycle invalide** : Pas de cycles interdits dans le graphe
  - Vérification dans `hierarchy.json`
  - Vérification dans `dependencies.json`
  - Cycles détectés et validés comme acceptables si présents

- [ ] **Hiérarchie cohérente** : La hiérarchie respecte les règles architecturales
  - Kernel au niveau le plus bas
  - Cores au-dessus du Kernel
  - Domaines au-dessus des Cores
  - Pas d'inversion hiérarchique

- [ ] **Pas de duplication** : Aucun bloc dupliqué dans l'index
  - Vérification dans `blocks.json`
  - Vérification dans `files.json`
  - Unicité garantie

- [ ] **Pas de conflit layer** : Aucun bloc dans plusieurs couches simultanément
  - Vérification dans `layers.json`
  - Un bloc = une couche unique
  - Cohérence avec `@layer` dans le code

---

## 3. Vérifications par Phase d'Implémentation

### 3.1 Phase 1 — Kernel

**Modules concernés :** `config`, `id`, `time`, `log`, `lifecycle`

#### Vérifications MSCM Kernel

- [ ] Tous les modules Kernel sont balisés MSCM
- [ ] Identifiants uniques pour chaque module (`kernel_config_*`, `kernel_id_*`, `kernel_time_*`, `kernel_log_*`, `kernel_lifecycle_*`)
- [ ] Rôles cohérents : `infra` pour les modules de base
- [ ] Couche déclarée : `kernel` pour tous les modules
- [ ] Descriptions `@human` présentes et claires

#### Vérifications MIP Kernel

- [ ] Index MIP généré pour le Kernel
- [ ] Tous les modules présents dans `blocks.json`
- [ ] Hiérarchie Kernel valide dans `hierarchy.json`
- [ ] Couche `kernel` identifiée dans `layers.json`
- [ ] Pas de dépendances externes (Kernel est autonome)

#### Critères de Validation Phase 1

- [ ] Tous les modules Kernel implémentés et testés
- [ ] Conformité MSCM complète
- [ ] Index MIP valide et cohérent
- [ ] Tests unitaires passants
- [ ] Documentation inline complète

---

### 3.2 Phase 2 — Cores Système

**Cores concernés :** StrongFather, KindMother, BondingBrother, CaringNanny, MasterButler, BorderGuard, EverBuddy, WorrySentinel, TAMR, LogisticsSteward

#### Vérifications MSCM Cores

Pour chaque Core :

- [ ] Tous les composants du Core sont balisés MSCM
- [ ] Identifiants uniques avec préfixe Core (`strongfather_*`, `kindmother_*`, etc.)
- [ ] Rôles cohérents avec la fonction du Core
  - StrongFather : `logic`, `policy`
  - KindMother : `data`, `storage`
  - BondingBrother : `api`, `integration`
  - CaringNanny : `observability`, `monitoring`
  - MasterButler : `orchestration`, `workflow`
  - BorderGuard : `security`, `boundary`
  - EverBuddy : `sync`, `replication`
  - WorrySentinel : `security`, `threat`
  - TAMR : `data`, `analytics`
  - LogisticsSteward : `orchestration`, `workflow`
- [ ] Couche déclarée : `core` pour tous les Cores
- [ ] Descriptions `@human` présentes et spécifiques au Core
- [ ] Dépendances vers Kernel déclarées explicitement

#### Vérifications MIP Cores

Pour chaque Core :

- [ ] Index MIP mis à jour avec le Core
- [ ] Tous les composants présents dans `blocks.json`
- [ ] Hiérarchie Core valide (dépendances Kernel déclarées)
- [ ] Couche `core` identifiée dans `layers.json`
- [ ] Domaines métier identifiés dans `domains.json`
- [ ] Dépendances vers Kernel présentes dans `dependencies.json`
- [ ] Pas de dépendances circulaires entre Cores

#### Critères de Validation Phase 2

Pour chaque Core :

- [ ] Core implémenté selon sa documentation fondatrice
- [ ] Conformité MSCM complète
- [ ] Index MIP mis à jour et cohérent
- [ ] Tests unitaires passants
- [ ] Contrats d'intégration respectés
- [ ] Documentation inline complète

#### Ordre d'Implémentation (Dépendances)

- [ ] StrongFather (dépend de Kernel uniquement)
- [ ] KindMother (dépend de Kernel uniquement)
- [ ] BorderGuard (dépend de Kernel uniquement)
- [ ] BondingBrother (dépend de Kernel, StrongFather, KindMother)
- [ ] CaringNanny (dépend de Kernel, KindMother)
- [ ] MasterButler (dépend de Kernel, StrongFather, KindMother)
- [ ] EverBuddy (dépend de Kernel, KindMother, BondingBrother)
- [ ] WorrySentinel (dépend de Kernel, StrongFather, BorderGuard)
- [ ] TAMR (dépend de Kernel, KindMother, CaringNanny)
- [ ] LogisticsSteward (dépend de Kernel, StrongFather, KindMother, MasterButler)

---

### 3.3 Phase 3 — MiyukiniAdmin

**Composants concernés :** Backend, Frontend, Intégration avec tous les Cores

#### Vérifications MSCM MiyukiniAdmin

- [ ] Tous les composants backend sont balisés MSCM
- [ ] Tous les composants frontend sont balisés MSCM (si applicable)
- [ ] Identifiants uniques avec préfixe `miyukiniadmin_*`
- [ ] Rôles cohérents : `api`, `ui`, `orchestration`, `admin`
- [ ] Couches déclarées : `api`, `service`, `domain` selon l'architecture
- [ ] Descriptions `@human` présentes et spécifiques
- [ ] Dépendances vers tous les Cores déclarées explicitement

#### Vérifications MIP MiyukiniAdmin

- [ ] Index MIP mis à jour avec MiyukiniAdmin
- [ ] Tous les composants présents dans `blocks.json`
- [ ] Hiérarchie MiyukiniAdmin valide (dépendances Cores déclarées)
- [ ] Couches identifiées dans `layers.json`
- [ ] Domaines métier identifiés dans `domains.json`
- [ ] Dépendances vers tous les Cores présentes dans `dependencies.json`
- [ ] Flux administratifs identifiés dans `flows.json`

#### Critères de Validation Phase 3

- [ ] MiyukiniAdmin implémenté selon sa documentation
- [ ] Conformité MSCM complète
- [ ] Index MIP final valide et cohérent
- [ ] Tests unitaires et d'intégration passants
- [ ] Intégration avec tous les Cores validée
- [ ] Documentation inline complète

---

## 4. Vérifications Avant Livraison

### 4.1 Vérification Globale MSCM

- [ ] **Scan complet** : Tous les fichiers de code scannés pour vérifier le balisage MSCM
  - Aucun fichier source sans balisage si requis
  - Couverture complète des blocs critiques
  - Cohérence globale du balisage

- [ ] **Validation des identifiants** : Vérification de l'unicité globale
  - Script de validation exécuté
  - Aucun conflit détecté
  - Conventions de nommage respectées

- [ ] **Validation des rôles** : Vérification de la cohérence des rôles
  - Rôles alignés avec la documentation
  - Pas de rôle générique inapproprié
  - Rôles spécifiques et descriptifs

- [ ] **Validation des couches** : Vérification de la cohérence des couches
  - Hiérarchie respectée
  - Pas de conflit de couche
  - Attribution cohérente avec l'architecture

### 4.2 Vérification Globale MIP

- [ ] **Régénération complète** : Index MIP régénéré depuis le code source
  - Génération réussie sans erreur
  - Tous les fichiers d'index présents
  - Intégrité validée (`registry.json → integrity: "ok"`)

- [ ] **Validation structurelle** : Vérification de l'intégrité structurelle
  - Aucun bloc orphelin
  - Aucun cycle invalide
  - Hiérarchie cohérente
  - Pas de duplication
  - Pas de conflit layer

- [ ] **Validation des dépendances** : Vérification de la cohérence des dépendances
  - Toutes les dépendances déclarées existent
  - Pas de dépendance circulaire invalide
  - Ordre de dépendance respecté (Kernel → Cores → MiyukiniAdmin)

- [ ] **Validation des métriques** : Vérification de la cohérence des statistiques
  - `stats.json` cohérent avec le contenu réel
  - Métriques alignées avec les attentes
  - Pas d'incohérence statistique

### 4.3 Tests et Validation Fonctionnelle

- [ ] **Tests unitaires** : Tous les tests unitaires passants
  - Couverture des blocs critiques
  - Tests pour chaque module/component
  - Aucun test en échec

- [ ] **Tests d'intégration** : Tests d'intégration passants (si applicable)
  - Intégration entre modules validée
  - Intégration entre Cores validée
  - Intégration MiyukiniAdmin validée

- [ ] **Validation fonctionnelle** : Fonctionnalités validées
  - Fonctionnalités conformes à la documentation
  - Pas de comportement implicite
  - Gestion d'erreurs explicite

---

## 5. Critères de Gel et Versionnement

### 5.1 Prérequis au Gel

- [ ] **Conformité MSCM complète** : Tous les critères MSCM validés
  - Balisage complet et conforme
  - Vérifications globales passées
  - Aucune non-conformité restante

- [ ] **Index MIP valide** : Index MIP final généré et validé
  - Génération réussie sans erreur
  - Intégrité validée
  - Structure cohérente

- [ ] **Tests passants** : Tous les tests unitaires et d'intégration passants
  - Aucun test en échec
  - Couverture suffisante
  - Validation fonctionnelle complète

- [ ] **Documentation complète** : Documentation inline et externe complète
  - Documentation inline pour tous les blocs critiques
  - Documentation externe à jour
  - Références croisées valides

### 5.2 Génération de l'Index MIP Final

- [ ] **Index MIP généré** : Index MIP final généré avant gel
  - Tous les fichiers d'index présents dans `mscm_index/`
  - Génération réussie sans erreur
  - Timestamp de génération présent

- [ ] **Intégrité validée** : Intégrité de l'index validée
  - `registry.json → integrity: "ok"`
  - Aucune erreur de validation
  - Structure cohérente

- [ ] **Version MIP associée** : Version de l'index MIP associée au gel
  - Version MIP déclarée dans `registry.json`
  - Version alignée avec la version du projet
  - Traçabilité garantie

### 5.3 Critères de Gel

- [ ] **Gel documenté** : Document de gel officiel rédigé
  - Liste exhaustive des éléments gelés
  - Version explicite attribuée
  - Conditions de dégel documentées

- [ ] **Interdiction de modification** : Toute modification impose un nouveau cycle
  - Règle de dégel explicite
  - Processus de migration documenté
  - Traçabilité complète

---

## 6. Vérifications par Type de Composant

### 6.1 Modules Kernel

**Critères spécifiques :**

- [ ] Balisage MSCM avec préfixe `kernel_*`
- [ ] Couche `kernel` déclarée
- [ ] Rôle `infra` pour les modules de base
- [ ] Aucune dépendance externe (autonomie Kernel)
- [ ] Index MIP avec couche `kernel` identifiée

### 6.2 Cores Système

**Critères spécifiques :**

- [ ] Balisage MSCM avec préfixe Core (`strongfather_*`, `kindmother_*`, etc.)
- [ ] Couche `core` déclarée
- [ ] Rôles spécifiques au Core (voir section 3.2)
- [ ] Dépendances vers Kernel déclarées
- [ ] Index MIP avec couche `core` identifiée
- [ ] Domaines métier identifiés

### 6.3 MiyukiniAdmin

**Critères spécifiques :**

- [ ] Balisage MSCM avec préfixe `miyukiniadmin_*`
- [ ] Couches multiples selon architecture (`api`, `service`, `domain`)
- [ ] Rôles variés (`api`, `ui`, `orchestration`, `admin`)
- [ ] Dépendances vers tous les Cores déclarées
- [ ] Index MIP avec toutes les couches identifiées
- [ ] Flux administratifs identifiés

---

## 7. Checklist Rapide — Avant Chaque Commit

### Vérifications Minimales

- [ ] Nouveaux blocs balisés MSCM (`@id`, `@role`, `@layer`, `@human`)
- [ ] Identifiants uniques (pas de duplication)
- [ ] Dépendances déclarées si présentes
- [ ] Index MIP régénéré après modifications
- [ ] Génération MIP sans erreur

### Vérifications Recommandées

- [ ] Tests unitaires passants
- [ ] Documentation inline à jour
- [ ] Cohérence avec la documentation de référence
- [ ] Pas de code mort ou de duplication

---

## 8. Checklist Rapide — Avant Livraison Phase

### Phase 1 — Kernel

- [ ] Tous les modules Kernel balisés MSCM
- [ ] Index MIP généré et valide
- [ ] Tests unitaires passants
- [ ] Documentation complète
- [ ] Intégrité validée

### Phase 2 — Cores

- [ ] Tous les Cores implémentés balisés MSCM
- [ ] Index MIP mis à jour avec tous les Cores
- [ ] Tests unitaires et d'intégration passants
- [ ] Contrats d'intégration respectés
- [ ] Documentation complète
- [ ] Intégrité validée

### Phase 3 — MiyukiniAdmin

- [ ] MiyukiniAdmin balisé MSCM
- [ ] Index MIP final généré et valide
- [ ] Tests unitaires et d'intégration passants
- [ ] Intégration avec tous les Cores validée
- [ ] Documentation complète
- [ ] Intégrité validée
- [ ] Critères de gel satisfaits

---

## 9. Erreurs Communes et Anti-Patterns

### Erreurs MSCM à Éviter

- [ ] **Bloc sans `@id`** : Tous les blocs critiques doivent avoir un identifiant unique
- [ ] **Identifiant dupliqué** : Vérifier l'unicité globale avant commit
- [ ] **Rôle générique** : Éviter les rôles trop génériques (`misc`, `other`, `util`)
- [ ] **Couche incorrecte** : Respecter la hiérarchie (kernel < core < domain < infra)
- [ ] **Description vide** : Toujours fournir une description `@human` claire
- [ ] **Dépendances non déclarées** : Toutes les dépendances doivent être explicites

### Erreurs MIP à Éviter

- [ ] **Index non régénéré** : Régénérer après chaque modification de code MSCM
- [ ] **Modification manuelle** : Ne jamais modifier l'index manuellement
- [ ] **Blocs orphelins** : Vérifier que tous les blocs référencés existent
- [ ] **Cycles invalides** : Détecter et corriger les cycles interdits
- [ ] **Incohérence hiérarchique** : Respecter l'ordre Kernel → Cores → MiyukiniAdmin
- [ ] **Conflit de couche** : Un bloc ne peut être dans plusieurs couches

---

## 10. Outils et Scripts de Validation

### Scripts Recommandés

- [ ] **Script de validation MSCM** : Vérification du balisage dans le code
  - Détection des blocs sans `@id`
  - Vérification de l'unicité des identifiants
  - Validation des rôles et couches

- [ ] **Script de validation MIP** : Vérification de l'intégrité de l'index
  - Détection des blocs orphelins
  - Détection des cycles invalides
  - Validation de la cohérence structurelle

- [ ] **Script de régénération MIP** : Régénération automatique de l'index
  - Parsing MSCM
  - Construction du graphe
  - Génération des fichiers d'index

### Intégration CI/CD

- [ ] **Validation automatique** : Intégration dans le pipeline CI/CD
  - Validation MSCM avant merge
  - Régénération et validation MIP
  - Blocage en cas de non-conformité

---

## 11. Références Documentaires

### Protocoles Obligatoires

- [MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)
- [Protocole d'implémentation générale](../../protocols/Miyukini%20Prompt%20Protocol%20-%20Implémentation%20générale.md)

### Documentation de Référence

- Documentation Kernel (`docs/kernel/`)
- Documentation Cores (`docs/core/`)
- Documentation MiyukiniAdmin (`docs/core/MiyukiniAdmin/`)

### Guides d'Implémentation

- Guides d'implémentation par Core (`docs/core/*/implementation/`)
- Contrats d'intégration (`docs/core/*/contracts/`)

---

## 12. Traçabilité et Audit

### Enregistrement des Vérifications

- [ ] **Log des vérifications** : Enregistrement de toutes les vérifications effectuées
  - Date et heure de chaque vérification
  - Résultats (pass/échec)
  - Actions correctives si nécessaire

- [ ] **Rapport de conformité** : Rapport de conformité MSCM/MIP généré
  - Statut global (conforme/non conforme)
  - Détails par phase
  - Points d'attention identifiés

### Audit Final

- [ ] **Audit de conformité** : Audit complet avant gel
  - Vérification exhaustive de tous les critères
  - Documentation des écarts éventuels
  - Validation finale par l'équipe

---

## Conclusion

Cette check-list constitue la **référence contractuelle** pour la conformité MSCM/MIP dans le cadre de l'implémentation de Miyukini COG vers. 0.1.0.

**Rappel :**
- Tout code produit DOIT être conforme MSCM
- Tout projet DOIT maintenir un index MIP valide
- Un fichier sans balisage MSCM conforme est considéré comme **non livrable**
- Un projet sans index MIP valide ne peut pas être gelé

**Utilisation :**
- Vérifier chaque critère avant chaque commit
- Valider complètement avant chaque livraison de phase
- S'assurer de la conformité complète avant gel

---

**Version :** 1.0  
**Date de création :** 2026-01-28  
**Statut :** Contractuel — Obligatoire  
**Mainteneur :** Équipe d'implémentation Miyukini COG vers. 0.1.0
