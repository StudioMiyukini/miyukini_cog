# MiyukiniAdmin — UI Design Philosophy

## 1. Contexte

Ce document definit la **philosophie de design** de l'interface utilisateur de MiyukiniAdmin. L'UI est concue comme une **console d'administration** (pas une UI Operateur), inspiree de PHPMyAdmin mais adaptee aux principes Miyukini.

## 2. Portee / Scope

Ce document definit :
- Les principes de design
- La philosophie visuelle
- Les patterns d'interaction
- Les guidelines generales

Ce document **ne couvre pas** :
- Les specifications detaillees des ecrans (voir documents dedies)
- L'implementation technique
- Les composants specifiques

---

## 3. Principes Fondamentaux

### 3.1 Console Root, Pas Produit Metier

> **MiyukiniAdmin est une console d'administration technique, pas une interface utilisateur finale.**

**Implications :**
- Priorite a la fonctionnalite sur l'esthetique
- Information dense mais lisible
- Actions explicites et tracees
- Feedback immediat

### 3.2 Inspiration PHPMyAdmin

**Ce que nous empruntons :**
- Structure en panneaux (navigation + contenu)
- Tableaux de donnees avec pagination
- Actions rapides sur les lignes
- Feedback visuel des operations

**Ce que nous adaptons :**
- Design plus moderne
- Metriques en temps reel
- Integration securite Miyukini
- Tracabilite renforcee

### 3.3 Principes Cles

| Principe | Description |
|----------|-------------|
| **Clarte** | Information lisible, hierarchisee |
| **Efficacite** | Actions rapides, peu de clics |
| **Securite** | Confirmations pour actions critiques |
| **Tracabilite** | Historique visible des actions |
| **Feedback** | Retour immediat sur chaque action |

---

## 4. Architecture de l'Interface

### 4.1 Structure Generale

```
┌─────────────────────────────────────────────────────────────────────┐
│  HEADER                                                             │
│  Logo | Navigation principale | User | Alerts | Security Level      │
├──────────────┬──────────────────────────────────────────────────────┤
│              │                                                      │
│  SIDEBAR     │  MAIN CONTENT                                        │
│              │                                                      │
│  - Dashboard │  [Zone de contenu principal]                         │
│  - Metriques │                                                      │
│  - Database  │                                                      │
│  - Tests     │                                                      │
│  - Securite  │                                                      │
│  - Logs      │                                                      │
│              │                                                      │
├──────────────┴──────────────────────────────────────────────────────┤
│  FOOTER                                                             │
│  Version | Uptime | Last sync | Trust Level indicator               │
└─────────────────────────────────────────────────────────────────────┘
```

### 4.2 Zones Principales

| Zone | Role | Persistance |
|------|------|-------------|
| **Header** | Navigation, statut global | Toujours visible |
| **Sidebar** | Navigation sections | Toujours visible |
| **Main Content** | Contenu contextuel | Change selon section |
| **Footer** | Informations systeme | Toujours visible |

---

## 5. Palette de Couleurs

### 5.1 Couleurs Principales

| Role | Couleur | Code | Usage |
|------|---------|------|-------|
| **Primary** | Bleu fonce | #1a365d | Actions principales |
| **Secondary** | Gris | #4a5568 | Elements secondaires |
| **Background** | Blanc/Gris clair | #f7fafc | Fond |
| **Text** | Gris fonce | #2d3748 | Texte principal |

### 5.2 Couleurs Semantiques

| Etat | Couleur | Code | Usage |
|------|---------|------|-------|
| **Success** | Vert | #38a169 | Operations reussies |
| **Warning** | Jaune/Orange | #d69e2e | Alertes, attention |
| **Error** | Rouge | #e53e3e | Erreurs, critiques |
| **Info** | Bleu clair | #3182ce | Informations |

### 5.3 Couleurs Niveaux de Securite

| Niveau | Couleur | Code |
|--------|---------|------|
| 0 - PUBLIC | Vert | #48bb78 |
| 1 - STANDARD | Bleu | #4299e1 |
| 2 - SENSITIVE | Jaune | #ecc94b |
| 3 - CRITICAL | Orange | #ed8936 |
| 4 - HARDENED | Rouge | #f56565 |

### 5.4 Couleurs Trust Levels

| Niveau | Couleur | Code |
|--------|---------|------|
| T0 - Normal | Vert | #48bb78 |
| T1 - Attention | Jaune | #ecc94b |
| T2 - Degrade | Orange | #ed8936 |
| T3 - Critique | Rouge | #f56565 |
| T4 - Urgence | Rouge fonce | #c53030 |

---

## 6. Typographie

### 6.1 Polices

| Usage | Police | Fallback |
|-------|--------|----------|
| **Titres** | Inter | sans-serif |
| **Corps** | Inter | sans-serif |
| **Code/Donnees** | JetBrains Mono | monospace |

### 6.2 Tailles

| Element | Taille | Poids |
|---------|--------|-------|
| H1 | 24px | 700 |
| H2 | 20px | 600 |
| H3 | 16px | 600 |
| Body | 14px | 400 |
| Small | 12px | 400 |
| Code | 13px | 400 |

---

## 7. Composants de Base

### 7.1 Boutons

| Type | Usage | Style |
|------|-------|-------|
| **Primary** | Action principale | Fond bleu, texte blanc |
| **Secondary** | Action secondaire | Fond gris, texte fonce |
| **Danger** | Action destructive | Fond rouge, texte blanc |
| **Ghost** | Action discrete | Transparent, texte bleu |

```
┌─────────────────────┐  ┌─────────────────────┐
│    EXECUTER         │  │    ANNULER          │
│    [Primary]        │  │    [Secondary]      │
└─────────────────────┘  └─────────────────────┘

┌─────────────────────┐  ┌─────────────────────┐
│    SUPPRIMER        │  │    Details          │
│    [Danger]         │  │    [Ghost]          │
└─────────────────────┘  └─────────────────────┘
```

### 7.2 Tableaux (Style PHPMyAdmin)

```
┌──────┬────────────────┬─────────────┬───────────┬─────────┐
│  #   │ Nom            │ Type        │ Taille    │ Actions │
├──────┼────────────────┼─────────────┼───────────┼─────────┤
│  1   │ users          │ TABLE       │ 15.2 MB   │ [...]   │
│  2   │ orders         │ TABLE       │ 45.8 MB   │ [...]   │
│  3   │ products       │ TABLE       │ 8.3 MB    │ [...]   │
├──────┴────────────────┴─────────────┴───────────┴─────────┤
│ Affichage 1-3 sur 45 | < Precedent | Page 1 | Suivant >   │
└───────────────────────────────────────────────────────────┘
```

### 7.3 Cards Metriques

```
┌───────────────────────────┐
│  CPU Usage                │
│  ┌─────────────────────┐  │
│  │        45%          │  │
│  │    [Gauge/Chart]    │  │
│  └─────────────────────┘  │
│  Load: 2.4 | Cores: 8     │
└───────────────────────────┘
```

### 7.4 Alertes

```
┌─────────────────────────────────────────────────────────┐
│ ⚠ WARNING                                               │
│ CPU usage has exceeded 85% for the last 5 minutes       │
│                                          [Dismiss] [View]│
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ ✓ SUCCESS                                               │
│ Migration completed successfully                        │
│                                          [Dismiss]       │
└─────────────────────────────────────────────────────────┘
```

---

## 8. Patterns d'Interaction

### 8.1 Confirmations

**Regles :**
- Actions reversibles : Pas de confirmation
- Actions avec impact : Confirmation simple
- Actions critiques : Confirmation + justification

```
┌─────────────────────────────────────────────────────────┐
│  Confirmer le changement de niveau de securite ?        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  De: Niveau 2 (SENSITIVE)                               │
│  Vers: Niveau 3 (CRITICAL)                              │
│                                                         │
│  Cette action affectera tous les Operateurs.            │
│                                                         │
│  Justification (obligatoire):                           │
│  ┌───────────────────────────────────────────────────┐  │
│  │                                                   │  │
│  └───────────────────────────────────────────────────┘  │
│                                                         │
│           [Annuler]              [Confirmer]            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 8.2 Feedback Operations

| Etat | Indicateur |
|------|------------|
| En cours | Spinner + texte "En cours..." |
| Succes | Check vert + texte "Termine" |
| Erreur | X rouge + message d'erreur |

### 8.3 Navigation

**Breadcrumbs pour le contexte :**
```
Dashboard > Database > Tables > users > Structure
```

**Actions contextuelles dans la zone de contenu :**
```
[Rafraichir] [Exporter] [Filtrer] [...]
```

---

## 9. Responsive et Accessibilite

### 9.1 Breakpoints

| Breakpoint | Largeur | Adaptation |
|------------|---------|------------|
| Desktop | > 1200px | Layout complet |
| Tablet | 768-1200px | Sidebar collapsible |
| Mobile | < 768px | Non supporte (admin) |

**Note :** MiyukiniAdmin est une console admin. Le support mobile complet n'est pas prioritaire. Une version reduite est acceptable.

### 9.2 Accessibilite

| Critere | Implementation |
|---------|----------------|
| Contraste | WCAG AA minimum |
| Clavier | Navigation complete |
| Focus | Indicateurs visibles |
| Labels | ARIA labels |

---

## 10. Etats Speciaux

### 10.1 Mode Recovery

```
┌─────────────────────────────────────────────────────────────────┐
│ ⚠️ MODE RECOVERY ACTIF                                          │
│ Temps restant: 25:30                                            │
│ [Terminer Recovery]                                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  [Contenu normal avec bandeau rouge]                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 10.2 Mode Degradation

```
┌─────────────────────────────────────────────────────────────────┐
│ ⚠ SYSTEME EN MODE DEGRADE (T2)                                  │
│ Certaines fonctionnalites peuvent etre limitees                 │
├─────────────────────────────────────────────────────────────────┤
```

### 10.3 Chargement

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│                    [Spinner]                                    │
│                 Chargement...                                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 11. Do's and Don'ts

### 11.1 Do's

| Faire | Raison |
|-------|--------|
| Feedback immediat | Utilisateur informe |
| Confirmations pour critiques | Eviter erreurs |
| Historique visible | Tracabilite |
| Codes couleur coherents | Comprehension rapide |
| Donnees paginées | Performance |

### 11.2 Don'ts

| Ne pas faire | Raison |
|--------------|--------|
| Animations excessives | Console admin ≠ marketing |
| Auto-refresh sans controle | Perturbant |
| Actions silencieuses | Tracabilite |
| Modales empilees | UX degradee |
| Donnees non paginées | Performance |

---

## 12. Documents Associes

- [MiyukiniAdmin - Dashboard & Metrics Display](./MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md)
- [MiyukiniAdmin - DB Management Interface](./MiyukiniAdmin%20-%20DB%20Management%20Interface.md)
- [MiyukiniAdmin - Security Control Panel](./MiyukiniAdmin%20-%20Security%20Control%20Panel.md)
- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document de reference
