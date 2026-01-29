# Border Guard — FAQ & Common Questions

## Contexte

Ce document répond aux **questions fréquentes** concernant Border Guard. Il clarifie les points de confusion courants et fournit des réponses concises et précises basées sur les contrats FONDATION.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut :** Document de référence — Informatif

---

## Portée / Scope

- **Applicable à :** Développeurs, architectes, et toute personne travaillant avec Border Guard
- **Objectif :** Répondre rapidement aux questions courantes sans relire toute la documentation
- **Usage :** Consultation rapide, onboarding, clarification de doutes

---

## Questions générales

### Q1 : Qu'est-ce que Border Guard en une phrase ?

**Réponse :** Border Guard est l'autorité de définition des frontières et des niveaux de confiance qui établit les règles de franchissement sans jamais les appliquer lui-même, séparant strictement la définition conceptuelle de l'exécution technique.

---

### Q2 : Quelle est la question fondamentale à laquelle Border Guard répond ?

**Réponse :** "Où sont les frontières du système, et quelles règles gouvernent leur franchissement ?"

Cette question se décline en :
- Qu'est-ce qui est "interne" et qu'est-ce qui est "externe" ?
- Quel niveau de confiance accorder à une source ou une destination ?
- Quelles conditions doivent être respectées pour franchir une frontière ?
- Comment classifier les intégrations selon leur nature et leur risque ?

---

### Q3 : Quelle est la strate de Border Guard dans l'architecture Miyukini ?

**Réponse :** Border Guard est dans la **Strate 2 — Frontière**. Son rôle est la définition des frontières et la classification de confiance.

---

### Q4 : Border Guard est-il un composant technique ou conceptuel ?

**Réponse :** Border Guard est un **core conceptuel**. Il ne possède aucune capacité d'exécution. Il définit, classifie, et établit des règles, mais n'exécute jamais rien lui-même. Toute exécution est déléguée aux autres cores (notamment BondingBrother).

---

## Questions sur les responsabilités

### Q5 : Que fait Border Guard exactement ?

**Réponse :** Border Guard a quatre responsabilités exclusives :

1. **Définition des frontières** — Identifier, nommer, et formaliser chaque frontière du système
2. **Classification des niveaux de confiance** — Classifier les sources selon trusted, verified, unknown, hostile
3. **Établissement des règles de franchissement** — Définir les conditions déclaratives pour traverser une frontière
4. **Gouvernance conceptuelle des intégrations** — Classifier et encadrer les relations avec les systèmes externes

---

### Q6 : Que ne fait PAS Border Guard ?

**Réponse :** Border Guard ne fait **jamais** :

| Interdit | Responsable |
|----------|-------------|
| Filtrer les interactions | BondingBrother |
| Bloquer les accès | BondingBrother (sur décision StrongFather) |
| Gérer l'authentification technique | Produit / Module auth |
| Persister des données | KindMother |
| Prendre des décisions stratégiques | StrongFather |
| Exécuter des actions techniques | BondingBrother / Adaptateurs |
| Modifier l'état du système | CaringNanny / Cores exécutants |
| Contenir de la logique métier | Produits |

---

### Q7 : Border Guard peut-il prendre des décisions ?

**Réponse :** **Non, jamais.** Border Guard informe, classifie, et définit, mais la décision finale appartient toujours à StrongFather ou aux autorités appropriées. C'est l'invariant INV-BG-3.

**Exemple :**
- Border Guard dit : "Cette source est classifiée 'hostile'"
- StrongFather décide : "Je refuse l'accès"
- BondingBrother exécute : "Blocage effectué"

---

### Q8 : Border Guard peut-il persister des données ?

**Réponse :** **Non, jamais directement.** Border Guard n'accède jamais directement à la persistance (INV-BG-2). Toute définition qui doit être persistée est transmise à KindMother via les canaux appropriés.

---

## Questions sur les frontières

### Q9 : Quels sont les types de frontières ?

**Réponse :** Border Guard reconnaît trois types canoniques :

| Type | Description |
|------|-------------|
| **Frontière externe** | Sépare l'écosystème du monde extérieur (internet, utilisateurs non authentifiés) |
| **Frontière interne** | Sépare différentes zones de confiance au sein de l'écosystème |
| **Frontière d'intégration** | Sépare l'écosystème d'un système externe intégré (API partenaire, Supabase) |

---

### Q10 : Une frontière peut-elle être implicite ?

**Réponse :** **Non, jamais.** Toute frontière doit être explicitement définie et documentée (INV-BG-5). Si une démarcation existe dans le système, elle doit être formalisée par Border Guard.

---

### Q11 : Qu'est-ce que la perméabilité d'une frontière ?

**Réponse :** La perméabilité caractérise la propension d'une frontière à autoriser le franchissement :

| Niveau | Description | Usage typique |
|--------|-------------|---------------|
| **Ouverte** | Franchissement libre sous conditions minimales | Frontières vers zones publiques |
| **Contrôlée** | Franchissement soumis à vérification | Frontières standard, intégrations |
| **Fermée** | Franchissement interdit sauf exceptions | Zones critiques, isolement |

---

## Questions sur les niveaux de confiance

### Q12 : Quels sont les niveaux de confiance ?

**Réponse :** Border Guard définit exactement quatre niveaux canoniques :

| Niveau | Icône | Signification |
|--------|-------|---------------|
| **Trusted** | 🟢 | Confiance totale — cercle de confiance absolu |
| **Verified** | 🔵 | Confiance vérifiée — authentifié et validé |
| **Unknown** | 🟡 | Confiance inconnue — niveau par défaut |
| **Hostile** | 🔴 | Confiance nulle — source malveillante identifiée |

---

### Q13 : Quel est le niveau par défaut si une source n'est pas classifiée ?

**Réponse :** **Unknown.** Par défaut, tout ce qui n'est pas explicitement classifié est considéré comme "unknown" (INV-BG-4). C'est un défaut sécuritaire par conception.

---

### Q14 : "Unknown" signifie-t-il "hostile" ?

**Réponse :** **Non.** "Unknown" n'est pas "hostile". C'est un état d'attente qui peut évoluer vers "verified" (après authentification) ou vers "hostile" (si un pattern malveillant est détecté).

---

### Q15 : Comment passer de "unknown" à "trusted" ?

**Réponse :** La transition vers "trusted" est **toujours progressive** (TRANS-1) :

```
UNKNOWN → VERIFIED → TRUSTED
```

On ne peut jamais passer directement de "unknown" à "trusted". Il faut d'abord être "verified", puis obtenir une certification complète pour devenir "trusted".

---

### Q16 : Comment une source devient-elle "hostile" ?

**Réponse :** Une source peut devenir "hostile" **immédiatement** depuis n'importe quel niveau (TRANS-2) si :
- Elle est blacklistée
- Un pattern d'attaque est détecté
- Une compromission est confirmée
- Une violation grave est constatée

---

### Q17 : Une source "hostile" peut-elle redevenir "verified" ?

**Réponse :** **Non, pas directement.** La réhabilitation depuis "hostile" passe obligatoirement par "unknown" (TRANS-3) :

```
HOSTILE → (réhabilitation formelle) → UNKNOWN → (auth réussie) → VERIFIED
```

---

## Questions sur les règles de franchissement

### Q18 : Qu'est-ce qu'une règle de franchissement ?

**Réponse :** Une règle de franchissement est une **condition déclarative** qui doit être satisfaite pour qu'une interaction puisse traverser une frontière. Elle exprime ce qui est requis, pas comment le vérifier techniquement.

**Exemple :**
- ✅ Déclaratif : "Niveau de confiance minimum : verified"
- ❌ Procédural : "Vérifier le token JWT et valider la signature"

---

### Q19 : Pourquoi les règles doivent-elles être déclaratives ?

**Réponse :** Pour respecter la séparation définition/application (INV-BG-7) et la neutralité conceptuelle (INV-BG-10) :
- Border Guard définit CE QUI est requis
- BondingBrother décide COMMENT le vérifier techniquement

Cela permet de changer l'implémentation technique sans modifier les règles conceptuelles.

---

### Q20 : Qui applique les règles de franchissement ?

**Réponse :** **BondingBrother.** Border Guard définit les règles (déclaratives), BondingBrother les applique (techniquement). Cette séparation est absolue et non négociable (INV-BG-7).

---

### Q21 : Quels sont les types de règles de franchissement ?

**Réponse :** Border Guard reconnaît cinq types :

| Type | Code | Description |
|------|------|-------------|
| **Niveau de confiance** | `niveau_confiance` | Niveau requis pour franchir |
| **Authentification** | `authentification` | État d'auth requis |
| **Données** | `donnees` | Nature des données autorisées |
| **Action** | `action` | Actions autorisées |
| **Temporel** | `temporel` | Contraintes de temps |

---

## Questions sur les interactions avec les autres cores

### Q22 : Quelle est la relation entre Border Guard et StrongFather ?

**Réponse :** Relation de **conseil** :
- Border Guard informe StrongFather sur le contexte de confiance
- StrongFather utilise ces informations pour prendre ses décisions
- Border Guard ne décide jamais à la place de StrongFather

---

### Q23 : Quelle est la relation entre Border Guard et BondingBrother ?

**Réponse :** Relation de **définition/application** :
- Border Guard définit les frontières et les règles de franchissement
- BondingBrother consulte ces définitions et les applique techniquement
- Border Guard ne filtre jamais, BondingBrother ne définit jamais

---

### Q24 : Quelle est la relation entre Border Guard et CaringNanny ?

**Réponse :** Relation d'**information** :
- Border Guard informe CaringNanny sur l'état des frontières
- CaringNanny intègre cette information dans l'état global du système
- Border Guard ne modifie jamais l'état global

---

### Q25 : Quelle est la relation entre Border Guard et KindMother ?

**Réponse :** Relation de **complémentarité** :
- KindMother traite les données une fois qu'elles sont "à l'intérieur"
- Border Guard définit les conditions pour qu'elles y entrent
- Border Guard délègue toute persistance à KindMother

---

## Questions sur les invariants

### Q26 : Quels sont les 10 invariants de Border Guard ?

**Réponse :**

| Invariant | Description |
|-----------|-------------|
| **INV-BG-1** | Aucune capacité d'exécution |
| **INV-BG-2** | Aucune persistance directe |
| **INV-BG-3** | Aucune décision autonome |
| **INV-BG-4** | Classification exhaustive |
| **INV-BG-5** | Frontières explicites |
| **INV-BG-6** | Règles déclaratives |
| **INV-BG-7** | Séparation définition/application |
| **INV-BG-8** | Traçabilité complète |
| **INV-BG-9** | Cohérence globale |
| **INV-BG-10** | Neutralité conceptuelle |

---

### Q27 : Que se passe-t-il si un invariant est violé ?

**Réponse :** Une violation d'invariant constitue une **faute architecturale** qui doit être corrigée immédiatement. Un système qui viole un invariant est en état d'incohérence fondamentale.

---

## Questions sur l'implémentation

### Q28 : Border Guard gère-t-il l'authentification ?

**Réponse :** **Non, jamais.** Border Guard ne gère pas l'authentification technique (tokens, sessions, OAuth, JWT). L'authentification est du ressort du produit ou d'un module auth dédié. Border Guard utilise le résultat de l'authentification pour classifier, mais n'authentifie jamais lui-même.

---

### Q29 : Border Guard peut-il référencer des technologies spécifiques ?

**Réponse :** **Non.** Border Guard ne fait jamais de supposition sur la technologie d'implémentation (INV-BG-10). Les définitions sont purement conceptuelles.

**Exemples :**
- ✅ "Authentification requise" (neutre)
- ❌ "Token JWT RS256 requis" (couplé)

---

### Q30 : Comment Border Guard s'adapte-t-il aux niveaux de sécurité ?

**Réponse :** Les frontières et les règles s'adaptent selon le niveau de sécurité déclaré (0-4) :

| Niveau | Perméabilité par défaut | Critères |
|--------|-------------------------|----------|
| **0 - PUBLIC** | Ouverte | Assouplis |
| **1 - STANDARD** | Contrôlée | Standard |
| **2 - SENSITIVE** | Contrôlée renforcée | Renforcés |
| **3 - CRITICAL** | Strictement contrôlée | Stricts |
| **4 - HARDENED** | Fermée par défaut | Ultra-stricts |

---

## Questions pratiques

### Q31 : Comment savoir si une fonctionnalité appartient à Border Guard ?

**Réponse :** Posez-vous ces questions :
1. Est-ce une **définition** de frontière ou de règle ? → Border Guard
2. Est-ce une **classification** de niveau de confiance ? → Border Guard
3. Est-ce une **application/exécution** de règle ? → BondingBrother
4. Est-ce une **décision** d'accepter ou refuser ? → StrongFather
5. Est-ce une **persistance** de données ? → KindMother

---

### Q32 : Comment vérifier qu'une implémentation respecte Border Guard ?

**Réponse :** Utilisez la check-list mentale des 10 invariants avant toute implémentation :

1. INV-BG-1 préservé ? → Aucune exécution dans Border Guard ?
2. INV-BG-2 préservé ? → Aucune persistance directe ?
3. INV-BG-3 préservé ? → Aucune décision autonome ?
4. INV-BG-4 préservé ? → Toute source classifiée ?
5. INV-BG-5 préservé ? → Toute frontière explicite ?
6. INV-BG-6 préservé ? → Règles déclaratives uniquement ?
7. INV-BG-7 préservé ? → Définition séparée de l'application ?
8. INV-BG-8 préservé ? → Traçabilité complète ?
9. INV-BG-9 préservé ? → Cohérence globale maintenue ?
10. INV-BG-10 préservé ? → Aucune supposition technique ?

---

### Q33 : Où trouver plus d'informations ?

**Réponse :** Consultez les documents suivants :

| Besoin | Document |
|--------|----------|
| Vision complète | [Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) |
| Définition des frontières | [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) |
| Niveaux de confiance | [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) |
| Règles de franchissement | [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) |
| Invariants et garanties | [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) |
| Violations et anti-patterns | [Violations & Anti-Patterns](../contracts/governance/Border%20Guard%20-%20Violations%20&%20Anti-Patterns.md) |
| Guide d'implémentation | [Reference Implementation Guidelines](../implementation/Border%20Guard%20-%20Reference%20Implementation%20Guidelines.md) |
| Vocabulaire | [Vocabulary & Glossary](./Border%20Guard%20-%20Vocabulary%20&%20Glossary.md) |
| Exemples | [Examples & Use Cases](./Border%20Guard%20-%20Examples%20&%20Use%20Cases.md) |

---

## Documents de référence

| Document | Relation |
|----------|----------|
| [Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Index de Navigation](..\\_index.md) | Vue d'ensemble |
| [Vocabulary & Glossary](./Border%20Guard%20-%20Vocabulary%20&%20Glossary.md) | Définitions des termes |
| [Examples & Use Cases](./Border%20Guard%20-%20Examples%20&%20Use%20Cases.md) | Exemples concrets |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Document de référence — FAQ  
**Référence :** Border Guard v1.5, Tous les contrats FONDATION
