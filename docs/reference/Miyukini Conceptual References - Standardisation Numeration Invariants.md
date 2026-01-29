# Miyukini Conceptual References - Standardisation Numérotation Invariants

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Standard de numérotation des invariants  
**Portée :** Tous les cores du système Miyukini

---

## 1. Contexte

Ce document définit le **standard de numérotation des invariants** pour garantir la cohérence et la traçabilité dans l'ensemble de l'écosystème Miyukini Core System.

**Problématique identifiée :** La numérotation des invariants n'est pas standardisée entre les index (`_index.md`) et les contrats détaillés, créant de la confusion et des risques de dérive.

**Objectif :** Établir un système de numérotation unique, cohérent et traçable pour tous les invariants.

---

## 2. Portée / Scope

Ce document définit :
- Le format standard de numérotation des invariants
- Les règles de mapping entre index et contrats détaillés
- Les conventions de nommage par catégorie d'invariant
- Les règles de migration pour les invariants existants

Ce document **ne couvre pas** :
- Le contenu des invariants (voir contrats spécifiques)
- La validation des invariants (voir contrats de gouvernance)
- L'évolution des invariants (voir Versioning & Evolution Contracts)

---

## 3. Standard de numérotation

### 3.1 Format canonique

**Format :** `INV-<PREFIX>-<NUMERO>`

| Composant | Description | Exemples |
|-----------|-------------|----------|
| **INV** | Préfixe fixe "Invariant" | INV |
| **PREFIX** | Code du core (2-3 lettres) | K (Kernel), SF (StrongFather), BB (BondingBrother) |
| **NUMERO** | Numéro séquentiel (1, 2, 3...) | 1, 2, 3, ... |

**Exemples :**
- `INV-K-1` : Invariant 1 du Kernel
- `INV-SF-1` : Invariant 1 de StrongFather
- `INV-BB-1` : Invariant 1 de BondingBrother

### 3.2 Codes de préfixe par core

| Core | Code | Exemple |
|------|------|---------|
| **Kernel** | K | INV-K-1 |
| **StrongFather** | SF | INV-SF-1 |
| **KindMother** | KM | INV-KM-1 |
| **BondingBrother** | BB | INV-BB-1 |
| **CaringNanny** | CN | INV-CN-1 |
| **BorderGuard** | BG | INV-BG-1 |
| **WorrySentinel** | WS | INV-WS-1 |
| **MasterButler** | MB | INV-MB-1 |
| **EverBuddy** | EB | INV-EB-1 |
| **TAMR** | TM | INV-TM-1 |
| **LogisticsSteward** | LS | INV-LS-1 |
| **MiyukiniAdmin** | MA | INV-MA-1 |

---

## 4. Règles de numérotation

### 4.1 Numérotation séquentielle

**Règle :** Les invariants sont numérotés de manière séquentielle à partir de 1, sans gap.

**Exemple :**
- ✅ `INV-K-1`, `INV-K-2`, `INV-K-3`, ..., `INV-K-10`
- ❌ `INV-K-1`, `INV-K-3`, `INV-K-5` (gaps interdits)

### 4.2 Cohérence index ↔ contrats

**Règle :** Les numéros dans `_index.md` et dans les contrats détaillés doivent être **identiques**.

**Format dans `_index.md` :**
```markdown
| Invariant | Description |
|-----------|-------------|
| **INV-K-1** | Aucune logique métier |
| **INV-K-2** | Aucune dépendance externe critique |
```

**Format dans contrats détaillés :**
```markdown
### 4.1 INV-K-1 : Aucune logique métier
```

### 4.3 Catégorisation (optionnelle)

**Règle :** Les invariants peuvent être organisés par catégorie, mais la numérotation reste séquentielle.

**Exemple (Kernel) :**
- **Identité** : INV-K-1, INV-K-2, INV-K-3, INV-K-4
- **Observabilité** : INV-K-5, INV-K-6, INV-K-7, INV-K-8
- **Autonomie** : INV-K-9, INV-K-10

**Note :** La catégorisation est informative, pas normative. La numérotation reste `INV-K-1` à `INV-K-10`.

---

## 5. Mapping avec numérotation interne

### 5.1 Problème identifié

Certains cores utilisent une numérotation interne différente dans leurs contrats détaillés :

| Core | Index | Contrats détaillés | Problème |
|------|-------|--------------------|----------|
| **StrongFather** | INV-SF-1 à INV-SF-8 | INV-AUTH-1, INV-BEHAV-2, INV-DEC-1 | Numérotation par catégorie |
| **BondingBrother** | INV-BB-1 à INV-BB-7 | INV-NAT-01, INV-NEG-01 | Format différent |

### 5.2 Solution : Mapping explicite

**Règle :** Créer un **mapping explicite** dans chaque contrat détaillé.

**Format recommandé :**

```markdown
## Mapping des invariants

| Numéro canonique | Numéro interne | Catégorie | Description |
|------------------|----------------|-----------|-------------|
| **INV-SF-1** | INV-AUTH-1 | Authentification | ... |
| **INV-SF-2** | INV-BEHAV-2 | Comportement | ... |
| **INV-SF-3** | INV-DEC-1 | Décision | ... |
```

**Utilisation :**
- Le **numéro canonique** (`INV-SF-1`) est utilisé dans `_index.md` et références croisées
- Le **numéro interne** (`INV-AUTH-1`) peut être utilisé dans le contrat détaillé pour organisation
- Le mapping garantit la traçabilité

### 5.3 Migration progressive

**Stratégie :** Migration progressive vers la numérotation canonique.

**Phase 1 :** Ajouter le mapping dans les contrats existants  
**Phase 2 :** Utiliser progressivement la numérotation canonique  
**Phase 3 :** Déprécier la numérotation interne (optionnel)

---

## 6. Règles de référencement

### 6.1 Références croisées

**Format :** Toujours utiliser le numéro canonique.

**Exemples :**
- ✅ "Voir INV-K-1 (Aucune logique métier)"
- ❌ "Voir INV-AUTH-1" (sans contexte)

### 6.2 Documentation

**Règle :** Le numéro canonique doit apparaître :
- Dans `_index.md` (table des invariants)
- Dans les contrats détaillés (en-tête de section)
- Dans les références croisées
- Dans les logs et traces (optionnel)

---

## 7. Exceptions et cas spéciaux

### 7.1 Invariants dérivés

**Règle :** Les invariants dérivés d'autres invariants conservent leur propre numéro.

**Exemple :**
- `INV-K-5` (Non-mutation) dérive de `INV-MOC-1` (Kernel Maintenance Observability)
- `INV-K-5` garde son numéro, référence `INV-MOC-1` dans la documentation

### 7.2 Invariants transversaux

**Règle :** Les invariants transversaux (LOI-1 à LOI-8) ne suivent pas ce format.

**Format :** `LOI-<NUMERO>` (Lois d'Autonomie Système)

---

## 8. Vérification et conformité

### 8.1 Checklist de conformité

Pour chaque core, vérifier :

- [ ] Les invariants dans `_index.md` utilisent le format `INV-<PREFIX>-<NUMERO>`
- [ ] Les invariants dans les contrats détaillés utilisent le même format ou un mapping explicite
- [ ] La numérotation est séquentielle sans gap
- [ ] Les références croisées utilisent le numéro canonique
- [ ] Le code de préfixe correspond au core

### 8.2 Outils de vérification

**Script de vérification (à créer) :**
```bash
# Vérifier la cohérence des invariants
./scripts/verify-invariants.sh

# Vérifier le format de numérotation
./scripts/verify-invariant-format.sh
```

---

## 9. Migration des cores existants

### 9.1 StrongFather

**État actuel :**
- Index : INV-SF-1 à INV-SF-8
- Contrats : INV-AUTH-1, INV-BEHAV-2, INV-DEC-1, etc.

**Action :**
1. Créer un mapping dans `StrongFather - Invariants & Guarantees.md`
2. Utiliser INV-SF-1 à INV-SF-8 dans les références croisées
3. Conserver INV-AUTH-1, etc. dans le contrat détaillé (organisation interne)

### 9.2 BondingBrother

**État actuel :**
- Index : INV-BB-1 à INV-BB-7
- Contrats : INV-NAT-01, INV-NEG-01, etc.

**Action :**
1. Créer un mapping dans `BondingBrother - Invariants & Guarantees.md`
2. Utiliser INV-BB-1 à INV-BB-7 dans les références croisées
3. Migrer progressivement vers le format canonique

### 9.3 Autres cores

**Règle :** Vérifier et aligner selon le même processus.

---

## 10. Références

- [Kernel - Invariants & Guarantees](../kernel/contracts/Kernel%20-%20Invariants%20%26%20Guarantees.md)
- [StrongFather - Invariants & Guarantees](../core/StrongFather/contracts/governance/StrongFather%20-%20Invariants%20%26%20Guarantees.md)
- [BondingBrother - Invariants & Guarantees](../core/BondingBrother/contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Standard de numérotation des invariants  
**Action requise :** Appliquer ce standard lors de la création ou mise à jour des invariants
