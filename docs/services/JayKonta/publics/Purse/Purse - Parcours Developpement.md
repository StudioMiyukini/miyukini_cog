# Purse (JayKonta) — Parcours de Développement

> **Template v1.0** — Exemple d'application du Template Miyukini  
> **Date de création :** 2026-02-07  
> **Responsable :** Miyukini Team  
> **Type d'entité :** [X] Service | [ ] Opérateur | [ ] Toolkit | [ ] Outil  
> **Strate cible :** [ ] 6 (Outils) | [X] 7 (Opérateurs) | [ ] 9 (Admin)

---

## 📋 Vue d'ensemble

**Description en une phrase :**  
Purse est le point d'entrée de JayKonta pour le budget personnel, permettant aux particuliers de suivre leurs dépenses, gérer des budgets occasionnels et atteindre des objectifs d'épargne.

**Objectif principal :**  
Donner une vision claire du solde et des dépenses, piloter des budgets occasionnels, suivre des objectifs sans bruit ni surcharge.

**Non-objectifs :**  
- ❌ Devis et facturation entreprise
- ❌ Comptabilité AP/AR
- ❌ Gestion multi-devises avancée
- ❌ Import automatique bancaire (pour v1)

**Statut actuel :** [X] Conception | [ ] Implémentation | [ ] Raffinement | [ ] Livré

---

## Phase 1 : Conception & Fondations 🎯

### 1.1 Idéation & Cadrage

**Date de début :** 2026-01-15  
**Date de fin :** 2026-01-20

#### Checklist
- [✅] Idée formulée et documentée (brain note ou doc initial)
- [✅] Type d'entité identifié (Service JayKonta, point d'entrée Purse)
- [✅] Strate dans la pyramide confirmée (Strate 7 - Opérateurs)
- [✅] Conformité aux 8 Lois d'Autonomie vérifiée
  - [✅] LOI-1 : Aucune dépendance externe critique (DB locale KindMother)
  - [✅] LOI-2 : Accepte l'isolement comme état normal
  - [✅] LOI-3 : État local souverain
  - [✅] LOI-4 : Pas de temps global requis
  - [✅] LOI-5 : Coût proportionnel au hardware
  - [✅] LOI-6 : Autonomie compatible avec fédération
  - [✅] LOI-7 : Strate Cores immuable
  - [✅] LOI-8 : Migration = diplomatie entre environnements
- [✅] Portée définie et bornée

#### Livrables
- [✅] `Purse - Analyse des besoins.md` (version 2.0)
- [✅] Validation architecturale

**Critères de passage :** ✅ L'idée est claire, bornée, et architecturalement valide.

---

### 1.2 Documentation Fondatrice

**Date de début :** 2026-01-21  
**Date de fin :** 2026-01-28

#### Checklist
- [✅] Document Fondateur rédigé (intégré dans JayKonta - Document Fondateur)
- [✅] Section **Contexte** : Budget personnel pour particuliers et foyers
- [✅] Section **Portée / Scope** : Mouvements, budgets occasionnels, objectifs, alertes, rapports
- [✅] Section **Vision & Objectifs** : O1 à O5 documentés
- [✅] Section **Non-objectifs** : Devis, facturation légale, AP/AR entreprise
- [✅] Section **Dépendances**
- [✅] Bornage fonctionnel documenté (IN / OUT)
- [✅] Analyse des dépendances complète
  - [✅] Cores sollicités identifiés
    - StrongFather : Permissions et mandats
    - KindMother : Persistance comptes, mouvements, budgets
    - Border Guard : Sécurité niveau 2
    - Caring Nanny : Monitoring soldes
    - Master Butler : Capacités export et alertes
  - [✅] Outils requis listés (Toolkits MiyuXxx)
  - [✅] Opérateurs liés documentés (JayKoa pour rappels)
- [⏳] Première réflexion sur la gamification (EN COURS)
  - Idées : Streaks de saisie, badges objectifs atteints, progression visuelle

#### Livrables
- [✅] `Purse - Analyse des besoins.md` (v2.0)
- [✅] `Purse - Operateurs et Toolkits.md`
- [✅] `Purse - Parcours Capacites Livrables.md` (v2.0)

**Critères de passage :** ✅ La portée est définie, les frontières sont claires.

---

## Phase 2 : Architecture & Contrats 🏗️

### 2.1 Architecture Technique

**Date de début :** 2026-01-29  
**Date de fin :** 2026-02-05

#### Checklist
- [⏳] Schéma d'architecture créé (Mermaid ou diagramme)
- [⏳] Flux de données documenté
- [✅] Interactions entre composants définies (Contrats CK-xxx)
- [⏳] Décisions techniques prises
  - [✅] Structure du crate définie (crates/jaykonta)
  - [⏳] Modules identifiés (purse/, mouvements/, budgets/)
  - [✅] Patterns Rust sélectionnés (admin_cell, context, errors)
- [⏳] Plan de persistance établi
  - [⏳] Schéma KindMother défini
    - Tables : purse_accounts, purse_movements, purse_budgets, purse_goals
  - [⏳] Tables et migrations planifiées
  - [ ] Stratégie de backup/restore
- [✅] Interfaces avec les Cores définies
  - [✅] StrongFather : CK-SVC-01, CK-SVC-02 (permissions)
  - [✅] KindMother : CK-OP-01, CK-OP-02, CK-OP-03 (persistance)
  - [✅] BondingBrother : CK-INT-03 (liaison JayKoa)
  - [✅] Master Butler : Capacités exposées (CK-TK-11, CK-TK-51, CK-TK-61)
  - [✅] Border Guard : Niveau sécurité 2
  - [ ] Caring Nanny : Monitoring soldes (à définir)
  - [ ] Ever Buddy : Cycle de vie (à définir)
  - [✅] WorrySentinel : CK-AUD-01, CK-AUD-02 (audit)
  - [ ] TAMR : Intervention humaine (à définir)

#### Livrables
- [ ] `Purse - Architecture Technique.md` (À CRÉER)
- [ ] Diagrammes d'architecture (Mermaid)
- [ ] `Purse - Schema Persistance KindMother.md` (À CRÉER)

**Critères de passage :** ⏳ L'architecture est validée, stable et documentée. **EN COURS**

---

### 2.2 Conception UX/UI ⚠️ CRITIQUE

**Date de début :** 2026-02-06  
**Date de fin :** ___________

#### Checklist
- [⚠️] Wireframes basse-fidélité créés
  - [⏳] Navigation principale (Dashboard → Mouvements → Budgets → Objectifs)
  - [⏳] Écrans principaux identifiés (6 parcours P1-P6)
  - [✅] Flux utilisateur esquissé (Parcours Capacités Livrables)
- [❌] Design System défini
  - [ ] Composants réutilisables listés
  - [ ] Layouts types définis
  - [ ] Palette de couleurs
  - [ ] Typographie
  - [ ] Espacement et grilles
- [❌] Maquettes interactives réalisées
  - [ ] **MiyuLayoutBuilder** (NON DISPONIBLE - BLOCAGE MAJEUR)
  - [ ] Prototype cliquable
- [✅] Parcours utilisateur documenté
  - [✅] Happy path (P1-P6)
  - [✅] Edge cases (Risques UX documentés)
  - [✅] Gestion d'erreurs
- [ ] Intégration de la gamification
  - [ ] Points de friction identifiés
  - [ ] Système de récompenses défini
  - [ ] Progression utilisateur planifiée
  - [ ] Challenges optionnels

#### Livrables
- [ ] `Purse - Wireframes UI.md` (À CRÉER)
- [ ] `Purse - Design System.md` (À CRÉER)
- [ ] `Purse - Parcours Utilisateur.md` (PARTIEL - Parcours Capacités existe)
- [ ] `Purse - Gamification Design.md` (À CRÉER)

**⚠️ BLOCAGE MAJEUR :** Schémas UI avec containers ne fonctionnent pas bien. **MiyuLayoutBuilder requis.**

**Critères de passage :** ❌ L'UI est conçue, validée, et prête pour l'implémentation. **BLOQUÉ**

---

### 2.3 Spécifications & Contrats

**Date de début :** 2026-02-01  
**Date de fin :** 2026-02-05

#### Checklist
- [✅] Contrats d'interface rédigés
  - [✅] API publiques documentées (Contrats CK-xxx)
  - [✅] Signatures de fonctions définies
  - [✅] Formats de données spécifiés
- [✅] Règles de gouvernance établies
  - [✅] Permissions et mandats définis
  - [✅] Décisions StrongFather spécifiées
  - [✅] Niveaux de sécurité assignés (Niveau 2 minimum)
- [✅] Tests d'acceptation écrits (Critères CA-1 à CA-4)
- [✅] Documentation des cas limites
  - [✅] Comportements en conditions dégradées
  - [✅] Gestion des erreurs
  - [✅] Fallbacks et modes offline

#### Livrables
- [✅] `JayKonta - Contrats Service Operateurs et Toolkits.md`
- [✅] `Purse - Analyse des besoins.md` (Section Critères d'acceptation)
- [✅] Risques et mitigations documentés

**Critères de passage :** ✅ Les contrats sont validés, les interfaces sont stables.

---

## Phase 3 : Implémentation & Validation 💻

### 3.1 Développement Itératif (TDD)

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist : Structure du Crate
- [ ] `crates/jaykonta/src/purse/admin_cell.rs` créé
- [ ] `crates/jaykonta/src/purse/context.rs` créé
- [ ] `crates/jaykonta/src/purse/errors.rs` créé
- [ ] `crates/jaykonta/src/purse/mod.rs` créé
- [ ] `crates/jaykonta/Cargo.toml` mis à jour
- [ ] Structure de modules définie (mouvements, budgets, goals)

#### Checklist : Tests Unitaires
- [ ] Tests unitaires écrits AVANT le code
- [ ] Couverture de code > 80%
- [ ] Tests de gouvernance (StrongFather)
- [ ] Tests de persistance (KindMother)
- [ ] Tests d'erreurs et edge cases

#### Checklist : Implémentation Core
- [ ] Logique métier implémentée
  - [ ] Création compte Purse
  - [ ] Gestion mouvements (CRUD)
  - [ ] Gestion catégories
  - [ ] Budgets occasionnels
  - [ ] Objectifs et progression
  - [ ] Alertes configurables
- [ ] Interactions avec les Cores fonctionnelles
- [ ] Gestion d'erreurs robuste
- [ ] Documentation inline (MSCM tags)

#### Checklist : Intégration KindMother
- [ ] Schéma de base de données créé
  - [ ] Table `purse_accounts`
  - [ ] Table `purse_movements`
  - [ ] Table `purse_categories`
  - [ ] Table `purse_budgets`
  - [ ] Table `purse_goals`
- [ ] Migrations implémentées
- [ ] Requêtes CRUD fonctionnelles
- [ ] Tests de persistance passants

#### Checklist : Interface UI
- [ ] Composants Dioxus créés
  - [ ] Dashboard
  - [ ] Formulaire mouvement rapide
  - [ ] Liste mouvements
  - [ ] Écran budgets occasionnels
  - [ ] Écran objectifs
  - [ ] Module rapports/export
- [ ] Layouts implémentés
- [ ] Styles appliqués
- [ ] Navigation fonctionnelle

#### Livrables par Itération
- [ ] Code fonctionnel avec tests passants
- [ ] Documentation inline (MSCM tags)
- [ ] Mise à jour de l'index MIP

**Critères de passage :** ⏳ Le code fonctionne, les tests passent, la documentation est à jour. **EN ATTENTE PHASE 2**

---

### 3.2 Tests & Qualité

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist : Types de Tests
- [ ] **Tests unitaires** : chaque fonction critique
- [ ] **Tests d'intégration** : interactions entre modules
- [ ] **Tests de gouvernance** : validation décisions StrongFather
- [ ] **Tests de persistance** : KindMother, migrations, rollback
- [ ] **Tests UX** : parcours utilisateur P1-P6, accessibilité
- [ ] **Tests de performance** : 
  - [ ] Dashboard < 3s médiane (NFR-PUR-04)
  - [ ] Saisie mouvement < 2s médiane (NFR-PUR-05)
- [ ] **Tests de sécurité** : validation niveau 2 minimum

#### Checklist : Validation Qualité
- [ ] Aucun linter error
- [ ] `cargo clippy` passe sans warnings critiques
- [ ] `cargo fmt` appliqué
- [ ] Code review effectuée
- [ ] Documentation complète

#### Livrables
- [ ] Rapport de tests (coverage, résultats)
- [ ] `Purse - Rapport Qualite.md`

**Critères de passage :** ⏳ Tous les tests passent, qualité validée. **EN ATTENTE PHASE 3.1**

---

### 3.3 Intégration dans le Central

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist
- [ ] Enregistrement de l'Opérateur Purse dans le système
- [ ] Configuration des permissions
  - [ ] Mandats définis (CK-SVC-01, CK-SVC-02)
  - [ ] Niveaux de sécurité configurés (Niveau 2)
- [ ] Documentation d'intégration rédigée
- [ ] Vérification des dépendances (conformité LOI-1)
- [ ] Tests d'intégration avec JayKoa (rappels optionnels)
- [ ] Validation par StrongFather

#### Livrables
- [ ] `Purse - Integration Central.md`
- [ ] Configuration de déploiement

**Critères de passage :** ⏳ L'Opérateur est intégré et fonctionnel dans le COG. **EN ATTENTE PHASE 3.2**

---

## Phase 4 : Raffinement & Gamification 🎮

### 4.1 Gamification (si applicable)

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist
- [ ] Système de progression implémenté
  - [ ] **Streaks de saisie** : badges 7 jours, 30 jours, 90 jours
  - [ ] **Niveaux** : Novice → Confirmé → Expert → Maître Budget
  - [ ] **Achievements** : Premier objectif atteint, Premier budget bouclé, etc.
- [ ] Récompenses intégrées
  - [ ] Feedback positif ("Bravo ! 10 jours consécutifs de saisie")
  - [ ] Encouragements contextuels
  - [ ] Badges / trophées visuels
- [ ] Challenges créés
  - [ ] **Challenge "Budget Zéro"** : Respecter un budget occasionnel sans dépassement
  - [ ] **Challenge "Objectif 30 jours"** : Atteindre un objectif d'épargne en 30 jours
  - [ ] Objectifs secondaires optionnels
- [ ] Narration cohérente
  - [ ] Storytelling aligné avec l'univers Miyukini
  - [ ] Messages encourageants et bienveillants

#### Livrables
- [ ] `Purse - Systeme Gamification.md`
- [ ] Code de gamification testé

**Critères de passage :** ⏳ La gamification enrichit l'expérience sans nuire à l'utilisabilité. **EN ATTENTE**

---

### 4.2 Polish UI/UX

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist
- [ ] Animations implémentées
  - [ ] Transitions fluides entre écrans
  - [ ] Feedback visuel (boutons, validations)
  - [ ] Loading states (spinner, skeleton screens)
- [ ] Responsive Design
  - [ ] Adaptation mobile (NFR-PUR-07)
  - [ ] Adaptation tablette
  - [ ] Adaptation desktop
- [ ] Thème appliqué
  - [ ] Cohérence visuelle Miyukini
  - [ ] Mode clair / sombre (si applicable)
- [ ] Accessibilité validée
  - [ ] WCAG 2.1 niveau AA minimum
  - [ ] Navigation au clavier
  - [ ] Screen readers compatibles

#### Livrables
- [ ] UI polie et testée
- [ ] `Purse - Rapport Accessibilite.md`

**Critères de passage :** ⏳ L'UI est polie, fluide, et accessible. **EN ATTENTE**

---

## Phase 5 : Livraison & Documentation 📦

### 5.1 Documentation Utilisateur

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist
- [ ] Guide de démarrage rédigé
  - [ ] Premiers pas avec Purse
  - [ ] Onboarding (Parcours P1)
- [ ] Documentation de référence complète
  - [ ] Toutes les fonctionnalités documentées (P2-P6)
  - [ ] Captures d'écran / vidéos
- [ ] FAQ créée
  - [ ] "Comment créer un budget occasionnel ?"
  - [ ] "Comment configurer des alertes ?"
  - [ ] Solutions aux problèmes courants
- [ ] Tutoriels rédigés
  - [ ] "Gérer un budget vacances"
  - [ ] "Atteindre un objectif d'épargne"
  - [ ] Scénarios réels

#### Livrables
- [ ] `Purse - Guide Demarrage.md`
- [ ] `Purse - Documentation Reference.md`
- [ ] `Purse - FAQ.md`
- [ ] `Purse - Tutoriels.md`

---

### 5.2 Documentation Technique

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist
- [ ] Architecture finale documentée
  - [ ] Schémas à jour
  - [ ] Décisions techniques expliquées
- [ ] Contrats & API documentés
  - [ ] Documentation complète des interfaces
  - [ ] Exemples d'utilisation
- [ ] Guide de maintenance rédigé
  - [ ] Comment étendre (nouveaux types de budgets, etc.)
  - [ ] Comment déboguer
  - [ ] Points d'attention (alertes, performance)
- [ ] Index MIP à jour
  - [ ] Toutes les balises MSCM présentes
  - [ ] Index global régénéré

#### Livrables
- [ ] `Purse - Architecture Finale.md`
- [ ] `Purse - API Reference.md`
- [ ] `Purse - Guide Maintenance.md`
- [ ] Index MIP mis à jour

---

### 5.3 Release

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist Finale
- [ ] ✅ Tous les tests passent
- [ ] ✅ Documentation complète
- [ ] ✅ Conformité architecturale validée
- [ ] ✅ Aucun linter error
- [ ] ✅ Performance acceptable (NFR-PUR-04, NFR-PUR-05)
- [ ] ✅ Sécurité auditée (niveau 2 minimum)
- [ ] ✅ Code review finale
- [ ] ✅ Validation par les pairs
- [ ] ✅ Approbation StrongFather

#### Checklist Release
- [ ] Version taggée (ex: v1.0.0)
- [ ] Notes de release rédigées
- [ ] Migration guide créé (si applicable)
- [ ] Changelog mis à jour
- [ ] Commit de release créé
- [ ] Push vers le repository

#### Livrables
- [ ] **Version 1.0.0** déployée
- [ ] `Purse - Notes Release v1.0.0.md`
- [ ] `Purse - Migration Guide.md` (si applicable)
- [ ] `CHANGELOG.md` mis à jour

**Critères de passage :** ⏳ Le projet est livré, documenté, et prêt pour la production. **EN ATTENTE**

---

## 📊 Métriques de Suivi

### Avancement Global
- **Phase 1 :** [X] 100% ✅
- **Phase 2 :** [X] 60% ⏳ (BLOQUÉ sur 2.2 UI)
- **Phase 3 :** [ ] 0%
- **Phase 4 :** [ ] 0%
- **Phase 5 :** [ ] 0%

### Temps estimé vs réel
| Phase | Estimé | Réel | Delta |
|-------|--------|------|-------|
| Phase 1 | 16 h | 14 h | -2 h |
| Phase 2 | 24 h | 18 h (en cours) | ___ h |
| Phase 3 | 80 h | ___ h | ___ h |
| Phase 4 | 16 h | ___ h | ___ h |
| Phase 5 | 12 h | ___ h | ___ h |
| **TOTAL** | **148 h** | **32 h** | **___ h** |

### Qualité
- **Couverture de tests :** N/A (pas encore implémenté)
- **Linter errors :** 0
- **Performance (temps de réponse moyen) :** N/A
- **Score accessibilité :** N/A

---

## 🚨 Blocages & Risques

### Blocages Actuels
| Date | Blocage | Impact | Solution proposée | Statut |
|------|---------|--------|-------------------|--------|
| 2026-02-07 | Schémas UI avec containers ne fonctionnent pas bien | 🔴 Haut | Créer MiyuLayoutBuilder pour concevoir layouts Dioxus visuellement | ⏳ Identifié |
| 2026-02-07 | Pas d'outil pour builder des layouts | 🔴 Haut | Développer MiyuLayoutBuilder comme projet pilote | ⏳ Planifié |

### Risques Identifiés
| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Surcharge fonctionnelle Purse | 🟡 Moyenne | 🟡 Moyen | Maintenir focus budget personnel, éviter logique entreprise |
| Alertes trop fréquentes (alert fatigue) | 🟡 Moyenne | 🔴 Haut | Seuils configurables, profilage utilisateur, granularité fine |
| Confusion catégorie/projet occasionnel | 🟢 Basse | 🟡 Moyen | UX séparant clairement catégories globales et budgets dédiés |
| Performance dashboard sur mobile | 🟡 Moyenne | 🟡 Moyen | Optimisation requêtes, lazy loading, caching |
| Complexité UI sur petits écrans | 🟡 Moyenne | 🔴 Haut | Responsive design, priorisation widgets, progressive disclosure |

---

## 📝 Notes & Apprentissages

### Ce qui a bien fonctionné
- Documentation fondatrice structurée (Analyse besoins, Contrats, Parcours)
- Définition claire des frontières (IN/OUT scope)
- Priorisation explicite des besoins (P0-P3)
- Conformité aux 8 Lois d'Autonomie validée dès le début

### Ce qui peut être amélioré
- **UI conçue trop tard** : devrait être en Phase 2, AVANT implémentation
- **Manque d'outils pour concevoir layouts** : blocage identifié → MiyuLayoutBuilder requis
- **Gamification réfléchie tardivement** : devrait être intégrée dès Phase 1.2

### Leçons apprises
- ✅ **Ne jamais implémenter sans avoir conçu l'UI** : évite refactorisations coûteuses
- ✅ **Documenter les contrats AVANT le code** : clarifie les interfaces, réduit les bugs
- ⚠️ **Besoin d'un toolkit interne pour builder des layouts** : investissement prioritaire

---

## 🔗 Références

### Documents liés
- [Purse - Analyse des besoins](./Purse%20-%20Analyse%20des%20besoins.md)
- [Purse - Parcours Capacites Livrables](./Purse%20-%20Parcours%20Capacites%20Livrables.md)
- [Purse - Operateurs et Toolkits](./Purse%20-%20Operateurs%20et%20Toolkits.md)
- [JayKonta - Document Fondateur](../../JayKonta%20-%20Document%20Fondateur.md)
- [JayKonta - Contrats Service Operateurs et Toolkits](../../JayKonta%20-%20Contrats%20Service%20Operateurs%20et%20Toolkits.md)

### Dépendances externes
- **Cores :** StrongFather, KindMother, Border Guard, Caring Nanny, Master Butler, WorrySentinel
- **Toolkits :** MiyuAuth, MiyuBudget, MiyuReport, MiyuAlert
- **Opérateurs :** JayKoa (rappels optionnels)

### Contacts
- **Responsable technique :** Miyukini Team
- **Responsable UX/UI :** À définir
- **Validation architecture :** Miyukini Team

---

## 🎯 Prochaines Actions

### Priorité Immédiate
1. **[CRITIQUE]** Débloquer Phase 2.2 :
   - Option A : Développer MiyuLayoutBuilder (projet pilote)
   - Option B : Utiliser outil externe temporaire (Figma, Penpot)
   - Option C : Wireframes papier/drawio (rapide mais limité)

2. **[HAUTE]** Finaliser Phase 2.1 :
   - Créer `Purse - Architecture Technique.md`
   - Créer schémas Mermaid (flux données, interactions Cores)
   - Définir schéma KindMother complet

3. **[MOYENNE]** Compléter Phase 2.3 :
   - Définir interfaces Caring Nanny, Ever Buddy, TAMR
   - Documenter stratégie backup/restore

---

**✨ Projet créé avec le Template Parcours Développement Miyukini COG v1.0**
