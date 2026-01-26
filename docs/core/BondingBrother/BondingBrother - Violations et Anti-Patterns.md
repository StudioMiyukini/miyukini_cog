# BondingBrother - Violations et Anti-Patterns

## 1. Contexte

Ce document liste exhaustivement les violations et anti-patterns que Bonding Brother ne doit **JAMAIS** commettre. Il complète la Section 10 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et les [Invariants et Garanties](./BondingBrother%20-%20Invariants%20et%20Garanties.md) en détaillant ce qui est explicitement interdit.

Ce document sert de référence pour :
- Les développeurs implémentant Bonding Brother
- Les audits de code et d'architecture
- Les revues de design
- Les tests de non-régression

Les violations incluent également celles des [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md) : dépendances externes critiques (**LOI-1**), blocage en attente de ressources externes (**LOI-2**), remise en question de l'état local (**LOI-3**), dépendance au temps global (**LOI-4**), consommation excessive de ressources (**LOI-5**).

## 2. Portée / Scope

Ce document couvre :
- Les violations d'invariants (ce que BB ne doit jamais faire)
- Les anti-patterns architecturaux (structures interdites)
- Les anti-patterns comportementaux (comportements interdits)
- Les anti-patterns d'intégration (interactions interdites)
- Les mécanismes de détection

Ce document **ne couvre pas** :
- Les erreurs de traduction (voir Error & Rejection Model)
- Les cas d'erreur légitimes (voir Error & Rejection Model)
- Les détails d'implémentation des vérifications

---

## 3. Principe fondamental

**Toute violation listée ici est une faute critique qui remet en question la nature même de Bonding Brother.**

Ces violations ne sont pas des erreurs à gérer, mais des impossibilités structurelles. Si une violation est détectée, c'est un défaut de conception ou d'implémentation qui doit être corrigé immédiatement.

---

## 4. Violations de nature (ce que Bonding Brother ne peut pas être)

### 4.1 VIOL-NAT-01 : Devenir une autorité

**Violation :** Bonding Brother détient une vérité, prend une décision finale, ou définit une règle.

**Exemples de violation :**
- Un composant stocke l'état des permissions et décide de l'autorisation
- Un composant crée une règle métier dynamiquement
- Un composant détient une copie de données métier comme source de vérité

**Détection :**
- Recherche de méthodes `decide()`, `rule()`, `store_truth()` dans le code
- Audit des structures de données : aucune ne doit représenter un "état courant" métier
- Vérification qu'aucun composant ne prend de décision basée sur des critères métier

**Correction :**
- Déléguer toute décision à Strong Father ou Kind Mother
- Supprimer tout stockage de vérité métier
- Transmettre les règles depuis les autorités, ne jamais les créer

---

### 4.2 VIOL-NAT-02 : Exécuter des actions métier

**Violation :** Bonding Brother modifie, crée, ou supprime des entités métier directement.

**Exemples de violation :**
- Un composant écrit directement dans une base de données métier
- Un composant crée un contenu sans passer par Kind Mother
- Un composant supprime une ressource sans délégation

**Détection :**
- Recherche d'appels directs à des bases de données métier
- Vérification que tous les accès aux données passent par les adaptateurs d'autorité
- Audit des opérations CRUD : toutes doivent être déléguées

**Correction :**
- Toute action métier doit être traduite en intention et déléguée à une autorité
- Supprimer tout accès direct aux données métier

---

### 4.3 VIOL-NAT-03 : Être source d'information

**Violation :** Bonding Brother génère ou fabrique des données sans source autoritaire.

**Exemples de violation :**
- Un composant génère des identifiants de ressources métier
- Un composant fabrique une réponse sans avoir reçu de réponse d'autorité
- Un composant crée des données de synthèse non demandées par une autorité

**Détection :**
- Traçabilité complète : toute donnée sortante doit avoir une source (autorité) identifiée
- Vérification qu'aucun composant ne génère de données métier
- Audit des réponses : toutes doivent provenir d'une autorité

**Correction :**
- Toute donnée doit provenir d'une autorité ou être une métadonnée technique
- Supprimer toute génération de données métier

---

## 5. Violations de non-action (ce que Bonding Brother ne doit jamais faire)

### 5.1 VIOL-NEG-01 : Prendre une décision métier

**Violation :** Bonding Brother prend une décision stratégique, politique, ou opérationnelle.

**Exemples de violation :**
- Un composant autorise ou refuse un accès sans consulter Strong Father
- Un composant valide une donnée métier sans consulter Kind Mother
- Un composant choisit entre plusieurs options métier
- Un composant définit une priorité métier

**Détection :**
- Analyse de code : aucune logique conditionnelle basée sur des critères métier
- Vérification que toutes les décisions sont déléguées
- Audit des branches conditionnelles : seules les décisions techniques sont autorisées

**Correction :**
- Déléguer toute décision métier à l'autorité appropriée
- Transformer les conditions métier en délégations

---

### 5.2 VIOL-NEG-02 : Stocker la vérité

**Violation :** Bonding Brother stocke l'état des données, des identités, ou des permissions.

**Exemples de violation :**
- Un composant maintient un cache de données métier
- Un composant stocke les permissions des utilisateurs
- Un composant réplique l'état d'une autorité
- Un composant garde une copie de données "pour performance"

**Détection :**
- Audit des structures de données : aucune ne doit représenter un état métier
- Recherche de caches de données métier
- Vérification que seuls le journal et le buffer offline sont utilisés pour stockage

**Correction :**
- Supprimer tout cache de données métier
- Utiliser uniquement le journal (immuable) et le buffer offline (temporaire)
- Toujours interroger l'autorité pour obtenir la vérité

---

### 5.3 VIOL-NEG-03 : Créer une règle

**Violation :** Bonding Brother crée, modifie, ou supprime une règle.

**Exemples de violation :**
- Un composant définit dynamiquement qui peut accéder à quoi
- Un composant crée une règle de validation
- Un composant modifie une règle de filtrage sans source externe

**Détection :**
- Vérification que toutes les règles sont chargées depuis une source externe
- Audit des règles : aucune ne doit être générée par le code
- Recherche de méthodes `create_rule()`, `modify_rule()`

**Correction :**
- Charger toutes les règles depuis les autorités ou la configuration
- Supprimer toute génération de règles

---

### 5.4 VIOL-NEG-04 : Permettre le contournement d'autorité

**Violation :** Bonding Brother permet à un produit d'accéder directement aux autorités.

**Exemples de violation :**
- Exposition d'une API directe vers Kind Mother ou Strong Father
- Mode "bypass" qui contourne Bonding Brother
- Endpoint qui permet d'appeler une autorité sans passer par BB

**Détection :**
- Analyse réseau : les autorités ne doivent être accessibles que via BB
- Audit des API exposées : aucune ne doit pointer vers une autorité
- Vérification qu'aucun produit ne peut contourner BB

**Correction :**
- Supprimer toute API directe vers les autorités
- Forcer tous les accès à passer par Bonding Brother

---

### 5.5 VIOL-NEG-05 : Modifier une décision d'autorité

**Violation :** Bonding Brother modifie, interprète, ou remplace une décision d'autorité.

**Exemples de violation :**
- Transformer un "refusé" en "accepté"
- Ajouter des permissions non accordées par Strong Father
- Supprimer des restrictions imposées par Kind Mother
- Interpréter une décision de manière différente

**Détection :**
- Comparaison automatisée entre décision reçue et décision transmise
- Vérification que la traduction ne modifie pas le sens de la décision
- Audit des filtrages : ils ne doivent pas modifier la décision

**Correction :**
- Transmettre fidèlement toute décision sans modification
- Séparer clairement traduction (format) et décision (sens)

---

### 5.6 VIOL-NEG-06 : Cacher l'origine d'une intention

**Violation :** Bonding Brother masque, modifie, ou omet l'origine d'une intention aux autorités.

**Exemples de violation :**
- Ne pas transmettre l'identité du produit
- Modifier l'identité de l'utilisateur
- Oublier des éléments du contexte
- Anonymiser les intentions

**Détection :**
- Audit des demandes transmises : toutes doivent contenir le contexte complet
- Vérification que l'origine est toujours présente
- Tests de traçabilité : l'origine doit être traçable

**Correction :**
- Transmettre intégralement le contexte à chaque demande
- Ne jamais filtrer ou modifier l'origine

---

## 6. Anti-patterns architecturaux

### 6.1 ANTI-ARCH-01 : Couche qui saute une étape

**Anti-pattern :** Une couche accède directement à une couche non adjacente.

**Exemple :**
- La Couche Produit accède directement à la Couche Autorité
- La Couche Traduction accède directement au journal

**Détection :**
- Analyse des dépendances entre composants
- Vérification que chaque couche n'accède qu'aux couches adjacentes

**Correction :**
- Réorganiser les appels pour respecter l'ordre des couches
- Utiliser les interfaces des couches adjacentes

---

### 6.2 ANTI-ARCH-02 : Partage d'état entre couches

**Anti-pattern :** Deux couches partagent un état mutable.

**Exemple :**
- La Couche Produit et la Couche Médiation partagent un cache
- La Couche Traduction modifie un état global

**Détection :**
- Audit des structures de données partagées
- Vérification que chaque couche a son propre état isolé

**Correction :**
- Isoler l'état de chaque couche
- Utiliser des interfaces immutables pour la communication

---

### 6.3 ANTI-ARCH-03 : Dépendance circulaire

**Anti-pattern :** Deux composants dépendent l'un de l'autre.

**Exemple :**
- ProductGateway dépend de FilterEngine, qui dépend de ProductGateway
- IntentTranslator dépend de ResponseTranslator, qui dépend de IntentTranslator

**Détection :**
- Analyse des dépendances : détection de cycles
- Vérification de la structure acyclique

**Correction :**
- Réorganiser les dépendances pour éliminer les cycles
- Introduire une abstraction commune si nécessaire

---

### 6.4 ANTI-ARCH-04 : Composant avec responsabilités multiples

**Anti-pattern :** Un composant assume plusieurs responsabilités non liées.

**Exemple :**
- Un composant traduit ET filtre ET journalise
- Un composant reçoit les intentions ET prend des décisions

**Détection :**
- Analyse de la responsabilité unique de chaque composant
- Vérification qu'aucun composant ne fait trop de choses

**Correction :**
- Séparer les responsabilités en composants distincts
- Respecter le principe de responsabilité unique

---

## 7. Anti-patterns comportementaux

### 7.1 ANTI-COMP-01 : Traduction avec effet de bord

**Anti-pattern :** La traduction modifie un état ou appelle une autorité.

**Exemple :**
- La traduction met à jour un cache
- La traduction interroge Strong Father pour enrichir les données

**Détection :**
- Vérification que les fonctions de traduction sont pures
- Tests unitaires : même entrée = même sortie

**Correction :**
- Rendre la traduction pure (sans effet de bord)
- Déplacer les effets de bord vers d'autres composants

---

### 7.2 ANTI-COMP-02 : Filtrage qui décide

**Anti-pattern :** Le filtrage prend une décision métier au lieu d'appliquer une règle.

**Exemple :**
- Le filtrage autorise ou refuse sans consulter une autorité
- Le filtrage valide des données métier

**Détection :**
- Vérification que le filtrage applique uniquement des règles définies
- Audit des règles de filtrage : aucune ne doit être une décision

**Correction :**
- Le filtrage applique des règles, ne prend pas de décision
- Déléguer les décisions aux autorités

---

### 7.3 ANTI-COMP-03 : Journalisation sélective

**Anti-pattern :** Certaines interactions ne sont pas journalisées.

**Exemple :**
- Les erreurs ne sont pas journalisées
- Certains types d'intentions sont omis du journal

**Détection :**
- Vérification que toutes les interactions sont journalisées
- Tests de couverture : 100% des interactions doivent être tracées

**Correction :**
- Journaliser systématiquement toutes les interactions
- Aucune exception à la journalisation

---

### 7.4 ANTI-COMP-04 : Retry avec modification

**Anti-pattern :** Un retry modifie l'intention au lieu de réessayer l'identique.

**Exemple :**
- Un retry change le contexte
- Un retry modifie le payload

**Détection :**
- Vérification que les retries sont identiques à l'intention originale
- Tests de retry : l'intention doit être préservée

**Correction :**
- Les retries doivent être identiques à l'intention originale
- Ne jamais modifier une intention lors d'un retry

---

## 8. Anti-patterns d'intégration

### 8.1 ANTI-INT-01 : Adaptation bidirectionnelle

**Anti-pattern :** Bonding Brother s'adapte aux produits au lieu de l'inverse.

**Exemple :**
- BB modifie son interface pour un produit spécifique
- BB supporte un format propriétaire d'un produit

**Détection :**
- Vérification que l'interface de BB est stable
- Audit des adaptations : aucune ne doit être spécifique à un produit

**Correction :**
- Les produits s'adaptent à BB, jamais l'inverse
- L'interface de BB reste stable

---

### 8.2 ANTI-INT-02 : Cache d'autorité

**Anti-pattern :** Bonding Brother maintient un cache des réponses d'autorité.

**Exemple :**
- Cache des permissions pour "performance"
- Cache des données pour éviter les appels répétés

**Détection :**
- Recherche de caches d'autorité
- Vérification qu'aucun cache ne stocke de vérité

**Correction :**
- Supprimer tout cache d'autorité
- Toujours interroger l'autorité pour la vérité

---

### 8.3 ANTI-INT-03 : Aggregation de réponses

**Anti-pattern :** Bonding Brother agrège des réponses de plusieurs autorités pour créer une réponse composite.

**Exemple :**
- BB combine une réponse de Kind Mother et Strong Father
- BB synthétise des données de plusieurs sources

**Détection :**
- Vérification qu'aucune réponse n'est agrégée
- Audit des réponses : chaque réponse doit être transmise individuellement

**Correction :**
- Transmettre chaque réponse individuellement
- Ne jamais agréger les réponses d'autorités

---

## 9. Mécanismes de détection

### 9.1 Détection statique (au build)

**Outils :**
- Analyse statique de code (détection de patterns interdits)
- Vérification des dépendances (détection de cycles)
- Audit des structures de données (détection de stockage de vérité)

**Fréquence :** À chaque build / CI

### 9.2 Détection dynamique (au runtime)

**Outils :**
- Comparaison décision reçue / transmise
- Vérification de traçabilité (origine toujours présente)
- Monitoring des violations (alertes en temps réel)

**Fréquence :** Temps réel

### 9.3 Détection par audit

**Outils :**
- Revue architecturale périodique
- Audit de sécurité
- Tests de non-régression

**Fréquence :** Mensuel / à chaque release

---

## 10. Processus de correction

### 10.1 Détection d'une violation

**Action immédiate :**
1. Arrêter le traitement si la violation est critique
2. Journaliser la violation avec tous les détails
3. Notifier les administrateurs

### 10.2 Analyse de la violation

**Étapes :**
1. Identifier la cause racine
2. Évaluer l'impact (données affectées, produits impactés)
3. Déterminer la correction nécessaire

### 10.3 Correction

**Processus :**
1. Corriger le code / l'architecture
2. Ajouter des tests pour prévenir la récurrence
3. Vérifier que la correction n'introduit pas d'autres violations
4. Déployer la correction

### 10.4 Prévention

**Actions :**
1. Mettre à jour ce document si une nouvelle violation est découverte
2. Ajouter des tests de détection
3. Documenter la leçon apprise

---

## 11. Liste de vérification

Cette liste peut être utilisée lors des revues de code et d'architecture :

- [ ] Aucun composant ne détient de vérité métier
- [ ] Aucun composant ne prend de décision métier
- [ ] Aucun composant ne crée de règle
- [ ] Aucun cache de données métier n'existe
- [ ] Toutes les décisions sont déléguées aux autorités
- [ ] Toutes les interactions sont journalisées
- [ ] Aucune couche n'accède à une couche non adjacente
- [ ] Aucune dépendance circulaire n'existe
- [ ] La traduction est pure (sans effet de bord)
- [ ] Le filtrage applique des règles, ne décide pas
- [ ] Aucune API directe vers les autorités n'est exposée
- [ ] Les décisions d'autorité sont transmises fidèlement
- [ ] L'origine des intentions est toujours transmise
- [ ] Aucune agrégation de réponses d'autorités

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut INTERDICTION**. Il établit les violations et anti-patterns que Bonding Brother ne doit jamais commettre, sous peine de remettre en question sa nature même.

Toute violation détectée est un défaut critique qui doit être corrigé immédiatement. Toute implémentation de Bonding Brother doit être vérifiée contre cette liste.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** INTERDICTION — Non négociable  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 10)
- Invariants et Garanties v1.0
