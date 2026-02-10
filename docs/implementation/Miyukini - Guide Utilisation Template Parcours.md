# Guide d'Utilisation du Template Parcours Développement

## 🎯 Objectif

Ce guide vous explique comment utiliser efficacement le **Template Parcours Développement Projet** pour structurer vos projets Miyukini.

---

## 📋 Démarrage Rapide

### 1. Copier le Template

```bash
# Depuis le dossier racine du projet
cp "docs/implementation/Miyukini - Template Parcours Developpement Projet.md" "docs/services/[VOTRE_SERVICE]/[NOM_PROJET] - Parcours Developpement.md"
```

**Exemples :**
- `docs/services/JayKonta/Purse - Parcours Developpement.md`
- `docs/tools/MiyuLayoutBuilder/MiyuLayoutBuilder - Parcours Developpement.md`

---

### 2. Remplir l'En-tête

```markdown
# Purse — Parcours de Développement

> **Template v1.0**  
> **Date de création :** 2026-02-07  
> **Responsable :** Miyukini Team  
> **Type d'entité :** [X] Service | [ ] Opérateur | [ ] Toolkit | [ ] Outil  
> **Strate cible :** [ ] 6 (Outils) | [X] 7 (Opérateurs) | [ ] 9 (Admin)
```

---

### 3. Travailler Phase par Phase

**Règle d'or :** Ne passez JAMAIS à la phase suivante sans avoir validé les critères de passage.

```
Phase 1 → Critères validés ✅ → Phase 2 → Critères validés ✅ → Phase 3 → ...
```

---

## 🔧 Utilisation au Quotidien

### Mise à Jour des Checklists

**Format recommandé :**
- `[ ]` : Non commencé
- `[⏳]` : En cours
- `[✅]` : Terminé
- `[❌]` : Annulé/Non applicable
- `[⚠️]` : Bloqué

**Exemple :**
```markdown
#### Checklist
- [✅] Document Fondateur rédigé
- [⏳] Bornage fonctionnel documenté (IN / OUT)
- [ ] Analyse des dépendances complète
- [⚠️] Première réflexion sur la gamification (BLOQUÉ : attente validation)
```

---

### Suivre l'Avancement

Mettez à jour la section **Métriques de Suivi** régulièrement (idéalement chaque semaine).

```markdown
### Avancement Global
- **Phase 1 :** [X] 100%
- **Phase 2 :** [X] 75% ← Actuellement ici
- **Phase 3 :** [ ] 0%
```

---

### Documenter les Blocages

Dès qu'un blocage apparaît, documentez-le immédiatement :

```markdown
| Date | Blocage | Impact | Solution proposée | Statut |
|------|---------|--------|-------------------|--------|
| 2026-02-07 | Pas d'outil pour créer des layouts UI | 🔴 Haut | Créer MiyuLayoutBuilder | ⏳ En cours |
```

---

## 💡 Bonnes Pratiques

### 1. **Ne Sautez Pas les Phases**

❌ **Mauvais :**
```
Idée → Implémentation directe → Casse-tête UI
```

✅ **Bon :**
```
Idée → Documentation → Architecture → UI Design → Implémentation
```

---

### 2. **Documentez AVANT d'Implémenter**

Chaque phase de conception (1 et 2) doit produire des **livrables documentés** avant de passer au code.

**Pourquoi ?**
- Évite les refactorisations coûteuses
- Clarifie les décisions techniques
- Facilite la collaboration
- Réduit les bugs architecturaux

---

### 3. **Validez les Critères de Passage**

Chaque phase se termine par des **critères de passage**. Ne les ignorez pas !

**Exemple Phase 2.2 (Conception UX/UI) :**
```
✅ L'UI est conçue, validée, et prête pour l'implémentation.
```

Si ce critère n'est pas rempli → **restez en Phase 2.2**.

---

### 4. **Utilisez les Livrables comme Guides**

Chaque checklist indique les **livrables attendus**. Créez ces fichiers au fur et à mesure :

```
docs/services/JayKonta/Purse/
├── Purse - Parcours Developpement.md ← Le template rempli
├── Purse - Document Fondateur.md
├── Purse - Architecture Technique.md
├── Purse - Wireframes UI.md
├── Purse - Design System.md
└── ...
```

---

### 5. **Mettez à Jour le Statut Régulièrement**

**En haut du document :**
```markdown
**Statut actuel :** [X] Implémentation
```

Cela vous donne une vue rapide de l'avancement sans parcourir tout le document.

---

## 🚀 Workflows Spécifiques

### Workflow : Nouveau Service

1. **Copier le template** dans `docs/services/[SERVICE_NAME]/`
2. **Phase 1** : Rédiger le Document Fondateur
3. **Phase 2** : Concevoir l'architecture + UI
4. **Phase 3** : Implémenter avec TDD
5. **Phase 4** : Polish + gamification
6. **Phase 5** : Documentation + Release

---

### Workflow : Nouveau Toolkit

1. **Copier le template** dans `docs/tools/[TOOLKIT_NAME]/`
2. **Phase 1** : Définir les Outils du Kit
3. **Phase 2** : Spécifier les contrats d'interface
4. **Phase 3** : Implémenter les Outils
5. **Phase 4** : (Optionnel) Polish UI si le Toolkit a une interface
6. **Phase 5** : Documentation technique + Release

---

### Workflow : Nouvel Opérateur

1. **Copier le template** dans `docs/services/[SERVICE_NAME]/operators/[OPERATOR_NAME]/`
2. **Phase 1** : Définir le rôle de l'Opérateur
3. **Phase 2** : Spécifier les interactions avec les Cores
4. **Phase 3** : Implémenter avec gouvernance StrongFather
5. **Phase 4** : Intégrer dans le Central
6. **Phase 5** : Documentation + Release

---

## 🎨 Personnalisation du Template

### Ajouter des Sections Spécifiques

Si votre projet nécessite des sections supplémentaires, ajoutez-les :

**Exemple : Projet avec intégration externe**
```markdown
### 2.4 Intégration Services Externes

#### Checklist
- [ ] API externe documentée
- [ ] Contrat d'interface défini
- [ ] Stratégie de fallback en cas d'indisponibilité
- [ ] Tests de conformité LOI-1 (pas de dépendance critique)
```

---

### Adapter les Checklists

Les checklists sont des **guides**, pas des obligations rigides. Adaptez-les à votre contexte :

❌ **Ne supprimez pas :** Les vérifications de conformité architecturale (Lois, Cores, etc.)  
✅ **Vous pouvez :** Ajouter des items spécifiques à votre projet

---

## 📊 Suivi Multi-Projets

### Créer un Dashboard de Projets

Créez un fichier `_index.md` dans `docs/services/` ou `docs/tools/` pour suivre plusieurs projets :

```markdown
# Projets en Cours

| Projet | Type | Statut | Phase Actuelle | Blocages |
|--------|------|--------|----------------|----------|
| Purse | Service | 🟡 En cours | Phase 3 | 1 blocage |
| MiyuLayoutBuilder | Toolkit | 🟢 Conception | Phase 2 | Aucun |
| JayXpose Dashboard | UI | 🔴 Bloqué | Phase 4 | 3 blocages |
```

---

## 🛠️ Outils Recommandés

### Suivi de Tâches
- **Markdown** (ce template) : Documentation et suivi détaillé
- **Trello / Notion** : Vue Kanban pour les sprints
- **Git Issues** : Suivi des bugs et features

### Gestion de Temps
- **Pomodoro** : 25 min de travail, 5 min de pause
- **Time Blocking** : Planifier des blocs dédiés à chaque phase

---

## ❓ FAQ

### Q1 : Dois-je obligatoirement suivre toutes les phases ?

**R :** Oui, pour les projets moyens à grands. Pour les **micro-projets** (< 2 heures), vous pouvez :
- Fusionner Phase 1 et 2
- Réduire la documentation au minimum
- Sauter Phase 4 si pas de gamification

### Q2 : Combien de temps prend chaque phase ?

**R :** Cela dépend du projet. Voici des ordres de grandeur :

| Phase | Petit Projet | Moyen Projet | Grand Projet |
|-------|--------------|--------------|--------------|
| Phase 1 | 1-2h | 4-8h | 1-2 jours |
| Phase 2 | 2-4h | 8-16h | 2-5 jours |
| Phase 3 | 4-8h | 2-5 jours | 1-3 semaines |
| Phase 4 | 1-2h | 4-8h | 2-5 jours |
| Phase 5 | 1-2h | 4-8h | 1-2 jours |

### Q3 : Que faire si je découvre un problème architectural en Phase 3 ?

**R :** **Revenez en Phase 2 !** C'est normal. Mettez à jour :
1. La documentation d'architecture
2. Les contrats d'interface
3. Le template de parcours (marquez la Phase 2 comme "révisée")

### Q4 : Dois-je remplir toutes les dates ?

**R :** Non. Les dates sont **optionnelles** mais recommandées pour :
- Analyser votre vélocité
- Identifier les phases qui prennent plus de temps que prévu
- Améliorer vos estimations futures

### Q5 : Comment gérer les projets multi-personnes ?

**R :** 
- Assignez un **responsable par phase**
- Utilisez Git pour le versioning du template
- Faites des **revues de phase** en équipe avant de valider les critères de passage

---

## 🎯 Checklist d'Adoption

Vous savez que vous maîtrisez le template quand :

- [✅] Vous avez copié le template pour un nouveau projet
- [✅] Vous avez rempli au moins une phase complète
- [✅] Vous avez documenté un blocage
- [✅] Vous avez validé des critères de passage
- [✅] Vous avez mis à jour les métriques d'avancement
- [✅] Vous avez personnalisé une checklist
- [✅] Vous avez créé tous les livrables d'une phase

---

## 📚 Références

- **Template principal :** `docs/implementation/Miyukini - Template Parcours Developpement Projet.md`
- **Architecture Miyukini :** `.cursor/skills/miyukini-architecture/SKILL.md`
- **Glossaire :** `.cursor/skills/miyukini-glossary/SKILL.md`
- **Patterns Rust :** `.cursor/skills/miyukini-rust-patterns/SKILL.md`

---

**✨ Bon développement avec le Parcours Miyukini !**
