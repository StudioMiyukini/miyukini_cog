# BondingBrother â€” Translation Contract

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif

---

## 1. Contexte

Ce document dÃ©finit les rÃ¨gles contractuelles de traduction dans Bonding Brother. Il spÃ©cifie comment les intentions (exprimÃ©es dans le vocabulaire des produits) sont transformÃ©es en demandes (exprimÃ©es dans le vocabulaire des autoritÃ©s), et comment les rÃ©ponses sont transformÃ©es en rÃ©sultats.

**DÃ©pendances :**
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) â€” Section 5 (Nature du lien)
- [Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

La traduction fonctionne localement sans dÃ©pendance externe, conformÃ©ment Ã  **LOI-1** (aucune dÃ©pendance externe critique).

## 2. PortÃ©e / Scope

Ce document couvre :
- Les principes fondamentaux de la traduction
- Les rÃ¨gles de traduction intention â†’ demande
- Les rÃ¨gles de traduction rÃ©ponse â†’ rÃ©sultat
- Les mappings de vocabulaire
- Les garanties de fidÃ©litÃ© sÃ©mantique
- Les cas d'Ã©chec de traduction

Ce document **ne couvre pas** :
- Les rÃ¨gles de filtrage (voir [Filtering & Projection Contract](./BondingBrother%20-%20Filtering%20&%20Projection%20Contract.md))
- La gestion des erreurs de traduction (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md))

---

## 3. Principe fondamental

> **La traduction prÃ©serve la sÃ©mantique tout en adaptant le format.**

La traduction est une transformation pure : elle ne prend aucune dÃ©cision, ne modifie pas le sens, et n'a aucun effet de bord. Elle est bidirectionnelle et rÃ©versible (en thÃ©orie).

---

## 4. Directions de traduction

### 4.1 Traduction ascendante (Intention â†’ Demande)

| Aspect | Description |
|--------|-------------|
| **Direction** | Produit â†’ AutoritÃ© |
| **EntrÃ©e** | Intention (vocabulaire produit) |
| **Sortie** | Demande (vocabulaire autoritÃ©) |
| **Objectif** | Adapter l'intention au format de l'autoritÃ© |

### 4.2 Traduction descendante (RÃ©ponse â†’ RÃ©sultat)

| Aspect | Description |
|--------|-------------|
| **Direction** | AutoritÃ© â†’ Produit |
| **EntrÃ©e** | RÃ©ponse (vocabulaire autoritÃ©) |
| **Sortie** | RÃ©sultat (vocabulaire produit) |
| **Objectif** | Adapter la rÃ©ponse au format du produit |

---

## 5. PropriÃ©tÃ©s de la traduction

### 5.1 FidÃ©litÃ© sÃ©mantique

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **FID-01** | PrÃ©servation du sens | Le sens original est prÃ©servÃ© |
| **FID-02** | Pas d'interprÃ©tation | Format transformÃ©, sens intact |
| **FID-03** | Pas d'enrichissement mÃ©tier | Pas d'ajout d'information mÃ©tier |

**Exceptions autorisÃ©es :**
- Enrichissement technique (mÃ©tadonnÃ©es de traÃ§abilitÃ©)
- Normalisation de format (dates, nombres)
- ComplÃ©tion de champs techniques obligatoires

### 5.2 ComplÃ©tude

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **COMP-01** | Pas de perte essentielle | Information essentielle prÃ©servÃ©e |
| **COMP-02** | Omission non essentielle | Informations non supportÃ©es omissibles |

**Information essentielle :**
- Type d'intention
- Identifiants (produit, utilisateur, ressources)
- DonnÃ©es du payload nÃ©cessaires Ã  l'Ã©valuation
- Contexte minimal requis

### 5.3 PuretÃ©

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **PUR-01** | Pas d'effet de bord | Pas de modification d'Ã©tat, pas d'appel, pas de dÃ©cision |
| **PUR-02** | Fonction pure | MÃªme entrÃ©e â†’ mÃªme sortie |
| **PUR-03** | DÃ©terminisme | Pas de hasard, pas de dÃ©pendance Ã  l'Ã©tat global |

---

## 6. Traduction ascendante (Intention â†’ Demande)

### 6.1 Structure de la demande

```typescript
interface Demande {
    // Identifiants
    demande_id: DemandeId;
    intention_id: IntentionId;           // TraÃ§abilitÃ©
    
    // Type et contenu
    type: TypeDemande;                   // Vocabulaire autoritÃ©
    donnees: DonneesSpecifiques;         // DonnÃ©es traduites
    
    // Contexte
    contexte: Contexte;                  // Transmis intÃ©gralement
    
    // MÃ©tadonnÃ©es
    timestamp: Timestamp;
    autorite_cible: AutoriteId;
}
```

### 6.2 RÃ¨gles de traduction ascendante

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **ASC-01** | Mapping de type | Type intention â†’ type demande |
| **ASC-02** | Traduction payload | Champ par champ selon mapping |
| **ASC-03** | PrÃ©servation contexte | Contexte intÃ©gral sans modification |
| **ASC-04** | Ajout mÃ©tadonnÃ©es techniques | `intention_id`, `timestamp_demande` |
| **ASC-05** | Champs optionnels | Omissibles si non supportÃ©s |
| **ASC-06** | Validation format | Demande valide selon schÃ©ma autoritÃ© |

### 6.3 Exemples de mapping de type

| Type Intention (Produit) | Type Demande (AutoritÃ©) |
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

## 7. Traduction descendante (RÃ©ponse â†’ RÃ©sultat)

### 7.1 Structure du rÃ©sultat

```typescript
interface Resultat {
    // Identifiants
    resultat_id: ResultatId;
    intention_id: IntentionId;
    demande_id: DemandeId;
    
    // Statut
    statut: StatutResultat;              // SUCCÃˆS, REFUSÃ‰, ERREUR
    decision: DecisionAutorite;
    
    // DonnÃ©es (si applicable)
    donnees?: DonneesTraduites;
    
    // Erreurs (si applicable)
    erreurs?: ErreurTraduite[];
    
    // MÃ©tadonnÃ©es
    timestamp: Timestamp;
    autorite: AutoriteId;
}
```

### 7.2 RÃ¨gles de traduction descendante

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **DESC-01** | PrÃ©servation dÃ©cision | DÃ©cision de l'autoritÃ© intacte |
| **DESC-02** | Traduction statut | `accepted` â†’ `SUCCÃˆS`, `denied` â†’ `REFUSÃ‰` |
| **DESC-03** | Traduction donnÃ©es | Champ par champ selon mapping |
| **DESC-04** | Traduction erreurs | Code technique prÃ©servÃ© si nÃ©cessaire |
| **DESC-05** | Filtrage avant traduction | Filtrage appliquÃ© AVANT traduction |
| **DESC-06** | Champs absents | Omis (pas de valeur par dÃ©faut) |

### 7.3 Exemple de traduction descendante

**RÃ©ponse (KindMother) :**
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

**RÃ©sultat traduit (produit) :**
```json
{
  "resultat_id": "res-111",
  "intention_id": "int-123",
  "demande_id": "dem-789",
  "statut": "SUCCÃˆS",
  "decision": "ACCEPTÃ‰E",
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
| **Directe** | Champ â†’ Champ | `titre` â†’ `title` |
| **De type** | Conversion de type | String â†’ Enum |
| **De structure** | Aplatissement/imbrication | Objet â†’ Structure plate |
| **De valeur** | Valeur â†’ Valeur | `"crÃ©er"` â†’ `"create"` |

### 8.3 RÃ¨gles de mapping

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **MAP-01** | UnicitÃ© | Un champ â†’ un mapping max |
| **MAP-02** | ComplÃ©tude | Champs obligatoires mappÃ©s |
| **MAP-03** | RÃ©versibilitÃ© thÃ©orique | Mapping inverse possible |
| **MAP-04** | Versioning | Mappings versionnÃ©s |

---

## 9. Garanties de traduction

| Garantie | Engagement | Mesure |
|----------|------------|--------|
| **FidÃ©litÃ©** | SÃ©mantique prÃ©servÃ©e | Tests round-trip |
| **ComplÃ©tude** | Information essentielle prÃ©servÃ©e | VÃ©rification automatisÃ©e |
| **DÃ©terminisme** | MÃªme entrÃ©e â†’ mÃªme sortie | Tests de rÃ©gression |
| **Performance** | Temps constant | MÃ©triques temps |

---

## 10. Cas d'Ã©chec de traduction

### 10.1 Types d'Ã©chec

| Type | Cause | Exemple |
|------|-------|---------|
| **Ã‰chec mapping** | Type/champ non mappÃ© | Type inconnu |
| **Ã‰chec validation** | Demande invalide | Champ obligatoire manquant |
| **Ã‰chec format** | Format incompatible | Type de donnÃ©es non supportÃ© |

### 10.2 Traitement des Ã©checs

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **ECHEC-01** | Rejet immÃ©diat | Pas de transmission Ã  l'autoritÃ© |
| **ECHEC-02** | Journalisation | Intention + type Ã©chec + raison |
| **ECHEC-03** | Notification produit | `ERREUR_TRADUCTION` + message |
| **ECHEC-04** | Pas de retry | Ã‰chec non transitoire |

---

## 11. ExtensibilitÃ©

### 11.1 Ajout de nouveaux mappings

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **EXT-01** | DÃ©claration explicite | Registre de mappings |
| **EXT-02** | Tests obligatoires | Tests round-trip |
| **EXT-03** | Versioning | Avec les schÃ©mas |

### 11.2 Modification de mappings existants

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **EXT-04** | RÃ©trocompatibilitÃ© | Ou processus de dÃ©prÃ©ciation |
| **EXT-05** | Migration | Intentions en cours migrÃ©es |

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit les rÃ¨gles de traduction que Bonding Brother doit respecter pour garantir la fidÃ©litÃ© et la complÃ©tude des transformations entre produits et autoritÃ©s.

---

## Navigation

- [Index BondingBrother](../../_index.md)
- [Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md)
- [Filtering & Projection Contract](./BondingBrother%20-%20Filtering%20&%20Projection%20Contract.md)

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** Documentation Fondatrice v2.0, Intent Model Contract v2.0

