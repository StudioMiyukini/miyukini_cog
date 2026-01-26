# KindMother — CoreDataAPI (Surface d'Appel Conceptuelle)

## 1. Introduction

### Rôle de la CoreDataAPI

La CoreDataAPI est la surface d'appel minimale et conceptuelle exposée par KindMother. Elle définit l'ensemble des opérations que les adaptateurs produits peuvent invoquer pour interagir avec le moteur de données, sans exposer aucun détail d'implémentation, aucune structure technique, ni aucun mécanisme de persistance.

Cette API constitue le contrat unique et exclusif entre KindMother et ses appelants. Toute interaction avec les données persistées doit transiter par cette surface, garantissant ainsi l'abstraction complète de la persistance et la centralisation de la gestion des données.

### Pourquoi une surface minimale est essentielle

Une surface d'appel minimale est fondamentale pour plusieurs raisons :

**Simplicité d'intégration :** Un nombre réduit d'opérations bien définies facilite la compréhension et l'intégration par les adaptateurs produits. Chaque opération a une intention claire et non ambiguë, réduisant les erreurs d'utilisation et les malentendus.

**Maintenabilité :** Une surface minimale réduit la surface d'évolution et de maintenance. Les changements internes de KindMother n'impactent pas les adaptateurs tant que le contrat de la CoreDataAPI reste stable.

**Évolution contrôlée :** En limitant le nombre d'opérations exposées, KindMother conserve la liberté d'évolution interne sans compromettre la stabilité du contrat. Les optimisations, changements de stratégie de synchronisation, ou modifications de la persistance restent transparents pour les appelants.

**Cohérence systémique :** Une surface unique garantit que toutes les opérations passent par les mêmes validations, vérifications de permissions, et mécanismes de cohérence. Aucun contournement n'est possible, préservant l'intégrité du système.

**Clarté des responsabilités :** Chaque opération a une responsabilité précise et non redondante. Cette clarté facilite la compréhension du comportement attendu et évite les ambiguïtés sur le choix de l'opération à utiliser.

---

## 2. Principes de conception de la CoreDataAPI

La CoreDataAPI est conçue selon cinq principes fondamentaux qui guident toutes les décisions de conception et d'évolution.

### Minimalisme

La CoreDataAPI expose uniquement les opérations strictement nécessaires pour couvrir tous les profils d'usage identifiés. Aucune opération n'est ajoutée "au cas où" ou pour des cas d'usage hypothétiques. Chaque opération doit être justifiable par au moins un profil d'usage réel et documenté.

Ce principe garantit que la surface reste compréhensible et maintenable, tout en évitant la prolifération d'opérations redondantes ou spécialisées qui complexifieraient inutilement l'API.

### Explicitness

Chaque opération de la CoreDataAPI a une intention claire, non ambiguë, et documentée. Il ne doit pas être possible de se demander quelle opération utiliser pour un besoin donné : la réponse doit être évidente à partir de l'intention de l'opération.

L'explicitness s'applique également aux résultats : chaque type de résultat possible est documenté, et les conditions de retour de chaque résultat sont clairement définies. Aucune ambiguïté ne doit subsister sur le comportement attendu d'une opération.

### Autorité

KindMother conserve l'autorité exclusive sur toutes les opérations de données. Aucune opération ne permet à l'appelant de contourner les validations, les vérifications de permissions, ou les mécanismes de cohérence. L'appelant peut demander une opération, mais KindMother décide de son exécution selon ses propres règles.

Cette autorité garantit que toutes les opérations respectent les contraintes de sécurité, de cohérence, et d'intégrité, indépendamment de l'intention de l'appelant. Aucun mode "bypass" ou "force" n'existe.

### Offline-first

Toutes les opérations de la CoreDataAPI fonctionnent en mode offline. Une DB Fille peut exécuter toutes les opérations (lecture, écriture, inspection) sans connexion à la DB Mère. Les opérations de synchronisation sont conçues pour gérer les périodes de déconnexion et de reconnexion de manière transparente.

Ce principe garantit que les applications peuvent fonctionner de manière autonome, avec synchronisation différée, sans compromettre les fonctionnalités de base. L'offline-first n'est pas une option, mais une caractéristique fondamentale de toutes les opérations.

**Conformité LOI-1 et LOI-2 :** Ce principe respecte **LOI-1** (aucune dépendance externe critique) : toutes les opérations fonctionnent sans appel externe obligatoire, garantissant que le système peut démarrer et fonctionner sans connexion réseau. Il respecte également **LOI-2** (le système accepte l'isolement comme état normal) : l'absence de connexion à la DB Mère n'est pas traitée comme une erreur, mais comme un état valide où les opérations continuent localement avec synchronisation différée.

### Absence de bypass

Aucune opération de la CoreDataAPI ne permet de contourner les mécanismes de validation, de permission, ou de cohérence. Il n'existe aucun mode spécial, aucune option, aucun flag qui permettrait d'exécuter une opération sans passer par toutes les validations requises.

Ce principe garantit l'intégrité du système en empêchant toute tentative de contournement, même pour des cas d'usage légitimes comme le debugging ou les migrations. Toute opération doit respecter les mêmes règles, sans exception.

---

## 3. Vue d'ensemble des opérations

La CoreDataAPI expose exactement dix opérations, organisées en quatre catégories fonctionnelles. Cette liste est exhaustive : aucune autre opération n'est exposée par KindMother.

### Opérations de lecture

Les opérations de lecture permettent de consulter les données stockées dans KindMother sans les modifier.

**read :** Lecture d'une entité unique identifiée par son identifiant. Retourne l'entité complète si elle existe et si l'utilisateur a les permissions de lecture.

**list :** Lecture d'une collection d'entités selon des critères de filtrage simples. Retourne une liste d'entités correspondant aux critères, avec support conceptuel de pagination.

**query :** Consultation de données selon des critères de recherche complexes, incluant des filtres multiples, des relations, et des agrégations. Retourne les résultats correspondant à la requête.

### Opérations d'écriture

Les opérations d'écriture permettent de modifier les données via le mécanisme de WriteIntent.

**submitWriteIntent :** Soumission d'une intention d'écriture unique (création, modification, ou suppression d'une entité). Le WriteIntent est validé, puis appliqué si la validation réussit.

**submitBatchWriteIntent :** Soumission d'un ensemble d'intentions d'écriture à appliquer de manière atomique. Toutes les intentions sont validées ensemble, puis toutes appliquées ou toutes rejetées (transaction atomique).

### Opérations de synchronisation

Les opérations de synchronisation gèrent la propagation des changements entre instances (DB Mère et DB Filles).

**sync :** Exécution immédiate d'une synchronisation entre deux instances. Calcule les deltas, détecte et résout les conflits, puis applique les changements. Retourne un rapport détaillé de la synchronisation.

**requestSync :** Demande de synchronisation différée. Enregistre la demande de synchronisation qui sera exécutée ultérieurement selon la stratégie de KindMother. Retourne une confirmation de la demande, pas le résultat de la synchronisation.

### Opérations d'inspection

Les opérations d'inspection permettent de consulter l'état de KindMother et des instances sans modifier les données.

**getStatus :** Consultation de l'état général de KindMother et de l'instance (disponibilité, mode online/offline, santé de l'instance).

**getSyncState :** Consultation de l'état de synchronisation d'une instance (dernière synchronisation, état de la connexion, métriques de synchronisation).

**getPendingWriteIntents :** Consultation de la liste des WriteIntent en attente de synchronisation pour une instance donnée (état, nombre, métadonnées).

---

## 4. Opérations de lecture

Les opérations de lecture sont non-destructives et permettent de consulter les données stockées dans KindMother. Toutes les opérations de lecture vérifient les permissions conceptuelles avant de retourner les données.

### read

**Intention :** Lire une entité unique identifiée par son identifiant. Cette opération est conçue pour les accès directs à une entité spécifique dont l'identifiant est connu.

**Responsabilités :** KindMother vérifie les permissions de lecture de l'utilisateur pour l'entité demandée, résout l'instance contenant l'entité (Mère ou Fille locale selon le contexte), lit l'entité depuis la persistance, et retourne l'entité complète à l'appelant.

**Contexte requis :** L'opération nécessite un contexte utilisateur (identité de l'utilisateur), un contexte d'autorisation (règles de permissions), un contexte d'instance (instance à interroger), et l'identifiant de l'entité à lire.

**Garanties :** KindMother garantit que les permissions sont vérifiées avant la lecture, que l'entité retournée est cohérente avec l'état de la persistance au moment de la lecture, et que l'opération est non-destructive (aucune modification des données).

**Limites :** L'opération ne retourne qu'une seule entité. Si l'entité n'existe pas, un résultat indiquant l'absence est retourné (ce n'est pas une erreur). L'opération ne garantit pas de latence spécifique. L'opération ne retourne pas les relations de l'entité (seulement l'entité elle-même, sauf si les relations font partie de la structure de l'entité selon le contrat du module SPM).

### list

**Intention :** Lire une collection d'entités selon des critères de filtrage simples. Cette opération est conçue pour les listages avec filtres basiques (égalité, comparaison simple).

**Responsabilités :** KindMother vérifie les permissions de lecture de l'utilisateur pour le type d'entité demandé, applique les critères de filtrage fournis, filtre les résultats selon les permissions de l'utilisateur (seulement les entités accessibles), et retourne la liste des entités correspondantes avec support conceptuel de pagination.

**Contexte requis :** L'opération nécessite un contexte utilisateur, un contexte d'autorisation, un contexte d'instance, le type d'entité à lister, les critères de filtrage (optionnels), et les paramètres de pagination (optionnels).

**Garanties :** KindMother garantit que seules les entités accessibles selon les permissions de l'utilisateur sont retournées, que les critères de filtrage sont appliqués de manière cohérente, et que la pagination respecte les limites conceptuelles (sans imposer de mécanisme technique spécifique).

**Limites :** L'opération supporte uniquement des critères de filtrage simples. Les recherches complexes nécessitent l'opération query. L'opération ne garantit pas l'ordre des résultats (sauf si un ordre est spécifié dans les critères). L'opération ne garantit pas de latence spécifique, notamment pour les grandes collections.

### query

**Intention :** Consulter des données selon des critères de recherche complexes. Cette opération est conçue pour les recherches avancées avec filtres multiples, relations, et agrégations.

**Responsabilités :** KindMother vérifie les permissions de lecture de l'utilisateur pour les types d'entités concernés, interprète la requête complexe fournie, applique les filtres, relations, et agrégations, filtre les résultats selon les permissions, et retourne les résultats correspondant à la requête.

**Contexte requis :** L'opération nécessite un contexte utilisateur, un contexte d'autorisation, un contexte d'instance, et une description de la requête complexe (filtres multiples, relations à inclure, agrégations à calculer, paramètres de pagination).

**Garanties :** KindMother garantit que seuls les résultats accessibles selon les permissions sont retournés, que la requête est interprétée de manière cohérente, et que les résultats respectent les contraintes de la requête.

**Limites :** L'opération ne garantit pas la performance pour des requêtes très complexes ou sur de très grandes quantités de données. La structure et les capacités de la requête sont volontairement limitées et définies par KindMother. Aucune expressivité équivalente à un langage de requête généraliste n'est garantie. L'opération ne garantit pas de latence spécifique.

---

## 5. Opérations d'écriture

Les opérations d'écriture permettent de modifier les données via le mécanisme de WriteIntent. Toute modification doit passer par une de ces deux opérations ; aucune écriture directe n'est possible.

### submitWriteIntent

**Intention :** Soumettre une intention d'écriture unique pour validation et application. Le WriteIntent représente une demande de création, modification, ou suppression d'une entité, avec toutes les données et le contexte nécessaires.

**Responsabilités :** KindMother reçoit le WriteIntent avec les données à modifier et le contexte complet, valide les permissions d'écriture de l'utilisateur, valide la cohérence (références valides, contraintes respectées, règles métier), applique le WriteIntent dans la persistance si la validation réussit, et marque le WriteIntent pour synchronisation future si l'instance est une DB Fille.

**Contexte requis :** L'opération nécessite un contexte utilisateur, un contexte d'autorisation, un contexte d'instance, le WriteIntent contenant les données à modifier (type d'opération : création, modification, suppression), et les métadonnées de l'opération (horodatage, origine).

**Garanties :** KindMother garantit que le WriteIntent est entièrement validé avant application, que l'application est atomique (tout ou rien), que la cohérence locale est maintenue après application, et que le WriteIntent est tracé pour audit et synchronisation.

**Limites :** L'opération ne garantit pas l'ordre d'application global en DB Fille (l'ordre local est préservé, mais l'ordre global dépend de la synchronisation avec la DB Mère). L'opération ne garantit pas que le WriteIntent sera accepté par la DB Mère lors de la synchronisation (la validation finale se fait lors de la synchronisation). L'opération ne garantit pas de latence spécifique.

### submitBatchWriteIntent

**Intention :** Soumettre un ensemble d'intentions d'écriture à appliquer de manière atomique. Toutes les intentions sont validées ensemble, puis toutes appliquées ou toutes rejetées (transaction atomique).

**Responsabilités :** KindMother reçoit l'ensemble des WriteIntent avec le contexte complet, valide toutes les permissions pour tous les WriteIntent, valide la cohérence globale de l'ensemble (références croisées, contraintes globales), applique tous les WriteIntent de manière atomique si toutes les validations réussissent, ou rejette l'ensemble complet si une validation échoue.

**Contexte requis :** L'opération nécessite un contexte utilisateur, un contexte d'autorisation, un contexte d'instance, l'ensemble des WriteIntent à appliquer atomiquement, et les métadonnées de l'opération batch.

**Garanties :** KindMother garantit que tous les WriteIntent sont validés ensemble avant application, que l'application est atomique (tous appliqués ou tous rejetés), que la cohérence locale est maintenue après application, et que l'ordre d'application des WriteIntent dans le batch est préservé.

**Limites :** L'opération ne garantit pas l'ordre d'application global en DB Fille (comme pour submitWriteIntent). L'opération ne garantit pas que tous les WriteIntent seront acceptés par la DB Mère lors de la synchronisation. L'opération ne garantit pas de latence spécifique, et la latence peut être plus élevée que pour un WriteIntent unique en raison de la validation globale.

---

## 6. Opérations de synchronisation

Les opérations de synchronisation gèrent la propagation des changements entre instances (DB Mère et DB Filles). Elles calculent les deltas, détectent et résolvent les conflits, et appliquent les changements de manière cohérente.

### sync

**Intention :** Exécuter immédiatement une synchronisation complète entre deux instances. Cette opération effectue le calcul des deltas, la détection et résolution des conflits, et l'application des changements dans l'instance cible.

**Responsabilités :** KindMother identifie l'instance source et l'instance cible, calcule les deltas depuis le dernier point de synchronisation, valide chaque delta selon les permissions et la cohérence, détecte les conflits (modifications concurrentes du même élément), résout les conflits selon les règles définies, applique les deltas validés à l'instance cible de manière atomique, et met à jour le point de synchronisation.

**Contexte requis :** L'opération nécessite un contexte d'instance source, un contexte d'instance cible, les règles de résolution de conflits (définies par le produit), et le contexte d'autorisation pour valider les deltas.

**Garanties :** KindMother garantit que seuls les deltas (différences) sont transférés (pas l'état complet), que tous les conflits sont détectés, que la cohérence est maintenue après synchronisation, et que le point de synchronisation est mis à jour pour les prochaines synchronisations.

**Limites :** L'opération ne garantit pas la résolution automatique de tous les conflits (certains conflits peuvent nécessiter une intervention manuelle ou des règles spécifiques). L'opération ne garantit pas la disponibilité de la connexion réseau (la synchronisation échoue si la connexion n'est pas disponible). L'opération ne garantit pas de latence spécifique, et la latence peut être élevée pour de grandes quantités de changements.

### requestSync

**Intention :** Demander une synchronisation différée qui sera exécutée ultérieurement selon la stratégie de KindMother. Cette opération enregistre la demande de synchronisation sans l'exécuter immédiatement.

**Responsabilités :** KindMother reçoit la demande de synchronisation avec l'instance source et l'instance cible, enregistre la demande dans la queue de synchronisation, et retourne une confirmation de l'enregistrement de la demande. La synchronisation sera exécutée ultérieurement selon la stratégie de KindMother (périodique, événementielle, selon la disponibilité réseau).

**Contexte requis :** L'opération nécessite un contexte d'instance source, un contexte d'instance cible, et les règles de résolution de conflits (pour utilisation lors de l'exécution future).

**Garanties :** KindMother garantit que la demande est enregistrée de manière fiable, que la demande sera traitée ultérieurement (sans garantie de délai), et que le résultat de la synchronisation sera disponible via getSyncState une fois exécutée.

**Limites :** L'opération ne garantit pas le délai d'exécution de la synchronisation (elle sera exécutée selon la stratégie de KindMother). L'opération ne retourne pas le résultat de la synchronisation (seulement la confirmation de la demande). L'opération ne garantit pas que la synchronisation réussira (elle peut échouer lors de l'exécution future pour les mêmes raisons qu'une synchronisation immédiate).

---

## 7. Opérations d'inspection

Les opérations d'inspection permettent de consulter l'état de KindMother et des instances sans modifier les données. Ces opérations sont en lecture seule et retournent des vues contractuelles de l'état interne.

### getStatus

**Intention :** Consulter l'état général de KindMother et de l'instance utilisée. Cette opération fournit des informations sur la disponibilité, le mode de fonctionnement, et la santé de l'instance.

**Responsabilités :** KindMother consulte l'état interne de l'instance, vérifie la disponibilité et la santé, détermine le mode de fonctionnement (online/offline), et retourne un résumé de l'état général.

**Contexte requis :** L'opération nécessite un contexte d'instance (pour déterminer quelle instance interroger). Le contexte utilisateur et d'autorisation peuvent être optionnels selon l'implémentation, mais certaines informations peuvent être filtrées selon les permissions.

**Garanties :** KindMother garantit que l'opération est en lecture seule (aucune modification de l'état), que les informations retournées reflètent l'état actuel au moment de l'appel, et que les informations sont cohérentes avec l'état réel de l'instance.

**Limites :** L'opération retourne une vue contractuelle non exhaustive de l'état. Certaines informations internes ne sont pas exposées. La structure des informations retournées peut évoluer sans préavis (mais les informations essentielles restent stables). L'opération ne garantit pas de latence spécifique.

### getSyncState

**Intention :** Consulter l'état de synchronisation d'une instance. Cette opération fournit des informations sur la dernière synchronisation, l'état de la connexion, et les métriques de synchronisation.

**Responsabilités :** KindMother consulte l'historique de synchronisation de l'instance, vérifie l'état de la connexion avec la DB Mère (si applicable), calcule les métriques de synchronisation (nombre de WriteIntent en attente, dernière synchronisation réussie, etc.), et retourne un rapport de l'état de synchronisation.

**Contexte requis :** L'opération nécessite un contexte d'instance (pour déterminer quelle instance interroger). Le contexte utilisateur et d'autorisation peuvent être optionnels.

**Garanties :** KindMother garantit que l'opération est en lecture seule, que les informations retournées reflètent l'état actuel de la synchronisation, et que les métriques sont calculées de manière cohérente.

**Limites :** L'opération retourne une vue contractuelle non exhaustive. Certaines métriques détaillées peuvent ne pas être exposées. La structure des informations peut évoluer sans préavis. L'opération ne garantit pas de latence spécifique.

### getPendingWriteIntents

**Intention :** Consulter la liste des WriteIntent en attente de synchronisation pour une instance donnée. Cette opération permet de connaître l'état des modifications locales qui n'ont pas encore été synchronisées avec la DB Mère.

**Responsabilités :** KindMother consulte la liste des WriteIntent appliqués localement mais non encore synchronisés, récupère les métadonnées de chaque WriteIntent (type, horodatage, état), et retourne la liste des WriteIntent en attente avec leurs métadonnées.

**Contexte requis :** L'opération nécessite un contexte d'instance (pour déterminer quelle instance interroger). Le contexte utilisateur et d'autorisation peuvent être optionnels, mais certaines informations peuvent être filtrées selon les permissions.

**Garanties :** KindMother garantit que l'opération est en lecture seule, que la liste retournée contient tous les WriteIntent en attente de synchronisation (selon les permissions de l'utilisateur), et que les métadonnées sont exactes.

**Limites :** L'opération ne retourne que les WriteIntent en attente, pas ceux déjà synchronisés ou rejetés. La liste peut être très longue pour des instances avec beaucoup de modifications locales. L'opération ne garantit pas de latence spécifique, et la latence peut être élevée pour de grandes listes. La structure des métadonnées peut évoluer sans préavis. Cette opération est destinée à l'observation et au diagnostic, pas à piloter des logiques métier.

---

## 8. Résultats et états possibles

Chaque opération de la CoreDataAPI retourne un résultat qui indique le succès, l'échec, ou un état transitoire de l'opération. Les résultats sont conceptuels et ne dépendent d'aucun format technique spécifique.

### Succès

Un résultat de succès indique que l'opération a été exécutée avec succès et complétée.

**Pour les opérations de lecture :** Le résultat contient les données demandées. Si aucune donnée ne correspond aux critères (par exemple, une entité n'existe pas), un résultat vide est retourné, ce qui n'est pas une erreur mais un succès avec un résultat vide.

**Pour les opérations d'écriture :** Le résultat contient une confirmation que le WriteIntent a été validé et appliqué. En DB Mère, l'application est immédiate et définitive. En DB Fille, l'application est locale et le WriteIntent est marqué pour synchronisation future.

**Pour les opérations de synchronisation :** Le résultat contient un rapport détaillé de la synchronisation, incluant le nombre de changements propagés, les conflits détectés et résolus, et l'état final de la synchronisation.

**Pour les opérations d'inspection :** Le résultat contient les informations d'état demandées, structurées selon le contrat de l'opération.

### Échec définitif

Un échec définitif indique que l'opération a été rejetée pour une raison qui ne sera pas résolue par une nouvelle tentative avec les mêmes paramètres.

**Erreur de permission :** L'utilisateur n'a pas les permissions nécessaires pour l'opération. Cette erreur ne sera pas résolue en réessayant avec les mêmes paramètres et le même contexte.

**Erreur de cohérence :** Une contrainte de cohérence a été violée (référence invalide, intégrité référentielle, règle métier non respectée). Cette erreur ne sera pas résolue sans modifier les données ou le WriteIntent.

**Erreur de contexte :** Le contexte fourni est incomplet, incohérent, ou invalide. Cette erreur ne sera pas résolue sans corriger le contexte.

**Erreur fonctionnelle :** Une erreur fonctionnelle non récupérable (entité introuvable pour une modification, opération non supportée, etc.). Cette erreur ne sera pas résolue en réessayant.

### Échec temporaire

Un échec temporaire indique que l'opération a échoué pour une raison qui peut être résolue par une nouvelle tentative, éventuellement après un délai.

**Indisponibilité réseau :** La connexion réseau n'est pas disponible pour une synchronisation. Une nouvelle tentative peut réussir une fois la connexion rétablie.

**Instance temporairement indisponible :** L'instance est temporairement indisponible (maintenance, surcharge). Une nouvelle tentative peut réussir une fois l'instance disponible.

**Conflit non résolu automatiquement :** Un conflit détecté lors d'une synchronisation n'a pas pu être résolu automatiquement selon les règles définies. Une nouvelle tentative peut réussir si les règles de résolution sont ajustées ou si une résolution manuelle est effectuée.

**Verrouillage temporaire :** Une ressource est temporairement verrouillée par une autre opération. Une nouvelle tentative peut réussir une fois le verrou libéré.

### État transitoire

Un état transitoire indique que l'opération n'est pas encore complète, est en attente, ou a été partiellement effectuée.

**Opération en cours :** L'opération est en cours d'exécution mais n'est pas encore terminée. Le résultat indique que l'opération est en cours et que le résultat final sera disponible ultérieurement.

**Opération en attente :** L'opération a été acceptée mais mise en attente (par exemple, WriteIntent en attente de synchronisation, demande de synchronisation enregistrée). Le résultat indique l'état d'attente et, si applicable, les conditions de traitement.

**Opération partielle :** L'opération a été partiellement effectuée (par exemple, synchronisation partielle où certains changements ont été appliqués et d'autres rejetés). Le résultat indique le détail de ce qui a réussi et ce qui a échoué.

**Conflits en cours de résolution :** Des conflits ont été détectés et sont en cours de résolution. Le résultat indique les conflits et leur état de résolution (résolus automatiquement, nécessitant une intervention, etc.).

---

## 9. Ce que la CoreDataAPI ne fera JAMAIS

La CoreDataAPI respecte des règles non négociables qui garantissent l'intégrité du système et l'abstraction complète de la persistance. Ces règles sont absolues et aucune exception n'est possible.

### Accès direct au stockage

La CoreDataAPI ne fournit jamais d'accès direct au stockage utilisé par KindMother. Aucune opération ne permet d'exécuter des requêtes directes, d'accéder aux schémas, de lire les fichiers de base de données, ou d'interagir avec le moteur de persistance. L'abstraction de la persistance est totale et non négociable.

### Écriture sans WriteIntent

La CoreDataAPI ne permet jamais d'écriture directe qui contournerait le mécanisme de WriteIntent. Toute modification de données doit passer par submitWriteIntent ou submitBatchWriteIntent, avec validation complète. Aucune opération "writeDirect" ou "forceWrite" n'existe.

### Contournement des permissions

La CoreDataAPI ne fournit jamais de moyen de contourner les vérifications de permissions conceptuelles. Aucune opération ne permet d'exécuter une action avec des permissions insuffisantes, même avec un contexte modifié ou un mode spécial. Toutes les opérations de lecture et d'écriture vérifient les permissions sans exception.

### Modification d'instance par l'appelant

La CoreDataAPI ne permet jamais à l'appelant de modifier l'identité, le type, ou les métadonnées d'une instance (DB Mère ou DB Fille). Seul KindMother peut créer, modifier, ou supprimer des instances. L'appelant peut uniquement utiliser des instances existantes dans le contexte de ses opérations.

### Création, suppression ou réinitialisation d'instance

La CoreDataAPI ne fournit aucune opération de création, suppression ou réinitialisation d'instance. La gestion du cycle de vie des instances (DB Mère et DB Filles) est entièrement interne à KindMother et n'est pas exposée via la CoreDataAPI. L'appelant ne peut pas créer de nouvelles instances, supprimer des instances existantes, ou réinitialiser une instance.

### Contournement de la synchronisation

La CoreDataAPI ne permet jamais d'accès direct inter-instances qui contournerait le mécanisme de synchronisation. Aucune opération ne permet de lire directement depuis la DB Mère depuis une DB Fille, ou d'écrire directement dans une autre instance. Toute interaction inter-instances passe par les opérations de synchronisation.

### Mode bypass des validations

La CoreDataAPI ne fournit jamais de mode "bypass", "force", "admin", ou tout autre mode qui permettrait de contourner les validations de permissions, de cohérence, ou de contexte. Toutes les opérations passent par toutes les validations requises, sans exception et sans option de contournement.

### Exposition de détails d'implémentation

La CoreDataAPI n'expose jamais de détails d'implémentation internes à KindMother. Aucune opération ne révèle la structure interne, les algorithmes utilisés, les optimisations, ou les mécanismes techniques. L'abstraction est complète et les détails d'implémentation restent internes.

### Opérations non documentées

La CoreDataAPI n'expose que les opérations explicitement documentées dans ce document. Aucune opération "cachée", "expérimentale", ou "non documentée" n'est accessible. Toutes les opérations disponibles sont listées dans la section "Vue d'ensemble des opérations" et documentées en détail dans les sections suivantes.

---

## 10. Règles contractuelles finales

Cette section définit les garanties contractuelles entre KindMother et les appelants de la CoreDataAPI. Ces règles établissent ce que chaque partie peut attendre de l'autre.

### Ce que KindMother garantit

**Cohérence locale :** KindMother garantit que toutes les opérations maintiennent la cohérence locale de l'instance. Aucune opération ne laisse l'instance dans un état incohérent. Les contraintes de cohérence (références valides, intégrité référentielle, règles métier) sont toujours respectées après une opération réussie.

**Validation des permissions :** KindMother garantit que toutes les opérations de lecture et d'écriture sont précédées d'une vérification complète des permissions conceptuelles selon les règles fournies par le produit. Aucune opération n'est exécutée si les permissions sont insuffisantes.

**Atomicité :** KindMother garantit que les opérations sont atomiques (tout ou rien). En cas d'échec partiel pendant l'exécution, l'opération est entièrement annulée et l'état reste cohérent. Aucune opération ne laisse l'instance dans un état partiel.

**Traçabilité :** KindMother garantit que toutes les opérations sont tracées avec les métadonnées nécessaires (utilisateur, horodatage, type d'opération, résultat) pour permettre l'audit et le debugging. Ces traces sont conservées selon la stratégie de KindMother.

**Isolation :** KindMother garantit que les opérations concurrentes ne se corrompent pas mutuellement. L'isolation transactionnelle est maintenue pour éviter les lectures inconsistantes et les modifications conflictuelles.

**Messages d'erreur explicites :** KindMother garantit que toutes les erreurs sont accompagnées d'un message explicite indiquant la raison de l'échec, le type d'erreur, et les informations de contexte nécessaires pour comprendre et résoudre le problème.

**Offline-first :** KindMother garantit que toutes les opérations fonctionnent en mode offline. Une DB Fille peut exécuter toutes les opérations (lecture, écriture, inspection) sans connexion à la DB Mère. Les opérations de synchronisation gèrent les périodes de déconnexion de manière transparente.

Cette garantie respecte **LOI-1** (aucune dépendance externe critique) et **LOI-2** (le système accepte l'isolement comme état normal) : toutes les opérations fonctionnent localement sans dépendance externe, et l'isolement est un état normal du système, pas une erreur.

### Ce que l'appelant doit respecter

**Fournir un contexte complet :** L'appelant doit fournir un contexte complet et cohérent pour chaque opération. Le contexte doit inclure l'identité de l'utilisateur, les règles de permissions, l'instance à utiliser, et toutes les informations nécessaires à l'exécution de l'opération. Un contexte incomplet ou incohérent entraîne le rejet de l'opération.

**Respecter le contrat :** L'appelant doit respecter le contrat de chaque opération. Les paramètres doivent être fournis selon le contrat, les types de données doivent être conformes, et les opérations doivent être utilisées selon leur intention documentée. Toute utilisation non conforme peut entraîner des erreurs ou des comportements imprévisibles.

**Ne pas contourner les mécanismes :** L'appelant ne doit jamais tenter de contourner les mécanismes de KindMother. Aucune tentative d'accès direct au stockage, d'écriture sans WriteIntent, de contournement des permissions, ou de modification d'instance n'est autorisée. Toute tentative de contournement constitue une violation architecturale.

**Gérer les erreurs :** L'appelant doit gérer tous les types de résultats possibles (succès, échec définitif, échec temporaire, état transitoire). Les erreurs doivent être interprétées correctement et les actions appropriées doivent être prises (retry pour les erreurs temporaires, correction pour les erreurs définitives, attente pour les états transitoires).

**Respecter les limites :** L'appelant doit respecter les limites documentées de chaque opération. Les garanties de performance, de latence, ou de capacité ne doivent pas être supposées au-delà de ce qui est documenté. Les opérations doivent être utilisées dans les conditions prévues par leur documentation.

**Ne pas dépendre des détails d'implémentation :** L'appelant ne doit jamais faire d'hypothèses sur les détails d'implémentation de KindMother. La structure interne, les algorithmes, les optimisations, ou les mécanismes techniques ne doivent pas être supposés ou utilisés. Seul le contrat de la CoreDataAPI doit être utilisé.

### Évolution du contrat

Pendant la phase v0.x (version interne), KindMother ne garantit aucune compatibilité rétroactive de la CoreDataAPI. L'interface peut évoluer de manière non compatible entre versions pour permettre des évolutions architecturales significatives. Les appelants doivent s'adapter aux changements de contrat entre versions.

Une fois la version 1.0 atteinte, KindMother garantira la compatibilité rétroactive de la CoreDataAPI selon une politique de versioning à définir. Les évolutions seront alors gérées de manière compatible ou avec un processus de dépréciation contrôlé.

---

**Document créé le :** 2026-01-24  
**Version :** 1.0  
**Statut :** Documentation contractuelle CoreDataAPI  
**Référence :** Complète la documentation fondatrice et l'interface & contrat d'intégration de KindMother
