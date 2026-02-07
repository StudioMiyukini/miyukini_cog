# Master Butler — Boundary & Scope Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Boundary & Scope Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les frontières absolues de Master Butler, son périmètre d'action, ses responsabilités exclusives, et les limites qu'il ne franchit jamais dans le système Miyukini Core System v2.4.

Ce contrat complète la [Documentation Fondatrice de Master Butler](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) en définissant de manière formelle et contractuelle les frontières du Capability & Permission Core.

### Portée

Ce contrat s'applique à **Master Butler lui-même** et définit de manière absolue :
- Le périmètre exact des responsabilités de Master Butler
- Les frontières qu'il ne franchit jamais
- Ce qui relève exclusivement de Master Butler
- Ce qui ne relève jamais de Master Butler
- Les interactions autorisées avec les autres Cores
- Les invariants de frontière non négociables

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues que Master Butler applique sans exception. Ces règles ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat s'articule avec les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : Définit la raison d'être et les concepts fondamentaux
- **[Master Butler - Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : Définit le modèle du registre des capacités
- **[Master Butler - Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : Définit le modèle du registre des permissions
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) et **LOI-5** (coût proportionnel au hardware)

---

## 2. Définition formelle du périmètre (Scope)

### Énoncé du périmètre

Le périmètre de Master Butler est défini par la question fondamentale à laquelle il répond :

> **"Quelles sont les capacités du système, et quelles permissions existent pour y accéder ?"**

Cette question délimite exactement le périmètre de Master Butler :
- **Dans le périmètre** : Tout ce qui concerne la connaissance des possibilités du système
- **Hors périmètre** : Tout ce qui concerne la décision, l'exécution, ou la gestion des données

### Périmètre IN (ce qui relève de Master Butler)

| Domaine | Responsabilité | Caractère |
|---------|----------------|-----------|
| **Registre des capacités** | Recensement exhaustif de toutes les capacités du système | **EXCLUSIF** |
| **Registre des permissions** | Définition formelle de toutes les permissions | **EXCLUSIF** |
| **Associations** | Liens entre permissions et capacités | **EXCLUSIF** |
| **Déclarations** | Réception et validation des déclarations de capacités | **EXCLUSIF** |
| **Définitions** | Création et gestion des définitions de permissions | **EXCLUSIF** |
| **Découverte** | API de découverte des capacités et permissions | **EXCLUSIF** |
| **Contexte de capacité** | Calcul du contexte de capacité pour un demandeur | **EXCLUSIF** |
| **Métadonnées** | Gestion des métadonnées des capacités et permissions | **EXCLUSIF** |
| **Traçabilité des définitions** | Journalisation des créations, modifications, suppressions | **EXCLUSIF** |
| **Gouvernance des Tools** | Catalogue des Tools et Toolkits disponibles | **EXCLUSIF** |
| **Mapping Capability → Tool** | Association entre capacités et Tools | **EXCLUSIF** |

### Périmètre OUT (ce qui ne relève JAMAIS de Master Butler)

| Domaine | Responsabilité | Raison |
|---------|----------------|--------|
| **Décision** | Autoriser ou refuser une action | Relève de **StrongFather** |
| **Évaluation des intentions** | Évaluer si une intention est autorisée | Relève de **StrongFather** |
| **Politiques** | Définir les règles de décision | Relève de **StrongFather** |
| **Exécution** | Exécuter une action fonctionnelle | Relève des **Tools/Opérateurs** |
| **Implémentation des Tools** | Implémenter les Tools | Relève des **Tools eux-mêmes** |
| **Persistance des données métier** | Stocker des données métier | Relève de **KindMother** |
| **Gestion des identités** | Authentifier les utilisateurs | Relève du **système d'identité** |
| **Cycle de vie des Tools** | Versionner, déprécier les Tools | Relève de **Ever Buddy** |
| **État système** | Surveiller l'état du système | Relève de **Caring Nanny** |
| **Sécurité runtime** | Bloquer les menaces en temps réel | Relève de **WorrySentinel** |
| **Médiation** | Traduire les intentions | Relève de **BondingBrother** |

### Règle de délimitation absolue

> **Master Butler connaît ce qui est possible, mais ne décide jamais de ce qui est autorisé.**

Cette règle est **non négociable**. Toute extension de périmètre qui violerait cette règle est interdite.

---

## 3. Frontières absolues de Master Butler

### Frontière F1 : Frontière de décision

**Définition :** Master Butler fournit des informations, mais ne produit jamais de décision d'autorisation.

**Manifestation contractuelle :**
- Master Butler répond "cette capacité existe" — **AUTORISÉ**
- Master Butler répond "cette permission est définie" — **AUTORISÉ**
- Master Butler répond "ce rôle possède cette permission" — **AUTORISÉ**
- Master Butler répond "cette action est autorisée" — **INTERDIT**
- Master Butler répond "cette action est refusée" — **INTERDIT**

**Justification :** La décision appartient exclusivement à StrongFather. Master Butler fournit les informations nécessaires à la décision, mais ne prend jamais part à cette décision.

**Non-négociabilité :** Absolue. Aucune méthode de Master Butler ne retourne un booléen d'autorisation. Il retourne des informations, pas des décisions.

### Frontière F2 : Frontière d'exécution

**Définition :** Master Butler recense les capacités, mais n'exécute jamais d'action fonctionnelle.

**Manifestation contractuelle :**
- Master Butler catalogue la capacité `content.create` — **AUTORISÉ**
- Master Butler exécute la création de contenu — **INTERDIT**
- Master Butler déclare un Tool `file.write` — **AUTORISÉ**
- Master Butler exécute l'écriture de fichier — **INTERDIT**

**Justification :** L'exécution appartient aux Tools et aux Opérateurs. Master Butler est un catalogue, pas un exécuteur.

**Non-négociabilité :** Absolue. Aucune méthode de Master Butler n'exécute d'action fonctionnelle.

### Frontière F3 : Frontière de données métier

**Définition :** Master Butler stocke des métadonnées sur les capacités et permissions, mais ne stocke jamais de données métier.

**Manifestation contractuelle :**
- Master Butler stocke "la capacité content.create existe" — **AUTORISÉ**
- Master Butler stocke "la permission content.edit.own est définie" — **AUTORISÉ**
- Master Butler stocke le contenu d'un article — **INTERDIT**
- Master Butler stocke les données d'un utilisateur — **INTERDIT**

**Justification :** Les données métier appartiennent aux modules et sont gérées via KindMother. Master Butler gère uniquement les métadonnées des capacités et permissions.

**Non-négociabilité :** Absolue. Le registre de Master Butler ne contient que des métadonnées, jamais de données métier.

### Frontière F4 : Frontière de vérification runtime

**Définition :** Master Butler définit les permissions, mais ne vérifie jamais leur validité en temps réel lors d'une action.

**Manifestation contractuelle :**
- Master Butler définit "la permission content.edit.own existe" — **AUTORISÉ**
- Master Butler retourne "les permissions requises pour cette capacité sont..." — **AUTORISÉ**
- Master Butler vérifie "cet utilisateur a-t-il cette permission maintenant ?" — **INTERDIT**
- Master Butler vérifie "cette permission est-elle valide dans ce contexte ?" — **INTERDIT**

**Justification :** La vérification runtime des permissions appartient à StrongFather lors de l'évaluation des intentions. Master Butler fournit les définitions, pas les vérifications.

**Non-négociabilité :** Absolue. Master Butler ne vérifie jamais les permissions en temps réel.

### Frontière F5 : Frontière de politique

**Définition :** Master Butler définit ce qui existe comme permissions, mais ne définit jamais les règles d'utilisation de ces permissions.

**Manifestation contractuelle :**
- Master Butler définit "la permission admin.all existe" — **AUTORISÉ**
- Master Butler associe "admin.all → toutes les capacités" — **AUTORISÉ**
- Master Butler définit "admin.all ne peut être utilisé que par le super-admin" — **INTERDIT**
- Master Butler définit "admin.all expire après 24h" — **INTERDIT**

**Justification :** Les règles d'utilisation des permissions (politiques) appartiennent à StrongFather. Master Butler définit l'existence des permissions, pas leurs conditions d'utilisation.

**Non-négociabilité :** Absolue. Master Butler ne contient aucune politique de décision.

### Frontière F6 : Frontière d'identité

**Définition :** Master Butler connaît les rôles et leurs permissions associées, mais ne gère jamais les identités des utilisateurs ou des systèmes.

**Manifestation contractuelle :**
- Master Butler définit "le rôle editor possède les permissions X, Y, Z" — **AUTORISÉ**
- Master Butler retourne "les permissions associées au rôle editor" — **AUTORISÉ**
- Master Butler authentifie un utilisateur — **INTERDIT**
- Master Butler attribue un rôle à un utilisateur — **INTERDIT**
- Master Butler vérifie l'identité d'un utilisateur — **INTERDIT**

**Justification :** La gestion des identités appartient au système d'authentification. Master Butler connaît les associations rôles-permissions, mais ignore les attributions utilisateurs-rôles.

**Non-négociabilité :** Absolue. Master Butler ne gère jamais les identités.

### Frontière F7 : Frontière de contraintes métier

**Définition :** Master Butler définit les capacités techniques, mais n'applique jamais de contraintes métier.

**Manifestation contractuelle :**
- Master Butler définit "la capacité content.create existe" — **AUTORISÉ**
- Master Butler retourne "la capacité content.create est disponible" — **AUTORISÉ**
- Master Butler limite "un utilisateur ne peut créer que 10 contenus par jour" — **INTERDIT**
- Master Butler valide "le contenu respecte les règles métier" — **INTERDIT**

**Justification :** Les contraintes métier appartiennent aux modules et à StrongFather. Master Butler sait ce qui est techniquement possible, pas ce qui est métier-compatible.

**Non-négociabilité :** Absolue. Master Butler ne contient aucune logique métier.

---

## 4. Interactions autorisées avec les autres Cores

### Interaction avec StrongFather

**Type d'interaction :** Fournisseur d'informations → Décideur

**Flux autorisé :**

```
StrongFather : "Cette capacité existe-t-elle ?"
Master Butler : "Oui, voici ses métadonnées"

StrongFather : "Quelles permissions sont requises pour cette capacité ?"
Master Butler : "Voici les permissions associées"

StrongFather : "Quelles capacités sont couvertes par cette permission ?"
Master Butler : "Voici la liste des capacités"
```

**Flux INTERDIT :**

```
StrongFather : "Dois-je autoriser cette action ?"
Master Butler : "[VIOLATION] Master Butler ne décide pas"

StrongFather : "Exécute cette action"
Master Butler : "[VIOLATION] Master Butler n'exécute pas"
```

**Caractère :** StrongFather dépend de Master Butler pour connaître les possibilités, mais Master Butler ne dépend pas de StrongFather.

### Interaction avec KindMother

**Type d'interaction :** Consommateur de persistance

**Flux autorisé :**

```
Master Butler : "Persiste cette modification du registre"
KindMother : "Modification persistée"

Master Butler : "Récupère le registre"
KindMother : "Voici les données"
```

**Flux INTERDIT :**

```
Master Butler : "Persiste ces données métier"
KindMother : "[VIOLATION] Master Butler ne stocke pas de données métier"

Master Butler : "Accède directement à SQLite"
KindMother : "[VIOLATION] Abstraction totale requise"
```

**Caractère :** Master Butler peut utiliser KindMother pour persister son registre, mais ne manipule jamais directement la persistance.

### Interaction avec BondingBrother

**Type d'interaction :** Fournisseur d'informations pour traduction

**Flux autorisé :**

```
BondingBrother : "Cette capacité existe-t-elle dans ce module ?"
Master Butler : "Oui/Non, voici les détails"

BondingBrother : "Quelles permissions sont requises pour cette action ?"
Master Butler : "Voici les permissions nécessaires"

BondingBrother : "Quelles capacités sont disponibles pour ce contexte ?"
Master Butler : "Voici le contexte de capacité"
```

**Caractère :** BondingBrother utilise Master Butler pour traduire correctement les intentions et préparer le contexte pour StrongFather.

### Interaction avec les Opérateurs (Produits)

**Type d'interaction :** Récepteur de déclarations et fournisseur de découverte

**Flux autorisé :**

```
Opérateur : "Je déclare mes capacités"
Master Butler : "Déclaration enregistrée"

Opérateur : "Quelles capacités existent dans ce module ?"
Master Butler : "Voici la liste des capacités"

Opérateur : "Je définis une nouvelle permission"
Master Butler : "Permission définie et enregistrée"
```

**Flux INTERDIT :**

```
Opérateur : "Autorise-moi à faire cette action"
Master Butler : "[VIOLATION] Master Butler ne décide pas"

Opérateur : "Exécute cette capacité pour moi"
Master Butler : "[VIOLATION] Master Butler n'exécute pas"
```

**Caractère :** Les Opérateurs alimentent Master Butler (déclarations) et consomment Master Butler (découverte).

---

## 5. Invariants de frontière non négociables

### INV-MB-B1 : Non-décision absolue

**Énoncé :** Master Butler ne prend **JAMAIS** de décision d'autorisation, quel que soit le contexte.

**Application :**
- Aucune méthode ne retourne un booléen d'autorisation
- Aucune méthode ne valide une permission en temps réel
- Aucune méthode ne produit un verdict "autorisé" ou "refusé"
- Toutes les réponses sont des informations, pas des décisions

**Violation :** Toute implémentation qui retourne une décision d'autorisation viole cet invariant.

**Non-négociabilité :** Absolue. Aucune exception possible.

### INV-MB-B2 : Non-exécution absolue

**Énoncé :** Master Butler n'exécute **JAMAIS** d'action fonctionnelle, quel que soit le contexte.

**Application :**
- Aucune méthode n'exécute d'opération métier
- Aucune méthode ne modifie de données métier
- Aucune méthode ne déclenche d'effet de bord fonctionnel
- Master Butler est un registre passif, pas un exécuteur actif

**Violation :** Toute implémentation qui exécute une action fonctionnelle viole cet invariant.

**Non-négociabilité :** Absolue. Aucune exception possible.

### INV-MB-B3 : Registre de métadonnées uniquement

**Énoncé :** Le registre de Master Butler ne contient **QUE** des métadonnées sur les capacités et permissions, jamais de données métier.

**Application :**
- Seuls les identifiants, noms, descriptions, associations sont stockés
- Aucune donnée métier n'est jamais stockée
- Aucune référence directe à des données métier n'est stockée
- Le registre est léger et ne contient que des définitions

**Violation :** Toute implémentation qui stocke des données métier dans le registre viole cet invariant.

**Non-négociabilité :** Absolue. Aucune exception possible.

### INV-MB-B4 : Absence de logique métier

**Énoncé :** Master Butler ne contient **AUCUNE** logique métier, quel que soit le contexte.

**Application :**
- Aucune règle métier n'est encodée dans Master Butler
- Aucune contrainte métier n'est appliquée par Master Butler
- Aucune validation métier n'est effectuée par Master Butler
- Master Butler sait ce qui est techniquement possible, pas ce qui est métier-compatible

**Violation :** Toute implémentation qui encode ou applique une logique métier viole cet invariant.

**Non-négociabilité :** Absolue. Aucune exception possible.

### INV-MB-B5 : Absence de politique de décision

**Énoncé :** Master Butler ne contient **AUCUNE** politique de décision, quel que soit le contexte.

**Application :**
- Aucune règle de décision n'est définie dans Master Butler
- Aucune condition d'utilisation des permissions n'est définie
- Aucune règle temporelle ou contextuelle n'est encodée
- Master Butler définit l'existence, pas les conditions d'utilisation

**Violation :** Toute implémentation qui encode une politique de décision viole cet invariant.

**Non-négociabilité :** Absolue. Aucune exception possible.

### INV-MB-B6 : Indépendance vis-à-vis des identités

**Énoncé :** Master Butler ne gère **JAMAIS** les identités des utilisateurs ou des systèmes.

**Application :**
- Aucune authentification n'est effectuée par Master Butler
- Aucune attribution de rôle à un utilisateur n'est gérée
- Aucune vérification d'identité n'est effectuée
- Master Butler connaît les rôles et leurs permissions, pas les utilisateurs

**Violation :** Toute implémentation qui gère des identités viole cet invariant.

**Non-négociabilité :** Absolue. Aucune exception possible.

### INV-MB-B7 : Accessibilité universelle

**Énoncé :** Master Butler est accessible à **TOUS** les composants autorisés du système pour la consultation des capacités et permissions.

**Application :**
- Aucun composant ne peut être empêché d'interroger Master Butler
- L'API de découverte est universellement accessible
- Les informations sur les capacités sont disponibles pour tous
- Master Butler est un service partagé, pas un composant isolé

**Violation :** Toute implémentation qui restreint l'accès de manière arbitraire viole cet invariant.

**Non-négociabilité :** Absolue. Aucune exception possible.

---

## 6. Schéma ASCII des frontières

### 6.1. Périmètre de Master Butler

```
┌─────────────────────────────────────────────────────────────────┐
│              PÉRIMÈTRE DE MASTER BUTLER                          │
│              (Capability & Permission Core)                      │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              REGISTRE DES CAPACITÉS                        │ │
│  │                                                             │ │
│  │  ✓ Recensement des capacités                               │ │
│  │  ✓ Déclarations des modules/opérateurs                     │ │
│  │  ✓ Métadonnées des capacités                               │ │
│  │  ✓ Historique des modifications                            │ │
│  │  ✓ Catalogue des Tools et Toolkits                         │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              REGISTRE DES PERMISSIONS                      │ │
│  │                                                             │ │
│  │  ✓ Définition des permissions                              │ │
│  │  ✓ Associations permission → capacité                      │ │
│  │  ✓ Associations rôle → permission                          │ │
│  │  ✓ Métadonnées des permissions                             │ │
│  │  ✓ Historique des définitions                              │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              API DE DÉCOUVERTE                             │ │
│  │                                                             │ │
│  │  ✓ Découverte des capacités par module                     │ │
│  │  ✓ Découverte des permissions par capacité                 │ │
│  │  ✓ Calcul du contexte de capacité                          │ │
│  │  ✓ Interrogation par StrongFather                          │ │
│  │  ✓ Interrogation par BondingBrother                        │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              TRAÇABILITÉ                                   │ │
│  │                                                             │ │
│  │  ✓ Journalisation des déclarations                         │ │
│  │  ✓ Journalisation des définitions                          │ │
│  │  ✓ Historique des modifications                            │ │
│  │  ✓ Audit trail complet                                     │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2. Frontières avec les autres Cores

```
┌─────────────────────────────────────────────────────────────────┐
│                     HORS PÉRIMÈTRE                               │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  STRONGFATHER (Decision Core)                              │ │
│  │                                                             │ │
│  │  ✗ Décision d'autorisation                                 │ │
│  │  ✗ Évaluation des intentions                               │ │
│  │  ✗ Application des politiques                              │ │
│  │  ✗ Vérification des permissions en temps réel              │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            ▲                                      │
│                            │ Informations sur capacités           │
│                            │ et permissions                       │
│                            │                                      │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  KINDMOTHER (Data Core)                                    │ │
│  │                                                             │ │
│  │  ✗ Persistance des données métier                          │ │
│  │  ✗ Synchronisation des données                             │ │
│  │  ✗ Gestion des instances                                   │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            ▲                                      │
│                            │ Persistance du registre              │
│                            │ (via KindMother)                     │
│                            │                                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              MASTER BUTLER                                 │ │
│  │              (Capability & Permission Core)                │ │
│  │                                                             │ │
│  │  ✓ Registre des capacités                                  │ │
│  │  ✓ Registre des permissions                                │ │
│  │  ✓ API de découverte                                       │ │
│  │  ✓ Traçabilité des définitions                             │ │
│  │                                                             │ │
│  │  ✗ Décision                                                │ │
│  │  ✗ Exécution                                               │ │
│  │  ✗ Données métier                                          │ │
│  │  ✗ Identités                                               │ │
│  │  ✗ Politiques                                              │ │
│  │  ✗ Logique métier                                          │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                      │
│                            │ Informations sur capacités           │
│                            │ disponibles                          │
│                            ▼                                      │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDINGBROTHER (Mediation Core)                           │ │
│  │                                                             │ │
│  │  ✗ Traduction des intentions                               │ │
│  │  ✗ Médiation entre Opérateurs et Cores                     │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  OPÉRATEURS (Products)                                     │ │
│  │                                                             │ │
│  │  ✗ Exécution fonctionnelle                                 │ │
│  │  ✗ Logique métier                                          │ │
│  │  ✗ Interface utilisateur                                   │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            ▲                                      │
│                            │ Déclarations de capacités            │
│                            │ Définitions de permissions           │
│                            │                                      │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  TOOLS (Exécution)                                         │ │
│  │                                                             │ │
│  │  ✗ Implémentation des capacités                            │ │
│  │  ✗ Exécution des actions                                   │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 6.3. Flux d'information et frontières

```
OPÉRATEUR                    MASTER BUTLER                 STRONGFATHER
    │                              │                              │
    │ 1. Déclare capacités         │                              │
    │──────────────────────────────>│                              │
    │                              │                              │
    │ 2. Définit permissions       │                              │
    │──────────────────────────────>│                              │
    │                              │                              │
    │                              │                              │
    │ 3. Demande découverte        │                              │
    │──────────────────────────────>│                              │
    │                              │                              │
    │ 4. Retourne informations     │                              │
    │<──────────────────────────────│                              │
    │                              │                              │
    │                              │                              │
    │                              │ 5. Interroge sur capacité    │
    │                              │<──────────────────────────────│
    │                              │                              │
    │                              │ 6. Retourne informations     │
    │                              │──────────────────────────────>│
    │                              │                              │
    │                              │                              │
    │                              │ 7. StrongFather DÉCIDE       │
    │                              │                       ✓/✗    │
    │                              │                              │

═══════════════════════════════════════════════════════════════════
            FRONTIÈRE DE DÉCISION (ne franchit JAMAIS)
═══════════════════════════════════════════════════════════════════

    │                              │
    │  Master Butler ne décide     │
    │  JAMAIS si une action est    │
    │  autorisée ou refusée        │
    │                              │
```

---

## 7. Ce que Master Butler NE FAIT JAMAIS

### NF1 : Ne retourne jamais de booléen d'autorisation

**Interdit :**
```
fn is_authorized(user, action) -> bool  // INTERDIT
fn can_do(context, capability) -> bool   // INTERDIT
fn check_permission(role, permission) -> bool // INTERDIT
```

**Autorisé :**
```
fn get_capability(id) -> Option<Capability>  // Information
fn get_permissions_for(capability) -> Vec<Permission>  // Information
fn get_capabilities_for(role) -> Vec<Capability>  // Information
```

### NF2 : Ne vérifie jamais les permissions en temps réel

**Interdit :**
- Vérifier si un utilisateur possède une permission à l'instant T
- Valider si une permission est applicable dans un contexte donné
- Évaluer si les conditions d'une permission sont remplies

**Autorisé :**
- Retourner les permissions définies pour une capacité
- Retourner les permissions associées à un rôle
- Retourner les métadonnées d'une permission

### NF3 : Ne stocke jamais de données métier

**Interdit :**
- Stocker le contenu d'un article
- Stocker les préférences d'un utilisateur
- Stocker des données de transaction
- Stocker des fichiers ou médias

**Autorisé :**
- Stocker "la capacité content.create existe"
- Stocker "la permission editor.publish est définie"
- Stocker "le rôle admin possède les permissions X, Y, Z"

### NF4 : Ne définit jamais de politique de décision

**Interdit :**
- Définir "cette permission n'est valable que de 9h à 18h"
- Définir "cette permission expire après 24h"
- Définir "cette permission nécessite une authentification 2FA"
- Définir "cette permission est limitée à 10 utilisations par jour"

**Autorisé :**
- Définir "cette permission existe avec cet identifiant"
- Définir "cette permission est associée à ces capacités"
- Définir "cette permission a cette description"

### NF5 : N'exécute jamais d'action fonctionnelle

**Interdit :**
- Créer un contenu
- Modifier une hiérarchie
- Téléverser un média
- Envoyer une notification
- Appeler un Tool

**Autorisé :**
- Enregistrer une déclaration de capacité
- Créer une définition de permission
- Mettre à jour les métadonnées du registre

### NF6 : Ne gère jamais les identités

**Interdit :**
- Authentifier un utilisateur
- Attribuer un rôle à un utilisateur
- Vérifier l'identité d'un appelant
- Gérer les sessions utilisateur

**Autorisé :**
- Définir les permissions associées à un rôle
- Retourner les capacités accessibles pour un rôle
- Stocker les associations rôle → permission

---

## 8. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les Lois d'Autonomie Système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** Conforme

Les frontières de Master Butler garantissent une indépendance totale vis-à-vis des services externes :

- **Registre local** : Le registre des capacités et permissions est local
- **Interrogations locales** : Toutes les interrogations sont locales
- **Découverte locale** : L'API de découverte fonctionne sans connexion
- **Aucune dépendance externe** : Aucun service distant n'est requis

**Vérification :** Master Butler fonctionne-t-il si le réseau est indisponible ? → **Oui**

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** Conforme

Les frontières de Master Butler garantissent une empreinte minimale :

- **Registre de métadonnées** : Seules des métadonnées légères sont stockées
- **Pas de données métier** : Aucune donnée volumineuse n'est stockée
- **Pas d'exécution** : Aucun traitement lourd n'est effectué
- **Lookups simples** : Les opérations sont des consultations rapides

**Vérification :** Master Butler fonctionne-t-il sur un Raspberry Pi 4 ? → **Oui**

---

## 9. Conclusion

Ce contrat établit les frontières absolues et le périmètre exact de Master Butler dans l'écosystème Miyukini.

**Points clés :**
- **Périmètre clair** : Registre des capacités et permissions, API de découverte
- **Frontières absolues** : Jamais de décision, jamais d'exécution, jamais de données métier
- **Interactions définies** : Fournisseur d'informations pour StrongFather et BondingBrother
- **Invariants non négociables** : 7 invariants de frontière absolus
- **Conformité LOI-1 et LOI-5** : Autonomie et légèreté garanties

**Règle fondamentale :**

> **Master Butler connaît ce qui est possible, mais ne décide jamais de ce qui est autorisé.**

Cette règle est la ligne directrice absolue de toutes les frontières de Master Butler. Toute extension de périmètre qui violerait cette règle est interdite.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice  
**Type :** Contrat de frontières et périmètre non négociable

---

## 10. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

*Aucune erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
