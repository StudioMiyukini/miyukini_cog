# Module Recherche & Indexation — Contrat fonctionnel

> Capacité CMS cœur. Indexation fonctionnelle et recherche générique d'entités : indexation, mise à jour, recherche par critères.

---

## 1. Responsabilité du module

Le Module Recherche & Indexation fournit une **capacité fonctionnelle générique** d'indexation et de recherche d'entités (contenus, médias, etc.). Il permet d'indexer des champs d'entités, de rechercher par critères simples, et de maintenir la cohérence de l'index. Il ne décide jamais du scoring, du ranking, de la pertinence, des permissions, ni de la logique métier de recherche.

---

## 2. Concepts fondamentaux

### Index

Un **index** est une structure fonctionnelle qui associe des entités à leurs champs indexés. L'index permet de rechercher rapidement des entités par leurs champs. Un index est indépendant des entités source : il contient une copie fonctionnelle des données nécessaires à la recherche.

### IndexedEntity

Une **entité indexée** est une référence vers une entité externe (Content, Media, etc.) avec ses champs indexés. Chaque entité indexée possède :
- Un identifiant d'entité (EntityId) — référence vers l'entité source
- Un type d'entité (EntityType) — opaque, défini par le produit (ex. "content", "media")
- Des champs indexés (IndexedField) — données extraites de l'entité source pour la recherche

### IndexedField

Un **champ indexé** est une donnée extraite d'une entité et stockée dans l'index pour permettre la recherche. Chaque champ indexé possède :
- Un nom de champ (FieldName) — opaque, défini par le produit (ex. "title", "content_type", "status")
- Une valeur (FieldValue) — format opaque, défini par le produit (String, nombre, date, etc.)
- Un type de recherche (SearchType) — comment le champ peut être recherché (exact, contient, supérieur, etc.)

Le module ne valide pas les noms de champs, ne les interprète pas, ne les transforme pas. Il stocke et recherche selon les critères fournis. Le module ne compare jamais des types hétérogènes (ex. String vs Date). Le produit garantit la cohérence des types.

### Query

Une **requête** est un ensemble de critères de recherche. Chaque critère spécifie :
- Un nom de champ (FieldName)
- Un opérateur de recherche (Operator) — égal, contient, supérieur, inférieur, etc.
- Une valeur de recherche (SearchValue) — format opaque

Une requête peut contenir plusieurs critères combinés avec des opérateurs logiques (ET, OU). Le module ne valide pas la sémantique des critères, ne les interprète pas, ne les transforme pas. L'ordre d'évaluation des critères n'est pas garanti.

### SearchResult

Un **résultat de recherche** est une liste d'entités correspondant aux critères de la requête. Le résultat contient :
- Une liste d'identifiants d'entités (EntityId) — ordre non garanti (pas de ranking)
- Le nombre total de résultats (Total) — nombre total d'entités correspondantes
- Des métadonnées de pagination (Offset, Limit) — si pagination demandée

Le module ne garantit pas d'ordre spécifique, ne fait pas de ranking, ne calcule pas de score de pertinence.

### Relations avec les entités source

L'index est une **copie fonctionnelle** des données nécessaires à la recherche. Il n'est pas lié directement aux entités source :
- L'indexation est explicite : le produit doit appeler "indexer" pour ajouter/mettre à jour une entité dans l'index
- La désindexation est explicite : le produit doit appeler "désindexer" pour retirer une entité de l'index
- La cohérence est gérée par le produit : le module ne vérifie pas si une entité source existe encore

---

## 3. Invariants fonctionnels

### Garantis

1. **Identité unique :** Une entité ne peut être indexée qu'une seule fois par type d'entité. Réindexer une entité existante met à jour l'index.
2. **Cohérence fonctionnelle :** Les champs indexés reflètent l'état de l'entité au moment de l'indexation (pas de synchronisation automatique).
3. **Recherche déterministe :** Pour une même requête et un même index, les résultats sont déterministes (même si l'ordre n'est pas garanti).
4. **Données opaques :** Les noms de champs, valeurs, types de recherche sont stockés et restitués sans transformation.
5. **Indépendance des entités source :** L'index peut contenir des références vers des entités supprimées (le produit gère la cohérence).

### Interdits

1. **Pas de scoring :** Le module ne calcule pas de score de pertinence, ne fait pas de ranking.
2. **Pas de full-text linguistique :** Le module ne fait pas de stemming, lemmatisation, analyse linguistique.
3. **Pas de synchronisation automatique :** Le module ne synchronise pas automatiquement l'index avec les entités source.
4. **Pas de permissions :** Le module ne gère pas les permissions d'accès aux résultats.
5. **Pas de logique métier :** Le module ne valide pas les règles business, ne transforme pas les données.
6. **Pas d'ordre garanti :** Le module ne garantit pas d'ordre spécifique des résultats (pas de tri par défaut).
7. **Pas de recherche sémantique :** Le module ne fait pas de recherche sémantique, de similarité, de clustering.

---

## 4. Opérations fonctionnelles (conceptuelles)

### Indexer une entité

Ajouter ou mettre à jour une entité dans l'index. Spécifier : identifiant d'entité, type d'entité, liste de champs indexés (nom, valeur, type de recherche). Si l'entité existe déjà, remplacer ses champs indexés. Retourne confirmation ou erreur si contraintes.

### Désindexer une entité

Retirer une entité de l'index. Spécifier : identifiant d'entité, type d'entité. Retourne confirmation ou erreur si entité non indexée.

### Rechercher

Rechercher des entités dans l'index selon des critères. Spécifier : type d'entité (optionnel), liste de critères (champ, opérateur, valeur), opérateurs logiques (ET, OU), pagination (offset, limit). Retourne liste d'identifiants d'entités correspondantes, nombre total, métadonnées de pagination.

### Lister les entités indexées

Lister toutes les entités indexées d'un type donné. Spécifier : type d'entité (optionnel), pagination (offset, limit). Retourne liste d'identifiants d'entités, nombre total, métadonnées de pagination.

### Vérifier si une entité est indexée

Vérifier si une entité est présente dans l'index. Spécifier : identifiant d'entité, type d'entité. Retourne vrai/faux.

### Obtenir les champs indexés d'une entité

Récupérer les champs indexés d'une entité. Spécifier : identifiant d'entité, type d'entité. Retourne liste de champs indexés (nom, valeur, type de recherche) ou erreur si entité non indexée.

### Vider l'index

Supprimer toutes les entités indexées d'un type donné (ou de tous les types). Spécifier : type d'entité (optionnel). Retourne confirmation. **Opération administrative et destructive :** aucune annulation possible, pas de rollback. Le produit doit réindexer les entités après un vidage.

---

## 5. Cas d'usage supportés

### Recherche de contenus par titre

Indexer des contenus avec un champ "title", rechercher les contenus dont le titre contient un terme donné.

### Recherche par type et statut

Indexer des contenus avec des champs "content_type" et "status", rechercher les contenus d'un type donné avec un statut donné.

### Recherche par date

Indexer des contenus avec un champ "created_at", rechercher les contenus créés après une date donnée.

### Recherche combinée

Rechercher des contenus qui correspondent à plusieurs critères (ex. type="article" ET status="published" ET created_at > date).

### Recherche de médias par type MIME

Indexer des médias avec un champ "mime_type", rechercher les médias d'un type MIME donné.

### Recherche par taxonomie

Indexer des entités avec des champs de taxonomie (ex. "tags"), rechercher les entités ayant un tag donné.

---

## 6. Cas d'usage explicitement refusés

### Full-text search linguistique

Le module ne fait pas de recherche full-text avec stemming, lemmatisation, analyse linguistique. Le produit peut indexer le texte brut et rechercher par correspondance exacte ou contient, mais le module ne fait pas d'analyse linguistique.

### Scoring et ranking

Le module ne calcule pas de score de pertinence, ne fait pas de ranking des résultats. Le produit peut trier les résultats selon ses propres critères après la recherche.

### Recherche sémantique

Le module ne fait pas de recherche sémantique, de similarité, de clustering, d'embedding. Le produit peut implémenter cela en amont en indexant des embeddings comme champs opaques.

### Recherche avec permissions

Le module ne filtre pas les résultats selon les permissions utilisateur. Le produit filtre les résultats après la recherche selon ses propres règles.

### SEO et référencement

Le module ne gère pas le SEO, les meta tags, les sitemaps. Le produit utilise le module pour indexer des données SEO, mais le module ne fait pas de traitement SEO.

### Synchronisation automatique

Le module ne synchronise pas automatiquement l'index avec les entités source. Le produit doit appeler explicitement "indexer" lors des modifications d'entités.

### Recherche avec cache

Le module ne gère pas de cache de résultats. Le produit peut implémenter un cache en amont.

### Recherche distribuée

Le module ne gère pas la recherche distribuée, la réplication d'index, la sharding. Le produit peut implémenter cela dans son adaptateur.

---

## 7. Interactions avec les autres modules SPM

### Content

**Relation :** Le module Recherche peut indexer des contenus (ContentId).

**Opérations :**
- Indexer un contenu nécessite un ContentId valide (validation par le produit)
- Rechercher des contenus retourne des ContentId
- Le produit doit indexer/mettre à jour/désindexer les contenus lors des modifications

**Dépendance :** Aucune dépendance directe. Le produit orchestre l'indexation lors des opérations Content.

### Media

**Relation :** Le module Recherche peut indexer des médias (MediaId).

**Opérations :**
- Indexer un média nécessite un MediaId valide (validation par le produit)
- Rechercher des médias retourne des MediaId
- Le produit doit indexer/mettre à jour/désindexer les médias lors des modifications

**Dépendance :** Aucune dépendance directe. Le produit orchestre l'indexation lors des opérations Media.

### Publication

**Relation :** Indirecte via Content. Le module Recherche peut indexer des champs de publication (statut, date de publication) comme champs opaques.

**Opérations :**
- Aucune opération directe. Le produit indexe les champs de publication comme champs opaques.

**Dépendance :** Aucune dépendance directe. Le produit orchestre l'indexation.

### Taxonomies

**Relation :** Le module Recherche peut indexer des termes de taxonomie comme champs opaques.

**Opérations :**
- Indexer des termes de taxonomie comme champs (ex. "tags" avec valeur "news")
- Rechercher des entités par terme de taxonomie
- Le produit doit mettre à jour l'index lors des changements de taxonomie

**Dépendance :** Aucune dépendance directe. Le produit orchestre l'indexation.

### Hierarchy

**Relation :** Aucune relation directe. Le module Recherche peut indexer des informations hiérarchiques comme champs opaques (ex. "parent_id", "depth").

**Opérations :**
- Aucune opération croisée. Le produit indexe les informations hiérarchiques comme champs opaques.

**Dépendance :** Aucune dépendance.

---

## 8. Règles d'évolution

### Quand on pourra ajouter

**Nouveau concept :**
- Si besoin partagé par ≥2 produits CMS
- Si responsabilité strictement fonctionnelle (pas de métier, pas de technique)
- Si dépendances claires et unidirectionnelles

**Exemples acceptables :**
- Types de recherche supplémentaires (recherche par plage, recherche par regex simple)
- Opérateurs logiques supplémentaires (NOT, XOR)
- Métadonnées d'index (date d'indexation, version d'index)

**Nouvelle capacité :**
- Si opération fonctionnelle pure (indexation, recherche, maintenance)
- Si pas de logique métier
- Si pas de scoring, ranking, pertinence

**Exemples acceptables :**
- Recherche par plage de valeurs (ex. dates entre X et Y)
- Recherche par regex simple (correspondance de pattern)
- Statistiques d'index (nombre d'entités indexées par type)

### Quand on devra REFUSER

**Scoring et ranking :**
- Calcul de score de pertinence
- Ranking des résultats
- Boost de champs
- Fonctions de scoring personnalisées

**Full-text linguistique :**
- Stemming, lemmatisation
- Analyse linguistique
- Dictionnaires de synonymes
- Correction orthographique

**Recherche sémantique :**
- Embeddings, vecteurs
- Similarité sémantique
- Clustering
- Classification automatique

**Permissions et accès :**
- Filtrage par permissions
- Règles d'accès
- Validation des permissions

**Synchronisation automatique :**
- Synchronisation automatique avec entités source
- Écouteurs d'événements
- Hooks de synchronisation

**Cache et performance :**
- Cache de résultats
- Cache d'index
- Optimisations de performance spécifiques

**Recherche distribuée :**
- Réplication d'index
- Sharding
- Recherche distribuée
- Coordination multi-nœuds

**Anticipation :**
- Fonctionnalités "au cas où"
- Capacités non demandées par ≥2 produits
- Optimisations prématurées

---

## 9. Mini résumé : dérives évitées / erreurs classiques

### 1. Dérive vers le moteur de recherche technique

**Piège :** Transformer le module en moteur de recherche complet avec scoring, ranking, full-text linguistique, similarité sémantique.

**Évitement :** Contrat strict : pas de scoring, pas de ranking, pas de full-text linguistique. Le module fournit une recherche fonctionnelle basique, le produit peut utiliser un moteur technique en amont.

### 2. Dérive vers la synchronisation automatique

**Piège :** Ajouter des écouteurs d'événements, des hooks de synchronisation, une synchronisation automatique avec les entités source.

**Évitement :** Hors-scope explicite : pas de synchronisation automatique. L'indexation est explicite, le produit orchestre la cohérence.

### 3. Dérive vers les permissions

**Piège :** Filtrer les résultats selon les permissions utilisateur, gérer les règles d'accès.

**Évitement :** Hors-scope explicite : pas de permissions. Le produit filtre les résultats après la recherche selon ses propres règles.

### 4. Dérive vers le SEO

**Piège :** Gérer le SEO, les meta tags, les sitemaps, les optimisations de référencement.

**Évitement :** Hors-scope explicite : pas de SEO. Le produit peut indexer des données SEO comme champs opaques, mais le module ne fait pas de traitement SEO.

### 5. Dérive vers la recherche sémantique

**Piège :** Ajouter des embeddings, de la similarité sémantique, du clustering, de la classification automatique.

**Évitement :** Hors-scope explicite : pas de recherche sémantique. Le produit peut indexer des embeddings comme champs opaques, mais le module ne fait pas de traitement sémantique.

### 6. Dérive vers le cache et les optimisations prématurées

**Piège :** Ajouter un cache de résultats, des optimisations de performance spécifiques, des optimisations prématurées.

**Évitement :** Hors-scope explicite : pas de cache. Le produit peut implémenter un cache en amont. Pas d'optimisations prématurées.

### 7. Anticipation et bloat

**Piège :** Ajouter des fonctionnalités "au cas où" (recherche distribuée, sharding, réplication) avant qu'elles ne soient demandées par ≥2 produits.

**Évitement :** Règles d'évolution strictes : besoin partagé par ≥2 produits, responsabilité fonctionnelle pure. Refus explicite de l'anticipation.

---

## 10. Mini résumé erreurs / warnings rencontrés et corrigés pendant la rédaction

### Erreurs conceptuelles évitées

1. **Tentation full-text search linguistique**
   - **Erreur :** Inclure le stemming, la lemmatisation, l'analyse linguistique
   - **Correction :** Hors-scope explicite, le module fait de la recherche basique par correspondance

2. **Tentation scoring/ranking**
   - **Erreur :** Inclure le calcul de score de pertinence, le ranking des résultats
   - **Correction :** Hors-scope explicite, le module retourne des résultats sans ordre garanti

3. **Tentation permissions**
   - **Erreur :** Inclure le filtrage par permissions, les règles d'accès
   - **Correction :** Hors-scope explicite, le produit filtre après la recherche

4. **Tentation synchronisation automatique**
   - **Erreur :** Inclure des écouteurs d'événements, des hooks de synchronisation
   - **Correction :** Hors-scope explicite, l'indexation est explicite

5. **Tentation recherche sémantique**
   - **Erreur :** Inclure les embeddings, la similarité sémantique, le clustering
   - **Correction :** Hors-scope explicite, le produit peut indexer des embeddings comme champs opaques

6. **Tentation dépendances techniques**
   - **Erreur :** Inclure des dépendances vers des moteurs de recherche (Elasticsearch, Lucene, etc.)
   - **Correction :** Contrat fonctionnel pur, aucune dépendance technique imposée

7. **Tentation anticipation Phase 3**
   - **Erreur :** Inclure des fonctionnalités avancées (recherche distribuée, sharding) non demandées
   - **Correction :** Règles d'évolution strictes, refus de l'anticipation

### Warnings conceptuels

1. **Ordre des résultats non garanti**
   - **Warning :** Les résultats n'ont pas d'ordre garanti (pas de ranking)
   - **Action :** Documenté explicitement dans les invariants

2. **Cohérence fonctionnelle vs technique**
   - **Warning :** L'index peut contenir des références vers des entités supprimées
   - **Action :** Documenté explicitement, le produit gère la cohérence

3. **Données opaques**
   - **Warning :** Les noms de champs, valeurs, types de recherche sont opaques
   - **Action :** Documenté explicitement, le module ne valide pas, ne transforme pas

---

**Contrat fonctionnel :** Ce document définit le contrat fonctionnel, indépendant de toute implémentation technique. Une implémentation Rust pourrait exposer un trait `SearchManager`, mais le contrat reste fonctionnel.

**Extensibilité :** Le module est conçu pour être extensible par le produit (types d'entités, champs, types de recherche) sans modification du contrat de base.

**Performance :** Les considérations de performance (indexation, recherche, maintenance) sont du ressort du produit ou d'un module infra futur.
