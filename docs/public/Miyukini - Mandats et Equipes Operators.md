# Miyukini Conceptual References — Mandats et Équipes d'Opérateurs

## Contexte

Ce document définit les **mécanismes de collaboration entre Opérateurs** dans l'écosystème Miyukini. Il introduit les concepts de **Mandat de Permission (Allow Mandate)**, d'**Équipe d'Opérateurs (Operator Team)**, et clarifie la distinction fondamentale entre **Service** et **Opérateur (Operator)**.

**Phrase fondatrice :**

> **In Miyukini, complexity is handled by collaboration, not accumulation.**

## Portée / Scope

- **Applicable à :** Architecture des Opérateurs, gouvernance des collaborations, performance système
- **Audience :** Architectes, développeurs, opérateurs d'environnement
- **Statut :** Document de référence normatif

---

## 1. Problème : L'Opérateur Monolithique

### Pourquoi un Opérateur "trop gros" est une impasse

Un Opérateur façon "WordPress + 50 plugins" pose 4 problèmes majeurs :

| Problème | Impact |
|----------|--------|
| **Surface d'attaque énorme** | Vulnérabilités multipliées |
| **Sécurité uniforme forcée** | Tout au niveau le plus haut = surcoût |
| **Couplage fonctionnel** | Tout dépend de tout |
| **Évolution lente** | Toucher une brique = risquer l'ensemble |

**👉 Exactement les défauts que Miyukini veut éliminer.**

### Règle absolue

> **❌ Un Opérateur ne doit JAMAIS devenir un "super-Opérateur".**

---

## 2. Distinction Fondamentale : Service ≠ Opérateur

### Définitions canoniques

| Concept | Définition |
|---------|------------|
| **Service** | Capacité perçue par l'utilisateur |
| **Opérateur (Operator)** | Unité d'exécution gouvernée |

### Règle fondamentale

> **Un Service peut être porté par un Opérateur... ou par une Équipe d'Opérateurs.**

### Implications

- L'utilisateur voit des **Services**
- Le système exécute via des **Opérateurs**
- La complexité est gérée par **collaboration**, pas par accumulation

---

## 3. Équipe d'Opérateurs (Operator Team)

### Définition canonique

> **An Operator Team is a governed collective of Operators that collaborate under explicit rules to deliver a Service.**

**En français :**

> **Une Équipe d'Opérateurs est un collectif gouverné d'Opérateurs qui collaborent sous règles explicites pour délivrer un Service.**

### Ce qu'une Équipe d'Opérateurs N'EST PAS

| ❌ N'est pas | Pourquoi |
|--------------|----------|
| Un nouvel Opérateur | Pas une entité d'exécution |
| Un produit | Pas une unité livrable |
| Une hiérarchie libre | Règles explicites obligatoires |

**👉 C'est une structure d'orchestration supérieure.**

### Composition d'une Équipe

Une équipe contient :

| Élément | Description |
|---------|-------------|
| **Plusieurs Opérateurs** | Minimum 2 |
| **Hétérogénéité** | Sécurité, responsabilités, exposition différentes |
| **Contrat d'Équipe** | Règles de collaboration |
| **Validation StrongFather** | Règles approuvées |

### Règle de communication

> **📌 Aucun Opérateur ne parle librement à un autre.**

Toute communication entre Opérateurs :
- Passe par BondingBrother
- Est définie dans le Contrat d'Équipe
- Est autorisée par un Mandat de Permission

---

## 4. Contrat d'Équipe (Team Contract)

### Définition

Le **Contrat d'Équipe** définit les règles de collaboration possibles entre Opérateurs d'une même équipe.

### Contenu du Contrat

| Élément | Description |
|---------|-------------|
| **Opérateurs membres** | Liste des Opérateurs de l'équipe |
| **Flux autorisés** | Qui peut parler à qui |
| **Direction des flux** | Sens de communication |
| **Types d'échanges** | Nature des interactions |
| **Types de données** | Données échangeables |
| **Conditions** | Prérequis pour les échanges |
| **Niveau de validation** | Exigences de gouvernance |

### Caractéristiques

| Propriété | Valeur |
|-----------|--------|
| **Nature** | Statique |
| **Définition** | À la conception |
| **Validation** | Par StrongFather |
| **Modification** | Processus formel |

### Règle clé

> **👉 Le contrat est validé UNE FOIS, pas à chaque appel.**

---

## 5. Mandat de Permission (Allow Mandate)

### Définition canonique (EN)

> **An Allow Mandate is a bounded authorization issued by StrongFather that allows a defined set of Operators to collaborate under explicit conditions without requiring repeated governance checks.**

### Définition canonique (FR)

> **Un Mandat de Permission est une autorisation déléguée, temporaire et encadrée, émise par StrongFather, qui permet à des Opérateurs de collaborer sans repasser en permanence par la gouvernance centrale.**

### Pourquoi "Mandat" est le bon terme

| Caractéristique | ✅ Présent |
|-----------------|-----------|
| Autorité déléguée (pas liberté) | ✅ |
| Cadre explicite | ✅ |
| Révocable | ✅ |
| Temporaire ou conditionnel | ✅ |
| Institutionnel (juridique/politique) | ✅ |
| Pas technique, pas bas niveau | ✅ |

### Règle fondatrice

> **An Allow Mandate is not an optimization. It is a delegated act of governance.**

> **Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance délégué.**

### Contenu d'un Mandat de Permission

| Élément | Description |
|---------|-------------|
| **ID unique** | Identifiant du mandat |
| **Opérateurs autorisés** | Liste des Opérateurs mandatés |
| **Flux autorisés** | Qui parle à qui |
| **Types de données** | Données échangeables |
| **Niveau de sécurité maximum** | Plafond de sécurité |
| **Conditions de validité** | Quand le mandat est valide |
| **Règles de révocation** | Quand le mandat expire |

### Ce qu'un Mandat de Permission N'EST PAS

| ❌ N'est pas | Pourquoi |
|--------------|----------|
| Un token libre | Cadre strict |
| Une session classique | Pas une authentification |
| Un cache de décision | Pas une optimisation technique |
| Un droit implicite | Toujours explicite |
| Une permission globale | Toujours borné |

---

## 6. Cycle d'Utilisation avec Mandat de Permission

### Phase 1 : Initialisation du Service

```
Utilisateur
    │
    ▼
Requête de Service
    │
    ▼
┌───────────────────────────────────────┐
│  StrongFather :                       │
│  - Identifie les Opérateurs           │
│  - Vérifie niveaux de sécurité        │
│  - Vérifie cohérence de l'équipe      │
│  - Vérifie règles WorrySentinel       │
└───────────────────────────────────────┘
    │
    ▼
📜 Émission du Mandat de Permission
```

### Phase 2 : Phase Opérationnelle (⚡ haute performance)

Pendant que le Mandat est valide :

```
Équipe d'Opérateurs (active)
    │
    ▼
┌───────────────────────────────────────┐
│  Communication via BondingBrother     │
│  - Sans appeler StrongFather          │
│  - Respect strict du mandat           │
└───────────────────────────────────────┘
    │
    ▼
Tools & Toolkits
    │
    ▼
Résultats
```

**👉 Performances prévisibles et élevées**
**👉 Gouvernance préservée**

### Phase 3 : Fin, Rupture ou Anomalie

Le mandat est **immédiatement révoqué** si :

| Condition | Effet |
|-----------|-------|
| Service terminé | Révocation normale |
| Condition hors cadre | Révocation de sécurité |
| Violation de règle | Révocation immédiate |
| Alerte WorrySentinel | Révocation d'urgence |
| Utilisateur quitte le flux | Révocation normale |
| Environnement change | Révocation de contexte |

**➡️ Retour obligatoire à StrongFather**

---

## 7. Relation Contrat d'Équipe / Mandat de Permission

| Élément | Nature | Rôle |
|---------|--------|------|
| **Contrat d'Équipe** | Statique | Décrit la collaboration possible |
| **Mandat de Permission** | Dynamique | Autorise une instance réelle |

### Règle clé

> **Une Équipe d'Opérateurs ne peut exister opérationnellement que sous un Mandat de Permission valide.**

- L'équipe n'est pas "active" par défaut
- Elle est **mandatée**

---

## 8. Sécurité Hétérogène

### Principe fondamental

> **Un Opérateur n'a qu'un seul niveau de sécurité.**
> **Une Équipe peut en combiner plusieurs.**

### Exemple concret

| Opérateur | Rôle | Sécurité |
|-----------|------|----------|
| Opérateur UI | Affichage | 🟢 Faible (1) |
| Opérateur Contenu | CMS | 🟡 Moyen (2) |
| Opérateur Auth | Identité | 🔴 Élevé (3) |
| Opérateur Audit | Logs | 🔴 Élevé (3) |

### Résultat

- UI rapide
- CMS flexible
- Auth ultra sécurisé
- **Risque segmenté**

### Règles absolues de sécurité

| Règle | Statut |
|-------|--------|
| Un Opérateur ne peut jamais élever son niveau | **NON NÉGOCIABLE** |
| Un flux ne peut jamais descendre en sécurité | **NON NÉGOCIABLE** |
| Les ponts entre niveaux sont explicites | **NON NÉGOCIABLE** |
| Les ponts entre niveaux sont rares | **NON NÉGOCIABLE** |
| Les ponts entre niveaux sont auditables | **NON NÉGOCIABLE** |
| Les ponts sont validés par WorrySentinel | **NON NÉGOCIABLE** |

---

## 9. Schéma Mental Complet

```
┌─────────────────────────────────────────────────────────┐
│                    UTILISATEUR                          │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│              REQUÊTE DE SERVICE                         │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                   STRONGFATHER                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │ • Identification des Opérateurs                  │   │
│  │ • Vérification sécurité                         │   │
│  │ • Vérification Contrat d'Équipe                 │   │
│  │ • Consultation WorrySentinel                    │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│           📜 MANDAT DE PERMISSION ÉMIS                  │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│              ÉQUIPE D'OPÉRATEURS (active)               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐             │
│  │ Op. UI   │  │ Op. CMS  │  │ Op. Auth │             │
│  │   🟢     │  │   🟡     │  │   🔴     │             │
│  └──────────┘  └──────────┘  └──────────┘             │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  BONDINGBROTHER                         │
│         (médiation sans gouvernance répétée)            │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                 TOOLS & TOOLKITS                        │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                     RÉSULTATS                           │
└─────────────────────────────────────────────────────────┘
```

---

## 10. Résumé des Règles Fondamentales

| # | Règle | Statut |
|---|-------|--------|
| 1 | Pas d'Opérateur monolithique | **NON NÉGOCIABLE** |
| 2 | Des Opérateurs spécialisés | **OBLIGATOIRE** |
| 3 | Des Équipes d'Opérateurs pour les Services complexes | **RECOMMANDÉ** |
| 4 | Des Services perçus par l'utilisateur | **OBLIGATOIRE** |
| 5 | Des Mandats de Permission pour la performance | **OBLIGATOIRE** |
| 6 | Sécurité segmentée, pas globale | **NON NÉGOCIABLE** |

---

## 11. Vocabulaire Officiel

### Correspondance terminologique

| Anglais | Français | Définition |
|---------|----------|------------|
| Allow Mandate | Mandat de Permission | Autorisation déléguée et encadrée |
| Operator Team | Équipe d'Opérateurs | Collectif gouverné d'Opérateurs |
| Team Contract | Contrat d'Équipe | Règles de collaboration statiques |
| Service | Service | Capacité perçue par l'utilisateur |
| Mandated Collaboration | Collaboration Mandatée | Coopération sous mandat |
| Mandated Path | Chemin Mandaté | Flux autorisé par mandat |
| Operator | Opérateur | Entité fonctionnelle gouvernée |

### Termes obsolètes

| ❌ Ancien terme | ✅ Nouveau terme |
|-----------------|------------------|
| Decision Window | Mandat de Permission |
| Temporary Decision | Autorisation Mandatée |
| Fast Path | Chemin Mandaté |
| Operator Collaboration (libre) | Collaboration Mandatée |
| Operator | Opérateur |

---

## 12. Phrases Fondatrices

### Complexité

> **In Miyukini, complexity is handled by collaboration, not accumulation.**

> **Dans Miyukini, la complexité est gérée par la collaboration, pas par l'accumulation.**

### Mandat de Permission

> **An Allow Mandate is not an optimization. It is a delegated act of governance.**

> **Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance délégué.**

### Sécurité

> **Risque segmenté, pas sécurité uniforme.**

---

**Date de création :** 2026-01-27  
**Version :** 1.1 (terminologie Opérateur)  
**Statut :** Document de référence normatif

**Références croisées :**
- [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Dictionnaire officiel
- [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) : Définition des Opérateurs
- [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) : Gouvernance des capacités
- [StrongFather - Documentation Fondatrice](../core/StrongFather/StrongFather%20-%20Documentation%20Fondatrice.md) : Émetteur des Mandats
- [WorrySentinel - Documentation Fondatrice](../core/WorrySentinel/WorrySentinel%20-%20Documentation%20Fondatrice.md) : Validation sécurité
- [BondingBrother - Documentation Fondatrice](../core/BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md) : Médiation
- [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) : Architecture en strates
