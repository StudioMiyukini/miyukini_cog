# LogisticsSteward â€” Documentation Fondatrice

## 1. Contexte

Ce document definit l'identite, le role et les principes fondamentaux de **LogisticsSteward** : le core responsable de la gouvernance de l'allocation, de la priorisation et de la limitation des ressources au sein d'un environnement Miyukini.

LogisticsSteward arbitre l'usage des ressources selon des regles explicites, des politiques declarees et un etat systeme certifie. Il ne mesure jamais les ressources, ne les controle jamais techniquement â€” ces responsabilites appartiennent exclusivement au Kernel.

**Principe fondamental :**

> **"LogisticsSteward gouverne l'usage des ressources. Le Kernel les controle."**

La separation est absolue : LogisticsSteward decide *qui a droit a quoi*, le Kernel execute *comment*.

## 2. Portee / Scope

Ce document definit :
- L'identite et le role de LogisticsSteward
- Les invariants fondamentaux
- Les interdictions absolues
- Le perimetre fonctionnel exact
- Les relations avec les autres cores
- Les principes de degradation controlee

Ce document **ne couvre pas** :
- Les details d'implementation technique (voir Implementation Guidelines)
- Les protocoles de quota detailles (voir Quota Definition Contract)
- Les strategies de degradation detaillees (voir Degradation Strategy Contract)
- Les contrats d'integration specifiques (voir dossier contracts/integration/)

---

## 3. Definition Canonique

### 3.1 Enonce Canonique

> **"Qui a le droit d'utiliser quoi, quand, et a quel niveau de priorite ?"**

LogisticsSteward repond a cette question fondamentale. Il constitue :
- Le ministere du budget et des ressources de l'ecosysteme Miyukini
- L'arbitre de l'usage, pas le controleur technique
- Le garant de la cohabitation stable entre entites

### 3.2 Ce que LogisticsSteward EST

| Propriete | Description |
|-----------|-------------|
| **Arbitre** | Decide de l'allocation et de la priorite des ressources |
| **Gouverneur** | Etablit des quotas, plafonds et restrictions |
| **Protecteur** | Empeche la saturation et la monopolisation |
| **Declaratif** | Fonctionne sur des regles explicites et auditables |
| **Deterministe** | Memes entrees = memes decisions d'arbitrage |
| **Proactif** | Agit avant l'execution, jamais pendant |

### 3.3 Ce que LogisticsSteward N'EST PAS

| Propriete | Raison |
|-----------|--------|
| **Un scheduler** | Pas de planification de threads ou de taches |
| **Un gestionnaire memoire** | Pas d'allocation bas niveau |
| **Un orchestrateur d'execution** | Pas de pilotage d'execution |
| **Un outil d'optimisation** | Gouverne, n'optimise pas |
| **Un mesureur de ressources** | Ne lit jamais CPU, RAM, IO directement |
| **Un controleur technique** | Aucune action bas niveau |

---

## 4. Invariants Fondamentaux

### 4.1 Catalogue des Invariants

| Code | Invariant | Description |
|------|-----------|-------------|
| **INV-LS-1** | Arbitrage sans execution | LogisticsSteward n'a aucun pouvoir d'execution technique |
| **INV-LS-2** | Etat systeme abstrait | Opere uniquement sur un etat certifie fourni par le Kernel |
| **INV-LS-3** | Lecture seule du systeme | Jamais de modification directe de l'etat systeme |
| **INV-LS-4** | Decisions deterministes | Memes entrees = meme decision d'arbitrage |
| **INV-LS-5** | Regles explicites | Toute regle est declaree, jamais implicite |
| **INV-LS-6** | Tracabilite complete | Toute decision est journalisee et auditable |
| **INV-LS-7** | Separation Kernel | Aucun chevauchement avec les responsabilites du Kernel |
| **INV-LS-8** | Validation StrongFather | Decisions soumises a validation/invalidation par StrongFather |
| **INV-LS-9** | Degradation controlee | La degradation est un choix explicite, jamais chaotique |
| **INV-LS-10** | Resilience locale | Fonctionne meme en environnement degrade ou isole |

### 4.2 Regles Absolues (Non Negociables)

**LogisticsSteward ne mesure jamais directement les ressources systeme**

**LogisticsSteward n'execute aucune action technique bas niveau**

**LogisticsSteward ne stocke aucun etat operationnel**

**LogisticsSteward ne contourne jamais le Kernel pour acceder au hardware**

**LogisticsSteward ne prend aucune decision auto-appliquee**

Toute decision doit etre validee par StrongFather et executee par le Kernel.

---

## 5. Interdictions

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

---

## 6. Perimetre Fonctionnel

### 6.1 Acces a l'Etat Systeme

LogisticsSteward opere a partir d'un **etat systeme abstrait**, fourni par le Kernel.

Cet etat est :
- **Certifie** par le Kernel (source de verite)
- **Normalise** (independant de l'OS et du hardware)
- **En lecture seule** (aucune modification possible)
- **Representatif** de la verite operationnelle du moment

**Exemples d'informations accessibles :**
- Niveau de charge global (faible / normal / eleve / critique)
- Disponibilite relative des ressources
- Seuils de securite atteints ou proches
- Profil materiel declare
- Etat de degradation eventuel

### 6.2 Entites Gouvernees

LogisticsSteward peut arbitrer pour :

| Entite | Description |
|--------|-------------|
| **Operateurs** | Applications metier de l'ecosysteme |
| **Equipes d'Operateurs** | Groupes logiques d'Operateurs |
| **Outils et Toolkits** | Capacites reutilisables |
| **Services exposes** | Fonctionnalites accessibles aux utilisateurs |
| **MiyukiniAdmin** | Avec regles specifiques (voir Section 9) |

### 6.3 Types de Regles Gerees

| Type | Description |
|------|-------------|
| **Quotas** | Limites d'usage de ressources conceptuelles |
| **Priorites** | Niveaux relatifs entre entites |
| **Plafonds** | Maximums absolus d'utilisation |
| **Restrictions temporaires** | Limitations contextuelles |
| **Politiques de degradation** | Regles de reduction controlee |

Ces regles sont :
- **Explicites** : declarees, pas deduites
- **Declaratives** : ce qu'on veut, pas comment
- **Auditables** : tracables et verifiables
- **Deterministes** : comportement previsible

### 6.4 Processus d'Arbitrage

```
[Demande de ressource]
       â”‚
       â–¼
[LogisticsSteward]
  â”œâ”€â”€ Lecture etat systeme (Kernel)
  â”œâ”€â”€ Evaluation des regles
  â”œâ”€â”€ Decision d'arbitrage
       â”‚
       â–¼
[StrongFather]
  â””â”€â”€ Validation / Invalidation
       â”‚
       â–¼
[Kernel]
  â””â”€â”€ Execution de la decision
```

LogisticsSteward agit **avant l'execution, jamais pendant**.

---

## 7. Degradation et Protection du Systeme

### 7.1 Role de Protection Proactive

LogisticsSteward est un **core de protection proactive**. Il permet :

- La reduction volontaire de capacites
- La mise sous contrainte d'operateurs gourmands
- La desactivation progressive de fonctionnalites non critiques
- La preservation des services vitaux

### 7.2 Principes de Degradation

| Principe | Description |
|----------|-------------|
| **Controlee** | Jamais chaotique, toujours decidee |
| **Progressive** | Par paliers, pas brutale |
| **Reversible** | Retour a la normale possible |
| **Explicite** | Annoncee et justifiee |
| **Priorisee** | Services vitaux preserves en dernier |

### 7.3 Niveaux de Degradation

| Niveau | Description | Actions possibles |
|--------|-------------|-------------------|
| **D0 - Normal** | Aucune degradation | Toutes capacites disponibles |
| **D1 - Prudent** | Charge elevee | Limitation des operations non critiques |
| **D2 - Restreint** | Ressources limitees | Desactivation de fonctionnalites secondaires |
| **D3 - Critique** | Risque de saturation | Services minimaux uniquement |
| **D4 - Survie** | Etat d'urgence | Preservation du coeur systeme uniquement |

---

## 8. Relations avec les Autres Cores

### 8.1 Kernel

**Role :** Fournisseur d'etat systeme et executeur des decisions.

**Responsabilites :**
- Fournit l'etat systeme abstrait (lecture seule)
- Execute les arbitrages decides par LogisticsSteward
- Reste seul maitre du bas niveau technique
- Certifie la verite operationnelle

**Nature de la relation :** LogisticsSteward consomme l'etat, le Kernel l'execute.

### 8.2 StrongFather

**Role :** Autorite de validation des decisions d'arbitrage.

**Responsabilites :**
- Valide ou invalide les decisions d'arbitrage
- Tranche en cas de conflit de regles
- Garantit la coherence globale
- Applique les politiques de gouvernance

**Nature de la relation :** LogisticsSteward propose, StrongFather dispose.

### 8.3 MasterButler

**Role :** Exposition des capacites disponibles.

**Responsabilites :**
- Expose les capacites et services disponibles
- Declare ce qui existe (pas ce qui est autorise)
- Fournit le catalogue de fonctionnalites

**Nature de la relation :** MasterButler dit ce qui existe, LogisticsSteward limite l'usage (pas l'existence).

### 8.4 WorrySentinel

**Role :** Surveillance et controle de securite.

**Responsabilites :**
- Peut invalider un etat systeme juge incoherent
- Peut declencher un durcissement des regles
- Supervise les derives ou comportements suspects
- Alerte en cas d'anomalie de gouvernance

**Nature de la relation :** WorrySentinel surveille, LogisticsSteward adapte ses regles.

### 8.5 BondingBrother

**Role :** Transport des decisions d'arbitrage.

**Responsabilites :**
- Transporte les decisions d'arbitrage vers les entites concernees
- Applique les contraintes aux operateurs et services
- Ne les interprete jamais
- Garantit la tracabilite du transport

**Nature de la relation :** LogisticsSteward decide, BondingBrother transmet fidelement.

### 8.6 MiyukiniAdmin

**Role :** Console d'administration avec regles specifiques.

**Responsabilites :**
- Peut obtenir des priorites maximales
- Reste soumis a la gouvernance globale
- Sauf protocole d'exception explicitement valide
- Tracabilite complete des exceptions

**Nature de la relation :** MiyukiniAdmin peut demander des privileges, LogisticsSteward les arbitre.

---

## 9. Cas Particuliers

### 9.1 MiyukiniAdmin

LogisticsSteward applique des **regles specifiques** pour MiyukiniAdmin :

| Regle | Description |
|-------|-------------|
| **Priorite maximale possible** | MiyukiniAdmin peut demander la priorite la plus haute |
| **Gouvernance preservee** | Reste soumis aux regles globales |
| **Exception explicite** | Tout bypass necessite un protocole d'exception |
| **Tracabilite totale** | Chaque exception est journalisee |

MiyukiniAdmin n'est pas au-dessus de LogisticsSteward. Il peut demander des exceptions, pas les imposer.

### 9.2 Mode Recovery

En mode recovery systeme :

- LogisticsSteward maintient des regles minimales
- Les quotas sont relaches pour permettre la restauration
- La tracabilite reste active
- Le retour a la normale est explicite

---

## 10. Position dans la Pyramide

### 10.1 Positionnement

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 9 â€” MiyukiniAdmin (EXCEPTION)     â”‚
â”‚ Operateur Souverain d'administration     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
          â–²
          â”‚ (hors pyramide)
          â”‚
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 4 â€” Cores Systeme                 â”‚
â”‚ StrongFather, KindMother, WorrySentinel  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
          â–²
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 3 â€” Gouvernance Ressources        â”‚
â”‚ LogisticsSteward                         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
          â–²
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 2 â€” Capacites                     â”‚
â”‚ MasterButler                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
          â–²
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 1 â€” Kernel                        â”‚
â”‚ Infrastructure technique                 â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

LogisticsSteward est positionne en **Strate 3**, entre :
- Le Kernel (infrastructure technique) en dessous
- Les Cores Systeme (autorites) au-dessus

### 10.2 Regles de Communication

- Recoit l'etat systeme du Kernel (lecture seule)
- Soumet ses decisions a StrongFather (validation)
- Transmet via BondingBrother (transport)
- Repond aux alertes de WorrySentinel (adaptation)

---

## 11. Conformite aux Lois d'Autonomie Systeme

Ce core respecte les **Lois d'Autonomie Systeme** definies dans [Miyukini Conceptual References - Lois Autonomie Systeme.md](..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dependance externe critique a l'execution

**Conformite :** âœ… **Conforme**

LogisticsSteward respecte integralement LOI-1 :
- Fonctionne avec l'etat systeme local fourni par le Kernel
- Aucune dependance a un service externe pour l'arbitrage
- Les regles sont locales et declaratives
- L'absence de connexion reseau ne bloque jamais l'arbitrage

### LOI-2 : Le systeme accepte l'isolement comme etat normal

**Conformite :** âœ… **Conforme**

LogisticsSteward respecte integralement LOI-2 :
- L'isolement est un etat normal de fonctionnement
- Les decisions d'arbitrage sont prises avec l'etat local disponible
- Pas de blocage en attente de synchronisation
- Le mode degrade est explicitement gere

### LOI-3 : L'etat local est souverain

**Conformite :** âœ… **Conforme**

LogisticsSteward respecte integralement LOI-3 :
- L'etat systeme local (fourni par le Kernel) est la verite
- Les decisions sont valides localement
- Les journaux d'arbitrage constituent une trace d'audit complete
- A la reconnexion : reconciliation, pas de "correction en douce"

### LOI-4 : Pas de temps global requis

**Conformite :** âœ… **Conforme**

LogisticsSteward respecte integralement LOI-4 :
- Pas de dependance a une horloge reseau
- Les decisions sont basees sur l'etat actuel, pas sur des timestamps
- Les quotas sont evalues localement
- Compatible avec des horloges desynchronisees

### LOI-5 : Le cout doit etre proportionnel au hardware

**Conformite :** âœ… **Conforme**

LogisticsSteward respecte integralement LOI-5 :
- Moteur d'arbitrage pur, sans etat persistant massif
- Consommation memoire maitrisee et previsible
- Pas de workers permanents inutiles
- Compatible avec hardware simple (Raspberry Pi, mini PC)

### LOI-6 : L'autonomie n'empeche pas la federation

**Conformite :** âœ… **Conforme**

LogisticsSteward respecte integralement LOI-6 :
- Peut fonctionner seul ou en federation
- Les regles de gouvernance sont locales
- La federation n'impose pas de regles externes
- La synchronisation des quotas est explicite si federee

---

## 12. Vocabulaire Canonique

Le vocabulaire de LogisticsSteward est precis, stable, non ambigu.

### Arbitrage

L'**arbitrage** est le processus par lequel LogisticsSteward decide de l'allocation, de la priorite et de la limitation des ressources pour une entite donnee. L'arbitrage est base sur des regles explicites et un etat systeme certifie.

### Quota

Un **quota** est une limite declaree sur l'usage d'une ressource conceptuelle par une entite. Les quotas sont explicites, auditables et deterministes.

### Priorite

La **priorite** est le niveau relatif d'une entite par rapport aux autres dans l'acces aux ressources. Les priorites determinent l'ordre de service en cas de contention.

### Plafond

Un **plafond** est une limite absolue d'utilisation qui ne peut etre depassee, independamment de la priorite ou du contexte.

### Degradation

La **degradation** est la reduction controlee et explicite des capacites du systeme en reponse a une charge elevee ou des ressources limitees. La degradation est un choix, pas un accident.

### Etat Systeme Abstrait

L'**etat systeme abstrait** est une representation normalisee de l'etat des ressources, fournie par le Kernel. Cet etat est certifie, independant du hardware, et en lecture seule pour LogisticsSteward.

---

## 13. Phrase Fondatrice

**LogisticsSteward est le core qui empeche le chaos silencieux en garantissant que chaque entite a droit a ce qui lui est du â€” ni plus, ni moins â€” selon des regles explicites, deterministes et auditables, sans jamais executer ni controler techniquement.**

Cette phrase resume l'essence de LogisticsSteward : arbitre (pas executeur), gouverneur (pas optimiseur), protecteur (pas controleur technique), declaratif (pas implicite), deterministe (pas aleatoire).

Toute implementation de LogisticsSteward doit respecter cette phrase fondatrice. Toute evolution de LogisticsSteward doit preserver cette essence. Toute specialisation de LogisticsSteward doit rester fidele a cette nature.

---

## 14. Positionnement Conceptuel (Metaphore)

Si Miyukini est un Etat :

| Core | Metaphore |
|------|-----------|
| **Kernel** | Infrastructure physique (routes, reseaux, batiments) |
| **StrongFather** | Autorite decisionnelle (gouvernement, lois) |
| **MasterButler** | Catalogue des capacites (ministere des services) |
| **LogisticsSteward** | Ministere du budget et des ressources |
| **WorrySentinel** | Securite interieure (surveillance, protection) |
| **BondingBrother** | Services postaux (transport, communication) |

LogisticsSteward est le **ministere du budget** : il decide qui a droit a quoi, sans construire les routes ni appliquer les lois.

---

## 15. Statut Contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il etablit des regles absolues qui ne peuvent etre contournees, negociees, ou modifiees.

Toute implementation de LogisticsSteward doit respecter integralement ce document. Toute evolution de LogisticsSteward doit preserver les invariants definis ici. Toute specialisation de LogisticsSteward doit rester fidele a la nature decrite ici.

---

## 16. Documents Associes

- [LogisticsSteward - Index de Navigation](../_index.md)
- [LogisticsSteward - Architecture & Flows](../architecture/LogisticsSteward%20-%20Architecture%20&%20Flows.md)
- [LogisticsSteward - Quota Definition Contract](../contracts/resources/LogisticsSteward%20-%20Quota%20Definition%20Contract.md)
- [LogisticsSteward - Priority Management Contract](../contracts/resources/LogisticsSteward%20-%20Priority%20Management%20Contract.md)
- [LogisticsSteward - Degradation Strategy Contract](../contracts/degradation/LogisticsSteward%20-%20Degradation%20Strategy%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//..//miyukini-webway-system//reference//_index.md)
- [StrongFather - Documentation Fondatrice](../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)
- [BondingBrother - Documentation Fondatrice](../../BondingBrother/foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** FONDATION â€” Non negociable  
**Reference :** Miyukini Core System v2.4

