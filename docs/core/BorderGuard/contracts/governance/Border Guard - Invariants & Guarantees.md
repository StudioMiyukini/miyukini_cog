# Border Guard - Invariants & Guarantees

## 1. Contexte

Ce document définit les **invariants non négociables** et les **garanties** offertes par Border Guard dans l'écosystème Miyukini. Il formalise les règles absolues qui ne peuvent jamais être contournées, négociées, ou modifiées, ainsi que les engagements que Border Guard prend envers les autres cores du système.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non négociable**. Il dérive directement de la Documentation Fondatrice (Section 7 - Invariants non négociables et Section 11 - Garanties).

---

## 2. Portée / Scope

- **Applicable à :** Toute implémentation, configuration, ou utilisation de Border Guard
- **Responsable :** Border Guard (autorité de définition des frontières)
- **Consommateurs :** Tous les cores (StrongFather, BondingBrother, CaringNanny, KindMother), tous les modules, tous les produits
- **Ne couvre pas :** Les invariants des autres cores (voir leurs documents fondateurs respectifs)

---

## 3. Nature des invariants

### 3.1 Qu'est-ce qu'un invariant ?

Un **invariant** est une règle absolue qui :

- **Ne peut jamais être violée** — Aucune exception, aucune dérogation, aucun contournement
- **Est vérifiable** — On peut toujours déterminer si l'invariant est respecté ou non
- **Est indépendante du contexte** — L'invariant s'applique quelle que soit la situation
- **Est non négociable** — Aucune considération pratique ne peut justifier sa violation

**Conséquence d'une violation :** Toute violation d'un invariant constitue une **faute architecturale** qui doit être corrigée immédiatement. Un système qui viole un invariant est en état d'incohérence fondamentale.

### 3.2 Hiérarchie des invariants

Les invariants de Border Guard sont organisés en trois catégories :

| Catégorie | Description | Invariants |
|-----------|-------------|------------|
| **Identité** | Définissent ce que Border Guard EST et N'EST PAS | INV-BG-1, INV-BG-3 |
| **Comportement** | Définissent comment Border Guard DOIT agir | INV-BG-2, INV-BG-4, INV-BG-5, INV-BG-6 |
| **Qualité** | Définissent les propriétés que Border Guard DOIT maintenir | INV-BG-7, INV-BG-8, INV-BG-9, INV-BG-10 |

---

## 4. Invariants d'identité

### 4.1 INV-BG-1 : Aucune capacité d'exécution

**Énoncé canonique :**

> Border Guard ne possède **jamais** de capacité d'exécution. Il ne filtre pas, ne bloque pas, n'intercepte pas, n'applique pas. Toute capacité d'exécution viole cet invariant fondamental.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Identité |
| **Portée** | Absolue |
| **Vérification** | Aucune action d'exécution ne doit exister dans Border Guard |
| **Conséquence de violation** | Confusion des responsabilités, violation de la séparation des concerns |

**Ce que cela signifie concrètement :**

| Autorisé | Interdit |
|----------|----------|
| ✅ Définir une règle de filtrage | ❌ Exécuter le filtrage |
| ✅ Définir une condition de blocage | ❌ Bloquer un accès |
| ✅ Définir un protocole d'interception | ❌ Intercepter une communication |
| ✅ Décrire une action à appliquer | ❌ Appliquer l'action |

**Invariant lié :** INV-BG-7 (Séparation définition/application)

### 4.2 INV-BG-3 : Aucune décision autonome

**Énoncé canonique :**

> Border Guard ne prend **jamais** de décision de manière autonome. Il informe, il classifie, il définit, mais la décision finale appartient toujours à StrongFather ou aux autorités appropriées.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Identité |
| **Portée** | Absolue |
| **Vérification** | Aucune décision stratégique ou politique ne doit être prise par Border Guard |
| **Conséquence de violation** | Usurpation du rôle de StrongFather, incohérence décisionnelle |

**Ce que cela signifie concrètement :**

| Autorisé | Interdit |
|----------|----------|
| ✅ Classifier une source comme "hostile" | ❌ Décider de bloquer cette source |
| ✅ Informer que les conditions ne sont pas remplies | ❌ Décider de refuser l'accès |
| ✅ Fournir le contexte de confiance | ❌ Prendre une décision basée sur ce contexte |
| ✅ Évaluer un niveau de risque | ❌ Décider d'accepter ou refuser le risque |

**Relation avec StrongFather :** Border Guard conseille, StrongFather décide. Cette séparation est absolue.

---

## 5. Invariants de comportement

### 5.1 INV-BG-2 : Aucune persistance directe

**Énoncé canonique :**

> Border Guard n'accède **jamais** directement à la persistance. Toute définition de frontière ou de règle qui doit être persistée est transmise à KindMother via les canaux appropriés.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Comportement |
| **Portée** | Absolue |
| **Vérification** | Aucun accès direct à une base de données ou au système de fichiers |
| **Conséquence de violation** | Confusion avec KindMother, violation de la souveraineté des données |

**Ce que cela signifie concrètement :**

| Autorisé | Interdit |
|----------|----------|
| ✅ Demander à KindMother de persister une définition | ❌ Écrire directement dans une base de données |
| ✅ Recevoir des définitions chargées par KindMother | ❌ Lire directement depuis le stockage |
| ✅ Maintenir des définitions en mémoire | ❌ Implémenter un cache persisté |
| ✅ Transmettre un historique à persister | ❌ Gérer directement l'historique en base |

**Relation avec KindMother :** Border Guard définit, KindMother persiste. La persistance est du ressort exclusif de KindMother.

### 5.2 INV-BG-4 : Classification exhaustive

**Énoncé canonique :**

> Toute source, destination, ou interaction **doit** être classifiée selon un niveau de confiance. Aucune interaction ne peut exister sans classification. Par défaut, tout ce qui n'est pas explicitement classifié est considéré comme "unknown".

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Comportement |
| **Portée** | Absolue |
| **Vérification** | Chaque entité traversant une frontière possède un niveau de confiance |
| **Conséquence de violation** | Interactions non classifiées, faille de sécurité potentielle |

**Niveaux de confiance canoniques :**

| Niveau | Signification | Règle par défaut |
|--------|---------------|------------------|
| **Trusted** | Confiance totale | Franchissement libre |
| **Verified** | Confiance vérifiée | Franchissement conditionnel |
| **Unknown** | Confiance inconnue | Franchissement restreint |
| **Hostile** | Confiance nulle | Franchissement interdit |

**Règle de défaut :** Si une source n'est pas explicitement classifiée, elle est considérée comme **unknown**. Ce défaut est sécuritaire par conception.

**Référence :** [Border Guard - Trust Level Classification Contract](../boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md)

### 5.3 INV-BG-5 : Frontières explicites

**Énoncé canonique :**

> Toute frontière **doit** être explicitement définie et documentée. Aucune frontière implicite n'est autorisée. Si une démarcation existe dans le système, elle doit être formalisée par Border Guard.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Comportement |
| **Portée** | Absolue |
| **Vérification** | Chaque frontière possède une définition formelle avec toutes les propriétés requises |
| **Conséquence de violation** | Frontières fantômes, incohérence de sécurité, zones non protégées |

**Propriétés obligatoires d'une frontière :**

- Identifiant unique et stable
- Nom descriptif
- Description et justification
- Type (externe, interne, intégration)
- Direction (entrée, sortie, bidirectionnelle)
- Perméabilité (ouverte, contrôlée, fermée)
- Règles de franchissement associées
- Traçabilité (origine, date, historique)

**Référence :** [Border Guard - Boundary Definition Contract](../boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md)

### 5.4 INV-BG-6 : Règles déclaratives

**Énoncé canonique :**

> Toutes les règles de franchissement **doivent** être déclaratives. Aucune règle procédurale ou impérative n'est autorisée. Une règle exprime ce qui est requis, pas comment le vérifier.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Comportement |
| **Portée** | Absolue |
| **Vérification** | Les règles expriment des conditions, pas des procédures |
| **Conséquence de violation** | Couplage avec l'implémentation, violation de INV-BG-10 |

**Exemples de règles déclaratives vs procédurales :**

| ✅ Déclaratif (autorisé) | ❌ Procédural (interdit) |
|--------------------------|--------------------------|
| "Le niveau de confiance requis est verified" | "Vérifier le token JWT et valider la signature" |
| "L'authentification est requise" | "Appeler le service auth et vérifier la session" |
| "Les données sensibles ne peuvent pas traverser" | "Filtrer les champs marqués sensitive: true" |
| "L'origine doit être dans la liste blanche" | "Itérer sur la whitelist et comparer les IPs" |

**Invariant lié :** INV-BG-10 (Neutralité conceptuelle)

**Référence :** [Border Guard - Crossing Rules Contract](../boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md)

---

## 6. Invariants de qualité

### 6.1 INV-BG-7 : Séparation définition/application

**Énoncé canonique :**

> La définition des frontières et des règles est **strictement séparée** de leur application. Border Guard définit, BondingBrother applique. Cette séparation est non négociable et ne peut être contournée.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Qualité |
| **Portée** | Absolue |
| **Vérification** | Aucune logique d'application dans Border Guard, aucune définition dans BondingBrother |
| **Conséquence de violation** | Couplage fort, impossibilité de modifier indépendamment |

**Schéma de séparation :**

```
┌─────────────────────────────────────────────────────────────┐
│                       DÉFINITION                             │
│                     (Border Guard)                           │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  Frontières  │  │   Niveaux    │  │    Règles    │       │
│  │   définies   │  │  de confiance│  │ de franchiss.│       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│         │                │                 │                 │
└─────────│────────────────│─────────────────│─────────────────┘
          │                │                 │
          ▼                ▼                 ▼
     ─────────────────────────────────────────────────
                    CONTRAT D'INTERFACE
     ─────────────────────────────────────────────────
          │                │                 │
          ▼                ▼                 ▼
┌─────────────────────────────────────────────────────────────┐
│                       APPLICATION                            │
│                    (Bonding Brother)                         │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  Filtrage    │  │ Vérification │  │   Contrôle   │       │
│  │   concret    │  │    réelle    │  │    d'accès   │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Invariants liés :** INV-BG-1 (Aucune capacité d'exécution)

**Référence :** [Border Guard - BondingBrother Integration Contract](../integration/Border%20Guard%20-%20BondingBrother%20Integration%20Contract.md)

### 6.2 INV-BG-8 : Traçabilité complète

**Énoncé canonique :**

> Toute définition de frontière, toute classification de confiance, toute règle établie **doit** être traçable avec son origine, sa date, et sa justification.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Qualité |
| **Portée** | Absolue |
| **Vérification** | Chaque élément possède les métadonnées de traçabilité complètes |
| **Conséquence de violation** | Impossibilité d'audit, responsabilité non attribuable |

**Métadonnées de traçabilité obligatoires :**

| Métadonnée | Description | Obligatoire |
|------------|-------------|-------------|
| **Origine** | Qui a créé/modifié l'élément | ✅ Oui |
| **Date de création** | Horodatage de création | ✅ Oui |
| **Date de modification** | Horodatage de dernière modification | ✅ Oui |
| **Justification** | Pourquoi cet élément existe | ✅ Oui |
| **Historique** | Journal des modifications | ✅ Oui |
| **Version** | Numéro de version | ✅ Oui |

**Format de traçabilité :**

```
Traçabilité:
  origine: "BorderGuard/SecurityPolicy"
  créé_le: "2026-01-28T10:00:00Z"
  créé_par: "system/initialization"
  modifié_le: "2026-01-28T11:30:00Z"
  modifié_par: "admin/security-upgrade"
  justification: "Protection des données sensibles niveau 3"
  version: "1.2"
  historique:
    - date: "2026-01-28T11:30:00Z"
      action: "modification"
      détail: "Renforcement de la perméabilité"
```

### 6.3 INV-BG-9 : Cohérence globale

**Énoncé canonique :**

> Les définitions de Border Guard **doivent** être globalement cohérentes. Aucune contradiction entre frontières, niveaux de confiance, ou règles n'est autorisée.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Qualité |
| **Portée** | Absolue |
| **Vérification** | Aucune contradiction logique détectable entre définitions |
| **Conséquence de violation** | État système incohérent, comportement imprévisible |

**Types de cohérence à maintenir :**

| Type | Description | Exemple de violation |
|------|-------------|----------------------|
| **Cohérence frontière-zone** | Une frontière sépare exactement deux zones | Frontière sans zone de destination |
| **Cohérence niveau-règle** | Les règles sont compatibles avec les niveaux | Règle permettant à "hostile" de traverser une frontière fermée |
| **Cohérence règle-règle** | Les règles ne se contredisent pas | Deux règles contradictoires sur la même frontière |
| **Cohérence temporelle** | L'ordre chronologique est respecté | Date de création postérieure à la date de modification |

**Mécanisme de vérification :**

La cohérence globale est vérifiée :
- À chaque création de définition
- À chaque modification de définition
- Périodiquement lors des audits système

### 6.4 INV-BG-10 : Neutralité conceptuelle

**Énoncé canonique :**

> Border Guard **ne fait jamais** de supposition sur la technologie d'implémentation. Les définitions sont purement conceptuelles et peuvent être implémentées par n'importe quelle technologie.

| Aspect | Spécification |
|--------|---------------|
| **Catégorie** | Qualité |
| **Portée** | Absolue |
| **Vérification** | Aucune référence technologique spécifique dans les définitions |
| **Conséquence de violation** | Couplage technologique, impossibilité de portage |

**Ce que cela signifie concrètement :**

| ✅ Neutre (autorisé) | ❌ Couplé (interdit) |
|----------------------|----------------------|
| "Authentification requise" | "Token JWT requis" |
| "Source dans la liste autorisée" | "IP dans la whitelist nginx" |
| "Données chiffrées" | "Données chiffrées en AES-256" |
| "Connexion sécurisée" | "Connexion HTTPS/TLS 1.3" |
| "Session valide" | "Cookie de session Supabase valide" |

**Invariant lié :** INV-BG-6 (Règles déclaratives)

---

## 7. Garanties offertes

### 7.1 Nature des garanties

Une **garantie** est un engagement que Border Guard prend envers les autres cores et le système global. Contrairement aux invariants (règles absolues), les garanties sont des promesses de service.

### 7.2 Garantie d'exhaustivité

**Énoncé :**

> Border Guard garantit que **toute frontière du système est explicitement définie**.

| Aspect | Spécification |
|--------|---------------|
| **Ce que cela implique** | Pas de frontière implicite ou cachée |
| **Comment c'est vérifié** | Registre exhaustif des frontières, audit périodique |
| **Qui en bénéficie** | Tous les cores, tous les produits |
| **Invariant associé** | INV-BG-5 |

### 7.3 Garantie de classification complète

**Énoncé :**

> Border Guard garantit que **toute source et interaction est classifiée** selon un niveau de confiance.

| Aspect | Spécification |
|--------|---------------|
| **Ce que cela implique** | Pas d'entité non classifiée (défaut = unknown) |
| **Comment c'est vérifié** | Classification automatique par défaut |
| **Qui en bénéficie** | StrongFather (contexte de décision), BondingBrother (application) |
| **Invariant associé** | INV-BG-4 |

### 7.4 Garantie de cohérence

**Énoncé :**

> Border Guard garantit que **les définitions sont globalement cohérentes et non contradictoires**.

| Aspect | Spécification |
|--------|---------------|
| **Ce que cela implique** | Pas de contradiction entre définitions |
| **Comment c'est vérifié** | Vérification à chaque modification, audit périodique |
| **Qui en bénéficie** | Tout le système (prévisibilité comportementale) |
| **Invariant associé** | INV-BG-9 |

### 7.5 Garantie de traçabilité

**Énoncé :**

> Border Guard garantit que **toute définition est traçable avec son origine et sa justification**.

| Aspect | Spécification |
|--------|---------------|
| **Ce que cela implique** | Audit complet possible à tout moment |
| **Comment c'est vérifié** | Métadonnées obligatoires sur chaque définition |
| **Qui en bénéficie** | Auditeurs, responsables sécurité, opérateurs |
| **Invariant associé** | INV-BG-8 |

### 7.6 Garantie de neutralité technique

**Énoncé :**

> Border Guard garantit que **les définitions sont indépendantes de l'implémentation**.

| Aspect | Spécification |
|--------|---------------|
| **Ce que cela implique** | Portabilité des définitions vers toute technologie |
| **Comment c'est vérifié** | Revue des définitions, absence de références techniques |
| **Qui en bénéficie** | Équipes de développement, évolution technologique |
| **Invariant associé** | INV-BG-10 |

### 7.7 Garantie de séparation stricte

**Énoncé :**

> Border Guard garantit que **la définition est strictement séparée de l'application**.

| Aspect | Spécification |
|--------|---------------|
| **Ce que cela implique** | Modification indépendante des définitions et de l'application |
| **Comment c'est vérifié** | Architecture en couches, contrats d'interface |
| **Qui en bénéficie** | BondingBrother (liberté d'implémentation), évolution du système |
| **Invariant associé** | INV-BG-7 |

---

## 8. Matrice des invariants

### 8.1 Vue synthétique

| Invariant | Catégorie | Énoncé court | Relation principale |
|-----------|-----------|--------------|---------------------|
| **INV-BG-1** | Identité | Aucune capacité d'exécution | BondingBrother exécute |
| **INV-BG-2** | Comportement | Aucune persistance directe | KindMother persiste |
| **INV-BG-3** | Identité | Aucune décision autonome | StrongFather décide |
| **INV-BG-4** | Comportement | Classification exhaustive | Défaut = unknown |
| **INV-BG-5** | Comportement | Frontières explicites | Pas de frontière implicite |
| **INV-BG-6** | Comportement | Règles déclaratives | Pas de procédure |
| **INV-BG-7** | Qualité | Séparation définition/application | Contrat avec BondingBrother |
| **INV-BG-8** | Qualité | Traçabilité complète | Audit possible |
| **INV-BG-9** | Qualité | Cohérence globale | Pas de contradiction |
| **INV-BG-10** | Qualité | Neutralité conceptuelle | Pas de couplage tech |

### 8.2 Interdépendances

```
INV-BG-1 ──────────────────────────────┐
(Pas d'exécution)                      │
         │                             ▼
         └────────────────────► INV-BG-7
                                (Séparation déf/app)
INV-BG-3 ◄────────────────────────────┘
(Pas de décision)

INV-BG-6 ──────────────────────────────┐
(Règles déclaratives)                  │
         │                             ▼
         └────────────────────► INV-BG-10
                                (Neutralité tech)

INV-BG-5 ──────────────────────────────┐
(Frontières explicites)                │
         │                             ▼
         └────────────────────► INV-BG-8
                                (Traçabilité)
                                       │
                                       ▼
                                INV-BG-9
                                (Cohérence)
```

---

## 9. Références croisées

### Documents associés

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source (Section 7, 11) |
| [Border Guard - Violations & Anti-Patterns](./Border%20Guard%20-%20Violations%20&%20Anti-Patterns.md) | Violations de ces invariants |
| [Border Guard - Boundary Definition Contract](../boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) | Application de INV-BG-5, INV-BG-8 |
| [Border Guard - Trust Level Classification Contract](../boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Application de INV-BG-4 |
| [Border Guard - Crossing Rules Contract](../boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | Application de INV-BG-6 |
| [Border Guard - BondingBrother Integration Contract](../integration/Border%20Guard%20-%20BondingBrother%20Integration%20Contract.md) | Application de INV-BG-7 |

### Références glossaire

| Terme | Définition |
|-------|------------|
| **Invariant** | Règle absolue qui ne peut jamais être violée |
| **Garantie** | Engagement de service que Border Guard prend envers le système |
| **Traçabilité** | Capacité à retracer l'origine et l'historique d'une définition |
| **Cohérence** | Absence de contradiction entre les définitions |
| **Neutralité conceptuelle** | Indépendance vis-à-vis de la technologie d'implémentation |

---

## 10. Synthèse contractuelle

### Engagements de ce contrat

Ce contrat établit que :

1. **Les invariants sont absolus** — 10 invariants non négociables définissent les limites de Border Guard
2. **Les catégories sont claires** — Identité, Comportement, Qualité organisent les invariants
3. **Les garanties sont formelles** — 6 garanties de service envers le système
4. **Les interdépendances sont explicites** — Les invariants se renforcent mutuellement
5. **Les violations sont identifiables** — Chaque invariant est vérifiable

### Phrase de synthèse

> **Border Guard respecte 10 invariants non négociables (identité, comportement, qualité) et offre 6 garanties formelles (exhaustivité, classification, cohérence, traçabilité, neutralité, séparation), formant le socle contractuel de toute définition de frontière.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Référence :** Border Guard v1.5, Documentation Fondatrice Section 7, Section 11  
**Type :** Contrat de gouvernance — Invariants et Garanties
