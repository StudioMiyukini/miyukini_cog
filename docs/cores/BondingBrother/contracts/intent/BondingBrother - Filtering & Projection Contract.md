# BondingBrother â€” Filtering & Projection Contract

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif

---

## 1. Contexte

Ce document dÃ©finit les rÃ¨gles contractuelles de filtrage et de projection dans Bonding Brother. Il spÃ©cifie comment les intentions et les demandes sont filtrÃ©es avant transmission aux autoritÃ©s, et comment les rÃ©ponses sont filtrÃ©es et projetÃ©es avant transmission aux produits.

**DÃ©pendances :**
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) â€” Section 5 (Filtrage)
- [Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md)
- [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

Le filtrage fonctionne localement sans dÃ©pendance externe, conformÃ©ment Ã  **LOI-1** (aucune dÃ©pendance externe critique).

## 2. PortÃ©e / Scope

Ce document couvre :
- Les principes fondamentaux du filtrage
- Les rÃ¨gles de filtrage d'entrÃ©e (intention â†’ demande)
- Les rÃ¨gles de filtrage de sortie (rÃ©ponse â†’ rÃ©sultat)
- Les rÃ¨gles de projection (sÃ©lection de champs)
- Les garanties de sÃ©curitÃ© et de protection
- Les cas d'Ã©chec de filtrage

Ce document **ne couvre pas** :
- Les rÃ¨gles de traduction (voir [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md))
- La gestion des erreurs de filtrage (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md))

---

## 3. Principe fondamental

> **Le filtrage protÃ¨ge les autoritÃ©s et les produits en supprimant ou masquant les informations non autorisÃ©es ou non nÃ©cessaires.**

Le filtrage est systÃ©matique, non optionnel, et non contournable. Il s'applique Ã  toutes les interactions, sans exception.

---

## 4. Types de filtrage

### 4.1 Filtrage d'entrÃ©e

| Aspect | Description |
|--------|-------------|
| **Direction** | Produit â†’ AutoritÃ© |
| **Moment** | AprÃ¨s traduction, avant transmission |
| **Objectif** | ProtÃ©ger l'autoritÃ© des demandes invalides |

**Ce qui est filtrÃ© :**
- Demandes structurellement invalides
- Demandes contenant des champs interdits
- Demandes dÃ©passant les limites autorisÃ©es
- Demandes violant les rÃ¨gles de sÃ©curitÃ©

### 4.2 Filtrage de sortie

| Aspect | Description |
|--------|-------------|
| **Direction** | AutoritÃ© â†’ Produit |
| **Moment** | AprÃ¨s rÃ©ception rÃ©ponse, avant traduction |
| **Objectif** | ProtÃ©ger le produit des informations non autorisÃ©es |

**Ce qui est filtrÃ© :**
- Informations d'autres produits
- DÃ©tails internes de l'autoritÃ©
- DonnÃ©es sensibles non autorisÃ©es
- MÃ©tadonnÃ©es techniques non nÃ©cessaires

### 4.3 Projection

| Aspect | Description |
|--------|-------------|
| **Direction** | AutoritÃ© â†’ Produit |
| **Moment** | Pendant le filtrage de sortie |
| **Objectif** | SÃ©lectionner uniquement les champs nÃ©cessaires |

---

## 5. RÃ¨gles de filtrage d'entrÃ©e

### 5.1 Validation structurelle

| Code | RÃ¨gle | Action si Ã©chec |
|------|-------|-----------------|
| **FILT-IN-01** | Format valide | Rejet immÃ©diat |
| **FILT-IN-02** | Champs obligatoires | Rejet immÃ©diat |
| **FILT-IN-03** | Types de donnÃ©es | Rejet immÃ©diat |

### 5.2 Validation de sÃ©curitÃ©

| Code | RÃ¨gle | Action si Ã©chec |
|------|-------|-----------------|
| **FILT-IN-04** | Champs interdits | Suppression ou rejet |
| **FILT-IN-05** | Limites de taille | Rejet immÃ©diat |
| **FILT-IN-06** | Validation contexte | Rejet immÃ©diat |

### 5.3 Protection des autoritÃ©s

| Code | RÃ¨gle | Action si Ã©chec |
|------|-------|-----------------|
| **FILT-IN-07** | Demandes malveillantes | Rejet + journal sÃ©curitÃ© |
| **FILT-IN-08** | Rate limiting | Rejet ou file d'attente |

---

## 6. RÃ¨gles de filtrage de sortie

### 6.1 Filtrage par autorisation

| Code | RÃ¨gle | Action |
|------|-------|--------|
| **FILT-OUT-01** | PÃ©rimÃ¨tre produit | Suppression hors pÃ©rimÃ¨tre |
| **FILT-OUT-02** | Permissions requises | Masquage ou suppression |
| **FILT-OUT-03** | Isolation produits | Suppression systÃ©matique |

### 6.2 Filtrage de confidentialitÃ©

| Code | RÃ¨gle | Action |
|------|-------|--------|
| **FILT-OUT-04** | DonnÃ©es sensibles | Suppression ou masquage (`***`) |
| **FILT-OUT-05** | MÃ©tadonnÃ©es internes | Suppression |
| **FILT-OUT-06** | DÃ©tails implÃ©mentation | Suppression |

### 6.3 Projection de champs

| Code | RÃ¨gle | Action |
|------|-------|--------|
| **FILT-OUT-07** | Champs demandÃ©s | SÃ©lection uniquement |
| **FILT-OUT-08** | Champs nÃ©cessaires | Inclusion automatique |
| **FILT-OUT-09** | Champs optionnels | Inclusion conditionnelle |

---

## 7. RÃ¨gles de projection

### 7.1 Projection basÃ©e sur l'intention

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **PROJ-01** | Champs explicites | Si spÃ©cifiÃ©s, seuls ceux-lÃ  |
| **PROJ-02** | Champs par dÃ©faut | Si non spÃ©cifiÃ©s, dÃ©faut autoritÃ© |
| **PROJ-03** | Champs calculÃ©s | Si demandÃ©s et autorisÃ©s |

### 7.2 Projection basÃ©e sur les permissions

| Niveau | AccÃ¨s |
|--------|-------|
| **Lecture basique** | Champs publics uniquement |
| **Lecture Ã©tendue** | Publics + Ã©tendus autorisÃ©s |
| **Lecture complÃ¨te** | Tous les champs autorisÃ©s |

**Masquage partiel :** Si champ partiellement autorisÃ©, seule la partie autorisÃ©e est projetÃ©e (ex: `user@***.com`).

---

## 8. Ordre d'application

### 8.1 Flux de filtrage d'entrÃ©e

```
Intention
   â”‚
   â–¼
Traduction (Intention â†’ Demande)
   â”‚
   â–¼
Filtrage d'entrÃ©e
   â”œâ”€ Validation structurelle
   â”œâ”€ Validation de sÃ©curitÃ©
   â””â”€ Protection des autoritÃ©s
   â”‚
   â–¼
Demande filtrÃ©e â†’ AutoritÃ©
```

### 8.2 Flux de filtrage de sortie

```
RÃ©ponse autoritÃ©
   â”‚
   â–¼
Filtrage de sortie
   â”œâ”€ Filtrage par autorisation
   â”œâ”€ Filtrage de confidentialitÃ©
   â””â”€ Projection de champs
   â”‚
   â–¼
RÃ©ponse filtrÃ©e
   â”‚
   â–¼
Traduction (RÃ©ponse â†’ RÃ©sultat)
   â”‚
   â–¼
RÃ©sultat â†’ Produit
```

### 8.3 RÃ¨gles d'ordre

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **ORDRE-01** | Filtrage avant traduction (sortie) | Traduction du filtrÃ©, pas du brut |
| **ORDRE-02** | Traduction avant filtrage (entrÃ©e) | Filtrage de la demande traduite |

---

## 9. Garanties de filtrage

| Garantie | Engagement | Mesure |
|----------|------------|--------|
| **SÃ©curitÃ©** | Aucune info non autorisÃ©e transmise | Tests avec donnÃ©es sensibles |
| **ComplÃ©tude minimale** | Info minimale toujours prÃ©sente | VÃ©rification champs obligatoires |
| **Non-rÃ©gression** | DonnÃ©es autorisÃ©es prÃ©servÃ©es | Tests de rÃ©gression |
| **Performance** | Temps linÃ©aire | MÃ©triques temps |

---

## 10. Cas d'Ã©chec de filtrage

### 10.1 Types d'Ã©chec

| Type | Cause |
|------|-------|
| **Validation (entrÃ©e)** | Demande invalide, champs manquants |
| **SÃ©curitÃ© (entrÃ©e)** | Demande malveillante, limites dÃ©passÃ©es |
| **Projection (sortie)** | Aucun champ autorisÃ© disponible |

### 10.2 Traitement des Ã©checs

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **ECHEC-FILT-01** | Rejet immÃ©diat (entrÃ©e) | Pas de transmission |
| **ECHEC-FILT-02** | Journalisation | Source + type + raison + rÃ¨gles |
| **ECHEC-FILT-03** | Notification produit | `ERREUR_FILTRAGE` + message |
| **ECHEC-FILT-04** | RÃ©sultat partiel (sortie) | Champs autorisÃ©s + avertissement |
| **ECHEC-FILT-05** | Pas de retry | Ã‰chec non transitoire |

---

## 11. Configuration et rÃ¨gles

### 11.1 Source des rÃ¨gles

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **CONFIG-01** | DÃ©finition par autoritÃ© | Les autoritÃ©s dÃ©finissent les rÃ¨gles |
| **CONFIG-02** | Application par BB | BB applique, ne modifie jamais |
| **CONFIG-03** | Mise Ã  jour des rÃ¨gles | RÃ¨gles en vigueur au moment du filtrage |

### 11.2 RÃ¨gles par dÃ©faut

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **CONFIG-04** | RÃ¨gles minimales | Validation structurelle + anti-injection + isolation |
| **CONFIG-05** | RÃ¨gles spÃ©cifiques | ComplÃ¨tent ou remplacent par dÃ©faut |

---

## 12. Exemples

### 12.1 Filtrage d'entrÃ©e (XSS dÃ©tectÃ©)

**Intention reÃ§ue :**
```json
{
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Mon article",
    "contenu": "<script>alert('xss')</script>"
  }
}
```

**RÃ©sultat :** Rejet immÃ©diat, pattern XSS dÃ©tectÃ©, pas de transmission Ã  KindMother.

### 12.2 Filtrage de sortie

**RÃ©ponse KindMother :**
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

**AprÃ¨s filtrage :**
```json
{
  "data": {
    "content_id": "content-999",
    "title": "Mon article"
  }
}
```

SupprimÃ© : `internal_metadata` (dÃ©tails internes), `other_product_data` (autre produit).

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

**RÃ©sultat :** Seuls `titre` et `auteur` retournÃ©s, `contenu` omis car non demandÃ©.

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit les rÃ¨gles de filtrage et de projection que Bonding Brother doit respecter pour garantir la sÃ©curitÃ© et la protection des autoritÃ©s et des produits.

---

## Navigation

- [Index BondingBrother](../../_index.md)
- [Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md)
- [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md)

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** Documentation Fondatrice v2.0, Translation Contract v2.0

