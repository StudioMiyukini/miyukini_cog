# TAMR — Violations & Anti-Patterns

## 1. Introduction

### Objet du contrat

Ce document définit le **TAMR — Violations & Anti-Patterns** : un contrat normatif, non négociable, et de statut FONDATION qui établit le catalogue des violations contractuelles et des anti-patterns à éviter lors de l'implémentation ou de l'utilisation des règles d'intervention humaine définies par TAMR dans le Miyukini Core System.

Ce contrat précise ce qui constitue une violation dans le cadre de l'intervention humaine, les catégories de violations, les anti-patterns d'intervention identifiés, et les conséquences associées.

### Portée

Ce contrat s'applique à **toutes les implémentations et utilisations des règles TAMR** (intervention humaine) et définit de manière absolue :
- la définition formelle d'une violation dans le cadre TAMR,
- les catégories de violations,
- le catalogue des violations explicites,
- les anti-patterns d'intervention à éviter,
- les conséquences des violations.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat **référence et consolide** les violations définies dans :
- **[TAMR — Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md)** : Invariants INV-TAMR-1 à INV-TAMR-8
- **[TAMR — Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)** : Règles et invariants par type (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION)
- **[TAMR — Intervention Points Contract](../intervention/TAMR%20-%20Intervention%20Points%20Contract.md)** : Règles de déclaration et catégories de points
- **[TAMR — Authority Limits Contract](../boundaries/TAMR%20-%20Authority%20Limits%20Contract.md)** : Limites d'autorité et invariants INV-AL-*
- **[TAMR — Inviolable Limits Contract](../boundaries/TAMR%20-%20Inviolable%20Limits%20Contract.md)** : Limites infranchissables
- **[TAMR — Security Contract](../security/TAMR%20-%20Security%20Contract.md)** : Exigences de sécurité des interventions
- **[Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Terminologie TAMR
- **[Miyukini Conceptual References - Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)** : Principes de sécurité
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Violations des lois d'autonomie système
- **[Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)** : Niveaux T0-T4
- **[Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)** : Niveaux 0-4

Ce contrat est le **catalogue de référence** pour toutes les violations TAMR.

---

## 2. Définition d'une violation

### 2.1. Nature d'une violation

Une **violation** est un non-respect d'une règle, d'un invariant, ou d'une garantie définie dans les contrats TAMR relatifs à l'intervention humaine.

**Caractéristiques d'une violation :**

- **Contractuelle** : Une violation concerne toujours un contrat TAMR spécifique
- **Identifiable** : Une violation peut être identifiée et référencée (code VIOL-*)
- **Conséquentielle** : Une violation a des conséquences définies
- **Non-tolérable** : Une violation ne peut pas être ignorée ou tolérée

### 2.2. Gravité des violations

Les violations sont classées selon leur gravité :

**CRITIQUE :**

Violation d'un invariant fondamental TAMR (INV-TAMR-*), d'une limite infranchissable, ou d'une règle absolue. La violation compromet l'intégrité du cadre d'intervention humaine ou du système.

**MAJEURE :**

Violation d'une règle importante qui affecte le comportement des interventions (types, points, limites d'autorité) mais ne compromet pas les propriétés fondamentales de TAMR.

**MINEURE :**

Violation d'une règle secondaire qui n'affecte pas le comportement principal des interventions (ex. traçabilité incomplète, format de justification).

---

## 3. Catégories de violations

### 3.1. Violations de traçabilité

**Catégorie :** CRITIQUE

**Source :** Documentation Fondatrice (INV-TAMR-1)

**Violations :**

**VIOL-TRACE-1 : Intervention sans trace**

Une intervention humaine se produit sans être enregistrée (identité, type, moment, résultat).

*Invariant violé : INV-TAMR-1 (Traçabilité absolue)*

**VIOL-TRACE-2 : Trace modifiée ou supprimée**

Une trace d'intervention est modifiée ou supprimée après sa création.

*Invariant violé : INV-TAMR-1*

**VIOL-TRACE-3 : Trace incomplète**

Une trace ne contient pas tous les éléments obligatoires définis par le type d'intervention (intervention_id, type, identité intervenant, moment, contexte, résultat ; justification si override/escalade).

*Invariant violé : INV-TYPE-3 (Intervention Types Contract)*

### 3.2. Violations de responsabilité

**Catégorie :** CRITIQUE

**Source :** Documentation Fondatrice (INV-TAMR-2)

**Violations :**

**VIOL-RESP-1 : Intervention anonyme**

Une intervention est enregistrée sans identité de l'intervenant ou avec une identité non vérifiable.

*Invariant violé : INV-TAMR-2 (Responsabilité explicite), INV-TYPE-4 (Identité obligatoire)*

**VIOL-RESP-2 : Responsabilité non assumée**

L'intervenant n'assume pas explicitement la responsabilité de son intervention (override, escalade) lorsque c'est requis.

*Invariant violé : INV-TAMR-2, R-OVER-4*

### 3.3. Violations des limites infranchissables

**Catégorie :** CRITIQUE

**Source :** Inviolable Limits Contract, Documentation Fondatrice (INV-TAMR-3)

**Violations :**

**VIOL-INV-1 : Franchissement d'une limite infranchissable**

Une intervention (notamment un override) produit un effet qui franchit une limite infranchissable (LIM-INV-*).

*Invariant violé : INV-TAMR-3, INV-OVER-1, R-OVER-2*

**VIOL-INV-2 : Override sans vérification des limites**

Un override est appliqué sans que le système ait vérifié le respect des limites infranchissables.

*Règle violée : R-OVER-2, données de traçabilité limits_checked*

### 3.4. Violations de séparation conceptuel / technique

**Catégorie :** MAJEURE

**Source :** Documentation Fondatrice (INV-TAMR-4)

**Violations :**

**VIOL-SEP-1 : TAMR définit une implémentation technique**

Un document ou une règle présentée comme TAMR définit une interface, un protocole, une API, ou un mécanisme technique concret.

*Invariant violé : INV-TAMR-4 (Séparation conceptuel/technique)*

**VIOL-SEP-2 : Confusion TAMR / produit**

Les règles conceptuelles TAMR sont mélangées avec des choix d'implémentation produit (UI, notification, auth) sans séparation claire.

*Invariant violé : INV-TAMR-4*

### 3.5. Violations de non-décision

**Catégorie :** CRITIQUE

**Source :** Documentation Fondatrice (INV-TAMR-5)

**Violations :**

**VIOL-DEC-1 : TAMR prend une décision**

Un composant ou processus étiqueté TAMR autorise, refuse ou valide une intervention. La décision appartient à StrongFather.

*Invariant violé : INV-TAMR-5 (Non-décision), INV-TYPE-6, INV-AL-5*

**VIOL-DEC-2 : TAMR exécute une intervention**

Un composant TAMR exécute ou persiste une intervention. L'exécution est la responsabilité du produit ; la persistance est celle de KindMother.

*Invariant violé : INV-TYPE-5*

### 3.6. Violations par type d'intervention

**Catégorie :** MAJEURE à CRITIQUE

**Source :** Intervention Types Contract

**Violations — APPROVAL :**

**VIOL-APPR-1 : Approbation sans identité**

Une approbation est enregistrée sans identité de l'approbateur.

*Règle violée : R-APPR-1*

**VIOL-APPR-2 : Réponse multiple sur une même demande**

Une demande d'approbation reçoit plus d'une réponse valide.

*Règle violée : R-APPR-2*

**VIOL-APPR-3 : Comportement par défaut (timeout) non défini**

Une demande d'approbation peut expirer sans que le comportement par défaut (refus ou approbation) soit explicitement défini.

*Règle violée : R-APPR-4, INV-APPR-1*

**Violations — OVERRIDE :**

**VIOL-OVER-1 : Override sans justification**

Un override est enregistré sans justification explicite.

*Règle violée : R-OVER-1, INV-TAMR-7*

**VIOL-OVER-2 : Override sans décision automatique préalable**

Un override est effectué alors qu'aucune décision automatique n'a été préalablement enregistrée pour le sujet concerné.

*Règle violée : R-OVER-3*

**VIOL-OVER-3 : Override franchissant une limite infranchissable**

Un override est appliqué alors que son effet franchit une limite infranchissable.

*Règle violée : R-OVER-2, INV-OVER-1*

**Violations — ESCALATION :**

**VIOL-ESC-1 : Escalade bloquante indéfinie**

Une escalade bloque le système sans mécanisme de timeout, délégation automatique, ou rejet par défaut.

*Invariant violé : INV-TAMR-8, R-ESC-2*

**VIOL-ESC-2 : Escalade sans motif explicite**

Une escalade est initiée sans motif explicite justifiant le recours au niveau supérieur.

*Règle violée : R-ESC-3*

**VIOL-ESC-3 : Escalade vers un destinataire non défini**

Une escalade est dirigée vers un niveau ou un destinataire non défini dans la chaîne de responsabilité.

*Règle violée : R-ESC-1*

**Violations — SUPERVISION :**

**VIOL-SUP-1 : Supervision sans identité du superviseur**

Une supervision est enregistrée sans identité du superviseur.

*Règle violée : R-SUP-1*

**VIOL-SUP-2 : Supervision sans durée ou périmètre défini**

Une supervision est activée sans périmètre défini ou sans durée (explicite ou timeout).

*Règle violée : R-SUP-2, R-SUP-3*

**VIOL-SUP-3 : Supervision modifiant le comportement en mode passif**

En état passif, la supervision modifie le comportement du système.

*Règle violée : R-SUP-4*

### 3.7. Violations des points d'intervention

**Catégorie :** MAJEURE

**Source :** Intervention Points Contract, Authority Limits Contract (INV-AL-4)

**Violations :**

**VIOL-POINT-1 : Intervention hors point déclaré**

Une intervention est effectuée alors qu'elle ne s'inscrit pas dans un point d'intervention déclaré pour le processus concerné.

*Invariant violé : INV-AL-4*

**VIOL-POINT-2 : Point d'intervention non catégorisé**

Un point d'intervention est utilisé sans appartenir à une des catégories TAMR (DECISION_GATE, ANOMALY_RESPONSE, etc.).

*Contrat violé : Intervention Points Contract (catégories)*

**VIOL-POINT-3 : Type d'intervention non autorisé au point**

Un type d'intervention (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) est utilisé à un point qui ne déclare pas ce type.

*Contrat violé : Intervention Points Contract*

### 3.8. Violations des limites d'autorité

**Catégorie :** MAJEURE

**Source :** Authority Limits Contract

**Violations :**

**VIOL-AL-1 : Évaluation des limites hors hiérarchie**

Une intervention est autorisée alors qu'elle franchirait une limite infranchissable (les limites d'autorité s'appliquent en deçà des limites infranchissables).

*Invariant violé : INV-AL-1*

**VIOL-AL-2 : Contexte de sécurité réduisant les restrictions**

Une configuration ou un contexte fait qu'un niveau de sécurité plus élevé réduit les restrictions d'autorité (monotonie 0→4).

*Invariant violé : INV-AL-2*

**VIOL-AL-3 : Contexte de confiance réduisant les restrictions**

Une configuration ou un contexte fait qu'un niveau de confiance plus élevé (T0→T4) réduit les restrictions d'autorité.

*Invariant violé : INV-AL-3*

**VIOL-AL-4 : Évaluation sans point déclaré ou sans role reconnu**

Une évaluation des limites d'autorité est effectuée pour une intervention qui n'est pas dans un point déclaré ou pour un intervenant sans rôle reconnu.

*Invariant violé : INV-AL-4*

### 3.9. Violations des types (liste fermée)

**Catégorie :** MAJEURE

**Source :** Intervention Types Contract

**Violations :**

**VIOL-TYPE-1 : Type d'intervention non reconnu**

Une intervention est catégorisée sous un type qui n'est pas l'un des quatre reconnus (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION).

*Invariant violé : INV-TYPE-1*

**VIOL-TYPE-2 : Intervention à plusieurs types simultanés**

Une même intervention est enregistrée comme appartenant à plusieurs types.

*Invariant violé : INV-TYPE-2*

**VIOL-REL-1 : Chaîne d'interventions circulaire**

Une intervention A déclenche une intervention B qui déclenche A (circularité).

*Règle violée : R-REL-3*

### 3.10. Violations de justification et automatisation

**Catégorie :** MAJEURE

**Source :** Documentation Fondatrice (INV-TAMR-6, INV-TAMR-7)

**Violations :**

**VIOL-JUST-1 : Override sans justification enregistrée**

Tout override nécessite une justification explicite enregistrée ; l'absence de justification est une violation.

*Invariant violé : INV-TAMR-7*

**VIOL-AUTO-1 : Intervention humaine comme norme par défaut**

Le système est conçu pour que l'intervention humaine soit la norme au lieu de l'exception contrôlée.

*Invariant violé : INV-TAMR-6 (Automatisation par défaut)*

---

## 4. Anti-patterns

### 4.1. Anti-pattern : TAMR comme décideur

**Description :**

Faire porter à un composant ou processus TAMR la décision d'autoriser ou refuser une intervention. Confondre le cadre conceptuel (TAMR) avec le moteur de décision (StrongFather).

**Pourquoi c'est un anti-pattern :**

TAMR définit les règles et les types d'intervention ; StrongFather évalue les politiques et décide. Faire décider TAMR viole INV-TAMR-5 et crée un couplage incorrect.

**Symptômes :**

- Un module « TAMR » retourne « autorisé » ou « refusé » pour une intervention
- Les politiques d'autorisation sont codées dans le même composant que les définitions de types/points
- L'évaluation des limites d'autorité est faite par le composant qui définit les limites conceptuelles

**Solution :**

TAMR expose uniquement des définitions (types, points, limites). StrongFather (ou un moteur de politique qui l'implémente) évalue et décide. Le produit appelle StrongFather pour l'autorisation, pas TAMR.

### 4.2. Anti-pattern : Intervention sans trace

**Description :**

Permettre une intervention humaine (approbation, override, escalade, supervision) sans enregistrement immédiat et complet de la trace.

**Pourquoi c'est un anti-pattern :**

Toute intervention doit être tracée sans exception (INV-TAMR-1). L'absence de trace rend l'audit et la responsabilité impossibles.

**Symptômes :**

- « On tracera plus tard » ou « en batch »
- Trace optionnelle selon le type d'intervention
- Identité ou contexte omis pour « simplifier »

**Solution :**

Toute intervention produit une trace complète avant que l'effet ne soit appliqué. La persistance (KindMother) est appelée selon les structures définies par TAMR.

### 4.3. Anti-pattern : Override sans justification

**Description :**

Accepter ou implémenter un override sans champ de justification obligatoire, ou avec une justification vide.

**Pourquoi c'est un anti-pattern :**

INV-TAMR-7 et R-OVER-1 imposent une justification explicite pour tout override. Sans justification, la dérogation n'est pas auditable ni responsable.

**Symptômes :**

- Bouton « Override » sans zone de saisie de justification
- Justification optionnelle ou « à remplir plus tard »
- Override en masse sans justification par sujet

**Solution :**

Chaque override exige une justification saisie par l'intervenant, enregistrée dans la trace et non modifiable après enregistrement.

### 4.4. Anti-pattern : Escalade sans fin

**Description :**

Mettre en place une escalade sans comportement par défaut (timeout, délégation automatique, rejet) en cas de non-résolution, bloquant indéfiniment le flux.

**Pourquoi c'est un anti-pattern :**

INV-TAMR-8 et R-ESC-2 exigent qu'une escalade ne bloque pas indéfiniment le système. L'absence de mécanisme de terminaison viole le contrat.

**Symptômes :**

- Processus en attente « jusqu'à réponse » sans délai maximal
- Pas de délégation automatique ni de rejet par défaut configuré
- Escalade « en boucle » sans niveau final

**Solution :**

Chaque escalade a une durée maximale et un comportement explicite en cas d'expiration (rejet, approbation par défaut, délégation). La chaîne d'escalade a un niveau terminal.

### 4.5. Anti-pattern : Contournement des limites infranchissables

**Description :**

Permettre à un override (ou à toute intervention) de produire un effet qui franchit une limite infranchissable (audit désactivé, suppression de données d'audit, etc.) sous prétexte d'urgence ou de rôle élevé.

**Pourquoi c'est un anti-pattern :**

Les limites infranchissables (INV-TAMR-3, Inviolable Limits Contract) ne peuvent jamais être franchies. Un contournement compromet l'intégrité du système.

**Symptômes :**

- « Super-admin peut tout faire »
- Override accepté sans vérification des LIM-INV-*
- Désactivation de la traçabilité « temporaire » par intervention

**Solution :**

StrongFather refuse toute intervention dont l'effet franchirait une limite infranchissable. Aucune exception, aucun rôle ne peut overrider ces limites.

### 4.6. Anti-pattern : Approbation ou supervision anonyme

**Description :**

Enregistrer une approbation ou une supervision sans identité fiable de l'intervenant (anonymat, compte générique, identité non vérifiée).

**Pourquoi c'est un anti-pattern :**

INV-TAMR-2 et INV-TYPE-4 exigent que toute intervention identifie l'intervenant et que la responsabilité soit assumée. L'anonymat rend la responsabilité impossible.

**Symptômes :**

- Compte « system » ou « approbateur » partagé
- Intervention sans authentification préalable
- Trace avec identité vide ou non vérifiée

**Solution :**

Chaque intervention est associée à une identité vérifiée (auth). Les traces contiennent obligatoirement l'identifiant de l'intervenant ; les comptes partagés pour l'approbation sont proscrits.

### 4.7. Anti-pattern : TAMR définit l'UI ou la technique

**Description :**

Inclure dans la documentation ou les règles TAMR des spécifications d'interface utilisateur, de protocole, d'API, ou de mécanisme technique (notification, auth, stockage).

**Pourquoi c'est un anti-pattern :**

INV-TAMR-4 impose que TAMR reste purement conceptuel. L'implémentation technique (écrans, APIs, persistance) relève des produits et de KindMother.

**Symptômes :**

- Contrat TAMR qui décrit des écrans ou des champs de formulaire
- TAMR qui impose un protocole (REST, WebSocket) ou un format de stockage
- Mélange des règles conceptuelles et des choix d'implémentation dans le même document

**Solution :**

TAMR ne décrit que les concepts (types, points, limites, traçabilité). Les guides d'implémentation (Reference Implementation Guidelines) traduisent ces concepts en recommandations sans les figer en normes techniques dans le contrat TAMR.

### 4.8. Anti-pattern : Intervention hors point déclaré

**Description :**

Permettre une intervention à un endroit du processus qui n'a pas été déclaré comme point d'intervention, ou avec un type non autorisé pour ce point.

**Pourquoi c'est un anti-pattern :**

INV-AL-4 et l'Intervention Points Contract exigent que les interventions s'inscrivent dans des points déclarés et catégorisés. Les interventions « sauvages » violent le contrat et rendent l'audit incohérent.

**Symptômes :**

- Bouton « Approuver » ou « Overrider » disponible partout sans déclaration de point
- Points implicites ou « on demande quand même une approbation » hors processus déclaré
- Types d'intervention utilisés à un point qui ne les déclare pas

**Solution :**

Chaque processus déclare explicitement ses points d'intervention (catégorie, types autorisés). Seules les interventions sur ces points et avec ces types sont acceptées et tracées.

---

## 5. Conséquences des violations

### 5.1. Violations critiques

**Conséquences :**

1. **Non-conformité immédiate** : L'implémentation est considérée non conforme à TAMR
2. **Arrêt requis** : L'intervention en cours ne doit pas être appliquée (ou doit être annulée si déjà appliquée)
3. **Audit obligatoire** : Un audit des interventions et des traces doit être effectué
4. **Correction impérative** : La correction est obligatoire avant toute mise en production ou poursuite d'utilisation

### 5.2. Violations majeures

**Conséquences :**

1. **Avertissement de non-conformité** : L'implémentation est signalée comme non conforme
2. **Intervention invalide** : L'intervention associée est considérée invalide (ne doit pas être traitée comme autorisée sans correction)
3. **Correction requise** : La correction doit être planifiée et réalisée dans un délai défini

### 5.3. Violations mineures

**Conséquences :**

1. **Signalement** : La violation est signalée (logs, monitoring)
2. **Correction recommandée** : La correction est recommandée
3. **Traçabilité** : La violation est tracée pour suivi et amélioration continue

---

## 6. Règles de fermeture du contrat

### 6.1. Contrat fermé

Ce contrat est **fermé**. Seules les violations et les anti-patterns explicitement définis sont reconnus.

### 6.2. Catalogue de référence

Ce contrat est le **catalogue de référence** pour toutes les violations TAMR. Toute nouvelle violation identifiée doit être ajoutée à ce catalogue selon le processus d'évolution des contrats TAMR.

---

## 7. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable le catalogue des violations et anti-patterns relatifs à l'intervention humaine (TAMR).

Il garantit que :
- les violations sont exhaustivement cataloguées et référencées aux contrats sources,
- les anti-patterns d'intervention sont identifiés et documentés,
- les conséquences sont explicites selon la gravité,
- le contrat est fermé et constitue la référence unique pour les violations TAMR.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 8. Validation conceptuelle

### 8.1. Vérification de complétude

Ce document catalogue les violations en lien avec :
- ✅ Documentation Fondatrice : INV-TAMR-1 à INV-TAMR-8 (VIOL-TRACE-*, VIOL-RESP-*, VIOL-INV-*, VIOL-DEC-*, VIOL-JUST-*, VIOL-AUTO-*)
- ✅ Intervention Types Contract : R-APPR-*, R-OVER-*, R-ESC-*, R-SUP-*, INV-TYPE-*, INV-APPR-1, INV-OVER-*, R-REL-* (VIOL-APPR-*, VIOL-OVER-*, VIOL-ESC-*, VIOL-SUP-*, VIOL-TYPE-*, VIOL-REL-*)
- ✅ Intervention Points Contract : points déclarés, catégories, types autorisés (VIOL-POINT-*)
- ✅ Authority Limits Contract : INV-AL-1 à INV-AL-5 (VIOL-AL-*)
- ✅ Inviolable Limits Contract : LIM-INV-*, non-franchissement (VIOL-INV-*)
- ✅ Séparation conceptuel/technique : INV-TAMR-4 (VIOL-SEP-*)

### 8.2. Vérification de cohérence

- ✅ Toutes les violations référencent un contrat ou un invariant source
- ✅ Les gravités sont cohérentes avec l'importance des règles (critique pour traçabilité, responsabilité, limites infranchissables, non-décision)
- ✅ Les anti-patterns couvrent les thèmes : décision, trace, justification, escalade, limites, anonymat, UI/technique, points déclarés

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, TAMR Documentation Fondatrice v1.4  
**Type :** Catalogue des violations et anti-patterns TAMR non négociable

---

## 9. Mini log de génération

### Décision éditoriale E1 : Consolidation des violations

**Décision prise :** Consolidation des violations dispersées dans les contrats TAMR (Documentation Fondatrice, Intervention Types, Intervention Points, Authority Limits, Inviolable Limits) en un catalogue unique.

**Application :** Chaque violation référence son contrat et, le cas échéant, l'invariant ou la règle source (INV-TAMR-*, INV-TYPE-*, INV-AL-*, R-*-*, LIM-INV-*).

### Décision éditoriale E2 : Anti-patterns d'intervention

**Décision prise :** Inclusion d'anti-patterns spécifiques à l'intervention humaine : TAMR comme décideur, intervention sans trace, override sans justification, escalade sans fin, contournement des limites infranchissables, anonymat, TAMR technique, intervention hors point déclaré.

**Application :** 8 anti-patterns décrits avec description, symptômes et solution.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Violations alignées sur INV-TAMR-1 à INV-TAMR-8 et sur les contrats intervention, boundaries, security
- ✅ Références aux documents du plan (Glossaire, Doctrine Sécurité, Lois Autonomie, Integrity Degradation, Security Levels) intégrées en section 1
- ✅ Ton contractuel et statut FONDATION maintenus

**Conclusion :** Catalogue complet et cohérent avec les contrats TAMR existants.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
