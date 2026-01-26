# TAMR - Documentation Fondatrice

## 1. Introduction

### Objet du document

Ce document définit **TAMR (The Authority Must Rest)** : le Human Interaction Core du Miyukini Core System. Il établit un contrat normatif, non négociable, et de statut FONDATION qui définit conceptuellement où, quand, et comment l'humain intervient dans le système.

TAMR ne définit pas d'interface utilisateur, ne prend aucune décision, ne gère aucune technique. Il définit les points d'intervention humaine, les limites de l'autorité humaine, et les règles de coexistence entre automatisation et intervention humaine.

### Question fondamentale

**"Quand l'humain a-t-il le droit d'intervenir dans le système, et quelles sont les limites de cette intervention ?"**

TAMR répond à cette question en définissant :
- Les types d'intervention humaine (approbation, override, escalade, supervision)
- Les conditions qui déclenchent ou autorisent ces interventions
- Les limites de ce que l'humain peut et ne peut pas faire
- La traçabilité de toute intervention humaine
- La responsabilité partagée entre système et humain

### Portée

Ce contrat s'applique à **toutes les interactions entre un humain et le système Miyukini** et définit de manière absolue :
- la nature conceptuelle de l'intervention humaine,
- les types d'intervention autorisés,
- les invariants d'intervention,
- les limites de l'autorité humaine,
- les garanties de traçabilité,
- les responsabilités partagées.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

---

## 2. Raison d'être

### Pourquoi TAMR existe

TAMR existe parce que les systèmes automatisés ne peuvent pas tout décider seuls. Certaines situations nécessitent un jugement humain, une validation manuelle, ou une décision éthique que seul un humain peut prendre. Sans cadre conceptuel clair, les interventions humaines seraient :
- Arbitraires (décidées au cas par cas sans cohérence)
- Non traçables (sans historique ni responsabilité)
- Dangereuses (intervention dans des zones où l'humain ne devrait pas intervenir)
- Insuffisantes (absence d'intervention là où elle est nécessaire)

TAMR résout ces problèmes en définissant un cadre conceptuel qui :
- **Normalise** les types d'intervention humaine
- **Délimite** les zones où l'humain peut intervenir
- **Trace** toute intervention pour audit et responsabilité
- **Protège** le système contre les interventions non autorisées
- **Garantit** que les interventions nécessaires sont possibles

### Le problème de l'automatisation pure

Un système entièrement automatisé présente des risques :

1. **Décisions éthiques** : Certaines décisions nécessitent un jugement moral que l'automatisation ne peut pas porter
2. **Situations imprévues** : L'automatisation ne peut pas gérer tous les cas de figure
3. **Erreurs systémiques** : Une erreur dans la logique automatisée peut se propager sans contrôle
4. **Responsabilité légale** : Certaines décisions doivent être attribuables à un humain
5. **Confiance utilisateur** : Les utilisateurs ont besoin de savoir qu'un humain peut intervenir si nécessaire

### Le problème de l'intervention non encadrée

Une intervention humaine non encadrée présente également des risques :

1. **Incohérence** : Différents humains prennent différentes décisions pour des situations similaires
2. **Contournement** : L'humain contourne les règles du système
3. **Non-traçabilité** : Les interventions ne sont pas enregistrées
4. **Responsabilité floue** : Impossible de déterminer qui a fait quoi
5. **Fragilité** : Le système dépend trop de l'intervention humaine

TAMR équilibre ces deux extrêmes en définissant un cadre où l'intervention humaine est :
- **Possible** là où elle est nécessaire
- **Impossible** là où elle est dangereuse
- **Traçable** dans tous les cas
- **Responsabilisante** pour l'humain qui intervient

---

## 3. Positionnement familial

### Relation avec StrongFather

TAMR définit les points d'intervention humaine. **StrongFather décide** si cette intervention est autorisée.

La relation est complémentaire et non concurrente :
- TAMR dit : "Voici les types d'intervention humaine possibles"
- StrongFather dit : "Cette intervention spécifique est-elle autorisée selon les politiques ?"

TAMR ne décide jamais si une intervention est acceptée ou refusée. Il définit les règles conceptuelles, StrongFather applique les politiques concrètes. Quand un humain demande à intervenir, TAMR catégorise le type d'intervention, et StrongFather évalue si cette intervention est autorisée selon le contexte, l'utilisateur, et les politiques.

### Relation avec KindMother

TAMR ne persiste rien. **KindMother** est responsable de la persistance.

La relation est strictement unidirectionnelle :
- TAMR définit ce qui doit être tracé lors d'une intervention
- KindMother persiste les traces via les mécanismes standards
- TAMR ne connaît pas les détails de persistance

Les logs d'intervention humaine, les historiques d'override, et les traces d'escalade sont des données comme les autres. TAMR définit leur structure conceptuelle, KindMother gère leur persistance.

### Relation avec BondingBrother

TAMR utilise **BondingBrother** comme canal de médiation pour les interventions.

La relation est de service :
- L'intervention humaine est une intention comme une autre
- Cette intention transite par BondingBrother vers les autorités concernées
- BondingBrother traduit, filtre, et transmet selon ses règles

TAMR ne communique jamais directement avec les autorités. Toute intervention passe par BondingBrother, qui garantit le respect des règles de l'écosystème.

### Relation avec les produits

Les produits définissent **comment** les interventions sont présentées à l'humain. TAMR définit **quoi** et **quand**.

La relation est de séparation stricte :
- TAMR : types d'intervention, conditions, limites (conceptuel)
- Produits : interfaces, workflows, notifications (technique/UI)

Un produit peut implémenter une interface de validation humaine, mais les règles de cette validation (quand, qui, pourquoi) sont définies par TAMR et évaluées par StrongFather.

### Position dans la famille Miyukini

Dans la famille Miyukini, TAMR est le **gardien de la place de l'humain**. Il ne remplace aucune autorité, ne prend aucune décision, mais garantit que l'humain conserve sa juste place dans un système automatisé.

TAMR est le frère qui rappelle : "L'humain a le droit d'intervenir ici, dans ces conditions, avec ces limites."

---

## 4. Concepts fondamentaux

### Intervention humaine

Une **intervention humaine** est toute action délibérée d'un humain qui modifie, valide, suspend, ou annule un processus automatisé du système.

Caractéristiques d'une intervention :
- **Délibérée** : Consciente et volontaire, pas accidentelle
- **Traçable** : Enregistrée avec identité, moment, et contexte
- **Catégorisée** : Appartient à un type défini par TAMR
- **Limitée** : Soumise aux règles et limites définies
- **Responsabilisante** : L'humain assume la responsabilité de son intervention

### Types d'intervention

TAMR définit quatre types fondamentaux d'intervention humaine :

**1. Approbation (Approval)**

L'humain valide une action avant son exécution. Le système propose, l'humain approuve ou refuse.

Caractéristiques :
- Préventive : avant l'action
- Binaire : approuvé ou refusé
- Bloquante : l'action attend la décision humaine
- Obligatoire ou optionnelle selon la configuration

**2. Override (Dérogation)**

L'humain force une action malgré un refus automatique, ou empêche une action malgré une approbation automatique.

Caractéristiques :
- Dérogatoire : contredit la décision automatique
- Exceptionnelle : ne doit pas être la norme
- Justifiée : nécessite une raison explicite
- Auditée : fait l'objet d'un suivi renforcé

**3. Escalade (Escalation)**

L'humain élève une décision vers un niveau d'autorité supérieur humain ou demande une révision.

Caractéristiques :
- Hiérarchique : monte dans la chaîne de responsabilité
- Non bloquante immédiatement : peut différer la décision
- Collaborative : implique plusieurs humains
- Tracée : chemin d'escalade enregistré

**4. Supervision (Monitoring)**

L'humain observe et surveille sans modifier, avec capacité d'intervention si nécessaire.

Caractéristiques :
- Passive par défaut : observation sans action
- Activable : peut déclencher une intervention si nécessaire
- Continue : surveillance prolongée dans le temps
- Non intrusif : n'affecte pas le fonctionnement normal

### Point d'intervention

Un **point d'intervention** est un moment défini dans un processus où l'intervention humaine est possible ou requise.

Caractéristiques :
- **Défini** : Identifié explicitement dans le processus
- **Conditionnel** : Activé selon des conditions définies
- **Typé** : Associé à un ou plusieurs types d'intervention
- **Configurable** : Le produit peut ajuster les conditions

### Limite d'autorité

Une **limite d'autorité** est une restriction sur ce que l'humain peut faire lors d'une intervention.

Caractéristiques :
- **Explicite** : Définie clairement, jamais implicite
- **Absolue** : Certaines limites sont non négociables
- **Contextuelle** : Certaines limites dépendent du contexte
- **Protectrice** : Protège le système et l'humain

---

## 5. Responsabilités exclusives

### Définition des types d'intervention

TAMR est **exclusivement responsable** de définir les types d'intervention humaine. Aucun autre core ne peut créer, modifier, ou supprimer un type d'intervention.

Les quatre types (Approval, Override, Escalation, Supervision) sont définis par TAMR et ne peuvent être étendus qu'avec une évolution formelle de TAMR.

### Définition des points d'intervention

TAMR est **exclusivement responsable** de définir les catégories de points d'intervention. Les processus du système doivent déclarer leurs points d'intervention selon les catégories définies par TAMR.

TAMR ne définit pas les points d'intervention spécifiques à chaque produit, mais les catégories et règles que ces points doivent respecter.

### Définition des limites d'autorité

TAMR est **exclusivement responsable** de définir les limites d'autorité humaine. Ces limites sont des invariants non négociables que toute intervention doit respecter.

Les limites définies par TAMR sont :
- Les limites absolues (applicables à toute intervention)
- Les limites par type d'intervention
- Les limites par contexte (définies conceptuellement)

### Définition des exigences de traçabilité

TAMR est **exclusivement responsable** de définir ce qui doit être tracé lors d'une intervention humaine. La structure conceptuelle des traces est définie par TAMR.

Toute intervention doit être traçable selon les exigences de TAMR :
- Identité de l'humain intervenant
- Type d'intervention
- Moment de l'intervention
- Contexte de l'intervention
- Justification (si requise)
- Résultat de l'intervention

### Définition des règles de responsabilité

TAMR est **exclusivement responsable** de définir les règles de responsabilité partagée entre système et humain.

Quand un humain intervient :
- L'humain assume la responsabilité de son intervention
- Le système assume la responsabilité de permettre ou refuser l'intervention
- La responsabilité est tracée et attribuable

---

## 6. Ce que TAMR ne fait PAS

### Ne décide pas

TAMR ne prend aucune décision. Il définit les règles d'intervention, mais c'est **StrongFather** qui décide si une intervention spécifique est autorisée.

TAMR dit : "Une approbation est un type d'intervention valide."
StrongFather dit : "Cet utilisateur peut-il approuver cette action dans ce contexte ?"

### Ne persiste pas

TAMR ne persiste aucune donnée. Les traces d'intervention, les historiques, et les logs sont persistés par **KindMother** selon les structures définies par TAMR.

TAMR dit : "Une intervention doit être tracée avec ces informations."
KindMother persiste ces informations selon ses mécanismes.

### Ne définit pas d'interface utilisateur

TAMR ne définit aucune interface, aucun écran, aucun workflow visuel. Les **produits** sont responsables de l'implémentation technique des interfaces d'intervention.

TAMR dit : "Un point d'approbation existe à cet endroit du processus."
Le produit dit : "Voici l'écran que l'utilisateur verra pour approuver."

### Ne gère pas l'authentification

TAMR ne gère pas l'authentification technique. L'identité de l'humain intervenant est fournie par le produit via les mécanismes d'authentification du système.

TAMR dit : "L'intervention doit être tracée avec l'identité de l'intervenant."
Le produit fournit cette identité via ses mécanismes d'auth.

### Ne contient pas de logique métier

TAMR ne contient aucune logique métier spécifique. Les conditions qui déclenchent un point d'intervention sont définies par le produit selon ses règles métier.

TAMR dit : "Voici les types de conditions possibles pour déclencher une intervention."
Le produit dit : "Dans mon contexte, cette condition spécifique déclenche une approbation."

### Ne remplace pas l'automatisation

TAMR ne remplace pas l'automatisation. Il la complète en définissant où et quand l'humain peut intervenir. L'automatisation reste la norme, l'intervention humaine reste l'exception contrôlée.

### Ne gère pas la notification

TAMR ne gère pas la notification des humains. Comment un humain est informé qu'une intervention est nécessaire est la responsabilité du produit.

TAMR dit : "Un point d'approbation nécessite une réponse humaine."
Le produit dit : "J'envoie un email/notification/alerte à l'approbateur."

---

## 7. Invariants non négociables

### INV-TAMR-1 : Traçabilité absolue

**Toute intervention humaine est tracée, sans exception.**

Aucune intervention humaine ne peut se produire sans être enregistrée. Cette trace comprend au minimum : l'identité de l'intervenant, le type d'intervention, le moment, et le résultat.

Cet invariant est non contournable, même pour les interventions d'urgence ou les situations exceptionnelles.

### INV-TAMR-2 : Responsabilité explicite

**L'humain qui intervient assume explicitement la responsabilité de son intervention.**

Toute intervention engage la responsabilité de l'humain intervenant. Cette responsabilité est tracée et peut être auditée. L'humain ne peut pas intervenir anonymement ou sans assumer les conséquences de son intervention.

### INV-TAMR-3 : Limites infranchissables

**Certaines limites d'autorité sont absolues et ne peuvent être dépassées par aucune intervention humaine.**

Il existe des limites que même un override ne peut franchir. Ces limites protègent :
- L'intégrité du système
- Les données critiques
- Les règles de sécurité fondamentales
- Les contraintes légales ou réglementaires

### INV-TAMR-4 : Séparation conceptuel/technique

**TAMR reste purement conceptuel et ne définit jamais d'implémentation technique.**

TAMR ne définit pas d'interface, de protocole, d'API, ou de mécanisme technique. Il définit uniquement les concepts, types, limites, et règles de l'intervention humaine.

### INV-TAMR-5 : Non-décision

**TAMR ne prend jamais de décision, ne valide jamais d'intervention, ne refuse jamais d'intervention.**

TAMR définit les règles, mais la décision d'autoriser ou refuser une intervention appartient à StrongFather. TAMR est un cadre conceptuel, pas un moteur de décision.

### INV-TAMR-6 : Automatisation par défaut

**L'automatisation est la norme, l'intervention humaine est l'exception contrôlée.**

TAMR ne vise pas à remplacer l'automatisation par l'intervention humaine. L'intervention humaine est définie pour les cas où elle est nécessaire, pas pour éviter l'automatisation.

### INV-TAMR-7 : Justification obligatoire pour override

**Tout override nécessite une justification explicite enregistrée.**

Un override contredit une décision automatique. Cette dérogation exceptionnelle nécessite une justification qui sera tracée et auditable.

### INV-TAMR-8 : Escalade non bloquante

**Une escalade ne bloque pas indéfiniment le système.**

Une escalade élève une décision vers un niveau supérieur, mais le système doit prévoir des mécanismes pour gérer le cas où l'escalade n'est pas résolue dans un délai raisonnable (timeout, délégation automatique, rejet par défaut).

---

## 8. Interactions avec l'écosystème

### Flux d'approbation

```
1. Processus automatisé atteint un point d'approbation
2. Le système crée une demande d'approbation (intention)
3. L'intention transite par BondingBrother
4. StrongFather évalue si l'approbation est requise et par qui
5. Si requise : le produit notifie l'approbateur désigné
6. L'approbateur approuve ou refuse
7. L'intervention est tracée (identité, décision, moment, contexte)
8. Le processus reprend selon la décision
```

### Flux d'override

```
1. Décision automatique (acceptée ou refusée) émise
2. Un humain autorisé demande un override
3. L'intention d'override transite par BondingBrother
4. StrongFather évalue si l'override est autorisé
5. StrongFather vérifie que les limites infranchissables sont respectées
6. Si autorisé : l'humain fournit une justification
7. L'override est appliqué et tracé (avec justification)
8. Le processus reprend avec la décision overridée
```

### Flux d'escalade

```
1. Situation nécessitant une escalade identifiée
2. Demande d'escalade créée (intention)
3. L'intention transite par BondingBrother
4. StrongFather identifie le niveau d'escalade approprié
5. Le produit notifie le(s) responsable(s) du niveau supérieur
6. Le(s) responsable(s) prend/prennent une décision
7. L'escalade et sa résolution sont tracées
8. Le processus reprend selon la décision escaladée
```

### Flux de supervision

```
1. Processus activé pour supervision humaine
2. Le système enregistre l'état supervisé
3. L'humain superviseur observe via les interfaces produit
4. Si nécessaire : le superviseur déclenche une intervention (approval/override)
5. Toute observation et intervention sont tracées
6. La supervision peut se terminer explicitement ou par timeout
```

### Intégration avec les autres cores

| Core | Rôle dans l'intervention humaine |
|------|----------------------------------|
| StrongFather | Décide si l'intervention est autorisée |
| KindMother | Persiste les traces d'intervention |
| BondingBrother | Médiation des intentions d'intervention |
| CaringNanny | Observe l'état du système pendant l'intervention |
| BorderGuard | Définit si l'intervenant est de confiance |
| MasterButler | Expose les capacités d'intervention disponibles |
| EverBuddy | Gère l'évolution des règles d'intervention |

---

## 9. Vocabulaire canonique

### Intervention

Une **intervention** est l'action délibérée d'un humain qui modifie, valide, suspend, ou annule un processus automatisé. Toute intervention est typée, tracée, et soumise aux limites définies par TAMR.

### Intervenant

L'**intervenant** est l'humain qui effectue une intervention. Son identité est toujours tracée et il assume la responsabilité de son intervention.

### Approbation (Approval)

L'**approbation** est un type d'intervention où l'humain valide une action proposée par le système avant son exécution. L'approbation peut être acceptée ou refusée.

### Override (Dérogation)

L'**override** est un type d'intervention où l'humain contredit une décision automatique. L'override nécessite une justification et fait l'objet d'un suivi renforcé.

### Escalade (Escalation)

L'**escalade** est un type d'intervention où l'humain élève une décision vers un niveau d'autorité supérieur. L'escalade implique une chaîne de responsabilité.

### Supervision (Monitoring)

La **supervision** est un type d'intervention où l'humain observe le système avec capacité d'intervenir si nécessaire. La supervision est passive par défaut mais activable.

### Point d'intervention

Un **point d'intervention** est un moment défini dans un processus où l'intervention humaine est possible ou requise. Les points d'intervention sont déclarés par les processus et catégorisés selon les règles de TAMR.

### Limite d'autorité

Une **limite d'autorité** est une restriction sur ce que l'humain peut faire lors d'une intervention. Certaines limites sont absolues et infranchissables.

### Trace d'intervention

Une **trace d'intervention** est l'enregistrement d'une intervention comprenant : identité de l'intervenant, type d'intervention, moment, contexte, justification (si requise), et résultat.

### Justification

Une **justification** est l'explication fournie par l'humain pour une intervention exceptionnelle (notamment override). La justification est obligatoire pour certains types d'intervention et est tracée.

### Responsabilité partagée

La **responsabilité partagée** est le principe selon lequel le système et l'humain partagent la responsabilité d'une action : le système est responsable d'avoir permis ou refusé l'intervention, l'humain est responsable d'avoir effectué ou non l'intervention.

### Limite infranchissable

Une **limite infranchissable** est une limite d'autorité que même un override ne peut dépasser. Ces limites protègent l'intégrité du système, les données critiques, et les règles fondamentales.

---

## 10. Conclusion et statut contractuel

### Phrase fondatrice

**TAMR définit où, quand, et comment l'humain intervient dans le système Miyukini, garantissant que l'intervention humaine reste possible là où elle est nécessaire, impossible là où elle est dangereuse, et traçable dans tous les cas.**

Cette phrase résume l'essence de TAMR : définir le cadre conceptuel de l'intervention humaine dans un système automatisé, sans jamais devenir un décideur ou un exécuteur.

### Ce que TAMR garantit

1. **Possibilité d'intervention** : L'humain peut intervenir dans les cas définis
2. **Protection contre l'intervention abusive** : Des limites empêchent les interventions dangereuses
3. **Traçabilité complète** : Toute intervention est enregistrée et auditable
4. **Responsabilité claire** : L'intervenant assume la responsabilité de ses actions
5. **Coexistence automatisation/humain** : L'intervention complète l'automatisation sans la remplacer

### Ce que TAMR ne garantit pas

1. **Interface utilisateur** : Comment l'intervention est présentée (responsabilité produit)
2. **Notification** : Comment l'humain est informé (responsabilité produit)
3. **Authentification** : Comment l'identité est vérifiée (responsabilité produit/auth)
4. **Décision** : Si l'intervention est autorisée (responsabilité StrongFather)
5. **Persistance** : Comment les traces sont stockées (responsabilité KindMother)

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

Toute implémentation impliquant une intervention humaine doit respecter intégralement ce document. Toute évolution de TAMR doit préserver les invariants définis ici. Toute extension de TAMR doit rester fidèle à la nature conceptuelle décrite ici.

---

**Version :** 1.4  
**Date :** 2026-01-26  
**Statut :** FONDATION — Non négociable  
**Référence :** Miyukini Core System v2.4, [Miyukini Framework - Integrity & Degradation System](../../reference/Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md) (intervention humaine en T3), [Miyukini Framework - Mobile & WebApp Strategy](../../reference/Miyukini%20Framework%20-%20Mobile%20WebApp%20Strategy.md) (information utilisateur mobile), [Miyukini Framework - Security Protocols](../../reference/Miyukini%20Framework%20-%20Security%20Protocols.md) (traçabilité immédiate RT-SEC-5, information utilisateur AS-SEC-5), [Miyukini Framework - Security Levels](../../reference/Miyukini%20Framework%20-%20Security%20Levels.md) (adaptation intervention humaine selon niveau sécurité 0-4)

---

## 11. Conformité aux Lois d'Autonomie Système

Ce core respecte les **Lois d'Autonomie Système** définies dans [Miyukini Framework - Lois Autonomie Systeme.md](../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md). TAMR est **intrinsèquement compatible** avec ces lois de par sa nature purement conceptuelle.

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** ✅ **Conforme**

TAMR respecte intégralement LOI-1 :
- TAMR est un **cadre conceptuel pur** qui définit les règles d'intervention humaine sans aucune dépendance externe
- Les **types d'intervention** (Approval, Override, Escalation, Supervision) sont définis localement et ne nécessitent aucun appel réseau
- Les **limites d'autorité** sont évaluées localement par StrongFather
- Les **règles de traçabilité** s'appliquent localement, les traces étant persistées par KindMother en mode offline-first
- TAMR ne définit aucune interface, API, ou service externe — il définit uniquement des concepts

**Architecture :** TAMR est par nature indépendant de toute connexion externe. Les règles qu'il définit sont applicables que le système soit connecté ou isolé.

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

TAMR respecte intégralement LOI-2 :
- L'**intervention humaine reste possible en mode isolé** : les approbations, overrides, et supervisions peuvent être effectués localement
- Les **traces d'intervention** sont enregistrées localement et synchronisées ultérieurement (via KindMother)
- L'**INV-TAMR-8 (Escalade non bloquante)** garantit qu'une escalade ne bloque pas indéfiniment le système — des mécanismes de timeout, délégation automatique, ou rejet par défaut sont prévus
- Les **décisions d'intervention** ne dépendent pas d'une validation distante — StrongFather évalue localement si l'intervention est autorisée
- L'**isolement n'empêche pas** la prise de décision humaine, il la localise

**Architecture :** Les flux d'intervention (Section 8) fonctionnent entièrement en local. BondingBrother médiatise les intentions localement, StrongFather décide localement, et KindMother trace localement. La synchronisation avec d'autres nœuds est différée, jamais bloquante.

### Implications pour les autres lois

Bien que TAMR soit principalement concerné par LOI-1 et LOI-2, sa conception respecte également :

- **LOI-3 (État local souverain)** : Les interventions effectuées en mode isolé sont valides localement et ne seront jamais invalidées a posteriori — elles seront réconciliées explicitement si nécessaire
- **LOI-4 (Pas de temps global)** : TAMR ne définit aucune logique temporelle technique — les traces d'intervention utilisent l'horodatage local
- **LOI-5 (Coût proportionnel)** : TAMR ne définit aucune ressource consommée — c'est un cadre conceptuel sans worker ni service
- **LOI-6 (Fédération possible)** : Les règles d'intervention restent locales à chaque nœud, même dans un contexte fédéré

### Points de vigilance

Pour maintenir la conformité aux lois d'autonomie lors de l'implémentation :
- Les **produits** qui implémentent les interfaces d'intervention doivent garantir un fonctionnement offline
- Les **escalades** doivent toujours prévoir un comportement par défaut en cas de non-résolution (INV-TAMR-8)
- Les **traces d'intervention** doivent être persistées localement d'abord, synchronisées ensuite

---

## Annexe : Mini log de génération

### Warning W1 : Risque de confusion TAMR/UI

**Warning rencontré :** Risque de confusion entre le rôle conceptuel de TAMR et les interfaces utilisateur qui présentent les interventions.

**Décision prise :** Clarification explicite que TAMR ne définit aucune interface. La section 6 "Ce que TAMR ne fait PAS" liste explicitement "Ne définit pas d'interface utilisateur". L'invariant INV-TAMR-4 établit la séparation conceptuel/technique.

**Correction effectuée :** Sections 3, 6, et 10 rédigées avec cette distinction explicite.

### Warning W2 : Risque de confusion TAMR/StrongFather

**Warning rencontré :** Risque de confusion entre la définition des règles d'intervention (TAMR) et la décision d'autoriser une intervention (StrongFather).

**Décision prise :** Clarification explicite que TAMR définit les règles mais ne décide jamais. L'invariant INV-TAMR-5 établit que TAMR ne prend jamais de décision. La section 3 "Positionnement familial" clarifie la relation avec StrongFather.

**Correction effectuée :** Sections 3, 5, 6, et 7 rédigées avec cette distinction explicite.

### Ambiguïté A1 : Limites infranchissables vs limites contextuelles

**Ambiguïté rencontrée :** Comment distinguer les limites absolues des limites contextuelles ?

**Décision prise :** Les limites infranchissables sont des limites que même un override ne peut dépasser. Elles protègent l'intégrité du système, les données critiques, et les règles fondamentales. Les limites contextuelles peuvent être ajustées par le produit selon le contexte.

**Correction effectuée :** Section 4 "Concepts fondamentaux" et section 9 "Vocabulaire canonique" précisent la distinction. L'invariant INV-TAMR-3 établit l'existence de limites infranchissables.

### Ambiguïté A2 : Escalade et timeout

**Ambiguïté rencontrée :** Que se passe-t-il si une escalade n'est jamais résolue ?

**Décision prise :** L'invariant INV-TAMR-8 établit qu'une escalade ne bloque pas indéfiniment le système. Le produit doit prévoir des mécanismes pour gérer le cas où l'escalade n'est pas résolue (timeout, délégation automatique, rejet par défaut).

**Correction effectuée :** Invariant INV-TAMR-8 ajouté pour couvrir ce cas.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec StrongFather : Confirmée (complémentarité, TAMR définit, SF décide)
- ✅ Cohérence avec KindMother : Confirmée (KM persiste les traces définies par TAMR)
- ✅ Cohérence avec BondingBrother : Confirmée (BB médiation des intentions d'intervention)
- ✅ Aucune décision par TAMR : Confirmée (INV-TAMR-5)
- ✅ Aucune interface par TAMR : Confirmée (INV-TAMR-4, section 6)
- ✅ Traçabilité absolue : Confirmée (INV-TAMR-1)
- ✅ Structure imposée respectée : Confirmée (10 sections)
- ✅ Ton contractuel : Confirmée (formulations absolues)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
