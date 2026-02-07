# Ever Buddy - Violations & Anti-Patterns

## 1. Contexte

Ce document définit les **violations des invariants** et les **anti-patterns** liés à la gouvernance du cycle de vie par Ever Buddy. Il constitue le guide de référence pour identifier, comprendre, et éviter les pratiques qui compromettent l'intégrité de l'évolution du système Miyukini.

Chaque invariant de la Documentation Fondatrice d'Ever Buddy (INV-EB-1 à INV-EB-12) implique des violations spécifiques. Ce document catégorise ces violations, décrit leurs conséquences, et fournit des anti-patterns concrets à éviter.

**Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

- **Applicable à :** Tous les acteurs interagissant avec Ever Buddy (cores, adaptateurs, produits, Opérateurs)
- **Audience :** Architectes, développeurs, auditeurs, équipes de conformité
- **Statut :** Contrat normatif — Non négociable
- **Dépendances :** Documentation Fondatrice Ever Buddy, Invariants & Guarantees, Glossaire Miyukini

---

## 3. Classification des violations

Les violations sont classées en trois niveaux de gravité :

| Niveau | Nom | Description | Conséquence |
|--------|-----|-------------|-------------|
| **V1** | Critique | Violation d'un invariant fondamental | Rejet immédiat, système potentiellement corrompu |
| **V2** | Grave | Violation d'une règle structurelle | Rejet de l'opération, alerte émise |
| **V3** | Mineure | Violation d'une recommandation | Avertissement, correction recommandée |

**Principe :** Les invariants INV-EB-* génèrent des violations de niveau **V1** ou **V2**. Les violations **V3** concernent les bonnes pratiques non normatives.

---

## 4. Violations par invariant

### 4.1 Violations de INV-EB-1 : Aucune exécution de migration

**Invariant :**
> Ever Buddy ne possède **jamais** la capacité d'exécuter une migration, une transformation, ou une modification de données.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-1a** | Exécution directe de migration | Ever Buddy tente d'exécuter une migration de données |
| **VIO-EB-1b** | Modification de données | Ever Buddy modifie directement des données gérées par KindMother |
| **VIO-EB-1c** | Transformation de structure | Ever Buddy applique une transformation structurelle |
| **VIO-EB-1d** | Accès en écriture | Ever Buddy possède un mécanisme d'écriture de données |

**Conséquences :**
- Corruption potentielle des données
- Violation de la séparation gouvernance/exécution
- Perte de traçabilité des modifications
- Conflit d'autorité avec KindMother

**Anti-patterns associés :** [AP-01](#ap-01-gouverneur-executant), [AP-02](#ap-02-migration-directe)

---

### 4.2 Violations de INV-EB-2 : Traçabilité complète et immuable

**Invariant :**
> Toute transition d'état de cycle de vie est **obligatoirement** enregistrée et cet enregistrement est **immuable**.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-2a** | Transition non enregistrée | Une transition d'état est effectuée sans enregistrement |
| **VIO-EB-2b** | Modification d'historique | L'historique des transitions est modifié |
| **VIO-EB-2c** | Suppression d'enregistrement | Un enregistrement de transition est supprimé |
| **VIO-EB-2d** | Falsification de trace | Les métadonnées d'une transition sont falsifiées |

**Conséquences :**
- Perte d'auditabilité
- Impossibilité de comprendre l'évolution passée
- Violation de la confiance système
- Compromission de la conformité

**Anti-patterns associés :** [AP-03](#ap-03-historique-muable), [AP-04](#ap-04-transition-fantome)

---

### 4.3 Violations de INV-EB-3 : Aucun état ambigu

**Invariant :**
> Chaque élément du système possède **exactement un** état de cycle de vie à tout moment.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-3a** | État non défini | Un élément n'a aucun état de cycle de vie déclaré |
| **VIO-EB-3b** | États multiples | Un élément possède plusieurs états simultanés |
| **VIO-EB-3c** | État intermédiaire | Un élément est dans un état "en transition" persistant |
| **VIO-EB-3d** | État invalide | Un élément est dans un état non reconnu |

**Conséquences :**
- Incertitude sur le statut de l'élément
- Décisions incorrectes des consommateurs
- Comportement imprévisible du système
- Corruption de la gouvernance d'évolution

**Anti-patterns associés :** [AP-05](#ap-05-etat-schrodinger), [AP-06](#ap-06-etats-paralleles)

---

### 4.4 Violations de INV-EB-4 : Période de dépréciation obligatoire

**Invariant :**
> Aucun élément ACTIVE ne peut passer directement à RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-4a** | Fast-track ACTIVE → RETIRED | Transition directe sans passer par DEPRECATED |
| **VIO-EB-4b** | Fast-track ACTIVE → ARCHIVED | Archivage direct d'un élément actif |
| **VIO-EB-4c** | Période de dépréciation nulle | DEPRECATED avec durée zéro |
| **VIO-EB-4d** | Contournement d'urgence | Justification "urgente" pour éviter la dépréciation |

**Conséquences :**
- Rupture brutale pour les consommateurs
- Pas de temps de migration
- Violation de la confiance contractuelle
- Pertes potentielles de données ou de service

**Anti-patterns associés :** [AP-07](#ap-07-retirement-brutal), [AP-08](#ap-08-urgence-permanente)

---

### 4.5 Violations de INV-EB-5 : Rétrocompatibilité par défaut

**Invariant :**
> Toute évolution est **présumée rétrocompatible** sauf déclaration explicite contraire.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-5a** | Breaking change non déclaré | Évolution incompatible présentée comme compatible |
| **VIO-EB-5b** | Rupture silencieuse | Changement de comportement sans annonce |
| **VIO-EB-5c** | Version mineure incompatible | Version x.Y.z avec breaking change |
| **VIO-EB-5d** | Absence de plan de transition | Breaking change sans chemin de migration |

**Conséquences :**
- Consommateurs cassés sans préavis
- Perte de confiance
- Régressions en cascade
- Effort de migration non planifié

**Anti-patterns associés :** [AP-09](#ap-09-breaking-change-cache), [AP-10](#ap-10-semver-menteur)

---

### 4.6 Violations de INV-EB-6 : Vision long terme obligatoire

**Invariant :**
> Toute décision d'évolution doit considérer l'impact sur **au moins deux générations** de versions.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-6a** | Évolution myope | Décision sans considération des impacts futurs |
| **VIO-EB-6b** | Dette transférée | Solution immédiate créant un problème futur plus grave |
| **VIO-EB-6c** | Absence d'analyse d'impact | Évolution sans évaluation des conséquences |
| **VIO-EB-6d** | Incompatibilité prévisible | Évolution qui bloquera forcément des évolutions futures |

**Conséquences :**
- Accumulation de dette structurelle
- Évolutions futures bloquées
- Coût de maintenance croissant
- Fossilisation progressive du système

**Anti-patterns associés :** [AP-11](#ap-11-solution-court-termiste), [AP-12](#ap-12-dette-differee)

---

### 4.7 Violations de INV-EB-7 : Documentation obligatoire

**Invariant :**
> Toute transition d'état doit être **documentée** avec : raison, impact, chemin de migration, date effective.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-7a** | Transition non documentée | Transition sans aucune documentation |
| **VIO-EB-7b** | Documentation incomplète | Transition avec documentation partielle |
| **VIO-EB-7c** | Raison absente | Transition sans justification |
| **VIO-EB-7d** | Guide de migration manquant | DEPRECATED sans chemin de migration |

**Conséquences :**
- Consommateurs désorientés
- Impossibilité de comprendre les décisions
- Migration difficile ou impossible
- Perte de connaissance institutionnelle

**Anti-patterns associés :** [AP-13](#ap-13-documentation-posteriori), [AP-14](#ap-14-transition-muette)

---

### 4.8 Violations de INV-EB-8 : Indépendance des décisions

**Invariant :**
> Ever Buddy ne peut être contraint par un produit, un adaptateur, ou un utilisateur à modifier ses règles de cycle de vie pour un cas particulier.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-8a** | Exception produit | Règle modifiée pour un produit spécifique |
| **VIO-EB-8b** | Pression externe | Modification de règle sous pression |
| **VIO-EB-8c** | Favoritisme | Traitement différencié selon le demandeur |
| **VIO-EB-8d** | Override utilisateur | Utilisateur contournant les règles d'évolution |

**Conséquences :**
- Perte d'équité du système
- Précédents dangereux
- Érosion des règles universelles
- Chaos de la gouvernance

**Anti-patterns associés :** [AP-15](#ap-15-exception-speciale), [AP-16](#ap-16-client-roi)

---

### 4.9 Violations de INV-EB-9 : Prédictibilité des transitions

**Invariant :**
> Les règles de transition sont **publiques et stables**. Aucune règle ne peut être modifiée rétroactivement.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-9a** | Règle secrète | Règle de transition non publiée |
| **VIO-EB-9b** | Modification rétroactive | Changement de règle affectant des transitions passées |
| **VIO-EB-9c** | Règle instable | Règle modifiée fréquemment |
| **VIO-EB-9d** | Application incohérente | Règle appliquée différemment selon les cas |

**Conséquences :**
- Impossibilité de planifier les évolutions
- Perte de confiance des consommateurs
- Imprévisibilité du système
- Décisions arbitraires

**Anti-patterns associés :** [AP-17](#ap-17-regles-mouvantes), [AP-18](#ap-18-retroactivite)

---

### 4.10 Violations de INV-EB-10 : Unicité du successeur déclaré

**Invariant :**
> Un élément déprécié possède **au plus un** successeur déclaré à tout moment.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-10a** | Successeurs multiples | Plusieurs successeurs officiels déclarés |
| **VIO-EB-10b** | Successeur non désigné | Aucun successeur malgré des alternatives |
| **VIO-EB-10c** | Successeur ambigu | Successeur mal défini ou confus |
| **VIO-EB-10d** | Changement de successeur non documenté | Le successeur change sans annonce |

**Conséquences :**
- Confusion sur le chemin de migration
- Effort de migration gaspillé
- Fragmentation des consommateurs
- Incertitude prolongée

**Anti-patterns associés :** [AP-19](#ap-19-successeurs-concurrents), [AP-20](#ap-20-successeur-fantome)

---

### 4.11 Violations de INV-EB-11 : Non-rétroactivité des changements de règles

**Invariant :**
> Les règles d'évolution s'appliquent aux transitions **futures**. Un changement de règle ne peut pas modifier le statut d'éléments déjà en transition.

**Violations (V1 - Critique) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-11a** | Application rétroactive | Nouvelle règle appliquée à une transition en cours |
| **VIO-EB-11b** | Annulation de transition | Transition valide annulée par nouvelle règle |
| **VIO-EB-11c** | Modification de période en cours | Période de dépréciation modifiée après début |
| **VIO-EB-11d** | Changement de successeur forcé | Successeur changé pour une dépréciation en cours |

**Conséquences :**
- Violation de la confiance contractuelle
- Transitions perturbées
- Planification impossible
- Chaos pour les consommateurs engagés dans une migration

**Anti-patterns associés :** [AP-18](#ap-18-retroactivite), [AP-21](#ap-21-regles-a-geometrie-variable)

---

### 4.12 Violations de INV-EB-12 : Responsabilité de l'annonce

**Invariant :**
> Ever Buddy est **responsable** de l'annonce des transitions, mais les cores et produits sont **responsables** de réagir à ces annonces.

**Violations (V2 - Grave) :**

| ID | Violation | Description |
|----|-----------|-------------|
| **VIO-EB-12a** | Annonce manquante | Transition sans annonce préalable |
| **VIO-EB-12b** | Annonce tardive | Annonce effectuée après le début de transition |
| **VIO-EB-12c** | Annonce incomplète | Annonce sans toutes les informations requises |
| **VIO-EB-12d** | Canal d'annonce inadéquat | Annonce par un canal non surveillé |

**Conséquences :**
- Consommateurs non préparés
- Migrations d'urgence forcées
- Responsabilités floues
- Échecs de transition évitables

**Anti-patterns associés :** [AP-22](#ap-22-annonce-invisible), [AP-23](#ap-23-derniere-minute)

---

## 5. Anti-patterns détaillés

### AP-01 : Gouverneur-Exécutant

**Description :** Ever Buddy tente d'exécuter directement les migrations au lieu de simplement les gouverner.

**Symptômes :**
- Code d'exécution de migration dans Ever Buddy
- Appels directs aux APIs de données
- Transformations de données effectuées par Ever Buddy

**Correction :** Ever Buddy définit les règles de migration, KindMother ou les produits exécutent.

**Violations associées :** VIO-EB-1a, VIO-EB-1b, VIO-EB-1c

---

### AP-02 : Migration Directe

**Description :** Les migrations sont déclenchées directement sans passer par le cycle de gouvernance.

**Symptômes :**
- Migrations non tracées
- Absence de validation Ever Buddy
- Changements structurels "sauvages"

**Correction :** Toute migration doit être déclarée à Ever Buddy et suivre le cycle de vie.

**Violations associées :** VIO-EB-1a, VIO-EB-2a

---

### AP-03 : Historique Muable

**Description :** L'historique des transitions peut être modifié après coup.

**Symptômes :**
- Corrections d'historique
- Suppressions d'entrées anciennes
- "Nettoyage" de l'historique

**Correction :** L'historique est append-only, immuable, jamais modifiable.

**Violations associées :** VIO-EB-2b, VIO-EB-2c, VIO-EB-2d

---

### AP-04 : Transition Fantôme

**Description :** Des transitions d'état se produisent sans être enregistrées.

**Symptômes :**
- État actuel ne correspondant pas à l'historique
- Gaps dans la chaîne de transitions
- États "magiquement" changés

**Correction :** Toute transition passe par Ever Buddy et est atomiquement enregistrée.

**Violations associées :** VIO-EB-2a

---

### AP-05 : État Schrödinger

**Description :** Un élément n'a pas d'état défini ou son état est incertain.

**Symptômes :**
- Élément sans champ d'état
- État null ou undefined
- "Nous ne savons pas si c'est actif ou non"

**Correction :** Tout élément gouverné a un état explicite dès sa création (DRAFT par défaut).

**Violations associées :** VIO-EB-3a, VIO-EB-3d

---

### AP-06 : États Parallèles

**Description :** Un élément est considéré dans plusieurs états simultanément.

**Symptômes :**
- "C'est déprécié mais aussi actif"
- États conditionnels selon le contexte
- "Pour certains consommateurs c'est actif, pour d'autres déprécié"

**Correction :** Un élément = un état, universel et non contextuel.

**Violations associées :** VIO-EB-3b, VIO-EB-3c

---

### AP-07 : Retirement Brutal

**Description :** Un élément est retiré sans période de dépréciation.

**Symptômes :**
- ACTIVE → RETIRED direct
- "On n'a pas le temps de déprécier"
- Éléments qui disparaissent sans préavis

**Correction :** Période de dépréciation obligatoire, minimum 1 cycle de release.

**Violations associées :** VIO-EB-4a, VIO-EB-4b

---

### AP-08 : Urgence Permanente

**Description :** Invocation constante de l'urgence pour contourner les règles de dépréciation.

**Symptômes :**
- "C'est urgent" comme justification systématique
- Dérogations fréquentes aux périodes minimales
- Culture du fast-track

**Correction :** L'urgence ne justifie pas la violation des invariants. Planifier mieux.

**Violations associées :** VIO-EB-4c, VIO-EB-4d

---

### AP-09 : Breaking Change Caché

**Description :** Un changement incompatible est présenté comme rétrocompatible.

**Symptômes :**
- "C'est juste une petite modification"
- Consommateurs cassés par surprise
- Pas de déclaration d'incompatibilité

**Correction :** Tout breaking change doit être explicitement déclaré et géré par dépréciation.

**Violations associées :** VIO-EB-5a, VIO-EB-5b

---

### AP-10 : SemVer Menteur

**Description :** Le versionnement sémantique est utilisé de manière trompeuse.

**Symptômes :**
- Breaking changes en version mineure (x.Y.z)
- Version majeure pour des corrections mineures
- Versionnement marketing plutôt que technique

**Correction :** Respecter strictement le versionnement sémantique (majeur = incompatible).

**Violations associées :** VIO-EB-5c

---

### AP-11 : Solution Court-Termiste

**Description :** Adopter une solution rapide qui crée des problèmes futurs plus graves.

**Symptômes :**
- "On verra plus tard"
- Solutions qui bloquent des évolutions futures
- Absence d'analyse d'impact à long terme

**Correction :** Évaluer l'impact sur au moins deux générations avant toute décision.

**Violations associées :** VIO-EB-6a, VIO-EB-6b

---

### AP-12 : Dette Différée

**Description :** Transférer systématiquement la dette structurelle vers le futur.

**Symptômes :**
- Accumulation d'éléments DEPRECATED non résolus
- "On nettoiera plus tard"
- Dette croissante sans plan de réduction

**Correction :** Traiter la dette structurelle de manière continue, pas différée.

**Violations associées :** VIO-EB-6b, VIO-EB-6c

---

### AP-13 : Documentation À Posteriori

**Description :** Documenter les transitions après leur exécution plutôt qu'avant.

**Symptômes :**
- Documentation rédigée après la transition
- "On documentera quand on aura le temps"
- Informations incomplètes ou oubliées

**Correction :** La documentation fait partie de la transition, pas un ajout ultérieur.

**Violations associées :** VIO-EB-7a, VIO-EB-7b

---

### AP-14 : Transition Muette

**Description :** Transitions effectuées sans communication aux parties prenantes.

**Symptômes :**
- "On ne savait pas que c'était déprécié"
- Consommateurs découvrant les changements par accident
- Absence de canal de communication

**Correction :** Annonce proactive via les canaux appropriés avant toute transition.

**Violations associées :** VIO-EB-7c, VIO-EB-12a

---

### AP-15 : Exception Spéciale

**Description :** Créer des exceptions aux règles pour des cas particuliers.

**Symptômes :**
- "Pour ce produit, on fait une exception"
- Règles à géométrie variable
- Accumulation d'exceptions

**Correction :** Les règles sont universelles. Pas d'exception, pas de favoritisme.

**Violations associées :** VIO-EB-8a, VIO-EB-8c

---

### AP-16 : Client Roi

**Description :** Modifier les règles sous la pression d'un client ou d'un utilisateur.

**Symptômes :**
- "Le client X exige que..."
- Règles assouplies pour des clients importants
- Gouvernance soumise aux intérêts commerciaux

**Correction :** Ever Buddy est indépendant. Les règles ne se négocient pas.

**Violations associées :** VIO-EB-8b, VIO-EB-8d

---

### AP-17 : Règles Mouvantes

**Description :** Les règles de transition changent fréquemment sans stabilité.

**Symptômes :**
- Règles différentes d'un mois à l'autre
- "Maintenant on fait comme ça"
- Consommateurs perdus face aux changements

**Correction :** Les règles sont stables. Toute modification est exceptionnelle et annoncée.

**Violations associées :** VIO-EB-9c, VIO-EB-9d

---

### AP-18 : Rétroactivité

**Description :** Appliquer de nouvelles règles à des situations passées ou en cours.

**Symptômes :**
- "Avec les nouvelles règles, cette transition est invalide"
- Annulation de décisions passées
- Modifications de périodes en cours

**Correction :** Les nouvelles règles s'appliquent aux futures transitions uniquement.

**Violations associées :** VIO-EB-9b, VIO-EB-11a, VIO-EB-11b

---

### AP-19 : Successeurs Concurrents

**Description :** Plusieurs successeurs officiels sont déclarés pour un même élément déprécié.

**Symptômes :**
- "Vous pouvez migrer vers A ou B"
- Compétition entre successeurs
- Consommateurs divisés

**Correction :** Un seul successeur principal. Les alternatives sont documentées mais non officielles.

**Violations associées :** VIO-EB-10a

---

### AP-20 : Successeur Fantôme

**Description :** Aucun successeur n'est désigné malgré l'existence d'alternatives.

**Symptômes :**
- "C'est déprécié mais on ne sait pas par quoi le remplacer"
- Migration impossible par manque d'information
- Consommateurs bloqués

**Correction :** Déclarer explicitement le successeur (ou "aucun" si abandon volontaire).

**Violations associées :** VIO-EB-10b, VIO-EB-10c

---

### AP-21 : Règles à Géométrie Variable

**Description :** Les règles sont appliquées différemment selon les circonstances.

**Symptômes :**
- Deux éléments similaires traités différemment
- Interprétations variables des règles
- "Ça dépend du contexte"

**Correction :** Application uniforme et prévisible des règles, sans exception.

**Violations associées :** VIO-EB-9d, VIO-EB-11c, VIO-EB-11d

---

### AP-22 : Annonce Invisible

**Description :** L'annonce de transition existe mais n'est pas visible par les consommateurs.

**Symptômes :**
- Annonce dans un canal non surveillé
- Documentation technique obscure
- "C'était écrit quelque part"

**Correction :** Utiliser des canaux de communication actifs et vérifier la réception.

**Violations associées :** VIO-EB-12c, VIO-EB-12d

---

### AP-23 : Dernière Minute

**Description :** Annoncer les transitions au dernier moment.

**Symptômes :**
- Annonce quelques jours avant la transition
- Pas de temps de préparation
- Migrations d'urgence forcées

**Correction :** Respecter les périodes minimales d'annonce définies par catégorie d'élément.

**Violations associées :** VIO-EB-12a, VIO-EB-12b

---

## 6. Tableau récapitulatif des violations

| Invariant | Violations | Gravité | Anti-patterns |
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

## 7. Détection et prévention

### 7.1 Mécanismes de détection

| Mécanisme | Violations détectées | Moment |
|-----------|---------------------|--------|
| **Validation pré-transition** | VIO-EB-3*, VIO-EB-4*, VIO-EB-10* | Avant transition |
| **Audit d'historique** | VIO-EB-2* | Continu |
| **Vérification de documentation** | VIO-EB-7* | Avant transition |
| **Contrôle de compatibilité** | VIO-EB-5* | À chaque évolution |
| **Monitoring de règles** | VIO-EB-8*, VIO-EB-9* | Continu |
| **Vérification d'annonce** | VIO-EB-12* | Avant transition |

### 7.2 Prévention par conception

| Principe | Description | Violations prévenues |
|----------|-------------|---------------------|
| **Séparation stricte** | Ever Buddy n'a aucun accès en écriture aux données | VIO-EB-1* |
| **Historique append-only** | Aucune API de modification d'historique | VIO-EB-2* |
| **État obligatoire** | Champ d'état requis, non nullable | VIO-EB-3* |
| **Matrice de transitions** | Transitions invalides bloquées structurellement | VIO-EB-4* |
| **Validation de version** | Contrôle automatique du versionnement sémantique | VIO-EB-5* |
| **Règles immuables** | Règles versionnées et non modifiables rétroactivement | VIO-EB-9*, VIO-EB-11* |

### 7.3 Alertes et escalade

| Niveau de violation | Action | Escalade |
|---------------------|--------|----------|
| **V1 - Critique** | Rejet immédiat, alerte système | TAMR (intervention humaine) |
| **V2 - Grave** | Rejet, alerte | Caring Nanny (observation) |
| **V3 - Mineure** | Avertissement | Log uniquement |

---

## 8. Conformité aux Lois d'Autonomie

Ce contrat respecte les Lois d'Autonomie Système :

| Loi | Conformité | Application |
|-----|------------|-------------|
| **LOI-1** | ✅ | Détection de violations locale, pas de dépendance externe |
| **LOI-2** | ✅ | Violations détectables en mode isolé |
| **LOI-3** | ✅ | État de violation souverain localement |
| **LOI-4** | ✅ | Détection basée sur états, pas sur temps global |

**Référence :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

## 9. Références croisées

- **Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)
- **Contrat complémentaire :** [Ever Buddy - Invariants & Guarantees](./Ever%20Buddy%20-%20Invariants%20%26%20Guarantees.md) (définitions des invariants)
- **États de cycle de vie :** [Ever Buddy - Lifecycle States Contract](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md)
- **Règles de transition :** [Ever Buddy - Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md)
- **Glossaire :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- **Lois d'Autonomie :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif — Non négociable  
**Dérivé de :** Ever Buddy - Documentation Fondatrice v1.3, Section 7 (Invariants)  
**Type :** Contrat de gouvernance - Violations et Anti-Patterns
