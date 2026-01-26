# BondingBrother - Performance & Scalability Contract

## 1. Contexte

Ce document définit les contraintes de performance et de scalabilité de Bonding Brother. Il établit les engagements mesurables en termes de temps de traitement, de débit, de capacité, et de comportement sous charge, tout en respectant les invariants fondamentaux de Bonding Brother.

Ce document complète la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour définir les métriques de performance des flux.

## 2. Portée / Scope

Ce document couvre :
- Les métriques de performance (temps de traitement, débit, latence)
- Les contraintes de scalabilité (charge, capacité, croissance)
- Les garanties de performance envers les produits
- Les garanties de performance envers les autorités
- Les mécanismes d'optimisation autorisés
- Les limites et seuils critiques
- Le comportement en mode offline

Ce document **ne couvre pas** :
- Les détails d'implémentation des optimisations
- Les stratégies de déploiement
- Les métriques spécifiques aux autorités (Kind Mother, Strong Father)
- Les performances des produits individuels

---

## 3. Principes fondamentaux

### 3.1 Performance sans compromis sur les invariants

**Principe PERF-01 : Invariants prioritaires**

Aucune optimisation de performance ne peut violer les invariants de Bonding Brother. La performance est un objectif, les invariants sont des contraintes absolues.

**Implications :**
- Pas de cache de données métier (violerait INV-NEG-02)
- Pas de décision basée sur la performance (violerait INV-NEG-01)
- Pas de saut d'étape pour gagner du temps (violerait INV-FLUX-01)
- Pas de journalisation optionnelle (violerait INV-FLUX-02)

### 3.2 Scalabilité horizontale

**Principe PERF-02 : Stateless par design**

Bonding Brother est conçu pour être stateless (sans état partagé) afin de permettre la scalabilité horizontale.

**Implications :**
- Aucun état partagé entre instances
- Chaque intention peut être traitée par n'importe quelle instance
- Le journal est externe et partagé (mais immutable)
- La configuration est immuable après démarrage

### 3.3 Performance prévisible

**Principe PERF-03 : Délais bornés**

Les temps de traitement sont bornés et prévisibles, avec des seuils définis pour chaque étape.

**Implications :**
- Pas de boucles infinies
- Pas d'attentes non bornées
- Timeouts définis pour toutes les opérations
- Dégradation gracieuse en cas de surcharge

**Conformité autonomie :** Ce principe garantit **LOI-5** : la consommation de ressources est prévisible et maîtrisée, permettant le fonctionnement sur hardware simple (Raspberry Pi, mini PC, etc.).

---

## 4. Métriques de performance

### 4.1 Temps de traitement (latence)

#### 4.1.1 Flux Produit → Écosystème

| Étape | Métrique | Cible (P50) | Cible (P95) | Cible (P99) | Maximum |
|-------|----------|-------------|-------------|-------------|---------|
| Réception intention | Temps de réception | < 1 ms | < 5 ms | < 10 ms | 50 ms |
| Validation structurelle | Temps de validation | < 2 ms | < 5 ms | < 10 ms | 50 ms |
| Traduction ascendante | Temps de traduction | < 5 ms | < 15 ms | < 30 ms | 100 ms |
| Filtrage d'entrée | Temps de filtrage | < 3 ms | < 10 ms | < 20 ms | 100 ms |
| Journalisation | Temps de journalisation | < 5 ms | < 20 ms | < 50 ms | 200 ms |
| Transmission autorité | Temps réseau + traitement autorité | Variable | Variable | Variable | Timeout configurable |
| Réception réponse | Temps de réception | < 1 ms | < 5 ms | < 10 ms | 50 ms |
| Filtrage de sortie | Temps de filtrage | < 3 ms | < 10 ms | < 20 ms | 100 ms |
| Traduction descendante | Temps de traduction | < 5 ms | < 15 ms | < 30 ms | 100 ms |
| Émission résultat | Temps d'émission | < 1 ms | < 5 ms | < 10 ms | 50 ms |

**Temps total (sans autorité) :**
- P50 : < 30 ms
- P95 : < 100 ms
- P99 : < 200 ms
- Maximum : 800 ms (hors timeout autorité)

**Note :** Le temps de traitement par l'autorité (Kind Mother ou Strong Father) n'est pas inclus dans ces métriques car il dépend de l'autorité, pas de Bonding Brother.

#### 4.1.2 Flux Écosystème → Produit

| Étape | Métrique | Cible (P50) | Cible (P95) | Cible (P99) | Maximum |
|-------|----------|-------------|-------------|-------------|---------|
| Réception notification | Temps de réception | < 1 ms | < 5 ms | < 10 ms | 50 ms |
| Normalisation | Temps de normalisation | < 2 ms | < 5 ms | < 10 ms | 50 ms |
| Filtrage | Temps de filtrage | < 3 ms | < 10 ms | < 20 ms | 100 ms |
| Traduction | Temps de traduction | < 5 ms | < 15 ms | < 30 ms | 100 ms |
| Sélection produits | Temps de sélection | < 2 ms | < 5 ms | < 10 ms | 50 ms |
| Journalisation | Temps de journalisation | < 5 ms | < 20 ms | < 50 ms | 200 ms |
| Distribution | Temps de distribution | < 10 ms | < 50 ms | < 100 ms | 500 ms |

**Temps total :**
- P50 : < 30 ms
- P95 : < 120 ms
- P99 : < 250 ms
- Maximum : 1000 ms

### 4.2 Débit (throughput)

#### 4.2.1 Débit par instance

| Métrique | Cible minimale | Cible optimale | Maximum théorique |
|----------|----------------|----------------|-------------------|
| Intentions/seconde (flux ascendant) | 100 | 1000 | 10000 |
| Notifications/seconde (flux descendant) | 200 | 2000 | 20000 |
| Opérations journalisées/seconde | 300 | 3000 | 30000 |

**Note :** Ces métriques sont par instance. La scalabilité horizontale permet d'augmenter le débit total en ajoutant des instances.

#### 4.2.2 Débit agrégé (multi-instances)

| Métrique | Cible minimale | Cible optimale | Maximum théorique |
|----------|----------------|----------------|-------------------|
| Intentions/seconde (total) | 1000 | 10000 | 100000 |
| Notifications/seconde (total) | 2000 | 20000 | 200000 |

**Note :** Le débit agrégé dépend du nombre d'instances et de la capacité des autorités.

### 4.3 Capacité

#### 4.3.1 Buffer offline

| Métrique | Valeur minimale | Valeur recommandée | Maximum |
|----------|-----------------|---------------------|---------|
| Intentions en attente | 1000 | 10000 | 100000 |
| Taille mémoire buffer | 10 MB | 100 MB | 1 GB |
| Durée de rétention | 24 heures | 7 jours | 30 jours |

**Note :** Au-delà du maximum, les intentions les plus anciennes sont archivées ou rejetées selon la politique configurée.

#### 4.3.2 Journal

| Métrique | Valeur minimale | Valeur recommandée | Maximum |
|----------|-----------------|---------------------|---------|
| Entrées journalisées | 1 million | 10 millions | 100 millions |
| Taille journal | 1 GB | 10 GB | 100 GB |
| Durée de rétention | 30 jours | 90 jours | 365 jours |

**Note :** Le journal peut être archivé au-delà de la durée de rétention.

---

## 5. Garanties de performance

### 5.1 Garantie envers les produits

#### 5.1.1 GAR-PERF-PROD-01 : Temps de réponse borné

**Engagement :** Le temps de traitement d'une intention (hors temps d'autorité) ne dépasse pas les seuils définis dans 95% des cas (P95).

**Mesure :** Métriques de latence collectées en temps réel.

**Action en cas de violation :** Alerte et dégradation gracieuse (rejet avec code d'erreur approprié si timeout).

#### 5.1.2 GAR-PERF-PROD-02 : Débit minimal garanti

**Engagement :** Chaque instance peut traiter au minimum 100 intentions/seconde.

**Mesure :** Métriques de débit collectées en temps réel.

**Action en cas de violation :** Scaling horizontal automatique (si configuré) ou rejet avec code d'erreur de surcharge.

#### 5.1.3 GAR-PERF-PROD-03 : Pas de perte en mode offline

**Engagement :** Toutes les intentions exprimées en mode offline sont conservées et transmises à la reconnexion.

**Mesure :** Vérification de la complétude du buffer offline.

**Action en cas de violation :** Alerte critique et tentative de récupération.

### 5.2 Garantie envers les autorités

#### 5.2.1 GAR-PERF-AUTH-01 : Pas de surcharge

**Engagement :** Bonding Brother ne surcharge pas les autorités avec des demandes excessives.

**Mesure :** Limitation du débit vers chaque autorité (rate limiting).

**Action en cas de violation :** Mise en file d'attente avec backpressure.

#### 5.2.2 GAR-PERF-AUTH-02 : Transmission efficace

**Engagement :** Les demandes transmises aux autorités sont optimisées (pas de duplication, pas de redondance).

**Mesure :** Analyse des demandes transmises.

**Action en cas de violation :** Optimisation des traductions.

---

## 6. Optimisations autorisées

### 6.1 Optimisations de traduction

**Autorisé :**
- Cache de règles de traduction (configuration, pas données métier)
- Pré-compilation des règles de traduction
- Optimisation des transformations de format
- Pool de traducteurs réutilisables

**Interdit :**
- Cache de résultats de traduction (violerait INV-NEG-02)
- Traduction approximative pour gagner du temps (violerait GAR-PROD-02)
- Saut d'étapes de traduction (violerait INV-FLUX-01)

### 6.2 Optimisations de filtrage

**Autorisé :**
- Cache de règles de filtrage (configuration, pas données métier)
- Pré-compilation des règles de filtrage
- Optimisation des évaluations de règles
- Indexation des règles par type d'intention

**Interdit :**
- Cache de résultats de filtrage (violerait INV-NEG-02)
- Filtrage approximatif (violerait GAR-PROD-03)
- Saut d'étapes de filtrage (violerait INV-FLUX-01)

### 6.3 Optimisations de journalisation

**Autorisé :**
- Écriture asynchrone (avec garantie de persistance)
- Batching des écritures
- Compression des entrées
- Archivage automatique

**Interdit :**
- Journalisation optionnelle (violerait INV-FLUX-02)
- Perte d'entrées (violerait INV-FLUX-04)
- Modification d'entrées (violerait l'immutabilité)

### 6.4 Optimisations de routage

**Autorisé :**
- Cache de règles de routage (configuration)
- Pré-détermination de l'autorité cible
- Pool de connexions vers les autorités
- Load balancing entre instances d'autorités

**Interdit :**
- Routage basé sur la performance (violerait INV-NEG-01)
- Bypass d'autorité (violerait INV-NEG-04)
- Modification du routage selon la charge (violerait les règles de routage)

### 6.5 Optimisations de distribution

**Autorisé :**
- Distribution asynchrone
- Batching des notifications
- Pool de connexions vers les produits
- Retry avec backoff exponentiel

**Interdit :**
- Perte de notifications (violerait GAR-PERF-PROD-03)
- Modification du contenu (violerait GAR-AUTH-03)
- Distribution sélective basée sur la performance (violerait les règles de sélection)

---

## 7. Comportement sous charge

### 7.1 Dégradation gracieuse

**Règle CHARGE-01 : Pas de crash**

En cas de surcharge, Bonding Brother ne crash pas. Il rejette les nouvelles intentions avec un code d'erreur approprié.

**Mécanismes :**
- Rate limiting à l'entrée
- File d'attente avec limite de taille
- Rejet avec code `OVERLOAD` ou `SERVICE_UNAVAILABLE`
- Monitoring et alertes

### 7.2 Backpressure

**Règle CHARGE-02 : Propagation de la pression**

Si une autorité est surchargée, Bonding Brother propage la pression vers les produits (backpressure).

**Mécanismes :**
- Détection de surcharge des autorités
- Mise en file d'attente des intentions
- Notification aux produits du délai
- Rejet si la file d'attente est pleine

### 7.3 Priorisation

**Règle CHARGE-03 : Pas de priorisation métier**

Bonding Brother ne priorise jamais les intentions selon des critères métier (violerait INV-NEG-01).

**Autorisé :**
- Priorisation technique (FIFO par défaut)
- Priorisation par type d'intention (si définie par une autorité)
- Priorisation par produit (si définie par configuration)

**Interdit :**
- Priorisation basée sur le contenu métier
- Priorisation basée sur l'utilisateur (sauf si définie par Strong Father)
- Priorisation basée sur la valeur métier

---

## 8. Scalabilité

### 8.1 Scalabilité horizontale

**Principe SCALE-01 : Stateless**

Bonding Brother est conçu pour être stateless, permettant la scalabilité horizontale.

**Implications :**
- Aucun état partagé entre instances
- Chaque instance peut traiter n'importe quelle intention
- Le load balancing est possible sans sticky sessions
- Le scaling est linéaire (ajout d'instances = augmentation proportionnelle du débit)

### 8.2 Scalabilité verticale

**Principe SCALE-02 : Optimisation par instance**

Bonding Brother peut être optimisé verticalement (plus de CPU, mémoire, I/O) pour augmenter le débit par instance.

**Limites :**
- Les gains sont limités par la loi d'Amdahl
- Certaines opérations (I/O réseau, journalisation) ne scalent pas linéairement
- Le scaling vertical a un plafond

### 8.3 Scalabilité du journal

**Principe SCALE-03 : Journal distribué**

Le journal peut être distribué (sharding, réplication) pour scaler avec la charge.

**Contraintes :**
- L'immutabilité doit être préservée
- La traçabilité doit être maintenue
- La cohérence doit être garantie

### 8.4 Scalabilité des autorités

**Principe SCALE-04 : Adaptation aux autorités**

Bonding Brother s'adapte à la capacité des autorités, mais ne peut pas dépasser leurs limites.

**Implications :**
- Le débit total est limité par la capacité des autorités
- Le backpressure est nécessaire si les autorités sont surchargées
- Le scaling de Bonding Brother seul ne suffit pas si les autorités sont le goulot d'étranglement

---

## 9. Mode offline

### 9.1 Performance en mode offline

**Règle OFFLINE-PERF-01 : Pas de dégradation**

En mode offline, les performances de réception et de journalisation restent identiques.

**Métriques :**
- Temps de réception : identique (pas d'autorité)
- Temps de journalisation : identique
- Temps de mise en buffer : < 5 ms (P95)

**Conformité autonomie :** Cette règle garantit **LOI-2** : le système fonctionne normalement en mode offline sans dégradation de performance, confirmant que l'isolement est un état normal et non une erreur.

### 9.2 Synchronisation à la reconnexion

**Règle OFFLINE-PERF-02 : Synchronisation efficace**

La synchronisation à la reconnexion est optimisée pour minimiser le temps de traitement.

**Mécanismes :**
- Transmission par batch
- Parallélisation des transmissions
- Priorisation FIFO (ordre préservé)
- Retry automatique en cas d'échec partiel

**Métriques :**
- Débit de synchronisation : 1000 intentions/seconde minimum
- Temps de synchronisation : proportionnel à la taille du buffer

---

## 10. Monitoring et alertes

### 10.1 Métriques à surveiller

| Métrique | Seuil d'alerte | Seuil critique | Action |
|----------|----------------|----------------|--------|
| Latence P95 | > 100 ms | > 200 ms | Investigation |
| Latence P99 | > 200 ms | > 500 ms | Alerte |
| Débit | < 80% de la cible | < 50% de la cible | Scaling |
| Taux d'erreur | > 1% | > 5% | Alerte |
| Taille buffer offline | > 80% | > 95% | Alerte |
| Taille journal | > 80% | > 95% | Archivage |

### 10.2 Alertes automatiques

**Alerte PERF-01 : Latence élevée**
- Déclenchement : P95 > 200 ms pendant 5 minutes
- Action : Investigation, scaling si nécessaire

**Alerte PERF-02 : Débit insuffisant**
- Déclenchement : Débit < 50% de la cible pendant 5 minutes
- Action : Scaling horizontal

**Alerte PERF-03 : Buffer offline plein**
- Déclenchement : Buffer > 95% de capacité
- Action : Alerte critique, archivage ou rejet selon politique

**Alerte PERF-04 : Surcharge autorité**
- Déclenchement : Taux de timeout > 10% vers une autorité
- Action : Backpressure, alerte vers équipe autorité

---

## 11. Tests de performance

### 11.1 Tests de charge

**Objectif :** Vérifier que les métriques de performance sont respectées sous charge.

**Scénarios :**
- Charge normale (50% de la capacité)
- Charge élevée (80% de la capacité)
- Charge maximale (100% de la capacité)
- Charge excessive (120% de la capacité) - test de dégradation

**Métriques vérifiées :**
- Latence (P50, P95, P99)
- Débit
- Taux d'erreur
- Utilisation des ressources

### 11.2 Tests de scalabilité

**Objectif :** Vérifier que le scaling fonctionne correctement.

**Scénarios :**
- Ajout d'instances (scaling horizontal)
- Augmentation des ressources (scaling vertical)
- Réduction des instances (scaling down)

**Métriques vérifiées :**
- Débit total
- Distribution de charge
- Stabilité

### 11.3 Tests de résilience

**Objectif :** Vérifier le comportement en cas de défaillance.

**Scénarios :**
- Défaillance d'une autorité
- Défaillance d'une instance de Bonding Brother
- Défaillance du journal
- Déconnexion réseau

**Métriques vérifiées :**
- Temps de récupération
- Perte de données
- Continuité de service

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les engagements de performance de Bonding Brother qui doivent être respectés pour garantir un service fiable et performant.

Toute implémentation de Bonding Brother doit respecter ces contraintes de performance. Toute violation doit être corrigée ou justifiée par une évolution du contrat.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0
- Bilateral Flow Contract v1.0
- Architecture et Composants v1.0
- Invariants et Garanties v1.0
