# BondingBrother - Extension and Specialization Contract

## 1. Contexte

Ce document définit le mécanisme contractuel d'extension de Bonding Brother par spécialisation. Il spécifie comment de nouvelles capacités peuvent être ajoutées à Bonding Brother sans modifier son cœur stable, en suivant le principe d'extension par spécialisation établi dans la Documentation Fondatrice.

Ce document complète la Section 7 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Architecture et Composants](./BondingBrother%20-%20Architecture%20et%20Composants.md) et les [Product Adaptation Rules](./BondingBrother%20-%20Product%20Adaptation%20Rules.md).

Les extensions doivent respecter les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md) : elles ne peuvent pas introduire de dépendances externes critiques (**LOI-1**) ni compromettre le fonctionnement en mode offline (**LOI-2**).

## 2. Portée / Scope

Ce document couvre :
- Le principe d'extension par spécialisation
- Les points d'extension autorisés
- Les règles de création de spécialisations
- Les contraintes de compatibilité
- Le processus de versionnement des extensions

Ce document **ne couvre pas** :
- Les règles d'adaptation des produits (voir Product Adaptation Rules)
- Les détails d'implémentation technique
- Les règles de migration (voir Migration & Compatibility Contract)

---

## 3. Principe fondamental

**Bonding Brother s'étend par spécialisation, jamais par modification du cœur.**

Le cœur de Bonding Brother (ses principes, ses invariants, ses relations avec les autorités) reste immuable. Toute nouvelle capacité est ajoutée par création d'une spécialisation qui étend le cœur sans le modifier.

---

## 4. Architecture d'extension

### 4.1 Structure en couches extensible

```
┌─────────────────────────────────────────┐
│     COUCHE SPÉCIALISATION (extensible) │
│  (Nouvelles interfaces, nouveaux types)│
├─────────────────────────────────────────┤
│     COEUR STABLE (non modifiable)       │
│  (Principes, invariants, médiation)    │
└─────────────────────────────────────────┘
```

### 4.2 Principe de séparation

**Règle EXT-01 : Séparation cœur / spécialisation**

Le cœur de Bonding Brother et ses spécialisations sont strictement séparés :
- Le cœur ne connaît pas les spécialisations
- Les spécialisations étendent le cœur, ne le modifient pas
- Aucune dépendance du cœur vers les spécialisations

**Règle EXT-02 : Interface stable du cœur**

L'interface du cœur reste stable et ne change jamais. Les spécialisations ajoutent de nouvelles interfaces, jamais ne modifient les interfaces existantes.

---

## 5. Points d'extension autorisés

### 5.1 Extension des types d'intentions

**Point d'extension :** Ajout de nouveaux types d'intentions canoniques.

**Règles :**
- Un nouveau type doit être justifié métier
- Un nouveau type doit cibler une autorité identifiée
- Un nouveau type doit avoir un schéma de payload défini
- Un nouveau type doit être documenté

**Processus :**
1. Proposition du nouveau type avec justification
2. Définition du schéma de payload
3. Identification de l'autorité cible
4. Validation par processus formel
5. Ajout à la liste des types canoniques
6. Mise à jour de la documentation

**Exemple :**
```typescript
// Type canonique existant
type: "CREATE_CONTENT"

// Nouveau type proposé (après validation)
type: "PUBLISH_CONTENT"  // Nouvelle spécialisation
```

### 5.2 Extension des interfaces produit

**Point d'extension :** Ajout de nouvelles interfaces spécialisées pour des besoins spécifiques.

**Règles :**
- Les nouvelles interfaces doivent étendre les interfaces de base
- Les nouvelles interfaces ne doivent pas modifier les interfaces existantes
- Les nouvelles interfaces doivent être optionnelles (rétrocompatibilité)
- Les nouvelles interfaces doivent être documentées

**Processus :**
1. Identification du besoin non couvert par l'interface de base
2. Conception de l'interface spécialisée
3. Validation de la non-régression
4. Implémentation de l'interface
5. Documentation et versionnement

**Exemple :**
```typescript
// Interface de base (cœur)
interface IIntentSubmission {
  submitIntention(intention: Intention): Promise<IntentionId>;
}

// Interface spécialisée (extension)
interface IBatchIntentSubmission extends IIntentSubmission {
  submitBatchIntentions(intentions: Intention[]): Promise<IntentionId[]>;
}
```

### 5.3 Extension des vocabulaires

**Point d'extension :** Ajout de mappings de vocabulaire pour de nouveaux produits.

**Règles :**
- Les nouveaux mappings ne modifient pas les mappings existants
- Les nouveaux mappings doivent être réversibles
- Les nouveaux mappings doivent préserver la sémantique
- Les nouveaux mappings doivent être documentés

**Processus :**
1. Analyse du vocabulaire du produit
2. Création du mapping vers vocabulaire canonique
3. Validation de la préservation sémantique
4. Ajout du mapping au système
5. Tests de traduction bidirectionnelle

### 5.4 Extension des règles de filtrage

**Point d'extension :** Ajout de nouvelles règles de filtrage pour des besoins spécifiques.

**Règles :**
- Les nouvelles règles ne doivent pas modifier les règles existantes
- Les nouvelles règles doivent être définies par une autorité
- Les nouvelles règles doivent être documentées
- Les nouvelles règles doivent être testables

**Processus :**
1. Identification du besoin de filtrage
2. Définition de la règle par l'autorité concernée
3. Implémentation de la règle dans FilterEngine
4. Tests de la règle
5. Documentation

---

## 6. Contraintes de spécialisation

### 6.1 Contrainte SPEC-01 : Préservation des invariants

**Énoncé :** Toute spécialisation doit préserver tous les invariants du cœur de Bonding Brother.

**Invariants à préserver :**
- Bonding Brother ne décide jamais
- Bonding Brother ne stocke jamais la vérité
- Bonding Brother journalise toujours
- Bonding Brother ne contourne jamais les autorités
- Bonding Brother ne modifie jamais les décisions des autorités

**Vérification :**
- Analyse statique du code de spécialisation
- Tests de non-régression des invariants
- Audit formel avant validation

### 6.2 Contrainte SPEC-02 : Rétrocompatibilité

**Énoncé :** Toute spécialisation doit être rétrocompatible avec le cœur et les spécialisations existantes.

**Obligations :**
- Les produits utilisant le cœur continuent de fonctionner
- Les produits utilisant des spécialisations existantes continuent de fonctionner
- Aucune régression de fonctionnalité
- Aucune modification de comportement existant

**Vérification :**
- Tests de régression complets
- Validation avec produits existants
- Tests d'intégration

### 6.3 Contrainte SPEC-03 : Isolation

**Énoncé :** Les spécialisations sont isolées les unes des autres et du cœur.

**Obligations :**
- Une spécialisation ne peut pas dépendre d'une autre spécialisation
- Une spécialisation ne peut pas modifier le cœur
- Le cœur ne peut pas dépendre d'une spécialisation
- Les spécialisations peuvent coexister sans interférence

**Vérification :**
- Analyse des dépendances
- Tests d'isolation
- Validation architecturale

### 6.4 Contrainte SPEC-04 : Documentabilité

**Énoncé :** Toute spécialisation doit être documentée de manière complète.

**Obligations :**
- Documentation de l'objectif et de la justification
- Documentation de l'interface et de l'utilisation
- Documentation des contraintes et limitations
- Documentation des exemples d'utilisation

**Vérification :**
- Revue de documentation
- Validation de complétude
- Tests d'utilisation basés sur la documentation

---

## 7. Processus de création d'une spécialisation

### 7.1 Phase 1 : Proposition

**Objectif :** Documenter le besoin et justifier la création d'une spécialisation.

**Étapes :**
1. Identifier le besoin non couvert par le cœur ou les spécialisations existantes
2. Analyser si une modification du cœur serait nécessaire (interdit)
3. Concevoir la spécialisation proposée
4. Documenter la justification métier
5. Soumettre la proposition

**Livrables :**
- Document de proposition
- Justification métier
- Conception préliminaire
- Analyse d'impact

### 7.2 Phase 2 : Validation

**Objectif :** Valider que la spécialisation respecte toutes les contraintes.

**Étapes :**
1. Vérification de la préservation des invariants
2. Vérification de la rétrocompatibilité
3. Vérification de l'isolation
4. Validation architecturale
5. Approbation formelle

**Livrables :**
- Rapport de validation
- Approbation formelle
- Plan d'implémentation

### 7.3 Phase 3 : Implémentation

**Objectif :** Implémenter la spécialisation selon les règles établies.

**Étapes :**
1. Créer la structure de la spécialisation (séparée du cœur)
2. Implémenter l'interface de spécialisation
3. Implémenter les règles de traduction (si nécessaire)
4. Implémenter les règles de filtrage (si nécessaire)
5. Créer les tests unitaires et d'intégration

**Livrables :**
- Code source de la spécialisation
- Tests unitaires
- Tests d'intégration
- Documentation technique

### 7.4 Phase 4 : Tests et validation

**Objectif :** Valider que la spécialisation fonctionne correctement et respecte toutes les contraintes.

**Étapes :**
1. Tests de fonctionnalité
2. Tests de non-régression
3. Tests de rétrocompatibilité
4. Tests d'isolation
5. Tests de performance
6. Validation finale

**Livrables :**
- Rapport de tests
- Validation finale
- Documentation utilisateur

### 7.5 Phase 5 : Publication

**Objectif :** Publier la spécialisation et la rendre disponible.

**Étapes :**
1. Versionnement de la spécialisation
2. Mise à jour de la documentation
3. Publication dans le registre des spécialisations
4. Communication aux produits
5. Formation si nécessaire

**Livrables :**
- Version publiée de la spécialisation
- Documentation mise à jour
- Registre des spécialisations mis à jour

---

## 8. Versionnement des spécialisations

### 8.1 Principe de versionnement

**Règle VERS-01 : Versionnement indépendant**

Chaque spécialisation a son propre numéro de version, indépendant du cœur et des autres spécialisations.

**Format :** `v<major>.<minor>.<patch>`

**Règle VERS-02 : Compatibilité du cœur**

Le cœur de Bonding Brother reste stable. Les spécialisations doivent être compatibles avec la version du cœur qu'elles étendent.

**Règle VERS-03 : Évolution des spécialisations**

Les spécialisations peuvent évoluer indépendamment :
- Version majeure : changement non rétrocompatible
- Version mineure : ajout de fonctionnalités rétrocompatibles
- Version patch : corrections de bugs

### 8.2 Registre des spécialisations

**Contenu du registre :**
- Nom de la spécialisation
- Version actuelle
- Version du cœur requise
- Description et objectif
- Interface exposée
- Date de création
- Statut (active, dépréciée, obsolète)

**Mise à jour :**
- Automatique lors de la publication
- Accessible publiquement
- Versionné

---

## 9. Exemples de spécialisations

### 9.1 Exemple : Spécialisation pour traitement par lots

**Besoin :** Permettre aux produits de soumettre plusieurs intentions en une seule opération.

**Spécialisation :**
```typescript
interface IBatchIntentSubmission {
  submitBatchIntentions(
    intentions: Intention[]
  ): Promise<BatchResult>;
}

interface BatchResult {
  intention_ids: IntentionId[];
  errors?: BatchError[];
}
```

**Caractéristiques :**
- Étend l'interface de base sans la modifier
- Optionnelle (rétrocompatibilité)
- Isolée du cœur
- Documentée

### 9.2 Exemple : Spécialisation pour streaming

**Besoin :** Permettre aux produits de recevoir des résultats en streaming pour des opérations longues.

**Spécialisation :**
```typescript
interface IStreamingResultConsumption {
  subscribeToStream(
    intention_id: IntentionId,
    callback: (chunk: ResultChunk) => void
  ): Promise<StreamSubscription>;
}
```

**Caractéristiques :**
- Nouvelle interface, n'affecte pas l'interface de base
- Optionnelle
- Isolée
- Documentée

---

## 10. Anti-patterns interdits

### 10.1 Anti-pattern AP-01 : Modification du cœur

**Description :** Modifier directement le code du cœur pour ajouter une fonctionnalité.

**Pourquoi interdit :** Violation du principe de stabilité du cœur.

**Solution :** Créer une spécialisation qui étend le cœur.

### 10.2 Anti-pattern AP-02 : Dépendance entre spécialisations

**Description :** Une spécialisation dépend d'une autre spécialisation.

**Pourquoi interdit :** Violation du principe d'isolation.

**Solution :** Réorganiser pour que chaque spécialisation soit indépendante, ou fusionner si nécessaire.

### 10.3 Anti-pattern AP-03 : Spécialisation obligatoire

**Description :** Rendre une spécialisation obligatoire pour tous les produits.

**Pourquoi interdit :** Violation du principe de rétrocompatibilité.

**Solution :** Intégrer la fonctionnalité dans le cœur si elle est universelle, ou la garder optionnelle.

### 10.4 Anti-pattern AP-04 : Spécialisation qui modifie le comportement existant

**Description :** Une spécialisation modifie le comportement d'une interface existante.

**Pourquoi interdit :** Violation du principe de rétrocompatibilité.

**Solution :** Créer une nouvelle interface spécialisée au lieu de modifier l'existante.

---

## 11. Checklist de création d'une spécialisation

Avant de publier une spécialisation, vérifier :

- [ ] Le besoin ne peut pas être satisfait par le cœur ou une spécialisation existante
- [ ] La spécialisation préserve tous les invariants du cœur
- [ ] La spécialisation est rétrocompatible
- [ ] La spécialisation est isolée (pas de dépendances vers autres spécialisations)
- [ ] La spécialisation est documentée complètement
- [ ] Les tests de non-régression passent
- [ ] Les tests d'isolation passent
- [ ] La spécialisation est versionnée
- [ ] Le registre des spécialisations est mis à jour
- [ ] La documentation utilisateur est à jour

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles d'extension de Bonding Brother que toute spécialisation doit respecter.

Toute spécialisation de Bonding Brother doit respecter ces règles. Toute violation entraîne un rejet de la spécialisation ou une non-certification.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 7)
- Architecture et Composants v1.0
- Product Adaptation Rules v1.0
- Product Interface Contract (référencé, à créer)
