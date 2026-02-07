# KindMother — Interface & Contrat d'Intégration

## 1. Introduction

### Rôle de l'interface KindMother

L'interface KindMother définit le contrat conceptuel entre le moteur de données interne et les autres parties du système Miyukini Core System. Elle établit les règles d'interaction, les responsabilités de chaque partie, et les garanties fournies par KindMother.

Cette interface permet aux adaptateurs produits et aux produits eux-mêmes de s'intégrer avec KindMother sans connaître les détails d'implémentation de la persistance, de la synchronisation, ou de la gestion des instances.

### Pourquoi cette interface existe

L'interface KindMother existe pour plusieurs raisons fondamentales :

**Séparation des responsabilités :** Elle établit une frontière claire entre les responsabilités de KindMother (gestion de la persistance, synchronisation, cohérence) et celles des adaptateurs produits (traduction entre les types SPM et les opérations KindMother).

**Abstraction de la persistance :** Elle masque complètement les détails techniques de stockage (structure, schémas, requêtes) aux appelants, permettant à KindMother d'évoluer son implémentation sans impact sur les adaptateurs.

Cette abstraction respecte **LOI-1** (aucune dépendance externe critique) : en masquant les détails de persistance, KindMother garantit que les adaptateurs ne créent pas de dépendances externes critiques. La persistance est gérée localement par KindMother, sans nécessiter de services distants.

**Contrat stable :** Elle définit un contrat conceptuel stable qui permet aux adaptateurs de s'appuyer sur des garanties claires, tout en laissant à KindMother la liberté d'évolution interne (notamment pendant la phase v0.x où aucune compatibilité rétroactive n'est garantie).

**Centralisation des règles :** Elle centralise l'application des règles de permissions conceptuelles et de cohérence, évitant la duplication et les incohérences dans les adaptateurs.

### Positionnement par rapport à la documentation fondatrice

Cette documentation complète la documentation fondatrice de KindMother en se concentrant exclusivement sur l'interface contractuelle. Alors que la documentation fondatrice définit les concepts, l'architecture interne, et les responsabilités globales de KindMother, ce document définit précisément comment les autres parties du système doivent s'interface avec KindMother.

La documentation fondatrice répond à la question "Qu'est-ce que KindMother et comment fonctionne-t-il ?". Ce document répond à la question "Comment dois-je m'interface avec KindMother ?".

---

## 2. Qui peut appeler KindMother

### Produits

Les produits n'appellent **jamais directement** KindMother. Ils accèdent aux fonctionnalités de persistance uniquement via les modules SPM CMS, qui eux-mêmes utilisent des adaptateurs produits.

**Règle stricte :** Aucun produit ne doit avoir de dépendance directe vers KindMother. Toute tentative d'appel direct depuis un produit constitue une violation architecturale.

### Adaptateurs SPM

Les **adaptateurs produits** sont les seuls composants autorisés à appeler directement KindMother. Un adaptateur est une implémentation d'un trait SPM CMS (par exemple, `ContentManager`, `MediaManager`) qui traduit les opérations du module SPM en opérations KindMother.

**Responsabilités des adaptateurs :**
- Recevoir les demandes des modules SPM (opérations fonctionnelles)
- Traduire ces demandes en opérations KindMother (lecture, WriteIntent, synchronisation)
- Fournir le contexte nécessaire à KindMother (utilisateur, autorisations, instance)
- Retourner les résultats des modules SPM après traduction depuis KindMother

**Règle :** Chaque adaptateur est responsable de la traduction bidirectionnelle entre les types et opérations des modules SPM et les opérations conceptuelles de KindMother.

### Modules SPM

Les modules SPM CMS (Content, Hierarchy, Taxonomies, Media, Publication, Search) **ne connaissent pas** KindMother et ne peuvent pas l'appeler.

**Règle fondamentale :** Aucun module SPM ne doit avoir de référence, directe ou indirecte, vers KindMother. Les modules SPM exposent uniquement des traits fonctionnels purs, sans aucune notion de persistance, synchronisation, ou instance.

### Kernel

Le kernel est utilisé par KindMother (Id, Clock, Logger), mais KindMother n'est jamais appelé par le kernel.

**Règle :** Les dépendances sont strictement unidirectionnelles : Produit → Adaptateurs → KindMother → Kernel. Aucune dépendance inverse n'est autorisée.

### Violations architecturales

Les actions suivantes constituent des violations architecturales strictes :

- Un produit appelant directement KindMother
- Un module SPM ayant une référence vers KindMother
- Un adaptateur contournant KindMother pour accéder directement à la persistance
- Le kernel appelant KindMother
- Tout contournement des adaptateurs pour accéder à KindMother

---

## 3. Contexte d'appel

Toute opération sur KindMother nécessite la fourniture d'un contexte complet qui définit les conditions d'exécution de l'opération.

### Contexte utilisateur

Le contexte utilisateur identifie l'utilisateur effectuant l'opération. Il contient :

- **Identité de l'utilisateur :** Identifiant unique de l'utilisateur (fourni par le produit)
- **Métadonnées optionnelles :** Informations additionnelles sur l'utilisateur si nécessaire pour les règles de permissions (rôles, organisation, etc.)

**Responsabilité :** L'adaptateur fournit le contexte utilisateur à KindMother pour chaque opération. KindMother utilise ce contexte pour appliquer les règles de permissions conceptuelles.

### Contexte d'autorisation

Le contexte d'autorisation contient les informations nécessaires pour vérifier les permissions conceptuelles :

- **Règles de permissions :** Les règles définies par le produit (qui peut lire/écrire quelles entités selon quelles conditions)
- **Rôles et permissions :** Les rôles et permissions de l'utilisateur dans le contexte de l'opération
- **Contexte métier :** Informations additionnelles nécessaires aux règles (organisation, projet, ressource parente, etc.)

**Responsabilité :** Le produit définit les règles de permissions conceptuelles. L'adaptateur fournit ces règles à KindMother, qui les applique lors des opérations. KindMother ne définit aucune règle par défaut ; il exécute uniquement les règles fournies.

**Note :** KindMother gère les permissions conceptuelles (vérifications au niveau des données), pas l'authentification technique (tokens, sessions, OAuth). L'authentification reste du ressort du produit.

### Contexte d'instance

Le contexte d'instance identifie l'instance de base de données utilisée pour l'opération :

- **Type d'instance :** DB Mère ou DB Fille
- **Identité d'instance :** Identifiant unique de l'instance (généré par KindMother lors de la création)
- **Relation Mère-Fille :** Pour une DB Fille, référence vers la DB Mère associée

**Responsabilité :** KindMother gère l'identité des instances. L'adaptateur indique à KindMother quelle instance utiliser (généralement déterminée par la configuration du produit). En mode offline, KindMother utilise automatiquement la DB Fille locale.

### Contexte d'exécution

Le contexte d'exécution définit les conditions d'exécution de l'opération :

- **Mode online :** Connexion à la DB Mère disponible, opérations synchrones
- **Mode offline :** DB Fille autonome, opérations locales, synchronisation différée
  - Ce mode respecte **LOI-2** (le système accepte l'isolement comme état normal) : l'absence de connexion à la DB Mère n'est pas traitée comme une erreur, mais comme un état valide où les opérations continuent localement avec synchronisation différée.
- **État de synchronisation :** Pour une DB Fille, état de la dernière synchronisation avec la Mère

**Responsabilité :** KindMother détecte automatiquement le mode d'exécution (online/offline) et adapte son comportement. L'adaptateur peut interroger l'état de synchronisation mais ne contrôle pas directement le mode d'exécution.

### Fourniture du contexte

Le contexte est fourni par l'adaptateur à KindMother pour chaque opération. KindMother valide la complétude et la cohérence du contexte avant d'exécuter l'opération. Un contexte incomplet ou incohérent entraîne le rejet de l'opération avec une erreur de contexte.

---

## 4. Types d'opérations exposées par KindMother

KindMother expose quatre types d'opérations conceptuelles. Ces opérations sont définies de manière purement conceptuelle, sans signature technique ni détail d'implémentation.

### Lecture

Les opérations de lecture permettent de consulter les données stockées dans KindMother.

**Opérations conceptuelles :**
- Lecture d'une entité par identifiant
- Lecture de plusieurs entités selon des critères de filtrage
- Consultation de relations entre entités
- Consultation de métadonnées et d'état

**Caractéristiques :**
- Les opérations de lecture sont non-destructives
- Les résultats peuvent être filtrés selon des critères fournis par l'adaptateur
- La pagination est supportée conceptuellement (sans imposer de mécanisme technique)
- Les permissions conceptuelles sont vérifiées avant toute lecture

**Résultat :** Les données demandées sont retournées à l'adaptateur, ou une erreur si l'entité n'existe pas, si les permissions sont insuffisantes, ou si le contexte est invalide.

### Écriture (via WriteIntent)

Toutes les opérations d'écriture passent par le mécanisme de WriteIntent.

**WriteIntent :** Un WriteIntent représente une intention d'écriture avant validation et application. Il contient :
- Les données à modifier (création, modification, suppression)
- Le contexte complet (utilisateur, autorisations, instance)
- Les métadonnées de l'opération (horodatage, origine)

**Cycle de vie d'un WriteIntent :**
1. Création par l'adaptateur avec les données et le contexte
2. Validation par KindMother (permissions, cohérence)
3. Application si validation réussie, rejet sinon
4. En DB Fille, marquage pour synchronisation future

**Opérations conceptuelles :**
- Création d'entité (WriteIntent de création)
- Modification d'entité (WriteIntent de modification)
- Suppression d'entité (WriteIntent de suppression)
- Opérations atomiques multiples (plusieurs WriteIntent dans une transaction)

**Caractéristiques :**
- Toute écriture doit passer par WriteIntent ; aucune écriture directe n'est autorisée
- Les WriteIntent sont validés avant application (permissions, cohérence, références)
- En DB Mère, les WriteIntent validés sont appliqués immédiatement
- En DB Fille, les WriteIntent validés sont appliqués localement et marqués pour synchronisation

**Résultat :** Succès avec confirmation d'application, ou erreur avec raison du rejet (permission, cohérence, contexte invalide).

### Synchronisation

Les opérations de synchronisation gèrent la propagation des changements entre instances (DB Mère et DB Filles).

**Opérations conceptuelles :**
- Synchronisation Mère → Fille (propagation des changements de la Mère vers une Fille)
- Synchronisation Fille → Mère (remontée des changements d'une Fille vers la Mère)
- Détection de deltas (différences entre deux instances)
- Résolution de conflits (quand le même élément a été modifié dans les deux instances)

**Caractéristiques :**
- La synchronisation est bidirectionnelle conceptuellement, mais la DB Mère conserve l'autorité finale
- Seuls les deltas (différences) sont transférés, pas l'état complet
- Les conflits sont détectés automatiquement et résolus selon les règles définies
- La synchronisation peut être déclenchée automatiquement (périodique) ou manuellement
- Un appelant peut demander une synchronisation, mais ne peut pas en forcer le résultat

**Résultat :** Rapport de synchronisation avec nombre de changements propagés, conflits détectés et résolus, ou erreur si la synchronisation échoue.

### Inspection d'état

Les opérations d'inspection permettent de consulter l'état de KindMother et des instances sans modifier les données.

**Opérations conceptuelles :**
- Consultation de l'état de synchronisation (dernière sync, WriteIntent en attente)
- Consultation de l'état de connexion (online/offline, disponibilité de la Mère)
- Consultation de l'état des WriteIntent (en attente, appliqués, rejetés)
- Consultation des métadonnées d'instance (identité, type, relations)

**Caractéristiques :**
- Les opérations d'inspection sont en lecture seule
- Elles ne modifient pas l'état de KindMother
- Elles peuvent être utilisées pour le monitoring et le debugging
- Les informations retournées par les opérations d'inspection sont des vues contractuelles, non exhaustives, et peuvent évoluer sans préavis

**Résultat :** Informations sur l'état demandé, ou erreur si l'information n'est pas disponible.

---

## 5. Cycle de vie d'un appel

Tout appel à KindMother suit un cycle de vie standardisé en quatre étapes.

### Réception

KindMother reçoit la demande d'opération depuis l'adaptateur. La demande contient :
- Le type d'opération (lecture, WriteIntent, synchronisation, inspection)
- Les paramètres de l'opération (identifiants, données, critères)
- Le contexte complet (utilisateur, autorisations, instance, exécution)

**Validation initiale :** KindMother vérifie immédiatement que la demande est bien formée (présence des paramètres requis, format valide). Une demande mal formée est rejetée immédiatement sans traitement.

### Validation

KindMother valide la demande selon plusieurs critères :

**Validation du contexte :** Vérification que le contexte est complet et cohérent (utilisateur identifié, instance valide, mode d'exécution compatible).

**Validation des permissions :** Pour les opérations de lecture et d'écriture, vérification des permissions conceptuelles selon les règles fournies par le produit. L'opération est rejetée si les permissions sont insuffisantes.

**Validation de la cohérence :** Pour les WriteIntent, vérification des contraintes de cohérence :
- Références valides (les entités référencées existent)
- Intégrité référentielle (pas de référence orpheline)
- Règles métier définies par le produit (contraintes spécifiques)

**Validation de l'instance :** Vérification que l'instance est dans un état valide pour l'opération (pas de corruption, état de synchronisation cohérent).

Si une validation échoue, l'opération est rejetée avec une erreur explicite indiquant la raison du rejet.

### Exécution

Si toutes les validations réussissent, KindMother exécute l'opération :

**Pour une lecture :** KindMother résout l'instance (Mère ou Fille locale selon le contexte), lit les données depuis la persistance, et les retourne à l'adaptateur.

**Pour un WriteIntent :** KindMother applique le WriteIntent dans la persistance. En DB Mère, l'application est immédiate. En DB Fille, l'application est locale et le WriteIntent est marqué pour synchronisation future.

**Pour une synchronisation :** KindMother calcule les deltas, valide chaque delta, détecte et résout les conflits, puis applique les changements à l'instance cible.

**Pour une inspection :** KindMother consulte l'état interne et retourne les informations demandées.

**Garanties d'exécution :** KindMother garantit que les opérations sont atomiques (tout ou rien) et maintiennent la cohérence locale. En cas d'échec pendant l'exécution, l'opération est annulée et l'état reste cohérent.

### Résultat

KindMother retourne le résultat de l'opération à l'adaptateur :

**Succès :** Le résultat contient les données demandées (pour une lecture), la confirmation d'application (pour un WriteIntent), ou le rapport d'opération (pour une synchronisation ou inspection).

**Échec :** Le résultat contient une erreur explicite avec :
- Le type d'erreur (permission, cohérence, contexte, instance, synchronisation)
- La raison de l'échec (message descriptif)
- Les informations de contexte nécessaires pour comprendre l'erreur

L'adaptateur est responsable de traduire le résultat KindMother en résultat du module SPM, puis de le retourner au produit.

---

## 6. Gestion des résultats

KindMother retourne différents types de résultats selon le succès ou l'échec de l'opération, et selon l'état du système.

### Succès

Un résultat de succès indique que l'opération a été exécutée avec succès.

**Pour une lecture :** Les données demandées sont retournées. Si aucune donnée ne correspond aux critères, un résultat vide est retourné (ce n'est pas une erreur).

**Pour un WriteIntent :** Confirmation que le WriteIntent a été validé et appliqué. En DB Mère, l'application est immédiate et définitive. En DB Fille, l'application est locale et le WriteIntent est marqué pour synchronisation.

**Pour une synchronisation :** Rapport de synchronisation indiquant le nombre de changements propagés, les conflits détectés et résolus, et l'état final de la synchronisation.

**Pour une inspection :** Les informations d'état demandées sont retournées.

### Échec fonctionnel

Un échec fonctionnel indique que l'opération a été rejetée pour une raison fonctionnelle (pas une erreur technique).

**Entité introuvable :** L'entité demandée n'existe pas dans l'instance. Ce n'est pas une erreur de permission, mais une absence de données.

**Contrainte violée :** Une contrainte de cohérence a été violée (référence invalide, règle métier non respectée). Le WriteIntent est rejeté.

**Règle métier non respectée :** Une règle métier définie par le produit n'est pas respectée. Le WriteIntent est rejeté avec indication de la règle violée.

**État invalide :** L'entité ou l'instance est dans un état qui ne permet pas l'opération demandée (ex. modification d'une entité verrouillée).

### Échec de permission

Un échec de permission indique que l'utilisateur n'a pas les permissions nécessaires pour effectuer l'opération.

**Permission de lecture refusée :** L'utilisateur ne peut pas lire l'entité demandée selon les règles de permissions conceptuelles.

**Permission d'écriture refusée :** L'utilisateur ne peut pas modifier l'entité selon les règles de permissions conceptuelles.

**Contexte d'autorisation insuffisant :** Le contexte d'autorisation fourni ne contient pas les informations nécessaires pour vérifier les permissions.

**Règle de permission non satisfaite :** Les règles de permissions conceptuelles définies par le produit ne sont pas satisfaites pour cette opération.

### Échec de cohérence

Un échec de cohérence indique qu'une contrainte de cohérence a été violée.

**Référence invalide :** Une référence vers une entité qui n'existe pas ou qui n'est pas accessible.

**Intégrité référentielle :** Tentative de supprimer une entité référencée par d'autres entités, ou modification qui créerait une référence orpheline.

**Cohérence transactionnelle :** Échec d'une transaction atomique (plusieurs WriteIntent) où une partie a échoué, entraînant l'annulation de toute la transaction.

**État incohérent détecté :** KindMother a détecté un état incohérent dans l'instance (corruption, désynchronisation).

### États transitoires

Certains résultats indiquent un état transitoire où l'opération n'est pas encore complète ou est en attente.

**Offline — opération en attente :** En mode offline, certaines opérations peuvent être acceptées mais mises en attente de synchronisation. Le résultat indique que l'opération a été acceptée localement mais nécessite une synchronisation pour être effective globalement.

**WriteIntent en attente de synchronisation :** Un WriteIntent a été appliqué localement en DB Fille mais n'a pas encore été synchronisé avec la DB Mère. Le résultat indique l'état d'attente.

**Conflits détectés :** Lors d'une synchronisation, des conflits ont été détectés et sont en cours de résolution. Le résultat indique les conflits et leur état de résolution.

**Synchronisation partielle :** Une synchronisation a été partiellement effectuée (certains changements appliqués, d'autres rejetés). Le résultat indique le détail de la synchronisation partielle.

---

## 7. Gestion des erreurs

KindMother définit plusieurs types d'erreurs conceptuelles et garantit certains comportements, mais ne garantit pas tout.

### Types d'erreurs conceptuelles

**Erreur de permission :** L'utilisateur n'a pas les permissions nécessaires pour l'opération. L'erreur indique quelle permission est manquante et pourquoi.

**Erreur de cohérence :** Une contrainte de cohérence a été violée. L'erreur indique quelle contrainte a été violée et comment.

**Erreur d'instance :** L'instance est dans un état invalide (corruption, désynchronisation, indisponibilité). L'erreur indique l'état de l'instance et la raison de l'invalidité.

**Erreur de synchronisation :** Une erreur s'est produite lors d'une synchronisation (connexion perdue, conflit non résolu, delta invalide). L'erreur indique la nature du problème de synchronisation.

**Erreur de contexte :** Le contexte fourni est incomplet, incohérent, ou invalide. L'erreur indique ce qui manque ou ce qui est incorrect dans le contexte.

**Erreur fonctionnelle :** Une erreur fonctionnelle non catégorisée (entité introuvable, opération non supportée, etc.). L'erreur indique la nature du problème fonctionnel.

### Ce que KindMother garantit

**Cohérence locale :** KindMother garantit que toutes les opérations maintiennent la cohérence locale de l'instance. Aucune opération ne laisse l'instance dans un état incohérent.

**Validation des permissions :** KindMother garantit que toutes les opérations de lecture et d'écriture sont précédées d'une vérification des permissions conceptuelles selon les règles fournies par le produit.

**Traçabilité :** KindMother garantit que toutes les opérations sont tracées (utilisateur, horodatage, type d'opération) pour permettre l'audit et le debugging.

**Atomicité :** KindMother garantit que les opérations sont atomiques (tout ou rien). En cas d'échec partiel, l'opération est entièrement annulée.

**Isolation :** KindMother garantit que les opérations concurrentes ne se corrompent pas mutuellement (isolation transactionnelle).

**Messages d'erreur explicites :** KindMother garantit que toutes les erreurs sont accompagnées d'un message explicite indiquant la raison de l'échec.

### Ce que KindMother ne garantit pas

**Compatibilité rétroactive (v0.x) :** Pendant la phase v0.x (version interne), KindMother ne garantit aucune compatibilité rétroactive. L'interface peut évoluer de manière non compatible entre versions.

**Latence :** KindMother ne garantit aucune latence spécifique pour les opérations. Les performances dépendent de l'implémentation et de l'état de l'instance.

**Disponibilité réseau :** KindMother ne garantit pas la disponibilité de la connexion réseau pour les synchronisations. Les opérations en mode offline sont acceptées mais peuvent échouer lors de la synchronisation si la connexion n'est pas disponible.

**Résolution automatique de conflits :** KindMother détecte les conflits mais ne garantit pas leur résolution automatique. Certains conflits peuvent nécessiter une intervention manuelle ou des règles spécifiques définies par le produit.

**Disponibilité de l'instance :** KindMother ne garantit pas que l'instance sera toujours disponible. En cas de corruption, de maintenance, ou d'indisponibilité, les opérations peuvent échouer.

**Ordre d'application des WriteIntent :** L'ordre logique des WriteIntent est préservé localement, mais leur ordre d'application global dépend de la synchronisation et de la validation par la DB Mère.

**Résolution de tous les conflits :** KindMother ne garantit pas que tous les conflits peuvent être résolus automatiquement. Certains conflits peuvent nécessiter une résolution manuelle.

---

## 8. Ce que l'appelant ne peut PAS faire

KindMother impose des restrictions strictes sur ce que les appelants peuvent faire. Ces restrictions garantissent l'intégrité du système et l'abstraction complète de la persistance.

### Accéder au stockage

**Interdiction absolue :** Aucun appelant ne peut accéder directement au stockage utilisé par KindMother (structure, schémas, requêtes, fichiers).

**Raison :** L'abstraction complète de la persistance est un principe fondamental. Toute violation de cette abstraction compromet l'évolution future de KindMother et crée des dépendances indésirables.

**Conséquence :** Toute tentative d'accès direct au stockage (par exemple, exécution de requêtes SQL, lecture de fichiers de base de données, accès aux schémas) constitue une violation architecturale majeure.

### Forcer une écriture

**Interdiction :** Aucun appelant ne peut forcer une écriture qui contourne le mécanisme de WriteIntent et sa validation.

**Raison :** Le mécanisme de WriteIntent garantit la validation des permissions et de la cohérence avant toute modification. Contourner ce mécanisme compromet l'intégrité des données.

**Conséquence :** Toute écriture doit passer par WriteIntent. Aucune écriture directe, même "pour tester" ou "pour optimiser", n'est autorisée.

### Contourner les permissions

**Interdiction :** Aucun appelant ne peut contourner les vérifications de permissions conceptuelles.

**Raison :** Les permissions conceptuelles garantissent la sécurité et la cohérence des accès. Tout contournement compromet la sécurité du système.

**Conséquence :** Toute tentative de bypass des permissions (par exemple, utilisation d'un contexte utilisateur différent, appel direct à la persistance, modification des règles de permissions pour une opération spécifique) constitue une violation de sécurité.

### Modifier l'instance

**Interdiction :** Aucun appelant ne peut modifier l'identité, le type, ou les métadonnées d'une instance (DB Mère ou DB Fille).

**Raison :** L'identité et le type d'instance sont gérés exclusivement par KindMother. Toute modification externe compromet l'intégrité de la gestion des instances.

**Conséquence :** Seul KindMother peut créer, modifier, ou supprimer des instances. Les appelants peuvent uniquement utiliser des instances existantes dans le contexte de leurs opérations.

### Contourner la synchronisation

**Interdiction :** Aucun appelant ne peut contourner le mécanisme de synchronisation pour accéder directement aux données d'une autre instance.

**Raison :** La synchronisation garantit la cohérence globale entre instances. Tout contournement compromet cette cohérence.

**Conséquence :** Toute tentative d'accès direct inter-instances (par exemple, lecture directe depuis la DB Mère depuis une DB Fille, ou écriture directe dans une autre instance) constitue une violation architecturale.

### Modifier les règles de permissions pour une opération

**Interdiction :** Aucun appelant ne peut modifier temporairement les règles de permissions pour une opération spécifique.

**Raison :** Les règles de permissions sont définies par le produit et appliquées de manière cohérente par KindMother. Toute modification temporaire compromet la sécurité et la traçabilité.

**Conséquence :** Les règles de permissions doivent être modifiées au niveau du produit, pas au niveau d'une opération individuelle. KindMother applique toujours les règles fournies dans le contexte d'autorisation.

### Accéder aux métadonnées internes

**Interdiction :** Aucun appelant ne peut accéder aux métadonnées internes de KindMother (structure interne, états de synchronisation détaillés, logs internes) sauf via les opérations d'inspection d'état explicitement prévues.

**Raison :** Les métadonnées internes sont un détail d'implémentation. L'accès direct crée des dépendances indésirables.

**Conséquence :** Seules les opérations d'inspection d'état documentées permettent d'accéder à certaines informations sur l'état de KindMother. Tout autre accès est interdit.

### Exécuter des opérations en mode "bypass"

**Interdiction :** Aucun appelant ne peut demander à KindMother d'exécuter une opération en mode "bypass" qui contourne les validations.

**Raison :** Les validations (permissions, cohérence, contexte) sont essentielles à l'intégrité du système. Aucun mode de contournement n'existe.

**Conséquence :** Toute opération doit passer par toutes les validations. Aucune option, flag, ou mode spécial ne permet de contourner les validations.

---

## Conclusion

Cette documentation définit le contrat d'intégration entre KindMother et les autres parties du système Miyukini Core System. Elle établit les règles d'interaction, les responsabilités de chaque partie, et les garanties fournies par KindMother.

**Points clés :**
- Seuls les adaptateurs produits peuvent appeler KindMother directement
- Toute opération nécessite un contexte complet (utilisateur, autorisations, instance, exécution)
- Toutes les écritures passent par WriteIntent avec validation
- KindMother garantit la cohérence locale, la validation des permissions, et la traçabilité
- Aucun accès direct au stockage, aucune écriture forcée, aucun contournement des permissions n'est autorisé

Cette interface permet aux adaptateurs de s'intégrer avec KindMother de manière fiable et prévisible, tout en laissant à KindMother la liberté d'évolution interne pendant la phase v0.x.

---

**Document créé le :** 2026-01-24  
**Version :** 1.0  
**Statut :** Documentation contractuelle validée  
**Référence :** Complète la documentation fondatrice "Miyukini Core System — KindMother Documentation Fondatrice"
