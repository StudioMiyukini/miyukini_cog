# BondingBrother — Translation Contract

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif

---

## 1. Contexte

Ce document définit les règles contractuelles de traduction dans Bonding Brother. Il spécifie comment les intentions (exprimées dans le vocabulaire des produits) sont transformées en demandes (exprimées dans le vocabulaire des autorités), et comment les réponses sont transformées en résultats.

**Dépendances :**
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) — Section 5 (Nature du lien)
- [Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

La traduction fonctionne localement sans dépendance externe, conformément à **LOI-1** (aucune dépendance externe critique).

## 2. Portée / Scope

Ce document couvre :
- Les principes fondamentaux de la traduction
- Les règles de traduction intention → demande
- Les règles de traduction réponse → résultat
- Les mappings de vocabulaire
- Les garanties de fidélité sémantique
- Les cas d'échec de traduction

Ce document **ne couvre pas** :
- Les règles de filtrage (voir [Filtering & Projection Contract](./BondingBrother%20-%20Filtering%20&%20Projection%20Contract.md))
- La gestion des erreurs de traduction (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md))

---

## 3. Principe fondamental

> **La traduction préserve la sémantique tout en adaptant le format.**

La traduction est une transformation pure : elle ne prend aucune décision, ne modifie pas le sens, et n'a aucun effet de bord. Elle est bidirectionnelle et réversible (en théorie).

---

## 4. Directions de traduction

### 4.1 Traduction ascendante (Intention → Demande)

| Aspect | Description |
|--------|-------------|
| **Direction** | Produit → Autorité |
| **Entrée** | Intention (vocabulaire produit) |
| **Sortie** | Demande (vocabulaire autorité) |
| **Objectif** | Adapter l'intention au format de l'autorité |

### 4.2 Traduction descendante (Réponse → Résultat)

| Aspect | Description |
|--------|-------------|
| **Direction** | Autorité → Produit |
| **Entrée** | Réponse (vocabulaire autorité) |
| **Sortie** | Résultat (vocabulaire produit) |
| **Objectif** | Adapter la réponse au format du produit |

---

## 5. Propriétés de la traduction

### 5.1 Fidélité sémantique

| Code | Règle | Description |
|------|-------|-------------|
| **FID-01** | Préservation du sens | Le sens original est préservé |
| **FID-02** | Pas d'interprétation | Format transformé, sens intact |
| **FID-03** | Pas d'enrichissement métier | Pas d'ajout d'information métier |

**Exceptions autorisées :**
- Enrichissement technique (métadonnées de traçabilité)
- Normalisation de format (dates, nombres)
- Complétion de champs techniques obligatoires

### 5.2 Complétude

| Code | Règle | Description |
|------|-------|-------------|
| **COMP-01** | Pas de perte essentielle | Information essentielle préservée |
| **COMP-02** | Omission non essentielle | Informations non supportées omissibles |

**Information essentielle :**
- Type d'intention
- Identifiants (produit, utilisateur, ressources)
- Données du payload nécessaires à l'évaluation
- Contexte minimal requis

### 5.3 Pureté

| Code | Règle | Description |
|------|-------|-------------|
| **PUR-01** | Pas d'effet de bord | Pas de modification d'état, pas d'appel, pas de décision |
| **PUR-02** | Fonction pure | Même entrée → même sortie |
| **PUR-03** | Déterminisme | Pas de hasard, pas de dépendance à l'état global |

---

## 6. Traduction ascendante (Intention → Demande)

### 6.1 Structure de la demande

```typescript
interface Demande {
    // Identifiants
    demande_id: DemandeId;
    intention_id: IntentionId;           // Traçabilité
    
    // Type et contenu
    type: TypeDemande;                   // Vocabulaire autorité
    donnees: DonneesSpecifiques;         // Données traduites
    
    // Contexte
    contexte: Contexte;                  // Transmis intégralement
    
    // Métadonnées
    timestamp: Timestamp;
    autorite_cible: AutoriteId;
}
```

### 6.2 Règles de traduction ascendante

| Code | Règle | Description |
|------|-------|-------------|
| **ASC-01** | Mapping de type | Type intention → type demande |
| **ASC-02** | Traduction payload | Champ par champ selon mapping |
| **ASC-03** | Préservation contexte | Contexte intégral sans modification |
| **ASC-04** | Ajout métadonnées techniques | `intention_id`, `timestamp_demande` |
| **ASC-05** | Champs optionnels | Omissibles si non supportés |
| **ASC-06** | Validation format | Demande valide selon schéma autorité |

### 6.3 Exemples de mapping de type

| Type Intention (Produit) | Type Demande (Autorité) |
|--------------------------|-------------------------|
| `CREATE_CONTENT` | `create_content` |
| `AUTHORIZE` | `check_permission` |
| `READ_CONTENT` | `read_content` |

### 6.4 Exemple de traduction ascendante

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
  "contexte": { "..." },
  "timestamp": "2026-01-28T10:00:00Z",
  "version": "2.0.0"
}
```

**Demande traduite (KindMother) :**
```json
{
  "demande_id": "dem-789",
  "intention_id": "int-123",
  "type": "create_content",
  "donnees": {
    "title": "Mon article",
    "body": "Contenu...",
    "author_id": "user-456"
  },
  "contexte": { "..." },
  "timestamp": "2026-01-28T10:00:00Z",
  "autorite_cible": "kind_mother"
}
```

---

## 7. Traduction descendante (Réponse → Résultat)

### 7.1 Structure du résultat

```typescript
interface Resultat {
    // Identifiants
    resultat_id: ResultatId;
    intention_id: IntentionId;
    demande_id: DemandeId;
    
    // Statut
    statut: StatutResultat;              // SUCCÈS, REFUSÉ, ERREUR
    decision: DecisionAutorite;
    
    // Données (si applicable)
    donnees?: DonneesTraduites;
    
    // Erreurs (si applicable)
    erreurs?: ErreurTraduite[];
    
    // Métadonnées
    timestamp: Timestamp;
    autorite: AutoriteId;
}
```

### 7.2 Règles de traduction descendante

| Code | Règle | Description |
|------|-------|-------------|
| **DESC-01** | Préservation décision | Décision de l'autorité intacte |
| **DESC-02** | Traduction statut | `accepted` → `SUCCÈS`, `denied` → `REFUSÉ` |
| **DESC-03** | Traduction données | Champ par champ selon mapping |
| **DESC-04** | Traduction erreurs | Code technique préservé si nécessaire |
| **DESC-05** | Filtrage avant traduction | Filtrage appliqué AVANT traduction |
| **DESC-06** | Champs absents | Omis (pas de valeur par défaut) |

### 7.3 Exemple de traduction descendante

**Réponse (KindMother) :**
```json
{
  "response_id": "resp-456",
  "request_id": "dem-789",
  "status": "accepted",
  "data": {
    "content_id": "content-999",
    "title": "Mon article",
    "created_at": "2026-01-28T10:05:00Z"
  }
}
```

**Résultat traduit (produit) :**
```json
{
  "resultat_id": "res-111",
  "intention_id": "int-123",
  "demande_id": "dem-789",
  "statut": "SUCCÈS",
  "decision": "ACCEPTÉE",
  "donnees": {
    "id": "content-999",
    "titre": "Mon article",
    "cree_le": "2026-01-28T10:05:00Z"
  },
  "autorite": "kind_mother"
}
```

---

## 8. Mappings de vocabulaire

### 8.1 Structure d'un mapping

```typescript
interface MappingVocabulaire {
    intention_type: TypeIntention;
    autorite: AutoriteId;
    direction: Direction;               // ASCENDANTE | DESCENDANTE
    
    mappings_champs: Map<ChampProduit, ChampAutorite>;
    mappings_types: Map<TypeProduit, TypeAutorite>;
    mappings_valeurs?: Map<ValeurProduit, ValeurAutorite>;
    
    transformations?: Transformation[];
}
```

### 8.2 Types de transformations

| Type | Description | Exemple |
|------|-------------|---------|
| **Directe** | Champ → Champ | `titre` → `title` |
| **De type** | Conversion de type | String → Enum |
| **De structure** | Aplatissement/imbrication | Objet → Structure plate |
| **De valeur** | Valeur → Valeur | `"créer"` → `"create"` |

### 8.3 Règles de mapping

| Code | Règle | Description |
|------|-------|-------------|
| **MAP-01** | Unicité | Un champ → un mapping max |
| **MAP-02** | Complétude | Champs obligatoires mappés |
| **MAP-03** | Réversibilité théorique | Mapping inverse possible |
| **MAP-04** | Versioning | Mappings versionnés |

---

## 9. Garanties de traduction

| Garantie | Engagement | Mesure |
|----------|------------|--------|
| **Fidélité** | Sémantique préservée | Tests round-trip |
| **Complétude** | Information essentielle préservée | Vérification automatisée |
| **Déterminisme** | Même entrée → même sortie | Tests de régression |
| **Performance** | Temps constant | Métriques temps |

---

## 10. Cas d'échec de traduction

### 10.1 Types d'échec

| Type | Cause | Exemple |
|------|-------|---------|
| **Échec mapping** | Type/champ non mappé | Type inconnu |
| **Échec validation** | Demande invalide | Champ obligatoire manquant |
| **Échec format** | Format incompatible | Type de données non supporté |

### 10.2 Traitement des échecs

| Code | Règle | Description |
|------|-------|-------------|
| **ECHEC-01** | Rejet immédiat | Pas de transmission à l'autorité |
| **ECHEC-02** | Journalisation | Intention + type échec + raison |
| **ECHEC-03** | Notification produit | `ERREUR_TRADUCTION` + message |
| **ECHEC-04** | Pas de retry | Échec non transitoire |

---

## 11. Extensibilité

### 11.1 Ajout de nouveaux mappings

| Code | Règle | Description |
|------|-------|-------------|
| **EXT-01** | Déclaration explicite | Registre de mappings |
| **EXT-02** | Tests obligatoires | Tests round-trip |
| **EXT-03** | Versioning | Avec les schémas |

### 11.2 Modification de mappings existants

| Code | Règle | Description |
|------|-------|-------------|
| **EXT-04** | Rétrocompatibilité | Ou processus de dépréciation |
| **EXT-05** | Migration | Intentions en cours migrées |

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de traduction que Bonding Brother doit respecter pour garantir la fidélité et la complétude des transformations entre produits et autorités.

---

## Navigation

- [Index BondingBrother](../../_index.md)
- [Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md)
- [Filtering & Projection Contract](./BondingBrother%20-%20Filtering%20&%20Projection%20Contract.md)

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** Documentation Fondatrice v2.0, Intent Model Contract v2.0
