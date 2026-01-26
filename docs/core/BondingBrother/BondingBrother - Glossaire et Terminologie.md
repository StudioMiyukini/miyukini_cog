# BondingBrother - Glossaire et Terminologie

## 1. Contexte

Ce document étend et précise le vocabulaire canonique introduit dans la Section 11 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md). Il établit le dictionnaire complet et définitif de tous les termes utilisés dans l'écosystème Bonding Brother.

Les termes liés à l'autonomie (isolement, état local, synchronisation) sont définis conformément aux [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

## 2. Portée / Scope

Ce document couvre :
- Les termes fondamentaux hérités du document fondateur
- Les termes architecturaux dérivés de la structure technique
- Les termes opérationnels utilisés dans les flux
- Les termes contractuels utilisés dans les spécifications

Ce document **établit** :
- La définition canonique et unique de chaque terme
- Les relations entre termes
- Les usages autorisés et interdits

---

## 3. Règles terminologiques

### 3.1 Règle d'unicité

Chaque concept a **un seul terme** autorisé. Les synonymes sont interdits dans la documentation officielle.

### 3.2 Règle de précision

Chaque terme a **une seule définition**. Aucune interprétation contextuelle n'est autorisée.

### 3.3 Règle de stabilité

Les termes sont **versionnés** avec la documentation. Un terme ne peut changer de sens qu'avec un changement de version majeure.

### 3.4 Règle d'usage

L'usage d'un terme non défini dans ce glossaire est **interdit** dans la documentation contractuelle.

---

## 4. Termes fondamentaux

### 4.1 Intention

**Définition :** Expression structurée par un produit de sa volonté d'effectuer une action dans l'écosystème.

**Caractéristiques :**
- N'est pas une commande
- N'est pas une instruction d'exécution
- Est une déclaration de volonté
- Requiert une évaluation par une autorité

**Forme canonique :**
```
Intention {
    type: TypeIntention,
    payload: DonnéesSpécifiques,
    contexte: Contexte,
    origine: IdentitéProduit
}
```

**Termes apparentés :**
- Demande (résultat de la traduction d'une intention)
- Requête (terme interdit — utiliser "intention" ou "demande")

---

### 4.2 Autorité

**Définition :** Entité qui détient la vérité et prend les décisions dans un domaine spécifique de l'écosystème.

**Caractéristiques :**
- Détient une vérité non contestable
- Prend des décisions non négociables
- Définit des règles dans son domaine

**Types d'autorité dans l'écosystème :**

| Autorité | Domaine | Vérité détenue |
|----------|---------|----------------|
| Kind Mother | Données | État des données, persistance, cohérence |
| Strong Father | Identités et Permissions | Qui peut faire quoi, règles politiques |

**Ce qui n'est pas une autorité :**
- Bonding Brother (médiateur, pas autorité)
- Les produits (consommateurs, pas autorités)

---

### 4.3 Contexte

**Définition :** Ensemble des informations nécessaires à l'évaluation d'une intention par une autorité.

**Éléments obligatoires :**
- `produit_id` : Identité du produit émetteur
- `utilisateur_id` : Identité de l'utilisateur (si applicable)
- `timestamp` : Moment de l'intention
- `session_id` : Identifiant de session (si applicable)

**Éléments facultatifs :**
- `permissions_déclarées` : Permissions revendiquées
- `environnement` : Contexte d'exécution (dev, staging, prod)
- `metadata` : Informations complémentaires

**Règle :** Le contexte est toujours transmis intégralement aux autorités. Bonding Brother ne filtre jamais le contexte en entrée.

---

### 4.4 Traduction

**Définition :** Transformation d'une structure (intention ou réponse) d'un vocabulaire vers un autre, en préservant la sémantique.

**Deux directions :**

| Direction | Entrée | Sortie |
|-----------|--------|--------|
| Ascendante | Intention (vocabulaire produit) | Demande (vocabulaire autorité) |
| Descendante | Réponse (vocabulaire autorité) | Résultat (vocabulaire produit) |

**Propriétés de la traduction :**
- **Fidélité** : Le sens est préservé
- **Complétude** : Aucune information essentielle n'est perdue
- **Pureté** : Aucun effet de bord, aucune décision

**Ce que la traduction n'est pas :**
- Une validation (pas de décision oui/non)
- Un filtrage (pas de suppression d'information)
- Une exécution (pas d'action métier)

---

### 4.5 Résultat filtré

**Définition :** Réponse d'une autorité qui a été transformée pour être consommable par un produit.

**Transformations appliquées :**
- Traduction vers le vocabulaire du produit
- Filtrage des informations non nécessaires
- Filtrage des informations non autorisées
- Adaptation du format

**Ce qu'un résultat filtré contient :**
- L'information demandée (si autorisée)
- Le statut de l'opération
- Les erreurs éventuelles (dans un format produit)

**Ce qu'un résultat filtré ne contient jamais :**
- Les détails internes de l'autorité
- Les informations d'autres produits
- Les données au-delà du périmètre de la demande

---

### 4.6 Délégation

**Définition :** Acte par lequel Bonding Brother transmet une demande à une autorité et attend sa décision, sans jamais décider lui-même.

**Caractéristiques :**
- Bonding Brother est l'émetteur de la délégation
- L'autorité est le récepteur de la délégation
- La décision appartient exclusivement à l'autorité
- Bonding Brother transmet fidèlement la décision

**Ce que la délégation implique :**
- Absence de décision de la part de Bonding Brother
- Transmission fidèle de la demande
- Transmission fidèle de la réponse
- Traçabilité complète

---

## 5. Termes architecturaux

### 5.1 Couche

**Définition :** Niveau d'abstraction dans l'architecture de Bonding Brother, avec une responsabilité unique et des interfaces définies.

**Couches de Bonding Brother :**

| Couche | Responsabilité | Position |
|--------|----------------|----------|
| Couche Produit | Interface vers les produits | Haute |
| Couche Traduction | Transformation des structures | Intermédiaire haute |
| Couche Médiation | Orchestration et filtrage | Intermédiaire basse |
| Couche Autorité | Interface vers les autorités | Basse |

**Règle :** Une couche ne communique qu'avec ses couches adjacentes.

---

### 5.2 Composant

**Définition :** Unité fonctionnelle au sein d'une couche, avec une responsabilité atomique et une interface définie.

**Exemples :**
- ProductGateway (composant de la Couche Produit)
- IntentTranslator (composant de la Couche Traduction)
- FilterEngine (composant de la Couche Médiation)
- KindMotherAdapter (composant de la Couche Autorité)

**Propriétés d'un composant :**
- Responsabilité unique
- Interface stable
- Implémentation encapsulée
- Indépendance fonctionnelle

---

### 5.3 Adaptateur

**Définition :** Composant spécialisé qui transforme les appels entre Bonding Brother et une entité externe (autorité).

**Adaptateurs de Bonding Brother :**
- `KindMotherAdapter` : Interface avec Kind Mother
- `StrongFatherAdapter` : Interface avec Strong Father

**Rôle de l'adaptateur :**
- Adapter les formats d'appel
- Normaliser les réponses
- Isoler Bonding Brother des détails de l'autorité

---

### 5.4 Gateway

**Définition :** Point d'entrée unique pour les interactions avec Bonding Brother depuis l'extérieur.

**ProductGateway :**
- Point d'entrée unique pour les produits
- Validation structurelle des intentions
- Routage vers les composants internes

**Propriétés d'une gateway :**
- Point unique (pas de contournement possible)
- Validation précoce
- Traçabilité dès l'entrée

---

## 6. Termes opérationnels

### 6.1 Flux

**Définition :** Séquence ordonnée d'étapes par lesquelles une donnée (intention ou réponse) transite dans Bonding Brother.

**Deux flux principaux :**

| Flux | Direction | Déclencheur |
|------|-----------|-------------|
| Flux ascendant | Produit → Écosystème | Intention d'un produit |
| Flux descendant | Écosystème → Produit | Notification ou réponse |

**Propriétés d'un flux :**
- Ordre strict des étapes
- Pas de saut d'étape
- Traçabilité à chaque étape

---

### 6.2 Filtrage

**Définition :** Application de règles pour supprimer ou masquer des informations non autorisées ou non nécessaires.

**Deux types de filtrage :**

| Type | Moment | Objectif |
|------|--------|----------|
| Filtrage d'entrée | Avant transmission à l'autorité | Protéger l'autorité des demandes invalides |
| Filtrage de sortie | Avant transmission au produit | Protéger le produit des informations non autorisées |

**Ce que le filtrage ne fait pas :**
- Ne prend pas de décision métier
- Ne modifie pas la sémantique
- Ne remplace pas l'autorité

---

### 6.3 Journalisation

**Définition :** Enregistrement systématique et horodaté de toutes les interactions transitant par Bonding Brother.

**Éléments journalisés :**
- Toute intention reçue
- Toute demande transmise
- Toute réponse reçue
- Tout résultat émis
- Toute erreur survenue

**Propriétés de la journalisation :**
- **Systématique** : Aucune exception
- **Complète** : Toutes les informations nécessaires à la reconstitution
- **Non contournable** : Pas d'option pour désactiver
- **Immuable** : Pas de modification après enregistrement

---

### 6.4 Mode offline

**Définition :** État de fonctionnement de Bonding Brother lorsque les autorités ne sont pas accessibles immédiatement.

**Comportement en mode offline :**
- Réception des intentions : continue normalement
- Traduction : continue normalement
- Transmission aux autorités : différée
- Journalisation : continue normalement (avec marqueur offline)

**Termes associés :**
- **Buffer offline** : Stockage temporaire des demandes en attente
- **Autorité différée** : Autorité qui sera consultée à la reconnexion
- **Synchronisation** : Processus de transmission des demandes buffées

---

### 6.5 Synchronisation

**Définition :** Processus de transmission des intentions journalisées pendant le mode offline vers les autorités une fois la connexion rétablie.

**Étapes de la synchronisation :**
1. Détection de la reconnexion
2. Récupération des intentions buffées (ordre chronologique)
3. Transmission séquentielle aux autorités
4. Réception des réponses différées
5. Transmission des résultats aux produits

**Propriétés :**
- Ordre préservé (FIFO)
- Aucune perte d'intention
- Traçabilité complète

---

## 7. Termes contractuels

### 7.1 Contrat

**Définition :** Document normatif qui définit les règles, interfaces, ou comportements que Bonding Brother s'engage à respecter.

**Types de contrats :**

| Type | Portée | Exemple |
|------|--------|---------|
| Contrat d'interface | API et formats | Product Interface Contract |
| Contrat d'intégration | Interactions avec autorités | KindMother Integration Contract |
| Contrat de comportement | Invariants et garanties | Invariants & Guarantees |

---

### 7.2 Invariant

**Définition :** Propriété qui doit toujours être vraie, quelles que soient les circonstances, et qui ne peut jamais être violée.

**Exemples d'invariants de Bonding Brother :**
- Bonding Brother ne décide jamais
- Bonding Brother ne stocke jamais la vérité
- Bonding Brother journalise toujours

**Propriétés d'un invariant :**
- Non négociable
- Non configurable
- Non désactivable
- Vérifié structurellement

---

### 7.3 Garantie

**Définition :** Engagement de Bonding Brother envers ses consommateurs (produits ou autorités) sur un comportement ou une propriété.

**Exemples de garanties :**
- Traduction fidèle (la sémantique est préservée)
- Transmission complète (rien n'est perdu)
- Traçabilité totale (tout est enregistré)

**Différence avec l'invariant :**
- L'invariant est interne (Bonding Brother s'impose à lui-même)
- La garantie est externe (Bonding Brother promet aux autres)

---

### 7.4 Violation

**Définition :** Situation où une règle, un invariant, ou un contrat n'est pas respecté.

**Traitement des violations :**
- Violations d'invariant : Impossible par construction (erreur de conception si détectée)
- Violations de contrat : Rejet de l'opération, journalisation, notification

---

## 8. Termes interdits

Les termes suivants sont **interdits** dans la documentation de Bonding Brother car ils sont ambigus ou porteurs de mauvaises connotations :

| Terme interdit | Raison | Terme à utiliser |
|----------------|--------|------------------|
| Requête | Ambigu (query vs request) | Intention ou Demande |
| Ordre | Implique une exécution directe | Intention |
| Commande | Implique une exécution directe | Intention |
| Cache | Implique un stockage de vérité | Buffer ou Journal |
| Base de données | Bonding Brother ne stocke pas | Journal |
| Décision | Bonding Brother ne décide pas | Délégation |
| Règle (créée par BB) | BB ne crée pas de règles | Règle (définie par autorité) |
| Proxy | Trop technique, ambigu | Médiateur |

---

## 9. Index alphabétique

| Terme | Section | Catégorie |
|-------|---------|-----------|
| Adaptateur | 5.3 | Architectural |
| Autorité | 4.2 | Fondamental |
| Composant | 5.2 | Architectural |
| Contrat | 7.1 | Contractuel |
| Contexte | 4.3 | Fondamental |
| Couche | 5.1 | Architectural |
| Délégation | 4.6 | Fondamental |
| Filtrage | 6.2 | Opérationnel |
| Flux | 6.1 | Opérationnel |
| Garantie | 7.3 | Contractuel |
| Gateway | 5.4 | Architectural |
| Intention | 4.1 | Fondamental |
| Invariant | 7.2 | Contractuel |
| Journalisation | 6.3 | Opérationnel |
| Mode offline | 6.4 | Opérationnel |
| Résultat filtré | 4.5 | Fondamental |
| Synchronisation | 6.5 | Opérationnel |
| Traduction | 4.4 | Fondamental |
| Violation | 7.4 | Contractuel |

---

## 10. Statut contractuel

Ce document est **contractuel, normatif, et de statut RÉFÉRENCE**. Il établit le vocabulaire officiel de Bonding Brother qui doit être utilisé dans toute documentation, code, et communication.

Tout terme utilisé dans un document contractuel de Bonding Brother doit être défini dans ce glossaire. Toute modification terminologique nécessite une nouvelle version de ce document.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** RÉFÉRENCE — Normatif  
**Dépendance :** Documentation Fondatrice v1.0 (Section 11)
