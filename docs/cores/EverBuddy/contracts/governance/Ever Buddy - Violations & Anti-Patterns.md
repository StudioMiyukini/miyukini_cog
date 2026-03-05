# Ever Buddy - Violations & Anti-Patterns

## 1. Contexte

Ce document dÃ©finit les **violations des invariants** et les **anti-patterns** liÃ©s Ã  la gouvernance du cycle de vie par Ever Buddy. Il constitue le guide de rÃ©fÃ©rence pour identifier, comprendre, et Ã©viter les pratiques qui compromettent l'intÃ©gritÃ© de l'Ã©volution du systÃ¨me Miyukini.

Chaque invariant de la Documentation Fondatrice d'Ever Buddy (INV-EB-1 Ã  INV-EB-12) implique des violations spÃ©cifiques. Ce document catÃ©gorise ces violations, dÃ©crit leurs consÃ©quences, et fournit des anti-patterns concrets Ã  Ã©viter.

**Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Tous les acteurs interagissant avec Ever Buddy (cores, adaptateurs, produits, OpÃ©rateurs)
- **Audience :** Architectes, dÃ©veloppeurs, auditeurs, Ã©quipes de conformitÃ©
- **Statut :** Contrat normatif â€” Non nÃ©gociable
- **DÃ©pendances :** Documentation Fondatrice Ever Buddy, Invariants & Guarantees, Glossaire Miyukini

---

## 3. Classification des violations

Les violations sont classÃ©es en trois niveaux de gravitÃ© :

| Niveau | Nom | Description | ConsÃ©quence |
|--------|-----|-------------|-------------|
| **V1** | Critique | Violation d'un invariant fondamental | Rejet immÃ©diat, systÃ¨me potentiellement corrompu |
| **V2** | Grave | Violation d'une rÃ¨gle structurelle | Rejet de l'opÃ©ration, alerte Ã©mise |
| **V3** | Mineure | Violation d'une recommandation | Avertissement, correction recommandÃ©e |

**Principe :** Les invariants INV-EB-* gÃ©nÃ¨rent des violations de niveau **V1** ou **V2**. Les violations **V3** concernent les bonnes pratiques non normatives.

---

## 4. Violations par invariant

### 4.1 Violations de INV-EB-1 : Aucune exÃ©cution de migration

**Invariant :**
> Ever Buddy ne possÃ¨de **jamais** la capacitÃ© d'exÃ©cuter une migration, une transformation, ou une modification de donnÃ©es.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-1a** | ExÃ©cution directe de migration | Ever Buddy tente d'exÃ©cuter une migration de donnÃ©es |
| **VIO-EB-1b** | Modification de donnÃ©es | Ever Buddy modifie directement des donnÃ©es gÃ©rÃ©es par KindMother |
| **VIO-EB-1c** | Transformation de structure | Ever Buddy applique une transformation structurelle |
| **VIO-EB-1d** | AccÃ¨s en Ã©criture | Ever Buddy possÃ¨de un mÃ©canisme d'Ã©criture de donnÃ©es |

**ConsÃ©quences :**
- Corruption potentielle des donnÃ©es
- Violation de la sÃ©paration gouvernance/exÃ©cution
- Perte de traÃ§abilitÃ© des modifications
- Conflit d'autoritÃ© avec KindMother

**Anti-patterns associÃ©s :** [AP-01](#ap-01-gouverneur-executant), [AP-02](#ap-02-migration-directe)

---

### 4.2 Violations de INV-EB-2 : TraÃ§abilitÃ© complÃ¨te et immuable

**Invariant :**
> Toute transition d'Ã©tat de cycle de vie est **obligatoirement** enregistrÃ©e et cet enregistrement est **immuable**.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-2a** | Transition non enregistrÃ©e | Une transition d'Ã©tat est effectuÃ©e sans enregistrement |
| **VIO-EB-2b** | Modification d'historique | L'historique des transitions est modifiÃ© |
| **VIO-EB-2c** | Suppression d'enregistrement | Un enregistrement de transition est supprimÃ© |
| **VIO-EB-2d** | Falsification de trace | Les mÃ©tadonnÃ©es d'une transition sont falsifiÃ©es |

**ConsÃ©quences :**
- Perte d'auditabilitÃ©
- ImpossibilitÃ© de comprendre l'Ã©volution passÃ©e
- Violation de la confiance systÃ¨me
- Compromission de la conformitÃ©

**Anti-patterns associÃ©s :** [AP-03](#ap-03-historique-muable), [AP-04](#ap-04-transition-fantome)

---

### 4.3 Violations de INV-EB-3 : Aucun Ã©tat ambigu

**Invariant :**
> Chaque Ã©lÃ©ment du systÃ¨me possÃ¨de **exactement un** Ã©tat de cycle de vie Ã  tout moment.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-3a** | Ã‰tat non dÃ©fini | Un Ã©lÃ©ment n'a aucun Ã©tat de cycle de vie dÃ©clarÃ© |
| **VIO-EB-3b** | Ã‰tats multiples | Un Ã©lÃ©ment possÃ¨de plusieurs Ã©tats simultanÃ©s |
| **VIO-EB-3c** | Ã‰tat intermÃ©diaire | Un Ã©lÃ©ment est dans un Ã©tat "en transition" persistant |
| **VIO-EB-3d** | Ã‰tat invalide | Un Ã©lÃ©ment est dans un Ã©tat non reconnu |

**ConsÃ©quences :**
- Incertitude sur le statut de l'Ã©lÃ©ment
- DÃ©cisions incorrectes des consommateurs
- Comportement imprÃ©visible du systÃ¨me
- Corruption de la gouvernance d'Ã©volution

**Anti-patterns associÃ©s :** [AP-05](#ap-05-etat-schrodinger), [AP-06](#ap-06-etats-paralleles)

---

### 4.4 Violations de INV-EB-4 : PÃ©riode de dÃ©prÃ©ciation obligatoire

**Invariant :**
> Aucun Ã©lÃ©ment ACTIVE ne peut passer directement Ã  RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-4a** | Fast-track ACTIVE â†’ RETIRED | Transition directe sans passer par DEPRECATED |
| **VIO-EB-4b** | Fast-track ACTIVE â†’ ARCHIVED | Archivage direct d'un Ã©lÃ©ment actif |
| **VIO-EB-4c** | PÃ©riode de dÃ©prÃ©ciation nulle | DEPRECATED avec durÃ©e zÃ©ro |
| **VIO-EB-4d** | Contournement d'urgence | Justification "urgente" pour Ã©viter la dÃ©prÃ©ciation |

**ConsÃ©quences :**
- Rupture brutale pour les consommateurs
- Pas de temps de migration
- Violation de la confiance contractuelle
- Pertes potentielles de donnÃ©es ou de service

**Anti-patterns associÃ©s :** [AP-07](#ap-07-retirement-brutal), [AP-08](#ap-08-urgence-permanente)

---

### 4.5 Violations de INV-EB-5 : RÃ©trocompatibilitÃ© par dÃ©faut

**Invariant :**
> Toute Ã©volution est **prÃ©sumÃ©e rÃ©trocompatible** sauf dÃ©claration explicite contraire.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-5a** | Breaking change non dÃ©clarÃ© | Ã‰volution incompatible prÃ©sentÃ©e comme compatible |
| **VIO-EB-5b** | Rupture silencieuse | Changement de comportement sans annonce |
| **VIO-EB-5c** | Version mineure incompatible | Version x.Y.z avec breaking change |
| **VIO-EB-5d** | Absence de plan de transition | Breaking change sans chemin de migration |

**ConsÃ©quences :**
- Consommateurs cassÃ©s sans prÃ©avis
- Perte de confiance
- RÃ©gressions en cascade
- Effort de migration non planifiÃ©

**Anti-patterns associÃ©s :** [AP-09](#ap-09-breaking-change-cache), [AP-10](#ap-10-semver-menteur)

---

### 4.6 Violations de INV-EB-6 : Vision long terme obligatoire

**Invariant :**
> Toute dÃ©cision d'Ã©volution doit considÃ©rer l'impact sur **au moins deux gÃ©nÃ©rations** de versions.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-6a** | Ã‰volution myope | DÃ©cision sans considÃ©ration des impacts futurs |
| **VIO-EB-6b** | Dette transfÃ©rÃ©e | Solution immÃ©diate crÃ©ant un problÃ¨me futur plus grave |
| **VIO-EB-6c** | Absence d'analyse d'impact | Ã‰volution sans Ã©valuation des consÃ©quences |
| **VIO-EB-6d** | IncompatibilitÃ© prÃ©visible | Ã‰volution qui bloquera forcÃ©ment des Ã©volutions futures |

**ConsÃ©quences :**
- Accumulation de dette structurelle
- Ã‰volutions futures bloquÃ©es
- CoÃ»t de maintenance croissant
- Fossilisation progressive du systÃ¨me

**Anti-patterns associÃ©s :** [AP-11](#ap-11-solution-court-termiste), [AP-12](#ap-12-dette-differee)

---

### 4.7 Violations de INV-EB-7 : Documentation obligatoire

**Invariant :**
> Toute transition d'Ã©tat doit Ãªtre **documentÃ©e** avec : raison, impact, chemin de migration, date effective.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-7a** | Transition non documentÃ©e | Transition sans aucune documentation |
| **VIO-EB-7b** | Documentation incomplÃ¨te | Transition avec documentation partielle |
| **VIO-EB-7c** | Raison absente | Transition sans justification |
| **VIO-EB-7d** | Guide de migration manquant | DEPRECATED sans chemin de migration |

**ConsÃ©quences :**
- Consommateurs dÃ©sorientÃ©s
- ImpossibilitÃ© de comprendre les dÃ©cisions
- Migration difficile ou impossible
- Perte de connaissance institutionnelle

**Anti-patterns associÃ©s :** [AP-13](#ap-13-documentation-posteriori), [AP-14](#ap-14-transition-muette)

---

### 4.8 Violations de INV-EB-8 : IndÃ©pendance des dÃ©cisions

**Invariant :**
> Ever Buddy ne peut Ãªtre contraint par un produit, un adaptateur, ou un utilisateur Ã  modifier ses rÃ¨gles de cycle de vie pour un cas particulier.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-8a** | Exception produit | RÃ¨gle modifiÃ©e pour un produit spÃ©cifique |
| **VIO-EB-8b** | Pression externe | Modification de rÃ¨gle sous pression |
| **VIO-EB-8c** | Favoritisme | Traitement diffÃ©renciÃ© selon le demandeur |
| **VIO-EB-8d** | Override utilisateur | Utilisateur contournant les rÃ¨gles d'Ã©volution |

**ConsÃ©quences :**
- Perte d'Ã©quitÃ© du systÃ¨me
- PrÃ©cÃ©dents dangereux
- Ã‰rosion des rÃ¨gles universelles
- Chaos de la gouvernance

**Anti-patterns associÃ©s :** [AP-15](#ap-15-exception-speciale), [AP-16](#ap-16-client-roi)

---

### 4.9 Violations de INV-EB-9 : PrÃ©dictibilitÃ© des transitions

**Invariant :**
> Les rÃ¨gles de transition sont **publiques et stables**. Aucune rÃ¨gle ne peut Ãªtre modifiÃ©e rÃ©troactivement.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-9a** | RÃ¨gle secrÃ¨te | RÃ¨gle de transition non publiÃ©e |
| **VIO-EB-9b** | Modification rÃ©troactive | Changement de rÃ¨gle affectant des transitions passÃ©es |
| **VIO-EB-9c** | RÃ¨gle instable | RÃ¨gle modifiÃ©e frÃ©quemment |
| **VIO-EB-9d** | Application incohÃ©rente | RÃ¨gle appliquÃ©e diffÃ©remment selon les cas |

**ConsÃ©quences :**
- ImpossibilitÃ© de planifier les Ã©volutions
- Perte de confiance des consommateurs
- ImprÃ©visibilitÃ© du systÃ¨me
- DÃ©cisions arbitraires

**Anti-patterns associÃ©s :** [AP-17](#ap-17-regles-mouvantes), [AP-18](#ap-18-retroactivite)

---

### 4.10 Violations de INV-EB-10 : UnicitÃ© du successeur dÃ©clarÃ©

**Invariant :**
> Un Ã©lÃ©ment dÃ©prÃ©ciÃ© possÃ¨de **au plus un** successeur dÃ©clarÃ© Ã  tout moment.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-10a** | Successeurs multiples | Plusieurs successeurs officiels dÃ©clarÃ©s |
| **VIO-EB-10b** | Successeur non dÃ©signÃ© | Aucun successeur malgrÃ© des alternatives |
| **VIO-EB-10c** | Successeur ambigu | Successeur mal dÃ©fini ou confus |
| **VIO-EB-10d** | Changement de successeur non documentÃ© | Le successeur change sans annonce |

**ConsÃ©quences :**
- Confusion sur le chemin de migration
- Effort de migration gaspillÃ©
- Fragmentation des consommateurs
- Incertitude prolongÃ©e

**Anti-patterns associÃ©s :** [AP-19](#ap-19-successeurs-concurrents), [AP-20](#ap-20-successeur-fantome)

---

### 4.11 Violations de INV-EB-11 : Non-rÃ©troactivitÃ© des changements de rÃ¨gles

**Invariant :**
> Les rÃ¨gles d'Ã©volution s'appliquent aux transitions **futures**. Un changement de rÃ¨gle ne peut pas modifier le statut d'Ã©lÃ©ments dÃ©jÃ  en transition.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-11a** | Application rÃ©troactive | Nouvelle rÃ¨gle appliquÃ©e Ã  une transition en cours |
| **VIO-EB-11b** | Annulation de transition | Transition valide annulÃ©e par nouvelle rÃ¨gle |
| **VIO-EB-11c** | Modification de pÃ©riode en cours | PÃ©riode de dÃ©prÃ©ciation modifiÃ©e aprÃ¨s dÃ©but |
| **VIO-EB-11d** | Changement de successeur forcÃ© | Successeur changÃ© pour une dÃ©prÃ©ciation en cours |

**ConsÃ©quences :**
- Violation de la confiance contractuelle
- Transitions perturbÃ©es
- Planification impossible
- Chaos pour les consommateurs engagÃ©s dans une migration

**Anti-patterns associÃ©s :** [AP-18](#ap-18-retroactivite), [AP-21](#ap-21-regles-a-geometrie-variable)

---

### 4.12 Violations de INV-EB-12 : ResponsabilitÃ© de l'annonce

**Invariant :**
> Ever Buddy est **responsable** de l'annonce des transitions, mais les cores et produits sont **responsables** de rÃ©agir Ã  ces annonces.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-12a** | Annonce manquante | Transition sans annonce prÃ©alable |
| **VIO-EB-12b** | Annonce tardive | Annonce effectuÃ©e aprÃ¨s le dÃ©but de transition |
| **VIO-EB-12c** | Annonce incomplÃ¨te | Annonce sans toutes les informations requises |
| **VIO-EB-12d** | Canal d'annonce inadÃ©quat | Annonce par un canal non surveillÃ© |

**ConsÃ©quences :**
- Consommateurs non prÃ©parÃ©s
- Migrations d'urgence forcÃ©es
- ResponsabilitÃ©s floues
- Ã‰checs de transition Ã©vitables

**Anti-patterns associÃ©s :** [AP-22](#ap-22-annonce-invisible), [AP-23](#ap-23-derniere-minute)

---

## 5. Anti-patterns dÃ©taillÃ©s

### AP-01 : Gouverneur-ExÃ©cutant

**Description :** Ever Buddy tente d'exÃ©cuter directement les migrations au lieu de simplement les gouverner.

**SymptÃ´mes :**
- Code d'exÃ©cution de migration dans Ever Buddy
- Appels directs aux APIs de donnÃ©es
- Transformations de donnÃ©es effectuÃ©es par Ever Buddy

**Correction :** Ever Buddy dÃ©finit les rÃ¨gles de migration, KindMother ou les produits exÃ©cutent.

**Violations associÃ©es :** VIO-EB-1a, VIO-EB-1b, VIO-EB-1c

---

### AP-02 : Migration Directe

**Description :** Les migrations sont dÃ©clenchÃ©es directement sans passer par le cycle de gouvernance.

**SymptÃ´mes :**
- Migrations non tracÃ©es
- Absence de validation Ever Buddy
- Changements structurels "sauvages"

**Correction :** Toute migration doit Ãªtre dÃ©clarÃ©e Ã  Ever Buddy et suivre le cycle de vie.

**Violations associÃ©es :** VIO-EB-1a, VIO-EB-2a

---

### AP-03 : Historique Muable

**Description :** L'historique des transitions peut Ãªtre modifiÃ© aprÃ¨s coup.

**SymptÃ´mes :**
- Corrections d'historique
- Suppressions d'entrÃ©es anciennes
- "Nettoyage" de l'historique

**Correction :** L'historique est append-only, immuable, jamais modifiable.

**Violations associÃ©es :** VIO-EB-2b, VIO-EB-2c, VIO-EB-2d

---

### AP-04 : Transition FantÃ´me

**Description :** Des transitions d'Ã©tat se produisent sans Ãªtre enregistrÃ©es.

**SymptÃ´mes :**
- Ã‰tat actuel ne correspondant pas Ã  l'historique
- Gaps dans la chaÃ®ne de transitions
- Ã‰tats "magiquement" changÃ©s

**Correction :** Toute transition passe par Ever Buddy et est atomiquement enregistrÃ©e.

**Violations associÃ©es :** VIO-EB-2a

---

### AP-05 : Ã‰tat SchrÃ¶dinger

**Description :** Un Ã©lÃ©ment n'a pas d'Ã©tat dÃ©fini ou son Ã©tat est incertain.

**SymptÃ´mes :**
- Ã‰lÃ©ment sans champ d'Ã©tat
- Ã‰tat null ou undefined
- "Nous ne savons pas si c'est actif ou non"

**Correction :** Tout Ã©lÃ©ment gouvernÃ© a un Ã©tat explicite dÃ¨s sa crÃ©ation (DRAFT par dÃ©faut).

**Violations associÃ©es :** VIO-EB-3a, VIO-EB-3d

---

### AP-06 : Ã‰tats ParallÃ¨les

**Description :** Un Ã©lÃ©ment est considÃ©rÃ© dans plusieurs Ã©tats simultanÃ©ment.

**SymptÃ´mes :**
- "C'est dÃ©prÃ©ciÃ© mais aussi actif"
- Ã‰tats conditionnels selon le contexte
- "Pour certains consommateurs c'est actif, pour d'autres dÃ©prÃ©ciÃ©"

**Correction :** Un Ã©lÃ©ment = un Ã©tat, universel et non contextuel.

**Violations associÃ©es :** VIO-EB-3b, VIO-EB-3c

---

### AP-07 : Retirement Brutal

**Description :** Un Ã©lÃ©ment est retirÃ© sans pÃ©riode de dÃ©prÃ©ciation.

**SymptÃ´mes :**
- ACTIVE â†’ RETIRED direct
- "On n'a pas le temps de dÃ©prÃ©cier"
- Ã‰lÃ©ments qui disparaissent sans prÃ©avis

**Correction :** PÃ©riode de dÃ©prÃ©ciation obligatoire, minimum 1 cycle de release.

**Violations associÃ©es :** VIO-EB-4a, VIO-EB-4b

---

### AP-08 : Urgence Permanente

**Description :** Invocation constante de l'urgence pour contourner les rÃ¨gles de dÃ©prÃ©ciation.

**SymptÃ´mes :**
- "C'est urgent" comme justification systÃ©matique
- DÃ©rogations frÃ©quentes aux pÃ©riodes minimales
- Culture du fast-track

**Correction :** L'urgence ne justifie pas la violation des invariants. Planifier mieux.

**Violations associÃ©es :** VIO-EB-4c, VIO-EB-4d

---

### AP-09 : Breaking Change CachÃ©

**Description :** Un changement incompatible est prÃ©sentÃ© comme rÃ©trocompatible.

**SymptÃ´mes :**
- "C'est juste une petite modification"
- Consommateurs cassÃ©s par surprise
- Pas de dÃ©claration d'incompatibilitÃ©

**Correction :** Tout breaking change doit Ãªtre explicitement dÃ©clarÃ© et gÃ©rÃ© par dÃ©prÃ©ciation.

**Violations associÃ©es :** VIO-EB-5a, VIO-EB-5b

---

### AP-10 : SemVer Menteur

**Description :** Le versionnement sÃ©mantique est utilisÃ© de maniÃ¨re trompeuse.

**SymptÃ´mes :**
- Breaking changes en version mineure (x.Y.z)
- Version majeure pour des corrections mineures
- Versionnement marketing plutÃ´t que technique

**Correction :** Respecter strictement le versionnement sÃ©mantique (majeur = incompatible).

**Violations associÃ©es :** VIO-EB-5c

---

### AP-11 : Solution Court-Termiste

**Description :** Adopter une solution rapide qui crÃ©e des problÃ¨mes futurs plus graves.

**SymptÃ´mes :**
- "On verra plus tard"
- Solutions qui bloquent des Ã©volutions futures
- Absence d'analyse d'impact Ã  long terme

**Correction :** Ã‰valuer l'impact sur au moins deux gÃ©nÃ©rations avant toute dÃ©cision.

**Violations associÃ©es :** VIO-EB-6a, VIO-EB-6b

---

### AP-12 : Dette DiffÃ©rÃ©e

**Description :** TransfÃ©rer systÃ©matiquement la dette structurelle vers le futur.

**SymptÃ´mes :**
- Accumulation d'Ã©lÃ©ments DEPRECATED non rÃ©solus
- "On nettoiera plus tard"
- Dette croissante sans plan de rÃ©duction

**Correction :** Traiter la dette structurelle de maniÃ¨re continue, pas diffÃ©rÃ©e.

**Violations associÃ©es :** VIO-EB-6b, VIO-EB-6c

---

### AP-13 : Documentation Ã€ Posteriori

**Description :** Documenter les transitions aprÃ¨s leur exÃ©cution plutÃ´t qu'avant.

**SymptÃ´mes :**
- Documentation rÃ©digÃ©e aprÃ¨s la transition
- "On documentera quand on aura le temps"
- Informations incomplÃ¨tes ou oubliÃ©es

**Correction :** La documentation fait partie de la transition, pas un ajout ultÃ©rieur.

**Violations associÃ©es :** VIO-EB-7a, VIO-EB-7b

---

### AP-14 : Transition Muette

**Description :** Transitions effectuÃ©es sans communication aux parties prenantes.

**SymptÃ´mes :**
- "On ne savait pas que c'Ã©tait dÃ©prÃ©ciÃ©"
- Consommateurs dÃ©couvrant les changements par accident
- Absence de canal de communication

**Correction :** Annonce proactive via les canaux appropriÃ©s avant toute transition.

**Violations associÃ©es :** VIO-EB-7c, VIO-EB-12a

---

### AP-15 : Exception SpÃ©ciale

**Description :** CrÃ©er des exceptions aux rÃ¨gles pour des cas particuliers.

**SymptÃ´mes :**
- "Pour ce produit, on fait une exception"
- RÃ¨gles Ã  gÃ©omÃ©trie variable
- Accumulation d'exceptions

**Correction :** Les rÃ¨gles sont universelles. Pas d'exception, pas de favoritisme.

**Violations associÃ©es :** VIO-EB-8a, VIO-EB-8c

---

### AP-16 : Client Roi

**Description :** Modifier les rÃ¨gles sous la pression d'un client ou d'un utilisateur.

**SymptÃ´mes :**
- "Le client X exige que..."
- RÃ¨gles assouplies pour des clients importants
- Gouvernance soumise aux intÃ©rÃªts commerciaux

**Correction :** Ever Buddy est indÃ©pendant. Les rÃ¨gles ne se nÃ©gocient pas.

**Violations associÃ©es :** VIO-EB-8b, VIO-EB-8d

---

### AP-17 : RÃ¨gles Mouvantes

**Description :** Les rÃ¨gles de transition changent frÃ©quemment sans stabilitÃ©.

**SymptÃ´mes :**
- RÃ¨gles diffÃ©rentes d'un mois Ã  l'autre
- "Maintenant on fait comme Ã§a"
- Consommateurs perdus face aux changements

**Correction :** Les rÃ¨gles sont stables. Toute modification est exceptionnelle et annoncÃ©e.

**Violations associÃ©es :** VIO-EB-9c, VIO-EB-9d

---

### AP-18 : RÃ©troactivitÃ©

**Description :** Appliquer de nouvelles rÃ¨gles Ã  des situations passÃ©es ou en cours.

**SymptÃ´mes :**
- "Avec les nouvelles rÃ¨gles, cette transition est invalide"
- Annulation de dÃ©cisions passÃ©es
- Modifications de pÃ©riodes en cours

**Correction :** Les nouvelles rÃ¨gles s'appliquent aux futures transitions uniquement.

**Violations associÃ©es :** VIO-EB-9b, VIO-EB-11a, VIO-EB-11b

---

### AP-19 : Successeurs Concurrents

**Description :** Plusieurs successeurs officiels sont dÃ©clarÃ©s pour un mÃªme Ã©lÃ©ment dÃ©prÃ©ciÃ©.

**SymptÃ´mes :**
- "Vous pouvez migrer vers A ou B"
- CompÃ©tition entre successeurs
- Consommateurs divisÃ©s

**Correction :** Un seul successeur principal. Les alternatives sont documentÃ©es mais non officielles.

**Violations associÃ©es :** VIO-EB-10a

---

### AP-20 : Successeur FantÃ´me

**Description :** Aucun successeur n'est dÃ©signÃ© malgrÃ© l'existence d'alternatives.

**SymptÃ´mes :**
- "C'est dÃ©prÃ©ciÃ© mais on ne sait pas par quoi le remplacer"
- Migration impossible par manque d'information
- Consommateurs bloquÃ©s

**Correction :** DÃ©clarer explicitement le successeur (ou "aucun" si abandon volontaire).

**Violations associÃ©es :** VIO-EB-10b, VIO-EB-10c

---

### AP-21 : RÃ¨gles Ã  GÃ©omÃ©trie Variable

**Description :** Les rÃ¨gles sont appliquÃ©es diffÃ©remment selon les circonstances.

**SymptÃ´mes :**
- Deux Ã©lÃ©ments similaires traitÃ©s diffÃ©remment
- InterprÃ©tations variables des rÃ¨gles
- "Ã‡a dÃ©pend du contexte"

**Correction :** Application uniforme et prÃ©visible des rÃ¨gles, sans exception.

**Violations associÃ©es :** VIO-EB-9d, VIO-EB-11c, VIO-EB-11d

---

### AP-22 : Annonce Invisible

**Description :** L'annonce de transition existe mais n'est pas visible par les consommateurs.

**SymptÃ´mes :**
- Annonce dans un canal non surveillÃ©
- Documentation technique obscure
- "C'Ã©tait Ã©crit quelque part"

**Correction :** Utiliser des canaux de communication actifs et vÃ©rifier la rÃ©ception.

**Violations associÃ©es :** VIO-EB-12c, VIO-EB-12d

---

### AP-23 : DerniÃ¨re Minute

**Description :** Annoncer les transitions au dernier moment.

**SymptÃ´mes :**
- Annonce quelques jours avant la transition
- Pas de temps de prÃ©paration
- Migrations d'urgence forcÃ©es

**Correction :** Respecter les pÃ©riodes minimales d'annonce dÃ©finies par catÃ©gorie d'Ã©lÃ©ment.

**Violations associÃ©es :** VIO-EB-12a, VIO-EB-12b

---

## 6. Tableau rÃ©capitulatif des violations

| Invariant | Violations | GravitÃ© | Anti-patterns |
|-----------|------------|---------|---------------|
| INV-EB-1 | VIO-EB-1a, 1b, 1c, 1d | V1 | AP-01, AP-02 |
| INV-EB-2 | VIO-EB-2a, 2b, 2c, 2d | V1 | AP-03, AP-04 |
| INV-EB-3 | VIO-EB-3a, 3b, 3c, 3d | V1 | AP-05, AP-06 |
| INV-EB-4 | VIO-EB-4a, 4b, 4c, 4d | V1 | AP-07, AP-08 |
| INV-EB-5 | VIO-EB-5a, 5b, 5c, 5d | V2 | AP-09, AP-10 |
| INV-EB-6 | VIO-EB-6a, 6b, 6c, 6d | V2 | AP-11, AP-12 |
| INV-EB-7 | VIO-EB-7a, 7b, 7c, 7d | V2 | AP-13, AP-14 |
| INV-EB-8 | VIO-EB-8a, 8b, 8c, 8d | V1 | AP-15, AP-16 |
| INV-EB-9 | VIO-EB-9a, 9b, 9c, 9d | V1 | AP-17, AP-18 |
| INV-EB-10 | VIO-EB-10a, 10b, 10c, 10d | V2 | AP-19, AP-20 |
| INV-EB-11 | VIO-EB-11a, 11b, 11c, 11d | V1 | AP-18, AP-21 |
| INV-EB-12 | VIO-EB-12a, 12b, 12c, 12d | V2 | AP-22, AP-23 |

---

## 7. DÃ©tection et prÃ©vention

### 7.1 MÃ©canismes de dÃ©tection

| MÃ©canisme | Violations dÃ©tectÃ©es | Moment |
|-----------|---------------------|--------|
| **Validation prÃ©-transition** | VIO-EB-3*, VIO-EB-4*, VIO-EB-10* | Avant transition |
| **Audit d'historique** | VIO-EB-2* | Continu |
| **VÃ©rification de documentation** | VIO-EB-7* | Avant transition |
| **ContrÃ´le de compatibilitÃ©** | VIO-EB-5* | Ã€ chaque Ã©volution |
| **Monitoring de rÃ¨gles** | VIO-EB-8*, VIO-EB-9* | Continu |
| **VÃ©rification d'annonce** | VIO-EB-12* | Avant transition |

### 7.2 PrÃ©vention par conception

| Principe | Description | Violations prÃ©venues |
|----------|-------------|---------------------|
| **SÃ©paration stricte** | Ever Buddy n'a aucun accÃ¨s en Ã©criture aux donnÃ©es | VIO-EB-1* |
| **Historique append-only** | Aucune API de modification d'historique | VIO-EB-2* |
| **Ã‰tat obligatoire** | Champ d'Ã©tat requis, non nullable | VIO-EB-3* |
| **Matrice de transitions** | Transitions invalides bloquÃ©es structurellement | VIO-EB-4* |
| **Validation de version** | ContrÃ´le automatique du versionnement sÃ©mantique | VIO-EB-5* |
| **RÃ¨gles immuables** | RÃ¨gles versionnÃ©es et non modifiables rÃ©troactivement | VIO-EB-9*, VIO-EB-11* |

### 7.3 Alertes et escalade

| Niveau de violation | Action | Escalade |
|---------------------|--------|----------|
| **V1 - Critique** | Rejet immÃ©diat, alerte systÃ¨me | TAMR (intervention humaine) |
| **V2 - Grave** | Rejet, alerte | Caring Nanny (observation) |
| **V3 - Mineure** | Avertissement | Log uniquement |

---

## 8. ConformitÃ© aux Lois d'Autonomie

Ce contrat respecte les Lois d'Autonomie SystÃ¨me :

| Loi | ConformitÃ© | Application |
|-----|------------|-------------|
| **LOI-1** | âœ… | DÃ©tection de violations locale, pas de dÃ©pendance externe |
| **LOI-2** | âœ… | Violations dÃ©tectables en mode isolÃ© |
| **LOI-3** | âœ… | Ã‰tat de violation souverain localement |
| **LOI-4** | âœ… | DÃ©tection basÃ©e sur Ã©tats, pas sur temps global |

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 9. RÃ©fÃ©rences croisÃ©es

- **Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)
- **Contrat complÃ©mentaire :** [Ever Buddy - Invariants & Guarantees](./Ever%20Buddy%20-%20Invariants%20%26%20Guarantees.md) (dÃ©finitions des invariants)
- **Ã‰tats de cycle de vie :** [Ever Buddy - Lifecycle States Contract](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md)
- **RÃ¨gles de transition :** [Ever Buddy - Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md)
- **Glossaire :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)
- **Lois d'Autonomie :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif â€” Non nÃ©gociable  
**DÃ©rivÃ© de :** Ever Buddy - Documentation Fondatrice v1.3, Section 7 (Invariants)  
**Type :** Contrat de gouvernance - Violations et Anti-Patterns

