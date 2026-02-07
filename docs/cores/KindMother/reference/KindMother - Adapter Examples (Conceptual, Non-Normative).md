# KindMother — Adapter Examples (Conceptual, Non-Normative)

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il illustre comment utiliser KindMother correctement via des scénarios conceptuels narratifs.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment utiliser KindMother dans différents contextes d'application, en illustrant les concepts contractuels par des scénarios narratifs.

**Avertissement :** Ce document contient uniquement des exemples narratifs conceptuels. Aucun code, pseudo-code, technologie, protocole, ou format de données n'est inclus. Ces exemples sont purement illustratifs et ne prescrivent aucune implémentation.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document illustre comment utiliser KindMother correctement via des scénarios conceptuels narratifs. Il montre comment les concepts contractuels se traduisent en situations d'utilisation réelles, sans exposer d'implémentation.

### 1.2. Nature narrative

Ce document est **purement narratif et conceptuel**. Il décrit des scénarios d'utilisation sous forme d'histoires conceptuelles, sans entrer dans les détails techniques ou d'implémentation.

### 1.3. Sources contractuelles

Ce document se base sur les contrats FONDATION suivants :

- **Instance Model Contract** : Relations Mère/Fille, rôles systémiques
- **Authority Graph & Cross-Domain Contract** : Multi-domaines, Authority Domains
- **Identity & Cross-Domain Trust Contract** : Séparation identité/autorisation
- **CoreDataAPI Contract** : Opérations autorisées, règles d'appel
- **Sync & Conflict Resolution Contract** : Synchronisation, résolution de conflits
- **Write Intent Lifecycle Contract** : Cycle de vie des intentions
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Les exemples illustrent notamment **LOI-2** (isolement comme état normal), **LOI-3** (état local souverain), et **LOI-6** (autonomie n'empêche pas la fédération)

---

## 2. Scénario 1 : Application offline-first

### 2.1. Contexte

**Situation :** Une application mobile qui doit fonctionner de manière autonome, même en l'absence de connexion réseau avec le serveur.

**Architecture :**
- **Instance Mère :** Serveur central qui constitue la source d'autorité de référence
- **Instance Fille :** Application mobile qui maintient une copie locale des données

**Relation :** L'application mobile (Instance Fille) reconnaît l'autorité de l'Instance Mère (serveur) et synchronise périodiquement avec elle.

### 2.2. Intentions émises

**Création d'entités en offline :**

L'utilisateur crée de nouvelles entités (par exemple, des notes, des tâches, des contacts) pendant que l'application est hors ligne. Ces créations sont formulées comme des Write Intents et soumises à l'Instance Fille locale.

**Modification d'entités en offline :**

L'utilisateur modifie des entités existantes (par exemple, met à jour le contenu d'une note, change le statut d'une tâche) pendant que l'application est hors ligne. Ces modifications sont formulées comme des Write Intents et soumises à l'Instance Fille locale.

**Suppression d'entités en offline :**

L'utilisateur supprime des entités (par exemple, supprime une note, archive une tâche) pendant que l'application est hors ligne. Ces suppressions sont formulées comme des Write Intents et soumises à l'Instance Fille locale.

### 2.3. Validations attendues

**Validation locale :**

L'Instance Fille valide localement chaque Write Intent selon les règles de cohérence locales. Les validations locales vérifient que :
- Le contexte est complet (utilisateur, permissions, instance)
- Les permissions locales sont suffisantes
- La cohérence locale est préservée
- Aucune contrainte locale n'est violée

Si la validation locale réussit, l'intention est appliquée localement, permettant à l'utilisateur de continuer à utiliser l'application même hors ligne.

**Validation Mère lors de sync :**

Lorsque la connexion réseau est rétablie, l'Instance Fille synchronise avec l'Instance Mère. Les Write Intents appliquées localement sont soumises à l'Instance Mère pour validation définitive.

L'Instance Mère valide chaque intention selon les règles de cohérence de référence :
- Le contexte est valide et complet
- Les permissions sont suffisantes selon les règles de référence
- La cohérence de référence est préservée
- Aucune contrainte de référence n'est violée
- Aucun conflit avec d'autres modifications n'est détecté

### 2.4. Rejets possibles

**Conflit autoritaire :**

L'Instance Mère peut rejeter une intention locale si elle entre en conflit avec une modification effectuée sur l'Instance Mère pendant que l'application était hors ligne. Par exemple, si l'utilisateur a modifié une note localement, mais que la même note a été supprimée sur l'Instance Mère, l'intention de modification est rejetée.

**Violation de cohérence :**

L'Instance Mère peut rejeter une intention locale si elle viole une contrainte de cohérence de référence. Par exemple, si l'utilisateur a créé localement une entité qui viole une contrainte métier définie sur l'Instance Mère, l'intention de création est rejetée.

**Permissions insuffisantes :**

L'Instance Mère peut rejeter une intention locale si les permissions de l'utilisateur ne sont pas suffisantes selon les règles de référence. Par exemple, si l'utilisateur a créé localement une entité qui nécessite des permissions qu'il n'a pas selon l'Instance Mère, l'intention est rejetée.

### 2.5. Garanties fournies

**Durabilité locale :**

Les Write Intents appliquées localement sont persistées localement, garantissant que les modifications de l'utilisateur ne sont pas perdues même si l'application est fermée ou si l'appareil est redémarré.

**Synchronisation ultérieure :**

Les Write Intents appliquées localement sont marquées pour synchronisation ultérieure. Lorsque la connexion réseau est rétablie, ces intentions sont automatiquement soumises à l'Instance Mère pour validation définitive.

**Cohérence locale :**

L'Instance Fille maintient la cohérence locale, garantissant que les données locales sont cohérentes même si elles ne sont pas encore synchronisées avec l'Instance Mère.

**Conformité LOI-2 et LOI-3 :** Ce scénario illustre **LOI-2** (le système accepte l'isolement comme état normal) : l'application fonctionne localement même sans connexion réseau, l'isolement n'est pas traité comme une erreur mais comme un état valide. Il illustre également **LOI-3** (l'état local est souverain) : l'Instance Fille détient l'autorité locale sur son état, et la réconciliation avec l'Instance Mère est explicite et traçable.

**Alignement contractuel :**

- Respecte l'invariant INST-F-2 (copie locale synchronisée)
- Respecte l'invariant INST-F-4 (autonomie limitée)
- Respecte l'invariant INST-F-5 (soumission des opérations à la validation)
- Aligné avec le Write Intent Lifecycle Contract (intentions locales vs intentions définitives)

---

## 3. Scénario 2 : CMS local avec synchronisation

### 3.1. Contexte

**Situation :** Un système de gestion de contenu (CMS) avec édition locale et publication sur un serveur.

**Architecture :**
- **Instance Mère :** Serveur de publication qui constitue la source d'autorité de référence pour le contenu publié
- **Instance Fille :** Éditeur local qui permet l'édition de contenu avant publication

**Relation :** L'éditeur local (Instance Fille) reconnaît l'autorité de l'Instance Mère (serveur de publication) et synchronise périodiquement pour publier le contenu édité.

### 3.2. Intentions émises

**Création de contenu :**

L'éditeur crée de nouveaux contenus (par exemple, des articles, des pages, des médias) localement. Ces créations sont formulées comme des Write Intents et soumises à l'Instance Fille locale.

**Modification de contenu :**

L'éditeur modifie des contenus existants (par exemple, met à jour le texte d'un article, change les métadonnées d'une page) localement. Ces modifications sont formulées comme des Write Intents et soumises à l'Instance Fille locale.

**Publication de contenu :**

L'éditeur déclenche la publication de contenu, ce qui synchronise les Write Intents locales avec l'Instance Mère pour validation et publication.

### 3.3. Validations attendues

**Validation locale :**

L'Instance Fille valide localement chaque Write Intent selon les règles d'édition locales. Les validations locales vérifient que :
- Le contexte est complet
- Les permissions d'édition sont suffisantes
- La cohérence éditoriale locale est préservée
- Aucune contrainte d'édition locale n'est violée

**Validation Mère pour publication :**

Lors de la synchronisation pour publication, l'Instance Mère valide chaque intention selon les règles de publication :
- Le contexte est valide et complet
- Les permissions de publication sont suffisantes
- La cohérence de publication est préservée
- Aucune contrainte de publication n'est violée
- Le contenu respecte les règles de publication (format, longueur, qualité, etc.)

### 3.4. Rejets possibles

**Contrainte de publication violée :**

L'Instance Mère peut rejeter une intention de publication si elle viole une contrainte de publication. Par exemple, si un article dépasse la longueur maximale autorisée, ou si le contenu ne respecte pas les règles de qualité, l'intention de publication est rejetée.

**Permissions de publication insuffisantes :**

L'Instance Mère peut rejeter une intention de publication si les permissions de l'éditeur ne sont pas suffisantes pour publier. Par exemple, si l'éditeur a les permissions d'édition mais pas les permissions de publication, l'intention est rejetée.

**Conflit avec contenu publié :**

L'Instance Mère peut rejeter une intention de publication si elle entre en conflit avec du contenu déjà publié. Par exemple, si l'éditeur a modifié localement un article qui a été modifié différemment sur l'Instance Mère, l'intention de publication est rejetée.

### 3.5. Garanties fournies

**Édition locale :**

L'Instance Fille permet l'édition locale de contenu, garantissant que l'éditeur peut travailler localement sans dépendre de la connexion réseau.

**Publication contrôlée :**

L'Instance Mère contrôle la publication, garantissant que seuls les contenus validés et conformes aux règles de publication sont publiés.

**Cohérence éditoriale :**

L'Instance Fille maintient la cohérence éditoriale locale, garantissant que les contenus édités localement sont cohérents même s'ils ne sont pas encore publiés.

**Conformité LOI-2 et LOI-3 :** Ce scénario illustre **LOI-2** (le système accepte l'isolement comme état normal) : l'éditeur peut travailler localement sans dépendre de la connexion réseau. Il illustre également **LOI-3** (l'état local est souverain) : les contenus édités localement sont valables localement jusqu'à la réconciliation explicite avec l'Instance Mère lors de la publication.

**Alignement contractuel :**

- Respecte l'invariant INST-F-1 (reconnaissance de l'autorité de l'Instance Mère)
- Respecte l'invariant INST-F-2 (copie locale synchronisée)
- Respecte l'invariant INST-F-5 (soumission des opérations à la validation)
- Aligné avec le Sync & Conflict Resolution Contract (synchronisation, résolution de conflits)

---

## 4. Scénario 3 : Jeu multi-domaines

### 4.1. Contexte

**Situation :** Un jeu avec plusieurs domaines d'autorité distincts (Identity, Game, Commerce) qui doivent communiquer de manière contrôlée.

**Architecture :**
- **Instance Mère Identity :** Gère l'identité et l'authentification des joueurs
- **Instance Mère Game :** Gère les données de jeu (scores, progression, inventaire)
- **Instance Mère Commerce :** Gère les transactions commerciales (achats, paiements)
- **Instance Fille (client) :** Application cliente qui interagit avec les trois domaines

**Relation :** L'application cliente (Instance Fille) reconnaît l'autorité de chaque Instance Mère pour son domaine respectif et synchronise avec chacune d'elles.

### 4.2. Intentions émises

**Actions de jeu :**

Le joueur effectue des actions de jeu (par exemple, complète un niveau, gagne des points, obtient un objet). Ces actions sont formulées comme des Write Intents et soumises à l'Instance Mère Game via l'Instance Fille.

**Transactions commerciales :**

Le joueur effectue des transactions commerciales (par exemple, achète un objet, effectue un paiement). Ces transactions sont formulées comme des Write Intents et soumises à l'Instance Mère Commerce via l'Instance Fille.

**Mises à jour d'identité :**

Le joueur met à jour son profil ou ses préférences. Ces mises à jour sont formulées comme des Write Intents et soumises à l'Instance Mère Identity via l'Instance Fille.

### 4.3. Validations attendues

**Validation par domaine :**

Chaque Instance Mère valide les intentions selon les règles de son domaine :
- **Instance Mère Identity :** Valide les intentions d'identité selon les règles d'identité
- **Instance Mère Game :** Valide les intentions de jeu selon les règles de jeu
- **Instance Mère Commerce :** Valide les intentions commerciales selon les règles commerciales

**Intentions Certifiées inter-domaines :**

Lorsqu'une action nécessite une communication entre domaines (par exemple, un achat dans le jeu nécessite à la fois une validation Game et une validation Commerce), des Intentions Certifiées sont créées et validées par KindMother pour permettre la communication contrôlée entre domaines.

### 4.4. Rejets possibles

**Violation inter-domaines :**

Une intention peut être rejetée si elle viole les règles de communication inter-domaines. Par exemple, si une intention de jeu tente d'accéder directement aux données du domaine Commerce sans passer par une Intention Certifiée, elle est rejetée.

**Autorité non reconnue :**

Une intention peut être rejetée si l'autorité du domaine n'est pas reconnue. Par exemple, si l'Instance Fille tente de soumettre une intention à un domaine qui n'est pas reconnu ou qui n'a pas autorisé l'Instance Fille, l'intention est rejetée.

**Permissions insuffisantes :**

Une intention peut être rejetée si les permissions du joueur ne sont pas suffisantes pour le domaine. Par exemple, si un joueur tente d'effectuer une action de jeu qui nécessite des permissions qu'il n'a pas, l'intention est rejetée.

### 4.5. Garanties fournies

**Isolation par domaine :**

Chaque domaine maintient son isolation, garantissant que les données d'un domaine ne sont pas directement accessibles depuis un autre domaine.

**Communication contrôlée :**

La communication entre domaines est contrôlée par KindMother via des Intentions Certifiées, garantissant que seules les communications autorisées et validées sont permises.

**Cohérence par domaine :**

Chaque domaine maintient sa propre cohérence, garantissant que les données de chaque domaine sont cohérentes selon les règles de ce domaine.

**Conformité LOI-6 :** Ce scénario illustre **LOI-6** (l'autonomie n'empêche pas la fédération) : chaque domaine reste autonome (LOI-1 à LOI-5) tout en participant à une fédération contrôlée via des Intentions Certifiées. La communication inter-domaines est explicite, contrôlée, observable, et réversible.

**Alignement contractuel :**

- Respecte l'Authority Graph & Cross-Domain Contract (multi-domaines, isolation)
- Respecte l'Identity & Cross-Domain Trust Contract (séparation identité/autorisation, Intentions Certifiées)
- Respecte l'invariant INST-3 (isolation systémique)
- Aligné avec le CoreDataAPI Contract (pas de communication directe inter-domaines)

---

## 5. Scénario 4 : Application hybride (local + serveur)

### 5.1. Contexte

**Situation :** Une application avec données locales et synchronisation périodique avec un serveur.

**Architecture :**
- **Instance Mère :** Serveur central qui constitue la source d'autorité de référence
- **Instance Fille :** Application cliente qui maintient une copie locale des données

**Relation :** L'application cliente (Instance Fille) reconnaît l'autorité de l'Instance Mère (serveur) et synchronise périodiquement pour maintenir la cohérence.

### 5.2. Intentions émises

**Modifications locales avec sync périodique :**

L'utilisateur modifie des données localement (par exemple, met à jour un profil, modifie des préférences, crée des entités). Ces modifications sont formulées comme des Write Intents et soumises à l'Instance Fille locale.

La synchronisation avec l'Instance Mère est effectuée périodiquement (par exemple, toutes les heures, ou lorsque l'utilisateur le demande explicitement).

### 5.3. Validations attendues

**Validation locale immédiate :**

L'Instance Fille valide localement chaque Write Intent selon les règles de cohérence locales. Si la validation locale réussit, l'intention est appliquée localement, permettant à l'utilisateur de voir immédiatement ses modifications.

**Validation Mère différée :**

Lors de la synchronisation périodique, les Write Intents appliquées localement sont soumises à l'Instance Mère pour validation définitive. L'Instance Mère valide chaque intention selon les règles de cohérence de référence.

### 5.4. Rejets possibles

**Conflit lors de synchronisation :**

L'Instance Mère peut rejeter une intention locale si elle entre en conflit avec une modification effectuée sur l'Instance Mère pendant que l'application était locale. Par exemple, si l'utilisateur a modifié localement une entité qui a été supprimée sur l'Instance Mère, l'intention de modification est rejetée.

**Violation de cohérence de référence :**

L'Instance Mère peut rejeter une intention locale si elle viole une contrainte de cohérence de référence. Par exemple, si l'utilisateur a créé localement une entité qui viole une contrainte métier définie sur l'Instance Mère, l'intention de création est rejetée.

### 5.5. Garanties fournies

**Fonctionnement autonome :**

L'Instance Fille permet un fonctionnement autonome, garantissant que l'utilisateur peut utiliser l'application même en l'absence de connexion réseau, dans les limites autorisées.

**Cohérence ultérieure :**

La synchronisation périodique garantit que la cohérence avec l'Instance Mère est rétablie ultérieurement, même si des modifications locales ont été effectuées.

**Durabilité locale :**

Les Write Intents appliquées localement sont persistées localement, garantissant que les modifications de l'utilisateur ne sont pas perdues même si l'application est fermée.

**Conformité LOI-2 et LOI-3 :** Ce scénario illustre **LOI-2** (le système accepte l'isolement comme état normal) : l'application fonctionne de manière autonome même en l'absence de connexion réseau, avec synchronisation périodique différée. Il illustre également **LOI-3** (l'état local est souverain) : les modifications locales sont valables localement jusqu'à la réconciliation explicite avec l'Instance Mère.

**Alignement contractuel :**

- Respecte l'invariant INST-F-2 (copie locale synchronisée)
- Respecte l'invariant INST-F-3 (synchronisation périodique)
- Respecte l'invariant INST-F-4 (autonomie limitée)
- Aligné avec le Sync & Conflict Resolution Contract (synchronisation, résolution de conflits)

---

## 6. Exemples d'erreurs courantes côté adaptateur

### 6.1. Erreur 1 : Supposer que l'identité = autorisation

**Scénario :**

Un adaptateur pense qu'une identité reconnue par KindMother autorise automatiquement toutes les opérations pour cette identité. L'adaptateur suppose que si l'identité est valide, les permissions sont automatiquement accordées.

**Conséquence :**

KindMother rejette les opérations car l'identité ne confère pas automatiquement des autorisations. Les permissions doivent être évaluées séparément selon les règles du domaine d'autorité.

**Exemple conceptuel :**

Un adaptateur soumet une Write Intent avec une identité valide mais sans fournir les permissions nécessaires. L'adaptateur suppose que l'identité valide suffit, mais KindMother rejette l'intention car les permissions ne sont pas suffisantes.

**Correction :**

L'adaptateur DOIT fournir un contexte complet incluant à la fois l'identité ET les permissions. L'identité et l'autorisation sont séparées selon le Identity & Cross-Domain Trust Contract.

**Alignement contractuel :**

- Respecte la séparation identité/autorisation du Identity & Cross-Domain Trust Contract
- Respecte l'invariant INST-6 (validation obligatoire)
- Respecte la boundary de permissions du Runtime Boundary Contract

### 6.2. Erreur 2 : Contourner la CoreDataAPI pour performance

**Scénario :**

Un adaptateur tente d'accéder directement aux données pour "optimiser" les performances, pensant que passer par la CoreDataAPI est trop lent.

**Conséquence :**

KindMother détecte la tentative de contournement et rejette l'opération. Si la tentative est répétée, l'adaptateur peut être mis en quarantaine.

**Exemple conceptuel :**

Un adaptateur crée un mécanisme de "lecture directe" qui contourne la CoreDataAPI pour accéder directement aux données, pensant améliorer les performances. KindMother détecte cette tentative et bloque l'accès.

**Correction :**

L'adaptateur DOIT utiliser exclusivement la CoreDataAPI pour toutes les opérations. Aucun contournement n'est autorisé, même pour des raisons de performance.

**Alignement contractuel :**

- Respecte l'invariant INV-API-1 (unicité de la surface d'appel)
- Respecte les règles UNIQ-1 à UNIQ-5 (unicité de la CoreDataAPI)
- Respecte l'interdiction INTERDIT-2 (pas d'accès direct à la persistance)
- Aligné avec la réponse systémique R3 (mise en quarantaine) pour violations répétées

### 6.3. Erreur 3 : Ignorer les rejets de synchronisation

**Scénario :**

Un adaptateur ignore les rejets de synchronisation, pensant que les modifications locales sont suffisantes et que les rejets de l'Instance Mère peuvent être ignorés.

**Conséquence :**

Les données locales deviennent incohérentes avec l'Instance Mère. Les modifications locales rejetées par l'Instance Mère restent dans l'Instance Fille, créant une divergence permanente.

**Exemple conceptuel :**

Un adaptateur soumet des Write Intents locales à l'Instance Mère lors de la synchronisation. Certaines intentions sont rejetées par l'Instance Mère, mais l'adaptateur ignore ces rejets et continue à utiliser les données locales comme si elles étaient valides.

**Correction :**

L'adaptateur DOIT accepter les décisions de l'Instance Mère. Si une intention locale est rejetée par l'Instance Mère, l'adaptateur DOIT annuler les modifications locales correspondantes et informer l'utilisateur.

**Alignement contractuel :**

- Respecte l'invariant INST-F-1 (reconnaissance de l'autorité de l'Instance Mère)
- Respecte l'invariant INST-F-5 (soumission des opérations à la validation)
- Aligné avec le Sync & Conflict Resolution Contract (acceptation des décisions de la Mère)

---

## 7. Ce que KindMother accepte / refuse

### 7.1. Accepte : Intentions valides avec contexte complet

**KindMother accepte :**

- **Intentions avec contexte complet :** Intentions accompagnées d'un contexte complet incluant l'identité, les permissions, l'instance, et le domaine d'autorité
- **Intentions conformes aux règles :** Intentions qui respectent toutes les règles de cohérence, de permissions, et de validation
- **Intentions légales :** Intentions qui utilisent des opérations légales de la CoreDataAPI
- **Intentions non conflictuelles :** Intentions qui ne créent pas de conflits avec l'état actuel des données

**Exemple conceptuel :**

Un adaptateur soumet une Write Intent pour créer une nouvelle entité. L'intention est accompagnée d'un contexte complet (utilisateur identifié, permissions suffisantes, instance valide, domaine d'autorité valide). L'intention respecte toutes les règles de cohérence. KindMother accepte l'intention, la valide, et l'applique.

### 7.2. Refuse : Intentions sans contexte, tentatives de contournement, violations d'invariants

**KindMother refuse :**

- **Intentions sans contexte complet :** Intentions qui ne sont pas accompagnées d'un contexte complet (identité manquante, permissions manquantes, instance manquante, domaine manquant)
- **Tentatives de contournement :** Tentatives d'accéder directement aux données, de contourner la CoreDataAPI, ou de contourner les validations
- **Violations d'invariants :** Intentions qui violeraient un invariant contractuel (par exemple, violation de l'isolation, violation de l'autorité exclusive)
- **Intentions non conformes :** Intentions qui ne respectent pas les règles de cohérence, de permissions, ou de validation
- **Intentions conflictuelles :** Intentions qui créent des conflits avec l'état actuel des données

**Exemple conceptuel :**

Un adaptateur tente de soumettre une Write Intent sans fournir les permissions nécessaires. KindMother rejette l'intention car le contexte est incomplet. L'adaptateur reçoit une erreur explicite indiquant que les permissions sont manquantes.

**Exemple conceptuel :**

Un adaptateur tente d'accéder directement aux données sans passer par la CoreDataAPI. KindMother détecte cette tentative de contournement et rejette l'opération. Si la tentative est répétée, l'adaptateur est mis en quarantaine.

---

## 8. Conclusion

Ce document illustre comment utiliser KindMother correctement via des scénarios conceptuels narratifs.

**Points clés :**
- Les scénarios illustrent les concepts contractuels dans différents contextes d'utilisation
- Les exemples d'erreurs montrent ce qu'il ne faut pas faire
- Les règles d'acceptation/refus clarifient ce que KindMother accepte et refuse

**Nature informative :**
Ce document est purement informatif et narratif. Il ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à illustrer les concepts contractuels par des exemples narratifs.

**Rappel :** Les contrats FONDATION priment toujours sur ces exemples. En cas de doute, se référer aux contrats FONDATION.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System v2.4, KindMother Documentation, Instance Model Contract, Authority Graph & Cross-Domain Contract, Identity & Cross-Domain Trust Contract, CoreDataAPI Contract, Sync & Conflict Resolution Contract, Write Intent Lifecycle Contract  
**Type :** Document informatif narratif conceptuel

---

## 9. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Niveau de détail narratif

**Arbitrage rencontré :** Quel niveau de détail inclure dans les scénarios narratifs ? Les scénarios doivent rester conceptuels sans devenir trop abstraits ou trop techniques.

**Décision prise :** Les scénarios sont narratifs et descriptifs, décrivant des situations d'utilisation réelles de manière conceptuelle, sans détails techniques. Ils illustrent les concepts contractuels par des histoires compréhensibles.

**Justification :** Cette approche rend les concepts contractuels accessibles tout en restant purement conceptuelle. Les scénarios aident à comprendre sans prescrire d'implémentation.

**Documentation :** Tous les scénarios (sections 2 à 5) sont narratifs et descriptifs, sans détails techniques.

### Arbitrage A2 : Exemples d'erreurs vs prescriptions

**Arbitrage rencontré :** Comment illustrer les erreurs courantes sans créer l'impression que certaines erreurs sont "acceptables" ou "tolérées" ?

**Décision prise :** Les exemples d'erreurs sont clairement présentés comme des erreurs à éviter, avec leurs conséquences et leurs corrections. Aucune ambiguïté n'est laissée sur le fait que ces erreurs sont interdites.

**Justification :** Cette approche éducative aide les développeurs à comprendre les erreurs courantes et à les éviter, tout en restant claire sur le fait que ces erreurs sont interdites.

**Documentation :** Section 6 (Exemples d'erreurs courantes) avec conséquences et corrections explicites.

### Arbitrage A3 : Scénarios multi-domaines

**Arbitrage rencontré :** Comment illustrer le scénario multi-domaines sans créer de confusion sur la complexité de la gestion multi-domaines ?

**Décision prise :** Le scénario multi-domaines est simplifié pour illustrer les concepts clés (isolation, communication contrôlée, Intentions Certifiées) sans entrer dans tous les détails de la gestion multi-domaines.

**Justification :** Cette simplification permet d'illustrer les concepts essentiels sans surcharger le document avec tous les détails de la gestion multi-domaines, qui sont couverts dans les contrats FONDATION.

**Documentation :** Section 4 (Scénario 3 : Jeu multi-domaines) avec focus sur les concepts clés.

### Arbitrage A4 : Balance entre exemples positifs et négatifs

**Arbitrage rencontré :** Comment équilibrer les exemples positifs (scénarios d'utilisation correcte) et les exemples négatifs (erreurs courantes) ?

**Décision prise :** Les scénarios positifs (sections 2 à 5) illustrent l'utilisation correcte, tandis que la section 6 illustre les erreurs courantes. La section 7 clarifie ce que KindMother accepte et refuse.

**Justification :** Cette organisation permet de montrer d'abord l'utilisation correcte, puis les erreurs à éviter, puis les règles d'acceptation/refus. Cette progression pédagogique facilite la compréhension.

**Documentation :** Organisation en sections : scénarios positifs (2-5), erreurs (6), règles d'acceptation/refus (7).

### Arbitrage A5 : Références aux contrats dans les scénarios

**Arbitrage rencontré :** Comment référencer les contrats FONDATION dans les scénarios narratifs sans interrompre le flux narratif ?

**Décision prise :** Les références aux contrats sont incluses dans une sous-section "Alignement contractuel" à la fin de chaque scénario, permettant de maintenir le flux narratif tout en fournissant les références contractuelles.

**Justification :** Cette approche permet de maintenir la lisibilité narrative tout en fournissant les références contractuelles nécessaires pour comprendre l'alignement avec les contrats FONDATION.

**Documentation :** Chaque scénario (sections 2 à 5) inclut une sous-section "Alignement contractuel" avec références explicites.

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
