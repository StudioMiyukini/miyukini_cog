# Miyukini Conceptual References — Ecosystem Dependency Contract

## 1. Introduction

### Objet du document

Ce document définit le **Ecosystem Dependency Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles de gouvernance de l'écosystème Miyukini, la distinction entre le socle non substituable et les strates extensibles, et les obligations des développeurs tiers.

Ce contrat précise ce qui est strictement gouverné par Miyukini, ce qui peut être étendu par des développeurs externes, et comment cette extension doit être réalisée.

### Portée

Ce contrat s'applique à **toute production logicielle** dans l'écosystème Miyukini et définit de manière absolue :
- La distinction entre socle non substituable (Strates 0-5) et strates extensibles (Strates 6-7)
- Les obligations des développeurs tiers
- Les interdictions absolues
- Les protocoles et interfaces obligatoires
- Les garanties de sécurité structurelle

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Doctrine fondamentale

**"Miyukini n'est pas une bibliothèque. C'est un environnement gouverné dans lequel des Opérateurs existent."**

**👉 Ce n'est pas un framework "open-ended".**  
**👉 C'est un écosystème gouverné.**

---

## 2. Dépendance Verticale — Structure

### 2.1 Schéma de Gouvernance

```
┌──────────────────────────────────────────────┐
│ 🔺 STRATES 6-7 : EXTENSION AUTORISÉE          │
│ (Outils, Kits d'Outils & Opérateurs)         │
│ → Développeurs tiers autorisés                │
│ → Cadre Miyukini imposé                       │
└──────────────────────────────────────────────┘
                    ▲
                    │ (via Interfaces officielles)
┌──────────────────────────────────────────────┐
│ 🔻 STRATES 0-5 : SOCLE NON SUBSTITUABLE       │
│ (Hardware → Interfaces)                      │
│ → Miyukini only                               │
│ → Aucune substitution                         │
└──────────────────────────────────────────────┘
```

### 2.2 Strates 0 → 5 : Socle Non Substituable

**Règle absolue :** Aucune implémentation externe ne peut remplacer ou court-circuiter ces strates.

| Strate | Nature | Dépendance | Substitution |
|--------|--------|------------|--------------|
| **Strate 0 — Hardware & OS** | Physique | Contrainte matérielle | ❌ Impossible |
| **Kernel** | Technique | Miyukini only | ❌ Interdite |
| **Strate 3 — Invariants & Contrats** | Conceptuelle | Miyukini only | ❌ Interdite |
| **Strate 4 — Cores Système** | Conceptuelle | Miyukini only | ❌ Interdite |
| **Strate 5 — Interfaces & Adaptation** | Technique | Miyukini only | ❌ Interdite |

**Caractéristiques :**
- **Strictement dépendantes de l'écosystème Miyukini**
- **Aucune implémentation externe autorisée**
- **Aucun contournement possible**
- **Gouvernance exclusive par Miyukini**

**Justification :**
- Garantie de sécurité structurelle
- Garantie de cohérence décisionnelle
- Garantie d'autonomie matérielle
- Garantie de traçabilité complète

### 2.3 Strates 6 → 7 : Extension Autorisée, Cadre Imposé

**Règle fondamentale :** Les développeurs tiers peuvent créer des Outils et Opérateurs, mais **dans le cadre strict de Miyukini**.

#### Autorisations

**Développeurs tiers PEUVENT :**
- ✅ Créer des Outils et Kits d'Outils (Strate 6)
- ✅ Créer des Opérateurs (Strate 7)
- ✅ Créer les deux
- ✅ Composer des modules existants
- ✅ Définir des UX
- ✅ Gérer du métier spécifique

#### Contraintes Obligatoires

**Développeurs tiers DOIVENT :**
- ✅ Respecter les protocoles Miyukini
- ✅ Passer par les interfaces officielles (Strate 5)
- ✅ Accepter les limitations volontaires
- ✅ Se conformer aux contrats système
- ✅ Utiliser BondingBrother pour toute interaction avec les cores
- ✅ Respecter les invariants de la Strate 3

**Développeurs tiers NE PEUVENT PAS :**
- ❌ Bypasser StrongFather (décisions)
- ❌ Persister arbitrairement (contourner KindMother)
- ❌ Introduire de la logique implicite
- ❌ Modifier l'état global sans Caring Nanny
- ❌ Sortir des frontières Border Guard
- ❌ Outrepasser Master Butler (capacités)
- ❌ Accéder directement aux cores (Strate 4)
- ❌ Créer des dépendances inverses
- ❌ Implémenter des fonctionnalités "sauvages"

**Principe :** Les développeurs tiers ne codent pas "au-dessus" de Miyukini, ils codent "à l'intérieur" de Miyukini.

---

## 3. Modèle Conceptuel

### 3.1 Comparaison avec Systèmes Forts

Miyukini suit le modèle conceptuel des systèmes forts gouvernés :

| Système | Équivalent Miyukini | Rôle |
|---------|---------------------|------|
| **OS** | Kernel + Cores | Fondation technique et conceptuelle |
| **JVM** | Protocoles + Invariants | Environnement d'exécution gouverné |
| **Unreal Engine** | Cadre complet, pas juste moteur | Plateforme de développement |
| **Kubernetes** | Gouvernance, pas app | Orchestration et contrôle |

**Différence clé :** Miyukini contrôle la philosophie architecturale, pas seulement l'implémentation.

### 3.2 Positionnement des Couches Externes

```
Développeur tiers
        │
        ▼
┌─────────────────────────┐
│  Opérateur              │ ──┐
│  (Strate 7)             │   │
└─────────────────────────┘   │
                              ├─► via Interfaces Miyukini
┌─────────────────────────┐   │   (Strate 5)
│  Outils & Kits d'Outils │ ──┘
│  (Strate 6)             │
└─────────────────────────┘
        │
        ▼
┌─────────────────────────┐
│  Miyukini Ecosystem     │
│  (Strates 0-5)          │
│  → Socle gouverné      │
└─────────────────────────┘
```

**Règles absolues :**
- ✅ Aucun accès direct aux cores
- ✅ Aucune dépendance inverse
- ✅ Aucune implémentation sauvage
- ✅ Toute interaction passe par les interfaces officielles

---

## 4. Obligations des Développeurs Tiers

### 4.1 Protocoles Obligatoires

**Tout développeur tiers DOIT :**

1. **Utiliser BondingBrother** pour toute interaction avec les cores
   - Pas d'accès direct à StrongFather
   - Pas d'accès direct à KindMother
   - Pas d'accès direct aux autres cores

2. **Respecter les contrats système**
   - Intent Model Contract (StrongFather)
   - CoreDataAPI Contract (KindMother)
   - Tous les contrats FONDATION

3. **Passer par les interfaces officielles**
   - Interfaces de la Strate 5 uniquement
   - Pas de contournement
   - Pas d'implémentation directe

4. **Accepter les limitations volontaires**
   - Pas de persistance arbitraire
   - Pas de décisions hors StrongFather
   - Pas de modification d'état global sans Caring Nanny

### 4.2 Interdictions Absolues

**Aucun développeur tiers NE PEUT :**

#### 4.2.1 Court-circuiter StrongFather

**Interdiction :** Bypasser le moteur de décision stratégique.

**Raison :** Garantie de cohérence décisionnelle globale.

**Conséquence :** Toute décision doit passer par StrongFather via BondingBrother.

#### 4.2.2 Contourner KindMother

**Interdiction :** Persister des données sans passer par KindMother.

**Raison :** Garantie de cohérence des données et synchronisation.

**Conséquence :** Toute persistance doit passer par KindMother via les adaptateurs produits.

#### 4.2.3 Ignorer Caring Nanny

**Interdiction :** Modifier l'état global sans informer Caring Nanny.

**Raison :** Garantie d'observabilité et de diagnostic.

**Conséquence :** Toute modification d'état doit être observable par Caring Nanny.

#### 4.2.4 Sortir des Frontières Border Guard

**Interdiction :** Définir des frontières ou niveaux de confiance hors Border Guard.

**Raison :** Garantie de sécurité et de classification cohérente.

**Conséquence :** Toute définition de frontière doit respecter Border Guard.

#### 4.2.5 Outrepasser Master Butler

**Interdiction :** Définir des capacités ou permissions hors Master Butler.

**Raison :** Garantie de registre centralisé des possibilités.

**Conséquence :** Toute capacité doit être enregistrée dans Master Butler.

#### 4.2.6 Accès Direct aux Cores

**Interdiction :** Accéder directement aux cores (Strate 4) sans passer par les interfaces (Strate 5).

**Raison :** Garantie de médiation et de traçabilité.

**Conséquence :** Toute interaction avec les cores passe par BondingBrother (Strate 5).

### 4.3 Ce qui est Autorisé

**Développeurs tiers PEUVENT :**

#### 4.3.1 Créer des Produits Intermédiaires

**Autorisation :** Créer des capacités produits prêtes à l'emploi, recomposables.

**Exemples :**
- Auth / Identity
- Billing Core
- Content Engine
- Realtime Engine
- Workflow Engine
- Notification
- Search / Index

**Contraintes :**
- Doivent utiliser les interfaces officielles (Strate 5)
- Doivent respecter les protocoles Miyukini
- Ne peuvent pas contenir de logique métier spécifique client

#### 4.3.2 Créer des Produits Finis

**Autorisation :** Créer des produits livrables aux clients finaux.

**Exemples :**
- SaaS complets
- Applications mobiles/desktop/web
- Jeux
- CMS complets
- Outils métier spécialisés

**Contraintes :**
- Doivent combiner des produits intermédiaires (Strate 6)
- Peuvent contenir de la logique métier spécifique
- Doivent utiliser les interfaces officielles (Strate 5)

#### 4.3.3 Composer des Modules

**Autorisation :** Combiner des produits intermédiaires existants.

**Exemples :**
- Combiner Auth + Billing + Content pour créer un CMS
- Combiner Realtime + Notification pour créer un système de communication

**Contraintes :**
- Doivent respecter les contrats des produits intermédiaires
- Doivent passer par les interfaces officielles

#### 4.3.4 Définir des UX

**Autorisation :** Créer des interfaces utilisateur personnalisées.

**Exemples :**
- UI web personnalisée
- CLI spécialisée
- API REST/GraphQL personnalisée

**Contraintes :**
- Doivent utiliser les Outils, Kits d'Outils ou Opérateurs (Strates 6-7)
- Ne peuvent pas accéder directement aux cores

---

## 5. Garanties de Sécurité Structurelle

### 5.1 Protection contre les Mauvaises Implémentations

**Garantie :** Même un développeur inexpérimenté ou malveillant ne peut pas :

- ❌ Casser le système
- ❌ Corrompre les décisions
- ❌ "Bidouiller" les cores
- ❌ Introduire des failles de sécurité structurelles
- ❌ Contourner les invariants

**Mécanisme :** Les interfaces officielles (Strate 5) et les contrats système empêchent tout accès direct aux cores.

### 5.2 Scalabilité Humaine

**Garantie :** L'écosystème peut être ouvert aux contributions externes sans perdre le contrôle.

**Bénéfices :**
- ✅ Acceptation de contributions tierces
- ✅ Industrialisation possible
- ✅ Écosystème extensible
- ✅ Contrôle maintenu sur le socle

**Mécanisme :** La séparation stricte entre socle gouverné (Strates 0-5) et strates extensibles (Strates 6-7) garantit que les contributions externes ne peuvent pas compromettre le socle.

### 5.3 Autonomie Matérielle

**Garantie :** L'autonomie matérielle est préservée même avec des Outils et Opérateurs tiers.

**Bénéfices :**
- ✅ Pas de dépendance cloud imposée
- ✅ Pas d'API magique externe
- ✅ Tout peut tourner local / isolé
- ✅ Compatible hardware faible

**Mécanisme :** Les Outils et Opérateurs tiers utilisent les mêmes interfaces que ceux de Miyukini, garantissant que l'autonomie structurelle est préservée.

---

## 6. Vérification et Conformité

### 6.1 Vérification de Conformité

**Pour vérifier qu'un Outil ou Opérateur tiers est conforme :**

1. **Vérifier l'utilisation de BondingBrother**
   - Toute interaction avec les cores passe-t-elle par BondingBrother ?
   - Y a-t-il des accès directs aux cores ?

2. **Vérifier le respect des contrats**
   - Les contrats FONDATION sont-ils respectés ?
   - Y a-t-il des violations d'invariants ?

3. **Vérifier l'utilisation des interfaces officielles**
   - Les interfaces de la Strate 5 sont-elles utilisées ?
   - Y a-t-il des contournements ?

4. **Vérifier l'absence de dépendances inverses**
   - Les cores dépendent-ils des Outils ou Opérateurs ?
   - Y a-t-il des couplages interdits ?

### 6.2 Non-Conformité

**En cas de non-conformité :**

- L'Outil ou Opérateur ne peut pas être intégré à l'écosystème
- Les violations doivent être corrigées avant intégration
- Aucune exception n'est autorisée

**Documentation associée :**
- [BondingBrother - Documentation Fondatrice](../core/BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [StrongFather - Documentation Fondatrice](../core/StrongFather/StrongFather%20-%20Documentation%20Fondatrice.md)
- [KindMother - Documentation Fondatrice](../core/KindMother/KindMother%20-%20Documentation%20Fondatrice.md)

---

## 7. Conclusion

Le Ecosystem Dependency Contract établit que Miyukini est un **environnement gouverné**, pas une bibliothèque open-ended. Cette gouvernance garantit :

- **Sécurité structurelle** : Même les mauvaises implémentations ne peuvent pas casser le système
- **Scalabilité humaine** : L'écosystème peut être ouvert sans perdre le contrôle
- **Autonomie matérielle** : L'autonomie est préservée même avec des Outils et Opérateurs tiers
- **Cohérence globale** : Tous les Outils et Opérateurs respectent les mêmes règles et protocoles

Cette gouvernance est la garantie que l'écosystème reste cohérent, sécurisé, et évolutif, même avec des contributions externes.

---

**Date de création :** 2026-01-26  
**Version :** 1.0  
**Statut :** Contrat FONDATION, non négociable

**Documentation associée :**
- [Miyukini Conceptual References - Pyramide Architecture Complète](Miyukini%20Framework%20-%20Pyramide%20Architecture%20Complete.md) : Section 5.2 — Gouvernance d'Écosystème
- [Miyukini Conceptual References - Vision Stratégique](Miyukini%20Framework%20-%20Vision%20Strategique.md) : Section 8 — Principe de Gouvernance d'Écosystème
- [Miyukini Conceptual References - Integrity & Degradation System](Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md) : Système de dégradation graduée (T0-T4)
- [Miyukini Conceptual References - External Signal & Trust Reinforcement Contract](Miyukini%20Framework%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md) : Intégration Internet comme signal externe
- [Miyukini Conceptual References - Security Protocols](Miyukini%20Framework%20-%20Security%20Protocols.md) : Protocoles de sécurité pour développeurs tiers
- [Miyukini Conceptual References - Security Performance Impact](Miyukini%20Framework%20-%20Security%20Performance%20Impact.md) : Impact performance des protocoles de sécurité
- [Miyukini Conceptual References - Security Levels](Miyukini%20Framework%20-%20Security%20Levels.md) : Niveaux de sécurité (0-4) - Opérateurs déclarent, cores gouvernent
