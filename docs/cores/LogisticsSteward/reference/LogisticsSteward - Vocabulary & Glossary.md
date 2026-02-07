# LogisticsSteward — Vocabulary & Glossary

## Contexte

Ce document constitue le **dictionnaire officiel** de la terminologie specifique a LogisticsSteward. Il regroupe les definitions des concepts, processus, types, invariants et regles propres a la gouvernance de l'allocation, de la priorisation et de la limitation des ressources.

**Ce glossaire est la source de verite terminologique pour LogisticsSteward.**

## Portee / Scope

- **Applicable a :** Documentation LogisticsSteward, implementation, tests
- **Audience :** Developpeurs, architectes, mainteneurs
- **Statut :** Document de reference normatif — GLOSSAIRE LOGISTICSSTEWARD
- **Relation :** Complete le [Glossaire General Miyukini](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## A

### Arbitrage

**Processus central de LogisticsSteward.** L'arbitrage est le mecanisme par lequel LogisticsSteward decide de l'allocation, de la priorite et de la limitation des ressources pour une entite donnee.

**Caracteristiques :**

| Propriete | Description |
|-----------|-------------|
| **Base sur des regles** | Utilise des regles explicites et declaratives |
| **Deterministe** | Memes entrees = meme decision |
| **Proactif** | Agit avant l'execution, jamais pendant |
| **Soumis a validation** | Decision validee par StrongFather |
| **Trace** | Chaque arbitrage est journalise |

**Ce que l'arbitrage N'EST PAS :**

- ❌ Une execution technique
- ❌ Une mesure de ressources
- ❌ Un scheduling de threads
- ❌ Une optimisation d'execution

**Voir aussi :** Quota, Priorite, Etat systeme abstrait

---

### Attribution

**Processus d'assignation** d'un quota ou d'une priorite a une entite. L'attribution suit un processus formel avec validation StrongFather.

**Caracteristiques :**

- Toujours explicite (pas d'attribution implicite)
- Toujours tracee avec origine et justification
- Toujours validee avant application
- Reversible et modifiable

**Hierarchie d'attribution :**

| Niveau | Description |
|--------|-------------|
| **Global** | Quotas par defaut pour tout l'ecosysteme |
| **Equipe** | Quotas specifiques a une equipe d'operateurs |
| **Entite** | Quotas specifiques a une entite |
| **Exception** | Quotas d'exception valides par StrongFather |

**Voir aussi :** Quota, Priorite, Validation StrongFather

---

## C

### Contention

**Situation de conflit d'acces** ou plusieurs entites demandent simultanement acces a une ressource dont la capacite est insuffisante pour satisfaire toutes les demandes.

**Caracteristiques :**

- Declencheur de la preemption
- Resolue par l'ordre des priorites
- A egalite de priorite : ordre FIFO
- Tracee dans les journaux d'arbitrage

**Voir aussi :** Preemption, Priorite

---

## D

### Degradation

**Reduction controlee et explicite** des capacites du systeme en reponse a une charge elevee, des ressources limitees, ou un etat systeme defavorable.

**Caracteristiques fondamentales :**

| Propriete | Description |
|-----------|-------------|
| **Controlee** | Decidee selon des regles explicites, jamais chaotique |
| **Progressive** | Par paliers (D0 → D1 → D2 → D3 → D4) |
| **Reversible** | Retour a la normale possible et explicite |
| **Explicite** | Annoncee et justifiee, jamais silencieuse |
| **Priorisee** | Services vitaux preserves en dernier |
| **Deterministe** | Memes conditions = meme niveau de degradation |

**Ce que la degradation N'EST PAS :**

- ❌ Un accident ou une panne
- ❌ Une defaillance technique
- ❌ Un etat subi sans controle

**Voir aussi :** Niveau de degradation, Recuperation, Transition

---

### Desescalade

**Retour a la normale** apres une escalade de priorite. La desescalade peut etre automatique (expiration) ou demandee.

**Caracteristiques :**

- Ne necessite pas de validation StrongFather
- Automatique a l'expiration de l'escalade
- Tracee comme toute modification de priorite

**Voir aussi :** Escalade, Priorite

---

## E

### Entite gouvernee

**Sujet de l'arbitrage.** Une entite gouvernee est tout element du systeme soumis aux regles d'allocation de LogisticsSteward.

**Types d'entites gouvernees :**

| Type | Description |
|------|-------------|
| **Operateurs** | Applications metier de l'ecosysteme |
| **Equipes d'Operateurs** | Groupes logiques d'Operateurs |
| **Outils et Toolkits** | Capacites reutilisables |
| **Services exposes** | Fonctionnalites accessibles aux utilisateurs |
| **MiyukiniAdmin** | Avec regles specifiques (priorite maximale possible) |

**Voir aussi :** Operateur, MiyukiniAdmin

---

### Escalade

**Elevation temporaire de la priorite** d'une entite selon des regles explicites et validees.

**Caracteristiques :**

| Propriete | Description |
|-----------|-------------|
| **Temporaire** | Toujours bornee dans le temps |
| **Validee** | Escalade vers P0-P2 validee par StrongFather |
| **Tracee** | Avec justification et duree |
| **Reversible** | Desescalade automatique a expiration |

**Invariants associes :**

- **INV-ESC-1** : Une escalade est toujours temporaire
- **INV-ESC-2** : Escalade vers P0-P2 validee par StrongFather
- **INV-ESC-3** : Desescalade automatique a expiration

**Voir aussi :** Priorite, Desescalade, Validation StrongFather

---

### Etat systeme abstrait

**Representation normalisee** de l'etat des ressources, fournie par le Kernel a LogisticsSteward.

**Caracteristiques :**

| Propriete | Description |
|-----------|-------------|
| **Certifie** | Par le Kernel (source de verite) |
| **Normalise** | Independant de l'OS et du hardware |
| **Lecture seule** | Aucune modification possible par LogisticsSteward |
| **Representatif** | Verite operationnelle du moment |

**Informations typiques :**

- Niveau de charge global (faible / normal / eleve / critique)
- Disponibilite relative des ressources
- Seuils de securite atteints ou proches
- Profil materiel declare
- Etat de degradation eventuel

**Ce que l'etat systeme abstrait N'EST PAS :**

- ❌ Des metriques techniques brutes (CPU%, RAM%)
- ❌ Un dump systeme
- ❌ Un etat modifiable par LogisticsSteward

**Voir aussi :** Kernel, INV-LS-2, INV-LS-3

---

## H

### Hysteresis

**Mecanisme de stabilite** ou les seuils de recuperation sont plus exigeants que les seuils de declenchement pour eviter les oscillations.

**Exemple :**

| Indicateur | Seuil degradation | Seuil recuperation | Delta |
|------------|-------------------|--------------------| ------|
| Disponibilite ressources D1 | < 70% | > 75% | 5% |
| Disponibilite ressources D2 | < 50% | > 60% | 10% |

**Utilite :** Eviter le "flip-flop" entre niveaux de degradation.

**Voir aussi :** Degradation, Recuperation, Transition

---

## I

### INV-LS-1 a INV-LS-10 (Invariants LogisticsSteward)

**Invariants fondamentaux** de LogisticsSteward extraits de la Documentation Fondatrice.

| Invariant | Enonce |
|-----------|--------|
| **INV-LS-1** | Arbitrage sans execution — LogisticsSteward n'a aucun pouvoir d'execution technique |
| **INV-LS-2** | Etat systeme abstrait — Opere uniquement sur un etat certifie fourni par le Kernel |
| **INV-LS-3** | Lecture seule du systeme — Jamais de modification directe de l'etat systeme |
| **INV-LS-4** | Decisions deterministes — Memes entrees = meme decision d'arbitrage |
| **INV-LS-5** | Regles explicites — Toute regle est declaree, jamais implicite |
| **INV-LS-6** | Tracabilite complete — Toute decision est journalisee et auditable |
| **INV-LS-7** | Separation Kernel — Aucun chevauchement avec les responsabilites du Kernel |
| **INV-LS-8** | Validation StrongFather — Decisions soumises a validation/invalidation par StrongFather |
| **INV-LS-9** | Degradation controlee — La degradation est un choix explicite, jamais chaotique |
| **INV-LS-10** | Resilience locale — Fonctionne meme en environnement degrade ou isole |

**Voir aussi :** Invariants de priorite, Invariants de degradation

---

### INV-PRIO-1 a INV-PRIO-5 (Invariants de priorite)

**Invariants du contrat de gestion des priorites.**

| Invariant | Enonce |
|-----------|--------|
| **INV-PRIO-1** | Priorite explicite — Toute entite possede une priorite explicitement declaree |
| **INV-PRIO-2** | Ordre total — Les niveaux de priorite forment un ordre total |
| **INV-PRIO-3** | Stabilite intra-arbitrage — La priorite ne change pas pendant un cycle d'arbitrage |
| **INV-PRIO-4** | Determinisme — A entrees identiques, l'ordre de service est identique |
| **INV-PRIO-5** | Tracabilite — Toute attribution ou modification de priorite est tracee |

**Voir aussi :** Priorite, INV-LS-1 a INV-LS-10

---

### INV-DEG-1 a INV-DEG-6 (Invariants de degradation)

**Invariants du contrat de strategie de degradation.**

| Invariant | Enonce |
|-----------|--------|
| **INV-DEG-1** | Degradation explicite — Tout niveau de degradation est explicitement declare et visible |
| **INV-DEG-2** | Degradation tracee — Toute transition est tracee avec conditions, horodatage et source |
| **INV-DEG-3** | Degradation progressive — La degradation suit les niveaux definis |
| **INV-DEG-4** | Degradation reversible — Toute degradation est reversible selon les conditions de recuperation |
| **INV-DEG-5** | Services vitaux preserves — Les services vitaux sont preserves jusqu'au dernier niveau (D4) |
| **INV-DEG-6** | Determinisme — A conditions identiques, le niveau de degradation est identique |

**Voir aussi :** Degradation, Niveau de degradation, Recuperation

---

### INTERD-LS-1 a INTERD-LS-10 (Interdictions LogisticsSteward)

**Interdictions fondamentales** de LogisticsSteward extraites de la Documentation Fondatrice.

| Code | Interdiction | Raison |
|------|--------------|--------|
| **INTERD-LS-1** | Mesure directe des ressources | Maintenir la separation avec le Kernel |
| **INTERD-LS-2** | Execution technique | Maintenir le role d'arbitre pur |
| **INTERD-LS-3** | Allocation memoire/CPU | Responsabilite exclusive du Kernel |
| **INTERD-LS-4** | Planification de threads | Responsabilite exclusive du Kernel |
| **INTERD-LS-5** | Pilotage de scheduler | Responsabilite exclusive du Kernel |
| **INTERD-LS-6** | Optimisation d'execution | Hors perimetre de gouvernance |
| **INTERD-LS-7** | Stockage d'etat operationnel | Maintenir la purete fonctionnelle |
| **INTERD-LS-8** | Decision auto-appliquee | Maintenir la validation StrongFather |
| **INTERD-LS-9** | Bypass du Kernel | Maintenir l'architecture en strates |
| **INTERD-LS-10** | Regles implicites | Maintenir l'auditabilite |

**Voir aussi :** INV-LS-1 a INV-LS-10

---

## L

### LogisticsSteward

**Core de gouvernance des ressources** de l'ecosysteme Miyukini. LogisticsSteward repond a la question fondamentale : "Qui a le droit d'utiliser quoi, quand, et a quel niveau de priorite ?"

**Ce que LogisticsSteward EST :**

| Propriete | Description |
|-----------|-------------|
| **Arbitre** | Decide de l'allocation et de la priorite des ressources |
| **Gouverneur** | Etablit des quotas, plafonds et restrictions |
| **Protecteur** | Empeche la saturation et la monopolisation |
| **Declaratif** | Fonctionne sur des regles explicites et auditables |
| **Deterministe** | Memes entrees = memes decisions d'arbitrage |
| **Proactif** | Agit avant l'execution, jamais pendant |

**Ce que LogisticsSteward N'EST PAS :**

| Propriete | Raison |
|-----------|--------|
| **Un scheduler** | Pas de planification de threads ou de taches |
| **Un gestionnaire memoire** | Pas d'allocation bas niveau |
| **Un orchestrateur d'execution** | Pas de pilotage d'execution |
| **Un outil d'optimisation** | Gouverne, n'optimise pas |
| **Un mesureur de ressources** | Ne lit jamais CPU, RAM, IO directement |
| **Un controleur technique** | Aucune action bas niveau |

**Phrase fondatrice :**

> **LogisticsSteward est le core qui empeche le chaos silencieux en garantissant que chaque entite a droit a ce qui lui est du — ni plus, ni moins — selon des regles explicites, deterministes et auditables, sans jamais executer ni controler techniquement.**

**Voir aussi :** Arbitrage, Quota, Priorite

---

## M

### MiyukiniAdmin

**Operateur souverain d'administration** avec des regles specifiques dans LogisticsSteward.

**Regles specifiques :**

| Regle | Description |
|-------|-------------|
| **Priorite par defaut** | P2 (HIGH) |
| **Priorite maximale possible** | Peut demander jusqu'a P0 (CRITICAL) |
| **Gouvernance preservee** | Reste soumis aux regles globales |
| **Exception explicite** | Tout bypass necessite un protocole d'exception |
| **Tracabilite totale** | Chaque exception est journalisee |

**En degradation :**

| Niveau | Comportement MiyukiniAdmin |
|--------|---------------------------|
| D0-D2 | Acces complet, priorite P2 |
| D3 | Acces maintenu, quotas reduits |
| D4 | Acces de survie, operations critiques uniquement |

**Voir aussi :** Priorite, Protocole d'exception

---

### Mode Recovery

**Etat distinct de la degradation** ou le systeme est en cours de restauration apres incident.

**Differences avec la degradation :**

| Aspect | Degradation | Mode Recovery |
|--------|-------------|---------------|
| **Objectif** | Preservation | Restauration |
| **Quotas** | Reduits | Relaches |
| **Priorites** | Suspendues par niveau | P1 automatique pour recovery |
| **Duree** | Variable | Bornee |

**Voir aussi :** Degradation, Recuperation

---

## N

### Niveau de degradation

**Etat du systeme** caracterise par un ensemble de restrictions et de capacites reduites. LogisticsSteward definit 5 niveaux :

| Niveau | Code | Description | Etat systeme |
|--------|------|-------------|--------------|
| **D0** | `NORMAL` | Aucune degradation | Toutes capacites disponibles |
| **D1** | `PRUDENT` | Charge elevee detectee | Limitation des operations non critiques |
| **D2** | `RESTREINT` | Ressources limitees | Desactivation de fonctionnalites secondaires |
| **D3** | `CRITIQUE` | Risque de saturation | Services minimaux uniquement |
| **D4** | `SURVIE` | Etat d'urgence | Preservation du coeur systeme uniquement |

**Impacts par niveau :**

| Niveau | Quotas | Priorites actives |
|--------|--------|-------------------|
| D0 | 100% | P0-P6 |
| D1 | 80-90% non-critiques | P0-P5 |
| D2 | 50-70% non-critiques | P0-P4 |
| D3 | 20-30% non-critiques | P0-P3 |
| D4 | Urgence | P0-P2 |

**Voir aussi :** Degradation, Transition, Recuperation

---

### Niveau de priorite

**Classification des entites** selon leur importance relative dans l'acces aux ressources. LogisticsSteward definit 7 niveaux :

| Niveau | Code | Description | Usage type |
|--------|------|-------------|------------|
| **P0** | `CRITICAL` | Priorite critique systeme | Fonctions vitales du systeme |
| **P1** | `EMERGENCY` | Priorite d'urgence | Operations de recovery, alertes critiques |
| **P2** | `HIGH` | Priorite haute | Operations administratives, maintenance |
| **P3** | `ELEVATED` | Priorite elevee | Operations metier prioritaires |
| **P4** | `NORMAL` | Priorite normale | Operations metier standard |
| **P5** | `LOW` | Priorite basse | Operations de fond, batch |
| **P6** | `BACKGROUND` | Priorite d'arriere-plan | Taches non urgentes, preemptables |

**Priorite par defaut :**

| Entite | Priorite par defaut |
|--------|---------------------|
| Operateur standard | P4 (NORMAL) |
| Operateur batch | P5 (LOW) |
| Service systeme | P3 (ELEVATED) |
| MiyukiniAdmin | P2 (HIGH) |
| Mode recovery | P1 (EMERGENCY) |

**Voir aussi :** Priorite, Preemption

---

## O

### Operateur

**Application metier** de l'ecosysteme Miyukini, soumise aux regles d'arbitrage de LogisticsSteward.

**Caracteristiques :**

- Possede des quotas attribues (volume, concurrence, capacite)
- Possede une priorite (P4 par defaut)
- Peut appartenir a une equipe d'operateurs
- Peut heriter des quotas de son equipe

**Voir aussi :** Entite gouvernee, Equipe d'Operateurs

---

## P

### Plafond

**Limite absolue d'utilisation** qui ne peut etre depassee, independamment de la priorite ou du contexte.

**Caracteristiques :**

| Propriete | Description |
|-----------|-------------|
| **Absolu** | Ne peut jamais etre depasse |
| **Independant du contexte** | Pas d'exception possible |
| **Distinct du quota** | Le quota peut etre consomme, le plafond est une limite dure |

**Voir aussi :** Quota

---

### Preemption

**Capacite a interrompre ou retarder l'acces** aux ressources d'une entite au profit d'une entite de priorite superieure.

**Caracteristiques :**

| Propriete | Description |
|-----------|-------------|
| **Conditionnelle** | S'applique uniquement en cas de contention |
| **Ordonnee** | Suit strictement l'ordre des priorites |
| **Tracee** | Toute preemption est journalisee |
| **Reversible** | Ne modifie pas les droits futurs de l'entite preemptee |

**Ordre de preemption :** P0 > P1 > P2 > P3 > P4 > P5 > P6

**Limites de preemption :**

| Niveau de l'entite preemptee | Peut etre preemptee par |
|------------------------------|-------------------------|
| P6 (BACKGROUND) | P0 a P5 |
| P5 (LOW) | P0 a P4 |
| P4 (NORMAL) | P0 a P3 |
| P3 (ELEVATED) | P0 a P2 |
| P2 (HIGH) | P0 a P1 |
| P1 (EMERGENCY) | P0 uniquement |
| P0 (CRITICAL) | **Non preemptable** |

**Invariant associe :** INV-PREEMP-2 — Une entite de priorite P0 ne peut jamais etre preemptee.

**Voir aussi :** Priorite, Contention, Famine

---

### Priorite

**Niveau relatif** d'une entite par rapport aux autres dans l'acces aux ressources. Les priorites determinent l'ordre de service en cas de contention.

**Caracteristiques :**

| Propriete | Description |
|-----------|-------------|
| **Relative** | N'a de sens que par rapport aux autres priorites |
| **Explicite** | Declaree, jamais implicite |
| **Deterministe** | Produit toujours le meme ordre d'arbitrage |
| **Immutable pendant arbitrage** | Ne change pas pendant un cycle d'arbitrage |
| **Auditable** | Toute attribution est tracable |

**Voir aussi :** Niveau de priorite, Preemption, Escalade

---

### Protection anti-famine

**Mecanisme de protection** des entites a priorite basse contre l'attente indefinie.

**Regles :**

| Regle | Description |
|-------|-------------|
| **R-FAMINE-1** | Elevation anti-famine : entite en attente depuis seuil configurable elevee d'un niveau (plafond P3) |
| **R-FAMINE-2** | Quota de preemption : entite ne peut etre preemptee qu'un nombre limite de fois par periode |
| **R-FAMINE-3** | Reservation minimale : chaque niveau de priorite dispose d'une reservation minimale garantie |

**Voir aussi :** Preemption, Priorite

---

### Protocole d'exception

**Procedure formelle** permettant a MiyukiniAdmin de demander un bypass des regles normales de gouvernance.

**Caracteristiques :**

- Requiert validation StrongFather
- Duree limitee
- Tracabilite complete
- Ne contourne jamais les invariants fondamentaux

**Voir aussi :** MiyukiniAdmin, Validation StrongFather

---

## Q

### Quota

**Limite declaree** sur l'usage d'une ressource conceptuelle par une entite. Le quota est l'unite fondamentale de gouvernance des ressources geree par LogisticsSteward.

**Caracteristiques fondamentales :**

| Propriete | Description |
|-----------|-------------|
| **Declaratif** | Declaration de limite, pas une mesure technique |
| **Explicite** | Formellement defini et documente |
| **Deterministe** | A contexte identique, le quota calcule est toujours le meme |
| **Auditable** | Toute attribution est tracable avec origine et justification |
| **Revisable** | Peut etre modifie selon des regles definies |

**Ce qu'un quota N'EST PAS :**

- ❌ Une mesure technique de ressource (CPU, RAM, IO)
- ❌ Un compteur d'utilisation en temps reel
- ❌ Un mecanisme de throttling technique
- ❌ Une allocation memoire ou systeme
- ❌ Un scheduler ou ordonnanceur

**Voir aussi :** Types de quotas, Attribution

---

### Quota conditionnel

**Type de quota** dont la valeur varie selon le contexte ou les conditions du systeme.

**Caracteristiques :**

| Aspect | Specification |
|--------|---------------|
| **Unite typique** | Variable selon le quota sous-jacent |
| **Periode** | Variable selon le quota sous-jacent |
| **Renouvellement** | A chaque evaluation des conditions |
| **Usage typique** | Adaptation dynamique aux conditions systeme |

**Exemples :**

- 1000 requetes/h en conditions normales, 200/h en degradation
- 5 sessions si charge faible, 2 si charge elevee

**Voir aussi :** Quota, Degradation

---

### Quota de capacite

**Type de quota** limitant la quantite totale d'une ressource stockable ou reservable.

**Caracteristiques :**

| Aspect | Specification |
|--------|---------------|
| **Unite typique** | Octets, enregistrements, entites |
| **Periode** | Non applicable (cumul) |
| **Renouvellement** | A la liberation ou suppression |
| **Usage typique** | Limitation du stockage, du nombre d'objets |

**Exemples :**

- 10 Go de stockage par operateur
- 1000 documents par equipe
- 50 integrations actives par service

**Voir aussi :** Quota

---

### Quota de concurrence

**Type de quota** limitant le nombre d'operations simultanees ou de ressources actives.

**Caracteristiques :**

| Aspect | Specification |
|--------|---------------|
| **Unite typique** | Sessions, connexions, processus actifs |
| **Periode** | Non applicable (instantane) |
| **Renouvellement** | A la liberation de la ressource |
| **Usage typique** | Limitation des sessions actives, des telechargements paralleles |

**Exemples :**

- 5 sessions actives simultanees par utilisateur
- 3 telechargements paralleles par operateur
- 10 connexions WebSocket simultanees par service

**Voir aussi :** Quota

---

### Quota de priorite

**Type de quota** definissant le niveau de service ou de priorite d'acces aux ressources.

**Caracteristiques :**

| Aspect | Specification |
|--------|---------------|
| **Unite typique** | Niveau (1-10), classe (gold, silver, bronze) |
| **Periode** | Non applicable (permanent jusqu'a modification) |
| **Renouvellement** | Sur decision explicite |
| **Usage typique** | Differenciation de service, QoS conceptuel |

**Exemples :**

- Priorite niveau 8/10 pour MiyukiniAdmin
- Priorite classe "gold" pour les operateurs premium

**Voir aussi :** Quota, Priorite

---

### Quota de volume

**Type de quota** limitant le nombre total d'operations ou d'unites consommables sur une periode.

**Caracteristiques :**

| Aspect | Specification |
|--------|---------------|
| **Unite typique** | Requetes, operations, transactions |
| **Periode typique** | Minute, heure, jour, mois |
| **Renouvellement** | A la fin de la periode |
| **Usage typique** | Limitation des appels API, des creations d'entites |

**Exemples :**

- 1000 requetes API par heure par operateur
- 50 creations d'utilisateurs par jour par equipe
- 10 exports de donnees par mois par service

**Voir aussi :** Quota

---

## R

### Recuperation

**Processus de retour** a un niveau de degradation inferieur (vers la normale). La recuperation suit des regles specifiques pour eviter les oscillations.

**Principes :**

| Principe | Description |
|----------|-------------|
| **Prudence** | Recuperation plus lente que degradation |
| **Stabilite** | Verification de stabilite avant chaque retour |
| **Progressivite** | Un niveau a la fois |
| **Hysteresis** | Seuils de recuperation differents des seuils de declenchement |
| **Confirmation** | Retour confirme avant restauration des capacites |

**Delais de recuperation :**

| Transition | Delai minimum |
|------------|---------------|
| D4 → D3 | ~5 min |
| D3 → D2 | ~10 min |
| D2 → D1 | ~15 min |
| D1 → D0 | ~20 min |

**Voir aussi :** Degradation, Hysteresis, Transition

---

### Regle explicite

**Principe fondamental** de LogisticsSteward : toute regle est declaree, jamais implicite.

**Invariant associe :** INV-LS-5 — Toute regle est declaree, jamais implicite.

**Implications :**

- Pas de quota par defaut non declare
- Pas de priorite implicite
- Pas de decision basee sur des conventions non documentees
- Auditabilite complete

**Voir aussi :** INV-LS-5, Tracabilite

---

## S

### Separation Kernel

**Principe fondamental** : LogisticsSteward gouverne l'usage des ressources, le Kernel les controle.

**La separation est absolue :**

| LogisticsSteward | Kernel |
|------------------|--------|
| Decide *qui a droit a quoi* | Execute *comment* |
| Arbitre, gouverne, protege | Mesure, alloue, execute |
| Regles declaratives | Actions techniques |
| Aucun pouvoir d'execution | Aucune decision de gouvernance |

**Invariant associe :** INV-LS-7 — Aucun chevauchement avec les responsabilites du Kernel.

**Voir aussi :** INV-LS-7, Kernel

---

### Service vital

**Service protege** meme en cas de degradation severe.

**Services toujours proteges :**

| Service | Niveau de protection |
|---------|----------------------|
| **Journalisation** | Jusqu'a D3 |
| **Authentification** | Jusqu'a D3 |
| **Persistence critique** | Jusqu'a D4 |
| **Monitoring vital** | Jusqu'a D4 |

**Invariant associe :** INV-DEG-5 — Les services vitaux sont preserves jusqu'au dernier niveau (D4).

**Voir aussi :** Degradation, Niveau de degradation

---

### Strate 3

**Position de LogisticsSteward** dans la pyramide d'architecture Miyukini.

**Positionnement :**

| Strate | Contenu |
|--------|---------|
| Strate 4 | Cores Systeme (StrongFather, KindMother, WorrySentinel) |
| **Strate 3** | **Gouvernance Ressources (LogisticsSteward)** |
| Strate 2 | Capacites (MasterButler) |
| Strate 1 | Kernel (Infrastructure technique) |

**Implications :**

- Recoit l'etat systeme du Kernel (lecture seule)
- Soumet ses decisions a StrongFather (validation)
- Transmet via BondingBrother (transport)
- Repond aux alertes de WorrySentinel (adaptation)

**Voir aussi :** Pyramide d'architecture

---

## T

### Tracabilite

**Principe fondamental** : toute decision est journalisee et auditable.

**Invariant associe :** INV-LS-6 — Toute decision est journalisee et auditable.

**Ce qui est trace :**

- Toute attribution de quota
- Toute modification de priorite
- Toute preemption
- Toute transition de degradation
- Toute escalade/desescalade
- Tout protocole d'exception

**Voir aussi :** INV-LS-6, Auditabilite

---

### Transition

**Passage d'un niveau de degradation a un autre.** Les transitions sont regies par des conditions explicites et validees.

**Types de transition :**

| Type | Direction | Description |
|------|-----------|-------------|
| **Ascendante** | Vers D4 | Degradation (D0→D1→D2→D3→D4) |
| **Descendante** | Vers D0 | Recuperation (D4→D3→D2→D1→D0) |

**Regles de transition :**

- Declenchement progressif (un niveau maximum, sauf urgence)
- Declenchement trace
- Declenchement annonce
- Delai de stabilisation entre transitions

**Voir aussi :** Degradation, Recuperation, Hysteresis

---

## V

### Validation StrongFather

**Soumission des decisions** de LogisticsSteward a l'autorite de StrongFather pour validation ou invalidation.

**Invariant associe :** INV-LS-8 — Decisions soumises a validation/invalidation par StrongFather.

**Ce qui requiert validation StrongFather :**

- Escalades vers P0, P1, P2
- Protocoles d'exception MiyukiniAdmin
- Modifications de regles de priorite
- Attributions de quota d'exception
- Transitions de degradation

**Ce qui ne requiert PAS validation :**

- Attributions de priorite P3 a P6 (par configuration)
- Degradations de priorite
- Desescalades

**Voir aussi :** StrongFather, Escalade, Protocole d'exception

---

## Tableau de correspondance terminologique

| Terme incorrect | Terme correct |
|-----------------|---------------|
| Core de scheduling | **LogisticsSteward** (arbitre, pas scheduler) |
| Gestionnaire memoire | ❌ Responsabilite du Kernel |
| Optimiseur de ressources | ❌ LogisticsSteward gouverne, n'optimise pas |
| Quota implicite | ❌ Interdit (INV-LS-5) |
| Priorite auto-attribuee | ❌ Interdit (INTERD-ATTR-1) |
| Degradation silencieuse | ❌ Interdit (INV-DEG-1) |
| Mesure de CPU/RAM | ❌ Interdit (INTERD-LS-1) |
| Decision auto-appliquee | ❌ Interdit (INTERD-LS-8) |

---

## Phrases Fondatrices

### Separation Kernel / LogisticsSteward

> **"LogisticsSteward gouverne l'usage des ressources. Le Kernel les controle."**

### Nature de LogisticsSteward

> **"LogisticsSteward est le core qui empeche le chaos silencieux en garantissant que chaque entite a droit a ce qui lui est du — ni plus, ni moins — selon des regles explicites, deterministes et auditables, sans jamais executer ni controler techniquement."**

### Question fondamentale

> **"Qui a le droit d'utiliser quoi, quand, et a quel niveau de priorite ?"**

### Degradation

> **"La degradation est un choix explicite, jamais un accident."**

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document de reference normatif — GLOSSAIRE LOGISTICSSTEWARD

**References croisees :**

- [LogisticsSteward - Documentation Fondatrice](../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) : Definition du core
- [LogisticsSteward - Quota Definition Contract](../contracts/resources/LogisticsSteward%20-%20Quota%20Definition%20Contract.md) : Types de quotas
- [LogisticsSteward - Priority Management Contract](../contracts/resources/LogisticsSteward%20-%20Priority%20Management%20Contract.md) : Niveaux de priorite
- [LogisticsSteward - Degradation Strategy Contract](../contracts/degradation/LogisticsSteward%20-%20Degradation%20Strategy%20Contract.md) : Niveaux de degradation
- [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Glossaire general
