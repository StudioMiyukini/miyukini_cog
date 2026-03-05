# LogisticsSteward â€” FAQ & Common Questions

## 1. Contexte

Ce document repond aux **questions frequentes** sur LogisticsSteward. Il clarifie les confusions courantes, precise les limites du core, et aide a comprendre rapidement son role dans l'ecosysteme Miyukini.

Ce document est destine aux :
- Developpeurs implementant ou integrant LogisticsSteward
- Architectes concevant des systemes utilisant LogisticsSteward
- Nouveaux membres de l'equipe decouvrant l'ecosysteme

## 2. Portee / Scope

Ce document couvre :
- Les questions sur l'identite et le role de LogisticsSteward
- Les confusions frequentes sur ce qu'il fait et ne fait pas
- Les questions sur les relations avec les autres cores
- Les questions pratiques sur les quotas, priorites et arbitrage
- Les questions sur la degradation et la resilience
- Les questions sur l'implementation

Ce document **ne couvre pas** :
- Les specifications detaillees (voir contrats specifiques)
- Les procedures d'implementation (voir Implementation Guidelines)
- Les definitions formelles (voir Documentation Fondatrice)

---

## 3. Questions sur l'Identite et le Role

### Q1 : Qu'est-ce que LogisticsSteward en une phrase ?

**R :** LogisticsSteward est le core qui **gouverne l'usage des ressources** en decidant qui a droit a quoi, quand, et a quel niveau de priorite â€” sans jamais executer ni controler techniquement.

---

### Q2 : Pourquoi "Steward" et pas "Manager" ou "Controller" ?

**R :** Le terme "Steward" (intendant) est choisi deliberement :

| Terme | Implication | Pourquoi pas |
|-------|-------------|--------------|
| **Manager** | Implique execution et controle | LogisticsSteward ne gere pas techniquement |
| **Controller** | Implique controle bas niveau | LogisticsSteward n'a aucun pouvoir technique |
| **Steward** | Implique gouvernance et arbitrage | Exact : il gouverne selon des regles |

Un intendant (steward) sait ce qui est disponible, qui peut utiliser quoi, et selon quelles regles â€” mais il ne fait pas le travail lui-meme.

---

### Q3 : Quelle est la question fondamentale a laquelle repond LogisticsSteward ?

**R :** 

> **"Qui a le droit d'utiliser quoi, quand, et a quel niveau de priorite ?"**

Cette question se decline en :
- Quels quotas s'appliquent a chaque entite ?
- Quelles priorites relatives gouvernent l'arbitrage ?
- Quels plafonds d'usage sont en vigueur ?
- Quelle strategie de degradation appliquer en cas de surcharge ?

---

### Q4 : Dans la metaphore de l'Etat, quel ministere represente LogisticsSteward ?

**R :** LogisticsSteward est le **ministere du budget et des ressources**. Il connait les regles d'allocation, sait qui peut utiliser quoi, definit les priorites et les limites â€” sans construire les routes (Kernel), appliquer les lois (StrongFather), ou livrer le courrier (BondingBrother).

---

### Q5 : A quelle strate appartient LogisticsSteward ?

**R :** LogisticsSteward est positionne en **Strate 3 (Gouvernance Ressources)**, entre :
- **Strate 2** (MasterButler - Capacites) en dessous
- **Strate 4** (StrongFather, KindMother, WorrySentinel - Cores Systeme) au-dessus

Cette position lui permet de limiter l'usage des capacites exposees par MasterButler, tout en soumettant ses decisions a validation par StrongFather.

---

## 4. Questions sur ce que LogisticsSteward Fait et Ne Fait Pas

### Q6 : LogisticsSteward peut-il mesurer la memoire ou le CPU ?

**R :** **Non, jamais.** C'est une interdiction absolue (INTERD-LS-1).

LogisticsSteward ne mesure jamais directement les ressources systeme. Il recoit un **etat systeme abstrait** fourni par le Kernel, qui contient des informations normalisees comme :
- Niveau de charge global (faible / normal / eleve / critique)
- Disponibilite relative des ressources
- Seuils de securite atteints ou proches

C'est le Kernel qui mesure et certifie cet etat.

---

### Q7 : LogisticsSteward peut-il allouer de la memoire ou creer des threads ?

**R :** **Non, jamais.** Ce sont des interdictions absolues (INTERD-LS-3, INTERD-LS-4).

LogisticsSteward n'a aucune capacite technique. Il produit des **decisions** qui sont ensuite executees par le Kernel. La separation est absolue.

---

### Q8 : Quelle est la difference entre LogisticsSteward et un scheduler ?

**R :** 

| Aspect | LogisticsSteward | Scheduler |
|--------|------------------|-----------|
| **Role** | Gouverne l'allocation | Planifie l'execution |
| **Moment** | Avant execution | Pendant execution |
| **Pouvoir** | Decide les droits | Ordonne les taches |
| **Technique** | Aucun | Manipulation de threads/quantum |

LogisticsSteward decide qui a droit a quoi. Le scheduler (gere par le Kernel) decide dans quel ordre executer.

---

### Q9 : LogisticsSteward peut-il "optimiser" les performances ?

**R :** **Non.** L'optimisation n'est pas son role (INTERD-LS-6).

LogisticsSteward **gouverne** selon des regles explicites, il n'**optimise** pas. Si vous cherchez a ameliorer les performances par des heuristiques ou du tuning, ce n'est pas le role de LogisticsSteward.

La gouvernance n'est pas l'optimisation. La gouvernance dit "tu as droit a X". L'optimisation cherche a faire mieux avec X.

---

### Q10 : LogisticsSteward stocke-t-il un etat operationnel ?

**R :** **Non.** LogisticsSteward est conceptuellement **stateless** (INTERD-LS-7).

Il recoit l'etat systeme du Kernel a chaque evaluation et les regles depuis la source de politique. Il ne maintient pas d'etat operationnel entre les arbitrages.

Note : Les regles elles-memes sont persistees par KindMother, pas par LogisticsSteward.

---

## 5. Questions sur les Relations avec les Autres Cores

### Q11 : Pourquoi les decisions de LogisticsSteward doivent-elles etre validees par StrongFather ?

**R :** Parce que LogisticsSteward est un **arbitre**, pas une **autorite**.

La separation des pouvoirs est fondamentale :
- LogisticsSteward **propose** une decision basee sur les regles et l'etat
- StrongFather **valide ou invalide** cette decision selon les politiques globales
- Le Kernel **execute** la decision validee

Cette validation garantit la coherence globale et empeche les decisions contradictoires avec d'autres aspects du systeme.

---

### Q12 : Quelle est la difference entre LogisticsSteward et MasterButler ?

**R :** 

| Aspect | MasterButler | LogisticsSteward |
|--------|--------------|------------------|
| **Question** | "Quelles capacites existent ?" | "Qui peut les utiliser ?" |
| **Expose** | Catalogue des capacites | Regles d'usage |
| **Dit** | "Cette API existe" | "Tu as droit a 100 appels/minute" |
| **Nature** | Existence | Autorisation |

MasterButler dit ce qui **existe**, LogisticsSteward limite ce qui est **autorise**.

---

### Q13 : Comment LogisticsSteward interagit-il avec WorrySentinel ?

**R :** WorrySentinel **surveille** et peut **declencher** des adaptations :

| WorrySentinel detecte... | LogisticsSteward reagit... |
|-------------------------|---------------------------|
| Derive anormale | Durcissement des regles |
| Comportement suspect | Restriction temporaire |
| Etat incoherent | Mode prudent |
| Menace identifiee | Regles de protection |

WorrySentinel peut invalider un etat juge incoherent, demander un durcissement des regles, ou alerter sur des comportements suspects. LogisticsSteward adapte ses regles en consequence.

---

### Q14 : Comment LogisticsSteward communique-t-il ses decisions ?

**R :** Via **BondingBrother**, jamais directement.

BondingBrother transporte les decisions d'arbitrage vers les entites concernees sans les interpreter. LogisticsSteward ne connait pas les operateurs directement â€” il produit des decisions, BondingBrother les livre.

---

### Q15 : Quelle est la relation avec le Kernel ?

**R :** Le Kernel est le **fournisseur d'etat** et l'**executeur** :

| Direction | Flux |
|-----------|------|
| Kernel â†’ LogisticsSteward | Etat systeme abstrait (lecture seule) |
| LogisticsSteward â†’ Kernel | Decisions d'arbitrage a executer |

LogisticsSteward **consomme** l'etat, le Kernel **l'execute**. Jamais l'inverse. LogisticsSteward ne modifie jamais l'etat systeme.

---

## 6. Questions sur les Quotas et Priorites

### Q16 : Qu'est-ce qu'un quota dans LogisticsSteward ?

**R :** Un **quota** est une limite declaree sur l'usage d'une ressource conceptuelle par une entite.

Exemples :
- 1000 requetes API par heure pour un operateur
- 500 Mo d'espace de travail pour une equipe
- 50 operations simultanees pour un service

Les quotas sont :
- **Explicites** : declares, pas deduits
- **Auditables** : tracables et verifiables
- **Deterministes** : comportement previsible
- **Non techniques** : conceptuels, pas bas niveau

---

### Q17 : Quelle est la difference entre quota, plafond et restriction ?

**R :** 

| Concept | Definition | Exemple |
|---------|------------|---------|
| **Quota** | Limite attribuee a une entite | Operateur X : 1000 req/h |
| **Plafond** | Limite maximale absolue du systeme | Maximum 10000 req/h global |
| **Restriction** | Limitation temporaire contextuelle | Operateur Y : bloque 24h |

- Un quota peut etre augmente (dans la limite du plafond)
- Un plafond ne peut pas etre depasse
- Une restriction est temporaire et conditionnelle

---

### Q18 : Comment fonctionne la preemption ?

**R :** La preemption permet a une demande de haute priorite d'interrompre une allocation de priorite inferieure.

**Conditions :**
1. Priorite strictement superieure
2. Ressources liberees suffisantes
3. Entite preemptee pas en operation critique
4. Niveau de degradation autorise la preemption

**Exception :** MiyukiniAdmin ne peut jamais etre preempte (sauf mode survie D4).

---

### Q19 : MiyukiniAdmin peut-il depasser les quotas ?

**R :** MiyukiniAdmin peut **demander** des priorites maximales et des exceptions, mais :
- Il reste soumis a la gouvernance globale
- Toute exception necessite un protocole explicite
- Toute exception est tracee et auditable
- Il ne peut pas **imposer**, seulement **demander**

MiyukiniAdmin n'est pas au-dessus de LogisticsSteward. Il peut demander des exceptions, pas les imposer.

---

## 7. Questions sur la Degradation

### Q20 : Qu'est-ce que la degradation dans LogisticsSteward ?

**R :** La **degradation** est la reduction controlee et explicite des capacites du systeme en reponse a une charge elevee ou des ressources limitees.

La degradation est :
- **Un choix**, pas un accident
- **Controlee**, jamais chaotique
- **Progressive**, par paliers
- **Reversible**, retour a la normale possible
- **Explicite**, annoncee et justifiee

---

### Q21 : Quels sont les niveaux de degradation ?

**R :** 

| Niveau | Nom | Description | Actions typiques |
|--------|-----|-------------|------------------|
| **D0** | Normal | Aucune degradation | Toutes capacites disponibles |
| **D1** | Prudent | Charge elevee | Limitation operations non critiques |
| **D2** | Restreint | Ressources limitees | Desactivation fonctionnalites secondaires |
| **D3** | Critique | Risque de saturation | Services minimaux uniquement |
| **D4** | Survie | Etat d'urgence | Preservation coeur systeme uniquement |

---

### Q22 : Qui decide du niveau de degradation ?

**R :** Le niveau de degradation est determine par :
1. **L'etat systeme** fourni par le Kernel
2. **Les politiques de degradation** declarees
3. **Les alertes de WorrySentinel** eventuelles
4. **La validation de StrongFather** pour les changements

LogisticsSteward applique les regles de degradation, il ne les invente pas.

---

### Q23 : La degradation peut-elle etre automatique ?

**R :** Oui, si les regles de degradation le prevoient explicitement.

Mais attention :
- Les regles de degradation automatique sont declarees a l'avance
- La degradation automatique est tracee comme toute decision
- StrongFather peut invalider une degradation automatique
- La degradation reste un choix explicite (dans les regles), pas un accident

---

## 8. Questions sur l'Arbitrage

### Q24 : Qu'est-ce qu'un arbitrage ?

**R :** L'**arbitrage** est le processus par lequel LogisticsSteward decide de l'allocation, de la priorite et de la limitation des ressources pour une entite donnee.

Un arbitrage :
- Recoit une demande de ressource
- Lit l'etat systeme (Kernel)
- Evalue les regles applicables
- Produit une decision justifiee
- Soumet a validation (StrongFather)
- Emet la decision finale

---

### Q25 : Quels sont les verdicts possibles d'un arbitrage ?

**R :** 

| Verdict | Description |
|---------|-------------|
| **ACCORDE** | Demande integralement acceptee |
| **REFUSE** | Demande rejetee |
| **PARTIEL** | Demande partiellement acceptee |
| **DIFFERE** | Demande mise en attente (preemption possible) |

Chaque verdict inclut une justification detaillee avec les regles appliquees.

---

### Q26 : Que se passe-t-il si StrongFather invalide une decision ?

**R :** Si StrongFather invalide une decision :
- La decision est rejetee definitivement pour cette demande
- Le demandeur est notifie de l'invalidation
- La raison de l'invalidation est tracee
- Le demandeur peut reformuler sa demande

StrongFather peut invalider mais aussi **durcir** (jamais assouplir) une decision.

---

### Q27 : L'arbitrage est-il deterministe ?

**R :** **Oui, absolument.** C'est un invariant fondamental (INV-LS-4).

A entrees identiques (demande, etat systeme, regles), l'arbitrage produit **toujours** la meme decision. Il n'y a aucune source d'aleatoire dans le processus.

---

## 9. Questions sur la Resilience et l'Autonomie

### Q28 : LogisticsSteward peut-il fonctionner hors ligne ?

**R :** **Oui.** C'est une exigence des Lois d'Autonomie Systeme.

LogisticsSteward fonctionne avec :
- L'etat systeme local fourni par le Kernel
- Les regles locales chargees au demarrage
- Aucune dependance a un service externe

L'isolement est un etat normal de fonctionnement, pas une exception.

---

### Q29 : Que se passe-t-il si le Kernel ne fournit pas l'etat systeme ?

**R :** LogisticsSteward passe en **mode degrade prudent** :
- Regles minimales appliquees
- Decisions conservatrices
- Tracabilite maintenue
- Attente de retour a la normale

L'absence d'etat certifie ne bloque jamais completement l'arbitrage.

---

### Q30 : LogisticsSteward depend-il d'un temps global ?

**R :** **Non.** C'est une exigence des Lois d'Autonomie (LOI-4).

Les decisions sont basees sur l'etat actuel, pas sur des timestamps. Les quotas sont evalues localement. LogisticsSteward est compatible avec des horloges desynchronisees.

---

## 10. Questions sur l'Implementation

### Q31 : Quelles sont les erreurs les plus courantes d'implementation ?

**R :** Les erreurs les plus frequentes sont :

1. **Mesurer les ressources directement** au lieu d'utiliser l'etat Kernel
2. **Executer des actions** au lieu de produire des decisions
3. **Bypasser StrongFather** pour des decisions "urgentes"
4. **Maintenir un etat operationnel** entre les arbitrages
5. **Creer des regles implicites** basees sur des heuristiques

Voir le document [Violations & Anti-Patterns](../contracts/governance/LogisticsSteward%20-%20Violations%20&%20Anti-Patterns.md) pour le catalogue complet.

---

### Q32 : Comment detecter si une implementation viole les invariants ?

**R :** Plusieurs mecanismes :

| Detection | Methode |
|-----------|---------|
| **Statique** | Recherche d'imports systeme, API bas niveau |
| **Dynamique** | Verification que toute decision a une validation SF |
| **Audit** | Revue de conformite aux invariants |

Signaux d'alerte :
- Import `std::os`, `libc`, `nix`
- Import `thread::spawn`
- Variable `static mut`
- Condition sur nom/type d'entite

---

### Q33 : LogisticsSteward doit-il etre single-threaded ?

**R :** **Conceptuellement, oui.** LogisticsSteward ne gere pas de threads.

L'implementation technique peut utiliser la concurrence si necessaire, mais LogisticsSteward ne doit jamais **creer, gerer ou manipuler** des threads. La concurrence est la responsabilite du Kernel.

---

### Q34 : Quelles dependances sont autorisees ?

**R :** 

| Autorise | Interdit |
|----------|----------|
| Structures de donnees standard | Librairies systeme (psutil, nix, etc.) |
| Serialisation (serde) | API d'allocation memoire |
| Logging (tracabilite) | Threading direct |
| Validation de schema | Acces fichiers systeme |

Aucune dependance a des API systeme ou des librairies bas niveau.

---

## 11. Questions sur la Tracabilite

### Q35 : Toute decision est-elle tracee ?

**R :** **Oui, absolument.** C'est un invariant fondamental (INV-LS-6).

Chaque decision d'arbitrage produit une trace complete incluant :
- Identifiant de la demande
- Etat systeme au moment de l'arbitrage
- Regles evaluees et resultats
- Decision finale avec justification
- Validation StrongFather
- Timestamp

---

### Q36 : Comment auditer les decisions de LogisticsSteward ?

**R :** Les traces d'arbitrage permettent de :
- Reconstruire n'importe quelle decision
- Verifier les regles appliquees
- Identifier les patterns de decision
- Detecter les anomalies
- Valider la conformite

Chaque trace contient suffisamment d'information pour reproduire la decision.

---

### Q37 : Les decisions peuvent-elles etre contestees ?

**R :** Les decisions peuvent etre **auditees** et **expliquees**, pas "contestees" au sens juridique :
- La trace montre exactement pourquoi la decision a ete prise
- Si la decision respecte les regles, elle est correcte
- Si les regles sont inadaptees, c'est les regles qu'il faut modifier
- Si la decision viole les regles, c'est un bug d'implementation

---

## 12. Questions Diverses

### Q38 : LogisticsSteward peut-il etre desactive ?

**R :** LogisticsSteward ne peut pas etre "desactive" dans le sens ou un systeme Miyukini a besoin de gouvernance des ressources.

Cependant, les regles peuvent etre configurees pour etre tres permissives (quotas eleves, pas de restrictions), ce qui revient a une gouvernance minimale.

---

### Q39 : Quelle est la difference entre LogisticsSteward et un WAF/Rate Limiter ?

**R :** 

| Aspect | Rate Limiter / WAF | LogisticsSteward |
|--------|-------------------|------------------|
| **Niveau** | Technique (HTTP, IP) | Conceptuel (entites, ressources) |
| **Execution** | Bloque/autorise techniquement | Decide sans executer |
| **Scope** | Requetes reseau | Toutes ressources conceptuelles |
| **Validation** | Auto-applique | Soumis a StrongFather |

LogisticsSteward gouverne a un niveau plus abstrait. Un rate limiter est un mecanisme technique d'execution.

---

### Q40 : Comment contribuer a la documentation de LogisticsSteward ?

**R :** Toute contribution doit respecter :
- Les conventions de nomenclature Miyukini
- La structure documentaire standard
- Les invariants fondamentaux
- Le vocabulaire canonique

Voir les documents de reference et les Guidelines d'implementation pour les standards a respecter.

---

## 13. Recapitulatif des Invariants Cles

Pour reference rapide, voici les invariants fondamentaux :

| Code | Invariant |
|------|-----------|
| **INV-LS-1** | Arbitrage sans execution |
| **INV-LS-2** | Etat systeme abstrait |
| **INV-LS-3** | Lecture seule du systeme |
| **INV-LS-4** | Decisions deterministes |
| **INV-LS-5** | Regles explicites |
| **INV-LS-6** | Tracabilite complete |
| **INV-LS-7** | Separation Kernel |
| **INV-LS-8** | Validation StrongFather |
| **INV-LS-9** | Degradation controlee |
| **INV-LS-10** | Resilience locale |

---

## 14. Documents Associes

- [LogisticsSteward - Documentation Fondatrice](../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md)
- [LogisticsSteward - Index de Navigation](../_index.md)
- [LogisticsSteward - Vocabulary & Glossary](./LogisticsSteward%20-%20Vocabulary%20&%20Glossary.md)
- [LogisticsSteward - Examples & Use Cases](./LogisticsSteward%20-%20Examples%20&%20Use%20Cases.md)
- [LogisticsSteward - Violations & Anti-Patterns](../contracts/governance/LogisticsSteward%20-%20Violations%20&%20Anti-Patterns.md)
- [LogisticsSteward - Resource Arbitration Contract](../contracts/resources/LogisticsSteward%20-%20Resource%20Arbitration%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** REFERENCE â€” Aide a la comprehension  
**Dependencies :**
- [Documentation Fondatrice v1.0](../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md)
- [Index de Navigation](../_index.md)

