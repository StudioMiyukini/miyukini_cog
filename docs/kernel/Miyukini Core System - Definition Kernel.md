# Miyukini Core System — Definition Kernel

## Contexte

Dépôt **miyukini-core-system**, langage **Rust**. Cas d’usage : backends SaaS, sites web, applications temps réel, jeux multijoueurs (MMO à long terme). Priorité : livrer des produits SaaS et web en premier. Contraintes : développement solo ou petite équipe, coûts d’infrastructure très faibles, pas de bloat ni d’abstractions prématurées, un seul kernel pour plusieurs surfaces (web, mobile, jeu), maintenabilité 5 à 10 ans.

**Dans le cadre de Miyukini Core System, le terme « kernel » désigne le noyau technique minimal de la fondation, et non un kernel système au sens OS.**

## Portée

Ce document définit le périmètre du kernel : ce qu’il est, ce qu’il n’est pas, ses responsabilités, ses exclusions et ses frontières. Il ne décrit pas l’architecture des produits ni l’implémentation. **La dépendance est strictement unidirectionnelle : les produits dépendent du kernel, jamais l’inverse.**

---

## 1. Définition du Kernel

### Ce que le kernel EST

- La **fondation technique réutilisable** sur laquelle s’appuient plusieurs produits (SaaS, web, mobile, jeu).
- Une **couche d’exécution et de coordination** : boot, config, arrêt, observabilité de base. Pas l’application.
- Un **ensemble de briques transversales** en Rust : identifiants, temps, configuration, logs, lifecycle. Toutes les surfaces (web, mobile, jeu) en dépendent pour ces besoins.
- **Agnostique produit** : aucun métier, aucune UI, aucun protocole applicatif (HTTP, WebSocket, etc.) n’y est implémenté.

Le kernel ne connaît pas les sessions utilisateur, les commandes, ni les entités métier.

### Ce que le kernel N’EST PAS

- Un framework applicatif complet (pas d’Axum, Actix, Rocket à l’intérieur).
- Un ORM, une couche d’accès données ou un serveur HTTP.
- Le lieu du métier (auth, facturation, réservations, gameplay).
- Une suite d’outils d’ops (APM, tracing distribué, métriques avancées) — au mieux, des points d’accroche pour des intégrations futures.

---

## 2. Responsabilités fondamentales

Seules les responsabilités **strictement infra et transverses** appartiennent au kernel. Chaque entrée ci‑dessous justifie pourquoi elle vit dans le kernel.

### Boot et lifecycle

Tout processus (SaaS, serveur de jeu, worker) doit démarrer, charger la config, initialiser les briques, puis s’arrêter proprement. C’est transverse et infra. Le lifecycle gère l’ordre d’initialisation et d’arrêt des briques techniques, **pas l’orchestration de workflows métier** — ni jobs métier, ni hooks applicatifs.

### Configuration

Env, fichiers, secrets. Tous les produits en ont besoin ; centraliser évite la duplication et les erreurs (ex. clés en clair). Le kernel fournit le chargement et l’accès structuré ; le produit décide des clés et des valeurs. Le kernel ne définit aucune politique de configuration (noms de variables, structure métier) ; il fournit uniquement les mécanismes de chargement et d’accès.

### Identifiants

Génération d’IDs (UUID, ULID, ou dérivés déterministes pour le jeu). Partagé par l’API, la persistance, les événements. Un seul point de génération évite les incohérences et les dépendances dispersées.

### Abstraction temps

`now()`, timezone, horodatage. Indispensable pour tests, audit, et jeux (tick, simulation). Un seul point de vérité évite les appels système dispersés et permet l’injection en test.

### Logging structuré

Logs par niveau, format structuré (JSON ou clé‑valeur). Base d’observabilité sans imposer un stack (Prometheus, Jaeger, etc.). Le produit choisit les backends et les niveaux ; le kernel fournit le contrat et une implémentation par défaut. L’implémentation par défaut est minimale et remplaçable ; elle ne doit pas imposer un backend ou un format spécifique.

### Connexions / pooling (Phase 2)

Abstractions minimales pour DB, Redis, etc. : création, pool, fermeture au shutdown. Partagé par SaaS et serveurs de jeu. **Exclu de la v0.1** : les premiers produits gèrent leur propre pool ; le module entre au kernel quand au moins deux produits en ont besoin sans duplication raisonnable.

---

## 3. Exclusions explicites

Les éléments suivants **ne doivent jamais** faire partie du kernel.

| Élément | Raison |
|--------|--------|
| **Auth** (JWT, OAuth, sessions, RBAC) | Métier et spécifique produit. Un module auth peut importer le kernel ; le kernel n’importe pas l’auth. |
| **ORM, requêtes SQL, schémas de BDD** | Couche données = produit. |
| **Routes HTTP, middlewares, validation de payloads** | Choix du framework et du produit. |
| **Règles métier** (facturation, résa, gameplay, matchmaking) | 100 % produit. |
| **UI, templates, assets, SSR** | Hors périmètre backend. |
| **Métriques, APM, tracing distribué** | Outils d’ops ; le kernel fournit au plus des hooks (ex. log), pas l’intégration. |
| **Moteur de jeu** (physique, rendu, audio) | Hors kernel. |
| **Clients spécifiques** (Stripe, SendGrid, etc.) | Produit ou modules applicatifs. |
| **Sérialisation et formats** (JSON, protobuf, etc.) | Chaque produit choisit ; le kernel ne impose pas. |
| **Runtime async / executor** | Le kernel définit des contrats (traits, types) ; le choix de Tokio ou autre reste au produit qui l’intègre. |
| **Jobs métier, hooks applicatifs, workflows** | Le lifecycle est technique ; l’orchestration métier vit dans le produit. |

---

## 4. Frontières du Kernel

Les interactions sont décrites comme **contrats** (rôles, responsabilités), pas comme implémentations.

### Frontends web

Ils parlent au **backend (produit)**. Le backend consomme le kernel (config, log, id, time, lifecycle).

**Contrat :** le kernel expose des traits / types (Config, Logger, IdGenerator, Clock, Lifecycle) que le binaire serveur utilise. Le produit choisit le serveur HTTP, les routes et le format des réponses. Le kernel ne connaît pas HTTP.

### Applications mobiles

Le mobile appelle le **backend**. Le kernel vit côté serveur.

Si, plus tard, du Rust partagé (logique offline, etc.) tourne côté mobile : mêmes contrats (config, id, time, log) ; pas de contrat spécifique « mobile » dans le kernel.

### Clients de jeu

Le client parle au **serveur de jeu (produit)**. Le serveur utilise le kernel pour config, log, id, time, lifecycle.

**Contrat :** identique aux autres surfaces. Aucun protocole de jeu (messages, états, tick) dans le kernel.

### Services externes

Le kernel **n’appelle pas** de services externes. Il ne contient pas de clients (email, paiement, API tierce).

Un produit construit ses clients au‑dessus du kernel. En v0.1, pas de module « http‑client » ou « retry » dans le kernel.

---

## 5. Modules minimaux du Kernel (v0.1)

Ensemble **le plus petit** pour livrer un premier produit (SaaS / web) et prouver la réutilisation. Chaque module : une responsabilité, utile à **au moins deux types de produits** (ex. SaaS + jeu, ou web + worker).

| Module | Responsabilité | Bénéficiaires (ex.) |
|--------|----------------|----------------------|
| **config** | Chargement de la configuration (env, fichiers, secrets). | SaaS, workers, serveurs de jeu. |
| **id** | Génération d’identifiants (UUID/ULID). | API, BDD, événements ; jeu pour IDs déterministes si extension. |
| **time** | Abstraction temps (now, timezone, tests). | Audit, jobs, simulation. |
| **log** | Logging structuré (niveaux, sortie). | Tous les binaires. |
| **lifecycle** | Boot / shutdown : ordre d’init, hooks d’arrêt des briques techniques. **Pas l’orchestration de workflows métier** — ni jobs métier, ni hooks applicatifs. | API, worker, serveur de jeu. |

**Phase 2 (hors v0.1) :** `connection` / `pool` (quand 2+ produits en ont besoin) ; `error` (types partagés lorsque le besoin transverse apparaît).

---

## 6. Stratégie d’évolution

### Évolution dans le temps

Le kernel grandit uniquement quand une nécessité **transversale** et **infra** apparaît. Priorité : stabilité des contrats (traits, signatures) pour ne pas casser les produits. Les dépendances externes (crates) restent minimales ; chaque ajout doit être justifié.

**Le kernel ne doit jamais devenir un point d’attraction de dépendances applicatives. Toute dépendance ajoutée doit être justifiée comme strictement infra et transverse.**

### Règles pour AJOUTER un module

Toutes doivent être vraies :

1. Au moins **2 produits ou 2 surfaces** en ont besoin.
2. La responsabilité est **clairement infra** (pas de métier).
3. Le module reste **petit** et sans dépendance business.
4. Aucun produit existant ne peut raisonnablement le fournir sans duplication inutile.

### Règles pour NE PAS ajouter

1. Un **seul** produit en a besoin → ce produit l’implémente.
2. C’est du **métier** (auth, facturation, règles de jeu, etc.) → reste dans le produit.
3. Cela introduit des **dépendances lourdes** ou un framework applicatif → rejet.
4. La frontière avec le produit devient **floue** (ex. « un peu d’auth pour les tests ») → ne pas mettre dans le kernel.

En cas de doute, on n’ajoute pas. Il est plus facile d’extraire du produit vers le kernel que de retirer du kernel.

---

## 7. Critères de succès

Le kernel est **suffisamment bon** pour commencer à livrer des produits lorsque :

1. **Livraison d’un premier produit** — Un service (ex. API SaaS minimale) peut démarrer, tourner et s’arrêter en s’appuyant uniquement sur les modules du kernel (config, log, id, time, lifecycle).

2. **Zéro métier dans le kernel** — Aucune règle métier, aucun concept domaine (user, order, match, etc.) n’apparaît dans le kernel.

3. **Compréhension rapide** — Un nouveau dev comprend le périmètre du kernel (ce qu’il fait et ce qu’il ne fait pas) en **moins d’une heure** via ce document et l’arborescence des modules.

4. **Réutilisation effective** — Un **second** produit ou une **seconde** surface (ex. worker, autre API, serveur de jeu minimal) réutilise le kernel **sans fork** et sans dupliquer config / log / id / time / lifecycle.

5. **Stabilité des contrats** — Les traits / types exposés sont identifiés ; les changements de contrat sont explicitement versionnés et documentés (même de façon minimale).

On ne passe pas à la phase suivante (ex. généralisation à plus de surfaces) tant que ces cinq critères ne sont pas atteints.
