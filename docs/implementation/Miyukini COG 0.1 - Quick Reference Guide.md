# Miyukini COG 0.1 - Quick Reference Guide

**Guide rapide pour agents IA**  
**Version :** 0.1  
**Statut :** Référence d'implémentation  
**Audience :** Agents IA implémentant Miyukini COG 0.1

---

## Contexte

Ce document est un **guide condensé** pour les agents IA qui implémentent Miyukini COG 0.1. Il fournit les références essentielles, les check-lists critiques et les règles non négociables.

**Document complet :** Voir `Miyukini COG 0.1 - Documentation Implementation Reference.md`  
**Check-list conformité :** Voir `Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`

---

## 1. Qu'est-ce que Miyukini COG 0.1 ?

**COG = Core-Orchestrated Governance Environment**

Miyukini COG 0.1 est un environnement de gouvernance orchestré par des cores qui coordonne, sécurise et fait fonctionner des systèmes logiciels complets, du noyau jusqu'à l'utilisateur final.

### Périmètre COG 0.1

- ✅ **Phase 1 :** Kernel (fondation technique)
- ✅ **Phase 2 :** Cores système (10 cores)
- ✅ **Phase 3 :** MiyukiniAdmin (opérateur souverain)

### Exclusions explicites

- ❌ Pas de modules produits (SPM CMS, etc.)
- ❌ Pas d'interfaces utilisateur finales
- ❌ Pas de déploiement production

---

## 2. Protocoles Obligatoires

### 2.1 Cycle d'implémentation strict

**Référence :** `docs/protocols/Miyukini Prompt Protocol - Implémentation générale.md`

```
1. Planification
   ↓
2. Distribution des tâches aux agents
   ↓
3. Vérification, corrections et tests
   ↓
4. Gel et versionnement
```

**Règle absolue :** Aucune étape ne peut être sautée ou fusionnée.

### 2.2 Protocole MSCM v1

**Référence :** `docs/protocols/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol.md`

**Obligations minimales MSCM :**

- ✅ Chaque bloc fonctionnel DOIT avoir un `@id` unique
- ✅ Chaque bloc DOIT avoir un `@role` explicite
- ✅ Chaque bloc DOIT avoir un `@layer` déclaré
- ✅ Chaque bloc DOIT avoir une description `@human`
- ✅ Les dépendances inter-blocs DOIVENT être déclarées

**Exemple de balisage :**

```rust
// @id: kernel-config-load
// @role: primitive
// @layer: kernel
// @human: Charge la configuration système depuis le système de fichiers
pub fn load_config() -> Result<Config, ConfigError> {
    // ...
}
```

### 2.3 Protocole MIP v1

**Référence :** `docs/protocols/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol.md`

**Règles MIP :**

- ✅ L'index MIP est **externe** au code (dossier `mscm_index/`)
- ✅ L'index est **reconstruit**, jamais modifié manuellement
- ✅ Le code est la seule source de vérité
- ✅ Régénérer l'index MIP après chaque modification

**Structure de l'index MIP :**

```
mscm_index/
├── registry.json      # Métadonnées et intégrité
├── blocks.json        # Identité sémantique
├── hierarchy.json     # Structure hiérarchique
├── graph.json         # Relations transverses
├── flows.json         # Processus métier
├── domains.json       # Vision métier
├── layers.json        # Architecture technique
├── dependencies.json  # Dépendances logiques
├── files.json         # Cartographie code
└── stats.json         # Métriques
```

---

## 3. Ordre d'Implémentation Strict

### Phase 1 : Kernel (fondation)

**Ordre des modules :**

1. `config` - Configuration système
2. `id` - Génération d'identifiants
3. `time` - Gestion du temps
4. `log` - Système de logging
5. `lifecycle` - Gestion du cycle de vie

**Références Kernel :**

- Documentation : `docs/kernel/`
- Guidelines : `docs/kernel/implementation/Kernel - Reference Implementation Guidelines.md`
- Tests : `docs/kernel/tests/Kernel - Tests Unitaires Specification.md`

**Check-list Phase 1 :**

- [ ] Tous les modules Kernel implémentés
- [ ] Tests unitaires passants
- [ ] Balisage MSCM complet
- [ ] Index MIP régénéré sans erreur
- [ ] Conformité aux invariants Kernel (INV-K-*)

### Phase 2 : Cores Système

**Ordre d'implémentation (selon dépendances) :**

1. **StrongFather** - Moteur de décision
2. **KindMother** - Autorité des données
3. **BorderGuard** - Définition des frontières
4. **MasterButler** - Registre des capacités
5. **CaringNanny** - Observateur d'état
6. **EverBuddy** - Gouvernance du cycle de vie
7. **TAMR** - Intervention humaine
8. **WorrySentinel** - Gouvernance de sécurité
9. **BondingBrother** - Médiation et adaptation
10. **LogisticsSteward** - Orchestration opérationnelle

**Références par Core :**

Chaque Core a sa documentation dans `docs/core/<CoreName>/` :

- `foundation/` - Documentation fondatrice
- `implementation/` - Guidelines d'implémentation
- `contracts/` - Contrats d'intégration

**Check-list Phase 2 :**

- [ ] Cores implémentés dans l'ordre strict
- [ ] Contrats d'intégration respectés
- [ ] Tests obligatoires passants
- [ ] Balisage MSCM complet
- [ ] Index MIP régénéré après chaque Core
- [ ] Conformité aux invariants Core (INV-*-*)

### Phase 3 : MiyukiniAdmin

**Architecture :**

- Backend : Interface avec les Cores
- Frontend : Interface opérateur souverain

**Références MiyukiniAdmin :**

- Documentation : `docs/core/MiyukiniAdmin/`
- Foundation : `docs/core/MiyukiniAdmin/foundation/`
- Implementation : `docs/core/MiyukiniAdmin/implementation/`

**Check-list Phase 3 :**

- [ ] Backend implémenté
- [ ] Frontend implémenté
- [ ] Intégration avec tous les Cores validée
- [ ] Tests end-to-end passants
- [ ] Balisage MSCM complet
- [ ] Index MIP final régénéré

---

## 4. Règles Strictes pour Agents IA

### 4.1 Règles Générales

- ✅ Respecter rigoureusement les protocoles référencés
- ✅ Ne jamais contourner les invariants documentés
- ✅ Toujours baliser le code en MSCM
- ✅ Régénérer l'index MIP après chaque modification
- ✅ Respecter l'ordre d'implémentation strict

### 4.2 Règles de Code

- ✅ Qualité optimale (pas de code "quick and dirty")
- ✅ Tests unitaires obligatoires
- ✅ Documentation inline complète
- ✅ Gestion d'erreurs explicite
- ✅ Pas de dépendances non autorisées

### 4.3 Règles MSCM/MIP

- ✅ Chaque bloc fonctionnel DOIT avoir un `@id` unique
- ✅ Chaque bloc DOIT avoir un `@role` explicite
- ✅ Chaque bloc DOIT avoir un `@layer` déclaré
- ✅ Chaque bloc DOIT avoir une description `@human`
- ✅ Les dépendances inter-blocs DOIVENT être déclarées

### 4.4 Règles de Distribution

- ✅ **1 agent = 1 fichier**
- ✅ Contexte vierge obligatoire pour chaque agent
- ✅ Maximum **4 agents simultanés**
- ✅ Pas de tâche mutualisation
- ✅ Pas de batch/vague/groupe de tâches

### 4.5 Règle d'Arrêt Stricte

Un agent DOIT S'ARRÊTER IMMÉDIATEMENT si :

- ❌ Une ambiguïté bloquante est détectée
- ❌ Une dépendance manquante est rencontrée
- ❌ La fenêtre de contexte devient insuffisante
- ❌ Le fichier et le test unitaire sont terminés et corrects

**Action :** Rendre la main à l'humain, aucun fichier partiel généré.

---

## 5. Check-lists de Conformité

### 5.1 Check-list Avant Livraison d'un Fichier

- [ ] Code implémenté selon la documentation de référence
- [ ] Balisage MSCM complet et conforme
- [ ] Tests unitaires présents et passants
- [ ] Gestion d'erreurs explicite
- [ ] Documentation inline complète
- [ ] Aucune dépendance non autorisée
- [ ] Conformité aux invariants du composant

### 5.2 Check-list Avant Phase 4 (Gel)

**Contrôles MSCM :**

- [ ] Tous les blocs critiques sont balisés MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (`@layer`) sont cohérentes avec l'architecture
- [ ] Aucun bloc orphelin (sans `@id` ou `@role`)
- [ ] Les dépendances inter-blocs sont déclarées

**Régénération de l'index MIP :**

- [ ] L'index MIP peut être régénéré sans erreur
- [ ] Aucun bloc orphelin détecté
- [ ] Aucun cycle invalide dans le graphe
- [ ] Intégrité validée (`registry.json → integrity: "ok"`)

**Tests et Validation :**

- [ ] Tous les tests unitaires passants
- [ ] Tests d'intégration passants
- [ ] Validation fonctionnelle complète
- [ ] Audit de code effectué

---

## 6. Références Essentielles

### 6.1 Protocoles

| Document | Chemin | Usage |
|----------|--------|-------|
| **Protocole d'implémentation générale** | `docs/protocols/Miyukini Prompt Protocol - Implémentation générale.md` | Cycle strict, règles agents |
| **MIP v1 MSCM Index Protocol** | `docs/protocols/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol.md` | Indexation structurelle |
| **Écriture documentation conceptuelle** | `docs/protocols/Miyukini Prompt Protocol - Ecriture Documentation Conceptuelle.md` | Standards documentaires |

### 6.2 Kernel

| Document | Chemin | Usage |
|----------|--------|-------|
| **Definition Kernel** | `docs/kernel/Miyukini Core System - Definition Kernel.md` | Périmètre, responsabilités |
| **Structure du Kernel** | `docs/kernel/Miyukini Core System - Structure du Kernel.md` | Crates, dépendances |
| **Implementation Guidelines** | `docs/kernel/implementation/Kernel - Reference Implementation Guidelines.md` | Guide d'implémentation |
| **Tests Specification** | `docs/kernel/tests/Kernel - Tests Unitaires Specification.md` | Spécifications tests |
| **Invariants & Guarantees** | `docs/kernel/contracts/Kernel - Invariants & Guarantees.md` | Invariants INV-K-* |

### 6.3 Cores Système

**Structure standard par Core :**

```
docs/core/<CoreName>/
├── foundation/
│   └── <CoreName> - Documentation Fondatrice.md
├── implementation/
│   └── <CoreName> - Reference Implementation Guidelines.md
├── contracts/
│   └── [contrats spécifiques]
└── _index.md
```

**Cores à implémenter :**

1. **StrongFather** - `docs/core/StrongFather/`
2. **KindMother** - `docs/core/KindMother/`
3. **BorderGuard** - `docs/core/BorderGuard/`
4. **MasterButler** - `docs/core/MasterButler/`
5. **CaringNanny** - `docs/core/CaringNanny/`
6. **EverBuddy** - `docs/core/EverBuddy/`
7. **TAMR** - `docs/core/TAMR/`
8. **WorrySentinel** - `docs/core/WorrySentinel/`
9. **BondingBrother** - `docs/core/BondingBrother/`
10. **LogisticsSteward** - `docs/core/LogisticsSteward/`

### 6.4 MiyukiniAdmin

| Document | Chemin | Usage |
|----------|--------|-------|
| **Foundation** | `docs/core/MiyukiniAdmin/foundation/` | Documentation fondatrice |
| **Implementation** | `docs/core/MiyukiniAdmin/implementation/` | Guidelines d'implémentation |
| **Architecture** | `docs/core/MiyukiniAdmin/architecture/` | Architecture backend/frontend |

### 6.5 Références Conceptuelles

| Document | Chemin | Usage |
|----------|--------|-------|
| **Definition COG** | `docs/reference/Miyukini Conceptual References - Definition COG.md` | Qu'est-ce que COG |
| **Vision Stratégique** | `docs/reference/Miyukini Conceptual References - Vision Strategique.md` | Objectifs fondamentaux |
| **Lois Autonomie Système** | `docs/reference/Miyukini Conceptual References - Lois Autonomie Systeme.md` | Contraintes d'autonomie |
| **Pyramide Architecture** | `docs/reference/Miyukini Conceptual References - Pyramide Architecture Complete.md` | Architecture globale |
| **Glossaire** | `docs/reference/Miyukini Conceptual References - Glossaire.md` | Terminologie |

### 6.6 Sécurité

| Document | Chemin | Usage |
|----------|--------|-------|
| **Doctrine Sécurité Fondamentale** | `docs/reference/Miyukini Conceptual References - Doctrine Securite Fondamentale.md` | Principes sécurité |
| **Security Implementation** | `docs/security/implementation/Security - Reference Implementation Guidelines.md` | Guidelines sécurité |

---

## 7. Invariants Critiques par Composant

### 7.1 Kernel (INV-K-*)

**INV-K-1 :** Zéro logique métier  
**INV-K-2 :** Pas de dépendance externe critique  
**INV-K-3 :** Primitives locales sûres uniquement  
**INV-K-4 :** Pas de réseau, pas de fichiers système  
**INV-K-5 :** Pas de panics silencieux  
**INV-K-6 :** Déterminisme

**Référence complète :** `docs/kernel/contracts/Kernel - Invariants & Guarantees.md`

### 7.2 StrongFather (INV-SF-*)

**INV-SF-1 :** Décisions locales uniquement  
**INV-SF-2 :** Complémentaire avec KindMother  
**INV-SF-3 :** Moteur pur, jamais d'exécution  
**INV-SF-4 :** Pas de logique temporelle technique

**Référence complète :** `docs/core/StrongFather/contracts/governance/StrongFather - Invariants & Guarantees.md`

### 7.3 KindMother (INV-KM-*)

**INV-KM-1 :** Autorité absolue des données  
**INV-KM-2 :** Offline-first fondamental  
**INV-KM-3 :** SQLite est un détail d'implémentation  
**INV-KM-4 :** Aucun module SPM ne parle directement à une base de données  
**INV-KM-5 :** Offline-first est un principe fondamental non négociable

**Référence complète :** `docs/core/KindMother/contracts/governance/KindMother - Invariants & Guarantees.md`

### 7.4 Autres Cores

Chaque Core a ses propres invariants documentés dans `docs/core/<CoreName>/contracts/governance/`.

**Règle absolue :** Ne jamais violer un invariant documenté.

---

## 8. Processus de Validation

### 8.1 Validation d'un Fichier

1. **Vérification code :**
   - [ ] Conforme à la documentation de référence
   - [ ] Gestion d'erreurs explicite
   - [ ] Documentation inline complète

2. **Vérification MSCM :**
   - [ ] Balisage complet
   - [ ] Identifiants uniques
   - [ ] Rôles et couches cohérents

3. **Vérification tests :**
   - [ ] Tests unitaires présents
   - [ ] Tests passants
   - [ ] Couverture suffisante

### 8.2 Validation d'une Phase

1. **Vérification complétude :**
   - [ ] Tous les composants de la phase implémentés
   - [ ] Tous les tests passants
   - [ ] Documentation à jour

2. **Vérification MSCM/MIP :**
   - [ ] Index MIP régénéré sans erreur
   - [ ] Intégrité validée
   - [ ] Graphe cohérent

3. **Vérification conformité :**
   - [ ] Invariants respectés
   - [ ] Contrats d'intégration respectés
   - [ ] Aucune violation détectée

### 8.3 Validation Avant Gel (Phase 4)

1. **Audit complet :**
   - [ ] Audit de code effectué
   - [ ] Rapport de conformité MSCM/MIP
   - [ ] Liste exhaustive des éléments gelés

2. **Index MIP final :**
   - [ ] Index MIP généré et inclus dans le gel
   - [ ] Intégrité validée
   - [ ] Version de l'index MIP associée

3. **Versionnement :**
   - [ ] Version explicite attribuée (ex : v0.1.0)
   - [ ] Règles d'évolution futures définies
   - [ ] Conditions de dégel documentées

---

## 9. Erreurs Communes à Éviter

### 9.1 Erreurs MSCM

- ❌ **Oublier le balisage MSCM** → Code non livrable
- ❌ **Identifiants dupliqués** → Erreur MIP
- ❌ **Couches incohérentes** → Architecture violée
- ❌ **Dépendances non déclarées** → Graphe invalide

### 9.2 Erreurs d'Implémentation

- ❌ **Violer un invariant** → Non conforme
- ❌ **Code "quick and dirty"** → Qualité insuffisante
- ❌ **Tests manquants** → Non livrable
- ❌ **Dépendances non autorisées** → Architecture violée

### 9.3 Erreurs de Processus

- ❌ **Sauter une étape du cycle** → Processus violé
- ❌ **Modifier l'index MIP manuellement** → Source de vérité corrompue
- ❌ **Ne pas régénérer l'index MIP** → Index obsolète
- ❌ **Parallélisation excessive** → Plus de 4 agents simultanés

---

## 10. Aide-Mémoire Rapide

### 10.1 Avant de Commencer

1. ✅ Lire la documentation de référence du composant
2. ✅ Identifier les invariants à respecter
3. ✅ Vérifier les dépendances
4. ✅ Définir le balisage MSCM attendu

### 10.2 Pendant l'Implémentation

1. ✅ Baliser chaque bloc fonctionnel en MSCM
2. ✅ Respecter les invariants strictement
3. ✅ Écrire les tests unitaires
4. ✅ Documenter le code inline

### 10.3 Avant de Livrer

1. ✅ Vérifier la conformité MSCM
2. ✅ Exécuter tous les tests
3. ✅ Vérifier la gestion d'erreurs
4. ✅ Régénérer l'index MIP

### 10.4 En Cas de Blocage

1. ✅ S'arrêter immédiatement
2. ✅ Ne pas générer de fichier partiel
3. ✅ Rendre la main à l'humain
4. ✅ Documenter l'ambiguïté ou la dépendance manquante

---

## 11. Contacts et Références

**Document principal :** `Miyukini COG 0.1 - Documentation Implementation Reference.md`  
**Check-list conformité :** `Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`

**Protocoles :** `docs/protocols/`  
**Kernel :** `docs/kernel/`  
**Cores :** `docs/core/`  
**Références :** `docs/reference/`

---

**Document créé le :** 2026-01-28  
**Version :** 0.1  
**Statut :** Référence d'implémentation
