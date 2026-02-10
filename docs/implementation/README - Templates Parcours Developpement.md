# 📋 Templates de Parcours de Développement Miyukini

## 🎯 Vue d'ensemble

Ce dossier contient les **templates et guides** pour structurer vos projets de développement Miyukini COG, de l'idée au livrable.

---

## 📄 Fichiers Disponibles

### 1. **Template Principal**
**Fichier :** `Miyukini - Template Parcours Developpement Projet.md`

**Description :** Template complet et réutilisable pour tous vos projets (Services, Opérateurs, Toolkits, Outils).

**Contenu :**
- 5 phases de développement structurées
- Checklists détaillées pour chaque étape
- Critères de passage entre phases
- Métriques de suivi et reporting
- Gestion des blocages et risques

**Utilisation :**
```bash
# Copier le template dans votre projet
cp "docs/implementation/Miyukini - Template Parcours Developpement Projet.md" \
   "docs/services/[VOTRE_SERVICE]/[NOM_PROJET] - Parcours Developpement.md"
```

---

### 2. **Guide d'Utilisation**
**Fichier :** `Miyukini - Guide Utilisation Template Parcours.md`

**Description :** Guide complet pour utiliser efficacement le template.

**Contenu :**
- Démarrage rapide
- Bonnes pratiques
- Workflows spécifiques (Service, Toolkit, Opérateur)
- Personnalisation du template
- FAQ

**Quand le lire :** AVANT d'utiliser le template pour la première fois.

---

### 3. **Exemple Concret : Purse**
**Fichier :** `docs/services/JayKonta/publics/Purse/Purse - Parcours Developpement.md`

**Description :** Exemple d'application du template au projet **Purse** (JayKonta).

**Intérêt :**
- Voir comment remplir concrètement le template
- Comprendre la granularité attendue
- S'inspirer pour vos propres projets

**Quand le consulter :** Quand vous avez un doute sur comment remplir une section.

---

## 🚀 Démarrage Rapide

### Étape 1 : Lire le Guide
```bash
# Ouvrir le guide d'utilisation
code "docs/implementation/Miyukini - Guide Utilisation Template Parcours.md"
```

### Étape 2 : Copier le Template
```bash
# Pour un Service
cp "docs/implementation/Miyukini - Template Parcours Developpement Projet.md" \
   "docs/services/[SERVICE]/[PROJET] - Parcours Developpement.md"

# Pour un Toolkit
cp "docs/implementation/Miyukini - Template Parcours Developpement Projet.md" \
   "docs/tools/[TOOLKIT]/[PROJET] - Parcours Developpement.md"
```

### Étape 3 : Remplir l'En-tête
```markdown
# [NOM DU PROJET] — Parcours de Développement

> **Template v1.0**
> **Date de création :** [DATE]
> **Responsable :** [NOM]
> **Type d'entité :** [X] Service | [ ] Opérateur | [ ] Toolkit | [ ] Outil
> **Strate cible :** [ ] 6 (Outils) | [ ] 7 (Opérateurs) | [ ] 9 (Admin)
```

### Étape 4 : Travailler Phase par Phase
```
Phase 1 → Valider critères ✅ → Phase 2 → Valider critères ✅ → ...
```

---

## 📚 Structure des 5 Phases

### **Phase 1 : Conception & Fondations** 🎯
- Idéation & cadrage
- Documentation fondatrice
- **Livrable clé :** Document Fondateur

### **Phase 2 : Architecture & Contrats** 🏗️
- Architecture technique
- **Conception UX/UI (AVANT implémentation !)**
- Spécifications & contrats
- **Livrable clé :** Architecture + Wireframes validés

### **Phase 3 : Implémentation & Validation** 💻
- Développement itératif (TDD)
- Tests & qualité
- Intégration dans le Central
- **Livrable clé :** Code fonctionnel testé

### **Phase 4 : Raffinement & Gamification** 🎮
- Gamification (si applicable)
- Polish UI/UX
- **Livrable clé :** Expérience utilisateur optimisée

### **Phase 5 : Livraison & Documentation** 📦
- Documentation utilisateur
- Documentation technique
- Release
- **Livrable clé :** Projet livré et documenté

---

## 💡 Règles d'Or

### 1. **Ne Sautez JAMAIS les Phases**
❌ Idée → Implémentation directe → Casse-tête UI  
✅ Idée → Documentation → Architecture → UI Design → Implémentation

### 2. **Concevez l'UI AVANT d'Implémenter**
Phase 2.2 (Conception UX/UI) est **CRITIQUE** et doit être terminée avant Phase 3.

### 3. **Validez les Critères de Passage**
Chaque phase se termine par des critères de passage. Ne les ignorez pas !

### 4. **Documentez les Blocages Immédiatement**
Un blocage non documenté est un blocage oublié.

### 5. **Mettez à Jour Régulièrement**
Statut, métriques, checklists → update hebdomadaire minimum.

---

## 🎨 Personnalisation

Le template est **adaptable** :

✅ **Vous pouvez :**
- Ajouter des sections spécifiques à votre projet
- Adapter les checklists à votre contexte
- Fusionner des phases pour micro-projets (< 2h)

❌ **Vous NE devez PAS :**
- Supprimer les vérifications de conformité architecturale (8 Lois)
- Sauter Phase 2.2 (Conception UX/UI) pour projets avec interface
- Ignorer les critères de passage

---

## 🛠️ Outils Recommandés

### Documentation
- **Markdown** (ce template) : Suivi détaillé
- **Mermaid** : Diagrammes intégrés
- **DrawIO** : Schémas d'architecture

### Conception UI
- **MiyuLayoutBuilder** (à venir) : Outil interne Miyukini
- **Figma / Penpot** : Maquettes haute-fidélité
- **Wireframes papier** : Prototypage rapide

### Gestion
- **Trello / Notion** : Vue Kanban
- **Git Issues** : Suivi bugs/features
- **Pomodoro** : Time management

---

## 📊 Amélioration Continue

### Après Chaque Projet
1. **Remplir la section "Notes & Apprentissages"**
2. **Analyser les métriques** (temps estimé vs réel)
3. **Identifier les patterns récurrents**
4. **Améliorer vos estimations**

### Feedback sur le Template
Si vous identifiez des améliorations au template :
1. Documentez-les dans un `brain_template_feedback.md`
2. Proposez une évolution (v1.1, v2.0, etc.)
3. Mettez à jour le template principal

---

## 🎯 Objectifs du Système de Parcours

### Avant (Ancien Processus)
- Conception UI tardive → refactorisations coûteuses
- Gamification réfléchie en fin → intégration difficile
- Manque d'outils pour concevoir layouts → blocages
- Documentation éparse → perte d'information

### Après (Nouveau Processus)
- ✅ UI conçue AVANT implémentation
- ✅ Gamification intégrée dès Phase 1
- ✅ Workflows clairs et reproductibles
- ✅ Documentation centralisée et structurée
- ✅ Métriques pour amélioration continue

---

## 📈 Statistiques d'Utilisation

### Projets Utilisant ce Template
| Projet | Type | Phase Actuelle | Statut |
|--------|------|----------------|--------|
| Purse | Service | Phase 2 (60%) | 🟡 En cours |
| [Votre Projet] | ... | ... | ... |

---

## 🔗 Références

### Skills Miyukini
- `.cursor/skills/miyukini-architecture/SKILL.md` : Architecture pyramidale
- `.cursor/skills/miyukini-glossary/SKILL.md` : Terminologie officielle
- `.cursor/skills/miyukini-docs/SKILL.md` : Nomenclature documentation
- `.cursor/skills/miyukini-rust-patterns/SKILL.md` : Patterns Rust

### Documentation Conceptuelle
- `docs/reference/Miyukini Conceptual References - Lois Autonomie Systeme.md`
- `docs/reference/Miyukini Conceptual References - Pyramide Architecture Complete.md`
- `docs/reference/Miyukini Conceptual References - Glossaire.md`

---

## ❓ Questions Fréquentes

### Q : Dois-je vraiment suivre toutes les phases ?
**R :** Oui pour projets moyens/grands. Pour micro-projets (< 2h), vous pouvez fusionner Phase 1+2 et sauter Phase 4.

### Q : Combien de temps prend chaque phase ?
**R :** Voir le guide d'utilisation pour les ordres de grandeur par taille de projet.

### Q : Comment gérer plusieurs projets simultanés ?
**R :** Créez un `_index.md` dans votre dossier de service pour avoir un dashboard de projets.

### Q : Le template est-il figé ?
**R :** Non ! Vous pouvez (et devez) l'adapter à vos besoins. Proposez des améliorations.

---

## 🎊 Contributeurs

- **Créateur :** Miyukini Team
- **Version :** 1.0
- **Date :** 2026-02-07
- **Feedback :** Documentez vos retours dans `brain_template_feedback.md`

---

**✨ Bon développement avec le Parcours Miyukini COG !**
