# BondingBrother - Filtering & Projection Contract

## 1. Contexte

Ce document définit les règles contractuelles de filtrage et de projection dans Bonding Brother. Il spécifie comment les intentions et les demandes sont filtrées avant transmission aux autorités, et comment les réponses sont filtrées et projetées avant transmission aux produits.

Ce document complète la Section 3 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Architecture et Composants](./BondingBrother%20-%20Architecture%20et%20Composants.md) et le [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md) pour définir les règles précises de filtrage.

Le filtrage fonctionne localement sans dépendance externe, conformément à **LOI-1** (aucune dépendance externe critique) définie dans les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

## 2. Portée / Scope

Ce document couvre :
- Les principes fondamentaux du filtrage
- Les règles de filtrage d'entrée (intention → demande)
- Les règles de filtrage de sortie (réponse → résultat)
- Les règles de projection (sélection de champs)
- Les garanties de sécurité et de protection
- Les cas d'échec de filtrage

Ce document **ne couvre pas** :
- Les règles de traduction (voir Translation Contract)
- La gestion des erreurs de filtrage (voir Error & Rejection Model)
- Les détails d'implémentation du FilterEngine
- Les règles métier spécifiques (définies par les autorités)

---

## 3. Principe fondamental

**Le filtrage protège les autorités et les produits en supprimant ou masquant les informations non autorisées ou non nécessaires.**

Le filtrage est systématique, non optionnel, et non contournable. Il s'applique à toutes les interactions, sans exception.

---

## 4. Types de filtrage

### 4.1 Filtrage d'entrée

**Direction :** Produit → Autorité

**Moment :** Après traduction, avant transmission à l'autorité

**Objectif :** Protéger l'autorité des demandes invalides, mal formées, ou non autorisées

**Ce qui est filtré :**
- Demandes structurellement invalides
- Demandes contenant des champs interdits
- Demandes dépassant les limites autorisées
- Demandes violant les règles de sécurité

### 4.2 Filtrage de sortie

**Direction :** Autorité → Produit

**Moment :** Après réception de la réponse, avant traduction en résultat

**Objectif :** Protéger le produit des informations non autorisées, sensibles, ou inutiles

**Ce qui est filtré :**
- Informations d'autres produits
- Détails internes de l'autorité
- Données sensibles non autorisées
- Métadonnées techniques non nécessaires

### 4.3 Projection

**Direction :** Autorité → Produit

**Moment :** Pendant le filtrage de sortie

**Objectif :** Sélectionner uniquement les champs nécessaires et autorisés pour le produit

**Ce qui est projeté :**
- Champs explicitement demandés par l'intention
- Champs nécessaires à la compréhension du résultat
- Champs autorisés selon les permissions du produit

---

## 5. Règles de filtrage d'entrée

### 5.1 Validation structurelle

**Règle FILT-IN-01 : Format valide**

La demande doit être structurellement valide selon le schéma de l'autorité cible.

**Action en cas d'échec :** Rejet immédiat, pas de transmission à l'autorité

**Règle FILT-IN-02 : Champs obligatoires**

Tous les champs marqués comme obligatoires par l'autorité doivent être présents.

**Action en cas d'échec :** Rejet immédiat

**Règle FILT-IN-03 : Types de données**

Chaque champ doit respecter le type attendu par l'autorité.

**Action en cas d'échec :** Rejet immédiat

### 5.2 Validation de sécurité

**Règle FILT-IN-04 : Champs interdits**

Certains champs sont interdits dans les demandes vers les autorités :
- Champs de configuration système
- Champs de métadonnées internes
- Champs réservés à l'autorité

**Action en cas d'échec :** Suppression du champ (si non critique) ou rejet (si critique)

**Règle FILT-IN-05 : Limites de taille**

La taille de la demande ne doit pas dépasser les limites définies par l'autorité.

**Action en cas d'échec :** Rejet immédiat

**Règle FILT-IN-06 : Validation de contexte**

Le contexte doit être présent et valide (produit_id, environnement, etc.).

**Action en cas d'échec :** Rejet immédiat

### 5.3 Protection des autorités

**Règle FILT-IN-07 : Demandes malveillantes**

Les demandes contenant des patterns malveillants connus sont rejetées :
- Injection de code
- Overflow de buffer
- Patterns d'attaque connus

**Action en cas d'échec :** Rejet immédiat + journalisation sécurité

**Règle FILT-IN-08 : Rate limiting**

Les demandes dépassant le taux autorisé sont rejetées ou mises en file d'attente.

**Action en cas d'échec :** Rejet ou mise en file d'attente

---

## 6. Règles de filtrage de sortie

### 6.1 Filtrage par autorisation

**Règle FILT-OUT-01 : Périmètre du produit**

Seules les données appartenant au produit demandeur ou autorisées explicitement sont transmises.

**Action :** Suppression des données hors périmètre

**Règle FILT-OUT-02 : Permissions requises**

Les données nécessitent les permissions appropriées pour être transmises au produit.

**Action :** Masquage ou suppression des données non autorisées

**Règle FILT-OUT-03 : Isolation des produits**

Aucune donnée d'un autre produit ne doit être transmise, même par erreur.

**Action :** Suppression systématique

### 6.2 Filtrage de confidentialité

**Règle FILT-OUT-04 : Données sensibles**

Les données sensibles (mots de passe, tokens, clés) ne sont jamais transmises aux produits.

**Action :** Suppression ou masquage (remplacement par `***`)

**Règle FILT-OUT-05 : Métadonnées internes**

Les métadonnées internes de l'autorité ne sont pas transmises :
- IDs internes non pertinents
- Timestamps de traitement interne
- Informations de debug

**Action :** Suppression

**Règle FILT-OUT-06 : Détails d'implémentation**

Les détails d'implémentation de l'autorité ne sont pas exposés :
- Structure de stockage interne
- Algorithmes utilisés
- Configuration interne

**Action :** Suppression

### 6.3 Projection de champs

**Règle FILT-OUT-07 : Champs demandés**

Seuls les champs explicitement demandés dans l'intention sont projetés (si l'autorité le supporte).

**Action :** Sélection des champs demandés uniquement

**Règle FILT-OUT-08 : Champs nécessaires**

Les champs nécessaires à la compréhension du résultat sont toujours inclus, même s'ils ne sont pas explicitement demandés :
- Statut de l'opération
- Identifiants de ressources créées/modifiées
- Codes d'erreur

**Action :** Inclusion automatique

**Règle FILT-OUT-09 : Champs optionnels**

Les champs optionnels sont inclus uniquement s'ils sont demandés et autorisés.

**Action :** Inclusion conditionnelle

---

## 7. Règles de projection

### 7.1 Projection basée sur l'intention

**Règle PROJ-01 : Champs explicites**

Si l'intention spécifie des champs à retourner, seuls ces champs sont projetés (si supporté par l'autorité).

**Exemple :**
```json
{
  "type": "READ_CONTENT",
  "payload": {
    "content_id": "123",
    "fields": ["titre", "contenu", "auteur"]
  }
}
```
→ Seuls `titre`, `contenu`, et `auteur` sont retournés

**Règle PROJ-02 : Champs par défaut**

Si aucun champ n'est spécifié, les champs par défaut sont projetés (définis par l'autorité).

**Règle PROJ-03 : Champs calculés**

Les champs calculés ou dérivés peuvent être projetés s'ils sont demandés et autorisés.

### 7.2 Projection basée sur les permissions

**Règle PROJ-04 : Niveaux de permission**

Les champs sont projetés selon le niveau de permission du produit :
- **Lecture basique :** Champs publics uniquement
- **Lecture étendue :** Champs publics + champs étendus autorisés
- **Lecture complète :** Tous les champs autorisés

**Règle PROJ-05 : Masquage partiel**

Si un champ est partiellement autorisé, seule la partie autorisée est projetée.

**Exemple :** Email masqué : `user@***.com` au lieu de `user@example.com`

---

## 8. Ordre d'application

### 8.1 Flux de filtrage d'entrée

```
Intention
   │
   ▼
Traduction (Intention → Demande)
   │
   ▼
Filtrage d'entrée
   ├─ Validation structurelle
   ├─ Validation de sécurité
   └─ Protection des autorités
   │
   ▼
Demande filtrée → Autorité
```

### 8.2 Flux de filtrage de sortie

```
Réponse autorité
   │
   ▼
Filtrage de sortie
   ├─ Filtrage par autorisation
   ├─ Filtrage de confidentialité
   └─ Projection de champs
   │
   ▼
Réponse filtrée
   │
   ▼
Traduction (Réponse → Résultat)
   │
   ▼
Résultat → Produit
```

**Règle ORDRE-01 : Filtrage avant traduction (sortie)**

Le filtrage de sortie est appliqué **avant** la traduction descendante. La traduction traduit ce qui a été filtré, pas la réponse brute.

**Règle ORDRE-02 : Traduction avant filtrage (entrée)**

La traduction ascendante est appliquée **avant** le filtrage d'entrée. Le filtrage valide la demande traduite.

---

## 9. Garanties de filtrage

### 9.1 Garantie de sécurité

**Engagement :** Aucune information non autorisée n'est transmise aux produits, et aucune demande invalide n'atteint les autorités.

**Mesure :** Tests automatisés de filtrage avec données sensibles et demandes malveillantes.

### 9.2 Garantie de complétude minimale

**Engagement :** Les résultats filtrés contiennent toujours les informations minimales nécessaires à la compréhension du résultat.

**Mesure :** Vérification que les champs obligatoires sont toujours présents après filtrage.

### 9.3 Garantie de non-régression

**Engagement :** Le filtrage ne supprime jamais d'informations nécessaires et autorisées.

**Mesure :** Tests de régression avec vérification que les données autorisées sont préservées.

### 9.4 Garantie de performance

**Engagement :** Le filtrage est effectué en temps linéaire par rapport à la taille des données.

**Mesure :** Métriques de temps de filtrage.

---

## 10. Cas d'échec de filtrage

### 10.1 Types d'échec

**Échec de validation (entrée) :**
- Demande structurellement invalide
- Champs obligatoires manquants
- Types de données incorrects

**Échec de sécurité (entrée) :**
- Demande malveillante détectée
- Limites dépassées
- Rate limiting

**Échec de projection (sortie) :**
- Aucun champ autorisé disponible
- Permissions insuffisantes pour tous les champs demandés

### 10.2 Traitement des échecs

**Règle ECHEC-FILT-01 : Rejet immédiat (entrée)**

En cas d'échec de filtrage d'entrée, la demande est rejetée immédiatement, sans transmission à l'autorité.

**Règle ECHEC-FILT-02 : Journalisation**

Tout échec de filtrage est journalisé avec :
- La demande ou réponse source
- Le type d'échec
- La raison détaillée
- Les règles violées

**Règle ECHEC-FILT-03 : Notification au produit**

Le produit reçoit un résultat avec statut `ERREUR_FILTRAGE` et un message d'erreur explicite (sans révéler les détails de sécurité).

**Règle ECHEC-FILT-04 : Résultat partiel (sortie)**

En cas d'échec partiel de projection (certains champs non autorisés), le résultat contient uniquement les champs autorisés, avec un avertissement si nécessaire.

**Règle ECHEC-FILT-05 : Pas de retry automatique**

Les échecs de filtrage ne sont pas retentés automatiquement (ce n'est pas une erreur transitoire).

---

## 11. Configuration et règles

### 11.1 Source des règles

**Règle CONFIG-01 : Définition par autorité**

Les règles de filtrage sont définies par les autorités, pas par Bonding Brother.

**Règle CONFIG-02 : Application par Bonding Brother**

Bonding Brother applique les règles définies par les autorités, mais ne les modifie jamais.

**Règle CONFIG-03 : Mise à jour des règles**

Les règles de filtrage peuvent être mises à jour par les autorités. Bonding Brother applique les règles en vigueur au moment du filtrage.

### 11.2 Règles par défaut

**Règle CONFIG-04 : Règles minimales**

Même sans règles explicites, Bonding Brother applique des règles minimales de sécurité :
- Validation structurelle de base
- Protection contre les injections
- Isolation des produits

**Règle CONFIG-05 : Règles spécifiques**

Les autorités peuvent définir des règles spécifiques qui remplacent ou complètent les règles par défaut.

---

## 12. Exemples

### 12.1 Filtrage d'entrée

**Intention reçue :**
```json
{
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Mon article",
    "contenu": "<script>alert('xss')</script>",
    "auteur": "user-123"
  }
}
```

**Demande après traduction :**
```json
{
  "type": "create_content",
  "données": {
    "title": "Mon article",
    "body": "<script>alert('xss')</script>",
    "author_id": "user-123"
  }
}
```

**Filtrage d'entrée :** Détection de pattern XSS → Rejet immédiat

**Résultat :** Intention rejetée, pas de transmission à Kind Mother

### 12.2 Filtrage de sortie

**Réponse de Kind Mother :**
```json
{
  "status": "accepted",
  "data": {
    "content_id": "content-999",
    "title": "Mon article",
    "body": "Contenu...",
    "author_id": "user-123",
    "internal_metadata": {
      "storage_location": "/data/content/999",
      "index_key": "idx-12345"
    },
    "other_product_data": {
      "product_b": "data-b"
    }
  }
}
```

**Filtrage de sortie :**
- Suppression de `internal_metadata` (détails internes)
- Suppression de `other_product_data` (données d'un autre produit)
- Conservation de `content_id`, `title`, `body`, `author_id` (données autorisées)

**Réponse filtrée :**
```json
{
  "status": "accepted",
  "data": {
    "content_id": "content-999",
    "title": "Mon article",
    "body": "Contenu...",
    "author_id": "user-123"
  }
}
```

### 12.3 Projection

**Intention avec projection :**
```json
{
  "type": "READ_CONTENT",
  "payload": {
    "content_id": "content-999",
    "fields": ["titre", "auteur"]
  }
}
```

**Réponse filtrée et projetée :**
```json
{
  "statut": "SUCCÈS",
  "données": {
    "titre": "Mon article",
    "auteur": "user-123"
  }
}
```

(Le champ `contenu` n'est pas retourné car non demandé)

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de filtrage et de projection que Bonding Brother doit respecter pour garantir la sécurité et la protection des autorités et des produits.

Tout filtrage effectué par Bonding Brother doit respecter ce contrat. Toute violation entraîne un rejet avec code d'erreur approprié.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 3)
- Architecture et Composants v1.0
- Translation Contract v1.0
- Glossaire et Terminologie v1.0
