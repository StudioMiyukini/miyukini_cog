# Miyukini Core System — Phase 2.3 : Permissions & Access Control

## 1. Objectif du document

Ce document définit la **stratégie d'architecture** pour les permissions et contrôles d'accès dans le système Miyukini Core System. Il analyse où cette logique doit vivre, compare les options possibles, et propose une stratégie recommandée avec une alternative acceptable.

**Rôle :** Décision architecturale de long terme (5-10 ans). Ce document guide les décisions futures sans imposer d'implémentation technique.

**Ce qu'il couvre :**
- Analyse des options architecturales
- Comparaison kernel / SPM / produit / adaptateurs
- Stratégie recommandée
- Alternative acceptable
- Ce qui ne sera jamais implémenté

**Ce qu'il ne couvre pas :**
- Implémentation technique (auth, RBAC, ACL)
- Framework de permissions
- Détails de sécurité (tokens, sessions)
- Protocoles d'authentification

---

## 2. Contexte et contraintes

### Architecture actuelle

**Kernel (infra) :**
- 5 modules : `config`, `id`, `time`, `log`, `lifecycle`
- Responsabilité : fondation technique réutilisable
- Principe : aucune logique métier, aucune dépendance produit

**SPM CMS :**
- Modules Phase 0 : Content, Hierarchy, Taxonomies
- Modules Phase 1 : Media, Publication
- Modules Phase 2 : Search
- Responsabilité : capacités fonctionnelles génériques CMS
- Principe : pas de logique métier, pas de permissions (hors-scope explicite Phase 0)

**Produit :**
- Logique métier spécifique
- Règles business
- UI, rendu, intégrations

### Contraintes absolues

1. **Kernel :** Ne doit jamais contenir de logique métier ou produit
2. **SPM CMS :** Ne doit jamais contenir de logique métier spécifique
3. **Dépendances :** Unidirectionnelles (kernel → SPM → produit)
4. **Réutilisabilité :** Kernel et SPM doivent être réutilisables par plusieurs produits

---

## 3. Analyse : où les permissions DOIVENT vivre

### Option 1 : Dans le Kernel

**Ce que cela signifierait :**
- Module `permissions` dans le kernel
- Trait `PermissionChecker` ou `AccessControl`
- Types génériques (Role, Permission, Policy)

**Pourquoi c'est REFUSÉ :**

1. **Violation du principe kernel :** Le kernel est infra pure, sans logique métier. Les permissions sont de la logique métier (qui peut faire quoi selon les règles business).

2. **Couplage produit :** Les règles de permissions varient drastiquement selon le produit :
   - CMS : éditeur peut modifier, visiteur peut lire
   - SaaS B2B : admin organisation peut gérer équipe, membre peut voir projet
   - Jeu : joueur peut modifier son profil, admin peut bannir
   - E-shop : client peut commander, vendeur peut gérer catalogue

3. **Dépendances inversées :** Le kernel devrait connaître les entités du produit (Content, Media, etc.) pour vérifier les permissions, créant une dépendance circulaire.

4. **Bloat :** Ajouter des permissions au kernel le transforme en framework applicatif, ce qui est explicitement refusé.

**Verdict : ❌ REFUSÉ**

---

### Option 2 : Dans le SPM CMS

**Ce que cela signifierait :**
- Module `permissions` dans SPM CMS
- Trait `CmsPermissionChecker`
- Intégration avec Content, Media, Publication

**Pourquoi c'est REFUSÉ :**

1. **Violation du contrat Phase 0 :** La Phase 0 exclut explicitement les permissions du SPM CMS. C'est un invariant gelé.

2. **Logique métier spécifique :** Les règles de permissions sont spécifiques au produit :
   - Un CMS peut avoir "éditeur peut modifier tous les articles"
   - Un autre CMS peut avoir "auteur peut modifier seulement ses articles"
   - Un autre peut avoir "modérateur peut publier, auteur ne peut que proposer"

3. **Couplage fort :** Les modules SPM devraient exposer des hooks de permissions, créant un couplage fort entre modules et permissions.

4. **Bloat SPM :** Transformer le SPM en framework de permissions le rend moins réutilisable et plus complexe.

5. **Anticipation :** Aucun besoin partagé par ≥2 produits CMS identifié. Les besoins varient trop.

**Verdict : ❌ REFUSÉ**

---

### Option 3 : Dans le Produit

**Ce que cela signifierait :**
- Module `permissions` dans le produit
- Implémentation des règles business spécifiques
- Intégration avec les modules SPM via adaptateurs

**Pourquoi c'est ACCEPTABLE :**

1. **Logique métier :** Les permissions sont de la logique métier spécifique au produit.

2. **Flexibilité :** Le produit peut implémenter exactement les règles dont il a besoin.

3. **Pas de couplage :** Le produit orchestre les vérifications de permissions avant d'appeler les modules SPM.

4. **Réutilisabilité préservée :** Les modules SPM restent génériques et réutilisables.

**Limites :**
- Pas de mutualisation entre produits
- Chaque produit doit réimplémenter
- Pas de contrat standardisé

**Verdict : ✅ ACCEPTABLE (alternative)**

---

### Option 4 : Dans les Adaptateurs

**Ce que cela signifierait :**
- Les adaptateurs (implémentations des traits SPM) vérifient les permissions
- Exemple : `PostgresContentManager` vérifie les permissions avant chaque opération

**Pourquoi c'est PARTIELLEMENT ACCEPTABLE :**

1. **Séparation des responsabilités :** Les adaptateurs sont déjà spécifiques au produit.

2. **Intégration technique :** Les adaptateurs peuvent intégrer avec le système de permissions du produit.

**Limites :**
- Duplication : chaque adaptateur doit implémenter les vérifications
- Risque d'incohérence : vérifications différentes selon l'adaptateur
- Couplage : les adaptateurs deviennent plus complexes

**Verdict : ⚠️ PARTIELLEMENT ACCEPTABLE (mais non recommandé)**

---

## 4. Stratégie recommandée : Produit avec contrat d'interface

### Principe

Les permissions vivent **dans le produit**, mais le système expose un **contrat d'interface minimal** pour permettre l'intégration propre. Cette approche garantit **LOI-1** (aucune dépendance externe critique) : les vérifications de permissions sont locales et ne nécessitent pas d'appel externe. Le système fonctionne même en isolement (**LOI-2**), avec l'état local souverain (**LOI-3**). Voir [Lois d'Autonomie Système](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

### Architecture

```
┌─────────────────────────────────────────┐
│           PRODUIT                        │
│  ┌───────────────────────────────────┐  │
│  │  PermissionManager (trait)        │  │
│  │  - check_permission()              │  │
│  │  - has_access()                    │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  ProductPermissionManager          │  │
│  │  (implémentation produit)          │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Adaptateurs SPM                    │  │
│  │  (appellent PermissionManager)     │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────┐
│         SPM CMS                          │
│  (pas de permissions, opérations pures) │
└─────────────────────────────────────────┘
```

### Contrat d'interface minimal

Le produit définit un trait `PermissionManager` (ou interface) :

```rust
// Conceptuel, pas d'implémentation
trait PermissionManager {
    fn check_permission(
        &self,
        user_id: UserId,
        resource_type: ResourceType,
        resource_id: ResourceId,
        action: Action,
    ) -> Result<bool, PermissionError>;
}
```

**Caractéristiques :**
- Contrat minimal : seulement ce qui est nécessaire pour l'intégration
- Pas d'implémentation : le produit implémente selon ses besoins
- Pas de règles : le produit définit ses propres règles
- Pas de dépendance SPM : le trait est dans le produit, pas dans SPM

### Intégration avec SPM

Les adaptateurs SPM (implémentations des traits ContentManager, MediaManager, etc.) :

1. Reçoivent une référence au `PermissionManager` du produit
2. Vérifient les permissions avant chaque opération
3. Appellent le module SPM seulement si la permission est accordée

**Exemple conceptuel :**
```
AdaptateurContentManager {
    permission_manager: PermissionManager,
    content_manager: MemoryContentManager,
}

impl ContentManager for AdaptateurContentManager {
    fn get_content(&self, id: ContentId) -> Result<Content, ContentError> {
        // 1. Vérifier permission (produit)
        if !self.permission_manager.check_permission(user_id, "content", id, "read")? {
            return Err(ContentError::PermissionDenied);
        }
        
        // 2. Appeler module SPM (opération pure)
        self.content_manager.get_content(id)
    }
}
```

### Avantages

1. **Séparation claire :** Permissions = produit, opérations = SPM
2. **Flexibilité :** Le produit implémente exactement ce dont il a besoin
3. **Réutilisabilité :** Les modules SPM restent génériques
4. **Testabilité :** Les modules SPM sont testables sans permissions
5. **Évolutivité :** Le produit peut changer ses règles sans modifier SPM

### Inconvénients

1. **Pas de mutualisation :** Chaque produit doit implémenter
2. **Duplication possible :** Si plusieurs produits ont les mêmes besoins
3. **Responsabilité produit :** Le produit doit gérer la cohérence

---

## 5. Alternative acceptable : Produit pur (sans contrat)

### Principe

Les permissions vivent **uniquement dans le produit**, sans contrat d'interface standardisé. Chaque produit implémente ses propres règles et vérifications.

### Architecture

```
┌─────────────────────────────────────────┐
│           PRODUIT                        │
│  ┌───────────────────────────────────┐  │
│  │  Règles de permissions             │  │
│  │  (implémentation libre)            │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Adaptateurs SPM                    │  │
│  │  (vérifications ad-hoc)            │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

### Caractéristiques

- **Aucun contrat :** Pas de trait ou interface standardisé
- **Implémentation libre :** Le produit choisit sa structure
- **Vérifications ad-hoc :** Les adaptateurs vérifient selon leurs besoins

### Avantages

1. **Liberté totale :** Le produit fait exactement ce qu'il veut
2. **Pas de contrainte :** Aucun contrat à respecter
3. **Simplicité :** Pas de couche d'abstraction supplémentaire

### Inconvénients

1. **Pas de standardisation :** Difficile de partager du code entre produits
2. **Duplication :** Chaque produit réinvente
3. **Maintenance :** Plus difficile de maintenir plusieurs implémentations

### Quand utiliser cette alternative

- Produit unique, pas de mutualisation prévue
- Besoins très spécifiques, pas de standardisation possible
- Phase exploratoire, besoins non stabilisés

---

## 6. Comparaison des stratégies

| Critère | Stratégie recommandée | Alternative acceptable |
|---------|----------------------|------------------------|
| **Flexibilité** | ✅ Haute (contrat minimal) | ✅✅ Très haute (aucun contrat) |
| **Standardisation** | ✅✅ Contrat standardisé | ❌ Aucune standardisation |
| **Mutualisation** | ✅ Possible (via contrat) | ❌ Difficile |
| **Complexité** | ⚠️ Moyenne (contrat à définir) | ✅ Faible (pas de contrat) |
| **Testabilité** | ✅✅ Haute (contrat testable) | ⚠️ Moyenne (tests ad-hoc) |
| **Évolutivité** | ✅✅ Haute (contrat évolutif) | ⚠️ Moyenne (refactoring manuel) |
| **Réutilisabilité** | ✅✅ Haute | ⚠️ Faible |

**Recommandation :** Stratégie recommandée (produit avec contrat) pour la majorité des cas. Alternative acceptable pour produits uniques ou besoins très spécifiques.

---

## 7. Ce qui ne sera JAMAIS implémenté

### Dans le Kernel

- ❌ Module permissions
- ❌ Trait PermissionChecker
- ❌ Types Role, Permission, Policy
- ❌ Vérifications d'accès
- ❌ Authentification
- ❌ Autorisation

**Raison :** Le kernel est infra pure, sans logique métier.

### Dans le SPM CMS

- ❌ Module permissions
- ❌ Intégration permissions dans Content, Media, etc.
- ❌ Hooks de vérification dans les traits
- ❌ Filtrage automatique par permissions
- ❌ RBAC générique
- ❌ ACL générique

**Raison :** Les permissions sont de la logique métier spécifique, hors-scope SPM (invariant Phase 0).

### Framework de permissions

- ❌ Framework RBAC complet
- ❌ Framework ACL complet
- ❌ Framework ABAC (Attribute-Based Access Control)
- ❌ Moteur de règles de permissions
- ❌ Système de rôles prédéfinis

**Raison :** Les besoins varient trop entre produits. Un framework serait soit trop générique (inutile), soit trop spécifique (non réutilisable).

### Authentification technique

- ❌ Gestion de tokens (JWT, OAuth)
- ❌ Sessions utilisateur
- ❌ Gestion de mots de passe
- ❌ SSO (Single Sign-On)
- ❌ OAuth2 / OpenID Connect

**Raison :** Ce sont des préoccupations techniques, pas fonctionnelles. Le produit choisit sa stack (Auth0, Firebase Auth, etc.).

### Permissions distribuées

- ❌ Réplication de permissions
- ❌ Synchronisation multi-nœuds
- ❌ Cache distribué de permissions

**Raison :** Préoccupations techniques d'infrastructure, hors-scope fonctionnel.

---

## 8. Exemples d'implémentation (conceptuels)

### Exemple 1 : CMS simple

**Produit :** CMS avec rôles simples (admin, éditeur, visiteur)

**Implémentation :**
- `PermissionManager` avec règles simples
- Vérifications dans les adaptateurs avant appels SPM
- Règles : admin = tout, éditeur = CRUD contenu, visiteur = lecture seule

### Exemple 2 : SaaS B2B

**Produit :** SaaS avec organisations, équipes, projets

**Implémentation :**
- `PermissionManager` avec contexte (organisation, équipe, projet)
- Vérifications multi-niveaux
- Règles : membre organisation peut voir projets équipe, admin peut tout

### Exemple 3 : E-shop

**Produit :** E-shop avec clients, vendeurs, admins

**Implémentation :**
- `PermissionManager` avec rôles métier
- Vérifications par ressource (produit, commande, catalogue)
- Règles : client peut commander, vendeur peut gérer catalogue, admin peut tout

**Point commun :** Tous implémentent dans le produit, selon leurs besoins spécifiques.

---

## 9. Règles d'évolution

### Quand on pourra ajouter (dans le produit)

**Nouveau concept :**
- Si besoin spécifique au produit
- Si logique métier claire
- Si pas de dépendance vers kernel ou SPM

**Exemples acceptables :**
- Nouveaux rôles métier
- Nouvelles règles de permissions
- Nouveaux contextes (organisation, projet, etc.)

### Quand on devra REFUSER

**Dans le kernel :**
- Toute logique de permissions
- Toute dépendance vers entités produit

**Dans le SPM :**
- Toute intégration de permissions dans les modules
- Toute modification des traits pour ajouter des hooks de permissions
- Tout filtrage automatique par permissions

**Framework générique :**
- Framework RBAC/ACL/ABAC complet
- Moteur de règles générique
- Système de rôles prédéfinis

---

## 10. Mini résumé : pièges classiques évités

### 1. Piège du kernel universel

**Piège :** Ajouter un module permissions au kernel pour "mutualiser" entre tous les produits.

**Évitement :** Le kernel reste infra pure. Les permissions sont de la logique métier, donc dans le produit.

### 2. Piège du SPM avec permissions intégrées

**Piège :** Intégrer les permissions directement dans les modules SPM (Content, Media, etc.) pour "simplifier" l'usage.

**Évitement :** Les modules SPM restent génériques. Les permissions sont dans le produit, vérifiées avant les appels SPM.

### 3. Piège du framework RBAC complet

**Piège :** Créer un framework RBAC complet "au cas où" pour couvrir tous les besoins futurs.

**Évitement :** Pas de framework générique. Le produit implémente exactement ce dont il a besoin.

### 4. Piège de l'authentification dans le kernel

**Piège :** Ajouter l'authentification (tokens, sessions) au kernel pour "mutualiser".

**Évitement :** L'authentification est technique, pas fonctionnelle. Le produit choisit sa stack (Auth0, Firebase, etc.).

### 5. Piège des hooks de permissions dans les traits

**Piège :** Ajouter des hooks `before_read`, `after_write` dans les traits SPM pour permettre l'injection de vérifications.

**Évitement :** Les traits SPM restent purs. Les vérifications sont dans les adaptateurs, avant les appels.

### 6. Piège du filtrage automatique

**Piège :** Faire filtrer automatiquement les résultats par permissions dans les modules SPM.

**Évitement :** Les modules SPM retournent tous les résultats. Le produit filtre selon ses permissions.

### 7. Piège de l'anticipation multi-produits

**Piège :** Créer un système de permissions "générique" pour anticiper les besoins de plusieurs produits futurs.

**Évitement :** Pas d'anticipation. On implémente seulement ce qui est demandé par un produit réel.

---

## 11. Conclusion

### Stratégie retenue

**Permissions dans le produit avec contrat d'interface minimal.**

- Les permissions sont de la logique métier spécifique
- Le produit implémente selon ses besoins
- Un contrat minimal permet la standardisation sans contrainte excessive
- Les modules SPM restent génériques et réutilisables

### Alternative

**Permissions dans le produit sans contrat** pour produits uniques ou besoins très spécifiques.

### Garanties

- ✅ Kernel reste infra pure
- ✅ SPM reste générique et réutilisable
- ✅ Produit a la flexibilité totale
- ✅ Pas de couplage indésirable
- ✅ Architecture évolutive

---

**Phase 2.3 : STRATÉGIE VALIDÉE ✓**

**Date :** 2026-01-24
