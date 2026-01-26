# BondingBrother - Translation Contract

## 1. Contexte

Ce document définit les règles contractuelles de traduction dans Bonding Brother. Il spécifie comment les intentions (exprimées dans le vocabulaire des produits) sont transformées en demandes (exprimées dans le vocabulaire des autorités), et comment les réponses (exprimées dans le vocabulaire des autorités) sont transformées en résultats (exprimées dans le vocabulaire des produits).

Ce document complète la Section 3 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md) pour définir les structures d'entrée et de sortie.

La traduction fonctionne localement sans dépendance externe, conformément à **LOI-1** (aucune dépendance externe critique) définie dans les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

## 2. Portée / Scope

Ce document couvre :
- Les principes fondamentaux de la traduction
- Les règles de traduction intention → demande
- Les règles de traduction réponse → résultat
- Les mappings de vocabulaire
- Les garanties de fidélité sémantique
- Les cas d'échec de traduction

Ce document **ne couvre pas** :
- Les règles de filtrage (voir Filtering & Projection Contract)
- La gestion des erreurs de traduction (voir Error & Rejection Model)
- Les détails d'implémentation des traducteurs

---

## 3. Principe fondamental

**La traduction préserve la sémantique tout en adaptant le format.**

La traduction est une transformation pure : elle ne prend aucune décision, ne modifie pas le sens, et n'a aucun effet de bord. Elle est bidirectionnelle et réversible (en théorie).

---

## 4. Directions de traduction

### 4.1 Traduction ascendante (Intention → Demande)

**Direction :** Produit → Autorité

**Entrée :** Intention (vocabulaire produit, format produit)

**Sortie :** Demande (vocabulaire autorité, format autorité)

**Objectif :** Adapter l'intention du produit au format et au vocabulaire que l'autorité comprend.

### 4.2 Traduction descendante (Réponse → Résultat)

**Direction :** Autorité → Produit

**Entrée :** Réponse (vocabulaire autorité, format autorité)

**Sortie :** Résultat (vocabulaire produit, format produit)

**Objectif :** Adapter la réponse de l'autorité au format et au vocabulaire que le produit comprend.

---

## 5. Propriétés de la traduction

### 5.1 Fidélité sémantique

**Règle FID-01 : Préservation du sens**

La traduction doit préserver le sens original. Ce que le produit veut faire doit être compris de la même manière par l'autorité.

**Règle FID-02 : Pas d'interprétation**

La traduction ne doit pas interpréter l'intention. Elle transforme le format, pas le sens.

**Règle FID-03 : Pas d'enrichissement métier**

La traduction ne doit pas ajouter d'informations métier non présentes dans l'intention originale.

**Exceptions autorisées :**
- Enrichissement technique (ajout de métadonnées de traçabilité)
- Normalisation de format (dates, nombres)
- Complétion de champs techniques obligatoires

### 5.2 Complétude

**Règle COMP-01 : Aucune perte d'information essentielle**

Toute information essentielle présente dans l'intention doit être présente dans la demande.

**Règle COMP-02 : Information non essentielle**

Les informations non essentielles peuvent être omises si elles ne sont pas supportées par l'autorité.

**Définition d'information essentielle :**
- Type d'intention
- Identifiants (produit, utilisateur, ressources)
- Données du payload nécessaires à l'évaluation
- Contexte minimal requis

### 5.3 Pureté

**Règle PUR-01 : Pas d'effet de bord**

La traduction ne doit avoir aucun effet de bord :
- Pas de modification d'état
- Pas d'appel à une autorité
- Pas de logique métier
- Pas de décision

**Règle PUR-02 : Fonction pure**

La traduction est une fonction pure : pour une même entrée, elle produit toujours la même sortie.

**Règle PUR-03 : Déterminisme**

La traduction est déterministe : pas de hasard, pas de dépendance à l'état global.

---

## 6. Traduction ascendante (Intention → Demande)

### 6.1 Structure de la demande

Une demande est la version traduite d'une intention, adaptée au vocabulaire et au format de l'autorité cible.

```typescript
interface Demande {
    // Identifiants
    demande_id: DemandeId;              // ID de la demande (peut être l'ID d'intention)
    intention_id: IntentionId;           // ID de l'intention source (traçabilité)
    
    // Type et contenu
    type: TypeDemande;                   // Type dans le vocabulaire de l'autorité
    données: DonnéesSpécifiques;         // Données traduites
    
    // Contexte (transmis intégralement)
    contexte: Contexte;                  // Contexte original (non modifié)
    
    // Métadonnées
    timestamp: Timestamp;                // Timestamp de création de la demande
    autorité_cible: AutoritéId;          // Kind Mother ou Strong Father
}
```

### 6.2 Règles de traduction ascendante

**Règle ASC-01 : Mapping de type**

Le type d'intention est mappé vers le type de demande correspondant dans le vocabulaire de l'autorité.

**Exemples de mapping :**
- `CREATE_CONTENT` (produit) → `create_content` (Kind Mother)
- `AUTHORIZE` (produit) → `check_permission` (Strong Father)

**Règle ASC-02 : Traduction du payload**

Le payload de l'intention est traduit champ par champ selon les règles de mapping définies.

**Règles de mapping de champ :**
- Nom de champ : traduit selon le vocabulaire de l'autorité
- Type de données : adapté si nécessaire (ex: string → enum)
- Structure : préservée ou adaptée selon le format de l'autorité

**Règle ASC-03 : Préservation du contexte**

Le contexte est transmis intégralement, sans modification ni filtrage.

**Règle ASC-04 : Ajout de métadonnées techniques**

Des métadonnées techniques peuvent être ajoutées (ex: `intention_id`, `timestamp_demande`), mais pas de métadonnées métier.

**Règle ASC-05 : Gestion des champs optionnels**

Les champs optionnels de l'intention peuvent être omis dans la demande si l'autorité ne les supporte pas.

**Règle ASC-06 : Validation de format**

La demande traduite doit être structurellement valide selon le schéma de l'autorité.

### 6.3 Exemple de traduction ascendante

**Intention (produit) :**
```json
{
  "id": "int-123",
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Mon article",
    "contenu": "Contenu...",
    "auteur": "user-456"
  },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:00:00Z",
  "version": "1.0.0"
}
```

**Demande traduite (Kind Mother) :**
```json
{
  "demande_id": "dem-789",
  "intention_id": "int-123",
  "type": "create_content",
  "données": {
    "title": "Mon article",
    "body": "Contenu...",
    "author_id": "user-456"
  },
  "contexte": { ... },  // Préservé intégralement
  "timestamp": "2026-01-26T10:00:00Z",
  "autorité_cible": "kind_mother"
}
```

---

## 7. Traduction descendante (Réponse → Résultat)

### 7.1 Structure du résultat

Un résultat est la version traduite d'une réponse d'autorité, adaptée au vocabulaire et au format du produit.

```typescript
interface Résultat {
    // Identifiants
    résultat_id: RésultatId;            // ID du résultat
    intention_id: IntentionId;          // ID de l'intention source
    demande_id: DemandeId;              // ID de la demande (traçabilité)
    
    // Statut
    statut: StatutRésultat;             // SUCCÈS, REFUSÉ, ERREUR
    décision: DécisionAutorité;         // Décision de l'autorité (traduite)
    
    // Données (si applicable)
    données?: DonnéesTraduites;         // Données traduites pour le produit
    
    // Erreurs (si applicable)
    erreurs?: ErreurTraduite[];         // Erreurs dans le vocabulaire produit
    
    // Métadonnées
    timestamp: Timestamp;                // Timestamp de la réponse
    autorité: AutoritéId;                // Autorité qui a répondu
}
```

### 7.2 Règles de traduction descendante

**Règle DESC-01 : Préservation de la décision**

La décision de l'autorité (acceptée, refusée, erreur) est préservée intégralement. Aucune modification n'est autorisée.

**Règle DESC-02 : Traduction du statut**

Le statut de la réponse est traduit dans le vocabulaire du produit.

**Exemples de mapping :**
- `accepted` (Kind Mother) → `SUCCÈS` (produit)
- `denied` (Strong Father) → `REFUSÉ` (produit)
- `error` (autorité) → `ERREUR` (produit)

**Règle DESC-03 : Traduction des données**

Les données de la réponse sont traduites champ par champ selon les règles de mapping définies.

**Règle DESC-04 : Traduction des erreurs**

Les erreurs de l'autorité sont traduites dans le vocabulaire du produit, avec préservation du code d'erreur technique (si nécessaire pour le support).

**Règle DESC-05 : Filtrage avant traduction**

Le filtrage de sortie est appliqué **avant** la traduction descendante (voir Filtering & Projection Contract). La traduction ne filtre pas, elle traduit ce qui a été filtré.

**Règle DESC-06 : Gestion des champs absents**

Si un champ attendu par le produit est absent de la réponse, il est omis (pas de valeur par défaut).

### 7.3 Exemple de traduction descendante

**Réponse (Kind Mother) :**
```json
{
  "response_id": "resp-456",
  "request_id": "dem-789",
  "status": "accepted",
  "data": {
    "content_id": "content-999",
    "title": "Mon article",
    "created_at": "2026-01-26T10:05:00Z"
  },
  "timestamp": "2026-01-26T10:05:00Z"
}
```

**Résultat traduit (produit) :**
```json
{
  "résultat_id": "res-111",
  "intention_id": "int-123",
  "demande_id": "dem-789",
  "statut": "SUCCÈS",
  "décision": "ACCEPTÉE",
  "données": {
    "id": "content-999",
    "titre": "Mon article",
    "créé_le": "2026-01-26T10:05:00Z"
  },
  "timestamp": "2026-01-26T10:05:00Z",
  "autorité": "kind_mother"
}
```

---

## 8. Mappings de vocabulaire

### 8.1 Registre de mappings

Les mappings de vocabulaire sont définis dans un registre centralisé, organisé par :
- Type d'intention
- Autorité cible
- Direction (ascendante ou descendante)

### 8.2 Structure d'un mapping

```typescript
interface MappingVocabulaire {
    // Identifiants
    intention_type: TypeIntention;      // Type d'intention source
    autorité: AutoritéId;                // Autorité cible
    direction: Direction;                // ASCENDANTE ou DESCENDANTE
    
    // Mappings
    mappings_champs: Map<ChampProduit, ChampAutorité>;
    mappings_types: Map<TypeProduit, TypeAutorité>;
    mappings_valeurs?: Map<ValeurProduit, ValeurAutorité>;
    
    // Règles de transformation
    transformations?: Transformation[];
}
```

### 8.3 Types de transformations

**Transformation directe :**
- Champ produit → Champ autorité (même nom ou nom mappé)
- Type préservé

**Transformation de type :**
- String → Enum
- Number → String (si nécessaire)
- Date → Timestamp

**Transformation de structure :**
- Objet imbriqué → Structure plate
- Tableau → Structure relationnelle

**Transformation de valeur :**
- Valeur produit → Valeur autorité (ex: "créer" → "create")

### 8.4 Règles de mapping

**Règle MAP-01 : Unicité**

Chaque champ produit a au plus un mapping vers un champ autorité (dans une direction donnée).

**Règle MAP-02 : Complétude**

Tous les champs obligatoires de l'autorité doivent avoir un mapping ou une valeur par défaut.

**Règle MAP-03 : Réversibilité théorique**

En théorie, un mapping doit être réversible (pour la traçabilité), même si la réversibilité n'est pas utilisée en pratique.

**Règle MAP-04 : Versioning**

Les mappings sont versionnés avec les schémas d'intention et de demande.

---

## 9. Garanties de traduction

### 9.1 Garantie de fidélité

**Engagement :** La sémantique de l'intention est préservée dans la demande, et la sémantique de la réponse est préservée dans le résultat.

**Mesure :** Tests de round-trip (traduction aller-retour) avec vérification de préservation du sens.

### 9.2 Garantie de complétude

**Engagement :** Toute information essentielle est préservée lors de la traduction.

**Mesure :** Vérification automatisée que tous les champs essentiels sont présents après traduction.

### 9.3 Garantie de déterminisme

**Engagement :** Pour une même intention, la traduction produit toujours la même demande.

**Mesure :** Tests de régression avec intentions identiques.

### 9.4 Garantie de performance

**Engagement :** La traduction est effectuée en temps constant (pas de dépendance à la taille des données au-delà de la linéarité).

**Mesure :** Métriques de temps de traduction.

---

## 10. Cas d'échec de traduction

### 10.1 Types d'échec

**Échec de mapping :**
- Type d'intention non mappé
- Champ produit sans mapping vers autorité
- Transformation impossible

**Échec de validation :**
- Demande traduite invalide selon le schéma de l'autorité
- Champ obligatoire manquant après traduction

**Échec de format :**
- Format de données incompatible
- Type de données non supporté

### 10.2 Traitement des échecs

**Règle ECHEC-01 : Rejet immédiat**

En cas d'échec de traduction, l'intention est rejetée immédiatement, sans transmission à l'autorité.

**Règle ECHEC-02 : Journalisation**

L'échec de traduction est journalisé avec :
- L'intention source
- Le type d'échec
- La raison détaillée

**Règle ECHEC-03 : Notification au produit**

Le produit reçoit un résultat avec statut `ERREUR_TRADUCTION` et un message d'erreur explicite.

**Règle ECHEC-04 : Pas de retry automatique**

Les échecs de traduction ne sont pas retentés automatiquement (ce n'est pas une erreur transitoire).

---

## 11. Extensibilité

### 11.1 Ajout de nouveaux mappings

**Règle EXT-01 : Déclaration explicite**

Tout nouveau mapping doit être déclaré explicitement dans le registre de mappings.

**Règle EXT-02 : Tests obligatoires**

Tout nouveau mapping doit être accompagné de tests de traduction (round-trip).

**Règle EXT-03 : Versioning**

Les nouveaux mappings sont versionnés avec les schémas.

### 11.2 Modification de mappings existants

**Règle EXT-04 : Rétrocompatibilité**

Les modifications de mappings doivent préserver la rétrocompatibilité ou suivre un processus de dépréciation.

**Règle EXT-05 : Migration**

Les changements de mapping nécessitent une migration des intentions en cours (si applicable).

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de traduction que Bonding Brother doit respecter pour garantir la fidélité et la complétude des transformations entre produits et autorités.

Toute traduction effectuée par Bonding Brother doit respecter ce contrat. Toute violation entraîne un rejet avec code d'erreur approprié.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 3)
- Intent Model Contract v1.0
- Architecture et Composants v1.0
- Glossaire et Terminologie v1.0
