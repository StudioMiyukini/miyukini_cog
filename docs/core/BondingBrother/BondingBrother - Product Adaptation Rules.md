# BondingBrother - Product Adaptation Rules

## 1. Contexte

Ce document définit les règles contractuelles d'adaptation des produits à Bonding Brother. Il spécifie comment un produit doit s'adapter à l'interface stable de Bonding Brother pour intégrer l'écosystème Miyukini, sans jamais attendre que Bonding Brother s'adapte au produit.

Ce document complète la Section 7 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur les principes établis dans l'[Architecture et Composants](./BondingBrother%20-%20Architecture%20et%20Composants.md) et l'[Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md).

L'adaptation des produits doit tenir compte des [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md) : les produits doivent gérer le mode offline (**LOI-2**) et accepter que l'état local soit souverain (**LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- Les principes fondamentaux d'adaptation unidirectionnelle
- Les règles d'implémentation de l'interface produit
- Les contraintes de format et de structure
- Les obligations de conformité
- Les mécanismes de validation de l'adaptation

Ce document **ne couvre pas** :
- Les détails de l'interface produit (voir Product Interface Contract)
- Les mécanismes d'extension (voir Extension & Specialization Contract)
- Les règles de traduction (voir Translation Contract)
- Les détails d'implémentation technique

---

## 3. Principe fondamental

**Les produits s'adaptent à Bonding Brother, jamais l'inverse.**

Bonding Brother offre une interface stable, prévisible et documentée. Tous les produits doivent implémenter cette interface sans exception. Bonding Brother ne s'adapte jamais aux spécificités d'un produit individuel.

---

## 4. Règles d'adaptation

### 4.1 Règle ADAPT-01 : Implémentation obligatoire de l'interface

**Énoncé :** Tout produit souhaitant interagir avec l'écosystème via Bonding Brother doit implémenter intégralement l'interface `IIntentSubmission` définie par Bonding Brother.

**Obligations :**
- Implémenter toutes les méthodes de l'interface
- Respecter les signatures exactes (types, paramètres, valeurs de retour)
- Gérer les erreurs selon le contrat d'erreur de Bonding Brother
- Ne pas ajouter de méthodes supplémentaires à l'interface

**Interdictions :**
- Modifier les signatures de l'interface
- Implémenter partiellement l'interface
- Créer des variantes de l'interface
- Contourner l'interface par des mécanismes alternatifs

### 4.2 Règle ADAPT-02 : Format d'intention canonique

**Énoncé :** Toute intention soumise par un produit doit respecter strictement le format canonique défini dans l'Intent Model Contract.

**Obligations :**
- Utiliser la structure JSON exacte définie
- Inclure tous les champs obligatoires
- Respecter les types de données spécifiés
- Fournir un contexte complet et valide

**Interdictions :**
- Ajouter des champs non définis dans le schéma
- Omettre des champs obligatoires
- Utiliser des formats alternatifs (XML, protobuf, etc.) sans accord explicite
- Modifier la structure pour des besoins spécifiques

### 4.3 Règle ADAPT-03 : Vocabulaire standard

**Énoncé :** Les produits doivent utiliser le vocabulaire canonique de Bonding Brother pour exprimer leurs intentions.

**Obligations :**
- Utiliser les types d'intentions canoniques (CREATE_CONTENT, UPDATE_CONTENT, etc.)
- Utiliser les noms de champs standardisés
- Respecter la sémantique définie pour chaque terme
- Mapper leur vocabulaire interne vers le vocabulaire canonique

**Interdictions :**
- Créer de nouveaux types d'intentions sans processus formel
- Utiliser un vocabulaire propriétaire non mappé
- Interpréter librement les termes canoniques
- Exiger que Bonding Brother comprenne leur vocabulaire

### 4.4 Règle ADAPT-04 : Gestion des résultats

**Énoncé :** Les produits doivent consommer les résultats selon le format défini par Bonding Brother, sans attendre de format personnalisé.

**Obligations :**
- Implémenter l'interface `IResultConsumption`
- Traiter les résultats filtrés tels quels
- Gérer les codes d'erreur standardisés
- Accepter les formats de réponse définis

**Interdictions :**
- Exiger un format de résultat personnalisé
- Ignorer les champs non compris (doivent être tolérés)
- Rejeter des résultats valides pour des raisons de format
- Contourner le filtrage en demandant des données brutes

### 4.5 Règle ADAPT-05 : Gestion des notifications

**Énoncé :** Les produits doivent s'abonner aux notifications selon le protocole défini par Bonding Brother.

**Obligations :**
- Implémenter l'interface `INotificationSubscription`
- Gérer les notifications dans le format standard
- Respecter les règles d'abonnement/désabonnement
- Traiter les notifications de manière asynchrone

**Interdictions :**
- Exiger des notifications dans un format propriétaire
- Polling actif au lieu d'abonnements
- Ignorer les notifications non comprises
- Créer des mécanismes de notification alternatifs

---

## 5. Processus d'adaptation

### 5.1 Phase 1 : Analyse de compatibilité

**Objectif :** Identifier les écarts entre l'interface actuelle du produit et l'interface requise par Bonding Brother.

**Étapes :**
1. Inventorier les interactions actuelles du produit avec l'écosystème (si existantes)
2. Identifier les types d'intentions nécessaires
3. Analyser le vocabulaire utilisé par le produit
4. Lister les écarts avec le format canonique

**Livrables :**
- Document d'analyse de compatibilité
- Liste des écarts identifiés
- Plan de migration

### 5.2 Phase 2 : Implémentation de l'interface

**Objectif :** Implémenter l'interface Bonding Brother dans le produit.

**Étapes :**
1. Intégrer la bibliothèque cliente Bonding Brother (si disponible)
2. Implémenter `IIntentSubmission`
3. Implémenter `IResultConsumption`
4. Implémenter `INotificationSubscription`
5. Créer les mappers vocabulaire produit → vocabulaire canonique

**Livrables :**
- Code source implémentant les interfaces
- Tests unitaires de conformité
- Documentation technique interne

### 5.3 Phase 3 : Adaptation des intentions

**Objectif :** Transformer la logique métier du produit pour exprimer des intentions au lieu d'exécuter directement.

**Étapes :**
1. Identifier tous les points d'interaction avec l'écosystème
2. Remplacer les appels directs par des soumissions d'intentions
3. Adapter la gestion des résultats (asynchrone)
4. Implémenter la gestion des erreurs selon le contrat

**Livrables :**
- Code source refactoré
- Tests d'intégration
- Documentation des changements

### 5.4 Phase 4 : Validation et certification

**Objectif :** Valider que l'adaptation est conforme aux règles contractuelles.

**Étapes :**
1. Tests de conformité structurelle (format, types, champs)
2. Tests de conformité sémantique (vocabulaire, types d'intentions)
3. Tests d'intégration avec Bonding Brother
4. Validation par un processus de certification (si applicable)

**Livrables :**
- Rapport de validation
- Certificat de conformité (si applicable)
- Documentation de l'adaptation

---

## 6. Contraintes et limites

### 6.1 Contrainte CONSTR-01 : Pas de négociation

**Énoncé :** Les règles d'adaptation ne sont pas négociables. Un produit ne peut pas demander d'exception ou de modification des règles pour des besoins spécifiques.

**Implications :**
- Tous les produits suivent les mêmes règles
- Aucune exception n'est accordée
- Les besoins spécifiques doivent être satisfaits par des extensions (voir Extension & Specialization Contract)

### 6.2 Contrainte CONSTR-02 : Stabilité de l'interface

**Énoncé :** L'interface de Bonding Brother est stable et évolue selon des règles strictes de versionnement. Les produits doivent suivre ces évolutions.

**Implications :**
- Les produits doivent être compatibles avec la version d'interface qu'ils utilisent
- Les mises à jour de l'interface suivent un processus de versionnement
- Les produits doivent migrer vers les nouvelles versions selon les règles de migration

### 6.3 Contrainte CONSTR-03 : Pas de couplage fort

**Énoncé :** L'adaptation ne doit pas créer de couplage fort entre le produit et Bonding Brother.

**Implications :**
- Le produit doit pouvoir fonctionner (partiellement) sans Bonding Brother
- Les erreurs de Bonding Brother ne doivent pas bloquer le produit
- Le produit doit gérer les modes offline et les déconnexions

---

## 7. Patterns d'adaptation

### 7.1 Pattern ADAPT-PAT-01 : Adapter Pattern

**Description :** Utiliser le pattern Adapter pour mapper le vocabulaire du produit vers le vocabulaire canonique.

**Structure :**
```
Produit (vocabulaire interne)
    │
    ▼
Adapter (mapping vocabulaire)
    │
    ▼
Bonding Brother (vocabulaire canonique)
```

**Avantages :**
- Séparation claire entre vocabulaire produit et vocabulaire canonique
- Facilite la maintenance
- Permet l'évolution indépendante

### 7.2 Pattern ADAPT-PAT-02 : Facade Pattern

**Description :** Créer une façade qui encapsule toute l'interaction avec Bonding Brother.

**Structure :**
```
Produit (logique métier)
    │
    ▼
BondingBrotherFacade (abstraction)
    │
    ▼
Bonding Brother (interface)
```

**Avantages :**
- Simplifie l'utilisation pour le reste du produit
- Centralise la gestion des erreurs
- Facilite les tests

### 7.3 Pattern ADAPT-PAT-03 : Observer Pattern

**Description :** Utiliser le pattern Observer pour gérer les notifications de Bonding Brother.

**Structure :**
```
Bonding Brother (émetteur)
    │
    ▼
NotificationHandler (observateur)
    │
    ▼
Produit (gestionnaire métier)
```

**Avantages :**
- Découplage entre réception et traitement
- Gestion asynchrone naturelle
- Facilite l'extension

---

## 8. Gestion des erreurs d'adaptation

### 8.1 Erreurs de format

**Type :** L'intention soumise ne respecte pas le format canonique.

**Gestion :**
- Bonding Brother rejette l'intention avec un code d'erreur de validation
- Le produit doit corriger le format et réessayer
- Aucune exception n'est accordée pour des formats non conformes

### 8.2 Erreurs de vocabulaire

**Type :** Le vocabulaire utilisé n'est pas reconnu par Bonding Brother.

**Gestion :**
- Bonding Brother rejette l'intention avec un code d'erreur de vocabulaire
- Le produit doit mapper son vocabulaire vers le vocabulaire canonique
- Aucune extension de vocabulaire n'est possible sans processus formel

### 8.3 Erreurs d'interface

**Type :** L'implémentation de l'interface ne respecte pas le contrat.

**Gestion :**
- Détectées lors des tests de conformité
- Le produit doit corriger l'implémentation
- Aucune dérogation n'est possible

---

## 9. Exemples d'adaptation

### 9.1 Exemple : Produit avec vocabulaire propriétaire

**Situation initiale :**
```typescript
// Produit utilise son propre vocabulaire
produit.createArticle({
  title: "Mon article",
  body: "Contenu..."
});
```

**Adaptation requise :**
```typescript
// Adapter vers vocabulaire canonique
const intention = {
  id: generateId(),
  produit_id: "mon-produit",
  type: "CREATE_CONTENT",  // Type canonique
  payload: {
    titre: "Mon article",  // Champ canonique
    contenu: "Contenu..."   // Champ canonique
  },
  contexte: { /* contexte complet */ },
  timestamp: new Date().toISOString(),
  version: "1.0.0"
};

bondingBrother.submitIntention(intention);
```

### 9.2 Exemple : Produit avec gestion synchrone

**Situation initiale :**
```typescript
// Produit attend une réponse synchrone
const result = database.create(content);
console.log(result.id);
```

**Adaptation requise :**
```typescript
// Adapter vers gestion asynchrone
const intention = bondingBrother.submitIntention({
  type: "CREATE_CONTENT",
  payload: content,
  // ...
});

// Gérer le résultat de manière asynchrone
bondingBrother.onResult(intention.id, (result) => {
  console.log(result.id);
});
```

---

## 10. Checklist de conformité

Avant de considérer un produit comme adapté à Bonding Brother, vérifier :

- [ ] Interface `IIntentSubmission` implémentée intégralement
- [ ] Interface `IResultConsumption` implémentée intégralement
- [ ] Interface `INotificationSubscription` implémentée intégralement
- [ ] Format d'intention respecte le schéma canonique
- [ ] Vocabulaire mappé vers le vocabulaire canonique
- [ ] Types d'intentions utilisent uniquement les types canoniques
- [ ] Contexte fourni de manière complète
- [ ] Gestion des erreurs conforme au contrat
- [ ] Gestion asynchrone des résultats
- [ ] Gestion des notifications implémentée
- [ ] Tests de conformité passés
- [ ] Documentation de l'adaptation à jour

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles d'adaptation que tous les produits doivent respecter pour intégrer l'écosystème via Bonding Brother.

Toute adaptation d'un produit doit respecter ces règles. Toute violation entraîne un rejet des intentions ou une non-certification du produit.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 7)
- Architecture et Composants v1.0
- Intent Model Contract v1.0
- Product Interface Contract (référencé, à créer)
