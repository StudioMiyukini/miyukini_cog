# Border Guard - Boundary Definition Contract

## 1. Contexte

Ce document définit les **types de frontières** reconnus par Border Guard dans l'écosystème Miyukini. Il spécifie formellement ce qu'est une frontière, ses propriétés, ses caractéristiques, et la taxonomie complète des types de frontières.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non négociable**. Il dérive directement de la Documentation Fondatrice (Section 4 - Concepts fondamentaux : Frontière).

---

## 2. Portée / Scope

- **Applicable à :** Toute démarcation conceptuelle entre zones de confiance dans l'écosystème Miyukini
- **Responsable :** Border Guard (responsabilité exclusive de définition des frontières - INV-BG-5)
- **Consommateurs :** StrongFather (contexte de confiance), BondingBrother (application des règles), CaringNanny (état des frontières)
- **Ne couvre pas :** L'application technique des frontières (responsabilité de BondingBrother)

---

## 3. Définition canonique de la frontière

### 3.1 Qu'est-ce qu'une frontière ?

Une **frontière** est une démarcation conceptuelle qui sépare deux zones de confiance différentes. Elle représente le point de transition entre un niveau de confiance et un autre.

**Caractéristiques fondamentales :**

1. **Conceptuelle** — Une frontière est une abstraction, pas une implémentation technique
2. **Explicite** — Toute frontière doit être formellement définie et documentée (INV-BG-5)
3. **Stable** — Une frontière possède une identité unique et pérenne
4. **Orientée** — Une frontière a une direction (entrée, sortie, bidirectionnelle)
5. **Perméable** — Une frontière a un niveau de perméabilité définissant sa propension au franchissement

**Ce qu'une frontière n'est PAS :**

- ❌ Un firewall technique
- ❌ Une règle de filtrage
- ❌ Un point de contrôle d'authentification
- ❌ Une implémentation de sécurité

### 3.2 Responsabilité de Border Guard

Border Guard est **exclusivement responsable** de la définition formelle des frontières du système. Cette responsabilité inclut :

- Identifier et nommer chaque frontière
- Classifier la nature de chaque frontière (externe, interne, intégration)
- Définir la direction de chaque frontière (entrée, sortie, bidirectionnelle)
- Établir le niveau de perméabilité de chaque frontière
- Maintenir le registre exhaustif des frontières du système

**Invariant associé :** INV-BG-5 — Toute frontière **doit** être explicitement définie et documentée. Aucune frontière implicite n'est autorisée.

---

## 4. Propriétés d'une frontière

Toute frontière possède les propriétés obligatoires suivantes :

### 4.1 Identité

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Identifiant** | Identifiant unique et stable dans le système | ✅ Oui |
| **Nom** | Nom descriptif et non ambigu | ✅ Oui |
| **Description** | Description de la frontière et de sa raison d'être | ✅ Oui |
| **Date de création** | Horodatage de création de la frontière | ✅ Oui |

### 4.2 Classification

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Type** | Type de frontière (externe, interne, intégration) | ✅ Oui |
| **Zone source** | Zone de confiance côté source | ✅ Oui |
| **Zone destination** | Zone de confiance côté destination | ✅ Oui |

### 4.3 Comportement

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Direction** | Direction du flux autorisé (entrée, sortie, bidirectionnelle) | ✅ Oui |
| **Perméabilité** | Niveau de perméabilité (ouverte, contrôlée, fermée) | ✅ Oui |
| **Règles associées** | Références aux règles de franchissement applicables | ✅ Oui |

### 4.4 Traçabilité

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Origine** | Qui a créé cette frontière | ✅ Oui |
| **Justification** | Pourquoi cette frontière existe | ✅ Oui |
| **Historique** | Historique des modifications | ✅ Oui |

**Invariant associé :** INV-BG-8 — Toute définition de frontière est **traçable** avec son origine, sa date, et sa justification.

---

## 5. Taxonomie des types de frontières

Border Guard reconnaît trois types canoniques de frontières.

### 5.1 Frontière externe

**Définition :** Sépare l'écosystème Miyukini du monde extérieur (internet, systèmes tiers, utilisateurs non authentifiés). C'est la limite entre le "dehors" et le "dedans".

| Aspect | Spécification |
|--------|---------------|
| **Zone source** | Monde extérieur (unknown ou hostile par défaut) |
| **Zone destination** | Écosystème Miyukini |
| **Confiance par défaut** | Unknown (aucune confiance accordée a priori) |
| **Direction typique** | Entrée (flux venant de l'extérieur vers l'intérieur) |
| **Perméabilité typique** | Contrôlée (vérifications systématiques) |

**Exemples de frontières externes :**

- Frontière API publique — Point d'entrée des requêtes HTTP externes
- Frontière utilisateur non authentifié — Point d'entrée des utilisateurs anonymes
- Frontière webhook — Point d'entrée des notifications externes
- Frontière réseau — Point d'entrée des connexions réseau

**Implications :**

- Tout ce qui traverse une frontière externe est présumé "unknown" jusqu'à classification
- Les règles de franchissement sont restrictives par défaut
- Bonding Brother applique des contrôles systématiques

### 5.2 Frontière interne

**Définition :** Sépare différentes zones de confiance au sein de l'écosystème (zone admin vs zone utilisateur, module sensible vs module standard, données critiques vs données publiques).

| Aspect | Spécification |
|--------|---------------|
| **Zone source** | Zone interne avec niveau de confiance X |
| **Zone destination** | Zone interne avec niveau de confiance Y (X ≠ Y) |
| **Confiance par défaut** | Héritée de la zone source |
| **Direction typique** | Bidirectionnelle (selon les règles) |
| **Perméabilité typique** | Variable (selon les zones) |

**Exemples de frontières internes :**

- Frontière admin/utilisateur — Entre l'espace d'administration et l'espace utilisateur
- Frontière données sensibles — Entre les données critiques et les données standard
- Frontière cores — Entre différents cores du système (sauf pour les flux explicites)
- Frontière module critique — Autour d'un module à haute sécurité (niveau 3-4)

**Implications :**

- Les frontières internes permettent la défense en profondeur
- Chaque zone interne peut avoir ses propres règles de franchissement
- La confiance peut varier entre zones internes

### 5.3 Frontière d'intégration

**Définition :** Sépare l'écosystème d'un système externe avec lequel il interagit de manière contrôlée (API partenaire, service tiers, base de données externe).

| Aspect | Spécification |
|--------|---------------|
| **Zone source** | Écosystème Miyukini ou système externe intégré |
| **Zone destination** | Système externe intégré ou écosystème Miyukini |
| **Confiance par défaut** | Selon classification de l'intégration (verified typiquement) |
| **Direction typique** | Bidirectionnelle (échanges avec le système intégré) |
| **Perméabilité typique** | Contrôlée (protocoles d'intégration) |

**Exemples de frontières d'intégration :**

- Frontière Supabase — Avec le backend Supabase
- Frontière API partenaire — Avec une API tierce certifiée
- Frontière service de paiement — Avec un processeur de paiement (Stripe, etc.)
- Frontière service d'authentification — Avec un IdP externe (OAuth, SAML)

**Implications :**

- Une intégration peut être classifiée "verified" si elle respecte les protocoles
- L'état de l'intégration peut être signalé à CaringNanny
- Les règles de franchissement sont spécifiques à chaque intégration

---

## 6. Niveaux de perméabilité

La perméabilité caractérise la propension d'une frontière à autoriser le franchissement.

### 6.1 Perméabilité ouverte

**Définition :** Franchissement libre sous conditions minimales.

| Aspect | Spécification |
|--------|---------------|
| **Vérification** | Minimale (validation structurelle uniquement) |
| **Blocage** | Rare (uniquement en cas d'anomalie évidente) |
| **Usage typique** | Frontières vers des zones publiques |
| **Niveau de sécurité associé** | 0 (PUBLIC / DISPLAY) |

**Exemples :**

- Frontière vers une API publique en lecture seule
- Frontière vers des ressources statiques
- Frontière vers des données publiques

### 6.2 Perméabilité contrôlée

**Définition :** Franchissement soumis à vérification selon les règles définies.

| Aspect | Spécification |
|--------|---------------|
| **Vérification** | Systématique (selon règles de franchissement) |
| **Blocage** | Conditionnel (si règles non respectées) |
| **Usage typique** | Frontières standard, intégrations |
| **Niveau de sécurité associé** | 1-3 (STANDARD à CRITICAL) |

**Exemples :**

- Frontière utilisateur authentifié
- Frontière d'intégration avec API partenaire
- Frontière vers des données sensibles

### 6.3 Perméabilité fermée

**Définition :** Franchissement interdit sauf conditions exceptionnelles.

| Aspect | Spécification |
|--------|---------------|
| **Vérification** | Maximale (toutes les conditions doivent être satisfaites) |
| **Blocage** | Par défaut (franchissement exceptionnel) |
| **Usage typique** | Frontières vers des zones critiques, isolement |
| **Niveau de sécurité associé** | 4 (HARDENED / ISOLATED) |

**Exemples :**

- Frontière vers des clés cryptographiques
- Frontière en mode quarantaine
- Frontière vers des zones isolées en mode survie

---

## 7. Direction de franchissement

### 7.1 Entrée (Inbound)

**Définition :** Flux autorisé uniquement de l'extérieur vers l'intérieur (par rapport à la zone de confiance supérieure).

| Aspect | Spécification |
|--------|---------------|
| **Flux autorisé** | Source → Destination uniquement |
| **Usage typique** | Frontières externes, réception de données |
| **Contrôle** | Sur ce qui entre |

### 7.2 Sortie (Outbound)

**Définition :** Flux autorisé uniquement de l'intérieur vers l'extérieur.

| Aspect | Spécification |
|--------|---------------|
| **Flux autorisé** | Destination → Source uniquement |
| **Usage typique** | Envoi de données vers l'extérieur |
| **Contrôle** | Sur ce qui sort |

### 7.3 Bidirectionnel

**Définition :** Flux autorisé dans les deux sens, chaque direction pouvant avoir ses propres règles.

| Aspect | Spécification |
|--------|---------------|
| **Flux autorisé** | Source ↔ Destination |
| **Usage typique** | Intégrations, frontières internes |
| **Contrôle** | Règles distinctes par direction |

---

## 8. Zones de confiance

### 8.1 Définition

Une **zone de confiance** est un espace conceptuel délimité par des frontières, où tous les éléments partagent un même niveau de confiance.

### 8.2 Propriétés d'une zone

| Propriété | Description |
|-----------|-------------|
| **Identifiant** | Identifiant unique de la zone |
| **Niveau de confiance** | Niveau de confiance homogène (trusted, verified, unknown, hostile) |
| **Frontières** | Liste des frontières délimitant la zone |
| **Contenu** | Composants, données, services contenus dans la zone |

### 8.3 Hiérarchie des zones

Les zones de confiance sont organisées hiérarchiquement :

```
┌─────────────────────────────────────────────────────────────┐
│ ZONE EXTERNE (hostile/unknown)                              │
│   ┌─────────────────────────────────────────────────────┐   │
│   │ ZONE PÉRIPHÉRIQUE (unknown/verified)                │   │
│   │   ┌─────────────────────────────────────────────┐   │   │
│   │   │ ZONE UTILISATEUR (verified)                 │   │   │
│   │   │   ┌─────────────────────────────────────┐   │   │   │
│   │   │   │ ZONE ADMIN (verified+)              │   │   │   │
│   │   │   │   ┌─────────────────────────────┐   │   │   │   │
│   │   │   │   │ ZONE SYSTÈME (trusted)      │   │   │   │   │
│   │   │   │   │   ┌─────────────────────┐   │   │   │   │   │
│   │   │   │   │   │ ZONE CRITIQUE       │   │   │   │   │   │
│   │   │   │   │   │ (trusted isolé)     │   │   │   │   │   │
│   │   │   │   │   └─────────────────────┘   │   │   │   │   │
│   │   │   │   └─────────────────────────────┘   │   │   │   │
│   │   │   └─────────────────────────────────────┘   │   │   │
│   │   └─────────────────────────────────────────────┘   │   │
│   └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

**Règle fondamentale :** Chaque frontière sépare exactement deux zones de niveaux de confiance différents.

---

## 9. Adaptation selon les niveaux de sécurité

Les frontières s'adaptent selon le niveau de sécurité déclaré par l'Opérateur.

**Référence :** [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)

### 9.1 Adaptation par niveau

| Niveau | Perméabilité par défaut | Frontières internes | Frontières externes |
|--------|-------------------------|---------------------|---------------------|
| **0 - PUBLIC** | Ouverte | Minimales | Assouplies |
| **1 - STANDARD** | Contrôlée | Standard | Standard |
| **2 - SENSITIVE** | Contrôlée renforcée | Renforcées | Renforcées |
| **3 - CRITICAL** | Strictement contrôlée | Strictes | Strictes |
| **4 - HARDENED** | Fermée par défaut | Maximales, isolement | Maximales, isolement |

### 9.2 Règles d'adaptation

| Règle | Description |
|-------|-------------|
| **RÈGLE-ADAPT-1** | Le niveau de sécurité influence la perméabilité par défaut des nouvelles frontières |
| **RÈGLE-ADAPT-2** | Une frontière peut être plus restrictive que le niveau, jamais moins |
| **RÈGLE-ADAPT-3** | L'élévation du niveau resserre automatiquement les frontières existantes |
| **RÈGLE-ADAPT-4** | La réduction du niveau ne desserre pas automatiquement les frontières |

---

## 10. Règles de définition

### 10.1 Règles obligatoires

| Règle | Description |
|-------|-------------|
| **RÈGLE-DEF-1** | Toute frontière doit être explicitement définie (INV-BG-5) |
| **RÈGLE-DEF-2** | Toute frontière doit avoir une identité unique et stable |
| **RÈGLE-DEF-3** | Toute frontière doit séparer exactement deux zones de confiance |
| **RÈGLE-DEF-4** | Toute frontière doit avoir au moins une règle de franchissement associée |
| **RÈGLE-DEF-5** | Toute définition de frontière est traçable (INV-BG-8) |

### 10.2 Anti-patterns de définition

| Anti-pattern | Description | Pourquoi c'est interdit |
|--------------|-------------|-------------------------|
| **Frontière implicite** | Frontière non déclarée formellement | Viole INV-BG-5 |
| **Frontière flottante** | Frontière sans zones clairement définies | Viole RÈGLE-DEF-3 |
| **Frontière sans règles** | Frontière sans règles de franchissement | Viole RÈGLE-DEF-4 |
| **Frontière technique** | Frontière définie par l'implémentation | Viole INV-BG-10 |

---

## 11. Interactions avec les autres cores

### 11.1 Flux vers StrongFather

Border Guard fournit à StrongFather le **contexte de frontière** pour ses décisions :

- Quelles frontières sont traversées par une intention
- Quelle est la zone source de l'intention
- Quelle est la zone destination de l'intention
- Quel niveau de confiance est associé aux zones

### 11.2 Flux vers BondingBrother

Border Guard fournit à BondingBrother les **définitions de frontières** :

- Type, direction, perméabilité de chaque frontière
- Règles de franchissement applicables
- État actuel des frontières (via CaringNanny)

### 11.3 Flux vers CaringNanny

Border Guard informe CaringNanny de l'**état des frontières** :

- Création, modification, suppression de frontières
- Changement d'état d'une frontière (ex: passage en mode fermé)
- Anomalies détectées sur une frontière

---

## 12. Références croisées

### Invariants associés (Documentation Fondatrice - Section 7)

| Invariant | Énoncé | Relation |
|-----------|--------|----------|
| INV-BG-5 | Frontières explicites | Fondement de ce contrat |
| INV-BG-8 | Traçabilité complète | Toute frontière est traçable |
| INV-BG-9 | Cohérence globale | Pas de contradiction entre frontières |
| INV-BG-10 | Neutralité conceptuelle | Pas de supposition technique |

### Documents associés

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Border Guard - Trust Level Classification Contract](./Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Niveaux de confiance des zones |
| [Border Guard - Crossing Rules Contract](./Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | Règles de franchissement |
| [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Adaptation selon niveau sécurité |

### Références glossaire

| Terme | Définition |
|-------|------------|
| **Frontière** | Démarcation conceptuelle entre deux zones de confiance différentes |
| **Zone de confiance** | Espace conceptuel où tous les éléments partagent un niveau de confiance homogène |
| **Perméabilité** | Propension d'une frontière à autoriser le franchissement |
| **Frontière externe** | Sépare l'écosystème du monde extérieur |
| **Frontière interne** | Sépare différentes zones au sein de l'écosystème |
| **Frontière d'intégration** | Sépare l'écosystème d'un système externe intégré |

**Source :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 13. Synthèse contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Les frontières sont définies** — Trois types canoniques avec propriétés explicites
2. **Les zones sont claires** — Chaque frontière sépare exactement deux zones
3. **La perméabilité est classifiée** — Trois niveaux (ouverte, contrôlée, fermée)
4. **L'adaptation est automatique** — Les frontières s'adaptent au niveau de sécurité
5. **La traçabilité est complète** — Toute frontière est documentée et traçable

### Phrase de synthèse

> **Une frontière est une démarcation conceptuelle, explicite et traçable, qui sépare deux zones de confiance différentes et dont la perméabilité s'adapte au niveau de sécurité du système.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Référence :** Border Guard v1.5, Documentation Fondatrice Section 4  
**Type :** Contrat de définition de frontières
