# BondingBrother - Product Adaptation Rules

## 1. Contexte

Ce document dÃ©finit les rÃ¨gles contractuelles d'adaptation des produits Ã  Bonding Brother. Il spÃ©cifie comment un produit doit s'adapter Ã  l'interface stable de Bonding Brother pour intÃ©grer l'Ã©cosystÃ¨me Miyukini, sans jamais attendre que Bonding Brother s'adapte au produit.

Ce document complÃ¨te la Section 7 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur les principes Ã©tablis dans l'[Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md) et l'[Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md).

L'adaptation des produits doit tenir compte des [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) : les produits doivent gÃ©rer le mode offline (**LOI-2**) et accepter que l'Ã©tat local soit souverain (**LOI-3**).

## 2. PortÃ©e / Scope

Ce document couvre :
- Les principes fondamentaux d'adaptation unidirectionnelle
- Les rÃ¨gles d'implÃ©mentation de l'interface produit
- Les contraintes de format et de structure
- Les obligations de conformitÃ©
- Les mÃ©canismes de validation de l'adaptation

Ce document **ne couvre pas** :
- Les dÃ©tails de l'interface produit (voir [Product Interface Contract](./BondingBrother%20-%20Product%20Interface%20Contract.md))
- Les mÃ©canismes d'extension (voir [Extension & Specialization Contract](./BondingBrother%20-%20Extension%20&%20Specialization%20Contract.md))
- Les rÃ¨gles de traduction (voir [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md))
- Les dÃ©tails d'implÃ©mentation technique

---

## 3. Principe fondamental

**Les produits s'adaptent Ã  Bonding Brother, jamais l'inverse.**

Bonding Brother offre une interface stable, prÃ©visible et documentÃ©e. Tous les produits doivent implÃ©menter cette interface sans exception. Bonding Brother ne s'adapte jamais aux spÃ©cificitÃ©s d'un produit individuel.

---

## 4. RÃ¨gles d'adaptation

### 4.1 RÃ¨gle ADAPT-01 : ImplÃ©mentation obligatoire de l'interface

**Ã‰noncÃ© :** Tout produit souhaitant interagir avec l'Ã©cosystÃ¨me via Bonding Brother doit implÃ©menter intÃ©gralement l'interface `IIntentSubmission` dÃ©finie par Bonding Brother.

**Obligations :**
- ImplÃ©menter toutes les mÃ©thodes de l'interface
- Respecter les signatures exactes (types, paramÃ¨tres, valeurs de retour)
- GÃ©rer les erreurs selon le contrat d'erreur de Bonding Brother
- Ne pas ajouter de mÃ©thodes supplÃ©mentaires Ã  l'interface

**Interdictions :**
- Modifier les signatures de l'interface
- ImplÃ©menter partiellement l'interface
- CrÃ©er des variantes de l'interface
- Contourner l'interface par des mÃ©canismes alternatifs

### 4.2 RÃ¨gle ADAPT-02 : Format d'intention canonique

**Ã‰noncÃ© :** Toute intention soumise par un produit doit respecter strictement le format canonique dÃ©fini dans l'[Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md).

**Obligations :**
- Utiliser la structure JSON exacte dÃ©finie
- Inclure tous les champs obligatoires
- Respecter les types de donnÃ©es spÃ©cifiÃ©s
- Fournir un contexte complet et valide

**Interdictions :**
- Ajouter des champs non dÃ©finis dans le schÃ©ma
- Omettre des champs obligatoires
- Utiliser des formats alternatifs (XML, protobuf, etc.) sans accord explicite
- Modifier la structure pour des besoins spÃ©cifiques

### 4.3 RÃ¨gle ADAPT-03 : Vocabulaire standard

**Ã‰noncÃ© :** Les produits doivent utiliser le vocabulaire canonique de Bonding Brother pour exprimer leurs intentions.

**Obligations :**
- Utiliser les types d'intentions canoniques (CREATE_CONTENT, UPDATE_CONTENT, etc.)
- Utiliser les noms de champs standardisÃ©s
- Respecter la sÃ©mantique dÃ©finie pour chaque terme
- Mapper leur vocabulaire interne vers le vocabulaire canonique

**Interdictions :**
- CrÃ©er de nouveaux types d'intentions sans processus formel
- Utiliser un vocabulaire propriÃ©taire non mappÃ©
- InterprÃ©ter librement les termes canoniques
- Exiger que Bonding Brother comprenne leur vocabulaire

### 4.4 RÃ¨gle ADAPT-04 : Gestion des rÃ©sultats

**Ã‰noncÃ© :** Les produits doivent consommer les rÃ©sultats selon le format dÃ©fini par Bonding Brother, sans attendre de format personnalisÃ©.

**Obligations :**
- ImplÃ©menter l'interface `IResultConsumption`
- Traiter les rÃ©sultats filtrÃ©s tels quels
- GÃ©rer les codes d'erreur standardisÃ©s
- Accepter les formats de rÃ©ponse dÃ©finis

**Interdictions :**
- Exiger un format de rÃ©sultat personnalisÃ©
- Ignorer les champs non compris (doivent Ãªtre tolÃ©rÃ©s)
- Rejeter des rÃ©sultats valides pour des raisons de format
- Contourner le filtrage en demandant des donnÃ©es brutes

### 4.5 RÃ¨gle ADAPT-05 : Gestion des notifications

**Ã‰noncÃ© :** Les produits doivent s'abonner aux notifications selon le protocole dÃ©fini par Bonding Brother.

**Obligations :**
- ImplÃ©menter l'interface `INotificationSubscription`
- GÃ©rer les notifications dans le format standard
- Respecter les rÃ¨gles d'abonnement/dÃ©sabonnement
- Traiter les notifications de maniÃ¨re asynchrone

**Interdictions :**
- Exiger des notifications dans un format propriÃ©taire
- Polling actif au lieu d'abonnements
- Ignorer les notifications non comprises
- CrÃ©er des mÃ©canismes de notification alternatifs

---

## 5. Processus d'adaptation

### 5.1 Phase 1 : Analyse de compatibilitÃ©

**Objectif :** Identifier les Ã©carts entre l'interface actuelle du produit et l'interface requise par Bonding Brother.

**Ã‰tapes :**
1. Inventorier les interactions actuelles du produit avec l'Ã©cosystÃ¨me (si existantes)
2. Identifier les types d'intentions nÃ©cessaires
3. Analyser le vocabulaire utilisÃ© par le produit
4. Lister les Ã©carts avec le format canonique

**Livrables :**
- Document d'analyse de compatibilitÃ©
- Liste des Ã©carts identifiÃ©s
- Plan de migration

### 5.2 Phase 2 : ImplÃ©mentation de l'interface

**Objectif :** ImplÃ©menter l'interface Bonding Brother dans le produit.

**Ã‰tapes :**
1. IntÃ©grer la bibliothÃ¨que cliente Bonding Brother (si disponible)
2. ImplÃ©menter `IIntentSubmission`
3. ImplÃ©menter `IResultConsumption`
4. ImplÃ©menter `INotificationSubscription`
5. CrÃ©er les mappers vocabulaire produit â†’ vocabulaire canonique

**Livrables :**
- Code source implÃ©mentant les interfaces
- Tests unitaires de conformitÃ©
- Documentation technique interne

### 5.3 Phase 3 : Adaptation des intentions

**Objectif :** Transformer la logique mÃ©tier du produit pour exprimer des intentions au lieu d'exÃ©cuter directement.

**Ã‰tapes :**
1. Identifier tous les points d'interaction avec l'Ã©cosystÃ¨me
2. Remplacer les appels directs par des soumissions d'intentions
3. Adapter la gestion des rÃ©sultats (asynchrone)
4. ImplÃ©menter la gestion des erreurs selon le contrat

**Livrables :**
- Code source refactorÃ©
- Tests d'intÃ©gration
- Documentation des changements

### 5.4 Phase 4 : Validation et certification

**Objectif :** Valider que l'adaptation est conforme aux rÃ¨gles contractuelles.

**Ã‰tapes :**
1. Tests de conformitÃ© structurelle (format, types, champs)
2. Tests de conformitÃ© sÃ©mantique (vocabulaire, types d'intentions)
3. Tests d'intÃ©gration avec Bonding Brother
4. Validation par un processus de certification (si applicable)

**Livrables :**
- Rapport de validation
- Certificat de conformitÃ© (si applicable)
- Documentation de l'adaptation

---

## 6. Contraintes et limites

### 6.1 Contrainte CONSTR-01 : Pas de nÃ©gociation

**Ã‰noncÃ© :** Les rÃ¨gles d'adaptation ne sont pas nÃ©gociables. Un produit ne peut pas demander d'exception ou de modification des rÃ¨gles pour des besoins spÃ©cifiques.

**Implications :**
- Tous les produits suivent les mÃªmes rÃ¨gles
- Aucune exception n'est accordÃ©e
- Les besoins spÃ©cifiques doivent Ãªtre satisfaits par des extensions (voir [Extension & Specialization Contract](./BondingBrother%20-%20Extension%20&%20Specialization%20Contract.md))

### 6.2 Contrainte CONSTR-02 : StabilitÃ© de l'interface

**Ã‰noncÃ© :** L'interface de Bonding Brother est stable et Ã©volue selon des rÃ¨gles strictes de versionnement. Les produits doivent suivre ces Ã©volutions.

**Implications :**
- Les produits doivent Ãªtre compatibles avec la version d'interface qu'ils utilisent
- Les mises Ã  jour de l'interface suivent un processus de versionnement
- Les produits doivent migrer vers les nouvelles versions selon les rÃ¨gles de migration

### 6.3 Contrainte CONSTR-03 : Pas de couplage fort

**Ã‰noncÃ© :** L'adaptation ne doit pas crÃ©er de couplage fort entre le produit et Bonding Brother.

**Implications :**
- Le produit doit pouvoir fonctionner (partiellement) sans Bonding Brother
- Les erreurs de Bonding Brother ne doivent pas bloquer le produit
- Le produit doit gÃ©rer les modes offline et les dÃ©connexions

---

## 7. Patterns d'adaptation

### 7.1 Pattern ADAPT-PAT-01 : Adapter Pattern

**Description :** Utiliser le pattern Adapter pour mapper le vocabulaire du produit vers le vocabulaire canonique.

**Structure :**
```
Produit (vocabulaire interne)
    â”‚
    â–¼
Adapter (mapping vocabulaire)
    â”‚
    â–¼
Bonding Brother (vocabulaire canonique)
```

**Avantages :**
- SÃ©paration claire entre vocabulaire produit et vocabulaire canonique
- Facilite la maintenance
- Permet l'Ã©volution indÃ©pendante

### 7.2 Pattern ADAPT-PAT-02 : Facade Pattern

**Description :** CrÃ©er une faÃ§ade qui encapsule toute l'interaction avec Bonding Brother.

**Structure :**
```
Produit (logique mÃ©tier)
    â”‚
    â–¼
BondingBrotherFacade (abstraction)
    â”‚
    â–¼
Bonding Brother (interface)
```

**Avantages :**
- Simplifie l'utilisation pour le reste du produit
- Centralise la gestion des erreurs
- Facilite les tests

### 7.3 Pattern ADAPT-PAT-03 : Observer Pattern

**Description :** Utiliser le pattern Observer pour gÃ©rer les notifications de Bonding Brother.

**Structure :**
```
Bonding Brother (Ã©metteur)
    â”‚
    â–¼
NotificationHandler (observateur)
    â”‚
    â–¼
Produit (gestionnaire mÃ©tier)
```

**Avantages :**
- DÃ©couplage entre rÃ©ception et traitement
- Gestion asynchrone naturelle
- Facilite l'extension

---

## 8. Gestion des erreurs d'adaptation

### 8.1 Erreurs de format

**Type :** L'intention soumise ne respecte pas le format canonique.

**Gestion :**
- Bonding Brother rejette l'intention avec un code d'erreur de validation
- Le produit doit corriger le format et rÃ©essayer
- Aucune exception n'est accordÃ©e pour des formats non conformes

### 8.2 Erreurs de vocabulaire

**Type :** Le vocabulaire utilisÃ© n'est pas reconnu par Bonding Brother.

**Gestion :**
- Bonding Brother rejette l'intention avec un code d'erreur de vocabulaire
- Le produit doit mapper son vocabulaire vers le vocabulaire canonique
- Aucune extension de vocabulaire n'est possible sans processus formel

### 8.3 Erreurs d'interface

**Type :** L'implÃ©mentation de l'interface ne respecte pas le contrat.

**Gestion :**
- DÃ©tectÃ©es lors des tests de conformitÃ©
- Le produit doit corriger l'implÃ©mentation
- Aucune dÃ©rogation n'est possible

---

## 9. Exemples d'adaptation

### 9.1 Exemple : Produit avec vocabulaire propriÃ©taire

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
// Produit attend une rÃ©ponse synchrone
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

// GÃ©rer le rÃ©sultat de maniÃ¨re asynchrone
bondingBrother.onResult(intention.id, (result) => {
  console.log(result.id);
});
```

---

## 10. Checklist de conformitÃ©

Avant de considÃ©rer un produit comme adaptÃ© Ã  Bonding Brother, vÃ©rifier :

- [ ] Interface `IIntentSubmission` implÃ©mentÃ©e intÃ©gralement
- [ ] Interface `IResultConsumption` implÃ©mentÃ©e intÃ©gralement
- [ ] Interface `INotificationSubscription` implÃ©mentÃ©e intÃ©gralement
- [ ] Format d'intention respecte le schÃ©ma canonique
- [ ] Vocabulaire mappÃ© vers le vocabulaire canonique
- [ ] Types d'intentions utilisent uniquement les types canoniques
- [ ] Contexte fourni de maniÃ¨re complÃ¨te
- [ ] Gestion des erreurs conforme au contrat
- [ ] Gestion asynchrone des rÃ©sultats
- [ ] Gestion des notifications implÃ©mentÃ©e
- [ ] Tests de conformitÃ© passÃ©s
- [ ] Documentation de l'adaptation Ã  jour

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit les rÃ¨gles d'adaptation que tous les produits doivent respecter pour intÃ©grer l'Ã©cosystÃ¨me via Bonding Brother.

Toute adaptation d'un produit doit respecter ces rÃ¨gles. Toute violation entraÃ®ne un rejet des intentions ou une non-certification du produit.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0 (Section 7)
- [Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md) v2.0
- [Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) v2.0
- [Product Interface Contract](./BondingBrother%20-%20Product%20Interface%20Contract.md) v2.0

