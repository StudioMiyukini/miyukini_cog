# BondingBrother — Filtering & Projection Contract

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif

---

## 1. Contexte

Ce document définit les règles contractuelles de filtrage et de projection dans Bonding Brother. Il spécifie comment les intentions et les demandes sont filtrées avant transmission aux autorités, et comment les réponses sont filtrées et projetées avant transmission aux produits.

**Dépendances :**
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) — Section 5 (Filtrage)
- [Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md)
- [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

Le filtrage fonctionne localement sans dépendance externe, conformément à **LOI-1** (aucune dépendance externe critique).

## 2. Portée / Scope

Ce document couvre :
- Les principes fondamentaux du filtrage
- Les règles de filtrage d'entrée (intention → demande)
- Les règles de filtrage de sortie (réponse → résultat)
- Les règles de projection (sélection de champs)
- Les garanties de sécurité et de protection
- Les cas d'échec de filtrage

Ce document **ne couvre pas** :
- Les règles de traduction (voir [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md))
- La gestion des erreurs de filtrage (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md))

---

## 3. Principe fondamental

> **Le filtrage protège les autorités et les produits en supprimant ou masquant les informations non autorisées ou non nécessaires.**

Le filtrage est systématique, non optionnel, et non contournable. Il s'applique à toutes les interactions, sans exception.

---

## 4. Types de filtrage

### 4.1 Filtrage d'entrée

| Aspect | Description |
|--------|-------------|
| **Direction** | Produit → Autorité |
| **Moment** | Après traduction, avant transmission |
| **Objectif** | Protéger l'autorité des demandes invalides |

**Ce qui est filtré :**
- Demandes structurellement invalides
- Demandes contenant des champs interdits
- Demandes dépassant les limites autorisées
- Demandes violant les règles de sécurité

### 4.2 Filtrage de sortie

| Aspect | Description |
|--------|-------------|
| **Direction** | Autorité → Produit |
| **Moment** | Après réception réponse, avant traduction |
| **Objectif** | Protéger le produit des informations non autorisées |

**Ce qui est filtré :**
- Informations d'autres produits
- Détails internes de l'autorité
- Données sensibles non autorisées
- Métadonnées techniques non nécessaires

### 4.3 Projection

| Aspect | Description |
|--------|-------------|
| **Direction** | Autorité → Produit |
| **Moment** | Pendant le filtrage de sortie |
| **Objectif** | Sélectionner uniquement les champs nécessaires |

---

## 5. Règles de filtrage d'entrée

### 5.1 Validation structurelle

| Code | Règle | Action si échec |
|------|-------|-----------------|
| **FILT-IN-01** | Format valide | Rejet immédiat |
| **FILT-IN-02** | Champs obligatoires | Rejet immédiat |
| **FILT-IN-03** | Types de données | Rejet immédiat |

### 5.2 Validation de sécurité

| Code | Règle | Action si échec |
|------|-------|-----------------|
| **FILT-IN-04** | Champs interdits | Suppression ou rejet |
| **FILT-IN-05** | Limites de taille | Rejet immédiat |
| **FILT-IN-06** | Validation contexte | Rejet immédiat |

### 5.3 Protection des autorités

| Code | Règle | Action si échec |
|------|-------|-----------------|
| **FILT-IN-07** | Demandes malveillantes | Rejet + journal sécurité |
| **FILT-IN-08** | Rate limiting | Rejet ou file d'attente |

---

## 6. Règles de filtrage de sortie

### 6.1 Filtrage par autorisation

| Code | Règle | Action |
|------|-------|--------|
| **FILT-OUT-01** | Périmètre produit | Suppression hors périmètre |
| **FILT-OUT-02** | Permissions requises | Masquage ou suppression |
| **FILT-OUT-03** | Isolation produits | Suppression systématique |

### 6.2 Filtrage de confidentialité

| Code | Règle | Action |
|------|-------|--------|
| **FILT-OUT-04** | Données sensibles | Suppression ou masquage (`***`) |
| **FILT-OUT-05** | Métadonnées internes | Suppression |
| **FILT-OUT-06** | Détails implémentation | Suppression |

### 6.3 Projection de champs

| Code | Règle | Action |
|------|-------|--------|
| **FILT-OUT-07** | Champs demandés | Sélection uniquement |
| **FILT-OUT-08** | Champs nécessaires | Inclusion automatique |
| **FILT-OUT-09** | Champs optionnels | Inclusion conditionnelle |

---

## 7. Règles de projection

### 7.1 Projection basée sur l'intention

| Code | Règle | Description |
|------|-------|-------------|
| **PROJ-01** | Champs explicites | Si spécifiés, seuls ceux-là |
| **PROJ-02** | Champs par défaut | Si non spécifiés, défaut autorité |
| **PROJ-03** | Champs calculés | Si demandés et autorisés |

### 7.2 Projection basée sur les permissions

| Niveau | Accès |
|--------|-------|
| **Lecture basique** | Champs publics uniquement |
| **Lecture étendue** | Publics + étendus autorisés |
| **Lecture complète** | Tous les champs autorisés |

**Masquage partiel :** Si champ partiellement autorisé, seule la partie autorisée est projetée (ex: `user@***.com`).

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

### 8.3 Règles d'ordre

| Code | Règle | Description |
|------|-------|-------------|
| **ORDRE-01** | Filtrage avant traduction (sortie) | Traduction du filtré, pas du brut |
| **ORDRE-02** | Traduction avant filtrage (entrée) | Filtrage de la demande traduite |

---

## 9. Garanties de filtrage

| Garantie | Engagement | Mesure |
|----------|------------|--------|
| **Sécurité** | Aucune info non autorisée transmise | Tests avec données sensibles |
| **Complétude minimale** | Info minimale toujours présente | Vérification champs obligatoires |
| **Non-régression** | Données autorisées préservées | Tests de régression |
| **Performance** | Temps linéaire | Métriques temps |

---

## 10. Cas d'échec de filtrage

### 10.1 Types d'échec

| Type | Cause |
|------|-------|
| **Validation (entrée)** | Demande invalide, champs manquants |
| **Sécurité (entrée)** | Demande malveillante, limites dépassées |
| **Projection (sortie)** | Aucun champ autorisé disponible |

### 10.2 Traitement des échecs

| Code | Règle | Description |
|------|-------|-------------|
| **ECHEC-FILT-01** | Rejet immédiat (entrée) | Pas de transmission |
| **ECHEC-FILT-02** | Journalisation | Source + type + raison + règles |
| **ECHEC-FILT-03** | Notification produit | `ERREUR_FILTRAGE` + message |
| **ECHEC-FILT-04** | Résultat partiel (sortie) | Champs autorisés + avertissement |
| **ECHEC-FILT-05** | Pas de retry | Échec non transitoire |

---

## 11. Configuration et règles

### 11.1 Source des règles

| Code | Règle | Description |
|------|-------|-------------|
| **CONFIG-01** | Définition par autorité | Les autorités définissent les règles |
| **CONFIG-02** | Application par BB | BB applique, ne modifie jamais |
| **CONFIG-03** | Mise à jour des règles | Règles en vigueur au moment du filtrage |

### 11.2 Règles par défaut

| Code | Règle | Description |
|------|-------|-------------|
| **CONFIG-04** | Règles minimales | Validation structurelle + anti-injection + isolation |
| **CONFIG-05** | Règles spécifiques | Complètent ou remplacent par défaut |

---

## 12. Exemples

### 12.1 Filtrage d'entrée (XSS détecté)

**Intention reçue :**
```json
{
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Mon article",
    "contenu": "<script>alert('xss')</script>"
  }
}
```

**Résultat :** Rejet immédiat, pattern XSS détecté, pas de transmission à KindMother.

### 12.2 Filtrage de sortie

**Réponse KindMother :**
```json
{
  "data": {
    "content_id": "content-999",
    "title": "Mon article",
    "internal_metadata": { "storage_location": "/data/999" },
    "other_product_data": { "product_b": "data-b" }
  }
}
```

**Après filtrage :**
```json
{
  "data": {
    "content_id": "content-999",
    "title": "Mon article"
  }
}
```

Supprimé : `internal_metadata` (détails internes), `other_product_data` (autre produit).

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

**Résultat :** Seuls `titre` et `auteur` retournés, `contenu` omis car non demandé.

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de filtrage et de projection que Bonding Brother doit respecter pour garantir la sécurité et la protection des autorités et des produits.

---

## Navigation

- [Index BondingBrother](../../_index.md)
- [Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md)
- [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md)

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** Documentation Fondatrice v2.0, Translation Contract v2.0
